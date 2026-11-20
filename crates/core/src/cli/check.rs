//! `fabrewk check` (section 6; part 7d2, D-055): every verification of this phase, each finding with its code, its
//! severity and its control's label; it exits 3 when any finding is an error. It only reads. A verification that
//! cannot run, for a tool missing or failing or a file that does not read, is `CHECK_UNKNOWN`, never a pass. C5 to C7
//! and C12 come with phase 8.

use super::{Cx, FOUND, Failed, Out};
use crate::code::{Code::*, Error, normal, refuse};
use crate::record::{self, Publication, Repo};
use crate::settings::{self, Level, RESERVED, is_alias};
use crate::table::Control::{self, Cooperative, Effective};
use crate::{alambique, domain, fermento, identity};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

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
    Check { id: "C2", codes: &[PolicyAbsent, PolicyPending, InstallationIncomplete], control: Effective },
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
    Check { id: "C5", codes: &[ViewStale], control: Effective },
    Check { id: "C6", codes: &[SeparatorMissing, SeparatorAmbiguous, SeparatorDamaged], control: Effective },
    Check { id: "C7", codes: &[AlambiqueUnapproved], control: Effective },
    Check {
        id: "C8",
        codes: &[DomainUnknownEntry, AliasInvalid, GitignoreChanged, DomainNotIgnored],
        control: Effective,
    },
    Check { id: "C9", codes: &[CredentialSuspected], control: Cooperative },
    Check { id: "C10", codes: &[RecordUncommitted], control: Effective },
    Check { id: "C11", codes: &[PrivateFolderPermissions], control: Effective },
    Check { id: "C12", codes: &[MaintainerUnlisted, FermentoMissing], control: Effective },
];

/// A control's name in the catalogue and in `--json`.
pub fn label(c: Control) -> &'static str {
    match c {
        Effective => "effective",
        Cooperative => "cooperative",
    }
}

/// A failure of the verification itself: a tool or a file it needed, or a repository it cannot walk.
fn unknown(e: Error) -> Error {
    match e {
        Error::Refused { code: ToolMissing | ToolFailed | HeadDetached | NotARepository, path, detail } => {
            let detail = format!("{detail}; so this verification could not run");
            Error::Refused { code: CheckUnknown, path, detail }
        }
        Error::Io { path, .. } => refuse(CheckUnknown, Some(&path), "cannot be read"),
        other => other,
    }
}

/// A check's label where the mode is `mode`: C4 is cooperative on one machine (section 6).
pub(crate) fn control_of(c: &Check, mode: Option<Publication>) -> Control {
    if c.id == "C4" && mode == Some(Publication::Unpublished) { Cooperative } else { c.control }
}

/// What the fermentos and the alambique cite (D-055 point 2): the relative targets of the Markdown links and images
/// of each citing file that is a regular file in a directory that is not a link, resolved from its directory.
pub(crate) fn cited(domain: &Path) -> BTreeSet<PathBuf> {
    let regular = |p: &Path| std::fs::symlink_metadata(p).is_ok_and(|m| m.is_file());
    let mut citing = vec![domain.join("ALAMBIQUE.md")];
    for entry in std::fs::read_dir(domain).into_iter().flatten().flatten() {
        if entry.file_name().to_str().is_some_and(is_alias) && entry.file_type().is_ok_and(|t| t.is_dir()) {
            citing.push(entry.path().join("fermento.md"));
        }
    }
    let mut out = BTreeSet::new();
    for file in citing.into_iter().filter(|f| regular(f)) {
        // A citing file that is not UTF-8 is read as far as it is; one that does not read is C9's `CHECK_UNKNOWN`.
        let (Ok(bytes), Some(dir)) = (std::fs::read(&file), file.parent()) else { continue };
        for target in targets(&String::from_utf8_lossy(&bytes)).iter().filter_map(|t| link(t)) {
            out.insert(normal(&dir.join(target)));
        }
    }
    out
}

