//! The installation (ring `install`, kind `Installer`; D-097, part 10a): what `assess` reads, a closed list (REQ-INS-1,
//! point 3 of "Installation, as phase 10's opening fixed it"), the verdict and its reasons (point 4), and the plan
//! (`fabrewk.plan/1`, point 5): every file with its action and exact content, the first alambique among them
//! (REQ-INS-8), and its digest. `assess` uses no network and writes nothing, not a file, not a cache, not a Git lock.
//! Every human word comes from the core's catalogue.

mod effects;
mod files;

use fabrewk_core::cli::{catalogue, say};
use fabrewk_core::install::{Asked, Installer, git, main_worktree};
use fabrewk_core::record::{Repo, encode, record_branch, sha256_hex};
use fabrewk_core::settings::{Level, is_alias, key_blob};
use fabrewk_core::{Code, Error};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The installer, as the binary wires it.
pub const INSTALLER: Installer = Installer { assess };

/// The domain's directory.
const DOMAIN: &str = ".brewstillery";

/// A file over this size is reported by name and size, and not read (point 3).
const MIB: u64 = 1 << 20;

/// Another harness, by the names it leaves in a repository (point 3).
const HARNESSES: &[&str] = &[
    ".claude",
    ".codex",
    ".cursor",
    ".github/copilot-instructions.md",
    "GEMINI.md",
    ".aider.conf.yml",
    ".clinerules",
    ".windsurfrules",
];

/// The catalogued tools' settings in the repository and in the home (point 3); Codex's are TOML.
const SETTINGS: [&str; 4] = [".claude/settings.json", ".claude/settings.local.json", ".mcp.json", ".codex/config.toml"];
const HOME_SETTINGS: [&str; 2] = [".claude/settings.json", ".codex/config.toml"];

/// What a Git directory holds while an operation is in progress, and the operation.
const OPERATIONS: [(&str, &str); 6] = [
    ("MERGE_HEAD", "merge"),
    ("rebase-merge", "rebase"),
    ("rebase-apply", "rebase"),
    ("CHERRY_PICK_HEAD", "cherry-pick"),
    ("REVERT_HEAD", "revert"),
    ("BISECT_LOG", "bisect"),
];

/// The entry files, in the product's working tree (REQ-DOM-6).
const ENTRIES: [&str; 2] = ["AGENTS.md", "CLAUDE.md"];

fn refuse(code: Code, path: Option<&Path>, detail: impl Into<String>) -> Error {
    Error::Refused { code, path: path.map(Path::to_path_buf), detail: detail.into() }
}

/// What the entry at `rel` under `base` is, no link followed on the way: `absent`, `file`, `directory`, `link` (the
/// path or one of its directories is one), `other` or `unreadable`.
fn kind(base: &Path, rel: &str) -> &'static str {
    let mut path = base.to_path_buf();
    let parts: Vec<&str> = rel.split('/').collect();
    for (i, part) in parts.iter().enumerate() {
        path.push(part);
        let last = i + 1 == parts.len();
        match std::fs::symlink_metadata(&path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return "absent",
            Err(_) => return "unreadable",
            Ok(m) if m.file_type().is_symlink() => return "link",
            Ok(m) if !last && !m.is_dir() => return "absent",
            Ok(m) if !last || m.is_dir() => continue,
            Ok(m) if m.is_file() => return "file",
            Ok(_) => return "other",
        }
    }
    "directory"
}

/// Every input read, by the path the plan shows (point 5).
#[derive(Default)]
struct Reading {
    inputs: BTreeMap<String, Value>,
}

impl Reading {
    /// Records `rel` under `base`, shown as `shown`, by kind, and gives a regular file's bytes, read when it holds at
    /// most 1 MiB; a larger one is recorded by its size only. In the repository no link is followed; in the home
    /// (`shown` as `~/…`), the person's own, a link to a regular file is read, as dotfiles often are links.
    fn read(&mut self, base: &Path, rel: &str, shown: &str) -> Option<Vec<u8>> {
        let mut entry = json!({"path": shown, "kind": kind(base, rel)});
        let path = base.join(rel);
        let followed = shown.starts_with("~/") && std::fs::metadata(&path).is_ok_and(|m| m.is_file());
        let mut out = None;
        if entry["kind"] == "file" || (entry["kind"] == "link" && followed) {
            match std::fs::metadata(&path).map(|m| m.len()) {
                Ok(n) if n > MIB => entry["bytes"] = n.into(),
                _ => match std::fs::read(&path) {
                    Ok(b) => {
                        entry["digest"] = sha256_hex(&b).into();
                        out = Some(b);
                    }
                    Err(_) => entry["kind"] = "unreadable".into(),
                },
            }
        }
        self.inputs.insert(shown.to_owned(), entry);
        out
    }

