//! The Codex adapter's launch, cells, machine rule and preflight (REQ-SEC-1 for the adapter, REQ-SEC-6, REQ-SEC-7,
//! PD-27, 6.38, `confinement-design.md` 3.6 and 3.9; part 9d), with fake CLIs placed as the real tool's release in real
//! cells. Every fixture lives under Cargo's `CARGO_TARGET_TMPDIR`, its home the fixture's own; nothing of the machine's
//! home or the real tool's credential is read.

use fabrewk_adapter_codex::{ADAPTER, SETTINGS, profile};
use fabrewk_confine_bwrap::BACKEND;
use fabrewk_core::adapter::{self, Adapter, Launch, Role, Tool};
use fabrewk_core::confine::{self, End, Limits, Programs, Scratch};
use fabrewk_core::detect::{Detection, Found, How, Space};
use fabrewk_core::record::sha256_hex;
use fabrewk_core::settings::{self, Def};
use fabrewk_core::{Code, Error};
use serde_json::json;
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

fn which(executable: &str) -> PathBuf {
    std::env::split_paths(&std::env::var_os("PATH").unwrap())
        .map(|d| d.join(executable))
        .find(|p| p.is_file())
        .unwrap_or_else(|| panic!("{executable} is not in PATH; part 9d's tests need it"))
}

fn dir(name: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let d = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "codex-{name}-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst)
    ));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(d.join("domain/local")).unwrap();
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

/// A release as Codex's installer lays it out, `releases/9.9.9/` with a fake `bin/codex` made of `script` and a
/// helper in `codex-resources/`, found through a link in `bin/` as the person's installer leaves it.
fn release(at: &Path, script: &str) -> PathBuf {
    let rel = at.join("codex/packages/standalone/releases/9.9.9");
    std::fs::create_dir_all(rel.join("bin")).unwrap();
    std::fs::create_dir_all(rel.join("codex-resources")).unwrap();
    let exe = rel.join("bin/codex");
    std::fs::write(&exe, format!("#!/bin/sh\n{script}")).unwrap();
    std::fs::write(rel.join("codex-resources/helper"), "#!/bin/sh\necho helped\n").unwrap();
    for f in [&exe, &rel.join("codex-resources/helper")] {
        std::fs::set_permissions(f, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    std::fs::create_dir_all(at.join("bin")).unwrap();
    std::os::unix::fs::symlink(&exe, at.join("bin/codex")).unwrap();
    at.join("bin/codex")
}

/// A fake whose version needs its helper, and whose sandbox runs the command after `--` in a nested `bwrap`, the copy
/// read-only under the reviewer's profile, as the real tool's does.
const FAKE: &str = r#"case "$1" in
--version) "$(dirname "$0")/../codex-resources/helper" >/dev/null && echo 'codex-cli 9.9.9';;
sandbox) work=--bind; for a; do case "$a" in *'":read-only"'*) work=--ro-bind;; esac; done
  while [ "$1" != -- ]; do shift; done; shift
  exec /usr/bin/bwrap --unshare-all --die-with-parent --ro-bind / / $work /work /work --tmpfs /tmp --proc /proc -- "$@";;
esac
"#;

fn refused(r: Result<(), Error>, code: Code) -> String {
    match r {
        Err(Error::Refused { code: c, detail, path }) if c == code => {
            assert!(path.is_some_and(|p| p.join("stdout").is_file()), "{detail}");
            detail
        }
        other => panic!("expected {code}, got {other:?}"),
    }
}

