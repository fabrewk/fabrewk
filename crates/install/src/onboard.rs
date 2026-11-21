//! `onboard`'s interview (REQ-INS-4, D-097 point 8; part 10c): the language first, then, in it, the mode, the
//! publication with point 9's warning and its exits, the alias, the key with Git's commands to sign with it, the tools
//! and, when no alambique exists, REQ-INS-8's three questions; then the assessment with those answers, the whole plan,
//! the explicit yes and the person's words. An answer outside a question's choices, an empty one where an answer is
//! needed, or the end of input, at any question, applies nothing, says so, and gives no plan.

use fabrewk_core::cli::{CATALOGUES, Options, catalogue, printable, say};
use fabrewk_core::install::{Asked, Interview};
use fabrewk_core::{Error, settings::is_tag};
use serde_json::Value;

/// The terminal's failure, as an internal error on it.
fn io(e: std::io::Error) -> Error {
    Error::Io { path: "/dev/tty".into(), error: e }
}

/// Writes `text` on the terminal as a person reads it: no hidden character acts on it.
fn put(i: &mut Interview, text: &str) -> Result<(), Error> {
    i.out.write_all(printable(text).as_bytes()).and_then(|()| i.out.flush()).map_err(io)
}

/// Asks `question` and gives the line answered without its end; `None` at the end of input, even inside the line
/// (P10C-9), or for a line the terminal cannot give as UTF-8, which no choice is: either applies nothing.
fn ask(i: &mut Interview, question: &str) -> Result<Option<String>, Error> {
    put(i, &format!("{question} "))?;
    let mut line = String::new();
    // Only the line's end is not the answer: a carriage return typed literally is (P10C-5).
    Ok(i.tty.read_line(&mut line).ok().and_then(|_| line.strip_suffix('\n')).map(str::to_owned))
}

/// Asks `question`, its choices shown after it, and gives the one answered.
fn choose(i: &mut Interview, question: &str, choices: &[&str]) -> Result<Option<String>, Error> {
    let answer = ask(i, &format!("{question} ({})", choices.join(", ")))?;
    Ok(answer.map(|a| a.trim().to_lowercase()).filter(|a| choices.contains(&a.as_str())))
}

/// Asks `question` where an answer is needed: an empty one is none; any other is kept as given (P10C-1).
fn needed(i: &mut Interview, question: &str) -> Result<Option<String>, Error> {
    Ok(ask(i, question)?.filter(|a| !a.is_empty()))
}

/// The assessment with the answers so far: the document and text `assess` gives with them as its options.
fn assessed(i: &Interview, o: &Options) -> Result<(Value, String), Error> {
    super::assessed(&Asked { root: i.root, home: i.home, options: o, adapters: i.adapters })
}

pub fn interview(i: &mut Interview, o: &mut Options) -> Result<Option<String>, Vec<Error>> {
    asked(i, o).map_err(|e| vec![e])
}

