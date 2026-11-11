//! Part 8a at the library's level (core-specification 12: REQ-FER-1, REQ-FER-2, REQ-FER-4, REQ-FER-10, section 4's
//! proposals): the separator found once outside fences, the points, the payloads, the proposals' states derived from
//! the record, the answer generated from it, and the skills a sweep lists.

mod common;

use common::*;
use fabrewk_core::Code;
use fabrewk_core::detect::Program;
use fabrewk_core::fermento::{self, MARKER, Point};
use fabrewk_core::record::{Draft, Say, Session, Sign, sha256_hex};
use proptest::prelude::*;
use serde_json::{Value, json};
use std::os::unix::fs::PermissionsExt;

const H: &str = "0000000000000000000000000000000000000000000000000000000000000000";

fn block(alias: &str) -> String {
    fermento::block("en", alias)
}

fn split(text: &str) -> Result<fermento::Parts, Code> {
    fermento::split(text, "xel").map_err(|e| e.0)
}

#[test]
fn req_fer_1_the_separator_is_found_once_outside_fences() {
    let input = "# Fermento\n\n1. A point.\n\n";
    let text = format!("{input}{}\n## Answer\n\nText.\n", block("xel"));
    let parts = split(&text).unwrap();
    assert_eq!((parts.input_end, parts.after), (input.len(), input.len() + block("xel").len()));
    assert_eq!(&text[parts.after..], "\n## Answer\n\nText.\n");
    // The catalogue's other rendering is a separator too, and so is a block that ends the file without a break.
    let es = format!("{input}{}", fermento::block("es", "xel"));
    assert!(split(&es).is_ok());
    let bare = format!("{input}{}", block("xel").trim_end());
    assert_eq!(split(&bare).unwrap().after, bare.len());
    // A marker inside a fenced block does not count, whichever fence holds it, and one outside does.
    for fence in ["```", "~~~", "````", "  ```"] {
        let held = format!("{fence}\n{MARKER}\n{}\n", fence.trim());
        assert_eq!(split(&held), Err(Code::SeparatorMissing), "{fence}");
        assert!(split(&format!("{held}{}", block("xel"))).is_ok(), "{fence}");
    }
}

#[test]
fn refusal_separator_missing_ambiguous_and_damaged() {
    let b = block("xel");
    let missing = [
        String::new(),
        "# Fermento\n".into(),
        // Only inside a fence: backticks, tildes, a fence never closed.
        format!("```\n{b}```\n"),
        format!("~~~\n{b}~~~\n"),
        format!("```\n{b}"),
        // A closer shorter than the opener, or of the other mark, does not close.
        format!("````\n{b}```\n"),
        format!("```\n{b}~~~\n"),
        // The marker indented, or with anything else on its line.
        format!(" {MARKER}\n"),
        format!("{MARKER} \n"),
        format!("{}\n", MARKER.replace("v1", "v2")),
    ];
    for text in &missing {
        assert_eq!(split(text), Err(Code::SeparatorMissing), "{text:?}");
    }
    // A fence indented four spaces is code, not a fence; an info string of a backtick fence holds no backtick.
    assert!(split(&format!("    ```\n{b}")).is_ok());
    assert!(split(&format!("```a`b\n{b}")).is_ok());
    // Two markers outside fences, even when one stands in the answer zone.
    assert_eq!(split(&format!("{b}{b}")), Err(Code::SeparatorAmbiguous));
    assert_eq!(split(&format!("{b}\nAnswer\n{MARKER}\n")), Err(Code::SeparatorAmbiguous));
    // A banner line edited, taken away or added, the marker alone, a block of another alias.
    let damaged = [
        b.replace("INPUT", "input"),
        b.lines().filter(|l| !l.starts_with("## The")).collect::<Vec<_>>().join("\n") + "\n",
        format!("{b}#\n").replace("####\n#", "####\n"),
        format!("{MARKER}\n"),
        block("ana"),
        format!("x\n{}", b.replacen("\n#", "\n\n#", 1)),
    ];
    for text in &damaged[..5] {
        assert_eq!(split(text), Err(Code::SeparatorDamaged), "{text:?}");
    }
    assert_eq!(split(&damaged[5]), Err(Code::SeparatorDamaged));
}

