//! The side effects in the catalogued tools' settings (REQ-INS-1, point 3): hooks, automatic updates, telemetry and
//! error reports, MCP servers, tools approved in advance, a sandbox turned off, and hooks or commands that run `git
//! commit` or `git push`. Each is reported by file and key path, with a count where it is a list or a table of names,
//! never with a value, which may hold a credential: a value is read only to say whether a rule holds.

use serde_json::{Value, json};
use toml::de::{DeTable, DeValue};

/// When a key path is a side effect.
enum When {
    Present,
    /// A list or a table that is not empty; its size is the count.
    Count,
    True,
    False,
    Is(&'static str),
    Not(&'static str),
}

use When::*;

/// The rules: a key path (`*` is any one name), the kind of side effect, and when it holds. Claude Code's and Codex's
/// keys do not overlap, so every file is read with all of them.
const RULES: &[(&str, &str, When)] = &[
    ("hooks", "hooks", Count),
    ("notify", "hooks", Count),
    ("permissions.allow", "approvals", Count),
    ("permissions.defaultMode", "approvals", Not("default")),
    ("skipDangerousModePermissionPrompt", "approvals", True),
    ("approval_policy", "approvals", Is("never")),
    ("projects.*.trust_level", "approvals", Is("trusted")),
    ("sandbox.enabled", "sandbox", False),
    ("sandbox_mode", "sandbox", Is("danger-full-access")),
    ("mcpServers", "mcp", Count),
    ("mcp_servers", "mcp", Count),
    ("enabledMcpjsonServers", "mcp", Count),
    ("enableAllProjectMcpServers", "mcp", True),
    ("autoUpdates", "updates", Present),
    ("autoUpdatesChannel", "updates", Present),
    ("check_for_update_on_startup", "updates", Present),
    ("env.DISABLE_AUTOUPDATER", "updates", Present),
    ("env.CLAUDE_CODE_ENABLE_TELEMETRY", "telemetry", Present),
    ("env.DISABLE_TELEMETRY", "telemetry", Present),
    ("env.DISABLE_ERROR_REPORTING", "telemetry", Present),
    ("env.OTEL_EXPORTER_OTLP_ENDPOINT", "telemetry", Present),
    ("otel", "telemetry", Present),
    ("analytics", "telemetry", Present),
];

/// Where a command that commits or pushes may be named.
const COMMANDS: [&str; 3] = ["hooks", "notify", "permissions.allow"];

/// Every side effect `bytes`, the file `file` (as the plan shows it), holds; one that does not read is reported as
/// such, since what it holds cannot be told.
pub fn scan(file: &str, bytes: &[u8], toml: bool) -> Vec<Value> {
    let text = std::str::from_utf8(bytes).ok();
    let value = text.and_then(|t| if toml { DeTable::parse(t).ok().map(|d| table(d.get_ref())) } else { json(t) });
    let Some(v) = value else { return vec![json!({"file": file, "key": "", "kind": "unparsed"})] };
    let mut out = Vec::new();
    for (path, kind, when) in RULES {
        let hits: Vec<&Value> = at(&v, path).into_iter().filter(|x| holds(when, x)).collect();
        let count = match when {
            Count => Some(hits.iter().map(|h| size(h)).sum()),
            _ if path.contains('*') => Some(hits.len()),
            _ => None,
        };
        if !hits.is_empty() {
            out.push(json!({"file": file, "key": path, "kind": kind, "count": count}));
        }
    }
    for path in COMMANDS {
        let n: usize = at(&v, path).into_iter().map(commits).sum();
        if n > 0 {
            out.push(json!({"file": file, "key": path, "kind": "commits", "count": n}));
        }
    }
    out
}

/// A JSON document, refusing a member named twice (`fabrewk_core::adapter::json`).
fn json(text: &str) -> Option<Value> {
    fabrewk_core::adapter::json(text)
}

/// A TOML table as JSON: strings, booleans, lists and tables as they are; any other value as `null`, present.
fn table(t: &DeTable) -> Value {
    Value::Object(t.iter().map(|(k, v)| (k.get_ref().to_string(), value(v.get_ref()))).collect())
}

fn value(v: &DeValue) -> Value {
    match v {
        DeValue::String(s) => Value::String(s.to_string()),
        DeValue::Boolean(b) => Value::Bool(*b),
        DeValue::Array(a) => Value::Array(a.iter().map(|x| value(x.get_ref())).collect()),
        DeValue::Table(t) => table(t),
        _ => Value::Null,
    }
}

/// The values at `path` in `v`, `*` taking every member of a table.
fn at<'a>(v: &'a Value, path: &str) -> Vec<&'a Value> {
    path.split('.').fold(vec![v], |found, seg| {
        found
            .into_iter()
            .flat_map(|x| match (seg, x.as_object()) {
                ("*", Some(o)) => o.values().collect(),
                (name, Some(o)) => o.get(name).into_iter().collect(),
                _ => Vec::new(),
            })
            .collect()
    })
}

fn size(v: &Value) -> usize {
    match v {
        Value::Array(a) => a.len(),
        Value::Object(o) => o.len(),
        _ => 0,
    }
}

fn holds(when: &When, v: &Value) -> bool {
    match when {
        Present => true,
        Count => size(v) > 0,
        True => *v == Value::Bool(true),
        False => *v == Value::Bool(false),
        Is(s) => v.as_str() == Some(s),
        Not(s) => v.as_str() != Some(s),
    }
}

