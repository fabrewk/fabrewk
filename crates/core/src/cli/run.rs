//! `run` past confinement (part 9e; REQ-RUN-1, REQ-RUN-2, REQ-RUN-5 to REQ-RUN-7, REQ-RUN-10, REQ-TASK-3's base,
//! REQ-TASK-5 on the building side; S-5, S-6; D-079, D-081, D-082). The guards of section 4's `run-started` in their
//! order: those that refuse write nothing; a halting code (REQ-TASK-5) halts the task instead (`task-halted`, exit 4).
//! Under the lock, which the run keeps to its end (REQ-REG-5), come the confinement and the tool's preflight, the free
//! space, the run's two events, the attempt's copy, the builder's dispatch, its end, the change set and the candidate.
//! Until part 9f runs the checks and the review, a recorded candidate halts the task `CONFINEMENT_UNAVAILABLE` (D-082).

use super::write::{Appender, Then};
use super::{Cx, Failed, Out};
use crate::adapter::{self, Adapter, Launch, Role, Tool};
use crate::candidate::{self, Agent, Lanes};
use crate::code::{Code, Error, refuse};
use crate::confine::{self, Backend, End, Limits, Programs, Scratch};
use crate::detect::{self, Detection, Space};
use crate::record::git::{self, os};
use crate::record::{Draft, Event, Prepared, Replay, Repo, fault};
use crate::settings::{self, Effective, Policy, Source, Val};
use crate::table;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// The bytes kept of each of a builder's output channels, past which its cell is ended.
const OUTPUT: u64 = 32 << 20;

/// The longest packet, in bytes: it is the tool's last argument, and Linux takes no argument longer than 128 KiB.
const PACKET: usize = 120_000;

/// The place of the open attempt's start in a task's events, if one is open.
pub(super) fn open_attempt(events: &[Event]) -> Option<u64> {
    let last = events.iter().rposition(|e| e.kind == "attempt-started")?;
    (!events[last..].iter().any(|e| e.kind == "attempt-ended")).then_some(events[last].seq)
}

/// The name of an attempt's directories, under `local/tmp/` and `local/evidence/`.
pub(super) fn attempt_name(stream: &str, seq: u64) -> String {
    format!("attempt-{}-{seq}", &stream["task-".len()..])
}

/// What `events` spent: attempts started, seconds (an attempt whose end Fabrewk did not observe counts the whole time
/// it was allowed, S-5), and tokens read fresh and written, `None` when any part is unknown (D-082).
fn spent<'a>(events: impl Iterator<Item = &'a Event>) -> (u64, u64, Option<u64>) {
    let (mut n, mut seconds, mut tokens, mut allowed) = (0, 0u64, Some(0u64), BTreeMap::new());
    for e in events {
        let p = &e.value["payload"];
        match e.kind.as_str() {
            "attempt-started" => {
                n += 1;
                allowed.insert(e.seq, p["allowedSeconds"].as_u64().unwrap_or(u64::MAX));
            }
            "attempt-ended" => {
                let whole = p["attempt"].as_u64().and_then(|a| allowed.get(&a)).copied().unwrap_or(u64::MAX);
                seconds = seconds.saturating_add(p["usage"]["seconds"].as_u64().unwrap_or(whole));
                let t = &p["usage"]["tokens"];
                tokens = tokens.and_then(|s| s.checked_add(t["input"].as_u64()?)?.checked_add(t["output"].as_u64()?));
            }
            _ => {}
        }
    }
    (n, seconds, tokens)
}

