//! Integration as a library (part 9g; REQ-GATE-8, S-4, S-11): the third guard on the change set recomputed from the
//! base's and the candidate's trees (PD-16 at integration), and what the index and the working tree hold at the
//! touched paths, then brought to the candidate from the objects' bytes.

mod common;

use common::*;
use fabrewk_core::Code;
use fabrewk_core::candidate::{Lanes, Mode};
use fabrewk_core::merge::{self, Held};
use fabrewk_core::record::Repo;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn run(root: &Path, args: &[&str], input: &str) -> String {
    let mut c = Command::new("git");
    c.current_dir(root).args(args).stdin(Stdio::piped()).stdout(Stdio::piped());
    let mut child = c.spawn().unwrap();
    use std::io::Write;
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success(), "git {args:?}");
    String::from_utf8(out.stdout).unwrap().trim_end().to_owned()
}

/// A repository whose base commit holds `README.md` and `old/old.txt`, its working tree and index at it.
fn base(name: &str) -> (PathBuf, String) {
    let root = tmp(name).join("repo");
    fs::create_dir_all(root.join("old")).unwrap();
    git(&root, &["init", "-q", "-b", "main"]);
    for (k, v) in [("user.name", "Fixture"), ("user.email", "fixture@example.invalid"), ("commit.gpgsign", "false")] {
        git(&root, &["config", k, v]);
    }
    fs::write(root.join("README.md"), "fixture\n").unwrap();
    fs::write(root.join("old/old.txt"), "old\n").unwrap();
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "core.hooksPath=/dev/null", "commit", "-q", "-m", "base"]);
    let head = git(&root, &["rev-parse", "HEAD"]);
    (root, head)
}

fn blob(root: &Path, bytes: &str) -> String {
    run(root, &["hash-object", "-w", "--stdin"], bytes)
}

/// A tree of `entries` (mode, name, object), made with `mktree`, which checks no name.
fn tree(root: &Path, entries: &[(&str, &str, String)]) -> String {
    let kind = |m: &str| if m == "040000" { "tree" } else { "blob" };
    let text: String = entries.iter().map(|(m, n, o)| format!("{m} {} {o}\t{n}\n", kind(m))).collect();
    run(root, &["mktree"], &text)
}

/// A commit on `base` whose root holds the base's entries and `more`.
fn candidate(root: &Path, base: &str, more: &[(&str, &str, String)]) -> String {
    let listed = git(root, &["ls-tree", base]);
    let mut entries: Vec<(&str, &str, String)> = listed
        .lines()
        .map(|l| {
            let (meta, name) = l.split_once('\t').unwrap();
            let m: Vec<&str> = meta.split(' ').collect();
            (if m[0] == "040000" { "040000" } else { "100644" }, name, m[2].to_owned())
        })
        .collect();
    entries.retain(|(_, n, _)| more.iter().all(|(_, m, _)| m != n));
    entries.extend(more.iter().cloned());
    let t = tree(root, &entries);
    run(root, &["commit-tree", &t, "-p", base, "-m", "candidate"], "")
}

fn changes(
    root: &Path,
    base: &str,
    cand: &str,
    allowed: &[&str],
) -> Result<Vec<fabrewk_core::candidate::Change>, Code> {
    let names = vec!["CLAUDE.md".to_owned()];
    let lanes = Lanes { allowed, names: &names, paths: &[] };
    merge::changes(&Repo { root: root.to_path_buf(), prefix: PathBuf::new() }, base, cand, &lanes)
        .map_err(|e| e.code().unwrap())
}

