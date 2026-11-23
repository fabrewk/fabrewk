//! The fermento and its answer (REQ-FER-1 to REQ-FER-4, REQ-FER-8, REQ-FER-10; part 8a, D-072): the separator found
//! once outside fenced code blocks, the points above it, the payloads of the sweep and of the proposals, the proposals'
//! states derived from the record, and the answer zone generated from the events alone. A fence is a top-level
//! CommonMark fenced code block: one inside a list item or a quotation starts after a prefix this finder does not
//! read, and a separator at the start of a line is never inside one.

use crate::cli::{CATALOGUES, say};
use crate::code::{Code, Error, refuse};
use crate::detect::Program;
use crate::record::{Event, Replay, StreamKind, sha256_hex, stream_kind};
use crate::table;
use crate::task::{T, fit};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// The line that separates a maintainer's input from the agents' answers (REQ-FER-1).
pub const MARKER: &str = "<!-- fabrewk:fermento-separator v1 -->";

/// What a fermento is split into: the input ends at `input_end`, the separator block runs to `after`.
#[derive(Debug, PartialEq, Eq)]
pub struct Parts {
    pub input_end: usize,
    pub after: usize,
}

/// The lines of `text` with the byte offset each starts at; a final line without a break is a line, and a break is
/// `\n` or `\r\n` (CommonMark), neither part of the line.
pub(crate) fn lines(text: &str) -> Vec<(usize, &str)> {
    let mut at = 0;
    text.split_inclusive('\n')
        .map(|l| {
            let start = at;
            at += l.len();
            (start, l.strip_suffix('\n').map_or(l, |l| l.strip_suffix('\r').unwrap_or(l)))
        })
        .collect()
}

/// Whether each line lies in a fenced code block, its fences included: a run of three backticks or tildes opens one
/// after at most three spaces (a backtick fence's info string holds no backtick), and a run of the same mark at least
/// as long, with only white space after it, closes it; one never closed runs to the end.
pub fn fenced(lines: &[&str]) -> Vec<bool> {
    let mut open: Option<(char, usize)> = None;
    let mut out = Vec::with_capacity(lines.len());
    for line in lines {
        let rest = line.trim_start_matches(' ');
        let run = |c: char| rest.chars().take_while(|x| *x == c).count();
        let fence = if line.len() - rest.len() <= 3 {
            ['`', '~'].into_iter().map(|c| (c, run(c))).find(|(_, n)| *n >= 3)
        } else {
            None
        };
        match (open, fence) {
            (None, Some((c, n))) if c == '~' || !rest[n..].contains('`') => {
                open = Some((c, n));
                out.push(true);
            }
            (None, _) => out.push(false),
            (Some((c, n)), Some((d, m))) if d == c && m >= n && rest[m..].trim_matches([' ', '\t']).is_empty() => {
                open = None;
                out.push(true);
            }
            (Some(_), _) => out.push(true),
        }
    }
    out
}

/// Every rendering of the separator's block a catalogue has for `alias`, as lines.
fn renderings(alias: &str) -> Vec<Vec<String>> {
    let block = |lang: &str| say(lang, "fermento.separator", &[("alias", alias)]);
    CATALOGUES.iter().map(|(lang, _)| block(lang).lines().map(str::to_owned).collect()).collect()
}

/// The block the separator stands in, as `lang` renders it (the file's line before the answer).
pub fn block(lang: &str, alias: &str) -> String {
    say(lang, "fermento.separator", &[("alias", alias)])
}

/// Finds the separator (REQ-FER-1): the marker exactly once outside fences, inside one of the catalogue's renderings.
pub fn split(text: &str, alias: &str) -> Result<Parts, (Code, &'static str)> {
    let all = lines(text);
    let texts: Vec<&str> = all.iter().map(|l| l.1).collect();
    let fenced = fenced(&texts);
    let marks: Vec<usize> = (0..texts.len()).filter(|&i| !fenced[i] && texts[i] == MARKER).collect();
    let [at] = marks[..] else {
        return Err(if marks.is_empty() {
            (Code::SeparatorMissing, "no line is the marker outside a fenced code block")
        } else {
            (Code::SeparatorAmbiguous, "the marker stands on more than one line outside fenced code blocks")
        });
    };
    for lines in renderings(alias) {
        let Some(start) = lines.iter().position(|l| l == MARKER).and_then(|i| at.checked_sub(i)) else { continue };
        if texts.get(start..start + lines.len()).is_some_and(|t| t.iter().zip(&lines).all(|(a, b)| a == b)) {
            let end = start + lines.len();
            return Ok(Parts { input_end: all[start].0, after: all.get(end).map_or(text.len(), |l| l.0) });
        }
    }
    Err((Code::SeparatorDamaged, "the lines around the marker are none of the catalogue's renderings"))
}

