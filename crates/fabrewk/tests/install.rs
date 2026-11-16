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
    assert_eq!(input("--key")["digest"], fabrewk_core::record::sha256_hex(std::fs::read(fx.key()).unwrap().as_slice()));
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
        assert!(doc["plan"]["inputs"].as_array().unwrap().iter().all(|i| i["path"] != "--key"));
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
