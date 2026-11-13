//! `run` past confinement (parts 9e and 9f; REQ-RUN-1 to REQ-RUN-10, REQ-TASK-3's base, REQ-TASK-5, REQ-GATE-1 to
//! REQ-GATE-6; S-5, S-6; D-079, D-081, D-082, D-085). The guards of section 4's `run-started` in their order: those
//! that refuse write nothing; a halting code (REQ-TASK-5) halts the task instead (`task-halted`, exit 4). Under the
//! lock, which the run keeps to its end (REQ-REG-5), come the confinement and the tools' preflights, the free space,
//! `run-started`, and the rounds: each the builder's attempt (none in a revalidation's first round, REQ-RUN-9) and its
//! candidate, the required checks on it and, once they pass, the reviewer's attempt; a reject or a failed check starts
//! the next round without a person (REQ-GATE-6), an examined accept verifies the task (REQ-GATE-5).

use super::write::{Appender, Then};
use super::{Cx, Failed, Out};
use crate::adapter::{self, Adapter, Launch, Report, Role, Tokens, Tool};
use crate::candidate::{self, Lanes};
use crate::code::{Code, Error, refuse};
use crate::confine::{self, Backend, End, Limits, Programs, Scratch};
use crate::detect::{self, Detection, Space};
use crate::gate;
use crate::record::git::{self, os};
use crate::record::{Draft, Event, Prepared, Replay, Repo, fault, sha256_hex};
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

