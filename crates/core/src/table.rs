//! The transition table (REQ-TASK-1, REQ-SEC-5; core-specification section 4; part 7d1, D-053): every state, move,
//! actor, guard, refusal and control label, declared once. Commands evaluate a move's guards in the order it lists
//! them; the replay holds every stage and task stream to it. Proposals are held to it by [`crate::fermento::proposals`] (part 8a); the
//! `project` stream's order is `apply`'s (phase 10).

use crate::code::{Code, Error, refuse};
use crate::record::{Event, Replay, StreamKind, stream_kind, subject_digest};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::LazyLock;

/// One row per line: subject | kinds | from | to | who | guards. A kind marked `+` is recorded after the row's move,
/// from its target state; one marked `^` is another stream's move that makes this one. `-` is no state: the subject's
/// creation. `say` marks a move that takes the person's words. A guard is a refusal code, or a requirement whose
/// codes it stands for; one marked `~` is cooperative (a process of the same user could bypass it), the others
/// effective (enforced by a signature, by code with confinement, or by the Git host).
const TABLE: &str = "
task | task-proposed | - | proposed | anyone | STAGE_CLOSED~ TASK_INVALID~ PROTECTED_PATH~ PROTECTED_PATH_WILDCARD~
task | task-authorized ^stage-authorized | proposed halted verified accepted | authorized | maestro say | ROLE_FORBIDDEN WORDS_MISSING WORDS_COPIED~ TASK_CHANGED PROTECTED_PATH PROTECTED_PATH_WILDCARD WIDENING_UNNAMED LIMIT_UNOBSERVABLE
task | run-started +attempt-started | authorized verified | running | maestro say | POLICY_ABSENT DEPENDENCY_OPEN BASE_MOVED LOCK_HELD~ REQ-RUN-2~ ALAMBIQUE_UNAPPROVED CONFINEMENT_UNAVAILABLE
task | attempt-ended candidate-recorded check-recorded review-recorded | running | running | fabrewk | ATTEMPT_ENDED LANE_VIOLATION PROTECTED_PATH CREDENTIAL_SUSPECTED~ REVIEW_INVALID
task | task-verified | running | verified | fabrewk | REQ-GATE-5
task | task-halted | authorized running | halted | fabrewk | REQ-TASK-5
task | task-accepted | authorized halted verified | accepted | maestro say | REQ-GATE-7 EVIDENCE_STALE ALAMBIQUE_UNAPPROVED
task | task-integrated | accepted | integrated | fabrewk | REQ-GATE-8 BASE_MOVED WORKTREE_DIRTY EVIDENCE_STALE
task | merge-failed | accepted | accepted | fabrewk |
task | task-rejected | proposed authorized halted verified accepted | rejected | maestro say | MERGE_IN_PROGRESS
stage | stage-proposed | - | proposed | anyone | TASK_INVALID~ CHECK_DUPLICATE~
stage | stage-authorized | proposed | authorized | maestro say | ROLE_FORBIDDEN WORDS_MISSING WORDS_COPIED~ TASK_CHANGED CHECK_DUPLICATE PROTECTED_PATH PROTECTED_PATH_WILDCARD WIDENING_UNNAMED LIMIT_UNOBSERVABLE
stage | stage-closed | authorized | closed | maestro say | STAGE_OPEN_TASKS SWEEP_MISSING ALAMBIQUE_UNAPPROVED RECORD_UNCOMMITTED
proposal | proposal-recorded | - | pending | agent | REQ-FER-3
proposal | proposal-incorporated | pending | incorporated | maestro say | PROPOSAL_STALE ALAMBIQUE_CHANGED ALAMBIQUE_UNAPPROVED UNIT_AMBIGUOUS
proposal | proposal-discarded | pending | discarded | maestro say |
proposal | proposal-withdrawn | pending | withdrawn | fabrewk |
";

/// Whether a control is enforced or rests on cooperation (REQ-SEC-5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Control {
    Effective,
    Cooperative,
}

/// A guard: a refusal code or a requirement, and its label.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Guard {
    pub name: &'static str,
    pub control: Control,
}

