//! The record's tests at the library's level (part 7b, D-047): REQ-REG-1 to REQ-REG-11, PD-01 to PD-05, PD-06's
//! place, PD-31's replay, and every code the record raises. Tests that run `git` do it in an isolated child process.

mod common;

use common::*;
use fabrewk_core::record::{
    Code, FIELDS, Head, KINDS, Publication, Repo, Session, check_mark, encode, replay, sha256_hex,
    uncommitted, walk_history,
};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

/// Appends the project's four events of `apply` (three signed person moves and `installed`).
fn project(f: &Fixture, s: &mut Session) -> Head {
    let mut head = None;
    for (kind, words) in [
        ("assessed", "Evalúa."),
        ("publication-decided", "Publicado."),
        ("policy-adopted", "Adopto."),
    ] {
        head = Some(
            s.append(
                "project",
                head.as_ref(),
                &person(kind, words, json!({})),
                Some(&f.signer()),
            )
            .unwrap(),
        );
    }
    s.append(
        "project",
        head.as_ref(),
        &fabrewk("installed", json!({"files": []})),
        None,
    )
    .unwrap()
}

/// A task created and authorized; gives its stream and head.
fn task(f: &Fixture, s: &mut Session) -> (String, Head) {
    let (stream, draft) = creation("task-proposed", json!({"goal": "x"}));
    let h = s.append(&stream, None, &draft, None).unwrap();
    let h = s
        .append(
            &stream,
            Some(&h),
            &person("task-authorized", "Autorizo.", json!({})),
            Some(&f.signer()),
        )
        .unwrap();
    (stream, h)
}

fn ok_replay(f: &Fixture) -> fabrewk_core::record::Replay {
    replay(&f.domain, &f.verifier()).unwrap()
}

fn snapshot(dir: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).into_iter().flatten().flatten() {
            let p = e.path();
            if p.is_dir() {
                if p.file_name() != Some(".git".as_ref()) {
                    stack.push(p.clone());
                }
                out.push((p, Vec::new()));
            } else {
                out.push((p.clone(), std::fs::read(&p).unwrap()));
            }
        }
    }
    out.sort();
    out
}

// ---- REQ-REG-1, REQ-REG-2, REQ-REG-9 -------------------------------------------------------------------------------

#[test]
fn req_reg_1_names_streams_and_identifiers() {
    let f = Fixture::new("names", false);
    let mut s = begin(&f, None);
    project(&f, &mut s);
    let (stream, _) = task(&f, &mut s);
    s.finish(&f.verifier(), &[]).unwrap();
    assert!(
        stream.starts_with("task-t-")
            && stream.len() == 15
            && stream[7..].bytes().all(|b| b.is_ascii_hexdigit())
    );
    let r = ok_replay(&f);
    for events in r.streams.values() {
        for e in events {
            let bytes = std::fs::read(&e.path).unwrap();
            let name = e.path.file_name().unwrap().to_str().unwrap().to_owned();
            assert_eq!(
                name,
                format!("{:06}-{}.json", e.seq, &sha256_hex(&bytes)[..16])
            );
            assert_eq!(e.digest, sha256_hex(&bytes));
            assert_eq!(
                e.path
                    .parent()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap(),
                e.stream
            );
        }
    }
    let first = &r.streams[&stream][0];
    let mut payload = encode(&first.value["payload"]).unwrap();
    payload.pop();
    assert_eq!(stream, format!("task-t-{}", &sha256_hex(&payload)[..8]));
}

#[test]
fn req_reg_1_append_refuses_a_creation_outside_its_stream() {
    let f = Fixture::new("creation-place", false);
    let mut s = begin(&f, None);
    let (_, draft) = creation("task-proposed", json!({}));
    let e = s.append("task-t-00000000", None, &draft, None).unwrap_err();
    assert_eq!(code(&e), Some(Code::RecordBroken));
    let e = s.append("task-x", None, &draft, None).unwrap_err();
    assert_eq!(code(&e), Some(Code::RecordBroken));
    assert!(!f.domain.join("data/events/task-t-00000000").exists());
}

#[test]
fn refusal_record_id_collision() {
    // Two creating payloads whose digests share their first 8 hexadecimal digits, nonces chosen by the fixture.
    let mut seen = std::collections::HashMap::new();
    let (a, b) = (0u64..)
        .find_map(|i| {
            let payload = json!({"nonce": format!("{i:032x}")});
            let stream = fabrewk_core::record::subject_stream("task-proposed", &payload).unwrap();
            seen.insert(stream, payload.clone())
                .map(|other| (other, payload))
        })
        .unwrap();
    let stream = fabrewk_core::record::subject_stream("task-proposed", &a).unwrap();
    let f = Fixture::new("collision", false);
    f.forge(&stream, 1, None, &agent("task-proposed", a.clone()), None);
    f.forge(&stream, 1, None, &agent("task-proposed", b.clone()), None);
    assert_eq!(
        codes(replay(&f.domain, &f.verifier())),
        [Code::RecordIdCollision]
    );
    std::fs::remove_dir_all(f.stream_dir(&stream)).unwrap();
    // One creation and one event that is not the stream's own at `seq` 1: a fork, not a collision.
    f.forge(&stream, 1, None, &agent("task-proposed", a.clone()), None);
    f.forge(
        &stream,
        1,
        None,
        &agent("task-proposed", json!({"nonce": "f".repeat(32)})),
        None,
    );
    assert_eq!(codes(replay(&f.domain, &f.verifier())), [Code::RecordFork]);
    std::fs::remove_dir_all(f.stream_dir(&stream)).unwrap();
    // Two creations deriving the stream's identifier, later than `seq` 1: a fork.
    let first = f.forge(&stream, 1, None, &agent("task-proposed", a.clone()), None);
    let d1 = sha256_hex(&std::fs::read(&first).unwrap());
    f.forge(&stream, 2, Some(&d1), &agent("task-proposed", a), None);
    f.forge(&stream, 2, Some(&d1), &agent("task-proposed", b), None);
    assert_eq!(codes(replay(&f.domain, &f.verifier())), [Code::RecordFork]);
    // Two events at `seq` 1 of a stream that is not a stage's or a task's: a fork.
    std::fs::remove_dir_all(f.stream_dir(&stream)).unwrap();
    f.forge(
        "project",
        1,
        None,
        &fabrewk("installed", json!({"n": 1})),
        None,
    );
    f.forge(
        "project",
        1,
        None,
        &fabrewk("installed", json!({"n": 2})),
        None,
    );
    assert_eq!(codes(replay(&f.domain, &f.verifier())), [Code::RecordFork]);
}

#[test]
fn req_reg_2_fields_and_say_only_on_person_moves() {
    let f = Fixture::new("fields", false);
    let mut s = begin(&f, None);
    project(&f, &mut s);
    task(&f, &mut s);
    s.finish(&f.verifier(), &[]).unwrap();
    for events in ok_replay(&f).streams.values() {
        for e in events {
            let o = e.value.as_object().unwrap();
            let person = fabrewk_core::record::class_of(&e.kind)
                == Some(fabrewk_core::record::Class::Person);
            let mut keys: Vec<&str> = o.keys().map(String::as_str).collect();
            keys.sort();
            let mut want: Vec<&str> = FIELDS.to_vec();
            if person {
                want.push("say");
            }
            want.sort();
            assert_eq!(keys, want, "{}", e.kind);
            assert_eq!(o["prev"].is_null(), e.seq == 1);
            assert!(fabrewk_core::record::is_utc_timestamp(
                o["at"].as_str().unwrap()
            ));
            assert_eq!(o["fabrewk"], json!(fabrewk_core::VERSION));
        }
    }
}

#[test]
fn req_reg_2_order_comes_from_prev_not_time() {
    // The same moves, written with their `at` values permuted, replay to the same state.
    let times = [
        "2026-11-05T10:00:00Z",
        "2020-01-01T00:00:00Z",
        "2030-06-30T12:30:00Z",
        "2026-11-05T09:59:59Z",
    ];
    let mut states = Vec::new();
    for order in [[0, 1, 2, 3], [3, 2, 1, 0], [1, 3, 0, 2]] {
        let f = Fixture::new("permuted", false);
        let mut prev: Option<String> = None;
        for (seq, i) in order.iter().enumerate() {
            let mut v = fabrewk("installed", json!({"n": seq})).build(
                "project",
                seq as u64 + 1,
                prev.as_deref(),
            );
            v["at"] = json!(times[*i]);
            let bytes = encode(&v).unwrap();
            f.place("project", seq as u64 + 1, &bytes);
            prev = Some(sha256_hex(&bytes));
        }
        let r = ok_replay(&f);
        states.push(
            r.streams["project"]
                .iter()
                .map(|e| (e.seq, e.kind.clone(), e.value["payload"].clone()))
                .collect::<Vec<_>>(),
        );
    }
    assert_eq!(states[0], states[1]);
    assert_eq!(states[0], states[2]);
}

