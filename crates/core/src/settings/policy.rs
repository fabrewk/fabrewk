//! The policy in force and the effective configuration (REQ-SET-4 to REQ-SET-7, REQ-ID-3's refusal at adoption). The
//! policy is the project file's content carried by the last `policy-adopted` event; the file on disk is a proposal
//! until a maestro adopts it, and every widening it holds must be named.

use super::{Def, Dir, KEYS, Key, Level, Settings, Type, Val, admit, convert, declared, instances, table_of};
use crate::code::{Code, Error, refuse};
use crate::record::{Replay, Repo, sha256_hex};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// The policy in force: the adopted text, its digest, and its settings.
#[derive(Debug, Clone)]
pub struct Policy {
    pub digest: String,
    pub text: String,
    pub settings: Settings,
}

/// The policy carried by the last `policy-adopted` of the `project` stream; `None` while none is adopted.
pub fn in_force(record: &Replay) -> Result<Option<Policy>, Vec<Error>> {
    let last = record.streams.get("project").and_then(|s| s.iter().rev().find(|e| e.kind == "policy-adopted"));
    let Some(event) = last else { return Ok(None) };
    // The event's check (schema 1) already holds the content to its digest.
    let text = event.value["payload"]["content"].as_str().unwrap_or_default().to_owned();
    let settings = super::parse(&event.path, &text, Level::Project)?;
    Ok(Some(Policy { digest: sha256_hex(text.as_bytes()), text, settings }))
}

/// The policy, or `POLICY_ABSENT` for a command that needs one.
pub fn require(policy: Option<Policy>, domain: &Path) -> Result<Policy, Error> {
    policy.ok_or_else(|| refuse(Code::PolicyAbsent, Some(domain), "no policy is adopted"))
}

/// What a change does to authority (REQ-SET-5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Widening,
    Narrowing,
    Neutral,
}

/// One difference between two project files: a key, or a whole named table (`maintainers.ana`); `line` is its line in
/// the new file, `was` its line in the old one, so a removal is located too.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    pub key: String,
    pub effect: Effect,
    pub old: Option<Val>,
    pub new: Option<Val>,
    pub line: Option<usize>,
    pub was: Option<usize>,
}

impl Change {
    /// Where the change is: its line in the file, or, for a removal, its line in the adopted policy.
    pub fn place(&self) -> String {
        match (self.line, self.was) {
            (Some(l), _) => format!("{} (line {l})", self.key),
            (None, Some(w)) => format!("{} (removed; line {w} of the policy in force)", self.key),
            (None, None) => self.key.clone(),
        }
    }
}

/// The effect of changing `path` from `old` to `new`, which differ (both callers skip equal values). Any change to a maintainer widens (REQ-SET-5); a key with no
/// value is at its widest (an absent area is the whole project, absent tokens are unlimited); a change that is
/// neither larger nor smaller (a list both gaining and losing, a name) widens a key that has a direction.
pub(super) fn effect(key: &Key, path: &str, old: Option<&Val>, new: Option<&Val>) -> Effect {
    match (key.dir, old, new) {
        _ if path.starts_with("maintainers.") => return Effect::Widening,
        (Dir::Neutral, ..) => return Effect::Neutral,
        // From no value (the widest) to one narrows; the way back has no order, and so widens below.
        (_, None, _) => return Effect::Narrowing,
        _ => {}
    }
    let larger = match (old, new) {
        (Some(Val::Bool(a)), Some(Val::Bool(b))) => Some(b > a),
        (Some(Val::Int(a)), Some(Val::Int(b))) => Some(b > a),
        (Some(Val::Str(a)), Some(Val::Str(b))) => match key.ty {
            Type::Ordered(set) => Some(set.iter().position(|s| s == b) > set.iter().position(|s| s == a)),
            _ => None,
        },
        (Some(Val::List(a)), Some(Val::List(b))) => {
            let (added, removed) = (b.iter().any(|x| !a.contains(x)), a.iter().any(|x| !b.contains(x)));
            (added != removed).then_some(added)
        }
        _ => None,
    };
    match (key.dir, larger) {
        (Dir::Widens, Some(true)) | (Dir::Narrows, Some(false)) | (_, None) => Effect::Widening,
        _ => Effect::Narrowing,
    }
}

/// Where an effective value comes from (REQ-SET-7). `Project` carries the adopted digest; a value no level set is
/// `Default`, also when its default is another key's value (a profile's limit follows `limits`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    Default,
    Project(String),
    Personal(String),
    Stage(String),
    Task(String),
}

type Values = BTreeMap<String, (Val, Source)>;