    /// Records `rel` under `root` by kind alone, unless it was read.
    fn note(&mut self, root: &Path, rel: &str) -> &'static str {
        let k = kind(root, rel);
        self.inputs.entry(rel.to_owned()).or_insert_with(|| json!({"path": rel, "kind": k}));
        k
    }
}

fn reason(key: &str, blocks: bool, path: Option<&str>, detail: Option<&str>) -> Value {
    let mut r = json!({"reason": key, "blocks": blocks});
    for (field, v) in [("path", path), ("detail", detail)] {
        if let Some(v) = v {
            r[field] = v.into();
        }
    }
    r
}

/// Git as `assess` runs it: the core's way, and with every filter driver the configuration names turned off, so that
/// no clean or process filter runs when Git reads a changed file (P10A-1): a driver with no command passes the bytes
/// through, and one not required fails nothing.
struct Git {
    root: PathBuf,
    off: Vec<String>,
}

impl Git {
    fn new(root: &Path) -> Result<Git, Error> {
        let (_, names) = git(root, &["config", "--name-only", "-z", "--get-regexp", r"^filter\."], b"", &[])?;
        let mut off = Vec::new();
        for name in names.split(|b| *b == 0).filter_map(|n| std::str::from_utf8(n).ok()) {
            if let Some((driver, _)) = name.strip_prefix("filter.").and_then(|n| n.rsplit_once('.')) {
                for k in ["clean=", "smudge=", "process=", "required=false"] {
                    off.extend(["-c".to_owned(), format!("filter.{driver}.{k}")]);
                }
            }
        }
        Ok(Git { root: root.to_path_buf(), off })
    }

    fn run(&self, args: &[&str]) -> Result<(Option<i32>, Vec<u8>), Error> {
        self.with(args, b"")
    }

    fn with(&self, args: &[&str], input: &[u8]) -> Result<(Option<i32>, Vec<u8>), Error> {
        git(
            &self.root,
            &self.off.iter().map(String::as_str).chain(args.iter().copied()).collect::<Vec<_>>(),
            input,
            &[],
        )
    }

    /// Git's output, or `TOOL_FAILED` naming the command and its exit status; Git's text never reaches the words.
    fn ok(&self, args: &[&str]) -> Result<Vec<u8>, Error> {
        git_ok(&self.root, &self.run(args)?, args)
    }

    /// The object ids, by path, of what `args` lists (`ls-files -s`, `ls-tree`: the id before the tab, the path after);
    /// an entry in conflict, at a stage other than 0, has no id.
    fn ids(&self, args: &[&str]) -> Result<BTreeMap<String, Option<String>>, Error> {
        let out = self.ok(args)?;
        let entries = out.split(|b| *b == 0).filter_map(|e| std::str::from_utf8(e).ok()?.split_once('\t'));
        Ok(entries
            .map(|(meta, path)| {
                let words: Vec<&str> = meta.split(' ').collect();
                let conflict = args[0] == "ls-files" && words.get(2) != Some(&"0");
                (
                    path.to_owned(),
                    words.get(1 + usize::from(args[0] == "ls-tree")).filter(|_| !conflict).map(|w| w.to_string()),
                )
            })
            .collect())
    }
}

