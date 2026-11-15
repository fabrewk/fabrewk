//! Extensions through the command line (REQ-EXT-1 to REQ-EXT-5, REQ-CLI-3, REQ-SEC-2; S-9; part 9h, D-094). Every
//! extension here is a shell script installed outside the repository and the system directories, so S-9's first case
//! runs in each test that runs one. The cells are real: these tests need `bwrap`, `pasta` and `python3`.

mod common;

use common::{Fx, Ran, path_with, whole};
use fabrewk_core::cli::say;
use serde_json::{Value, json};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

/// The language the fixture's maintainer reads (`es-MX`, which falls back to `es`).
const LANG: &str = "es";

/// The directory beside the fixture's repository where its extensions are installed: outside the repository and the
/// system directories (S-9).
fn bin(fx: &Fx) -> PathBuf {
    fx.root.parent().unwrap().join("ext-bin")
}

/// A fixture whose policy permits the extensions of `more` (tables `[extensions.<name>]`), with [`bin`] first in
/// `PATH`.
fn fixture(name: &str, more: &str) -> Fx {
    fixture_in(name, "unpublished", more)
}

/// [`fixture`] in the publication mode `mode`.
fn fixture_in(name: &str, mode: &str, more: &str) -> Fx {
    let mut fx = Fx::new_with(name, mode, more);
    fs::create_dir_all(bin(&fx)).unwrap();
    let path = std::env::join_paths(
        std::iter::once(bin(&fx)).chain(std::env::split_paths(&std::env::var_os("PATH").unwrap())),
    );
    fx.path = Some(path.unwrap());
    fx
}

/// Installs `fabrewk-<name>`, a shell script of `script`, and gives its digest.
fn install(fx: &Fx, name: &str, script: &str) -> String {
    install_with(fx, name, "#!/bin/sh", script)
}

/// [`install`] with its own first line.
fn install_with(fx: &Fx, name: &str, shebang: &str, script: &str) -> String {
    let file = bin(fx).join(format!("fabrewk-{name}"));
    let text = format!("{shebang}\n{script}\n");
    fs::write(&file, &text).unwrap();
    fs::set_permissions(&file, fs::Permissions::from_mode(0o755)).unwrap();
    fabrewk_core::record::sha256_hex(text.as_bytes())
}

/// The extension `name`'s own area, where its home and `/tmp` lie.
fn area(fx: &Fx, name: &str) -> PathBuf {
    fx.domain.join("local/ext").join(name)
}

/// One request line, as an extension prints it with `printf '%s\n'`.
fn line(v: &Value) -> String {
    format!("printf '%s\\n' '{}'", serde_json::to_string(v).unwrap())
}

fn note(text: &str) -> Value {
    json!({"protocol": 1, "request": "note", "text": text})
}

/// A `propose` request: a stage with one task, as `propose`'s file holds them.
fn proposal(base: &str) -> Value {
    json!({
        "protocol": 1, "request": "propose",
        "stage": {"lang": "en", "goal": "Index it.", "budget": {"dispatches": 6, "minutes": 120}, "checks": {}, "settings": {}},
        "tasks": [{
            "lang": "en", "goal": "Make the index.", "requirements": [{"text": "It builds.", "criterion": "cargo build passes"}],
            "base": base, "dependencies": [], "allowed": ["src/**"], "profiles": {"build": "build", "review": "review"},
            "checks": [], "limits": {"rounds": 3, "dispatches": 6, "minutes": 120}, "delegationLanguage": "en",
            "settings": {},
        }],
    })
}

/// Every stream of the record, by name, with its number of events.
fn streams(fx: &Fx) -> Vec<(String, usize)> {
    let dir = fx.domain.join("data/events");
    let mut out: Vec<(String, usize)> = fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .map(|s| (s.clone(), fx.events(&s).len()))
        .collect();
    out.sort();
    out
}

