//! The generated views (REQ-VIEW-1 to REQ-VIEW-3; part 8c, D-074): `STATUS.md` and `RULES.md`, each a pure function of
//! the replay and the catalogue in the project's canonical documentation language, so that a fresh generation is the
//! committed text (PD-42, PD-43). They hold no clock, no branch head and no file read from anywhere else: what a person
//! waits for is shown with the time of the event that began the wait. `check` compares each with a fresh generation
//! (`VIEW_STALE`); every write command regenerates both.

use crate::cli::{catalogue, say, verbatim};
use crate::code::{Code, Error, io_at, refuse, unlinked};
use crate::record::{Event, Publication, Replay, StreamKind, stream_kind};
use crate::settings::{self, Val};
use crate::{alambique, domain, fermento, identity, table};
use std::path::{Path, PathBuf};

/// The views' files in the domain.
pub const STATUS: &str = "STATUS.md";
pub const RULES: &str = "RULES.md";

/// How many base rules the catalogue holds (`rules.base.1` to `rules.base.12`).
const BASE_RULES: usize = 12;

/// Who a decision waits for: every maestro, or the integrator (an agent).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Who {
    Maestro,
    Integrator,
}

/// One decision that waits: for whom, what (a catalogue key under `view.wait`), the stage, task, proposal or alias it
/// is about, when the event that began the wait was recorded, a word of detail, and the commands that exist for it.
#[derive(Debug, Clone)]
pub struct Wait {
    pub who: Who,
    pub what: &'static str,
    pub id: String,
    pub since: String,
    pub detail: String,
    pub commands: Vec<String>,
}

/// The publication mode as the record and the catalogue name it.
pub(crate) fn mode_name(m: Publication) -> &'static str {
    match m {
        Publication::Published => "published",
        Publication::OwnRepository => "own-repository",
        Publication::Unpublished => "unpublished",
    }
}

/// The language the views are in: the policy in force's canonical documentation language, `en` with none.
pub fn language(record: &Replay) -> &'static str {
    let policy = settings::in_force(record).ok().flatten();
    let tag = policy.and_then(|p| match settings::effective(&p, None, None, &[], &[]).ok()?.values.remove(KEY) {
        Some((Val::Str(s), _)) => Some(s),
        _ => None,
    });
    catalogue(tag.as_deref().unwrap_or("en"))
}

const KEY: &str = "languages.canonicalDocumentationLanguage";

fn at(e: &Event) -> String {
    e.value["at"].as_str().unwrap_or_default().to_owned()
}

fn text<'a>(e: &'a Event, field: &str) -> &'a str {
    e.value["payload"][field].as_str().unwrap_or_default()
}

fn terminal(state: &str) -> bool {
    matches!(state, "integrated" | "rejected")
}

/// The task streams of stage `id`, in the order of their identifiers.
pub(crate) fn tasks_of<'a>(record: &'a Replay, id: &str) -> Vec<(&'a String, &'a Vec<Event>)> {
    let of = |(n, e): &(&String, &Vec<Event>)| stream_kind(n) == Some(StreamKind::Task) && text(&e[0], "stage") == id;
    record.streams.iter().filter(|s| of(s)).collect()
}

