//! Part 8b at the library's level (core-specification 12: REQ-FER-5 to REQ-FER-7, REQ-FER-8, section 4's incorporated
//! proposals): the units an edit replaces whole, the rules, the difference, the text last approved, and the replay's
//! checks of the streams `alambique` and `rules`.

mod common;

use common::*;
use fabrewk_core::Code;
use fabrewk_core::alambique::{self, Rules, apply, apply_rules, difference, rules_digest};
use fabrewk_core::record::{Draft, Say, Session, Sign, sha256_hex};
use proptest::prelude::*;
use serde_json::{Value, json};

const H: &str = "0000000000000000000000000000000000000000000000000000000000000000";

const DOC: &str = "# Title\n\nOpening text.\n\n## 1. One\n\nIntro.\n\n**1.1. A:** first.\n\n**1.2. B:** second,\n- a list\n\n### Sub\n\n**1.3. C:** third.\n\n```\n## 2. Not a heading\n**1.9. nor a point**\n```\n\n## 2. Two\n\n**2.1. D:** fourth.\n";

fn unit(kind: &str, n: Option<u64>, m: Option<u64>, text: &str) -> Value {
    let mut u = json!({"kind": kind, "text": text});
    u["n"] = json!(n);
    u["m"] = json!(m);
    u.as_object_mut().unwrap().retain(|_, v| !v.is_null());
    u
}

fn applied(text: &str, units: &[Value]) -> Result<String, Code> {
    apply(text, units).map_err(|e| e.code().unwrap())
}

#[test]
fn req_fer_6_the_opening_a_section_and_a_point_are_each_replaced_whole() {
    // The opening runs from the start to the last line that is not blank before section 1.
    let out = applied(DOC, &[unit("opening", None, None, "# New\n\nNew opening.")]).unwrap();
    assert!(out.starts_with("# New\n\nNew opening.\n\n## 1. One\n"), "{out}");
    // A section runs to the next heading of level 1 or 2 outside fences: its subheading and its fence are inside it.
    let out = applied(DOC, &[unit("section", Some(1), None, "## 1. One\n\nGone.")]).unwrap();
    assert_eq!(out, "# Title\n\nOpening text.\n\n## 1. One\n\nGone.\n\n## 2. Two\n\n**2.1. D:** fourth.\n");
    // The last section runs to the end, and the final line break stays.
    let out = applied(DOC, &[unit("section", Some(2), None, "## 2. Two\n\nNone.")]).unwrap();
    assert!(out.ends_with("```\n\n## 2. Two\n\nNone.\n"), "{out}");
    // A point runs to the next point or heading: the list under it is inside; the blank line before the heading stays.
    let out = applied(DOC, &[unit("point", Some(1), Some(2), "**1.2. B:** rewritten.")]).unwrap();
    assert!(out.contains("**1.1. A:** first.\n\n**1.2. B:** rewritten.\n\n### Sub\n"), "{out}");
    assert!(!out.contains("a list"));
    // Several units are each found in the text as it stands, whatever their order.
    let out = applied(DOC, &[unit("point", Some(2), Some(1), "**2.1. D:** new."), unit("opening", None, None, "# T")])
        .unwrap();
    assert!(out.starts_with("# T\n\n## 1. One") && out.ends_with("**2.1. D:** new.\n"), "{out}");
    // A text with CRLF breaks keeps them outside the unit.
    let crlf = DOC.replace('\n', "\r\n");
    let out = applied(&crlf, &[unit("point", Some(1), Some(1), "**1.1. A:** new.")]).unwrap();
    assert!(out.contains("\r\n\r\n**1.1. A:** new.\r\n\r\n**1.2. B:**"), "{out:?}");
}

