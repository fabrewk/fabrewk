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

/// The text outside fenced blocks, with every line of a block, and every line holding only white space, blank. A fence
/// opens with three or more backticks or tildes indented at most three spaces (a backtick fence's info holding no
/// backtick) and closes with a line of the same character, as many or more, and nothing else.
fn prose(text: &str) -> String {
    let mut fence: Option<(u8, usize)> = None;
    let mut out = String::new();
    for line in text.lines() {
        let t = line.trim_start_matches(' ');
        let (c, n) = (t.bytes().next().unwrap_or(0), t.bytes().take_while(|b| Some(*b) == t.bytes().next()).count());
        let marker = line.len() - t.len() <= 3 && matches!(c, b'`' | b'~') && n >= 3;
        match fence {
            Some((f, m)) if marker && c == f && n >= m && t[n..].trim().is_empty() => fence = None,
            None if marker && (c == b'~' || !t[n..].contains('`')) => fence = Some((c, n)),
            None if !line.trim().is_empty() => out += line,
            _ => {}
        }
        out.push('\n');
    }
    out
}

/// The destinations of the inline links and images of a Markdown text, `[text](destination "title")`, outside fenced
/// blocks and code spans (which may cross lines, not a blank one); a link's text stays on one line, and a backslash
/// escapes the punctuation after it (P7D2-3, P7D2-10).
fn targets(text: &str) -> Vec<String> {
    let text = prose(text);
    let b = text.as_bytes();
    let run = |i: usize| b[i..].iter().take_while(|c| **c == b'`').count();
    let (mut out, mut i, mut depth) = (Vec::new(), 0, 0usize);
    while i < b.len() {
        match b[i] {
            b'\\' if b.get(i + 1).is_some_and(u8::is_ascii_punctuation) => i += 1,
            b'`' => {
                let n = run(i);
                let paragraph = text[i..].find("\n\n").map_or(b.len(), |p| i + p);
                let end = (i + n..paragraph).find(|&j| run(j) == n && b[j - 1] != b'`');
                i = end.map_or(i + n, |j| j + n) - 1;
            }
            b'\n' => depth = 0,
            b'[' => depth += 1,
            b']' if depth > 0 => {
                depth -= 1;
                if let Some((dest, used)) = (b.get(i + 1) == Some(&b'(')).then(|| tail(&b[i + 2..])).flatten() {
                    out.push(dest);
                    i += 1 + used;
                }
            }
            _ => {}
        }
        i += 1;
    }
    out
}

/// A link's tail after its `(`: the destination, unescaped, and how many bytes the tail takes up to its `)`; `None`
/// when it is not one. A destination in `<…>` may hold spaces; one without may hold balanced parentheses. A title is
/// quoted with `"`, `'` or parentheses, and is never a destination (P7D2-11).
fn tail(b: &[u8]) -> Option<(String, usize)> {
    let space = |mut i: usize| {
        while b.get(i).is_some_and(|c| matches!(c, b' ' | b'\t' | b'\n')) {
            i += 1;
        }
        i
    };
    let (mut dest, mut i, mut depth) = (Vec::new(), space(0), 0usize);
    let angle = b.get(i) == Some(&b'<');
    i += usize::from(angle);
    while let Some(&c) = b.get(i) {
        if c == b'\\' && b.get(i + 1).is_some_and(u8::is_ascii_punctuation) {
            dest.push(b[i + 1]);
            i += 2;
            continue;
        }
        match c {
            b'>' if angle => break,
            b'<' | b'\n' if angle => return None,
            b' ' | b'\t' | b'\n' if !angle => break,
            b'(' if !angle => depth += 1,
            b')' if !angle && depth == 0 => break,
            b')' if !angle => depth -= 1,
            _ => {}
        }
        dest.push(c);
        i += 1;
    }
    if angle && b.get(i) != Some(&b'>') || !angle && depth > 0 {
        return None;
    }
    let after = i + usize::from(angle);
    let mut j = space(after);
    if j > after && b.get(j) != Some(&b')') {
        let close = match b.get(j)? {
            b'"' => b'"',
            b'\'' => b'\'',
            b'(' => b')',
            _ => return None,
        };
        j += 1;
        while *b.get(j)? != close {
            if b[j] == b'(' && close == b')' {
                return None;
            }
            j += if b[j] == b'\\' { 2 } else { 1 };
        }
        j = space(j + 1);
    }
    (b.get(j) == Some(&b')')).then(|| (String::from_utf8_lossy(&dest).into_owned(), j + 1))
}

/// A destination as a relative path, without its fragment or query; `None` for a scheme, an absolute path or only a
/// fragment.
fn link(dest: &str) -> Option<&str> {
    let path = dest.split(['#', '?']).next()?;
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
        let dir = entry.file_type().is_ok_and(|t| t.is_dir());
        if entry.file_name().to_str().is_some_and(is_alias) && dir && std::fs::symlink_metadata(&file).is_ok() {
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
