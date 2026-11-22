//! The bootstrap's import through the built binary (REQ-INS-9; "The bootstrap's import and stage 0", points 2 to 7,
//! D-113; part 11a): F-4 over a copy of this repository's `bootstrap/` as committed, `BOOTSTRAP_CHANGED`'s cases, each
//! of point 2's reasons blocking by a fixture of its own, and `onboard` reaching the import without the tools question.

mod common;

use common::import::*;
use common::*;
use fabrewk_core::cli::say;
use fabrewk_core::record::sha256_hex;
use serde_json::{Value, json};

fn assess(fx: &Fx, options: &[String], more: &[&str]) -> Value {
    let all: Vec<&str> = options.iter().map(String::as_str).chain(more.iter().copied()).collect();
    let ran = fx.run(&[&["assess", "--json"][..], &all].concat());
    assert_eq!(ran.status, 0, "{ran:?}");
    ran.doc()
}

/// The findings `check` gives, by code and path from the root.
fn findings(fx: &Fx) -> Vec<(String, String, String)> {
    let doc = fx.run(&["check", "--json"]).doc();
    let all = doc["findings"].as_array().cloned().unwrap_or_default();
    let path = |f: &Value| {
        f["path"]
            .as_str()
            .map(|p| p.strip_prefix(fx.root.to_str().unwrap()).unwrap_or(p).trim_start_matches('/').to_owned())
    };
    all.iter()
        .map(|f| {
            (
                f["code"].as_str().unwrap_or_default().to_owned(),
                path(f).unwrap_or_default(),
                f["detail"].as_str().unwrap_or_default().to_owned(),
            )
        })
        .collect()
}

/// The entries' headings an independent reading finds: `## D-` or `## R-` lines outside fences of backticks.
fn headings(text: &str) -> Vec<String> {
    let mut fenced = false;
    let mut out = Vec::new();
    for line in text.lines() {
        if line.starts_with("```") {
            fenced = !fenced;
        } else if !fenced && (line.starts_with("## D-") || line.starts_with("## R-")) {
            out.push(line[3..].split(" — ").next().unwrap().to_owned());
        }
    }
    out
}