/// The authorization in force for the task of `stream` (P9E-3), as the replay holds a run to it: its last
/// `task-authorized`, or its stage's last `stage-authorized` that binds it, whichever comes later in the task's events;
/// with the place in them after which its limits count.
fn in_force(record: &Replay, stream: &str) -> Option<(Value, usize)> {
    let events = &record.streams[stream];
    let name = |e: &Event| json!({"stream": e.stream, "seq": e.seq, "digest": e.digest});
    // Its limits count from the events after it, the first of which is at its own place.
    let own = events.iter().rfind(|e| e.kind == "task-authorized");
    let own = own.map(|e| (name(e), usize::try_from(e.seq).unwrap_or(usize::MAX)));
    let stage = format!("stage-{}", events[0].value["payload"]["stage"].as_str().unwrap_or_default());
    let binds = |e: &Event| {
        let tasks = e.value["payload"]["tasks"].as_array()?;
        tasks.iter().find(|t| t["task"] == stream["task-".len()..])?["head"]["seq"].as_u64()
    };
    let by_stage = record.streams.get(&stage).into_iter().flatten().filter(|e| e.kind == "stage-authorized");
    let by_stage = by_stage.filter_map(|e| Some((name(e), usize::try_from(binds(e)?).ok()?)));
    own.into_iter().chain(by_stage).max_by_key(|(_, since)| *since)
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
/// tree must be the one the last of its dependencies' integrations left, its candidate's (`BASE_MOVED` otherwise). The
/// last is read from the branch's history, never from the clock (P9E-9): the first commit, going back from the head
/// along first parents, that is a dependency's candidate or names it in a `Fabrewk-Candidate` trailer (REQ-GATE-8).
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
    let candidates: Vec<&str> = deps
        .filter_map(|d| {
            let events = record.streams.get(&format!("task-{d}"))?;
            let at = events.iter().rposition(|e| e.kind == "task-integrated")?;
            let candidate = events[..at].iter().rev().find(|e| e.kind == "candidate-recorded")?;
            candidate.value["payload"]["commit"].as_str()
        })
        .collect();
    let format = "--format=%H %(trailers:key=Fabrewk-Candidate,valueonly,separator=%x20)";
    let log = git::git(&repo, &os(&["log", "-z", "--first-parent", format, &head]), b"", None)?;
    let text = String::from_utf8_lossy(&log);
    let last = text.split('\0').find_map(|c| c.split_whitespace().find(|w| candidates.contains(w)));
    let last = last.ok_or_else(|| moved("no integration of its dependencies is in the record branch's history"))?;
    if product_tree(&repo, &head)? != product_tree(&repo, last)? {
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

fn home() -> PathBuf {
    std::env::var_os("HOME").map(PathBuf::from).unwrap_or_default()
}

/// One role's agent as its profile gives it: its adapter, its tool and the version it reported (PD-29), its
/// executable and release as they resolve, the profile, the model and effort requested, the seconds the profile allows
/// an attempt, and its packet's rules.
#[derive(Clone)]
struct Agent {
    adapter: &'static Adapter,
    tool: String,
    version: Option<String>,
    exe: PathBuf,
    release: PathBuf,
    profile: String,
    model: String,
    effort: String,
    seconds: u64,
    rules: Vec<String>,
}

/// REQ-RUN-2 for `role`'s agent of the task `p`: its profile valid, its tool and required tools present, these as the
/// cell would show them (S-6).
fn agent(
    cx: &Cx,
    (policy, eff, detection): (&Policy, &Effective, &Detection),
    record: &Replay,
    p: &Value,
    role: Role,
) -> Result<Agent, Error> {
    let profile = p["profiles"][role.name()].as_str().unwrap_or_default();
    detect::check_profile(policy, eff, detection, profile)?;
    let key = |leaf: &str| format!("profiles.{profile}.{leaf}");
    let tool = text(eff, &key("tool")).to_owned();
    let missing = |why: String| refuse(Code::ToolMissing, None, why);
    let name = text(eff, &format!("tools.{tool}.adapter"));
    let adapter = cx.adapters.iter().copied().find(|a| a.program.adapter == Some(name));
    let adapter = adapter.ok_or_else(|| missing(format!("{tool}: no built-in adapter is named {name}")))?;
    let found = detection.get(Space::Tool, &tool);
    let (exe, release) =
        adapter::resolve(adapter, &found.map(|f| f.path.clone()).unwrap_or_default(), &[&home(), &cx.root])?;
    // S-6: a required tool is present only when the cell shows it, under its system directories.
    if let Some(backend) = cx.backend {
        for t in list(eff, &key("requiredTools")) {
            let at = detection.get(Space::Required, &t).and_then(|f| f.path.canonicalize().ok());
            if !at.is_some_and(|at| backend.system.iter().any(|s| at.starts_with(s))) {
                return Err(missing(format!("{t}: it lives outside the system directories a confined cell shows")));
            }
        }
    }
    let model = text(eff, &key("model"));
    Ok(Agent {
        adapter,
        version: found.and_then(|f| f.version.clone()),
        exe,
        release,
        profile: profile.to_owned(),
        model: text(eff, &format!("models.{model}.id")).to_owned(),
        effort: text(eff, &key("effort")).to_owned(),
        seconds: int(eff, &key("minutes")).unwrap_or(1).saturating_mul(fault::minute()),
        rules: adapter::rules(record, role, p["delegationLanguage"].as_str().unwrap_or("en")),
        tool,
    })
}

/// What a run needs under the lock, owned: where, the task and its stage, the agents, what the task and its stage have
/// spent, what the change set may hold, the required checks, the rounds so far, and a halt the guards already reached.
struct Dispatch {
    domain: PathBuf,
    home: PathBuf,
    product: Repo,
    stream: String,
    base: String,
    /// The task's and its stage's proposals.
    task: Value,
    stage: Value,
    backend: Option<&'static Backend>,
    detection: Detection,
    build: Agent,
    /// `None` only when the run halts before any dispatch (`PROVIDER_MISSING`).
    review: Option<Agent>,
    code: String,
    /// The attempts, seconds and tokens spent by the task since its authorization and by its stage (D-082).
    spent: [(u64, u64, Option<u64>); 2],
    lanes: (Vec<String>, Vec<String>, Vec<String>),
    sign_off: bool,
    free: u64,
    checks: Vec<gate::Check>,
    policy: String,
    requirements: String,
    /// Whether the reviewer's provider is the builder's where `review.differentProvider` asks another (REQ-GATE-4).
    limited: bool,
    /// The rounds spent since the authorization in force, the progress measure of each that ended, and what the last
    /// leaves for the next: confirmed findings and the checks that did not pass (REQ-GATE-6, REQ-RUN-8).
    rounds: u64,
    counts: Vec<u64>,
    previous: (Vec<Value>, Vec<Value>),
    /// The task's last candidate (commit, tree, its product tree's digest), and whether the first round revalidates
    /// it (REQ-RUN-9).
    candidate: Option<(String, String, String)>,
    revalidate: bool,
    /// Whether the task was `verified`, which no halt leaves (section 4).
    verified: bool,
    halt: Option<Error>,
}

impl Dispatch {
    fn agent(&self, role: Role) -> &Agent {
        match role {
            Role::Build => &self.build,
            Role::Review => self.review.as_ref().expect("a run that reviews has its reviewer"),
        }
    }

    /// `role`'s packet (REQ-RUN-8), with what the round before left, if any.
    fn packet(&self, role: Role) -> String {
        let (findings, failed) = &self.previous;
        let previous = (!findings.is_empty() || !failed.is_empty()).then_some(adapter::Previous { findings, failed });
        adapter::packet(&self.task, role, &self.agent(role).rules, &self.code, previous.as_ref())
    }

    /// The seconds the task's limits and its stage's budget leave (D-082), or `BUDGET_EXHAUSTED`.
    fn room(&self) -> Result<u64, Error> {
        let task = room(&self.task["limits"], self.spent[0], "the task")?;
        Ok(task.min(room(&self.stage["budget"], self.spent[1], "its stage")?))
    }

    /// REQ-RUN-10 before each copy: `DISK_LOW` below `limits.freeDiskMegabytes`.
    fn free_space(&self) -> Result<u64, Error> {
        let df = self.detection.get(Space::Program, "df").map(|f| f.path.clone());
        candidate::free_space(&self.domain.join("local"), self.free, df.as_deref())
    }
}

/// The guards of `run-started` for `stream`, in the table's order (REQ-TASK-1): DEPENDENCY_OPEN, BASE_MOVED,
/// LOCK_HELD, REQ-RUN-2 (the profiles, the tools as the cell shows them, the required checks, the rounds, the budget, the
/// reviewer's provider, the packet's size) and ALAMBIQUE_UNAPPROVED; a halting code among them stops the evaluation and
/// halts the run. CONFINEMENT_UNAVAILABLE comes under the lock. Gives the `run-started` the person signs, its head filled
/// when it is made, and its authorization too with `fresh` (the widenings an authorization the same command writes
/// names, after which nothing is spent), and what writes the run.
pub(super) fn start(
    cx: &Cx,
    record: &Replay,
    (policy, states): (&Policy, &BTreeMap<String, &'static str>),
    stream: &str,
    fresh: Option<&[String]>,
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
            "REQ-RUN-2" => d = Some(prepare(cx, record, policy, stream, fresh, state(stream))?),
            "ALAMBIQUE_UNAPPROVED" if d.as_ref().is_some_and(|d: &Dispatch| d.halt.is_none()) => {
                crate::alambique::check(record, &cx.domain)?
            }
            _ => {}
        }
        Ok(())
    })?;
    let mut d = d.expect("REQ-RUN-2 is a guard of run-started");
    d.base.clone_from(&base);
    let authorization = match in_force(record, stream) {
        _ if fresh.is_some() => Value::Null,
        found => found.map_or(Value::Null, |(by, _)| by),
    };
    let payload = json!({"task": &stream["task-".len()..], "head": null, "authorization": authorization, "base": base});
    let (actor, say) = who;
    let policy = policy.digest.clone();
    let draft = Draft { kind: "run-started".into(), actor, policy, instance: String::new(), say: Some(say), payload };
    Ok((draft, Box::new(move |a, prepared| go(d, a, prepared))))
}

