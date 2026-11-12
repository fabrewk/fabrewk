//! Claude Code's stream and its audit (REQ-GATE-4, PD-25, PD-26; part 9c), on the stream recorded from a real
//! review (core-specification 11.2 rule 3): Claude Code 2.1.316, `claude-sonnet-5-5` at effort `low`, confined as
//! this adapter launches it, on a copy of three files. Only the account's rate-limit figures were removed from it.
//! Every sub-case of PD-26 inserts its anomaly at every position of that stream.

use fabrewk_adapter_claude::ADAPTER;
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

/// The examinations of the recorded stream: one `Bash` use of three simple commands.
const EXAMINED: u64 = 3;

/// Where the recorded stream's events are: its `Bash` use and its answer, and its result, last.
const USE: usize = 1;
const ANSWER: usize = 3;

fn bash_id() -> String {
    lines()[USE]["message"]["content"][0]["id"].as_str().unwrap().to_owned()
}

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

fn assistant(content: Value) -> Value {
    json!({"type": "assistant", "message": {"model": "claude-sonnet-5-5", "role": "assistant", "content": content}})
}

fn user(content: Value) -> Value {
    json!({"type": "user", "message": {"role": "user", "content": content}})
}

fn bash(id: &str, command: Value) -> Value {
    assistant(json!([{"type": "tool_use", "id": id, "name": "Bash", "input": {"command": command}}]))
}

fn answer(id: Value, is_error: Option<Value>) -> Value {
    let mut block = json!({"type": "tool_result", "tool_use_id": id, "content": "ok"});
    if let Some(e) = is_error {
        block["is_error"] = e;
    }
    user(json!([block]))
}

/// Every position at which an event may be inserted into the recorded stream and still lie before its result.
fn inner() -> (usize, usize) {
    (1, lines().len() - 1)
}

#[test]
fn pd_26_the_recorded_stream_is_pinned_and_audits_as_examined() {
    assert_eq!(sha256_hex(RECORDED.as_bytes()), "f4f7f63adbb80d803a9fdeca2df23d26e8cfb400bf7804d22846d96d2ee708fa");
    let r = read(RECORDED);
    assert_eq!(r.result, Some(json!({"verdict": "accept", "findings": []})));
    assert_eq!(r.model.as_deref(), Some("claude-sonnet-5-5"));
    assert_eq!(r.tokens, Some(Tokens { input: Some(4 + 9078), output: Some(155), cached: Some(8838) }));
    assert_eq!(r.examined, EXAMINED);
    // Read again from its events, without the final newline, it is the same.
    assert_eq!(read(join(&lines()).trim_end_matches('\n')), r);
}

#[test]
fn pd_25_an_accept_after_no_command_examines_nothing() {
    let mut v = lines();
    v.drain(USE..=ANSWER);
    let r = read(&join(&v));
    assert_eq!(r.result, Some(json!({"verdict": "accept", "findings": []})));
    assert_eq!(r.examined, 0);
}

#[test]
fn pd_26_1_a_command_with_no_error_flag_is_void() {
    let (from, to) = inner();
    void_at_every(&lines(), &[bash("x1", json!("ls")), answer(json!("x1"), None)], from, to, "no flag");
    let mut v = lines();
    v[ANSWER]["message"]["content"][0].as_object_mut().unwrap().remove("is_error");
    void(&join(&v), "the recorded answer without its flag");
}

#[test]
fn pd_26_2_quoted_separators_do_not_split_and_unquoted_ones_do() {
    use fabrewk_adapter_claude::segments;
    assert_eq!(segments("echo 'a;b' \"c|d && e\" f\\;g"), ["echo 'a;b' \"c|d && e\" f\\;g"]);
    assert_eq!(segments("a; b && c || d | e\nf"), ["a", "b", "c", "d", "e", "f"]);
    assert_eq!(segments("printf 'x\ny'"), ["printf 'x\ny'"]);
    assert_eq!(segments("\n ;; \n"), Vec::<String>::new());
    let (from, to) = inner();
    for (command, more) in [("echo 'a;b' \"c|d\"", 1), ("echo a; echo b && echo c", 3), ("x|y\nz", 3)] {
        for p in from..=to {
            let mut v = lines();
            v.splice(p..p, [bash("x2", json!(command)), answer(json!("x2"), Some(json!(false)))]);
            assert_eq!(read(&join(&v)).examined, EXAMINED + more, "{command} at {p}");
        }
    }
    // A command with no simple command in it is not a command.
    void_at_every(&lines(), &[bash("x2", json!(" ;\n")), answer(json!("x2"), Some(json!(false)))], from, to, "empty");
}