/// A numbered item of the input (REQ-FER-2): its number, the digest of its text and the text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    pub number: u32,
    pub digest: String,
    pub text: String,
}

pub(crate) fn heading(line: &str) -> bool {
    let rest = line.trim_start_matches(' ');
    let hashes = rest.chars().take_while(|c| *c == '#').count();
    line.len() - rest.len() <= 3
        && (1..=6).contains(&hashes)
        && rest[hashes..].chars().next().is_none_or(|c| c == ' ' || c == '\t')
}

fn number(line: &str) -> Option<u32> {
    let digits = line.bytes().take_while(u8::is_ascii_digit).count();
    let after = line.get(digits..)?.strip_prefix('.')?;
    ((1..=9).contains(&digits) && after.chars().next().is_none_or(|c| c == ' ' || c == '\t'))
        .then(|| line[..digits].parse().ok())?
}

/// The points of `input`, the text above the separator (REQ-FER-2): the top-level numbered items outside fences, each
/// running to the next point or heading.
pub fn points(input: &str) -> Vec<Point> {
    let all: Vec<&str> = lines(input).into_iter().map(|l| l.1).collect();
    let fenced = fenced(&all);
    let (mut out, mut open): (Vec<Point>, Option<(u32, Vec<&str>)>) = (Vec::new(), None);
    let mut close = |open: &mut Option<(u32, Vec<&str>)>| {
        if let Some((number, mut text)) = open.take() {
            while text.last().is_some_and(|l| l.trim().is_empty()) {
                text.pop();
            }
            let text = text.join("\n");
            out.push(Point { number, digest: sha256_hex(text.as_bytes()), text });
        }
    };
    for (i, line) in all.iter().enumerate() {
        if !fenced[i] && (heading(line) || number(line).is_some()) {
            close(&mut open);
            open = number(line).map(|n| (n, vec![*line]));
        } else if let Some((_, text)) = &mut open {
            text.push(line);
        }
    }
    close(&mut open);
    out
}

/// The unit kinds an edit replaces whole (REQ-FER-6): the formula's is its whole text (formula 1a).
const UNITS: &[&str] = &["opening", "section", "point", "rule", "formula"];
const TARGETS: &[&str] = &["alambique", "rules", "formula"];
const POINT_REF: T = T::Obj(&[("number", T::Count), ("digest", T::Hex(64))]);
const UNIT: T = T::Obj(&[("kind", T::One(UNITS)), ("?n", T::Count), ("?m", T::Count), ("text", T::Nullable(&T::Text))]);
const KEY: T = T::Obj(&[("point", T::Text), ("key", T::Text)]);
const EDIT: T = T::Obj(&[
    ("target", T::One(TARGETS)),
    ("base", T::Hex(64)),
    ("units", T::List(&UNIT)),
    ("keys", T::List(&KEY)),
    ("result", T::Hex(64)),
]);
const RECOMMENDATION: T = T::Obj(&[("point", T::Text), ("clarify", T::Text), ("destination", T::Text)]);
const SWEPT: T = T::Obj(&[("number", T::Count), ("digest", T::Hex(64)), ("recommendation", RECOMMENDATION)]);
const SKILLS: T =
    T::Obj(&[("tool", T::Text), ("listed", T::Bool), ("names", T::List(&T::Text)), ("shadows", T::List(&T::Text))]);
