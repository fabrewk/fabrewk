//! The one declaration of commands (REQ-CLI-1; D-053 point 3, D-055 point 3): every command, its argument, its flags,
//! the moves it may cause, its refusals and its documents. The parser, help, the fish and POSIX completions and the
//! reference documentation are generated from it, so they list exactly the commands that exist.

use super::text::say;
use crate::code::Code::{self, *};
use crate::settings::KEYS;
use std::ffi::OsString;

/// One command.
#[derive(Debug)]
pub struct Command {
    pub name: &'static str,
    /// Its one optional argument's name, if it takes one.
    pub arg: Option<&'static str>,
    pub flags: &'static [&'static str],
    pub moves: &'static [&'static str],
    pub refusals: &'static [Code],
    /// The `schema` of each document its `--json` gives.
    pub schemas: &'static [&'static str],
}

const READ: &[&str] = &["json", "help"];

/// Every command of this phase (D-053): the reading commands. `check`'s findings are its checks' codes (`CHECKS`).
pub const COMMANDS: &[Command] = &[
    Command {
        name: "status",
        arg: Some("id"),
        flags: READ,
        moves: &[],
        refusals: &[],
        schemas: &["fabrewk.status/1", "fabrewk.subject/1"],
    },
    Command {
        name: "config",
        arg: Some("key"),
        flags: READ,
        moves: &[],
        refusals: &[
            PolicyAbsent,
            SettingsInvalid,
            SettingsKeyNotPersonal,
            AliasMismatch,
            PrivateFolderPermissions,
            KeyUnknown,
        ],
        schemas: &["fabrewk.config/1", "fabrewk.value/1"],
    },
    Command { name: "check", arg: None, flags: READ, moves: &[], refusals: &[], schemas: &["fabrewk.check/1"] },
];

/// The program's own flags, without a command; `--version` also takes `--json`.
pub const PROGRAM_FLAGS: [&str; 2] = ["help", "version"];

/// The documents every command may give besides its own: a refusal, a usage error, an internal error, the version.
pub const SCHEMAS: [&str; 4] = ["fabrewk.refused/1", "fabrewk.usage/1", "fabrewk.internal/1", "fabrewk.version/1"];

/// The exit statuses of this phase (REQ-CLI-3); 4 and 10 come with phase 9.
pub const EXITS: [i32; 5] = [0, 1, 2, 3, 70];

