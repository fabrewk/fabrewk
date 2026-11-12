//! The tool adapter's contract (REQ-RUN-7, REQ-RUN-8, REQ-SEC-1 for the adapter, REQ-SEC-7, REQ-GATE-4's audit;
//! `confinement-design.md` 3.6 and 3.9; part 9c). The core names no tool (PB-8): each built-in module declares an
//! [`Adapter`], and the core builds from it the cell of each role, the packet, the role's result schema and its
//! validation, the usage, and the preflight. What a stream means is the adapter's: one grammar per tool, read only
//! once its cell has ended (3.8 rules 1 and 8), and the audit of what a reviewer ran (PD-26).

use crate::alambique;
use crate::cli::say;
use crate::code::{Code, Error, io_at, refuse};
use crate::confine::{self, Access, Backend, Cell, End, Limits, Mount, Network, Programs, Ran, Scratch, Unfinished};
use crate::detect::Program;
use crate::record::Replay;
use serde_json::{Value, json};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// The roles an agent is dispatched in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Build,
    Review,
}

impl Role {
    pub const ALL: [Role; 2] = [Role::Build, Role::Review];

    pub const fn name(self) -> &'static str {
        match self {
            Role::Build => "build",
            Role::Review => "review",
        }
    }
}

/// Tokens as the tool reports them (REQ-RUN-7, S-5): each unknown, never zero, when not reported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Tokens {
    pub input: Option<u64>,
    pub output: Option<u64>,
    pub cached: Option<u64>,
}

/// What an adapter reads in a run's standard output by its tool's one grammar: the role's result, only when the
/// stream ends in one successful final result; the model the stream says answered; the tokens it reports; and how
/// many examinations of the copy its audit counts, zero whenever an anomaly voids the stream (PD-26).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Report {
    pub result: Option<Value>,
    pub model: Option<String>,
    pub tokens: Option<Tokens>,
    pub examined: u64,
}

/// One dispatch, as the adapter turns it into its tool's command.
#[derive(Debug, Clone, Copy)]
pub struct Launch<'a> {
    pub role: Role,
    pub packet: &'a str,
    pub schema: &'a str,
    pub model: &'a str,
    pub effort: &'a str,
}

/// A tool adapter (AD-5's built-in module): everything the core needs to detect, confine, launch and read one tool.
pub struct Adapter {
    /// How the tool is detected (D-051); its `adapter` is the name `tools.<name>.adapter` gives.
    pub program: Program,
    /// The provider's identity, as the adapter declares it (REQ-GATE-4).
    pub provider: &'static str,
    /// The environment variables that show a process runs inside this tool (REQ-SEC-7).
    pub markers: &'static [&'static str],
    /// The names of the tool's instruction and configuration files, protected at any depth (S-4); each is in
    /// `gate.protectedNames`' default.
    pub protected: &'static [&'static str],
    /// Catalogue keys `residual.<key>` that `config` lists (S-7).
    pub residual: &'static [&'static str],
    /// Its machine rule (6.38, 3.9): the names that may not be found at any depth of the launch directory, under ASCII
    /// case folding, checked at every dispatch before any process starts.
    pub machine: &'static [&'static str],
    /// The tool's credential, relative to the person's home; bound read-only at the same place under the cell's home
    /// (3.6), never copied.
    pub credential: &'static str,
    /// The variables the tool runs with, beyond the cell's own.
    pub env: &'static [(&'static str, &'static str)],
    /// The tool's release directory, from its executable as it resolves; `None` when it cannot be told.
    pub release: fn(&Path) -> Option<PathBuf>,
    /// The command of one dispatch, from the executable as it resolves.
    pub command: fn(&Path, &Launch) -> Vec<OsString>,
    /// The preflight's commands (REQ-SEC-1, PD-27), each to end with status 0 in each role's cell with no network
    /// and no credential: the tool's version, its helper programs, and its own sandbox creating a temporary file.
    pub preflight: fn(&Path) -> Vec<Vec<OsString>>,
    /// Reads a finished run's standard output.
    pub read: fn(&[u8]) -> Report,
}

/// One JSON value from `text`, or `None` when it is not one or an object in it names a member twice, of which
/// `serde_json` would keep only the last (P9D-1): an adapter reads its stream's evidence as written, once.
pub fn json(text: &str) -> Option<Value> {
    serde_json::from_str::<Strict>(text).ok().map(|s| s.0)
}

/// A JSON value read by [`json`].
struct Strict(Value);

