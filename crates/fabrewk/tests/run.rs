//! `run` past confinement through the built binary (part 9e; REQ-RUN-1, REQ-RUN-2, REQ-RUN-4 to REQ-RUN-7, REQ-RUN-10,
//! REQ-TASK-3, REQ-TASK-5, REQ-CLI-3's exit 4; S-5, S-6; D-079, D-082): a fake Claude Code on `PATH` under the real
//! tool's name (11.2 rule 3), the real adapter and the real backend; the run's events and their payloads, its halts,
//! its recovery and its candidate. These tests run real cells and need `bwrap`, `pasta`, `socat` and the network, as
//! the backend's own do.

mod common;

use common::*;
use fabrewk_core::cli;
use fabrewk_core::identity::PolicyVerifier;
use fabrewk_core::record::{Draft, Session, sha256_hex};
use serde_json::{Value, json};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::time::{Duration, Instant};

const SAY: &str = "Que corra.";

/// The fake builder's script that writes `src/new.txt` in its copy, then succeeds.
const WRITES: &str = "mkdir -p src && printf 'new\\n' > src/new.txt\n";

/// A stage and its tasks proposed through the binary on the fixture's head, with `stage` and each of `tasks` merged
/// into their defaults; gives the stage's identifier and each task's.
fn plan(fx: &Fx, stage: Value, tasks: Vec<Value>) -> (String, Vec<String>) {
    let head = fx.git(&["rev-parse", "HEAD"]);
    let mut s = json!({"lang": "en", "goal": "Build it.", "budget": {"dispatches": 6, "minutes": 120}, "checks": {}, "settings": {}});
    for (k, v) in stage.as_object().unwrap() {
        s[k] = v.clone();
    }
    let tasks: Vec<Value> = tasks
        .into_iter()
        .map(|more| {
            let mut t = json!({
                "lang": "en", "goal": "Make it.", "requirements": [{"text": "It builds.", "criterion": "it builds"}],
                "base": head, "dependencies": [], "allowed": ["src/**"], "profiles": {"build": "build", "review": "review"},
                "checks": [], "limits": {"rounds": 3, "dispatches": 6, "minutes": 120}, "delegationLanguage": "en",
                "settings": {},
            });
            for (k, v) in more.as_object().unwrap() {
                t[k] = v.clone();
            }
            t
        })
        .collect();
    fs::write(fx.root.join("plan.json"), serde_json::to_vec(&json!({"stage": s, "tasks": tasks})).unwrap()).unwrap();
    let ran = fx.run(&["propose", "plan.json", "--json"]);
    fs::remove_file(fx.root.join("plan.json")).unwrap();
    assert_eq!(ran.status, 0, "{ran:?}");
    let doc = ran.doc();
    let ids = doc["tasks"].as_array().unwrap().iter().map(|t| t["id"].as_str().unwrap().to_owned()).collect();
    (doc["stage"]["id"].as_str().unwrap().to_owned(), ids)
}

/// Authorizes `id`, a stage or a task, with the digest `status` shows; `more` are further arguments.
fn authorize(fx: &Fx, id: &str, more: &[&str]) -> Ran {
    let digest = fx.run(&["status", id, "--json"]).doc()["shown"]["digest"].as_str().unwrap().to_owned();
    fx.run(&[&["authorize", id, "--seen", &digest, "--say", SAY][..], more].concat())
}

/// One task proposed with `more` and authorized; gives its identifier.
fn authorized(fx: &Fx, more: Value) -> String {
    let id = plan(fx, json!({}), vec![more]).1.remove(0);
    let ran = authorize(fx, &id, &[]);
    assert_eq!(ran.status, 0, "{ran:?}");
    id
}

fn stream(id: &str) -> String {
    format!("task-{id}")
}

fn kinds(events: &[Value]) -> Vec<&str> {
    events.iter().map(|e| e["kind"].as_str().unwrap()).collect()
}

/// `run <id>`, which halts and waits for a person (exit 4); gives the events it added to the task's stream.
fn halts(fx: &Fx, id: &str, code: &str) -> Vec<Value> {
    let before = fx.events(&stream(id)).len();
    let ran = fx.run(&["run", id, "--say", SAY]);
    assert_eq!(ran.status, 4, "{ran:?}");
    whole(&ran.all());
    let words = cli::say("es", "run.halted", &[("id", id), ("code", code)]);
    assert!(ran.stdout.contains(&words), "{code}: {ran:?}");
    let events = fx.events(&stream(id))[before..].to_vec();
    let halt = events.last().unwrap();
    assert_eq!(
        (halt["kind"].as_str(), halt["payload"]["code"].as_str()),
        (Some("task-halted"), Some(code)),
        "{events:?}"
    );
    // The record holds no path of this machine (the reason names the evidence relative to the repository).
    let reason = halt["payload"]["reason"].as_str().unwrap();
    let base = fx.root.parent().unwrap().to_str().unwrap();
    assert!(!reason.contains(base) && !reason.contains(fx.home.to_str().unwrap()), "{halt}");
    ok(&fx.run(&["check"]));
    events
}

fn ok(ran: &Ran) {
    assert_eq!(ran.status, 0, "{ran:?}");
}

/// The directories of the runs' cells and evidence whose names start with `prefix`.
fn dirs(fx: &Fx, area: &str, prefix: &str) -> Vec<PathBuf> {
    let dir = fx.domain.join("local").join(area);
    let mut out: Vec<PathBuf> = fs::read_dir(dir).map(|d| d.map(|e| e.unwrap().path()).collect()).unwrap_or_default();
    out.retain(|p| p.file_name().unwrap().to_str().unwrap().starts_with(prefix));
    out.sort();
    out
}

/// Waits until `f` holds, at most a minute.
fn until(what: &str, f: impl Fn() -> bool) {
    let start = Instant::now();
    while !f() {
        assert!(start.elapsed() < Duration::from_secs(60), "waited for {what}");
        std::thread::sleep(Duration::from_millis(50));
    }
}

