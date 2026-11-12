//! The attempt's copy and the candidate (REQ-RUN-3, REQ-RUN-4, REQ-RUN-10, REQ-SEC-3 in a change set, REQ-GATE-9's
//! names and hooks directory, S-2, S-3, S-4, S-11; part 9b), at the library's level: a fake agent is the test, which
//! writes in the copy what an agent's cell would leave there. Each test runs isolated, with a home of its own.

mod common;

use common::*;
use fabrewk_core::Code;
use fabrewk_core::candidate::{self, Agent, Change, Entry, Lanes, Mode};
use fabrewk_core::confine::Scratch;
use fabrewk_core::record::Repo;
use fabrewk_core::settings::{self, Def, Effective, Source, Val};
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};

/// The default protected names (D-076), as `gate.protectedNames` declares them.
const NAMES: [&str; 7] = [".gitattributes", ".gitmodules", "CLAUDE.md", ".claude", "AGENTS.md", ".codex", ".agents"];

const AGENT: Agent = Agent { tool: "fake-agent", provider: "nowhere", model: "fake-1" };

/// A repository at `<tmp>/repo` whose first commit holds `files` (a path and its bytes) and the links `links` (a path
/// and its target); gives it, its base commit, and a run's directories in its domain.
struct Base {
    root: PathBuf,
    base: String,
    scratch: Scratch,
}

impl Base {
    fn new(name: &str, files: &[(&[u8], &[u8])], links: &[(&str, &str)]) -> Base {
        let root = tmp(name).join("repo");
        fs::create_dir_all(&root).unwrap();
        git(&root, &["init", "-q", "-b", "main"]);
        for (k, v) in [("user.name", "Fixture"), ("user.email", "fixture@example.invalid"), ("commit.gpgsign", "false")]
        {
            git(&root, &["config", k, v]);
        }
        fs::write(root.join("README.md"), "fixture\n").unwrap();
        for (path, bytes) in files {
            let at = root.join(OsStr::from_bytes(path));
            fs::create_dir_all(at.parent().unwrap()).unwrap();
            fs::write(&at, bytes).unwrap();
        }
        for (path, target) in links {
            let at = root.join(path);
            fs::create_dir_all(at.parent().unwrap()).unwrap();
            symlink(target, &at).unwrap();
        }
        git(&root, &["add", "-A", "-f"]);
        git(&root, &["-c", "core.hooksPath=/dev/null", "commit", "-q", "-m", "base"]);
        let base = git(&root, &["rev-parse", "HEAD"]);
        let scratch = Scratch::create(&root.join(".brewstillery"), "t-00000001-1").unwrap();
        Base { root, base, scratch }
    }

    fn repo(&self) -> Repo {
        Repo { root: self.root.clone(), prefix: PathBuf::new() }
    }

    fn work(&self) -> &Path {
        &self.scratch.work
    }

    fn export(&self) {
        candidate::export(&self.repo(), &self.base, self.work()).unwrap();
    }

    fn put(&self, path: &str, bytes: &[u8]) {
        let at = self.work().join(path);
        fs::create_dir_all(at.parent().unwrap()).unwrap();
        fs::write(at, bytes).unwrap();
    }

    fn changes(&self, allowed: &[&str]) -> Result<Vec<Change>, fabrewk_core::Error> {
        self.changes_with(allowed, &[])
    }

    fn changes_with(&self, allowed: &[&str], paths: &[String]) -> Result<Vec<Change>, fabrewk_core::Error> {
        let names: Vec<String> = NAMES.iter().map(|n| n.to_string()).collect();
        let lanes = Lanes { allowed, names: &names, paths };
        candidate::changes(&self.repo(), &self.base, self.work(), &self.scratch.root, &lanes)
    }

    fn commit(&self, changes: &[Change], sign_off: bool, previous: Option<&str>) -> candidate::Candidate {
        candidate::commit(
            &self.repo(),
            &self.base,
            changes,
            self.work(),
            &self.scratch.root,
            "t-00000001",
            &AGENT,
            sign_off,
            previous,
        )
        .unwrap()
    }

    fn blob(&self, commit: &str, path: &str) -> Vec<u8> {
        let out = std::process::Command::new("git")
            .current_dir(&self.root)
            .args(["cat-file", "blob", &format!("{commit}:{path}")])
            .output()
            .unwrap();
        assert!(out.status.success(), "{path} is not in {commit}");
        out.stdout
    }
}

fn refused<T: std::fmt::Debug>(r: Result<T, fabrewk_core::Error>, code: Code) -> String {
    let e = r.expect_err("a refusal");
    assert_eq!(e.code(), Some(code), "{e:?}");
    e.detail()
}

fn paths(changes: &[Change]) -> Vec<&str> {
    changes.iter().map(|c| c.path.as_str()).collect()
}

/// Every path under `dir`, as bytes relative to it, a link's own path included and never followed.
fn listing(dir: &Path) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for e in fs::read_dir(dir).unwrap() {
        let path = e.unwrap().path();
        let rel = path.strip_prefix(dir).unwrap().as_os_str().as_bytes().to_vec();
        if fs::symlink_metadata(&path).unwrap().is_dir() {
            out.extend(listing(&path).into_iter().map(|p| [&rel[..], b"/", &p[..]].concat()));
        }
        out.push(rel);
    }
    out.sort();
    out
}

