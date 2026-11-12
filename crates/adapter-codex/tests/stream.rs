//! Codex's stream and its audit (REQ-GATE-4, PD-25, PD-26; part 9d), on the streams recorded from real dispatches
//! (core-specification 11.2 rule 3): `codex-cli` 0.166.0, `gpt-6-luna` at effort `low`, confined as this adapter
//! launches it, on a copy of a few files. They hold no figure of the account and were kept whole. Every sub-case of
//! PD-26 inserts its anomaly at every position of the recorded review.

use fabrewk_adapter_codex::{ADAPTER, count, words};
use fabrewk_core::adapter::{Report, Tokens};
use fabrewk_core::record::sha256_hex;
use serde_json::{Value, json};

const RECORDED: &str = include_str!("streams/review.jsonl");

fn read(text: &str) -> Report {
    (ADAPTER.read)(text.as_bytes())
}

fn lines() -> Vec<Value> {
    RECORDED.lines().map(|l| serde_json::from_str(l).unwrap()).collect()
}

fn join(events: &[Value]) -> String {
    events.iter().map(|e| e.to_string() + "\n").collect()
}

/// The examinations of the recorded review: `pwd && rg --files …` and `cat … && cat … && python3 …`.
const EXAMINED: u64 = 5;

/// Where the recorded review's events are: its first command's start and completion, its second's, its result's
/// message, and the turn's end, last.
const START: usize = 3;
const DONE: usize = 4;
const RESULT: usize = 7;

fn void(text: &str, what: &str) {
    assert_eq!(read(text), Report::default(), "{what}:\n{text}");
}

/// `events` with `insert` placed at each position from `from` to `to` (inclusive), each variant void.
fn void_at_every(events: &[Value], insert: &[Value], from: usize, to: usize, what: &str) {
    for p in from..=to {
        let mut v = events.to_vec();
        v.splice(p..p, insert.iter().cloned());
        void(&join(&v), &format!("{what} at {p}"));
    }
}

/// Every position at which an event may be inserted into the recorded review and still lie before its end.
fn inner() -> (usize, usize) {
    (2, lines().len() - 1)
}

fn command(id: Value, phase: &str, command: Value, output: &str, exit: Value, status: &str) -> Value {
    json!({"type": phase, "item": {"id": id, "type": "command_execution", "command": command,
        "aggregated_output": output, "exit_code": exit, "status": status}})
}

/// A command that started and completed with status 0, printing `ok`.
fn ran(id: &str, line: &str) -> [Value; 2] {
    [
        command(json!(id), "item.started", json!(line), "", Value::Null, "in_progress"),
        command(json!(id), "item.completed", json!(line), "ok\n", json!(0), "completed"),
    ]
}

/// The recorded review with its commands replaced by `commands`.
fn with(commands: &[Value]) -> String {
    let mut v = lines();
    v.splice(START..=DONE + 2, commands.iter().cloned());
    join(&v)
}

#[test]
fn pd_26_the_recorded_review_is_pinned_and_audits_as_examined() {
    assert_eq!(sha256_hex(RECORDED.as_bytes()), "e1c90148e797be35787419c379708c265dae454e267445eecf0e58e1e1323ab2");
    let r = read(RECORDED);
    assert_eq!(r.result, Some(json!({"verdict": "accept", "findings": []})));
    assert_eq!(r.examined, EXAMINED);
    // The stream does not say which model answered; the tokens are its turn's, the input less what the cache gave.
    assert_eq!(r.model, None);
    assert_eq!(r.tokens, Some(Tokens { input: Some(29301 - 19072), output: Some(167), cached: Some(19072) }));
}

#[test]
fn pd_25_an_accept_after_no_command_examines_nothing() {
    let r = read(&with(&[]));
    assert_eq!(r.result, Some(json!({"verdict": "accept", "findings": []})));
    assert_eq!(r.examined, 0);
}

#[test]
fn pd_26_1_a_completed_command_with_no_exit_code_is_void() {
    let (from, to) = inner();
    for exit in [Value::Null, json!("0"), json!(0.0)] {
        let done = command(json!("x"), "item.completed", json!("cat sum.py"), "ok\n", exit, "completed");
        void_at_every(&lines(), &[done], from, to, "no exit code");
    }
    // And one without the field at all.
    let mut done = ran("x", "cat sum.py")[1].clone();
    done["item"].as_object_mut().unwrap().remove("exit_code");
    void_at_every(&lines(), &[done], from, to, "no exit code field");
}

