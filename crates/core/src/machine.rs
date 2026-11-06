//! The private folder, the machine file and the instance identity (REQ-SEC-4, REQ-SET-1, REQ-REG-12; AD-15; parts
//! 7c2a and 7c2b). The folder is outside every repository, for its owner only; Fabrewk refuses to read it when its
//! permissions are wider. It holds `machine.toml` and the machine key, made by the first write command with
//! `ssh-keygen -t ed25519` and no passphrase; every event records its fingerprint as `instance`, which nothing verifies
//! yet. Reading commands only look: they create nothing.

use crate::code::{Code, Error, io_at, refuse};
use crate::record::sign::ssh_keygen;
use crate::settings::{self, Level, Settings, key_blob};
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
    let meta = fs::symlink_metadata(folder).map_err(io_at(folder))?;
    if !meta.is_dir() {
        return Err(refuse(Code::PrivateFolderPermissions, Some(folder), "not a directory; a link is not followed"));
    }
    let mode = meta.permissions().mode() & 0o777;
    if mode & 0o077 != 0 {
        let why = format!("mode {mode:04o}; it must give access to its owner only (0700)");
        return Err(refuse(Code::PrivateFolderPermissions, Some(folder), why));
    }
    Ok(())
}

/// `machine.toml` (REQ-SET-1, AD-15; part 7c2b): machine data detection cannot find, read only from a folder whose
/// permissions are its owner's; `None` when there is no folder or no file. Its values are declared, never permitted.
pub fn machine_file(folder: &Path) -> Result<Option<Settings>, Vec<Error>> {
    // Absent is only what does not exist; what cannot be examined is an error, never an absence (P7D2-24).
    let absent = |p: &Path| match fs::symlink_metadata(p) {
        Ok(_) => Ok(false),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(true),
        Err(e) => Err(vec![io_at(p)(e)]),
    };
    if absent(folder)? {
        return Ok(None);
    }
    check_permissions(folder).map_err(|e| vec![e])?;
    let path = folder.join("machine.toml");
    if absent(&path)? {
        return Ok(None);
    }
    settings::read(&path, Level::Machine).map(Some)
}

/// The instance, for a reading command: `None` when there is no private folder or no machine key yet. Writes
/// nothing.
pub fn instance(folder: &Path) -> Result<Option<String>, Error> {
    if !folder.exists() {
        return Ok(None);
    }
    check_permissions(folder)?;
    let key = folder.join("instance");
    if !key.is_file() {
        return Ok(None);
    }
    // The identity is the private key: its public half is derived from it, never read from a file beside it. A key
    // that does not read (damaged by hand) is none, and a write command makes it again.
    let args: Vec<&std::ffi::OsStr> =
        ["-y".as_ref(), "-P".as_ref(), "".as_ref(), "-f".as_ref(), key.as_os_str()].into();
    let out = ssh_keygen(&args, b"")?;
    Ok(out.status.success().then(|| fingerprint(String::from_utf8_lossy(&out.stdout).trim_end())).flatten())
}

/// The instance, for a write command: creates the private folder (`0700`) and the machine key when absent, and gives
/// the key's fingerprint. Two commands on one machine (in two repositories) make it under the folder's `lock`, so
/// the machine gets one key. The private key is renamed into place last, so a key half made by a stopped command is
/// none, and is made again.
pub fn ensure_instance(folder: &Path) -> Result<String, Error> {
    match prepare_instance(folder)? {
        (fp, None) => Ok(fp),
        (_, Some(pending)) => pending.install(),
    }
}

/// A machine key made under a temporary name and not yet in place (part 7d3a, P7D3A-1): [`Pending::install`] puts it
/// in place; dropped instead, it removes everything it made, the folders it created included, so that a write command
/// refused after making it leaves the private folder as it was.
pub struct Pending {
    folder: PathBuf,
    dirs: Vec<PathBuf>,
    files: Vec<PathBuf>,
    fingerprint: String,
    _lock: Option<fs::File>,
    keep: bool,
}

