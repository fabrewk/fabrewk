//! A release (D-097 point 10; part 10d, D-108): `fabrewk-<version>-linux-aarch64.tar.gz`, holding the musl binary,
//! `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`, the third-party notices and the completion files, and `SHA256SUMS`;
//! a maestro then signs `SHA256SUMS` with `ssh-keygen -Y sign -n fabrewk-release`, which writes `SHA256SUMS.sig`.
//! The archive is reproducible: its entries sorted, owned by 0, dated by the commit, and gzip writes no name or time.

use crate::json::{self, Value};
use crate::notices;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The one platform a release is for (D-097 point 10).
pub const PLATFORM: &str = "linux-aarch64";
/// The namespace a release's signature is made in and verified against.
pub const NAMESPACE: &str = "fabrewk-release";
/// The completion files the release carries, from `crates/fabrewk/completions/`.
pub const COMPLETIONS: &[&str] = &["fabrewk.fish", "fabrewk.sh"];

/// The release's base name for `version`: the archive's name without `.tar.gz`, and its one directory.
pub fn base(version: &str) -> String {
    format!("fabrewk-{version}-{PLATFORM}")
}

fn run(c: &mut Command) -> Result<String, String> {
    let shown = format!("{c:?}");
    let out = c.output().map_err(|e| format!("cannot run {shown}: {e}"))?;
    if !out.status.success() {
        return Err(format!("{shown} failed: {}", String::from_utf8_lossy(&out.stderr).trim()));
    }
    String::from_utf8(out.stdout).map_err(|_| format!("{shown} printed non-UTF-8"))
}

fn put(from: &Path, to: &Path, mode: u32) -> Result<(), String> {
    std::fs::copy(from, to).map_err(|e| format!("{}: {e}", from.display()))?;
    set_mode(to, mode)
}

fn set_mode(path: &Path, mode: u32) -> Result<(), String> {
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode))
        .map_err(|e| format!("{}: {e}", path.display()))
}

/// Refuses a binary that is not this version's or that was built with a feature (the crash suite's seam).
pub fn check_binary(said: &str, version: &str) -> Result<(), String> {
    let doc = json::parse(said)?;
    if doc.get("version").and_then(Value::as_str) != Some(version) {
        return Err(format!("the binary is not version {version}: {}", said.trim()));
    }
    if !matches!(doc.get("features"), Some(Value::Array(f)) if f.is_empty()) {
        return Err(format!("the binary was built with features: {}", said.trim()));
    }
    Ok(())
}

/// Refuses a binary that is not the release target's: a 64-bit little-endian ELF for AArch64, static (no program
/// interpreter, which a GNU build names) and stripped (no symbol table), as `--target aarch64-unknown-linux-musl`
/// with the release profile builds it.
pub fn check_elf(bytes: &[u8]) -> Result<(), String> {
    let u16_at = |at: usize| bytes.get(at..at + 2).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize);
    let u32_at = |at: usize| bytes.get(at..at + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    let u64_at =
        |at: usize| bytes.get(at..at + 8).and_then(|b| usize::try_from(u64::from_le_bytes(b.try_into().ok()?)).ok());
    if bytes.get(..6) != Some(&[0x7f, b'E', b'L', b'F', 2, 1][..]) || u16_at(18) != Some(183) {
        return Err("the binary is not a 64-bit little-endian ELF for AArch64, the release's target".into());
    }
    // Each entry's type: a program header's `p_type` at its offset 0, a section header's `sh_type` at its offset 4.
    let types = |off: usize, size: usize, count: usize, at: usize| -> Option<Vec<u32>> {
        (0..count).map(|i| u32_at(off.checked_add(i.checked_mul(size)?)?.checked_add(at)?)).collect()
    };
    let (Some(phoff), Some(56), Some(phnum), Some(shoff), Some(64), Some(shnum)) =
        (u64_at(32), u16_at(54), u16_at(56), u64_at(40), u16_at(58), u16_at(60))
    else {
        return Err("the binary's ELF header is not one this check reads".into());
    };
    let segments = types(phoff, 56, phnum, 0).ok_or("the binary's program headers are cut short")?;
    if segments.contains(&3) {
        return Err("the binary names a program interpreter: it is dynamic, not the static musl build".into());
    }
    let sections = types(shoff, 64, shnum, 4).ok_or("the binary's section headers are cut short")?;
    if sections.contains(&2) {
        return Err("the binary holds a symbol table: it is not the release profile's stripped build".into());
    }
    Ok(())
}

/// Makes the release of the workspace at `root` from the musl binary `binary` (checked as [`check_elf`] says, then
/// asked its version) into `out`, a directory absent or empty: the archive and `SHA256SUMS`, which it returns.
pub fn make(root: &Path, binary: &Path, out: &Path) -> Result<Vec<PathBuf>, String> {
    let version = env!("CARGO_PKG_VERSION");
    check_elf(&std::fs::read(binary).map_err(|e| format!("{}: {e}", binary.display()))?)?;
    check_binary(&run(Command::new(binary).args(["--version", "--json"]))?, version)?;
    if out.exists() && std::fs::read_dir(out).map_err(|e| format!("{}: {e}", out.display()))?.next().is_some() {
        return Err(format!("{} is not empty; a release is made into an empty directory", out.display()));
    }
    let base = base(version);
    let stage = out.join(&base);
    let mkdir = |d: &Path| std::fs::create_dir_all(d).map_err(|e| format!("{}: {e}", d.display()));
    mkdir(&stage.join("completions"))?;
    for d in [&stage, &stage.join("completions")] {
        set_mode(d, 0o755)?;
    }
    put(binary, &stage.join("fabrewk"), 0o755)?;
    for f in ["LICENSE-MIT", "LICENSE-APACHE", "NOTICE"] {
        put(&root.join(f), &stage.join(f), 0o644)?;
    }
    notices::write(root, &stage)?;
    for f in [notices::FILE, notices::STD_FILE] {
        set_mode(&stage.join(f), 0o644)?;
    }
    for f in COMPLETIONS {
        put(&root.join("crates/fabrewk/completions").join(f), &stage.join("completions").join(f), 0o644)?;
    }
    let epoch = run(Command::new("git").current_dir(root).args(["log", "-1", "--format=%ct", "HEAD"]))?;
    let tar = format!("{base}.tar");
    run(Command::new("tar").current_dir(out).args([
        "--sort=name",
        "--owner=0",
        "--group=0",
        "--numeric-owner",
        &format!("--mtime=@{}", epoch.trim()),
        "--format=ustar",
        "-cf",
        &tar,
        &base,
    ]))?;
    run(Command::new("gzip").current_dir(out).args(["-n", "-9", &tar]))?;
    std::fs::remove_dir_all(&stage).map_err(|e| format!("{}: {e}", stage.display()))?;
    let archive = format!("{tar}.gz");
    let sums = run(Command::new("sha256sum").current_dir(out).arg(&archive))?;
    std::fs::write(out.join("SHA256SUMS"), sums).map_err(|e| format!("{}: {e}", out.display()))?;
    Ok(vec![out.join(archive), out.join("SHA256SUMS")])
}
