//! Schema 1's envelope (REQ-REG-1, REQ-REG-2, REQ-REG-9): the closed list of section 3's kinds, the streams, the
//! actor of each kind, and the check of one event's fields against its place. Payloads beyond the creating nonce are
//! fixed by the part that first builds each kind (D-047).

use super::canon::{encode, hex, sha256_hex};
use super::{Code, time};
use serde_json::{Value, json};
use std::io::Read;

/// The kind of a stream, from its name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamKind {
    Project,
    Alambique,
    Rules,
    Fermento,
    Stage,
    Task,
    Ext,
}

/// Who may record a kind: a person (signed, with words), Fabrewk, an agent, any actor but a person, an extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    Person,
    Fabrewk,
    Agent,
    NotPerson,
    Extension,
}

use Class::{Agent, Extension, Fabrewk, NotPerson, Person};
use StreamKind::{Alambique, Ext, Fermento, Project, Rules, Stage, Task};

/// Every kind of schema 1, the stream it is recorded in, and its actor (core-specification section 3).
pub const KINDS: &[(&str, StreamKind, Class)] = &[
    ("assessed", Project, Person),
    ("publication-decided", Project, Person),
    ("policy-adopted", Project, Person),
    ("installed", Project, Fabrewk),
    ("alambique-baseline", Alambique, Person),
    ("proposal-incorporated", Alambique, Person),
    ("proposal-incorporated", Rules, Person),
    ("direct-edit-approved", Alambique, Person),
    ("sweep-recorded", Fermento, Agent),
    ("proposal-recorded", Fermento, Agent),
    ("proposal-withdrawn", Fermento, Fabrewk),
    ("proposal-discarded", Fermento, Person),
    ("stage-proposed", Stage, NotPerson),
    ("stage-authorized", Stage, Person),
    ("stage-closed", Stage, Person),
    ("task-proposed", Task, NotPerson),
    ("task-authorized", Task, Person),
    ("run-started", Task, Person),
    ("attempt-started", Task, Fabrewk),
    ("attempt-ended", Task, Fabrewk),
    ("candidate-recorded", Task, Fabrewk),
    ("check-recorded", Task, Fabrewk),
    ("review-recorded", Task, Fabrewk),
    ("task-verified", Task, Fabrewk),
    ("task-halted", Task, Fabrewk),
    ("task-accepted", Task, Person),
    ("merge-failed", Task, Fabrewk),
    ("task-integrated", Task, Fabrewk),
    ("task-rejected", Task, Person),
    ("extension-note", Ext, Extension),
];

/// The fields of every event of schema 1; a person's event has `say` besides.
pub const FIELDS: [&str; 11] =
    ["actor", "at", "fabrewk", "instance", "kind", "payload", "policy", "prev", "schema", "seq", "stream"];

/// The actor class of a kind of schema 1.
pub fn class_of(kind: &str) -> Option<Class> {
    KINDS.iter().find(|k| k.0 == kind).map(|k| k.2)
}

/// The kind of the stream named `name`, if the name is valid (REQ-REG-1).
pub fn stream_kind(name: &str) -> Option<StreamKind> {
    let kind = match name {
        "project" => Project,
        "alambique" => Alambique,
        "rules" => Rules,
        _ if name.strip_prefix("fermento-").is_some_and(is_name) => Fermento,
        _ if name.strip_prefix("stage-").is_some_and(|id| is_id(id, 's')) => Stage,
        _ if name.strip_prefix("task-").is_some_and(|id| is_id(id, 't')) => Task,
        _ if name.strip_prefix("ext-").is_some_and(is_name) => Ext,
        _ => return None,
    };
    Some(kind)
}

/// The stream a creating event opens: `stage-s-…` or `task-t-…`, from the digest of its canonical payload.
pub fn subject_stream(kind: &str, payload: &Value) -> Option<String> {
    let (prefix, letter) = match kind {
        "stage-proposed" => ("stage", 's'),
        "task-proposed" => ("task", 't'),
        _ => return None,
    };
    let mut bytes = encode(payload)?;
    bytes.pop();
    Some(format!("{prefix}-{letter}-{}", &sha256_hex(&bytes)[..8]))
}

/// Whether a kind creates its subject, so it opens its stream (`stage-proposed`, `task-proposed`).
pub(crate) fn is_creation(kind: &str) -> bool {
    matches!(kind, "stage-proposed" | "task-proposed")
}