#[test]
fn pd_03_schema_1_is_pinned() {
    // Any change to an event's fields must raise `schema` (REQ-REG-9); this pin fails first.
    assert_eq!(
        FIELDS,
        [
            "actor", "at", "fabrewk", "instance", "kind", "payload", "policy", "prev", "schema",
            "seq", "stream"
        ]
    );
    let pinned: Vec<String> = KINDS
        .iter()
        .map(|(k, s, c)| format!("{k} {s:?} {c:?}"))
        .collect();
    assert_eq!(
        pinned.join("\n"),
        "assessed Project Person\npublication-decided Project Person\npolicy-adopted Project Person\n\
         installed Project Fabrewk\nalambique-baseline Alambique Person\nproposal-incorporated Alambique Person\n\
         proposal-incorporated Rules Person\ndirect-edit-approved Alambique Person\nsweep-recorded Fermento Agent\n\
         proposal-recorded Fermento Agent\nproposal-withdrawn Fermento Fabrewk\nproposal-discarded Fermento Person\n\
         stage-proposed Stage NotPerson\nstage-authorized Stage Person\nstage-closed Stage Person\n\
         task-proposed Task NotPerson\ntask-authorized Task Person\nrun-started Task Person\n\
         attempt-started Task Fabrewk\nattempt-ended Task Fabrewk\ncandidate-recorded Task Fabrewk\n\
         check-recorded Task Fabrewk\nreview-recorded Task Fabrewk\ntask-verified Task Fabrewk\n\
         task-halted Task Fabrewk\ntask-accepted Task Person\nmerge-failed Task Fabrewk\n\
         task-integrated Task Fabrewk\ntask-rejected Task Person\nextension-note Ext Extension"
    );
}

fn forged_with(f: &Fixture, edit: impl Fn(&mut Value)) -> Vec<Code> {
    let mut v = fabrewk("installed", json!({})).build("project", 1, None);
    edit(&mut v);
    f.place("project", 1, &encode(&v).unwrap());
    let r = codes(replay(&f.domain, &f.verifier()));
    std::fs::remove_dir_all(f.stream_dir("project")).unwrap();
    r
}

#[test]
fn refusal_record_version_unknown() {
    let f = Fixture::new("version", false);
    let unknown = [Code::RecordVersionUnknown];
    assert_eq!(forged_with(&f, |v| v["schema"] = json!(2)), unknown);
    assert_eq!(forged_with(&f, |v| v["schema"] = json!("1")), unknown);
    assert_eq!(forged_with(&f, |v| v["extra"] = json!(1)), unknown);
    assert_eq!(
        forged_with(&f, |v| drop(v.as_object_mut().unwrap().remove("instance"))),
        unknown
    );
    assert_eq!(
        forged_with(&f, |v| v["kind"] = json!("task-teleported")),
        unknown
    );
    assert_eq!(forged_with(&f, |v| v["at"] = json!("yesterday")), unknown);
    assert_eq!(forged_with(&f, |v| v["policy"] = json!("abc")), unknown);
    assert_eq!(forged_with(&f, |v| v["payload"] = json!([])), unknown);
    assert_eq!(
        forged_with(&f, |v| v["actor"] = json!({"kind": "fabrewk", "name": "x"})),
        unknown
    );
    assert_eq!(forged_with(&f, |v| v["fabrewk"] = json!("")), unknown);
    assert_eq!(
        forged_with(&f, |v| v["say"] = json!("x")),
        [Code::RecordTransitionInvalid]
    );
    let f2 = Fixture::new("version-say", false);
    for say in [
        json!({"a": 1, "b": 2}),
        json!({"text": 1, "lang": "es"}),
        json!({"text": "x", "lang": ""}),
        json!("x"),
    ] {
        let mut v = person("assessed", "x", json!({})).build("project", 1, None);
        v["say"] = say.clone();
        let path = f2.place("project", 1, &encode(&v).unwrap());
        std::fs::write(
            sig(&path),
            sign_with(&f2.signer(), &std::fs::read(&path).unwrap(), "fabrewk"),
        )
        .unwrap();
        assert_eq!(codes(replay(&f2.domain, &f2.verifier())), unknown, "{say}");
        std::fs::remove_dir_all(f2.stream_dir("project")).unwrap();
    }
    // A record holding every version this binary knows (schema 1) replays.
    let mut s = begin(&f, None);
    project(&f, &mut s);
    s.finish(&f.verifier(), &[]).unwrap();
    ok_replay(&f);
}

// ---- REQ-REG-3, PD-04 ---------------------------------------------------------------------------------------------

#[test]
fn pd_04_unicode_text_is_stored_and_replayed_intact() {
    let f = Fixture::new("unicode", false);
    let text = "a\u{85}b\u{2028}c\u{2029}d\re\u{0}f\n\"\\/é😀";
    let mut s = begin(&f, None);
    s.append(
        "project",
        None,
        &person(
            "assessed",
            text,
            json!({"file": format!("x{text}.md"), "finding": text}),
        ),
        Some(&f.signer()),
    )
    .unwrap();
    let (stream, draft) = creation("task-proposed", json!({"goal": text}));
    s.append(&stream, None, &draft, None).unwrap();
    s.finish(&f.verifier(), &[]).unwrap();
    let r = ok_replay(&f);
    let e = &r.streams["project"][0];
    assert_eq!(e.value["say"]["text"], json!(text));
    assert_eq!(e.value["payload"]["finding"], json!(text));
    assert_eq!(r.streams[&stream][0].value["payload"]["goal"], json!(text));
    for events in r.streams.values() {
        for e in events {
            assert_eq!(encode(&e.value).unwrap(), std::fs::read(&e.path).unwrap());
        }
    }
    // A truncated file, and one with a byte changed, are still refused.
    let path = &r.streams[&stream][0].path;
    let bytes = std::fs::read(path).unwrap();
    std::fs::write(path, &bytes[..bytes.len() / 2]).unwrap();
    assert_eq!(
        codes(replay(&f.domain, &f.verifier())),
        [Code::RecordDigestMismatch]
    );
    let mut changed = bytes.clone();
    changed[1] = b' ';
    std::fs::remove_file(path).unwrap();
    f.place(&stream, 1, &changed);
    assert_eq!(
        codes(replay(&f.domain, &f.verifier())),
        [Code::RecordNoncanonical]
    );
}

#[test]
fn pd_04_no_line_splitting_in_the_record() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/record");
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        let source = std::fs::read_to_string(&path).unwrap();
        for banned in [
            ".lines()",
            "split_terminator",
            "split_whitespace",
            "split_ascii_whitespace",
            "BufRead",
            "read_line",
        ] {
            assert!(
                !source.contains(banned),
                "{} calls {banned}",
                path.display()
            );
        }
    }
}

#[test]
fn refusal_record_noncanonical() {
    let f = Fixture::new("noncanonical", false);
    let good = String::from_utf8(
        encode(&fabrewk("installed", json!({"n": 1})).build("project", 1, None)).unwrap(),
    )
    .unwrap();
    let edits: [&dyn Fn(&str) -> String; 5] = [
        &|s| {
            s.replacen("{\"actor\"", "{\"schema\":1,\"actor\"", 1)
                .replacen(",\"schema\":1", "", 1)
        },
        &|s| s.replacen("\"n\":1", "\"n\": 1", 1),
        &|s| s.replacen("\"n\":1", "\"n\":1.0", 1),
        &|s| s.replacen("\"installed\"", "\"install\\u0065d\"", 1),
        &|s| s.trim_end_matches('\n').to_owned(),
    ];
    for edit in edits {
        let bad = edit(&good);
        assert_ne!(bad, good);
        let path = f.place("project", 1, bad.as_bytes());
        assert_eq!(
            codes(replay(&f.domain, &f.verifier())),
            [Code::RecordNoncanonical],
            "{bad}"
        );
        std::fs::remove_file(path).unwrap();
    }
}

// ---- REQ-REG-4 ----------------------------------------------------------------------------------------------------