/// A limit's room left: `Err` with what is spent when nothing is left, else the seconds left (`u64::MAX` for none).
fn room(limits: &Value, (n, seconds, tokens): (u64, u64, Option<u64>), whose: &str) -> Result<u64, Error> {
    let exhausted = |what: String| refuse(Code::BudgetExhausted, None, format!("{whose}: {what}"));
    if let Some(d) = limits["dispatches"].as_u64().filter(|d| n >= *d) {
        return Err(exhausted(format!("{n} of {d} dispatches spent")));
    }
    match (limits["tokens"].as_u64(), tokens) {
        (Some(t), None) => return Err(exhausted(format!("the tokens spent are unknown, and its limit is {t}"))),
        (Some(t), Some(s)) if s >= t => return Err(exhausted(format!("{s} of {t} tokens spent"))),
        _ => {}
    }
    let minutes = limits["minutes"].as_u64().map(|m| m.saturating_mul(fault::minute()));
    match minutes.map(|m| m.saturating_sub(seconds)) {
        Some(0) => Err(exhausted(format!("{seconds} seconds of its {} minutes spent", limits["minutes"]))),
        left => Ok(left.unwrap_or(u64::MAX)),
    }
}

/// The product tree of `commit` (REQ-GATE-8): its root's entries but `.brewstillery`.
fn product_tree(repo: &Repo, commit: &str) -> Result<Vec<u8>, Error> {
    let listed = git::git(repo, &os(&["ls-tree", "-z", "--full-tree", commit]), b"", None)?;
    let entries = listed.split(|b| *b == 0).filter(|e| !e.ends_with(b"\t.brewstillery"));
    Ok(entries.flat_map(|e| e.iter().copied().chain([0])).collect())
}

/// The base of a run (REQ-TASK-3): a task's own, or for one with dependencies the record branch's head, whose product
/// tree must be the one the last of its dependencies' integrations left, its candidate's (`BASE_MOVED` otherwise).
fn base(cx: &Cx, record: &Replay, stream: &str) -> Result<String, Error> {
    let p = &record.streams[stream][0].value["payload"];
    if let Some(base) = p["base"].as_str() {
        return Ok(base.to_owned());
    }
    let repo = super::propose::product(cx);
    let head = git::head_of(&repo, &git::record_branch(&repo)?)?;
    let moved = |why: &str| refuse(Code::BaseMoved, None, why.to_owned());
    let head = head.ok_or_else(|| moved("the record branch has no commit"))?;
    let deps = p["dependencies"].as_array().into_iter().flatten().filter_map(Value::as_str);
    let last = deps
        .filter_map(|d| {
            let events = record.streams.get(&format!("task-{d}"))?;
            let at = events.iter().rposition(|e| e.kind == "task-integrated")?;
            let candidate = events[..at].iter().rev().find(|e| e.kind == "candidate-recorded")?;
            Some((events[at].value["at"].as_str()?, candidate.value["payload"]["commit"].as_str()?))
        })
        .max();
    let (_, candidate) = last.ok_or_else(|| moved("no integration of its dependencies is on record"))?;
    if product_tree(&repo, &head)? != product_tree(&repo, candidate)? {
        return Err(moved("the product at the record branch's head is not what its dependencies' integration left"));
    }
    Ok(head)
}

/// The text of a setting in `eff`, the empty text when absent.
fn text<'a>(eff: &'a Effective, key: &str) -> &'a str {
    detect::text(eff, key).unwrap_or_default()
}

fn list(eff: &Effective, key: &str) -> Vec<String> {
    match eff.values.get(key) {
        Some((Val::List(l), _)) => l.clone(),
        _ => Vec::new(),
    }
}

fn int(eff: &Effective, key: &str) -> Option<u64> {
    match eff.values.get(key) {
        Some((Val::Int(n), _)) => u64::try_from(*n).ok(),
        _ => None,
    }
}

/// What a builder's dispatch needs under the lock, owned: where, the task, the tool, the cell's limits, the packet,
/// what the change set may hold, and a halt the guards already reached (the budget, the packet's size).
struct Dispatch {
    domain: PathBuf,
    home: PathBuf,
    product: Repo,
    stream: String,
    base: String,
    adapter: &'static Adapter,
    backend: Option<&'static Backend>,
    detection: Detection,
    tool: String,
    exe: PathBuf,
    release: PathBuf,
    profile: String,
    model: String,
    effort: String,
    packet: String,
    allowed: u64,
    capped: bool,
    lanes: (Vec<String>, Vec<String>, Vec<String>),
    sign_off: bool,
    free: u64,
    halt: Option<Error>,
}

