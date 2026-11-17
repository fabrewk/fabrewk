//! `assess` and its plan (part 10a; REQ-INS-1, REQ-INS-2, REQ-INS-8, D-097 points 2 to 5 and 9): what it reads and
//! never writes, confined or not; the side effects it reports; the three verdicts and their reasons; the plan's files
//! and its digest; the first alambique; the `apply` command it prints, for fish and for a POSIX shell.

mod common;

use common::*;
use serde_json::Value;
use std::path::Path;
use std::process::Command;

/// A repository with a README and a licence committed, a home, and the maestro `ana`'s key outside it; no domain.
fn product(name: &str) -> Fx {
    let base = tmp(name);
    let (root, home, keys) = (base.join("product"), base.join("home"), base.join("keys"));
    for d in [&root, &home, &keys] {
        std::fs::create_dir_all(d).unwrap();
    }
    let fx = Fx { domain: root.join(".brewstillery"), root, home, keys, path: None };
    let key = fx.keys.join("ana");
    fx.program("ssh-keygen", &["-q", "-t", "ed25519", "-N", "", "-C", "ana", "-f", key.to_str().unwrap()]);
    std::fs::write(fx.root.join("README.md"), "product\n").unwrap();
    std::fs::write(fx.root.join("LICENSE"), "MIT\n").unwrap();
    fx.git(&["init", "-q", "-b", "main"]);
    for (k, v) in [("user.name", "Fixture"), ("user.email", "fixture@example.invalid"), ("commit.gpgsign", "false")] {
        fx.git(&["config", k, v]);
    }
    fx.commit("product");
    fx
}

impl Fx {
    fn key(&self) -> String {
        self.keys.join("ana.pub").to_str().unwrap().to_owned()
    }

    /// `assess --json` with the complete answers and `more`.
    fn assess(&self, more: &[&str]) -> Value {
        let key = self.key();
        let mut args = [&["assess", "--json", "--alias", "ana", "--key", &key][..], more].concat();
        if !more.contains(&"--publication") {
            args.extend(["--publication", "published"]);
        }
        let out = self.run(&args);
        assert_eq!(out.status, 0, "{out:?}");
        out.doc()
    }
}

fn reasons(doc: &Value) -> Vec<(String, bool, String)> {
    let s = |v: &Value| v.as_str().unwrap_or_default().to_owned();
    doc["plan"]["reasons"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| (s(&r["reason"]), r["blocks"] == true, s(&r["path"])))
        .collect()
}

fn file<'a>(doc: &'a Value, path: &str) -> &'a Value {
    let files = doc["plan"]["files"].as_array().unwrap();
    files.iter().find(|f| f["path"] == path).unwrap_or_else(|| panic!("{path}: {doc}"))
}

fn content(doc: &Value, path: &str) -> String {
    file(doc, path)["content"].as_str().unwrap().to_owned()
}

#[test]
fn req_ins_1_assess_writes_nothing_and_a_read_only_cell_with_no_network_gives_the_same_plan() {
    let fx = product("ins-1-cell");
    std::fs::create_dir_all(fx.home.join(".claude")).unwrap();
    std::fs::write(fx.home.join(".claude/settings.json"), r#"{"autoUpdates": true}"#).unwrap();
    let key = fx.key();
    let args = ["assess", "--json", "--alias", "ana", "--key", &key, "--publication", "unpublished", "--tool", "codex"];
    let before = fx.snapshot();
    let free = fx.run(&args);
    assert_eq!(fx.snapshot(), before, "assess wrote");
    // The whole file system read-only, the home and the repository included, and no network.
    let mut c = Command::new("bwrap");
    isolate(&mut c, &fx.home, None);
    c.args([
        "--ro-bind",
        "/",
        "/",
        "--dev",
        "/dev",
        "--proc",
        "/proc",
        "--unshare-net",
        "--die-with-parent",
        "--chdir",
    ])
    .arg(&fx.root)
    .arg(env!("CARGO_BIN_EXE_fabrewk"))
    .args(args);
    let out = c.output().expect("bwrap is needed");
    assert_eq!(String::from_utf8(out.stderr).unwrap(), "");
    assert_eq!((out.status.code(), String::from_utf8(out.stdout).unwrap()), (Some(0), free.stdout.clone()));
    assert_eq!(fx.snapshot(), before);
    let doc = free.doc();
    assert_eq!(doc["schema"], "fabrewk.assessment/1");
    assert_eq!(doc["plan"]["verdict"], "adopt");
}

#[test]
fn req_ins_1_each_side_effect_is_reported_by_file_and_key_never_with_a_value() {
    let fx = product("ins-1-effects");
    let write = |base: &Path, rel: &str, text: &str| {
        std::fs::create_dir_all(base.join(rel).parent().unwrap()).unwrap();
        std::fs::write(base.join(rel), text).unwrap();
    };
    // A hook in `.claude/` settings, a sync that commits, pre-approvals a tool generated, MCP servers, a Codex setting,
    // automatic updates and telemetry in the home.
    write(
        &fx.root,
        ".claude/settings.json",
        r#"{"hooks": {"Stop": [{"hooks": [{"type": "command", "command": "git add -A; git commit -qm sync && git push"}]}]}}"#,
    );
    write(
        &fx.root,
        ".claude/settings.local.json",
        r#"{"permissions": {"allow": ["Bash(npm test:*)", "Bash(git push:*)", "WebFetch(domain:secret.example)"]}}"#,
    );
    write(&fx.root, ".mcp.json", r#"{"mcpServers": {"db": {"command": "x", "env": {"TOKEN": "tok-123"}}}}"#);
    write(&fx.root, ".codex/config.toml", "approval_policy = \"never\"\nsandbox_mode = \"danger-full-access\"\n");
    write(
        &fx.home,
        ".claude/settings.json",
        r#"{"autoUpdates": true, "env": {"CLAUDE_CODE_ENABLE_TELEMETRY": "1", "OTEL_EXPORTER_OTLP_ENDPOINT": "https://otel.secret.example"}}"#,
    );
    write(
        &fx.home,
        ".codex/config.toml",
        "check_for_update_on_startup = true\n[otel]\nexporter = \"otlp-http\"\n[mcp_servers.x]\ncommand = \"y\"\n",
    );
    fx.git(&["add", "-A"]);
    fx.commit("tools");
    let doc = fx.assess(&[]);
    let found: Vec<(String, String, String, Value)> = doc["plan"]["effects"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["file"].as_str().unwrap().into(),
                e["key"].as_str().unwrap().into(),
                e["kind"].as_str().unwrap().into(),
                e["count"].clone(),
            )
        })
        .collect();
    let n = |v: u64| Value::from(v);
    for want in [
        (".claude/settings.json", "hooks", "hooks", n(1)),
        (".claude/settings.json", "hooks", "commits", n(1)),
        (".claude/settings.local.json", "permissions.allow", "approvals", n(3)),
        (".claude/settings.local.json", "permissions.allow", "commits", n(1)),
        (".mcp.json", "mcpServers", "mcp", n(1)),
        (".codex/config.toml", "approval_policy", "approvals", Value::Null),
        (".codex/config.toml", "sandbox_mode", "sandbox", Value::Null),
        ("~/.claude/settings.json", "autoUpdates", "updates", Value::Null),
        ("~/.claude/settings.json", "env.CLAUDE_CODE_ENABLE_TELEMETRY", "telemetry", Value::Null),
        ("~/.claude/settings.json", "env.OTEL_EXPORTER_OTLP_ENDPOINT", "telemetry", Value::Null),
        ("~/.codex/config.toml", "check_for_update_on_startup", "updates", Value::Null),
        ("~/.codex/config.toml", "otel", "telemetry", Value::Null),
        ("~/.codex/config.toml", "mcp_servers", "mcp", n(1)),
    ] {
        let want = (want.0.to_owned(), want.1.to_owned(), want.2.to_owned(), want.3);
        assert!(found.contains(&want), "{want:?}: {found:?}");
    }
    assert_eq!(found.len(), 13, "{found:?}");
    for r in [".claude", ".codex"] {
        assert!(reasons(&doc).contains(&("harness".into(), false, r.into())), "{doc}");
    }
    assert!(reasons(&doc).iter().any(|r| r.0 == "effects"));
    assert_eq!(doc["plan"]["verdict"], "adopt", "side effects inform, never block");
    // No value, and no home path, reaches either output.
    let human = fx.run(&["assess", "--alias", "ana", "--key", &fx.key(), "--publication", "published"]);
    for out in [serde_json::to_string(&doc).unwrap(), human.stdout.clone()] {
        for secret in ["tok-123", "secret.example", "otlp-http", "danger-full", "npm test"] {
            assert!(!out.contains(secret), "{secret}");
        }
        assert!(!out.contains(fx.home.to_str().unwrap()), "{out}");
    }
    assert!(human.stdout.contains("~/.claude/settings.json autoUpdates: "), "{}", human.stdout);
    whole(&human.all());
}

