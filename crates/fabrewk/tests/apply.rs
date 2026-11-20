//! `fabrewk apply` through the built binary (REQ-INS-3, REQ-INS-8's baseline, REQ-DOM-1, REQ-DOM-4, REQ-DOM-6,
//! REQ-DOM-7; D-097 points 6 and 7; part 10b): the plan `assess` printed applied with the same options, its files and
//! five events, the receipt, the commit by mode, the interrupted `apply` finished by the same command, and its
//! refusals.

mod common;

use common::*;
use fabrewk_core::record::sha256_hex;
use serde_json::json;
use std::fs;

const SAY: &str = "Lo aplico.";

/// A product repository with one commit, a licence, the maestro `ana`'s key and Git signing with it.
fn product(name: &str) -> Fx {
    let base = tmp(name);
    let (root, home, keys) = (base.join("product"), base.join("home"), base.join("keys"));
    for d in [&root, &home, &keys] {
        fs::create_dir_all(d).unwrap();
    }
    let fx = Fx { domain: root.join(".brewstillery"), root, home, keys, path: None };
    for who in ["ana", "stranger"] {
        let key = fx.keys.join(who);
        fx.program("ssh-keygen", &["-q", "-t", "ed25519", "-N", "", "-C", who, "-f", key.to_str().unwrap()]);
    }
    fs::write(fx.root.join("README.md"), "product\n").unwrap();
    fs::write(fx.root.join("LICENSE"), "MIT\n").unwrap();
    fx.git(&["init", "-q", "-b", "main"]);
    for (k, v) in [("user.name", "Fixture"), ("user.email", "fixture@example.invalid"), ("commit.gpgsign", "false")] {
        fx.git(&["config", k, v]);
    }
    fx.git(&["config", "user.signingkey", fx.keys.join("ana.pub").to_str().unwrap()]);
    fx.git(&["config", "gpg.format", "ssh"]);
    // The person's own identity, which a repository `apply` makes for the domain takes.
    fs::write(fx.home.join(".gitconfig"), "[user]\n\tname = Ana\n\temail = ana@example.invalid\n").unwrap();
    fx.commit("product");
    fx
}

/// `assess`'s options for `mode`, and `more`.
fn options(fx: &Fx, mode: &str, more: &[&str]) -> Vec<String> {
    let key = fx.keys.join("ana.pub").to_str().unwrap().to_owned();
    let mut out: Vec<String> =
        ["--alias", "ana", "--key", &key, "--publication", mode, "--tool", "claude"].map(String::from).into();
    out.extend(more.iter().map(|s| s.to_string()));
    out
}

/// The plan's digest `assess` gives with `options`.
fn digest(fx: &Fx, options: &[String]) -> String {
    let args: Vec<&str> = ["assess", "--json"].into_iter().chain(options.iter().map(String::as_str)).collect();
    let ran = fx.run(&args);
    assert_eq!(ran.status, 0, "{ran:?}");
    ran.doc()["digest"].as_str().unwrap().to_owned()
}

/// `apply <digest> <options> --say … --json`, with `env`.
fn apply_env(fx: &Fx, digest: &str, options: &[String], env: &[(&str, &str)]) -> Ran {
    let mut args: Vec<&str> = vec!["apply", digest];
    args.extend(options.iter().map(String::as_str));
    args.extend(["--say", SAY, "--json"]);
    fx.run_env(&args, env)
}

fn apply(fx: &Fx, digest: &str, options: &[String]) -> Ran {
    apply_env(fx, digest, options, &[])
}

/// The kinds of a stream's events.
fn kinds(fx: &Fx, stream: &str) -> Vec<String> {
    fx.events(stream).iter().map(|e| e["kind"].as_str().unwrap().to_owned()).collect()
}

