//! The write commands of part 7d3a through the built binary (D-047, D-053, D-062): `adopt` and `propose`, the person's
//! words (REQ-TASK-4; PD-30 to PD-32), the agent's environment (REQ-SEC-7), signing with Git's signing key (D-055),
//! the machine key (REQ-REG-12), the record commit (REQ-REG-10), and every refusal of the write path, the record's
//! codes among them, each asserting that nothing was written (11.2 rule 4).

mod common;

use common::*;
use fabrewk_core::cli;
use fabrewk_core::record::{Draft, KeySigner, Say, Session, sha256_hex};
use serde_json::{Value, json};
use std::ffi::OsString;
use std::fs;

const SAY: &str = "Lo adopto: las rondas alcanzan para esta etapa.";

fn published(name: &str) -> Fx {
    Fx::new(name, "published")
}

/// The project file the fixtures adopt, with `extra` appended.
fn policy_with(fx: &Fx, extra: &str) -> String {
    policy_text(&fx.keys) + extra
}

/// A stage with one task each, as `propose` reads it; `task` is merged into the task.
fn file(task: Value) -> Value {
    let mut t = json!({
        "lang": "en", "goal": "Make it.", "requirements": [{"text": "It builds.", "criterion": "cargo build passes"}],
        "base": "0".repeat(40), "dependencies": [], "allowed": ["src/**"],
        "profiles": {"build": "build", "review": "review"}, "checks": [],
        "limits": {"rounds": 3, "dispatches": 6, "minutes": 120}, "delegationLanguage": "en", "settings": {},
    });
    for (k, v) in task.as_object().unwrap() {
        t[k] = v.clone();
    }
    json!({
        "stage": {"lang": "en", "goal": "Build it.", "budget": {"dispatches": 6, "minutes": 120}, "checks": {}, "settings": {}},
        "tasks": [t],
    })
}

/// Writes `value` as the proposal file outside the domain and gives its name, relative to the repository.
fn proposal(fx: &Fx, name: &str, value: &Value) -> String {
    fs::write(fx.root.join(name), serde_json::to_vec(value).unwrap()).unwrap();
    name.to_owned()
}

fn ok(ran: &Ran) {
    assert_eq!(ran.status, 0, "{ran:?}");
    whole(&ran.all());
}

