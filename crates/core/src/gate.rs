//! Checks, review and verification as a library (part 9f; REQ-GATE-1 to REQ-GATE-3, REQ-GATE-5, REQ-GATE-6's progress
//! measure, REQ-TASK-5's `NO_PROGRESS`, REQ-RUN-8's previous round; D-085): the required checks and their definitions,
//! a check result's identity, a check's cell and outcome, whether a verification still holds, the confirmed blocking
//! findings of each round, and what one round leaves for the next. The run's rounds are `cli::run`'s.

use crate::adapter::CELL_ENV;
use crate::code::{Code, Error, refuse};
use crate::confine::{Access, Cell, End, Mount, Network, Ran, Scratch};
use crate::record::{Event, encode, sha256_hex};
use crate::settings::{Effective, Val};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// A required check: its name, its definition (`command`, `network`, `minutes`, `tools`) and each tool it names as
/// its cell finds it, where it resolves and the version it reports, as the check's identity holds them.
#[derive(Debug, Clone)]
pub struct Check {
    pub name: String,
    pub definition: Value,
    pub tools: Value,
}

fn texts(v: &Value) -> impl Iterator<Item = String> + '_ {
    v.as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_owned)
}

/// REQ-GATE-1: a task's required checks, by name: the union of its own (`task`, its proposal), its build profile's and
/// those its stage (`stage`, its proposal) approved, each with its definition, the stage's or the policy's, which must be
/// the same, as the policy in force reads (`CHECK_DUPLICATE`); `SETTINGS_INVALID` for a name neither defines. A tool a check names must be found on
/// the cell's own `PATH`, under the system directories a cell shows (S-6), else `TOOL_MISSING`.
pub fn required(eff: &Effective, task: &Value, stage: &Value) -> Result<Vec<Check>, Error> {
    // A policy adopted after the stage may define one of its checks otherwise (P9F-2).
    crate::task::duplicates(&stage["checks"], eff)?;
    let mut names: BTreeSet<String> = texts(&task["checks"]).collect();
    let profile = format!("profiles.{}.checks", task["profiles"]["build"].as_str().unwrap_or_default());
    if let Some((Val::List(l), _)) = eff.values.get(&profile) {
        names.extend(l.iter().cloned());
    }
    names.extend(stage["checks"].as_object().into_iter().flatten().map(|(k, _)| k.clone()));
    let path = CELL_ENV[0].1;
    let one = |name: String| {
        let definition = stage["checks"].get(&name).cloned().or_else(|| crate::task::definition(eff, &name));
        let why = format!("checks.{name}: a required check that neither the policy nor the stage defines");
        let definition = definition.ok_or_else(|| refuse(Code::SettingsInvalid, None, why))?;
        let mut tools = serde_json::Map::new();
        for t in texts(&definition["tools"]) {
            let (at, version) = crate::detect::in_cell(path, &t).ok_or_else(|| {
                let why = format!("{t}: the check {name} names it, and it lives outside what a confined cell shows");
                refuse(Code::ToolMissing, None, why)
            })?;
            tools.insert(t, json!({"path": at.to_string_lossy(), "version": version}));
        }
        Ok(Check { name, definition, tools: tools.into() })
    };
    names.into_iter().map(one).collect()
}

/// REQ-GATE-3: a check result's identity, the digest of the candidate's product tree (`product`, its own digest), the
/// check's definition, the policy's digest, the tools it names as they resolve with their versions, and the platform.
pub fn identity(product: &str, check: &Check, policy: &str, platform: &str) -> String {
    let parts = json!({
        "product": product, "definition": check.definition, "policy": policy, "tools": check.tools,
        "platform": platform,
    });
    sha256_hex(&encode(&parts).expect("an identity's parts are integers and texts"))
}

/// The platform a check runs on: the operating system, the processor and the confinement backend; with
/// `fault-injection`, what `FABREWK_FAULT_PLATFORM` names, for tests.
pub fn platform(backend: &str) -> String {
    crate::record::fault::platform(format!("{}-{}-{backend}", std::env::consts::OS, std::env::consts::ARCH))
}

/// The digest of a task's requirements, which a review and a verification bind (REQ-GATE-5).
pub fn requirements(task: &Value) -> String {
    sha256_hex(&encode(&task["requirements"]).expect("requirements are texts"))
}

/// A check's cell (`confinement-design.md` 3.4): its fresh copy writable at `/work`, no network, no credential, no
/// namespace of its own, and the cell's own variables; its command run without a shell.
pub fn cell(scratch: &Scratch, check: &Check) -> Cell {
    Cell {
        home: scratch.home.clone(),
        tmp: scratch.tmp.clone(),
        mounts: vec![Mount { host: scratch.work.clone(), inside: "/work".into(), access: Access::ReadWrite }],
        network: Network::None,
        nested: false,
        env: CELL_ENV.iter().map(|(k, v)| (k.into(), v.into())).collect(),
        cwd: "/work".into(),
        argv: texts(&check.definition["command"]).map(Into::into).collect(),
    }
}

/// REQ-GATE-2: a check's outcome: `passed` only when it exited with status 0; `failed` for another status,
/// `timed-out` at its limit, `interrupted` by a signal or past the bytes kept.
pub fn outcome(ran: &Ran) -> &'static str {
    match ran.end {
        End::Exited(0) => "passed",
        End::Exited(_) => "failed",
        End::TimedOut => "timed-out",
        End::Signalled | End::OutputCapped => "interrupted",
    }
}

