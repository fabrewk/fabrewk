//! Part 7d1 (D-053): the transition table and its labels (REQ-TASK-1, REQ-SEC-5), the replay held to it
//! (`RECORD_TRANSITION_INVALID`, PD-06's binding, PD-10's `ATTEMPT_ENDED`), the stage and task payloads (REQ-TASK-2,
//! REQ-TASK-3), the allowed-file patterns and their guards (REQ-TASK-7, PD-15, PD-17, REQ-GATE-9), `CHECK_DUPLICATE`
//! (PD-24), `LIMIT_UNOBSERVABLE` (REQ-RUN-7) and the credential heuristic (REQ-SEC-3), at the library's level.

mod common;

use common::*;
use fabrewk_core::detect::Program;
use fabrewk_core::record::{Code, Draft, Head, Replay, replay, sha256_hex, subject_digest};
use fabrewk_core::settings::{self, Effective, Level, Policy};
use fabrewk_core::table::{self, Control};
use fabrewk_core::{scan, task};
use proptest::prelude::*;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

fn spec() -> String {
    fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../.brewstillery/bootstrap/core-specification.md"))
        .unwrap()
}

/// The text between backticks, in order.
fn ticks(s: &str) -> Vec<&str> {
    s.split('`').skip(1).step_by(2).collect()
}

/// A guard cell's guards, in order: every code in backticks, and every requirement cited outside parentheses.
fn guards(cell: &str) -> Vec<String> {
    let (mut out, mut depth) = (Vec::new(), 0);
    for (i, c) in cell.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            '`' if cell[..i].matches('`').count().is_multiple_of(2) => {
                let code = cell[i + 1..].split('`').next().unwrap();
                if code.bytes().all(|b| b.is_ascii_uppercase() || b == b'_') {
                    out.push(code.to_owned());
                }
            }
            'R' if depth == 0 && cell[i..].starts_with("REQ-") => {
                let id: String = cell[i..].chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '-').collect();
                out.push(id.trim_end_matches('-').to_owned());
            }
            _ => {}
        }
    }
    out
}

// ---- REQ-TASK-1 and REQ-SEC-5 --------------------------------------------------------------------------------------

#[test]
fn req_task_1_the_table_is_section_4() {
    let spec = spec();
    let section = &spec[spec.find("## 4. States and transitions").unwrap()..spec.find("## 5. Commands").unwrap()];
    let rows: Vec<&str> = section.lines().filter(|l| l.starts_with("| `")).collect();
    let tasks: Vec<_> = table::moves().iter().filter(|m| m.subject == "task").collect();
    assert_eq!(rows.len(), tasks.len());
    for (row, m) in rows.iter().zip(&tasks) {
        let c: Vec<&str> = row.split('|').map(str::trim).collect();
        let kinds: Vec<&str> = m.kinds.iter().chain(&m.then).chain(&m.covered).copied().collect();
        assert_eq!(ticks(c[1]), kinds, "{row}");
        let mut from: Vec<&str> = ticks(c[2]);
        if c[2].contains("any waiting state") {
            from.extend(["authorized", "halted", "verified"]);
        }
        let mut ours = m.from.clone();
        for v in [&mut from, &mut ours] {
            v.sort();
            v.dedup();
        }
        assert_eq!(from, ours, "{row}");
        assert_eq!(ticks(c[3]), [m.to], "{row}");
        assert!(c[4].starts_with(m.who), "{row}");
        assert_eq!(c[4].contains("--say"), m.say, "{row}");
        let names: Vec<&str> = m.guards.iter().map(|g| g.name).collect();
        assert_eq!(guards(c[5]), names, "{row}");
    }
    // Stages and proposals, from their paragraphs: each state and the move that reaches it.
    let prose = section.replace('\n', " ");
    for m in table::moves().iter().filter(|m| m.subject != "task") {
        assert!(prose.contains(&format!("`{}` (`{}`", m.to, m.kinds[0])), "{:?}", m.kinds);
    }
    let close = table::find("stage", "stage-closed").unwrap();
    let req = &spec[spec.find("**REQ-TASK-6.**").unwrap() + "**REQ-TASK-6.**".len()..];
    let req = &req[..req.find("\n- **").unwrap()];
    assert_eq!(close.guards.iter().map(|g| g.name).collect::<Vec<_>>(), guards(req));
    // Every subject's states, as section 4 names them.
    let states = |s: &str| table::states(s).into_iter().collect::<Vec<_>>();
    assert_eq!(
        states("task"),
        ["accepted", "authorized", "halted", "integrated", "proposed", "rejected", "running", "verified"]
    );
    assert_eq!(states("stage"), ["authorized", "closed", "proposed"]);
    assert_eq!(states("proposal"), ["discarded", "incorporated", "pending", "withdrawn"]);
}

#[test]
fn req_sec_5_every_guard_is_a_code_or_a_requirement_and_has_a_label() {
    let spec = spec();
    let codes = &spec[spec.find("All codes, in one list:").unwrap()..];
    let codes = &codes[..codes.find("\n\n").unwrap()];
    for m in table::moves() {
        for g in &m.guards {
            let known = codes.contains(&format!("`{}`", g.name)) || spec.contains(&format!("**{}.**", g.name));
            assert!(known, "{}: {} is neither a code nor a requirement", m.kinds[0], g.name);
        }
    }
    // What a process of the same user could bypass is cooperative; the rest is enforced.
    let cooperative: Vec<(&str, &str)> = table::moves()
        .iter()
        .flat_map(|m| m.guards.iter().filter(|g| g.control == Control::Cooperative).map(|g| (m.kinds[0], g.name)))
        .collect();
    assert_eq!(
        cooperative,
        [
            ("task-proposed", "STAGE_CLOSED"),
            ("task-proposed", "TASK_INVALID"),
            ("task-proposed", "PROTECTED_PATH"),
            ("task-proposed", "PROTECTED_PATH_WILDCARD"),
            ("task-authorized", "WORDS_COPIED"),
            ("run-started", "LOCK_HELD"),
            ("run-started", "REQ-RUN-2"),
            ("attempt-ended", "CREDENTIAL_SUSPECTED"),
            ("stage-proposed", "TASK_INVALID"),
            ("stage-proposed", "CHECK_DUPLICATE"),
            ("stage-authorized", "WORDS_COPIED"),
        ]
    );
}

