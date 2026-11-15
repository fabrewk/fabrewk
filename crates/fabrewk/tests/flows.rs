//! The whole flows of core-specification 11.5 through the built binary, with fake tools (11.2 rule 3; part 9i): F-1, a
//! stage administered from its proposal to its close in `published`; F-2, the same flow in `own-repository` and in
//! `unpublished`; F-3, a maestro and a contributor in two clones. After every write command `check` exits 0, the views
//! equal a fresh generation, and each repository that holds the record has one record commit for the command, with one
//! `Fabrewk-Event` trailer per event it wrote, preceded by the alambique's own commit for `approve` and by the
//! integration's for `accept`. These tests run real cells and need `bwrap`, `pasta` and the network.

mod common;

use common::flow::*;
use common::*;
use fabrewk_core::record::sha256_hex;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Where a mode's record commits go: the product's repository, the domain's own, or none.
fn record_repo(fx: &Fx, mode: &str) -> Option<PathBuf> {
    match mode {
        "published" => Some(fx.root.clone()),
        "own-repository" => Some(fx.domain.clone()),
        _ => None,
    }
}

/// Every event file of the record by its digest (the file's bytes), with its path.
fn event_files(fx: &Fx) -> BTreeSet<(String, PathBuf)> {
    let mut out = BTreeSet::new();
    for stream in fs::read_dir(fx.domain.join("data/events")).into_iter().flatten().flatten() {
        for file in fs::read_dir(stream.path()).unwrap().flatten() {
            let path = file.path();
            if path.extension().is_some_and(|e| e == "json") {
                out.insert((sha256_hex(&fs::read(&path).unwrap()), path));
            }
        }
    }
    out
}

/// The flow's state between commands: the repositories' heads and the record's event files.
struct Flow<'a> {
    fx: &'a Fx,
    mode: &'a str,
    product: String,
    record: Option<String>,
    events: BTreeSet<(String, PathBuf)>,
}

