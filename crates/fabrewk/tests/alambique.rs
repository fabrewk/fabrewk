//! Part 8b through the built binary (core-specification 12: REQ-FER-5 to REQ-FER-8, PD-41's `approve` half, C7): `approve`
//! of a proposal for the alambique and for the rules, `approve alambique`, `ALAMBIQUE_UNAPPROVED` at `check` and at `run`,
//! and every refusal they give, each asserting that nothing was written (11.2 rule 4).

mod common;

use common::*;
use fabrewk_core::alambique::{Rules, apply, apply_rules, rules_digest};
use fabrewk_core::record::sha256_hex;
use serde_json::{Value, json};
use std::fs;

const SAY: &str = "Lo apruebo: está bien.";
const H: &str = "0000000000000000000000000000000000000000000000000000000000000000";

const DOC: &str = "# Alambique\n\nOpening.\n\n## 1. One\n\n**1.1. A:** first.\n\n**1.2. B:** second.\n\n## 2. Two\n\n**2.1. C:** third [decidido].\n";

fn ok(ran: &Ran) {
    assert_eq!(ran.status, 0, "{ran:?}");
    if !ran.stdout.starts_with('{') {
        whole(&ran.all());
    }
}

fn published(name: &str) -> Fx {
    let fx = Fx::new(name, "published");
    fx.alambique(DOC, "published");
    fx
}

fn file(fx: &Fx) -> String {
    fs::read_to_string(fx.domain.join("ALAMBIQUE.md")).unwrap()
}

fn log(fx: &Fx) -> usize {
    fx.git(&["rev-list", "--count", "HEAD"]).parse().unwrap()
}

fn unit(kind: &str, n: Option<u64>, m: Option<u64>, text: &str) -> Value {
    let mut u = json!({"kind": kind, "text": text});
    for (k, v) in [("n", n), ("m", m)] {
        if let Some(v) = v {
            u[k] = json!(v);
        }
    }
    u
}

/// An edit of the alambique as it stands, whose predicted result is what the units give.
fn edit(fx: &Fx, units: Vec<Value>) -> Value {
    let text = file(fx);
    let result = sha256_hex(apply(&text, &units).unwrap().as_bytes());
    json!({"target": "alambique", "base": sha256_hex(text.as_bytes()), "keys": [], "result": result, "units": units})
}

/// An edit of the rules `held`, whose predicted result is what the units give.
fn rules_edit(held: &Rules, units: Vec<Value>) -> Value {
    let result = rules_digest(&apply_rules(held, &units).unwrap());
    json!({"target": "rules", "base": rules_digest(held), "keys": [], "result": result, "units": units})
}