#[test]
fn req_task_1_guards_are_evaluated_in_the_order_listed() {
    for m in table::moves().iter().filter(|m| m.guards.len() > 1) {
        for (i, first) in m.guards.iter().enumerate() {
            for second in &m.guards[i + 1..] {
                let mut seen = Vec::new();
                let got = table::evaluate(m, |g| {
                    seen.push(g.name);
                    if g.name == first.name || g.name == second.name { Err(g.name) } else { Ok(()) }
                });
                assert_eq!(got, Err(first.name), "{:?}", m.kinds);
                assert_eq!(seen.last(), Some(&first.name));
            }
        }
    }
    assert_eq!(table::evaluate(table::find("task", "task-rejected").unwrap(), |_| Ok::<(), ()>(())), Ok(()));
}

#[test]
fn refusal_move_not_allowed() {
    let refused = |subject, state, kind| {
        let e = table::step(subject, state, kind).unwrap_err();
        assert_eq!(code(&e), Some(Code::MoveNotAllowed));
        e.detail()
    };
    let why = refused("task", Some("proposed"), "run-started");
    assert_eq!(why, "run-started from proposed; allowed: task-authorized, task-rejected");
    let why = refused("task", Some("integrated"), "task-authorized");
    assert_eq!(why, "task-authorized from integrated; allowed: ");
    refused("task", None, "task-authorized");
    refused("task", Some("proposed"), "task-proposed");
    refused("task", Some("authorized"), "attempt-started");
    refused("stage", Some("closed"), "stage-closed");
    refused("proposal", Some("discarded"), "proposal-incorporated");
    refused("task", Some("proposed"), "stage-proposed");
    // Allowed: the creation, a revalidation's authorization, the attempt after a run's start, a failed merge.
    assert_eq!(table::step("task", None, "task-proposed").unwrap().to, "proposed");
    assert_eq!(table::step("task", Some("accepted"), "task-authorized").unwrap().to, "authorized");
    assert_eq!(table::step("task", Some("running"), "attempt-started").unwrap().to, "running");
    assert_eq!(table::step("task", Some("accepted"), "merge-failed").unwrap().to, "accepted");
    assert_eq!(table::allowed("task", Some("halted")), ["task-authorized", "task-accepted", "task-rejected"]);
}

// ---- The replay held to the table ----------------------------------------------------------------------------------

/// A fixture with a stage and a task forged on disk, and helpers to forge the task's next moves.
struct Task {
    f: Fixture,
    stream: String,
}

impl Task {
    fn new(name: &str) -> Task {
        let f = Fixture::new(name, false);
        let (stream, _) = forge_stage_and_task(&f);
        Task { f, stream }
    }

    fn head(&self, stream: &str) -> (u64, String) {
        let files = self.f.events(stream);
        (files.len() as u64, sha256_hex(&fs::read(files.last().unwrap()).unwrap()))
    }

    /// Forges `draft` as the next event of `stream`, signed when a person's.
    fn next_in(&self, stream: &str, draft: &Draft) -> std::path::PathBuf {
        let (seq, prev) = self.head(stream);
        let signer = draft.say.is_some().then(|| self.f.signer());
        self.f.forge(stream, seq + 1, Some(&prev), draft, signer.as_ref())
    }

    fn next(&self, draft: &Draft) -> std::path::PathBuf {
        self.next_in(&self.stream, draft)
    }

    fn authorize(&self) -> std::path::PathBuf {
        self.next(&person("task-authorized", "Autorizo.", bound(&self.f, &self.stream)))
    }

    fn replay(&self) -> Result<Replay, Vec<Code>> {
        replay(&self.f.domain, &self.f.verifier()).map_err(|e| e.iter().map(|e| e.code().unwrap()).collect())
    }
}

#[test]
fn refusal_record_transition_invalid_from_the_table() {
    let t = Task::new("table-moves");
    t.authorize();
    t.next(&person("run-started", "Corre.", json!({})));
    for kind in ["attempt-started", "attempt-ended", "candidate-recorded", "check-recorded", "review-recorded"] {
        t.next(&fabrewk(kind, json!({})));
    }
    let verified = t.next(&fabrewk("task-verified", json!({})));
    t.replay().unwrap();
    // A hand-made `task-integrated` from `verified` (11.4).
    let integrated = t.next(&fabrewk("task-integrated", json!({})));
    assert_eq!(t.replay().unwrap_err(), [Code::RecordTransitionInvalid]);
    fs::remove_file(integrated).unwrap();
    // From `verified` a run starts again (a revalidation); from `proposed`, with no authorization, it does not.
    t.next(&person("run-started", "Revalida.", json!({})));
    t.replay().unwrap();
    let other = Task::new("table-unauthorized");
    other.next(&person("run-started", "Corre.", json!({})));
    assert_eq!(other.replay().unwrap_err(), [Code::RecordTransitionInvalid]);
    let _ = verified;
}

#[test]
fn pd_10_a_second_end_of_an_attempt_on_replay() {
    let t = Task::new("pd10");
    t.authorize();
    t.next(&person("run-started", "Corre.", json!({})));
    t.next(&fabrewk("attempt-started", json!({})));
    t.next(&fabrewk("attempt-ended", json!({})));
    t.replay().unwrap();
    let second = t.next(&fabrewk("attempt-ended", json!({})));
    assert_eq!(t.replay().unwrap_err(), [Code::AttemptEnded]);
    fs::remove_file(second).unwrap();
    // A new attempt starts only once the last one ended, and the task leaves `running` with none open (REQ-RUN-1).
    t.next(&fabrewk("attempt-started", json!({})));
    let replacing = t.next(&fabrewk("attempt-started", json!({})));
    assert_eq!(t.replay().unwrap_err(), [Code::RecordTransitionInvalid]);
    fs::remove_file(replacing).unwrap();
    let halted = t.next(&fabrewk("task-halted", json!({})));
    assert_eq!(t.replay().unwrap_err(), [Code::RecordTransitionInvalid]);
    fs::remove_file(halted).unwrap();
    t.next(&fabrewk("attempt-ended", json!({})));
    t.next(&fabrewk("task-halted", json!({})));
    t.replay().unwrap();
}

