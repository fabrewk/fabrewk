//! `fabrewk status [<id>]` (section 5; part 7d2, D-055 point 4): the record's health, the policy in force, the
//! publication mode and each stage's and task's state; with an identifier, that stage or task, its events and the
//! moves the table allows next. An absent record is a statement, not a refusal. It takes no lock and writes nothing
//! (REQ-VIEW-4); the whole view, with what waits for whom, is phase 8's.

use super::{COMMANDS, Cx, Failed, Out, Usage};
use crate::code::Code;
use crate::domain;
use crate::record::{Event, Publication, Replay, StreamKind, stream_kind};
use crate::{settings, table};
use serde_json::{Value, json};

pub(super) fn run(cx: &Cx, id: Option<&str>) -> Result<Out, Failed> {
    match id {
        Some(id) => subject(cx, id),
        None => Ok(overview(cx)),
    }
}

pub(crate) fn mode_name(m: Publication) -> &'static str {
    match m {
        Publication::Published => "published",
        Publication::OwnRepository => "own-repository",
        Publication::Unpublished => "unpublished",
    }
}

fn text_of<'a>(e: &'a Event, field: &str) -> &'a str {
    e.value["payload"][field].as_str().unwrap_or_default()
}

fn overview(cx: &Cx) -> Out {
    let mut doc = json!({
        "schema": "fabrewk.status/1", "record": "absent", "problems": [], "policy": null, "mode": null,
        "stages": [], "next": [],
    });
    let mut text = String::new();
    let mut next = vec![];
    match &cx.record {
        Err(errors) if errors.iter().all(|e| e.code() == Some(Code::RecordAbsent)) => {
            text += &cx.t("status.absent", &[])
        }
        Err(errors) => {
            doc["record"] = json!("broken");
            let problems: Vec<Value> = errors.iter().map(|e| cx.entry(e)).collect();
            text += &cx.t("status.broken", &[]);
            text += "\n";
            text += &problems.iter().map(|p| cx.entry_text(p, "  ")).collect::<String>();
            doc["problems"] = json!(problems);
            next.push("fabrewk check");
        }
        Ok(record) => {
            next.push("fabrewk check");
            if let Some(pending) = record_lines(cx, record, &mut doc, &mut text) {
                next.extend(pending.then_some("fabrewk config"));
            }
        }
    }
    text = text.trim_end().to_owned() + "\n";
    text += &cx.next(&next);
    doc["next"] = json!(next);
    Out::new(doc, text)
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
            cx.t("status.stage", &[("id", id), ("state", &cx.t(&format!("state.{state}"), &[])), ("goal", goal)])
        );
        let mut tasks = Vec::new();
        for (tname, tevents) in
            record.streams.iter().filter(|(n, e)| n.starts_with("task-") && text_of(&e[0], "stage") == id)
        {
            let (tid, tstate, tgoal) =
                (&tname["task-".len()..], states.get(tname).copied().unwrap_or_default(), text_of(&tevents[0], "goal"));
            let words = cx.t(&format!("state.{tstate}"), &[]);
            *text += &format!("  {}\n", cx.t("status.task", &[("id", tid), ("state", &words), ("goal", tgoal)]));
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
    let stream = match id.split_once('-') {
        Some(("t", _)) => format!("task-{id}"),
        Some(("s", _)) => format!("stage-{id}"),
        _ => id.to_owned(),
    };
    let unknown = || Failed::Usage(Usage { key: "usage.id", word: id.to_owned(), command: Some(&COMMANDS[0]) });
    let record = cx.record.as_ref().map_err(|_| unknown())?;
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
    let mut text =
        cx.t(&format!("status.{subject}"), &[("id", short), ("state", &state_words), ("goal", text_of(first, "goal"))]);
    text += "\n";
    if let Some(stage) = stage {
        text += &format!("{}\n", cx.t("status.in-stage", &[("stage", stage)]));
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
    let doc = json!({
        "schema": "fabrewk.subject/1", "id": short, "subject": subject, "state": state, "goal": text_of(first, "goal"),
        "lang": text_of(first, "lang"), "stage": stage, "events": list, "allowed": allowed,
    });
    Ok(Out::new(doc, text))
}