/// A reference written from CommonMark's rules for fenced code blocks, apart from the finder: whether each line lies
/// outside every fence.
fn outside(lines: &[String]) -> Vec<bool> {
    let mut open: Option<(u8, usize)> = None;
    let mut out = Vec::new();
    for line in lines {
        let b = line.as_bytes();
        let indent = b.iter().take_while(|c| **c == b' ').count();
        let rest = &b[indent..];
        let mark = rest.first().copied().filter(|c| *c == b'`' || *c == b'~');
        let run = mark.map_or(0, |m| rest.iter().take_while(|c| **c == m).count());
        let fence = (indent < 4 && run >= 3).then(|| (mark.unwrap(), run, &rest[run..]));
        match (open, fence) {
            (None, Some((m, n, tail))) if m == b'~' || !tail.contains(&b'`') => {
                open = Some((m, n));
                out.push(false);
            }
            (None, _) => out.push(true),
            (Some((m, n)), Some((c, k, tail)))
                if c == m && k >= n && tail.iter().all(|c| *c == b' ' || *c == b'\t') =>
            {
                open = None;
                out.push(false);
            }
            (Some(_), _) => out.push(false),
        }
    }
    out
}

const LINES: &[&str] = &[
    "```",
    "````",
    "~~~",
    "~~~~",
    "  ```",
    "   ~~~",
    "    ```",
    "``` rust",
    "```a`b",
    "~~~ a`b",
    "``` ",
    "```x",
    "text",
    "",
    "1. point",
    "# heading",
    MARKER,
    " <!-- fabrewk:fermento-separator v1 -->",
    "<!-- fabrewk:fermento-separator v2 -->",
];

proptest! {
    #[test]
    fn req_fer_1_the_finder_agrees_with_a_reference_fence_scanner(
        lines in proptest::collection::vec(proptest::sample::select(LINES), 0..24),
        end in any::<bool>(),
    ) {
        let owned: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
        let text = owned.join("\n") + if end && !owned.is_empty() { "\n" } else { "" };
        let marks = owned.iter().zip(outside(&owned)).filter(|(l, out)| *out && l.as_str() == MARKER).count();
        // No block stands around the marker, so a single marker is a damaged separator, never a found one.
        let want = match marks {
            0 => Code::SeparatorMissing,
            1 => Code::SeparatorDamaged,
            _ => Code::SeparatorAmbiguous,
        };
        prop_assert_eq!(split(&text), Err(want), "{:?}", text);
    }
}

const PLAIN: &[&str] = &[
    "```",
    "````",
    "~~~",
    "~~~~",
    "  ```",
    "   ~~~",
    "    ```",
    "``` rust",
    "```a`b",
    "~~~ a`b",
    "text",
    "",
    "1. point",
    "# heading",
    "> quoted",
    "- item",
];

proptest! {
    #[test]
    fn req_fer_1_a_separator_after_any_input_is_found_where_the_reference_says(
        input in proptest::collection::vec(proptest::sample::select(PLAIN), 0..16),
        answer in proptest::collection::vec(proptest::sample::select(PLAIN), 0..8),
    ) {
        let lines: Vec<String> = input.iter().map(|l| l.to_string()).collect();
        let head: String = lines.iter().map(|l| format!("{l}\n")).collect();
        let tail: String = answer.iter().map(|l| format!("\n{l}")).collect();
        let text = format!("{head}{}{tail}", block("xel"));
        let all: Vec<String> = text.lines().map(str::to_owned).collect();
        let found = split(&text);
        if outside(&all)[lines.len()] {
            let parts = found.unwrap();
            prop_assert_eq!((parts.input_end, parts.after), (head.len(), head.len() + block("xel").len()));
        } else {
            prop_assert_eq!(found, Err(Code::SeparatorMissing));
        }
    }
}

fn numbered(points: &[Point]) -> Vec<(u32, String)> {
    points.iter().map(|p| (p.number, p.text.clone())).collect()
}

