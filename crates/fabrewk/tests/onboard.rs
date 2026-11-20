//! `fabrewk onboard` through the built binary on a pseudo-terminal (REQ-INS-4, REQ-INS-8's questions; D-097 points 8
//! and 9; part 10c): the questions in the language chosen, the warning shown, the whole plan, and `apply`'s own path
//! only after the catalogue's yes and the person's words; any other answer, an empty one or the end of input applies
//! nothing; `TERMINAL_REQUIRED` with no terminal.

mod common;

use common::*;
use fabrewk_core::cli::say;
use serde_json::json;
use std::io::Write;
use std::process::{Command, Stdio};

/// Runs the binary with `args` on a new pseudo-terminal, its controlling terminal, typing `typed` and then the end of
/// input: what the terminal showed, its line ends as written, and the exit status.
const DRIVE: &str = r#"
import os, pty, sys
typed = sys.stdin.buffer.read()
pid, fd = pty.fork()
if pid == 0:
    os.execv(sys.argv[1], sys.argv[1:])
os.write(fd, typed + b"\x04")
out = b""
while True:
    try:
        chunk = os.read(fd, 65536)
    except OSError:
        break
    if not chunk:
        break
    out += chunk
_, status = os.waitpid(pid, 0)
sys.stdout.buffer.write(out)
sys.exit(os.waitstatus_to_exitcode(status))
"#;

/// `onboard` on a pseudo-terminal, each of `lines` typed and ended, with `env` added.
fn onboard_env(fx: &Fx, lines: &[&str], env: &[(&str, &str)]) -> Ran {
    let mut c = Command::new("python3");
    isolate(&mut c, &fx.home, None);
    c.envs(env.iter().copied()).current_dir(&fx.root).args([
        "-I",
        "-c",
        DRIVE,
        env!("CARGO_BIN_EXE_fabrewk"),
        "onboard",
    ]);
    let mut child = c.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    let typed: String = lines.iter().map(|l| format!("{l}\n")).collect();
    child.stdin.take().unwrap().write_all(typed.as_bytes()).unwrap();
    let out = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout).replace("\r\n", "\n");
    Ran { status: out.status.code().unwrap_or(-1), stdout, stderr: String::from_utf8_lossy(&out.stderr).into() }
}

fn onboard(fx: &Fx, lines: &[&str]) -> Ran {
    onboard_env(fx, lines, &[])
}

fn key(fx: &Fx) -> String {
    fx.keys.join("ana.pub").to_str().unwrap().to_owned()
}

/// The digest `assess` gives with `options`.
fn digest(fx: &Fx, options: &[&str]) -> String {
    let ran = fx.run(&[&["assess", "--json"], options].concat());
    assert_eq!(ran.status, 0, "{ran:?}");
    ran.doc()["digest"].as_str().unwrap().to_owned()
}

