//! The alambique, the project rules and the formula (REQ-FER-5 to REQ-FER-7; part 8b, D-073; formula 1a): the units
//! an edit replaces whole, the rules the stream `rules` holds, the digest of the last approved text of the alambique and
//! of the formula, the difference a direct edit records, the formula's cuts, and the replay's checks of the three
//! streams. A fence is read as the fermento's finder reads it (D-072).

use crate::code::{Code, Error, io_at, refuse, unlinked};
use crate::fermento::{Proposal, fenced, heading, lines};
use crate::record::{Event, Replay, Repo, committed, sha256_hex, staged, subject_digest};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::Path;

/// A text a maestro approves: its file in the domain, the stream of its approvals, and the codes of a file missing, one
/// that is not the text an edit starts from, and one that is not the text last approved.
#[derive(Debug)]
pub struct Doc {
    pub file: &'static str,
    pub stream: &'static str,
    pub missing: Code,
    pub changed: Code,
    pub unapproved: Code,
}

/// The alambique (REQ-FER-5 to REQ-FER-7).
pub const ALAMBIQUE: Doc = Doc {
    file: "ALAMBIQUE.md",
    stream: "alambique",
    missing: Code::AlambiqueMissing,
    changed: Code::AlambiqueChanged,
    unapproved: Code::AlambiqueUnapproved,
};

/// The formula, the project's roadmap (formula 1a): a project has none until its integrator proposes one.
pub const FORMULA: Doc = Doc {
    file: "FORMULA.md",
    stream: "formula",
    missing: Code::FormulaMissing,
    changed: Code::FormulaChanged,
    unapproved: Code::FormulaUnapproved,
};

/// The project rules: `R<n>` and its text (REQ-FER-6).
pub type Rules = BTreeMap<String, String>;

fn ambiguous(why: impl Into<String>) -> Error {
    refuse(Code::UnitAmbiguous, None, why)
}

fn number(s: &str) -> Option<u64> {
    (!s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())).then(|| s.parse().ok())?
}

/// `**<n>.<m>.` at the start of a line: a point's number.
fn point(line: &str) -> Option<(u64, u64)> {
    let (n, rest) = line.strip_prefix("**")?.split_once('.')?;
    Some((number(n)?, number(rest.split_once('.')?.0)?))
}

/// `## <n>.` at the start of a line: a section's number.
fn section(line: &str) -> Option<u64> {
    number(line.strip_prefix("## ")?.split_once('.')?.0)
}

/// The bytes a unit spans, from its first line to the end of its last line that is not blank, without the break.
/// The opening runs from the start to section 1; a section to the next heading of level 1 or 2; a point to the next
/// point or heading. A unit that does not stand exactly once outside fences is `UNIT_AMBIGUOUS`.
fn span(text: &str, kind: &str, n: u64, m: u64) -> Result<(usize, usize), Error> {
    let all = lines(text);
    let t: Vec<&str> = all.iter().map(|l| l.1).collect();
    let fence = fenced(&t);
    let after = |from: usize, stop: &dyn Fn(&str) -> bool| (from..t.len()).find(|&j| !fence[j] && stop(t[j]));
    let once = |is: &dyn Fn(&str) -> bool, what: String| {
        let hits: Vec<usize> = (0..t.len()).filter(|&i| !fence[i] && is(t[i])).collect();
        match hits[..] {
            [i] => Ok(i),
            _ => Err(ambiguous(format!("{what} stands {} times outside fenced code blocks", hits.len()))),
        }
    };
    let (first, end) = match kind {
        "opening" => (0, once(&|l| section(l) == Some(1), "section 1".into())?),
        "section" => {
            let i = once(&|l| section(l) == Some(n), format!("section {n}"))?;
            (
                i,
                after(i + 1, &|l| {
                    heading(l)
                        && l.trim_start_matches(' ').starts_with('#')
                        && !l.trim_start_matches(' ').starts_with("###")
                })
                .unwrap_or(t.len()),
            )
        }
        _ => {
            let i = once(&|l| point(l) == Some((n, m)), format!("point {n}.{m}"))?;
            (i, after(i + 1, &|l| heading(l) || point(l).is_some()).unwrap_or(t.len()))
        }
    };
    if end == first {
        return Ok((all[first].0, all[first].0));
    }
    let last = (first..end).rev().find(|&j| !t[j].trim().is_empty()).unwrap_or(first);
    Ok((all[first].0, all[last].0 + t[last].len()))
}

