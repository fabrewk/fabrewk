//! Replay (REQ-REG-6, REQ-REG-7, REQ-REG-1's identifiers): every stream read and every event checked, each failure
//! refused by its code and never by a raw error. Readers take no lock and skip names starting with `.`; a signature
//! whose event never reached its place is an interrupted write's leftover and is skipped too.

use super::event::{self, Class, StreamKind};
pub use super::sign::Verify;
use super::{Code, Error, canon, io_at, refuse};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

/// One replayed event.
#[derive(Debug, Clone)]
pub struct Event {
    pub stream: String,
    pub seq: u64,
    pub digest: String,
    pub kind: String,
    pub value: Value,
    pub path: PathBuf,
}

/// The record, replayed: each stream's events in order.
#[derive(Debug, Default)]
pub struct Replay {
    pub streams: BTreeMap<String, Vec<Event>>,
}

/// What a stream's directory holds, by name only.
pub(crate) struct Listing {
    pub events: BTreeMap<u64, Vec<PathBuf>>,
    pub signatures: BTreeSet<PathBuf>,
    pub orphan_signatures: Vec<PathBuf>,
    pub temporary: Vec<PathBuf>,
    pub unknown: Vec<PathBuf>,
}

/// The `.sig` beside an event file.
pub(crate) fn signature_of(event: &Path) -> PathBuf {
    let mut name = event.as_os_str().to_owned();
    name.push(".sig");
    PathBuf::from(name)
}

/// The position named by an event file's name `<seq>-<digest16>.json`.
fn seq_of(name: &str) -> Option<u64> {
    let (seq, rest) = name.split_at_checked(6)?;
    let digest = rest.strip_prefix('-')?.strip_suffix(".json")?;
    let ok = seq.bytes().all(|b| b.is_ascii_digit()) && event::is_hex(digest, 16);
    ok.then(|| seq.parse().ok()).flatten().filter(|&s| s > 0)
}

pub(crate) fn name_matches(path: &Path, digest: &str) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.get(7..23) == Some(&digest[..16]))
}

pub(crate) fn list_stream(dir: &Path) -> Result<Listing, Error> {
    let mut l = Listing {
        events: BTreeMap::new(),
        signatures: BTreeSet::new(),
        orphan_signatures: Vec::new(),
        temporary: Vec::new(),
        unknown: Vec::new(),
    };
    let entries = match fs::read_dir(dir) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(l),
        other => other.map_err(io_at(dir))?,
    };
    let mut sigs = Vec::new();
    for entry in entries {
        let path = entry.map_err(io_at(dir))?.path();
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_owned();
        if path
            .file_name()
            .is_some_and(|n| n.as_encoded_bytes().starts_with(b"."))
        {
            // Skipped by readers; removed by a writer only if it is one of the record's own temporary files.
            let own = name.strip_prefix('.').and_then(|n| n.strip_suffix(".tmp"));
            if own.is_some_and(|n| {
                seq_of(n)
                    .or_else(|| n.strip_suffix(".sig").and_then(seq_of))
                    .is_some()
            }) && path.is_file()
            {
                l.temporary.push(path);
            }
        } else if let Some(seq) = seq_of(&name).filter(|_| path.is_file()) {
            l.events.entry(seq).or_default().push(path);
        } else if name.strip_suffix(".sig").and_then(seq_of).is_some() && path.is_file() {
            sigs.push(path);
        } else {
            l.unknown.push(path);
        }
    }
    let all: BTreeSet<&PathBuf> = l.events.values().flatten().collect();
    for sig in sigs {
        if all.contains(&sig.with_extension("")) {
            l.signatures.insert(sig);
        } else {
            l.orphan_signatures.push(sig);
        }
    }
    l.events.values_mut().for_each(|files| files.sort());
    Ok(l)
}

