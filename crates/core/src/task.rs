//! Stages and tasks (REQ-TASK-2, REQ-TASK-3, REQ-TASK-7, REQ-GATE-1's duplicates, REQ-GATE-9, REQ-RUN-7's refusal;
//! part 7d1, D-053): the payloads of the kinds that create and authorize them, held to their shape by schema 1 and to
//! their content at `propose`, at `authorize` and on replay; the allowed-file patterns, their matcher and the guards
//! built on it.

use crate::code::{Code, Error, refuse};
use crate::detect::{Program, text};
use crate::record::{is_hex, is_id};
use crate::settings::{self, Effective, Level, Val};
use serde_json::{Value, json};
use std::collections::BTreeSet;

/// A payload field's type. A wrong JSON type, a missing or an unknown field fails the shape; a value outside its set
/// (an empty or blank text, a tag that is not one, a count below 1, a malformed identifier) fails the content.
enum T {
    Text,
    Tag,
    Count,
    Bool,
    Hex(usize),
    Id(char),
    Base,
    Value,
    List(&'static T),
    Map(&'static T),
    /// An object's fields; a name starting with `?` may be absent.
    Obj(&'static [(&'static str, T)]),
}

const LIMITS: T =
    T::Obj(&[("rounds", T::Count), ("dispatches", T::Count), ("minutes", T::Count), ("?tokens", T::Count)]);
const HEAD: T = T::Obj(&[("seq", T::Count), ("digest", T::Hex(64))]);
const CHECK: T = T::Obj(&[
    ("command", T::List(&T::Text)),
    ("network", T::Bool),
    ("minutes", T::Count),
    ("tools", T::List(&T::Text)),
]);
const BUDGET: T = T::Obj(&[("dispatches", T::Count), ("minutes", T::Count), ("?tokens", T::Count)]);
const REQUIREMENT: T = T::Obj(&[("text", T::Text), ("criterion", T::Text)]);
const STAGE: T = T::Obj(&[
    ("nonce", T::Hex(32)),
    ("lang", T::Tag),
    ("goal", T::Text),
    ("budget", BUDGET),
    ("checks", T::Map(&CHECK)),
    ("settings", T::Map(&T::Value)),
]);
const TASK: T = T::Obj(&[
    ("nonce", T::Hex(32)),
    ("stage", T::Id('s')),
    ("lang", T::Tag),
    ("goal", T::Text),
    ("requirements", T::List(&REQUIREMENT)),
    ("base", T::Base),
    ("dependencies", T::List(&T::Id('t'))),
    ("allowed", T::List(&T::Text)),
    ("profiles", T::Obj(&[("build", T::Text), ("review", T::Text)])),
    ("checks", T::List(&T::Text)),
    ("limits", LIMITS),
    ("delegationLanguage", T::Tag),
    ("settings", T::Map(&T::Value)),
]);
const BOUND: T = T::Obj(&[("task", T::Id('t')), ("digest", T::Hex(64)), ("head", HEAD), ("limits", LIMITS)]);
const TASK_AUTHORIZED: T = T::Obj(&[
    ("task", T::Id('t')),
    ("digest", T::Hex(64)),
    ("head", HEAD),
    ("limits", LIMITS),
    ("shown", T::Hex(64)),
    ("widened", T::List(&T::Text)),
]);
const STAGE_AUTHORIZED: T = T::Obj(&[
    ("stage", T::Id('s')),
    ("digest", T::Hex(64)),
    ("head", HEAD),
    ("tasks", T::List(&BOUND)),
    ("shown", T::Hex(64)),
    ("widened", T::List(&T::Text)),
]);
const TASK_REJECTED: T = T::Obj(&[("task", T::Id('t')), ("head", HEAD)]);

