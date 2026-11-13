//! `fabrewk status [<id>]` (section 5; part 7d2, D-055 point 4): the record's health, the policy in force, the
//! publication mode and each stage's and task's state; with an identifier, that stage or task, its events and the
//! moves the table allows next. An absent record is a statement, not a refusal. It takes no lock and writes nothing
//! (REQ-VIEW-4); the whole view, with what waits for whom, is phase 8's.

use super::{COMMANDS, Cx, Failed, Out, Usage};
use crate::code::Code;
use crate::domain;
use crate::identity;
use crate::record::{Event, Replay, StreamKind, stream_kind};
use crate::view::{self, mode_name};
use crate::{settings, table};
use serde_json::{Value, json};

pub(super) fn run(cx: &Cx, id: Option<&str>) -> Result<Out, Failed> {
    match id {
        Some(id) => subject(cx, id),
        None => overview(cx),
    }
}

fn text_of<'a>(e: &'a Event, field: &str) -> &'a str {
    e.value["payload"][field].as_str().unwrap_or_default()
}

fn overview(cx: &Cx) -> Result<Out, Failed> {
    let mut doc = json!({
        "schema": "fabrewk.status/3", "record": "absent", "problems": [], "policy": null, "mode": null, "local": null,
        "stages": [], "waiting": [], "next": [],
    });
    let mut text = String::new();
    let mut next: Vec<String> = vec![];
    match &cx.record {
        Err(errors) if errors.iter().all(|e| e.code() == Some(Code::RecordAbsent)) => {
            text += &cx.t("status.absent", &[])
        }
        // A record that cannot be read at all is an internal error, never a state.
        Err(errors) if errors.iter().any(|e| e.code().is_none()) => return Err(Failed::Refused(errors.clone())),
        Err(errors) => {
            doc["record"] = json!("broken");
            let problems: Vec<Value> = errors.iter().map(|e| cx.entry(e)).collect();
            text += &cx.t("status.broken", &[]);
            text += "\n";
            text += &problems.iter().map(|p| cx.entry_text(p, "  ")).collect::<String>();
            doc["problems"] = json!(problems);
            next.push("fabrewk check".into());
        }
        Ok(record) => {
            next.push("fabrewk check".into());
            if let Some(pending) = record_lines(cx, record, &mut doc, &mut text) {
                next.extend(pending.then(|| "fabrewk config".to_owned()));
            }
            next.extend(waiting(cx, record, &mut doc, &mut text));
        }
    }
    // No cell's directories are walked (REQ-RUN-3, P9E-1, P9E-7): only cells make directories in `local/tmp/`, and while
    // one is there, running or kept unread, what `local/tmp/` holds is unknown.
    let local = cx.domain.join("local");
    // An entry that cannot be examined counts as a cell's.
    let cell = |e: std::io::Result<std::fs::DirEntry>| e.and_then(|e| e.file_type()).map_or(true, |t| t.is_dir());
    let cells = std::fs::read_dir(local.join("tmp")).map(|mut d| d.any(cell));
    let (evidence, tmp) =
        (held(&local.join("evidence")), if cells.is_ok_and(|c| c) { None } else { held(&local.join("tmp")) });
    doc["local"] = json!({"evidence": evidence, "tmp": tmp});
    let words = |n: Option<u64>| {
        n.map_or_else(|| cx.t("status.size-unknown", &[]), |n| cx.t("status.size", &[("bytes", &n.to_string())]))
    };
    text = text.trim_end().to_owned() + "\n";
    text += &format!("{}\n", cx.t("status.local", &[("evidence", &words(evidence)), ("tmp", &words(tmp))]));
    text += &cx.next(&next.iter().map(String::as_str).collect::<Vec<_>>());
    doc["next"] = json!(next);
    Ok(Out::new(doc, text))
}

/// How many bytes the files under `dir` hold (REQ-DOM-5, S-3), following no link, however deep; an absent directory
/// holds none, and one that cannot be listed whole gives `None`.
fn held(dir: &std::path::Path) -> Option<u64> {
    if std::fs::symlink_metadata(dir).is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound) {
        return Some(0);
    }
    let (mut sum, mut left) = (0, vec![dir.to_path_buf()]);
    while let Some(path) = left.pop() {
        let meta = std::fs::symlink_metadata(&path).ok()?;
        if meta.is_dir() {
            for entry in std::fs::read_dir(&path).ok()? {
                left.push(entry.ok()?.path());
            }
        } else if meta.is_file() {
            sum += meta.len();
        }
    }
    Some(sum)
}