/// Refuses `PRIVATE_FOLDER_INCLUDED` when Git's configuration includes a file of the private folder, which every Git
/// run in the repository would open (D-100). Git loads its configuration, includes followed, as soon as it starts in a
/// repository, so each file is read apart, by a Git started outside any repository with no configuration of its own,
/// before any other Git: the repository's, the person's and the system's, and the `-c` values the environment carries.
/// An include inside an included file is not seen: a declared residual.
fn includes(root: &Path) -> Result<(), Error> {
    let var = |k: &str| std::env::var_os(k).filter(|v| !v.is_empty()).map(PathBuf::from);
    let Ok(private) = fabrewk_core::machine::private_folder(|k| std::env::var_os(k)) else { return Ok(()) };
    let inside = |p: &Path| {
        let p = p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
        p.starts_with(&private) || private.canonicalize().is_ok_and(|f| p.starts_with(f))
    };
    let found = |p: &Path| refuse(Code::PrivateFolderIncluded, Some(p), "an include of Git's configuration");
    // The repository's Git directory: `.git`, or the one its `gitdir:` file names.
    let dot = root.join(".git");
    let gitdir = match std::fs::read_to_string(&dot)
        .ok()
        .and_then(|t| t.strip_prefix("gitdir: ").map(|d| d.trim().to_owned()))
    {
        Some(d) => root.join(d),
        None => dot,
    };
    let home = var("HOME");
    let xdg = var("XDG_CONFIG_HOME").or_else(|| home.as_ref().map(|h| h.join(".config")));
    let mut files = vec![gitdir.join("config"), gitdir.join("config.worktree")];
    match var("GIT_CONFIG_GLOBAL") {
        Some(g) => files.push(g),
        None => files.extend(xdg.map(|x| x.join("git/config")).into_iter().chain(home.map(|h| h.join(".gitconfig")))),
    }
    if var("GIT_CONFIG_NOSYSTEM").is_none() {
        files.push(var("GIT_CONFIG_SYSTEM").unwrap_or_else(|| PathBuf::from("/etc/gitconfig")));
    }
    let alone = [
        ("GIT_CONFIG_GLOBAL", "/dev/null"),
        ("GIT_CONFIG_NOSYSTEM", "1"),
        ("GIT_CONFIG_COUNT", "0"),
        ("GIT_CONFIG_PARAMETERS", ""),
    ];
    let regexp = r"^include(if\..*)?\.path$";
    for file in files.iter().filter(|f| f.is_file()) {
        let Some(name) = file.to_str() else { continue };
        let args = ["config", "--no-includes", "--file", name, "--type=path", "-z", "--get-regexp", regexp];
        let (_, out) = git(Path::new("/"), &args, b"", &alone)?;
        for entry in out.split(|b| *b == 0).filter_map(|e| std::str::from_utf8(e).ok()) {
            // A relative include is relative to the file that names it.
            if let Some((_, value)) = entry.split_once('\n') {
                let target = file.parent().unwrap_or(root).join(value);
                if inside(&target) {
                    return Err(found(&target));
                }
            }
        }
    }
    // The `-c` values Git takes from the environment.
    let n: usize = std::env::var("GIT_CONFIG_COUNT").ok().and_then(|n| n.parse().ok()).unwrap_or(0);
    let values = (0..n).filter(|i| {
        std::env::var(format!("GIT_CONFIG_KEY_{i}")).is_ok_and(|k| k.to_ascii_lowercase().starts_with("include"))
    });
    let mut values: Vec<PathBuf> = values.filter_map(|i| var(&format!("GIT_CONFIG_VALUE_{i}"))).collect();
    values
        .extend(std::env::var("GIT_CONFIG_PARAMETERS").ok().filter(|p| p.contains("include")).map(|_| private.clone()));
    match values.iter().find(|v| inside(&root.join(v))) {
        Some(v) => Err(found(v)),
        None => Ok(()),
    }
}

/// Whether tracked files have, or may have, changes (D-100): staged changes, the index against `HEAD` by object id, or
/// a tracked file whose size or modification time differs from what the index recorded; no file's content is read.
fn changes(root: &Path, g: &Git, head: bool) -> Result<bool, Error> {
    let staged = match head {
        true => match g.run(&["diff-index", "--cached", "--quiet", "--no-renames", "HEAD", "--"])? {
            (Some(0), _) => false,
            (Some(1), _) => true,
            other => return git_ok(root, &other, &["diff-index", "--cached"]).map(|_| true),
        },
        false => !g.ok(&["ls-files", "-z"])?.is_empty(),
    };
    // `<mode> <id> <stage>\t<path>`, then, after its zero byte, the lines of what the index recorded of it.
    let listing = g.ok(&["ls-files", "-z", "-s", "--debug"])?;
    let mut tokens = listing.split(|b| *b == 0);
    let mut entry = tokens.next().map(<[u8]>::to_vec);
    for t in tokens {
        let at = t.iter().rposition(|b| *b == b'\n').map_or(0, |i| i + 1);
        if let Some(e) = entry.take().filter(|e| !e.starts_with(b"160000 "))
            && differs(root, &e, &t[..at])
        {
            return Ok(true);
        }
        entry = Some(t[at..].to_vec()).filter(|e| !e.is_empty());
    }
    Ok(staged)
}

/// Whether the file of the index entry `entry` differs, by size or modification time, from what `recorded` says.
fn differs(root: &Path, entry: &[u8], recorded: &[u8]) -> bool {
    use std::os::unix::{ffi::OsStrExt, fs::MetadataExt};
    let path = entry.iter().position(|b| *b == b'\t').map_or(entry, |i| &entry[i + 1..]);
    let Ok(meta) = std::fs::symlink_metadata(root.join(std::ffi::OsStr::from_bytes(path))) else { return true };
    let text = String::from_utf8_lossy(recorded);
    let field = |name: &str| text.lines().find_map(|l| l.trim_start().strip_prefix(name)?.split_whitespace().next());
    let mtime = field("mtime: ").and_then(|m| m.split_once(':')).map(|(s, n)| (s.parse().ok(), n.parse().ok()));
    let size = field("size: ").and_then(|s| s.parse::<u64>().ok());
    // An index that kept no nanoseconds recorded 0 for them.
    let time = mtime.is_some_and(|(s, n)| s == Some(meta.mtime()) && (n == Some(0) || n == Some(meta.mtime_nsec())));
    size != Some(meta.len() & 0xffff_ffff) || !time
}

