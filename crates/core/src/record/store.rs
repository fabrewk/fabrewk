//! Writing the record (REQ-REG-5): one writer per repository holds the lock `local/lock` for its whole run; each
//! append re-reads its stream's head under the lock and fails if it moved; every file is written to a temporary name
//! starting with `.`, synced, renamed and its directory synced. A person's signature is in place before its event.

use super::event::{self, Class, Draft};
use super::git::{self, Repo};
use super::replay::{self, Replay, Verify, list_stream};
use super::{Code, Error, canon, fault, io_at, mark, refuse};
use crate::code::unlinked;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

/// A stream's last event: its position and full digest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Head {
    pub seq: u64,
    pub digest: String,
}

/// An event made and signed by [`Session::prepare`], not yet written.
pub struct Prepared {
    stream: String,
    expected: Option<Head>,
    bytes: Vec<u8>,
    path: PathBuf,
    head: Head,
    signature: Option<(Vec<u8>, Vec<u8>)>,
}

impl Prepared {
    /// The head the stream will have once it is written.
    pub fn head(&self) -> &Head {
        &self.head
    }
}

/// A write command's hold on the record, from the lock to the record commit.
pub struct Session {
    domain: PathBuf,
    _lock: File,
    appended: Vec<(String, PathBuf)>,
    git: Option<(Repo, String, Option<String>)>,
    record: Option<Replay>,
}

impl Session {
    /// Opens a write command (REQ-REG-5, REQ-REG-8, REQ-REG-11). First, reading only: with `repo` (the modes
    /// that commit), that the repository's root is its main working tree's (`NOT_A_REPOSITORY` otherwise: a linked
    /// working tree holds another domain), the record branch and its head, refusing `HEAD_DETACHED`; the replay, refused by its first
    /// code (an absent record is no refusal here: `apply` writes the first event); with `repo`, the whole history;
    /// the mark. Then the lock, at once or `LOCK_HELD`, never waiting. Then it removes what an interrupted writer
    /// left: temporary files, signatures whose event never reached its place, the record commit's temporary index.
    pub fn begin(domain: &Path, repo: Option<Repo>, verifier: &dyn Verify) -> Result<Session, Error> {
        let git = match repo {
            Some(repo) => {
                git::check_main_worktree(&repo)?;
                let branch = git::record_branch(&repo)?;
                let head = git::head_of(&repo, &branch)?;
                Some((repo, branch, head))
            }
            None => None,
        };
        let record = match replay::replay(domain, verifier) {
            Ok(r) => Some(r),
            Err(errors) if errors.iter().all(|e| e.code() == Some(Code::RecordAbsent)) => None,
            Err(mut errors) => return Err(errors.remove(0)),
        };
        if let Some((repo, branch, _)) = &git {
            git::walk_history(repo, branch)?;
        }
        mark::check_mark(domain, record.as_ref().unwrap_or(&Replay::default()))?;
        let local = domain.join("local");
        let path = local.join("lock");
        // Neither the unversioned area nor the lock is followed to a place outside the domain (REQ-DOM-1, P8C-6).
        if !unlinked(domain, "local/lock") {
            return Err(refuse(
                Code::RecordBroken,
                Some(&path),
                "the lock is reached through a link; it is not followed",
            ));
        }
        fs::create_dir_all(&local).map_err(io_at(&local))?;
        fault::pause("before-lock");
        let lock = open_lock(domain, &path)?;
        match lock.try_lock() {
            Ok(()) => {}
            Err(fs::TryLockError::WouldBlock) => {
                return Err(refuse(Code::LockHeld, Some(&path), "another writer"));
            }
            Err(fs::TryLockError::Error(e)) => return Err(io_at(&path)(e)),
        }
        // The record commit's temporary index and the lock Git keeps beside it while writing it (P7B-1).
        let mut leftovers = vec![local.join("tmp/record-index"), local.join("tmp/record-index.lock")];
        leftovers.retain(|p| p.is_file());
        let events = domain.join("data/events");
        let stream = |p: &PathBuf| p.file_name().and_then(|n| n.to_str()).and_then(event::stream_kind).is_some();
        for dir in
            fs::read_dir(&events).into_iter().flatten().flatten().map(|e| e.path()).filter(|p| p.is_dir() && stream(p))
        {
            let listing = list_stream(&dir)?;
            leftovers.extend(listing.temporary.into_iter().chain(listing.orphan_signatures));
        }
        for leftover in &leftovers {
            fault::point();
            fs::remove_file(leftover).map_err(io_at(leftover))?;
        }
        Ok(Session { domain: domain.to_path_buf(), _lock: lock, appended: Vec::new(), git, record })
    }

