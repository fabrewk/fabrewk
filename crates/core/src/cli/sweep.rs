//! `fabrewk sweep <alias> --answer <file>` and `fabrewk discard <proposal> --say <words>` (REQ-FER-3, REQ-FER-4,
//! REQ-FER-9, REQ-FER-10; section 5; part 8a, D-072). A sweep takes the integrator's answer to one maintainer's
//! fermento: it records what it saw, one proposal for each drafted edit and a withdrawal for each pending proposal
//! whose point is gone, and rewrites only the text below the separator, from the record. A discard is a maestro's
//! signed move on a pending proposal; it rewrites nothing: the answer shows it at the next sweep.

use super::adopt::written_lines;
use super::write::{self, Plan, View};
use super::{Cx, Failed, Options, Out};
use crate::code::{Code, Error, refuse};
use crate::fermento::{self, Proposal};
use crate::record::{Draft, Replay, Repo, StreamKind, stream_kind, subject_digest};
use crate::{identity, settings, table};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// What a sweep reports: the points it answered, and the proposals it recorded and withdrew.
struct Swept {
    points: usize,
    recorded: Vec<String>,
    withdrawn: Vec<String>,
}

/// The skills the tools load from outside the project, as this process's home gives them (REQ-FER-10).
fn installed_skills(cx: &Cx) -> Value {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    fermento::skills(&cx.modules, home.as_deref(), &cx.domain)
}

/// Every stage and task stream's head, as a sweep saw them (REQ-TASK-6).
fn heads(record: &Replay) -> Value {
    let streams =
        record.streams.iter().filter(|(s, _)| matches!(stream_kind(s), Some(StreamKind::Stage | StreamKind::Task)));
    let map: BTreeMap<&String, Value> = streams
        .filter_map(|(s, events)| Some((s, events.last().map(|e| json!({"seq": e.seq, "digest": e.digest}))?)))
        .collect();
    json!(map)
}

fn plan(cx: &Cx, alias: &str, answer: &Value, path: &Path, record: &Replay) -> Result<Plan<Swept>, Failed> {
    let st = write::state(cx, record)?;
    if !identity::maintainers(&st.policy.settings).iter().any(|m| m.alias == alias) {
        return Err(
            refuse(Code::MaintainerUnlisted, None, format!("{alias} is no maintainer of the policy in force")).into()
        );
    }
    let (text, parts) = fermento::open(&cx.domain, alias)?;
    let digest = fermento::input_digest(&text, &parts);
    // Where the mode commits, the commit holds the fermento whole: the input in it must be the input committed.
    if let Some(repo) = Repo::for_domain(&cx.domain, st.mode) {
        let file = Path::new(alias).join("fermento.md");
        let held = crate::record::committed(&repo, &file)?;
        // The commit puts the fermento in the person's index: an edit of it staged there is theirs, and is not lost.
        if held != crate::record::staged(&repo, &file)? {
            let why = "a staged edit of the fermento would be lost; commit or unstage it, and the sweep will commit the answer";
            return Err(refuse(Code::InputUncommitted, Some(&cx.domain.join(&file)), why).into());
        }
        let held = held.and_then(|b| String::from_utf8(b).ok());
        let held = held.and_then(|t| Some(fermento::input_digest(&t, &fermento::split(&t, alias).ok()?)));
        if held.as_deref() != Some(digest.as_str()) {
            let why = "the input differs from the committed one; commit it, and the sweep will commit the answer";
            return Err(refuse(Code::InputUncommitted, Some(&cx.domain.join(alias).join("fermento.md")), why).into());
        }
    }
    // The answer names each point of the input once, with the digest of its text as it stands (REQ-FER-3).
    let input = fermento::points(&text[..parts.input_end]);
    let changed = |why: String| refuse(Code::PointChanged, Some(path), why);
    let mut numbers = BTreeSet::new();
    if let Some(p) = input.iter().find(|p| !numbers.insert(p.number)) {
        return Err(changed(format!("point {} stands twice in the fermento", p.number)).into());
    }
    let given = answer["points"].as_array().map(Vec::as_slice).unwrap_or_default();
    for g in given {
        let number = g["number"].as_u64().unwrap_or_default();
        match input.iter().find(|p| u64::from(p.number) == number) {
            None => return Err(changed(format!("point {number} does not exist in the fermento")).into()),
            Some(p) if g["digest"] != p.digest => return Err(changed(format!("point {number} changed")).into()),
            Some(_) => {}
        }
    }
    if let Some(p) = input.iter().find(|p| !given.iter().any(|g| g["number"] == p.number)) {
        return Err(changed(format!("point {} has no recommendation in the answer", p.number)).into());
    }
    let agent = &answer["agent"];
    let actor = json!({
        "kind": "agent", "tool": agent["tool"], "provider": agent["provider"], "modelRequested": agent["model"],
        "modelObserved": null,
    });
    let draft = |kind: &str, actor: &Value, payload: Value| Draft {
        kind: kind.into(),
        actor: actor.clone(),
        policy: st.policy.digest.clone(),
        instance: String::new(),
        say: None,
        payload,
    };
    let swept: Vec<Value> = given
        .iter()
        .map(|g| json!({"number": g["number"], "digest": g["digest"], "recommendation": g["recommendation"]}))
        .collect();
    let stream = format!("fermento-{alias}");
    let mut events = vec![(
        stream.clone(),
        draft(
            "sweep-recorded",
            &actor,
            fermento::sweep_payload(&digest, &json!(swept), heads(record), installed_skills(cx)),
        ),
    )];
    let known = fermento::proposals(record)?;
    let fabrewk = json!({"kind": "fabrewk"});
    let mut out = Swept { points: given.len(), recorded: Vec::new(), withdrawn: Vec::new() };
    // A pending proposal whose point is gone (no point bears its number) is withdrawn.
    for p in known.values().filter(|p| p.alias == alias && p.state == "pending") {
        if !numbers.contains(&u32::try_from(p.payload["point"]["number"].as_u64().unwrap_or_default()).unwrap_or(0)) {
            out.withdrawn.push(p.id.clone());
            events.push((
                stream.clone(),
                draft("proposal-withdrawn", &fabrewk, json!({"id": p.id, "why": "point-gone"})),
            ));
        }
    }
    for g in given {
        let Some(edit) = g.get("edit") else { continue };
        let mut payload = json!({"point": {"number": g["number"], "digest": g["digest"]}});
        for field in ["target", "base", "units", "keys", "result"] {
            payload[field] = edit[field].clone();
        }
        // Two maintainers' identical edits are two proposals: the identifier also names the fermento they stand in.
        let id =
            format!("p-{}", &subject_digest(&json!({"fermento": alias, "proposal": payload})).unwrap_or_default()[..8]);
        payload["id"] = json!(id);
        fermento::check("proposal-recorded", &payload)?;
        // The same edit swept again is the proposal already on record; an identifier two different edits share is not.
        match known.get(&id) {
            Some(Proposal { alias: from, payload: old, .. }) if from == alias && *old == payload => continue,
            Some(_) => {
                return Err(refuse(Code::RecordIdCollision, Some(path), format!("{id} names another proposal")).into());
            }
            None => {}
        }
        out.recorded.push(id);
        events.push((stream.clone(), draft("proposal-recorded", &actor, payload)));
    }
    let lang = write::interaction_lang(cx, &st.policy, alias)?;
    let lang = super::catalogue(&lang);
    let name = alias.to_owned();
    let view = View { alias: name.clone(), digest, answer: Box::new(move |r| fermento::answer(r, &name, lang)) };
    Ok(Plan { events, signer: None, sealed: false, view: Some(view), alambique: None, then: None, out })
}

