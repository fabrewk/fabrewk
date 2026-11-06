//! Identity and roles (REQ-REG-4's second half, REQ-ID-2 to REQ-ID-5; part 7c2a, D-048, D-050). A person's event is
//! verified against two policies: the one it cites and the one in force at its place. In the `project` stream the
//! place is the stream's own order, and the two are one: the events `apply` writes before the first adoption cite it,
//! the first adoption cites itself, a later one the adoption before it. In any other stream the place comes from the
//! seals: each `policy-adopted` records every other stream's head, so an event beyond an adoption's seal was written
//! after it. The signer must hold, in both policies, the alias's key the signature carries and the role the event
//! claims, and the role must allow the move: in the first core only a maestro makes a person's move (REQ-ID-3).

use crate::code::{Code, Error, refuse};
use crate::record::{self, Class, Event, Replay, Sign, Verify, class_of};
use crate::settings::{self, Level, Settings, Val, key_blob};
use std::collections::BTreeMap;
use std::path::Path;

/// One adoption of the `project` stream: its place, the policy it adopts and its seal.
struct Adopted<'a> {
    event: &'a Event,
    digest: &'a str,
    settings: Settings,
    seal: BTreeMap<&'a str, (u64, &'a str)>,
}

/// Every adoption, in the stream's order. A policy that does not read is refused by the settings' codes.
fn adoptions(record: &Replay) -> Result<Vec<Adopted<'_>>, Vec<Error>> {
    let mut out = Vec::new();
    for event in record.streams.get("project").into_iter().flatten().filter(|e| e.kind == "policy-adopted") {
        // Schema 1's check already holds the payload to its shape and the content to its digest.
        let payload = &event.value["payload"];
        let text = payload["content"].as_str().unwrap_or_default();
        let settings = settings::parse(&event.path, text, Level::Project)?;
        let seal = payload["heads"].as_object().into_iter().flatten();
        let seal =
            seal.map(|(s, h)| (s.as_str(), (h["seq"].as_u64().unwrap_or(0), h["digest"].as_str().unwrap_or(""))));
        let digest = payload["digest"].as_str().unwrap_or_default();
        out.push(Adopted { event, digest, settings, seal: seal.collect() });
    }
    Ok(out)
}

/// The policy in force at an event's place (D-050), `None` when it was written before every adoption.
fn in_force_at<'a, 'b>(adopted: &'b [Adopted<'a>], e: &Event) -> Option<&'b Adopted<'a>> {
    if e.stream == "project" {
        let before = adopted.iter().rfind(|a| a.event.seq < e.seq);
        return before.or(adopted.first());
    }
    adopted.iter().rfind(|a| a.seal.get(e.stream.as_str()).is_none_or(|(seq, _)| *seq < e.seq))
}

/// A maintainer as a policy lists them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Maintainer {
    pub alias: String,
    pub role: String,
    pub key: String,
}

/// Every maintainer a policy's settings list, by alias.
pub fn maintainers(policy: &Settings) -> Vec<Maintainer> {
    let text = |path: String| match policy.values.get(&path).map(|e| &e.value) {
        Some(Val::Str(s)) => s.clone(),
        _ => String::new(),
    };
    let aliases = policy.values.keys().filter_map(|k| k.strip_prefix("maintainers.")?.strip_suffix(".role"));
    aliases
        .map(|a| Maintainer {
            alias: a.to_owned(),
            role: text(format!("maintainers.{a}.role")),
            key: text(format!("maintainers.{a}.key")),
        })
        .collect()
}

fn maintainer(policy: &Settings, alias: &str) -> Option<Maintainer> {
    maintainers(policy).into_iter().find(|m| m.alias == alias)
}

/// The record's verifier: every person's event against its policies, and every seal against its stream.
pub struct PolicyVerifier;