    /// The record as replayed when the command began, or at its last [`Session::refresh`]; `None` if there was none.
    pub fn record(&self) -> Option<&Replay> {
        self.record.as_ref()
    }

    /// Replays the record again now that the lock is held, so that a command plans from what no other writer can
    /// change while it runs (part 7d3a); refused by its first code.
    pub fn refresh(&mut self, verifier: &dyn Verify) -> Result<(), Error> {
        self.record = Some(replay::replay(&self.domain, verifier).map_err(|mut errors| errors.remove(0))?);
        Ok(())
    }

    /// The head of `stream` as it is on disk now, `None` for a stream with no event.
    pub fn head(&self, stream: &str) -> Result<Option<Head>, Error> {
        let dir = self.domain.join("data/events").join(stream);
        let listing = list_stream(&dir)?;
        let Some((&seq, files)) = listing.events.last_key_value() else {
            return Ok(None);
        };
        if files.len() > 1 {
            return Err(refuse(Code::RecordFork, Some(&dir), format!("two events at {seq}")));
        }
        let bytes = fs::read(&files[0]).map_err(io_at(&files[0]))?;
        let digest = canon::sha256_hex(&bytes);
        if !replay::name_matches(&files[0], &digest) {
            return Err(refuse(Code::RecordDigestMismatch, Some(&files[0]), "name and content differ"));
        }
        Ok(Some(Head { seq, digest }))
    }

    /// The seal a `policy-adopted` records (D-050): the head of every stream but `project`, as it is on disk now.
    pub fn seal(&self) -> Result<std::collections::BTreeMap<String, Head>, Error> {
        let mut heads = std::collections::BTreeMap::new();
        let events = self.domain.join("data/events");
        for entry in fs::read_dir(&events).into_iter().flatten() {
            let name = entry.map_err(io_at(&events))?.file_name().to_string_lossy().into_owned();
            if name != "project"
                && event::stream_kind(&name).is_some()
                && let Some(head) = self.head(&name)?
            {
                heads.insert(name, head);
            }
        }
        Ok(heads)
    }

    /// Appends one event to `stream` if its head is still `expected` (`RECORD_HEAD_MOVED` otherwise, nothing written).
    /// A person's event needs `signer`; the event is checked against schema 1 before anything is written.
    pub fn append(
        &mut self,
        stream: &str,
        expected: Option<&Head>,
        draft: &Draft,
        signer: Option<&dyn super::Sign>,
    ) -> Result<Head, Error> {
        let prepared = self.prepare(stream, expected, draft, signer)?;
        self.write(prepared)
    }