/// The decisions that wait, in the order of the streams: the stage to authorize or close, the tasks to authorize, run,
/// retry or accept, the proposals to approve, and the sweeps a stage needs before it closes (REQ-VIEW-1, PD-33).
pub fn waits(record: &Replay) -> Vec<Wait> {
    let states = table::current(record).unwrap_or_default();
    let state = |n: &str| states.get(n).copied().unwrap_or_default();
    let aliases: Vec<String> = settings::in_force(record)
        .ok()
        .flatten()
        .map(|p| identity::maintainers(&p.settings).into_iter().map(|m| m.alias).collect())
        .unwrap_or_default();
    let mut out = Vec::new();
    let mut wait = |who, what, id: &str, since: String, detail: String, commands: &[String]| {
        out.push(Wait { who, what, id: id.to_owned(), since, detail, commands: commands.to_vec() });
    };
    let cmd = |c: &str, id: &str| format!("fabrewk {c} {id}");
    for (name, events) in &record.streams {
        let id = name.split_once('-').map_or("", |x| x.1);
        let last = events.last().map(at).unwrap_or_default();
        match stream_kind(name) {
            Some(StreamKind::Stage) => match state(name) {
                "proposed" => wait(Who::Maestro, "stage-authorize", id, last, String::new(), &[cmd("status", id)]),
                "authorized" => {
                    let tasks = tasks_of(record, id);
                    if tasks.is_empty() || tasks.iter().any(|(n, _)| !terminal(state(n))) {
                        continue;
                    }
                    let ended = tasks.iter().filter_map(|(_, e)| e.last().map(at)).max().unwrap_or_default();
                    let names: Vec<&String> = tasks.iter().map(|(n, _)| *n).collect();
                    let missing: Vec<&String> =
                        aliases.iter().filter(|a| fermento::unswept(record, a, &names).is_some()).collect();
                    for alias in &missing {
                        let help = "fabrewk sweep --help".to_owned();
                        wait(Who::Integrator, "sweep", alias, ended.clone(), id.to_owned(), &[help]);
                    }
                    if missing.is_empty() {
                        wait(Who::Maestro, "stage-close", id, ended, String::new(), &[cmd("close", id)]);
                    }
                }
                _ => {}
            },
            Some(StreamKind::Task) => {
                let stage = format!("stage-{}", text(&events[0], "stage"));
                let opened =
                    record.streams.get(&stage).and_then(|s| s.iter().rev().find(|e| e.kind == "stage-authorized"));
                let status = cmd("status", id);
                match state(name) {
                    "proposed" if state(&stage) == "authorized" => {
                        wait(Who::Maestro, "task-authorize", id, last, String::new(), &[status])
                    }
                    "authorized" => {
                        let since = opened.map(at).into_iter().chain([last]).max().unwrap_or_default();
                        wait(
                            Who::Maestro,
                            "task-run",
                            id,
                            since,
                            String::new(),
                            &[status, cmd("run", id), cmd("reject", id)],
                        )
                    }
                    s @ ("halted" | "verified") => {
                        let halt = events.iter().rev().find(|e| e.kind == "task-halted");
                        let code = halt.map(|e| text(e, "code")).filter(|_| s == "halted").unwrap_or_default();
                        let what = if s == "halted" { "task-halted" } else { "task-verified" };
                        wait(Who::Maestro, what, id, last, code.to_owned(), &[status, cmd("reject", id)])
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    let proposals = fermento::proposals(record).unwrap_or_default();
    for p in proposals.values().filter(|p| p.state == "pending") {
        let recorded =
            record.streams.get(&format!("fermento-{}", p.alias)).and_then(|s| s.iter().find(|e| text(e, "id") == p.id));
        let since = recorded.map(at).unwrap_or_default();
        wait(Who::Maestro, "proposal", &p.id, since, p.alias.clone(), &[cmd("approve", &p.id), cmd("discard", &p.id)]);
    }
    out
}

/// A task's usage (PD-39): its dispatches, and its tokens as the tool reported them; a sum with a
/// part not reported, or an attempt not ended, is unknown, never zero.
fn usage(events: &[Event]) -> Option<(usize, Option<u64>)> {
    let started = events.iter().filter(|e| e.kind == "attempt-started").count();
    let ended: Vec<&Event> = events.iter().filter(|e| e.kind == "attempt-ended").collect();
    let tokens = ended.iter().try_fold(0u64, |sum, e| sum.checked_add(e.value["payload"]["usage"]["tokens"].as_u64()?));
    (started > 0).then_some((started, tokens.filter(|_| ended.len() == started)))
}

/// `STATUS.md` in `lang`: the record's health, the stages with their tasks, what waits for each maestro and for the
/// integrator, the last integrated work, and the usage (REQ-VIEW-1).
pub fn status(record: &Replay, lang: &str) -> String {
    let t = |key: &str, params: &[(&str, &str)]| say(lang, key, params);
    let states = table::current(record).unwrap_or_default();
    let state = |n: &str| t(&format!("state.{}", states.get(n).copied().unwrap_or_default()), &[]);
    let mut out = format!(
        "# {}\n\n{}\n\n## {}\n\n",
        t("view.status-title", &[]),
        t("view.status-notice", &[]),
        t("view.record", &[])
    );
    let events = record.streams.values().map(Vec::len).sum::<usize>().to_string();
    out +=
        &format!("- {}\n", t("status.replays", &[("streams", &record.streams.len().to_string()), ("events", &events)]));
    let policy = settings::in_force(record).ok().flatten();
    match &policy {
        Some(p) => out += &format!("- {}\n", t("status.policy", &[("digest", &p.digest[..12])])),
        None => out += &format!("- {}\n", t("status.no-policy", &[])),
    }
    let mode = domain::publication(record).map(mode_name);
    match mode {
        Some(m) => out += &format!("- {}\n", t("status.mode", &[("mode", &t(&format!("mode.{m}"), &[]))])),
        None => out += &format!("- {}\n", t("status.no-mode", &[])),
    }
    if mode == Some("unpublished") {
        out += &format!("\n> {}\n", t("view.unpublished", &[]));
    }
    out += &format!("\n## {}\n\n", t("view.stages", &[]));
    let stages: Vec<_> = record.streams.iter().filter(|(n, _)| stream_kind(n) == Some(StreamKind::Stage)).collect();
    if stages.is_empty() {
        out += &format!("{}\n", t("status.no-stages", &[]));
    }
    for (name, events) in stages {
        let (id, goal) = (&name[6..], text(&events[0], "goal"));
        out += &format!(
            "### {}\n\n",
            t("status.stage", &[("id", id), ("state", &state(name)), ("goal", &verbatim(goal, "    "))])
        );
        for (tname, tevents) in tasks_of(record, id) {
            let goal = verbatim(text(&tevents[0], "goal"), "    ");
            let line = t("status.task", &[("id", &tname[5..]), ("state", &state(tname)), ("goal", &goal)]);
            let halt = tevents.iter().rev().find(|e| e.kind == "task-halted").map(|e| text(e, "code"));
            let halt = halt.filter(|_| states.get(tname) == Some(&"halted")).map(|c| t("view.halted", &[("code", c)]));
            out += &format!("- {line}{}\n", halt.map(|h| format!(" ({h})")).unwrap_or_default());
        }
        out.push('\n');
    }
    out += &format!("## {}\n\n", t("view.waiting", &[]));
    let all = waits(record);
    let line = |w: &Wait| {
        let head = t(&format!("view.wait.{}", w.what), &[("id", &w.id), ("detail", &w.detail), ("since", &w.since)]);
        let commands: String = w.commands.iter().map(|c| format!("\n  - `{c}`")).collect();
        format!("- {head}{commands}\n")
    };
    let people = policy.map(|p| identity::maintainers(&p.settings)).unwrap_or_default();
    for m in people.iter().filter(|m| m.role == "maestro") {
        let role = t(&format!("role.{}", m.role), &[]);
        out += &format!("### {}\n\n", t("view.waits-for", &[("alias", &m.alias), ("role", &role)]));
        let mine: String = all.iter().filter(|w| w.who == Who::Maestro).map(line).collect();
        out += &if mine.is_empty() { format!("{}\n", t("view.waits-none", &[("alias", &m.alias)])) } else { mine };
        out.push('\n');
    }
    out += &format!("### {}\n\n", t("view.integrator", &[]));
    let theirs: String = all.iter().filter(|w| w.who == Who::Integrator).map(line).collect();
    out += &if theirs.is_empty() { format!("{}\n", t("view.integrator-none", &[])) } else { theirs };
    out += &format!("\n## {}\n\n", t("view.integrated", &[]));
    let integrated =
        record.streams.values().flatten().filter(|e| e.kind == "task-integrated").max_by_key(|e| (at(e), &e.stream));
    match integrated {
        Some(e) => {
            let goal = record.streams.get(&e.stream).map(|s| text(&s[0], "goal")).unwrap_or_default();
            let params = [("id", &e.stream[5..]), ("at", &at(e)), ("goal", &verbatim(goal, "    "))];
            out += &format!("{}\n", t("view.integrated-line", &params));
        }
        None => out += &format!("{}\n", t("view.integrated-none", &[])),
    }
    out += &format!("\n## {}\n\n", t("view.usage", &[]));
    let used: Vec<_> = record.streams.iter().filter_map(|(n, e)| Some((n, usage(e)?))).collect();
    if used.is_empty() {
        out += &format!("{}\n", t("view.usage-none", &[]));
    }
    for (name, (dispatches, tokens)) in used {
        let tokens = tokens.map_or_else(|| t("view.unknown", &[]), |n| n.to_string());
        out += &format!(
            "- {}\n",
            t("view.usage-line", &[("id", &name[5..]), ("dispatches", &dispatches.to_string()), ("tokens", &tokens)])
        );
    }
    out
}

/// `RULES.md` in `lang`: the binary's base rules, then the project's own that a maestro approved (REQ-VIEW-2).
pub fn rules(record: &Replay, lang: &str) -> String {
    let t = |key: &str| say(lang, key, &[]);
    let mut out =
        format!("# {}\n\n{}\n\n## {}\n\n", t("view.rules-title"), t("view.rules-notice"), t("view.rules-base"));
    for n in 1..=BASE_RULES {
        out += &format!("{n}. {}\n", t(&format!("rules.base.{n}")));
    }
    out += &format!("\n## {}\n\n", t("view.rules-project"));
    let mut own: Vec<_> = alambique::rules(record).into_iter().collect();
    own.sort_by_key(|(k, _)| (k[1..].parse::<u64>().unwrap_or(u64::MAX), k.clone()));
    if own.is_empty() {
        out += &format!("{}\n", t("view.rules-none"));
    }
    for (key, rule) in own {
        out += &format!("- **{key}.** {}\n", verbatim(&rule, "  "));
    }
    out
}

/// Both views as a fresh generation gives them, by file name.
fn fresh(record: &Replay) -> [(&'static str, String); 2] {
    let lang = language(record);
    [(STATUS, status(record, lang)), (RULES, rules(record, lang))]
}

/// Writes both views from `record` (only a file that differs), and gives their paths relative to the domain.
pub fn write(domain: &Path, record: &Replay) -> Result<Vec<PathBuf>, Error> {
    let mut out = Vec::new();
    for (name, text) in fresh(record) {
        let path = domain.join(name);
        // A view that is a link, or not a regular file, is replaced, as `stale` reports it.
        let regular = std::fs::symlink_metadata(&path).is_ok_and(|m| m.is_file()) && unlinked(domain, name);
        if !regular || std::fs::read(&path).ok().as_deref() != Some(text.as_bytes()) {
            crate::record::write_atomic(&path, text.as_bytes())?;
        }
        out.push(PathBuf::from(name));
    }
    Ok(out)
}

/// Each view that is not what a fresh generation gives (`VIEW_STALE`): absent, not a regular file, reached through a
/// link, or any byte different (REQ-VIEW-3, PD-43). A file that cannot be read is an operating-system error.
pub fn stale(domain: &Path, record: &Replay) -> Vec<Error> {
    let mut out = Vec::new();
    for (name, text) in fresh(record) {
        let path = domain.join(name);
        let why = match std::fs::symlink_metadata(&path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => "it is absent",
            Err(e) => {
                out.push(io_at(&path)(e));
                continue;
            }
            Ok(m) if !m.is_file() || !unlinked(domain, name) => {
                "it is not a regular file, or is reached through a link"
            }
            Ok(_) => match std::fs::read(&path) {
                Ok(bytes) if bytes == text.as_bytes() => continue,
                Ok(_) => "it differs from a fresh generation",
                Err(e) => {
                    out.push(io_at(&path)(e));
                    continue;
                }
            },
        };
        out.push(refuse(Code::ViewStale, Some(&path), why));
    }
    out
}
