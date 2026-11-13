//! The gate as a library (part 9f; REQ-GATE-2, REQ-GATE-3, REQ-GATE-5, REQ-GATE-6, REQ-TASK-5's `NO_PROGRESS`,
//! REQ-RUN-8): a check result's identity and each of its inputs, whether a verification still holds, the rounds and
//! their progress measure, a check's outcome, and what a failed check carries to the next round.

use fabrewk_core::confine::{End, Kept, Ran};
use fabrewk_core::gate::{self, Check};
use fabrewk_core::record::{Event, sha256_hex};
use serde_json::{Value, json};
use std::collections::BTreeMap;

fn check() -> Check {
    Check {
        name: "test".into(),
        definition: json!({"command": ["sh", "-c", "true"], "network": false, "minutes": 30, "tools": ["python3"]}),
        tools: json!({"python3": {"path": "/usr/bin/python3.14", "version": "Python 3.14.0"}}),
    }
}

/// EVIDENCE_STALE's test, one input at a time (11.4): the product tree, the definition, the policy, a named tool's
/// version and the platform each change the identity; nothing else does.
#[test]
fn refusal_evidence_stale_each_input_of_an_identity_changes_it() {
    let (product, policy, platform) = ("a".repeat(64), "b".repeat(64), "linux-aarch64-bubblewrap");
    let base = gate::identity(&product, &check(), &policy, platform);
    assert_eq!(base, gate::identity(&product, &check(), &policy, platform));
    assert_eq!(base.len(), 64);
    let mut defined = check();
    defined.definition["minutes"] = json!(31);
    let mut versioned = check();
    versioned.tools["python3"]["version"] = json!("Python 3.14.1");
    let mut renamed = check();
    renamed.name = "other".into();
    for (what, other) in [
        ("tree", gate::identity(&"c".repeat(64), &check(), &policy, platform)),
        ("definition", gate::identity(&product, &defined, &policy, platform)),
        ("policy", gate::identity(&product, &check(), &"d".repeat(64), platform)),
        ("tool's version", gate::identity(&product, &versioned, &policy, platform)),
        ("platform", gate::identity(&product, &check(), &policy, "linux-x86_64-bubblewrap")),
    ] {
        assert_ne!(base, other, "{what}");
    }
    // The name is the check's key, not an input of its identity.
    assert_eq!(base, gate::identity(&product, &renamed, &policy, platform));
}

/// REQ-GATE-5: a verification holds only with every check's identity current, none more or less, and a review accept
/// bound to the same tree and requirements' digest.
#[test]
fn req_gate_5_an_accept_bound_to_another_tree_or_digest_does_not_count() {
    let (tree, requirements) = ("1".repeat(40), "2".repeat(64));
    let current: BTreeMap<String, String> = [("test".to_owned(), "3".repeat(64))].into();
    let verified = json!({"candidate": "4".repeat(40), "tree": tree, "requirements": requirements, "checks": {"test": "3".repeat(64)}, "review": 9});
    let review = json!({"verdict": "accept", "tree": tree, "requirements": requirements});
    assert!(gate::holds(&verified, &review, &tree, &requirements, &current));
    let with = |v: &Value, k: &str, x: Value| {
        let mut v = v.clone();
        v[k] = x;
        v
    };
    for (what, v, r, t, q) in [
        (
            "another review tree",
            verified.clone(),
            with(&review, "tree", json!("5".repeat(40))),
            tree.clone(),
            requirements.clone(),
        ),
        (
            "another review digest",
            verified.clone(),
            with(&review, "requirements", json!("6".repeat(64))),
            tree.clone(),
            requirements.clone(),
        ),
        ("a reject", verified.clone(), with(&review, "verdict", json!("reject")), tree.clone(), requirements.clone()),
        ("another candidate's tree", verified.clone(), review.clone(), "7".repeat(40), requirements.clone()),
        ("other requirements", verified.clone(), review.clone(), tree.clone(), "8".repeat(64)),
        (
            "a stale check",
            with(&verified, "checks", json!({"test": "9".repeat(64)})),
            review.clone(),
            tree.clone(),
            requirements.clone(),
        ),
        (
            "a check more",
            with(&verified, "checks", json!({"test": "3".repeat(64), "lint": "3".repeat(64)})),
            review.clone(),
            tree.clone(),
            requirements.clone(),
        ),
    ] {
        assert!(!gate::holds(&v, &r, &t, &q, &current), "{what}");
    }
    // A check removed from what is required now (PD-23) is stale too.
    assert!(!gate::holds(&verified, &review, &tree, &requirements, &BTreeMap::new()));
}

fn event(kind: &str, seq: u64, payload: Value) -> Event {
    Event {
        stream: "task-t-00000001".into(),
        seq,
        digest: "0".repeat(64),
        kind: kind.into(),
        value: json!({"payload": payload}),
        path: Default::default(),
    }
}

fn finding(severity: &str, status: &str) -> Value {
    json!({"severity": severity, "status": status, "path": null, "summary": "x"})
}

