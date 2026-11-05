//! Signatures of a person's events (REQ-REG-4, the mechanism; D-047): `ssh-keygen -Y sign -n fabrewk` over the event
//! file's bytes, verified with `ssh-keygen -Y verify` against an allowed-signers file. Which file, and whether the
//! signer's role allows the kind, is 7c's: it derives both from the policy the event cites.

use super::{Code, Error, refuse};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Makes the detached signature of an event's bytes.
pub trait Sign {
    fn sign(&self, bytes: &[u8]) -> Result<Vec<u8>, Error>;
}

/// Checks a person's event: `alias` is the actor the event names, `sig` its `.sig` file.
pub trait Verify {
    fn verify(&self, alias: &str, bytes: &[u8], sig: &Path) -> Result<(), Error>;
}

/// Signs with an SSH private key file.
pub struct KeySigner(pub PathBuf);

/// Verifies against an allowed-signers file whose principals are aliases.
pub struct SshVerifier(pub PathBuf);

impl Sign for KeySigner {
    fn sign(&self, bytes: &[u8]) -> Result<Vec<u8>, Error> {
        let key = self.0.as_os_str();
        let out = ssh_keygen(
            &[
                "-Y".as_ref(),
                "sign".as_ref(),
                "-n".as_ref(),
                "fabrewk".as_ref(),
                "-f".as_ref(),
                key,
            ],
            bytes,
        )?;
        if !out.status.success() {
            return Err(refuse(
                Code::ToolFailed,
                Some(&self.0),
                "ssh-keygen -Y sign failed",
            ));
        }
        Ok(out.stdout)
    }
}

impl Verify for SshVerifier {
    fn verify(&self, alias: &str, bytes: &[u8], sig: &Path) -> Result<(), Error> {
        let args = ["-Y", "verify", "-n", "fabrewk", "-I", alias];
        let mut all: Vec<&std::ffi::OsStr> = args.iter().map(|a| a.as_ref()).collect();
        all.extend([
            "-f".as_ref(),
            self.0.as_os_str(),
            "-s".as_ref(),
            sig.as_os_str(),
        ]);
        if ssh_keygen(&all, bytes)?.status.success() {
            Ok(())
        } else {
            Err(refuse(
                Code::RecordSignatureInvalid,
                Some(sig),
                "ssh-keygen -Y verify refused the signature",
            ))
        }
    }
}

fn ssh_keygen(args: &[&std::ffi::OsStr], input: &[u8]) -> Result<std::process::Output, Error> {
    let mut child = Command::new("ssh-keygen")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| {
            let code = if e.kind() == std::io::ErrorKind::NotFound {
                Code::ToolMissing
            } else {
                Code::ToolFailed
            };
            refuse(code, None, "ssh-keygen")
        })?;
    // The message is small (one event); a closed pipe means ssh-keygen stopped early, which its status reports.
    let _ = child.stdin.take().map(|mut s| s.write_all(input));
    child
        .wait_with_output()
        .map_err(|_| refuse(Code::ToolFailed, None, "ssh-keygen"))
}
