//! The catalogue of human messages (REQ-CLI-2; part 7d2): one TOML file per language, data the counter reads as data
//! (D-044). A message's key is its table path and field, `code.KEY_UNKNOWN.reason`; in the canonical `en` every table
//! of messages has its description in `about`. A language falls back to its shorter tags, then to `en`. Placeholders
//! are `{name}`; commands appear only in `run` fields, whole, so that a test can parse and shell-check each one.

use std::collections::BTreeMap;
use std::sync::LazyLock;
use toml::de::{DeTable, DeValue};

/// Every catalogue, `en` first.
pub const CATALOGUES: [(&str, &str); 2] =
    [("en", include_str!("../../catalogue/en.toml")), ("es", include_str!("../../catalogue/es.toml"))];

static READ: LazyLock<Vec<(&str, BTreeMap<String, String>)>> =
    LazyLock::new(|| CATALOGUES.iter().map(|(l, text)| (*l, messages(text))).collect());

/// Every message of a catalogue's text, by key; a text that does not read gives none (a test reads both).
pub fn messages(text: &str) -> BTreeMap<String, String> {
    fn walk(table: &DeTable, at: &str, out: &mut BTreeMap<String, String>) {
        for (k, v) in table.iter() {
            let key = if at.is_empty() { k.get_ref().to_string() } else { format!("{at}.{}", k.get_ref()) };
            match v.get_ref() {
                DeValue::Table(inner) => walk(inner, &key, out),
                DeValue::String(s) => {
                    out.insert(key, s.to_string());
                }
                _ => {}
            }
        }
    }
    let mut out = BTreeMap::new();
    if let Ok(doc) = DeTable::parse(text) {
        walk(doc.get_ref(), "", &mut out);
    }
    out
}

/// Whether `c` could move the terminal or reorder a line: a control character, a bidirectional mark, a line or
/// paragraph separator.
pub(super) fn hidden(c: char) -> bool {
    c.is_control()
        || matches!(c, '\u{61c}' | '\u{200e}' | '\u{200f}' | '\u{2028}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
}

/// `c` as a person reads it: `\u` and four hex digits, without braces, which a catalogue's placeholder uses.
pub(super) fn escape(c: char) -> String {
    format!("\\u{:04x}", c as u32)
}

/// A human output as it reaches the terminal: every hidden character but the line break written as an escape, so that
/// no text the record holds, in whatever field, can hide or reorder what follows it (P7D3B-4).
pub fn printable(text: &str) -> String {
    text.chars().map(|c| if c != '\n' && hidden(c) { escape(c) } else { c.to_string() }).collect()
}

/// The message `key` in `lang`, or in `en`, if either holds it.
pub fn get(lang: &str, key: &str) -> Option<&'static str> {
    let find = |l: &str| READ.iter().find(|(c, _)| *c == l).and_then(|(_, m)| m.get(key)).map(String::as_str);
    find(lang).or_else(|| find("en"))
}

/// The message `key` in `lang` with each `{name}` of `params` filled. A key no catalogue holds is a defect the tests
/// catch; it renders as the catalogue's own sentence for a missing message, never as the bare key.
pub fn say(lang: &str, key: &str, params: &[(&str, &str)]) -> String {
    let mut rest = get(lang, key).or_else(|| get(lang, "internal.missing")).unwrap_or_default();
    // In one pass over the message: a value filled in is never read again for a name (P7D3B-3).
    let mut out = String::with_capacity(rest.len());
    while let Some(open) = rest.find('{') {
        out += &rest[..open];
        let after = &rest[open + 1..];
        let named = after.find('}').and_then(|close| {
            params.iter().find(|(name, _)| *name == &after[..close]).map(|(_, value)| (*value, close))
        });
        match named {
            Some((value, close)) => {
                out += value;
                rest = &after[close + 1..];
            }
            None => {
                out.push('{');
                rest = after;
            }
        }
    }
    out + rest
}
