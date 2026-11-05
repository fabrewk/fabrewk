//! The workspace against its budget (REQ-BUD-2 to REQ-BUD-5; core-specification 11.6). These tests measure the real
//! workspace: a ring, a crate count or the binary over its limit fails them.

use budget::budget::{
    APPROVED_RUNTIME_CRATES, BINARY_PACKAGE, MAX_BINARY_BYTES, MAX_DIRECT_RUNTIME_CRATES,
    MAX_RELEASE_CRATES, MSRV, RELEASE_TARGET, RINGS, RingKind,
};
use budget::deps::{Report, analyse, cargo_metadata, config_problems};
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

/// The real workspace's release build, from `cargo metadata`.
fn real_report() -> Report {
    let root = workspace_root();
    let counts = count_rings(&root, RINGS).unwrap();
    budget::release_report(&root, &counts).unwrap()
}

#[test]
fn req_bud_2_crate_counts_within_budget() {
    let report = real_report();
    let root = workspace_root();
    assert_eq!(
        report.local,
        [root.join("crates/core"), root.join("crates/fabrewk")]
            .into_iter()
            .collect()
    );
    assert_eq!(dependency_problems(&report), Vec::<String>::new());
}

/// A metadata document shaped like Cargo's, with every case the analysis tells apart.
const METADATA: &str = r#"{"packages":[
 {"id":"bin","name":"fabrewk","version":"0.0.0","source":null,"manifest_path":"/w/crates/fabrewk/Cargo.toml",
  "targets":[{"kind":["bin"],"src_path":"/w/crates/fabrewk/src/main.rs"},{"kind":["test"],"src_path":"/w/crates/fabrewk/tests/t.rs"}]},
 {"id":"core","name":"fabrewk-core","version":"0.0.0","source":null,"manifest_path":"/w/crates/core/Cargo.toml",
  "targets":[{"kind":["lib"],"src_path":"/w/crates/core/src/lib.rs"},{"kind":["custom-build"],"src_path":"/w/crates/core/build.rs"},
             {"kind":["proc-macro"],"src_path":"/w/crates/core/src/m.rs"},{"kind":["bench"],"src_path":"/w/crates/core/benches/b.rs"}]},
 {"id":"serde","name":"serde","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index","manifest_path":"/r/serde/Cargo.toml","targets":[]},
 {"id":"derive","name":"serde_derive","version":"1.0.0","source":"registry+r","manifest_path":"/r/derive/Cargo.toml","targets":[]},
 {"id":"cc","name":"cc","version":"1.1.0","source":"registry+r","manifest_path":"/r/cc/Cargo.toml","targets":[]},
 {"id":"proptest","name":"proptest","version":"1.0.0","source":"registry+r","manifest_path":"/r/proptest/Cargo.toml","targets":[]},
 {"id":"vendored","name":"serde","version":"0.0.0","source":null,"manifest_path":"/w/tools/serde (x)/Cargo.toml","targets":[]},
 {"id":"budget","name":"budget","version":"0.0.0","source":null,"manifest_path":"/w/tools/budget/Cargo.toml","targets":[]},
 {"id":"stray","name":"stray","version":"0.0.0","source":null,"manifest_path":"/w/stray/Cargo.toml","targets":[]}],
 "workspace_members":["bin","core","budget"],
 "resolve":{"nodes":[
  {"id":"bin","deps":[{"pkg":"core","dep_kinds":[{"kind":null,"target":null}]},{"pkg":"proptest","dep_kinds":[{"kind":"dev"}]},
                      {"pkg":"vendored","dep_kinds":[{"kind":null}]}]},
  {"id":"core","deps":[{"pkg":"serde","dep_kinds":[{"kind":null}]},{"pkg":"cc","dep_kinds":[{"kind":"build"}]}]},
  {"id":"serde","deps":[{"pkg":"derive","dep_kinds":[{"kind":null}]}]},
  {"id":"derive","deps":[]},{"id":"cc","deps":[]},{"id":"proptest","deps":[]},{"id":"vendored","deps":[]},
  {"id":"budget","deps":[{"pkg":"core","dep_kinds":[{"kind":null}]}]},{"id":"stray","deps":[]}]}}"#;

