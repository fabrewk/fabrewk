//! The whole flow F-1 (core-specification 11.5; part 9i), shared by the flows' tests and the crash suite over it: its
//! fixture, a fake builder and a fake reviewer of Claude Code, and its write commands in order.

use super::*;
use fabrewk_core::alambique::{Rules, apply, apply_rules, rules_digest};
use fabrewk_core::record::sha256_hex;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

pub const SAY: &str = "Que siga.";

/// The alambique the flows approve an edit of.
pub const DOC: &str = "# Alambique\n\nOpening.\n\n## 1. One\n\n**1.1. A:** first.\n\n**1.2. B:** second.\n";

/// The fake builder: one line more in `src/log.txt` of the copy it is given, at every dispatch.
pub const BUILDS: &str = "mkdir -p src && printf 'x\\n' >> src/log.txt\n";

/// The fake reviewer: it rejects a candidate whose `src/log.txt` holds one line, with one confirmed blocking finding,
/// and accepts any other, so that a task's first round is rejected and its second accepted.
pub fn reviewer() -> String {
    let finding = json!({"severity": "blocking", "status": "confirmed", "path": "src/log.txt", "summary": "One line is not enough."});
    let reject = review(&json!({"verdict": "reject", "findings": [finding]}).to_string());
    let accept = review(r#"{"verdict":"accept","findings":[]}"#);
    format!("if [ \"$(wc -l < /work/src/log.txt)\" -eq 1 ]; then\n{reject}else\n{accept}fi\n")
}

pub fn git_in(fx: &Fx, repo: &Path, args: &[&str]) -> String {
    fx.git(&[&["-C", repo.to_str().unwrap()][..], args].concat())
}

/// Makes the domain its own repository, as `own-repository` needs, with its files committed there.
pub fn own(fx: &Fx) {
    git_in(fx, &fx.domain, &["init", "-q", "-b", "main"]);
    for (k, v) in [("user.name", "Fixture"), ("user.email", "fixture@example.invalid"), ("commit.gpgsign", "false")] {
        git_in(fx, &fx.domain, &["config", k, v]);
    }
    git_in(fx, &fx.domain, &["add", "-A"]);
    git_in(fx, &fx.domain, &["commit", "-q", "-m", "record"]);
}

/// What precedes a write command's record commit.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Before {
    Nothing,
    /// `approve` of a proposal for the alambique: the alambique's own commit, holding that file alone.
    Alambique,
    /// `accept`: the integration, in the product's repository.
    Integration,
}

/// The digest `status` shows for `id`, to authorize it.
pub fn shown(fx: &Fx, id: &str) -> String {
    fx.run(&["status", id, "--json"]).doc()["shown"]["digest"].as_str().unwrap().to_owned()
}

/// A file written outside the repository, as the integrator writes `propose`'s and `sweep`'s.
pub fn outside(fx: &Fx, name: &str, doc: &Value) -> String {
    let path = fx.root.parent().unwrap().join(name);
    fs::write(&path, serde_json::to_vec(doc).unwrap()).unwrap();
    path.to_str().unwrap().to_owned()
}

/// The stage of F-1: two tasks, the second depending on the first.
pub fn plan(fx: &Fx) -> String {
    let head = fx.git(&["rev-parse", "HEAD"]);
    let task = |more: Value| {
        let mut t = json!({
            "lang": "en", "goal": "Make it.", "requirements": [{"text": "It builds.", "criterion": "it builds"}],
            "base": head, "dependencies": [], "allowed": ["src/**"], "profiles": {"build": "build", "review": "review"},
            "checks": [], "limits": {"rounds": 3, "dispatches": 6, "minutes": 120}, "delegationLanguage": "en",
            "settings": {},
        });
        for (k, v) in more.as_object().unwrap() {
            t[k] = v.clone();
        }
        t
    };
    let stage = json!({"lang": "en", "goal": "Build it.", "budget": {"dispatches": 12, "minutes": 240}, "checks": {}, "settings": {}});
    let tasks =
        [task(json!({"name": "first"})), task(json!({"goal": "Then.", "base": null, "dependencies": ["first"]}))];
    outside(fx, "plan.json", &json!({"stage": stage, "tasks": tasks}))
}

/// The answer to `alias`'s fermento as it stands, with `edits` by point number.
pub fn answer(fx: &Fx, alias: &str, edits: &[(u32, Value)]) -> String {
    answer_saying(fx, alias, edits, "Say it.")
}

/// [`answer`], each recommendation asking to clarify with `clarify`.
pub fn answer_saying(fx: &Fx, alias: &str, edits: &[(u32, Value)], clarify: &str) -> String {
    let (text, parts) = fabrewk_core::fermento::open(&fx.domain, alias).unwrap();
    let rows: Vec<Value> = fabrewk_core::fermento::points(&text[..parts.input_end])
        .iter()
        .map(|p| {
            let rec = json!({"point": format!("About {}", p.number), "clarify": clarify, "destination": "alambique 1"});
            let mut row = json!({"number": p.number, "digest": p.digest, "recommendation": rec});
            if let Some((_, e)) = edits.iter().find(|(n, _)| *n == p.number) {
                row["edit"] = e.clone();
            }
            row
        })
        .collect();
    let doc = json!({"agent": {"tool": "claude-code", "provider": "anthropic", "model": "claude-sonnet-5-5"}, "points": rows});
    outside(fx, &format!("answer-{alias}.json"), &doc)
}

