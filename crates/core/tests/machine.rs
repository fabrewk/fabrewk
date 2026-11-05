//! The machine (part 7c2b, D-051): `machine.toml` read from the private folder and shown as declared (REQ-SET-1,
//! REQ-SET-7), detection with the effective set (REQ-SET-8) and the validation of profiles (REQ-SET-9). Fake programs
//! stand for the modules' (core-specification 11.2 rule 2); `PATH` is the fixture's own.

mod common;

use common::*;
use fabrewk_core::detect::{self, Detection, How, Program, Space};
use fabrewk_core::machine;
use fabrewk_core::record::{Code, is_utc_timestamp, sha256_hex};
use fabrewk_core::settings::{self, Effective, Level, Policy, Source, Val};
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

const CLAUDE: Program = Program { adapter: Some("claude-code"), executable: "fake-claude", version: &["--version"] };
const BACKEND: Program = Program { adapter: None, executable: "fake-bwrap", version: &["--version"] };

/// A policy whose text is `text`, as an adoption would give it.
fn policy(dir: &Path, text: &str) -> Policy {
    let path = dir.join("settings.toml");
    let settings = settings::parse(&path, text, Level::Project).unwrap();
    Policy { digest: sha256_hex(text.as_bytes()), text: text.into(), settings }
}

/// A program in `bin` that prints `version` when asked with `--version`, and a usage line, failing, otherwise.
fn program(bin: &Path, name: &str, version: &str) -> PathBuf {
    fs::create_dir_all(bin).unwrap();
    let path = bin.join(name);
    fs::write(&path, format!("#!/bin/sh\n[ \"$1\" = --version ] && echo '{version}' && exit 0\necho usage\nexit 3\n"))
        .unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    path
}

/// `bin` before this process's own `PATH`, so the core's `git` and `ssh-keygen` are found.
fn path_with(bin: &Path) -> OsString {
    let mut dirs = vec![bin.to_path_buf()];
    dirs.extend(std::env::split_paths(&std::env::var_os("PATH").unwrap()));
    std::env::join_paths(dirs).unwrap()
}

fn eff(p: &Policy, machine: Option<&settings::Settings>) -> Effective {
    settings::effective(p, None, machine, &[], &[]).unwrap()
}

const CATALOGUE: &str = r#"[project]
name = "fixture"

[tools.claude]
adapter = "claude-code"
providers = ["anthropic"]

[tools.codex]
adapter = "codex"
providers = ["openai"]

[providers.anthropic]
allowed = true

[providers.openai]
allowed = false

[models.opus]
provider = "anthropic"
id = "claude-opus-5-5"
efforts = ["high", "max"]

[models.sol]
provider = "openai"
id = "gpt-6.1-sol"
efforts = ["high"]

[profiles.build]
kind = "build"
tool = "claude"
model = "opus"
effort = "high"
requiredTools = ["fake-cargo"]
"#;

#[test]
fn req_set_1_the_machine_file_is_read_from_the_private_folder_and_shown_as_declared() {
    let base = tmp("machine-file");
    let folder = base.join("fabrewk");
    assert!(machine::machine_file(&folder).unwrap().is_none());
    fs::create_dir(&folder).unwrap();
    fs::set_permissions(&folder, fs::Permissions::from_mode(0o700)).unwrap();
    assert!(machine::machine_file(&folder).unwrap().is_none());

    let text = "[tools.claude]\npath = \"/opt/claude/bin/claude\"\n\n[tools.other]\npath = \"/opt/other\"\n";
    fs::write(folder.join("machine.toml"), text).unwrap();
    let declared = machine::machine_file(&folder).unwrap().unwrap();
    let p = policy(&base, CATALOGUE);
    let e = eff(&p, Some(&declared));
    assert_eq!(e.values["tools.claude.path"], (Val::Str("/opt/claude/bin/claude".into()), Source::Declared));
    assert_eq!(e.values["tools.other.path"].1, Source::Declared);
    assert!(matches!(&e.values["tools.claude.adapter"].1, Source::Project(d) if *d == p.digest));
    assert_eq!(fs::read_to_string(folder.join("machine.toml")).unwrap(), text, "never rewritten");

    // A path that is not absolute, or a key that is not the machine's, is refused with its line.
    fs::write(folder.join("machine.toml"), "\n[tools.claude]\npath = \"bin/claude\"\n").unwrap();
    let e = machine::machine_file(&folder).unwrap_err();
    assert_eq!(e.iter().map(|e| e.code().unwrap()).collect::<Vec<_>>(), vec![Code::SettingsInvalid]);
    assert!(format!("{e:?}").contains("tools.claude.path (line 3)"), "{e:?}");
    fs::write(folder.join("machine.toml"), "[limits]\nrounds = 9\n").unwrap();
    let e = machine::machine_file(&folder).unwrap_err();
    assert!(format!("{e:?}").contains("limits.rounds (line 2): not allowed"), "{e:?}");

    // A folder others can read is not read (REQ-SEC-4).
    fs::set_permissions(&folder, fs::Permissions::from_mode(0o755)).unwrap();
    let e = machine::machine_file(&folder).unwrap_err();
    assert_eq!(e.iter().map(|e| e.code().unwrap()).collect::<Vec<_>>(), vec![Code::PrivateFolderPermissions]);
}