impl<'a> Flow<'a> {
    fn new(fx: &'a Fx, mode: &'a str) -> Flow<'a> {
        let mut f = Flow { fx, mode, product: String::new(), record: None, events: BTreeSet::new() };
        f.mark();
        whole_and_fresh(fx);
        f
    }

    fn mark(&mut self) {
        self.product = self.fx.git(&["rev-parse", "HEAD"]);
        self.record = record_repo(self.fx, self.mode).map(|r| git_in(self.fx, &r, &["rev-parse", "HEAD"]));
        self.events = event_files(self.fx);
    }

    /// Runs the write command `args`, which must succeed, and checks what it left (see the module's description);
    /// gives its `--json` document.
    fn write(&mut self, args: &[&str], before: Before) -> Value {
        let ran = self.fx.run(&[args, &["--json"]].concat());
        assert_eq!(ran.status, 0, "{args:?}: {ran:?}");
        let fx = self.fx;
        whole_and_fresh(fx);
        let now = event_files(fx);
        assert!(self.events.is_subset(&now), "{args:?} lost an event");
        let new: BTreeSet<String> = now.difference(&self.events).map(|(d, _)| d.clone()).collect();
        assert!(!new.is_empty(), "{args:?} wrote no event");
        let repo = record_repo(fx, self.mode);
        let in_product = repo.as_deref() == Some(fx.root.as_path());
        let product_new = self.commits(&fx.root, &self.product);
        let record_new = match &repo {
            Some(_) if in_product => product_new.clone(),
            Some(r) => self.commits(r, self.record.as_ref().unwrap()),
            None => Vec::new(),
        };
        // What precedes the record commit in its repository.
        let preceding = record_new.split_last().map_or(&[][..], |(_, rest)| rest);
        if let Some(repo) = &repo {
            // One record commit, the last, naming exactly the events the command wrote; nothing of the domain left
            // out of it.
            let last = record_new.last().unwrap_or_else(|| panic!("{args:?}: no record commit"));
            assert_eq!(git_in(fx, repo, &["log", "-1", "--format=%s", last]), "fabrewk: record", "{args:?}");
            let trailers = git_in(fx, repo, &["log", "-1", "--format=%(trailers:key=Fabrewk-Event,valueonly)", last]);
            let named: BTreeSet<String> = trailers.lines().filter(|l| !l.is_empty()).map(str::to_owned).collect();
            assert_eq!(named, new, "{args:?}");
            let domain = if in_product { ".brewstillery" } else { "." };
            assert_eq!(git_in(fx, repo, &["status", "--porcelain", "--", domain]), "", "{args:?}");
        } else {
            // `unpublished`: no commit of the record, and the product's exclude line in place.
            let exclude = fs::read_to_string(fx.root.join(".git/info/exclude")).unwrap();
            assert!(exclude.lines().any(|l| l == "/.brewstillery/"), "{exclude}");
        }
        match before {
            Before::Nothing => assert!(preceding.is_empty(), "{args:?}: {record_new:?}"),
            Before::Alambique => {
                // The alambique alone, in the repository of the record; none where the mode commits nothing.
                assert_eq!(preceding.len(), usize::from(repo.is_some()), "{args:?}: {record_new:?}");
                if let (Some(repo), Some(commit)) = (&repo, preceding.first()) {
                    let files = git_in(fx, repo, &["show", "--name-only", "--format=", commit]);
                    assert_eq!(files, if in_product { ".brewstillery/ALAMBIQUE.md" } else { "ALAMBIQUE.md" });
                }
            }
            Before::Integration => {
                // The branch moved from where it was to the candidate itself, or to an integration commit that names
                // it and whose only parent is where it was (REQ-GATE-8); in `published` the record commit follows.
                let integrated =
                    new_events(&now, &self.events).into_iter().find(|(_, e)| e["kind"] == "task-integrated");
                let (stream, integrated) = integrated.unwrap_or_else(|| panic!("{args:?}: no integration"));
                let integrated = integrated["payload"].clone();
                let to = if in_product {
                    assert_eq!(preceding.len(), 1, "{args:?}: {record_new:?}");
                    preceding[0].clone()
                } else {
                    assert!(preceding.is_empty(), "{args:?}: {record_new:?}");
                    fx.git(&["rev-parse", "HEAD"])
                };
                assert_eq!(integrated, json!({"from": self.product, "to": to}), "{args:?}");
                let candidate = fx.events(&stream);
                let candidate = candidate.iter().rfind(|e| e["kind"] == "candidate-recorded").unwrap();
                let candidate = candidate["payload"]["commit"].as_str().unwrap();
                let named = fx.git(&["log", "-1", "--format=%(trailers:key=Fabrewk-Candidate,valueonly)", &to]);
                if named.is_empty() {
                    assert_eq!(to, candidate, "{args:?}");
                } else {
                    assert_eq!(
                        (named.as_str(), fx.git(&["rev-parse", &format!("{to}^@")])),
                        (candidate, self.product.clone())
                    );
                }
                assert_eq!(fx.git(&["status", "--porcelain", "--", "src"]), "", "{args:?}");
            }
        }
        // Outside `published` the product moves only by an integration.
        if !in_product && before != Before::Integration {
            assert!(product_new.is_empty(), "{args:?}: the product moved: {product_new:?}");
        }
        self.mark();
        ran.doc()
    }

    /// The commits of `repo` after `since`, oldest first, along first parents.
    fn commits(&self, repo: &Path, since: &str) -> Vec<String> {
        let log = git_in(self.fx, repo, &["rev-list", "--reverse", "--first-parent", &format!("{since}..HEAD")]);
        log.lines().map(str::to_owned).collect()
    }
}

/// The events of `now` that are not in `before`, each with its stream, as JSON.
fn new_events(now: &BTreeSet<(String, PathBuf)>, before: &BTreeSet<(String, PathBuf)>) -> Vec<(String, Value)> {
    let stream = |p: &Path| p.parent().unwrap().file_name().unwrap().to_str().unwrap().to_owned();
    now.difference(before).map(|(_, p)| (stream(p), serde_json::from_slice(&fs::read(p).unwrap()).unwrap())).collect()
}

