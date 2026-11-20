//! The path every write command takes (part 7d3a; D-047, D-053, D-055, D-062). Its guards run first on the record as
//! the command read it without the lock, so that a refusal writes nothing; then the mode's checks (REQ-DOM-7), the
//! session (REQ-REG-5, REQ-REG-8, REQ-REG-11), the same guards again on the record replayed under the lock, the record
//! commit of what an earlier command left uncommitted (REQ-REG-10), the machine key (REQ-REG-12), the events, each
//! appended by compare-and-swap after the head its plan saw, and the command's record commit. A person's move is
//! refused inside an agent (`AGENT_ENVIRONMENT`), is signed with Git's signing key and carries the person's words
//! (REQ-TASK-4).

use super::{Cx, Failed, Options, get};
use crate::code::{Code, Error, refuse};
use crate::detect::{self, Space};
use crate::identity::{self, GitSigner, PolicyVerifier};
use crate::record::{Draft, Head, Prepared, Publication, Replay, Repo, Say, Session};
use crate::settings::{self, Policy, Val};
use crate::{alambique, fermento, view};
use crate::{domain, machine};
use std::collections::BTreeMap;

/// The core's own marker: the environment of a process an extension runs (REQ-SEC-7).
pub const EXTENSION_MARKER: &str = "FABREWK_EXTENSION";

/// What a write command plans from a record: the policy in force and the publication mode.
pub(super) struct State {
    pub policy: Policy,
    pub mode: Publication,
}

/// The policy in force (`POLICY_ABSENT` with none) and the publication mode `apply` records with it, once the
/// installation is complete.
pub(super) fn state(cx: &Cx, record: &Replay) -> Result<State, Failed> {
    installation(&cx.domain, record)?;
    let policy = settings::in_force(record)?;
    let policy = policy.ok_or_else(|| refuse(Code::PolicyAbsent, Some(&cx.domain), "no policy is adopted"))?;
    let mode = domain::publication(record);
    let mode = mode.ok_or_else(|| {
        refuse(Code::RecordBroken, Some(&cx.domain), "the project stream records no publication mode")
    })?;
    Ok(State { policy, mode })
}

/// Refuses `INSTALLATION_INCOMPLETE` while an `apply` is not complete (D-097 point 7, P10O-6): the record it opened with
/// `assessed` holds no `installed`, or, in `unpublished`, a view is not written, or, in a mode that commits, the record
/// branch's tip lacks `installed`. Facts of the record, the views and the branch, never of the index (P10O-15). A
/// record no `apply` opened is no installation.
pub(super) fn installation(domain: &std::path::Path, record: &Replay) -> Result<(), Error> {
    let project = record.streams.get("project").map(Vec::as_slice).unwrap_or_default();
    if project.first().is_none_or(|e| e.kind != "assessed") {
        return Ok(());
    }
    let incomplete = |why: &str| Err(refuse(Code::InstallationIncomplete, Some(domain), why.to_owned()));
    let Some(installed) = project.iter().find(|e| e.kind == "installed") else {
        return incomplete("the record holds no installed event");
    };
    match domain::publication(record).and_then(|m| Repo::for_domain(domain, m)) {
        Some(repo) if crate::record::uncommitted(&repo)?.contains(&installed.path) => {
            incomplete("the record commit holding the installed event is not made")
        }
        None if ["STATUS.md", "RULES.md"]
            .iter()
            .any(|v| !std::fs::symlink_metadata(domain.join(v)).is_ok_and(|m| m.is_file())) =>
        {
            incomplete("the views are not written")
        }
        _ => Ok(()),
    }
}

/// Refuses a person's move where the environment shows an agent's or an extension's marker (`AGENT_ENVIRONMENT`): a
/// cooperative control, since a process may clear its environment (REQ-TASK-4, REQ-SEC-7).
pub(super) fn not_an_agent(cx: &Cx) -> Result<(), Error> {
    let found = cx.markers.iter().copied().chain([EXTENSION_MARKER]).find(|m| std::env::var_os(m).is_some());
    match found {
        Some(m) => Err(refuse(Code::AgentEnvironment, None, format!("{m} is set in the environment"))),
        None => Ok(()),
    }
}

