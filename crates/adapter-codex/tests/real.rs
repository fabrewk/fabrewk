//! The real Codex on this machine (core-specification 11.2 rule 1): never part of `cargo test`. Each test is
//! ignored and runs only when `FABREWK_REAL_CODEX` is set, with a maestro's permission and budget: the preflight,
//! which calls no model, and the dispatches, which do. Each keeps its evidence under Cargo's `CARGO_TARGET_TMPDIR` and
//! prints where.

use fabrewk_adapter_codex::ADAPTER;
use fabrewk_confine_bwrap::BACKEND;
use fabrewk_core::adapter::{self, Launch, Role, Tool};
use fabrewk_core::cli::say;
use fabrewk_core::confine::{self, Limits, Programs, Scratch};
use fabrewk_core::detect::{Detection, Found, How, Space};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

fn which(executable: &str) -> PathBuf {
    std::env::split_paths(&std::env::var_os("PATH").unwrap())
        .map(|d| d.join(executable))
        .find(|p| p.is_file())
        .unwrap_or_else(|| panic!("{executable} is not in PATH"))
}

fn allowed() -> bool {
    std::env::var_os("FABREWK_REAL_CODEX").is_some()
}

fn domain(name: &str) -> PathBuf {
    let d = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("real-{name}-{}", std::process::id()));
    std::fs::create_dir_all(d.join("local")).unwrap();
    d
}

fn programs(domain: &Path) -> Programs {
    let mut found = BTreeMap::new();
    for name in ["bwrap", "pasta"] {
        let at = Found { path: which(name), version: None, how: How::Observed, verified: None };
        found.insert((Space::Program, name.to_owned()), Some(at));
    }
    confine::available(Some(&BACKEND), &mut Detection { at: String::new(), found }, domain).unwrap()
}

fn tool() -> (PathBuf, PathBuf) {
    let home = PathBuf::from(std::env::var_os("HOME").unwrap());
    adapter::resolve(&ADAPTER, &which("codex"), &[&home, Path::new(env!("CARGO_MANIFEST_DIR"))]).unwrap()
}

#[test]
#[ignore = "runs the real Codex; set FABREWK_REAL_CODEX"]
fn real_preflight() {
    assert!(allowed(), "set FABREWK_REAL_CODEX");
    let d = domain("preflight");
    let (exe, release) = tool();
    let t = Tool { adapter: &ADAPTER, executable: &exe, release: &release };
    let r = adapter::preflight(t, &BACKEND, &programs(&d), &d);
    eprintln!("preflight: {r:?}; evidence in {}", d.join("local/evidence").display());
    r.unwrap();
}

/// One dispatch in `role` on a copy holding `files`, with the model and effort `FABREWK_REAL_CODEX` names
/// (`<model>:<effort>`), its packet built for `task`. Its directories are kept for reading.
fn dispatch(role: Role, files: &[(&str, &str)], task: &serde_json::Value) {
    assert!(allowed(), "set FABREWK_REAL_CODEX");
    let spec = std::env::var("FABREWK_REAL_CODEX").unwrap();
    let (model, effort) = spec.split_once(':').expect("<model>:<effort>");
    let d = domain(role.name());
    let (exe, release) = tool();
    let t = Tool { adapter: &ADAPTER, executable: &exe, release: &release };
    let scratch = Scratch::create(&d, "attempt").unwrap();
    for (path, text) in files {
        let p = scratch.work.join(path);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, text).unwrap();
    }
    let mut rules = vec![say("en", &format!("packet.{}", role.name()), &[])];
    rules.extend([8, 11, 12].map(|n| say("en", &format!("rules.base.{n}"), &[])));
    let packet = adapter::packet(task, role, &rules, "en", None);
    std::fs::write(d.join("packet.json"), &packet).unwrap();
    let schema = adapter::schema(role).to_string();
    let launch = Launch { role, packet: &packet, schema: &schema, model, effort };
    let evidence = confine::evidence(&d, "attempt").unwrap();
    let home = PathBuf::from(std::env::var_os("HOME").unwrap());
    let limits = Limits { time: Duration::from_secs(600), bytes: 8 << 20 };
    let r = adapter::dispatch(t, &BACKEND, &programs(&d), &launch, &home, &scratch, &evidence, limits);
    eprintln!("dispatch: {r:?}\nevidence in {}; copy in {}", evidence.display(), scratch.work.display());
}

