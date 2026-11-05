//! The policy in force on a fixture record (part 7c1, D-048): adopted only through `policy-adopted`, its pending
//! changes, and the refusals of an adoption. Fixtures as in `record.rs` (core-specification 11.2 rule 2).

mod common;

use common::*;
use fabrewk_core::record::{Code, Replay, replay};
use fabrewk_core::settings::{self, Effect, Source, Val};
use serde_json::json;
use std::fs;

/// The project file of a fixture whose maestro is `xel`, with `extra` appended.
fn project_file(f: &Fixture, extra: &str) -> String {
    let key = fs::read_to_string(f.keys.join("xel.pub")).unwrap();
    format!(
        "[project]\nname = \"fixture\"\n\n[maintainers.xel]\nrole = \"maestro\"\nkey = \"{}\"\n{extra}",
        key.trim()
    )
}

fn write_settings(f: &Fixture, text: &str) {
    fs::write(f.domain.join("settings.toml"), text).unwrap();
}

/// Adopts the file on disk as `xel` and gives the record replayed.
fn adopt_on_record(f: &Fixture, widen: &[String]) -> Result<Replay, Vec<fabrewk_core::Error>> {
    let mut s = begin(f, f.repo());
    let in_force = s
        .record()
        .map(|r| settings::in_force(r).unwrap())
        .unwrap_or(None);
    let adoption = settings::adopt(&f.domain, in_force.as_ref(), widen, f.repo().as_ref())?;
    let head = s.head("project").unwrap();
    s.append(
        "project",
        head.as_ref(),
        &person("policy-adopted", "Adopto.", adoption.payload()),
        Some(&f.signer()),
    )
    .unwrap();
    s.finish(&f.verifier(), &[]).unwrap();
    Ok(replay(&f.domain, &f.verifier()).unwrap())
}

fn commit_settings(f: &Fixture) {
    git(&f.root, &["add", ".brewstillery/settings.toml"]);
    git(&f.root, &["commit", "-q", "-m", "settings"]);
}

#[test]
fn req_set_4_the_policy_is_the_last_adopted_content_whatever_the_file_says() {
    isolated(
        "req_set_4_the_policy_is_the_last_adopted_content_whatever_the_file_says",
        || {
            let f = Fixture::new("set-4", true);
            let first = project_file(&f, "");
            write_settings(&f, &first);
            commit_settings(&f);
            let record = adopt_on_record(&f, &[]).unwrap();
            let policy = settings::in_force(&record).unwrap().unwrap();
            assert_eq!(policy.text, first);
            assert_eq!(
                policy.digest,
                fabrewk_core::record::sha256_hex(first.as_bytes())
            );
            assert_eq!(settings::pending(&policy, &f.domain).unwrap(), None);

            // An edit raising a limit grants nothing until adopted: the policy keeps 3, the change shows as pending.
            write_settings(&f, &format!("{first}\n[limits]\nrounds = 5\n"));
            let record = replay(&f.domain, &f.verifier()).unwrap();
            let policy = settings::in_force(&record).unwrap().unwrap();
            let eff = settings::effective(&policy, None, &[], &[]).unwrap();
            assert_eq!(eff.values["limits.rounds"], (Val::Int(3), Source::Default));
            let pending = settings::pending(&policy, &f.domain).unwrap().unwrap();
            assert_eq!(pending.len(), 1);
            assert_eq!(
                (pending[0].key.as_str(), pending[0].effect, pending[0].line),
                ("limits.rounds", Effect::Widening, Some(9))
            );

            // A change of comments alone is pending with no change of value.
            write_settings(&f, &format!("# a note\n{first}"));
            assert_eq!(settings::pending(&policy, &f.domain).unwrap(), Some(vec![]));

            // TOML-like text in a fermento or the alambique changes no effective value (REQ-SET-1).
            write_settings(&f, &first);
            fs::create_dir_all(f.domain.join("xel")).unwrap();
            fs::write(f.domain.join("xel/fermento.md"), "[limits]\nrounds = 9\n").unwrap();
            fs::write(f.domain.join("ALAMBIQUE.md"), "[limits]\nrounds = 9\n").unwrap();
            let again = settings::effective(&policy, None, &[], &[]).unwrap();
            assert_eq!(again.values["limits.rounds"].0, Val::Int(3));

            // The second adoption, named, moves the policy; the event lists the widening.
            write_settings(&f, &format!("{first}\n[limits]\nrounds = 5\n"));
            commit_settings(&f);
            let record = adopt_on_record(&f, &["limits.rounds".into()]).unwrap();
            let policy = settings::in_force(&record).unwrap().unwrap();
            let eff = settings::effective(&policy, None, &[], &[]).unwrap();
            assert_eq!(
                eff.values["limits.rounds"],
                (Val::Int(5), Source::Project(policy.digest.clone()))
            );
            let last = record.streams["project"].last().unwrap();
            assert_eq!(last.value["payload"]["widened"], json!(["limits.rounds"]));
        },
    );
}