#[test]
fn pd_22_the_export_holds_neither_git_nor_the_domain_at_any_depth() {
    isolated("pd_22_the_export_holds_neither_git_nor_the_domain_at_any_depth", || {
        let marker = b"MARKER-9b-settings";
        let files: &[(&[u8], &[u8])] = &[
            (b".brewstillery/settings.toml", marker),
            (b".brewstillery/xel/fermento.md", b"MARKER-9b-fermento"),
            (b"src/.brewstillery/x", b"MARKER-9b-nested"),
            (b"docs/.BrewStillery/y", b"MARKER-9b-folded"),
            (b"src/main.rs", b"fn main() {}\n"),
        ];
        let b = Base::new("export", files, &[]);
        // A submodule's entry, which no copy can fill.
        let sub = git(&b.root, &["rev-parse", "HEAD"]);
        git(&b.root, &["update-index", "--add", "--cacheinfo", &format!("160000,{sub},vendor/sub")]);
        git(&b.root, &["-c", "core.hooksPath=/dev/null", "commit", "-q", "-m", "sub"]);
        let b = Base { base: git(&b.root, &["rev-parse", "HEAD"]), ..b };
        b.export();
        let all = listing(b.work());
        assert_eq!(all, [&b"README.md"[..], b"src", b"src/main.rs", b"vendor", b"vendor/sub"].map(<[u8]>::to_vec));
        for p in &all {
            let at = b.work().join(OsStr::from_bytes(p));
            if fs::symlink_metadata(&at).unwrap().is_file() {
                assert!(!fs::read(&at).unwrap().windows(9).any(|w| w == b"MARKER-9b"), "{p:?}");
            }
        }
        // Untouched, the copy is no change, the submodule's empty directory included.
        assert_eq!(b.changes(&["**"]).unwrap(), vec![]);
    });
}

#[test]
fn pd_19_pd_21_links_are_copied_as_links_and_an_untouched_one_is_no_change() {
    isolated("pd_19_pd_21_links_are_copied_as_links_and_an_untouched_one_is_no_change", || {
        let links = [("rel", "README.md"), ("abs", "/etc/passwd"), ("missing", "nowhere/at/all"), ("dir", "src")];
        let b = Base::new("links", &[(b"src/a.rs", b"a\n")], &links);
        b.export();
        for (path, target) in links {
            let at = b.work().join(path);
            assert!(fs::symlink_metadata(&at).unwrap().file_type().is_symlink(), "{path}");
            assert_eq!(fs::read_link(&at).unwrap(), Path::new(target));
        }
        assert_eq!(b.changes(&["src/**"]).unwrap(), vec![]);
        // A new link inside the lane is a change, its target as written; outside it, a violation.
        symlink("../../etc/shadow", b.work().join("src/new")).unwrap();
        let changes = b.changes(&["src/**"]).unwrap();
        assert_eq!(paths(&changes), ["src/new"]);
        assert_eq!(changes[0].after.as_ref().map(|e| e.mode), Some(Mode::Link));
        let c = b.commit(&changes, false, None);
        assert_eq!(b.blob(&c.commit, "src/new"), b"../../etc/shadow");
        symlink("/etc", b.work().join("out")).unwrap();
        let why = refused(b.changes(&["src/**"]), Code::LaneViolation);
        assert!(why.contains("\"out\""), "{why}");
        // A tracked link changed, or replaced by a file, is a change too.
        fs::remove_file(b.work().join("out")).unwrap();
        fs::remove_file(b.work().join("rel")).unwrap();
        symlink("src/a.rs", b.work().join("rel")).unwrap();
        fs::remove_file(b.work().join("abs")).unwrap();
        fs::write(b.work().join("abs"), "now a file\n").unwrap();
        let changes = b.changes(&["src/**", "rel", "abs"]).unwrap();
        assert_eq!(paths(&changes), ["abs", "rel", "src/new"]);
        assert_eq!(changes[0].before.as_ref().map(|e| e.mode), Some(Mode::Link));
        assert_eq!(changes[0].after.as_ref().map(|e| e.mode), Some(Mode::File));
    });
}

#[test]
fn pd_20_an_executable_bit_alone_is_a_change() {
    isolated("pd_20_an_executable_bit_alone_is_a_change", || {
        let b = Base::new("mode", &[(b"src/run.sh", b"echo hi\n"), (b"src/b.rs", b"b\n")], &[]);
        b.export();
        let script = b.work().join("src/run.sh");
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
        let changes = b.changes(&["src/**"]).unwrap();
        assert_eq!(paths(&changes), ["src/run.sh"]);
        let (before, after) = (changes[0].before.clone().unwrap(), changes[0].after.clone().unwrap());
        assert_eq!((before.mode, after.mode, before.oid == after.oid), (Mode::File, Mode::Executable, true));
        let c = b.commit(&changes, false, None);
        assert!(git(&b.root, &["ls-tree", &c.commit, "src/run.sh"]).starts_with("100755 blob "));
        // With another file edited, both are in the candidate.
        b.put("src/b.rs", b"b2\n");
        let changes = b.changes(&["src/**"]).unwrap();
        assert_eq!(paths(&changes), ["src/b.rs", "src/run.sh"]);
        // An executable bit taken away is a change as well.
        fs::set_permissions(&script, fs::Permissions::from_mode(0o644)).unwrap();
        assert_eq!(paths(&b.changes(&["src/**"]).unwrap()), ["src/b.rs"]);
    });
}

/// Names that a quoting ignore check once failed to match (PD-18).
const ODD: [&str; 7] = ["a space", "a\"quote", "a\\slash", "a\nline", "a\ttab", "acentó", "a\u{2028}sep"];

#[test]
fn pd_18_ignored_new_files_never_enter_and_tracked_odd_names_are_kept_byte_exact() {
    isolated("pd_18_ignored_new_files_never_enter_and_tracked_odd_names_are_kept_byte_exact", || {
        let tracked: Vec<(Vec<u8>, Vec<u8>)> =
            ODD.iter().map(|n| (format!("src/{n}.log").into_bytes(), b"old\n".to_vec())).collect();
        let mut files: Vec<(&[u8], &[u8])> = tracked.iter().map(|(p, c)| (&p[..], &c[..])).collect();
        files.push((b".gitignore", b"*.log\nout/\n"));
        let b = Base::new("ignored", &files, &[]);
        b.export();
        for n in ODD {
            b.put(&format!("src/new {n}.log"), b"ignored\n");
            b.put(&format!("out/{n}"), b"ignored\n");
            b.put(&format!("src/{n}.log"), format!("new {n}\r\n").as_bytes());
        }
        let changes = b.changes(&["src/**"]).unwrap();
        let mut want: Vec<String> = ODD.iter().map(|n| format!("src/{n}.log")).collect();
        want.sort();
        assert_eq!(paths(&changes), want);
        let c = b.commit(&changes, false, None);
        for n in ODD {
            let out = std::process::Command::new("git")
                .current_dir(&b.root)
                .args(["ls-tree", "-z", "--name-only", "-r", &c.commit])
                .output()
                .unwrap()
                .stdout;
            assert!(out.split(|&x| x == 0).any(|p| p == format!("src/{n}.log").as_bytes()), "{n:?}");
            let blob = git(&b.root, &["rev-parse", &format!("{}:src/{n}.log", c.commit)]);
            let bytes = run(&b.root, "git", &["cat-file", "blob", &blob]);
            assert_eq!(bytes, format!("new {n}\r\n"), "{n:?} kept byte-exact");
        }
    });
}

