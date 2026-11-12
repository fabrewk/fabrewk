//! The Claude Code adapter (ring `ToolAdapter`; part 9c): how Claude Code is detected, confined, launched per role,
//! preflighted and read. The release is the directory that holds the executable as it resolves. The tool runs headless
//! with its discovery off as far as its flags allow, its own sandbox required for every command the model runs, with
//! no network for those commands, and the permissions of its role; its stream is read by one grammar, which the audit
//! of a reviewer's examinations shares.

use fabrewk_core::adapter::{Adapter, Launch, Report, Role};
use fabrewk_core::detect::Program;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

mod stream;

pub use fabrewk_core::adapter::segments;

/// The adapter, as the binary wires it.
pub const ADAPTER: Adapter = Adapter {
    program: Program {
        adapter: Some("claude-code"),
        executable: "claude",
        version: &["--version"],
        tokens: true,
        skills: &[".claude/skills"],
    },
    provider: "anthropic",
    markers: &["CLAUDECODE"],
    protected: &["CLAUDE.md", ".claude"],
    residual: &["tool-credential", "audit-limit", "tool-flags"],
    machine: &[".git"],
    credential: ".claude/.credentials.json",
    expiry,
    env: &ENV,
    release,
    command,
    preflight,
    read,
};

/// No update, telemetry, error report or other traffic of the tool's own (REQ-SEC-6).
const ENV: [(&str, &str); 5] = [
    ("DISABLE_AUTOUPDATER", "1"),
    ("DISABLE_UPDATES", "1"),
    ("DISABLE_TELEMETRY", "1"),
    ("DISABLE_ERROR_REPORTING", "1"),
    ("CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC", "1"),
];

/// When the login expires: its OAuth token's `expiresAt`, in milliseconds since 1970 (D-079). Only that field is read.
fn expiry(bytes: &[u8]) -> Option<u64> {
    let credential = fabrewk_core::adapter::json(std::str::from_utf8(bytes).ok()?)?;
    Some(credential.get("claudeAiOauth")?.get("expiresAt")?.as_u64()? / 1000)
}

/// The directory that holds the executable: Claude Code's installer keeps one executable per version in a directory
/// named `versions`; an executable anywhere else has no release this adapter can tell, so none is bound.
fn release(exe: &Path) -> Option<PathBuf> {
    exe.parent().filter(|d| d.file_name().is_some_and(|n| n == "versions")).map(Path::to_path_buf)
}

/// The settings every launch adds (`--settings`): the tool's own sandbox on, required, with no way out of it and no
/// domain allowed, so that the model's commands run without network (3.6); the credential's folder unreadable.
pub const SETTINGS: &str = concat!(
    r#"{"sandbox":{"enabled":true,"failIfUnavailable":true,"allowUnsandboxedCommands":false,"#,
    r#""autoAllowBashIfSandboxed":true,"network":{"allowedDomains":[]},"#,
    r#""filesystem":{"denyRead":["/home/agent/.claude"]}},"#,
    r#""permissions":{"deny":["Read(//home/agent/.claude/**)","Edit(//home/agent/.claude/**)","WebFetch","WebSearch"]}}"#
);

/// The tools and the permission mode of each role: a builder edits its copy; a reviewer only reads and runs.
fn role(role: Role) -> (&'static str, &'static str) {
    match role {
        Role::Build => ("Bash,Read,Edit,Write,Glob,Grep", "acceptEdits"),
        Role::Review => ("Bash,Read,Glob,Grep", "dontAsk"),
    }
}

fn command(exe: &Path, launch: &Launch) -> Vec<OsString> {
    let (tools, mode) = role(launch.role);
    let mut argv: Vec<OsString> = vec![exe.into()];
    argv.extend(
        [
            "--print",
            "--output-format",
            "stream-json",
            "--verbose",
            "--safe-mode",
            "--restricted",
            "--strict-mcp-config",
            "--disable-slash-commands",
            "--no-session-persistence",
            "--permission-prompts",
            "none",
            "--permission-mode",
            mode,
            "--tools",
            tools,
            "--settings",
            SETTINGS,
            "--json-schema",
            launch.schema,
            "--model",
            launch.model,
            "--effort",
            launch.effort,
            "--",
            launch.packet,
        ]
        .map(OsString::from),
    );
    argv
}

/// The preflight's commands: the tool's version; the two programs its sandbox drives on Linux; and a sandbox of the
/// kind the tool makes, nested in the cell, creating a temporary file in the copy and in its own `/tmp`.
fn preflight(exe: &Path) -> Vec<Vec<OsString>> {
    let nested = "f=$(mktemp -p /work) && test -f \"$f\" && rm \"$f\" && g=$(mktemp) && rm \"$g\"";
    vec![
        vec![exe.into(), "--version".into()],
        ["/usr/bin/socat", "-V"].map(OsString::from).into(),
        [
            "/usr/bin/bwrap",
            "--unshare-all",
            "--die-with-parent",
            "--ro-bind",
            "/",
            "/",
            "--bind",
            "/work",
            "/work",
            "--tmpfs",
            "/tmp",
            "--proc",
            "/proc",
            "--",
            "/bin/sh",
            "-c",
            nested,
        ]
        .map(OsString::from)
        .into(),
    ]
}

fn read(out: &[u8]) -> Report {
    stream::read(out)
}