#[test]
fn refusal_policy_absent() {
    let f = Fixture::new("policy-absent", false);
    let e = settings::require(None, &f.domain).unwrap_err();
    assert_eq!(e.code(), Some(Code::PolicyAbsent));
    let mut s = begin(&f, None);
    let head = s.head("project").unwrap();
    s.append(
        "project",
        head.as_ref(),
        &person("assessed", "Evalúa.", json!({})),
        Some(&f.signer()),
    )
    .unwrap();
    s.finish(&f.verifier(), &[]).unwrap();
    let record = replay(&f.domain, &f.verifier()).unwrap();
    assert!(
        settings::in_force(&record).unwrap().is_none(),
        "a record with no policy-adopted has no policy"
    );
}

#[test]
fn refusal_policy_uncommitted() {
    isolated("refusal_policy_uncommitted", || {
        let f = Fixture::new("policy-uncommitted", true);
        write_settings(&f, &project_file(&f, ""));
        let codes =
            |r: Result<_, Vec<fabrewk_core::Error>>| codes(r.map(|_: settings::Adoption| ()));
        assert_eq!(
            codes(settings::adopt(&f.domain, None, &[], f.repo().as_ref())),
            [Code::PolicyUncommitted]
        );
        commit_settings(&f);
        settings::adopt(&f.domain, None, &[], f.repo().as_ref()).unwrap();
        write_settings(&f, &project_file(&f, "[git]\nsignOff = true\n"));
        git(&f.root, &["add", ".brewstillery/settings.toml"]);
        assert_eq!(
            codes(settings::adopt(&f.domain, None, &[], f.repo().as_ref())),
            [Code::PolicyUncommitted],
            "staged only"
        );
        // Unpublished: there is no commit to compare with.
        settings::adopt(&f.domain, None, &[], None).unwrap();
    });
}

#[test]
fn refusal_widening_unnamed() {
    isolated("refusal_widening_unnamed", || {
        let f = Fixture::new("widening-unnamed", true);
        let first = project_file(&f, "");
        write_settings(&f, &first);
        commit_settings(&f);
        adopt_on_record(&f, &[]).unwrap();
        let record = replay(&f.domain, &f.verifier()).unwrap();
        let policy = settings::in_force(&record).unwrap();
        let adopt = |widen: &[&str]| {
            let widen: Vec<String> = widen.iter().map(|w| (*w).to_owned()).collect();
            settings::adopt(&f.domain, policy.as_ref(), &widen, f.repo().as_ref())
        };
        write_settings(&f, &format!("{first}[limits]\nrounds = 4\n"));
        commit_settings(&f);
        assert_eq!(codes(adopt(&[]).map(|_| ())), [Code::WideningUnnamed]);
        assert_eq!(
            adopt(&["limits.rounds"]).unwrap().widened,
            ["limits.rounds"]
        );
        let ana = fs::read_to_string(f.keys.join("stranger.pub")).unwrap();
        write_settings(
            &f,
            &format!(
                "{first}[maintainers.ana]\nrole = \"contributor\"\nkey = \"{}\"\n",
                ana.trim()
            ),
        );
        commit_settings(&f);
        assert_eq!(
            codes(adopt(&["limits.rounds"]).map(|_| ())),
            [Code::WideningUnnamed]
        );
        let a = adopt(&["maintainers.ana"]).unwrap();
        assert_eq!(a.widened, ["maintainers.ana"]);
        assert_eq!(
            a.payload()["digest"],
            json!(fabrewk_core::record::sha256_hex(a.content.as_bytes()))
        );
        // A narrowing needs no name.
        write_settings(&f, &format!("{first}[limits]\nrounds = 2\n"));
        commit_settings(&f);
        assert!(adopt(&[]).unwrap().widened.is_empty());
    });
}

#[test]
fn refusal_role_not_enforced() {
    let f = Fixture::new("role-not-enforced", false);
    let text = project_file(&f, "").replace("\"maestro\"", "\"supervisor\"");
    write_settings(&f, &text);
    let r = settings::adopt(&f.domain, None, &[], None);
    assert_eq!(codes(r.map(|_| ())), [Code::RoleNotEnforced]);
}

#[test]
fn refusal_settings_invalid_at_adoption_and_in_an_adopted_policy() {
    let f = Fixture::new("settings-invalid", false);
    write_settings(&f, "[project]\nname = 1\n");
    let r = settings::adopt(&f.domain, None, &[], None);
    assert_eq!(codes(r.map(|_| ())), [Code::SettingsInvalid]);
    // A policy this binary does not accept is refused naming the event that carries it.
    let mut s = begin(&f, None);
    let text = "[project]\nname = \"x\"\nnosuch = 1\n";
    let payload = json!({"content": text, "digest": fabrewk_core::record::sha256_hex(text.as_bytes()), "widened": []});
    let head = s.head("project").unwrap();
    let event = s
        .append(
            "project",
            head.as_ref(),
            &person("policy-adopted", "Adopto.", payload),
            Some(&f.signer()),
        )
        .unwrap();
    drop(s);
    let record = replay(&f.domain, &f.verifier()).unwrap();
    let errors = settings::in_force(&record).unwrap_err();
    assert_eq!(errors[0].code(), Some(Code::SettingsInvalid));
    let name = format!("000001-{}.json", &event.digest[..16]);
    assert!(
        matches!(&errors[0], fabrewk_core::Error::Refused { path: Some(p), .. } if p.ends_with(&name))
    );
}
