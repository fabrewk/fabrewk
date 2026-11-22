//! A repository the bootstrap's import applies to (part 11a, D-113): its domain holds `bootstrap/` and no record, its
//! other files in their production form with a fixture maestro's key (F-4).

use super::*;
use std::path::Path;

/// This repository's `bootstrap/` as its `HEAD` commits it: each file's path under it and its bytes.
pub fn committed_bootstrap() -> Vec<(String, Vec<u8>)> {
    let repo = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let git = |args: &[&str]| {
        let out = std::process::Command::new("git").arg("-C").arg(repo).args(args).output().unwrap();
        assert!(out.status.success(), "git {args:?}");
        out.stdout
    };
    let names = git(&["ls-tree", "-r", "-z", "--name-only", "HEAD", "--", ".brewstillery/bootstrap"]);
    let names: Vec<String> =
        names.split(|b| *b == 0).filter(|n| !n.is_empty()).map(|n| String::from_utf8(n.to_vec()).unwrap()).collect();
    names
        .into_iter()
        .map(|n| (n[".brewstillery/bootstrap/".len()..].to_owned(), git(&["cat-file", "blob", &format!("HEAD:{n}")])))
        .collect()
}

/// A `decisions.md` of two entries, the second holding a fenced `## ` line, and a `plan.md`.
pub const MINI: &str = "# Decisions\n\nThe opening.\n\n## D-001 — The first\n\n- Status: decided\n- Decided: 2026-11-21\n- Decision: one\n  and two\n- Other: kept in the text alone\n\n## R-001 — A session\n\n- Status: recorded\n\n```text\n## D-009 — fenced\n- Status: fenced\n```\n";

/// The project file in its production form (point 8.2), with the fixture's key.
pub fn project(keys: &Path) -> String {
    format!(
        "[project]\nname = \"Fabrewk\"\n\n[maintainers.xel]\nrole = \"maestro\"\nkey = \"{}\"\n\n[languages]\ncanonicalDocumentationLanguage = \"en\"\ndocumentationLanguages = [\"en\", \"es-MX\"]\n\n[tools.claude]\nadapter = \"claude-code\"\nproviders = [\"anthropic\"]\n\n[providers.anthropic]\nallowed = true\n",
        public(keys, "xel")
    )
}

/// A repository whose domain holds `bootstrap/` (this repository's when `real`, else [`MINI`]) and no record, its other
/// files in their production form with a fixture maestro's key, committed; and `assess`'s options for it.
pub fn importable(name: &str, real: bool) -> (Fx, Vec<String>) {
    let base = tmp(name);
    let (root, home, keys) = (base.join("repo"), base.join("home"), base.join("keys"));
    let fx = Fx { domain: root.join(".brewstillery"), root, home, keys, path: None };
    for d in [&fx.domain.join("bootstrap"), &fx.domain.join("xel"), &fx.home, &fx.keys] {
        std::fs::create_dir_all(d).unwrap();
    }
    for who in ["xel", "stranger"] {
        let key = fx.keys.join(who);
        fx.program("ssh-keygen", &["-q", "-t", "ed25519", "-N", "", "-C", who, "-f", key.to_str().unwrap()]);
    }
    let files = if real {
        committed_bootstrap()
    } else {
        vec![("decisions.md".into(), MINI.into()), ("plan.md".into(), b"# Plan\n".to_vec())]
    };
    for (path, bytes) in files {
        std::fs::write(fx.domain.join("bootstrap").join(path), bytes).unwrap();
    }
    let write = |rel: &str, text: &str| std::fs::write(fx.root.join(rel), text).unwrap();
    write(".brewstillery/settings.toml", &project(&fx.keys));
    write(".brewstillery/xel/settings.toml", PERSONAL);
    write(".brewstillery/xel/fermento.md", &fermento(INPUT));
    write(".brewstillery/ALAMBIQUE.md", "# Fabrewk\n\n## 1. What it must achieve\n\nTo build itself.\n");
    write(".brewstillery/STATUS.md", "# Status, written by hand\n");
    write("AGENTS.md", "# Rules\n\nThe project's rules.\n");
    write("CLAUDE.md", "@AGENTS.md\n");
    write("README.md", "fixture\n");
    write("LICENSE", "MIT\n");
    std::fs::write(fx.home.join(".gitconfig"), "[user]\n\tname = Fixture\n\temail = fixture@example.invalid\n")
        .unwrap();
    fx.git(&["init", "-q", "-b", "main"]);
    fx.git(&["config", "commit.gpgsign", "false"]);
    fx.git(&["config", "gpg.format", "ssh"]);
    fx.git(&["config", "user.signingkey", fx.keys.join("xel.pub").to_str().unwrap()]);
    fx.commit("bootstrap");
    let key = fx.keys.join("xel.pub").to_str().unwrap().to_owned();
    let options = ["--alias", "xel", "--key", &key, "--publication", "published", "--lang", "en"];
    (fx, options.map(String::from).into())
}