pub fn unit(kind: &str, n: u64, m: Option<u64>, text: &str) -> Value {
    let mut u = json!({"kind": kind, "n": n, "text": text});
    if let Some(m) = m {
        u["m"] = json!(m);
    }
    u
}

/// The kinds of `id`'s task's events.
pub fn kinds(fx: &Fx, id: &str) -> Vec<String> {
    fx.events(&format!("task-{id}")).iter().map(|e| e["kind"].as_str().unwrap().to_owned()).collect()
}

/// A fake builder's and reviewer's fixture in `mode`, with an approved alambique, ready for F-1.
/// Installed by a real `apply` (part 10b), the alambique [`DOC`] kept as its baseline.
pub fn fixture(name: &str, mode: &str) -> Fx {
    let fx = Fx::applied(name, mode, Some(DOC)).tooled(&format!("{BUILDS}{SUCCESS}"));
    fx.reviewer(Some(&reviewer()));
    fx
}

/// F-1 on `fx` (core-specification 11.5): each write command run by `step`, with what precedes its record commit,
/// which gives its `--json` document.
pub fn f1(fx: &Fx, step: &mut dyn FnMut(&[&str], Before) -> Value) {
    // A stage of two tasks, the second depending on the first.
    let plan = plan(fx);
    let doc = step(&["propose", &plan], Before::Nothing);
    let stage = doc["stage"]["id"].as_str().unwrap().to_owned();
    let ids: Vec<String> =
        doc["tasks"].as_array().unwrap().iter().map(|t| t["id"].as_str().unwrap().to_owned()).collect();
    step(&["authorize", &stage, "--seen", &shown(fx, &stage), "--say", SAY], Before::Nothing);
    // The second waits for the first: it may not start a run until the first is integrated.
    refused(fx, &["run", &ids[1], "--say", SAY], "DEPENDENCY_OPEN", 1, "es");
    // The stage's run takes the first: rejected by the review in round 1, accepted in round 2, verified.
    step(&["run", &stage, "--say", SAY], Before::Nothing);
    let reviews: Vec<Value> =
        fx.events(&format!("task-{}", ids[0])).into_iter().filter(|e| e["kind"] == "review-recorded").collect();
    let verdicts: Vec<(&str, u64)> = reviews
        .iter()
        .map(|r| (r["payload"]["verdict"].as_str().unwrap(), r["payload"]["round"].as_u64().unwrap()))
        .collect();
    assert_eq!(verdicts, [("reject", 1), ("accept", 2)]);
    assert_eq!(kinds(fx, &ids[0]).last().unwrap(), "task-verified");
    assert_eq!(fx.run(&["status", &ids[1], "--json"]).doc()["state"], "authorized");
    step(&["accept", &ids[0], "--seen", &accepting(fx, &ids[0]), "--say", SAY], Before::Integration);
    assert_eq!(kinds(fx, &ids[0]).last().unwrap(), "task-integrated");
    assert_eq!(fs::read_to_string(fx.root.join("src/log.txt")).unwrap(), "x\nx\n");
    // Then the second runs, on what the first's integration left, and is integrated.
    step(&["run", &stage, "--say", SAY], Before::Nothing);
    assert_eq!(kinds(fx, &ids[1]).last().unwrap(), "task-verified");
    step(&["accept", &ids[1], "--seen", &accepting(fx, &ids[1]), "--say", SAY], Before::Integration);
    assert_eq!(fs::read_to_string(fx.root.join("src/log.txt")).unwrap(), "x\nx\nx\n");
    // The sweep: point 1 proposes an edit of the alambique, point 2 one of the rules.
    let text = fs::read_to_string(fx.domain.join("ALAMBIQUE.md")).unwrap();
    let units = vec![unit("point", 1, Some(2), "**1.2. B:** rewritten.")];
    let result = sha256_hex(apply(&text, &units).unwrap().as_bytes());
    let alambique = json!({"target": "alambique", "base": sha256_hex(text.as_bytes()), "keys": [], "result": result, "units": units});
    let none = Rules::new();
    let units = vec![unit("rule", 1, None, "Be brief.")];
    let result = rules_digest(&apply_rules(&none, &units).unwrap());
    let rules = json!({"target": "rules", "base": rules_digest(&none), "keys": [], "result": result, "units": units});
    let file = answer(fx, "xel", &[(1, alambique), (2, rules)]);
    let doc = step(&["sweep", "xel", "--answer", &file], Before::Nothing);
    let proposals: Vec<&str> = doc["proposals"].as_array().unwrap().iter().map(|p| p.as_str().unwrap()).collect();
    assert_eq!(proposals.len(), 2, "{doc}");
    for (id, before) in proposals.iter().zip([Before::Alambique, Before::Nothing]) {
        step(&["approve", id, "--say", SAY], before);
    }
    assert_eq!(fs::read_to_string(fx.domain.join("ALAMBIQUE.md")).unwrap(), text.replace("second.", "rewritten."));
    assert!(fs::read_to_string(fx.domain.join("RULES.md")).unwrap().contains("Be brief."));
    step(&["close", &stage, "--say", SAY], Before::Nothing);
    assert_eq!(fx.events(&format!("stage-{stage}")).last().unwrap()["kind"], "stage-closed");
}