#[test]
fn req_ins_3_apply_writes_the_plan_s_files_and_its_five_events_in_order() {
    let fx = product("apply-published");
    let opts = options(&fx, "published", &[]);
    let d = digest(&fx, &opts);
    let ran = apply(&fx, &d, &opts);
    assert_eq!(ran.status, 0, "{ran:?}");
    let doc = ran.doc();
    assert_eq!((doc["schema"].clone(), doc["applied"].clone()), (json!("fabrewk.applied/1"), json!(true)), "{doc}");
    assert_eq!(kinds(&fx, "project"), ["assessed", "publication-decided", "policy-adopted", "installed"]);
    assert_eq!(kinds(&fx, "alambique"), ["alambique-baseline"]);
    let project = fx.events("project");
    assert_eq!(project[0]["payload"]["digest"], json!(d));
    assert_eq!(project[1]["payload"], json!({"mode": "published", "warningShown": false}));
    // The receipt: every file written or changed, with the digest of its bytes after.
    for f in project[3]["payload"]["files"].as_array().unwrap() {
        let bytes = fs::read(fx.root.join(f["path"].as_str().unwrap())).unwrap();
        assert_eq!(f["digest"], json!(sha256_hex(&bytes)), "{f}");
    }
    assert_eq!(fs::read_to_string(fx.domain.join(".gitignore")).unwrap(), "local/\n");
    assert_eq!(fs::read_to_string(fx.root.join("CLAUDE.md")).unwrap(), "@AGENTS.md\n");
    // One record commit holds everything apply wrote, and `check` passes.
    assert_eq!(fx.git(&["log", "-1", "--format=%s"]), "fabrewk: record");
    assert_eq!(fx.git(&["status", "--porcelain"]), "", "{}", fx.git(&["status", "--porcelain"]));
    let check = fx.run(&["check"]);
    assert_eq!(check.status, 0, "{check:?}");
    // The same apply again says so, writes nothing and exits 0.
    let before = fx.snapshot();
    let again = apply(&fx, &d, &opts);
    assert_eq!((again.status, again.doc()["applied"].clone()), (0, json!(false)), "{again:?}");
    assert_eq!(fx.snapshot(), before);
}

#[test]
fn req_dom_7_own_repository_commits_in_the_domain_s_repository_and_unpublished_commits_nothing() {
    for mode in ["own-repository", "unpublished"] {
        let fx = product(&format!("apply-{mode}"));
        let opts = options(&fx, mode, &[]);
        let (d, head) = (digest(&fx, &opts), fx.git(&["rev-parse", "HEAD"]));
        let ran = apply(&fx, &d, &opts);
        assert_eq!(ran.status, 0, "{mode}: {ran:?}");
        let doc = ran.doc();
        // The product's history does not move; its exclude file holds the line once.
        assert_eq!(fx.git(&["rev-parse", "HEAD"]), head, "{mode}");
        let exclude = fs::read_to_string(fx.root.join(".git/info/exclude")).unwrap();
        assert_eq!(exclude.lines().filter(|l| *l == "/.brewstillery/").count(), 1, "{mode}: {exclude}");
        assert_eq!(fx.git(&["check-ignore", ".brewstillery"]), ".brewstillery", "{mode}");
        // The entry files are left in the product's working tree, uncommitted, named with the commands that commit
        // them, which apply runs not.
        assert_eq!(doc["left"], json!(["AGENTS.md", "CLAUDE.md"]), "{mode}: {doc}");
        assert_eq!(doc["run"], json!(["git add -- AGENTS.md CLAUDE.md", "git commit"]), "{mode}: {doc}");
        assert_eq!(doc["commit"].is_null(), mode == "unpublished", "{mode}: {doc}");
        let status = fx.git(&["status", "--porcelain"]);
        assert_eq!(status, "?? AGENTS.md\n?? CLAUDE.md", "{mode}");
        if mode == "own-repository" {
            let git = |args: &[&str]| fx.git(&[&["-C", fx.domain.to_str().unwrap()][..], args].concat());
            assert_eq!(git(&["log", "--format=%s"]), "fabrewk: record", "one commit, the record's");
            assert_eq!(git(&["status", "--porcelain"]), "");
            assert_eq!(git(&["symbolic-ref", "HEAD"]), "refs/heads/main");
            let files = git(&["ls-files"]);
            for f in ["settings.toml", ".gitignore", "ALAMBIQUE.md", "STATUS.md", "RULES.md", "ana/fermento.md"] {
                assert!(files.lines().any(|l| l == f), "{f}: {files}");
            }
        } else {
            assert_eq!(fx.events("project")[1]["payload"], json!({"mode": "unpublished", "warningShown": true}));
        }
        let check = fx.run(&["check"]);
        assert_eq!(check.status, 0, "{mode}: {check:?}");
        let again = apply(&fx, &d, &opts);
        assert_eq!((again.status, again.doc()["applied"].clone()), (0, json!(false)), "{mode}: {again:?}");
    }
}