/// `check` exits 0, and the views are what a fresh generation from the record gives (REQ-VIEW-3).
fn whole_and_fresh(fx: &Fx) {
    let ran = fx.run(&["check"]);
    assert_eq!(ran.status, 0, "{ran:?}");
    let record = fabrewk_core::record::replay(&fx.domain, &fabrewk_core::identity::PolicyVerifier).unwrap();
    let lang = fabrewk_core::view::language(&record);
    assert_eq!(fs::read_to_string(fx.domain.join("STATUS.md")).unwrap(), fabrewk_core::view::status(&record, lang));
    assert_eq!(fs::read_to_string(fx.domain.join("RULES.md")).unwrap(), fabrewk_core::view::rules(&record, lang));
}

/// A second clone of `fx`'s repository, at `name` beside it, whose person is `ana` (the fixture's stranger's key), with
/// a home of its own; the binary runs there through the [`Fx`] it gives.
fn clone(fx: &Fx, name: &str) -> Fx {
    let base = fx.root.parent().unwrap().join(name);
    let (root, home) = (base.join("repo"), base.join("home"));
    fs::create_dir_all(&home).unwrap();
    fx.git(&["clone", "-q", fx.root.to_str().unwrap(), root.to_str().unwrap()]);
    let other = Fx { domain: root.join(".brewstillery"), root, home, keys: fx.keys.clone(), path: fx.path.clone() };
    for (k, v) in [("user.name", "Ana"), ("user.email", "ana@example.invalid"), ("commit.gpgsign", "false")] {
        other.git(&["config", k, v]);
    }
    other.git(&["config", "user.signingkey", fx.keys.join("stranger.pub").to_str().unwrap()]);
    other
}

/// Brings `from`'s branch into `into`'s, the person taking `into`'s side where both changed a text (a view, an answer).
fn merge(into: &Fx, from: &Fx) {
    into.git(&["fetch", "-q", from.root.to_str().unwrap(), "main"]);
    into.git(&["merge", "-q", "--no-edit", "-X", "ours", "FETCH_HEAD"]);
}

