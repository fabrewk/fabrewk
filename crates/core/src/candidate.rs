//! The attempt's copy and the candidate (REQ-RUN-3, REQ-RUN-4, REQ-RUN-10, REQ-SEC-3 in a change set, REQ-GATE-9's
//! names and hooks directory; `confinement-design.md` 3.8; part 9b). The copy is written from the objects' bytes, with
//! no filter, conversion or checkout, and holds no `.git` and no `.brewstillery` at any depth. What a cell left in it
//! is walked by Fabrewk itself, following nothing, once the cell has ended, and read only as bytes: no Git command reads
//! the copy's attributes, ignore files or modules, and the base's ignore rules are the base tree's own files. The
//! candidate is a commit on the base, made through a temporary index and put on `fabrewk/<task-id>` by
//! compare-and-swap.

use crate::code::{Code, Error, refuse};
use crate::record::git::{self, Repo, line, os};
use crate::settings::{Effective, Val};
use crate::{scan, task};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// REQ-RUN-10: refuses `DISK_LOW` unless the file system that holds `dir` has `megabytes` free, as `df -Pk` (the
/// program at `df`) reports it; a free space that cannot be read, `df` absent included, refuses too. Gives the
/// megabytes free.
pub fn free_space(dir: &Path, megabytes: u64, df: Option<&Path>) -> Result<u64, Error> {
    let low = |why: &str| refuse(Code::DiskLow, Some(dir), why);
    let df = df.ok_or_else(|| low("df is not present, so the free space cannot be read"))?;
    let mut df = Command::new(df);
    df.arg("-Pk").arg("--").arg(dir).env_clear().env("LC_ALL", "C").stdin(Stdio::null()).stderr(Stdio::null());
    let out = df.output().ok().filter(|o| o.status.success());
    let free = out.and_then(|o| available(&o.stdout)).ok_or_else(|| low("df could not say how much space is free"))?;
    let free = free / 1024;
    if free < megabytes {
        return Err(low(&format!("{free} MB free, below limits.freeDiskMegabytes ({megabytes} MB)")));
    }
    Ok(free)
}

/// The kibibytes available in `df -Pk`'s output: its one line after the header, whose capacity follows three counts,
/// the third of them the space available, in one place only. A count is anything `df` may print there (digits, signed
/// or not and of any length, or `-`) and a capacity digits with `%` or `-`, so that the real columns are always a
/// reading and a name or a mount point that looks like them too makes the line one that cannot be read (P9B-3,
/// P9B-10); so is a space available that is not a count of kibibytes.
fn available(out: &[u8]) -> Option<u64> {
    let mut lines = std::str::from_utf8(out).ok()?.lines().skip(1);
    let row: Vec<&str> = lines.next()?.split_whitespace().collect();
    if lines.next().is_some() {
        return None;
    }
    let digits = |f: &str| !f.is_empty() && f.bytes().all(|b| b.is_ascii_digit());
    let count = |f: &&str| *f == "-" || digits(f.strip_prefix('-').unwrap_or(f));
    let capacity = |f: &str| f == "-" || f.strip_suffix('%').is_some_and(digits);
    let mut readings = (4..row.len()).filter(|&i| capacity(row[i]) && row[i - 3..i].iter().all(count));
    let i = readings.next()?;
    if readings.next().is_some() {
        return None;
    }
    row[i - 1].parse().ok()
}

/// How a tree holds an entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    File,
    Executable,
    Link,
    Submodule,
}

impl Mode {
    /// The mode as Git writes it.
    pub fn git(self) -> &'static str {
        match self {
            Mode::File => "100644",
            Mode::Executable => "100755",
            Mode::Link => "120000",
            Mode::Submodule => "160000",
        }
    }
}

/// A tree's entry: its mode and its object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub mode: Mode,
    pub oid: String,
}

/// One change from the base to the candidate: a path, valid text, and its entry before and after; `None` where it is
/// absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    pub path: String,
    pub before: Option<Entry>,
    pub after: Option<Entry>,
}

