//! Part 8c through the built binary (core-specification 12: REQ-VIEW-1 to REQ-VIEW-3, REQ-TASK-6; PD-33, PD-40, PD-42,
//! PD-43; C5): the generated `STATUS.md` and `RULES.md`, what `status` says waits and for whom, `close` and every
//! refusal it gives, each asserting that nothing was written (11.2 rule 4).

mod common;

use common::*;
use fabrewk_core::alambique::{Rules, apply_rules, rules_digest};
use fabrewk_core::cli::say;
use serde_json::{Value, json};
use std::fs;

const SAY: &str = "Lo decido: es lo que leí.";
const H: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const DOC: &str = "# Alambique\n\nOpening.\n\n## 1. One\n\n**1.1. A:** first.\n\n**1.2. B:** second.\n";

fn ok(ran: &Ran) {
    assert_eq!(ran.status, 0, "{ran:?}");
    if !ran.stdout.starts_with('{') {
        whole(&ran.all());
    }
}

fn published(name: &str) -> Fx {
    Fx::new(name, "published")
}

fn read(fx: &Fx, view: &str) -> String {
    fs::read_to_string(fx.domain.join(view)).unwrap()
}

/// The stage and its one task, proposed through the binary; gives their identifiers.
fn propose(fx: &Fx, goal: &str) -> (String, String) {
    let stage =
        json!({"lang": "en", "goal": goal, "budget": {"dispatches": 6, "minutes": 120}, "checks": {}, "settings": {}});
    let task = json!({
        "lang": "en", "goal": "Make it.",
        "requirements": [{"text": "It builds.", "criterion": "cargo build passes"}],
        "base": "0".repeat(40), "dependencies": [], "allowed": ["src/**"],
        "profiles": {"build": "build", "review": "review"}, "checks": [],
        "limits": {"rounds": 3, "dispatches": 6, "minutes": 120}, "delegationLanguage": "en", "settings": {},
    });
    fs::write(fx.root.join("plan.json"), serde_json::to_vec(&json!({"stage": stage, "tasks": [task]})).unwrap())
        .unwrap();
    let ran = fx.run(&["propose", "plan.json", "--json"]);
    fs::remove_file(fx.root.join("plan.json")).unwrap();
    ok(&ran);
    let doc = ran.doc();
    (doc["stage"]["id"].as_str().unwrap().to_owned(), doc["tasks"][0]["id"].as_str().unwrap().to_owned())
}

/// `authorize <id>` with the digest `status` shows.
fn authorize(fx: &Fx, id: &str) {
    let digest = fx.run(&["status", id, "--json"]).doc()["shown"]["digest"].as_str().unwrap().to_owned();
    ok(&fx.run(&["authorize", id, "--seen", &digest, "--say", SAY, "--json"]));
}

/// The `at` of the last event of `stream`.
fn at(fx: &Fx, stream: &str) -> String {
    fx.events(stream).pop().unwrap()["at"].as_str().unwrap().to_owned()
}

/// Sweeps `xel`'s fermento, recommending nothing.
fn sweep(fx: &Fx) {
    let (text, parts) = fabrewk_core::fermento::open(&fx.domain, "xel").unwrap();
    let rows: Vec<Value> = fabrewk_core::fermento::points(&text[..parts.input_end])
        .iter()
        .map(|p| {
            let rec = json!({"point": "About it", "clarify": "Say it.", "destination": "alambique 6.1"});
            json!({"number": p.number, "digest": p.digest, "recommendation": rec})
        })
        .collect();
    let doc = json!({"agent": {"tool": "claude-code", "provider": "anthropic", "model": "claude-sonnet-5-5"}, "points": rows});
    fs::write(fx.root.join("answer.json"), serde_json::to_vec(&doc).unwrap()).unwrap();
    ok(&fx.run(&["sweep", "xel", "--answer", "answer.json", "--json"]));
}

/// A stage with its task rejected, so that it waits for a sweep and then for its close; not swept yet.
fn ended(fx: &Fx) -> (String, String) {
    let (stage, task) = propose(fx, "Build it.");
    authorize(fx, &stage);
    ok(&fx.run(&["reject", &task, "--say", SAY, "--json"]));
    (stage, task)
}

/// The views as the last record commit holds them equal the files, and a fresh generation is that text (PD-43).
fn committed(fx: &Fx) {
    for view in ["STATUS.md", "RULES.md"] {
        let held = fx.git(&["show", &format!("HEAD:.brewstillery/{view}")]);
        assert_eq!(held.trim_end(), read(fx, view).trim_end(), "{view}");
    }
    assert_eq!(fx.git(&["status", "--porcelain", "--", ".brewstillery/STATUS.md", ".brewstillery/RULES.md"]), "");
}

// ---- REQ-VIEW-3, PD-43: every write command regenerates, check compares ------------------------------------------------

