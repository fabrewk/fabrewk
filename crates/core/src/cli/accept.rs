//! `fabrewk accept <task> --seen <digest> --say <words>` and the integration it makes (part 9g; REQ-GATE-7, REQ-GATE-8,
//! REQ-GATE-10, REQ-REG-10; D-086, D-087): what an acceptance binds, which `status <task>` shows with its digest; the
//! integration's preconditions (evidence current, the third guard and Git's locks, the base unmoved or the branch
//! already at the candidate, the working tree clean at the touched paths) checked before the signature, where they
//! refuse, again just before the move and again with the branch at the candidate, where they record `merge-failed`;
//! the move by compare-and-swap, the working tree and the index brought to the candidate from the objects' bytes, and
//! `task-integrated`. `run` on an accepted task finishes its integration under the same checks, and `reject` says where
//! the record branch is.

use super::tasks::{self, Bound, verbatim};
use super::write::{self, Appender, Plan, Then};
use super::{Cx, Failed, Options, Out, say};
use crate::candidate::{self, Change, Lanes};
use crate::code::{Code, Error, refuse};
use crate::merge::{self, product_tree};
use crate::record::{Draft, Event, Replay, Repo, fault, git, sha256_hex};
use crate::settings::{Effective, Policy};
use crate::{gate, table};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// A task's last candidate and where the record branch is: its commit, and whether the branch's product is the
/// candidate's (REQ-GATE-8); `None` with no candidate. A Git that cannot say is its failure, never a place (P9G-10).
pub(super) fn located(cx: &Cx, events: &[Event]) -> Result<Option<(String, bool)>, Error> {
    let Some(c) = events.iter().rfind(|e| e.kind == "candidate-recorded") else { return Ok(None) };
    let commit = c.value["payload"]["commit"].as_str().unwrap_or_default().to_owned();
    let product = super::propose::product(cx);
    let head = git::head_of(&product, &git::record_branch(&product)?)?;
    let at = match head {
        Some(h) => product_tree(&product, &h)? == product_tree(&product, &commit)?,
        None => false,
    };
    Ok(Some((commit, at)))
}

/// The last verification of a task's candidate whose tree is `tree`, and its review, as the record holds them.
fn verified<'e>(events: &'e [Event], tree: &str) -> Option<(&'e Value, &'e Value)> {
    let mut verifications = events.iter().filter(|e| e.kind == "task-verified");
    let v = &verifications.rfind(|e| e.value["payload"]["tree"] == tree)?.value["payload"];
    let review = events.get(usize::try_from(v["review"].as_u64()?).ok()?.checked_sub(1)?)?;
    Some((v, &review.value["payload"]))
}

/// What an acceptance of `stream`'s task would bind (REQ-GATE-7, PD-37, D-086 point 2), when the table allows one from
/// its state and, but from `verified`, its record branch is at its candidate: the task as an authorization shows it,
/// then the candidate, its files, its checks with their identities and its review, in the person's language; and the
/// digest of that rendering, which `accept` takes with `--seen`.
pub(super) fn acceptance<'a>(cx: &Cx, record: &'a Replay, stream: &str) -> Result<Option<Bound<'a>>, Error> {
    let Some((name, events)) = record.streams.get_key_value(stream) else { return Ok(None) };
    let state = table::current(record).ok().and_then(|s| s.get(stream).copied());
    if table::step("task", state, "task-accepted").is_err() {
        return Ok(None);
    }
    // Where the branch is, read from a Git that may fail, which is said, never taken for a place (P9G-13).
    let Some((_, at)) = located(cx, events)? else { return Ok(None) };
    if state != Some("verified") && !at {
        return Ok(None);
    }
    let Some(c) = events.iter().rfind(|e| e.kind == "candidate-recorded").map(|e| &e.value["payload"]) else {
        return Ok(None);
    };
    let lang = cx.lang_in(record);
    let t = |k: &str, p: &[(&str, &str)]| say(lang, &format!("accept.{k}"), p);
    let s = |v: &Value| v.as_str().unwrap_or_default().to_owned();
    let mut text = format!("{}\n", t("title", &[]));
    text += &tasks::lines(lang, (name, events), "");
    text += &format!("  {}\n", t("candidate", &[("commit", &s(&c["commit"])), ("tree", &s(&c["tree"]))]));
    text += &format!("  {}\n", t("files", &[]));
    for f in c["changes"].as_array().into_iter().flatten() {
        let how = match (f["before"].is_null(), f["after"].is_null()) {
            (true, _) => "added",
            (_, true) => "removed",
            _ => "changed",
        };
        text += &format!("    {}\n", t(how, &[("path", &verbatim(&s(&f["path"]), "      "))]));
    }
    match verified(events, &s(&c["tree"])) {
        Some((v, r)) => {
            let checks = v["checks"].as_object().into_iter().flatten();
            let checks: Vec<String> = checks.map(|(k, i)| format!("{} = {}", verbatim(k, "    "), s(i))).collect();
            let checks = if checks.is_empty() { say(lang, "bind.none", &[]) } else { checks.join(", ") };
            text += &format!("  {}\n", t("checks", &[("checks", &checks)]));
            let (seq, examined) = (v["review"].to_string(), r["examined"].to_string());
            let found = r["findings"].as_array().map_or(0, Vec::len).to_string();
            let params =
                [("seq", seq.as_str()), ("verdict", &s(&r["verdict"])), ("examined", &examined), ("findings", &found)];
            text += &format!("  {}\n", t("review", &params));
        }
        None => text += &format!("  {}\n", t("unverified", &[])),
    }
    Ok(Some(Bound { digest: sha256_hex(text.as_bytes()), text, tasks: vec![(name, events)] }))
}

