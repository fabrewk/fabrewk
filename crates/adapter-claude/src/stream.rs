//! Claude Code's stream (`--print --output-format stream-json --verbose`, as 2.1.316 writes it) and the audit of what
//! a reviewer ran (REQ-GATE-4, PD-25, PD-26). One grammar, listed here and nowhere else, read only once the cell has
//! ended: one JSON object per line, lines ended by a newline alone; the `init` event first; the estimates of thinking;
//! the assistant's text, thinking and tool uses; the user's turn holding only the answers to those uses; rate-limit
//! notices; one `result`, last. Anything else is an anomaly, and any anomaly voids the whole stream: no result, no
//! usage, no examination (T-12). Examination counts the uses answered without an error of the tools that read or run
//! (`Bash`, by its simple commands and only when it printed something, and `Read`, `Grep`, `Glob`), never the
//! provider's own answer tool, and only when the run ended in success. It detects a lazy or failed review, not a
//! hostile one: the model's commands could print what this reads (T-13).

use fabrewk_core::adapter::{Report, Tokens};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

/// The tools whose answered use examines the copy.
const EXAMINING: [&str; 4] = ["Bash", "Read", "Grep", "Glob"];

/// What the tool answers for a command that printed nothing (2.1.316, recorded).
const SILENT: &str = "(Bash completed with no output)";

/// A tool use, by its identifier: the tool, its count of examinations, and once answered whether it examined: an
/// answer without an error and, for a command, with output.
struct Use {
    tool: String,
    count: u64,
    answer: Option<bool>,
}

/// What a stream reads to, before the end decides the report; `None` when an anomaly voided it.
#[derive(Default)]
struct Read {
    model: Option<String>,
    uses: BTreeMap<String, Use>,
    result: Option<Map<String, Value>>,
}

pub fn read(out: &[u8]) -> Report {
    let Some(r) = std::str::from_utf8(out).ok().and_then(grammar) else {
        return Report::default();
    };
    let Some(end) = r.result else {
        return Report::default();
    };
    let tokens = end.get("usage").and_then(Value::as_object).map(tokens);
    let success = end.get("subtype").and_then(Value::as_str) == Some("success")
        && end.get("is_error") == Some(&Value::Bool(false))
        && end.get("structured_output").is_some_and(Value::is_object);
    if !success {
        return Report { result: None, model: r.model, tokens, examined: 0 };
    }
    // A use answered with an error examined nothing (P9C-1).
    let examined = r
        .uses
        .values()
        .filter(|u| EXAMINING.contains(&u.tool.as_str()) && u.answer == Some(true))
        .map(|u| u.count)
        .sum();
    Report { result: end.get("structured_output").cloned(), model: r.model, tokens, examined }
}

/// The tokens `usage` reports (S-5): input read fresh (cache writes included), output, and input read from the cache;
/// each unknown unless a whole number.
fn tokens(usage: &Map<String, Value>) -> Tokens {
    let n = |k: &str| usage.get(k).and_then(Value::as_u64);
    let input = n("input_tokens").zip(n("cache_creation_input_tokens")).and_then(|(a, b)| a.checked_add(b));
    Tokens { input, output: n("output_tokens"), cached: n("cache_read_input_tokens") }
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
        // The tool writes each event compact: white space around it (a carriage return) is not its.
        if line.starts_with(char::is_whitespace) || line.ends_with(char::is_whitespace) || r.result.is_some() {
            return None;
        }
        let event: Map<String, Value> = serde_json::from_str(line).ok()?;
        let kind = event.get("type").and_then(Value::as_str)?;
        // Turns are only the assistant's and the user's: a message anywhere else is evidence misplaced (P9C-2).
        if event.contains_key("message") && !matches!(kind, "assistant" | "user") {
            return None;
        }
        match (n, kind) {
            (0, "system") => {
                (event.get("subtype").and_then(Value::as_str) == Some("init")).then_some(())?;
                r.model = Some(event.get("model").and_then(Value::as_str)?.to_owned());
            }
            (0, _) => return None,
            // The estimate of the thinking so far, which a run that thinks writes between its turns.
            (_, "system") if event.get("subtype").and_then(Value::as_str) == Some("thinking_tokens") => {}
            (_, "assistant") => assistant(&event, &mut r)?,
            (_, "user") => user(&event, &mut r)?,
            (_, "rate_limit_event") => {}
            (_, "result") => {
                // Its flag and subtype are of their types, whatever they say (P9C-6).
                (event.get("is_error").is_some_and(Value::is_boolean)
                    && event.get("subtype").is_some_and(Value::is_string))
                .then_some(())?;
                // Every use was answered before the result (PD-26.7).
                r.uses.values().all(|u| u.answer.is_some()).then_some(())?;
                r.result = Some(event);
            }
            _ => return None,
        }
    }
    Some(r)
}

fn blocks(event: &Map<String, Value>) -> Option<&Vec<Value>> {
    event.get("message")?.get("content")?.as_array()
}