#[test]
fn req_view_3_every_write_command_regenerates_both_views_and_check_compares_them() {
    let fx = published("view-3");
    ok(&fx.run(&["check"]));
    let (stage, task) = propose(&fx, "Build it.");
    // `propose` committed the views with the events, in the one record commit.
    assert_eq!(fx.git(&["rev-list", "--count", "HEAD"]), "3");
    let files = fx.git(&["show", "--name-only", "--format=", "HEAD"]);
    assert!(files.lines().any(|f| f == ".brewstillery/STATUS.md"), "{files}");
    assert!(read(&fx, "STATUS.md").contains(&stage) && read(&fx, "STATUS.md").contains(&task));
    committed(&fx);
    ok(&fx.run(&["check"]));
    authorize(&fx, &stage);
    committed(&fx);
    assert!(read(&fx, "STATUS.md").contains(&say("en", "state.authorized", &[])));
    // Every command that writes: a refusal leaves the views as they were.
    ok(&fx.run(&["check"]));
    let doc = fx.run(&["check", "--json"]).doc();
    let c5 = doc["checks"].as_array().unwrap().iter().find(|c| c["check"] == "C5").unwrap();
    assert_eq!(c5["control"], "effective");
}

#[test]
fn pd_43_a_view_changed_absent_or_linked_is_stale_and_the_next_write_regenerates_it() {
    let fx = published("view-stale");
    let status = fx.domain.join("STATUS.md");
    let fresh = read(&fx, "STATUS.md");
    // A byte changed by hand.
    fs::write(&status, fresh.clone() + " ").unwrap();
    let e = refused(&fx, &["check"], "VIEW_STALE", 3, "es");
    assert_eq!((e["check"].as_str(), e["control"].as_str()), (Some("C5"), Some("effective")));
    assert_eq!(e["path"], ".brewstillery/STATUS.md");
    // The reading commands do not regenerate it.
    for args in [&["status"][..], &["config"]] {
        assert_eq!(fx.run(args).status, 0);
    }
    assert_eq!(read(&fx, "STATUS.md"), fresh.clone() + " ");
    // The next command that writes does, and the check passes.
    propose(&fx, "Build it.");
    assert!(!read(&fx, "STATUS.md").ends_with("  "));
    ok(&fx.run(&["check"]));
    // An absent view, a changed one and one reached through a link are each stale.
    fs::remove_file(fx.domain.join("RULES.md")).unwrap();
    refused(&fx, &["check"], "VIEW_STALE", 3, "es");
    fx.git(&["checkout", "--", ".brewstillery/RULES.md"]);
    ok(&fx.run(&["check"]));
    fs::write(fx.domain.join("RULES.md"), "# Rules\n").unwrap();
    let e = refused(&fx, &["check"], "VIEW_STALE", 3, "es");
    assert_eq!(e["path"], ".brewstillery/RULES.md");
    fx.git(&["checkout", "--", ".brewstillery/RULES.md"]);
    let saved = fx.domain.join("RULES.real");
    fs::rename(fx.domain.join("RULES.md"), &saved).unwrap();
    std::os::unix::fs::symlink(&saved, fx.domain.join("RULES.md")).unwrap();
    refused(&fx, &["check"], "VIEW_STALE", 3, "es");
    // The link holds the very text a fresh generation gives, and the next write still replaces it with a file.
    propose(&fx, "Other.");
    assert!(fs::symlink_metadata(fx.domain.join("RULES.md")).unwrap().is_file());
    ok(&fx.run(&["check"]));
}

// ---- REQ-VIEW-1: what STATUS.md holds -----------------------------------------------------------------------------------