#[test]
fn req_reg_4_signatures_verify_with_ssh_keygen() {
    let f = Fixture::new("signatures", false);
    let mut s = begin(&f, None);
    project(&f, &mut s);
    s.finish(&f.verifier(), &[]).unwrap();
    let r = ok_replay(&f);
    let signed: Vec<_> = r.streams["project"]
        .iter()
        .filter(|e| e.value.get("say").is_some())
        .collect();
    assert_eq!(signed.len(), 3);
    for e in signed {
        let out = std::process::Command::new("ssh-keygen")
            .args(["-Y", "verify", "-n", "fabrewk", "-I", "xel", "-f"])
            .arg(f.keys.join("allowed"))
            .arg("-s")
            .arg(sig(&e.path))
            .stdin(std::fs::File::open(&e.path).unwrap())
            .output()
            .unwrap();
        assert!(out.status.success());
    }
    assert!(!sig(&r.streams["project"][3].path).exists());
}

#[test]
fn refusal_record_signature_invalid() {
    let f = Fixture::new("signature-invalid", false);
    let draft = person("assessed", "Evalúa.", json!({}));
    let check = |setup: &dyn Fn(&Path)| {
        let path = f.forge("project", 1, None, &draft, None);
        setup(&path);
        let r = codes(replay(&f.domain, &f.verifier()));
        std::fs::remove_dir_all(f.stream_dir("project")).unwrap();
        r
    };
    let invalid = [Code::RecordSignatureInvalid];
    // No `.sig`.
    assert_eq!(check(&|_| {}), invalid);
    // Signed by a key absent from the signers.
    assert_eq!(
        check(&|p| std::fs::write(
            sig(p),
            sign_with(&f.stranger(), &std::fs::read(p).unwrap(), "fabrewk")
        )
        .unwrap()),
        invalid
    );
    // A signature over other bytes.
    assert_eq!(
        check(&|p| std::fs::write(sig(p), sign_with(&f.signer(), b"other", "fabrewk")).unwrap()),
        invalid
    );
    // Another namespace.
    assert_eq!(
        check(&|p| std::fs::write(
            sig(p),
            sign_with(&f.signer(), &std::fs::read(p).unwrap(), "git")
        )
        .unwrap()),
        invalid
    );
    // A signature beside an event no person records.
    let path = f.forge("project", 1, None, &fabrewk("installed", json!({})), None);
    std::fs::write(
        sig(&path),
        sign_with(&f.signer(), &std::fs::read(&path).unwrap(), "fabrewk"),
    )
    .unwrap();
    assert_eq!(codes(replay(&f.domain, &f.verifier())), invalid);
    std::fs::remove_dir_all(f.stream_dir("project")).unwrap();
    // The library writes no person's event without a signer.
    let mut s = begin(&f, None);
    assert_eq!(
        code(&s.append("project", None, &draft, None).unwrap_err()),
        Some(Code::RecordSignatureInvalid)
    );
    assert!(!f.stream_dir("project").exists() || f.events("project").is_empty());
}

// ---- REQ-REG-5 ----------------------------------------------------------------------------------------------------

#[test]
fn refusal_lock_held() {
    let f = Fixture::new("lock", false);
    let s = begin(&f, None);
    assert_eq!(
        code(
            &Session::begin(&f.domain, None, &f.verifier())
                .err()
                .unwrap()
        ),
        Some(Code::LockHeld)
    );
    // Another process is refused at once too.
    let out = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["lock_child", "--exact", "--nocapture", "--test-threads=1"])
        .env("FABREWK_TEST_LOCK_DOMAIN", &f.domain)
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&out.stdout).contains("test lock_child ... ok"));
    drop(s);
    begin(&f, None);
}

#[test]
fn lock_child() {
    if let Some(domain) = std::env::var_os("FABREWK_TEST_LOCK_DOMAIN") {
        let started = std::time::Instant::now();
        let verifier = fabrewk_core::record::SshVerifier(PathBuf::from("/nonexistent"));
        assert_eq!(
            code(
                &Session::begin(Path::new(&domain), None, &verifier)
                    .err()
                    .unwrap()
            ),
            Some(Code::LockHeld)
        );
        assert!(started.elapsed() < std::time::Duration::from_secs(2));
    }
}

#[test]
fn refusal_record_head_moved() {
    let f = Fixture::new("head-moved", false);
    let mut s = begin(&f, None);
    let h1 = s
        .append(
            "project",
            None,
            &fabrewk("installed", json!({"n": 1})),
            None,
        )
        .unwrap();
    // A writer that ignores the lock appends meanwhile.
    f.forge(
        "project",
        2,
        Some(&h1.digest),
        &fabrewk("installed", json!({"n": 2})),
        None,
    );
    let before = snapshot(&f.domain);
    let e = s
        .append(
            "project",
            Some(&h1),
            &fabrewk("installed", json!({"n": 3})),
            None,
        )
        .unwrap_err();
    assert_eq!(code(&e), Some(Code::RecordHeadMoved));
    assert_eq!(snapshot(&f.domain), before);
    // A stale "no head yet" is refused the same way.
    assert_eq!(
        code(
            &s.append("project", None, &fabrewk("installed", json!({})), None)
                .unwrap_err()
        ),
        Some(Code::RecordHeadMoved)
    );
}

#[test]
fn req_reg_5_readers_skip_temporary_names_and_writers_remove_them() {
    let f = Fixture::new("temporary", false);
    let mut s = begin(&f, None);
    let h = s
        .append("project", None, &fabrewk("installed", json!({})), None)
        .unwrap();
    drop(s);
    let dir = f.stream_dir("project");
    // What a writer killed mid-write leaves: a temporary event and a signature whose event never got in place.
    std::fs::write(dir.join(".000002-0123456789abcdef.json.tmp"), b"{").unwrap();
    std::fs::write(dir.join("000002-0123456789abcdef.json.sig"), b"x").unwrap();
    std::fs::write(f.domain.join("data/events/.hidden"), b"").unwrap();
    // Other names starting with `.` are skipped and left alone, a directory included.
    std::fs::write(dir.join(".notes"), b"").unwrap();
    std::fs::create_dir(dir.join(".keep")).unwrap();
    assert_eq!(ok_replay(&f).streams["project"].len(), 1);
    let mut s = begin(&f, None);
    assert!(!dir.join(".000002-0123456789abcdef.json.tmp").exists());
    assert!(!dir.join("000002-0123456789abcdef.json.sig").exists());
    assert!(dir.join(".notes").exists() && dir.join(".keep").is_dir());
    assert_eq!(s.head("project").unwrap(), Some(h.clone()));
    s.append("project", Some(&h), &fabrewk("installed", json!({})), None)
        .unwrap();
}

// ---- REQ-REG-6 ----------------------------------------------------------------------------------------------------

#[test]
fn refusal_record_broken() {
    let f = Fixture::new("broken", false);
    let one = f.forge(
        "project",
        1,
        None,
        &fabrewk("installed", json!({"n": 1})),
        None,
    );
    let d1 = sha256_hex(&std::fs::read(&one).unwrap());
    let broken = [Code::RecordBroken];
    // A gap in `seq`.
    let three = f.forge(
        "project",
        3,
        Some(&d1),
        &fabrewk("installed", json!({"n": 3})),
        None,
    );
    assert_eq!(codes(replay(&f.domain, &f.verifier())), broken);
    std::fs::remove_file(three).unwrap();
    // A `prev` that names no event.
    let two = f.forge(
        "project",
        2,
        Some(&"0".repeat(64)),
        &fabrewk("installed", json!({"n": 2})),
        None,
    );
    assert_eq!(codes(replay(&f.domain, &f.verifier())), broken);
    std::fs::remove_file(two).unwrap();
    // An event copied into another stream's directory.
    std::fs::create_dir_all(f.stream_dir("rules")).unwrap();
    std::fs::copy(&one, f.stream_dir("rules").join(one.file_name().unwrap())).unwrap();
    assert_eq!(codes(replay(&f.domain, &f.verifier())), broken);
    std::fs::remove_dir_all(f.stream_dir("rules")).unwrap();
    // A file that is not an event, and a directory that is not a stream.
    std::fs::write(f.stream_dir("project").join("notes.txt"), b"").unwrap();
    assert_eq!(codes(replay(&f.domain, &f.verifier())), broken);
    std::fs::remove_file(f.stream_dir("project").join("notes.txt")).unwrap();
    std::fs::create_dir_all(f.domain.join("data/events/Project")).unwrap();
    assert_eq!(codes(replay(&f.domain, &f.verifier())), broken);
    std::fs::remove_dir_all(f.domain.join("data/events/Project")).unwrap();
    // `prev` set at `seq` 1.
    std::fs::remove_file(&one).unwrap();
    f.forge(
        "project",
        1,
        Some(&d1),
        &fabrewk("installed", json!({})),
        None,
    );
    assert_eq!(codes(replay(&f.domain, &f.verifier())), broken);
}

