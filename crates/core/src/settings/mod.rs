//! Settings (REQ-SET-1 to REQ-SET-7, REQ-SET-10, REQ-ID-1, REQ-DOM-3; part 7c1, D-048). Three TOML files, never
//! rewritten: the project's, each maintainer's and the machine's. Every key is declared once ([`KEYS`]); every refusal
//! names the file, the key path and the line. The policy in force is the project file a maestro adopted, never the
//! file on disk ([`policy`]).

mod keys;
mod lang;
mod policy;

pub use keys::KEYS;
pub use lang::{fallbacks, is_tag};
pub use policy::{
    Adoption, Change, Effect, Effective, Policy, Source, adopt, changes, effective, in_force, lookup, pending, require,
};

use crate::code::{Code, Error, io_at, refuse};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use toml::de::{DeTable, DeValue};

/// Where a value is set (REQ-SET-6): the files, and a stage's or a task's authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Project,
    Personal,
    Machine,
    Stage,
    Task,
}

/// Whether a larger or added value widens authority, narrows it, or neither (section 8, "Dir.").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Widens,
    Narrows,
    Neutral,
}

/// A key's type. `Ordered` lists its values so that a later one is larger, read with the key's direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Str,
    Alias,
    Bool,
    Int,
    Strs,
    Command,
    Tag,
    Tags,
    SignerKey,
    Enum(&'static [&'static str]),
    Ordered(&'static [&'static str]),
}

/// A key's default: none (the key must be set), none at all, a TOML literal, or another key's value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Def {
    Required,
    Absent,
    Lit(&'static str),
    From(&'static str),
}

/// One declared key (REQ-SET-2).
#[derive(Debug, Clone, Copy)]
pub struct Key {
    pub path: &'static str,
    pub ty: Type,
    pub levels: &'static [Level],
    pub default: Def,
    pub dir: Dir,
    pub about: &'static str,
}

/// A value: every list in the first core is a list of strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Val {
    Bool(bool),
    Int(i64),
    Str(String),
    List(Vec<String>),
}

/// A value read from a file, with its line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub value: Val,
    pub line: usize,
}

/// One file, read and validated: its values by key path, and every table it opens with the table's line.
#[derive(Debug, Clone)]
pub struct Settings {
    pub path: PathBuf,
    pub level: Level,
    pub values: BTreeMap<String, Entry>,
    pub tables: BTreeMap<String, usize>,
}

/// Names an alias may not take (REQ-DOM-3); the list lives in the code, never in a settings file.
pub const RESERVED: [&str; 9] =
    ["ALAMBIQUE.md", "STATUS.md", "RULES.md", "settings.toml", ".gitignore", "data", "local", "review", "bootstrap"];

/// Whether `s` may be an alias: REQ-DOM-3's pattern, at most 32 characters, not reserved.
pub fn is_alias(s: &str) -> bool {
    crate::record::is_name(s) && !RESERVED.contains(&s)
}

/// The declaration of a key path, `*` matching any one name.
pub fn declared(path: &str) -> Option<&'static Key> {
    let segs: Vec<&str> = path.split('.').collect();
    KEYS.iter().find(|k| matches(k.path, &segs))
}

fn matches(pattern: &str, segs: &[&str]) -> bool {
    let p: Vec<&str> = pattern.split('.').collect();
    p.len() == segs.len() && p.iter().zip(segs).all(|(a, b)| *a == "*" || a == b)
}

/// Whether some key lies below the table `segs`; and whether that table's last segment is a name (`*`).
fn table_of(segs: &[&str]) -> Option<bool> {
    let mut named = None;
    for k in KEYS {
        let p: Vec<&str> = k.path.split('.').collect();
        if p.len() > segs.len() && matches(&p[..segs.len()].join("."), segs) {
            named = Some(p[segs.len() - 1] == "*");
        }
    }
    named
}

/// The concrete paths of `key` in a file whose named tables are `tables`.
pub(crate) fn instances(key: &Key, tables: &BTreeMap<String, usize>) -> Vec<String> {
    let Some((table, last)) = key.path.rsplit_once('.').filter(|(t, _)| t.contains('*')) else {
        return vec![key.path.to_owned()];
    };
    let named = |t: &&String| matches(table, &t.split('.').collect::<Vec<_>>());
    tables.keys().filter(named).map(|t| format!("{t}.{last}")).collect()
}

/// Reads and validates one file at `level`.
pub fn read(path: &Path, level: Level) -> Result<Settings, Vec<Error>> {
    read_text(path, level).map(|(_, s)| s)
}

/// Reads one file at `level`, giving its text with its settings.
pub(crate) fn read_text(path: &Path, level: Level) -> Result<(String, Settings), Vec<Error>> {
    let bytes = std::fs::read(path).map_err(|e| vec![io_at(path)(e)])?;
    let text =
        String::from_utf8(bytes).map_err(|_| vec![refuse(Code::SettingsInvalid, Some(path), "line 1: not UTF-8")])?;
    parse(path, &text, level).map(|s| (text, s))
}

/// Validates the text of a file at `level`; `path` names it in every refusal.
pub fn parse(path: &Path, text: &str, level: Level) -> Result<Settings, Vec<Error>> {
    let mut cx = Cx {
        path,
        text,
        out: Settings { path: path.to_path_buf(), level, values: BTreeMap::new(), tables: BTreeMap::new() },
        errors: Vec::new(),
        seen: Default::default(),
    };
    match DeTable::parse(text) {
        Ok(doc) => cx.walk(doc.get_ref(), &mut Vec::new()),
        Err(e) => {
            let span = e.span().unwrap_or(0..0);
            let why = format!("{} (line {}): {}", locate(text, span.clone()), cx.line(span.start), e.message());
            return Err(vec![refuse(Code::SettingsInvalid, Some(path), why)]);
        }
    }
    cx.finish();
    if cx.errors.is_empty() { Ok(cx.out) } else { Err(cx.errors) }
}