#[test]
fn req_view_1_status_shows_what_waits_for_whom_and_since_when_from_the_replay_alone() {
    let fx = published("view-1");
    let alias = [("alias", "xel")];
    let (waits_for, none) =
        (say("en", "view.waits-for", &[("alias", "xel"), ("role", "maestro")]), say("en", "view.waits-none", &alias));
    // PD-40: with nothing pending for the maestro, the view says there is none.
    assert!(read(&fx, "STATUS.md").contains(&format!("### {waits_for}\n\n{none}\n")), "{}", read(&fx, "STATUS.md"));
    let wait = |what: &str, id: &str, detail: &str, since: &str| {
        say("en", &format!("view.wait.{what}"), &[("id", id), ("detail", detail), ("since", since)])
    };
    let (stage, task) = propose(&fx, "Build it.");
    let status = read(&fx, "STATUS.md");
    let line = wait("stage-authorize", &stage, "", &at(&fx, &format!("stage-{stage}")));
    let (maestro, agents) =
        (status.find(&waits_for).unwrap(), status.find(&say("en", "view.integrator", &[])).unwrap());
    let at_line = status.find(&line).unwrap_or_else(|| panic!("{line}\n{status}"));
    assert!(maestro < at_line && at_line < agents);
    assert!(status.contains(&format!("`fabrewk status {stage}`")));
    // The task is not asked for while its stage waits: the stage's authorization covers it.
    assert!(!status.contains("was proposed into an authorized stage"), "{status}");
    // A gap of seconds, so that the authorization is told from the proposal by its time.
    std::thread::sleep(std::time::Duration::from_millis(1100));
    authorize(&fx, &stage);
    let since = std::cmp::max(at(&fx, &format!("stage-{stage}")), at(&fx, &format!("task-{task}")));
    let status = read(&fx, "STATUS.md");
    assert!(status.contains(&wait("task-run", &task, "", &since)), "{status}");
    assert!(status.contains(&format!("`fabrewk run {task}`")) && status.contains(&format!("`fabrewk reject {task}`")));
    // The task is rejected: the stage waits for a sweep, which waits for the integrator, not the maestro.
    ok(&fx.run(&["reject", &task, "--say", SAY, "--json"]));
    let ended = at(&fx, &format!("task-{task}"));
    let status = read(&fx, "STATUS.md");
    assert!(status.contains(&format!("### {waits_for}\n\n{none}\n")), "{status}");
    assert!(status.contains(&wait("sweep", "xel", &stage, &ended)), "{status}");
    assert!(status.contains("`fabrewk sweep --help`"));
    // Swept: the maestro closes it, and waits since the sweep that completed what the stage needed (P8C-2).
    std::thread::sleep(std::time::Duration::from_millis(1100));
    sweep(&fx);
    let swept = at(&fx, "fermento-xel");
    assert_ne!(swept, ended);
    let status = read(&fx, "STATUS.md");
    assert!(status.contains(&wait("stage-close", &stage, "", &swept)), "{status}");
    assert!(status.contains(&format!("`fabrewk close {stage}`")));
    assert!(status.contains(&say("en", "view.integrator-none", &[])));
    // Another sweep, with nothing changed, does not restart a wait that was already open (P8C-5).
    std::thread::sleep(std::time::Duration::from_millis(1100));
    sweep(&fx);
    assert_ne!(at(&fx, "fermento-xel"), swept);
    assert!(read(&fx, "STATUS.md").contains(&wait("stage-close", &stage, "", &swept)), "{}", read(&fx, "STATUS.md"));
    ok(&fx.run(&["close", &stage, "--say", SAY, "--json"]));
    let status = read(&fx, "STATUS.md");
    assert!(status.contains(&format!("### {waits_for}\n\n{none}\n")), "{status}");
    assert!(status.contains(&say(
        "en",
        "status.stage",
        &[("id", &stage), ("state", &say("en", "state.closed", &[])), ("goal", "Build it.")]
    )));
    assert!(
        status.contains(&say("en", "view.integrated-none", &[])) && status.contains(&say("en", "view.usage-none", &[]))
    );
    committed(&fx);
    ok(&fx.run(&["check"]));
}

#[test]
fn pd_42_pd_43_every_figure_follows_the_replay_and_no_fact_a_later_event_could_falsify_is_held() {
    let fx = published("view-42");
    let counts = |fx: &Fx| {
        let streams: Vec<String> = fs::read_dir(fx.domain.join("data/events"))
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .collect();
        (streams.len(), streams.iter().map(|s| fx.events(s).len()).sum::<usize>())
    };
    let figure = |fx: &Fx| {
        let (streams, events) = counts(fx);
        say("en", "status.replays", &[("streams", &streams.to_string()), ("events", &events.to_string())])
    };
    assert!(read(&fx, "STATUS.md").contains(&figure(&fx)));
    let (stage, _) = propose(&fx, "Build it.");
    assert!(read(&fx, "STATUS.md").contains(&figure(&fx)), "events added, the figure changed");
    authorize(&fx, &stage);
    assert!(read(&fx, "STATUS.md").contains(&figure(&fx)));
    // No branch head, no commit, no clock: the text holds nothing a later event could make false.
    let status = read(&fx, "STATUS.md");
    assert!(
        !status.contains(&fx.git(&["rev-parse", "HEAD"]))
            && !status.contains(&fx.git(&["rev-parse", "--short", "HEAD"]))
    );
    // The file is what a fresh generation from the replay gives, and what the record commit holds.
    let record = fabrewk_core::record::replay(&fx.domain, &fabrewk_core::identity::PolicyVerifier).unwrap();
    assert_eq!(status, fabrewk_core::view::status(&record, "en"));
    assert_eq!(read(&fx, "RULES.md"), fabrewk_core::view::rules(&record, "en"));
    committed(&fx);
}

