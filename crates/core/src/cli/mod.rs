//! The command surface (REQ-CLI-1 to REQ-CLI-4, REQ-VIEW-4; part 7d2, D-053, D-055): one declaration of commands, from
//! which the parser, help, completions and the reference documentation are generated; every human message from the
//! catalogue in the person's language; with `--json`, one document with a `schema` field; the exit statuses. The
//! person is the maintainer whose key Git's `user.signingkey` names (D-055). The reading commands take no lock and
//! write nothing.

mod check;
mod config;
mod decl;
mod status;
mod text;

pub use check::CHECKS;
pub use decl::{COMMANDS, Command, EXITS, PROGRAM_FLAGS, Parsed, SCHEMAS, Usage, fish, help, parse, posix, synopsis};
pub use text::{CATALOGUES, get, messages, say};

use crate::code::{Code, Error};
use crate::detect::Program;
use crate::identity::{self, PolicyVerifier};
use crate::record::{Replay, replay};
use crate::settings::{self, Level, Policy, Val};
use serde_json::{Value, json};
use std::ffi::OsString;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Exit statuses (REQ-CLI-3).
pub const DONE: i32 = 0;
pub const REFUSED: i32 = 1;
pub const USAGE: i32 = 2;
pub const FOUND: i32 = 3;
pub const INTERNAL: i32 = 70;

/// What a command gives: its document, its human text, where the text goes and the exit status.
pub(crate) struct Out {
    doc: Value,
    text: String,
    error: bool,
    exit: i32,
}

impl Out {
    fn new(doc: Value, text: String) -> Out {
        Out { doc, text, error: false, exit: DONE }
    }
}

/// Why a command gives no document of its own.
pub(crate) enum Failed {
    Refused(Vec<Error>),
    Usage(Usage),
}

impl From<Vec<Error>> for Failed {
    fn from(errors: Vec<Error>) -> Failed {
        Failed::Refused(errors)
    }
}

impl From<Error> for Failed {
    fn from(error: Error) -> Failed {
        Failed::Refused(vec![error])
    }
}

/// What every reading command reads first, without a lock: the record, the policy in force, the person, the language.
pub(crate) struct Cx {
    root: PathBuf,
    domain: PathBuf,
    record: Result<Replay, Vec<Error>>,
    policy: Option<Policy>,
    person: Option<String>,
    lang: &'static str,
    modules: &'static [Program],
}

impl Cx {
    fn new(root: PathBuf, modules: &'static [Program]) -> Cx {
        let domain = root.join(".brewstillery");
        let record = replay(&domain, &PolicyVerifier);
        let policy = record.as_ref().ok().and_then(|r| settings::in_force(r).ok().flatten());
        let person = policy.as_ref().and_then(|p| identity::person(&p.settings, &root));
        let lang = language(&domain, policy.as_ref(), person.as_deref());
        Cx { root, domain, record, policy, person, lang, modules }
    }

    fn t(&self, key: &str, params: &[(&str, &str)]) -> String {
        say(self.lang, key, params)
    }

    /// A path as the person reads it: relative to where they ran the command.
    fn show(&self, path: &Path) -> String {
        path.strip_prefix(&self.root).unwrap_or(path).display().to_string()
    }

    /// One refusal or finding as a document's entry: its code, the catalogue's reason and next step, the command to
    /// run if any, where, and the core's detail.
    fn entry(&self, e: &Error) -> Value {
        let (code, path, detail) = match e {
            Error::Refused { code, path, detail } => (*code, path.as_deref(), detail.as_str()),
            // The operating system's own text never reaches a person.
            Error::Io { path, .. } => (Code::CheckUnknown, Some(path.as_path()), "cannot be read"),
        };
        let k = |field: &str| format!("code.{code}.{field}");
        json!({
            "code": code.to_string(), "reason": self.t(&k("reason"), &[]), "next": self.t(&k("next"), &[]),
            "run": get(self.lang, &k("run")), "path": path.map(|p| self.show(p)), "detail": detail,
        })
    }

    fn entry_text(&self, v: &Value, indent: &str) -> String {
        let s = |k: &str| v[k].as_str().unwrap_or_default();
        let mut out = format!("{indent}{}: {}\n", s("code"), s("reason"));
        if !s("path").is_empty() {
            out += &format!("{indent}  {}: {}\n", self.t("ui.where", &[]), s("path"));
        }
        if !s("detail").is_empty() {
            out += &format!("{indent}  {}: {}\n", self.t("ui.detail", &[]), s("detail"));
        }
        out += &format!("{indent}  {}: {}\n", self.t("ui.next", &[]), s("next"));
        if !s("run").is_empty() {
            out += &format!("{indent}    {}\n", s("run"));
        }
        out
    }

    fn refused(&self, errors: Vec<Error>) -> Out {
        if let Some(Error::Io { path, .. }) = errors.iter().find(|e| e.code().is_none()) {
            return internal(self.lang, &self.show(path));
        }
        let entries: Vec<Value> = errors.iter().map(|e| self.entry(e)).collect();
        let text = entries.iter().map(|v| self.entry_text(v, "")).collect();
        Out { doc: json!({"schema": "fabrewk.refused/1", "refusals": entries}), text, error: true, exit: REFUSED }
    }