/// The guards of `run-started` for `stream`, in the table's order (REQ-TASK-1): DEPENDENCY_OPEN, BASE_MOVED,
/// LOCK_HELD, REQ-RUN-2 (the profile, the tools as the cell shows them, the budget, the packet's size) and
/// ALAMBIQUE_UNAPPROVED; a halting code among them stops the evaluation and halts the run. CONFINEMENT_UNAVAILABLE comes
/// under the lock. Gives the `run-started` the person signs, its head filled when it is made, and its authorization
/// too with `fresh` (an authorization the same command writes, after which nothing is spent), and what writes the run.
pub(super) fn start(
    cx: &Cx,
    record: &Replay,
    (policy, states): (&Policy, &BTreeMap<String, &'static str>),
    stream: &str,
    fresh: bool,
    who: (Value, crate::record::Say),
) -> Result<(Draft, Then), Failed> {
    let m = table::find("task", "run-started").expect("the table holds run-started");
    let state = |n: &str| states.get(n).copied().unwrap_or("none");
    let events = &record.streams[stream];
    let p = &events[0].value["payload"];
    let (mut base, mut d) = (String::new(), None);
    table::evaluate(m, |g| -> Result<(), Failed> {
        match g.name {
            "DEPENDENCY_OPEN" => {
                let deps = p["dependencies"].as_array().into_iter().flatten().filter_map(Value::as_str);
                if let Some(t) = deps.map(|t| format!("task-{t}")).find(|t| state(t) != "integrated") {
                    return Err(refuse(Code::DependencyOpen, None, format!("{} is {}", &t[5..], state(&t))).into());
                }
            }
            "BASE_MOVED" => base = self::base(cx, record, stream)?,
            "LOCK_HELD" => {
                if let Some((n, _)) = states.iter().find(|(_, s)| **s == "running") {
                    let why = format!("{} is running; tasks run one at a time", &n[5..]);
                    return Err(refuse(Code::LockHeld, None, why).into());
                }
            }
            "REQ-RUN-2" => d = Some(prepare(cx, record, policy, stream, fresh)?),
            "ALAMBIQUE_UNAPPROVED" if d.as_ref().is_some_and(|d: &Dispatch| d.halt.is_none()) => {
                crate::alambique::check(record, &cx.domain)?
            }
            _ => {}
        }
        Ok(())
    })?;
    let mut d = d.expect("REQ-RUN-2 is a guard of run-started");
    d.base.clone_from(&base);
    let stage = &record.streams[&format!("stage-{}", p["stage"].as_str().unwrap_or_default())];
    let authorization = match events.iter().rposition(|e| e.kind == "task-authorized") {
        _ if fresh => Value::Null,
        Some(i) => json!({"stream": stream, "seq": events[i].seq, "digest": events[i].digest}),
        None => {
            let by = stage.iter().rev().find(|e| e.kind == "stage-authorized").expect("an authorized task's");
            json!({"stream": by.stream, "seq": by.seq, "digest": by.digest})
        }
    };
    let payload = json!({"task": &stream["task-".len()..], "head": null, "authorization": authorization, "base": base});
    let (actor, say) = who;
    let policy = policy.digest.clone();
    let draft = Draft { kind: "run-started".into(), actor, policy, instance: String::new(), say: Some(say), payload };
    Ok((draft, Box::new(move |a, prepared| go(d, a, prepared))))
}

/// REQ-RUN-2 for `stream`'s builder: the profile valid and its tool and required tools present, these as the cell
/// would show them (S-6); then the budget since the task's last authorization and its stage's over all its tasks
/// (D-082), and the packet's size, which halt the run instead.
fn prepare(cx: &Cx, record: &Replay, policy: &Policy, stream: &str, fresh: bool) -> Result<Dispatch, Failed> {
    let events = &record.streams[stream];
    let p = &events[0].value["payload"];
    let stage_id = p["stage"].as_str().unwrap_or_default();
    let stage = &record.streams[&format!("stage-{stage_id}")];
    let layers = vec![
        (Source::Stage(stage_id.to_owned()), super::propose::layer(&stage[0].value["payload"])),
        (Source::Task(stream["task-".len()..].to_owned()), super::propose::layer(p)),
    ];
    let named: Vec<String> = layers.iter().flat_map(|(_, l)| l.keys().cloned()).collect();
    let machine = crate::machine::machine_file(&cx.folder()?)?;
    let eff = settings::effective(policy, None, machine.as_ref(), &layers, &named)?;
    let detection = detect::detect(&cx.domain, &eff, &cx.modules, std::env::var_os("PATH").as_deref());
    let profile = p["profiles"]["build"].as_str().unwrap_or_default();
    detect::check_profile(policy, &eff, &detection, profile)?;
    let key = |leaf: &str| format!("profiles.{profile}.{leaf}");
    let tool = text(&eff, &key("tool")).to_owned();
    let missing = |why: String| refuse(Code::ToolMissing, None, why);
    let name = text(&eff, &format!("tools.{tool}.adapter"));
    let adapter = cx.adapters.iter().copied().find(|a| a.program.adapter == Some(name));
    let adapter = adapter.ok_or_else(|| missing(format!("{tool}: no built-in adapter is named {name}")))?;
    let home = std::env::var_os("HOME").map(PathBuf::from).unwrap_or_default();
    let found = detection.get(Space::Tool, &tool).map(|f| f.path.clone()).unwrap_or_default();
    let (exe, release) = adapter::resolve(adapter, &found, &[&home, &cx.root])?;
    // S-6: a required tool is present only when the cell shows it, under its system directories.
    if let Some(backend) = cx.backend {
        for t in list(&eff, &key("requiredTools")) {
            let at = detection.get(Space::Required, &t).and_then(|f| f.path.canonicalize().ok());
            if !at.is_some_and(|at| backend.system.iter().any(|s| at.starts_with(s))) {
                return Err(
                    missing(format!("{t}: it lives outside the system directories a confined cell shows")).into()
                );
            }
        }
    }
    let since = match events.iter().rposition(|e| e.kind == "task-authorized") {
        _ if fresh => events.len(),
        at => at.map_or(0, |i| i + 1),
    };
    let of_stage = crate::view::tasks_of(record, stage_id).into_iter().flat_map(|(_, e)| e.iter());
    let profile_seconds = int(&eff, &key("minutes")).unwrap_or(1).saturating_mul(fault::minute());
    let left = room(&p["limits"], spent(events[since..].iter()), "the task")
        .and_then(|t| Ok(t.min(room(&stage[0].value["payload"]["budget"], spent(of_stage), "its stage")?)));
    let rules = adapter::rules(record, Role::Build, p["delegationLanguage"].as_str().unwrap_or("en"));
    let packet = adapter::packet(p, Role::Build, &rules, text(&eff, "languages.codeLanguage"), None);
    let halt = match &left {
        Err(e) => Some(e.clone()),
        Ok(_) if packet.len() > PACKET => Some(refuse(
            Code::ToolFailed,
            None,
            format!("its packet holds {} bytes, more than the {PACKET} a tool's argument may", packet.len()),
        )),
        Ok(_) => None,
    };
    let left = left.unwrap_or(1);
    let model = text(&eff, &key("model"));
    Ok(Dispatch {
        domain: cx.domain.clone(),
        home,
        product: super::propose::product(cx),
        stream: stream.to_owned(),
        base: String::new(),
        adapter,
        backend: cx.backend,
        detection,
        tool,
        exe,
        release,
        profile: profile.to_owned(),
        model: text(&eff, &format!("models.{model}.id")).to_owned(),
        effort: text(&eff, &key("effort")).to_owned(),
        packet,
        allowed: left.min(profile_seconds),
        capped: left < profile_seconds,
        lanes: (
            p["allowed"].as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_owned).collect(),
            candidate::protected_names(&eff),
            candidate::protected_paths(&eff, &super::propose::product(cx))?,
        ),
        sign_off: eff.values.get("git.signOff").is_some_and(|v| v.0 == Val::Bool(true)),
        free: int(&eff, "limits.freeDiskMegabytes").unwrap_or(0),
        halt,
    })
}