fn git_ok(root: &Path, ran: &(Option<i32>, Vec<u8>), args: &[&str]) -> Result<Vec<u8>, Error> {
    match ran.clone() {
        (Some(0), out) => Ok(out),
        (status, _) => {
            let status = status.map_or_else(|| "a signal".to_owned(), |n| format!("exit status {n}"));
            Err(refuse(Code::ToolFailed, Some(root), format!("git {} failed with {status}", args[..2].join(" "))))
        }
    }
}

fn line(bytes: Vec<u8>) -> String {
    String::from_utf8_lossy(&bytes).trim_end_matches('\n').to_owned()
}

/// Refuses `NOT_A_REPOSITORY` unless `root` is the top of the main working tree of a repository that is not bare.
fn repository(root: &Path) -> Result<(), Error> {
    let bare = git(root, &["rev-parse", "--is-bare-repository"], b"", &[])?;
    if bare != (Some(0), b"false\n".to_vec()) {
        return Err(refuse(Code::NotARepository, Some(root), "not the working tree of a repository that is not bare"));
    }
    main_worktree(root)
}

/// What `.brewstillery/data/` holds: whether an event (a file under `events/` that is neither a signature nor a
/// record's temporary file), and whether anything but directories, signatures and such temporary files, which is what
/// a killed `apply` leaves before its first event (P10O-10).
fn data(dir: &Path, events: bool) -> (bool, bool) {
    let mut out = (false, false);
    for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let (path, name) = (entry.path(), entry.file_name().to_string_lossy().into_owned());
        // No link is followed, nor is one an event (P10A-2).
        let inner = match entry.file_type() {
            Ok(t) if t.is_dir() => data(&path, events || (dir.ends_with("data") && name == "events")),
            Ok(t) if t.is_file() => {
                let leftover = name.ends_with(".sig") || (name.starts_with('.') && name.ends_with(".tmp"));
                (!leftover && events, !leftover)
            }
            _ => (false, true),
        };
        out = (out.0 || inner.0, out.1 || inner.1);
    }
    out
}

/// The domain's top entries `assess` reads (point 3): all but `local/`, and but a `data/` that holds only what a killed
/// `apply` leaves; none when the domain is not a directory. Gives them, and whether the record holds an event.
fn domain(root: &Path) -> (Vec<String>, bool) {
    let dir = root.join(DOMAIN);
    if kind(root, DOMAIN) != "directory" {
        return (Vec::new(), false);
    }
    let (event, other) = match kind(root, &format!("{DOMAIN}/data")) {
        "directory" => data(&dir.join("data"), false),
        k => (false, k != "absent"),
    };
    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n != "local" && (n != "data" || other))
        .collect();
    names.sort();
    (names, event)
}

/// The answers, as the plan holds them: each option given, the defaults of `--mode` and `--lang`, the tools sorted.
fn answers(asked: &Asked) -> BTreeMap<&'static str, Value> {
    let mut out: BTreeMap<&str, Value> = asked.options.answers.iter().map(|(k, v)| (*k, v.as_str().into())).collect();
    out.entry("mode").or_insert("adopt".into());
    out.entry("lang").or_insert("en".into());
    let mut tools = asked.options.tools.clone();
    tools.sort();
    tools.dedup();
    out.insert("tool", tools.into());
    out
}

/// The commands that give `parts` to `head`, each `<prefix>'<value>'`: one for fish and a POSIX shell alike, or, when a
/// value holds a single quote or a backslash, one for each, named (point 2): fish reads a backslash inside single
/// quotes, a POSIX shell does not.
fn commands(head: &str, parts: &[(String, &str)]) -> Vec<Value> {
    let posix = |v: &str| format!("'{}'", v.replace('\'', "'\\''"));
    let fish = |v: &str| format!("'{}'", v.replace('\\', "\\\\").replace('\'', "\\'"));
    let line = |q: &dyn Fn(&str) -> String| parts.iter().fold(head.to_owned(), |a, (p, v)| format!("{a} {p}{}", q(v)));
    if !parts.iter().any(|(_, v)| v.contains(['\'', '\\'])) {
        return vec![json!({"shell": "any", "command": line(&posix)})];
    }
    vec![json!({"shell": "fish", "command": line(&fish)}), json!({"shell": "posix", "command": line(&posix)})]
}

