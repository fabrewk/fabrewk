//! Identity on a fixture record (part 7c2a, D-048, D-050): signers derived from the policy each event cites and from the
//! policy in force at its place, the seal of every head at adoption, the role against the move, the derived
//! allowed-signers file, the key report, the private folder and the machine key. Fixtures as in `record.rs`
//! (core-specification 11.2 rule 2): every key and home is the test's own.

mod common;

use common::*;
use fabrewk_core::identity::{self, PolicyVerifier};
use fabrewk_core::machine;
use fabrewk_core::record::{Code, Draft, Head, KINDS, KeySigner, Replay, carried_key, class_of, replay, sha256_hex};
use fabrewk_core::settings::{self, Level};
use serde_json::{Value, json};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

fn public(f: &Fixture, who: &str) -> String {
    fs::read_to_string(f.keys.join(format!("{who}.pub"))).unwrap().trim().to_owned()
}

/// A project file listing `xel` as maestro, and `stranger` under `ana` with `ana_role` when given.
fn policy_text(f: &Fixture, ana_role: Option<&str>, extra: &str) -> String {
    let mut text = format!(
        "[project]\nname = \"fixture\"\n\n[maintainers.xel]\nrole = \"maestro\"\nkey = \"{}\"\n",
        public(f, "xel")
    );
    if let Some(role) = ana_role {
        text += &format!("\n[maintainers.ana]\nrole = \"{role}\"\nkey = \"{}\"\n", public(f, "stranger"));
    }
    text + extra
}

/// A move of `alias` with `role`, citing `policy`.
fn moved(kind: &str, alias: &str, role: &str, policy: &str, payload: Value) -> Draft {
    let mut d = person(kind, "Lo decido.", payload);
    d.actor = json!({"kind": "person", "alias": alias, "role": role});
    d.policy = policy.into();
    d
}

fn ok(f: &Fixture) -> Replay {
    replay(&f.domain, &PolicyVerifier).unwrap()
}

fn refused(f: &Fixture) -> Vec<Code> {
    codes(replay(&f.domain, &PolicyVerifier))
}

/// Writes `text` as the project file and adopts it as `xel` with the library, as `apply` (the first) or `adopt` would;
/// gives the adopted digest.
fn adopt(f: &Fixture, text: &str, widen: &[&str]) -> String {
    fs::write(f.domain.join("settings.toml"), text).unwrap();
    let mut s = begin_with(f, None, &PolicyVerifier).unwrap();
    let in_force = s.record().map(|r| settings::in_force(r).unwrap()).unwrap_or(None);
    let widen: Vec<String> = widen.iter().map(|w| w.to_string()).collect();
    let adoption = settings::adopt(&f.domain, in_force.as_ref(), &widen, None).unwrap();
    let cites = in_force.map_or(adoption.digest.clone(), |p| p.digest);
    let draft = moved("policy-adopted", "xel", "maestro", &cites, adoption.payload(&s.seal().unwrap()));
    let head = s.head("project").unwrap();
    s.append("project", head.as_ref(), &draft, Some(&f.signer())).unwrap();
    s.finish(&PolicyVerifier, &[]).unwrap();
    adoption.digest
}

/// Opens a task stream with its creation (an agent's), giving the stream and its head.
fn task(f: &Fixture) -> (String, Head) {
    let mut s = begin_with(f, None, &PolicyVerifier).unwrap();
    let (stream, head) = stage_and_task(&mut s);
    s.finish(&PolicyVerifier, &[]).unwrap();
    (stream, head)
}

/// The library's append of a person's move signed by `signer`.
fn append(f: &Fixture, stream: &str, draft: &Draft, signer: &KeySigner) -> Head {
    let mut s = begin_with(f, None, &PolicyVerifier).unwrap();
    let head = s.head(stream).unwrap();
    let h = s.append(stream, head.as_ref(), draft, Some(signer)).unwrap();
    s.finish(&PolicyVerifier, &[]).unwrap();
    h
}

// ---- REQ-REG-4 and D-050 --------------------------------------------------------------------------------------------