#[test]
fn s_2_the_copy_is_read_as_bytes_and_no_rule_of_its_own_applies() {
    isolated("s_2_the_copy_is_read_as_bytes_and_no_rule_of_its_own_applies", || {
        // The base asks for conversions and a filter the person's Git has configured; none applies to the candidate.
        let attributes: &[u8] = b"*.txt text eol=crlf filter=evil\n";
        let b = Base::new("bytes", &[(b".gitattributes", attributes), (b"src/old.txt", b"old\n")], &[]);
        let ran = b.root.join("filter-ran");
        let clean = format!("sh -c 'touch {}; cat'", ran.display());
        git(&b.root, &["config", "filter.evil.clean", &clean]);
        git(&b.root, &["config", "filter.evil.smudge", &clean]);
        git(&b.root, &["config", "core.autocrlf", "true"]);
        b.export();
        assert_eq!(fs::read(b.work().join("src/old.txt")).unwrap(), b"old\n", "the export writes the bytes");
        b.put("src/a.txt", b"x\ny\n");
        let changes = b.changes(&["src/**"]).unwrap();
        let c = b.commit(&changes, false, None);
        assert_eq!(b.blob(&c.commit, "src/a.txt"), b"x\ny\n");
        assert!(!ran.exists(), "a filter ran");
        // A `.gitignore` edited to hide a new file changes a file, not what Fabrewk sees.
        b.put(".gitignore", b"secret.txt\n");
        b.put("secret.txt", b"not hidden\n");
        let changes = b.changes(&["src/**", ".gitignore", "secret.txt"]).unwrap();
        assert_eq!(paths(&changes), [".gitignore", "secret.txt", "src/a.txt"]);
        // A path that is not valid text is refused.
        fs::write(b.work().join(OsStr::from_bytes(b"src/\xff.txt")), b"x").unwrap();
        let why = refused(b.changes(&["**"]), Code::LaneViolation);
        assert!(why.contains("not valid text"), "{why}");
    });
}

#[test]
fn s_2_a_pipe_or_a_socket_is_refused_unless_ignored() {
    isolated("s_2_a_pipe_or_a_socket_is_refused_unless_ignored", || {
        let b = Base::new("special", &[(b".gitignore", b"*.log\n")], &[]);
        b.export();
        let fifo = |p: &Path| run(Path::new("/"), "mkfifo", &[p.to_str().unwrap()]);
        fifo(&b.work().join("ignored.log"));
        // A socket's path must be short: it is bound from its directory (an isolated test runs alone in its process).
        std::env::set_current_dir(b.work()).unwrap();
        let _socket = std::os::unix::net::UnixListener::bind("other.log").unwrap();
        assert_eq!(b.changes(&["**"]).unwrap(), vec![], "ignored, never opened");
        fifo(&b.work().join("pipe"));
        let why = refused(b.changes(&["**"]), Code::LaneViolation);
        assert!(why.contains("\"pipe\"") && why.contains("not a file"), "{why}");
        fs::remove_file(b.work().join("pipe")).unwrap();
        let _socket = std::os::unix::net::UnixListener::bind("sock").unwrap();
        refused(b.changes(&["**"]), Code::LaneViolation);
    });
}

#[test]
fn pd_16_s_2_a_protected_name_is_refused_at_any_depth_before_git_sees_the_copy() {
    isolated("pd_16_s_2_a_protected_name_is_refused_at_any_depth_before_git_sees_the_copy", || {
        for committed in [false, true] {
            let files: &[(&[u8], &[u8])] = if committed { &[(b".brewstillery/settings.toml", b"x")] } else { &[] };
            let b = Base::new(&format!("protected-{committed}"), files, &[]);
            let names = [
                ".brewstillery",
                ".brewstillery/x",
                ".BrewStillery/x",
                "a/.git/config",
                ".GIT",
                "src/sub/.git/HEAD",
                "src/.git",
                ".git.",
                ".git ",
                "GIT~1/x",
                ".git:stream",
                "src/.g\u{200c}it",
                "src/.brewstillery\u{feff}",
                "src/a\\.git",
            ];
            for name in names {
                b.export();
                b.put(name, b"planted\n");
                let why = refused(b.changes(&["**"]), Code::ProtectedPath);
                assert!(why.contains("no candidate may hold"), "{name}: {why}");
                fs::remove_dir_all(b.work()).unwrap();
                fs::create_dir(b.work()).unwrap();
            }
            b.export();
            symlink("/", b.work().join(".brewstillery")).unwrap();
            refused(b.changes(&["**"]), Code::ProtectedPath);
            // Names that only look alike are files like any other.
            fs::remove_file(b.work().join(".brewstillery")).unwrap();
            for name in [".gitx", "git~2", ".brewstillery-old/x", "brewstillery/x", "x.git"] {
                b.put(name, b"kept\n");
            }
            assert_eq!(b.changes(&["**"]).unwrap().len(), 5);
        }
    });
}