/// The destinations of every text shaped as a Markdown link, read lexically (D-056): a `[`, then on the same line
/// `](`, then a destination, wherever it stands. A destination is `<…>` up to its `>` on the same line, or every
/// character up to the first white space or the `)` that closes it, parentheses balanced; a backslash escapes the
/// punctuation after it.
fn targets(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let Some(open) = line.find('[') else { continue };
        for (at, _) in line.match_indices("](").filter(|(at, _)| *at > open) {
            let (mut dest, mut depth) = (String::new(), 0usize);
            let mut chars = line[at + 2..].chars().peekable();
            let angle = chars.next_if_eq(&'<').is_some();
            let mut closed = !angle;
            while let Some(c) = chars.next() {
                if c == '\\' && chars.peek().is_some_and(char::is_ascii_punctuation) {
                    dest.extend(chars.next());
                    continue;
                }
                match c {
                    '>' if angle => {
                        closed = true;
                        break;
                    }
                    c if c.is_whitespace() && !angle => break,
                    // Only a destination without `<…>` counts its parentheses.
                    '(' => depth += 1,
                    ')' if !angle && depth == 0 => break,
                    ')' if !angle => depth -= 1,
                    _ => {}
                }
                dest.push(c);
            }
            if closed {
                out.push(dest);
            }
        }
    }
    out
}

/// A destination as a relative path, without its fragment or query; `None` for a scheme or an absolute path. An
/// empty one (only a fragment) names the citing file's own directory, which holds no file to scan.
fn link(dest: &str) -> Option<&str> {
    let path = dest.split(['#', '?']).next()?;
    let scheme = path.split_once(':').is_some_and(|(s, _)| !s.is_empty() && !s.contains('/'));
    (!path.starts_with('/') && !scheme).then_some(path)
}