#[test]
fn pd_06_an_approval_copied_into_another_stream_is_refused_for_its_place() {
    let f = Fixture::new("pd06", false);
    let mut s = begin(&f, None);
    let (a, _) = task(&f, &mut s);
    let (b, b1) = {
        let (stream, draft) = creation("task-proposed", json!({"goal": "x"}));
        let h = s.append(&stream, None, &draft, None).unwrap();
        (stream, h)
    };
    s.finish(&f.verifier(), &[]).unwrap();
    let approval = f.events(&a)[1].clone();
    let copy = f.stream_dir(&b).join(approval.file_name().unwrap());
    std::fs::copy(&approval, &copy).unwrap();
    std::fs::copy(sig(&approval), sig(&copy)).unwrap();
    assert_eq!(b1.seq, 1);
    assert_eq!(
        codes(replay(&f.domain, &f.verifier())),
        [Code::RecordBroken]
    );
}

#[test]
fn refusal_record_digest_mismatch() {
    let f = Fixture::new("digest", false);
    let path = f.forge("project", 1, None, &fabrewk("installed", json!({})), None);
    // A file renamed with another digest.
    let other = f.stream_dir("project").join("000001-0000000000000000.json");
    std::fs::rename(&path, &other).unwrap();
    assert_eq!(
        codes(replay(&f.domain, &f.verifier())),
        [Code::RecordDigestMismatch]
    );
    std::fs::rename(&other, &path).unwrap();
    // A byte changed.
    let mut bytes = std::fs::read(&path).unwrap();
    bytes[10] ^= 1;
    std::fs::write(&path, bytes).unwrap();
    assert_eq!(
        codes(replay(&f.domain, &f.verifier())),
        [Code::RecordDigestMismatch]
    );
}

#[test]
fn refusal_record_transition_invalid() {
    let f = Fixture::new("transition", false);
    let invalid = [Code::RecordTransitionInvalid];
    let check = |stream: &str, draft: &fabrewk_core::record::Draft, signed: bool| {
        f.forge(stream, 1, None, draft, signed.then(|| f.signer()).as_ref());
        let r = codes(replay(&f.domain, &f.verifier()));
        std::fs::remove_dir_all(f.stream_dir(stream)).unwrap();
        r
    };
    // A kind in a stream that does not record it; an actor the kind does not allow.
    assert_eq!(
        check("project", &fabrewk("task-integrated", json!({})), false),
        invalid
    );
    assert_eq!(
        check("project", &fabrewk("assessed", json!({})), false),
        invalid
    );
    assert_eq!(
        check("project", &agent("installed", json!({})), false),
        invalid
    );
    let mut words = person("installed", "Hola.", json!({}));
    assert_eq!(check("project", &words, false), invalid);
    words.say = None;
    assert_eq!(check("project", &words, false), invalid);
    // Words on a move no person records, and a person's move without words.
    let mut said = fabrewk("installed", json!({}));
    said.say = person("assessed", "Hola.", json!({})).say;
    assert_eq!(check("project", &said, false), invalid);
    let mut silent = person("assessed", "Hola.", json!({}));
    silent.say = None;
    assert_eq!(check("project", &silent, true), invalid);
    // A second creation later in its own stream (P7B-4).
    let (stream, draft) = creation("task-proposed", json!({"goal": "x"}));
    let first = f.forge(&stream, 1, None, &draft, None);
    let d1 = sha256_hex(&std::fs::read(&first).unwrap());
    f.forge(&stream, 2, Some(&d1), &draft, None);
    assert_eq!(codes(replay(&f.domain, &f.verifier())), invalid);
    std::fs::remove_dir_all(f.stream_dir(&stream)).unwrap();
    // A task stream that does not begin with its creation.
    assert_eq!(
        check(
            "task-t-00000000",
            &person("task-authorized", "Autorizo.", json!({})),
            true
        ),
        invalid
    );
}

#[test]
fn pd_31_empty_words_on_replay() {
    let f = Fixture::new("pd31", false);
    for words in ["", " ", "\t", "\n", "\u{2028}", " \u{85}\u{3000} "] {
        f.forge(
            "project",
            1,
            None,
            &person("assessed", words, json!({})),
            Some(&f.signer()),
        );
        assert_eq!(
            codes(replay(&f.domain, &f.verifier())),
            [Code::RecordTransitionInvalid],
            "{words:?}"
        );
        std::fs::remove_dir_all(f.stream_dir("project")).unwrap();
        let mut s = begin(&f, None);
        let e = s
            .append(
                "project",
                None,
                &person("assessed", words, json!({})),
                Some(&f.signer()),
            )
            .unwrap_err();
        assert_eq!(code(&e), Some(Code::RecordTransitionInvalid));
        assert!(!f.stream_dir("project").exists() || f.events("project").is_empty());
    }
}

// ---- REQ-REG-7, PD-05 ---------------------------------------------------------------------------------------------

#[test]
fn pd_05_absence_is_never_an_empty_record() {
    let f = Fixture::new("absent", false);
    let absent = [Code::RecordAbsent];
    assert_eq!(codes(replay(&f.domain, &f.verifier())), absent);
    std::fs::create_dir_all(f.domain.join("data")).unwrap();
    assert_eq!(codes(replay(&f.domain, &f.verifier())), absent);
    std::fs::create_dir_all(f.domain.join("data/events")).unwrap();
    assert_eq!(codes(replay(&f.domain, &f.verifier())), absent);
    std::fs::create_dir_all(f.stream_dir("project")).unwrap();
    assert_eq!(codes(replay(&f.domain, &f.verifier())), absent);
}

// ---- REQ-REG-8: the mark, PD-02 -----------------------------------------------------------------------------------

#[test]
fn pd_02_an_invalid_mark_refuses_and_only_a_person_removes_it() {
    let f = Fixture::new("pd02", false);
    let mut s = begin(&f, None);
    project(&f, &mut s);
    s.finish(&f.verifier(), &[]).unwrap();
    let mark = f.domain.join("local/cache/heads.json");
    let good = std::fs::read(&mark).unwrap();
    let r = ok_replay(&f);
    check_mark(&f.domain, &r).unwrap();
    let wrong = String::from_utf8(good.clone())
        .unwrap()
        .replace(&r.streams["project"][3].digest, &"e".repeat(64));
    for bad in [
        &b""[..],
        b"{not json",
        b"{\"heads\":{},\"schema\":2}\n",
        b"{\"heads\":{\"project\":{\"seq\":1}},\"schema\":1}\n",
        wrong.as_bytes(),
    ] {
        std::fs::write(&mark, bad).unwrap();
        assert_eq!(
            code(&check_mark(&f.domain, &r).unwrap_err()),
            Some(Code::MarkInvalid),
            "{}",
            String::from_utf8_lossy(bad)
        );
        assert_eq!(
            code(&try_begin(&f, None).err().unwrap()),
            Some(Code::MarkInvalid)
        );
        assert_eq!(
            std::fs::read(&mark).unwrap(),
            bad,
            "nothing deletes or rewrites the mark"
        );
    }
    // Once a person removes it, the next write command writes a fresh mark.
    std::fs::remove_file(&mark).unwrap();
    let mut s = begin(&f, None);
    s.append(
        "project",
        s.head("project").unwrap().as_ref(),
        &fabrewk("installed", json!({})),
        None,
    )
    .unwrap();
    s.finish(&f.verifier(), &[]).unwrap();
    check_mark(&f.domain, &ok_replay(&f)).unwrap();
}