/// Whether a path component names `.git` or `.brewstillery` as Git, or a file system it protects, would read it
/// (REQ-RUN-3): under ASCII case folding, with the code points HFS+ ignores removed, and with what NTFS drops after
/// the name (trailing dots and spaces, an alternate stream after `:`), in any part of it between backslashes, which
/// separate directories on NTFS; or NTFS's short names of either (`git~1`, `brewst~1`).
pub fn protected(name: &[u8]) -> bool {
    let ignorable = |c: &char| matches!(c, '\u{200c}'..='\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{206a}'..='\u{206f}');
    let kept: String = String::from_utf8_lossy(name).chars().filter(|c| !ignorable(c) && *c != '\u{feff}').collect();
    kept.split('\\').any(|part| {
        let base = part.split(':').next().unwrap_or_default().trim_end_matches(['.', ' ']).to_ascii_lowercase();
        [".git", ".brewstillery", "git~1", "brewst~1"].contains(&base.as_str())
    })
}

fn failed(path: Option<&Path>, why: impl Into<String>) -> Error {
    refuse(Code::ToolFailed, path, why)
}

/// Refuses `TOOL_FAILED` unless `id` names an object as Git writes it, 40 or 64 hexadecimal digits, so that no other
/// text reaches Git's command line.
pub(crate) fn object(id: &str) -> Result<(), Error> {
    let hex =
        (id.len() == 40 || id.len() == 64) && id.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
    if hex { Ok(()) } else { Err(failed(None, format!("{id:?} names no commit"))) }
}

/// What the copy could not hold, as `TOOL_FAILED` naming the path (REQ-RUN-3: a failed export is recorded so).
fn io(path: &Path) -> impl FnOnce(std::io::Error) -> Error + '_ {
    move |e| failed(Some(path), format!("the copy could not be written or read: {}", e.kind()))
}

/// A path as the person reads it in a refusal: as text, with what is not text replaced and every character quoted.
pub(crate) fn shown(path: &[u8]) -> String {
    format!("{:?}", String::from_utf8_lossy(path))
}

/// The entries of `commit`'s tree by path, every path with a protected component left out (REQ-RUN-3); an entry of
/// a mode Git does not write, or with an empty, `.` or `..` component, is `TOOL_FAILED`.
fn tree(repo: &Repo, commit: &str) -> Result<BTreeMap<Vec<u8>, Entry>, Error> {
    let spec = format!("{commit}^{{tree}}");
    let out = git::git(repo, &os(&["ls-tree", "-r", "-z", "--full-tree", &spec]), b"", None)?;
    let mut entries = BTreeMap::new();
    for record in out.split(|&b| b == 0).filter(|r| !r.is_empty()) {
        let tab = record.iter().position(|&b| b == b'\t').unwrap_or(record.len());
        let meta = String::from_utf8_lossy(&record[..tab]);
        let path = record.get(tab + 1..).unwrap_or_default().to_vec();
        let parts: Vec<&str> = meta.split(' ').collect();
        let mode = match parts[..] {
            ["100644", "blob", _] => Mode::File,
            ["100755", "blob", _] => Mode::Executable,
            ["120000", "blob", _] => Mode::Link,
            ["160000", "commit", _] => Mode::Submodule,
            _ => return Err(failed(None, format!("the base holds {} with a mode Git does not write", shown(&path)))),
        };
        let mut components = path.split(|&b| b == b'/');
        if components.clone().any(|c| matches!(c, b"" | b"." | b"..")) {
            return Err(failed(None, format!("the base holds the path {}, which no copy can hold", shown(&path))));
        }
        if !components.any(protected) {
            entries.insert(path, Entry { mode, oid: parts[2].to_owned() });
        }
    }
    Ok(entries)
}

