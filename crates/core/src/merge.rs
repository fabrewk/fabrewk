//! Integration as a library (part 9g; REQ-GATE-8, REQ-GATE-10, S-4 and S-11 at integration; D-087): a commit's
//! product tree, the change set between two commits and its third guard, what the index and the working tree hold at
//! the touched paths, the integration commit and the move by compare-and-swap, and the working tree and the index
//! brought to the candidate from the objects' bytes, with no filter, conversion or checkout command (3.8 rule 9).

use crate::candidate::{self, Change, Entry, Lanes, Mode};
use crate::code::{Code, Error, io_at, refuse};
use crate::record::Repo;
use crate::record::git::{self, line, os};
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// The product tree of `commit` (REQ-GATE-8): its root's entries but `.brewstillery`, as `ls-tree` lists them.
pub fn product_tree(repo: &Repo, commit: &str) -> Result<Vec<u8>, Error> {
    let listed = git::git(repo, &os(&["ls-tree", "-z", "--full-tree", commit]), b"", None)?;
    let entries = listed.split(|b| *b == 0).filter(|e| !e.ends_with(b"\t.brewstillery"));
    Ok(entries.flat_map(|e| e.iter().copied().chain([0])).collect())
}

fn entry(mode: &str, oid: &str) -> Option<Entry> {
    let mode = match mode {
        "100644" => Mode::File,
        "100755" => Mode::Executable,
        "120000" => Mode::Link,
        "160000" => Mode::Submodule,
        _ => return None,
    };
    Some(Entry { mode, oid: oid.to_owned() })
}

/// REQ-GATE-8's third guard: the change set from `base` to `candidate`, recomputed from their whole trees, under
/// REQ-RUN-3's rules: a path with a protected component, `.git` or `.brewstillery` as Git or a file system it protects
/// reads it, or an empty, `.` or `..` one (`PROTECTED_PATH`); a path that is not valid text (`LANE_VIOLATION`); then the lanes (`LANE_VIOLATION`,
/// `PROTECTED_PATH_WILDCARD`), independent of the change set's guard and the lane guard before it.
pub fn changes(repo: &Repo, base: &str, candidate: &str, lanes: &Lanes) -> Result<Vec<Change>, Error> {
    candidate::object(base)?;
    candidate::object(candidate)?;
    let args = os(&["diff-tree", "-r", "-z", "--no-renames", "--no-abbrev", "--raw", base, candidate]);
    let raw = git::git(repo, &args, b"", None)?;
    let mut fields = raw.split(|&b| b == 0).filter(|f| !f.is_empty());
    let mut out = Vec::new();
    while let (Some(meta), Some(path)) = (fields.next(), fields.next()) {
        let meta = String::from_utf8_lossy(meta.strip_prefix(b":").unwrap_or(meta)).into_owned();
        let m: Vec<&str> = meta.split(' ').collect();
        let [before, after, old, new, ..] = m[..] else {
            return Err(refuse(Code::ToolFailed, Some(&repo.root), "git diff-tree gave what it does not write"));
        };
        if path.split(|&b| b == b'/').any(|c| matches!(c, b"" | b"." | b"..") || candidate::protected(c)) {
            let why = format!("the candidate changes {}, a protected path", candidate::shown(path));
            return Err(refuse(Code::ProtectedPath, None, why));
        }
        let text = String::from_utf8(path.to_vec()).map_err(|_| {
            let why = format!("the candidate changes {}, a path that is not valid text", candidate::shown(path));
            refuse(Code::LaneViolation, None, why)
        })?;
        out.push(Change { path: text, before: entry(before, old), after: entry(after, new) });
    }
    candidate::held_to(&out, lanes)?;
    Ok(out)
}

/// What a touched path holds in the index or the working tree: nothing, an entry, or what no tree's entry is (an
/// unmerged entry, a directory, a path below a link or a file, a special file).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Held {
    Absent,
    Entry(Entry),
    Other,
}

impl Held {
    /// Whether it holds what `e` says.
    pub fn is(&self, e: &Option<Entry>) -> bool {
        *self == e.clone().map_or(Held::Absent, Held::Entry)
    }
}

fn submodule(e: &Option<Entry>) -> bool {
    e.as_ref().is_some_and(|e| e.mode == Mode::Submodule)
}

