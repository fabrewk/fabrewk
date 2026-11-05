//! Fabrewk's size counter and budget checks: a development tool kept in the workspace, never shipped (REQ-BUD-1).

pub mod budget;
pub mod count;
pub mod deps;
pub mod json;
pub mod rings;
pub mod sources;

use budget::{
    APPROVED_RUNTIME_CRATES, MAX_BINARY_BYTES, MAX_DIRECT_RUNTIME_CRATES, MAX_RELEASE_CRATES, Ring,
};
use std::path::{Path, PathBuf};

/// The release build's packages, every feature and every platform counted.
pub fn release_report(root: &Path, counts: &[rings::RingCount]) -> Result<deps::Report, String> {
    let metadata = deps::cargo_metadata(&cargo(), root)?;
    deps::analyse(
        &metadata,
        budget::BINARY_PACKAGE,
        &own_packages(root, budget::RINGS),
        counts,
    )
}

/// The workspace this tool was built in.
pub fn workspace_root() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    root.canonicalize().unwrap_or(root)
}

/// The ring directories under `root`, as absolute paths: Fabrewk's own packages.
pub fn own_packages(root: &Path, rings: &[Ring]) -> Vec<PathBuf> {
    rings
        .iter()
        .flat_map(|r| r.dirs)
        .map(|d| root.join(d))
        .collect()
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
pub fn dependency_problems(report: &deps::Report) -> Vec<String> {
    let mut problems = Vec::new();
    if report.direct.len() > MAX_DIRECT_RUNTIME_CRATES {
        problems.push(format!(
            "{} direct runtime crates, over the limit of {MAX_DIRECT_RUNTIME_CRATES}",
            report.direct.len()
        ));
    }
    let names: std::collections::BTreeSet<&String> = report.direct.values().collect();
    for name in names {
        if !APPROVED_RUNTIME_CRATES
            .iter()
            .any(|(approved, _)| approved == name)
        {
            problems.push(format!(
                "direct runtime crate {name} has no approved reason"
            ));
        }
    }
    problems.extend(report.problems.iter().cloned());
    if report.all.len() > MAX_RELEASE_CRATES {
        problems.push(format!(
            "{} crates in the release build, over the limit of {MAX_RELEASE_CRATES}",
            report.all.len()
        ));
    }
    problems
}

/// Every file the compiler read for the binary (its dep-info, `<binary>.d`) that the count did not see, as a
/// sentence; empty when every one is counted or is a data file a counted file names.
pub fn source_problems(dep_info: &str, counts: &[rings::RingCount]) -> Result<Vec<String>, String> {
    let deps = sources::parse_dep_info(dep_info)?;
    Ok(sources::unaccounted(&deps, counts)
        .iter()
        .map(|p| {
            format!(
                "{} is compiled into the binary but not counted",
                p.display()
            )
        })
        .collect())
}

/// The binary's size against its ceiling, as a sentence when over.
pub fn binary_problem(bytes: u64) -> Option<String> {
    (bytes > MAX_BINARY_BYTES).then(|| {
        format!("the release binary has {bytes} bytes, over the limit of {MAX_BINARY_BYTES}")
    })
}