#[test]
fn req_ins_4_onboard_applies_the_plan_after_the_yes_and_the_words_in_spanish_and_english() {
    let cases = [
        ("es-MX", "unpublished", "sí", "Lo aplico yo.", "Un proyecto de prueba"),
        ("en", "published", "yes", "I apply it.", "A test project"),
    ];
    for (tag, mode, yes, words, purpose) in cases {
        let fx = product(&format!("onboard-{tag}"));
        let lang = &tag[..2];
        // A key given from the home is read as a shell reads it.
        std::fs::copy(fx.keys.join("ana.pub"), fx.home.join("ana.pub")).unwrap();
        let mut lines = vec![tag, "adopt", mode];
        if mode == "unpublished" {
            // The warning's exits are offered, and keeping it unpublished is an answer.
            lines.push("unpublished");
        }
        lines.extend(["ana", "~/ana.pub", "claude", purpose, "", "", yes, words]);
        let ran = onboard(&fx, &lines);
        assert_eq!(ran.status, 0, "{ran:?}");
        // The questions in the language chosen; the whole plan, the first alambique included.
        let q = |k: &str| say(lang, &format!("onboard.{k}"), &[]);
        for k in ["mode", "publication", "alias", "key", "tools", "purpose", "audience", "excluded", "confirm", "words"]
        {
            assert!(ran.stdout.contains(&q(k)), "{tag} {k}: {}", ran.stdout);
        }
        assert!(ran.stdout.contains(&say(lang, "skeleton.opening", &[]).trim_end().to_owned()), "{}", ran.stdout);
        let warned = ran.stdout.matches(&say(lang, "warning.title", &[])).count();
        assert_eq!(warned, if mode == "unpublished" { 2 } else { 0 }, "{}", ran.stdout);
        let events = fx.events("project");
        let kinds: Vec<&str> = events.iter().map(|e| e["kind"].as_str().unwrap()).collect();
        assert_eq!(kinds, ["assessed", "publication-decided", "policy-adopted", "installed"]);
        assert_eq!(fx.events("alambique").len(), 1);
        assert_eq!(events[1]["payload"], json!({"mode": mode, "warningShown": mode == "unpublished"}));
        assert_eq!(events[1]["say"], json!({"text": words, "lang": tag}));
        let alambique = std::fs::read_to_string(fx.domain.join("ALAMBIQUE.md")).unwrap();
        assert!(alambique.contains(&format!("\n{purpose}\n")), "{alambique}");
        assert_eq!(alambique.matches(&say(lang, "skeleton.empty", &[])).count(), 2, "{alambique}");
        // The answers as given, the key's path from the home; and the installation verifies.
        let home_key = fx.home.join("ana.pub").to_str().unwrap().to_owned();
        let answers = json!({"mode": "adopt", "publication": mode, "alias": "ana", "key": home_key, "tool": ["claude"],
            "lang": tag, "purpose": purpose});
        assert_eq!(events[0]["payload"]["plan"]["answers"], answers);
        assert_eq!(fx.run(&["check"]).status, 0);
    }
}

#[test]
fn req_ins_4_onboard_reaches_the_digest_assess_gives_with_the_same_answers() {
    let fx = product("onboard-digest");
    let k = key(&fx);
    let expected = digest(&fx, &["--publication", "published", "--alias", "ana", "--key", &k, "--lang", "en"]);
    let ran = onboard(&fx, &["en", "adopt", "published", "ana", &k, "", "", "", "", "yes", "Mine."]);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(fx.events("project")[0]["payload"]["digest"], json!(expected));
}

#[test]
fn req_ins_4_no_yes_no_words_or_the_end_of_input_applies_nothing() {
    let fx = product("onboard-no");
    let k = key(&fx);
    let before = fx.snapshot();
    let to = |more: &[&'static str]| -> Vec<String> {
        let mut v: Vec<String> =
            ["en", "adopt", "published", "ana", &k, "codex claude", "", "", ""].map(String::from).into();
        v.extend(more.iter().map(|s| s.to_string()));
        v
    };
    let cases: Vec<Vec<String>> = vec![
        vec![],
        vec!["".into()],
        vec!["fr".into()],
        vec!["en".into(), "".into()],
        vec!["en".into(), "administer".into()],
        vec!["en".into(), "adopt".into(), "public".into()],
        vec!["en".into(), "adopt".into(), "unpublished".into(), "".into()],
        vec!["en".into(), "adopt".into(), "published".into(), "ana".into(), "".into()],
        vec!["en".into(), "adopt".into(), "published".into(), "ana".into(), k.clone(), "cursor".into()],
        to(&[]),
        to(&[""]),
        to(&["no"]),
        to(&["y"]),
        to(&["yes please"]),
        // Another catalogue's yes is no yes in this one.
        to(&["sí"]),
        to(&["yes"]),
        to(&["yes", ""]),
    ];
    for lines in &cases {
        let lines: Vec<&str> = lines.iter().map(String::as_str).collect();
        let ran = onboard(&fx, &lines);
        assert_eq!(ran.status, 0, "{lines:?}: {ran:?}");
        assert!(ran.stdout.trim_end().ends_with(&say("en", "onboard.stopped", &[])), "{lines:?}: {}", ran.stdout);
        assert_eq!(fx.snapshot(), before, "{lines:?}");
    }
    // In Spanish, both spellings are a yes, and the answer's case is no matter; `yes` is not.
    let es = |yes: &'static str| -> Vec<String> {
        let mut v: Vec<String> = ["es", "adopt", "published", "ana", &k, "", "", "", ""].map(String::from).into();
        v.extend([yes.to_owned(), String::new()]);
        v
    };
    for yes in ["yes", "sí", "SI"] {
        let lines = es(yes);
        let lines: Vec<&str> = lines.iter().map(String::as_str).collect();
        let ran = onboard(&fx, &lines);
        let words = ran.stdout.contains(&say("es", "onboard.words", &[]));
        assert_eq!((ran.status, words), (0, yes != "yes"), "{yes}: {}", ran.stdout);
        assert!(ran.stdout.trim_end().ends_with(&say("es", "onboard.stopped", &[])), "{yes}: {}", ran.stdout);
        assert_eq!(fx.snapshot(), before, "{yes}");
    }
}

