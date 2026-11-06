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