#[test]
fn pd_27_the_preflight_passes_only_with_the_release_directory_bound_whole() {
    let d = dir("preflight");
    let (exe, rel) = adapter::resolve(&ADAPTER, &release(&d, FAKE), &[]).unwrap();
    assert_eq!(rel, d.join("codex/packages/standalone/releases/9.9.9").canonicalize().unwrap());
    let domain = d.join("domain");
    let p = programs(&domain);
    adapter::preflight(Tool { adapter: &ADAPTER, executable: &exe, release: &rel }, &BACKEND, &p, &domain).unwrap();
    assert_eq!(std::fs::read_dir(domain.join("local/tmp")).unwrap().count(), 0);
    let kept = std::fs::read_dir(domain.join("local/evidence")).unwrap().count();
    assert_eq!(kept, 1 + 3 * 2, "the backend's probe and the preflight's six runs");
    // Only `bin/` bound: the helper is not there, and the preflight refuses before any dispatch.
    let alone = Adapter { release: |exe| exe.parent().map(Path::to_path_buf), ..ADAPTER };
    let (exe, bin) = adapter::resolve(&alone, &d.join("bin/codex"), &[]).unwrap();
    let t = Tool { adapter: &alone, executable: &exe, release: &bin };
    let why = refused(adapter::preflight(t, &BACKEND, &p, &domain), Code::ConfinementUnavailable);
    assert!(why.contains("step 1") && why.contains("build"), "{why}");
}

#[test]
fn pd_27_a_sandbox_that_cannot_start_or_lets_a_reviewer_write_fails_the_preflight() {
    // Its sandbox needs a place the cell does not show.
    let d = dir("nosandbox");
    let script = FAKE.replace("--ro-bind / /", "--ro-bind / / --ro-bind /home/person /home/person");
    let (exe, rel) = adapter::resolve(&ADAPTER, &release(&d, &script), &[]).unwrap();
    let domain = d.join("domain");
    let t = Tool { adapter: &ADAPTER, executable: &exe, release: &rel };
    let why = refused(adapter::preflight(t, &BACKEND, &programs(&domain), &domain), Code::ConfinementUnavailable);
    assert!(why.contains("step 2"), "{why}");
    // Its sandbox ignores the reviewer's profile: the copy stays writable, and the reviewer's step refuses.
    let d = dir("writable");
    let script = FAKE.replace("work=--ro-bind", "work=--bind");
    let (exe, rel) = adapter::resolve(&ADAPTER, &release(&d, &script), &[]).unwrap();
    let domain = d.join("domain");
    let t = Tool { adapter: &ADAPTER, executable: &exe, release: &rel };
    let why = refused(adapter::preflight(t, &BACKEND, &programs(&domain), &domain), Code::ConfinementUnavailable);
    assert!(why.contains("step 3"), "{why}");
}

#[test]
fn the_tool_and_its_release_resolve_or_are_missing() {
    let d = dir("resolve");
    let link = release(&d, FAKE);
    let (exe, rel) = adapter::resolve(&ADAPTER, &link, &[]).unwrap();
    assert!(exe.starts_with(&rel) && exe.ends_with("bin/codex"));
    let missing =
        |r: Result<(PathBuf, PathBuf), Error>| matches!(r, Err(Error::Refused { code: Code::ToolMissing, .. }));
    assert!(missing(adapter::resolve(&ADAPTER, &d.join("bin/nothing"), &[])));
    // A release that holds the person's home or the repository would show it whole.
    let releases = d.join("codex/packages/standalone/releases");
    assert!(missing(adapter::resolve(&ADAPTER, &link, &[&releases.join("9.9.9/home")])));
    assert!(adapter::resolve(&ADAPTER, &link, &[&d.join("codex"), &d.join("bin")]).is_ok());
    // An executable outside `releases/<version>/bin/`, as one installed straight into a home, has no release.
    for loose in ["loose/codex", "loose/bin/codex", "releases/bin/codex"] {
        let at = d.join("other").join(loose);
        std::fs::create_dir_all(at.parent().unwrap()).unwrap();
        std::fs::copy(&exe, &at).unwrap();
        assert!(missing(adapter::resolve(&ADAPTER, &at, &[])), "{loose}");
    }
}

/// Every file under `root`, by path, with its digest; links by their target.
fn snapshot(root: &Path) -> BTreeMap<PathBuf, String> {
    let mut out = BTreeMap::new();
    let mut todo = vec![root.to_path_buf()];
    while let Some(p) = todo.pop() {
        let meta = std::fs::symlink_metadata(&p).unwrap();
        if meta.is_dir() {
            todo.extend(std::fs::read_dir(&p).unwrap().map(|e| e.unwrap().path()));
        } else if meta.is_symlink() {
            out.insert(p.clone(), format!("-> {}", std::fs::read_link(&p).unwrap().display()));
        } else {
            out.insert(p.clone(), sha256_hex(&std::fs::read(&p).unwrap()));
        }
    }
    out
}