/// A task's integration, owned, as a write command plans it from a record: where, the task, its stage and its events,
/// the settings in force, the candidate, its tree and its base (its parent, REQ-RUN-4), the record branch and the
/// allowed files.
pub(super) struct Merge {
    product: Repo,
    domain: PathBuf,
    home: PathBuf,
    stream: String,
    task: Value,
    stage: Value,
    events: Vec<Event>,
    eff: Effective,
    policy: String,
    platform: String,
    candidate: String,
    tree: String,
    base: String,
    branch: String,
    allowed: Vec<String>,
}

/// The integration of `stream`'s task from `record`; `MOVE_NOT_ALLOWED` when it has no candidate.
pub(super) fn plan_merge(cx: &Cx, record: &Replay, policy: &Policy, stream: &str) -> Result<Merge, Failed> {
    let events = &record.streams[stream];
    let c = events.iter().rfind(|e| e.kind == "candidate-recorded").map(|e| &e.value["payload"]);
    let c = c.ok_or_else(|| refuse(Code::MoveNotAllowed, None, "task-accepted: the task has no candidate"))?;
    let product = super::propose::product(cx);
    let candidate = c["commit"].as_str().unwrap_or_default().to_owned();
    let base = git::head_of(&product, &format!("{candidate}^1"))?;
    let base = base.ok_or_else(|| refuse(Code::ToolFailed, None, "the candidate has no parent"))?;
    let p = &events[0].value["payload"];
    let stage = &record.streams[&format!("stage-{}", p["stage"].as_str().unwrap_or_default())][0].value["payload"];
    Ok(Merge {
        branch: git::record_branch(&product)?,
        product,
        domain: cx.domain.clone(),
        home: super::run::home(),
        stream: stream.to_owned(),
        task: p.clone(),
        stage: stage.clone(),
        events: events.clone(),
        eff: super::run::effective(cx, record, policy, stream, None)?.0,
        policy: policy.digest.clone(),
        platform: gate::platform(cx.backend.map_or("none", |b| b.name)),
        tree: c["tree"].as_str().unwrap_or_default().to_owned(),
        candidate,
        base,
        allowed: p["allowed"].as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_owned).collect(),
    })
}

impl Merge {
    /// REQ-GATE-3, REQ-GATE-10: the task's verification, every identity recomputed now on the candidate, while it holds
    /// (`EVIDENCE_STALE` otherwise, a check whose identity cannot be recomputed included); gives its payload.
    fn evidence(&self) -> Result<Value, Error> {
        let stale = |why: String| refuse(Code::EvidenceStale, None, why);
        let checks = gate::required(&self.eff, &self.task, &self.stage).map_err(|e| stale(e.detail()))?;
        let product = sha256_hex(&product_tree(&self.product, &self.candidate)?);
        let now: BTreeMap<String, String> = checks
            .iter()
            .map(|c| (c.name.clone(), gate::identity(&product, c, &self.policy, &self.platform)))
            .collect();
        let requirements = gate::requirements(&self.task);
        let held = gate::verification(&self.events, &self.tree, &requirements, &now, &self.policy);
        let why = "its verification no longer holds: a check's identity, the required checks or the policy changed";
        held.cloned().ok_or_else(|| stale(why.to_owned()))
    }

