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

    /// Proposes, as an agent, a stage with one task; gives their identifiers.
    pub fn stage_and_task(&self) -> (String, String) {
        let agent = json!({"kind": "agent", "tool": "codex", "provider": "openai", "modelRequested": "gpt-6.1-sol", "modelObserved": null});
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
        let bytes = if meta.is_file() { std::fs::read(&path).unwrap_or_default() } else { Vec::new() };
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