    fn usage(&self, u: Usage) -> Out {
        let message = self.t(u.key, &[("word", &u.word)]);
        let run = match u.command {
            Some(c) if u.key == "usage.id" => format!("fabrewk {}", c.name),
            Some(c) => format!("fabrewk {} --help", c.name),
            None => "fabrewk --help".to_owned(),
        };
        let text = format!("{message}\n  {}: {}\n    {run}\n", self.t("ui.next", &[]), self.t("usage.next", &[]));
        let doc = json!({"schema": "fabrewk.usage/1", "message": message, "run": run});
        Out { doc, text, error: true, exit: USAGE }
    }

    /// The private folder, from this process's environment.
    fn folder(&self) -> Result<PathBuf, Error> {
        crate::machine::private_folder(|k| std::env::var_os(k))
    }

    /// The person's own file, when the policy knows who they are and they have one.
    fn personal(&self) -> Result<Option<settings::Settings>, Vec<Error>> {
        let Some(alias) = &self.person else { return Ok(None) };
        let path = self.domain.join(alias).join("settings.toml");
        if std::fs::symlink_metadata(&path).is_err() {
            return Ok(None);
        }
        settings::read(&path, Level::Personal).map(Some)
    }

    /// Next steps, each a command that exists, under their heading.
    fn next(&self, commands: &[&str]) -> String {
        if commands.is_empty() {
            return String::new();
        }
        format!(
            "{}:\n{}",
            self.t("ui.next-steps", &[]),
            commands.iter().map(|c| format!("  {c}\n")).collect::<String>()
        )
    }
}

/// The interface language (D-055): the person's `languages.interfaceLanguage` when the policy knows them, else the
/// policy's canonical documentation language, else `en`; then the first catalogue its fallbacks reach.
fn language(domain: &Path, policy: Option<&Policy>, person: Option<&str>) -> &'static str {
    let tag = policy.and_then(|p| {
        let file = person.map(|a| domain.join(a).join("settings.toml"));
        let personal = file.and_then(|f| settings::read(&f, Level::Personal).ok());
        let eff = settings::effective(p, personal.as_ref(), None, &[], &[]).ok()?;
        match eff.values.get("languages.interfaceLanguage") {
            Some((Val::Str(s), _)) => Some(s.clone()),
            _ => None,
        }
    });
    let tags = settings::fallbacks(tag.as_deref().unwrap_or("en"));
    tags.iter().find_map(|t| CATALOGUES.iter().find(|(l, _)| l == t).map(|(l, _)| *l)).unwrap_or("en")
}

fn internal(lang: &str, path: &str) -> Out {
    let doc = json!({"schema": "fabrewk.internal/1", "path": path});
    Out { doc, text: say(lang, "internal.io", &[("path", path)]) + "\n", error: true, exit: INTERNAL }
}

fn version() -> Out {
    let features: Vec<&str> = if cfg!(feature = "fault-injection") { vec!["fault-injection"] } else { vec![] };
    let doc = json!({"schema": "fabrewk.version/1", "version": crate::VERSION, "features": features});
    Out::new(doc, format!("fabrewk {}\n", crate::VERSION))
}

/// Runs the program with `args` (without its name) from the current directory and gives its exit status; `modules`
/// are the built-in modules' programs (D-051).
pub fn main(args: Vec<OsString>, modules: &'static [Program]) -> i32 {
    std::panic::set_hook(Box::new(|_| {
        let _ = writeln!(std::io::stderr(), "{}", say("en", "internal.panic", &[]));
        std::process::exit(INTERNAL);
    }));
    let asked = args.iter().any(|a| a == "--json");
    let parsed = decl::parse(args);
    let json = asked && !matches!(parsed, Ok(Parsed::Help(_)));
    let cx = || std::env::current_dir().map(|root| Cx::new(root, modules));
    let out = match (parsed, cx()) {
        (Ok(Parsed::Version { .. }), _) => version(),
        (_, Err(_)) => internal("en", "."),
        (Err(u), Ok(cx)) => cx.usage(u),
        (Ok(Parsed::Help(c)), Ok(cx)) => Out::new(Value::Null, decl::help(cx.lang, c)),
        (Ok(Parsed::Run { command, arg, .. }), Ok(cx)) => {
            let result = match command.name {
                "status" => status::run(&cx, arg.as_deref()),
                "config" => config::run(&cx, arg.as_deref()),
                _ => check::run(&cx),
            };
            match result {
                Ok(out) => out,
                Err(Failed::Usage(u)) => cx.usage(u),
                Err(Failed::Refused(errors)) => cx.refused(errors),
            }
        }
    };
    let text = if json { crate::record::encode(&out.doc).map(String::from_utf8).and_then(Result::ok) } else { None };
    let text = text.unwrap_or(out.text);
    let _ = if out.error && !json {
        std::io::stderr().write_all(text.as_bytes())
    } else {
        std::io::stdout().write_all(text.as_bytes())
    };
    out.exit
}

/// The reference documentation in `lang` (D-055 point 3).
pub fn reference(lang: &str) -> String {
    let checks: Vec<(String, String, String)> = CHECKS
        .iter()
        .map(|c| {
            let codes: Vec<String> = c.codes.iter().map(|k| format!("`{k}`")).collect();
            (c.id.to_owned(), codes.join(", "), say(lang, &format!("control.{}", check::label(c.control)), &[]))
        })
        .collect();
    decl::reference(lang, &checks)
}