/// REQ-RUN-2 for `stream`'s run: the layers read with the widenings the authorization in force named (P9E-12); both
/// agents (with `review.differentProvider` at `require`, a reviewer of the builder's provider, or an absent reviewer's
/// tool, halts `PROVIDER_MISSING` unless the authorization named it, D-076); the required checks (REQ-GATE-1); the rounds
/// and the budget since the task's last authorization and its stage's (D-082), and the packet's size, which halt the
/// run. A verified task (`state`) is revalidated only when its evidence is stale (REQ-RUN-9): otherwise
/// `MOVE_NOT_ALLOWED`; and since a verified task cannot halt, what would halt it is refused.
fn prepare(
    cx: &Cx,
    record: &Replay,
    policy: &Policy,
    stream: &str,
    fresh: Option<&[String]>,
    state: &str,
) -> Result<Dispatch, Failed> {
    let events = &record.streams[stream];
    let p = &events[0].value["payload"];
    let stage_id = p["stage"].as_str().unwrap_or_default();
    let stage = &record.streams[&format!("stage-{stage_id}")];
    let layers = vec![
        (Source::Stage(stage_id.to_owned()), super::propose::layer(&stage[0].value["payload"])),
        (Source::Task(stream["task-".len()..].to_owned()), super::propose::layer(p)),
    ];
    // The widenings the authorization in force named, read against the policy in force now (P9E-12); an
    // authorization this command writes checked its own.
    let named: Vec<String> = match (fresh, in_force(record, stream)) {
        (Some(widen), _) => layers.iter().flat_map(|(_, l)| l.keys().cloned()).chain(widen.iter().cloned()).collect(),
        (None, found) => {
            let at = found.map(|(by, _)| {
                record.streams[by["stream"].as_str().unwrap_or_default()][by["seq"].as_u64().unwrap_or(1) as usize - 1]
                    .value["payload"]["widened"]
                    .clone()
            });
            at.and_then(|w| serde_json::from_value(w).ok()).unwrap_or_default()
        }
    };
    let machine = crate::machine::machine_file(&cx.folder()?)?;
    let eff = settings::effective(policy, None, machine.as_ref(), &layers, &named)?;
    let detection = detect::detect(&cx.domain, &eff, &cx.modules, std::env::var_os("PATH").as_deref());
    let found = (policy, &eff, &detection);
    let build = agent(cx, found, record, p, Role::Build)?;
    let differ = text(&eff, "review.differentProvider");
    let provider = |why: String| refuse(Code::ProviderMissing, None, why);
    let mut halts = Vec::new();
    let review = match agent(cx, found, record, p, Role::Review) {
        Err(e) if differ == "require" && e.code() == Some(Code::ToolMissing) => {
            halts.push(provider(format!("the reviewer's tool is absent: {}", e.detail())));
            None
        }
        r => Some(r?),
    };
    let same = review.as_ref().is_some_and(|r| r.adapter.provider == build.adapter.provider);
    // Only an authorization that named the key grants a reviewer of the builder's provider; a layer's own keys, which an
    // authorization this command writes admits to read the settings, do not (P9F-1).
    let granted = match fresh {
        Some(widen) => widen.to_vec(),
        None => named.clone(),
    };
    let widened = granted.iter().any(|k| k == "review.differentProvider");
    if same && differ == "require" && !widened {
        halts.push(provider(format!("the reviewer's provider, {}, is the builder's", build.adapter.provider)));
    }
    let checks = gate::required(&eff, p, &stage[0].value["payload"])?;
    let since = match (fresh, in_force(record, stream)) {
        (Some(_), _) => events.len(),
        (None, found) => found.map_or(0, |(_, since)| since),
    };
    let (rounds, counts) = gate::rounds(&events[since..]);
    let of_stage = crate::view::tasks_of(record, stage_id).into_iter().flat_map(|(_, e)| e.iter());
    // The task keeps its candidate for a revalidation when it is verified, or was authorized from a state that holds
    // one checked (REQ-RUN-9, REQ-GATE-8).
    let kept = events[..since].iter().rev().find(|e| e.kind != "task-authorized");
    let held = ["task-verified", "task-accepted", "merge-failed"];
    let last = events.iter().rfind(|e| e.kind == "candidate-recorded").map(|e| &e.value["payload"]);
    let product = super::propose::product(cx);
    let candidate = match last {
        Some(c) => {
            let (commit, tree) = (c["commit"].as_str().unwrap_or_default(), c["tree"].as_str().unwrap_or_default());
            Some((commit.to_owned(), tree.to_owned(), sha256_hex(&product_tree(&product, commit)?)))
        }
        None => None,
    };
    let revalidate =
        candidate.is_some() && (state == "verified" || kept.is_some_and(|e| held.contains(&e.kind.as_str())));
    let mut d = Dispatch {
        domain: cx.domain.clone(),
        home: home(),
        stream: stream.to_owned(),
        base: String::new(),
        task: p.clone(),
        stage: stage[0].value["payload"].clone(),
        backend: cx.backend,
        build,
        review,
        code: text(&eff, "languages.codeLanguage").to_owned(),
        spent: [spent(events[since..].iter()), spent(of_stage)],
        lanes: (
            p["allowed"].as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_owned).collect(),
            candidate::protected_names(&eff),
            candidate::protected_paths(&eff, &product)?,
        ),
        product,
        sign_off: eff.values.get("git.signOff").is_some_and(|v| v.0 == Val::Bool(true)),
        free: int(&eff, "limits.freeDiskMegabytes").unwrap_or(0),
        checks,
        policy: policy.digest.clone(),
        requirements: gate::requirements(p),
        limited: same && differ != "off",
        rounds,
        counts,
        previous: gate::previous(events, &cx.domain),
        candidate,
        revalidate,
        verified: state == "verified",
        detection,
        halt: None,
    };
    if state == "verified" && current(&d, events) {
        let why = "run-started from verified: its evidence is current, so a maestro accepts or rejects it";
        return Err(refuse(Code::MoveNotAllowed, None, why).into());
    }
    let limit = p["limits"]["rounds"].as_u64().unwrap_or(1);
    let packet = (!d.revalidate).then(|| d.packet(Role::Build).len()).filter(|n| *n > PACKET);
    let halt = [
        (d.rounds >= limit).then(|| exhausted(d.rounds, limit)),
        d.room().err(),
        halts.into_iter().next(),
        packet.map(|n| {
            refuse(
                Code::ToolFailed,
                None,
                format!("its packet holds {n} bytes, more than the {PACKET} a tool's argument may"),
            )
        }),
    ];
    d.halt = halt.into_iter().flatten().next();
    match d.halt.take() {
        Some(e) if state == "verified" => Err(e.into()),
        h => {
            d.halt = h;
            Ok(d)
        }
    }
}

