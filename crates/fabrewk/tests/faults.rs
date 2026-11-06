//! The write commands' refusals that need the feature `fault-injection` (core-specification 11.2 rules 4 and 8; part
//! 7d3a): a stream's head moved while the command ran (`RECORD_HEAD_MOVED`, REQ-REG-5), the branch moved under the
//! record commit (`RECORD_COMMIT_RACE`, REQ-REG-10), and a replay that fails before the commit (PD-46). The binary
//! pauses at a named step while the test acts, as another process would.

mod common;

use common::*;
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

/// A fixture whose machine key exists, made by an adoption that ran to its end, and whose project file has another
/// change to adopt, committed.
fn ready(name: &str) -> Fx {
    let fx = Fx::new(name, "published");
    fx.edit_policy(&(policy_text(&fx.keys) + "\n[limits]\nminutes = 70\n"));
    assert_eq!(fx.run(&["adopt", "--say", SAY]).status, 0);
    fx.edit_policy(&(policy_text(&fx.keys) + "\n[limits]\nminutes = 60\n"));
    fx
}

#[test]
fn refusal_record_head_moved() {
    let fx = ready("head-moved");
    let dir = tmp("head-moved-steps");
    let child = paused(&fx, &["adopt", "--say", SAY], "before-append", &dir);
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
    assert_eq!(ran.status, 1, "{ran:?}");
    assert!(ran.stderr.contains("RECORD_HEAD_MOVED"), "{ran:?}");
}

#[test]
fn refusal_record_commit_race() {
    let fx = ready("race");
    let head = fx.git(&["rev-parse", "HEAD"]);
    let dir = tmp("race-steps");
    let child = paused(&fx, &["adopt", "--say", SAY], "before-commit", &dir);
    // Someone commits, on the record branch, the very file the record commit holds.
    let new = files(&fx.domain.join("data/events/project")).pop().unwrap();
    let new = new.with_extension("").with_extension("json");
    fx.git(&["add", new.to_str().unwrap()]);
    fx.git(&["commit", "-q", "-m", "someone"]);
    let theirs = fx.git(&["rev-parse", "HEAD"]);
    let ran = resume(child, "before-commit", &dir);
    assert_eq!(ran.status, 1, "{ran:?}");
    assert!(ran.stderr.contains("RECORD_COMMIT_RACE"), "{ran:?}");
    // The branch is theirs; the event stays written.
    assert_eq!(fx.git(&["rev-parse", "HEAD"]), theirs);
    assert_eq!(fx.git(&["rev-parse", "HEAD~1"]), head);
    assert!(new.exists());
}

#[test]
fn pd_46_a_replay_that_fails_before_the_commit_commits_nothing() {
    let fx = ready("pd-46");
    let head = fx.git(&["rev-parse", "HEAD"]);
    let before = files(&fx.domain.join("data/events/project")).len();
    let args: Vec<std::ffi::OsString> = ["adopt", "--say", SAY].iter().map(Into::into).collect();
    let ran = fx.run_os(&args, &[("FABREWK_FAULT_FAIL", std::ffi::OsStr::new("replay-before-commit"))]);
    assert_eq!(ran.status, 1, "{ran:?}");
    assert!(ran.stderr.contains("RECORD_BROKEN") && ran.stderr.contains("fault injection"), "{ran:?}");
    assert_eq!(fx.git(&["rev-parse", "HEAD"]), head, "nothing committed");
    // The events stay written and uncommitted, and the next write command commits them first.
    assert_eq!(files(&fx.domain.join("data/events/project")).len(), before + 2);
    let doc = fx.run(&["check", "--json"]).doc();
    assert!(doc["findings"].as_array().unwrap().iter().any(|f| f["code"] == "RECORD_UNCOMMITTED"), "{doc}");
    fs::write(fx.root.join("plan.json"), "{}").unwrap();
    assert_eq!(fx.run(&["propose", "plan.json"]).status, 1);
    fs::remove_file(fx.root.join("plan.json")).unwrap();
    assert_eq!(fx.git(&["rev-parse", "HEAD"]), head, "a refused command commits nothing");
    fx.edit_policy(&(policy_text(&fx.keys) + "\n[limits]\nminutes = 50\n"));
    assert_eq!(fx.run(&["adopt", "--say", SAY]).status, 0);
    let first = fx.git(&["log", "-1", "--format=%B", "HEAD~1"]);
    assert_eq!(first.matches("Fabrewk-Event: ").count(), 1, "the leftover in a commit of its own: {first}");
    assert_eq!(fx.run(&["check"]).status, 0, "{:?}", fx.run(&["check"]));
}