/// The `--key` file: its one public key, recorded by its digest; or the reason it cannot be the first maestro's key,
/// a file's content never shown nor digested unless it is a public key (point 4).
fn key(root: &Path, given: &str, r: &mut Reading) -> Result<String, &'static str> {
    let path = root.join(given);
    let meta = std::fs::metadata(&path).map_err(|_| "key-absent")?;
    if !meta.is_file() {
        return Err("key-not-a-file");
    }
    if meta.len() > MIB {
        return Err("key-too-large");
    }
    // A key is no maintainer's material: never one in this repository or in any Fabrewk domain (as Git's, D-055).
    let resolved = path.canonicalize().map_err(|_| "key-absent")?;
    if root.canonicalize().is_ok_and(|r| resolved.starts_with(r))
        || resolved.components().any(|c| c.as_os_str() == DOMAIN)
    {
        return Err("key-inside");
    }
    let bytes = std::fs::read(&path).map_err(|_| "key-absent")?;
    let text = std::str::from_utf8(&bytes).map_err(|_| "key-not-public")?.trim();
    if text.contains('\n') || key_blob(text).is_none() {
        return Err("key-not-public");
    }
    r.inputs.insert("--key".into(), json!({"path": "--key", "kind": "file", "digest": sha256_hex(&bytes)}));
    Ok(text.to_owned())
}

/// `assess` (REQ-INS-1, REQ-INS-2): reads point 3's list, gives the verdict, the plan and its digest, and the `apply`
/// command the plan corresponds to.
fn assess(asked: &Asked) -> Result<(Value, String), Vec<Error>> {
    assessed(asked).map_err(|e| vec![e])
}

