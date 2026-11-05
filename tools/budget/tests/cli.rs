//! The `budget` command: what it prints and its exit status (0 within, 1 over, 2 usage, 70 error).

use budget::budget::{MAX_BINARY_BYTES, RINGS};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn budget(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_budget"))
        .args(args)
        .output()
        .unwrap()
}

fn stdout(out: &Output) -> String {
    String::from_utf8(out.stdout.clone()).unwrap()
}

/// A file of `bytes` bytes standing for the release binary, with a dep-info beside it listing the rings' sources and
/// `extra`.
fn fake_binary(name: &str, bytes: u64, extra: &[&str]) -> PathBuf {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    fs::File::create(&path).unwrap().set_len(bytes).unwrap();
    let root = budget::workspace_root();
    let mut deps: Vec<String> = ["crates/core/src/lib.rs", "crates/fabrewk/src/main.rs"]
        .map(|p| root.join(p).display().to_string())
        .to_vec();
    deps.extend(extra.iter().map(|e| e.to_string()));
    fs::write(
        path.with_extension("d"),
        format!("{}: {}\n", path.display(), deps.join(" ")),
    )
    .unwrap();
    path
}

#[test]
fn req_bud_1_count_prints_every_ring() {
    let out = budget(&["count"]);
    assert_eq!(out.status.code(), Some(0));
    let text = stdout(&out);
    assert_eq!(text.lines().count(), RINGS.len());
    for ring in RINGS {
        let line = text
            .lines()
            .find(|l| l.starts_with(&format!("ring {}: ", ring.name)))
            .unwrap();
        assert!(
            line.contains(&format!(" of {} lines, ", ring.kind.limit())),
            "{line}"
        );
    }
}

#[test]
fn req_bud_2_check_within_and_over_the_binary_limit() {
    let within = fake_binary("within", MAX_BINARY_BYTES, &[]);
    let out = budget(&["check", "--binary", within.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(0), "{}", stdout(&out));
    let text = stdout(&out);
    assert!(text.starts_with("ring core: "), "{text}");
    assert!(text.contains("crates in the release build, every platform counted: "));
    assert!(text.contains(&format!(": {MAX_BINARY_BYTES} bytes\n")));
    assert!(!text.contains("over budget"));

    let over = fake_binary("over", MAX_BINARY_BYTES + 1, &[]);
    let out = budget(&["check", "--binary", over.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(1));
    let text = stdout(&out);
    assert!(
        text.contains("over budget: the release binary has"),
        "{text}"
    );
}

#[test]
fn req_bud_1_check_reports_compiled_sources_not_counted() {
    let binary = fake_binary("uncounted", 1, &["/elsewhere/hidden.inc"]);
    let out = budget(&["check", "--binary", binary.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stdout(&out).contains(
        "over budget: /elsewhere/hidden.inc is compiled into the binary but not counted\n"
    ));

    fs::remove_file(binary.with_extension("d")).unwrap();
    let out = budget(&["check", "--binary", binary.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(70));
    assert!(
        String::from_utf8(out.stderr)
            .unwrap()
            .contains("uncounted.d: ")
    );
}

#[test]
fn req_bud_2_check_without_its_binary_is_an_error() {
    let out = budget(&["check", "--binary", "/nonexistent/fabrewk"]);
    assert_eq!(out.status.code(), Some(70));
    assert!(
        String::from_utf8(out.stderr)
            .unwrap()
            .starts_with("budget: /nonexistent/fabrewk: ")
    );
}

#[test]
fn budget_usage_errors() {
    for args in [
        &[][..],
        &["check"],
        &["check", "--binary"],
        &["count", "extra"],
        &["check", "--target", "x"],
        &["check", "--binary", "b", "--target", "x"],
    ] {
        let out = budget(args);
        assert_eq!(out.status.code(), Some(2), "{args:?}");
        assert!(out.stdout.is_empty());
        assert!(
            String::from_utf8(out.stderr)
                .unwrap()
                .starts_with("usage: budget count")
        );
    }
}