/// A random nonce of 16 bytes, as 32 hexadecimal digits, for a creating payload (REQ-REG-1).
pub fn nonce() -> std::io::Result<String> {
    let mut bytes = [0u8; 16];
    std::fs::File::open("/dev/urandom")?.read_exact(&mut bytes)?;
    Ok(hex(&bytes))
}

/// A person's words and their language tag.
#[derive(Debug, Clone)]
pub struct Say {
    pub text: String,
    pub lang: String,
}

/// What the caller decides of a new event; the record fills `schema`, `stream`, `seq`, `prev`, `at` and `fabrewk`.
#[derive(Debug, Clone)]
pub struct Draft {
    pub kind: String,
    pub actor: Value,
    pub policy: String,
    pub instance: String,
    pub say: Option<Say>,
    pub payload: Value,
}

impl Draft {
    /// The event this draft makes at `seq` of `stream` after `prev`; [`super::Session::append`] writes it.
    pub fn build(&self, stream: &str, seq: u64, prev: Option<&str>) -> Value {
        let mut v = json!({
            "schema": 1, "stream": stream, "seq": seq, "prev": prev, "kind": self.kind, "at": time::now_utc(),
            "instance": self.instance, "fabrewk": crate::VERSION, "actor": self.actor, "policy": self.policy,
            "payload": self.payload,
        });
        if let Some(say) = &self.say {
            v["say"] = json!({"text": say.text, "lang": say.lang});
        }
        v
    }
}

/// Checks an event's fields against schema 1 and its place (`stream`, `seq` from the file's name); gives its class.
pub(crate) fn check(v: &Value, stream: &str, seq: u64) -> Result<Class, (Code, &'static str)> {
    use Code::{RecordBroken as Broken, RecordTransitionInvalid as Invalid, RecordVersionUnknown as Unknown};
    let o = v.as_object().ok_or((Unknown, "not an object"))?;
    if o.get("schema") != Some(&json!(1)) {
        return Err((Unknown, "schema"));
    }
    let kind = o.get("kind").and_then(Value::as_str).ok_or((Unknown, "kind"))?;
    let class = class_of(kind).ok_or((Unknown, "kind"))?;
    let say = o.contains_key("say");
    if o.len() != FIELDS.len() + usize::from(say) || !FIELDS.iter().all(|f| o.contains_key(*f)) {
        return Err((Unknown, "fields"));
    }
    let (Some(at_stream), Some(at_seq)) = (o["stream"].as_str(), o["seq"].as_u64()) else {
        return Err((Unknown, "stream or seq"));
    };
    let prev_ok = o["prev"].is_null() || o["prev"].as_str().is_some_and(|p| is_hex(p, 64));
    let text = |f: &str| o[f].as_str().is_some_and(|s| !s.is_empty());
    let types_ok = prev_ok
        && o["at"].as_str().is_some_and(time::is_utc_timestamp)
        && text("instance")
        && text("fabrewk")
        && o["policy"].as_str().is_some_and(|p| is_hex(p, 64))
        && o["payload"].is_object();
    let actor = actor_class(&o["actor"]);
    if !types_ok || actor.is_none() {
        return Err((Unknown, "field types"));
    }
    if at_stream != stream || at_seq != seq {
        return Err((Broken, "stream or seq differs from the file's place"));
    }
    if o["prev"].is_null() != (seq == 1) {
        return Err((Broken, "prev"));
    }
    let place = stream_kind(stream).ok_or((Broken, "stream name"))?;
    if !KINDS.iter().any(|k| k.0 == kind && k.1 == place) {
        return Err((Invalid, "kind not recorded in this stream"));
    }
    if actor != Some(class) && !(class == NotPerson && matches!(actor, Some(Agent | Fabrewk))) {
        return Err((Invalid, "actor"));
    }
    if say != (class == Person) {
        return Err((Invalid, "words on a move that is not a person's, or a person's move without words"));
    }
    if say {
        let w = o["say"].as_object().filter(|w| w.len() == 2);
        let (Some(words), Some(lang)) =
            (w.and_then(|w| w.get("text")?.as_str()), w.and_then(|w| w.get("lang")?.as_str()))
        else {
            return Err((Unknown, "say"));
        };
        if lang.is_empty() {
            return Err((Unknown, "say"));
        }
        if words.trim().is_empty() {
            return Err((Invalid, "empty words"));
        }
    }
    let nonce = o["payload"].get("nonce").and_then(Value::as_str);
    if is_creation(kind) && !nonce.is_some_and(|n| is_hex(n, 32)) {
        return Err((Unknown, "nonce"));
    }
    if kind == "policy-adopted" {
        policy_payload(&o["payload"])?;
    }
    Ok(class)
}

