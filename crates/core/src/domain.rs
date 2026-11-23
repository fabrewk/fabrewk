//! The domain's layout (REQ-DOM-2, REQ-DOM-4, REQ-DOM-7; part 7c2b, D-051): what may stand at `.brewstillery/`'s top,
//! its ignore file, the publication mode the record holds, and whether the product's repository ignores the domain
//! in the modes that must not publish it. Every check here only reads; nothing is ever deleted.

use crate::code::{Code, Error, io_at, refuse};
use crate::record::{self, Publication, Replay, Repo, write_atomic_via};
use crate::settings::{RESERVED, is_alias};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

/// The exact content of `.brewstillery/.gitignore` (REQ-DOM-4).
pub const GITIGNORE: &str = "local/\n";

/// The line `apply` adds to the product's exclude file in `own-repository` and `unpublished` (REQ-DOM-7).
pub const EXCLUDE: &str = "/.brewstillery/";

const FILES: [&str; 6] = ["ALAMBIQUE.md", "FORMULA.md", "STATUS.md", "RULES.md", "settings.toml", ".gitignore"];
const DIRS: [&str; 3] = ["data", "local", "skills"];

/// Every entry at the domain's top that is not part of it (REQ-DOM-2), in name order: a directory whose name is not an
/// alias and not reserved is `ALIAS_INVALID`; anything else not listed, a listed name of the wrong kind, and a link
/// (never followed) are `DOMAIN_UNKNOWN_ENTRY`. `bootstrap/`, and `review/` while review mode is not built, are
/// reserved and so unknown entries (D-051), `bootstrap/` until the record holds its import, `imported`, and then part
/// of the domain (D-113 point 7). In `own-repository` the domain's own repository, `.git` as a directory, is part of it
/// (D-091). Each is a finding for `check`, never a reason to delete.
pub fn layout(domain: &Path, mode: Option<Publication>, imported: bool) -> Result<Vec<Error>, Error> {
    let own = mode == Some(Publication::OwnRepository);
    let mut entries = Vec::new();
    for entry in fs::read_dir(domain).map_err(io_at(domain))? {
        let entry = entry.map_err(io_at(domain))?;
        let kind = entry.file_type().map_err(io_at(&entry.path()))?;
        entries.push((entry.path(), kind));
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let mut found = Vec::new();
    for (path, kind) in entries {
        let name = path.file_name().and_then(OsStr::to_str);
        let alias = (Code::AliasInvalid, "a directory whose name is not an alias");
        let (code, why) = match name {
            Some(n) if FILES.contains(&n) && kind.is_file() => continue,
            Some(n) if DIRS.contains(&n) && kind.is_dir() => continue,
            Some(".git") if own && kind.is_dir() => continue,
            Some("bootstrap") if imported && kind.is_dir() => continue,
            Some(n) if kind.is_dir() && is_alias(n) => continue,
            Some(n) if kind.is_dir() && !RESERVED.contains(&n) => alias,
            None if kind.is_dir() => alias,
            _ => (Code::DomainUnknownEntry, "not part of the domain"),
        };
        found.push(refuse(code, Some(&path), why));
    }
    Ok(found)
}

/// Refuses `GITIGNORE_CHANGED` unless `.brewstillery/.gitignore` is a file holding exactly [`GITIGNORE`] (REQ-DOM-4).
pub fn check_gitignore(domain: &Path) -> Result<(), Error> {
    let path = domain.join(".gitignore");
    let file = fs::symlink_metadata(&path).is_ok_and(|m| m.is_file());
    if !file || fs::read(&path).map_err(io_at(&path))? != GITIGNORE.as_bytes() {
        return Err(refuse(Code::GitignoreChanged, Some(&path), "differs from what Fabrewk writes"));
    }
    Ok(())
}

/// The publication mode in force: that of the last `publication-decided` (REQ-DOM-7); `None` before `apply`.
pub fn publication(record: &Replay) -> Option<Publication> {
    let decided = record.streams.get("project")?.iter().rev().find(|e| e.kind == "publication-decided")?;
    match decided.value["payload"]["mode"].as_str()? {
        "published" => Some(Publication::Published),
        "own-repository" => Some(Publication::OwnRepository),
        "unpublished" => Some(Publication::Unpublished),
        _ => None,
    }
}

/// The product's repository: the working tree that holds the domain at its top.
fn product(domain: &Path) -> Result<Repo, Error> {
    let not = || refuse(Code::NotARepository, Some(domain), "the domain has no parent");
    Ok(Repo {
        root: domain.parent().ok_or_else(not)?.to_path_buf(),
        prefix: PathBuf::from(domain.file_name().ok_or_else(not)?),
    })
}

/// What every write command checks first in `own-repository` and `unpublished` (REQ-DOM-7): that the domain stands at
/// the root of the product's main working tree (`NOT_A_REPOSITORY` otherwise, since a repository found further up is
/// not the product's) and that the product's repository ignores it, by `git check-ignore`, so no later commit there
/// publishes it; else `DOMAIN_NOT_IGNORED`. A domain the product tracks is not ignored. In `published` there is
/// nothing to check.
pub fn check_ignored(domain: &Path, mode: Publication) -> Result<(), Error> {
    if mode == Publication::Published {
        return Ok(());
    }
    let repo = product(domain)?;
    record::check_main_worktree(&repo)?;
    if !record::ignores(&repo, &repo.prefix)? {
        return Err(refuse(Code::DomainNotIgnored, Some(domain), "the product's repository would publish it"));
    }
    Ok(())
}

/// Adds [`EXCLUDE`] to the product's exclude file unless a line already holds it, for `apply` (REQ-DOM-7, phase 10);
/// gives whether it wrote.
pub fn exclude_domain(domain: &Path) -> Result<bool, Error> {
    let path = record::exclude_file(&product(domain)?)?;
    let mut text = match fs::read(&path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        other => other.map_err(io_at(&path))?,
    };
    if text.split(|b| *b == b'\n').any(|l| l == EXCLUDE.as_bytes()) {
        return Ok(false);
    }
    if text.last().is_some_and(|b| *b != b'\n') {
        text.push(b'\n');
    }
    text.extend_from_slice(format!("{EXCLUDE}\n").as_bytes());
    let dir = path.parent().unwrap_or(domain);
    fs::create_dir_all(dir).map_err(io_at(dir))?;
    // Its temporary file in the domain's unversioned area: the Git directory is the person's (P10B-1).
    write_atomic_via(&path, &text, &domain.join("local/tmp")).map(|()| true)
}