fn ring_count(sources: &[&str]) -> RingCount {
    RingCount {
        name: "core",
        kind: RingKind::Core,
        files: sources.len(),
        lines: 1,
        sources: sources.iter().map(PathBuf::from).collect(),
        data: vec![],
    }
}

#[test]
fn req_bud_2_metadata_is_classified() {
    let own = [
        PathBuf::from("/w/crates/fabrewk"),
        PathBuf::from("/w/crates/core"),
    ];
    let count = ring_count(&["/w/crates/fabrewk/src/main.rs"]);
    let report = analyse(METADATA, "fabrewk", &own, std::slice::from_ref(&count)).unwrap();
    let direct: Vec<_> = report
        .direct
        .iter()
        .map(|(id, name)| format!("{id}={name}"))
        .collect();
    assert_eq!(direct, ["cc=cc", "serde=serde"]);
    let all: Vec<_> = report.all.values().map(String::as_str).collect();
    assert_eq!(all, ["cc 1.1.0", "serde_derive 1.0.0", "serde 1.0.0"]);
    assert_eq!(report.local, own.iter().cloned().collect());
    assert_eq!(
        report.problems,
        [
            "/w/stray is a local package outside the workspace",
            "/w/tools/serde (x) is a local package outside the workspace",
            "/w/tools/serde (x) is a local package the binary reaches outside the rings",
            "/w/crates/core/src/lib.rs is the root of a lib target but not counted",
            "/w/crates/core/build.rs is a build script in a ring",
            "/w/crates/core/src/m.rs is a procedural macro in a ring",
        ]
    );
    // The problems of the closed world are budget problems too, beside `cc`, which has no approved reason.
    let problems = dependency_problems(&report);
    assert_eq!(problems.len(), 7);
    assert_eq!(
        problems[0],
        "direct runtime crate cc has no approved reason"
    );
    let counted = ring_count(&["/w/crates/fabrewk/src/main.rs", "/w/crates/core/src/lib.rs"]);
    let report = analyse(METADATA, "fabrewk", &own, &[counted]).unwrap();
    assert_eq!(report.problems.len(), 5);
    assert!(analyse(METADATA, "budget-missing", &own, &[]).is_err());
    assert!(
        analyse(
            &METADATA.replace("\"resolve\"", "\"other\""),
            "fabrewk",
            &own,
            &[]
        )
        .is_err()
    );
    assert!(
        analyse(
            &METADATA.replace("{\"pkg\":\"cc\"", "{\"pkg\":\"gone\""),
            "fabrewk",
            &own,
            &[]
        )
        .is_err()
    );
    assert!(
        analyse(
            &METADATA.replace("\"manifest_path\":\"/w/stray/Cargo.toml\",", ""),
            "fabrewk",
            &own,
            &[]
        )
        .is_err()
    );
}