#[test]
fn req_set_5_adopt_records_the_policy_its_widenings_and_its_seal() {
    let fx = published("adopt");
    fx.stage_and_task();
    fx.commit("stage");
    fx.edit_policy(&policy_with(&fx, "\n[limits]\nrounds = 4\nminutes = 60\n"));
    // A widening not named is refused, and every change is shown classed.
    let e = refused(&fx, &["adopt", "--say", SAY], "WIDENING_UNNAMED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("limits.rounds"), "{e}");
    let ran = fx.run(&["adopt", "--say", SAY, "--widen", "limits.rounds"]);
    ok(&ran);
    assert!(ran.stdout.contains(&cli::say("es", "effect.widening", &[])), "{ran:?}");
    assert!(ran.stdout.contains(&cli::say("es", "effect.narrowing", &[])), "{ran:?}");
    let events = fx.events("project");
    let adopted = events.last().unwrap();
    let text = fs::read_to_string(fx.domain.join("settings.toml")).unwrap();
    assert_eq!(adopted["kind"], "policy-adopted");
    assert_eq!(adopted["payload"]["content"], text.as_str());
    assert_eq!(adopted["payload"]["digest"], sha256_hex(text.as_bytes()));
    assert_eq!(adopted["payload"]["widened"], json!(["limits.rounds"]));
    // It cites the policy it replaces, and seals every other stream's head.
    assert_eq!(adopted["policy"], events[0]["payload"]["digest"]);
    let seal = adopted["payload"]["heads"].as_object().unwrap();
    assert_eq!(seal.len(), 2, "{seal:?}");
    for (stream, head) in seal {
        let last = fx.events(stream).len();
        assert_eq!(head["seq"], last, "{stream}");
    }
    assert_eq!(adopted["say"], json!({"text": SAY, "lang": "es-MX"}));
    let doc = fx.run(&["config", "limits.rounds", "--json"]).doc();
    assert_eq!(doc["value"], 4, "{doc}");
    // The record replays with its signature, the allowed signers follow the policy, and one record commit holds it.
    assert_eq!(fx.run(&["check"]).status, 0, "{:?}", fx.run(&["check"]));
    let signers = fs::read_to_string(fx.domain.join("local/cache/allowed_signers")).unwrap();
    assert!(signers.starts_with("xel namespaces=\"fabrewk,git\" ssh-ed25519 "), "{signers}");
    let message = fx.git(&["log", "-1", "--format=%B"]);
    let digest = sha256_hex(&fs::read(newest(&fx, "project")).unwrap());
    assert_eq!(message, format!("fabrewk: record\n\nFabrewk-Event: {digest}"));
    let files = fx.git(&["show", "--name-only", "--format=", "HEAD"]);
    assert!(files.lines().all(|f| f.starts_with(".brewstillery/data/events/project/")), "{files}");
    assert_eq!(files.lines().count(), 2, "the event and its signature: {files}");
    // `--json` gives the adoption's document.
    fx.edit_policy(&policy_with(&fx, "\n[limits]\nrounds = 4\nminutes = 30\n"));
    let doc = fx.run(&["adopt", "--say", SAY, "--json"]).doc();
    assert_eq!(doc["schema"], "fabrewk.adopted/1");
    assert_eq!(doc["changes"][0]["effect"], "narrowing");
    assert_eq!(doc["events"][0]["stream"], "project");
    assert!(doc["commit"].is_string());
}

/// The newest event file of `stream`.
fn newest(fx: &Fx, stream: &str) -> std::path::PathBuf {
    let dir = fx.domain.join("data/events").join(stream);
    let mut files: Vec<_> = fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect();
    files.retain(|f| f.extension().is_some_and(|e| e == "json"));
    files.sort();
    files.pop().unwrap()
}

#[test]
fn refusal_policy_uncommitted() {
    let fx = published("uncommitted-policy");
    fs::write(fx.domain.join("settings.toml"), policy_with(&fx, "\n[limits]\nminutes = 60\n")).unwrap();
    refused(&fx, &["adopt", "--say", SAY], "POLICY_UNCOMMITTED", 1, "es");
    // In `unpublished` nothing is committed, so the file is adopted as it is.
    let fx = Fx::new("unpublished-policy", "unpublished");
    fs::write(fx.domain.join("settings.toml"), policy_with(&fx, "\n[limits]\nminutes = 60\n")).unwrap();
    let ran = fx.run(&["adopt", "--say", SAY]);
    ok(&ran);
    assert!(ran.stdout.contains(&cli::say("es", "write.uncommitted", &[])), "{ran:?}");
}

#[test]
fn refusal_role_not_enforced_and_settings_invalid_at_adopt() {
    let fx = published("supervisor");
    let ana = format!("\n[maintainers.ana]\nrole = \"supervisor\"\nkey = \"{}\"\n", public(&fx.keys, "stranger"));
    fx.edit_policy(&policy_with(&fx, &ana));
    refused(&fx, &["adopt", "--say", SAY, "--widen", "maintainers.ana"], "ROLE_NOT_ENFORCED", 1, "es");
    fx.edit_policy(&policy_with(&fx, "\n[nosuch]\nkey = 1\n"));
    refused(&fx, &["adopt", "--say", SAY], "SETTINGS_INVALID", 1, "es");
}

#[test]
fn refusal_widening_unnamed_for_a_maintainer_added() {
    let fx = published("widening");
    let ana = format!("\n[maintainers.ana]\nrole = \"contributor\"\nkey = \"{}\"\n", public(&fx.keys, "stranger"));
    fx.edit_policy(&policy_with(&fx, &ana));
    refused(&fx, &["adopt", "--say", SAY], "WIDENING_UNNAMED", 1, "es");
    ok(&fx.run(&["adopt", "--say", SAY, "--widen", "maintainers.ana"]));
    assert_eq!(fx.events("project").last().unwrap()["payload"]["widened"], json!(["maintainers.ana"]));
}

#[test]
fn refusal_role_forbidden() {
    // A key that is no maintainer's; then no signing key at all.
    let fx = published("forbidden");
    fx.git(&["config", "user.signingkey", fx.keys.join("stranger.pub").to_str().unwrap()]);
    // Nobody is known, so the messages are in the policy's canonical language.
    refused(&fx, &["adopt", "--say", SAY], "ROLE_FORBIDDEN", 1, "en");
    fx.git(&["config", "--unset", "user.signingkey"]);
    refused(&fx, &["adopt", "--say", SAY], "ROLE_FORBIDDEN", 1, "en");
    // A contributor makes no person's move (REQ-ID-3).
    let fx = Fx::bare("contributor", "published");
    let ana = format!("\n[maintainers.ana]\nrole = \"contributor\"\nkey = \"{}\"\n", public(&fx.keys, "stranger"));
    fx.edit_policy(&policy_with(&fx, &ana));
    fx.adopt("published");
    fx.commit("record");
    fx.git(&["config", "user.signingkey", fx.keys.join("stranger").to_str().unwrap()]);
    let e = refused(&fx, &["adopt", "--say", SAY], "ROLE_FORBIDDEN", 1, "en");
    assert!(e["detail"].as_str().unwrap().contains("contributor"), "{e}");
}

#[test]
fn refusal_agent_environment() {
    let fx = published("agent");
    // Each adapter's markers, as the binary declares them (D-062), and the core's own.
    for marker in ["CLAUDECODE", "CODEX_SANDBOX", "CODEX_SANDBOX_NETWORK_DISABLED", cli::EXTENSION_MARKER] {
        let before = fx.snapshot();
        let ran = fx.run_env(&["adopt", "--say", SAY], &[(marker, "1")]);
        assert_eq!(fx.snapshot(), before, "{marker}");
        assert_eq!(ran.status, 1, "{marker}: {ran:?}");
        let reason = cli::say("es", "code.AGENT_ENVIRONMENT.reason", &[]);
        assert!(ran.stderr.contains(&format!("AGENT_ENVIRONMENT: {reason}")), "{ran:?}");
        assert!(reason.contains("cooperativo"), "the message says the control is cooperative");
        assert!(ran.stderr.contains(marker), "{ran:?}");
        let json = fx.run_env(&["adopt", "--say", SAY, "--json"], &[(marker, "1")]);
        assert_eq!(json.doc()["refusals"][0]["code"], "AGENT_ENVIRONMENT");
    }
    // Proposing grants nothing, so an agent may.
    let name = proposal(&fx, "plan.json", &file(json!({})));
    ok(&fx.run_env(&["propose", &name], &[("CLAUDECODE", "1")]));
}

#[test]
fn pd_30_words_are_recorded_verbatim_with_their_tag() {
    let fx = published("words");
    for (i, (words, lang)) in
        [("Sí, lo adopto\u{2028}con\u{85}esto.\r", None), ("I adopt it.", Some("en-GB"))].into_iter().enumerate()
    {
        fx.edit_policy(&policy_with(&fx, &format!("\n[limits]\nminutes = {}\n", 100 - i)));
        let mut args = vec!["adopt", "--say", words];
        args.extend(lang.iter().flat_map(|l| ["--say-lang", l]));
        ok(&fx.run(&args));
        let say = fx.events("project").last().unwrap()["say"].clone();
        assert_eq!(say["text"], words);
        assert_eq!(say["lang"], lang.unwrap_or("es-MX"));
    }
    // `propose` takes no words and records no person.
    let name = proposal(&fx, "plan.json", &file(json!({})));
    let before = fx.snapshot();
    assert_eq!(fx.run(&["propose", &name, "--say", "x"]).status, 2);
    assert_eq!(fx.snapshot(), before);
    let doc = fx.run(&["propose", &name, "--json"]).doc();
    let stage = format!("stage-{}", doc["stage"]["id"].as_str().unwrap());
    for e in fx.events(&stage) {
        assert_eq!((e["actor"].clone(), e.get("say")), (json!({"kind": "fabrewk"}), None));
    }
}

#[test]
fn pd_31_words_missing() {
    let fx = published("missing");
    fx.edit_policy(&policy_with(&fx, "\n[limits]\nminutes = 60\n"));
    refused(&fx, &["adopt"], "WORDS_MISSING", 1, "es");
    for words in ["", " ", "\t\n", "\u{2028}"] {
        refused(&fx, &["adopt", "--say", words], "WORDS_MISSING", 1, "es");
    }
    // `--say` with no value, and words that are not UTF-8.
    refused(&fx, &["adopt", "--say"], "WORDS_MISSING", 1, "es");
    use std::os::unix::ffi::OsStringExt;
    let before = fx.snapshot();
    let args = ["adopt".into(), "--say".into(), OsString::from_vec(vec![b'S', 0xff, b'.'])];
    let ran = fx.run_os(&args, &[]);
    assert_eq!((ran.status, fx.snapshot() == before), (1, true), "{ran:?}");
    assert!(ran.stderr.contains("WORDS_MISSING"), "{ran:?}");
}

#[test]
fn pd_32_printed_commands_offer_no_words_and_examples_are_refused() {
    let fx = published("copied");
    fx.edit_policy(&policy_with(&fx, "\n[limits]\nminutes = 60\n"));
    for (lang, _) in cli::CATALOGUES {
        let example = cli::get(lang, "words.example").unwrap();
        assert!(cli::get(lang, "flag.say").unwrap().contains(example), "{lang}: help prints its example");
        refused(&fx, &["adopt", "--say", example], "WORDS_COPIED", 1, "es");
        refused(&fx, &["adopt", "--say", &format!("  {example}\n")], "WORDS_COPIED", 1, "es");
    }
    // Every command a message prints that needs words is printed without them: run as printed, it refuses.
    let mut printed = 0;
    for (_, text) in cli::CATALOGUES {
        for (k, run) in cli::messages(text).into_iter().filter(|(k, _)| k.ends_with(".run")) {
            let words: Vec<&str> = run.split(' ').skip(1).collect();
            assert!(!words.contains(&"--say"), "{k}: {run}");
            if words.first() == Some(&"adopt") {
                refused(&fx, &words, "WORDS_MISSING", 1, "es");
                printed += 1;
            }
        }
    }
    assert!(printed > 0);
}

#[test]
fn req_reg_12_the_first_write_makes_the_machine_key() {
    let fx = published("instance");
    assert!(!fx.folder().exists());
    for args in [&["status"][..], &["config"], &["check"]] {
        fx.run(args);
        assert!(!fx.folder().exists(), "{args:?} wrote the private folder");
    }
    let name = proposal(&fx, "plan.json", &file(json!({})));
    ok(&fx.run(&["propose", &name]));
    use std::os::unix::fs::PermissionsExt;
    assert_eq!(fs::metadata(fx.folder()).unwrap().permissions().mode() & 0o777, 0o700);
    let line = fx.program("ssh-keygen", &["-l", "-f", fx.folder().join("instance").to_str().unwrap()]);
    let fingerprint = line.split(' ').nth(1).unwrap();
    assert!(line.contains("ED25519"), "{line}");
    let doc = fx.run(&["propose", &name, "--json"]).doc();
    let stream = format!("task-{}", doc["tasks"][0]["id"].as_str().unwrap());
    assert_eq!(fx.events(&stream)[0]["instance"], fingerprint);
    // Detection is kept by write commands, dated, with the programs they used.
    let detected: Value =
        serde_json::from_slice(&fs::read(fx.domain.join("local/cache/detected.json")).unwrap()).unwrap();
    assert!(detected["programs"]["git"]["verified"].is_string(), "{detected}");
}

#[test]
fn req_reg_10_one_record_commit_leaves_the_persons_edits_alone() {
    let fx = published("commit");
    fs::write(fx.root.join("README.md"), "staged\n").unwrap();
    fx.git(&["add", "README.md"]);
    fs::write(fx.root.join("README.md"), "unstaged\n").unwrap();
    fs::write(fx.root.join("new.txt"), "untracked\n").unwrap();
    let staged = fx.git(&["diff", "--cached"]);
    let name = proposal(&fx, ".plan.json", &file(json!({})));
    let head = fx.git(&["rev-parse", "HEAD"]);
    ok(&fx.run(&["propose", &name]));
    assert_eq!(fx.git(&["diff", "--cached"]), staged);
    assert_eq!(fs::read_to_string(fx.root.join("README.md")).unwrap(), "unstaged\n");
    assert_eq!(fx.git(&["rev-parse", "HEAD~1"]), head, "one commit");
    let files = fx.git(&["show", "--name-only", "--format=", "HEAD"]);
    assert_eq!(files.lines().count(), 2, "{files}");
    assert!(files.lines().all(|f| f.starts_with(".brewstillery/data/events/")), "{files}");
    let trailers = fx.git(&["log", "-1", "--format=%B"]).matches("Fabrewk-Event: ").count();
    assert_eq!(trailers, 2);
}

#[test]
fn refusal_record_uncommitted_is_committed_first() {
    let fx = published("leftovers");
    fx.stage_and_task();
    let head = fx.git(&["rev-parse", "HEAD"]);
    fx.edit_policy(&policy_with(&fx, "\n[limits]\nminutes = 60\n"));
    let policy = fx.git(&["rev-parse", "HEAD"]);
    assert_ne!(head, policy);
    ok(&fx.run(&["adopt", "--say", SAY]));
    // The leftovers in a record commit of their own, then the adoption's.
    let first = fx.git(&["log", "-1", "--format=%B", "HEAD~1"]);
    assert_eq!(first.matches("Fabrewk-Event: ").count(), 2, "{first}");
    assert_eq!(fx.git(&["rev-parse", "HEAD~2"]), policy);
    assert_eq!(fx.run(&["check"]).status, 0, "{:?}", fx.run(&["check"]));
}

#[test]
fn propose_a_stage_with_dependent_tasks_and_add_to_it() {
    let fx = published("propose");
    let mut plan = file(json!({"name": "first"}));
    let mut second = plan["tasks"][0].clone();
    second["name"] = json!("second");
    second["goal"] = json!("Make it again.");
    second["base"] = Value::Null;
    second["dependencies"] = json!(["first"]);
    plan["tasks"].as_array_mut().unwrap().push(second);
    let name = proposal(&fx, "plan.json", &plan);
    let doc = fx.run(&["propose", &name, "--json"]).doc();
    assert_eq!(doc["schema"], "fabrewk.proposed/1");
    assert_eq!(doc["stage"]["new"], true);
    let (stage, first, second) = (
        doc["stage"]["id"].as_str().unwrap(),
        doc["tasks"][0]["id"].as_str().unwrap(),
        doc["tasks"][1]["id"].as_str().unwrap(),
    );
    let payload = &fx.events(&format!("task-{second}"))[0]["payload"];
    assert_eq!((payload["stage"].as_str(), payload["dependencies"].clone()), (Some(stage), json!([first])));
    assert!(payload["nonce"].as_str().is_some_and(|n| n.len() == 32));
    assert_eq!(fx.run(&["check"]).status, 0, "{:?}", fx.run(&["check"]));
    let status = fx.run(&["status", "--json"]).doc();
    assert_eq!(status["stages"][0]["tasks"].as_array().unwrap().len(), 2);
    // Tasks added to the stage, one depending on a task of the record.
    let mut more = file(json!({"base": null, "dependencies": [second]}));
    more["stage"] = json!(stage);
    let ran = fx.run(&["propose", &proposal(&fx, "more.json", &more)]);
    ok(&ran);
    assert!(ran.stdout.contains(&format!("fabrewk status {stage}")), "{ran:?}");
    let doc = fx.run(&["status", stage, "--json"]).doc();
    assert_eq!(doc["state"], "proposed");
    assert_eq!(fx.run(&["status", "--json"]).doc()["stages"][0]["tasks"].as_array().unwrap().len(), 3);
}

#[test]
fn refusal_task_invalid() {
    let fx = published("task-invalid");
    let cases = [
        file(json!({"requirements": []})),
        file(json!({"requirements": [{"text": "It builds."}]})),
        file(json!({"allowed": ["src/../.git/config"]})),
        file(json!({"allowed": ["a".repeat(257)]})),
        file(json!({"base": null})),
        file(json!({"dependencies": ["t-00000000"], "base": null})),
        file(json!({"name": "t-0123abcd"})),
        file(json!({"nonce": "0".repeat(32)})),
        json!({"stage": "s-00000000", "tasks": [file(json!({}))["tasks"][0].clone()]}),
        json!({"stage": file(json!({}))["stage"].clone(), "tasks": []}),
        json!({"tasks": []}),
    ];
    for (i, case) in cases.iter().enumerate() {
        let name = proposal(&fx, &format!("bad-{i}.json"), case);
        refused(&fx, &["propose", &name], "TASK_INVALID", 1, "es");
    }
    // A file that is not JSON, one that does not exist, and one in a domain.
    fs::write(fx.root.join("not.json"), "{").unwrap();
    refused(&fx, &["propose", "not.json"], "TASK_INVALID", 1, "es");
    refused(&fx, &["propose", "nosuch.json"], "TASK_INVALID", 1, "es");
    fs::write(fx.domain.join("xel/plan.json"), serde_json::to_vec(&file(json!({}))).unwrap()).unwrap();
    refused(&fx, &["propose", ".brewstillery/xel/plan.json"], "TASK_INVALID", 1, "es");
}

#[test]
fn pd_15_refusal_protected_path_at_propose() {
    let fx = published("protected");
    let refused_patterns = [
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
    ];
    for (i, p) in refused_patterns.iter().enumerate() {
        let name = proposal(&fx, &format!("p-{i}.json"), &file(json!({"allowed": [p]})));
        refused(&fx, &["propose", &name], "PROTECTED_PATH", 1, "es");
    }
    let kept = [
        "*.md",
        "src/**",
        ".github/**",
        ".gitignore",
        ".git*ignore",
        "tests/fixtures/brewstillery/**",
        ".brewstillery-old/x",
        "brewstillery-old/x",
    ];
    // `.github/` is protected by default (REQ-GATE-9), so a policy that protects only `Cargo.toml` lets the lane
    // guard alone judge `.github/**`.
    fx.edit_policy(&policy_with(&fx, "\n[gate]\nprotectedPaths = [\"Cargo.toml\"]\n"));
    ok(&fx.run(&["adopt", "--say", SAY, "--widen", "gate.protectedPaths"]));
    let name = proposal(&fx, "kept.json", &file(json!({"allowed": kept})));
    ok(&fx.run(&["propose", &name]));
}

#[test]
fn refusal_protected_path_wildcard() {
    let fx = published("wildcard");
    let name = proposal(&fx, "w.json", &file(json!({"allowed": ["*.toml"]})));
    refused(&fx, &["propose", &name], "PROTECTED_PATH_WILDCARD", 1, "es");
    let name = proposal(&fx, "l.json", &file(json!({"allowed": ["Cargo.toml", "src/**"]})));
    ok(&fx.run(&["propose", &name]));
}

#[test]
fn pd_24_refusal_check_duplicate() {
    let fx = published("duplicate");
    fx.edit_policy(&policy_with(&fx, "\n[checks.test]\ncommand = [\"false\"]\n"));
    ok(&fx.run(&["adopt", "--say", SAY, "--widen", "checks.test"]));
    let mut plan = file(json!({}));
    plan["stage"]["checks"] = json!({"test": {"command": ["true"], "network": false, "minutes": 30, "tools": []}});
    refused(&fx, &["propose", &proposal(&fx, "d.json", &plan)], "CHECK_DUPLICATE", 1, "es");
    plan["stage"]["checks"]["test"]["command"] = json!(["false"]);
    ok(&fx.run(&["propose", &proposal(&fx, "same.json", &plan)]));
}

#[test]
fn refusal_stage_closed() {
    let fx = published("closed");
    let (stage, _) = fx.stage_and_task();
    // The stage authorized and closed by the maestro, as phase 8's `close` will record it.
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
    let digest = sha256_hex(&{
        let mut b = fabrewk_core::record::encode(&fx.events(&stream)[0]["payload"]).unwrap();
        b.pop();
        b
    });
    let payload = json!({"stage": stage, "digest": digest, "head": {"seq": 1, "digest": head.digest}, "tasks": [], "widened": []});
    let h = s.append(&stream, Some(&head), &draft("stage-authorized", payload), Some(&signer)).unwrap();
    s.append(&stream, Some(&h), &draft("stage-closed", json!({})), Some(&signer)).unwrap();
    s.finish(&fabrewk_core::identity::PolicyVerifier, &[]).unwrap();
    fx.commit("closed");
    let mut plan = file(json!({}));
    plan["stage"] = json!(stage);
    refused(&fx, &["propose", &proposal(&fx, "c.json", &plan)], "STAGE_CLOSED", 1, "es");
}

#[test]
fn signing_with_a_key_literal_in_the_agent() {
    let fx = published("agent-key");
    let sock = fx.home.join("agent.sock");
    let agent = fx.program("ssh-agent", &["-a", sock.to_str().unwrap(), "-s"]);
    let pid = agent.split("SSH_AGENT_PID=").nth(1).unwrap().split(';').next().unwrap().to_owned();
    let env = [("SSH_AUTH_SOCK", sock.as_os_str())];
    let mut add = std::process::Command::new("ssh-add");
    add.env("SSH_AUTH_SOCK", &sock).arg("-q").arg(fx.keys.join("xel"));
    assert!(add.output().unwrap().status.success());
    let literal = format!("key::{}", public(&fx.keys, "xel"));
    fx.git(&["config", "user.signingkey", &literal]);
    fx.edit_policy(&policy_with(&fx, "\n[limits]\nminutes = 60\n"));
    let args: Vec<OsString> = ["adopt", "--say", SAY].iter().map(Into::into).collect();
    let ran = fx.run_os(&args, &env);
    let _ = std::process::Command::new("kill").arg(&pid).status();
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(fx.run(&["check"]).status, 0);
    assert!(!fx.domain.join("local/tmp/signing-key.pub").exists(), "the literal is removed after signing");
    // Without the agent, the key cannot sign: the tool fails, in words, with its text kept out.
    fx.edit_policy(&policy_with(&fx, "\n[limits]\nminutes = 50\n"));
    let e = refused(&fx, &["adopt", "--say", SAY], "TOOL_FAILED", 1, "es");
    assert!(!e["detail"].as_str().unwrap().contains("agent"), "{e}");
}

#[test]
fn refusal_tool_failed_when_the_key_cannot_sign() {
    // On a machine with no machine key yet (P7D3A-1): the refusal makes none.
    let fx = published("cannot-sign");
    fs::rename(fx.keys.join("xel"), fx.keys.join("moved")).unwrap();
    fx.edit_policy(&policy_with(&fx, "\n[limits]\nminutes = 60\n"));
    let e = refused(&fx, &["adopt", "--say", SAY], "TOOL_FAILED", 1, "es");
    assert_eq!(e["detail"], "ssh-keygen -Y sign failed");
    assert!(!fx.folder().exists());
    // With a machine key and events an earlier command left uncommitted: neither committed by the refusal.
    fs::rename(fx.keys.join("moved"), fx.keys.join("xel")).unwrap();
    ok(&fx.run(&["propose", &proposal(&fx, "plan.json", &file(json!({})))]));
    fx.stage_and_task();
    fs::rename(fx.keys.join("xel"), fx.keys.join("moved")).unwrap();
    refused(&fx, &["adopt", "--say", SAY], "TOOL_FAILED", 1, "es");
}

#[test]
fn refusal_tool_failed_for_a_key_literal_without_its_agent_writes_nothing() {
    // The first write on the machine, with no `local/tmp/` yet: the literal's copy and its directory are removed.
    let fx = published("literal-first");
    fx.git(&["config", "user.signingkey", &format!("key::{}", public(&fx.keys, "xel"))]);
    fx.edit_policy(&policy_with(&fx, "\n[limits]\nminutes = 60\n"));
    assert!(!fx.domain.join("local/tmp").exists());
    refused(&fx, &["adopt", "--say", SAY], "TOOL_FAILED", 1, "es");
    assert!(!fx.folder().exists());
}

#[test]
fn a_person_s_move_is_signed_once_before_anything_is_written() {
    // An `ssh-keygen` that signs once and fails after (P7D3A-1): the event's own signature is the only one, made before
    // the machine key is in place, so the move succeeds; a second signature would be refused with the key made.
    let fx = published("sign-once");
    let real = fx.program("sh", &["-c", "command -v ssh-keygen"]).trim().to_owned();
    let bin = fx.home.join("bin");
    fs::create_dir_all(&bin).unwrap();
    let count = fx.home.join("signs");
    let script = format!(
        "#!/bin/sh\nif [ \"$1\" = -Y ] && [ \"$2\" = sign ]; then\n  n=$(cat '{c}' 2>/dev/null || echo 0); n=$((n + 1)); echo $n > '{c}'\n  [ $n -ge 2 ] && exit 1\nfi\nexec '{real}' \"$@\"\n",
        c = count.display()
    );
    fs::write(bin.join("ssh-keygen"), script).unwrap();
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(bin.join("ssh-keygen"), fs::Permissions::from_mode(0o755)).unwrap();
    let path = format!("{}:{}", bin.display(), std::env::var("PATH").unwrap());
    fx.edit_policy(&policy_with(&fx, "\n[limits]\nminutes = 60\n"));
    let ran = fx.run_with(&["adopt", "--say", SAY], Some(std::ffi::OsStr::new(&path)));
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(fs::read_to_string(&count).unwrap().trim(), "1");
    assert!(fx.folder().join("instance").exists());
}

#[test]
fn a_git_lock_is_refused_before_anything_is_written() {
    let fx = published("git-lock");
    fs::write(fx.root.join(".git/index.lock"), "").unwrap();
    let name = proposal(&fx, "plan.json", &file(json!({})));
    refused(&fx, &["propose", &name], "TOOL_FAILED", 1, "es");
    assert!(!fx.folder().exists());
}

#[test]
fn a_private_folder_reached_through_a_link_into_a_domain_is_refused() {
    // `XDG_CONFIG_HOME` is a link to a maintainer's directory, where `fabrewk/` does not exist yet (P7D3A-8).
    let fx = published("folder-link");
    let link = fx.home.join("config-link");
    std::os::unix::fs::symlink(fx.domain.join("xel"), &link).unwrap();
    let name = proposal(&fx, "plan.json", &file(json!({})));
    let before = fx.snapshot();
    let args: Vec<OsString> = ["propose", &name, "--json"].iter().map(Into::into).collect();
    let ran = fx.run_os(&args, &[("XDG_CONFIG_HOME", link.as_os_str())]);
    assert_eq!(fx.snapshot(), before);
    assert_eq!((ran.status, ran.doc()["refusals"][0]["code"].as_str()), (1, Some("PRIVATE_FOLDER_PERMISSIONS")));
}

#[test]
fn a_git_that_cannot_name_the_author_is_refused_before_anything_is_written() {
    let fx = published("no-ident");
    fx.git(&["config", "user.name", ""]);
    fx.edit_policy_as(&policy_with(&fx, "\n[limits]\nminutes = 60\n"));
    refused(&fx, &["adopt", "--say", SAY], "TOOL_FAILED", 1, "es");
    assert!(!fx.folder().exists());
}

#[test]
fn refusal_check_unknown_for_a_shallow_history() {
    let fx = published("shallow");
    let head = fx.git(&["rev-parse", "HEAD"]);
    fs::write(fx.root.join(".git/shallow"), format!("{head}\n")).unwrap();
    refused(&fx, &["adopt", "--say", SAY], "CHECK_UNKNOWN", 1, "es");
}

#[test]
fn a_record_commit_git_refuses_leaves_the_events_written_and_says_so() {
    // A hook that refuses every ref update (P7D3A-18, D-065): the events stay written and not committed, as after a
    // race, the refusal says so, and the next write command commits them first.
    let fx = published("hook");
    fx.edit_policy(&policy_with(&fx, "\n[limits]\nminutes = 60\n"));
    let hook = fx.root.join(".git/hooks/reference-transaction");
    fs::write(&hook, "#!/bin/sh\nexit 1\n").unwrap();
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
    let head = fx.git(&["rev-parse", "HEAD"]);
    let doc = fx.run(&["adopt", "--say", SAY, "--json"]).doc();
    let e = &doc["refusals"][0];
    assert_eq!(e["code"], "TOOL_FAILED", "{doc}");
    assert!(e["detail"].as_str().unwrap().contains("written and not committed"), "{doc}");
    assert_eq!(fx.git(&["rev-parse", "HEAD"]), head);
    let check = fx.run(&["check", "--json"]).doc();
    assert!(check["findings"].as_array().unwrap().iter().any(|f| f["code"] == "RECORD_UNCOMMITTED"), "{check}");
    fs::remove_file(&hook).unwrap();
    ok(&fx.run(&["propose", &proposal(&fx, "plan.json", &file(json!({})))]));
    let first = fx.git(&["log", "-1", "--format=%B", "HEAD~1"]);
    assert_eq!(first.matches("Fabrewk-Event: ").count(), 1, "the adoption in a commit of its own: {first}");
    assert_eq!(fx.run(&["check"]).status, 0, "{:?}", fx.run(&["check"]));
}

#[test]
fn a_dangling_link_as_a_git_lock_is_a_lock() {
    let fx = published("dangling-lock");
    std::os::unix::fs::symlink("missing-lock-target", fx.root.join(".git/index.lock")).unwrap();
    let name = proposal(&fx, "plan.json", &file(json!({})));
    refused(&fx, &["propose", &name], "TOOL_FAILED", 1, "es");
}

#[test]
fn a_private_folder_whose_place_cannot_be_told_is_refused() {
    // `<link>/unmade/../confignew`, the link leading into a maintainer's directory (P7D3A-8).
    let fx = published("folder-dotdot");
    let link = fx.home.join("config-link");
    std::os::unix::fs::symlink(fx.domain.join("xel"), &link).unwrap();
    let config = link.join("unmade/../confignew");
    let name = proposal(&fx, "plan.json", &file(json!({})));
    let before = fx.snapshot();
    let args: Vec<OsString> = ["propose", &name, "--json"].iter().map(Into::into).collect();
    let ran = fx.run_os(&args, &[("XDG_CONFIG_HOME", config.as_os_str())]);
    assert_eq!(fx.snapshot(), before);
    assert_eq!((ran.status, ran.doc()["refusals"][0]["code"].as_str()), (1, Some("PRIVATE_FOLDER_PERMISSIONS")));
}

#[test]
fn a_persons_staged_files_where_replay_does_not_read_stay_staged() {
    // A file in a hidden directory under `data/events/`, committed, then staged with other content (P7D3A-12).
    let fx = published("hidden-index");
    let note = fx.domain.join("data/events/.personal/note.txt");
    fs::create_dir_all(note.parent().unwrap()).unwrap();
    fs::write(&note, "original\n").unwrap();
    fx.git(&["add", "-f", note.to_str().unwrap()]);
    fx.git(&["commit", "-q", "-m", "note"]);
    fs::write(&note, "staged personal text\n").unwrap();
    fx.git(&["add", "-f", note.to_str().unwrap()]);
    fs::write(&note, "original\n").unwrap();
    ok(&fx.run(&["propose", &proposal(&fx, "plan.json", &file(json!({})))]));
    assert_eq!(fx.git(&["show", ":.brewstillery/data/events/.personal/note.txt"]), "staged personal text");
}

#[test]
fn what_is_no_stream_is_never_committed() {
    // A hidden directory under `data/events/` holds no event: replay skips it, and no record commit takes it (P7D3A-9).
    let fx = published("hidden");
    let hidden = fx.domain.join("data/events/.personal");
    fs::create_dir_all(&hidden).unwrap();
    let file_in = hidden.join("000001-0000000000000000.json");
    fs::write(&file_in, "person material, not an event\n").unwrap();
    ok(&fx.run(&["propose", &proposal(&fx, "plan.json", &file(json!({})))]));
    let committed = fx.git(&["log", "--name-only", "--format="]);
    assert!(!committed.contains(".personal"), "{committed}");
    assert_eq!(fs::read_to_string(&file_in).unwrap(), "person material, not an event\n");
}

#[test]
fn refusal_settings_invalid_in_the_machine_file_writes_nothing() {
    let fx = published("machine-file");
    use std::os::unix::fs::PermissionsExt;
    fs::create_dir_all(fx.folder()).unwrap();
    fs::set_permissions(fx.folder(), fs::Permissions::from_mode(0o700)).unwrap();
    fs::write(fx.folder().join("machine.toml"), "unknown = 1\n").unwrap();
    let name = proposal(&fx, "plan.json", &file(json!({})));
    refused(&fx, &["propose", &name], "SETTINGS_INVALID", 1, "es");
}

#[test]
fn a_signing_key_is_never_read_from_a_domain_or_a_pipe() {
    // Outside the repository, `signing` is a link to the maestro's private key put in a maintainer's directory, and
    // `signing.pub` a regular copy of the public key (P7D3A-4): `ssh-keygen` would open the domain's file.
    let fx = published("domain-key");
    fs::copy(fx.keys.join("xel"), fx.domain.join("xel/uncited-key")).unwrap();
    let outside = fx.keys.join("signing");
    std::os::unix::fs::symlink(fx.domain.join("xel/uncited-key"), &outside).unwrap();
    fs::copy(fx.keys.join("xel.pub"), fx.keys.join("signing.pub")).unwrap();
    fx.git(&["config", "user.signingkey", outside.to_str().unwrap()]);
    fx.edit_policy(&policy_with(&fx, "\n[limits]\nminutes = 60\n"));
    refused(&fx, &["adopt", "--say", SAY], "ROLE_FORBIDDEN", 1, "en");
    // The public key file named, with a link into the domain as its private key beside it.
    let public = fx.keys.join("signing.pub");
    fx.git(&["config", "user.signingkey", public.to_str().unwrap()]);
    refused(&fx, &["adopt", "--say", SAY], "ROLE_FORBIDDEN", 1, "en");
    // A pipe as the key, with its public key beside it: refused at once, never opened.
    fs::remove_file(&outside).unwrap();
    fx.program("mkfifo", &[outside.to_str().unwrap()]);
    fx.git(&["config", "user.signingkey", outside.to_str().unwrap()]);
    refused(&fx, &["adopt", "--say", SAY], "ROLE_FORBIDDEN", 1, "en");
}

/// The record's codes through a write command (D-047, 11.2 rule 4): each damage refused by its code, nothing written.
#[test]
fn the_records_refusals_through_a_write_command() {
    let propose = |fx: &Fx| proposal(fx, "plan.json", &file(json!({})));
    // No record: no events directory, then an empty one (PD-05).
    let fx = Fx::bare("no-record", "published");
    let name = propose(&fx);
    refused(&fx, &["propose", &name], "RECORD_ABSENT", 1, "en");
    refused(&fx, &["adopt", "--say", SAY], "RECORD_ABSENT", 1, "en");
    fs::create_dir_all(fx.domain.join("data/events")).unwrap();
    refused(&fx, &["propose", &name], "RECORD_ABSENT", 1, "en");
    // A record with no policy adopted.
    let mut s = Session::begin(&fx.domain, None, &fabrewk_core::identity::PolicyVerifier).unwrap();
    let installed = Draft {
        kind: "installed".into(),
        actor: json!({"kind": "fabrewk"}),
        policy: "0".repeat(64),
        instance: "SHA256:fixture".into(),
        say: None,
        payload: json!({}),
    };
    s.append("project", None, &installed, None).unwrap();
    s.finish(&fabrewk_core::identity::PolicyVerifier, &[]).unwrap();
    refused(&fx, &["propose", &name], "POLICY_ABSENT", 1, "en");
    refused(&fx, &["adopt", "--say", SAY], "POLICY_ABSENT", 1, "en");

    // A record that does not replay names nobody, so its refusals are in the canonical language; one that replays
    // knows the person, whose language is Spanish.
    let damaged = |label: &str, damage: &dyn Fn(&Fx), code: &str| {
        let fx = published(label);
        let name = propose(&fx);
        damage(&fx);
        let replays = ["MARK_INVALID", "RECORD_ROLLBACK", "HEAD_DETACHED"].contains(&code);
        let lang = if replays { "es" } else { "en" };
        refused(&fx, &["propose", &name], code, 1, lang);
        refused(&fx, &["adopt", "--say", SAY], code, 1, lang);
    };
    let second = |fx: &Fx| newest(fx, "project");
    damaged(
        "digest",
        &|fx| {
            let p = second(fx);
            let mut b = fs::read(&p).unwrap();
            b[10] ^= 1;
            fs::write(&p, b).unwrap();
        },
        "RECORD_DIGEST_MISMATCH",
    );
    damaged("version", &|fx| rewrite(&second(fx), |v| v["schema"] = json!(2)), "RECORD_VERSION_UNKNOWN");
    damaged(
        "noncanonical",
        &|fx| {
            let p = second(fx);
            let mut b = fs::read(&p).unwrap();
            b.insert(1, b' ');
            rename_to_digest(&p, &b);
        },
        "RECORD_NONCANONICAL",
    );
    damaged(
        "fork",
        &|fx| {
            let p = second(fx);
            let mut v: Value = serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
            v["at"] = json!("2026-01-01T00:00:00Z");
            let b = fabrewk_core::record::encode(&v).unwrap();
            let other = p.with_file_name(format!("000002-{}.json", &sha256_hex(&b)[..16]));
            fs::write(&other, &b).unwrap();
            fs::copy(format!("{}.sig", p.display()), format!("{}.sig", other.display())).unwrap();
        },
        "RECORD_FORK",
    );
    damaged(
        "broken",
        &|fx| {
            let dir = fx.domain.join("data/events/project");
            for f in fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path()) {
                if f.file_name().unwrap().to_str().unwrap().starts_with("000001-") {
                    fs::remove_file(f).unwrap();
                }
            }
        },
        "RECORD_BROKEN",
    );
    damaged(
        "signature",
        &|fx| fs::remove_file(format!("{}.sig", second(fx).display())).unwrap(),
        "RECORD_SIGNATURE_INVALID",
    );
    damaged("mark", &|fx| fs::write(fx.domain.join("local/cache/heads.json"), "").unwrap(), "MARK_INVALID");
    damaged(
        "rollback",
        &|fx| {
            let p = second(fx);
            fs::remove_file(format!("{}.sig", p.display())).unwrap();
            fs::remove_file(p).unwrap();
        },
        "RECORD_ROLLBACK",
    );
    damaged(
        "transition",
        &|fx| {
            let (_, task) = fx.stage_and_task();
            fx.commit("stage");
            append_raw(fx, &format!("task-{task}"), "task-integrated");
        },
        "RECORD_TRANSITION_INVALID",
    );
    damaged(
        "attempt",
        &|fx| {
            let (_, task) = fx.stage_and_task();
            let stream = format!("task-{task}");
            run_by_hand(fx, &stream);
            append_raw(fx, &stream, "attempt-ended");
        },
        "ATTEMPT_ENDED",
    );
    damaged("detached", &|fx| _ = fx.git(&["checkout", "-q", "--detach"]), "HEAD_DETACHED");
}