#[test]
fn s_4_a_protected_name_needs_its_path_listed_literally() {
    isolated("s_4_a_protected_name_needs_its_path_listed_literally", || {
        let def = settings::declared("gate.protectedNames").unwrap().default;
        let listed = NAMES.map(|n| format!("{n:?}")).join(", ");
        assert!(
            matches!(def, Def::Lit(l) if l == format!("[{listed}]")),
            "{:?}",
            settings::declared("gate.protectedNames")
        );
        let b = Base::new("names", &[(b"src/lib.rs", b"x\n")], &[]);
        for path in ["src/AGENTS.md", "src/.claude/settings.json", "src/.gitattributes", "src/deep/claude.md"] {
            b.export();
            b.put(path, b"steer the next agent\n");
            let why = refused(b.changes(&["src/**"]), Code::ProtectedPathWildcard);
            assert!(why.contains(&format!("{path:?}")), "{why}");
            assert_eq!(paths(&b.changes(&["src/**", path]).unwrap()), [path], "listed literally, kept");
            fs::remove_dir_all(b.work()).unwrap();
            fs::create_dir(b.work()).unwrap();
        }
        // `*.md` and `src/**` stay valid lanes: only the change that reaches such a name is refused.
        b.export();
        b.put("src/README.md", b"fine\n");
        assert_eq!(paths(&b.changes(&["*.md", "src/**"]).unwrap()), ["src/README.md"]);
    });
}

fn effective(protected: &[&str]) -> Effective {
    let list = Val::List(protected.iter().map(|p| p.to_string()).collect());
    Effective { values: BTreeMap::from([("gate.protectedPaths".to_owned(), (list, Source::Default))]), widened: vec![] }
}

#[test]
fn s_4_a_tracked_hooks_directory_under_a_wildcard_is_refused() {
    isolated("s_4_a_tracked_hooks_directory_under_a_wildcard_is_refused", || {
        let b = Base::new("hooks", &[(b"tools/hooks/pre-commit", b"#!/bin/sh\n")], &[("a", "tools")]);
        let eff = effective(&["Cargo.toml"]);
        assert_eq!(candidate::protected_paths(&eff, &b.repo()).unwrap(), ["Cargo.toml"]);
        for (value, dir) in [("tools/hooks", "tools/hooks/"), ("./tools/hooks/", "tools/hooks/"), (".", "/")] {
            git(&b.root, &["config", "core.hooksPath", value]);
            assert_eq!(candidate::protected_paths(&eff, &b.repo()).unwrap(), ["Cargo.toml", dir], "{value}");
        }
        // A directory reached through a link is protected as written and where the link leads.
        git(&b.root, &["config", "core.hooksPath", "a/hooks"]);
        let through = candidate::protected_paths(&eff, &b.repo()).unwrap();
        assert_eq!(through, ["Cargo.toml", "a/hooks/", "tools/hooks/", "a"]);
        b.export();
        fs::remove_file(b.work().join("a")).unwrap();
        symlink("elsewhere", b.work().join("a")).unwrap();
        let why = refused(b.changes_with(&["*", "tools/**"], &through), Code::ProtectedPathWildcard);
        assert!(why.contains("\"a\""), "a link above the hooks directory: {why}");
        fs::remove_dir_all(b.work()).unwrap();
        fs::create_dir(b.work()).unwrap();
        // Where a link leads is read before a `..` after it, and a directory not made yet is protected where it
        // would be (P9B-2).
        let absent = Base::new("hooks-absent", &[(b"tools/readme", b"r\n")], &[("a", "tools")]);
        git(&absent.root, &["config", "core.hooksPath", "a/hooks"]);
        assert_eq!(
            candidate::protected_paths(&eff, &absent.repo()).unwrap(),
            ["Cargo.toml", "a/hooks/", "tools/hooks/", "a"]
        );
        absent.export();
        fs::create_dir(absent.work().join("tools/hooks")).unwrap();
        absent.put("tools/hooks/pre-commit", b"#!/bin/sh\n");
        let protected = candidate::protected_paths(&eff, &absent.repo()).unwrap();
        refused(absent.changes_with(&["tools/**"], &protected), Code::ProtectedPathWildcard);
        let files: &[(&[u8], &[u8])] = &[(b"tools/sub/x", b"x\n"), (b"tools/hooks/pre-commit", b"#!/bin/sh\n")];
        let parent = Base::new("hooks-parent", files, &[("a", "tools/sub")]);
        git(&parent.root, &["config", "core.hooksPath", "a/../hooks"]);
        let protected = candidate::protected_paths(&eff, &parent.repo()).unwrap();
        assert!(protected.contains(&"tools/hooks/".to_owned()), "{protected:?}");
        // The link the path passes through is protected too, since changing it moves the directory.
        assert!(protected.contains(&"a".to_owned()), "{protected:?}");
        parent.export();
        fs::remove_file(parent.work().join("a")).unwrap();
        symlink("tools/elsewhere", parent.work().join("a")).unwrap();
        refused(parent.changes_with(&["*"], &protected), Code::ProtectedPathWildcard);
        fs::remove_file(parent.work().join("a")).unwrap();
        symlink("tools/sub", parent.work().join("a")).unwrap();
        parent.put("tools/hooks/pre-commit", b"#!/bin/sh\necho planted\n");
        refused(parent.changes_with(&["tools/**"], &protected), Code::ProtectedPathWildcard);
        // The kernel follows 40 links in a path, so a chain of 40 is read where it leads; a loop protects the
        // directory as written only.
        let mut chain: Vec<(String, String)> = (1..40).map(|i| (format!("l{i}"), format!("l{}", i + 1))).collect();
        chain.push(("l40".into(), "tools".into()));
        chain.extend([("x1".into(), "x2".into()), ("x2".into(), "x1".into())]);
        let chain: Vec<(&str, &str)> = chain.iter().map(|(a, b)| (a.as_str(), b.as_str())).collect();
        let linked = Base::new("hooks-chain", &[(b"tools/readme", b"r\n")], &chain);
        git(&linked.root, &["config", "core.hooksPath", "l1/hooks"]);
        let mut expected: Vec<String> = ["Cargo.toml", "l1/hooks/", "tools/hooks/"].map(String::from).to_vec();
        expected.extend((1..=40).map(|i| format!("l{i}")));
        assert_eq!(candidate::protected_paths(&eff, &linked.repo()).unwrap(), expected);
        // A root reached through a link of its own spends none of the 40.
        let alias = linked.root.parent().unwrap().join("alias");
        symlink(&linked.root, &alias).unwrap();
        let aliased = Repo { root: alias, prefix: PathBuf::new() };
        assert_eq!(candidate::protected_paths(&eff, &aliased).unwrap(), expected);
        git(&linked.root, &["config", "core.hooksPath", "x1/hooks"]);
        assert_eq!(candidate::protected_paths(&eff, &linked.repo()).unwrap(), ["Cargo.toml", "x1/hooks/"]);
        // A component that is not text protects the directory above it, which holds it.
        let bytes = OsStr::from_bytes(b"tools/\xff/hooks");
        let set = std::process::Command::new("git")
            .current_dir(&b.root)
            .args(["config", "core.hooksPath"])
            .arg(bytes)
            .status()
            .unwrap();
        assert!(set.success());
        assert_eq!(candidate::protected_paths(&eff, &b.repo()).unwrap(), ["Cargo.toml", "tools/"]);
        // Outside the working tree it protects nothing here.
        git(&b.root, &["config", "core.hooksPath", "/elsewhere/hooks"]);
        assert_eq!(candidate::protected_paths(&eff, &b.repo()).unwrap(), ["Cargo.toml"]);
        git(&b.root, &["config", "core.hooksPath", "tools/hooks"]);
        let protected = candidate::protected_paths(&eff, &b.repo()).unwrap();
        // At the lane guard on patterns, and at the change set on paths.
        let e = fabrewk_core::task::wildcards(&["tools/**"], &protected).unwrap_err();
        assert_eq!(e.code(), Some(Code::ProtectedPathWildcard));
        assert!(fabrewk_core::task::wildcards(&["tools/hooks/pre-commit"], &protected).is_ok());
        let root = vec!["/".to_owned()];
        assert!(fabrewk_core::task::wildcards(&["*.md"], &root).is_err(), "a hooks directory at the root");
        b.export();
        b.put("tools/hooks/pre-commit", b"#!/bin/sh\ncurl evil\n");
        refused(b.changes_with(&["tools/**"], &protected), Code::ProtectedPathWildcard);
        let kept = b.changes_with(&["tools/**", "tools/hooks/pre-commit"], &protected).unwrap();
        assert_eq!(paths(&kept), ["tools/hooks/pre-commit"]);
        refused(b.changes_with(&["**/pre-commit"], &root), Code::ProtectedPathWildcard);
        // The directory itself replaced is a change at a protected path too.
        fs::remove_dir_all(b.work().join("tools/hooks")).unwrap();
        symlink("/tmp", b.work().join("tools/hooks")).unwrap();
        refused(b.changes_with(&["tools/**"], &protected), Code::ProtectedPathWildcard);
    });
}

