//! The write commands' refusals that need the feature `fault-injection` (core-specification 11.2 rules 4 and 8; part
//! 7d3a): a stream's head moved while the command ran (`RECORD_HEAD_MOVED`, REQ-REG-5), the branch moved under the
//! record commit (`RECORD_COMMIT_RACE`, REQ-REG-10), a replay that fails before the commit (PD-46), and the lock held
//! until the last cache is written (REQ-REG-5). The binary pauses at a named step while the test acts, as another
//! process would. Each refusal is checked in both forms, with its reason and next step, against a snapshot taken
//! after the test's last injection, with the person's staged and unstaged edits in place.

mod common;

use common::*;
use fabrewk_core::cli;
use fabrewk_core::record::{Draft, encode, sha256_hex};
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

const SAY: &str = "Lo adopto: las rondas alcanzan para esta etapa.";

/// Starts the binary in the fixture, paused at `step`; gives the child once it waits there.
fn paused(fx: &Fx, args: &[&str], step: &str, dir: &Path) -> Child {
    paused_env(fx, args, step, dir, &[])
}

/// [`paused`] with more of the binary's environment; a command that never reaches `step` fails the test in 30 s.
fn paused_env(fx: &Fx, args: &[&str], step: &str, dir: &Path, env: &[(&str, &str)]) -> Child {
    let mut c = Command::new(env!("CARGO_BIN_EXE_fabrewk"));
    isolate(&mut c, &fx.home, fx.path.as_deref());
    c.env("FABREWK_FAULT_PAUSE", format!("{step}:{}", dir.display())).envs(env.iter().copied());
    let mut child = c.current_dir(&fx.root).args(args).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    for _ in 0..600 {
        if dir.join(format!("{step}.paused")).exists() {
            return child;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    let _ = child.kill();
    let _ = child.wait();
    panic!("{args:?} never paused at {step}");
}

fn resume(child: Child, step: &str, dir: &Path) -> Ran {
    fs::write(dir.join(format!("{step}.resume")), b"").unwrap();
    let out = child.wait_with_output().unwrap();
    Ran {
        status: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8(out.stdout).unwrap(),
        stderr: String::from_utf8(out.stderr).unwrap(),
    }
}

fn files(dir: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect();
    out.sort();
    out
}

/// A fixture whose machine key exists, made by an adoption that ran to its end, whose project file has another
/// change to adopt, committed, and with a person's staged and unstaged edits.
fn ready(name: &str) -> Fx {
    let fx = Fx::new(name, "published");
    fx.edit_policy(&(policy_text(&fx.keys) + "\n[limits]\nminutes = 70\n"));
    assert_eq!(fx.run(&["adopt", "--say", SAY]).status, 0);
    fx.edit_policy(&(policy_text(&fx.keys) + "\n[limits]\nminutes = 60\n"));
    fs::write(fx.root.join("README.md"), "staged\n").unwrap();
    fx.git(&["add", "README.md"]);
    fs::write(fx.root.join("README.md"), "unstaged\n").unwrap();
    fx
}

/// The refusal's contract in one form: exit 1, the code with the catalogue's reason and next step in `es` (the
/// person's), or the code and the same words in the document.
fn says(ran: &Ran, code: &str, json: bool) {
    let (reason, next) =
        (cli::say("es", &format!("code.{code}.reason"), &[]), cli::say("es", &format!("code.{code}.next"), &[]));
    assert_eq!(ran.status, 1, "{ran:?}");
    if json {
        let doc = ran.doc();
        let e = doc["refusals"].as_array().unwrap().iter().find(|e| e["code"] == code).cloned();
        let e = e.unwrap_or_else(|| panic!("{code}: {doc}"));
        assert_eq!((e["reason"].as_str(), e["next"].as_str()), (Some(reason.as_str()), Some(next.as_str())));
    } else {
        whole(&ran.all());
        assert!(ran.stderr.contains(&format!("{code}: {reason}")) && ran.stderr.contains(&next), "{ran:?}");
    }
}

fn args(json: bool) -> Vec<&'static str> {
    let mut a = vec!["adopt", "--say", SAY];
    a.extend(json.then_some("--json"));
    a
}

#[test]
fn refusal_record_head_moved() {
    for json in [false, true] {
        let fx = ready(&format!("head-moved-{json}"));
        let dir = tmp("head-moved-steps");
        let child = paused(&fx, &args(json), "before-append", &dir);
        // Another process writes in the project stream without the lock.
        let stream = fx.domain.join("data/events/project");
        let last = files(&stream).into_iter().rfind(|f| f.extension().is_some_and(|e| e == "json")).unwrap();
        let bytes = fs::read(&last).unwrap();
        let prev: Value = serde_json::from_slice(&bytes).unwrap();
        let draft = Draft {
            kind: "installed".into(),
            actor: json!({"kind": "fabrewk"}),
            policy: prev["policy"].as_str().unwrap().to_owned(),
            instance: "SHA256:other".into(),
            say: None,
            payload: json!({}),
        };
        let seq = prev["seq"].as_u64().unwrap() + 1;
        let event = encode(&draft.build("project", seq, Some(&sha256_hex(&bytes)))).unwrap();
        fs::write(stream.join(format!("{seq:06}-{}.json", &sha256_hex(&event)[..16])), &event).unwrap();
        let before = fx.snapshot();
        let ran = resume(child, "before-append", &dir);
        assert_eq!(fx.snapshot(), before, "nothing written after the head moved");
        says(&ran, "RECORD_HEAD_MOVED", json);
    }
}

#[test]
fn refusal_record_commit_race() {
    for json in [false, true] {
        let fx = ready(&format!("race-{json}"));
        let head = fx.git(&["rev-parse", "HEAD"]);
        // The mark as the command found it: the race does not change it, so neither may the refusal (P7D3A-10).
        let mark = fs::read(fx.domain.join("local/cache/heads.json")).unwrap();
        let dir = tmp("race-steps");
        let child = paused(&fx, &args(json), "before-commit", &dir);
        // Someone commits, on the record branch, the very file the record commit holds, and only it.
        let new = files(&fx.domain.join("data/events/project")).pop().unwrap();
        let new = new.with_extension("").with_extension("json");
        fx.git(&["add", "--", new.to_str().unwrap()]);
        fx.git(&["commit", "-q", "-m", "someone", "--", new.to_str().unwrap()]);
        let theirs = fx.git(&["rev-parse", "HEAD"]);
        // Git's object store gains the commit Fabrewk could not place, unreachable; nothing else may change.
        // What Git printed for the compare-and-swap it refused is kept, in the evidence the refusal names (P9G-15).
        let (objects, store) = (fx.root.join(".git/objects"), fx.domain.join("local/evidence"));
        let snapshot = || {
            let mut s = fx.snapshot();
            s.retain(|p, _| !p.starts_with(&objects) && !p.starts_with(&store));
            s
        };
        let before = snapshot();
        let ran = resume(child, "before-commit", &dir);
        says(&ran, "RECORD_COMMIT_RACE", json);
        assert!(ran.all().contains("Git's output is kept in"), "{ran:?}");
        assert_eq!(fs::read_dir(&store).unwrap().count(), 1);
        // Nothing changed after the race: the branch is theirs, the event and the person's edits stay as they were.
        assert_eq!(snapshot(), before);
        assert_eq!((fx.git(&["rev-parse", "HEAD"]), fx.git(&["rev-parse", "HEAD~1"])), (theirs, head));
        assert_eq!(fx.git(&["show", ":README.md"]), "staged");
        assert!(new.exists());
        assert_eq!(fs::read(fx.domain.join("local/cache/heads.json")).unwrap(), mark);
    }
}

#[test]
fn pd_46_a_replay_that_fails_before_the_commit_commits_nothing() {
    for json in [false, true] {
        let fx = ready(&format!("pd-46-{json}"));
        let head = fx.git(&["rev-parse", "HEAD"]);
        let events = fx.domain.join("data/events/project");
        let (count, before) = (files(&events).len(), fx.snapshot());
        let argv: Vec<std::ffi::OsString> = args(json).iter().map(Into::into).collect();
        let ran = fx.run_os(&argv, &[("FABREWK_FAULT_FAIL", std::ffi::OsStr::new("replay-before-commit"))]);
        says(&ran, "RECORD_BROKEN", json);
        assert!(ran.all().contains("fault injection"), "{ran:?}");
        assert_eq!(fx.git(&["rev-parse", "HEAD"]), head, "nothing committed");
        // Only the event and its signature were added; every file there before, the index with the person's staged
        // edit among them, is as it was.
        let mut after = fx.snapshot();
        let added: Vec<PathBuf> = after.keys().filter(|k| !before.contains_key(*k)).cloned().collect();
        assert_eq!(added.len(), 2, "{added:?}");
        assert!(added.iter().all(|p| p.starts_with(&events)), "{added:?}");
        after.retain(|k, _| before.contains_key(k));
        assert_eq!(after, before);
        assert_eq!(fx.git(&["show", ":README.md"]), "staged");
        assert_eq!(files(&events).len(), count + 2);
        assert_eq!(fs::read_to_string(fx.root.join("README.md")).unwrap(), "unstaged\n");
        let doc = fx.run(&["check", "--json"]).doc();
        assert!(doc["findings"].as_array().unwrap().iter().any(|f| f["code"] == "RECORD_UNCOMMITTED"), "{doc}");
        // The next write command commits them first, in a record commit of their own.
        fx.edit_policy(&(policy_text(&fx.keys) + "\n[limits]\nminutes = 50\n"));
        assert_eq!(fx.run(&["adopt", "--say", SAY]).status, 0);
        let first = fx.git(&["log", "-1", "--format=%B", "HEAD~1"]);
        assert_eq!(first.matches("Fabrewk-Event: ").count(), 1, "the leftover in a commit of its own: {first}");
        assert_eq!(fx.run(&["check"]).status, 0, "{:?}", fx.run(&["check"]));
    }
}

#[test]
fn req_reg_5_the_lock_is_held_until_the_last_cache_is_written() {
    let fx = ready("lock-to-the-end");
    let dir = tmp("lock-steps");
    let child = paused(&fx, &["adopt", "--say", SAY], "before-caches", &dir);
    let lock = fs::File::options().write(true).open(fx.domain.join("local/lock")).unwrap();
    assert!(matches!(lock.try_lock(), Err(fs::TryLockError::WouldBlock)), "the lock is free before the caches");
    let ran = resume(child, "before-caches", &dir);
    assert_eq!(ran.status, 0, "{ran:?}");
    lock.try_lock().unwrap();
}

#[test]
fn a_rebuilt_record_commit_never_stands_on_a_rollback() {
    // While the command waits to commit, someone moves the branch to a commit that drops an older event, leaving it
    // in the working tree (P7D3A-16): the commit is not rebuilt on that head.
    let fx = ready("rebuilt-on-rollback");
    let dir = tmp("rollback-steps");
    let child = paused(&fx, &args(true), "before-commit", &dir);
    let oldest = files(&fx.domain.join("data/events/project")).remove(0);
    let rel = oldest.strip_prefix(&fx.root).unwrap().display().to_string();
    let script = format!(
        "export GIT_INDEX_FILE=\"$PWD/.git/drop-index\"; git read-tree HEAD && git update-index --force-remove '{rel}' && \
         c=$(git commit-tree -p HEAD -m drop $(git write-tree)) && git update-ref refs/heads/main $c && rm \"$GIT_INDEX_FILE\""
    );
    fx.program("sh", &["-c", &script]);
    let theirs = fx.git(&["rev-parse", "HEAD"]);
    let ran = resume(child, "before-commit", &dir);
    assert_eq!(ran.status, 1, "{ran:?}");
    assert_eq!(ran.doc()["refusals"][0]["code"], "RECORD_ROLLBACK", "{ran:?}");
    assert_eq!(fx.git(&["rev-parse", "HEAD"]), theirs, "no record commit on it");
}

#[test]
fn refusal_record_id_collision_at_propose() {
    // A nonce chosen by the test gives every draw one identifier (P7D3A-15): the second proposal of the same stage, and
    // two equal tasks in one file, are refused with nothing written.
    let fx = Fx::new("id-collision", "published");
    let task = json!({
        "lang": "en", "goal": "Make it.", "requirements": [{"text": "It builds.", "criterion": "cargo build passes"}],
        "base": "0".repeat(40), "dependencies": [], "allowed": ["src/**"],
        "profiles": {"build": "build", "review": "review"}, "checks": [],
        "limits": {"rounds": 3, "dispatches": 6, "minutes": 120}, "delegationLanguage": "en", "settings": {},
    });
    let stage = json!({"lang": "en", "goal": "Build it.", "budget": {"dispatches": 6, "minutes": 120}, "checks": {}, "settings": {}});
    let nonce = [("FABREWK_FAULT_NONCE", std::ffi::OsStr::new("0123456789abcdef0123456789abcdef"))];
    let run = |name: &str, plan: Value| {
        fs::write(fx.root.join(name), serde_json::to_vec(&plan).unwrap()).unwrap();
        let argv: Vec<std::ffi::OsString> = ["propose", name, "--json"].iter().map(Into::into).collect();
        fx.run_os(&argv, &nonce)
    };
    assert_eq!(run("one.json", json!({"stage": stage, "tasks": [task]})).status, 0);
    fx.commit("one");
    for (name, plan) in [
        ("again.json", json!({"stage": stage, "tasks": [task]})),
        (
            "twice.json",
            json!({"stage": {"lang": "en", "goal": "Other.", "budget": {"dispatches": 6, "minutes": 120}, "checks": {}, "settings": {}}, "tasks": [task, task]}),
        ),
    ] {
        fs::write(fx.root.join(name), serde_json::to_vec(&plan).unwrap()).unwrap();
        let before = fx.snapshot();
        let ran = run(name, plan);
        assert_eq!(fx.snapshot(), before, "{name}");
        assert_eq!(
            (ran.status, ran.doc()["refusals"][0]["code"].as_str()),
            (1, Some("RECORD_ID_COLLISION")),
            "{ran:?}"
        );
    }
    assert_eq!(fx.run(&["check"]).status, 0);
}

#[test]
fn a_rebuilt_record_commit_never_stands_on_a_rewound_branch() {
    // While the command waits to commit, someone moves the branch back to an ancestor that lacks an event already
    // committed, the working tree and index unchanged (P7D3A-19, PD-01 (b)): the commit is not rebuilt there.
    let fx = ready("rewound");
    let ancestor = fx.git(&["rev-parse", "HEAD~2"]);
    let dir = tmp("rewound-steps");
    let child = paused(&fx, &args(true), "before-commit", &dir);
    fx.git(&["update-ref", "refs/heads/main", &ancestor]);
    let ran = resume(child, "before-commit", &dir);
    assert_eq!(ran.status, 1, "{ran:?}");
    assert_eq!(ran.doc()["refusals"][0]["code"], "RECORD_ROLLBACK", "{ran:?}");
    assert_eq!(fx.git(&["rev-parse", "HEAD"]), ancestor, "no record commit on it");
}

#[test]
fn a_command_plans_from_the_record_as_it_is_once_it_holds_the_lock() {
    // Between the command's first replay and its lock, the maestro closes the stage (written by the library, which
    // takes the lock and leaves it): the command plans again from the record under the lock and refuses.
    let fx = Fx::new("refresh", "published");
    let (stage, _) = fx.stage_and_task();
    fx.commit("stage");
    let plan = json!({"stage": stage, "tasks": [{
        "lang": "en", "goal": "More.", "requirements": [{"text": "It builds.", "criterion": "cargo build passes"}],
        "base": "0".repeat(40), "dependencies": [], "allowed": ["src/**"],
        "profiles": {"build": "build", "review": "review"}, "checks": [],
        "limits": {"rounds": 3, "dispatches": 6, "minutes": 120}, "delegationLanguage": "en", "settings": {},
    }]});
    fs::write(fx.root.join("more.json"), serde_json::to_vec(&plan).unwrap()).unwrap();
    let dir = tmp("refresh-steps");
    let child = paused(&fx, &["propose", "more.json", "--json"], "before-lock", &dir);
    close_stage(&fx, &stage);
    let ran = resume(child, "before-lock", &dir);
    assert_eq!(ran.status, 1, "{ran:?}");
    assert_eq!(ran.doc()["refusals"][0]["code"], "STAGE_CLOSED", "{ran:?}");
}

/// The maestro authorizes the stage `stage` (it binds no task) and closes it, as phase 8's commands will record it.
fn close_stage(fx: &Fx, stage: &str) {
    use fabrewk_core::record::{KeySigner, Say, Session};
    let signer = KeySigner(fx.keys.join("xel"));
    let mut s = Session::begin(&fx.domain, None, &fabrewk_core::identity::PolicyVerifier).unwrap();
    let stream = format!("stage-{stage}");
    let head = s.head(&stream).unwrap().unwrap();
    let policy = fx.events("project")[0]["payload"]["digest"].as_str().unwrap().to_owned();
    let draft = |kind: &str, payload: Value| Draft {
        kind: kind.into(),
        actor: json!({"kind": "person", "alias": "xel", "role": "maestro"}),
        policy: policy.clone(),
        instance: "SHA256:fixture".into(),
        say: Some(Say { text: "Lo decido.".into(), lang: "es".into() }),
        payload,
    };
    let mut bytes = encode(&fx.events(&stream)[0]["payload"]).unwrap();
    bytes.pop();
    let head_json = json!({"seq": 1, "digest": head.digest});
    let payload = json!({"stage": stage, "digest": sha256_hex(&bytes), "head": head_json, "tasks": [], "shown": "0".repeat(64), "widened": []});
    let h = s.append(&stream, Some(&head), &draft("stage-authorized", payload), Some(&signer)).unwrap();
    s.append(
        &stream,
        Some(&h),
        &draft("stage-closed", json!({"stage": stage, "head": {"seq": h.seq, "digest": h.digest}, "sweeps": []})),
        Some(&signer),
    )
    .unwrap();
    s.finish(&fabrewk_core::identity::PolicyVerifier, &[]).unwrap();
}

/// What `status` shows of `task` for an authorization: its digest.
fn shown_digest(fx: &Fx, task: &str) -> String {
    fx.run(&["status", task, "--json"]).doc()["shown"]["digest"].as_str().unwrap().to_owned()
}

#[test]
fn refusal_task_changed_when_the_language_changes_before_the_lock() {
    // The rendering bound is in the person's language as the record under the lock and their file give it, not as
    // the command read them first (P7D3B-1): a policy adopted, or the personal file changed, between the first replay
    // and the lock is `TASK_CHANGED`, and nothing is written.
    let say = "Lo autorizo: el alcance es el que leí.";
    for personal in [false, true] {
        let fx = Fx::new(if personal { "language-personal" } else { "language-policy" }, "published");
        if !personal {
            fs::remove_file(fx.domain.join("xel/settings.toml")).unwrap();
            fx.commit("the policy's language");
        }
        let (_, task) = fx.stage_and_task();
        fx.commit("stage");
        let seen = shown_digest(&fx, &task);
        let dir = tmp(if personal { "language-personal-steps" } else { "language-policy-steps" });
        let child = paused(&fx, &["authorize", &task, "--seen", &seen, "--say", say, "--json"], "before-lock", &dir);
        if personal {
            fs::write(fx.domain.join("xel/settings.toml"), PERSONAL.replace("es-MX", "en")).unwrap();
        } else {
            fx.edit_policy(&policy_text(&fx.keys).replace("Language = \"en\"", "Language = \"es\""));
            assert_eq!(fx.run(&["adopt", "--say", SAY]).status, 0);
        }
        assert_ne!(shown_digest(&fx, &task), seen);
        let ran = resume(child, "before-lock", &dir);
        // The refusal is worded as the command first read the language; its code is what is held.
        assert_eq!((ran.status, &ran.doc()["refusals"][0]["code"]), (1, &json!("TASK_CHANGED")), "{ran:?}");
        assert_eq!(fx.events(&format!("task-{task}")).len(), 1, "{ran:?}");
    }
}

/// The fixture's answer to its fermento, one recommendation for each point as it stands.
fn answer_for(fx: &Fx) -> String {
    let (text, parts) = fabrewk_core::fermento::open(&fx.domain, "xel").unwrap();
    let rows: Vec<Value> = fabrewk_core::fermento::points(&text[..parts.input_end])
        .iter()
        .map(|p| {
            let rec = json!({"point": "About", "clarify": "Say it.", "destination": "alambique"});
            json!({"number": p.number, "digest": p.digest, "recommendation": rec})
        })
        .collect();
    let doc = json!({"agent": {"tool": "claude-code", "provider": "anthropic", "model": "m"}, "points": rows});
    fs::write(fx.root.join("answer.json"), serde_json::to_vec(&doc).unwrap()).unwrap();
    text
}

#[test]
fn refusal_input_changed() {
    for json in [false, true] {
        let fx = Fx::new(&format!("input-changed-{json}"), "published");
        let text = answer_for(&fx);
        let dir = tmp("input-changed-steps");
        let mut argv = vec!["sweep", "xel", "--answer", "answer.json"];
        argv.extend(json.then_some("--json"));
        let child = paused(&fx, &argv, "before-view", &dir);
        // The maintainer saves an edit of the input while the sweep waits: the sweep writes nothing, theirs stays.
        let edited = text.replacen("First point.", "First point, edited.", 1);
        fs::write(fx.domain.join("xel/fermento.md"), &edited).unwrap();
        // The key a first write prepares waits in the private folder under a temporary name, and a refusal removes it:
        // the folder is left out of the comparison and must not exist after.
        let outside = |mut s: std::collections::BTreeMap<PathBuf, (u32, Vec<u8>)>| {
            s.retain(|k, _| !k.starts_with(&fx.home));
            s
        };
        let before = outside(fx.snapshot());
        let ran = resume(child, "before-view", &dir);
        says(&ran, "INPUT_CHANGED", json);
        assert_eq!(outside(fx.snapshot()), before, "nothing written after the input changed");
        assert!(!fx.folder().exists(), "the private folder was made");
        assert!(fx.events("fermento-xel").is_empty());
    }
}

#[test]
fn refusal_input_changed_after_the_events_leaves_them_and_the_maintainers_edit() {
    let fx = Fx::new("input-changed-late", "published");
    let text = answer_for(&fx);
    let dir = tmp("input-changed-late-steps");
    let argv = ["sweep", "xel", "--answer", "answer.json", "--json"];
    let child = paused(&fx, &argv, "before-append", &dir);
    let edited = text.replacen("First point.", "First point, edited.", 1);
    fs::write(fx.domain.join("xel/fermento.md"), &edited).unwrap();
    let ran = resume(child, "before-append", &dir);
    says(&ran, "INPUT_CHANGED", true);
    let detail = ran.doc()["refusals"][0]["detail"].as_str().unwrap().to_owned();
    assert!(detail.contains("events are written") && detail.contains("next sweep"), "{detail}");
    // The sweep's event stays; the answer was not written over the maintainer's edit.
    assert_eq!(fx.events("fermento-xel").len(), 1);
    assert_eq!(fs::read_to_string(fx.domain.join("xel/fermento.md")).unwrap(), edited);
    // Once they commit, the next sweep commits the event left, answers the edited input and checks clean.
    fx.commit("edited");
    answer_for(&fx);
    assert_eq!(fx.run(&["sweep", "xel", "--answer", "answer.json"]).status, 0);
    assert_eq!(fx.events("fermento-xel").len(), 2);
    assert_eq!(fx.run(&["check"]).status, 0);
}

/// A proposal to replace point 1.1 of the fixture's alambique, swept; gives its identifier.
fn alambique_proposal(fx: &Fx) -> String {
    let text = fs::read_to_string(fx.domain.join("ALAMBIQUE.md")).unwrap();
    let units = vec![json!({"kind": "point", "n": 1, "m": 1, "text": "**1.1. A:** new."})];
    let result = sha256_hex(fabrewk_core::alambique::apply(&text, &units).unwrap().as_bytes());
    let edit = json!({"target": "alambique", "base": sha256_hex(text.as_bytes()), "keys": [], "result": result, "units": units});
    answer_for(fx);
    let mut doc: Value = serde_json::from_slice(&fs::read(fx.root.join("answer.json")).unwrap()).unwrap();
    doc["points"][0]["edit"] = edit;
    fs::write(fx.root.join("answer.json"), serde_json::to_vec(&doc).unwrap()).unwrap();
    let ran = fx.run(&["sweep", "xel", "--answer", "answer.json", "--json"]);
    assert_eq!(ran.status, 0, "{ran:?}");
    ran.doc()["proposals"][0].as_str().unwrap().to_owned()
}

#[test]
fn refusal_alambique_changed_while_the_command_ran_writes_nothing() {
    for json in [false, true] {
        let fx = Fx::new(&format!("alambique-moved-{json}"), "published");
        fx.alambique("# A\n\n## 1. S\n\n**1.1. A:** first.\n", "published");
        let id = alambique_proposal(&fx);
        let dir = tmp("alambique-moved-steps");
        let mut argv = vec!["approve", &id, "--say", SAY];
        argv.extend(json.then_some("--json"));
        let child = paused(&fx, &argv, "before-alambique", &dir);
        // The maestro saves an edit while the command waits: it writes nothing and theirs stays.
        let edited = "# A\n\n## 1. S\n\n**1.1. A:** first, edited.\n";
        fs::write(fx.domain.join("ALAMBIQUE.md"), edited).unwrap();
        let before = fx.snapshot();
        let ran = resume(child, "before-alambique", &dir);
        says(&ran, "ALAMBIQUE_CHANGED", json);
        assert_eq!(fx.snapshot(), before, "nothing written after the alambique changed");
        assert_eq!(fx.events("alambique").len(), 1);
        assert_eq!(fs::read_to_string(fx.domain.join("ALAMBIQUE.md")).unwrap(), edited);
    }
}

const SMALL: &str = "# A\n\n## 1. S\n\n**1.1. A:** first.\n";

#[test]
fn refusal_alambique_changed_in_the_index_while_the_command_ran_writes_nothing() {
    // P8B-1: the working file is restored to the drafted text, but the person staged another while the command waited.
    let fx = Fx::new("alambique-staged", "published");
    fx.alambique(SMALL, "published");
    let id = alambique_proposal(&fx);
    let dir = tmp("alambique-staged-steps");
    let child = paused(&fx, &["approve", &id, "--say", SAY, "--json"], "before-alambique", &dir);
    let edited = SMALL.replace("first.", "first, staged.");
    fs::write(fx.domain.join("ALAMBIQUE.md"), &edited).unwrap();
    fx.git(&["add", ".brewstillery/ALAMBIQUE.md"]);
    fs::write(fx.domain.join("ALAMBIQUE.md"), SMALL).unwrap();
    let (head, before) = (fx.git(&["rev-parse", "HEAD"]), fx.snapshot());
    let ran = resume(child, "before-alambique", &dir);
    says(&ran, "ALAMBIQUE_CHANGED", true);
    assert_eq!((fx.git(&["rev-parse", "HEAD"]), fx.snapshot()), (head, before));
    assert!(fx.git(&["show", ":.brewstillery/ALAMBIQUE.md"]).contains("staged"));
}

#[test]
fn refusal_alambique_changed_on_a_fresh_machine_leaves_no_private_folder() {
    // P8B-3: nothing was written before the refusal, the machine's identity included.
    let fx = Fx::new("alambique-fresh", "published");
    fx.alambique(SMALL, "published");
    fs::write(fx.domain.join("ALAMBIQUE.md"), SMALL.replace("first.", "first, edited.")).unwrap();
    fx.commit("edited");
    assert!(!fx.folder().exists());
    let dir = tmp("alambique-fresh-steps");
    let child = paused(&fx, &["approve", "alambique", "--say", SAY, "--json"], "before-alambique", &dir);
    fs::write(fx.domain.join("ALAMBIQUE.md"), SMALL.replace("first.", "first, edited again.")).unwrap();
    let ran = resume(child, "before-alambique", &dir);
    says(&ran, "ALAMBIQUE_CHANGED", true);
    assert!(!fx.folder().exists(), "the private folder was made");
    assert_eq!(fx.events("alambique").len(), 1);
}

#[test]
fn pd_46_a_replay_that_fails_before_an_approval_commits_nothing() {
    // P8B-5: not even the alambique's own commit, and the alambique is not written.
    for json in [false, true] {
        let fx = Fx::new(&format!("alambique-replay-{json}"), "published");
        fx.alambique(SMALL, "published");
        let id = alambique_proposal(&fx);
        let head = fx.git(&["rev-parse", "HEAD"]);
        let mut argv = vec!["approve", id.as_str(), "--say", SAY];
        argv.extend(json.then_some("--json"));
        let argv: Vec<std::ffi::OsString> = argv.iter().map(Into::into).collect();
        let ran = fx.run_os(&argv, &[("FABREWK_FAULT_FAIL", std::ffi::OsStr::new("replay-before-commit"))]);
        says(&ran, "RECORD_BROKEN", json);
        assert_eq!(fx.git(&["rev-parse", "HEAD"]), head, "nothing committed");
        assert_eq!(fs::read_to_string(fx.domain.join("ALAMBIQUE.md")).unwrap(), SMALL);
    }
}

#[test]
fn an_alambique_commit_made_is_reported_made_when_the_index_is_held_after_it() {
    for spaced in [false, true] {
        index_held(spaced);
    }
}

fn index_held(spaced: bool) {
    // P8B-4: another Git takes the index's lock once the ref moves (a hook no longer runs, S-11); the person is told
    // to bring the index to the commit, and the record is committed by the next write command.
    // A path a shell would split is not printed as a command, but said in words.
    let fx = Fx::new(if spaced { "alambique index spaced" } else { "alambique-index" }, "published");
    fx.alambique(SMALL, "published");
    let id = alambique_proposal(&fx);
    let head = fx.git(&["rev-parse", "HEAD"]);
    let dir = tmp("alambique-index-steps");
    let child = paused(&fx, &["approve", &id, "--say", SAY, "--json"], "record-moved", &dir);
    fs::write(fx.root.join(".git/index.lock"), b"").unwrap();
    let ran = resume(child, "record-moved", &dir);
    assert_eq!(ran.status, 1, "{ran:?}");
    let detail = ran.doc()["refusals"][0]["detail"].as_str().unwrap().to_owned();
    let fix = if spaced {
        "stage .brewstillery/ALAMBIQUE.md in the repository at"
    } else {
        " add .brewstillery/ALAMBIQUE.md"
    };
    assert!(detail.contains(fix) && detail.contains("events are written"), "{detail}");
    assert_ne!(fx.git(&["rev-parse", "HEAD"]), head, "the alambique's commit is made");
    fs::remove_file(fx.root.join(".git/index.lock")).unwrap();
    fx.git(&["add", ".brewstillery/ALAMBIQUE.md"]);
    assert_eq!(fx.git(&["diff", "--cached", "--name-only"]), "");
    // The events left are committed by the next write command, and the check passes.
    answer_for(&fx);
    assert_eq!(fx.run(&["sweep", "xel", "--answer", "answer.json"]).status, 0);
    assert_eq!(fx.run(&["check"]).status, 0);
}

#[test]
fn an_alambique_commit_made_in_an_own_repository_names_that_repository_in_the_fix() {
    // P8B-6: the command printed runs from where the person is, so it names the record's repository.
    let fx = Fx::new("alambique-own", "own-repository");
    let domain = fx.domain.to_str().unwrap().to_owned();
    let inner = |args: &[&str]| fx.git(&[&["-C", domain.as_str()][..], args].concat());
    inner(&["init", "-q", "-b", "main"]);
    for (k, v) in [("user.name", "Fixture"), ("user.email", "fixture@example.invalid"), ("commit.gpgsign", "false")] {
        inner(&["config", k, v]);
    }
    inner(&["config", "user.signingkey", fx.keys.join("xel.pub").to_str().unwrap()]);
    fx.alambique(SMALL, "own-repository");
    inner(&["add", "-A"]);
    inner(&["commit", "-q", "-m", "record"]);
    let id = alambique_proposal(&fx);
    // Another Git takes the index's lock once the ref moves (a hook no longer runs, S-11).
    let steps = tmp("alambique-own-steps");
    let child = paused(&fx, &["approve", &id, "--say", SAY, "--json"], "record-moved", &steps);
    fs::write(fx.domain.join(".git/index.lock"), b"").unwrap();
    let ran = resume(child, "record-moved", &steps);
    assert_eq!(ran.status, 1, "{ran:?}");
    let detail = ran.doc()["refusals"][0]["detail"].as_str().unwrap().to_owned();
    let fix = format!("run git -C {domain} add ALAMBIQUE.md;");
    assert!(detail.contains(&fix), "{detail}");
    fs::remove_file(fx.domain.join(".git/index.lock")).unwrap();
    // The command as printed, run from the product's root, brings that index to the commit.
    fx.git(&["-C", &domain, "add", "ALAMBIQUE.md"]);
    assert_eq!(inner(&["diff", "--cached", "--name-only"]), "");
}

#[test]
fn req_view_3_a_command_stopped_before_its_commit_leaves_fresh_views_and_the_next_command_commits_them() {
    let fx = ready("views-stopped");
    let dir = tmp("views-stopped-steps");
    let mut child = paused(&fx, &args(false), "before-commit", &dir);
    // The events and the views are written and not committed: killed here, as a power cut would.
    child.kill().unwrap();
    child.wait().unwrap();
    let status = fs::read_to_string(fx.domain.join("STATUS.md")).unwrap();
    assert_eq!(fx.git(&["status", "--porcelain", "--", ".brewstillery/STATUS.md"]).chars().next(), Some(' '));
    let doc = fx.run(&["check", "--json"]).doc();
    let found: Vec<&str> = doc["findings"].as_array().unwrap().iter().map(|f| f["code"].as_str().unwrap()).collect();
    assert!(found.contains(&"RECORD_UNCOMMITTED") && !found.contains(&"VIEW_STALE"), "{found:?}");
    // The next command that writes commits the leftovers first, then its own events with the views.
    fx.edit_policy(&(policy_text(&fx.keys) + "\n[limits]\nminutes = 50\n"));
    assert_eq!(fx.run(&["adopt", "--say", SAY]).status, 0);
    assert_eq!(fx.run(&["check"]).status, 0, "{:?}", fx.run(&["check"]));
    let held = fx.git(&["show", "HEAD:.brewstillery/STATUS.md"]);
    assert_eq!(held.trim_end(), fs::read_to_string(fx.domain.join("STATUS.md")).unwrap().trim_end());
    assert_ne!(held.trim_end(), status.trim_end(), "the second adoption changed what the view shows");
}

#[test]
fn p8c_7_a_lock_replaced_by_a_link_between_the_check_and_the_open_is_not_followed() {
    // The link leads to nothing (the open would create a file outside the domain) or to a file that exists.
    for (json, existing) in [(false, false), (true, false), (false, true), (true, true)] {
        let fx = ready(&format!("lock-raced-{json}-{existing}"));
        let dir = tmp(&format!("lock-raced-steps-{json}-{existing}"));
        let child = paused(&fx, &args(json), "before-lock", &dir);
        // Another process swaps the lock for a link to a file outside the domain, after the check.
        let (lock, outside) = (fx.domain.join("local/lock"), fx.root.join("outside-raced-lock"));
        fs::remove_file(&lock).unwrap();
        if existing {
            fs::write(&outside, b"x").unwrap();
        }
        std::os::unix::fs::symlink(&outside, &lock).unwrap();
        let ran = resume(child, "before-lock", &dir);
        says(&ran, "RECORD_BROKEN", json);
        assert_eq!(outside.exists(), existing);
        if existing {
            assert_eq!(fs::read(&outside).unwrap(), b"x");
        }
    }
}

#[test]
fn a_record_commit_made_is_reported_made_when_the_index_is_held_after_it() {
    // Another Git takes the index's lock once the ref is updated (P7D3A-21; a hook no longer runs, S-11): the commit
    // is made, the command says so, and the next write command brings the index to it.
    let fx = ready("late-index");
    let dir = tmp("late-index-steps");
    let head = fx.git(&["rev-parse", "HEAD"]);
    let child = paused(&fx, &["adopt", "--say", SAY], "record-moved", &dir);
    fs::write(fx.root.join(".git/index.lock"), b"").unwrap();
    let ran = resume(child, "record-moved", &dir);
    assert_eq!(ran.status, 0, "{ran:?}");
    // The commit is made, and the words name what Git printed for the index it could not update (D-088).
    let detail = cli::say("es", "write.recovered", &[("detail", "git update-index -z failed with exit status 128")]);
    assert!(ran.stdout.contains(&format!("{detail}; Git's output is kept in")), "{ran:?}");
    assert_eq!(fx.git(&["rev-parse", "HEAD~1"]), head);
    fs::remove_file(fx.root.join(".git/index.lock")).unwrap();
    assert_eq!(fx.run(&["check"]).status, 0, "{:?}", fx.run(&["check"]));
    let plan = json!({
        "stage": {"lang": "en", "goal": "Build it.", "budget": {"dispatches": 6, "minutes": 120}, "checks": {}, "settings": {}},
        "tasks": [{
            "lang": "en", "goal": "Make it.", "requirements": [{"text": "It builds.", "criterion": "cargo build passes"}],
            "base": "0".repeat(40), "dependencies": [], "allowed": ["src/**"],
            "profiles": {"build": "build", "review": "review"}, "checks": [],
            "limits": {"rounds": 3, "dispatches": 6, "minutes": 120}, "delegationLanguage": "en", "settings": {},
        }],
    });
    fs::write(fx.root.join("plan.json"), serde_json::to_vec(&plan).unwrap()).unwrap();
    let proposed = fx.run(&["propose", "plan.json"]);
    assert_eq!(proposed.status, 0, "{proposed:?}");
    // The person's staged edit stays; the record's files are in the index as the branch holds them.
    assert_eq!(fx.git(&["diff", "--cached", "--name-only"]), "README.md", "the index follows the record");
}

// ---- A builder's time (part 9e; PD-28, REQ-TASK-5's minutes, S-5) ---------------------------------------------------

/// A task proposed on the fixture's head with `limits`, authorized through the binary; gives its identifier.
fn authorized_with(fx: &Fx, limits: Value) -> String {
    let head = fx.git(&["rev-parse", "HEAD"]);
    let task = json!({
        "lang": "en", "goal": "Make it.", "requirements": [{"text": "It builds.", "criterion": "it builds"}],
        "base": head, "dependencies": [], "allowed": ["src/**"], "profiles": {"build": "build", "review": "review"},
        "checks": [], "limits": limits, "delegationLanguage": "en", "settings": {"profiles.build.minutes": 2},
    });
    let stage = json!({"lang": "en", "goal": "Build it.", "budget": {"dispatches": 6, "minutes": 120}, "checks": {}, "settings": {}});
    fs::write(fx.root.join("plan.json"), serde_json::to_vec(&json!({"stage": stage, "tasks": [task]})).unwrap())
        .unwrap();
    let ran = fx.run(&["propose", "plan.json", "--json"]);
    fs::remove_file(fx.root.join("plan.json")).unwrap();
    let id = ran.doc()["tasks"][0]["id"].as_str().unwrap().to_owned();
    let digest = fx.run(&["status", &id, "--json"]).doc()["shown"]["digest"].as_str().unwrap().to_owned();
    // The task's layer raises the profile's minutes: a widening, named.
    let ran = fx.run(&["authorize", &id, "--seen", &digest, "--say", SAY, "--widen", "profiles.build.minutes"]);
    assert_eq!(ran.status, 0, "{ran:?}");
    id
}

/// A builder that prints, then sleeps past its time: its cell is ended at the profile's minutes (here two seconds a
/// minute, `FABREWK_FAULT_MINUTE`), what it printed is kept as evidence (PD-28), and the task halts `TOOL_FAILED`; when
/// the task's own minutes are what ended it, `BUDGET_EXHAUSTED`.
#[test]
fn pd_28_a_builder_that_times_out_keeps_its_output_and_halts() {
    let fx = Fx::builder("timeout", "published", "echo printed-before; echo said-before >&2; sleep 30\n");
    for (limits, code, why) in [
        (json!({"rounds": 3, "dispatches": 6, "minutes": 120}), "TOOL_FAILED", "ended at its time limit"),
        (json!({"rounds": 3, "dispatches": 6, "minutes": 1}), "BUDGET_EXHAUSTED", "minutes are spent"),
    ] {
        let id = authorized_with(&fx, limits);
        let ran = fx.run_env(&["run", &id, "--say", SAY], &[("FABREWK_FAULT_MINUTE", "2")]);
        assert_eq!(ran.status, 4, "{ran:?}");
        let events = fx.events(&format!("task-{id}"));
        let (end, halt) = (&events[events.len() - 2]["payload"], &events[events.len() - 1]["payload"]);
        assert_eq!((end["outcome"].as_str(), halt["code"].as_str()), (Some("failure"), Some(code)), "{events:?}");
        assert!(halt["reason"].as_str().unwrap().contains(why), "{halt}");
        let seconds = end["usage"]["seconds"].as_u64().unwrap();
        assert!((1..10).contains(&seconds), "{seconds}");
        let attempt = end["attempt"].as_u64().unwrap();
        let evidence = fx.domain.join(format!("local/evidence/attempt-{id}-{attempt}"));
        assert_eq!(fs::read_to_string(evidence.join("stdout")).unwrap(), "printed-before\n");
        assert_eq!(fs::read_to_string(evidence.join("stderr")).unwrap(), "said-before\n");
        assert_eq!(end["evidence"]["stdout"], sha256_hex(b"printed-before\n"));
    }
}

/// An extension past its minutes (`extensions.<name>.minutes`, D-094; here two seconds a minute) is ended, what it
/// printed is kept as evidence, and none of its requests is applied: `TOOL_FAILED`.
#[test]
fn d_094_an_extension_past_its_minutes_is_ended_and_applies_nothing() {
    use std::os::unix::fs::PermissionsExt;
    let mut fx = Fx::new_with("ext-time", "unpublished", "\n[extensions.slow]\nminutes = 1\n");
    let bin = fx.root.parent().unwrap().join("ext-bin");
    fs::create_dir_all(&bin).unwrap();
    let request = r#"{"protocol":1,"request":"note","text":"too late"}"#;
    fs::write(bin.join("fabrewk-slow"), format!("#!/bin/sh\necho '{request}'\necho said >&2\nsleep 30\n")).unwrap();
    fs::set_permissions(bin.join("fabrewk-slow"), fs::Permissions::from_mode(0o755)).unwrap();
    let path =
        std::env::join_paths(std::iter::once(bin).chain(std::env::split_paths(&std::env::var_os("PATH").unwrap())));
    fx.path = Some(path.unwrap());
    let begun = std::time::Instant::now();
    let ran = fx.run_env(&["slow"], &[("FABREWK_FAULT_MINUTE", "2")]);
    assert!(begun.elapsed() < std::time::Duration::from_secs(15), "{:?}", begun.elapsed());
    assert_eq!(ran.status, 1, "{ran:?}");
    assert!(ran.stderr.contains("TOOL_FAILED") && ran.stderr.contains("ended at its time limit"), "{ran:?}");
    assert!(ran.stderr.starts_with("said\n"), "its standard error reaches the person: {ran:?}");
    assert!(fx.events("ext-slow").is_empty());
    let store = fx.domain.join("local/evidence");
    let kept = fs::read_dir(&store)
        .unwrap()
        .map(|e| e.unwrap().path())
        .find(|p| p.file_name().unwrap().to_str().unwrap().starts_with("ext-slow-"));
    assert_eq!(fs::read_to_string(kept.unwrap().join("stdout")).unwrap(), format!("{request}\n"));
}

/// P9H-2: two `propose` requests of one extension's output never draw one identifier: with every nonce the same, the
/// second's stage collides with the first's after eight draws, `RECORD_ID_COLLISION`, and nothing is written.
#[test]
fn p9h_2_an_extensions_proposals_share_the_identifiers_they_draw() {
    use std::os::unix::fs::PermissionsExt;
    let mut fx = Fx::new_with("ext-nonce", "unpublished", "\n[extensions.twice]\n");
    let bin = fx.root.parent().unwrap().join("ext-bin");
    fs::create_dir_all(&bin).unwrap();
    let request = json!({
        "protocol": 1, "request": "propose",
        "stage": {"lang": "en", "goal": "Index it.", "budget": {"dispatches": 6, "minutes": 120}, "checks": {}, "settings": {}},
        "tasks": [{
            "lang": "en", "goal": "Make it.", "requirements": [{"text": "It builds.", "criterion": "it builds"}],
            "base": fx.git(&["rev-parse", "HEAD"]), "dependencies": [], "allowed": ["src/**"],
            "profiles": {"build": "build", "review": "review"}, "checks": [],
            "limits": {"rounds": 3, "dispatches": 6, "minutes": 120}, "delegationLanguage": "en", "settings": {},
        }],
    });
    let line = format!("printf '%s\\n' '{}'", serde_json::to_string(&request).unwrap());
    fs::write(bin.join("fabrewk-twice"), format!("#!/bin/sh\n{line}\n{line}\n")).unwrap();
    fs::set_permissions(bin.join("fabrewk-twice"), fs::Permissions::from_mode(0o755)).unwrap();
    let path =
        std::env::join_paths(std::iter::once(bin).chain(std::env::split_paths(&std::env::var_os("PATH").unwrap())));
    fx.path = Some(path.unwrap());
    let before = fs::read_dir(fx.domain.join("data/events")).unwrap().count();
    let ran = fx.run_env(&["twice"], &[("FABREWK_FAULT_NONCE", &"0".repeat(32))]);
    assert_eq!(ran.status, 1, "{ran:?}");
    assert!(ran.stderr.contains("RECORD_ID_COLLISION") && ran.stderr.contains("line 2: "), "{ran:?}");
    assert_eq!(fs::read_dir(fx.domain.join("data/events")).unwrap().count(), before);
    // One of them alone is written.
    let one = format!("#!/bin/sh\n{line}\n");
    fs::write(fx.root.parent().unwrap().join("ext-bin/fabrewk-twice"), one).unwrap();
    let ran = fx.run_env(&["twice"], &[("FABREWK_FAULT_NONCE", &"0".repeat(32))]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(fs::read_dir(fx.domain.join("data/events")).unwrap().count(), before + 2);
}

/// Appends to `stream` an event of `kind`, a maestro's (signed) or Fabrewk's, through the library; commits it.
fn appended(fx: &Fx, stream: &str, kind: &str, payload: Value, person: bool) {
    use fabrewk_core::record::{KeySigner, Say, Session, Sign};
    let mut s = Session::begin(&fx.domain, None, &fabrewk_core::identity::PolicyVerifier).unwrap();
    let head = s.head(stream).unwrap();
    let policy = fx.events("project").last().unwrap()["policy"].as_str().unwrap().to_owned();
    let (actor, say) = match person {
        true => (
            json!({"kind": "person", "alias": "xel", "role": "maestro"}),
            Some(Say { text: "Corre.".into(), lang: "es".into() }),
        ),
        false => (json!({"kind": "fabrewk"}), None),
    };
    let key = KeySigner(fx.keys.join("xel"));
    let signer: Option<&dyn Sign> = if person { Some(&key) } else { None };
    let draft = Draft { kind: kind.into(), actor, policy, instance: "SHA256:fixture".into(), say, payload };
    s.append(stream, head.as_ref(), &draft, signer).unwrap();
    s.finish(&fabrewk_core::identity::PolicyVerifier, &[]).unwrap();
    fx.regen();
    fx.commit(kind);
}

/// REQ-RUN-5 stopped at every write (P9E-8): the binary killed at each of a recovery's writes, then the next write
/// command recovers whole, the attempt's copy removed, one end, one halt.
#[test]
fn p9e_8_a_recovery_stopped_at_any_write_is_made_again_whole() {
    for n in 1.. {
        let fx = Fx::new(&format!("recovery-{n}"), "published");
        let (_, id) = fx.stage_and_task();
        fx.commit("proposed");
        let digest = fx.run(&["status", &id, "--json"]).doc()["shown"]["digest"].as_str().unwrap().to_owned();
        assert_eq!(fx.run(&["authorize", &id, "--seen", &digest, "--say", SAY]).status, 0);
        let stream = format!("task-{id}");
        let events = fx.events(&stream);
        let auth = format!("{:06}", events.len());
        let file = fs::read_dir(fx.domain.join("data/events").join(&stream))
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| p.file_name().unwrap().to_str().unwrap().starts_with(&auth) && p.extension().unwrap() == "json")
            .unwrap();
        let at = json!({"seq": events.len(), "digest": sha256_hex(&fs::read(file).unwrap())});
        let run = json!({"task": id, "head": at, "authorization": {"stream": stream, "seq": at["seq"], "digest": at["digest"]}, "base": events[0]["payload"]["base"]});
        appended(&fx, &stream, "run-started", run, true);
        let attempt = json!({"role": "build", "profile": "build", "tool": "claude", "version": null, "model": "m", "effort": "low", "allowedSeconds": 60});
        appended(&fx, &stream, "attempt-started", attempt, false);
        let seq = fx.events(&stream).len();
        let copy = fx.domain.join(format!("local/tmp/attempt-{id}-{seq}"));
        fs::create_dir_all(copy.join("work/src")).unwrap();
        fs::write(copy.join("work/src/left"), "x").unwrap();
        let n_text = n.to_string();
        let ran = fx.run_env(&["reject", &id, "--say", SAY], &[("FABREWK_FAULT_ABORT_AT", &n_text)]);
        let whole = ran.status == 0;
        // Stopped before its rejection was written, the command is made again; after it, the next writer commits it.
        let rejected = fx.events(&stream).iter().any(|e| e["kind"] == "task-rejected");
        if !whole {
            assert_ne!(ran.status, 1, "{n}: {ran:?}");
            let again = if rejected { fx.run(&["status"]) } else { fx.run(&["reject", &id, "--say", SAY]) };
            assert_eq!(again.status, 0, "{n}: {again:?}");
        }
        let kinds: Vec<Value> = fx.events(&stream)[seq..].iter().map(|e| e["kind"].clone()).collect();
        assert_eq!(kinds, [json!("attempt-ended"), json!("task-halted"), json!("task-rejected")], "{n}");
        assert!(!copy.exists(), "{n}");
        // What a writer stopped after its events leaves for the next one is only uncommitted, or a view not regenerated,
        // and the view's temporary file a stop while it was written leaves (part 8c's; destination: part 9i's crash
        // suite).
        let doc = fx.run(&["check", "--json"]).doc();
        let left = |f: &Value| {
            ["RECORD_UNCOMMITTED", "VIEW_STALE"].contains(&f["code"].as_str().unwrap())
                || (f["code"] == "DOMAIN_UNKNOWN_ENTRY" && f["path"].as_str().unwrap().ends_with(".md.tmp"))
        };
        assert!(doc["findings"].as_array().unwrap().iter().all(left), "{n}: {doc}");
        if whole {
            break;
        }
    }
}

/// A run that started and died while another writer waited for the lock (P9E-13): that writer, reading the record
/// under the lock, recovers it before its own move, whatever it read before.
#[test]
fn p9e_13_a_writer_recovers_what_it_finds_running_under_the_lock() {
    for command in ["reject", "run"] {
        let fx = Fx::builder(&format!("late-recovery-{command}"), "published", SUCCESS);
        let (stage, ids) = {
            let (stage, a) = fx.stage_and_task();
            fx.commit("proposed");
            let (_, b) = fx.stage_and_task();
            fx.commit("proposed");
            (stage, vec![a, b])
        };
        let _ = stage;
        for id in &ids {
            let digest = fx.run(&["status", id, "--json"]).doc()["shown"]["digest"].as_str().unwrap().to_owned();
            assert_eq!(fx.run(&["authorize", id, "--seen", &digest, "--say", SAY]).status, 0);
        }
        // The writer of the second task reads a record with nothing running, then waits before the lock.
        let dir = fx.root.parent().unwrap().join(format!("pause-{command}"));
        fs::create_dir_all(&dir).unwrap();
        let child = paused(&fx, &[command, &ids[1], "--say", SAY], "before-lock", &dir);
        // Meanwhile a run of the first starts its attempt and dies.
        let stream = format!("task-{}", ids[0]);
        let events = fx.events(&stream);
        let auth = format!("{:06}", events.len());
        let file = fs::read_dir(fx.domain.join("data/events").join(&stream))
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| p.file_name().unwrap().to_str().unwrap().starts_with(&auth) && p.extension().unwrap() == "json")
            .unwrap();
        let at = json!({"seq": events.len(), "digest": sha256_hex(&fs::read(file).unwrap())});
        let run = json!({"task": ids[0], "head": at, "authorization": {"stream": stream, "seq": at["seq"], "digest": at["digest"]}, "base": events[0]["payload"]["base"]});
        appended(&fx, &stream, "run-started", run, true);
        let attempt = json!({"role": "build", "profile": "build", "tool": "claude", "version": null, "model": "m", "effort": "low", "allowedSeconds": 60});
        appended(&fx, &stream, "attempt-started", attempt, false);
        let ran = resume(child, "before-lock", &dir);
        // Recovered by that writer: its attempt ended unknown, the task halted; then its own move made (a rejection, a
        // run that verifies the task).
        let kinds: Vec<Value> = fx.events(&stream).iter().map(|e| e["kind"].clone()).collect();
        assert_eq!(kinds[kinds.len() - 2..], [json!("attempt-ended"), json!("task-halted")], "{command}: {ran:?}");
        assert_eq!(fx.events(&stream).last().unwrap()["payload"]["code"], "OUTCOME_UNKNOWN");
        assert_eq!(ran.status, 0, "{command}: {ran:?}");
    }
}

