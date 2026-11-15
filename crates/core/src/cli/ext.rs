//! Extensions (REQ-EXT-1 to REQ-EXT-5, REQ-CLI-3; AD-5's contract, S-9; part 9h, D-094). `fabrewk <name> …` runs the
//! program `fabrewk-<name>` found in `PATH` when `[extensions.<name>]` permits it and its name is no core command's,
//! confined: the repository read-only at its own path, `.brewstillery/local/` hidden but for its own area
//! `local/ext/<name>/` (writable, its home and `/tmp` inside) and `local/cache/` (read-only), the running `fabrewk` and
//! the program, read-only at their own paths, no network. Its standard output carries only requests, one JSON object
//! per line; once it exits 0 the outer core validates every one and applies them all or none, with the extension's
//! name and digest as actor and no person's signature. Its standard error goes to the person.

use super::write::{self, Plan};
use super::{Cx, Failed, Out, Usage, propose};
use crate::code::{Code, Error, io_at, refuse};
use crate::confine::{self, Access, Cell, End, Limits, Mount, Network, Scratch};
use crate::detect::Space;
use crate::record::{Draft, Replay, fault, sha256_hex};
use crate::settings::{self, Policy, Val};
use serde_json::{Value, json};
use std::ffi::{OsStr, OsString};
use std::io::Write;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// The contract's version (REQ-EXT-2), and the exit status of an extension that does not support it (REQ-CLI-3).
pub const PROTOCOL: u64 = 1;
pub const UNSUPPORTED: i32 = 10;

/// The bytes kept of each of an extension's output channels; past them its cell is ended.
const OUTPUT: u64 = 1 << 20;

/// The name of the extension `args` call, when their first word is a name no core command has.
pub(super) fn called(args: &[OsString]) -> Option<String> {
    let first = args.first()?.to_str()?;
    (!first.starts_with('-') && !super::COMMANDS.iter().any(|c| c.name == first)).then(|| first.to_owned())
}

/// The extensions `policy` permits: each table `[extensions.<name>]`.
pub(super) fn permitted(policy: &Policy) -> impl Iterator<Item = &str> {
    policy.settings.tables.keys().filter_map(|t| t.strip_prefix("extensions.")).filter(|n| !n.contains('.'))
}

/// The program of the extension `name` as `path` finds it, resolved, with its bytes (REQ-EXT-1).
pub(super) fn found(name: &str, path: Option<&OsStr>) -> Option<(PathBuf, Vec<u8>)> {
    let program = crate::detect::search(path, &format!("fabrewk-{name}"))?.canonicalize().ok()?;
    let bytes = std::fs::read(&program).ok()?;
    Some((program, bytes))
}

/// The cell of an extension (`confinement-design.md` 3.4, S-9): see the module's description. `program` is the copy of
/// the bytes whose digest was taken, and the path it is shown at.
fn cell(cx: &Cx, area: &Path, bin: &Path, program: [&Path; 2], name: &str, args: &[OsString]) -> Cell {
    let local = cx.domain.join("local");
    let mut mounts = vec![
        Mount { host: cx.root.clone(), inside: cx.root.clone(), access: Access::ReadOnly },
        Mount { host: local.clone(), inside: local.clone(), access: Access::Hidden },
        Mount { host: area.to_path_buf(), inside: area.to_path_buf(), access: Access::ReadWrite },
    ];
    let cache = local.join("cache");
    if crate::code::unlinked(&cx.domain, "local/cache") && cache.is_dir() {
        mounts.push(Mount { host: cache.clone(), inside: cache, access: Access::ReadOnly });
    }
    for (host, inside) in [(bin, bin), (program[0], program[1])] {
        mounts.push(Mount { host: host.to_path_buf(), inside: inside.to_path_buf(), access: Access::ReadOnly });
    }
    let mut env: Vec<(OsString, OsString)> =
        crate::adapter::CELL_ENV.iter().map(|(k, v)| (k.into(), v.into())).collect();
    env.extend([
        ("FABREWK_PROTOCOL".into(), PROTOCOL.to_string().into()),
        ("FABREWK_BIN".into(), bin.into()),
        ("FABREWK_ROOT".into(), cx.root.clone().into()),
        (super::EXTENSION_MARKER.into(), name.into()),
    ]);
    Cell {
        home: area.join("home"),
        tmp: area.join("tmp"),
        mounts,
        network: Network::None,
        nested: false,
        env,
        cwd: cx.root.clone(),
        argv: std::iter::once(program[1].as_os_str().to_owned()).chain(args.iter().cloned()).collect(),
    }
}