/// F-3 (core-specification 11.5): a maestro and a contributor in two clones. The contributor's moves are refused
/// `ROLE_FORBIDDEN`; their fermento is swept as the maestro's is; events written in both clones on one stream and
/// merged are `RECORD_FORK`, which every write command refuses, handing it to a person.
#[test]
fn f_3_a_maestro_and_a_contributor_in_two_clones() {
    let maestro = Fx::builder("f-3", "published", &format!("{BUILDS}{SUCCESS}"));
    let ana = format!("\n[maintainers.ana]\nrole = \"contributor\"\nkey = \"{}\"\n", public(&maestro.keys, "stranger"));
    maestro.edit_policy(&format!("{}{ana}", fs::read_to_string(maestro.domain.join("settings.toml")).unwrap()));
    fs::create_dir(maestro.domain.join("ana")).unwrap();
    let personal = "[maintainer]\nalias = \"ana\"\n\n[languages]\nagentInteraction = \"es-MX\"\n";
    fs::write(maestro.domain.join("ana/settings.toml"), personal).unwrap();
    fs::write(maestro.domain.join("ana/fermento.md"), format!("{INPUT}{}", fabrewk_core::fermento::block("en", "ana")))
        .unwrap();
    maestro.commit("ana's space");
    // Before the adoption the project file lists a maintainer the policy does not: `check` reports it.
    assert_eq!(maestro.run(&["adopt", "--say", SAY, "--widen", "maintainers.ana"]).status, 0);
    let mut a = Flow::new(&maestro, "published");
    let doc = a.write(&["propose", &plan(&maestro)], Before::Nothing);
    let stage = doc["stage"]["id"].as_str().unwrap().to_owned();
    let task = doc["tasks"][0]["id"].as_str().unwrap().to_owned();

    let contributor = clone(&maestro, "ana");
    whole_and_fresh(&contributor);
    // The contributor makes no person's move (REQ-ID-3), each refused with nothing written, in their language.
    for args in [
        vec!["authorize", &stage, "--seen", &shown(&contributor, &stage), "--say", SAY],
        vec!["reject", &task, "--say", SAY],
        vec!["adopt", "--say", SAY],
    ] {
        let e = refused(&contributor, &args, "ROLE_FORBIDDEN", 1, "es");
        assert!(e["detail"].as_str().unwrap().contains("contributor"), "{args:?}: {e}");
    }
    // Their fermento is swept as the maestro's is: in their clone, then brought to the maestro's.
    let mut b = Flow::new(&contributor, "published");
    let swept = b.write(&["sweep", "ana", "--answer", &answer(&contributor, "ana", &[])], Before::Nothing);
    merge(&maestro, &contributor);
    whole_and_fresh(&maestro);
    a.mark();
    let mine = a.write(&["sweep", "xel", "--answer", &answer(&maestro, "xel", &[])], Before::Nothing);
    let (theirs, ours) = (maestro.events("fermento-ana").pop().unwrap(), maestro.events("fermento-xel").pop().unwrap());
    assert_eq!((theirs["kind"].clone(), theirs["actor"].clone()), (ours["kind"].clone(), ours["actor"].clone()));
    assert_eq!(swept["points"], mine["points"], "{swept} {mine}");
    for alias in ["ana", "xel"] {
        let zone = fs::read_to_string(maestro.domain.join(alias).join("fermento.md")).unwrap();
        let separator = zone.find(&fabrewk_core::fermento::block("en", alias)).unwrap();
        assert!(zone[separator..].contains("About 1"), "{alias}: {zone}");
    }

    // Both clones sweep the maestro's fermento apart, on one stream: merged, the record is forked.
    merge(&contributor, &maestro);
    b.mark();
    b.write(&["sweep", "xel", "--answer", &answer(&contributor, "xel", &[])], Before::Nothing);
    a.write(&["sweep", "xel", "--answer", &answer_saying(&maestro, "xel", &[], "Say it here.")], Before::Nothing);
    merge(&maestro, &contributor);
    let doc = maestro.run(&["check", "--json"]).doc();
    let codes: Vec<&str> = doc["findings"].as_array().unwrap().iter().map(|f| f["code"].as_str().unwrap()).collect();
    assert!(codes.contains(&"RECORD_FORK"), "{doc}");
    for args in [vec!["close", &stage, "--say", SAY], vec!["sweep", "xel", "--answer", &answer(&maestro, "xel", &[])]] {
        // A record that does not replay names no person, so its words are in the canonical language; its next step
        // hands the fork to a person.
        let e = refused(&maestro, &args, "RECORD_FORK", 1, "en");
        assert!(e["path"].as_str().unwrap().ends_with("data/events/fermento-xel"), "{e}");
    }
}

#[test]
fn f_1_a_stage_from_its_proposal_to_its_close_in_published() {
    let fx = fixture("f-1", "published");
    let mut flow = Flow::new(&fx, "published");
    f1(&fx, &mut |args, before| flow.write(args, before));
}

#[test]
fn f_2_the_same_flow_in_own_repository_and_unpublished() {
    for mode in ["own-repository", "unpublished"] {
        let fx = fixture(&format!("f-2-{mode}"), mode);
        let product = fx.git(&["rev-list", "--count", "HEAD"]);
        let mut flow = Flow::new(&fx, mode);
        f1(&fx, &mut |args, before| flow.write(args, before));
        // No record commit in the product: its history grew only by the candidates its integrations brought.
        let log = fx.git(&["log", "--format=%s"]);
        assert!(!log.lines().any(|s| s == "fabrewk: record"), "{mode}: {log}");
        assert_ne!(fx.git(&["rev-list", "--count", "HEAD"]), product, "{mode}");
    }
}