#[test]
fn req_view_1_the_warning_for_unpublished_is_shown_in_that_mode_only() {
    let warning = say("en", "view.unpublished", &[]);
    let fx = Fx::new("view-unpublished", "unpublished");
    assert!(read(&fx, "STATUS.md").contains(&warning));
    propose(&fx, "Build it.");
    assert!(read(&fx, "STATUS.md").contains(&warning));
    ok(&fx.run(&["check"]));
    for mode in ["published", "own-repository"] {
        let fx = Fx::new(&format!("view-{mode}"), mode);
        assert!(!read(&fx, "STATUS.md").contains(&warning), "{mode}");
    }
}

// ---- PD-33: who acts next -----------------------------------------------------------------------------------------------

#[test]
fn pd_33_status_names_who_acts_next_and_shows_commands_only_to_who_decides() {
    let fx = published("pd-33");
    let next = |fx: &Fx| -> Vec<String> {
        let doc = fx.run(&["status", "--json"]).doc();
        doc["next"].as_array().unwrap().iter().map(|c| c.as_str().unwrap().to_owned()).collect()
    };
    let waiting = |fx: &Fx| fx.run(&["status", "--json"]).doc()["waiting"].clone();
    assert_eq!((waiting(&fx), next(&fx)), (json!([]), vec!["fabrewk check".to_owned()]));
    let (stage, task) = propose(&fx, "Build it.");
    // The maestro is asked to decide, and is given the command that shows what they would decide.
    assert_eq!(waiting(&fx)[0]["who"], "maestro");
    assert_eq!(waiting(&fx)[0]["what"], "stage-authorize");
    assert_eq!(next(&fx), ["fabrewk check".to_owned(), format!("fabrewk status {stage}")]);
    let human = fx.run(&["status"]);
    ok(&human);
    assert!(
        human.stdout.contains(&say(
            "es",
            "status.waits-maestro",
            &[(
                "line",
                &say(
                    "es",
                    "view.wait.stage-authorize",
                    &[("id", &stage), ("detail", ""), ("since", &at(&fx, &format!("stage-{stage}")))]
                )
            )]
        )),
        "{human:?}"
    );
    // Someone who is no maintainer, an agent, sees no command that is a maestro's to run.
    let stranger = fx.keys.join("stranger.pub");
    fx.git(&["config", "user.signingkey", stranger.to_str().unwrap()]);
    assert_eq!(next(&fx), ["fabrewk check".to_owned()]);
    fx.git(&["config", "user.signingkey", fx.keys.join("xel.pub").to_str().unwrap()]);
    authorize(&fx, &stage);
    assert_eq!(
        next(&fx),
        [
            "fabrewk check".to_owned(),
            format!("fabrewk status {task}"),
            format!("fabrewk run {task}"),
            format!("fabrewk reject {task}")
        ]
    );
    ok(&fx.run(&["reject", &task, "--say", SAY, "--json"]));
    // Now the integrator is the one who acts: the maestro sees no command, the agent sees the sweep's.
    assert_eq!(waiting(&fx)[0]["who"], "integrator");
    assert_eq!(next(&fx), ["fabrewk check".to_owned()]);
    fx.git(&["config", "user.signingkey", stranger.to_str().unwrap()]);
    assert_eq!(next(&fx), ["fabrewk check".to_owned(), "fabrewk sweep --help".to_owned()]);
}

// ---- REQ-VIEW-2: RULES.md -----------------------------------------------------------------------------------------------

#[test]
fn req_view_2_rules_hold_the_base_rules_then_the_project_rules_a_maestro_approved() {
    let fx = published("view-2");
    fx.alambique(DOC, "published");
    let rules = read(&fx, "RULES.md");
    let mut at = 0;
    for n in 1..=12 {
        let rule = format!("{n}. {}\n", say("en", &format!("rules.base.{n}"), &[]));
        let found = rules[at..].find(&rule).unwrap_or_else(|| panic!("{n}: {rules}"));
        at += found + rule.len();
    }
    assert!(rules.contains(&say("en", "view.rules-none", &[])));
    // Two project rules, approved: R10 follows R2.
    let none = Rules::new();
    let unit = |n: u64, text: &str| json!({"kind": "rule", "n": n, "text": text});
    let units = vec![unit(10, "Be last."), unit(2, "Be brief.")];
    let result = rules_digest(&apply_rules(&none, &units).unwrap());
    let edit = json!({"target": "rules", "base": rules_digest(&none), "keys": [], "result": result, "units": units});
    let (text, parts) = fabrewk_core::fermento::open(&fx.domain, "xel").unwrap();
    let rows: Vec<Value> = fabrewk_core::fermento::points(&text[..parts.input_end])
        .iter()
        .map(|p| {
            let mut row = json!({"number": p.number, "digest": p.digest, "recommendation": {"point": "x", "clarify": "y", "destination": "alambique 6.1"}});
            if p.number == 1 {
                row["edit"] = edit.clone();
            }
            row
        })
        .collect();
    let doc = json!({"agent": {"tool": "claude-code", "provider": "anthropic", "model": "claude-sonnet-5-5"}, "points": rows});
    fs::write(fx.root.join("answer.json"), serde_json::to_vec(&doc).unwrap()).unwrap();
    let swept = fx.run(&["sweep", "xel", "--answer", "answer.json", "--json"]);
    ok(&swept);
    let id = swept.doc()["proposals"][0].as_str().unwrap().to_owned();
    ok(&fx.run(&["approve", &id, "--say", SAY, "--json"]));
    let rules = read(&fx, "RULES.md");
    let (r2, r10) = (rules.find("- **R2.** Be brief.\n").unwrap(), rules.find("- **R10.** Be last.\n").unwrap());
    assert!(r2 < r10 && !rules.contains(&say("en", "view.rules-none", &[])), "{rules}");
    committed(&fx);
    ok(&fx.run(&["check"]));
}

