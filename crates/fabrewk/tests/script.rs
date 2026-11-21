//! The installation script `install.sh` in `dash` (REQ-INS-6, REQ-INS-7; D-097 point 10; part 10d): a copy of the
//! script whose address points to a local release signed by a fixture key installs exactly one file,
//! `~/.local/bin/fabrewk`; a changed binary, checksum file or signature installs nothing; an unsupported platform is
//! `PLATFORM_UNSUPPORTED` with the supported list; it takes no option and has no `sudo`; inside a repository with a
//! terminal it offers `fabrewk onboard`, never inside an agent. Every run but the terminal's is in a session of its
//! own (`setsid`), so that no terminal is reachable.

mod common;

use common::{DRIVE, path_with, tmp};
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn script_text() -> String {
    fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../install.sh")).unwrap()
}

fn base() -> String {
    format!("fabrewk-{VERSION}-linux-aarch64")
}

/// The program the fixture's release carries: it says its version, and where it ran with which arguments, writing
/// the directory exactly into `FIXTURE_CWD` when that names a file.
const PROGRAM: &str = "#!/bin/sh\ncase \"$1\" in\n--version) echo \"fabrewk fixture\" ;;\n*) [ -z \"${FIXTURE_CWD:-}\" ] || printf %s \"$PWD\" >\"$FIXTURE_CWD\"; echo \"fixture ran in $(pwd): $*\" ;;\nesac\n";

struct Script {
    dir: PathBuf,
    home: PathBuf,
    tmpdir: PathBuf,
    work: PathBuf,
    release: PathBuf,
    key: PathBuf,
    script: PathBuf,
}

fn run_quiet(c: &mut Command) {
    let out = c.output().unwrap();
    assert!(out.status.success(), "{c:?}: {}", String::from_utf8_lossy(&out.stderr));
}

/// A fixture key, as `ssh-keygen` makes one, with no passphrase.
fn key(dir: &Path, name: &str) -> PathBuf {
    let key = dir.join(name);
    run_quiet(Command::new("ssh-keygen").args(["-q", "-t", "ed25519", "-N", "", "-C", name, "-f"]).arg(&key));
    key
}

fn sign(key: &Path, namespace: &str, file: &Path) {
    let _ = fs::remove_file(file.with_extension("sig"));
    run_quiet(Command::new("ssh-keygen").args(["-Y", "sign", "-n", namespace, "-f"]).arg(key).arg(file));
}

/// Writes into `release` the archive holding `program`, its checksum file, and the checksum file's signature by `key`.
fn make_release(dir: &Path, release: &Path, key: &Path, program: &str) {
    let stage = dir.join("stage");
    let _ = fs::remove_dir_all(&stage);
    fs::create_dir_all(stage.join(base())).unwrap();
    let bin = stage.join(base()).join("fabrewk");
    fs::write(&bin, program).unwrap();
    fs::set_permissions(&bin, fs::Permissions::from_mode(0o755)).unwrap();
    fs::write(stage.join(base()).join("NOTICE"), "notice\n").unwrap();
    fs::create_dir_all(release).unwrap();
    let archive = release.join(format!("{}.tar.gz", base()));
    run_quiet(Command::new("tar").arg("-czf").arg(&archive).arg("-C").arg(&stage).arg(base()));
    let sums = Command::new("sha256sum").arg(format!("{}.tar.gz", base())).current_dir(release).output().unwrap();
    fs::write(release.join("SHA256SUMS"), sums.stdout).unwrap();
    sign(key, "fabrewk-release", &release.join("SHA256SUMS"));
}

/// The script with its address and its signers replaced, once each, by the fixture's.
fn script_copy(to: &Path, url: &str, signers: &str) {
    let text = script_text();
    let mut replaced = (0, 0);
    let lines: Vec<String> = text
        .lines()
        .map(|l| {
            if l.starts_with("release_url=") {
                replaced.0 += 1;
                format!("release_url={url}")
            } else if l.starts_with("allowed_signers=") {
                replaced.1 += 1;
                format!("allowed_signers='{signers}'")
            } else {
                l.to_owned()
            }
        })
        .collect();
    assert_eq!(replaced, (1, 1));
    fs::write(to, lines.join("\n") + "\n").unwrap();
}

