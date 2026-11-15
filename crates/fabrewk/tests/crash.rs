//! The crash suite over F-1 (core-specification 11.5; PD-08's property, 6.41's test; part 9i), built only with the
//! feature `fault-injection`: the binary is stopped (`abort`, as a kill stops it) at every write point of every write
//! command of F-1, before and after each event, signature, mark, view, commit, ref update and working tree update.
//! After each stop: the record replays or is refused by a code, never by a raw error; the next write command leaves no
//! task `running`; nothing is dispatched without a person's move recorded after the stop; no event already committed
//! is lost. F-1 runs whole once per worker, keeping its fixture as it stood before each command, and each stop starts
//! from there, at the same paths, since a fixture holds its own paths. These tests run real cells and need `bwrap`,
//! `pasta` and the network.

mod common;

use common::flow::*;
use common::*;
use fabrewk_core::identity::PolicyVerifier;
use fabrewk_core::record::replay;
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

/// The workers, each with a fixture of its own.
const WORKERS: usize = 3;

/// Copies `from` to `to` whole, modes and links as they are; `to` must not exist.
fn copy(from: &Path, to: &Path) {
    let ok = Command::new("cp").arg("-a").arg(from).arg(to).status().unwrap().success();
    assert!(ok, "cp -a {from:?} {to:?}");
}

/// Puts the fixture back as it stood before a command.
fn restore(snapshot: &Path, base: &Path) {
    open(base);
    let _ = std::fs::remove_dir_all(base);
    copy(snapshot, base);
}

/// Gives the owner access to every directory under `dir`, which a cell may have closed.
fn open(dir: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let Ok(meta) = std::fs::symlink_metadata(dir) else { return };
    if meta.is_dir() {
        let _ = std::fs::set_permissions(dir, std::fs::Permissions::from_mode(meta.permissions().mode() | 0o700));
        for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            open(&entry.path());
        }
    }
}

/// The event files the record branch's head holds.
fn committed(fx: &Fx) -> BTreeSet<String> {
    let tree = fx.git(&["ls-tree", "-r", "--name-only", "HEAD", "--", ".brewstillery/data/events"]);
    tree.lines().filter(|l| l.ends_with(".json")).map(str::to_owned).collect()
}

/// Every task's attempts started, by stream.
fn attempts(fx: &Fx) -> usize {
    let mut n = 0;
    for stream in std::fs::read_dir(fx.domain.join("data/events")).into_iter().flatten().flatten() {
        let name = stream.file_name().into_string().unwrap();
        if name.starts_with("task-") {
            n += fx.events(&name).iter().filter(|e| e["kind"] == "attempt-started").count();
        }
    }
    n
}

/// A command's document lists its findings or refusals, each with a code.
fn coded(ran: &Ran, list: &str, what: &str) {
    let doc: Value = serde_json::from_str(&ran.stdout).unwrap_or_else(|e| panic!("{what}: {e}: {ran:?}"));
    for f in doc[list].as_array().into_iter().flatten() {
        assert!(f["code"].as_str().is_some_and(|c| !c.is_empty()), "{what}: {f}");
    }
}