pub(super) fn run(cx: &Cx) -> Result<Out, Failed> {
    let mut found: Vec<(&str, Error)> = Vec::new();
    let d = &cx.domain;
    match std::fs::symlink_metadata(d) {
        Ok(m) if m.is_dir() => {}
        Ok(_) => {
            found.push(("C3", refuse(RecordBroken, Some(d), "the domain is not a directory; a link is not followed")));
            return Ok(render(cx, found, None));
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            found.push(("C4", refuse(RecordAbsent, Some(d), "there is no domain here")));
            return Ok(render(cx, found, None));
        }
        // A domain that cannot be examined is not an absent one (P7D2-27).
        Err(_) => {
            found.push(("C4", refuse(CheckUnknown, Some(d), "cannot be examined")));
            return Ok(render(cx, found, None));
        }
    }
    // C1: the project file, each maintainer's file, the machine's.
    let mut add = |id, r: Result<(), Vec<Error>>| found.extend(r.err().into_iter().flatten().map(|e| (id, unknown(e))));
    add("C1", settings::read(&d.join("settings.toml"), Level::Project).map(drop));
    // Every maintainer's file, each in a directory that is not a link; what cannot be listed is unknown (P7D2-21).
    let unreadable = |p: &Path| vec![refuse(CheckUnknown, Some(p), "cannot be listed or examined")];
    match std::fs::read_dir(d) {
        Err(_) => add("C1", Err(unreadable(d))),
        Ok(entries) => {
            for entry in entries {
                let Ok(entry) = entry else {
                    add("C1", Err(unreadable(d)));
                    break;
                };
                let (file, dir) = (entry.path().join("settings.toml"), entry.file_type().is_ok_and(|t| t.is_dir()));
                if !dir || !entry.file_name().to_str().is_some_and(is_alias) {
                    continue;
                }
                match std::fs::symlink_metadata(&file) {
                    Ok(_) => add("C1", settings::read(&file, Level::Personal).map(drop)),
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                    Err(_) => add("C1", Err(unreadable(&file))),
                }
            }
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
        (Ok(_), Some(p)) => match settings::pending(p, d) {
            Ok(Some(changes)) => {
                let places: Vec<String> = changes.iter().map(|c| c.place()).collect();
                found.push(("C2", refuse(PolicyPending, Some(&d.join("settings.toml")), places.join(", "))));
            }
            Ok(None) => {}
            Err(_) => found.push(("C2", refuse(CheckUnknown, Some(&d.join("settings.toml")), "it does not read"))),
        },
        (Ok(_), None) => found.push(("C2", refuse(PolicyAbsent, Some(d), "no policy is adopted"))),
        (Err(_), _) if absent => found.push(("C2", refuse(PolicyAbsent, Some(d), "no policy is adopted"))),
        (Err(_), _) => found.push(("C2", refuse(CheckUnknown, Some(d), "the record does not replay"))),
    }
    // An `apply` not complete (D-097 point 7).
    if let Ok(r) = record {
        found.extend(super::write::installation(d, r).err().map(|e| ("C2", unknown(e))));
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
    // With no mode, what the mode decides is unknown; a record that does not replay has its C4 unknown already.
    if mode.is_none() && !absent {
        let (ids, why): (&[&str], _) = match record {
            Ok(_) => (&["C4", "C8", "C10"], "no publication mode is recorded"),
            Err(_) => (&["C8", "C10"], "the record does not replay, so its publication mode is unknown"),
        };
        found.extend(ids.iter().map(|id| (*id, refuse(CheckUnknown, Some(d), why))));
    }
    if let Some(repo) = &repo {
        let walk = record::record_branch(repo).and_then(|b| record::walk_history(repo, &b));
        found.extend(walk.err().map(|e| ("C4", unknown(e))));
    }
    // C8: the layout, the ignore file, and the product's ignore rule in the modes that do not publish the domain.
    match domain::layout(d, mode) {
        Ok(entries) => found.extend(entries.into_iter().map(|e| ("C8", e))),
        Err(e) => found.push(("C8", unknown(e))),
    }
    found.extend(domain::check_gitignore(d).err().map(|e| ("C8", unknown(e))));
    if let Some(mode) = mode {
        found.extend(domain::check_ignored(d, mode).err().map(|e| ("C8", unknown(e))));
    }
    // C7: the alambique is the text a maestro last approved (REQ-FER-7).
    match record {
        Ok(r) => found.extend(alambique::check(r, d).err().map(|e| ("C7", unknown(e)))),
        Err(_) if !absent => found.push(("C7", refuse(CheckUnknown, Some(d), "the record does not replay"))),
        Err(_) => {}
    }
    // C5: the views are what a fresh generation gives (REQ-VIEW-3).
    match record {
        Ok(r) => found.extend(crate::view::stale(d, r).into_iter().map(|e| ("C5", unknown(e)))),
        Err(_) if !absent => found.push(("C5", refuse(CheckUnknown, Some(d), "the record does not replay"))),
        Err(_) => {}
    }
    // C6 and C12: each maintainer's directory is listed in the policy and holds a fermento, whose separator stands
    // once (REQ-FER-1, REQ-FER-9). A directory that cannot be listed is C1's `CHECK_UNKNOWN`.
    let listed: Option<BTreeSet<String>> =
        cx.policy.as_ref().map(|p| identity::maintainers(&p.settings).into_iter().map(|m| m.alias).collect());
    let dirs = std::fs::read_dir(d).into_iter().flatten().flatten();
    let mut aliases: Vec<String> = dirs
        .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| is_alias(n) && !RESERVED.contains(&n.as_str()))
        .collect();
    aliases.sort();
    for alias in aliases {
        if listed.as_ref().is_some_and(|l| !l.contains(&alias)) {
            found.push((
                "C12",
                refuse(MaintainerUnlisted, Some(&d.join(&alias)), "the policy in force does not list it"),
            ));
        }
        match fermento::open(d, &alias) {
            Ok(_) => {}
            Err(e) if e.code() == Some(FermentoMissing) => found.push(("C12", e)),
            Err(e) => found.push(("C6", unknown(e))),
        }
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
        Ok(f) => match std::fs::symlink_metadata(&f) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            _ => found.extend(crate::machine::check_permissions(&f).err().map(|e| ("C11", unknown(e)))),
        },
        Err(e) => found.push(("C11", unknown(e))),
    }
    Ok(render(cx, found, mode))
}

fn render(cx: &Cx, found: Vec<(&str, Error)>, mode: Option<Publication>) -> Out {
    let mut entries = Vec::new();
    let mut text = String::new();
    for (id, e) in &found {
        let warning = matches!(e.code(), Some(DomainUnknownEntry | PolicyPending));
        let check = CHECKS.iter().find(|c| c.id == *id).expect("a finding names a declared check");
        let control = control_of(check, mode);
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
    let checks: Vec<Value> =
        CHECKS.iter().map(|c| json!({"check": c.id, "control": label(control_of(c, mode))})).collect();
    let doc = json!({"schema": "fabrewk.check/1", "checks": checks, "findings": entries, "errors": errors, "warnings": warnings});
    Out { doc, text, error: false, exit: if errors > 0 { FOUND } else { super::DONE } }
}
