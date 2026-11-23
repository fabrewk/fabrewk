//! The size budget (REQ-BUD-2) and the approved runtime crates (REQ-BUD-4).
//!
//! FROZEN by the maestro after measuring the skeleton (D-042): AD-5's ceilings, the binary's 8 MB read as 8,000,000
//! bytes; the core's raised from 4,000 to 7,500 after part 7d1 was measured (D-054), to 8,000 before part 8c (D-074), to 11,000 when phase 9 opened (D-075), to 12,000 before part 9g (D-086) and to 12,500 when phase 10 opened (D-097), to 12,750 when phase 11 opened (D-113) and to 13,100 when the formula's subphase 1a opened (the maestro's answer in its chat, 2026-11-22). Changing a number here, or the list of approved crates, is a maestro's recorded decision with its reason,
//! never an agent's.

/// What a ring is, which sets its ceiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RingKind {
    Core,
    ToolAdapter,
    Confinement,
    Installer,
}

impl RingKind {
    /// The ceiling in counted lines (REQ-BUD-1's unit).
    pub const fn limit(self) -> usize {
        match self {
            RingKind::Core => 13_100,
            RingKind::ToolAdapter => 600,
            RingKind::Confinement => 400,
            RingKind::Installer => 1_400,
        }
    }
}

/// A ring is its directories, relative to the workspace root (REQ-BUD-1).
#[derive(Debug, Clone, Copy)]
pub struct Ring {
    pub name: &'static str,
    pub kind: RingKind,
    pub dirs: &'static [&'static str],
}

/// Every ring of the program. Every Rust file outside `tools/` must be in one of them.
pub const RINGS: &[Ring] = &[
    Ring { name: "core", kind: RingKind::Core, dirs: &["crates/core", "crates/fabrewk"] },
    // Phase 9's modules, each a ring at its kind's frozen ceiling (D-075 point 2).
    Ring { name: "confine-bwrap", kind: RingKind::Confinement, dirs: &["crates/confine-bwrap"] },
    Ring { name: "adapter-claude", kind: RingKind::ToolAdapter, dirs: &["crates/adapter-claude"] },
    Ring { name: "adapter-codex", kind: RingKind::ToolAdapter, dirs: &["crates/adapter-codex"] },
    // Phase 10's installer, at its kind's frozen ceiling (D-097 point 2).
    Ring { name: "install", kind: RingKind::Installer, dirs: &["crates/install"] },
];

/// Direct runtime crates: those a ring crate depends on directly.
pub const MAX_DIRECT_RUNTIME_CRATES: usize = 8;

/// Crates in the release build (`cargo tree -e normal,build` for the release target), Fabrewk's own not counted.
pub const MAX_RELEASE_CRATES: usize = 50;

/// The stripped release binary, in bytes (8 MB, decimal).
pub const MAX_BINARY_BYTES: u64 = 8_000_000;

/// The release target (AD-4, REQ-BUD-3).
pub const RELEASE_TARGET: &str = "aarch64-unknown-linux-musl";

/// The package whose tree is the release build.
pub const BINARY_PACKAGE: &str = "fabrewk";

/// The MSRV: the Fedora rustc at phase 7's start (AD-4, REQ-BUD-3, D-041).
pub const MSRV: &str = "1.99.1";

/// Runtime crates a maestro approved, with the decision that holds each one's reason (AD-4's table; section 9 of the
/// core specification for `lexopt`).
pub const APPROVED_RUNTIME_CRATES: &[(&str, &str)] =
    &[("serde", "D-036"), ("serde_json", "D-036"), ("toml", "D-036"), ("sha2", "D-036"), ("lexopt", "D-040")];
