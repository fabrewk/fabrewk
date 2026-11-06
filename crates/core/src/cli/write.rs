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
use crate::record::{Draft, Head, Publication, Replay, Repo, Say, Session};
use crate::settings::{self, Policy, Val};
use crate::{domain, machine};
use std::collections::BTreeMap;

/// The core's own marker: the environment of a process an extension runs (REQ-SEC-7).
pub const EXTENSION_MARKER: &str = "FABREWK_EXTENSION";

/// What a write command plans from a record: the policy in force and the publication mode.
pub(super) struct State {
    pub policy: Policy,
    pub mode: Publication,
}

/// The policy in force (`POLICY_ABSENT` with none) and the publication mode `apply` records with it.
pub(super) fn state(cx: &Cx, record: &Replay) -> Result<State, Failed> {
    let policy = settings::in_force(record)?;
    let policy = policy.ok_or_else(|| refuse(Code::PolicyAbsent, Some(&cx.domain), "no policy is adopted"))?;
    let mode = domain::publication(record);
    let mode = mode.ok_or_else(|| {
        refuse(Code::RecordBroken, Some(&cx.domain), "the project stream records no publication mode")
    })?;
    Ok(State { policy, mode })
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
    let key = identity::signing(&cx.root)
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
        None => {
            let personal = cx.personal_of(alias)?;
            let eff = settings::effective(policy, personal.as_ref(), None, &[], &[])?;
            match eff.values.get("languages.agentInteraction") {
                Some((Val::Str(s), _)) => s.clone(),
                _ => "en".to_owned(),
            }
        }
    };
    Ok(Say { text, lang })
}

/// What a command's guards allow it to write: its events in order, by stream, the signer of the person's among them,
/// whether the first is an adoption whose seal the session gives, and what the command reports.
pub(super) struct Plan<T> {
    pub events: Vec<(String, Draft)>,
    pub signer: Option<GitSigner>,
    pub sealed: bool,
    pub out: T,
}

/// What a write command wrote: each event's stream and head, and the record commit, if the mode commits.
pub(super) struct Written {
    pub events: Vec<(String, Head)>,
    pub commit: Option<String>,
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
    let record = cx.record.as_ref().map_err(|e| Failed::from(e.clone()))?;
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
    // A lock a stopped Git left is refused before anything is written (P7D3A-7); the commit checks it again.
    if let Some(repo) = Repo::for_domain(&cx.domain, mode) {
        crate::record::check_git_locks(&repo, &crate::record::record_branch(&repo)?)?;
    }
    let folder = cx.folder()?;
    // The private folder and the machine file are read before anything is written, as every write uses them.
    let machine = machine::machine_file(&folder)?;
    plan(record)?;
    let mut session = Session::begin(&cx.domain, Repo::for_domain(&cx.domain, mode), &PolicyVerifier)?;
    session.refresh(&PolicyVerifier)?;
    let record = session.record().ok_or_else(|| refuse(Code::RecordAbsent, Some(&cx.domain), "no event"))?;
    let planned = plan(record)?;
    let mut heads: BTreeMap<String, Option<Head>> =
        planned.events.iter().map(|(s, _)| (s.clone(), head_in(record, s))).collect();
    // The machine key is made under a temporary name and put in place only once every event is signed, so that a key
    // that cannot sign leaves the private folder as it was (P7D3A-1).
    let (instance, pending) = machine::prepare_instance(&folder)?;
    let mut prepared = Vec::new();
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
        let p = session.prepare(&stream, expected.as_ref(), &draft, signer)?;
        heads.insert(stream.clone(), Some(p.head().clone()));
        prepared.push((stream, p));
    }
    if let Some(pending) = pending {
        pending.install()?;
    }
    session.commit_uncommitted(&PolicyVerifier)?;
    crate::record::fault::pause("before-append");
    let mut written = Vec::new();
    for (stream, p) in prepared {
        written.push((stream, session.write(p)?));
    }
    let commit = session.finish_held(&PolicyVerifier, &[])?;
    crate::record::fault::pause("before-caches");
    // The caches a write command keeps, from the policy in force once it wrote, under the lock (REQ-ID-2, REQ-SET-8).
    let after = session.record().ok_or_else(|| refuse(Code::RecordAbsent, Some(&cx.domain), "no event"))?;
    let policy = settings::in_force(after)?.ok_or_else(|| refuse(Code::PolicyAbsent, Some(&cx.domain), "none"))?;
    identity::write_allowed_signers(&cx.domain, &policy.settings)?;
    let eff = settings::effective(&policy, None, machine.as_ref(), &[], &[])?;
    let mut detection = detect::detect(&cx.domain, &eff, cx.modules, std::env::var_os("PATH").as_deref());
    // Every write command ran both: Git for the mode's checks or the commit, `ssh-keygen` for the machine key.
    detection.used(Space::Program, "ssh-keygen");
    detection.used(Space::Program, "git");
    detection.keep(&cx.domain)?;
    drop(session);
    Ok((planned.out, Written { events: written, commit }))
}
