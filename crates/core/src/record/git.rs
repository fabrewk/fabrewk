//! The record in Git (REQ-REG-8's history walk, REQ-REG-10, REQ-REG-11). Every object is read as stored: replacement
//! objects, grafts and the commit graph are off for every call. Every call names a hooks directory under which no file
//! can exist and turns the file-system monitor off, so no hook a repository or a candidate holds runs as Fabrewk's
//! (S-11). The record commit is built through a temporary index, so the person's staged and unstaged changes stay as
//! they were, and the branch moves by compare-and-swap. A Git command that fails is named with its exit status; in a
//! write command its standard error and exit status are kept as evidence, which the words name (D-083, D-086).

use super::replay::list_stream;
use super::{Code, Error, fault, io_at, refuse};
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// Where the record is committed (REQ-DOM-7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Publication {
    Published,
    OwnRepository,
    Unpublished,
}

/// The repository that holds the record: its working tree's root and the domain's path inside it.
#[derive(Debug, Clone)]
pub struct Repo {
    pub root: PathBuf,
    pub prefix: PathBuf,
}

impl Repo {
    /// The repository the mode commits in; `None` in `unpublished`.
    pub fn for_domain(domain: &Path, mode: Publication) -> Option<Repo> {
        match mode {
            Publication::Published => {
                Some(Repo { root: domain.parent()?.to_path_buf(), prefix: PathBuf::from(domain.file_name()?) })
            }
            Publication::OwnRepository => Some(Repo { root: domain.to_path_buf(), prefix: PathBuf::new() }),
            Publication::Unpublished => None,
        }
    }

    fn events(&self) -> PathBuf {
        self.prefix.join("data/events")
    }
}

/// Git, in `repo`'s working tree, reading every object as stored and running no hook (S-11): `/dev/null` is no
/// directory, so no hook can be found under it.
pub(crate) fn command(repo: &Repo, args: &[&OsStr]) -> Command {
    let mut c = Command::new("git");
    c.current_dir(&repo.root);
    for var in ["GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE", "GIT_COMMON_DIR", "GIT_OBJECT_DIRECTORY"] {
        c.env_remove(var);
    }
    for var in ["GIT_ALTERNATE_OBJECT_DIRECTORIES", "GIT_NAMESPACE", "GIT_REPLACE_REF_BASE", "GIT_SHALLOW_FILE"] {
        c.env_remove(var);
    }
    c.env("GIT_NO_REPLACE_OBJECTS", "1").env("GIT_GRAFT_FILE", "/dev/null");
    c.args(["--no-replace-objects", "-c", "core.commitGraph=false", "-c", "core.hooksPath=/dev/null"]);
    c.args(["-c", "core.fsmonitor=false"]);
    // With `fault-injection`, a command whose words hold `FABREWK_FAULT_GIT`'s text fails as Git fails, for tests.
    if fault::git(args) {
        c.arg("--fabrewk-fault");
    }
    c.args(args);
    c
}

/// Where a write command keeps what a failed Git printed (D-083, D-086): its domain, set by the write path; reading
/// commands set none and keep nothing (REQ-REG-12).
static KEEP: std::sync::Mutex<Option<PathBuf>> = std::sync::Mutex::new(None);

/// Keeps what a failed Git prints in `domain`'s evidence from now on, in this process.
pub fn keep_in(domain: &Path) {
    *KEEP.lock().unwrap_or_else(|e| e.into_inner()) = Some(domain.to_path_buf());
}

/// The first evidence `detail`, a failure's words, names under `domain`: its standard error's digest and Git's exit
/// status, `None` for a signal (D-089); none when it names none, or what it names cannot be read.
pub fn kept_in(domain: &Path, detail: &str) -> Option<(String, Option<i32>)> {
    let after = detail.split_once(KEPT_IN)?.1;
    let at = after.find("local/evidence/git-")? + "local/evidence/".len();
    let name: String = after[at..].chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '-').collect();
    let dir = domain.join("local/evidence").join(name);
    let stderr = std::fs::read(dir.join("stderr")).ok()?;
    let status = std::fs::read_to_string(dir.join("status")).ok()?;
    Some((super::sha256_hex(&stderr), status.trim().parse().ok()))
}