/// The bytes of each blob of `entries`, in order, read as stored.
pub(crate) fn contents(repo: &Repo, entries: &[&Entry]) -> Result<Vec<Vec<u8>>, Error> {
    let input: String = entries.iter().map(|e| format!("{}\n", e.oid)).collect();
    let out = git::git(repo, &os(&["cat-file", "--batch"]), input.as_bytes(), None)?;
    let (mut rest, mut blobs) = (&out[..], Vec::new());
    let short = || failed(Some(&repo.root), "git cat-file gave less than was asked");
    for e in entries {
        let nl = rest.iter().position(|&b| b == b'\n').ok_or_else(short)?;
        let header = String::from_utf8_lossy(&rest[..nl]).into_owned();
        let size = match header.split(' ').collect::<Vec<_>>()[..] {
            [oid, "blob", size] if oid == e.oid => size.parse::<usize>().map_err(|_| short())?,
            _ => return Err(short()),
        };
        let body = rest.get(nl + 1..nl + 1 + size).ok_or_else(short)?;
        blobs.push(body.to_vec());
        rest = rest.get(nl + 2 + size..).ok_or_else(short)?;
    }
    Ok(blobs)
}

/// Makes the directories above `rel` under `root`, refusing to pass anything that is not a directory, a link above
/// all, which the copy never follows.
pub(crate) fn parents(root: &Path, rel: &[u8]) -> Result<(), Error> {
    let mut dir = root.to_path_buf();
    let components: Vec<&[u8]> = rel.split(|&b| b == b'/').collect();
    for c in &components[..components.len() - 1] {
        dir.push(OsStr::from_bytes(c));
        match fs::symlink_metadata(&dir) {
            Ok(m) if m.is_dir() => {}
            Ok(_) => return Err(failed(Some(&dir), "the base holds a path below one that is not a directory")),
            Err(_) => fs::create_dir(&dir).map_err(io(&dir))?,
        }
    }
    Ok(())
}

/// Writes `path` new under `root` with `bytes`, as a file of `mode` or a link whose target is `bytes`.
fn write_entry(root: &Path, path: &[u8], mode: Mode, bytes: &[u8]) -> Result<(), Error> {
    parents(root, path)?;
    let at = root.join(OsStr::from_bytes(path));
    match mode {
        Mode::Link => std::os::unix::fs::symlink(OsStr::from_bytes(bytes), &at).map_err(io(&at)),
        Mode::Submodule => fs::create_dir(&at).map_err(io(&at)),
        Mode::File | Mode::Executable => {
            let bits = if mode == Mode::Executable { 0o755 } else { 0o644 };
            let mut file = fs::File::options().write(true).create_new(true).mode(bits).open(&at).map_err(io(&at))?;
            file.write_all(bytes).map_err(io(&at))?;
            file.set_permissions(fs::Permissions::from_mode(bits)).map_err(io(&at))
        }
    }
}

/// REQ-RUN-3: writes `commit`'s tree into `work`, an empty directory, from the objects' bytes, with no filter,
/// conversion or checkout: links as links with their targets as stored, never followed; the executable bit as the
/// tree records it; a submodule as an empty directory; and no entry with a protected component, at any depth. A
/// failure is `TOOL_FAILED`.
pub fn export(repo: &Repo, commit: &str, work: &Path) -> Result<(), Error> {
    object(commit)?;
    let entries = tree(repo, commit)?;
    let blobs: Vec<&Entry> = entries.values().filter(|e| e.mode != Mode::Submodule).collect();
    let mut bytes = contents(repo, &blobs)?.into_iter();
    for (path, e) in &entries {
        let content = if e.mode == Mode::Submodule { Vec::new() } else { bytes.next().unwrap_or_default() };
        write_entry(work, path, e.mode, &content)?;
    }
    Ok(())
}

/// Paths as bytes, which is how a copy names its entries.
type Paths<T> = BTreeMap<Vec<u8>, T>;

/// What the walk found at a path that is not a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Found {
    File { executable: bool },
    Link(Vec<u8>),
    Other,
}

