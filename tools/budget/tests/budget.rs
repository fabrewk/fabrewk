//! The workspace against its budget (REQ-BUD-2 to REQ-BUD-5; core-specification 11.6). These tests measure the real
//! workspace: a ring, a crate count or the binary over its limit fails them.

use budget::budget::{
    APPROVED_RUNTIME_CRATES, BINARY_PACKAGE, MAX_BINARY_BYTES, MAX_DIRECT_RUNTIME_CRATES,
    MAX_RELEASE_CRATES, MSRV, RELEASE_TARGET, RINGS, RingKind,
};
use budget::deps::{Report, TreeLine, analyse, cargo_tree, parse_tree};
use budget::rings::{RingCount, count_rings};
use budget::sources::{parse_dep_info, unaccounted};
use budget::{
    binary_problem, cargo, dependency_problems, own_packages, ring_problems, source_problems,
    workspace_root,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn read(path: &str) -> String {
    fs::read_to_string(workspace_root().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// The members' manifests, from the root manifest's `members` line.
fn member_manifests() -> Vec<String> {
    let root = read("Cargo.toml");
    let line = root
        .lines()
        .find(|l| l.starts_with("members = ["))
        .expect("a single-line members list");
    line.split('"')
        .skip(1)
        .step_by(2)
        .map(|m| format!("{m}/Cargo.toml"))
        .collect()
}

#[test]
fn req_bud_1_counter_reports_every_ring() {
    let counts = count_rings(&workspace_root(), RINGS).unwrap();
    assert_eq!(
        counts.iter().map(|c| c.name).collect::<Vec<_>>(),
        RINGS.iter().map(|r| r.name).collect::<Vec<_>>()
    );
    assert!(
        counts.iter().all(|c| c.files > 0 && c.lines > 0),
        "{counts:?}"
    );
}

#[test]
fn req_bud_2_rings_within_budget() {
    let counts = count_rings(&workspace_root(), RINGS).unwrap();
    assert_eq!(ring_problems(&counts), Vec::<String>::new());
}

#[test]
fn req_bud_2_a_ring_over_its_limit_is_reported() {
    let at = RingCount {
        name: "core",
        kind: RingKind::Core,
        files: 1,
        lines: RingKind::Core.limit(),
        sources: Vec::new(),
        data: Vec::new(),
    };
    let over = RingCount {
        lines: at.lines + 1,
        ..at.clone()
    };
    assert!(ring_problems(std::slice::from_ref(&at)).is_empty());
    assert_eq!(ring_problems(&[at, over]).len(), 1);
    assert_eq!(
        [
            RingKind::Core,
            RingKind::ToolAdapter,
            RingKind::Confinement,
            RingKind::Installer
        ]
        .map(RingKind::limit),
        [4_000, 600, 400, 1_200]
    );
}

#[test]
fn req_bud_2_crate_counts_within_budget() {
    let root = workspace_root();
    let tree = cargo_tree(&cargo(), &root, BINARY_PACKAGE, RELEASE_TARGET).unwrap();
    let report = analyse(&parse_tree(&tree).unwrap(), &own_packages(&root, RINGS)).unwrap();
    assert!(
        report.local.contains(&root.join("crates/fabrewk")),
        "{report:?}"
    );
    assert_eq!(dependency_problems(&report), Vec::<String>::new());
}

fn line(depth: usize, name: &str, path: Option<&str>) -> TreeLine {
    TreeLine {
        depth,
        name: name.into(),
        version: "1.0.0".into(),
        path: path.map(PathBuf::from),
        proc_macro: false,
    }
}

#[test]
fn req_bud_2_tree_is_parsed_and_classified() {
    let text = "0fabrewk v0.0.0 (/w/crates/fabrewk)\n1fabrewk-core v0.0.0 (/w/crates/core)\n2serde v1.0.229\n\
                3serde_derive v1.0.229 (proc-macro)\n4syn v3.0.9\n2cc v1.11.0\n1sha2 v0.10.9\n2cfg-if v1.0.5\n\
                1other v0.1.0 (/elsewhere/other)\n2leaf v2.0.0\n1vendored v0.1.0 (/w/vendor/vendored)\n\
                2tools v0.0.0 (/w/tools/budget)\n";
    let lines = parse_tree(text).unwrap();
    assert_eq!(
        lines[0],
        TreeLine {
            depth: 0,
            name: "fabrewk".into(),
            version: "0.0.0".into(),
            path: Some("/w/crates/fabrewk".into()),
            proc_macro: false,
        }
    );
    assert_eq!((lines[3].path.clone(), lines[3].proc_macro), (None, true));
    // Fabrewk's own packages are the ring directories exactly; a package elsewhere under the repository is a crate.
    let own = [
        PathBuf::from("/w/crates/fabrewk"),
        PathBuf::from("/w/crates/core"),
    ];
    let report = analyse(&lines, &own).unwrap();
    let direct: Vec<_> = report.direct.iter().map(String::as_str).collect();
    assert_eq!(direct, ["cc", "other", "serde", "sha2", "vendored"]);
    assert_eq!(report.all.len(), 10);
    assert_eq!(report.local, own.clone().into_iter().collect());
    assert!(report.own_proc_macros.is_empty());
    let with_macro = parse_tree(
        "0fabrewk v0.0.0 (/w/crates/fabrewk)\n1fabrewk-core v0.0.0 (/w/crates/core) (proc-macro)\n",
    )
    .unwrap();
    let report = analyse(&with_macro, &own).unwrap();
    assert_eq!(
        report.own_proc_macros,
        [PathBuf::from("/w/crates/core")].into_iter().collect()
    );
    assert_eq!(dependency_problems(&report).len(), 1);
    assert!(parse_tree("fabrewk v0.0.0\n").is_err());
    assert!(parse_tree("0fabrewk\n").is_err());
    assert!(parse_tree("0fabrewk 0.0.0\n").is_err());
    assert!(analyse(&[line(0, "a", None), line(2, "b", None)], &[]).is_err());
}

#[test]
fn req_bud_2_crate_counts_over_limits_are_reported() {
    let mut report = Report::default();
    report.local.insert(PathBuf::from("/w/crates/core"));
    for (name, _) in APPROVED_RUNTIME_CRATES {
        report.direct.insert(name.to_string());
    }
    for n in 0..MAX_RELEASE_CRATES {
        report.all.insert((format!("c{n}"), "1.0.0".into()));
    }
    assert_eq!(dependency_problems(&report), Vec::<String>::new());

    let mut more = report.clone();
    more.all.insert(("one-more".into(), "1.0.0".into()));
    assert_eq!(dependency_problems(&more).len(), 1);

    let mut unapproved = report.clone();
    unapproved.direct.insert("unapproved".into());
    assert_eq!(dependency_problems(&unapproved).len(), 1);

    let mut at_limit = report.clone();
    at_limit.direct = (0..MAX_DIRECT_RUNTIME_CRATES)
        .map(|n| format!("d{n}"))
        .collect();
    assert_eq!(dependency_problems(&at_limit).len(), at_limit.direct.len());

    let mut too_many = report.clone();
    too_many.direct = (0..=MAX_DIRECT_RUNTIME_CRATES)
        .map(|n| format!("d{n}"))
        .collect();
    assert_eq!(
        dependency_problems(&too_many).len(),
        1 + too_many.direct.len()
    );
}

#[test]
fn req_bud_2_binary_within_budget() {
    // The frozen limit is the musl release binary's (D-042), built here by the cargo running the suite, which must
    // have that target (D-043); this test fails, never skips, when it cannot.
    let target_dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("release-size");
    let status = Command::new(cargo())
        .current_dir(workspace_root())
        .args([
            "build",
            "--release",
            "--locked",
            "--offline",
            "--package",
            BINARY_PACKAGE,
            "--target",
            RELEASE_TARGET,
            "--target-dir",
        ])
        .arg(&target_dir)
        .status()
        .unwrap();
    assert!(
        status.success(),
        "{} cannot build for {RELEASE_TARGET}: run the suite with the rustup toolchain that has it",
        cargo().display()
    );
    let binary = target_dir
        .join(RELEASE_TARGET)
        .join("release")
        .join(BINARY_PACKAGE);
    let dep_info = fs::read_to_string(binary.with_extension("d")).unwrap();
    let counts = count_rings(&workspace_root(), RINGS).unwrap();
    assert_eq!(
        source_problems(&dep_info, &counts).unwrap(),
        Vec::<String>::new()
    );
    let bytes = fs::metadata(&binary).unwrap().len();
    assert_eq!(binary_problem(bytes), None, "{bytes} bytes");
    assert_eq!(binary_problem(MAX_BINARY_BYTES), None);
    assert!(binary_problem(MAX_BINARY_BYTES + 1).is_some());
}

#[test]
fn req_bud_3_msrv_and_edition() {
    let root = read("Cargo.toml");
    assert!(
        root.lines()
            .any(|l| l == format!("rust-version = \"{MSRV}\"")),
        "rust-version is not {MSRV}"
    );
    assert!(root.lines().any(|l| l == "edition = \"2024\""));
    for manifest in member_manifests() {
        let text = read(&manifest);
        for key in ["rust-version", "edition"] {
            assert!(
                text.lines().any(|l| l == format!("{key}.workspace = true")),
                "{manifest}: {key}"
            );
        }
    }
}

#[test]
fn req_bud_4_direct_crates_are_approved() {
    let root = workspace_root();
    let report = analyse(
        &parse_tree(&cargo_tree(&cargo(), &root, BINARY_PACKAGE, RELEASE_TARGET).unwrap()).unwrap(),
        &own_packages(&root, RINGS),
    )
    .unwrap();
    for name in &report.direct {
        assert!(
            APPROVED_RUNTIME_CRATES.iter().any(|(a, _)| a == name),
            "{name} has no approved reason"
        );
    }
    assert!(read("Cargo.lock").starts_with("# This file is automatically @generated by Cargo."));
}

#[test]
fn req_bud_5_licence_declared_and_files_present() {
    let root = read("Cargo.toml");
    assert!(root.lines().any(|l| l == "license = \"MIT OR Apache-2.0\""));
    for manifest in member_manifests() {
        assert!(
            read(&manifest)
                .lines()
                .any(|l| l == "license.workspace = true"),
            "{manifest}"
        );
    }
    assert!(
        read("LICENSE-APACHE")
            .trim_start()
            .starts_with("Apache License\n                        Version 2.0, January 2004")
    );
    assert!(read("LICENSE-MIT").contains("Permission is hereby granted, free of charge"));
    assert!(read("NOTICE").starts_with("Fabrewk\n"));
}

#[test]
fn req_bud_1_dep_info_is_parsed() {
    let deps = parse_dep_info("/t/fabrewk: /w/a.rs /w/my\\ dir/b.rs \\\n /w/c.txt\n\n").unwrap();
    assert_eq!(
        deps,
        [
            PathBuf::from("/w/a.rs"),
            PathBuf::from("/w/my dir/b.rs"),
            PathBuf::from("/w/c.txt")
        ]
    );
    assert_eq!(parse_dep_info("/t/x:\n").unwrap(), Vec::<PathBuf>::new());
    // A colon after the target belongs to a path.
    let deps = parse_dep_info("/t/x: /w/a:b.rs /w/c: /w/d.rs\n").unwrap();
    assert_eq!(
        deps,
        [
            PathBuf::from("/w/a:b.rs"),
            PathBuf::from("/w/c:"),
            PathBuf::from("/w/d.rs")
        ]
    );
    assert!(parse_dep_info("").is_err());
    assert!(parse_dep_info("/w/a.rs /w/b.rs\n").is_err());
}

#[test]
fn req_bud_1_unaccounted_sources_are_named() {
    let count = RingCount {
        name: "core",
        kind: RingKind::Core,
        files: 1,
        lines: 1,
        sources: vec![PathBuf::from("/w/crates/core/src/lib.rs")],
        data: vec![PathBuf::from("/w/crates/core/src/en.toml")],
    };
    let deps = [
        "/w/crates/core/src/lib.rs",
        "/w/crates/core/src/en.toml",
        "/w/tools/x.rs",
        "/w/crates/core/src/hidden.inc",
    ]
    .map(PathBuf::from);
    assert_eq!(
        unaccounted(&deps, std::slice::from_ref(&count)),
        [
            PathBuf::from("/w/tools/x.rs"),
            PathBuf::from("/w/crates/core/src/hidden.inc")
        ]
    );
    let problems =
        source_problems("/t/f: /w/crates/core/src/lib.rs /w/tools/x.rs\n", &[count]).unwrap();
    assert_eq!(
        problems,
        ["/w/tools/x.rs is compiled into the binary but not counted"]
    );
}

/// Builds a small workspace whose manifests point Cargo at files outside the ring, as Codex did in its second review
/// of part 7a, and checks that the dep-info names every one while the walk alone would not see them.
#[test]
fn req_bud_1_compiled_sources_outside_the_count_are_reported() {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("escapes");
    let _ = fs::remove_dir_all(&root);
    let files = [
        (
            "Cargo.toml",
            "[workspace]\nresolver = \"3\"\nmembers = [\"crates/lib\", \"crates/bin\"]\n",
        ),
        // A quoted key and a dotted key, both of which a reading of the text missed.
        (
            "crates/lib/Cargo.toml",
            "[package]\nname = \"lib\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\"build\" = \"../../tools/b.rs\"\n\n[lib]\n'path' = \"../../tools/lib.inc\"\n",
        ),
        ("crates/lib/src/lib.rs", "pub fn never_built() {}\n"),
        (
            "crates/bin/Cargo.toml",
            "[package]\nname = \"bin\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[dependencies]\nlib = { path = \"../lib\" }\n",
        ),
        (
            "crates/bin/src/main.rs",
            "const T: &str = include_str!(\"data.txt\");\nfn main() { println!(\"{} {}\", lib::c(), T.len()); }\n",
        ),
        ("crates/bin/src/data.txt", "data\n"),
        ("tools/lib.inc", "pub fn c() -> u32 { 3 }\n"),
        ("tools/b.rs", "fn main() {}\n"),
    ];
    for (path, content) in files {
        fs::create_dir_all(root.join(path).parent().unwrap()).unwrap();
        fs::write(root.join(path), content).unwrap();
    }
    let status = Command::new(cargo())
        .current_dir(&root)
        .args([
            "build",
            "--release",
            "--offline",
            "--quiet",
            "--package",
            "bin",
        ])
        .status()
        .unwrap();
    assert!(status.success());
    let rings = [budget::budget::Ring {
        name: "core",
        kind: RingKind::Core,
        dirs: &["crates/lib", "crates/bin"],
    }];
    let counts = count_rings(&root, &rings).unwrap();
    assert_eq!(
        counts[0].files, 2,
        "the walk sees only the two files of the rings"
    );
    let dep_info = fs::read_to_string(root.join("target/release/bin.d")).unwrap();
    let mut problems = source_problems(&dep_info, &counts).unwrap();
    problems.sort();
    let tools = root.canonicalize().unwrap().join("tools");
    assert_eq!(
        problems,
        [
            format!(
                "{} is compiled into the binary but not counted",
                tools.join("b.rs").display()
            ),
            format!(
                "{} is compiled into the binary but not counted",
                tools.join("lib.inc").display()
            ),
        ]
    );
}