#[test]
fn f_4_the_bootstrap_s_import_records_every_entry_and_file_and_check_passes() {
    let (fx, options) = importable("f4", true);
    let decisions = std::fs::read_to_string(fx.domain.join("bootstrap/decisions.md")).unwrap();
    // Before the import, `bootstrap/` is an unknown entry of the domain, a warning (D-051).
    assert!(findings(&fx).iter().any(|f| f.0 == "DOMAIN_UNKNOWN_ENTRY" && f.1 == ".brewstillery/bootstrap"));
    let doc = assess(&fx, &options, &[]);
    let plan = &doc["plan"];
    assert_eq!(plan["verdict"], "adopt", "{}", plan["reasons"]);
    let keys: Vec<Value> = headings(&decisions).into_iter().map(Value::from).collect();
    let planned: Vec<Value> = plan["import"]["entries"].as_array().unwrap().iter().map(|e| e["key"].clone()).collect();
    assert_eq!(planned[0], Value::Null);
    assert_eq!(planned[1..], keys[..]);
    let files: Vec<&str> =
        plan["import"]["files"].as_array().unwrap().iter().map(|f| f["path"].as_str().unwrap()).collect();
    assert_eq!(files.len(), committed_bootstrap().len());
    let actions: Vec<(&str, &str)> = plan["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| (f["path"].as_str().unwrap(), f["action"].as_str().unwrap()))
        .collect();
    for kept in [
        ".brewstillery/settings.toml",
        ".brewstillery/xel/settings.toml",
        ".brewstillery/xel/fermento.md",
        ".brewstillery/ALAMBIQUE.md",
    ] {
        assert!(actions.contains(&(kept, "keep")), "{kept}: {actions:?}");
    }
    assert!(actions.contains(&(".brewstillery/STATUS.md", "replace")), "{actions:?}");
    let ran = fx.apply(&options, &[]);
    assert_eq!(ran.status, 0, "{ran:?}");
    // One `entry-imported` per entry, the opening first, whose texts joined are `decisions.md`'s bytes.
    let events = fx.events("bootstrap");
    let (last, entries) = events.split_last().unwrap();
    assert_eq!(
        entries[0]["payload"],
        json!({"key": null, "title": null, "fields": [], "text": entries[0]["payload"]["text"]})
    );
    let texts: String = entries.iter().map(|e| e["payload"]["text"].as_str().unwrap()).collect();
    assert_eq!(texts, decisions);
    assert!(entries.iter().all(|e| e["kind"] == "entry-imported" && e["actor"] == json!({"kind": "fabrewk"})));
    let found: Vec<Value> = entries[1..].iter().map(|e| e["payload"]["key"].clone()).collect();
    assert_eq!(found, keys);
    // An entry's fields, in its order; a fenced `## ` line stays inside the entry that holds it.
    let d113 = entries.iter().find(|e| e["payload"]["key"] == "D-113").unwrap();
    let labels: Vec<&str> =
        d113["payload"]["fields"].as_array().unwrap().iter().map(|f| f[0].as_str().unwrap()).collect();
    assert_eq!(labels, ["Status", "Decided", "Origin", "Decision", "Reason", "Consequences", "Sources", "Supersedes"]);
    assert_eq!(d113["payload"]["fields"][1], json!(["Decided", "2026-11-21"]));
    let r002 = entries.iter().find(|e| e["payload"]["key"] == "R-002").unwrap();
    assert!(r002["payload"]["text"].as_str().unwrap().contains("\n```"), "{r002}");
    assert_eq!(last["kind"], "bootstrap-imported");
    assert_eq!(last["payload"]["entries"], json!(entries.len()));
    assert_eq!(last["payload"]["files"], plan["import"]["files"]);
    // The first adoption's seal holds the stream at its last event.
    let kinds: Vec<Value> = fx.events("project").iter().map(|e| e["kind"].clone()).collect();
    assert_eq!(kinds, ["assessed", "publication-decided", "policy-adopted", "installed"]);
    let dir = fx.domain.join("data/events/bootstrap");
    let mut names: Vec<_> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path()).collect();
    names.sort();
    let head = json!({"digest": sha256_hex(&std::fs::read(names.last().unwrap()).unwrap()), "seq": events.len()});
    assert_eq!(fx.events("project")[2]["payload"]["heads"], json!({"bootstrap": head}));
    let ran = fx.run(&["check"]);
    assert_eq!(ran.status, 0, "{ran:?}");
    // The status view says the import, and the tree is clean.
    let line =
        say("en", "status.bootstrap", &[("entries", &entries.len().to_string()), ("files", &files.len().to_string())]);
    assert!(std::fs::read_to_string(fx.domain.join("STATUS.md")).unwrap().contains(&line));
    assert_eq!(fx.git(&["status", "--porcelain"]), "");
    // A second `apply` of the plan writes nothing.
    let again = fx.apply_digest(doc["digest"].as_str().unwrap(), &options, &[]);
    assert_eq!((again.status, again.doc()["applied"].clone()), (0, json!(false)), "{again:?}");
    assert_eq!(fx.events("bootstrap").len(), events.len());
    bootstrap_changed(&fx);
}

