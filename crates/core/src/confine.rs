//! The cell's contract and its runner (REQ-SEC-1, REQ-SEC-2, REQ-REG-13; `confinement-design.md` 3.1 to 3.7 and 3.9;
//! part 9a). The core describes what one confined process may reach and names no mechanism; a backend, which the
//! binary wires for its operating system, turns the description into the system's own (principle 2). The runner keeps
//! the process's two output channels in `local/evidence/`, ends it at its limits, and returns only once every process
//! of the cell has ended (principle 4). Where no backend exists, a program it drives is absent or its probe fails,
//! confinement refuses (`CONFINEMENT_UNAVAILABLE`): no role ever runs with less (principle 5).

use crate::code::{Code, Error, io_at, refuse};
use crate::detect::{Detection, Program, Space};
use crate::record::hex;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::Child;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// How a mount shows its host path inside: read-only, writable, or not at all (an empty read-only directory over it).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    ReadOnly,
    ReadWrite,
    Hidden,
}

/// A host path shown at a path inside the cell; for `Hidden`, the host path is unused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mount {
    pub host: PathBuf,
    pub inside: PathBuf,
    pub access: Access,
}

/// A cell's network: none at all, or a private one that only connects outwards (S-1, D-075).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    None,
    Outbound,
}

/// Everything a confined process may reach; nothing else exists inside (principle 1). Beyond it a backend shows only
/// the system's directories, read-only.
#[derive(Debug, Clone)]
pub struct Cell {
    /// The host directories shown as the cell's home (`/home/agent`) and its `/tmp`: with the writable mounts, the only
    /// places it writes (S-10).
    pub home: PathBuf,
    pub tmp: PathBuf,
    pub mounts: Vec<Mount>,
    pub network: Network,
    /// Whether it may make namespaces of its own, for a tool's own sandbox.
    pub nested: bool,
    /// The whole environment; nothing of Fabrewk's own is inherited.
    pub env: Vec<(OsString, OsString)>,
    pub cwd: PathBuf,
    pub argv: Vec<OsString>,
}

/// The programs a backend drives, by executable, at the paths detection found them (REQ-SET-8). Only
/// [`available`] makes one, so that no cell starts under a backend that is absent or has not passed its probe
/// (principle 5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Programs(BTreeMap<&'static str, PathBuf>);

impl Programs {
    /// Where the program `executable` was found.
    pub fn get(&self, executable: &str) -> Option<&Path> {
        self.0.get(executable).map(PathBuf::as_path)
    }
}

/// A started cell: Fabrewk's direct child, whose end ends the cell; what the backend started beside it, ended after
/// it; and the host's identifier of the cell's first process, gone only once every process of the cell has ended.
pub struct Started {
    pub child: Child,
    pub beside: Vec<Child>,
    pub first: Option<u32>,
}

/// A confinement backend (`confinement-design.md` 3.2): the programs it drives, how it starts and ends a cell, and
/// the residual risks it cannot close.
pub struct Backend {
    pub name: &'static str,
    pub programs: &'static [Program],
    /// Starts a cell, writing what it needs in the scratch directory and any output of its own in the evidence
    /// directory; gives its direct child with both output channels piped and its command released.
    pub start: fn(&Cell, &Programs, &Path, &Path) -> Result<Started, Error>,
    /// Ends a started cell if it has not ended, and returns once every process of it has ended.
    pub stop: fn(&mut Started) -> Result<(), Error>,
    /// Catalogue keys `residual.<key>` that `config` lists (REQ-SEC-5, S-7).
    pub residual: &'static [&'static str],
}

/// The residual risks of the core itself, whatever the backend (`confinement-design.md` 3.10: T-17, T-18, T-25).
pub const RESIDUAL: [&str; 3] = ["check-proves", "reviewer-steered", "same-user"];