impl<'de> serde::Deserialize<'de> for Strict {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Strict, D::Error> {
        d.deserialize_any(Strict(Value::Null))
    }
}

impl<'de> serde::de::Visitor<'de> for Strict {
    type Value = Strict;
    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("a JSON value whose objects name each member once")
    }
    fn visit_bool<E>(self, v: bool) -> Result<Strict, E> {
        Ok(Strict(v.into()))
    }
    fn visit_i64<E>(self, v: i64) -> Result<Strict, E> {
        Ok(Strict(v.into()))
    }
    fn visit_u64<E>(self, v: u64) -> Result<Strict, E> {
        Ok(Strict(v.into()))
    }
    fn visit_f64<E>(self, v: f64) -> Result<Strict, E> {
        Ok(Strict(v.into()))
    }
    fn visit_str<E>(self, v: &str) -> Result<Strict, E> {
        Ok(Strict(v.into()))
    }
    fn visit_unit<E>(self) -> Result<Strict, E> {
        Ok(Strict(Value::Null))
    }
    fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<Strict, A::Error> {
        let mut out = Vec::new();
        while let Some(Strict(v)) = seq.next_element()? {
            out.push(v);
        }
        Ok(Strict(Value::Array(out)))
    }
    fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Strict, A::Error> {
        let mut out = serde_json::Map::new();
        while let Some(key) = map.next_key::<String>()? {
            let Strict(v) = map.next_value()?;
            if out.insert(key, v).is_some() {
                return Err(serde::de::Error::custom("a member named twice"));
            }
        }
        Ok(Strict(Value::Object(out)))
    }
}

/// A tool found on this machine: its adapter, its executable and its release directory, both as they resolve.
#[derive(Clone, Copy)]
pub struct Tool<'a> {
    pub adapter: &'a Adapter,
    pub executable: &'a Path,
    pub release: &'a Path,
}

/// The executable at `found` and its release directory, both resolved. A release that cannot be told, that does not
/// hold the executable, or that holds any of `outside` (the person's home, the repository), which a cell would then
/// show whole, is `TOOL_MISSING`.
pub fn resolve(adapter: &Adapter, found: &Path, outside: &[&Path]) -> Result<(PathBuf, PathBuf), Error> {
    let missing = |why: &str| refuse(Code::ToolMissing, Some(found), format!("{}: {why}", adapter.program.executable));
    let exe = found.canonicalize().map_err(|_| missing("its executable does not resolve"))?;
    let release = (adapter.release)(&exe).ok_or_else(|| missing("its release directory cannot be told"))?;
    let release = release.canonicalize().map_err(|_| missing("its release directory does not resolve"))?;
    if !exe.starts_with(&release) {
        return Err(missing("its executable lies outside its release directory"));
    }
    // A place that does not resolve is taken as written.
    if outside.iter().any(|o| o.canonicalize().unwrap_or_else(|_| o.to_path_buf()).starts_with(&release)) {
        return Err(missing("its release directory holds the person's home or the repository"));
    }
    Ok((exe, release))
}

/// Where a dispatch's cell holds the role's output schema, for a tool that reads it from a file: in the cell's home,
/// which the core writes before the tool starts.
pub const SCHEMA: &str = "/home/agent/schema.json";

/// The variables every agent's cell holds (3.3).
const CELL_ENV: [(&str, &str); 4] =
    [("PATH", "/usr/bin:/bin"), ("HOME", "/home/agent"), ("TMPDIR", "/tmp"), ("LANG", "C.UTF-8")];

/// The cell of a role (3.4): the copy writable at `/work`, the release read-only at its own path, nested namespaces
/// for the tool's own sandbox; with the credential, bound read-only, a private outbound network; without it, none.
fn cell(tool: Tool, scratch: &Scratch, argv: Vec<OsString>, credential: Option<&Path>) -> Cell {
    let mut mounts = vec![
        Mount { host: tool.release.to_path_buf(), inside: tool.release.to_path_buf(), access: Access::ReadOnly },
        Mount { host: scratch.work.clone(), inside: "/work".into(), access: Access::ReadWrite },
    ];
    if let Some(file) = credential {
        let inside = Path::new("/home/agent").join(tool.adapter.credential);
        mounts.push(Mount { host: file.to_path_buf(), inside, access: Access::ReadOnly });
    }
    let env = CELL_ENV.iter().chain(tool.adapter.env).map(|(k, v)| (k.into(), v.into())).collect();
    Cell {
        home: scratch.home.clone(),
        tmp: scratch.tmp.clone(),
        mounts,
        network: if credential.is_some() { Network::Outbound } else { Network::None },
        nested: true,
        env,
        cwd: "/work".into(),
        argv,
    }
}