fn found(d: &Detection, name: &str) -> detect::Found {
    d.get(Space::Tool, name).cloned().unwrap_or_else(|| panic!("{name} absent: {d:?}"))
}

fn own(d: &Detection, executable: &str) -> detect::Found {
    d.get(Space::Program, executable).cloned().unwrap_or_else(|| panic!("{executable} absent: {d:?}"))
}

/// The names looked for in `space`.
fn names(d: &Detection, space: Space) -> Vec<String> {
    d.found.keys().filter(|(s, _)| *s == space).map(|(_, n)| n.clone()).collect()
}

#[test]
fn req_set_8_detection_is_fresh_at_every_start_and_the_effective_set_is_permitted_and_present() {
    let base = tmp("detect");
    let (domain, bin) = (base.join(".brewstillery"), base.join("bin"));
    fs::create_dir_all(&domain).unwrap();
    let claude = program(&bin, "fake-claude", "9.9.0 (fake)");
    program(&bin, "fake-bwrap", "bubblewrap 0.0.1");
    program(&bin, "fake-cargo", "cargo 0.1");
    let other = program(&base.join("opt"), "other", "other 1");
    let folder = base.join("private");
    fs::create_dir(&folder).unwrap();
    fs::set_permissions(&folder, fs::Permissions::from_mode(0o700)).unwrap();
    let machine_text = format!("[tools.other]\npath = \"{}\"\n", other.display());
    fs::write(folder.join("machine.toml"), &machine_text).unwrap();
    let declared = machine::machine_file(&folder).unwrap();
    let p = policy(&base, CATALOGUE);
    let e = eff(&p, declared.as_ref());
    let path = path_with(&bin);
    let modules = [CLAUDE, BACKEND];

    let d = detect::detect(&domain, &e, &modules, Some(&path));
    assert!(is_utc_timestamp(&d.at), "{}", d.at);
    assert_eq!(names(&d, Space::Program), ["fake-bwrap", "git", "ssh-keygen"]);
    assert_eq!(names(&d, Space::Tool), ["claude", "codex", "other"]);
    assert_eq!(names(&d, Space::Required), ["fake-cargo"]);
    let c = found(&d, "claude");
    assert_eq!(
        (c.path, c.version.as_deref(), c.how, c.verified),
        (claude.clone(), Some("9.9.0 (fake)"), How::Observed, None)
    );
    assert_eq!(own(&d, "fake-bwrap").version.as_deref(), Some("bubblewrap 0.0.1"));
    // A required tool has no module: present, with no version read.
    assert_eq!(d.get(Space::Required, "fake-cargo").unwrap().version, None);
    assert!(own(&d, "git").version.unwrap().starts_with("git version "));
    assert_eq!(own(&d, "ssh-keygen").version, None);
    // Declared in `machine.toml`, present, and not in the catalogue: not effective. `codex` is permitted, but no
    // module drives its adapter here and nothing declares where it is: absent, and not effective either.
    assert_eq!(found(&d, "other").how, How::Declared);
    assert_eq!(d.get(Space::Tool, "codex"), None);
    assert_eq!(detect::effective_tools(&e, &d).into_iter().collect::<Vec<_>>(), ["claude"]);
    assert!(!domain.join("local").exists(), "detection writes nothing");

    // Removed between two commands: absent at the second, whatever an earlier command kept.
    d.keep(&domain).unwrap();
    fs::remove_file(&claude).unwrap();
    let d = detect::detect(&domain, &e, &modules, Some(&path));
    assert_eq!(d.get(Space::Tool, "claude").cloned(), None);
    assert!(detect::effective_tools(&e, &d).is_empty());
    assert_eq!(fs::read_to_string(folder.join("machine.toml")).unwrap(), machine_text, "no settings file is written");
    assert_eq!(fs::read_to_string(base.join("settings.toml")).ok(), None);
}

