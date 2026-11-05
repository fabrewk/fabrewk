//! The private folder and the instance identity (REQ-SEC-4, REQ-REG-12; AD-15; part 7c2a). The folder is outside
//! every repository, for its owner only; Fabrewk refuses to read it when its permissions are wider. It holds the
//! machine key, made by the first write command with `ssh-keygen -t ed25519` and no passphrase; every event records
//! its fingerprint as `instance`, which nothing verifies yet. Reading commands only look: they create nothing.

use crate::code::{Code, Error, io_at, refuse};
use crate::record::sign::ssh_keygen;
use crate::settings::key_blob;
use sha2::{Digest, Sha256};
use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
use std::path::{Path, PathBuf};

/// The private folder (AD-15) from the environment `var` gives: `$XDG_CONFIG_HOME/fabrewk/` when that is an absolute
/// path, else `~/.config/fabrewk/`; on macOS `~/Library/Application Support/fabrewk/`.
pub fn private_folder(var: impl Fn(&str) -> Option<OsString>) -> Result<PathBuf, Error> {
    let absolute = |name: &str| var(name).map(PathBuf::from).filter(|p| p.is_absolute());
    let home = || absolute("HOME").ok_or_else(|| io_at(Path::new("$HOME"))(std::io::ErrorKind::NotFound.into()));
    let base = if cfg!(target_os = "macos") {
        home()?.join("Library/Application Support")
    } else {
        match absolute("XDG_CONFIG_HOME") {
            Some(dir) => dir,
            None => home()?.join(".config"),
        }
    };
    Ok(base.join("fabrewk"))
}

/// Refuses a private folder that is not a directory with access for its owner only (`PRIVATE_FOLDER_PERMISSIONS`).
pub fn check_permissions(folder: &Path) -> Result<(), Error> {
    let meta = fs::metadata(folder).map_err(io_at(folder))?;
    if !meta.is_dir() {
        return Err(refuse(Code::PrivateFolderPermissions, Some(folder), "not a directory"));
    }
    let mode = meta.permissions().mode() & 0o777;
    if mode & 0o077 != 0 {
        let why = format!("mode {mode:04o}; it must give access to its owner only (0700)");
        return Err(refuse(Code::PrivateFolderPermissions, Some(folder), why));
    }
    Ok(())
}

/// The instance, for a reading command: `None` when there is no private folder or no machine key yet. Writes
/// nothing.
pub fn instance(folder: &Path) -> Result<Option<String>, Error> {
    if !folder.exists() {
        return Ok(None);
    }
    check_permissions(folder)?;
    let (key, public) = (folder.join("instance"), folder.join("instance.pub"));
    if !key.is_file() || !public.is_file() {
        return Ok(None);
    }
    // A public key file that holds no key is a key half made: there is none, and a write command makes it again.
    let line = fs::read_to_string(&public).map_err(io_at(&public))?;
    Ok(fingerprint(line.trim_end()))
}

/// The instance, for a write command: creates the private folder (`0700`) and the machine key when absent, and gives
/// the key's fingerprint. A key half made by a stopped command (its two files are renamed into place, the public one
/// first) is made again.
pub fn ensure_instance(folder: &Path) -> Result<String, Error> {
    if !folder.exists() {
        let parent = folder.parent().unwrap_or(folder);
        fs::create_dir_all(parent).map_err(io_at(parent))?;
        fs::DirBuilder::new().mode(0o700).create(folder).map_err(io_at(folder))?;
    }
    if let Some(fp) = instance(folder)? {
        return Ok(fp);
    }
    let new = folder.join(".instance-new");
    let new_public = folder.join(".instance-new.pub");
    for leftover in [&new, &new_public] {
        if leftover.exists() {
            fs::remove_file(leftover).map_err(io_at(leftover))?;
        }
    }
    let mut args: Vec<&std::ffi::OsStr> =
        ["-q", "-t", "ed25519", "-N", "", "-C", "fabrewk-instance", "-f"].iter().map(|a| a.as_ref()).collect();
    args.push(new.as_os_str());
    if !ssh_keygen(&args, b"")?.status.success() {
        return Err(refuse(Code::ToolFailed, Some(folder), "ssh-keygen -t ed25519 failed"));
    }
    fs::rename(&new_public, folder.join("instance.pub")).map_err(io_at(folder))?;
    fs::rename(&new, folder.join("instance")).map_err(io_at(folder))?;
    instance(folder)?.ok_or_else(|| refuse(Code::ToolFailed, Some(folder), "the machine key is not in place"))
}

/// A public key's fingerprint as `ssh-keygen -l` prints it: `SHA256:` and the unpadded base64 of the key blob's digest.
pub fn fingerprint(public: &str) -> Option<String> {
    let digest = Sha256::digest(key_blob(public)?);
    const DIGITS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::from("SHA256:");
    for chunk in digest.chunks(3) {
        let n = chunk.iter().fold(0u32, |acc, b| acc << 8 | u32::from(*b)) << (8 * (3 - chunk.len()));
        for i in 0..=chunk.len() {
            out.push(char::from(DIGITS[(n >> (18 - 6 * i) & 63) as usize]));
        }
    }
    Some(out)
}