/// Walks the copy at `work` itself, following nothing (3.8 rule 2): a directory is entered, made its owner's again
/// first, since a cell may have closed it; a file's mode and a link's target as written are read; anything else is
/// noted and never opened. An entry with a protected name is `PROTECTED_PATH`, before any Git command sees the copy
/// (rule 3). Gives every entry that is not a directory, and the empty directories.
fn walk(work: &Path) -> Result<(Paths<Found>, BTreeSet<Vec<u8>>), Error> {
    let (mut found, mut empty, mut dirs) = (BTreeMap::new(), BTreeSet::new(), vec![Vec::new()]);
    while let Some(rel) = dirs.pop() {
        let dir = if rel.is_empty() { work.to_path_buf() } else { work.join(OsStr::from_bytes(&rel)) };
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).map_err(io(&dir))?;
        let mut none = true;
        for entry in fs::read_dir(&dir).map_err(io(&dir))? {
            let entry = entry.map_err(io(&dir))?;
            none = false;
            let name = entry.file_name().into_vec();
            let path = if rel.is_empty() { name.clone() } else { [&rel[..], b"/", &name].concat() };
            if protected(&name) {
                let why = format!("the copy holds {}, a name no candidate may hold", shown(&path));
                return Err(refuse(Code::ProtectedPath, None, why));
            }
            let meta = fs::symlink_metadata(entry.path()).map_err(io(&entry.path()))?;
            let kind = meta.file_type();
            if kind.is_dir() {
                dirs.push(path);
            } else if kind.is_symlink() {
                found.insert(
                    path,
                    Found::Link(fs::read_link(entry.path()).map_err(io(&entry.path()))?.into_os_string().into_vec()),
                );
            } else if kind.is_file() {
                let mode = meta.permissions().mode();
                // Made readable by its owner, as a closed directory is; its executable bit is kept.
                let readable = mode | 0o400;
                if readable != mode {
                    fs::set_permissions(entry.path(), fs::Permissions::from_mode(readable))
                        .map_err(io(&entry.path()))?;
                }
                found.insert(path, Found::File { executable: mode & 0o100 != 0 });
            } else {
                found.insert(path, Found::Other);
            }
        }
        if none && !rel.is_empty() {
            empty.insert(rel);
        }
    }
    Ok((found, empty))
}

/// The paths of `new` that the base's ignore rules exclude (3.8 rule 5): the base tree's own `.gitignore` files,
/// written apart under `scratch/rules` and read there by `git check-ignore` with no other source of rules, case
/// sensitively; never the copy's, nor the person's own exclude files.
fn ignored(
    repo: &Repo,
    base: &BTreeMap<Vec<u8>, Entry>,
    scratch: &Path,
    new: &[&[u8]],
) -> Result<BTreeSet<Vec<u8>>, Error> {
    let rules: Vec<(&Vec<u8>, &Entry)> = base
        .iter()
        .filter(|(p, e)| {
            matches!(e.mode, Mode::File | Mode::Executable) && p.rsplit(|&b| b == b'/').next() == Some(b".gitignore")
        })
        .collect();
    if rules.is_empty() || new.is_empty() {
        return Ok(BTreeSet::new());
    }
    let dir = scratch.join("rules");
    // Made new at every reading, so that no rule of an earlier one is left.
    let _ = fs::remove_dir_all(&dir);
    let mut init = os(&["init", "-q", "--template="]);
    init.push(dir.as_os_str());
    git::git(repo, &init, b"", None)?;
    for ((path, _), bytes) in rules.iter().zip(contents(repo, &rules.iter().map(|r| r.1).collect::<Vec<_>>())?) {
        write_entry(&dir, path, Mode::File, &bytes)?;
    }
    let input: Vec<u8> = new.iter().flat_map(|p| [*p, b"\0"].concat()).collect();
    let args = ["-c", "core.excludesFile=/dev/null", "-c", "core.ignoreCase=false", "check-ignore", "--no-index"];
    let mut args = os(&args);
    args.extend(os(&["--stdin", "-z"]));
    let rules_repo = Repo { root: dir, prefix: PathBuf::new() };
    let out = git::run(git::command(&rules_repo, &args), &input)?;
    match out.status.code() {
        Some(0) => Ok(out.stdout.split(|&b| b == 0).filter(|p| !p.is_empty()).map(<[u8]>::to_vec).collect()),
        Some(1) => Ok(BTreeSet::new()),
        _ => Err(failed(Some(&repo.root), "git check-ignore failed")),
    }
}

