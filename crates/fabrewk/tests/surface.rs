//! The command surface (part 7d2, D-053, D-055): the one declaration of commands and what is generated from it
//! (REQ-CLI-1), the catalogue (REQ-CLI-2; PD-34, PD-36, PD-38), the version (11.2 rule 8). Run with
//! `FABREWK_REGENERATE=1` to rewrite the generated files after a change to the declaration or the catalogue.

mod common;

use common::*;
use fabrewk_core::cli::{self, CATALOGUES, CHECKS, COMMANDS, PROGRAM_FLAGS, Parsed, messages};
use fabrewk_core::record::KINDS;
use fabrewk_core::settings::KEYS;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

fn crate_file(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

/// The generated files, each with its fresh generation.
fn generated() -> Vec<(PathBuf, String)> {
    let mut out = vec![
        (crate_file("completions/fabrewk.fish"), cli::fish()),
        (crate_file("completions/fabrewk.sh"), cli::posix()),
    ];
    for (lang, _) in CATALOGUES {
        out.push((crate_file(&format!("doc/reference.{lang}.md")), cli::reference(lang)));
    }
    out
}

fn parse(args: &[&str]) -> Result<Parsed, cli::Usage> {
    cli::parse(args.iter().map(std::ffi::OsString::from))
}

fn shell_ok(shell: &str, flag: &str, script: &str) -> bool {
    let out = Command::new(shell).args([flag, "-c", script]).output().unwrap_or_else(|_| panic!("{shell} is needed"));
    out.status.success()
}

#[test]
fn req_cli_1_generated_files_equal_a_fresh_generation() {
    for (path, fresh) in generated() {
        if std::env::var_os("FABREWK_REGENERATE").is_some() {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, &fresh).unwrap();
        }
        let kept = std::fs::read_to_string(&path).unwrap_or_default();
        assert_eq!(kept, fresh, "{} differs from a fresh generation; run with FABREWK_REGENERATE=1", path.display());
    }
}

#[test]
fn req_cli_1_every_command_appears_in_help_completions_and_reference() {
    let fish = cli::fish();
    let posix = cli::posix();
    for (lang, _) in CATALOGUES {
        let help = cli::help(lang, None);
        let reference = cli::reference(lang);
        for c in COMMANDS {
            assert!(help.contains(&format!("  {} ", &cli::synopsis(c)["fabrewk ".len()..])), "{lang} help: {}", c.name);
            assert!(reference.contains(&format!("## `{}`", cli::synopsis(c))), "{lang} reference: {}", c.name);
            let one = cli::help(lang, Some(c));
            assert!(one.contains(&cli::synopsis(c)), "{lang} help of {}", c.name);
        }
        for check in CHECKS {
            assert!(reference.contains(&format!("| {} |", check.id)), "{lang} reference: {}", check.id);
        }
    }
    for c in COMMANDS {
        assert!(fish.contains(&format!("-a {} ", c.name)), "fish: {}", c.name);
        assert!(posix.contains(&format!("        {}) ", c.name)), "posix: {}", c.name);
    }
    // Only the commands that exist (D-053 point 3): nothing of a later phase.
    for later in ["accept", "approve", "close"] {
        assert!(COMMANDS.iter().all(|c| c.name != later), "{later}");
        assert!(!fish.contains(&format!("-a {later} ")) && !posix.contains(&format!(" {later})")), "{later}");
    }
}