#[test]
fn req_view_2_the_views_are_in_the_canonical_documentation_language() {
    let fx = published("view-lang");
    let record = fabrewk_core::record::replay(&fx.domain, &fabrewk_core::identity::PolicyVerifier).unwrap();
    assert_eq!(fabrewk_core::view::language(&record), "en");
    let (en, es) = (fabrewk_core::view::rules(&record, "en"), fabrewk_core::view::rules(&record, "es"));
    assert!(en.contains(&say("en", "rules.base.1", &[])) && es.contains(&say("es", "rules.base.1", &[])));
    assert_ne!(en, es);
    // The same record gives the same text.
    assert_eq!(fabrewk_core::view::status(&record, "es"), fabrewk_core::view::status(&record, "es"));
    assert_ne!(fabrewk_core::view::status(&record, "es"), fabrewk_core::view::status(&record, "en"));
}

// ---- REQ-TASK-6: close ---------------------------------------------------------------------------------------------------

#[test]
fn req_task_6_close_records_the_stage_its_head_and_the_sweeps_it_relied_on() {
    let fx = published("close");
    let (stage, task) = ended(&fx);
    sweep(&fx);
    let before = fx.git(&["rev-list", "--count", "HEAD"]).parse::<usize>().unwrap();
    let head = fx.events(&format!("stage-{stage}")).pop().unwrap();
    let ran = fx.run(&["close", &stage, "--say", SAY, "--json"]);
    ok(&ran);
    let doc = ran.doc();
    assert_eq!((doc["schema"].as_str(), doc["id"].as_str()), (Some("fabrewk.closed/2"), Some(stage.as_str())));
    let closed = fx.events(&format!("stage-{stage}")).pop().unwrap();
    assert_eq!(closed["kind"], "stage-closed");
    assert_eq!(closed["say"], json!({"text": SAY, "lang": "es-MX"}));
    assert_eq!(closed["actor"], json!({"kind": "person", "alias": "xel", "role": "maestro"}));
    let sweep_event = fx.events("fermento-xel").pop().unwrap();
    assert_eq!(sweep_event["kind"], "sweep-recorded");
    assert_eq!(
        closed["payload"],
        json!({
            "stage": stage,
            "head": {"seq": head["seq"], "digest": fabrewk_core::record::sha256_hex(&fs::read(newest(&fx, &format!("stage-{stage}"), 1)).unwrap())},
            "sweeps": [{"alias": "xel", "head": {"seq": sweep_event["seq"], "digest": fabrewk_core::record::sha256_hex(&fs::read(newest(&fx, "fermento-xel", 0)).unwrap())}}],
        })
    );
    assert_eq!(fx.git(&["rev-list", "--count", "HEAD"]).parse::<usize>().unwrap(), before + 1, "one record commit");
    assert_eq!(fx.run(&["status", &stage, "--json"]).doc()["state"], "closed");
    committed(&fx);
    ok(&fx.run(&["check"]));
    // A closed stage takes no task (STAGE_CLOSED) and is closed no more (MOVE_NOT_ALLOWED, D-053 point 3).
    refused(&fx, &["close", &stage, "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
    let _ = task;
}

/// The path of the `n`th newest event file of `stream` (0 is the last).
fn newest(fx: &Fx, stream: &str, n: usize) -> std::path::PathBuf {
    let dir = fx.domain.join("data/events").join(stream);
    let mut files: Vec<_> = fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|f| f.extension().is_some_and(|e| e == "json"))
        .collect();
    files.sort();
    files.reverse();
    files.remove(n)
}

#[test]
fn refusal_stage_open_tasks() {
    let fx = published("close-open");
    let (stage, task) = propose(&fx, "Build it.");
    authorize(&fx, &stage);
    // The task is authorized and has not ended.
    let e = refused(&fx, &["close", &stage, "--say", SAY], "STAGE_OPEN_TASKS", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains(&task), "{e}");
    assert!(e["detail"].as_str().unwrap().contains("authorized"), "{e}");
    assert_eq!(fx.run(&["status", &stage, "--json"]).doc()["state"], "authorized");
}

#[test]
fn refusal_sweep_missing_for_no_sweep_and_for_a_sweep_before_the_last_task_ended() {
    let fx = published("close-sweep");
    let (stage, task) = propose(&fx, "Build it.");
    authorize(&fx, &stage);
    // A sweep that saw the task before it ended does not count.
    sweep(&fx);
    ok(&fx.run(&["reject", &task, "--say", SAY, "--json"]));
    let e = refused(&fx, &["close", &stage, "--say", SAY], "SWEEP_MISSING", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("xel") && e["detail"].as_str().unwrap().contains(&task), "{e}");
    // Swept after: it closes.
    fx.commit("answer");
    sweep(&fx);
    ok(&fx.run(&["close", &stage, "--say", SAY]));
    // No sweep at all.
    let fx = published("close-sweep-none");
    let (stage, _) = ended(&fx);
    refused(&fx, &["close", &stage, "--say", SAY], "SWEEP_MISSING", 1, "es");
}

#[test]
fn refusal_alambique_unapproved_at_close() {
    let fx = published("close-alambique");
    fx.alambique(DOC, "published");
    let (stage, _) = ended(&fx);
    sweep(&fx);
    fs::write(fx.domain.join("ALAMBIQUE.md"), DOC.replace("first", "changed")).unwrap();
    refused(&fx, &["close", &stage, "--say", SAY], "ALAMBIQUE_UNAPPROVED", 1, "es");
    // A maestro approves it, and the stage closes.
    ok(&fx.run(&["approve", "alambique", "--say", SAY, "--json"]));
    ok(&fx.run(&["close", &stage, "--say", SAY]));
}

#[test]
fn refusal_record_uncommitted_at_close() {
    let fx = published("close-uncommitted");
    let (stage, _) = ended(&fx);
    sweep(&fx);
    // Events a stopped command left: a stage proposed through the library, not committed.
    fx.stage_and_task();
    let e = refused(&fx, &["close", &stage, "--say", SAY], "RECORD_UNCOMMITTED", 1, "es");
    assert!(e["path"].as_str().unwrap().starts_with(".brewstillery/data/events/"), "{e}");
    // The next write command commits them first, and then the stage closes.
    fx.commit("events");
    ok(&fx.run(&["close", &stage, "--say", SAY]));
}

#[test]
fn refusal_close_of_a_proposed_stage_a_task_an_unknown_id_a_stranger_and_an_agent() {
    let fx = published("close-moves");
    let (stage, task) = propose(&fx, "Build it.");
    refused(&fx, &["close", &stage, "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
    // A task's identifier, and one the record does not hold, are usage errors.
    for id in [task.as_str(), "s-00000000"] {
        let before = fx.snapshot();
        let ran = fx.run(&["close", id, "--say", SAY]);
        assert_eq!((ran.status, fx.snapshot() == before), (2, true), "{ran:?}");
    }
    let ran = fx.run(&["close", &task, "--say", SAY]);
    assert!(ran.stderr.contains(&say("es", "usage.task", &[("word", &task)])), "{ran:?}");
    assert_eq!(fx.run(&["close", &task, "--say", SAY, "--json"]).doc()["run"], "fabrewk close --help");
    // Words are the person's own.
    authorize(&fx, &stage);
    ok(&fx.run(&["reject", &task, "--say", SAY, "--json"]));
    sweep(&fx);
    refused(&fx, &["close", &stage], "WORDS_MISSING", 1, "es");
    for marker in ["CLAUDECODE", "CODEX_SANDBOX", fabrewk_core::cli::EXTENSION_MARKER] {
        let before = fx.snapshot();
        let ran = fx.run_env(&["close", &stage, "--say", SAY], &[(marker, "1")]);
        assert_eq!((ran.status, fx.snapshot() == before), (1, true), "{marker}: {ran:?}");
        assert!(ran.stderr.contains("AGENT_ENVIRONMENT"), "{marker}: {ran:?}");
    }
    let stranger = fx.keys.join("stranger.pub");
    fx.git(&["config", "user.signingkey", stranger.to_str().unwrap()]);
    refused(&fx, &["close", &stage, "--say", SAY], "ROLE_FORBIDDEN", 1, "en");
    let _ = H;
}

#[test]
fn req_view_1_a_pending_proposal_waits_for_the_maestro_until_it_is_decided() {
    let fx = published("view-proposal");
    fx.alambique(DOC, "published");
    let text = fs::read_to_string(fx.domain.join("ALAMBIQUE.md")).unwrap();
    let units = vec![json!({"kind": "point", "n": 1, "m": 2, "text": "**1.2. B:** rewritten."})];
    let result = fabrewk_core::record::sha256_hex(fabrewk_core::alambique::apply(&text, &units).unwrap().as_bytes());
    let edit = json!({"target": "alambique", "base": fabrewk_core::record::sha256_hex(text.as_bytes()), "keys": [], "result": result, "units": units});
    let (fermento, parts) = fabrewk_core::fermento::open(&fx.domain, "xel").unwrap();
    let rows: Vec<Value> = fabrewk_core::fermento::points(&fermento[..parts.input_end])
        .iter()
        .map(|p| {
            let mut row = json!({"number": p.number, "digest": p.digest, "recommendation": {"point": "x", "clarify": "y", "destination": "alambique 6.1"}});
            if p.number == 1 {
                row["edit"] = edit.clone();
            }
            row
        })
        .collect();
    let doc = json!({"agent": {"tool": "claude-code", "provider": "anthropic", "model": "claude-sonnet-5-5"}, "points": rows});
    fs::write(fx.root.join("answer.json"), serde_json::to_vec(&doc).unwrap()).unwrap();
    let swept = fx.run(&["sweep", "xel", "--answer", "answer.json", "--json"]);
    ok(&swept);
    let id = swept.doc()["proposals"][0].as_str().unwrap().to_owned();
    // The wait starts at the event that recorded the proposal.
    let recorded = fx.events("fermento-xel").into_iter().find(|e| e["kind"] == "proposal-recorded").unwrap();
    let since = recorded["at"].as_str().unwrap();
    let line = say("en", "view.wait.proposal", &[("id", &id), ("detail", "xel"), ("since", since)]);
    let status = read(&fx, "STATUS.md");
    assert!(status.contains(&line), "{line}\n{status}");
    assert!(status.contains(&format!("`fabrewk approve {id}`")) && status.contains(&format!("`fabrewk discard {id}`")));
    let waits = fx.run(&["status", "--json"]).doc()["waiting"].clone();
    assert_eq!(
        (waits[0]["what"].as_str(), waits[0]["id"].as_str(), waits[0]["detail"].as_str()),
        (Some("proposal"), Some(id.as_str()), Some("xel"))
    );
    // A second proposal, from a later sweep, waits since its own sweep, not since the first one.
    std::thread::sleep(std::time::Duration::from_millis(1100));
    let units = vec![json!({"kind": "point", "n": 1, "m": 1, "text": "**1.1. A:** rewritten."})];
    let result = fabrewk_core::record::sha256_hex(fabrewk_core::alambique::apply(&text, &units).unwrap().as_bytes());
    let second = json!({"target": "alambique", "base": fabrewk_core::record::sha256_hex(text.as_bytes()), "keys": [], "result": result, "units": units});
    let mut rows = doc["points"].clone();
    rows[0]["edit"] = second;
    fs::write(
        fx.root.join("answer.json"),
        serde_json::to_vec(&json!({"agent": doc["agent"], "points": rows})).unwrap(),
    )
    .unwrap();
    fx.commit("first sweep");
    let swept = fx.run(&["sweep", "xel", "--answer", "answer.json", "--json"]);
    ok(&swept);
    let other = swept.doc()["proposals"][0].as_str().unwrap().to_owned();
    assert_ne!(other, id);
    let recorded: Vec<Value> =
        fx.events("fermento-xel").into_iter().filter(|e| e["kind"] == "proposal-recorded").collect();
    assert_ne!(recorded[0]["at"], recorded[1]["at"]);
    let status = read(&fx, "STATUS.md");
    for (pid, e) in [(&id, &recorded[0]), (&other, &recorded[1])] {
        let line =
            say("en", "view.wait.proposal", &[("id", pid), ("detail", "xel"), ("since", e["at"].as_str().unwrap())]);
        assert!(status.contains(&line), "{line}\n{status}");
    }
    // Discarded, it waits no more.
    ok(&fx.run(&["discard", &id, "--say", SAY, "--json"]));
    assert!(!read(&fx, "STATUS.md").contains(&format!("`fabrewk approve {id}`")));
    let waiting: Vec<Value> = fx.run(&["status", "--json"]).doc()["waiting"].as_array().unwrap().clone();
    assert_eq!(waiting.iter().map(|w| w["id"].as_str().unwrap()).collect::<Vec<_>>(), [other.as_str()]);
    ok(&fx.run(&["discard", &other, "--say", SAY, "--json"]));
    assert_eq!(fx.run(&["status", "--json"]).doc()["waiting"], json!([]));
}

#[test]
fn c5_is_unknown_for_a_record_that_does_not_replay_and_silent_for_an_absent_one() {
    let finding = |fx: &Fx| -> Vec<String> {
        let doc = fx.run(&["check", "--json"]).doc();
        let list = doc["findings"].as_array().unwrap().iter().filter(|f| f["check"] == "C5");
        list.map(|f| f["code"].as_str().unwrap().to_owned()).collect()
    };
    let fx = published("c5-broken");
    fs::write(fx.domain.join("data/events/project/000009-0000000000000000.json"), "{}").unwrap();
    assert_eq!(finding(&fx), ["CHECK_UNKNOWN"]);
    let bare = Fx::bare("c5-absent", "published");
    assert_eq!(finding(&bare), Vec::<String>::new());
}

#[test]
fn a_view_in_a_domain_reached_through_a_link_is_stale_and_replaced_by_a_write() {
    let fx = published("view-linked-domain");
    let record = fabrewk_core::record::replay(&fx.domain, &fabrewk_core::identity::PolicyVerifier).unwrap();
    assert!(fabrewk_core::view::stale(&fx.domain, &record).is_empty());
    let link = fx.root.join("linked");
    std::os::unix::fs::symlink(&fx.domain, &link).unwrap();
    let stale = fabrewk_core::view::stale(&link, &record);
    assert_eq!(stale.len(), 2, "{stale:?}");
    assert!(stale.iter().all(|e| e.code() == Some(fabrewk_core::Code::ViewStale)), "{stale:?}");
}

#[test]
fn a_stage_closed_payload_holds_its_shape() {
    use fabrewk_core::task::check;
    let good = json!({"stage": "s-0123abcd", "head": {"seq": 2, "digest": H}, "sweeps": [{"alias": "xel", "head": {"seq": 1, "digest": H}}]});
    assert!(check("stage-closed", &good).is_ok());
    for bad in [json!({}), json!({"stage": "s-0123abcd", "head": {"seq": 2, "digest": H}}), {
        let mut p = good.clone();
        p["extra"] = json!(1);
        p
    }] {
        let e = check("stage-closed", &bad).unwrap_err();
        assert_eq!(e.code(), Some(fabrewk_core::Code::RecordVersionUnknown), "{bad}");
    }
}

#[test]
fn req_view_1_a_stage_with_no_task_waits_for_its_close_since_its_authorization() {
    use fabrewk_core::record::{Draft, Session, nonce, subject_stream};
    let fx = published("view-empty-stage");
    let policy =
        fabrewk_core::record::sha256_hex(fs::read_to_string(fx.domain.join("settings.toml")).unwrap().as_bytes());
    let mut payload = json!({"lang": "en", "goal": "Nothing yet.", "budget": {"dispatches": 6, "minutes": 120}, "checks": {}, "settings": {}});
    payload["nonce"] = json!(nonce().unwrap());
    let stream = subject_stream("stage-proposed", &payload).unwrap();
    let agent = json!({"kind": "agent", "tool": "codex", "provider": "openai", "modelRequested": "gpt-6.1-sol", "modelObserved": null});
    let draft = Draft {
        kind: "stage-proposed".into(),
        actor: agent,
        policy,
        instance: "SHA256:fixture".into(),
        say: None,
        payload,
    };
    let mut s = Session::begin(&fx.domain, None, &fabrewk_core::identity::PolicyVerifier).unwrap();
    s.append(&stream, None, &draft, None).unwrap();
    s.finish(&fabrewk_core::identity::PolicyVerifier, &[]).unwrap();
    fx.commit("stage");
    let stage = stream["stage-".len()..].to_owned();
    authorize(&fx, &stage);
    let waits = fx.run(&["status", "--json"]).doc()["waiting"].clone();
    assert_eq!(
        (waits[0]["what"].as_str(), waits[0]["since"].as_str()),
        (Some("stage-close"), Some(at(&fx, &stream).as_str())),
        "{waits}"
    );
    assert!(read(&fx, "STATUS.md").contains(&format!("`fabrewk close {stage}`")));
    ok(&fx.run(&["close", &stage, "--say", SAY]));
}

#[test]
fn p8c_1_a_link_at_a_views_temporary_name_is_removed_not_followed() {
    let fx = published("view-tmp-link");
    let outside = fx.root.join("outside.txt");
    fs::write(&outside, "sentinel\n").unwrap();
    for name in [".STATUS.md.tmp", ".RULES.md.tmp"] {
        std::os::unix::fs::symlink(&outside, fx.domain.join(name)).unwrap();
    }
    // Both views need regenerating: the stage changes STATUS.md, and RULES.md is replaced by hand.
    fs::write(fx.domain.join("RULES.md"), "by hand\n").unwrap();
    propose(&fx, "Build it.");
    assert_eq!(fs::read_to_string(&outside).unwrap(), "sentinel\n");
    for name in ["STATUS.md", "RULES.md"] {
        assert!(fs::symlink_metadata(fx.domain.join(name)).unwrap().is_file(), "{name}");
    }
    ok(&fx.run(&["check"]));
}