    /// Where the record branch is (REQ-GATE-8): its head, and whether its product is the candidate's; `BASE_MOVED`
    /// when it is neither the candidate's nor the base's.
    fn place(&self) -> Result<(String, bool), Error> {
        let moved = |why: &str| refuse(Code::BaseMoved, None, why.to_owned());
        let (head, now, at) = self.at()?;
        let head = head.ok_or_else(|| moved("the record branch has no commit"))?;
        if !at && now != product_tree(&self.product, &self.base)? {
            return Err(moved("the product at the record branch's head is neither the task's base nor its candidate"));
        }
        Ok((head, at))
    }

    /// The record branch's head, if any, its product tree, and whether that is the candidate's.
    fn at(&self) -> Result<(Option<String>, Vec<u8>, bool), Error> {
        let Some(head) = git::head_of(&self.product, &self.branch)? else { return Ok((None, Vec::new(), false)) };
        let now = product_tree(&self.product, &head)?;
        let at = now == product_tree(&self.product, &self.candidate)?;
        Ok((Some(head), now, at))
    }

    fn scratch(&self) -> PathBuf {
        self.domain.join("local/tmp").join(format!("merge-{}", &self.stream["task-".len()..]))
    }

    /// Whether the index and the working tree hold, at every touched path, the base's content when `base` admits it, or
    /// the candidate's when `candidate` does (`WORKTREE_DIRTY`).
    fn clean(&self, changes: &[Change], base: bool, candidate: bool) -> Result<(), Error> {
        let held = merge::held(&self.product, changes, &self.scratch());
        let _ = std::fs::remove_dir_all(self.scratch());
        merge::clean(changes, &held?, base, candidate)
    }

    /// The integration's preconditions in `task-integrated`'s order (section 4): REQ-GATE-8's Git locks and third guard,
    /// the base unmoved or the branch at the candidate, the touched paths clean, the evidence current. Gives the change
    /// set, the branch's head and whether it is at the candidate.
    fn preconditions(&self) -> Result<(Vec<Change>, String, bool), Error> {
        let m = table::find("task", "task-integrated").expect("the table holds task-integrated");
        let (mut changes, mut place) = (Vec::new(), (String::new(), false));
        table::evaluate(m, |g| -> Result<(), Error> {
            match g.name {
                "REQ-GATE-8" => {
                    git::check_git_locks(&self.product, &self.branch)?;
                    let (names, paths) =
                        (candidate::protected_names(&self.eff), candidate::protected_paths(&self.eff, &self.product)?);
                    let allowed: Vec<&str> = self.allowed.iter().map(String::as_str).collect();
                    let lanes = Lanes { allowed: &allowed, names: &names, paths: &paths };
                    changes = merge::changes(&self.product, &self.base, &self.candidate, &lanes)?;
                }
                "BASE_MOVED" => place = self.place()?,
                "WORKTREE_DIRTY" => self.clean(&changes, true, place.1)?,
                "EVIDENCE_STALE" => _ = self.evidence()?,
                _ => {}
            }
            Ok(())
        })?;
        Ok((changes, place.0, place.1))
    }

    /// Records `merge-failed` for `e` (REQ-GATE-8): its code, one of an integration's, else `TOOL_FAILED`; the core's
    /// words, with no path of this machine; whether the branch is at the candidate; and what a failed Git left.
    fn failed(&self, a: &mut Appender, out: &mut Value, e: &Error) -> Result<(), Error> {
        // The evidence the failure's own words name first, before anything else runs Git (P9G-8, D-089).
        let kept = git::kept_in(&self.domain, &e.detail());
        let code = e.code().map(|c| c.to_string()).filter(|c| crate::task::MERGE_FAILURES.contains(&c.as_str()));
        let code = code.unwrap_or_else(|| Code::ToolFailed.to_string());
        let mut words = super::run::words(e);
        // Whether the branch is at the candidate now, whoever moved it (P9G-2, P9G-6); unknown when it cannot be read.
        let moved = match self.at() {
            Ok((_, _, at)) => json!(at),
            Err(place) => {
                words += &format!("; where the record branch is could not be read: {}", super::run::words(&place));
                Value::Null
            }
        };
        let reason = super::run::unplaced(words, &self.domain, &self.home);
        let evidence = kept.map(|(stderr, status)| json!({"status": status, "stderr": stderr}));
        let payload = json!({"code": code, "reason": reason, "moved": moved, "evidence": evidence});
        a.fabrewk(&self.stream, "merge-failed", payload)?;
        out["failed"] = json!({"code": code, "reason": reason, "moved": moved});
        Ok(())
    }
}

