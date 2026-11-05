//! The record (REQ-REG-1 to REQ-REG-11; part 7b, D-047): immutable event files under `data/events/<stream>/`,
//! appended under one lock with a compare-and-swap of each stream's head, replayed with every refusal named by its
//! code, guarded against rollback by a mark per machine and by a walk of the whole Git history, and committed through
//! a temporary index. Signers come from the caller (7c derives them from the cited policy); the transition table is
//! 7d's. Nothing here prints: every failure is an [`Error`] that the command surface words from its catalogue.

mod canon;
mod event;
pub mod fault;
mod git;
mod mark;
mod replay;
mod sign;
mod store;
mod time;

use std::path::{Path, PathBuf};

pub use canon::{MAX_DEPTH, decode, encode, sha256_hex};
pub use event::{
    Class, Draft, FIELDS, KINDS, Say, StreamKind, class_of, nonce, stream_kind, subject_stream,
};
pub use git::{Publication, Repo, record_branch, uncommitted, walk_history};
pub use mark::check_mark;
pub use replay::{Event, Replay, replay};
pub use sign::{KeySigner, Sign, SshVerifier, Verify};
pub use store::{Head, Session};
pub use time::{is_utc_timestamp, now_utc};

/// The refusal codes the record raises (core-specification section 7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Code {
    CheckUnknown,
    HeadDetached,
    LockHeld,
    MarkInvalid,
    NotARepository,
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
    ToolFailed,
    ToolMissing,
}

impl Code {
    /// The code as the catalogue and `--json` name it.
    pub const fn as_str(self) -> &'static str {
        match self {
            Code::CheckUnknown => "CHECK_UNKNOWN",
            Code::HeadDetached => "HEAD_DETACHED",
            Code::LockHeld => "LOCK_HELD",
            Code::MarkInvalid => "MARK_INVALID",
            Code::NotARepository => "NOT_A_REPOSITORY",
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
            Code::ToolFailed => "TOOL_FAILED",
            Code::ToolMissing => "TOOL_MISSING",
        }
    }
}

/// A failure of the record: a refusal with its code, or an operating-system error the caller words.
#[derive(Debug)]
pub enum Error {
    Refused {
        code: Code,
        path: Option<PathBuf>,
        detail: String,
    },
    Io {
        path: PathBuf,
        error: std::io::Error,
    },
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
    Error::Refused {
        code,
        path: path.map(Path::to_path_buf),
        detail: detail.into(),
    }
}

pub(crate) fn io_at(path: &Path) -> impl FnOnce(std::io::Error) -> Error + '_ {
    move |error| Error::Io {
        path: path.to_path_buf(),
        error,
    }
}

#[cfg(test)]
mod tests {
    use super::Code::*;

    #[test]
    fn codes_are_named_as_the_specification_lists_them() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../.brewstillery/bootstrap/core-specification.md"
        );
        let spec = std::fs::read_to_string(path).unwrap();
        let list = &spec[spec.find("All codes, in one list:").unwrap()..];
        let list = &list[..list.find("\n\n").unwrap()];
        for code in [
            CheckUnknown,
            HeadDetached,
            LockHeld,
            MarkInvalid,
            NotARepository,
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
            ToolFailed,
            ToolMissing,
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