#[test]
fn req_set_8_where_a_program_is_found_and_where_it_is_not() {
    let base = tmp("detect-where");
    let (domain, bin) = (base.join(".brewstillery"), base.join("bin"));
    let first = program(&bin, "fake-claude", "1");
    program(&base.join("later"), "fake-claude", "2");
    let p = policy(&base, CATALOGUE);
    let e = eff(&p, None);
    let detect =
        |path: &OsString| detect::detect(&domain, &e, &[CLAUDE], Some(path)).get(Space::Tool, "claude").cloned();
    // The first directory of `PATH` that holds it; a relative directory and an empty entry are never searched.
    let later = base.join("later");
    let both = std::env::join_paths([bin.clone(), later.clone()]).unwrap();
    assert_eq!(detect(&both).unwrap().path, first);
    // The directory `rel`, named relative to this process's own directory (up to the root, then down).
    let rel = base.join("rel");
    program(&rel, "fake-claude", "3");
    let up = std::env::current_dir().unwrap().components().count() - 1;
    let relative = Path::new(&"../".repeat(up)).join(rel.strip_prefix("/").unwrap());
    assert!(relative.join("fake-claude").is_file());
    let mut entries = OsString::from(relative.as_os_str());
    entries.push("::");
    assert_eq!(detect(&entries), None);
    // A file that is not executable, or a directory, is not the program.
    fs::set_permissions(&first, fs::Permissions::from_mode(0o644)).unwrap();
    assert_eq!(detect(&both).unwrap().path, later.join("fake-claude"));
    fs::remove_file(&first).unwrap();
    fs::create_dir(&first).unwrap();
    assert_eq!(detect(&both).unwrap().path, later.join("fake-claude"));
    // A version that fails is none, whatever it printed; the program is still present.
    let failing = Program { version: &["--fail"], ..CLAUDE };
    let d = detect::detect(&domain, &e, &[failing], Some(&both));
    assert_eq!(d.get(Space::Tool, "claude").cloned().as_ref().unwrap().version, None);
    // No `PATH`: only what is declared.
    assert_eq!(detect(&OsString::new()), None);
    assert_eq!(detect::detect(&domain, &e, &[CLAUDE], None).get(Space::Tool, "claude"), None);

    // A declared path takes the place of the search: present there, or absent, never looked for in `PATH`.
    let folder = base.join("private");
    fs::create_dir(&folder).unwrap();
    fs::set_permissions(&folder, fs::Permissions::from_mode(0o700)).unwrap();
    let elsewhere = program(&base.join("elsewhere"), "claude", "7");
    for (declared, expected) in [(elsewhere.clone(), Some(elsewhere)), (base.join("missing"), None)] {
        fs::write(folder.join("machine.toml"), format!("[tools.claude]\npath = \"{}\"\n", declared.display())).unwrap();
        let m = machine::machine_file(&folder).unwrap();
        let e = eff(&p, m.as_ref());
        let got = detect::detect(&domain, &e, &[CLAUDE], Some(&both)).get(Space::Tool, "claude").cloned();
        assert_eq!(got.as_ref().map(|f| (&f.path, f.how)), expected.as_ref().map(|p| (p, How::Declared)));
        if expected.is_some() {
            assert_eq!(got.unwrap().version.as_deref(), Some("7"));
        }
    }
    // The core's own programs are found in `PATH` only, whatever `machine.toml` says.
    fs::write(folder.join("machine.toml"), "[tools.git]\npath = \"/nowhere/git\"\n").unwrap();
    let m = machine::machine_file(&folder).unwrap();
    let e = eff(&p, m.as_ref());
    let path = path_with(&bin);
    let d = detect::detect(&domain, &e, &[], Some(&path));
    assert_eq!(own(&d, "git").how, How::Observed);
    assert_eq!(d.get(Space::Tool, "git").cloned(), None, "the declared name, apart, is absent there");
    // A tool the catalogue names like a program is not that program: absent here, and not effective.
    let text = CATALOGUE.replace("[tools.codex]", "[tools.git]");
    let e = eff(&policy(&base, &text), None);
    let d = detect::detect(&domain, &e, &[CLAUDE], Some(&path));
    assert_eq!((d.get(Space::Tool, "git").cloned().clone(), own(&d, "git").how), (None, How::Observed));
    assert!(detect::effective_tools(&e, &d).is_empty(), "`claude` is a directory here, and `git` is not the program");
    // A required tool's name holding `/` is never a path.
    let text = CATALOGUE.replace("[\"fake-cargo\"]", &format!("[\"{}\"]", base.join("bin/fake-claude").display()));
    let e = eff(&policy(&base, &text), None);
    let d = detect::detect(&domain, &e, &[], Some(&path));
    assert_eq!(d.get(Space::Required, &base.join("bin/fake-claude").display().to_string()), None);
}