fn signers_line(key: &Path, principal: &str) -> String {
    let public = fs::read_to_string(key.with_extension("pub")).unwrap();
    format!("{principal} namespaces=\"fabrewk-release\" {}", public.trim())
}

fn fixture(name: &str) -> Script {
    let dir = tmp(&format!("script-{name}"));
    let (home, tmpdir, work, release) = (dir.join("home"), dir.join("tmp"), dir.join("work"), dir.join("release"));
    for d in [&home, &tmpdir, &work] {
        fs::create_dir_all(d).unwrap();
    }
    let key = key(&dir, "fixture");
    make_release(&dir, &release, &key, PROGRAM);
    let script = dir.join("install.sh");
    script_copy(&script, &format!("file://{}", release.display()), &signers_line(&key, "fixture"));
    Script { dir, home, tmpdir, work, release, key, script }
}

/// A directory with `uname` answering as `os` and `arch` would.
fn uname(dir: &Path, os: &str, arch: &str) -> PathBuf {
    let bin = dir.join(format!("uname-{os}-{arch}"));
    fs::create_dir_all(&bin).unwrap();
    let program = format!("#!/bin/sh\ncase \"$1\" in\n-s) echo '{os}' ;;\n-m) echo '{arch}' ;;\nesac\n");
    fs::write(bin.join("uname"), program).unwrap();
    fs::set_permissions(bin.join("uname"), fs::Permissions::from_mode(0o755)).unwrap();
    bin
}

#[derive(Debug)]
struct Ran {
    status: i32,
    stdout: String,
    stderr: String,
}

impl Script {
    /// The environment of a run: a `PATH` with the fake `uname` of Linux on ARM64 first and then `path` (this
    /// process's when none), the fixture's home and temporary directory, and no repository above the working directory.
    fn command(&self, program: &str, path: Option<std::ffi::OsString>, env: &[(&str, &str)]) -> Command {
        self.command_on(("Linux", "aarch64"), program, path, env)
    }

    fn command_on(
        &self,
        (os, arch): (&str, &str),
        program: &str,
        path: Option<std::ffi::OsString>,
        env: &[(&str, &str)],
    ) -> Command {
        let fake = uname(&self.dir, os, arch);
        let path = path.unwrap_or_else(|| std::env::var_os("PATH").unwrap());
        let mut joined = fake.into_os_string();
        joined.push(":");
        joined.push(path);
        let mut c = Command::new(program);
        c.env_clear()
            .env("PATH", joined)
            .env("HOME", &self.home)
            .env("TMPDIR", &self.tmpdir)
            .env("LC_ALL", "C")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", self.home.join(".gitconfig"))
            .env("GIT_CEILING_DIRECTORIES", &self.dir)
            .envs(env.iter().copied())
            .current_dir(&self.work);
        c
    }

    /// `dash` runs the script with `args`, in a session of its own.
    fn run(&self, args: &[&str]) -> Ran {
        self.run_with(args, None, &[])
    }

    fn run_with(&self, args: &[&str], path: Option<std::ffi::OsString>, env: &[(&str, &str)]) -> Ran {
        self.run_on(("Linux", "aarch64"), args, path, env)
    }

    fn run_on(
        &self,
        platform: (&str, &str),
        args: &[&str],
        path: Option<std::ffi::OsString>,
        env: &[(&str, &str)],
    ) -> Ran {
        let mut c = self.command_on(platform, "setsid", path, env);
        c.args(["-w", "dash"]).arg(&self.script).args(args).stdin(Stdio::null());
        let out = c.output().unwrap();
        Ran {
            status: out.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&out.stdout).into(),
            stderr: String::from_utf8_lossy(&out.stderr).into(),
        }
    }

    /// The script piped into `dash`, as `curl … | sh` runs it.
    fn piped(&self) -> Ran {
        let mut c = self.command("setsid", None, &[]);
        let mut child =
            c.args(["-w", "dash"]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        child.stdin.take().unwrap().write_all(&fs::read(&self.script).unwrap()).unwrap();
        let out = child.wait_with_output().unwrap();
        Ran {
            status: out.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&out.stdout).into(),
            stderr: String::from_utf8_lossy(&out.stderr).into(),
        }
    }

    /// The script on a pseudo-terminal, its controlling terminal, `typed` typed and then the end of input.
    fn on_terminal(&self, typed: &str, env: &[(&str, &str)]) -> Ran {
        self.on_terminal_in(&self.work, typed, env)
    }

    fn on_terminal_in(&self, dir: &Path, typed: &str, env: &[(&str, &str)]) -> Ran {
        let mut c = self.command("python3", None, env);
        c.current_dir(dir);
        c.args(["-I", "-c", DRIVE, "/usr/bin/env", "dash"]).arg(&self.script);
        let mut child = c.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        child.stdin.take().unwrap().write_all(typed.as_bytes()).unwrap();
        let out = child.wait_with_output().unwrap();
        Ran {
            status: out.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&out.stdout).replace("\r\n", "\n"),
            stderr: String::from_utf8_lossy(&out.stderr).into(),
        }
    }

    fn installed(&self) -> PathBuf {
        self.home.join(".local/bin/fabrewk")
    }

    /// Every path under the home, and under the temporary directory, relative to each.
    fn written(&self) -> (BTreeSet<String>, BTreeSet<String>) {
        (tree(&self.home), tree(&self.tmpdir))
    }
}