#[test]
fn req_bud_1_json_is_read() {
    let doc = budget::json::parse(r#"{"esc":"a\"\\\/\b\f\n\r\t\ud83d\ude00\u00e9","n":-1.5e3,"t":true,"f":false,"z":null,"o":{}}"#).unwrap();
    assert_eq!(
        doc.get("esc").and_then(|v| v.as_str()),
        Some("a\"\\/\u{8}\u{c}\n\r\t😀é")
    );
    assert_eq!(
        doc.get("n"),
        Some(&budget::json::Value::Number("-1.5e3".into()))
    );
    assert_eq!(doc.get("t"), Some(&budget::json::Value::Bool(true)));
    assert_eq!(doc.get("z"), Some(&budget::json::Value::Null));
    let literal = budget::json::parse("[\"é€😀a\\u00e9\"]").unwrap();
    assert_eq!(literal.as_array()[0].as_str(), Some("é€😀aé"));
    for bad in [
        "",
        "{",
        "[1,]",
        "{\"a\" 1}",
        "\"\\x\"",
        "\"\\u12\"",
        "tru",
        "{} x",
        "[1 2]",
        "\"open",
        "{1:2}",
    ] {
        assert!(budget::json::parse(bad).is_err(), "{bad:?}");
    }
}

#[test]
fn req_bud_2_crate_counts_over_limits_are_reported() {
    let mut report = Report::default();
    report.local.insert(PathBuf::from("/w/crates/core"));
    for (name, _) in APPROVED_RUNTIME_CRATES {
        report.direct.insert(format!("id-{name}"), name.to_string());
    }
    for n in 0..MAX_RELEASE_CRATES {
        report.all.insert(format!("c{n}"), format!("c{n} 1.0.0"));
    }
    assert_eq!(dependency_problems(&report), Vec::<String>::new());

    let mut more = report.clone();
    more.all.insert("one-more".into(), "one-more 1.0.0".into());
    assert_eq!(dependency_problems(&more).len(), 1);

    let mut unapproved = report.clone();
    unapproved
        .direct
        .insert("id-unapproved".into(), "unapproved".into());
    assert_eq!(dependency_problems(&unapproved).len(), 1);
    // Two packages with one name count twice and are reported once.
    unapproved
        .direct
        .insert("id-unapproved-2".into(), "unapproved".into());
    assert_eq!(dependency_problems(&unapproved).len(), 1);

    let mut at_limit = report.clone();
    at_limit.direct = (0..MAX_DIRECT_RUNTIME_CRATES)
        .map(|n| (format!("i{n}"), format!("d{n}")))
        .collect();
    assert_eq!(dependency_problems(&at_limit).len(), at_limit.direct.len());

    let mut too_many = report.clone();
    too_many.direct = (0..=MAX_DIRECT_RUNTIME_CRATES)
        .map(|n| (format!("i{n}"), format!("d{n}")))
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
    assert_eq!(real_report().problems, Vec::<String>::new());
    assert_eq!(
        config_problems(&workspace_root(), std::env::vars()),
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
    for name in real_report().direct.values() {
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

/// Builds a small workspace with the escapes Codex found in its second to fourth reviews of part 7a: a quoted
/// `"build"` key and a `'path'` key pointing at `tools/`; a library whose root is a file also included as data; a
/// build script inside a ring that turns on `cfg(test)`; a ring package excluded from the workspace, with its own build
/// script; an optional dependency, enabled only by a feature, from a path whose name holds ` (`; a Cargo configuration
/// inside a package. The walk sees none of them; the dep-info, the metadata and the configuration check report each.
#[test]
fn req_bud_1_compiled_sources_outside_the_count_are_reported() {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("escapes");
    let _ = fs::remove_dir_all(&root);
    let package = |name: &str, extra: &str| {
        format!("[package]\nname = \"{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\n{extra}")
    };
    let workspace = "[workspace]\nresolver = \"3\"\nmembers = [\"crates/lib\", \"crates/data\", \"crates/bin\"]\n\
                     exclude = [\"crates/nonmember\", \"tools/serde (x)\"]\n";
    let bin = "\n[dependencies]\nlib = { path = \"../lib\" }\ndata = { path = \"../data\" }\n\
               nonmember = { path = \"../nonmember\" }\nserde = { path = \"../../tools/serde (x)\", optional = true }\n\n\
               [features]\nhidden = [\"dep:serde\"]\n";
    let files = [
        ("Cargo.toml", workspace.to_string()),
        ("crates/lib/Cargo.toml", package("lib", "\"build\" = \"../../tools/b.rs\"\n\n[lib]\n'path' = \"../../tools/lib.inc\"\n")),
        ("crates/lib/src/lib.rs", "pub fn never_built() {}\n".to_string()),
        ("crates/data/Cargo.toml", package("data", "\n[lib]\npath = \"src/payload.inc\"\n")),
        ("crates/data/src/payload.inc", "pub fn p() -> u32 { 4 }\n".to_string()),
        ("crates/nonmember/Cargo.toml", package("nonmember", "")),
        ("crates/nonmember/build.rs", "fn main() {}\n".to_string()),
        ("crates/nonmember/src/lib.rs", "pub fn n() -> u32 { 5 }\n".to_string()),
        ("crates/bin/Cargo.toml", package("bin", bin)),
        ("crates/bin/build.rs", "fn main() { println!(\"cargo::rustc-cfg=test\"); }\n".to_string()),
        ("crates/bin/.cargo/config.toml", "[build]\nrustflags = [\"--cfg\", \"test\"]\n".to_string()),
        (
            "crates/bin/src/main.rs",
            "const T: &str = include_str!(\"data.txt\");\nconst P: &str = include_str!(\"../../data/src/payload.inc\");\n\
             fn main() { println!(\"{} {} {} {} {}\", lib::c(), data::p(), nonmember::n(), T.len(), P.len()); }\n"
                .to_string(),
        ),
        ("crates/bin/src/data.txt", "data\n".to_string()),
        ("tools/lib.inc", "pub fn c() -> u32 { 3 }\n".to_string()),
        ("tools/b.rs", "fn main() {}\n".to_string()),
        ("tools/serde (x)/Cargo.toml", package("serde", "")),
        ("tools/serde (x)/src/lib.rs", "pub fn s() {}\n".to_string()),
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
        dirs: &[
            "crates/lib",
            "crates/data",
            "crates/bin",
            "crates/nonmember",
        ],
    }];
    let counts = count_rings(&root, &rings).unwrap();
    assert_eq!(
        counts[0].files, 5,
        "lib.rs, both build.rs, main.rs and nonmember's lib.rs"
    );
    let dep_info = fs::read_to_string(root.join("target/release/bin.d")).unwrap();
    let mut problems = source_problems(&dep_info, &counts).unwrap();
    let metadata = cargo_metadata(&cargo(), &root).unwrap();
    problems.extend(
        analyse(&metadata, "bin", &own_packages(&root, &rings), &counts)
            .unwrap()
            .problems,
    );
    problems.extend(config_problems(&root, Vec::new()));
    problems.sort();
    let real = root.canonicalize().unwrap();
    let at = |p: &str| real.join(p).display().to_string();
    let mut expected = [
        format!(
            "{} is compiled into the binary but not counted",
            at("tools/b.rs")
        ),
        format!(
            "{} is compiled into the binary but not counted",
            at("tools/lib.inc")
        ),
        format!("{} is a build script in a ring", at("tools/b.rs")),
        format!(
            "{} is the root of a lib target but not counted",
            at("tools/lib.inc")
        ),
        format!(
            "{} is the root of a lib target but not counted",
            at("crates/data/src/payload.inc")
        ),
        format!("{} is a build script in a ring", at("crates/bin/build.rs")),
        format!(
            "{} is a build script in a ring",
            at("crates/nonmember/build.rs")
        ),
        format!(
            "{} is a local package outside the workspace",
            at("crates/nonmember")
        ),
        format!(
            "{} is a local package outside the workspace",
            at("tools/serde (x)")
        ),
        format!(
            "{} is a local package the binary reaches outside the rings",
            at("tools/serde (x)")
        ),
        format!(
            "{} may change how the rings compile; Fabrewk keeps none",
            root.join("crates/bin/.cargo").display()
        ),
    ];
    expected.sort();
    assert_eq!(problems, expected);
}

#[test]
fn req_bud_1_cargo_configuration_and_rustflags_are_reported() {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("config");
    let _ = fs::remove_dir_all(&root);
    for dir in [".git/.cargo", "target/.cargo", "src"] {
        fs::create_dir_all(root.join(dir)).unwrap();
    }
    let env = |pairs: &[(&str, &str)]| {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<Vec<_>>()
    };
    let clean = env(&[("RUSTFLAGS", "-C opt-level=3"), ("PATH", "cfg")]);
    assert_eq!(config_problems(&root, clean.clone()), Vec::<String>::new());
    let flags = env(&[
        ("RUSTFLAGS", "--cfg test"),
        (
            "CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS",
            "--cfg=test",
        ),
    ]);
    assert_eq!(config_problems(&root, flags).len(), 2);
    for dir in [".cargo", "crates/core/.cargo", "a/b/c/.cargo"] {
        fs::create_dir_all(root.join(dir)).unwrap();
    }
    let found = config_problems(&root, clean);
    assert_eq!(found.len(), 3, "{found:?}");
    assert!(found[0].ends_with(".cargo may change how the rings compile; Fabrewk keeps none"));
}

#[test]
fn req_bud_2_packages_are_told_apart_by_id_and_every_platform_counts() {
    // Codex's case in its fifth review of part 7a: packages of one name and version from distinct sources.
    let doc = r#"{"packages":[
     {"id":"bin","name":"fabrewk","version":"0.0.0","source":null,"manifest_path":"/w/crates/fabrewk/Cargo.toml","targets":[]},
     {"id":"git-a","name":"serde","version":"0.0.1","source":"git+a","manifest_path":"/g/a/Cargo.toml","targets":[]},
     {"id":"git-b","name":"serde","version":"0.0.1","source":"git+b","manifest_path":"/g/b/Cargo.toml","targets":[]}],
     "workspace_members":["bin"],
     "resolve":{"nodes":[{"id":"bin","deps":[{"pkg":"git-a","dep_kinds":[{"kind":null}]},{"pkg":"git-b","dep_kinds":[{"kind":null}]}]},
      {"id":"git-a","deps":[]},{"id":"git-b","deps":[]}]}}"#;
    let own = [PathBuf::from("/w/crates/fabrewk")];
    let report = analyse(doc, "fabrewk", &own, &[]).unwrap();
    assert_eq!((report.direct.len(), report.all.len()), (2, 2));
    // Edges under any platform's cfg are followed: Codex's case in its sixth review of part 7a, a macro the target
    // reaches whose dependencies only the host builds.
    let cross = r#"{"packages":[
     {"id":"bin","name":"fabrewk","version":"0.0.0","source":null,"manifest_path":"/w/crates/fabrewk/Cargo.toml","targets":[]},
     {"id":"macro","name":"serde","version":"0.1.0","source":"git+m","manifest_path":"/g/m/Cargo.toml","targets":[]},
     {"id":"helper","name":"helper","version":"0.1.0","source":"git+h","manifest_path":"/g/h/Cargo.toml","targets":[]}],
     "workspace_members":["bin"],
     "resolve":{"nodes":[{"id":"bin","deps":[{"pkg":"macro","dep_kinds":[{"kind":null,"target":"cfg(target_env = \"musl\")"}]}]},
      {"id":"macro","deps":[{"pkg":"helper","dep_kinds":[{"kind":null,"target":"cfg(target_env = \"gnu\")"}]}]},
      {"id":"helper","deps":[]}]}}"#;
    let report = analyse(cross, "fabrewk", &own, &[]).unwrap();
    assert_eq!(report.all.len(), 2);
}