/// The person who makes a move of `kind` (D-055): the maintainer whose key Git signs with, their actor and a signer;
/// `ROLE_FORBIDDEN` when Git names no key, the key is no maintainer's, or the role may not make the move (REQ-ID-3).
pub(super) fn person(cx: &Cx, policy: &Policy, kind: &str) -> Result<(String, serde_json::Value, GitSigner), Error> {
    let forbidden = |why: &str| refuse(Code::RoleForbidden, None, why.to_owned());
    // A Git that cannot say which key it signs with is its failure, never a key unset (D-083); read once (P9G-15).
    let named = crate::record::signing_key_read(&cx.root)?;
    let key = named
        .and_then(|n| identity::key_named(&cx.root, n))
        .ok_or_else(|| forbidden("Git names no signing key (user.signingkey) that reads as a public key"))?;
    let alias = identity::holder(&policy.settings, &key.public).ok_or_else(|| {
        forbidden("the key Git signs with (user.signingkey) is no maintainer's in the policy in force")
    })?;
    let signer = GitSigner { key, tmp: cx.domain.join("local/tmp/signing-key.pub") };
    let actor = identity::actor(&policy.settings, &alias, &signer, kind)?;
    Ok((alias, actor, signer))
}

/// The person's words (REQ-TASK-4): given, valid UTF-8 and not empty after trimming (`WORDS_MISSING`), and not an
/// example a catalogue prints (`WORDS_COPIED`); their tag is `--say-lang`, else the person's
/// `languages.agentInteraction`.
pub(super) fn words(cx: &Cx, policy: &Policy, alias: &str, options: &Options) -> Result<Say, Failed> {
    let missing = |why: &str| refuse(Code::WordsMissing, None, why.to_owned());
    let given = options.say.clone().ok_or_else(|| missing("no --say was given"))?;
    let text = given.into_string().map_err(|_| missing("the words are not valid UTF-8"))?;
    if text.trim().is_empty() {
        return Err(missing("the words are empty after trimming").into());
    }
    let examples = super::CATALOGUES.iter().filter_map(|(lang, _)| get(lang, "words.example"));
    if examples.map(str::trim).any(|e| e == text.trim()) {
        return Err(refuse(Code::WordsCopied, None, "the words are the catalogue's example").into());
    }
    let lang = match &options.say_lang {
        Some(lang) => lang.clone(),
        None => interaction_lang(cx, policy, alias)?,
    };
    Ok(Say { text, lang })
}

/// The language a maintainer talks to agents in (`languages.agentInteraction`), `en` when none is set.
pub(super) fn interaction_lang(cx: &Cx, policy: &Policy, alias: &str) -> Result<String, Failed> {
    let personal = cx.personal_of(alias)?;
    let eff = settings::effective(policy, personal.as_ref(), None, &[], &[])?;
    Ok(match eff.values.get("languages.agentInteraction") {
        Some((Val::Str(s), _)) => s.clone(),
        _ => "en".to_owned(),
    })
}

/// What a command's guards allow it to write: its events in order, by stream, the signer of the person's among them,
/// whether the first is an adoption whose seal the session gives, and what the command reports.
pub(super) struct Plan<T> {
    pub events: Vec<(String, Draft)>,
    pub signer: Option<GitSigner>,
    pub sealed: bool,
    pub view: Option<View>,
    pub alambique: Option<Edit>,
    /// What writes the events instead, once they are made and signed, under the lock (a run, part 9e): it may write
    /// others of Fabrewk's own after, or instead of, them; what it gives the command reports.
    pub then: Option<Then>,
    pub out: T,
}

/// A write command's continuation under the lock (see [`Plan::then`]).
pub(super) type Then = Box<dyn FnOnce(&mut Appender, Vec<(String, Prepared)>) -> Result<serde_json::Value, Error>>;

/// What a continuation writes with: the session, the machine's identity and the policy in force for Fabrewk's own
/// events, what it wrote, and the programs and tools a use verified, which the detection kept then marks.
pub(super) struct Appender<'s> {
    session: &'s mut Session,
    instance: String,
    policy: String,
    written: Vec<(String, Head)>,
    pub used: Vec<(Space, String)>,
}