/// Holds `v` to `t`: a failure of shape is the error, where; the first failure of content is kept in `content`, so
/// that a payload is refused for its shape whenever its shape fails.
fn fit(t: &T, v: &Value, at: &str, content: &mut Option<String>) -> Result<(), String> {
    let mut ok = |good: bool| {
        if !good && content.is_none() {
            *content = Some(at.to_owned());
        }
        Ok(())
    };
    match (t, v) {
        (T::Text, Value::String(s)) => ok(!s.trim().is_empty()),
        (T::Tag, Value::String(s)) => ok(settings::is_tag(s)),
        (T::Count, Value::Number(n)) => ok(n.as_u64().is_some_and(|n| n > 0)),
        (T::Bool, Value::Bool(_)) | (T::Base, Value::Null) => Ok(()),
        (T::Hex(n), Value::String(s)) => ok(is_hex(s, *n)),
        (T::Id(c), Value::String(s)) => ok(is_id(s, *c)),
        (T::Base, Value::String(s)) => ok(is_hex(s, 40) || is_hex(s, 64)),
        (T::Value, v) if value(v).is_some() => Ok(()),
        (T::List(t), Value::Array(a)) => {
            a.iter().enumerate().try_for_each(|(i, v)| fit(t, v, &format!("{at}[{i}]"), content))
        }
        (T::Map(t), Value::Object(o)) => o.iter().try_for_each(|(k, v)| fit(t, v, &format!("{at}.{k}"), content)),
        (T::Obj(fields), Value::Object(o)) => {
            let name = |f: &'static str| f.trim_start_matches('?');
            if !o.keys().all(|k| fields.iter().any(|(f, _)| name(f) == k)) {
                return Err(at.to_owned());
            }
            for (f, t) in *fields {
                match o.get(name(f)) {
                    Some(v) => fit(t, v, &format!("{at}.{}", name(f)), content)?,
                    None if f.starts_with('?') => {}
                    None => return Err(format!("{at}.{}", name(f))),
                }
            }
            Ok(())
        }
        _ => Err(at.to_owned()),
    }
}

/// A settings value as a stage's or a task's layer holds it: a boolean, an integer, a string or a list of strings.
pub fn value(v: &Value) -> Option<Val> {
    Some(match v {
        Value::Bool(b) => Val::Bool(*b),
        Value::Number(n) => Val::Int(n.as_i64()?),
        Value::String(s) => Val::Str(s.clone()),
        Value::Array(a) => Val::List(a.iter().map(|v| v.as_str().map(str::to_owned)).collect::<Option<_>>()?),
        _ => return None,
    })
}

fn json_of(v: &Val) -> Value {
    match v {
        Val::Bool(b) => json!(b),
        Val::Int(i) => json!(i),
        Val::Str(s) => json!(s),
        Val::List(l) => json!(l),
    }
}

/// Whether `kind`'s payload has its shape (schema 1 refuses it `RECORD_VERSION_UNKNOWN` otherwise); kinds whose
/// payload a later part fixes pass.
pub(crate) fn shape(kind: &str, p: &Value) -> bool {
    check(kind, p).map_or_else(|e| e.code() != Some(Code::RecordVersionUnknown), |_| true)
}

/// Holds the payload of `kind` to its shape (`RECORD_VERSION_UNKNOWN`) and its content (`TASK_INVALID`): every field
/// of REQ-TASK-2 or REQ-TASK-3; a layer's keys each one that level may set, with a value of its type; a task's base
/// commit exactly when it has no dependencies, distinct dependencies, at least one requirement and one allowed file,
/// each a valid pattern. Kinds whose payload a later part fixes pass.
pub fn check(kind: &str, p: &Value) -> Result<(), Error> {
    let (schema, level) = match kind {
        "stage-proposed" => (&STAGE, Level::Stage),
        "task-proposed" => (&TASK, Level::Task),
        "stage-authorized" => (&STAGE_AUTHORIZED, Level::Stage),
        "task-authorized" => (&TASK_AUTHORIZED, Level::Task),
        "task-rejected" => (&TASK_REJECTED, Level::Task),
        _ => return Ok(()),
    };
    let fail = |code: Code, why: String| refuse(code, None, format!("{kind}: {why}"));
    let invalid = |why: &str| Err(fail(Code::TaskInvalid, why.to_owned()));
    let mut content = None;
    fit(schema, p, "payload", &mut content).map_err(|at| fail(Code::RecordVersionUnknown, format!("{at}: shape")))?;
    if let Some(at) = content {
        return invalid(&format!("{at} is not valid"));
    }
    for (path, v) in p.get("settings").and_then(Value::as_object).into_iter().flatten() {
        let key = settings::declared(path).filter(|k| k.levels.contains(&level));
        if !key.is_some_and(|k| value(v).is_some_and(|v| settings::admit(k.ty, &v).is_ok())) {
            return invalid(&format!("settings.{path} is not a value this layer may set"));
        }
    }
    let checks = p.get("checks").and_then(Value::as_object).into_iter().flatten();
    if checks.map(|(_, c)| &c["command"]).any(|c| c.as_array().is_some_and(Vec::is_empty)) {
        return invalid("a check's command is empty");
    }
    if kind != "task-proposed" {
        return Ok(());
    }
    let deps = p["dependencies"].as_array().into_iter().flatten().filter_map(Value::as_str).collect::<Vec<_>>();
    if p["base"].is_null() != !deps.is_empty() {
        return invalid("a task has a base commit exactly when it has no dependencies");
    }
    if deps.iter().collect::<BTreeSet<_>>().len() != deps.len() {
        return invalid("a dependency is named twice");
    }
    let allowed = p["allowed"].as_array().into_iter().flatten().filter_map(Value::as_str);
    if p["requirements"].as_array().is_none_or(Vec::is_empty) || p["allowed"].as_array().is_none_or(Vec::is_empty) {
        return invalid("a task needs a requirement and an allowed file");
    }
    allowed.map(pattern).find_map(Result::err).map_or(Ok(()), Err)
}