#[test]
fn req_set_8_write_commands_keep_the_detection_dated_and_only_a_use_verifies() {
    let base = tmp("detect-keep");
    let (domain, bin) = (base.join(".brewstillery"), base.join("bin"));
    fs::create_dir_all(&domain).unwrap();
    program(&bin, "fake-claude", "9.9.0");
    let p = policy(&base, CATALOGUE);
    let e = eff(&p, None);
    let path = path_with(&bin);
    let detect = || detect::detect(&domain, &e, &[CLAUDE], Some(&path));

    let mut d = detect();
    d.used(Space::Tool, "claude");
    d.used(Space::Tool, "codex");
    let at = found(&d, "claude").verified.unwrap();
    assert!(is_utc_timestamp(&at));
    assert_eq!(d.get(Space::Tool, "codex").cloned(), None, "an absent program is never verified");
    d.keep(&domain).unwrap();
    let kept = fs::read(domain.join("local/cache/detected.json")).unwrap();
    let value = fabrewk_core::record::decode(&kept).expect("canonical JSON");
    assert_eq!(value["schema"], 1);
    assert_eq!(value["at"], d.at.as_str());
    assert_eq!(value["tools"]["claude"]["how"], "observed");
    assert_eq!(value["tools"]["claude"]["verified"], at.as_str());
    assert_eq!(value["tools"]["codex"], serde_json::Value::Null);

    // The next start observes again and carries the verification: same path, same version.
    let again = detect();
    assert_eq!(found(&again, "claude").verified.as_deref(), Some(at.as_str()));
    assert_eq!(own(&again, "git").verified, None);
    assert_eq!(value["programs"]["git"]["how"], "observed");
    // A new version, or the program elsewhere, is not the one verified.
    program(&bin, "fake-claude", "10.0.0");
    assert_eq!(found(&detect(), "claude").verified, None);
    program(&bin, "fake-claude", "9.9.0");
    assert!(found(&detect(), "claude").verified.is_some());
    let moved = base.join("moved");
    let other = std::env::join_paths([moved.clone()]).unwrap();
    program(&moved, "fake-claude", "9.9.0");
    let d = detect::detect(&domain, &e, &[CLAUDE], Some(&other));
    assert_eq!(found(&d, "claude").verified, None);
    // A cache that does not read is none: nothing verified, nothing refused.
    for damage in [&b"{"[..], b"[]", b"{\"tools\":{\"claude\":7}}"] {
        fs::write(domain.join("local/cache/detected.json"), damage).unwrap();
        assert_eq!(found(&detect(), "claude").verified, None);
    }
}