/// `BOOTSTRAP_CHANGED` (section 11.4): after the import, each change alone at `check`, then restored.
fn bootstrap_changed(fx: &Fx) {
    let b = fx.domain.join("bootstrap");
    let outside = fx.root.parent().unwrap().join("plan-copy.md");
    std::fs::copy(b.join("plan.md"), &outside).unwrap();
    let cases: [(&str, &str, &dyn Fn()); 5] = [
        ("decisions.md", "changed", &|| {
            let mut bytes = std::fs::read(b.join("decisions.md")).unwrap();
            bytes[0] ^= 1;
            std::fs::write(b.join("decisions.md"), bytes).unwrap();
        }),
        ("added.md", "added", &|| std::fs::write(b.join("added.md"), "x\n").unwrap()),
        ("plan.md", "removed", &|| std::fs::remove_file(b.join("plan.md")).unwrap()),
        ("empty", "added", &|| std::fs::create_dir(b.join("empty")).unwrap()),
        ("plan.md", "changed", &|| {
            std::fs::remove_file(b.join("plan.md")).unwrap();
            std::os::unix::fs::symlink(&outside, b.join("plan.md")).unwrap();
        }),
    ];
    for (path, how, change) in cases {
        change();
        let found = findings(fx);
        let changed: Vec<_> = found.iter().filter(|f| f.0 == "BOOTSTRAP_CHANGED").collect();
        assert_eq!(changed.len(), 1, "{path}: {found:?}");
        assert_eq!(changed[0].1, format!(".brewstillery/bootstrap/{path}"), "{found:?}");
        assert!(changed[0].2.starts_with(how), "{path}: {found:?}");
        assert_eq!(fx.run(&["check"]).status, 3);
        let _ = std::fs::remove_file(b.join(path));
        let _ = std::fs::remove_dir(b.join(path));
        fx.git(&["checkout", "-q", "--", ".brewstillery/bootstrap"]);
        assert_eq!(fx.run(&["check"]).status, 0, "{path} restored");
    }
}

#[test]
fn req_ins_9_a_fenced_heading_stays_in_its_entry_and_an_unknown_label_in_the_text_alone() {
    let (fx, options) = importable("mini", false);
    let doc = assess(&fx, &options, &[]);
    assert_eq!(doc["plan"]["verdict"], "adopt", "{}", doc["plan"]["reasons"]);
    let ran = fx.apply(&options, &[]);
    assert_eq!(ran.status, 0, "{ran:?}");
    let events = fx.events("bootstrap");
    assert_eq!(events.len(), 4);
    assert_eq!(events[0]["payload"]["text"], "# Decisions\n\nThe opening.\n\n");
    assert_eq!(
        events[1]["payload"]["fields"],
        json!([["Status", "decided"], ["Decided", "2026-11-21"], ["Decision", "one\n  and two"]])
    );
    assert!(events[2]["payload"]["text"].as_str().unwrap().ends_with("## D-009 — fenced\n- Status: fenced\n```\n"));
    assert_eq!(events[2]["payload"]["fields"], json!([["Status", "recorded"]]));
    assert_eq!(fx.run(&["check"]).status, 0);
}

/// `assess`'s reasons in a fixture changed by `change`, with `more` options: the verdict is `none` and `reason` blocks.
fn blocks(name: &str, reason: &str, more: &[&str], change: &dyn Fn(&Fx)) {
    let (fx, options) = importable(&format!("reason-{name}"), false);
    change(&fx);
    // An option `more` names again takes its place.
    let named: Vec<&str> = more.iter().copied().filter(|m| m.starts_with("--")).collect();
    let pairs: Vec<&[String]> = options.chunks(2).filter(|p| !named.contains(&p[0].as_str())).collect();
    let doc = assess(&fx, &pairs.concat(), more);
    let plan = &doc["plan"];
    assert_eq!(plan["verdict"], "none", "{name}: {}", plan["reasons"]);
    let found = plan["reasons"].as_array().unwrap().iter().any(|r| r["reason"] == reason && r["blocks"] == true);
    assert!(found, "{name}: {}", plan["reasons"]);
    assert!(plan["import"].is_object(), "{name}");
    assert_eq!(plan["files"], json!([]));
}

fn decisions(fx: &Fx, text: &str) {
    std::fs::write(fx.domain.join("bootstrap/decisions.md"), text).unwrap();
    fx.commit("decisions");
}

