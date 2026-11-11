//! Fixtures for the command line's tests (core-specification 11.2 rules 2 and 4): a Git repository, SSH keys, a home
//! and a private folder of the fixture's own, all under Cargo's `CARGO_TARGET_TMPDIR`. Git runs only through
//! [`Fx::git`] and the binary only through [`Fx::run`], both with the fixture's home and Git configuration; the
//! record is written with the library without Git (until phase 10's `apply`, as rule 2 says), then committed.
#![allow(dead_code)]

use fabrewk_core::record::{Draft, KeySigner, Say, Session, nonce, sha256_hex, subject_stream};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn tmp(name: &str) -> PathBuf {
    sweep_ended("cli-");
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "cli-{name}-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// The environment every program of a fixture runs with: nothing of the machine's but `PATH`.
pub fn isolate(c: &mut Command, home: &Path, path: Option<&std::ffi::OsStr>) {
    c.env_clear()
        .env("PATH", path.map(|p| p.to_owned()).or_else(|| std::env::var_os("PATH")).unwrap_or_default())
        .env("HOME", home)
        .env("XDG_CONFIG_HOME", home.join(".config"))
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", home.join(".gitconfig"))
        // No maintenance in the background, which would write while a test compares the tree.
        .env("GIT_CONFIG_COUNT", "2")
        .env("GIT_CONFIG_KEY_0", "maintenance.auto")
        .env("GIT_CONFIG_VALUE_0", "false")
        .env("GIT_CONFIG_KEY_1", "gc.auto")
        .env("GIT_CONFIG_VALUE_1", "0")
        .env("LC_ALL", "C");
}

/// What the binary did.
#[derive(Debug)]
pub struct Ran {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

impl Ran {
    /// The `--json` document on standard output.
    pub fn doc(&self) -> Value {
        serde_json::from_str(&self.stdout).unwrap_or_else(|e| panic!("{e}: {self:?}"))
    }

    pub fn all(&self) -> String {
        format!("{}{}", self.stdout, self.stderr)
    }
}

/// A fixture: a repository holding the domain, the maestro `xel`'s key and a stranger's, and a home.
pub struct Fx {
    pub root: PathBuf,
    pub domain: PathBuf,
    pub home: PathBuf,
    pub keys: PathBuf,
}

pub fn public(keys: &Path, who: &str) -> String {
    std::fs::read_to_string(keys.join(format!("{who}.pub"))).unwrap().trim().to_owned()
}

/// The project file the fixtures adopt: `xel` as maestro, English as the canonical language.
pub fn policy_text(keys: &Path) -> String {
    format!(
        "[project]\nname = \"fixture\"\n\n[maintainers.xel]\nrole = \"maestro\"\nkey = \"{}\"\n\n[languages]\ncanonicalDocumentationLanguage = \"en\"\n",
        public(keys, "xel")
    )
}

/// The input of the fixtures' fermento: two points.
pub const INPUT: &str = "# Fermento\n\n## Proposals\n\n1. First point.\n\n2. Second point,\n   on two lines.\n\n";

/// A fermento made of `input` and the separator block of the catalogue's `en` (REQ-FER-1).
pub fn fermento(input: &str) -> String {
    format!("{input}{}", fabrewk_core::fermento::block("en", "xel"))
}

/// `xel`'s personal file: agents and messages in `es-MX`.
pub const PERSONAL: &str = "[maintainer]\nalias = \"xel\"\n\n[languages]\nagentInteraction = \"es-MX\"\n";

impl Fx {
    /// A repository with the domain, its ignore file, the project file and `xel`'s personal file, and Git's signing key
    /// set to `xel`'s, committed; in a `mode` other than `published` the product ignores the domain instead of
    /// committing it. No record yet.
    pub fn bare(name: &str, mode: &str) -> Fx {
        let base = tmp(name);
        let (root, home, keys) = (base.join("repo"), base.join("home"), base.join("keys"));
        let domain = root.join(".brewstillery");
        for d in [&domain.join("xel"), &home, &keys] {
            std::fs::create_dir_all(d).unwrap();
        }
        let fx = Fx { root, domain, home, keys };
        for alias in ["xel", "stranger"] {
            let key = fx.keys.join(alias);
            fx.program("ssh-keygen", &["-q", "-t", "ed25519", "-N", "", "-C", alias, "-f", key.to_str().unwrap()]);
        }
        std::fs::write(fx.domain.join(".gitignore"), "local/\n").unwrap();
        std::fs::write(fx.domain.join("settings.toml"), policy_text(&fx.keys)).unwrap();
        std::fs::write(fx.domain.join("xel/settings.toml"), PERSONAL).unwrap();
        std::fs::write(fx.domain.join("xel/fermento.md"), fermento(INPUT)).unwrap();
        std::fs::write(fx.root.join("README.md"), "fixture\n").unwrap();
        fx.git(&["init", "-q", "-b", "main"]);
        for (k, v) in [("user.name", "Fixture"), ("user.email", "fixture@example.invalid"), ("commit.gpgsign", "false")]
        {
            fx.git(&["config", k, v]);
        }
        fx.git(&["config", "user.signingkey", fx.keys.join("xel.pub").to_str().unwrap()]);
        if mode != "published" {
            std::fs::write(fx.root.join(".git/info/exclude"), "/.brewstillery/\n").unwrap();
        }
        fx.commit("fixture");
        fx
    }

    /// [`Fx::bare`] with the policy adopted and the publication mode decided as `mode`, all committed.
    pub fn new(name: &str, mode: &str) -> Fx {
        let fx = Fx::bare(name, mode);
        fx.adopt(mode);
        if mode == "published" {
            fx.commit("record");
        }
        fx
    }

    /// Runs `program` with the fixture's environment in its root; panics if it fails.
    pub fn program(&self, program: &str, args: &[&str]) -> String {
        let mut c = Command::new(program);
        isolate(&mut c, &self.home, None);
        let out = c.current_dir(if self.root.exists() { &self.root } else { &self.keys }).args(args).output().unwrap();
        assert!(out.status.success(), "{program} {args:?}: {}", String::from_utf8_lossy(&out.stderr));
        String::from_utf8(out.stdout).unwrap()
    }

    pub fn git(&self, args: &[&str]) -> String {
        self.program("git", args).trim_end().to_owned()
    }

    pub fn commit(&self, message: &str) {
        self.git(&["add", "-A"]);
        self.git(&["commit", "-q", "--allow-empty", "-m", message]);
    }

    pub fn signer(&self) -> KeySigner {
        KeySigner(self.keys.join("xel"))
    }

    fn draft(&self, kind: &str, policy: &str, payload: Value) -> Draft {
        let actor = json!({"kind": "person", "alias": "xel", "role": "maestro"});
        let say = Some(Say { text: "Lo decido.".into(), lang: "es-MX".into() });
        Draft { kind: kind.into(), actor, policy: policy.into(), instance: "SHA256:fixture".into(), say, payload }
    }

    /// Writes the project events `apply` would: the policy adopted (citing itself), then the mode decided.
    pub fn adopt(&self, mode: &str) {
        let text = std::fs::read_to_string(self.domain.join("settings.toml")).unwrap();
        let digest = sha256_hex(text.as_bytes());
        let mut s = Session::begin(&self.domain, None, &fabrewk_core::identity::PolicyVerifier).unwrap();
        let payload = json!({"content": text, "digest": digest, "heads": {}, "widened": []});
        let head =
            s.append("project", None, &self.draft("policy-adopted", &digest, payload), Some(&self.signer())).unwrap();
        let payload = json!({"mode": mode, "warningShown": mode == "unpublished"});
        let d = self.draft("publication-decided", &digest, payload);
        s.append("project", Some(&head), &d, Some(&self.signer())).unwrap();
        s.finish(&fabrewk_core::identity::PolicyVerifier, &[]).unwrap();
    }

    /// Writes the baseline `apply` would: the digest of the alambique as it stands, approved by `xel`.
    pub fn baseline(&self) {
        let policy = sha256_hex(std::fs::read_to_string(self.domain.join("settings.toml")).unwrap().as_bytes());
        let digest = sha256_hex(&std::fs::read(self.domain.join("ALAMBIQUE.md")).unwrap());
        let mut s = Session::begin(&self.domain, None, &fabrewk_core::identity::PolicyVerifier).unwrap();
        let head = s.head("alambique").unwrap();
        let d = self.draft("alambique-baseline", &policy, json!({"digest": digest}));
        s.append("alambique", head.as_ref(), &d, Some(&self.signer())).unwrap();
        s.finish(&fabrewk_core::identity::PolicyVerifier, &[]).unwrap();
    }

    /// Writes `text` as the alambique and its baseline; in `published` mode commits both.
    pub fn alambique(&self, text: &str, mode: &str) {
        std::fs::write(self.domain.join("ALAMBIQUE.md"), text).unwrap();
        self.baseline();
        if mode == "published" {
            self.commit("alambique");
        }
    }

    /// Proposes, as an agent, a stage with one task; gives their identifiers.
    pub fn stage_and_task(&self) -> (String, String) {
        self.stage_and_task_by(json!({"kind": "agent", "tool": "codex", "provider": "openai", "modelRequested": "gpt-6.1-sol", "modelObserved": null}))
    }

    /// A stage and its task, proposed by `agent` (an actor's object) through the library.
    pub fn stage_and_task_by(&self, agent: Value) -> (String, String) {
        let digest = sha256_hex(std::fs::read_to_string(self.domain.join("settings.toml")).unwrap().as_bytes());
        let draft = |kind: &str, mut payload: Value| {
            payload["nonce"] = json!(nonce().unwrap());
            let stream = subject_stream(kind, &payload).unwrap();
            let d = Draft {
                kind: kind.into(),
                actor: agent.clone(),
                policy: digest.clone(),
                instance: "SHA256:fixture".into(),
                say: None,
                payload,
            };
            (stream, d)
        };
        let mut s = Session::begin(&self.domain, None, &fabrewk_core::identity::PolicyVerifier).unwrap();
        let stage = json!({"lang": "en", "goal": "Build it.", "budget": {"dispatches": 6, "minutes": 120}, "checks": {}, "settings": {}});
        let (stage, d) = draft("stage-proposed", stage);
        s.append(&stage, None, &d, None).unwrap();
        let task = json!({
            "stage": &stage["stage-".len()..], "lang": "en", "goal": "Make it.",
            "requirements": [{"text": "It builds.", "criterion": "cargo build passes"}],
            "base": "0".repeat(40), "dependencies": [], "allowed": ["src/**"],
            "profiles": {"build": "build", "review": "review"}, "checks": [],
            "limits": {"rounds": 3, "dispatches": 6, "minutes": 120}, "delegationLanguage": "en", "settings": {},
        });
        let (task, d) = draft("task-proposed", task);
        s.append(&task, None, &d, None).unwrap();
        s.finish(&fabrewk_core::identity::PolicyVerifier, &[]).unwrap();
        (stage["stage-".len()..].to_owned(), task["task-".len()..].to_owned())
    }

    /// The private folder of the fixture's home.
    pub fn folder(&self) -> PathBuf {
        self.home.join(".config/fabrewk")
    }

    /// Runs the built binary in the fixture's root.
    pub fn run(&self, args: &[&str]) -> Ran {
        self.run_with(args, None)
    }

    /// As [`Fx::run`], with `PATH` replaced.
    pub fn run_with(&self, args: &[&str], path: Option<&std::ffi::OsStr>) -> Ran {
        run_in(&self.root, &self.home, args, path)
    }

    /// As [`Fx::run`], with arguments that need not be UTF-8 and variables added to the environment.
    pub fn run_os(&self, args: &[std::ffi::OsString], env: &[(&str, &std::ffi::OsStr)]) -> Ran {
        let mut c = Command::new(env!("CARGO_BIN_EXE_fabrewk"));
        isolate(&mut c, &self.home, None);
        for (k, v) in env {
            c.env(k, v);
        }
        let out = c.current_dir(&self.root).args(args).output().unwrap();
        Ran {
            status: out.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        }
    }

    /// As [`Fx::run`], with variables added to the environment.
    pub fn run_env(&self, args: &[&str], env: &[(&str, &str)]) -> Ran {
        let args: Vec<std::ffi::OsString> = args.iter().map(Into::into).collect();
        let env: Vec<(&str, &std::ffi::OsStr)> = env.iter().map(|(k, v)| (*k, std::ffi::OsStr::new(*v))).collect();
        self.run_os(&args, &env)
    }

    /// Every event of `stream`, in order, as JSON.
    pub fn events(&self, stream: &str) -> Vec<Value> {
        let dir = self.domain.join("data/events").join(stream);
        let mut files: Vec<PathBuf> =
            std::fs::read_dir(&dir).map(|d| d.map(|e| e.unwrap().path()).collect()).unwrap_or_default();
        files.retain(|f| f.extension().is_some_and(|e| e == "json"));
        files.sort();
        files.iter().map(|f| serde_json::from_slice(&std::fs::read(f).unwrap()).unwrap()).collect()
    }

    /// As [`Fx::edit_policy`], committing with a fixed author, whatever the repository's own configuration says.
    pub fn edit_policy_as(&self, text: &str) {
        std::fs::write(self.domain.join("settings.toml"), text).unwrap();
        self.git(&["add", ".brewstillery/settings.toml"]);
        self.git(&["-c", "user.name=Fixture", "commit", "-q", "-m", "policy"]);
    }

    /// Replaces the project file with `text` and commits it.
    pub fn edit_policy(&self, text: &str) {
        std::fs::write(self.domain.join("settings.toml"), text).unwrap();
        self.git(&["add", ".brewstillery/settings.toml"]);
        self.git(&["commit", "-q", "-m", "policy"]);
    }

    /// Every file and directory under the root, the home and the keys, with its mode and bytes.
    pub fn snapshot(&self) -> BTreeMap<PathBuf, (u32, Vec<u8>)> {
        let mut out = BTreeMap::new();
        for dir in [&self.root, &self.home] {
            walk(dir, &mut out);
        }
        out
    }
}

fn walk(dir: &Path, out: &mut BTreeMap<PathBuf, (u32, Vec<u8>)>) {
    use std::os::unix::fs::PermissionsExt;
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        let meta = std::fs::symlink_metadata(&path).unwrap();
        // A link's target, as written and never followed, so that a changed target is a change.
        let bytes = if meta.is_file() {
            std::fs::read(&path).unwrap_or_default()
        } else if meta.file_type().is_symlink() {
            std::fs::read_link(&path).map(|t| t.into_os_string().into_encoded_bytes()).unwrap_or_default()
        } else {
            Vec::new()
        };
        out.insert(path.clone(), (meta.permissions().mode(), bytes));
        if meta.is_dir() {
            walk(&path, out);
        }
    }
}

/// Runs the built binary in `dir` with `home` as its home.
pub fn run_in(dir: &Path, home: &Path, args: &[&str], path: Option<&std::ffi::OsStr>) -> Ran {
    let mut c = Command::new(env!("CARGO_BIN_EXE_fabrewk"));
    isolate(&mut c, home, path);
    let out = c.current_dir(dir).args(args).output().unwrap();
    Ran {
        status: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8(out.stdout).unwrap(),
        stderr: String::from_utf8(out.stderr).unwrap(),
    }
}

/// A directory holding links to the programs named, found in this process's `PATH`, and nothing else.
pub fn path_with(base: &Path, programs: &[&str]) -> std::ffi::OsString {
    let bin = base.join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    for p in programs {
        let found = std::env::split_paths(&std::env::var_os("PATH").unwrap()).map(|d| d.join(p)).find(|f| f.is_file());
        std::os::unix::fs::symlink(found.unwrap_or_else(|| panic!("{p} is needed by the test")), bin.join(p)).unwrap();
    }
    bin.into_os_string()
}

/// A test that fails between taking a permission away and giving it back leaves no directory its owner cannot
/// remove: dropping a fixture gives the owner access to every directory under it.
impl Drop for Fx {
    fn drop(&mut self) {
        fn open(dir: &Path) {
            use std::os::unix::fs::PermissionsExt;
            let Ok(meta) = std::fs::symlink_metadata(dir) else { return };
            if meta.is_dir() {
                let _ =
                    std::fs::set_permissions(dir, std::fs::Permissions::from_mode(meta.permissions().mode() | 0o700));
                for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
                    open(&entry.path());
                }
            }
        }
        if let Some(base) = self.root.parent() {
            open(base);
        }
    }
}