#[test]
fn pd_01_unpublished_rollback_by_deleting_event_files() {
    let f = Fixture::new("pd01-unpublished", false);
    let mut s = begin(&f, None);
    project(&f, &mut s);
    let (stream, _) = task(&f, &mut s);
    s.finish(&f.verifier(), &[]).unwrap();
    let rollback = Some(Code::RecordRollback);
    // (a) and (d): the last events cut; (b): back to an earlier state; a whole stream deleted.
    let files = f.events("project");
    let saved: Vec<_> = files
        .iter()
        .map(|p| {
            (
                p.clone(),
                std::fs::read(p).unwrap(),
                std::fs::read(sig(p)).ok(),
            )
        })
        .collect();
    for cut in [1, 2, 3] {
        for p in &files[files.len() - cut..] {
            std::fs::remove_file(p).unwrap();
            let _ = std::fs::remove_file(sig(p));
        }
        assert_eq!(
            code(&check_mark(&f.domain, &ok_replay(&f)).unwrap_err()),
            rollback
        );
        assert_eq!(code(&try_begin(&f, None).err().unwrap()), rollback);
        for (p, bytes, signature) in &saved {
            std::fs::write(p, bytes).unwrap();
            if let Some(signature) = signature {
                std::fs::write(sig(p), signature).unwrap();
            }
        }
    }
    std::fs::remove_dir_all(f.stream_dir(&stream)).unwrap();
    assert_eq!(
        code(&check_mark(&f.domain, &ok_replay(&f)).unwrap_err()),
        rollback
    );
    std::fs::remove_dir_all(f.domain.join("data")).unwrap();
    assert_eq!(code(&try_begin(&f, None).err().unwrap()), rollback);
}

#[test]
fn req_reg_8_reading_leaves_everything_byte_identical() {
    isolated("req_reg_8_reading_leaves_everything_byte_identical", || {
        let f = Fixture::new("reading", true);
        let mut s = begin(&f, f.repo());
        project(&f, &mut s);
        s.finish(&f.verifier(), &[]).unwrap();
        let before = snapshot(&f.root);
        let refs = git(&f.root, &["for-each-ref"]);
        let r = ok_replay(&f);
        check_mark(&f.domain, &r).unwrap();
        let repo = f.repo().unwrap();
        walk_history(&repo, &fabrewk_core::record::record_branch(&repo).unwrap()).unwrap();
        assert!(uncommitted(&repo).unwrap().is_empty());
        assert_eq!(snapshot(&f.root), before);
        assert_eq!(git(&f.root, &["for-each-ref"]), refs);
    });
}

// ---- REQ-REG-8: the history walk, PD-01 ---------------------------------------------------------------------------

/// A published record with three record commits: the project, a task, one more event. Gives the commits.
fn published(f: &Fixture) -> Vec<String> {
    let mut commits = Vec::new();
    let mut s = begin(f, f.repo());
    project(f, &mut s);
    commits.push(s.finish(&f.verifier(), &[]).unwrap().unwrap());
    let mut s = begin(f, f.repo());
    task(f, &mut s);
    commits.push(s.finish(&f.verifier(), &[]).unwrap().unwrap());
    let mut s = begin(f, f.repo());
    let h = s.head("project").unwrap();
    s.append(
        "project",
        h.as_ref(),
        &fabrewk("installed", json!({"n": 5})),
        None,
    )
    .unwrap();
    commits.push(s.finish(&f.verifier(), &[]).unwrap().unwrap());
    commits
}

fn walk(f: &Fixture) -> Option<Code> {
    let repo = f.repo().unwrap();
    walk_history(&repo, "refs/heads/main")
        .err()
        .and_then(|e| e.code())
}

fn last_event(f: &Fixture) -> String {
    let p = f.events("project").pop().unwrap();
    p.strip_prefix(&f.root)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}

#[test]
fn pd_01_history_walk() {
    isolated("pd_01_history_walk", || {
        let rollback = Some(Code::RecordRollback);
        // (a) a commit whose `data/` is a prefix of its parent's.
        let f = Fixture::new("pd01a", true);
        published(&f);
        assert_eq!(walk(&f), None);
        git(&f.root, &["rm", "-q", &last_event(&f)]);
        git(&f.root, &["commit", "-q", "-m", "cut"]);
        assert_eq!(walk(&f), rollback);
        // (b) the branch reset to an ancestor: refused by the mark.
        let f = Fixture::new("pd01b", true);
        let commits = published(&f);
        git(&f.root, &["reset", "-q", "--hard", &commits[1]]);
        assert_eq!(walk(&f), None);
        assert_eq!(
            code(&check_mark(&f.domain, &ok_replay(&f)).unwrap_err()),
            rollback
        );
        assert_eq!(code(&try_begin(&f, f.repo()).err().unwrap()), rollback);
        // (c) a merge whose first parent is the old state and the second the watched one.
        let f = Fixture::new("pd01c", true);
        let commits = published(&f);
        git(&f.root, &["checkout", "-q", "-b", "old", &commits[1]]);
        git(
            &f.root,
            &["merge", "-q", "-s", "ours", "-m", "merge", "main"],
        );
        git(&f.root, &["checkout", "-q", "main"]);
        git(&f.root, &["reset", "-q", "--hard", "old"]);
        assert_eq!(walk(&f), rollback);
        // (d) events seen, cut and regrown.
        let f = Fixture::new("pd01d", true);
        published(&f);
        let last = last_event(&f);
        git(&f.root, &["rm", "-q", &last]);
        git(&f.root, &["commit", "-q", "-m", "cut"]);
        let last = ok_replay(&f).streams["project"].last().unwrap().clone();
        f.forge(
            "project",
            last.seq + 1,
            Some(&last.digest),
            &fabrewk("installed", json!({"regrown": true})),
            None,
        );
        git(&f.root, &["add", ".brewstillery/data"]);
        git(&f.root, &["commit", "-q", "-m", "regrown"]);
        assert_eq!(walk(&f), rollback);
        assert_eq!(code(&try_begin(&f, f.repo()).err().unwrap()), rollback);
        // (e) a replacement object hiding the cut, and (f) a graft file doing the same.
        for graft in [false, true] {
            let f = Fixture::new("pd01ef", true);
            let commits = published(&f);
            git(&f.root, &["rm", "-q", &last_event(&f)]);
            git(&f.root, &["commit", "-q", "-m", "cut"]);
            let cut = git(&f.root, &["rev-parse", "HEAD"]);
            let tree = git(&f.root, &["rev-parse", "HEAD^{tree}"]);
            if graft {
                std::fs::write(
                    f.root.join(".git/info/grafts"),
                    format!("{cut} {}\n", commits[1]),
                )
                .unwrap();
            } else {
                let hidden = git(
                    &f.root,
                    &["commit-tree", &tree, "-p", &commits[1], "-m", "cut"],
                );
                git(&f.root, &["replace", &cut, &hidden]);
            }
            // Read through the hiding, the history looks whole; read as stored, it is a rollback.
            let parent = git(&f.root, &["rev-parse", "HEAD^"]);
            assert_eq!(parent, commits[1]);
            assert_eq!(walk(&f), rollback);
        }
        // (g) the last two events dropped, then restored byte-identical with a new one appended.
        let f = Fixture::new("pd01g", true);
        published(&f);
        let files: Vec<String> = f
            .events("project")
            .iter()
            .rev()
            .take(2)
            .map(|p| {
                p.strip_prefix(&f.root)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned()
            })
            .collect();
        let saved: Vec<Vec<u8>> = files
            .iter()
            .map(|p| std::fs::read(f.root.join(p)).unwrap())
            .collect();
        for p in &files {
            git(&f.root, &["rm", "-q", "--cached", p]);
        }
        git(&f.root, &["commit", "-q", "-m", "drop"]);
        for (p, bytes) in files.iter().zip(&saved) {
            std::fs::write(f.root.join(p), bytes).unwrap();
        }
        let last = ok_replay(&f).streams["project"].last().unwrap().clone();
        f.forge(
            "project",
            last.seq + 1,
            Some(&last.digest),
            &fabrewk("installed", json!({"n": 6})),
            None,
        );
        git(&f.root, &["add", ".brewstillery/data"]);
        git(&f.root, &["commit", "-q", "-m", "restored"]);
        assert_eq!(walk(&f), rollback);
        // The working tree after the tip: an event file deleted or changed there is a rollback too.
        let f = Fixture::new("pd01w", true);
        published(&f);
        let last = f.root.join(last_event(&f));
        std::fs::remove_file(&last).unwrap();
        assert_eq!(walk(&f), rollback);
    });
}

#[test]
fn req_reg_8_a_shallow_clone_cannot_be_walked() {
    isolated("req_reg_8_a_shallow_clone_cannot_be_walked", || {
        let f = Fixture::new("shallow", true);
        published(&f);
        let clone = f.root.parent().unwrap().join("clone");
        let url = format!("file://{}", f.root.display());
        git(
            f.root.parent().unwrap(),
            &["clone", "-q", "--depth", "1", &url, clone.to_str().unwrap()],
        );
        let repo = Repo {
            root: clone,
            prefix: ".brewstillery".into(),
        };
        assert_eq!(
            walk_history(&repo, "refs/heads/main").unwrap_err().code(),
            Some(Code::CheckUnknown)
        );
    });
}