/// How many strings in `v`, at any depth, run `git commit` or `git push`: in one command of the string, between its
/// separators, a word `git` (or a path to it) and, after it and its options, `commit` or `push` (P10A-6).
fn commits(v: &Value) -> usize {
    match v {
        Value::String(s) => usize::from(s.split([';', '&', '|', '(', ')', '`', '\n']).any(|command| {
            let words: Vec<&str> =
                command.split_whitespace().map(|w| w.trim_matches(|c: char| !c.is_ascii_alphanumeric())).collect();
            let git = words.iter().position(|w| *w == "git" || w.ends_with("/git"));
            git.is_some_and(|at| words[at..].iter().any(|w| matches!(*w, "commit" | "push")))
        })),
        Value::Array(a) => a.iter().map(commits).sum(),
        Value::Object(o) => o.values().map(commits).sum(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(file: &str, text: &str, toml: bool) -> Vec<(String, String, Value)> {
        scan(file, text.as_bytes(), toml)
            .iter()
            .map(|e| {
                (e["key"].as_str().unwrap().to_owned(), e["kind"].as_str().unwrap().to_owned(), e["count"].clone())
            })
            .collect()
    }

    #[test]
    fn claude_settings_are_read_by_key_and_count() {
        let text = r#"{"hooks": {"PostToolUse": [{"hooks": [{"type": "command", "command": "git  add -A && git commit -m x"}]}],
            "Stop": [{"hooks": [{"command": "echo"}]}]},
            "permissions": {"allow": ["Bash(git push:*)", "Read", "Bash(git -C . commit -am x)", "Bash(git log)"], "defaultMode": "bypassPermissions"},
            "env": {"DISABLE_TELEMETRY": "1", "SECRET": "sk-x"}, "autoUpdates": false, "sandbox": {"enabled": false},
            "enableAllProjectMcpServers": true}"#;
        let found = kinds("~/.claude/settings.json", text, false);
        for (key, kind, count) in [
            ("hooks", "hooks", json!(2)),
            ("permissions.allow", "approvals", json!(4)),
            ("permissions.defaultMode", "approvals", Value::Null),
            ("sandbox.enabled", "sandbox", Value::Null),
            ("enableAllProjectMcpServers", "mcp", Value::Null),
            ("autoUpdates", "updates", Value::Null),
            ("env.DISABLE_TELEMETRY", "telemetry", Value::Null),
            ("hooks", "commits", json!(1)),
            ("permissions.allow", "commits", json!(2)),
        ] {
            assert!(found.contains(&(key.to_owned(), kind.to_owned(), count.clone())), "{key} {kind}: {found:?}");
        }
        assert_eq!(found.len(), 9, "{found:?}");
        // No value reaches the report.
        let all = serde_json::to_string(&scan("f", text.as_bytes(), false)).unwrap();
        assert!(!all.contains("sk-x") && !all.contains("bypass") && !all.contains("SECRET"), "{all}");
    }

    #[test]
    fn codex_settings_are_read_by_key_and_count() {
        let text = "approval_policy = \"never\"\nsandbox_mode = \"danger-full-access\"\nnotify = [\"sh\", \"-c\", \"git push\"]\n\
            check_for_update_on_startup = true\n[mcp_servers.a]\ncommand = \"x\"\n[mcp_servers.b]\ncommand = \"y\"\n\
            [projects.\"/home/x/p\"]\ntrust_level = \"trusted\"\n[projects.\"/home/x/q\"]\ntrust_level = \"untrusted\"\n[otel]\nexporter = \"none\"\n";
        let found = kinds("~/.codex/config.toml", text, true);
        for (key, kind, count) in [
            ("approval_policy", "approvals", Value::Null),
            ("sandbox_mode", "sandbox", Value::Null),
            ("notify", "hooks", json!(3)),
            ("notify", "commits", json!(1)),
            ("check_for_update_on_startup", "updates", Value::Null),
            ("mcp_servers", "mcp", json!(2)),
            ("projects.*.trust_level", "approvals", json!(1)),
            ("otel", "telemetry", Value::Null),
        ] {
            assert!(found.contains(&(key.to_owned(), kind.to_owned(), count.clone())), "{key} {kind}: {found:?}");
        }
        assert_eq!(found.len(), 8, "{found:?}");
        assert!(!serde_json::to_string(&scan("f", text.as_bytes(), true)).unwrap().contains("/home/x"));
    }

    #[test]
    fn what_holds_no_side_effect_is_not_reported() {
        let quiet = r#"{"hooks": {}, "permissions": {"allow": [], "defaultMode": "default"}, "sandbox": {"enabled": true},
            "approval_policy": "on-request", "enableAllProjectMcpServers": false}"#;
        assert_eq!(kinds("f", quiet, false), vec![]);
        assert_eq!(kinds("f", "sandbox_mode = \"workspace-write\"\n", true), vec![]);
        for (text, toml) in [("{", false), ("{\"a\": 1, \"a\": 2}", false), ("= x", true), ("\u{0}", false)] {
            assert_eq!(kinds("f", text, toml), vec![(String::new(), "unparsed".to_owned(), Value::Null)], "{text}");
        }
    }
}
