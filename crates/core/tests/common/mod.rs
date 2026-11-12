//! Fixtures for the record's tests (core-specification 11.2 rule 2): every repository, key and home is built by the
//! test under Cargo's `CARGO_TARGET_TMPDIR`; nothing is read from the machine's home, private folder or SSH agent.
#![allow(dead_code)]

use fabrewk_core::record::{
    Class, Code, Draft, Error, KeySigner, Publication, Replay, Repo, Say, Session, Verify, class_of, encode, nonce,
    sha256_hex,
};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

pub const ISOLATED: &str = "FABREWK_TEST_ISOLATED";

/// A new, empty directory for one test.
pub fn tmp(name: &str) -> PathBuf {
    sweep_ended("record-");
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
    let text = String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success(), "isolated {name} failed:\n{text}");
    assert!(text.contains(&format!("test {name} ... ok")), "isolated {name} did not run:\n{text}");
}

/// Runs a command in `dir` and gives its standard output; panics, with its output, if it fails.
pub fn run(dir: &Path, program: &str, args: &[&str]) -> String {
    let out = Command::new(program).current_dir(dir).args(args).output().unwrap();
    assert!(out.status.success(), "{program} {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).unwrap()
}

pub fn git(dir: &Path, args: &[&str]) -> String {
    run(dir, "git", args).trim_end().to_owned()
}

pub fn code(e: &Error) -> Option<Code> {
    e.code()
}

pub fn codes<T: std::fmt::Debug>(r: Result<T, Vec<Error>>) -> Vec<Code> {
    r.expect_err("expected a refusal").iter().map(|e| e.code().expect("a code, not an I/O error")).collect()
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
        Fixture { domain: root.join(".brewstillery"), root, keys: base.join("keys") }
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
            run(&keys, "ssh-keygen", &["-q", "-t", "ed25519", "-N", "", "-C", alias, "-f", key.to_str().unwrap()]);
        }
        let public = std::fs::read_to_string(keys.join("xel.pub")).unwrap();
        std::fs::write(keys.join("allowed"), format!("xel namespaces=\"fabrewk\" {public}")).unwrap();
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

    /// The record as it replays now; panics if it does not.
    pub fn replay(&self) -> Replay {
        fabrewk_core::record::replay(&self.domain, &self.verifier()).unwrap()
    }

    pub fn stranger(&self) -> KeySigner {
        KeySigner(self.keys.join("stranger"))
    }

    /// A verifier that knows only `xel`'s key, for the record's own tests; the identity's tests use the policy's.
    pub fn verifier(&self) -> FixedVerifier {
        let public = std::fs::read_to_string(self.keys.join("xel.pub")).unwrap();
        FixedVerifier(vec![("xel".into(), fabrewk_core::settings::key_blob(public.trim()).unwrap())])
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

/// Verifies every person's event with the key a fixed list gives its alias, by the same check as the policy's.
pub struct FixedVerifier(pub Vec<(String, Vec<u8>)>);

impl Verify for FixedVerifier {
    fn verify(&self, record: &Replay) -> Result<(), Vec<Error>> {
        let mut errors = Vec::new();
        for e in record.streams.values().flatten().filter(|e| class_of(&e.kind) == Some(Class::Person)) {
            let alias = e.value["actor"]["alias"].as_str().unwrap_or("");
            let key = self.0.iter().find(|k| k.0 == alias).map(|k| k.1.clone()).unwrap_or_default();
            let bytes = std::fs::read(&e.path).unwrap();
            if let Err(err) = fabrewk_core::record::check(&key, &bytes, &sig(&e.path)) {
                errors.push(err);
            }
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
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
    begin_with(f, repo, &f.verifier())
}

/// As [`try_begin`], with `verifier`.
pub fn begin_with(f: &Fixture, repo: Option<Repo>, verifier: &dyn Verify) -> Result<Session, Error> {
    for _ in 0..300 {
        match Session::begin(&f.domain, repo.clone(), verifier) {
            Err(e) if e.code() == Some(Code::LockHeld) => std::thread::sleep(std::time::Duration::from_millis(10)),
            other => return other,
        }
    }
    Session::begin(&f.domain, repo, verifier)
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
        .args(["-Y", "sign", "-n", namespace, "-f", signer.0.to_str().unwrap()])
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
    Draft { kind: kind.into(), actor, policy: POLICY.into(), instance: "SHA256:fixture".into(), say, payload }
}

/// The project file the fixtures adopt.
pub const PROJECT: &str = "[project]\nname = \"fixture\"\n";

/// A `project` event's payload: `policy-adopted` carries [`PROJECT`] (part 7c1), `publication-decided` the mode
/// `published` (part 7c2b); the others are not fixed yet.
pub fn project_payload(kind: &str) -> Value {
    match kind {
        "policy-adopted" => {
            json!({"content": PROJECT, "digest": sha256_hex(PROJECT.as_bytes()), "heads": {}, "widened": []})
        }
        "publication-decided" => json!({"mode": "published", "warningShown": false}),
        _ => json!({}),
    }
}

/// A move of the maestro `xel`, with words.
pub fn person(kind: &str, words: &str, payload: Value) -> Draft {
    let actor = json!({"kind": "person", "alias": "xel", "role": "maestro"});
    draft(kind, actor, Some(Say { text: words.into(), lang: "es-MX".into() }), payload)
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

/// A stage's fields, valid for `stage-proposed` (part 7d1); [`creation`] adds the nonce.
pub fn stage_fields() -> Value {
    json!({"lang": "en", "goal": "Build it.", "budget": {"dispatches": 6, "minutes": 120}, "checks": {}, "settings": {}})
}

/// A task's fields in the stage `stage` (its stream's name), valid for `task-proposed` (part 7d1).
pub fn task_fields(stage: &str) -> Value {
    json!({
        "stage": &stage["stage-".len()..], "lang": "en", "goal": "Make it.",
        "requirements": [{"text": "It builds.", "criterion": "cargo build passes"}],
        "base": "0".repeat(40), "dependencies": [], "allowed": ["src/**"],
        "profiles": {"build": "build", "review": "review"}, "checks": [],
        "limits": {"rounds": 3, "dispatches": 6, "minutes": 120}, "delegationLanguage": "en", "settings": {},
    })
}

/// Appends a stage and one task in it, as an agent proposes them; gives the task's stream and head.
pub fn stage_and_task(s: &mut Session) -> (String, fabrewk_core::record::Head) {
    let (stage, draft) = creation("stage-proposed", stage_fields());
    s.append(&stage, None, &draft, None).unwrap();
    let (task, draft) = creation("task-proposed", task_fields(&stage));
    let head = s.append(&task, None, &draft, None).unwrap();
    (task, head)
}

/// Forges a stage and one task in it on disk; gives the task's stream and its first file.
pub fn forge_stage_and_task(f: &Fixture) -> (String, PathBuf) {
    let (stage, draft) = creation("stage-proposed", stage_fields());
    f.forge(&stage, 1, None, &draft, None);
    let (task, draft) = creation("task-proposed", task_fields(&stage));
    let first = f.forge(&task, 1, None, &draft, None);
    (task, first)
}

/// The payload of a person's move that binds the task of `stream` (`task-authorized`, or the binding of
/// `task-accepted`) to the task's digest and to the head its stream has on disk now (part 7d1, PD-06), with the digest
/// of a rendering (`shown`, part 7d3b).
pub fn bound(f: &Fixture, stream: &str) -> Value {
    let files = f.events(stream);
    let first: Value = serde_json::from_slice(&std::fs::read(&files[0]).unwrap()).unwrap();
    let last = std::fs::read(files.last().unwrap()).unwrap();
    json!({
        "task": &stream["task-".len()..],
        "digest": fabrewk_core::record::subject_digest(&first["payload"]).unwrap(),
        "head": {"seq": files.len(), "digest": sha256_hex(&last)},
        "limits": first["payload"]["limits"], "shown": "0".repeat(64), "widened": [],
    })
}

/// The events of `stream` on disk, in order, with their digests.
fn on_disk(f: &Fixture, stream: &str) -> Vec<(Value, String)> {
    let read = |p: &PathBuf| std::fs::read(p).unwrap();
    f.events(stream).iter().map(|p| (serde_json::from_slice(&read(p)).unwrap(), sha256_hex(&read(p)))).collect()
}

/// The payload of a run that starts after the head `stream` has on disk now, relying on its last `task-authorized`,
/// else its stage's last `stage-authorized` (part 9e); with neither, on a made-up one.
pub fn started(f: &Fixture, stream: &str) -> Value {
    let events = on_disk(f, stream);
    let last = |stream: &str, kind: &str| {
        let events = on_disk(f, stream);
        let at = events.iter().rposition(|(e, _)| e["kind"] == kind)?;
        Some(json!({"stream": stream, "seq": at + 1, "digest": events[at].1}))
    };
    let stage = format!("stage-{}", events[0].0["payload"]["stage"].as_str().unwrap());
    let auth = last(stream, "task-authorized").or_else(|| last(&stage, "stage-authorized"));
    let auth = auth.unwrap_or_else(|| json!({"stream": stream, "seq": 1, "digest": "0".repeat(64)}));
    json!({
        "task": &stream["task-".len()..], "head": {"seq": events.len(), "digest": events.last().unwrap().1},
        "authorization": auth, "base": "0".repeat(40),
    })
}

/// An attempt's start (part 9e).
pub fn attempt() -> Value {
    json!({"role": "build", "profile": "build", "tool": "claude", "model": "m", "effort": "low", "allowedSeconds": 60})
}

/// The end of the last attempt `stream` started on disk, with `usage` (part 9e, S-5).
pub fn ended(f: &Fixture, stream: &str, usage: Value) -> Value {
    let events = on_disk(f, stream);
    let at = events.iter().rposition(|(e, _)| e["kind"] == "attempt-started").map_or(1, |i| i + 1);
    json!({"attempt": at, "outcome": "success", "model": null, "usage": usage, "evidence": null})
}

/// A usage with `seconds` and no tokens reported.
pub fn seconds(n: u64) -> Value {
    json!({"seconds": n, "tokens": null})
}

/// A halt with `code` (part 9e).
pub fn halted(code: &str) -> Value {
    json!({"code": code, "reason": "A reason."})
}

/// A candidate's record with no change (part 9e).
pub fn candidate() -> Value {
    json!({"commit": "1".repeat(40), "tree": "2".repeat(40), "changes": []})
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