/// The words of a failure an event may hold: the core's own, never an operating system's text (REQ-REG-13).
fn words(e: &Error) -> String {
    match e {
        Error::Refused { detail, .. } => detail.clone(),
        Error::Io { path, .. } => format!("{} cannot be read or written", path.display()),
    }
}

/// Halts the task with `e`'s code, a halting one, or `TOOL_FAILED`; the reason names `evidence` when there is one, and
/// every path in it relative to the repository's root, so that the record holds nothing of this machine's layout.
fn halt(a: &mut Appender, out: &mut Value, d: &Dispatch, e: &Error, evidence: Option<&Path>) -> Result<(), Error> {
    let code = e.code().map(|c| c.to_string()).filter(|c| crate::task::HALTS.contains(&c.as_str()));
    let code = code.unwrap_or_else(|| Code::ToolFailed.to_string());
    let mut reason = words(e);
    if let Some(dir) = evidence.filter(|d| !reason.contains(&*d.display().to_string())) {
        reason += &format!("; its output is kept in {}", dir.display());
    }
    let root = d.domain.parent().unwrap_or(&d.domain);
    let reason = reason.replace(&format!("{}/", root.display()), "");
    a.fabrewk(&d.stream, "task-halted", json!({"code": code, "reason": reason}))?;
    out["halt"] = json!({"code": code, "reason": reason});
    Ok(())
}