/// The first entry under `dir`, at any depth and following no link, named one of `names` under ASCII case folding.
fn holds(dir: &Path, names: &[&str]) -> Result<Option<PathBuf>, Error> {
    let mut todo = vec![dir.to_path_buf()];
    while let Some(d) = todo.pop() {
        for entry in std::fs::read_dir(&d).map_err(io_at(&d))? {
            let entry = entry.map_err(io_at(&d))?;
            let name = entry.file_name();
            if names.iter().any(|n| name.as_encoded_bytes().eq_ignore_ascii_case(n.as_bytes())) {
                return Ok(Some(entry.path()));
            }
            if entry.file_type().map_err(io_at(&entry.path()))?.is_dir() {
                todo.push(entry.path());
            }
        }
    }
    Ok(None)
}

/// Runs one dispatch of `tool` in `launch.role`'s cell, the copy being `scratch.work` and the credential the
/// person's, under `home` (3.6). A copy that breaks the adapter's machine rule is `CONFINEMENT_UNAVAILABLE`, and a
/// credential that is not a regular file there `TOOL_FAILED`, both before any process starts. The role's schema is
/// written at [`SCHEMA`]. Gives the run and what the adapter read in its standard output once its cell has ended.
#[allow(clippy::too_many_arguments)]
pub fn dispatch(
    tool: Tool,
    backend: &Backend,
    programs: &Programs,
    launch: &Launch,
    home: &Path,
    scratch: &Scratch,
    evidence: &Path,
    limits: Limits,
) -> Result<(Ran, Report), Unfinished> {
    let early = |error| Unfinished { error, ended: true };
    let name = tool.adapter.program.adapter.unwrap_or_default();
    if let Some(found) = holds(&scratch.work, tool.adapter.machine).map_err(early)? {
        let why = format!("{name}: its launch directory holds {}, which its machine rule refuses", found.display());
        return Err(early(refuse(Code::ConfinementUnavailable, Some(&found), why)));
    }
    // Reached through no link, its own or a folder's on the way from the home (P9C-5): what it resolves to is what the
    // home's resolution and the declared path give, and it is a regular file.
    let declared = home.join(tool.adapter.credential);
    let credential = home.canonicalize().ok().map(|h| h.join(tool.adapter.credential)).filter(|c| {
        c.canonicalize().ok().as_ref() == Some(c) && std::fs::symlink_metadata(c).is_ok_and(|m| m.is_file())
    });
    let Some(credential) = credential else {
        let why = format!(
            "{name}: no credential at {}, reached through no link; log in with the tool first",
            declared.display()
        );
        return Err(early(refuse(Code::ToolFailed, Some(&declared), why)));
    };
    let schema = scratch.home.join("schema.json");
    std::fs::write(&schema, launch.schema).map_err(io_at(&schema)).map_err(early)?;
    let argv = (tool.adapter.command)(tool.executable, launch);
    let ran =
        confine::run(backend, programs, &cell(tool, scratch, argv, Some(&credential)), scratch, evidence, limits)?;
    let out = std::fs::read(&ran.stdout.path).map_err(io_at(&ran.stdout.path));
    let out = out.map_err(early)?;
    let report = (tool.adapter.read)(&out);
    Ok((ran, report))
}

/// The preflight (REQ-SEC-1, PD-27, 3.9): each of the adapter's commands, in each role's cell with no network and no
/// credential, must end with status 0. A failure refuses (`CONFINEMENT_UNAVAILABLE`) and names its evidence; it calls
/// no model.
pub fn preflight(tool: Tool, backend: &Backend, programs: &Programs, domain: &Path) -> Result<(), Error> {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos());
    let limits = Limits { time: Duration::from_secs(60), bytes: 1 << 20 };
    let name = tool.adapter.program.adapter.unwrap_or(tool.adapter.program.executable);
    for role in Role::ALL {
        for (n, argv) in (tool.adapter.preflight)(tool.executable).into_iter().enumerate() {
            let run = format!("preflight-{name}-{}-{n}-{}-{nanos}", role.name(), std::process::id());
            let scratch = Scratch::create(domain, &run)?;
            let evidence = confine::evidence(domain, &run)?;
            let ran = confine::run(backend, programs, &cell(tool, &scratch, argv, None), &scratch, &evidence, limits);
            let fail = |how: String| {
                let words = format!(
                    "{name}'s preflight {how} in the {} role's cell; its output is kept in {}",
                    role.name(),
                    evidence.display()
                );
                refuse(Code::ConfinementUnavailable, Some(&evidence), words)
            };
            match ran {
                Err(u) if !u.ended => return Err(fail("did not end".into())),
                Err(u) => {
                    scratch.remove()?;
                    return Err(fail(format!("could not run: {}", u.error.detail())));
                }
                Ok(ran) => {
                    scratch.remove()?;
                    if ran.end != End::Exited(0) {
                        return Err(fail(format!("step {} failed ({:?})", n + 1, ran.end)));
                    }
                }
            }
        }
    }
    Ok(())
}