// ---- REQ-REG-6: forks from two clones ------------------------------------------------------------------------------

#[test]
fn refusal_record_fork() {
    isolated("refusal_record_fork", || {
        let f = Fixture::new("fork", true);
        let mut s = begin(&f, f.repo());
        project(&f, &mut s);
        s.finish(&f.verifier(), &[]).unwrap();
        let clone = f.root.parent().unwrap().join("clone");
        git(
            f.root.parent().unwrap(),
            &[
                "clone",
                "-q",
                f.root.to_str().unwrap(),
                clone.to_str().unwrap(),
            ],
        );
        git(&clone, &["config", "user.name", "Clone"]);
        git(&clone, &["config", "user.email", "clone@example.invalid"]);
        for (root, n) in [(&f.root, 1), (&clone, 2)] {
            let domain = root.join(".brewstillery");
            let mut s = Session::begin(
                &domain,
                Repo::for_domain(&domain, Publication::Published),
                &f.verifier(),
            )
            .unwrap();
            let h = s.head("project").unwrap();
            s.append(
                "project",
                h.as_ref(),
                &fabrewk("installed", json!({"clone": n})),
                None,
            )
            .unwrap();
            s.finish(&f.verifier(), &[]).unwrap();
        }
        git(
            &f.root,
            &[
                "pull",
                "-q",
                "--no-rebase",
                "--no-edit",
                clone.to_str().unwrap(),
                "main",
            ],
        );
        assert_eq!(codes(replay(&f.domain, &f.verifier())), [Code::RecordFork]);
        assert_eq!(
            code(&try_begin(&f, f.repo()).err().unwrap()),
            Some(Code::RecordFork)
        );
    });
}

// ---- REQ-REG-10, REQ-REG-11 ----------------------------------------------------------------------------------------