/// Authorizes the task of `stream` and starts its run, signed by the maestro, as phase 9's commands will record them.
fn run_by_hand(fx: &Fx, stream: &str) {
    let signer = KeySigner(fx.keys.join("xel"));
    let mut s = Session::begin(&fx.domain, None, &fabrewk_core::identity::PolicyVerifier).unwrap();
    let head = s.head(stream).unwrap().unwrap();
    let first = &fx.events(stream)[0]["payload"];
    let mut bytes = fabrewk_core::record::encode(first).unwrap();
    bytes.pop();
    let person = |kind: &str, payload: Value| Draft {
        kind: kind.into(),
        actor: json!({"kind": "person", "alias": "xel", "role": "maestro"}),
        policy: fx.events("project")[0]["payload"]["digest"].as_str().unwrap().to_owned(),
        instance: "SHA256:fixture".into(),
        say: Some(Say { text: "Lo decido.".into(), lang: "es".into() }),
        payload,
    };
    let bound = json!({
        "task": &stream["task-".len()..], "digest": sha256_hex(&bytes), "head": {"seq": head.seq, "digest": head.digest},
        "limits": first["limits"], "widened": [],
    });
    let h = s.append(stream, Some(&head), &person("task-authorized", bound), Some(&signer)).unwrap();
    s.append(stream, Some(&h), &person("run-started", json!({})), Some(&signer)).unwrap();
    s.finish(&fabrewk_core::identity::PolicyVerifier, &[]).unwrap();
    fx.commit("run");
}