impl Appender<'_> {
    /// Writes an event made and signed before (compare-and-swap on its stream's head).
    pub fn write(&mut self, stream: String, p: Prepared) -> Result<(), Error> {
        let head = self.session.write(p)?;
        self.written.push((stream, head));
        Ok(())
    }

    /// The branch of the repository at `root` moved to `commit` by this command (see [`Session::moved`]).
    pub fn moved(&mut self, root: &std::path::Path, commit: &str) {
        self.session.moved(root, commit);
    }

    /// The place the next event of `stream` takes.
    pub fn next(&self, stream: &str) -> Result<u64, Error> {
        Ok(self.session.head(stream)?.map_or(1, |h| h.seq + 1))
    }

    /// Appends one of Fabrewk's own events of `kind` to `stream`, after the head it has now.
    pub fn fabrewk(&mut self, stream: &str, kind: &str, payload: serde_json::Value) -> Result<Head, Error> {
        crate::record::fault::point();
        let actor = serde_json::json!({"kind": "fabrewk"});
        let (instance, policy) = (self.instance.clone(), self.policy.clone());
        let draft = Draft { kind: kind.into(), actor, policy, instance, say: None, payload };
        let p = self.session.prepare(stream, self.session.head(stream)?.as_ref(), &draft, None)?;
        let head = p.head().clone();
        self.write(stream.to_owned(), p)?;
        Ok(head)
    }
}

/// The alambique's approval (REQ-FER-5): the digest of the file as the plan saw it, the text to
/// put there (`None` when the person's own is approved) and the message of the commit that holds it alone.
pub(super) struct Edit {
    pub before: String,
    pub bytes: Option<Vec<u8>>,
    pub message: String,
}

/// A maintainer's fermento whose answer the command rewrites from the record once its events are written (REQ-FER-3):
/// the digest of the input it planned on, and the answer from the record as it then is.
pub(super) struct View {
    pub alias: String,
    pub digest: String,
    pub answer: Box<dyn Fn(&Replay) -> String>,
}

/// What a write command wrote: each event's stream and head, and the record commit, if the mode commits.
pub(super) struct Written {
    pub events: Vec<(String, Head)>,
    pub commit: Option<String>,
    /// What the continuation gave, if any.
    pub ran: serde_json::Value,
    /// What failed in Git that the command went on from, each naming its evidence (D-088).
    pub recovered: Vec<String>,
}

fn head_in(record: &Replay, stream: &str) -> Option<Head> {
    let last = record.streams.get(stream)?.last()?;
    Some(Head { seq: last.seq, digest: last.digest.clone() })
}

/// Runs a write command whose guards and events `plan` gives from a record (see the module's description). Nothing
/// is written before every guard has passed and every event is made and signed: the record commit of what an earlier
/// command left, the machine key and the events come after, so that a key that cannot sign refuses with nothing
/// written. On a machine with no machine key yet, the signer first signs a probe, so that the key is made only when the
/// person's key signs. The lock is held until the last cache is written.
pub(super) fn run<T>(cx: &Cx, plan: impl Fn(&Replay) -> Result<Plan<T>, Failed>) -> Result<(T, Written), Failed> {
    // What a failed Git prints is kept as this command's evidence (D-083, D-086 point 3).
    crate::record::git::keep_in(&cx.domain);
    // What a killed run left beside its cells is ended first, whatever this command does (D-092).
    crate::confine::end_idle(cx.backend, &cx.domain);
    // What failed in Git that the command went on from, a recovery's first included (D-089).
    let mut notes = Vec::new();
    // A task found running, before the lock or under it, is recovered first (REQ-RUN-5); then the command plans again
    // on the record the recovery left (P9E-13). Each turn either recovers a task, refuses, or ends.
    let mut owned: Option<Replay> = None;
    loop {
        let record = match &owned {
            Some(r) => r,
            None => cx.record.as_ref().map_err(|e| Failed::from(e.clone()))?,
        };
        // What a recovery went on from is named by a refusal that follows it too (P9G-20).
        let before = notes.clone();
        if running(record) {
            apply(cx, record, &|r: &Replay| recovery(cx, r), true, &mut notes).map_err(|f| noted(f, &before))?;
        } else if let Some((out, mut written)) =
            apply(cx, record, &plan, false, &mut notes).map_err(|f| noted(f, &before))?
        {
            written.recovered = notes;
            return Ok((out, written));
        }
        owned = Some(crate::record::replay(&cx.domain, &PolicyVerifier)?);
    }
}

/// Whether `record` holds a task in `running`.
fn running(record: &Replay) -> bool {
    crate::table::current(record).is_ok_and(|s| s.values().any(|s| *s == "running"))
}

