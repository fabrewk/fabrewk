//! The refusal codes and the error every part of the core returns (core-specification section 7). Nothing here
//! prints: the command surface words each code from its catalogue.

use std::path::{Path, PathBuf};

/// The refusal codes the core raises (core-specification section 7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Code {
    AliasInvalid,
    AliasMismatch,
    AttemptEnded,
    CheckDuplicate,
    CheckUnknown,
    CredentialSuspected,
    DomainNotIgnored,
    DomainUnknownEntry,
    GitignoreChanged,
    HeadDetached,
    KeyUnknown,
    LimitUnobservable,
    LockHeld,
    MarkInvalid,
    MoveNotAllowed,
    NotARepository,
    PolicyAbsent,
    PolicyPending,
    PolicyUncommitted,
    PrivateFolderPermissions,
    ProtectedPath,
    ProtectedPathWildcard,
    RecordAbsent,
    RecordBroken,
    RecordCommitRace,
    RecordDigestMismatch,
    RecordFork,
    RecordHeadMoved,
    RecordIdCollision,
    RecordNoncanonical,
    RecordRollback,
    RecordSignatureInvalid,
    RecordTransitionInvalid,
    RecordUncommitted,
    RecordVersionUnknown,
    RoleForbidden,
    RoleNotEnforced,
    SettingsInvalid,
    SettingsKeyNotPersonal,
    TaskChanged,
    TaskInvalid,
    ToolFailed,
    ToolMissing,
    WideningUnnamed,
}

/// The code as the catalogue and `--json` name it: the variant's name in upper snake case (`RecordFork` is
/// `RECORD_FORK`); a test holds every name to section 7's list.
impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, c) in format!("{self:?}").char_indices() {
            if i > 0 && c.is_ascii_uppercase() {
                f.write_str("_")?;
            }
            write!(f, "{}", c.to_ascii_uppercase())?;
        }
        Ok(())
    }
}

/// A failure: a refusal with its code, or an operating-system error the caller words.
#[derive(Debug)]
pub enum Error {
    Refused { code: Code, path: Option<PathBuf>, detail: String },
    Io { path: PathBuf, error: std::io::Error },
}

/// A copy; an operating-system error keeps its kind, which is all the command surface words.
impl Clone for Error {
    fn clone(&self) -> Error {
        match self {
            Error::Refused { code, path, detail } => {
                Error::Refused { code: *code, path: path.clone(), detail: detail.clone() }
            }
            Error::Io { path, error } => Error::Io { path: path.clone(), error: error.kind().into() },
        }
    }
}

impl Error {
    /// What the refusal says beyond its code, or the operating system's error.
    pub fn detail(&self) -> String {
        match self {
            Error::Refused { detail, .. } => detail.clone(),
            Error::Io { error, .. } => error.to_string(),
        }
    }

    /// The refusal's code; `None` for an operating-system error.
    pub fn code(&self) -> Option<Code> {
        match self {
            Error::Refused { code, .. } => Some(*code),
            Error::Io { .. } => None,
        }
    }
}

/// Whether no entry from `base` itself down to `base/rel` is a link; one that does not exist is none (P7D2-18).
pub(crate) fn unlinked(base: &Path, rel: &str) -> bool {
    let link = |p: &Path| std::fs::symlink_metadata(p).is_ok_and(|m| m.file_type().is_symlink());
    let mut path = base.to_path_buf();
    !link(&path)
        && rel.split('/').all(|part| {
            path.push(part);
            !link(&path)
        })
}

/// `path` with `.` dropped and each `..` taken against the part before it, read as written.
pub(crate) fn normal(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in path.components() {
        match c {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    out
}

/// Whether `path`, as written or as it resolves, lies in a Fabrewk domain (a `.brewstillery` component) or under
/// `root`: what Fabrewk reads from a place the person names must lie in neither (P7D2-22, P7D2-28 to P7D2-30).
pub(crate) fn in_domain(path: &Path, root: &Path) -> bool {
    let root_canonical = root.canonicalize().ok();
    in_a_domain(path)
        || [Some(normal(path)), path.canonicalize().ok()]
            .iter()
            .flatten()
            .any(|p| p.starts_with(root) || root_canonical.as_ref().is_some_and(|r| p.starts_with(r)))
}

/// Whether `path`, as written or as it resolves, has a `.brewstillery` component.
pub(crate) fn in_a_domain(path: &Path) -> bool {
    let domain = |p: &Path| p.components().any(|c| c.as_os_str() == ".brewstillery");
    domain(&normal(path)) || path.canonicalize().is_ok_and(|p| domain(&p))
}

pub(crate) fn refuse(code: Code, path: Option<&Path>, detail: impl Into<String>) -> Error {
    Error::Refused { code, path: path.map(Path::to_path_buf), detail: detail.into() }
}

pub(crate) fn io_at(path: &Path) -> impl FnOnce(std::io::Error) -> Error + '_ {
    move |error| Error::Io { path: path.to_path_buf(), error }
}

#[cfg(test)]
mod tests {
    use super::Code::*;

    #[test]
    fn codes_are_named_as_the_specification_lists_them() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../.brewstillery/bootstrap/core-specification.md");
        let spec = std::fs::read_to_string(path).unwrap();
        let list = &spec[spec.find("All codes, in one list:").unwrap()..];
        let list = &list[..list.find("\n\n").unwrap()];
        for code in [
            AliasInvalid,
            AliasMismatch,
            AttemptEnded,
            CheckDuplicate,
            CheckUnknown,
            CredentialSuspected,
            DomainNotIgnored,
            DomainUnknownEntry,
            GitignoreChanged,
            HeadDetached,
            KeyUnknown,
            LimitUnobservable,
            LockHeld,
            MarkInvalid,
            MoveNotAllowed,
            NotARepository,
            PolicyAbsent,
            PolicyPending,
            PolicyUncommitted,
            PrivateFolderPermissions,
            ProtectedPath,
            ProtectedPathWildcard,
            RecordAbsent,
            RecordBroken,
            RecordCommitRace,
            RecordDigestMismatch,
            RecordFork,
            RecordHeadMoved,
            RecordIdCollision,
            RecordNoncanonical,
            RecordRollback,
            RecordSignatureInvalid,
            RecordTransitionInvalid,
            RecordUncommitted,
            RecordVersionUnknown,
            RoleForbidden,
            RoleNotEnforced,
            SettingsInvalid,
            SettingsKeyNotPersonal,
            TaskChanged,
            TaskInvalid,
            ToolFailed,
            ToolMissing,
            WideningUnnamed,
        ] {
            assert!(list.contains(&format!("`{code}`")), "{code:?}");
            assert!(code.to_string().bytes().all(|b| b.is_ascii_uppercase() || b == b'_'), "{code}");
        }
    }
}