/// Appends, without the replay a write command makes, a Fabrewk event of `kind` to `stream`.
fn append_raw(fx: &Fx, stream: &str, kind: &str) {
    let mut s = Session::begin(&fx.domain, None, &fabrewk_core::identity::PolicyVerifier).unwrap();
    let head = s.head(stream).unwrap();
    let draft = Draft {
        kind: kind.into(),
        actor: json!({"kind": "fabrewk"}),
        policy: fx.events("project")[0]["payload"]["digest"].as_str().unwrap().to_owned(),
        instance: "SHA256:fixture".into(),
        say: None,
        payload: json!({}),
    };
    s.append(stream, head.as_ref(), &draft, None).unwrap();
    drop(s);
    fx.commit("hand-made");
}

/// Rewrites an event file with `change`, canonical, under the name its new digest gives.
fn rewrite(path: &std::path::Path, change: impl Fn(&mut Value)) {
    let mut v: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    change(&mut v);
    rename_to_digest(path, &fabrewk_core::record::encode(&v).unwrap());
}

fn rename_to_digest(path: &std::path::Path, bytes: &[u8]) {
    let name = path.file_name().unwrap().to_str().unwrap();
    let new = path.with_file_name(format!("{}-{}.json", &name[..6], &sha256_hex(bytes)[..16]));
    fs::remove_file(path).unwrap();
    fs::write(&new, bytes).unwrap();
    let sig = format!("{}.sig", path.display());
    if std::path::Path::new(&sig).exists() {
        fs::rename(&sig, format!("{}.sig", new.display())).unwrap();
    }
}

