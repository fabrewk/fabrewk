//! The formula's commands that take no person's move (formula 1a points 3 and 5): `fabrewk import <commit>` records the
//! formula's first approval, which a maestro made before Fabrewk could record one by signing a commit of `FORMULA.md`
//! alone (the bridge); `fabrewk cut <id> [--report]` records the cut of the approved formula the integrator opens, or
//! reports done for the maestro's audit. Neither grants anything: anyone may run them, they take no words, and Fabrewk
//! is their events' actor. The import's guards: the formula has no approval yet (`MOVE_NOT_ALLOWED`), and the commit
//! is in the record branch's history, changes `FORMULA.md` alone, and Git verifies its signature against a maestro's
//! key in the policy in force, given as the only allowed signers (`FORMULA_BRIDGE_INVALID`).

use super::adopt::written_lines;
use super::write::{self, Plan};
use super::{Cx, Failed, Options, Out};
use crate::alambique::{self, FORMULA, cuts};
use crate::code::{Code, Error, refuse};
use crate::identity;
use crate::record::git::{command, failed, git, line, os, run};
use crate::record::{Draft, Replay, Repo, sha256_hex};
use serde_json::{Value, json};

fn draft(policy: &str, kind: &str, payload: Value) -> Draft {
    let actor = json!({"kind": "fabrewk"});
    Draft { kind: kind.into(), actor, policy: policy.to_owned(), instance: String::new(), say: None, payload }
}

fn plan(events: Vec<(String, Draft)>, out: Value) -> Plan<Value> {
    Plan { events, signer: None, sealed: false, view: None, alambique: None, then: None, out }
}

/// Git's answer to `args` in `repo` when it exits 0 or 1, `None` for 1; any other end is Git's failure.
fn asked(repo: &Repo, args: &[&str]) -> Result<Option<String>, Error> {
    let out = run(command(repo, &os(args)), b"")?;
    match out.status.code() {
        Some(0) => Ok(Some(line(out.stdout))),
        Some(1) => Ok(None),
        _ => Err(failed(repo, &os(args), &out)),
    }
}

/// The import's plan: the bridge commit `rev` verified, and its `FORMULA.md` recorded as the first approval.
fn bridge(cx: &Cx, rev: &str, record: &Replay) -> Result<Plan<Value>, Failed> {
    let st = write::state(cx, record)?;
    let invalid = |why: String| refuse(Code::FormulaBridgeInvalid, None, why);
    if record.streams.contains_key(FORMULA.stream) {
        let why = "the record holds the formula's approvals already; a bridge is only its first";
        return Err(refuse(Code::MoveNotAllowed, None, why).into());
    }
    let repo = Repo::for_domain(&cx.domain, st.mode);
    let repo = repo.ok_or_else(|| invalid("an unpublished domain is in no repository that holds a commit".into()))?;
    let spec = format!("{rev}^{{commit}}");
    let commit = asked(&repo, &["rev-parse", "--verify", "--quiet", "--end-of-options", &spec])?;
    let commit = commit.ok_or_else(|| invalid(format!("{rev} names no commit")))?;
    let branch = crate::record::record_branch(&repo)?;
    if asked(&repo, &["merge-base", "--is-ancestor", &commit, &branch])?.is_none() {
        return Err(invalid(format!("{commit} is not in the history of the record branch {branch}")).into());
    }
    let rel = repo.prefix.join(FORMULA.file).to_string_lossy().into_owned();
    let tree = ["diff-tree", "--root", "--no-commit-id", "--name-only", "-r", "-z", &commit];
    if git(&repo, &os(&tree), b"", None)? != format!("{rel}\0").as_bytes() {
        let why = format!("{commit} does not change {rel} alone: a bridge is a commit of that file only");
        return Err(invalid(why).into());
    }
    // Git verifies the signature with the maestros of the policy in force as the only allowed signers. As Git does with
    // the signature it verifies, their file is one of the system's temporary directory, made new (no link followed),
    // readable by its owner only and removed once read: a refusal writes nothing in the repository.
    let maestros = identity::maintainers(&st.policy.settings).into_iter().filter(|m| m.role == "maestro");
    let key = |k: &str| k.split(' ').filter(|p| !p.is_empty()).take(2).collect::<Vec<_>>().join(" ");
    let signers: String = maestros.map(|m| format!("{} namespaces=\"git\" {}\n", m.alias, key(&m.key))).collect();
    let n = crate::record::nonce().map_err(|e| Error::Io { path: "/dev/urandom".into(), error: e })?;
    let file = std::env::temp_dir().join(format!("fabrewk-signers-{n}"));
    let mut options = std::fs::File::options();
    std::os::unix::fs::OpenOptionsExt::mode(options.write(true).create_new(true), 0o600);
    let wrote = options.open(&file).and_then(|mut f| std::io::Write::write_all(&mut f, signers.as_bytes()));
    wrote.map_err(crate::code::io_at(&file))?;
    let allowed = format!("gpg.ssh.allowedSignersFile={}", file.display());
    let shown = git(&repo, &os(&["-c", &allowed, "show", "-s", "--format=%G?%n%GS", &commit]), b"", None);
    let _ = std::fs::remove_file(&file);
    let shown = line(shown?);
    let alias = shown.strip_prefix("G\n").filter(|a| !a.is_empty() && !a.contains('\n'));
    let why = || format!("{commit} is not signed with the key of a maestro of the policy in force");
    let alias = alias.ok_or_else(|| invalid(why()))?.to_owned();
    let bytes = git(&repo, &os(&["cat-file", "blob", &format!("{commit}:{rel}")]), b"", None)?;
    let payload = json!({"commit": commit, "digest": sha256_hex(&bytes), "alias": alias});
    Ok(plan(vec![(FORMULA.stream.into(), draft(&st.policy.digest, "formula-imported", payload.clone()))], payload))
}