/// One request, as the outer core applies it.
enum Request {
    Note(String),
    Propose(Value),
}

/// Every line of `out` as a request (REQ-EXT-3), else `EXTENSION_OUTPUT_INVALID` naming the line and why, never its
/// bytes; a member named twice is no request.
fn requests(out: &[u8], file: &Path) -> Result<Vec<(usize, Request)>, Error> {
    let invalid = |n: usize, why: &str| refuse(Code::ExtensionOutputInvalid, Some(file), format!("line {n}: {why}"));
    let text = std::str::from_utf8(out).map_err(|_| invalid(0, "the output is not UTF-8"))?;
    let mut all = Vec::new();
    for (i, line) in text.split_terminator('\n').enumerate() {
        let n = i + 1;
        let v = crate::adapter::json(line).ok_or_else(|| invalid(n, "not one JSON object"))?;
        let o = v.as_object().ok_or_else(|| invalid(n, "not one JSON object"))?;
        if o.get("protocol").and_then(Value::as_u64) != Some(PROTOCOL) {
            return Err(invalid(n, "protocol is not 1"));
        }
        let has = |keys: &[&str]| o.len() == keys.len() + 2 && keys.iter().all(|k| o.contains_key(*k));
        let request = match o.get("request").and_then(Value::as_str) {
            Some("note") if has(&["text"]) => match o["text"].as_str().filter(|t| !t.trim().is_empty()) {
                Some(t) => Request::Note(t.to_owned()),
                None => return Err(invalid(n, "a note's text is not a text")),
            },
            Some("propose") if has(&["stage", "tasks"]) => {
                Request::Propose(json!({"stage": o["stage"], "tasks": o["tasks"]}))
            }
            Some("note" | "propose") => return Err(invalid(n, "a request with other fields than its kind's")),
            _ => return Err(invalid(n, "request is not note or propose")),
        };
        all.push((n, request));
    }
    Ok(all)
}

fn usage(name: &str) -> Failed {
    Failed::Usage(Usage { key: "usage.command", word: name.to_owned(), command: None })
}

/// The minutes and network the policy gives the extension `name`.
fn limits(policy: &Policy, name: &str) -> Result<(u64, bool), Failed> {
    let eff = settings::effective(policy, None, None, &[], &[])?;
    let minutes = match eff.values.get(&format!("extensions.{name}.minutes")) {
        Some((Val::Int(m), _)) => u64::try_from(*m).unwrap_or(0),
        _ => 30,
    };
    Ok((minutes, eff.values.get(&format!("extensions.{name}.network")).map(|v| &v.0) == Some(&Val::Bool(true))))
}

/// Refuses an extension whose digest is not the one the policy pins (`EXTENSION_DIGEST`).
fn pinned(policy: &Policy, name: &str, digest: &str) -> Result<(), Error> {
    let pin = match policy.settings.values.get(&format!("extensions.{name}.digest")).map(|e| &e.value) {
        Some(Val::Str(pin)) => pin,
        _ => return Ok(()),
    };
    if pin == digest {
        return Ok(());
    }
    let why = format!("fabrewk-{name}'s digest is {digest}; the policy in force pins {pin}");
    Err(refuse(Code::ExtensionDigest, None, why))
}