#[test]
fn req_fer_2_points_are_the_top_level_numbered_items_outside_fences() {
    let input = "\
# Fermento

## Proposals

1. First point,
   on two lines.

9. Ninth.
10. Tenth, after nine.

```
11. inside a fence
```

   2. indented, so nested
- 3. a list item
2. Second, with code:

   ```text
   4. not a point
   ```

## Another heading

Text under a heading is no point.
12.no space
13. After a heading
";
    let got = numbered(&fermento::points(input));
    let want: Vec<(u32, String)> = vec![
        (1, "1. First point,\n   on two lines.".into()),
        (9, "9. Ninth.".into()),
        (
            10,
            "10. Tenth, after nine.\n\n```\n11. inside a fence\n```\n\n   2. indented, so nested\n- 3. a list item"
                .into(),
        ),
        (2, "2. Second, with code:\n\n   ```text\n   4. not a point\n   ```".into()),
        (13, "13. After a heading".into()),
    ];
    assert_eq!(got, want);
    // A point's digest is its text's, and the same words under another number are another point.
    let points = fermento::points(input);
    assert_eq!(points[0].digest, sha256_hex("1. First point,\n   on two lines.".as_bytes()));
    assert_eq!(fermento::points("1. same\n2. same\n")[0].digest, sha256_hex(b"1. same"));
    assert_ne!(fermento::points("1. same\n2. same\n")[1].digest, sha256_hex(b"1. same"));
    // Trailing blank lines are not part of the text, and a tab or the end of the line follows the dot.
    assert_eq!(numbered(&fermento::points("1.\n\n\n2.\tx\n")), [(1, "1.".into()), (2, "2.\tx".into())]);
    assert!(fermento::points("").is_empty() && fermento::points("text only\n").is_empty());
    assert!(fermento::points("1234567890. ten digits\n").is_empty());
    // A heading ends a point's text when a space or a tab follows its hashes, and a hash that starts a word does not.
    let headed = fermento::points("1. a\n#tag\n#\tTitle\n2. b\n");
    assert_eq!(numbered(&headed), [(1, "1. a\n#tag".into()), (2, "2. b".into())]);
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

fn recommendation(n: u32, text: &str) -> Value {
    let rec = json!({"point": format!("About {n}"), "clarify": "Say it in one line.", "destination": "alambique 6.1"});
    json!({"number": n, "digest": sha256_hex(text.as_bytes()), "recommendation": rec})
}

fn sweep(points: Vec<Value>, skills: Value) -> Draft {
    agent("sweep-recorded", json!({"input": H, "points": points, "heads": {}, "skills": skills}))
}

fn proposal(id: &str, n: u32, text: &str, unit_text: &str) -> Draft {
    let unit = json!({"kind": "point", "n": 6, "m": 1, "text": unit_text});
    let key = json!({"point": "6.1", "key": "[decidido]"});
    agent(
        "proposal-recorded",
        json!({"id": id, "point": {"number": n, "digest": sha256_hex(text.as_bytes())}, "target": "alambique",
               "base": H, "units": [unit], "keys": [key], "result": H}),
    )
}

/// Appends `drafts` to a stream and finishes; `person` drafts are signed.
fn write(f: &Fixture, stream: &str, drafts: &[Draft]) {
    let mut s = Session::begin(&f.domain, None, &f.verifier()).unwrap();
    let mut head = s.head(stream).unwrap();
    for d in drafts {
        let signer = d.say.is_some().then(|| f.signer());
        head = Some(s.append(stream, head.as_ref(), d, signer.as_ref().map(|s| s as &dyn Sign)).unwrap());
    }
    s.finish(&f.verifier(), &[]).unwrap();
}

#[test]
fn req_fer_4_the_answer_is_generated_from_the_events_in_the_maintainers_language() {
    let f = Fixture::new("answer", false);
    let (one, two) = ("1. First point.", "2. Second point.");
    let skills = json!([{"tool": "claude", "listed": true, "names": ["fermento", "zeta"], "shadows": ["fermento"]},
                        {"tool": "codex", "listed": false, "names": [], "shadows": []},
                        {"tool": "other", "listed": true, "names": [], "shadows": []}]);
    write(
        &f,
        "fermento-xel",
        &[
            sweep(vec![recommendation(1, one), recommendation(2, two)], skills),
            proposal("p-1234abcd", 2, two, "**6.1.** New text with ``` inside."),
        ],
    );
    let record = f.replay();
    let en = fermento::answer(&record, "xel", "en");
    assert!(
        en.starts_with(
            "## Integrator's answer\n\n**1. About 1**\n\nSay it in one line.\n\nDestination: alambique 6.1\n\n"
        ),
        "{en}"
    );
    assert!(en.contains("**2. About 2**"));
    // The proposal shows its state and the command that discards it first, its identifier only in that command and
    // after the text, as a reference; the drafted text is fenced by a fence longer than any backticks it holds.
    let proposal = &en[en.find("### Proposals for a maestro").unwrap()..];
    assert!(proposal.contains("**Point 2, a proposal for the alambique (pending)**\n\nTo approve it, a maestro runs this, saying why with --say:\n\n    fabrewk approve p-1234abcd\n\nTo discard it, a maestro runs this, saying why with --say:\n\n    fabrewk discard p-1234abcd\n\n"), "{proposal}");
    // PD-32: the command is printed without words, so running it as printed refuses.
    assert!(!proposal.contains("--say \"") && !proposal.contains("<your words>"), "{proposal}");
    assert!(
        proposal.contains("Point 6.1, replaced whole:\n\n````\n**6.1.** New text with ``` inside.\n````\n\n"),
        "{proposal}"
    );
    assert!(proposal.contains("Status key: point 6.1 becomes [decidido].\n\nReference: p-1234abcd\n\n"), "{proposal}");
    assert_eq!(en.matches("p-1234abcd").count(), 3);
    // The skills come from the event: the names, a warning for a name an approved skill bears, and `unknown`.
    let skills = &en[en.find("### Skills the tools load").unwrap()..];
    assert!(skills.contains("- claude: fermento, zeta\n  - claude loads a skill named fermento, which is also an approved skill of this project.\n"), "{skills}");
    assert!(skills.contains("- codex: unknown, a folder could not be listed\n- other: none\n"), "{skills}");
    // The same record gives the same text, and Spanish for a Spanish catalogue; a tag with no catalogue falls to `en`.
    assert_eq!(en, fermento::answer(&record, "xel", "en"));
    let es = fermento::answer(&record, "xel", "es");
    assert!(
        es.starts_with(
            "## Respuesta del integrador\n\n**1. About 1**\n\nSay it in one line.\n\nDestino: alambique 6.1\n\n"
        ),
        "{es}"
    );
    assert!(
        es.contains("Para descartarla, un maestro corre esto, diciendo por qué con --say:")
            && es.contains("Para aprobarla, un maestro corre esto, diciendo por qué con --say:")
            && es.contains("pendiente"),
        "{es}"
    );
    // No generated line is the separator's marker, so the next sweep finds it once.
    assert!(!en.lines().any(|l| l == MARKER) && !es.lines().any(|l| l == MARKER));
    // Another maintainer's fermento has nothing to show.
    assert_eq!(
        fermento::answer(&record, "ana", "en"),
        "## Integrator's answer\n\nNo current recommendations: your input has no points, and nothing waits for a maestro.\n"
    );
}

#[test]
fn req_fer_4_a_stale_proposal_and_a_decided_one() {
    let f = Fixture::new("stale", false);
    let (one, two) = ("1. First point.", "2. Second point.");
    write(
        &f,
        "fermento-xel",
        &[
            sweep(vec![recommendation(1, one), recommendation(2, two)], json!([])),
            proposal("p-aaaaaaaa", 2, two, "**6.1.** A."),
            proposal("p-bbbbbbbb", 1, one, "**6.1.** B."),
        ],
    );
    // A later sweep read the second point changed: its proposal is stale, the other still pending.
    write(&f, "fermento-xel", &[sweep(vec![recommendation(1, one), recommendation(2, "2. Changed.")], json!([]))]);
    let en = fermento::answer(&f.replay(), "xel", "en");
    assert!(
        en.contains("Point 2, a proposal for the alambique (stale: the point changed since it was drafted)"),
        "{en}"
    );
    assert!(en.contains("**Point 1, a proposal for the alambique (pending)**"), "{en}");
    // A stale proposal can only be discarded: approving it would be refused.
    assert!(en.contains("fabrewk approve p-bbbbbbbb") && !en.contains("fabrewk approve p-aaaaaaaa"), "{en}");
    // Discarded, a proposal leaves the answer; the order is by point, then identifier.
    write(&f, "fermento-xel", &[person("proposal-discarded", json!({"id": "p-aaaaaaaa"}))]);
    let record = f.replay();
    let en = fermento::answer(&record, "xel", "en");
    assert!(!en.contains("p-aaaaaaaa") && en.contains("p-bbbbbbbb"), "{en}");
    let all = fermento::proposals(&record).unwrap();
    assert_eq!((all["p-aaaaaaaa"].state, all["p-bbbbbbbb"].state), ("discarded", "pending"));
    assert!(!fermento::stale(&all["p-bbbbbbbb"], fermento::last_sweep(&record, "xel")));
    assert!(fermento::stale(&all["p-aaaaaaaa"], fermento::last_sweep(&record, "xel")));
}

#[test]
fn req_reg_6_proposals_are_held_to_the_table_on_replay() {
    let two = "2. Second point.";
    let base = || vec![sweep(vec![recommendation(2, two)], json!([])), proposal("p-aaaaaaaa", 2, two, "**6.1.** A.")];
    let broken = |name: &str, more: Vec<Draft>, stream: &str| {
        let f = Fixture::new(name, false);
        write(&f, "fermento-xel", &base());
        let mut s = Session::begin(&f.domain, None, &f.verifier()).unwrap();
        let mut head = s.head(stream).unwrap();
        for d in &more {
            let signer = d.say.is_some().then(|| f.signer());
            head = Some(s.append(stream, head.as_ref(), d, signer.as_ref().map(|s| s as &dyn Sign)).unwrap());
        }
        s.finish(&f.verifier(), &[]).unwrap_err_or_replay_error(&f)
    };
    // Recorded twice; decided twice; withdrawn after a discard; unknown; another maintainer's; incorporated after a
    // discard.
    let fabrewk = |d: Draft| Draft { actor: json!({"kind": "fabrewk"}), say: None, ..d };
    let cases: Vec<(&str, Vec<Draft>, &str)> = vec![
        ("duplicate", vec![sweep(vec![recommendation(2, two), recommendation(2, two)], json!([]))], "fermento-xel"),
        (
            "withdrawn-elsewhere",
            vec![fabrewk(agent("proposal-withdrawn", json!({"id": "p-aaaaaaaa", "why": "point-gone"})))],
            "fermento-ana",
        ),
        ("twice", vec![proposal("p-aaaaaaaa", 2, two, "**6.1.** A.")], "fermento-xel"),
        ("unknown", vec![person("proposal-discarded", json!({"id": "p-cccccccc"}))], "fermento-xel"),
        ("elsewhere", vec![person("proposal-discarded", json!({"id": "p-aaaaaaaa"}))], "fermento-ana"),
        (
            "after-discard",
            vec![
                person("proposal-discarded", json!({"id": "p-aaaaaaaa"})),
                Draft {
                    actor: json!({"kind": "fabrewk"}),
                    say: None,
                    ..agent("proposal-withdrawn", json!({"id": "p-aaaaaaaa", "why": "point-gone"}))
                },
            ],
            "fermento-xel",
        ),
    ];
    for (name, more, stream) in cases {
        assert_eq!(broken(name, more, stream), Code::RecordTransitionInvalid, "{name}");
    }
}

trait ReplayError {
    fn unwrap_err_or_replay_error(self, f: &Fixture) -> Code;
}

impl<T: std::fmt::Debug> ReplayError for Result<T, fabrewk_core::Error> {
    fn unwrap_err_or_replay_error(self, f: &Fixture) -> Code {
        // `finish` replays the whole record before it commits, so a record the table refuses fails there.
        match self {
            Err(e) => e.code().unwrap(),
            Ok(_) => fabrewk_core::record::replay(&f.domain, &f.verifier()).unwrap_err()[0].code().unwrap(),
        }
    }
}

#[test]
fn refusal_answer_invalid_and_the_payloads_of_the_kinds() {
    let answer = |edit: Value| {
        json!({"agent": {"tool": "t", "provider": "p", "model": "m"},
               "points": [{"number": 1, "digest": H, "recommendation": {"point": "a", "clarify": "b", "destination": "c"}, "edit": edit}]})
    };
    let good = json!({"target": "alambique", "base": H, "keys": [], "result": H,
                      "units": [{"kind": "section", "n": 6, "text": "## 6. New\n"}]});
    fermento::check_answer(&answer(good.clone())).unwrap();
    let with = |f: &dyn Fn(&mut Value)| {
        let mut e = good.clone();
        f(&mut e);
        fermento::check_answer(&answer(e)).unwrap_err().code()
    };
    let invalid = Some(Code::AnswerInvalid);
    assert_eq!(with(&|e| e["units"] = json!([])), invalid);
    // A unit its target does not take, one with the numbers its kind does not need, a text that is blank or null.
    assert_eq!(with(&|e| e["units"] = json!([{"kind": "rule", "n": 1, "text": "R1."}])), invalid);
    assert_eq!(with(&|e| e["units"] = json!([{"kind": "opening", "n": 1, "text": "x"}])), invalid);
    assert_eq!(with(&|e| e["units"] = json!([{"kind": "point", "n": 6, "text": "x"}])), invalid);
    assert_eq!(with(&|e| e["units"] = json!([{"kind": "section", "n": 6, "m": 1, "text": "x"}])), invalid);
    assert_eq!(with(&|e| e["units"] = json!([{"kind": "section", "n": 6, "text": null}])), invalid);
    assert_eq!(with(&|e| e["units"] = json!([{"kind": "section", "n": 6, "text": " "}])), invalid);
    let twice = json!({"kind": "section", "n": 6, "text": "x"});
    assert_eq!(with(&|e| e["units"] = json!([twice, twice])), invalid);
    // A rules edit holds rules only, and a rule may be removed with `null`.
    let rules = |units: Value| {
        fermento::check_answer(&answer(json!({"target": "rules", "base": H, "keys": [], "result": H, "units": units})))
    };
    rules(json!([{"kind": "rule", "n": 4, "text": null}, {"kind": "rule", "n": 5, "text": "R5. Be brief."}])).unwrap();
    assert_eq!(rules(json!([{"kind": "section", "n": 4, "text": "x"}])).unwrap_err().code(), invalid);
    // A target no edit has, a field of the wrong type or an unknown one; a repeated number, the marker as a line.
    assert_eq!(with(&|e| e["target"] = json!("elsewhere")), invalid);
    assert_eq!(with(&|e| e["extra"] = json!(1)), invalid);
    assert_eq!(with(&|e| e["keys"] = json!("none")), invalid);
    // In a recorded payload the same shape is the record's own failure, which the version check names.
    let recorded = json!({"id": "p-aaaaaaaa", "point": {"number": 1, "digest": H}, "target": "alambique", "base": H,
                          "units": [], "keys": [], "result": H, "extra": 1});
    assert_eq!(fermento::check("proposal-recorded", &recorded).unwrap_err().code(), Some(Code::RecordVersionUnknown));
    // Each kind of a fermento holds its shape and its content in the record; a sweep names a point once.
    let rec = |n: u32| json!({"number": n, "digest": H, "recommendation": {"point": "a", "clarify": "b", "destination": "c"}});
    let sweep_of = |points: Value| json!({"input": H, "points": points, "heads": {}, "skills": []});
    assert_eq!(
        fermento::check("sweep-recorded", &json!({"input": 1})).unwrap_err().code(),
        Some(Code::RecordVersionUnknown)
    );
    assert_eq!(fermento::check("sweep-recorded", &sweep_of(json!([rec(1), rec(1)]))).unwrap_err().code(), invalid);
    fermento::check("sweep-recorded", &sweep_of(json!([rec(1), rec(2)]))).unwrap();
    for (kind, bad) in [("proposal-withdrawn", json!({"id": "p-aaaaaaaa"})), ("proposal-discarded", json!({"id": 5}))] {
        assert_eq!(fermento::check(kind, &bad).unwrap_err().code(), Some(Code::RecordVersionUnknown), "{kind}");
    }
    let empty = json!({"id": "p-aaaaaaaa", "point": {"number": 1, "digest": H}, "target": "alambique", "base": H,
                       "units": [], "keys": [], "result": H});
    assert_eq!(fermento::check("proposal-recorded", &empty).unwrap_err().code(), invalid);
    let mut twice = answer(good.clone());
    twice["points"] = json!([twice["points"][0].clone(), twice["points"][0].clone()]);
    assert_eq!(fermento::check_answer(&twice).unwrap_err().code(), invalid);
    let mut marked = answer(good);
    marked["points"][0]["recommendation"]["clarify"] = json!(format!("one\n{MARKER}\ntwo"));
    assert_eq!(fermento::check_answer(&marked).unwrap_err().code(), invalid);
}

fn tool(skills: &'static [&'static str]) -> Program {
    Program { adapter: Some("a"), executable: "fake-tool", version: &[], tokens: false, skills }
}