#[test]
fn refusal_lane_violation() {
    isolated("refusal_lane_violation", || {
        let b = Base::new("lane", &[(b"src/a.rs", b"a\n"), (b"docs/x.md", b"x\n")], &[]);
        b.export();
        b.put("src/a.rs", b"a2\n");
        assert_eq!(paths(&b.changes(&["src/*.rs"]).unwrap()), ["src/a.rs"]);
        b.put("Cargo.toml", b"[package]\n");
        let why = refused(b.changes(&["src/*.rs"]), Code::LaneViolation);
        assert!(why.contains("\"Cargo.toml\"") && why.contains("do not match"), "{why}");
        fs::remove_file(b.work().join("Cargo.toml")).unwrap();
        // A deletion outside the lane is a change outside it.
        fs::remove_file(b.work().join("docs/x.md")).unwrap();
        refused(b.changes(&["src/*.rs"]), Code::LaneViolation);
        let changes = b.changes(&["src/*.rs", "docs/*"]).unwrap();
        assert_eq!(changes[0], Change { path: "docs/x.md".into(), before: changes[0].before.clone(), after: None });
        let c = b.commit(&changes, false, None);
        assert!(git(&b.root, &["ls-tree", "-r", "--name-only", &c.commit]).lines().all(|l| l != "docs/x.md"));
    });
}

#[test]
fn refusal_credential_suspected_in_a_candidate() {
    isolated("refusal_credential_suspected_in_a_candidate", || {
        let key = "-----BEGIN OPENSSH PRIVATE KEY-----\nb3Blbg==\n-----END OPENSSH PRIVATE KEY-----\n";
        let cases: [(&str, String); 6] = [
            ("src/conf.rs", format!("let k = \"AKIA{}\";\n", "A".repeat(16))),
            ("src/token.txt", format!("ghp_{}\n", "x".repeat(20))),
            ("src/key.txt", key.to_owned()),
            ("src/id_rsa", "anything\n".to_owned()),
            ("src/.env", "A=1\n".to_owned()),
            ("src/cert.pem", "anything\n".to_owned()),
        ];
        let b = Base::new("credentials", &[(b"src/a.rs", b"a\n")], &[]);
        for (path, content) in &cases {
            b.export();
            b.put(path, content.as_bytes());
            let why = refused(b.changes(&["src/**"]), Code::CredentialSuspected);
            assert!(why.contains("heuristic") && why.contains(&format!("{path:?}")), "{why}");
            fs::remove_dir_all(b.work()).unwrap();
            fs::create_dir(b.work()).unwrap();
        }
        // A link is matched by its name, never followed.
        b.export();
        symlink("/etc/hostname", b.work().join("src/server.key")).unwrap();
        refused(b.changes(&["src/**"]), Code::CredentialSuspected);
        fs::remove_file(b.work().join("src/server.key")).unwrap();
        // A refused content never reaches the object store.
        let secret = format!("AKIA{}\n", "Q".repeat(16));
        b.put("src/secret.txt", secret.as_bytes());
        refused(b.changes(&["src/**"]), Code::CredentialSuspected);
        let secret_file = b.work().join("src/secret.txt");
        let oid = run(&b.root, "git", &["hash-object", "--no-filters", secret_file.to_str().unwrap()]);
        let present =
            std::process::Command::new("git").current_dir(&b.root).args(["cat-file", "-e", oid.trim()]).output();
        assert!(!present.unwrap().status.success(), "the refused content is not in the object store");
        fs::remove_file(b.work().join("src/secret.txt")).unwrap();
        // Near misses are no match.
        b.put("src/near.txt", format!("AKIA{} sk-{}\n", "A".repeat(15), "x".repeat(19)).as_bytes());
        b.put("src/id_ed25519.pub", b"ssh-ed25519 AAAA\n");
        assert_eq!(paths(&b.changes(&["src/**"]).unwrap()), ["src/id_ed25519.pub", "src/near.txt"]);
    });
}

