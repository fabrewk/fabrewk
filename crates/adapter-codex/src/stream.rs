//! Codex's stream (`exec --json`, as 0.166.0 writes it) and the audit of what a reviewer ran (REQ-GATE-4, PD-25,
//! PD-26). One grammar, listed here and nowhere else, read only once the cell has ended: one compact JSON object per
//! line, lines ended by a newline alone; `thread.started`, then `turn.started`; then items started, updated and
//! completed, and notices of errors the tool recovered from; one `turn.completed`, last. Every event and item holds
//! exactly its fields, of their types. Anything else is an anomaly, and any anomaly voids the whole stream: no result,
//! no usage, no examination (T-12); a turn that failed (`turn.failed`) gives nothing more, so it is read as one. The
//! result is the last message of the agent, read as a JSON object. Examination counts the simple commands of each
//! command that completed with status 0 and printed something; a command that failed, was declined or printed nothing
//! examined nothing. It detects a lazy or failed review, not a hostile one: the model's commands could print what this
//! reads (T-13). The stream does not say which model answered.

use fabrewk_core::adapter::{self, Report, Tokens, segments};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

/// An item, by its identifier: its kind, its command when it is one, and whether it completed.
struct Item {
    kind: String,
    command: Option<String>,
    done: bool,
}

/// What a stream reads to; `None` when an anomaly voided it.
#[derive(Default)]
struct Read {
    items: BTreeMap<String, Item>,
    message: Option<String>,
    examined: u64,
    /// The turn's usage, once it completed.
    end: Option<Map<String, Value>>,
}

pub fn read(out: &[u8]) -> Report {
    let Some(r) = std::str::from_utf8(out).ok().and_then(grammar) else {
        return Report::default();
    };
    let Some(usage) = r.end else {
        return Report::default();
    };
    let tokens = Some(tokens(&usage));
    let result = r.message.as_deref().and_then(adapter::json).filter(Value::is_object);
    let examined = if result.is_some() { r.examined } else { 0 };
    Report { result, model: None, tokens, examined }
}

/// The tokens `usage` reports (S-5): input read fresh (the input less what came from the cache), output (reasoning
/// included), and input read from the cache; each unknown unless a whole number.
fn tokens(usage: &Map<String, Value>) -> Tokens {
    let n = |k: &str| usage.get(k).and_then(Value::as_u64);
    let input = n("input_tokens").zip(n("cached_input_tokens")).and_then(|(a, b)| a.checked_sub(b));
    Tokens { input, output: n("output_tokens"), cached: n("cached_input_tokens") }
}

/// Whether `o` holds exactly the fields `keys`.
fn exactly(o: &Map<String, Value>, keys: &[&str]) -> bool {
    o.len() == keys.len() && keys.iter().all(|k| o.contains_key(*k))
}

fn string<'a>(o: &'a Map<String, Value>, k: &str) -> Option<&'a str> {
    o.get(k).and_then(Value::as_str)
}

/// The stream read by the grammar; `None` at the first anomaly.
fn grammar(text: &str) -> Option<Read> {
    let mut lines: Vec<&str> = text.split('\n').collect();
    // A final newline ends the last line; it is not a blank line (PD-26.19).
    if lines.last() == Some(&"") {
        lines.pop();
    }
    let mut r = Read::default();
    for (n, line) in lines.into_iter().enumerate() {
        if line.starts_with(char::is_whitespace) || line.ends_with(char::is_whitespace) || r.end.is_some() {
            return None;
        }
        let Value::Object(event) = adapter::json(line)? else {
            return None;
        };
        match (n, string(&event, "type")?) {
            (0, "thread.started") => {
                (exactly(&event, &["type", "thread_id"]) && !string(&event, "thread_id")?.is_empty()).then_some(())?
            }
            (1, "turn.started") => exactly(&event, &["type"]).then_some(())?,
            (0 | 1, _) => return None,
            (_, "item.started" | "item.updated" | "item.completed") if exactly(&event, &["type", "item"]) => {
                item(&event, &mut r)?
            }
            // A notice of what the tool retried or recovered from; it carries no evidence.
            (_, "error") => {
                (exactly(&event, &["type", "message"]) && string(&event, "message").is_some()).then_some(())?
            }
            (_, "turn.completed") => {
                let usage =
                    event.get("usage").and_then(Value::as_object).filter(|_| exactly(&event, &["type", "usage"]))?;
                // Every item started was completed before the end (PD-26.7).
                r.items.values().all(|i| i.done).then_some(())?;
                r.end = Some(usage.clone());
            }
            _ => return None,
        }
    }
    Some(r)
}

/// The fields of each kind of item beside `id` and `type`, and whether it has a status.
fn fields(kind: &str) -> Option<&'static [&'static str]> {
    Some(match kind {
        "agent_message" | "reasoning" => &["id", "type", "text"],
        "command_execution" => &["id", "type", "command", "aggregated_output", "exit_code", "status"],
        "file_change" => &["id", "type", "changes", "status"],
        "todo_list" => &["id", "type", "items"],
        "error" => &["id", "type", "message"],
        _ => return None,
    })
}