/// The key path that text which is not valid TOML is wrong at, resolved by the parser itself: the text at the error is
/// replaced by a key no file holds and read again, and that key's place is the path (a duplicate key or table); if the
/// error is in a value, the text is replaced by `0` and the key holding it is the path; else the last key read before
/// the error; `(document)` when none.
fn locate(text: &str, span: std::ops::Range<usize>) -> String {
    const MARK: &str = "fabrewk-locate-0b5e";
    let Some(at) = text.get(span.clone()) else { return "(document)".into() };
    let keys = |with: &str| {
        let changed = format!("{}{with}{}", &text[..span.start], &text[span.end..]);
        let (doc, _) = DeTable::parse_recoverable(&changed);
        let mut out = Vec::new();
        let mut stack = vec![(doc.get_ref().clone(), Vec::new())];
        while let Some((table, prefix)) = stack.pop() {
            for (k, v) in table.iter() {
                let mut path = prefix.clone();
                path.push(k.get_ref().to_string());
                out.push((path.clone(), v.span(), k.span().start));
                if let DeValue::Table(inner) = v.get_ref() {
                    stack.push((inner.clone(), path));
                }
            }
        }
        out
    };
    let probe = format!("{at} = 0");
    let name =
        DeTable::parse(&probe).ok().and_then(|d| d.get_ref().iter().next().map(|(k, _)| k.get_ref().to_string()));
    if let (Some(name), Some((mut path, ..))) =
        (name, keys(MARK).into_iter().find(|(p, ..)| p.last().map(String::as_str) == Some(MARK)))
    {
        path.pop();
        path.push(name);
        let rest = DeTable::parse(&probe).ok().map(|d| {
            let (mut segs, mut t) = (Vec::new(), d.into_inner());
            while let Some((_, v)) = t.into_iter().next() {
                let DeValue::Table(inner) = v.into_inner() else { break };
                if let Some((k, _)) = inner.iter().next() {
                    segs.push(k.get_ref().to_string());
                }
                t = inner;
            }
            segs
        });
        path.extend(rest.unwrap_or_default());
        return path.join(".");
    }
    let holding = keys("0").into_iter().filter(|(_, v, _)| v.start == span.start).max_by_key(|(p, ..)| p.len());
    // Else the last key read before the error (an unclosed header names its table).
    let before = || keys(at).into_iter().filter(|(.., k)| *k <= span.start).max_by_key(|(p, _, k)| (*k, p.len()));
    let path = holding.or_else(before).map(|(p, ..)| p.join(".")).unwrap_or_default();
    if path.is_empty() { "(document)".into() } else { path }
}

struct Cx<'a> {
    path: &'a Path,
    text: &'a str,
    out: Settings,
    errors: Vec<Error>,
    seen: std::collections::BTreeSet<String>,
}

impl Cx<'_> {
    fn line(&self, offset: usize) -> usize {
        self.text.as_bytes()[..offset.min(self.text.len())].iter().filter(|b| **b == b'\n').count() + 1
    }

    fn err(&mut self, code: Code, key: &str, line: usize, why: &str) {
        self.errors.push(refuse(code, Some(self.path), format!("{key} (line {line}): {why}")));
    }

    fn walk(&mut self, table: &DeTable, at: &mut Vec<String>) {
        for (k, v) in table.iter() {
            at.push(k.get_ref().to_string());
            let segs: Vec<&str> = at.iter().map(String::as_str).collect();
            let path = at.join(".");
            let line = self.line(k.span().start);
            if let Some(key) = KEYS.iter().find(|d| matches(d.path, &segs)) {
                self.leaf(key, &path, v.get_ref(), self.line(v.span().start));
            } else if let Some(named) = table_of(&segs) {
                let name = segs[segs.len() - 1];
                if let DeValue::Table(inner) = v.get_ref() {
                    if named && segs[0] == "maintainers" && !is_alias(name) {
                        self.err(Code::AliasInvalid, &path, line, "not a valid alias, or a reserved name");
                    } else if named && !crate::record::is_name(name) {
                        self.err(Code::SettingsInvalid, &path, line, "not a valid name");
                    } else {
                        self.out.tables.insert(path, line);
                        self.walk(inner, at);
                    }
                } else {
                    self.err(Code::SettingsInvalid, &path, line, "a table is expected");
                }
            } else {
                self.err(Code::SettingsInvalid, &path, line, "unknown key");
            }
            at.pop();
        }
    }

    fn leaf(&mut self, key: &Key, path: &str, v: &DeValue, line: usize) {
        self.seen.insert(path.to_owned());
        if !key.levels.contains(&self.out.level) {
            let code = match self.out.level {
                Level::Personal => Code::SettingsKeyNotPersonal,
                _ => Code::SettingsInvalid,
            };
            return self.err(code, path, line, "not allowed in this file");
        }
        match convert(v).and_then(|value| admit(key.ty, &value).map(|()| value)) {
            Ok(value) => {
                self.out.values.insert(path.to_owned(), Entry { value, line });
            }
            Err((code, why)) => self.err(code, path, line, why),
        }
    }

    fn finish(&mut self) {
        let level = self.out.level;
        for key in KEYS.iter().filter(|k| k.default == Def::Required && k.levels.contains(&level)) {
            for path in instances(key, &self.out.tables) {
                if !self.seen.contains(&path) {
                    let table = path.rsplit_once('.').map_or("", |t| t.0);
                    let line = self.out.tables.get(table).copied().unwrap_or(1);
                    self.err(Code::SettingsInvalid, &path, line, "required");
                }
            }
        }
        let areas: Vec<(String, usize)> =
            self.out.values.iter().filter(|(p, _)| p.ends_with(".area")).map(|(p, e)| (p.clone(), e.line)).collect();
        for (path, line) in areas {
            let role = format!("{}.role", &path[..path.len() - 5]);
            if self.out.values.get(&role).map(|e| &e.value) != Some(&Val::Str("supervisor".into())) {
                self.err(Code::SettingsInvalid, &path, line, "only a supervisor has an area");
            }
        }
        if level == Level::Personal {
            let dir = self.path.parent().and_then(Path::file_name).and_then(|n| n.to_str()).unwrap_or("");
            if let Some(e) = self.out.values.get("maintainer.alias").filter(|e| e.value != Val::Str(dir.into())) {
                let line = e.line;
                self.err(Code::AliasMismatch, "maintainer.alias", line, "differs from the directory's name");
            }
        }
    }
}