/// One row of the table.
#[derive(Debug, Clone)]
pub struct Move {
    pub subject: &'static str,
    pub kinds: Vec<&'static str>,
    pub then: Vec<&'static str>,
    pub covered: Vec<&'static str>,
    pub from: Vec<&'static str>,
    pub to: &'static str,
    pub who: &'static str,
    pub say: bool,
    pub guards: Vec<Guard>,
}

static MOVES: LazyLock<Vec<Move>> = LazyLock::new(|| {
    let row = |line: &'static str| {
        let c: Vec<&str> = line.split('|').map(str::trim).collect();
        let words = |i: usize| c[i].split_whitespace();
        let guard = |g: &'static str| match g.strip_suffix('~') {
            Some(name) => Guard { name, control: Control::Cooperative },
            None => Guard { name: g, control: Control::Effective },
        };
        Move {
            subject: c[0],
            kinds: words(1).filter(|k| !k.starts_with(['+', '^'])).collect(),
            then: words(1).filter_map(|k| k.strip_prefix('+')).collect(),
            covered: words(1).filter_map(|k| k.strip_prefix('^')).collect(),
            from: words(2).filter(|s| *s != "-").collect(),
            to: c[3],
            who: words(4).next().unwrap_or_default(),
            say: words(4).any(|w| w == "say"),
            guards: words(5).map(guard).collect(),
        }
    };
    TABLE.lines().filter(|l| !l.is_empty()).map(row).collect()
});

/// Every row, in the table's order.
pub fn moves() -> &'static [Move] {
    &MOVES
}

/// The row that records `kind` for `subject`, as its own move or one recorded after it.
pub fn find(subject: &str, kind: &str) -> Option<&'static Move> {
    moves().iter().find(|m| m.subject == subject && (m.kinds.contains(&kind) || m.then.contains(&kind)))
}

/// Every state of `subject`.
pub fn states(subject: &str) -> BTreeSet<&'static str> {
    let rows = moves().iter().filter(|m| m.subject == subject);
    rows.flat_map(|m| m.from.iter().copied().chain([m.to])).collect()
}

/// The kinds `subject` may record from `state` (`None` before its creation).
pub fn allowed(subject: &str, state: Option<&str>) -> Vec<&'static str> {
    let from = |m: &Move, k: &str| if m.then.contains(&k) { vec![m.to] } else { m.from.clone() };
    let ok = |m: &Move, k: &str| state.map_or(from(m, k).is_empty(), |s| from(m, k).contains(&s));
    let rows = moves().iter().filter(|m| m.subject == subject);
    rows.flat_map(|m| m.kinds.iter().chain(&m.then).filter(move |k| ok(m, k)).copied()).collect()
}

/// The row of `kind` if the table allows it from `state`; `MOVE_NOT_ALLOWED` otherwise, naming the state and the
/// moves it allows (REQ-TASK-1).
pub fn step(subject: &str, state: Option<&str>, kind: &str) -> Result<&'static Move, Error> {
    match find(subject, kind) {
        Some(m) if allowed(subject, state).contains(&kind) => Ok(m),
        _ => {
            let (state, ok) = (state.unwrap_or("none"), allowed(subject, state).join(", "));
            Err(refuse(Code::MoveNotAllowed, None, format!("{kind} from {state}; allowed: {ok}")))
        }
    }
}

/// Evaluates a move's guards in the order the table lists them; the first that fails is the refusal (REQ-TASK-1).
pub fn evaluate<E>(m: &Move, guard: impl FnMut(&Guard) -> Result<(), E>) -> Result<(), E> {
    m.guards.iter().try_for_each(guard)
}

fn invalid(e: &Event, why: impl Into<String>) -> Error {
    refuse(Code::RecordTransitionInvalid, Some(&e.path), why)
}