/// REQ-GATE-6: a round's progress measure is its review's confirmed blocking findings, or its checks that did not pass,
/// one each; NO_PROGRESS is two rounds in a row lowering nothing (REQ-TASK-5's criterion: 2, 2, 2 halts, 2, 3, 1 does
/// not).
#[test]
fn req_gate_6_rounds_and_their_progress_measure() {
    let events = [
        event("check-recorded", 1, json!({"round": 1, "outcome": "failed"})),
        event("check-recorded", 2, json!({"round": 1, "outcome": "timed-out"})),
        event("check-recorded", 3, json!({"round": 1, "outcome": "passed"})),
        event("check-recorded", 4, json!({"round": 2, "outcome": "passed"})),
        event(
            "review-recorded",
            5,
            json!({"round": 2, "findings": [finding("blocking", "confirmed"), finding("blocking", "suspected"), finding("minor", "confirmed")]}),
        ),
        event("check-recorded", 6, json!({"round": 3, "outcome": "passed"})),
    ];
    // Round 3's checks passed, and it ended with no review: it is spent, and has no measure yet.
    assert_eq!(gate::rounds(&events), (3, vec![2, 1]));
    assert_eq!(gate::rounds(&[]), (0, vec![]));
    for (counts, halts) in [
        (&[2, 2, 2][..], true),
        (&[2, 3, 1], false),
        (&[1, 2, 3], true),
        (&[3, 2, 2], false),
        (&[0, 0, 0], true),
        (&[5, 2, 2], false),
        (&[2, 2], false),
        (&[9, 2, 2, 2], true),
    ] {
        assert_eq!(gate::no_progress(counts), halts, "{counts:?}");
    }
    let findings =
        json!([finding("blocking", "confirmed"), finding("blocking", "suspected"), finding("minor", "confirmed")]);
    assert_eq!(gate::blocking(&findings), 1);
    assert_eq!(gate::confirmed(&findings).len(), 2);
}

fn ran(end: End) -> Ran {
    let kept = Kept { path: Default::default(), digest: String::new(), bytes: 0, capped: false };
    Ran { end, stdout: kept.clone(), stderr: kept, seconds: 1 }
}

/// REQ-GATE-2: only exit status 0 passes; another status, a timeout, a signal and output past its bytes do not.
#[test]
fn req_gate_2_only_a_check_that_exited_0_passed() {
    for (end, outcome) in [
        (End::Exited(0), "passed"),
        (End::Exited(1), "failed"),
        (End::Exited(137), "failed"),
        (End::TimedOut, "timed-out"),
        (End::Signalled, "interrupted"),
        (End::OutputCapped, "interrupted"),
    ] {
        assert_eq!(gate::outcome(&ran(end)), outcome, "{end:?}");
    }
}

/// REQ-RUN-8: a failed check carries the last 4,000 characters of each channel its evidence kept, whatever its bytes;
/// the previous round is the one after the last candidate or revalidation, and a review that examined nothing carries
/// no finding.
#[test]
fn req_run_8_what_a_round_leaves_for_the_next() {
    let base = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("gate-{}", std::process::id()));
    let evidence = base.join("local/evidence").join(gate::check_name("task-t-00000001", 7));
    std::fs::create_dir_all(&evidence).unwrap();
    std::fs::write(evidence.join("stdout"), format!("{}é\u{2028}", "a".repeat(5000))).unwrap();
    std::fs::write(evidence.join("stderr"), b"\xffend").unwrap();
    let out = gate::output(&evidence, "test", "failed");
    let stdout = out["stdout"].as_str().unwrap();
    assert_eq!((stdout.chars().count(), stdout.ends_with("é\u{2028}")), (4000, true));
    assert_eq!(out["stderr"], "\u{fffd}end");
    let reject = |examined: u64| json!({"verdict": "reject", "examined": examined, "findings": [finding("blocking", "confirmed"), finding("minor", "suspected")]});
    let events = [
        event("candidate-recorded", 5, json!({})),
        event("review-recorded", 6, reject(1)),
        event("candidate-recorded", 6, json!({})),
        event("check-recorded", 7, json!({"check": "test", "outcome": "failed"})),
        event("check-recorded", 8, json!({"check": "lint", "outcome": "passed"})),
    ];
    let (findings, failed) = gate::previous(&events, &base);
    assert_eq!((findings.len(), failed.len()), (0, 1));
    assert_eq!(failed[0]["check"], "test");
    assert_eq!(failed[0]["stdout"].as_str().unwrap().chars().count(), 4000);
    let (findings, failed) = gate::previous(&events[..2], &base);
    assert_eq!((findings.len(), failed.len()), (1, 0));
    let (findings, _) =
        gate::previous(&[event("run-started", 1, json!({})), event("review-recorded", 2, reject(0))], &base);
    assert!(findings.is_empty());
    assert_eq!(gate::requirements(&json!({"requirements": []})), sha256_hex(b"[]\n"));
    std::fs::remove_dir_all(base).unwrap();
}
