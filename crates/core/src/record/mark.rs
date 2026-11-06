//! The mark (REQ-REG-8, PD-01, PD-02): each machine keeps the last head it saw of every stream in
//! `local/cache/heads.json`, replaced whole. A stream that no longer reaches a marked position is a rollback; a mark
//! that is empty, not canonical, of another shape, or naming a digest its stream does not hold at that position is
//! refused `MARK_INVALID` and only a person removes it. Reading commands compare and never write it.

use super::replay::Replay;
use super::{Code, Error, canon, event, io_at, refuse, store};
use serde_json::{Map, Value, json};
use std::path::{Path, PathBuf};

fn path(domain: &Path) -> PathBuf {
    domain.join("local/cache/heads.json")
}

/// Compares the record with this machine's mark; no mark is no comparison.
pub fn check_mark(domain: &Path, replay: &Replay) -> Result<(), Error> {
    let path = path(domain);
    if !crate::code::unlinked(domain, "local/cache/heads.json")
        || std::fs::symlink_metadata(&path).is_ok_and(|m| !m.is_file())
    {
        return Err(refuse(Code::MarkInvalid, Some(&path), "not a regular file"));
    }
    let bytes = match std::fs::read(&path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        other => other.map_err(io_at(&path))?,
    };
    let invalid = |why: &str| refuse(Code::MarkInvalid, Some(&path), why);
    let value = canon::decode(&bytes).ok_or_else(|| invalid("not canonical JSON"))?;
    let heads = value.as_object().filter(|o| o.len() == 2 && o.get("schema") == Some(&json!(1)));
    let heads = heads.and_then(|o| o.get("heads")?.as_object()).ok_or_else(|| invalid("shape"))?;
    for (stream, head) in heads {
        let seq = head.get("seq").and_then(Value::as_u64).filter(|&s| s > 0);
        let digest = head.get("digest").and_then(Value::as_str).filter(|d| event::is_hex(d, 64));
        let (Some(seq), Some(digest), Some(2)) = (seq, digest, head.as_object().map(Map::len)) else {
            return Err(invalid("shape"));
        };
        if event::stream_kind(stream).is_none() {
            return Err(invalid("stream name"));
        }
        let events = replay.streams.get(stream).map_or(&[][..], Vec::as_slice);
        match events.get(seq as usize - 1) {
            None => {
                return Err(refuse(Code::RecordRollback, Some(&path), format!("{stream} no longer reaches {seq}")));
            }
            Some(e) if e.digest != digest => {
                return Err(invalid("a digest its stream does not hold there"));
            }
            Some(_) => {}
        }
    }
    Ok(())
}

/// Replaces the mark with the heads of `replay` (temporary file, sync, rename).
pub(crate) fn write(domain: &Path, replay: &Replay) -> Result<(), Error> {
    let mut heads = Map::new();
    for (stream, events) in &replay.streams {
        if let Some(last) = events.last() {
            heads.insert(stream.clone(), json!({"seq": last.seq, "digest": last.digest}));
        }
    }
    let bytes = canon::encode(&json!({"schema": 1, "heads": heads})).unwrap_or_default();
    let path = path(domain);
    let dir = path.parent().unwrap_or(domain);
    std::fs::create_dir_all(dir).map_err(io_at(dir))?;
    store::write_atomic(&path, &bytes)
}