/// A TOML value as a [`Val`]; anything else (a float, a date, a table, a list of non-strings) is refused.
pub(crate) fn convert(v: &DeValue) -> Result<Val, (Code, &'static str)> {
    let bad = (Code::SettingsInvalid, "wrong type");
    Ok(match v {
        DeValue::Boolean(b) => Val::Bool(*b),
        DeValue::Integer(i) => Val::Int(i64::from_str_radix(i.as_str(), i.radix()).map_err(|_| bad)?),
        DeValue::String(s) => Val::Str(s.to_string()),
        DeValue::Array(a) => {
            Val::List(a.iter().map(|v| v.get_ref().as_str().map(str::to_owned).ok_or(bad)).collect::<Result<_, _>>()?)
        }
        _ => return Err(bad),
    })
}

/// Whether `value` is of type `ty` and within its allowed set.
pub(crate) fn admit(ty: Type, value: &Val) -> Result<(), (Code, &'static str)> {
    let ok = match (ty, value) {
        (Type::Str, Val::Str(_)) | (Type::Bool, Val::Bool(_)) | (Type::Strs, Val::List(_)) => true,
        (Type::Int, Val::Int(n)) => *n >= 0,
        (Type::Alias, Val::Str(s)) => {
            return if is_alias(s) { Ok(()) } else { Err((Code::AliasInvalid, "not a valid alias")) };
        }
        (Type::Command, Val::List(l)) => l.first().is_some_and(|c| !c.is_empty()),
        (Type::Tag, Val::Str(s)) => is_tag(s),
        (Type::Tags, Val::List(l)) => l.iter().all(|s| is_tag(s)),
        (Type::SignerKey, Val::Str(s)) => is_signer_key(s),
        (Type::Enum(set) | Type::Ordered(set), Val::Str(s)) => set.contains(&s.as_str()),
        _ => return Err((Code::SettingsInvalid, "wrong type")),
    };
    if ok { Ok(()) } else { Err((Code::SettingsInvalid, "value outside the allowed set")) }
}

/// The SSH key types `ssh-keygen -Y` signs with (REQ-ID-5 reports which one a maintainer holds).
pub const KEY_TYPES: [&str; 7] = [
    "ssh-ed25519",
    "ssh-rsa",
    "sk-ssh-ed25519@openssh.com",
    "ecdsa-sha2-nistp256",
    "ecdsa-sha2-nistp384",
    "ecdsa-sha2-nistp521",
    "sk-ecdsa-sha2-nistp256@openssh.com",
];

/// A public key as an allowed-signers line holds it after the principal: `<type> <base64> [comment]`, the base64
/// being a key blob made only of SSH strings, the first one the same type (REQ-ID-1).
fn is_signer_key(s: &str) -> bool {
    if s.chars().any(char::is_control) {
        return false;
    }
    let mut parts = s.split(' ').filter(|p| !p.is_empty());
    let (Some(ty), Some(blob)) = (parts.next(), parts.next()) else {
        return false;
    };
    let bytes = base64(blob).unwrap_or_default();
    KEY_TYPES.contains(&ty) && ssh_strings(&bytes).is_some_and(|s| s.first() == Some(&ty.as_bytes()))
}

/// The SSH strings (a 32-bit length, then the bytes) that make up `b` exactly; `None` if they do not.
pub(crate) fn ssh_strings(mut b: &[u8]) -> Option<Vec<&[u8]>> {
    let mut out = Vec::new();
    while !b.is_empty() {
        let len = u32::from_be_bytes(b.get(..4)?.try_into().ok()?) as usize;
        out.push(b.get(4..4 + len)?);
        b = &b[4 + len..];
    }
    Some(out)
}

/// Decodes standard, padded base64; `None` if it is not.
pub(crate) fn base64(s: &str) -> Option<Vec<u8>> {
    let digit = |c: u8| match c {
        b'A'..=b'Z' => Some(c - b'A'),
        b'a'..=b'z' => Some(c - b'a' + 26),
        b'0'..=b'9' => Some(c - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    };
    let b = s.as_bytes();
    let pad = b.iter().rev().take_while(|c| **c == b'=').count();
    if b.is_empty() || !b.len().is_multiple_of(4) || pad > 2 {
        return None;
    }
    let (mut out, mut acc, mut bits) = (Vec::with_capacity(b.len() / 4 * 3), 0u32, 0);
    for &c in &b[..b.len() - pad] {
        acc = (acc << 6) + u32::from(digit(c)?);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
            acc &= (1 << bits) - 1;
        }
    }
    // The bits left over must be zero, so that one blob has one spelling.
    (acc == 0).then_some(out)
}

#[cfg(test)]
mod tests {
    // Settings at the library's level (part 7c1): every declared key, every refusal with its file, key path and line,
    // the classing of changes and the precedence of levels.

    use super::*;
    use std::collections::BTreeMap;

    const RSA: &str = "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQDDcwAENgdH/zyYbUd3tD5jYGYOuRHZlALoYcEU3VpIz/RCyoVwLyESOa428vi8r5Q+oXQxc/1rPy+l2Rn61+dI8S16CO883Ti46jIXOXSg+nN+9QFxo0HKCRS0lDR/GP2wWTFuhhkU6I+FaFz6H9f/xv+FXJwQzNfeDI/nc7EJ5c9OdEjcak0Tb4Bd6BDRpAe88hAA1u2eG7Yu4dNUsnOjvMqzv2RYBePAtWc2/G47g8mEfby11l+qym+Dsuis/gOk7lEPBWv46r4PtYAHpIq1+p7CrvaIRSkZWopvRGp8dedojby+8AMDZIHsX+DGFp+DykyKdWHK3o8Vh/utJDeN";
    const KEY: &str = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIByTOVQl/Cf6KVagH6iburcFw5dfZyrrEe+CbRpOAb6Y ana";

