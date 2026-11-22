//! Part 11a (D-113 point 5): the stream `bootstrap` held to the plan that imports it, on a record built in memory, at
//! the table's level: every binding replay holds, each broken alone, refused `RECORD_TRANSITION_INVALID`; a prefix
//! stands while no adoption is written; the first adoption's seal holds the whole stream.

use fabrewk_core::Code;
use fabrewk_core::bootstrap::DECISIONS;
use fabrewk_core::record::{Event, Replay, encode, sha256_hex, subject_digest};
use serde_json::{Value, json};
use std::path::PathBuf;

const POLICY: &str = "[project]\nname = \"x\"\n";

const TEXTS: [&str; 3] =
    ["# D\n\nThe opening.\n\n", "## D-001 — One\n\n- Status: x\n\n", "## R-001 — Two\n\n- Status: y\n"];

/// The opening's and the entries' payloads, as the installer's split gives them for [`TEXTS`].
fn payloads() -> Vec<Value> {
    vec![
        json!({"key": null, "title": null, "fields": [], "text": TEXTS[0]}),
        json!({"key": "D-001", "title": "One", "fields": [["Status", "x"]], "text": TEXTS[1]}),
        json!({"key": "R-001", "title": "Two", "fields": [["Status", "y"]], "text": TEXTS[2]}),
    ]
}

fn policy() -> String {
    sha256_hex(POLICY.as_bytes())
}

/// The plan that imports `entries` (as it names them) with `decisions` the digest it gives `decisions.md`.
fn plan(entries: &[Value], decisions: &str) -> Value {
    let named: Vec<Value> =
        entries.iter().map(|p| json!({"key": p["key"], "title": p["title"], "digest": subject_digest(p)})).collect();
    json!({
        "answers": {"publication": "published"},
        "files": [{"path": ".brewstillery/settings.toml", "action": "keep", "content": POLICY, "digest": policy()}],
        "import": {"entries": named, "files": [{"path": DECISIONS, "digest": decisions}]},
    })
}

fn event(stream: &str, seq: usize, kind: &str, payload: Value, prev: Option<&Event>) -> Event {
    let value = json!({
        "schema": 1, "stream": stream, "seq": seq, "prev": prev.map(|p| p.digest.clone()), "kind": kind,
        "at": "2026-11-21T00:00:00Z", "instance": "i", "fabrewk": "0.1.0", "actor": {"kind": "fabrewk"},
        "policy": policy(), "payload": payload,
    });
    let digest = sha256_hex(&encode(&value).unwrap());
    Event { stream: stream.into(), seq: seq as u64, digest, kind: kind.into(), value, path: PathBuf::from(stream) }
}

/// A stream of `kinds` and payloads, each event chained to the one before it.
fn stream(name: &str, events: Vec<(&str, Value)>) -> Vec<Event> {
    let mut out: Vec<Event> = Vec::new();
    for (i, (kind, payload)) in events.into_iter().enumerate() {
        let e = event(name, i + 1, kind, payload, out.last());
        out.push(e);
    }
    out
}

/// A record whose `project` stream opens with an import of `plan`, its `bootstrap` stream `events`, and, with
/// `adopted`, the adoption sealing `seal` and the receipt.
fn record(plan: &Value, events: Vec<(&str, Value)>, adopted: Option<Value>) -> Replay {
    let digest = sha256_hex(&encode(plan).unwrap());
    let mut project = vec![
        ("assessed", json!({"plan": plan, "digest": digest})),
        ("publication-decided", json!({"mode": "published", "warningShown": false})),
    ];
    if let Some(seal) = adopted {
        let adoption = json!({"content": POLICY, "digest": policy(), "heads": seal, "widened": []});
        project.extend([("policy-adopted", adoption), ("installed", json!({"plan": digest, "files": []}))]);
    }
    let mut r = Replay::default();
    r.streams.insert("project".into(), stream("project", project));
    if !events.is_empty() {
        r.streams.insert("bootstrap".into(), stream("bootstrap", events));
    }
    r
}

/// The stream the plan gives: each entry, then `bootstrap-imported`.
fn imported(plan: &Value, entries: &[Value]) -> Vec<(&'static str, Value)> {
    let digest = sha256_hex(&encode(plan).unwrap());
    let mut out: Vec<(&str, Value)> = entries.iter().map(|p| ("entry-imported", p.clone())).collect();
    out.push((
        "bootstrap-imported",
        json!({"plan": digest, "entries": entries.len(), "files": plan["import"]["files"]}),
    ));
    out
}

/// The seal of the stream `events` would make.
fn seal(events: &[(&str, Value)]) -> Value {
    let last = stream("bootstrap", events.to_vec()).pop().unwrap();
    json!({"bootstrap": {"digest": last.digest, "seq": last.seq}})
}

fn decisions() -> String {
    sha256_hex(TEXTS.concat().as_bytes())
}

