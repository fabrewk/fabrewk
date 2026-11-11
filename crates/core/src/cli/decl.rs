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
    /// Its one argument's name, if it takes one.
    pub arg: Option<&'static str>,
    /// Whether the argument must be given.
    pub required: bool,
    pub flags: &'static [&'static str],
    pub moves: &'static [&'static str],
    pub refusals: &'static [Code],
    /// The `schema` of each document its `--json` gives.
    pub schemas: &'static [&'static str],
}

const READ: &[&str] = &["json", "help"];

/// The options that take a value, with the word their help shows for it (D-062); only `widen` may be given more than
/// once. Every other option is a switch.
pub const VALUES: [(&str, &str); 5] =
    [("say", "words"), ("say-lang", "tag"), ("widen", "key"), ("seen", "digest"), ("answer", "file")];

/// The refusals every write command may give before its own (part 7d3a): the record's, the lock's, the repository's,
/// the domain's, the private folder's and the tools'.
const WRITE: &[Code] = &[
    RecordAbsent,
    PolicyAbsent,
    RecordBroken,
    RecordDigestMismatch,
    RecordNoncanonical,
    RecordFork,
    RecordTransitionInvalid,
    AttemptEnded,
    RecordSignatureInvalid,
    RecordVersionUnknown,
    RecordIdCollision,
    RecordRollback,
    MarkInvalid,
    CheckUnknown,
    DomainNotIgnored,
    NotARepository,
    HeadDetached,
    PrivateFolderPermissions,
    SettingsInvalid,
    LockHeld,
    RecordHeadMoved,
    RecordCommitRace,
    ToolFailed,
    ToolMissing,
];