#[test]
fn refusal_unit_ambiguous_a_unit_not_found_once_or_overlapping() {
    // Inside a fence a heading and a point are not found: section 2 stands once, point 1.9 not at all.
    assert!(applied(DOC, &[unit("section", Some(2), None, "## 2. T")]).is_ok());
    for (what, units) in [
        ("a point only inside a fence", vec![unit("point", Some(1), Some(9), "x")]),
        ("a section that is not there", vec![unit("section", Some(7), None, "x")]),
        ("a point that is not there", vec![unit("point", Some(1), Some(4), "x")]),
        ("a section inside another", vec![unit("section", Some(1), None, "x"), unit("point", Some(1), Some(1), "y")]),
    ] {
        assert_eq!(applied(DOC, &units), Err(Code::UnitAmbiguous), "{what}");
    }
    // A heading that stands twice, a point twice and no section 1 at all.
    let twice = format!("{DOC}\n## 2. Again\n\n**1.1. A:** again.\n");
    assert_eq!(applied(&twice, &[unit("section", Some(2), None, "x")]), Err(Code::UnitAmbiguous));
    assert_eq!(applied(&twice, &[unit("point", Some(1), Some(1), "x")]), Err(Code::UnitAmbiguous));
    assert_eq!(applied("# T\n\n## 2. Only\n", &[unit("opening", None, None, "x")]), Err(Code::UnitAmbiguous));
    // A point's number is exact: `1.1` is not `1.10`, and `**1.1` without its dot is none.
    let numbers = "## 1. S\n\n**1.10. A:** a.\n\n**1.1 B** b.\n";
    assert_eq!(applied(numbers, &[unit("point", Some(1), Some(1), "x")]), Err(Code::UnitAmbiguous));
    assert!(applied(numbers, &[unit("point", Some(1), Some(10), "x")]).is_ok());
    // An opening may be empty: the text begins with section 1.
    assert_eq!(applied("## 1. S\n", &[unit("opening", None, None, "# T\n")]).unwrap(), "# T\n## 1. S\n");
}

proptest! {
    #[test]
    fn req_fer_6_a_unit_replaced_with_itself_changes_nothing(
        pieces in proptest::collection::vec("[a-z ]{0,12}", 3..8),
        pick in 0usize..4,
    ) {
        // A document of two sections, the first with points made of `pieces`; replacing a unit with its own text is the
        // identity, so the units' spans are exactly what they say.
        let mut doc = String::from("# T\n\nOpen.\n\n## 1. S\n\n");
        for (i, p) in pieces.iter().enumerate() {
            doc += &format!("**1.{}. P{i}:** {p}\n\n", i + 1);
        }
        doc += "## 2. E\n\n**2.1. Z:** end.\n";
        let n = pieces.len();
        let own = |a: usize, b: usize| doc[a..b].to_owned();
        let (kind, num, m) = match pick {
            0 => ("opening", None, None),
            1 => ("section", Some(1), None),
            2 => ("section", Some(2), None),
            _ => ("point", Some(1), Some(n as u64)),
        };
        let probe = apply(&doc, &[unit(kind, num, m, "\u{1}")]).unwrap();
        let start = probe.find('\u{1}').unwrap();
        let end = doc.len() - (probe.len() - start - 1);
        let back = apply(&doc, &[unit(kind, num, m, &own(start, end))]).unwrap();
        prop_assert_eq!(back, doc);
    }
}

fn rules(pairs: &[(&str, &str)]) -> Rules {
    pairs.iter().map(|(k, v)| ((*k).to_owned(), (*v).to_owned())).collect()
}

#[test]
fn req_fer_6_rules_are_added_replaced_and_removed_whole() {
    let none = Rules::new();
    let one =
        apply_rules(&none, &[unit("rule", Some(1), None, "Be brief."), unit("rule", Some(10), None, "Ten.")]).unwrap();
    assert_eq!(one, rules(&[("R1", "Be brief."), ("R10", "Ten.")]));
    let removed = json!({"kind": "rule", "n": 10, "text": null});
    let two = apply_rules(&one, &[unit("rule", Some(1), None, "Be shorter."), removed.clone()]).unwrap();
    assert_eq!(two, rules(&[("R1", "Be shorter.")]));
    // A rule removed that is not there is not found exactly once.
    let e = apply_rules(&two, &[removed]).unwrap_err();
    assert_eq!(e.code(), Some(Code::UnitAmbiguous));
    // The digest is that of the canonical JSON, `{}` for none, and follows the text.
    assert_eq!(rules_digest(&none), sha256_hex(b"{}"));
    assert_ne!(rules_digest(&one), rules_digest(&two));
    assert_eq!(rules_digest(&two), rules_digest(&rules(&[("R1", "Be shorter.")])));
}