fn asked(i: &mut Interview, o: &mut Options) -> Result<Option<String>, Error> {
    // The first question, before a language is chosen, in every catalogue's words.
    let langs: Vec<&str> = CATALOGUES.iter().map(|c| c.0).collect();
    let first: Vec<String> = langs.iter().map(|l| say(l, "onboard.language", &[])).collect();
    let tag = ask(i, &format!("{} ({})", first.join(" / "), langs.join(", ")))?.map(|t| t.trim().to_owned());
    // A tag whose language has a catalogue, as `--lang` takes it (`es-MX` reaches `es`).
    let known =
        |t: &str| is_tag(t) && langs.iter().any(|l| t.split('-').next().is_some_and(|p| p.eq_ignore_ascii_case(l)));
    let Some(tag) = tag.filter(|t| known(t)) else { return stop(i, "en") };
    let lang = catalogue(&tag.to_ascii_lowercase());
    o.answers.insert("lang", tag);
    // Where `assess` refuses (no repository, an include of the private folder), nothing more is asked.
    assessed(i, o)?;
    let q = |k: &str| say(lang, &format!("onboard.{k}"), &[]);
    let Some(mode) = choose(i, &q("mode"), &["adopt", "review"])? else { return stop(i, lang) };
    o.answers.insert("mode", mode);
    let modes = ["published", "own-repository", "unpublished"];
    let mut publication = choose(i, &q("publication"), &modes)?;
    if publication.as_deref() == Some("unpublished") {
        let warning =
            format!("\n{}\n\n{}\n\n", say(lang, "warning.title", &[]), say(lang, "warning.text", &[]).trim_end());
        put(i, &warning)?;
        publication = choose(i, &q("exits"), &modes)?;
    }
    let Some(publication) = publication else { return stop(i, lang) };
    o.answers.insert("publication", publication);
    let Some(alias) = needed(i, &q("alias"))? else { return stop(i, lang) };
    o.answers.insert("alias", alias);
    let Some(key) = needed(i, &q("key"))? else { return stop(i, lang) };
    // A path from the home, as a shell would read it.
    // The home, then the rest, so that `~//x` names `$HOME//x`, as in a shell (P10C-7); a home that is not UTF-8,
    // which no answer can hold, is not read for another: the answer is then kept as given (P10C-12).
    let key = match (key.strip_prefix("~/"), i.home.and_then(std::path::Path::to_str)) {
        (Some(rest), Some(home)) => format!("{home}/{rest}"),
        _ => key,
    };
    o.answers.insert("key", key);
    // Git's commands to sign with that key, which the plan holds exactly when Git does not: shown, never run.
    let (doc, _) = assessed(i, o)?;
    let commands: Vec<&Value> =
        doc["plan"]["next"].as_array().into_iter().flatten().filter(|n| n.get("command").is_some()).collect();
    if !commands.is_empty() {
        let lines: String = commands.iter().map(|c| super::shell_line(lang, c, "")).collect();
        put(i, &format!("{}\n{lines}", q("sign")))?;
    }
    let names: Vec<&str> = i.adapters.iter().map(|a| a.program.executable).collect();
    let Some(tools) = ask(i, &format!("{} ({})", q("tools"), names.join(", ")))? else { return stop(i, lang) };
    // Choices in any case (P10C-3).
    for t in tools.to_lowercase().split(|c: char| c.is_whitespace() || c == ',').filter(|t| !t.is_empty()) {
        if !names.contains(&t) {
            return stop(i, lang);
        }
        o.tools.push(t.to_owned());
    }
    // REQ-INS-8's questions, each answer as given; an empty one leaves the catalogue's placeholder line.
    if super::kind(i.root, ".brewstillery/ALAMBIQUE.md") == "absent" {
        for k in ["purpose", "audience", "excluded"] {
            let Some(answer) = ask(i, &q(k))? else { return stop(i, lang) };
            if !answer.is_empty() {
                o.answers.insert(k, answer);
            }
        }
    }
    // The verdict and the whole plan, the first alambique included; a plan that cannot be applied says why.
    let (doc, text) = assessed(i, o)?;
    put(i, &format!("\n{text}\n"))?;
    let Some(digest) = doc["digest"].as_str().filter(|_| doc["apply"].as_array().is_some_and(|a| !a.is_empty())) else {
        return stop(i, lang);
    };
    // Only the catalogue's word for yes is a yes.
    let yes = q("yes");
    let answer = ask(i, &q("confirm"))?.map(|a| a.trim().to_lowercase());
    if !answer.is_some_and(|a| yes.split_whitespace().any(|y| y == a)) {
        return stop(i, lang);
    }
    let Some(words) = needed(i, &q("words"))? else { return stop(i, lang) };
    o.say = Some(words.into());
    Ok(Some(digest.to_owned()))
}

/// Says that nothing was applied, and gives no plan.
fn stop(i: &mut Interview, lang: &str) -> Result<Option<String>, Error> {
    put(i, &format!("\n{}\n", say(lang, "onboard.stopped", &[])))?;
    Ok(None)
}