#[test]
fn req_dom_6_the_entry_blocks_change_nothing_outside_them() {
    let fx = product("apply-blocks");
    let mine = "# Ours\n\nKeep this.\n";
    fs::write(fx.root.join("AGENTS.md"), mine).unwrap();
    fs::write(fx.root.join("CLAUDE.md"), "Claude: ours.\n").unwrap();
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(fx.root.join("AGENTS.md"), fs::Permissions::from_mode(0o755)).unwrap();
    fx.commit("entry files");
    let opts = options(&fx, "published", &[]);
    let ran = apply(&fx, &digest(&fx, &opts), &opts);
    assert_eq!(ran.status, 0, "{ran:?}");
    let agents = fs::read_to_string(fx.root.join("AGENTS.md")).unwrap();
    let claude = fs::read_to_string(fx.root.join("CLAUDE.md")).unwrap();
    assert!(agents.starts_with(&format!("{mine}\n<!-- fabrewk:entry begin -->\n")), "{agents}");
    assert!(claude.starts_with("Claude: ours.\n\n<!-- fabrewk:entry begin -->\n"), "{claude}");
    assert!(agents.ends_with("<!-- fabrewk:entry end -->\n") && claude.ends_with("<!-- fabrewk:entry end -->\n"));
    // The file's mode stays, in the working tree and in the commit.
    assert_eq!(fs::metadata(fx.root.join("AGENTS.md")).unwrap().permissions().mode() & 0o777, 0o755);
    assert!(fx.git(&["ls-tree", "HEAD", "AGENTS.md"]).starts_with("100755 "));
    assert_eq!(fx.git(&["status", "--porcelain"]), "");
}

#[test]
fn req_ins_8_the_first_alambique_is_the_plan_s_text_and_its_digest_the_baseline() {
    for (lang, answers) in
        [("en", vec!["--purpose", "To count.", "--audience", "Us.", "--excluded", "Money."]), ("es", vec![])]
    {
        let fx = product(&format!("apply-alambique-{lang}"));
        let mut more = vec!["--lang", lang];
        more.extend(answers.iter());
        let opts = options(&fx, "published", &more);
        let args: Vec<&str> = ["assess", "--json"].into_iter().chain(opts.iter().map(String::as_str)).collect();
        let doc = fx.run(&args).doc();
        let planned =
            doc["plan"]["files"].as_array().unwrap().iter().find(|f| f["path"] == ".brewstillery/ALAMBIQUE.md");
        let text = planned.unwrap()["content"].as_str().unwrap().to_owned();
        assert!(answers.iter().skip(1).step_by(2).all(|a| text.contains(a)), "{text}");
        let ran = apply(&fx, doc["digest"].as_str().unwrap(), &opts);
        assert_eq!(ran.status, 0, "{lang}: {ran:?}");
        assert_eq!(fs::read_to_string(fx.domain.join("ALAMBIQUE.md")).unwrap(), text);
        assert_eq!(fx.events("alambique")[0]["payload"]["digest"], json!(sha256_hex(text.as_bytes())));
        assert_eq!(fx.run(&["check"]).status, 0, "{lang}");
    }
    // An alambique present is kept byte for byte, and its digest is the baseline.
    let fx = product("apply-alambique-kept");
    fs::create_dir_all(&fx.domain).unwrap();
    fs::write(fx.domain.join("ALAMBIQUE.md"), "# Ours\n").unwrap();
    fx.commit("alambique");
    let opts = options(&fx, "published", &[]);
    assert_eq!(apply(&fx, &digest(&fx, &opts), &opts).status, 0);
    assert_eq!(fs::read_to_string(fx.domain.join("ALAMBIQUE.md")).unwrap(), "# Ours\n");
    assert_eq!(fx.events("alambique")[0]["payload"]["digest"], json!(sha256_hex(b"# Ours\n")));
    assert_eq!(fx.run(&["check"]).status, 0);
}

#[test]
fn req_reg_10_apply_keeps_the_person_s_other_staged_and_unstaged_changes() {
    let fx = product("apply-edits");
    fs::write(fx.root.join("README.md"), "staged\n").unwrap();
    fx.git(&["add", "README.md"]);
    fs::write(fx.root.join("README.md"), "unstaged\n").unwrap();
    let opts = options(&fx, "published", &[]);
    let ran = apply(&fx, &digest(&fx, &opts), &opts);
    assert_eq!(ran.status, 0, "{ran:?}");
    assert_eq!(fx.git(&["show", "HEAD:README.md"]), "product");
    assert_eq!(fx.git(&["show", ":README.md"]), "staged");
    assert_eq!(fs::read_to_string(fx.root.join("README.md")).unwrap(), "unstaged\n");
    assert_eq!(fx.git(&["status", "--porcelain"]), "MM README.md");
}

