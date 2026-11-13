//! The task commands (part 7d3b; sections 4 and 5; D-053 point 4, D-062, D-068): `authorize <stage|task> --seen
//! <digest> --say <words> [--widen <key>]… [--run]`, `reject <task> --say <words>` and `run <task|stage> --say
//! <words>`, each a maestro's signed move whose guards run in the order section 4's table lists them, after the move
//! itself (`MOVE_NOT_ALLOWED`); and what an authorization binds, which `status <id>` shows with the digest that
//! `authorize` takes with `--seen` (REQ-GATE-7, PD-37). `run` and `authorize --run` run the task past confinement
//! (part 9e, `super::run`).

use super::adopt::written_lines;
use super::propose::layer;
use super::write::{self, Plan};
use super::{COMMANDS, Cx, Failed, Options, Out, Usage, say};
use crate::code::{Code, Error, refuse};
use crate::record::{Draft, Event, Replay, StreamKind, sha256_hex, stream_kind, subject_digest};
use crate::settings::{self, Source, Val};
use crate::{table, task};
use serde_json::{Value, json};
use std::collections::BTreeMap;

type States = BTreeMap<String, &'static str>;
type Stream<'a> = (&'a String, &'a Vec<Event>);

/// The stream an identifier names: a task's `t-…`, a stage's `s-…`, or the stream's own name.
pub(super) fn stream_of(id: &str) -> String {
    match id.split_once('-') {
        Some(("t", _)) => format!("task-{id}"),
        Some(("s", _)) => format!("stage-{id}"),
        _ => id.to_owned(),
    }
}

/// The subject and the stream `id` names in `record`, a task's or, where `stages` allows, a stage's; a usage error
/// otherwise, as `status <id>` gives for an identifier the record does not hold.
fn resolve(record: &Replay, id: &str, stages: bool, command: &str) -> Result<(&'static str, String), Failed> {
    let stream = stream_of(id);
    let usage = |key, command| Failed::Usage(Usage { key, word: id.to_owned(), command });
    let subject = match stream_kind(&stream) {
        _ if !record.streams.contains_key(&stream) => return Err(usage("usage.id", Some(&COMMANDS[0]))),
        Some(StreamKind::Task) => "task",
        Some(StreamKind::Stage) if stages => "stage",
        Some(StreamKind::Stage) => return Err(usage("usage.stage", COMMANDS.iter().find(|c| c.name == command))),
        _ => return Err(usage("usage.id", Some(&COMMANDS[0]))),
    };
    Ok((subject, stream))
}

fn dependencies(events: &[Event]) -> impl Iterator<Item = &str> {
    events[0].value["payload"]["dependencies"].as_array().into_iter().flatten().filter_map(Value::as_str)
}

/// The tasks of stage `id`, each after the tasks of the stage it depends on, else in the order of their identifiers.
fn tasks_of<'a>(record: &'a Replay, id: &str) -> Vec<Stream<'a>> {
    let of_stage = |(n, e): &Stream| stream_kind(n) == Some(StreamKind::Task) && e[0].value["payload"]["stage"] == id;
    let mut left: Vec<Stream> = record.streams.iter().filter(of_stage).collect();
    let mut out = Vec::new();
    while !left.is_empty() {
        let waits = |(_, e): &Stream| dependencies(e).any(|d| left.iter().any(|(n, _)| n[5..] == *d));
        let next = left.iter().position(|t| !waits(t)).unwrap_or(0);
        out.push(left.remove(next));
    }
    out
}

pub(super) fn head(events: &[Event]) -> Value {
    let last = events.last().expect("a stream holds its creation");
    json!({"seq": last.seq, "digest": last.digest})
}

/// A text as `status` shows it: verbatim, but for what could move the terminal or reorder the line (control
/// characters, bidirectional marks, line and paragraph separators), shown as escapes, and with every line after the
/// first indented, so that no text passes for another field.
pub(crate) fn verbatim(s: &str, indent: &str) -> String {
    let one = |c: char| match c {
        '\n' => format!("\n{indent}"),
        c if super::text::hidden(c) => super::text::escape(c),
        c => c.to_string(),
    };
    s.chars().map(one).collect()
}