/// The words that name a failed Git's evidence.
const KEPT_IN: &str = "Git's output is kept in ";

/// `TOOL_FAILED` for a Git command that failed: its first words and its exit status, and in a write command where its
/// standard error and status are kept, `local/evidence/git-<time>-<process>-<n>/` (`0700`, files `0600`).
pub(crate) fn failed(repo: &Repo, args: &[&OsStr], out: &Output) -> Error {
    static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let what: Vec<_> = args.iter().take(2).map(|a| a.to_string_lossy()).collect();
    let status = out.status.code();
    let mut why = match status {
        Some(n) => format!("git {} failed with exit status {n}", what.join(" ")),
        None => format!("git {} was ended by a signal", what.join(" ")),
    };
    let domain = KEEP.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if let Some(domain) = domain {
        let n = N.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
        let name = format!("git-{secs}-{}-{n}", std::process::id());
        let status_text = status.map_or_else(|| "signal\n".to_owned(), |n| format!("{n}\n"));
        let kept = crate::confine::evidence(&domain, &name).and_then(|dir| {
            for (file, bytes) in [("stderr", &out.stderr[..]), ("status", status_text.as_bytes())] {
                let path = dir.join(file);
                let mut options = std::fs::File::options();
                std::os::unix::fs::OpenOptionsExt::mode(options.write(true).create_new(true), 0o600);
                options.open(&path).and_then(|mut f| f.write_all(bytes)).map_err(io_at(&path))?;
            }
            Ok(dir)
        });
        if let Ok(dir) = kept {
            why += &format!("; {KEPT_IN}{}", dir.display());
        }
    }
    refuse(Code::ToolFailed, Some(&repo.root), why)
}

pub(crate) fn run(mut c: Command, input: &[u8]) -> Result<Output, Error> {
    let mut child = c.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().map_err(|e| {
        let code = if e.kind() == std::io::ErrorKind::NotFound { Code::ToolMissing } else { Code::ToolFailed };
        refuse(code, None, "git")
    })?;
    let stdin = child.stdin.take();
    std::thread::scope(|s| {
        s.spawn(move || stdin.map(|mut w| w.write_all(input)));
        child.wait_with_output()
    })
    .map_err(|_| refuse(Code::ToolFailed, None, "git"))
}

/// Runs git and gives its output, refusing `TOOL_FAILED` on a failure (see [`failed`]); Git's own text never reaches
/// the words.
pub(crate) fn git(repo: &Repo, args: &[&OsStr], input: &[u8], index: Option<&Path>) -> Result<Vec<u8>, Error> {
    let mut c = command(repo, args);
    if let Some(index) = index {
        c.env("GIT_INDEX_FILE", index);
    }
    let out = run(c, input)?;
    if !out.status.success() {
        return Err(failed(repo, args, &out));
    }
    Ok(out.stdout)
}

pub(crate) fn line(bytes: Vec<u8>) -> String {
    String::from_utf8_lossy(&bytes).trim_end_matches('\n').to_owned()
}

pub(crate) fn os<'a>(items: &[&'a str]) -> Vec<&'a OsStr> {
    items.iter().map(|s| OsStr::new(*s)).collect()
}

/// The bytes of `path`, relative to the domain, at the record branch's head; `None` if it is not there or nothing is
/// committed yet (REQ-SET-4's `POLICY_UNCOMMITTED`).
pub(crate) fn committed(repo: &Repo, path: &Path) -> Result<Option<Vec<u8>>, Error> {
    let branch = record_branch(repo)?;
    let Some(head) = head_of(repo, &branch)? else {
        return Ok(None);
    };
    let rel = repo.prefix.join(path).to_string_lossy().into_owned();
    // Whether the tip holds it is read apart, so that a Git that fails is never taken for its absence (P9G-3).
    if git(repo, &os(&["ls-tree", "-z", &head, "--", &format!(":(literal){rel}")]), b"", None)?.is_empty() {
        return Ok(None);
    }
    git(repo, &os(&["cat-file", "blob", &format!("{head}:{rel}")]), b"", None).map(Some)
}

