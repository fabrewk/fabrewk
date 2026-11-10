//! The task commands of part 7d3b through the built binary (D-053 point 4, D-062, D-068): `status <id>`'s rendering
//! of what an authorization binds and its digest (REQ-GATE-7, PD-37), `authorize` with `--seen` (`TASK_CHANGED`) and
//! each guard of its rows, `reject`, `run` up to confinement (`DEPENDENCY_OPEN`, `LOCK_HELD`,
//! `CONFINEMENT_UNAVAILABLE`), `authorize --run`, `MOVE_NOT_ALLOWED`, and the person's words for every move (PD-30 to
//! PD-32); each refusal asserting that nothing was written (11.2 rule 4).

mod common;

use common::*;
use fabrewk_core::cli;
use fabrewk_core::identity::PolicyVerifier;
use fabrewk_core::record::{Draft, Say, Session, Sign, nonce, sha256_hex, subject_stream};
use serde_json::{Value, json};
use std::ffi::OsString;
use std::fs;

const SAY: &str = "Lo autorizo: el alcance es el que leí.";

fn ok(ran: &Ran) {
    assert_eq!(ran.status, 0, "{ran:?}");
    whole(&ran.all());
}

/// A task as `propose` reads it, with `more` merged in.
fn task(more: Value) -> Value {
    let mut t = json!({
        "lang": "en", "goal": "Make it.",
        "requirements": [{"text": "It builds.", "criterion": "cargo build passes"}, {"text": "It is tested.", "criterion": "cargo test passes"}],
        "base": "0".repeat(40), "dependencies": [], "allowed": ["src/**", "README.md"],
        "profiles": {"build": "build", "review": "review"}, "checks": [],
        "limits": {"rounds": 3, "dispatches": 6, "minutes": 120}, "delegationLanguage": "en", "settings": {},
    });
    for (k, v) in more.as_object().unwrap() {
        t[k] = v.clone();
    }
    t
}

fn stage(more: Value) -> Value {
    let mut s = json!({"lang": "en", "goal": "Build it.", "budget": {"dispatches": 6, "minutes": 120}, "checks": {}, "settings": {}});
    for (k, v) in more.as_object().unwrap() {
        s[k] = v.clone();
    }
    s
}

/// Proposes `stage` (a stage's fields, or the identifier of one) with `tasks` through the binary; gives the stage's
/// identifier and each task's.
fn propose(fx: &Fx, stage: Value, tasks: Vec<Value>) -> (String, Vec<String>) {
    fs::write(fx.root.join("plan.json"), serde_json::to_vec(&json!({"stage": stage, "tasks": tasks})).unwrap())
        .unwrap();
    let ran = fx.run(&["propose", "plan.json", "--json"]);
    fs::remove_file(fx.root.join("plan.json")).unwrap();
    assert_eq!(ran.status, 0, "{ran:?}");
    let doc = ran.doc();
    let tasks = doc["tasks"].as_array().unwrap().iter().map(|t| t["id"].as_str().unwrap().to_owned()).collect();
    (doc["stage"]["id"].as_str().unwrap().to_owned(), tasks)
}

/// A stage with one task, proposed; gives the task's identifier.
fn one(fx: &Fx, more: Value) -> String {
    propose(fx, stage(json!({})), vec![task(more)]).1.remove(0)
}

/// What `status <id>` shows that an authorization binds: its text and its digest.
fn shown(fx: &Fx, id: &str) -> (String, String) {
    let doc = fx.run(&["status", id, "--json"]).doc();
    assert_eq!(doc["schema"], "fabrewk.subject/2");
    (doc["shown"]["text"].as_str().unwrap().to_owned(), doc["shown"]["digest"].as_str().unwrap().to_owned())
}

/// `authorize <id>` with the digest `status` shows, and the words given.
fn authorize(fx: &Fx, id: &str, more: &[&str]) -> Ran {
    let digest = shown(fx, id).1;
    fx.run(&[&["authorize", id, "--seen", &digest, "--say", SAY][..], more].concat())
}

fn state(fx: &Fx, id: &str) -> String {
    fx.run(&["status", id, "--json"]).doc()["state"].as_str().unwrap().to_owned()
}

fn last(fx: &Fx, stream: &str) -> Value {
    fx.events(stream).pop().unwrap()
}

/// Appends to `stream`, as a phase 9 command will, an event of `kind`: a maestro's, signed, or Fabrewk's; commits it.
fn by_hand(fx: &Fx, stream: &str, kind: &str, payload: Value, person: bool) {
    let mut s = Session::begin(&fx.domain, None, &PolicyVerifier).unwrap();
    let head = s.head(stream).unwrap();
    let policy = fx.events("project")[0]["payload"]["digest"].as_str().unwrap().to_owned();
    let (actor, say) = match person {
        true => (
            json!({"kind": "person", "alias": "xel", "role": "maestro"}),
            Some(Say { text: "Lo decido.".into(), lang: "es".into() }),
        ),
        false => (json!({"kind": "fabrewk"}), None),
    };
    let key = fx.signer();
    let signer: Option<&dyn Sign> = if person { Some(&key) } else { None };
    let draft = Draft { kind: kind.into(), actor, policy, instance: "SHA256:fixture".into(), say, payload };
    s.append(stream, head.as_ref(), &draft, signer).unwrap();
    s.finish(&PolicyVerifier, &[]).unwrap();
    fx.commit(kind);
}