/// A run's directories under `local/tmp/<name>/`, on disk (AD-8): its home, `/tmp` and `/work`, the only places its
/// cell writes (S-10), and where the backend writes what the cell needs; removed whole when the run ends.
#[derive(Debug)]
pub struct Scratch {
    pub root: PathBuf,
    pub home: PathBuf,
    pub tmp: PathBuf,
    pub work: PathBuf,
}

/// Creates `dir` new, readable and writable by its owner only.
fn private_dir(dir: &Path) -> Result<(), Error> {
    fs::create_dir(dir).and_then(|()| fs::set_permissions(dir, fs::Permissions::from_mode(0o700))).map_err(io_at(dir))
}

impl Scratch {
    /// Creates the run `name`'s directories; a name already in use refuses as an operating-system error.
    pub fn create(domain: &Path, name: &str) -> Result<Scratch, Error> {
        let parent = domain.join("local/tmp");
        fs::create_dir_all(&parent).map_err(io_at(&parent))?;
        let root = parent.join(name);
        private_dir(&root)?;
        let [home, tmp, work] = ["home", "tmp", "work"].map(|d| root.join(d));
        for d in [&home, &tmp, &work] {
            private_dir(d)?;
        }
        Ok(Scratch { root, home, tmp, work })
    }

    /// Removes the directories with all a cell left in them, following no link; only once the cell has ended.
    pub fn remove(self) -> Result<(), Error> {
        remove_tree(&self.root)
    }
}

/// Removes `path` and everything under it, never following a link: a directory is made the owner's again (a cell may
/// have closed it) and emptied before it is removed.
fn remove_tree(path: &Path) -> Result<(), Error> {
    let meta = fs::symlink_metadata(path).map_err(io_at(path))?;
    if !meta.is_dir() {
        return fs::remove_file(path).map_err(io_at(path));
    }
    fs::set_permissions(path, fs::Permissions::from_mode(0o700)).map_err(io_at(path))?;
    for entry in fs::read_dir(path).map_err(io_at(path))? {
        remove_tree(&entry.map_err(io_at(path))?.path())?;
    }
    fs::remove_dir(path).map_err(io_at(path))
}

/// The evidence directory of the run `name`, `local/evidence/<name>/`, made new; it and `local/evidence/` are
/// readable and writable by their owner only (REQ-REG-13).
pub fn evidence(domain: &Path, name: &str) -> Result<PathBuf, Error> {
    let store = domain.join("local/evidence");
    fs::create_dir_all(&store)
        .and_then(|()| fs::set_permissions(&store, fs::Permissions::from_mode(0o700)))
        .map_err(io_at(&store))?;
    let dir = store.join(name);
    private_dir(&dir)?;
    Ok(dir)
}

/// What a run may take: its time, and the bytes kept of each output channel, past which the cell is ended.
#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub time: Duration,
    pub bytes: u64,
}

/// How a run ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum End {
    Exited(i32),
    Signalled,
    TimedOut,
    OutputCapped,
}

/// One output channel as kept: its file, the digest and length of what the file holds, and whether more was cut off.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kept {
    pub path: PathBuf,
    pub digest: String,
    pub bytes: u64,
    pub capped: bool,
}

/// A finished run: how it ended, its two channels as kept, and its whole seconds, from start to the cell's end.
#[derive(Debug, Clone)]
pub struct Ran {
    pub end: End,
    pub stdout: Kept,
    pub stderr: Kept,
    pub seconds: u64,
}

impl Ran {
    /// The refusal for `tool`'s run that did not succeed (REQ-RUN-6): `TOOL_FAILED`, saying how it ended and naming
    /// its evidence (REQ-REG-13); no byte of its output is in it.
    pub fn failure(&self, tool: &str) -> Error {
        let how = match self.end {
            End::Exited(n) => format!("exited with status {n}"),
            End::Signalled => "was ended by a signal".to_owned(),
            End::TimedOut => "was ended at its time limit".to_owned(),
            End::OutputCapped => "was ended when its output passed the bytes kept".to_owned(),
        };
        let dir = self.stdout.path.parent().unwrap_or(Path::new(""));
        refuse(Code::ToolFailed, Some(dir), format!("{tool} {how}; its output is kept in {}", dir.display()))
    }
}

