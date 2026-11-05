//! What is present on the machine (REQ-SET-8, REQ-SET-9; AD-2, "Present, apart from permitted"; part 7c2b, D-051).
//! A command that needs it detects at its start, in memory: the core's own programs, each program a built-in module
//! drives, every catalogued tool, every profile's required tools and every name `machine.toml` declares, each found in
//! `PATH` or where `machine.toml` declares it, with its version. Write commands keep the result in
//! `local/cache/detected.json`, which carries one thing forward: when a use verified a program at that path and
//! version. Presence is never read from it. Naming a tool grants nothing: the effective set is what is both permitted
//! and present.

use crate::code::{Code, Error, io_at, refuse};
use crate::record::{decode, encode, now_utc, write_atomic};
use crate::settings::{Effective, Policy, Source, Val};
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A program a built-in module drives: the contract every tool adapter and confinement backend fulfils (D-051).
#[derive(Debug, Clone, Copy)]
pub struct Program {
    /// The adapter it serves (`tools.<name>.adapter`); `None` for a program known by its executable's name.
    pub adapter: Option<&'static str>,
    /// The executable's name, searched in `PATH`.
    pub executable: &'static str,
    /// The arguments that print its version on the first line of its standard output without calling any model;
    /// empty when it prints none.
    pub version: &'static [&'static str],
}

/// The core's own programs (AD-4), found in `PATH` only, as the core runs them. `ssh-keygen` prints no version.
pub const CORE: [Program; 2] = [
    Program { adapter: None, executable: "git", version: &["--version"] },
    Program { adapter: None, executable: "ssh-keygen", version: &[] },
];

/// How a present program was found: in `PATH`, or where `machine.toml` declares it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum How {
    Observed,
    Declared,
}

/// A present program: where, its version, how it was found, and when a use last verified it at that path and version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Found {
    pub path: PathBuf,
    pub version: Option<String>,
    pub how: How,
    pub verified: Option<String>,
}

/// One detection: when it ran, and every name looked for, `None` when absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Detection {
    pub at: String,
    pub tools: BTreeMap<String, Option<Found>>,
}

/// What to look for under one name: its executable (none when no module drives a catalogued tool's adapter), its
/// version's arguments, and whether `machine.toml` may say where it is.
struct Want<'a> {
    executable: Option<&'a str>,
    version: &'a [&'a str],
    declarable: bool,
}

fn text<'a>(eff: &'a Effective, key: &str) -> Option<&'a str> {
    match eff.values.get(key) {
        Some((Val::Str(s), _)) => Some(s),
        _ => None,
    }
}

fn list<'a>(eff: &'a Effective, key: &str) -> &'a [String] {
    match eff.values.get(key) {
        Some((Val::List(l), _)) => l,
        _ => &[],
    }
}

/// The names `<name>` that hold a value at `<table>.<name>.<leaf>`.
fn names<'a>(eff: &'a Effective, table: &'a str, leaf: &'a str) -> impl Iterator<Item = &'a str> {
    eff.values
        .keys()
        .filter_map(move |k| k.strip_prefix(table)?.strip_prefix('.')?.strip_suffix(leaf)?.strip_suffix('.'))
}

fn cache(domain: &Path) -> PathBuf {
    domain.join("local/cache/detected.json")
}

fn is_executable(path: &Path) -> bool {
    std::fs::metadata(path).is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
}

/// The first executable file named `executable` in an absolute directory of `path`; a name with a `/` is none.
fn search(path: Option<&OsStr>, executable: &str) -> Option<PathBuf> {
    if executable.contains('/') {
        return None;
    }
    std::env::split_paths(path?).filter(|d| d.is_absolute()).map(|d| d.join(executable)).find(|p| is_executable(p))
}

/// The first line of what `args` print, when they succeed; the program reads nothing.
fn version(path: &Path, args: &[&str]) -> Option<String> {
    if args.is_empty() {
        return None;
    }
    let out = Command::new(path).args(args).stdin(Stdio::null()).stderr(Stdio::null()).output().ok()?;
    let first = String::from_utf8_lossy(&out.stdout).lines().next()?.trim().to_owned();
    (out.status.success() && !first.is_empty()).then_some(first)
}

/// Detects, in memory, every program `eff` and `modules` name (REQ-SET-8). `path` is the value of `PATH`. The cache
/// under `domain` is read only for when a use verified a program; a cache that does not read is none. Writes nothing.
pub fn detect(domain: &Path, eff: &Effective, modules: &[Program], path: Option<&OsStr>) -> Detection {
    let mut wanted: BTreeMap<&str, Want> = BTreeMap::new();
    let program = |p: &Program, declarable| Want { executable: Some(p.executable), version: p.version, declarable };
    for p in &CORE {
        wanted.insert(p.executable, program(p, false));
    }
    for p in modules.iter().filter(|p| p.adapter.is_none()) {
        wanted.entry(p.executable).or_insert_with(|| program(p, true));
    }
    for name in names(eff, "tools", "adapter") {
        let adapter = text(eff, &format!("tools.{name}.adapter"));
        let module = modules.iter().find(|p| p.adapter.is_some() && p.adapter == adapter);
        let none = Want { executable: None, version: &[], declarable: true };
        wanted.entry(name).or_insert_with(|| module.map_or(none, |p| program(p, true)));
    }
    let required =
        names(eff, "profiles", "requiredTools").flat_map(|p| list(eff, &format!("profiles.{p}.requiredTools")));
    for name in required.map(String::as_str) {
        wanted.entry(name).or_insert(Want { executable: Some(name), version: &[], declarable: true });
    }
    for name in names(eff, "tools", "path") {
        wanted.entry(name).or_insert(Want { executable: None, version: &[], declarable: true });
    }
    let marks = std::fs::read(cache(domain)).ok().and_then(|b| decode(&b));
    let marks = marks.as_ref().and_then(|v| v.get("tools")?.as_object());
    let mut tools = BTreeMap::new();
    for (name, want) in wanted {
        let declared = if want.declarable { text(eff, &format!("tools.{name}.path")) } else { None };
        let (found, how) = match declared {
            Some(p) => (Some(PathBuf::from(p)).filter(|p| is_executable(p)), How::Declared),
            None => (want.executable.and_then(|e| search(path, e)), How::Observed),
        };
        let found = found.map(|path| {
            let version = version(&path, want.version);
            let mark = marks.and_then(|m| m.get(name)?.as_object());
            let same = mark.is_some_and(|m| {
                m.get("path") == Some(&json!(path.to_string_lossy())) && m.get("version") == Some(&json!(version))
            });
            let verified = mark.filter(|_| same).and_then(|m| Some(m.get("verified")?.as_str()?.to_owned()));
            Found { path, version, how, verified }
        });
        tools.insert(name.to_owned(), found);
    }
    Detection { at: now_utc(), tools }
}