// ---- The run to its candidate (REQ-RUN-1, REQ-RUN-4, S-5, D-082) ---------------------------------------------------

#[test]
fn a_fake_builder_reaches_a_recorded_candidate() {
    let fx = Fx::builder("candidate", "published", &format!("{WRITES}{SUCCESS}"));
    let id = authorized(&fx, json!({}));
    let base = fx.events(&stream(&id))[0]["payload"]["base"].as_str().unwrap().to_owned();
    let before = fx.events(&stream(&id)).len();
    let head = fx.git(&["rev-parse", "HEAD"]);
    let new = halts(&fx, &id, "CONFINEMENT_UNAVAILABLE");
    assert_eq!(kinds(&new), ["run-started", "attempt-started", "attempt-ended", "candidate-recorded", "task-halted"]);
    // The run binds the stream's head, the authorization it relies on and its base, with the person's words.
    let authorization = fx.events(&stream(&id))[before - 1].clone();
    let digest = |seq: usize| sha256_hex(&fs::read(event_file(&fx, &stream(&id), seq)).unwrap());
    let at = json!({"seq": before, "digest": digest(before)});
    assert_eq!(authorization["kind"], "task-authorized");
    assert_eq!(
        new[0]["payload"],
        json!({"task": id, "head": at, "authorization": {"stream": stream(&id), "seq": before, "digest": digest(before)}, "base": base})
    );
    assert_eq!(new[0]["say"]["text"], SAY);
    // The attempt: its role, profile, tool and the version it reported (PD-29), the model and effort requested, and the time it is allowed: the profile's
    // minute (REQ-RUN-1).
    assert_eq!(
        new[1]["payload"],
        json!({"role": "build", "profile": "build", "tool": "claude", "version": "2.1.316 (Claude Code)", "model": "claude-sonnet-5-5", "effort": "low", "allowedSeconds": 60})
    );
    // Its end: the model observed, Fabrewk's seconds and the tool's tokens (S-5), and the digests of what it printed,
    // kept in the evidence store, readable by its owner only (REQ-REG-13, PD-28).
    let end = &new[2]["payload"];
    let attempt = before as u64 + 2;
    assert_eq!((end["attempt"].as_u64(), end["outcome"].as_str()), (Some(attempt), Some("success")));
    assert_eq!(end["model"], "claude-sonnet-5-5");
    assert!(end["usage"]["seconds"].is_u64(), "{end}");
    assert_eq!(end["usage"]["tokens"], json!({"input": 12, "output": 5, "cached": 7}));
    let evidence = fx.domain.join(format!("local/evidence/attempt-{id}-{attempt}"));
    assert_eq!(fs::metadata(&evidence).unwrap().permissions().mode() & 0o777, 0o700);
    for channel in ["stdout", "stderr"] {
        let file = evidence.join(channel);
        assert_eq!(fs::metadata(&file).unwrap().permissions().mode() & 0o777, 0o600);
        assert_eq!(end["evidence"][channel], sha256_hex(&fs::read(&file).unwrap()), "{channel}");
    }
    assert!(fs::read_to_string(evidence.join("stdout")).unwrap().contains("Done."));
    // The candidate: its commit on `fabrewk/<task>`, whose parent is the base, its tree and its change set (REQ-RUN-4,
    // PD-45).
    let candidate = &new[3]["payload"];
    let commit = fx.git(&["rev-parse", &format!("fabrewk/{id}")]);
    assert_eq!(candidate["commit"], commit);
    assert_eq!(candidate["tree"], fx.git(&["rev-parse", &format!("{commit}^{{tree}}")]));
    assert_eq!(fx.git(&["rev-parse", &format!("{commit}^")]), base);
    let oid = fx.git(&["rev-parse", &format!("{commit}:src/new.txt")]);
    assert_eq!(
        candidate["changes"],
        json!([{"path": "src/new.txt", "before": null, "after": {"mode": "100644", "oid": oid}}])
    );
    let message = fx.git(&["log", "-1", "--format=%B", &commit]);
    assert_eq!(
        message.lines().filter(|l| l.starts_with("Co-authored-by: ")).collect::<Vec<_>>(),
        ["Co-authored-by: claude (anthropic, claude-sonnet-5-5) <noreply@fabrewk.invalid>"]
    );
    assert!(!message.contains("Signed-off-by"), "{message}");
    // Until part 9f the task halts on its candidate (D-082), and says where it is.
    let reason = new[4]["payload"]["reason"].as_str().unwrap();
    assert!(reason.contains(&format!("fabrewk/{id}")) && reason.contains("9f") && !reason.contains("kept"), "{reason}");
    // The attempt's directories are gone; the person's branch and working tree did not move.
    assert!(dirs(&fx, "tmp", "attempt-").is_empty());
    assert_eq!(fx.git(&["rev-parse", "HEAD~1"]), head);
    assert_eq!(fx.git(&["status", "--porcelain"]), "");
    // One record commit holds the run's five events (REQ-REG-10).
    let trailers = fx.git(&["log", "-1", "--format=%B"]).lines().filter(|l| l.starts_with("Fabrewk-Event: ")).count();
    assert_eq!(trailers, 5);
    // The backend's programs and the tool's preflight are kept as verified by use (3.9).
    let detected: Value =
        serde_json::from_slice(&fs::read(fx.domain.join("local/cache/detected.json")).unwrap()).unwrap();
    for (space, name) in [("tools", "claude"), ("programs", "bwrap"), ("programs", "pasta")] {
        assert!(detected[space][name]["verified"].is_string(), "{space} {name}: {detected}");
    }
    // `status` says why the task waits; the view shows its usage.
    let doc = fx.run(&["status", &id, "--json"]).doc();
    assert_eq!(
        (doc["state"].as_str(), doc["halt"]["code"].as_str()),
        (Some("halted"), Some("CONFINEMENT_UNAVAILABLE"))
    );
    let status = fs::read_to_string(fx.domain.join("STATUS.md")).unwrap();
    assert!(status.contains("; tokens read 12, written 5, read from the cache 7."), "{status}");
}