#[test]
fn pd_16_the_integrations_third_guard_refuses_a_protected_path_in_any_form() {
    let (root, base) = base("merge-guard");
    let file = blob(&root, "x\n");
    let inner = |name: &str| tree(&root, &[("100644", name, file.clone())]);
    let nested = tree(&root, &[("040000", ".git", inner("config"))]);
    for (what, more) in [
        ("domain", vec![("040000", ".brewstillery", inner("x"))]),
        ("folded", vec![("040000", ".BrewStillery", inner("x"))]),
        ("nested git", vec![("040000", "a", nested.clone())]),
        ("git file", vec![("100644", ".GIT", file.clone())]),
        ("a link named as the domain", vec![("120000", ".brewstillery", blob(&root, "elsewhere"))]),
        ("a way out", vec![("040000", "..", inner("x"))]),
    ] {
        let c = candidate(&root, &base, &more);
        assert_eq!(changes(&root, &base, &c, &["**"]).unwrap_err(), Code::ProtectedPath, "{what}");
    }
    // The lanes, again: a protected name reached by a wildcard, and a path outside the allowed files.
    let c = candidate(&root, &base, &[("100644", "CLAUDE.md", file.clone())]);
    assert_eq!(changes(&root, &base, &c, &["*"]).unwrap_err(), Code::ProtectedPathWildcard);
    assert_eq!(changes(&root, &base, &c, &["CLAUDE.md"]).unwrap().len(), 1);
    assert_eq!(changes(&root, &base, &c, &["src/**"]).unwrap_err(), Code::LaneViolation);
}

#[test]
fn req_gate_8_the_touched_paths_are_clean_then_brought_to_the_candidate() {
    let (root, base) = base("merge-bring");
    let repo = Repo { root: root.clone(), prefix: PathBuf::new() };
    let src = tree(&root, &[("100644", "a.txt", blob(&root, "a\n"))]);
    // The candidate adds src/a.txt, changes README.md, and removes old/old.txt with its directory.
    let c = candidate(&root, &base, &[("040000", "src", src), ("100644", "README.md", blob(&root, "new\n"))]);
    let c = {
        let t = run(&root, &["rev-parse", &format!("{c}^{{tree}}")], "");
        let listed = git(&root, &["ls-tree", &t]);
        let kept: String = listed.lines().filter(|l| !l.ends_with("\told")).map(|l| format!("{l}\n")).collect();
        let t = run(&root, &["mktree"], &kept);
        run(&root, &["commit-tree", &t, "-p", &base, "-m", "candidate"], "")
    };
    let changes = changes(&root, &base, &c, &["**"]).unwrap();
    assert_eq!(changes.len(), 3, "{changes:?}");
    let scratch = root.join(".brewstillery/local/tmp/merge");
    let held = merge::held(&repo, &changes, &scratch).unwrap();
    merge::clean(&changes, &held, true, false).unwrap();
    assert_eq!(merge::clean(&changes, &held, false, true).unwrap_err().code(), Some(Code::WorktreeDirty));
    // A person's edit at a touched path, in the working tree, is neither the base's nor the candidate's.
    fs::write(root.join("README.md"), "mine\n").unwrap();
    let edited = merge::held(&repo, &changes, &scratch).unwrap();
    assert_eq!(merge::clean(&changes, &edited, true, true).unwrap_err().code(), Some(Code::WorktreeDirty));
    fs::write(root.join("README.md"), "fixture\n").unwrap();
    // A link where a file is held is not that file.
    fs::remove_file(root.join("README.md")).unwrap();
    std::os::unix::fs::symlink("fixture", root.join("README.md")).unwrap();
    let linked = merge::held(&repo, &changes, &scratch).unwrap();
    assert!(
        matches!(&linked[changes.iter().position(|c| c.path == "README.md").unwrap()].1, Held::Entry(e) if e.mode == Mode::Link)
    );
    assert!(merge::clean(&changes, &linked, true, true).is_err());
    fs::remove_file(root.join("README.md")).unwrap();
    fs::write(root.join("README.md"), "fixture\n").unwrap();
    merge::bring(&repo, &changes, &held, &scratch).unwrap();
    let now = merge::held(&repo, &changes, &scratch).unwrap();
    merge::clean(&changes, &now, false, true).unwrap();
    assert_eq!(fs::read(root.join("src/a.txt")).unwrap(), b"a\n");
    assert_eq!(fs::read(root.join("README.md")).unwrap(), b"new\n");
    assert!(!root.join("old").exists(), "the directory the removal emptied is gone");
    // Brought again, a path that holds the candidate's content already is left as it is.
    merge::bring(&repo, &changes, &now, &scratch).unwrap();
    let tree = run(&root, &["rev-parse", &format!("{c}^{{tree}}")], "");
    assert_eq!(run(&root, &["write-tree"], ""), tree, "the index holds the candidate's tree");
}

