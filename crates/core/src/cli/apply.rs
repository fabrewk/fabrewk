//! `fabrewk apply <plan> <assess's options> --say <words> [--say-lang <tag>] [--json]` (REQ-INS-3; D-097 points 6 and
//! 7; part 10b): a maestro applies the plan `assess` printed. It refuses inside an agent, reassesses with the same
//! options and refuses another digest (`PLAN_CHANGED`), a verdict `none` (`VERDICT_NONE`) or `review`
//! (`MODE_NOT_AVAILABLE`), a key Git does not sign with that is not the plan's (`ROLE_FORBIDDEN`) and words that are
//! not the person's. Then, under the domain's lock, the installation's entry to the write path (P10O-4): `assessed` and
//! `publication-decided`; the plan's files, the entry blocks, the domain's own repository and the exclude line;
//! `policy-adopted`, `alambique-baseline` and `installed` with the receipt; the views and the record commit. An `apply`
//! stopped before its end is finished by the same command, from the plan its `assessed` holds, writing only what is
//! missing; no file it did not write is ever overwritten.

use super::write::{self, Written};
use super::{Command, Cx, Failed, Options, Out};
use crate::code::{Code, Error, refuse};
use crate::identity::PolicyVerifier;
use crate::record::{self, Draft, Head, Prepared, Publication, Replay, Repo, Session, sha256_hex};
use crate::settings::{self, Level, Policy};
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};

fn changed(why: &str) -> Error {
    refuse(Code::PlanChanged, None, why.to_owned())
}

pub(super) fn run(cx: &Cx, command: &'static Command, digest: &str, options: &Options) -> Result<Out, Failed> {
    write::not_an_agent(cx)?;
    super::answers(cx, command, options)?;
    let record = recorded(cx)?;
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let asked = crate::install::Asked { root: &cx.root, home: home.as_deref(), options, adapters: cx.adapters };
    let plan = match record.as_ref().and_then(|r| Some((r, held(r)?))) {
        // An `apply` of this plan began: it is finished from the plan it recorded, never reassessed, and only with the
        // options that plan was made from (point 7, D-097 decision 3; P10B-7).
        Some((_, (d, plan))) if d == digest && plan["answers"] != (cx.installer.answers)(&asked) => {
            return Err(changed("an option differs from those the recorded plan was made from").into());
        }
        Some((r, (d, plan))) if d == digest => match write::pending(&cx.domain, r)? {
            None => return Ok(out(cx, digest, None)),
            Some(_) => plan.clone(),
        },
        _ => {
            let (doc, _) = (cx.installer.assess)(&asked)?;
            if doc["digest"] != digest {
                let now = doc["digest"].as_str().unwrap_or_default();
                return Err(changed(&format!("the plan assess gives now, with these options, is {now}")).into());
            }
            match doc["plan"]["verdict"].as_str() {
                Some("none") => return Err(refuse(Code::VerdictNone, None, "a reason of the plan blocks it").into()),
                Some("review") => return Err(refuse(Code::ModeNotAvailable, None, "review mode is not built").into()),
                _ => doc["plan"].clone(),
            }
        }
    };
    // The policy, the signer and the mode come from the plan, not from the record (P10O-4).
    let text = crate::identity::planned_policy(&plan).ok_or_else(|| changed("the plan creates no project file"))?;
    let settings = settings::parse(&cx.domain.join("settings.toml"), text, Level::Project)?;
    let policy = Policy { digest: sha256_hex(text.as_bytes()), text: text.to_owned(), settings };
    let (alias, actor, signer) = write::person(cx, &policy, "assessed")?;
    // The words' tag is `--say-lang`, else the first maestro's language: the plan's project file gives it as the
    // canonical language, which `languages.agentInteraction` defaults to, and their own file as that key.
    let say = write::words(cx, &policy, &alias, options)?;
    let mode = match plan["answers"]["publication"].as_str() {
        Some("own-repository") => Publication::OwnRepository,
        Some("unpublished") => Publication::Unpublished,
        // Never published by default: a plan with no mode it knows is not one `assess` gives.
        Some("published") => Publication::Published,
        _ => return Err(changed("the plan names no publication mode").into()),
    };
    let installed = install(cx, &plan, digest, &policy, (actor, signer, say), mode)?;
    Ok(out(cx, digest, installed.as_ref().map(|(written, receipt)| (written, &receipt[..], mode))))
}

