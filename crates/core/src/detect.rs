//! What is present on the machine (REQ-SET-8, REQ-SET-9; AD-2, "Present, apart from permitted"; part 7c2b, D-051).
//! A command that needs it detects at its start, in memory: the core's own programs and each built-in module's, by
//! executable; and apart from them, by name, every catalogued tool, every profile's required tools and every name
//! `machine.toml` declares; each found in `PATH` or where `machine.toml` declares it, with its version. Write commands
//! keep the result in `local/cache/detected.json`, which carries one thing forward: when a use verified a program at
//! that path and version. Presence is never read from it. Naming a tool grants nothing: the effective set is what is
//! both permitted and present.

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
    /// Whether the tool reports the tokens it used, so that a token limit can be observed (REQ-RUN-7).
    pub tokens: bool,
    /// The folders, relative to the user's home, from which the tool loads skills that are not the project's; a sweep
    /// lists their names (REQ-FER-10, D-072).
    pub skills: &'static [&'static str],
}

/// The core's own programs (AD-4), found in `PATH` only, as the core runs them. `ssh-keygen` prints no version.
pub const CORE: [Program; 2] = [
    Program { adapter: None, executable: "git", version: &["--version"], tokens: false, skills: &[] },
    Program { adapter: None, executable: "ssh-keygen", version: &[], tokens: false, skills: &[] },
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

/// Where a name is looked for: the core's programs and each module's own, by executable; the catalogued tools and the
/// names `machine.toml` declares; and the profiles' required tools, each by its own name. The three are apart, so that
/// a name in one is never found as the same name in another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Space {
    Program,
    Tool,
    Required,
}

impl Space {
    const ALL: [Space; 3] = [Space::Program, Space::Tool, Space::Required];

    /// The space's key in `local/cache/detected.json`.
    pub const fn key(self) -> &'static str {
        match self {
            Space::Program => "programs",
            Space::Tool => "tools",
            Space::Required => "required",
        }
    }
}

/// One detection: when it ran, and every name looked for in each space, `None` when absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Detection {
    pub at: String,
    pub found: BTreeMap<(Space, String), Option<Found>>,
}

/// What to look for under one name: its executable (none when no module drives a catalogued tool's adapter), its
/// version's arguments, and whether `machine.toml` may say where it is.
struct Want<'a> {
    executable: Option<&'a str>,
    version: &'a [&'a str],
    declarable: bool,
}

pub(crate) fn text<'a>(eff: &'a Effective, key: &str) -> Option<&'a str> {
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

/// Whether this process may execute `path`: a file with an execute bit, which `sh`'s `test -x` accepts (D-052), so
/// the operating system answers for the user, its groups, access lists and mounts.
fn is_executable(path: &Path) -> bool {
    std::fs::metadata(path).is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        && Command::new("/bin/sh")
            .args(["-c", "test -x \"$1\"", "sh"])
            .arg(path)
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|s| s.success())
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
    let mut wanted: BTreeMap<(Space, &str), Want> = BTreeMap::new();
    let own = CORE.iter().map(|p| (p, false)).chain(modules.iter().filter(|p| p.adapter.is_none()).map(|p| (p, true)));
    for (p, declarable) in own {
        let want = Want { executable: Some(p.executable), version: p.version, declarable };
        wanted.entry((Space::Program, p.executable)).or_insert(want);
    }
    for name in names(eff, "tools", "adapter") {
        let adapter = text(eff, &format!("tools.{name}.adapter"));
        let module = modules.iter().find(|p| p.adapter.is_some() && p.adapter == adapter);
        let version = module.map_or(&[][..], |p| p.version);
        wanted
            .insert((Space::Tool, name), Want { executable: module.map(|p| p.executable), version, declarable: true });
    }
    for name in names(eff, "tools", "path") {
        wanted.entry((Space::Tool, name)).or_insert(Want { executable: None, version: &[], declarable: true });
    }
    let required =
        names(eff, "profiles", "requiredTools").flat_map(|p| list(eff, &format!("profiles.{p}.requiredTools")));
    for name in required.map(String::as_str) {
        wanted.insert((Space::Required, name), Want { executable: Some(name), version: &[], declarable: true });
    }
    let regular = crate::code::unlinked(domain, "local/cache/detected.json")
        && std::fs::symlink_metadata(cache(domain)).is_ok_and(|m| m.is_file());
    let marks = regular.then(|| std::fs::read(cache(domain)).ok()).flatten().and_then(|b| decode(&b));
    let mut found = BTreeMap::new();
    for ((space, name), want) in wanted {
        let declared = if want.declarable { text(eff, &format!("tools.{name}.path")) } else { None };
        let (at, how) = match declared {
            Some(p) => (Some(PathBuf::from(p)).filter(|p| is_executable(p)), How::Declared),
            None => (want.executable.and_then(|e| search(path, e)), How::Observed),
        };
        let at = at.map(|path| {
            let version = version(&path, want.version);
            // A path that is not UTF-8 is kept as none, so no verification is carried to it.
            let mark = marks.as_ref().and_then(|m| m.get(space.key())?.get(name)?.as_object());
            let same = mark.is_some_and(|m| {
                path.to_str().is_some_and(|p| m.get("path") == Some(&json!(p)))
                    && m.get("version") == Some(&json!(version))
            });
            let verified = mark.filter(|_| same).and_then(|m| Some(m.get("verified")?.as_str()?.to_owned()));
            Found { path, version, how, verified }
        });
        found.insert((space, name.to_owned()), at);
    }
    Detection { at: now_utc(), found }
}

