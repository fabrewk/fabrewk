//! `fabrewk propose <file>` (sections 4 and 5; REQ-TASK-2, REQ-TASK-3, REQ-TASK-7, REQ-GATE-1, REQ-GATE-9; part
//! 7d3a): a stage and its tasks, or tasks added to a stage the record holds, from a JSON file the integrator wrote.
//! It grants nothing: anyone may run it, it takes no words, and Fabrewk is the events' actor; an extension's `propose`
//! request is planned alike, with the extension as their actor (REQ-EXT-3, part 9h). Each move's guards run
//! in the table's order: for a new stage `TASK_INVALID` and `CHECK_DUPLICATE`; for each task `STAGE_CLOSED`,
//! `TASK_INVALID`, `PROTECTED_PATH` and `PROTECTED_PATH_WILDCARD`.

use super::adopt::written_lines;
use super::write::{self, Plan};
use super::{Cx, Failed, Out};
use crate::code::{Code, Error, refuse};
use crate::record::{Draft, Replay, nonce, subject_stream};
use crate::settings::{self, Source, Val};
use crate::{table, task};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::Path;

/// The largest file `propose` reads.
const MAX_FILE: u64 = 1 << 20;

fn invalid(path: &Path, why: impl Into<String>) -> Error {
    refuse(Code::TaskInvalid, Some(path), why)
}

/// The file as JSON: a regular file outside every Fabrewk domain, at most [`MAX_FILE`] bytes, else refused with `code`
/// (`TASK_INVALID` for `propose`'s file, `ANSWER_INVALID` for `sweep`'s).
pub(super) fn read(path: &Path, code: Code) -> Result<Value, Error> {
    let invalid = |path: &Path, why: &str| refuse(code, Some(path), why.to_owned());
    let meta = std::fs::metadata(path).map_err(|_| invalid(path, "cannot be read"))?;
    if !meta.is_file() || crate::code::in_a_domain(path) {
        return Err(invalid(path, "not a regular file outside every Fabrewk domain"));
    }
    if meta.len() > MAX_FILE {
        return Err(invalid(path, "larger than 1 MiB"));
    }
    let bytes = std::fs::read(path).map_err(|_| invalid(path, "cannot be read"))?;
    serde_json::from_slice(&bytes).map_err(|e| invalid(path, &format!("not JSON (line {})", e.line())))
}

/// A payload's layer of settings, as `effective` takes it.
pub(super) fn layer(p: &Value) -> BTreeMap<String, Val> {
    let entries = p["settings"].as_object().into_iter().flatten();
    entries.filter_map(|(k, v)| Some((k.clone(), task::value(v)?))).collect()
}

/// A payload's content and shape at a command: any failure is `TASK_INVALID`, naming the file (section 3).
fn content(path: &Path, kind: &str, p: &Value) -> Result<(), Error> {
    task::check(kind, p).map_err(|e| invalid(path, e.detail()))
}

/// What `propose` reports: the stage, whether it is new, and each task with its name in the file.
pub(super) struct Proposed {
    stage: (String, bool, String),
    tasks: Vec<(String, Option<String>, String)>,
}

/// The identifiers a command's plan has drawn so far.
pub(super) type Taken = std::cell::RefCell<std::collections::BTreeSet<String>>;

