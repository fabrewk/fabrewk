//! Part 8a through the built binary (core-specification 12: REQ-FER-1 to REQ-FER-4, REQ-FER-8 to REQ-FER-10; PD-41's
//! sweep half; C6 and C12): `sweep` and `discard`, and every refusal they and `check` give, each asserting that
//! nothing was written (11.2 rule 4).

mod common;

use common::*;
use fabrewk_core::fermento::{self, Point};
use fabrewk_core::record::{Draft, Session, sha256_hex, subject_digest};
use serde_json::{Value, json};
use std::fs;

const SAY: &str = "Lo descarto: ya no hace falta.";
const H: &str = "0000000000000000000000000000000000000000000000000000000000000000";

fn published(name: &str) -> Fx {
    Fx::new(name, "published")
}

fn ok(ran: &Ran) {
    assert_eq!(ran.status, 0, "{ran:?}");
    if !ran.stdout.starts_with('{') {
        whole(&ran.all());
    }
}

fn fermento_path(fx: &Fx) -> std::path::PathBuf {
    fx.domain.join("xel/fermento.md")
}

fn text(fx: &Fx) -> String {
    fs::read_to_string(fermento_path(fx)).unwrap()
}

/// The points of the fermento as it stands.
fn points(fx: &Fx) -> Vec<Point> {
    let (text, parts) = fermento::open(&fx.domain, "xel").unwrap();
    fermento::points(&text[..parts.input_end])
}

fn edit(text: &str) -> Value {
    json!({"target": "alambique", "base": H, "keys": [], "result": H,
           "units": [{"kind": "point", "n": 6, "m": 1, "text": text}]})
}

/// The answer for every point of the fermento as it stands, with `edits` by point number; written outside the
/// domain as `answer.json`.
fn answer(fx: &Fx, edits: &[(u32, Value)]) -> &'static str {
    let rows: Vec<Value> = points(fx)
        .iter()
        .map(|p| {
            let rec = json!({"point": format!("About {}", p.number), "clarify": "Say it in one line.", "destination": "alambique 6.1"});
            let mut row = json!({"number": p.number, "digest": p.digest, "recommendation": rec});
            if let Some((_, e)) = edits.iter().find(|(n, _)| *n == p.number) {
                row["edit"] = e.clone();
            }
            row
        })
        .collect();
    let doc = json!({"agent": {"tool": "claude-code", "provider": "anthropic", "model": "claude-sonnet-5-5"}, "points": rows});
    fs::write(fx.root.join("answer.json"), serde_json::to_vec(&doc).unwrap()).unwrap();
    "answer.json"
}

fn sweep(fx: &Fx, file: &str) -> Ran {
    fx.run(&["sweep", "xel", "--answer", file, "--json"])
}

fn log(fx: &Fx) -> usize {
    fx.git(&["rev-list", "--count", "HEAD"]).parse().unwrap()
}