#[test]
fn req_cli_1_the_completions_run_in_their_shells() {
    let dir = tmp("completions");
    let (fish, posix) = (dir.join("fabrewk.fish"), dir.join("fabrewk.sh"));
    std::fs::write(&fish, cli::fish()).unwrap();
    std::fs::write(&posix, cli::posix()).unwrap();
    let ok = |c: &mut Command| c.output().unwrap().status.success();
    assert!(ok(Command::new("fish").arg("--no-execute").arg(&fish)), "fish --no-execute");
    assert!(ok(Command::new("dash").arg("-n").arg(&posix)), "dash -n");
    // In bash, the POSIX file completes each command, then each command's options and `config`'s keys.
    let complete = |words: &str, cword: usize| {
        let script = format!(
            ". '{}'; COMP_WORDS=({words}); COMP_CWORD={cword}; _fabrewk fabrewk \"${{COMP_WORDS[{cword}]}}\" x; printf '%s\\n' \"${{COMPREPLY[@]}}\"",
            posix.display()
        );
        let out = Command::new("bash").args(["--norc", "-c", &script]).output().unwrap();
        assert!(out.status.success());
        String::from_utf8(out.stdout).unwrap().lines().map(str::to_owned).collect::<Vec<_>>()
    };
    assert_eq!(complete("fabrewk st", 1), ["status"]);
    let all = [
        "status",
        "config",
        "check",
        "adopt",
        "propose",
        "authorize",
        "reject",
        "run",
        "sweep",
        "discard",
        "--help",
        "--version",
    ];
    assert_eq!(complete("fabrewk ''", 1), all);
    assert_eq!(complete("fabrewk authorize --s", 2), ["--seen", "--say", "--say-lang"]);
    assert_eq!(complete("fabrewk adopt --w", 2), ["--widen"]);
    assert_eq!(complete("fabrewk check --", 2), ["--json", "--help"]);
    assert!(complete("fabrewk config limits.", 2).contains(&"limits.rounds".to_owned()));
    // In fish, the file completes the commands.
    let script = format!("source '{}'; complete -C 'fabrewk ch'", fish.display());
    let out = Command::new("fish").args(["--no-config", "-c", &script]).output().unwrap();
    assert!(String::from_utf8(out.stdout).unwrap().starts_with("check\t"), "fish completes check");
}