impl Pending {
    /// Renames the key into place, its public half first, and gives its fingerprint.
    pub fn install(mut self) -> Result<String, Error> {
        let folder = self.folder.clone();
        fs::rename(folder.join(".instance-new.pub"), folder.join("instance.pub")).map_err(io_at(&folder))?;
        fs::rename(folder.join(".instance-new"), folder.join("instance")).map_err(io_at(&folder))?;
        self.keep = true;
        let fp =
            instance(&folder)?.ok_or_else(|| refuse(Code::ToolFailed, Some(&folder), "no machine key in place"))?;
        if fp != self.fingerprint {
            return Err(refuse(Code::ToolFailed, Some(&folder), "the machine key in place is another"));
        }
        Ok(fp)
    }
}

impl Drop for Pending {
    fn drop(&mut self) {
        if self.keep {
            return;
        }
        for f in self.files.iter().rev() {
            let _ = fs::remove_file(f);
        }
        for d in self.dirs.iter().rev() {
            let _ = fs::remove_dir(d);
        }
    }
}

/// The machine key's fingerprint if it is in place (`None` for the pending key), or the key made under a temporary
/// name, with the private folder and its lock, when there is none.
pub fn prepare_instance(folder: &Path) -> Result<(String, Option<Pending>), Error> {
    if let Some(fp) = instance(folder)? {
        return Ok((fp, None));
    }
    let absent = |p: &Path| fs::symlink_metadata(p).is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound);
    let dirs: Vec<PathBuf> = folder.ancestors().take_while(|p| absent(p)).map(Path::to_path_buf).collect();
    let mut pending = Pending {
        folder: folder.to_path_buf(),
        dirs: dirs.into_iter().rev().collect(),
        files: Vec::new(),
        fingerprint: String::new(),
        _lock: None,
        keep: false,
    };
    // Every directory it makes is its owner's only, the folder's parents included; one that exists is kept as it is.
    fs::DirBuilder::new().mode(0o700).recursive(true).create(folder).map_err(io_at(folder))?;
    let path = folder.join("lock");
    if absent(&path) {
        pending.files.push(path.clone());
    }
    let lock = fs::File::options().create(true).truncate(false).write(true).open(&path).map_err(io_at(&path))?;
    lock.lock().map_err(io_at(&path))?;
    pending._lock = Some(lock);
    if let Some(fp) = instance(folder)? {
        // Another command put one in place meanwhile; what it uses stays.
        pending.keep = true;
        return Ok((fp, None));
    }
    let new = folder.join(".instance-new");
    let names = [folder.join("instance"), folder.join("instance.pub"), new.clone(), folder.join(".instance-new.pub")];
    for leftover in names.iter().filter(|p| p.exists()) {
        fs::remove_file(leftover).map_err(io_at(leftover))?;
    }
    pending.files.extend([names[2].clone(), names[3].clone()]);
    let mut args: Vec<&std::ffi::OsStr> =
        ["-q", "-t", "ed25519", "-N", "", "-C", "fabrewk-instance", "-f"].iter().map(|a| a.as_ref()).collect();
    args.push(new.as_os_str());
    if !ssh_keygen(&args, b"")?.status.success() {
        return Err(refuse(Code::ToolFailed, Some(folder), "ssh-keygen -t ed25519 failed"));
    }
    let args: Vec<&std::ffi::OsStr> =
        ["-y".as_ref(), "-P".as_ref(), "".as_ref(), "-f".as_ref(), new.as_os_str()].into();
    let out = ssh_keygen(&args, b"")?;
    let public = String::from_utf8_lossy(&out.stdout).trim_end().to_owned();
    pending.fingerprint = fingerprint(&public)
        .filter(|_| out.status.success())
        .ok_or_else(|| refuse(Code::ToolFailed, Some(folder), "ssh-keygen -y failed"))?;
    Ok((pending.fingerprint.clone(), Some(pending)))
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