#[test]
fn pd_26_3_a_second_completion_of_the_same_command_is_void() {
    let (_, to) = inner();
    void_at_every(&lines(), &[answer(json!(bash_id()), Some(json!(false)))], ANSWER + 1, to, "second answer");
}

#[test]
fn pd_26_4_every_dispatch_is_audited_and_a_stream_without_its_end_examines_nothing() {
    void("", "empty");
    let v = lines();
    for cut in 1..v.len() {
        void(&join(&v[..cut]), &format!("cut at {cut}"));
    }
}

#[test]
fn pd_26_5_a_bare_tool_use_or_answer_is_void() {
    let (from, to) = inner();
    for block in [
        json!({"type": "tool_use", "name": "Bash", "input": {"command": "ls"}}),
        json!({"type": "tool_use", "id": "x5", "input": {"command": "ls"}}),
        json!({"type": "tool_use", "id": "x5", "name": "Read"}),
        json!({"type": "tool_use", "id": "", "name": "Read", "input": {}}),
        json!({"type": "tool_use"}),
    ] {
        void_at_every(&lines(), &[assistant(json!([block]))], from, to, "bare use");
    }
    let bare = user(json!([{"type": "tool_result", "content": "ok", "is_error": false}]));
    void_at_every(&lines(), &[bare], from, to, "bare answer");
}

#[test]
fn pd_26_6_a_run_ended_by_its_turn_limit_examines_nothing_whatever_its_output() {
    for subtype in ["error_max_turns", "error_during_execution", "error_max_budget_usd"] {
        let mut v = lines();
        let last = v.len() - 1;
        v[last]["subtype"] = json!(subtype);
        let r = read(&join(&v));
        assert_eq!((r.result, r.examined), (None, 0), "{subtype}");
        // What the tool reported stays its report.
        assert!(r.tokens.is_some() && r.model.is_some());
    }
}

#[test]
fn pd_26_7_a_command_started_and_never_completed_is_void() {
    let (from, to) = inner();
    void_at_every(&lines(), &[bash("x7", json!("ls"))], from, to, "unanswered");
    let mut v = lines();
    v.remove(ANSWER);
    void(&join(&v), "the recorded use unanswered");
}

#[test]
fn pd_26_8_a_tool_name_that_is_not_text_or_a_command_without_text_is_void() {
    let (from, to) = inner();
    for name in [json!(true), json!(5), json!(["Bash"]), json!(" ")] {
        let block = json!({"type": "tool_use", "id": "x8", "name": name, "input": {"command": "ls"}});
        void_at_every(
            &lines(),
            &[assistant(json!([block])), answer(json!("x8"), Some(json!(false)))],
            from,
            to,
            "name",
        );
    }
    let no_text = assistant(json!([{"type": "tool_use", "id": "x8", "name": "Bash", "input": {}}]));
    void_at_every(&lines(), &[no_text, answer(json!("x8"), Some(json!(false)))], from, to, "no text");
}

#[test]
fn pd_26_9_a_failed_result_before_a_success_a_misplaced_event_or_a_command_that_is_not_text_is_void() {
    let (from, to) = inner();
    let failed = json!({"type": "result", "subtype": "error_during_execution", "is_error": true});
    void_at_every(&lines(), &[failed], from, to, "failed result first");
    let used = json!([{"type": "tool_use", "id": "x9", "name": "Read", "input": {"file_path": "/work/sum.py"}}]);
    void_at_every(&lines(), &[user(used.clone())], from, to, "a use in the user's turn");
    let answered = json!([{"type": "tool_result", "tool_use_id": bash_id(), "is_error": false}]);
    let mut v = lines();
    v.remove(ANSWER);
    void_at_every(&v, &[assistant(answered)], ANSWER, v.len() - 1, "an answer in the assistant's turn");
    for command in [json!(5), json!({"c": "ls"}), json!(null), json!(["ls"])] {
        void_at_every(&lines(), &[bash("x9", command), answer(json!("x9"), Some(json!(false)))], from, to, "not text");
    }
}

