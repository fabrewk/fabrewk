//! `fabrewk approve <proposal|alambique> --say <words>` (REQ-FER-5 to REQ-FER-7; sections 4 and 5; part 8b, D-073). A
//! maestro approves a pending proposal: for the alambique its edit is applied and the alambique committed alone, for
//! the rules the new rules are recorded; or approves the alambique as it stands, which records a baseline when none
//! was (the first text approved) and otherwise a direct edit with its difference.

use super::adopt::written_lines;
use super::write::{self, Edit, Plan};
use super::{Cx, Failed, Options, Out};
use crate::alambique::{self, FILE};
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
                let repo = Repo::for_domain(&cx.domain, st.mode);
                file = alambique::unchanged(&cx.domain, repo.as_ref(), base, true)?;
            }
            ("ALAMBIQUE_UNAPPROVED", "alambique") => alambique::check(record, &cx.domain)?,
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
            before: base.to_owned(),
            bytes: Some(file),
            message: format!("alambique: proposal {}\n", p.id),
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
    let case = if edit.is_some() { "alambique" } else { "rules" };
    let events = vec![(target.to_owned(), draft)];
    let out = Approved { case, id: p.id.clone(), digest };
    Ok(Plan { events, signer: Some(signer), sealed: false, view: None, alambique: edit, then: None, out })
}

fn direct(cx: &Cx, options: &Options, record: &Replay) -> Result<Plan<Approved>, Failed> {
    let st = write::state(cx, record)?;
    let approved = alambique::approved(record);
    let kind = if approved.is_some() { "direct-edit-approved" } else { "alambique-baseline" };
    let (alias, actor, signer) = write::person(cx, &st.policy, kind)?;
    let say = Some(write::words(cx, &st.policy, &alias, options)?);
    let missing = |why: &str| refuse(Code::AlambiqueMissing, Some(&cx.domain.join(FILE)), why.to_owned());
    let bytes = alambique::read(&cx.domain)?.ok_or_else(|| missing("there is no ALAMBIQUE.md"))?;
    let new = String::from_utf8(bytes.clone()).map_err(|_| missing("not valid text"))?;
    let digest = sha256_hex(&bytes);
    if approved.as_deref() == Some(digest.as_str()) {
        return Err(refuse(Code::MoveNotAllowed, None, "the alambique equals its last approved text").into());
    }
    let payload = match &approved {
        None => json!({"digest": digest}),
        Some(old) => {
            // The old text is the record branch's, when it is the one approved; the record holds only its digest.
            let tip = Repo::for_domain(&cx.domain, st.mode).and_then(|r| committed(&r, Path::new(FILE)).ok()?);
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
    let edit = Edit { before: digest.clone(), bytes: None, message: format!("alambique: {kind}\n") };
    let case = if approved.is_some() { "direct" } else { "baseline" };
    let out = Approved { case, id: String::new(), digest };
    Ok(Plan {
        events: vec![("alambique".into(), draft)],
        signer: Some(signer),
        sealed: false,
        view: None,
        alambique: Some(edit),
        then: None,
        out,
    })
}

/// `fabrewk approve <proposal|alambique> --say <words>`.
pub(super) fn approve(cx: &Cx, what: &str, options: &Options) -> Result<Out, Failed> {
    write::not_an_agent(cx)?;
    let plan = |record: &Replay| match what {
        "alambique" => direct(cx, options, record),
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
