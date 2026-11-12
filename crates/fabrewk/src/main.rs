//! The `fabrewk` program: the core's command surface (REQ-CLI-1), wired to the built-in modules: on Linux the
//! bubblewrap backend (part 9a); the tool adapters, Claude Code's first (part 9c), each with its programs, markers and
//! residual risks (D-051).

use fabrewk_core::adapter::Adapter;
use fabrewk_core::confine::Backend;
use std::process::ExitCode;

/// The confinement backend of this operating system; none where no backend exists, so that dispatch refuses there
/// (REQ-SEC-2).
#[cfg(target_os = "linux")]
const BACKEND: Option<&Backend> = Some(&fabrewk_confine_bwrap::BACKEND);
#[cfg(not(target_os = "linux"))]
const BACKEND: Option<&Backend> = None;

/// The built-in tool adapters (AD-5).
const ADAPTERS: &[&Adapter] = &[&fabrewk_adapter_claude::ADAPTER];

/// The environment variables that show a process runs inside an agent (REQ-SEC-7) of an adapter not built yet,
/// declared here on its behalf until part 9d moves them into its module (D-062): Codex's sandbox variables (read from
/// the strings of `codex-cli` 0.166.0's binary). Each adapter declares its own; the core adds `FABREWK_EXTENSION`.
const MARKERS: &[&str] = &["CODEX_SANDBOX", "CODEX_SANDBOX_NETWORK_DISABLED"];

fn main() -> ExitCode {
    let status = fabrewk_core::cli::main(std::env::args_os().skip(1).collect(), ADAPTERS, MARKERS, BACKEND);
    ExitCode::from(u8::try_from(status).unwrap_or(70))
}
