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
    // P8B-4: the hook takes the index's lock once the ref moves; the person is told to bring the index to the commit,
    // and the record is committed by the next write command.
    use std::os::unix::fs::PermissionsExt;
    // A path a shell would split is not printed as a command, but said in words.
    let fx = Fx::new(if spaced { "alambique index spaced" } else { "alambique-index" }, "published");
    fx.alambique(SMALL, "published");
    let id = alambique_proposal(&fx);
    let hook = fx.root.join(".git/hooks/reference-transaction");
    fs::write(&hook, "#!/bin/sh\nif [ \"$1\" = committed ]; then touch .git/index.lock; fi\nexit 0\n").unwrap();
    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
    let head = fx.git(&["rev-parse", "HEAD"]);
    let ran = fx.run(&["approve", &id, "--say", SAY, "--json"]);
    assert_eq!(ran.status, 1, "{ran:?}");
    let detail = ran.doc()["refusals"][0]["detail"].as_str().unwrap().to_owned();
    let fix = if spaced {
        "stage .brewstillery/ALAMBIQUE.md in the repository at"
    } else {
        " add .brewstillery/ALAMBIQUE.md"
    };
    assert!(detail.contains(fix) && detail.contains("events are written"), "{detail}");
    assert_ne!(fx.git(&["rev-parse", "HEAD"]), head, "the alambique's commit is made");
    fs::remove_file(&hook).unwrap();
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
    use std::os::unix::fs::PermissionsExt;
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
    let hook = fx.domain.join(".git/hooks/reference-transaction");
    fs::write(&hook, "#!/bin/sh\nif [ \"$1\" = committed ]; then touch .git/index.lock; fi\nexit 0\n").unwrap();
    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
    let ran = fx.run(&["approve", &id, "--say", SAY, "--json"]);
    assert_eq!(ran.status, 1, "{ran:?}");
    let detail = ran.doc()["refusals"][0]["detail"].as_str().unwrap().to_owned();
    let fix = format!("run git -C {domain} add ALAMBIQUE.md;");
    assert!(detail.contains(&fix), "{detail}");
    fs::remove_file(&hook).unwrap();
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
