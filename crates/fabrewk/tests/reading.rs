//! The reading commands through the built binary (part 7d2, D-053, D-055): `status`, `config` and `check`, the
//! refusals they raise (11.2 rule 4; PD-35), the exit statuses (REQ-CLI-3), the documents (REQ-CLI-4), no lock and
//! no write (REQ-VIEW-4), the person and their language (D-055), the controls listed (REQ-SEC-5), the credential scan
//! at `check` (REQ-SEC-3), and tools that fail (PD-36).

mod common;

use common::*;
use fabrewk_core::cli::{self, CHECKS, Parsed};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::os::unix::fs::PermissionsExt;

/// A human output holds no placeholder, no internal key and no command the declaration refuses (PD-36, PD-38).
fn whole(text: &str) {
    assert!(!text.contains(['{', '}']), "a placeholder: {text}");
    assert!(!text.contains("(a message the catalogue lacks)") && !text.contains("(un mensaje que falta"), "{text}");
    for line in text.lines().map(str::trim).filter(|l| l.starts_with("fabrewk ") && !l.contains(['<', '['])) {
        let words: Vec<&str> = line.split(' ').collect();
        assert!(cli::parse(words[1..].iter().map(std::ffi::OsString::from)).is_ok(), "{line}");
    }
}

/// Runs `args` as a person and with `--json`; asserts that nothing changed on disk, the exit status, that the code
/// is in both outputs with the catalogue's reason and next step in `lang`, and that a command to run parses (11.2
/// rule 4). Gives the document's entry for the code.
fn refused(fx: &Fx, args: &[&str], code: &str, exit: i32, lang: &str) -> Value {
    let before = fx.snapshot();
    let human = fx.run(args);
    let json = fx.run(&[args, &["--json"]].concat());
    assert_eq!(fx.snapshot(), before, "{args:?} wrote");
    assert_eq!((human.status, json.status), (exit, exit), "{args:?}: {human:?}");
    whole(&human.all());
    assert!(
        human.all().contains(&format!("{code}: {}", cli::say(lang, &format!("code.{code}.reason"), &[]))),
        "{human:?}"
    );
    assert!(human.all().contains(&cli::say(lang, &format!("code.{code}.next"), &[])), "{human:?}");
    let doc = json.doc();
    let list = if doc["schema"] == "fabrewk.check/1" { &doc["findings"] } else { &doc["refusals"] };
    let entry = list.as_array().unwrap().iter().find(|e| e["code"] == code).unwrap_or_else(|| panic!("{code}: {doc}"));
    assert_eq!(entry["next"], cli::say(lang, &format!("code.{code}.next"), &[]));
    if let Some(run) = entry["run"].as_str() {
        assert!(human.all().contains(run));
        assert!(matches!(cli::parse(run.split(' ').skip(1).map(std::ffi::OsString::from)), Ok(Parsed::Run { .. })));
    }
    entry.clone()
}

/// The codes of `check --json`'s findings, with their severity.
fn findings(fx: &Fx) -> Vec<(String, String)> {
    let doc = fx.run(&["check", "--json"]).doc();
    let list = doc["findings"].as_array().unwrap();
    list.iter().map(|f| (f["code"].as_str().unwrap().to_owned(), f["severity"].as_str().unwrap().to_owned())).collect()
}

fn published(name: &str) -> Fx {
    Fx::new(name, "published")
}

#[test]
fn req_view_4_reading_commands_take_no_lock_and_write_nothing() {
    let fx = published("view-4");
    let (_, task) = fx.stage_and_task();
    fx.commit("record");
    let lock = fs::File::options().create(true).truncate(false).write(true).open(fx.domain.join("local/lock")).unwrap();
    lock.lock().unwrap();
    let before = fx.snapshot();
    for args in [&["status"][..], &["status", &task], &["config"], &["config", "limits.rounds"], &["check"]] {
        for json in [false, true] {
            let all = [args, if json { &["--json"][..] } else { &[] }].concat();
            let out = fx.run(&all);
            assert_eq!(out.status, 0, "{all:?}: {out:?}");
            if !json {
                whole(&out.stdout);
            }
        }
    }
    assert_eq!(fx.snapshot(), before);
    drop(lock);
}

#[test]
fn req_cli_3_each_exit_status() {
    let fx = published("cli-3");
    assert_eq!(fx.run(&["status"]).status, 0);
    assert_eq!(fx.run(&["config", "nosuch.key"]).status, 1);
    let usage = fx.run(&["status", "--nope"]);
    assert_eq!((usage.status, usage.stdout.as_str()), (2, ""));
    assert!(usage.stderr.contains("fabrewk status --help"), "{usage:?}");
    let doc = fx.run(&["status", "--nope", "--json"]).doc();
    assert_eq!((doc["schema"].as_str(), doc["run"].as_str()), (Some("fabrewk.usage/1"), Some("fabrewk status --help")));
    assert_eq!(fx.run(&[]).status, 2);
    fs::write(fx.domain.join(".gitignore"), "local/\nmore/\n").unwrap();
    assert_eq!(fx.run(&["check"]).status, 3);
    fs::write(fx.domain.join(".gitignore"), "local/\n").unwrap();
    assert_eq!(fx.run(&["check"]).status, 0);
    // An internal error: the project file that `config` compares with the policy does not read.
    fs::set_permissions(fx.domain.join("settings.toml"), fs::Permissions::from_mode(0o000)).unwrap();
    let internal = fx.run(&["config"]);
    assert_eq!(internal.status, 70, "{internal:?}");
    assert!(internal.stderr.contains(&cli::say("es", "internal.io", &[("path", ".brewstillery/settings.toml")])));
    assert_eq!(fx.run(&["config", "--json"]).doc()["schema"], "fabrewk.internal/1");
    assert_eq!(fx.run(&["--help"]).status, 0);
}

#[test]
fn d_055_the_person_is_the_maintainer_whose_key_git_signs_with() {
    let fx = published("person");
    let es = cli::say("es", "status.adopted", &[]);
    let en = cli::say("en", "status.adopted", &[]);
    let person = |fx: &Fx| fx.run(&["config", "--json"]).doc()["person"].clone();
    assert!(fx.run(&["status"]).stdout.contains(&es));
    assert_eq!(person(&fx), "xel");
    // The private key's path, with its `.pub` beside it, and a `key::` literal name the same person.
    fx.git(&["config", "user.signingkey", fx.keys.join("xel").to_str().unwrap()]);
    assert_eq!(person(&fx), "xel");
    fx.git(&["config", "user.signingkey", &format!("key::{}", public(&fx.keys, "xel"))]);
    assert_eq!(person(&fx), "xel");
    // A key the policy does not hold, and none, name nobody: the canonical language, no personal file.
    fx.git(&["config", "user.signingkey", fx.keys.join("stranger.pub").to_str().unwrap()]);
    assert_eq!(person(&fx), Value::Null);
    assert!(fx.run(&["status"]).stdout.contains(&en));
    fx.git(&["config", "--unset", "user.signingkey"]);
    assert_eq!(person(&fx), Value::Null);
    let out = fx.run(&["config"]);
    assert!(out.stdout.contains(&cli::say("en", "config.heading", &[("digest", "")])[..40]), "{out:?}");
    let doc = fx.run(&["config", "languages.interfaceLanguage", "--json"]).doc();
    assert_eq!((doc["value"].as_str(), doc["source"].as_str()), (Some("en"), Some("default")));
}