#[test]
fn req_reg_4_apply_cites_the_first_adoption() {
    let f = Fixture::new("apply-cites", false);
    let text = policy_text(&f, None, "");
    fs::write(f.domain.join("settings.toml"), &text).unwrap();
    let digest = sha256_hex(text.as_bytes());
    let adoption = settings::adopt(&f.domain, None, &[], None).unwrap();
    // The plan `apply` records creates the project file; before the first adoption its events are verified against
    // that file (P10O-5): a citation of another digest cites no adopted policy.
    let plan = json!({
        "answers": {"publication": "published"},
        "files": [{"path": ".brewstillery/settings.toml", "action": "create", "content": text}],
    });
    let planned = assessed(plan);
    let mut s = begin_with(&f, None, &PolicyVerifier).unwrap();
    let mut head: Option<fabrewk_core::record::Head> = None;
    for (kind, payload) in [
        ("assessed", planned.clone()),
        ("publication-decided", project_payload("publication-decided")),
        ("policy-adopted", adoption.payload(&s.seal().unwrap())),
    ] {
        let draft = moved(kind, "xel", "maestro", &digest, payload);
        if kind != "assessed" {
            s.finish(&PolicyVerifier, &[]).unwrap();
            assert_eq!(ok(&f).streams["project"].len(), if kind == "policy-adopted" { 2 } else { 1 });
            if kind == "publication-decided" {
                let other = moved(kind, "xel", "maestro", &"0".repeat(64), project_payload(kind));
                let path = f.forge("project", 2, Some(&head.as_ref().unwrap().digest), &other, Some(&f.signer()));
                let got =
                    replay(&f.domain, &PolicyVerifier).err().map(|e| e.iter().map(|e| e.code()).collect::<Vec<_>>());
                assert_eq!(got, Some(vec![Some(Code::RecordSignatureInvalid)]));
                fs::remove_file(sig(&path)).unwrap();
                fs::remove_file(&path).unwrap();
            }
            s = begin_with(&f, None, &PolicyVerifier).unwrap();
        }
        head = Some(s.append("project", head.as_ref(), &draft, Some(&f.signer())).unwrap());
    }
    let receipt = json!({"plan": planned["digest"], "files": []});
    s.append("project", head.as_ref(), &fabrewk("installed", receipt), None).unwrap();
    let baseline = moved("alambique-baseline", "xel", "maestro", &digest, json!({"digest": digest}));
    s.append("alambique", None, &baseline, Some(&f.signer())).unwrap();
    s.finish(&PolicyVerifier, &[]).unwrap();
    let r = ok(&f);
    assert_eq!(r.streams["project"].len(), 4);
    // Each person's event of the project stream cites the policy in force at its place, and nothing else does.
    let second = policy_text(&f, None, "[limits]\nrounds = 2\n");
    let second_digest = adopt(&f, &second, &[]);
    for (kind, cites, expected) in [
        ("publication-decided", &second_digest, vec![]),
        ("publication-decided", &digest, vec![Code::RecordSignatureInvalid]),
        ("policy-adopted", &second_digest, vec![]),
        ("policy-adopted", &digest, vec![Code::RecordSignatureInvalid]),
    ] {
        let before = f.events("project");
        let last = before.last().unwrap();
        let prev = sha256_hex(&fs::read(last).unwrap());
        let payload =
            if kind == "policy-adopted" { adoption.payload(&Default::default()) } else { project_payload(kind) };
        let draft = moved(kind, "xel", "maestro", cites, payload);
        let path = f.forge("project", before.len() as u64 + 1, Some(&prev), &draft, Some(&f.signer()));
        let got =
            replay(&f.domain, &PolicyVerifier).err().map(|e| e.iter().map(|e| e.code().unwrap()).collect::<Vec<_>>());
        assert_eq!(got.unwrap_or_default(), expected, "{kind} citing {cites}");
        fs::remove_file(sig(&path)).unwrap();
        fs::remove_file(&path).unwrap();
    }
}

/// Before the first adoption, only a record that holds the `project` stream an `apply` opened, and no other, is verified
/// against its plan's project file (P10O-5): with an event of another stream, each stream's events cite no adopted
/// policy.
#[test]
fn req_reg_4_before_the_first_adoption_only_apply_s_stream_is_verified_against_its_plan() {
    let f = Fixture::new("apply-before-adoption", false);
    let text = policy_text(&f, None, "");
    let digest = sha256_hex(text.as_bytes());
    let planned = assessed(json!({
        "answers": {"publication": "published"},
        "files": [{"path": ".brewstillery/settings.toml", "action": "create", "content": text}],
    }));
    let mut prev: Option<String> = None;
    for (seq, kind, payload) in
        [(1, "assessed", planned), (2, "publication-decided", project_payload("publication-decided"))]
    {
        let draft = moved(kind, "xel", "maestro", &digest, payload);
        let path = f.forge("project", seq, prev.as_deref(), &draft, Some(&f.signer()));
        prev = Some(sha256_hex(&fs::read(path).unwrap()));
    }
    ok(&f);
    let baseline = moved("alambique-baseline", "xel", "maestro", &digest, json!({"digest": digest}));
    f.forge("alambique", 1, None, &baseline, Some(&f.signer()));
    assert_eq!(refused(&f), [Code::RecordSignatureInvalid; 2]);
}