const FILES: [(&str, &str); 3] = [
    ("README.md", "# sum\n\nA tiny library: `sum.py` adds a list of numbers.\n"),
    ("sum.py", "def total(xs):\n    t = 0\n    for x in xs:\n        t += x\n    return t\n"),
    ("test_sum.py", "from sum import total\n\nassert total([1, 2, 3]) == 6\nassert total([]) == 0\nprint('ok')\n"),
];

#[test]
#[ignore = "calls a model; set FABREWK_REAL_CODEX"]
fn real_review() {
    let task = serde_json::json!({
        "goal": "sum.py's total adds a list of numbers",
        "requirements": [
            {"text": "total(xs) returns the sum of the numbers in xs, 0 for an empty list",
             "criterion": "python3 test_sum.py prints ok"},
        ],
        "allowed": ["sum.py", "test_sum.py"],
        "delegationLanguage": "en",
    });
    dispatch(Role::Review, &FILES, &task);
}

#[test]
#[ignore = "calls a model; set FABREWK_REAL_CODEX"]
fn real_build() {
    let task = serde_json::json!({
        "goal": "sum.py also gives the mean of a list of numbers",
        "requirements": [
            {"text": "mean(xs) in sum.py returns the arithmetic mean of xs and raises ValueError for an empty list",
             "criterion": "python3 test_sum.py prints ok, and test_sum.py checks mean([1, 2, 3]) == 2 and the ValueError"},
        ],
        "allowed": ["sum.py", "test_sum.py"],
        "delegationLanguage": "en",
    });
    dispatch(Role::Build, &FILES, &task);
}

/// What the tool's own sandbox shows the model's commands (3.6, T-05): run once by the reviewer, reported verbatim.
const PROBE: &str = r#"echo "status: $(grep -E '^(Seccomp|NoNewPrivs):' /proc/self/status | tr -s '\t\n' '  ')"
echo "interfaces: $(tail -n +3 /proc/net/dev | cut -d: -f1 | tr -d ' ' | tr '\n' ,)"
echo "proxy: ${HTTPS_PROXY:-none} ${ALL_PROXY:-none}"
echo "codex home: $(ls -A "$HOME/.codex" 2>&1 | tr '\n' ' ')"
echo "tmpdir: ${TMPDIR:-none}"
python3 -c "import socket;socket.create_connection(('1.1.1.1',443),3);print('direct: open')" 2>&1 | tail -n 1
python3 -c "import urllib.request;urllib.request.urlopen('https://example.com',timeout=5);print('proxied: open')" 2>&1 | tail -n 1
if cat "$HOME/.codex/auth.json" >/dev/null 2>&1; then echo "credential: readable"; else echo "credential: unreadable"; fi
if touch "$HOME/probe-write" 2>/dev/null; then echo "home: writable"; else echo "home: not writable"; fi
if (echo x >> /work/.gitconfig) 2>/dev/null; then echo "gitconfig: writable"; else echo "gitconfig: not writable"; fi
if touch /work/probe-write 2>/dev/null; then echo "work: writable"; else echo "work: not writable"; fi
echo "variables: $(env | cut -d= -f1 | sort | tr '\n' ' ')"
"#;

/// What the tool's own sandbox shows the model's commands (3.6, T-05), and how the stream reports a command that
/// prints nothing (P9C-7): one review, two commands.
#[test]
#[ignore = "calls a model; set FABREWK_REAL_CODEX"]
fn real_sandbox_probe_and_silent_command() {
    let task = serde_json::json!({
        "goal": "probe.sh reports what a command in this copy can reach",
        "requirements": [
            {"text": "Run exactly two commands, each as its own command and in this order: first `test -f sum.py`, then `sh probe.sh`; run nothing else",
             "criterion": "the review's verdict is accept with exactly one finding, minor and suspected, with no path, whose summary is the whole output of sh probe.sh, verbatim"},
        ],
        "allowed": ["probe.sh"],
        "delegationLanguage": "en",
    });
    let mut files = FILES.to_vec();
    files.push(("probe.sh", PROBE));
    dispatch(Role::Review, &files, &task);
}