fn tree(root: &Path) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let mut dirs = vec![root.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            found.insert(path.strip_prefix(root).unwrap().display().to_string());
            if path.is_dir() && !path.is_symlink() {
                dirs.push(path);
            }
        }
    }
    found
}

fn set(paths: &[&str]) -> BTreeSet<String> {
    paths.iter().map(|p| p.to_string()).collect()
}

#[test]
fn req_ins_6_the_script_installs_exactly_one_file_from_a_verified_release() {
    let s = fixture("installs");
    let ran = s.run(&[]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(s.written(), (set(&[".local", ".local/bin", ".local/bin/fabrewk"]), set(&[])), "{ran:?}");
    let program = fs::read_to_string(s.installed()).unwrap();
    assert_eq!(program, PROGRAM);
    assert_eq!(fs::metadata(s.installed()).unwrap().permissions().mode() & 0o7777, 0o755);
    for said in [
        "the signature of SHA256SUMS is fixture's",
        "the checksum of fabrewk-",
        "installed ",
        ": fabrewk fixture",
        "fish_add_path ~/.local/bin",
        "export PATH=\"$HOME/.local/bin:$PATH\"",
        "inside a Git repository, run fabrewk onboard to bring Fabrewk into it.",
    ] {
        assert!(ran.stdout.contains(said), "{said}: {ran:?}");
    }
    assert!(!ran.stdout.contains("fixture ran"), "{ran:?}");
    // Piped into the shell, as `curl … | sh` runs it, with the directory on the path: the same one file, replaced
    // through a temporary file in that directory, and no word on the path.
    fs::write(s.installed(), "an older one").unwrap();
    let bin = s.home.join(".local/bin").into_os_string();
    let mut path = bin.clone();
    path.push(":");
    path.push(std::env::var_os("PATH").unwrap());
    let ran = s.run_with(&[], Some(path), &[]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert!(!ran.stdout.contains("fish_add_path"), "{ran:?}");
    let ran = s.piped();
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(fs::read_to_string(s.installed()).unwrap(), PROGRAM);
    assert_eq!(s.written(), (set(&[".local", ".local/bin", ".local/bin/fabrewk"]), set(&[])));
}

#[test]
fn req_ins_6_a_changed_binary_checksum_file_or_signature_installs_nothing() {
    let s = fixture("changed");
    let sums = s.release.join("SHA256SUMS");
    let archive = s.release.join(format!("{}.tar.gz", base()));
    let other = key(&s.dir, "other");
    type Change<'a> = (&'a str, Box<dyn Fn() + 'a>, &'a str);
    let changes: Vec<Change> = vec![
        (
            "a binary changed after signing",
            Box::new(|| {
                let (signed, sig) = (fs::read(&sums).unwrap(), fs::read(sums.with_extension("sig")).unwrap());
                make_release(&s.dir, &s.release, &s.key, "#!/bin/sh\necho changed\n");
                fs::write(&sums, signed).unwrap();
                fs::write(sums.with_extension("sig"), sig).unwrap();
            }),
            "the checksum of fabrewk-",
        ),
        (
            "a checksum file changed after signing",
            Box::new(|| {
                let sig = fs::read(sums.with_extension("sig")).unwrap();
                make_release(&s.dir, &s.release, &s.key, "#!/bin/sh\necho changed\n");
                fs::write(sums.with_extension("sig"), sig).unwrap();
            }),
            "the signature of SHA256SUMS is not valid",
        ),
        ("a signature by another key", Box::new(|| sign(&other, "fabrewk-release", &sums)), "is not valid"),
        ("a signature in another namespace", Box::new(|| sign(&s.key, "file", &sums)), "is not valid"),
        ("no signature", Box::new(|| fs::remove_file(sums.with_extension("sig")).unwrap()), "SHA256SUMS.sig failed"),
        (
            "the archive listed twice",
            Box::new(|| {
                let line = fs::read_to_string(&sums).unwrap();
                fs::write(&sums, format!("{line}{line}")).unwrap();
                sign(&s.key, "fabrewk-release", &sums);
            }),
            "does not list fabrewk-",
        ),
        (
            "the archive not listed",
            Box::new(|| {
                fs::write(&sums, format!("{}  other.tar.gz\n", "0".repeat(64))).unwrap();
                sign(&s.key, "fabrewk-release", &sums);
            }),
            "does not list fabrewk-",
        ),
        (
            "an archive with no program",
            Box::new(|| {
                let stage = s.dir.join("empty");
                fs::create_dir_all(stage.join(base())).unwrap();
                run_quiet(Command::new("tar").arg("-czf").arg(&archive).arg("-C").arg(&stage).arg(base()));
                let line = Command::new("sha256sum").arg(archive.file_name().unwrap()).current_dir(&s.release).output();
                fs::write(&sums, line.unwrap().stdout).unwrap();
                sign(&s.key, "fabrewk-release", &sums);
            }),
            "the archive holds no program fabrewk",
        ),
    ];
    // What the home held before, an earlier install among it, stays as it was.
    fs::create_dir_all(s.home.join(".local/bin")).unwrap();
    fs::write(s.installed(), "the earlier one").unwrap();
    for (what, change, why) in changes {
        make_release(&s.dir, &s.release, &s.key, PROGRAM);
        change();
        let ran = s.run(&[]);
        assert_eq!(ran.status, 1, "{what}: {ran:?}");
        assert!(ran.stderr.contains(why) && ran.stderr.contains("nothing was installed"), "{what}: {ran:?}");
        assert_eq!(fs::read_to_string(s.installed()).unwrap(), "the earlier one", "{what}");
        assert_eq!(s.written(), (set(&[".local", ".local/bin", ".local/bin/fabrewk"]), set(&[])), "{what}");
    }
    // A directory where the program goes is refused, and nothing is left beside it.
    make_release(&s.dir, &s.release, &s.key, PROGRAM);
    fs::remove_file(s.installed()).unwrap();
    fs::create_dir(s.installed()).unwrap();
    let ran = s.run(&[]);
    assert_eq!(ran.status, 1, "{ran:?}");
    assert!(ran.stderr.contains("is a directory; nothing was installed"), "{ran:?}");
    assert_eq!(s.written(), (set(&[".local", ".local/bin", ".local/bin/fabrewk"]), set(&[])));
}

#[test]
fn req_ins_7_an_unsupported_platform_is_refused_with_the_supported_list() {
    let s = fixture("platform");
    fs::remove_dir_all(&s.release).unwrap();
    for (os, arch) in [("Linux", "x86_64"), ("Darwin", "arm64"), ("Darwin", "x86_64"), ("MINGW64_NT-10.0", "x86_64")] {
        let ran = s.run_on((os, arch), &[], None, &[]);
        assert_eq!(ran.status, 1, "{ran:?}");
        let said = format!("PLATFORM_UNSUPPORTED: {os} on {arch} is not supported yet; supported: linux-aarch64.\n");
        assert_eq!((ran.stdout.as_str(), ran.stderr.as_str()), ("", said.as_str()));
        assert_eq!(s.written(), (set(&[]), set(&[])));
    }
}

#[test]
fn req_ins_6_the_script_takes_no_option_and_names_its_version_source_and_signers() {
    let s = fixture("options");
    for args in [&["--no-verify"][..], &["--insecure"], &["-k"], &["0.0.1"], &[""]] {
        let ran = s.run(args);
        assert_eq!(ran.status, 2, "{args:?}: {ran:?}");
        assert!(ran.stderr.starts_with("usage: sh install.sh\nIt takes no option;"), "{ran:?}");
        assert_eq!(s.written(), (set(&[]), set(&[])));
    }
    let text = script_text();
    let code: Vec<&str> = text.lines().filter(|l| !l.trim_start().starts_with('#')).collect();
    assert!(code.iter().all(|l| !l.contains("sudo") && !l.contains("--insecure") && !l.contains(" -k ")));
    assert!(code.contains(&format!("version={VERSION}").as_str()), "{text}");
    let url = format!("release_url=https://github.com/fabrewk/fabrewk/releases/download/v{VERSION}");
    assert!(code.contains(&url.as_str()), "{text}");
    let signers: Vec<&&str> = code.iter().filter(|l| l.starts_with("allowed_signers=")).collect();
    assert_eq!(signers.len(), 1);
    assert!(signers[0].starts_with("allowed_signers='xel namespaces=\"fabrewk-release\" ssh-ed25519 "), "{text}");
    // `curl` keeps to the address's own protocol, redirections included.
    assert!(text.contains("curl --proto \"=$proto\" --proto-redir \"=$proto\" --tlsv1.2 --fail"), "{text}");
}

#[test]
fn req_ins_6_a_missing_tool_stops_and_says_how_to_get_it() {
    let s = fixture("tools");
    let all = ["curl", "ssh-keygen", "sha256sum", "tar", "gzip", "mktemp", "awk", "rm", "cp", "chmod", "mv", "mkdir"];
    for missing in ["curl", "ssh-keygen", "sha256sum", "tar", "gzip"] {
        let base = s.dir.join(format!("path-{missing}"));
        let programs: Vec<&str> = all.iter().copied().filter(|p| *p != missing).chain(["setsid", "dash"]).collect();
        let path = path_with(&base, &programs);
        let ran = s.run_with(&[], Some(path), &[]);
        assert_eq!(ran.status, 1, "{missing}: {ran:?}");
        let said = format!("TOOL_MISSING: {missing} is not installed; install it with your system's package manager");
        assert!(ran.stderr.starts_with(&said) && ran.stderr.contains("It installs no tool by itself."), "{ran:?}");
        assert_eq!(s.written(), (set(&[]), set(&[])));
    }
    // With every tool and no `git`, outside a repository: installed.
    let path = path_with(&s.dir.join("path-all"), &[&all[..], &["setsid", "dash"]].concat());
    let ran = s.run_with(&[], Some(path), &[]);
    assert_eq!(ran.status, 0, "{ran:?}");
}

/// The adapters' markers and the extensions', as the core refuses them (REQ-SEC-7).
fn markers() -> Vec<&'static str> {
    let mut m: Vec<&str> = [fabrewk_adapter_claude::ADAPTER.markers, fabrewk_adapter_codex::ADAPTER.markers].concat();
    m.push(fabrewk_core::cli::EXTENSION_MARKER);
    m
}