    fn at(level: Level) -> PathBuf {
        match level {
            Level::Personal => PathBuf::from("/d/.brewstillery/ana/settings.toml"),
            Level::Machine => PathBuf::from("/m/fabrewk/machine.toml"),
            _ => PathBuf::from("/d/.brewstillery/settings.toml"),
        }
    }

    fn ok(level: Level, text: &str) -> Settings {
        parse(&at(level), text, level).unwrap_or_else(|e| panic!("{text}\n{e:?}"))
    }

    fn refusals(level: Level, text: &str) -> Vec<(Code, String)> {
        let errors = parse(&at(level), text, level).expect_err(text);
        errors
            .into_iter()
            .map(|e| match e {
                Error::Refused { code, path, detail } => {
                    assert_eq!(path.as_deref(), Some(at(level).as_path()), "every refusal names the file");
                    (code, detail)
                }
                Error::Io { .. } => panic!("{text}"),
            })
            .collect()
    }

    fn refused(level: Level, text: &str, code: Code, detail: &str) {
        let all = refusals(level, text);
        assert!(
            all.iter().any(|(c, d)| *c == code && d.starts_with(detail)),
            "{text}\nwanted {code:?} {detail}\n{all:?}"
        );
    }

    /// The concrete path of a declared key, `*` read as `ana`.
    fn concrete(key: &Key) -> String {
        key.path.replace('*', "ana")
    }