impl Verify for PolicyVerifier {
    fn verify(&self, record: &Replay) -> Result<(), Vec<Error>> {
        let adopted = adoptions(record)?;
        let mut errors = Vec::new();
        for a in &adopted {
            for (stream, (seq, digest)) in &a.seal {
                let held = record.streams.get(*stream).and_then(|s| s.get(*seq as usize - 1));
                if held.map(|e| e.digest.as_str()) != Some(*digest) {
                    let why = format!("the adoption's seal names {stream} at {seq}, which the stream does not hold");
                    errors.push(refuse(Code::RecordRollback, Some(&a.event.path), why));
                }
            }
        }
        for events in record.streams.values() {
            let person = events.iter().filter(|e| class_of(&e.kind) == Some(Class::Person));
            if let Some(e) = person.map(|e| event(&adopted, e)).find_map(Result::err) {
                errors.push(e);
            }
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

fn event(adopted: &[Adopted], e: &Event) -> Result<(), Error> {
    let bad = |why: &str| refuse(Code::RecordSignatureInvalid, Some(&e.path), why.to_owned());
    let cites = e.value["policy"].as_str().unwrap_or_default();
    let cited = adopted.iter().find(|a| a.digest == cites).ok_or_else(|| bad("it cites no adopted policy"))?;
    let place = in_force_at(adopted, e).ok_or_else(|| bad("it was written before any policy was adopted"))?;
    if e.stream == "project" && place.digest != cites {
        return Err(bad("it cites a policy other than the one in force at its place"));
    }
    let (alias, role) = (e.value["actor"]["alias"].as_str(), e.value["actor"]["role"].as_str());
    let mut keys = Vec::new();
    for policy in [cited, place] {
        let m = alias.and_then(|a| maintainer(&policy.settings, a));
        let m = m.ok_or_else(|| bad("its alias is not a maintainer in the policy"))?;
        if Some(m.role.as_str()) != role {
            return Err(bad("its role is not the one the policy gives the alias"));
        }
        keys.push(key_blob(&m.key).ok_or_else(|| bad("the policy's key is not a key"))?);
    }
    if keys[0] != keys[1] {
        return Err(bad("the alias's key differs between the policy it cites and the one in force at its place"));
    }
    if role != Some("maestro") {
        return Err(bad("its role may make no person's move"));
    }
    let bytes = std::fs::read(&e.path).map_err(crate::code::io_at(&e.path))?;
    let mut sig = e.path.clone().into_os_string();
    sig.push(".sig");
    record::check(&keys[0], &bytes, Path::new(&sig))
}

/// The actor of a person's move of `kind` by `alias`, signing with `signer`, under the policy `policy` (REQ-ID-3,
/// REQ-ID-4): the alias must be listed with exactly the signer's key, and its role must allow the move
/// (`ROLE_FORBIDDEN` otherwise). The role comes only from the policy.
pub fn actor(policy: &Settings, alias: &str, signer: &dyn Sign, kind: &str) -> Result<serde_json::Value, Error> {
    let forbidden = |why: String| refuse(Code::RoleForbidden, Some(&policy.path), why);
    if class_of(kind) != Some(Class::Person) {
        return Err(forbidden(format!("{kind} is not a person's move")));
    }
    let signing = key_blob(&signer.public()?);
    let m = maintainer(policy, alias).filter(|m| signing.is_some() && key_blob(&m.key) == signing);
    let m = m.ok_or_else(|| forbidden(format!("the key {alias} signs with holds no role in the policy")))?;
    if m.role != "maestro" {
        return Err(forbidden(format!("a {} may not make {kind}", m.role)));
    }
    Ok(serde_json::json!({"kind": "person", "alias": m.alias, "role": m.role}))
}

/// The key Git's `user.signingkey` names in the repository at `root` (D-055): its public key, and the file to give
/// `ssh-keygen -Y sign`, as Git does: the private key file, or the public key file (whose private key is beside it or
/// in the SSH agent); none for a `key::` literal, whose private key is in the agent.
#[derive(Debug, Clone)]
pub struct GitKey {
    pub public: String,
    pub file: Option<std::path::PathBuf>,
}

/// The key Git signs with in the repository at `root`; `None` when Git names none, or names one that is not a key.
pub fn signing(root: &Path) -> Option<GitKey> {
    let named = record::signing_key(root)?;
    if let Some(literal) = named.strip_prefix("key::") {
        return key_blob(literal.trim()).is_some().then(|| GitKey { public: literal.trim().to_owned(), file: None });
    }
    // Only a regular file is opened, so that a pipe never blocks a command (P7D2-17), and never one in a domain or in
    // this repository, however it is reached: a key is no maintainer's cited material. That holds for every file
    // `ssh-keygen -Y sign` may open too (P7D3A-4): the file named, and, for a public key file, the private key it
    // looks for beside it, its name without `.pub`.
    let inside = |p: &Path| crate::code::in_domain(p, root);
    let acceptable = |p: &Path| !inside(p) && std::fs::metadata(p).is_ok_and(|m| m.is_file());
    let absent = |p: &Path| std::fs::symlink_metadata(p).is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound);
    let public_in = |p: &Path| {
        acceptable(p).then(|| std::fs::read_to_string(p).ok()).flatten().filter(|t| key_blob(t.trim()).is_some())
    };
    let path = root.join(&named);
    let with_pub = |p: &Path| {
        let mut s = p.as_os_str().to_owned();
        s.push(".pub");
        std::path::PathBuf::from(s)
    };
    let public = match public_in(&path) {
        // The public key file: its private key, if beside it, must be acceptable too, or absent (in the agent).
        Some(public) => {
            let private = path.to_str().and_then(|p| p.strip_suffix(".pub")).map(Path::new);
            if private.is_some_and(|p| !absent(p) && !acceptable(p)) {
                return None;
            }
            public
        }
        // The private key file, acceptable, with its public key beside it.
        None if acceptable(&path) => public_in(&with_pub(&path))?,
        None => return None,
    };
    Some(GitKey { public: public.trim().to_owned(), file: Some(path) })
}

/// The maintainer of `policy` who runs this command (D-055): the one whose key is the public key Git's
/// `user.signingkey` names in the repository at `root`, a `key::` literal, a public key file, or a private key file
/// with its `.pub` beside it. `None` when Git names none or no maintainer holds it.
pub fn person(policy: &Settings, root: &Path) -> Option<String> {
    holder(policy, &signing(root)?.public)
}

/// The maintainer of `policy` whose key is `public`.
pub fn holder(policy: &Settings, public: &str) -> Option<String> {
    let blob = key_blob(public);
    maintainers(policy).into_iter().find(|m| blob.is_some() && key_blob(&m.key) == blob).map(|m| m.alias)
}

/// Signs as Git signs with its signing key (D-055 point 1, D-062): `ssh-keygen -Y sign -n fabrewk -f <file>`; for a
/// `key::` literal the public key is written to `tmp` first, in the domain's unversioned area, and removed after.
pub struct GitSigner {
    pub key: GitKey,
    pub tmp: std::path::PathBuf,
}

impl Sign for GitSigner {
    fn sign(&self, bytes: &[u8]) -> Result<Vec<u8>, Error> {
        let made = self.key.file.is_none() && self.tmp.parent().is_some_and(|d| !d.exists());
        let file = match &self.key.file {
            Some(f) => f.clone(),
            None => {
                let dir = self.tmp.parent().unwrap_or(&self.tmp);
                std::fs::create_dir_all(dir).map_err(crate::code::io_at(dir))?;
                record::write_atomic(&self.tmp, format!("{}\n", self.key.public).as_bytes())?;
                self.tmp.clone()
            }
        };
        let signed = record::sign_with(&file, bytes);
        if self.key.file.is_none() {
            let _ = std::fs::remove_file(&self.tmp);
            // The directory too, when this made it: a refused command leaves the domain as it was (P7D3A-1).
            if made {
                let _ = std::fs::remove_dir(self.tmp.parent().unwrap_or(&self.tmp));
            }
        }
        signed
    }

    fn public(&self) -> Result<String, Error> {
        Ok(self.key.public.clone())
    }
}

/// The allowed-signers file derived from a policy (REQ-ID-2), one line per maintainer, for `ssh-keygen` and Git. It is
/// a cache: Fabrewk never verifies with it (D-048), so a line added by hand verifies nothing.
pub fn allowed_signers(policy: &Settings) -> String {
    let line = |m: Maintainer| {
        let key: Vec<&str> = m.key.split(' ').filter(|p| !p.is_empty()).take(2).collect();
        format!("{} namespaces=\"fabrewk,git\" {}\n", m.alias, key.join(" "))
    };
    maintainers(policy).into_iter().map(line).collect()
}

/// Replaces `local/cache/allowed_signers` under `domain` with a fresh derivation; only write commands call it.
pub fn write_allowed_signers(domain: &Path, policy: &Settings) -> Result<(), Error> {
    let cache = domain.join("local/cache");
    std::fs::create_dir_all(&cache).map_err(crate::code::io_at(&cache))?;
    record::write_atomic(&cache.join("allowed_signers"), allowed_signers(policy).as_bytes())
}

/// What `config` reports of a maintainer's key (REQ-ID-5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyReport {
    pub alias: String,
    /// The key's type, as its public key names it.
    pub kind: String,
    /// Always `unknown`: no verification Fabrewk makes requires a touch (`check-novalidate` reads no options, D-048),
    /// and an agent's confirmation setting cannot be seen.
    pub touch: &'static str,
}

/// Each maintainer's key type and whether a touch is enforced.
pub fn key_reports(policy: &Settings) -> Vec<KeyReport> {
    let report = |m: Maintainer| KeyReport {
        kind: m.key.split(' ').find(|p| !p.is_empty()).unwrap_or_default().to_owned(),
        alias: m.alias,
        touch: "unknown",
    };
    maintainers(policy).into_iter().map(report).collect()
}