/// What the index and the working tree hold at each of `changes`' paths, in order, read without the index's stat data
/// and with no filter: each file hashed as its bytes, a link's target as written (put under `scratch` to be hashed).
pub fn held(repo: &Repo, changes: &[Change], scratch: &Path) -> Result<Vec<(Held, Held)>, Error> {
    let listed = git::git(repo, &os(&["ls-files", "-s", "-z"]), b"", None)?;
    let mut index: BTreeMap<&[u8], Held> = BTreeMap::new();
    for r in listed.split(|&b| b == 0).filter(|r| !r.is_empty()) {
        let tab = r.iter().position(|&b| b == b'\t').unwrap_or(r.len());
        let meta: Vec<&str> = std::str::from_utf8(&r[..tab]).unwrap_or_default().split(' ').collect();
        let held = match meta[..] {
            [mode, oid, "0"] => entry(mode, oid).map_or(Held::Other, Held::Entry),
            _ => Held::Other,
        };
        index.entry(r.get(tab + 1..).unwrap_or_default()).and_modify(|h| *h = Held::Other).or_insert(held);
    }
    fs::create_dir_all(scratch).map_err(io_at(scratch))?;
    // A file and a directory may trade places: a path below a touched one that is not a directory holds nothing, and
    // a directory at a touched path above others holds no entry of its own; each of those paths is read on its own.
    let touched: std::collections::BTreeSet<&str> = changes.iter().map(|c| c.path.as_str()).collect();
    let above = |p: &str| touched.iter().any(|t| t.strip_prefix(p).is_some_and(|r| r.starts_with('/')));
    let (mut work, mut hash, mut modes) = (Vec::new(), Vec::new(), Vec::new());
    for (i, c) in changes.iter().enumerate() {
        let parts: Vec<&str> = c.path.split('/').collect();
        let mut at = repo.root.clone();
        let mut held = None;
        for (n, p) in parts[..parts.len() - 1].iter().enumerate() {
            at.push(p);
            match fs::symlink_metadata(&at) {
                Ok(m) if m.is_dir() => {}
                Ok(_) if touched.contains(parts[..=n].join("/").as_str()) => held = Some(Held::Absent),
                Ok(_) => held = Some(Held::Other),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => held = Some(Held::Absent),
                Err(e) => return Err(io_at(&at)(e)),
            }
            if held.is_some() {
                break;
            }
        }
        at = repo.root.join(&c.path);
        let held = match held {
            Some(h) => h,
            None => match fs::symlink_metadata(&at) {
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Held::Absent,
                Err(e) => return Err(io_at(&at)(e)),
                Ok(m) if m.is_file() => {
                    hash.push(at.clone());
                    modes.push((i, if m.permissions().mode() & 0o100 != 0 { Mode::Executable } else { Mode::File }));
                    Held::Other
                }
                Ok(m) if m.file_type().is_symlink() => {
                    let file = scratch.join(format!("link-{i}"));
                    fs::write(&file, fs::read_link(&at).map_err(io_at(&at))?.as_os_str().as_bytes())
                        .map_err(io_at(&file))?;
                    hash.push(file);
                    modes.push((i, Mode::Link));
                    Held::Other
                }
                Ok(m) if m.is_dir() && above(&c.path) => Held::Absent,
                Ok(_) => Held::Other,
            },
        };
        work.push(held);
    }
    if !hash.is_empty() {
        let mut args = os(&["hash-object", "--no-filters", "--"]);
        args.extend(hash.iter().map(|p| p.as_os_str()));
        let ids = line(git::git(repo, &args, b"", None)?);
        for ((i, mode), oid) in modes.into_iter().zip(ids.split('\n')) {
            work[i] = Held::Entry(Entry { mode, oid: oid.to_owned() });
        }
    }
    // An entry the path would displace in the index, one below it or a file above it, each one that is no touched path,
    // is the person's: the path holds no entry of the base's or the candidate's (P9G-5, P9G-7).
    let displaced = |p: &str| {
        let below = format!("{p}/");
        let untouched = |k: &[u8]| !std::str::from_utf8(k).is_ok_and(|k| touched.contains(k));
        let mut under = index.range(below.as_bytes()..).take_while(|(k, _)| k.starts_with(below.as_bytes()));
        let parts: Vec<&str> = p.split('/').collect();
        let mut over = (1..parts.len()).map(|n| parts[..n].join("/"));
        under.any(|(k, _)| untouched(k)) || over.any(|a| index.contains_key(a.as_bytes()) && untouched(a.as_bytes()))
    };
    let index = changes.iter().map(|c| match displaced(&c.path) {
        true => Held::Other,
        false => index.get(c.path.as_bytes()).cloned().unwrap_or(Held::Absent),
    });
    Ok(index.zip(work).collect())
}