#[test]
fn refusal_lock_held() {
    let fx = published("lock");
    let name = proposal(&fx, "plan.json", &file(json!({})));
    let lock = fs::File::options().write(true).open(fx.domain.join("local/lock")).unwrap();
    lock.try_lock().unwrap();
    refused(&fx, &["propose", &name], "LOCK_HELD", 1, "es");
    drop(lock);
    ok(&fx.run(&["propose", &name]));
}

#[test]
fn refusal_not_a_repository_from_a_linked_working_tree() {
    let fx = published("worktree");
    let linked = fx.root.with_file_name("linked");
    fx.git(&["worktree", "add", "-q", linked.to_str().unwrap()]);
    let name = proposal(&fx, "plan.json", &file(json!({})));
    fs::copy(fx.root.join(&name), linked.join(&name)).unwrap();
    let before = fx.snapshot();
    let ran = run_in(&linked, &fx.home, &["propose", &name, "--json"], None);
    assert_eq!(fx.snapshot(), before);
    assert_eq!((ran.status, ran.doc()["refusals"][0]["code"].as_str()), (1, Some("NOT_A_REPOSITORY")), "{ran:?}");
}

#[test]
fn refusal_head_detached_in_every_mode() {
    for mode in ["unpublished", "published"] {
        let fx = Fx::new(&format!("detached-{mode}"), mode);
        let name = proposal(&fx, "plan.json", &file(json!({})));
        fx.git(&["checkout", "-q", "--detach"]);
        refused(&fx, &["propose", &name], "HEAD_DETACHED", 1, "es");
        refused(&fx, &["adopt", "--say", SAY], "HEAD_DETACHED", 1, "es");
    }
}

#[test]
fn refusal_domain_not_ignored_at_a_write_command() {
    let fx = Fx::new("write-not-ignored", "unpublished");
    let name = proposal(&fx, "plan.json", &file(json!({})));
    ok(&fx.run(&["propose", &name]));
    fs::write(fx.root.join(".git/info/exclude"), "").unwrap();
    refused(&fx, &["propose", &name], "DOMAIN_NOT_IGNORED", 1, "es");
    refused(&fx, &["adopt", "--say", SAY], "DOMAIN_NOT_IGNORED", 1, "es");
}

#[test]
fn refusal_private_folder_permissions_at_a_write_command() {
    let fx = published("write-folder");
    use std::os::unix::fs::PermissionsExt;
    fs::create_dir_all(fx.folder()).unwrap();
    fs::set_permissions(fx.folder(), fs::Permissions::from_mode(0o755)).unwrap();
    let name = proposal(&fx, "plan.json", &file(json!({})));
    refused(&fx, &["propose", &name], "PRIVATE_FOLDER_PERMISSIONS", 1, "es");
}