fn assessed(asked: &Asked) -> Result<(Value, String), Error> {
    let (root, home) = (asked.root, asked.home);
    let answers = answers(asked);
    let a = |k: &str| answers.get(k).and_then(Value::as_str);
    let (tag, mode, publication, alias) =
        (a("lang").unwrap_or("en"), a("mode").unwrap_or("adopt"), a("publication"), a("alias"));
    // The tag is written as given; the words come from the first catalogue it reaches (`es-MX`'s is `es`).
    let lang = catalogue(&tag.to_ascii_lowercase());
    if let Some(alias) = alias.filter(|x| !is_alias(x)) {
        return Err(refuse(Code::AliasInvalid, None, format!("{alias:?} is not an alias, or is a reserved name")));
    }
    includes(root)?;
    repository(root)?;
    let g = Git::new(root)?;
    let mut r = Reading::default();
    let mut reasons = Vec::new();
    // Git's state.
    let repo = Repo { root: root.to_path_buf(), prefix: PathBuf::new() };
    let branch = match record_branch(&repo) {
        Ok(b) => Some(b.trim_start_matches("refs/heads/").to_owned()),
        Err(e) if e.code() == Some(Code::HeadDetached) => {
            reasons.push(reason("detached", true, None, None));
            None
        }
        Err(e) => return Err(e),
    };
    let gitdir = PathBuf::from(line(g.ok(&["rev-parse", "--path-format=absolute", "--git-dir"])?));
    if let Some((_, op)) = OPERATIONS.iter().find(|(f, _)| std::fs::symlink_metadata(gitdir.join(f)).is_ok()) {
        reasons.push(reason("operation", true, None, Some(op)));
    }
    let head = g.run(&["rev-parse", "-q", "--verify", "HEAD^{commit}"])?.0 == Some(0);
    if changes(root, &g, head)? {
        reasons.push(reason("changes", false, None, None));
    }
    // The entry files: in the index and in `HEAD` by object id, and through a filter or not (D-100).
    let index = g.ids(&["ls-files", "-z", "-s", "--", "AGENTS.md", "CLAUDE.md"])?;
    let committed =
        if head { g.ids(&["ls-tree", "-z", "HEAD", "--", "AGENTS.md", "CLAUDE.md"])? } else { BTreeMap::new() };
    let attrs = g.ok(&["check-attr", "-z", "filter", "--", "AGENTS.md", "CLAUDE.md"])?;
    let attrs: Vec<&[u8]> = attrs.split(|b| *b == 0).collect();
    let filtered: Vec<String> = attrs
        .chunks(3)
        .filter(|c| c.len() == 3 && !matches!(c[2], b"unspecified" | b"unset"))
        .map(|c| String::from_utf8_lossy(c[0]).into_owned())
        .collect();
    // The root's entries and the domain's, by name and kind.
    let (domain_names, record) = domain(root);
    let domain_absent = kind(root, DOMAIN) == "directory" && domain_names.is_empty();
    let mut licence = false;
    for entry in std::fs::read_dir(root).map_err(|e| Error::Io { path: root.to_path_buf(), error: e })?.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let upper = name.to_ascii_uppercase();
        if ["LICENSE", "LICENCE", "COPYING"].iter().any(|l| upper.starts_with(l)) || upper == "UNLICENSE" {
            r.read(root, &name, &name);
            licence |= r.inputs[&name]["kind"] == "file";
        }
        if name != DOMAIN || !domain_absent {
            r.note(root, &name);
        }
    }
    for name in &domain_names {
        r.note(root, &format!("{DOMAIN}/{name}"));
    }
    if record {
        reasons.push(reason("record", true, None, None));
    }
    if !licence {
        reasons.push(reason("no-licence", false, None, None));
    }
    // The entry files (point 4).
    let mut entries = BTreeMap::new();
    for name in ENTRIES {
        let bytes = r.read(root, name, name);
        let k = r.inputs[name]["kind"].as_str().unwrap_or_default().to_owned();
        let why = match (&bytes, k.as_str()) {
            (_, "absent") => None,
            (None, "file") => Some("too-large"),
            (None, _) => Some("not-a-file"),
            (Some(b), _) if std::str::from_utf8(b).is_err() => Some("not-utf8"),
            _ => None,
        };
        let why = why.or_else(|| filtered.iter().any(|f| f == name).then_some("filtered"));
        // The working tree's bytes hashed by Git from standard input, with the path's attributes and no filter, so that
        // no index flag hides an edit (P10A-12).
        let worktree = match (&why, &bytes) {
            (None, Some(b)) => Some(line(git_ok(
                root,
                &g.with(&["hash-object", "--stdin", &format!("--path={name}")], b)?,
                &["hash-object", "--stdin"],
            )?)),
            _ => None,
        };
        let changed =
            index.get(name) != committed.get(name) || worktree.as_ref() != index.get(name).and_then(Option::as_ref);
        let why = why.or_else(|| changed.then_some("uncommitted"));
        match why {
            Some(w) => reasons.push(reason(w, true, Some(name), None)),
            None => {
                entries.insert(name, bytes.map(|b| String::from_utf8_lossy(&b).into_owned()));
            }
        }
    }
    for h in HARNESSES {
        if kind(root, h) != "absent" {
            r.note(root, h);
            reasons.push(reason("harness", false, Some(h), None));
        }
    }
    // The side effects in the catalogued tools' settings, in the repository and in the home.
    let mut effects = Vec::new();
    let sources = SETTINGS
        .iter()
        .map(|s| (root, *s, s.to_string()))
        .chain(home.into_iter().flat_map(|h| HOME_SETTINGS.iter().map(move |s| (h, *s, format!("~/{s}")))));
    for (base, rel, shown) in sources.collect::<Vec<_>>() {
        if let Some(bytes) = r.read(base, rel, &shown) {
            effects.extend(effects::scan(&shown, &bytes, rel.ends_with(".toml")));
        } else if r.inputs[&shown]["kind"] == "absent" {
            r.inputs.remove(&shown);
        }
    }
    if !effects.is_empty() {
        reasons.push(reason("effects", false, None, None));
    }
    // The key, and whether Git signs with it.
    let key = a("key").map(|k| key(root, k, &mut r)).transpose().unwrap_or_else(|why| {
        reasons.push(reason(why, false, None, None));
        None
    });
    let ssh = g.run(&["config", "--get", "gpg.format"])? == (Some(0), b"ssh\n".to_vec());
    let named = g.run(&["config", "--type=path", "--get", "user.signingkey"])?.1;
    let named = String::from_utf8_lossy(&named).trim_end_matches('\n').to_owned();
    let mut next = Vec::new();
    let mut signs = false;
    if let (Some(given), Some(k)) = (a("key"), &key) {
        // Git's key, by its literal or by its path, the file itself never opened (P10A-3): the `--key` file, or the
        // private key beside it.
        let same = |p: PathBuf| p.canonicalize().ok() == root.join(given).canonicalize().ok();
        let held = match named.strip_prefix("key::") {
            Some(literal) => key_blob(literal.trim()) == key_blob(k),
            None => !named.is_empty() && (same(root.join(&named)) || same(root.join(format!("{named}.pub")))),
        };
        signs = ssh && held;
        if !ssh {
            next.extend(commands("git config gpg.format", &[(String::new(), "ssh")]));
        }
        if !held {
            next.extend(commands("git config user.signingkey", &[(String::new(), given)]));
        }
    }
    // The plan's files (point 5), for `adopt`.
    let mut files = Vec::new();
    let mut tools = Vec::new();
    for t in answers["tool"].as_array().into_iter().flatten().filter_map(Value::as_str) {
        if let Some(adapter) = asked.adapters.iter().find(|x| x.program.executable == t) {
            tools.push((t, adapter.program.adapter.unwrap_or_default(), adapter.provider));
        }
    }
    let name = root.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    if mode == "adopt" {
        let project = files::project(&name, alias, key.as_deref(), tag, &tools);
        if let (Some(_), Some(_)) = (alias, &key) {
            let path = Path::new(".brewstillery/settings.toml");
            if let Err(e) = fabrewk_core::settings::parse(path, &project, Level::Project) {
                reasons.push(reason("settings-invalid", false, None, e.first().map(Error::detail).as_deref()));
            }
        }
        let mut create = vec![
            (format!("{DOMAIN}/settings.toml"), Some(project)),
            (format!("{DOMAIN}/.gitignore"), Some(files::GITIGNORE.to_owned())),
        ];
        if let Some(alias) = alias {
            create.push((format!("{DOMAIN}/{alias}/settings.toml"), Some(files::personal(alias, tag))));
            create.push((format!("{DOMAIN}/{alias}/fermento.md"), Some(files::fermento(alias, lang))));
        }
        let alambique = format!("{DOMAIN}/ALAMBIQUE.md");
        match r.read(root, &alambique, &alambique) {
            Some(bytes) => files.push(json!({"path": alambique, "action": "keep", "digest": sha256_hex(&bytes)})),
            None if r.inputs[&alambique]["kind"] == "absent" => {
                let answered = [a("purpose"), a("audience"), a("excluded")];
                create.push((alambique, Some(files::alambique(&name, lang, answered))));
            }
            None => reasons.push(reason("not-a-file", true, Some(&alambique), None)),
        }
        if entries.get("CLAUDE.md") == Some(&None) {
            create.push(("CLAUDE.md".into(), Some(files::CLAUDE.to_owned())));
        }
        for view in ["STATUS.md", "RULES.md"] {
            create.push((format!("{DOMAIN}/{view}"), None));
        }
        if publication == Some("own-repository") {
            create.push((format!("{DOMAIN}/.git"), None));
        }
        // A target to create present in any form, or under something that is not a directory, blocks (point 4).
        let mut dirs = vec![DOMAIN.to_owned()];
        dirs.extend(alias.map(|x| format!("{DOMAIN}/{x}")));
        for d in dirs {
            if !matches!(kind(root, &d), "absent" | "directory") {
                reasons.push(reason("present", true, Some(&d), None));
            }
        }
        for (path, content) in create {
            r.read(root, &path, &path);
            if r.inputs[&path]["kind"] != "absent" {
                reasons.push(reason("present", true, Some(&path), None));
            }
            let action = match (&content, path.ends_with(".git")) {
                (Some(_), _) => "create",
                (None, true) => "repository",
                (None, false) => "view",
            };
            let mut f = json!({"path": path, "action": action});
            match content {
                Some(c) => f["content"] = c.into(),
                None if action == "repository" => f["branch"] = branch.clone().into(),
                None => {}
            }
            files.push(f);
        }
        let block = files::block(lang);
        for (name, text) in &entries {
            let Some(text) = text.as_deref().or((*name == "AGENTS.md").then_some("")) else { continue };
            match files::with_block(text, &block) {
                Some((after, replaced)) => files.push(
                    json!({"path": name, "action": "block", "content": after, "lines": block, "replaced": replaced}),
                ),
                None => reasons.push(reason("block-damaged", true, Some(name), None)),
            }
        }
        if matches!(publication, Some("own-repository" | "unpublished")) {
            let exclude = line(g.ok(&["rev-parse", "--git-path", "info/exclude"])?);
            let present = r
                .read(root, &exclude, &exclude)
                .is_some_and(|b| b.split(|c| *c == b'\n').any(|l| l == b"/.brewstillery/"));
            if !matches!(r.inputs[&exclude]["kind"].as_str(), Some("absent" | "file")) {
                reasons.push(reason("not-a-file", true, Some(&exclude), None));
            }
            files.push(json!({"path": exclude, "action": "exclude", "line": "/.brewstillery/", "present": present}));
        }
        files.sort_by(|x, y| x["path"].as_str().cmp(&y["path"].as_str()));
        next.push(json!({"text": say(lang, "install.next-models", &[])}));
    }
    let blocked = reasons.iter().any(|x| x["blocks"] == true);
    let verdict = if blocked { "none" } else { mode };
    if verdict != "adopt" {
        files.clear();
    }
    let incomplete: Vec<&str> = ["publication", "alias", "key"].into_iter().filter(|k| a(k).is_none()).collect();
    let mut plan = json!({
        "schema": "fabrewk.plan/1", "verdict": verdict, "reasons": reasons, "incomplete": incomplete,
        "answers": answers, "git": {"branch": branch, "signs": signs},
        "inputs": r.inputs.into_values().collect::<Vec<_>>(), "effects": effects, "next": next, "files": files,
    });
    if publication == Some("unpublished") {
        plan["warning"] = json!({"title": say(lang, "warning.title", &[]), "text": say(lang, "warning.text", &[])});
    }
    let bytes = encode(&plan).ok_or_else(|| refuse(Code::CheckUnknown, None, "the plan cannot be encoded"))?;
    let digest = sha256_hex(&bytes);
    let ready = verdict == "adopt"
        && incomplete.is_empty()
        && key.is_some()
        && !plan["reasons"].as_array().into_iter().flatten().any(|x| x["reason"] == "settings-invalid");
    let apply = if ready { apply_commands(&digest, &answers, lang) } else { Vec::new() };
    let text = render(lang, &plan, &digest, &apply);
    Ok((json!({"schema": "fabrewk.assessment/1", "digest": digest, "plan": plan, "apply": apply}), text))
}