/// A field's values: each string verbatim, anything else as JSON; the catalogue's "none" for none.
fn items(lang: &str, v: &Value, indent: &str) -> String {
    let one = |v: &Value| verbatim(&v.as_str().map_or_else(|| v.to_string(), str::to_owned), indent);
    let all: Vec<String> = match v {
        Value::Array(a) => a.iter().map(one).collect(),
        Value::Object(o) => o.iter().map(|(k, v)| format!("{} = {}", verbatim(k, indent), one(v))).collect(),
        Value::Null => Vec::new(),
        v => vec![one(v)],
    };
    if all.is_empty() { say(lang, "bind.none", &[]) } else { all.join(", ") }
}

/// A task's limits or a stage's budget, each in words, in a fixed order.
fn limits(lang: &str, v: &Value) -> String {
    let each = ["rounds", "dispatches", "minutes", "tokens"].into_iter().filter_map(|k| Some((k, v.get(k)?)));
    each.map(|(k, n)| say(lang, &format!("limit.{k}"), &[("n", &n.to_string())])).collect::<Vec<_>>().join(", ")
}

/// The lines of a task or a stage, at `at`: its identifier and fields' digest, its stream's head, then each field.
fn lines(ui: &str, (name, events): Stream, at: &str) -> String {
    let p = &events[0].value["payload"];
    let (lang, deeper) = (p["lang"].as_str().unwrap_or_default(), format!("{at}    "));
    let t = |k: &str, params: &[(&str, &str)]| say(ui, &format!("bind.{k}"), params);
    let (id, digest) = (&name[name.find('-').unwrap_or_default() + 1..], subject_digest(p).unwrap_or_default());
    let task = name.starts_with("task-");
    let mut out = match p["stage"].as_str().filter(|_| task) {
        Some(stage) => format!("{at}{}\n", t("task", &[("id", id), ("stage", stage), ("digest", &digest)])),
        None => format!("{at}{}\n", t("stage", &[("id", id), ("digest", &digest)])),
    };
    let last = events.last().expect("a stream holds its creation");
    out += &format!("{at}  {}\n", t("head", &[("seq", &last.seq.to_string()), ("digest", &last.digest)]));
    out +=
        &format!("{at}  {} [{lang}]: {}\n", t("goal", &[]), verbatim(p["goal"].as_str().unwrap_or_default(), &deeper));
    let field = |k: &str, v: String| format!("{at}  {}: {v}\n", t(k, &[]));
    if !task {
        out += &field("budget", limits(ui, &p["budget"]));
        out += &field("checks", items(ui, &p["checks"], &deeper));
        return out + &field("settings", items(ui, &p["settings"], &deeper));
    }
    out += &format!("{at}  {} [{lang}]:\n", t("requirements", &[]));
    for (i, r) in p["requirements"].as_array().into_iter().flatten().enumerate() {
        let text = |k: &str| verbatim(r[k].as_str().unwrap_or_default(), &format!("{deeper}   "));
        out += &format!("{at}    {}. {}\n", i + 1, text("text"));
        out += &format!("{deeper}   {} [{lang}]: {}\n", t("criterion", &[]), text("criterion"));
    }
    for k in ["base", "dependencies", "allowed"] {
        out += &field(k, items(ui, &p[k], &deeper));
    }
    let profile = |k: &str| verbatim(p["profiles"][k].as_str().unwrap_or_default(), &deeper);
    out += &format!("{at}  {}\n", t("profiles", &[("build", &profile("build")), ("review", &profile("review"))]));
    out += &field("checks", items(ui, &p["checks"], &deeper));
    out += &field("limits", limits(ui, &p["limits"]));
    out += &field("delegation", items(ui, &p["delegationLanguage"], &deeper));
    out + &field("settings", items(ui, &p["settings"], &deeper))
}

/// What an authorization of a stream binds (REQ-GATE-7, PD-37): the rendering `status <id>` shows, in the person's
/// language, each text verbatim with its language tag; its digest, which `authorize` takes with `--seen`; and the tasks
/// it authorizes: the task itself, or each task of the stage that waits for an authorization, in their order.
pub(super) struct Bound<'a> {
    pub text: String,
    pub digest: String,
    pub tasks: Vec<Stream<'a>>,
}