#[test]
fn req_reg_10_one_record_commit_with_its_trailers_and_the_persons_edits_untouched() {
    isolated(
        "req_reg_10_one_record_commit_with_its_trailers_and_the_persons_edits_untouched",
        || {
            let f = Fixture::new("commit", true);
            std::fs::write(f.root.join("README.md"), "staged\n").unwrap();
            git(&f.root, &["add", "README.md"]);
            std::fs::write(f.root.join("README.md"), "unstaged\n").unwrap();
            std::fs::write(f.root.join("notes.txt"), "untracked\n").unwrap();
            let staged = git(&f.root, &["diff", "--cached"]);
            let start = git(&f.root, &["rev-parse", "HEAD"]);
            let mut s = begin(&f, f.repo());
            project(&f, &mut s);
            std::fs::write(f.domain.join("STATUS.md"), "view\n").unwrap();
            let commit = s
                .finish(&f.verifier(), &[PathBuf::from("STATUS.md")])
                .unwrap()
                .unwrap();
            assert_eq!(git(&f.root, &["rev-parse", "HEAD"]), commit);
            assert_eq!(git(&f.root, &["rev-parse", "HEAD^"]), start);
            let files = git(
                &f.root,
                &["diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"],
            );
            let mut want: Vec<String> = f
                .events("project")
                .iter()
                .map(|p| {
                    p.strip_prefix(&f.root)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
                .collect();
            want.extend(
                f.events("project")
                    .iter()
                    .take(3)
                    .map(|p| format!("{}.sig", p.strip_prefix(&f.root).unwrap().display())),
            );
            want.push(".brewstillery/STATUS.md".into());
            want.sort();
            assert_eq!(files.split('\n').collect::<Vec<_>>(), want);
            let message = git(&f.root, &["log", "-1", "--format=%B"]);
            let digests: Vec<String> = ok_replay(&f).streams["project"]
                .iter()
                .map(|e| format!("Fabrewk-Event: {}", e.digest))
                .collect();
            assert_eq!(
                message.trim_end(),
                format!("fabrewk: record\n\n{}", digests.join("\n"))
            );
            assert_eq!(
                std::fs::read_to_string(f.root.join("README.md")).unwrap(),
                "unstaged\n"
            );
            assert_eq!(git(&f.root, &["diff", "--cached"]), staged);
            assert_eq!(
                std::fs::read_to_string(f.root.join("notes.txt")).unwrap(),
                "untracked\n"
            );
            assert_eq!(
                git(
                    &f.root,
                    &[
                        "diff-index",
                        "--cached",
                        "--name-only",
                        "HEAD",
                        "--",
                        ".brewstillery"
                    ]
                ),
                ""
            );
            assert!(!f.domain.join("local/tmp/record-index").exists());
            assert!(uncommitted(&f.repo().unwrap()).unwrap().is_empty());
        },
    );
}

#[test]
fn req_reg_10_a_moved_head_rebuilds_or_races() {
    isolated("req_reg_10_a_moved_head_rebuilds_or_races", || {
        // Moved at an untouched file: the commit is rebuilt on the new head.
        let f = Fixture::new("rebuild", true);
        let mut s = begin(&f, f.repo());
        project(&f, &mut s);
        std::fs::write(f.root.join("other.txt"), "x\n").unwrap();
        git(&f.root, &["add", "other.txt"]);
        git(&f.root, &["commit", "-q", "-m", "meanwhile"]);
        let moved = git(&f.root, &["rev-parse", "HEAD"]);
        let commit = s.finish(&f.verifier(), &[]).unwrap().unwrap();
        assert_eq!(git(&f.root, &["rev-parse", "HEAD^"]), moved);
        assert_eq!(git(&f.root, &["rev-parse", "HEAD"]), commit);
        // Moved at a file the commit holds: RECORD_COMMIT_RACE, the events kept and uncommitted, nothing else.
        let f = Fixture::new("race", true);
        let mut s = begin(&f, f.repo());
        project(&f, &mut s);
        std::fs::write(f.domain.join("STATUS.md"), "theirs\n").unwrap();
        git(&f.root, &["add", ".brewstillery/STATUS.md"]);
        git(&f.root, &["commit", "-q", "-m", "meanwhile"]);
        std::fs::write(f.domain.join("STATUS.md"), "ours\n").unwrap();
        let refs = git(&f.root, &["for-each-ref"]);
        let index = std::fs::read(f.root.join(".git/index")).unwrap();
        let e = s
            .finish(&f.verifier(), &[PathBuf::from("STATUS.md")])
            .unwrap_err();
        assert_eq!(code(&e), Some(Code::RecordCommitRace));
        assert_eq!(git(&f.root, &["for-each-ref"]), refs);
        assert_eq!(std::fs::read(f.root.join(".git/index")).unwrap(), index);
        assert!(!f.domain.join("local/tmp/record-index").exists());
        let left = uncommitted(&f.repo().unwrap()).unwrap();
        assert_eq!(left.len(), 7);
        // The next write command first commits them in a record commit of their own.
        let mut s = begin(&f, f.repo());
        let first = s.commit_uncommitted(&f.verifier()).unwrap().unwrap();
        assert!(uncommitted(&f.repo().unwrap()).unwrap().is_empty());
        let h = s.head("project").unwrap();
        s.append(
            "project",
            h.as_ref(),
            &fabrewk("installed", json!({})),
            None,
        )
        .unwrap();
        let second = s.finish(&f.verifier(), &[]).unwrap().unwrap();
        assert_eq!(git(&f.root, &["rev-parse", "HEAD^"]), first);
        assert_eq!(git(&f.root, &["rev-parse", "HEAD"]), second);
        assert_eq!(
            git(&f.root, &["log", "-1", "--format=%B", &first])
                .matches("Fabrewk-Event: ")
                .count(),
            4
        );
    });
}

#[test]
fn req_reg_10_a_failed_replay_commits_nothing() {
    isolated("req_reg_10_a_failed_replay_commits_nothing", || {
        let f = Fixture::new("replay-fails", true);
        let mut s = begin(&f, f.repo());
        project(&f, &mut s);
        // Damage that only the replay before the commit sees.
        std::fs::write(f.stream_dir("project").join("junk"), b"").unwrap();
        let head = git(&f.root, &["rev-parse", "HEAD"]);
        assert_eq!(
            code(&s.finish(&f.verifier(), &[]).unwrap_err()),
            Some(Code::RecordBroken)
        );
        assert_eq!(git(&f.root, &["rev-parse", "HEAD"]), head);
        assert!(!f.domain.join("local/cache/heads.json").exists());
    });
}

#[test]
fn req_reg_10_modes() {
    isolated("req_reg_10_modes", || {
        // unpublished: no commit.
        let f = Fixture::new("unpublished", true);
        let head = git(&f.root, &["rev-parse", "HEAD"]);
        let mut s = begin(&f, Repo::for_domain(&f.domain, Publication::Unpublished));
        project(&f, &mut s);
        assert_eq!(s.finish(&f.verifier(), &[]).unwrap(), None);
        assert_eq!(git(&f.root, &["rev-parse", "HEAD"]), head);
        // own-repository: the domain is a repository of its own, and the commit is made there.
        let f = Fixture::new("own", false);
        git(&f.domain, &["init", "-q", "-b", "main"]);
        git(&f.domain, &["config", "user.name", "Fixture"]);
        git(
            &f.domain,
            &["config", "user.email", "fixture@example.invalid"],
        );
        let repo = Repo::for_domain(&f.domain, Publication::OwnRepository);
        let mut s = begin(&f, repo.clone());
        project(&f, &mut s);
        s.finish(&f.verifier(), &[]).unwrap().unwrap();
        let files = git(&f.domain, &["ls-tree", "-r", "--name-only", "HEAD"]);
        assert!(
            files
                .split('\n')
                .all(|p| p.starts_with("data/events/project/")),
            "{files}"
        );
        assert_eq!(files.split('\n').count(), 7);
        assert_eq!(git(&f.domain, &["rev-list", "--count", "HEAD"]), "1");
        walk_history(&repo.unwrap(), "refs/heads/main").unwrap();
    });
}

#[test]
fn refusal_head_detached() {
    isolated("refusal_head_detached", || {
        let f = Fixture::new("detached", true);
        git(&f.root, &["checkout", "-q", "--detach"]);
        let e = try_begin(&f, f.repo()).err().unwrap();
        assert_eq!(code(&e), Some(Code::HeadDetached));
        assert!(!f.domain.join("data").exists());
    });
}

#[test]
fn req_reg_11_the_record_branch_is_the_checked_out_one() {
    isolated(
        "req_reg_11_the_record_branch_is_the_checked_out_one",
        || {
            let f = Fixture::new("trunk", true);
            git(&f.root, &["checkout", "-q", "-b", "trunk"]);
            let main = git(&f.root, &["rev-parse", "main"]);
            let mut s = begin(&f, f.repo());
            project(&f, &mut s);
            let commit = s.finish(&f.verifier(), &[]).unwrap().unwrap();
            assert_eq!(git(&f.root, &["rev-parse", "trunk"]), commit);
            assert_eq!(git(&f.root, &["rev-parse", "main"]), main);
            // From a linked working tree, the branch is still the main working tree's.
            let linked = f.root.parent().unwrap().join("linked");
            git(
                &f.root,
                &[
                    "worktree",
                    "add",
                    "-q",
                    "-b",
                    "side",
                    linked.to_str().unwrap(),
                ],
            );
            let repo = Repo {
                root: linked,
                prefix: ".brewstillery".into(),
            };
            assert_eq!(
                fabrewk_core::record::record_branch(&repo).unwrap(),
                "refs/heads/trunk"
            );
            // But a write command there is refused: the linked tree holds another domain, lock and index (P7B-3).
            let main = begin(&f, f.repo());
            let domain = repo.root.join(".brewstillery");
            let e = Session::begin(&domain, Some(repo.clone()), &f.verifier())
                .err()
                .unwrap();
            assert_eq!(e.code(), Some(Code::NotARepository));
            assert!(!domain.join("local").exists());
            drop(main);
            // So is one whose root is a subdirectory of the main working tree.
            std::fs::create_dir_all(f.root.join("sub/.brewstillery")).unwrap();
            let sub = Repo {
                root: f.root.join("sub"),
                prefix: ".brewstillery".into(),
            };
            let e = Session::begin(&f.root.join("sub/.brewstillery"), Some(sub), &f.verifier())
                .err()
                .unwrap();
            assert_eq!(e.code(), Some(Code::NotARepository));
        },
    );
}

// ---- REQ-REG-6: random damage --------------------------------------------------------------------------------------

#[test]
fn req_reg_6_random_damage_is_always_refused_by_a_code() {
    use proptest::prelude::*;
    let f = Fixture::new("damage", false);
    let mut s = begin(&f, None);
    project(&f, &mut s);
    task(&f, &mut s);
    s.finish(&f.verifier(), &[]).unwrap();
    let mut files: Vec<PathBuf> = ok_replay(&f)
        .streams
        .values()
        .flatten()
        .map(|e| e.path.clone())
        .collect();
    files.sort();
    let allowed = [
        Code::RecordVersionUnknown,
        Code::RecordDigestMismatch,
        Code::RecordNoncanonical,
        Code::RecordBroken,
        Code::RecordFork,
        Code::RecordSignatureInvalid,
        Code::RecordTransitionInvalid,
        Code::RecordIdCollision,
    ];
    let config = ProptestConfig {
        cases: 64,
        failure_persistence: None,
        ..ProptestConfig::default()
    };
    proptest!(config, |(which in 0..files.len(), at in any::<prop::sample::Index>(), byte in any::<u8>(), cut in any::<bool>())| {
        let path = &files[which];
        let good = std::fs::read(path).unwrap();
        let mut bad = good.clone();
        let i = at.index(bad.len());
        if cut { bad.truncate(i) } else { bad[i] = if bad[i] == byte { byte.wrapping_add(1) } else { byte } }
        std::fs::write(path, &bad).unwrap();
        let result = replay(&f.domain, &f.verifier());
        std::fs::write(path, &good).unwrap();
        let errors = result.expect_err("damage accepted");
        prop_assert!(errors.iter().all(|e| e.code().is_some_and(|c| allowed.contains(&c))));
    });
}

// ---- Failures that are not absence ---------------------------------------------------------------------------------

#[test]
fn an_unreadable_place_is_an_error_never_an_absence() {
    let f = Fixture::new("unreadable", false);
    // `data/events` that is a file: neither an absent record nor an empty one.
    std::fs::create_dir_all(f.domain.join("data")).unwrap();
    std::fs::write(f.domain.join("data/events"), b"").unwrap();
    let errors = replay(&f.domain, &f.verifier()).unwrap_err();
    assert!(errors.iter().all(|e| e.code().is_none()), "{errors:?}");
    std::fs::remove_file(f.domain.join("data/events")).unwrap();
    // A stream that is a file, read by a writer's head.
    let s = begin(&f, None);
    std::fs::create_dir_all(f.domain.join("data/events")).unwrap();
    std::fs::write(f.stream_dir("project"), b"").unwrap();
    assert_eq!(s.head("project").unwrap_err().code(), None);
    drop(s);
    std::fs::remove_file(f.stream_dir("project")).unwrap();
    // A mark that is a directory.
    let mut s = begin(&f, None);
    s.append("project", None, &fabrewk("installed", json!({})), None)
        .unwrap();
    s.finish(&f.verifier(), &[]).unwrap();
    let mark = f.domain.join("local/cache/heads.json");
    std::fs::remove_file(&mark).unwrap();
    std::fs::create_dir(&mark).unwrap();
    assert_eq!(
        check_mark(&f.domain, &ok_replay(&f)).unwrap_err().code(),
        None
    );
}

#[test]
fn refusal_record_broken_names() {
    let f = Fixture::new("names-broken", false);
    let bytes = encode(&fabrewk("installed", json!({})).build("project", 1, None)).unwrap();
    let digest = &sha256_hex(&bytes)[..16];
    for name in [
        format!("+00001-{digest}.json"),
        format!("000000-{digest}.json"),
        format!("00001-{digest}.json"),
        format!("000001-{}.json", &digest[..15]),
    ] {
        std::fs::create_dir_all(f.stream_dir("project")).unwrap();
        std::fs::write(f.stream_dir("project").join(&name), &bytes).unwrap();
        assert_eq!(
            codes(replay(&f.domain, &f.verifier())),
            [Code::RecordBroken],
            "{name}"
        );
        std::fs::remove_dir_all(f.stream_dir("project")).unwrap();
    }
}

#[test]
fn pd_02_a_mark_at_position_zero_is_invalid() {
    let f = Fixture::new("mark-zero", false);
    let mut s = begin(&f, None);
    let h = s
        .append("project", None, &fabrewk("installed", json!({})), None)
        .unwrap();
    s.finish(&f.verifier(), &[]).unwrap();
    let mark = f.domain.join("local/cache/heads.json");
    let zero = encode(&json!({"schema": 1, "heads": {"project": {"seq": 0, "digest": h.digest}}}))
        .unwrap();
    std::fs::write(&mark, zero).unwrap();
    assert_eq!(
        code(&check_mark(&f.domain, &ok_replay(&f)).unwrap_err()),
        Some(Code::MarkInvalid)
    );
    let bad_stream =
        encode(&json!({"schema": 1, "heads": {"Project": {"seq": 1, "digest": h.digest}}}))
            .unwrap();
    std::fs::write(&mark, bad_stream).unwrap();
    assert_eq!(
        code(&check_mark(&f.domain, &ok_replay(&f)).unwrap_err()),
        Some(Code::MarkInvalid)
    );
}

#[test]
fn req_reg_5_the_session_reports_the_record_and_a_forked_head() {
    let f = Fixture::new("session-record", false);
    let mut s = begin(&f, None);
    assert!(s.record().is_none());
    let h = s
        .append(
            "project",
            None,
            &fabrewk("installed", json!({"n": 1})),
            None,
        )
        .unwrap();
    s.finish(&f.verifier(), &[]).unwrap();
    let s = begin(&f, None);
    assert_eq!(s.record().unwrap().streams["project"].len(), 1);
    // Two events at the next position, written meanwhile by writers that ignore the lock.
    f.forge(
        "project",
        2,
        Some(&h.digest),
        &fabrewk("installed", json!({"n": 2})),
        None,
    );
    f.forge(
        "project",
        2,
        Some(&h.digest),
        &fabrewk("installed", json!({"n": 3})),
        None,
    );
    assert_eq!(
        code(&s.head("project").unwrap_err()),
        Some(Code::RecordFork)
    );
}

#[test]
fn req_reg_1_a_stream_holds_at_most_999999_events() {
    let f = Fixture::new("full", false);
    let mut s = begin(&f, None);
    let last = f.forge(
        "project",
        999_998,
        Some(&"0".repeat(64)),
        &fabrewk("installed", json!({})),
        None,
    );
    let head = Head {
        seq: 999_998,
        digest: sha256_hex(&std::fs::read(last).unwrap()),
    };
    let head = s
        .append(
            "project",
            Some(&head),
            &fabrewk("installed", json!({})),
            None,
        )
        .unwrap();
    assert_eq!(head.seq, 999_999);
    let e = s
        .append(
            "project",
            Some(&head),
            &fabrewk("installed", json!({})),
            None,
        )
        .unwrap_err();
    assert_eq!(code(&e), Some(Code::RecordBroken));
}

#[test]
fn refusal_tool_missing() {
    if std::env::var_os("FABREWK_TEST_NO_TOOLS").is_none() {
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "refusal_tool_missing",
                "--exact",
                "--nocapture",
                "--test-threads=1",
            ])
            .env(
                "FABREWK_TEST_NO_TOOLS",
                Fixture::new("no-tools", false).root,
            )
            .env("PATH", "/nonexistent")
            .output()
            .unwrap();
        let text = String::from_utf8_lossy(&out.stdout);
        assert!(
            out.status.success() && text.contains("test refusal_tool_missing ... ok"),
            "{text}"
        );
        return;
    }
    let root = PathBuf::from(std::env::var_os("FABREWK_TEST_NO_TOOLS").unwrap());
    let signer = fabrewk_core::record::KeySigner(root.parent().unwrap().join("keys/xel"));
    let e = fabrewk_core::record::Sign::sign(&signer, b"x").unwrap_err();
    assert_eq!(e.code(), Some(Code::ToolMissing));
    let repo = Repo {
        root,
        prefix: ".brewstillery".into(),
    };
    assert_eq!(
        fabrewk_core::record::record_branch(&repo)
            .unwrap_err()
            .code(),
        Some(Code::ToolMissing)
    );
}