/// The bytes of `path`, relative to the domain, as the person's index holds them; `None` if the index lacks it.
pub(crate) fn staged(repo: &Repo, path: &Path) -> Result<Option<Vec<u8>>, Error> {
    let rel = repo.prefix.join(path).to_string_lossy().into_owned();
    if git(repo, &os(&["ls-files", "-z", "--", &format!(":(literal){rel}")]), b"", None)?.is_empty() {
        return Ok(None);
    }
    git(repo, &os(&["cat-file", "blob", &format!(":{rel}")]), b"", None).map(Some)
}

/// Whether the repository ignores `path`, relative to its root, by `git check-ignore` (REQ-DOM-7): a tracked path is
/// not ignored.
pub(crate) fn ignores(repo: &Repo, path: &Path) -> Result<bool, Error> {
    let args = [OsStr::new("check-ignore"), OsStr::new("-q"), OsStr::new("--"), path.as_os_str()];
    let out = run(command(repo, &args), b"")?;
    match out.status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        _ => Err(failed(repo, &args, &out)),
    }
}

/// Git's `user.signingkey` in the repository at `root`, a path or a `key::` literal (D-055); `None` when it is unset or
/// Git cannot say.
pub(crate) fn signing_key(root: &Path) -> Option<String> {
    signing_key_read(root).ok().flatten()
}

/// [`signing_key`] for a write command: a Git that cannot say is its failure, with its evidence, never an unset key;
/// Git's exit status 1 alone says that the key is unset.
pub(crate) fn signing_key_read(root: &Path) -> Result<Option<String>, Error> {
    let repo = Repo { root: root.to_path_buf(), prefix: PathBuf::new() };
    let args = os(&["config", "--show-origin", "-z", "--type=path", "--get", "user.signingkey"]);
    let out = run(command(&repo, &args), b"")?;
    match out.status.code() {
        Some(0) => Ok(read_key(root, out.stdout)),
        Some(1) => Ok(None),
        _ => Err(failed(&repo, &args, &out)),
    }
}

fn read_key(root: &Path, stdout: Vec<u8>) -> Option<String> {
    let text = String::from_utf8(stdout).ok()?;
    let (origin, value) = text.strip_suffix('\0')?.split_once('\0')?;
    // A value Git takes from a file in a Fabrewk domain (an `include.path` there) is that file's content, read
    // for Fabrewk: never used (P7D2-30).
    let file = origin.strip_prefix("file:").map(|f| root.join(f));
    let foreign = file.is_some_and(|f| crate::code::in_a_domain(&f));
    (!foreign && !value.is_empty()).then(|| value.to_owned())
}

/// The directory the repository's own `core.hooksPath` names (S-4), Fabrewk's own value aside, as paths relative to
/// the working tree's root with a final `/` (`/` alone for the root itself): as written, and where its links lead,
/// each when it lies inside the working tree, with every path its reading passes through inside it, a link or a
/// directory a `..` leaves (whose change to a link moves the directory, P9B-12), but those above the directory, which
/// the guard covers already; a component that is not text protects the directory above it. None when it is unset or lies
/// outside. Read at each guard; a configuration Git cannot read is `TOOL_FAILED`.
pub(crate) fn hooks_dir(repo: &Repo) -> Result<Vec<String>, Error> {
    let args = os(&["config", "--show-origin", "-z", "--type=path", "--get-all", "core.hooksPath"]);
    let out = run(command(repo, &args), b"")?;
    // Fabrewk's own value is always on the command line, so the key is never unset and any failure is Git's.
    if !out.status.success() {
        return Err(failed(repo, &args, &out));
    }
    let fields: Vec<&[u8]> = out.stdout.split(|&b| b == 0).collect();
    let value = fields.chunks(2).filter(|p| p.len() == 2 && p[0] != b"command line:").map(|p| p[1]).next_back();
    let Some(value) = value.filter(|v| !v.is_empty()) else {
        return Ok(Vec::new());
    };
    let value = Path::new(<OsStr as std::os::unix::ffi::OsStrExt>::from_bytes(value));
    let mut dirs = Vec::new();
    let mut add = |path: &Path, root: &Path, dir: bool| {
        let Ok(rel) = path.strip_prefix(root) else { return };
        let text: Vec<&str> = rel.components().map_while(|c| c.as_os_str().to_str()).collect();
        let whole = text.len() == rel.components().count();
        let rel = match text.join("/") {
            r if dir || !whole => format!("{r}/"),
            // A path that leaves the root and comes back to it passes through the root, which is no path of its own.
            r if r.is_empty() => return,
            r => r,
        };
        if !dirs.contains(&rel) {
            dirs.push(rel);
        }
    };
    // A root given relative to the working directory is read as the same root (P9B-11).
    let unreadable = |_| refuse(Code::ToolFailed, Some(&repo.root), "the working tree's root cannot be resolved");
    let root = crate::code::normal(&std::path::absolute(&repo.root).map_err(unreadable)?);
    add(&crate::code::normal(&root.join(value)), &root, true);
    // Git runs a hook from the working tree's root, so the root's own links spend nothing of the path's.
    let root = repo.root.canonicalize().map_err(unreadable)?;
    let (path, passed) = physical(&root, value);
    if let Some(path) = &path {
        add(path, &root, true);
    }
    for p in passed.iter().filter(|p| !path.as_ref().is_some_and(|d| d.starts_with(p))) {
        add(p, &root, false);
    }
    Ok(dirs)
}

