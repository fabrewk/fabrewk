//! The tool adapter's contract in the core (REQ-RUN-7, REQ-RUN-8; part 9c): the packet holds only its listed parts,
//! each role's result is validated by the core (`REVIEW_INVALID`), and the usage is S-5's shape.

use fabrewk_core::adapter::{self, Previous, Role, Tokens};
use fabrewk_core::cli::say;
use fabrewk_core::record::Replay;
use fabrewk_core::{Code, Error};
use serde_json::{Value, json};

fn task() -> Value {
    json!({
        "nonce": "0".repeat(32), "stage": "s1", "lang": "es", "goal": "the goal",
        "requirements": [{"text": "it works", "criterion": "the test passes"}],
        "base": {"commit": "c"}, "dependencies": [], "allowed": ["src/**"],
        "profiles": {"build": "b", "review": "r"}, "checks": ["unit"],
        "limits": {"rounds": 3, "dispatches": 6, "minutes": 120}, "delegationLanguage": "en",
        "settings": {"gate.protectedPaths": ["secret/"]},
    })
}

fn keys(v: &Value) -> Vec<&str> {
    v.as_object().unwrap().keys().map(String::as_str).collect()
}

#[test]
fn req_run_8_the_packet_holds_only_its_listed_parts() {
    let rules = adapter::rules(&Replay::default(), Role::Review, "en");
    for role in Role::ALL {
        let p: Value = serde_json::from_str(&adapter::packet(&task(), role, &rules, "en", None)).unwrap();
        assert_eq!(keys(&p), ["allowed", "goal", "languages", "requirements", "role", "rules", "schema"]);
        assert_eq!(p["role"], role.name());
        assert_eq!((&p["goal"], &p["allowed"]), (&json!("the goal"), &json!(["src/**"])));
        assert_eq!(p["requirements"], task()["requirements"]);
        assert_eq!(p["languages"], json!({"delegation": "en", "code": "en"}));
        assert_eq!(p["schema"], adapter::schema(role));
        assert_eq!(p["rules"], json!(rules));
        // Nothing else of the task reaches it: not its base, its profiles, its checks or its settings.
        let text = p.to_string();
        assert!(!text.contains("secret/") && !text.contains("\"unit\"") && !text.contains("nonce"), "{text}");
    }
    // From the second round on: the previous round's confirmed findings and the failed checks' output.
    let findings = [json!({"severity": "blocking", "status": "confirmed", "summary": "it fails"})];
    let failed = [json!({"check": "unit", "output": "1 failed"})];
    let previous = Previous { findings: &findings, failed: &failed };
    let p: Value = serde_json::from_str(&adapter::packet(&task(), Role::Build, &rules, "en", Some(&previous))).unwrap();
    assert_eq!(keys(&p), ["allowed", "goal", "languages", "previous", "requirements", "role", "rules", "schema"]);
    assert_eq!(p["previous"], json!({"findings": findings, "failedChecks": failed}));
}

#[test]
fn req_run_8_the_rules_are_the_roles_instruction_the_base_rules_an_agent_needs_and_the_projects() {
    for (lang, says) in [("en", "en"), ("es", "es"), ("fr", "en")] {
        for role in Role::ALL {
            let rules = adapter::rules(&Replay::default(), role, lang);
            let mut expected = vec![say(says, &format!("packet.{}", role.name()), &[])];
            expected.extend([8, 11, 12].map(|n| say(says, &format!("rules.base.{n}"), &[])));
            assert_eq!(rules, expected, "{lang}");
            assert!(rules.iter().all(|r| !r.is_empty() && !r.contains('{')), "{rules:?}");
        }
    }
}

fn finding(severity: &str, status: &str) -> Value {
    json!({"severity": severity, "status": status, "summary": "a finding", "path": "src/a.rs"})
}

fn invalid(v: Value) {
    match adapter::validate(Role::Review, &v) {
        Err(Error::Refused { code: Code::ReviewInvalid, .. }) => {}
        other => panic!("{v} gave {other:?}"),
    }
}