/// Every command of this phase (D-053, D-062): the reading commands, then the write commands of parts 7d3a and 7d3b.
/// `check`'s findings are its checks' codes (`CHECKS`); a write command's refusals are its own, then `WRITE`'s.
pub const COMMANDS: &[Command] = &[
    Command {
        name: "status",
        arg: Some("id"),
        required: false,
        flags: READ,
        moves: &[],
        refusals: &[],
        schemas: &["fabrewk.status/1", "fabrewk.subject/2"],
    },
    Command {
        name: "config",
        arg: Some("key"),
        required: false,
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
    Command {
        name: "check",
        arg: None,
        required: false,
        flags: READ,
        moves: &[],
        refusals: &[],
        schemas: &["fabrewk.check/1"],
    },
    Command {
        name: "adopt",
        arg: None,
        required: false,
        flags: &["say", "say-lang", "widen", "json", "help"],
        moves: &["policy-adopted"],
        refusals: &[
            AgentEnvironment,
            RoleForbidden,
            WordsMissing,
            WordsCopied,
            SettingsInvalid,
            SettingsKeyNotPersonal,
            AliasMismatch,
            RoleNotEnforced,
            PolicyUncommitted,
            WideningUnnamed,
        ],
        schemas: &["fabrewk.adopted/1"],
    },
    Command {
        name: "propose",
        arg: Some("file"),
        required: true,
        flags: READ,
        moves: &["stage-proposed", "task-proposed"],
        refusals: &[StageClosed, TaskInvalid, ProtectedPath, ProtectedPathWildcard, CheckDuplicate],
        schemas: &["fabrewk.proposed/1"],
    },
    Command {
        name: "authorize",
        arg: Some("id"),
        required: true,
        flags: &["seen", "say", "say-lang", "widen", "run", "json", "help"],
        moves: &["task-authorized", "stage-authorized", "run-started"],
        refusals: &[
            AgentEnvironment,
            MoveNotAllowed,
            RoleForbidden,
            WordsMissing,
            WordsCopied,
            TaskChanged,
            CheckDuplicate,
            ProtectedPath,
            ProtectedPathWildcard,
            WideningUnnamed,
            LimitUnobservable,
            DependencyOpen,
            LockHeld,
            AlambiqueUnapproved,
            ConfinementUnavailable,
        ],
        schemas: &["fabrewk.authorized/1"],
    },
    Command {
        name: "reject",
        arg: Some("task"),
        required: true,
        flags: &["say", "say-lang", "json", "help"],
        moves: &["task-rejected"],
        refusals: &[AgentEnvironment, MoveNotAllowed, RoleForbidden, WordsMissing, WordsCopied, MergeInProgress],
        schemas: &["fabrewk.rejected/1"],
    },
    Command {
        name: "run",
        arg: Some("id"),
        required: true,
        flags: &["say", "say-lang", "json", "help"],
        moves: &["run-started"],
        refusals: &[
            AgentEnvironment,
            MoveNotAllowed,
            RoleForbidden,
            WordsMissing,
            WordsCopied,
            DependencyOpen,
            LockHeld,
            AlambiqueUnapproved,
            ConfinementUnavailable,
        ],
        schemas: &[],
    },
    Command {
        name: "sweep",
        arg: Some("alias"),
        required: true,
        flags: &["answer", "json", "help"],
        moves: &["sweep-recorded", "proposal-recorded", "proposal-withdrawn"],
        refusals: &[
            AliasInvalid,
            MaintainerUnlisted,
            FermentoMissing,
            SeparatorMissing,
            SeparatorAmbiguous,
            SeparatorDamaged,
            AnswerInvalid,
            PointChanged,
            InputUncommitted,
            InputChanged,
        ],
        schemas: &["fabrewk.swept/1"],
    },
    Command {
        name: "discard",
        arg: Some("proposal"),
        required: true,
        flags: &["say", "say-lang", "json", "help"],
        moves: &["proposal-discarded"],
        refusals: &[AgentEnvironment, ProposalUnknown, MoveNotAllowed, RoleForbidden, WordsMissing, WordsCopied],
        schemas: &["fabrewk.discarded/1"],
    },
    Command {
        name: "approve",
        arg: Some("proposal"),
        required: true,
        flags: &["say", "say-lang", "json", "help"],
        moves: &["proposal-incorporated", "direct-edit-approved", "alambique-baseline"],
        refusals: &[
            AgentEnvironment,
            ProposalUnknown,
            MoveNotAllowed,
            RoleForbidden,
            WordsMissing,
            WordsCopied,
            ProposalStale,
            AlambiqueChanged,
            AlambiqueUnapproved,
            AlambiqueMissing,
            UnitAmbiguous,
        ],
        schemas: &["fabrewk.approved/1"],
    },
];

/// The catalogue key of an option's words: `flag.<name>`, but `flag.then-run` for `--run`, since a catalogue field
/// named `run` holds a command (REQ-CLI-2).
pub fn flag_key(f: &str) -> String {
    format!("flag.{}", if f == "run" { "then-run" } else { f })
}

/// Whether a command writes (it lists the moves it may cause).
pub fn writes(c: &Command) -> bool {
    !c.moves.is_empty()
}

/// A command's refusals as its help and the reference list them: its own, then, for a write command, `WRITE`'s.
pub fn refusals(c: &Command) -> Vec<Code> {
    let mut out = c.refusals.to_vec();
    if writes(c) {
        out.extend(WRITE.iter().filter(|w| !c.refusals.contains(w)));
    }
    out
}

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
    Run { command: &'static Command, arg: Option<String>, json: bool, options: Options },
}

/// The values of the options a write command takes. The words are kept as given, so that words that are not UTF-8,
/// or none at all, are the command's refusal (`WORDS_MISSING`), never a usage error (REQ-TASK-4).
#[derive(Debug, Default)]
pub struct Options {
    pub say: Option<OsString>,
    pub say_lang: Option<String>,
    pub widen: Vec<String>,
    /// The digest of what `status <id>` showed (`authorize --seen`, D-062).
    pub seen: Option<String>,
    /// `authorize --run`: start the run in the same command.
    pub run: bool,
    /// The file of the integrator's answer (`sweep --answer`, REQ-FER-3).
    pub answer: Option<String>,
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
    let mut options = Options::default();
    loop {
        let next = p.next().map_err(|e| match e {
            lexopt::Error::UnexpectedValue { option, .. } => usage("usage.value", option, command),
            _ => usage("usage.utf8", "", command),
        })?;
        // Owned, so that an option's value can be read from the parser next.
        enum A {
            Long(String),
            Short(char),
            Value(OsString),
        }
        let next = match next {
            Some(Long(f)) => Some(A::Long(f.to_owned())),
            Some(Short(c)) => Some(A::Short(c)),
            Some(Value(v)) => Some(A::Value(v)),
            None => None,
        };
        let allowed = |f: &str| command.map_or(PROGRAM_FLAGS.contains(&f) || f == "json", |c| c.flags.contains(&f));
        match next {
            None => break,
            Some(A::Long(f)) if allowed(&f) && VALUES.iter().any(|v| v.0 == f) => {
                let word = format!("--{f}");
                // `--say` with no value is words missing, as is `--say` absent (REQ-TASK-4). A value is attached with
                // `=`, or is the next argument unless that one starts with `-`: so `--say --json` gives no words
                // rather than the words `--json`, and words that start with `-` are given as `--say=-…`.
                let attached = p.optional_value();
                let next_is_option = attached.is_none()
                    && p.raw_args().ok().and_then(|r| r.peek().map(|a| a.as_encoded_bytes().starts_with(b"-")))
                        != Some(false);
                let value = match attached {
                    Some(v) => v,
                    None if next_is_option && f == "say" => OsString::new(),
                    None if next_is_option => return Err(usage("usage.missing", word, command)),
                    None => p.value().map_err(|_| usage("usage.missing", word.clone(), command))?,
                };
                if f == "say" {
                    if options.say.replace(value).is_some() {
                        return Err(usage("usage.repeat", word, command));
                    }
                    continue;
                }
                let value = value.into_string().map_err(|_| usage("usage.utf8", "", command))?;
                let slot = match f.as_str() {
                    "widen" => {
                        options.widen.push(value);
                        continue;
                    }
                    "seen" => &mut options.seen,
                    "answer" => &mut options.answer,
                    _ if !crate::settings::is_tag(&value) => return Err(usage("usage.tag", value, command)),
                    _ => &mut options.say_lang,
                };
                if slot.replace(value).is_some() {
                    return Err(usage("usage.repeat", word, command));
                }
            }
            Some(A::Long(f)) if allowed(&f) => match f.as_str() {
                "help" => help = true,
                "version" => version = true,
                "run" => options.run = true,
                _ => json = true,
            },
            Some(A::Long(f)) => return Err(usage("usage.flag", format!("--{f}"), command)),
            Some(A::Short(c)) => return Err(usage("usage.flag", format!("-{c}"), command)),
            Some(A::Value(v)) => {
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
        (Some(c), false, _) if c.required && arg.is_none() => {
            Err(usage("usage.argument", format!("<{}>", c.arg.unwrap_or_default()), Some(c)))
        }
        // `--seen`, the digest of what the person read, is needed whenever a command takes it (D-062, D-068); a
        // missing `--say` is the command's refusal instead (REQ-TASK-4).
        (Some(c), false, _) if c.flags.contains(&"seen") && options.seen.is_none() => {
            Err(usage("usage.needed", "--seen", Some(c)))
        }
        (Some(c), false, _) if c.flags.contains(&"answer") && options.answer.is_none() => {
            Err(usage("usage.needed", "--answer", Some(c)))
        }
        (Some(command), false, _) => Ok(Parsed::Run { command, arg, json, options }),
        (None, true, true) => Err(usage("usage.flag", "--version", None)),
        (None, false, false) => Err(usage("usage.none", "", None)),
        (Some(c), true, true) => Err(usage("usage.flag", "--version", Some(c))),
    }
}

/// A command's synopsis: `fabrewk status [<id>] [--json]`, `fabrewk adopt --say <words> [--widen <key>]… [--json]`.
pub fn synopsis(c: &Command) -> String {
    let arg = match (c.arg, c.required) {
        (Some(a), true) => format!(" <{a}>"),
        (Some(a), false) => format!(" [<{a}>]"),
        (None, _) => String::new(),
    };
    let mut out = format!("fabrewk {}{arg}", c.name);
    for f in c.flags.iter().filter(|f| **f != "help") {
        out += &match VALUES.iter().find(|v| v.0 == *f) {
            Some((f @ ("say" | "seen" | "answer"), word)) => format!(" --{f} <{word}>"),
            Some(("widen", word)) => format!(" [--widen <{word}>]…"),
            Some((f, word)) => format!(" [--{f} <{word}>]"),
            None => format!(" [--{f}]"),
        };
    }
    out
}

/// A file's fields as help and the reference print them, in `lang`: each with its depth, whether it may be absent
/// and its type.
fn format_rows(lang: &str, rows: Vec<crate::task::FormatLine>) -> String {
    let t = |k: &str, of: &str| say(lang, k, &[("of", of)]);
    let ty = |chain: &[&str]| chain.iter().rev().fold(String::new(), |of, w| t(&format!("type.{w}"), &of));
    let mut out = String::new();
    for (depth, field, optional, chain) in rows {
        let optional = if optional { format!(" {}", t("format.optional", "")) } else { String::new() };
        out += &format!("{}{field}{optional}: {}\n", "  ".repeat(depth), ty(&chain));
    }
    out
}

/// The format of `propose`'s file, in `lang`: the stage, then the tasks, each field with its type (part 7d3a).
pub fn file_format(lang: &str) -> String {
    let t = |k: &str, of: &str| say(lang, k, &[("of", of)]);
    let [stage, task] = crate::task::file_format();
    let mut out = format!("{}\n{}", t("format.stage", ""), format_rows(lang, stage));
    out += &format!("{}\n  name {}: {}\n", t("format.tasks", ""), t("format.optional", ""), t("format.name", ""));
    out + &format_rows(lang, task)
}

/// The format of `sweep`'s answer file, in `lang` (part 8a).
pub fn answer_format(lang: &str) -> String {
    let rows = crate::task::format_of(&crate::fermento::ANSWER);
    format!(
        "{}\n{}\n{}",
        say(lang, "format.answer", &[]),
        format_rows(lang, rows),
        say(lang, "format.answer-note", &[])
    )
}

/// An option as help lists it: `--say <words>`, `--json`.
fn option(f: &str) -> String {
    match VALUES.iter().find(|v| v.0 == f) {
        Some((_, word)) => format!("--{f} <{word}>"),
        None => format!("--{f}"),
    }
}

fn pad(rows: &[(String, String)]) -> String {
    let width = rows.iter().map(|r| r.0.chars().count()).max().unwrap_or(0);
    rows.iter().map(|(a, b)| format!("  {a:width$}  {b}\n")).collect()
}

/// Help in `lang`: the program's, or one command's.
pub fn help(lang: &str, command: Option<&Command>) -> String {
    let t = |k: &str| say(lang, k, &[]);
    let flag = |f: &str| (option(f), t(&flag_key(f)));
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
            if c.name == "propose" {
                out += &format!("\n{}:\n{}", t("help.format"), file_format(lang));
            }
            if c.name == "sweep" {
                out += &format!("\n{}:\n{}", t("help.format"), answer_format(lang));
            }
            let refusals = refusals(c);
            if !refusals.is_empty() {
                let rows: Vec<_> = refusals.iter().map(|r| (r.to_string(), t(&format!("code.{r}.reason")))).collect();
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
            quoted(&say("en", &flag_key(f), &[]))
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
            let d = quoted(&say("en", &flag_key(f), &[]));
            let r = if VALUES.iter().any(|v| v.0 == *f) { " -r" } else { "" };
            out += &format!("complete -c fabrewk -n '__fish_seen_subcommand_from {}' -l {f}{r} -d {d}\n", c.name);
        }
        if c.name == "propose" {
            out += "complete -c fabrewk -n '__fish_seen_subcommand_from propose' -F\n";
        }
        if c.name == "sweep" {
            out += "complete -c fabrewk -n '__fish_seen_subcommand_from sweep' -l answer -r -F\n";
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
            out += &format!("- `{}`: {}\n", option(f), t(&flag_key(f)));
        }
        let moves: Vec<String> = c.moves.iter().map(|m| format!("`{m}` ({})", t(&format!("kind.{m}")))).collect();
        let moves = if moves.is_empty() { t("reference.none") } else { moves.join(", ") + "." };
        out += &format!("\n{} {moves}\n", t("reference.moves"));
        let schemas: Vec<String> = c.schemas.iter().map(|s| format!("`{s}`")).collect();
        out += &format!("\n{} {}\n", t("reference.documents"), schemas.join(", "));
        if c.name == "propose" {
            out += &format!("\n{}\n\n```text\n{}```\n", t("help.format"), file_format(lang));
        }
        if c.name == "sweep" {
            out += &format!("\n{}\n\n```text\n{}```\n", t("help.format"), answer_format(lang));
        }
        let refusals = refusals(c);
        if !refusals.is_empty() {
            out += &format!("\n{}\n\n", t("help.refusals"));
            for r in refusals {
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
        out += &format!("- `fabrewk --{f}`: {}\n", t(&flag_key(f)));
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