/// The longest text a result may hold in one field, in characters; the most findings a review may give.
const TEXT: usize = 4000;
const FINDINGS: usize = 50;

/// The output schema of `role` (REQ-RUN-8), as JSON Schema: a build's summary; a review's verdict and findings. Every
/// property is required, as a provider's strict mode asks (Codex's, part 9d): a finding with no path holds `null`.
pub fn schema(role: Role) -> Value {
    let text = json!({"type": "string", "minLength": 1, "maxLength": TEXT});
    match role {
        Role::Build => json!({
            "type": "object", "additionalProperties": false, "required": ["summary"],
            "properties": {"summary": text},
        }),
        Role::Review => json!({
            "type": "object", "additionalProperties": false, "required": ["verdict", "findings"],
            "properties": {
                "verdict": {"enum": ["accept", "reject"]},
                "findings": {"type": "array", "maxItems": FINDINGS, "items": {
                    "type": "object", "additionalProperties": false,
                    "required": ["severity", "status", "path", "summary"],
                    "properties": {
                        "severity": {"enum": ["blocking", "minor"]},
                        "status": {"enum": ["confirmed", "suspected"]},
                        "path": {"type": ["string", "null"], "minLength": 1, "maxLength": TEXT},
                        "summary": text,
                    },
                }},
            },
        }),
    }
}

/// Whether `v` is an object with exactly the fields `required` and some of `optional`.
fn fields(v: &Value, required: &[&str], optional: &[&str]) -> bool {
    v.as_object().is_some_and(|o| {
        required.iter().all(|k| o.contains_key(*k))
            && o.keys().all(|k| required.contains(&&**k) || optional.contains(&&**k))
    })
}

/// Whether `v` is a text of 1 to [`TEXT`] characters.
fn text(v: &Value) -> bool {
    v.as_str().is_some_and(|s| !s.is_empty() && s.chars().count() <= TEXT)
}

fn one_of(v: &Value, set: &[&str]) -> bool {
    v.as_str().is_some_and(|s| set.contains(&s))
}

/// Validates a role's result against its schema (REQ-RUN-8), by the core, whatever the tool checked. A review that
/// does not fit, or whose verdict and findings disagree (a reject with no confirmed blocking finding, an accept with
/// one), is `REVIEW_INVALID`; a build's that does not fit is `TOOL_FAILED`.
pub fn validate(role: Role, result: &Value) -> Result<(), Error> {
    match role {
        Role::Build if fields(result, &["summary"], &[]) && text(&result["summary"]) => Ok(()),
        Role::Build => Err(refuse(Code::ToolFailed, None, "the builder's result does not fit its role's schema")),
        Role::Review => {
            let invalid = |why: &str| Err(refuse(Code::ReviewInvalid, None, why));
            let findings = result.get("findings").and_then(Value::as_array);
            if !fields(result, &["verdict", "findings"], &[]) || !one_of(&result["verdict"], &["accept", "reject"]) {
                return invalid("the verdict is not accept or reject, or the review has other fields");
            }
            let Some(findings) = findings.filter(|f| f.len() <= FINDINGS) else {
                return invalid("the findings are not a list of at most 50");
            };
            let well_formed = |f: &Value| {
                fields(f, &["severity", "status", "path", "summary"], &[])
                    && one_of(&f["severity"], &["blocking", "minor"])
                    && one_of(&f["status"], &["confirmed", "suspected"])
                    && text(&f["summary"])
                    && (f["path"].is_null() || text(&f["path"]))
            };
            if !findings.iter().all(well_formed) {
                return invalid("a finding lacks a severity, a status, a path or a summary, or has another field");
            }
            let blocking = findings.iter().any(|f| f["severity"] == "blocking" && f["status"] == "confirmed");
            match (result["verdict"].as_str(), blocking) {
                (Some("reject"), false) => invalid("a reject needs at least one confirmed blocking finding"),
                (Some("accept"), true) => invalid("an accept cannot hold a confirmed blocking finding"),
                _ => Ok(()),
            }
        }
    }
}