/// What waits and for whom (REQ-VIEW-1, PD-33), in the person's language; gives the commands for the person who
/// reads: a maestro's for a maestro, the integrator's for whoever is no maintainer, none for a maintainer with nothing
/// to decide.
fn waiting(cx: &Cx, record: &Replay, doc: &mut Value, text: &mut String) -> Vec<String> {
    let all = view::waits(record);
    let role = cx
        .policy
        .as_ref()
        .zip(cx.person.as_deref())
        .and_then(|(p, a)| identity::maintainers(&p.settings).into_iter().find(|m| m.alias == a).map(|m| m.role));
    *text += &format!("{}\n", cx.t("status.waiting", &[]));
    let (mut commands, mut list) = (Vec::new(), Vec::new());
    for w in &all {
        let (who, key) = match w.who {
            view::Who::Maestro => ("maestro", "status.waits-maestro"),
            view::Who::Integrator => ("integrator", "status.waits-integrator"),
        };
        let line = cx.t(&format!("view.wait.{}", w.what), &[("id", &w.id), ("detail", &w.detail), ("since", &w.since)]);
        *text += &format!("  {}\n", cx.t(key, &[("line", &super::tasks::verbatim(&line, "    "))]));
        let mine = match w.who {
            view::Who::Maestro => role.as_deref() == Some("maestro"),
            view::Who::Integrator => role.is_none(),
        };
        commands
            .extend(w.commands.iter().filter(|_| mine).filter(|c| !commands.contains(*c)).cloned().collect::<Vec<_>>());
        list.push(json!({"who": who, "what": w.what, "id": w.id, "since": w.since, "detail": w.detail}));
    }
    if all.is_empty() {
        *text += &format!("  {}\n", cx.t("status.waits-none", &[]));
    }
    doc["waiting"] = json!(list);
    commands
}

/// The lines of a record that replays; gives whether the project file has changes not adopted, if a policy is.
fn record_lines(cx: &Cx, record: &Replay, doc: &mut Value, text: &mut String) -> Option<bool> {
    let states = table::current(record).unwrap_or_default();
    let (streams, events) = (record.streams.len().to_string(), record.streams.values().map(Vec::len).sum::<usize>());
    doc["record"] = json!("replays");
    *text += &format!("{}\n", cx.t("status.replays", &[("streams", &streams), ("events", &events.to_string())]));
    let mut pending = None;
    match &cx.policy {
        None => *text += &format!("{}\n", cx.t("status.no-policy", &[])),
        Some(p) => {
            pending = settings::pending(p, &cx.domain).ok().map(|c| c.is_some());
            doc["policy"] = json!({"digest": p.digest, "pending": pending});
            *text += &format!("{}\n", cx.t("status.policy", &[("digest", &p.digest[..12])]));
            let line = match pending {
                Some(true) => "status.pending",
                Some(false) => "status.adopted",
                None => "status.pending-unknown",
            };
            *text += &format!("  {}\n", cx.t(line, &[]));
        }
    }
    match domain::publication(record).map(mode_name) {
        Some(m) => {
            doc["mode"] = json!(m);
            *text += &format!("{}\n", cx.t("status.mode", &[("mode", &cx.t(&format!("mode.{m}"), &[]))]));
        }
        None => *text += &format!("{}\n", cx.t("status.no-mode", &[])),
    }
    let mut stages = Vec::new();
    for (name, events) in record.streams.iter().filter(|(n, _)| stream_kind(n) == Some(StreamKind::Stage)) {
        let id = &name["stage-".len()..];
        let state = states.get(name).copied().unwrap_or_default();
        let goal = text_of(&events[0], "goal");
        *text += &format!(
            "{}\n",
            cx.t(
                "status.stage",
                &[
                    ("id", id),
                    ("state", &cx.t(&format!("state.{state}"), &[])),
                    ("goal", &super::tasks::verbatim(goal, "  "))
                ]
            )
        );
        let mut tasks = Vec::new();
        for (tname, tevents) in
            record.streams.iter().filter(|(n, e)| n.starts_with("task-") && text_of(&e[0], "stage") == id)
        {
            let (tid, tstate, tgoal) =
                (&tname["task-".len()..], states.get(tname).copied().unwrap_or_default(), text_of(&tevents[0], "goal"));
            let words = cx.t(&format!("state.{tstate}"), &[]);
            *text += &format!(
                "  {}\n",
                cx.t(
                    "status.task",
                    &[("id", tid), ("state", &words), ("goal", &super::tasks::verbatim(tgoal, "    "))]
                )
            );
            tasks.push(json!({"id": tid, "state": tstate, "goal": tgoal}));
        }
        stages.push(json!({"id": id, "state": state, "goal": goal, "tasks": tasks}));
    }
    if stages.is_empty() {
        *text += &format!("{}\n", cx.t("status.no-stages", &[]));
    }
    doc["stages"] = json!(stages);
    pending
}

/// The actor of an event, in words.
fn actor(cx: &Cx, e: &Event) -> (String, String) {
    let a = &e.value["actor"];
    let kind = a["kind"].as_str().unwrap_or_default();
    let s = |k: &str| a[k].as_str().unwrap_or_default();
    let role = cx.t(&format!("role.{}", s("role")), &[]);
    let words = cx.t(
        &format!("actor.{kind}"),
        &[("alias", s("alias")), ("role", &role), ("tool", s("tool")), ("name", s("name"))],
    );
    (kind.to_owned(), words)
}