#[test]
fn pd_45_the_candidate_names_its_agent_and_the_sign_off_the_project_asks() {
    isolated("pd_45_the_candidate_names_its_agent_and_the_sign_off_the_project_asks", || {
        let b = Base::new("candidate", &[(b"src/a.rs", b"a\n")], &[]);
        // The person's staged and unstaged edits stay as they were.
        fs::write(b.root.join("README.md"), "staged\n").unwrap();
        git(&b.root, &["add", "README.md"]);
        fs::write(b.root.join("README.md"), "unstaged\n").unwrap();
        let index = fs::read(b.root.join(".git/index")).unwrap();
        b.export();
        b.put("src/a.rs", b"a2\n");
        let changes = b.changes(&["src/**"]).unwrap();
        let first = b.commit(&changes, false, None);
        assert_eq!(git(&b.root, &["rev-parse", &format!("{}^@", first.commit)]), b.base, "its only parent is the base");
        assert_eq!(git(&b.root, &["rev-parse", "refs/heads/fabrewk/t-00000001"]), first.commit);
        assert_eq!(git(&b.root, &["rev-parse", &format!("{}^{{tree}}", first.commit)]), first.tree);
        let message = git(&b.root, &["log", "-1", "--format=%B", &first.commit]);
        let coauthors: Vec<&str> = message.lines().filter(|l| l.starts_with("Co-authored-by: ")).collect();
        assert_eq!(coauthors, ["Co-authored-by: fake-agent (nowhere, fake-1) <noreply@fabrewk.invalid>"]);
        assert!(!message.contains("Signed-off-by:"), "{message}");
        assert!(message.starts_with("fabrewk: candidate for t-00000001\n"), "{message}");
        let trailers = git(&b.root, &["log", "-1", "--format=%(trailers:key=Co-authored-by,valueonly)", &first.commit]);
        assert_eq!(trailers, "fake-agent (nowhere, fake-1) <noreply@fabrewk.invalid>", "Git reads it as a trailer");
        // A later round moves the branch from the candidate the record names; from anywhere else it refuses.
        let second = b.commit(&changes, true, Some(&first.commit));
        let message = git(&b.root, &["log", "-1", "--format=%B", &second.commit]);
        assert!(message.ends_with("\nSigned-off-by: Fixture <fixture@example.invalid>"), "{message}");
        assert_eq!(git(&b.root, &["rev-parse", "refs/heads/fabrewk/t-00000001"]), second.commit);
        let stale = candidate::commit(
            &b.repo(),
            &b.base,
            &changes,
            b.work(),
            &b.scratch.root,
            "t-00000001",
            &AGENT,
            false,
            Some(&first.commit),
        );
        assert!(refused(stale, Code::ToolFailed).contains("not where the record says"));
        let absent = candidate::commit(
            &b.repo(),
            &b.base,
            &changes,
            b.work(),
            &b.scratch.root,
            "t-00000001",
            &AGENT,
            false,
            None,
        );
        refused(absent, Code::ToolFailed);
        assert_eq!(fs::read(b.root.join(".git/index")).unwrap(), index, "the person's index is untouched");
        assert_eq!(fs::read_to_string(b.root.join("README.md")).unwrap(), "unstaged\n");
        assert_eq!(git(&b.root, &["rev-parse", "HEAD"]), b.base, "the record branch is untouched");
        // A name no trailer can carry, as a hostile stream might report it, is refused.
        let long = "m".repeat(101);
        for model in ["a\nSigned-off-by: someone", "x <y>", "", "é", &long] {
            let agent = Agent { model, ..AGENT };
            let r = candidate::commit(
                &b.repo(),
                &b.base,
                &changes,
                b.work(),
                &b.scratch.root,
                "t-00000002",
                &agent,
                false,
                None,
            );
            refused(r, Code::ToolFailed);
        }
        let r = candidate::commit(&b.repo(), &b.base, &changes, b.work(), &b.scratch.root, "x", &AGENT, false, None);
        refused(r, Code::ToolFailed);
        let longest = Agent { model: &long[..100], ..AGENT };
        let r = candidate::commit(
            &b.repo(),
            &b.base,
            &changes,
            b.work(),
            &b.scratch.root,
            "t-00000003",
            &longest,
            false,
            None,
        );
        assert!(git(&b.root, &["log", "-1", "--format=%B", &r.unwrap().commit]).contains(&long[..100]));
        // A copy changed after its change set was read is refused, never committed as the set says.
        b.put("src/a.rs", b"a3\n");
        let r = candidate::commit(
            &b.repo(),
            &b.base,
            &changes,
            b.work(),
            &b.scratch.root,
            "t-00000004",
            &AGENT,
            false,
            None,
        );
        assert!(refused(r, Code::ToolFailed).contains("changed after"));
    });
}

#[test]
fn s_11_the_candidate_runs_no_hook() {
    isolated("s_11_the_candidate_runs_no_hook", || {
        let b = Base::new("candidate-hook", &[(b"hooks/reference-transaction", b"")], &[]);
        let ran = b.root.join("hook-ran");
        let hook = b.root.join("hooks/reference-transaction");
        fs::write(&hook, format!("#!/bin/sh\necho \"$1\" >> '{}'\nexit 1\n", ran.display())).unwrap();
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
        fs::copy(&hook, b.root.join(".git/hooks/reference-transaction")).unwrap();
        git(&b.root, &["config", "core.hooksPath", "hooks"]);
        b.export();
        b.put("src/a.rs", b"a\n");
        let changes = b.changes(&["src/**"]).unwrap();
        b.commit(&changes, false, None);
        assert!(!ran.exists(), "a hook ran");
    });
}

