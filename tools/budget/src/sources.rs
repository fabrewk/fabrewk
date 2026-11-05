//! What the compiler really read, from the dep-info Cargo writes beside the binary (`<binary>.d`), against the count.
//!
//! Every file there must be a file the counter counted, or a data file a counted file names in `include_str!` or
//! `include_bytes!`. Anything else (a module behind a `path` attribute, a target or build script the manifest points
//! elsewhere, a file pulled in by a macro) is Rust the budget would not see.

use crate::rings::{RingCount, canonical};
use std::collections::BTreeSet;
use std::path::PathBuf;

/// The prerequisites of a Makefile-style dep-info holding one rule: `target: dep dep …`. A backslash anywhere is
/// refused: Cargo writes `\ ` for a space in a name, but a name may also end in a backslash, so `a\ b` can be one path
/// or two, and reading it either way could match the wrong file (Codex's P7A-22 and P7A-23). A repository whose path
/// holds a space therefore cannot be checked: a false positive D-044 accepts.
pub fn parse_dep_info(text: &str) -> Result<Vec<PathBuf>, String> {
    if text.contains('\\') {
        return Err("dep-info with a backslash; a path with a space or a backslash cannot be read safely".to_string());
    }
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let line = lines.next().ok_or("empty dep-info")?;
    if lines.next().is_some() {
        // Cargo writes the binary's dep-info as one rule; a second line could be the rest of a path holding a newline,
        // which this reader cannot tell apart, so it refuses rather than drop what follows.
        return Err("dep-info with more than one rule; it cannot be read safely".to_string());
    }
    let (mut deps, mut current, mut chars) = (Vec::new(), String::new(), line.chars());
    let mut target_done = false;
    while let Some(c) = chars.next() {
        match c {
            ':' if !target_done && chars.clone().next().is_none_or(char::is_whitespace) => {
                target_done = true;
                current.clear();
            }
            c if c.is_whitespace() => {
                if target_done && !current.is_empty() {
                    deps.push(PathBuf::from(std::mem::take(&mut current)));
                }
                current.clear();
            }
            c => current.push(c),
        }
    }
    if !target_done {
        return Err("dep-info without a target".to_string());
    }
    if !current.is_empty() {
        deps.push(PathBuf::from(current));
    }
    Ok(deps)
}

/// The compiled files neither counted nor named as data, canonical.
pub fn unaccounted(deps: &[PathBuf], counts: &[RingCount]) -> Vec<PathBuf> {
    let known: BTreeSet<&PathBuf> = counts.iter().flat_map(|c| c.sources.iter().chain(&c.data)).collect();
    deps.iter().map(|d| canonical(d)).filter(|d| !known.contains(d)).collect()
}