fn exhausted(rounds: u64, limit: u64) -> Error {
    refuse(Code::RoundsExhausted, None, format!("{rounds} of its {limit} rounds since its authorization are spent"))
}

/// Whether a verified task's evidence is current (REQ-GATE-3, REQ-GATE-5): each required check's identity recomputed
/// now on its candidate, and its review, are what its `task-verified` binds.
fn current(d: &Dispatch, events: &[Event]) -> bool {
    let Some((_, tree, product)) = &d.candidate else { return false };
    let platform = gate::platform(d.backend.map_or("none", |b| b.name));
    let now: BTreeMap<String, String> =
        d.checks.iter().map(|c| (c.name.clone(), gate::identity(product, c, &d.policy, &platform))).collect();
    let verified = events.iter().rfind(|e| e.kind == "task-verified").map(|e| &e.value["payload"]);
    let review = |v: &Value| events.get(usize::try_from(v["review"].as_u64()?).ok()?.checked_sub(1)?);
    verified
        .and_then(|v| Some((v, &review(v)?.value["payload"])))
        .is_some_and(|(v, r)| gate::holds(v, r, tree, &d.requirements, &now))
}

/// Words of a failure an event may hold: the core's own, never an operating system's text (REQ-REG-13).
fn words(e: &Error) -> String {
    match e {
        Error::Refused { detail, .. } => detail.clone(),
        Error::Io { path, .. } => format!("{} cannot be read or written", path.display()),
    }
}

/// Halts the task with `e`'s code, a halting one, or `TOOL_FAILED`; the reason names `evidence` when there is one, and
/// every path in it relative to the repository's root or to the person's home (`~/`, P9E-2), so that the record holds
/// nothing of this machine's layout.
fn halt(a: &mut Appender, out: &mut Value, d: &Dispatch, e: &Error, evidence: Option<&Path>) -> Result<(), Error> {
    let code = e.code().map(|c| c.to_string()).filter(|c| crate::task::HALTS.contains(&c.as_str()));
    let code = code.unwrap_or_else(|| Code::ToolFailed.to_string());
    let mut reason = words(e);
    if let Some(dir) = evidence.filter(|d| !reason.contains(&*d.display().to_string())) {
        reason += &format!("; its output is kept in {}", dir.display());
    }
    let root = d.domain.parent().unwrap_or(&d.domain);
    let reason = reason.replace(&format!("{}/", root.display()), "").replace(&format!("{}/", d.home.display()), "~/");
    a.fabrewk(&d.stream, "task-halted", json!({"code": code, "reason": reason}))?;
    out["halt"] = json!({"code": code, "reason": reason});
    Ok(())
}