/// REQ-RUN-5: every task left `running` by a process that no longer runs (none holds the lock, which a run keeps for
/// its whole run) has its open attempt ended as `unknown`, counted, and halts (`OUTCOME_UNKNOWN`); its attempt's
/// directories, whose cell ended with that process, are removed. Its own record commit, before the command's.
fn recovery(cx: &Cx, record: &Replay) -> Result<Plan<()>, Failed> {
    let policy = state(cx, record)?.policy;
    let mut events = Vec::new();
    let mut gone = Vec::new();
    for (stream, s) in crate::table::current(record)? {
        if s != "running" {
            continue;
        }
        let draft = |kind: &str, payload| Draft {
            kind: kind.into(),
            actor: serde_json::json!({"kind": "fabrewk"}),
            policy: policy.digest.clone(),
            instance: String::new(),
            say: None,
            payload,
        };
        let events_of = &record.streams[&stream];
        if let Some(open) = super::run::open_attempt(events_of) {
            let payload = serde_json::json!({
                "attempt": open, "outcome": "unknown", "model": null,
                "usage": crate::adapter::usage(None, None), "evidence": null,
            });
            events.push((stream.clone(), draft("attempt-ended", payload)));
        }
        // The last attempt's directories, open or ended before the run stopped (P9E-4).
        if let Some(last) = events_of.iter().rfind(|e| e.kind == "attempt-started") {
            gone.push(cx.domain.join("local/tmp").join(super::run::attempt_name(&stream, last.seq)));
        }
        // The directories of a check the stopped run made, after its start; an earlier run's left unread stay so
        // (part 9f).
        let check = crate::gate::check_name(&stream, 0);
        let prefix = &check[..check.len() - 1];
        let since = events_of.iter().rfind(|e| e.kind == "run-started").map_or(0, |e| e.seq);
        let ours = |n: &std::ffi::OsStr| {
            n.to_str().and_then(|n| n.strip_prefix(prefix)?.parse::<u64>().ok()).is_some_and(|seq| seq > since)
        };
        let tmp = std::fs::read_dir(cx.domain.join("local/tmp")).into_iter().flatten().flatten();
        gone.extend(tmp.map(|e| e.path()).filter(|p| p.file_name().is_some_and(ours)));
        let reason = "the process that ran it no longer runs; its attempt is counted as spent, and a retry spends a \
                      dispatch";
        events.push((stream, draft("task-halted", serde_json::json!({"code": "OUTCOME_UNKNOWN", "reason": reason}))));
    }
    let backend = cx.backend;
    let then: Then = Box::new(move |a, prepared| {
        // Removed before the events, so that a recovery stopped between them is made again whole (P9E-8); following no
        // link, and only what exists: a run killed before it made them left none. A cell the backend cannot show ended
        // with nothing left beside it keeps its directories, for a later command (D-092, P9I-2).
        let dirs = gone.into_iter().filter(|d| std::fs::symlink_metadata(d).is_ok_and(|m| m.is_dir()));
        for root in dirs.filter(|d| crate::confine::settled(backend, d)) {
            let [home, tmp, work] = ["home", "tmp", "work"].map(|d| root.join(d));
            crate::confine::Scratch { root, home, tmp, work }.remove()?;
        }
        for (stream, p) in prepared {
            a.write(stream, p)?;
        }
        Ok(serde_json::Value::Null)
    });
    Ok(Plan { events, signer: None, sealed: false, view: None, alambique: None, then: Some(then), out: () })
}

/// Runs a write command's `plan` on `record` (see [`run`]); `None`, with nothing written, when the record read under
/// the lock holds a task in `running` that is not this plan's to recover.
fn apply<T>(
    cx: &Cx,
    record: &Replay,
    plan: &dyn Fn(&Replay) -> Result<Plan<T>, Failed>,
    recovering: bool,
    notes: &mut Vec<String>,
) -> Result<Option<(T, Written)>, Failed> {
    // A record cut below this machine's mark is a rollback before anything it lacks is missed.
    crate::record::check_mark(&cx.domain, record)?;
    let mode = state(cx, record)?.mode;
    domain::check_ignored(&cx.domain, mode)?;
    // The record branch is the product's checked-out branch in every mode (REQ-REG-11); the session checks it where
    // it commits.
    if mode != Publication::Published
        && let Some(root) = cx.domain.parent()
    {
        let product = Repo { root: root.to_path_buf(), prefix: std::path::PathBuf::from(".brewstillery") };
        crate::record::record_branch(&product)?;
    }
    // A lock a stopped Git left, and a Git that cannot name the commit's author, are refused before anything is
    // written (P7D3A-7, P7D3A-14); the commit checks the lock again.
    if let Some(repo) = Repo::for_domain(&cx.domain, mode) {
        crate::record::check_git_locks(&repo, &crate::record::record_branch(&repo)?)?;
        crate::record::check_ident(&repo)?;
    }
    let folder = cx.folder()?;
    // The private folder and the machine file are read before anything is written, as every write uses them.
    let machine = machine::machine_file(&folder)?;
    plan(record)?;
    let mut session = Session::begin(&cx.domain, Repo::for_domain(&cx.domain, mode), &PolicyVerifier)?;
    // What the session's commits went on from leaves with the command's outcome, whichever it is (D-089).
    let result = held(cx, plan, (recovering, mode), machine, &mut session);
    let taken = session.take_notes();
    drop(session);
    let refused = result.map_err(|f| noted(f, &taken));
    notes.extend(taken);
    refused
}