    #[test]
    fn req_set_2_every_key_is_declared_once_and_whole() {
        let spec = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../.brewstillery/bootstrap/core-specification.md"
        ))
        .unwrap();
        let section = &spec[spec.find("## 8. Settings keys").unwrap()..spec.find("## 9. What phase 5").unwrap()];
        let mut seen = std::collections::BTreeSet::new();
        for key in KEYS {
            assert!(seen.insert(key.path), "{} declared twice", key.path);
            assert!(!key.levels.is_empty() && !key.about.is_empty(), "{}", key.path);
            let last = key.path.rsplit('.').next().unwrap();
            assert!(
                section.contains(&format!("`{}`", key.path.replace("*", "<")).replace("<.", "<"))
                    || section.contains(&format!(".{last}`"))
                    || section.contains(&format!("`{last}`")),
                "{} not in section 8",
                key.path
            );
            match key.default {
                Def::Lit(lit) => {
                    let v = convert(toml::de::DeValue::parse(lit).unwrap().get_ref()).unwrap();
                    admit(key.ty, &v).unwrap();
                }
                Def::From(other) => {
                    assert!(declared(other).is_some_and(|o| !o.path.contains('*')), "{}", key.path)
                }
                Def::Required | Def::Absent => {}
            }
        }
    }

    #[test]
    fn req_set_2_each_key_refuses_a_wrong_type_a_value_outside_its_set_and_a_level_not_its_own() {
        for key in KEYS {
            let path = concrete(key);
            let level = key.levels[0];
            refused(
                level,
                &format!("# one\n# two\n{path} = 1.5\n"),
                Code::SettingsInvalid,
                &format!("{path} (line 3)"),
            );
            let outside = match key.ty {
                Type::Int => Some("-1"),
                Type::Enum(_) | Type::Ordered(_) => Some("\"nope\""),
                Type::Tag => Some("\"not a tag\""),
                Type::Tags => Some("[\"en\", \"not a tag\"]"),
                Type::Command => Some("[]"),
                Type::SignerKey => Some("\"ssh-ed25519 AAAA\""),
                Type::Alias => Some("\"Ana\""),
                Type::Str | Type::Bool | Type::Strs => None,
            };
            if let Some(v) = outside {
                let code = if key.ty == Type::Alias { Code::AliasInvalid } else { Code::SettingsInvalid };
                refused(level, &format!("\n{path} = {v}\n"), code, &format!("{path} (line 2)"));
            }
            for other in [Level::Project, Level::Personal, Level::Machine] {
                if !key.levels.contains(&other) {
                    let code =
                        if other == Level::Personal { Code::SettingsKeyNotPersonal } else { Code::SettingsInvalid };
                    refused(other, &format!("{path} = 1.5\n"), code, &format!("{path} (line 1): not allowed"));
                }
            }
        }
        refused(
            Level::Project,
            "[project]\nname = \"x\"\nnosuch = 1\n",
            Code::SettingsInvalid,
            "project.nosuch (line 3)",
        );
        refused(Level::Project, "[nosuch]\n", Code::SettingsInvalid, "nosuch (line 1)");
        refused(Level::Project, "project = 1\n", Code::SettingsInvalid, "project (line 1)");
        refused(Level::Project, "[tools.\"a.b\"]\n", Code::SettingsInvalid, "tools.a.b (line 1)");
    }

    #[test]
    fn pd_24_a_duplicate_table_and_text_that_is_not_toml_are_refused_with_their_line() {
        refused(
            Level::Project,
            "[project]\nname = \"a\"\n\n[project]\nname = \"b\"\n",
            Code::SettingsInvalid,
            "project (line 4)",
        );
        refused(
            Level::Project,
            "[project]\nname = \"a\"\nname = \"b\"\n",
            Code::SettingsInvalid,
            "project.name (line 3)",
        );
        refused(Level::Project, "[project\n", Code::SettingsInvalid, "project (line 1)");
        refused(Level::Project, "a = 1\na = 2\nb = 3\n", Code::SettingsInvalid, "a (line 2)");
        refused(Level::Project, "[project]\nname = 'x'\n[project]\n", Code::SettingsInvalid, "project (line 3)");
        let between = "[limits]\nrounds = 1\nminutes = 2\nrounds = 3\n";
        refused(Level::Project, between, Code::SettingsInvalid, "limits.rounds (line 4)");
        refused(Level::Project, "[t]\na.b = 1\nc = 2\n\"a\".b = 3\n", Code::SettingsInvalid, "t.a.b (line 4)");
        refused(Level::Project, "[t]\n\"x y\" = 1\nc = 2\n'x y' = 3\n", Code::SettingsInvalid, "t.x y (line 4)");
        let parent = "limits.rounds = 1\nprofiles.p.rounds = 2\nlimits.rounds = 3\n";
        refused(Level::Project, parent, Code::SettingsInvalid, "limits.rounds (line 3)");
        refused(
            Level::Project,
            "[limits]\nminutes = 1\nrounds = minutes\n",
            Code::SettingsInvalid,
            "limits.rounds (line 3)",
        );
        refused(Level::Project, "a = { b = { c = 1, c = 2 } }\n", Code::SettingsInvalid, "a.b.c (line 1)");
        refused(Level::Project, "limits = { rounds = nope }\n", Code::SettingsInvalid, "limits.rounds (line 1)");
        let multiline = "[t]\ns = \"\"\"\n[u]\nk = 0\n\"\"\"\nk = 1\nk = 2\n";
        refused(Level::Project, multiline, Code::SettingsInvalid, "t.k (line 7)");
        refused(Level::Project, "[t]\nk = 1\n[t.k]\n", Code::SettingsInvalid, "t.k (line 3)");
        refused(Level::Project, "[t]\n\"a\" = 1\n\"a\" = 2\n", Code::SettingsInvalid, "t.a (line 3)");
        refused(Level::Project, "'b' = 1\n'b' = 2\n", Code::SettingsInvalid, "b (line 2)");
        refused(
            Level::Project,
            "[project] # note\nname = 'x'\nname = 'y'\n",
            Code::SettingsInvalid,
            "project.name (line 3)",
        );
        refused(
            Level::Project,
            "limits = {\nrounds = 1,\nrounds = 2,\n}\n",
            Code::SettingsInvalid,
            "limits.rounds (line 3)",
        );
        refused(Level::Project, "[a]\nx = 1\n[ \"b\" . c ] # n\n[b.c]\n", Code::SettingsInvalid, "b.c (line 4)");
        refused(Level::Project, "\n\n= 1\n", Code::SettingsInvalid, "(document) (line 3)");
    }

    #[test]
    fn req_set_1_toml_1_1_is_read() {
        // A newline inside an inline table and a trailing comma are TOML 1.1, not 1.0 (D-048).
        let s = ok(Level::Project, "limits = {\n  rounds = 4,\n  minutes = 9,\n}\n[project]\nname = \"x\"\n");
        assert_eq!(s.values["limits.rounds"].value, Val::Int(4));
        assert_eq!(s.values["limits.minutes"].line, 3);
        let personal = ok(Level::Personal, "maintainer = {\n  alias = 'ana',\n}\n");
        assert_eq!(personal.values["maintainer.alias"], Entry { value: Val::Str("ana".into()), line: 2 });
    }

    #[test]
    fn req_set_2_required_keys_are_named_at_their_tables_line() {
        refused(Level::Project, "\n[project]\n", Code::SettingsInvalid, "project.name (line 2): required");
        refused(Level::Project, "", Code::SettingsInvalid, "project.name (line 1): required");
        let profile = "[project]\nname = \"x\"\n[profiles.p]\nkind = \"build\"\n";
        for missing in ["tool", "model", "effort"] {
            refused(
                Level::Project,
                profile,
                Code::SettingsInvalid,
                &format!("profiles.p.{missing} (line 3): required"),
            );
        }
        refused(
            Level::Personal,
            "[environment]\nshell = \"fish\"\n",
            Code::SettingsInvalid,
            "maintainer.alias (line 1)",
        );
    }

    #[test]
    fn req_dom_3_this_repositorys_bootstrap_keys_are_refused_in_the_first_core() {
        let text =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../.brewstillery/settings.toml")).unwrap();
        let all = refusals(Level::Project, &text);
        for key in ["project.command", "project.domain", "maintainers.aliases", "layout"] {
            assert!(
                all.iter().any(|(c, d)| *c == Code::SettingsInvalid && d.starts_with(&format!("{key} ("))),
                "{key}"
            );
        }
        assert_eq!(all.len(), 4, "{all:?}");
    }

    #[test]
    fn req_set_3_personal_keys_are_accepted_and_this_repositorys_personal_file_reads() {
        let text =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../.brewstillery/xel/settings.toml"))
                .unwrap();
        let path = PathBuf::from("/d/.brewstillery/xel/settings.toml");
        let s = parse(&path, &text, Level::Personal).unwrap();
        assert_eq!(s.values["environment.shell"].value, Val::Str("fish".into()));
        let all = "[maintainer]\nalias = \"ana\"\n[languages]\nagentInteraction = \"es-MX\"\ninterfaceLanguage = \"es\"\n\
                   [environment]\nshell = \"zsh\"\n";
        assert_eq!(ok(Level::Personal, all).values.len(), 4);
    }

    #[test]
    fn refusal_settings_key_not_personal() {
        refused(
            Level::Personal,
            "[review]\ndifferentProvider = \"off\"\n",
            Code::SettingsKeyNotPersonal,
            "review.differentProvider (line 2)",
        );
    }

    #[test]
    fn refusal_alias_mismatch() {
        refused(Level::Personal, "[maintainer]\nalias = \"xel\"\n", Code::AliasMismatch, "maintainer.alias (line 2)");
    }

    #[test]
    fn refusal_alias_invalid() {
        let long = "a".repeat(33);
        let mut bad = vec!["Xel", "9a", "a--b", "a-", long.as_str()];
        bad.extend(RESERVED);
        for alias in bad {
            let text =
                format!("[project]\nname = \"x\"\n[maintainers.\"{alias}\"]\nrole = \"maestro\"\nkey = \"{KEY}\"\n");
            refused(Level::Project, &text, Code::AliasInvalid, &format!("maintainers.{alias} (line 3)"));
            refused(
                Level::Personal,
                &format!("[maintainer]\nalias = \"{alias}\"\n"),
                Code::AliasInvalid,
                "maintainer.alias (line 2)",
            );
        }
        for alias in ["xel", "ana-maria", "a1"] {
            ok(
                Level::Project,
                &format!("[project]\nname = \"x\"\n[maintainers.{alias}]\nrole = \"maestro\"\nkey = \"{KEY}\"\n"),
            );
        }
    }

    #[test]
    fn req_id_1_maintainer_tables() {
        let head = "[project]\nname = \"x\"\n[maintainers.ana]\n";
        let s = ok(
            Level::Project,
            &format!("{head}role = \"supervisor\"\nkey = \"{KEY}\"\nhostAccount = \"ana\"\narea = [\"docs/\"]\n"),
        );
        assert_eq!(s.values["maintainers.ana.area"].value, Val::List(vec!["docs/".into()]));
        ok(Level::Project, &format!("{head}role = \"maestro\"\nkey = \"{RSA}\"\n"));
        assert_eq!(s.tables["maintainers.ana"], 3);
        refused(
            Level::Project,
            &format!("{head}key = \"{KEY}\"\n"),
            Code::SettingsInvalid,
            "maintainers.ana.role (line 3): required",
        );
        refused(
            Level::Project,
            &format!("{head}role = \"maestro\"\n"),
            Code::SettingsInvalid,
            "maintainers.ana.key (line 3): required",
        );
        refused(
            Level::Project,
            &format!("{head}role = \"maestro\"\nkey = \"{KEY}\"\narea = []\n"),
            Code::SettingsInvalid,
            "maintainers.ana.area (line 6): only a supervisor",
        );
        for bad in [
            "ssh-rsa AAAAC3NzaC1lZDI1NTE5AAAAIByTOVQl/Cf6KVagH6iburcFw5dfZyrrEe+CbRpOAb6Y",
            "ssh-ed25519 !!!!",
            "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIByTOVQl/Cf6KVagH6iburcFw5dfZyrrEe+CbRpO",
            "ssh-ed25519",
            "",
            "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIByTOVQl/Cf6KVagH6iburcFw5dfZyrrEe+CbRpOAb6YAAAA",
            // One line only: a line break or a tab could carry a second signer.
            "ssh-ed25519\\nAAAAC3NzaC1lZDI1NTE5AAAAIByTOVQl/Cf6KVagH6iburcFw5dfZyrrEe+CbRpOAb6Y",
            "ssh-ed25519\\tAAAAC3NzaC1lZDI1NTE5AAAAIByTOVQl/Cf6KVagH6iburcFw5dfZyrrEe+CbRpOAb6Y",
            "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIByTOVQl/Cf6KVagH6iburcFw5dfZyrrEe+CbRpOAb6Y ana\\nbea ssh-ed25519 AAAA",
        ] {
            refused(
                Level::Project,
                &format!("{head}role = \"maestro\"\nkey = \"{bad}\"\n"),
                Code::SettingsInvalid,
                "maintainers.ana.key (line 5)",
            );
        }
    }

    #[test]
    fn base64_decodes_only_canonical_padded_text() {
        assert_eq!(base64("TWFu").unwrap(), b"Man");
        assert_eq!(base64("TWE=").unwrap(), b"Ma");
        assert_eq!(base64("TQ==").unwrap(), b"M");
        assert_eq!(base64("+/+/").unwrap(), [0xfb, 0xff, 0xbf]);
        for bad in ["", "TWF", "TQ=", "T===", "TR==", "TW=u", "TWF\n", "TW-u"] {
            assert_eq!(base64(bad), None, "{bad}");
        }
    }

    #[test]
    fn req_set_10_language_tags_and_their_fallbacks() {
        for good in [
            "en",
            "es-MX",
            "es-419",
            "zh-Hant-TW",
            "sl-rozaj-biske",
            "de-CH-1901",
            "en-a-bbb-x-a-ccc",
            "zh-yue-HK",
            "x",
        ] {
            assert_eq!(is_tag(good), good != "x", "{good}");
        }
        for good in ["x-private", "X-a-b", "i-klingon", "I-KLINGON", "en-GB-oed", "art-lojban", "zh-min-nan", "EN-us"] {
            assert!(is_tag(good), "{good}");
        }
        for bad in [
            "",
            "e",
            "english-",
            "es_MX",
            "es-MX-",
            "-es",
            "en-a",
            "en-x",
            "en-x-a-123456789",
            "toolongtag",
            "1en",
            "zh-aaa-bbb-ccc-ddd",
            "abcde-yue",
            "de-1901-1901",
            "en-a-foo-a-bar",
            "en-a-foo-A-bar",
            "x-",
            "i-nosuch",
        ] {
            assert!(!is_tag(bad), "{bad}");
        }
        assert_eq!(fallbacks("es-MX"), ["es-MX", "es", "en"]);
        assert_eq!(fallbacks("zh-Hant-TW"), ["zh-Hant-TW", "zh-Hant", "zh", "en"]);
        assert_eq!(fallbacks("en-GB"), ["en-GB", "en"]);
    }

    const BASE: &str = "[project]\nname = \"x\"\n[maintainers.ana]\nrole = \"maestro\"\nkey = \"ssh-ed25519 \
        AAAAC3NzaC1lZDI1NTE5AAAAIByTOVQl/Cf6KVagH6iburcFw5dfZyrrEe+CbRpOAb6Y ana\"\n[profiles.p]\nkind = \"build\"\n\
        tool = \"t\"\nmodel = \"m\"\neffort = \"high\"\n[tools.t]\nadapter = \"codex\"\nproviders = [\"o\"]\n[checks.c]\n\
        command = [\"true\"]\n[models.m]\nprovider = \"o\"\nid = \"m1\"\nefforts = [\"high\"]\n[providers.o]\n\
        [extensions.e]\n";

    /// [`BASE`] with `path` set to `value`, in its table, replacing the line the base holds.
    fn with(path: &str, value: &str) -> String {
        let (table, last) = path.rsplit_once('.').unwrap();
        let header = format!("[{table}]\n");
        let Some(at) = BASE.find(&header) else {
            return format!("{BASE}{header}{last} = {value}\n");
        };
        let (head, rest) = BASE.split_at(at + header.len());
        let end = rest.find("\n[").map_or(rest.len(), |i| i + 1);
        let body: String =
            rest[..end].lines().filter(|l| !l.starts_with(&format!("{last} ="))).map(|l| format!("{l}\n")).collect();
        format!("{head}{last} = {value}\n{body}{}", &rest[end..])
    }

    fn change(old: &str, new: &str) -> Vec<(String, Effect)> {
        let (a, b) = (ok(Level::Project, &format!("{BASE}{old}")), ok(Level::Project, &format!("{BASE}{new}")));
        changes(&a, &b).into_iter().map(|c| (c.key, c.effect)).collect()
    }

    #[test]
    fn req_set_5_every_key_is_classed_each_way_by_its_direction() {
        use Effect::*;
        let other = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIJtneqO20aGvL2mzi8YOPtPl8igyd+t7J6aVlFIR85vw bea";
        for key in KEYS.iter().filter(|k| k.levels.contains(&Level::Project)) {
            let name = match key.path.split('.').next() {
                Some("maintainers") => "ana",
                Some("profiles") => "p",
                Some("tools") => "t",
                Some("checks") => "c",
                Some("models") => "m",
                Some("providers") => "o",
                _ => "e",
            };
            let path = key.path.replace('*', name);
            let quote = |s: &str| format!("\"{s}\"");
            let (small, large) = match key.ty {
                Type::Int => ("1".into(), "2".into()),
                Type::Bool => ("false".into(), "true".into()),
                Type::Strs | Type::Command => ("[\"a\"]".into(), "[\"a\", \"b\"]".into()),
                Type::Tags => ("[\"en\"]".into(), "[\"en\", \"es\"]".into()),
                Type::Enum(set) | Type::Ordered(set) => (quote(set[0]), quote(set[set.len() - 1])),
                Type::Tag => (quote("en"), quote("es")),
                Type::SignerKey => (quote(KEY), quote(other)),
                Type::Str | Type::Alias => (quote("a"), quote("b")),
            };
            let role = path.ends_with(".role");
            let fix = |t: String| {
                if path.ends_with(".area") { t.replace("\"maestro\"", "\"supervisor\"") } else { t }
            };
            let (a, b) = (ok(Level::Project, &fix(with(&path, &small))), ok(Level::Project, &fix(with(&path, &large))));
            let comparable =
                matches!(key.ty, Type::Int | Type::Bool | Type::Ordered(_) | Type::Strs | Type::Command | Type::Tags);
            let (up, down) = match key.dir {
                _ if path.starts_with("maintainers.") => (Widening, Widening),
                Dir::Neutral => (Neutral, Neutral),
                _ if !comparable => (Widening, Widening),
                Dir::Widens => (Widening, Narrowing),
                Dir::Narrows => (Narrowing, Widening),
            };
            let effects =
                |x: &Settings, y: &Settings| changes(x, y).into_iter().map(|c| (c.key, c.effect)).collect::<Vec<_>>();
            let mut want_up = vec![(path.clone(), up)];
            let mut want_down = vec![(path.clone(), down)];
            if role {
                // contributor → supervisor: the role changes, and nothing else.
                want_up.truncate(1);
                want_down.truncate(1);
            }
            assert_eq!(effects(&a, &b), want_up, "{path} up");
            assert_eq!(effects(&b, &a), want_down, "{path} down");
        }
    }

    #[test]
    fn req_set_5_tables_and_maintainers() {
        use Effect::*;
        let bea = format!("[maintainers.bea]\nrole = \"contributor\"\nkey = \"{KEY}\"\n");
        assert_eq!(change("", &bea), [("maintainers.bea".into(), Widening)]);
        assert_eq!(change(&bea, ""), [("maintainers.bea".into(), Widening)], "removing a maintainer widens");
        let tool = "[tools.u]\nadapter = \"codex\"\nproviders = []\n";
        assert_eq!(change("", tool), [("tools.u".into(), Widening)]);
        assert_eq!(change(tool, ""), [("tools.u".into(), Narrowing)]);
        assert!(change("", "[limits]\nrounds = 3\n").is_empty(), "a default written out is no change");
        assert_eq!(change("[limits]\nrounds = 4\n", ""), [("limits.rounds".into(), Narrowing)]);
        let absent = ok(Level::Project, &with("maintainers.ana.role", "\"supervisor\""));
        let present =
            ok(Level::Project, &with("maintainers.ana.area", "[\"docs/\"]").replace("\"maestro\"", "\"supervisor\""));
        assert_eq!(changes(&present, &absent)[0].effect, Widening, "an area removed is the whole project");
        let order = |a: &str, b: &str| {
            let (x, y) = (with("review.differentProvider", a), with("review.differentProvider", b));
            changes(&ok(Level::Project, &x), &ok(Level::Project, &y))[0].effect
        };
        assert_eq!(order("\"require\"", "\"prefer\""), Widening);
        assert_eq!(order("\"prefer\"", "\"require\""), Narrowing);
        assert_eq!(order("\"prefer\"", "\"off\""), Widening);
        // A key that narrows and has no value is at its widest too (none is declared yet; the rule is general).
        let narrows = Key {
            path: "x.y",
            ty: Type::Strs,
            levels: &[Level::Project],
            default: Def::Absent,
            dir: Dir::Narrows,
            about: "",
        };
        let some = Val::List(vec!["a".into()]);
        assert_eq!(policy::effect(&narrows, "x.y", None, Some(&some)), Narrowing);
        assert_eq!(policy::effect(&narrows, "x.y", Some(&some), None), Widening);
        let tokens = |v: &str| ok(Level::Project, &with("profiles.p.tokens", v));
        let base = ok(Level::Project, BASE);
        assert_eq!(changes(&tokens("5"), &base)[0].effect, Widening, "tokens removed are unlimited");
        assert_eq!(changes(&base, &tokens("5"))[0].effect, Narrowing);
    }

    fn policy(extra: &str) -> Policy {
        let text = format!("{BASE}{extra}");
        Policy { digest: crate::record::sha256_hex(text.as_bytes()), settings: ok(Level::Project, &text), text }
    }

    fn layer(source: Source, values: &[(&str, Val)]) -> (Source, BTreeMap<String, Val>) {
        (source, values.iter().map(|(k, v)| ((*k).to_owned(), v.clone())).collect())
    }

    #[test]
    fn req_set_7_every_value_has_one_source() {
        let p = policy("[limits]\nrounds = 4\n");
        let personal =
            ok(Level::Personal, "[maintainer]\nalias = \"ana\"\n[languages]\nagentInteraction = \"es-MX\"\n");
        let stage = layer(Source::Stage("s-00000001".into()), &[("limits.dispatches", Val::Int(2))]);
        let task = layer(Source::Task("t-00000002".into()), &[("limits.minutes", Val::Int(60))]);
        let eff = effective(&p, Some(&personal), &[stage, task], &[]).unwrap();
        let source = |k: &str| lookup(&eff, k).unwrap().unwrap().1.clone();
        assert_eq!(source("limits.rounds"), Source::Project(p.digest.clone()));
        assert_eq!(source("limits.dispatches"), Source::Stage("s-00000001".into()));
        assert_eq!(source("limits.minutes"), Source::Task("t-00000002".into()));
        assert_eq!(source("languages.agentInteraction"), Source::Personal("ana".into()));
        assert_eq!(source("languages.canonicalDocumentationLanguage"), Source::Default);
        assert_eq!(lookup(&eff, "languages.interfaceLanguage").unwrap().unwrap().0, Val::Str("es-MX".into()));
        assert_eq!(lookup(&eff, "languages.documentationLanguages").unwrap().unwrap().0, Val::List(vec!["en".into()]));
        assert_eq!(
            lookup(&eff, "profiles.p.rounds").unwrap().unwrap(),
            &(Val::Int(4), Source::Default),
            "follows limits"
        );
        assert_eq!(lookup(&eff, "profiles.p.minutes").unwrap().unwrap().0, Val::Int(60), "follows the task's limit");
        assert_eq!(lookup(&eff, "maintainers.ana.hostAccount").unwrap(), None);
        assert_eq!(lookup(&eff, "nosuch.key").unwrap_err().code(), Some(Code::KeyUnknown));
        assert_eq!(lookup(&eff, "maintainers.ana").unwrap_err().code(), Some(Code::KeyUnknown));
    }

    #[test]
    fn req_set_6_layers_may_not_set_personal_keys_or_open_tables() {
        let p = policy("");
        // REQ-SET-2: every key refused at each layer it may not be set at.
        for key in KEYS {
            let path = key.path.replace("maintainers.*", "maintainers.ana").replace('*', "p");
            for (source, level) in
                [(Source::Stage("s-1".into()), Level::Stage), (Source::Task("t-1".into()), Level::Task)]
            {
                let l = layer(source, &[(&path, Val::Int(1))]);
                let r = effective(&p, None, &[l], std::slice::from_ref(&path));
                if !key.levels.contains(&level) {
                    assert_eq!(r.unwrap_err().code(), Some(Code::SettingsInvalid), "{path} at {level:?}");
                }
            }
        }
        for (key, value) in [
            ("languages.agentInteraction", Val::Str("en".into())),
            ("profiles.q.model", Val::Str("m".into())),
            ("project.name", Val::Str("y".into())),
            ("limits.rounds", Val::Str("y".into())),
            ("nosuch", Val::Int(1)),
        ] {
            let l = layer(Source::Stage("s-1".into()), &[(key, value)]);
            assert_eq!(
                effective(&p, None, &[l], &[key.into()]).unwrap_err().code(),
                Some(Code::SettingsInvalid),
                "{key}"
            );
        }
    }

    mod property {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            /// REQ-SET-6: the effective value is the last level that set it; a raise by a stage or a task is a widening,
            /// refused unless named, then listed; personal values are untouched.
            #[test]
            fn req_set_6_precedence(project in proptest::option::of(0i64..9), stage in proptest::option::of(0i64..9),
                                    task in proptest::option::of(0i64..9), named in any::<bool>()) {
                let p = policy(&project.map(|v| format!("[limits]\nrounds = {v}\n")).unwrap_or_default());
                let personal = ok(Level::Personal, "[maintainer]\nalias = \"ana\"\n");
                let mut layers = Vec::new();
                if let Some(v) = stage { layers.push(layer(Source::Stage("s-1".into()), &[("limits.rounds", Val::Int(v))])); }
                if let Some(v) = task { layers.push(layer(Source::Task("t-1".into()), &[("limits.rounds", Val::Int(v))])); }
                let widen = if named { vec!["limits.rounds".to_owned()] } else { vec![] };
                let mut level = project.unwrap_or(3);
                let mut raised = false;
                for v in [stage, task].into_iter().flatten() {
                    raised |= v > level;
                    level = v;
                }
                match effective(&p, Some(&personal), &layers, &widen) {
                    Ok(eff) => {
                        prop_assert!(named || !raised);
                        prop_assert_eq!(&eff.values["limits.rounds"].0, &Val::Int(level));
                        prop_assert_eq!(eff.widened.is_empty(), !raised);
                        prop_assert_eq!(&eff.values["maintainer.alias"].1, &Source::Personal("ana".into()));
                    }
                    Err(e) => {
                        prop_assert!(raised && !named);
                        prop_assert_eq!(e.code(), Some(Code::WideningUnnamed));
                    }
                }
            }
        }
    }
}