/// What an authorization of `stream` binds; `None` when the table allows none from its state.
pub(super) fn bound<'a>(cx: &Cx, record: &'a Replay, states: &States, stream: &str) -> Option<Bound<'a>> {
    let (name, events) = record.streams.get_key_value(stream)?;
    let kind = if stream.starts_with("task-") { "task" } else { "stage" };
    table::step(kind, states.get(stream).copied(), &format!("{kind}-authorized")).ok()?;
    let lang = cx.lang_in(record);
    let waits = &table::find("task", "task-authorized")?.from;
    let mut text = format!("{}\n", say(lang, "bind.title", &[]));
    let tasks: Vec<Stream> = if kind == "task" {
        vec![(name, events)]
    } else {
        text += &lines(lang, (name, events), "");
        text += &format!("  {}\n", say(lang, "bind.tasks", &[]));
        let stage = tasks_of(record, &stream["stage-".len()..]);
        stage.into_iter().filter(|(n, _)| states.get(*n).is_some_and(|s| waits.contains(s))).collect()
    };
    for t in &tasks {
        text += &lines(lang, *t, if kind == "task" { "" } else { "    " });
    }
    Some(Bound { digest: sha256_hex(text.as_bytes()), text, tasks })
}

/// The first task of stage `id` that may start a run, in the stage's order (`run <stage>`, section 5);
/// `MOVE_NOT_ALLOWED` when none may.
fn first_to_run(record: &Replay, states: &States, id: &str) -> Result<String, Error> {
    let tasks = tasks_of(record, id);
    let may = |n: &String| table::allowed("task", states.get(n).copied()).contains(&"run-started");
    tasks.iter().find(|(n, _)| may(n)).map(|(n, _)| (*n).clone()).ok_or_else(|| {
        let each: Vec<String> = tasks.iter().map(|(n, _)| format!("{} {}", &n[5..], states[*n])).collect();
        let why = format!("run-started: no task of the stage {id} may start a run ({})", each.join(", "));
        refuse(Code::MoveNotAllowed, None, why)
    })
}

/// What `authorize` authorized: the subject, its identifier, the tasks bound and the widenings named.
struct Authorized {
    subject: &'static str,
    id: String,
    tasks: Vec<(String, String)>,
    widened: Vec<String>,
}