/// The events of `file` on `record`, with `actor` as theirs and none of the identifiers in `taken`, which it adds to;
/// a refusal names `path`.
pub(super) fn plan(
    cx: &Cx,
    path: &Path,
    file: &Value,
    record: &Replay,
    actor: &Value,
    taken: &Taken,
) -> Result<Plan<Proposed>, Failed> {
    let policy = write::state(cx, record)?.policy;
    let o = file.as_object().filter(|o| o.len() == 2 && o.contains_key("stage") && o.contains_key("tasks"));
    let o = o.ok_or_else(|| invalid(path, "the file holds exactly stage and tasks"))?;
    let tasks = o["tasks"].as_array().filter(|t| !t.is_empty());
    let tasks = tasks.ok_or_else(|| invalid(path, "tasks: a list of one task or more"))?;
    let draft = |kind: &str, payload: Value| Draft {
        kind: kind.into(),
        actor: actor.clone(),
        policy: policy.digest.clone(),
        instance: String::new(),
        say: None,
        payload,
    };
    // A creation's identifier is the first 8 digits of its digest, so two may meet: a nonce whose identifier the
    // record or this file already holds is drawn again, and after 8 draws the command refuses, with nothing written
    // (REQ-REG-1, P7D3A-15).
    let fresh = |kind: &str, p: Value| -> Result<Value, Error> {
        for _ in 0..8 {
            let n = nonce().map_err(|e| Error::Io { path: "/dev/urandom".into(), error: e })?;
            let mut p = p.clone();
            p.as_object_mut().map(|o| o.insert("nonce".into(), json!(n)));
            let stream = subject_stream(kind, &p).unwrap_or_default();
            if !record.streams.contains_key(&stream) && taken.borrow_mut().insert(stream) {
                return Ok(p);
            }
        }
        Err(refuse(Code::RecordIdCollision, Some(path), format!("{kind}: eight nonces drawn gave identifiers taken")))
    };
    let mut events = Vec::new();
    let (stream, new, goal, stage_layer) = match &o["stage"] {
        Value::String(id) => {
            let stream = format!("stage-{id}");
            let events = record.streams.get(&stream).filter(|_| crate::record::is_id(id, 's'));
            let events = events.ok_or_else(|| invalid(path, format!("stage: {id} is no stage of the record")))?;
            let states = table::current(record)?;
            if states.get(&stream) == Some(&"closed") {
                return Err(refuse(Code::StageClosed, Some(path), format!("stage {id} is closed")).into());
            }
            let first = &events[0].value["payload"];
            (stream, false, first["goal"].as_str().unwrap_or_default().to_owned(), layer(first))
        }
        stage => {
            let payload = fresh("stage-proposed", stage.clone())?;
            if stage.get("nonce").is_some() {
                return Err(invalid(path, "stage.nonce: drawn by propose, never given").into());
            }
            content(path, "stage-proposed", &payload)?;
            task::duplicates(&payload["checks"], &settings::effective(&policy, None, None, &[], &[])?)
                .map_err(|e| refuse(Code::CheckDuplicate, Some(path), e.detail()))?;
            let stream = subject_stream("stage-proposed", &payload).unwrap_or_default();
            let goal = payload["goal"].as_str().unwrap_or_default().to_owned();
            let l = layer(&payload);
            events.push((stream.clone(), draft("stage-proposed", payload)));
            (stream, true, goal, l)
        }
    };
    let stage_id = stream["stage-".len()..].to_owned();
    let mut names: BTreeMap<String, String> = BTreeMap::new();
    let mut out = Vec::new();
    for (i, t) in tasks.iter().enumerate() {
        let at = |why: String| invalid(path, format!("tasks[{i}]: {why}"));
        let mut t = t.as_object().cloned().ok_or_else(|| at("not an object".into()))?;
        if t.contains_key("nonce") || t.contains_key("stage") {
            return Err(at("nonce and stage are given by propose, never by the file".into()).into());
        }
        let name = match t.remove("name") {
            None => None,
            Some(Value::String(n)) if !n.trim().is_empty() && !crate::record::is_id(&n, 't') => Some(n),
            Some(_) => return Err(at("name: a text that is not a task's identifier".into()).into()),
        };
        // A dependency is a task of the record, or a task named earlier in the file.
        if let Some(Value::Array(deps)) = t.get_mut("dependencies") {
            for d in deps.iter_mut() {
                let Some(s) = d.as_str() else { continue };
                if let Some(id) = names.get(s) {
                    *d = json!(id);
                } else if !record.streams.contains_key(&format!("task-{s}")) {
                    return Err(at(format!("dependencies: {s} is no task of the record or of the file above")).into());
                }
            }
        }
        t.insert("stage".into(), json!(stage_id));
        let payload = fresh("task-proposed", Value::Object(t))?;
        content(path, "task-proposed", &payload)?;
        let allowed: Vec<&str> =
            payload["allowed"].as_array().into_iter().flatten().filter_map(Value::as_str).collect();
        task::lane(&allowed)
            .map_err(|e| refuse(Code::ProtectedPath, Some(path), format!("tasks[{i}]: {}", e.detail())))?;
        let id = subject_stream("task-proposed", &payload).unwrap_or_default()["task-".len()..].to_owned();
        // The protected paths in force for the task, with its stage's and its own layer; their widenings are named at
        // the authorization, not here.
        let layers =
            [(Source::Stage(stage_id.clone()), stage_layer.clone()), (Source::Task(id.clone()), layer(&payload))];
        let widen: Vec<String> = layers.iter().flat_map(|(_, l)| l.keys().cloned()).collect();
        let eff = settings::effective(&policy, None, None, &layers, &widen)
            .map_err(|e| at(format!("settings: {}", e.detail())))?;
        let protected = crate::candidate::protected_paths(&eff, &product(cx)).map_err(Failed::from)?;
        task::wildcards(&allowed, &protected)
            .map_err(|e| refuse(Code::ProtectedPathWildcard, Some(path), format!("tasks[{i}]: {}", e.detail())))?;
        if let Some(n) = &name
            && names.insert(n.clone(), id.clone()).is_some()
        {
            return Err(at(format!("name: {n} is given twice")).into());
        }
        out.push((id.clone(), name, payload["goal"].as_str().unwrap_or_default().to_owned()));
        events.push((format!("task-{id}"), draft("task-proposed", payload)));
    }
    Ok(Plan {
        events,
        signer: None,
        sealed: false,
        view: None,
        alambique: None,
        then: None,
        out: Proposed { stage: (stage_id, new, goal), tasks: out },
    })
}