/// Whether every field of `item` is of its type, by its kind.
fn typed(item: &Map<String, Value>, kind: &str) -> bool {
    let all = |k: &str, each: &dyn Fn(&Map<String, Value>) -> bool| {
        item.get(k).and_then(Value::as_array).is_some_and(|a| a.iter().all(|v| v.as_object().is_some_and(each)))
    };
    let status = |set: &[&str]| string(item, "status").is_some_and(|s| set.contains(&s));
    match kind {
        "agent_message" | "reasoning" => string(item, "text").is_some(),
        "error" => string(item, "message").is_some(),
        // Running, it has no exit code; completed, a whole number; failed or declined, either (PD-26.1, PD-26.15).
        "command_execution" => {
            let code = &item["exit_code"];
            string(item, "command").is_some()
                && string(item, "aggregated_output").is_some()
                && match string(item, "status") {
                    Some("in_progress") => code.is_null(),
                    Some("completed") => code.is_i64(),
                    Some("failed" | "declined") => code.is_null() || code.is_i64(),
                    _ => false,
                }
        }
        "file_change" => {
            status(&["in_progress", "completed", "failed"])
                && all("changes", &|c| {
                    exactly(c, &["path", "kind"])
                        && string(c, "path").is_some()
                        && string(c, "kind").is_some_and(|k| ["add", "delete", "update"].contains(&k))
                })
        }
        _ => all("items", &|i| {
            exactly(i, &["text", "completed"]) && string(i, "text").is_some() && i["completed"].is_boolean()
        }),
    }
}

/// An item's event: a new item started, or one started and not completed updated or completed, keeping its kind and
/// its command; an item with a status in progress only until it completes. A completed message of the agent is the
/// result so far; a completed command that ended with status 0 and printed something examines.
fn item(event: &Map<String, Value>, r: &mut Read) -> Option<()> {
    let item = event.get("item")?.as_object()?;
    let (id, kind) = (string(item, "id").filter(|s| !s.is_empty())?, string(item, "type")?);
    (exactly(item, fields(kind)?) && typed(item, kind)).then_some(())?;
    let command = string(item, "command").map(str::to_owned);
    let running = string(item, "status") == Some("in_progress");
    let finished = item.contains_key("status") && !running;
    let phase = string(event, "type")?;
    match (phase, r.items.get_mut(id)) {
        ("item.started", None) if !finished => {
            r.items.insert(id.to_owned(), Item { kind: kind.to_owned(), command, done: false });
            return Some(());
        }
        ("item.updated", Some(i)) if !i.done && i.kind == kind && i.command == command && !finished => return Some(()),
        ("item.completed", Some(i)) if !i.done && i.kind == kind && i.command == command && !running => i.done = true,
        ("item.completed", None) if !running => {
            r.items.insert(id.to_owned(), Item { kind: kind.to_owned(), command, done: true });
        }
        _ => return None,
    }
    match kind {
        "agent_message" => r.message = string(item, "text").map(str::to_owned),
        "command_execution"
            if string(item, "status") == Some("completed")
                && item["exit_code"] == 0
                && !string(item, "aggregated_output")?.trim().is_empty() =>
        {
            r.examined = r.examined.checked_add(count(string(item, "command")?))?;
        }
        _ => {}
    }
    Some(())
}

/// The simple commands of a command as Codex shows it: its words joined and quoted for a shell; a shell's `-c` or
/// `-lc` with one script counts the script's, any other command its own line's. A command whose quoting does not close
/// counts none.
pub fn count(command: &str) -> u64 {
    let Some(words) = words(command) else {
        return 0;
    };
    let shell = |w: &str| ["sh", "bash", "dash", "zsh"].contains(&w.rsplit('/').next().unwrap_or(w));
    let script = match &words[..] {
        [sh, flag, script] if shell(sh) && (flag == "-c" || flag == "-lc") => script.as_str(),
        _ => command,
    };
    u64::try_from(segments(script).len()).unwrap_or(u64::MAX)
}

/// A line's words as a POSIX shell reads them, unquoted: single quotes keep every character; double quotes let a
/// backslash escape `$`, `` ` ``, `"`, `\` and a newline; outside quotes a backslash escapes the next character. Any
/// other expansion is kept as written. `None` when a quote or an escape does not close.
pub fn words(line: &str) -> Option<Vec<String>> {
    let (mut out, mut word, mut quote, mut started) = (Vec::new(), String::new(), None, false);
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        match (quote, c) {
            (Some(q), c) if c == q => quote = None,
            (Some('"'), '\\') => match chars.next()? {
                n @ ('$' | '`' | '"' | '\\') => word.push(n),
                '\n' => {}
                n => word.extend(['\\', n]),
            },
            (Some(_), c) => word.push(c),
            (None, '\'' | '"') => (quote, started) = (Some(c), true),
            (None, '\\') => (word, started) = (word + &chars.next()?.to_string(), true),
            (None, c) if c.is_whitespace() => {
                if std::mem::take(&mut started) {
                    out.push(std::mem::take(&mut word));
                }
            }
            (None, c) => (word, started) = (word + &c.to_string(), true),
        }
    }
    if quote.is_some() {
        return None;
    }
    if started {
        out.push(word);
    }
    Some(out)
}
