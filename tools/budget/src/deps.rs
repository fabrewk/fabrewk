//! The release build's packages, from one `cargo metadata` with every feature and the release target's platform
//! (REQ-BUD-2, REQ-BUD-4; core-specification 11.6; D-044).
//!
//! The world is closed: every local package (one with no registry source) must be a workspace member, and every local
//! package the binary reaches must be a ring package whose `lib` and `bin` roots are counted, with no build script and
//! no procedural macro. Everything else the binary reaches is a crate, counted and, when direct, approved.

use crate::json::{self, Value};
use crate::rings::{RingCount, canonical};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Runs `cargo metadata` for every feature and `target`'s platform in the workspace at `root`, without network.
pub fn cargo_metadata(cargo: &Path, root: &Path, target: &str) -> Result<String, String> {
    let out = Command::new(cargo)
        .current_dir(root)
        .args([
            "metadata",
            "--locked",
            "--offline",
            "--format-version",
            "1",
            "--all-features",
        ])
        .args(["--filter-platform", target])
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

/// What the release build depends on.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Report {
    /// Names of crates a ring package the binary reaches depends on directly.
    pub direct: BTreeSet<String>,
    /// Every crate the binary reaches that is not a local package, by name and version.
    pub all: BTreeSet<(String, String)>,
    /// The ring packages the binary reaches, by directory.
    pub local: BTreeSet<PathBuf>,
    /// What breaks the closed world, as sentences.
    pub problems: Vec<String>,
}

struct Package<'a> {
    name: &'a str,
    version: &'a str,
    local: bool,
    dir: PathBuf,
    targets: Vec<(Vec<&'a str>, PathBuf)>,
}

fn text<'a>(v: &'a Value, key: &str) -> Result<&'a str, String> {
    v.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("cargo metadata: no {key}"))
}

/// Reads `cargo metadata`'s output from the package named `binary`: what it reaches through normal and build
/// dependencies, against the ring directories `own` and the counted files of `counts`.
pub fn analyse(
    metadata: &str,
    binary: &str,
    own: &[PathBuf],
    counts: &[RingCount],
) -> Result<Report, String> {
    let doc = json::parse(metadata)?;
    let own: Vec<PathBuf> = own.iter().map(|d| canonical(d)).collect();
    let counted: BTreeSet<&PathBuf> = counts.iter().flat_map(|c| &c.sources).collect();
    let members: BTreeSet<&str> = doc
        .get("workspace_members")
        .map_or(&[][..], Value::as_array)
        .iter()
        .filter_map(Value::as_str)
        .collect();
    let mut packages = BTreeMap::new();
    for p in doc.get("packages").map_or(&[][..], Value::as_array) {
        let mut targets = Vec::new();
        for t in p.get("targets").map_or(&[][..], Value::as_array) {
            let kinds = t
                .get("kind")
                .map_or(&[][..], Value::as_array)
                .iter()
                .filter_map(Value::as_str)
                .collect();
            targets.push((kinds, canonical(Path::new(text(t, "src_path")?))));
        }
        let manifest = Path::new(text(p, "manifest_path")?);
        let package = Package {
            name: text(p, "name")?,
            version: text(p, "version")?,
            local: p.get("source").is_none_or(|s| *s == Value::Null),
            dir: canonical(manifest.parent().unwrap_or(Path::new(""))),
            targets,
        };
        packages.insert(text(p, "id")?, package);
    }
    let mut edges: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let nodes = doc
        .get("resolve")
        .and_then(|r| r.get("nodes"))
        .ok_or("cargo metadata: no resolve")?;
    for node in nodes.as_array() {
        let runtime = node
            .get("deps")
            .map_or(&[][..], Value::as_array)
            .iter()
            .filter(|d| {
                d.get("dep_kinds")
                    .map_or(&[][..], Value::as_array)
                    .iter()
                    .any(|k| k.get("kind").and_then(Value::as_str) != Some("dev"))
            });
        edges.insert(
            text(node, "id")?,
            runtime.map(|d| text(d, "pkg")).collect::<Result<_, _>>()?,
        );
    }

    let mut report = Report::default();
    for (id, p) in &packages {
        if p.local && !members.contains(id) {
            report.problems.push(format!(
                "{} is a local package outside the workspace",
                p.dir.display()
            ));
        }
    }
    let root = packages
        .iter()
        .find(|(id, p)| p.name == binary && members.contains(*id))
        .map(|(id, _)| *id)
        .ok_or_else(|| format!("cargo metadata: no workspace package {binary}"))?;
    let (mut seen, mut queue) = (BTreeSet::from([root]), vec![root]);
    while let Some(id) = queue.pop() {
        let p = &packages[id];
        let ring = p.local && own.contains(&p.dir);
        if ring {
            report.local.insert(p.dir.clone());
            report.problems.extend(target_problems(p, &counted));
        } else if p.local {
            report.problems.push(format!(
                "{} is a local package the binary reaches outside the rings",
                p.dir.display()
            ));
        } else {
            report
                .all
                .insert((p.name.to_string(), p.version.to_string()));
        }
        for dep in edges.get(id).map_or(&[][..], Vec::as_slice) {
            let d = packages
                .get(dep)
                .ok_or_else(|| format!("cargo metadata: unknown package {dep}"))?;
            if ring && !d.local {
                report.direct.insert(d.name.to_string());
            }
            if seen.insert(dep) {
                queue.push(dep);
            }
        }
    }
    Ok(report)
}

/// A ring package's targets: `lib` and `bin` roots counted, no build script, no procedural macro.
fn target_problems(p: &Package, counted: &BTreeSet<&PathBuf>) -> Vec<String> {
    let mut problems = Vec::new();
    for (kinds, root) in &p.targets {
        for kind in kinds {
            match *kind {
                "custom-build" => {
                    problems.push(format!("{} is a build script in a ring", root.display()))
                }
                "proc-macro" => problems.push(format!(
                    "{} is a procedural macro in a ring",
                    root.display()
                )),
                "test" | "bench" | "example" => {}
                _ if !counted.contains(root) => problems.push(format!(
                    "{} is the root of a {kind} target but not counted",
                    root.display()
                )),
                _ => {}
            }
        }
    }
    problems
}

/// A `.cargo` directory anywhere in the repository, or `RUSTFLAGS` with a `cfg` among `env`, as sentences: either
/// can change how the rings compile, and Fabrewk keeps none.
pub fn config_problems(
    root: &Path,
    env: impl IntoIterator<Item = (String, String)>,
) -> Vec<String> {
    let mut problems = Vec::new();
    let mut dirs = vec![root.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        let mut entries: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
        entries.sort();
        for path in entries {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            if name == ".cargo" {
                problems.push(format!(
                    "{} may change how the rings compile; Fabrewk keeps none",
                    path.display()
                ));
            } else if path.is_dir()
                && !path.is_symlink()
                && !(dir == root && [".git", "target"].contains(&name))
            {
                dirs.push(path);
            }
        }
    }
    problems.sort();
    for (name, value) in env {
        if name.ends_with("RUSTFLAGS") && value.contains("cfg") {
            problems.push(format!("{name} sets a cfg for the build: {value}"));
        }
    }
    problems
}