/// The `apply` command the plan corresponds to (point 2): the same options, each value quoted, and the place of the
/// person's words.
fn apply_commands(digest: &str, answers: &BTreeMap<&str, Value>, lang: &str) -> Vec<Value> {
    let words = format!("<{}>", say(lang, "assess.words", &[]));
    let mut parts: Vec<(String, &str)> = Vec::new();
    for k in ["mode", "publication", "alias", "key", "tool", "lang", "purpose", "audience", "excluded"] {
        let values: Vec<&str> = match &answers.get(k) {
            Some(Value::Array(a)) => a.iter().filter_map(Value::as_str).collect(),
            Some(v) => v.as_str().into_iter().collect(),
            None => Vec::new(),
        };
        parts.extend(values.into_iter().map(|v| (format!("--{k}="), v)));
    }
    parts.push(("--say=".into(), &words));
    commands(&format!("fabrewk apply {digest}"), &parts)
}

/// The plan as a person reads it, in `lang`.
fn render(lang: &str, plan: &Value, digest: &str, apply: &[Value]) -> String {
    let t = |k: &str, p: &[(&str, &str)]| say(lang, &format!("assess.{k}"), p);
    let s = |v: &Value| v.as_str().unwrap_or_default().to_owned();
    let list = |v: &Value| v.as_array().cloned().unwrap_or_default();
    let verdict = t(&format!("verdict-{}", s(&plan["verdict"])), &[]);
    let mut out = format!("{}\n\n{}:\n", t("verdict", &[("verdict", &verdict)]), t("reasons", &[]));
    for x in list(&plan["reasons"]) {
        let text = t(&format!("reason.{}", s(&x["reason"])), &[("path", &s(&x["path"])), ("detail", &s(&x["detail"]))]);
        let blocks = if x["blocks"] == true { format!(" ({})", t("blocks", &[])) } else { String::new() };
        out += &format!("  - {text}{blocks}\n");
    }
    if list(&plan["reasons"]).is_empty() {
        out += &format!("  {}\n", t("no-reasons", &[]));
    }
    let incomplete: Vec<String> = list(&plan["incomplete"]).iter().map(|k| format!("--{}", s(k))).collect();
    if !incomplete.is_empty() {
        out += &format!("\n{}\n", t("incomplete", &[("names", &incomplete.join(", "))]));
    }
    if !list(&plan["effects"]).is_empty() {
        out += &format!("\n{}:\n", t("effects", &[]));
        for e in list(&plan["effects"]) {
            let what = t(&format!("effect.{}", s(&e["kind"])), &[]);
            let count = e["count"].as_u64().map_or(String::new(), |n| format!(" ({n})"));
            out += &format!("  {} {}: {what}{count}\n", s(&e["file"]), s(&e["key"]));
        }
    }
    if !list(&plan["files"]).is_empty() {
        out += &format!("\n{}:\n", t("files", &[]));
    }
    for f in list(&plan["files"]) {
        let action = s(&f["action"]);
        let params = [("path", s(&f["path"])), ("branch", s(&f["branch"]))];
        let params: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let present = if f["present"] == true { "-present" } else { "" };
        out += &format!("  {}\n", t(&format!("action.{action}{present}"), &params));
        let shown = match action.as_str() {
            "create" => s(&f["content"]),
            "block" => s(&f["lines"]),
            _ => String::new(),
        };
        out += &shown.lines().map(|l| format!("      {l}\n")).collect::<String>();
    }
    if let Some(w) = plan.get("warning") {
        out += &format!("\n{}\n\n{}\n", s(&w["title"]), s(&w["text"]).trim_end());
    }
    if !list(&plan["next"]).is_empty() {
        out += &format!("\n{}:\n", say(lang, "ui.next-steps", &[]));
    }
    for n in list(&plan["next"]) {
        out += &shell_line(lang, &n, &s(&n["text"]));
    }
    out += &format!("\n{}\n", t("digest", &[("digest", digest)]));
    let why = match s(&plan["verdict"]).as_str() {
        _ if !apply.is_empty() => "apply",
        "none" => "cannot-none",
        "review" => "cannot-review",
        _ => "cannot-incomplete",
    };
    out += &format!("{}\n", t(why, &[]));
    for c in apply {
        out += &shell_line(lang, c, "");
    }
    out
}

/// One next step: a sentence, or a command, named for its shell when it serves only one.
fn shell_line(lang: &str, v: &Value, text: &str) -> String {
    let shell = v["shell"].as_str().unwrap_or("any");
    let label = if shell == "any" {
        String::new()
    } else {
        format!("  {}:\n", say(lang, &format!("assess.shell-{shell}"), &[]))
    };
    let command = v["command"].as_str().unwrap_or(text);
    format!("{label}  {command}\n")
}
