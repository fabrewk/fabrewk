//! `fabrewk approve <proposal|alambique|formula> --say <words>` (REQ-FER-5 to REQ-FER-7; sections 4 and 5; part 8b,
//! D-073; formula 1a point 2). A maestro approves a pending proposal: for the alambique or the formula its edit is
//! applied and the file committed alone, for the rules the new rules are recorded; or approves the alambique as it
//! stands, which records a baseline when none was (the first text approved) and otherwise a direct edit with its
//! difference; or approves the formula as it stands, which records its digest and the one it replaces.

use super::adopt::written_lines;
use super::write::{self, Edit, Plan};
use super::{Cx, Failed, Options, Out};
use crate::alambique::{self, ALAMBIQUE, Doc, FORMULA};
use crate::code::{Code, Error, refuse};
use crate::record::{Draft, Replay, Repo, committed, sha256_hex};
use crate::{fermento, table};
use serde_json::{Value, json};
use std::path::Path;

/// What an approval decided: the catalogue's name of the case, the proposal's identifier if there is one, the digest.
struct Approved {
    case: &'static str,
    id: String,
    digest: String,
}

fn stale(why: impl Into<String>) -> Error {
    refuse(Code::ProposalStale, None, why)
}

fn proposal(cx: &Cx, id: &str, options: &Options, record: &Replay) -> Result<Plan<Approved>, Failed> {
    let st = write::state(cx, record)?;
    let all = fermento::proposals(record)?;
    let p =
        all.get(id).ok_or_else(|| refuse(Code::ProposalUnknown, None, format!("{id} is no proposal of the record")))?;
    let m = table::step("proposal", Some(p.state), "proposal-incorporated")?;
    let (alias, actor, signer) = write::person(cx, &st.policy, "proposal-incorporated")?;
    let say = Some(write::words(cx, &st.policy, &alias, options)?);
    let target = p.payload["target"].as_str().unwrap_or_default();
    let (base, units) = (p.payload["base"].as_str().unwrap_or_default(), &p.payload["units"]);
    let units = units.as_array().map(Vec::as_slice).unwrap_or_default();
    let held = alambique::rules(record);
    let (mut file, mut rules) = (Vec::new(), None);
    let repo = Repo::for_domain(&cx.domain, st.mode);
    table::evaluate(m, |g| -> Result<(), Failed> {
        match (g.name, target) {
            ("PROPOSAL_STALE", _) => {
                // The point as the fermento stands now: the same words under another number do not count.
                let (text, parts) = fermento::open(&cx.domain, &p.alias)?;
                let point = &p.payload["point"];
                let same = |q: &fermento::Point| point["number"] == q.number && point["digest"] == q.digest;
                if !fermento::points(&text[..parts.input_end]).iter().any(same) {
                    return Err(stale(format!("point {} of the fermento is gone or changed", point["number"])).into());
                }
                if target == "rules" && base != alambique::rules_digest(&held) {
                    return Err(stale("another proposal changed the project rules since").into());
                }
            }
            ("ALAMBIQUE_CHANGED", "alambique") => {
                file = alambique::unchanged(&cx.domain, repo.as_ref(), base, true, &ALAMBIQUE)?;
            }
            ("FORMULA_CHANGED", "formula") => {
                alambique::unchanged(&cx.domain, repo.as_ref(), base, true, &FORMULA)?;
            }
            ("ALAMBIQUE_UNAPPROVED", "alambique") => alambique::check(record, &cx.domain, &ALAMBIQUE)?,
            ("FORMULA_UNAPPROVED", "formula") => alambique::check(record, &cx.domain, &FORMULA)?,
            // The formula's one unit is its whole text.
            ("UNIT_AMBIGUOUS", "formula") => {
                file = units.first().and_then(|u| u["text"].as_str()).unwrap_or_default().into();
            }
            ("UNIT_AMBIGUOUS", "alambique") => {
                let text = String::from_utf8(std::mem::take(&mut file));
                let text = text.map_err(|_| refuse(Code::AlambiqueMissing, None, "not valid text"))?;
                file = alambique::apply(&text, units)?.into_bytes();
            }
            ("UNIT_AMBIGUOUS", _) => rules = Some(alambique::apply_rules(&held, units)?),
            _ => {}
        }
        Ok(())
    })?;
    let digest = rules.as_ref().map_or_else(|| sha256_hex(&file), alambique::rules_digest);
    if p.payload["result"] != digest.as_str() {
        return Err(stale("the drafted text does not give the result it predicted").into());
    }
    let mut payload = json!({"id": p.id, "result": digest});
    let edit = match &rules {
        Some(rules) => {
            payload["rules"] = json!(rules);
            None
        }
        None => Some(Edit {
            doc: if target == "formula" { &FORMULA } else { &ALAMBIQUE },
            before: base.to_owned(),
            bytes: Some(file),
            message: format!("{target}: proposal {}\n", p.id),
        }),
    };
    let draft = Draft {
        kind: "proposal-incorporated".into(),
        actor,
        policy: st.policy.digest.clone(),
        instance: String::new(),
        say,
        payload,
    };
    let case = match target {
        "formula" => "formula",
        _ if edit.is_some() => "alambique",
        _ => "rules",
    };
    let events = vec![(target.to_owned(), draft)];
    let out = Approved { case, id: p.id.clone(), digest };
    Ok(Plan { events, signer: Some(signer), sealed: false, view: None, alambique: edit, then: None, out })
}