/// The replay held to the table (REQ-REG-6; D-047, D-053). Each stage and task stream's moves are allowed from the
/// state its earlier events left, with their payloads' content (REQ-TASK-2, REQ-TASK-3) and their bindings to their
/// own subject's identifier, digest and previous head (PD-06; a rejection, an acceptance and a run bind no digest, part
/// 7d3b; a run binds the authorization in force too, part 9e); a stage's authorization makes each task it binds
/// authorized after the head it names, which its stream must hold (`RECORD_ROLLBACK` otherwise). An end names its
/// attempt: a second end is `ATTEMPT_ENDED` (PD-10), the end of one a later attempt replaced `ATTEMPT_REPLACED` (PD-08);
/// an attempt that starts while another is open, or a move out of `running` with one open, is
/// `RECORD_TRANSITION_INVALID`, as is every other failure: each attempt ends exactly once (REQ-RUN-1).
pub fn validate(record: &Replay) -> Result<(), Vec<Error>> {
    let mut errors = current(record).err().unwrap_or_default();
    errors.extend(project(record));
    match crate::fermento::proposals(record) {
        Ok(all) => errors.extend(crate::alambique::validate(record, &all)),
        Err(e) => errors.extend(e),
    }
    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

/// The `project` stream's order (section 4, "Project"; part 10b): `apply` opens it with `assessed`,
/// `publication-decided`, `policy-adopted` and `installed`, in that order, and neither `assessed` nor `installed` stands
/// anywhere else; an installation stopped before its end holds a prefix of that opening.
fn project(record: &Replay) -> Option<Error> {
    const OPENING: [&str; 4] = ["assessed", "publication-decided", "policy-adopted", "installed"];
    let events = record.streams.get("project").map(Vec::as_slice).unwrap_or_default();
    let opened = events.first().is_some_and(|e| e.kind == "assessed");
    let out = events.iter().enumerate().find(|(i, e)| match OPENING.get(*i).filter(|_| opened) {
        Some(kind) => e.kind != *kind,
        None => matches!(e.kind.as_str(), "assessed" | "installed"),
    });
    out.map(|(_, e)| invalid(e, "the project stream opens with apply's events, in their order, and holds them once"))
}

/// The state of every stage and task stream after its events, held to the table as [`validate`] holds them.
pub fn current(record: &Replay) -> Result<BTreeMap<String, &'static str>, Vec<Error>> {
    let (mut covers, mut out, mut errors) = (Covers::new(), BTreeMap::new(), Vec::new());
    let of = |k| record.streams.iter().filter(move |(n, _)| stream_kind(n) == Some(k));
    for s in of(StreamKind::Stage) {
        match stage(record, s, &mut covers) {
            Ok(state) => _ = out.insert(s.0.clone(), state),
            Err(e) => errors.push(e),
        }
    }
    for t in of(StreamKind::Task) {
        match task(record, t, &covers) {
            Ok(state) => _ = out.insert(t.0.clone(), state),
            Err(e) => errors.push(e),
        }
    }
    if errors.is_empty() { Ok(out) } else { Err(errors) }
}

/// One event's move from `state`, its payload's content and, for a person's move bound to its subject, its binding.
fn walk(subject: &str, state: Option<&str>, e: &Event, digest: &str) -> Result<&'static str, Error> {
    let m = step(subject, state, &e.kind).map_err(|x| invalid(e, x.detail()))?;
    let content = |x: Error| match x.code() {
        Some(Code::TaskInvalid) => invalid(e, x.detail()),
        _ => x,
    };
    crate::task::check(&e.kind, &e.value["payload"]).map_err(content)?;
    if ["stage-authorized", "stage-closed", "task-authorized", "task-accepted", "task-rejected", "run-started"]
        .contains(&e.kind.as_str())
    {
        let p = &e.value["payload"];
        let id = &e.stream[subject.len() + 1..];
        let head = json!({"seq": e.seq - 1, "digest": e.value["prev"]});
        if p[subject] != id || p["head"] != head || (e.kind.ends_with("-authorized") && p["digest"] != digest) {
            return Err(invalid(e, "bound to another subject, its digest or its head"));
        }
    }
    Ok(m.to)
}

/// Each task a stage's authorization binds, at its stream's head then: the authorization's stream, place and digest.
type Covers = BTreeMap<(String, u64), Value>;

/// An authorization as `run-started` names it: its stream, its place and its digest.
fn authorization(e: &Event) -> Value {
    json!({"stream": e.stream, "seq": e.seq, "digest": e.digest})
}

