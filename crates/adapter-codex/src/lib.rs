//! The Codex adapter (ring `ToolAdapter`; part 9d): how Codex is detected, confined, launched per role, preflighted
//! and read. The release is the directory its installer keeps per version under `releases/`, holding the executable in
//! `bin/` and its helpers beside it. The tool runs headless (`exec`), with its discovery and its extras off as far as
//! its options allow, its own sandbox for every command the model runs under a permission profile of its role, with no
//! network and its credential's folder unreadable; its stream is read by one grammar, which the audit of a reviewer's
//! examinations shares. Its machine rule (6.38): it is never launched where another repository lies.

use fabrewk_core::adapter::{self, Adapter, Launch, Report, Role};
use fabrewk_core::detect::Program;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

mod stream;

pub use stream::{count, words};

/// The adapter, as the binary wires it.
pub const ADAPTER: Adapter = Adapter {
    program: Program {
        adapter: Some("codex"),
        executable: "codex",
        version: &["--version"],
        tokens: true,
        skills: &[".codex/skills", ".agents/skills"],
    },
    provider: "openai",
    // Set for the commands it runs: its sandbox's (read from `codex-cli` 0.166.0's binary; the second seen inside it)
    // and its thread's, seen inside it whatever its network.
    markers: &["CODEX_SANDBOX", "CODEX_SANDBOX_NETWORK_DISABLED", "CODEX_THREAD_ID"],
    protected: &["AGENTS.md", ".codex", ".agents"],
    residual: &["tool-credential", "audit-limit", "tool-flags"],
    machine: &[".git"],
    credential: ".codex/auth.json",
    expiry,
    env: &[],
    release,
    command,
    preflight,
    read,
};

/// How long Codex keeps a ChatGPT login before it renews it, rewriting its credential: about eight days, by its
/// documentation (D-081).
const RENEWAL: u64 = 8 * 86_400;

/// When Codex would renew its login: its `last_refresh` and [`RENEWAL`] (D-081). Only that field is read.
fn expiry(bytes: &[u8]) -> Option<u64> {
    let auth = fabrewk_core::adapter::json(std::str::from_utf8(bytes).ok()?)?;
    fabrewk_core::record::seconds_of(auth.get("last_refresh")?.as_str()?)?.checked_add(RENEWAL)
}

/// The release that holds `bin/codex`: its installer keeps one directory per version under `releases/`, with the
/// helpers its sandbox and its searches use beside `bin/`; an executable anywhere else has no release this adapter can
/// tell, so none is bound.
fn release(exe: &Path) -> Option<PathBuf> {
    let bin = exe.parent().filter(|d| d.file_name().is_some_and(|n| n == "bin"))?;
    let release = bin.parent()?;
    release.parent().filter(|d| d.file_name().is_some_and(|n| n == "releases"))?;
    Some(release.to_path_buf())
}

/// The permission profile of each role for the model's commands, under the tool's own sandbox: a builder writes its
/// copy, a reviewer nothing; neither has network, and neither reads the folder of the credential (3.6).
pub fn profile(role: Role) -> [&'static str; 8] {
    let extends = match role {
        Role::Build => r#"permissions.fabrewk.extends=":workspace""#,
        Role::Review => r#"permissions.fabrewk.extends=":read-only""#,
    };
    [
        "-c",
        extends,
        "-c",
        r#"permissions.fabrewk.filesystem={"/home/agent/.codex"="none"}"#,
        "-c",
        "permissions.fabrewk.network.enabled=false",
        "-c",
        r#"default_permissions="fabrewk""#,
    ]
}

/// What every launch turns off: approvals, which a headless run cannot ask; the instruction files, skills, rules and
/// user settings it would discover; the web; its updates, analytics, feedback, telemetry and history; and the features
/// that reach beyond the copy (REQ-SEC-6, T-22).
pub const SETTINGS: [&str; 33] = [
    r#"approval_policy="never""#,
    "project_doc_max_bytes=0",
    "skills.include_instructions=false",
    "skills.bundled.enabled=false",
    r#"web_search="disabled""#,
    "check_for_update_on_startup=false",
    "analytics.enabled=false",
    "feedback.enabled=false",
    r#"otel.exporter="none""#,
    r#"otel.metrics_exporter="none""#,
    r#"history.persistence="none""#,
    r#"cli_auth_credentials_store="file""#,
    "memories.use_memories=false",
    "memories.generate_memories=false",
    "features.hooks=false",
    "features.plugins=false",
    "features.remote_plugin=false",
    "features.plugin_discovery_prefetch=false",
    "features.apps=false",
    "features.memories=false",
    "features.image_generation=false",
    // Its own reading of an image by path, outside the sandbox of the model's commands.
    "features.view_image=false",
    "features.browser_use=false",
    "features.computer_use=false",
    "features.goals=false",
    "features.tool_suggest=false",
    "features.skill_search=false",
    "features.skill_mcp_dependency_install=false",
    "features.shell_snapshot=false",
    // Every command runs to its end in one call: none left running for a later one (P9C-8).
    "features.unified_exec=false",
    "features.daemon_auto_start=false",
    "features.workspace_dependencies=false",
    // A run that cannot reach its provider ends, rather than retrying until its limit.
    "features.unbounded_connection_retries=false",
];

fn command(exe: &Path, launch: &Launch) -> Vec<OsString> {
    let mut argv: Vec<OsString> = vec![exe.into()];
    argv.extend(
        [
            "exec",
            "--json",
            "--color",
            "never",
            "--strict-config",
            "--skip-git-repo-check",
            "--ephemeral",
            "--ignore-user-config",
            "--ignore-rules",
            "--cd",
            "/work",
        ]
        .map(OsString::from),
    );
    argv.extend(profile(launch.role).map(OsString::from));
    argv.extend(SETTINGS.iter().flat_map(|s| ["-c", s]).map(OsString::from));
    argv.extend(["--model", launch.model, "-c"].map(OsString::from));
    argv.push(format!("model_reasoning_effort=\"{}\"", launch.effort).into());
    argv.extend(["--output-schema", adapter::SCHEMA, "--", launch.packet].map(OsString::from));
    argv
}

/// The preflight's commands: the tool's version; and its own sandbox under each role's profile, the builder's creating
/// a temporary file in the copy and in its `/tmp`, the reviewer's failing to write in the copy.
fn preflight(exe: &Path) -> Vec<Vec<OsString>> {
    let sandbox = |role: Role, script: &str| {
        let mut argv: Vec<OsString> = vec![exe.into(), "sandbox".into()];
        argv.extend(profile(role).map(OsString::from));
        argv.extend(["-P", "fabrewk", "-C", "/work", "--", "/bin/sh", "-c", script].map(OsString::from));
        argv
    };
    let writes = "f=$(mktemp -p /work) && test -f \"$f\" && rm \"$f\" && g=$(mktemp) && rm \"$g\"";
    // Only the copy's being read-only passes, never a failure of something else (a redirection, a missing program).
    let reads = "test -r /work/. && o=$(touch /work/.fabrewk-preflight 2>&1); case $o in *'Read-only file system'*) \
        exit 0;; esac; exit 1";
    vec![vec![exe.into(), "--version".into()], sandbox(Role::Build, writes), sandbox(Role::Review, reads)]
}

fn read(out: &[u8]) -> Report {
    stream::read(out)
}
