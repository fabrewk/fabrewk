//! The Claude Code adapter's launch, cells and preflight (REQ-SEC-1 for the adapter, REQ-SEC-6, REQ-SEC-7, PD-27,
//! `confinement-design.md` 3.6 and 3.9; part 9c), with fake CLIs placed as the real tool's release in real cells.
//! Every fixture lives under Cargo's `CARGO_TARGET_TMPDIR`, its home the fixture's own; nothing of the machine's home
//! or the real tool's credential is read.

use fabrewk_adapter_claude::{ADAPTER, SETTINGS};
use fabrewk_confine_bwrap::BACKEND;
use fabrewk_core::adapter::{self, Adapter, Launch, Role, Tool};
use fabrewk_core::confine::{self, End, Limits, Programs, Scratch};
use fabrewk_core::detect::{Detection, Found, How, Space};
use fabrewk_core::record::sha256_hex;
use fabrewk_core::settings::{self, Def};
use fabrewk_core::{Code, Error};
use serde_json::{Value, json};
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
        .unwrap_or_else(|| panic!("{executable} is not in PATH; part 9c's tests need it"))
}

fn dir(name: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let d = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "claude-{name}-{}-{}",
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

/// A release directory `versions/` holding a fake `claude`, made of `script`, and its sibling `helper`.
fn release(at: &Path, script: &str) -> PathBuf {
    let versions = at.join("share/claude/versions");
    std::fs::create_dir_all(&versions).unwrap();
    let exe = versions.join("9.9.9");
    std::fs::write(&exe, format!("#!/bin/sh\n{script}")).unwrap();
    std::fs::write(versions.join("helper"), "#!/bin/sh\necho helped\n").unwrap();
    for f in [&exe, &versions.join("helper")] {
        std::fs::set_permissions(f, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    // Found as the person's installer leaves it: a link in `bin/` to the version.
    std::fs::create_dir_all(at.join("bin")).unwrap();
    std::os::unix::fs::symlink(&exe, at.join("bin/claude")).unwrap();
    at.join("bin/claude")
}

/// A fake whose version needs its sibling helper.
const NEEDS_HELPER: &str =
    "test \"$1\" = --version && \"$(dirname \"$0\")/helper\" >/dev/null && echo '9.9.9 (Claude Code)'\n";

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
    let (exe, rel) = adapter::resolve(&ADAPTER, &release(&d, NEEDS_HELPER), &[]).unwrap();
    assert_eq!(rel, d.join("share/claude/versions").canonicalize().unwrap());
    let domain = d.join("domain");
    let p = programs(&domain);
    adapter::preflight(Tool { adapter: &ADAPTER, executable: &exe, release: &rel }, &BACKEND, &p, &domain).unwrap();
    // Every run's directories are gone, and each step of each role kept its evidence: three steps, two roles.
    assert_eq!(std::fs::read_dir(domain.join("local/tmp")).unwrap().count(), 0);
    let kept = std::fs::read_dir(domain.join("local/evidence")).unwrap().count();
    assert_eq!(kept, 1 + 3 * 2, "the backend's probe and the preflight's six runs");
    // Only the executable bound: its helper is not there, and the preflight refuses before any dispatch.
    let alone = Adapter { release: |exe| Some(exe.to_path_buf()), ..ADAPTER };
    let (exe, file) = adapter::resolve(&alone, &d.join("bin/claude"), &[]).unwrap();
    let t = Tool { adapter: &alone, executable: &exe, release: &file };
    let why = refused(adapter::preflight(t, &BACKEND, &p, &domain), Code::ConfinementUnavailable);
    assert!(why.contains("step 1") && why.contains("build"), "{why}");
}

#[test]
fn pd_27_a_fake_whose_own_sandbox_cannot_start_in_a_roles_cell_fails_the_preflight() {
    let d = dir("sandbox");
    // Its sandbox needs a place the cell does not show.
    let script = "case \"$1\" in --version) echo '9.9.9 (Claude Code)';; sandbox) exec /usr/bin/bwrap --unshare-all \
        --ro-bind / / --ro-bind /home/person /home/person -- /bin/true;; esac\n";
    let with_sandbox = Adapter {
        preflight: |exe| vec![vec![exe.into(), "--version".into()], vec![exe.into(), "sandbox".into()]],
        ..ADAPTER
    };
    let (exe, rel) = adapter::resolve(&with_sandbox, &release(&d, script), &[]).unwrap();
    let domain = d.join("domain");
    let t = Tool { adapter: &with_sandbox, executable: &exe, release: &rel };
    let why = refused(adapter::preflight(t, &BACKEND, &programs(&domain), &domain), Code::ConfinementUnavailable);
    assert!(why.contains("step 2"), "{why}");
}

#[test]
fn the_tool_and_its_release_resolve_or_are_missing() {
    let d = dir("resolve");
    let link = release(&d, NEEDS_HELPER);
    let (exe, rel) = adapter::resolve(&ADAPTER, &link, &[]).unwrap();
    assert!(exe.starts_with(&rel) && exe.ends_with("9.9.9"));
    let outside = Adapter { release: |_| Some(PathBuf::from("/usr")), ..ADAPTER };
    let gone = Adapter { release: |_| None, ..ADAPTER };
    let missing =
        |r: Result<(PathBuf, PathBuf), Error>| matches!(r, Err(Error::Refused { code: Code::ToolMissing, .. }));
    for (a, path) in [(&ADAPTER, d.join("bin/nothing")), (&outside, link.clone()), (&gone, link.clone())] {
        assert!(missing(adapter::resolve(a, &path, &[])));
    }
    // A release that holds the person's home or the repository would show it whole: refused, as written or resolved.
    let versions = d.join("share/claude/versions");
    for place in [versions.join("home"), versions.clone(), d.join("share/claude/versions/../versions/x")] {
        assert!(missing(adapter::resolve(&ADAPTER, &link, &[Path::new("/elsewhere"), &place])), "{place:?}");
    }
    assert!(adapter::resolve(&ADAPTER, &link, &[&d.join("share"), &d.join("bin")]).is_ok());
    // An executable outside a `versions` directory, as one installed straight into a home, has no release.
    let loose = d.join("loose");
    std::fs::create_dir_all(&loose).unwrap();
    std::fs::copy(&exe, loose.join("claude")).unwrap();
    assert!(missing(adapter::resolve(&ADAPTER, &loose.join("claude"), &[])));
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

/// The fake tool of a dispatch: it keeps its arguments and environment in the copy, tries to rewrite its credential
/// and the person's settings, and prints the stream of a successful run.
const DISPATCHED: &str = r#"for a in "$@"; do printf '%s\0' "$a"; done > /work/argv
env > /work/env
c=$HOME/.claude/.credentials.json
{ cat "$c" >/dev/null && echo read; printf 'x' >> "$c" 2>/dev/null && echo appended
  printf 'new' > "$c.tmp" && mv -f "$c.tmp" "$c" 2>/dev/null && echo renamed
  printf '{}' > "$HOME/.claude/settings.json" && echo settings; ls -A "$HOME" "$HOME/.claude"; } > /work/tried 2>&1
echo '{"type":"system","subtype":"init","model":"claude-fake"}'
echo '{"type":"result","subtype":"success","is_error":false,"structured_output":{"summary":"done"},"usage":{"input_tokens":1,"cache_creation_input_tokens":2,"output_tokens":3}}'
"#;

#[test]
fn req_sec_6_a_dispatch_leaves_the_fixture_home_byte_identical_and_cannot_rewrite_the_credential() {
    let d = dir("dispatch");
    let (exe, rel) = adapter::resolve(&ADAPTER, &release(&d, DISPATCHED), &[&d.join("home")]).unwrap();
    let home = d.join("home");
    std::fs::create_dir_all(home.join(".claude/skills/mine")).unwrap();
    std::fs::write(home.join(".claude/.credentials.json"), "{\"fixture\":\"credential\"}\n").unwrap();
    std::fs::set_permissions(home.join(".claude/.credentials.json"), std::fs::Permissions::from_mode(0o600)).unwrap();
    std::fs::write(home.join(".claude/settings.json"), "{\"hooks\":{\"x\":1}}\n").unwrap();
    std::fs::write(home.join(".claude.json"), "{\"person\":true}\n").unwrap();
    std::fs::write(home.join(".claude/skills/mine/SKILL.md"), "a skill of the person's\n").unwrap();
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
        assert_eq!(report.model.as_deref(), Some("claude-fake"));
        let tried = std::fs::read_to_string(scratch.work.join("tried")).unwrap();
        // Its own credential it reads; it can neither append to it nor replace it; the person's settings are not
        // there, and what it writes in its home stays in the attempt's.
        assert!(tried.starts_with("read\n") && tried.lines().any(|l| l == "settings"), "{tried}");
        assert!(tried.contains("Read-only file system"), "{tried}");
        assert!(!tried.contains("appended") && !tried.contains("renamed"), "{tried}");
        assert!(!tried.contains("skills") && !tried.contains("hooks"), "{tried}");
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
        for line in ["HOME=/home/agent", "DISABLE_AUTOUPDATER=1", "DISABLE_TELEMETRY=1", "PATH=/usr/bin:/bin"] {
            assert!(env.lines().any(|l| l == line), "{line} in {env}");
        }
        assert!(env.lines().all(|l| !l.starts_with("CARGO") && !l.starts_with("XDG")), "{env}");
        scratch.remove().unwrap();
    }
    assert_eq!(snapshot(&home), before);
}

#[test]
fn a_dispatch_with_no_credential_fails_before_any_process() {
    let d = dir("nocredential");
    let (exe, rel) = adapter::resolve(&ADAPTER, &release(&d, DISPATCHED), &[&d.join("home")]).unwrap();
    let home = d.join("home");
    std::fs::create_dir_all(home.join(".claude")).unwrap();
    let domain = d.join("domain");
    let p = programs(&domain);
    // Absent, then a link to a file: neither is the tool's own credential file.
    for setup in [None, Some(d.join("elsewhere"))] {
        if let Some(target) = &setup {
            std::fs::write(target, "{}").unwrap();
            std::os::unix::fs::symlink(target, home.join(".claude/.credentials.json")).unwrap();
        }
        let scratch = Scratch::create(&domain, &format!("a{}", setup.is_some())).unwrap();
        let evidence = confine::evidence(&domain, &format!("a{}", setup.is_some())).unwrap();
        let launch = Launch { role: Role::Build, packet: "{}", schema: "{}", model: "m", effort: "low" };
        let limits = Limits { time: Duration::from_secs(60), bytes: 1 << 20 };
        let t = Tool { adapter: &ADAPTER, executable: &exe, release: &rel };
        let u = adapter::dispatch(t, &BACKEND, &p, &launch, &home, &scratch, &evidence, limits).unwrap_err();
        assert!(u.ended && u.error.code() == Some(Code::ToolFailed), "{:?}", u.error);
        assert_eq!(std::fs::read_dir(&evidence).unwrap().count(), 0);
        assert_eq!(std::fs::read_dir(&scratch.work).unwrap().count(), 0);
    }
}

#[test]
fn each_role_launches_headless_with_its_discovery_off_its_own_sandbox_required_and_its_permissions() {
    let exe = Path::new("/opt/claude/versions/9.9.9");
    for (role, tools, mode) in [
        (Role::Build, "Bash,Read,Edit,Write,Glob,Grep", "acceptEdits"),
        (Role::Review, "Bash,Read,Glob,Grep", "dontAsk"),
    ] {
        let launch = Launch { role, packet: "the packet", schema: "{\"s\":1}", model: "m-1", effort: "low" };
        let argv: Vec<OsString> = (ADAPTER.command)(exe, &launch);
        let a: Vec<&str> = argv.iter().map(|a| a.to_str().unwrap()).collect();
        assert_eq!(a[0], exe.to_str().unwrap());
        let after = |flag: &str| a[a.iter().position(|x| *x == flag).unwrap_or_else(|| panic!("{flag}")) + 1];
        for flag in ["--print", "--verbose", "--safe-mode", "--restricted", "--strict-mcp-config"] {
            assert!(a.contains(&flag), "{flag}");
        }
        for flag in ["--disable-slash-commands", "--no-session-persistence"] {
            assert!(a.contains(&flag), "{flag}");
        }
        let pairs = [
            ("--output-format", "stream-json"),
            ("--permission-prompts", "none"),
            ("--permission-mode", mode),
            ("--tools", tools),
            ("--settings", SETTINGS),
            ("--json-schema", "{\"s\":1}"),
            ("--model", "m-1"),
            ("--effort", "low"),
        ];
        for (flag, value) in pairs {
            assert_eq!(after(flag), value, "{flag}");
        }
        // The packet is the last argument, after the end of the options.
        assert_eq!(&a[a.len() - 2..], ["--", "the packet"]);
        assert!(!a.iter().any(|x| x.contains("bypass") || x.contains("dangerously")));
    }
    let s: Value = serde_json::from_str(SETTINGS).unwrap();
    assert_eq!(s["sandbox"]["enabled"], true);
    assert_eq!(s["sandbox"]["failIfUnavailable"], true);
    assert_eq!(s["sandbox"]["allowUnsandboxedCommands"], false);
    assert_eq!(s["sandbox"]["network"]["allowedDomains"], json!([]));
    let deny = s["permissions"]["deny"].as_array().unwrap();
    for rule in ["Read(//home/agent/.claude/**)", "WebFetch", "WebSearch"] {
        assert!(deny.contains(&json!(rule)), "{rule}");
    }
}

#[test]
fn req_sec_7_and_s_4_the_adapter_declares_its_markers_its_names_its_program_and_its_credential() {
    assert_eq!(ADAPTER.markers, ["CLAUDECODE"]);
    assert_eq!(ADAPTER.protected, ["CLAUDE.md", ".claude"]);
    // Each name it declares is in `gate.protectedNames`' default (S-4), which the core declares for every adapter.
    let Def::Lit(default) = settings::declared("gate.protectedNames").unwrap().default else { panic!() };
    for name in ADAPTER.protected {
        assert!(default.contains(&format!("\"{name}\"")), "{name}");
    }
    assert_eq!(
        (ADAPTER.program.adapter, ADAPTER.program.executable, ADAPTER.program.version, ADAPTER.program.tokens),
        (Some("claude-code"), "claude", &["--version"][..], true)
    );
    assert_eq!(ADAPTER.program.skills, [".claude/skills"]);
    assert_eq!((ADAPTER.provider, ADAPTER.credential), ("anthropic", ".claude/.credentials.json"));
    // No update, telemetry or error report of the tool's own (REQ-SEC-6).
    for var in ["DISABLE_AUTOUPDATER", "DISABLE_UPDATES", "DISABLE_TELEMETRY", "DISABLE_ERROR_REPORTING"] {
        assert!(ADAPTER.env.contains(&(var, "1")), "{var}");
    }
    assert!(ADAPTER.env.contains(&("CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC", "1")));
}
