//! `fabrewk config [<key>]` (REQ-SET-7, REQ-ID-5, REQ-SEC-5; section 5; part 7d2): every effective value with its
//! source, the project file's changes not adopted, each maintainer's key, what is present on this machine, every
//! control with its label, and the residual risks of the core and of the confinement backend (S-7, part 9a); with a
//! key, that one value. It takes no lock and writes nothing (REQ-VIEW-4).

use super::check::{CHECKS, control_of, label};
use super::{Cx, Failed, Out};
use crate::code::{Code, refuse};
use crate::detect::{self, How};
use crate::settings::{self, Effect, Source, Val};
use crate::{confine, identity, machine, table};
use serde_json::{Value, json};

fn value(v: &Val) -> Value {
    match v {
        Val::Bool(b) => json!(b),
        Val::Int(n) => json!(n),
        Val::Str(s) => json!(s),
        Val::List(l) => json!(l),
    }
}

/// A value as the settings files write it.
fn written(v: &Val) -> String {
    let q = |s: &String| serde_json::to_string(s).unwrap_or_default();
    match v {
        Val::Bool(b) => b.to_string(),
        Val::Int(n) => n.to_string(),
        Val::Str(s) => q(s),
        Val::List(l) => format!("[{}]", l.iter().map(q).collect::<Vec<_>>().join(", ")),
    }
}

/// A source's name, and what it names: the adopted digest, the maintainer, the stage or the task.
fn source(s: &Source) -> (&'static str, Option<&str>) {
    match s {
        Source::Default => ("default", None),
        Source::Project(d) => ("project", Some(d)),
        Source::Personal(a) => ("personal", Some(a)),
        Source::Stage(id) => ("stage", Some(id)),
        Source::Task(id) => ("task", Some(id)),
        Source::Declared => ("declared", None),
    }
}

/// The catalogue's words for a key, found by its declared pattern.
fn about(cx: &Cx, key: &str) -> String {
    let pattern = settings::declared(key).map_or(key, |k| k.path);
    cx.t(&format!("setting.{pattern}"), &[])
}

fn entry(cx: &Cx, key: &str, v: &Val, s: &Source) -> (Value, String) {
    let (name, from) = source(s);
    let short = from.map(|f| if name == "project" { &f[..12.min(f.len())] } else { f }).unwrap_or_default();
    let words = cx.t(&format!("source.{name}"), &[("from", short)]);
    let doc = json!({"key": key, "value": value(v), "source": name, "from": from, "about": about(cx, key)});
    (doc, format!("  {key} = {}\n      {}; {words}\n", written(v), about(cx, key)))
}