#[test]
fn refusal_disk_low() {
    isolated("refusal_disk_low", || {
        let dir = tmp("disk");
        let df = std::env::split_paths(&std::env::var_os("PATH").unwrap()).map(|d| d.join("df")).find(|p| p.is_file());
        let df = df.expect("df is needed");
        let free = candidate::free_space(&dir, 0, Some(&df)).unwrap();
        assert!(free > 0);
        let why = refused(candidate::free_space(&dir, free + 1_000_000, Some(&df)), Code::DiskLow);
        assert!(why.contains("below limits.freeDiskMegabytes"), "{why}");
        refused(candidate::free_space(&dir, 0, None), Code::DiskLow);
        // A `df` that fails, or prints what cannot be read, refuses too; one that prints POSIX's form is read.
        let fake = |name: &str, body: &str| {
            let p = dir.join(name);
            fs::write(&p, format!("#!/bin/sh\n{body}\n")).unwrap();
            fs::set_permissions(&p, fs::Permissions::from_mode(0o755)).unwrap();
            p
        };
        let header = "echo 'Filesystem 1024-blocks Used Available Capacity Mounted on'";
        for (name, body) in [
            ("fails", "exit 1".to_owned()),
            ("empty", String::new()),
            ("words", format!("{header}; echo 'disk many some more full% /'")),
            ("two", format!("{header}; echo 'a 10 5 5 50% /'; echo 'b 10 5 5 50% /x'")),
            ("negative", format!("{header}; echo 'a 10 5 -5 50% /'")),
        ] {
            refused(candidate::free_space(&dir, 0, Some(&fake(name, &body))), Code::DiskLow);
        }
        let spaced = fake("spaced", &format!("{header}; echo 'my disk 9 3 4096 1% /mnt/a b 7%'"));
        assert_eq!(candidate::free_space(&dir, 4, Some(&spaced)).unwrap(), 4);
        refused(candidate::free_space(&dir, 5, Some(&spaced)), Code::DiskLow);
        // A name that holds three counts and a word with `%` is no capacity; the capacity is the first that follows
        // three counts.
        let counted = fake("counted", &format!("{header}; echo 'my 1 2 3 x% 9 3 4096 1% /'"));
        assert_eq!(candidate::free_space(&dir, 4, Some(&counted)).unwrap(), 4);
        // A line that reads two ways, a name or a mount point that holds counts and a capacity, refuses (P9B-3).
        for (name, line) in [
            ("twice", "vol 1 2 4096 1% 10000 9999 1 100% /fixture"),
            ("mount", "a 10 5 5 50% /x 1 2 3 4%"),
            ("negative", "a 1 2 4096 1% 10 9 -1 100% /"),
        ] {
            let why = refused(
                candidate::free_space(&dir, 0, Some(&fake(name, &format!("{header}; echo '{line}'")))),
                Code::DiskLow,
            );
            assert!(why.contains("could not say"), "{why}");
        }
    });
}

#[test]
fn an_entry_of_a_base_is_read_as_its_mode_says() {
    isolated("an_entry_of_a_base_is_read_as_its_mode_says", || {
        let b = Base::new("entry", &[(b"bin/tool", b"#!/bin/sh\n")], &[]);
        fs::set_permissions(b.root.join("bin/tool"), fs::Permissions::from_mode(0o755)).unwrap();
        git(&b.root, &["add", "bin/tool"]);
        git(&b.root, &["-c", "core.hooksPath=/dev/null", "commit", "-q", "-m", "x"]);
        let b = Base { base: git(&b.root, &["rev-parse", "HEAD"]), ..b };
        b.export();
        let mode = fs::metadata(b.work().join("bin/tool")).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o755);
        assert_eq!(fs::metadata(b.work().join("README.md")).unwrap().permissions().mode() & 0o777, 0o644);
        // A cell that closed a directory and a file leaves them readable to the walk, the executable bit kept.
        fs::set_permissions(b.work().join("bin/tool"), fs::Permissions::from_mode(0o100)).unwrap();
        fs::set_permissions(b.work().join("bin"), fs::Permissions::from_mode(0o000)).unwrap();
        assert_eq!(b.changes(&["**"]).unwrap(), vec![]);
        let names = Val::List(vec!["X.md".into()]);
        let eff = Effective {
            values: BTreeMap::from([("gate.protectedNames".into(), (names, Source::Default))]),
            widened: vec![],
        };
        assert_eq!(candidate::protected_names(&eff), ["X.md"]);
        assert_eq!(candidate::protected_names(&effective(&[])), Vec::<String>::new());
        let entry = Entry { mode: Mode::Executable, oid: String::new() };
        assert_eq!(entry.mode.git(), "100755");
    });
}

/// Runs Git at `dir` with `input` on its standard input; gives its output, trimmed.
fn git_input(dir: &Path, args: &[&str], input: &[u8]) -> String {
    use std::io::Write;
    let mut child = std::process::Command::new("git")
        .current_dir(dir)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success(), "git {args:?}");
    String::from_utf8(out.stdout).unwrap().trim().to_owned()
}

