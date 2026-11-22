//! The bootstrap's import, as the core holds it (REQ-INS-9; "The bootstrap's import and stage 0", D-113; part 11a):
//! the walk of `.brewstillery/bootstrap/`, no link followed; the committed state of the files an import names, compared
//! raw with `HEAD`'s blobs and the index (point 2's last reason); the stream's events the plan gives, which `apply`
//! writes (point 6); the replay's binding of the stream `bootstrap` to the plan (point 5); and the read-only check C13
//! (point 7). The split of `decisions.md` and the import's reasons are the installer's.

use crate::code::{Code, Error, io_at, refuse, unlinked};
use crate::record::{Event, Replay, sha256_hex, subject_digest};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::Path;

/// The directory the import reads, from the repository's root.
pub const DIR: &str = ".brewstillery/bootstrap";

/// The file whose entries the stream holds.
pub const DECISIONS: &str = ".brewstillery/bootstrap/decisions.md";

/// The entries of `decisions.md` as the installer splits them: each `entry-imported`'s payload, the opening first, or
/// `None` when the text does not split (point 3).
pub type Split = fn(&str) -> Option<Vec<Value>>;

/// What the walk of `bootstrap/` finds.
#[derive(Debug, Default)]
pub struct Walk {
    /// Each regular file of at most 1 MiB, by its path from the repository's root, with its bytes.
    pub files: BTreeMap<String, Vec<u8>>,
    /// Every other entry, by its path, and what it is: `link`, `other` (neither a regular file nor a directory),
    /// `not-utf8` (its name), `too-large`, `empty` (a directory that holds nothing but directories, at any depth) or
    /// `unreadable`.
    pub odd: Vec<(String, &'static str)>,
}

/// Walks `bootstrap/` under `root`, following no link on the way or at an entry; `None` when it is absent. A
/// `bootstrap/` that is itself a link or not a directory is its one odd entry.
pub fn walk(root: &Path) -> Option<Walk> {
    let mut out = Walk::default();
    match std::fs::symlink_metadata(root.join(DIR)) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return None,
        Err(_) => out.odd.push((DIR.into(), "unreadable")),
        Ok(m) if m.is_dir() && unlinked(root, DIR) => _ = visit(root, DIR, &mut out),
        Ok(m) if m.is_dir() || m.file_type().is_symlink() || !unlinked(root, DIR) => out.odd.push((DIR.into(), "link")),
        Ok(_) => out.odd.push((DIR.into(), "other")),
    }
    Some(out)
}

/// Walks the directory `rel`; whether it holds anything but directories, at any depth.
fn visit(root: &Path, rel: &str, out: &mut Walk) -> bool {
    let Ok(mut entries) = std::fs::read_dir(root.join(rel)).and_then(|e| e.collect::<Result<Vec<_>, _>>()) else {
        out.odd.push((rel.into(), "unreadable"));
        return true;
    };
    entries.sort_by_key(|e| e.file_name());
    let mut held = false;
    for e in entries {
        let Some(name) = e.file_name().to_str().map(|n| format!("{rel}/{n}")) else {
            out.odd.push((format!("{rel}/{}", e.file_name().to_string_lossy()), "not-utf8"));
            held = true;
            continue;
        };
        let what = match std::fs::symlink_metadata(e.path()) {
            Ok(m) if m.is_dir() => {
                held |= visit(root, &name, out);
                continue;
            }
            Ok(m) if m.file_type().is_symlink() => "link",
            Ok(m) if m.is_file() && m.len() > 1 << 20 => "too-large",
            Ok(m) if m.is_file() => match std::fs::read(e.path()) {
                Ok(bytes) => {
                    out.files.insert(name, bytes);
                    held = true;
                    continue;
                }
                Err(_) => "unreadable",
            },
            Ok(_) => "other",
            Err(_) => "unreadable",
        };
        out.odd.push((name, what));
        held = true;
    }
    if !held {
        out.odd.push((rel.into(), "empty"));
    }
    held
}