/// An allowed-file pattern's segments (REQ-TASK-7), empty and `.` segments dropped; `TASK_INVALID` for a leading `/`,
/// a `..` segment, more than 256 bytes or 32 segments, or no segment left.
pub fn pattern(p: &str) -> Result<Vec<&str>, Error> {
    let segs: Vec<&str> = p.split('/').filter(|s| !s.is_empty() && *s != ".").collect();
    if p.starts_with('/') || p.len() > 256 || segs.len() > 32 || segs.is_empty() || segs.contains(&"..") {
        return Err(refuse(Code::TaskInvalid, None, format!("allowed files: {p:?} is not a valid pattern")));
    }
    Ok(segs)
}

/// Whether the pattern's segments match `path` (REQ-TASK-7, REQ-RUN-3), case-sensitively and without backtracking: a
/// table over the segments, each cell matching one pattern segment against one path segment character by character.
/// `steps` counts the cells, which stay within (pattern length + 1) × (path length + 1).
pub fn matches(pattern: &[&str], path: &str, steps: &mut usize) -> bool {
    let path: Vec<&str> = path.split('/').collect();
    let mut reach = vec![false; path.len() + 1];
    reach[0] = true;
    for seg in pattern {
        let mut next = vec![false; path.len() + 1];
        let mut any = false;
        for j in 0..=path.len() {
            if *seg == "**" {
                *steps += 1;
                any |= reach[j];
                next[j] = any;
            } else if j > 0 && reach[j - 1] {
                next[j] = segment(seg, path[j - 1], steps);
            }
        }
        reach = next;
    }
    reach[path.len()]
}

/// One segment: `*` matches any run of characters, `?` one character, anything else itself.
fn segment(pat: &str, name: &str, steps: &mut usize) -> bool {
    let name: Vec<char> = name.chars().collect();
    let mut row: Vec<bool> = (0..=name.len()).map(|j| j == 0).collect();
    *steps += row.len();
    for c in pat.chars() {
        let mut next = vec![false; name.len() + 1];
        for j in 0..=name.len() {
            *steps += 1;
            next[j] = match c {
                '*' => row[j] || (j > 0 && next[j - 1]),
                '?' => j > 0 && row[j - 1],
                c => j > 0 && row[j - 1] && name[j - 1] == c,
            };
        }
        row = next;
    }
    row[name.len()]
}

/// The lane guard (REQ-TASK-7; at `propose` and again at `authorize`): every pattern valid (`TASK_INVALID`), and none
/// that can reach `.git` or `.brewstillery` under ASCII case folding (`PROTECTED_PATH`): one whose first segment is a
/// bare `**`, which can match any first component, or any of whose other segments can match either name. A bare `**`
/// deeper is not refused: the change set refuses those names at any depth (REQ-RUN-3).
pub fn lane(allowed: &[&str]) -> Result<(), Error> {
    for p in allowed {
        let segs = pattern(p)?;
        let protected = |s: &&str| {
            *s != "**" && [".git", ".brewstillery"].iter().any(|n| segment(&s.to_ascii_lowercase(), n, &mut 0))
        };
        if segs[0] == "**" || segs.iter().any(protected) {
            return Err(refuse(
                Code::ProtectedPath,
                None,
                format!("allowed files: {p:?} can reach .git or .brewstillery"),
            ));
        }
    }
    Ok(())
}

