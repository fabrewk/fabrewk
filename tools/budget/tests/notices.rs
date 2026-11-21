//! The release's third-party notices and the release itself (REQ-BUD-5; part 10d, D-108): every crate the binary
//! links on the release's target, with its declared licence and the texts it carries, each distinct text once; the
//! toolchain's libraries checked against the toolchain; a fresh generation gives the same bytes; a release holds
//! exactly its files, reproducibly, and its notices equal a fresh generation.

use budget::notices::{self, Inputs, Linked, Toolchain};
use budget::release;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

fn tmp(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("notices").join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn linked(name: &str, version: &str, license: &str, texts: &[(&str, &str)]) -> Linked {
    Linked {
        name: name.into(),
        version: version.into(),
        license: license.into(),
        texts: texts.iter().map(|(f, t)| (f.to_string(), t.to_string())).collect(),
    }
}

#[test]
fn req_bud_5_each_distinct_text_once_with_what_carries_it_in_the_list_order() {
    let inputs = Inputs {
        version: "9.8.7".into(),
        rustc: "1.2.3".into(),
        crates: vec![
            linked(
                "alpha",
                "1.0.0",
                "MIT OR Apache-2.0",
                &[("LICENSE-APACHE", "APACHE\n"), ("LICENSE-MIT", "MIT a\n")],
            ),
            linked("beta", "2.0.0", "Apache-2.0", &[("LICENSE", "APACHE\n")]),
        ],
        toolchain: vec![linked("musl libc", "1.2.5", "MIT", &[("COPYRIGHT", "MUSL, no line end")])],
    };
    let text = notices::render(&inputs);
    assert!(text.starts_with("Third-party notices of fabrewk 9.8.7 for aarch64-unknown-linux-musl\n"), "{text}");
    assert!(text.contains("rustc 1.2.3;"), "{text}");
    for line in [
        "  alpha 1.0.0: MIT OR Apache-2.0\n",
        "  beta 2.0.0: Apache-2.0\n",
        "  musl libc 1.2.5: MIT\n",
        "  The Rust standard library 1.2.3: MIT OR Apache-2.0, with the notices of what it holds, as\n",
        "    the toolchain ships them, in THIRD-PARTY-NOTICES-std.html\n",
    ] {
        assert!(text.contains(line), "{line:?} in {text}");
    }
    // The shared text once, carried by both, before the others; a text with no line end gets one.
    let rule = "=".repeat(80);
    let thin = "-".repeat(80);
    let expected = format!(
        "\n{rule}\nText 1 of 3, carried by:\n  alpha 1.0.0, LICENSE-APACHE\n  beta 2.0.0, LICENSE\n{thin}\nAPACHE\n\
         \n{rule}\nText 2 of 3, carried by:\n  alpha 1.0.0, LICENSE-MIT\n{thin}\nMIT a\n\
         \n{rule}\nText 3 of 3, carried by:\n  musl libc 1.2.5, COPYRIGHT\n{thin}\nMUSL, no line end\n"
    );
    assert!(text.ends_with(&expected), "{text}");
    assert_eq!(text.matches("\nAPACHE\n").count(), 1);
}

#[test]
fn req_bud_5_a_package_s_licence_texts_are_its_top_files_named_as_licences() {
    for (name, is) in [
        ("LICENSE", true),
        ("license-mit", true),
        ("LICENCE.txt", true),
        ("COPYING", true),
        ("COPYRIGHT", true),
        ("NOTICE", true),
        ("UNLICENSE", true),
        ("README.md", false),
        ("src", false),
        ("THE-LICENSE", false),
    ] {
        assert_eq!(notices::is_licence_text(name), is, "{name}");
    }
    let dir = tmp("texts");
    fs::write(dir.join("LICENSE-MIT"), "mit").unwrap();
    fs::write(dir.join("COPYING"), "copying").unwrap();
    fs::write(dir.join("README.md"), "readme").unwrap();
    fs::create_dir(dir.join("LICENSES")).unwrap();
    fs::create_dir(dir.join("sub")).unwrap();
    fs::write(dir.join("sub/LICENSE"), "deeper").unwrap();
    let texts = notices::licence_texts(&dir, "x 1").unwrap();
    assert_eq!(texts, [("COPYING".to_string(), "copying".to_string()), ("LICENSE-MIT".into(), "mit".into())]);
    let empty = tmp("no-texts");
    fs::write(empty.join("README.md"), "readme").unwrap();
    let err = notices::licence_texts(&empty, "x 1").unwrap_err();
    assert!(err.starts_with("x 1 carries no licence text in "), "{err}");
}

/// A metadata document shaped like Cargo's, the packages' directories under `root`.
fn metadata(root: &Path, extra_license: &str) -> String {
    let m = |dir: &str| root.join(dir).join("Cargo.toml").display().to_string();
    format!(
        r#"{{"packages":[
 {{"id":"bin","name":"fabrewk","version":"0.1.0","source":null,"manifest_path":"{}","targets":[]}},
 {{"id":"z","name":"zeta","version":"1.0.0","source":"registry+r","license":"MIT","manifest_path":"{}","targets":[]}},
 {{"id":"a","name":"alpha","version":"2.0.0","source":"registry+r","license":"MIT OR Apache-2.0","manifest_path":"{}","targets":[]}},
 {{"id":"a1","name":"alpha","version":"1.0.0","source":"registry+r"{extra_license},"manifest_path":"{}","targets":[]}},
 {{"id":"dev","name":"proptest","version":"1.0.0","source":"registry+r","license":"MIT","manifest_path":"{}","targets":[]}}],
 "workspace_members":["bin"],
 "resolve":{{"nodes":[
  {{"id":"bin","deps":[{{"pkg":"z","dep_kinds":[{{"kind":null}}]}},{{"pkg":"dev","dep_kinds":[{{"kind":"dev"}}]}},
                       {{"pkg":"a1","dep_kinds":[{{"kind":"build"}}]}}]}},
  {{"id":"z","deps":[{{"pkg":"a","dep_kinds":[{{"kind":null}}]}}]}},
  {{"id":"a","deps":[]}},{{"id":"a1","deps":[]}},{{"id":"dev","deps":[]}}]}}}}"#,
        m("bin"),
        m("zeta"),
        m("alpha2"),
        m("alpha1"),
        m("proptest")
    )
}

