//! What Cargo will compile for each ring package, from `cargo metadata`, and what may change how it compiles.
//!
//! A ring package's `lib` and `bin` targets must have their root among the counted files, even when the same file is
//! also named as data. A ring package may have no build script (one can emit `cargo::rustc-cfg=test` and so compile the
//! excluded test modules) and may not be a procedural macro (one can read files the dep-info does not list). No Cargo
//! configuration in the repository and no `RUSTFLAGS` with a `cfg` in the environment may change how they compile.

use crate::json::{self, Value};
use crate::rings::{RingCount, canonical};
use std::path::{Path, PathBuf};
use std::process::Command;

/// One target of a workspace package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    pub package_dir: PathBuf,
    pub kinds: Vec<String>,
    pub src_path: PathBuf,
}

/// Runs `cargo metadata --no-deps` in the workspace at `root`, without network.
pub fn cargo_metadata(cargo: &Path, root: &Path) -> Result<String, String> {
    let out = Command::new(cargo)
        .current_dir(root)
        .args([
            "metadata",
            "--locked",
            "--offline",
            "--no-deps",
            "--format-version",
            "1",
        ])
        .output()
        .map_err(|e| format!("cannot run {}: {e}", cargo.display()))?;
    if !out.status.success() {
        return Err(format!(
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    String::from_utf8(out.stdout).map_err(|_| "cargo metadata printed non-UTF-8".to_string())
}

/// The targets of every package in `cargo metadata`'s output.
pub fn parse_targets(text: &str) -> Result<Vec<Target>, String> {
    let doc = json::parse(text)?;
    let mut targets = Vec::new();
    for package in doc.get("packages").map_or(&[][..], Value::as_array) {
        let manifest = package
            .get("manifest_path")
            .and_then(Value::as_str)
            .ok_or("package without manifest_path")?;
        let package_dir = Path::new(manifest)
            .parent()
            .unwrap_or(Path::new(""))
            .to_path_buf();
        for target in package.get("targets").map_or(&[][..], Value::as_array) {
            let kinds = target.get("kind").map_or(&[][..], Value::as_array);
            let src_path = target
                .get("src_path")
                .and_then(Value::as_str)
                .ok_or("target without src_path")?;
            targets.push(Target {
                package_dir: package_dir.clone(),
                kinds: kinds
                    .iter()
                    .filter_map(Value::as_str)
                    .map(str::to_string)
                    .collect(),
                src_path: PathBuf::from(src_path),
            });
        }
    }
    Ok(targets)
}

/// The problems of the ring packages' targets, as sentences.
pub fn target_problems(targets: &[Target], own: &[PathBuf], counts: &[RingCount]) -> Vec<String> {
    let own: Vec<PathBuf> = own.iter().map(|d| canonical(d)).collect();
    let counted: Vec<&PathBuf> = counts.iter().flat_map(|c| &c.sources).collect();
    let mut problems = Vec::new();
    for t in targets
        .iter()
        .filter(|t| own.contains(&canonical(&t.package_dir)))
    {
        let root = canonical(&t.src_path);
        for kind in &t.kinds {
            match kind.as_str() {
                "custom-build" => {
                    problems.push(format!("{} is a build script in a ring", root.display()))
                }
                "proc-macro" => problems.push(format!(
                    "{} is a procedural macro in a ring",
                    root.display()
                )),
                "test" | "bench" | "example" => {}
                _ if !counted.contains(&&root) => problems.push(format!(
                    "{} is the root of a {kind} target but not counted",
                    root.display()
                )),
                _ => {}
            }
        }
    }
    problems
}

/// A Cargo configuration in the repository, or `RUSTFLAGS` with a `cfg` among `env`, as sentences.
pub fn config_problems(
    root: &Path,
    env: impl IntoIterator<Item = (String, String)>,
) -> Vec<String> {
    let mut problems: Vec<String> = ["config", "config.toml"]
        .iter()
        .map(|name| root.join(".cargo").join(name))
        .filter(|path| path.exists())
        .map(|path| {
            format!(
                "{} may change how the rings compile; Fabrewk keeps none",
                path.display()
            )
        })
        .collect();
    for (name, value) in env {
        if name.ends_with("RUSTFLAGS") && value.contains("cfg") {
            problems.push(format!("{name} sets a cfg for the build: {value}"));
        }
    }
    problems
}