/// The base rules a dispatched agent needs (REQ-RUN-8): names only commands that exist, reports outcomes as they
/// are, works from its packet alone; the rest govern the domain, which no copy holds.
const AGENT_RULES: [usize; 3] = [8, 11, 12];

/// The rules of `role`'s packet, in `lang`: the role's own instruction, the base rules an agent needs and the
/// project's own rules a maestro approved (`RULES.md`'s source).
pub fn rules(record: &Replay, role: Role, lang: &str) -> Vec<String> {
    let mut out = vec![say(lang, &format!("packet.{}", role.name()), &[])];
    out.extend(AGENT_RULES.iter().map(|n| say(lang, &format!("rules.base.{n}"), &[])));
    let mut own: Vec<_> = alambique::rules(record).into_iter().collect();
    own.sort_by_key(|(k, _)| (k[1..].parse::<u64>().unwrap_or(u64::MAX), k.clone()));
    out.extend(own.into_iter().map(|(k, rule)| format!("{k}. {rule}")));
    out
}

/// What a round after the first carries: the previous round's confirmed findings and the output of each check that
/// failed (`{"check", "output"}`).
pub struct Previous<'a> {
    pub findings: &'a [Value],
    pub failed: &'a [Value],
}

/// The packet of `role` for the task whose proposal is `task` (REQ-RUN-8): its goal, requirements and allowed files,
/// the rules, the delegation and code languages, the role's output schema and, from the second round on,
/// `previous`. Nothing else is in it.
pub fn packet(task: &Value, role: Role, rules: &[String], code: &str, previous: Option<&Previous>) -> String {
    let mut p = json!({
        "role": role.name(),
        "goal": task["goal"],
        "requirements": task["requirements"],
        "allowed": task["allowed"],
        "rules": rules,
        "languages": {"delegation": task["delegationLanguage"], "code": code},
        "schema": schema(role),
    });
    if let Some(prev) = previous {
        p["previous"] = json!({"findings": prev.findings, "failedChecks": prev.failed});
    }
    serde_json::to_string_pretty(&p).expect("a packet serializes")
}

/// `attempt-ended`'s `usage` (S-5): Fabrewk's own seconds, `null` when it did not observe the end, and the tokens as
/// the tool reported them, `null` when it reported none.
pub fn usage(seconds: Option<u64>, tokens: Option<Tokens>) -> Value {
    let tokens = tokens.map(|t| json!({"input": t.input, "output": t.output, "cached": t.cached}));
    json!({"seconds": seconds, "tokens": tokens})
}

/// A command's simple commands, as the adapters' audits count them (PD-26): split on newlines, `;`, `&&`, `||` and
/// `|` outside quotes; a backslash escapes the next character outside single quotes, and with a newline joins two
/// lines (P9C-4); a `#` that starts a word outside quotes comments the rest of its line out (P9C-3); blank parts
/// dropped (PD-26.2).
pub fn segments(command: &str) -> Vec<String> {
    let (mut parts, mut current, mut quote) = (Vec::new(), String::new(), None);
    let mut chars = command.chars().peekable();
    while let Some(c) = chars.next() {
        match (quote, c) {
            (Some('\''), '\'') | (Some('"'), '"') => {
                quote = None;
                current.push(c);
            }
            // A backslash and a newline join two lines, as the shell removes them before reading (P9C-4).
            (q, '\\') if q != Some('\'') && chars.next_if_eq(&'\n').is_some() => {}
            (q, '\\') if q != Some('\'') => {
                current.push(c);
                current.extend(chars.next());
            }
            (Some(_), _) => current.push(c),
            (None, '\'' | '"') => {
                quote = Some(c);
                current.push(c);
            }
            (None, '#') if current.chars().last().is_none_or(char::is_whitespace) => {
                while chars.next_if(|&n| n != '\n').is_some() {}
            }
            (None, '\n' | ';') => parts.push(std::mem::take(&mut current)),
            (None, '&' | '|') if chars.peek() == Some(&c) => {
                chars.next();
                parts.push(std::mem::take(&mut current));
            }
            (None, '|') => parts.push(std::mem::take(&mut current)),
            (None, _) => current.push(c),
        }
    }
    parts.push(current);
    parts.into_iter().map(|p| p.trim().to_owned()).filter(|p| !p.is_empty()).collect()
}