#[test]
fn req_fer_7_a_direct_edit_records_one_hunk() {
    let hunk = |old: &str, new: &str| difference(old, new);
    assert_eq!(hunk("a\nb\nc\n", "a\nB\nc\n"), json!({"from": 2, "removed": ["b"], "added": ["B"]}));
    assert_eq!(hunk("a\nc\n", "a\nb\nc\n"), json!({"from": 2, "removed": [], "added": ["b"]}));
    assert_eq!(hunk("a\nb\nc\n", "a\nc\n"), json!({"from": 2, "removed": ["b"], "added": []}));
    assert_eq!(
        hunk("a\nb\nc\nd\ne\n", "a\nB\nc\nD\ne\n"),
        json!({"from": 2, "removed": ["b", "c", "d"], "added": ["B", "c", "D"]})
    );
    assert_eq!(hunk("", "x\ny\n"), json!({"from": 1, "removed": [], "added": ["x", "y"]}));
    // Equal ends are not counted twice: a line repeated at both ends of a change stays whole.
    assert_eq!(hunk("a\na\n", "a\na\na\n"), json!({"from": 3, "removed": [], "added": ["a"]}));
    assert_eq!(hunk("a\r\nb\r\n", "a\r\nc\r\n"), json!({"from": 2, "removed": ["b"], "added": ["c"]}));
}

proptest! {
    #[test]
    fn req_fer_7_the_hunk_turns_the_old_text_into_the_new(
        old in proptest::collection::vec("[ab]{0,2}", 0..8),
        new in proptest::collection::vec("[ab]{0,2}", 0..8),
    ) {
        let join = |v: &[String]| v.iter().map(|l| format!("{l}\n")).collect::<String>();
        let (a, b) = (join(&old), join(&new));
        let d = difference(&a, &b);
        let from = d["from"].as_u64().unwrap() as usize - 1;
        let strings = |k: &str| d[k].as_array().unwrap().iter().map(|l| l.as_str().unwrap().to_owned()).collect::<Vec<_>>();
        let (removed, added) = (strings("removed"), strings("added"));
        prop_assert_eq!(&old[from..from + removed.len()], &removed[..]);
        let mut rebuilt = old.clone();
        rebuilt.splice(from..from + removed.len(), added);
        prop_assert_eq!(rebuilt, new);
    }
}

fn agent(kind: &str, payload: Value) -> Draft {
    let actor = json!({"kind": "agent", "tool": "claude-code", "provider": "anthropic", "modelRequested": "m", "modelObserved": null});
    Draft { kind: kind.into(), actor, policy: H.into(), instance: "i".into(), say: None, payload }
}

fn person(kind: &str, payload: Value) -> Draft {
    let actor = json!({"kind": "person", "alias": "xel", "role": "maestro"});
    let say = Some(Say { text: "Lo decido.".into(), lang: "es-MX".into() });
    Draft { kind: kind.into(), actor, policy: H.into(), instance: "i".into(), say, payload }
}

/// Appends `drafts` to `stream` and finishes; the code of the first refusal if the record no longer replays.
fn put(f: &Fixture, stream: &str, drafts: &[Draft]) -> Result<(), Code> {
    let code = |e: fabrewk_core::Error| e.code().unwrap();
    let mut s = Session::begin(&f.domain, None, &f.verifier()).map_err(code)?;
    let mut head = s.head(stream).unwrap();
    for d in drafts {
        let signer = d.say.is_some().then(|| f.signer());
        let signer = signer.as_ref().map(|s| s as &dyn Sign);
        head = Some(s.append(stream, head.as_ref(), d, signer).map_err(code)?);
    }
    s.finish(&f.verifier(), &[]).map(drop).map_err(code)
}

fn digest(n: u8) -> String {
    sha256_hex(&[n])
}

/// A proposal recorded for the alambique (or the rules) from `base`, predicting `result`.
fn proposal(id: &str, target: &str, base: &str, result: &str, units: Value) -> Draft {
    let text = format!("{id} point");
    agent(
        "proposal-recorded",
        json!({"id": id, "point": {"number": 1, "digest": sha256_hex(text.as_bytes())}, "target": target,
               "base": base, "units": units, "keys": [], "result": result}),
    )
}

fn point_unit() -> Value {
    json!([{"kind": "point", "n": 6, "m": 1, "text": "**6.1.** A."}])
}