#[test]
fn req_reg_3_the_writer_never_writes_what_the_reader_refuses() {
    // P7B-2: nesting is bounded, the same way when writing and when reading.
    let f = Fixture::new("depth", false);
    let nest = |n: usize| (0..n).fold(json!("leaf"), |v, _| json!({"n": v}));
    // The event is depth 1 and its payload depth 2: a payload of 63 objects reaches the limit, 64 pass it.
    let mut s = begin(&f, None);
    let h = s
        .append(
            "project",
            None,
            &fabrewk("installed", nest(fabrewk_core::record::MAX_DEPTH - 1)),
            None,
        )
        .unwrap();
    let before = snapshot(&f.domain);
    let e = s
        .append(
            "project",
            Some(&h),
            &fabrewk("installed", nest(fabrewk_core::record::MAX_DEPTH)),
            None,
        )
        .unwrap_err();
    assert_eq!(code(&e), Some(Code::RecordNoncanonical));
    assert_eq!(snapshot(&f.domain), before);
    s.finish(&f.verifier(), &[]).unwrap();
    begin(&f, None);
}

#[test]
fn req_reg_10_a_git_stopped_inside_the_record_commit_leaves_nothing_that_blocks() {
    isolated(
        "req_reg_10_a_git_stopped_inside_the_record_commit_leaves_nothing_that_blocks",
        || {
            // P7B-1: what a Git killed while writing the temporary index leaves.
            let f = Fixture::new("git-killed", true);
            let mut s = begin(&f, f.repo());
            project(&f, &mut s);
            s.finish(&f.verifier(), &[]).unwrap();
            let tmp = f.domain.join("local/tmp");
            std::fs::write(tmp.join("record-index.lock"), b"partial").unwrap();
            std::fs::write(tmp.join("record-index"), b"partial").unwrap();
            let mut s = begin(&f, f.repo());
            assert!(!tmp.join("record-index.lock").exists() && !tmp.join("record-index").exists());
            let h = s.head("project").unwrap();
            s.append(
                "project",
                h.as_ref(),
                &fabrewk("installed", json!({})),
                None,
            )
            .unwrap();
            s.finish(&f.verifier(), &[]).unwrap().unwrap();
            assert!(uncommitted(&f.repo().unwrap()).unwrap().is_empty());
        },
    );
}

#[test]
fn pd_11_a_lock_git_left_is_named_and_never_removed() {
    isolated("pd_11_a_lock_git_left_is_named_and_never_removed", || {
        // P7B-6: a Git stopped while moving the branch or writing the index leaves its lock in `.git`.
        for lock in [
            ".git/index.lock",
            ".git/HEAD.lock",
            ".git/refs/heads/main.lock",
        ] {
            let f = Fixture::new("git-lock", true);
            let mut s = begin(&f, f.repo());
            project(&f, &mut s);
            let lock = f.root.join(lock);
            std::fs::write(&lock, b"").unwrap();
            let e = s.finish(&f.verifier(), &[]).unwrap_err();
            assert_eq!(
                (
                    e.code(),
                    matches!(&e, fabrewk_core::record::Error::Refused { path: Some(p), .. } if *p == lock)
                ),
                (Some(Code::ToolFailed), true)
            );
            // Reading still works; the next write command refuses the same way and removes nothing.
            ok_replay(&f);
            let mut s = begin(&f, f.repo());
            let e = s.commit_uncommitted(&f.verifier()).unwrap_err();
            let named = matches!(&e, fabrewk_core::record::Error::Refused { path: Some(p), .. } if *p == lock);
            assert_eq!((e.code(), named), (Some(Code::ToolFailed), true), "P7B-8");
            assert!(lock.exists());
            assert_eq!(uncommitted(&f.repo().unwrap()).unwrap().len(), 7);
            drop(s);
            // Once the person removes it, the next write command commits what was left.
            std::fs::remove_file(&lock).unwrap();
            let mut s = begin(&f, f.repo());
            s.commit_uncommitted(&f.verifier()).unwrap().unwrap();
            assert!(uncommitted(&f.repo().unwrap()).unwrap().is_empty());
        }
    });
}

#[test]
fn req_reg_2_an_observed_model_may_be_any_string() {
    // P7B-5: `modelObserved` is a string or null, the empty string included.
    let f = Fixture::new("observed", false);
    let mut s = begin(&f, None);
    let mut draft = agent("sweep-recorded", json!({}));
    draft.actor["modelObserved"] = json!("");
    s.append("fermento-xel", None, &draft, None).unwrap();
    s.finish(&f.verifier(), &[]).unwrap();
    assert_eq!(
        ok_replay(&f).streams["fermento-xel"][0].value["actor"]["modelObserved"],
        json!("")
    );
}
