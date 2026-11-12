//! `fabrewk adopt --say <words> [--say-lang <tag>] [--widen <key>]…` (REQ-SET-4, REQ-SET-5, REQ-ID-3; section 5; part
//! 7d3a): a maestro adopts the project file as the policy in force. It prints every change against the policy in
//! force, each classed by its effect, and refuses a widening not named with `--widen`, a supervisor, an edit not
//! committed where the mode commits, and a file that does not validate. The event carries the file's content, its
//! digest, the seal of every other stream's head and the widenings named; it cites the policy it replaces.

use super::write::{self, Plan};
use super::{Cx, Failed, Options, Out};
use crate::record::{Draft, Repo};
use crate::settings::{self, Adoption, Effect};
use serde_json::{Value, json};

pub(super) fn run(cx: &Cx, options: &Options) -> Result<Out, Failed> {
    write::not_an_agent(cx)?;
    let plan = |record: &crate::record::Replay| -> Result<Plan<Adoption>, Failed> {
        let st = write::state(cx, record)?;
        let (alias, actor, signer) = write::person(cx, &st.policy, "policy-adopted")?;
        let say = write::words(cx, &st.policy, &alias, options)?;
        let repo = Repo::for_domain(&cx.domain, st.mode);
        let adoption = settings::adopt(&cx.domain, Some(&st.policy), &options.widen, repo.as_ref())?;
        let draft = Draft {
            kind: "policy-adopted".into(),
            actor,
            policy: st.policy.digest.clone(),
            instance: String::new(),
            say: Some(say),
            payload: adoption.payload(&Default::default()),
        };
        Ok(Plan {
            events: vec![("project".into(), draft)],
            signer: Some(signer),
            sealed: true,
            view: None,
            alambique: None,
            then: None,
            out: adoption,
        })
    };
    let (adoption, written) = write::run(cx, plan)?;
    let mut text = cx.t("adopt.done", &[("digest", &adoption.digest[..12])]) + "\n";
    let mut changes = Vec::new();
    if adoption.changes.is_empty() {
        text += &format!("  {}\n", cx.t("adopt.unchanged", &[]));
    }
    for c in &adoption.changes {
        let effect = match c.effect {
            Effect::Widening => "widening",
            Effect::Narrowing => "narrowing",
            Effect::Neutral => "neutral",
        };
        changes.push(json!({"key": c.key, "effect": effect, "place": c.place()}));
        text += &format!("  {}: {}\n", c.place(), cx.t(&format!("effect.{effect}"), &[]));
    }
    let mut doc = json!({
        "schema": "fabrewk.adopted/1", "digest": adoption.digest, "changes": changes, "widened": adoption.widened,
    });
    text += &written_lines(cx, &written, &mut doc);
    text += &cx.next(&["fabrewk config", "fabrewk status"]);
    Ok(Out::new(doc, text))
}

/// The lines and the document's fields every write command reports: its events and its record commit.
pub(super) fn written_lines(cx: &Cx, written: &write::Written, doc: &mut Value) -> String {
    let mut text = String::new();
    let mut events = Vec::new();
    for (stream, head) in &written.events {
        text += &format!("{}\n", cx.t("write.event", &[("seq", &head.seq.to_string()), ("stream", stream)]));
        events.push(json!({"stream": stream, "seq": head.seq, "digest": head.digest}));
    }
    text += &match &written.commit {
        Some(c) => cx.t("write.commit", &[("commit", &c[..12.min(c.len())])]),
        None => cx.t("write.uncommitted", &[]),
    };
    text.push('\n');
    doc["events"] = json!(events);
    doc["commit"] = json!(written.commit);
    text
}