/// What stops a run: the halt and the evidence directory its reason names, if any.
struct Stop(Error, Option<PathBuf>);

/// A step of the run: an error of the command itself, or its outcome, which may be a halt.
type Step<T> = Result<Result<T, Stop>, Error>;

fn stop<T>(e: Error) -> Step<T> {
    Ok(Err(Stop(e, None)))
}

/// What a run's cells need: the backend, its programs and the platform a check's identity holds.
struct Ready<'r> {
    programs: &'r Programs,
    backend: &'r Backend,
    platform: String,
}

/// The run under the lock (see [`start`]): the events planned before it (an authorization) are written first; then
/// CONFINEMENT_UNAVAILABLE (a check with network, which this backend cannot confine, D-085; the backend, its probe and
/// each agent's tool's preflight, each unless a use verified it) and DISK_LOW, which halt the task as a halt the guards
/// reached does; then `run-started` and the rounds, until a round verifies the task or a halt stops it. A verified task,
/// which the table halts from no state but `authorized` and `running`, has its `run-started` written first.
fn go(mut d: Dispatch, a: &mut Appender, mut prepared: Vec<(String, Prepared)>) -> Result<Value, Error> {
    let (stream, started) = prepared.pop().expect("run-started is the plan's last event");
    for (s, p) in prepared {
        a.write(s, p)?;
    }
    let mut started = Some(started);
    if d.verified {
        a.write(stream.clone(), started.take().expect("not written yet"))?;
    }
    let mut out = json!({"task": &stream["task-".len()..], "halt": null, "candidate": null, "attempt": null});
    if let Some(e) = d.halt.take() {
        halt(a, &mut out, &d, &e, None)?;
        return Ok(out);
    }
    let ready = (|| {
        if let Some(c) = d.checks.iter().find(|c| c.definition["network"] == true) {
            let why =
                format!("the check {} has network, which this backend cannot confine without letting it nest", c.name);
            return Err(refuse(Code::ConfinementUnavailable, None, why));
        }
        let programs = confine::available(d.backend, &mut d.detection, &d.domain)?;
        let backend = d.backend.expect("an available backend");
        a.used.extend(backend.programs.iter().map(|p| (Space::Program, p.executable.to_owned())));
        // A preflight's pass is kept as verified by use for the tool's path and version (3.9), one per tool.
        for agent in [Some(&d.build), d.review.as_ref()].into_iter().flatten() {
            let used = (Space::Tool, agent.tool.clone());
            if d.detection.get(Space::Tool, &agent.tool).is_none_or(|f| f.verified.is_none()) && !a.used.contains(&used)
            {
                let tool = Tool { adapter: agent.adapter, executable: &agent.exe, release: &agent.release };
                adapter::preflight(tool, backend, &programs, &d.domain)?;
            }
            a.used.push(used);
        }
        d.free_space()?;
        Ok::<_, Error>((programs, backend))
    })();
    let (programs, backend) = match ready {
        Ok(ready) => ready,
        Err(e) => {
            halt(a, &mut out, &d, &e, None)?;
            return Ok(out);
        }
    };
    if let Some(started) = started {
        a.write(stream, started)?;
    }
    let r = Ready { programs: &programs, backend, platform: gate::platform(backend.name) };
    loop {
        match round(&mut d, a, &mut out, &r)? {
            Ok(true) => return Ok(out),
            Ok(false) => {}
            Err(Stop(e, at)) => {
                halt(a, &mut out, &d, &e, at.as_deref())?;
                return Ok(out);
            }
        }
    }
}

/// One round (REQ-GATE-6): the round limit, checked before its first dispatch (REQ-RUN-2); the builder's attempt unless
/// the round revalidates (REQ-RUN-9); each required check on the candidate; and, once every one passed, the reviewer's
/// attempt. Gives `true` once the task is verified (REQ-GATE-5), `false` when the round ended with findings.
fn round(d: &mut Dispatch, a: &mut Appender, out: &mut Value, r: &Ready) -> Step<bool> {
    let limit = d.task["limits"]["rounds"].as_u64().unwrap_or(1);
    if d.rounds >= limit {
        return stop(exhausted(d.rounds, limit));
    }
    d.rounds += 1;
    if !std::mem::take(&mut d.revalidate)
        && let Err(s) = build(d, a, out, r)?
    {
        return Ok(Err(s));
    }
    let (commit, tree, product) = d.candidate.clone().expect("a round has its candidate");
    let (mut identities, mut failed) = (BTreeMap::new(), Vec::new());
    for c in d.checks.clone() {
        let identity = gate::identity(&product, &c, &d.policy, &r.platform);
        match check(d, a, r, &c, &commit, &identity)? {
            Ok(f) => failed.extend(f),
            Err(s) => return Ok(Err(s)),
        }
        identities.insert(c.name, identity);
    }
    // Each check that did not pass is one confirmed blocking finding, and no reviewer is dispatched (REQ-GATE-6).
    if !failed.is_empty() {
        return Ok(next(d, failed.len() as u64, (Vec::new(), failed)));
    }
    let (review, seq) = match review(d, a, out, r, &commit, &tree)? {
        Ok(review) => review,
        Err(s) => return Ok(Err(s)),
    };
    if review["verdict"] == "reject" {
        let found = (gate::confirmed(&review["findings"]), Vec::new());
        return Ok(next(d, gate::blocking(&review["findings"]), found));
    }
    let verified =
        json!({"candidate": commit, "tree": tree, "requirements": d.requirements, "checks": identities, "review": seq});
    a.fabrewk(&d.stream, "task-verified", verified)?;
    out["verified"] = json!(true);
    Ok(Ok(true))
}