#[test]
fn refusal_plan_changed() {
    let fx = product("apply-plan-changed");
    let opts = options(&fx, "published", &[]);
    let d = digest(&fx, &opts);
    // A root file changed between assess and apply.
    fs::write(fx.root.join("NOTES.md"), "new\n").unwrap();
    let args = |opts: &[String]| -> Vec<String> {
        ["apply".to_owned(), d.clone()]
            .into_iter()
            .chain(opts.iter().cloned())
            .chain(["--say".into(), SAY.into()])
            .collect()
    };
    let words = args(&opts);
    refused(&fx, &words.iter().map(String::as_str).collect::<Vec<_>>(), "PLAN_CHANGED", 1, "en");
    fs::remove_file(fx.root.join("NOTES.md")).unwrap();
    // Another option, a key other than the plan's.
    let other =
        options(&fx, "published", &[]).into_iter().map(|o| o.replace("ana.pub", "stranger.pub")).collect::<Vec<_>>();
    let words = args(&other);
    refused(&fx, &words.iter().map(String::as_str).collect::<Vec<_>>(), "PLAN_CHANGED", 1, "en");
    // An unknown digest.
    let words: Vec<String> = ["apply".to_owned(), "0".repeat(64)]
        .into_iter()
        .chain(opts.iter().cloned())
        .chain(["--say".into(), SAY.into()])
        .collect();
    refused(&fx, &words.iter().map(String::as_str).collect::<Vec<_>>(), "PLAN_CHANGED", 1, "en");
}

#[test]
fn refusal_verdict_none_and_refusal_mode_not_available() {
    let fx = product("apply-none");
    fs::write(fx.root.join("AGENTS.md"), "uncommitted\n").unwrap();
    let opts = options(&fx, "published", &[]);
    let d = digest(&fx, &opts);
    let words: Vec<String> =
        ["apply".to_owned(), d].into_iter().chain(opts.iter().cloned()).chain(["--say".into(), SAY.into()]).collect();
    refused(&fx, &words.iter().map(String::as_str).collect::<Vec<_>>(), "VERDICT_NONE", 1, "en");
    fs::remove_file(fx.root.join("AGENTS.md")).unwrap();
    let opts = options(&fx, "published", &["--mode", "review"]);
    let d = digest(&fx, &opts);
    let words: Vec<String> =
        ["apply".to_owned(), d].into_iter().chain(opts.iter().cloned()).chain(["--say".into(), SAY.into()]).collect();
    refused(&fx, &words.iter().map(String::as_str).collect::<Vec<_>>(), "MODE_NOT_AVAILABLE", 1, "en");
}

#[test]
fn refusal_role_forbidden_words_and_agent_environment_at_apply() {
    let fx = product("apply-person");
    let opts = options(&fx, "published", &[]);
    let d = digest(&fx, &opts);
    let with = |say: &str| -> Vec<String> {
        ["apply".to_owned(), d.clone()]
            .into_iter()
            .chain(opts.iter().cloned())
            .chain(["--say".into(), say.into()])
            .collect()
    };
    let words = with(SAY);
    let args: Vec<&str> = words.iter().map(String::as_str).collect();
    // Git signs with a key that is not the plan's, as the plan itself says (its next steps name the plan's).
    fx.git(&["config", "user.signingkey", fx.keys.join("stranger.pub").to_str().unwrap()]);
    let other = digest(&fx, &opts);
    let words_other: Vec<String> = ["apply".to_owned(), other]
        .into_iter()
        .chain(opts.iter().cloned())
        .chain(["--say".into(), SAY.into()])
        .collect();
    refused(&fx, &words_other.iter().map(String::as_str).collect::<Vec<_>>(), "ROLE_FORBIDDEN", 1, "en");
    fx.git(&["config", "user.signingkey", fx.keys.join("ana.pub").to_str().unwrap()]);
    for (say, code) in
        [(" ", "WORDS_MISSING"), (fabrewk_core::cli::get("en", "words.example").unwrap(), "WORDS_COPIED")]
    {
        let words = with(say);
        refused(&fx, &words.iter().map(String::as_str).collect::<Vec<_>>(), code, 1, "en");
    }
    // Inside an agent, nothing is read or written.
    let before = fx.snapshot();
    let ran = fx.run_env(&[&args[..], &["--json"]].concat(), &[("CLAUDECODE", "1")]);
    assert_eq!(ran.status, 1, "{ran:?}");
    assert_eq!(ran.doc()["refusals"][0]["code"], "AGENT_ENVIRONMENT");
    assert_eq!(fx.snapshot(), before);
    // And then, as the person, it applies.
    assert_eq!(fx.run(&args).status, 0);
}

