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
    // An internal error: the project file that `config` compares with the policy is a directory.
    fs::remove_file(fx.domain.join("settings.toml")).unwrap();
    fs::create_dir(fx.domain.join("settings.toml")).unwrap();
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
    assert!(human.contains("CREDENTIAL_SUSPECTED (cooperativo)") && human.contains("C9 "), "{human}");
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
    let doc = fx.run_with(&["check", "--json"], Some(&path)).doc();
    assert!(
        doc["findings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|f| f["code"] == "CHECK_UNKNOWN" || f["code"] == "RECORD_SIGNATURE_INVALID")
    );
}

/// Each `(path, type)` a document holds, arrays as `[]`.
fn shape(v: &Value, at: &str, out: &mut BTreeSet<String>) {
    let ty = match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(a) => {
            a.iter().for_each(|x| shape(x, &format!("{at}[]"), out));
            "array"
        }
        Value::Object(o) => {
            o.iter().for_each(|(k, x)| shape(x, &format!("{at}.{k}"), out));
            "object"
        }
    };
    out.insert(format!("{at}:{ty}"));
}

/// The pinned fields of each document (REQ-CLI-4): a change of fields needs a new `schema`.
const PINNED: &[(&str, &str)] = &[
    (
        "fabrewk.status/1",
        ".mode .next .next[] .policy .policy.digest .policy.pending .problems .problems[] .problems[].code .problems[].detail .problems[].next .problems[].path .problems[].reason .problems[].run .record .schema .stages .stages[] .stages[].goal .stages[].id .stages[].state .stages[].tasks .stages[].tasks[] .stages[].tasks[].goal .stages[].tasks[].id .stages[].tasks[].state",
    ),
    (
        "fabrewk.subject/1",
        ".allowed .allowed[] .events .events[] .events[].actor .events[].at .events[].kind .events[].seq .goal .id .lang .schema .stage .state .subject",
    ),
    (
        "fabrewk.config/1",
        ".controls .controls[] .controls[].at .controls[].control .controls[].guard .controls[].subject .keys .keys[] .keys[].alias .keys[].kind .keys[].touch .machine .machine[] .machine[].how .machine[].name .machine[].path .machine[].present .machine[].space .machine[].verified .machine[].version .pending .pending[] .pending[].effect .pending[].key .pending[].place .person .policy .schema .tools .values .values[] .values[].about .values[].from .values[].key .values[].source .values[].value .values[].value[]",
    ),
    ("fabrewk.value/1", ".about .from .key .schema .source .value"),
    (
        "fabrewk.check/1",
        ".checks .checks[] .checks[].check .checks[].control .errors .findings .findings[] .findings[].check .findings[].code .findings[].control .findings[].detail .findings[].next .findings[].path .findings[].reason .findings[].run .findings[].severity .schema .warnings",
    ),
    (
        "fabrewk.refused/1",
        ".refusals .refusals[] .refusals[].code .refusals[].detail .refusals[].next .refusals[].path .refusals[].reason .refusals[].run .schema",
    ),
    ("fabrewk.usage/1", ".message .run .schema"),
    ("fabrewk.internal/1", ".path .schema"),
    ("fabrewk.version/1", ".features .schema .version"),
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
    fs::remove_file(broken.domain.join("settings.toml")).unwrap();
    fs::create_dir(broken.domain.join("settings.toml")).unwrap();
    fs::remove_file(broken.domain.join("data/events/project/000009-0000000000000000.json")).unwrap();
    docs.push(broken.run(&["config", "--json"]).doc());
    docs.push(run_in(&fx.root, &fx.home, &["--version", "--json"], None).doc());
    let mut seen: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for doc in &docs {
        let schema = doc["schema"].as_str().unwrap_or_else(|| panic!("no schema: {doc}")).to_owned();
        let mut s = BTreeSet::new();
        shape(doc, "", &mut s);
        seen.entry(schema).or_default().extend(s);
    }
    for (schema, fields) in PINNED {
        let pinned: BTreeSet<&str> = fields.split(' ').collect();
        let got = seen.get(*schema).unwrap_or_else(|| panic!("no document of {schema}"));
        let paths: BTreeSet<&str> =
            got.iter().filter_map(|f| f.rsplit_once(':')).map(|(p, _)| p).filter(|p| !p.is_empty()).collect();
        assert_eq!(paths, pinned, "{schema}: its fields changed; give it a new schema");
    }
    assert_eq!(seen.len(), PINNED.len(), "{:?}", seen.keys());
    // Each refusal's code is in its document.
    assert_eq!(docs[5]["refusals"][0]["code"], "KEY_UNKNOWN");
}