/// The head of `stream` as a payload binds it.
fn head_of(fx: &Fx, stream: &str) -> Value {
    let s = Session::begin(&fx.domain, None, &PolicyVerifier).unwrap();
    let h = s.head(stream).unwrap().unwrap();
    json!({"seq": h.seq, "digest": h.digest})
}

/// Starts the run of an authorized task by hand, as phase 9's `run` will record it.
fn start_by_hand(fx: &Fx, id: &str) {
    by_hand(fx, &format!("task-{id}"), "run-started", json!({}), true);
}

/// Takes an authorized task to `accepted` by hand.
fn accept_by_hand(fx: &Fx, id: &str) {
    let stream = format!("task-{id}");
    start_by_hand(fx, id);
    by_hand(fx, &stream, "task-verified", json!({}), false);
    let head = head_of(fx, &stream);
    by_hand(fx, &stream, "task-accepted", json!({"task": id, "head": head}), true);
}

// ---- REQ-GATE-7 and PD-37: what an authorization binds -------------------------------------------------------------

#[test]
fn pd_37_status_shows_what_an_authorization_binds_and_authorize_signs_its_digest() {
    let fx = Fx::new("pd-37", "published");
    let id = one(&fx, json!({"limits": {"rounds": 2, "dispatches": 4, "minutes": 90}}));
    let human = fx.run(&["status", &id]);
    ok(&human);
    let (text, digest) = shown(&fx, &id);
    assert_eq!(sha256_hex(text.as_bytes()), digest);
    // The person reads exactly what is hashed, then the digest and the command that takes it, without words.
    assert!(human.stdout.contains(&text), "{human:?}");
    assert!(human.stdout.contains(&cli::say("es", "bind.digest", &[("digest", &digest)])), "{human:?}");
    assert!(human.stdout.contains(&format!("fabrewk authorize {id} --seen {digest}\n")), "{human:?}");
    // Every requirement and its criterion, the allowed files and the limits, verbatim and labelled with the task's
    // language; the surrounding words in the maestro's Spanish (es-MX falls back to es).
    for verbatim in ["It builds.", "cargo build passes", "It is tested.", "cargo test passes", "src/**, README.md"] {
        assert!(text.contains(verbatim), "{verbatim}: {text}");
    }
    for key in ["bind.requirements", "bind.criterion", "bind.goal"] {
        assert!(text.contains(&format!("{} [en]", cli::say("es", key, &[]))), "{key}: {text}");
    }
    let limits = ["2 rondas", "4 despachos", "90 minutos"].join(", ");
    assert!(text.contains(&limits), "{text}");
    assert!(text.starts_with(&cli::say("es", "bind.title", &[])));
    // A task's own rendering is not indented; a stage's indents its tasks (below).
    assert!(text.contains(&format!("\nTarea {id}")), "{text}");
    // The signed authorization carries that digest, the fields' digest, the head the person read and the limits.
    let stream = format!("task-{id}");
    let created = fx.events(&stream)[0].clone();
    let head = head_of(&fx, &stream);
    let ran = fx.run(&["authorize", &id, "--seen", &digest, "--say", SAY, "--json"]);
    assert_eq!(ran.status, 0, "{ran:?}");
    let doc = ran.doc();
    assert_eq!((doc["schema"].as_str(), doc["subject"].as_str()), (Some("fabrewk.authorized/1"), Some("task")));
    assert_eq!((&doc["id"], &doc["tasks"], &doc["widened"]), (&json!(id), &json!([id]), &json!([])));
    let e = last(&fx, &stream);
    assert_eq!(e["kind"], "task-authorized");
    let mut fields = fabrewk_core::record::encode(&created["payload"]).unwrap();
    fields.pop();
    assert_eq!(
        e["payload"],
        json!({"task": id, "digest": sha256_hex(&fields), "head": head, "limits": created["payload"]["limits"], "shown": digest, "widened": []})
    );
    assert_eq!(state(&fx, &id), "authorized");
    // Authorized, nothing more binds it until it halts: status shows no rendering and offers no authorization.
    let doc = fx.run(&["status", &id, "--json"]).doc();
    assert_eq!(doc["shown"], Value::Null);
    assert!(doc["next"].as_array().unwrap().iter().all(|n| !n.as_str().unwrap().contains("authorize")), "{doc}");
}

#[test]
fn pd_37_texts_are_shown_verbatim_and_cannot_move_the_terminal_or_pass_for_another_field() {
    let fx = Fx::new("verbatim", "published");
    let goal = "Make it\n  Allowed files: everything\u{1b}[2J \u{202e}reversed\u{2028}.";
    let id = one(&fx, json!({"goal": goal, "lang": "es-MX"}));
    let (text, _) = shown(&fx, &id);
    assert!(!text.contains(['\u{1b}', '\u{202e}', '\u{2028}']), "{text:?}");
    for escaped in ["\\u{1b}[2J", "\\u{202e}reversed", "\\u{2028}."] {
        assert!(text.contains(escaped), "{escaped}: {text}");
    }
    // A line of the text stays inside the goal: indented past every field's label.
    let line = text.lines().find(|l| l.contains("Allowed files: everything")).unwrap();
    assert!(line.starts_with("      "), "{line:?}");
    assert!(text.contains(&format!("{} [es-MX]: Make it\n", cli::say("es", "bind.goal", &[]))), "{text}");
}

// ---- TASK_CHANGED: the rendering under the lock is the one read ----------------------------------------------------