/// `f` with `notes`, what failed in Git that the command went on from before it refused, named after its own words.
fn noted(f: Failed, notes: &[String]) -> Failed {
    match f {
        Failed::Refused(mut errors) if !notes.is_empty() => {
            if let Some(Error::Refused { detail, .. }) = errors.first_mut() {
                *detail += &format!("; before it, {}", notes.join("; "));
            }
            Failed::Refused(errors)
        }
        f => f,
    }
}

/// [`apply`] once its session holds the lock.
fn held<T>(
    cx: &Cx,
    plan: &dyn Fn(&Replay) -> Result<Plan<T>, Failed>,
    (recovering, mode): (bool, Publication),
    machine: Option<crate::settings::Settings>,
    session: &mut Session,
) -> Result<Option<(T, Written)>, Failed> {
    let folder = cx.folder()?;
    session.refresh(&PolicyVerifier)?;
    let record = session.record().ok_or_else(|| refuse(Code::RecordAbsent, Some(&cx.domain), "no event"))?;
    if !recovering && running(record) {
        return Ok(None);
    }
    let planned = plan(record)?;
    let policy = state(cx, record)?.policy.digest;
    let mut heads: BTreeMap<String, Option<Head>> =
        planned.events.iter().map(|(s, _)| (s.clone(), head_in(record, s))).collect();
    // The machine key is made under a temporary name and put in place only once every event is signed, so that a key
    // that cannot sign leaves the private folder as it was (P7D3A-1).
    let (instance, pending) = machine::prepare_instance(&folder)?;
    let mut prepared: Vec<(String, Prepared)> = Vec::new();
    for (i, (stream, mut draft)) in planned.events.into_iter().enumerate() {
        draft.instance = instance.clone();
        if planned.sealed && i == 0 {
            let seal = session.seal()?;
            let heads: serde_json::Map<_, _> =
                seal.iter().map(|(s, h)| (s.clone(), serde_json::json!({"digest": h.digest, "seq": h.seq}))).collect();
            draft.payload["heads"] = heads.into();
        }
        let signer = draft.say.as_ref().and(planned.signer.as_ref()).map(|s| s as &dyn crate::record::Sign);
        let expected = heads.get(&stream).cloned().flatten();
        // A run binds its stream's head as made here and, when the same command authorizes it, that authorization.
        if draft.kind == "run-started" {
            draft.payload["head"] = serde_json::json!({"seq": expected.as_ref().map(|h| h.seq), "digest": expected.as_ref().map(|h| &h.digest)});
            if let Some((by, p)) = prepared.last().filter(|_| draft.payload["authorization"].is_null()) {
                let h = p.head();
                draft.payload["authorization"] = serde_json::json!({"stream": by, "seq": h.seq, "digest": h.digest});
            }
        }
        let p = session.prepare(&stream, expected.as_ref(), &draft, signer)?;
        heads.insert(stream.clone(), Some(p.head().clone()));
        prepared.push((stream, p));
    }
    // Every guard that can still refuse comes before the machine key is put in place or anything is committed: a
    // refusal here writes nothing (11.2 rule 4). The alambique is the text the plan saw, in the file and, for an edit
    // of it, in the record branch's tip and the person's index; it is checked again once the events are written.
    let repo = Repo::for_domain(&cx.domain, mode);
    let unchanged = |edit: &Edit| -> Result<(), Error> {
        alambique::unchanged(&cx.domain, repo.as_ref(), &edit.before, edit.bytes.is_some()).map(drop)
    };
    if let Some(a) = &planned.alambique {
        crate::record::fault::pause("before-alambique");
        unchanged(a)?;
    }
    if let Some(v) = &planned.view {
        crate::record::fault::pause("before-view");
        fermento::current(&cx.domain, &v.alias, &v.digest)?;
    }
    if let Some(pending) = pending {
        pending.install()?;
    }
    session.commit_uncommitted(&PolicyVerifier)?;
    crate::record::fault::pause("before-append");
    // A change that comes after the events are written leaves them written, as a failed record commit does (D-065).
    let late = |what: &str| {
        let rest = format!("; this command's events are written and not committed, and {what}");
        move |e: Error| match e {
            Error::Refused { code, path, detail } => Error::Refused { code, path, detail: format!("{detail}{rest}") },
            other => other,
        }
    };
    let (mut written, mut ran, mut used) = (Vec::new(), serde_json::Value::Null, Vec::new());
    match planned.then {
        None => {
            for (stream, p) in prepared {
                written.push((stream, session.write(p)?));
            }
        }
        Some(then) => {
            let mut a = Appender { session, instance: instance.clone(), policy, written, used };
            let result = then(&mut a, prepared);
            (written, used) = (a.written, a.used);
            ran = result.map_err(late("the next write command ends what it left"))?;
        }
    }
    // The alambique is put in place and committed alone before the record commit, which the events it depends on
    // precede (REQ-REG-10).
    if let Some(a) = &planned.alambique {
        let late = &late("the next approval shows what was decided");
        let path = cx.domain.join(alambique::FILE);
        unchanged(a).map_err(late)?;
        // The new events must replay before anything depends on them in a commit (PD-46).
        session.replayable(&PolicyVerifier).map_err(late)?;
        if let Some(bytes) = &a.bytes {
            crate::record::write_atomic(&path, bytes).map_err(late)?;
        }
        session.commit_alone(&path, &a.message).map_err(late)?;
    }
    // The views are generated from the record these events made: the fermento's answer, whose input is checked once
    // more first, and `STATUS.md` and `RULES.md` (REQ-VIEW-3). A change now leaves the events written and the views not.
    let mut views = Vec::new();
    // The new events must replay before a view depends on them in a commit (PD-46); an alambique edit checked it above.
    if planned.alambique.is_none() {
        session.replayable(&PolicyVerifier)?;
    }
    let late_view = late("the next write command regenerates the views");
    let record = crate::record::replay(&cx.domain, &PolicyVerifier).map_err(|mut e| late_view(e.remove(0)))?;
    if let Some(v) = &planned.view {
        let late = late("the next sweep writes the answer");
        let bytes = fermento::compose(&cx.domain, &v.alias, &v.digest, &(v.answer)(&record)).map_err(late)?;
        let path = std::path::PathBuf::from(&v.alias).join("fermento.md");
        crate::record::write_atomic(&cx.domain.join(&path), &bytes)?;
        views.push(path);
    }
    views.extend(view::write(&cx.domain, &record).map_err(late_view)?);
    let commit = session.finish_held(&PolicyVerifier, &views)?;
    caches(cx, session, machine.as_ref(), &used)?;
    Ok(Some((planned.out, Written { events: written, commit, ran, recovered: Vec::new() })))
}

/// The caches a write command keeps, from the policy in force once it wrote, under the lock (REQ-ID-2, REQ-SET-8).
pub(super) fn caches(
    cx: &Cx,
    session: &Session,
    machine: Option<&settings::Settings>,
    used: &[(Space, String)],
) -> Result<(), Failed> {
    crate::record::fault::pause("before-caches");
    let after = session.record().ok_or_else(|| refuse(Code::RecordAbsent, Some(&cx.domain), "no event"))?;
    let policy = settings::in_force(after)?.ok_or_else(|| refuse(Code::PolicyAbsent, Some(&cx.domain), "none"))?;
    identity::write_allowed_signers(&cx.domain, &policy.settings)?;
    let eff = settings::effective(&policy, None, machine, &[], &[])?;
    let mut detection = detect::detect(&cx.domain, &eff, &cx.modules, std::env::var_os("PATH").as_deref());
    // Every write command ran both: Git for the mode's checks or the commit, `ssh-keygen` for the machine key.
    detection.used(Space::Program, "ssh-keygen");
    detection.used(Space::Program, "git");
    for (space, name) in used {
        detection.used(*space, name);
    }
    detection.keep(&cx.domain)?;
    Ok(())
}