#[test]
fn req_ins_4_a_plan_that_cannot_be_applied_is_shown_and_no_yes_is_asked() {
    let fx = product("onboard-review");
    // Git does not sign with the key: its commands are shown at the key's question, and none is run.
    fx.git(&["config", "--unset", "user.signingkey"]);
    let k = key(&fx);
    let before = fx.snapshot();
    for (mode, why) in [("review", "cannot-review"), ("adopt", "apply")] {
        let ran = onboard(&fx, &["en", mode, "published", "ana", &k, "", "", "", "", "no"]);
        assert_eq!(ran.status, 0, "{ran:?}");
        let sign = ran.stdout.find(&say("en", "onboard.sign", &[])).expect(&ran.stdout);
        assert!(ran.stdout[sign..].contains(&format!("git config user.signingkey '{k}'")), "{}", ran.stdout);
        assert!(sign < ran.stdout.find(&say("en", "onboard.tools", &[])).unwrap());
        assert!(ran.stdout.contains(&say("en", &format!("assess.{why}"), &[])), "{}", ran.stdout);
        let confirm = ran.stdout.contains(&say("en", "onboard.confirm", &[]));
        assert_eq!(confirm, mode == "adopt", "{}", ran.stdout);
        assert_eq!(fx.snapshot(), before);
    }
}

#[test]
fn req_ins_8_an_alambique_present_is_kept_and_its_questions_are_not_asked() {
    let fx = product("onboard-alambique");
    std::fs::create_dir_all(&fx.domain).unwrap();
    std::fs::write(fx.domain.join("ALAMBIQUE.md"), "# Ours\n").unwrap();
    fx.commit("alambique");
    let k = key(&fx);
    let ran = onboard(&fx, &["en", "adopt", "published", "ana", &k, "", "yes", "Mine."]);
    assert_eq!(ran.status, 0, "{ran:?}");
    for q in ["purpose", "audience", "excluded"] {
        assert!(!ran.stdout.contains(&say("en", &format!("onboard.{q}"), &[])), "{q}: {}", ran.stdout);
    }
    assert_eq!(std::fs::read_to_string(fx.domain.join("ALAMBIQUE.md")).unwrap(), "# Ours\n");
    assert_eq!(fx.events("project").len(), 4);
}

#[test]
fn onboard_s_refusals_are_assess_s_in_the_language_chosen_and_come_before_the_next_question() {
    let fx = product("onboard-refused");
    let k = key(&fx);
    let before = fx.snapshot();
    let ran = onboard(&fx, &["es", "adopt", "published", "Ana Pérez", &k, "", "", "", "", "sí", "Lo aplico."]);
    assert_eq!(ran.status, 1, "{ran:?}");
    let reason = say("es", "code.ALIAS_INVALID.reason", &[]);
    assert!(ran.stdout.contains(&format!("ALIAS_INVALID: {reason}")), "{}", ran.stdout);
    assert!(!ran.stdout.contains(&say("es", "onboard.tools", &[])), "{}", ran.stdout);
    assert_eq!(fx.snapshot(), before);
    // Outside a repository, the language is the only question.
    let outside = Fx {
        root: fx.keys.clone(),
        domain: fx.keys.join(".brewstillery"),
        home: fx.home.clone(),
        keys: fx.keys.clone(),
        path: None,
    };
    let ran = onboard(&outside, &["en", "adopt"]);
    std::mem::forget(outside);
    assert_eq!(ran.status, 1, "{ran:?}");
    assert!(ran.stdout.contains("NOT_A_REPOSITORY: ") && !ran.stdout.contains(&say("en", "onboard.mode", &[])));
}