#[test]
fn a_base_is_named_by_its_id_and_its_tree_is_never_written_through_a_link() {
    isolated("a_base_is_named_by_its_id_and_its_tree_is_never_written_through_a_link", || {
        let b = Base::new("crafted", &[], &[]);
        let (wide, long) = ("g".repeat(40), "a".repeat(41));
        for id in ["HEAD", "main", &b.base[..8], &b.base[..39], &wide, &long, "--output=x", &b.base.to_uppercase()] {
            let why = refused(candidate::export(&b.repo(), id, b.work()), Code::ToolFailed);
            assert!(why.contains("names no commit"), "{id}: {why}");
        }
        assert_eq!(listing(b.work()), Vec::<Vec<u8>>::new());
        // A tree that holds `a` twice, as a link and as a directory, as only a hostile history can: the copy never
        // writes through the link, which leads outside it.
        let outside = tmp("crafted-outside");
        fs::create_dir_all(&outside).unwrap();
        let raw = |hex: &str| -> Vec<u8> {
            (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect()
        };
        let file = git_input(&b.root, &["hash-object", "-w", "--stdin"], b"x\n");
        let target = git_input(&b.root, &["hash-object", "-w", "--stdin"], outside.as_os_str().as_bytes());
        let sub = git_input(&b.root, &["mktree"], format!("100644 blob {file}\tb\n").as_bytes());
        let mut tree = b"120000 a\0".to_vec();
        tree.extend(raw(&target));
        tree.extend(b"40000 a\0");
        tree.extend(raw(&sub));
        let tree = git_input(&b.root, &["hash-object", "-t", "tree", "--literally", "-w", "--stdin"], &tree);
        let commit = git_input(&b.root, &["commit-tree", &tree, "-m", "crafted"], b"");
        let why = refused(candidate::export(&b.repo(), &commit, b.work()), Code::ToolFailed);
        assert!(why.contains("not a directory"), "{why}");
        assert_eq!(listing(&outside), Vec::<Vec<u8>>::new(), "nothing was written through the link");
    });
}

#[test]
fn a_submodule_the_cell_filled_leaves_and_what_it_holds_enters() {
    isolated("a_submodule_the_cell_filled_leaves_and_what_it_holds_enters", || {
        let b = Base::new("submodule", &[], &[]);
        let sub = git(&b.root, &["rev-parse", "HEAD"]);
        git(&b.root, &["update-index", "--add", "--cacheinfo", &format!("160000,{sub},vendor/sub")]);
        git(&b.root, &["-c", "core.hooksPath=/dev/null", "commit", "-q", "-m", "sub"]);
        let b = Base { base: git(&b.root, &["rev-parse", "HEAD"]), ..b };
        b.export();
        b.put("vendor/sub/x", b"x\n");
        let changes = b.changes(&["**"]).unwrap();
        assert_eq!(paths(&changes), ["vendor/sub", "vendor/sub/x"]);
        assert_eq!(changes[0].after, None, "the submodule leaves");
        let c = b.commit(&changes, false, None);
        assert_eq!(
            git(&b.root, &["ls-tree", "-r", &c.commit, "vendor"]),
            format!("100644 blob {}\tvendor/sub/x", { git_input(&b.root, &["hash-object", "--stdin"], b"x\n") })
        );
    });
}

#[test]
fn s_2_a_linked_gitignore_holds_no_rule_and_a_path_no_rule_matches_enters() {
    isolated("s_2_a_linked_gitignore_holds_no_rule_and_a_path_no_rule_matches_enters", || {
        // Git never reads a `.gitignore` that is a link; neither does the copy, which then reads no rule at all.
        let b = Base::new("linked-rules", &[], &[(".gitignore", "pipe")]);
        b.export();
        run(Path::new("/"), "mkfifo", &[b.work().join("pipe").to_str().unwrap()]);
        let why = refused(b.changes(&["**"]), Code::LaneViolation);
        assert!(why.contains("\"pipe\""), "{why}");
        assert!(!b.scratch.root.join("rules").exists(), "no rule, so no rules repository");
        // A new path that no rule of the base matches enters the change set.
        let b = Base::new("unmatched-rules", &[(b".gitignore", b"*.log\n")], &[]);
        b.export();
        b.put("src/new.rs", b"new\n");
        assert_eq!(paths(&b.changes(&["**"]).unwrap()), ["src/new.rs"]);
    });
}

#[test]
fn s_4_a_protected_directory_replaced_by_a_file_is_guarded_and_a_path_beside_it_is_not() {
    isolated("s_4_a_protected_directory_replaced_by_a_file_is_guarded_and_a_path_beside_it_is_not", || {
        let b = Base::new("replaced", &[(b".github/ci.yml", b"ci\n")], &[]);
        b.export();
        fs::remove_dir_all(b.work().join(".github")).unwrap();
        b.put(".github", b"not a directory\n");
        let why = refused(b.changes_with(&["**"], &[".github/".into()]), Code::ProtectedPathWildcard);
        assert!(why.contains("\".github\""), "{why}");
        assert_eq!(paths(&b.changes_with(&["**"], &["tools/".into()]).unwrap()), [".github", ".github/ci.yml"]);
        // A protected path written as a file guards that path alone.
        let file = ["Cargo.toml".to_owned()];
        assert_eq!(paths(&b.changes_with(&["**"], &file).unwrap()), [".github", ".github/ci.yml"]);
        b.put("Cargo.toml", b"[package]\n");
        refused(b.changes_with(&["**"], &file), Code::ProtectedPathWildcard);
    });
}

#[test]
fn p9b_1_a_path_git_refuses_to_hold_is_refused_before_anything_is_written() {
    isolated("p9b_1_a_path_git_refuses_to_hold_is_refused_before_anything_is_written", || {
        let b = Base::new("refused-by-git", &[(b"readme", b"r\n")], &[]);
        git(&b.root, &["config", "core.protectNTFS", "true"]);
        b.export();
        let objects = || git(&b.root, &["count-objects", "-v"]);
        let before = objects();
        for name in [".gitmodules", "GI7EBA~1", ".GitModules"] {
            symlink("planted target", b.work().join(name)).unwrap();
            // Each listed literally, as a protected name must be, so that only Git's refusal is left.
            let why = refused(b.changes(&["*", ".gitmodules", ".GitModules"]), Code::ProtectedPath);
            assert!(why.contains(&format!("{name:?}")) && why.contains("Git refuses"), "{why}");
            fs::remove_file(b.work().join(name)).unwrap();
        }
        assert_eq!(objects(), before, "nothing refused reached the object store");
        // A change set read before Git's rule changed is checked again in the candidate's own index.
        symlink("planted target", b.work().join("GI7EBA~1")).unwrap();
        git(&b.root, &["config", "core.protectNTFS", "false"]);
        let changes = b.changes(&["*"]).unwrap();
        git(&b.root, &["config", "core.protectNTFS", "true"]);
        let before = objects();
        let r = candidate::commit(
            &b.repo(),
            &b.base,
            &changes,
            b.work(),
            &b.scratch.root,
            "t-00000001",
            &AGENT,
            false,
            None,
        );
        assert!(refused(r, Code::ToolFailed).contains("Git would not hold"));
        assert_eq!(objects(), before, "the candidate's check comes before any object is written");
        assert!(
            std::process::Command::new("git")
                .current_dir(&b.root)
                .args(["rev-parse", "--verify", "-q", "refs/heads/fabrewk/t-00000001"])
                .status()
                .unwrap()
                .code()
                == Some(1)
        );
    });
}