/// A refusal of `fabrewk <args>`: its exit status, its code with the catalogue's reason and next step on standard
/// error, and nothing in the record.
fn refused(fx: &Fx, args: &[&str], code: &str, exit: i32) -> Ran {
    let before = streams(fx);
    let ran = fx.run(args);
    assert_eq!(ran.status, exit, "{ran:?}");
    whole(&ran.all());
    assert!(ran.stderr.contains(&format!("{code}: {}", say(LANG, &format!("code.{code}.reason"), &[]))), "{ran:?}");
    assert!(ran.stderr.contains(&say(LANG, &format!("code.{code}.next"), &[])), "{ran:?}");
    assert_eq!(streams(fx), before, "{args:?} wrote");
    ran
}

#[test]
fn req_ext_1_a_permitted_present_extension_runs_and_nothing_else_does() {
    let fx = fixture("ext-1", "\n[extensions.hello]\n\n[extensions.absent]\n\n[extensions.status]\n");
    let hello = install(&fx, "hello", "echo \"$@\" > \"$HOME/args\"");
    let ran = fx.run(&["hello", "a", "--json", "b c"]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(ran.stdout, say(LANG, "extension.none", &[("name", "hello")]) + "\n");
    // Every word after its name is its own, `--json` among them.
    assert_eq!(fs::read_to_string(area(&fx, "hello").join("home/args")).unwrap(), "a --json b c\n");
    // Permitted but absent, or present but not permitted: a usage error, as an unknown command is.
    install(&fx, "other", "touch \"$HOME/ran\"");
    for name in ["absent", "other", "Bad-Name"] {
        let ran = fx.run(&[name]);
        assert_eq!(ran.status, 2, "{name}: {ran:?}");
        assert!(ran.stderr.contains(&say(LANG, "usage.command", &[("word", name)])), "{ran:?}");
    }
    assert!(!area(&fx, "other").exists());
    // A core command's name is never an extension's: `fabrewk status` is the core's, even with `fabrewk-status`
    // present and permitted.
    install(&fx, "status", "touch \"$HOME/ran\"");
    let ran = fx.run(&["status", "--json"]);
    assert_eq!((ran.status, ran.doc()["schema"].as_str()), (0, Some("fabrewk.status/3")), "{ran:?}");
    assert!(!area(&fx, "status").exists());
    // With a record that does not replay, a present extension's permission cannot be read: the record's refusal; an
    // absent one is still an unknown command.
    let event = fs::read_dir(fx.domain.join("data/events/project"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .find(|p| p.extension().is_some_and(|e| e == "json"));
    let event = event.unwrap();
    let bytes = fs::read(&event).unwrap();
    fs::write(&event, [&bytes[..], b" "].concat()).unwrap();
    let ran = fx.run(&["hello"]);
    assert_eq!(ran.status, 1, "{ran:?}");
    assert!(ran.stderr.contains("RECORD_"), "{ran:?}");
    assert_eq!(fx.run(&["absent"]).status, 2);
    fs::write(&event, bytes).unwrap();
    // A pinned digest that differs is refused before it runs; the one pinned runs.
    let script = "touch \"$HOME/ran\"";
    let digest = fabrewk_core::record::sha256_hex(format!("#!/bin/sh\n{script}\n").as_bytes());
    for (pin, runs) in [("0".repeat(64), false), (digest.clone(), true)] {
        let fx = fixture("ext-1-pin", &format!("\n[extensions.hello]\ndigest = \"{pin}\"\n"));
        assert_eq!(install(&fx, "hello", script), digest);
        if runs {
            assert_eq!(fx.run(&["hello"]).status, 0);
        } else {
            let ran = refused(&fx, &["hello"], "EXTENSION_DIGEST", 1);
            assert!(ran.stderr.contains(&digest), "{ran:?}");
        }
        assert_eq!(area(&fx, "hello").join("home/ran").exists(), runs);
    }
    // `config` shows each permitted extension with its path and digest as found, without running it.
    let doc = fx.run(&["config", "--json"]).doc();
    assert_eq!(doc["schema"], "fabrewk.config/3");
    let found = bin(&fx).join("fabrewk-hello").canonicalize().unwrap();
    let listed = json!([
        {"name": "absent", "path": null, "digest": null},
        {"name": "hello", "path": found, "digest": hello},
        {"name": "status", "path": bin(&fx).join("fabrewk-status").canonicalize().unwrap(), "digest": doc["extensions"][2]["digest"]},
    ]);
    assert_eq!(doc["extensions"], listed);
    let text = fx.run(&["config"]).stdout;
    assert!(text.contains(&format!("  hello: {}, digest {hello}\n", found.display())), "{text}");
    assert!(text.contains(&format!("  {}\n", say(LANG, "config.extension-absent", &[("name", "absent")]))), "{text}");
}

#[test]
fn req_ext_2_an_extension_sees_the_four_variables_and_reads_the_status() {
    let fx = fixture("ext-2", "\n[extensions.probe]\n");
    install(
        &fx,
        "probe",
        "env | grep '^FABREWK_' | sort > \"$HOME/env\"; pwd > \"$HOME/pwd\"; \"$FABREWK_BIN\" status --json > \"$HOME/status\"",
    );
    let ran = fx.run(&["probe"]);
    assert_eq!(ran.status, 0, "{ran:?}");
    let home = area(&fx, "probe").join("home");
    let bin = PathBuf::from(env!("CARGO_BIN_EXE_fabrewk")).canonicalize().unwrap();
    let root = fx.root.canonicalize().unwrap();
    assert_eq!(
        fs::read_to_string(home.join("env")).unwrap(),
        format!(
            "FABREWK_BIN={}\nFABREWK_EXTENSION=probe\nFABREWK_PROTOCOL=1\nFABREWK_ROOT={}\n",
            bin.display(),
            root.display()
        )
    );
    assert_eq!(fs::read_to_string(home.join("pwd")).unwrap(), format!("{}\n", root.display()));
    let status: Value = serde_json::from_slice(&fs::read(home.join("status")).unwrap()).unwrap();
    assert_eq!(status["schema"], "fabrewk.status/3", "{status}");
    // The same record, read from inside; only what `local/` holds is hidden there.
    let mut outside = fx.run(&["status", "--json"]).doc();
    outside["local"] = json!({"evidence": 0, "tmp": 0});
    assert_eq!(status, outside);
}

#[test]
fn req_ext_3_requests_are_applied_by_the_outer_core_with_the_extension_as_actor() {
    let fx = fixture("ext-3", "\n[extensions.index]\n");
    let base = fx.git(&["rev-parse", "HEAD"]);
    let script = format!(
        "{}\n{}\nprintf 'to the person \\033[2J\\n' >&2",
        line(&note("Indexed 3 files.")),
        line(&proposal(&base))
    );
    let digest = install(&fx, "index", &script);
    let ran = fx.run(&["index"]);
    assert_eq!(ran.status, 0, "{ran:?}");
    whole(&ran.all());
    // Its standard error reaches the person, its hidden characters escaped.
    assert!(ran.stderr.contains("to the person ") && !ran.stderr.contains('\u{1b}'), "{ran:?}");
    assert!(ran.stdout.contains(&say(LANG, "extension.applied", &[("name", "index"), ("digest", &digest)])));
    assert!(ran.stdout.contains(&say(LANG, "extension.noted", &[("name", "index")])), "{ran:?}");
    let actor = json!({"kind": "extension", "name": "index", "digest": digest});
    let notes = fx.events("ext-index");
    assert_eq!(notes.len(), 1);
    assert_eq!((&notes[0]["kind"], &notes[0]["actor"]), (&json!("extension-note"), &actor));
    assert_eq!(notes[0]["payload"], json!({"text": "Indexed 3 files."}));
    let stages: Vec<_> =
        streams(&fx).into_iter().filter(|(s, _)| s.starts_with("stage-") || s.starts_with("task-")).collect();
    assert_eq!(stages.len(), 2, "{stages:?}");
    for (s, _) in &stages {
        let e = &fx.events(s)[0];
        assert_eq!(e["actor"], actor, "{e}");
        assert!(e.get("say").is_none());
        assert!(ran.stdout.contains(&s[s.find('-').unwrap() + 1..]), "{ran:?}");
    }
    // None carries a person's signature, and the record replays.
    let sigs = walk(&fx.domain.join("data/events")).into_iter().filter(|p| p.extension().is_some_and(|e| e == "sig"));
    let signed: Vec<PathBuf> = sigs.filter(|p| !p.starts_with(fx.domain.join("data/events/project"))).collect();
    assert!(signed.is_empty(), "{signed:?}");
    let check = fx.run(&["check", "--json"]);
    assert_eq!(check.status, 0, "{check:?}");
    // The backend's pass is kept as verified by use, as a write command keeps it.
    let cache: Value = serde_json::from_slice(&fs::read(fx.domain.join("local/cache/detected.json")).unwrap()).unwrap();
    for p in ["bwrap", "pasta"] {
        assert!(cache["programs"][p]["verified"].is_string(), "{cache}");
    }
    // Inside an extension a person's move refuses (AD-5 point 3), and its cell cannot write the record anyway.
    install(
        &fx,
        "index",
        "\"$FABREWK_BIN\" authorize t-00000000 --seen x --say words > \"$HOME/move\" 2>&1; echo $? >> \"$HOME/move\"",
    );
    let before = streams(&fx);
    assert_eq!(fx.run(&["index"]).status, 0);
    let moved = fs::read_to_string(area(&fx, "index").join("home/move")).unwrap();
    assert!(moved.contains("AGENT_ENVIRONMENT") && moved.ends_with("1\n"), "{moved}");
    assert_eq!(streams(&fx), before);
}

fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for e in fs::read_dir(dir).unwrap().flatten() {
        if e.path().is_dir() {
            out.extend(walk(&e.path()));
        } else {
            out.push(e.path());
        }
    }
    out
}

#[test]
fn refusal_extension_output_invalid() {
    let fx = fixture("ext-invalid", "\n[extensions.bad]\n");
    let ok = line(&note("fine"));
    let cases = [
        (format!("{ok}\necho 'not json'"), "line 2: not one JSON object"),
        (format!("{ok}\nprintf '\\377\\n'"), "line 0: the output is not UTF-8"),
        (line(&json!({"protocol": 2, "request": "note", "text": "x"})), "line 1: protocol is not 1"),
        (line(&json!({"protocol": 1, "request": "approve", "text": "x"})), "line 1: request is not note or propose"),
        (line(&json!({"protocol": 1, "request": "note", "text": "x", "more": 1})), "line 1: a request with other"),
        (line(&json!({"protocol": 1, "request": "note", "text": " "})), "line 1: a note's text is not a text"),
        ("echo '{\"protocol\":1,\"request\":\"note\",\"text\":\"a\",\"text\":\"b\"}'".into(), "line 1: not one JSON"),
        (format!("{ok}\necho\n{ok}"), "line 2: not one JSON object"),
        (format!("echo '[]'\n{ok}"), "line 1: not one JSON object"),
    ];
    for (script, detail) in cases {
        install(&fx, "bad", &script);
        let ran = refused(&fx, &["bad"], "EXTENSION_OUTPUT_INVALID", 1);
        assert!(ran.stderr.contains(detail), "{detail}: {ran:?}");
        assert!(ran.stderr.contains(".brewstillery/local/evidence/ext-bad-"), "{ran:?}");
        // The detail names the line, never its bytes.
        assert!(!ran.stderr.contains("not json\n") && !ran.stderr.contains("approve"), "{ran:?}");
    }
    // A request whose content its own move refuses keeps that move's code, with nothing applied.
    let mut bad = proposal(&fx.git(&["rev-parse", "HEAD"]));
    bad["tasks"][0]["requirements"] = json!([]);
    install(&fx, "bad", &format!("{ok}\n{}", line(&bad)));
    let ran = refused(&fx, &["bad"], "TASK_INVALID", 1);
    assert!(ran.stderr.contains("line 2: "), "{ran:?}");
    assert!(fx.events("ext-bad").is_empty());
}

#[test]
fn req_ext_4_an_extension_reads_the_repository_and_writes_only_its_area() {
    let fx = fixture("ext-4", "\n[extensions.probe]\n");
    // An SSH agent's socket and the private folder, both outside what the cell shows.
    let sock = fx.root.parent().unwrap().join("agent.sock");
    let _listener = std::os::unix::net::UnixListener::bind(&sock).unwrap();
    fs::create_dir_all(fx.folder()).unwrap();
    fs::set_permissions(fx.folder(), fs::Permissions::from_mode(0o700)).unwrap();
    fs::create_dir_all(fx.domain.join("local/cache")).unwrap();
    let (root, own) = (fx.root.display().to_string(), area(&fx, "probe").display().to_string());
    let tries = [
        format!("{root}/new"),
        format!("{root}/.brewstillery/data/new"),
        format!("{root}/.brewstillery/local/new"),
        format!("{root}/.brewstillery/local/cache/new"),
        format!("{root}/.brewstillery/local/evidence/new"),
        format!("{own}/new"),
        "/home/agent/new".into(),
        "/tmp/new".into(),
        "/new".into(),
        "/usr/new".into(),
    ];
    let mut script = String::from("o=\"$HOME/result\"; : > \"$o\"\n");
    script += "head -1 \"$FABREWK_ROOT/README.md\" >> \"$o\"\n";
    script += "ls \"$FABREWK_ROOT/.brewstillery/data/events\" | grep -c . >> \"$o\"\n";
    script += "\"$FABREWK_BIN\" status --json > /dev/null && echo status >> \"$o\"\n";
    for t in &tries {
        script +=
            &format!("(echo x > '{t}') 2>/dev/null && echo 'wrote {t}' >> \"$o\" || echo 'refused {t}' >> \"$o\"\n");
    }
    for (what, p) in [("folder", fx.folder()), ("socket", sock.clone()), ("home", fx.home.clone())] {
        script +=
            &format!("test -e '{}' && echo '{what} seen' >> \"$o\" || echo '{what} unseen' >> \"$o\"\n", p.display());
    }
    script += "echo \"agent ${SSH_AUTH_SOCK:-unset}\" >> \"$o\"\n";
    script += "python3 -c 'import socket; socket.create_connection((\"1.1.1.1\", 53), 3)' 2>/dev/null && echo 'network' >> \"$o\" || echo 'no network' >> \"$o\"\n";
    install(&fx, "probe", &script);
    let ran = fx.run_env(&["probe"], &[("SSH_AUTH_SOCK", sock.to_str().unwrap())]);
    assert_eq!(ran.status, 0, "{ran:?}");
    let result = fs::read_to_string(area(&fx, "probe").join("home/result")).unwrap();
    let events = fs::read_dir(fx.domain.join("data/events")).unwrap().count();
    let mut expected = format!("fixture\n{events}\nstatus\n");
    for t in &tries {
        let writable = t.starts_with(&own) || t.starts_with("/home/agent") || t.starts_with("/tmp");
        expected += &format!("{} {t}\n", if writable { "wrote" } else { "refused" });
    }
    expected += "folder unseen\nsocket unseen\nhome unseen\nagent unset\nno network\n";
    assert_eq!(result, expected);
    // What it wrote is in its area alone: its home and `/tmp` are directories inside it.
    for t in ["new", "home/new", "tmp/new"] {
        assert!(area(&fx, "probe").join(t).is_file(), "{t}");
    }
    for t in ["new", ".brewstillery/data/new", ".brewstillery/local/new", ".brewstillery/local/cache/new"] {
        assert!(!fx.root.join(t).exists(), "{t}");
    }
}

#[test]
fn s_9_an_extension_whose_interpreter_lies_outside_its_cell_fails_saying_so() {
    let fx = fixture("ext-s9", "\n[extensions.py]\n");
    // An interpreter installed beside the extension, outside the repository and the system directories.
    let interp = bin(&fx).join("interp");
    fs::copy("/bin/sh", &interp).unwrap();
    install_with(&fx, "py", &format!("#!{}", interp.display()), "touch \"$HOME/ran\"");
    let ran = refused(&fx, &["py"], "TOOL_FAILED", 1);
    assert!(ran.stderr.contains("could not start: an interpreter or a support file"), "{ran:?}");
    assert!(ran.stderr.contains(".brewstillery/local/evidence/ext-py-"), "{ran:?}");
    assert!(!area(&fx, "py").join("home/ran").exists());
    // The same program with a system interpreter runs.
    install(&fx, "py", "touch \"$HOME/ran\"");
    assert_eq!(fx.run(&["py"]).status, 0);
    assert!(area(&fx, "py").join("home/ran").exists());
}

#[test]
fn req_ext_5_the_core_returns_once_the_extension_has_ended_and_gives_it_no_network() {
    let fx = fixture("ext-5", "\n[extensions.slow]\n\n[extensions.net]\nnetwork = true\n");
    install(&fx, "slow", "(sleep 2; touch \"$HOME/late\") &\nsleep 1; touch \"$HOME/done\"");
    let ran = fx.run(&["slow"]);
    assert_eq!(ran.status, 0, "{ran:?}");
    let home = area(&fx, "slow").join("home");
    assert!(home.join("done").exists());
    // What it left behind ended with it.
    std::thread::sleep(std::time::Duration::from_secs(3));
    assert!(!home.join("late").exists());
    // Network permitted: this backend cannot confine it without letting it nest, so it never runs (D-094).
    install(&fx, "net", "touch \"$HOME/ran\"");
    let ran = refused(&fx, &["net"], "CONFINEMENT_UNAVAILABLE", 1);
    assert!(ran.stderr.contains("the extension net has network"), "{ran:?}");
    assert!(!area(&fx, "net").exists());
}

#[test]
fn req_cli_3_an_extension_that_does_not_support_the_protocol_exits_10() {
    let fx = fixture("ext-10", "\n[extensions.old]\n");
    install(&fx, "old", &format!("{}\nexit 10", line(&note("never"))));
    let before = streams(&fx);
    let ran = fx.run(&["old"]);
    assert_eq!(ran.status, 10, "{ran:?}");
    assert_eq!(ran.stderr, say(LANG, "extension.unsupported", &[("name", "old")]) + "\n");
    assert_eq!(streams(&fx), before);
    // Another status applies nothing either, and names its evidence (D-094 point 3).
    install(&fx, "old", &format!("{}\nexit 3", line(&note("never"))));
    let ran = refused(&fx, &["old"], "TOOL_FAILED", 1);
    assert!(ran.stderr.contains("the extension old exited with status 3"), "{ran:?}");
}

#[test]
fn req_sec_2_with_no_backend_an_extension_refuses_and_never_runs() {
    let fx = fixture("ext-sec2", "\n[extensions.probe]\n");
    install(&fx, "probe", "touch \"$HOME/ran\"");
    let system = path_with(&fx.root.parent().unwrap().join("nobackend"), &["git", "ssh-keygen", "df"]);
    let path = std::env::join_paths([bin(&fx).as_os_str(), system.as_os_str()]).unwrap();
    let ran = fx.run_with(&["probe"], Some(&path));
    assert_eq!(ran.status, 1, "{ran:?}");
    assert!(ran.stderr.contains("CONFINEMENT_UNAVAILABLE") && ran.stderr.contains("bwrap"), "{ran:?}");
    assert!(!area(&fx, "probe").exists());
}

#[test]
fn an_extension_area_reached_through_a_link_is_refused_and_nothing_is_made_through_it() {
    let fx = fixture("ext-link", "\n[extensions.probe]\n");
    install(&fx, "probe", "touch \"$HOME/ran\"");
    let outside = fx.root.parent().unwrap().join("outside");
    fs::create_dir_all(&outside).unwrap();
    fs::create_dir_all(fx.domain.join("local")).unwrap();
    std::os::unix::fs::symlink(&outside, fx.domain.join("local/ext")).unwrap();
    let ran = refused(&fx, &["probe"], "TOOL_FAILED", 1);
    assert!(ran.stderr.contains("a link on the extension's area's path"), "{ran:?}");
    assert_eq!(fs::read_dir(&outside).unwrap().count(), 0);
}

/// P9H-1: a run of an extension, confined with write access to its area, cannot swap the area's home for a link that
/// another run then mounts: two runs of one extension never overlap (`LOCK_HELD`), and a link left behind is refused.
#[test]
fn p9h_1_no_run_of_an_extension_mounts_a_home_another_run_swapped() {
    let fx = fixture("ext-p9h1", "\n[extensions.probe]\n");
    let own = area(&fx, "probe");
    let marker = own.join("home/started");
    install(&fx, "probe", "touch \"$HOME/started\"; sleep 3");
    let mut first = fx.spawn(&["probe"]);
    let begun = std::time::Instant::now();
    while !marker.exists() {
        assert!(begun.elapsed() < std::time::Duration::from_secs(30), "the first run never started");
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    let ran = refused(&fx, &["probe"], "LOCK_HELD", 1);
    assert!(ran.stderr.contains("the extension probe runs in another command"), "{ran:?}");
    assert!(first.wait().unwrap().success());
    // A home swapped for a link, from inside, is refused at the next run, and nothing is written through it.
    let target = fx.root.parent().unwrap().join("target-dir");
    fs::create_dir_all(&target).unwrap();
    let script = format!("mv '{0}/home' '{0}/old' && ln -s '{1}' '{0}/home'", own.display(), target.display());
    install(&fx, "probe", &script);
    assert_eq!(fx.run(&["probe"]).status, 0);
    assert!(fs::symlink_metadata(own.join("home")).unwrap().file_type().is_symlink());
    install(&fx, "probe", "touch \"$HOME/ran\"");
    let ran = refused(&fx, &["probe"], "TOOL_FAILED", 1);
    assert!(ran.stderr.contains("a link on the extension's area's path"), "{ran:?}");
    assert_eq!(fs::read_dir(&target).unwrap().count(), 0);
}

/// P9H-3: another command may write while an extension runs. Its requests are planned on the record as it then is:
/// a proposal written meanwhile does not make its note a rollback, and a policy adopted meanwhile that no longer
/// permits it refuses them, `EXTENSION_DIGEST`.
#[test]
fn p9h_3_an_extensions_requests_are_planned_on_the_record_as_it_is_once_it_ends() {
    let fx = fixture_in("ext-p9h3", "published", "\n[extensions.wait]\n");
    let home = area(&fx, "wait").join("home");
    let script = format!(
        "touch \"$HOME/started\"; while [ ! -e \"$HOME/go\" ]; do sleep 0.1; done; rm \"$HOME/go\"\n{}",
        line(&note("after"))
    );
    install(&fx, "wait", &script);
    let wait_started = || {
        let begun = std::time::Instant::now();
        while !home.join("started").exists() {
            assert!(begun.elapsed() < std::time::Duration::from_secs(30), "the extension never started");
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        fs::remove_file(home.join("started")).unwrap();
    };
    let ext = fx.spawn(&["wait"]);
    wait_started();
    let mut plan = proposal(&fx.git(&["rev-parse", "HEAD"]));
    plan.as_object_mut().unwrap().retain(|k, _| k == "stage" || k == "tasks");
    fs::write(fx.root.join("plan.json"), serde_json::to_vec(&plan).unwrap()).unwrap();
    let proposed = fx.run(&["propose", "plan.json"]);
    assert_eq!(proposed.status, 0, "{proposed:?}");
    fs::write(home.join("go"), "").unwrap();
    let out = ext.wait_with_output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", String::from_utf8_lossy(&out.stderr));
    assert_eq!(fx.events("ext-wait").len(), 1);
    // A policy adopted meanwhile that no longer permits it: nothing is applied.
    let ext = fx.spawn(&["wait"]);
    wait_started();
    fx.edit_policy(&common::policy_text(&fx.keys));
    let adopted = fx.run(&["adopt", "--say", "Ya no la permito."]);
    assert_eq!(adopted.status, 0, "{adopted:?}");
    fs::write(home.join("go"), "").unwrap();
    let out = ext.wait_with_output().unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(1), "{stderr}");
    assert!(stderr.contains("EXTENSION_DIGEST") && stderr.contains("no longer permits the extension wait"), "{stderr}");
    assert_eq!(fx.events("ext-wait").len(), 1);
}