/// Copies `pipe` into `file` up to `cap` bytes, hashing what it keeps, and reads the rest to its end without keeping
/// it, so that the end of the pipe still shows the cell's; past the cap it raises `over`.
fn keep(pipe: Option<impl Read>, mut file: File, cap: u64, over: &AtomicBool) -> std::io::Result<(String, u64, bool)> {
    let (mut hash, mut kept, mut capped) = (Sha256::new(), 0u64, false);
    let mut buf = vec![0u8; 1 << 16];
    if let Some(mut pipe) = pipe {
        loop {
            let n = match pipe.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            let room = usize::try_from(cap - kept).unwrap_or(usize::MAX).min(n);
            file.write_all(&buf[..room])?;
            hash.update(&buf[..room]);
            kept += room as u64;
            if room < n {
                capped = true;
                over.store(true, Ordering::SeqCst);
            }
        }
    }
    file.sync_all()?;
    Ok((hex(&hash.finalize()), kept, capped))
}

/// Runs `cell` under `backend` (`confinement-design.md` 3.2, 3.7): its standard output and error kept in `evidence`
/// as `stdout` and `stderr` (files `0600`), the cell ended at its time limit or once a channel passes its bytes, and
/// nothing returned before every process of the cell has ended. Only a failure of the backend's `stop` leaves the
/// cell possibly running (`TOOL_FAILED`): then nothing of its directories may be read or removed.
pub fn run(
    backend: &Backend,
    programs: &Programs,
    cell: &Cell,
    scratch: &Scratch,
    evidence: &Path,
    limits: Limits,
) -> Result<Ran, Error> {
    let mut files = Vec::new();
    for name in ["stdout", "stderr"] {
        let path = evidence.join(name);
        files.push((File::options().write(true).create_new(true).mode(0o600).open(&path).map_err(io_at(&path))?, path));
    }
    let begun = Instant::now();
    let mut started = (backend.start)(cell, programs, &scratch.root, evidence)?;
    let over = Arc::new(AtomicBool::new(false));
    let pipes: [Option<Box<dyn Read + Send>>; 2] = [
        started.child.stdout.take().map(|p| Box::new(p) as Box<dyn Read + Send>),
        started.child.stderr.take().map(|p| Box::new(p) as Box<dyn Read + Send>),
    ];
    let readers: Vec<_> = pipes
        .into_iter()
        .zip(files)
        .map(|(pipe, (file, path))| {
            let over = Arc::clone(&over);
            (std::thread::spawn(move || keep(pipe, file, limits.bytes, &over)), path)
        })
        .collect();
    let waited = loop {
        match started.child.try_wait() {
            Ok(Some(status)) => break Ok(status.code().map_or(End::Signalled, End::Exited)),
            Ok(None) if over.load(Ordering::SeqCst) => break Ok(End::OutputCapped),
            Ok(None) if begun.elapsed() >= limits.time => break Ok(End::TimedOut),
            Ok(None) => std::thread::sleep(Duration::from_millis(10)),
            Err(e) => break Err(e),
        }
    };
    // A cell not shown ended may still hold its channels open, so its readers are left, not waited for.
    (backend.stop)(&mut started)?;
    let seconds = begun.elapsed().as_secs();
    let mut kept = Vec::new();
    for (reader, path) in readers {
        let (digest, bytes, capped) = reader
            .join()
            .unwrap_or_else(|_| Err(std::io::Error::other("the output's reader stopped")))
            .map_err(io_at(&path))?;
        kept.push(Kept { path, digest, bytes, capped });
    }
    let end = waited.map_err(io_at(evidence))?;
    let end = if kept.iter().any(|k| k.capped) { End::OutputCapped } else { end };
    let stderr = kept.pop().expect("two channels");
    Ok(Ran { end, stdout: kept.pop().expect("two channels"), stderr, seconds })
}