#[test]
fn refusal_record_signature_invalid_against_the_policy() {
    let f = Fixture::new("signature-policy", false);
    let digest = adopt(&f, &policy_text(&f, Some("contributor"), ""), &[]);
    let (stream, head) = task(&f);
    let check = |draft: &Draft, signer: &KeySigner, namespace: &str, over: Option<&[u8]>| {
        let path = f.forge(&stream, 2, Some(&head.digest), draft, None);
        let bytes = over.map(<[u8]>::to_vec).unwrap_or_else(|| fs::read(&path).unwrap());
        fs::write(sig(&path), sign_with(signer, &bytes, namespace)).unwrap();
        let got = refused(&f);
        fs::remove_file(sig(&path)).unwrap();
        fs::remove_file(&path).unwrap();
        got
    };
    let invalid = vec![Code::RecordSignatureInvalid];
    let xel = moved("task-authorized", "xel", "maestro", &digest, bound(&f, &stream));
    // The maestro's move, signed by its key in the namespace `fabrewk`, verifies.
    let path = f.forge(&stream, 2, Some(&head.digest), &xel, Some(&f.signer()));
    ok(&f);
    fs::remove_file(sig(&path)).unwrap();
    fs::remove_file(&path).unwrap();
    // Signed by a key absent from the cited policy; over other bytes; in another namespace.
    assert_eq!(check(&xel, &f.stranger(), "fabrewk", None), invalid);
    assert_eq!(check(&xel, &f.signer(), "fabrewk", Some(b"other")), invalid);
    assert_eq!(check(&xel, &f.signer(), "git", None), invalid);
    // Citing a policy no adoption holds; an alias the policy does not list; a role the policy does not give.
    let unknown = moved("task-authorized", "xel", "maestro", &"0".repeat(64), bound(&f, &stream));
    assert_eq!(check(&unknown, &f.signer(), "fabrewk", None), invalid);
    let nobody = moved("task-authorized", "eva", "maestro", &digest, bound(&f, &stream));
    assert_eq!(check(&nobody, &f.signer(), "fabrewk", None), invalid);
    let claimed = moved("task-authorized", "ana", "maestro", &digest, bound(&f, &stream));
    assert_eq!(check(&claimed, &f.stranger(), "fabrewk", None), invalid);
    // A contributor's move, signed by the contributor's own key, is no move (REQ-ID-3).
    let contributor = moved("task-authorized", "ana", "contributor", &digest, bound(&f, &stream));
    assert_eq!(check(&contributor, &f.stranger(), "fabrewk", None), invalid);
    // A key added by hand to the derived allowed-signers file verifies nothing (REQ-ID-2).
    let policy = settings::in_force(&ok(&f)).unwrap().unwrap();
    identity::write_allowed_signers(&f.domain, &policy.settings).unwrap();
    let file = f.domain.join("local/cache/allowed_signers");
    let mut text = fs::read_to_string(&file).unwrap();
    text += &format!("eva namespaces=\"fabrewk\" {}\n", public(&f, "stranger"));
    fs::write(&file, text).unwrap();
    let eva = moved("task-authorized", "eva", "maestro", &digest, bound(&f, &stream));
    assert_eq!(check(&eva, &f.stranger(), "fabrewk", None), invalid);
}

