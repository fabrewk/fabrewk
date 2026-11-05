//! The `fabrewk` program. Skeleton of phase 7: it only answers `--version`; the command surface comes from the core's
//! single declaration of commands (REQ-CLI-1) in a later part.

use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args_os().skip(1);
    match (args.next(), args.next()) {
        (Some(flag), None) if flag == "--version" => {
            println!("fabrewk {}", fabrewk_core::VERSION);
            ExitCode::SUCCESS
        }
        // Usage error (REQ-CLI-3).
        _ => ExitCode::from(2),
    }
}