/// Adds each default that no level set: literals first, then the keys that take another key's value, in order.
fn fill(map: &mut Values, tables: &BTreeMap<String, usize>, levels: &[Level]) {
    for derived in [false, true] {
        for key in KEYS.iter().filter(|k| k.levels.iter().any(|l| levels.contains(l))) {
            for path in instances(key, tables) {
                if map.get(&path).is_some_and(|(_, s)| *s != Source::Default) {
                    continue;
                }
                let value = match (key.default, derived) {
                    (Def::Lit(lit), false) => {
                        let v = toml::de::DeValue::parse(lit).ok().and_then(|v| convert(v.get_ref()).ok());
                        v.expect("every literal default converts (tested)")
                    }
                    (Def::From(other), true) => match (map.get(other), key.ty) {
                        (Some((Val::Str(s), _)), Type::Tags) => Val::List(vec![s.clone()]),
                        (Some((v, _)), _) => v.clone(),
                        (None, _) => continue,
                    },
                    _ => continue,
                };
                map.insert(path, (value, Source::Default));
            }
        }
    }
}

fn is_named(table: &str) -> bool {
    table_of(&table.split('.').collect::<Vec<_>>()) == Some(true)
}

/// Every difference between the policy `old` and the file `new`, each classed by its key's direction (REQ-SET-5).
/// Adding a named table widens and removing one narrows, but a maintainer's removal widens; a value that only
/// follows another key (a profile's limit from `limits`) is not a change of its own.
pub fn changes(old: &Settings, new: &Settings) -> Vec<Change> {
    let named = |s: &Settings| s.tables.keys().filter(|t| is_named(t)).cloned().collect::<BTreeSet<_>>();
    let (a, b) = (named(old), named(new));
    let moved: Vec<&String> = a.symmetric_difference(&b).collect();
    let mut out: Vec<Change> = moved
        .iter()
        .map(|t| {
            let added = b.contains(*t);
            let effect = if added || t.starts_with("maintainers.") { Effect::Widening } else { Effect::Narrowing };
            Change {
                key: (*t).clone(),
                effect,
                old: None,
                new: None,
                line: new.tables.get(*t).copied(),
                was: old.tables.get(*t).copied(),
            }
        })
        .collect();
    let values = |s: &Settings| {
        let set = |(p, e): (&String, &super::Entry)| (p.clone(), (e.value.clone(), Source::Project(String::new())));
        let mut map: Values = s.values.iter().map(set).collect();
        fill(&mut map, &s.tables, &[s.level]);
        map.into_iter().map(|(p, (v, _))| (p, v)).collect::<BTreeMap<_, _>>()
    };
    let (va, vb) = (values(old), values(new));
    for path in va.keys().chain(vb.keys()).collect::<BTreeSet<_>>() {
        let key = declared(path).expect("a value's path is declared");
        let follows =
            matches!(key.default, Def::From(_)) && !old.values.contains_key(path) && !new.values.contains_key(path);
        let (x, y) = (va.get(path), vb.get(path));
        if x == y || follows || moved.iter().any(|t| path.starts_with(&format!("{t}."))) {
            continue;
        }
        let (line, was) = (new.values.get(path).map(|e| e.line), old.values.get(path).map(|e| e.line));
        out.push(Change {
            key: path.clone(),
            effect: effect(key, path, x, y),
            old: x.cloned(),
            new: y.cloned(),
            line,
            was,
        });
    }
    out
}

/// The project file's changes against the policy in force, for `config` and `check` (`POLICY_PENDING`); `None` when
/// the file is the policy byte for byte. Nothing uses a pending change.
pub fn pending(policy: &Policy, domain: &Path) -> Result<Option<Vec<Change>>, Vec<Error>> {
    let (text, file) = super::read_text(&domain.join("settings.toml"), Level::Project)?;
    Ok((sha256_hex(text.as_bytes()) != policy.digest).then(|| changes(&policy.settings, &file)))
}

/// What `adopt` would record: the file's content and digest, its changes against the policy in force, and the
/// widenings named.
#[derive(Debug, Clone)]
pub struct Adoption {
    pub content: String,
    pub digest: String,
    pub settings: Settings,
    pub changes: Vec<Change>,
    pub widened: Vec<String>,
}

impl Adoption {
    /// The `policy-adopted` payload: `content`, `digest` (its SHA-256) and `widened` (the widenings named).
    pub fn payload(&self) -> Value {
        json!({"content": self.content, "digest": self.digest, "widened": self.widened})
    }
}

