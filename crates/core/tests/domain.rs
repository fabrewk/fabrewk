//! The domain on fixture records (part 7c2b, D-051): its layout (REQ-DOM-2), its ignore file (REQ-DOM-4), the cache
//! write commands rebuild and reading ones leave absent (REQ-DOM-5), and the publication mode with the check that the
//! product ignores the domain (REQ-DOM-7). Fixtures as in `record.rs` (core-specification 11.2 rule 2).

mod common;

use common::*;
use fabrewk_core::detect;
use fabrewk_core::domain::{self, GITIGNORE};
use fabrewk_core::identity;
use fabrewk_core::record::{Code, Publication, Repo, check_mark, replay};
use fabrewk_core::settings::{self, Level};
use serde_json::json;
use std::collections::BTreeMap;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

/// Each finding of the layout as (name, code), with no publication mode on record.
fn findings(domain: &Path) -> Vec<(String, Code)> {
    findings_in(domain, None)
}

/// Each finding of the layout as (name, code), under the publication mode `mode`.
fn findings_in(domain: &Path, mode: Option<Publication>) -> Vec<(String, Code)> {
    let found = domain::layout(domain, mode).unwrap();
    let name = |e: &fabrewk_core::Error| match e {
        fabrewk_core::Error::Refused { path: Some(p), .. } => p.file_name().unwrap().to_string_lossy().into_owned(),
        other => panic!("{other:?}"),
    };
    found.iter().map(|e| (name(e), e.code().unwrap())).collect()
}

#[test]
fn req_dom_2_the_top_holds_only_the_listed_names_and_anything_else_is_reported_never_deleted() {
    let f = Fixture::new("dom-2", false);
    let d = &f.domain;
    for file in ["ALAMBIQUE.md", "STATUS.md", "RULES.md", "settings.toml"] {
        fs::write(d.join(file), "x\n").unwrap();
    }
    // `skills/` holds the skills the maestros approved (D-060): reserved, so never an alias, and listed.
    assert!(!settings::is_alias("skills"));
    for dir in ["data", "local", "skills", "xel", "ana-maria", "a1"] {
        fs::create_dir(d.join(dir)).unwrap();
    }
    assert_eq!(findings(d), []);

    // A file is an unknown entry: a warning, and the file is kept.
    fs::write(d.join("notes.md"), "mine\n").unwrap();
    assert_eq!(findings(d), [("notes.md".into(), Code::DomainUnknownEntry)]);
    assert_eq!(fs::read_to_string(d.join("notes.md")).unwrap(), "mine\n");
    fs::remove_file(d.join("notes.md")).unwrap();

    // A directory whose name is not an alias is `ALIAS_INVALID`; a reserved name is an unknown entry (D-051).
    let long = "a".repeat(33);
    for dir in ["Xel", "9a", "a--b", "a-", "-x", long.as_str(), "bootstrap", "review", "settings.toml.d"] {
        fs::create_dir(d.join(dir)).unwrap();
    }
    fs::write(d.join("bob"), "a file with an alias's name\n").unwrap();
    symlink(d.join("xel"), d.join("carol")).unwrap();
    assert_eq!(
        findings(d),
        [
            ("-x".into(), Code::AliasInvalid),
            ("9a".into(), Code::AliasInvalid),
            ("Xel".into(), Code::AliasInvalid),
            ("a-".into(), Code::AliasInvalid),
            ("a--b".into(), Code::AliasInvalid),
            (long.clone(), Code::AliasInvalid),
            ("bob".into(), Code::DomainUnknownEntry),
            ("bootstrap".into(), Code::DomainUnknownEntry),
            ("carol".into(), Code::DomainUnknownEntry),
            ("review".into(), Code::DomainUnknownEntry),
            ("settings.toml.d".into(), Code::AliasInvalid),
        ]
    );
    for dir in ["Xel", "9a", "a--b", "a-", "-x", long.as_str(), "bootstrap", "review", "settings.toml.d"] {
        assert!(d.join(dir).is_dir(), "{dir} kept");
    }

    // A name that is not UTF-8 is no alias: a directory so named is `ALIAS_INVALID`, a file an unknown entry.
    let f = Fixture::new("dom-2-bytes", false);
    let odd = |tail: &[u8]| f.domain.join(std::ffi::OsStr::from_bytes(&[b"x", tail].concat()));
    fs::create_dir(odd(b"\xff")).unwrap();
    fs::write(odd(b"\xfe"), "x\n").unwrap();
    let codes: Vec<Code> = findings(&f.domain).into_iter().map(|(_, c)| c).collect();
    assert_eq!(codes, [Code::DomainUnknownEntry, Code::AliasInvalid]);
}