/// The first touched path whose index or working tree holds neither the base's content, when `base` admits it, nor the
/// candidate's, when `candidate` does (REQ-GATE-8's "clean"): `WORKTREE_DIRTY` naming it. A submodule's path in the
/// working tree is Git's own checkout, a directory or none, which the integration never writes.
pub fn clean(changes: &[Change], held: &[(Held, Held)], base: bool, candidate: bool) -> Result<(), Error> {
    let fits = |h: &Held, c: &Change| (base && h.is(&c.before)) || (candidate && h.is(&c.after));
    let sub =
        |h: &Held, c: &Change| (submodule(&c.before) || submodule(&c.after)) && matches!(h, Held::Other | Held::Absent);
    match changes.iter().zip(held).find(|(c, (i, w))| !fits(i, c) || !(fits(w, c) || sub(w, c))) {
        Some((c, _)) => Err(dirty(c)),
        None => Ok(()),
    }
}

fn dirty(c: &Change) -> Error {
    let why = format!("{} holds an edit the integration would overwrite", candidate::shown(c.path.as_bytes()));
    refuse(Code::WorktreeDirty, None, why)
}

/// The integration commit (REQ-GATE-8): `head`'s tree with `changes` applied, through a temporary index at `index`,
/// its only parent `head`, its message the candidate's with a `Fabrewk-Candidate` trailer; its product tree must be
/// the candidate's.
pub fn commit(repo: &Repo, head: &str, candidate: &str, changes: &[Change], index: &Path) -> Result<String, Error> {
    let info = index_info(
        &changes.iter().collect::<Vec<_>>(),
        changes.first().and_then(|c| c.before.as_ref().or(c.after.as_ref())),
    );
    let _ = fs::remove_file(index);
    let made = (|| {
        git::git(repo, &os(&["read-tree", head]), b"", Some(index))?;
        git::git(repo, &os(&["update-index", "-z", "--index-info"]), info.as_bytes(), Some(index))?;
        git::git(repo, &os(&["write-tree"]), b"", Some(index)).map(line)
    })();
    let _ = fs::remove_file(index);
    let tree = made?;
    let message = String::from_utf8_lossy(&git::git(repo, &os(&["show", "-s", "--format=%B", candidate]), b"", None)?)
        .trim_end()
        .to_owned();
    let message = format!("{message}\nFabrewk-Candidate: {candidate}\n");
    let commit = line(git::git(repo, &os(&["commit-tree", &tree, "-p", head, "-F", "-"]), message.as_bytes(), None)?);
    if product_tree(repo, &commit)? != product_tree(repo, candidate)? {
        return Err(refuse(
            Code::ToolFailed,
            Some(&repo.root),
            "the integration commit's product is not the candidate's",
        ));
    }
    Ok(commit)
}

/// What `update-index --index-info` reads to make `changes`: removals first, so that a file and a directory may trade
/// places, each removal named by the zero object of the repository's length, which `sample` shows.
fn index_info(changes: &[&Change], sample: Option<&Entry>) -> String {
    let zero = "0".repeat(sample.map_or(40, |e| e.oid.len()));
    let (gone, new): (Vec<&&Change>, Vec<&&Change>) = changes.iter().partition(|c| c.after.is_none());
    let gone = gone.into_iter().map(|c| format!("0 {zero}\t{}\0", c.path));
    let new =
        new.into_iter().filter_map(|c| c.after.as_ref().map(|e| format!("{} {}\t{}\0", e.mode.git(), e.oid, c.path)));
    gone.chain(new).collect()
}

/// Moves `branch` from `old` to `new` by one compare-and-swap (REQ-GATE-8); a branch another Git moved meanwhile is
/// `BASE_MOVED`, the branch untouched.
pub fn move_branch(repo: &Repo, branch: &str, old: &str, new: &str) -> Result<(), Error> {
    let args = os(&["update-ref", "-m", "fabrewk: integrate", branch, new, old]);
    let out = git::run(git::command(repo, &args), b"")?;
    if out.status.success() {
        return Ok(());
    }
    // What Git printed is kept whatever the cause (D-083, P9G-4).
    let failed = git::failed(repo, &args, &out);
    if git::head_of(repo, branch)?.as_deref() != Some(old) {
        let why = format!("the record branch moved while the integration ran: {}", failed.detail());
        return Err(refuse(Code::BaseMoved, None, why));
    }
    Err(failed)
}