/// A file that becomes a directory, and a directory that becomes a file: each side's paths are clean at the base,
/// brought to the candidate, and clean at the candidate; the index follows.
#[test]
fn req_gate_8_a_file_and_a_directory_trade_places() {
    let (root, base) = base("merge-trade");
    let repo = Repo { root: root.clone(), prefix: PathBuf::new() };
    // `README.md` (a file) becomes a directory holding `inner`; `old` (a directory) becomes a file.
    let readme = tree(&root, &[("100644", "inner", blob(&root, "in\n"))]);
    let c = candidate(&root, &base, &[("040000", "README.md", readme), ("100644", "old", blob(&root, "now a file\n"))]);
    let changes = changes(&root, &base, &c, &["**"]).unwrap();
    assert_eq!(changes.len(), 4, "{changes:?}");
    let scratch = root.join(".brewstillery/local/tmp/merge");
    let held = merge::held(&repo, &changes, &scratch).unwrap();
    merge::clean(&changes, &held, true, false).unwrap();
    merge::bring(&repo, &changes, &held, &scratch).unwrap();
    merge::clean(&changes, &merge::held(&repo, &changes, &scratch).unwrap(), false, true).unwrap();
    assert_eq!(fs::read(root.join("README.md/inner")).unwrap(), b"in\n");
    assert_eq!(fs::read(root.join("old")).unwrap(), b"now a file\n");
    let tree = run(&root, &["rev-parse", &format!("{c}^{{tree}}")], "");
    assert_eq!(run(&root, &["write-tree"], ""), tree);
}

/// P9G-1: an edit made after the paths were read, before they are written, is never overwritten: each path is read
/// again just before it is written.
#[test]
fn p9g_1_an_edit_after_the_reading_is_not_overwritten() {
    let (root, base) = base("merge-late-edit");
    let repo = Repo { root: root.clone(), prefix: PathBuf::new() };
    let c = candidate(&root, &base, &[("100644", "README.md", blob(&root, "new\n"))]);
    let changes = changes(&root, &base, &c, &["**"]).unwrap();
    let scratch = root.join(".brewstillery/local/tmp/merge");
    let held = merge::held(&repo, &changes, &scratch).unwrap();
    fs::write(root.join("README.md"), "mine\n").unwrap();
    assert_eq!(merge::bring(&repo, &changes, &held, &scratch).unwrap_err().code(), Some(Code::WorktreeDirty));
    assert_eq!(fs::read(root.join("README.md")).unwrap(), b"mine\n");
    // The same for the index: a staged edit after the reading stays staged.
    fs::write(root.join("README.md"), "fixture\n").unwrap();
    let held = merge::held(&repo, &changes, &scratch).unwrap();
    let staged = blob(&root, "staged\n");
    run(&root, &["update-index", "--cacheinfo", &format!("100644,{staged},README.md")], "");
    assert_eq!(merge::bring(&repo, &changes, &held, &scratch).unwrap_err().code(), Some(Code::WorktreeDirty));
    assert!(git(&root, &["ls-files", "-s", "README.md"]).contains(&staged));
}