#[test]
fn req_dom_2_a_listed_name_of_the_wrong_kind_is_not_the_listed_entry() {
    let f = Fixture::new("dom-2-kinds", false);
    let d = &f.domain;
    fs::create_dir(d.join("settings.toml")).unwrap();
    fs::write(d.join("data"), "a file\n").unwrap();
    fs::create_dir(d.join("elsewhere")).unwrap();
    symlink(d.join("elsewhere"), d.join("local")).unwrap();
    fs::remove_file(d.join(".gitignore")).unwrap();
    symlink(f.keys.join("allowed"), d.join(".gitignore")).unwrap();
    fs::write(d.join("STATUS.md"), "x\n").unwrap();
    symlink(d.join("STATUS.md"), d.join("RULES.md")).unwrap();
    fs::write(d.join("skills"), "a file\n").unwrap();
    assert_eq!(
        findings(d),
        [
            (".gitignore".into(), Code::DomainUnknownEntry),
            ("RULES.md".into(), Code::DomainUnknownEntry),
            ("data".into(), Code::DomainUnknownEntry),
            ("local".into(), Code::DomainUnknownEntry),
            ("settings.toml".into(), Code::DomainUnknownEntry),
            ("skills".into(), Code::DomainUnknownEntry),
        ]
    );

    // A link named `skills` is not the skills' directory either (D-060).
    let f = Fixture::new("dom-2-skills-link", false);
    fs::create_dir(f.domain.join("elsewhere")).unwrap();
    symlink(f.domain.join("elsewhere"), f.domain.join("skills")).unwrap();
    assert_eq!(findings(&f.domain), [("skills".into(), Code::DomainUnknownEntry)]);
}

/// D-091: in `own-repository` the domain's own repository is part of it, as a directory; in another mode, with no mode
/// on record, or not as a directory, `.git` is still a finding.
#[test]
fn req_dom_2_the_domain_s_own_git_is_part_of_it_only_in_own_repository() {
    let f = Fixture::new("dom-2-git", false);
    let d = &f.domain;
    fs::create_dir(d.join(".git")).unwrap();
    assert_eq!(findings_in(d, Some(Publication::OwnRepository)), []);
    for mode in [None, Some(Publication::Published), Some(Publication::Unpublished)] {
        assert_eq!(findings_in(d, mode), [(".git".into(), Code::AliasInvalid)], "{mode:?}");
    }
    fs::remove_dir(d.join(".git")).unwrap();
    fs::write(d.join(".git"), "gitdir: elsewhere\n").unwrap();
    assert_eq!(findings_in(d, Some(Publication::OwnRepository)), [(".git".into(), Code::DomainUnknownEntry)]);
    fs::remove_file(d.join(".git")).unwrap();
    fs::create_dir(d.join("elsewhere")).unwrap();
    symlink(d.join("elsewhere"), d.join(".git")).unwrap();
    assert_eq!(findings_in(d, Some(Publication::OwnRepository)), [(".git".into(), Code::DomainUnknownEntry)]);
}

#[test]
fn req_dom_2_this_repository_s_domain_holds_only_the_bootstrap_beside_the_listed_names() {
    let domain = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.brewstillery");
    assert_eq!(findings(&domain), [("bootstrap".into(), Code::DomainUnknownEntry)]);
}

#[test]
fn refusal_gitignore_changed() {
    let f = Fixture::new("gitignore", false);
    let path = f.domain.join(".gitignore");
    domain::check_gitignore(&f.domain).unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), GITIGNORE);
    for changed in ["local/\nnotes/\n", "local/", "local/\r\n", "/local/\n", ""] {
        fs::write(&path, changed).unwrap();
        let e = domain::check_gitignore(&f.domain).unwrap_err();
        assert_eq!(e.code(), Some(Code::GitignoreChanged), "{changed:?}");
    }
    fs::remove_file(&path).unwrap();
    assert_eq!(domain::check_gitignore(&f.domain).unwrap_err().code(), Some(Code::GitignoreChanged));
    let elsewhere = f.keys.join("ignore");
    fs::write(&elsewhere, GITIGNORE).unwrap();
    symlink(&elsewhere, &path).unwrap();
    assert_eq!(domain::check_gitignore(&f.domain).unwrap_err().code(), Some(Code::GitignoreChanged));
    fs::remove_file(&path).unwrap();
    fs::create_dir(&path).unwrap();
    assert_eq!(domain::check_gitignore(&f.domain).unwrap_err().code(), Some(Code::GitignoreChanged));
}