/// `rel` read from `from`, absolute, as the kernel reads a path (P9B-2): each link met is replaced by where it leads
/// before what follows it, a `..` after a link included, and the part that does not exist yet is kept as written.
/// Gives it, `None` past 40 links, and every path it passed through, links and directories alike, up to the 41st
/// link: the kernel meets the same ones first, so a path past the limit can be made to resolve only by changing one
/// of them (P9B-9).
fn physical(from: &Path, rel: &Path) -> (Option<PathBuf>, Vec<PathBuf>) {
    use std::path::Component;
    let parts = |p: &Path| -> Vec<std::ffi::OsString> {
        p.components()
            .filter_map(|c| match c {
                Component::Normal(n) => Some(n.to_owned()),
                Component::ParentDir => Some("..".into()),
                _ => None,
            })
            .rev()
            .collect()
    };
    let mut out = if rel.is_absolute() { PathBuf::from("/") } else { from.to_path_buf() };
    let (mut todo, mut passed, mut links) = (parts(rel), Vec::new(), 0);
    while let Some(c) = todo.pop() {
        if c == ".." {
            out.pop();
            continue;
        }
        out.push(&c);
        passed.push(out.clone());
        if let Ok(target) = std::fs::read_link(&out) {
            links += 1;
            if links > 40 {
                return (None, passed);
            }
            out.pop();
            if target.is_absolute() {
                out = PathBuf::from("/");
            }
            todo.extend(parts(&target));
        }
    }
    (Some(out), passed)
}

/// The repository's exclude file, `info/exclude` in its Git directory, as an absolute path.
pub(crate) fn exclude_file(repo: &Repo) -> Result<PathBuf, Error> {
    let out = git(repo, &os(&["rev-parse", "--path-format=absolute", "--git-path", "info/exclude"]), b"", None)?;
    Ok(PathBuf::from(line(out)))
}

/// The commit a ref, or a revision, names, `None` for an unborn branch, which has no ref at all, or a revision that
/// names none; a ref that names no commit, or a Git that cannot say, is `TOOL_FAILED` (P7D2-12, P9G-11).
pub(crate) fn head_of(repo: &Repo, name: &str) -> Result<Option<String>, Error> {
    let target = format!("{name}^{{commit}}");
    let args = os(&["rev-parse", "--verify", "-q", &target]);
    let out = run(command(repo, &args), b"")?;
    if out.status.success() {
        return Ok(Some(line(out.stdout)));
    }
    // Only exit status 1 says that the name resolves to no commit; any other is a Git that failed, never an absence,
    // and keeps what it printed (P9G-11, P9G-12).
    if out.status.code() != Some(1) {
        return Err(failed(repo, &args, &out));
    }
    // A ref that exists and names no commit keeps what Git printed, as every failed Git in a write command does (P9G-9).
    let why = |e: Error| refuse(Code::ToolFailed, Some(&repo.root), format!("{name} names no commit: {}", e.detail()));
    let refname = match name {
        "HEAD" => line(git(repo, &os(&["symbolic-ref", "-q", "HEAD"]), b"", None).map_err(why)?),
        _ => name.to_owned(),
    };
    // The reading's own output is kept too when the listing that follows it fails (P9G-15).
    let listed = git(repo, &os(&["for-each-ref", "--format=%(refname)", &refname]), b"", None).map_err(|e| {
        let first = failed(repo, &args, &out);
        refuse(
            Code::ToolFailed,
            Some(&repo.root),
            format!("{name} could not be read: {}; {}", first.detail(), e.detail()),
        )
    })?;
    if listed.is_empty() { Ok(None) } else { Err(why(failed(repo, &args, &out))) }
}