const HEAD: T = T::Obj(&[("seq", T::Count), ("digest", T::Hex(64))]);
const SWEEP: T = T::Obj(&[
    ("input", T::Hex(64)),
    ("points", T::List(&SWEPT)),
    ("heads", T::Map(&HEAD)),
    ("skills", T::List(&SKILLS)),
]);
const RECORDED: T = T::Obj(&[
    ("id", T::Id('p')),
    ("point", POINT_REF),
    ("target", T::One(TARGETS)),
    ("base", T::Hex(64)),
    ("units", T::List(&UNIT)),
    ("keys", T::List(&KEY)),
    ("result", T::Hex(64)),
]);
const BASELINE: T = T::Obj(&[("digest", T::Hex(64))]);
const HUNK: T =
    T::Obj(&[("from", T::Count), ("removed", T::Nullable(&T::List(&T::Value))), ("added", T::List(&T::Value))]);
const DIRECT: T = T::Obj(&[("old", T::Hex(64)), ("new", T::Hex(64)), ("difference", HUNK)]);
const IMPORTED: T = T::Obj(&[("commit", T::Hex(40)), ("digest", T::Hex(64)), ("alias", T::Text)]);
const APPROVED: T = T::Obj(&[("old", T::Nullable(&T::Hex(64))), ("digest", T::Hex(64))]);
const CUT: T =
    T::Obj(&[("cut", T::Text), ("phase", T::Text), ("title", T::Text), ("reported", T::Bool), ("formula", T::Hex(64))]);
const INCORPORATED: T = T::Obj(&[("id", T::Id('p')), ("result", T::Hex(64)), ("?rules", T::Map(&T::Text))]);
const WITHDRAWN: T = T::Obj(&[("id", T::Id('p')), ("why", T::One(&["point-gone"]))]);
const DISCARDED: T = T::Obj(&[("id", T::Id('p'))]);
/// The integrator's answer to a sweep (REQ-FER-3), the file `sweep --answer` reads.
pub(crate) const ANSWER: T = T::Obj(&[
    ("agent", T::Obj(&[("tool", T::Text), ("provider", T::Text), ("model", T::Text)])),
    (
        "points",
        T::List(&T::Obj(&[
            ("number", T::Count),
            ("digest", T::Hex(64)),
            ("recommendation", RECOMMENDATION),
            ("?edit", EDIT),
        ])),
    ),
]);

fn invalid(why: impl Into<String>) -> Error {
    refuse(Code::AnswerInvalid, None, why)
}

/// An edit's content beyond its shape: at least one unit, each of a kind its target takes with the numbers its kind
/// needs and a text (a rule alone may be removed, with `null`), no unit twice; the formula's one unit is its text.
pub(crate) fn edit(e: &Value) -> Result<(), Error> {
    let units = e["units"].as_array().map(Vec::as_slice).unwrap_or_default();
    let (rules, formula) = (e["target"] == "rules", e["target"] == "formula");
    let mut seen = BTreeSet::new();
    for u in units {
        let (kind, n, m) = (u["kind"].as_str().unwrap_or_default(), u.get("n"), u.get("m"));
        let numbers = match kind {
            "opening" | "formula" => n.is_none() && m.is_none(),
            "point" => n.is_some() && m.is_some(),
            _ => n.is_some() && m.is_none(),
        };
        let key = (kind, n.and_then(Value::as_u64), m.and_then(Value::as_u64));
        let wrong = (kind == "rule") != rules || (kind == "formula") != formula;
        if !numbers || wrong || (u["text"].is_null() && kind != "rule") || !seen.insert(key) {
            return Err(invalid(format!(
                "units: {u} is not a unit of the {} once",
                e["target"].as_str().unwrap_or("")
            )));
        }
    }
    if units.is_empty() { Err(invalid("units: at least one")) } else { Ok(()) }
}

fn strings(v: &Value, out: &mut Vec<String>) {
    match v {
        Value::String(s) => out.push(s.clone()),
        Value::Array(a) => a.iter().for_each(|v| strings(v, out)),
        Value::Object(o) => o.values().for_each(|v| strings(v, out)),
        _ => {}
    }
}