/// What a stop left, checked (see the module's description).
fn after_stop(fx: &Fx, what: &str) {
    // The record replays, or is refused by codes.
    let running = match replay(&fx.domain, &PolicyVerifier) {
        Ok(r) => fabrewk_core::table::current(&r).unwrap().values().any(|s| *s == "running"),
        Err(errors) => {
            assert!(errors.iter().all(|e| e.code().is_some()), "{what}: {errors:?}");
            false
        }
    };
    let ran = fx.run(&["check", "--json"]);
    assert!(matches!(ran.status, 0 | 3), "{what}: {ran:?}");
    coded(&ran, "findings", what);
    let (kept, dispatched) = (committed(fx), attempts(fx));
    // The next write command, which a person makes and which dispatches nothing: a task left running is recovered.
    let ran = fx.run(&["discard", "p-00000000", "--say", "Descártala.", "--json"]);
    assert_eq!(ran.status, 1, "{what}: {ran:?}");
    coded(&ran, "refusals", what);
    if let Ok(r) = replay(&fx.domain, &PolicyVerifier) {
        let states = fabrewk_core::table::current(&r).unwrap();
        assert!(states.values().all(|s| *s != "running"), "{what}: {states:?}");
        if running {
            let halted = r
                .streams
                .values()
                .flatten()
                .any(|e| e.kind == "task-halted" && e.value["payload"]["code"] == "OUTCOME_UNKNOWN");
            assert!(halted, "{what}: a running task was not halted");
        }
    }
    assert_eq!(attempts(fx), dispatched, "{what}: dispatched without a person's move");
    let now = committed(fx);
    assert!(kept.is_subset(&now), "{what}: lost {:?}", kept.difference(&now).collect::<Vec<_>>());
    for file in &kept {
        assert!(fx.root.join(file).is_file(), "{what}: {file} is not on disk");
    }
    let ran = fx.run(&["check", "--json"]);
    assert!(matches!(ran.status, 0 | 3), "{what}: {ran:?}");
    coded(&ran, "findings", what);
}

/// One worker's fixture: F-1 run whole, with the fixture kept before each write command and that command's words.
struct Worker {
    fx: Fx,
    steps: Vec<(PathBuf, Vec<String>)>,
}

fn worker(n: usize) -> Worker {
    let fx = fixture(&format!("crash-{n}"), "published");
    let base = fx.root.parent().unwrap().to_path_buf();
    let mut steps = Vec::new();
    f1(&fx, &mut |args, _| {
        let snapshot =
            base.with_file_name(format!("{}-before-{}", base.file_name().unwrap().to_str().unwrap(), steps.len()));
        copy(&base, &snapshot);
        let words: Vec<String> = args.iter().map(|a| a.to_string()).chain(["--json".to_owned()]).collect();
        let ran = fx.run(&words.iter().map(String::as_str).collect::<Vec<_>>());
        assert_eq!(ran.status, 0, "{args:?}: {ran:?}");
        steps.push((snapshot, words));
        ran.doc()
    });
    Worker { fx, steps }
}

#[test]
fn the_crash_suite_over_f_1() {
    let workers: Vec<Worker> = std::thread::scope(|s| {
        let handles: Vec<_> = (0..WORKERS).map(|n| s.spawn(move || worker(n))).collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let commands = workers[0].steps.len();
    assert!(commands >= 10, "{commands} write commands");
    // The commands, the heaviest (the runs) first, taken by whichever worker is free.
    let mut queue: Vec<usize> = (0..commands).collect();
    queue.sort_by_key(|i| !workers[0].steps[*i].1[0].starts_with("run"));
    let queue = Mutex::new(queue.into_iter());
    let points = Mutex::new(Vec::new());
    std::thread::scope(|s| {
        for w in &workers {
            let (queue, points) = (&queue, &points);
            s.spawn(move || {
                let base = w.fx.root.parent().unwrap();
                while let Some(i) = queue.lock().unwrap().next() {
                    let (snapshot, words) = &w.steps[i];
                    let args: Vec<&str> = words.iter().map(String::as_str).collect();
                    let mut n = 1;
                    loop {
                        restore(snapshot, base);
                        let ran = w.fx.run_env(&args, &[("FABREWK_FAULT_ABORT_AT", &n.to_string())]);
                        // A command that ends by itself has passed fewer points than `n`: all were stopped at.
                        if ran.status != -1 {
                            assert_eq!(ran.status, 0, "{words:?} at {n}: {ran:?}");
                            break;
                        }
                        after_stop(&w.fx, &format!("{words:?} stopped at point {n}"));
                        n += 1;
                    }
                    points.lock().unwrap().push((words[0].clone(), n - 1));
                }
            });
        }
    });
    let points = points.into_inner().unwrap();
    eprintln!("write points stopped at, by command: {points:?}");
    assert_eq!(points.len(), commands);
    assert!(points.iter().all(|(_, n)| *n > 0), "{points:?}");
    for w in &workers {
        for (snapshot, _) in &w.steps {
            open(snapshot);
            std::fs::remove_dir_all(snapshot).unwrap();
        }
    }
}