fn authorize_plan(cx: &Cx, id: &str, options: &Options, record: &Replay) -> Result<Plan<Authorized>, Failed> {
    let policy = write::state(cx, record)?.policy;
    let (subject, stream) = resolve(record, id, true, "authorize")?;
    let states = table::current(record)?;
    let kind = format!("{subject}-authorized");
    let m = table::step(subject, states.get(&stream).copied(), &kind)?;
    let b = bound(cx, record, &states, &stream).expect("the table allows the authorization");
    let events = &record.streams[&stream];
    let stage = match subject {
        "task" => format!("stage-{}", events[0].value["payload"]["stage"].as_str().unwrap_or_default()),
        _ => stream.clone(),
    };
    let stage_payload = &record.streams[&stage][0].value["payload"];
    // The layers of each task bound, its stage's then its own; a stage that binds none, its own alone.
    let stage_layer = (Source::Stage(stage["stage-".len()..].to_owned()), layer(stage_payload));
    let mut sets: Vec<Vec<(Source, BTreeMap<String, Val>)>> = b
        .tasks
        .iter()
        .map(|(n, e)| vec![stage_layer.clone(), (Source::Task(n[5..].to_owned()), layer(&e[0].value["payload"]))])
        .collect();
    if sets.is_empty() {
        sets.push(vec![stage_layer]);
    }
    let payloads: Vec<&Value> = b.tasks.iter().map(|(_, e)| &e[0].value["payload"]).collect();
    let allowed = |p: &Value| -> Vec<String> {
        p["allowed"].as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_owned).collect()
    };
    let (mut who, mut say, mut effs) = (None, None, Vec::new());
    table::evaluate(m, |g| -> Result<(), Failed> {
        match g.name {
            "ROLE_FORBIDDEN" => who = Some(write::person(cx, &policy, &kind)?),
            "WORDS_MISSING" => {
                say = Some(write::words(cx, &policy, &who.as_ref().expect("ROLE_FORBIDDEN first").0, options)?)
            }
            "TASK_CHANGED" if options.seen.as_deref() != Some(b.digest.as_str()) => {
                // The digest of what is shown now is not printed here: the person reads it with what it binds.
                let why = format!("what status {id} shows now is not what --seen names; read it again");
                return Err(refuse(Code::TaskChanged, None, why).into());
            }
            "CHECK_DUPLICATE" => {
                task::duplicates(&stage_payload["checks"], &settings::effective(&policy, None, None, &[], &[])?)?
            }
            "PROTECTED_PATH" => {
                for p in &payloads {
                    task::lane(&allowed(p).iter().map(String::as_str).collect::<Vec<_>>())?;
                }
            }
            // The protected paths in force for each task, with its stage's and its own layer, their widenings named
            // or not: those are the next guard's.
            "PROTECTED_PATH_WILDCARD" => {
                for (p, layers) in payloads.iter().zip(&sets) {
                    let named: Vec<String> = layers.iter().flat_map(|(_, l)| l.keys().cloned()).collect();
                    let eff = settings::effective(&policy, None, None, layers, &named)?;
                    let protected = crate::candidate::protected_paths(&eff, &super::propose::product(cx))?;
                    task::wildcards(&allowed(p).iter().map(String::as_str).collect::<Vec<_>>(), &protected)?;
                }
            }
            "WIDENING_UNNAMED" => {
                for layers in &sets {
                    effs.push(settings::effective(&policy, None, None, layers, &options.widen)?);
                }
            }
            "LIMIT_UNOBSERVABLE" => {
                for (p, eff) in payloads.iter().zip(&effs) {
                    let profiles = ["build", "review"].map(|k| p["profiles"][k].as_str().unwrap_or_default());
                    let tokens = p["limits"].get("tokens").is_some() || stage_payload["budget"].get("tokens").is_some();
                    task::observable(eff, &cx.modules, &profiles, tokens)?;
                }
            }
            _ => {}
        }
        Ok(())
    })?;
    // With `--run`, the run's guards are evaluated on the states the authorization leaves, before either is written
    // (D-062 point 1); its `run-started` follows the authorization, with the same words.
    let run = match options.run {
        false => None,
        true => {
            let mut after = states.clone();
            after.extend(b.tasks.iter().map(|(n, _)| ((*n).clone(), "authorized")));
            let next = if subject == "task" { stream.clone() } else { first_to_run(record, &after, &stage[6..])? };
            let (_, actor, _) = who.as_ref().expect("ROLE_FORBIDDEN is a guard of every authorization");
            let who = (actor.clone(), say.clone().expect("WORDS_MISSING is a guard of every authorization"));
            // A stage's authorization may bind other tasks than the one it runs, which relies on its own (P9E-3).
            let fresh = b.tasks.iter().any(|(n, _)| **n == next).then_some(options.widen.as_slice());
            Some((next.clone(), super::run::start(cx, record, (&policy, &after), &next, fresh, who)?))
        }
    };
    let (_, actor, signer) = who.expect("ROLE_FORBIDDEN is a guard of every authorization");
    let mut widened: Vec<String> = effs.iter().flat_map(|e| e.widened.clone()).collect();
    // A reviewer of the builder's provider where the policy requires another is a widening no layer makes: named, it
    // is recorded, and lets the task continue with the limitation recorded (REQ-GATE-4).
    widened.extend(options.widen.iter().filter(|w| *w == "review.differentProvider").cloned());
    widened.sort();
    widened.dedup();
    let bind = |(n, e): &Stream| {
        let p = &e[0].value["payload"];
        json!({"task": &n[5..], "digest": subject_digest(p), "head": head(e), "limits": p["limits"]})
    };
    let mut payload = match subject {
        "task" => bind(&b.tasks[0]),
        _ => json!({
            "stage": &stream[6..], "digest": subject_digest(stage_payload), "head": head(events),
            "tasks": b.tasks.iter().map(bind).collect::<Vec<_>>(),
        }),
    };
    payload["shown"] = json!(b.digest);
    payload["widened"] = json!(widened);
    let draft = Draft { kind, actor, policy: policy.digest.clone(), instance: String::new(), say, payload };
    let goal = |e: &[Event]| e[0].value["payload"]["goal"].as_str().unwrap_or_default().to_owned();
    let out = Authorized {
        subject,
        id: stream[subject.len() + 1..].to_owned(),
        tasks: b.tasks.iter().map(|(n, e)| (n[5..].to_owned(), goal(e))).collect(),
        widened,
    };
    let mut events = vec![(stream, draft)];
    let then = run.map(|(next, (started, then))| {
        events.push((next, started));
        then
    });
    Ok(Plan { events, signer: Some(signer), sealed: false, view: None, alambique: None, then, out })
}