/// Holds a payload of a fermento's kind to its shape (`RECORD_VERSION_UNKNOWN`) and its content (`ANSWER_INVALID`);
/// other kinds pass. Content: a sweep names each point once, an edit is well formed, and no text holds a line that is
/// the separator's marker, which would make the next separator ambiguous.
pub fn check(kind: &str, p: &Value) -> Result<(), Error> {
    let schema = match kind {
        "sweep-recorded" => &SWEEP,
        "proposal-recorded" => &RECORDED,
        "proposal-withdrawn" => &WITHDRAWN,
        "proposal-discarded" => &DISCARDED,
        "proposal-incorporated" => &INCORPORATED,
        "alambique-baseline" => &BASELINE,
        "direct-edit-approved" => &DIRECT,
        "formula-imported" => &IMPORTED,
        "formula-approved" => &APPROVED,
        "cut-recorded" => &CUT,
        _ => return Ok(()),
    };
    content(schema, kind, p)?;
    match kind {
        "sweep-recorded" => numbers(&p["points"]),
        "proposal-recorded" => edit(p),
        _ => Ok(()),
    }
}

/// Whether `kind`'s payload has its shape (schema 1 refuses it `RECORD_VERSION_UNKNOWN` otherwise); its content is
/// held on replay, with the proposal's state.
pub(crate) fn shape(kind: &str, p: &Value) -> bool {
    check(kind, p).map_or_else(|e| e.code() != Some(Code::RecordVersionUnknown), |_| true)
}

/// The answer file's content: its shape and the content of [`check`], and each point named once; every failure is
/// `ANSWER_INVALID`.
pub fn check_answer(v: &Value) -> Result<(), Error> {
    // At a command a file of the wrong shape is as invalid as one of the wrong content.
    content(&ANSWER, "answer", v).map_err(|e| match e {
        Error::Refused { code: Code::RecordVersionUnknown, path, detail } => {
            Error::Refused { code: Code::AnswerInvalid, path, detail }
        }
        other => other,
    })?;
    numbers(&v["points"])?;
    v["points"].as_array().into_iter().flatten().filter_map(|p| p.get("edit")).try_for_each(edit)
}

fn content(schema: &T, what: &str, v: &Value) -> Result<(), Error> {
    let mut content = None;
    fit(schema, v, "payload", &mut content)
        .map_err(|at| refuse(Code::RecordVersionUnknown, None, format!("{what}: {at}: shape")))?;
    if let Some(at) = content {
        return Err(invalid(format!("{what}: {at} is not valid")));
    }
    let mut texts = Vec::new();
    strings(v, &mut texts);
    if texts.iter().any(|t| t.lines().any(|l| l == MARKER)) {
        return Err(invalid(format!("{what}: a text holds the separator's marker as a line")));
    }
    Ok(())
}

fn numbers(points: &Value) -> Result<(), Error> {
    let mut seen = BTreeSet::new();
    match points.as_array().into_iter().flatten().all(|p| seen.insert(p["number"].as_u64())) {
        true => Ok(()),
        false => Err(invalid("points: a number is given twice")),
    }
}

/// A proposal as the record derives it (section 4): where it was recorded and its state.
#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: String,
    pub alias: String,
    pub state: &'static str,
    pub payload: Value,
}

fn bad(e: &Event, why: impl Into<String>) -> Error {
    refuse(Code::RecordTransitionInvalid, Some(&e.path), why)
}

/// Every proposal of the record and its state, held to the table: recorded once, in a fermento's stream; withdrawn or
/// discarded only from pending, in that same stream; incorporated only from pending, in `alambique`, `rules` or
/// `formula`.
pub fn proposals(record: &Replay) -> Result<BTreeMap<String, Proposal>, Vec<Error>> {
    let (mut out, mut errors) = (BTreeMap::new(), Vec::new());
    let fermentos = record.streams.iter().filter(|(s, _)| stream_kind(s) == Some(StreamKind::Fermento));
    let decided = record.streams.iter().filter(|(s, _)| matches!(s.as_str(), "alambique" | "rules" | "formula"));
    for (stream, events) in fermentos.chain(decided) {
        let alias = stream.strip_prefix("fermento-").unwrap_or_default();
        for e in events.iter().filter(|e| e.kind.starts_with("proposal-") || e.kind == "sweep-recorded") {
            let step = |state: Option<&str>| table::step("proposal", state, &e.kind).map_err(|x| bad(e, x.detail()));
            let mut apply = || -> Result<(), Error> {
                check(&e.kind, &e.value["payload"]).map_err(|x| match x.code() {
                    Some(Code::AnswerInvalid) => bad(e, x.detail()),
                    _ => x,
                })?;
                let id = e.value["payload"]["id"].as_str().unwrap_or_default().to_owned();
                match (e.kind.as_str(), out.get_mut(&id)) {
                    ("sweep-recorded", _) => {}
                    ("proposal-recorded", None) => {
                        let state = step(None)?.to;
                        let payload = e.value["payload"].clone();
                        out.insert(id.clone(), Proposal { id, alias: alias.to_owned(), state, payload });
                    }
                    ("proposal-recorded", Some(_)) => return Err(bad(e, format!("{id} is recorded twice"))),
                    (_, None) => return Err(bad(e, format!("{id} is no proposal"))),
                    (kind, Some(p)) => {
                        if kind != "proposal-incorporated" && p.alias != alias {
                            return Err(bad(e, format!("{id} belongs to another maintainer's fermento")));
                        }
                        p.state = step(Some(p.state))?.to;
                    }
                }
                Ok(())
            };
            errors.extend(apply().err());
        }
    }
    if errors.is_empty() { Ok(out) } else { Err(errors) }
}