#[test]
fn req_ins_2_the_plan_holds_every_item_and_its_identity_is_its_digest() {
    let fx = product("ins-2-plan");
    let key = std::fs::read_to_string(fx.key()).unwrap().trim().to_owned();
    let doc = fx.assess(&["--tool", "codex", "--tool", "claude", "--tool", "codex"]);
    let plan = &doc["plan"];
    // Its identity is the digest of its canonical bytes, and an unchanged repository gives it again.
    let bytes = fabrewk_core::record::encode(plan).unwrap();
    assert_eq!(doc["digest"], fabrewk_core::record::sha256_hex(&bytes));
    assert_eq!(fx.assess(&["--tool", "claude", "--tool", "codex"])["digest"], doc["digest"], "the tools are a set");
    assert_eq!(plan["schema"], "fabrewk.plan/1");
    assert_eq!(plan["verdict"], "adopt");
    assert_eq!(plan["incomplete"], serde_json::json!([]));
    assert_eq!(plan["answers"]["tool"], serde_json::json!(["claude", "codex"]));
    assert_eq!((plan["answers"]["mode"].as_str(), plan["answers"]["lang"].as_str()), (Some("adopt"), Some("en")));
    assert_eq!(plan["git"], serde_json::json!({"branch": "main", "signs": false}));
    // Every input read, by path, with its digest or its kind.
    let inputs = plan["inputs"].as_array().unwrap();
    let input = |p: &str| inputs.iter().find(|i| i["path"] == p).unwrap_or_else(|| panic!("{p}: {inputs:?}")).clone();
    assert_eq!(input("LICENSE")["digest"], fabrewk_core::record::sha256_hex(b"MIT\n"));
    assert_eq!(input("README.md")["kind"], "file");
    assert_eq!(input("AGENTS.md")["kind"], "absent");
    let key_input = inputs.iter().find(|i| i["option"] == "key").unwrap();
    assert_eq!(key_input["digest"], fabrewk_core::record::sha256_hex(std::fs::read(fx.key()).unwrap().as_slice()));
    assert!(
        !serde_json::to_string(plan).unwrap().contains(fx.root.to_str().unwrap()),
        "no path outside the repository"
    );
    // The files, each with its action and its exact content: the publication mode, the first maestro, the tools and
    // the local area are in them.
    let actions: Vec<(String, String)> = plan["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| (f["path"].as_str().unwrap().into(), f["action"].as_str().unwrap().into()))
        .collect();
    let want: Vec<(String, String)> = [
        (".brewstillery/.gitignore", "create"),
        (".brewstillery/ALAMBIQUE.md", "create"),
        (".brewstillery/RULES.md", "view"),
        (".brewstillery/STATUS.md", "view"),
        (".brewstillery/ana/fermento.md", "create"),
        (".brewstillery/ana/settings.toml", "create"),
        (".brewstillery/settings.toml", "create"),
        ("AGENTS.md", "block"),
        ("CLAUDE.md", "create"),
    ]
    .iter()
    .map(|(p, a)| (p.to_string(), a.to_string()))
    .collect();
    assert_eq!(actions, want);
    let project = content(&doc, ".brewstillery/settings.toml");
    let name = fx.root.file_name().unwrap().to_str().unwrap();
    assert_eq!(
        project,
        format!(
            "[project]\nname = \"{name}\"\n\n[maintainers.ana]\nrole = \"maestro\"\nkey = \"{key}\"\n\n[languages]\ncanonicalDocumentationLanguage = \"en\"\n\n[tools.claude]\nadapter = \"claude-code\"\nproviders = [\"anthropic\"]\n\n[tools.codex]\nadapter = \"codex\"\nproviders = [\"openai\"]\n\n[providers.anthropic]\nallowed = true\n\n[providers.openai]\nallowed = true\n"
        )
    );
    fabrewk_core::settings::parse(Path::new("settings.toml"), &project, fabrewk_core::settings::Level::Project)
        .unwrap();
    let personal = content(&doc, ".brewstillery/ana/settings.toml");
    assert_eq!(personal, "[maintainer]\nalias = \"ana\"\n\n[languages]\nagentInteraction = \"en\"\n");
    fabrewk_core::settings::parse(
        Path::new(".brewstillery/ana/settings.toml"),
        &personal,
        fabrewk_core::settings::Level::Personal,
    )
    .unwrap();
    assert_eq!(content(&doc, ".brewstillery/.gitignore"), "local/\n");
    assert_eq!(content(&doc, "CLAUDE.md"), "@AGENTS.md\n");
    // The fermento: the catalogue's empty input, then the separator block, which the finder reads with no point.
    let fermento = content(&doc, ".brewstillery/ana/fermento.md");
    let parts = fabrewk_core::fermento::split(&fermento, "ana").unwrap();
    assert!(fabrewk_core::fermento::points(&fermento[..parts.input_end]).is_empty());
    assert!(fermento.ends_with(&fabrewk_core::fermento::block("en", "ana")));
    let block = file(&doc, "AGENTS.md");
    assert_eq!(block["content"], block["lines"]);
    assert!(block["lines"].as_str().unwrap().starts_with("<!-- fabrewk:entry begin -->\n"), "{block}");
    // A root file changed between two assessments makes another plan.
    std::fs::write(fx.root.join("LICENSE"), "Apache-2.0\n").unwrap();
    assert_ne!(fx.assess(&["--tool", "claude", "--tool", "codex"])["digest"], doc["digest"]);
}

#[test]
fn req_ins_2_the_three_verdicts_are_reached() {
    // adopt and review.
    let fx = product("ins-2-verdicts");
    assert_eq!(fx.assess(&[])["plan"]["verdict"], "adopt");
    let review = fx.assess(&["--mode", "review"]);
    assert_eq!(review["plan"]["verdict"], "review");
    assert_eq!(review["plan"]["files"], serde_json::json!([]), "a plan for review holds no file");
    assert_eq!(review["apply"], serde_json::json!([]));
    // none, for each blocker.
    let none = |fx: &Fx, why: &str, path: &str| {
        let doc = fx.assess(&[]);
        assert_eq!(doc["plan"]["verdict"], "none", "{why}");
        assert!(reasons(&doc).contains(&(why.into(), true, path.into())), "{why}: {doc}");
        assert_eq!(
            (doc["plan"]["files"].clone(), doc["apply"].clone()),
            (serde_json::json!([]), serde_json::json!([]))
        );
        let human = fx.run(&["assess", "--alias", "ana", "--key", &fx.key(), "--publication", "published"]);
        assert!(human.stdout.contains(&fabrewk_core::cli::say("en", "assess.cannot-none", &[])), "{}", human.stdout);
        whole(&human.all());
    };
    let head = fx.git(&["rev-parse", "HEAD"]);
    fx.git(&["checkout", "-q", "--detach", &head]);
    none(&fx, "detached", "");
    fx.git(&["checkout", "-q", "main"]);
    std::fs::write(fx.root.join(".git/MERGE_HEAD"), format!("{head}\n")).unwrap();
    none(&fx, "operation", "");
    std::fs::remove_file(fx.root.join(".git/MERGE_HEAD")).unwrap();
    for (target, why) in [(".brewstillery/settings.toml", "present"), (".brewstillery/STATUS.md", "present")] {
        std::fs::create_dir_all(fx.domain.join("local")).unwrap();
        std::fs::write(fx.root.join(target), "x").unwrap();
        none(&fx, why, target);
        std::fs::remove_file(fx.root.join(target)).unwrap();
    }
    std::os::unix::fs::symlink("elsewhere", fx.domain.join("ana")).unwrap();
    none(&fx, "present", ".brewstillery/ana");
    std::fs::remove_file(fx.domain.join("ana")).unwrap();
    // A record: one event under `data/events/`.
    std::fs::create_dir_all(fx.domain.join("data/events/project")).unwrap();
    std::fs::write(fx.domain.join("data/events/project/000001"), "{}").unwrap();
    none(&fx, "record", "");
    std::fs::remove_dir_all(fx.domain.join("data")).unwrap();
    // The entry files: changes not committed, staged or not, untracked, not UTF-8, over 1 MiB, not a regular file.
    std::fs::write(fx.root.join("AGENTS.md"), "# Agents\n").unwrap();
    none(&fx, "uncommitted", "AGENTS.md");
    fx.git(&["add", "AGENTS.md"]);
    none(&fx, "uncommitted", "AGENTS.md");
    fx.commit("agents");
    assert_eq!(fx.assess(&[])["plan"]["verdict"], "adopt");
    std::fs::write(fx.root.join("AGENTS.md"), "# Agents, edited\n").unwrap();
    none(&fx, "uncommitted", "AGENTS.md");
    fx.git(&["checkout", "-q", "--", "AGENTS.md"]);
    for (bytes, why) in [(vec![0xff, b'\n'], "not-utf8"), (vec![b'a'; (1 << 20) + 1], "too-large")] {
        std::fs::write(fx.root.join("CLAUDE.md"), bytes).unwrap();
        fx.git(&["add", "CLAUDE.md"]);
        fx.commit(why);
        none(&fx, why, "CLAUDE.md");
    }
    fx.git(&["rm", "-q", "CLAUDE.md"]);
    std::os::unix::fs::symlink("AGENTS.md", fx.root.join("CLAUDE.md")).unwrap();
    fx.git(&["add", "CLAUDE.md"]);
    fx.commit("link");
    none(&fx, "not-a-file", "CLAUDE.md");
    fx.git(&["rm", "-q", "CLAUDE.md"]);
    fx.commit("unlinked");
    std::fs::write(fx.root.join("AGENTS.md"), "<!-- fabrewk:entry begin -->\nonly the start\n").unwrap();
    fx.git(&["add", "AGENTS.md"]);
    fx.commit("damaged");
    none(&fx, "block-damaged", "AGENTS.md");
}

#[test]
fn req_ins_2_what_informs_never_blocks_and_an_incomplete_plan_names_what_it_lacks() {
    let fx = product("ins-2-informs");
    std::fs::remove_file(fx.root.join("LICENSE")).unwrap();
    std::fs::write(fx.root.join("README.md"), "changed\n").unwrap();
    std::fs::write(fx.root.join("GEMINI.md"), "x\n").unwrap();
    let doc = fx.assess(&[]);
    assert_eq!(doc["plan"]["verdict"], "adopt");
    for r in [("no-licence", false, ""), ("changes", false, ""), ("harness", false, "GEMINI.md")] {
        assert!(reasons(&doc).contains(&(r.0.into(), r.1, r.2.into())), "{r:?}: {doc}");
    }
    // Without the publication, the alias and the key, the plan says what it lacks, and no `apply` command is printed.
    let out = fx.run(&["assess", "--json"]);
    let doc = out.doc();
    assert_eq!(doc["plan"]["incomplete"], serde_json::json!(["publication", "alias", "key"]));
    assert_eq!((doc["plan"]["verdict"].as_str(), doc["apply"].clone()), (Some("adopt"), serde_json::json!([])));
    let human = fx.run(&["assess"]);
    assert_eq!(human.status, 0);
    assert!(human.stdout.contains("--publication, --alias, --key"), "{}", human.stdout);
    assert!(human.stdout.contains(&fabrewk_core::cli::say("en", "assess.cannot-incomplete", &[])));
    assert!(!human.stdout.contains("fabrewk apply"));
    whole(&human.all());
    // A key file that is not one public key is a reason, and its content is never shown nor digested.
    let private = fx.keys.join("ana");
    let secret = std::fs::read_to_string(&private).unwrap();
    for (file, why) in
        [(private.clone(), "key-not-public"), (fx.keys.join("nope"), "key-absent"), (fx.keys.clone(), "key-not-a-file")]
    {
        let out = fx.run(&[
            "assess",
            "--json",
            "--alias",
            "ana",
            "--key",
            file.to_str().unwrap(),
            "--publication",
            "published",
        ]);
        let doc = out.doc();
        assert!(reasons(&doc).contains(&(why.into(), false, String::new())), "{why}: {doc}");
        assert!(doc["plan"]["inputs"].as_array().unwrap().iter().all(|i| i["option"] != "key"));
        assert_eq!(doc["apply"], serde_json::json!([]));
        assert!(!content(&doc, ".brewstillery/settings.toml").contains("key ="));
        let body = secret.lines().nth(1).unwrap();
        assert!(
            !out.stdout.contains(body) && !out.stdout.contains(&fabrewk_core::record::sha256_hex(secret.as_bytes()))
        );
    }
    std::fs::copy(fx.key(), fx.root.join("ana.pub")).unwrap();
    let doc = fx.run(&["assess", "--json", "--alias", "ana", "--key", "ana.pub", "--publication", "published"]).doc();
    assert!(reasons(&doc).contains(&("key-inside".into(), false, String::new())), "{doc}");
}

#[test]
fn req_ins_2_the_publication_modes_and_git_s_signing() {
    let fx = product("ins-2-modes");
    // `unpublished`: the exclude line and the warning in the plan's language.
    for lang in ["en", "es"] {
        let doc = fx.assess(&["--publication", "unpublished", "--lang", lang]);
        assert_eq!(file(&doc, ".git/info/exclude")["line"], "/.brewstillery/");
        assert_eq!(doc["plan"]["warning"]["text"], fabrewk_core::cli::say(lang, "warning.text", &[]));
        assert!(fabrewk_core::cli::say(lang, "warning.text", &[]).contains("own-repository"));
    }
    let key = fx.key();
    // `own-repository`: the exclude line, and the domain's own repository on the product's branch; no warning.
    let doc = fx.run(&["assess", "--json", "--alias", "ana", "--key", &key, "--publication", "own-repository"]).doc();
    assert_eq!(file(&doc, ".brewstillery/.git")["action"], "repository");
    assert_eq!(file(&doc, ".brewstillery/.git")["branch"], "main");
    assert_eq!(file(&doc, ".git/info/exclude")["present"], false);
    assert!(doc["plan"].get("warning").is_none());
    std::fs::write(fx.root.join(".git/info/exclude"), "*.log\n/.brewstillery/\n").unwrap();
    let doc = fx.run(&["assess", "--json", "--alias", "ana", "--key", &key, "--publication", "own-repository"]).doc();
    assert_eq!(file(&doc, ".git/info/exclude")["present"], true);
    // `published`: neither.
    let doc = fx.assess(&[]);
    assert!(
        doc["plan"]["files"]
            .as_array()
            .unwrap()
            .iter()
            .all(|f| f["action"] != "exclude" && f["action"] != "repository")
    );
    // Git does not sign with the key: the next steps show the commands, which assess never runs.
    let next = |doc: &Value| -> Vec<String> {
        doc["plan"]["next"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|n| n["command"].as_str().map(str::to_owned))
            .collect()
    };
    assert_eq!(next(&doc), ["git config gpg.format 'ssh'".to_owned(), format!("git config user.signingkey '{key}'")]);
    fx.git(&["config", "gpg.format", "ssh"]);
    fx.git(&["config", "user.signingkey", &key]);
    let doc = fx.assess(&[]);
    assert_eq!((next(&doc), doc["plan"]["git"]["signs"].clone()), (Vec::<String>::new(), Value::Bool(true)));
}

#[test]
fn req_ins_8_the_first_alambique_holds_the_answers_as_given_or_keeps_the_one_present() {
    let fx = product("ins-8");
    let name = fx.root.file_name().unwrap().to_str().unwrap().to_owned();
    for lang in ["en", "es"] {
        let t = |k: &str| fabrewk_core::cli::say(lang, &format!("skeleton.{k}"), &[]);
        let skeleton = |answers: [&str; 3]| {
            let mut out = format!("# {name}\n\n{}\n", t("opening").trim_end());
            for (n, (title, a)) in ["purpose", "audience", "excluded"].iter().zip(answers).enumerate() {
                out += &format!("\n## {}. {}\n\n{a}\n", n + 1, t(title));
            }
            out
        };
        let answers = ["A tool that *does* x.\n\nTwo paragraphs, `code` and {braces}.", "Me, and «you»", " "];
        let doc =
            fx.assess(&["--lang", lang, "--purpose", answers[0], "--audience", answers[1], "--excluded", answers[2]]);
        assert_eq!(content(&doc, ".brewstillery/ALAMBIQUE.md"), skeleton(answers), "{lang}");
        let doc = fx.assess(&["--lang", lang]);
        let empty = t("empty");
        assert_eq!(content(&doc, ".brewstillery/ALAMBIQUE.md"), skeleton([&empty, &empty, &empty]), "{lang}");
        // The opening explains the analogy; the sections' titles are stable and numbered for the alambique's units.
        for word in ["Brewstillery", "fermento", ".brewstillery/"] {
            assert!(t("opening").contains(word), "{lang}: {word}");
        }
        // Everything but the answers is in the plan's language.
        assert!(content(&doc, ".brewstillery/ana/fermento.md").starts_with(&fabrewk_core::cli::say(
            lang,
            "install.fermento",
            &[("alias", "ana")]
        )));
    }
    // An alambique present is kept, byte for byte, its digest in the plan.
    std::fs::create_dir_all(&fx.domain).unwrap();
    std::fs::write(fx.domain.join("ALAMBIQUE.md"), "# Ours\n").unwrap();
    let doc = fx.assess(&["--purpose", "ignored"]);
    let kept = file(&doc, ".brewstillery/ALAMBIQUE.md");
    assert_eq!(
        (kept["action"].as_str(), kept["digest"].as_str()),
        (Some("keep"), Some(fabrewk_core::record::sha256_hex(b"# Ours\n").as_str()))
    );
    assert!(kept.get("content").is_none());
    assert_eq!(doc["plan"]["verdict"], "adopt");
}

#[test]
fn req_ins_2_what_a_killed_apply_leaves_before_its_first_event_reads_as_absent() {
    let fx = product("ins-2-leftovers");
    let clean = fx.assess(&[])["digest"].clone();
    std::fs::create_dir_all(fx.domain.join("local")).unwrap();
    std::fs::write(fx.domain.join("local/lock"), "1\n").unwrap();
    std::fs::create_dir_all(fx.domain.join("data/events/project")).unwrap();
    std::fs::write(fx.domain.join("data/events/project/000001.sig"), "sig").unwrap();
    std::fs::write(fx.domain.join("data/events/project/.000001.tmp"), "{").unwrap();
    assert_eq!(fx.assess(&[])["digest"], clean);
    // Anything else in the domain is read.
    std::fs::write(fx.domain.join("notes.txt"), "x").unwrap();
    assert_ne!(fx.assess(&[])["digest"], clean);
}

#[test]
fn req_ins_2_an_existing_entry_file_keeps_everything_outside_the_block() {
    let fx = product("ins-2-block");
    let block = |lang: &str| {
        format!(
            "<!-- fabrewk:entry begin -->\n{}\n<!-- fabrewk:entry end -->\n",
            fabrewk_core::cli::say(lang, "install.entry", &[])
        )
    };
    std::fs::write(fx.root.join("AGENTS.md"), "# Agents\n\nRules.").unwrap();
    std::fs::write(fx.root.join("CLAUDE.md"), format!("@AGENTS.md\n\n{}tail\n", block("es"))).unwrap();
    fx.git(&["add", "-A"]);
    fx.commit("entries");
    let doc = fx.assess(&[]);
    assert_eq!(content(&doc, "AGENTS.md"), format!("# Agents\n\nRules.\n\n{}", block("en")));
    assert_eq!(file(&doc, "AGENTS.md")["replaced"], Value::Null);
    assert_eq!(content(&doc, "CLAUDE.md"), format!("@AGENTS.md\n\n{}tail\n", block("en")));
    assert_eq!(file(&doc, "CLAUDE.md")["replaced"], block("es"));
}

/// The arguments each shell gives `fabrewk` when it runs `command`.
fn argv(shell: &str, command: &str) -> Vec<String> {
    let script = match shell {
        "fish" => format!("function fabrewk; for a in $argv; printf '%s\\0' $a; end; end; {command}"),
        _ => format!("fabrewk() {{ for a; do printf '%s\\0' \"$a\"; done; }}; {command}"),
    };
    let flag = if shell == "fish" { "--no-config" } else { "-e" };
    let out = Command::new(shell).args([flag, "-c", &script]).output().unwrap_or_else(|_| panic!("{shell} is needed"));
    assert!(out.status.success(), "{shell}: {command}: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).unwrap().split('\0').filter(|a| !a.is_empty()).map(str::to_owned).collect()
}

#[test]
fn pd_38_the_apply_command_assess_prints_gives_the_same_options_in_fish_and_in_a_posix_shell() {
    let fx = product("ins-2-quoting");
    let key = fx.key();
    // Values a shell would split, expand or unquote; with a single quote or a backslash, one command for each shell.
    let cases: [(&[&str], usize); 3] = [
        (&["--tool", "claude", "--purpose", "A $HOME `x` * ; | & > < \"q\" {a,b} ~ # !", "--excluded=-flag-like"], 1),
        (&["--purpose", "it's", "--audience", "back\\slash \\' and '\\''"], 2),
        (&["--excluded", "line one\nline two", "--audience", ""], 1),
    ];
    for (more, n) in cases {
        let args =
            [&["assess", "--json", "--alias", "ana", "--key", &key, "--publication", "unpublished"][..], more].concat();
        let doc = fx.run(&args).doc();
        let commands = doc["apply"].as_array().unwrap();
        assert_eq!(commands.len(), n, "{commands:?}");
        let shells: Vec<&str> = commands.iter().map(|c| c["shell"].as_str().unwrap()).collect();
        assert_eq!(shells, if n == 1 { vec!["any"] } else { vec!["fish", "posix"] });
        for c in commands {
            let command = c["command"].as_str().unwrap();
            let run_in: &[&str] = match c["shell"].as_str().unwrap() {
                "fish" => &["fish"],
                "posix" => &["dash"],
                _ => &["fish", "dash"],
            };
            for shell in run_in {
                let words = argv(shell, command);
                assert_eq!(words[..2], ["apply".to_owned(), doc["digest"].as_str().unwrap().to_owned()], "{shell}");
                // The same options give the same plan: `apply`'s answers are `assess`'s.
                let parsed = fabrewk_core::cli::parse(words.iter().map(std::ffi::OsString::from));
                let Ok(fabrewk_core::cli::Parsed::Run { options, .. }) = parsed else { panic!("{shell}: {words:?}") };
                let again =
                    fabrewk_core::cli::parse(args.iter().filter(|a| **a != "--json").map(std::ffi::OsString::from));
                let Ok(fabrewk_core::cli::Parsed::Run { options: given, .. }) = again else { panic!() };
                let mut want = given.answers.clone();
                want.entry("mode").or_insert("adopt".into());
                want.entry("lang").or_insert("en".into());
                assert_eq!((options.answers, options.tools), (want, given.tools.clone()), "{shell}: {command}");
                assert_eq!(options.say.unwrap(), "<your words>");
            }
        }
        // The human output prints the same commands, each shell named when they differ.
        let human = fx.run(&args.iter().filter(|a| **a != "--json").copied().collect::<Vec<_>>());
        for c in commands {
            assert!(human.stdout.contains(&format!("  {}\n", c["command"].as_str().unwrap())), "{}", human.stdout);
        }
        assert_eq!(human.stdout.contains("In fish:"), n == 2);
    }
}

#[test]
fn refusal_not_a_repository() {
    let fx = product("ins-refusal-repo");
    for (dir, setup) in [("nowhere", None), ("sub", Some("sub")), ("bare.git", Some("bare"))] {
        let at = fx.root.parent().unwrap().join(dir);
        match setup {
            None => std::fs::create_dir_all(&at).unwrap(),
            Some("sub") => {
                let sub = fx.root.join("sub");
                std::fs::create_dir_all(&sub).unwrap();
                let moved = Fx {
                    root: sub.clone(),
                    domain: sub.join(".brewstillery"),
                    home: fx.home.clone(),
                    keys: fx.keys.clone(),
                    path: None,
                };
                refused(&moved, &["assess"], "NOT_A_REPOSITORY", 1, "en");
                continue;
            }
            Some(_) => {
                fx.program("git", &["init", "-q", "--bare", at.to_str().unwrap()]);
            }
        }
        let other = Fx {
            root: at.clone(),
            domain: at.join(".brewstillery"),
            home: fx.home.clone(),
            keys: fx.keys.clone(),
            path: None,
        };
        refused(&other, &["assess"], "NOT_A_REPOSITORY", 1, "en");
        refused(&other, &["assess", "--lang", "es"], "NOT_A_REPOSITORY", 1, "es");
    }
}

#[test]
fn refusal_alias_invalid() {
    let fx = product("ins-refusal-alias");
    for alias in ["Ana", "data", "skills", "review", "a--b", "x".repeat(33).as_str()] {
        refused(&fx, &["assess", "--alias", alias], "ALIAS_INVALID", 1, "en");
    }
}

#[test]
fn assess_s_usage_errors_name_what_it_takes() {
    let fx = product("ins-usage");
    let usage = |args: &[&str], key: &str, word: &str| {
        let before = fx.snapshot();
        let out = fx.run(args);
        assert_eq!(fx.snapshot(), before);
        assert_eq!(out.status, 2, "{out:?}");
        assert!(out.stderr.contains(&fabrewk_core::cli::say("en", key, &[("word", word)])), "{out:?}");
        whole(&out.all());
    };
    usage(&["assess", "--tool", "cursor"], "usage.tool", "cursor: claude, codex");
    usage(&["assess", "--lang", "fr"], "usage.lang", "fr: en, es");
    usage(&["assess", "--mode", "administer"], "usage.choice", "--mode: adopt, review");
    usage(
        &["assess", "--publication", "public"],
        "usage.choice",
        "--publication: published, own-repository, unpublished",
    );
    // `apply` is declared, takes `assess`'s options, and is not built in this part (D-099): it reads and writes nothing.
    usage(
        &["apply", "d", "--alias", "ana", "--key", "k", "--publication", "published", "--say", "w"],
        "usage.unbuilt",
        "apply",
    );
    usage(&["apply", "d", "--alias", "ana", "--key", "k"], "usage.needed", "--publication");
    // `es-MX` reaches the `es` catalogue.
    let out = fx.run(&["assess", "--lang", "es-MX"]);
    assert!(out.stdout.starts_with("Dictamen: "), "{out:?}");
}

#[test]
fn assess_reads_no_private_folder_and_follows_no_link_out_of_the_repository() {
    let fx = product("ins-links");
    let outside = fx.root.parent().unwrap().join("outside");
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(outside.join("settings.json"), r#"{"autoUpdates": true}"#).unwrap();
    std::os::unix::fs::symlink(&outside, fx.root.join(".claude")).unwrap();
    fx.git(&["add", ".claude"]);
    fx.commit("link");
    // A private folder that cannot be read is never opened.
    let private = fx.home.join(".config/fabrewk");
    std::fs::create_dir_all(&private).unwrap();
    std::fs::write(private.join("machine.toml"), "x").unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&private, std::fs::Permissions::from_mode(0o000)).unwrap();
    let doc = fx.assess(&[]);
    std::fs::set_permissions(&private, std::fs::Permissions::from_mode(0o700)).unwrap();
    assert_eq!(doc["plan"]["effects"], serde_json::json!([]), "{doc}");
    let inputs = doc["plan"]["inputs"].as_array().unwrap();
    assert!(inputs.iter().any(|i| i["path"] == ".claude/settings.json" && i["kind"] == "link"), "{inputs:?}");
    assert!(!serde_json::to_string(&doc).unwrap().contains("fabrewk/machine"));
    // In the home, the person's own, a link to a regular file is read: dotfiles are often links.
    std::fs::create_dir_all(fx.home.join(".codex")).unwrap();
    std::fs::write(outside.join("config.toml"), "approval_policy = \"never\"\n").unwrap();
    std::os::unix::fs::symlink(outside.join("config.toml"), fx.home.join(".codex/config.toml")).unwrap();
    let doc = fx.assess(&[]);
    let effects = doc["plan"]["effects"].as_array().unwrap();
    assert!(effects.iter().any(|e| e["file"] == "~/.codex/config.toml" && e["key"] == "approval_policy"), "{doc}");
    let inputs = doc["plan"]["inputs"].as_array().unwrap();
    assert!(
        inputs.iter().any(|i| i["path"] == "~/.codex/config.toml" && i["kind"] == "link" && i["digest"].is_string())
    );
}

#[test]
fn req_ins_2_a_repository_with_no_commit_yet_is_assessed() {
    let base = tmp("ins-2-unborn");
    let (root, home, keys) = (base.join("fresh"), base.join("home"), base.join("keys"));
    for d in [&root, &home, &keys] {
        std::fs::create_dir_all(d).unwrap();
    }
    let fx = Fx { domain: root.join(".brewstillery"), root, home, keys, path: None };
    fx.program(
        "ssh-keygen",
        &["-q", "-t", "ed25519", "-N", "", "-C", "ana", "-f", fx.keys.join("ana").to_str().unwrap()],
    );
    fx.git(&["init", "-q", "-b", "trunk"]);
    let before = fx.snapshot();
    let doc = fx.assess(&["--publication", "own-repository"]);
    assert_eq!(fx.snapshot(), before);
    assert_eq!(
        (doc["plan"]["verdict"].as_str(), doc["plan"]["git"]["branch"].as_str()),
        (Some("adopt"), Some("trunk"))
    );
    assert_eq!(file(&doc, ".brewstillery/.git")["branch"], "trunk");
}

#[test]
fn every_message_the_installer_names_is_in_both_catalogues() {
    const REASONS: [&str; 19] = [
        "detached",
        "operation",
        "record",
        "present",
        "not-a-file",
        "not-utf8",
        "too-large",
        "uncommitted",
        "block-damaged",
        "harness",
        "effects",
        "changes",
        "no-licence",
        "key-absent",
        "key-not-a-file",
        "key-too-large",
        "key-inside",
        "key-not-public",
        "settings-invalid",
    ];
    let mut named: Vec<String> = REASONS.iter().map(|r| format!("assess.reason.{r}")).collect();
    for kind in ["hooks", "approvals", "sandbox", "mcp", "updates", "telemetry", "commits", "unparsed"] {
        named.push(format!("assess.effect.{kind}"));
    }
    for action in ["create", "keep", "block", "exclude", "exclude-present", "repository", "view"] {
        named.push(format!("assess.action.{action}"));
    }
    for k in [
        "verdict",
        "verdict-adopt",
        "verdict-review",
        "verdict-none",
        "reasons",
        "no-reasons",
        "blocks",
        "incomplete",
        "effects",
        "files",
        "digest",
        "apply",
        "cannot-none",
        "cannot-review",
        "cannot-incomplete",
        "shell-fish",
        "shell-posix",
        "words",
    ] {
        named.push(format!("assess.{k}"));
    }
    // Every reason the code gives is listed, and every key it names whole is held.
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../install/src");
    for f in std::fs::read_dir(dir).unwrap() {
        let text = std::fs::read_to_string(f.unwrap().path()).unwrap();
        let code = text.split("#[cfg(test)]").next().unwrap().to_owned();
        for piece in code.split("reason(\"").skip(1) {
            let r = piece.split('"').next().unwrap();
            assert!(REASONS.contains(&r), "{r} is not listed");
        }
        for literal in code.split('"').skip(1).step_by(2) {
            let group = literal.split('.').next().unwrap();
            let whole = literal.contains('.') && !literal.contains([' ', '{', '/']);
            if ["assess", "install", "skeleton", "warning", "ui"].contains(&group) && whole {
                named.push(literal.to_owned());
            }
            assert!(!literal.starts_with("key-") || REASONS.contains(&literal), "{literal} is not listed");
        }
    }
    for k in ["opening", "purpose", "audience", "excluded", "empty"] {
        named.push(format!("skeleton.{k}"));
    }
    for (lang, text) in fabrewk_core::cli::CATALOGUES {
        let messages = fabrewk_core::cli::messages(text);
        for k in &named {
            assert!(messages.contains_key(k.as_str()), "{lang} lacks {k}");
        }
    }
}

/// Every file `args` opens, by any of its processes, as `strace` saw them.
fn traced(fx: &Fx, args: &[&str]) -> String {
    let trace = fx.root.parent().unwrap().join("trace");
    let mut c = Command::new("strace");
    isolate(&mut c, &fx.home, None);
    c.args(["-f", "-qq", "-e", "trace=execve,open,openat,openat2", "-o"])
        .arg(&trace)
        .arg(env!("CARGO_BIN_EXE_fabrewk"));
    let out = c.args(args).current_dir(&fx.root).output().expect("strace is needed");
    let text = std::fs::read_to_string(trace).unwrap_or_default();
    // Only a trace of the command really run proves what it did not open (P10A-20).
    let ran = format!("execve(\"{}\"", env!("CARGO_BIN_EXE_fabrewk"));
    assert!(
        text.contains(&ran) && text.contains("openat("),
        "strace saw nothing: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    text
}

#[test]
fn p10a_1_assess_runs_no_filter_git_would_run_on_a_changed_file() {
    let fx = product("p10a-1");
    std::fs::write(fx.root.join(".gitattributes"), "README.md filter=probe\nAGENTS.md filter=probe\n").unwrap();
    std::fs::write(fx.root.join("AGENTS.md"), "# Agents\n").unwrap();
    fx.commit("filtered");
    // The repository's own filter driver, as Git LFS's: a clean filter, a process filter, required.
    for (k, v) in [
        ("filter.probe.clean", "touch filter-ran; cat"),
        ("filter.probe.process", "touch process-ran"),
        ("filter.probe.required", "true"),
        ("filter.Other.Name.clean", "touch other-ran; cat"),
    ] {
        fx.git(&["config", k, v]);
    }
    // The same size, so that only the content tells the change, which Git reads through the filter.
    std::fs::write(fx.root.join("README.md"), "PRODUCT\n").unwrap();
    let before = fx.snapshot();
    let doc = fx.assess(&[]);
    assert_eq!(fx.snapshot(), before, "a filter ran");
    assert!(reasons(&doc).contains(&("changes".into(), false, String::new())), "{doc}");
    // An entry file through a filter blocks with its own reason, the filter never run (D-100).
    assert!(reasons(&doc).contains(&("filtered".into(), true, "AGENTS.md".into())), "{doc}");
}

#[test]
fn p10a_2_a_data_directory_that_is_a_link_is_not_walked() {
    let fx = product("p10a-2");
    let outside = fx.root.parent().unwrap().join("outside");
    std::fs::create_dir_all(outside.join("events/project")).unwrap();
    std::fs::write(outside.join("events/project/000001"), "{}").unwrap();
    std::fs::create_dir_all(&fx.domain).unwrap();
    std::os::unix::fs::symlink(&outside, fx.domain.join("data")).unwrap();
    let doc = fx.assess(&[]);
    assert!(!reasons(&doc).iter().any(|r| r.0 == "record"), "{doc}");
    let inputs = doc["plan"]["inputs"].as_array().unwrap();
    assert!(inputs.iter().any(|i| i["path"] == ".brewstillery/data" && i["kind"] == "link"), "{inputs:?}");
    // Nor is a link inside it an event.
    std::fs::remove_file(fx.domain.join("data")).unwrap();
    std::fs::create_dir_all(fx.domain.join("data/events/project")).unwrap();
    std::os::unix::fs::symlink(outside.join("events/project/000001"), fx.domain.join("data/events/project/000001"))
        .unwrap();
    assert!(!reasons(&fx.assess(&[])).iter().any(|r| r.0 == "record"));
}

#[test]
fn p10a_3_the_file_git_s_signing_key_names_is_never_opened() {
    let fx = product("p10a-3");
    let private = fx.home.join(".config/fabrewk");
    std::fs::create_dir_all(&private).unwrap();
    std::fs::write(private.join("secret"), "PRIVATE-MARKER\n").unwrap();
    use std::os::unix::fs::PermissionsExt;
    // A file its owner cannot read: opening it fails, which a test of the access time would only suggest.
    std::fs::set_permissions(private.join("secret"), std::fs::Permissions::from_mode(0o000)).unwrap();
    fx.git(&["config", "user.signingkey", private.join("secret").to_str().unwrap()]);
    fx.git(&["config", "gpg.format", "ssh"]);
    let trace = traced(&fx, &["assess", "--alias", "ana", "--key", &fx.key(), "--publication", "published"]);
    std::fs::set_permissions(private.join("secret"), std::fs::Permissions::from_mode(0o600)).unwrap();
    assert!(!trace.contains(".config/fabrewk"), "the private folder was opened");
    // Git signs with the key when its signing key names the `--key` file, or the private key beside it.
    fx.git(&["config", "user.signingkey", &fx.key()]);
    assert_eq!(fx.assess(&[])["plan"]["git"]["signs"], true);
    fx.git(&["config", "user.signingkey", fx.keys.join("ana").to_str().unwrap()]);
    assert_eq!(fx.assess(&[])["plan"]["git"]["signs"], true);
    let literal = format!("key::{}", std::fs::read_to_string(fx.key()).unwrap().trim());
    fx.git(&["config", "user.signingkey", &literal]);
    assert_eq!(fx.assess(&[])["plan"]["git"]["signs"], true);
    fx.git(&["config", "gpg.format", "openpgp"]);
    assert_eq!(fx.assess(&[])["plan"]["git"]["signs"], false);
}

#[test]
fn p10a_4_assess_reads_no_event() {
    let fx = product("p10a-4");
    std::fs::create_dir_all(fx.domain.join("data/events/project")).unwrap();
    std::fs::write(fx.domain.join("data/events/project/000001-0000000000000000.json"), "{}").unwrap();
    let trace = traced(&fx, &["assess", "--alias", "ana", "--key", &fx.key(), "--publication", "published"]);
    assert!(!trace.contains("000001-0000000000000000.json"), "an event was opened");
    assert!(reasons(&fx.assess(&[])).contains(&("record".into(), true, String::new())));
}

#[test]
fn p10a_5_a_tag_in_capitals_reaches_its_catalogue() {
    let fx = product("p10a-5");
    let doc = fx.assess(&["--lang", "ES-MX", "--publication", "unpublished"]);
    assert!(
        content(&doc, ".brewstillery/ALAMBIQUE.md")
            .contains(&fabrewk_core::cli::say("es", "skeleton.opening", &[]).trim_end().to_owned())
    );
    assert_eq!(doc["plan"]["warning"]["title"], fabrewk_core::cli::say("es", "warning.title", &[]));
    assert!(content(&doc, ".brewstillery/settings.toml").contains("canonicalDocumentationLanguage = \"ES-MX\""));
}

#[test]
fn p10a_7_an_incomplete_plan_lists_its_files_as_far_as_its_answers_go() {
    let fx = product("p10a-7");
    let doc = fx.run(&["assess", "--json"]).doc();
    assert_eq!(doc["apply"], serde_json::json!([]));
    assert!(!content(&doc, ".brewstillery/settings.toml").contains("[maintainers"));
    assert!(doc["plan"]["files"].as_array().unwrap().iter().all(|f| !f["path"].as_str().unwrap().contains("/ana/")));
}

#[test]
fn p10a_8_assess_fetches_no_missing_object_of_a_partial_clone() {
    let fx = product("p10a-8");
    fx.git(&["config", "uploadpack.allowFilter", "true"]);
    std::fs::write(fx.root.join("AGENTS.md"), "# Agents\n").unwrap();
    std::fs::write(fx.root.join("NOTES"), "line one\nline two\nline three\nline four\n").unwrap();
    fx.commit("agents");
    let clone = fx.root.parent().unwrap().join("partial");
    let url = format!("file://{}", fx.root.display());
    fx.program("git", &["clone", "-q", "--filter=blob:none", "--no-checkout", &url, clone.to_str().unwrap()]);
    let partial = Fx {
        domain: clone.join(".brewstillery"),
        root: clone.clone(),
        home: fx.home.clone(),
        keys: fx.keys.clone(),
        path: None,
    };
    partial.git(&["config", "remote.origin.uploadpack", "touch lazy-fetch-ran; git-upload-pack"]);
    // The index from `HEAD`'s trees alone, so that no blob of `HEAD` is local and the entry file is absent; then a
    // staged rename, which a rename detection could tell only from the missing blob (Git fetched it, P10A-8).
    partial.git(&["read-tree", "HEAD"]);
    std::fs::write(clone.join("NOTES2"), "line one\nline two\nline three\nline 4\n").unwrap();
    partial.git(&["add", "NOTES2"]);
    partial.git(&["rm", "-q", "--cached", "NOTES"]);
    let before = partial.snapshot();
    let doc = partial.assess(&[]);
    assert_eq!(partial.snapshot(), before, "a fetch wrote");
    assert!(!clone.join("lazy-fetch-ran").exists());
    assert!(reasons(&doc).contains(&("uncommitted".into(), true, "AGENTS.md".into())), "{doc}");
    assert!(reasons(&doc).contains(&("changes".into(), false, String::new())), "{doc}");
}

#[test]
fn p10a_10_no_tracked_file_over_1_mib_is_read() {
    let fx = product("p10a-10");
    let big = vec![b'x'; (1 << 20) + 1];
    std::fs::write(fx.root.join("big.bin"), &big).unwrap();
    fx.commit("big");
    std::fs::write(fx.root.join("big.bin"), vec![b'y'; big.len()]).unwrap();
    let trace = traced(&fx, &["assess", "--alias", "ana", "--key", &fx.key(), "--publication", "published"]);
    assert!(!trace.contains("big.bin"), "a large file was opened");
    assert!(reasons(&fx.assess(&[])).contains(&("changes".into(), false, String::new())));
}

#[test]
fn p10a_11_an_entry_file_through_a_filter_blocks_with_its_own_reason_and_line_ends_are_converted() {
    let fx = product("p10a-11");
    std::fs::write(fx.root.join(".gitattributes"), "AGENTS.md filter=probe\nCLAUDE.md text eol=crlf\n").unwrap();
    fx.git(&["config", "filter.probe.clean", "tr A-Z a-z"]);
    std::fs::write(fx.root.join("AGENTS.md"), "UPPER\n").unwrap();
    std::fs::write(fx.root.join("CLAUDE.md"), "@AGENTS.md\n").unwrap();
    fx.commit("filtered");
    // The checkout writes CLAUDE.md with CRLF; its index entry holds LF: no change.
    std::fs::remove_file(fx.root.join("CLAUDE.md")).unwrap();
    fx.git(&["checkout", "--", "CLAUDE.md"]);
    assert_eq!(std::fs::read(fx.root.join("CLAUDE.md")).unwrap(), b"@AGENTS.md\r\n");
    let doc = fx.assess(&[]);
    let found = reasons(&doc);
    assert!(found.contains(&("filtered".into(), true, "AGENTS.md".into())), "{doc}");
    assert!(!found.iter().any(|r| r.2 == "CLAUDE.md"), "{doc}");
}

#[test]
fn p10a_12_no_index_flag_hides_an_edit_of_an_entry_file() {
    let fx = product("p10a-12");
    std::fs::write(fx.root.join("AGENTS.md"), "original\n").unwrap();
    fx.commit("agents");
    for flag in ["--assume-unchanged", "--skip-worktree"] {
        fx.git(&["update-index", flag, "AGENTS.md"]);
        std::fs::write(fx.root.join("AGENTS.md"), "uncommitted\n").unwrap();
        assert!(reasons(&fx.assess(&[])).contains(&("uncommitted".into(), true, "AGENTS.md".into())), "{flag}");
        std::fs::write(fx.root.join("AGENTS.md"), "original\n").unwrap();
        assert_eq!(fx.assess(&[])["plan"]["verdict"], "adopt", "{flag}");
        fx.git(&["update-index", &flag.replace("--", "--no-"), "AGENTS.md"]);
    }
}

#[test]
fn p10a_17_a_mode_change_of_an_entry_file_blocks() {
    let fx = product("p10a-17");
    std::fs::write(fx.root.join("AGENTS.md"), "# Agents\n").unwrap();
    fx.commit("agents");
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(fx.root.join("AGENTS.md"), std::fs::Permissions::from_mode(0o755)).unwrap();
    assert!(reasons(&fx.assess(&[])).contains(&("uncommitted".into(), true, "AGENTS.md".into())), "unstaged");
    fx.git(&["add", "AGENTS.md"]);
    assert!(reasons(&fx.assess(&[])).contains(&("uncommitted".into(), true, "AGENTS.md".into())), "staged");
    fx.git(&["reset", "-q", "--", "AGENTS.md"]);
    // When Git ignores the executable bit, so does assess.
    fx.git(&["config", "core.fileMode", "false"]);
    assert_eq!(fx.assess(&[])["plan"]["verdict"], "adopt");
}

#[test]
fn p10a_19_a_root_file_over_1_mib_is_reported_by_its_size() {
    let fx = product("p10a-19");
    std::fs::write(fx.root.join("big.bin"), vec![b'x'; (1 << 20) + 1]).unwrap();
    let doc = fx.assess(&[]);
    let inputs = doc["plan"]["inputs"].as_array().unwrap();
    assert!(inputs.iter().any(|i| i["path"] == "big.bin" && i["bytes"] == (1 << 20) + 1), "{inputs:?}");
    assert!(inputs.iter().all(|i| i["path"] != "README.md" || i.get("bytes").is_none()));
}

/// A private folder holding `secret`, a valid Git configuration.
fn secret(fx: &Fx) -> String {
    let private = fx.home.join(".config/fabrewk");
    std::fs::create_dir_all(&private).unwrap();
    std::fs::write(private.join("secret"), "[user]\n\tname = PRIVATE\n").unwrap();
    private.join("secret").to_str().unwrap().to_owned()
}

/// `assess` with complete answers and `env`, under `strace`: its exit status, and whether it opened `secret`; `strace`
/// names the path opened, so a link to `secret` is named by `needles` too.
fn opened(fx: &Fx, env: &[(&str, &str)], secret: &str) -> (i32, bool) {
    opened_any(fx, env, &[secret])
}

fn opened_any(fx: &Fx, env: &[(&str, &str)], needles: &[&str]) -> (i32, bool) {
    let trace = fx.root.parent().unwrap().join("trace-env");
    let mut c = Command::new("strace");
    isolate(&mut c, &fx.home, None);
    c.args(["-f", "-qq", "-e", "trace=execve,open,openat,openat2", "-o"])
        .arg(&trace)
        .arg(env!("CARGO_BIN_EXE_fabrewk"));
    c.args(["assess", "--alias", "ana", "--key", &fx.key(), "--publication", "published"]).envs(env.iter().copied());
    let out = c.current_dir(&fx.root).output().expect("strace is needed");
    let text = std::fs::read_to_string(trace).unwrap_or_default();
    assert!(text.contains(&format!("execve(\"{}\"", env!("CARGO_BIN_EXE_fabrewk"))), "strace saw nothing");
    (out.status.code().unwrap_or(-1), needles.iter().any(|n| text.contains(n)))
}

#[test]
fn p10a_9_the_repository_s_own_include_of_the_private_folder_stops_assess_before_any_other_git() {
    let fx = product("p10a-9");
    let secret = secret(&fx);
    fx.git(&["config", "include.path", &secret]);
    assert_eq!(opened(&fx, &[], &secret), (1, false));
    refused(&fx, &["assess"], "PRIVATE_FOLDER_INCLUDED", 1, "en");
    fx.git(&["config", "--unset", "include.path"]);
    fx.git(&["config", "includeIf.onbranch:main.path", "../../home/.config/fabrewk/secret"]);
    refused(&fx, &["assess"], "PRIVATE_FOLDER_INCLUDED", 1, "en");
    // A repository whose configuration is a link to a file of the private folder.
    fx.git(&["config", "--unset", "includeIf.onbranch:main.path"]);
    let config = fx.root.join(".git/config");
    std::fs::copy(&config, &secret).unwrap();
    std::fs::remove_file(&config).unwrap();
    std::os::unix::fs::symlink(&secret, &config).unwrap();
    assert_eq!(opened_any(&fx, &[], &[&secret, "[\"git\""]), (1, false));
}

#[test]
fn p10a_14_p10a_22_to_p10a_27_the_person_s_git_configuration_makes_assess_neither_write_nor_open_private_files() {
    let fx = product("p10a-22");
    let secret = secret(&fx);
    let include = fx.root.parent().unwrap().join("includes-secret");
    std::fs::write(&include, format!("[include]\n\tpath = {secret}\n")).unwrap();
    let include = include.to_str().unwrap().to_owned();
    // The global configuration: an include of the private folder, a trace Git would write (P10A-22).
    let trace2 = fx.root.join(".git/assess-trace.json");
    std::fs::write(
        fx.home.join(".gitconfig"),
        format!("[include]\n\tpath = {secret}\n[trace2]\n\teventTarget = {}\n", trace2.display()),
    )
    .unwrap();
    // The default attributes file, a link to the private folder (P10A-27).
    std::fs::create_dir_all(fx.home.join(".config/git")).unwrap();
    std::os::unix::fs::symlink(&secret, fx.home.join(".config/git/attributes")).unwrap();
    let before = fx.snapshot();
    let parameters = format!("'Include.path={secret}'");
    let tricky = format!("'includeIf.onbranch:ma=in.path'='{secret}'");
    for env in [
        vec![],
        vec![("GIT_CONFIG_PARAMETERS", parameters.as_str())],
        vec![("GIT_CONFIG_PARAMETERS", tricky.as_str())],
        vec![
            ("GIT_CONFIG_COUNT", "1"),
            ("GIT_CONFIG_KEY_0", "include.path"),
            ("GIT_CONFIG_VALUE_0", "~/.config/fabrewk/secret"),
        ],
        vec![("GIT_CONFIG_COUNT", "1"), ("GIT_CONFIG_KEY_0", "core.attributesFile"), ("GIT_CONFIG_VALUE_0", &secret)],
        vec![("GIT_CONFIG_NOSYSTEM", "0"), ("GIT_CONFIG_SYSTEM", &include)],
        vec![("GIT_TRACE", trace2.to_str().unwrap()), ("GIT_TRACE2_EVENT", trace2.to_str().unwrap())],
    ] {
        assert_eq!(opened_any(&fx, &env, &[&secret, ".config/git/attributes"]), (0, false), "{env:?}");
        assert_eq!(fx.snapshot(), before, "{env:?} wrote");
    }
    // A global configuration that is itself a file of the private folder, absolute or relative (P10A-24, P10A-25).
    let relative = "../home/.config/fabrewk/secret".to_owned();
    for global in [secret.as_str(), relative.as_str()] {
        assert_eq!(opened(&fx, &[("GIT_CONFIG_GLOBAL", global)], &secret), (1, false), "{global}");
    }
    // The repository's attributes file, a file of the private folder, is not one Git of assess reads (P10A-15).
    fx.git(&["config", "core.attributesFile", &secret]);
    assert_eq!(opened(&fx, &[], &secret), (0, false));
}

#[test]
fn p10a_22_the_person_s_global_signing_settings_are_read_without_their_includes() {
    let fx = product("p10a-22-global");
    std::fs::write(fx.home.join(".gitconfig"), format!("[gpg]\n\tformat = ssh\n[user]\n\tsigningkey = {}\n", fx.key()))
        .unwrap();
    assert_eq!(fx.assess(&[])["plan"]["git"]["signs"], true);
    // The repository's own value wins.
    fx.git(&["config", "gpg.format", "openpgp"]);
    assert_eq!(fx.assess(&[])["plan"]["git"]["signs"], false);
}

#[test]
fn p10a_28_only_the_owner_s_executable_bit_is_a_mode() {
    let fx = product("p10a-28");
    std::fs::write(fx.root.join("AGENTS.md"), "# Agents\n").unwrap();
    fx.commit("agents");
    use std::os::unix::fs::PermissionsExt;
    for mode in [0o654, 0o645] {
        std::fs::set_permissions(fx.root.join("AGENTS.md"), std::fs::Permissions::from_mode(mode)).unwrap();
        assert_eq!(fx.assess(&[])["plan"]["verdict"], "adopt", "{mode:o}");
    }
}

#[test]
fn p10a_29_the_installation_s_commands_read_no_record_even_for_a_usage_error() {
    let fx = product("p10a-29");
    std::fs::create_dir_all(fx.domain.join("data/events/project")).unwrap();
    std::fs::write(fx.domain.join("data/events/project/000001-0000000000000000.json"), "{}").unwrap();
    for args in [
        &["apply", "d", "--alias", "ana", "--key", "k", "--publication", "published", "--say", "w"][..],
        &["assess", "--mode", "wrong"],
        &["--json", "assess", "--lang", "fr"],
    ] {
        let trace = traced(&fx, args);
        assert!(!trace.contains("000001-0000000000000000.json"), "{args:?} opened an event");
    }
}

#[test]
fn p10a_31_to_p10a_33_no_link_leads_assess_into_the_private_folder() {
    let fx = product("p10a-31");
    let secret = secret(&fx);
    std::fs::write(&secret, "approval_policy = \"never\"\n").unwrap();
    // A home's settings that are a link to the private folder are not read (P10A-31).
    std::fs::create_dir_all(fx.home.join(".codex")).unwrap();
    std::os::unix::fs::symlink(&secret, fx.home.join(".codex/config.toml")).unwrap();
    assert_eq!(opened_any(&fx, &[], &[&secret, ".codex/config.toml"]), (0, false));
    assert_eq!(fx.assess(&[])["plan"]["effects"], serde_json::json!([]));
    // The repository's attributes file, a link to the private folder (P10A-32).
    std::os::unix::fs::symlink(&secret, fx.root.join(".git/info/attributes")).unwrap();
    // Refused before any Git runs.
    assert_eq!(opened_any(&fx, &[], &[&secret, "info/attributes", "[\"git\""]), (1, false));
    refused(&fx, &["assess"], "PRIVATE_FOLDER_INCLUDED", 1, "en");
    std::fs::remove_file(fx.root.join(".git/info/attributes")).unwrap();
    // A `.git` that is a link to a pointer in the private folder (P10A-33).
    let metadata = fx.root.parent().unwrap().join("metadata");
    std::fs::rename(fx.root.join(".git"), &metadata).unwrap();
    std::fs::write(&secret, format!("gitdir: {}\n", metadata.display())).unwrap();
    std::os::unix::fs::symlink(&secret, fx.root.join(".git")).unwrap();
    assert_eq!(opened_any(&fx, &[], &[&secret, "[\"git\""]), (1, false));
}

#[test]
fn p10a_34_a_repository_s_empty_signing_key_wins_over_the_global_one() {
    let fx = product("p10a-34");
    std::fs::write(fx.home.join(".gitconfig"), format!("[gpg]\n\tformat = ssh\n[user]\n\tsigningkey = {}\n", fx.key()))
        .unwrap();
    assert_eq!(fx.assess(&[])["plan"]["git"]["signs"], true);
    fx.git(&["config", "user.signingkey", ""]);
    let doc = fx.assess(&[]);
    assert_eq!(doc["plan"]["git"]["signs"], false);
    let next: Vec<&str> =
        doc["plan"]["next"].as_array().unwrap().iter().filter_map(|n| n["command"].as_str()).collect();
    assert!(next.iter().any(|c| c.starts_with("git config user.signingkey")), "{next:?}");
}

#[test]
fn p10a_43_what_lies_inside_the_git_directory_is_the_person_s_own_state() {
    // A link inside the Git directory, which a clone cannot plant, is a declared residual (D-103): it neither blocks
    // nor is walked, a hook that no Git of assess runs among them.
    let fx = product("p10a-43");
    let secret = secret(&fx);
    std::os::unix::fs::symlink(&secret, fx.root.join(".git/hooks/pre-commit")).unwrap();
    assert_eq!(opened(&fx, &[], &secret), (0, false));
    assert_eq!(fx.assess(&[])["plan"]["verdict"], "adopt");
}

#[test]
fn p10a_42_no_file_of_the_private_folder_is_read_even_a_regular_one() {
    let fx = product("p10a-42");
    // A home inside the private folder, its settings a regular file there.
    let home = fx.home.join(".config/fabrewk/home");
    std::fs::create_dir_all(home.join(".codex")).unwrap();
    std::fs::write(home.join(".codex/config.toml"), "approval_policy = \"never\"\n").unwrap();
    let key = fx.key();
    let args = ["assess", "--json", "--alias", "ana", "--key", &key, "--publication", "published"];
    let home_s = home.to_str().unwrap();
    let doc =
        fx.run_env(&args, &[("HOME", home_s), ("XDG_CONFIG_HOME", fx.home.join(".config").to_str().unwrap())]).doc();
    assert_eq!(doc["plan"]["effects"], serde_json::json!([]), "{doc}");
    let inputs = doc["plan"]["inputs"].as_array().unwrap();
    assert!(inputs.iter().any(|i| i["path"] == "~/.codex/config.toml" && i["kind"] == "private"), "{inputs:?}");
    // A key file there is no key assess reads.
    let private_key = fx.home.join(".config/fabrewk/ana.pub");
    std::fs::copy(fx.key(), &private_key).unwrap();
    let doc = fx
        .run(&[
            "assess",
            "--json",
            "--alias",
            "ana",
            "--key",
            private_key.to_str().unwrap(),
            "--publication",
            "published",
        ])
        .doc();
    assert!(reasons(&doc).contains(&("key-inside".into(), false, String::new())), "{doc}");
}

#[test]
fn p10a_36_p10a_37_a_filter_driver_by_any_name_blocks_and_only_a_file_there_is() {
    for driver in ["unspecified", "unset", "probe"] {
        let fx = product(&format!("p10a-36-{driver}"));
        std::fs::write(fx.root.join(".gitattributes"), format!("*.md filter={driver}\n")).unwrap();
        fx.commit("attributes");
        // No entry file yet: nothing to compare, nothing blocks (P10A-37).
        assert_eq!(fx.assess(&[])["plan"]["verdict"], "adopt", "{driver}");
        std::fs::write(fx.root.join("AGENTS.md"), "a\n").unwrap();
        fx.commit("agents");
        assert!(reasons(&fx.assess(&[])).contains(&("filtered".into(), true, "AGENTS.md".into())), "{driver}");
    }
}

#[test]
fn p10a_38_p10a_44_a_truncated_key_is_not_a_public_key() {
    let fx = product("p10a-38");
    let file = fx.keys.join("truncated.pub");
    for line in ["ssh-ed25519 AAAAC3NzaC1lZDI1NTE5 invalid", "ssh-rsa AAAAB3NzaC1yc2EAAAABAQ== invalid"] {
        std::fs::write(&file, format!("{line}\n")).unwrap();
        let args =
            ["assess", "--json", "--alias", "ana", "--key", file.to_str().unwrap(), "--publication", "published"];
        let doc = fx.run(&args).doc();
        assert!(reasons(&doc).contains(&("key-not-public".into(), false, String::new())), "{line}: {doc}");
        assert_eq!(doc["apply"], serde_json::json!([]));
    }
}

#[test]
fn p10a_39_a_later_global_file_s_empty_value_wins() {
    let fx = product("p10a-39");
    // With no `GIT_CONFIG_GLOBAL` (the fixture's is emptied), Git reads the XDG file, then `~/.gitconfig`.
    let signs = || {
        let args = ["assess", "--json", "--alias", "ana", "--key", &fx.key(), "--publication", "published"];
        fx.run_env(&args, &[("GIT_CONFIG_GLOBAL", "")]).doc()["plan"]["git"]["signs"].clone()
    };
    std::fs::create_dir_all(fx.home.join(".config/git")).unwrap();
    let xdg = format!("[gpg]\n\tformat = ssh\n[user]\n\tsigningkey = {}\n", fx.key());
    std::fs::write(fx.home.join(".config/git/config"), xdg).unwrap();
    assert_eq!(signs(), true);
    std::fs::write(fx.home.join(".gitconfig"), "[user]\n\tsigningkey =\n").unwrap();
    assert_eq!(signs(), false);
}

#[test]
fn p10a_45_p10a_46_a_place_that_is_not_the_top_of_the_main_working_tree_is_refused_before_any_git() {
    let fx = product("p10a-45");
    let secret = secret(&fx);
    fx.git(&["config", "include.path", &secret]);
    // A subdirectory, a linked working tree: refused before Git would find the repository and its include.
    std::fs::create_dir_all(fx.root.join("sub")).unwrap();
    let linked = fx.root.parent().unwrap().join("linked");
    fx.git(&["worktree", "add", "-q", "--detach", linked.to_str().unwrap()]);
    for dir in [fx.root.join("sub"), linked] {
        let at = Fx {
            domain: dir.join(".brewstillery"),
            root: dir.clone(),
            home: fx.home.clone(),
            keys: fx.keys.clone(),
            path: None,
        };
        assert_eq!(opened_any(&at, &[], &[&secret, "[\"git\""]), (1, false), "{}", dir.display());
        refused(&at, &["assess"], "NOT_A_REPOSITORY", 1, "en");
    }
    fx.git(&["config", "--unset", "include.path"]);
    // A `core.worktree` that moves the working tree into the private folder (P10A-46).
    let elsewhere = fx.home.join(".config/fabrewk/worktree");
    std::fs::create_dir_all(&elsewhere).unwrap();
    std::fs::write(elsewhere.join(".gitattributes"), "AGENTS.md filter=private-marker\n").unwrap();
    fx.git(&["config", "core.worktree", elsewhere.to_str().unwrap()]);
    let trace = traced(&fx, &["assess", "--alias", "ana", "--key", &fx.key(), "--publication", "published"]);
    assert!(!trace.contains("worktree/.gitattributes"), "the private folder was opened");
    refused(&fx, &["assess"], "NOT_A_REPOSITORY", 1, "en");
}

#[test]
fn p10a_47_to_p10a_49_no_discovery_any_encoding_and_the_branch_s_whole_name() {
    let fx = product("p10a-47");
    let secret = secret(&fx);
    fx.git(&["config", "include.path", &secret]);
    // An empty `.git` in a subdirectory: Git is given its Git directory, so it finds no other (P10A-47).
    std::fs::create_dir_all(fx.root.join("sub/.git")).unwrap();
    let sub = fx.root.join("sub");
    let at = Fx {
        domain: sub.join(".brewstillery"),
        root: sub.clone(),
        home: fx.home.clone(),
        keys: fx.keys.clone(),
        path: None,
    };
    assert!(!opened_any(&at, &[], &[&secret]).1);
    assert_eq!(at.run(&["assess"]).status, 1);
    fx.git(&["config", "--unset", "include.path"]);
    // An include whose path is not UTF-8 (P10A-48).
    use std::os::unix::ffi::OsStrExt;
    let odd = std::path::PathBuf::from(std::ffi::OsStr::from_bytes(&[secret.as_bytes(), b"-\xff"].concat()));
    std::fs::copy(&secret, &odd).unwrap();
    let mut config = std::fs::read(fx.root.join(".git/config")).unwrap();
    config.extend_from_slice(&[b"[include]\n\tpath = ", odd.as_os_str().as_bytes(), b"\n"].concat());
    std::fs::write(fx.root.join(".git/config"), config).unwrap();
    refused(&fx, &["assess"], "PRIVATE_FOLDER_INCLUDED", 1, "en");
    fx.git(&["config", "--unset", "include.path"]);
    // A branch whose name starts as a ref does (P10A-49).
    fx.git(&["branch", "-m", "refs/heads/topic"]);
    let doc = fx.assess(&["--publication", "own-repository"]);
    assert_eq!(doc["plan"]["git"]["branch"], "refs/heads/topic");
    assert_eq!(file(&doc, ".brewstillery/.git")["branch"], "refs/heads/topic");
}

#[test]
fn p10a_50_p10a_51_a_bare_repository_is_refused_and_a_root_of_any_name_is_assessed() {
    let fx = product("p10a-50");
    fx.git(&["config", "core.bare", "true"]);
    refused(&fx, &["assess"], "NOT_A_REPOSITORY", 1, "en");
    fx.git(&["config", "core.bare", "false"]);
    assert_eq!(fx.assess(&[])["plan"]["verdict"], "adopt");
    // `git init --bare <root>/.git`.
    let other = product("p10a-50-nested");
    std::fs::remove_dir_all(other.root.join(".git")).unwrap();
    other.program("git", &["init", "-q", "--bare", other.root.join(".git").to_str().unwrap()]);
    refused(&other, &["assess"], "NOT_A_REPOSITORY", 1, "en");
    // A root whose name ends with a line break, or is not UTF-8 (P10A-51).
    use std::os::unix::ffi::OsStrExt;
    for name in [&b"product-\n"[..], b"product-\xff"] {
        let fx = product(&format!("p10a-51-{}", name.len()));
        let moved = fx.root.parent().unwrap().join(std::ffi::OsStr::from_bytes(name));
        std::fs::rename(&fx.root, &moved).unwrap();
        let at = Fx {
            domain: moved.join(".brewstillery"),
            root: moved.clone(),
            home: fx.home.clone(),
            keys: fx.keys.clone(),
            path: None,
        };
        assert_eq!(at.assess(&[])["plan"]["verdict"], "adopt", "{name:?}");
    }
}

#[test]
fn p10a_52_to_p10a_55_operations_worktree_configuration_and_an_empty_git_directory() {
    use std::os::unix::ffi::OsStrExt;
    // An operation in progress in a root that is not UTF-8 (P10A-52), and a stopped sequence (P10A-54).
    let fx = product("p10a-52");
    let moved = fx.root.parent().unwrap().join(std::ffi::OsStr::from_bytes(b"product-\xff"));
    std::fs::rename(&fx.root, &moved).unwrap();
    let at = Fx {
        domain: moved.join(".brewstillery"),
        root: moved.clone(),
        home: fx.home.clone(),
        keys: fx.keys.clone(),
        path: None,
    };
    for marker in ["MERGE_HEAD", "sequencer"] {
        let path = moved.join(".git").join(marker);
        if marker == "sequencer" {
            std::fs::create_dir_all(&path).unwrap()
        } else {
            std::fs::write(&path, "x\n").unwrap()
        }
        assert!(reasons(&at.assess(&[])).iter().any(|r| r.0 == "operation" && r.1), "{marker}");
        let _ = std::fs::remove_file(&path).or_else(|_| std::fs::remove_dir_all(&path));
    }
    assert_eq!(at.assess(&[])["plan"]["verdict"], "adopt");
    // A `config.worktree` not in force changes nothing (P10A-53).
    std::fs::write(moved.join(".git/config.worktree"), "[core]\n\tbare = true\n\tworktree = /elsewhere\n").unwrap();
    assert_eq!(at.assess(&[])["plan"]["verdict"], "adopt");
    at.git(&["config", "extensions.worktreeConfig", "true"]);
    refused(&at, &["assess"], "NOT_A_REPOSITORY", 1, "en");
    // An empty `.git` is no repository (P10A-55).
    let other = product("p10a-55");
    std::fs::remove_dir_all(other.root.join(".git")).unwrap();
    std::fs::create_dir(other.root.join(".git")).unwrap();
    refused(&other, &["assess"], "NOT_A_REPOSITORY", 1, "en");
}

#[test]
fn p10a_56_p10a_57_the_git_file_s_bytes_and_the_configuration_s_precedence() {
    // A Git directory named with a final space, beside one named without it (P10A-56).
    let fx = product("p10a-56");
    let spaced = fx.root.parent().unwrap().join("metadata ");
    std::fs::rename(fx.root.join(".git"), &spaced).unwrap();
    std::fs::write(fx.root.join(".git"), format!("gitdir: {}\n", spaced.display())).unwrap();
    std::fs::create_dir_all(fx.root.parent().unwrap().join("metadata")).unwrap();
    std::fs::write(spaced.join("MERGE_HEAD"), "x\n").unwrap();
    assert!(reasons(&fx.assess(&[])).iter().any(|r| r.0 == "operation"), "the named Git directory");
    std::fs::remove_file(spaced.join("MERGE_HEAD")).unwrap();
    assert_eq!(fx.assess(&[])["plan"]["verdict"], "adopt");
    // `config.worktree` in force overrides the main file (P10A-57).
    let fx = product("p10a-57");
    fx.git(&["config", "extensions.worktreeConfig", "true"]);
    fx.git(&["config", "core.bare", "true"]);
    std::fs::write(fx.root.join(".git/config.worktree"), "[core]\n\tbare = false\n").unwrap();
    assert_eq!(fx.assess(&[])["plan"]["verdict"], "adopt");
}

#[test]
fn p10a_58_p10a_59_the_exclude_file_is_the_git_directory_s_own() {
    let fx = product("p10a-58");
    // A Git directory apart from the working tree.
    let apart = fx.root.parent().unwrap().join("metadata");
    std::fs::rename(fx.root.join(".git"), &apart).unwrap();
    std::fs::write(fx.root.join(".git"), format!("gitdir: {}\n", apart.display())).unwrap();
    std::fs::write(apart.join("info/exclude"), "/.brewstillery/\n# first\n").unwrap();
    let doc = fx.assess(&["--publication", "unpublished"]);
    let shown = apart.join("info/exclude").display().to_string();
    assert_eq!(file(&doc, &shown)["present"], true, "{doc}");
    let input = doc["plan"]["inputs"].as_array().unwrap().iter().find(|i| i["path"] == shown).unwrap().clone();
    assert_eq!(input["kind"], "file");
    std::fs::write(apart.join("info/exclude"), "/.brewstillery/\n# second\n").unwrap();
    assert_ne!(fx.assess(&["--publication", "unpublished"])["digest"], doc["digest"]);
    // A root that is not UTF-8: the exclude file shown as the repository's own (P10A-59).
    use std::os::unix::ffi::OsStrExt;
    let fx = product("p10a-59");
    let moved = fx.root.parent().unwrap().join(std::ffi::OsStr::from_bytes(b"product-\xff"));
    std::fs::rename(&fx.root, &moved).unwrap();
    let at = Fx {
        domain: moved.join(".brewstillery"),
        root: moved.clone(),
        home: fx.home.clone(),
        keys: fx.keys.clone(),
        path: None,
    };
    std::fs::write(moved.join(".git/info/exclude"), "/.brewstillery/\n").unwrap();
    assert_eq!(file(&at.assess(&["--publication", "unpublished"]), ".git/info/exclude")["present"], true);
}

#[test]
fn p10a_60_a_root_entry_of_any_name_is_read_by_its_bytes() {
    use std::os::unix::ffi::OsStrExt;
    let fx = product("p10a-60");
    let at = |bytes: &[u8]| fx.root.join(std::ffi::OsStr::from_bytes(bytes));
    std::fs::write(at(b"untracked-\xfe"), "x").unwrap();
    let first = fx.assess(&[]);
    let input =
        |doc: &Value, path: &str| doc["plan"]["inputs"].as_array().unwrap().iter().find(|i| i["path"] == path).cloned();
    assert_eq!(input(&first, "untracked-\\xfe").unwrap()["kind"], "file", "{first}");
    std::fs::rename(at(b"untracked-\xfe"), at(b"untracked-\xff")).unwrap();
    let second = fx.assess(&[]);
    assert_ne!(second["digest"], first["digest"]);
    assert_eq!(input(&second, "untracked-\\xff").unwrap()["kind"], "file");
    // A licence named so is read.
    std::fs::remove_file(fx.root.join("LICENSE")).unwrap();
    std::fs::write(at(b"LICENSE-\xff"), "MIT\n").unwrap();
    let doc = fx.assess(&[]);
    assert!(input(&doc, "LICENSE-\\xff").unwrap()["digest"].is_string());
    assert!(!reasons(&doc).iter().any(|r| r.0 == "no-licence"));
}

#[test]
fn p10a_59_p10a_61_a_git_directory_of_any_name_and_a_root_entry_named_as_an_option() {
    use std::os::unix::ffi::OsStrExt;
    let fx = product("p10a-59-apart");
    let apart = fx.root.parent().unwrap().join(std::ffi::OsStr::from_bytes(b"metadata-\xff"));
    std::fs::rename(fx.root.join(".git"), &apart).unwrap();
    std::fs::write(fx.root.join(".git"), [&b"gitdir: "[..], apart.as_os_str().as_bytes(), b"\n"].concat()).unwrap();
    let doc = fx.assess(&["--publication", "unpublished"]);
    let exclude = doc["plan"]["files"].as_array().unwrap().iter().find(|f| f["action"] == "exclude").unwrap().clone();
    assert!(exclude["path"].as_str().unwrap().ends_with("metadata-\\xff/info/exclude"), "{exclude}");
    // A root entry named `--key` is one entry, the key another (P10A-61).
    let fx = product("p10a-61");
    std::fs::write(fx.root.join("--key"), "ordinary\n").unwrap();
    let first = fx.assess(&[]);
    std::fs::remove_file(fx.root.join("--key")).unwrap();
    std::fs::create_dir(fx.root.join("--key")).unwrap();
    let second = fx.assess(&[]);
    assert_ne!(first["digest"], second["digest"]);
    let entry = second["plan"]["inputs"].as_array().unwrap().iter().find(|i| i["path"] == "--key").cloned();
    assert_eq!(entry.unwrap()["kind"], "directory");
}

#[test]
fn p10a_62_p10a_63_a_branch_and_a_signing_key_of_any_name() {
    use std::os::unix::ffi::OsStrExt;
    // Two branches whose names differ only in a byte that is not UTF-8 make two plans (P10A-62).
    let fx = product("p10a-62");
    let rename = |name: &[u8]| {
        let status = Command::new("git")
            .args(["-C", fx.root.to_str().unwrap(), "branch", "-m"])
            .arg(std::ffi::OsStr::from_bytes(name))
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", fx.home.join(".gitconfig"))
            .status()
            .unwrap();
        assert!(status.success());
    };
    rename(b"topic-\xfe");
    let first = fx.assess(&["--publication", "own-repository"]);
    assert_eq!(first["plan"]["git"]["branch"], "topic-\\xfe");
    rename(b"topic-\xff");
    let second = fx.assess(&["--publication", "own-repository"]);
    assert_ne!(first["digest"], second["digest"]);
    assert_eq!(file(&second, ".brewstillery/.git")["branch"], "topic-\\xff");
    // A signing key whose path ends with a line break (P10A-63).
    let fx = product("p10a-63");
    let private = fx.keys.join("key\n");
    std::fs::rename(fx.keys.join("ana"), &private).unwrap();
    let public = fx.keys.join("key\n.pub");
    std::fs::rename(fx.keys.join("ana.pub"), &public).unwrap();
    fx.git(&["config", "gpg.format", "ssh"]);
    fx.git(&["config", "user.signingkey", private.to_str().unwrap()]);
    let args = ["assess", "--json", "--alias", "ana", "--key", public.to_str().unwrap(), "--publication", "published"];
    assert_eq!(fx.run(&args).doc()["plan"]["git"]["signs"], true);
}

#[test]
fn p10a_64_p10a_65_a_head_that_names_no_branch_and_a_project_of_any_name() {
    let fx = product("p10a-64");
    fx.git(&["update-ref", "refs/tags/release", "HEAD"]);
    fx.git(&["symbolic-ref", "HEAD", "refs/tags/release"]);
    let doc = fx.assess(&["--publication", "own-repository"]);
    assert!(reasons(&doc).contains(&("detached".into(), true, String::new())), "{doc}");
    assert_eq!(doc["apply"], serde_json::json!([]));
    // A project named by a directory whose name is not UTF-8 (P10A-65).
    use std::os::unix::ffi::OsStrExt;
    let fx = product("p10a-65");
    let moved = fx.root.parent().unwrap().join(std::ffi::OsStr::from_bytes(b"product-\xfe"));
    std::fs::rename(&fx.root, &moved).unwrap();
    let at = Fx {
        domain: moved.join(".brewstillery"),
        root: moved.clone(),
        home: fx.home.clone(),
        keys: fx.keys.clone(),
        path: None,
    };
    let doc = at.assess(&[]);
    assert!(content(&doc, ".brewstillery/ALAMBIQUE.md").starts_with("# product-\\xfe\n"));
}

#[test]
fn p10a_66_to_p10a_68_a_tilde_directory_an_unreadable_record_and_a_name_of_lines() {
    // A directory named `~` in the repository is no home: a link there is not read (P10A-66).
    let fx = product("p10a-66");
    let tilde = fx.root.join("~/metadata");
    std::fs::create_dir_all(fx.root.join("~")).unwrap();
    std::fs::rename(fx.root.join(".git"), &tilde).unwrap();
    std::fs::write(fx.root.join(".git"), "gitdir: ~/metadata\n").unwrap();
    let outside = fx.root.parent().unwrap().join("outside");
    std::fs::write(&outside, "EXTERNAL\n").unwrap();
    std::fs::remove_file(tilde.join("info/exclude")).unwrap();
    std::os::unix::fs::symlink(&outside, tilde.join("info/exclude")).unwrap();
    let doc = fx.assess(&["--publication", "unpublished"]);
    let input =
        doc["plan"]["inputs"].as_array().unwrap().iter().find(|i| i["path"] == "~/metadata/info/exclude").cloned();
    assert_eq!(input.unwrap().get("digest"), None, "{doc}");
    // A record that cannot be read is no absent one (P10A-67).
    let fx = product("p10a-67");
    std::fs::create_dir_all(fx.domain.join("data/events/project")).unwrap();
    std::fs::write(fx.domain.join("data/events/project/000001.json"), "{}").unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(fx.domain.join("data/events/project"), std::fs::Permissions::from_mode(0o000)).unwrap();
    let out = fx.run(&["assess", "--json", "--alias", "ana", "--key", &fx.key(), "--publication", "published"]);
    std::fs::set_permissions(fx.domain.join("data/events/project"), std::fs::Permissions::from_mode(0o755)).unwrap();
    assert_ne!(out.status, 0, "{out:?}");
    assert!(!out.stdout.contains("fabrewk apply"));
    // A directory's name of several lines is one title line, and the sections stay the alambique's (P10A-68).
    let fx = product("p10a-68");
    let moved = fx.root.parent().unwrap().join("product\n```\n## 1. Injected");
    std::fs::rename(&fx.root, &moved).unwrap();
    let at = Fx {
        domain: moved.join(".brewstillery"),
        root: moved.clone(),
        home: fx.home.clone(),
        keys: fx.keys.clone(),
        path: None,
    };
    let alambique = content(&at.assess(&[]), ".brewstillery/ALAMBIQUE.md");
    assert!(alambique.starts_with("# product\\u{a}```\\u{a}## 1. Injected\n"), "{alambique}");
    assert_eq!(alambique.matches("\n## 1. ").count(), 1);
}
