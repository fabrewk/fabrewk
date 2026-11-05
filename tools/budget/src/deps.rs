//! The release build's crates, read from `cargo tree` (REQ-BUD-2, REQ-BUD-4; core-specification 11.6).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

/// One line of `cargo tree --prefix depth --format {p}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeLine {
    pub depth: usize,
    pub name: String,
    pub version: String,
    /// The package's directory, for a path package.
    pub path: Option<PathBuf>,
}

/// What the release build depends on.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Report {
    /// Names of crates a workspace package depends on directly.
    pub direct: BTreeSet<String>,
    /// Every crate of the build that is not a workspace package, by name and version.
    pub all: BTreeSet<(String, String)>,
    /// Workspace packages in the build.
    pub local: BTreeSet<PathBuf>,
}

/// Parses `cargo tree --prefix depth --no-dedupe --format {p}`: a depth, then `name vVERSION`, then optionally
/// ` (path or source)` and ` (proc-macro)`.
pub fn parse_tree(text: &str) -> Result<Vec<TreeLine>, String> {
    let mut lines = Vec::new();
    for raw in text.lines().filter(|l| !l.trim().is_empty()) {
        let digits = raw.bytes().take_while(u8::is_ascii_digit).count();
        let depth = raw[..digits]
            .parse()
            .map_err(|_| format!("no depth in tree line: {raw}"))?;
        let mut parts = raw[digits..].splitn(3, ' ');
        let (Some(name), Some(version)) = (parts.next(), parts.next()) else {
            return Err(format!("no name and version in tree line: {raw}"));
        };
        let Some(version) = version.strip_prefix('v') else {
            return Err(format!("no version in tree line: {raw}"));
        };
        let path = parts
            .next()
            .and_then(|rest| rest.strip_prefix('('))
            .and_then(|rest| rest.split_once(')'))
            .map(|(inside, _)| inside)
            .filter(|inside| inside.starts_with('/'))
            .map(PathBuf::from);
        lines.push(TreeLine {
            depth,
            name: name.to_string(),
            version: version.to_string(),
            path,
        });
    }
    Ok(lines)
}

/// Classifies the tree: a line is local when its path is under `root`.
pub fn analyse(lines: &[TreeLine], root: &Path) -> Result<Report, String> {
    let mut report = Report::default();
    let mut stack: Vec<bool> = Vec::new(); // whether each ancestor is local
    for line in lines {
        if line.depth > stack.len() {
            return Err(format!(
                "tree line deeper than its parent: {} at depth {}",
                line.name, line.depth
            ));
        }
        stack.truncate(line.depth);
        let local = line.path.as_ref().is_some_and(|p| p.starts_with(root));
        if local {
            report.local.insert(line.path.clone().unwrap_or_default());
        } else {
            if stack.last() == Some(&true) {
                report.direct.insert(line.name.clone());
            }
            report.all.insert((line.name.clone(), line.version.clone()));
        }
        stack.push(local);
    }
    Ok(report)
}

/// Runs `cargo tree` for `package` and `target` in the workspace at `root`, without network.
pub fn cargo_tree(
    cargo: &Path,
    root: &Path,
    package: &str,
    target: &str,
) -> Result<String, String> {
    let out = Command::new(cargo)
        .current_dir(root)
        .args([
            "tree",
            "--locked",
            "--offline",
            "--package",
            package,
            "--edges",
            "normal,build",
        ])
        .args([
            "--target",
            target,
            "--prefix",
            "depth",
            "--no-dedupe",
            "--format",
            "{p}",
        ])
        .output()
        .map_err(|e| format!("cannot run {}: {e}", cargo.display()))?;
    if !out.status.success() {
        return Err(format!(
            "cargo tree failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    String::from_utf8(out.stdout).map_err(|_| "cargo tree printed non-UTF-8".to_string())
}