#[test]
fn req_reg_4_a_removed_maestro_signs_nothing_after_the_adoption() {
    let f = Fixture::new("revocation", false);
    let first = adopt(&f, &policy_text(&f, Some("maestro"), ""), &[]);
    let (stream, _) = task(&f);
    let ana = moved("task-authorized", "ana", "maestro", &first, bound(&f, &stream));
    let h2 = append(&f, &stream, &ana, &f.stranger());
    // Later moves of the task: from `authorized`, a run starts.
    let ana = moved("run-started", "ana", "maestro", &first, started(&f, &stream));
    // A second adoption removes `ana`; its seal holds the task's head.
    let second = adopt(&f, &policy_text(&f, None, ""), &["maintainers.ana"]);
    let last = ok(&f).streams["project"].last().unwrap().value.clone();
    assert_eq!(last["payload"]["heads"][&stream], json!({"digest": h2.digest, "seq": 2}));
    assert_eq!(last["policy"], json!(first));
    // `ana`'s move written before the adoption still verifies; one written after it, citing the old policy, does not.
    ok(&f);
    let forged = f.forge(&stream, 3, Some(&h2.digest), &ana, Some(&f.stranger()));
    assert_eq!(refused(&f), [Code::RecordSignatureInvalid]);
    fs::remove_file(sig(&forged)).unwrap();
    fs::remove_file(&forged).unwrap();
    // The concurrent work of a maestro who keeps their key and role is accepted, whichever policy it cites.
    for cites in [&first, &second] {
        let xel = moved("run-started", "xel", "maestro", cites, started(&f, &stream));
        let path = f.forge(&stream, 3, Some(&h2.digest), &xel, Some(&f.signer()));
        ok(&f);
        fs::remove_file(sig(&path)).unwrap();
        fs::remove_file(&path).unwrap();
    }
    // A stream created after the adoption is beyond its seal too.
    let (other, oh) = task(&f);
    let ana = moved("task-authorized", "ana", "maestro", &first, bound(&f, &other));
    let forged = f.forge(&other, 2, Some(&oh.digest), &ana, Some(&f.stranger()));
    assert_eq!(refused(&f), [Code::RecordSignatureInvalid]);
    fs::remove_file(sig(&forged)).unwrap();
    fs::remove_file(&forged).unwrap();
    // A maestro whose key changes signs with the new key only after the adoption.
    let mut text = policy_text(&f, None, "");
    text = text.replace(&public(&f, "xel"), &public(&f, "stranger"));
    let third_digest = sha256_hex(text.as_bytes());
    adopt(&f, &text, &["maintainers.xel.key"]);
    let xel = moved("run-started", "xel", "maestro", &second, started(&f, &stream));
    let h = ok(&f).streams[&stream].last().unwrap().digest.clone();
    let path = f.forge(&stream, 3, Some(&h), &xel, Some(&f.signer()));
    assert_eq!(refused(&f), [Code::RecordSignatureInvalid]);
    fs::remove_file(sig(&path)).unwrap();
    fs::remove_file(&path).unwrap();
    let xel = moved("run-started", "xel", "maestro", &third_digest, started(&f, &stream));
    let path = f.forge(&stream, 3, Some(&h), &xel, Some(&f.stranger()));
    ok(&f);
    fs::remove_file(sig(&path)).unwrap();
    fs::remove_file(&path).unwrap();
}

#[test]
fn req_reg_4_a_seal_its_stream_does_not_hold_is_a_rollback() {
    let f = Fixture::new("seal-rollback", false);
    let first = adopt(&f, &policy_text(&f, None, ""), &[]);
    let (stream, _) = task(&f);
    let xel = moved("task-authorized", "xel", "maestro", &first, bound(&f, &stream));
    let h2 = append(&f, &stream, &xel, &f.signer());
    adopt(&f, &policy_text(&f, None, "[limits]\nrounds = 2\n"), &[]);
    ok(&f);
    // The stream cut below its seal: the cut event's file removed.
    let events = f.events(&stream);
    let (cut, cut_sig) = (fs::read(&events[1]).unwrap(), fs::read(sig(&events[1])).unwrap());
    fs::remove_file(&events[1]).unwrap();
    fs::remove_file(sig(&events[1])).unwrap();
    assert_eq!(refused(&f), [Code::RecordRollback]);
    // Rewritten at the sealed position, with another valid event.
    let mut other = moved("task-authorized", "xel", "maestro", &first, bound(&f, &stream));
    other.say = Some(fabrewk_core::record::Say { text: "Otras palabras.".into(), lang: "es-MX".into() });
    let path = f.forge(&stream, 2, Some(&ok_head(&f, &stream, 1)), &other, Some(&f.signer()));
    assert_eq!(refused(&f), [Code::RecordRollback]);
    fs::remove_file(sig(&path)).unwrap();
    fs::remove_file(&path).unwrap();
    fs::write(&events[1], cut).unwrap();
    fs::write(sig(&events[1]), cut_sig).unwrap();
    assert_eq!(ok(&f).streams[&stream][1].digest, h2.digest);
}

/// The digest of the event at `seq` of `stream`, read from its file.
fn ok_head(f: &Fixture, stream: &str, seq: usize) -> String {
    sha256_hex(&fs::read(&f.events(stream)[seq - 1]).unwrap())
}

// ---- REQ-ID-2 to REQ-ID-5 -------------------------------------------------------------------------------------------

