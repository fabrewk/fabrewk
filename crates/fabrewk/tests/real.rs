//! One real dispatch per provider through the built binary (core-specification 13, phase 9; plan row 9; part 9i): a
//! `run` whose builder is this machine's Claude Code and whose reviewer is its Codex, each tool's preflight first,
//! then `accept`. Never part of `cargo test`: ignored, and run only when `FABREWK_REAL_RUN` is set, with a maestro's
//! permission and budget (D-095). The fixture is a repository of a few files under Cargo's `CARGO_TARGET_TMPDIR`, with
//! its own private folder and Git configuration; `HOME` is the person's, so that each tool's credential is bound
//! read-only, of which Fabrewk reads only its expiry. It prints where the receipts are.

mod common;

use common::*;
use serde_json::{Value, json};
use std::path::PathBuf;
use std::process::Command;

const SAY: &str = "Que corra con las herramientas reales.";

/// Claude Code builds and Codex reviews, each with one model at effort `low` and ten minutes (D-095).
const CATALOGUE: &str = "\n[tools.claude]\nadapter = \"claude-code\"\nproviders = [\"anthropic\"]\n\n[tools.codex]\nadapter = \"codex\"\nproviders = [\"openai\"]\n\n[providers.anthropic]\nallowed = true\n\n[providers.openai]\nallowed = true\n\n[models.sonnet]\nprovider = \"anthropic\"\nid = \"claude-sonnet-5-5\"\nefforts = [\"low\"]\n\n[models.luna]\nprovider = \"openai\"\nid = \"gpt-6-luna\"\nefforts = [\"low\"]\n\n[profiles.build]\nkind = \"build\"\ntool = \"claude\"\nmodel = \"sonnet\"\neffort = \"low\"\nminutes = 10\n\n[profiles.review]\nkind = \"review\"\ntool = \"codex\"\nmodel = \"luna\"\neffort = \"low\"\nminutes = 10\n";

/// Runs the binary in the fixture as [`Fx::run`] does, but with the person's `HOME`; the private folder and Git's
/// configuration stay the fixture's.
fn real(fx: &Fx, args: &[&str]) -> Ran {
    let mut c = Command::new(env!("CARGO_BIN_EXE_fabrewk"));
    isolate(&mut c, &fx.home, None);
    c.env("HOME", std::env::var_os("HOME").unwrap());
    let out = c.current_dir(&fx.root).args(args).output().unwrap();
    let ran = Ran {
        status: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    };
    eprintln!("fabrewk {args:?}: exit {}\n{}{}", ran.status, ran.stdout, ran.stderr);
    ran
}

/// What the receipts are: each attempt's role, outcome, model observed and usage, and where its evidence is.
fn receipts(fx: &Fx, id: &str) -> Vec<Value> {
    let events = fx.events(&format!("task-{id}"));
    events
        .iter()
        .filter(|e| e["kind"] == "attempt-ended")
        .map(|e| {
            let started = events.iter().find(|s| s["kind"] == "attempt-started" && s["seq"] == e["payload"]["attempt"]);
            json!({"role": started.map(|s| s["payload"]["role"].clone()), "ended": e["payload"]})
        })
        .collect()
}

#[test]
#[ignore = "runs the real Claude Code and Codex and calls their models; set FABREWK_REAL_RUN"]
fn real_one_dispatch_per_provider_through_run_and_accept() {
    assert!(std::env::var_os("FABREWK_REAL_RUN").is_some(), "set FABREWK_REAL_RUN");
    let fx = Fx::new_with("real-run", "published", CATALOGUE);
    std::fs::create_dir_all(fx.root.join("src")).unwrap();
    std::fs::write(fx.root.join("src/greeting.txt"), "hola\n").unwrap();
    fx.commit("a product of two files");
    let head = fx.git(&["rev-parse", "HEAD"]);
    let task = json!({
        "lang": "en", "goal": "Add the file src/hello.txt holding the single line hello, and change nothing else.",
        "requirements": [{"text": "src/hello.txt exists and holds the line hello.", "criterion": "the file src/hello.txt holds exactly hello and a newline"}],
        "base": head, "dependencies": [], "allowed": ["src/**"], "profiles": {"build": "build", "review": "review"},
        "checks": [], "limits": {"rounds": 1, "dispatches": 2, "minutes": 20}, "delegationLanguage": "en",
        "settings": {"review.differentProvider": "require"},
    });
    let stage = json!({"lang": "en", "goal": "A first real run.", "budget": {"dispatches": 2, "minutes": 20}, "checks": {}, "settings": {}});
    let file: PathBuf = fx.root.parent().unwrap().join("plan.json");
    std::fs::write(&file, serde_json::to_vec(&json!({"stage": stage, "tasks": [task]})).unwrap()).unwrap();
    let ran = real(&fx, &["propose", file.to_str().unwrap(), "--json"]);
    assert_eq!(ran.status, 0);
    let id = ran.doc()["tasks"][0]["id"].as_str().unwrap().to_owned();
    let digest = fx.run(&["status", &id, "--json"]).doc()["shown"]["digest"].as_str().unwrap().to_owned();
    assert_eq!(real(&fx, &["authorize", &id, "--seen", &digest, "--say", SAY]).status, 0);
    // The run: each tool's preflight (no model), then the builder's dispatch and the reviewer's.
    let ran = real(&fx, &["run", &id, "--say", SAY]);
    let evidence = fx.domain.join("local/evidence");
    eprintln!("receipts: {:#}", json!(receipts(&fx, &id)));
    eprintln!("evidence in {}", evidence.display());
    let tools = fx.run(&["config", "--json"]).doc();
    eprintln!("tools as detected: {:#}", tools["detected"]);
    assert_eq!(ran.status, 0, "the run did not verify the task");
    assert_eq!(real(&fx, &["accept", &id, "--seen", &accepting(&fx, &id), "--say", SAY]).status, 0);
    assert_eq!(std::fs::read_to_string(fx.root.join("src/hello.txt")).unwrap(), "hello\n");
    assert_eq!(real(&fx, &["check"]).status, 0);
}