#[test]
fn pd_06_an_approval_bound_to_another_task_or_head_is_refused() {
    // Two tasks with identical fields; only their nonces differ.
    let t = Task::new("pd06-binding");
    let first: Value = serde_json::from_slice(&fs::read(&t.f.events(&t.stream)[0]).unwrap()).unwrap();
    let mut fields = first["payload"].clone();
    fields.as_object_mut().unwrap().remove("nonce");
    let (twin, draft) = creation("task-proposed", fields);
    t.f.forge(&twin, 1, None, &draft, None);
    let approval = person("task-authorized", "Autorizo.", bound(&t.f, &t.stream));
    // Signed again in the twin's stream, at its place: its binding names the other task.
    let copy = t.next_in(&twin, &approval);
    assert_eq!(t.replay().unwrap_err(), [Code::RecordTransitionInvalid]);
    fs::remove_file(sig(&copy)).unwrap();
    fs::remove_file(copy).unwrap();
    // In its own stream it binds; re-appended later, after a halt, it names an old head.
    t.next(&approval);
    t.next(&fabrewk("task-halted", json!({"code": "NO_PROGRESS"})));
    t.replay().unwrap();
    let again = t.next(&approval);
    assert_eq!(t.replay().unwrap_err(), [Code::RecordTransitionInvalid]);
    fs::remove_file(sig(&again)).unwrap();
    fs::remove_file(again).unwrap();
    // A digest that is not the task's.
    let mut wrong = bound(&t.f, &t.stream);
    wrong["digest"] = json!(subject_digest(&json!({"other": 1})).unwrap());
    let path = t.next(&person("task-authorized", "Autorizo.", wrong));
    assert_eq!(t.replay().unwrap_err(), [Code::RecordTransitionInvalid]);
    fs::remove_file(sig(&path)).unwrap();
    fs::remove_file(path).unwrap();
    // The same for `task-accepted`: bound to the twin, refused; bound to its own head, it moves the halted task.
    let mut accepted = bound(&t.f, &twin);
    accepted.as_object_mut().unwrap().retain(|k, _| k == "task" || k == "head");
    let path = t.next(&person("task-accepted", "Acepto.", accepted));
    assert_eq!(t.replay().unwrap_err(), [Code::RecordTransitionInvalid]);
    fs::remove_file(sig(&path)).unwrap();
    fs::remove_file(path).unwrap();
    let mut accepted = bound(&t.f, &t.stream);
    accepted.as_object_mut().unwrap().retain(|k, _| k == "task" || k == "head");
    t.next(&person("task-accepted", "Acepto.", accepted));
    t.replay().unwrap();
}

#[test]
fn req_task_3_a_stage_authorization_authorizes_the_tasks_it_binds() {
    let t = Task::new("stage-binds");
    let first: Value = serde_json::from_slice(&fs::read(&t.f.events(&t.stream)[0]).unwrap()).unwrap();
    let stage = format!("stage-{}", first["payload"]["stage"].as_str().unwrap());
    let (head_seq, head) = t.head(&t.stream);
    let stage_binding = |tasks: Value| {
        let (seq, digest) = t.head(&stage);
        let created: Value = serde_json::from_slice(&fs::read(&t.f.events(&stage)[0]).unwrap()).unwrap();
        json!({
            "stage": &stage["stage-".len()..], "digest": subject_digest(&created["payload"]).unwrap(),
            "head": {"seq": seq, "digest": digest}, "tasks": tasks, "widened": [],
        })
    };
    let entry = |seq: u64, digest: &str| {
        json!({"task": &t.stream["task-".len()..], "digest": subject_digest(&first["payload"]).unwrap(),
               "head": {"seq": seq, "digest": digest}, "limits": first["payload"]["limits"]})
    };
    // A head the task's stream does not hold is a rollback; a task of another stage is not this stage's.
    let path = t.next_in(&stage, &person("stage-authorized", "Autorizo.", stage_binding(json!([entry(2, &head)]))));
    assert_eq!(t.replay().unwrap_err(), [Code::RecordRollback]);
    fs::remove_file(sig(&path)).unwrap();
    fs::remove_file(path).unwrap();
    let path =
        t.next_in(&stage, &person("stage-authorized", "Autorizo.", stage_binding(json!([entry(1, &"0".repeat(64))]))));
    assert_eq!(t.replay().unwrap_err(), [Code::RecordRollback]);
    fs::remove_file(sig(&path)).unwrap();
    fs::remove_file(path).unwrap();
    let mut other_digest = entry(1, &head);
    other_digest["digest"] = json!("0".repeat(64));
    let path = t.next_in(&stage, &person("stage-authorized", "Autorizo.", stage_binding(json!([other_digest]))));
    assert_eq!(t.replay().unwrap_err(), [Code::RecordTransitionInvalid]);
    fs::remove_file(sig(&path)).unwrap();
    fs::remove_file(path).unwrap();
    let (elsewhere, _) = forge_stage_and_task(&t.f);
    let mut foreign = entry(1, &head);
    foreign["task"] = json!(&elsewhere["task-".len()..]);
    let path = t.next_in(&stage, &person("stage-authorized", "Autorizo.", stage_binding(json!([foreign]))));
    assert_eq!(t.replay().unwrap_err(), [Code::RecordTransitionInvalid]);
    fs::remove_file(sig(&path)).unwrap();
    fs::remove_file(path).unwrap();
    // Bound to the task's head: the task is authorized, so a run starts and a second authorization is refused.
    t.next_in(&stage, &person("stage-authorized", "Autorizo.", stage_binding(json!([entry(head_seq, &head)]))));
    t.replay().unwrap();
    let path = t.authorize();
    assert_eq!(t.replay().unwrap_err(), [Code::RecordTransitionInvalid]);
    fs::remove_file(sig(&path)).unwrap();
    fs::remove_file(path).unwrap();
    t.next(&person("run-started", "Corre.", json!({})));
    t.replay().unwrap();
    // A second `stage-authorized` is no move of an authorized stage.
    t.next_in(&stage, &person("stage-authorized", "Autorizo.", stage_binding(json!([]))));
    assert_eq!(t.replay().unwrap_err(), [Code::RecordTransitionInvalid]);
}