/// What the command line asks for.
#[derive(Debug)]
pub enum Parsed {
    Help(Option<&'static Command>),
    Version { json: bool },
    Run { command: &'static Command, arg: Option<String>, json: bool },
}

/// A command line the declaration does not accept: the catalogue's message, the word at fault, and the command whose
/// help to point to.
#[derive(Debug)]
pub struct Usage {
    pub key: &'static str,
    pub word: String,
    pub command: Option<&'static Command>,
}

fn usage(key: &'static str, word: impl Into<String>, command: Option<&'static Command>) -> Usage {
    Usage { key, word: word.into(), command }
}

/// Parses `args` (without the program's name) against the declaration: exactly what it lists, nothing else.
pub fn parse(args: impl IntoIterator<Item = OsString>) -> Result<Parsed, Usage> {
    use lexopt::Arg::{Long, Short, Value};
    let mut p = lexopt::Parser::from_args(args);
    let (mut command, mut arg, mut help, mut version, mut json): (Option<&'static Command>, _, _, _, _) =
        (None, None, false, false, false);
    loop {
        let next = p.next().map_err(|e| match e {
            lexopt::Error::UnexpectedValue { option, .. } => usage("usage.value", option, command),
            _ => usage("usage.utf8", "", command),
        })?;
        let allowed = |f: &str| command.map_or(PROGRAM_FLAGS.contains(&f) || f == "json", |c| c.flags.contains(&f));
        match next {
            None => break,
            Some(Long(f)) if allowed(f) => match f {
                "help" => help = true,
                "version" => version = true,
                _ => json = true,
            },
            Some(Long(f)) => return Err(usage("usage.flag", format!("--{f}"), command)),
            Some(Short(c)) => return Err(usage("usage.flag", format!("-{c}"), command)),
            Some(Value(v)) => {
                let v = v.into_string().map_err(|_| usage("usage.utf8", "", command))?;
                match command {
                    None if version => return Err(usage("usage.extra", v, None)),
                    None => {
                        let found = COMMANDS.iter().find(|c| c.name == v);
                        command = Some(found.ok_or_else(|| usage("usage.command", v.clone(), None))?);
                    }
                    Some(c) if c.arg.is_some() && arg.is_none() => arg = Some(v),
                    Some(c) => return Err(usage("usage.extra", v, Some(c))),
                }
            }
        }
    }
    match (command, help, version) {
        (_, true, false) => Ok(Parsed::Help(command)),
        (None, false, false) if json => Err(usage("usage.flag", "--json", None)),
        (None, false, true) => Ok(Parsed::Version { json }),
        (Some(command), false, _) => Ok(Parsed::Run { command, arg, json }),
        (None, true, true) => Err(usage("usage.flag", "--version", None)),
        (None, false, false) => Err(usage("usage.none", "", None)),
        (Some(c), true, true) => Err(usage("usage.flag", "--version", Some(c))),
    }
}

/// A command's synopsis: `fabrewk status [<id>] [--json]`.
pub fn synopsis(c: &Command) -> String {
    let arg = c.arg.map(|a| format!(" [<{a}>]")).unwrap_or_default();
    let json = if c.flags.contains(&"json") { " [--json]" } else { "" };
    format!("fabrewk {}{arg}{json}", c.name)
}

fn pad(rows: &[(String, String)]) -> String {
    let width = rows.iter().map(|r| r.0.chars().count()).max().unwrap_or(0);
    rows.iter().map(|(a, b)| format!("  {a:width$}  {b}\n")).collect()
}

/// Help in `lang`: the program's, or one command's.
pub fn help(lang: &str, command: Option<&Command>) -> String {
    let t = |k: &str| say(lang, k, &[]);
    let flag = |f: &str| (format!("--{f}"), t(&format!("flag.{f}")));
    match command {
        None => {
            let commands: Vec<_> = COMMANDS
                .iter()
                .map(|c| (synopsis(c)[8..].to_owned(), t(&format!("command.{}.summary", c.name))))
                .collect();
            let flags: Vec<_> = PROGRAM_FLAGS.iter().map(|f| flag(f)).collect();
            let word = t("help.command-word");
            format!(
                "{}\n\n{}:\n  fabrewk <{word}> [--help]\n  fabrewk --version [--json]\n\n{}:\n{}\n{}:\n{}",
                t("help.intro"),
                t("help.usage"),
                t("help.commands"),
                pad(&commands),
                t("help.options"),
                pad(&flags)
            )
        }
        Some(c) => {
            let mut out = format!("{}\n\n  {}\n", t(&format!("command.{}.summary", c.name)), synopsis(c));
            if let Some(a) = c.arg {
                out += &format!(
                    "\n{}:\n{}",
                    t("help.argument"),
                    pad(&[(format!("<{a}>"), t(&format!("command.{}.arg", c.name)))])
                );
            }
            let flags: Vec<_> = c.flags.iter().map(|f| flag(f)).collect();
            out += &format!("\n{}:\n{}", t("help.options"), pad(&flags));
            if !c.refusals.is_empty() {
                let rows: Vec<_> = c.refusals.iter().map(|r| (r.to_string(), t(&format!("code.{r}.reason")))).collect();
                out += &format!("\n{}:\n{}", t("help.refusals"), pad(&rows));
            }
            out
        }
    }
}

fn quoted(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

/// The keys `config <key>` completes: every declared key without a name to fill.
fn keys() -> impl Iterator<Item = &'static str> {
    KEYS.iter().map(|k| k.path).filter(|p| !p.contains('*'))
}

/// The fish completion, in the canonical language.
pub fn fish() -> String {
    let mut out = String::from(
        "# fabrewk completion for fish, generated from the declaration of commands.\ncomplete -c fabrewk -f\n",
    );
    for f in PROGRAM_FLAGS {
        out += &format!(
            "complete -c fabrewk -n __fish_use_subcommand -l {f} -d {}\n",
            quoted(&say("en", &format!("flag.{f}"), &[]))
        );
    }
    out += &format!(
        "complete -c fabrewk -n '__fish_seen_argument -l version' -l json -d {}\n",
        quoted(&say("en", "flag.json", &[]))
    );
    for c in COMMANDS {
        let summary = say("en", &format!("command.{}.summary", c.name), &[]);
        out += &format!("complete -c fabrewk -n __fish_use_subcommand -a {} -d {}\n", c.name, quoted(&summary));
        for f in c.flags {
            let d = quoted(&say("en", &format!("flag.{f}"), &[]));
            out += &format!("complete -c fabrewk -n '__fish_seen_subcommand_from {}' -l {f} -d {d}\n", c.name);
        }
    }
    let words: Vec<&str> = keys().collect();
    out += &format!("complete -c fabrewk -n '__fish_seen_subcommand_from config' -a {}\n", quoted(&words.join(" ")));
    out
}

/// The POSIX completion: POSIX `sh` syntax that registers a completion with bash's `complete` (zsh's through
/// `bashcompinit`); bash's arrays are read and set only inside `eval`, so `dash -n` accepts the file (D-055).
pub fn posix() -> String {
    let mut cases = String::new();
    for c in COMMANDS {
        let mut words: Vec<String> = c.flags.iter().map(|f| format!("--{f}")).collect();
        if c.name == "config" {
            words.extend(keys().map(str::to_owned));
        }
        cases += &format!("        {}) _fabrewk_words='{}' ;;\n", c.name, words.join(" "));
    }
    let names: Vec<&str> = COMMANDS.iter().map(|c| c.name).collect();
    format!(
        "# fabrewk completion for bash, and for zsh with bashcompinit; generated from the declaration of commands.\n\
         _fabrewk() {{\n    \
         eval '_fabrewk_command=${{COMP_WORDS[1]}}'\n    \
         if [ \"$COMP_CWORD\" -eq 1 ]; then\n        \
         _fabrewk_words='{} --help --version'\n    \
         else\n        \
         case $_fabrewk_command in\n{cases}        \
         --version) _fabrewk_words='--json' ;;\n        \
         *) _fabrewk_words='' ;;\n        \
         esac\n    \
         fi\n    \
         eval 'COMPREPLY=($(compgen -W \"$_fabrewk_words\" -- \"$2\"))'\n\
         }}\n\
         complete -F _fabrewk fabrewk\n",
        names.join(" ")
    )
}

/// The reference documentation in `lang`, in Markdown.
pub fn reference(lang: &str, checks: &[(String, String, String)]) -> String {
    let t = |k: &str| say(lang, k, &[]);
    let mut out = format!("# {}\n\n{}\n", t("reference.title"), t("reference.intro"));
    for c in COMMANDS {
        out += &format!("\n## `{}`\n\n{}\n\n", synopsis(c), t(&format!("command.{}.summary", c.name)));
        if let Some(a) = c.arg {
            out += &format!("- `<{a}>`: {}\n", t(&format!("command.{}.arg", c.name)));
        }
        for f in c.flags {
            out += &format!("- `--{f}`: {}\n", t(&format!("flag.{f}")));
        }
        out += &format!("\n{} {}\n", t("reference.moves"), t("reference.none"));
        let schemas: Vec<String> = c.schemas.iter().map(|s| format!("`{s}`")).collect();
        out += &format!("\n{} {}\n", t("reference.documents"), schemas.join(", "));
        if !c.refusals.is_empty() {
            out += &format!("\n{}\n\n", t("help.refusals"));
            for r in c.refusals {
                out += &format!("- `{r}`: {}\n", t(&format!("code.{r}.reason")));
            }
        }
        if c.name == "check" {
            out += &format!(
                "\n| | {} | {} | {} |\n| --- | --- | --- | --- |\n",
                t("reference.verifies"),
                t("reference.codes"),
                t("reference.control")
            );
            for (id, codes, control) in checks {
                out += &format!("| {id} | {} | {codes} | {control} |\n", t(&format!("check.{id}")));
            }
        }
    }
    out += &format!("\n## {}\n\n", t("reference.program"));
    for f in PROGRAM_FLAGS {
        out += &format!("- `fabrewk --{f}`: {}\n", t(&format!("flag.{f}")));
    }
    out += &format!(
        "\n{} {}\n\n## {}\n\n",
        t("reference.documents"),
        SCHEMAS.map(|s| format!("`{s}`")).join(", "),
        t("reference.exits")
    );
    for e in EXITS {
        out += &format!("- `{e}`: {}\n", t(&format!("exit.{e}")));
    }
    out
}
