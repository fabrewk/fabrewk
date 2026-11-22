//! The bootstrap's import in `assess` (REQ-INS-9; "The bootstrap's import and stage 0", points 2 to 4, D-113; part
//! 11a): in a repository whose domain holds `bootstrap/` and no record, what `assess` reads of it, the split of
//! `decisions.md` into its opening and its entries, every reason that blocks the import, and the plan's `import` with
//! the files it keeps and the one it replaces. The core writes the stream from it, binds it on replay and checks
//! `bootstrap/` after (C13).

use super::{Reading, reason};
use fabrewk_core::bootstrap::{self, DECISIONS, DIR};
use fabrewk_core::record::{sha256_hex, subject_digest};
use fabrewk_core::settings::{self, Level, key_blob};
use fabrewk_core::{Code, Error, identity, scan};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::Path;

/// The labels an entry's fields take (point 3): those the file's opening names, with those its entries use besides.
const LABELS: [&str; 10] =
    ["Status", "Decided", "Date", "Origin", "Who", "Decision", "Reason", "Consequences", "Sources", "Supersedes"];

/// Why `decisions.md` does not split, with the line (from 1) where it fails.
pub type Unsplit = (usize, &'static str);

/// The split of `decisions.md` (point 3): each `entry-imported`'s payload, the opening first. Its lines are its bytes
/// split after each `\n`; an entry starts at a line outside fenced code blocks that begins with `## `, and must be
/// `## <key> — <title>` with its line end, each series (`D-`, `R-`) numbered from 1 in the file's order; its fields are
/// its top-level list items with a closed label, each value its line's rest and the lines after it indented by two
/// spaces, outside fences; `Status` stands in every entry, no label twice. The texts, joined, are the file.
pub fn split(text: &str) -> Result<Vec<Value>, Unsplit> {
    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let bare: Vec<&str> = lines.iter().map(|l| l.strip_suffix('\n').unwrap_or(l)).collect();
    let fenced = fabrewk_core::fermento::fenced(&bare);
    let starts: Vec<usize> = (0..lines.len()).filter(|&i| !fenced[i] && lines[i].starts_with("## ")).collect();
    let opening = lines[..starts.first().copied().unwrap_or(lines.len())].concat();
    let mut out = vec![json!({"key": null, "title": null, "fields": [], "text": opening})];
    let mut series = BTreeMap::new();
    for (n, &at) in starts.iter().enumerate() {
        let end = starts.get(n + 1).copied().unwrap_or(lines.len());
        let heading = lines[at].strip_suffix('\n').and_then(|l| l.strip_prefix("## ")?.split_once(" — "));
        let Some((key, title)) = heading.filter(|(k, t)| !t.is_empty() && is_key(k)) else {
            return Err((at + 1, "heading"));
        };
        let next = series.entry(&key[..2]).or_insert(0u64);
        *next += 1;
        if key[2..].parse::<u64>() != Ok(*next) {
            return Err((at + 1, "order"));
        }
        let mut fields: Vec<(&str, String)> = Vec::new();
        let mut i = at + 1;
        while i < end {
            let label = bare[i].strip_prefix("- ").and_then(|l| l.split_once(':')).filter(|_| !fenced[i]);
            let label = label.filter(|(l, rest)| LABELS.contains(l) && (rest.is_empty() || rest.starts_with(' ')));
            let Some((label, rest)) = label else {
                i += 1;
                continue;
            };
            if fields.iter().any(|(l, _)| *l == label) {
                return Err((i + 1, "twice"));
            }
            let mut value = vec![rest.strip_prefix(' ').unwrap_or(rest)];
            i += 1;
            while i < end && !fenced[i] && bare[i].starts_with("  ") {
                value.push(bare[i]);
                i += 1;
            }
            fields.push((label, value.join("\n")));
        }
        if !fields.iter().any(|(l, _)| *l == "Status") {
            return Err((at + 1, "status"));
        }
        let fields: Vec<Value> = fields.into_iter().map(|(l, v)| json!([l, v])).collect();
        out.push(json!({"key": key, "title": title, "fields": fields, "text": lines[at..end].concat()}));
    }
    Ok(out)
}

/// `D-` or `R-` and three or more digits.
fn is_key(key: &str) -> bool {
    let digits = key.strip_prefix("D-").or_else(|| key.strip_prefix("R-"));
    digits.is_some_and(|d| d.len() >= 3 && d.bytes().all(|b| b.is_ascii_digit()))
}

/// What an import keeps and replaces, which the plan's files hold, and its `import`.
pub struct Import {
    pub plan: Value,
    pub keep: Vec<Value>,
    /// Whether `.brewstillery/.gitignore` is present (and so kept, exact, or blocking).
    pub gitignore: bool,
    /// `STATUS.md`'s digest, when present and so replaced.
    pub status: Option<String>,
}

/// A reason pushed: its key, whether it blocks, its path and its detail.
type Say<'a> = dyn FnMut(&str, bool, Option<&str>, Option<&str>) + 'a;

/// The line (from 1) a byte offset of `bytes` lies on.
fn line_of(bytes: &[u8], at: usize) -> String {
    (bytes[..at].iter().filter(|b| **b == b'\n').count() + 1).to_string()
}

/// What `assess` reads and decides of an import (point 2), each reason that blocks it pushed on `reasons` and each
/// file read recorded in `r`: `bootstrap/` walked, `decisions.md` split, the publication and the tools, the project
/// file with its one maestro, the maestro's own file and fermento, the alambique, the domain's ignore file and its other
/// entries, what C9 would report, and the committed state of every file the plan names unchanged.
pub(super) fn read(
    root: &Path,
    gitdir: &Path,
    (alias, key, publication, tools): (Option<&str>, Option<&str>, Option<&str>, bool),
    r: &mut Reading,
    reasons: &mut Vec<Value>,
) -> Result<Import, Error> {
    let mut say = |key: &str, blocks: bool, path: Option<&str>, detail: Option<&str>| {
        reasons.push(reason(key, blocks, path, detail));
    };
    let walk = bootstrap::walk(root).unwrap_or_default();
    let mut set = BTreeMap::new();
    for (path, what) in &walk.odd {
        say(&format!("import-{what}"), true, Some(path), None);
    }
    let mut read: BTreeMap<String, Vec<u8>> = walk.files.clone();
    for (path, bytes) in &walk.files {
        let digest = sha256_hex(bytes);
        r.inputs.insert(path.clone(), json!({"path": path, "kind": "file", "digest": digest}));
        set.insert(path.clone(), digest);
    }
    let mut entries = Vec::new();
    match walk.files.get(DECISIONS).map(|b| (b, std::str::from_utf8(b))) {
        None => say("decisions-absent", true, Some(DECISIONS), None),
        Some((b, Err(e))) => say("decisions-not-utf8", true, Some(DECISIONS), Some(&line_of(b, e.valid_up_to()))),
        Some((b, Ok(t))) if t.contains('\r') => {
            say("decisions-carriage-return", true, Some(DECISIONS), Some(&line_of(b, t.find('\r').unwrap_or(0))));
        }
        Some((_, Ok(t))) => match split(t) {
            Ok(all) => {
                let entry = |p: &Value| json!({"key": p["key"], "title": p["title"], "digest": subject_digest(p)});
                entries = all.iter().map(entry).collect();
            }
            Err((line, why)) => say(&format!("decisions-{why}"), true, Some(DECISIONS), Some(&line.to_string())),
        },
    }
    if let Some(p) = publication.filter(|p| *p != "published") {
        say("import-publication", true, None, Some(p));
    }
    if tools {
        say("import-tool", true, None, None);
    }
    // The files an import keeps, each recorded with its digest; the project file with its content too.
    let domain = root.join(".brewstillery");
    let mut keep = Vec::new();
    let mut kept = |r: &mut Reading, rel: String, block: &mut Say| {
        let bytes = r.read(root, &rel, &rel, false);
        match (bytes, r.inputs[&rel]["kind"].as_str()) {
            (Some(b), _) => {
                set.insert(rel.clone(), sha256_hex(&b));
                read.insert(rel.clone(), b.clone());
                keep.push(json!({"path": rel, "action": "keep", "digest": sha256_hex(&b)}));
                return Some(b);
            }
            (None, Some("absent")) => block("import-absent", true, Some(&rel), None),
            (None, _) => block("not-a-file", true, Some(&rel), None),
        }
        None
    };
    let file = ".brewstillery/settings.toml";
    let bytes = kept(r, file.into(), &mut say);
    let project = bytes.map(String::from_utf8).transpose().unwrap_or_else(|_| {
        say("import-project-invalid", true, Some(file), Some("line 1: not UTF-8"));
        None
    });
    if let Some(text) = &project {
        match settings::parse(&root.join(file), text, Level::Project) {
            Err(e) => say("import-project-invalid", true, Some(file), e.first().map(Error::detail).as_deref()),
            Ok(s) => {
                let one = match &identity::maintainers(&s)[..] {
                    [m] => {
                        m.role == "maestro"
                            && alias.is_none_or(|a| a == m.alias)
                            && key.is_none_or(|k| key_blob(k) == key_blob(&m.key))
                    }
                    _ => false,
                };
                if !one {
                    say("import-maintainers", true, Some(file), None);
                }
            }
        }
    }
    if let Some(alias) = alias {
        let own = format!(".brewstillery/{alias}/settings.toml");
        if kept(r, own.clone(), &mut say).is_some()
            && let Err(e) = settings::read(&root.join(&own), Level::Personal)
        {
            say("import-personal-invalid", true, Some(&own), e.first().map(Error::detail).as_deref());
        }
        let fermento = format!(".brewstillery/{alias}/fermento.md");
        if kept(r, fermento.clone(), &mut say).is_some() && fabrewk_core::fermento::open(&domain, alias).is_err() {
            say("import-separator", true, Some(&fermento), None);
        }
    }
    kept(r, ".brewstillery/ALAMBIQUE.md".into(), &mut say);
    let gitignore = ".brewstillery/.gitignore";
    let present = super::kind(root, gitignore) != "absent";
    if present && kept(r, gitignore.into(), &mut say).is_some_and(|b| b != fabrewk_core::domain::GITIGNORE.as_bytes()) {
        say("import-gitignore", true, Some(gitignore), None);
    }
    let status = ".brewstillery/STATUS.md";
    let status = r.read(root, status, status, false).map(|b| {
        read.insert(".brewstillery/STATUS.md".into(), b.clone());
        sha256_hex(&b)
    });
    set.extend(status.clone().map(|d| (".brewstillery/STATUS.md".to_owned(), d)));
    // What `check` would report right after the import, which it does not change (P11O-7), within what `assess`
    // reads: the domain's top entries, when it is a directory and no link, and C9's heuristic over the files read and
    // the names under `bootstrap/` (P11A-3).
    let listed = super::kind(root, ".brewstillery") == "directory";
    let top = listed.then(|| std::fs::read_dir(&domain)).transpose();
    for entry in top.map_err(|e| Error::Io { path: domain.clone(), error: e })?.into_iter().flatten().flatten() {
        let name = entry.file_name();
        let ours = ["bootstrap", "skills", "data", "local"].iter().chain(&alias).any(|n| name == *n);
        if entry.file_type().is_ok_and(|t| t.is_dir()) && !ours {
            say("import-directory", true, Some(&format!(".brewstillery/{}", name.to_string_lossy())), None);
        }
    }
    let rel = |e: &Error| match e {
        Error::Refused { path: Some(p), .. } => p.strip_prefix(root).unwrap_or(p).to_string_lossy().into_owned(),
        _ => String::new(),
    };
    let mode = Some(fabrewk_core::record::Publication::Published);
    for e in listed.then(|| fabrewk_core::domain::layout(&domain, mode, true)).transpose()?.into_iter().flatten() {
        if e.code() == Some(Code::DomainUnknownEntry) {
            say("import-unknown", false, Some(&rel(&e)), None);
        }
    }
    let name = |p: &str| scan::suspect_name(p.rsplit('/').next().unwrap_or(p));
    let suspected = read.iter().filter(|(p, b)| name(p) || scan::suspect(b).is_some()).map(|(p, _)| p);
    for path in suspected.chain(walk.odd.iter().map(|(p, _)| p).filter(|p| name(p))) {
        say("import-credential", true, Some(path), None);
    }
    for path in bootstrap::uncommitted(root, Some(gitdir), &set, DIR)? {
        say("uncommitted", true, Some(&path), None);
    }
    if let (Some(text), Some(k)) = (project, keep.iter_mut().find(|f| f["path"] == file)) {
        k["content"] = text.into();
    }
    let files: Vec<Value> = walk.files.iter().map(|(p, b)| json!({"path": p, "digest": sha256_hex(b)})).collect();
    Ok(Import { plan: json!({"entries": entries, "files": files}), keep, gitignore: present, status })
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn joined(entries: &[Value]) -> String {
        entries.iter().map(|e| e["text"].as_str().unwrap()).collect()
    }

    #[test]
    fn this_repository_s_decisions_split_into_its_opening_and_entries_whose_texts_are_the_file() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../.brewstillery/bootstrap/decisions.md");
        let text = std::fs::read_to_string(path).unwrap();
        let entries = split(&text).unwrap();
        assert_eq!(joined(&entries), text);
        assert!(entries[0]["key"].is_null() && !entries[0]["text"].as_str().unwrap().contains("\n## "));
        assert!(entries[1..].iter().all(|e| e["fields"][0][0] == "Status"));
        let keys: Vec<&str> = entries[1..].iter().map(|e| e["key"].as_str().unwrap()).collect();
        assert_eq!(keys[0], "D-001");
        assert!(keys.contains(&"R-002") && keys.contains(&"D-113"));
    }

    #[test]
    fn a_text_that_does_not_split_names_its_line_and_why() {
        let entry = "## D-001 — One\n- Status: x\n";
        for (text, at) in [
            ("# D\n## D-1 — Short\n- Status: x\n", (2, "heading")),
            ("## X-001 — Other series\n- Status: x\n", (1, "heading")),
            ("## D-001 - Hyphen\n- Status: x\n", (1, "heading")),
            ("## D-001 — \n- Status: x\n", (1, "heading")),
            ("## D-001 — No line end", (1, "heading")),
            ("## D-001a — Letter\n- Status: x\n", (1, "heading")),
            (&format!("{entry}## D-003 — Gap\n- Status: x\n"), (3, "order")),
            (&format!("{entry}## D-001 — Repeat\n- Status: x\n"), (3, "order")),
            ("## R-002 — Starts at two\n- Status: x\n", (1, "order")),
            (&format!("{entry}## R-001 — None\n- Decided: x\n"), (3, "status")),
            ("## D-001 — One\n- Status: x\n- Origin: o\n- Status: y\n", (4, "twice")),
        ] {
            assert_eq!(split(text).map(|_| ()), Err(at), "{text:?}");
        }
        // Each series counts on its own; a heading fenced, or not at a line's start, starts no entry.
        let ok = "# O\n\n## D-001 — A\n- Status: x\n## R-001 — B\n- Status: y\n```\n## D-005 — fenced\n```\n ## D-009 — no\n## D-002 — C\n- Status: z\n";
        let entries = split(ok).unwrap();
        assert_eq!(entries.len(), 4);
        assert_eq!(joined(&entries), ok);
        assert_eq!(split("").unwrap(), vec![json!({"key": null, "title": null, "fields": [], "text": ""})]);
    }

    #[test]
    fn a_field_s_value_is_its_line_s_rest_and_the_lines_indented_after_it_outside_fences() {
        let text = "## D-001 — One — and more\n- Status:\n  first\n  second\n- Decided: today\n```\n  fenced\n```\n- Origin: o\n```\n- Who: fenced\n```\n- Reason:no space\n-  Sources: two spaces\n- Other: x\n  - Supersedes: nested\n";
        let entries = split(text).unwrap();
        assert_eq!(entries[1]["title"], "One — and more");
        let fields = json!([["Status", "\n  first\n  second"], ["Decided", "today"], ["Origin", "o"]]);
        assert_eq!(entries[1]["fields"], fields);
    }

    /// One block of an entry's body: a field (a label's index, the rest of its line, its lines after), a fence, or
    /// a line of the text alone.
    #[derive(Debug, Clone)]
    enum Block {
        Field(String, Vec<&'static str>),
        Fence(Vec<&'static str>),
        Plain(&'static str),
    }

    fn block() -> impl Strategy<Value = Block> {
        let rest = prop::sample::select(vec!["", "value", "a: b", "- Status: x", "## D-001 — inside"]);
        let more = prop::collection::vec(prop::sample::select(vec!["  more", "  - Status: nested", "    deep"]), 0..3);
        let fence = prop::sample::select(vec![
            vec!["```", "## D-999 — fenced", "- Status: fenced", "  indented", "```"],
            vec!["~~~text", "- Decided: fenced", "~~~"],
            vec!["````", "```", "## R-999 — fenced", "````"],
        ]);
        let plain =
            prop::sample::select(vec!["", "text", "* item", "- Other: x", "- Status:x", "-Status: x", " ## D-1 — no"]);
        prop_oneof![
            (rest, more).prop_map(|(r, m)| Block::Field(r.to_owned(), m)),
            fence.prop_map(Block::Fence),
            plain.prop_map(Block::Plain),
        ]
    }

    /// An entry: its series, its title, the order of its labels (`Status` among them) and its other blocks.
    fn entry() -> impl Strategy<Value = (bool, String, Vec<&'static str>, Vec<Block>)> {
        let labels =
            Just(LABELS.to_vec()).prop_shuffle().prop_flat_map(|l| (1..=l.len()).prop_map(move |n| l[..n].to_vec()));
        let labels = labels.prop_map(|mut l| {
            if !l.contains(&"Status") {
                l.push("Status");
            }
            l
        });
        (any::<bool>(), "[A-Za-z][A-Za-z ]{0,8}", labels, prop::collection::vec(block(), 0..6))
    }

    proptest! {
        /// Roundtrip: a document made of an opening and entries splits into exactly those entries and fields.
        #[test]
        fn a_document_splits_into_the_entries_it_was_made_of(
            opening in prop::collection::vec(prop::sample::select(vec!["# Decisions", "", "text", "- Status: o"]), 0..6),
            fenced in any::<bool>(),
            entries in prop::collection::vec(entry(), 0..6),
        ) {
            // An opening, with a fenced `## ` line or not.
            let mut text: String = opening.iter().map(|l| format!("{l}\n")).collect();
            if fenced {
                text += "```\n## fenced\n```\n";
            }
            let mut expected = vec![json!({"key": null, "title": null, "fields": [], "text": text.clone()})];
            let (mut d, mut r) = (0, 0);
            for (series, title, labels, blocks) in entries {
                let n = if series { d += 1; d } else { r += 1; r };
                let key = format!("{}-{n:03}", if series { "D" } else { "R" });
                let mut body = format!("## {key} — {title}\n");
                let mut fields = Vec::new();
                let mut blocks = blocks.into_iter();
                for label in labels {
                    // A field, then a block of its own that is no line after it.
                    let (rest, more) = match blocks.next() {
                        Some(Block::Field(rest, more)) => (rest, more),
                        _ => (String::new(), Vec::new()),
                    };
                    body += &format!("- {label}:{}{rest}\n", if rest.is_empty() { "" } else { " " });
                    body += &more.iter().map(|l| format!("{l}\n")).collect::<String>();
                    let value: Vec<String> = std::iter::once(rest).chain(more.iter().map(|m| (*m).to_owned())).collect();
                    fields.push(json!([label, value.join("\n")]));
                    match blocks.next() {
                        Some(Block::Fence(lines)) => body += &lines.iter().map(|l| format!("{l}\n")).collect::<String>(),
                        Some(Block::Plain(line)) => body += &format!("{line}\n"),
                        _ => body += "\n",
                    }
                }
                expected.push(json!({"key": key, "title": title, "fields": fields, "text": body.clone()}));
                text += &body;
            }
            prop_assert_eq!(split(&text).unwrap(), expected);
        }

        /// Whatever the lines, a text that splits is its entries' texts joined, each entry starting with `## `.
        #[test]
        fn a_text_that_splits_is_its_entries_joined(
            lines in prop::collection::vec(prop::sample::select(vec![
                "## D-001 — A", "## D-002 — B", "## R-001 — C", "## D-001 — ", "- Status: x", "- Status:", "  more",
                "```", "~~~", "````", "text", "", "- Decided: d", "## x",
            ]), 0..30),
            last_line_end in any::<bool>(),
        ) {
            let mut text = lines.join("\n");
            if last_line_end && !text.is_empty() {
                text.push('\n');
            }
            if let Ok(entries) = split(&text) {
                prop_assert_eq!(joined(&entries), text);
                prop_assert!(entries[1..].iter().all(|e| e["text"].as_str().unwrap().starts_with("## ")));
            }
        }
    }
}