#[test]
fn req_task_3_a_task_names_a_stage_and_dependencies_the_record_holds() {
    let f = Fixture::new("task-names", false);
    let (stage, draft) = creation("stage-proposed", stage_fields());
    let mut fields = task_fields(&stage);
    let (task, d) = creation("task-proposed", fields.clone());
    f.forge(&task, 1, None, &d, None);
    let refused =
        || replay(&f.domain, &f.verifier()).map_err(|e| e.iter().map(|e| e.code().unwrap()).collect::<Vec<_>>());
    assert_eq!(refused().unwrap_err(), [Code::RecordTransitionInvalid]);
    f.forge(&stage, 1, None, &draft, None);
    refused().unwrap();
    // A dependent task has no base; its dependencies must be tasks of the record.
    fields["base"] = Value::Null;
    fields["dependencies"] = json!([&task["task-".len()..]]);
    let (dependent, d) = creation("task-proposed", fields.clone());
    f.forge(&dependent, 1, None, &d, None);
    refused().unwrap();
    fields["dependencies"] = json!(["t-00000000"]);
    let (orphan, d) = creation("task-proposed", fields);
    f.forge(&orphan, 1, None, &d, None);
    assert_eq!(refused().unwrap_err(), [Code::RecordTransitionInvalid]);
}

// ---- REQ-TASK-2: the payloads --------------------------------------------------------------------------------------

#[test]
fn refusal_task_invalid() {
    let fields = |edit: &dyn Fn(&mut Value)| {
        let mut v = task_fields("stage-s-00000000");
        v["nonce"] = json!("0".repeat(32));
        edit(&mut v);
        task::check("task-proposed", &v).map_err(|e| e.code().unwrap())
    };
    assert_eq!(fields(&|_| {}), Ok(()));
    // Every field is required: a missing one fails the payload's shape (schema 1).
    for field in task_fields("stage-s-00000000").as_object().unwrap().keys() {
        assert_eq!(
            fields(&|v| _ = v.as_object_mut().unwrap().remove(field)),
            Err(Code::RecordVersionUnknown),
            "{field}"
        );
    }
    assert_eq!(fields(&|v| v["requirements"] = json!([{"text": "x"}])), Err(Code::RecordVersionUnknown));
    assert_eq!(fields(&|v| v["limits"]["rounds"] = json!("3")), Err(Code::RecordVersionUnknown));
    assert_eq!(fields(&|v| v["extra"] = json!(1)), Err(Code::RecordVersionUnknown));
    // A shape that fails anywhere wins over content that fails before it.
    assert_eq!(
        fields(&|v| {
            v["goal"] = json!("");
            v["settings"] = json!([])
        }),
        Err(Code::RecordVersionUnknown)
    );
    // Content (11.4: no requirement, a requirement with no criterion, a pattern with `..`).
    let invalid = Err(Code::TaskInvalid);
    let put = |edits: &[(&str, Value)]| {
        fields(&|v| {
            for (at, x) in edits {
                let (parent, leaf) = at.rsplit_once('/').unwrap();
                v.pointer_mut(parent).unwrap()[leaf] = x.clone();
            }
        })
    };
    for edits in [
        vec![("/requirements", json!([]))],
        vec![("/requirements", json!([{"text": "x", "criterion": " "}]))],
        vec![("/allowed", json!(["src/../.git/config"]))],
        vec![("/allowed", json!([]))],
        vec![("/goal", json!("\t\n"))],
        vec![("/lang", json!("en_US"))],
        vec![("/delegationLanguage", json!(""))],
        vec![("/limits/minutes", json!(0))],
        vec![("/limits/tokens", json!(-1))],
        vec![("/stage", json!("t-00000000"))],
        vec![("/base", json!("abc"))],
        vec![("/base", Value::Null)],
        vec![("/dependencies", json!(["t-00000000"]))],
        vec![("/base", Value::Null), ("/dependencies", json!(["t-00000000", "t-00000000"]))],
        vec![("/profiles/build", json!(""))],
        vec![("/settings", json!({"nosuch.key": 1}))],
        vec![("/settings", json!({"maintainers.xel.role": "maestro"}))],
        vec![("/settings", json!({"limits.rounds": "three"}))],
    ] {
        assert_eq!(put(&edits), invalid, "{edits:?}");
    }
    // Kept: a dependent task with no base; a task layer that narrows a profile's checks; a 64-digit base.
    assert_eq!(
        fields(&|v| {
            v["base"] = Value::Null;
            v["dependencies"] = json!(["t-0000000a"])
        }),
        Ok(())
    );
    assert_eq!(fields(&|v| v["settings"] = json!({"limits.rounds": 2, "profiles.build.checks": ["test"]})), Ok(()));
    assert_eq!(fields(&|v| v["base"] = json!("a".repeat(64))), Ok(()));
    // A stage: its fields, its checks' definitions, and a layer only a stage may set.
    let stage = |edit: &dyn Fn(&mut Value)| {
        let mut v = stage_fields();
        v["nonce"] = json!("0".repeat(32));
        edit(&mut v);
        task::check("stage-proposed", &v).map_err(|e| e.code().unwrap())
    };
    let check = json!({"command": ["cargo", "test"], "network": false, "minutes": 30, "tools": []});
    assert_eq!(stage(&|v| v["checks"] = json!({"test": check})), Ok(()));
    assert_eq!(stage(&|v| v["checks"] = json!({"test": {"command": ["cargo"]}})), Err(Code::RecordVersionUnknown));
    assert_eq!(
        stage(&|v| v["checks"] = json!({"test": {"command": [], "network": false, "minutes": 30, "tools": []}})),
        invalid
    );
    assert_eq!(stage(&|v| v["budget"]["dispatches"] = json!(0)), invalid);
    // A layer's value: of no settings type is the shape's failure; of another type than its key's, the content's.
    assert_eq!(put(&[("/settings", json!({"limits.rounds": {"x": 1}}))]), Err(Code::RecordVersionUnknown));
    assert_eq!(put(&[("/settings", json!({"limits.rounds": true}))]), invalid);
    // The authorizations' payloads are held to their shape too.
    for kind in ["task-authorized", "stage-authorized"] {
        assert_eq!(task::check(kind, &json!({})).map_err(|e| e.code().unwrap()), Err(Code::RecordVersionUnknown));
    }
    // The shape is schema 1's: a record holding a malformed payload does not replay.
    let f = Fixture::new("task-shape", false);
    let (stream, d) = creation("task-proposed", json!({"goal": "x"}));
    f.forge(&stream, 1, None, &d, None);
    assert_eq!(codes(replay(&f.domain, &f.verifier())), [Code::RecordVersionUnknown]);
    // Content the commands refuse `TASK_INVALID` is a move the table does not allow, on replay.
    std::fs::remove_dir_all(f.stream_dir(&stream)).unwrap();
    let (stage, d) = creation("stage-proposed", stage_fields());
    f.forge(&stage, 1, None, &d, None);
    let mut blank = task_fields(&stage);
    blank["goal"] = json!(" ");
    let (stream, d) = creation("task-proposed", blank);
    f.forge(&stream, 1, None, &d, None);
    assert_eq!(codes(replay(&f.domain, &f.verifier())), [Code::RecordTransitionInvalid]);
}