/// Removes, once per test process, the fixtures of test processes that have ended (their names carry the process's
/// identifier), opening every directory first; so that runs that repeat, as mutation testing does, do not fill the
/// disk (session 21).
fn sweep_ended(prefix: &str) {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        fn open(dir: &Path) {
            use std::os::unix::fs::PermissionsExt;
            let Ok(meta) = std::fs::symlink_metadata(dir) else { return };
            if meta.is_dir() {
                let _ =
                    std::fs::set_permissions(dir, std::fs::Permissions::from_mode(meta.permissions().mode() | 0o700));
                for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
                    open(&entry.path());
                }
            }
        }
        for entry in std::fs::read_dir(env!("CARGO_TARGET_TMPDIR")).into_iter().flatten().flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            let pid =
                name.strip_prefix(prefix).and_then(|rest| rest.rsplit('-').nth(1)).and_then(|p| p.parse::<u32>().ok());
            if pid.is_some_and(|p| p != std::process::id() && !Path::new(&format!("/proc/{p}")).exists()) {
                open(&entry.path());
                let _ = std::fs::remove_dir_all(entry.path());
            }
        }
    });
}

/// A human output holds no placeholder, no internal key and no command the declaration refuses (PD-36, PD-38).
pub fn whole(text: &str) {
    assert!(!text.contains(['{', '}']), "a placeholder: {text}");
    assert!(!text.contains("(a message the catalogue lacks)") && !text.contains("(un mensaje que falta"), "{text}");
    for line in text.lines().map(str::trim).filter(|l| l.starts_with("fabrewk ") && !l.contains(['<', '['])) {
        let words: Vec<&str> = line.split(' ').collect();
        assert!(fabrewk_core::cli::parse(words[1..].iter().map(std::ffi::OsString::from)).is_ok(), "{line}");
    }
}