#[test]
fn req_ins_6_onboard_is_offered_inside_a_repository_with_a_terminal_and_never_inside_an_agent() {
    let s = fixture("onboard");
    run_quiet(Command::new("git").args(["init", "-q"]).arg(&s.work));
    let question = "Run fabrewk onboard in ";
    // A yes runs `fabrewk onboard` itself, in the repository; any other answer, or none, runs nothing.
    let ran = s.on_terminal("y\n", &[]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert!(ran.stdout.contains(&format!("{question}{} now?", s.work.display())), "{ran:?}");
    // The terminal echoes what is typed as it is typed, before the question.
    assert!(ran.stdout.ends_with(&format!("[y/N] fixture ran in {}: onboard\n", s.work.display())), "{ran:?}");
    // From a directory inside the repository, `onboard` runs at its root, where it is asked.
    let inside = s.work.join("inside");
    fs::create_dir_all(&inside).unwrap();
    let ran = s.on_terminal_in(&inside, "y\n", &[]);
    assert!(ran.stdout.contains(&format!("{question}{} now?", s.work.display())), "{ran:?}");
    assert!(ran.stdout.ends_with(&format!("[y/N] fixture ran in {}: onboard\n", s.work.display())), "{ran:?}");
    for typed in ["n\n", "\n", "", "yes please\n", "sí\n"] {
        let ran = s.on_terminal(typed, &[]);
        assert_eq!(ran.status, 0, "{typed:?}: {ran:?}");
        assert!(ran.stdout.contains(question) && !ran.stdout.contains("fixture ran"), "{typed:?}: {ran:?}");
        assert!(ran.stdout.contains("when you want it, run fabrewk onboard inside the repository."), "{ran:?}");
    }
    // Inside an agent, with a terminal: nothing is asked, and the agent's route goes on with `assess`.
    let markers = markers();
    assert_eq!(
        markers,
        ["CLAUDECODE", "CODEX_SANDBOX", "CODEX_SANDBOX_NETWORK_DISABLED", "CODEX_THREAD_ID", "FABREWK_EXTENSION"]
    );
    let text = script_text();
    assert!(text.contains(&format!("for marker in {}; do", markers.join(" "))), "{text}");
    for marker in markers {
        for value in ["1", ""] {
            let ran = s.on_terminal("y\n", &[(marker, value)]);
            assert_eq!(ran.status, 0, "{marker}: {ran:?}");
            assert!(!ran.stdout.contains(question) && !ran.stdout.contains("fixture ran"), "{marker}: {ran:?}");
            let said = format!("{marker} is set, so this runs inside an agent: the next step is fabrewk assess");
            assert!(ran.stdout.contains(&said), "{ran:?}");
        }
    }
    // In a repository with no terminal, nothing is asked.
    let ran = s.run(&[]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert!(!ran.stdout.contains(question) && !ran.stdout.contains("fixture ran"), "{ran:?}");
    assert!(ran.stdout.contains("inside a Git repository, run fabrewk onboard"), "{ran:?}");
    // Outside a repository, with a terminal, nothing is asked either.
    fs::remove_dir_all(s.work.join(".git")).unwrap();
    let ran = s.on_terminal("y\n", &[]);
    assert!(!ran.stdout.contains(question) && !ran.stdout.contains("fixture ran"), "{ran:?}");
    assert_eq!(s.written().0, set(&[".local", ".local/bin", ".local/bin/fabrewk"]));
}

/// Each README (point 11): one ` ```text ` block, the agent's preset prompt, whose commands are its lines indented
/// by four spaces and whose last command is `fabrewk assess`, never `fabrewk apply`; the script's allowed-signers line
/// as the script writes it; the script's commands as the manual writes them; and every relative link to a file that
/// exists.
#[test]
fn req_ins_6_the_agent_route_ends_at_assess_and_the_readmes_name_the_script_s_signers() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let text = script_text();
    let signers = text.lines().find(|l| l.starts_with("allowed_signers=")).unwrap();
    let signers = &signers["allowed_signers='".len()..signers.len() - 1];
    for (readme, manual) in
        [("README.md", "docs/en/installation.md"), ("README.es-MX.md", "docs/es-MX/installation.md")]
    {
        let doc = fs::read_to_string(root.join(readme)).unwrap();
        assert_eq!(doc.matches("```text\n").count(), 1, "{readme}");
        let prompt = &doc[doc.find("```text\n").unwrap() + 8..];
        let prompt = &prompt[..prompt.find("\n```").unwrap()];
        let commands: Vec<&str> =
            prompt.lines().filter_map(|l| l.strip_prefix("    ")).filter(|l| !l.starts_with(' ')).collect();
        assert!(commands.len() >= 2, "{readme}: {prompt}");
        assert!(commands.last().unwrap().starts_with("fabrewk assess "), "{readme}: {commands:?}");
        assert!(commands.iter().all(|c| !c.starts_with("fabrewk apply")), "{readme}: {commands:?}");
        assert!(commands.iter().all(|c| !c.contains("| sh")), "{readme}: the agent never pipes the script");
        assert!(doc.contains(&format!("\n{signers}\n")), "{readme}");
        let guide = fs::read_to_string(root.join(manual)).unwrap();
        assert!(guide.contains(&format!("\n    {signers}\n")), "{manual}");
        for fetch in [
            format!(
                "curl --proto '=https' --tlsv1.2 -fsSLO https://raw.githubusercontent.com/fabrewk/fabrewk/v{VERSION}/install.sh"
            ),
            format!(
                "curl --proto '=https' --tlsv1.2 -fsSL https://raw.githubusercontent.com/fabrewk/fabrewk/v{VERSION}/install.sh | sh"
            ),
        ] {
            assert!(doc.contains(&format!("\n{fetch}\n")), "{readme}: {fetch}");
            assert!(guide.contains(&format!("\n    {fetch}\n")), "{manual}: {fetch}");
        }
        for link in doc.split("](").skip(1).map(|l| &l[..l.find(')').unwrap()]) {
            let file = link.split('#').next().unwrap();
            if !link.contains("://") && !file.is_empty() {
                assert!(root.join(file).is_file(), "{readme}: {link}");
            }
        }
    }
    assert!(text.contains(&format!("raw.githubusercontent.com/fabrewk/fabrewk/v{VERSION}/install.sh | sh")));
    // The manuals' relative links reach files that exist, from their own directory.
    for lang in ["en", "es-MX"] {
        for page in ["installation.md", "working.md", "commands.md"] {
            let dir = root.join("docs").join(lang);
            let doc = fs::read_to_string(dir.join(page)).unwrap();
            for link in doc.split("](").skip(1).map(|l| &l[..l.find(')').unwrap()]) {
                let file = link.split('#').next().unwrap();
                if !link.contains("://") && !file.is_empty() {
                    assert!(dir.join(file).is_file(), "docs/{lang}/{page}: {link}");
                }
            }
        }
    }
}

/// A directory holding `name`, a program that runs `body`, first on a `PATH` of this process's.
fn wrapper(dir: &Path, name: &str, body: &str) -> std::ffi::OsString {
    let bin = dir.join(format!("wrap-{name}"));
    fs::create_dir_all(&bin).unwrap();
    fs::write(bin.join(name), format!("#!/bin/sh\n{body}\n")).unwrap();
    fs::set_permissions(bin.join(name), fs::Permissions::from_mode(0o755)).unwrap();
    let mut path = bin.into_os_string();
    path.push(":");
    path.push(std::env::var_os("PATH").unwrap());
    path
}

fn real(program: &str) -> PathBuf {
    std::env::split_paths(&std::env::var_os("PATH").unwrap()).map(|d| d.join(program)).find(|f| f.is_file()).unwrap()
}

#[test]
fn req_ins_6_a_signal_or_a_closed_output_leaves_nothing_behind() {
    let s = fixture("signals");
    // A signal right after the first temporary directory is made, before its name is kept (P10D-2), and right after
    // the program's temporary file is made beside it.
    let mktemp = real("mktemp");
    for (when, body) in [
        ("the first", format!("{} \"$@\"\nkill -TERM \"$PPID\"", mktemp.display())),
        ("the second", format!("{} \"$@\"\ncase \"$1\" in -d) ;; *) kill -TERM \"$PPID\" ;; esac", mktemp.display())),
    ] {
        let ran = s.run_with(&[], Some(wrapper(&s.dir.join(when), "mktemp", &body)), &[]);
        assert_eq!(ran.status, 1, "{when}: {ran:?}");
        let (home, tmp) = s.written();
        assert!(tmp.is_empty() && !home.contains(".local/bin/fabrewk"), "{when}: {home:?} {tmp:?}");
        assert!(home.iter().all(|p| !p.contains(".fabrewk.")), "{when}: {home:?}");
    }
    // The output closed under it, as `| head -n 1` does, while it downloads (P10D-1).
    let flag = s.dir.join("closed");
    let body = format!(
        "i=0\nwhile [ ! -e '{}' ] && [ $i -lt 200 ]; do sleep 0.05; i=$((i + 1)); done\nexec {} \"$@\"",
        flag.display(),
        real("curl").display()
    );
    let mut c = s.command("setsid", Some(wrapper(&s.dir, "curl", &body)), &[]);
    let mut child = c.args(["-w", "dash"]).arg(&s.script).stdin(Stdio::null()).stdout(Stdio::piped()).spawn().unwrap();
    let mut first = String::new();
    std::io::BufRead::read_line(&mut std::io::BufReader::new(child.stdout.take().unwrap()), &mut first).unwrap();
    assert!(first.starts_with("fabrewk install: downloading Fabrewk"), "{first}");
    fs::write(&flag, "").unwrap();
    let status = child.wait().unwrap();
    assert_eq!(status.code(), Some(1), "{status:?}");
    // Only the directories the earlier cases made stay; no download, no program.
    assert_eq!(s.written(), (set(&[".local", ".local/bin"]), set(&[])));
}

#[test]
fn req_ins_6_onboard_runs_at_the_exact_top_of_a_repository_whose_name_ends_in_a_line_end() {
    let s = fixture("line-end");
    let (odd, sibling) = (s.work.join("project\n"), s.work.join("project"));
    for d in [&odd, &sibling] {
        run_quiet(Command::new("git").args(["init", "-q"]).arg(d));
    }
    let record = s.dir.join("cwd");
    let ran = s.on_terminal_in(&odd, "y\n", &[("FIXTURE_CWD", record.to_str().unwrap())]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert!(ran.stdout.contains(&format!("Run fabrewk onboard in {} now?", odd.display())), "{ran:?}");
    assert_eq!(fs::read(&record).unwrap(), odd.as_os_str().as_encoded_bytes());
}

#[test]
fn req_ins_6_a_temporary_directory_whose_name_holds_a_backslash_is_used_as_it_is() {
    let s = fixture("backslash");
    let odd = s.dir.join("tmp=\\b \\n'x");
    fs::create_dir_all(&odd).unwrap();
    let ran = s.run_with(&[], None, &[("TMPDIR", odd.to_str().unwrap())]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(fs::read_to_string(s.installed()).unwrap(), PROGRAM);
    assert_eq!(fs::read_dir(&odd).unwrap().count(), 0);
}

#[test]
fn req_ins_6_a_home_or_temporary_directory_that_is_not_absolute_is_refused() {
    let s = fixture("relative");
    for dir in ["-h", "home", "-t", "tmp=path"] {
        fs::create_dir_all(s.work.join(dir)).unwrap();
    }
    for (var, value, said) in [
        ("HOME", "-h", "HOME is not an absolute path"),
        ("HOME", "home", "HOME is not an absolute path"),
        ("HOME", "", "HOME is not an absolute path"),
        ("TMPDIR", "-t", "TMPDIR is not an absolute path"),
        ("TMPDIR", "tmp=path", "TMPDIR is not an absolute path"),
    ] {
        let ran = s.run_with(&[], None, &[(var, value)]);
        assert_eq!(ran.status, 1, "{var}={value}: {ran:?}");
        assert!(ran.stderr.starts_with(said), "{var}={value}: {ran:?}");
        let made = tree(&s.work);
        assert_eq!(made, set(&["-h", "home", "-t", "tmp=path"]), "{var}={value}");
        assert_eq!(s.written(), (set(&[]), set(&[])));
    }
}

/// As [`DRIVE`], but the program's standard input is a pipe holding `STDIN_TEXT`, apart from its terminal.
const DRIVE_APART: &str = r#"
import os, pty, sys
typed = sys.stdin.buffer.read()
r, w = os.pipe()
pid, fd = pty.fork()
if pid == 0:
    os.close(w)
    os.dup2(r, 0)
    os.execv(sys.argv[1], sys.argv[1:])
os.close(r)
os.write(w, os.environ["STDIN_TEXT"].encode())
os.close(w)
os.write(fd, typed + b"\x04")
out = b""
while True:
    try:
        chunk = os.read(fd, 65536)
    except OSError:
        break
    if not chunk:
        break
    out += chunk
_, status = os.waitpid(pid, 0)
sys.stdout.buffer.write(out)
sys.exit(os.waitstatus_to_exitcode(status))
"#;

#[test]
fn req_ins_6_the_offer_is_answered_on_the_terminal_never_on_standard_input() {
    let s = fixture("apart");
    run_quiet(Command::new("git").args(["init", "-q"]).arg(&s.work));
    for (terminal, input, runs) in [("n\n", "y\n", false), ("y\n", "n\n", true)] {
        let mut c = s.command("python3", None, &[("STDIN_TEXT", input)]);
        c.args(["-I", "-c", DRIVE_APART, "/usr/bin/env", "dash"]).arg(&s.script);
        let mut child = c.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        child.stdin.take().unwrap().write_all(terminal.as_bytes()).unwrap();
        let out = child.wait_with_output().unwrap();
        let shown = String::from_utf8_lossy(&out.stdout).replace("\r\n", "\n");
        assert_eq!(out.status.code(), Some(0), "{shown}");
        assert!(shown.contains("Run fabrewk onboard in "), "{shown}");
        assert_eq!(shown.contains("fixture ran in "), runs, "terminal {terminal:?}, input {input:?}: {shown}");
    }
}