/// The record as it stands, `None` when there is none.
fn recorded(cx: &Cx) -> Result<Option<Replay>, Failed> {
    match record::replay(&cx.domain, &PolicyVerifier) {
        Ok(r) => Ok(Some(r)),
        Err(e) if e.iter().all(|e| e.code() == Some(Code::RecordAbsent)) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// The plan an `apply` recorded, with its digest: the payload of the `assessed` that opens the `project` stream.
fn held(record: &Replay) -> Option<(&str, &Value)> {
    let first = record.streams.get("project")?.first().filter(|e| e.kind == "assessed")?;
    Some((first.value["payload"]["digest"].as_str()?, &first.value["payload"]["plan"]))
}

/// Each target of the plan as its inputs found it or as the plan writes it (point 7): `PLAN_CHANGED` otherwise, so
/// that `apply` never overwrites a file it did not write. A path reached through a link is neither.
fn targets(root: &Path, plan: &Value, record: Option<&Replay>) -> Result<(), Error> {
    let inputs = plan["inputs"].as_array().map(Vec::as_slice).unwrap_or_default();
    for f in plan["files"].as_array().into_iter().flatten() {
        let rel = f["path"].as_str().unwrap_or_default();
        let refused = || changed(&format!("{rel} is neither as the plan found it nor as it writes it"));
        // No path of the plan passes through a link, which would take a write out of the repository. A Git directory
        // outside the working tree, whose exclude file the plan names absolutely, never reaches here: the main working
        // tree's check refuses it first (`NOT_A_REPOSITORY`).
        if !crate::code::unlinked(root, rel) {
            return Err(refused());
        }
        let path = root.join(rel);
        let meta = fs::symlink_metadata(&path);
        let absent = meta.as_ref().is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound);
        let bytes = meta.as_ref().is_ok_and(fs::Metadata::is_file).then(|| fs::read(&path).ok()).flatten();
        let now = bytes.as_deref().map(sha256_hex);
        let input = inputs.iter().find(|i| i["path"] == rel);
        let found =
            input.is_some_and(|i| (i["kind"] == "absent" && absent) || (now.is_some() && i["digest"] == json!(now)));
        let written = now.is_some() && f["content"].as_str().map(|c| sha256_hex(c.as_bytes())) == now;
        let ok = match f["action"].as_str() {
            Some("create" | "block") => found || written,
            Some("keep") => now.is_some() && f["digest"] == json!(now),
            Some("exclude") => {
                found || bytes.is_some_and(|b| b.split(|c| *c == b'\n').any(|l| l == b"/.brewstillery/"))
            }
            Some("repository") => absent || meta.is_ok_and(|m| m.is_dir()),
            // A view is absent, or what the record as it stands gives, which an earlier `apply` wrote (P10B-2).
            _ => {
                let named = |e: &Error| match e {
                    Error::Refused { path: p, .. } => p.as_deref() == Some(path.as_path()),
                    Error::Io { path: p, .. } => p == &path,
                };
                let fresh = |r| !crate::view::stale(&root.join(".brewstillery"), r).iter().any(named);
                absent || record.is_some_and(fresh)
            }
        };
        if !ok {
            return Err(refused());
        }
    }
    Ok(())
}

/// The installation's entry to the write path (P10O-4, see the module's description); gives what it wrote and the
/// receipt's paths.
fn install(
    cx: &Cx,
    plan: &Value,
    digest: &str,
    policy: &Policy,
    (actor, signer, say): (Value, crate::identity::GitSigner, record::Say),
    mode: Publication,
) -> Result<Option<(Written, Vec<String>)>, Failed> {
    // The record branch is the product's checked-out branch in every mode, at the top of its main working tree.
    let product = Repo { root: cx.root.clone(), prefix: PathBuf::from(".brewstillery") };
    record::check_main_worktree(&product)?;
    record::record_branch(&product)?;
    // In `own-repository` the domain's repository commits once `apply` has made it.
    let own = fs::symlink_metadata(cx.domain.join(".git")).is_ok_and(|m| m.is_dir());
    let repo = Repo::for_domain(&cx.domain, mode).filter(|_| mode != Publication::OwnRepository || own);
    if let Some(r) = &repo {
        record::check_git_locks(r, &record::record_branch(r)?)?;
        record::check_ident(r)?;
    } else if mode == Publication::OwnRepository {
        // The repository `apply` makes has no configuration of its own: Git must name the author from the person's
        // global one, which it reads outside any repository, or the record commit would fail after the events.
        record::check_ident(&Repo { root: PathBuf::from("/"), prefix: PathBuf::new() })?;
    }
    let folder = cx.folder()?;
    let machine = crate::machine::machine_file(&folder)?;
    // What a failed Git prints is kept from here on, once nothing that refuses before the lock is left (D-083).
    record::git::keep_in(&cx.domain);
    crate::confine::end_idle(cx.backend, &cx.domain);
    let mut session = Session::begin(&cx.domain, repo, &PolicyVerifier)?;
    // Under the lock, read again, the record holds nothing, or only this plan's installation not complete (point 7).
    let record = recorded(cx)?;
    if record.as_ref().is_some_and(|r| held(r).is_none_or(|(d, _)| d != digest)) {
        return Err(changed("the record holds another installation than this plan's").into());
    }
    if let Some(r) = &record
        && write::pending(&cx.domain, r)?.is_none()
    {
        return Ok(None);
    }
    let present: Vec<(String, String)> =
        record.iter().flat_map(|r| r.streams.values().flatten()).map(|e| (e.stream.clone(), e.kind.clone())).collect();
    let has = |stream: &str, kind: &str| present.iter().any(|(s, k)| s == stream && k == kind);
    targets(&cx.root, plan, record.as_ref())?;
    // Every person's event missing is made and signed before anything is written.
    let (instance, pending) = crate::machine::prepare_instance(&folder)?;
    let alambique = plan["files"].as_array().into_iter().flatten().find(|f| f["path"] == ".brewstillery/ALAMBIQUE.md");
    let baseline = alambique.and_then(|f| {
        f["digest"].as_str().map(str::to_owned).or_else(|| Some(sha256_hex(f["content"].as_str()?.as_bytes())))
    });
    let events = [
        ("project", "assessed", json!({"plan": plan, "digest": digest})),
        (
            "project",
            "publication-decided",
            json!({"mode": plan["answers"]["publication"], "warningShown": plan.get("warning").is_some()}),
        ),
        (
            "project",
            "policy-adopted",
            json!({"content": policy.text, "digest": policy.digest, "heads": {}, "widened": []}),
        ),
        ("alambique", "alambique-baseline", json!({"digest": baseline})),
    ];
    let mut heads = [("project", session.head("project")?), ("alambique", session.head("alambique")?)];
    let mut prepared: Vec<(&str, &str, Prepared)> = Vec::new();
    for (stream, kind, payload) in events.into_iter().filter(|(s, k, _)| !has(s, k)) {
        let draft = Draft {
            kind: kind.into(),
            actor: actor.clone(),
            policy: policy.digest.clone(),
            instance: instance.clone(),
            say: Some(say.clone()),
            payload,
        };
        let head = heads.iter_mut().find(|(s, _)| *s == stream).map(|(_, h)| h).ok_or_else(|| changed(stream))?;
        let p = session.prepare(stream, head.as_ref(), &draft, Some(&signer))?;
        *head = Some(p.head().clone());
        prepared.push((stream, kind, p));
    }
    if let Some(pending) = pending {
        pending.install()?;
    }
    let mut written: Vec<(String, Head)> = Vec::new();
    let mut later = Vec::new();
    for (stream, kind, p) in prepared {
        if matches!(kind, "assessed" | "publication-decided") {
            written.push((stream.to_owned(), session.write(p)?));
        } else {
            later.push((stream, p));
        }
    }
    // The plan's files, each whole and atomically; the entry blocks; the domain's repository and the exclude line.
    let mut receipt = Vec::new();
    for action in ["create", "block", "repository", "exclude"] {
        for f in plan["files"].as_array().into_iter().flatten().filter(|f| f["action"] == action) {
            let rel = f["path"].as_str().unwrap_or_default().to_owned();
            let path = cx.root.join(&rel);
            match (action, f["content"].as_str()) {
                ("repository", _) => {
                    if !fs::symlink_metadata(&path).is_ok_and(|m| m.is_dir()) {
                        let domain = Repo { root: cx.domain.clone(), prefix: PathBuf::new() };
                        let branch = f["branch"].as_str().unwrap_or("main");
                        record::fault::point();
                        record::git::git(&domain, &record::git::os(&["init", "-q", "-b", branch]), b"", None)?;
                    }
                    session.attach(Repo { root: cx.domain.clone(), prefix: PathBuf::new() })?;
                    continue;
                }
                ("exclude", _) => _ = crate::domain::exclude_domain(&cx.domain)?,
                (_, Some(content)) if fs::read(&path).ok().as_deref() != Some(content.as_bytes()) => {
                    let dir = path.parent().unwrap_or(&cx.root);
                    fs::create_dir_all(dir).map_err(crate::code::io_at(dir))?;
                    // An entry file keeps its permissions (REQ-DOM-6), given before it is in place (P10B-3).
                    record::write_atomic_via(&path, content.as_bytes(), &cx.domain.join("local/tmp"))?;
                }
                _ => {}
            }
            receipt.push(rel);
        }
    }
    // The mode's checks, once the exclude line and the domain's repository are in place.
    crate::domain::check_ignored(&cx.domain, mode)?;
    for (stream, p) in later {
        written.push((stream.to_owned(), session.write(p)?));
    }
    if !has("project", "installed") {
        let mut files = Vec::new();
        for rel in &receipt {
            let path = cx.root.join(rel);
            let bytes = fs::read(&path).map_err(crate::code::io_at(&path))?;
            files.push(json!({"path": rel, "digest": sha256_hex(&bytes)}));
        }
        let payload = json!({"plan": digest, "files": files});
        let fabrewk = json!({"kind": "fabrewk"});
        let draft = Draft {
            kind: "installed".into(),
            actor: fabrewk,
            policy: policy.digest.clone(),
            instance,
            say: None,
            payload,
        };
        let p = session.prepare("project", session.head("project")?.as_ref(), &draft, None)?;
        written.push(("project".into(), session.write(p)?));
    }
    // The views from the record these events made, then the record commit with the receipt's committed paths.
    session.replayable(&PolicyVerifier)?;
    let after = record::replay(&cx.domain, &PolicyVerifier).map_err(|mut e| e.remove(0))?;
    let views = crate::view::write(&cx.domain, &after)?;
    // The alambique kept is the approved text of departure, which the baseline names: committed with it, so that the
    // domain's files are whole in the commit (point 6) and a later approval starts from the committed text.
    let kept = plan["files"].as_array().into_iter().flatten().filter(|f| f["action"] == "keep");
    let kept: Vec<String> = kept.filter_map(|f| f["path"].as_str().map(str::to_owned)).collect();
    let committed = receipt.iter().chain(&kept).filter(|r| match mode {
        Publication::Published => true,
        Publication::OwnRepository => r.starts_with(".brewstillery/"),
        Publication::Unpublished => false,
    });
    session.install(committed.map(|r| cx.root.join(r)).collect())?;
    let commit = session.finish_held(&PolicyVerifier, &views)?;
    write::caches(cx, &session, machine.as_ref(), &[])?;
    let recovered = session.take_notes();
    Ok(Some((Written { events: written, commit, ran: Value::Null, recovered }, receipt)))
}

/// What `apply` reports: the plan applied, or applied already, what it wrote and committed, and what it left
/// uncommitted in the product with the Git commands that commit it, running none.
fn out(cx: &Cx, digest: &str, applied: Option<(&Written, &[String], Publication)>) -> Out {
    let short = &digest[..12.min(digest.len())];
    let Some((written, receipt, mode)) = applied else {
        let text = cx.t("apply.already", &[("digest", short)]) + "\n" + &cx.next(&["fabrewk status", "fabrewk check"]);
        return Out::new(json!({"schema": "fabrewk.applied/1", "digest": digest, "applied": false}), text);
    };
    let mut text = cx.t("apply.done", &[("digest", short)]) + "\n";
    let mut left: Vec<&str> = match mode {
        Publication::Published => Vec::new(),
        _ => receipt.iter().map(String::as_str).filter(|r| matches!(*r, "AGENTS.md" | "CLAUDE.md")).collect(),
    };
    left.sort_unstable();
    let mut doc =
        json!({"schema": "fabrewk.applied/1", "digest": digest, "applied": true, "files": receipt, "left": left});
    text += &super::adopt::written_lines(cx, written, &mut doc);
    if !left.is_empty() {
        let run = [format!("git add -- {}", left.join(" ")), "git commit".to_owned()];
        text += &format!("{}\n  {}\n  {}\n", cx.t("apply.left", &[("path", &left.join(", "))]), run[0], run[1]);
        doc["run"] = json!(run);
    }
    Out::new(doc, text + &cx.next(&["fabrewk status", "fabrewk check"]))
}
