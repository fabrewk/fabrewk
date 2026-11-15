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

/// `run <id>`, which verifies the task (exit 0, REQ-GATE-5); gives the events it added to the task's stream.
fn verifies(fx: &Fx, id: &str) -> Vec<Value> {
    let before = fx.events(&stream(id)).len();
    let ran = fx.run(&["run", id, "--say", SAY]);
    assert_eq!(ran.status, 0, "{ran:?}");
    whole(&ran.all());
    assert!(ran.stdout.contains(&cli::say("es", "run.verified", &[("id", id)])), "{ran:?}");
    let events = fx.events(&stream(id))[before..].to_vec();
    assert_eq!(events.last().unwrap()["kind"], "task-verified", "{events:?}");
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
fn a_fake_builder_and_reviewer_verify_a_task() {
    let fx = Fx::builder("candidate", "published", &format!("{WRITES}{SUCCESS}"));
    let id = authorized(&fx, json!({}));
    let base = fx.events(&stream(&id))[0]["payload"]["base"].as_str().unwrap().to_owned();
    let before = fx.events(&stream(&id)).len();
    let head = fx.git(&["rev-parse", "HEAD"]);
    let new = verifies(&fx, &id);
    assert_eq!(
        kinds(&new),
        [
            "run-started",
            "attempt-started",
            "attempt-ended",
            "candidate-recorded",
            "attempt-started",
            "attempt-ended",
            "review-recorded",
            "task-verified"
        ]
    );
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
    // With no required check, the reviewer is dispatched on the same candidate as a new attempt (REQ-GATE-4), its
    // review recorded with the audit's count and bound to the candidate's tree and the requirements (REQ-GATE-5).
    assert_eq!(new[4]["payload"]["role"], "review");
    let tree = candidate["tree"].clone();
    let requirements = sha256_hex(b"[{\"criterion\":\"it builds\",\"text\":\"It builds.\"}]\n");
    assert_eq!(
        new[6]["payload"],
        json!({"attempt": attempt + 3, "round": 1, "verdict": "accept", "findings": [], "examined": 1, "tree": tree, "requirements": requirements, "provider": "anthropic", "limited": true})
    );
    assert_eq!(
        new[7]["payload"],
        json!({"candidate": commit, "tree": tree, "requirements": requirements, "checks": {}, "review": attempt + 5})
    );
    // The attempt's directories are gone; the person's branch and working tree did not move.
    assert!(dirs(&fx, "tmp", "attempt-").is_empty());
    assert_eq!(fx.git(&["rev-parse", "HEAD~1"]), head);
    assert_eq!(fx.git(&["status", "--porcelain"]), "");
    // One record commit holds the run's eight events (REQ-REG-10).
    let trailers = fx.git(&["log", "-1", "--format=%B"]).lines().filter(|l| l.starts_with("Fabrewk-Event: ")).count();
    assert_eq!(trailers, 8);
    // The backend's programs and the tool's preflight are kept as verified by use (3.9).
    let detected: Value =
        serde_json::from_slice(&fs::read(fx.domain.join("local/cache/detected.json")).unwrap()).unwrap();
    for (space, name) in [("tools", "claude"), ("programs", "bwrap"), ("programs", "pasta")] {
        assert!(detected[space][name]["verified"].is_string(), "{space} {name}: {detected}");
    }
    // `status` shows the task verified, and the review's limitation: its provider is the builder's (REQ-GATE-4,
    // `prefer`); the view shows both attempts' usage.
    let doc = fx.run(&["status", &id, "--json"]).doc();
    assert_eq!((doc["state"].as_str(), &doc["halt"]), (Some("verified"), &Value::Null));
    let human = fx.run(&["status", &id]);
    assert!(human.stdout.contains(&cli::say("es", "status.limited", &[])), "{human:?}");
    let status = fs::read_to_string(fx.domain.join("STATUS.md")).unwrap();
    assert!(status.contains("; tokens read 13, written 6, read from the cache 7."), "{status}");
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
    verifies(&fx, &id);
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
    assert_eq!(ran.status, 0, "{ran:?}");
    let doc = ran.doc();
    assert_eq!(
        (doc["schema"].as_str(), doc["run"]["schema"].as_str()),
        (Some("fabrewk.authorized/3"), Some("fabrewk.run/2"))
    );
    assert_eq!(doc["run"]["halt"], Value::Null);
    let events = fx.events(&stream(&id));
    assert_eq!(kinds(&events[1..3]), ["task-authorized", "run-started"]);
    assert_eq!(events[2]["payload"]["authorization"]["seq"], 2);
    assert_eq!(events[2]["say"], events[1]["say"]);
    // Verified, `authorize --run` again: the run relies on the authorization it writes, not the earlier one, and
    // revalidates the candidate the task keeps (REQ-RUN-9).
    let ran = authorize(&fx, &id, &["--run"]);
    assert_eq!(ran.status, 0, "{ran:?}");
    let events = fx.events(&stream(&id));
    let at = events.iter().rposition(|e| e["kind"] == "task-authorized").unwrap();
    assert_eq!(events[at + 1]["payload"]["authorization"]["seq"], at + 1);
    ok(&fx.run(&["check"]));
    // `run <stage>`: the stage's first task that may run.
    let (stage, ids) = plan(&fx, json!({}), vec![json!({}), json!({"goal": "Two."})]);
    ok(&authorize(&fx, &stage, &[]));
    let ran = fx.run(&["run", &stage, "--say", SAY, "--json"]);
    assert_eq!(ran.status, 0, "{ran:?}");
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
    verifies(&fx, &ids[0]);
    // Verified, the task is bound again by its stage's authorization; its run names that one.
    ok(&authorize(&fx, &stage, &[]));
    let new = verifies(&fx, &ids[0]);
    assert_eq!(new[0]["payload"]["authorization"]["stream"], format!("stage-{stage}"));
    // A task authorized alone, then its stage authorized with `--run`, which binds no task: the run relies on the
    // task's own.
    let (stage, ids) = plan(&fx, json!({}), vec![json!({})]);
    ok(&authorize(&fx, &ids[0], &[]));
    let ran = authorize(&fx, &stage, &["--run"]);
    assert_eq!(ran.status, 0, "{ran:?}");
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
    verifies(&fx, &id);
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
    // The stage's dispatches (the first task's builder and reviewer), then its tokens (12 read and 5 written by the
    // first task's builder, 1 and 1 by its reviewer), then tokens it cannot know.
    for (budget, why) in [
        (json!({"dispatches": 2, "minutes": 120}), "2 of 2 dispatches spent"),
        (json!({"dispatches": 6, "minutes": 120, "tokens": 19}), "19 of 19 tokens spent"),
    ] {
        let (stage, ids) = plan(&fx, json!({"budget": budget}), vec![json!({}), json!({"goal": "Two."})]);
        ok(&authorize(&fx, &stage, &[]));
        verifies(&fx, &ids[0]);
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
    // Within the run, the reviewer is not dispatched once the builder's tokens are unknown (REQ-RUN-2).
    let new = halts(&fx, &ids[0], "BUDGET_EXHAUSTED");
    assert_eq!(kinds(&new), ["run-started", "attempt-started", "attempt-ended", "candidate-recorded", "task-halted"]);
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
    verifies(&fx, &id);
    let now = dirs(&fx, "evidence", "preflight-").len();
    assert!(now > preflights, "{preflights} {now}");
    let id = authorized(&fx, json!({}));
    let new = verifies(&fx, &id);
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
    verifies(&fx, &ids[0]);
    // The first is accepted and integrated by hand (part 9g builds it): the record branch takes its candidate's file,
    // in a commit that names it (REQ-GATE-8).
    integrate(&fx, &ids[0]);
    // The second's base is the record branch's head, whose product is what the integration left.
    let new = verifies(&fx, &ids[1]);
    assert_eq!(new[0]["payload"]["base"], fx.git(&["rev-parse", "HEAD~1"]));
    // Another change of the product after the integration: refused, with nothing written.
    ok(&authorize(&fx, &ids[1], &[]));
    fs::write(fx.root.join("README.md"), "moved\n").unwrap();
    fx.commit("a person's change");
    refused(&fx, &["run", &ids[1], "--say", SAY], "BASE_MOVED", 1, "es");
    // The domain's own commits do not move it.
    fs::write(fx.root.join("README.md"), "fixture\n").unwrap();
    fx.commit("back");
    verifies(&fx, &ids[1]);
}

/// Accepts `id`'s verified candidate with the digest `status` shows, which integrates it (part 9g, REQ-GATE-8).
fn integrate(fx: &Fx, id: &str) {
    let digest = fx.run(&["status", id, "--json"]).doc()["accepting"]["digest"].as_str().unwrap().to_owned();
    let ran = fx.run(&["accept", id, "--seen", &digest, "--say", SAY]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(fx.events(&stream(id)).last().unwrap()["kind"], "task-integrated");
}

/// The last of a task's dependencies' integrations is read from the record branch's history, never from the clock
/// (P9E-9): the product must be the one the latest left, and an integration the history does not hold is none.
#[test]
fn refusal_base_moved_reads_the_last_integration_from_the_history() {
    let fx = Fx::builder("base-chain", "published", SUCCESS);
    fx.fake(&format!("mkdir -p src && printf 'a\\n' > src/a.txt\n{SUCCESS}"));
    // Each task's builder and reviewer, and the revalidations, spend the stage's dispatches.
    let (stage, ids) = plan(
        &fx,
        json!({"budget": {"dispatches": 12, "minutes": 120}}),
        vec![
            json!({"name": "a"}),
            json!({"name": "b", "goal": "B.", "base": null, "dependencies": ["a"]}),
            json!({"goal": "C.", "base": null, "dependencies": ["a", "b"]}),
        ],
    );
    ok(&authorize(&fx, &stage, &[]));
    verifies(&fx, &ids[0]);
    integrate(&fx, &ids[0]);
    fx.fake(&format!("mkdir -p src && printf 'b\\n' > src/b.txt\n{SUCCESS}"));
    verifies(&fx, &ids[1]);
    integrate(&fx, &ids[1]);
    // What the later integration left removed: the product is the earlier one's, which is not the last.
    fx.git(&["rm", "-q", "src/b.txt"]);
    fx.commit("b removed");
    refused(&fx, &["run", &ids[2], "--say", SAY], "BASE_MOVED", 1, "es");
    fx.git(&["revert", "--no-edit", "HEAD"]);
    verifies(&fx, &ids[2]);
    // The history, not the clock, says which came last: a commit naming the first's candidate after the second's
    // integration, its product the first's, makes the first the last, though its event is older.
    ok(&authorize(&fx, &ids[2], &[]));
    let first = fx.git(&["rev-parse", &format!("fabrewk/{}", ids[0])]);
    fx.git(&["rm", "-q", "src/b.txt"]);
    fx.git(&["commit", "-q", "-m", &format!("the first again\n\nFabrewk-Candidate: {first}")]);
    verifies(&fx, &ids[2]);
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
    let new = verifies(&fx, &id);
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

/// Git failing to say where the candidate's branch is, before the candidate is made (P9E-14): the task halts, its
/// evidence its own.
#[test]
fn a_branch_git_cannot_read_halts_naming_its_own_evidence() {
    let mut fx = Fx::builder("git-reads", "published", &format!("{WRITES}{SUCCESS}"));
    let id = authorized(&fx, json!({}));
    let wrapper = fx.root.parent().unwrap().join("git-wrapper");
    fs::create_dir_all(&wrapper).unwrap();
    let git =
        String::from_utf8(std::process::Command::new("sh").args(["-c", "command -v git"]).output().unwrap().stdout)
            .unwrap();
    let script = format!(
        "#!/bin/sh\ncase \"$*\" in *\"for-each-ref --format=%(refname) fabrewk/{id}\"*) exit 128;; esac\nexec {} \"$@\"\n",
        git.trim()
    );
    fs::write(wrapper.join("git"), script).unwrap();
    fs::set_permissions(wrapper.join("git"), fs::Permissions::from_mode(0o755)).unwrap();
    let path = std::env::join_paths(std::iter::once(wrapper).chain(std::env::split_paths(fx.path.as_ref().unwrap())));
    fx.path = Some(path.unwrap());
    let new = halts(&fx, &id, "TOOL_FAILED");
    assert_eq!(new[2]["payload"]["outcome"], "success");
    assert!(dirs(&fx, "evidence", &format!("attempt-{id}"))[0].join("candidate").is_file());
    assert!(dirs(&fx, "tmp", "attempt-").is_empty());
}

/// A changed path that is a name of white space alone is a path like any other: recorded, and the record replays.
#[test]
fn a_candidate_whose_path_is_white_space_is_recorded_and_replays() {
    let fx = Fx::builder("space", "published", &format!("printf 'x' > ' '\n{SUCCESS}"));
    let id = authorized(&fx, json!({"allowed": ["?"]}));
    let new = verifies(&fx, &id);
    assert_eq!(new[3]["payload"]["changes"][0]["path"], " ");
}

/// A stream that names an empty model: recorded as it said, and the record replays.
#[test]
fn an_empty_model_a_stream_names_is_recorded_and_replays() {
    let fx = Fx::builder(
        "empty-model",
        "published",
        &format!("{WRITES}{}", SUCCESS.replace(r#""model":"claude-sonnet-5-5""#, r#""model":"""#)),
    );
    let id = authorized(&fx, json!({}));
    let new = verifies(&fx, &id);
    assert_eq!(new[2]["payload"]["model"], "");
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
    // No cached tokens reported, then zero; each a task of its own.
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
        let id = authorized(&fx, json!({}));
        let new = verifies(&fx, &id);
        assert_eq!(new[2]["payload"]["usage"]["tokens"]["cached"], cached);
    }
    // The view sums each task's, its builder's and its reviewer's: one part not reported, so the cache's sum is
    // unknown; the others are figures.
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
    // The cell's pasta, as the backend wrote it beside the cell (D-092).
    let beside = dirs(&fx, "tmp", &format!("attempt-{}", ids[0])).pop().unwrap().join("beside");
    until("the helper's name", || fs::read_to_string(&beside).is_ok_and(|l| l.ends_with('\n')));
    let line = fs::read_to_string(&beside).unwrap();
    let (pasta, program) =
        (line.split(' ').next().unwrap(), format!("{}\0", line.trim_end().rsplit(' ').next().unwrap()));
    assert!(program.ends_with("/pasta\0"), "{line}");
    let alive = || fs::read(format!("/proc/{pasta}/cmdline")).is_ok_and(|c| c.starts_with(program.as_bytes()));
    assert!(alive(), "{pasta}");
    // Killed before the attempt's end (PD-08): the cell ends with it, its pasta stays idle until a write command.
    child.kill().unwrap();
    child.wait().unwrap();
    assert_eq!(fx.run(&["status", &ids[0], "--json"]).doc()["state"], "running");
    assert!(alive(), "{pasta}");
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
    until("the idle pasta's end", || !alive());
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
    let new = verifies(&fx, &ids[0]);
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

/// A process's start time, as `/proc/<pid>/stat` gives it.
fn start_time(pid: u32) -> String {
    let stat = fs::read_to_string(format!("/proc/{pid}/stat")).unwrap();
    stat.rsplit_once(')').unwrap().1.split_whitespace().nth(19).unwrap().to_owned()
}

/// D-092, P9I-2: a stopped run's cell that has not ended when the next write command recovers it keeps its directories
/// and what names its helper, which a later write command ends once the cell is gone.
#[test]
fn d_092_a_cell_still_ending_keeps_its_directories_until_its_helper_is_ended() {
    let fx = Fx::builder("still-ending", "published", &format!("{WAITS}{WRITES}{SUCCESS}"));
    let (stage, ids) = plan(&fx, json!({}), vec![json!({}), json!({"goal": "Two."})]);
    ok(&authorize(&fx, &stage, &[]));
    let (mut child, work) = started(&fx, &ids[0]);
    let beside = work.parent().unwrap().join("beside");
    until("the helper's name", || fs::read_to_string(&beside).is_ok_and(|l| l.ends_with('\n')));
    let real = fs::read_to_string(&beside).unwrap().split(' ').next().unwrap().to_owned();
    child.kill().unwrap();
    child.wait().unwrap();
    // The cell's first process still runs (a process of the test's stands for it) and its helper is a `pasta` of the
    // test's; the real one is ended by hand.
    assert!(std::process::Command::new("kill").arg(&real).status().unwrap().success());
    let pasta = fx.root.parent().unwrap().join("pasta");
    fs::copy("/usr/bin/sleep", &pasta).unwrap();
    let mut first = std::process::Command::new("sleep").arg("600").spawn().unwrap();
    let mut helper = std::process::Command::new(&pasta).arg("600").spawn().unwrap();
    let line = format!(
        "{} {} {} {} {}\n",
        helper.id(),
        start_time(helper.id()),
        first.id(),
        start_time(first.id()),
        pasta.display()
    );
    fs::write(&beside, line).unwrap();
    // The recovery halts the task, but keeps the cell's directories and the helper's name.
    ok(&fx.run(&["reject", &ids[1], "--say", SAY]));
    assert_eq!(fx.events(&stream(&ids[0])).last().unwrap()["payload"]["code"], "OUTCOME_UNKNOWN");
    assert!(beside.is_file() && work.is_dir());
    assert!(helper.try_wait().unwrap().is_none());
    // Once the cell is gone, the next write command ends its helper.
    first.kill().unwrap();
    first.wait().unwrap();
    // Even one that then refuses, with nothing of the record written.
    assert_eq!(fx.run(&["discard", "p-00000000", "--say", SAY]).status, 1);
    let t = std::time::Instant::now();
    let status = loop {
        if let Some(status) = helper.try_wait().unwrap() {
            break status;
        }
        if t.elapsed() > std::time::Duration::from_secs(10) {
            helper.kill().unwrap();
            helper.wait().unwrap();
            panic!("the helper was not ended");
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    };
    assert_eq!(std::os::unix::process::ExitStatusExt::signal(&status), Some(9));
    assert!(!beside.exists());
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

// ---- Checks, review and verification (part 9f; REQ-GATE-1 to REQ-GATE-6, REQ-RUN-8, REQ-RUN-9, REQ-TASK-5; D-085) ----

/// A fake builder that keeps the packet it received in its candidate (REQ-RUN-8's criterion) and adds a line to a file,
/// so that a later round shows the copy it started from (D-085).
const KEEPS: &str =
    "mkdir -p src && for last; do :; done && printf '%s' \"$last\" > src/packet.json && printf 'x\\n' >> src/log.txt\n";

/// A review rejecting with `n` confirmed blocking findings, each summary `word` and its number.
fn reject(n: usize, word: &str) -> String {
    let findings: Vec<Value> = (1..=n)
        .map(|i| json!({"severity": "blocking", "status": "confirmed", "path": "src/log.txt", "summary": format!("{word} {i}")}))
        .collect();
    review(&json!({"verdict": "reject", "findings": findings}).to_string())
}

/// The roles of the attempts among `events`, in order.
fn roles(events: &[Value]) -> Vec<&str> {
    events.iter().filter(|e| e["kind"] == "attempt-started").map(|e| e["payload"]["role"].as_str().unwrap()).collect()
}

/// The packet a builder kept in the candidate `commit`.
fn packet(fx: &Fx, commit: &str) -> Value {
    serde_json::from_str(&fx.git(&["show", &format!("{commit}:src/packet.json")])).unwrap()
}

#[test]
fn pd_07_pd_33_a_reject_starts_the_next_round_until_the_rounds_are_spent() {
    let fx = Fx::builder("rounds", "published", &format!("{KEEPS}{SUCCESS}"));
    fx.reviewer(Some(&reject(1, "Missing")));
    let id = authorized(&fx, json!({"limits": {"rounds": 2, "dispatches": 6, "minutes": 120}}));
    let new = halts(&fx, &id, "ROUNDS_EXHAUSTED");
    // Two rounds, each its builder and its reviewer, with no person's move between them (PD-33); no third review is
    // dispatched (PD-07).
    assert_eq!(roles(&new), ["build", "review", "build", "review"]);
    let candidates: Vec<&Value> = new.iter().filter(|e| e["kind"] == "candidate-recorded").collect();
    let reviews: Vec<&Value> = new.iter().filter(|e| e["kind"] == "review-recorded").collect();
    assert_eq!((reviews[0]["payload"]["round"].as_u64(), reviews[1]["payload"]["round"].as_u64()), (Some(1), Some(2)));
    // The second round's copy was the first candidate (D-085), whose parent is still the base; its packet carries
    // the first round's confirmed findings, and nothing else of the task (REQ-RUN-8).
    let second = candidates[1]["payload"]["commit"].as_str().unwrap();
    assert_eq!(fx.git(&["show", &format!("{second}:src/log.txt")]), "x\nx");
    assert_eq!(fx.git(&["rev-parse", &format!("{second}^")]), new[0]["payload"]["base"].as_str().unwrap());
    let first = packet(&fx, candidates[0]["payload"]["commit"].as_str().unwrap());
    let keys = |p: &Value| p.as_object().unwrap().keys().cloned().collect::<Vec<_>>();
    assert_eq!(keys(&first), ["allowed", "goal", "languages", "requirements", "role", "rules", "schema"]);
    let packet = packet(&fx, second);
    assert_eq!(keys(&packet), ["allowed", "goal", "languages", "previous", "requirements", "role", "rules", "schema"]);
    assert_eq!(packet["previous"]["findings"], reviews[0]["payload"]["findings"]);
    assert_eq!(packet["previous"]["failedChecks"], json!([]));
    // Only an authorization grants more rounds.
    refused(&fx, &["run", &id, "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
    ok(&authorize(&fx, &id, &[]));
    fx.reviewer(None);
    let new = verifies(&fx, &id);
    assert_eq!(roles(&new), ["build", "review"]);
}

#[test]
fn refusal_no_progress_and_counts_that_fall_do_not_halt() {
    let fx = Fx::builder("progress", "published", &format!("{KEEPS}{SUCCESS}"));
    // Findings 2, 2, 2 over three rounds: NO_PROGRESS at the third round's end, before the round limit.
    fx.reviewer(Some(&reject(2, "Same")));
    let limits = json!({"rounds": 3, "dispatches": 9, "minutes": 120});
    let id = authorized(&fx, json!({"limits": limits}));
    let new = halts(&fx, &id, "NO_PROGRESS");
    assert_eq!(roles(&new), ["build", "review", "build", "review", "build", "review"]);
    assert!(new.last().unwrap()["payload"]["reason"].as_str().unwrap().contains("2, 2, 2"), "{new:?}");
    // Findings 2, 3, 1: no halt for progress; the rounds are spent instead. The reviewer knows its round by the
    // findings its packet carries.
    let script = format!(
        "case \"$*\" in *'\"summary\": \"Three'*) {}exit 0;; *'\"previous\"'*) {}exit 0;; esac\n{}",
        reject(1, "One"),
        reject(3, "Three"),
        reject(2, "Two")
    );
    fx.reviewer(Some(&script));
    let id = authorized(&fx, json!({"limits": limits}));
    let new = halts(&fx, &id, "ROUNDS_EXHAUSTED");
    let counts: Vec<usize> = new
        .iter()
        .filter(|e| e["kind"] == "review-recorded")
        .map(|e| e["payload"]["findings"].as_array().unwrap().len())
        .collect();
    assert_eq!(counts, [2, 3, 1]);
}

/// The policy's check `test` and a stage's `staged`, which the task's profile does not name (REQ-GATE-1, PD-23).
const CHECKED: &str =
    "\n[checks.test]\ncommand = [\"sh\", \"-c\", \"test -f src/new.txt && echo test-ran\"]\nminutes = 1\n";

fn staged(command: &str) -> Value {
    json!({"checks": {"staged": {"command": ["sh", "-c", command], "network": false, "minutes": 1, "tools": []}}})
}

#[test]
fn req_gate_1_2_the_required_checks_run_confined_on_a_fresh_export() {
    let fx = Fx::builder_with("checks", "published", &format!("{WRITES}{SUCCESS}"), CHECKED);
    // The stage's check probes its cell: the candidate's export, no repository in it, no route out (REQ-GATE-2), and
    // its own writable copy.
    let probe = "test -f src/new.txt && test ! -e .git && test -z \"$(tail -n +2 /proc/net/route)\" && touch here";
    let (_, ids) = plan(&fx, staged(probe), vec![json!({"checks": ["test"]})]);
    ok(&authorize(&fx, &ids[0], &[]));
    let new = verifies(&fx, &ids[0]);
    let checks: Vec<&Value> = new.iter().filter(|e| e["kind"] == "check-recorded").map(|e| &e["payload"]).collect();
    assert_eq!(checks.iter().map(|c| c["check"].as_str().unwrap()).collect::<Vec<_>>(), ["staged", "test"]);
    assert!(checks.iter().all(|c| c["outcome"] == "passed" && c["round"] == 1), "{checks:?}");
    // Their evidence is kept and named by their events' places; their copies are gone, unread.
    for (c, e) in checks.iter().zip(new.iter().filter(|e| e["kind"] == "check-recorded")) {
        let dir = fx.domain.join(format!("local/evidence/check-{}-{}", ids[0], e["seq"]));
        assert_eq!(c["evidence"]["stdout"], sha256_hex(&fs::read(dir.join("stdout")).unwrap()));
    }
    assert!(dirs(&fx, "tmp", "check-").is_empty());
    // The reviewer comes after them, and the verification binds each identity (REQ-GATE-3, REQ-GATE-5).
    assert_eq!(
        kinds(&new)[4..],
        ["check-recorded", "check-recorded", "attempt-started", "attempt-ended", "review-recorded", "task-verified"]
    );
    let verified = &new.last().unwrap()["payload"]["checks"];
    assert_eq!((&verified["staged"], &verified["test"]), (&checks[0]["identity"], &checks[1]["identity"]));
    // A check with network cannot be confined by this backend (D-085): the run halts before any dispatch.
    let mut networked = staged("true");
    networked["checks"]["staged"]["network"] = json!(true);
    let (_, ids) = plan(&fx, networked, vec![json!({})]);
    ok(&authorize(&fx, &ids[0], &[]));
    let new = halts(&fx, &ids[0], "CONFINEMENT_UNAVAILABLE");
    assert_eq!(kinds(&new), ["task-halted"]);
    assert!(new[0]["payload"]["reason"].as_str().unwrap().contains("network"), "{new:?}");
    // A stage's check the policy defines alike, then otherwise once adopted again: refused, nothing run (P9F-2).
    let twice = json!({"checks": {"twice": {"command": ["true"], "network": false, "minutes": 1, "tools": []}}});
    let (_, ids) = plan(&fx, twice, vec![json!({})]);
    ok(&authorize(&fx, &ids[0], &[]));
    let text = fs::read_to_string(fx.domain.join("settings.toml")).unwrap();
    fx.edit_policy(&format!("{text}\n[checks.twice]\ncommand = [\"false\"]\nminutes = 1\n"));
    ok(&fx.run(&["adopt", "--say", "La ajusto.", "--widen", "checks.twice"]));
    let e = refused(&fx, &["run", &ids[0], "--say", SAY], "CHECK_DUPLICATE", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("twice"), "{e}");
    // A required check that neither the policy nor the stage defines, and a tool a check names that no cell shows.
    let id = authorized(&fx, json!({"checks": ["nowhere"]}));
    refused(&fx, &["run", &id, "--say", SAY], "SETTINGS_INVALID", 1, "es");
    let mut tooled = staged("true");
    tooled["checks"]["staged"]["tools"] = json!(["no-such-tool"]);
    let (_, ids) = plan(&fx, tooled, vec![json!({})]);
    ok(&authorize(&fx, &ids[0], &[]));
    let e = refused(&fx, &["run", &ids[0], "--say", SAY], "TOOL_MISSING", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("no-such-tool"), "{e}");
}

#[test]
fn req_gate_6_a_check_that_does_not_pass_ends_the_round_without_a_reviewer() {
    // The builder makes `src/second` once its packet carries a failed check.
    let builder = format!("{WRITES}{KEEPS}case \"$*\" in *failedChecks*) touch src/second;; esac\n{SUCCESS}");
    let fx = Fx::builder("failed-check", "published", &builder);
    let (_, ids) = plan(&fx, staged("echo MARKER-OUT; echo MARKER-ERR >&2; test -f src/second"), vec![json!({})]);
    ok(&authorize(&fx, &ids[0], &[]));
    let new = verifies(&fx, &ids[0]);
    // Round 1: its check failed, no reviewer; round 2: the builder again, the check passed, the reviewer accepted.
    assert_eq!(roles(&new), ["build", "build", "review"]);
    let checks: Vec<&Value> = new.iter().filter(|e| e["kind"] == "check-recorded").map(|e| &e["payload"]).collect();
    assert_eq!(
        checks.iter().map(|c| (c["round"].as_u64().unwrap(), c["outcome"].as_str().unwrap())).collect::<Vec<_>>(),
        [(1, "failed"), (2, "passed")]
    );
    // The second round's packet carries the failed check's output (REQ-RUN-8); a check's identity changed with its
    // candidate's tree (REQ-GATE-3).
    let second = new.iter().filter(|e| e["kind"] == "candidate-recorded").nth(1).unwrap();
    let packet = packet(&fx, second["payload"]["commit"].as_str().unwrap());
    let failed = &packet["previous"]["failedChecks"][0];
    assert_eq!((failed["check"].as_str(), failed["outcome"].as_str()), (Some("staged"), Some("failed")));
    assert!(
        failed["stdout"].as_str().unwrap().contains("MARKER-OUT")
            && failed["stderr"].as_str().unwrap().contains("MARKER-ERR")
    );
    assert_eq!(packet["previous"]["findings"], json!([]));
    assert_ne!(checks[0]["identity"], checks[1]["identity"]);
    // A check ended by a signal counts as not passed (REQ-GATE-2): its cell's status is 128 and the signal's (P-7). Three
    // rounds of one finding each are no progress.
    let (_, ids) = plan(&fx, staged("kill -9 $$"), vec![json!({})]);
    ok(&authorize(&fx, &ids[0], &[]));
    let head = fx.git(&["rev-parse", "HEAD"]);
    let new = halts(&fx, &ids[0], "NO_PROGRESS");
    // PD-46: no verification, and no commit on the record branch but the record's own.
    assert!(!kinds(&new).contains(&"task-verified"));
    let touched = fx.git(&["diff", "--name-only", &head, "HEAD"]);
    assert!(touched.lines().all(|f| f.starts_with(".brewstillery/")), "{touched}");
    let outcomes: Vec<&str> = new
        .iter()
        .filter(|e| e["kind"] == "check-recorded")
        .map(|e| e["payload"]["outcome"].as_str().unwrap())
        .collect();
    assert_eq!(outcomes, ["failed", "failed", "failed"]);
    assert_eq!(roles(&new), ["build", "build", "build"]);
}

/// A second tool, of another provider, absent from this machine: a reviewer's profile that uses it.
const ABSENT: &str = "\n[tools.codex]\nadapter = \"codex\"\nproviders = [\"openai\"]\n\n[providers.openai]\nallowed = true\n\n[models.luna]\nprovider = \"openai\"\nid = \"gpt-6-luna\"\nefforts = [\"low\"]\n\n[profiles.other]\nkind = \"review\"\ntool = \"codex\"\nmodel = \"luna\"\neffort = \"low\"\nminutes = 1\n";

#[test]
fn refusal_provider_missing_and_the_limitation_recorded() {
    let mut fx = Fx::builder_with("provider", "published", &format!("{WRITES}{SUCCESS}"), ABSENT);
    // A `PATH` that holds the fake Claude Code and the programs the run drives, and no Codex.
    let bin =
        path_with(&fx.root.parent().unwrap().join("no-codex"), &["git", "ssh-keygen", "df", "bwrap", "pasta", "socat"]);
    let dir = fx.root.parent().unwrap().join("tool/versions");
    fx.path = Some(std::env::join_paths([dir.as_os_str(), bin.as_os_str()]).unwrap());
    // `require` with the builder's own provider: halted before any dispatch.
    let require = json!({"settings": {"review.differentProvider": "require"}});
    let id = authorized(&fx, require.clone());
    let new = halts(&fx, &id, "PROVIDER_MISSING");
    assert_eq!(kinds(&new), ["task-halted"]);
    // Only an authorization naming the key lets it continue, the limitation recorded and shown (REQ-GATE-4).
    ok(&authorize(&fx, &id, &["--widen", "review.differentProvider"]));
    let auth = fx.events(&stream(&id)).into_iter().rfind(|e| e["kind"] == "task-authorized").unwrap();
    assert_eq!(auth["payload"]["widened"], json!(["review.differentProvider"]));
    let new = verifies(&fx, &id);
    assert_eq!(new.iter().find(|e| e["kind"] == "review-recorded").unwrap()["payload"]["limited"], true);
    // Authorized and run at once, the layer's own key is no naming of it (P9F-1).
    let id = plan(&fx, json!({}), vec![require.clone()]).1.remove(0);
    let ran = authorize(&fx, &id, &["--run"]);
    assert_eq!(ran.status, 4, "{ran:?}");
    let last = fx.events(&stream(&id)).pop().unwrap();
    assert_eq!(
        (last["kind"].as_str(), last["payload"]["code"].as_str()),
        (Some("task-halted"), Some("PROVIDER_MISSING"))
    );
    // A stage's layer, the same.
    let (stage, ids) = plan(&fx, json!({"settings": {"review.differentProvider": "require"}}), vec![json!({})]);
    let ran = authorize(&fx, &stage, &["--run"]);
    assert_eq!(ran.status, 4, "{ran:?}");
    assert_eq!(fx.events(&stream(&ids[0])).pop().unwrap()["payload"]["code"], "PROVIDER_MISSING");
    // `off`, a widening the authorization names, records no limitation.
    let id = plan(&fx, json!({}), vec![json!({"settings": {"review.differentProvider": "off"}})]).1.remove(0);
    ok(&authorize(&fx, &id, &["--widen", "review.differentProvider"]));
    let new = verifies(&fx, &id);
    assert_eq!(new.iter().find(|e| e["kind"] == "review-recorded").unwrap()["payload"]["limited"], false);
    // A reviewer's tool that is absent: PROVIDER_MISSING with `require`, TOOL_MISSING otherwise.
    let other =
        json!({"profiles": {"build": "build", "review": "other"}, "settings": {"review.differentProvider": "require"}});
    let id = authorized(&fx, other);
    let new = halts(&fx, &id, "PROVIDER_MISSING");
    assert!(new[0]["payload"]["reason"].as_str().unwrap().contains("absent"), "{new:?}");
    let id = authorized(&fx, json!({"profiles": {"build": "build", "review": "other"}}));
    refused(&fx, &["run", &id, "--say", SAY], "TOOL_MISSING", 1, "es");
}

#[test]
fn pd_25_pd_26_a_review_that_examined_nothing_does_not_verify() {
    let fx = Fx::builder("unexamined", "published", &format!("{WRITES}{SUCCESS}"));
    let recorded: Vec<Value> =
        fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../adapter-claude/tests/streams/review.jsonl"))
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
    // The recorded real review: its use of `Bash` at 1, answered at 3; its answer tool at 4 and 5; its result at 6.
    let with = |f: &dyn Fn(&mut Vec<Value>)| {
        let mut lines = recorded.clone();
        f(&mut lines);
        lines.iter().map(|l| l.to_string() + "\n").collect::<String>()
    };
    let at = |i: usize, p: &'static str, v: Value| move |l: &mut Vec<Value>| *l[i].pointer_mut(p).unwrap() = v.clone();
    let cases: Vec<(&str, String, &str)> = vec![
        // A well-formed accept after no use at all.
        ("PD-25", with(&|l| _ = l.drain(1..6)), "zero"),
        ("PD-26.1", with(&|l| _ = l[3]["message"]["content"][0].as_object_mut().unwrap().remove("is_error")), "void"),
        (
            "PD-26.2",
            with(&|l| *l[1].pointer_mut("/message/content/0/input/command").unwrap() = json!("cat 'a;b' && ls")),
            "verifies",
        ),
        ("PD-26.3", with(&|l| l.insert(4, l[3].clone())), "void"),
        ("PD-26.4", with(&|l| _ = l.pop()), "void"),
        ("PD-26.5", with(&|l| l[1]["message"]["content"][0] = json!({"type": "tool_use"})), "void"),
        ("PD-26.6", with(&at(6, "/subtype", json!("error_max_turns"))), "void"),
        ("PD-26.7", with(&|l| _ = l.remove(3)), "void"),
        ("PD-26.8", with(&at(1, "/message/content/0/name", json!(true))), "void"),
        (
            "PD-26.9",
            with(&|l| {
                let mut failed = l[6].clone();
                failed["is_error"] = json!(true);
                l.insert(6, failed)
            }),
            "void",
        ),
        ("PD-26.10", with(&|l| l.push(json!({"type": "rate_limit_event"}))), "void"),
        (
            "PD-26.11",
            with(&|l| {
                l[1]["message"]["content"][0]["id"] = json!(1);
                l[3]["message"]["content"][0]["tool_use_id"] = json!(1)
            }),
            "void",
        ),
        ("PD-26.12", with(&|l| l.push(l[3].clone())), "void"),
        ("PD-26.13", with(&|l| l.insert(2, l[1].clone())), "void"),
        ("PD-26.14", with(&at(1, "/message/content/0/input/command", json!(""))), "void"),
        ("PD-26.15", with(&at(3, "/message/content/0/is_error", json!("false"))), "void"),
        (
            "PD-26.16",
            with(&|l| {
                let c = l[3]["message"]["content"][0]["content"].as_str().unwrap().to_owned();
                l[3]["message"]["content"][0]["content"] = json!(c + "\u{2028}")
            }),
            "verifies",
        ),
        ("PD-26.17", with(&|l| l.insert(2, json!({"type": "mystery"}))), "void"),
        (
            "PD-26.18",
            with(&|l| {
                l.remove(3);
                l.remove(1);
            }),
            "zero",
        ),
        ("PD-26.19", with(&|l| l.insert(2, json!(null))).replace("null\n", "\n"), "void"),
        ("PD-26.20", with(&at(6, "/is_error", json!(0))), "void"),
    ];
    let dir = fx.root.parent().unwrap().join("tool/versions");
    for (case, stream, expect) in cases {
        fs::write(dir.join("anomaly.jsonl"), &stream).unwrap();
        fx.reviewer(Some("cat \"$(dirname \"$0\")/anomaly.jsonl\"\n"));
        let id = authorized(&fx, json!({}));
        if expect == "verifies" {
            verifies(&fx, &id);
            continue;
        }
        let new = halts(&fx, &id, "REVIEW_UNEXAMINED");
        assert!(!kinds(&new).contains(&"task-verified"), "{case}");
        // A stream the audit voids gives no review to record; a well-formed review that examined nothing is recorded,
        // with its count.
        let recorded = new.iter().find(|e| e["kind"] == "review-recorded");
        assert_eq!(recorded.map(|e| e["payload"]["examined"].clone()), (expect == "zero").then(|| json!(0)), "{case}");
    }
    // A reject with no confirmed blocking finding is not a review (REQ-RUN-8).
    fx.reviewer(Some(&review(r#"{"verdict":"reject","findings":[]}"#)));
    let id = authorized(&fx, json!({}));
    let new = halts(&fx, &id, "REVIEW_INVALID");
    assert!(!kinds(&new).contains(&"review-recorded"));
}

#[test]
fn req_run_9_a_verified_task_whose_evidence_went_stale_is_revalidated() {
    let fx = Fx::builder_with("revalidate", "published", &format!("{WRITES}{SUCCESS}"), CHECKED);
    let id = authorized(&fx, json!({"checks": ["test"], "limits": {"rounds": 2, "dispatches": 6, "minutes": 120}}));
    verifies(&fx, &id);
    // Its evidence current, the task is accepted or rejected, not run.
    let e = refused(&fx, &["run", &id, "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("current"), "{e}");
    // The policy adopted again: every identity is stale (REQ-GATE-3); `run` revalidates the same candidate, its checks
    // and its review, and dispatches no builder (REQ-RUN-9); the revalidation is the task's second round.
    let stale = |fx: &Fx, n: usize| {
        let text = fs::read_to_string(fx.domain.join("settings.toml")).unwrap();
        fx.edit_policy(&format!("{text}\n# changed {n}\n"));
        ok(&fx.run(&["adopt", "--say", "La ajusto."]));
    };
    stale(&fx, 1);
    let candidate = fx.git(&["rev-parse", &format!("fabrewk/{id}")]);
    let new = verifies(&fx, &id);
    assert_eq!(roles(&new), ["review"]);
    assert_eq!(kinds(&new)[..2], ["run-started", "check-recorded"]);
    assert_eq!(new[1]["payload"]["round"], 2);
    assert_eq!(new.last().unwrap()["payload"]["candidate"], candidate);
    // Stale again, its rounds spent: refused, with nothing written, since a verified task cannot halt; an
    // authorization grants a round, and the run revalidates the candidate it keeps.
    stale(&fx, 2);
    refused(&fx, &["run", &id, "--say", SAY], "ROUNDS_EXHAUSTED", 1, "es");
    ok(&authorize(&fx, &id, &[]));
    let new = verifies(&fx, &id);
    assert_eq!(roles(&new), ["review"]);
    assert_eq!(fx.git(&["rev-parse", &format!("fabrewk/{id}")]), candidate);
    // Stale again with no backend: the run starts, then halts, since no halt leaves `verified` (section 4); the
    // record replays.
    stale(&fx, 3);
    let bin = path_with(&fx.root.parent().unwrap().join("nobackend"), &["git", "ssh-keygen", "df", "socat"]);
    let dir = fx.root.parent().unwrap().join("tool/versions");
    let path = std::env::join_paths([dir.as_os_str(), bin.as_os_str()]).unwrap();
    let before = fx.events(&stream(&id)).len();
    let ran = fx.run_with(&["run", &id, "--say", SAY], Some(&path));
    assert_eq!(ran.status, 4, "{ran:?}");
    let new = fx.events(&stream(&id))[before..].to_vec();
    assert_eq!(kinds(&new), ["run-started", "task-halted"]);
    assert_eq!(new[1]["payload"]["code"], "CONFINEMENT_UNAVAILABLE");
    ok(&fx.run(&["check"]));
}

#[test]
fn a_run_killed_during_a_check_is_recovered_and_its_copy_removed() {
    let fx = Fx::builder("killed-check", "published", &format!("{WRITES}{SUCCESS}"));
    let (stage, ids) =
        plan(&fx, staged("until [ -e /work/go ]; do sleep 0.1; done"), vec![json!({}), json!({"goal": "Two."})]);
    ok(&authorize(&fx, &stage, &[]));
    // The directories of a check an earlier run left unread, which no recovery of a later run removes.
    let earlier = fx.domain.join(format!("local/tmp/check-{}-1", ids[0]));
    fs::create_dir_all(earlier.join("work")).unwrap();
    let mut child = fx.spawn(&["run", &ids[0], "--say", SAY]);
    until("the check", || dirs(&fx, "tmp", &format!("check-{}", ids[0])).iter().any(|d| d.join("work/src").is_dir()));
    child.kill().unwrap();
    child.wait().unwrap();
    ok(&fx.run(&["reject", &ids[1], "--say", SAY]));
    assert_eq!(fx.events(&stream(&ids[0])).last().unwrap()["payload"]["code"], "OUTCOME_UNKNOWN");
    assert_eq!(dirs(&fx, "tmp", "check-"), [earlier]);
}

// ---- Acceptance and integration (part 9g; REQ-GATE-7, REQ-GATE-8, REQ-GATE-10; D-086, D-087) ------------------------

/// The task's acceptance and its integration, its last two events.
fn accepted_and_integrated(fx: &Fx, id: &str) -> (Value, Value) {
    let events = fx.events(&stream(id));
    let n = events.len();
    assert_eq!(kinds(&events[n - 2..]), ["task-accepted", "task-integrated"], "{events:?}");
    (events[n - 2].clone(), events[n - 1].clone())
}

#[test]
fn req_gate_7_8_pd_37_accept_shows_what_it_binds_and_integrates_a_verified_task() {
    let fx = Fx::builder("accept", "published", &format!("{WRITES}{SUCCESS}"));
    let id = verified_task(&fx, json!({}), json!({}));
    let candidate = fx.git(&["rev-parse", &format!("fabrewk/{id}")]);
    let tree = fx.git(&["rev-parse", &format!("{candidate}^{{tree}}")]);
    // What it binds, before it is signed, in the person's language (PD-37): the task, the candidate, its files, its
    // checks and its review.
    let human = fx.run(&["status", &id]);
    ok(&human);
    whole(&human.all());
    let digest = accepting(&fx, &id);
    let before = fx.events(&stream(&id));
    let v = &before.iter().rfind(|e| e["kind"] == "task-verified").unwrap()["payload"];
    let r = &before[v["review"].as_u64().unwrap() as usize - 1]["payload"];
    let (seq, examined, found) =
        (v["review"].to_string(), r["examined"].to_string(), r["findings"].as_array().unwrap().len());
    let review =
        [("seq", seq.as_str()), ("verdict", "accept"), ("examined", &examined), ("findings", &found.to_string())];
    for words in [
        cli::say("es", "accept.review", &review),
        cli::say("es", "accept.title", &[]),
        cli::say("es", "accept.candidate", &[("commit", &candidate), ("tree", &tree)]),
        cli::say("es", "accept.added", &[("path", "src/new.txt")]),
        cli::say("es", "accept.digest", &[("digest", &digest)]),
        format!("fabrewk accept {id} --seen {digest}"),
    ] {
        assert!(human.stdout.contains(&words), "{words}: {human:?}");
    }
    // `--seen` is required; another digest is `TASK_CHANGED`, with nothing written.
    assert_eq!(fx.run(&["accept", &id, "--say", SAY]).status, 2);
    refused(&fx, &["accept", &id, "--seen", &"0".repeat(64), "--say", SAY], "TASK_CHANGED", 1, "es");
    let head = fx.git(&["rev-parse", "HEAD"]);
    let ran = fx.run(&["accept", &id, "--seen", &digest, "--say", SAY]);
    ok(&ran);
    whole(&ran.all());
    let integration = fx.git(&["rev-parse", "HEAD~1"]);
    assert!(ran.stdout.contains(&cli::say("es", "accept.integrated", &[("id", &id), ("commit", &integration)])));
    // The record commit starts from the integration's, with no compare-and-swap refused on the way.
    let recovered = cli::say("es", "write.recovered", &[("detail", "")]);
    assert!(!ran.stdout.contains(recovered.trim_end()), "{ran:?}");
    // The signed event binds what was shown (REQ-GATE-7).
    let (accepted, integrated) = accepted_and_integrated(&fx, &id);
    let events = fx.events(&stream(&id));
    let verified = &events.iter().rfind(|e| e["kind"] == "task-verified").unwrap()["payload"];
    let p = &accepted["payload"];
    assert_eq!((&p["candidate"], &p["tree"], &p["shown"]), (&json!(candidate), &json!(tree), &json!(digest)));
    assert_eq!(
        (&p["checks"], &p["review"], &p["requirements"]),
        (&verified["checks"], &verified["review"], &verified["requirements"])
    );
    assert_eq!(accepted["say"]["text"], SAY);
    // In `published`, the head is not the base: an integration commit on the head, its only parent, the candidate's
    // product and the head's domain, the candidate's message and trailers and `Fabrewk-Candidate`; then the record
    // commit of the command (REQ-GATE-8, REQ-REG-10).
    assert_eq!(integrated["payload"], json!({"from": head, "to": integration}));
    assert_eq!(fx.git(&["rev-parse", &format!("{integration}^@")]), head);
    let message = fx.git(&["show", "-s", "--format=%B", &integration]);
    assert!(message.contains("Co-authored-by: claude") && message.contains(&format!("Fabrewk-Candidate: {candidate}")));
    assert_eq!(
        fx.git(&["rev-parse", &format!("{integration}:src")]),
        fx.git(&["rev-parse", &format!("{candidate}:src")])
    );
    assert_eq!(
        fx.git(&["rev-parse", &format!("{integration}:.brewstillery")]),
        fx.git(&["rev-parse", &format!("{head}:.brewstillery")])
    );
    assert_eq!(fx.git(&["show", "-s", "--format=%s", "HEAD"]), "fabrewk: record");
    // The working tree and the index hold the candidate's file, from its object's bytes, and nothing else changed.
    assert_eq!(fs::read(fx.root.join("src/new.txt")).unwrap(), b"new\n");
    assert_eq!(fx.git(&["status", "--porcelain"]), "");
    ok(&fx.run(&["check"]));
    // Integrated is final: no acceptance, no rejection.
    refused(&fx, &["accept", &id, "--seen", &digest, "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
    refused(&fx, &["reject", &id, "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
}

/// Makes the domain its own repository, as `own-repository` needs, with its files committed there.
fn own(fx: &Fx) {
    let domain = fx.domain.to_str().unwrap().to_owned();
    let inner = |args: &[&str]| fx.git(&[&["-C", domain.as_str()][..], args].concat());
    inner(&["init", "-q", "-b", "main"]);
    for (k, v) in [("user.name", "Fixture"), ("user.email", "fixture@example.invalid"), ("commit.gpgsign", "false")] {
        inner(&["config", k, v]);
    }
    inner(&["add", "-A"]);
    inner(&["commit", "-q", "-m", "record"]);
}

#[test]
fn req_gate_8_with_the_head_at_the_base_the_branch_moves_to_the_candidate_itself() {
    for mode in ["own-repository", "unpublished"] {
        let fx = Fx::builder(&format!("ff-{mode}"), mode, &format!("{WRITES}{SUCCESS}"));
        if mode == "own-repository" {
            own(&fx);
        }
        let base = fx.git(&["rev-parse", "HEAD"]);
        let id = verified_task(&fx, json!({}), json!({}));
        let candidate = fx.git(&["rev-parse", &format!("fabrewk/{id}")]);
        ok(&fx.run(&["accept", &id, "--seen", &accepting(&fx, &id), "--say", SAY]));
        assert_eq!(fx.git(&["rev-parse", "HEAD"]), candidate, "{mode}");
        assert_eq!(accepted_and_integrated(&fx, &id).1["payload"], json!({"from": base, "to": candidate}), "{mode}");
        assert_eq!(fs::read(fx.root.join("src/new.txt")).unwrap(), b"new\n");
        assert_eq!(fx.git(&["status", "--porcelain"]), "", "{mode}");
        // In `own-repository` the domain's own `.git` is part of it (D-091).
        ok(&fx.run(&["check"]));
    }
}

/// What a candidate removes and a link it adds: status shows the removal, the integration writes the link as a link,
/// and the files it hashed to read the working tree are gone, none left in the repository.
#[test]
fn req_gate_8_a_removal_and_a_link_are_integrated() {
    let script = "rm src/old.txt && ln -s new.txt src/link && ";
    let fx = Fx::builder("accept-kinds", "published", &format!("{script}{WRITES}{SUCCESS}"));
    fs::create_dir_all(fx.root.join("src")).unwrap();
    fs::write(fx.root.join("src/old.txt"), "old\n").unwrap();
    fx.commit("an old file");
    let id = verified_task(&fx, json!({}), json!({}));
    let human = fx.run(&["status", &id]);
    assert!(human.stdout.contains(&cli::say("es", "accept.removed", &[("path", "src/old.txt")])), "{human:?}");
    assert!(human.stdout.contains(&cli::say("es", "accept.added", &[("path", "src/link")])), "{human:?}");
    integrate(&fx, &id);
    assert_eq!(fs::read_link(fx.root.join("src/link")).unwrap(), PathBuf::from("new.txt"));
    assert!(!fx.root.join("src/old.txt").exists());
    assert_eq!(fx.git(&["status", "--porcelain"]), "");
    let tmp = fx.domain.join("local/tmp");
    let left: Vec<_> = fs::read_dir(&tmp).map(|d| d.map(|e| e.unwrap().file_name()).collect()).unwrap_or_default();
    assert!(left.iter().all(|n| !n.to_string_lossy().starts_with("merge-")), "{left:?}");
}

#[test]
fn refusal_move_not_allowed_for_accept() {
    let fx = Fx::builder("accept-moves", "published", &format!("{WRITES}{SUCCESS}"));
    // A task with no candidate, and an authorized one whose branch is not at its candidate: nothing to accept.
    let proposed = plan(&fx, json!({}), vec![json!({})]).1.remove(0);
    let e = refused(&fx, &["accept", &proposed, "--seen", &"0".repeat(64), "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("task-accepted from proposed"), "{e}");
    let id = verified_task(&fx, json!({}), json!({}));
    ok(&authorize(&fx, &id, &[]));
    assert!(fx.run(&["status", &id, "--json"]).doc()["accepting"].is_null());
    let e = refused(&fx, &["accept", &id, "--seen", &"0".repeat(64), "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("not at its candidate"), "{e}");
}

/// D-087 point 1: a precondition already false before the signature refuses `accept`, nothing written: a product that
/// moved (record commits alone do not move it), a touched path with a person's edit in the working tree or in the
/// index, an alambique no maestro approved, and evidence a policy adopted since makes stale, though the task requires no check (D-086 point 4).
#[test]
fn refusal_base_moved_worktree_dirty_and_evidence_stale_at_accept() {
    let fx = Fx::builder("accept-pre", "published", &format!("{WRITES}{SUCCESS}"));
    let id = verified_task(&fx, json!({}), json!({}));
    let digest = accepting(&fx, &id);
    let accept = ["accept", &id, "--seen", &digest, "--say", SAY];
    fs::write(fx.root.join("README.md"), "moved\n").unwrap();
    fx.commit("a person's change");
    refused(&fx, &accept, "BASE_MOVED", 1, "es");
    fx.git(&["revert", "--no-edit", "HEAD"]);
    // A file the candidate adds, already there with other bytes; then the same bytes staged.
    fs::create_dir_all(fx.root.join("src")).unwrap();
    fs::write(fx.root.join("src/new.txt"), "mine\n").unwrap();
    let e = refused(&fx, &accept, "WORKTREE_DIRTY", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("src/new.txt"), "{e}");
    fx.git(&["add", "src/new.txt"]);
    fs::remove_file(fx.root.join("src/new.txt")).unwrap();
    refused(&fx, &accept, "WORKTREE_DIRTY", 1, "es");
    fx.git(&["rm", "-q", "--cached", "src/new.txt"]);
    // An alambique changed by hand, which no maestro approved.
    let alambique = fx.domain.join("ALAMBIQUE.md");
    let kept = fs::read(&alambique).ok();
    fs::write(&alambique, "# Changed by hand\n").unwrap();
    refused(&fx, &accept, "ALAMBIQUE_UNAPPROVED", 1, "es");
    match kept {
        Some(bytes) => fs::write(&alambique, bytes).unwrap(),
        None => fs::remove_file(&alambique).unwrap(),
    }
    // A policy adopted after the verification.
    let text = fs::read_to_string(fx.domain.join("settings.toml")).unwrap() + "\n[limits]\nminutes = 60\n";
    fx.edit_policy(&text);
    ok(&fx.run(&["adopt", "--say", SAY]));
    let digest = accepting(&fx, &id);
    refused(&fx, &["accept", &id, "--seen", &digest, "--say", SAY], "EVIDENCE_STALE", 1, "es");
    // `run` revalidates it, and then it integrates.
    verifies(&fx, &id);
    integrate(&fx, &id);
}

/// PD-44: a task in `authorized`, `halted` and `verified` whose record branch a person brought to its candidate (a
/// cherry-pick in `published`, so that the record's commits stay): `accept` integrates it with no move, and `reject`
/// stops it, saying where the branch is; with no acceptance on record `reject` is allowed (PD-14).
#[test]
fn pd_44_pd_14_a_branch_brought_to_the_candidate_by_hand_is_accepted_or_rejected() {
    for (state, accept) in [("verified", true), ("authorized", true), ("halted", true), ("verified", false)] {
        let fx = Fx::builder(&format!("pd-44-{state}-{accept}"), "published", &format!("{WRITES}{SUCCESS}"));
        // A stage's budget spent by the verification halts the next run before any dispatch.
        let id = verified_task(&fx, json!({"budget": {"dispatches": 2, "minutes": 120}}), json!({}));
        if state != "verified" {
            ok(&authorize(&fx, &id, &[]));
        }
        if state == "halted" {
            halts(&fx, &id, "BUDGET_EXHAUSTED");
        }
        let candidate = fx.git(&["rev-parse", &format!("fabrewk/{id}")]);
        fx.git(&["cherry-pick", &candidate]);
        let head = fx.git(&["rev-parse", "HEAD"]);
        if accept {
            ok(&fx.run(&["accept", &id, "--seen", &accepting(&fx, &id), "--say", SAY]));
            assert_eq!(accepted_and_integrated(&fx, &id).1["payload"], json!({"from": null, "to": head}), "{state}");
            assert_eq!(fx.git(&["rev-parse", "HEAD~1"]), head);
        } else {
            let ran = fx.run(&["reject", &id, "--say", SAY]);
            ok(&ran);
            assert!(ran.stdout.contains(&cli::say("es", "reject.at", &[("commit", &candidate)])), "{ran:?}");
            let e = fx.events(&stream(&id)).last().unwrap().clone();
            assert_eq!((&e["payload"]["candidate"], &e["payload"]["atCandidate"]), (&json!(candidate), &json!(true)));
        }
        ok(&fx.run(&["check"]));
    }
}

/// T-28 through `accept` (S-11): a candidate that changes the repository's own hooks directory, listed literally, is
/// integrated and its hook runs for none of Fabrewk's ref updates; a configured filter the base's `.gitattributes` names
/// leaves the working tree's bytes those of the objects.
#[test]
fn t_28_no_hook_or_filter_a_candidate_brings_runs_at_integration() {
    let hook = "mkdir -p hooks && printf '#!/bin/sh\\ntouch \"$PWD/../ran\"\\n' > hooks/reference-transaction && chmod +x hooks/reference-transaction\n";
    let fx = Fx::builder("t-28", "published", &format!("{WRITES}{hook}{SUCCESS}"));
    fs::write(fx.root.join(".gitattributes"), "*.txt filter=upper\n").unwrap();
    fx.commit("attributes");
    fx.git(&["config", "core.hooksPath", "hooks"]);
    fx.git(&["config", "filter.upper.smudge", "tr a-z A-Z"]);
    fx.git(&["config", "filter.upper.clean", "tr A-Z a-z"]);
    let allowed = json!({"allowed": ["src/**", "hooks/reference-transaction"]});
    let id = verified_task(&fx, json!({}), allowed);
    integrate(&fx, &id);
    assert!(fx.root.join("hooks/reference-transaction").is_file());
    assert_eq!(fs::read(fx.root.join("src/new.txt")).unwrap(), b"new\n");
    // The hook is the repository's now, and Fabrewk's later ref updates still run none of it.
    ok(&fx.run(&["reject", &plan(&fx, json!({}), vec![json!({})]).1[0], "--say", SAY]));
    assert!(!fx.root.parent().unwrap().join("ran").exists() && !fx.root.join("../ran").exists());
}