/// The policy `CATALOGUE` with `edit` applied to its text.
fn profile_case(base: &Path, edit: impl Fn(&str) -> String) -> Policy {
    policy(base, &edit(CATALOGUE))
}

#[test]
fn req_set_9_each_invalid_profile_is_refused_before_any_dispatch() {
    let base = tmp("profiles");
    let (domain, bin) = (base.join(".brewstillery"), base.join("bin"));
    program(&bin, "fake-claude", "9.9.0");
    program(&bin, "fake-cargo", "1");
    let path = path_with(&bin);
    let check = |p: &Policy, layers: &[(Source, BTreeMap<String, Val>)]| {
        let e = settings::effective(p, None, None, layers, &[]).unwrap();
        let d = detect::detect(&domain, &e, &[CLAUDE], Some(&path));
        detect::check_profile(p, &e, &d, "build")
    };
    let valid = profile_case(&base, str::to_owned);
    check(&valid, &[]).unwrap();

    let line_of = |text: &str, needle: &str| text.lines().position(|l| l.starts_with(needle)).unwrap() + 1;
    let cases: [(&str, &str, &str, &str); 5] = [
        ("tool = \"claude\"", "tool = \"nosuch\"", "tool", "not a tool of the catalogue"),
        ("model = \"opus\"", "model = \"nosuch\"", "model", "not a model of the catalogue"),
        ("model = \"opus\"", "model = \"sol\"", "model", "of a provider the tool does not use"),
        ("effort = \"high\"\nrequired", "effort = \"low\"\nrequired", "effort", "not one of the model's efforts"),
        ("[providers.anthropic]\nallowed = true", "[providers.anthropic]\nallowed = false", "model", "not permitted"),
    ];
    for (from, to, leaf, why) in cases {
        let text = CATALOGUE.replace(from, to);
        assert_ne!(text, CATALOGUE);
        let p = policy(&base, &text);
        let e = check(&p, &[]).unwrap_err();
        assert_eq!(e.code(), Some(Code::SettingsInvalid), "{to}");
        let line = line_of(&text, &format!("{leaf} = "));
        let detail = format!("{e:?}");
        assert!(detail.contains(&format!("profiles.build.{leaf} (line {line}): ")), "{to}: {detail}");
        assert!(detail.contains(why), "{to}: {detail}");
        assert!(detail.contains("settings.toml"), "{detail}");
    }
    // A provider with no table is not permitted either.
    let p = profile_case(&base, |t| t.replace("[providers.anthropic]\nallowed = true\n", ""));
    assert_eq!(check(&p, &[]).unwrap_err().code(), Some(Code::SettingsInvalid));
    // A value a stage set is named by the stage.
    let stage = (
        Source::Stage("s-0000abcd".into()),
        BTreeMap::from([("profiles.build.effort".into(), Val::Str("low".into()))]),
    );
    let e = settings::effective(&valid, None, None, std::slice::from_ref(&stage), &["profiles.build.effort".into()]);
    let e = e.unwrap();
    let d = detect::detect(&domain, &e, &[CLAUDE], Some(&path));
    let err = detect::check_profile(&valid, &e, &d, "build").unwrap_err();
    assert!(
        format!("{err:?}").contains("profiles.build.effort (s-0000abcd): not one of the model's efforts"),
        "{err:?}"
    );
    // No such profile.
    let e = settings::effective(&valid, None, None, &[], &[]).unwrap();
    let d = detect::detect(&domain, &e, &[CLAUDE], Some(&path));
    let err = detect::check_profile(&valid, &e, &d, "nosuch").unwrap_err();
    assert_eq!(err.code(), Some(Code::SettingsInvalid));

    // The tool permitted but absent, and a required tool absent: `TOOL_MISSING`.
    fs::remove_file(bin.join("fake-cargo")).unwrap();
    let err = check(&valid, &[]).unwrap_err();
    assert_eq!(err.code(), Some(Code::ToolMissing));
    assert!(format!("{err:?}").contains("fake-cargo"), "{err:?}");
    fs::remove_file(bin.join("fake-claude")).unwrap();
    let err = check(&valid, &[]).unwrap_err();
    assert_eq!(err.code(), Some(Code::ToolMissing));
    assert!(format!("{err:?}").contains("claude: not present"), "{err:?}");
    // What the policy says is checked before what the machine holds.
    let p = profile_case(&base, |t| t.replace("effort = \"high\"\nrequired", "effort = \"low\"\nrequired"));
    assert_eq!(check(&p, &[]).unwrap_err().code(), Some(Code::SettingsInvalid));
}

