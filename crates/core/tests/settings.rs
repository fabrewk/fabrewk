//! The policy in force on a fixture record (part 7c1, D-048): adopted only through `policy-adopted`, its pending
//! changes, and the refusals of an adoption. Fixtures as in `record.rs` (core-specification 11.2 rule 2).

mod common;

use common::*;
use fabrewk_core::record::{Code, Publication, Replay, Repo, replay};
use fabrewk_core::settings::{self, Effect, Source, Val};
use serde_json::json;
use std::fs;

/// The project file of a fixture whose maestro is `xel`, with `extra` appended.
fn project_file(f: &Fixture, extra: &str) -> String {
    let key = fs::read_to_string(f.keys.join("xel.pub")).unwrap();
    format!("[project]\nname = \"fixture\"\n\n[maintainers.xel]\nrole = \"maestro\"\nkey = \"{}\"\n{extra}", key.trim())
}

fn write_settings(f: &Fixture, text: &str) {
    fs::write(f.domain.join("settings.toml"), text).unwrap();
}

/// Adopts the file on disk as `xel` and gives the record replayed.
fn adopt_on_record(f: &Fixture, widen: &[String]) -> Result<Replay, Vec<fabrewk_core::Error>> {
    adopt_in(f, f.repo(), widen)
}

/// As [`adopt_on_record`], in the mode `repo` stands for; the settings file is byte-identical after (REQ-SET-1).
fn adopt_in(f: &Fixture, repo: Option<Repo>, widen: &[String]) -> Result<Replay, Vec<fabrewk_core::Error>> {
    let before = fs::read(f.domain.join("settings.toml")).unwrap();
    let mut s = begin(f, repo.clone());
    let in_force = s.record().map(|r| settings::in_force(r).unwrap()).unwrap_or(None);
    let adoption = settings::adopt(&f.domain, in_force.as_ref(), widen, repo.as_ref())?;
    assert_eq!(fs::read(f.domain.join("settings.toml")).unwrap(), before);
    let head = s.head("project").unwrap();
    let payload = adoption.payload(&s.seal().unwrap());
    s.append("project", head.as_ref(), &person("policy-adopted", "Adopto.", payload), Some(&f.signer())).unwrap();
    s.finish(&f.verifier(), &[]).unwrap();
    Ok(replay(&f.domain, &f.verifier()).unwrap())
}

fn commit_settings(f: &Fixture) {
    git(&f.root, &["add", ".brewstillery/settings.toml"]);
    git(&f.root, &["commit", "-q", "-m", "settings"]);
}

#[test]
fn req_set_4_the_policy_is_the_last_adopted_content_whatever_the_file_says() {
    isolated("req_set_4_the_policy_is_the_last_adopted_content_whatever_the_file_says", || {
        let f = Fixture::new("set-4", true);
        let first = project_file(&f, "");
        write_settings(&f, &first);
        commit_settings(&f);
        let record = adopt_on_record(&f, &[]).unwrap();
        let policy = settings::in_force(&record).unwrap().unwrap();
        assert_eq!(policy.text, first);
        assert_eq!(policy.digest, fabrewk_core::record::sha256_hex(first.as_bytes()));
        assert_eq!(settings::pending(&policy, &f.domain).unwrap(), None);

        // An edit raising a limit grants nothing until adopted: the policy keeps 3, the change shows as pending.
        write_settings(&f, &format!("{first}\n[limits]\nrounds = 5\n"));
        let record = replay(&f.domain, &f.verifier()).unwrap();
        let policy = settings::in_force(&record).unwrap().unwrap();
        let eff = settings::effective(&policy, None, None, &[], &[]).unwrap();
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
        let personal_file = f.domain.join("xel/settings.toml");
        let personal_text = "maintainer = {\n  alias = \"xel\",\n}\nlanguages.agentInteraction = \"es-MX\"\n";
        fs::write(&personal_file, personal_text).unwrap();
        let personal = settings::read(&personal_file, settings::Level::Personal).unwrap();
        let again = settings::effective(&policy, Some(&personal), None, &[], &[]).unwrap();
        assert_eq!(again.values["languages.agentInteraction"].1, Source::Personal("xel".into()));
        assert_eq!(fs::read(&personal_file).unwrap(), personal_text.as_bytes(), "never rewritten");
        assert_eq!(again.values["limits.rounds"].0, Val::Int(3));

        // The second adoption, named, moves the policy; the event lists the widening.
        write_settings(&f, &format!("{first}\n[limits]\nrounds = 5\n"));
        commit_settings(&f);
        let record = adopt_on_record(&f, &["limits.rounds".into()]).unwrap();
        let policy = settings::in_force(&record).unwrap().unwrap();
        let eff = settings::effective(&policy, None, None, &[], &[]).unwrap();
        assert_eq!(eff.values["limits.rounds"], (Val::Int(5), Source::Project(policy.digest.clone())));
        let last = record.streams["project"].last().unwrap();
        assert_eq!(last.value["payload"]["widened"], json!(["limits.rounds"]));

        let raised = format!("{first}\n[limits]\nrounds = 5\n");
        // A removal is located at its line in the policy in force.
        write_settings(&f, &raised.replace("[maintainers.xel]", "[maintainers.xel]\nhostAccount = \"x\""));
        commit_settings(&f);
        adopt_on_record(&f, &["maintainers.xel.hostAccount".into()]).unwrap();
        let record = replay(&f.domain, &f.verifier()).unwrap();
        let with_account = settings::in_force(&record).unwrap().unwrap();
        write_settings(&f, &raised);
        let gone = settings::pending(&with_account, &f.domain).unwrap().unwrap();
        assert_eq!((gone[0].line, gone[0].was), (None, Some(5)));
        assert_eq!(gone[0].place(), "maintainers.xel.hostAccount (removed; line 5 of the policy in force)");
    });
}