#[test]
fn req_task_2_an_authorization_binds_the_digest_of_the_fields() {
    let mut v = task_fields("stage-s-00000000");
    v["nonce"] = json!("0".repeat(32));
    let digest = subject_digest(&v).unwrap();
    let stream = fabrewk_core::record::subject_stream("task-proposed", &v).unwrap();
    assert_eq!(&stream["task-t-".len()..], &digest[..8]);
    // Each field changed in turn changes the digest, so a binding to the old one names another task.
    for field in v.as_object().unwrap().keys().cloned().collect::<Vec<_>>() {
        let mut w = v.clone();
        w[&field] = json!(["changed"]);
        assert_ne!(subject_digest(&w).unwrap(), digest, "{field}");
    }
    let _ = Head { seq: 1, digest };
}

// ---- REQ-TASK-7, PD-15, PD-17 --------------------------------------------------------------------------------------

#[test]
fn pd_15_the_lane_guard() {
    for p in [
        "**/*.png",
        "*/xel/**",
        "?brewstillery/settings.toml",
        "**/attachments/**",
        "*",
        "**",
        "**/*.py",
        "*/config/*",
        "*.git",
        ".G*",
        "./.brewstillery/x",
        "tests/fixtures/.brewstillery/**",
    ] {
        let e = task::lane(&[p]).unwrap_err();
        assert_eq!(code(&e), Some(Code::ProtectedPath), "{p}");
    }
    for p in ["src/../.git/config", &"a".repeat(257)] {
        assert_eq!(code(&task::lane(&[p]).unwrap_err()), Some(Code::TaskInvalid), "{p}");
    }
    for p in [
        "*.md",
        "src/**",
        ".github/**",
        ".gitignore",
        ".git*ignore",
        "tests/fixtures/brewstillery/**",
        ".brewstillery-old/x",
        "brewstillery-old/x",
    ] {
        task::lane(&[p]).unwrap_or_else(|e| panic!("{p}: {e:?}"));
    }
}

#[test]
fn req_task_7_pattern_syntax() {
    let invalid = |p: &str| code(&task::pattern(p).unwrap_err()) == Some(Code::TaskInvalid);
    assert!(invalid(&vec!["a"; 33].join("/")));
    assert!(task::pattern(&vec!["a"; 32].join("/")).is_ok());
    assert!(invalid("/src/x"));
    assert!(invalid("src/../x"));
    assert!(invalid(""));
    assert!(invalid("./"));
    assert!(invalid(&"a".repeat(257)));
    assert!(task::pattern(&"a".repeat(256)).is_ok());
    // Empty and `.` segments are dropped before matching.
    assert_eq!(task::pattern("./src//./x/").unwrap(), ["src", "x"]);
    assert!(task::matches(&task::pattern("./src//x").unwrap(), "src/x", &mut 0));
}

/// The definition the matcher implements, by recursion (PD-17's reference).
fn reference(p: &[&str], path: &[&str]) -> bool {
    fn seg(p: &[char], n: &[char]) -> bool {
        match p.split_first() {
            None => n.is_empty(),
            Some(('*', r)) => (0..=n.len()).any(|k| seg(r, &n[k..])),
            Some(('?', r)) => !n.is_empty() && seg(r, &n[1..]),
            Some((c, r)) => n.first() == Some(c) && seg(r, &n[1..]),
        }
    }
    match p.split_first() {
        None => path.is_empty(),
        Some((&"**", rest)) => (0..=path.len()).any(|k| reference(rest, &path[k..])),
        Some((s, rest)) => {
            let (s, n): (Vec<char>, Vec<char>) =
                (s.chars().collect(), path.first().map_or(vec![], |n| n.chars().collect()));
            !path.is_empty() && seg(&s, &n) && reference(rest, &path[1..])
        }
    }
}