/// `bytes` quoted as C quotes it, every byte as its octal escape, which `git hash-object --stdin-paths` reads back
/// exactly, whatever a name holds.
fn quoted(bytes: &[u8]) -> Vec<u8> {
    let mut out = vec![b'"'];
    for &b in bytes {
        out.extend(format!("\\{b:03o}").bytes());
    }
    out.extend(b"\"\n");
    out
}

/// The blob identifiers of `files`, hashed as their bytes with no filter or conversion, and written into the object
/// store only when `write`.
fn hash(repo: &Repo, files: &[PathBuf], write: bool) -> Result<Vec<String>, Error> {
    if files.is_empty() {
        return Ok(Vec::new());
    }
    let input: Vec<u8> = files.iter().flat_map(|f| quoted(f.as_os_str().as_bytes())).collect();
    let args = if write {
        os(&["hash-object", "-w", "--no-filters", "--stdin-paths"])
    } else {
        os(&["hash-object", "--no-filters", "--stdin-paths"])
    };
    let ids: Vec<String> = line(git::git(repo, &args, &input, None)?).split('\n').map(str::to_owned).collect();
    if ids.len() != files.len() {
        return Err(failed(Some(&repo.root), "git hash-object gave another count"));
    }
    Ok(ids)
}

/// What a candidate may change (REQ-TASK-7, REQ-GATE-9): the task's allowed files, the protected names and the
/// protected paths in force.
pub struct Lanes<'a> {
    pub allowed: &'a [&'a str],
    pub names: &'a [String],
    pub paths: &'a [String],
}

/// Whether `path` reaches a protected name (a component equal to one, under ASCII case folding) or a protected path:
/// the path itself; one under a protected directory, written with a final `/` (`/` alone is the whole tree); or one
/// above it, whose change (a link or a file in a directory's place) moves what the directory holds.
fn guarded(path: &str, lanes: &Lanes) -> bool {
    let named = path.split('/').any(|c| lanes.names.iter().any(|n| n.eq_ignore_ascii_case(c)));
    let below = |a: &str, b: &str| a.strip_prefix(b).is_some_and(|r| r.starts_with('/'));
    named
        || lanes.paths.iter().any(|q| match q.strip_suffix('/') {
            Some("") => true,
            Some(dir) => path == dir || below(path, dir) || below(dir, path),
            None => path == q,
        })
}