#[test]
fn req_set_8_presence_is_what_this_process_may_execute_and_each_space_is_apart() {
    let base = tmp("detect-spaces");
    let (domain, bin) = (base.join(".brewstillery"), base.join("bin"));
    fs::create_dir_all(&domain).unwrap();
    let claude = program(&bin, "fake-claude", "1");
    program(&bin, "fake-cargo", "1");
    let path = std::env::join_paths([bin.clone()]).unwrap();
    let p = policy(&base, CATALOGUE);
    let e = eff(&p, None);

    // An execute bit this process may not use is no presence (D-052): `0601` gives it to others only.
    for (mode, present) in [(0o601, false), (0o100, true), (0o755, true)] {
        fs::set_permissions(&claude, fs::Permissions::from_mode(mode)).unwrap();
        let d = detect::detect(&domain, &e, &[CLAUDE], Some(&path));
        assert_eq!(d.present(Space::Tool, "claude"), present, "{mode:o}");
    }

    // A required tool is looked for by its own name, never as the catalogued tool of that name (P7C2B-2).
    let text = CATALOGUE.replace("requiredTools = [\"fake-cargo\"]", "requiredTools = [\"claude\"]");
    let p = policy(&base, &text);
    let e = eff(&p, None);
    let d = detect::detect(&domain, &e, &[CLAUDE], Some(&path));
    assert!(d.present(Space::Tool, "claude"));
    assert!(!d.present(Space::Required, "claude"));
    let err = detect::check_profile(&p, &e, &d, "build").unwrap_err();
    assert_eq!(err.code(), Some(Code::ToolMissing));
    program(&bin, "claude", "1");
    let d = detect::detect(&domain, &e, &[CLAUDE], Some(&path));
    detect::check_profile(&p, &e, &d, "build").unwrap();

    // A catalogued tool named like a core program is not that program (P7C2B-1): `git` in `PATH`, its module's
    // executable absent, and the profile that uses it refused.
    let text = CATALOGUE.replace("[tools.claude]", "[tools.git]").replace("tool = \"claude\"", "tool = \"git\"");
    let p = policy(&base, &text);
    let e = eff(&p, None);
    let real = std::env::var_os("PATH").unwrap();
    let d = detect::detect(&domain, &e, &[CLAUDE], Some(&real));
    assert!(d.present(Space::Program, "git") && !d.present(Space::Tool, "git"));
    assert!(detect::effective_tools(&e, &d).is_empty());
    assert_eq!(detect::check_profile(&p, &e, &d, "build").unwrap_err().code(), Some(Code::ToolMissing));

    // A path that is not UTF-8 carries no verification, to itself or to another that reads the same (P7C2B-4).
    use std::os::unix::ffi::OsStrExt;
    let odd = |b: u8| base.join(std::ffi::OsStr::from_bytes(&[b'd', b]));
    let e = eff(&policy(&base, CATALOGUE), None);
    let (first, second) = (odd(0xff), odd(0xfe));
    program(&first, "fake-claude", "1");
    program(&second, "fake-claude", "1");
    let in_dir = |d: &Path| std::env::join_paths([d.to_path_buf()]).unwrap();
    let mut d = detect::detect(&domain, &e, &[CLAUDE], Some(&in_dir(&first)));
    d.used(Space::Tool, "claude");
    d.keep(&domain).unwrap();
    let kept = fabrewk_core::record::decode(&fs::read(domain.join("local/cache/detected.json")).unwrap()).unwrap();
    assert_eq!(kept["tools"]["claude"]["path"], serde_json::Value::Null);
    for dir in [&first, &second] {
        let d = detect::detect(&domain, &e, &[CLAUDE], Some(&in_dir(dir)));
        assert_eq!(found(&d, "claude").verified, None, "{dir:?}");
    }
}