/// The integration under the lock (REQ-GATE-8): the preconditions again just before the move; the move, to the
/// candidate when the head is the base itself, else to an integration commit on the head, none when the branch is at
/// the candidate already; the working tree and the index brought to it; the preconditions once more with the branch at
/// the candidate; then `task-integrated`. A failure records `merge-failed`, saying whether the branch moved, and
/// nothing is moved back.
fn integrate(m: &Merge, a: &mut Appender, out: &mut Value) -> Result<(), Error> {
    fault::pause("before-move");
    let (changes, head, at) = match m.preconditions() {
        Ok(found) => found,
        Err(e) => return m.failed(a, out, &e),
    };
    let mut from = None;
    if !at {
        let index = m.scratch().join("index");
        let moved = (|| {
            std::fs::create_dir_all(m.scratch()).map_err(crate::code::io_at(&m.scratch()))?;
            let new = match head == m.base {
                true => m.candidate.clone(),
                false => merge::commit(&m.product, &head, &m.candidate, &changes, &index)?,
            };
            fault::point();
            fault::pause("before-update-ref");
            merge::move_branch(&m.product, &m.branch, &head, &new).map(|()| new)
        })();
        match moved {
            Ok(new) => a.moved(&m.product.root, &new),
            Err(e) => return m.failed(a, out, &e),
        }
        from = Some(head);
    }
    fault::point();
    fault::pause("after-move");
    let synced = (|| {
        let held = merge::held(&m.product, &changes, &m.scratch())?;
        merge::clean(&changes, &held, true, true)?;
        merge::bring(&m.product, &changes, &held, &m.scratch())?;
        let (to, at) = m.place()?;
        if !at {
            return Err(refuse(Code::BaseMoved, None, "the record branch is no longer at the candidate"));
        }
        m.clean(&changes, false, true)?;
        m.evidence().map(|_| to)
    })();
    let _ = std::fs::remove_dir_all(m.scratch());
    match synced {
        Ok(to) => {
            a.fabrewk(&m.stream, "task-integrated", json!({"from": from, "to": to}))?;
            out["integrated"] = json!(to);
            Ok(())
        }
        Err(e) => m.failed(a, out, &e),
    }
}

fn then(merge: Merge) -> Then {
    Box::new(move |a, prepared| {
        for (s, p) in prepared {
            a.write(s, p)?;
        }
        let id = &merge.stream["task-".len()..];
        let mut out = json!({"task": id, "integration": true, "integrated": null, "failed": null});
        integrate(&merge, a, &mut out)?;
        Ok(out)
    })
}

