//! What the compiler really read, from the dep-info Cargo writes beside the binary (`<binary>.d`), against the count.
//!
//! Every file there must be a file the counter counted, or a data file a counted file names in `include_str!` or
//! `include_bytes!`. Anything else (a module behind a `path` attribute, a target or build script the manifest points
//! elsewhere, a file pulled in by a macro) is Rust the budget would not see.

use crate::rings::{RingCount, canonical};
use std::collections::BTreeSet;
use std::path::PathBuf;

/// The prerequisites of a Makefile-style dep-info: `target: dep dep …`, spaces escaped as `\ `, lines continued
/// with a trailing `\`.
pub fn parse_dep_info(text: &str) -> Result<Vec<PathBuf>, String> {
    let joined = text.replace("\\\n", " ");
    let line = joined
        .lines()
        .find(|l| !l.trim().is_empty())
        .ok_or("empty dep-info")?;
    let (mut deps, mut current, mut chars) = (Vec::new(), String::new(), line.chars());
    let mut target_done = false;
    while let Some(c) = chars.next() {
        match c {
            '\\' => current.extend(chars.next()),
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
    let known: BTreeSet<&PathBuf> = counts
        .iter()
        .flat_map(|c| c.sources.iter().chain(&c.data))
        .collect();
    deps.iter()
        .map(|d| canonical(d))
        .filter(|d| !known.contains(d))
        .collect()
}