#[test]
fn req_fer_5_the_alambique_stream_opens_with_its_baseline_and_each_step_starts_from_the_text_approved() {
    let (a, b, c) = (digest(1), digest(2), digest(3));
    let sweep = agent("sweep-recorded", json!({"input": H, "points": [], "heads": {}, "skills": []}));
    let step = |name: &str, drafts: Vec<Draft>, want: Option<Code>| {
        let f = Fixture::new(name, false);
        put(&f, "fermento-xel", &[sweep.clone(), proposal("p-aaaaaaaa", "alambique", &a, &b, point_unit())]).unwrap();
        assert_eq!(put(&f, "alambique", &drafts), want.map_or(Ok(()), Err), "{name}");
    };
    let base = || person("alambique-baseline", json!({"digest": a}));
    let direct = |old: &str, new: &str| {
        person(
            "direct-edit-approved",
            json!({"old": old, "new": new, "difference": {"from": 1, "removed": [], "added": ["x"]}}),
        )
    };
    let incorporated = |id: &str, result: &str| person("proposal-incorporated", json!({"id": id, "result": result}));
    let bad = Some(Code::RecordTransitionInvalid);
    step("ok", vec![base(), incorporated("p-aaaaaaaa", &b), direct(&b, &c)], None);
    step("no-baseline", vec![direct(&a, &b)], bad);
    step("baseline-twice", vec![base(), base()], bad);
    step("direct-from-another-text", vec![base(), direct(&b, &c)], bad);
    step("direct-changes-nothing", vec![base(), direct(&a, &a)], bad);
    step("proposal-from-another-text", vec![base(), direct(&a, &c), incorporated("p-aaaaaaaa", &b)], bad);
    step("proposal-with-another-result", vec![base(), incorporated("p-aaaaaaaa", &c)], bad);
    step("unknown-proposal", vec![base(), incorporated("p-bbbbbbbb", &b)], bad);
    // A payload of the wrong shape is refused where it is written.
    step("shape", vec![person("alambique-baseline", json!({"digest": 1}))], Some(Code::RecordVersionUnknown));
    step("content", vec![person("alambique-baseline", json!({"digest": "short"}))], bad);
}

#[test]
fn req_fer_6_a_rules_proposal_is_recorded_in_the_stream_rules_with_the_rules_the_units_give() {
    let (empty, one) = (rules_digest(&Rules::new()), rules_digest(&rules(&[("R1", "Be brief.")])));
    let units = json!([{"kind": "rule", "n": 1, "text": "Be brief."}]);
    let sweep = agent("sweep-recorded", json!({"input": H, "points": [], "heads": {}, "skills": []}));
    let step = |name: &str, target: &str, base: &str, result: &str, payload: Value, want: Option<Code>| {
        let f = Fixture::new(name, false);
        let units = if target == "rules" { units.clone() } else { point_unit() };
        let p = proposal("p-aaaaaaaa", target, base, result, units);
        put(&f, "fermento-xel", &[sweep.clone(), p]).unwrap();
        let d = person("proposal-incorporated", payload);
        assert_eq!(put(&f, "rules", &[d]), want.map_or(Ok(()), Err), "{name}");
    };
    let right = json!({"id": "p-aaaaaaaa", "result": one, "rules": {"R1": "Be brief."}});
    let bad = Some(Code::RecordTransitionInvalid);
    step("ok", "rules", &empty, &one, right.clone(), None);
    step(
        "another-text",
        "rules",
        &empty,
        &one,
        json!({"id": "p-aaaaaaaa", "result": one, "rules": {"R1": "Other."}}),
        bad,
    );
    step(
        "another-digest",
        "rules",
        &empty,
        &one,
        json!({"id": "p-aaaaaaaa", "result": empty, "rules": {"R1": "Be brief."}}),
        bad,
    );
    step("not-from-empty", "rules", &one, &one, right.clone(), bad);
    step("predicted-another", "rules", &empty, &empty, right.clone(), bad);
    step("alambique-target", "alambique", &empty, &one, right, bad);
    step(
        "shape",
        "rules",
        &empty,
        &one,
        json!({"id": "p-aaaaaaaa", "result": one, "rules": {"R1": 1}}),
        Some(Code::RecordVersionUnknown),
    );
    // The record gives the rules and their digest back.
    let f = Fixture::new("held", false);
    put(&f, "fermento-xel", &[sweep, proposal("p-aaaaaaaa", "rules", &empty, &one, units)]).unwrap();
    put(
        &f,
        "rules",
        &[person("proposal-incorporated", json!({"id": "p-aaaaaaaa", "result": one, "rules": {"R1": "Be brief."}}))],
    )
    .unwrap();
    assert_eq!(alambique::rules(&f.replay()), rules(&[("R1", "Be brief.")]));
}