#[test]
fn refusal_policy_absent() {
    let f = Fixture::new("policy-absent", false);
    let e = settings::require(None, &f.domain).unwrap_err();
    assert_eq!(e.code(), Some(Code::PolicyAbsent));
    let mut s = begin(&f, None);
    let head = s.head("project").unwrap();
    s.append("project", head.as_ref(), &person("assessed", "Evalúa.", json!({})), Some(&f.signer())).unwrap();
    s.finish(&f.verifier(), &[]).unwrap();
    let record = replay(&f.domain, &f.verifier()).unwrap();
    assert!(settings::in_force(&record).unwrap().is_none(), "a record with no policy-adopted has no policy");
}

#[test]
fn refusal_policy_uncommitted() {
    isolated("refusal_policy_uncommitted", || {
        let f = Fixture::new("policy-uncommitted", true);
        write_settings(&f, &project_file(&f, ""));
        let codes = |r: Result<_, Vec<fabrewk_core::Error>>| codes(r.map(|_: settings::Adoption| ()));
        assert_eq!(codes(settings::adopt(&f.domain, None, &[], f.repo().as_ref())), [Code::PolicyUncommitted]);
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
        let refusal = adopt(&[]).unwrap_err();
        assert_eq!(refusal[0].code(), Some(Code::WideningUnnamed));
        assert!(format!("{:?}", refusal[0]).contains("limits.rounds (line 8)"), "{refusal:?}");
        assert_eq!(adopt(&["limits.rounds"]).unwrap().widened, ["limits.rounds"]);
        let ana = fs::read_to_string(f.keys.join("stranger.pub")).unwrap();
        write_settings(&f, &format!("{first}[maintainers.ana]\nrole = \"contributor\"\nkey = \"{}\"\n", ana.trim()));
        commit_settings(&f);
        assert_eq!(codes(adopt(&["limits.rounds"]).map(|_| ())), [Code::WideningUnnamed]);
        let a = adopt(&["maintainers.ana"]).unwrap();
        assert_eq!(a.widened, ["maintainers.ana"]);
        assert_eq!(
            a.payload(&Default::default())["digest"],
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
    // Only a role is a role: the word elsewhere is a value like any other.
    write_settings(&f, &project_file(&f, "").replace("name = \"fixture\"", "name = \"supervisor\""));
    settings::adopt(&f.domain, None, &[], None).unwrap();
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
    let payload = json!({"content": text, "digest": fabrewk_core::record::sha256_hex(text.as_bytes()), "heads": {}, "widened": []});
    let head = s.head("project").unwrap();
    let event =
        s.append("project", head.as_ref(), &person("policy-adopted", "Adopto.", payload), Some(&f.signer())).unwrap();
    drop(s);
    let record = replay(&f.domain, &f.verifier()).unwrap();
    let errors = settings::in_force(&record).unwrap_err();
    assert_eq!(errors[0].code(), Some(Code::SettingsInvalid));
    let name = format!("000001-{}.json", &event.digest[..16]);
    assert!(matches!(&errors[0], fabrewk_core::Error::Refused { path: Some(p), .. } if p.ends_with(&name)));
}

#[test]
fn req_set_4_in_each_mode() {
    isolated("req_set_4_in_each_mode", || {
        for mode in [Publication::OwnRepository, Publication::Unpublished] {
            let f = Fixture::new(&format!("set-4-{mode:?}"), false);
            if mode == Publication::OwnRepository {
                for args in [
                    &["init", "-q", "-b", "main"][..],
                    &["config", "user.name", "Fixture"],
                    &["config", "user.email", "f@example.invalid"],
                    &["config", "commit.gpgsign", "false"],
                    &["add", "."],
                    &["commit", "-q", "-m", "domain"],
                ] {
                    git(&f.domain, args);
                }
            }
            let repo = Repo::for_domain(&f.domain, mode);
            let first = project_file(&f, "");
            write_settings(&f, &first);
            if let Some(r) = &repo {
                let refused = settings::adopt(&f.domain, None, &[], Some(r)).map(|_| ());
                assert_eq!(codes(refused), [Code::PolicyUncommitted], "{mode:?}");
                git(&r.root, &["add", "settings.toml"]);
                git(&r.root, &["commit", "-q", "-m", "settings"]);
            }
            adopt_in(&f, repo, &[]).unwrap();
            write_settings(&f, &format!("{first}[limits]\nrounds = 5\n"));
            let record = replay(&f.domain, &f.verifier()).unwrap();
            let policy = settings::in_force(&record).unwrap().unwrap();
            assert_eq!(policy.text, first, "{mode:?}");
            let eff = settings::effective(&policy, None, None, &[], &[]).unwrap();
            assert_eq!(eff.values["limits.rounds"].0, Val::Int(3), "{mode:?}");
            let pending = settings::pending(&policy, &f.domain).unwrap().unwrap();
            assert_eq!((pending[0].key.as_str(), pending[0].effect), ("limits.rounds", Effect::Widening));
        }
    });
}