#[test]
fn pd_17_the_matcher_stays_within_its_bound() {
    // The steps are counted: a segment's cells, and one per path position for a bare `**`.
    let mut steps = 0;
    assert!(task::matches(&["a"], "a", &mut steps));
    assert_eq!(steps, 4);
    let mut steps = 0;
    assert!(task::matches(&["**"], "a/b", &mut steps));
    assert_eq!(steps, 3);
    let bound = |p: &str, path: &str| (p.len() + 1) * (path.len() + 1);
    let p = format!("src/{}b", "**a".repeat(8));
    let path = format!("src/{}", "a".repeat(56));
    let mut steps = 0;
    assert!(!task::matches(&task::pattern(&p).unwrap(), &path, &mut steps));
    assert!(steps <= bound(&p, &path), "{steps}");
    let segs = vec!["**"; 150];
    let p = segs.join("/");
    let path = "a/".repeat(1999) + "ab";
    let mut steps = 0;
    assert!(task::matches(&segs, &path, &mut steps));
    assert!(steps <= bound(&p, &path), "{steps}");
}

const TOKENS: [&str; 9] = ["a", "G", ".", "/", "*", "?", "**", ".git", ".brewstillery"];

fn joined(tokens: &[usize]) -> String {
    tokens.iter().map(|&i| TOKENS[i]).collect()
}

/// One path segment that the pattern segment `s` matches: its literal characters, `a` for `?`, nothing for `*`.
fn witness(s: &str) -> String {
    s.chars().filter(|c| *c != '*').map(|c| if c == '?' { 'a' } else { c }).collect()
}

/// REQ-TASK-7's refusal, decided by the matcher: some segment other than a bare `**` matches a protected name, or the
/// pattern matches a path whose first component is one, followed by witnesses of the pattern's remaining segments
/// (under ASCII case folding: the pattern folded, the names lower case).
fn protected_by_reference(p: &str) -> bool {
    let folded = p.to_ascii_lowercase();
    let segs = task::pattern(&folded).unwrap();
    let names = [".git", ".brewstillery"];
    let by_segment = segs.iter().any(|s| *s != "**" && names.iter().any(|n| task::matches(&[*s], n, &mut 0)));
    let first = names.iter().any(|n| {
        (0..=segs.len()).any(|j| {
            let tail: Vec<String> = segs[j..].iter().filter(|s| **s != "**").map(|s| witness(s)).collect();
            let path = std::iter::once(n.to_string()).chain(tail).collect::<Vec<_>>().join("/");
            task::matches(&segs, &path, &mut 0)
        })
    });
    by_segment || first
}

proptest! {
    #[test]
    fn pd_17_the_matcher_is_the_recursive_definition(
        p in proptest::collection::vec(proptest::sample::select(&["a", "b", "\n", "é", "*", "?", "**"][..]), 0..6),
        path in proptest::collection::vec(proptest::sample::select(&["a", "b", "\n", "é", "ab"][..]), 0..5),
        cut in proptest::collection::vec(any::<bool>(), 0..6),
    ) {
        // Patterns of segments from the tokens; paths of segments, newlines and non-ASCII included.
        let mut segs: Vec<String> = Vec::new();
        for (i, t) in p.iter().enumerate() {
            match segs.last_mut() {
                Some(last) if !cut.get(i).copied().unwrap_or(true) && *t != "**" && last != "**" => last.push_str(t),
                _ => segs.push(t.to_string()),
            }
        }
        let segs: Vec<&str> = segs.iter().map(String::as_str).collect();
        let path = path.join("/");
        let parts: Vec<&str> = path.split('/').collect();
        let mut steps = 0;
        prop_assert_eq!(task::matches(&segs, &path, &mut steps), reference(&segs, &parts));
    }

    #[test]
    fn pd_15_the_guard_is_its_definition(tokens in proptest::collection::vec(0..TOKENS.len(), 1..=6)) {
        let p = joined(&tokens);
        if task::pattern(&p).is_ok() {
            let refused = task::lane(&[&p]).is_err_and(|e| e.code() == Some(Code::ProtectedPath));
            prop_assert_eq!(refused, protected_by_reference(&p), "{}", p);
        }
    }

    #[test]
    fn pd_15_a_kept_pattern_never_reaches_a_protected_first_component(tokens in proptest::collection::vec(0..TOKENS.len(), 1..40)) {
        let p = joined(&tokens);
        if task::lane(&[&p]).is_ok() {
            let segs = task::pattern(&p).unwrap();
            for n in [".git", ".GIT", ".Git", ".brewstillery", ".BrewStillery"] {
                for j in 0..=segs.len() {
                    let tail: Vec<String> = segs[j..].iter().filter(|s| **s != "**").map(|s| witness(s)).collect();
                    let path = std::iter::once(n.to_string()).chain(tail).collect::<Vec<_>>().join("/");
                    prop_assert!(!task::matches(&segs, &path, &mut 0), "{} matches {}", p, path);
                }
            }
        }
    }
}

// ---- REQ-GATE-9, PD-24, REQ-RUN-7 ----------------------------------------------------------------------------------

fn effective(text: &str) -> Effective {
    let dir = tmp("effective");
    let settings = settings::parse(&dir.join("settings.toml"), text, Level::Project).unwrap();
    let policy = Policy { digest: sha256_hex(text.as_bytes()), text: text.into(), settings };
    settings::effective(&policy, None, None, &[], &[]).unwrap()
}

#[test]
fn refusal_protected_path_wildcard() {
    let protected: Vec<String> = ["Cargo.toml", "Cargo.lock", ".github/"].map(String::from).to_vec();
    let refused = |p: &str| {
        let e = task::wildcards(&[p], &protected).unwrap_err();
        assert_eq!(code(&e), Some(Code::ProtectedPathWildcard), "{p}");
    };
    // `.githu?` matches the protected directory's own entry (a link, say), not only what is under it (P7D1-2).
    for p in [
        "*.toml",
        "Cargo.*",
        "Cargo.tom?",
        "**",
        "**/Cargo.toml",
        ".github/**",
        ".git*/workflows/ci.yml",
        "*/x",
        ".githu?",
        "*",
    ] {
        refused(p);
    }
    for p in ["Cargo.toml", ".github/workflows/ci.yml", "src/**", "*.md", "crates/*/Cargo.toml", ".githubx/*"] {
        task::wildcards(&[p], &protected).unwrap_or_else(|e| panic!("{p}: {e:?}"));
    }
}