#[test]
fn refusal_role_forbidden() {
    let f = Fixture::new("role-forbidden", false);
    let text = policy_text(&f, Some("contributor"), "");
    let policy = settings::parse(&f.domain.join("settings.toml"), &text, Level::Project).unwrap();
    for kind in KINDS.iter().map(|k| k.0).filter(|k| class_of(k) == Some(fabrewk_core::record::Class::Person)) {
        let actor = identity::actor(&policy, "xel", &f.signer(), kind).unwrap();
        assert_eq!(actor, json!({"kind": "person", "alias": "xel", "role": "maestro"}), "{kind}");
        // A contributor makes no move; a key with no role makes none either.
        for (alias, signer) in [("ana", f.stranger()), ("xel", f.stranger()), ("ana", f.signer()), ("eva", f.signer())]
        {
            let e = identity::actor(&policy, alias, &signer, kind).unwrap_err();
            assert_eq!(e.code(), Some(Code::RoleForbidden), "{kind} by {alias}");
        }
    }
    let e = identity::actor(&policy, "xel", &f.signer(), "installed").unwrap_err();
    assert_eq!(e.code(), Some(Code::RoleForbidden));
}

#[test]
fn req_id_2_allowed_signers_is_derived_from_the_policy_in_force() {
    let f = Fixture::new("allowed-signers", false);
    let first = adopt(&f, &policy_text(&f, Some("contributor"), ""), &[]);
    let policy = settings::in_force(&ok(&f)).unwrap().unwrap();
    identity::write_allowed_signers(&f.domain, &policy.settings).unwrap();
    let file = f.domain.join("local/cache/allowed_signers");
    let derived = fs::read_to_string(&file).unwrap();
    assert_eq!(derived, identity::allowed_signers(&policy.settings));
    let line = |alias: &str, who: &str| {
        let key: Vec<String> = public(&f, who).split(' ').take(2).map(str::to_owned).collect();
        format!("{alias} namespaces=\"fabrewk,git\" {}\n", key.join(" "))
    };
    assert_eq!(derived, line("ana", "stranger") + &line("xel", "xel"));
    // `ssh-keygen -Y verify` accepts the maestro's events with it, as Git would.
    let (stream, head) = task(&f);
    append(&f, &stream, &moved("task-authorized", "xel", "maestro", &first, bound(&f, &stream)), &f.signer());
    let event = &ok(&f).streams[&stream][1];
    let out = std::process::Command::new("ssh-keygen")
        .args(["-Y", "verify", "-n", "fabrewk", "-I", "xel", "-f"])
        .arg(&file)
        .arg("-s")
        .arg(sig(&event.path))
        .stdin(fs::File::open(&event.path).unwrap())
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(event.value["prev"], json!(head.digest));
    // After an adoption that removes `ana`, a fresh derivation lists only `xel`.
    adopt(&f, &policy_text(&f, None, ""), &["maintainers.ana"]);
    let policy = settings::in_force(&ok(&f)).unwrap().unwrap();
    identity::write_allowed_signers(&f.domain, &policy.settings).unwrap();
    assert_eq!(fs::read_to_string(&file).unwrap(), line("xel", "xel"));
}

#[test]
fn req_id_4_roles_come_only_from_the_policy() {
    isolated("req_id_4_roles_come_only_from_the_policy", || {
        let f = Fixture::new("roles-policy", false);
        let first = adopt(&f, &policy_text(&f, None, ""), &[]);
        let (stream, _) = task(&f);
        append(&f, &stream, &moved("task-authorized", "xel", "maestro", &first, bound(&f, &stream)), &f.signer());
        let before = ok(&f).streams[&stream][1].value.clone();
        assert_eq!(before["actor"], json!({"kind": "person", "alias": "xel", "role": "maestro"}));
        // The file on disk names `xel` a contributor, not adopted; a personal file and a private folder appear.
        fs::write(f.domain.join("settings.toml"), policy_text(&f, None, "").replace("maestro", "contributor")).unwrap();
        fs::create_dir_all(f.domain.join("xel")).unwrap();
        fs::write(f.domain.join("xel/settings.toml"), "[maintainer]\nalias = \"xel\"\n").unwrap();
        let private = f.root.parent().unwrap().join("private/fabrewk");
        machine::ensure_instance(&private).unwrap();
        fs::write(private.join("machine.toml"), "[maintainers.xel]\nrole = \"contributor\"\n").unwrap();
        assert_eq!(ok(&f).streams[&stream][1].value, before);
        let policy = settings::in_force(&ok(&f)).unwrap().unwrap();
        assert!(identity::actor(&policy.settings, "xel", &f.signer(), "task-accepted").is_ok());
    });
}