    /// Makes one event of `stream` after `expected` and signs it, writing nothing (part 7d3a): a command prepares
    /// every event it will write, so that a signature that cannot be made refuses before any write.
    pub fn prepare(
        &self,
        stream: &str,
        expected: Option<&Head>,
        draft: &Draft,
        signer: Option<&dyn super::Sign>,
    ) -> Result<Prepared, Error> {
        let dir = self.domain.join("data/events").join(stream);
        if event::stream_kind(stream).is_none() {
            return Err(refuse(Code::RecordBroken, Some(&dir), "stream name"));
        }
        let seq = expected.map_or(1, |h| h.seq + 1);
        if seq > 999_999 {
            return Err(refuse(Code::RecordBroken, Some(&dir), "stream full"));
        }
        let value = draft.build(stream, seq, expected.map(|h| h.digest.as_str()));
        let class = event::check(&value, stream, seq).map_err(|(code, why)| refuse(code, Some(&dir), why))?;
        if seq == 1
            && event::is_creation(&draft.kind)
            && event::subject_stream(&draft.kind, &draft.payload).as_deref() != Some(stream)
        {
            return Err(refuse(Code::RecordBroken, Some(&dir), "the stream is not the subject's"));
        }
        let bytes = canon::encode(&value).ok_or_else(|| refuse(Code::RecordNoncanonical, Some(&dir), "a number"))?;
        let digest = canon::sha256_hex(&bytes);
        let path = dir.join(format!("{seq:06}-{}.json", &digest[..16]));
        let signature = match (class, signer) {
            (Class::Person, Some(s)) => {
                let key = crate::settings::key_blob(&s.public()?).ok_or_else(|| {
                    refuse(Code::RecordSignatureInvalid, Some(&path), "the signer's key is not a key")
                })?;
                Some((s.sign(&bytes)?, key))
            }
            (Class::Person, None) => {
                return Err(refuse(Code::RecordSignatureInvalid, Some(&path), "no signer"));
            }
            (_, _) => None,
        };
        let expected = expected.cloned();
        Ok(Prepared { stream: stream.to_owned(), expected, bytes, path, head: Head { seq, digest }, signature })
    }

    /// Writes a prepared event if its stream's head on disk is still the one it was made after (`RECORD_HEAD_MOVED`
    /// otherwise, nothing written); a signature is checked in place before the event exists.
    pub fn write(&mut self, p: Prepared) -> Result<Head, Error> {
        let dir = self.domain.join("data/events").join(&p.stream);
        if self.head(&p.stream)? != p.expected {
            return Err(refuse(Code::RecordHeadMoved, Some(&dir), "the head moved"));
        }
        create_dir(&dir)?;
        if let Some((sig, key)) = &p.signature {
            // Checked in place before the event exists, so a signature that would not verify writes no event: the
            // `.sig` alone is an interrupted write's leftover, removed here or by the next writer.
            let sig_path = replay::signature_of(&p.path);
            write_atomic(&sig_path, sig)?;
            if let Err(e) = super::sign::check(key, &p.bytes, &sig_path) {
                fs::remove_file(&sig_path).map_err(io_at(&sig_path))?;
                return Err(e);
            }
            self.appended.push((p.head.digest.clone(), sig_path));
        }
        write_atomic(&p.path, &p.bytes)?;
        self.appended.push((p.head.digest.clone(), p.path));
        Ok(p.head)
    }

    /// Ends the write: replays the whole record (PD-46: if it fails, nothing is committed), checks the mark and
    /// replaces it with the new heads, then, in the modes that commit, makes the record commit of the files appended
    /// and of `views` (paths relative to the domain). Gives the commit, if one was made, and releases the lock.
    pub fn finish(mut self, verifier: &dyn Verify, views: &[PathBuf]) -> Result<Option<String>, Error> {
        self.finish_held(verifier, views)
    }