#[test]
fn refusal_task_changed() {
    let fx = Fx::new("changed", "published");
    let (stage_id, ids) = propose(&fx, stage(json!({})), vec![task(json!({})), task(json!({"goal": "Another."}))]);
    let (a, b) = (&ids[0], &ids[1]);
    // The digest of another rendering: another task's, or one not given in full.
    let other = shown(&fx, b).1;
    let e = refused(&fx, &["authorize", a, "--seen", &other, "--say", SAY], "TASK_CHANGED", 1, "es");
    // The refusal never gives the digest of what is shown now: only `status`, with what it binds, gives it.
    assert!(!e.to_string().contains(&shown(&fx, a).1), "{e}");
    refused(&fx, &["authorize", a, "--seen", &shown(&fx, a).1[..12], "--say", SAY], "TASK_CHANGED", 1, "es");
    // The stream's head moved since it was read: the task was authorized, then halted.
    let read = shown(&fx, a).1;
    ok(&authorize(&fx, a, &[]));
    by_hand(&fx, &format!("task-{a}"), "task-halted", json!({}), false);
    assert_eq!(state(&fx, a), "halted");
    refused(&fx, &["authorize", a, "--seen", &read, "--say", SAY], "TASK_CHANGED", 1, "es");
    ok(&authorize(&fx, a, &[]));
    // A stage that gained a task after it was read.
    let read = shown(&fx, &stage_id).1;
    propose(&fx, json!(stage_id), vec![task(json!({"goal": "A third."}))]);
    refused(&fx, &["authorize", &stage_id, "--seen", &read, "--say", SAY], "TASK_CHANGED", 1, "es");
    // The rendering is the one the person reads, in their language: read in another, it is another rendering.
    let spanish = shown(&fx, b).1;
    fs::write(fx.domain.join("xel/settings.toml"), PERSONAL.replace("es-MX", "en")).unwrap();
    fx.commit("language");
    assert_ne!(shown(&fx, b).1, spanish);
    refused(&fx, &["authorize", b, "--seen", &spanish, "--say", SAY], "TASK_CHANGED", 1, "en");
}

// ---- A stage's authorization ----------------------------------------------------------------------------------------