fn invalid(r: &Replay, what: &str) {
    let errors = fabrewk_core::table::validate(r).expect_err(what);
    assert!(errors.iter().any(|e| e.code() == Some(Code::RecordTransitionInvalid)), "{what}: {errors:?}");
}

#[test]
fn the_import_replays_whole_and_every_prefix_stands_before_the_adoption() {
    let entries = payloads();
    let plan = plan(&entries, &decisions());
    let events = imported(&plan, &entries);
    for n in 0..=events.len() {
        let r = record(&plan, events[..n].to_vec(), None);
        assert!(fabrewk_core::table::validate(&r).is_ok(), "prefix {n}");
    }
    let r = record(&plan, events.clone(), Some(seal(&events)));
    assert_eq!(fabrewk_core::table::validate(&r).map_err(|e| format!("{e:?}")), Ok(()));
    // Once adopted, the stream is whole and sealed at its last event.
    let short = events[..events.len() - 1].to_vec();
    invalid(&record(&plan, short.clone(), Some(seal(&short))), "adopted before the import ended");
    invalid(&record(&plan, events.clone(), Some(json!({}))), "a seal without the stream");
    invalid(&record(&plan, events.clone(), Some(seal(&short))), "a seal short of the stream's end");
    let mut other = seal(&events);
    other["bootstrap"]["digest"] = json!("0".repeat(64));
    invalid(&record(&plan, events.clone(), Some(other)), "a seal naming another digest");
}

#[test]
fn p11o_6_each_binding_of_point_5_broken_alone_is_refused() {
    let entries = payloads();
    let plan_ = plan(&entries, &decisions());
    let good = imported(&plan_, &entries);
    let with = |i: usize, f: &dyn Fn(&mut Value)| {
        let mut events = good.clone();
        f(&mut events[i].1);
        record(&plan_, events.clone(), None)
    };
    invalid(&with(1, &|p| p["key"] = json!("D-002")), "a key changed");
    invalid(&with(1, &|p| p["title"] = json!("Other")), "a title changed");
    invalid(&with(2, &|p| p["fields"][0][1] = json!("z")), "a field's value changed");
    invalid(&with(0, &|p| p["text"] = json!("# D\n")), "the opening's text changed");
    invalid(&with(3, &|p| p["plan"] = json!("0".repeat(64))), "another plan");
    invalid(&with(3, &|p| p["entries"] = json!(2)), "another count");
    invalid(&with(3, &|p| p["files"] = json!([])), "other files");
    // An entry's text moved into the one before it: the texts joined are still the file, the entries not the plan's.
    let mut moved = good.clone();
    moved[1].1["text"] = json!(format!("{}## R-001 — Two\n", TEXTS[1]));
    moved[2].1["text"] = json!("\n- Status: y\n");
    let joined: String = moved[..3].iter().map(|e| e.1["text"].as_str().unwrap()).collect();
    assert_eq!(joined, TEXTS.concat());
    invalid(&record(&plan_, moved, None), "a text moved between entries");
    // The texts, joined, are the file whose digest the plan gives, even with each entry the plan's.
    let elsewhere = plan(&entries, &"0".repeat(64));
    invalid(&record(&elsewhere, imported(&elsewhere, &entries), None), "texts that are not decisions.md");
    // The stream's order: entries, then one `bootstrap-imported`, nothing after it, nothing missing.
    let mut after = good.clone();
    after.push(("entry-imported", entries[2].clone()));
    invalid(&record(&plan_, after, None), "an event after bootstrap-imported");
    let mut twice = good.clone();
    twice.push(good[3].clone());
    invalid(&record(&plan_, twice, None), "two bootstrap-imported");
    let mut missing = good.clone();
    missing.remove(2);
    invalid(&record(&plan_, missing, None), "an entry missing");
    let early = vec![good[0].clone(), good[3].clone()];
    invalid(&record(&plan_, early, None), "bootstrap-imported before the entries end");
    let mut wrong = good.clone();
    wrong[1].0 = "bootstrap-imported";
    invalid(&record(&plan_, wrong, None), "another kind at an entry's place");
}

#[test]
fn the_stream_stands_only_where_the_plan_imports_and_cites_its_project_file() {
    let entries = payloads();
    let plan_ = plan(&entries, &decisions());
    let events = imported(&plan_, &entries);
    let mut bare = plan_.clone();
    bare.as_object_mut().unwrap().remove("import");
    invalid(&record(&bare, events.clone(), None), "a plan that does not import");
    let mut r = record(&plan_, events.clone(), None);
    r.streams.remove("project");
    invalid(&r, "no project stream");
    // An event citing another policy than the project file the plan keeps.
    let mut r = record(&plan_, events, None);
    let e = &mut r.streams.get_mut("bootstrap").unwrap()[1];
    e.value["policy"] = json!("0".repeat(64));
    invalid(&r, "another policy cited");
}