/// `fabrewk import <commit>`.
pub(super) fn import(cx: &Cx, rev: &str) -> Result<Out, Failed> {
    let (p, written) = write::run(cx, |record| bridge(cx, rev, record))?;
    let s = |k: &str| p[k].as_str().unwrap_or_default().to_owned();
    let params = [("commit", &s("commit")[..12]), ("alias", &s("alias")), ("digest", &s("digest")[..12])];
    let mut text = cx.t("import.done", &params) + "\n";
    // `commit` is the record commit, as in every write command's document; the bridge is named apart.
    let mut doc = json!({"schema": "fabrewk.imported/1", "bridge": p["commit"], "digest": p["digest"]});
    doc["alias"] = p["alias"].clone();
    text += &written_lines(cx, &written, &mut doc);
    text += &cx.next(&["fabrewk check"]);
    Ok(Out::new(doc, text))
}

/// The cut's plan: a cut of the formula as last approved, opened, or reported when it is the one in progress.
fn recorded(cx: &Cx, id: &str, report: bool, record: &Replay) -> Result<Plan<Value>, Failed> {
    let st = write::state(cx, record)?;
    let path = cx.domain.join(FORMULA.file);
    alambique::check(record, &cx.domain, &FORMULA)?;
    let digest = alambique::approved(record, &FORMULA);
    let digest = digest.ok_or_else(|| refuse(Code::FormulaUnapproved, Some(&path), "the project has no formula"))?;
    let text = alambique::read(&cx.domain, &FORMULA)?.map(|b| String::from_utf8_lossy(&b).into_owned());
    let c = cuts(&text.unwrap_or_default()).into_iter().find(|c| c.id == id);
    let c =
        c.ok_or_else(|| refuse(Code::CutUnknown, Some(&path), format!("{id} is no cut of the approved formula")))?;
    let last = alambique::cut(record).map(|e| &e.value["payload"]);
    let open = last.filter(|p| p["reported"] == false).map(|p| p["cut"].as_str().unwrap_or_default());
    if report != (open == Some(id)) {
        let why = match open {
            Some(o) if report => format!("the cut in progress is {o}, not {id}"),
            None if report => "no cut is in progress".to_owned(),
            _ => format!("{id} is in progress already"),
        };
        return Err(refuse(Code::MoveNotAllowed, None, why).into());
    }
    let payload = json!({"cut": c.id, "phase": c.phase, "title": c.title, "reported": report, "formula": digest});
    Ok(plan(vec![(FORMULA.stream.into(), draft(&st.policy.digest, "cut-recorded", payload.clone()))], payload))
}

/// `fabrewk cut <id> [--report]`.
pub(super) fn cut(cx: &Cx, id: &str, options: &Options) -> Result<Out, Failed> {
    let (p, written) = write::run(cx, |record| recorded(cx, id, options.report, record))?;
    let s = |k: &str| p[k].as_str().unwrap_or_default().to_owned();
    let key = if options.report { "cut.reported" } else { "cut.opened" };
    let mut text = cx.t(key, &[("id", id), ("title", &s("title")), ("phase", &s("phase"))]) + "\n";
    let mut doc = json!({"schema": "fabrewk.cut/1", "cut": p["cut"], "reported": p["reported"]});
    doc["formula"] = p["formula"].clone();
    text += &written_lines(cx, &written, &mut doc);
    text += &cx.next(&["fabrewk status"]);
    Ok(Out::new(doc, text))
}