#[test]
fn pd_26_10_an_error_flag_on_the_result_or_anything_after_the_end_examines_nothing() {
    let mut v = lines();
    let last = v.len() - 1;
    v[last]["is_error"] = json!(true);
    assert_eq!(read(&join(&v)).examined, 0);
    let end = lines().len();
    void_at_every(&lines(), &[bash("x10", json!("ls"))], end, end, "a late start");
    void_at_every(&lines(), &[answer(json!(bash_id()), Some(json!(false)))], end, end, "text after the end");
}

#[test]
fn pd_26_11_an_identifier_that_is_not_text_is_void() {
    let mut v = lines();
    v[ANSWER]["message"]["content"][0]["tool_use_id"] = json!(1);
    void(&join(&v), "a numeric answer");
    let (from, to) = inner();
    let numeric = assistant(json!([{"type": "tool_use", "id": 1, "name": "Bash", "input": {"command": "ls"}}]));
    void_at_every(&lines(), &[numeric, answer(json!(1), Some(json!(false)))], from, to, "a numeric use");
    void_at_every(&lines(), &[bash("1", json!("ls")), answer(json!(1), Some(json!(false)))], from, to, "converted");
}

#[test]
fn pd_26_12_an_error_after_the_end_does_not_reopen_the_count() {
    let end = lines().len();
    let error = json!({"type": "result", "subtype": "error_during_execution", "is_error": true});
    void_at_every(&lines(), &[error], end, end, "an error after the end");
    void_at_every(&lines(), &[lines()[end - 1].clone()], end, end, "a second result");
}

#[test]
fn pd_26_13_a_start_repeated_before_its_completion_is_void() {
    let (_, to) = inner();
    void_at_every(&lines(), &[lines()[USE].clone()], USE + 1, to, "repeated");
}

#[test]
fn pd_26_14_a_command_changed_to_empty_or_to_a_value_that_is_not_text_is_void() {
    for command in [json!(""), json!("  "), json!(5), json!(null), json!(false)] {
        let mut v = lines();
        v[USE]["message"]["content"][0]["input"]["command"] = command.clone();
        void(&join(&v), &format!("{command}"));
    }
}

#[test]
fn pd_26_15_and_26_20_an_error_flag_that_is_not_a_boolean_is_void() {
    for flag in [json!(0), json!(1), json!("false"), json!(0.5), json!(null), json!([false])] {
        let mut v = lines();
        v[ANSWER]["message"]["content"][0]["is_error"] = flag.clone();
        void(&join(&v), &format!("{flag}"));
        let (from, to) = inner();
        let pair = [bash("x15", json!("ls")), answer(json!("x15"), Some(flag.clone()))];
        void_at_every(&lines(), &pair, from, to, &format!("{flag}"));
    }
}

#[test]
fn pd_26_16_line_separators_inside_a_text_do_not_break_the_stream() {
    let valid = read(RECORDED);
    let text = lines()[ANSWER]["message"]["content"][0]["content"].as_str().unwrap().to_owned();
    for c in ['\u{85}', '\u{2028}', '\u{2029}'] {
        for at in (0..=text.chars().count()).step_by(7) {
            let mut v = lines();
            let mut changed: String = text.chars().take(at).collect();
            changed.push(c);
            changed.extend(text.chars().skip(at));
            v[ANSWER]["message"]["content"][0]["content"] = json!(changed);
            let s = join(&v);
            assert!(s.contains(c));
            assert_eq!(read(&s), valid, "{c:?} at {at}");
        }
    }
    // Written raw into the recorded bytes, too.
    let raw = RECORDED.replacen("ok\"", "o\u{2028}k\u{85}\u{2029}\"", 1);
    assert_ne!(raw, RECORDED);
    assert_eq!(read(&raw), valid);
}

#[test]
fn pd_26_17_an_unknown_event_or_block_is_void() {
    let (from, to) = inner();
    for event in [
        json!({"type": "unknown"}),
        json!({"type": "system", "subtype": "hook_started"}),
        json!({"type": "system", "subtype": "init", "model": "claude-sonnet-5-5"}),
        json!({"type": "stream_event"}),
        json!({"kind": "assistant"}),
        assistant(json!([{"type": "server_tool_use", "id": "x17", "name": "web_search", "input": {}}])),
        user(json!([{"type": "text", "text": "ok"}])),
        json!(["not an object"]),
        json!("text"),
    ] {
        void_at_every(&lines(), &[event], from, to, "unknown");
    }
    let mut v = lines();
    v.remove(0);
    void(&join(&v), "no init first");
}