// ---- The integration's failures (part 9g; REQ-GATE-8, REQ-RUN-6, PD-11 to PD-14; D-083, D-087) ---------------------

/// The fake builder's script that writes `src/new.txt` in its copy, then succeeds.
const WRITES: &str = "mkdir -p src && printf 'new\\n' > src/new.txt\n";

/// A verified task in `published` and the arguments that accept it.
fn to_accept(name: &str) -> (Fx, String, Vec<String>) {
    let fx = Fx::builder(name, "published", &format!("{WRITES}{SUCCESS}"));
    let id = verified_task(&fx, json!({}), json!({}));
    let args = ["accept", &id, "--seen", &accepting(&fx, &id), "--say", SAY].map(str::to_owned).to_vec();
    (fx, id, args)
}

fn last_of(fx: &Fx, id: &str) -> Value {
    fx.events(&format!("task-{id}")).last().unwrap().clone()
}

/// Files with their modes and bytes, by path.
type Snapshot = Vec<(PathBuf, (u32, Vec<u8>))>;

/// What the person's repository holds outside the record: the product tree at the branch's head, the index's entries
/// outside the domain and every file of the working tree outside it.
fn product(fx: &Fx) -> (String, String, Snapshot) {
    let head = fx.git(&["rev-parse", "HEAD"]);
    let tree = fx
        .git(&["ls-tree", "-r", &head])
        .lines()
        .filter(|l| !l.contains("\t.brewstillery/"))
        .collect::<Vec<_>>()
        .join("\n");
    let index =
        fx.git(&["ls-files", "-s"]).lines().filter(|l| !l.contains("\t.brewstillery/")).collect::<Vec<_>>().join("\n");
    let work = fx
        .snapshot()
        .into_iter()
        .filter(|(p, _)| p.starts_with(&fx.root) && !p.starts_with(&fx.domain) && !p.starts_with(fx.root.join(".git")))
        .collect();
    (tree, index, work)
}