/// Refuses `NOT_A_REPOSITORY` unless `repo.root` is the repository's main working tree, at its top: the record, its
/// lock and its index live there, and a linked working tree or a subdirectory would hold another copy (P7B-3).
pub(crate) fn check_main_worktree(repo: &Repo) -> Result<(), Error> {
    let not = || refuse(Code::NotARepository, Some(&repo.root), "not the main working tree's root");
    let args = os(&["worktree", "list", "--porcelain", "-z"]);
    let out = run(command(repo, &args), b"")?;
    // What Git printed is kept, and named, whatever the code says (D-083).
    if !out.status.success() {
        let why = format!("not the main working tree's root: {}", failed(repo, &args, &out).detail());
        return Err(refuse(Code::NotARepository, Some(&repo.root), why));
    }
    let first = out.stdout.split(|&b| b == 0).next().unwrap_or_default();
    let main = std::str::from_utf8(first).ok().and_then(|l| l.strip_prefix("worktree ")).map(PathBuf::from);
    let same = main.and_then(|m| m.canonicalize().ok()) == repo.root.canonicalize().ok();
    if same { Ok(()) } else { Err(not()) }
}

/// The record branch (REQ-REG-11): the branch checked out in the main working tree; `HEAD_DETACHED` otherwise.
pub fn record_branch(repo: &Repo) -> Result<String, Error> {
    let common = line(git(repo, &os(&["rev-parse", "--git-common-dir"]), b"", None)?);
    let args = os(&["--git-dir", &common, "symbolic-ref", "-q", "HEAD"]);
    let out = run(command(repo, &args), b"")?;
    // Exit status 1 alone says that HEAD is no symbolic ref; any other is a Git that failed (P9G-14).
    match out.status.code() {
        Some(0) => {}
        Some(1) => return Err(refuse(Code::HeadDetached, Some(&repo.root), "HEAD names no branch")),
        _ => return Err(failed(repo, &args, &out)),
    }
    Ok(line(out.stdout))
}

/// Whether raw diff output (`-z`) holds an event file deleted, changed or retyped.
fn loses(raw: &[u8]) -> bool {
    raw.split(|&b| b == 0).any(|t| t.first() == Some(&b':') && matches!(t.last(), Some(b'D' | b'M' | b'T')))
}

/// The history walk (REQ-REG-8, PD-01): every parent of every commit of `branch`, and the working tree after its tip,
/// must keep, unchanged, every file under `data/events/` its parent holds. A shallow repository cannot be walked
/// whole (`CHECK_UNKNOWN`); any loss is `RECORD_ROLLBACK`, even when a later commit restores it.
pub fn walk_history(repo: &Repo, branch: &str) -> Result<(), Error> {
    if line(git(repo, &os(&["rev-parse", "--is-shallow-repository"]), b"", None)?) != "false" {
        return Err(refuse(Code::CheckUnknown, Some(&repo.root), "a shallow repository"));
    }
    let Some(tip) = head_of(repo, branch)? else {
        return Ok(());
    };
    let events = repo.events();
    let revs = git(repo, &os(&["rev-list", &tip]), b"", None)?;
    let mut args = os(&["diff-tree", "--stdin", "-m", "-r", "--no-renames", "--raw", "-z", "--"]);
    args.push(events.as_os_str());
    if loses(&git(repo, &args, &revs, None)?) {
        return Err(refuse(Code::RecordRollback, Some(&repo.root), "a commit drops or changes an event file"));
    }
    // The working tree after the tip, compared file by file without the index, which a stopped record commit may
    // have left behind the branch.
    let mut args = os(&["ls-tree", "-r", "-z", &tip, "--"]);
    args.push(events.as_os_str());
    let listing = git(repo, &args, b"", None)?;
    let (mut paths, mut blobs) = (Vec::new(), Vec::new());
    for entry in listing.split(|&b| b == 0).filter(|e| !e.is_empty()) {
        let text = String::from_utf8_lossy(entry);
        let (meta, path) = text.split_once('\t').unwrap_or_default();
        blobs.push(meta.split(' ').nth(2).unwrap_or_default().to_owned());
        paths.push(path.to_owned());
    }
    let lost = || refuse(Code::RecordRollback, Some(&repo.root), "the working tree drops or changes an event file");
    if paths.iter().any(|p| !std::fs::symlink_metadata(repo.root.join(p)).is_ok_and(|m| m.is_file())) {
        return Err(lost());
    }
    if !paths.is_empty() {
        let mut args = os(&["hash-object", "--no-filters", "--"]);
        args.extend(paths.iter().map(OsStr::new));
        if line(git(repo, &args, b"", None)?).split('\n').ne(blobs.iter().map(String::as_str)) {
            return Err(lost());
        }
    }
    Ok(())
}