#[test]
fn own_repository_needs_an_identity_git_names_outside_the_product_before_anything_is_written() {
    let fx = product("apply-own-ident");
    fs::remove_file(fx.home.join(".gitconfig")).unwrap();
    let opts = options(&fx, "own-repository", &[]);
    let words: Vec<String> = ["apply".to_owned(), digest(&fx, &opts)]
        .into_iter()
        .chain(opts.iter().cloned())
        .chain(["--say".into(), SAY.into()])
        .collect();
    let e = refused(&fx, &words.iter().map(String::as_str).collect::<Vec<_>>(), "TOOL_FAILED", 1, "en");
    assert!(e["detail"].as_str().unwrap().contains("GIT_AUTHOR_IDENT"), "{e}");
}

/// P10B-1: no file that stands where `apply` would put a temporary one, beside an entry file or the exclude file, is
/// removed or changed: outside the domain `apply` writes its temporary files in the domain's unversioned area.
#[test]
fn p10b_1_a_file_at_a_temporary_name_beside_a_target_is_kept() {
    let fx = product("apply-tmp-names");
    let mine = [".AGENTS.md.tmp", ".CLAUDE.md.tmp", ".git/info/.exclude.tmp"];
    for (i, name) in mine.iter().enumerate() {
        fs::write(fx.root.join(name), format!("mine {i}\n")).unwrap();
    }
    let opts = options(&fx, "unpublished", &[]);
    let ran = apply(&fx, &digest(&fx, &opts), &opts);
    assert_eq!(ran.status, 0, "{ran:?}");
    for (i, name) in mine.iter().enumerate() {
        assert_eq!(fs::read_to_string(fx.root.join(name)).unwrap(), format!("mine {i}\n"), "{name}");
    }
    assert!(fs::read_to_string(fx.root.join(".git/info/exclude")).unwrap().contains("/.brewstillery/\n"));
}

/// P10B-4: the domain's unversioned area reached through a link is not followed: `apply` refuses before writing, and
/// nothing outside the repository changes.
#[test]
fn p10b_4_a_link_at_the_unversioned_area_s_temporary_directory_is_not_followed() {
    let fx = product("apply-tmp-link");
    let outside = fx.root.parent().unwrap().join("outside");
    fs::create_dir_all(&outside).unwrap();
    fs::write(outside.join(".AGENTS.md.tmp"), "mine\n").unwrap();
    fs::create_dir_all(fx.domain.join("local")).unwrap();
    std::os::unix::fs::symlink(&outside, fx.domain.join("local/tmp")).unwrap();
    let opts = options(&fx, "unpublished", &[]);
    let ran = apply(&fx, &digest(&fx, &opts), &opts);
    assert_eq!(ran.status, 1, "{ran:?}");
    assert_eq!(ran.doc()["refusals"][0]["code"], "RECORD_BROKEN", "{ran:?}");
    let names: Vec<_> = fs::read_dir(&outside).unwrap().map(|e| e.unwrap().file_name()).collect();
    assert_eq!(names, [".AGENTS.md.tmp"]);
    assert_eq!(fs::read_to_string(outside.join(".AGENTS.md.tmp")).unwrap(), "mine\n");
    assert!(!fx.domain.join("data").exists() && !fx.root.join("AGENTS.md").exists());
}

/// P10B-5 and P10B-6 read so: a repository whose Git directory lies outside its working tree (`.git` a `gitdir:`
/// file) gets a plan from `assess`, and `apply`, as every write command since part 7b, refuses it at the main working
/// tree's check (`NOT_A_REPOSITORY`, `git worktree list` naming the Git directory), before the lock and with nothing
/// written, the exclude file of that Git directory included.
#[test]
fn a_git_directory_apart_from_its_working_tree_is_refused_with_nothing_written() {
    let fx = product("apply-gitdir-apart");
    let gitdir = fx.root.parent().unwrap().join("gitdir");
    fs::rename(fx.root.join(".git"), &gitdir).unwrap();
    fs::write(fx.root.join(".git"), format!("gitdir: {}\n", gitdir.display())).unwrap();
    let opts = options(&fx, "unpublished", &[]);
    let exclude = fs::read(gitdir.join("info/exclude")).unwrap();
    let words: Vec<String> =
        ["apply".to_owned(), digest(&fx, &opts)].into_iter().chain(opts).chain(["--say".into(), SAY.into()]).collect();
    refused(&fx, &words.iter().map(String::as_str).collect::<Vec<_>>(), "NOT_A_REPOSITORY", 1, "en");
    assert_eq!(fs::read(gitdir.join("info/exclude")).unwrap(), exclude);
}
