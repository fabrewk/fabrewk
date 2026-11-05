//! Counting each ring's directories, and checking that every Rust file of the program belongs to exactly one ring.

use crate::budget::{Ring, RingKind};
use crate::count::{CountError, scan_source};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// Directories a package may hold that are outside the count (REQ-BUD-1), plus Cargo's output.
const UNCOUNTED_IN_PACKAGE: &[&str] = &["tests", "benches", "examples", "target"];

/// Directories of the repository that hold no program code.
const OUTSIDE_PROGRAM: &[&str] = &[".git", "target", ".brewstillery", "tools"];

#[derive(Debug)]
pub enum Error {
    Io(PathBuf, std::io::Error),
    NotUtf8(PathBuf),
    Count(PathBuf, CountError),
    Symlink(PathBuf),
    MissingDir(PathBuf),
    Unassigned(PathBuf),
    Overlap(&'static str, &'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(p, e) => write!(f, "{}: {e}", p.display()),
            Error::NotUtf8(p) => write!(f, "{}: not UTF-8", p.display()),
            Error::Count(p, e) => write!(f, "{}: {e}", p.display()),
            Error::Symlink(p) => write!(
                f,
                "{}: symbolic link inside the program's code",
                p.display()
            ),
            Error::MissingDir(p) => {
                write!(f, "{}: a ring's directory that does not exist", p.display())
            }
            Error::Unassigned(p) => write!(f, "{}: Rust code in no ring", p.display()),
            Error::Overlap(a, b) => write!(f, "ring directories {a} and {b} overlap"),
        }
    }
}

impl std::error::Error for Error {}

/// One ring's measure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RingCount {
    pub name: &'static str,
    pub kind: RingKind,
    pub files: usize,
    pub lines: usize,
    /// The counted files, canonical.
    pub sources: Vec<PathBuf>,
    /// The data files the counted files include with `include_str!` or `include_bytes!`, canonical when they exist.
    pub data: Vec<PathBuf>,
}

impl RingCount {
    pub fn limit(&self) -> usize {
        self.kind.limit()
    }

    pub fn within(&self) -> bool {
        self.lines <= self.limit()
    }
}

/// Counts every ring under `root`, after checking that rings do not overlap and that all program code is in one.
pub fn count_rings(root: &Path, rings: &[Ring]) -> Result<Vec<RingCount>, Error> {
    let dirs: Vec<&'static str> = rings.iter().flat_map(|r| r.dirs.iter().copied()).collect();
    for (n, a) in dirs.iter().enumerate() {
        for b in &dirs[n + 1..] {
            if Path::new(a).starts_with(b) || Path::new(b).starts_with(a) {
                return Err(Error::Overlap(a, b));
            }
        }
    }
    for d in &dirs {
        if !root.join(d).is_dir() {
            return Err(Error::MissingDir(root.join(d)));
        }
    }
    check_assigned(root, &dirs)?;
    rings.iter().map(|ring| count_ring(root, ring)).collect()
}

fn count_ring(root: &Path, ring: &Ring) -> Result<RingCount, Error> {
    let mut measure = RingCount {
        name: ring.name,
        kind: ring.kind,
        files: 0,
        lines: 0,
        sources: Vec::new(),
        data: Vec::new(),
    };
    for dir in ring.dirs {
        walk(&root.join(dir), &mut |file| {
            let bytes = fs::read(file).map_err(|e| Error::Io(file.to_path_buf(), e))?;
            let src = String::from_utf8(bytes).map_err(|_| Error::NotUtf8(file.to_path_buf()))?;
            let scan = scan_source(&src).map_err(|e| Error::Count(file.to_path_buf(), e))?;
            let dir = file.parent().unwrap_or(Path::new(""));
            measure.lines += scan.lines;
            measure.files += 1;
            measure.sources.push(canonical(file));
            measure
                .data
                .extend(scan.data.iter().map(|d| canonical(&dir.join(d))));
            Ok(())
        })?;
    }
    Ok(measure)
}

/// Every counted Rust file under `root`, outside `OUTSIDE_PROGRAM`, must lie under one of `dirs`; and every directory
/// under `crates/` must be one of them.
fn check_assigned(root: &Path, dirs: &[&str]) -> Result<(), Error> {
    let assigned = |p: &Path| dirs.iter().any(|d| p.starts_with(root.join(d)));
    let crates = root.join("crates");
    if crates.is_dir() {
        for entry in sorted_entries(&crates)? {
            if !assigned(&entry) {
                return Err(Error::Unassigned(entry));
            }
        }
    }
    for entry in sorted_entries(root)? {
        let name = entry
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        if OUTSIDE_PROGRAM.contains(&name) {
            continue;
        }
        let file_type = fs::symlink_metadata(&entry)
            .map_err(|e| Error::Io(entry.clone(), e))?
            .file_type();
        if file_type.is_dir() {
            walk(&entry, &mut |file| {
                if assigned(file) {
                    Ok(())
                } else {
                    Err(Error::Unassigned(file.to_path_buf()))
                }
            })?;
        } else if is_rust(&entry) || file_type.is_symlink() {
            return Err(if file_type.is_symlink() {
                Error::Symlink(entry)
            } else {
                Error::Unassigned(entry)
            });
        }
    }
    Ok(())
}

fn is_rust(path: &Path) -> bool {
    path.extension().is_some_and(|e| e == "rs")
}

/// A package is a directory whose `Cargo.toml` has a `[package]` table.
fn is_package(dir: &Path) -> bool {
    fs::read_to_string(dir.join("Cargo.toml"))
        .is_ok_and(|t| t.lines().any(|l| l.trim() == "[package]"))
}

fn sorted_entries(dir: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut entries = fs::read_dir(dir)
        .and_then(|it| {
            it.map(|e| e.map(|e| e.path()))
                .collect::<Result<Vec<_>, _>>()
        })
        .map_err(|e| Error::Io(dir.to_path_buf(), e))?;
    entries.sort();
    Ok(entries)
}

/// Visits every counted `.rs` file under `dir`; a symbolic link anywhere is an error.
fn walk(dir: &Path, visit: &mut dyn FnMut(&Path) -> Result<(), Error>) -> Result<(), Error> {
    let package = is_package(dir);
    for entry in sorted_entries(dir)? {
        let file_type = fs::symlink_metadata(&entry)
            .map_err(|e| Error::Io(entry.clone(), e))?
            .file_type();
        let name = entry
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        if file_type.is_symlink() {
            return Err(Error::Symlink(entry));
        } else if file_type.is_dir() {
            if !(package && UNCOUNTED_IN_PACKAGE.contains(&name)) {
                walk(&entry, visit)?;
            }
        } else if is_rust(&entry) {
            visit(&entry)?;
        }
    }
    Ok(())
}

/// The path with `.` and `..` resolved and links followed when it exists; as given otherwise.
pub fn canonical(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}