#[test]
fn terminal_required_without_a_controlling_terminal_and_agent_environment_before_it() {
    let fx = product("onboard-tty");
    let before = fx.snapshot();
    // A new session has no controlling terminal: `/dev/tty` cannot be opened. `onboard` takes no `--json` (D-097
    // point 2), so the code is asserted in the human output alone.
    for (env, code) in [(&[][..], "TERMINAL_REQUIRED"), (&[("CLAUDECODE", "1")][..], "AGENT_ENVIRONMENT")] {
        let mut c = Command::new("setsid");
        isolate(&mut c, &fx.home, None);
        c.envs(env.iter().copied()).current_dir(&fx.root).args(["-w", env!("CARGO_BIN_EXE_fabrewk"), "onboard"]);
        let out = c.stdin(Stdio::null()).output().unwrap();
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert_eq!(out.status.code(), Some(1), "{stderr}");
        assert!(stderr.starts_with(&format!("{code}: ")), "{stderr}");
        assert_eq!(fx.snapshot(), before);
    }
    let usage = fx.run(&["onboard", "--json"]);
    assert_eq!((usage.status, usage.doc()["schema"].clone()), (2, json!("fabrewk.usage/1")), "{usage:?}");
    // Inside an agent, with a terminal, nothing is asked.
    let ran = onboard_env(&fx, &["en"], &[("CLAUDECODE", "1")]);
    // The terminal echoes what is typed; nothing else comes before the refusal.
    assert!(ran.stdout.starts_with("en\nAGENT_ENVIRONMENT: "), "{ran:?}");
    for lang in ["en", "es"] {
        assert!(!ran.stdout.contains(&say(lang, "onboard.language", &[])), "{ran:?}");
    }
    assert_eq!((ran.status, fx.snapshot() == before), (1, true), "{ran:?}");
}

#[test]
fn p10c_1_to_3_answers_are_kept_as_given_and_choices_are_taken_in_any_case() {
    let fx = product("onboard-given");
    let k = key(&fx);
    let before = fx.snapshot();
    // An alias with spaces is the answer given, which `assess` refuses as it refuses `--alias ' ana '` (P10C-1).
    let ran = onboard(&fx, &["en", "adopt", "published", " ana ", &k, "", "", "", "", "yes", "Mine."]);
    assert_eq!(ran.status, 1, "{ran:?}");
    assert!(ran.stdout.contains("ALIAS_INVALID: "), "{}", ran.stdout);
    assert_eq!(fx.snapshot(), before);
    // Words of spaces alone are words given, which `apply` refuses (`WORDS_MISSING`), with nothing written.
    let ran = onboard(&fx, &["en", "adopt", "published", "ana", &k, "", "", "", "", "yes", "   "]);
    assert!(ran.status == 1 && ran.stdout.contains("WORDS_MISSING: "), "{ran:?}");
    assert_eq!(fx.snapshot(), before);
    // A key's file name ending in a space is the path given (P10C-1): the plan shown is `assess`'s with that path.
    let spaced = format!("{k} ");
    std::fs::copy(&k, &spaced).unwrap();
    let base = ["--publication", "published", "--alias", "ana", "--lang", "en"];
    let expected = digest(&fx, &[&base[..], &["--key", &spaced]].concat());
    let ran = onboard(&fx, &["en", "adopt", "published", "ana", &spaced, "", "", "", "", "no"]);
    assert!(ran.status == 0 && ran.stdout.contains(&expected), "{ran:?}");
    assert_eq!(fx.snapshot(), before);
    // The tools in any case (P10C-3); an answer of spaces alone kept as given in the alambique (P10C-2); the plan
    // applied is the one `assess` gives with the same answers as options.
    let more = ["--key", &k, "--tool", "claude", "--tool", "codex", "--purpose", "   ", "--audience", "Us"];
    let expected = digest(&fx, &[&base[..], &more].concat());
    let lines = ["en", "ADOPT", " Published ", "ana", &k, "CLAUDE, Codex", "   ", "Us", "", "YES", "Mine."];
    let ran = onboard(&fx, &lines);
    assert_eq!(ran.status, 0, "{ran:?}");
    let assessed = &fx.events("project")[0]["payload"];
    assert_eq!(assessed["digest"], json!(expected));
    assert_eq!(assessed["plan"]["answers"]["tool"], json!(["claude", "codex"]));
    assert_eq!(assessed["plan"]["answers"]["purpose"], json!("   "));
    let alambique = std::fs::read_to_string(fx.domain.join("ALAMBIQUE.md")).unwrap();
    assert!(alambique.contains("\n   \n"), "{alambique}");
}