/// The alambique's or the formula's direct approval: the file as it stands.
fn direct(cx: &Cx, options: &Options, record: &Replay, doc: &'static Doc) -> Result<Plan<Approved>, Failed> {
    let st = write::state(cx, record)?;
    let approved = alambique::approved(record, doc);
    let kind = match (doc.stream, &approved) {
        ("formula", _) => "formula-approved",
        (_, Some(_)) => "direct-edit-approved",
        _ => "alambique-baseline",
    };
    let (alias, actor, signer) = write::person(cx, &st.policy, kind)?;
    let say = Some(write::words(cx, &st.policy, &alias, options)?);
    let missing = |why: &str| refuse(doc.missing, Some(&cx.domain.join(doc.file)), why.to_owned());
    let bytes = alambique::read(&cx.domain, doc)?.ok_or_else(|| missing(&format!("there is no {}", doc.file)))?;
    let new = String::from_utf8(bytes.clone()).map_err(|_| missing("not valid text"))?;
    let digest = sha256_hex(&bytes);
    if approved.as_deref() == Some(digest.as_str()) {
        return Err(refuse(Code::MoveNotAllowed, None, format!("{} equals its last approved text", doc.file)).into());
    }
    let payload = match &approved {
        _ if doc.stream == "formula" => json!({"old": approved, "digest": digest}),
        None => json!({"digest": digest}),
        Some(old) => {
            // The old text is the record branch's, when it is the one approved; the record holds only its digest.
            let tip = Repo::for_domain(&cx.domain, st.mode).and_then(|r| committed(&r, Path::new(doc.file)).ok()?);
            let old_text = tip.filter(|b| sha256_hex(b) == *old).and_then(|b| String::from_utf8(b).ok());
            let difference = match old_text {
                Some(old) => alambique::difference(&old, &new),
                None => json!({"from": 1, "removed": null, "added": alambique::difference("", &new)["added"]}),
            };
            json!({"old": old, "new": digest, "difference": difference})
        }
    };
    let draft =
        Draft { kind: kind.into(), actor, policy: st.policy.digest.clone(), instance: String::new(), say, payload };
    let edit = Edit { doc, before: digest.clone(), bytes: None, message: format!("{}: {kind}\n", doc.stream) };
    let case = match (doc.stream, &approved) {
        ("formula", _) => "formula-direct",
        (_, Some(_)) => "direct",
        _ => "baseline",
    };
    let out = Approved { case, id: String::new(), digest };
    Ok(Plan {
        events: vec![(doc.stream.into(), draft)],
        signer: Some(signer),
        sealed: false,
        view: None,
        alambique: Some(edit),
        then: None,
        out,
    })
}

/// `fabrewk approve <proposal|alambique|formula> --say <words>`.
pub(super) fn approve(cx: &Cx, what: &str, options: &Options) -> Result<Out, Failed> {
    write::not_an_agent(cx)?;
    let plan = |record: &Replay| match what {
        "alambique" => direct(cx, options, record, &ALAMBIQUE),
        "formula" => direct(cx, options, record, &FORMULA),
        id => proposal(cx, id, options, record),
    };
    let (a, written) = write::run(cx, plan)?;
    let mut text = cx.t(&format!("approve.{}", a.case), &[("id", &a.id), ("digest", &a.digest)]) + "\n";
    let id: Value = if a.id.is_empty() { Value::Null } else { json!(a.id) };
    let mut doc = json!({"schema": "fabrewk.approved/2", "case": a.case, "id": id, "digest": a.digest});
    text += &written_lines(cx, &written, &mut doc);
    text += &cx.next(&["fabrewk check"]);
    Ok(Out::new(doc, text))
}