#[test]
fn req_ins_9_each_of_point_2_s_reasons_blocks_the_import() {
    use std::os::unix::fs::PermissionsExt;
    let b = |fx: &Fx| fx.domain.join("bootstrap");
    blocks("link", "import-link", &[], &|fx| {
        std::os::unix::fs::symlink("decisions.md", b(fx).join("l")).unwrap();
        fx.commit("link");
    });
    blocks("other", "import-other", &[], &|fx| _ = fx.program("mkfifo", &[b(fx).join("fifo").to_str().unwrap()]));
    blocks("large", "import-too-large", &[], &|fx| {
        std::fs::write(b(fx).join("large.md"), vec![b'a'; (1 << 20) + 1]).unwrap();
        fx.commit("large");
    });
    blocks("empty", "import-empty", &[], &|fx| std::fs::create_dir_all(b(fx).join("empty/deeper")).unwrap());
    blocks("absent", "decisions-absent", &[], &|fx| {
        std::fs::remove_file(b(fx).join("decisions.md")).unwrap();
        fx.commit("none");
    });
    blocks("not-utf8-bytes", "decisions-not-utf8", &[], &|fx| {
        std::fs::write(b(fx).join("decisions.md"), b"# D\n\xff\n").unwrap();
        fx.commit("bytes");
    });
    blocks("cr", "decisions-carriage-return", &[], &|fx| decisions(fx, "# D\r\n"));
    blocks("heading", "decisions-heading", &[], &|fx| decisions(fx, "# D\n\n## D-01 — Two digits\n\n- Status: x\n"));
    blocks("no-title", "decisions-heading", &[], &|fx| decisions(fx, "# D\n\n## D-001 — \n\n- Status: x\n"));
    blocks("no-line-end", "decisions-heading", &[], &|fx| decisions(fx, "# D\n\n## D-001 — Last"));
    blocks("order", "decisions-order", &[], &|fx| {
        decisions(fx, "# D\n\n## D-001 — One\n\n- Status: x\n\n## D-003 — Three\n\n- Status: x\n");
    });
    blocks("repeat", "decisions-order", &[], &|fx| {
        decisions(fx, "# D\n\n## D-001 — One\n\n- Status: x\n\n## D-001 — Again\n\n- Status: x\n");
    });
    blocks("status", "decisions-status", &[], &|fx| decisions(fx, "# D\n\n## D-001 — One\n\n- Decided: x\n"));
    blocks("twice", "decisions-twice", &[], &|fx| decisions(fx, "# D\n\n## D-001 — One\n\n- Status: x\n- Status: y\n"));
    blocks("publication", "import-publication", &["--publication", "own-repository"], &|_| {});
    blocks("tool", "import-tool", &["--tool", "claude"], &|_| {});
    blocks("second-maintainer", "import-maintainers", &[], &|fx| {
        let more = format!("\n[maintainers.ana]\nrole = \"contributor\"\nkey = \"{}\"\n", public(&fx.keys, "stranger"));
        let text = std::fs::read_to_string(fx.domain.join("settings.toml")).unwrap();
        std::fs::write(fx.domain.join("settings.toml"), text.replace("\n[languages]", &format!("{more}\n[languages]")))
            .unwrap();
        fx.commit("ana");
    });
    blocks("another-key", "import-maintainers", &["--key", "../keys/stranger.pub"], &|_| {});
    blocks("project-invalid", "import-project-invalid", &[], &|fx| {
        std::fs::write(fx.domain.join("settings.toml"), "[project]\nname = 1\n").unwrap();
        fx.commit("invalid");
    });
    // P11A-1: bytes that are not UTF-8 are no project file, never another text adopted in its place.
    blocks("project-not-utf8", "import-project-invalid", &[], &|fx| {
        let mut bytes = std::fs::read(fx.domain.join("settings.toml")).unwrap();
        bytes.extend(b"# \xff\n");
        std::fs::write(fx.domain.join("settings.toml"), bytes).unwrap();
        fx.commit("not utf-8");
    });
    blocks("domain-link", "import-link", &[], &|fx| {
        let elsewhere = fx.root.parent().unwrap().join("domain");
        std::fs::rename(&fx.domain, &elsewhere).unwrap();
        std::os::unix::fs::symlink(&elsewhere, &fx.domain).unwrap();
    });
    blocks("personal-invalid", "import-personal-invalid", &[], &|fx| {
        std::fs::write(fx.domain.join("xel/settings.toml"), "[maintainer]\nalias = \"ana\"\n").unwrap();
        fx.commit("mismatch");
    });
    blocks("separator", "import-separator", &[], &|fx| {
        std::fs::write(fx.domain.join("xel/fermento.md"), INPUT).unwrap();
        fx.commit("no separator");
    });
    blocks("alambique", "import-absent", &[], &|fx| {
        std::fs::remove_file(fx.domain.join("ALAMBIQUE.md")).unwrap();
        fx.commit("no alambique");
    });
    blocks("gitignore", "import-gitignore", &[], &|fx| {
        std::fs::write(fx.domain.join(".gitignore"), "local/\nother/\n").unwrap();
        fx.commit("gitignore");
    });
    blocks("directory", "import-directory", &[], &|fx| {
        std::fs::create_dir(fx.domain.join("ana")).unwrap();
        std::fs::write(fx.domain.join("ana/fermento.md"), fermento(INPUT)).unwrap();
        fx.commit("ana");
    });
    blocks("credential", "import-credential", &[], &|fx| {
        std::fs::write(b(fx).join("key.md"), "-----BEGIN OPENSSH PRIVATE KEY-----\n").unwrap();
        fx.commit("key");
    });
    // The committed state, compared raw (P11O-1, P11O-2, P11O-5).
    blocks("uncommitted", "uncommitted", &[], &|fx| {
        std::fs::write(fx.domain.join("ALAMBIQUE.md"), "# Changed\n").unwrap();
    });
    blocks("crlf", "uncommitted", &[], &|fx| {
        fx.git(&["config", "core.autocrlf", "true"]);
        std::fs::write(b(fx).join("plan.md"), "# Plan\r\n").unwrap();
    });
    blocks("executable", "uncommitted", &[], &|fx| {
        std::fs::set_permissions(fx.domain.join("settings.toml"), std::fs::Permissions::from_mode(0o755)).unwrap();
    });
    blocks("untracked", "uncommitted", &[], &|fx| std::fs::write(b(fx).join("new.md"), "x\n").unwrap());
    blocks("deleted", "uncommitted", &[], &|fx| std::fs::remove_file(b(fx).join("plan.md")).unwrap());
    blocks("assumed", "uncommitted", &[], &|fx| {
        fx.git(&["update-index", "--assume-unchanged", ".brewstillery/bootstrap/plan.md"]);
    });
    blocks("staged-only", "uncommitted", &[], &|fx| {
        std::fs::write(b(fx).join("plan.md"), "# Other\n").unwrap();
        fx.git(&["add", "--", ".brewstillery/bootstrap/plan.md"]);
        std::fs::write(b(fx).join("plan.md"), "# Plan\n").unwrap();
    });
    // P11A-4: a file `HEAD` holds under `bootstrap/` whose name is not UTF-8, gone from the working tree.
    blocks("not-utf8-in-head", "uncommitted", &[], &|fx| {
        use std::os::unix::ffi::OsStrExt;
        let name = b(fx).join(std::ffi::OsStr::from_bytes(b"hidden-\xff.md"));
        std::fs::write(&name, "x\n").unwrap();
        fx.commit("hidden");
        std::fs::remove_file(name).unwrap();
    });
    blocks("removed-from-index", "uncommitted", &[], &|fx| {
        fx.git(&["rm", "-q", "--cached", "--", ".brewstillery/bootstrap/plan.md"]);
        fx.git(&["commit", "-q", "-m", "untrack"]);
    });
}