/// `text` with each of `units` (an alambique proposal's) replaced whole, each found in `text` as it stands; two units
/// that overlap are `UNIT_AMBIGUOUS`.
pub fn apply(text: &str, units: &[Value]) -> Result<String, Error> {
    let mut spans = Vec::new();
    for u in units {
        let (n, m) = (u["n"].as_u64().unwrap_or(0), u["m"].as_u64().unwrap_or(0));
        let (start, end) = span(text, u["kind"].as_str().unwrap_or_default(), n, m)?;
        spans.push((start, end, u["text"].as_str().unwrap_or_default()));
    }
    spans.sort();
    if spans.windows(2).any(|w| w[0].1 > w[1].0) {
        return Err(ambiguous("two units overlap"));
    }
    let mut out = text.to_owned();
    for (start, end, new) in spans.into_iter().rev() {
        out.replace_range(start..end, new);
    }
    Ok(out)
}

/// The rules the record holds: those of the last `proposal-incorporated` of the stream `rules`, none before it.
pub fn rules(record: &Replay) -> Rules {
    let last = record.streams.get("rules").and_then(|s| s.iter().rev().find(|e| e.kind == "proposal-incorporated"));
    let held = last.and_then(|e| e.value["payload"]["rules"].as_object());
    held.into_iter().flatten().filter_map(|(k, v)| Some((k.clone(), v.as_str()?.to_owned()))).collect()
}

/// The digest of the rules: that of their canonical JSON, `{}` when there are none.
pub fn rules_digest(rules: &Rules) -> String {
    subject_digest(&json!(rules)).unwrap_or_default()
}

/// `base` with each of `units` (a rules proposal's) added, replaced or, with no text, removed; a rule removed that is
/// not there is `UNIT_AMBIGUOUS`.
pub fn apply_rules(base: &Rules, units: &[Value]) -> Result<Rules, Error> {
    let mut out = base.clone();
    for u in units {
        let key = format!("R{}", u["n"]);
        match u["text"].as_str() {
            Some(text) => _ = out.insert(key, text.to_owned()),
            None => _ = out.remove(&key).ok_or_else(|| ambiguous(format!("{key} is not a rule of the project")))?,
        }
    }
    Ok(out)
}

/// The digest of the text a maestro last approved: the alambique's baseline, an incorporated proposal's result, a
/// direct edit's new digest, or the formula's imported or approved digest, whichever is last; `None` before any. A cut
/// names no digest of its own.
pub fn approved(record: &Replay, doc: &Doc) -> Option<String> {
    let events = record.streams.get(doc.stream)?;
    let digest = |e: &Event| -> Option<String> {
        ["new", "result", "digest"].iter().find_map(|f| Some(e.value["payload"][f].as_str()?.to_owned()))
    };
    events.iter().rev().find_map(digest)
}

/// The file's bytes, `None` when there is no file; a file that is not regular, or is reached through a link, is
/// refused with the document's `missing` code.
pub fn read(domain: &Path, doc: &Doc) -> Result<Option<Vec<u8>>, Error> {
    let path = domain.join(doc.file);
    match std::fs::symlink_metadata(&path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(io_at(&path)(e)),
        Ok(m) if !m.is_file() || !unlinked(domain, doc.file) => {
            return Err(refuse(doc.missing, Some(&path), "not a regular file, or reached through a link"));
        }
        Ok(_) => {}
    }
    std::fs::read(&path).map(Some).map_err(io_at(&path))
}