#[test]
fn req_id_5_key_type_and_touch() {
    let ssh_string = |b: &[u8]| [&(b.len() as u32).to_be_bytes()[..], b].concat();
    let encode = |bytes: &[u8]| {
        const D: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::new();
        for c in bytes.chunks(3) {
            let n = c.iter().fold(0u32, |a, b| a << 8 | u32::from(*b)) << (8 * (3 - c.len()));
            for i in 0..4 {
                out.push(if i <= c.len() { char::from(D[(n >> (18 - 6 * i) & 63) as usize]) } else { '=' });
            }
        }
        out
    };
    let sk_ed = [ssh_string(b"sk-ssh-ed25519@openssh.com"), ssh_string(&[7; 32]), ssh_string(b"ssh:")].concat();
    let sk_ec = [
        ssh_string(b"sk-ecdsa-sha2-nistp256@openssh.com"),
        ssh_string(b"nistp256"),
        ssh_string(&[4; 65]),
        ssh_string(b"ssh:"),
    ]
    .concat();
    let f = Fixture::new("key-report", false);
    let text = format!(
        "[project]\nname = \"f\"\n\n[maintainers.xel]\nrole = \"maestro\"\nkey = \"{}\"\n\n[maintainers.ana]\nrole = \"contributor\"\nkey = \"sk-ssh-ed25519@openssh.com {}\"\n\n[maintainers.eva]\nrole = \"contributor\"\nkey = \"sk-ecdsa-sha2-nistp256@openssh.com {} eva@host\"\n",
        public(&f, "xel"),
        encode(&sk_ed),
        encode(&sk_ec)
    );
    let policy = settings::parse(Path::new("settings.toml"), &text, Level::Project).unwrap();
    let reports: Vec<(String, String, &str)> =
        identity::key_reports(&policy).into_iter().map(|r| (r.alias, r.kind, r.touch)).collect();
    assert_eq!(
        reports,
        [
            ("ana".into(), "sk-ssh-ed25519@openssh.com".into(), "unknown"),
            ("eva".into(), "sk-ecdsa-sha2-nistp256@openssh.com".into(), "unknown"),
            ("xel".into(), "ssh-ed25519".into(), "unknown"),
        ]
    );
}

// ---- REQ-REG-12 and REQ-SEC-4 ---------------------------------------------------------------------------------------

/// Every file and directory under `dir` with its mode and content, to compare before and after.
fn tree(dir: &Path) -> Vec<(String, u32, Vec<u8>)> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else { return out };
    for entry in entries {
        let path = entry.unwrap().path();
        let mode = fs::metadata(&path).unwrap().permissions().mode();
        let content = if path.is_file() { fs::read(&path).unwrap() } else { Vec::new() };
        out.push((path.display().to_string(), mode, content));
        out.extend(tree(&path));
    }
    out.sort();
    out
}

#[test]
fn req_reg_12_the_first_write_makes_the_machine_key() {
    isolated("req_reg_12_the_first_write_makes_the_machine_key", || {
        let folder = machine::private_folder(|v| std::env::var_os(v)).unwrap();
        assert_eq!(folder, Path::new(&std::env::var_os("XDG_CONFIG_HOME").unwrap()).join("fabrewk"));
        // A reading command finds no instance and creates nothing.
        assert_eq!(machine::instance(&folder).unwrap(), None);
        assert!(!folder.exists());
        let fp = machine::ensure_instance(&folder).unwrap();
        assert_eq!(fs::metadata(&folder).unwrap().permissions().mode() & 0o777, 0o700);
        // An ed25519 key with no passphrase, named by the fingerprint `ssh-keygen -l` prints.
        let public = fs::read_to_string(folder.join("instance.pub")).unwrap();
        assert!(public.starts_with("ssh-ed25519 "));
        let derived = run(&folder, "ssh-keygen", &["-y", "-P", "", "-f", "instance"]);
        assert_eq!(derived.split(' ').take(2).collect::<Vec<_>>(), public.split(' ').take(2).collect::<Vec<_>>());
        let listed = run(&folder, "ssh-keygen", &["-l", "-E", "sha256", "-f", "instance.pub"]);
        assert_eq!(listed.split(' ').nth(1), Some(fp.as_str()));
        // Reading again and writing again give the same instance, and reading changes nothing.
        let before = tree(&folder);
        assert_eq!(machine::instance(&folder).unwrap().as_deref(), Some(fp.as_str()));
        assert_eq!(tree(&folder), before);
        assert_eq!(machine::ensure_instance(&folder).unwrap(), fp);
        assert_eq!(tree(&folder), before);
        // An event records it as its instance.
        let f = Fixture::new("instance", false);
        let mut s = begin(&f, None);
        let mut draft = note(json!({}));
        draft.instance = fp.clone();
        s.append("ext-x", None, &draft, None).unwrap();
        s.finish(&f.verifier(), &[]).unwrap();
        assert_eq!(replay(&f.domain, &f.verifier()).unwrap().streams["ext-x"][0].value["instance"], json!(fp));
        // The fingerprint is the private key's, whatever the public file beside it says (P7C2A-2).
        fs::write(folder.join("instance.pub"), "not a key\n").unwrap();
        assert_eq!(machine::instance(&folder).unwrap().as_deref(), Some(fp.as_str()));
        fs::copy(f.keys.join("stranger.pub"), folder.join("instance.pub")).unwrap();
        assert_eq!(machine::ensure_instance(&folder).unwrap(), fp);
        // A key half made by a stopped command (the private key not yet in place) is made again, its leftovers
        // removed; a private key that does not read is none.
        fs::remove_file(folder.join("instance")).unwrap();
        fs::write(folder.join(".instance-new"), "partial").unwrap();
        assert_eq!(machine::instance(&folder).unwrap(), None);
        let again = machine::ensure_instance(&folder).unwrap();
        assert_ne!(again, fp);
        assert!(!folder.join(".instance-new").exists() && !folder.join(".instance-new.pub").exists());
        let public = fs::read_to_string(folder.join("instance.pub")).unwrap();
        assert_eq!(machine::fingerprint(public.trim()).as_deref(), Some(again.as_str()));
        fs::write(folder.join("instance"), "not a key\n").unwrap();
        assert_eq!(machine::instance(&folder).unwrap(), None);
        let third = machine::ensure_instance(&folder).unwrap();
        assert_eq!(machine::instance(&folder).unwrap().as_deref(), Some(third.as_str()));
        // Commands in several repositories at once make one key (P7C2A-1).
        let shared = folder.parent().unwrap().join("shared");
        let threads: Vec<_> = (0..6)
            .map(|_| {
                let shared = shared.clone();
                std::thread::spawn(move || machine::ensure_instance(&shared).unwrap())
            })
            .collect();
        let fps: Vec<String> = threads.into_iter().map(|t| t.join().unwrap()).collect();
        assert!(fps.iter().all(|f| *f == fps[0]), "{fps:?}");
        assert_eq!(machine::instance(&shared).unwrap().as_deref(), Some(fps[0].as_str()));
    });
}