/// P9G-5: an index entry that a touched path would displace (one below it, or a file above it) is the person's: the
/// path is not clean, before the writes and when the index is read again just before its update.
#[test]
fn p9g_5_an_index_entry_a_touched_path_displaces_is_kept() {
    let (root, base) = base("merge-displaced");
    let repo = Repo { root: root.clone(), prefix: PathBuf::new() };
    let src = tree(&root, &[("100644", "a.txt", blob(&root, "a\n"))]);
    let c = candidate(&root, &base, &[("040000", "src", src)]);
    let changes = changes(&root, &base, &c, &["**"]).unwrap();
    let scratch = root.join(".brewstillery/local/tmp/merge");
    let mine = blob(&root, "staged mine\n");
    for entry in ["src/a.txt/other", "src"] {
        run(&root, &["update-index", "--add", "--cacheinfo", &format!("100644,{mine},{entry}")], "");
        let held = merge::held(&repo, &changes, &scratch).unwrap();
        assert_eq!(
            merge::clean(&changes, &held, true, false).unwrap_err().code(),
            Some(Code::WorktreeDirty),
            "{entry}"
        );
        run(&root, &["update-index", "--force-remove", entry], "");
        // Staged after the reading, it is found when the index is read again.
        let held = merge::held(&repo, &changes, &scratch).unwrap();
        merge::clean(&changes, &held, true, false).unwrap();
        run(&root, &["update-index", "--add", "--cacheinfo", &format!("100644,{mine},{entry}")], "");
        assert_eq!(
            merge::bring(&repo, &changes, &held, &scratch).unwrap_err().code(),
            Some(Code::WorktreeDirty),
            "{entry}"
        );
        assert!(git(&root, &["ls-files", "-s"]).contains(&mine), "{entry}");
        run(&root, &["update-index", "--force-remove", entry], "");
        for d in ["src/a.txt", "src"] {
            let _ = fs::remove_file(root.join(d));
            let _ = fs::remove_dir(root.join(d));
        }
    }
}

/// P9G-7: an index entry below a touched path that is itself no touched path is the person's, though another entry
/// below that path is touched.
#[test]
fn p9g_7_a_touched_entry_below_hides_no_other() {
    let (root, base) = base("merge-mixed");
    let repo = Repo { root: root.clone(), prefix: PathBuf::new() };
    // The base holds `old/old.txt`; the candidate removes it and adds the file `old`.
    let t = run(&root, &["rev-parse", &format!("{base}^{{tree}}")], "");
    let kept: String =
        git(&root, &["ls-tree", &t]).lines().filter(|l| !l.ends_with("\told")).map(|l| format!("{l}\n")).collect();
    let mut kept = kept;
    kept += &format!("100644 blob {}\told\n", blob(&root, "a file\n"));
    let t = run(&root, &["mktree"], &kept);
    let c = run(&root, &["commit-tree", &t, "-p", &base, "-m", "candidate"], "");
    let changes = changes(&root, &base, &c, &["**"]).unwrap();
    let scratch = root.join(".brewstillery/local/tmp/merge");
    let mine = blob(&root, "staged mine\n");
    run(&root, &["update-index", "--add", "--cacheinfo", &format!("100644,{mine},old/other")], "");
    let held = merge::held(&repo, &changes, &scratch).unwrap();
    assert_eq!(merge::clean(&changes, &held, true, false).unwrap_err().code(), Some(Code::WorktreeDirty));
}

/// P9G-21: the evidence a failure names is read from its own directory, the last component of the path its words
/// give, whatever the repository's path holds.
#[test]
fn p9g_21_the_evidence_is_read_from_its_own_directory() {
    let domain = tmp("merge-kept").join("local/evidence/git-parent/repo/.brewstillery");
    let dir = domain.join("local/evidence/git-1-2-3");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("stderr"), "fatal\n").unwrap();
    fs::write(dir.join("status"), "128\n").unwrap();
    let words =
        format!("git update-ref failed with exit status 128; Git's output is kept in {}; and more", dir.display());
    let kept = fabrewk_core::record::kept_in(&domain, &words).expect("the directory named");
    assert_eq!(kept, (fabrewk_core::record::sha256_hex(b"fatal\n"), Some(128)));
}