/// The fake tool of a dispatch: it keeps its arguments, its environment and the schema it was given in the copy, tries
/// to rewrite its credential and the person's settings, and prints the stream of a successful run.
const DISPATCHED: &str = r#"for a in "$@"; do printf '%s\0' "$a"; done > /work/argv
env > /work/env
cp /home/agent/schema.json /work/schema
c=$HOME/.codex/auth.json
{ cat "$c" >/dev/null && echo read; printf 'x' >> "$c" 2>/dev/null && echo appended
  printf 'new' > "$c.tmp" && mv -f "$c.tmp" "$c" 2>/dev/null && echo renamed
  printf 'x = 1' > "$HOME/.codex/config.toml" && echo settings; ls -A "$HOME" "$HOME/.codex"; } > /work/tried 2>&1
echo '{"type":"thread.started","thread_id":"t"}'
echo '{"type":"turn.started"}'
echo '{"type":"item.completed","item":{"id":"i","type":"agent_message","text":"{\"summary\":\"done\"}"}}'
echo '{"type":"turn.completed","usage":{"input_tokens":3,"cached_input_tokens":1,"output_tokens":2}}'
"#;

#[test]
fn req_sec_6_a_dispatch_leaves_the_fixture_home_byte_identical_and_cannot_rewrite_the_credential() {
    let d = dir("dispatch");
    let home = d.join("home");
    let (exe, rel) = adapter::resolve(&ADAPTER, &release(&d, DISPATCHED), &[&home]).unwrap();
    std::fs::create_dir_all(home.join(".codex/skills/mine")).unwrap();
    std::fs::create_dir_all(home.join(".agents/skills/theirs")).unwrap();
    std::fs::write(home.join(".codex/auth.json"), "{\"fixture\":\"credential\"}\n").unwrap();
    std::fs::set_permissions(home.join(".codex/auth.json"), std::fs::Permissions::from_mode(0o600)).unwrap();
    std::fs::write(home.join(".codex/config.toml"), "[hooks]\nx = 1\n").unwrap();
    std::fs::write(home.join(".codex/AGENTS.md"), "the person's instructions\n").unwrap();
    std::fs::write(home.join(".codex/skills/mine/SKILL.md"), "a skill of the person's\n").unwrap();
    std::fs::write(home.join(".agents/skills/theirs/SKILL.md"), "another\n").unwrap();
    let before = snapshot(&home);
    let domain = d.join("domain");
    let p = programs(&domain);
    for role in Role::ALL {
        let scratch = Scratch::create(&domain, role.name()).unwrap();
        let evidence = confine::evidence(&domain, role.name()).unwrap();
        let schema = adapter::schema(role).to_string();
        let launch = Launch { role, packet: "{\"the\":\"packet\"}", schema: &schema, model: "m-1", effort: "low" };
        let limits = Limits { time: Duration::from_secs(60), bytes: 1 << 20 };
        let t = Tool { adapter: &ADAPTER, executable: &exe, release: &rel };
        let (ran, report) = adapter::dispatch(t, &BACKEND, &p, &launch, &home, &scratch, &evidence, limits).unwrap();
        assert_eq!(ran.end, End::Exited(0));
        assert_eq!(report.result, Some(json!({"summary": "done"})));
        assert_eq!(report.tokens.unwrap().input, Some(2));
        // The role's schema is where the command names it.
        assert_eq!(std::fs::read_to_string(scratch.work.join("schema")).unwrap(), schema);
        let tried = std::fs::read_to_string(scratch.work.join("tried")).unwrap();
        // Its own credential it reads; it can neither append to it nor replace it; the person's settings, skills and
        // instructions are not there, and what it writes in its home stays in the attempt's.
        assert!(tried.starts_with("read\n") && tried.lines().any(|l| l == "settings"), "{tried}");
        assert!(tried.contains("Read-only file system") || tried.contains("Device or resource busy"), "{tried}");
        assert!(!tried.contains("appended") && !tried.contains("renamed"), "{tried}");
        assert!(!tried.contains("skills") && !tried.contains("AGENTS") && !tried.contains(".agents"), "{tried}");
        let argv: Vec<String> = std::fs::read(scratch.work.join("argv"))
            .unwrap()
            .split(|b| *b == 0)
            .filter(|a| !a.is_empty())
            .map(|a| String::from_utf8(a.to_vec()).unwrap())
            .collect();
        let expected: Vec<String> =
            (ADAPTER.command)(&exe, &launch).into_iter().skip(1).map(|a| a.into_string().unwrap()).collect();
        assert_eq!(argv, expected);
        let env = std::fs::read_to_string(scratch.work.join("env")).unwrap();
        for line in ["HOME=/home/agent", "PATH=/usr/bin:/bin", "TMPDIR=/tmp"] {
            assert!(env.lines().any(|l| l == line), "{line} in {env}");
        }
        assert!(
            env.lines().all(|l| !l.starts_with("CARGO") && !l.starts_with("XDG") && !l.starts_with("CODEX")),
            "{env}"
        );
        scratch.remove().unwrap();
    }
    assert_eq!(snapshot(&home), before);
}