fn subject(cx: &Cx, id: &str) -> Result<Out, Failed> {
    let stream = super::tasks::stream_of(id);
    let unknown = || Failed::Usage(Usage { key: "usage.id", word: id.to_owned(), command: Some(&COMMANDS[0]) });
    // A record that does not replay cannot say whether it holds the identifier: its state is shown instead.
    let record = match &cx.record {
        Ok(record) => record,
        Err(errors) if errors.iter().all(|e| e.code() == Some(Code::RecordAbsent)) => return Err(unknown()),
        Err(_) => return overview(cx),
    };
    let subject = match stream_kind(&stream) {
        Some(StreamKind::Stage) => "stage",
        Some(StreamKind::Task) => "task",
        _ => return Err(unknown()),
    };
    let events = record.streams.get(&stream).ok_or_else(unknown)?;
    let short = &stream[subject.len() + 1..];
    let state = table::current(record).ok().and_then(|s| s.get(&stream).copied()).unwrap_or_default();
    let (first, state_words) = (&events[0], cx.t(&format!("state.{state}"), &[]));
    let stage = (subject == "task").then(|| text_of(first, "stage"));
    let mut text = cx.t(
        &format!("status.{subject}"),
        &[("id", short), ("state", &state_words), ("goal", &super::tasks::verbatim(text_of(first, "goal"), "  "))],
    );
    text += "\n";
    if let Some(stage) = stage {
        text += &format!("{}\n", cx.t("status.in-stage", &[("stage", stage)]));
    }
    // A halted task says why it waits: the code, its catalogue reason and the halt's own words (PD-08).
    let halt = events.iter().rev().find(|e| e.kind == "task-halted").filter(|_| state == "halted");
    let halt = halt.map(|e| (text_of(e, "code"), text_of(e, "reason")));
    if let Some((code, reason)) = halt {
        let why = cx.t(&format!("code.{code}.reason"), &[]);
        text += &format!("{}\n", cx.t("status.halted", &[("code", code), ("why", &why)]));
        text += &format!("  {}\n", super::tasks::verbatim(reason, "  "));
    }
    // The last review, recorded with the limitation of a reviewer of the builder's own provider, says so (REQ-GATE-4).
    let review = events.iter().rfind(|e| e.kind == "review-recorded");
    if review.is_some_and(|e| e.value["payload"]["limited"] == true) {
        text += &format!("{}\n", cx.t("status.limited", &[]));
    }
    text += &format!("{}\n", cx.t("status.events", &[]));
    let mut list = Vec::new();
    for e in events {
        let (kind, words) = actor(cx, e);
        let at = e.value["at"].as_str().unwrap_or_default();
        let what = cx.t(&format!("kind.{}", e.kind), &[]);
        let (seq, line) = (
            e.seq.to_string(),
            cx.t("status.event", &[("what", &what), ("kind", &e.kind), ("by", &words), ("at", at)]),
        );
        text += &format!("  {seq}. {line}\n");
        list.push(json!({"seq": e.seq, "kind": e.kind, "actor": kind, "at": at}));
    }
    let allowed = table::allowed(subject, Some(state));
    let words: Vec<String> = allowed.iter().map(|k| format!("{} ({k})", cx.t(&format!("kind.{k}"), &[]))).collect();
    let moves = if words.is_empty() { cx.t("config.none", &[]) } else { words.join(", ") };
    text += &format!("{}\n", cx.t("status.allowed", &[("moves", &moves)]));
    // What an authorization would bind, with the digest `authorize` takes (REQ-GATE-7, PD-37), and the moves a person
    // may make, each printed without words (PD-32).
    let states = table::current(record).unwrap_or_default();
    let bound = super::tasks::bound(cx, record, &states, &stream);
    let mut next = Vec::new();
    if let Some(b) = &bound {
        text += &format!("\n{}{}\n", b.text, cx.t("bind.digest", &[("digest", &b.digest)]));
        next.push(format!("fabrewk authorize {short} --seen {}", b.digest));
    }
    for (kind, command) in [("run-started", "run"), ("task-rejected", "reject")] {
        if allowed.contains(&kind) {
            next.push(format!("fabrewk {command} {short}"));
        }
    }
    text += &cx.next(&next.iter().map(String::as_str).collect::<Vec<_>>());
    let shown = bound.map(|b| json!({"text": b.text, "digest": b.digest}));
    let doc = json!({
        "schema": "fabrewk.subject/3", "id": short, "subject": subject, "state": state, "goal": text_of(first, "goal"),
        "lang": text_of(first, "lang"), "stage": stage, "events": list, "allowed": allowed, "shown": shown,
        "next": next, "halt": halt.map(|(code, reason)| json!({"code": code, "reason": reason})),
    });
    Ok(Out::new(doc, text))
}