#[test]
fn req_dom_7_publication_decided_holds_the_mode_and_the_words() {
    let f = Fixture::new("dom-7-payload", false);
    let mut s = begin(&f, None);
    assert_eq!(domain::publication(s.record().unwrap_or(&Default::default())), None);
    let mut head = None;
    let modes = [("published", "Se publica."), ("unpublished", "No se publica."), ("own-repository", "Aparte.")];
    for (mode, words) in modes {
        let payload = json!({"mode": mode, "warningShown": mode == "unpublished"});
        let draft = person("publication-decided", words, payload);
        head = Some(s.append("project", head.as_ref(), &draft, Some(&f.signer())).unwrap());
        s.finish(&f.verifier(), &[]).unwrap();
        let r = replay(&f.domain, &f.verifier()).unwrap();
        let last = r.streams["project"].last().unwrap();
        assert_eq!(last.value["say"]["text"], words);
        assert_eq!(last.value["payload"]["warningShown"], mode == "unpublished");
        let expected = match mode {
            "published" => Publication::Published,
            "unpublished" => Publication::Unpublished,
            _ => Publication::OwnRepository,
        };
        assert_eq!(domain::publication(&r), Some(expected));
        s = begin(&f, None);
    }
    drop(s);

    // Any other payload is not schema 1 as this binary knows it.
    let prev = fabrewk_core::record::sha256_hex(&fs::read(f.events("project").last().unwrap()).unwrap());
    for bad in [
        json!({}),
        json!({"mode": "published"}),
        json!({"mode": "public", "warningShown": false}),
        json!({"mode": "published", "warningShown": "no"}),
        json!({"mode": "published", "warningShown": false, "words": "x"}),
        json!({"mode": 1, "warningShown": false}),
    ] {
        let path =
            f.forge("project", 3, Some(&prev), &person("publication-decided", "x", bad.clone()), Some(&f.signer()));
        assert_eq!(codes(replay(&f.domain, &f.verifier())), [Code::RecordVersionUnknown], "{bad}");
        fs::remove_file(sig(&path)).unwrap();
        fs::remove_file(&path).unwrap();
    }
    replay(&f.domain, &f.verifier()).unwrap();
}

/// A repository whose domain the product does not track.
fn untracked(name: &str) -> Fixture {
    let f = Fixture::new(name, true);
    git(&f.root, &["rm", "-q", "-r", "--cached", ".brewstillery"]);
    git(&f.root, &["commit", "-q", "-m", "untrack the domain"]);
    f
}

#[test]
fn refusal_domain_not_ignored() {
    isolated("refusal_domain_not_ignored", || {
        let f = untracked("dom-7-ignored");
        let exclude = f.root.join(".git/info/exclude");
        // Git's own template may have written the file; it holds no rule for the domain.
        let template = fs::read(&exclude).unwrap_or_default();
        for mode in [Publication::Unpublished, Publication::OwnRepository] {
            let e = domain::check_ignored(&f.domain, mode).unwrap_err();
            assert_eq!(e.code(), Some(Code::DomainNotIgnored), "{mode:?}");
        }
        domain::check_ignored(&f.domain, Publication::Published).unwrap();

        // The exclude line, added once: then the domain is ignored in both modes.
        assert!(domain::exclude_domain(&f.domain).unwrap());
        assert!(!domain::exclude_domain(&f.domain).unwrap());
        let text = fs::read(&exclude).unwrap();
        assert_eq!(text, [template.as_slice(), b"/.brewstillery/\n"].concat());
        domain::check_ignored(&f.domain, Publication::Unpublished).unwrap();
        git(&f.domain, &["init", "-q", "-b", "main"]);
        domain::check_ignored(&f.domain, Publication::OwnRepository).unwrap();
        fs::remove_dir_all(f.domain.join(".git")).unwrap();

        // No exclude file yet: it is made. One that cannot be read is an error, and nothing is left beside it.
        fs::remove_file(&exclude).unwrap();
        assert!(domain::exclude_domain(&f.domain).unwrap());
        assert_eq!(fs::read_to_string(&exclude).unwrap(), "/.brewstillery/\n");
        fs::remove_file(&exclude).unwrap();
        fs::create_dir(&exclude).unwrap();
        assert!(domain::exclude_domain(&f.domain).unwrap_err().code().is_none());
        assert_eq!(fs::read_dir(exclude.parent().unwrap()).unwrap().count(), 1, "no temporary left");
        fs::remove_dir(&exclude).unwrap();

        // The line removed: refused again. A file without a final newline keeps its last line.
        fs::write(&exclude, "*.log").unwrap();
        assert_eq!(
            domain::check_ignored(&f.domain, Publication::Unpublished).unwrap_err().code(),
            Some(Code::DomainNotIgnored)
        );
        assert!(domain::exclude_domain(&f.domain).unwrap());
        assert_eq!(fs::read_to_string(&exclude).unwrap(), "*.log\n/.brewstillery/\n");
        domain::check_ignored(&f.domain, Publication::Unpublished).unwrap();

        // A domain the product tracks would be published by its next commit, whatever the exclude file says.
        let tracked = Fixture::new("dom-7-tracked", true);
        domain::exclude_domain(&tracked.domain).unwrap();
        let e = domain::check_ignored(&tracked.domain, Publication::Unpublished).unwrap_err();
        assert_eq!(e.code(), Some(Code::DomainNotIgnored));

        // A domain that is not at the root of a repository's main working tree has no product to ask: this fixture
        // lies inside the repository that builds it, which ignores it, and that is no pass.
        let plain = Fixture::new("dom-7-plain", false);
        let e = domain::check_ignored(&plain.domain, Publication::Unpublished).unwrap_err();
        assert_eq!(e.code(), Some(Code::NotARepository));
        let sub = f.root.join("sub");
        fs::create_dir_all(sub.join(".brewstillery")).unwrap();
        fs::write(&exclude, "/sub/.brewstillery/\n").unwrap();
        let e = domain::check_ignored(&sub.join(".brewstillery"), Publication::Unpublished).unwrap_err();
        assert_eq!(e.code(), Some(Code::NotARepository));
    });
}

