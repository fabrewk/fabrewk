//! The refusal codes and the error every part of the core returns (core-specification section 7). Nothing here
//! prints: the command surface words each code from its catalogue.

use std::path::{Path, PathBuf};

/// The refusal codes the core raises (core-specification section 7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Code {
    AliasInvalid,
    AliasMismatch,
    CheckUnknown,
    DomainNotIgnored,
    DomainUnknownEntry,
    GitignoreChanged,
    HeadDetached,
    KeyUnknown,
    LockHeld,
    MarkInvalid,
    NotARepository,
    PolicyAbsent,
    PolicyPending,
    PolicyUncommitted,
    PrivateFolderPermissions,
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
    RecordVersionUnknown,
    RoleForbidden,
    RoleNotEnforced,
    SettingsInvalid,
    SettingsKeyNotPersonal,
    ToolFailed,
    ToolMissing,
    WideningUnnamed,
}

impl Code {
    /// The code as the catalogue and `--json` name it.
    pub const fn as_str(self) -> &'static str {
        match self {
            Code::AliasInvalid => "ALIAS_INVALID",
            Code::AliasMismatch => "ALIAS_MISMATCH",
            Code::CheckUnknown => "CHECK_UNKNOWN",
            Code::DomainNotIgnored => "DOMAIN_NOT_IGNORED",
            Code::DomainUnknownEntry => "DOMAIN_UNKNOWN_ENTRY",
            Code::GitignoreChanged => "GITIGNORE_CHANGED",
            Code::HeadDetached => "HEAD_DETACHED",
            Code::KeyUnknown => "KEY_UNKNOWN",
            Code::LockHeld => "LOCK_HELD",
            Code::MarkInvalid => "MARK_INVALID",
            Code::NotARepository => "NOT_A_REPOSITORY",
            Code::PolicyAbsent => "POLICY_ABSENT",
            Code::PolicyPending => "POLICY_PENDING",
            Code::PolicyUncommitted => "POLICY_UNCOMMITTED",
            Code::PrivateFolderPermissions => "PRIVATE_FOLDER_PERMISSIONS",
            Code::RecordAbsent => "RECORD_ABSENT",
            Code::RecordBroken => "RECORD_BROKEN",
            Code::RecordCommitRace => "RECORD_COMMIT_RACE",
            Code::RecordDigestMismatch => "RECORD_DIGEST_MISMATCH",
            Code::RecordFork => "RECORD_FORK",
            Code::RecordHeadMoved => "RECORD_HEAD_MOVED",
            Code::RecordIdCollision => "RECORD_ID_COLLISION",
            Code::RecordNoncanonical => "RECORD_NONCANONICAL",
            Code::RecordRollback => "RECORD_ROLLBACK",
            Code::RecordSignatureInvalid => "RECORD_SIGNATURE_INVALID",
            Code::RecordTransitionInvalid => "RECORD_TRANSITION_INVALID",
            Code::RecordVersionUnknown => "RECORD_VERSION_UNKNOWN",
            Code::RoleForbidden => "ROLE_FORBIDDEN",
            Code::RoleNotEnforced => "ROLE_NOT_ENFORCED",
            Code::SettingsInvalid => "SETTINGS_INVALID",
            Code::SettingsKeyNotPersonal => "SETTINGS_KEY_NOT_PERSONAL",
            Code::ToolFailed => "TOOL_FAILED",
            Code::ToolMissing => "TOOL_MISSING",
            Code::WideningUnnamed => "WIDENING_UNNAMED",
        }
    }
}

/// A failure: a refusal with its code, or an operating-system error the caller words.
#[derive(Debug)]
pub enum Error {
    Refused { code: Code, path: Option<PathBuf>, detail: String },
    Io { path: PathBuf, error: std::io::Error },
}

impl Error {
    /// The refusal's code; `None` for an operating-system error.
    pub fn code(&self) -> Option<Code> {
        match self {
            Error::Refused { code, .. } => Some(*code),
            Error::Io { .. } => None,
        }
    }
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
            CheckUnknown,
            DomainNotIgnored,
            DomainUnknownEntry,
            GitignoreChanged,
            HeadDetached,
            KeyUnknown,
            LockHeld,
            MarkInvalid,
            NotARepository,
            PolicyAbsent,
            PolicyPending,
            PolicyUncommitted,
            PrivateFolderPermissions,
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
            RecordVersionUnknown,
            RoleForbidden,
            RoleNotEnforced,
            SettingsInvalid,
            SettingsKeyNotPersonal,
            ToolFailed,
            ToolMissing,
            WideningUnnamed,
        ] {
            assert!(list.contains(&format!("`{}`", code.as_str())), "{code:?}");
            let words: String = format!("{code:?}")
                .chars()
                .flat_map(|c| [c.is_uppercase().then_some('_'), Some(c)])
                .flatten()
                .collect();
            assert_eq!(code.as_str(), words.trim_start_matches('_').to_uppercase());
        }
    }
}