/// The run under the lock (see [`start`]): the events planned before it (an authorization) are written first; then
/// CONFINEMENT_UNAVAILABLE (the backend, its probe and the tool's preflight, each unless a use verified it) and
/// DISK_LOW, which halt the task as a halt the guards reached does; then `run-started` and the attempt.
fn go(mut d: Dispatch, a: &mut Appender, mut prepared: Vec<(String, Prepared)>) -> Result<Value, Error> {
    let (stream, started) = prepared.pop().expect("run-started is the plan's last event");
    for (s, p) in prepared {
        a.write(s, p)?;
    }
    let mut out = json!({"task": &stream["task-".len()..], "halt": null, "candidate": null, "attempt": null});
    if let Some(e) = d.halt.take() {
        halt(a, &mut out, &d, &e, None)?;
        return Ok(out);
    }
    let ready = (|| {
        let programs = confine::available(d.backend, &mut d.detection, &d.domain)?;
        let backend = d.backend.expect("an available backend");
        a.used.extend(backend.programs.iter().map(|p| (Space::Program, p.executable.to_owned())));
        let tool = Tool { adapter: d.adapter, executable: &d.exe, release: &d.release };
        // A preflight's pass is kept as verified by use for the tool's path and version (3.9).
        if d.detection.get(Space::Tool, &d.tool).is_none_or(|f| f.verified.is_none()) {
            adapter::preflight(tool, backend, &programs, &d.domain)?;
        }
        a.used.push((Space::Tool, d.tool.clone()));
        let df = d.detection.get(Space::Program, "df").map(|f| f.path.clone());
        candidate::free_space(&d.domain.join("local"), d.free, df.as_deref())?;
        Ok::<_, Error>((programs, backend))
    })();
    let (programs, backend) = match ready {
        Ok(ready) => ready,
        Err(e) => {
            halt(a, &mut out, &d, &e, None)?;
            return Ok(out);
        }
    };
    a.write(stream.clone(), started)?;
    let started = json!({
        "role": "build", "profile": d.profile, "tool": d.tool, "model": d.model, "effort": d.effort,
        "allowedSeconds": d.allowed,
    });
    let seq = a.fabrewk(&stream, "attempt-started", started)?.seq;
    out["attempt"] = json!(seq);
    let name = attempt_name(&stream, seq);
    let scratch = Scratch::create(&d.domain, &name)?;
    let evidence = confine::evidence(&d.domain, &name)?;
    let (e, remove) = attempt(&d, a, &mut out, (&programs, backend), seq, &scratch, &evidence)?;
    // A cell the backend could not show ended keeps its directories, unread (principle 4).
    if remove {
        scratch.remove()?;
    }
    // A failure names its evidence (REQ-REG-13); the halt that follows a candidate is no failure (D-082).
    let failed = out["candidate"].is_null();
    halt(a, &mut out, &d, &e, failed.then_some(evidence.as_path()))?;
    Ok(out)
}