/// PD-11, REQ-RUN-6, D-083: a Git error in one task's integration (its move, made to fail as Git fails) is recorded in
/// that task as `merge-failed`, `TOOL_FAILED`, with Git's exit status and the digest of its output, kept as evidence;
/// nothing moved, the task stays accepted, reading commands work, another task builds meanwhile, and `run` then
/// finishes the integration.
#[test]
fn pd_11_a_git_error_in_one_tasks_integration_is_recorded_in_that_task() {
    let (fx, id, args) = to_accept("pd-11");
    let before = product(&fx);
    let argv: Vec<&str> = args.iter().map(String::as_str).collect();
    let ran = fx.run_env(&argv, &[("FABREWK_FAULT_GIT", "fabrewk: integrate")]);
    assert_eq!(ran.status, 4, "{ran:?}");
    whole(&ran.all());
    assert!(ran.stdout.contains(&cli::say("es", "accept.failed", &[("id", &id), ("code", "TOOL_FAILED")])), "{ran:?}");
    let failed = last_of(&fx, &id);
    let p = &failed["payload"];
    assert_eq!(
        (failed["kind"].as_str(), p["code"].as_str(), &p["moved"]),
        (Some("merge-failed"), Some("TOOL_FAILED"), &json!(false))
    );
    assert_eq!(p["evidence"]["status"], 129, "{failed}");
    // The evidence the reason names, relative to the repository, holds Git's own output, whose digest the event holds.
    let reason = p["reason"].as_str().unwrap();
    let kept = reason.split("kept in ").nth(1).unwrap();
    assert!(kept.starts_with(".brewstillery/local/evidence/git-"), "{reason}");
    let stderr = fs::read(fx.root.join(kept).join("stderr")).unwrap();
    assert_eq!(p["evidence"]["stderr"], sha256_hex(&stderr));
    assert!(String::from_utf8_lossy(&stderr).contains("fabrewk-fault"), "Git's own words");
    assert_eq!(product(&fx), before, "nothing moved");
    assert_eq!(fx.git(&["show", "-s", "--format=%s", "HEAD"]), "fabrewk: record");
    assert_eq!(fx.run(&["status", &id, "--json"]).doc()["state"], "accepted");
    assert_eq!(fx.run(&["check"]).status, 0);
    // The failure waits for a maestro, who runs it again; another task builds meanwhile.
    let waits = fx.run(&["status", "--json"]).doc()["waiting"].clone();
    assert!(waits.as_array().unwrap().iter().any(|w| w["what"] == "task-merge" && w["id"] == id.as_str()), "{waits}");
    let other = verified_task(&fx, json!({}), json!({"goal": "Another."}));
    let ran = fx.run(&["run", &id, "--say", SAY, "--json"]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(ran.doc()["schema"], "fabrewk.accepted/1");
    assert_eq!(last_of(&fx, &id)["kind"], "task-integrated");
    assert_eq!(fs::read(fx.root.join("src/new.txt")).unwrap(), b"new\n");
    let _ = other;
}

/// PD-12, PD-13, D-087: a precondition made false just before the move (a product that moved, a person's edit at a
/// touched path, a branch moved under the compare-and-swap) records `task-accepted` and `merge-failed`, nothing moved but the record commit; once the cause is gone,
/// `run` finishes it.
#[test]
fn pd_12_a_precondition_false_just_before_the_move() {
    // `race` moves the branch between Fabrewk's read and its `update-ref`, whose compare-and-swap fails (PD-13).
    for what in ["base", "edit", "race", "race-to-candidate"] {
        let (fx, id, args) = to_accept(&format!("pd-12-before-{what}"));
        let steps = tmp(&format!("pd-12-before-{what}-steps"));
        let argv: Vec<&str> = args.iter().map(String::as_str).collect();
        let step = if what.starts_with("race") { "before-update-ref" } else { "before-move" };
        let child = paused(&fx, &argv, step, &steps);
        let code = match what {
            "race-to-candidate" => {
                fx.git(&["cherry-pick", &fx.git(&["rev-parse", &format!("fabrewk/{id}")])]);
                "BASE_MOVED"
            }
            "base" | "race" => {
                // Only the person's file: the command's events are written and not committed yet.
                fs::write(fx.root.join("README.md"), "moved\n").unwrap();
                fx.git(&["commit", "-q", "-m", "a person's change", "README.md"]);
                "BASE_MOVED"
            }
            _ => {
                fs::create_dir_all(fx.root.join("src")).unwrap();
                fs::write(fx.root.join("src/new.txt"), "mine\n").unwrap();
                "WORKTREE_DIRTY"
            }
        };
        let before = product(&fx);
        let ran = resume(child, step, &steps);
        assert_eq!(ran.status, 4, "{what}: {ran:?}");
        let events = fx.events(&format!("task-{id}"));
        let n = events.len();
        assert_eq!(
            (events[n - 2]["kind"].as_str(), events[n - 1]["kind"].as_str()),
            (Some("task-accepted"), Some("merge-failed"))
        );
        assert_eq!(
            (&events[n - 1]["payload"]["code"], &events[n - 1]["payload"]["moved"]),
            (&json!(code), &json!(what == "race-to-candidate"))
        );
        assert_eq!(product(&fx), before, "{what}: nothing moved");
        let place = if what == "race-to-candidate" { "accept.moved" } else { "accept.unmoved" };
        assert!(ran.stdout.contains(&cli::say("es", place, &[])), "{what}: {ran:?}");
        // The compare-and-swap Git refused keeps Git's output as evidence, as every failed Git does (P9G-4).
        // A branch someone else brought to the candidate meanwhile is at it, which the failure says (P9G-6).
        if what.starts_with("race") {
            let p = &events[n - 1]["payload"];
            assert!(p["evidence"]["stderr"].is_string(), "{p}");
            let kept = p["reason"].as_str().unwrap().split("kept in ").nth(1).unwrap().to_owned();
            assert!(fx.root.join(kept).join("stderr").is_file());
        }
        // With the cause gone, `run` finishes it.
        match what {
            // The person's commit, under the record commit the failure made.
            "base" | "race" => fx.git(&["revert", "--no-edit", "HEAD~1"]),
            "race-to-candidate" => String::new(),
            _ => {
                fs::remove_file(fx.root.join("src/new.txt")).unwrap();
                String::new()
            }
        };
        let ran = fx.run(&["run", &id, "--say", SAY]);
        assert_eq!(ran.status, 0, "{what}: {ran:?}");
        assert_eq!(last_of(&fx, &id)["kind"], "task-integrated");
    }
}

/// PD-12 and PD-14: with the branch at the candidate, a person's edit at a touched path is never written: `merge-failed`
/// saying that the branch is at the candidate, the edit byte-identical, `reject` refused (`MERGE_IN_PROGRESS`); and the
/// binary killed between the move and the working tree's update: in both, `run` finishes the integration.
#[test]
fn pd_12_pd_14_with_the_branch_at_the_candidate() {
    for what in ["edit", "kill"] {
        let (fx, id, args) = to_accept(&format!("pd-12-after-{what}"));
        let steps = tmp(&format!("pd-12-after-{what}-steps"));
        let argv: Vec<&str> = args.iter().map(String::as_str).collect();
        let mut child = paused(&fx, &argv, "after-move", &steps);
        let integration = fx.git(&["rev-parse", "HEAD"]);
        assert_eq!(fx.git(&["show", "-s", "--format=%s", "HEAD"]), format!("fabrewk: candidate for {id}"));
        if what == "kill" {
            child.kill().unwrap();
            child.wait().unwrap();
            assert!(!fx.root.join("src/new.txt").exists());
            // An accepted task whose integration stopped without a failure on record waits for no one.
            let waits = fx.run(&["status", "--json"]).doc()["waiting"].clone();
            assert!(waits.as_array().unwrap().iter().all(|w| w["what"] != "task-merge"), "{waits}");
            // Run again with a person's edit there: the branch is at the candidate, and the failure says so (P9G-2).
            fs::create_dir_all(fx.root.join("src")).unwrap();
            fs::write(fx.root.join("src/new.txt"), "mine\n").unwrap();
            let ran = fx.run(&["run", &id, "--say", SAY]);
            assert_eq!(ran.status, 4, "{ran:?}");
            let failed = last_of(&fx, &id);
            assert_eq!(
                (&failed["payload"]["code"], &failed["payload"]["moved"]),
                (&json!("WORKTREE_DIRTY"), &json!(true))
            );
            assert_eq!(fs::read(fx.root.join("src/new.txt")).unwrap(), b"mine\n");
            fs::remove_file(fx.root.join("src/new.txt")).unwrap();
        } else {
            fs::create_dir_all(fx.root.join("src")).unwrap();
            fs::write(fx.root.join("src/new.txt"), "mine\n").unwrap();
            let ran = resume(child, "after-move", &steps);
            assert_eq!(ran.status, 4, "{ran:?}");
            assert!(ran.stdout.contains(&cli::say("es", "accept.moved", &[])), "{ran:?}");
            let failed = last_of(&fx, &id);
            assert_eq!(
                (&failed["payload"]["code"], &failed["payload"]["moved"]),
                (&json!("WORKTREE_DIRTY"), &json!(true))
            );
            assert_eq!(fs::read(fx.root.join("src/new.txt")).unwrap(), b"mine\n", "the person's edit survives");
            assert_eq!(fx.git(&["rev-parse", "HEAD~1"]), integration);
            refused(&fx, &["reject", &id, "--say", SAY], "MERGE_IN_PROGRESS", 1, "es");
            fs::remove_file(fx.root.join("src/new.txt")).unwrap();
        }
        let ran = fx.run(&["run", &id, "--say", SAY]);
        assert_eq!(ran.status, 0, "{what}: {ran:?}");
        // No move was needed: the branch is where the integration left it, with the record's commits on top.
        let done = last_of(&fx, &id);
        let to = done["payload"]["to"].as_str().unwrap();
        assert_eq!(done["payload"]["from"], Value::Null, "{what}");
        fx.git(&["merge-base", "--is-ancestor", &integration, to]);
        fx.git(&["diff", "--quiet", &integration, to, "--", ".", ":!.brewstillery"]);
        assert_eq!(fs::read(fx.root.join("src/new.txt")).unwrap(), b"new\n");
        assert_eq!(fx.git(&["status", "--porcelain"]), "", "{what}");
        assert_eq!(fx.run(&["check"]).status, 0, "{what}");
    }
}

/// REQ-GATE-3, REQ-GATE-10 through the flow: the platform a check's identity holds, changed between the verification
/// and the acceptance (`FABREWK_FAULT_PLATFORM`), makes the evidence stale: `accept` refuses `EVIDENCE_STALE` before
/// the signature, nothing written (D-087 point 1).
#[test]
fn refusal_evidence_stale_through_the_flow() {
    let fx = Fx::builder("stale-platform", "published", &format!("{WRITES}{SUCCESS}"));
    let check = json!({"checks": {"truth": {"command": ["true"], "network": false, "minutes": 1, "tools": []}}});
    let id = verified_task(&fx, check, json!({}));
    let args = ["accept", &id, "--seen", &accepting(&fx, &id), "--say", SAY];
    let before = fx.snapshot();
    let ran = fx.run_env(&args, &[("FABREWK_FAULT_PLATFORM", "another-platform")]);
    says(&ran, "EVIDENCE_STALE", false);
    assert_eq!(fx.snapshot(), before);
    // Accepted, its integration failed on a person's file; the evidence stale when it runs again is refused just before
    // the move, nothing moved (point 2).
    let steps = tmp("stale-platform-steps");
    let child = paused(&fx, &args, "before-move", &steps);
    fs::create_dir_all(fx.root.join("src")).unwrap();
    fs::write(fx.root.join("src/new.txt"), "mine\n").unwrap();
    assert_eq!(resume(child, "before-move", &steps).status, 4);
    fs::remove_file(fx.root.join("src/new.txt")).unwrap();
    let product_before = product(&fx);
    let ran = fx.run_env(&["run", &id, "--say", SAY], &[("FABREWK_FAULT_PLATFORM", "another-platform")]);
    assert_eq!(ran.status, 4, "{ran:?}");
    let p = &last_of(&fx, &id)["payload"];
    assert_eq!((&p["code"], &p["moved"]), (&json!("EVIDENCE_STALE"), &json!(false)), "{p}");
    assert_eq!(product(&fx), product_before);
    let ran = fx.run(&["run", &id, "--say", SAY]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(last_of(&fx, &id)["kind"], "task-integrated");
}

/// P9G-3, D-083: a Git that fails while a write command reads a committed or a staged file is `TOOL_FAILED` with Git's
/// output kept as evidence, never taken for the file's absence.
#[test]
fn p9g_3_a_failed_git_read_is_no_absence() {
    let fx = Fx::new("p9g-3", "published");
    // The committed file, read for the adoption, and the staged one, read for an approval of a direct edit.
    fx.alambique(SMALL, "published");
    fs::write(fx.domain.join("ALAMBIQUE.md"), format!("{SMALL}\nEdited by hand.\n")).unwrap();
    fx.commit("a direct edit");
    fx.edit_policy(&(policy_text(&fx.keys) + "\n[limits]\nminutes = 60\n"));
    let adopt = vec!["adopt", "--say", SAY, "--json"];
    let approve = vec!["approve", "alambique", "--say", SAY, "--json"];
    for (fault, args) in [("cat-file blob", &adopt), ("ls-tree -z", &adopt), ("ls-files -z", &approve)] {
        let ran = fx.run_env(args, &[("FABREWK_FAULT_GIT", fault)]);
        assert_eq!(ran.status, 1, "{fault}: {ran:?}");
        let e = &ran.doc()["refusals"][0];
        assert_eq!(e["code"], "TOOL_FAILED", "{fault}: {e}");
        assert!(e["detail"].as_str().unwrap().contains("Git's output is kept in"), "{fault}: {e}");
    }
    assert_eq!(fx.run(&["adopt", "--say", SAY]).status, 0);
}

/// P9G-6: a branch moved away from the candidate after Fabrewk's move is no longer at it, which the failure says.
#[test]
fn p9g_6_a_branch_moved_away_after_the_move_is_not_at_the_candidate() {
    let (fx, id, args) = to_accept("p9g-6");
    let steps = tmp("p9g-6-steps");
    let argv: Vec<&str> = args.iter().map(String::as_str).collect();
    let child = paused(&fx, &argv, "after-move", &steps);
    fs::write(fx.root.join("README.md"), "moved\n").unwrap();
    fx.git(&["commit", "-q", "-m", "a person's change", "README.md"]);
    let ran = resume(child, "after-move", &steps);
    assert_eq!(ran.status, 4, "{ran:?}");
    let failed = last_of(&fx, &id);
    assert_eq!((&failed["payload"]["code"], &failed["payload"]["moved"]), (&json!("BASE_MOVED"), &json!(false)));
}

/// P9G-8: a failure after the move whose recording cannot read where the branch is (a second Git failing) keeps the
/// first failure's evidence, the one its reason names, and says the branch's place is unknown (`moved` null), naming the
/// second's evidence too.
#[test]
fn p9g_8_p9g_9_the_evidence_recorded_is_the_failures_own() {
    // Where the branch is read by its products' trees (P9G-8) or by its head (P9G-9).
    for then in ["ls-tree -z --full-tree", "rev-parse --verify -q"] {
        // `unpublished`: the head is the base, so that the first index update is the person's, after the move.
        let fx = Fx::builder(&format!("p9g-8-{}", &then[..3]), "unpublished", &format!("{WRITES}{SUCCESS}"));
        let id = verified_task(&fx, json!({}), json!({}));
        let args = ["accept", &id, "--seen", &accepting(&fx, &id), "--say", SAY];
        let env = [("FABREWK_FAULT_GIT", "update-index -z --index-info"), ("FABREWK_FAULT_GIT_THEN", then)];
        let ran = fx.run_env(&args, &env);
        assert_eq!(ran.status, 4, "{then}: {ran:?}");
        let failed = last_of(&fx, &id);
        let p = &failed["payload"];
        assert_eq!((&p["code"], &p["moved"]), (&json!("TOOL_FAILED"), &Value::Null), "{then}: {failed}");
        let reason = p["reason"].as_str().unwrap();
        let dirs: Vec<&str> = reason.split("kept in ").skip(1).map(|r| r.split([';', ' ']).next().unwrap()).collect();
        assert_eq!(dirs.len(), 2, "{then}: {reason}");
        assert!(reason.contains("git update-index"), "{reason}");
        assert_eq!(p["evidence"]["stderr"], sha256_hex(&fs::read(fx.root.join(dirs[0]).join("stderr")).unwrap()));
        assert_eq!(p["evidence"]["status"], 129);
    }
}

/// P9G-10, P9G-11: a Git that fails while a write command reads where the record branch is, or the candidate's parent,
/// is `TOOL_FAILED` with its evidence, never an absence: `reject` does not reject an integration halfway done because
/// the branch could not be read, and `accept` never says that a candidate has no parent.
#[test]
fn p9g_10_p9g_11_a_failed_reading_of_a_commit_is_no_absence() {
    // `unpublished`: no record commit, so that `reject` reads the branch's head first where it locates the candidate.
    let fx = Fx::builder("p9g-10", "unpublished", &format!("{WRITES}{SUCCESS}"));
    let id = verified_task(&fx, json!({}), json!({}));
    let digest = accepting(&fx, &id);
    let args = ["accept", &id, "--seen", &digest, "--say", SAY];
    let ran = fx.run_env(&args, &[("FABREWK_FAULT_GIT", "^1^{commit}")]);
    assert_eq!(ran.status, 1, "{ran:?}");
    assert!(ran.stderr.contains("TOOL_FAILED") && ran.stderr.contains("Git's output is kept in"), "{ran:?}");
    let steps = tmp("p9g-10-steps");
    let mut child = paused(&fx, &args, "after-move", &steps);
    child.kill().unwrap();
    child.wait().unwrap();
    let ran = fx.run_env(&["reject", &id, "--say", SAY], &[("FABREWK_FAULT_GIT", "rev-parse --verify -q refs/heads/")]);
    assert_eq!(ran.status, 1, "{ran:?}");
    assert!(ran.stderr.contains("TOOL_FAILED") && ran.stderr.contains("Git's output is kept in"), "{ran:?}");
    assert_eq!(last_of(&fx, &id)["kind"], "task-accepted");
    refused(&fx, &["reject", &id, "--say", SAY], "MERGE_IN_PROGRESS", 1, "es");
}

/// P9G-13, P9G-14 and the sweep of their class (D-083, D-086 point 3): wherever a write command reads a state from Git
/// (the branch checked out, the key it signs with, the main working tree, the base's ignore rules, the candidate's
/// branch), a Git that fails is `TOOL_FAILED`, or keeps its code, with Git's exit status and evidence named, never
/// taken for that state; `status` says it too, writing nothing.
#[test]
fn p9g_13_p9g_14_a_failed_git_is_never_a_state() {
    let fx = Fx::builder("p9g-13", "unpublished", &format!("{WRITES}{SUCCESS}"));
    let id = verified_task(&fx, json!({}), json!({}));
    let kept = |ran: &Ran, code: &str| {
        let doc = ran.doc();
        let e = doc["refusals"].as_array().unwrap().iter().find(|e| e["code"] == code).cloned();
        let e = e.unwrap_or_else(|| panic!("{code}: {doc}"));
        assert!(e["detail"].as_str().unwrap().contains("Git's output is kept in"), "{code}: {e}");
    };
    for (fault, code) in [
        ("symbolic-ref -q HEAD", "TOOL_FAILED"),
        ("user.signingkey", "TOOL_FAILED"),
        ("worktree list", "NOT_A_REPOSITORY"),
    ] {
        let ran = fx.run_env(&["reject", &id, "--say", SAY, "--json"], &[("FABREWK_FAULT_GIT", fault)]);
        assert_eq!(ran.status, 1, "{fault}: {ran:?}");
        kept(&ran, code);
    }
    let before = fx.snapshot();
    let ran = fx.run_env(&["status", &id], &[("FABREWK_FAULT_GIT", "rev-parse --verify -q refs/heads/")]);
    assert_eq!(ran.status, 1, "{ran:?}");
    assert!(ran.stderr.contains("TOOL_FAILED") && ran.stderr.contains("exit status 129"), "{ran:?}");
    assert_eq!(fx.snapshot(), before, "a reading command keeps nothing");
    // The run: the base's ignore rules (a base that has some), and the candidate's branch.
    fs::write(fx.root.join(".gitignore"), "*.log\n").unwrap();
    fx.git(&["add", ".gitignore"]);
    fx.git(&["commit", "-q", "-m", "ignore rules"]);
    for fault in ["check-ignore --no-index", "fabrewk: candidate"] {
        let other = verified_task_halting(&fx, fault);
        let halt = last_of(&fx, &other);
        assert_eq!(
            (&halt["kind"], &halt["payload"]["code"]),
            (&json!("task-halted"), &json!("TOOL_FAILED")),
            "{fault}"
        );
        assert!(halt["payload"]["reason"].as_str().unwrap().contains("Git's output is kept in"), "{fault}: {halt}");
    }
}

/// A task proposed and authorized, then run with `FABREWK_FAULT_GIT` naming `fault`, which halts it; gives its identifier.
fn verified_task_halting(fx: &Fx, fault: &str) -> String {
    let head = fx.git(&["rev-parse", "HEAD"]);
    let plan = json!({
        "stage": {"lang": "en", "goal": "Halt.", "budget": {"dispatches": 6, "minutes": 120}, "checks": {}, "settings": {}},
        "tasks": [{
            "lang": "en", "goal": fault, "requirements": [{"text": "It builds.", "criterion": "it builds"}],
            "base": head, "dependencies": [], "allowed": ["src/**"], "profiles": {"build": "build", "review": "review"},
            "checks": [], "limits": {"rounds": 3, "dispatches": 6, "minutes": 120}, "delegationLanguage": "en",
            "settings": {},
        }],
    });
    fs::write(fx.root.join("plan.json"), serde_json::to_vec(&plan).unwrap()).unwrap();
    let ran = fx.run(&["propose", "plan.json", "--json"]);
    fs::remove_file(fx.root.join("plan.json")).unwrap();
    let id = ran.doc()["tasks"][0]["id"].as_str().unwrap().to_owned();
    let digest = fx.run(&["status", &id, "--json"]).doc()["shown"]["digest"].as_str().unwrap().to_owned();
    assert_eq!(fx.run(&["authorize", &id, "--seen", &digest, "--say", SAY]).status, 0);
    let ran = fx.run_env(&["run", &id, "--say", SAY], &[("FABREWK_FAULT_GIT", fault)]);
    assert_eq!(ran.status, 4, "{fault}: {ran:?}");
    id
}

/// P9G-15: a failure whose auxiliary reading fails too keeps both outputs, the first in the event: the integration's
/// move, then the branch's head; a ref that names no commit, then its listing.
#[test]
fn p9g_15_a_failure_keeps_its_own_evidence_when_its_reading_fails_too() {
    let (fx, id, args) = to_accept("p9g-15");
    let argv: Vec<&str> = args.iter().map(String::as_str).collect();
    let env =
        [("FABREWK_FAULT_GIT", "fabrewk: integrate"), ("FABREWK_FAULT_GIT_THEN", "rev-parse --verify -q refs/heads/")];
    let ran = fx.run_env(&argv, &env);
    assert_eq!(ran.status, 4, "{ran:?}");
    let p = last_of(&fx, &id)["payload"].clone();
    let reason = p["reason"].as_str().unwrap();
    assert!(reason.starts_with("git update-ref -m failed with exit status 129"), "{reason}");
    // The move's, its head's reading, and the reading of `moved`, which fails as well: each kept and named.
    assert_eq!(reason.matches("kept in").count(), 3, "{reason}");
    assert_eq!(p["evidence"]["status"], 129, "{p}");
    // A branch whose ref names a blob, and a listing that fails: both outputs are kept and named.
    let blob = fx.git(&["hash-object", "-w", "README.md"]);
    let good = fx.git(&["rev-parse", "HEAD"]);
    fs::write(fx.root.join(".git/refs/heads/main"), format!("{blob}\n")).unwrap();
    let ran = fx.run_env(&["adopt", "--say", SAY], &[("FABREWK_FAULT_GIT", "for-each-ref")]);
    fs::write(fx.root.join(".git/refs/heads/main"), format!("{good}\n")).unwrap();
    assert_eq!(ran.status, 1, "{ran:?}");
    assert_eq!(ran.stderr.matches("Git's output is kept in").count(), 2, "{ran:?}");
}

/// P9G-16, D-089: what failed in Git that a command went on from is in its document too: the person's index that
/// cannot follow the record commit.
#[test]
fn p9g_16_what_a_command_went_on_from_is_in_its_document() {
    let fx = ready("p9g-16");
    let dir = tmp("p9g-16-steps");
    let child = paused(&fx, &["adopt", "--say", SAY, "--json"], "record-moved", &dir);
    fs::write(fx.root.join(".git/index.lock"), b"").unwrap();
    let ran = resume(child, "record-moved", &dir);
    fs::remove_file(fx.root.join(".git/index.lock")).unwrap();
    assert_eq!(ran.status, 0, "{ran:?}");
    let doc = ran.doc();
    let recovered = doc["recovered"].as_array().unwrap();
    assert_eq!(recovered.len(), 1, "{doc}");
    assert!(recovered[0].as_str().unwrap().contains("Git's output is kept in"), "{doc}");
}

/// P9G-18: the record commit's compare-and-swap that a moved branch refuses is named by every way out of its round: a
/// branch moved back to an ancestor is `RECORD_ROLLBACK`, naming it.
#[test]
fn p9g_18_a_rollback_after_the_compare_and_swap_names_its_evidence() {
    let fx = ready("p9g-18");
    let dir = tmp("p9g-18-steps");
    let child = paused(&fx, &["adopt", "--say", SAY], "before-commit", &dir);
    fx.git(&["update-ref", "refs/heads/main", "HEAD~1"]);
    let ran = resume(child, "before-commit", &dir);
    assert_eq!(ran.status, 1, "{ran:?}");
    assert!(ran.stderr.contains("RECORD_ROLLBACK") && ran.stderr.contains("Git's output is kept in"), "{ran:?}");
}

/// P9G-19, D-089: the event's evidence is the first its failure's words name: for a ref that names no commit, read when
/// the integration is about to move and whose listing fails too, the reading's own.
#[test]
fn p9g_19_the_events_evidence_is_the_first_its_words_name() {
    let (fx, id, args) = to_accept("p9g-19");
    let good = fx.git(&["rev-parse", "HEAD"]);
    let blob = fx.git(&["hash-object", "-w", "README.md"]);
    let steps = tmp("p9g-19-steps");
    let argv: Vec<&str> = args.iter().map(String::as_str).collect();
    let child = paused_env(&fx, &argv, "before-move", &steps, &[("FABREWK_FAULT_GIT", "for-each-ref")]);
    fs::write(fx.root.join(".git/refs/heads/main"), format!("{blob}\n")).unwrap();
    let ran = resume(child, "before-move", &steps);
    fs::write(fx.root.join(".git/refs/heads/main"), format!("{good}\n")).unwrap();
    // The record commit cannot read the branch either: the events stay written, and the failure is among them.
    assert_eq!(ran.status, 1, "{ran:?}");
    let failed = last_of(&fx, &id);
    assert_eq!(failed["kind"], "merge-failed", "{failed}");
    let p = &failed["payload"];
    assert_eq!(p["evidence"]["status"], 1, "{p}");
    let reason = p["reason"].as_str().unwrap();
    let first = reason.split("kept in ").nth(1).unwrap().split([';', ' ']).next().unwrap();
    assert_eq!(p["evidence"]["stderr"], sha256_hex(&fs::read(fx.root.join(first).join("stderr")).unwrap()));
}

/// P9G-20: what a recovery went on from (its record commit rebuilt on a branch someone moved) is named by the refusal
/// of the command that follows it.
#[test]
fn p9g_20_a_refusal_after_a_recovery_names_what_it_went_on_from() {
    let fx = Fx::new("p9g-20", "published");
    let (_, id) = fx.stage_and_task();
    fx.commit("proposed");
    let digest = fx.run(&["status", &id, "--json"]).doc()["shown"]["digest"].as_str().unwrap().to_owned();
    assert_eq!(fx.run(&["authorize", &id, "--seen", &digest, "--say", SAY]).status, 0);
    let stream = format!("task-{id}");
    let events = fx.events(&stream);
    let auth = format!("{:06}", events.len());
    let file = fs::read_dir(fx.domain.join("data/events").join(&stream))
        .unwrap()
        .map(|e| e.unwrap().path())
        .find(|p| p.file_name().unwrap().to_str().unwrap().starts_with(&auth) && p.extension().unwrap() == "json")
        .unwrap();
    let at = json!({"seq": events.len(), "digest": sha256_hex(&fs::read(file).unwrap())});
    let run = json!({"task": id, "head": at, "authorization": {"stream": stream, "seq": at["seq"], "digest": at["digest"]}, "base": events[0]["payload"]["base"]});
    appended(&fx, &stream, "run-started", run, true);
    let attempt = json!({"role": "build", "profile": "build", "tool": "claude", "version": null, "model": "m", "effort": "low", "allowedSeconds": 60});
    appended(&fx, &stream, "attempt-started", attempt, false);
    // The recovery's record commit meets a branch someone moved, and is rebuilt on it; then the words are empty.
    let steps = tmp("p9g-20-steps");
    let child = paused(&fx, &["reject", &id, "--say", " ", "--json"], "before-commit", &steps);
    fs::write(fx.root.join("README.md"), "moved\n").unwrap();
    fx.git(&["commit", "-q", "-m", "a person's change", "README.md"]);
    let ran = resume(child, "before-commit", &steps);
    assert_eq!(ran.status, 1, "{ran:?}");
    let e = &ran.doc()["refusals"][0];
    assert_eq!(e["code"], "WORDS_MISSING", "{e}");
    assert!(e["detail"].as_str().unwrap().contains("before it, git update-ref"), "{e}");
}

/// `INSTALLATION_INCOMPLETE` (D-097 point 7; 11.4): an `apply` stopped once `assessed` is written and before
/// `installed`; every other write command refuses it with nothing written, `check` reports it, and the same `apply`,
/// run again, finishes it, after which they work.
#[test]
fn refusal_installation_incomplete() {
    let (fx, options) = Fx::installable("installation-incomplete", "published", None);
    let mut n = 1;
    let digest = loop {
        let base = fx.root.parent().unwrap();
        let ran = fx.apply(&options, &[("FABREWK_FAULT_ABORT_AT", &n.to_string())]);
        assert_eq!(ran.status, -1, "{n}: {ran:?}");
        let kinds: Vec<_> = fx.events("project").iter().map(|e| e["kind"].clone()).collect();
        if kinds.len() == 2 {
            break fx.events("project")[0]["payload"]["digest"].as_str().unwrap().to_owned();
        }
        // Back to before it, as a person who never ran it.
        let _ = std::fs::remove_dir_all(&fx.domain);
        assert!(base.exists());
        n += 1;
    };
    let plan = flow::plan(&fx);
    refused(&fx, &["propose", &plan], "INSTALLATION_INCOMPLETE", 1, "en");
    refused(&fx, &["adopt", "--say", "Lo decido."], "INSTALLATION_INCOMPLETE", 1, "en");
    let doc = fx.run(&["check", "--json"]).doc();
    let found: Vec<&Value> =
        doc["findings"].as_array().unwrap().iter().filter(|f| f["code"] == "INSTALLATION_INCOMPLETE").collect();
    assert_eq!(found.len(), 1, "{doc}");
    assert_eq!(found[0]["check"], "C2", "{doc}");
    let again = fx.apply_digest(&digest, &options, &[]);
    assert_eq!(again.status, 0, "{again:?}");
    assert_eq!(fx.run(&["check"]).status, 0);
    assert_eq!(fx.run(&["propose", &plan]).status, 0);
}

/// An `apply` stopped after its first two events finishes from its plan, never reassessed; a directory of the plan
/// made a link meanwhile is refused (`PLAN_CHANGED`) and nothing is written through it (part 10b).
#[test]
fn a_resumed_apply_writes_through_no_link() {
    let (fx, options) = Fx::installable("apply-resume-link", "published", None);
    let mut n = 1;
    let digest = loop {
        let ran = fx.apply(&options, &[("FABREWK_FAULT_ABORT_AT", &n.to_string())]);
        assert_eq!(ran.status, -1, "{n}: {ran:?}");
        if fx.events("project").len() == 2 {
            break fx.events("project")[0]["payload"]["digest"].as_str().unwrap().to_owned();
        }
        let _ = std::fs::remove_dir_all(&fx.domain);
        n += 1;
    };
    assert!(!fx.domain.join("xel").exists());
    let outside = fx.root.parent().unwrap().join("outside");
    fs::create_dir(&outside).unwrap();
    std::os::unix::fs::symlink(&outside, fx.domain.join("xel")).unwrap();
    let ran = fx.apply_digest(&digest, &options, &[]);
    assert_eq!(ran.status, 1, "{ran:?}");
    assert_eq!(ran.doc()["refusals"][0]["code"], "PLAN_CHANGED", "{ran:?}");
    assert_eq!(fs::read_dir(&outside).unwrap().count(), 0);
    fs::remove_file(fx.domain.join("xel")).unwrap();
    assert_eq!(fx.apply_digest(&digest, &options, &[]).status, 0);
    assert_eq!(fx.run(&["check"]).status, 0);
}
