//! Fabrewk's core ring (AD-5). It names no tool, provider or model; built-in modules depend on it, never the reverse.

/// The version of this build, as Cargo records it.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod alambique;
pub mod cli;
mod code;
pub mod confine;
pub mod detect;
pub mod domain;
pub mod fermento;
pub mod identity;
pub mod machine;
pub mod record;
pub mod scan;
pub mod settings;
pub mod table;
pub mod task;
pub mod view;

pub use code::{Code, Error};
