//! The command surface (REQ-CLI-1 to REQ-CLI-4, REQ-VIEW-4; part 7d2, D-053, D-055): one declaration of commands, from
//! which the parser, help, completions and the reference documentation are generated; every human message from the
//! catalogue in the person's language; with `--json`, one document with a `schema` field; the exit statuses. The
//! person is the maintainer whose key Git's `user.signingkey` names (D-055). The reading commands take no lock and
//! write nothing.

mod accept;
mod adopt;
mod approve;
mod check;
mod close;
mod config;
mod decl;
mod ext;
mod propose;
mod run;
mod status;
mod sweep;
mod tasks;
mod text;
mod write;

pub use check::CHECKS;
pub use decl::{
    COMMANDS, Command, EXITS, Options, PROGRAM_FLAGS, Parsed, SCHEMAS, Usage, VALUES, answer_format, file_format, fish,
    flag_key, help, parse, posix, refusals, synopsis, writes,
};
pub(crate) use tasks::verbatim;
pub use text::{CATALOGUES, get, messages, say};
pub use write::EXTENSION_MARKER;

use crate::adapter::Adapter;
use crate::code::{Code, Error};
use crate::confine::Backend;
use crate::detect::Program;
use crate::identity::{self, PolicyVerifier};
use crate::install::{Asked, Installer};
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
pub const WAITING: i32 = 4;
pub const UNSUPPORTED: i32 = ext::UNSUPPORTED;
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
    /// The built-in modules' programs and the backend's, all detected alike (REQ-SET-8).
    modules: Vec<Program>,
    /// The agents' markers (REQ-SEC-7): each adapter's.
    markers: Vec<&'static str>,
    adapters: &'static [&'static Adapter],
    backend: Option<&'static Backend>,
    installer: &'static Installer,
    /// Whether the command is one that writes, so that an internal error does not say that nothing changed.
    writes: bool,
}

impl Cx {
    /// The context of a command run at `root`; with `read` false, as for `assess`, which reads no record (REQ-INS-1,
    /// P10A-4), nothing is read: no record, no policy, no person.
    fn new(root: PathBuf, wired: Wired, read: bool) -> Cx {
        let (adapters, backend, installer) = wired;
        let modules =
            adapters.iter().map(|a| a.program).chain(backend.map_or(&[][..], |b| b.programs).iter().copied()).collect();
        let markers = adapters.iter().flat_map(|a| a.markers).copied().collect();
        let domain = root.join(".brewstillery");
        let record = if read { replay(&domain, &PolicyVerifier) } else { Err(Vec::new()) };
        let policy = record.as_ref().ok().and_then(|r| settings::in_force(r).ok().flatten());
        let person = policy.as_ref().and_then(|p| identity::person(&p.settings, &root));
        let lang = language(&domain, policy.as_ref(), person.as_deref());
        Cx { root, domain, record, policy, person, lang, modules, markers, adapters, backend, installer, writes: false }
    }

    fn t(&self, key: &str, params: &[(&str, &str)]) -> String {
        say(self.lang, key, params)
    }

    /// The person's language as `record`'s policy and their file give it now: what a rendering planned on `record`
    /// under the lock is in, where `lang` is as the command first read them (P7D3B-1).
    fn lang_in(&self, record: &Replay) -> &'static str {
        let policy = settings::in_force(record).ok().flatten();
        let person = policy.as_ref().and_then(|p| identity::person(&p.settings, &self.root));
        language(&self.domain, policy.as_ref(), person.as_deref())
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
            return internal(self.lang, &self.show(path), self.writes);
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

    /// The private folder, from this process's environment; one inside this repository or inside any Fabrewk domain,
    /// written there or reached through a link, is refused, so that no maintainer's material is read as it (AD-15,
    /// P7D2-28, P7D2-29).
    fn folder(&self) -> Result<PathBuf, Error> {
        let folder = crate::machine::private_folder(|k| std::env::var_os(k))?;
        if crate::code::in_domain(&folder, &self.root) {
            let why = "inside this repository or a Fabrewk domain; the private folder lies outside every repository";
            return Err(crate::code::refuse(Code::PrivateFolderPermissions, Some(&folder), why));
        }
        Ok(folder)
    }