/// Brings the working tree and the index to the candidate at each touched path (REQ-GATE-8, S-11) that holds the
/// base's content, as `held` read it: removals first, each a file or a link, never a directory, then the directories
/// it emptied; then each new or changed entry written from its object's bytes under `scratch` and renamed into place,
/// following no link; then the index, by `update-index --index-info`. A path that already holds the candidate's is left;
/// none holds anything else, which [`clean`] refused.
pub fn bring(repo: &Repo, changes: &[Change], held: &[(Held, Held)], scratch: &Path) -> Result<(), Error> {
    let todo: Vec<(&Change, &(Held, Held))> = changes.iter().zip(held).collect();
    // Each path is read again just before it is written, so that an edit made since `held` read it is never
    // overwritten (P9G-1); what remains is the instant between that reading and the write, a cooperative control.
    let unchanged = |c: &Change, was: &Held| -> Result<(), Error> {
        let now = self::held(repo, std::slice::from_ref(c), scratch)?;
        if now[0].1 == *was { Ok(()) } else { Err(dirty(c)) }
    };
    let mut gone: Vec<(&Change, &(Held, Held))> = todo
        .iter()
        .filter(|(c, (_, w))| c.after.is_none() && !submodule(&c.before) && !w.is(&c.after))
        .copied()
        .collect();
    gone.sort_by(|a, b| b.0.path.cmp(&a.0.path));
    for (c, (_, was)) in gone {
        unchanged(c, was)?;
        let at = repo.root.join(&c.path);
        crate::record::fault::point();
        fs::remove_file(&at).map_err(io_at(&at))?;
        let mut dir = at.parent();
        while let Some(d) = dir.filter(|d| *d != repo.root && fs::remove_dir(d).is_ok()) {
            dir = d.parent();
        }
    }
    let new: Vec<(&Change, &(Held, Held))> = todo
        .iter()
        .filter(|(c, (_, w))| c.after.is_some() && !w.is(&c.after) && (!submodule(&c.after) || *w == Held::Absent))
        .copied()
        .collect();
    let files: Vec<&Entry> =
        new.iter().filter_map(|(c, _)| c.after.as_ref()).filter(|e| e.mode != Mode::Submodule).collect();
    let mut bytes = candidate::contents(repo, &files)?.into_iter();
    fs::create_dir_all(scratch).map_err(io_at(scratch))?;
    for (c, (_, was)) in new {
        let e = c.after.as_ref().expect("a change that writes");
        candidate::parents(&repo.root, c.path.as_bytes())?;
        let at = repo.root.join(&c.path);
        if e.mode == Mode::Submodule {
            fs::create_dir(&at).map_err(io_at(&at))?;
            continue;
        }
        let tmp = scratch.join("entry");
        let _ = fs::remove_file(&tmp);
        let body = bytes.next().unwrap_or_default();
        let written = match e.mode {
            Mode::Link => std::os::unix::fs::symlink(OsStr::from_bytes(&body), &tmp),
            _ => fs::write(&tmp, &body).and_then(|()| {
                let bits = if e.mode == Mode::Executable { 0o755 } else { 0o644 };
                fs::set_permissions(&tmp, fs::Permissions::from_mode(bits))
            }),
        };
        written.map_err(io_at(&tmp))?;
        crate::record::fault::point();
        unchanged(c, was)?;
        fs::rename(&tmp, &at).map_err(io_at(&at))?;
    }
    let pending: Vec<(&Change, &(Held, Held))> = todo.iter().filter(|(c, (i, _))| !i.is(&c.after)).copied().collect();
    let owned: Vec<Change> = pending.iter().map(|(c, _)| (*c).clone()).collect();
    let now = self::held(repo, &owned, scratch)?;
    if let Some(((c, _), _)) = pending.iter().zip(now).find(|((_, (was, _)), (now, _))| now != was) {
        return Err(dirty(c));
    }
    let pending: Vec<&Change> = pending.into_iter().map(|t| t.0).collect();
    let info = index_info(&pending, changes.first().and_then(|c| c.before.as_ref().or(c.after.as_ref())));
    if !info.is_empty() {
        crate::record::fault::point();
        git::git(repo, &os(&["update-index", "-z", "--index-info"]), info.as_bytes(), None)?;
    }
    Ok(())
}