/// The file of the event at `seq` of `stream`.
fn event_file(fx: &Fx, stream: &str, seq: usize) -> PathBuf {
    let dir = fx.domain.join("data/events").join(stream);
    let mut files: Vec<PathBuf> = fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect();
    files.retain(|p| p.extension().unwrap() == "json");
    files.sort();
    files[seq - 1].clone()
}

#[test]
fn pd_45_the_candidate_carries_the_sign_off_the_project_requires() {
    let fx = Fx::builder_with("sign-off", "published", &format!("{WRITES}{SUCCESS}"), "\n[git]\nsignOff = true\n");
    let id = authorized(&fx, json!({}));
    halts(&fx, &id, "CONFINEMENT_UNAVAILABLE");
    let message = fx.git(&["log", "-1", "--format=%B", &format!("fabrewk/{id}")]);
    assert!(message.contains("\nSigned-off-by: Fixture <fixture@example.invalid>"), "{message}");
}

#[test]
fn authorize_run_and_run_of_a_stage_run_at_once() {
    let fx = Fx::builder("at-once", "published", &format!("{WRITES}{SUCCESS}"));
    // `authorize --run`: the authorization, then the run, with the same words; the document holds both.
    let id = plan(&fx, json!({}), vec![json!({})]).1.remove(0);
    let digest = fx.run(&["status", &id, "--json"]).doc()["shown"]["digest"].as_str().unwrap().to_owned();
    let ran = fx.run(&["authorize", &id, "--seen", &digest, "--say", SAY, "--run", "--json"]);
    assert_eq!(ran.status, 4, "{ran:?}");
    let doc = ran.doc();
    assert_eq!(
        (doc["schema"].as_str(), doc["run"]["schema"].as_str()),
        (Some("fabrewk.authorized/2"), Some("fabrewk.run/1"))
    );
    assert_eq!(doc["run"]["halt"]["code"], "CONFINEMENT_UNAVAILABLE");
    let events = fx.events(&stream(&id));
    assert_eq!(kinds(&events[1..3]), ["task-authorized", "run-started"]);
    assert_eq!(events[2]["payload"]["authorization"]["seq"], 2);
    assert_eq!(events[2]["say"], events[1]["say"]);
    // `run <stage>`: the stage's first task that may run.
    let (stage, ids) = plan(&fx, json!({}), vec![json!({}), json!({"goal": "Two."})]);
    ok(&authorize(&fx, &stage, &[]));
    let ran = fx.run(&["run", &stage, "--say", SAY, "--json"]);
    assert_eq!(ran.status, 4, "{ran:?}");
    let first = &ran.doc()["id"];
    assert!(ids.contains(&first.as_str().unwrap().to_owned()), "{first}");
    // Run by a stage's authorization, it relies on that one.
    let started = fx.events(&stream(first.as_str().unwrap()))[1].clone();
    assert_eq!(started["payload"]["authorization"]["stream"], format!("stage-{stage}"));
}

/// The run relies on the authorization in force, whichever it is (P9E-3): a stage's that came after the task's own,
/// and the task's own when a stage's authorization with `--run` does not bind it.
#[test]
fn a_run_relies_on_the_authorization_in_force() {
    let fx = Fx::builder("in-force", "published", &format!("{WRITES}{SUCCESS}"));
    let (stage, ids) = plan(&fx, json!({}), vec![json!({})]);
    ok(&authorize(&fx, &ids[0], &[]));
    halts(&fx, &ids[0], "CONFINEMENT_UNAVAILABLE");
    // Halted, the task is bound again by its stage's authorization; its run names that one.
    ok(&authorize(&fx, &stage, &[]));
    let new = halts(&fx, &ids[0], "CONFINEMENT_UNAVAILABLE");
    assert_eq!(new[0]["payload"]["authorization"]["stream"], format!("stage-{stage}"));
    // A task authorized alone, then its stage authorized with `--run`, which binds no task: the run relies on the
    // task's own.
    let (stage, ids) = plan(&fx, json!({}), vec![json!({})]);
    ok(&authorize(&fx, &ids[0], &[]));
    let ran = authorize(&fx, &stage, &["--run"]);
    assert_eq!(ran.status, 4, "{ran:?}");
    let events = fx.events(&stream(&ids[0]));
    let started = events.iter().find(|e| e["kind"] == "run-started").unwrap();
    assert_eq!(started["payload"]["authorization"]["stream"], stream(&ids[0]));
    ok(&fx.run(&["check"]));
}

// ---- The guards before the run, refusing or halting (REQ-RUN-2, REQ-TASK-3, REQ-TASK-5, S-6) -----------------------

#[test]
fn refusal_tool_missing() {
    let fx = Fx::builder("tool-missing", "published", SUCCESS);
    // A required tool that lives outside the system directories a cell shows (S-6), one absent, then one under
    // `/usr`, which passes.
    let tool = fx.root.parent().unwrap().join("tool/versions/mytool");
    fs::write(&tool, "#!/bin/sh\n").unwrap();
    fs::set_permissions(&tool, fs::Permissions::from_mode(0o755)).unwrap();
    for (required, why) in [("mytool", "outside the system directories"), ("no-such-tool", "not present")] {
        let id = authorized(&fx, json!({"settings": {"profiles.build.requiredTools": [required]}}));
        let e = refused(&fx, &["run", &id, "--say", SAY], "TOOL_MISSING", 1, "es");
        assert!(e["detail"].as_str().unwrap().contains(why), "{e}");
        assert!(dirs(&fx, "evidence", "attempt-").is_empty());
    }
    let id = authorized(&fx, json!({"settings": {"profiles.build.requiredTools": ["sh"]}}));
    halts(&fx, &id, "CONFINEMENT_UNAVAILABLE");
    // The tool itself, absent from `PATH`.
    let id = authorized(&fx, json!({}));
    let path = path_with(&fx.root.parent().unwrap().join("without"), &["git", "ssh-keygen", "df", "bwrap", "pasta"]);
    let before = fx.snapshot();
    let ran = fx.run_with(&["run", &id, "--say", SAY], Some(&path));
    assert_eq!((ran.status, fx.snapshot() == before), (1, true), "{ran:?}");
    assert!(ran.stderr.contains("TOOL_MISSING"), "{ran:?}");
}