/// `fabrewk sweep <alias> --answer <file>`.
pub(super) fn sweep(cx: &Cx, alias: &str, options: &Options) -> Result<Out, Failed> {
    if !settings::is_alias(alias) {
        return Err(refuse(Code::AliasInvalid, None, format!("{alias:?} is not an alias")).into());
    }
    let path = cx.root.join(options.answer.as_deref().unwrap_or_default());
    let answer = super::propose::read(&path, Code::AnswerInvalid)?;
    fermento::check_answer(&answer).map_err(|e| match e {
        Error::Refused { code, detail, .. } => refuse(code, Some(&path), detail),
        other => other,
    })?;
    let (swept, written) = write::run(cx, |record| plan(cx, alias, &answer, &path, record))?;
    let count = |n: usize| n.to_string();
    let params = [
        ("alias", alias),
        ("points", &count(swept.points)),
        ("recorded", &count(swept.recorded.len())),
        ("withdrawn", &count(swept.withdrawn.len())),
    ];
    let mut text = cx.t("sweep.done", &params) + "\n";
    let mut doc = json!({
        "schema": "fabrewk.swept/1", "alias": alias, "points": swept.points, "proposals": swept.recorded,
        "withdrawn": swept.withdrawn,
    });
    text += &written_lines(cx, &written, &mut doc);
    text += &cx.next(&["fabrewk status"]);
    Ok(Out::new(doc, text))
}

/// `fabrewk discard <proposal> --say <words>`.
pub(super) fn discard(cx: &Cx, id: &str, options: &Options) -> Result<Out, Failed> {
    write::not_an_agent(cx)?;
    let plan = |record: &Replay| -> Result<Plan<String>, Failed> {
        let policy = write::state(cx, record)?.policy;
        let all = fermento::proposals(record)?;
        let p = all
            .get(id)
            .ok_or_else(|| refuse(Code::ProposalUnknown, None, format!("{id} is no proposal of the record")))?;
        table::step("proposal", Some(p.state), "proposal-discarded")?;
        let (alias, actor, signer) = write::person(cx, &policy, "proposal-discarded")?;
        let say = Some(write::words(cx, &policy, &alias, options)?);
        let draft = Draft {
            kind: "proposal-discarded".into(),
            actor,
            policy: policy.digest.clone(),
            instance: String::new(),
            say,
            payload: json!({"id": p.id}),
        };
        let events = vec![(format!("fermento-{}", p.alias), draft)];
        Ok(Plan {
            events,
            signer: Some(signer),
            sealed: false,
            view: None,
            alambique: None,
            then: None,
            out: p.id.clone(),
        })
    };
    let (id, written) = write::run(cx, plan)?;
    let mut text = cx.t("discard.done", &[("id", &id)]) + "\n";
    let mut doc = json!({"schema": "fabrewk.discarded/1", "id": id});
    text += &written_lines(cx, &written, &mut doc);
    text += &cx.next(&["fabrewk status"]);
    Ok(Out::new(doc, text))
}