#[test]
fn status_shows_each_stage_and_task_and_an_identifier_its_events() {
    let fx = published("status");
    let (stage, task) = fx.stage_and_task();
    let doc = fx.run(&["status", "--json"]).doc();
    assert_eq!(doc["record"], "replays");
    assert_eq!(doc["mode"], "published");
    assert_eq!(doc["policy"]["pending"], false);
    assert_eq!(doc["stages"][0]["id"], stage.as_str());
    assert_eq!(doc["stages"][0]["state"], "proposed");
    assert_eq!(doc["stages"][0]["tasks"][0]["id"], task.as_str());
    let human = fx.run(&["status"]).stdout;
    whole(&human);
    assert!(human.contains(&cli::say(
        "es",
        "status.task",
        &[("id", &task), ("state", "propuesta"), ("goal", "Make it.")]
    )));
    for id in [task.clone(), format!("task-{task}")] {
        let doc = fx.run(&["status", &id, "--json"]).doc();
        assert_eq!((doc["subject"].as_str(), doc["state"].as_str()), (Some("task"), Some("proposed")));
        assert_eq!(doc["stage"], stage.as_str());
        assert_eq!(doc["events"][0]["kind"], "task-proposed");
        assert!(doc["allowed"].as_array().unwrap().contains(&"task-authorized".into()));
    }
    let out = fx.run(&["status", &stage]);
    assert_eq!(out.status, 0);
    whole(&out.stdout);
    assert!(out.stdout.contains("stage-proposed"));
    for unknown in ["t-00000000", "s-0000000", "zzz", "project", "task-t-00000000"] {
        let out = fx.run(&["status", unknown]);
        assert_eq!(out.status, 2, "{unknown}: {out:?}");
        assert!(
            out.stderr.contains(&cli::say("es", "usage.id", &[("word", unknown)]))
                && out.stderr.contains("    fabrewk status\n")
        );
    }
}

#[test]
fn refusal_record_absent() {
    let fx = Fx::bare("absent", "published");
    let out = fx.run(&["status"]);
    assert_eq!((out.status, out.stdout.trim()), (0, cli::say("en", "status.absent", &[]).as_str()));
    assert_eq!(fx.run(&["status", "--json"]).doc()["record"], "absent");
    refused(&fx, &["check"], "RECORD_ABSENT", 3, "en");
    refused(&fx, &["check"], "POLICY_ABSENT", 3, "en");
    refused(&fx, &["config"], "POLICY_ABSENT", 1, "en");
    // Outside any domain.
    let dir = tmp("nowhere");
    assert_eq!(run_in(&dir, &fx.home, &["status", "--json"], None).doc()["record"], "absent");
    let out = run_in(&dir, &fx.home, &["check", "--json"], None);
    assert_eq!(out.status, 3);
    assert_eq!(out.doc()["findings"][0]["code"], "RECORD_ABSENT");
}

#[test]
fn a_record_that_does_not_replay_is_shown_by_status_and_refused_by_config_and_check() {
    let fx = published("broken");
    let dir = fx.domain.join("data/events/project");
    let first = fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .min()
        .unwrap();
    let mut bytes = fs::read(&first).unwrap();
    bytes[10] ^= 1;
    fs::write(&first, bytes).unwrap();
    let doc = fx.run(&["status", "--json"]).doc();
    assert_eq!(
        (doc["record"].as_str(), doc["problems"][0]["code"].as_str()),
        (Some("broken"), Some("RECORD_DIGEST_MISMATCH"))
    );
    whole(&fx.run(&["status"]).stdout);
    refused(&fx, &["check"], "RECORD_DIGEST_MISMATCH", 3, "en");
    refused(&fx, &["config"], "RECORD_DIGEST_MISMATCH", 1, "en");
}

#[test]
fn req_set_7_config_shows_every_value_with_its_source_and_one_key() {
    let fx = published("config");
    let doc = fx.run(&["config", "--json"]).doc();
    let values: BTreeMap<String, Value> =
        doc["values"].as_array().unwrap().iter().map(|v| (v["key"].as_str().unwrap().to_owned(), v.clone())).collect();
    let digest = doc["policy"].as_str().unwrap();
    assert_eq!(
        (values["project.name"]["source"].as_str(), values["project.name"]["from"].as_str()),
        (Some("project"), Some(digest))
    );
    assert_eq!(values["languages.agentInteraction"]["from"], "xel");
    assert_eq!(values["languages.interfaceLanguage"]["value"], "es-MX");
    assert_eq!(
        (values["limits.rounds"]["value"].as_i64(), values["limits.rounds"]["source"].as_str()),
        (Some(3), Some("default"))
    );
    assert_eq!(doc["keys"][0], serde_json::json!({"alias": "xel", "kind": "ssh-ed25519", "touch": "unknown"}));
    let machine = doc["machine"].as_array().unwrap();
    assert!(machine.iter().any(|m| m["name"] == "git" && m["present"] == true && m["how"] == "observed"));
    let one = fx.run(&["config", "limits.rounds", "--json"]).doc();
    assert_eq!((one["schema"].as_str(), one["value"].as_i64()), (Some("fabrewk.value/1"), Some(3)));
    let unset = fx.run(&["config", "maintainers.xel.hostAccount", "--json"]).doc();
    assert_eq!(unset["value"], Value::Null);
    whole(&fx.run(&["config", "maintainers.xel.hostAccount"]).stdout);
    whole(&fx.run(&["config"]).stdout);
}

#[test]
fn req_sec_5_config_lists_every_control_with_its_label() {
    let fx = published("sec-5");
    let doc = fx.run(&["config", "--json"]).doc();
    let listed: BTreeSet<(String, String, String)> = doc["controls"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| {
            (
                c["subject"].as_str().unwrap().into(),
                c["guard"].as_str().unwrap().into(),
                c["control"].as_str().unwrap().into(),
            )
        })
        .collect();
    for m in fabrewk_core::table::moves() {
        for g in &m.guards {
            let label = if g.control == fabrewk_core::table::Control::Effective { "effective" } else { "cooperative" };
            assert!(listed.contains(&(m.subject.into(), g.name.into(), label.into())), "{} {}", m.subject, g.name);
        }
    }
    for c in CHECKS {
        assert!(listed.iter().any(|(s, g, _)| s == c.id && g == c.id), "{}", c.id);
    }
    let human = fx.run(&["config"]).stdout;
    assert!(human.contains("(CREDENTIAL_SUSPECTED); cooperativo") && human.contains("C9 "), "{human}");
}

#[test]
fn refusal_key_unknown() {
    let fx = published("key-unknown");
    refused(&fx, &["config", "nosuch.key"], "KEY_UNKNOWN", 1, "es");
}

#[test]
fn refusal_settings_invalid() {
    let fx = published("settings-invalid");
    let path = fx.domain.join("settings.toml");
    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, format!("{text}\n[limits]\nrounds = \"three\"\n")).unwrap();
    let e = refused(&fx, &["check"], "SETTINGS_INVALID", 3, "es");
    assert_eq!((e["path"].as_str(), e["check"].as_str()), (Some(".brewstillery/settings.toml"), Some("C1")));
    assert!(e["detail"].as_str().unwrap().starts_with("limits.rounds (line 12)"), "{e}");
    refused(&fx, &["config"], "SETTINGS_INVALID", 1, "es");
}

#[test]
fn refusal_settings_key_not_personal() {
    let fx = published("not-personal");
    fs::write(fx.domain.join("xel/settings.toml"), format!("{PERSONAL}\n[review]\ndifferentProvider = \"off\"\n"))
        .unwrap();
    // The person's file does not read, so the messages are in the canonical language.
    refused(&fx, &["check"], "SETTINGS_KEY_NOT_PERSONAL", 3, "en");
    refused(&fx, &["config"], "SETTINGS_KEY_NOT_PERSONAL", 1, "en");
}