#[test]
fn refusal_budget_exhausted() {
    let fx = Fx::builder("budget", "published", &format!("{WRITES}{SUCCESS}"));
    // The stage's dispatches, then its tokens (12 read and 5 written by the first run), then tokens it cannot know.
    for (budget, why) in [
        (json!({"dispatches": 1, "minutes": 120}), "1 of 1 dispatches spent"),
        (json!({"dispatches": 6, "minutes": 120, "tokens": 17}), "17 of 17 tokens spent"),
    ] {
        let (stage, ids) = plan(&fx, json!({"budget": budget}), vec![json!({}), json!({"goal": "Two."})]);
        ok(&authorize(&fx, &stage, &[]));
        halts(&fx, &ids[0], "CONFINEMENT_UNAVAILABLE");
        let attempts = dirs(&fx, "evidence", "attempt-").len();
        let new = halts(&fx, &ids[1], "BUDGET_EXHAUSTED");
        // Nothing is dispatched: no run started, no attempt.
        assert_eq!(kinds(&new), ["task-halted"]);
        assert!(new[0]["payload"]["reason"].as_str().unwrap().contains(why), "{new:?}");
        assert_eq!(dirs(&fx, "evidence", "attempt-").len(), attempts);
        // Authorized again, the task's own limits are granted anew, but not the stage's.
        ok(&authorize(&fx, &ids[1], &[]));
        halts(&fx, &ids[1], "BUDGET_EXHAUSTED");
    }
    fx.fake(&format!("{WRITES}{}", SUCCESS.replace(r#","usage":{"input_tokens":10,"cache_creation_input_tokens":2,"output_tokens":5,"cache_read_input_tokens":7}"#, "")));
    let (stage, ids) = plan(
        &fx,
        json!({"budget": {"dispatches": 6, "minutes": 120, "tokens": 1000}}),
        vec![json!({}), json!({"goal": "Two."})],
    );
    ok(&authorize(&fx, &stage, &[]));
    let new = halts(&fx, &ids[0], "CONFINEMENT_UNAVAILABLE");
    assert_eq!(new[2]["payload"]["usage"]["tokens"], Value::Null);
    let new = halts(&fx, &ids[1], "BUDGET_EXHAUSTED");
    assert!(new[0]["payload"]["reason"].as_str().unwrap().contains("unknown"), "{new:?}");
}

#[test]
fn refusal_confinement_unavailable() {
    let fx = Fx::builder("confinement", "published", &format!("{WRITES}{SUCCESS}"));
    // No `bwrap` nor `pasta` on this machine's `PATH`: the backend is unavailable, and the run halts before it starts.
    let id = authorized(&fx, json!({}));
    let dir = fx.root.parent().unwrap().join("tool/versions");
    let bin = path_with(&fx.root.parent().unwrap().join("nobackend"), &["git", "ssh-keygen", "df", "socat"]);
    let path = std::env::join_paths([dir.as_os_str(), bin.as_os_str()]).unwrap();
    let before = fx.events(&stream(&id)).len();
    let ran = fx.run_with(&["run", &id, "--say", SAY], Some(&path));
    assert_eq!(ran.status, 4, "{ran:?}");
    let new = &fx.events(&stream(&id))[before..];
    assert_eq!(kinds(new), ["task-halted"]);
    assert_eq!(new[0]["payload"]["code"], "CONFINEMENT_UNAVAILABLE");
    assert!(new[0]["payload"]["reason"].as_str().unwrap().contains("bwrap"), "{new:?}");
    // The tool's preflight fails inside its cell (its version, there only): the run halts and names the evidence.
    fx.fake_raw(&format!(
        "case \"$1\" in --version) [ \"$HOME\" = /home/agent ] && exit 7; echo '2.1.316 (Claude Code)'; exit 0;; esac\n{WRITES}{SUCCESS}"
    ));
    let id = authorized(&fx, json!({}));
    let new = halts(&fx, &id, "CONFINEMENT_UNAVAILABLE");
    assert_eq!(kinds(&new), ["task-halted"]);
    let reason = new[0]["payload"]["reason"].as_str().unwrap();
    assert!(reason.contains("preflight") && reason.contains(".brewstillery/local/evidence/preflight-"), "{reason}");
    // Repaired, the preflight runs again, passes, and is kept: the next run's tool is verified by use.
    fx.fake(&format!("{WRITES}{SUCCESS}"));
    let id = authorized(&fx, json!({}));
    let preflights = dirs(&fx, "evidence", "preflight-").len();
    halts(&fx, &id, "CONFINEMENT_UNAVAILABLE");
    let now = dirs(&fx, "evidence", "preflight-").len();
    assert!(now > preflights, "{preflights} {now}");
    let id = authorized(&fx, json!({}));
    let new = halts(&fx, &id, "CONFINEMENT_UNAVAILABLE");
    assert_eq!(new[3]["kind"], "candidate-recorded");
    assert_eq!(dirs(&fx, "evidence", "preflight-").len(), now);
}

#[test]
fn refusal_disk_low() {
    let fx = Fx::builder_with("disk", "published", SUCCESS, "\n[limits]\nfreeDiskMegabytes = 999999999\n");
    let id = authorized(&fx, json!({}));
    let new = halts(&fx, &id, "DISK_LOW");
    assert_eq!(kinds(&new), ["task-halted"]);
}

#[test]
fn a_packet_longer_than_an_argument_may_be_halts_before_any_dispatch() {
    let fx = Fx::builder("packet", "published", SUCCESS);
    let id = authorized(&fx, json!({"goal": "x".repeat(125_000)}));
    let new = halts(&fx, &id, "TOOL_FAILED");
    assert_eq!(kinds(&new), ["task-halted"]);
    assert!(new[0]["payload"]["reason"].as_str().unwrap().contains("packet"), "{new:?}");
}

#[test]
fn refusal_base_moved_and_a_dependent_tasks_base_fixed_at_its_run() {
    let fx = Fx::builder("base", "published", &format!("{WRITES}{SUCCESS}"));
    let (stage, ids) = plan(
        &fx,
        json!({}),
        vec![json!({"name": "first"}), json!({"goal": "Second.", "base": null, "dependencies": ["first"]})],
    );
    ok(&authorize(&fx, &stage, &[]));
    halts(&fx, &ids[0], "CONFINEMENT_UNAVAILABLE");
    // The first is accepted and integrated by hand (part 9g builds it): the record branch takes its candidate's file,
    // in a commit that names it (REQ-GATE-8).
    integrate(&fx, &ids[0]);
    // The second's base is the record branch's head, whose product is what the integration left.
    let new = halts(&fx, &ids[1], "CONFINEMENT_UNAVAILABLE");
    assert_eq!(new[0]["payload"]["base"], fx.git(&["rev-parse", "HEAD~1"]));
    // Another change of the product after the integration: refused, with nothing written.
    ok(&authorize(&fx, &ids[1], &[]));
    fs::write(fx.root.join("README.md"), "moved\n").unwrap();
    fx.commit("a person's change");
    refused(&fx, &["run", &ids[1], "--say", SAY], "BASE_MOVED", 1, "es");
    // The domain's own commits do not move it.
    fs::write(fx.root.join("README.md"), "fixture\n").unwrap();
    fx.commit("back");
    halts(&fx, &ids[1], "CONFINEMENT_UNAVAILABLE");
}

/// Integrates `id`'s candidate by hand, as part 9g will: accepted, then its file brought into the record branch in a
/// commit that names the candidate in a `Fabrewk-Candidate` trailer (REQ-GATE-8), then `task-integrated`.
fn integrate(fx: &Fx, id: &str) {
    let s = stream(id);
    let head = fx.events(&s).len();
    let digest = sha256_hex(&fs::read(event_file(fx, &s, head)).unwrap());
    by_hand(fx, &s, "task-accepted", json!({"task": id, "head": {"seq": head, "digest": digest}}), true);
    let candidate = fx.git(&["rev-parse", &format!("fabrewk/{id}")]);
    let files = fx.git(&["diff", "--name-only", &format!("{candidate}^"), &candidate]);
    for f in files.lines() {
        fx.git(&["checkout", &candidate, "--", f]);
    }
    fx.git(&["add", "-A"]);
    fx.git(&["commit", "-q", "-m", &format!("integrated by hand\n\nFabrewk-Candidate: {candidate}")]);
    by_hand(fx, &s, "task-integrated", json!({}), false);
}

/// The last of a task's dependencies' integrations is read from the record branch's history, never from the clock
/// (P9E-9): the product must be the one the latest left, and an integration the history does not hold is none.
#[test]
fn refusal_base_moved_reads_the_last_integration_from_the_history() {
    let fx = Fx::builder("base-chain", "published", SUCCESS);
    fx.fake(&format!("mkdir -p src && printf 'a\\n' > src/a.txt\n{SUCCESS}"));
    let (stage, ids) = plan(
        &fx,
        json!({}),
        vec![
            json!({"name": "a"}),
            json!({"name": "b", "goal": "B.", "base": null, "dependencies": ["a"]}),
            json!({"goal": "C.", "base": null, "dependencies": ["a", "b"]}),
        ],
    );
    ok(&authorize(&fx, &stage, &[]));
    halts(&fx, &ids[0], "CONFINEMENT_UNAVAILABLE");
    integrate(&fx, &ids[0]);
    fx.fake(&format!("mkdir -p src && printf 'b\\n' > src/b.txt\n{SUCCESS}"));
    halts(&fx, &ids[1], "CONFINEMENT_UNAVAILABLE");
    integrate(&fx, &ids[1]);
    // What the later integration left removed: the product is the earlier one's, which is not the last.
    fx.git(&["rm", "-q", "src/b.txt"]);
    fx.commit("b removed");
    refused(&fx, &["run", &ids[2], "--say", SAY], "BASE_MOVED", 1, "es");
    fx.git(&["revert", "--no-edit", "HEAD"]);
    halts(&fx, &ids[2], "CONFINEMENT_UNAVAILABLE");
    // The history, not the clock, says which came last: a commit naming the first's candidate after the second's
    // integration, its product the first's, makes the first the last, though its event is older.
    ok(&authorize(&fx, &ids[2], &[]));
    let first = fx.git(&["rev-parse", &format!("fabrewk/{}", ids[0])]);
    fx.git(&["rm", "-q", "src/b.txt"]);
    fx.git(&["commit", "-q", "-m", &format!("the first again\n\nFabrewk-Candidate: {first}")]);
    halts(&fx, &ids[2], "CONFINEMENT_UNAVAILABLE");
}

/// Appends to `stream` an event of `kind`, a maestro's (signed) or Fabrewk's, as a later part's command will; commits it.
fn by_hand(fx: &Fx, stream: &str, kind: &str, payload: Value, person: bool) {
    let mut s = Session::begin(&fx.domain, None, &PolicyVerifier).unwrap();
    let head = s.head(stream).unwrap();
    let policy = fx.events("project").last().unwrap()["policy"].as_str().unwrap().to_owned();
    let (actor, say) = match person {
        true => (
            json!({"kind": "person", "alias": "xel", "role": "maestro"}),
            Some(fabrewk_core::record::Say { text: "Lo decido.".into(), lang: "es".into() }),
        ),
        false => (json!({"kind": "fabrewk"}), None),
    };
    let key = fx.signer();
    let signer: Option<&dyn fabrewk_core::record::Sign> = if person { Some(&key) } else { None };
    let draft = Draft { kind: kind.into(), actor, policy, instance: "SHA256:fixture".into(), say, payload };
    s.append(stream, head.as_ref(), &draft, signer).unwrap();
    s.finish(&PolicyVerifier, &[]).unwrap();
    fx.regen();
    fx.commit(kind);
}

// ---- The attempt: its copy, its dispatch, its end (REQ-RUN-3, REQ-RUN-6, REQ-REG-13, D-079) --------------------------

#[test]
fn pd_21_a_failed_export_is_recorded_before_any_dispatch_and_a_link_is_exported_as_a_link() {
    let fx = Fx::builder("pd-21", "published", &format!("{WRITES}{SUCCESS}"));
    // A base the repository does not hold.
    let id = authorized(&fx, json!({"base": "0".repeat(40)}));
    let new = halts(&fx, &id, "TOOL_FAILED");
    assert_eq!(kinds(&new), ["run-started", "attempt-started", "attempt-ended", "task-halted"]);
    let end = &new[2]["payload"];
    assert_eq!((end["outcome"].as_str(), &end["evidence"]), (Some("failure"), &Value::Null));
    let evidence = &dirs(&fx, "evidence", "attempt-")[0];
    let reason = new[3]["payload"]["reason"].as_str().unwrap();
    let shown = evidence.strip_prefix(&fx.root).unwrap().display().to_string();
    assert!(reason.contains(&shown), "{reason}");
    assert!(evidence.join("export").is_file() && !evidence.join("stdout").exists());
    assert_eq!(fs::metadata(evidence.join("export")).unwrap().permissions().mode() & 0o777, 0o600);
    // A base holding a link to an absolute path: exported as a link, untouched, and no change.
    std::os::unix::fs::symlink("/etc/passwd", fx.root.join("passwd")).unwrap();
    fx.commit("a link");
    let id = authorized(&fx, json!({}));
    let new = halts(&fx, &id, "CONFINEMENT_UNAVAILABLE");
    assert_eq!(new[3]["payload"]["changes"].as_array().unwrap().len(), 1, "{:?}", new[3]);
}

#[test]
fn d_079_a_login_that_expires_within_the_attempt_halts_before_the_tool_starts() {
    let fx = Fx::builder("expiry", "published", SUCCESS);
    let soon = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() + 30_000;
    fs::write(fx.home.join(".claude/.credentials.json"), format!("{{\"claudeAiOauth\":{{\"expiresAt\":{soon}}}}}"))
        .unwrap();
    let id = authorized(&fx, json!({}));
    let new = halts(&fx, &id, "TOOL_FAILED");
    assert_eq!(kinds(&new), ["run-started", "attempt-started", "attempt-ended", "task-halted"]);
    assert!(new[3]["payload"]["reason"].as_str().unwrap().contains("renew"), "{new:?}");
    // The tool never started: its output channels were never opened.
    assert!(!dirs(&fx, "evidence", "attempt-")[0].join("stdout").exists());
    // A credential whose expiry cannot be read, or that is absent: the reason names it from the home, not where the
    // home lies on this machine (P9E-2).
    for credential in [Some("{}"), None] {
        let path = fx.home.join(".claude/.credentials.json");
        match credential {
            Some(c) => fs::write(&path, c).unwrap(),
            None => fs::remove_file(&path).unwrap(),
        }
        let id = authorized(&fx, json!({}));
        let new = halts(&fx, &id, "TOOL_FAILED");
        assert!(new[3]["payload"]["reason"].as_str().unwrap().contains("~/.claude/.credentials.json"), "{new:?}");
    }
}

#[test]
fn the_change_set_halts_a_candidate_it_refuses() {
    let fx = Fx::builder("lanes", "published", SUCCESS);
    for (script, code) in [
        ("printf 'x\\n' > README.md\n", "LANE_VIOLATION"),
        ("mkdir -p src/.git && printf 'x\\n' > src/.git/config\n", "PROTECTED_PATH"),
        (
            "mkdir -p src && printf -- '-----BEGIN OPENSSH PRIVATE KEY-----\\n' > src/id_ed25519\n",
            "CREDENTIAL_SUSPECTED",
        ),
    ] {
        fx.fake(&format!("{script}{SUCCESS}"));
        let id = authorized(&fx, json!({}));
        let new = halts(&fx, &id, code);
        assert_eq!(kinds(&new), ["run-started", "attempt-started", "attempt-ended", "task-halted"], "{code}");
        assert_eq!(new[2]["payload"]["outcome"], "success");
        assert!(fx.git(&["branch", "--list", &format!("fabrewk/{id}")]).is_empty(), "{code}");
    }
    assert!(dirs(&fx, "tmp", "attempt-").is_empty());
}

/// A failure of Git while the candidate is made, after the builder succeeded (P9E-11): the halt names evidence of its
/// own, holding the core's words, never the builder's output.
#[test]
fn a_candidate_git_cannot_make_halts_naming_its_own_evidence() {
    let fx = Fx::builder("git-fails", "published", &format!("{WRITES}{SUCCESS}"));
    let id = authorized(&fx, json!({}));
    let lock = fx.root.join(format!(".git/refs/heads/fabrewk/{id}.lock"));
    fs::create_dir_all(lock.parent().unwrap()).unwrap();
    fs::write(&lock, "").unwrap();
    let new = halts(&fx, &id, "TOOL_FAILED");
    assert_eq!(new[2]["payload"]["outcome"], "success");
    let reason = new[3]["payload"]["reason"].as_str().unwrap();
    let evidence = dirs(&fx, "evidence", &format!("attempt-{id}"))[0].join("candidate");
    let shown = evidence.strip_prefix(&fx.root).unwrap().display().to_string();
    assert!(reason.contains(&shown) && !reason.contains("its output is kept"), "{reason}");
    assert_eq!(fs::metadata(&evidence).unwrap().permissions().mode() & 0o777, 0o600);
    fs::remove_file(lock).unwrap();
}

/// A task's layer that a policy adopted after its authorization makes a widening is not granted by that authorization
/// (P9E-12): the run refuses it, with nothing written, until an authorization names it.
#[test]
fn refusal_widening_unnamed_at_run_after_a_stricter_policy() {
    let fx = Fx::builder("widening", "published", &format!("{WRITES}{SUCCESS}"));
    let paths = json!(["Cargo.toml", "Cargo.lock", ".github/"]);
    let id = authorized(&fx, json!({"settings": {"gate.protectedPaths": paths}}));
    let text = fs::read_to_string(fx.domain.join("settings.toml")).unwrap();
    fx.edit_policy(&format!(
        "{text}\n[gate]\nprotectedPaths = [\"Cargo.toml\", \"Cargo.lock\", \".github/\", \"src/secret.txt\"]\n"
    ));
    let digest = fx.run(&["status", "--json"]).doc()["policy"]["digest"].clone();
    let ran = fx.run(&["adopt", "--say", "La ajusto."]);
    assert_eq!(ran.status, 0, "{ran:?} {digest}");
    let e = refused(&fx, &["run", &id, "--say", SAY], "WIDENING_UNNAMED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("gate.protectedPaths"), "{e}");
}

#[test]
fn pd_28_pd_29_a_failed_builders_output_is_evidence_and_never_enters_the_record() {
    let fx = Fx::builder("pd-28", "published", SUCCESS);
    let error = r#"echo '{"type":"system","subtype":"init","model":"claude-sonnet-5-5"}'
echo '{"type":"result","subtype":"error_MARKER-ONE","is_error":true,"result":"MARKER-TWO","usage":{"input_tokens":1,"cache_creation_input_tokens":0,"output_tokens":1,"server_tool_use":"MARKER-THREE"}}'
echo MARKER-FOUR >&2
"#;
    for (script, outcome, why) in [
        (format!("{error}exit 3\n"), "failure", "exited with status 3"),
        (error.to_owned(), "failure", "gave no result"),
        (String::new(), "failure", "gave no result"),
        (SUCCESS.replace(r#"{"summary":"Done."}"#, r#"{"wrong":1}"#), "failure", "does not fit"),
    ] {
        fx.fake(&script);
        let id = authorized(&fx, json!({}));
        let new = halts(&fx, &id, "TOOL_FAILED");
        let end = &new[2]["payload"];
        assert_eq!(end["outcome"], outcome, "{why}");
        let reason = new[3]["payload"]["reason"].as_str().unwrap();
        assert!(reason.contains(why) && reason.contains(".brewstillery/local/evidence/attempt-"), "{reason}");
        let evidence = dirs(&fx, "evidence", &format!("attempt-{id}"))[0].clone();
        for channel in ["stdout", "stderr"] {
            assert_eq!(end["evidence"][channel], sha256_hex(&fs::read(evidence.join(channel)).unwrap()));
        }
        if !script.is_empty() && why != "does not fit" {
            // The model as the stream reported it; the error's text, subtype, standard error and free-form usage only in
            // the evidence (PD-29).
            assert_eq!(end["model"], "claude-sonnet-5-5");
            let kept = fs::read_to_string(evidence.join("stdout")).unwrap()
                + &fs::read_to_string(evidence.join("stderr")).unwrap();
            for marker in ["MARKER-ONE", "MARKER-TWO", "MARKER-THREE", "MARKER-FOUR"] {
                assert!(kept.contains(marker), "{marker}");
            }
        }
    }
    let mut todo = vec![fx.domain.join("data")];
    while let Some(dir) = todo.pop() {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                todo.push(path);
            } else {
                assert!(!fs::read_to_string(&path).unwrap_or_default().contains("MARKER"), "{}", path.display());
            }
        }
    }
}

#[test]
fn pd_39_tokens_are_recorded_as_reported() {
    let fx = Fx::builder("pd-39", "published", SUCCESS);
    let id = authorized(&fx, json!({}));
    // No cached tokens reported, then zero.
    let usage =
        r#""usage":{"input_tokens":10,"cache_creation_input_tokens":2,"output_tokens":5,"cache_read_input_tokens":7}"#;
    for (reported, cached) in [
        (r#""usage":{"input_tokens":1,"cache_creation_input_tokens":0,"output_tokens":1}"#, Value::Null),
        (
            r#""usage":{"input_tokens":1,"cache_creation_input_tokens":0,"output_tokens":1,"cache_read_input_tokens":0}"#,
            json!(0),
        ),
    ] {
        fx.fake(&SUCCESS.replace(usage, reported));
        let new = halts(&fx, &id, "CONFINEMENT_UNAVAILABLE");
        assert_eq!(new[2]["payload"]["usage"]["tokens"]["cached"], cached);
        ok(&authorize(&fx, &id, &[]));
    }
    // The view sums them: one part not reported, so the cache's sum is unknown; the others are figures.
    let status = fs::read_to_string(fx.domain.join("STATUS.md")).unwrap();
    let unknown = cli::say("en", "view.unknown", &[]);
    assert!(status.contains(&format!("tokens read 2, written 2, read from the cache {unknown}.")), "{status}");
}

// ---- A run that ends without Fabrewk (REQ-RUN-5, PD-08, PD-10, REQ-REG-5, REQ-VIEW-4) --------------------------------

/// A fake builder that waits in its copy until the test lets it go, then succeeds.
const WAITS: &str = "until [ -e /work/go ]; do sleep 0.1; done\n";

/// Starts `run <id>` and waits until its builder runs; gives the process and its copy.
fn started(fx: &Fx, id: &str) -> (std::process::Child, PathBuf) {
    let child = fx.spawn(&["run", id, "--say", SAY]);
    until("the builder", || dirs(fx, "tmp", &format!("attempt-{id}")).iter().any(|d| d.join("work").is_dir()));
    let work = dirs(fx, "tmp", &format!("attempt-{id}")).pop().unwrap().join("work");
    (child, work)
}

#[test]
fn pd_08_pd_10_a_killed_run_is_ended_unknown_by_the_next_write_command() {
    let fx = Fx::builder("pd-08", "published", &format!("{WAITS}{WRITES}{SUCCESS}"));
    let (stage, ids) = plan(&fx, json!({}), vec![json!({}), json!({"goal": "Two."})]);
    ok(&authorize(&fx, &stage, &[]));
    let (mut child, _) = started(&fx, &ids[0]);
    // The attempt is on disk before the builder runs (PD-10); `status` reads while the run holds the lock
    // (REQ-VIEW-4), and another run is refused by that lock (REQ-REG-5, REQ-TASK-3).
    assert_eq!(kinds(&fx.events(&stream(&ids[0]))).last(), Some(&"attempt-started"));
    assert_eq!(fx.run(&["status", &ids[0], "--json"]).doc()["state"], "running");
    // Nor does `status` walk the attempt's directories while it runs (REQ-RUN-3, P9E-1): their size is unknown.
    let doc = fx.run(&["status", "--json"]).doc();
    assert_eq!((doc["local"]["tmp"].clone(), doc["local"]["evidence"].is_u64()), (Value::Null, true), "{doc}");
    refused(&fx, &["run", &ids[1], "--say", SAY], "LOCK_HELD", 1, "es");
    // Killed before the attempt's end (PD-08).
    child.kill().unwrap();
    child.wait().unwrap();
    assert_eq!(fx.run(&["status", &ids[0], "--json"]).doc()["state"], "running");
    // The next write command ends it as unknown, counts it, halts the task, removes its copy; then makes its own move.
    let before = fx.events(&stream(&ids[0])).len();
    ok(&fx.run(&["reject", &ids[1], "--say", SAY]));
    let new = &fx.events(&stream(&ids[0]))[before..];
    assert_eq!(kinds(new), ["attempt-ended", "task-halted"]);
    assert_eq!(
        new[0]["payload"],
        json!({"attempt": before, "outcome": "unknown", "model": null, "usage": {"seconds": null, "tokens": null}, "evidence": null})
    );
    assert_eq!(new[1]["payload"]["code"], "OUTCOME_UNKNOWN");
    assert!(dirs(&fx, "tmp", "attempt-").is_empty());
    assert!(fx.run(&["status", "--json"]).doc()["local"]["tmp"].is_u64());
    // A cell's directories kept unread, its task no longer running (a backend that could not show it ended): not
    // walked either (P9E-7).
    let kept = fx.domain.join("local/tmp/attempt-kept/work");
    fs::create_dir_all(&kept).unwrap();
    assert_eq!(fx.run(&["status", "--json"]).doc()["local"]["tmp"], Value::Null);
    fs::remove_dir_all(kept.parent().unwrap()).unwrap();
    // `status` says the attempt's result is unknown and that a retry spends a dispatch; the view, that its time is.
    let human = fx.run(&["status", &ids[0]]);
    assert!(human.stdout.contains(&cli::say("es", "code.OUTCOME_UNKNOWN.reason", &[])), "{human:?}");
    let view = fs::read_to_string(fx.domain.join("STATUS.md")).unwrap();
    let unknown = cli::say("en", "view.unknown", &[]);
    assert!(view.contains(&format!("1 dispatches, {unknown} seconds")), "{view}");
    // Nothing runs again without a person: authorized again, the retry is a new attempt.
    fx.fake(&format!("{WRITES}{SUCCESS}"));
    ok(&authorize(&fx, &ids[0], &[]));
    let new = halts(&fx, &ids[0], "CONFINEMENT_UNAVAILABLE");
    assert_eq!(new[3]["kind"], "candidate-recorded");
    // A late result for the attempt the retry replaced is refused (ATTEMPT_REPLACED), at the replay.
    let mut s = Session::begin(&fx.domain, None, &PolicyVerifier).unwrap();
    let head = s.head(&stream(&ids[0])).unwrap();
    let policy = fx.events("project").last().unwrap()["policy"].as_str().unwrap().to_owned();
    let mut late = json!({"attempt": before, "outcome": "success", "model": null, "usage": {"seconds": 1, "tokens": null}, "evidence": null});
    late["attempt"] = json!(before);
    let d = Draft {
        kind: "attempt-ended".into(),
        actor: json!({"kind": "fabrewk"}),
        policy,
        instance: "SHA256:fixture".into(),
        say: None,
        payload: late,
    };
    s.append(&stream(&ids[0]), head.as_ref(), &d, None).unwrap();
    drop(s);
    let doc = fx.run(&["check", "--json"]).doc();
    assert!(doc["findings"].as_array().unwrap().iter().any(|f| f["code"] == "ATTEMPT_REPLACED"), "{doc}");
}

#[test]
fn req_run_7_an_attempt_ended_unknown_counts_its_whole_time_against_the_minutes() {
    let fx = Fx::builder("unknown-minutes", "published", &format!("{WAITS}{SUCCESS}"));
    let (stage, ids) =
        plan(&fx, json!({"budget": {"dispatches": 6, "minutes": 1}}), vec![json!({}), json!({"goal": "Two."})]);
    ok(&authorize(&fx, &stage, &[]));
    let (mut child, _) = started(&fx, &ids[0]);
    child.kill().unwrap();
    child.wait().unwrap();
    // Recovered by the next write command (here a run that the minutes then refuse): its minute, which the profile
    // allowed, is spent whole.
    let new = halts(&fx, &ids[1], "BUDGET_EXHAUSTED");
    assert!(new[0]["payload"]["reason"].as_str().unwrap().contains("60 seconds"), "{new:?}");
    assert_eq!(fx.events(&stream(&ids[0])).last().unwrap()["payload"]["code"], "OUTCOME_UNKNOWN");
}
