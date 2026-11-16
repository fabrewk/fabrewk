//! The `fabrewk` program: the core's command surface (REQ-CLI-1), wired to the built-in modules: on Linux the
//! bubblewrap backend (part 9a); the tool adapters, Claude Code's (part 9c) and Codex's (part 9d), each with its
//! programs, markers and residual risks (D-051); the installer (part 10a).

use fabrewk_core::adapter::Adapter;
use fabrewk_core::confine::Backend;
use std::process::ExitCode;

/// The confinement backend of this operating system; none where no backend exists, so that dispatch refuses there
/// (REQ-SEC-2).
#[cfg(target_os = "linux")]
const BACKEND: Option<&Backend> = Some(&fabrewk_confine_bwrap::BACKEND);
#[cfg(not(target_os = "linux"))]
const BACKEND: Option<&Backend> = None;

/// The built-in tool adapters (AD-5); each declares its markers (REQ-SEC-7), to which the core adds
/// `FABREWK_EXTENSION`.
const ADAPTERS: &[&Adapter] = &[&fabrewk_adapter_claude::ADAPTER, &fabrewk_adapter_codex::ADAPTER];

fn main() -> ExitCode {
    let wired = (ADAPTERS, BACKEND, &fabrewk_install::INSTALLER);
    let status = fabrewk_core::cli::main(std::env::args_os().skip(1).collect(), wired);
    ExitCode::from(u8::try_from(status).unwrap_or(70))
}