/// The assistant's turn: text, thinking and tool uses, each use with a new text identifier, a text name and an
/// object input, a command's text when it is `Bash`.
fn assistant(event: &Map<String, Value>, r: &mut Read) -> Option<()> {
    for block in blocks(event)? {
        match block.get("type").and_then(Value::as_str)? {
            "text" => text(block, "text")?,
            "thinking" => text(block, "thinking")?,
            "tool_use" => {
                let id = block.get("id").and_then(Value::as_str).filter(|s| !s.is_empty())?;
                let tool = block.get("name").and_then(Value::as_str).filter(|s| !s.trim().is_empty())?;
                let input = block.get("input").and_then(Value::as_object)?;
                let count = match tool {
                    "Bash" => {
                        let command = input.get("command").and_then(Value::as_str)?;
                        let count = u64::try_from(segments(command).len()).ok().filter(|&c| c > 0)?;
                        // Started in the background, its answer says it started, not what it ran (P9C-8).
                        let background = input.get("run_in_background").is_some_and(|b| b != &Value::Bool(false));
                        if background { 0 } else { count }
                    }
                    _ => 1,
                };
                if r.uses.contains_key(id) {
                    return None;
                }
                r.uses.insert(id.to_owned(), Use { tool: tool.to_owned(), count, answer: None });
            }
            _ => return None,
        }
    }
    Some(())
}

/// Whether `block`'s field `key` is a text.
fn text(block: &Value, key: &str) -> Option<()> {
    block.get(key)?.as_str().map(|_| ())
}

/// The user's turn: one answer, to a use not answered yet, its error flag a boolean, and present when the use
/// examines; its content a text, or a list of text blocks. A command examines only when the run's own record beside
/// the answer, and the answer itself, show it completed and printed: a silent command, an uncalled function, an empty
/// loop, a command still running in the background, or one interrupted shows none (P9C-7 to P9C-9).
fn user(event: &Map<String, Value>, r: &mut Read) -> Option<()> {
    let [block] = &blocks(event)?[..] else {
        return None;
    };
    (block.get("type").and_then(Value::as_str)? == "tool_result").then_some(())?;
    let id = block.get("tool_use_id").and_then(Value::as_str)?;
    let used = r.uses.get_mut(id).filter(|u| u.answer.is_none())?;
    let mut said = String::new();
    match block.get("content")? {
        Value::String(s) => said.push_str(s),
        Value::Array(parts) => {
            for part in parts {
                (part.get("type").and_then(Value::as_str)? == "text").then_some(())?;
                said.push_str(part.get("text")?.as_str()?);
            }
        }
        _ => return None,
    }
    let failed = match block.get("is_error") {
        Some(Value::Bool(e)) => *e,
        None if !EXAMINING.contains(&used.tool.as_str()) => false,
        _ => return None,
    };
    let printed = |run: &Map<String, Value>| {
        let channel = |k: &str| run.get(k).and_then(Value::as_str).is_some_and(|s| !s.trim().is_empty());
        run.get("interrupted") == Some(&Value::Bool(false))
            && !run.contains_key("backgroundTaskId")
            && (channel("stdout") || channel("stderr"))
    };
    let ran = event.get("tool_use_result").and_then(Value::as_object).is_some_and(printed);
    let silent = used.tool == "Bash" && (!ran || said.trim().is_empty() || said == SILENT);
    used.answer = Some(!failed && !silent);
    Some(())
}

/// A command's simple commands: split on newlines, `;`, `&&`, `||` and `|` outside quotes; a backslash escapes the
/// next character outside single quotes, and with a newline joins two lines (P9C-4); a `#` that starts a word
/// outside quotes comments the rest of its line out (P9C-3); blank parts dropped (PD-26.2).
pub fn segments(command: &str) -> Vec<String> {
    let (mut parts, mut current, mut quote) = (Vec::new(), String::new(), None);
    let mut chars = command.chars().peekable();
    while let Some(c) = chars.next() {
        match (quote, c) {
            (Some('\''), '\'') | (Some('"'), '"') => {
                quote = None;
                current.push(c);
            }
            // A backslash and a newline join two lines, as the shell removes them before reading (P9C-4).
            (q, '\\') if q != Some('\'') && chars.next_if_eq(&'\n').is_some() => {}
            (q, '\\') if q != Some('\'') => {
                current.push(c);
                current.extend(chars.next());
            }
            (Some(_), _) => current.push(c),
            (None, '\'' | '"') => {
                quote = Some(c);
                current.push(c);
            }
            (None, '#') if current.chars().last().is_none_or(char::is_whitespace) => {
                while chars.next_if(|&n| n != '\n').is_some() {}
            }
            (None, '\n' | ';') => parts.push(std::mem::take(&mut current)),
            (None, '&' | '|') if chars.peek() == Some(&c) => {
                chars.next();
                parts.push(std::mem::take(&mut current));
            }
            (None, '|') => parts.push(std::mem::take(&mut current)),
            (None, _) => current.push(c),
        }
    }
    parts.push(current);
    parts.into_iter().map(|p| p.trim().to_owned()).filter(|p| !p.is_empty()).collect()
}