fn unavailable(path: Option<&Path>, detail: impl Into<String>) -> Error {
    refuse(Code::ConfinementUnavailable, path, detail)
}

/// The probe's script: it must not see the file planted beside its directories (`$1`), and must write in `/work`.
const PROBE: &str =
    "if test -e \"$1\"; then echo seen; else echo unseen; fi; echo written > /work/probe && cat /work/probe";

/// The backend's own probe (`confinement-design.md` 3.9): an empty cell with a network that must run, write in its
/// `/work`, and fail to see a file planted beside its directories. A failure refuses and names its evidence.
fn probe(backend: &Backend, programs: &Programs, domain: &Path) -> Result<(), Error> {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos());
    let name = format!("probe-{}-{nanos}", std::process::id());
    let scratch = Scratch::create(domain, &name)?;
    let planted = scratch.root.join("planted");
    fs::write(&planted, b"planted\n").map_err(io_at(&planted))?;
    let evidence = evidence(domain, &name)?;
    let cell = Cell {
        home: scratch.home.clone(),
        tmp: scratch.tmp.clone(),
        mounts: vec![Mount { host: scratch.work.clone(), inside: "/work".into(), access: Access::ReadWrite }],
        network: Network::Outbound,
        nested: true,
        env: vec![("PATH".into(), "/usr/bin:/bin".into()), ("HOME".into(), "/home/agent".into())],
        cwd: "/work".into(),
        argv: ["/bin/sh", "-c", PROBE, "probe"].map(OsString::from).into_iter().chain([planted.into()]).collect(),
    };
    let limits = Limits { time: Duration::from_secs(30), bytes: 1 << 16 };
    let ran = run(backend, programs, &cell, &scratch, &evidence, limits);
    // A cell the backend could not show ended keeps its directories, unread (principle 4).
    if ran.as_ref().is_err_and(|e| e.code() == Some(Code::ToolFailed)) {
        return Err(unavailable(Some(&evidence), format!("{}'s probe did not end", backend.name)));
    }
    let written = fs::read(scratch.work.join("probe")).ok();
    scratch.remove()?;
    let ran = ran
        .map_err(|e| unavailable(Some(&evidence), format!("{}'s probe could not run: {}", backend.name, e.detail())))?;
    let out = fs::read(&ran.stdout.path).map_err(io_at(&ran.stdout.path))?;
    if ran.end == End::Exited(0) && out == b"unseen\nwritten\n" && written.as_deref() == Some(b"written\n") {
        return Ok(());
    }
    let words = format!("{}'s probe failed; its output is kept in {}", backend.name, evidence.display());
    Err(unavailable(Some(&evidence), words))
}

/// Whether a cell can be confined on this machine (REQ-SEC-1, REQ-SEC-2, S-1): a backend exists for this system,
/// every program it drives is present, and its probe passed at their paths and versions. The probe runs when any of
/// them is not verified by use yet, and its pass is marked on `detection`, which a write command keeps. Gives the
/// programs' paths.
pub fn available(backend: Option<&Backend>, detection: &mut Detection, domain: &Path) -> Result<Programs, Error> {
    let backend =
        backend.ok_or_else(|| unavailable(None, "no confinement backend exists for this operating system"))?;
    let mut programs = Programs(BTreeMap::new());
    // A backend that drives no program has nothing verified by use, so it is probed at every use.
    let mut verified = !backend.programs.is_empty();
    for p in backend.programs {
        let found = detection.get(Space::Program, p.executable).ok_or_else(|| {
            unavailable(None, format!("{}: not present on this machine, and {} needs it", p.executable, backend.name))
        })?;
        verified &= found.verified.is_some();
        programs.0.insert(p.executable, found.path.clone());
    }
    if !verified {
        probe(backend, &programs, domain)?;
        for p in backend.programs {
            detection.used(Space::Program, p.executable);
        }
    }
    Ok(programs)
}
