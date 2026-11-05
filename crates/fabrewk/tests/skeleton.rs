//! The skeleton's only behaviour: `--version`, and a usage error for anything else.

use std::process::Command;

fn fabrewk(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_fabrewk")).args(args).output().expect("the built fabrewk binary runs")
}

#[test]
fn skeleton_prints_its_version() {
    let out = fabrewk(&["--version"]);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(String::from_utf8(out.stdout).unwrap(), format!("fabrewk {}\n", env!("CARGO_PKG_VERSION")));
    assert!(out.stderr.is_empty());
}

#[test]
fn skeleton_refuses_anything_else_as_usage() {
    for args in [&[][..], &["--help"], &["--version", "--json"], &["version"]] {
        let out = fabrewk(args);
        assert_eq!(out.status.code(), Some(2), "args {args:?}");
        assert!(out.stdout.is_empty(), "args {args:?}");
    }
}