/// The last sweep of `alias`'s fermento, if there is one.
pub fn last_sweep<'a>(record: &'a Replay, alias: &str) -> Option<&'a Event> {
    record.streams.get(&format!("fermento-{alias}"))?.iter().rev().find(|e| e.kind == "sweep-recorded")
}

/// Why `alias`'s last sweep did not see the end of `tasks` (task streams, all ended): it saw no head of one that is its
/// last event (REQ-TASK-6); `None` when it saw them all, or there is no task.
pub fn unswept(record: &Replay, alias: &str, tasks: &[&String]) -> Option<String> {
    let heads = last_sweep(record, alias).map(|s| &s.value["payload"]["heads"]);
    tasks.iter().find_map(|t| {
        let last = record.streams[*t].last()?;
        let seen = heads.is_some_and(|h| h[t.as_str()] == serde_json::json!({"seq": last.seq, "digest": last.digest}));
        (!seen).then(|| format!("the last sweep of {alias} predates the end of {}", &t[5..]))
    })
}

/// The first sweep of `alias` that saw the end of every one of `tasks` (see [`unswept`]): the moment from which the
/// stage was closable as far as that fermento goes; `None` when none did, or there is no task.
pub fn first_complete<'a>(record: &'a Replay, alias: &str, tasks: &[&String]) -> Option<&'a Event> {
    let events = record.streams.get(&format!("fermento-{alias}"))?;
    let saw = |s: &Event| {
        let heads = &s.value["payload"]["heads"];
        tasks.iter().all(|t| {
            let last = record.streams[*t].last();
            last.is_some_and(|l| heads[t.as_str()] == serde_json::json!({"seq": l.seq, "digest": l.digest}))
        })
    };
    events.iter().filter(|e| e.kind == "sweep-recorded").find(|s| saw(s)).filter(|_| !tasks.is_empty())
}

/// Whether a pending proposal no longer matches the input as its last sweep saw it (section 4): the point with its
/// number is gone or its text differs; the same words under another number do not count.
pub fn stale(p: &Proposal, sweep: Option<&Event>) -> bool {
    let swept = sweep.map(|s| s.value["payload"]["points"].as_array().cloned().unwrap_or_default());
    let held = swept.is_some_and(|pts| {
        pts.iter().any(|q| q["number"] == p.payload["point"]["number"] && q["digest"] == p.payload["point"]["digest"])
    });
    !held
}

/// A fence for `text` that no line of it can close: backticks, one more than its longest run, at least three.
fn fence(text: &str) -> String {
    let longest = text.split(|c| c != '`').map(str::len).max().unwrap_or(0);
    "`".repeat((longest + 1).max(3))
}