/// Event files and signatures in the working tree that the record branch's tip does not hold (REQ-REG-10).
pub fn uncommitted(repo: &Repo) -> Result<Vec<PathBuf>, Error> {
    let events = repo.events();
    let mut tracked = BTreeSet::new();
    if head_of(repo, "HEAD")?.is_some() {
        let mut args = os(&["ls-tree", "-r", "-z", "--name-only", "HEAD", "--"]);
        args.push(events.as_os_str());
        let out = git(repo, &args, b"", None)?;
        let paths = out.split(|&b| b == 0).filter_map(|p| std::str::from_utf8(p).ok()).filter(|p| !p.is_empty());
        tracked.extend(paths.map(|p| repo.root.join(p)));
    }
    let root = repo.root.join(&events);
    let mut out = Vec::new();
    for dir in std::fs::read_dir(&root)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.symlink_metadata().is_ok_and(|m| m.is_dir()))
        // Only the streams replay reads: a directory whose name is no stream's holds no event (P7D3A-9).
        .filter(|p| p.file_name().and_then(|n| n.to_str()).and_then(super::stream_kind).is_some())
    {
        let listing = list_stream(&dir)?;
        let files = listing.events.into_values().flatten().chain(listing.signatures);
        out.extend(files.filter(|f| !tracked.contains(f)));
    }
    out.sort();
    Ok(out)
}

/// Refuses `TOOL_FAILED`, naming the file, if a stopped Git left the index's lock, `HEAD`'s (P7B-7) or the record
/// branch's. Fabrewk never removes a lock in the repository, since a Git the person runs may hold it and Fabrewk's
/// own lock does not cover that Git (PD-11, P7B-6). Once the person removes it, the next write command commits what
/// is left.
pub(crate) fn check_git_locks(repo: &Repo, branch: &str) -> Result<(), Error> {
    let common = line(git(repo, &os(&["rev-parse", "--path-format=absolute", "--git-common-dir"]), b"", None)?);
    let common = PathBuf::from(common);
    for lock in [common.join("index.lock"), common.join("HEAD.lock"), common.join(format!("{branch}.lock"))] {
        // The entry itself, a dangling link included, which Git takes as a lock held (P7D3A-7).
        if std::fs::symlink_metadata(&lock).is_ok() {
            return Err(refuse(Code::ToolFailed, Some(&lock), "a lock a stopped Git left; remove it if no Git runs"));
        }
    }
    Ok(())
}

/// Refuses `TOOL_FAILED` unless Git can name the author and the committer of a record commit, so that a commit that
/// could not be made is refused before anything is written (P7D3A-14).
pub(crate) fn check_ident(repo: &Repo) -> Result<(), Error> {
    for var in ["GIT_AUTHOR_IDENT", "GIT_COMMITTER_IDENT"] {
        git(repo, &os(&["var", var]), b"", None).map_err(|e| {
            let why = format!("Git names no {var} (user.name, user.email): {}", e.detail());
            refuse(Code::ToolFailed, Some(&repo.root), why)
        })?;
    }
    Ok(())
}