pub(super) fn run(cx: &Cx, key: Option<&str>) -> Result<Out, Failed> {
    if let Err(errors) = &cx.record
        && errors.iter().any(|e| e.code() != Some(Code::RecordAbsent))
    {
        return Err(Failed::Refused(errors.clone()));
    }
    let policy =
        cx.policy.as_ref().ok_or_else(|| refuse(Code::PolicyAbsent, Some(&cx.domain), "no policy is adopted"))?;
    let personal = cx.personal()?;
    let folder = cx.folder()?;
    let machine = machine::machine_file(&folder)?;
    let eff = settings::effective(policy, personal.as_ref(), machine.as_ref(), &[], &[])?;
    if let Some(key) = key {
        let found = settings::lookup(&eff, key)?;
        let (mut doc, text) = match found {
            Some((v, s)) => entry(cx, key, v, s),
            None => {
                let doc = json!({"key": key, "value": null, "source": null, "from": null, "about": about(cx, key)});
                (doc, format!("  {key}: {}\n      {}\n", cx.t("config.unset", &[]), about(cx, key)))
            }
        };
        doc["schema"] = json!("fabrewk.value/1");
        return Ok(Out::new(doc, text));
    }
    let pending = settings::pending(policy, &cx.domain)?.unwrap_or_default();
    let digest = &policy.digest[..12];
    let person = cx.person.as_deref().unwrap_or_default();
    let mut text = cx.t(
        if person.is_empty() { "config.heading" } else { "config.heading-person" },
        &[("digest", digest), ("alias", person)],
    );
    text += "\n";
    let mut values = Vec::new();
    for (k, (v, s)) in &eff.values {
        let (doc, line) = entry(cx, k, v, s);
        values.push(doc);
        text += &line;
    }
    let mut changes = Vec::new();
    if !pending.is_empty() {
        text += &format!("\n{}\n", cx.t("config.pending", &[]));
    }
    for c in &pending {
        let effect = match c.effect {
            Effect::Widening => "widening",
            Effect::Narrowing => "narrowing",
            Effect::Neutral => "neutral",
        };
        changes.push(json!({"key": c.key, "effect": effect, "place": c.place()}));
        text += &format!("  {}: {}\n", c.place(), cx.t(&format!("effect.{effect}"), &[]));
    }
    text += &format!("\n{}\n", cx.t("config.keys", &[]));
    let keys: Vec<Value> = identity::key_reports(&policy.settings)
        .into_iter()
        .map(|r| {
            let touch = cx.t(&format!("touch.{}", r.touch), &[]);
            text +=
                &format!("  {}\n", cx.t("config.key", &[("alias", &r.alias), ("kind", &r.kind), ("touch", &touch)]));
            json!({"alias": r.alias, "kind": r.kind, "touch": r.touch})
        })
        .collect();
    let path = std::env::var_os("PATH");
    let detection = detect::detect(&cx.domain, &eff, &cx.modules, path.as_deref());
    text += &format!("\n{}\n", cx.t("config.machine", &[]));
    let mut present = Vec::new();
    for ((space, name), found) in &detection.found {
        let space = space.key();
        let label = cx.t(&format!("space.{space}"), &[("name", name)]);
        let doc = match found {
            None => {
                text += &format!("  {label}: {}\n", cx.t("config.absent", &[]));
                json!({"space": space, "name": name, "present": false, "path": null, "version": null, "how": null, "verified": null})
            }
            Some(f) => {
                let how = if f.how == How::Declared { "declared" } else { "observed" };
                let shown = f.path.display().to_string();
                let version = f.version.clone().unwrap_or_else(|| cx.t("config.no-version", &[]));
                let verified = f.verified.clone().unwrap_or_else(|| cx.t("config.never", &[]));
                let line =
                    cx.t(&format!("how.{how}"), &[("path", &shown), ("version", &version), ("verified", &verified)]);
                text += &format!("  {label}: {line}\n");
                json!({"space": space, "name": name, "present": true, "path": f.path.to_str(), "version": f.version, "how": how, "verified": f.verified})
            }
        };
        present.push(doc);
    }
    let tools: Vec<String> = detect::effective_tools(&eff, &detection).into_iter().collect();
    let list = if tools.is_empty() { cx.t("config.none", &[]) } else { tools.join(", ") };
    text += &format!("{}\n", cx.t("config.tools", &[("tools", &list)]));
    text += &format!("\n{}\n", cx.t("config.controls", &[]));
    let mut controls = Vec::new();
    let mode = cx.record.as_ref().ok().and_then(crate::domain::publication);
    let word = |key: String| cx.t(&key, &[]);
    for m in table::moves() {
        let kinds = m.kinds.join(" ");
        let at: Vec<String> = m.kinds.iter().map(|k| word(format!("kind.{k}"))).collect();
        for g in &m.guards {
            let control = label(g.control);
            let (guard, how) = (word(format!("guard.{}", g.name)), word(format!("control.{control}")));
            text += &format!("  {} ({kinds}): {guard} ({}); {how}\n", at.join(", "), g.name);
            controls.push(json!({"at": kinds, "subject": m.subject, "guard": g.name, "control": control}));
        }
    }
    for c in CHECKS {
        let control = label(control_of(c, mode));
        text += &format!("  {} {}: {}\n", c.id, word(format!("check.{}", c.id)), word(format!("control.{control}")));
        controls.push(json!({"at": "check", "subject": c.id, "guard": c.id, "control": control}));
    }
    text += &format!("\n{}\n", cx.t("config.residual", &[]));
    let backend = cx.backend.map(|b| (b.name, b.residual));
    let mut residual = Vec::new();
    let adapters = cx.adapters.iter().map(|a| (a.program.adapter.unwrap_or(a.program.executable), a.residual));
    for (from, risks) in std::iter::once(("core", &confine::RESIDUAL[..])).chain(backend).chain(adapters) {
        for risk in risks {
            text += &format!("  {} ({from}, {risk})\n", word(format!("residual.{risk}")));
            residual.push(json!({"from": from, "risk": risk}));
        }
    }
    if cx.backend.is_none() {
        text += &format!("  {}\n", cx.t("config.no-backend", &[]));
    }
    let doc = json!({
        "schema": "fabrewk.config/2", "policy": policy.digest, "person": cx.person, "values": values,
        "pending": changes, "keys": keys, "machine": present, "tools": tools, "controls": controls,
        "backend": backend.map(|b| b.0), "residual": residual,
    });
    Ok(Out::new(doc, text))
}
