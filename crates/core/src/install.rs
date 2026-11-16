//! The installer's contract (D-097 point 1; part 10a): the core declares `assess` and `apply`, parses their options and
//! words every refusal; the built-in module `install` holds what is the installation's own, what `assess` reads, the
//! verdict's rules, the plan, its files' contents and its digest. The core names no installer: the binary wires one.

use crate::adapter::Adapter;
use crate::code::Error;
use serde_json::Value;
use std::path::Path;

/// What `assess` is asked: where, the person's home, the answers as the command line gave them, and the built-in tool
/// adapters `--tool` may name.
pub struct Asked<'a> {
    pub root: &'a Path,
    pub home: Option<&'a Path>,
    pub options: &'a crate::cli::Options,
    pub adapters: &'a [&'a Adapter],
}

/// What `assess` gives: its document (`fabrewk.assessment/1`) and its human text, in the language the answers name.
pub type Assessed = (Value, String);

/// The installer, as the binary wires it.
pub struct Installer {
    /// `assess` (REQ-INS-1, REQ-INS-2); it writes nothing.
    pub assess: fn(&Asked) -> Result<Assessed, Vec<Error>>,
}

/// Git in the working tree at `root` as the core runs it (no hook, no file-system monitor, objects as stored) and
/// taking no optional lock, so that it writes nothing (REQ-INS-1): its exit status and standard output.
pub fn git(root: &Path, args: &[&str]) -> Result<(Option<i32>, Vec<u8>), Error> {
    let repo = crate::record::Repo { root: root.to_path_buf(), prefix: Default::default() };
    let args: Vec<&std::ffi::OsStr> = ["--no-optional-locks"].iter().chain(args).map(std::ffi::OsStr::new).collect();
    let out = crate::record::git::run(crate::record::git::command(&repo, &args), b"")?;
    Ok((out.status.code(), out.stdout))
}

/// Refuses `NOT_A_REPOSITORY` unless `root` is the top of a repository's main working tree, as REQ-DOM-7's check does.
pub fn main_worktree(root: &Path) -> Result<(), Error> {
    crate::record::check_main_worktree(&crate::record::Repo { root: root.to_path_buf(), prefix: Default::default() })
}