#[test]
fn req_fer_10_a_sweep_lists_the_names_of_skills_outside_the_project() {
    let base = tmp("skills");
    let (home, domain) = (base.join("home"), base.join("domain"));
    for dir in [
        "home/.tool/skills/alpha",
        "home/.tool/skills/fermento",
        "home/.other/skills/beta",
        "domain/skills/fermento",
        "domain/skills/mine",
    ] {
        std::fs::create_dir_all(base.join(dir)).unwrap();
    }
    // A file, a hidden folder and a link: only folders and links are skills, hidden ones are not; nothing is opened.
    std::fs::write(home.join(".tool/skills/README"), "not a skill").unwrap();
    std::fs::create_dir(home.join(".tool/skills/.system")).unwrap();
    std::os::unix::fs::symlink("alpha", home.join(".tool/skills/linked")).unwrap();
    std::fs::write(home.join(".tool/skills/alpha/SKILL.md"), "secret text of a skill").unwrap();
    let modules =
        [tool(&[".tool/skills", ".other/skills", ".absent/skills"]), tool(&[]), tool(&[".unreadable/skills"])];
    let got = fermento::skills(&modules, Some(&home), &domain);
    assert_eq!(
        got,
        json!([{"tool": "fake-tool", "listed": true, "names": ["alpha", "beta", "fermento", "linked"], "shadows": ["fermento"]},
               {"tool": "fake-tool", "listed": true, "names": [], "shadows": []}]),
    );
    // A folder that cannot be listed is `unknown`, never empty; so is any folder with no home to look under.
    let closed = home.join(".unreadable/skills");
    std::fs::create_dir_all(&closed).unwrap();
    std::fs::set_permissions(&closed, std::fs::Permissions::from_mode(0o000)).unwrap();
    let got = fermento::skills(&modules[2..], Some(&home), &domain);
    std::fs::set_permissions(&closed, std::fs::Permissions::from_mode(0o755)).unwrap();
    assert_eq!(got, json!([{"tool": "fake-tool", "listed": false, "names": [], "shadows": []}]));
    assert_eq!(fermento::skills(&modules[..1], None, &domain)[0]["listed"], json!(false));
    // No module that declares a folder, no entry; and the project's skills folder as a link is not followed.
    assert_eq!(fermento::skills(&[tool(&[])], Some(&home), &domain), json!([]));
    std::fs::rename(domain.join("skills"), base.join("elsewhere")).unwrap();
    std::os::unix::fs::symlink(base.join("elsewhere"), domain.join("skills")).unwrap();
    assert_eq!(fermento::skills(&modules[..1], Some(&home), &domain)[0]["shadows"], json!([]));
    // The skills were not touched.
    assert_eq!(std::fs::read_to_string(home.join(".tool/skills/alpha/SKILL.md")).unwrap(), "secret text of a skill");
}

