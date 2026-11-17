//! The installer through its contract, without the binary: `assess` outside a repository refuses before it reads
//! anything (REQ-INS-1), and its own refusal is the core's code.

use fabrewk_core::cli::{Options, Parsed, parse};
use fabrewk_core::install::Asked;

fn options(args: &[&str]) -> Options {
    match parse(args.iter().map(std::ffi::OsString::from)) {
        Ok(Parsed::Run { options, .. }) => options,
        other => panic!("{other:?}"),
    }
}

#[test]
fn assess_outside_a_repository_is_not_a_repository() {
    let dir =
        std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("install-contract-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let options = options(&["assess"]);
    let asked = Asked { root: &dir, home: None, options: &options, adapters: &[] };
    let refused = (fabrewk_install::INSTALLER.assess)(&asked).unwrap_err();
    assert_eq!(refused.len(), 1);
    assert_eq!(refused[0].code(), Some(fabrewk_core::Code::NotARepository));
    let alias = options_alias();
    let asked = Asked { root: &dir, home: None, options: &alias, adapters: &[] };
    assert_eq!(
        (fabrewk_install::INSTALLER.assess)(&asked).unwrap_err()[0].code(),
        Some(fabrewk_core::Code::AliasInvalid)
    );
    std::fs::remove_dir_all(&dir).unwrap();
}

fn options_alias() -> Options {
    options(&["assess", "--alias", "Not-An-Alias"])
}