/// The answer zone for `alias` generated from the record alone (REQ-FER-4, REQ-FER-8, PD-42), in the catalogue's
/// language `lang`: the last sweep's recommendations, the proposals still waiting with their state and the commands
/// that decide them, and the skills the tools load from outside the project.
pub fn answer(record: &Replay, alias: &str, lang: &str) -> String {
    let t = |key: &str, params: &[(&str, &str)]| say(lang, &format!("fermento.{key}"), params);
    let sweep = last_sweep(record, alias);
    let waiting: Vec<Proposal> = {
        let all = proposals(record).unwrap_or_default();
        let mut mine: Vec<Proposal> = all.into_values().filter(|p| p.alias == alias && p.state == "pending").collect();
        mine.sort_by_key(|p| (p.payload["point"]["number"].as_u64(), p.id.clone()));
        mine
    };
    let mut out = format!("## {}\n\n", t("heading", &[]));
    let payload = sweep.map(|s| &s.value["payload"]);
    let empty = Vec::new();
    let recommendations = payload.and_then(|p| p["points"].as_array()).unwrap_or(&empty);
    let skills = payload.and_then(|p| p["skills"].as_array()).unwrap_or(&empty);
    if recommendations.is_empty() && waiting.is_empty() && skills.is_empty() {
        return out + &t("none", &[]) + "\n";
    }
    for r in recommendations {
        let (n, rec) = (r["number"].to_string(), &r["recommendation"]);
        let s = |k: &str| rec[k].as_str().unwrap_or_default();
        out += &format!(
            "**{}**\n\n{}\n\n{}\n\n",
            t("point", &[("number", &n), ("point", s("point"))]),
            s("clarify"),
            t("destination", &[("destination", s("destination"))])
        );
    }
    if !waiting.is_empty() {
        out += &format!("### {}\n\n", t("proposals", &[]));
    }
    for p in &waiting {
        let n = p.payload["point"]["number"].to_string();
        let target = t(&format!("target.{}", p.payload["target"].as_str().unwrap_or_default()), &[]);
        let state = t(&format!("state.{}", if stale(p, sweep) { "stale" } else { "pending" }), &[]);
        out += &format!("**{}**\n\n", t("proposal", &[("number", &n), ("state", &state), ("target", &target)]));
        if !stale(p, sweep) {
            out += &format!("{}\n\n    fabrewk approve {}\n\n", t("approve", &[]), p.id);
        }
        out += &format!("{}\n\n    fabrewk discard {}\n\n", t("discard", &[]), p.id);
        for u in p.payload["units"].as_array().into_iter().flatten() {
            let (kind, num) = (u["kind"].as_str().unwrap_or_default(), |k: &str| u[k].to_string());
            let label = t(&format!("unit.{kind}"), &[("n", &num("n")), ("m", &num("m"))]);
            match u["text"].as_str() {
                Some(text) => out += &format!("{label}\n\n{f}\n{text}\n{f}\n\n", f = fence(text)),
                None => out += &format!("{label}: {}\n\n", t("removed", &[])),
            }
        }
        for k in p.payload["keys"].as_array().into_iter().flatten() {
            out += &format!(
                "{}\n\n",
                t(
                    "key",
                    &[
                        ("point", k["point"].as_str().unwrap_or_default()),
                        ("key", k["key"].as_str().unwrap_or_default())
                    ]
                )
            );
        }
        out += &format!("{}\n\n", t("reference", &[("id", &p.id)]));
    }
    if !skills.is_empty() {
        out += &format!("### {}\n\n{}\n\n", t("skills", &[]), t("skills-about", &[]));
    }
    for s in skills {
        let names: Vec<&str> = s["names"].as_array().into_iter().flatten().filter_map(Value::as_str).collect();
        let tool = s["tool"].as_str().unwrap_or_default();
        out += &match (s["listed"].as_bool(), names.is_empty()) {
            (Some(true), false) => format!("- {}\n", t("skill-names", &[("tool", tool), ("names", &names.join(", "))])),
            (Some(true), true) => format!("- {}\n", t("skill-none", &[("tool", tool)])),
            _ => format!("- {}\n", t("skill-unknown", &[("tool", tool)])),
        };
        for n in s["shadows"].as_array().into_iter().flatten().filter_map(Value::as_str) {
            out += &format!("  - {}\n", t("skill-shadow", &[("tool", tool), ("name", n)]));
        }
    }
    out
}