#[test]
fn req_reg_6_a_decision_in_the_alambique_belongs_to_no_fermento() {
    let two = "2. Second point.";
    let f = Fixture::new("incorporated", false);
    write(
        &f,
        "fermento-xel",
        &[sweep(vec![recommendation(2, two)], json!([])), proposal("p-aaaaaaaa", 2, two, "**6.1.** A.")],
    );
    // The stream opens with the baseline; the proposal starts from the text approved and gives the result it predicted.
    write(
        &f,
        "alambique",
        &[
            person("alambique-baseline", json!({"digest": H})),
            person("proposal-incorporated", json!({"id": "p-aaaaaaaa", "result": H})),
        ],
    );
    let all = fermento::proposals(&f.replay()).unwrap();
    assert_eq!(all["p-aaaaaaaa"].state, "incorporated");
}

#[test]
fn req_reg_6_a_payload_of_the_wrong_shape_is_refused_where_it_is_written_and_one_of_the_wrong_content_is_not() {
    let f = Fixture::new("shapes", false);
    let try_append = |d: Draft| {
        let mut s = Session::begin(&f.domain, None, &f.verifier()).unwrap();
        let head = s.head("fermento-xel").unwrap();
        let signer = d.say.is_some().then(|| f.signer());
        s.append("fermento-xel", head.as_ref(), &d, signer.as_ref().map(|s| s as &dyn Sign))
            .map(|_| ())
            .map_err(|e| e.code())
    };
    let wrong = [
        sweep_with(json!({"input": 1})),
        Draft { actor: json!({"kind": "fabrewk"}), ..agent("proposal-withdrawn", json!({"id": "p-aaaaaaaa"})) },
        person("proposal-discarded", json!({"id": 5})),
        agent("proposal-recorded", json!({"id": "p-aaaaaaaa"})),
    ];
    for d in wrong {
        let kind = d.kind.clone();
        assert_eq!(try_append(d), Err(Some(Code::RecordVersionUnknown)), "{kind}");
    }
    // Content is held on replay with the record's table: written, it passes the shape.
    assert_eq!(try_append(sweep(vec![recommendation(2, "2. x"), recommendation(2, "2. x")], json!([]))), Ok(()));
}