fn stage(record: &Replay, (name, events): (&String, &Vec<Event>), covers: &mut Covers) -> Result<&'static str, Error> {
    let digest = subject_digest(&events[0].value["payload"]).unwrap_or_default();
    let mut state = None;
    for e in events {
        state = Some(walk("stage", state, e, &digest)?);
        for t in e.value["payload"]["tasks"].as_array().filter(|_| e.kind == "stage-authorized").into_iter().flatten() {
            let stream = format!("task-{}", t["task"].as_str().unwrap_or_default());
            let first = record.streams.get(&stream).and_then(|s| s.first());
            let own = first.filter(|f| {
                f.value["payload"]["stage"] == name[6..]
                    && t["digest"] == subject_digest(&f.value["payload"]).unwrap_or_default()
            });
            if own.is_none() {
                return Err(invalid(e, format!("{stream}: not a task of this stage, or not its digest")));
            }
            let seq = t["head"]["seq"].as_u64().unwrap_or_default();
            let held = record.streams[&stream].get(seq as usize - 1).is_some_and(|h| t["head"]["digest"] == h.digest);
            if !held {
                return Err(refuse(
                    Code::RecordRollback,
                    Some(&e.path),
                    format!("{stream}: a head its stream does not hold"),
                ));
            }
            covers.insert((stream, seq), authorization(e));
        }
    }
    Ok(state.unwrap_or_default())
}

fn task(record: &Replay, (name, events): (&String, &Vec<Event>), covers: &Covers) -> Result<&'static str, Error> {
    let payload = &events[0].value["payload"];
    let digest = subject_digest(payload).unwrap_or_default();
    let stage = format!("stage-{}", payload["stage"].as_str().unwrap_or_default());
    let deps = payload["dependencies"].as_array().into_iter().flatten();
    if let Some(missing) = std::iter::once(stage)
        .chain(deps.map(|d| format!("task-{}", d.as_str().unwrap_or_default())))
        .find(|s| !record.streams.contains_key(s))
    {
        return Err(invalid(&events[0], format!("{missing}: no such stage or task")));
    }
    let authorize = find("task", "task-authorized").expect("the table holds task-authorized");
    // The open attempt and every attempt started, by the place of its start; the authorization in force.
    let (mut state, mut open, mut started, mut auth) = (None, None, BTreeSet::new(), Value::Null);
    for e in events {
        // An end names its attempt (REQ-RUN-1, REQ-RUN-5): one already ended, or replaced by a later one, is refused
        // whatever the state, before the move.
        if e.kind == "attempt-ended" {
            let attempt = e.value["payload"]["attempt"].as_u64();
            if attempt.is_none_or(|a| Some(a) != open) {
                let code = match attempt {
                    Some(a) if started.contains(&a) && started.last() > Some(&a) => Code::AttemptReplaced,
                    // An attempt started and not replaced, while one is open, is the open one: none is open here.
                    _ if open.is_none() => Code::AttemptEnded,
                    _ => Code::RecordTransitionInvalid,
                };
                return Err(refuse(code, Some(&e.path), "an attempt ended twice, or never started, or was replaced"));
            }
        }
        let to = walk("task", state, e, &digest)?;
        if open.is_some() && (e.kind == "attempt-started" || to != "running") {
            return Err(invalid(e, "an attempt is open: it ends before another starts or the task leaves running"));
        }
        if e.kind == "run-started" && e.value["payload"]["authorization"] != auth {
            return Err(invalid(e, "the run names another authorization than the one in force"));
        }
        // An acceptance binds the task's last candidate (REQ-GATE-7).
        if e.kind == "task-accepted" {
            let p = &e.value["payload"];
            // Up to the acceptance, itself no candidate.
            let last = events[..e.seq as usize].iter().rfind(|x| x.kind == "candidate-recorded");
            let last = last.map(|c| &c.value["payload"]);
            if !last.is_some_and(|c| c["commit"] == p["candidate"] && c["tree"] == p["tree"]) {
                return Err(invalid(e, "an acceptance of another candidate than the task's last"));
            }
        }
        match e.kind.as_str() {
            "attempt-started" => {
                open = Some(e.seq);
                started.insert(e.seq);
            }
            "attempt-ended" => open = None,
            "task-authorized" => auth = authorization(e),
            _ => {}
        }
        state = Some(to);
        if let Some(by) = covers.get(&(name.clone(), e.seq)) {
            if !authorize.from.contains(&to) {
                return Err(invalid(e, "a stage's authorization binds a task that waits for none"));
            }
            state = Some(authorize.to);
            auth = by.clone();
        }
    }
    Ok(state.unwrap_or_default())
}