#[test]
fn a_launch_is_refused_in_a_directory_that_holds_another_repository() {
    let d = dir("machine");
    let home = d.join("home");
    let (exe, rel) = adapter::resolve(&ADAPTER, &release(&d, DISPATCHED), &[&home]).unwrap();
    // With no credential either: the machine rule is checked first.
    std::fs::create_dir_all(&home).unwrap();
    let domain = d.join("domain");
    let p = programs(&domain);
    type Plant = (&'static str, fn(&Path));
    let plant: [Plant; 4] = [
        ("nested/repo/.git", |p| std::fs::create_dir_all(p.join("objects")).unwrap()),
        (".git", |p| std::fs::write(p, "gitdir: /elsewhere\n").unwrap()),
        ("deep/er/.GiT", |p| std::fs::create_dir_all(p).unwrap()),
        ("a/.git", |p| std::os::unix::fs::symlink("/home", p).unwrap()),
    ];
    for (n, (path, make)) in plant.into_iter().enumerate() {
        let scratch = Scratch::create(&domain, &format!("m{n}")).unwrap();
        let evidence = confine::evidence(&domain, &format!("m{n}")).unwrap();
        let at = scratch.work.join(path);
        std::fs::create_dir_all(at.parent().unwrap()).unwrap();
        make(&at);
        let launch = Launch { role: Role::Review, packet: "{}", schema: "{}", model: "m", effort: "low" };
        let limits = Limits { time: Duration::from_secs(60), bytes: 1 << 20 };
        let t = Tool { adapter: &ADAPTER, executable: &exe, release: &rel };
        let u = adapter::dispatch(t, &BACKEND, &p, &launch, &home, &scratch, &evidence, limits).unwrap_err();
        assert!(u.ended && u.error.code() == Some(Code::ConfinementUnavailable), "{path}: {:?}", u.error);
        assert!(u.error.detail().contains(".git") || u.error.detail().contains(".GiT"), "{:?}", u.error);
        // No process started: nothing kept, nothing written in its home.
        assert_eq!(std::fs::read_dir(&evidence).unwrap().count(), 0, "{path}");
        assert_eq!(std::fs::read_dir(&scratch.home).unwrap().count(), 0, "{path}");
    }
    // Names that only resemble it are not another repository: the launch goes on, to the missing credential.
    let scratch = Scratch::create(&domain, "near").unwrap();
    let evidence = confine::evidence(&domain, "near").unwrap();
    for near in [".github/x", "x.git/y", ".gitignore", ".git2/z"] {
        let at = scratch.work.join(near);
        std::fs::create_dir_all(at.parent().unwrap()).unwrap();
        std::fs::write(&at, "").unwrap();
    }
    let launch = Launch { role: Role::Review, packet: "{}", schema: "{}", model: "m", effort: "low" };
    let limits = Limits { time: Duration::from_secs(60), bytes: 1 << 20 };
    let t = Tool { adapter: &ADAPTER, executable: &exe, release: &rel };
    let u = adapter::dispatch(t, &BACKEND, &p, &launch, &home, &scratch, &evidence, limits).unwrap_err();
    assert_eq!(u.error.code(), Some(Code::ToolFailed), "{:?}", u.error);
}

#[test]
fn each_role_launches_headless_with_its_discovery_off_and_its_own_sandbox_under_its_profile() {
    let exe = Path::new("/opt/codex/releases/9.9.9/bin/codex");
    for (role, extends) in [(Role::Build, "\":workspace\""), (Role::Review, "\":read-only\"")] {
        let launch = Launch { role, packet: "the packet", schema: "{\"s\":1}", model: "m-1", effort: "low" };
        let argv: Vec<OsString> = (ADAPTER.command)(exe, &launch);
        let a: Vec<&str> = argv.iter().map(|a| a.to_str().unwrap()).collect();
        assert_eq!(a[..2], [exe.to_str().unwrap(), "exec"]);
        let after = |flag: &str| a[a.iter().position(|x| *x == flag).unwrap_or_else(|| panic!("{flag}")) + 1];
        for flag in ["--json", "--strict-config", "--skip-git-repo-check", "--ephemeral"] {
            assert!(a.contains(&flag), "{flag}");
        }
        for flag in ["--ignore-user-config", "--ignore-rules"] {
            assert!(a.contains(&flag), "{flag}");
        }
        let pairs = [("--color", "never"), ("--cd", "/work"), ("--model", "m-1"), ("--output-schema", adapter::SCHEMA)];
        for (flag, value) in pairs {
            assert_eq!(after(flag), value, "{flag}");
        }
        // Every setting comes after `-c`, the role's profile among them, and the effort.
        let set: Vec<&str> = a.windows(2).filter(|w| w[0] == "-c").map(|w| w[1]).collect();
        for s in SETTINGS.iter().chain(profile(role).iter().filter(|s| **s != "-c")) {
            assert!(set.contains(s), "{s}");
        }
        assert!(set.contains(&format!("permissions.fabrewk.extends={extends}").as_str()));
        assert!(set.contains(&"model_reasoning_effort=\"low\""));
        for s in [
            "approval_policy=\"never\"",
            "default_permissions=\"fabrewk\"",
            "permissions.fabrewk.network.enabled=false",
            "permissions.fabrewk.filesystem={\"/home/agent/.codex\"=\"none\"}",
            "project_doc_max_bytes=0",
            "skills.include_instructions=false",
            "web_search=\"disabled\"",
            "features.hooks=false",
            "features.plugins=false",
            "features.unified_exec=false",
        ] {
            assert!(set.contains(&s), "{s}");
        }
        // The packet is the last argument, after the end of the options.
        assert_eq!(&a[a.len() - 2..], ["--", "the packet"]);
        assert!(!a.iter().any(|x| x.contains("bypass") || x.contains("dangerously") || x.contains("full-access")));
        assert!(!a.iter().any(|x| *x == "--sandbox" || *x == "-s" || x.contains("network_access=true")));
    }
}

#[test]
fn req_sec_7_and_s_4_the_adapter_declares_its_markers_its_names_its_program_and_its_credential() {
    assert_eq!(ADAPTER.markers, ["CODEX_SANDBOX", "CODEX_SANDBOX_NETWORK_DISABLED", "CODEX_THREAD_ID"]);
    assert_eq!(ADAPTER.protected, ["AGENTS.md", ".codex", ".agents"]);
    let Def::Lit(default) = settings::declared("gate.protectedNames").unwrap().default else { panic!() };
    for name in ADAPTER.protected {
        assert!(default.contains(&format!("\"{name}\"")), "{name}");
    }
    assert_eq!(
        (ADAPTER.program.adapter, ADAPTER.program.executable, ADAPTER.program.version, ADAPTER.program.tokens),
        (Some("codex"), "codex", &["--version"][..], true)
    );
    assert_eq!(ADAPTER.program.skills, [".codex/skills", ".agents/skills"]);
    assert_eq!((ADAPTER.provider, ADAPTER.credential), ("openai", ".codex/auth.json"));
    assert_eq!(ADAPTER.machine, [".git"]);
    assert!(ADAPTER.env.is_empty());
    // Every setting turns something off: none turns anything on.
    assert!(SETTINGS.iter().all(|s| !s.contains("=true")), "{SETTINGS:?}");
}
