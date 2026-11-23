//! The formula's subphase 1a through the built binary (FORMULA.md, "1a. The formula in the record"): `FORMULA.md` in
//! the domain (point 1); its edits as proposals and its direct approval, each one signed move whose digest the record
//! keeps (point 2); the bridge imported from a maestro's signed commit, refused for an unsigned commit and for one
//! signed by another key (point 3); `FORMULA_UNAPPROVED` stopping `run`, `accept` and `close` and found by `check`
//! (point 4); and the formula's state, its cut in progress and what waits for the maestro in `status` and `STATUS.md`
//! (point 5). Every refusal asserts that nothing was written (11.2 rule 4).

mod common;

use common::*;
use fabrewk_core::record::{Draft, Say, Session, sha256_hex};
use serde_json::{Value, json};
use std::fs;

const SAY: &str = "La apruebo: es la hoja de ruta.";
const WRITES: &str = "mkdir -p src && printf 'new\\n' > src/new.txt\n";

/// A formula with two cuts in its first phase, and a heading in a fence that is no cut.
const FORMULA: &str = "# Formula\n\nHow it works.\n\n## Phase 1: the first\n\n### 1a. The first cut\n\n1. Do it.\n\n\
                       ### 1b. The second cut\n\n1. Then this.\n\n```\n### 9z. Not a cut\n```\n";

fn ok(ran: &Ran) {
    assert_eq!(ran.status, 0, "{ran:?}");
    if !ran.stdout.starts_with('{') {
        whole(&ran.all());
    }
}

fn path(fx: &Fx) -> std::path::PathBuf {
    fx.domain.join("FORMULA.md")
}

fn approve(fx: &Fx, what: &str) -> Ran {
    fx.run(&["approve", what, "--say", SAY, "--json"])
}

fn log(fx: &Fx) -> usize {
    fx.git(&["rev-list", "--count", "HEAD"]).parse().unwrap()
}

/// A published fixture whose formula is `FORMULA`, approved by `xel` and committed.
fn approved(name: &str) -> Fx {
    let fx = Fx::new(name, "published");
    fs::write(path(&fx), FORMULA).unwrap();
    ok(&approve(&fx, "formula"));
    fx
}

/// Commits `FORMULA.md` (and whatever else is staged), signed with `key`'s key, or unsigned with `None`; gives its id.
fn commit_signed(fx: &Fx, key: Option<&str>, message: &str) -> String {
    fx.git(&["add", ".brewstillery/FORMULA.md"]);
    match key {
        Some(k) => {
            let key = format!("user.signingkey={}", fx.keys.join(format!("{k}.pub")).display());
            fx.git(&["-c", "gpg.format=ssh", "-c", &key, "commit", "-q", "-S", "-m", message]);
        }
        None => _ = fx.git(&["commit", "-q", "-m", message]),
    }
    fx.git(&["rev-parse", "HEAD"])
}

// ---- Point 1: FORMULA.md is part of the domain -------------------------------------------------------------------------

#[test]
fn formula_1a_1_formula_md_is_part_of_the_domain_and_the_installation_writes_none() {
    // The installation writes no formula, and the record holds none: a project has none until it is proposed.
    let fx = Fx::applied("formula-installed", "published", None);
    assert!(!path(&fx).exists());
    assert!(fx.events("formula").is_empty());
    let doc = fx.run(&["check", "--json"]).doc();
    assert_eq!(doc["findings"], json!([]), "{doc}");
    // A formula written is no unknown entry of the domain: it is the formula, unapproved until a maestro approves it.
    let fx = Fx::new("formula-entry", "published");
    fs::write(path(&fx), FORMULA).unwrap();
    let doc = fx.run(&["check", "--json"]).doc();
    let codes: Vec<&str> = doc["findings"].as_array().unwrap().iter().map(|f| f["code"].as_str().unwrap()).collect();
    assert_eq!(codes, ["FORMULA_UNAPPROVED"], "{doc}");
    // A FORMULA.md that is not a file is no formula, as for any file of the domain: a link is an unknown entry, never
    // followed, and a directory a name that is not an alias.
    let code = |fx: &Fx, code: &str| {
        let doc = fx.run(&["check", "--json"]).doc();
        assert!(doc["findings"].as_array().unwrap().iter().any(|f| f["code"] == code), "{code}: {doc}");
    };
    fs::rename(path(&fx), fx.root.join("elsewhere.md")).unwrap();
    std::os::unix::fs::symlink(fx.root.join("elsewhere.md"), path(&fx)).unwrap();
    code(&fx, "DOMAIN_UNKNOWN_ENTRY");
    code(&fx, "FORMULA_UNAPPROVED");
    fs::remove_file(path(&fx)).unwrap();
    fs::create_dir(path(&fx)).unwrap();
    code(&fx, "ALIAS_INVALID");
}

// ---- Point 2: an edit is a proposal, approved with one signed move ------------------------------------------------------