#[test]
fn req_reg_12_the_private_folder_is_found_from_the_environment() {
    let at = |pairs: &[(&str, &str)]| machine::private_folder(|v| pairs.iter().find(|p| p.0 == v).map(|p| p.1.into()));
    if cfg!(target_os = "macos") {
        assert_eq!(at(&[("HOME", "/h")]).unwrap(), Path::new("/h/Library/Application Support/fabrewk"));
        return;
    }
    assert_eq!(at(&[("HOME", "/h"), ("XDG_CONFIG_HOME", "/x")]).unwrap(), Path::new("/x/fabrewk"));
    assert_eq!(at(&[("HOME", "/h")]).unwrap(), Path::new("/h/.config/fabrewk"));
    assert_eq!(at(&[("HOME", "/h"), ("XDG_CONFIG_HOME", "rel")]).unwrap(), Path::new("/h/.config/fabrewk"));
    assert_eq!(at(&[("HOME", "/h"), ("XDG_CONFIG_HOME", "")]).unwrap(), Path::new("/h/.config/fabrewk"));
    assert!(at(&[("XDG_CONFIG_HOME", "rel")]).is_err());
    assert!(at(&[("HOME", "h")]).is_err());
    assert!(at(&[]).is_err());
}

#[test]
fn refusal_private_folder_permissions() {
    isolated("refusal_private_folder_permissions", || {
        let base = tmp("private-permissions");
        let folder = base.join("fabrewk");
        machine::ensure_instance(&folder).unwrap();
        for mode in [0o755, 0o750, 0o701, 0o710, 0o707] {
            fs::set_permissions(&folder, fs::Permissions::from_mode(mode)).unwrap();
            let before = tree(&base);
            for result in [machine::instance(&folder).map(|_| ()), machine::ensure_instance(&folder).map(|_| ())] {
                assert_eq!(result.unwrap_err().code(), Some(Code::PrivateFolderPermissions), "{mode:o}");
            }
            assert_eq!(tree(&base), before);
        }
        // Not a directory.
        let file = base.join("file");
        fs::write(&file, "").unwrap();
        assert_eq!(machine::instance(&file).unwrap_err().code(), Some(Code::PrivateFolderPermissions));
        // A folder the person made with access for its owner only is used as it is.
        let own = base.join("own");
        fs::create_dir(&own).unwrap();
        fs::set_permissions(&own, fs::Permissions::from_mode(0o700)).unwrap();
        assert!(machine::ensure_instance(&own).unwrap().starts_with("SHA256:"));
    });
}