#[test]
fn req_fer_3_a_valid_answer_records_the_events_and_changes_only_the_text_below_the_separator() {
    let fx = published("sweep");
    // A person's staged and unstaged edits of another file stay as they were (PD-41, the sweep half).
    fs::write(fx.root.join("README.md"), "staged\n").unwrap();
    fx.git(&["add", "README.md"]);
    fs::write(fx.root.join("README.md"), "unstaged\n").unwrap();
    let before = text(&fx);
    let (input_before, commits) = {
        let (t, p) = fermento::open(&fx.domain, "xel").unwrap();
        (fermento::input_digest(&t, &p), log(&fx))
    };
    let file = answer(&fx, &[(2, edit("**6.1.** New text."))]);
    let ran = sweep(&fx, file);
    ok(&ran);
    let doc = ran.doc();
    assert_eq!(
        (doc["schema"].as_str(), doc["points"].as_u64(), doc["proposals"].as_array().unwrap().len()),
        (Some("fabrewk.swept/1"), Some(2), 1)
    );
    // The events: the sweep, then the proposal, by the agent the answer names; the heads the sweep saw.
    let events = fx.events("fermento-xel");
    let kinds: Vec<&str> = events.iter().map(|e| e["kind"].as_str().unwrap()).collect();
    assert_eq!(kinds, ["sweep-recorded", "proposal-recorded"]);
    assert_eq!(
        events[0]["actor"],
        json!({"kind": "agent", "tool": "claude-code", "provider": "anthropic", "modelRequested": "claude-sonnet-5-5", "modelObserved": null})
    );
    assert_eq!(events[0]["payload"]["input"], json!(input_before));
    assert_eq!(events[0]["payload"]["skills"], json!([]));
    assert_eq!(events[1]["payload"]["id"], doc["proposals"][0]);
    assert!(events.iter().all(|e| e.get("say").is_none()));
    // Only the text below the separator changed: the input's digest is the same, and the answer is in the
    // maintainer's language and equals one generated from the events.
    let after = text(&fx);
    let (t, p) = fermento::open(&fx.domain, "xel").unwrap();
    assert_eq!(fermento::input_digest(&t, &p), input_before);
    assert!(after.starts_with(&before[..before.len()]) && after.len() > before.len());
    let record = fabrewk_core::record::replay(&fx.domain, &fabrewk_core::identity::PolicyVerifier).unwrap();
    assert_eq!(&after[p.after..], format!("\n{}", fermento::answer(&record, "xel", "es")));
    assert!(after[p.after..].starts_with("\n## Respuesta del integrador\n\n**1. About 1**\n"), "{after}");
    // One record commit holds the events and the fermento, and nothing of the person's.
    assert_eq!(log(&fx), commits + 1);
    assert_eq!(fx.git(&["log", "-1", "--format=%s"]), "fabrewk: record");
    let files = fx.git(&["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        files.contains(".brewstillery/xel/fermento.md") && files.contains("data/events/fermento-xel/000002-"),
        "{files}"
    );
    assert!(!files.contains("README.md") && !files.contains("answer.json"), "{files}");
    assert_eq!(fx.git(&["diff", "--cached", "--name-only"]), "README.md");
    assert_eq!(fs::read_to_string(fx.root.join("README.md")).unwrap(), "unstaged\n");
    assert_eq!(fx.git(&["status", "--porcelain", "--", ".brewstillery"]), "");
    // The check passes, and a sweep of the same answer records the sweep and no second proposal.
    ok(&fx.run(&["check"]));
    ok(&sweep(&fx, file));
    let kinds: Vec<String> = fx.events("fermento-xel").iter().map(|e| e["kind"].as_str().unwrap().to_owned()).collect();
    assert_eq!(kinds, ["sweep-recorded", "proposal-recorded", "sweep-recorded"]);
    assert_eq!(text(&fx), after);
}

#[test]
fn req_fer_3_where_the_mode_does_not_commit_the_input_need_not_be_committed() {
    let fx = Fx::new("sweep-unpublished", "unpublished");
    fs::write(fermento_path(&fx), text(&fx).replacen("First point.", "First point, edited.", 1)).unwrap();
    let ran = fx.run(&["sweep", "xel", "--answer", answer(&fx, &[]), "--json"]);
    ok(&ran);
    assert!(ran.doc()["commit"].is_null());
    assert!(text(&fx).contains("## Respuesta del integrador") && text(&fx).contains("First point, edited."));
}

#[test]
fn req_fer_3_points_that_are_gone_withdraw_their_open_proposals() {
    let fx = published("withdrawn");
    let doc = sweep(&fx, answer(&fx, &[(1, edit("**6.1.** One.")), (2, edit("**6.2.** Two."))])).doc();
    let ids: Vec<String> =
        doc["proposals"].as_array().unwrap().iter().map(|i| i.as_str().unwrap().to_owned()).collect();
    assert_eq!(ids.len(), 2);
    // The second point goes; the first is reworded (its proposal is stale, not withdrawn).
    fs::write(
        fermento_path(&fx),
        text(&fx).replace("2. Second point,\n   on two lines.\n\n", "").replace("First point.", "First, reworded."),
    )
    .unwrap();
    fx.commit("input");
    let ran = sweep(&fx, answer(&fx, &[]));
    ok(&ran);
    let doc = ran.doc();
    assert_eq!(doc["withdrawn"].as_array().unwrap().len(), 1);
    let events = fx.events("fermento-xel");
    let last = events.last().unwrap();
    assert_eq!((last["kind"].as_str(), last["actor"]["kind"].as_str()), (Some("proposal-withdrawn"), Some("fabrewk")));
    assert_eq!(last["payload"], json!({"id": doc["withdrawn"][0], "why": "point-gone"}));
    let answer_zone = text(&fx);
    assert!(answer_zone.contains("obsoleta"), "{answer_zone}");
    assert!(!answer_zone.contains(doc["withdrawn"][0].as_str().unwrap()));
    // A pending proposal of the first point remains, and the third sweep adds none.
    assert_eq!(answer_zone.matches("fabrewk discard").count(), 1);
}

#[test]
fn req_fer_8_a_state_typed_into_the_answer_is_overwritten_at_the_next_sweep() {
    let fx = published("typed");
    ok(&sweep(&fx, answer(&fx, &[(2, edit("**6.1.** New."))])));
    let generated = text(&fx);
    let typed = generated.replace("pendiente", "incorporada");
    assert_ne!(typed, generated);
    fs::write(fermento_path(&fx), &typed).unwrap();
    fx.commit("typed by hand");
    ok(&sweep(&fx, answer(&fx, &[(2, edit("**6.1.** New."))])));
    assert_eq!(text(&fx), generated);
}

#[test]
fn req_fer_9_uncited_files_of_a_maintainers_space_are_never_read() {
    use std::os::unix::fs::PermissionsExt;
    let fx = published("uncited");
    let secret = fx.domain.join("xel/notes.txt");
    fs::write(&secret, "private notes\n").unwrap();
    fs::set_permissions(&secret, fs::Permissions::from_mode(0o000)).unwrap();
    ok(&fx.run(&["check"]));
    ok(&sweep(&fx, answer(&fx, &[])));
    ok(&fx.run(&["check"]));
    assert!(!text(&fx).contains("private notes"));
    fs::set_permissions(&secret, fs::Permissions::from_mode(0o644)).unwrap();
}

#[test]
fn refusal_answer_invalid() {
    let fx = published("invalid");
    let good = answer(&fx, &[]);
    let doc: Value = serde_json::from_str(&fs::read_to_string(fx.root.join(good)).unwrap()).unwrap();
    let write = |v: &Value| fs::write(fx.root.join("bad.json"), serde_json::to_vec(v).unwrap()).unwrap();
    let args = ["sweep", "xel", "--answer", "bad.json"];
    // Not there, not JSON, a directory, a file of a domain.
    refused(&fx, &["sweep", "xel", "--answer", "none.json"], "ANSWER_INVALID", 1, "es");
    fs::write(fx.root.join("bad.json"), "{ not json").unwrap();
    refused(&fx, &args, "ANSWER_INVALID", 1, "es");
    refused(&fx, &["sweep", "xel", "--answer", ".brewstillery/settings.toml"], "ANSWER_INVALID", 1, "es");
    // A shape that is not the file's, a point twice, an edit with no unit, the marker as a line of a text.
    let mut bad = doc.clone();
    bad["points"] = json!("none");
    write(&bad);
    refused(&fx, &args, "ANSWER_INVALID", 1, "es");
    let mut bad = doc.clone();
    bad["points"][1] = bad["points"][0].clone();
    write(&bad);
    refused(&fx, &args, "ANSWER_INVALID", 1, "es");
    let mut bad = doc.clone();
    bad["points"][0]["edit"] = json!({"target": "alambique", "base": H, "keys": [], "result": H, "units": []});
    write(&bad);
    refused(&fx, &args, "ANSWER_INVALID", 1, "es");
    let mut bad = doc.clone();
    bad["points"][0]["recommendation"]["clarify"] = json!(format!("a\n{}\nb", fermento::MARKER));
    write(&bad);
    refused(&fx, &args, "ANSWER_INVALID", 1, "es");
    // Without `--answer`, a usage error.
    assert_eq!(fx.run(&["sweep", "xel"]).status, 2);
}

#[test]
fn refusal_point_changed() {
    let fx = published("point");
    let file = answer(&fx, &[]);
    let args = ["sweep", "xel", "--answer", file];
    let doc: Value = serde_json::from_str(&fs::read_to_string(fx.root.join(file)).unwrap()).unwrap();
    // A point the answer lacks, one the fermento lacks, a digest that differs.
    let write = |v: &Value| fs::write(fx.root.join(file), serde_json::to_vec(v).unwrap()).unwrap();
    let mut short = doc.clone();
    short["points"].as_array_mut().unwrap().pop();
    write(&short);
    let e = refused(&fx, &args, "POINT_CHANGED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("point 2"), "{e}");
    let mut extra = doc.clone();
    extra["points"][1]["number"] = json!(7);
    write(&extra);
    let e = refused(&fx, &args, "POINT_CHANGED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("point 7"), "{e}");
    write(&doc);
    fs::write(fermento_path(&fx), text(&fx).replace("First point.", "First point, changed.")).unwrap();
    fx.commit("changed");
    let e = refused(&fx, &args, "POINT_CHANGED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("point 1 changed"), "{e}");
}

#[test]
fn refusal_separator_missing_ambiguous_and_damaged_write_nothing() {
    let fx = published("separator");
    let file = answer(&fx, &[]);
    let args = ["sweep", "xel", "--answer", file];
    let good = text(&fx);
    let block = fermento::block("en", "xel");
    let cases = [
        ("SEPARATOR_MISSING", good.replace(fermento::MARKER, "")),
        ("SEPARATOR_MISSING", format!("```\n{good}```\n")),
        ("SEPARATOR_AMBIGUOUS", format!("{good}\n{block}")),
        ("SEPARATOR_DAMAGED", good.replace("INPUT", "input")),
    ];
    for (code, broken) in cases {
        fs::write(fermento_path(&fx), &broken).unwrap();
        refused(&fx, &args, code, 1, "es");
        // `check` finds the same, as C6.
        let e = refused(&fx, &["check"], code, 3, "es");
        assert_eq!(e["check"], "C6");
    }
}

#[test]
fn refusal_input_uncommitted_where_the_mode_commits() {
    let fx = published("uncommitted");
    fs::write(fermento_path(&fx), text(&fx).replacen("First point.", "First point, edited.", 1)).unwrap();
    let args = ["sweep", "xel", "--answer", answer(&fx, &[])];
    refused(&fx, &args, "INPUT_UNCOMMITTED", 1, "es");
    // A staged edit is no better than an unstaged one; a committed one passes.
    fx.git(&["add", ".brewstillery/xel/fermento.md"]);
    refused(&fx, &args, "INPUT_UNCOMMITTED", 1, "es");
    fx.commit("edited");
    ok(&fx.run(&args));
}

#[test]
fn refusal_maintainer_unlisted_fermento_missing_and_alias_invalid() {
    let fx = published("maintainer");
    let file = answer(&fx, &[]);
    refused(&fx, &["sweep", "ana", "--answer", file], "MAINTAINER_UNLISTED", 1, "es");
    refused(&fx, &["sweep", "Xel", "--answer", file], "ALIAS_INVALID", 1, "es");
    // A directory with no fermento: `check` finds it unlisted (C12), and a listed one without a fermento missing.
    fs::create_dir(fx.domain.join("ana")).unwrap();
    let e = refused(&fx, &["check"], "MAINTAINER_UNLISTED", 3, "es");
    assert_eq!(e["check"], "C12");
    fs::write(fx.domain.join("ana/fermento.md"), fermento("# Ana\n\n")).unwrap();
    let ran = fx.run(&["check"]);
    assert_eq!(ran.status, 3, "an unlisted directory stays a finding: {ran:?}");
}

#[test]
fn refusal_fermento_missing() {
    let fx = published("missing");
    let file = answer(&fx, &[]);
    fs::remove_file(fermento_path(&fx)).unwrap();
    refused(&fx, &["sweep", "xel", "--answer", file], "FERMENTO_MISSING", 1, "es");
    let e = refused(&fx, &["check"], "FERMENTO_MISSING", 3, "es");
    assert_eq!(e["check"], "C12");
    // A link or a directory in its place is no fermento, and is not followed.
    fs::create_dir(fermento_path(&fx)).unwrap();
    refused(&fx, &["sweep", "xel", "--answer", file], "FERMENTO_MISSING", 1, "es");
    fs::remove_dir(fermento_path(&fx)).unwrap();
    std::os::unix::fs::symlink("../ALAMBIQUE.md", fermento_path(&fx)).unwrap();
    refused(&fx, &["sweep", "xel", "--answer", file], "FERMENTO_MISSING", 1, "es");
}

#[test]
fn req_fer_5_discard_records_the_move_and_the_next_sweep_drops_the_proposal() {
    let fx = published("discard");
    let doc = sweep(&fx, answer(&fx, &[(2, edit("**6.1.** New."))])).doc();
    let id = doc["proposals"][0].as_str().unwrap().to_owned();
    let before = text(&fx);
    refused(&fx, &["discard", "p-00000000", "--say", SAY], "PROPOSAL_UNKNOWN", 1, "es");
    refused(&fx, &["discard", "nonsense", "--say", SAY], "PROPOSAL_UNKNOWN", 1, "es");
    refused(&fx, &["discard", &id], "WORDS_MISSING", 1, "es");
    refused(&fx, &["discard", &id, "--say", "Estoy de acuerdo."], "WORDS_COPIED", 1, "es");
    let ran = fx.run(&["discard", &id, "--say", SAY, "--json"]);
    ok(&ran);
    let events = fx.events("fermento-xel");
    let last = events.last().unwrap();
    assert_eq!(last["kind"], "proposal-discarded");
    assert_eq!((last["payload"].clone(), last["say"]["text"].as_str()), (json!({"id": id}), Some(SAY)));
    assert_eq!(last["actor"], json!({"kind": "person", "alias": "xel", "role": "maestro"}));
    // It rewrites nothing: the answer shows it gone at the next sweep.
    assert_eq!(text(&fx), before);
    refused(&fx, &["discard", &id, "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
    ok(&sweep(&fx, answer(&fx, &[])));
    assert!(!text(&fx).contains(&id));
    ok(&fx.run(&["check"]));
}

#[test]
fn refusal_discard_by_a_stranger_or_inside_an_agent() {
    let fx = published("discard-refused");
    let id = sweep(&fx, answer(&fx, &[(2, edit("**6.1.** New."))])).doc()["proposals"][0].as_str().unwrap().to_owned();
    for marker in ["CLAUDECODE", "CODEX_SANDBOX"] {
        let before = fx.snapshot();
        let ran = fx.run_env(&["discard", &id, "--say", SAY], &[(marker, "1")]);
        assert_eq!((ran.status, fx.snapshot() == before), (1, true), "{marker}: {ran:?}");
        assert!(ran.stderr.contains("AGENT_ENVIRONMENT"), "{ran:?}");
    }
    fx.git(&["config", "user.signingkey", fx.keys.join("stranger.pub").to_str().unwrap()]);
    refused(&fx, &["discard", &id, "--say", SAY], "ROLE_FORBIDDEN", 1, "en");
}

#[test]
fn req_cli_1_sweep_and_discard_are_declared_with_their_formats() {
    let fx = published("help");
    let help = fx.run(&["sweep", "--help"]);
    ok(&help);
    for field in ["agent", "points", "recommendation", "clarify", "destination", "edit", "units", "result"] {
        assert!(help.stdout.contains(field), "sweep --help lacks {field}");
    }
    assert!(help.stdout.contains("--answer <file>") && help.stdout.contains("ANSWER_INVALID"));
    assert!(fx.run(&["discard", "--help"]).stdout.contains("PROPOSAL_UNKNOWN"));
    assert_eq!(fx.run(&["sweep", "--answer", "a.json"]).status, 2);
    assert_eq!(fx.run(&["discard"]).status, 2);
    let digest = sha256_hex(b"x");
    assert_eq!(digest.len(), 64);
}

#[test]
fn req_fer_3_a_decided_proposal_whose_point_is_gone_is_left_as_it_was() {
    let fx = published("decided-gone");
    let doc = sweep(&fx, answer(&fx, &[(2, edit("**6.1.** Two."))])).doc();
    let id = doc["proposals"][0].as_str().unwrap().to_owned();
    ok(&fx.run(&["discard", &id, "--say", SAY, "--json"]));
    fs::write(fermento_path(&fx), text(&fx).replace("2. Second point,\n   on two lines.\n\n", "")).unwrap();
    fx.commit("input");
    let ran = sweep(&fx, answer(&fx, &[]));
    ok(&ran);
    assert_eq!(ran.doc()["withdrawn"], json!([]));
    assert_eq!(fx.events("fermento-xel").last().unwrap()["kind"], "sweep-recorded");
}

#[test]
fn refusal_record_id_collision_for_an_identifier_another_proposal_holds() {
    let fx = published("collision");
    let given = edit("**6.1.** Two.");
    let two = points(&fx).into_iter().find(|p| p.number == 2).unwrap();
    // The proposal the sweep would make, and another one the record already holds under its identifier.
    let payload = json!({"point": {"number": 2, "digest": two.digest}, "target": given["target"], "base": given["base"],
                         "units": given["units"], "keys": given["keys"], "result": given["result"]});
    let id = format!("p-{}", &subject_digest(&payload).unwrap()[..8]);
    let mut other = payload.clone();
    other["id"] = json!(id);
    other["result"] = json!("1".repeat(64));
    let policy = sha256_hex(fs::read_to_string(fx.domain.join("settings.toml")).unwrap().as_bytes());
    let agent = json!({"kind": "agent", "tool": "claude-code", "provider": "anthropic", "modelRequested": "m", "modelObserved": null});
    let d = Draft {
        kind: "proposal-recorded".into(),
        actor: agent,
        policy,
        instance: "SHA256:fixture".into(),
        say: None,
        payload: other,
    };
    let mut s = Session::begin(&fx.domain, None, &fabrewk_core::identity::PolicyVerifier).unwrap();
    s.append("fermento-xel", None, &d, None).unwrap();
    s.finish(&fabrewk_core::identity::PolicyVerifier, &[]).unwrap();
    let file = answer(&fx, &[(2, given)]);
    refused(&fx, &["sweep", "xel", "--answer", file], "RECORD_ID_COLLISION", 1, "es");
}

#[test]
fn req_fer_1_a_directory_that_is_no_maintainers_is_no_finding_of_c6_or_c12() {
    let fx = published("not-maintainers");
    // One that cannot be an alias, and one a name the domain reserves for something else.
    for dir in ["Notes", "skills", "bootstrap"] {
        fs::create_dir_all(fx.domain.join(dir)).unwrap();
    }
    // C8 names them in its own way; C6 and C12 find nothing there.
    let ran = fx.run(&["check"]);
    for code in ["MAINTAINER_UNLISTED", "FERMENTO_MISSING", "SEPARATOR_MISSING"] {
        assert!(!ran.stdout.contains(code), "{code}: {ran:?}");
    }
}