/// Its own area, `local/ext/<name>/`, with its home and `/tmp` (`0700`), reached through no link, and its lock,
/// `local/ext/<name>.lock`, which no cell reaches: held for the whole run, so that no other run of the extension,
/// confined with write access to the area, can swap its home or `/tmp` for a link between their check and their
/// mount (P9H-1). Another run holding it is `LOCK_HELD`.
fn area(cx: &Cx, name: &str) -> Result<(PathBuf, std::fs::File), Error> {
    let area = cx.domain.join("local/ext").join(name);
    let rels = [format!("local/ext/{name}.lock"), format!("local/ext/{name}/home"), format!("local/ext/{name}/tmp")];
    let linked = || !rels.iter().all(|r| crate::code::unlinked(&cx.domain, r));
    let refused = || refuse(Code::ToolFailed, Some(&area), "a link on the extension's area's path; not followed");
    if linked() {
        return Err(refused());
    }
    let parent = cx.domain.join("local/ext");
    std::fs::create_dir_all(&parent).map_err(io_at(&parent))?;
    let path = parent.join(format!("{name}.lock"));
    let lock = std::fs::File::options().create(true).append(true).mode(0o600).open(&path).map_err(io_at(&path))?;
    match lock.try_lock() {
        Ok(()) => {}
        Err(std::fs::TryLockError::WouldBlock) => {
            return Err(refuse(Code::LockHeld, Some(&path), format!("the extension {name} runs in another command")));
        }
        Err(std::fs::TryLockError::Error(e)) => return Err(io_at(&path)(e)),
    }
    for dir in [area.join("home"), area.join("tmp")] {
        std::fs::create_dir_all(&dir).map_err(io_at(&dir))?;
    }
    if linked() {
        return Err(refused());
    }
    for dir in [area.clone(), area.join("home"), area.join("tmp")] {
        std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700)).map_err(io_at(&dir))?;
    }
    Ok((area, lock))
}