/// Checks the project file under `domain` for adoption (REQ-SET-4, REQ-SET-5, REQ-ID-3): valid, listing no
/// supervisor (`ROLE_NOT_ENFORCED`), committed at the record branch's head when the mode commits (`repo`;
/// `POLICY_UNCOMMITTED`), and every widening against `in_force` named in `widen` (`WIDENING_UNNAMED`). With no policy
/// in force there is nothing to widen: the first adoption is `apply`'s, whose plan the person accepts whole.
pub fn adopt(
    domain: &Path,
    in_force: Option<&Policy>,
    widen: &[String],
    repo: Option<&Repo>,
) -> Result<Adoption, Vec<Error>> {
    let path = domain.join("settings.toml");
    let (content, settings) = super::read_text(&path, Level::Project)?;
    let mut errors = Vec::new();
    for (key, e) in settings.values.iter().filter(|(k, e)| {
        k.starts_with("maintainers.") && k.ends_with(".role") && e.value == Val::Str("supervisor".into())
    }) {
        let why = format!("{key} (line {}): the supervisor's role is not enforced yet", e.line);
        errors.push(refuse(Code::RoleNotEnforced, Some(&path), why));
    }
    if let Some(repo) = repo {
        let committed = crate::record::committed(repo, Path::new("settings.toml")).map_err(|e| vec![e])?;
        if committed.as_deref() != Some(content.as_bytes()) {
            errors.push(refuse(Code::PolicyUncommitted, Some(&path), "not as committed at the record branch's head"));
        }
    }
    let changes = in_force.map(|p| changes(&p.settings, &settings)).unwrap_or_default();
    let widenings: Vec<&Change> = changes.iter().filter(|c| c.effect == Effect::Widening).collect();
    let unnamed: Vec<String> = widenings.iter().filter(|c| !widen.contains(&c.key)).map(|c| c.place()).collect();
    if !unnamed.is_empty() {
        let why = format!("widenings not named with --widen: {}", unnamed.join(", "));
        errors.push(refuse(Code::WideningUnnamed, Some(&path), why));
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    let widened = widenings.iter().map(|c| c.key.clone()).collect();
    Ok(Adoption { digest: sha256_hex(content.as_bytes()), content, settings, changes, widened })
}

/// The effective configuration: every value with its source, and the widenings the stage and task layers made.
#[derive(Debug, Clone)]
pub struct Effective {
    pub values: BTreeMap<String, (Val, Source)>,
    pub widened: Vec<String>,
}

/// The effective values (REQ-SET-6): defaults, then the policy, then each layer in order (a stage's values, then a
/// task's, each `(Source::Stage(id) or Source::Task(id), values)`); `personal` sets only personal keys, which no layer
/// may set. A layer may narrow freely; a widening must be named in `widen` (`WIDENING_UNNAMED`) and is listed.
pub fn effective(
    policy: &Policy,
    personal: Option<&Settings>,
    layers: &[(Source, BTreeMap<String, Val>)],
    widen: &[String],
) -> Result<Effective, Error> {
    let tables = &policy.settings.tables;
    let mut map: Values = BTreeMap::new();
    for (path, e) in &policy.settings.values {
        map.insert(path.clone(), (e.value.clone(), Source::Project(policy.digest.clone())));
    }
    if let Some(p) = personal {
        let alias = match p.values.get("maintainer.alias").map(|e| &e.value) {
            Some(Val::Str(a)) => a.clone(),
            _ => String::new(),
        };
        for (path, e) in &p.values {
            map.insert(path.clone(), (e.value.clone(), Source::Personal(alias.clone())));
        }
    }
    let levels = [Level::Project, Level::Personal, Level::Stage, Level::Task];
    fill(&mut map, tables, &levels);
    let mut widened = Vec::new();
    for (source, values) in layers {
        let (level, id) = match source {
            Source::Stage(id) => (Level::Stage, id),
            Source::Task(id) => (Level::Task, id),
            _ => {
                return Err(refuse(Code::SettingsInvalid, None, "a layer is a stage's or a task's"));
            }
        };
        for (path, value) in values {
            let bad = |why: &str| refuse(Code::SettingsInvalid, None, format!("{path} ({id}): {why}"));
            let key = declared(path).filter(|k| k.levels.contains(&level)).ok_or_else(|| bad("not allowed here"))?;
            if !instances(key, tables).contains(path) {
                return Err(bad("no such table in the policy"));
            }
            admit(key.ty, value).map_err(|(_, why)| bad(why))?;
            let below = map.get(path).map(|(v, _)| v);
            if below != Some(value) && effect(key, path, below, Some(value)) == Effect::Widening {
                if !widen.contains(path) {
                    let why = format!("{path} ({id}): a widening, not named with --widen");
                    return Err(refuse(Code::WideningUnnamed, None, why));
                }
                widened.push(path.clone());
            }
            map.insert(path.clone(), (value.clone(), source.clone()));
        }
        fill(&mut map, tables, &levels);
    }
    Ok(Effective { values: map, widened })
}

/// One effective value, for `config <key>`: `KEY_UNKNOWN` if no key is declared at that path; `None` if it is
/// declared and has no value (an absent `hostAccount`, a table that does not exist).
pub fn lookup<'a>(eff: &'a Effective, key: &str) -> Result<Option<&'a (Val, Source)>, Error> {
    declared(key).ok_or_else(|| refuse(Code::KeyUnknown, None, key.to_owned()))?;
    Ok(eff.values.get(key))
}