/// REQ-RUN-3, REQ-GATE-9 and REQ-SEC-3 on a copy whose cell has ended (3.8 rule 1): the change set from `base` to
/// what `work` holds, the base's ignored new paths left out, each change a file, an executable bit, a link or a
/// deletion. Its guards, in order: what the walk refuses (`PROTECTED_PATH`); an entry that is neither a file, a
/// link nor a directory outside the ignored paths, a changed path that is not valid text, or one no allowed file
/// matches (`LANE_VIOLATION`); a change that reaches a protected name or path and matches no allowed file written
/// without a wildcard (`PROTECTED_PATH_WILDCARD`); a change Git refuses to hold in a tree (`PROTECTED_PATH`, P9B-1);
/// a changed file that looks like a credential, by its name or, for a file, its content (`CREDENTIAL_SUSPECTED`).
/// Nothing is written into the object store.
pub fn changes(repo: &Repo, base: &str, work: &Path, scratch: &Path, lanes: &Lanes) -> Result<Vec<Change>, Error> {
    object(base)?;
    let (found, empty) = walk(work)?;
    let tree = tree(repo, base)?;
    let new: Vec<&[u8]> = found.keys().filter(|p| !tree.contains_key(*p)).map(Vec::as_slice).collect();
    let ignored = ignored(repo, &tree, scratch, &new)?;
    let kept: Vec<(&Vec<u8>, &Found)> = found.iter().filter(|(p, _)| !ignored.contains(*p)).collect();
    if let Some((p, _)) = kept.iter().find(|(_, f)| **f == Found::Other) {
        let why = format!("the copy leaves {}, which is not a file, a link or a directory", shown(p));
        return Err(refuse(Code::LaneViolation, None, why));
    }
    let after = entries_of(repo, work, scratch, &kept, false)?;
    let mut paths: BTreeSet<&Vec<u8>> = tree.keys().collect();
    paths.extend(after.keys());
    let mut out = Vec::new();
    for p in paths {
        let before = tree.get(p).cloned();
        let now = match &before {
            Some(e) if e.mode == Mode::Submodule && empty.contains(p) => before.clone(),
            _ => after.get(p).cloned(),
        };
        if before == now {
            continue;
        }
        let text = String::from_utf8(p.clone()).map_err(|_| {
            refuse(Code::LaneViolation, None, format!("the copy changes {}, a path that is not valid text", shown(p)))
        })?;
        out.push(Change { path: text, before, after: now });
    }
    held_to(&out, lanes)?;
    let check = scratch.join("check-index");
    let dropped = stage(repo, base, &check, &out);
    let _ = fs::remove_file(&check);
    if let Some(c) = dropped? {
        let why = format!("the copy leaves {}, a path Git refuses to hold in a tree", shown(c.path.as_bytes()));
        return Err(refuse(Code::ProtectedPath, None, why));
    }
    for c in out.iter().filter(|c| c.after.is_some()) {
        let name = c.path.rsplit('/').next().unwrap_or_default();
        let file = work.join(&c.path);
        let content = match c.after.as_ref().map(|e| e.mode) {
            Some(Mode::File | Mode::Executable) => scan::suspect(&fs::read(&file).map_err(io(&file))?),
            _ => None,
        };
        if let Some(why) = scan::suspect_name(name).then_some("its name").or(content) {
            let why = format!(
                "{} looks like a credential ({why}); a heuristic, a cooperative control",
                shown(c.path.as_bytes())
            );
            return Err(refuse(Code::CredentialSuspected, None, why));
        }
    }
    Ok(out)
}

/// The lanes of a change set (REQ-TASK-7, REQ-GATE-9): every change matches an allowed file (`LANE_VIOLATION`), and
/// one that reaches a protected name or path matches one written without a wildcard (`PROTECTED_PATH_WILDCARD`).
pub(crate) fn held_to(out: &[Change], lanes: &Lanes) -> Result<(), Error> {
    let patterns: Vec<(Vec<&str>, bool)> = lanes
        .allowed
        .iter()
        .map(|a| task::pattern(a).map(|s| (s, !a.contains(['*', '?']))))
        .collect::<Result<_, _>>()?;
    let matched =
        |path: &str, literal: bool| patterns.iter().any(|(s, l)| (*l || !literal) && task::matches(s, path, &mut 0));
    if let Some(c) = out.iter().find(|c| !matched(&c.path, false)) {
        let why = format!("the copy changes {}, which the allowed files do not match", shown(c.path.as_bytes()));
        return Err(refuse(Code::LaneViolation, None, why));
    }
    if let Some(c) = out.iter().find(|c| guarded(&c.path, lanes) && !matched(&c.path, true)) {
        let why = format!(
            "the copy changes {}, protected: list it literally among the allowed files",
            shown(c.path.as_bytes())
        );
        return Err(refuse(Code::ProtectedPathWildcard, None, why));
    }
    Ok(())
}

/// The entries of the files and links `kept` found in `work`, by path: each hashed as its bytes, a link's target
/// written apart under `scratch/links/` to be hashed; written into the object store only when `write`.
fn entries_of(
    repo: &Repo,
    work: &Path,
    scratch: &Path,
    kept: &[(&Vec<u8>, &Found)],
    write: bool,
) -> Result<BTreeMap<Vec<u8>, Entry>, Error> {
    let links = scratch.join("links");
    let _ = fs::remove_dir_all(&links);
    fs::create_dir(&links).map_err(io(&links))?;
    let (mut files, mut modes) = (Vec::new(), Vec::new());
    for (i, (path, f)) in kept.iter().enumerate() {
        let (file, mode) = match f {
            Found::File { executable } => {
                (work.join(OsStr::from_bytes(path)), if *executable { Mode::Executable } else { Mode::File })
            }
            Found::Link(target) => {
                let file = links.join(i.to_string());
                fs::write(&file, target).map_err(io(&file))?;
                (file, Mode::Link)
            }
            Found::Other => continue,
        };
        files.push(file);
        modes.push(((*path).clone(), mode));
    }
    let ids = hash(repo, &files, write)?;
    Ok(modes.into_iter().zip(ids).map(|((p, mode), oid)| (p, Entry { mode, oid })).collect())
}