#[test]
fn pd_26_18_the_providers_own_answer_tool_is_never_an_examination() {
    let mut v = lines();
    v[USE]["message"]["content"][0]["name"] = json!("StructuredOutput");
    let r = read(&join(&v));
    assert_eq!((r.examined, r.result.is_some()), (0, true));
    let (from, to) = inner();
    for p in from..=to {
        let mut v = lines();
        let used = assistant(json!([{"type": "tool_use", "id": "x18", "name": "StructuredOutput", "input": {}}]));
        v.splice(p..p, [used, answer(json!("x18"), None)]);
        assert_eq!(read(&join(&v)).examined, EXAMINED, "at {p}");
    }
}

#[test]
fn pd_26_19_a_blank_line_is_void_and_a_final_newline_is_not_one() {
    let v = lines();
    for p in 0..=v.len() {
        let mut s: Vec<String> = v.iter().map(Value::to_string).collect();
        s.insert(p, String::new());
        void(&(s.join("\n") + "\n"), &format!("blank at {p}"));
    }
    void(&(RECORDED.to_owned() + "\n"), "a blank last line");
    assert_eq!(read(RECORDED.trim_end_matches('\n')).examined, EXAMINED);
    void(&RECORDED.replace('\n', "\r\n"), "carriage returns");
}

#[test]
fn a_stream_that_is_not_utf8_or_not_json_is_void() {
    let mut bytes = RECORDED.as_bytes().to_vec();
    bytes.insert(10, 0xff);
    assert_eq!((ADAPTER.read)(&bytes), Report::default());
    void(&RECORDED.replacen('}', "", 1), "truncated object");
}

#[test]
fn p9c_1_a_tool_use_answered_with_an_error_examines_nothing() {
    let mut v = lines();
    v[ANSWER]["message"]["content"][0]["is_error"] = json!(true);
    let r = read(&join(&v));
    assert_eq!((r.examined, r.result.is_some()), (0, true));
    let (from, to) = inner();
    for p in from..=to {
        let mut v = lines();
        v.splice(p..p, [bash("e1", json!("cat missing.py")), answer(json!("e1"), Some(json!(true)))]);
        assert_eq!(read(&join(&v)).examined, EXAMINED, "at {p}");
    }
}

#[test]
fn p9c_2_evidence_in_a_misplaced_field_or_of_a_wrong_type_is_void() {
    let (from, to) = inner();
    let used = json!({"content": [{"type": "tool_use", "id": "m", "name": "Bash", "input": {"command": "cat a"}}]});
    for kind in ["rate_limit_event", "system", "result"] {
        let mut e = json!({"type": kind, "message": used});
        if kind == "system" {
            e["subtype"] = json!("init");
        }
        void_at_every(&lines(), &[e], from, to, kind);
    }
    let mut v = lines();
    v[2]["message"] = used;
    void(&join(&v), "a message on the rate-limit notice");
    for content in
        [None, Some(json!(42)), Some(json!(null)), Some(json!([{"type": "text", "text": 5}])), Some(json!([1]))]
    {
        let mut v = lines();
        let block = v[ANSWER]["message"]["content"][0].as_object_mut().unwrap();
        match &content {
            None => {
                block.remove("content");
            }
            Some(c) => {
                block.insert("content".into(), c.clone());
            }
        }
        void(&join(&v), &format!("answer content {content:?}"));
    }
    let mut v = lines();
    v[ANSWER]["message"]["content"][0]["content"] = json!([{"type": "text", "text": "ok"}]);
    assert_eq!(read(&join(&v)).examined, EXAMINED, "an answer's content as text blocks");
    for block in
        [json!({"type": "text", "text": 42}), json!({"type": "text"}), json!({"type": "thinking", "thinking": 1})]
    {
        void_at_every(&lines(), &[assistant(json!([block]))], from, to, "text not text");
    }
    let fine = [json!({"type": "text", "text": "fine"}), json!({"type": "thinking", "thinking": "", "signature": "s"})];
    for p in from..=to {
        let mut v = lines();
        v.insert(p, assistant(json!(fine)));
        assert_eq!(read(&join(&v)).examined, EXAMINED, "at {p}");
    }
}