#[test]
fn formula_1a_2_a_maestro_approves_the_formula_as_it_stands_and_the_record_keeps_each_digest() {
    let fx = Fx::new("formula-direct", "published");
    // No formula: nothing to approve.
    refused(&fx, &["approve", "formula", "--say", SAY], "FORMULA_MISSING", 1, "es");
    fs::write(path(&fx), FORMULA).unwrap();
    // A person's move: without their words, refused.
    refused(&fx, &["approve", "formula"], "WORDS_MISSING", 1, "es");
    let before = log(&fx);
    let ran = approve(&fx, "formula");
    ok(&ran);
    let first = sha256_hex(FORMULA.as_bytes());
    assert_eq!((ran.doc()["case"].as_str(), ran.doc()["digest"].as_str()), (Some("formula-direct"), Some(&first[..])));
    let e = fx.events("formula").pop().unwrap();
    assert_eq!((e["kind"].as_str(), &e["payload"]), (Some("formula-approved"), &json!({"old": null, "digest": first})));
    assert_eq!(e["say"], json!({"text": SAY, "lang": "es-MX"}));
    assert_eq!(e["actor"], json!({"kind": "person", "alias": "xel", "role": "maestro"}));
    // Two commits: the formula alone, by the person's Git with no co-author, then the record.
    assert_eq!(log(&fx), before + 2);
    assert_eq!(fx.git(&["show", "--name-only", "--format=", "HEAD~1"]), ".brewstillery/FORMULA.md");
    assert_eq!(fx.git(&["log", "-1", "--format=%B", "HEAD~1"]), "formula: formula-approved");
    ok(&fx.run(&["check"]));
    refused(&fx, &["approve", "formula", "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
    // An edit: unapproved until a maestro approves it again, which records the digest it replaces.
    let second = format!("{FORMULA}\nA new line.\n");
    fs::write(path(&fx), &second).unwrap();
    refused(&fx, &["check"], "FORMULA_UNAPPROVED", 3, "es");
    ok(&approve(&fx, "formula"));
    let digests: Vec<Value> = fx.events("formula").iter().map(|e| e["payload"].clone()).collect();
    assert_eq!(
        digests,
        [json!({"old": null, "digest": first}), json!({"old": first, "digest": sha256_hex(second.as_bytes())})]
    );
    ok(&fx.run(&["check"]));
    // Inside an agent, the move is refused with nothing written.
    fs::write(path(&fx), FORMULA).unwrap();
    let before = fx.snapshot();
    let ran = fx.run_env(&["approve", "formula", "--say", SAY, "--json"], &[("CLAUDECODE", "1")]);
    assert_eq!((ran.status, ran.doc()["refusals"][0]["code"].as_str()), (1, Some("AGENT_ENVIRONMENT")), "{ran:?}");
    assert_eq!(before, fx.snapshot());
}

/// The answer for `xel`'s fermento as it stands, with `edit` on point 1; gives the proposals' identifiers.
fn sweep(fx: &Fx, edit: Value) -> Ran {
    let (text, parts) = fabrewk_core::fermento::open(&fx.domain, "xel").unwrap();
    let rows: Vec<Value> = fabrewk_core::fermento::points(&text[..parts.input_end])
        .iter()
        .map(|p| {
            let rec = json!({"point": "The roadmap", "clarify": "Say it.", "destination": "the formula"});
            let mut row = json!({"number": p.number, "digest": p.digest, "recommendation": rec});
            if p.number == 1 {
                row["edit"] = edit.clone();
            }
            row
        })
        .collect();
    let doc =
        json!({"agent": {"tool": "claude-code", "provider": "anthropic", "model": "claude-opus-5-5"}, "points": rows});
    fs::write(fx.root.join("answer.json"), serde_json::to_vec(&doc).unwrap()).unwrap();
    fx.run(&["sweep", "xel", "--answer", "answer.json", "--json"])
}

fn formula_edit(base: &str, text: &str) -> Value {
    json!({
        "target": "formula", "base": sha256_hex(base.as_bytes()), "keys": [],
        "units": [{"kind": "formula", "text": text}], "result": sha256_hex(text.as_bytes()),
    })
}

#[test]
fn formula_1a_2_a_formula_edit_is_a_proposal_a_maestro_approves_with_one_signed_move() {
    let fx = approved("formula-proposal");
    let new = FORMULA.replace("Then this.", "Then that.");
    let ran = sweep(&fx, formula_edit(FORMULA, &new));
    ok(&ran);
    let id = ran.doc()["proposals"][0].as_str().unwrap().to_owned();
    // The fermento's answer shows it, with what approving it does.
    let answer = fs::read_to_string(fx.domain.join("xel/fermento.md")).unwrap();
    assert!(answer.contains(&fabrewk_core::cli::say("en", "fermento.target.formula", &[])), "{answer}");
    assert!(answer.contains(&format!("fabrewk approve {id}")) && answer.contains("Then that."), "{answer}");
    let before = log(&fx);
    let ran = approve(&fx, &id);
    ok(&ran);
    assert_eq!((ran.doc()["case"].as_str(), ran.doc()["id"].as_str()), (Some("formula"), Some(id.as_str())));
    assert_eq!(fs::read_to_string(path(&fx)).unwrap(), new);
    let e = fx.events("formula").pop().unwrap();
    assert_eq!(
        (e["kind"].as_str(), &e["payload"]),
        (Some("proposal-incorporated"), &json!({"id": id, "result": sha256_hex(new.as_bytes())}))
    );
    assert_eq!(e["say"]["text"], SAY);
    assert_eq!(log(&fx), before + 2);
    assert_eq!(fx.git(&["show", "--name-only", "--format=", "HEAD~1"]), ".brewstillery/FORMULA.md");
    assert_eq!(fx.git(&["log", "-1", "--format=%B", "HEAD~1"]), format!("formula: proposal {id}"));
    ok(&fx.run(&["check"]));
    refused(&fx, &["approve", &id, "--say", SAY], "MOVE_NOT_ALLOWED", 1, "es");
}

#[test]
fn formula_1a_2_refusals_of_a_formula_proposal() {
    let fx = approved("formula-proposal-refused");
    let new = FORMULA.replace("Do it.", "Do it well.");
    // A unit of the formula's kind for another target, or two units: not an edit of the formula.
    let mut bad = formula_edit(FORMULA, &new);
    bad["target"] = json!("alambique");
    let before = fx.snapshot();
    let ran = sweep(&fx, bad);
    assert_eq!((ran.status, ran.doc()["refusals"][0]["code"].as_str()), (1, Some("ANSWER_INVALID")), "{ran:?}");
    let mut two = formula_edit(FORMULA, &new);
    two["units"] = json!([{"kind": "formula", "text": new}, {"kind": "formula", "text": "x"}]);
    let ran = sweep(&fx, two);
    assert_eq!((ran.status, ran.doc()["refusals"][0]["code"].as_str()), (1, Some("ANSWER_INVALID")), "{ran:?}");
    // A formula unit that names a number, as a section or a point does.
    let mut numbered = formula_edit(FORMULA, &new);
    numbered["units"] = json!([{"kind": "formula", "n": 1, "text": new}]);
    let ran = sweep(&fx, numbered);
    assert_eq!((ran.status, ran.doc()["refusals"][0]["code"].as_str()), (1, Some("ANSWER_INVALID")), "{ran:?}");
    fs::remove_file(fx.root.join("answer.json")).unwrap();
    assert_eq!(before, fx.snapshot());
    // The formula changed since the proposal was drafted: FORMULA_CHANGED, then FORMULA_UNAPPROVED once that change is
    // the text the proposal starts from but no maestro approved it.
    let ran = sweep(&fx, formula_edit(FORMULA, &new));
    ok(&ran);
    let id = ran.doc()["proposals"][0].as_str().unwrap().to_owned();
    fx.commit("answer");
    fs::write(path(&fx), format!("{FORMULA}\nBy hand.\n")).unwrap();
    refused(&fx, &["approve", &id, "--say", SAY], "FORMULA_CHANGED", 1, "es");
    let by_hand = format!("{FORMULA}\nBy hand.\n");
    fx.commit("by hand");
    let ran = sweep(&fx, formula_edit(&by_hand, &new));
    ok(&ran);
    let id = ran.doc()["proposals"][0].as_str().unwrap().to_owned();
    fx.commit("answer");
    refused(&fx, &["approve", &id, "--say", SAY], "FORMULA_UNAPPROVED", 1, "es");
    // A directory where the formula stands is no text to start from.
    fs::remove_file(path(&fx)).unwrap();
    fs::create_dir(path(&fx)).unwrap();
    let e = refused(&fx, &["approve", &id, "--say", SAY], "FORMULA_CHANGED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("no file to start from"), "{e}");
}

// ---- Point 3: the bridge -----------------------------------------------------------------------------------------------

#[test]
fn formula_1a_3_a_maestro_s_signed_commit_of_the_formula_alone_is_its_first_approval() {
    let fx = Fx::new("formula-bridge", "published");
    fs::write(path(&fx), FORMULA).unwrap();
    let commit = commit_signed(&fx, Some("xel"), "The formula");
    // No new signature and no words: an agent runs it.
    let ran = fx.run_env(&["import", &commit, "--json"], &[("CLAUDECODE", "1")]);
    ok(&ran);
    let digest = sha256_hex(FORMULA.as_bytes());
    assert_eq!(
        (ran.doc()["schema"].as_str(), &ran.doc()["bridge"], &ran.doc()["digest"], &ran.doc()["alias"]),
        (Some("fabrewk.imported/1"), &json!(commit), &json!(digest), &json!("xel"))
    );
    let events = fx.events("formula");
    assert_eq!(events.len(), 1);
    assert_eq!(
        (events[0]["kind"].as_str(), &events[0]["actor"], &events[0]["payload"]),
        (
            Some("formula-imported"),
            &json!({"kind": "fabrewk"}),
            &json!({"commit": commit, "digest": digest, "alias": "xel"})
        )
    );
    assert!(events[0].get("say").is_none());
    let sigs = fs::read_dir(fx.domain.join("data/events/formula")).unwrap();
    assert!(sigs.flatten().all(|e| !e.file_name().to_string_lossy().ends_with(".sig")));
    ok(&fx.run(&["check"]));
    let status = fs::read_to_string(fx.domain.join("STATUS.md")).unwrap();
    assert!(
        status.contains(&format!("imported from the commit {} that the maestro xel signed", &commit[..12])),
        "{status}"
    );
    // Only the first approval is a bridge.
    refused(&fx, &["import", &commit], "MOVE_NOT_ALLOWED", 1, "es");
    // The formula then changes only by a maestro's approval.
    fs::write(path(&fx), format!("{FORMULA}\nMore.\n")).unwrap();
    refused(&fx, &["check"], "FORMULA_UNAPPROVED", 3, "es");
    ok(&approve(&fx, "formula"));
    assert_eq!(fx.events("formula").pop().unwrap()["payload"]["old"], json!(digest));
}

#[test]
fn formula_1a_3_refusal_formula_bridge_invalid_unsigned_another_key_not_alone_or_elsewhere() {
    let fx = Fx::new("formula-bridge-refused", "published");
    let invalid = |rev: &str, why: &str| {
        let e = refused(&fx, &["import", rev], "FORMULA_BRIDGE_INVALID", 1, "es");
        assert!(e["detail"].as_str().unwrap().contains(why), "{e}");
    };
    // An unsigned commit.
    fs::write(path(&fx), FORMULA).unwrap();
    let unsigned = commit_signed(&fx, None, "unsigned");
    invalid(&unsigned, "not signed with the key of a maestro");
    // A commit signed by another key, which no maestro holds.
    fs::write(path(&fx), format!("{FORMULA}\nOther.\n")).unwrap();
    let stranger = commit_signed(&fx, Some("stranger"), "signed by a stranger");
    invalid(&stranger, "not signed with the key of a maestro");
    // A maestro's signed commit that changes more than the formula.
    fs::write(path(&fx), format!("{FORMULA}\nThird.\n")).unwrap();
    fs::write(fx.root.join("README.md"), "changed too\n").unwrap();
    fx.git(&["add", "README.md"]);
    let both = commit_signed(&fx, Some("xel"), "the formula and the readme");
    invalid(&both, "does not change .brewstillery/FORMULA.md alone");
    // A maestro's signed commit that does not change it.
    fs::write(fx.root.join("README.md"), "only the readme\n").unwrap();
    fx.git(&["add", "README.md"]);
    let other = commit_signed(&fx, Some("xel"), "the readme alone");
    invalid(&other, "does not change .brewstillery/FORMULA.md alone");
    // A maestro's signed commit of the formula alone, outside the record branch's history.
    fx.git(&["checkout", "-q", "-b", "elsewhere"]);
    fs::write(path(&fx), format!("{FORMULA}\nElsewhere.\n")).unwrap();
    let elsewhere = commit_signed(&fx, Some("xel"), "elsewhere");
    fx.git(&["checkout", "-q", "main"]);
    invalid(&elsewhere, "not in the history of the record branch");
    // A name that is no commit.
    invalid("no-such-commit", "names no commit");
    assert!(fx.events("formula").is_empty());
}

/// A program at `fx.root/../<name>` that runs `body` and exits 0, whatever it is asked.
fn fake(fx: &Fx, name: &str, body: &str) -> String {
    let file = fx.root.parent().unwrap().join(name);
    fs::write(&file, format!("#!/bin/sh\ncat >/dev/null\n{body}\nexit 0\n")).unwrap();
    fs::set_permissions(&file, std::os::unix::fs::PermissionsExt::from_mode(0o755)).unwrap();
    file.display().to_string()
}

#[test]
fn formula_1a_3_the_bridge_is_verified_by_ssh_keygen_whatever_git_s_configuration_names() {
    let fx = Fx::new("formula-bridge-verifier", "published");
    // A verifier the repository's configuration names, which says every SSH signature is xel's: a stranger's commit
    // is still refused.
    fs::write(path(&fx), FORMULA).unwrap();
    let stranger = commit_signed(&fx, Some("stranger"), "signed by a stranger");
    let good = "case \"$*\" in *find-principals*) echo xel ;; \
                *) echo 'Good \"git\" signature for xel with ED25519 key SHA256:x' ;; esac";
    let ssh = fake(&fx, "fake-ssh-keygen", good);
    fx.git(&["config", "gpg.ssh.program", &ssh]);
    let e = refused(&fx, &["import", &stranger], "FORMULA_BRIDGE_INVALID", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("not signed with the key of a maestro"), "{e}");
    // An OpenPGP signature that a configured program says is good and fully trusted, its signer named xel: refused,
    // for only an SSH signature by a maestro's key is a bridge.
    fs::write(path(&fx), format!("{FORMULA}\nOpenPGP.\n")).unwrap();
    let unsigned = commit_signed(&fx, None, "to be wrapped");
    let object = fx.git(&["cat-file", "commit", &unsigned]);
    let (head, message) = object.split_once("\n\n").unwrap();
    let pgp = "gpgsig -----BEGIN PGP SIGNATURE-----\n \n AAAA\n -----END PGP SIGNATURE-----";
    let forged = format!("{head}\n{pgp}\n\n{message}\n");
    let file = fx.root.parent().unwrap().join("forged-commit");
    fs::write(&file, forged).unwrap();
    let pgp = fx.git(&["hash-object", "-t", "commit", "-w", file.to_str().unwrap()]);
    fx.git(&["update-ref", "HEAD", &pgp]);
    let status =
        "printf '%s\\n' '[GNUPG:] NEWSIG' '[GNUPG:] GOODSIG 0123456789ABCDEF xel' '[GNUPG:] TRUST_ULTIMATE 0 pgp'";
    let gpg = fake(&fx, "fake-gpg", status);
    for key in ["gpg.program", "gpg.openpgp.program"] {
        fx.git(&["config", key, &gpg]);
    }
    let e = refused(&fx, &["import", &pgp], "FORMULA_BRIDGE_INVALID", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("not signed with the key of a maestro"), "{e}");
    assert!(fx.events("formula").is_empty());
}

#[test]
fn formula_1a_3_refusal_formula_bridge_invalid_for_a_merge() {
    // A maestro's signed merge whose own change is the formula alone: a bridge is a commit on one parent.
    let fx = Fx::new("formula-bridge-merge", "published");
    fx.git(&["checkout", "-q", "-b", "side"]);
    fx.git(&["commit", "-q", "--allow-empty", "-m", "side"]);
    fx.git(&["checkout", "-q", "main"]);
    fx.git(&["commit", "-q", "--allow-empty", "-m", "main"]);
    fx.git(&["merge", "-q", "--no-ff", "--no-commit", "side"]);
    fs::write(path(&fx), FORMULA).unwrap();
    let merge = commit_signed(&fx, Some("xel"), "the formula in a merge");
    let e = refused(&fx, &["import", &merge], "FORMULA_BRIDGE_INVALID", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("is a merge"), "{e}");
}

#[test]
fn formula_1a_3_refusal_formula_bridge_invalid_for_a_formula_that_is_not_a_regular_file() {
    let fx = Fx::new("formula-bridge-kind", "published");
    // A maestro's signed commit that removes the formula, or makes it a link: no text to approve.
    fs::write(path(&fx), FORMULA).unwrap();
    fx.commit("a formula, unsigned");
    fx.git(&["rm", "-q", ".brewstillery/FORMULA.md"]);
    fx.git(&["-c", "gpg.format=ssh", "commit", "-q", "-S", "-m", "the formula removed"]);
    let removed = fx.git(&["rev-parse", "HEAD"]);
    let e = refused(&fx, &["import", &removed], "FORMULA_BRIDGE_INVALID", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("as a regular file"), "{e}");
    std::os::unix::fs::symlink("ALAMBIQUE.md", path(&fx)).unwrap();
    let link = commit_signed(&fx, Some("xel"), "the formula a link");
    let e = refused(&fx, &["import", &link], "FORMULA_BRIDGE_INVALID", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("as a regular file"), "{e}");
}

// ---- Point 4: a formula changed since its approval stops every run -----------------------------------------------------

#[test]
fn formula_1a_4_an_unapproved_formula_stops_run_and_accept_and_check_reports_it() {
    let fx = Fx::builder("formula-run", "published", &format!("{WRITES}{SUCCESS}"));
    // No formula and nothing approved: nothing stops.
    let id = verified_task(&fx, json!({}), json!({}));
    fs::write(path(&fx), FORMULA).unwrap();
    let e = refused(&fx, &["check"], "FORMULA_UNAPPROVED", 3, "es");
    assert_eq!((e["check"].as_str(), e["control"].as_str()), (Some("C7"), Some("effective")));
    assert!(e["detail"].as_str().unwrap().contains("never approved"), "{e}");
    let accept = ["accept", &id, "--seen", &accepting(&fx, &id), "--say", SAY];
    refused(&fx, &accept, "FORMULA_UNAPPROVED", 1, "es");
    ok(&approve(&fx, "formula"));
    fx.commit("approved");
    // Changed since its approval: the run is refused after REQ-RUN-2 and before confinement.
    fs::write(path(&fx), format!("{FORMULA}\nChanged.\n")).unwrap();
    let e = refused(&fx, &["check"], "FORMULA_UNAPPROVED", 3, "es");
    assert!(e["detail"].as_str().unwrap().contains("differs"), "{e}");
    let other = verified_task_after_approval(&fx);
    let e = refused(&fx, &["run", &other, "--say", SAY], "FORMULA_UNAPPROVED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("differs"), "{e}");
    // Gone after an approval, too.
    fs::remove_file(path(&fx)).unwrap();
    let e = refused(&fx, &["check"], "FORMULA_UNAPPROVED", 3, "es");
    assert!(e["detail"].as_str().unwrap().contains("gone"), "{e}");
}

/// A task proposed and authorized, not run: what `run` starts.
fn verified_task_after_approval(fx: &Fx) -> String {
    let head = fx.git(&["rev-parse", "HEAD"]);
    let stage = json!({"lang": "en", "goal": "Build more.", "budget": {"dispatches": 6, "minutes": 120}, "checks": {}, "settings": {}});
    let task = json!({
        "lang": "en", "goal": "Make more.", "requirements": [{"text": "It builds.", "criterion": "it builds"}],
        "base": head, "dependencies": [], "allowed": ["src/**"], "profiles": {"build": "build", "review": "review"},
        "checks": [], "limits": {"rounds": 3, "dispatches": 6, "minutes": 120}, "delegationLanguage": "en",
        "settings": {},
    });
    fs::write(fx.root.join("plan.json"), serde_json::to_vec(&json!({"stage": stage, "tasks": [task]})).unwrap())
        .unwrap();
    let ran = fx.run(&["propose", "plan.json", "--json"]);
    fs::remove_file(fx.root.join("plan.json")).unwrap();
    ok(&ran);
    let id = ran.doc()["tasks"][0]["id"].as_str().unwrap().to_owned();
    let digest = fx.run(&["status", &id, "--json"]).doc()["shown"]["digest"].as_str().unwrap().to_owned();
    ok(&fx.run(&["authorize", &id, "--seen", &digest, "--say", SAY]));
    id
}

#[test]
fn formula_1a_4_a_halt_of_req_run_2_comes_before_formula_unapproved() {
    // A stage of two dispatches: its first task's builder and reviewer spend them, so its second halts at REQ-RUN-2,
    // which the table lists before FORMULA_UNAPPROVED, even with the formula changed since its approval.
    let fx = Fx::builder("formula-run-halt", "published", &format!("{WRITES}{SUCCESS}"));
    fs::write(path(&fx), FORMULA).unwrap();
    ok(&approve(&fx, "formula"));
    let head = fx.git(&["rev-parse", "HEAD"]);
    let stage = json!({"lang": "en", "goal": "Build it.", "budget": {"dispatches": 2, "minutes": 120}, "checks": {}, "settings": {}});
    let task = |goal: &str| {
        json!({
            "lang": "en", "goal": goal, "requirements": [{"text": "It builds.", "criterion": "it builds"}],
            "base": head, "dependencies": [], "allowed": ["src/**"], "profiles": {"build": "build", "review": "review"},
            "checks": [], "limits": {"rounds": 3, "dispatches": 6, "minutes": 120}, "delegationLanguage": "en",
            "settings": {},
        })
    };
    fs::write(
        fx.root.join("plan.json"),
        serde_json::to_vec(&json!({"stage": stage, "tasks": [task("One."), task("Two.")]})).unwrap(),
    )
    .unwrap();
    let ran = fx.run(&["propose", "plan.json", "--json"]);
    fs::remove_file(fx.root.join("plan.json")).unwrap();
    ok(&ran);
    let ids: Vec<String> = (0..2).map(|i| ran.doc()["tasks"][i]["id"].as_str().unwrap().to_owned()).collect();
    let stage = ran.doc()["stage"]["id"].as_str().unwrap().to_owned();
    let digest = fx.run(&["status", &stage, "--json"]).doc()["shown"]["digest"].as_str().unwrap().to_owned();
    ok(&fx.run(&["authorize", &stage, "--seen", &digest, "--say", SAY]));
    ok(&fx.run(&["run", &ids[0], "--say", SAY]));
    assert_eq!(fx.events(&format!("task-{}", ids[0])).pop().unwrap()["kind"], "task-verified");
    let second = &ids[1];
    fs::write(path(&fx), format!("{FORMULA}\nChanged.\n")).unwrap();
    let ran = fx.run(&["run", second, "--say", SAY, "--json"]);
    assert_eq!(ran.status, 4, "{ran:?}");
    let last = fx.events(&format!("task-{second}")).pop().unwrap();
    assert_eq!(
        (last["kind"].as_str(), last["payload"]["code"].as_str()),
        (Some("task-halted"), Some("BUDGET_EXHAUSTED"))
    );
}

#[test]
fn formula_1a_4_an_unapproved_formula_stops_close() {
    let fx = approved("formula-close");
    let stage = json!({"lang": "en", "goal": "Build it.", "budget": {"dispatches": 6, "minutes": 120}, "checks": {}, "settings": {}});
    let task = json!({
        "lang": "en", "goal": "Make it.", "requirements": [{"text": "It builds.", "criterion": "cargo build passes"}],
        "base": "0".repeat(40), "dependencies": [], "allowed": ["src/**"],
        "profiles": {"build": "build", "review": "review"}, "checks": [],
        "limits": {"rounds": 3, "dispatches": 6, "minutes": 120}, "delegationLanguage": "en", "settings": {},
    });
    fs::write(fx.root.join("plan.json"), serde_json::to_vec(&json!({"stage": stage, "tasks": [task]})).unwrap())
        .unwrap();
    let ran = fx.run(&["propose", "plan.json", "--json"]);
    ok(&ran);
    let (stage, task) = (
        ran.doc()["stage"]["id"].as_str().unwrap().to_owned(),
        ran.doc()["tasks"][0]["id"].as_str().unwrap().to_owned(),
    );
    let digest = fx.run(&["status", &stage, "--json"]).doc()["shown"]["digest"].as_str().unwrap().to_owned();
    ok(&fx.run(&["authorize", &stage, "--seen", &digest, "--say", SAY]));
    ok(&fx.run(&["reject", &task, "--say", SAY]));
    let rec = json!({"point": "About it", "clarify": "Say it.", "destination": "the formula"});
    let (text, parts) = fabrewk_core::fermento::open(&fx.domain, "xel").unwrap();
    let rows: Vec<Value> = fabrewk_core::fermento::points(&text[..parts.input_end])
        .iter()
        .map(|p| json!({"number": p.number, "digest": p.digest, "recommendation": rec}))
        .collect();
    let doc =
        json!({"agent": {"tool": "claude-code", "provider": "anthropic", "model": "claude-opus-5-5"}, "points": rows});
    fs::write(fx.root.join("answer.json"), serde_json::to_vec(&doc).unwrap()).unwrap();
    ok(&fx.run(&["sweep", "xel", "--answer", "answer.json"]));
    fs::write(path(&fx), format!("{FORMULA}\nChanged.\n")).unwrap();
    refused(&fx, &["close", &stage, "--say", SAY], "FORMULA_UNAPPROVED", 1, "es");
    ok(&approve(&fx, "formula"));
    ok(&fx.run(&["close", &stage, "--say", SAY]));
}

// ---- Point 5: the state, the cut in progress and what waits ------------------------------------------------------------

fn status_md(fx: &Fx) -> String {
    fs::read_to_string(fx.domain.join("STATUS.md")).unwrap()
}

#[test]
fn formula_1a_5_status_shows_the_formula_the_cut_in_progress_and_the_audit_that_waits() {
    let t = |key: &str, params: &[(&str, &str)]| fabrewk_core::cli::say("en", key, params);
    let fx = Fx::new("formula-status", "published");
    // No formula: STATUS.md says so, and names nothing to read next.
    assert!(status_md(&fx).contains(&format!("## {}\n\n- {}\n", t("view.formula", &[]), t("view.formula-none", &[]))));
    assert!(!status_md(&fx).contains("FORMULA.md next"));
    let doc = fx.run(&["status", "--json"]).doc();
    assert_eq!(doc["formula"], json!({"approved": null, "unapproved": null, "cut": null}), "{doc}");
    refused(&fx, &["cut", "1a"], "FORMULA_UNAPPROVED", 1, "es");
    // Approved: its digest, no cut yet, and FORMULA.md as what an agent reads next.
    fs::write(path(&fx), FORMULA).unwrap();
    ok(&approve(&fx, "formula"));
    let digest = sha256_hex(FORMULA.as_bytes());
    let status = status_md(&fx);
    assert!(status.contains(&format!("- Approved: digest {}, by xel at ", &digest[..12])), "{status}");
    assert!(status.contains(&t("view.formula-no-cut", &[])), "{status}");
    assert!(status.contains(&format!("- {}\n", t("view.formula-next", &[]))), "{status}");
    let doc = fx.run(&["status", "--json"]).doc();
    assert!(!doc["next"].as_array().unwrap().iter().any(|c| c == "fabrewk approve formula"), "{doc}");
    // The cuts are the formula's own: one in a fence is none; a cut not in progress is not reported.
    refused(&fx, &["cut", "9z"], "CUT_UNKNOWN", 1, "es");
    let e = refused(&fx, &["cut", "1a", "--report"], "MOVE_NOT_ALLOWED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("no cut is in progress"), "{e}");
    // The integrator opens 1a, with no words, as an agent.
    let ran = fx.run_env(&["cut", "1a", "--json"], &[("CLAUDECODE", "1")]);
    ok(&ran);
    assert_eq!((ran.doc()["schema"].as_str(), ran.doc()["cut"].as_str()), (Some("fabrewk.cut/1"), Some("1a")));
    let e = fx.events("formula").pop().unwrap();
    assert_eq!(
        (e["kind"].as_str(), &e["actor"], &e["payload"]),
        (
            Some("cut-recorded"),
            &json!({"kind": "fabrewk"}),
            &json!({"cut": "1a", "phase": "Phase 1: the first", "title": "The first cut", "reported": false, "formula": digest})
        )
    );
    assert!(status_md(&fx).contains("- Cut in progress: 1a, The first cut (Phase 1: the first), opened at "));
    let e = refused(&fx, &["cut", "1a"], "MOVE_NOT_ALLOWED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("in progress already"), "{e}");
    let e = refused(&fx, &["cut", "1b", "--report"], "MOVE_NOT_ALLOWED", 1, "es");
    assert!(e["detail"].as_str().unwrap().contains("the cut in progress is 1a"), "{e}");
    // Reported: it waits for the maestro's audit, in the view and in status, with no command to type.
    ok(&fx.run(&["cut", "1a", "--report"]));
    let status = status_md(&fx);
    assert!(status.contains("- Cut 1a, The first cut (Phase 1: the first), reported done at "), "{status}");
    let waits = &status
        [status.find("### Waiting for xel (maestro)").unwrap()..status.find("### Waiting for the integrator").unwrap()];
    assert!(
        waits.contains("- Cut 1a of the formula, The first cut, was reported done: the maestro audits it"),
        "{waits}"
    );
    let doc = fx.run(&["status", "--json"]).doc();
    assert_eq!(doc["formula"]["cut"]["reported"], true);
    assert_eq!(doc["formula"]["approved"], json!(digest));
    assert!(doc["waiting"].as_array().unwrap().iter().any(|w| w["what"] == "cut-audit" && w["id"] == "1a"), "{doc}");
    // Once the maestro accepts it in the chat, the integrator opens the next cut, and nothing waits.
    ok(&fx.run(&["cut", "1b"]));
    let doc = fx.run(&["status", "--json"]).doc();
    assert!(doc["waiting"].as_array().unwrap().iter().all(|w| w["what"] != "cut-audit"), "{doc}");
    ok(&fx.run(&["check"]));
    // A formula changed by hand: status says so and gives the maestro the move; no cut is recorded on it.
    fs::write(path(&fx), format!("{FORMULA}\nChanged.\n")).unwrap();
    let ran = fx.run(&["status"]);
    assert!(ran.stdout.contains(&fabrewk_core::cli::say("es", "status.formula-unapproved", &[])), "{ran:?}");
    assert!(ran.stdout.contains("  fabrewk approve formula\n"), "{ran:?}");
    let doc = fx.run(&["status", "--json"]).doc();
    assert!(doc["formula"]["unapproved"].as_str().unwrap().contains("differs"), "{doc}");
    refused(&fx, &["cut", "1a"], "FORMULA_UNAPPROVED", 1, "es");
}

// ---- The replay holds the formula's stream to its order ----------------------------------------------------------------

#[test]
fn formula_1a_the_replay_holds_the_stream_to_its_order() {
    let maestro = json!({"kind": "person", "alias": "xel", "role": "maestro"});
    let agent = json!({"kind": "agent", "tool": "claude-code", "provider": "anthropic", "modelRequested": "m", "modelObserved": null});
    let fabrewk = json!({"kind": "fabrewk"});
    let ev = |stream: &str, kind: &str, actor: &Value, payload: Value| {
        (stream.to_owned(), kind.to_owned(), actor.clone(), payload)
    };
    let digest = sha256_hex(FORMULA.as_bytes());
    let imported = ev(
        "formula",
        "formula-imported",
        &fabrewk,
        json!({"commit": "a".repeat(40), "digest": digest, "alias": "xel"}),
    );
    // A proposal recorded in xel's fermento for `target` from `base`, whose drafted text gives `result`.
    let recorded = |target: &str, base: &str, unit: Value| {
        let point = json!({"number": 1, "digest": "d".repeat(64)});
        let p = json!({"id": "p-0000000a", "point": point, "target": target, "base": base, "units": [unit], "keys": [], "result": "e".repeat(64)});
        ev("fermento-xel", "proposal-recorded", &agent, p)
    };
    let incorporated =
        ev("formula", "proposal-incorporated", &maestro, json!({"id": "p-0000000a", "result": "e".repeat(64)}));
    let order = "the import comes first, an approval or a proposal starts from the text last approved";
    for (name, events, why) in [
        // An import after an approval.
        (
            "after",
            vec![ev("formula", "formula-approved", &maestro, json!({"old": null, "digest": digest})), imported.clone()],
            order,
        ),
        // An approval that does not start from the digest last approved.
        (
            "old",
            vec![
                imported.clone(),
                ev("formula", "formula-approved", &maestro, json!({"old": "b".repeat(64), "digest": "c".repeat(64)})),
            ],
            order,
        ),
        // A cut of no approved formula.
        (
            "cut",
            vec![ev(
                "formula",
                "cut-recorded",
                &fabrewk,
                json!({"cut": "1a", "phase": "P", "title": "T", "reported": false, "formula": digest}),
            )],
            order,
        ),
        // A proposal for the formula from another text than the one approved, though it gives the result it predicted.
        (
            "stale",
            vec![
                imported.clone(),
                recorded("formula", &"b".repeat(64), json!({"kind": "formula", "text": "x"})),
                incorporated.clone(),
            ],
            order,
        ),
        // A proposal for the alambique, from the formula's digest, incorporated as the formula's.
        (
            "target",
            vec![
                imported.clone(),
                recorded("alambique", &digest, json!({"kind": "point", "n": 1, "m": 1, "text": "x"})),
                incorporated.clone(),
            ],
            order,
        ),
        // Payloads of their shape whose content is not valid: no commit and no alias, a digest that is not one.
        (
            "content",
            vec![ev(
                "formula",
                "formula-imported",
                &fabrewk,
                json!({"commit": "not a commit", "digest": digest, "alias": ""}),
            )],
            "is not valid",
        ),
        (
            "digest",
            vec![ev("formula", "formula-approved", &maestro, json!({"old": null, "digest": "garbage"}))],
            "is not valid",
        ),
        (
            "cut-content",
            vec![
                imported.clone(),
                ev(
                    "formula",
                    "cut-recorded",
                    &fabrewk,
                    json!({"cut": "", "phase": "P", "title": "T", "reported": false, "formula": digest}),
                ),
            ],
            "is not valid",
        ),
    ] {
        let fx = Fx::new(&format!("formula-order-{name}"), "published");
        let policy = sha256_hex(fs::read_to_string(fx.domain.join("settings.toml")).unwrap().as_bytes());
        let mut s = Session::begin(&fx.domain, None, &fabrewk_core::identity::PolicyVerifier).unwrap();
        for (stream, kind, actor, payload) in events {
            let say = (actor["kind"] == "person").then(|| Say { text: "Lo decido.".into(), lang: "es-MX".into() });
            let signer = say.is_some().then(|| fx.signer());
            let d = Draft { kind, actor, policy: policy.clone(), instance: "SHA256:fixture".into(), say, payload };
            let head = s.head(&stream).unwrap();
            s.append(&stream, head.as_ref(), &d, signer.as_ref().map(|k| k as _)).unwrap();
        }
        drop(s);
        let doc = fx.run(&["check", "--json"]).doc();
        let found = doc["findings"].as_array().unwrap().iter().filter(|f| f["code"] == "RECORD_TRANSITION_INVALID");
        let details: Vec<&str> = found.filter_map(|f| f["detail"].as_str()).collect();
        assert!(details.iter().any(|d| d.contains(why)), "{name}: {details:?} {doc}");
    }
}