#[test]
fn req_reg_4_append_writes_no_event_whose_signature_would_not_verify() {
    let f = Fixture::new("append-check", false);
    let digest = adopt(&f, &policy_text(&f, None, ""), &[]);
    // A signer that names another key than the one it signs with.
    let before = f.events("project");
    let mut s = begin_with(&f, None, &PolicyVerifier).unwrap();
    let head = s.head("project").unwrap();
    let draft = moved("publication-decided", "xel", "maestro", &digest, project_payload("publication-decided"));
    let e = s.append("project", head.as_ref(), &draft, Some(&Mixed(&f))).unwrap_err();
    assert_eq!(e.code(), Some(Code::RecordSignatureInvalid));
    assert_eq!(f.events("project"), before);
    let leftovers: Vec<_> = fs::read_dir(f.stream_dir("project")).unwrap().map(|e| e.unwrap().path()).collect();
    assert_eq!(leftovers.len(), before.len() * 2, "{leftovers:?}");
    s.finish(&PolicyVerifier, &[]).unwrap();
    ok(&f);
}

struct Mixed<'a>(&'a Fixture);

impl fabrewk_core::record::Sign for Mixed<'_> {
    fn sign(&self, bytes: &[u8]) -> Result<Vec<u8>, fabrewk_core::Error> {
        self.0.signer().sign(bytes)
    }
    fn public(&self) -> Result<String, fabrewk_core::Error> {
        self.0.stranger().public()
    }
}

#[test]
fn req_reg_4_the_key_a_signature_carries() {
    let f = Fixture::new("carried-key", false);
    let blob = fabrewk_core::settings::key_blob(&public(&f, "xel")).unwrap();
    assert_eq!(carried_key(&sign_with(&f.signer(), b"x", "fabrewk")), Some(blob));
    assert_eq!(carried_key(&sign_with(&f.signer(), b"x", "git")), None);
    let good = sign_with(&f.signer(), b"x", "fabrewk");
    for bad in [&good[1..], &good[..good.len() - 1], b"-----BEGIN SSH SIGNATURE-----\n-----END SSH SIGNATURE-----\n"] {
        assert_eq!(carried_key(bad), None);
    }
}

#[test]
fn req_reg_4_a_maestro_made_contributor_signs_nothing_after_the_adoption() {
    let f = Fixture::new("demotion", false);
    let first = adopt(&f, &policy_text(&f, Some("maestro"), ""), &[]);
    let (stream, _) = task(&f);
    let h2 =
        append(&f, &stream, &moved("task-authorized", "ana", "maestro", &first, bound(&f, &stream)), &f.stranger());
    adopt(&f, &policy_text(&f, Some("contributor"), ""), &["maintainers.ana.role"]);
    ok(&f);
    for role in ["maestro", "contributor"] {
        let forged = f.forge(
            &stream,
            3,
            Some(&h2.digest),
            &moved("run-started", "ana", role, &first, started(&f, &stream)),
            Some(&f.stranger()),
        );
        assert_eq!(refused(&f), [Code::RecordSignatureInvalid], "{role}");
        fs::remove_file(sig(&forged)).unwrap();
        fs::remove_file(&forged).unwrap();
    }
}

#[test]
fn req_reg_4_an_event_every_seal_holds_was_written_before_any_adoption() {
    let f = Fixture::new("before-adoption", false);
    let text = policy_text(&f, None, "");
    fs::write(f.domain.join("settings.toml"), &text).unwrap();
    let digest = sha256_hex(text.as_bytes());
    // A task stream, and a maestro's move in it, written before the first adoption, whose seal then holds them.
    let (stream, first) = forge_stage_and_task(&f);
    let h1 = sha256_hex(&fs::read(&first).unwrap());
    let moved_early = f.forge(
        &stream,
        2,
        Some(&h1),
        &moved("task-authorized", "xel", "maestro", &digest, bound(&f, &stream)),
        Some(&f.signer()),
    );
    let h2 = sha256_hex(&fs::read(&moved_early).unwrap());
    let adoption = settings::adopt(&f.domain, None, &[], None).unwrap();
    let seal = [(stream.clone(), Head { seq: 2, digest: h2 })].into_iter().collect();
    f.forge(
        "project",
        1,
        None,
        &moved("policy-adopted", "xel", "maestro", &digest, adoption.payload(&seal)),
        Some(&f.signer()),
    );
    assert_eq!(refused(&f), [Code::RecordSignatureInvalid]);
    // Without that move, the seal names a position its stream no longer reaches.
    fs::remove_file(sig(&moved_early)).unwrap();
    fs::remove_file(&moved_early).unwrap();
    assert_eq!(refused(&f), [Code::RecordRollback]);
}