#[test]
fn p9c_3_a_comment_is_not_a_command() {
    use fabrewk_adapter_claude::segments;
    assert_eq!(segments("# no command executed"), Vec::<String>::new());
    assert_eq!(segments("ls # list\n  # nothing\ncat a"), ["ls", "cat a"]);
    assert_eq!(segments("echo a#b '# not' \"#\" \\#x"), ["echo a#b '# not' \"#\" \\#x"]);
    assert_eq!(segments("ls;# x && y\npwd"), ["ls", "pwd"]);
    let mut v = lines();
    v[USE]["message"]["content"][0]["input"]["command"] = json!("# no command executed");
    void(&join(&v), "a comment alone");
}

/// The stream recorded from a real build, as the review's was: it reads whole, gives its summary, and its tokens.
#[test]
fn the_recorded_build_reads_whole() {
    let text = include_str!("streams/build.jsonl");
    assert_eq!(sha256_hex(text.as_bytes()), "0f1c2473f2189821cdf430cfe5b94b8e554ad02447cfaaf4e6b0156bbda59df9");
    let r = read(text);
    let summary = r.result.as_ref().and_then(|v| v["summary"].as_str()).unwrap();
    assert!(summary.starts_with("Added mean(xs) to sum.py"), "{summary}");
    assert_eq!(r.model.as_deref(), Some("claude-sonnet-5-5"));
    assert!(r.tokens.is_some_and(|t| t.input.is_some() && t.output.is_some() && t.cached.is_some()));
    // One `Bash` use of a heredoc, each of whose lines the split counts, and one of a command.
    assert_eq!(r.examined, 14);
}

#[test]
fn the_estimates_of_thinking_a_real_run_writes_read_as_nothing() {
    let estimate = json!({"type": "system", "subtype": "thinking_tokens", "estimated_tokens": 50,
        "estimated_tokens_delta": 50, "session_id": "s", "uuid": "u"});
    let (from, to) = inner();
    for p in from..=to {
        let mut v = lines();
        v.insert(p, estimate.clone());
        assert_eq!(read(&join(&v)), read(RECORDED), "at {p}");
    }
    let mut v = lines();
    v.insert(0, estimate);
    void(&join(&v), "an estimate first");
}

#[test]
#[ignore = "reads a stream kept under target/ by a real dispatch"]
fn a_kept_stream_reads_whole() {
    let path = std::env::var("FABREWK_STREAM").unwrap();
    let r = (ADAPTER.read)(&std::fs::read(path).unwrap());
    eprintln!("{r:?}");
    assert!(r.result.is_some());
}

#[test]
fn p9c_4_a_line_continuation_is_not_a_command() {
    use fabrewk_adapter_claude::segments;
    assert_eq!(segments("\\\n"), Vec::<String>::new());
    assert_eq!(segments("\\\n# no command executed"), Vec::<String>::new());
    assert_eq!(segments("ls \\\n  -a"), ["ls   -a"]);
    assert_eq!(segments("echo \"a\\\nb\""), ["echo \"ab\""]);
    assert_eq!(segments("echo 'a\\\nb'"), ["echo 'a\\\nb'"]);
    for command in ["\\\n", "\\\n# no command executed", " \\\n\\\n "] {
        let mut v = lines();
        v[USE]["message"]["content"][0]["input"]["command"] = json!(command);
        void(&join(&v), &format!("{command:?}"));
    }
}

#[test]
fn p9c_6_a_final_event_whose_flag_or_subtype_is_not_its_type_is_void() {
    for (key, value) in
        [("is_error", json!(0)), ("is_error", json!(null)), ("subtype", json!(1)), ("subtype", json!(null))]
    {
        let mut v = lines();
        let last = v.len() - 1;
        v[last][key] = value.clone();
        void(&join(&v), &format!("{key} {value}"));
    }
    let mut v = lines();
    let last = v.len() - 1;
    v[last].as_object_mut().unwrap().remove("is_error");
    void(&join(&v), "no flag");
}