/// `policy-adopted`'s payload (part 7c1): exactly `content` (the project file's text), `digest` (its SHA-256) and
/// `widened` (the widenings named, each a key path).
fn policy_payload(p: &Value) -> Result<(), (Code, &'static str)> {
    let o = p.as_object().filter(|o| o.len() == 3);
    let field = |f: &str| o.and_then(|o| o.get(f));
    let (Some(content), Some(digest), Some(widened)) = (
        field("content").and_then(Value::as_str),
        field("digest").and_then(Value::as_str),
        field("widened").and_then(Value::as_array),
    ) else {
        return Err((Code::RecordVersionUnknown, "payload"));
    };
    if !widened.iter().all(|w| w.as_str().is_some_and(|w| !w.is_empty())) {
        return Err((Code::RecordVersionUnknown, "payload"));
    }
    if sha256_hex(content.as_bytes()) != digest {
        return Err((Code::RecordBroken, "the policy's digest is not its content's"));
    }
    Ok(())
}

fn actor_class(a: &Value) -> Option<Class> {
    let o = a.as_object()?;
    let has = |keys: &[&str]| o.len() == keys.len() && keys.iter().all(|k| o.contains_key(*k));
    let text = |f: &str| o.get(f).and_then(Value::as_str).is_some_and(|s| !s.is_empty());
    let class = match o.get("kind")?.as_str()? {
        "person" => Person,
        "agent" => Agent,
        "extension" => Extension,
        "fabrewk" => Fabrewk,
        _ => return None,
    };
    let ok = match class {
        Person => {
            has(&["kind", "alias", "role"])
                && o["alias"].as_str().is_some_and(is_name)
                && matches!(o["role"].as_str(), Some("maestro" | "supervisor" | "contributor"))
        }
        Agent => {
            has(&["kind", "tool", "provider", "modelRequested", "modelObserved"])
                && text("tool")
                && text("provider")
                && text("modelRequested")
                && (o["modelObserved"].is_null() || o["modelObserved"].is_string())
        }
        Extension => {
            has(&["kind", "name", "digest"])
                && o["name"].as_str().is_some_and(is_name)
                && o["digest"].as_str().is_some_and(|d| is_hex(d, 64))
        }
        Fabrewk | NotPerson => has(&["kind"]),
    };
    ok.then_some(class)
}

/// The alias pattern of REQ-DOM-3, also used for extension names: `^[a-z][a-z0-9]*(-[a-z0-9]+)*$`, at most 32.
pub(crate) fn is_name(s: &str) -> bool {
    let b = s.as_bytes();
    !b.is_empty()
        && b.len() <= 32
        && b[0].is_ascii_lowercase()
        && b[b.len() - 1] != b'-'
        && !s.contains("--")
        && b.iter().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'-')
}

fn is_id(s: &str, letter: char) -> bool {
    let mut chars = s.chars();
    chars.next() == Some(letter) && chars.next() == Some('-') && is_hex(chars.as_str(), 8)
}