#[test]
fn pd_26_2_quoted_separators_do_not_split_and_unquoted_ones_do() {
    assert_eq!(count(r#"/bin/sh -lc "pwd && rg --files -g 'sum.py' -g 'test_sum.py'""#), 2);
    assert_eq!(count("/bin/sh -lc 'cat sum.py && cat test_sum.py && python3 test_sum.py'"), 3);
    assert_eq!(count(r#"/bin/sh -lc "grep 'a;b|c' x""#), 1);
    assert_eq!(count(r#"/bin/bash -c "grep \"a && b\" x; ls""#), 2);
    assert_eq!(count("/bin/sh -lc 'cat a\ncat b'"), 2);
    assert_eq!(count("bash -lc 'a | b || c'"), 3);
    // A command that is not a shell's script counts its own line; one whose quoting does not close counts none.
    assert_eq!(count("rg --files"), 1);
    assert_eq!(count("/bin/sh -lc 'cat a"), 0);
    assert_eq!(count("/bin/sh -x -c 'a; b'"), 1);
    let r = read(&with(&ran("x", "/bin/sh -lc 'cat a; cat b | wc -l'")));
    assert_eq!(r.examined, 3);
}

#[test]
fn pd_26_3_a_second_completion_of_the_same_command_is_void() {
    let (_, to) = inner();
    void_at_every(&lines(), &[lines()[DONE].clone()], DONE + 1, to, "a second completion");
}

#[test]
fn pd_26_4_every_dispatch_is_audited_and_a_stream_without_its_end_examines_nothing() {
    let all = lines();
    for cut in 0..all.len() {
        void(&join(&all[..cut]), &format!("cut at {cut}"));
    }
    // The failed run recorded: the provider refused its request; no result, no usage, no examination.
    void(include_str!("streams/failed.jsonl"), "the recorded failed run");
}

#[test]
fn pd_26_5_a_bare_item_or_event_is_void() {
    let (from, to) = inner();
    for bare in [
        json!({"type": "item.completed"}),
        json!({"type": "item.completed", "item": {}}),
        json!({"type": "item.completed", "item": {"id": "x"}}),
        json!({"type": "item.completed", "item": {"type": "command_execution"}}),
        json!({"type": "item.started", "item": {"id": "x", "type": "command_execution"}}),
        json!({"type": "item.completed", "item": {"id": "", "type": "agent_message", "text": "t"}}),
        json!({"type": "turn.completed"}),
        json!({"type": "turn.failed"}),
        json!({"type": "error"}),
        json!({}),
    ] {
        void_at_every(&lines(), std::slice::from_ref(&bare), from, to, &bare.to_string());
    }
}

#[test]
fn pd_26_6_a_turn_that_failed_examines_nothing_whatever_its_output() {
    let mut v = lines();
    let last = v.len() - 1;
    v[last] = json!({"type": "turn.failed", "error": {"message": "the turn limit was reached"}});
    let r = read(&join(&v));
    assert_eq!(r, Report::default());
}

#[test]
fn pd_26_7_a_command_started_and_never_completed_is_void() {
    let (from, to) = inner();
    void_at_every(&lines(), &[ran("x", "cat sum.py")[0].clone()], from, to, "a start never completed");
}

#[test]
fn pd_26_8_a_kind_that_is_not_text_or_a_command_without_text_is_void() {
    let (from, to) = inner();
    for kind in [json!(true), json!(5), json!(["command_execution"])] {
        let mut done = ran("x", "cat sum.py")[1].clone();
        done["item"]["type"] = kind;
        void_at_every(&lines(), &[done], from, to, "a kind not text");
    }
    let mut done = ran("x", "cat sum.py")[1].clone();
    done["item"].as_object_mut().unwrap().remove("command");
    void_at_every(&lines(), &[done], from, to, "no command");
}

#[test]
fn pd_26_9_an_end_before_another_a_misplaced_item_or_a_command_that_is_not_text_is_void() {
    let mut v = lines();
    v.insert(v.len() - 1, json!({"type": "turn.failed", "error": {"message": "m"}}));
    void(&join(&v), "a failed turn before a completed one");
    let (from, to) = inner();
    let item = ran("x", "cat sum.py")[1]["item"].clone();
    for misplaced in [json!({"type": "turn.started", "item": item}), json!({"type": "thread.started", "item": item})] {
        void_at_every(&lines(), &[misplaced], from, to, "an item in another event");
    }
    for line in [json!(5), json!(["cat", "sum.py"]), json!({"cmd": "cat"}), Value::Null] {
        let started = command(json!("x"), "item.started", line.clone(), "", Value::Null, "in_progress");
        let done = command(json!("x"), "item.completed", line, "ok\n", json!(0), "completed");
        void_at_every(&lines(), &[started, done], from, to, "a command not text");
    }
}

#[test]
fn pd_26_10_and_26_12_anything_after_the_end_is_void() {
    let end = lines().len();
    for after in [
        json!({"type": "error", "message": "late"}),
        ran("x", "cat sum.py")[1].clone(),
        ran("x", "cat sum.py")[0].clone(),
        json!({"type": "turn.completed", "usage": {}}),
        json!({"type": "turn.started"}),
    ] {
        let mut v = lines();
        v.insert(end, after.clone());
        void(&join(&v), &after.to_string());
    }
    // A start after its own completion.
    let (_, to) = inner();
    void_at_every(&lines(), &[lines()[START].clone()], DONE + 1, to, "a late start");
}

#[test]
fn pd_26_11_an_identifier_that_is_not_text_is_void() {
    let (from, to) = inner();
    for id in [json!(1), json!(true), json!(["item_1"])] {
        let start = command(id.clone(), "item.started", json!("cat a"), "", Value::Null, "in_progress");
        let done = command(json!("1"), "item.completed", json!("cat a"), "ok\n", json!(0), "completed");
        void_at_every(&lines(), &[start, done], from, to, "a numeric start with a text completion");
    }
}

#[test]
fn pd_26_13_a_start_repeated_before_its_completion_is_void() {
    void_at_every(&lines(), &[lines()[START].clone()], START + 1, DONE, "a repeated start");
}

#[test]
fn pd_26_14_a_command_changed_between_its_start_and_its_completion_is_void() {
    for line in [json!(""), json!(5), json!("cat other.py")] {
        let mut v = lines();
        v[DONE]["item"]["command"] = line.clone();
        void(&join(&v), &line.to_string());
        let mut v = lines();
        let mut update = v[START].clone();
        update["type"] = json!("item.updated");
        update["item"]["command"] = line.clone();
        v.insert(DONE, update);
        void(&join(&v), &format!("updated to {line}"));
    }
}

#[test]
fn pd_26_15_and_26_20_an_exit_code_that_is_not_a_whole_number_is_void() {
    for exit in [json!(true), json!(false), json!(0.5), json!("0")] {
        let mut v = lines();
        v[DONE]["item"]["exit_code"] = exit.clone();
        void(&join(&v), &format!("completion {exit}"));
        let mut v = lines();
        v[START]["item"]["exit_code"] = exit.clone();
        void(&join(&v), &format!("start {exit}"));
    }
    // A start that already has an exit code, or a status other than running.
    let mut v = lines();
    v[START]["item"]["exit_code"] = json!(0);
    void(&join(&v), "a start with an exit code");
}

#[test]
fn pd_26_16_line_separators_inside_a_text_do_not_break_the_stream() {
    for sep in ['\u{85}', '\u{2028}', '\u{2029}'] {
        let mut v = lines();
        v[2]["item"]["text"] = json!(format!("I will{sep}inspect"));
        v[DONE]["item"]["aggregated_output"] = json!(format!("/work{sep}\ntest_sum.py\n"));
        let r = read(&join(&v));
        assert_eq!(r.examined, EXAMINED, "{sep:?}");
        assert!(r.result.is_some());
    }
}

#[test]
fn pd_26_17_an_unknown_event_or_item_is_void() {
    let (from, to) = inner();
    for unknown in [
        json!({"type": "item.completed", "item": {"id": "x", "type": "mcp_tool_call", "server": "s", "tool": "t",
            "arguments": {}, "result": null, "error": null, "status": "completed"}}),
        json!({"type": "item.completed", "item": {"id": "x", "type": "collab_tool_call", "tool": "spawn_agent",
            "sender_thread_id": "a", "receiver_thread_ids": [], "prompt": "p", "agents_states": {}, "status": "completed"}}),
        json!({"type": "item.completed", "item": {"id": "x", "type": "web_search", "query": "q", "action": null}}),
        json!({"type": "turn.started"}),
        json!({"type": "thread.started", "thread_id": "t"}),
        json!({"type": "item.removed", "item": {"id": "x", "type": "agent_message", "text": "t"}}),
        json!({"type": "rate_limit_event"}),
    ] {
        void_at_every(&lines(), std::slice::from_ref(&unknown), from, to, &unknown.to_string());
    }
    // Evidence in a field of its own, or a field of another kind, beside the known ones (P9C-2).
    for (at, key) in [(DONE, "stdout"), (DONE, "text"), (RESULT, "command"), (2, "status")] {
        let mut v = lines();
        v[at]["item"][key] = json!("ok");
        void(&join(&v), key);
    }
    let mut v = lines();
    v[DONE]["extra"] = json!(1);
    void(&join(&v), "an event's extra field");
}

#[test]
fn pd_26_18_messages_reasoning_plans_and_edits_are_never_examinations() {
    let others = [
        json!({"type": "item.completed", "item": {"id": "m", "type": "agent_message", "text": "I ran cat sum.py"}}),
        json!({"type": "item.completed", "item": {"id": "r", "type": "reasoning", "text": "cat sum.py"}}),
        json!({"type": "item.started", "item": {"id": "t", "type": "todo_list", "items": [{"text": "read", "completed": false}]}}),
        json!({"type": "item.updated", "item": {"id": "t", "type": "todo_list", "items": [{"text": "read", "completed": true}]}}),
        json!({"type": "item.completed", "item": {"id": "t", "type": "todo_list", "items": [{"text": "read", "completed": true}]}}),
        json!({"type": "item.completed", "item": {"id": "f", "type": "file_change",
            "changes": [{"path": "/work/sum.py", "kind": "update"}], "status": "completed"}}),
    ];
    let r = read(&with(&others));
    assert_eq!(r.examined, 0);
    assert_eq!(r.result, Some(json!({"verdict": "accept", "findings": []})));
}

#[test]
fn pd_26_19_a_blank_line_is_void_and_a_final_newline_is_not_one() {
    assert_eq!(read(RECORDED.trim_end()).examined, EXAMINED);
    let all = lines();
    for p in 0..=all.len() {
        let mut text: Vec<String> = all.iter().map(Value::to_string).collect();
        text.insert(p, String::new());
        void(&(text.join("\n") + "\n"), &format!("blank at {p}"));
    }
    void(&RECORDED.replace('\n', "\r\n"), "carriage returns");
}

#[test]
fn a_stream_that_is_not_utf8_or_not_json_is_void() {
    void(&format!("{RECORDED}not json\n"), "not json");
    void("", "empty");
    assert_eq!(read_bytes(&[0xff, 0xfe]), Report::default());
}

fn read_bytes(b: &[u8]) -> Report {
    (ADAPTER.read)(b)
}

#[test]
fn p9c_1_a_command_that_failed_or_was_declined_examines_nothing() {
    for (exit, status) in
        [(json!(1), "failed"), (Value::Null, "declined"), (json!(1), "completed"), (json!(-1), "failed")]
    {
        let line = "/bin/sh -lc 'cat sum.py'";
        let start = command(json!("x"), "item.started", json!(line), "", Value::Null, "in_progress");
        let done = command(json!("x"), "item.completed", json!(line), "No such file\n", exit.clone(), status);
        let r = read(&with(&[start, done]));
        assert_eq!(r.examined, 0, "{exit} {status}");
        assert!(r.result.is_some());
    }
}

#[test]
fn p9c_3_and_p9c_4_a_comment_or_a_line_continuation_is_not_a_command() {
    assert_eq!(count("/bin/sh -lc '# cat sum.py'"), 0);
    assert_eq!(count("/bin/sh -lc 'cat a \\\n  b'"), 1);
    assert_eq!(read(&with(&ran("x", "/bin/sh -lc '# just a comment'"))).examined, 0);
}

#[test]
fn p9c_7_a_command_that_printed_nothing_examined_nothing() {
    // The recorded probe: `test -f sum.py`, silent, and `sh probe.sh`, which printed.
    let probe = include_str!("streams/probe.jsonl");
    assert_eq!(sha256_hex(probe.as_bytes()), PROBE_DIGEST);
    let r = read(probe);
    assert_eq!(r.examined, 1);
    let findings = r.result.unwrap()["findings"].clone();
    assert_eq!(findings[0]["path"], Value::Null);
    assert!(findings[0]["summary"].as_str().unwrap().contains("credential: unreadable"));
    // White space alone is nothing printed.
    let mut quiet = ran("x", "/bin/sh -lc 'echo'");
    quiet[1]["item"]["aggregated_output"] = json!(" \n\t\n");
    assert_eq!(read(&with(&quiet)).examined, 0);
}

const PROBE_DIGEST: &str = "3b9ea1513bb2c79b24455846fdf14d8a832e7480b22ed765d24115404cd518a4";

#[test]
fn the_recorded_build_reads_whole() {
    let build = include_str!("streams/build.jsonl");
    let r = read(build);
    let summary = r.result.unwrap()["summary"].as_str().unwrap().to_owned();
    assert!(summary.contains("mean(xs)"), "{summary}");
    assert_eq!(r.examined, 3);
    assert_eq!(r.tokens, Some(Tokens { input: Some(38951 - 18816), output: Some(415), cached: Some(18816) }));
}

#[test]
fn notices_of_errors_the_tool_recovered_from_read_as_nothing() {
    let (from, to) = inner();
    let notices = [
        json!({"type": "error", "message": "Reconnecting... 2/5 (stream disconnected before completion)"}),
        json!({"type": "item.completed", "item": {"id": "e", "type": "error", "message": "Falling back to HTTPS"}}),
    ];
    for p in from..=to {
        for notice in &notices {
            let mut v = lines();
            v.insert(p, notice.clone());
            let r = read(&join(&v));
            assert_eq!((r.examined, r.result.is_some()), (EXAMINED, true), "{notice} at {p}");
        }
    }
}

#[test]
fn a_result_that_is_not_a_json_object_is_none_and_examines_nothing() {
    for text in ["accept", "[1]", "\"accept\"", "{\"verdict\": "] {
        let mut v = lines();
        v[RESULT]["item"]["text"] = json!(text);
        let r = read(&join(&v));
        assert_eq!((r.result, r.examined), (None, 0), "{text}");
    }
    // The result is the last message: a message of prose after it leaves none.
    let mut v = lines();
    v.insert(
        RESULT + 1,
        json!({"type": "item.completed", "item": {"id": "z", "type": "agent_message", "text": "done"}}),
    );
    assert_eq!(read(&join(&v)).result, None);
}

#[test]
fn tokens_are_unknown_unless_whole_numbers_and_input_never_below_the_cache() {
    let end = lines().len() - 1;
    for (usage, expected) in [
        (
            json!({"input_tokens": 5, "cached_input_tokens": 7, "output_tokens": 1}),
            Tokens { input: None, output: Some(1), cached: Some(7) },
        ),
        (json!({"input_tokens": "5", "output_tokens": 1.5}), Tokens::default()),
        (json!({}), Tokens::default()),
    ] {
        let mut v = lines();
        v[end]["usage"] = usage;
        assert_eq!(read(&join(&v)).tokens, Some(expected));
    }
}

#[test]
fn a_lines_words_read_as_a_shell_reads_them() {
    let w = |s: &str| words(s).map(|v| v.join("|"));
    assert_eq!(w(r#"/bin/sh -lc "a \"b\" \$c \\d \e""#).unwrap(), r#"/bin/sh|-lc|a "b" $c \d \e"#);
    assert_eq!(w("a 'b c' d\\ e ''").unwrap(), "a|b c|d e|");
    assert_eq!(w("  a  ").unwrap(), "a");
    assert_eq!(w("a\"b\"'c'").unwrap(), "abc");
    assert_eq!(w("\"a\\\nb\"").unwrap(), "ab");
    assert_eq!(words("'a"), None);
    assert_eq!(words("\"a"), None);
    assert_eq!(words("a\\"), None);
    assert_eq!(words(""), Some(vec![]));
}

/// `line` with `member` inserted after its first `{`, as raw text: a member named twice when the line already has it,
/// which `serde_json::Value` cannot hold (P9D-1).
fn twice(line: &str, member: &str) -> String {
    line.replacen('{', &format!("{{{member},"), 1)
}

#[test]
fn p9d_1_a_member_named_twice_is_void_at_every_event_and_in_the_result() {
    let raw: Vec<&str> = RECORDED.lines().collect();
    for (n, line) in raw.iter().enumerate() {
        for member in [r#""type":false"#, r#""type":"turn.started""#] {
            let mut v: Vec<String> = raw.iter().map(|l| (*l).to_owned()).collect();
            v[n] = twice(line, member);
            void(&(v.join("\n") + "\n"), &format!("{member} at {n}"));
        }
    }
    // Inside an item: a boolean exit code hidden before the whole number.
    let hidden = RECORDED.replacen(r#""exit_code":0"#, r#""exit_code":true,"exit_code":0"#, 1);
    void(&hidden, "an exit code named twice");
    // Inside the result's text: a verdict named twice.
    let result = RECORDED.replacen(r#"{\"findings\":[]"#, r#"{\"verdict\":\"reject\",\"findings\":[]"#, 1);
    assert_ne!(result, RECORDED);
    let r = read(&result);
    assert_eq!((r.result, r.examined), (None, 0));
}

/// An item keeps its kind from its start to its completion, through its updates, even between two kinds whose fields
/// are the same (P9D-2).
#[test]
fn p9d_2_an_item_that_changes_its_kind_is_void() {
    let thinking =
        |phase: &str, id: &str, kind: &str| json!({"type": phase, "item": {"id": id, "type": kind, "text": "t"}});
    // The result's message started as reasoning.
    let mut v = lines();
    v.insert(RESULT, thinking("item.started", "item_3", "reasoning"));
    void(&join(&v), "a start as reasoning, a completion as a message");
    let (from, to) = inner();
    for (first, then) in [("reasoning", "agent_message"), ("agent_message", "reasoning")] {
        void_at_every(
            &lines(),
            &[
                thinking("item.started", "k", first),
                thinking("item.updated", "k", then),
                thinking("item.completed", "k", first),
            ],
            from,
            to,
            "an update of another kind",
        );
        void_at_every(
            &lines(),
            &[thinking("item.started", "k", first), thinking("item.completed", "k", then)],
            from,
            to,
            "a completion of another kind",
        );
    }
}

/// What the mutation of part 9d showed untested: the grammar's first two events, each kind's exact fields and types,
/// an item's status at its start and at a completion with no start.
#[test]
fn the_first_events_and_every_kind_hold_exactly_their_fields_and_types() {
    let all = lines();
    // The thread's start, then the turn's, and only there.
    for v in [
        all[1..].to_vec(),
        [&all[..1], &all[2..]].concat(),
        [vec![all[1].clone(), all[0].clone()], all[2..].to_vec()].concat(),
    ] {
        void(&join(&v), "the first two events");
    }
    for first in [
        json!({"type": "thread.started", "thread_id": "t", "x": 1}),
        json!({"type": "thread.started", "thread_id": ""}),
    ] {
        let mut v = all.clone();
        v[0] = first.clone();
        void(&join(&v), &first.to_string());
    }
    let (from, to) = inner();
    let bad = [
        json!({"type": "error", "message": 5}),
        json!({"type": "error", "message": "m", "x": 1}),
        json!({"type": "item.completed", "item": {"id": "f", "type": "file_change", "changes": [], "status": "done"}}),
        json!({"type": "item.completed", "item": {"id": "f", "type": "file_change", "changes": [{"path": "a", "kind": "update", "x": 1}], "status": "completed"}}),
        json!({"type": "item.completed", "item": {"id": "f", "type": "file_change", "changes": [{"path": 5, "kind": "update"}], "status": "completed"}}),
        json!({"type": "item.completed", "item": {"id": "f", "type": "file_change", "changes": [{"path": "a", "kind": "rename"}], "status": "completed"}}),
        json!({"type": "item.completed", "item": {"id": "t", "type": "todo_list", "items": [{"text": "a", "completed": true, "x": 1}]}}),
        json!({"type": "item.completed", "item": {"id": "t", "type": "todo_list", "items": [{"text": 5, "completed": true}]}}),
        json!({"type": "item.completed", "item": {"id": "t", "type": "todo_list", "items": [{"text": "a", "completed": "yes"}]}}),
        // A completion with no start, still running.
        command(json!("x"), "item.completed", json!("cat a"), "", Value::Null, "in_progress"),
    ];
    for b in bad {
        void_at_every(&all, std::slice::from_ref(&b), from, to, &b.to_string());
    }
    // A start that is already finished, then its completion.
    let line = "/bin/sh -lc 'cat a'";
    let started = command(json!("x"), "item.started", json!(line), "ok\n", json!(0), "completed");
    let done = command(json!("x"), "item.completed", json!(line), "ok\n", json!(0), "completed");
    void_at_every(&all, &[started, done], from, to, "a start already completed");
}

#[test]
fn only_a_shells_c_or_lc_with_one_script_counts_its_script() {
    // Three words that are not a shell's script count as the line: quoted separators do not split it.
    for line in ["cat -n 'a;b'", "python3 -c 'a; b'", "/bin/sh -x 'a; b'", "/usr/bin/env -c 'a | b'"] {
        assert_eq!(count(line), 1, "{line}");
    }
    assert_eq!(count("/usr/bin/zsh -c 'a; b'"), 2);
    assert_eq!(count("dash -lc 'a && b && c'"), 3);
}