/// The answer for the fermento as it stands, with `edits` by point number, written outside the domain.
fn answer(fx: &Fx, edits: &[(u32, Value)]) -> &'static str {
    let (text, parts) = fabrewk_core::fermento::open(&fx.domain, "xel").unwrap();
    let rows: Vec<Value> = fabrewk_core::fermento::points(&text[..parts.input_end])
        .iter()
        .map(|p| {
            let rec =
                json!({"point": format!("About {}", p.number), "clarify": "Say it.", "destination": "alambique 6.1"});
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

/// Sweeps with `edits`; gives the proposals' identifiers.
fn sweep(fx: &Fx, edits: &[(u32, Value)]) -> Vec<String> {
    let file = answer(fx, edits);
    let ran = fx.run(&["sweep", "xel", "--answer", file, "--json"]);
    ok(&ran);
    ran.doc()["proposals"].as_array().unwrap().iter().map(|p| p.as_str().unwrap().to_owned()).collect()
}

fn approve(fx: &Fx, what: &str) -> Ran {
    fx.run(&["approve", what, "--say", SAY, "--json"])
}

/// The alambique proposal for point 2 that replaces point 1.2.
fn one_proposal(fx: &Fx) -> (String, Value) {
    let e = edit(fx, vec![unit("point", Some(1), Some(2), "**1.2. B:** rewritten.")]);
    (sweep(fx, &[(2, e.clone())]).remove(0), e)
}

// ---- REQ-FER-5, PD-41: approve a proposal for the alambique ------------------------------------------------------------

#[test]
fn req_fer_5_approve_applies_the_edit_and_commits_the_alambique_alone() {
    let fx = published("approve");
    // A person's staged and unstaged edits stay as they were (PD-41, the approve half).
    fs::write(fx.root.join("README.md"), "staged\n").unwrap();
    fx.git(&["add", "README.md"]);
    fs::write(fx.root.join("README.md"), "unstaged\n").unwrap();
    let (id, e) = one_proposal(&fx);
    let before = log(&fx);
    let ran = approve(&fx, &id);
    ok(&ran);
    let doc = ran.doc();
    assert_eq!(
        (doc["schema"].as_str(), doc["case"].as_str(), doc["id"].as_str()),
        (Some("fabrewk.approved/1"), Some("alambique"), Some(id.as_str()))
    );
    assert_eq!(doc["digest"], e["result"]);
    let expected = DOC.replace("**1.2. B:** second.", "**1.2. B:** rewritten.");
    assert_eq!(file(&fx), expected);
    assert_eq!(sha256_hex(file(&fx).as_bytes()), e["result"].as_str().unwrap());
    // The event is the maestro's, signed, with their words, in the stream of its target.
    let events = fx.events("alambique");
    let kinds: Vec<&str> = events.iter().map(|e| e["kind"].as_str().unwrap()).collect();
    assert_eq!(kinds, ["alambique-baseline", "proposal-incorporated"]);
    assert_eq!(events[1]["payload"], json!({"id": id, "result": e["result"]}));
    assert_eq!(events[1]["say"], json!({"text": SAY, "lang": "es-MX"}));
    // Two commits: the alambique alone, then the record; the first has the person's Git as author and no co-author.
    assert_eq!(log(&fx), before + 2);
    assert_eq!(fx.git(&["show", "--name-only", "--format=", "HEAD~1"]), ".brewstillery/ALAMBIQUE.md");
    assert_eq!(fx.git(&["log", "-1", "--format=%an <%ae>", "HEAD~1"]), "Fixture <fixture@example.invalid>");
    let message = fx.git(&["log", "-1", "--format=%B", "HEAD~1"]);
    assert_eq!(message, format!("alambique: proposal {id}"));
    assert!(!message.contains("Co-Authored-By"));
    let files = fx.git(&["show", "--name-only", "--format=", "HEAD"]);
    assert!(files.contains("data/events/alambique/000002-") && !files.contains("ALAMBIQUE.md"), "{files}");
    assert_eq!(fx.git(&["log", "-1", "--format=%s", "HEAD"]), "fabrewk: record");
    assert_eq!(fx.git(&["diff", "--cached", "--name-only"]), "README.md");
    assert_eq!(fs::read_to_string(fx.root.join("README.md")).unwrap(), "unstaged\n");
    assert_eq!(fx.git(&["status", "--porcelain", "--", ".brewstillery"]), "");
    // C7 passes, and the proposal is incorporated: approving it again is not a move.
    ok(&fx.run(&["check"]));
    refused(&fx, &["approve", &id, "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
}

#[test]
fn req_fer_6_the_opening_a_section_and_a_point_are_replaced_whole_by_one_approval() {
    let fx = published("units");
    let e = edit(
        &fx,
        vec![
            unit("opening", None, None, "# Nuevo\n\nApertura."),
            unit("point", Some(1), Some(1), "**1.1. A:** primero."),
            unit("section", Some(2), None, "## 2. Dos\n\n**2.1. C:** tercero [propuesto]."),
        ],
    );
    let id = sweep(&fx, &[(1, e)]).remove(0);
    ok(&approve(&fx, &id));
    assert_eq!(
        file(&fx),
        "# Nuevo\n\nApertura.\n\n## 1. One\n\n**1.1. A:** primero.\n\n**1.2. B:** second.\n\n## 2. Dos\n\n**2.1. C:** tercero [propuesto].\n"
    );
    ok(&fx.run(&["check"]));
}

#[test]
fn req_fer_5_an_approval_in_a_mode_that_does_not_commit_writes_the_file_only() {
    let fx = Fx::new("unpublished", "unpublished");
    fx.alambique(DOC, "unpublished");
    let (id, _) = one_proposal(&fx);
    let before = log(&fx);
    ok(&approve(&fx, &id));
    assert_eq!(log(&fx), before);
    assert!(file(&fx).contains("rewritten"));
    ok(&fx.run(&["check"]));
    // A direct edit then has no record branch to take the old text from: it records that it is not known.
    fs::write(fx.domain.join("ALAMBIQUE.md"), format!("{}\nAfter.\n", file(&fx))).unwrap();
    ok(&approve(&fx, "alambique"));
    let hunk = &fx.events("alambique").pop().unwrap()["payload"]["difference"];
    assert_eq!(hunk["removed"], Value::Null);
    assert!(hunk["added"].as_array().unwrap().iter().any(|l| l == "After."));
}

// ---- Refusals of approve ------------------------------------------------------------------------------------------------

#[test]
fn refusal_proposal_stale_the_point_changed_moved_or_the_draft_does_not_give_its_result() {
    // The text of the point changed after the sweep.
    let fx = published("stale-text");
    let (id, _) = one_proposal(&fx);
    let path = fx.domain.join("xel/fermento.md");
    let original = fs::read_to_string(&path).unwrap();
    fs::write(&path, original.replace("Second point,", "Second point, changed,")).unwrap();
    fx.commit("input edited");
    let e = refused(&fx, &["approve", &id, "--say", SAY], "PROPOSAL_STALE", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("point 2"), "{e}");
    // The same words under another number do not count.
    fs::write(
        &path,
        original
            .replace("1. First point.", "1. A new one.\n\n2. First point.")
            .replace("2. Second point,", "3. Second point,"),
    )
    .unwrap();
    fx.commit("input renumbered");
    refused(&fx, &["approve", &id, "--say", SAY], "PROPOSAL_STALE", 1, "es");
    // The draft does not give the result it predicted.
    let fx = published("stale-draft");
    let mut e = edit(&fx, vec![unit("point", Some(1), Some(2), "**1.2. B:** rewritten.")]);
    e["result"] = json!(H);
    let id = sweep(&fx, &[(2, e)]).remove(0);
    let e = refused(&fx, &["approve", &id, "--say", SAY], "PROPOSAL_STALE", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("predicted"), "{e}");
    assert_eq!(file(&fx), DOC);
}

#[test]
fn refusal_alambique_changed_in_the_tree_the_commit_or_the_index() {
    // Edited and committed after the sweep: the file is not the text the edit starts from.
    let fx = published("changed");
    let (id, _) = one_proposal(&fx);
    fs::write(fx.domain.join("ALAMBIQUE.md"), format!("{DOC}\nMore.\n")).unwrap();
    let e = refused(&fx, &["approve", &id, "--say", SAY], "ALAMBIQUE_CHANGED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("starts from"), "{e}");
    // The tree is as drafted, and the index holds another text.
    fx.git(&["add", ".brewstillery/ALAMBIQUE.md"]);
    fs::write(fx.domain.join("ALAMBIQUE.md"), DOC).unwrap();
    let e = refused(&fx, &["approve", &id, "--say", SAY], "ALAMBIQUE_CHANGED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("index"), "{e}");
    // The alambique gone.
    fx.git(&["reset", "-q"]);
    fs::remove_file(fx.domain.join("ALAMBIQUE.md")).unwrap();
    refused(&fx, &["approve", &id, "--say", SAY], "ALAMBIQUE_CHANGED", 1, "es");
}

#[test]
fn refusal_alambique_unapproved_an_edit_drafted_from_a_text_nobody_approved() {
    let fx = published("drafted-unapproved");
    fs::write(fx.domain.join("ALAMBIQUE.md"), format!("{DOC}\nTyped by hand.\n")).unwrap();
    fx.commit("a hand edit");
    let (id, _) = one_proposal(&fx);
    let e = refused(&fx, &["approve", &id, "--say", SAY], "ALAMBIQUE_UNAPPROVED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("differs"), "{e}");
}

#[test]
fn refusal_unit_ambiguous_a_section_that_stands_twice() {
    let fx = Fx::new("ambiguous", "published");
    fx.alambique(&format!("{DOC}\n## 2. Again\n\n**2.2. D:** fourth.\n"), "published");
    let mut e = edit(&fx, vec![unit("point", Some(1), Some(1), "x")]);
    e["units"] = json!([unit("section", Some(2), None, "## 2. New")]);
    e["result"] = json!(H);
    let id = sweep(&fx, &[(1, e)]).remove(0);
    let e = refused(&fx, &["approve", &id, "--say", SAY], "UNIT_AMBIGUOUS", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("section 2"), "{e}");
}

#[test]
fn refusal_move_not_allowed_a_discarded_proposal_and_the_other_refusals_of_the_move() {
    let fx = published("moves");
    let (id, _) = one_proposal(&fx);
    ok(&fx.run(&["discard", &id, "--say", SAY]));
    let e = refused(&fx, &["approve", &id, "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("discarded"), "{e}");
    refused(&fx, &["approve", "p-00000000", "--say", SAY], "PROPOSAL_UNKNOWN", 1, "es");
    let (id, _) = {
        let e = edit(&fx, vec![unit("point", Some(1), Some(2), "**1.2. B:** other.")]);
        (sweep(&fx, &[(2, e.clone())]).remove(0), e)
    };
    refused(&fx, &["approve", &id], "WORDS_MISSING", 1, "es");
    refused(&fx, &["approve", &id, "--say", "   "], "WORDS_MISSING", 1, "es");
    // A key that signs for no maintainer of the policy.
    fx.git(&["config", "user.signingkey", fx.keys.join("stranger.pub").to_str().unwrap()]);
    // Nobody the policy knows holds that key, so the words are the policy's: English.
    refused(&fx, &["approve", &id, "--say", SAY], "ROLE_FORBIDDEN", 1, "en");
}

// ---- REQ-FER-6: the rules ----------------------------------------------------------------------------------------------

#[test]
fn req_fer_6_a_rules_proposal_is_recorded_in_its_stream_and_leaves_the_alambique_alone() {
    let fx = published("rules");
    let none = Rules::new();
    let first =
        rules_edit(&none, vec![unit("rule", Some(1), None, "Be brief."), unit("rule", Some(2), None, "Be clear.")]);
    let held = apply_rules(&none, first["units"].as_array().unwrap()).unwrap();
    let second = rules_edit(
        &held,
        vec![unit("rule", Some(1), None, "Be briefer."), json!({"kind": "rule", "n": 2, "text": null})],
    );
    // Two proposals from the same rules: only the first can be approved.
    let ids = sweep(&fx, &[(1, first.clone()), (2, first.clone())]);
    assert_eq!(ids.len(), 2);
    let alambique_commits = fx.git(&["log", "--format=%H", "--", ".brewstillery/ALAMBIQUE.md"]);
    let before = log(&fx);
    let ran = approve(&fx, &ids[0]);
    ok(&ran);
    assert_eq!(ran.doc()["case"], "rules");
    assert_eq!(log(&fx), before + 1, "one record commit, no alambique commit");
    assert_eq!(fx.git(&["log", "--format=%H", "--", ".brewstillery/ALAMBIQUE.md"]), alambique_commits);
    let e = fx.events("rules").pop().unwrap();
    assert_eq!(e["kind"], "proposal-incorporated");
    assert_eq!(
        e["payload"],
        json!({"id": ids[0], "result": first["result"], "rules": {"R1": "Be brief.", "R2": "Be clear."}})
    );
    assert_eq!(file(&fx), DOC);
    assert_eq!(fx.events("alambique").len(), 1);
    ok(&fx.run(&["check"]));
    // The other starts from rules that are no longer the project's.
    let e = refused(&fx, &["approve", &ids[1], "--say", SAY], "PROPOSAL_STALE", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("rules"), "{e}");
    // A proposal from the rules as they stand replaces one and removes the other.
    fx.commit("answer");
    let id = sweep(&fx, &[(1, second.clone())]).remove(0);
    ok(&approve(&fx, &id));
    let e = fx.events("rules").pop().unwrap();
    assert_eq!(e["payload"]["rules"], json!({"R1": "Be briefer."}));
    assert_eq!(e["payload"]["result"], second["result"]);
    assert_eq!(fx.events("rules").len(), 2);
    ok(&fx.run(&["check"]));
}

// ---- REQ-FER-7: C7 and the direct edit ------------------------------------------------------------------------------------

#[test]
fn req_fer_7_a_byte_changed_by_hand_is_unapproved_until_a_maestro_approves_it() {
    let fx = published("direct");
    ok(&fx.run(&["check"]));
    // A status key changed by hand outside an edit, and a proposal's state typed into the alambique (REQ-FER-8).
    fs::write(fx.domain.join("ALAMBIQUE.md"), DOC.replace("[decidido]", "[propuesto]")).unwrap();
    let e = refused(&fx, &["check"], "ALAMBIQUE_UNAPPROVED", 3, "es");
    assert_eq!((e["check"].as_str(), e["control"].as_str()), (Some("C7"), Some("effective")));
    // `run` refuses it before confinement, with nothing written.
    let (_, task) = fx.stage_and_task();
    let (shown, id) =
        (fx.run(&["status", &task, "--json"]).doc()["shown"]["digest"].as_str().unwrap().to_owned(), task);
    ok(&fx.run(&["authorize", &id, "--seen", &shown, "--say", SAY]));
    let e = refused(&fx, &["run", &id, "--say", SAY], "ALAMBIQUE_UNAPPROVED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("differs"), "{e}");
    // The maestro approves what is there: the event holds both digests and the one hunk; the commit holds the alambique.
    let before = log(&fx);
    let new = file(&fx);
    let ran = approve(&fx, "alambique");
    ok(&ran);
    assert_eq!(ran.doc()["case"], "direct");
    let e = fx.events("alambique").pop().unwrap();
    assert_eq!(e["kind"], "direct-edit-approved");
    assert_eq!(e["payload"]["old"], sha256_hex(DOC.as_bytes()));
    assert_eq!(e["payload"]["new"], sha256_hex(new.as_bytes()));
    assert_eq!(
        e["payload"]["difference"],
        json!({"from": 13, "removed": ["**2.1. C:** third [decidido]."], "added": ["**2.1. C:** third [propuesto]."]})
    );
    assert_eq!(log(&fx), before + 2);
    assert_eq!(fx.git(&["show", "--name-only", "--format=", "HEAD~1"]), ".brewstillery/ALAMBIQUE.md");
    ok(&fx.run(&["check"]));
    // Nothing to approve now.
    let e = refused(&fx, &["approve", "alambique", "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("equals"), "{e}");
}

#[test]
fn req_fer_7_a_direct_edit_the_person_already_committed_has_no_commit_of_the_alambique() {
    let fx = published("committed");
    fs::write(fx.domain.join("ALAMBIQUE.md"), format!("{DOC}\nAfter.\n")).unwrap();
    fx.commit("my edit");
    let before = log(&fx);
    ok(&approve(&fx, "alambique"));
    assert_eq!(log(&fx), before + 1);
    assert!(!fx.git(&["show", "--name-only", "--format=", "HEAD"]).contains("ALAMBIQUE.md"));
    ok(&fx.run(&["check"]));
}

#[test]
fn req_fer_7_a_file_nobody_approved_is_unapproved_and_its_approval_is_the_baseline() {
    let fx = Fx::new("baseline", "published");
    // No file and nothing approved: nothing to approve.
    ok(&fx.run(&["check"]));
    refused(&fx, &["approve", "alambique", "--say", SAY], "ALAMBIQUE_MISSING", 1, "es");
    fs::write(fx.domain.join("ALAMBIQUE.md"), DOC).unwrap();
    fx.commit("an alambique");
    let e = refused(&fx, &["check"], "ALAMBIQUE_UNAPPROVED", 3, "es");
    assert!(e["detail"].as_str().unwrap().contains("never approved"), "{e}");
    let before = log(&fx);
    let ran = approve(&fx, "alambique");
    ok(&ran);
    assert_eq!(ran.doc()["case"], "baseline");
    let e = fx.events("alambique").pop().unwrap();
    assert_eq!(
        (e["kind"].as_str(), &e["payload"]),
        (Some("alambique-baseline"), &json!({"digest": sha256_hex(DOC.as_bytes())}))
    );
    assert_eq!(log(&fx), before + 1);
    ok(&fx.run(&["check"]));
    // A directory, or a link, where the file stands.
    fs::remove_file(fx.domain.join("ALAMBIQUE.md")).unwrap();
    fs::create_dir(fx.domain.join("ALAMBIQUE.md")).unwrap();
    refused(&fx, &["approve", "alambique", "--say", SAY], "ALAMBIQUE_MISSING", 1, "es");
    refused(&fx, &["check"], "ALAMBIQUE_UNAPPROVED", 3, "es");
}