/// The name of a check's directories, under `local/tmp/` and `local/evidence/`: its task and the place its
/// `check-recorded` takes.
pub fn check_name(stream: &str, seq: u64) -> String {
    format!("check-{}-{seq}", &stream["task-".len()..])
}

/// REQ-GATE-5, REQ-GATE-3: whether `verified`, a `task-verified` payload, still holds: its candidate's tree and the
/// requirements' digest are `tree` and `requirements`, its `review`, a `review-recorded` payload, accepted both, and its
/// checks are exactly `current`, each with its identity recomputed now. Otherwise its evidence is stale.
pub fn holds(
    verified: &Value,
    review: &Value,
    tree: &str,
    requirements: &str,
    current: &BTreeMap<String, String>,
) -> bool {
    verified["tree"] == tree
        && verified["requirements"] == requirements
        && review["verdict"] == "accept"
        && review["tree"] == tree
        && review["requirements"] == requirements
        && verified["checks"] == json!(current)
}

/// The last verification among a task's `events` while it holds now (REQ-GATE-3, REQ-GATE-5): see [`holds`], with the
/// policy in force, `policy`, the one its review was recorded under (D-086 point 4), so that a policy adopted since
/// stales it though no check is required. Gives its payload.
pub fn verification<'e>(
    events: &'e [Event],
    tree: &str,
    requirements: &str,
    current: &BTreeMap<String, String>,
    policy: &str,
) -> Option<&'e Value> {
    let verified = &events.iter().rfind(|e| e.kind == "task-verified")?.value["payload"];
    let review = events.get(usize::try_from(verified["review"].as_u64()?).ok()?.checked_sub(1)?)?;
    let bound = review.kind == "review-recorded" && review.value["policy"] == policy;
    (bound && holds(verified, &review.value["payload"], tree, requirements, current)).then_some(verified)
}

/// The confirmed blocking findings among a review's.
pub fn blocking(findings: &Value) -> u64 {
    let all = findings.as_array().into_iter().flatten();
    all.filter(|f| f["severity"] == "blocking" && f["status"] == "confirmed").count() as u64
}

/// The rounds among `events` (a task's, since its authorization in force): the last round they name, and the
/// progress measure of each round that ended, in order (REQ-GATE-6): a review's confirmed blocking findings, or the
/// number of checks that did not pass, each one finding.
pub fn rounds(events: &[Event]) -> (u64, Vec<u64>) {
    let (mut last, mut by) = (0, BTreeMap::new());
    for e in events.iter().filter(|e| e.kind == "check-recorded" || e.kind == "review-recorded") {
        let p = &e.value["payload"];
        let round = p["round"].as_u64().unwrap_or_default();
        last = last.max(round);
        if e.kind == "review-recorded" {
            by.insert(round, blocking(&p["findings"]));
        } else if p["outcome"] != "passed" {
            *by.entry(round).or_insert(0) += 1;
        }
    }
    (last, by.into_values().collect())
}

/// `NO_PROGRESS` (REQ-TASK-5): two rounds in a row lowered no count: the last is not below the one before, nor that one
/// below the one before it.
pub fn no_progress(counts: &[u64]) -> bool {
    matches!(counts, [.., a, b, c] if c >= b && b >= a)
}

/// The most characters of each output channel a failed check carries to the next round.
const TAIL: usize = 4000;

/// A check that did not pass, as the next round's packet carries it (REQ-RUN-8): its name, its outcome and the last
/// [`TAIL`] characters of each channel its evidence kept.
pub fn output(evidence: &Path, check: &str, outcome: &str) -> Value {
    let tail = |channel: &str| {
        let text = String::from_utf8_lossy(&std::fs::read(evidence.join(channel)).unwrap_or_default()).into_owned();
        text.chars().skip(text.chars().count().saturating_sub(TAIL)).collect::<String>()
    };
    json!({"check": check, "outcome": outcome, "stdout": tail("stdout"), "stderr": tail("stderr")})
}

/// What the last round among a task's `events` leaves for the next (REQ-RUN-8, PD-33): the confirmed findings of a
/// review that rejected having examined, and each check that did not pass; the last round's events follow its
/// candidate, or a revalidation's run, whichever is later (that event itself carries nothing).
pub fn previous(events: &[Event], domain: &Path) -> (Vec<Value>, Vec<Value>) {
    let start = events.iter().rposition(|e| ["candidate-recorded", "run-started"].contains(&e.kind.as_str()));
    let (mut findings, mut failed) = (Vec::new(), Vec::new());
    for e in &events[start.unwrap_or(0)..] {
        let p = &e.value["payload"];
        match e.kind.as_str() {
            "check-recorded" if p["outcome"] != "passed" => {
                let evidence = domain.join("local/evidence").join(check_name(&e.stream, e.seq));
                failed.push(output(
                    &evidence,
                    p["check"].as_str().unwrap_or_default(),
                    p["outcome"].as_str().unwrap_or_default(),
                ));
            }
            "review-recorded" if p["verdict"] == "reject" && p["examined"].as_u64() > Some(0) => {
                findings = confirmed(&p["findings"]);
            }
            _ => {}
        }
    }
    (findings, failed)
}

/// The confirmed findings among a review's, blocking or minor.
pub fn confirmed(findings: &Value) -> Vec<Value> {
    findings.as_array().into_iter().flatten().filter(|f| f["status"] == "confirmed").cloned().collect()
}