/// Runs the extension `name` with `args` (see the module's description).
pub(super) fn run(cx: &Cx, name: &str, args: &[OsString]) -> Result<Out, Failed> {
    // A name that is no extension's, or whose program is absent, is an unknown command; a present one whose permission
    // cannot be read is the record's refusal.
    let path = std::env::var_os("PATH");
    let found = crate::record::is_name(name).then(|| found(name, path.as_deref())).flatten();
    let Some((program, bytes)) = found else { return Err(usage(name)) };
    let digest = sha256_hex(&bytes);
    let Some(policy) = cx.policy.as_ref() else {
        return Err(match &cx.record {
            Err(errors) if errors.iter().any(|e| e.code() != Some(Code::RecordAbsent)) => errors.clone().into(),
            _ => usage(name),
        });
    };
    if !permitted(policy).any(|n| n == name) {
        return Err(usage(name));
    }
    pinned(policy, name, &digest)?;
    let (minutes, network) = limits(policy, name)?;
    if network {
        let why =
            format!("the extension {name} has network, which this backend cannot confine without letting it nest");
        return Err(refuse(Code::ConfinementUnavailable, None, why).into());
    }
    let machine = crate::machine::machine_file(&cx.folder()?)?;
    let eff = settings::effective(policy, None, machine.as_ref(), &[], &[])?;
    let mut detection = crate::detect::detect(&cx.domain, &eff, &cx.modules, path.as_deref());
    let programs = confine::available(cx.backend, &mut detection, &cx.domain)?;
    let backend = cx.backend.expect("an available backend");
    let bin = std::env::current_exe().and_then(|p| p.canonicalize()).map_err(io_at(Path::new("fabrewk")))?;
    // Held until the requests are applied, so that no other run of it starts meanwhile.
    let (area, _lock) = area(cx, name)?;
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos());
    let run_name = format!("ext-{name}-{}-{nanos}", std::process::id());
    let scratch = Scratch::create(&cx.domain, &run_name)?;
    let evidence = confine::evidence(&cx.domain, &run_name)?;
    let time = Duration::from_secs(minutes.saturating_mul(fault::minute()));
    // What runs is the bytes whose digest was taken, whatever happens to the file meanwhile.
    let copy = scratch.root.join("program");
    std::fs::File::options()
        .write(true)
        .create_new(true)
        .mode(0o700)
        .open(&copy)
        .and_then(|mut f| f.write_all(&bytes))
        .map_err(io_at(&copy))?;
    let cell = cell(cx, &area, &bin, [&copy, &program], name, args);
    let ran = confine::run(backend, &programs, &cell, &scratch, &evidence, Limits { time, bytes: OUTPUT });
    let ran = match ran {
        Err(u) if !u.ended => return Err(u.error.into()),
        Err(u) => {
            scratch.remove()?;
            return Err(u.error.into());
        }
        Ok(ran) => ran,
    };
    scratch.remove()?;
    // Its messages reach the person as they would any text Fabrewk shows: no hidden character acts on the terminal.
    let messages = std::fs::read(&ran.stderr.path).map_err(io_at(&ran.stderr.path))?;
    let _ = std::io::stderr().write_all(super::text::printable(&String::from_utf8_lossy(&messages)).as_bytes());
    let tool = format!("the extension {name}");
    match ran.end {
        End::Exited(0) => {}
        End::Exited(UNSUPPORTED) => {
            let text = cx.t("extension.unsupported", &[("name", name)]) + "\n";
            return Ok(Out { doc: Value::Null, text, error: true, exit: UNSUPPORTED });
        }
        End::Exited(126 | 127) => {
            let dir = ran.stdout.path.parent().unwrap_or(&evidence);
            let why = format!(
                "{tool} could not start: an interpreter or a support file it needs may lie outside the system \
                 directories and the repository, which its cell does not show; its output is kept in {}",
                dir.display()
            );
            return Err(refuse(Code::ToolFailed, Some(dir), why).into());
        }
        _ => return Err(ran.failure(&tool).into()),
    }
    let out = std::fs::read(&ran.stdout.path).map_err(io_at(&ran.stdout.path))?;
    let requests = requests(&out, &ran.stdout.path)?;
    if requests.is_empty() {
        return Ok(Out::new(Value::Null, cx.t("extension.none", &[("name", name)]) + "\n"));
    }
    let actor = json!({"kind": "extension", "name": name, "digest": digest});
    let file = ran.stdout.path.clone();
    let plan = |record: &Replay| -> Result<Plan<Vec<String>>, Failed> {
        let policy = write::state(cx, record)?.policy;
        // The policy may have changed while the extension ran: its requests apply only under one that permits it at
        // the digest it ran with.
        if !permitted(&policy).any(|n| n == name) {
            let why = format!("the policy in force no longer permits the extension {name}");
            return Err(refuse(Code::ExtensionDigest, Some(&file), why).into());
        }
        pinned(&policy, name, &digest)?;
        let mut events = Vec::new();
        let mut lines = Vec::new();
        // The identifiers every request of this output takes, so that none is drawn twice (P9H-2).
        let taken = propose::Taken::default();
        for (n, request) in &requests {
            let at = |f: Failed| match f {
                Failed::Refused(mut errors) => {
                    for e in &mut errors {
                        if let Error::Refused { detail, .. } = e {
                            *detail = format!("line {n}: {detail}");
                        }
                    }
                    Failed::Refused(errors)
                }
                f => f,
            };
            match request {
                Request::Note(text) => {
                    let payload = json!({"text": text});
                    let draft = Draft {
                        kind: "extension-note".into(),
                        actor: actor.clone(),
                        policy: policy.digest.clone(),
                        instance: String::new(),
                        say: None,
                        payload,
                    };
                    events.push((format!("ext-{name}"), draft));
                    lines.push(cx.t("extension.noted", &[("name", name)]));
                }
                Request::Propose(file_value) => {
                    let p = propose::plan(cx, &file, file_value, record, &actor, &taken).map_err(at)?;
                    events.extend(p.events);
                    lines.push(propose::lines(cx, &p.out));
                }
            }
        }
        // Its backend's programs passed their probe or a use in this command: a use the detection kept marks.
        let used: Vec<_> = backend.programs.iter().map(|p| (Space::Program, p.executable.to_owned())).collect();
        let then: write::Then = Box::new(move |a, prepared| {
            a.used.extend(used);
            for (stream, p) in prepared {
                a.write(stream, p)?;
            }
            Ok(Value::Null)
        });
        Ok(Plan { events, signer: None, sealed: false, view: None, alambique: None, then: Some(then), out: lines })
    };
    let (lines, written) = write::run(cx, plan)?;
    let mut text = cx.t("extension.applied", &[("name", name), ("digest", &digest)]) + "\n";
    for l in lines {
        text += &format!("  {}\n", l.trim_end().replace('\n', "\n  "));
    }
    let mut doc = Value::Null;
    text += &super::adopt::written_lines(cx, &written, &mut doc);
    Ok(Out::new(Value::Null, text))
}