/// The attempt: the copy, the dispatch, its end and, on success, the change set and the candidate. Gives the halt that
/// follows, and whether the attempt's directories may be removed.
fn attempt(
    d: &Dispatch,
    a: &mut Appender,
    out: &mut Value,
    (programs, backend): (&Programs, &Backend),
    seq: u64,
    scratch: &Scratch,
    evidence: &Path,
) -> Result<(Error, bool), Error> {
    let stream = &d.stream;
    let end = |a: &mut Appender, outcome: &str, model: Option<&str>, usage: Value, kept: Value| {
        let payload = json!({"attempt": seq, "outcome": outcome, "model": model, "usage": usage, "evidence": kept});
        a.fabrewk(stream, "attempt-ended", payload)
    };
    // PD-21: a failed export ends the attempt before any process starts; its words are kept as evidence.
    if let Err(e) = candidate::export(&d.product, &d.base, &scratch.work) {
        let file = evidence.join("export");
        let mut kept = std::fs::File::options();
        let kept = kept.write(true).create_new(true).mode(0o600).open(&file);
        kept.and_then(|mut f| f.write_all(words(&e).as_bytes())).map_err(crate::code::io_at(&file))?;
        end(a, "failure", None, adapter::usage(None, None), Value::Null)?;
        return Ok((refuse(Code::ToolFailed, Some(evidence), "the base could not be exported for the attempt"), true));
    }
    let tool = Tool { adapter: d.adapter, executable: &d.exe, release: &d.release };
    let schema = adapter::schema(Role::Build).to_string();
    let launch = Launch { role: Role::Build, packet: &d.packet, schema: &schema, model: &d.model, effort: &d.effort };
    let limits = Limits { time: Duration::from_secs(d.allowed), bytes: OUTPUT };
    fault::pause("before-dispatch");
    let ran = adapter::dispatch(tool, backend, programs, &launch, &d.home, scratch, evidence, limits);
    fault::pause("before-attempt-ended");
    let (ran, report) = match ran {
        Err(u) if !u.ended => {
            end(a, "unknown", None, adapter::usage(None, None), Value::Null)?;
            let why = format!("its cell could not be shown ended, so it is left unread: {}", words(&u.error));
            return Ok((refuse(Code::OutcomeUnknown, Some(evidence), why), false));
        }
        Err(u) => {
            end(a, "failure", None, adapter::usage(None, None), Value::Null)?;
            return Ok((u.error, true));
        }
        Ok(ran) => ran,
    };
    let kept = json!({"stdout": ran.stdout.digest, "stderr": ran.stderr.digest});
    let tool_name = &d.tool;
    let failed = if ran.end != End::Exited(0) {
        Some(match ran.end {
            End::TimedOut if d.capped => {
                refuse(Code::BudgetExhausted, Some(evidence), "the task's or its stage's minutes are spent")
            }
            _ => ran.failure(tool_name),
        })
    } else {
        match &report.result {
            None => Some(refuse(Code::ToolFailed, Some(evidence), format!("{tool_name}'s stream gave no result"))),
            Some(r) => adapter::validate(Role::Build, r).err(),
        }
    };
    let outcome = if failed.is_some() { "failure" } else { "success" };
    end(a, outcome, report.model.as_deref(), adapter::usage(Some(ran.seconds), report.tokens), kept)?;
    if let Some(e) = failed {
        return Ok((e, true));
    }
    let (allowed, names, paths) = &d.lanes;
    let allowed: Vec<&str> = allowed.iter().map(String::as_str).collect();
    let lanes = Lanes { allowed: &allowed, names, paths };
    let changes = match candidate::changes(&d.product, &d.base, &scratch.work, &scratch.root, &lanes) {
        Ok(c) => c,
        Err(e) if e.code().is_some() => return Ok((e, true)),
        Err(e) => return Err(e),
    };
    let task = &stream["task-".len()..];
    let previous = git::head_of(&d.product, &format!("fabrewk/{task}"))?;
    let model = report.model.as_deref().unwrap_or(&d.model);
    let agent = Agent { tool: tool_name, provider: d.adapter.provider, model };
    let made = candidate::commit(
        &d.product,
        &d.base,
        &changes,
        &scratch.work,
        &scratch.root,
        task,
        &agent,
        d.sign_off,
        previous.as_deref(),
    );
    let c = match made {
        Ok(c) => c,
        Err(e) if e.code().is_some() => return Ok((e, true)),
        Err(e) => return Err(e),
    };
    let entry = |e: &Option<candidate::Entry>| e.as_ref().map(|e| json!({"mode": e.mode.git(), "oid": e.oid}));
    let listed: Vec<Value> =
        changes.iter().map(|c| json!({"path": c.path, "before": entry(&c.before), "after": entry(&c.after)})).collect();
    a.fabrewk(stream, "candidate-recorded", json!({"commit": c.commit, "tree": c.tree, "changes": listed}))?;
    out["candidate"] = json!({"commit": c.commit, "tree": c.tree, "branch": format!("fabrewk/{task}")});
    let why = format!(
        "its candidate is on fabrewk/{task}; this version does not run the checks and the review yet, which part 9f \
         builds"
    );
    Ok((refuse(Code::ConfinementUnavailable, None, why), true))
}

