//! `fabrewk check` (section 6; part 7d2, D-055): every verification of this phase, each finding with its code, its
//! severity and its control's label; it exits 3 when any finding is an error. It only reads. A verification that
//! cannot run, for a tool missing or failing or a file that does not read, is `CHECK_UNKNOWN`, never a pass. C5, C6,
//! C7 and C12 are declared with phase 8.

use super::{Cx, FOUND, Failed, Out};
use crate::code::{Code::*, Error, refuse};
use crate::domain;
use crate::record::{self, Publication, Repo};
use crate::settings::{self, Level, is_alias};
use crate::table::Control::{self, Cooperative, Effective};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

/// One verification: its number in section 6, the codes it finds and its control's label (REQ-SEC-5).
#[derive(Debug)]
pub struct Check {
    pub id: &'static str,
    pub codes: &'static [crate::Code],
    pub control: Control,
}

/// The verifications of this phase, in order. C4 is effective where the history is walked and cooperative on one
/// machine (`unpublished`), as its finding says.
pub const CHECKS: &[Check] = &[
    Check { id: "C1", codes: &[SettingsInvalid, SettingsKeyNotPersonal, AliasMismatch], control: Effective },
    Check { id: "C2", codes: &[PolicyAbsent, PolicyPending], control: Effective },
    Check {
        id: "C3",
        codes: &[
            RecordVersionUnknown,
            RecordBroken,
            RecordDigestMismatch,
            RecordNoncanonical,
            RecordFork,
            RecordTransitionInvalid,
            AttemptEnded,
            RecordSignatureInvalid,
            RecordIdCollision,
        ],
        control: Effective,
    },
    Check { id: "C4", codes: &[RecordAbsent, RecordRollback, MarkInvalid], control: Effective },
    Check {
        id: "C8",
        codes: &[DomainUnknownEntry, AliasInvalid, GitignoreChanged, DomainNotIgnored],
        control: Effective,
    },
    Check { id: "C9", codes: &[CredentialSuspected], control: Cooperative },
    Check { id: "C10", codes: &[RecordUncommitted], control: Effective },
    Check { id: "C11", codes: &[PrivateFolderPermissions], control: Effective },
];

/// A control's name in the catalogue and in `--json`.
pub fn label(c: Control) -> &'static str {
    match c {
        Effective => "effective",
        Cooperative => "cooperative",
    }
}

/// A failure of the verification itself: a tool or a file it needed.
fn unknown(e: Error) -> Error {
    match e {
        Error::Refused { code: ToolMissing | ToolFailed, path, detail } => {
            Error::Refused { code: CheckUnknown, path, detail: format!("{detail}: the tool is missing or failed") }
        }
        Error::Io { path, .. } => refuse(CheckUnknown, Some(&path), "cannot be read"),
        other => other,
    }
}

/// What the fermentos and the alambique cite (D-055 point 2): the relative targets of their Markdown links and images,
/// resolved from the citing file's directory.
pub(crate) fn cited(domain: &Path) -> BTreeSet<PathBuf> {
    let mut citing = vec![domain.join("ALAMBIQUE.md")];
    for entry in std::fs::read_dir(domain).into_iter().flatten().flatten() {
        if entry.file_name().to_str().is_some_and(is_alias) {
            citing.push(entry.path().join("fermento.md"));
        }
    }
    let mut out = BTreeSet::new();
    for file in citing {
        let (Ok(text), Some(dir)) = (std::fs::read_to_string(&file), file.parent()) else { continue };
        for target in text.split("](").skip(1).filter_map(|rest| link(rest.split(')').next()?)) {
            out.insert(normal(&dir.join(target)));
        }
    }
    out
}

/// A link's target as a relative path, without its title, fragment or query; `None` for a scheme, an absolute path or
/// only a fragment.
fn link(raw: &str) -> Option<&str> {
    let raw = raw.trim();
    let raw = match raw.strip_prefix('<') {
        Some(inner) => inner.split('>').next()?,
        None => raw.split_whitespace().next()?,
    };
    let path = raw.split(['#', '?']).next()?;
    let scheme = path.split_once(':').is_some_and(|(s, _)| !s.is_empty() && !s.contains('/'));
    (!path.is_empty() && !path.starts_with('/') && !scheme).then_some(path)
}

/// `path` with `.` dropped and each `..` taken against the part before it, read as written.
fn normal(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in path.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    out
}