/// Replays the record under `domain`: `RECORD_ABSENT` if it holds no event, otherwise every stream's first failure.
pub fn replay(domain: &Path, verifier: &dyn Verify) -> Result<Replay, Vec<Error>> {
    let root = domain.join("data/events");
    let absent = || vec![refuse(Code::RecordAbsent, Some(&root), "no event")];
    let entries = match fs::read_dir(&root) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Err(absent()),
        other => other.map_err(|e| vec![io_at(&root)(e)])?,
    };
    let mut names = Vec::new();
    let mut errors = Vec::new();
    for entry in entries {
        let path = match entry {
            Ok(e) => e.path(),
            Err(e) => return Err(vec![io_at(&root)(e)]),
        };
        let name = path.file_name().and_then(|n| n.to_str()).map(str::to_owned);
        match name {
            Some(n) if n.starts_with('.') => {}
            Some(n) if path.is_dir() && event::stream_kind(&n).is_some() => names.push(n),
            _ => errors.push(refuse(Code::RecordBroken, Some(&path), "not a stream")),
        }
    }
    names.sort();
    let mut out = Replay::default();
    for name in names {
        match stream(&root.join(&name), &name, verifier) {
            Ok(events) if events.is_empty() => {}
            Ok(events) => {
                out.streams.insert(name, events);
            }
            Err(e) => errors.push(e),
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    if out.streams.is_empty() {
        return Err(absent());
    }
    Ok(out)
}

fn stream(dir: &Path, name: &str, verifier: &dyn Verify) -> Result<Vec<Event>, Error> {
    let listing = list_stream(dir)?;
    if let Some(path) = listing.unknown.first() {
        return Err(refuse(Code::RecordBroken, Some(path), "not an event file"));
    }
    let mut read = BTreeMap::new();
    for (&seq, files) in &listing.events {
        let mut at_seq = Vec::new();
        for path in files {
            at_seq.push(file(path, name, seq, &listing, verifier)?);
        }
        read.insert(seq, at_seq);
    }
    let mut events = Vec::new();
    for (i, (seq, mut at_seq)) in read.into_iter().enumerate() {
        if seq != i as u64 + 1 {
            return Err(refuse(
                Code::RecordBroken,
                Some(dir),
                format!("no event at {}", i + 1),
            ));
        }
        if at_seq.len() > 1 {
            let kind = event::stream_kind(name);
            let creating = at_seq.iter().filter(|e| {
                event::subject_stream(&e.kind, &e.value["payload"]).as_deref() == Some(name)
            });
            let code = if seq == 1
                && matches!(kind, Some(StreamKind::Stage | StreamKind::Task))
                && creating.count() > 1
            {
                Code::RecordIdCollision
            } else {
                Code::RecordFork
            };
            return Err(refuse(
                code,
                Some(dir),
                format!("{} events at {seq}", at_seq.len()),
            ));
        }
        let e = at_seq.remove(0);
        let prev = events
            .last()
            .map(|p: &Event| Value::String(p.digest.clone()))
            .unwrap_or(Value::Null);
        if e.value["prev"] != prev {
            return Err(refuse(
                Code::RecordBroken,
                Some(&e.path),
                "prev names no event",
            ));
        }
        events.push(e);
    }
    if let (Some(first), Some(StreamKind::Stage | StreamKind::Task)) =
        (events.first(), event::stream_kind(name))
    {
        if !event::is_creation(&first.kind) {
            return Err(refuse(
                Code::RecordTransitionInvalid,
                Some(&first.path),
                "the stream does not begin with its creation",
            ));
        }
        if event::subject_stream(&first.kind, &first.value["payload"]).as_deref() != Some(name) {
            return Err(refuse(
                Code::RecordBroken,
                Some(&first.path),
                "the identifier is not the payload's",
            ));
        }
    }
    if let Some(e) = events.iter().skip(1).find(|e| event::is_creation(&e.kind)) {
        return Err(refuse(
            Code::RecordTransitionInvalid,
            Some(&e.path),
            "a second creation",
        ));
    }
    Ok(events)
}

fn file(
    path: &Path,
    stream: &str,
    seq: u64,
    listing: &Listing,
    verifier: &dyn Verify,
) -> Result<Event, Error> {
    let bytes = fs::read(path).map_err(io_at(path))?;
    let digest = canon::sha256_hex(&bytes);
    if !name_matches(path, &digest) {
        return Err(refuse(
            Code::RecordDigestMismatch,
            Some(path),
            "name and content differ",
        ));
    }
    let value = canon::decode(&bytes)
        .ok_or_else(|| refuse(Code::RecordNoncanonical, Some(path), "not canonical"))?;
    let class =
        event::check(&value, stream, seq).map_err(|(code, why)| refuse(code, Some(path), why))?;
    let sig = signature_of(path);
    match (class, listing.signatures.contains(&sig)) {
        (Class::Person, true) => {
            verifier.verify(value["actor"]["alias"].as_str().unwrap_or(""), &bytes, &sig)?
        }
        (Class::Person, false) => {
            return Err(refuse(
                Code::RecordSignatureInvalid,
                Some(path),
                "no signature",
            ));
        }
        (_, true) => {
            return Err(refuse(
                Code::RecordSignatureInvalid,
                Some(&sig),
                "a signature on an event no person records",
            ));
        }
        (_, false) => {}
    }
    let kind = value["kind"].as_str().unwrap_or("").to_owned();
    Ok(Event {
        stream: stream.to_owned(),
        seq,
        digest,
        kind,
        value,
        path: path.to_path_buf(),
    })
}
