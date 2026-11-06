//! The record (REQ-REG-1 to REQ-REG-11; part 7b, D-047): immutable event files under `data/events/<stream>/`,
//! appended under one lock with a compare-and-swap of each stream's head, replayed with every refusal named by its
//! code, guarded against rollback by a mark per machine and by a walk of the whole Git history, and committed through
//! a temporary index. Signatures are checked by the caller's [`Verify`] (the identity module derives the signers from
//! the cited policy); the transition table holds the stage and task streams (`crate::table`). Nothing here prints: every failure is an [`Error`] that the command surface words from its catalogue.

mod canon;
mod event;
pub mod fault;
mod git;
mod mark;
mod replay;
pub(crate) mod sign;
mod store;
mod time;

pub use canon::{MAX_DEPTH, decode, encode, sha256_hex};
pub use event::{
    Class, Draft, FIELDS, KINDS, Say, StreamKind, class_of, nonce, stream_kind, subject_digest, subject_stream,
};
pub use git::{Publication, Repo, record_branch, uncommitted, walk_history};
pub use mark::check_mark;
pub use replay::{Event, Replay, replay};
pub use sign::{KeySigner, Sign, Verify, carried_key, check};
pub use store::{Head, Prepared, Session};
pub use time::{is_utc_timestamp, now_utc};

pub use crate::code::{Code, Error};
pub(crate) use crate::code::{io_at, refuse};
pub(crate) use event::{is_hex, is_id, is_name};
pub(crate) use git::{check_main_worktree, committed, exclude_file, ignores, signing_key};
pub(crate) use sign::sign_with;
pub(crate) use store::write_atomic;