/// The end of a round with findings: its progress measure counted, then `NO_PROGRESS`, evaluated first (REQ-TASK-5);
/// else the next round carries what this one found (REQ-RUN-8).
fn next(d: &mut Dispatch, count: u64, found: (Vec<Value>, Vec<Value>)) -> Result<bool, Stop> {
    d.counts.push(count);
    if gate::no_progress(&d.counts) {
        let by: Vec<String> = d.counts.iter().map(u64::to_string).collect();
        let why = format!(
            "the confirmed blocking findings by round since its authorization are {}: reduce the scope, change the \
             design or replace a component",
            by.join(", ")
        );
        return Err(Stop(refuse(Code::NoProgress, None, why), None));
    }
    d.previous = found;
    Ok(false)
}

/// Keeps `words`, the core's own, in the evidence file `name`, `0600` (REQ-REG-13); gives its path.
fn keep(evidence: &Path, name: &str, words: &str) -> Result<PathBuf, Error> {
    let file = evidence.join(name);
    let mut options = std::fs::File::options();
    let made = options.write(true).create_new(true).mode(0o600).open(&file);
    made.and_then(|mut f| f.write_all(words.as_bytes())).map_err(crate::code::io_at(&file))?;
    Ok(file)
}

/// One attempt of `role`'s agent on a copy of `source` (REQ-RUN-1, REQ-RUN-2, REQ-RUN-10): the packet's size, the
/// budget left and the free space; `attempt-started`; the export (a failure ends the attempt before any process,
/// PD-21); the dispatch; and `attempt-ended`, `success` when the tool exited 0 and its stream's result fits the role's
/// schema. Gives the stream's report, the attempt's place, its directories, which the caller reads or deletes unread,
/// and its evidence.
fn dispatch(
    d: &mut Dispatch,
    a: &mut Appender,
    out: &mut Value,
    r: &Ready,
    role: Role,
    source: &str,
) -> Step<(Report, u64, Scratch, PathBuf)> {
    let agent = d.agent(role).clone();
    let packet = d.packet(role);
    if packet.len() > PACKET {
        let n = packet.len();
        return stop(refuse(
            Code::ToolFailed,
            None,
            format!("its packet holds {n} bytes, more than the {PACKET} a tool's argument may"),
        ));
    }
    let left = match d.room().and_then(|left| d.free_space().map(|_| left)) {
        Ok(left) => left,
        Err(e) => return stop(e),
    };
    let (allowed, capped) = (left.min(agent.seconds), left < agent.seconds);
    let started = json!({
        "role": role.name(), "profile": agent.profile, "tool": agent.tool, "version": agent.version,
        "model": agent.model, "effort": agent.effort, "allowedSeconds": allowed,
    });
    let seq = a.fabrewk(&d.stream, "attempt-started", started)?.seq;
    out["attempt"] = json!(seq);
    let name = attempt_name(&d.stream, seq);
    let scratch = Scratch::create(&d.domain, &name)?;
    let evidence = confine::evidence(&d.domain, &name)?;
    // An attempt's end, and what it spent: its seconds (an unknown end the whole time allowed, S-5) and its tokens.
    let mut end = |a: &mut Appender,
                   outcome: &str,
                   model: Option<&str>,
                   seconds: Option<u64>,
                   tokens: Option<Tokens>,
                   kept| {
        for s in &mut d.spent {
            s.0 += 1;
            s.1 = s.1.saturating_add(seconds.unwrap_or(allowed));
            s.2 = s.2.and_then(|t| t.checked_add(tokens?.input?)?.checked_add(tokens?.output?));
        }
        let payload = json!({"attempt": seq, "outcome": outcome, "model": model, "usage": adapter::usage(seconds, tokens), "evidence": kept});
        a.fabrewk(&d.stream, "attempt-ended", payload).map(drop)
    };
    if let Err(e) = candidate::export(&d.product, source, &scratch.work) {
        let file = keep(&evidence, "export", &words(&e))?;
        end(a, "failure", None, None, None, Value::Null)?;
        scratch.remove()?;
        let why = format!("the tree could not be exported for the attempt; why is kept in {}", file.display());
        return stop(refuse(Code::ToolFailed, Some(&file), why));
    }
    let tool = Tool { adapter: agent.adapter, executable: &agent.exe, release: &agent.release };
    let schema = adapter::schema(role).to_string();
    let launch = Launch { role, packet: &packet, schema: &schema, model: &agent.model, effort: &agent.effort };
    let limits = Limits { time: Duration::from_secs(allowed), bytes: OUTPUT };
    fault::pause("before-dispatch");
    let ran = adapter::dispatch(tool, r.backend, r.programs, &launch, &d.home, &scratch, &evidence, limits);
    fault::pause("before-attempt-ended");
    let (ran, report) = match ran {
        Err(u) if !u.ended => {
            end(a, "unknown", None, None, None, Value::Null)?;
            let why = format!("its cell could not be shown ended, so it is left unread: {}", words(&u.error));
            return Ok(Err(Stop(refuse(Code::OutcomeUnknown, Some(&evidence), why), Some(evidence))));
        }
        Err(u) => {
            end(a, "failure", None, None, None, Value::Null)?;
            scratch.remove()?;
            return Ok(Err(Stop(u.error, Some(evidence))));
        }
        Ok(ran) => ran,
    };
    let kept = json!({"stdout": ran.stdout.digest, "stderr": ran.stderr.digest});
    let failed = match (ran.end, &report.result) {
        (End::TimedOut, _) if capped => {
            Some(refuse(Code::BudgetExhausted, Some(&evidence), "the task's or its stage's minutes are spent"))
        }
        (End::Exited(0), None) if role == Role::Review => {
            let why = format!("{}'s stream cannot be read as a review, so it counts as not examined", agent.tool);
            Some(refuse(Code::ReviewUnexamined, Some(&evidence), why))
        }
        (End::Exited(0), None) => {
            Some(refuse(Code::ToolFailed, Some(&evidence), format!("{}'s stream gave no result", agent.tool)))
        }
        (End::Exited(0), Some(result)) => adapter::validate(role, result).err(),
        _ => Some(ran.failure(&agent.tool)),
    };
    let outcome = if failed.is_some() { "failure" } else { "success" };
    end(a, outcome, report.model.as_deref(), Some(ran.seconds), report.tokens, kept)?;
    if let Some(e) = failed {
        scratch.remove()?;
        return Ok(Err(Stop(e, Some(evidence))));
    }
    Ok(Ok((report, seq, scratch, evidence)))
}