/// Each path of `files` (from the root, with the SHA-256 its bytes must have) whose committed state is not those bytes
/// (point 2's last reason; P11O-1, P11O-2, P11O-5), and each file `HEAD`'s tree lists under `under` that `files` does
/// not hold: `HEAD`'s tree must hold it as a regular file whose blob, read raw, has that digest; the index entry must
/// be `HEAD`'s, at stage 0, with no `assume-unchanged` or `skip-worktree` flag; the working tree must hold it as a
/// regular file, reached through no link, with that digest and the executable bit `HEAD`'s mode gives. Git runs as
/// the installer's contract runs it, with `gitdir` when given.
pub fn uncommitted(
    root: &Path,
    gitdir: Option<&Path>,
    files: &BTreeMap<String, String>,
    under: &str,
) -> Result<Vec<String>, Error> {
    use std::os::unix::fs::PermissionsExt;
    let git = |args: &[&str]| -> Result<Vec<u8>, Error> {
        match crate::install::git(root, gitdir, args, b"")? {
            (Some(0), out) => Ok(out),
            (status, _) => Err(refuse(Code::ToolFailed, Some(root), format!("git {} failed: {status:?}", args[0]))),
        }
    };
    if crate::install::git(root, gitdir, &["rev-parse", "-q", "--verify", "HEAD^{commit}"], b"")?.0 != Some(0) {
        return Ok(files.keys().cloned().collect());
    }
    // `<mode> <type> <id>\t<path>` and `<tag> <mode> <id> <stage>\t<path>`, by path; a path that is not UTF-8, which
    // no file of `files` is, kept with a NUL no path holds, so that it is never taken for one nor dropped (P11A-4).
    let listed = |out: Vec<u8>| -> BTreeMap<String, Vec<String>> {
        let entries = out.split(|b| *b == 0).filter_map(|e| {
            let at = e.iter().position(|b| *b == b'\t')?;
            let path = std::str::from_utf8(&e[at + 1..])
                .map_or_else(|_| String::from_utf8_lossy(&e[at + 1..]) + "\0", Into::into);
            Some((path.into_owned(), String::from_utf8_lossy(&e[..at]).split(' ').map(str::to_owned).collect()))
        });
        entries.collect()
    };
    let paths: Vec<&str> = files.keys().map(String::as_str).chain([under]).collect();
    let tree = listed(git(&[&["ls-tree", "-r", "-z", "--full-tree", "HEAD", "--"][..], &paths].concat())?);
    let literal: Vec<String> = paths.iter().map(|p| format!(":(literal){p}")).collect();
    let literal: Vec<&str> = literal.iter().map(String::as_str).collect();
    let index = listed(git(&[&["ls-files", "-z", "-s", "-v", "--"][..], &literal].concat())?);
    let mut out: Vec<String> =
        tree.keys().filter(|p| p.starts_with(&format!("{under}/")) && !files.contains_key(*p)).cloned().collect();
    for (path, digest) in files {
        let held = tree.get(path).filter(|t| t[1] == "blob" && (t[0] == "100644" || t[0] == "100755"));
        let staged = index
            .get(path)
            .filter(|i| held.is_some_and(|t| i[0] == "H" && i[1] == t[0] && i[2] == t[2] && i[3] == "0"));
        let meta = std::fs::symlink_metadata(root.join(path)).ok().filter(|m| m.is_file() && unlinked(root, path));
        let bytes = meta.as_ref().and_then(|_| std::fs::read(root.join(path)).ok());
        let ok = match (held, staged, meta, bytes) {
            (Some(t), Some(_), Some(m), Some(b)) => {
                (m.permissions().mode() & 0o100 != 0) == (t[0] == "100755")
                    && sha256_hex(&b) == *digest
                    && sha256_hex(&git(&["cat-file", "blob", &t[2]])?) == *digest
            }
            _ => false,
        };
        if !ok {
            out.push(path.clone());
        }
    }
    out.sort();
    Ok(out)
}

/// The plan's `import.files`, by path, with their digests.
fn listed(import: &Value) -> BTreeMap<String, String> {
    let files = import["files"].as_array().into_iter().flatten();
    files.filter_map(|f| Some((f["path"].as_str()?.to_owned(), f["digest"].as_str()?.to_owned()))).collect()
}

/// The stream's events the plan gives, by their payloads (point 6), read under the lock before any write: the walk of
/// `bootstrap/` must find exactly the plan's `import.files`, each with its digest, and `decisions.md`, split as the
/// installer splits it, must give each `entry-imported` the plan names at its place, by key, title and payload's
/// digest; then `bootstrap-imported`. `PLAN_CHANGED` otherwise, nothing written.
pub fn planned(root: &Path, plan: &Value, digest: &str, split: Split) -> Result<Vec<Value>, Error> {
    let changed = |why: &str| refuse(Code::PlanChanged, Some(&root.join(DIR)), why.to_owned());
    let import = &plan["import"];
    let walk = walk(root).unwrap_or_default();
    let found: BTreeMap<String, String> = walk.files.iter().map(|(p, b)| (p.clone(), sha256_hex(b))).collect();
    if !walk.odd.is_empty() || found != listed(import) {
        return Err(changed("bootstrap/ no longer holds exactly the files the plan imports"));
    }
    let text = walk.files.get(DECISIONS).and_then(|b| std::str::from_utf8(b).ok());
    let mut payloads = text.and_then(split).ok_or_else(|| changed("decisions.md no longer splits"))?;
    let entries = import["entries"].as_array().map(Vec::as_slice).unwrap_or_default();
    if payloads.len() != entries.len() || payloads.iter().zip(entries).any(|(p, e)| !bound(e, p)) {
        return Err(changed("decisions.md no longer gives the entries the plan names"));
    }
    payloads.push(json!({"plan": digest, "entries": entries.len(), "files": import["files"]}));
    Ok(payloads)
}