/// Puts back in the person's index, from the tip, every event file a stopped record commit left out of it.
pub(crate) fn sync_index(repo: &Repo) -> Result<(), Error> {
    if head_of(repo, "HEAD")?.is_none() {
        return Ok(());
    }
    let events = repo.events();
    let mut args = os(&["diff-index", "--cached", "--no-renames", "--name-only", "-z", "HEAD", "--"]);
    args.push(events.as_os_str());
    let differ = git(repo, &args, b"", None)?;
    // Only the record's own files: an event or a signature in a stream replay reads (P7D3A-12). Anything else under
    // `data/events/`, a hidden directory's file among them, is the person's and stays staged as it is.
    let record_file = |p: &&str| {
        let rel = Path::new(p).strip_prefix(&events).ok();
        let parts: Vec<&str> = rel.into_iter().flat_map(Path::iter).filter_map(OsStr::to_str).collect();
        matches!(parts[..], [stream, name] if super::stream_kind(stream).is_some() && !name.starts_with('.'))
    };
    let paths: Vec<&OsStr> = differ
        .split(|&b| b == 0)
        .filter(|p| !p.is_empty())
        .filter_map(|p| std::str::from_utf8(p).ok())
        .filter(record_file)
        .map(OsStr::new)
        .collect();
    if paths.is_empty() {
        return Ok(());
    }
    // `ls-tree` prints what `--index-info` reads; a path the tip lacks is not printed and stays as it is.
    let mut args = os(&["ls-tree", "-z", "HEAD", "--"]);
    args.extend(paths);
    let entries = git(repo, &args, b"", None)?;
    fault::point();
    git(repo, &os(&["update-index", "-z", "--index-info"]), &entries, None)?;
    Ok(())
}

/// The message of a record commit: one trailer `Fabrewk-Event: <digest>` per event.
pub(crate) fn record_message(digests: &[String]) -> String {
    "fabrewk: record\n\n".to_owned() + &digests.iter().map(|d| format!("Fabrewk-Event: {d}\n")).collect::<String>()
}

/// Makes the record commit of `files` (absolute paths in the working tree) on the record branch, moving it from
/// `start` by compare-and-swap; if it moved, rebuilds once on the new head when no file of the commit changed there,
/// and otherwise refuses `RECORD_COMMIT_RACE`, the files left as written. One `Fabrewk-Event` trailer per digest. What
/// failed in Git that it went on from is pushed on `notes`, each naming its evidence (D-088, D-089).
#[allow(clippy::too_many_arguments)]
pub(crate) fn commit_record(
    repo: &Repo,
    domain: &Path,
    branch: &str,
    start: Option<&str>,
    files: &[PathBuf],
    message: &str,
    notes: &mut Vec<String>,
) -> Result<String, Error> {
    check_git_locks(repo, branch)?;
    let mut rel = Vec::new();
    for f in files {
        let r = f.strip_prefix(&repo.root).ok().and_then(Path::to_str).filter(|r| !r.contains(['\n', '\t', '\0']));
        rel.push(r.ok_or_else(|| refuse(Code::ToolFailed, Some(f), "a path outside the repository"))?);
    }
    let mut args = os(&["hash-object", "-w", "--no-filters", "--"]);
    args.extend(rel.iter().map(OsStr::new));
    let blobs = line(git(repo, &args, b"", None)?);
    if blobs.split('\n').count() != rel.len() {
        return Err(refuse(Code::ToolFailed, Some(&repo.root), "git hash-object gave another count"));
    }
    let entries: String = blobs.split('\n').zip(&rel).map(|(blob, path)| format!("100644 {blob}\t{path}\0")).collect();
    let tmp = domain.join("local/tmp");
    std::fs::create_dir_all(&tmp).map_err(io_at(&tmp))?;
    let index = tmp.join("record-index");
    let mut earlier = Vec::new();
    let result = attempt(repo, branch, start, &rel, &entries, message, &index, &mut earlier);
    let _ = std::fs::remove_file(&index);
    match result {
        Ok(commit) => {
            notes.extend(earlier);
            Ok(commit)
        }
        Err(e) => Err(after(e, &earlier)),
    }
}