/// `accept`'s plan: the move (from `verified`, or from a waiting state with the record branch at the candidate), the
/// person and their words, then `task-accepted`'s guards (REQ-GATE-7's `--seen`, `EVIDENCE_STALE`,
/// `ALAMBIQUE_UNAPPROVED`, `FORMULA_UNAPPROVED`) and the integration's preconditions, each refusing before the signature (D-087 point 1).
fn plan(cx: &Cx, id: &str, options: &Options, record: &Replay) -> Result<Plan<String>, Failed> {
    let policy = write::state(cx, record)?.policy;
    let (_, stream) = tasks::resolve(record, id, false, "accept")?;
    let state = table::current(record)?.get(&stream).copied();
    let m = table::step("task", state, "task-accepted")?;
    let events = &record.streams[&stream];
    let merge = plan_merge(cx, record, &policy, &stream)?;
    if state != Some("verified") && !merge.at()?.2 {
        let why = format!("task-accepted from {}: the record branch is not at its candidate", state.unwrap_or("none"));
        return Err(refuse(Code::MoveNotAllowed, None, why).into());
    }
    let (alias, actor, signer) = write::person(cx, &policy, "task-accepted")?;
    let say = write::words(cx, &policy, &alias, options)?;
    let (mut shown, mut evidence) = (String::new(), Value::Null);
    table::evaluate(m, |g| -> Result<(), Failed> {
        match g.name {
            "REQ-GATE-7" => {
                shown = acceptance(cx, record, &stream)?.map(|b| b.digest).unwrap_or_default();
                if options.seen.as_deref() != Some(shown.as_str()) {
                    let why = format!("what status {id} shows now is not what --seen names; read it again");
                    return Err(refuse(Code::TaskChanged, None, why).into());
                }
            }
            "EVIDENCE_STALE" => evidence = merge.evidence()?,
            "ALAMBIQUE_UNAPPROVED" => crate::alambique::check(record, &cx.domain, &crate::alambique::ALAMBIQUE)?,
            "FORMULA_UNAPPROVED" => crate::alambique::check(record, &cx.domain, &crate::alambique::FORMULA)?,
            _ => {}
        }
        Ok(())
    })?;
    merge.preconditions()?;
    let payload = json!({
        "task": &stream["task-".len()..], "head": tasks::head(events), "candidate": merge.candidate, "tree": merge.tree,
        "requirements": gate::requirements(&merge.task), "checks": evidence["checks"], "review": evidence["review"],
        "shown": shown,
    });
    let draft = Draft {
        kind: "task-accepted".into(),
        actor,
        policy: policy.digest,
        instance: String::new(),
        say: Some(say),
        payload,
    };
    let out = stream["task-".len()..].to_owned();
    Ok(Plan {
        events: vec![(stream, draft)],
        signer: Some(signer),
        sealed: false,
        view: None,
        alambique: None,
        then: Some(then(merge)),
        out,
    })
}

/// `run <task> --say` on an accepted task (D-087 point 2): the person and their words, checked and not recorded, then
/// the integration under the lock, which writes only Fabrewk's events.
pub(super) fn finish(
    cx: &Cx,
    record: &Replay,
    policy: &Policy,
    stream: &str,
    options: &Options,
) -> Result<Plan<()>, Failed> {
    let (alias, _, _) = write::person(cx, policy, "run-started")?;
    write::words(cx, policy, &alias, options)?;
    let merge = plan_merge(cx, record, policy, stream)?;
    Ok(Plan {
        events: Vec::new(),
        signer: None,
        sealed: false,
        view: None,
        alambique: None,
        then: Some(then(merge)),
        out: (),
    })
}

/// What `accept`, or `run` on an accepted task, reports: the task, then its integration, or the failure it waits on
/// (exit 4, REQ-CLI-3).
pub(super) fn report(cx: &Cx, accepted: bool, written: &write::Written) -> Out {
    let ran = &written.ran;
    let id = ran["task"].as_str().unwrap_or_default();
    let mut text = if accepted { cx.t("accept.done", &[("id", id)]) + "\n" } else { String::new() };
    let mut exit = super::DONE;
    if let Some(to) = ran["integrated"].as_str() {
        text += &(cx.t("accept.integrated", &[("id", id), ("commit", to)]) + "\n");
    }
    if let Some(f) = ran["failed"].as_object() {
        let s = |k: &str| f[k].as_str().unwrap_or_default().to_owned();
        text += &(cx.t("accept.failed", &[("id", id), ("code", &s("code"))]) + "\n");
        text += &format!("  {}: {}\n", cx.t("ui.detail", &[]), verbatim(&s("reason"), "    "));
        let place = match &f["moved"] {
            Value::Bool(true) => "accept.moved",
            Value::Bool(false) => "accept.unmoved",
            _ => "accept.unknown",
        };
        text += &format!("  {}\n", cx.t(place, &[]));
        exit = super::WAITING;
    }
    let mut doc =
        json!({"schema": "fabrewk.accepted/1", "id": id, "integrated": ran["integrated"], "failed": ran["failed"]});
    text += &super::adopt::written_lines(cx, written, &mut doc);
    text += &cx.next(&[&format!("fabrewk status {id}")]);
    Out { exit, ..Out::new(doc, text) }
}

/// `fabrewk accept <task> --seen <digest> --say <words>`.
pub(super) fn accept(cx: &Cx, id: &str, options: &Options) -> Result<Out, Failed> {
    write::not_an_agent(cx)?;
    let (_, written) = write::run(cx, |record| plan(cx, id, options, record))?;
    Ok(report(cx, true, &written))
}