#[test]
fn req_cli_1_the_parser_accepts_exactly_the_declaration() {
    for c in COMMANDS {
        // A required argument, and `--seen` where the command takes it, are in every accepted line; help needs none.
        let mut base = if c.required { vec![c.name, "x-1"] } else { vec![c.name] };
        if c.flags.contains(&"seen") {
            base.extend(["--seen", "d"]);
        }
        if c.flags.contains(&"answer") {
            base.extend(["--answer", "a.json"]);
        }
        let with = |more: &[&'static str]| [base.clone(), more.to_vec()].concat();
        let mut ok = vec![base.clone(), vec![c.name, "--help"]];
        for f in c.flags {
            ok.push(match *f {
                "json" => with(&["--json"]),
                "say" => with(&["--say", "words"]),
                "say-lang" => with(&["--say-lang", "es-MX"]),
                "widen" => with(&["--widen", "a.b", "--widen", "c.d"]),
                "run" => with(&["--run"]),
                _ => with(&["--help"]),
            });
        }
        if c.arg.is_some() && !c.required {
            ok.push(vec![c.name, "x-1"]);
        }
        if c.arg.is_some() {
            ok.push([&[c.name, "x-1", "--json"], base.get(2..).unwrap_or(&[])].concat());
            ok.push([&["--json", c.name, "x-1"], base.get(2..).unwrap_or(&[])].concat());
        }
        for args in &ok {
            assert!(parse(args).is_ok(), "{args:?}");
        }
        let mut bad = vec![with(&["--nope"]), with(&["-j"]), with(&["--version"]), with(&["--json=1"])];
        bad.push(if c.arg.is_some() { vec![c.name, "a", "b"] } else { vec![c.name, "a"] });
        if c.required {
            bad.push(vec![c.name]);
            bad.push(vec![c.name, "--json"]);
        }
        if c.flags.contains(&"seen") {
            bad.push(vec![c.name, "x-1"]);
            bad.push(vec![c.name, "x-1", "--seen"]);
            bad.push(with(&["--seen", "e"]));
        }
        if c.flags.contains(&"answer") {
            bad.push(vec![c.name, "x-1"]);
            bad.push(vec![c.name, "x-1", "--answer"]);
        }
        if !c.flags.contains(&"run") {
            bad.push(with(&["--run"]));
        }
        if !c.flags.contains(&"say") {
            bad.push(with(&["--say", "words"]));
        }
        for args in &bad {
            assert!(parse(args).is_err(), "{args:?}");
        }
    }
    // The options that take a value (D-062): `--say` keeps its words as given, absent or empty, for the command to
    // refuse; `--say-lang` takes one tag; only `--widen` repeats.
    let run = |args: &[&str]| match parse(args) {
        Ok(Parsed::Run { options, .. }) => options,
        other => panic!("{args:?}: {other:?}"),
    };
    assert_eq!(run(&["adopt"]).say, None);
    assert_eq!(run(&["adopt", "--say"]).say, Some("".into()));
    assert_eq!(run(&["adopt", "--say", "--json"]).say, Some("".into()));
    assert_eq!(run(&["adopt", "--say=--json"]).say, Some("--json".into()));
    assert!(parse(&["adopt", "--say", "-"]).is_err() && parse(&["adopt", "--say", "-x"]).is_err());
    assert_eq!(run(&["adopt", "--say=Sí."]).say, Some("Sí.".into()));
    let o = run(&["adopt", "--widen", "a.b", "--say-lang", "es-MX", "--widen=c.d"]);
    assert_eq!((o.widen, o.say_lang), (vec!["a.b".to_owned(), "c.d".to_owned()], Some("es-MX".to_owned())));
    for bad in [
        &["adopt", "--say", "a", "--say", "b"][..],
        &["adopt", "--say-lang", "es", "--say-lang", "en"],
        &["adopt", "--say-lang", "not a tag"],
        &["adopt", "--say-lang"],
        &["adopt", "--widen"],
        &["adopt", "--widen", "--json"],
    ] {
        assert!(parse(bad).is_err(), "{bad:?}");
    }
    use std::os::unix::ffi::OsStringExt;
    let raw = |v: Vec<u8>| vec!["adopt".into(), "--say".into(), std::ffi::OsString::from_vec(v)];
    assert!(matches!(cli::parse(raw(vec![0xff])), Ok(Parsed::Run { options, .. }) if options.say.is_some()));
    let widen = vec!["adopt".into(), "--widen".into(), std::ffi::OsString::from_vec(vec![0xff])];
    assert!(cli::parse(widen).is_err());
    for args in [&["--version"][..], &["--version", "--json"], &["--json", "--version"], &["--help"], &[]] {
        let r = parse(args);
        assert_eq!(r.is_ok(), !args.is_empty(), "{args:?}");
    }
    for args in [&["--json"][..], &["nosuch"], &["--version", "status"], &["help"], &["-h"], &["--version", "--help"]] {
        assert!(parse(args).is_err(), "{args:?}");
    }
    assert!(matches!(parse(&["status", "--help", "--json"]), Ok(Parsed::Help(Some(c))) if c.name == "status"));
    assert!(
        matches!(parse(&["config", "limits.rounds"]), Ok(Parsed::Run { arg: Some(a), .. }) if a == "limits.rounds")
    );
    // A name that is not a command, and bytes that are not UTF-8, are usage errors.
    assert!(cli::parse([std::ffi::OsString::from_vec(vec![0xff])]).is_err());
}

/// Every code the core raises, named as the catalogue names it: each variant of `Code`, in upper snake case.
fn codes() -> Vec<String> {
    let source = include_str!("../../core/src/code.rs");
    let body = &source[source.find("pub enum Code {").unwrap() + 15..];
    let variants = body[..body.find('}').unwrap()].lines().map(str::trim).filter_map(|l| l.strip_suffix(','));
    let snake = |v: &str| {
        let mut out = String::new();
        for (i, c) in v.char_indices() {
            if i > 0 && c.is_ascii_uppercase() {
                out.push('_');
            }
            out.push(c.to_ascii_uppercase());
        }
        out
    };
    let out: Vec<String> = variants.map(snake).collect();
    assert!(out.len() > 40 && out.contains(&"RECORD_UNCOMMITTED".to_owned()));
    out
}

/// Every message key of each catalogue, without the descriptions.
fn keys(text: &str) -> BTreeSet<String> {
    messages(text).into_keys().filter(|k| !k.ends_with(".about")).collect()
}

/// Every message the code can render, by key, with a value for each placeholder.
fn rendered(lang: &str) -> Vec<(String, String)> {
    let params: Vec<(&str, &str)> = [
        "word",
        "path",
        "check",
        "about",
        "severity",
        "control",
        "errors",
        "warnings",
        "streams",
        "events",
        "digest",
        "mode",
        "id",
        "state",
        "goal",
        "stage",
        "what",
        "kind",
        "by",
        "at",
        "moves",
        "alias",
        "role",
        "tool",
        "name",
        "touch",
        "tools",
        "from",
        "version",
        "verified",
        "seq",
        "stream",
        "commit",
        "of",
        "keys",
        "n",
        "build",
        "review",
        "key",
        "destination",
        "number",
        "point",
        "target",
        "names",
        "m",
        "points",
        "recorded",
        "withdrawn",
    ]
    .iter()
    .map(|p| (*p, "x"))
    .collect();
    let en = messages(CATALOGUES[0].1);
    en.keys().filter(|k| !k.ends_with(".about")).map(|k| (k.clone(), cli::say(lang, k, &params))).collect()
}

#[test]
fn pd_34_every_key_has_words_in_every_catalogue() {
    let en = messages(CATALOGUES[0].1);
    assert!(!en.is_empty() && CATALOGUES[0].0 == "en");
    // Every table of messages in `en` has its description; every key is in both catalogues.
    for k in keys(CATALOGUES[0].1) {
        let mut prefixes = k.match_indices('.').map(|(i, _)| &k[..i]);
        assert!(
            prefixes.any(|t| en.get(&format!("{t}.about")).is_some_and(|a| !a.is_empty())),
            "{k} has no description"
        );
    }
    for (lang, text) in CATALOGUES {
        assert_eq!(keys(text), keys(CATALOGUES[0].1), "{lang}'s keys");
    }
    // Every code, state, event kind, role, actor, setting, check, flag and command has words.
    let mut wanted: Vec<String> = Vec::new();
    for code in codes() {
        wanted.extend(["reason", "next"].map(|f| format!("code.{code}.{f}")));
    }
    for subject in ["task", "stage", "proposal"] {
        wanted.extend(fabrewk_core::table::states(subject).iter().map(|s| format!("state.{s}")));
    }
    wanted.extend(KINDS.iter().map(|k| format!("kind.{}", k.0)));
    wanted.extend(["maestro", "supervisor", "contributor"].map(|r| format!("role.{r}")));
    wanted.extend(["person", "agent", "fabrewk", "extension"].map(|a| format!("actor.{a}")));
    wanted.extend(["published", "own-repository", "unpublished"].map(|m| format!("mode.{m}")));
    wanted.extend(KEYS.iter().map(|k| format!("setting.{}", k.path)));
    wanted.extend(CHECKS.iter().map(|c| format!("check.{}", c.id)));
    wanted.extend(PROGRAM_FLAGS.iter().map(|f| cli::flag_key(f)));
    for c in COMMANDS {
        wanted.push(format!("command.{}.summary", c.name));
        wanted.extend(c.arg.map(|_| format!("command.{}.arg", c.name)));
        wanted.extend(c.flags.iter().map(|f| cli::flag_key(f)));
        wanted.extend(c.refusals.iter().map(|r| format!("code.{r}.about")));
    }
    wanted.extend(cli::EXITS.iter().map(|e| format!("exit.{e}")));
    for (lang, text) in CATALOGUES {
        let m = messages(text);
        for k in &wanted {
            let held = if k.ends_with(".about") { en.contains_key(k) } else { m.contains_key(k) };
            assert!(held, "{lang} lacks {k}");
        }
    }
    // Every key the command surface's code names is in the catalogue.
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../core/src/cli");
    let groups = [
        "ui",
        "usage",
        "internal",
        "help",
        "flag",
        "reference",
        "check",
        "status",
        "config",
        "source",
        "how",
        "adopt",
        "propose",
        "write",
        "format",
        "type",
        "words",
    ];
    for file in std::fs::read_dir(dir).unwrap() {
        let text = std::fs::read_to_string(file.unwrap().path()).unwrap();
        for literal in text.split('"').skip(1).step_by(2) {
            let group = literal.split('.').next().unwrap();
            if groups.contains(&group) && literal.contains('.') && !literal.contains(['{', ' ', '/']) {
                assert!(en.contains_key(literal), "the code names {literal}, which the catalogue lacks");
            }
        }
    }
}

#[test]
fn pd_34_pd_36_rendered_messages_are_whole() {
    for (lang, _) in CATALOGUES {
        for (key, text) in rendered(lang) {
            assert!(!text.is_empty() && text != key, "{lang} {key}: bare");
            assert!(!text.contains(['{', '}']), "{lang} {key}: a placeholder left: {text}");
            for doubled in ["..", ",,", ". .", " ,", " . ", "::", "  "] {
                assert!(!text.contains(doubled) && !text.ends_with(" ."), "{lang} {key}: {doubled:?} in {text}");
            }
        }
    }
    // A key no catalogue holds renders as the catalogue's own sentence, never as the bare key.
    assert_eq!(cli::say("es", "no.such.key", &[]), cli::say("es", "internal.missing", &[]));
    assert_eq!(cli::say("es-MX", "ui.next", &[]), cli::say("en", "ui.next", &[]));
}

/// Every command a message prints: the `run` fields, and the commands of the declaration's synopses with their
/// arguments filled.
fn printed() -> Vec<String> {
    let mut out = Vec::new();
    for (lang, text) in CATALOGUES {
        for (k, v) in messages(text) {
            if k.ends_with(".run") {
                out.push(v);
            } else if !k.ends_with(".about") {
                assert!(!v.contains("fabrewk "), "{lang} {k}: a command outside a run field: {v}");
            }
        }
    }
    for c in COMMANDS {
        let arg = if c.required { " x" } else { "" };
        let seen = if c.flags.contains(&"seen") { " --seen d" } else { "" };
        let answer = if c.flags.contains(&"answer") { " --answer a.json" } else { "" };
        out.push(format!("fabrewk {}{arg}{seen}{answer}", c.name));
        out.push(format!("fabrewk {} --help", c.name));
    }
    out.extend(["fabrewk --help", "fabrewk --version"].map(str::to_owned));
    out
}

#[test]
fn pd_38_every_printed_command_parses_and_runs_in_fish_and_sh() {
    let commands = printed();
    assert!(commands.iter().any(|c| c == "fabrewk config"));
    for command in &commands {
        let words: Vec<&str> = command.split(' ').collect();
        assert_eq!(words[0], "fabrewk", "{command}");
        assert!(parse(&words[1..]).is_ok(), "the declaration refuses {command}");
        assert!(shell_ok("fish", "--no-execute", command), "fish: {command}");
        assert!(shell_ok("dash", "-n", command), "dash: {command}");
    }
    // Every option any message names is one the declaration lists.
    let mut flags: BTreeSet<&str> = PROGRAM_FLAGS.into_iter().collect();
    flags.extend(COMMANDS.iter().flat_map(|c| c.flags.iter().copied()));
    for (lang, text) in CATALOGUES {
        // The separator's marker is an HTML comment, which holds `--` and names no option.
        for (k, v) in messages(text).into_iter().filter(|(k, _)| !k.ends_with(".about") && k != "fermento.separator") {
            for named in v.split("--").skip(1) {
                let name: String = named.chars().take_while(|c| c.is_ascii_lowercase() || *c == '-').collect();
                assert!(flags.contains(name.as_str()), "{lang} {k} names --{name}");
            }
        }
    }
}

#[test]
fn version_and_its_features() {
    let dir = tmp("version");
    let out = run_in(&dir, &dir, &["--version"], None);
    assert_eq!((out.status, out.stdout.as_str()), (0, format!("fabrewk {}\n", env!("CARGO_PKG_VERSION")).as_str()));
    let doc = run_in(&dir, &dir, &["--version", "--json"], None).doc();
    assert_eq!(doc["schema"], "fabrewk.version/1");
    assert_eq!(doc["version"], env!("CARGO_PKG_VERSION"));
    // 11.2 rule 8: the build lists its features; only the crash suite's feature can be one.
    let features = doc["features"].as_array().unwrap();
    assert!(features.iter().all(|f| f == "fault-injection"), "{features:?}");
    // A release build carries no test seam (P7D2-16): run this test with `--release` on the release binary.
    if !cfg!(debug_assertions) {
        assert!(features.is_empty(), "a release build with {features:?}");
    }
}

#[test]
fn usage_errors_say_what_is_wrong() {
    let dir = tmp("usage-words");
    let says = |args: &[&str], key: &str, word: &str| {
        let out = run_in(&dir, &dir, args, None);
        assert_eq!(out.status, 2, "{args:?}");
        assert!(out.all().contains(&cli::say("en", key, &[("word", word)])), "{args:?}: {out:?}");
    };
    says(&[], "usage.none", "");
    says(&["--json"], "usage.flag", "--json");
    says(&["status", "--json=1"], "usage.value", "--json");
    says(&["nosuch"], "usage.command", "nosuch");
    says(&["check", "extra"], "usage.extra", "extra");
    // `authorize` needs the digest of what the person read (D-062, D-068): without it, a usage error, never a move.
    says(&["authorize", "t-00000000", "--say", "Sí."], "usage.needed", "--seen");
    // Help lists refusals only for a command that has them.
    let refusals = format!("{}:", cli::say("en", "help.refusals", &[]));
    let status = COMMANDS.iter().find(|c| c.name == "status").unwrap();
    let config = COMMANDS.iter().find(|c| c.name == "config").unwrap();
    assert!(!cli::help("en", Some(status)).contains(&refusals));
    // Only `propose`'s help prints its file's format.
    let format = format!("{}:", cli::say("en", "help.format", &[]));
    let propose = COMMANDS.iter().find(|c| c.name == "propose").unwrap();
    assert!(cli::help("en", Some(propose)).contains(&format) && !cli::help("en", Some(status)).contains(&format));
    assert!(cli::help("en", Some(config)).contains(&refusals) && cli::help("en", Some(config)).contains("KEY_UNKNOWN"));
}