    /// As [`Session::finish`], keeping the lock until the session is dropped, so that a command holds it while it
    /// writes its caches (REQ-REG-5); [`Session::record`] is then the record as this replay read it.
    pub fn finish_held(&mut self, verifier: &dyn Verify, views: &[PathBuf]) -> Result<Option<String>, Error> {
        let replay = replay::replay(&self.domain, verifier).map_err(|mut errors| errors.remove(0))?;
        if fault::fails("replay-before-commit") {
            return Err(refuse(Code::RecordBroken, None, "replay failed (fault injection)"));
        }
        mark::check_mark(&self.domain, &replay)?;
        // The mark moves only once the record commit is made, where the mode commits (P7D3A-10).
        let Some((repo, branch, start)) = &self.git else {
            mark::write(&self.domain, &replay)?;
            self.record = Some(replay);
            return Ok(None);
        };
        fault::pause("before-commit");
        let mut files: Vec<PathBuf> = self.appended.iter().map(|a| a.1.clone()).collect();
        files.extend(views.iter().map(|v| self.domain.join(v)));
        let commit = if files.is_empty() {
            None
        } else {
            let json = |p: &PathBuf| p.extension().is_some_and(|e| e == "json");
            let digests: Vec<String> = self.appended.iter().filter(|a| json(&a.1)).map(|a| a.0.clone()).collect();
            // A record commit that fails leaves the events written and not committed, whatever the cause, and the
            // refusal says so (D-065); the next write command commits them first (REQ-REG-10).
            let message = git::record_message(&digests);
            let commit = git::commit_record(repo, &self.domain, branch, start.as_deref(), &files, &message);
            Some(commit.map_err(|e| match e {
                Error::Refused { code, path, detail } => Error::Refused {
                    code,
                    path,
                    detail: format!(
                        "{detail}; this command's events are written and not committed, and the next write command \
                         commits them"
                    ),
                },
                other => other,
            })?)
        };
        mark::write(&self.domain, &replay)?;
        self.record = Some(replay);
        Ok(commit)
    }

    /// Replays the record as it now stands, as the record commit does before it: a failure commits nothing (PD-46).
    pub fn replayable(&self, verifier: &dyn Verify) -> Result<(), Error> {
        let replay = replay::replay(&self.domain, verifier).map_err(|mut errors| errors.remove(0))?;
        if fault::fails("replay-before-commit") {
            return Err(refuse(Code::RecordBroken, None, "replay failed (fault injection)"));
        }
        mark::check_mark(&self.domain, &replay)
    }

    /// Commits `file` (absolute) alone, in a commit of its own on the record branch with `message`, as the person's
    /// Git names its author and with no co-author (REQ-FER-5); nothing when the mode does not commit or the branch
    /// already holds the file as it is. The record commit that follows starts from it.
    pub fn commit_alone(&mut self, file: &Path, message: &str) -> Result<Option<String>, Error> {
        let Some((repo, branch, start)) = &mut self.git else {
            return Ok(None);
        };
        let rel = file.strip_prefix(&self.domain).unwrap_or(file);
        let (bytes, held) = (fs::read(file).map_err(io_at(file))?, git::committed(repo, rel)?);
        if held.as_deref() == Some(bytes.as_slice()) && git::staged(repo, rel)?.as_deref() == Some(bytes.as_slice()) {
            return Ok(None);
        }
        let commit = git::commit_record(repo, &self.domain, branch, start.as_deref(), &[file.to_path_buf()], message)?;
        *start = Some(commit.clone());
        // The branch has moved; if the person's index could not follow (another Git held it), it still holds the text
        // before, which the next `git commit` would take for a reversion: say so, with the commit made.
        if git::staged(repo, rel)?.as_deref() != Some(bytes.as_slice()) {
            // The repository is named, for in `own-repository` the record's is not the product's.
            let (root, name) = (repo.root.display().to_string(), file.strip_prefix(&repo.root).unwrap_or(file));
            let name = name.display().to_string();
            let plain = |t: &str| t.chars().all(|c| c.is_ascii_alphanumeric() || "_./-+=,:@%~".contains(c));
            let fix = match plain(&format!("{root}{name}")) {
                true => format!("run git -C {root} add {name}"),
                false => format!("stage {name} in the repository at {root}"),
            };
            let why = format!("the commit {commit} is made, but the index could not be brought to it; {fix}");
            return Err(refuse(Code::ToolFailed, Some(file), why));
        }
        Ok(Some(commit))
    }

