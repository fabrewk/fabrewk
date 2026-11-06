//! The `fabrewk` program: the core's command surface (REQ-CLI-1), wired to the built-in modules. Phase 7 has no
//! module yet; phase 9's tool adapters and confinement backend add their programs here (D-051).

use std::process::ExitCode;

/// The programs the built-in modules drive (D-051).
const MODULES: &[fabrewk_core::detect::Program] = &[];

/// The environment variables that show a process runs inside an agent (REQ-SEC-7), declared here on each adapter's
/// behalf until phase 9 moves them into its module (D-062): Claude Code's, then Codex's sandbox variables (read from
/// the strings of `codex-cli` 0.166.0's binary). The core adds its own, `FABREWK_EXTENSION`.
const MARKERS: &[&str] = &["CLAUDECODE", "CODEX_SANDBOX", "CODEX_SANDBOX_NETWORK_DISABLED"];

fn main() -> ExitCode {
    let status = fabrewk_core::cli::main(std::env::args_os().skip(1).collect(), MODULES, MARKERS);
    ExitCode::from(u8::try_from(status).unwrap_or(70))
}