pub(crate) fn is_hex(s: &str, len: usize) -> bool {
    s.len() == len && s.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn req_reg_1_stream_names() {
        for (name, kind) in [
            ("project", Project),
            ("alambique", Alambique),
            ("rules", Rules),
            ("fermento-xel", Fermento),
            ("fermento-a1-b2", Fermento),
            ("stage-s-0123abcd", Stage),
            ("task-t-0123abcd", Task),
            ("ext-lint", Ext),
        ] {
            assert_eq!(stream_kind(name), Some(kind), "{name}");
        }
        for name in [
            "Project",
            "fermento-",
            "fermento-Xel",
            "fermento-x--y",
            "stage-t-0123abcd",
            "stage-s-0123ABCD",
            "stage-s-0123abc",
            "task-s-0123abcd",
            "task-t-0123abcdd",
            "task-t_0123abcd",
            "ext-",
            "ext-a-",
            "other",
        ] {
            assert_eq!(stream_kind(name), None, "{name}");
        }
        let payload = json!({"nonce": "0".repeat(32)});
        let stage = subject_stream("stage-proposed", &payload).unwrap();
        let task = subject_stream("task-proposed", &payload).unwrap();
        assert_eq!((&stage[..8], &task[..7]), ("stage-s-", "task-t-"));
        assert_eq!(stage[8..], task[7..]);
        assert_eq!(subject_stream("task-authorized", &payload), None);
    }

    #[test]
    fn req_dom_3_alias_pattern() {
        for good in ["x", "xel", "a1", "a-b", "a1-b2-c3", &"a".repeat(32)] {
            assert!(is_name(good), "{good}");
        }
        for bad in ["", "1a", "-a", "a-", "a--b", "A", "aB", "a_b", "a.b", "é", &"a".repeat(33)] {
            assert!(!is_name(bad), "{bad}");
        }
        assert!(is_id("t-0123abcd", 't'));
        for bad in ["s-0123abcd", "t0123abcde", "t-0123abc", "t-0123abcg", "t_0123abcd", "-t0123abcd"] {
            assert!(!is_id(bad, 't'), "{bad}");
        }
    }

    #[test]
    fn policy_adopted_payload_is_its_content_digest_and_widenings() {
        let content = "[project]\nname = \"x\"\n";
        let digest = sha256_hex(content.as_bytes());
        let event = |payload: Value| {
            let actor = json!({"kind": "person", "alias": "xel", "role": "maestro"});
            let say = Some(Say { text: "Adopto.".into(), lang: "es".into() });
            let d = Draft {
                kind: "policy-adopted".into(),
                actor,
                policy: "a".repeat(64),
                instance: "i".into(),
                say,
                payload,
            };
            check(&d.build("project", 1, None), "project", 1).map(|_| ()).map_err(|e| e.0)
        };
        assert_eq!(event(json!({"content": content, "digest": digest, "widened": ["limits.rounds"]})), Ok(()));
        for bad in [
            json!({}),
            json!({"content": content, "digest": digest}),
            json!({"content": content, "digest": digest, "widened": [""]}),
            json!({"content": content, "digest": digest, "widened": [1]}),
            json!({"content": 1, "digest": digest, "widened": []}),
            json!({"content": content, "digest": digest, "widened": [], "x": 1}),
        ] {
            assert_eq!(event(bad.clone()), Err(Code::RecordVersionUnknown), "{bad}");
        }
        let other = json!({"content": "other", "digest": digest, "widened": []});
        assert_eq!(event(other), Err(Code::RecordBroken));
    }

    #[test]
    fn req_reg_2_actor_shapes() {
        let ok = [
            (json!({"kind": "person", "alias": "xel", "role": "maestro"}), Person),
            (json!({"kind": "person", "alias": "xel", "role": "supervisor"}), Person),
            (json!({"kind": "person", "alias": "xel", "role": "contributor"}), Person),
            (
                json!({"kind": "agent", "tool": "t", "provider": "p", "modelRequested": "m", "modelObserved": null}),
                Agent,
            ),
            (
                json!({"kind": "agent", "tool": "t", "provider": "p", "modelRequested": "m", "modelObserved": "o"}),
                Agent,
            ),
            (json!({"kind": "extension", "name": "lint", "digest": "a".repeat(64)}), Extension),
            (json!({"kind": "fabrewk"}), Fabrewk),
        ];
        for (actor, class) in ok {
            assert_eq!(actor_class(&actor), Some(class), "{actor}");
        }
        let bad = [
            json!({"kind": "person", "alias": "Xel", "role": "maestro"}),
            json!({"kind": "person", "alias": "xel", "role": "owner"}),
            json!({"kind": "person", "alias": "xel"}),
            json!({"kind": "person", "alias": "xel", "role": "maestro", "x": 1}),
            json!({"kind": "agent", "tool": "", "provider": "p", "modelRequested": "m", "modelObserved": null}),
            json!({"kind": "agent", "tool": "t", "provider": "", "modelRequested": "m", "modelObserved": null}),
            json!({"kind": "agent", "tool": "t", "provider": "p", "modelRequested": "", "modelObserved": null}),
            json!({"kind": "agent", "tool": "t", "provider": "p", "modelRequested": "m", "modelObserved": 1}),
            json!({"kind": "agent", "tool": "t", "provider": "p", "modelRequested": "m"}),
            json!({"kind": "extension", "name": "Lint", "digest": "a".repeat(64)}),
            json!({"kind": "extension", "name": "lint", "digest": "a".repeat(63)}),
            json!({"kind": "extension", "name": "lint"}),
            json!({"kind": "fabrewk", "x": 1}),
            json!({"kind": "robot"}),
            json!({"alias": "xel"}),
            json!("fabrewk"),
        ];
        for actor in bad {
            assert_eq!(actor_class(&actor), None, "{actor}");
        }
    }
}