/// What was proposed, as the person reads it: the stage, then each task.
pub(super) fn lines(cx: &Cx, proposed: &Proposed) -> String {
    let (id, new, goal) = &proposed.stage;
    let key = if *new { "propose.stage" } else { "propose.added" };
    let mut text = cx.t(key, &[("id", id), ("goal", &super::tasks::verbatim(goal, "  "))]) + "\n";
    for (tid, _, goal) in &proposed.tasks {
        text +=
            &format!("  {}\n", cx.t("propose.task", &[("id", tid), ("goal", &super::tasks::verbatim(goal, "    "))]));
    }
    text
}

pub(super) fn run(cx: &Cx, file: &str) -> Result<Out, Failed> {
    let path = cx.root.join(file);
    let value = read(&path, Code::TaskInvalid)?;
    let fabrewk = json!({"kind": "fabrewk"});
    let (proposed, written) = write::run(cx, |record| plan(cx, &path, &value, record, &fabrewk, &Taken::default()))?;
    let (id, new, _) = &proposed.stage;
    let mut text = lines(cx, &proposed);
    let tasks: Vec<Value> =
        proposed.tasks.iter().map(|(tid, name, goal)| json!({"id": tid, "name": name, "goal": goal})).collect();
    text += &format!("{}\n", cx.t("propose.grants", &[]));
    let mut doc = json!({"schema": "fabrewk.proposed/2", "stage": {"id": id, "new": new}, "tasks": tasks});
    text += &written_lines(cx, &written, &mut doc);
    let mut next = vec![format!("fabrewk status {id}")];
    next.extend(proposed.tasks.iter().map(|t| format!("fabrewk status {}", t.0)));
    text += &cx.next(&next.iter().map(String::as_str).collect::<Vec<_>>());
    Ok(Out::new(doc, text))
}

/// The product's repository, where the command runs: its Git configuration names the hooks directory a guard reads.
pub(super) fn product(cx: &Cx) -> crate::record::Repo {
    crate::record::Repo { root: cx.root.clone(), prefix: std::path::PathBuf::new() }
}