/// The file's bytes once it is the text that digests to `before`: the file must be, and with `strict` and a `repo` (a
/// mode that commits) so must the record branch's tip and the person's index, which an edit starts from and a commit
/// of the file alone would otherwise overwrite. Anything else is the document's `changed` code.
pub fn unchanged(domain: &Path, repo: Option<&Repo>, before: &str, strict: bool, doc: &Doc) -> Result<Vec<u8>, Error> {
    let changed = |why: &str| refuse(doc.changed, Some(&domain.join(doc.file)), why.to_owned());
    let bytes = match read(domain, doc) {
        Ok(Some(bytes)) => bytes,
        Err(Error::Refused { code, .. }) if code == doc.missing => {
            return Err(changed("there is no file to start from"));
        }
        Ok(None) => return Err(changed("there is no file to start from")),
        Err(e) => return Err(e),
    };
    if sha256_hex(&bytes) != before {
        return Err(changed("the file is not the text the command starts from"));
    }
    if let Some(repo) = repo.filter(|_| strict) {
        let held = [committed(repo, Path::new(doc.file))?, staged(repo, Path::new(doc.file))?];
        if held.iter().any(|h| h.as_deref() != Some(bytes.as_slice())) {
            return Err(changed("the last commit or the index holds another text"));
        }
    }
    Ok(bytes)
}

/// The file equals the text a maestro last approved (C7, REQ-FER-7; formula 1a point 4): with no file and nothing
/// approved it does; a file never approved, a changed one, or one approved and gone is the document's `unapproved` code
/// (`ALAMBIQUE_UNAPPROVED`, `FORMULA_UNAPPROVED`).
pub fn check(record: &Replay, domain: &Path, doc: &Doc) -> Result<(), Error> {
    let path = domain.join(doc.file);
    let file = read(domain, doc).map_err(|e| match e {
        Error::Refused { detail, .. } => refuse(doc.unapproved, Some(&path), detail),
        other => other,
    })?;
    let (now, approved) = (file.map(|b| sha256_hex(&b)), approved(record, doc));
    let why = match (&approved, &now) {
        (a, n) if a == n => return Ok(()),
        (None, _) => "it was never approved",
        (_, None) => "it was approved and the file is gone",
        _ => "it differs from the last approved text",
    };
    Err(refuse(doc.unapproved, Some(&path), why))
}

/// A cut of the formula (formula 1a point 5): its identifier, its title and the phase it is in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cut {
    pub id: String,
    pub title: String,
    pub phase: String,
}

/// The cuts of a formula's text, in order: each heading `### <id>. <title>` outside fences, `<id>` without spaces,
/// with the text of the level-2 heading above it as its phase.
pub fn cuts(text: &str) -> Vec<Cut> {
    let t: Vec<&str> = lines(text).into_iter().map(|l| l.1).collect();
    let fence = fenced(&t);
    let (mut phase, mut out) = ("", Vec::new());
    for (_, line) in t.iter().enumerate().filter(|(i, _)| !fence[*i]) {
        phase = line.strip_prefix("## ").map_or(phase, str::trim);
        let head = line.strip_prefix("### ").and_then(|h| h.split_once(". ")).filter(|(id, _)| !id.contains(' '));
        if let Some((id, title)) = head.filter(|(id, _)| !id.is_empty()) {
            out.push(Cut { id: id.to_owned(), title: title.trim().to_owned(), phase: phase.to_owned() });
        }
    }
    out
}

/// The last cut the record holds, opened or reported (formula 1a point 5).
pub fn cut(record: &Replay) -> Option<&Event> {
    record.streams.get(FORMULA.stream)?.iter().rev().find(|e| e.kind == "cut-recorded")
}

/// What a direct edit changed, as one hunk: the first line that differs (from 1), the lines of the old text removed
/// and those of the new put, the equal lines at the start and the end left out.
pub fn difference(old: &str, new: &str) -> Value {
    let text = |t: &str| lines(t).into_iter().map(|l| l.1.to_owned()).collect::<Vec<String>>();
    let (a, b) = (text(old), text(new));
    let head = a.iter().zip(&b).take_while(|(x, y)| x == y).count();
    let tail =
        a.iter().rev().zip(b.iter().rev()).take_while(|(x, y)| x == y).count().min(a.len() - head).min(b.len() - head);
    json!({"from": head + 1, "removed": a[head..a.len() - tail], "added": b[head..b.len() - tail]})
}

fn invalid(e: &Event, why: impl Into<String>) -> Error {
    refuse(Code::RecordTransitionInvalid, Some(&e.path), why)
}