/// The names of the skills a folder holds: its subdirectories and links, by name, never opened; an absent folder
/// holds none. At most 512, in order.
pub fn names(dir: &Path) -> std::io::Result<BTreeSet<String>> {
    let entries = match std::fs::read_dir(dir) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(BTreeSet::new()),
        other => other?,
    };
    let mut out = BTreeSet::new();
    for entry in entries {
        let entry = entry?;
        let name = entry.file_name().into_string().ok().filter(|n| !n.starts_with('.'));
        if let Some(name) = name.filter(|_| entry.file_type().is_ok_and(|t| t.is_dir() || t.is_symlink())) {
            out.insert(name);
        }
    }
    Ok(out.into_iter().take(512).collect())
}

/// The skills the tools load from outside the project (REQ-FER-10): for each built-in module that declares folders
/// under the user's home, the names it finds, whether every folder could be listed, and which names an approved skill
/// of `.brewstillery/skills/` also bears. Nothing is opened or changed.
pub fn skills(modules: &[Program], home: Option<&Path>, domain: &Path) -> Value {
    let approved = if crate::code::unlinked(domain, "skills") {
        names(&domain.join("skills")).unwrap_or_default()
    } else {
        BTreeSet::new()
    };
    let mut out = Vec::new();
    for module in modules.iter().filter(|m| !m.skills.is_empty()) {
        let (mut found, mut listed) = (BTreeSet::new(), true);
        for dir in module.skills {
            match home.map(|h| names(&h.join(dir))) {
                Some(Ok(n)) => found.extend(n),
                _ => listed = false,
            }
        }
        let shadows: Vec<&String> = found.intersection(&approved).collect();
        out.push(json!({"tool": module.executable, "listed": listed, "names": found, "shadows": shadows}));
    }
    json!(out)
}

/// The text of a fermento at `path` (REQ-FER-9): a regular file reached by no link, valid UTF-8.
pub fn read(domain: &Path, alias: &str) -> Result<String, Error> {
    let path = domain.join(alias).join("fermento.md");
    let missing = |why: &str| refuse(Code::FermentoMissing, Some(&path), why.to_owned());
    match std::fs::symlink_metadata(&path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Err(missing("there is no fermento.md")),
        Err(e) => return Err(crate::code::io_at(&path)(e)),
        Ok(m) if !m.is_file() || !crate::code::unlinked(domain, &format!("{alias}/fermento.md")) => {
            return Err(missing("not a regular file, or reached through a link"));
        }
        Ok(_) => {}
    }
    let bytes = std::fs::read(&path).map_err(crate::code::io_at(&path))?;
    String::from_utf8(bytes).map_err(|_| refuse(Code::SeparatorDamaged, Some(&path), "not valid UTF-8"))
}

/// A fermento read and split: its text and where its parts begin and end.
pub fn open(domain: &Path, alias: &str) -> Result<(String, Parts), Error> {
    let text = read(domain, alias)?;
    let path = domain.join(alias).join("fermento.md");
    let parts = split(&text, alias).map_err(|(code, why)| refuse(code, Some(&path), why))?;
    Ok((text, parts))
}

/// The digest of a fermento's input: everything above the separator's block.
pub fn input_digest(text: &str, parts: &Parts) -> String {
    sha256_hex(&text.as_bytes()[..parts.input_end])
}

/// The fermento as it stands, split, once its input still has the digest `digest`; `INPUT_CHANGED` otherwise.
pub fn current(domain: &Path, alias: &str, digest: &str) -> Result<(String, Parts), Error> {
    let (text, parts) = open(domain, alias)?;
    if input_digest(&text, &parts) != digest {
        let path = domain.join(alias).join("fermento.md");
        return Err(refuse(Code::InputChanged, Some(&path), "the input changed while the command ran"));
    }
    Ok((text, parts))
}

/// The fermento with its answer replaced by `answer` and its input as it was (REQ-FER-3); `INPUT_CHANGED` when the
/// input's digest is no longer `digest`.
pub fn compose(domain: &Path, alias: &str, digest: &str, answer: &str) -> Result<Vec<u8>, Error> {
    let (text, parts) = current(domain, alias, digest)?;
    let head = &text[..parts.after];
    Ok(format!("{head}{}\n{answer}", if head.ends_with('\n') { "" } else { "\n" }).into_bytes())
}

/// A sweep's payload from what it saw (REQ-FER-3, REQ-FER-10).
pub fn sweep_payload(input: &str, points: &Value, heads: Value, skills: Value) -> Value {
    json!({"input": input, "points": points, "heads": heads, "skills": skills})
}