/// `fabrewk authorize <stage|task> --seen <digest> --say <words> [--widen <key>]… [--run]`.
pub(super) fn authorize(cx: &Cx, id: &str, options: &Options) -> Result<Out, Failed> {
    write::not_an_agent(cx)?;
    let (a, written) = write::run(cx, |record| authorize_plan(cx, id, options, record))?;
    let mut text = cx.t(&format!("authorize.{}", a.subject), &[("id", &a.id)]) + "\n";
    for (tid, goal) in &a.tasks {
        text += &format!("  {}\n", cx.t("authorize.bound", &[("id", tid), ("goal", &verbatim(goal, "    "))]));
    }
    if !a.widened.is_empty() {
        text += &format!("{}\n", cx.t("authorize.widened", &[("keys", &a.widened.join(", "))]));
    }
    let tasks: Vec<&String> = a.tasks.iter().map(|(t, _)| t).collect();
    let mut doc = json!({
        "schema": "fabrewk.authorized/2", "subject": a.subject, "id": a.id, "tasks": tasks, "widened": a.widened,
        "run": null,
    });
    // With `--run`, what the run reports follows, with the events of both moves and the exit it gives.
    if !written.ran.is_null() {
        let run = super::run::report(cx, &written.ran, &written);
        doc["run"] = run.doc;
        return Ok(Out { doc, text: text + &run.text, ..run });
    }
    text += &written_lines(cx, &written, &mut doc);
    text += &cx.next(&[&format!("fabrewk run {}", a.id), &format!("fabrewk status {}", a.id)]);
    Ok(Out::new(doc, text))
}

/// `fabrewk reject <task> --say <words>`: the move, the person, their words, then `MERGE_IN_PROGRESS`, which until
/// phase 9 can tell where the record branch is refuses whenever an acceptance follows the task's last candidate
/// (D-068). The event binds the task and its stream's head, as an authorization does.
pub(super) fn reject(cx: &Cx, id: &str, options: &Options) -> Result<Out, Failed> {
    write::not_an_agent(cx)?;
    let plan = |record: &Replay| -> Result<Plan<String>, Failed> {
        let policy = write::state(cx, record)?.policy;
        let (_, stream) = resolve(record, id, false, "reject")?;
        let m = table::step("task", table::current(record)?.get(&stream).copied(), "task-rejected")?;
        let (alias, actor, signer) = write::person(cx, &policy, "task-rejected")?;
        let say = Some(write::words(cx, &policy, &alias, options)?);
        let events = &record.streams[&stream];
        let at = |kind: &str| events.iter().rposition(|e| e.kind == kind);
        table::evaluate(m, |g| match g.name {
            "MERGE_IN_PROGRESS" if at("task-accepted") > at("candidate-recorded") => {
                Err(refuse(Code::MergeInProgress, None, "an acceptance of the task's last candidate is on record"))
            }
            _ => Ok(()),
        })?;
        let payload = json!({"task": &stream[5..], "head": head(events)});
        let draft = Draft {
            kind: "task-rejected".into(),
            actor,
            policy: policy.digest.clone(),
            instance: String::new(),
            say,
            payload,
        };
        Ok(Plan {
            out: stream[5..].to_owned(),
            events: vec![(stream, draft)],
            signer: Some(signer),
            sealed: false,
            view: None,
            alambique: None,
            then: None,
        })
    };
    let (task, written) = write::run(cx, plan)?;
    let mut text = cx.t("reject.done", &[("id", &task)]) + "\n";
    let mut doc = json!({"schema": "fabrewk.rejected/1", "id": task});
    text += &written_lines(cx, &written, &mut doc);
    text += &cx.next(&[&format!("fabrewk status {task}")]);
    Ok(Out::new(doc, text))
}

/// `fabrewk run <task|stage> --say <words>` (D-053 point 4, D-062 point 2; part 9e): the move, or a stage's first
/// task that may start a run, the person and their words, then the run's guards and the run itself (`super::run`).
pub(super) fn run(cx: &Cx, id: &str, options: &Options) -> Result<Out, Failed> {
    write::not_an_agent(cx)?;
    let plan = |record: &Replay| -> Result<Plan<()>, Failed> {
        let policy = write::state(cx, record)?.policy;
        let (subject, stream) = resolve(record, id, true, "run")?;
        let states = table::current(record)?;
        let task = match subject {
            "task" => table::step("task", states.get(&stream).copied(), "run-started").map(|_| stream)?,
            _ => first_to_run(record, &states, &stream["stage-".len()..])?,
        };
        let (alias, actor, signer) = write::person(cx, &policy, "run-started")?;
        let say = write::words(cx, &policy, &alias, options)?;
        let (draft, then) = super::run::start(cx, record, (&policy, &states), &task, None, (actor, say))?;
        let events = vec![(task, draft)];
        Ok(Plan { events, signer: Some(signer), sealed: false, view: None, alambique: None, then: Some(then), out: () })
    };
    let ((), written) = write::run(cx, plan)?;
    Ok(super::run::report(cx, &written.ran, &written))
}