#[test]
fn req_ins_9_an_unknown_entry_at_the_domain_s_top_is_shown_and_does_not_block() {
    let (fx, options) = importable("unknown", false);
    std::fs::write(fx.domain.join("notes.txt"), "x\n").unwrap();
    fx.commit("notes");
    let doc = assess(&fx, &options, &[]);
    assert_eq!(doc["plan"]["verdict"], "adopt", "{}", doc["plan"]["reasons"]);
    let shown = doc["plan"]["reasons"]
        .as_array()
        .unwrap()
        .iter()
        .any(|r| r["reason"] == "import-unknown" && r["blocks"] == false);
    assert!(shown, "{}", doc["plan"]["reasons"]);
}

#[test]
fn req_ins_9_a_repository_with_a_record_or_no_bootstrap_does_not_import() {
    let (fx, options) = importable("no-bootstrap", false);
    std::fs::remove_dir_all(fx.domain.join("bootstrap")).unwrap();
    fx.commit("none");
    assert!(assess(&fx, &options, &[])["plan"].get("import").is_none());
    let fx = Fx::applied("recorded", "published", None);
    std::fs::create_dir_all(fx.domain.join("bootstrap")).unwrap();
    std::fs::write(fx.domain.join("bootstrap/decisions.md"), MINI).unwrap();
    let key = fx.keys.join("xel.pub").to_str().unwrap().to_owned();
    let doc = assess(&fx, &["--alias".into(), "xel".into(), "--key".into(), key], &[]);
    assert!(doc["plan"].get("import").is_none(), "{doc}");
}

