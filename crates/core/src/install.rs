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

/// Git in the working tree at `root` as `assess` runs it (REQ-INS-1, D-102): the core's way (no hook, no file-system
/// monitor, objects as stored), taking no optional lock, fetching no missing object, with only the repository's own
/// configuration (no global or system file, no `-c` value or trace the environment carries) and no attributes file,
/// so that the person's Git configuration makes it neither write nor open anything: its exit status and standard
/// output, `input` on its standard input.
pub fn git(root: &Path, args: &[&str], input: &[u8]) -> Result<(Option<i32>, Vec<u8>), Error> {
    let repo = crate::record::Repo { root: root.to_path_buf(), prefix: Default::default() };
    let own = ["--no-optional-locks", "-c", "core.attributesFile=/dev/null"];
    let args: Vec<&std::ffi::OsStr> = own.iter().chain(args).map(std::ffi::OsStr::new).collect();
    let mut c = crate::record::git::command(&repo, &args);
    for (k, _) in std::env::vars_os() {
        if k.to_str().is_none_or(|k| k.starts_with("GIT_TRACE") || k.starts_with("GIT_CONFIG")) {
            c.env_remove(k);
        }
    }
    c.envs([("GIT_NO_LAZY_FETCH", "1"), ("GIT_CONFIG_GLOBAL", "/dev/null"), ("GIT_CONFIG_NOSYSTEM", "1")]);
    let out = crate::record::git::run(c, input)?;
    Ok((out.status.code(), out.stdout))
}