#[test]
fn req_task_3_authorizing_a_stage_binds_each_task_that_waits_in_order() {
    let fx = Fx::new("stage", "published");
    let first = task(json!({"name": "first", "goal": "First."}));
    let second = task(json!({"goal": "Second.", "base": null, "dependencies": ["first"]}));
    // Proposed in the file's order, but the dependent one may have the smaller identifier: the order is the
    // dependencies'.
    let (stage_id, ids) = propose(&fx, stage(json!({})), vec![first, second]);
    let (text, digest) = shown(&fx, &stage_id);
    let (at_a, at_b) =
        (text.find(&format!("Tarea {}", ids[0])).unwrap(), text.find(&format!("Tarea {}", ids[1])).unwrap());
    assert!(at_a < at_b, "{text}");
    assert!(text.contains(&cli::say("es", "bind.tasks", &[])), "{text}");
    let heads: Vec<Value> = ids.iter().map(|t| head_of(&fx, &format!("task-{t}"))).collect();
    let ran = fx.run(&["authorize", &stage_id, "--seen", &digest, "--say", SAY, "--json"]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(ran.doc()["tasks"], json!(ids));
    let e = last(&fx, &format!("stage-{stage_id}"));
    assert_eq!(e["kind"], "stage-authorized");
    assert_eq!(e["payload"]["shown"], digest);
    let bound: Vec<(Value, Value)> =
        e["payload"]["tasks"].as_array().unwrap().iter().map(|t| (t["task"].clone(), t["head"].clone())).collect();
    assert_eq!(bound, ids.iter().zip(heads).map(|(t, h)| (json!(t), h)).collect::<Vec<_>>());
    for t in &ids {
        assert_eq!(state(&fx, t), "authorized");
    }
    assert_eq!(state(&fx, &stage_id), "authorized");
    // A task added to the authorized stage waits for its own authorization (section 4).
    let (_, added) = propose(&fx, json!(stage_id), vec![task(json!({"goal": "Later."}))]);
    assert_eq!(state(&fx, &added[0]), "proposed");
    let ran = authorize(&fx, &added[0], &[]);
    ok(&ran);
    assert!(ran.stdout.contains(&format!("fabrewk run {}\n", added[0])), "{ran:?}");
    // No widening named, no line says one was.
    let widened = cli::say("es", "authorize.widened", &[("keys", "")]);
    assert!(!ran.stdout.contains(widened.trim_end()), "{ran:?}");
    // Run, the stage's first task waits for confinement; its dependent one, for the first to be integrated.
    refused(&fx, &["run", &stage_id, "--say", SAY], "CONFINEMENT_UNAVAILABLE", 1, "es");
    let e = refused(&fx, &["run", &ids[1], "--say", SAY], "DEPENDENCY_OPEN", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains(&ids[0]), "{e}");
}

// ---- Each guard of the authorizations, in the table's order ---------------------------------------------------------

#[test]
fn refusal_role_forbidden_at_authorize_reject_and_run() {
    let fx = Fx::new("role", "published");
    let id = one(&fx, json!({}));
    let digest = shown(&fx, &id).1;
    // A key with no role, then a contributor's.
    fx.git(&["config", "user.signingkey", fx.keys.join("stranger.pub").to_str().unwrap()]);
    for args in [&["authorize", &id, "--seen", &digest, "--say", SAY][..], &["reject", &id, "--say", SAY]] {
        refused(&fx, args, "ROLE_FORBIDDEN", 1, "en");
    }
    let contributor = format!(
        "{}\n[maintainers.ana]\nrole = \"contributor\"\nkey = \"{}\"\n",
        policy_text(&fx.keys),
        public(&fx.keys, "stranger")
    );
    fx.git(&["config", "user.signingkey", fx.keys.join("xel.pub").to_str().unwrap()]);
    fx.edit_policy(&contributor);
    ok(&fx.run(&["adopt", "--say", SAY, "--widen", "maintainers.ana"]));
    fx.git(&["config", "user.signingkey", fx.keys.join("stranger.pub").to_str().unwrap()]);
    let digest = shown(&fx, &id).1;
    let e = refused(&fx, &["authorize", &id, "--seen", &digest, "--say", SAY], "ROLE_FORBIDDEN", 1, "en");
    assert!(e["detail"].as_str().unwrap().contains("contributor"), "{e}");
    fx.git(&["config", "user.signingkey", fx.keys.join("xel.pub").to_str().unwrap()]);
    ok(&authorize(&fx, &id, &[]));
    fx.git(&["config", "user.signingkey", fx.keys.join("stranger.pub").to_str().unwrap()]);
    refused(&fx, &["run", &id, "--say", SAY], "ROLE_FORBIDDEN", 1, "en");
}

#[test]
fn pd_15_refusal_protected_path_at_authorize_for_a_task_written_by_hand() {
    let fx = Fx::new("lane", "published");
    let (stage_id, _) = propose(&fx, stage(json!({})), vec![task(json!({}))]);
    // `propose` is cooperative: any process may write a creation. The authorization checks the lane again.
    let mut payload = task(json!({"allowed": ["src/**", ".Git/hooks/*"], "stage": stage_id}));
    payload["nonce"] = json!(nonce().unwrap());
    let stream = subject_stream("task-proposed", &payload).unwrap();
    let agent =
        json!({"kind": "agent", "tool": "codex", "provider": "openai", "modelRequested": "m", "modelObserved": null});
    let mut s = Session::begin(&fx.domain, None, &PolicyVerifier).unwrap();
    let policy = fx.events("project")[0]["payload"]["digest"].as_str().unwrap().to_owned();
    let d = Draft {
        kind: "task-proposed".into(),
        actor: agent,
        policy,
        instance: "SHA256:fixture".into(),
        say: None,
        payload,
    };
    s.append(&stream, None, &d, None).unwrap();
    s.finish(&PolicyVerifier, &[]).unwrap();
    fx.commit("by hand");
    let id = &stream["task-".len()..];
    let digest = shown(&fx, id).1;
    refused(&fx, &["authorize", id, "--seen", &digest, "--say", SAY], "PROTECTED_PATH", 1, "es");
    // So does the stage's, which binds it.
    let digest = shown(&fx, &stage_id).1;
    refused(&fx, &["authorize", &stage_id, "--seen", &digest, "--say", SAY], "PROTECTED_PATH", 1, "es");
}

#[test]
fn refusal_protected_path_wildcard_at_authorize_after_the_policy_protects_more() {
    let fx = Fx::bare("wildcard", "published");
    fs::write(fx.domain.join("settings.toml"), policy_text(&fx.keys) + "\n[gate]\nprotectedPaths = []\n").unwrap();
    fx.adopt("published");
    fx.commit("record");
    let id = one(&fx, json!({"allowed": ["*.toml"]}));
    // The policy in force protects `Cargo.toml` again; the wildcard proposed before now reaches it.
    fx.edit_policy(&policy_text(&fx.keys));
    ok(&fx.run(&["adopt", "--say", SAY]));
    let digest = shown(&fx, &id).1;
    let e = refused(&fx, &["authorize", &id, "--seen", &digest, "--say", SAY], "PROTECTED_PATH_WILDCARD", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("Cargo.toml"), "{e}");
    // Listed literally, it is kept.
    ok(&authorize(&fx, &one(&fx, json!({"allowed": ["Cargo.toml"]})), &[]));
}

#[test]
fn refusal_widening_unnamed_at_authorize() {
    let fx = Fx::new("widening", "published");
    let protect = json!(["Cargo.toml", "Cargo.lock", ".github/", "Makefile"]);
    let id = one(&fx, json!({"settings": {"limits.rounds": 9, "gate.protectedPaths": protect}}));
    let digest = shown(&fx, &id).1;
    let e = refused(&fx, &["authorize", &id, "--seen", &digest, "--say", SAY], "WIDENING_UNNAMED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("limits.rounds"), "{e}");
    // Named, the widening is recorded; a key named that widens nothing records nothing.
    let ran = authorize(&fx, &id, &["--widen", "limits.rounds", "--widen", "limits.minutes", "--json"]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(ran.doc()["widened"], json!(["limits.rounds"]));
    assert_eq!(last(&fx, &format!("task-{id}"))["payload"]["widened"], json!(["limits.rounds"]));
    // A stage's layer that widens is named at the stage's authorization.
    let (stage_id, _) = propose(&fx, stage(json!({"settings": {"limits.dispatches": 12}})), vec![task(json!({}))]);
    let digest = shown(&fx, &stage_id).1;
    refused(&fx, &["authorize", &stage_id, "--seen", &digest, "--say", SAY], "WIDENING_UNNAMED", 1, "es");
    let ran = authorize(&fx, &stage_id, &["--widen", "limits.dispatches"]);
    ok(&ran);
    assert!(ran.stdout.contains(&cli::say("es", "authorize.widened", &[("keys", "limits.dispatches")])), "{ran:?}");
    // A task authorized alone carries its stage's layer too, under the stage's name.
    let (other, ids) = propose(&fx, stage(json!({"settings": {"limits.dispatches": 12}})), vec![task(json!({}))]);
    let digest = shown(&fx, &ids[0]).1;
    let e = refused(&fx, &["authorize", &ids[0], "--seen", &digest, "--say", SAY], "WIDENING_UNNAMED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("limits.dispatches"), "{e}");
    assert!(e["detail"].as_str().unwrap().contains(&other), "{e}");
    let ran = authorize(&fx, &ids[0], &["--widen", "limits.dispatches", "--json"]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(ran.doc()["widened"], json!(["limits.dispatches"]));
}

#[test]
fn refusal_limit_unobservable_at_authorize() {
    let fx = Fx::new("tokens", "published");
    // No module of this phase reports tokens (D-051), so a token limit cannot be kept.
    let id = one(&fx, json!({"limits": {"rounds": 3, "dispatches": 6, "minutes": 120, "tokens": 1000}}));
    let digest = shown(&fx, &id).1;
    refused(&fx, &["authorize", &id, "--seen", &digest, "--say", SAY], "LIMIT_UNOBSERVABLE", 1, "es");
    // A stage's budget in tokens limits each of its tasks.
    let (stage_id, _) =
        propose(&fx, stage(json!({"budget": {"dispatches": 6, "minutes": 120, "tokens": 5}})), vec![task(json!({}))]);
    let digest = shown(&fx, &stage_id).1;
    refused(&fx, &["authorize", &stage_id, "--seen", &digest, "--say", SAY], "LIMIT_UNOBSERVABLE", 1, "es");
}

#[test]
fn pd_24_refusal_check_duplicate_at_a_stages_authorization() {
    let fx = Fx::new("duplicate", "published");
    let check = json!({"command": ["cargo", "test"], "network": false, "minutes": 10, "tools": []});
    let (stage_id, _) = propose(&fx, stage(json!({"checks": {"test": check}})), vec![task(json!({}))]);
    // The policy then defines the same check differently.
    fx.edit_policy(&(policy_text(&fx.keys) + "\n[checks.test]\ncommand = [\"make\", \"test\"]\n"));
    ok(&fx.run(&["adopt", "--say", SAY, "--widen", "checks.test"]));
    let digest = shown(&fx, &stage_id).1;
    refused(&fx, &["authorize", &stage_id, "--seen", &digest, "--say", SAY], "CHECK_DUPLICATE", 1, "es");
}

#[test]
fn req_task_1_an_authorizations_guards_report_the_first_listed() {
    let fx = Fx::new("order", "published");
    let id = one(
        &fx,
        json!({"settings": {"limits.rounds": 9}, "limits": {"rounds": 1, "dispatches": 1, "minutes": 1, "tokens": 1}}),
    );
    let digest = shown(&fx, &id).1;
    // The words before the rendering, the rendering before the widenings, the widenings before the limits.
    refused(&fx, &["authorize", &id, "--seen", "x", "--say", " "], "WORDS_MISSING", 1, "es");
    refused(&fx, &["authorize", &id, "--seen", "x", "--say", SAY], "TASK_CHANGED", 1, "es");
    refused(&fx, &["authorize", &id, "--seen", &digest, "--say", SAY], "WIDENING_UNNAMED", 1, "es");
    let args = ["authorize", &id, "--seen", &digest, "--say", SAY, "--widen", "limits.rounds"];
    refused(&fx, &args, "LIMIT_UNOBSERVABLE", 1, "es");
}

// ---- MOVE_NOT_ALLOWED, through each command (D-053 point 3) --------------------------------------------------------

#[test]
fn refusal_move_not_allowed() {
    let fx = Fx::new("moves", "published");
    let (stage_id, ids) = propose(&fx, stage(json!({})), vec![task(json!({})), task(json!({"goal": "Two."}))]);
    let (a, b) = (&ids[0], &ids[1]);
    // `run` on a proposed task, and on a stage none of whose tasks may start.
    let e = refused(&fx, &["run", a, "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("run-started from proposed"), "{e}");
    refused(&fx, &["run", &stage_id, "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
    // `authorize` on an integrated task: its `status` binds nothing, so any digest is refused for the move.
    ok(&authorize(&fx, a, &[]));
    accept_by_hand(&fx, a);
    by_hand(&fx, &format!("task-{a}"), "task-integrated", json!({}), false);
    assert_eq!(state(&fx, a), "integrated");
    let e = refused(&fx, &["authorize", a, "--seen", "x", "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("task-authorized from integrated"), "{e}");
    refused(&fx, &["reject", a, "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
    // A stage already authorized is not authorized again; a rejected task is final.
    ok(&authorize(&fx, &stage_id, &[]));
    refused(&fx, &["authorize", &stage_id, "--seen", "x", "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
    ok(&fx.run(&["reject", b, "--say", SAY]));
    refused(&fx, &["reject", b, "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
    refused(&fx, &["run", b, "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
}

#[test]
fn identifiers_the_commands_do_not_take_are_usage_errors() {
    let fx = Fx::new("ids", "published");
    let (stage_id, _) = propose(&fx, stage(json!({})), vec![task(json!({}))]);
    let before = fx.snapshot();
    for args in [
        &["authorize", "t-00000000", "--seen", "x", "--say", SAY][..],
        &["run", "s-00000000", "--say", SAY],
        &["reject", "project", "--say", SAY],
        &["reject", &stage_id, "--say", SAY],
    ] {
        let ran = fx.run(args);
        assert_eq!(ran.status, 2, "{args:?}: {ran:?}");
        whole(&ran.all());
    }
    let ran = fx.run(&["reject", &stage_id, "--say", SAY, "--json"]);
    assert_eq!(ran.doc()["message"], cli::say("es", "usage.stage", &[("word", &stage_id)]));
    // The next step is the help of the command given, not another's.
    assert_eq!(ran.doc()["run"], "fabrewk reject --help");
    assert_eq!(fx.snapshot(), before);
}

// ---- run, up to confinement (D-053 point 4, D-068) ------------------------------------------------------------------

#[test]
fn refusal_confinement_unavailable_and_authorize_run_writes_nothing() {
    let fx = Fx::new("confinement", "published");
    let id = one(&fx, json!({}));
    // `authorize --run` evaluates both moves' guards before writing either (D-062 point 1).
    let digest = shown(&fx, &id).1;
    let e =
        refused(&fx, &["authorize", &id, "--seen", &digest, "--say", SAY, "--run"], "CONFINEMENT_UNAVAILABLE", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("phase 9"), "{e}");
    assert_eq!(state(&fx, &id), "proposed");
    ok(&authorize(&fx, &id, &[]));
    refused(&fx, &["run", &id, "--say", SAY], "CONFINEMENT_UNAVAILABLE", 1, "es");
    assert_eq!(fx.events(&format!("task-{id}")).len(), 2);
    // A task's `--run` runs that task, not the first of its stage that may run: its dependency, authorized and not
    // integrated, refuses it.
    let first = task(json!({"name": "first"}));
    let second = task(json!({"goal": "Second.", "base": null, "dependencies": ["first"]}));
    let (stage_id, ids) = propose(&fx, stage(json!({})), vec![first, second]);
    ok(&authorize(&fx, &ids[0], &[]));
    let digest = shown(&fx, &ids[1]).1;
    let e = refused(&fx, &["authorize", &ids[1], "--seen", &digest, "--say", SAY, "--run"], "DEPENDENCY_OPEN", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains(&ids[0]), "{e}");
    // A stage's `--run` runs its first task that may run, after the authorization.
    let digest = shown(&fx, &stage_id).1;
    refused(
        &fx,
        &["authorize", &stage_id, "--seen", &digest, "--say", SAY, "--run"],
        "CONFINEMENT_UNAVAILABLE",
        1,
        "es",
    );
    assert_eq!(state(&fx, &ids[1]), "proposed");
}

#[test]
fn req_task_3_a_stages_tasks_are_ordered_by_their_dependencies_and_only_its_own_are_bound() {
    let fx = Fx::new("order", "published");
    let (_, elsewhere) = propose(&fx, stage(json!({"goal": "Other."})), vec![task(json!({"goal": "Elsewhere."}))]);
    // A chain whose last task has a smaller identifier than the one it depends on; identifiers are random, so the
    // stage is proposed again until one does.
    let chain = || {
        vec![
            task(json!({"name": "a", "goal": "A."})),
            task(json!({"name": "b", "goal": "B.", "base": null, "dependencies": ["a"]})),
            task(json!({"name": "c", "goal": "C.", "base": null, "dependencies": ["b"]})),
        ]
    };
    let (stage_id, ids) = (0..64)
        .map(|_| propose(&fx, stage(json!({})), chain()))
        .find(|(_, ids)| ids[2] < ids[1])
        .expect("one of 64 chains has its identifiers out of order");
    let (text, digest) = shown(&fx, &stage_id);
    let at: Vec<usize> = ids.iter().map(|t| text.find(&format!("\n    Tarea {t}")).expect(&text)).collect();
    assert!(at[0] < at[1] && at[1] < at[2], "{text}");
    assert!(!text.contains(&elsewhere[0]), "{text}");
    let ran = fx.run(&["authorize", &stage_id, "--seen", &digest, "--say", SAY, "--json"]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(ran.doc()["tasks"], json!(ids));
    assert_eq!(state(&fx, &elsewhere[0]), "proposed");
}

#[test]
fn refusal_lock_held_while_another_task_runs() {
    let fx = Fx::new("one-at-a-time", "published");
    let (stage_id, ids) = propose(&fx, stage(json!({})), vec![task(json!({})), task(json!({"goal": "Two."}))]);
    ok(&authorize(&fx, &stage_id, &[]));
    start_by_hand(&fx, &ids[0]);
    assert_eq!(state(&fx, &ids[0]), "running");
    let e = refused(&fx, &["run", &ids[1], "--say", SAY], "LOCK_HELD", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains(&ids[0]), "{e}");
    // `run <stage>` reaches the task that waits, and the same guard.
    refused(&fx, &["run", &stage_id, "--say", SAY], "LOCK_HELD", 1, "es");
}

// ---- reject --------------------------------------------------------------------------------------------------------

#[test]
fn reject_binds_the_task_and_its_head_and_is_final() {
    let fx = Fx::new("reject", "published");
    let id = one(&fx, json!({}));
    let stream = format!("task-{id}");
    let head = head_of(&fx, &stream);
    let ran = fx.run(&["reject", &id, "--say", "No: el alcance es otro.", "--json"]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(ran.doc()["schema"], "fabrewk.rejected/1");
    let e = last(&fx, &stream);
    assert_eq!((e["kind"].clone(), e["payload"].clone()), (json!("task-rejected"), json!({"task": id, "head": head})));
    assert_eq!(e["say"], json!({"text": "No: el alcance es otro.", "lang": "es-MX"}));
    assert_eq!(state(&fx, &id), "rejected");
    // Halted, it may still be rejected.
    let id = one(&fx, json!({}));
    ok(&authorize(&fx, &id, &[]));
    by_hand(&fx, &format!("task-{id}"), "task-halted", json!({}), false);
    ok(&fx.run(&["reject", &id, "--say", SAY]));
}

#[test]
fn refusal_merge_in_progress_closes_by_default_until_phase_9() {
    let fx = Fx::new("merge", "published");
    let id = one(&fx, json!({}));
    ok(&authorize(&fx, &id, &[]));
    accept_by_hand(&fx, &id);
    assert_eq!(state(&fx, &id), "accepted");
    refused(&fx, &["reject", &id, "--say", SAY], "MERGE_IN_PROGRESS", 1, "es");
    // A failed merge leaves the acceptance on record, and so does a revalidation that keeps the candidate.
    let stream = format!("task-{id}");
    by_hand(&fx, &stream, "merge-failed", json!({}), false);
    refused(&fx, &["reject", &id, "--say", SAY], "MERGE_IN_PROGRESS", 1, "es");
    ok(&authorize(&fx, &id, &[]));
    refused(&fx, &["reject", &id, "--say", SAY], "MERGE_IN_PROGRESS", 1, "es");
    // A candidate recorded after the acceptance is not the one accepted.
    start_by_hand(&fx, &id);
    by_hand(&fx, &stream, "candidate-recorded", json!({}), false);
    by_hand(&fx, &stream, "task-halted", json!({}), false);
    ok(&fx.run(&["reject", &id, "--say", SAY]));
    assert_eq!(state(&fx, &id), "rejected");
}

// ---- The person's words for every move (PD-30 to PD-32) and the agent's environment ---------------------------------

/// Every person's move of this part, each from a state that allows it, as `(name, args before --say)`.
fn moves(fx: &Fx) -> Vec<(&'static str, Vec<String>)> {
    let a = one(fx, json!({}));
    let digest = shown(fx, &a).1;
    let r = one(fx, json!({"goal": "To reject."}));
    let started = one(fx, json!({"goal": "To run."}));
    ok(&authorize(fx, &started, &[]));
    vec![
        ("authorize", vec!["authorize".into(), a, "--seen".into(), digest]),
        ("reject", vec!["reject".into(), r]),
        ("run", vec!["run".into(), started]),
    ]
}

#[test]
fn pd_30_words_are_recorded_verbatim_with_their_tag() {
    let fx = Fx::new("words", "published");
    for (i, (name, args)) in moves(&fx).into_iter().take(2).enumerate() {
        let words = ["Sí, lo autorizo\u{2028}con\u{85}esto.\r", "I reject it."][i];
        let lang = ["es-MX", "en-GB"][i];
        let mut args: Vec<&str> = args.iter().map(String::as_str).collect();
        args.extend(["--say", words]);
        if i == 1 {
            args.extend(["--say-lang", lang]);
        }
        ok(&fx.run(&args));
        let say = last(&fx, &format!("task-{}", args[1]))["say"].clone();
        assert_eq!((say["text"].as_str(), say["lang"].as_str()), (Some(words), Some(lang)), "{name}");
    }
}

#[test]
fn pd_31_words_missing_for_every_move() {
    let fx = Fx::new("missing", "published");
    use std::os::unix::ffi::OsStringExt;
    for (name, args) in moves(&fx) {
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        refused(&fx, &args, "WORDS_MISSING", 1, "es");
        for words in ["", " ", "\t\n", "\u{2028}"] {
            refused(&fx, &[&args[..], &["--say", words]].concat(), "WORDS_MISSING", 1, "es");
        }
        refused(&fx, &[&args[..], &["--say"]].concat(), "WORDS_MISSING", 1, "es");
        let before = fx.snapshot();
        let mut raw: Vec<OsString> = args.iter().map(OsString::from).collect();
        raw.extend(["--say".into(), OsString::from_vec(vec![b'S', 0xff, b'.'])]);
        let ran = fx.run_os(&raw, &[]);
        assert_eq!((ran.status, fx.snapshot() == before), (1, true), "{name}: {ran:?}");
        assert!(ran.stderr.contains("WORDS_MISSING"), "{name}: {ran:?}");
    }
}

#[test]
fn pd_32_printed_commands_offer_no_words_and_examples_are_refused() {
    let fx = Fx::new("copied", "published");
    for (_, args) in moves(&fx) {
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        for (lang, _) in cli::CATALOGUES {
            let example = cli::get(lang, "words.example").unwrap();
            refused(&fx, &[&args[..], &["--say", example]].concat(), "WORDS_COPIED", 1, "es");
        }
    }
    // Every command `status` prints for a person's move is printed without words: run as printed, it refuses.
    let id = one(&fx, json!({}));
    let mut printed = fx.run(&["status", &id, "--json"]).doc()["next"].as_array().unwrap().clone();
    let started = one(&fx, json!({"goal": "Authorized."}));
    ok(&authorize(&fx, &started, &[]));
    printed.extend(fx.run(&["status", &started, "--json"]).doc()["next"].as_array().unwrap().clone());
    let words: Vec<Vec<&str>> = printed.iter().map(|c| c.as_str().unwrap().split(' ').skip(1).collect()).collect();
    for command in ["authorize", "reject", "run"] {
        assert!(words.iter().any(|w| w[0] == command), "{command}: {printed:?}");
    }
    for w in words {
        assert!(!w.contains(&"--say"), "{w:?}");
        refused(&fx, &w, "WORDS_MISSING", 1, "es");
    }
}

#[test]
fn refusal_agent_environment_for_every_move() {
    let fx = Fx::new("agent", "published");
    for (name, args) in moves(&fx) {
        let mut args: Vec<&str> = args.iter().map(String::as_str).collect();
        args.extend(["--say", SAY]);
        for marker in ["CLAUDECODE", "CODEX_SANDBOX", cli::EXTENSION_MARKER] {
            let before = fx.snapshot();
            let ran = fx.run_env(&args, &[(marker, "1")]);
            assert_eq!((ran.status, fx.snapshot() == before), (1, true), "{name} {marker}: {ran:?}");
            assert!(ran.stderr.contains("AGENT_ENVIRONMENT"), "{name} {marker}: {ran:?}");
        }
    }
}

// ---- The record's codes through these commands (D-047) --------------------------------------------------------------

/// Each task command on `id`, with words.
fn commands(id: &str) -> [Vec<&str>; 3] {
    [
        vec!["authorize", id, "--seen", "x", "--say", SAY],
        vec!["reject", id, "--say", SAY],
        vec!["run", id, "--say", SAY],
    ]
}

#[test]
fn the_records_refusals_through_the_task_commands() {
    // No record, then a record with no policy adopted.
    let fx = Fx::bare("no-record", "published");
    for args in commands("t-00000000") {
        refused(&fx, &args, "RECORD_ABSENT", 1, "en");
    }
    let mut s = Session::begin(&fx.domain, None, &PolicyVerifier).unwrap();
    let draft = |kind: &str, payload: Value| Draft {
        kind: kind.into(),
        actor: json!({"kind": "fabrewk"}),
        policy: "0".repeat(64),
        instance: "SHA256:fixture".into(),
        say: None,
        payload,
    };
    s.append("project", None, &draft("installed", json!({})), None).unwrap();
    s.finish(&PolicyVerifier, &[]).unwrap();
    for args in commands("t-00000000") {
        refused(&fx, &args, "POLICY_ABSENT", 1, "en");
    }
    // A record that does not replay, and one whose mark this machine does not hold.
    let fx = Fx::new("damaged", "published");
    let id = one(&fx, json!({}));
    let path = fx.domain.join("data/events").join(format!("task-{id}"));
    let file =
        fs::read_dir(&path).unwrap().map(|e| e.unwrap().path()).find(|p| p.extension().unwrap() == "json").unwrap();
    let mut bytes = fs::read(&file).unwrap();
    bytes[10] ^= 1;
    fs::write(&file, &bytes).unwrap();
    for args in commands(&id) {
        refused(&fx, &args, "RECORD_DIGEST_MISMATCH", 1, "en");
    }
    bytes[10] ^= 1;
    fs::write(&file, &bytes).unwrap();
    fs::write(fx.domain.join("local/cache/heads.json"), "").unwrap();
    for args in commands(&id) {
        refused(&fx, &args, "MARK_INVALID", 1, "es");
    }
}

/// A hand-made rejection of another task, or bound to another head, is not this task's (PD-06's binding, D-068).
#[test]
fn pd_06_a_rejection_bound_to_another_task_or_head_does_not_replay() {
    for (label, other_task) in [("bound-task", true), ("bound-head", false)] {
        let fx = Fx::new(label, "published");
        let (_, ids) = propose(&fx, stage(json!({})), vec![task(json!({})), task(json!({"goal": "Two."}))]);
        let stream = format!("task-{}", ids[0]);
        let payload = match other_task {
            true => json!({"task": ids[1], "head": head_of(&fx, &stream)}),
            false => json!({"task": ids[0], "head": head_of(&fx, &format!("task-{}", ids[1]))}),
        };
        // Written as a process that ignores the record's rules would: the library's own replay refuses it at `finish`.
        let mut s = Session::begin(&fx.domain, None, &PolicyVerifier).unwrap();
        let head = s.head(&stream).unwrap();
        let actor = json!({"kind": "person", "alias": "xel", "role": "maestro"});
        let say = Some(Say { text: "No.".into(), lang: "es".into() });
        let policy = fx.events("project")[0]["payload"]["digest"].as_str().unwrap().to_owned();
        let d = Draft { kind: "task-rejected".into(), actor, policy, instance: "SHA256:fixture".into(), say, payload };
        s.append(&stream, head.as_ref(), &d, Some(&fx.signer())).unwrap();
        drop(s);
        let doc = fx.run(&["check", "--json"]).doc();
        let codes: Vec<&Value> = doc["findings"].as_array().unwrap().iter().map(|f| &f["code"]).collect();
        assert!(codes.contains(&&json!("RECORD_TRANSITION_INVALID")), "{label}: {doc}");
    }
}
