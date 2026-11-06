//! What looks like a credential (REQ-SEC-3; part 7d1, D-053): a heuristic, a cooperative control, matched by hand with
//! no regular-expression crate. `check` scans the domain with it (C9); `run` will scan a candidate's change set.

use crate::code::{Code, Error, refuse};
use std::fs;
use std::path::Path;

/// Prefixes followed by at least so many token characters (ASCII letters, digits, `_` and `-`).
const TOKENS: [(&str, usize); 13] = [
    ("ghp_", 20),
    ("gho_", 20),
    ("ghu_", 20),
    ("ghs_", 20),
    ("ghr_", 20),
    ("github_pat_", 20),
    ("sk-ant-", 20),
    ("sk-", 20),
    ("xoxb-", 10),
    ("xoxa-", 10),
    ("xoxp-", 10),
    ("xoxr-", 10),
    ("xoxs-", 10),
];

/// Whether a file's name is a credential's: `.env`, `id_rsa`, `id_ecdsa`, `id_ed25519`, or ending in `.pem`, `.p12`,
/// `.pfx` or `.key`.
pub fn suspect_name(name: &str) -> bool {
    [".env", "id_rsa", "id_ecdsa", "id_ed25519"].contains(&name)
        || [".pem", ".p12", ".pfx", ".key"].iter().any(|e| name.ends_with(e))
}

/// What in `bytes` looks like a credential, if anything: a line that is exactly `-----BEGIN `, optional uppercase
/// words each followed by a space, and `PRIVATE KEY-----`; `AKIA` and 16 uppercase letters or digits; or a prefix of
/// [`TOKENS`] followed by enough token characters.
pub fn suspect(bytes: &[u8]) -> Option<&'static str> {
    let private = |line: &[u8]| {
        let line = line.strip_suffix(b"\r").unwrap_or(line);
        let words = line.strip_prefix(b"-----BEGIN ").and_then(|l| l.strip_suffix(b"PRIVATE KEY-----"));
        let word = |w: &[u8]| !w.is_empty() && w.iter().all(u8::is_ascii_uppercase);
        words.is_some_and(|w| w.is_empty() || w.strip_suffix(b" ").is_some_and(|w| w.split(|b| *b == b' ').all(word)))
    };
    if bytes.split(|b| *b == b'\n').any(private) {
        return Some("a private key");
    }
    let token = |b: &&u8| b.is_ascii_alphanumeric() || **b == b'_' || **b == b'-';
    for i in 0..bytes.len() {
        let rest = &bytes[i..];
        let key = |b: &&u8| b.is_ascii_uppercase() || b.is_ascii_digit();
        if rest.starts_with(b"AKIA") && rest[4..].iter().take_while(key).count() >= 16 {
            return Some("AKIA");
        }
        let long = |(p, n): &&(&str, usize)| {
            rest.starts_with(p.as_bytes()) && rest[p.len()..].iter().take_while(token).count() >= *n
        };
        if let Some((p, _)) = TOKENS.iter().find(long) {
            return Some(p);
        }
    }
    None
}

/// `check`'s C9: every file under `domain` that looks like a credential is `CREDENTIAL_SUSPECTED`, by its content, or
/// by its name alone for a maintainer's uncited material (in `<alias>/`, anything but `fermento.md`, `settings.toml`
/// and what `cited` accepts), so that only what is cited is read; links are matched by their names and never
/// followed. A file it must read and cannot, a directory it cannot list, or a domain that is not a directory (a link
/// included), is `CHECK_UNKNOWN`.
pub fn domain(domain: &Path, cited: &dyn Fn(&Path) -> bool) -> Vec<Error> {
    // Without a final `/` or `.`, which would resolve a link before its entry is examined (P7D1-5).
    let domain: &Path = &domain.components().collect::<std::path::PathBuf>();
    let named = domain.file_name().is_some();
    if !named || !fs::symlink_metadata(domain).is_ok_and(|m| m.is_dir()) {
        return vec![refuse(Code::CheckUnknown, Some(domain), "the domain is not a directory; a link is not followed")];
    }
    let (mut out, mut files, mut dirs) = (Vec::new(), Vec::new(), vec![domain.to_path_buf()]);
    while let Some(dir) = dirs.pop() {
        let entries = fs::read_dir(&dir).and_then(|e| e.map(|e| e.map(|e| e.path())).collect::<Result<Vec<_>, _>>());
        match entries {
            Ok(entries) => {
                for path in entries {
                    match fs::symlink_metadata(&path) {
                        Ok(m) if m.is_dir() => dirs.push(path),
                        Ok(m) => files.push((path, m.is_file())),
                        Err(_) => out.push(refuse(Code::CheckUnknown, Some(&path), "cannot be examined")),
                    }
                }
            }
            Err(_) => out.push(refuse(Code::CheckUnknown, Some(&dir), "cannot be listed")),
        }
    }
    files.sort();
    for (path, regular) in files {
        let rel = path.strip_prefix(domain).unwrap_or(&path);
        let mut parts = rel.iter().map(|p| p.to_str().unwrap_or_default());
        let (top, second) = (parts.next().unwrap_or_default(), parts.next());
        let own = matches!(second, Some("fermento.md" | "settings.toml")) && rel.iter().count() == 2;
        let uncited = crate::settings::is_alias(top) && second.is_some() && !own && !cited(&path);
        let name = path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();
        // Read only what must be read: a regular file outside a maintainer's uncited material.
        let content = match (regular && !uncited).then(|| fs::read(&path)) {
            Some(Ok(bytes)) => suspect(&bytes),
            Some(Err(_)) => {
                out.push(refuse(Code::CheckUnknown, Some(&path), "cannot be read"));
                None
            }
            None => None,
        };
        let why = if suspect_name(&name) { Some("its name") } else { content };
        if let Some(why) = why {
            let detail = format!("looks like a credential ({why}); a heuristic, a cooperative control");
            out.push(refuse(Code::CredentialSuspected, Some(&path), detail));
        }
    }
    out
}
