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

/// What `onboard`'s interview is given (D-097 point 8; part 10c): where, the person's home, the built-in tool adapters,
/// and the terminal it reads the answers from and writes its questions to.
pub struct Interview<'a> {
    pub root: &'a Path,
    pub home: Option<&'a Path>,
    pub adapters: &'a [&'a Adapter],
    pub tty: &'a mut dyn std::io::BufRead,
    pub out: &'a mut dyn std::io::Write,
}

/// The installer, as the binary wires it.
pub struct Installer {
    /// `assess` (REQ-INS-1, REQ-INS-2); it writes nothing.
    pub assess: fn(&Asked) -> Result<Assessed, Vec<Error>>,
    /// The answers as a plan holds them, which `apply` compares with a recorded plan's when it finishes it (P10B-7).
    pub answers: fn(&Asked) -> Value,
    /// `onboard`'s interview (REQ-INS-4): it fills `apply`'s options with the answers, as they come, and gives the
    /// plan's digest once the person said yes and gave their words; none when it stopped, having said so.
    pub interview: fn(&mut Interview, &mut crate::cli::Options) -> Interviewed,
    /// The split of the bootstrap's `decisions.md` (D-113 points 1 and 3), from which `apply` writes the stream.
    pub split: crate::bootstrap::Split,
}

/// What the interview gives: the plan's digest on the person's yes, none when it stopped, or its refusals.
pub type Interviewed = Result<Option<String>, Vec<Error>>;

/// Git in the working tree at `root` as `assess` runs it (REQ-INS-1, D-102): the core's way (no hook, no file-system
/// monitor, objects as stored), taking no optional lock, fetching no missing object, with only the repository's own
/// configuration (no global or system file, no `-c` value or trace the environment carries) and no attributes file,
/// so that the person's Git configuration makes it neither write nor open anything; with `gitdir`, that Git directory
/// and `root` as its working tree, so that Git looks for no other repository (P10A-47): its exit status and standard
/// output, `input` on its standard input.
pub fn git<A: AsRef<std::ffi::OsStr>>(
    root: &Path,
    gitdir: Option<&Path>,
    args: &[A],
    input: &[u8],
) -> Result<(Option<i32>, Vec<u8>), Error> {
    let repo = crate::record::Repo { root: root.to_path_buf(), prefix: Default::default() };
    let own = ["--no-optional-locks", "-c", "core.attributesFile=/dev/null"].map(std::ffi::OsStr::new);
    let args: Vec<&std::ffi::OsStr> = own.into_iter().chain(args.iter().map(AsRef::as_ref)).collect();
    let mut c = crate::record::git::command(&repo, &args);
    if let Some(dir) = gitdir {
        c.env("GIT_DIR", dir).env("GIT_WORK_TREE", root);
    }
    for (k, _) in std::env::vars_os() {
        if k.to_str().is_none_or(|k| k.starts_with("GIT_TRACE") || k.starts_with("GIT_CONFIG")) {
            c.env_remove(k);
        }
    }
    c.envs([("GIT_NO_LAZY_FETCH", "1"), ("GIT_CONFIG_GLOBAL", "/dev/null"), ("GIT_CONFIG_NOSYSTEM", "1")]);
    let out = crate::record::git::run(c, input)?;
    Ok((out.status.code(), out.stdout))
}
