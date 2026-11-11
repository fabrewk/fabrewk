//! The `fabrewk` program: the core's command surface (REQ-CLI-1), wired to the built-in modules: on Linux the
//! bubblewrap backend (part 9a); phase 9's tool adapters add their programs here (D-051).

use fabrewk_core::confine::Backend;
use fabrewk_core::detect::Program;
use std::process::ExitCode;

/// The confinement backend of this operating system; none where no backend exists, so that dispatch refuses there
/// (REQ-SEC-2).
#[cfg(target_os = "linux")]
const BACKEND: Option<&Backend> = Some(&fabrewk_confine_bwrap::BACKEND);
#[cfg(not(target_os = "linux"))]
const BACKEND: Option<&Backend> = None;

/// The programs the built-in tool adapters drive (D-051); the backend's are its own.
const MODULES: &[Program] = &[];

/// The environment variables that show a process runs inside an agent (REQ-SEC-7), declared here on each adapter's
/// behalf until phase 9 moves them into its module (D-062): Claude Code's, then Codex's sandbox variables (read from
/// the strings of `codex-cli` 0.166.0's binary). The core adds its own, `FABREWK_EXTENSION`.
const MARKERS: &[&str] = &["CLAUDECODE", "CODEX_SANDBOX", "CODEX_SANDBOX_NETWORK_DISABLED"];

fn main() -> ExitCode {
    let status = fabrewk_core::cli::main(std::env::args_os().skip(1).collect(), MODULES, MARKERS, BACKEND);
    ExitCode::from(u8::try_from(status).unwrap_or(70))
}