/// The events of the streams `alambique` and `rules` held to what they decide (REQ-FER-5 to REQ-FER-7): the
/// alambique's stream opens with its baseline, a direct edit starts from the text last approved, and an incorporated
/// proposal belongs to the stream of its target, starts from the digest that stream holds and gives the result it
/// predicted; for `rules`, the rules it records are those the units give. The errors are in the order of the events.
pub fn validate(record: &Replay, proposals: &BTreeMap<String, Proposal>) -> Vec<Error> {
    let mut errors = Vec::new();
    let mut approved: Option<String> = None;
    for (i, e) in record.streams.get("alambique").into_iter().flatten().enumerate() {
        let p = &e.value["payload"];
        let step = (|| -> Result<(), Error> {
            let content = |x: Error| if x.code() == Some(Code::AnswerInvalid) { invalid(e, x.detail()) } else { x };
            crate::fermento::check(&e.kind, p).map_err(content)?;
            let field = |k: &str| p[k].as_str().unwrap_or_default().to_owned();
            if (e.kind == "alambique-baseline") != (i == 0) {
                return Err(invalid(e, "the stream opens with the baseline, and only once"));
            }
            match e.kind.as_str() {
                "alambique-baseline" => approved = Some(field("digest")),
                "direct-edit-approved" => {
                    if approved.as_deref() != Some(&field("old")) || field("old") == field("new") {
                        return Err(invalid(e, "a direct edit starts from the text last approved and changes it"));
                    }
                    approved = Some(field("new"));
                }
                _ => {
                    let q = proposals.get(&field("id")).filter(|q| q.payload["target"] == "alambique");
                    let q = q.ok_or_else(|| invalid(e, "no proposal for the alambique has this identifier"))?;
                    if approved.as_deref() != q.payload["base"].as_str() || q.payload["result"] != p["result"] {
                        return Err(invalid(e, "the proposal starts from another text or gives another result"));
                    }
                    approved = Some(field("result"));
                }
            }
            Ok(())
        })();
        errors.extend(step.err());
    }
    // The formula's stream: an import only as its first event; each approval from the digest last approved, and to
    // another; each incorporated proposal for the formula from it, with its result; a cut only of the formula approved.
    let mut approved: Option<String> = None;
    for (i, e) in record.streams.get(FORMULA.stream).into_iter().flatten().enumerate() {
        let (p, field) = (&e.value["payload"], |k: &str| e.value["payload"][k].as_str().map(str::to_owned));
        let ok = match e.kind.as_str() {
            "formula-imported" => i == 0,
            "formula-approved" => field("old") == approved && field("digest") != approved,
            "cut-recorded" => approved.is_some() && field("formula") == approved,
            _ => proposals.get(p["id"].as_str().unwrap_or_default()).is_some_and(|q| {
                q.payload["target"] == "formula"
                    && q.payload["base"].as_str() == approved.as_deref()
                    && q.payload["result"] == p["result"]
            }),
        };
        if !ok {
            let why = "the import comes first, an approval or a proposal starts from the text last approved, a cut \
                       cites it";
            errors.push(invalid(e, why));
        }
        approved = field("digest").or(field("result")).or(approved);
    }
    let mut held = Rules::new();
    for e in record.streams.get("rules").into_iter().flatten() {
        let p = &e.value["payload"];
        let step = (|| -> Result<(), Error> {
            let q = proposals.get(p["id"].as_str().unwrap_or_default()).filter(|q| q.payload["target"] == "rules");
            let q = q.ok_or_else(|| invalid(e, "no proposal for the rules has this identifier"))?;
            let units = q.payload["units"].as_array().map(Vec::as_slice).unwrap_or_default();
            let next = apply_rules(&held, units).map_err(|x| invalid(e, x.detail()))?;
            let right = q.payload["base"] == rules_digest(&held).as_str() && p["rules"] == json!(next);
            if !right || p["result"] != rules_digest(&next).as_str() || q.payload["result"] != p["result"] {
                return Err(invalid(
                    e,
                    "the rules are not those the units give from the digest the proposal starts from",
                ));
            }
            held = next;
            Ok(())
        })();
        errors.extend(step.err());
    }
    errors
}