    /// The person's own file, when the policy knows who they are and they have one.
    fn personal(&self) -> Result<Option<settings::Settings>, Vec<Error>> {
        match &self.person {
            Some(alias) => self.personal_of(alias),
            None => Ok(None),
        }
    }

    /// The personal file of `alias`, if it has one; one that is not a regular file reached without a link, or does
    /// not read, is refused, never taken for absent (P7D2-26).
    fn personal_of(&self, alias: &str) -> Result<Option<settings::Settings>, Vec<Error>> {
        let path = self.domain.join(alias).join("settings.toml");
        // Absent is only what does not exist; what cannot be examined is an error (P7D2-26).
        match std::fs::symlink_metadata(&path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(vec![Error::Io { path, error: e }]),
            Ok(_) => {}
        }
        if !crate::code::unlinked(&self.domain, &format!("{alias}/settings.toml")) {
            let why = "(document) (line 1): a link on its path; it is not followed";
            return Err(vec![crate::code::refuse(Code::SettingsInvalid, Some(&path), why)]);
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
        let own = person.filter(|a| crate::code::unlinked(domain, &format!("{a}/settings.toml")));
        let file = own.map(|a| domain.join(a).join("settings.toml"));
        let personal = file.and_then(|f| settings::read(&f, Level::Personal).ok());
        let eff = settings::effective(p, personal.as_ref(), None, &[], &[]).ok()?;
        match eff.values.get("languages.interfaceLanguage") {
            Some((Val::Str(s), _)) => Some(s.clone()),
            _ => None,
        }
    });
    catalogue(tag.as_deref().unwrap_or("en"))
}

/// The first catalogue the tag's fallbacks reach.
pub fn catalogue(tag: &str) -> &'static str {
    let tags = settings::fallbacks(tag);
    tags.iter().find_map(|t| CATALOGUES.iter().find(|(l, _)| l == t).map(|(l, _)| *l)).unwrap_or("en")
}

/// An internal error: what could not be read or written; a write command may have written before it.
fn internal(lang: &str, path: &str, writes: bool) -> Out {
    let doc = json!({"schema": "fabrewk.internal/1", "path": path});
    let key = if writes { "internal.io-write" } else { "internal.io" };
    Out { doc, text: say(lang, key, &[("path", path)]) + "\n", error: true, exit: INTERNAL }
}

fn version() -> Out {
    let features: Vec<&str> = if cfg!(feature = "fault-injection") { vec!["fault-injection"] } else { vec![] };
    let doc = json!({"schema": "fabrewk.version/1", "version": crate::VERSION, "features": features});
    Out::new(doc, format!("fabrewk {}\n", crate::VERSION))
}

/// What the binary wires: the tool adapters (parts 9c and 9d), the confinement backend, if this system has one, and the
/// installer (part 10a).
pub type Wired = (&'static [&'static Adapter], Option<&'static Backend>, &'static Installer);