/// Runs `args` as a person and with `--json`; asserts that nothing changed on disk, the exit status, that the code
/// is in both outputs with the catalogue's reason and next step in `lang`, and that a command to run parses (11.2
/// rule 4). Gives the document's entry for the code.
pub fn refused(fx: &Fx, args: &[&str], code: &str, exit: i32, lang: &str) -> Value {
    // A write command's refusal is one its declaration lists, so its help and the reference name it (REQ-CLI-1).
    let command = fabrewk_core::cli::COMMANDS.iter().find(|c| Some(&c.name) == args.first());
    if let Some(c) = command.filter(|c| fabrewk_core::cli::writes(c)) {
        let listed: Vec<String> = fabrewk_core::cli::refusals(c).iter().map(|r| r.to_string()).collect();
        assert!(listed.iter().any(|r| r == code), "{} does not declare {code}", c.name);
    }
    let before = fx.snapshot();
    let human = fx.run(args);
    let json = fx.run(&[args, &["--json"]].concat());
    assert_eq!(fx.snapshot(), before, "{args:?} wrote");
    assert_eq!((human.status, json.status), (exit, exit), "{args:?}: {human:?}");
    whole(&human.all());
    assert!(
        human.all().contains(&format!("{code}: {}", fabrewk_core::cli::say(lang, &format!("code.{code}.reason"), &[]))),
        "{human:?}"
    );
    assert!(human.all().contains(&fabrewk_core::cli::say(lang, &format!("code.{code}.next"), &[])), "{human:?}");
    let doc = json.doc();
    let list = if doc["schema"] == "fabrewk.check/1" { &doc["findings"] } else { &doc["refusals"] };
    let entry = list.as_array().unwrap().iter().find(|e| e["code"] == code).unwrap_or_else(|| panic!("{code}: {doc}"));
    assert_eq!(entry["next"], fabrewk_core::cli::say(lang, &format!("code.{code}.next"), &[]));
    // The human output names where and the detail exactly when the document has them.
    for (field, label) in [("path", "ui.where"), ("detail", "ui.detail")] {
        let label = format!("  {}: ", fabrewk_core::cli::say(lang, label, &[]));
        match entry[field].as_str().filter(|v| !v.is_empty()) {
            Some(v) => assert!(human.all().contains(&format!("{label}{v}\n")), "{field}: {human:?}"),
            None => assert!(!human.all().contains(&label), "{field}: {human:?}"),
        }
    }
    if let Some(run) = entry["run"].as_str() {
        assert!(human.all().contains(run));
        assert!(matches!(
            fabrewk_core::cli::parse(run.split(' ').skip(1).map(std::ffi::OsString::from)),
            Ok(fabrewk_core::cli::Parsed::Run { .. })
        ));
    }
    entry.clone()
}