#[test]
fn req_fer_7_the_text_last_approved_is_the_baseline_a_result_or_a_direct_edit() {
    let f = Fixture::new("approved", false);
    let record = |f: &Fixture| f.replay_or_empty();
    assert_eq!(alambique::approved(&record(&f)), None);
    let (a, b, c) = (digest(1), digest(2), digest(3));
    let sweep = agent("sweep-recorded", json!({"input": H, "points": [], "heads": {}, "skills": []}));
    put(&f, "fermento-xel", &[sweep, proposal("p-aaaaaaaa", "alambique", &a, &b, point_unit())]).unwrap();
    put(&f, "alambique", &[person("alambique-baseline", json!({"digest": a}))]).unwrap();
    assert_eq!(alambique::approved(&record(&f)), Some(a));
    put(&f, "alambique", &[person("proposal-incorporated", json!({"id": "p-aaaaaaaa", "result": b}))]).unwrap();
    assert_eq!(alambique::approved(&record(&f)), Some(b.clone()));
    let hunk = json!({"from": 1, "removed": null, "added": ["x"]});
    put(&f, "alambique", &[person("direct-edit-approved", json!({"old": b, "new": c, "difference": hunk}))]).unwrap();
    assert_eq!(alambique::approved(&record(&f)), Some(c));
}

#[test]
fn req_fer_7_c7_passes_with_nothing_on_either_side_and_refuses_every_other_difference() {
    let f = Fixture::new("check", false);
    let file = f.domain.join("ALAMBIQUE.md");
    let unapproved = |f: &Fixture| alambique::check(&f.replay_or_empty(), &f.domain).map_err(|e| e.code());
    let why = |f: &Fixture| alambique::check(&f.replay_or_empty(), &f.domain).unwrap_err().detail();
    // Nothing approved and no file: nothing to approve.
    assert_eq!(unapproved(&f), Ok(()));
    // A file nothing approved.
    std::fs::write(&file, "text\n").unwrap();
    assert_eq!(unapproved(&f), Err(Some(Code::AlambiqueUnapproved)));
    assert!(why(&f).contains("never approved"));
    put(&f, "alambique", &[person("alambique-baseline", json!({"digest": sha256_hex(b"text\n")}))]).unwrap();
    assert_eq!(unapproved(&f), Ok(()));
    // A byte changed, then restored; a file gone; a directory or a link in its place.
    std::fs::write(&file, "text \n").unwrap();
    assert_eq!(unapproved(&f), Err(Some(Code::AlambiqueUnapproved)));
    assert!(why(&f).contains("differs"));
    std::fs::write(&file, "text\n").unwrap();
    assert_eq!(unapproved(&f), Ok(()));
    std::fs::remove_file(&file).unwrap();
    assert_eq!(unapproved(&f), Err(Some(Code::AlambiqueUnapproved)));
    assert!(why(&f).contains("gone"));
    std::fs::create_dir(&file).unwrap();
    assert_eq!(unapproved(&f), Err(Some(Code::AlambiqueUnapproved)));
    std::fs::remove_dir(&file).unwrap();
    std::fs::write(f.root.join("elsewhere.md"), "text\n").unwrap();
    std::os::unix::fs::symlink("../elsewhere.md", &file).unwrap();
    assert_eq!(unapproved(&f), Err(Some(Code::AlambiqueUnapproved)));
    // A file that cannot be read is the operating system's error, which `check` words as unknown.
    std::fs::remove_file(&file).unwrap();
    std::fs::write(&file, "text\n").unwrap();
    std::fs::set_permissions(&file, std::os::unix::fs::PermissionsExt::from_mode(0o000)).unwrap();
    assert_eq!(unapproved(&f), Err(None));
    std::fs::set_permissions(&file, std::os::unix::fs::PermissionsExt::from_mode(0o644)).unwrap();
}

trait Replayed {
    fn replay_or_empty(&self) -> fabrewk_core::record::Replay;
}

impl Replayed for Fixture {
    /// The record as it replays now, or an empty one before the first event.
    fn replay_or_empty(&self) -> fabrewk_core::record::Replay {
        fabrewk_core::record::replay(&self.domain, &self.verifier()).unwrap_or_default()
    }
}