#[test]
fn pd_24_refusal_check_duplicate() {
    let eff = effective("[project]\nname = \"x\"\n\n[checks.test]\ncommand = [\"cargo\", \"test\"]\n");
    let same = json!({"test": {"command": ["cargo", "test"], "network": false, "minutes": 30, "tools": []}});
    task::duplicates(&same, &eff).unwrap();
    task::duplicates(
        &json!({"lint": {"command": ["cargo", "clippy"], "network": false, "minutes": 30, "tools": []}}),
        &eff,
    )
    .unwrap();
    for differs in [
        json!({"test": {"command": ["cargo", "test", "--release"], "network": false, "minutes": 30, "tools": []}}),
        json!({"test": {"command": ["cargo", "test"], "network": true, "minutes": 30, "tools": []}}),
        json!({"test": {"command": ["cargo", "test"], "network": false, "minutes": 31, "tools": []}}),
        json!({"test": {"command": ["cargo", "test"], "network": false, "minutes": 30, "tools": ["cargo"]}}),
    ] {
        assert_eq!(code(&task::duplicates(&differs, &eff).unwrap_err()), Some(Code::CheckDuplicate), "{differs}");
    }
    // The legacy pair under one name in one file is a duplicate table (REQ-SET-2).
    let text =
        "[project]\nname = \"x\"\n\n[checks.test]\ncommand = [\"false\"]\n\n[checks.test]\ncommand = [\"true\"]\n";
    let got = settings::parse(Path::new("settings.toml"), text, Level::Project).unwrap_err();
    assert_eq!(got.iter().map(|e| e.code().unwrap()).collect::<Vec<_>>(), [Code::SettingsInvalid]);
}

#[test]
fn refusal_limit_unobservable() {
    let eff = effective(
        "[project]\nname = \"x\"\n\n[tools.claude]\nadapter = \"claude-code\"\nproviders = [\"anthropic\"]\n\n\
         [tools.codex]\nadapter = \"codex\"\nproviders = [\"openai\"]\n\n\
         [profiles.build]\nkind = \"build\"\ntool = \"claude\"\nmodel = \"m\"\neffort = \"high\"\n\n\
         [profiles.review]\nkind = \"review\"\ntool = \"codex\"\nmodel = \"m\"\neffort = \"high\"\ntokens = 1000\n",
    );
    let fake = |adapter: &'static str, tokens: bool| Program {
        adapter: Some(adapter),
        executable: "fake",
        version: &[],
        tokens,
    };
    let reporting = [fake("claude-code", true), fake("codex", true)];
    let silent = [fake("claude-code", true), fake("codex", false)];
    task::observable(&eff, &reporting, &["build", "review"], true).unwrap();
    // No token limit on the build profile: its tool's report does not matter.
    task::observable(&eff, &[fake("claude-code", false)], &["build"], false).unwrap();
    let refused = |modules: &[Program], profiles: &[&str], tokens| {
        code(&task::observable(&eff, modules, profiles, tokens).unwrap_err()) == Some(Code::LimitUnobservable)
    };
    // The review profile's own limit, on a tool that reports none; the task's limit, likewise; no module at all.
    assert!(refused(&silent, &["review"], false));
    assert!(refused(&[fake("claude-code", false)], &["build"], true));
    assert!(refused(&[], &["build"], true));
}

// ---- REQ-SEC-3 -----------------------------------------------------------------------------------------------------

#[test]
fn refusal_credential_suspected() {
    let key = format!("AKIA{}", "ABCDEFGHIJ012345");
    for text in [
        "-----BEGIN PRIVATE KEY-----\nMII\n".to_owned(),
        "x\n-----BEGIN OPENSSH PRIVATE KEY-----\r\n".to_owned(),
        "-----BEGIN ENCRYPTED RSA PRIVATE KEY-----".to_owned(),
        format!("id = {key}"),
        format!("ghp_{}", "a".repeat(20)),
        format!("token: gho_{}", "A1_-".repeat(5)),
        format!("ghu_{}", "b".repeat(30)),
        format!("ghs_{}", "c".repeat(20)),
        format!("ghr_{}", "d".repeat(20)),
        format!("github_pat_{}", "e".repeat(20)),
        format!("sk-ant-{}", "f".repeat(20)),
        format!("Bearer sk-{}", "g".repeat(20)),
        format!("xoxb-{}", "1".repeat(10)),
        format!("xoxa-{}", "1".repeat(10)),
        format!("xoxp-{}", "1".repeat(10)),
        format!("xoxr-{}", "1".repeat(10)),
        format!("xoxs-{}", "1".repeat(10)),
    ] {
        assert!(scan::suspect(text.as_bytes()).is_some(), "{text}");
    }
    // Near misses: 15 characters after `AKIA`, 19 after `sk-`, words in lower case, a line with more on it.
    for text in [
        format!("AKIA{}", "A".repeat(15)),
        format!("sk-{}", "g".repeat(19)),
        format!("ghp_{}.x", "a".repeat(19)),
        format!("xoxb-{}", "1".repeat(9)),
        "-----BEGIN rsa PRIVATE KEY-----".to_owned(),
        "-----BEGIN RSA  PRIVATE KEY-----".to_owned(),
        "a -----BEGIN PRIVATE KEY-----".to_owned(),
        "-----BEGIN PRIVATE KEY----- ".to_owned(),
        "`AKIA` followed by 16 uppercase letters or digits; `sk-` followed by at least 20 token characters".to_owned(),
    ] {
        assert_eq!(scan::suspect(text.as_bytes()), None, "{text}");
    }
    for name in [".env", "id_rsa", "id_ecdsa", "id_ed25519", "a.pem", "a.p12", "a.pfx", "a.key", ".pem"] {
        assert!(scan::suspect_name(name), "{name}");
    }
    for name in ["id_ed25519.pub", "env", ".envrc", "a.pem.txt", "key", "id_rsa2"] {
        assert!(!scan::suspect_name(name), "{name}");
    }
    // In a domain: content anywhere, a maintainer's uncited material by its name only.
    let f = Fixture::new("credentials", false);
    fs::create_dir_all(f.domain.join("xel/attachments")).unwrap();
    fs::write(f.domain.join("xel/fermento.md"), "nothing\n").unwrap();
    fs::write(f.domain.join("xel/attachments/notes.txt"), format!("{key}\n")).unwrap();
    fs::write(f.domain.join("xel/cited.txt"), "clean\n").unwrap();
    let cited = |p: &Path| p.ends_with("xel/cited.txt");
    assert!(scan::domain(&f.domain, &cited).is_empty());
    fs::write(f.domain.join("xel/cited.txt"), format!("{key}\n")).unwrap();
    fs::write(f.domain.join("xel/attachments/id_rsa"), "x").unwrap();
    fs::create_dir_all(f.domain.join("local/cache")).unwrap();
    fs::write(f.domain.join("local/cache/x"), format!("ghp_{}", "a".repeat(20))).unwrap();
    let found = scan::domain(&f.domain, &cited);
    let at: Vec<String> = found
        .iter()
        .map(|e| {
            assert_eq!(e.code(), Some(Code::CredentialSuspected));
            assert!(e.detail().contains("a heuristic, a cooperative control"));
            match e {
                fabrewk_core::Error::Refused { path, .. } => {
                    path.as_ref().unwrap().strip_prefix(&f.domain).unwrap().display().to_string()
                }
                _ => unreachable!(),
            }
        })
        .collect();
    assert_eq!(at, ["local/cache/x", "xel/attachments/id_rsa", "xel/cited.txt"]);
}