#[test]
fn refusal_alias_mismatch() {
    let fx = published("alias-mismatch");
    fs::write(fx.domain.join("xel/settings.toml"), PERSONAL.replace("alias = \"xel\"", "alias = \"ana\"")).unwrap();
    refused(&fx, &["check"], "ALIAS_MISMATCH", 3, "en");
    refused(&fx, &["config"], "ALIAS_MISMATCH", 1, "en");
}

#[test]
fn refusal_policy_pending() {
    let fx = published("pending");
    let path = fx.domain.join("settings.toml");
    fs::write(&path, fs::read_to_string(&path).unwrap() + "\n[limits]\nrounds = 5\n").unwrap();
    let e = refused(&fx, &["check"], "POLICY_PENDING", 0, "es");
    assert_eq!((e["severity"].as_str(), e["run"].as_str()), (Some("warning"), Some("fabrewk config")));
    let doc = fx.run(&["config", "--json"]).doc();
    assert_eq!(doc["pending"][0]["key"], "limits.rounds");
    assert_eq!(doc["pending"][0]["effect"], "widening");
    assert_eq!(fx.run(&["status", "--json"]).doc()["policy"]["pending"], true);
    // The policy in force is still the adopted one.
    assert_eq!(fx.run(&["config", "limits.rounds", "--json"]).doc()["value"], 3);
}

#[test]
fn refusal_domain_unknown_entry() {
    let fx = published("unknown-entry");
    fs::write(fx.domain.join("notes.md"), "mine\n").unwrap();
    let e = refused(&fx, &["check"], "DOMAIN_UNKNOWN_ENTRY", 0, "es");
    assert_eq!(e["severity"], "warning");
    assert_eq!(findings(&fx), [("DOMAIN_UNKNOWN_ENTRY".into(), "warning".into())]);
    assert_eq!(fs::read_to_string(fx.domain.join("notes.md")).unwrap(), "mine\n");
}

#[test]
fn refusal_alias_invalid() {
    let fx = published("alias-invalid");
    fs::create_dir(fx.domain.join("Xel")).unwrap();
    refused(&fx, &["check"], "ALIAS_INVALID", 3, "es");
}

#[test]
fn refusal_gitignore_changed() {
    let fx = published("gitignore");
    fs::write(fx.domain.join(".gitignore"), "local/\n*.log\n").unwrap();
    refused(&fx, &["check"], "GITIGNORE_CHANGED", 3, "es");
}

#[test]
fn refusal_domain_not_ignored() {
    let fx = Fx::new("not-ignored", "unpublished");
    assert_eq!(fx.run(&["check"]).status, 0, "{:?}", fx.run(&["check"]));
    // On one machine, the rollback check is cooperative.
    let doc = fx.run(&["check", "--json"]).doc();
    assert!(doc["checks"].as_array().unwrap().iter().any(|c| c["check"] == "C4"));
    fs::write(fx.root.join(".git/info/exclude"), "").unwrap();
    refused(&fx, &["check"], "DOMAIN_NOT_IGNORED", 3, "es");
}

#[test]
fn refusal_private_folder_permissions() {
    let fx = published("private-folder");
    fs::create_dir_all(fx.folder()).unwrap();
    fs::set_permissions(fx.folder(), fs::Permissions::from_mode(0o700)).unwrap();
    assert_eq!(fx.run(&["check"]).status, 0);
    fs::set_permissions(fx.folder(), fs::Permissions::from_mode(0o755)).unwrap();
    let e = refused(&fx, &["check"], "PRIVATE_FOLDER_PERMISSIONS", 3, "es");
    assert_eq!(e["check"], "C11");
    refused(&fx, &["config"], "PRIVATE_FOLDER_PERMISSIONS", 1, "es");
}

#[test]
fn refusal_mark_invalid() {
    let fx = published("mark");
    fs::write(fx.domain.join("local/cache/heads.json"), "").unwrap();
    refused(&fx, &["check"], "MARK_INVALID", 3, "es");
}

#[test]
fn refusal_record_rollback() {
    let fx = published("rollback");
    let dir = fx.domain.join("data/events/project");
    let mut files: Vec<_> = fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path()).collect();
    files.sort();
    for f in files.iter().filter(|f| f.to_string_lossy().contains("000002-")) {
        fs::remove_file(f).unwrap();
    }
    refused(&fx, &["check"], "RECORD_ROLLBACK", 3, "es");
}

#[test]
fn refusal_record_uncommitted() {
    let fx = published("uncommitted");
    fx.stage_and_task();
    let e = refused(&fx, &["check"], "RECORD_UNCOMMITTED", 3, "es");
    assert_eq!(e["check"], "C10");
}

