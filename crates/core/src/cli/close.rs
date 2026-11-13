//! `fabrewk close <stage> --say <words>` (REQ-TASK-6; part 8c, D-074): a maestro's signed move that closes an authorized
//! stage when every guard of section 4's `stage-closed` row holds, in the table's order: no task still open
//! (`STAGE_OPEN_TASKS`), every maintainer's fermento swept after the stage's last task ended (`SWEEP_MISSING`), the
//! alambique as last approved (`ALAMBIQUE_UNAPPROVED`, the function `run` and `check` call) and the record committed
//! where the mode commits (`RECORD_UNCOMMITTED`). The event binds the stage and its head and lists the sweeps it relied
//! on.

use super::adopt::written_lines;
use super::tasks::{head, stream_of};
use super::write::{self, Plan};
use super::{COMMANDS, Cx, Failed, Options, Out, Usage};
use crate::code::{Code, refuse};
use crate::record::{self, Draft, Replay, Repo, StreamKind, stream_kind};
use crate::{alambique, fermento, identity, table, view};
use serde_json::json;

/// `fabrewk close <stage> --say <words>`.
pub(super) fn close(cx: &Cx, id: &str, options: &Options) -> Result<Out, Failed> {
    write::not_an_agent(cx)?;
    let plan = |record: &Replay| -> Result<Plan<String>, Failed> {
        let st = write::state(cx, record)?;
        let stream = stream_of(id);
        let usage = |key| {
            Failed::Usage(Usage { key, word: id.to_owned(), command: COMMANDS.iter().find(|c| c.name == "close") })
        };
        let kind = stream_kind(&stream).filter(|_| record.streams.contains_key(&stream));
        match kind {
            Some(StreamKind::Stage) => {}
            Some(StreamKind::Task) => return Err(usage("usage.task")),
            _ => {
                return Err(Failed::Usage(Usage { key: "usage.id", word: id.to_owned(), command: Some(&COMMANDS[0]) }));
            }
        }
        let states = table::current(record)?;
        let m = table::step("stage", states.get(&stream).copied(), "stage-closed")?;
        let (alias, actor, signer) = write::person(cx, &st.policy, "stage-closed")?;
        let say = Some(write::words(cx, &st.policy, &alias, options)?);
        let tasks = view::tasks_of(record, &stream["stage-".len()..]);
        let names: Vec<&String> = tasks.iter().map(|(n, _)| *n).collect();
        let people: Vec<String> = identity::maintainers(&st.policy.settings).into_iter().map(|m| m.alias).collect();
        let mut sweeps = Vec::new();
        table::evaluate(m, |g| match g.name {
            "STAGE_OPEN_TASKS" => {
                let open = tasks.iter().find(|(n, _)| !matches!(states.get(*n), Some(&("integrated" | "rejected"))));
                match open {
                    Some((n, _)) => Err(refuse(Code::StageOpenTasks, None, format!("{} is {}", &n[5..], states[*n]))),
                    None => Ok(()),
                }
            }
            "SWEEP_MISSING" => {
                for who in &people {
                    if let Some(why) = fermento::unswept(record, who, &names) {
                        return Err(refuse(Code::SweepMissing, None, why));
                    }
                    if let Some(s) = fermento::last_sweep(record, who).filter(|_| !names.is_empty()) {
                        sweeps.push(json!({"alias": who, "head": {"seq": s.seq, "digest": s.digest}}));
                    }
                }
                Ok(())
            }
            "ALAMBIQUE_UNAPPROVED" => alambique::check(record, &cx.domain),
            "RECORD_UNCOMMITTED" => {
                let repo = Repo::for_domain(&cx.domain, st.mode);
                let files = repo.map(|r| record::uncommitted(&r)).transpose()?.unwrap_or_default();
                let why = format!("{} event or signature files not in the record branch's tip", files.len());
                files.first().map_or(Ok(()), |f| Err(refuse(Code::RecordUncommitted, Some(f), why)))
            }
            _ => Ok(()),
        })?;
        let payload = json!({"stage": &stream[6..], "head": head(&record.streams[&stream]), "sweeps": sweeps});
        let draft = Draft {
            kind: "stage-closed".into(),
            actor,
            policy: st.policy.digest.clone(),
            instance: String::new(),
            say,
            payload,
        };
        let out = stream[6..].to_owned();
        Ok(Plan {
            events: vec![(stream, draft)],
            signer: Some(signer),
            sealed: false,
            view: None,
            alambique: None,
            then: None,
            out,
        })
    };
    let (stage, written) = write::run(cx, plan)?;
    let mut text = cx.t("close.done", &[("id", &stage)]) + "\n";
    let mut doc = json!({"schema": "fabrewk.closed/2", "id": stage});
    text += &written_lines(cx, &written, &mut doc);
    text += &cx.next(&["fabrewk status"]);
    Ok(Out::new(doc, text))
}