#[test]
fn req_sec_3_unreadable_files() {
    isolated("req_sec_3_unreadable_files", || {
        use std::os::unix::fs::PermissionsExt;
        let f = Fixture::new("unreadable", false);
        fs::create_dir_all(f.domain.join("xel")).unwrap();
        fs::write(f.domain.join("xel/fermento.md"), "x\n").unwrap();
        let secret = f.domain.join("xel/uncited.txt");
        fs::write(&secret, "x").unwrap();
        fs::set_permissions(&secret, fs::Permissions::from_mode(0o000)).unwrap();
        let none = |_: &Path| false;
        assert!(fs::read(&secret).is_err(), "the test runs as a user that reads a file of mode 000");
        // Uncited material made unreadable changes nothing; a file the scan must read and cannot is unknown.
        assert!(scan::domain(&f.domain, &none).is_empty());
        fs::set_permissions(f.domain.join("xel/fermento.md"), fs::Permissions::from_mode(0o000)).unwrap();
        let got: Vec<_> = scan::domain(&f.domain, &none).iter().map(|e| e.code().unwrap()).collect();
        assert_eq!(got, [Code::CheckUnknown]);
        // A file whose name is a credential's is still read; unreadable, it is unknown besides (P7D1-4).
        fs::set_permissions(f.domain.join("xel/fermento.md"), fs::Permissions::from_mode(0o644)).unwrap();
        let key = f.domain.join("id_rsa");
        fs::write(&key, "x").unwrap();
        fs::set_permissions(&key, fs::Permissions::from_mode(0o000)).unwrap();
        let got: Vec<_> = scan::domain(&f.domain, &none).iter().map(|e| e.code().unwrap()).collect();
        assert_eq!(got, [Code::CheckUnknown, Code::CredentialSuspected]);
    });
}

#[test]
fn req_sec_3_links_are_matched_by_name_and_never_followed() {
    let base = tmp("links");
    let outside = base.join("outside");
    fs::create_dir_all(&outside).unwrap();
    let token = format!("ghp_{}", "a".repeat(20));
    fs::write(outside.join("notes.txt"), &token).unwrap();
    // A domain that is a link is not walked (P7D1-3).
    let linked = base.join(".brewstillery");
    std::os::unix::fs::symlink(&outside, &linked).unwrap();
    // However the path is written: a final `/` or `/.` resolves the link before its entry is examined (P7D1-5).
    let named = linked.to_str().unwrap();
    for path in
        [named.to_owned(), format!("{named}/"), format!("{named}/."), format!("{named}/./"), format!("{named}/..")]
    {
        let got: Vec<_> = scan::domain(Path::new(&path), &|_| true).iter().map(|e| e.code().unwrap()).collect();
        assert_eq!(got, [Code::CheckUnknown], "{path}");
    }
    // Inside a domain, a link to a file or a directory is neither read nor walked; a link's name is matched.
    let domain = base.join("domain");
    fs::create_dir_all(&domain).unwrap();
    std::os::unix::fs::symlink(outside.join("notes.txt"), domain.join("notes.txt")).unwrap();
    std::os::unix::fs::symlink(&outside, domain.join("dir")).unwrap();
    assert!(scan::domain(&domain, &|_| true).is_empty());
    let trailing = format!("{}/.", domain.display());
    assert!(scan::domain(Path::new(&trailing), &|_| true).is_empty());
    std::os::unix::fs::symlink(outside.join("notes.txt"), domain.join(".env")).unwrap();
    let got: Vec<_> = scan::domain(&domain, &|_| true).iter().map(|e| e.code().unwrap()).collect();
    assert_eq!(got, [Code::CredentialSuspected]);
}

#[test]
fn req_sec_3_this_repository_passes_the_scan() {
    let domain = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../.brewstillery"));
    let found = scan::domain(domain, &|_| true);
    assert!(found.is_empty(), "{found:?}");
}

#[test]
fn req_sec_3_no_regular_expression_crate() {
    let lock = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../Cargo.lock")).unwrap();
    assert!(!lock.lines().any(|l| l == "name = \"regex\"" || l == "name = \"regex-automata\""));
}