/// Who an agent's candidate names as co-author (REQ-RUN-4): its tool, provider and model, as the adapter observed
/// them.
pub struct Agent<'a> {
    pub tool: &'a str,
    pub provider: &'a str,
    pub model: &'a str,
}

/// A candidate: its commit and its tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub commit: String,
    pub tree: String,
}

/// The message of a candidate: its subject names the task; one `Co-authored-by` trailer names the agent, at an
/// address no mail reaches; with `sign_off`, a `Signed-off-by` trailer names the person whose Git commits it. A name
/// the trailer cannot carry (empty, longer than 100 bytes, or with a character outside printable ASCII, `<`, `>`,
/// `(`, `)` and `,`) is `TOOL_FAILED`.
fn message(repo: &Repo, task: &str, agent: &Agent, sign_off: bool) -> Result<String, Error> {
    for name in [agent.tool, agent.provider, agent.model] {
        let fits = |b: u8| (b' '..=b'~').contains(&b) && !b"<>(),".contains(&b);
        if name.trim().is_empty() || name.len() > 100 || !name.bytes().all(fits) {
            return Err(failed(None, format!("the agent's name {name:?} cannot be carried by a trailer")));
        }
    }
    let mut text = format!("fabrewk: candidate for {task}\n\n");
    text +=
        &format!("Co-authored-by: {} ({}, {}) <noreply@fabrewk.invalid>\n", agent.tool, agent.provider, agent.model);
    if sign_off {
        let ident = line(git::git(repo, &os(&["var", "GIT_COMMITTER_IDENT"]), b"", None)?);
        // The identity without its time and zone, its last two fields.
        let who = ident
            .rsplitn(3, ' ')
            .nth(2)
            .filter(|w| w.ends_with('>'))
            .ok_or_else(|| failed(Some(&repo.root), "Git names no committer"))?;
        text += &format!("Signed-off-by: {who}\n");
    }
    Ok(text)
}

/// REQ-RUN-4: commits `changes`, found in `work` by [`changes`], as the candidate of `task` (its identifier): a
/// commit whose only parent is `base`, built through a temporary index under `scratch` from the base's tree with each
/// change's object (written now, and refused `TOOL_FAILED` if it is no longer what the change set found), its message
/// as [`message`] makes it, and the branch `fabrewk/<task>` moved to it from `previous` (absent when `None`) by
/// compare-and-swap. The person's index and working tree are untouched.
#[allow(clippy::too_many_arguments)]
pub fn commit(
    repo: &Repo,
    base: &str,
    changes: &[Change],
    work: &Path,
    scratch: &Path,
    task: &str,
    agent: &Agent,
    sign_off: bool,
    previous: Option<&str>,
) -> Result<Candidate, Error> {
    object(base)?;
    if !crate::record::is_id(task, 't') {
        return Err(failed(None, format!("{task:?} is no task's identifier")));
    }
    if let Some(p) = previous {
        object(p)?;
    }
    let text = message(repo, task, agent, sign_off)?;
    let mut kept = Vec::new();
    for c in changes.iter().filter(|c| c.after.is_some()) {
        let at = work.join(&c.path);
        let meta = fs::symlink_metadata(&at).map_err(io(&at))?;
        let found = if meta.file_type().is_symlink() {
            Found::Link(fs::read_link(&at).map_err(io(&at))?.into_os_string().into_vec())
        } else {
            Found::File { executable: meta.permissions().mode() & 0o100 != 0 }
        };
        kept.push((c.path.as_bytes().to_vec(), found));
    }
    let refs: Vec<(&Vec<u8>, &Found)> = kept.iter().map(|(p, f)| (p, f)).collect();
    let index = scratch.join("candidate-index");
    let result = (|| {
        // The index is checked before any object is written, so that nothing Git refuses reaches the store.
        if let Some(c) = stage(repo, base, &index, changes)? {
            return Err(failed(Some(&work.join(&c.path)), "Git would not hold this path in the candidate"));
        }
        let written = entries_of(repo, work, scratch, &refs, true)?;
        let moved = |c: &&Change| c.after.as_ref().is_some_and(|e| written.get(c.path.as_bytes()) != Some(e));
        if let Some(c) = changes.iter().find(moved) {
            return Err(failed(Some(&work.join(&c.path)), "the copy changed after its change set was read"));
        }
        let tree = line(git::git(repo, &os(&["write-tree"]), b"", Some(&index))?);
        let commit = line(git::git(repo, &os(&["commit-tree", &tree, "-p", base, "-F", "-"]), text.as_bytes(), None)?);
        let branch = format!("refs/heads/fabrewk/{task}");
        let moved = git::run(
            git::command(
                repo,
                &os(&["update-ref", "-m", "fabrewk: candidate", &branch, &commit, previous.unwrap_or_default()]),
            ),
            b"",
        )?;
        if !moved.status.success() {
            return Err(failed(Some(&repo.root), format!("fabrewk/{task} is not where the record says it is")));
        }
        Ok(Candidate { commit, tree })
    })();
    let _ = fs::remove_file(&index);
    result
}