/// Runs the program with `args` (without its name) from the current directory and gives its exit status; the adapters
/// `wired` are the built-in tool adapters, whose programs are detected (D-051) and whose markers show a process runs
/// inside an agent (REQ-SEC-7).
pub fn main(args: Vec<OsString>, wired: Wired) -> i32 {
    // `fabrewk <name> …` for a name no core command has is an extension's, with every word after it its own (REQ-EXT-1).
    let extension = ext::called(&args);
    let asked = extension.is_none() && args.iter().any(|a| a == "--json");
    let parsed = decl::parse(args.clone());
    let writes = extension.is_some() || matches!(&parsed, Ok(Parsed::Run { command, .. }) if decl::writes(command));
    std::panic::set_hook(Box::new(move |_| {
        let key = if writes { "internal.panic-write" } else { "internal.panic" };
        let _ = writeln!(std::io::stderr(), "{}", say("en", key, &[]));
        std::process::exit(INTERNAL);
    }));
    let json = asked && !matches!(parsed, Ok(Parsed::Help(_)));
    // The version reads nothing: no record, no Git, no file the person names.
    let out = match (parsed, std::env::current_dir()) {
        (Ok(Parsed::Version { .. }), _) => version(),
        (_, Err(_)) => internal("en", ".", writes),
        (parsed, Ok(root)) => {
            let reads = !matches!(&parsed, Ok(Parsed::Run { command, .. }) if command.name == "assess");
            let mut cx = Cx { writes, ..Cx::new(root, wired, reads) };
            // The installation's commands speak the language their answers name (D-097 point 2).
            if let Ok(Parsed::Run { options, .. }) = &parsed {
                cx.lang = options.answers.get("lang").map_or(cx.lang, |l| catalogue(&l.to_ascii_lowercase()));
            }
            let result = match (parsed, extension) {
                (_, Some(name)) => ext::run(&cx, &name, &args[1..]),
                (parsed, None) => match parsed {
                    Err(u) => Err(Failed::Usage(u)),
                    Ok(Parsed::Help(c)) => Ok(Out::new(Value::Null, decl::help(cx.lang, c))),
                    Ok(Parsed::Run { command, arg, options, .. }) => match command.name {
                        "status" => status::run(&cx, arg.as_deref()),
                        "config" => config::run(&cx, arg.as_deref()),
                        "assess" => assess(&cx, command, &options),
                        "apply" => Err(Failed::Usage(Usage {
                            key: "usage.unbuilt",
                            word: "apply".into(),
                            command: Some(command),
                        })),
                        "adopt" => adopt::run(&cx, &options),
                        "propose" => propose::run(&cx, arg.as_deref().unwrap_or_default()),
                        "authorize" => tasks::authorize(&cx, arg.as_deref().unwrap_or_default(), &options),
                        "accept" => accept::accept(&cx, arg.as_deref().unwrap_or_default(), &options),
                        "reject" => tasks::reject(&cx, arg.as_deref().unwrap_or_default(), &options),
                        "run" => tasks::run(&cx, arg.as_deref().unwrap_or_default(), &options),
                        "sweep" => sweep::sweep(&cx, arg.as_deref().unwrap_or_default(), &options),
                        "discard" => sweep::discard(&cx, arg.as_deref().unwrap_or_default(), &options),
                        "approve" => approve::approve(&cx, arg.as_deref().unwrap_or_default(), &options),
                        "close" => close::close(&cx, arg.as_deref().unwrap_or_default(), &options),
                        _ => check::run(&cx),
                    },
                    Ok(Parsed::Version { .. }) => Ok(version()),
                },
            };
            match result {
                Ok(out) => out,
                Err(Failed::Usage(u)) => cx.usage(u),
                Err(Failed::Refused(errors)) => cx.refused(errors),
            }
        }
    };
    let text = if json { crate::record::encode(&out.doc).map(String::from_utf8).and_then(Result::ok) } else { None };
    let text = text.unwrap_or_else(|| text::printable(&out.text));
    let _ = if out.error && !json {
        std::io::stderr().write_all(text.as_bytes())
    } else {
        std::io::stdout().write_all(text.as_bytes())
    };
    out.exit
}

/// `assess` (D-097): the language and the tools the answers name, checked before the installer reads anything.
fn assess(cx: &Cx, command: &'static Command, options: &Options) -> Result<Out, Failed> {
    let tools: Vec<&str> = cx.adapters.iter().map(|a| a.program.executable).collect();
    if let Some(t) = options.tools.iter().find(|t| !tools.contains(&t.as_str())) {
        return Err(Failed::Usage(Usage {
            key: "usage.tool",
            word: format!("{t}: {}", tools.join(", ")),
            command: Some(command),
        }));
    }
    let lang = options.answers.get("lang").map_or("en", String::as_str);
    if !CATALOGUES.iter().any(|(l, _)| lang.split('-').next().is_some_and(|p| p.eq_ignore_ascii_case(l))) {
        let langs: Vec<&str> = CATALOGUES.iter().map(|c| c.0).collect();
        return Err(Failed::Usage(Usage {
            key: "usage.lang",
            word: format!("{lang}: {}", langs.join(", ")),
            command: Some(command),
        }));
    }
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let asked = Asked { root: &cx.root, home: home.as_deref(), options, adapters: cx.adapters };
    let (doc, text) = (cx.installer.assess)(&asked)?;
    Ok(Out::new(doc, text))
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
