//! Signatures of a person's events (REQ-REG-4; D-047, D-048): `ssh-keygen -Y sign -n fabrewk` over the event file's
//! bytes. A signature is verified without any allowed-signers file, so that a reading command writes nothing: the
//! public key the signature carries must be exactly the key expected, and `ssh-keygen -Y check-novalidate -n fabrewk`
//! must accept it ([`check`]). Which key is expected, and whether the role allows the kind, comes from the policy the
//! event cites: [`Verify`]'s implementation is the identity module's (part 7c2a, D-050).

use super::replay::Replay;
use super::{Code, Error, io_at, refuse};
use crate::settings::{base64, ssh_strings};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Makes the detached signature of an event's bytes.
pub trait Sign {
    fn sign(&self, bytes: &[u8]) -> Result<Vec<u8>, Error>;
    /// The public key it signs with, as `<type> <base64> [comment]`.
    fn public(&self) -> Result<String, Error>;
}

/// Checks the signatures of a record already replayed field by field: every person's event, each failure by its code.
pub trait Verify {
    fn verify(&self, record: &Replay) -> Result<(), Vec<Error>>;
}

/// Signs with an SSH private key file; its public key is the file beside it with `.pub` added.
pub struct KeySigner(pub PathBuf);

impl Sign for KeySigner {
    fn sign(&self, bytes: &[u8]) -> Result<Vec<u8>, Error> {
        let key = self.0.as_os_str();
        let out = ssh_keygen(
            &["-Y".as_ref(), "sign".as_ref(), "-n".as_ref(), "fabrewk".as_ref(), "-f".as_ref(), key],
            bytes,
        )?;
        if !out.status.success() {
            return Err(refuse(Code::ToolFailed, Some(&self.0), "ssh-keygen -Y sign failed"));
        }
        Ok(out.stdout)
    }

    fn public(&self) -> Result<String, Error> {
        let mut path = self.0.clone().into_os_string();
        path.push(".pub");
        let path = PathBuf::from(path);
        std::fs::read_to_string(&path).map(|s| s.trim_end().to_owned()).map_err(io_at(&path))
    }
}

/// The public key blob an armored SSH signature carries, if it is one made in the namespace `fabrewk`.
pub fn carried_key(sig: &[u8]) -> Option<Vec<u8>> {
    let text = std::str::from_utf8(sig).ok()?;
    let body = text.strip_prefix("-----BEGIN SSH SIGNATURE-----\n")?.strip_suffix("-----END SSH SIGNATURE-----\n")?;
    let blob = base64(&body.split('\n').collect::<String>())?;
    // The magic `SSHSIG`, version 1, then the key, the namespace, a reserved field, the hash and the signature.
    let parts = ssh_strings(blob.strip_prefix(b"SSHSIG\0\0\0\x01")?)?;
    (parts.len() == 5 && parts[1] == b"fabrewk").then(|| parts[0].to_vec())
}

/// Checks the signature file `sig` over `bytes` (D-048): it carries exactly `key` (a key blob), and
/// `ssh-keygen -Y check-novalidate -n fabrewk` accepts it. Writes nothing.
pub fn check(key: &[u8], bytes: &[u8], sig: &Path) -> Result<(), Error> {
    let carried = std::fs::read(sig).map_err(io_at(sig))?;
    if carried_key(&carried).as_deref() != Some(key) {
        return Err(refuse(Code::RecordSignatureInvalid, Some(sig), "not a signature by the expected key"));
    }
    let args = ["-Y", "check-novalidate", "-n", "fabrewk", "-s"];
    let mut all: Vec<&std::ffi::OsStr> = args.iter().map(|a| a.as_ref()).collect();
    all.push(sig.as_os_str());
    // `ssh-keygen` exits 255 when it refuses a signature; any other failure, a signal included, is the tool's own.
    match ssh_keygen(&all, bytes)?.status.code() {
        Some(0) => Ok(()),
        Some(255) => {
            Err(refuse(Code::RecordSignatureInvalid, Some(sig), "ssh-keygen -Y check-novalidate refused the signature"))
        }
        _ => Err(refuse(Code::ToolFailed, Some(sig), "ssh-keygen -Y check-novalidate failed")),
    }
}

pub(crate) fn ssh_keygen(args: &[&std::ffi::OsStr], input: &[u8]) -> Result<std::process::Output, Error> {
    let mut child = Command::new("ssh-keygen")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| {
            let code = if e.kind() == std::io::ErrorKind::NotFound { Code::ToolMissing } else { Code::ToolFailed };
            refuse(code, None, "ssh-keygen")
        })?;
    // The message is small (one event); a closed pipe means ssh-keygen stopped early, which its status reports.
    let _ = child.stdin.take().map(|mut s| s.write_all(input));
    child.wait_with_output().map_err(|_| refuse(Code::ToolFailed, None, "ssh-keygen"))
}