    /// Commits, in a record commit of their own, the event files a stopped or raced command left uncommitted
    /// (REQ-REG-10), after a replay; a write command calls it first. Also puts back in the person's index the event
    /// files a stopped record commit left out of it.
    pub fn commit_uncommitted(&mut self, verifier: &dyn Verify) -> Result<Option<String>, Error> {
        let Some((repo, branch, start)) = &mut self.git else {
            return Ok(None);
        };
        git::check_git_locks(repo, branch)?;
        git::sync_index(repo)?;
        let files = git::uncommitted(repo)?;
        if files.is_empty() {
            return Ok(None);
        }
        replay::replay(&self.domain, verifier).map_err(|mut errors| errors.remove(0))?;
        let mut digests = Vec::new();
        for f in files.iter().filter(|f| f.extension().is_some_and(|e| e == "json")) {
            digests.push(canon::sha256_hex(&fs::read(f).map_err(io_at(f))?));
        }
        let message = git::record_message(&digests);
        let commit = git::commit_record(repo, &self.domain, branch, start.as_deref(), &files, &message)?;
        *start = Some(commit.clone());
        Ok(Some(commit))
    }
}

fn create_dir(dir: &Path) -> Result<(), Error> {
    if dir.is_dir() {
        return Ok(());
    }
    let parent = dir.parent().unwrap_or(dir);
    create_dir(parent)?;
    fault::point();
    fs::create_dir(dir).map_err(io_at(dir))?;
    sync_dir(parent)
}

/// Writes `bytes` to `path` whole or not at all: a temporary name starting with `.` in the same directory, synced,
/// renamed, and the directory synced (REQ-REG-5).
pub(crate) fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), Error> {
    let dir = path.parent().unwrap_or(Path::new("."));
    let name = path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();
    let tmp = dir.join(format!(".{name}.tmp"));
    fault::point();
    // Whatever stands at the temporary name, a link included, is removed, not followed, and the file is created anew
    // (`O_EXCL`): the write never leaves the directory (P8C-1).
    let write = || -> std::io::Result<()> {
        match fs::remove_file(&tmp) {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => return Err(e),
            _ => {}
        }
        let mut file = File::options().write(true).create_new(true).open(&tmp)?;
        std::io::Write::write_all(&mut file, bytes)?;
        file.sync_all()
    };
    write().map_err(io_at(&tmp))?;
    fault::point();
    fs::rename(&tmp, path).map_err(io_at(path))?;
    fault::point();
    sync_dir(dir)
}

/// Opens the lock without following a link at its name, even one put there after the check (P8C-6, P8C-7): an existing
/// file is opened and must be the very file the name holds (not a link to another); a missing one is created
/// exclusively (`O_EXCL` does not follow a link) and only in a folder that lies inside the domain.
fn open_lock(domain: &Path, path: &Path) -> Result<File, Error> {
    use std::os::unix::fs::MetadataExt;
    let link = || refuse(Code::RecordBroken, Some(path), "the lock is reached through a link; it is not followed");
    for _ in 0..3 {
        let opened = match File::options().write(true).open(path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                let inside = path.parent().and_then(|p| p.canonicalize().ok()).zip(domain.canonicalize().ok());
                if !inside.is_some_and(|(local, domain)| local.starts_with(domain)) {
                    return Err(link());
                }
                match File::options().write(true).create_new(true).open(path) {
                    Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                    other => other,
                }
            }
            other => other,
        };
        let file = opened.map_err(io_at(path))?;
        let same = fs::symlink_metadata(path).ok().zip(file.metadata().ok());
        let same = same.is_some_and(|(l, f)| l.is_file() && (l.dev(), l.ino()) == (f.dev(), f.ino()));
        return if same { Ok(file) } else { Err(link()) };
    }
    Err(link())
}

fn sync_dir(dir: &Path) -> Result<(), Error> {
    File::open(dir).and_then(|d| d.sync_all()).map_err(io_at(dir))
}