#[test]
fn refusal_review_invalid() {
    // REQ-RUN-8's three cases.
    invalid(
        json!({"verdict": "reject", "findings": [finding("minor", "confirmed"), finding("blocking", "suspected")]}),
    );
    invalid(json!({"verdict": "maybe", "findings": []}));
    invalid(
        json!({"verdict": "accept", "findings": [{"status": "confirmed", "path": null, "summary": "no severity"}]}),
    );
    // And what the schema refuses besides.
    invalid(json!({"verdict": "accept", "findings": [finding("blocking", "confirmed")]}));
    invalid(json!({"verdict": "accept", "findings": [], "extra": 1}));
    invalid(json!({"verdict": "accept"}));
    invalid(json!({"verdict": true, "findings": []}));
    invalid(json!({"verdict": "accept", "findings": {}}));
    invalid(json!({"verdict": "accept", "findings": vec![finding("minor", "suspected"); 51]}));
    let one = |f: Value| json!({"verdict": "accept", "findings": [f]});
    invalid(one(json!({"severity": "minor", "status": "confirmed", "path": null, "summary": ""})));
    let long = "x".repeat(4001);
    invalid(one(json!({"severity": "minor", "status": "confirmed", "path": null, "summary": long})));
    invalid(one(json!({"severity": "major", "status": "confirmed", "path": null, "summary": "s"})));
    invalid(one(json!({"severity": "minor", "status": "sure", "path": null, "summary": "s"})));
    invalid(one(json!({"severity": "minor", "status": "confirmed", "summary": "s", "path": 5})));
    invalid(one(json!({"severity": "minor", "status": "confirmed", "summary": "s", "path": ""})));
    invalid(one(json!({"severity": "minor", "status": "confirmed", "path": null, "summary": "s", "line": 5})));
    // Every property is required: a finding with no path says so with `null` (part 9d).
    invalid(one(json!({"severity": "minor", "status": "confirmed", "summary": "s"})));
    invalid(json!("accept"));
    // What it admits.
    let ok = [
        json!({"verdict": "accept", "findings": []}),
        json!({"verdict": "accept", "findings": [finding("minor", "confirmed"), finding("blocking", "suspected")]}),
        json!({"verdict": "reject", "findings": [finding("blocking", "confirmed")]}),
        json!({"verdict": "reject", "findings": vec![finding("blocking", "confirmed"); 50]}),
        one(json!({"severity": "minor", "status": "confirmed", "path": null, "summary": "x".repeat(4000)})),
    ];
    for v in ok {
        adapter::validate(Role::Review, &v).unwrap_or_else(|e| panic!("{v}: {e:?}"));
    }
}

#[test]
fn a_builders_result_that_does_not_fit_its_schema_is_a_tools_failure() {
    adapter::validate(Role::Build, &json!({"summary": "what changed"})).unwrap();
    for v in [json!({}), json!({"summary": ""}), json!({"summary": 1}), json!({"summary": "s", "x": 1}), json!(null)] {
        let e = adapter::validate(Role::Build, &v).unwrap_err();
        assert_eq!(e.code(), Some(Code::ToolFailed), "{v}");
    }
}

#[test]
fn the_schemas_name_what_the_validation_admits() {
    let review = adapter::schema(Role::Review);
    assert_eq!(review["properties"]["verdict"]["enum"], json!(["accept", "reject"]));
    let item = &review["properties"]["findings"]["items"];
    assert_eq!(item["required"], json!(["severity", "status", "path", "summary"]));
    assert_eq!(item["properties"]["path"]["type"], json!(["string", "null"]));
    assert_eq!(item["properties"]["severity"]["enum"], json!(["blocking", "minor"]));
    assert_eq!(item["properties"]["status"]["enum"], json!(["confirmed", "suspected"]));
    assert_eq!(review["properties"]["findings"]["maxItems"], 50);
    assert_eq!(adapter::schema(Role::Build)["required"], json!(["summary"]));
    // Every object closed, and every property of it required, as a provider's strict mode asks (part 9d).
    for role in Role::ALL {
        let schema = adapter::schema(role);
        for object in [&schema, &schema["properties"]["findings"]["items"]].into_iter().filter(|o| o.is_object()) {
            assert_eq!(object["additionalProperties"], false);
            let mut keys: Vec<_> = object["properties"].as_object().unwrap().keys().cloned().collect();
            let mut required: Vec<_> =
                object["required"].as_array().unwrap().iter().map(|k| k.as_str().unwrap().to_owned()).collect();
            keys.sort();
            required.sort();
            assert_eq!(keys, required);
        }
    }
}

#[test]
fn req_run_7_s_5_the_usage_is_seconds_and_tokens_each_unknown_when_not_observed() {
    assert_eq!(adapter::usage(None, None), json!({"seconds": null, "tokens": null}));
    let t = Tokens { input: Some(10), output: None, cached: Some(0) };
    assert_eq!(
        adapter::usage(Some(7), Some(t)),
        json!({"seconds": 7, "tokens": {"input": 10, "output": null, "cached": 0}})
    );
}