fn sweep_with(payload: Value) -> Draft {
    agent("sweep-recorded", payload)
}

#[test]
fn req_fer_4_a_proposal_waits_in_the_answer_though_the_sweep_names_no_point() {
    let f = Fixture::new("waiting", false);
    let two = "2. Second point.";
    write(&f, "fermento-xel", &[sweep(vec![], json!([])), proposal("p-aaaaaaaa", 2, two, "**6.1.** A.")]);
    let en = fermento::answer(&f.replay(), "xel", "en");
    assert!(en.contains("fabrewk discard p-aaaaaaaa"), "{en}");
}

#[test]
fn refusal_fermento_missing_is_only_for_a_file_that_is_not_there() {
    let domain = tmp("read-error");
    assert_eq!(fermento::read(&domain, "xel").unwrap_err().code(), Some(Code::FermentoMissing));
    // The maintainer's directory is a file: the file system's own error, not a missing fermento.
    std::fs::write(domain.join("xel"), "not a directory").unwrap();
    let got = fermento::read(&domain, "xel").unwrap_err().code();
    assert_ne!(got, Some(Code::FermentoMissing));
}

#[test]
fn req_fer_1_a_break_is_a_line_feed_or_a_carriage_return_and_a_line_feed() {
    let marker = block("xel");
    // A fence closed by a line that ends in CRLF closes, so the separator after it stands outside.
    let closed = format!("```\ncode\n```\r\n{marker}");
    assert!(split(&closed).is_ok());
    // The same, a document broken only by CRLF, and a fence left open by it.
    let all = format!("# Fermento\n\n1. One.\n\n```\ncode\n```\n\n{marker}").replace('\n', "\r\n");
    let parts = fermento::split(&all, "xel").unwrap();
    assert_eq!(fermento::points(&all[..parts.input_end]).len(), 1);
    assert_eq!(split(&format!("```\r\ncode\r\n{marker}")), Err(Code::SeparatorMissing));
}
