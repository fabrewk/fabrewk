//! Fixtures for the record's tests (core-specification 11.2 rule 2): every repository, key and home is built by the
//! test under Cargo's `CARGO_TARGET_TMPDIR`; nothing is read from the machine's home, private folder or SSH agent.
#![allow(dead_code)]

use fabrewk_core::record::{
    Code, Draft, Error, KeySigner, Publication, Repo, Say, Session, SshVerifier, encode, nonce,
    sha256_hex,
};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

pub const ISOLATED: &str = "FABREWK_TEST_ISOLATED";

/// A new, empty directory for one test.
pub fn tmp(name: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "record-{name}-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Runs the test `name` again in a child process whose home, configuration and Git settings are the fixture's own,
/// so that the library's calls to `git` and `ssh-keygen` read nothing of the machine's; in the child, runs `body`.
pub fn isolated(name: &str, body: impl FnOnce()) {
    if std::env::var_os(ISOLATED).is_some() {
        body();
        return;
    }
    let home = tmp(&format!("home-{name}"));
    let out = Command::new(std::env::current_exe().unwrap())
        .args([name, "--exact", "--nocapture", "--test-threads=1"])
        .env(ISOLATED, "1")
        .env("HOME", &home)
        .env("XDG_CONFIG_HOME", home.join(".config"))
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", home.join(".gitconfig"))
        .env_remove("SSH_AUTH_SOCK")
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .output()
        .unwrap();
    let text =
        String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success(), "isolated {name} failed:\n{text}");
    assert!(
        text.contains(&format!("test {name} ... ok")),
        "isolated {name} did not run:\n{text}"
    );
}

/// Runs a command in `dir` and gives its standard output; panics, with its output, if it fails.
pub fn run(dir: &Path, program: &str, args: &[&str]) -> String {
    let out = Command::new(program)
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{program} {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

pub fn git(dir: &Path, args: &[&str]) -> String {
    run(dir, "git", args).trim_end().to_owned()
}

pub fn code(e: &Error) -> Option<Code> {
    e.code()
}

pub fn codes<T: std::fmt::Debug>(r: Result<T, Vec<Error>>) -> Vec<Code> {
    r.expect_err("expected a refusal")
        .iter()
        .map(|e| e.code().expect("a code, not an I/O error"))
        .collect()
}

/// A fixture: a Git repository (or a plain directory) holding the domain `.brewstillery/`, with the SSH keys of the
/// maestro `xel` and of a stranger outside it.
pub struct Fixture {
    pub root: PathBuf,
    pub domain: PathBuf,
    pub keys: PathBuf,
}

impl Fixture {
    /// The fixture made by [`Fixture::new`] under `base`, or a copy of one.
    pub fn at(base: &Path) -> Fixture {
        let root = base.join("repo");
        Fixture {
            domain: root.join(".brewstillery"),
            root,
            keys: base.join("keys"),
        }
    }

    pub fn new(name: &str, with_git: bool) -> Fixture {
        let base = tmp(name);
        let root = base.join("repo");
        let domain = root.join(".brewstillery");
        let keys = base.join("keys");
        std::fs::create_dir_all(&domain).unwrap();
        std::fs::create_dir_all(&keys).unwrap();
        std::fs::write(domain.join(".gitignore"), "local/\n").unwrap();
        std::fs::write(root.join("README.md"), "fixture\n").unwrap();
        for alias in ["xel", "stranger"] {
            let key = keys.join(alias);
            run(
                &keys,
                "ssh-keygen",
                &[
                    "-q",
                    "-t",
                    "ed25519",
                    "-N",
                    "",
                    "-C",
                    alias,
                    "-f",
                    key.to_str().unwrap(),
                ],
            );
        }
        let public = std::fs::read_to_string(keys.join("xel.pub")).unwrap();
        std::fs::write(
            keys.join("allowed"),
            format!("xel namespaces=\"fabrewk\" {public}"),
        )
        .unwrap();
        if with_git {
            git(&root, &["init", "-q", "-b", "main"]);
            git(&root, &["config", "user.name", "Fixture"]);
            git(&root, &["config", "user.email", "fixture@example.invalid"]);
            git(&root, &["config", "commit.gpgsign", "false"]);
            git(&root, &["add", "."]);
            git(&root, &["commit", "-q", "-m", "fixture"]);
        }
        Fixture { root, domain, keys }
    }

    pub fn repo(&self) -> Option<Repo> {
        Repo::for_domain(&self.domain, Publication::Published)
    }

    pub fn signer(&self) -> KeySigner {
        KeySigner(self.keys.join("xel"))
    }

    pub fn stranger(&self) -> KeySigner {
        KeySigner(self.keys.join("stranger"))
    }

    pub fn verifier(&self) -> SshVerifier {
        SshVerifier(self.keys.join("allowed"))
    }

    pub fn stream_dir(&self, stream: &str) -> PathBuf {
        self.domain.join("data/events").join(stream)
    }

    /// The event files of a stream, in order of name.
    pub fn events(&self, stream: &str) -> Vec<PathBuf> {
        let mut files: Vec<PathBuf> = std::fs::read_dir(self.stream_dir(stream))
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().is_some_and(|e| e == "json"))
            .collect();
        files.sort();
        files
    }

    /// Writes `bytes` as an event file of `stream` at `seq`, named by its digest, bypassing the record's checks.
    pub fn place(&self, stream: &str, seq: u64, bytes: &[u8]) -> PathBuf {
        let dir = self.stream_dir(stream);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join(format!("{seq:06}-{}.json", &sha256_hex(bytes)[..16]));
        std::fs::write(&path, bytes).unwrap();
        path
    }

    /// Forges an event as a writer that ignores the lock would: built from `draft`, placed, and signed by `signer`.
    pub fn forge(
        &self,
        stream: &str,
        seq: u64,
        prev: Option<&str>,
        draft: &Draft,
        signer: Option<&KeySigner>,
    ) -> PathBuf {
        let bytes = encode(&draft.build(stream, seq, prev)).unwrap();
        let path = self.place(stream, seq, &bytes);
        if let Some(signer) = signer {
            std::fs::write(sig(&path), sign_with(signer, &bytes, "fabrewk")).unwrap();
        }
        path
    }
}

/// Copies a directory tree, files and directories only.
pub fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let path = entry.unwrap().path();
        let target = to.join(path.file_name().unwrap());
        if path.is_dir() {
            copy_tree(&path, &target);
        } else {
            std::fs::copy(&path, &target).unwrap();
        }
    }
}