/// Whether an `entry-imported`'s payload is the one the plan's entry names: its key, its title and its digest.
fn bound(entry: &Value, p: &Value) -> bool {
    entry["key"] == p["key"] && entry["title"] == p["title"] && entry["digest"].as_str() == subject_digest(p).as_deref()
}

/// The stream `bootstrap` held to the plan (point 5): it stands only where the `project` stream opens with an
/// `assessed` whose plan imports; it holds `entry-imported` events, each the plan's at its place, citing the plan's
/// project file, then one `bootstrap-imported`, last, holding the plan's digest, entries and files, the texts joined
/// having the digest the plan gives `decisions.md`; and the first adoption's seal holds it at that event. A prefix
/// stands while no adoption is written.
pub(crate) fn validate(record: &Replay) -> Option<Error> {
    let invalid = |e: &Event, why: &str| Some(refuse(Code::RecordTransitionInvalid, Some(&e.path), why.to_owned()));
    let project = record.streams.get("project").map(Vec::as_slice).unwrap_or_default();
    let events = record.streams.get("bootstrap").map(Vec::as_slice).unwrap_or_default();
    let assessed = project.first().filter(|e| e.kind == "assessed").map(|e| &e.value["payload"]);
    let Some((a, import)) = assessed.map(|a| (a, &a["plan"]["import"])).filter(|(_, i)| i.is_object()) else {
        return events.first().and_then(|e| invalid(e, "the bootstrap stream stands only where apply's plan imports"));
    };
    let policy = crate::identity::planned_policy(&a["plan"]).map(|t| sha256_hex(t.as_bytes()));
    let entries = import["entries"].as_array().map(Vec::as_slice).unwrap_or_default();
    let mut text = Vec::new();
    for (i, e) in events.iter().enumerate() {
        let p = &e.value["payload"];
        let ok = e.value["policy"].as_str() == policy.as_deref()
            && match (e.kind.as_str(), entries.get(i)) {
                ("entry-imported", Some(entry)) => {
                    text.extend_from_slice(p["text"].as_str().unwrap_or_default().as_bytes());
                    bound(entry, p)
                }
                ("bootstrap-imported", None) => {
                    i + 1 == events.len()
                        && p["plan"] == a["digest"]
                        && p["entries"] == json!(entries.len())
                        && p["files"] == import["files"]
                        && listed(import).get(DECISIONS) == Some(&sha256_hex(&text))
                }
                _ => false,
            };
        if !ok {
            return invalid(e, "the bootstrap stream is not the import its plan names, in its order");
        }
    }
    let adoption = project.iter().find(|e| e.kind == "policy-adopted")?;
    let last = events.last().filter(|e| e.kind == "bootstrap-imported");
    let sealed = &adoption.value["payload"]["heads"]["bootstrap"];
    match last.is_some_and(|l| *sealed == json!({"digest": l.digest, "seq": l.seq})) {
        true => None,
        false => invalid(adoption, "the first adoption's seal does not hold the whole import"),
    }
}

/// The record's `bootstrap-imported`, once the import is complete.
pub fn imported(record: &Replay) -> Option<&Event> {
    record.streams.get("bootstrap")?.last().filter(|e| e.kind == "bootstrap-imported")
}

/// C13 (point 7): once the record holds `bootstrap-imported`, `bootstrap/` under `root` must be a directory holding
/// exactly the files it lists, each a regular file with its digest, and nothing else at any depth, an empty directory
/// included. Each difference is `BOOTSTRAP_CHANGED`, naming the path and whether it was changed, added or removed; an
/// entry that cannot be read is `CHECK_UNKNOWN`.
pub fn check(root: &Path, record: &Replay) -> Vec<Error> {
    let Some(event) = imported(record) else { return Vec::new() };
    let mut listed = listed(&event.value["payload"]);
    let walk = walk(root).unwrap_or_default();
    let mut out = Vec::new();
    let mut found = |path: &str, why: String| out.push(refuse(Code::BootstrapChanged, Some(&root.join(path)), why));
    for (path, what) in &walk.odd {
        let place = if listed.remove(path).is_some() { "changed" } else { "added" };
        match *what {
            "unreadable" => {}
            what => found(path, format!("{place}: {}", what.replace('-', " "))),
        }
    }
    for (path, bytes) in &walk.files {
        match listed.remove(path) {
            Some(d) if d == sha256_hex(bytes) => {}
            Some(_) => found(path, "changed: its bytes are not those imported".into()),
            None => found(path, "added: not a file the import lists".into()),
        }
    }
    for path in listed.keys() {
        found(path, "removed: a file the import lists is gone".into());
    }
    let unknown = walk.odd.iter().filter(|(_, w)| *w == "unreadable");
    out.extend(unknown.map(|(p, _)| io_at(&root.join(p))(std::io::ErrorKind::PermissionDenied.into())));
    out
}
