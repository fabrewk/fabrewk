//! Fabrewk's size counter and budget checks: a development tool kept in the workspace, never shipped (REQ-BUD-1).

pub mod budget;
pub mod count;
pub mod deps;
pub mod rings;

use budget::{
    APPROVED_RUNTIME_CRATES, MAX_BINARY_BYTES, MAX_DIRECT_RUNTIME_CRATES, MAX_RELEASE_CRATES, Ring,
};
use std::path::{Path, PathBuf};

/// The workspace this tool was built in.
pub fn workspace_root() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    root.canonicalize().unwrap_or(root)
}

/// The `cargo` that runs this tool, or the one on `PATH`.
pub fn cargo() -> PathBuf {
    std::env::var_os("CARGO").map_or_else(|| PathBuf::from("cargo"), PathBuf::from)
}

/// Every ring over its ceiling, as a sentence; empty when all are within.
pub fn ring_problems(counts: &[rings::RingCount]) -> Vec<String> {
    counts
        .iter()
        .filter(|c| !c.within())
        .map(|c| {
            format!(
                "ring {} has {} counted lines, over its limit of {}",
                c.name,
                c.lines,
                c.limit()
            )
        })
        .collect()
}

/// Every problem of the release build's crates, as a sentence; empty when within.
pub fn dependency_problems(report: &deps::Report, root: &Path, rings: &[Ring]) -> Vec<String> {
    let mut problems = Vec::new();
    if report.direct.len() > MAX_DIRECT_RUNTIME_CRATES {
        problems.push(format!(
            "{} direct runtime crates, over the limit of {MAX_DIRECT_RUNTIME_CRATES}",
            report.direct.len()
        ));
    }
    for name in &report.direct {
        if !APPROVED_RUNTIME_CRATES
            .iter()
            .any(|(approved, _)| approved == name)
        {
            problems.push(format!(
                "direct runtime crate {name} has no approved reason"
            ));
        }
    }
    if report.all.len() > MAX_RELEASE_CRATES {
        problems.push(format!(
            "{} crates in the release build, over the limit of {MAX_RELEASE_CRATES}",
            report.all.len()
        ));
    }
    for path in &report.local {
        if !rings
            .iter()
            .flat_map(|r| r.dirs)
            .any(|d| path.starts_with(root.join(d)))
        {
            problems.push(format!(
                "workspace package {} is in the release build but in no ring",
                path.display()
            ));
        }
    }
    problems
}

/// The binary's size against its ceiling, as a sentence when over.
pub fn binary_problem(bytes: u64) -> Option<String> {
    (bytes > MAX_BINARY_BYTES).then(|| {
        format!("the release binary has {bytes} bytes, over the limit of {MAX_BINARY_BYTES}")
    })
}