/// Writes `changes` over `base`'s tree into a new index at `index` and gives the first change it does not hold as the
/// change says: Git leaves out a path it refuses to hold (a `.gitmodules` that is a link, a name Windows or macOS
/// reads as one, under `core.protectNTFS` or `core.protectHFS`) and still succeeds (P9B-1), so what it holds is read
/// back. The objects need not exist yet.
fn stage<'c>(repo: &Repo, base: &str, index: &Path, changes: &'c [Change]) -> Result<Option<&'c Change>, Error> {
    let _ = fs::remove_file(index);
    let zero = "0".repeat(base.len());
    let info: String = changes
        .iter()
        .map(|c| match &c.after {
            None => format!("0 {zero}\t{}\0", c.path),
            Some(e) => format!("{} {}\t{}\0", e.mode.git(), e.oid, c.path),
        })
        .collect();
    git::git(repo, &os(&["read-tree", base]), b"", Some(index))?;
    git::git(repo, &os(&["update-index", "-z", "--index-info"]), info.as_bytes(), Some(index))?;
    let listed = git::git(repo, &os(&["ls-files", "-s", "-z"]), b"", Some(index))?;
    // Each entry is `<mode> <object> <stage>\t<path>`.
    let held: BTreeMap<&[u8], &[u8]> = listed
        .split(|&b| b == 0)
        .filter_map(|r| r.iter().position(|&b| b == b'\t').map(|t| (&r[t + 1..], &r[..t])))
        .collect();
    Ok(changes.iter().find(|c| {
        let got = held.get(c.path.as_bytes()).copied();
        let want = c.after.as_ref().map(|e| format!("{} {} 0", e.mode.git(), e.oid));
        got != want.as_ref().map(|w| w.as_bytes())
    }))
}

/// The protected paths in force (REQ-GATE-9, S-4): `gate.protectedPaths`, with the directory the repository's own
/// `core.hooksPath` names when it lies inside the working tree, read now, at each guard.
pub fn protected_paths(eff: &Effective, repo: &Repo) -> Result<Vec<String>, Error> {
    let mut paths = match eff.values.get("gate.protectedPaths") {
        Some((Val::List(l), _)) => l.clone(),
        _ => Vec::new(),
    };
    paths.extend(git::hooks_dir(repo)?);
    Ok(paths)
}

/// The protected names in force (REQ-GATE-9, S-4): `gate.protectedNames`.
pub fn protected_names(eff: &Effective) -> Vec<String> {
    match eff.values.get("gate.protectedNames") {
        Some((Val::List(l), _)) => l.clone(),
        _ => Vec::new(),
    }
}
