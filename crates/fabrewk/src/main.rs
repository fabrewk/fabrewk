//! The `fabrewk` program: the core's command surface (REQ-CLI-1), wired to the built-in modules. Phase 7 has no
//! module yet; phase 9's tool adapters and confinement backend add their programs here (D-051).

use std::process::ExitCode;

/// The programs the built-in modules drive (D-051).
const MODULES: &[fabrewk_core::detect::Program] = &[];

fn main() -> ExitCode {
    let status = fabrewk_core::cli::main(std::env::args_os().skip(1).collect(), MODULES);
    ExitCode::from(u8::try_from(status).unwrap_or(70))
}