impl Detection {
    /// Whether `name` was found.
    pub fn present(&self, name: &str) -> bool {
        matches!(self.tools.get(name), Some(Some(_)))
    }

    /// Records that a use of `name` succeeded now, at the path and version detected; a write command keeps it.
    pub fn used(&mut self, name: &str) {
        if let Some(Some(found)) = self.tools.get_mut(name) {
            found.verified = Some(now_utc());
        }
    }

    /// Replaces `local/cache/detected.json` under `domain` with this detection, dated; only write commands call it
    /// (REQ-REG-12, REQ-DOM-5). It is a cache: deleting it loses only when each program was verified.
    pub fn keep(&self, domain: &Path) -> Result<(), Error> {
        let entry = |f: &Found| {
            let how = if f.how == How::Declared { "declared" } else { "observed" };
            json!({"how": how, "path": f.path.to_string_lossy(), "verified": f.verified, "version": f.version})
        };
        let tools: Map<String, Value> =
            self.tools.iter().map(|(n, f)| (n.clone(), f.as_ref().map_or(Value::Null, entry))).collect();
        let bytes = encode(&json!({"at": self.at, "schema": 1, "tools": tools})).expect("three levels deep");
        let dir = domain.join("local/cache");
        std::fs::create_dir_all(&dir).map_err(io_at(&dir))?;
        write_atomic(&cache(domain), &bytes)
    }
}

/// The effective tools (REQ-SET-8): catalogued in `eff` and present in `detection`. A permitted tool that is absent,
/// and a present one the catalogue does not hold, are both outside it.
pub fn effective_tools(eff: &Effective, detection: &Detection) -> BTreeSet<String> {
    names(eff, "tools", "adapter").filter(|n| detection.present(n)).map(str::to_owned).collect()
}

/// Validates the profile `name` before a run (REQ-SET-9, D-051): its tool in the catalogue, its model in the catalogue
/// and of a provider the tool uses, that provider permitted and the effort one of the model's, else `SETTINGS_INVALID`
/// with the profile's key and the line, or the stage or task, that set it; then the tool and every required tool
/// present, else `TOOL_MISSING`.
pub fn check_profile(policy: &Policy, eff: &Effective, detection: &Detection, name: &str) -> Result<(), Error> {
    let key = |leaf: &str| format!("profiles.{name}.{leaf}");
    let file = Some(policy.settings.path.as_path());
    let bad = |leaf: &str, why: &str| {
        let k = key(leaf);
        let place = match (eff.values.get(&k).map(|v| &v.1), policy.settings.values.get(&k)) {
            (Some(Source::Stage(id) | Source::Task(id)), _) => id.clone(),
            (_, Some(entry)) => format!("line {}", entry.line),
            _ => "no value".into(),
        };
        Err(refuse(Code::SettingsInvalid, file, format!("{k} ({place}): {why}")))
    };
    if text(eff, &key("kind")).is_none() {
        return Err(refuse(Code::SettingsInvalid, file, format!("profiles.{name}: no such profile")));
    }
    let tool = text(eff, &key("tool")).unwrap_or_default();
    if text(eff, &format!("tools.{tool}.adapter")).is_none() {
        return bad("tool", "not a tool of the catalogue");
    }
    let model = text(eff, &key("model")).unwrap_or_default();
    let Some(provider) = text(eff, &format!("models.{model}.provider")) else {
        return bad("model", "not a model of the catalogue");
    };
    if !list(eff, &format!("tools.{tool}.providers")).iter().any(|p| p == provider) {
        return bad("model", "of a provider the tool does not use");
    }
    if eff.values.get(&format!("providers.{provider}.allowed")).map(|v| &v.0) != Some(&Val::Bool(true)) {
        return bad("model", "of a provider not permitted");
    }
    let effort = text(eff, &key("effort")).unwrap_or_default();
    if !list(eff, &format!("models.{model}.efforts")).iter().any(|e| e == effort) {
        return bad("effort", "not one of the model's efforts");
    }
    for t in std::iter::once(tool).chain(list(eff, &key("requiredTools")).iter().map(String::as_str)) {
        if !detection.present(t) {
            return Err(refuse(Code::ToolMissing, None, format!("{t}: not present on this machine")));
        }
    }
    Ok(())
}