/// Every file under `dir`, with its bytes.
fn snapshot(dir: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut out = BTreeMap::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in fs::read_dir(&d).into_iter().flatten() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path.clone());
            }
            out.insert(path.clone(), fs::read(&path).unwrap_or_default());
        }
    }
    out
}

#[test]
fn req_dom_5_write_commands_rebuild_the_cache_and_reading_ones_leave_it_absent() {
    isolated("req_dom_5_write_commands_rebuild_the_cache_and_reading_ones_leave_it_absent", || {
        let f = Fixture::new("dom-5", true);
        let key = fs::read_to_string(f.keys.join("xel.pub")).unwrap();
        let text =
            format!("[project]\nname = \"x\"\n\n[maintainers.xel]\nrole = \"maestro\"\nkey = \"{}\"\n", key.trim());
        let policy = settings::parse(&f.domain.join("settings.toml"), &text, Level::Project).unwrap();
        let p = settings::Policy { digest: fabrewk_core::record::sha256_hex(text.as_bytes()), text, settings: policy };
        let eff = settings::effective(&p, None, None, &[], &[]).unwrap();
        let path = std::env::var_os("PATH");
        let cache = f.domain.join("local/cache");
        let names = |dir: &Path| {
            let mut n: Vec<String> =
                fs::read_dir(dir).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).collect();
            n.sort();
            n
        };
        // A write command: the record, the derived signers and the detection, each rebuilt from what it derives from.
        let write = |words: &str| {
            let mut s = begin(&f, f.repo());
            let head = s.head("project").unwrap();
            s.append(
                "project",
                head.as_ref(),
                &person("publication-decided", words, project_payload("publication-decided")),
                Some(&f.signer()),
            )
            .unwrap();
            s.finish(&f.verifier(), &[]).unwrap();
            identity::write_allowed_signers(&f.domain, &p.settings).unwrap();
            let mut d = detect::detect(&f.domain, &eff, &[], path.as_deref());
            d.used(detect::Space::Program, "git");
            d.keep(&f.domain).unwrap();
        };
        // A reading command: everything it looks at, nothing written.
        let read = || {
            let r = replay(&f.domain, &f.verifier()).unwrap();
            check_mark(&f.domain, &r).unwrap();
            detect::detect(&f.domain, &eff, &[], path.as_deref());
            domain::layout(&f.domain, Some(Publication::Published)).unwrap();
            domain::check_gitignore(&f.domain).unwrap();
            domain::check_ignored(&f.domain, Publication::Published).unwrap();
            fabrewk_core::record::walk_history(&Repo::for_domain(&f.domain, Publication::Published).unwrap(), "main")
                .unwrap();
        };
        write("Primero.");
        assert_eq!(names(&cache), ["allowed_signers", "detected.json", "heads.json"]);
        assert_eq!(findings(&f.domain), [], "the flow leaves only the listed names");

        fs::remove_dir_all(&cache).unwrap();
        let before = snapshot(&f.domain);
        read();
        assert!(!cache.exists(), "a reading command leaves the cache absent");
        assert_eq!(snapshot(&f.domain), before, "and the domain byte-identical");

        write("Segundo.");
        assert_eq!(names(&cache), ["allowed_signers", "detected.json", "heads.json"]);
        assert_eq!(fs::read_to_string(cache.join("allowed_signers")).unwrap(), identity::allowed_signers(&p.settings));
        let kept = fabrewk_core::record::decode(&fs::read(cache.join("detected.json")).unwrap()).unwrap();
        assert!(kept["programs"]["git"]["verified"].is_string(), "{kept}");
        assert_eq!(findings(&f.domain), []);
    });
}