#[test]
fn req_bud_1_compiler_and_cargo_home_in_the_environment_are_reported() {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("environment");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("home")).unwrap();
    let inside = root.join("home").display().to_string();
    let env = [
        ("CARGO_HOME", inside.as_str()),
        ("RUSTC_WRAPPER", "/usr/bin/sccache"),
        ("CARGO_BUILD_RUSTC", "/opt/rustc"),
        ("RUSTC_WORKSPACE_WRAPPER", ""),
        ("CARGO_HOME_X", inside.as_str()),
    ]
    .map(|(k, v)| (k.to_string(), v.to_string()));
    assert_eq!(
        config_problems(&root, env),
        [
            format!("CARGO_HOME is inside the repository, so its configuration applies: {inside}"),
            "RUSTC_WRAPPER replaces or wraps the compiler: /usr/bin/sccache".to_string(),
            "CARGO_BUILD_RUSTC replaces or wraps the compiler: /opt/rustc".to_string(),
        ]
    );
    let outside = [("CARGO_HOME".to_string(), "/nonexistent/cargo".to_string())];
    assert_eq!(config_problems(&root, outside), Vec::<String>::new());
}

#[test]
fn req_bud_1_an_unreadable_directory_is_reported() {
    use std::os::unix::fs::PermissionsExt;
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("unreadable");
    let _ = fs::set_permissions(root.join("opaque"), fs::Permissions::from_mode(0o755));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("opaque/.cargo")).unwrap();
    fs::set_permissions(root.join("opaque"), fs::Permissions::from_mode(0o111)).unwrap();
    let problems = config_problems(&root, Vec::new());
    fs::set_permissions(root.join("opaque"), fs::Permissions::from_mode(0o755)).unwrap();
    assert_eq!(
        problems,
        [format!(
            "{} cannot be read, so the check cannot see what it holds",
            root.join("opaque").display()
        )]
    );
}