/// REQ-GATE-9: `PROTECTED_PATH_WILDCARD` for a pattern holding `*` or `?` that can match a protected path, or a path
/// under a protected directory (one written with a final `/`); a protected path listed literally is kept.
pub fn wildcards(allowed: &[&str], protected: &[String]) -> Result<(), Error> {
    for p in allowed.iter().filter(|p| p.contains(['*', '?'])) {
        let segs = pattern(p)?;
        let reaches = |q: &String| match q.strip_suffix('/') {
            // The pattern matches the directory itself; or some of its segments match it and some are left, or the
            // last of those is a `**` that goes on.
            Some(dir) => (0..=segs.len()).any(|i| {
                matches(&segs[..i], dir, &mut 0) || segs.get(i) == Some(&"**") && matches(&segs[..=i], dir, &mut 0)
            }),
            None => matches(&segs, q, &mut 0),
        };
        if let Some(q) = protected.iter().find(|q| reaches(q)) {
            let why = format!("allowed files: {p:?} reaches the protected {q:?} through a wildcard; list it literally");
            return Err(refuse(Code::ProtectedPathWildcard, None, why));
        }
    }
    Ok(())
}

/// REQ-GATE-1 (PD-24): `CHECK_DUPLICATE` when a stage defines a check that the policy defines differently.
pub fn duplicates(stage_checks: &Value, eff: &Effective) -> Result<(), Error> {
    for (name, def) in stage_checks.as_object().into_iter().flatten() {
        let leaf = |l: &str| eff.values.get(&format!("checks.{name}.{l}")).map_or(Value::Null, |v| json_of(&v.0));
        let ours = json!({"command": leaf("command"), "network": leaf("network"), "minutes": leaf("minutes"), "tools": leaf("tools")});
        if !ours["command"].is_null() && ours != *def {
            return Err(refuse(Code::CheckDuplicate, None, format!("checks.{name}: the stage defines it differently")));
        }
    }
    Ok(())
}

/// REQ-RUN-7: `LIMIT_UNOBSERVABLE` when a token limit (the task's, `tokens`, or a profile's own) applies to a profile
/// whose tool's module does not report the tokens it used (the modules' contract, D-051).
pub fn observable(eff: &Effective, modules: &[Program], profiles: &[&str], tokens: bool) -> Result<(), Error> {
    for p in profiles {
        let adapter = text(eff, &format!("profiles.{p}.tool")).and_then(|t| text(eff, &format!("tools.{t}.adapter")));
        let limited = tokens || eff.values.contains_key(&format!("profiles.{p}.tokens"));
        if limited && !modules.iter().any(|m| m.tokens && m.adapter.is_some() && m.adapter == adapter) {
            return Err(refuse(Code::LimitUnobservable, None, format!("profiles.{p}: its tool reports no tokens")));
        }
    }
    Ok(())
}

/// One field of `propose`'s file: its depth, its name, whether it may be absent, and its type as a chain read from the
/// outside in (`["list", "object"]`).
pub type FormatLine = (usize, &'static str, bool, Vec<&'static str>);

/// The format of `propose`'s file (section 5; part 7d3a): every field of a stage's payload and of a task's, but the
/// nonce, which `propose` draws, and a task's stage, which the file's stage gives. Help and the reference render it.
pub fn file_format() -> [Vec<FormatLine>; 2] {
    fn chain(t: &T) -> Vec<&'static str> {
        let word = match t {
            T::Text => "text",
            T::Tag => "tag",
            T::Count => "count",
            T::Bool => "bool",
            T::Hex(_) => "hex",
            T::Id('s') => "stage-id",
            T::Id(_) => "task-id",
            T::Base => "base",
            T::Value => "value",
            T::Obj(_) => "object",
            T::List(inner) | T::Map(inner) => {
                let outer = if matches!(t, T::List(_)) { "list" } else { "map" };
                return [vec![outer], chain(inner)].concat();
            }
        };
        vec![word]
    }
    fn walk(t: &T, depth: usize, out: &mut Vec<FormatLine>) {
        let fields = match t {
            T::Obj(fields) => *fields,
            T::List(inner) | T::Map(inner) => return walk(inner, depth, out),
            _ => return,
        };
        for (f, ty) in fields.iter().filter(|(f, _)| !["nonce", "stage"].contains(f)) {
            out.push((depth, f.trim_start_matches('?'), f.starts_with('?'), chain(ty)));
            walk(ty, depth + 1, out);
        }
    }
    let [mut stage, mut task] = [Vec::new(), Vec::new()];
    walk(&STAGE, 1, &mut stage);
    walk(&TASK, 1, &mut task);
    [stage, task]
}