/// `e` with `earlier`, what failed in Git before it in the same commit, named after its own words (D-089).
fn after(e: Error, earlier: &[String]) -> Error {
    match e {
        Error::Refused { code, path, detail } if !earlier.is_empty() => {
            Error::Refused { code, path, detail: format!("{detail}; before it, {}", earlier.join("; ")) }
        }
        e => e,
    }
}

/// The record commit's rounds (see [`commit_record`]); what failed in Git that it went on from is pushed on `earlier`.
#[allow(clippy::too_many_arguments)]
fn attempt(
    repo: &Repo,
    branch: &str,
    start: Option<&str>,
    rel: &[&str],
    entries: &str,
    message: &str,
    index: &Path,
    earlier: &mut Vec<String>,
) -> Result<String, Error> {
    let mut base = start.map(str::to_owned);
    for round in 0..2 {
        let _ = std::fs::remove_file(index);
        let read = match &base {
            Some(b) => os(&["read-tree", b]),
            None => os(&["read-tree", "--empty"]),
        };
        git(repo, &read, b"", Some(index))?;
        git(repo, &os(&["update-index", "-z", "--index-info"]), entries.as_bytes(), Some(index))?;
        let tree = line(git(repo, &os(&["write-tree"]), b"", Some(index))?);
        let mut args = vec!["commit-tree", &tree, "-F", "-"];
        if let Some(b) = &base {
            args.extend(["-p", b]);
        }
        fault::point();
        let commit = line(git(repo, &os(&args), message.as_bytes(), None)?);
        fault::point();
        let old = base.clone().unwrap_or_default();
        let update = os(&["update-ref", "-m", "fabrewk: record", branch, &commit, &old]);
        let moved = run(command(repo, &update), b"")?;
        if moved.status.success() {
            fault::point();
            fault::pause("record-moved");
            // The commit is made and the branch moved: the person's index is brought to it, and if another Git holds
            // the index now, the next write command does it (`sync_index`), so the commit is never reported as not
            // made (P7D3A-21); the command says what failed (D-088).
            if let Err(e) = git(repo, &os(&["update-index", "-z", "--index-info"]), entries.as_bytes(), None) {
                earlier.push(e.detail());
            }
            fault::point();
            return Ok(commit);
        }
        // What Git printed is kept whatever follows, and every way out of this round names it (P9G-15, P9G-18).
        let cas = failed(repo, &update, &moved);
        let now = head_of(repo, branch).map_err(|e| after(e, &[cas.detail()]))?;
        if now == base {
            return Err(cas);
        }
        let next = (|| -> Result<Option<String>, Error> {
            let race = || {
                refuse(Code::RecordCommitRace, Some(&repo.root), "the branch moved and changed a file of the commit")
            };
            let Some(n) = now.as_deref() else {
                return Err(race());
            };
            // The new head must descend from the one the command started from, which holds every event committed so
            // far (P7D3A-19), and is walked as the session's first one was: a head whose history drops or changes an
            // event is a rollback, never a base (P7D3A-16).
            if let Some(b) = &base {
                let args = os(&["merge-base", "--is-ancestor", b, n]);
                let kept = run(command(repo, &args), b"")?;
                match kept.status.code() {
                    Some(0) => {}
                    Some(1) => {
                        let why = "the branch moved to a commit that does not descend from the record's last one";
                        return Err(refuse(Code::RecordRollback, Some(&repo.root), why));
                    }
                    _ => return Err(failed(repo, &args, &kept)),
                }
            }
            let mut changed = match &base {
                Some(b) => os(&["diff-tree", "-r", "--name-only", "-z", b, n, "--"]),
                None => os(&["ls-tree", "-r", "--name-only", "-z", n, "--"]),
            };
            changed.extend(rel.iter().map(OsStr::new));
            if round == 1 || !git(repo, &changed, b"", None)?.is_empty() {
                return Err(race());
            }
            walk_history(repo, branch)?;
            Ok(now)
        })();
        match next {
            Ok(now) => {
                earlier.push(cas.detail());
                base = now;
            }
            Err(e) => return Err(after(e, &[cas.detail()])),
        }
    }
    Err(refuse(Code::RecordCommitRace, Some(&repo.root), "the branch moved twice"))
}