/// The builder's attempt (REQ-RUN-3, REQ-RUN-4): on a copy of the task's last candidate when it has one, so that it
/// corrects rather than rebuilds (D-085), else of its base; then the change set against the base, its guards, and the
/// candidate on `fabrewk/<task>`, whose parent is the base.
fn build(d: &mut Dispatch, a: &mut Appender, out: &mut Value, r: &Ready) -> Step<()> {
    let source = d.candidate.as_ref().map_or_else(|| d.base.clone(), |c| c.0.clone());
    let (report, _, scratch, evidence) = match dispatch(d, a, out, r, Role::Build, &source)? {
        Ok(x) => x,
        Err(s) => return Ok(Err(s)),
    };
    let made = candidate(d, a, out, &report, &scratch, &evidence);
    scratch.remove()?;
    made
}

/// The change set of a builder that succeeded, and its candidate, recorded (`candidate-recorded`). What fails after the
/// builder is not its output: the change set's own refusals name nothing; any other, Git's above all, keeps the core's
/// words as its own evidence, Git's own text being kept nowhere until part 9g (P9E-11, D-083).
fn candidate(
    d: &mut Dispatch,
    a: &mut Appender,
    out: &mut Value,
    report: &Report,
    scratch: &Scratch,
    evidence: &Path,
) -> Step<()> {
    let after = |e: Error| -> Step<()> {
        let own = [Code::LaneViolation, Code::ProtectedPath, Code::ProtectedPathWildcard, Code::CredentialSuspected];
        match e.code() {
            None => return Err(e),
            Some(c) if own.contains(&c) => return stop(e),
            Some(_) => {}
        }
        let file = keep(evidence, "candidate", &words(&e))?;
        let why = format!("{}; the candidate could not be made, and why is kept in {}", words(&e), file.display());
        stop(refuse(Code::ToolFailed, Some(&file), why))
    };
    let (allowed, names, paths) = &d.lanes;
    let allowed: Vec<&str> = allowed.iter().map(String::as_str).collect();
    let lanes = Lanes { allowed: &allowed, names, paths };
    let changes = match candidate::changes(&d.product, &d.base, &scratch.work, &scratch.root, &lanes) {
        Ok(c) => c,
        Err(e) => return after(e),
    };
    let task = &d.stream["task-".len()..];
    let previous = match git::head_of(&d.product, &format!("fabrewk/{task}")) {
        Ok(p) => p,
        Err(e) => return after(e),
    };
    // The model observed names the co-author, or the one requested when the stream names none, or only white space.
    let model = report.model.as_deref().filter(|m| !m.trim().is_empty()).unwrap_or(&d.build.model);
    let agent = candidate::Agent { tool: &d.build.tool, provider: d.build.adapter.provider, model };
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
    let c = match made.and_then(|c| Ok((sha256_hex(&product_tree(&d.product, &c.commit)?), c))) {
        Ok(c) => c,
        Err(e) => return after(e),
    };
    let (product, c) = c;
    let entry = |e: &Option<candidate::Entry>| e.as_ref().map(|e| json!({"mode": e.mode.git(), "oid": e.oid}));
    let listed: Vec<Value> =
        changes.iter().map(|c| json!({"path": c.path, "before": entry(&c.before), "after": entry(&c.after)})).collect();
    a.fabrewk(&d.stream, "candidate-recorded", json!({"commit": c.commit, "tree": c.tree, "changes": listed}))?;
    out["candidate"] = json!({"commit": c.commit, "tree": c.tree, "branch": format!("fabrewk/{task}")});
    d.candidate = Some((c.commit, c.tree, product));
    Ok(Ok(()))
}

