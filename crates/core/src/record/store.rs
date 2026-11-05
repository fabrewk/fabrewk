//! Writing the record (REQ-REG-5): one writer per repository holds the lock `local/lock` for its whole run; each
//! append re-reads its stream's head under the lock and fails if it moved; every file is written to a temporary name
//! starting with `.`, synced, renamed and its directory synced. A person's signature is in place before its event.

use super::event::{self, Class, Draft};
use super::git::{self, Repo};
use super::replay::{self, Replay, Verify, list_stream};
use super::{Code, Error, canon, fault, io_at, mark, refuse};
use std::fs::{self, File};
use std::path::{Path, PathBuf};

/// A stream's last event: its position and full digest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Head {
    pub seq: u64,
    pub digest: String,
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
        fs::create_dir_all(&local).map_err(io_at(&local))?;
        let path = local.join("lock");
        let lock = File::options().create(true).truncate(false).write(true).open(&path).map_err(io_at(&path))?;
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
        for dir in fs::read_dir(&events).into_iter().flatten().flatten().map(|e| e.path()).filter(|p| p.is_dir()) {
            let listing = list_stream(&dir)?;
            leftovers.extend(listing.temporary.into_iter().chain(listing.orphan_signatures));
        }
        for leftover in &leftovers {
            fault::point();
            fs::remove_file(leftover).map_err(io_at(leftover))?;
        }
        Ok(Session { domain: domain.to_path_buf(), _lock: lock, appended: Vec::new(), git, record })
    }

    /// The record as replayed when the command began; `None` if there was none yet.
    pub fn record(&self) -> Option<&Replay> {
        self.record.as_ref()
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
        let dir = self.domain.join("data/events").join(stream);
        if event::stream_kind(stream).is_none() {
            return Err(refuse(Code::RecordBroken, Some(&dir), "stream name"));
        }
        let head = self.head(stream)?;
        if head.as_ref() != expected {
            return Err(refuse(Code::RecordHeadMoved, Some(&dir), "the head moved"));
        }
        let seq = head.as_ref().map_or(1, |h| h.seq + 1);
        if seq > 999_999 {
            return Err(refuse(Code::RecordBroken, Some(&dir), "stream full"));
        }
        let value = draft.build(stream, seq, head.as_ref().map(|h| h.digest.as_str()));
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
        create_dir(&dir)?;
        if let Some((sig, key)) = signature {
            // Checked in place before the event exists, so a signature that would not verify writes no event: the
            // `.sig` alone is an interrupted write's leftover, removed here or by the next writer.
            let sig_path = replay::signature_of(&path);
            write_atomic(&sig_path, &sig)?;
            if let Err(e) = super::sign::check(&key, &bytes, &sig_path) {
                fs::remove_file(&sig_path).map_err(io_at(&sig_path))?;
                return Err(e);
            }
            self.appended.push((digest.clone(), sig_path));
        }
        write_atomic(&path, &bytes)?;
        self.appended.push((digest.clone(), path));
        Ok(Head { seq, digest })
    }

    /// Ends the write: replays the whole record (PD-46: if it fails, nothing is committed), checks the mark and
    /// replaces it with the new heads, then, in the modes that commit, makes the record commit of the files appended
    /// and of `views` (paths relative to the domain). Gives the commit, if one was made.
    pub fn finish(self, verifier: &dyn Verify, views: &[PathBuf]) -> Result<Option<String>, Error> {
        let replay = replay::replay(&self.domain, verifier).map_err(|mut errors| errors.remove(0))?;
        if fault::fails("replay-before-commit") {
            return Err(refuse(Code::RecordBroken, None, "replay failed (fault injection)"));
        }
        mark::check_mark(&self.domain, &replay)?;
        mark::write(&self.domain, &replay)?;
        let Some((repo, branch, start)) = &self.git else {
            return Ok(None);
        };
        let mut files: Vec<PathBuf> = self.appended.iter().map(|a| a.1.clone()).collect();
        files.extend(views.iter().map(|v| self.domain.join(v)));
        if files.is_empty() {
            return Ok(None);
        }
        let json = |p: &PathBuf| p.extension().is_some_and(|e| e == "json");
        let digests: Vec<String> = self.appended.iter().filter(|a| json(&a.1)).map(|a| a.0.clone()).collect();
        git::commit_record(repo, &self.domain, branch, start.as_deref(), &files, &digests).map(Some)
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
        let commit = git::commit_record(repo, &self.domain, branch, start.as_deref(), &files, &digests)?;
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
    let write = || -> std::io::Result<()> {
        let mut file = File::create(&tmp)?;
        std::io::Write::write_all(&mut file, bytes)?;
        file.sync_all()
    };
    write().map_err(io_at(&tmp))?;
    fault::point();
    fs::rename(&tmp, path).map_err(io_at(path))?;
    fault::point();
    sync_dir(dir)
}

fn sync_dir(dir: &Path) -> Result<(), Error> {
    File::open(dir).and_then(|d| d.sync_all()).map_err(io_at(dir))
}