#[test]
fn refusal_check_unknown() {
    let fx = published("check-unknown");
    // `git` absent from `PATH`.
    let path = path_with(&tmp("no-git"), &["ssh-keygen", "sh"]);
    let out = fx.run_with(&["check", "--json"], Some(&path));
    assert_eq!(out.status, 3);
    let doc = out.doc();
    let unknown: Vec<&str> = doc["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["code"] == "CHECK_UNKNOWN")
        .map(|f| f["check"].as_str().unwrap())
        .collect();
    assert!(unknown.contains(&"C4") && unknown.contains(&"C10"), "{doc}");
    // The history cannot be walked whole in a shallow clone.
    let base = tmp("shallow");
    let clone = base.join("repo");
    let url = format!("file://{}", fx.root.display());
    fx.program("git", &["clone", "-q", "--depth", "1", &url, clone.to_str().unwrap()]);
    let shallow =
        Fx { root: clone.clone(), domain: clone.join(".brewstillery"), home: fx.home.clone(), keys: fx.keys.clone() };
    let e = refused(&shallow, &["check"], "CHECK_UNKNOWN", 3, "en");
    assert_eq!(e["check"], "C4");
    // A file of the domain the scan must read and cannot.
    let alambique = fx.domain.join("ALAMBIQUE.md");
    fs::write(&alambique, "# Alambique\n").unwrap();
    fx.commit("alambique");
    fs::set_permissions(&alambique, fs::Permissions::from_mode(0o000)).unwrap();
    let e = refused(&fx, &["check"], "CHECK_UNKNOWN", 3, "es");
    assert_eq!(e["check"], "C9");
    fs::set_permissions(&alambique, fs::Permissions::from_mode(0o644)).unwrap();
}

#[test]
fn refusal_credential_suspected_at_check_reads_only_what_is_cited() {
    let fx = published("credential");
    let token = format!("gh{}_{}", "p", "a".repeat(36));
    let x = fx.domain.join("xel");
    fs::write(
        x.join("fermento.md"),
        "# Fermento\n\nSee [the notes](notes.txt \"notes\") and ![a capture](./img/cap.txt#top).\n",
    )
    .unwrap();
    fs::create_dir(x.join("img")).unwrap();
    fs::write(x.join("notes.txt"), format!("token {token}\n")).unwrap();
    fs::write(x.join("img/cap.txt"), "nothing\n").unwrap();
    // Uncited material is matched by its name only, and never read.
    fs::write(x.join("other.txt"), format!("token {token}\n")).unwrap();
    fs::write(x.join("id_rsa"), "not read\n").unwrap();
    // Near misses are not refused.
    fs::write(fx.domain.join("ALAMBIQUE.md"), format!("AKIA{}\nsk-{}\n", "A".repeat(15), "b".repeat(19))).unwrap();
    fs::write(x.join("id_ed25519.pub"), "a public key\n").unwrap();
    fx.commit("material");
    fs::write(x.join("secret.bin"), "unreadable\n").unwrap();
    fs::set_permissions(x.join("secret.bin"), fs::Permissions::from_mode(0o000)).unwrap();
    let doc = fx.run(&["check", "--json"]).doc();
    let found: BTreeSet<String> = doc["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| format!("{} {}", f["code"].as_str().unwrap(), f["path"].as_str().unwrap()))
        .collect();
    let want: BTreeSet<String> =
        ["CREDENTIAL_SUSPECTED .brewstillery/xel/id_rsa", "CREDENTIAL_SUSPECTED .brewstillery/xel/notes.txt"]
            .map(str::to_owned)
            .into();
    assert_eq!(found, want);
    let e = refused(&fx, &["check"], "CREDENTIAL_SUSPECTED", 3, "es");
    assert_eq!(e["control"], "cooperative");
    assert!(e["reason"].as_str().unwrap().contains("heurística"));
    // The alambique cites too, from the domain's top.
    fs::write(fx.domain.join("ALAMBIQUE.md"), "[other](xel/other.txt)\n").unwrap();
    assert!(fx.run(&["check", "--json"]).stdout.contains("xel/other.txt"));
    fs::set_permissions(x.join("secret.bin"), fs::Permissions::from_mode(0o644)).unwrap();
}

#[test]
fn req_sec_3_this_repository_passes_the_scan_at_check() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let home = tmp("this-home");
    let doc = run_in(&root, &home, &["check", "--json"], None).doc();
    let c9: Vec<&Value> = doc["findings"].as_array().unwrap().iter().filter(|f| f["check"] == "C9").collect();
    assert!(c9.is_empty(), "{c9:?}");
}

#[test]
fn pd_36_a_tool_that_fails_reaches_the_person_in_words() {
    let fx = published("tool-failed");
    let base = tmp("fake-tools");
    let bin = base.join("fake");
    fs::create_dir_all(&bin).unwrap();
    for tool in ["git", "ssh-keygen"] {
        let script = bin.join(tool);
        fs::write(&script, "#!/bin/sh\necho RAW-TOOL-TEXT\necho RAW-TOOL-TEXT >&2\nexit 1\n").unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
    }
    let path = std::ffi::OsString::from(format!("{}:/usr/bin:/bin", bin.display()));
    for args in [&["check"][..], &["status"], &["config"]] {
        let out = fx.run_with(args, Some(&path));
        assert!(!out.all().contains("RAW-TOOL-TEXT"), "{args:?}: {out:?}");
        whole(&out.all());
        assert_ne!(out.status, 70, "{args:?}: {out:?}");
    }
    // `git` failing leaves the commit check unknown; `ssh-keygen` failing, the replay.
    let doc = fx.run_with(&["check", "--json"], Some(&path)).doc();
    for check in ["C3", "C10"] {
        assert_eq!(finding_of(&doc, check), ["CHECK_UNKNOWN"], "{check}: {doc}");
    }
}

fn type_of(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

/// Each `path:type` a document holds, arrays' elements as `[]`.
fn shape(v: &Value, at: &str, out: &mut BTreeSet<String>) {
    match v {
        Value::Array(a) => a.iter().for_each(|x| shape(x, &format!("{at}[]"), out)),
        Value::Object(o) => o.iter().for_each(|(k, x)| shape(x, &format!("{at}.{k}"), out)),
        _ => {}
    }
    out.insert(format!("{at}:{}", type_of(v)));
}

/// Whether one document holds to its pinned schema: every value of a type pinned at its path, and every object with
/// exactly the fields pinned below its path.
fn holds(v: &Value, pinned: &BTreeMap<&str, BTreeSet<&str>>, at: &str) -> Result<(), String> {
    if !pinned.get(at).is_some_and(|t| t.contains(type_of(v))) {
        return Err(format!("{at} is {}", type_of(v)));
    }
    match v {
        Value::Object(o) => {
            let want: BTreeSet<&str> = pinned
                .keys()
                .filter_map(|p| p.strip_prefix(at)?.strip_prefix('.'))
                .filter(|k| !k.contains(['.', '[']))
                .collect();
            let got: BTreeSet<&str> = o.keys().map(String::as_str).collect();
            if want != got {
                return Err(format!("{at} has the fields {got:?}, pinned {want:?}"));
            }
            o.iter().try_for_each(|(k, x)| holds(x, pinned, &format!("{at}.{k}")))
        }
        Value::Array(a) => a.iter().try_for_each(|x| holds(x, pinned, &format!("{at}[]"))),
        _ => Ok(()),
    }
}

/// The pinned fields of each document and their types (REQ-CLI-4): a change of fields needs a new `schema`.
const PINNED: &[(&str, &str)] = &[
    (
        "fabrewk.check/1",
        ":object .checks:array .checks[]:object .checks[].check:string .checks[].control:string .errors:number .findings:array .findings[]:object .findings[].check:string .findings[].code:string .findings[].control:string .findings[].detail:string .findings[].next:string .findings[].path:null|string .findings[].reason:string .findings[].run:null|string .findings[].severity:string .schema:string .warnings:number",
    ),
    (
        "fabrewk.config/1",
        ":object .controls:array .controls[]:object .controls[].at:string .controls[].control:string .controls[].guard:string .controls[].subject:string .keys:array .keys[]:object .keys[].alias:string .keys[].kind:string .keys[].touch:string .machine:array .machine[]:object .machine[].how:null|string .machine[].name:string .machine[].path:null|string .machine[].present:bool .machine[].space:string .machine[].verified:null|string .machine[].version:null|string .pending:array .pending[]:object .pending[].effect:string .pending[].key:string .pending[].place:string .person:null|string .policy:string .schema:string .tools:array .tools[]:string .values:array .values[]:object .values[].about:string .values[].from:null|string .values[].key:string .values[].source:string .values[].value:array|bool|number|string .values[].value[]:string",
    ),
    ("fabrewk.internal/1", ":object .path:string .schema:string"),
    (
        "fabrewk.refused/1",
        ":object .refusals:array .refusals[]:object .refusals[].code:string .refusals[].detail:string .refusals[].next:string .refusals[].path:null|string .refusals[].reason:string .refusals[].run:null|string .schema:string",
    ),
    (
        "fabrewk.status/1",
        ":object .mode:null|string .next:array .next[]:string .policy:null|object .policy.digest:string .policy.pending:bool|null .problems:array .problems[]:object .problems[].code:string .problems[].detail:string .problems[].next:string .problems[].path:null|string .problems[].reason:string .problems[].run:null|string .record:string .schema:string .stages:array .stages[]:object .stages[].goal:string .stages[].id:string .stages[].state:string .stages[].tasks:array .stages[].tasks[]:object .stages[].tasks[].goal:string .stages[].tasks[].id:string .stages[].tasks[].state:string",
    ),
    (
        "fabrewk.subject/1",
        ":object .allowed:array .allowed[]:string .events:array .events[]:object .events[].actor:string .events[].at:string .events[].kind:string .events[].seq:number .goal:string .id:string .lang:string .schema:string .stage:null|string .state:string .subject:string",
    ),
    ("fabrewk.usage/1", ":object .message:string .run:string .schema:string"),
    (
        "fabrewk.value/1",
        ":object .about:string .from:null|string .key:string .schema:string .source:null|string .value:array|bool|null|number|string .value[]:string",
    ),
    ("fabrewk.version/1", ":object .features:array .features[]:string .schema:string .version:string"),
];

#[test]
fn req_cli_4_every_document_holds_to_its_pinned_schema() {
    let fx = published("cli-4");
    let (_, task) = fx.stage_and_task();
    let path = fx.domain.join("settings.toml");
    fs::write(&path, fs::read_to_string(&path).unwrap() + "\n[limits]\nrounds = 5\n").unwrap();
    fs::write(fx.domain.join("notes.md"), "x\n").unwrap();
    let mut docs: Vec<Value> = [
        &["status"][..],
        &["status", &task],
        &["config"],
        &["config", "limits.rounds"],
        &["config", "maintainers.xel.hostAccount"],
        &["config", "nosuch.key"],
        &["check"],
        &["status", "--nope"],
    ]
    .iter()
    .map(|a| fx.run(&[*a, &["--json"]].concat()).doc())
    .collect();
    let broken = published("cli-4-broken");
    fs::write(broken.domain.join("data/events/project/000009-0000000000000000.json"), "{}").unwrap();
    docs.push(broken.run(&["status", "--json"]).doc());
    docs.push(broken.run(&["check", "--json"]).doc());
    fs::set_permissions(broken.domain.join("settings.toml"), fs::Permissions::from_mode(0o000)).unwrap();
    fs::remove_file(broken.domain.join("data/events/project/000009-0000000000000000.json")).unwrap();
    docs.push(broken.run(&["config", "--json"]).doc());
    docs.push(run_in(&fx.root, &fx.home, &["--version", "--json"], None).doc());
    if std::env::var_os("FABREWK_PRINT_SCHEMAS").is_some() {
        let mut seen: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for doc in &docs {
            let mut s = BTreeSet::new();
            shape(doc, "", &mut s);
            seen.entry(doc["schema"].as_str().unwrap().to_owned()).or_default().extend(s);
        }
        for (schema, fields) in seen {
            let mut by: BTreeMap<String, Vec<String>> = BTreeMap::new();
            for f in fields {
                let (p, t) = f.rsplit_once(':').unwrap();
                by.entry(p.to_owned()).or_default().push(t.to_owned());
            }
            let line: Vec<String> = by.into_iter().map(|(p, t)| format!("{p}:{}", t.join("|"))).collect();
            println!("(\"{schema}\", \"{}\"),", line.join(" "));
        }
    }
    let pins: BTreeMap<&str, BTreeMap<&str, BTreeSet<&str>>> = PINNED
        .iter()
        .map(|(schema, fields)| {
            let map = fields.split(' ').map(|f| f.rsplit_once(':').unwrap()).map(|(p, t)| (p, t.split('|').collect()));
            (*schema, map.collect())
        })
        .collect();
    let mut schemas = BTreeSet::new();
    for doc in &docs {
        let schema = doc["schema"].as_str().unwrap_or_else(|| panic!("no schema: {doc}"));
        let pinned = pins.get(schema).unwrap_or_else(|| panic!("{schema} is not pinned"));
        holds(doc, pinned, "").unwrap_or_else(|e| panic!("{schema}: {e}; a change of fields needs a new schema"));
        schemas.insert(schema);
    }
    assert_eq!(schemas.len(), PINNED.len(), "a document of every schema: {schemas:?}");
    // A document that loses a field, or changes a field's type, no longer holds.
    let mut lost = docs[4].clone();
    lost.as_object_mut().unwrap().remove("source");
    assert!(holds(&lost, &pins["fabrewk.value/1"], "").is_err());
    let mut retyped = docs[3].clone();
    retyped["value"] = serde_json::json!({"n": 3});
    assert!(holds(&retyped, &pins["fabrewk.value/1"], "").is_err());
    // Each refusal's code is in its document.
    assert_eq!(docs[5]["refusals"][0]["code"], "KEY_UNKNOWN");
}

/// The finding of `check` for one file or for one check, with its code.
fn finding_of(doc: &Value, check: &str) -> Vec<String> {
    let list = doc["findings"].as_array().unwrap();
    list.iter().filter(|f| f["check"] == check).map(|f| f["code"].as_str().unwrap().to_owned()).collect()
}

#[test]
fn p7d2_1_a_record_that_cannot_be_read_is_an_internal_error_in_words() {
    let fx = published("status-io");
    let events = fx.domain.join("data/events");
    fs::set_permissions(&events, fs::Permissions::from_mode(0o000)).unwrap();
    for args in [&["status"][..], &["status", "--json"], &["status", "t-00000000"], &["config"]] {
        let out = fx.run(args);
        assert_eq!(out.status, 70, "{args:?}: {out:?}");
        assert!(!out.all().contains("os error") && !out.all().contains("Permission denied"), "{out:?}");
    }
    let doc = fx.run(&["check", "--json"]).doc();
    assert!(!doc.to_string().contains("os error"), "{doc}");
    fs::set_permissions(&events, fs::Permissions::from_mode(0o755)).unwrap();
    // A record directory that is not a directory is a broken record, shown as such.
    fs::rename(&events, fx.domain.join("data/moved")).unwrap();
    fs::write(&events, "not a directory\n").unwrap();
    assert_eq!(fx.run(&["status", "--json"]).doc()["problems"][0]["code"], "RECORD_BROKEN");
}

/// Runs `check --json` and fails if it does not end within ten seconds.
fn check_within(fx: &Fx) -> Value {
    let mut c = std::process::Command::new(env!("CARGO_BIN_EXE_fabrewk"));
    isolate(&mut c, &fx.home, None);
    let mut child =
        c.current_dir(&fx.root).args(["check", "--json"]).stdout(std::process::Stdio::piped()).spawn().unwrap();
    for _ in 0..1000 {
        if child.try_wait().unwrap().is_some() {
            let out = child.wait_with_output().unwrap();
            return serde_json::from_slice(&out.stdout).unwrap();
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    child.kill().unwrap();
    panic!("check did not end");
}

#[test]
fn p7d2_2_a_citing_file_that_is_a_link_or_not_regular_is_never_read() {
    let fx = published("cite-links");
    let token = format!("gh{}_{}", "p", "a".repeat(36));
    let x = fx.domain.join("xel");
    fs::write(x.join("uncited.md"), "[notes](uncited.txt)\n").unwrap();
    fs::write(x.join("uncited.txt"), format!("{token}\n")).unwrap();
    std::os::unix::fs::symlink("uncited.md", x.join("fermento.md")).unwrap();
    assert!(finding_of(&check_within(&fx), "C9").is_empty());
    fs::remove_file(x.join("fermento.md")).unwrap();
    assert!(std::process::Command::new("mkfifo").arg(x.join("fermento.md")).status().unwrap().success());
    assert!(finding_of(&check_within(&fx), "C9").is_empty());
}

#[test]
fn d_056_text_shaped_as_a_link_cites_its_whole_destination() {
    let fx = published("cite-syntax");
    let token = format!("gh{}_{}", "p", "a".repeat(36));
    let x = fx.domain.join("xel");
    // D-056: a `[`, then on the same line `](`, then a destination, wherever it stands, code and titles included.
    let cases = [
        ("[notes](notes(1).txt)", true),
        ("[notes](<notes(1).txt>)", true),
        ("![a [nested] capture](notes(1).txt \"a title\")", true),
        ("[notes](notes\\(1\\).txt)", true),
        ("[notes](notes(1).txt#part)", true),
        ("[notes](notes(1).txt", true),
        ("An example: `[notes](notes(1).txt)`", true),
        ("```\n[notes](notes(1).txt)\n```", true),
        ("- ```\n  [notes](notes(1).txt)\n  ```", true),
        ("[x](other.txt \"an example [link](notes(1).txt)\")", true),
        ("[notes](other.txt \"example\n\n[link](notes(1).txt)\")", true),
        ("This is not a link: ](notes(1).txt)", false),
        ("](notes(1).txt) and then [", false),
        ("[notes]\n(notes(1).txt)", false),
        ("[notes]( notes(1).txt)", false),
        // P7D2-23: any white space ends the destination, Unicode's included.
        ("[notes](notes(1).txt\u{c}title)", true),
        ("[notes](notes(1).txt\u{b}title)", true),
        ("[notes](notes(1).txt\u{a0}title)", true),
        ("[notes](notes(1).txt\u{2028}title)", true),
        ("[notes](<notes(1).txt)", false),
        ("[notes](#notes(1).txt)", false),
        ("[notes](/notes(1).txt)", false),
        ("[notes](https://example.invalid/notes(1).txt)", false),
    ];
    fs::write(x.join("notes(1).txt"), format!("{token}\n")).unwrap();
    for (text, cites) in cases {
        fs::write(x.join("fermento.md"), format!("# Fermento\n\n{text}\n")).unwrap();
        let found = finding_of(&check_within(&fx), "C9");
        assert_eq!(found.len(), usize::from(cites), "{text}: {found:?}");
    }
}

#[test]
fn p7d2_4_c2_cannot_compare_a_project_file_that_does_not_read() {
    let fx = published("c2-unreadable");
    fs::remove_file(fx.domain.join("settings.toml")).unwrap();
    fs::create_dir(fx.domain.join("settings.toml")).unwrap();
    let doc = fx.run(&["check", "--json"]).doc();
    assert_eq!(finding_of(&doc, "C2"), ["CHECK_UNKNOWN"], "{doc}");
}

#[test]
fn p7d2_5_a_verifier_that_fails_is_unknown_not_a_bad_signature() {
    let fx = published("ssh-failed");
    let bin = tmp("ssh-failed-bin");
    for (name, body) in [("killed", "kill -KILL $$"), ("one", "exit 1")] {
        let dir = bin.join(name);
        fs::create_dir_all(&dir).unwrap();
        let script = dir.join("ssh-keygen");
        fs::write(&script, format!("#!/bin/sh\n{body}\n")).unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
        let path = std::ffi::OsString::from(format!("{}:{}", dir.display(), std::env::var("PATH").unwrap()));
        let doc = fx.run_with(&["check", "--json"], Some(&path)).doc();
        assert_eq!(finding_of(&doc, "C3"), ["CHECK_UNKNOWN"], "{name}: {doc}");
    }
}

#[test]
fn p7d2_6_a_history_that_cannot_be_walked_is_unknown() {
    let fx = published("detached");
    fx.git(&["checkout", "--detach", "-q"]);
    let doc = fx.run(&["check", "--json"]).doc();
    assert_eq!(finding_of(&doc, "C4"), ["CHECK_UNKNOWN"], "{doc}");
}

#[test]
fn p7d2_7_c4_is_cooperative_on_one_machine_wherever_it_is_listed() {
    let fx = Fx::new("c4-label", "unpublished");
    let check = fx.run(&["check", "--json"]).doc();
    let c4 = check["checks"].as_array().unwrap().iter().find(|c| c["check"] == "C4").unwrap().clone();
    assert_eq!(c4["control"], "cooperative");
    let config = fx.run(&["config", "--json"]).doc();
    assert!(
        config["controls"].as_array().unwrap().iter().any(|c| c["subject"] == "C4" && c["control"] == "cooperative")
    );
    assert!(fx.run(&["config"]).stdout.contains(&format!("C4 {}: cooperativo", cli::say("es", "check.C4", &[]))));
    let published = published("c4-label-published");
    let check = published.run(&["check", "--json"]).doc();
    assert!(check["checks"].as_array().unwrap().iter().any(|c| c["check"] == "C4" && c["control"] == "effective"));
}

#[test]
fn p7d2_8_every_control_config_lists_is_in_words() {
    let fx = published("control-words");
    let human = fx.run(&["config"]).stdout;
    for (lang, text) in cli::CATALOGUES {
        let m = cli::messages(text);
        for g in fabrewk_core::table::moves().iter().flat_map(|m| &m.guards) {
            assert!(m.contains_key(&format!("guard.{}", g.name)), "{lang} lacks guard.{}", g.name);
        }
    }
    let line = format!(
        "{} (run-started): {} (REQ-RUN-2); cooperativo",
        cli::say("es", "kind.run-started", &[]),
        cli::say("es", "guard.REQ-RUN-2", &[])
    );
    assert!(human.contains(&line), "{human}");
}

#[test]
fn p7d2_15_a_citing_file_that_is_not_utf_8_still_cites() {
    let fx = published("cite-utf8");
    let x = fx.domain.join("xel");
    fs::write(x.join("secret.txt"), format!("gh{}_{}\n", "p", "a".repeat(36))).unwrap();
    fs::write(x.join("fermento.md"), b"[notes](secret.txt)\n\xff\n").unwrap();
    assert_eq!(finding_of(&check_within(&fx), "C9"), ["CREDENTIAL_SUSPECTED"]);
}

#[test]
fn p7d2_12_a_branch_whose_commit_git_cannot_give_is_unknown() {
    let fx = published("history-fails");
    let bin = tmp("history-fails-bin");
    let real =
        String::from_utf8(std::process::Command::new("sh").args(["-c", "command -v git"]).output().unwrap().stdout)
            .unwrap();
    let script = bin.join("git");
    let body = format!(
        "#!/bin/sh\nfor a in \"$@\"; do [ \"$a\" = 'refs/heads/main^{{commit}}' ] && exit 128; done\nexec {} \"$@\"\n",
        real.trim()
    );
    fs::write(&script, body).unwrap();
    fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
    let path = std::ffi::OsString::from(format!("{}:{}", bin.display(), std::env::var("PATH").unwrap()));
    let doc = fx.run_with(&["check", "--json"], Some(&path)).doc();
    assert_eq!(finding_of(&doc, "C4"), ["CHECK_UNKNOWN"], "{doc}");
    // A ref that names an object Git does not hold.
    fs::write(fx.root.join(".git/refs/heads/main"), format!("{}\n", "0123456789".repeat(4))).unwrap();
    let doc = fx.run(&["check", "--json"]).doc();
    assert!(finding_of(&doc, "C4").contains(&"CHECK_UNKNOWN".to_owned()), "{doc}");
}

#[test]
fn p7d2_13_status_of_an_identifier_in_a_record_that_cannot_be_read() {
    let fx = published("subject-io");
    let (_, task) = fx.stage_and_task();
    let events = fx.domain.join("data/events");
    fs::set_permissions(&events, fs::Permissions::from_mode(0o000)).unwrap();
    let out = fx.run(&["status", &task, "--json"]);
    assert_eq!((out.status, out.doc()["schema"].as_str()), (70, Some("fabrewk.internal/1")), "{out:?}");
    fs::set_permissions(&events, fs::Permissions::from_mode(0o755)).unwrap();
    // A record that does not replay shows its state instead of calling the identifier unknown.
    let dir = fx.domain.join("data/events/project");
    let first = fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .min()
        .unwrap();
    let mut bytes = fs::read(&first).unwrap();
    bytes[10] ^= 1;
    fs::write(&first, bytes).unwrap();
    assert_eq!(fx.run(&["status", &task, "--json"]).doc()["record"], "broken");
}

#[test]
fn p7d2_14_settings_that_are_links_or_not_regular_are_never_opened() {
    let fx = published("settings-fifo");
    let x = fx.domain.join("xel");
    fs::write(x.join("uncited.txt"), "not = [toml\n").unwrap();
    fs::remove_file(x.join("settings.toml")).unwrap();
    std::os::unix::fs::symlink("uncited.txt", x.join("settings.toml")).unwrap();
    let doc = check_within(&fx);
    let c1: Vec<&Value> = doc["findings"].as_array().unwrap().iter().filter(|f| f["check"] == "C1").collect();
    assert_eq!(c1.len(), 1, "{doc}");
    assert!(c1[0]["detail"].as_str().unwrap().contains("not a regular file"), "{doc}");
    fs::remove_file(x.join("settings.toml")).unwrap();
    assert!(std::process::Command::new("mkfifo").arg(x.join("settings.toml")).status().unwrap().success());
    check_within(&fx);
    for args in [&["status"][..], &["--help"], &["config"]] {
        let mut c = std::process::Command::new(env!("CARGO_BIN_EXE_fabrewk"));
        isolate(&mut c, &fx.home, None);
        let mut child = c
            .current_dir(&fx.root)
            .args(args)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap();
        let ended = (0..500).any(|_| {
            std::thread::sleep(std::time::Duration::from_millis(10));
            child.try_wait().unwrap().is_some()
        });
        let _ = child.kill();
        assert!(ended, "{args:?} did not end");
    }
    for file in [fx.domain.join("settings.toml"), fx.domain.join("local/cache/heads.json")] {
        fs::remove_file(&file).unwrap();
        assert!(std::process::Command::new("mkfifo").arg(&file).status().unwrap().success());
    }
    let codes: Vec<String> = check_within(&fx)["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["code"].as_str().unwrap().to_owned())
        .collect();
    assert!(codes.contains(&"MARK_INVALID".to_owned()) && codes.contains(&"SETTINGS_INVALID".to_owned()), "{codes:?}");
}

/// Runs `args` and fails if they do not end within five seconds.
fn ends(fx: &Fx, args: &[&str]) {
    let mut c = std::process::Command::new(env!("CARGO_BIN_EXE_fabrewk"));
    isolate(&mut c, &fx.home, None);
    let mut child = c
        .current_dir(&fx.root)
        .args(args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let ended = (0..500).any(|_| {
        std::thread::sleep(std::time::Duration::from_millis(10));
        child.try_wait().unwrap().is_some()
    });
    let _ = child.kill();
    assert!(ended, "{args:?} did not end");
}

#[test]
fn p7d2_17_a_signing_key_that_is_a_pipe_never_blocks() {
    let fx = published("key-fifo");
    for (named, fifo) in [("pipe", "pipe"), ("absent", "absent.pub")] {
        let path = fx.keys.join(fifo);
        assert!(std::process::Command::new("mkfifo").arg(&path).status().unwrap().success());
        fx.git(&["config", "user.signingkey", fx.keys.join(named).to_str().unwrap()]);
        for args in [&["status"][..], &["check"], &["config"], &["--help"], &["--version"], &["status", "--nope"]] {
            ends(&fx, args);
        }
    }
}

#[test]
fn p7d2_18_no_link_on_the_way_to_a_file_is_followed() {
    // A maintainer's directory that is a link: their file is not read, and the domain reports the entry.
    let fx = published("alias-link");
    let moved = fx.root.join("../elsewhere");
    fs::rename(fx.domain.join("xel"), &moved).unwrap();
    std::os::unix::fs::symlink(&moved, fx.domain.join("xel")).unwrap();
    refused(&fx, &["config"], "SETTINGS_INVALID", 1, "en");
    assert!(findings(&fx).contains(&("DOMAIN_UNKNOWN_ENTRY".into(), "warning".into())));
    // The mark's directory that is a link: the file behind it is not parsed.
    let fx = published("mark-link");
    let cache = fx.domain.join("local/cache");
    let away = fx.domain.join("xel/uncited-cache");
    fs::rename(&cache, &away).unwrap();
    fs::write(away.join("heads.json"), "UNCITED NOT JSON").unwrap();
    std::os::unix::fs::symlink("../xel/uncited-cache", &cache).unwrap();
    let e = refused(&fx, &["check"], "MARK_INVALID", 3, "es");
    assert_eq!(e["detail"], "not a regular file", "{e}");
    // A domain that is a link: no record, policy or settings is read through it.
    let fx = published("domain-link");
    let away = fx.root.join("../domain-elsewhere");
    fs::rename(&fx.domain, &away).unwrap();
    std::os::unix::fs::symlink(&away, &fx.domain).unwrap();
    let doc = fx.run(&["status", "--json"]).doc();
    assert_eq!((doc["record"].as_str(), doc["problems"][0]["code"].as_str()), (Some("broken"), Some("RECORD_BROKEN")));
    refused(&fx, &["config"], "RECORD_BROKEN", 1, "en");
    refused(&fx, &["check"], "RECORD_BROKEN", 3, "en");
    // A private folder that is a link: its machine file is not read.
    let fx = published("folder-link");
    let away = fx.home.join("elsewhere");
    fs::create_dir_all(&away).unwrap();
    fs::set_permissions(&away, fs::Permissions::from_mode(0o700)).unwrap();
    fs::write(away.join("machine.toml"), "not = [toml\n").unwrap();
    fs::create_dir_all(fx.home.join(".config")).unwrap();
    std::os::unix::fs::symlink(&away, fx.folder()).unwrap();
    let doc = fx.run(&["check", "--json"]).doc();
    assert_eq!(finding_of(&doc, "C1"), Vec::<String>::new(), "{doc}");
    assert_eq!(finding_of(&doc, "C11"), ["PRIVATE_FOLDER_PERMISSIONS"], "{doc}");
}

#[test]
fn p7d2_21_c1_cannot_list_what_it_must_read() {
    let fx = published("c1-list");
    let ana = fx.domain.join("ana");
    fs::create_dir(&ana).unwrap();
    fs::write(ana.join("settings.toml"), "[maintainer]\nalias = \"ana\"\n").unwrap();
    fs::set_permissions(&ana, fs::Permissions::from_mode(0o000)).unwrap();
    let doc = fx.run(&["check", "--json"]).doc();
    assert_eq!(finding_of(&doc, "C1"), ["CHECK_UNKNOWN"], "{doc}");
    fs::set_permissions(&ana, fs::Permissions::from_mode(0o755)).unwrap();
    fs::set_permissions(&fx.domain, fs::Permissions::from_mode(0o111)).unwrap();
    let doc = fx.run(&["check", "--json"]).doc();
    fs::set_permissions(&fx.domain, fs::Permissions::from_mode(0o755)).unwrap();
    assert!(finding_of(&doc, "C1").contains(&"CHECK_UNKNOWN".to_owned()), "{doc}");
}

#[test]
fn p7d2_22_a_signing_key_in_the_domain_is_never_read() {
    let fx = published("key-in-domain");
    let x = fx.domain.join("xel");
    std::os::unix::fs::symlink(fx.keys.join("xel.pub"), x.join("uncited-key")).unwrap();
    fs::copy(fx.keys.join("xel.pub"), x.join("copied-key")).unwrap();
    fs::create_dir(fx.root.join("outside")).unwrap();
    std::os::unix::fs::symlink(x.join("copied-key"), fx.root.join("outside/key")).unwrap();
    for named in [x.join("uncited-key"), x.join("copied-key"), fx.root.join("outside/key")] {
        fx.git(&["config", "user.signingkey", named.to_str().unwrap()]);
        assert_eq!(fx.run(&["config", "--json"]).doc()["person"], Value::Null, "{}", named.display());
    }
    // Outside the repository and every domain, a key reached through a link is the person's own.
    let own = tmp("key-own").join("own");
    std::os::unix::fs::symlink(fx.keys.join("xel.pub"), &own).unwrap();
    fx.git(&["config", "user.signingkey", own.to_str().unwrap()]);
    assert_eq!(fx.run(&["config", "--json"]).doc()["person"], "xel");
}

#[test]
fn p7d2_24_a_private_folder_that_cannot_be_examined_is_unknown() {
    let fx = published("folder-unknown");
    let folder = fx.folder();
    fs::create_dir_all(&folder).unwrap();
    fs::write(folder.join("machine.toml"), "not = [toml\n").unwrap();
    fs::set_permissions(&folder, fs::Permissions::from_mode(0o000)).unwrap();
    let doc = fx.run(&["check", "--json"]).doc();
    let config = fx.run(&["config", "--json"]);
    fs::set_permissions(&folder, fs::Permissions::from_mode(0o700)).unwrap();
    assert_eq!(doc["errors"].as_u64().map(|n| n > 0), Some(true), "{doc}");
    assert!(finding_of(&doc, "C1").contains(&"CHECK_UNKNOWN".to_owned()), "{doc}");
    assert_eq!(config.status, 70, "{config:?}");
    // A parent that cannot be searched, and a dangling link where the folder stands.
    fs::write(folder.join("machine.toml"), "").unwrap();
    fs::set_permissions(&folder, fs::Permissions::from_mode(0o777)).unwrap();
    let parent = fx.home.join(".config");
    fs::set_permissions(&parent, fs::Permissions::from_mode(0o600)).unwrap();
    let doc = fx.run(&["check", "--json"]).doc();
    fs::set_permissions(&parent, fs::Permissions::from_mode(0o755)).unwrap();
    assert_eq!(finding_of(&doc, "C11"), ["CHECK_UNKNOWN"], "{doc}");
    fs::remove_dir_all(&folder).unwrap();
    std::os::unix::fs::symlink(fx.home.join("nowhere"), &folder).unwrap();
    let doc = fx.run(&["check", "--json"]).doc();
    assert_eq!(finding_of(&doc, "C11"), ["PRIVATE_FOLDER_PERMISSIONS"], "{doc}");
}

#[test]
fn p7d2_25_the_version_reads_nothing() {
    let fx = published("version-reads");
    let (bin, marker) = (tmp("version-bin"), tmp("version-marker").join("ran"));
    for tool in ["git", "ssh-keygen"] {
        let script = bin.join(tool);
        fs::write(&script, format!("#!/bin/sh\necho {tool} >> '{}'\nexit 1\n", marker.display())).unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
    }
    let path = std::ffi::OsString::from(format!("{}:{}", bin.display(), std::env::var("PATH").unwrap()));
    for args in [&["--version"][..], &["--version", "--json"]] {
        assert_eq!(fx.run_with(args, Some(&path)).status, 0);
    }
    assert!(!marker.exists(), "--version ran {}", fs::read_to_string(&marker).unwrap_or_default());
    fx.run_with(&["status"], Some(&path));
    assert!(marker.exists(), "the fake tools are on the way of a command that reads");
}

#[test]
fn p7d2_26_config_never_takes_an_unreadable_personal_file_for_an_absent_one() {
    let fx = published("personal-unreadable");
    let x = fx.domain.join("xel");
    fs::write(x.join("settings.toml"), "[maintainer]\nalias = \"xel\"\n\n[environment]\nshell = \"fish\"\n").unwrap();
    assert_eq!(fx.run(&["config", "environment.shell", "--json"]).doc()["source"], "personal");
    fs::set_permissions(&x, fs::Permissions::from_mode(0o000)).unwrap();
    let (one, all) = (fx.run(&["config", "environment.shell", "--json"]), fx.run(&["config", "--json"]));
    fs::set_permissions(&x, fs::Permissions::from_mode(0o755)).unwrap();
    assert_eq!((one.status, all.status), (70, 70), "{one:?} {all:?}");
    assert_eq!(one.doc()["schema"], "fabrewk.internal/1");
}

#[test]
fn p7d2_27_a_domain_that_cannot_be_examined_is_not_absent() {
    let fx = published("root-unsearchable");
    // The command runs in the root, which it can no longer search once it is there.
    let mut c = std::process::Command::new("sh");
    isolate(&mut c, &fx.home, None);
    let script = format!("chmod 000 . && exec '{}' check --json", env!("CARGO_BIN_EXE_fabrewk"));
    let out = c.current_dir(&fx.root).args(["-c", &script]).output().unwrap();
    fs::set_permissions(&fx.root, fs::Permissions::from_mode(0o755)).unwrap();
    let doc: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!((out.status.code(), finding_of(&doc, "C4")), (Some(3), vec!["CHECK_UNKNOWN".to_owned()]), "{doc}");
    // An absent domain stays absent.
    fs::rename(&fx.domain, fx.root.join("elsewhere")).unwrap();
    assert_eq!(finding_of(&fx.run(&["check", "--json"]).doc(), "C4"), ["RECORD_ABSENT"]);
}

#[test]
fn p7d2_28_a_private_folder_in_the_repository_is_never_read() {
    let fx = published("folder-inside");
    let uncited = fx.domain.join("xel/uncited");
    fs::create_dir_all(uncited.join("fabrewk")).unwrap();
    fs::set_permissions(uncited.join("fabrewk"), fs::Permissions::from_mode(0o700)).unwrap();
    fs::write(uncited.join("fabrewk/machine.toml"), "not valid TOML [\n").unwrap();
    let run = |args: &[&str]| {
        let mut c = std::process::Command::new(env!("CARGO_BIN_EXE_fabrewk"));
        isolate(&mut c, &fx.home, None);
        let out = c.current_dir(&fx.root).env("XDG_CONFIG_HOME", &uncited).args(args).output().unwrap();
        (out.status.code().unwrap(), serde_json::from_slice::<Value>(&out.stdout).unwrap())
    };
    let (status, doc) = run(&["check", "--json"]);
    assert_eq!(
        (status, finding_of(&doc, "C1"), finding_of(&doc, "C11")),
        (3, vec![], vec!["PRIVATE_FOLDER_PERMISSIONS".to_owned()]),
        "{doc}"
    );
    let (status, doc) = run(&["config", "--json"]);
    assert_eq!((status, doc["refusals"][0]["code"].as_str()), (1, Some("PRIVATE_FOLDER_PERMISSIONS")), "{doc}");
}

#[test]
fn p7d2_29_a_private_folder_in_another_domain_is_never_read() {
    let (a, b) = (published("folder-other-a"), published("folder-other-b"));
    let uncited = b.domain.join("xel/uncited");
    fs::create_dir_all(uncited.join("fabrewk")).unwrap();
    fs::set_permissions(uncited.join("fabrewk"), fs::Permissions::from_mode(0o700)).unwrap();
    fs::write(uncited.join("fabrewk/machine.toml"), "not valid TOML [\n").unwrap();
    let outside = tmp("folder-other-link");
    std::os::unix::fs::symlink(&uncited, outside.join("config")).unwrap();
    for xdg in [uncited.clone(), outside.join("config")] {
        let mut c = std::process::Command::new(env!("CARGO_BIN_EXE_fabrewk"));
        isolate(&mut c, &a.home, None);
        let out = c.current_dir(&a.root).env("XDG_CONFIG_HOME", &xdg).args(["check", "--json"]).output().unwrap();
        let doc: Value = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(
            (finding_of(&doc, "C1"), finding_of(&doc, "C11")),
            (vec![], vec!["PRIVATE_FOLDER_PERMISSIONS".to_owned()]),
            "{doc}"
        );
    }
}

#[test]
fn p7d2_30_a_signing_key_in_another_domain_is_never_read() {
    let (a, b) = (published("key-other-a"), published("key-other-b"));
    let key = b.domain.join("xel/uncited-key.txt");
    fs::copy(a.keys.join("xel.pub"), &key).unwrap();
    fs::copy(a.keys.join("xel.pub"), b.domain.join("xel/private.pub")).unwrap();
    let outside = tmp("key-other-link");
    std::os::unix::fs::symlink(&key, outside.join("key")).unwrap();
    for named in [key.clone(), outside.join("key"), b.domain.join("xel/private")] {
        a.git(&["config", "user.signingkey", named.to_str().unwrap()]);
        assert_eq!(a.run(&["config", "--json"]).doc()["person"], Value::Null, "{}", named.display());
    }
    a.git(&["config", "user.signingkey", a.keys.join("xel.pub").to_str().unwrap()]);
    assert_eq!(a.run(&["config", "--json"]).doc()["person"], "xel");
}