/// The reviewer's attempt on a fresh copy of the candidate, deleted unread (REQ-GATE-4, T-20): its result recorded
/// with the audit's count (`review-recorded`), bound to the candidate's tree and the requirements' digest, with the
/// provider its adapter declares and the limitation of a reviewer of the builder's provider; a review that examined
/// nothing is `REVIEW_UNEXAMINED`, whatever its verdict (PD-25, PD-26). Gives the review and its place.
fn review(
    d: &mut Dispatch,
    a: &mut Appender,
    out: &mut Value,
    r: &Ready,
    commit: &str,
    tree: &str,
) -> Step<(Value, u64)> {
    let (report, attempt, scratch, evidence) = match dispatch(d, a, out, r, Role::Review, commit)? {
        Ok(x) => x,
        Err(s) => return Ok(Err(s)),
    };
    scratch.remove()?;
    let result = report.result.unwrap_or_default();
    let review = json!({
        "attempt": attempt, "round": d.rounds, "verdict": result["verdict"], "findings": result["findings"],
        "examined": report.examined, "tree": tree, "requirements": d.requirements,
        "provider": d.agent(Role::Review).adapter.provider, "limited": d.limited,
    });
    let seq = a.fabrewk(&d.stream, "review-recorded", review.clone())?.seq;
    if report.examined == 0 {
        let why = format!("{}'s stream shows no examination of the candidate", d.agent(Role::Review).tool);
        return Ok(Err(Stop(refuse(Code::ReviewUnexamined, Some(&evidence), why), Some(evidence))));
    }
    Ok(Ok((review, seq)))
}

/// One required check on a fresh export of `commit` (REQ-GATE-2, REQ-RUN-10): its cell, with no network, no
/// credential and its own time limit, which spends nothing of the task's (D-085); its outcome recorded with its
/// identity and evidence (`check-recorded`); its copy deleted unread. Gives what the next round carries of a check that
/// did not pass.
fn check(
    d: &Dispatch,
    a: &mut Appender,
    r: &Ready,
    c: &gate::Check,
    commit: &str,
    identity: &str,
) -> Step<Option<Value>> {
    if let Err(e) = d.free_space() {
        return stop(e);
    }
    let name = gate::check_name(&d.stream, a.next(&d.stream)?);
    let scratch = Scratch::create(&d.domain, &name)?;
    let evidence = confine::evidence(&d.domain, &name)?;
    let record = |a: &mut Appender, outcome: &str, kept: Value| {
        let p = json!({"check": c.name, "round": d.rounds, "identity": identity, "outcome": outcome, "evidence": kept});
        a.fabrewk(&d.stream, "check-recorded", p).map(drop)
    };
    if let Err(e) = candidate::export(&d.product, commit, &scratch.work) {
        let file = keep(&evidence, "export", &words(&e))?;
        scratch.remove()?;
        let why =
            format!("the candidate could not be exported for the check {}; why is kept in {}", c.name, file.display());
        return stop(refuse(Code::ToolFailed, Some(&file), why));
    }
    let time = Duration::from_secs(c.definition["minutes"].as_u64().unwrap_or(1).saturating_mul(fault::minute()));
    let ran = confine::run(
        r.backend,
        r.programs,
        &gate::cell(&scratch, c),
        &scratch,
        &evidence,
        Limits { time, bytes: OUTPUT },
    );
    let ran = match ran {
        Err(u) if !u.ended => {
            record(a, "unknown", Value::Null)?;
            let why = format!(
                "the check {}'s cell could not be shown ended, so it is left unread: {}",
                c.name,
                words(&u.error)
            );
            return Ok(Err(Stop(refuse(Code::OutcomeUnknown, Some(&evidence), why), Some(evidence))));
        }
        Err(u) => {
            scratch.remove()?;
            return Ok(Err(Stop(u.error, Some(evidence))));
        }
        Ok(ran) => ran,
    };
    scratch.remove()?;
    let outcome = gate::outcome(&ran);
    record(a, outcome, json!({"stdout": ran.stdout.digest, "stderr": ran.stderr.digest}))?;
    Ok(Ok((outcome != "passed").then(|| gate::output(&evidence, &c.name, outcome))))
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
    if ran["verified"] == true {
        text += &(cx.t("run.verified", &[("id", id)]) + "\n");
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