/// What `run` reports: the task, its attempt and candidate, and the halt it waits on (exit 4, REQ-CLI-3).
pub(super) fn report(cx: &Cx, ran: &Value, written: &super::write::Written) -> Out {
    let id = ran["task"].as_str().unwrap_or_default();
    let mut text = cx.t("run.done", &[("id", id)]) + "\n";
    if let Some(n) = ran["attempt"].as_u64() {
        text += &format!("  {}\n", cx.t("run.attempt", &[("n", &n.to_string())]));
    }
    if let Some(c) = ran["candidate"].as_object() {
        let s = |k: &str| c[k].as_str().unwrap_or_default().to_owned();
        text += &format!("  {}\n", cx.t("run.candidate", &[("commit", &s("commit")), ("branch", &s("branch"))]));
    }
    let mut doc = json!({"schema": "fabrewk.run/1", "id": id, "attempt": ran["attempt"], "candidate": ran["candidate"], "halt": ran["halt"]});
    let mut exit = super::DONE;
    if let Some(h) = ran["halt"].as_object() {
        let s = |k: &str| h[k].as_str().unwrap_or_default().to_owned();
        text += &(cx.t("run.halted", &[("id", id), ("code", &s("code"))]) + "\n");
        text += &format!("  {}: {}\n", cx.t("ui.detail", &[]), super::tasks::verbatim(&s("reason"), "    "));
        exit = super::WAITING;
    }
    text += &super::adopt::written_lines(cx, written, &mut doc);
    text += &cx.next(&[&format!("fabrewk status {id}")]);
    Out { exit, ..Out::new(doc, text) }
}
