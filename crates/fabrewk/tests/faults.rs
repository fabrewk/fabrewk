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
    let mut c = Command::new(env!("CARGO_BIN_EXE_fabrewk"));
    isolate(&mut c, &fx.home, None);
    c.env("FABREWK_FAULT_PAUSE", format!("{step}:{}", dir.display()));
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
        let objects = fx.root.join(".git/objects");
        let snapshot = || {
            let mut s = fx.snapshot();
            s.retain(|p, _| !p.starts_with(&objects));
            s
        };
        let before = snapshot();
        let ran = resume(child, "before-commit", &dir);
        says(&ran, "RECORD_COMMIT_RACE", json);
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
    let payload = json!({"stage": stage, "digest": sha256_hex(&bytes), "head": head_json, "tasks": [], "widened": []});
    let h = s.append(&stream, Some(&head), &draft("stage-authorized", payload), Some(&signer)).unwrap();
    s.append(&stream, Some(&h), &draft("stage-closed", json!({})), Some(&signer)).unwrap();
    s.finish(&fabrewk_core::identity::PolicyVerifier, &[]).unwrap();
}