/// Begins a write command on the fixture. A thread of the test harness that spawns a process holds, from its fork
/// to its exec, a copy of every open descriptor, the lock's included; a test that begins right after ending a
/// session in the same process waits out that window instead of reading it as a second writer.
pub fn try_begin(f: &Fixture, repo: Option<Repo>) -> Result<Session, Error> {
    for _ in 0..300 {
        match Session::begin(&f.domain, repo.clone(), &f.verifier()) {
            Err(e) if e.code() == Some(Code::LockHeld) => {
                std::thread::sleep(std::time::Duration::from_millis(10))
            }
            other => return other,
        }
    }
    Session::begin(&f.domain, repo, &f.verifier())
}

pub fn begin(f: &Fixture, repo: Option<Repo>) -> Session {
    try_begin(f, repo).unwrap()
}

pub fn sig(event: &Path) -> PathBuf {
    PathBuf::from(format!("{}.sig", event.display()))
}

pub fn sign_with(signer: &KeySigner, bytes: &[u8], namespace: &str) -> Vec<u8> {
    use std::io::Write;
    let mut child = Command::new("ssh-keygen")
        .args([
            "-Y",
            "sign",
            "-n",
            namespace,
            "-f",
            signer.0.to_str().unwrap(),
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    out.stdout
}

pub const POLICY: &str = "4c2a0d5c1f8a3b6e9d7c5b3a1f0e2d4c6b8a9f7e5d3c1b0a2f4e6d8c0b1a3f5e";

fn draft(kind: &str, actor: Value, say: Option<Say>, payload: Value) -> Draft {
    Draft {
        kind: kind.into(),
        actor,
        policy: POLICY.into(),
        instance: "SHA256:fixture".into(),
        say,
        payload,
    }
}

/// A move of the maestro `xel`, with words.
pub fn person(kind: &str, words: &str, payload: Value) -> Draft {
    let actor = json!({"kind": "person", "alias": "xel", "role": "maestro"});
    draft(
        kind,
        actor,
        Some(Say {
            text: words.into(),
            lang: "es-MX".into(),
        }),
        payload,
    )
}

pub fn fabrewk(kind: &str, payload: Value) -> Draft {
    draft(kind, json!({"kind": "fabrewk"}), None, payload)
}

pub fn agent(kind: &str, payload: Value) -> Draft {
    let actor = json!({"kind": "agent", "tool": "codex", "provider": "openai", "modelRequested": "gpt-6.1-sol", "modelObserved": null});
    draft(kind, actor, None, payload)
}

/// A creating event (`stage-proposed` or `task-proposed`) with a fresh nonce, and the stream it opens.
pub fn creation(kind: &str, payload: Value) -> (String, Draft) {
    let mut payload = payload;
    payload["nonce"] = json!(nonce().unwrap());
    let stream = fabrewk_core::record::subject_stream(kind, &payload).unwrap();
    (stream, agent(kind, payload))
}