#[test]
fn req_bud_5_the_crates_are_what_the_binary_reaches_but_through_dev_dependencies_sorted() {
    let root = tmp("crates");
    for (dir, file) in [("bin", "LICENSE"), ("zeta", "LICENSE"), ("alpha2", "LICENSE-MIT"), ("alpha1", "COPYING")] {
        fs::create_dir_all(root.join(dir)).unwrap();
        fs::write(root.join(dir).join(file), format!("{dir} text\n")).unwrap();
    }
    let own = [root.join("bin")];
    let crates = notices::crates(&metadata(&root, r#","license":"Unlicense""#), &own).unwrap();
    let names: Vec<String> = crates.iter().map(|c| format!("{} {}: {}", c.name, c.version, c.license)).collect();
    assert_eq!(names, ["alpha 1.0.0: Unlicense", "alpha 2.0.0: MIT OR Apache-2.0", "zeta 1.0.0: MIT"]);
    assert_eq!(crates[1].texts, [("LICENSE-MIT".to_string(), "alpha2 text\n".to_string())]);
    // A crate that declares no licence, or carries no text, stops the generation.
    let err = notices::crates(&metadata(&root, ""), &own).unwrap_err();
    assert_eq!(err, "alpha 1.0.0 declares no licence");
    fs::remove_file(root.join("zeta/LICENSE")).unwrap();
    let err = notices::crates(&metadata(&root, r#","license":"MIT""#), &own).unwrap_err();
    assert!(err.starts_with("zeta 1.0.0 carries no licence text in "), "{err}");
}

#[test]
fn req_bud_5_the_toolchain_s_libraries_are_checked_against_the_toolchain() {
    let sysroot = tmp("sysroot");
    let lib = sysroot.join("lib/rustlib/aarch64-unknown-linux-musl/lib/self-contained");
    fs::create_dir_all(&lib).unwrap();
    let t = |llvm: &str| Toolchain { release: "1.0.0".into(), llvm: llvm.into(), sysroot: sysroot.clone() };
    let err = notices::toolchain_libraries(&t("22.0.0")).unwrap_err();
    assert_eq!(err, "the toolchain's LLVM is 22.0.0; the libunwind text kept is for 23.1.1");
    fs::write(lib.join("libc.a"), b"x\x001.2.4\x00y 1.2.5").unwrap();
    let err = notices::toolchain_libraries(&t("23.1.1")).unwrap_err();
    assert!(err.ends_with("does not name musl 1.2.5, the version of the text kept"), "{err}");
    fs::write(lib.join("libc.a"), b"x\x001.2.5\x00y").unwrap();
    let libs = notices::toolchain_libraries(&t("23.1.1")).unwrap();
    let names: Vec<String> = libs.iter().map(|c| format!("{} {}: {}", c.name, c.version, c.license)).collect();
    assert_eq!(names, ["musl libc 1.2.5: MIT", "LLVM libunwind 23.1.1: Apache-2.0 WITH LLVM-exception"]);
    assert!(libs[0].texts[0].1.starts_with("musl as a whole is licensed under the following standard MIT license"));
    assert!(libs[1].texts[0].1.contains("The LLVM Project is under the Apache License v2.0 with LLVM Exceptions"));
    let err = notices::std_notices(&t("23.1.1")).unwrap_err();
    assert!(err.ends_with("; the toolchain must ship its notices"), "{err}");
}

fn budget(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_budget")).args(args).output().unwrap()
}

#[test]
fn req_bud_5_a_fresh_generation_gives_the_same_bytes_and_names_every_crate_the_binary_links() {
    let (one, two) = (tmp("fresh-1"), tmp("fresh-2"));
    for dir in [&one, &two] {
        let out = budget(&["notices", "--out", dir.to_str().unwrap()]);
        assert_eq!(out.status.code(), Some(0), "{}", String::from_utf8_lossy(&out.stderr));
    }
    for file in [notices::FILE, notices::STD_FILE] {
        assert_eq!(fs::read(one.join(file)).unwrap(), fs::read(two.join(file)).unwrap(), "{file}");
    }
    let text = fs::read_to_string(one.join(notices::FILE)).unwrap();
    // Every crate the release build counts on this target (`budget check`'s walk) is in the list, and none other.
    let lock = fs::read_to_string(budget::workspace_root().join("Cargo.lock")).unwrap();
    let list = &text[..text.find("\n=====").unwrap()];
    let listed: Vec<&str> = list.lines().filter(|l| l.starts_with("  ") && l.contains(": ")).collect();
    for name in ["serde_json", "sha2", "toml", "libc", "lexopt"] {
        assert!(listed.iter().any(|l| l.starts_with(&format!("  {name} "))), "{name}: {text}");
    }
    for line in listed.iter().filter(|l| !l.starts_with("  The ") && !l.starts_with("    ")) {
        let (name, version) = line.trim().split_once(':').unwrap().0.rsplit_once(' ').unwrap();
        if ["musl libc", "LLVM libunwind"].contains(&name) {
            continue;
        }
        let entry = format!("name = \"{name}\"\nversion = \"{version}\"\n");
        assert!(lock.contains(&entry), "{line} is not in Cargo.lock");
    }
    for dev in ["proptest", "rand", "r-efi"] {
        assert!(!listed.iter().any(|l| l.starts_with(&format!("  {dev} "))), "{dev}: {text}");
    }
    let sysroot = notices::toolchain().unwrap().sysroot;
    let std = fs::read(sysroot.join("share/doc/rust/COPYRIGHT-library.html")).unwrap();
    assert_eq!(fs::read(one.join(notices::STD_FILE)).unwrap(), std);
}

/// A program standing for the release binary, answering `--version --json` with `said`.
fn fake_binary(dir: &Path, said: &str) -> PathBuf {
    let path = dir.join("fabrewk");
    fs::write(&path, format!("#!/bin/sh\nprintf '%s\\n' '{said}'\n")).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    path
}

fn listing(archive: &Path) -> String {
    let out = Command::new("tar").args(["-tvzf"]).arg(archive).output().unwrap();
    assert!(out.status.success());
    String::from_utf8(out.stdout).unwrap()
}

#[test]
fn req_bud_5_a_release_holds_exactly_its_files_reproducibly_with_its_notices_fresh() {
    let version = env!("CARGO_PKG_VERSION");
    let base = release::base(version);
    assert_eq!(base, format!("fabrewk-{version}-linux-aarch64"));
    let bin = tmp("release-bin");
    let binary = fake_binary(&bin, &format!(r#"{{"features":[],"schema":"fabrewk.version/1","version":"{version}"}}"#));
    let (one, two) = (tmp("release-1"), tmp("release-2").join("new"));
    for out in [&one, &two] {
        let ran = budget(&["release", "--binary", binary.to_str().unwrap(), "--out", out.to_str().unwrap()]);
        assert_eq!(ran.status.code(), Some(0), "{}", String::from_utf8_lossy(&ran.stderr));
        let said = String::from_utf8(ran.stdout).unwrap();
        assert!(said.contains("ssh-keygen -Y sign -n fabrewk-release -f <the signing key> "), "{said}");
        let mut names: Vec<String> =
            fs::read_dir(out).unwrap().map(|e| e.unwrap().file_name().into_string().unwrap()).collect();
        names.sort();
        assert_eq!(names, ["SHA256SUMS".to_string(), format!("{base}.tar.gz")]);
    }
    let archive = one.join(format!("{base}.tar.gz"));
    // Reproducible: the same archive bytes from the same commit.
    assert_eq!(fs::read(&archive).unwrap(), fs::read(two.join(format!("{base}.tar.gz"))).unwrap());
    let sums = fs::read_to_string(one.join("SHA256SUMS")).unwrap();
    let digest = Command::new("sha256sum").arg(&archive).output().unwrap().stdout;
    let digest = String::from_utf8(digest).unwrap();
    assert_eq!(sums, format!("{}  {base}.tar.gz\n", digest.split(' ').next().unwrap()));
    let list = listing(&archive);
    let entries: Vec<(String, String)> = list
        .lines()
        .map(|l| {
            let f: Vec<&str> = l.split_whitespace().collect();
            (format!("{} {}", f[0], f[1]), f[5].to_owned())
        })
        .collect();
    let file = |m: &str, p: &str| (format!("{m} 0/0"), format!("{base}/{p}"));
    let expected = vec![
        file("drwxr-xr-x", ""),
        file("-rw-r--r--", "LICENSE-APACHE"),
        file("-rw-r--r--", "LICENSE-MIT"),
        file("-rw-r--r--", "NOTICE"),
        file("-rw-r--r--", "THIRD-PARTY-NOTICES"),
        file("-rw-r--r--", "THIRD-PARTY-NOTICES-std.html"),
        file("drwxr-xr-x", "completions/"),
        file("-rw-r--r--", "completions/fabrewk.fish"),
        file("-rw-r--r--", "completions/fabrewk.sh"),
        file("-rwxr-xr-x", "fabrewk"),
    ];
    assert_eq!(entries, expected, "{list}");
    // The notices it carries equal a fresh generation; the other files are the repository's.
    let unpacked = tmp("release-unpacked");
    assert!(Command::new("tar").arg("-xzf").arg(&archive).arg("-C").arg(&unpacked).status().unwrap().success());
    let fresh = tmp("release-fresh");
    assert_eq!(budget(&["notices", "--out", fresh.to_str().unwrap()]).status.code(), Some(0));
    let root = budget::workspace_root();
    for (inside, theirs) in [
        (notices::FILE, fresh.join(notices::FILE)),
        (notices::STD_FILE, fresh.join(notices::STD_FILE)),
        ("LICENSE-MIT", root.join("LICENSE-MIT")),
        ("LICENSE-APACHE", root.join("LICENSE-APACHE")),
        ("NOTICE", root.join("NOTICE")),
        ("completions/fabrewk.fish", root.join("crates/fabrewk/completions/fabrewk.fish")),
        ("completions/fabrewk.sh", root.join("crates/fabrewk/completions/fabrewk.sh")),
        ("fabrewk", binary.clone()),
    ] {
        assert_eq!(fs::read(unpacked.join(&base).join(inside)).unwrap(), fs::read(theirs).unwrap(), "{inside}");
    }
}

#[test]
fn req_bud_5_a_release_refuses_another_version_a_feature_or_a_directory_not_empty() {
    let version = env!("CARGO_PKG_VERSION");
    let bin = tmp("refused-bin");
    let cases = [
        (r#"{"features":[],"version":"0.0.0"}"#.to_owned(), "the binary is not version"),
        (format!(r#"{{"features":["fault-injection"],"version":"{version}"}}"#), "the binary was built with features"),
        (format!(r#"{{"version":"{version}"}}"#), "the binary was built with features"),
    ];
    for (said, why) in &cases {
        let binary = fake_binary(&bin, said);
        let out = tmp("refused-out");
        let ran = budget(&["release", "--binary", binary.to_str().unwrap(), "--out", out.to_str().unwrap()]);
        assert_eq!(ran.status.code(), Some(70));
        assert!(String::from_utf8_lossy(&ran.stderr).contains(why), "{}", String::from_utf8_lossy(&ran.stderr));
        assert_eq!(fs::read_dir(&out).unwrap().count(), 0);
    }
    let binary = fake_binary(&bin, &format!(r#"{{"features":[],"version":"{version}"}}"#));
    let out = tmp("refused-full");
    fs::write(out.join("left"), "x").unwrap();
    let ran = budget(&["release", "--binary", binary.to_str().unwrap(), "--out", out.to_str().unwrap()]);
    assert_eq!(ran.status.code(), Some(70));
    assert!(String::from_utf8_lossy(&ran.stderr).contains("is not empty"), "{ran:?}");
    assert_eq!(fs::read_dir(&out).unwrap().count(), 1);
    assert_eq!(budget(&["release", "--binary", binary.to_str().unwrap()]).status.code(), Some(2));
}