pub(super) fn run(cx: &Cx) -> Result<Out, Failed> {
    let mut found: Vec<(&str, Error)> = Vec::new();
    let d = &cx.domain;
    if !std::fs::symlink_metadata(d).is_ok_and(|m| m.is_dir()) {
        found.push(("C4", refuse(RecordAbsent, Some(d), "there is no domain here")));
        return Ok(render(cx, found, None));
    }
    // C1: the project file, each maintainer's file, the machine's.
    let mut add = |id, r: Result<(), Vec<Error>>| found.extend(r.err().into_iter().flatten().map(|e| (id, unknown(e))));
    add("C1", settings::read(&d.join("settings.toml"), Level::Project).map(drop));
    for entry in std::fs::read_dir(d).into_iter().flatten().flatten() {
        let file = entry.path().join("settings.toml");
        if entry.file_name().to_str().is_some_and(is_alias) && file.exists() {
            add("C1", settings::read(&file, Level::Personal).map(drop));
        }
    }
    let folder = cx.folder();
    if let Ok(folder) = &folder {
        let machine = crate::machine::machine_file(folder).map(drop);
        add("C1", machine.map_err(|e| e.into_iter().filter(|e| e.code() != Some(PrivateFolderPermissions)).collect()));
    }
    // C2 to C4: the policy and the record.
    let record = cx.record.as_ref();
    let absent = record.as_ref().err().is_some_and(|e| e.iter().all(|e| e.code() == Some(RecordAbsent)));
    match (record, &cx.policy) {
        (Ok(_), Some(p)) => {
            if let Ok(Some(changes)) = settings::pending(p, d) {
                let places: Vec<String> = changes.iter().map(|c| c.place()).collect();
                found.push(("C2", refuse(PolicyPending, Some(&d.join("settings.toml")), places.join(", "))));
            }
        }
        (Ok(_), None) => found.push(("C2", refuse(PolicyAbsent, Some(d), "no policy is adopted"))),
        (Err(_), _) if absent => found.push(("C2", refuse(PolicyAbsent, Some(d), "no policy is adopted"))),
        (Err(_), _) => found.push(("C2", refuse(CheckUnknown, Some(d), "the record does not replay"))),
    }
    match record {
        Err(errors) if absent => found.extend(errors.iter().map(|e| ("C4", refuse(RecordAbsent, Some(d), e.detail())))),
        Err(errors) => {
            found.extend(errors.iter().map(|e| ("C3", unknown(e.clone()))));
            found.push(("C4", refuse(CheckUnknown, Some(d), "the mark is compared with a record that replays")));
        }
        Ok(r) => found.extend(record::check_mark(d, r).err().map(|e| ("C4", unknown(e)))),
    }
    let mode = record.ok().and_then(domain::publication);
    let repo = mode.and_then(|m| Repo::for_domain(d, m));
    if record.is_ok() && mode.is_none() {
        for id in ["C4", "C8", "C10"] {
            found.push((id, refuse(CheckUnknown, Some(d), "no publication mode is recorded")));
        }
    }
    if let Some(repo) = &repo {
        let walk = record::record_branch(repo).and_then(|b| record::walk_history(repo, &b));
        found.extend(walk.err().map(|e| ("C4", unknown(e))));
    }
    // C8: the layout, the ignore file, and the product's ignore rule in the modes that do not publish the domain.
    match domain::layout(d) {
        Ok(entries) => found.extend(entries.into_iter().map(|e| ("C8", e))),
        Err(e) => found.push(("C8", unknown(e))),
    }
    found.extend(domain::check_gitignore(d).err().map(|e| ("C8", unknown(e))));
    if let Some(mode) = mode {
        found.extend(domain::check_ignored(d, mode).err().map(|e| ("C8", unknown(e))));
    }
    // C9: what looks like a credential, reading only what is cited in a maintainer's directory.
    let cited = cited(d);
    found.extend(crate::scan::domain(d, &|p: &Path| cited.contains(&normal(p))).into_iter().map(|e| ("C9", e)));
    // C10: the record's files committed where the mode commits.
    if let Some(repo) = &repo {
        match record::uncommitted(repo) {
            Ok(files) if !files.is_empty() => {
                let why = format!("{} event or signature files not in the record branch's tip", files.len());
                found.push(("C10", refuse(RecordUncommitted, Some(&files[0]), why)));
            }
            Ok(_) => {}
            Err(e) => found.push(("C10", unknown(e))),
        }
    }
    // C11: the private folder.
    match folder {
        Ok(f) if f.exists() => found.extend(crate::machine::check_permissions(&f).err().map(|e| ("C11", unknown(e)))),
        Ok(_) => {}
        Err(e) => found.push(("C11", unknown(e))),
    }
    Ok(render(cx, found, mode))
}

fn render(cx: &Cx, found: Vec<(&str, Error)>, mode: Option<Publication>) -> Out {
    let mut entries = Vec::new();
    let mut text = String::new();
    for (id, e) in &found {
        let warning = matches!(e.code(), Some(DomainUnknownEntry | PolicyPending));
        let one_machine = *id == "C4" && mode == Some(Publication::Unpublished);
        let check = CHECKS.iter().find(|c| c.id == *id).expect("a finding names a declared check");
        let control = if one_machine || e.code() == Some(CredentialSuspected) { Cooperative } else { check.control };
        let mut v = cx.entry(e);
        v["check"] = json!(id);
        v["severity"] = json!(if warning { "warning" } else { "error" });
        v["control"] = json!(label(control));
        let head = cx.t(
            "check.finding",
            &[
                ("check", id),
                ("about", &cx.t(&format!("check.{id}"), &[])),
                ("severity", &cx.t(&format!("severity.{}", v["severity"].as_str().unwrap_or_default()), &[])),
                ("control", &cx.t(&format!("control.{}", label(control)), &[])),
            ],
        );
        text += &format!("{head}\n{}", cx.entry_text(&v, "  "));
        entries.push(v);
    }
    let errors = entries.iter().filter(|v| v["severity"] == "error").count();
    let warnings = entries.len() - errors;
    let (e, w) = (errors.to_string(), warnings.to_string());
    text += &cx.t(if errors > 0 { "check.failed" } else { "check.passed" }, &[("errors", &e), ("warnings", &w)]);
    text.push('\n');
    let checks: Vec<Value> = CHECKS.iter().map(|c| json!({"check": c.id, "control": label(c.control)})).collect();
    let doc = json!({"schema": "fabrewk.check/1", "checks": checks, "findings": entries, "errors": errors, "warnings": warnings});
    Out { doc, text, error: false, exit: if errors > 0 { FOUND } else { super::DONE } }
}