#[test]
fn req_ins_4_onboard_reaches_the_import_by_its_yes_without_asking_the_tools() {
    use std::io::Write;
    let (fx, _) = importable("onboard", false);
    let key = fx.keys.join("xel.pub");
    let typed: String = ["en", "adopt", "published", "xel", key.to_str().unwrap(), "yes", "I import it."]
        .iter()
        .map(|l| format!("{l}\n"))
        .collect();
    let mut c = std::process::Command::new("python3");
    isolate(&mut c, &fx.home, None);
    c.current_dir(&fx.root).args(["-I", "-c", DRIVE, env!("CARGO_BIN_EXE_fabrewk"), "onboard"]);
    let io = std::process::Stdio::piped;
    let mut child = c.stdin(io()).stdout(io()).stderr(io()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(typed.as_bytes()).unwrap();
    let out = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(out.status.code(), Some(0), "{stdout}");
    assert!(!stdout.contains(&say("en", "onboard.tools", &[])), "{stdout}");
    assert!(stdout.contains(&say("en", "assess.import", &[])), "{stdout}");
    assert_eq!(fx.events("bootstrap").len(), 4, "{stdout}");
    assert_eq!(fx.run(&["check"]).status, 0);
}

#[test]
fn req_ins_9_a_file_of_bootstrap_named_with_a_tab_is_imported_and_committed() {
    let (fx, options) = importable("tab", false);
    std::fs::write(fx.domain.join("bootstrap/a\tb.md"), "x\n").unwrap();
    fx.commit("tab");
    let doc = assess(&fx, &options, &[]);
    assert_eq!(doc["plan"]["verdict"], "adopt", "{}", doc["plan"]["reasons"]);
    let ran = fx.apply(&options, &[]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(fx.run(&["check"]).status, 0);
    assert_eq!(fx.git(&["status", "--porcelain"]), "");
}

#[test]
fn p11a_3_the_import_s_credential_reason_reads_only_what_assess_reads() {
    // `local/` is never an input: a key there does not block.
    let (fx, options) = importable("p11a-3-local", false);
    std::fs::create_dir_all(fx.domain.join("local/cache")).unwrap();
    std::fs::write(fx.domain.join("local/cache/probe.txt"), "-----BEGIN OPENSSH PRIVATE KEY-----\n").unwrap();
    let doc = assess(&fx, &options, &[]);
    assert_eq!(doc["plan"]["verdict"], "adopt", "{}", doc["plan"]["reasons"]);
    // Nothing beyond 1 MiB is read: a key past it is no reason, the size is.
    let (fx, options) = importable("p11a-3-large", false);
    let mut large = vec![b'a'; (1 << 20) + 1];
    large.extend(b"\n-----BEGIN OPENSSH PRIVATE KEY-----\n");
    std::fs::write(fx.domain.join("bootstrap/large.md"), large).unwrap();
    fx.commit("large");
    let reasons = assess(&fx, &options, &[])["plan"]["reasons"].clone();
    let keys: Vec<&str> = reasons.as_array().unwrap().iter().map(|r| r["reason"].as_str().unwrap()).collect();
    assert!(keys.contains(&"import-too-large") && !keys.contains(&"import-credential"), "{reasons}");
    // A name C9 takes for a credential's, in `bootstrap/`, blocks without its content read.
    let (fx, options) = importable("p11a-3-name", false);
    std::fs::write(fx.domain.join("bootstrap/id_ed25519"), "nothing\n").unwrap();
    fx.commit("name");
    let reasons = assess(&fx, &options, &[])["plan"]["reasons"].clone();
    assert!(reasons.as_array().unwrap().iter().any(|r| r["reason"] == "import-credential"), "{reasons}");
}