impl Detection {
    /// Whether `name` was found in `space`.
    pub fn present(&self, space: Space, name: &str) -> bool {
        self.get(space, name).is_some()
    }

    /// What was found for `name` in `space`; `None` when absent or not looked for.
    pub fn get(&self, space: Space, name: &str) -> Option<&Found> {
        self.found.get(&(space, name.to_owned()))?.as_ref()
    }

    /// Records that a use of `name` in `space` succeeded now, at the path and version detected; a write command keeps
    /// it.
    pub fn used(&mut self, space: Space, name: &str) {
        if let Some(Some(found)) = self.found.get_mut(&(space, name.to_owned())) {
            found.verified = Some(now_utc());
        }
    }

    /// Replaces `local/cache/detected.json` under `domain` with this detection, dated; only write commands call it
    /// (REQ-REG-12, REQ-DOM-5). It is a cache: deleting it loses only when each program was verified.
    pub fn keep(&self, domain: &Path) -> Result<(), Error> {
        let entry = |f: &Found| {
            let how = if f.how == How::Declared { "declared" } else { "observed" };
            json!({"how": how, "path": f.path.to_str(), "verified": f.verified, "version": f.version})
        };
        let mut kept = json!({"at": self.at, "schema": 1});
        for space in Space::ALL {
            let map: Map<String, Value> = self
                .found
                .iter()
                .filter(|((s, _), _)| *s == space)
                .map(|((_, n), f)| (n.clone(), f.as_ref().map_or(Value::Null, entry)))
                .collect();
            kept[space.key()] = Value::Object(map);
        }
        let bytes = encode(&kept).expect("three levels deep");
        let dir = domain.join("local/cache");
        std::fs::create_dir_all(&dir).map_err(io_at(&dir))?;
        write_atomic(&cache(domain), &bytes)
    }
}

/// The effective tools (REQ-SET-8): catalogued in `eff` and present in `detection`. A permitted tool that is absent,
/// and a present one the catalogue does not hold, are both outside it.
pub fn effective_tools(eff: &Effective, detection: &Detection) -> BTreeSet<String> {
    names(eff, "tools", "adapter").filter(|n| detection.present(Space::Tool, n)).map(str::to_owned).collect()
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
    let required = list(eff, &key("requiredTools")).iter().map(|t| (Space::Required, t.as_str()));
    for (space, t) in std::iter::once((Space::Tool, tool)).chain(required) {
        if !detection.present(space, t) {
            return Err(refuse(Code::ToolMissing, None, format!("{t}: not present on this machine")));
        }
    }
    Ok(())
}
