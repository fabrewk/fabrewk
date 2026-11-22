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
use std::collections::BTreeMap;
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

/// The event files the record branch's head holds, each with its blob.
fn committed(fx: &Fx) -> BTreeMap<String, String> {
    let tree = fx.git(&["ls-tree", "-r", "HEAD", "--", ".brewstillery/data/events"]);
    let entry = |l: &str| {
        let (meta, path) = l.split_once('\t').unwrap();
        (path.to_owned(), meta.split(' ').nth(2).unwrap().to_owned())
    };
    tree.lines().map(entry).filter(|(p, _)| p.ends_with(".json")).collect()
}

/// `kept` is in `now`, each event with the same bytes (P9I-3).
fn holds(now: &BTreeMap<String, String>, kept: &BTreeMap<String, String>, what: &str) {
    for (path, blob) in kept {
        assert_eq!(now.get(path), Some(blob), "{what}: {path} lost or changed");
    }
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

/// A command's document lists its findings or refusals, each with a code, and at least one when it did not exit 0: a
/// raw error is no refusal (P9I-6).
fn coded(ran: &Ran, list: &str, what: &str) {
    let doc: Value = serde_json::from_str(&ran.stdout).unwrap_or_else(|e| panic!("{what}: {e}: {ran:?}"));
    let entries = doc[list].as_array().unwrap_or_else(|| panic!("{what}: no {list}: {ran:?}"));
    assert!(ran.status == 0 || !entries.is_empty(), "{what}: {ran:?}");
    for f in entries {
        assert!(f["code"].as_str().is_some_and(|c| !c.is_empty()), "{what}: {f}");
    }
}

/// The processes whose command line names `base`: what a stopped command of this fixture left running.
fn left(base: &Path) -> Vec<String> {
    let base = base.to_str().unwrap();
    let mut out = Vec::new();
    for entry in std::fs::read_dir("/proc").into_iter().flatten().flatten() {
        let line = std::fs::read(entry.path().join("cmdline")).unwrap_or_default();
        let line = String::from_utf8_lossy(&line).replace('\0', " ");
        if line.contains(base)
            && entry.file_name().to_str().is_some_and(|n| n.parse::<u32>().is_ok_and(|n| n != std::process::id()))
        {
            out.push(line);
        }
    }
    out
}

/// What a stop left, checked (see the module's description); `before` is what the record branch held before the
/// command ran.
fn after_stop(fx: &Fx, before: &BTreeMap<String, String>, what: &str) {
    // No process of the stopped command is left: its cells and helpers end with it, or none was running.
    let base = fx.root.parent().unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while !left(base).is_empty() {
        assert!(std::time::Instant::now() < deadline, "{what}: left running: {:?}", left(base));
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    // The record replays, or is refused by codes.
    let first = replay(&fx.domain, &PolicyVerifier);
    let replayed_before = first.is_ok();
    let running = match first {
        Ok(r) => fabrewk_core::table::current(&r).unwrap().values().any(|s| *s == "running"),
        Err(errors) => {
            assert!(errors.iter().all(|e| e.code().is_some()), "{what}: {errors:?}");
            false
        }
    };
    let ran = fx.run(&["check", "--json"]);
    assert!(matches!(ran.status, 0 | 3), "{what}: {ran:?}");
    coded(&ran, "findings", what);
    // Nothing committed before the command, nor by it before it stopped, is lost or changed.
    let (kept, dispatched) = (committed(fx), attempts(fx));
    holds(&kept, before, what);
    // The next write command, which a person makes and which dispatches nothing: a task left running is recovered.
    let ran = fx.run(&["discard", "p-00000000", "--say", "Descártala.", "--json"]);
    assert_eq!(ran.status, 1, "{what}: {ran:?}");
    coded(&ran, "refusals", what);
    let replayed = replay(&fx.domain, &PolicyVerifier);
    // A recovery never makes a record that replayed refuse.
    assert!(replayed.is_ok() || !replayed_before, "{what}: {replayed:?}");
    if let Ok(r) = replayed {
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
    holds(&now, &kept, what);
    // And each is on disk with the bytes committed.
    let paths: Vec<&String> = now.keys().collect();
    let mut c = Command::new("git");
    isolate(&mut c, &fx.home, None);
    let mut child = c
        .current_dir(&fx.root)
        .args(["hash-object", "--stdin-paths"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let list: String = paths.iter().map(|p| format!("{p}\n")).collect();
    std::io::Write::write_all(&mut child.stdin.take().unwrap(), list.as_bytes()).unwrap();
    let out = String::from_utf8(child.wait_with_output().unwrap().stdout).unwrap();
    let disk: Vec<&str> = out.lines().collect();
    let blobs: Vec<&str> = now.values().map(String::as_str).collect();
    assert_eq!(disk, blobs, "{what}: an event on disk is not what was committed");
    let ran = fx.run(&["check", "--json"]);
    assert!(matches!(ran.status, 0 | 3), "{what}: {ran:?}");
    coded(&ran, "findings", what);
}

/// One worker's fixture: F-1 run whole, with the fixture kept before each write command and that command's words.
struct Worker {
    fx: Fx,
    steps: Vec<(PathBuf, Vec<String>, BTreeMap<String, String>)>,
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
        let before = committed(&fx);
        let ran = fx.run(&words.iter().map(String::as_str).collect::<Vec<_>>());
        assert_eq!(ran.status, 0, "{args:?}: {ran:?}");
        steps.push((snapshot, words, before));
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
                    let (snapshot, words, before) = &w.steps[i];
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
                        after_stop(&w.fx, before, &format!("{words:?} stopped at point {n}"));
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
        for (snapshot, _, _) in &w.steps {
            open(snapshot);
            std::fs::remove_dir_all(snapshot).unwrap();
        }
    }
}

/// The crash suite over `apply` (D-097 point 7; part 10b): in each mode, the binary is stopped at every write point of
/// `apply`, the record commit's between the branch's move and the index's update among them. After each stop: the
/// record replays or is refused by codes; `check` reports by codes; every other write command refuses with a code,
/// `INSTALLATION_INCOMPLETE` once `assessed` is written and the installation is not complete; and the same `apply`, run
/// again, finishes it: `check` then passes, the project stream holds `apply`'s four events once, and in the modes that
/// commit the record branch holds every file of the domain as it is on disk.
#[test]
fn the_crash_suite_over_apply() {
    std::thread::scope(|s| {
        for mode in ["published", "own-repository", "unpublished"] {
            s.spawn(move || {
                let (fx, options) = Fx::installable(&format!("crash-apply-{mode}"), mode, None);
                // An entry file of the person's, executable: its mode stays whatever the stop (P10B-3).
                use std::os::unix::fs::PermissionsExt;
                std::fs::write(fx.root.join("AGENTS.md"), "# Ours\n").unwrap();
                std::fs::set_permissions(fx.root.join("AGENTS.md"), std::fs::Permissions::from_mode(0o755)).unwrap();
                fx.commit("entry");
                let base = fx.root.parent().unwrap().to_path_buf();
                let snapshot = base.with_file_name(format!("{}-before", base.file_name().unwrap().to_str().unwrap()));
                copy(&base, &snapshot);
                let assessed = fx.run(
                    &[&["assess", "--json"][..], &options.iter().map(String::as_str).collect::<Vec<_>>()].concat(),
                );
                let digest = assessed.doc()["digest"].as_str().unwrap().to_owned();
                let mut n = 1;
                loop {
                    restore(&snapshot, &base);
                    let what = format!("apply in {mode} stopped at point {n}");
                    let ran = fx.apply_digest(&digest, &options, &[("FABREWK_FAULT_ABORT_AT", &n.to_string())]);
                    if ran.status != -1 {
                        assert_eq!(ran.status, 0, "{what}: {ran:?}");
                        break;
                    }
                    let first = replay(&fx.domain, &PolicyVerifier);
                    if let Err(errors) = &first {
                        assert!(errors.iter().all(|e| e.code().is_some()), "{what}: {errors:?}");
                    }
                    let ran = fx.run(&["check", "--json"]);
                    assert!(matches!(ran.status, 0 | 3), "{what}: {ran:?}");
                    coded(&ran, "findings", &what);
                    let ran = fx.run(&["discard", "p-00000000", "--say", "Descártala.", "--json"]);
                    assert_eq!(ran.status, 1, "{what}: {ran:?}");
                    coded(&ran, "refusals", &what);
                    let opened = fx.events("project").first().is_some_and(|e| e["kind"] == "assessed");
                    let complete =
                        fx.events("project").iter().any(|e| e["kind"] == "installed") && fx.run(&["check"]).status == 0;
                    if opened && !complete {
                        assert_eq!(ran.doc()["refusals"][0]["code"], "INSTALLATION_INCOMPLETE", "{what}: {ran:?}");
                    }
                    // What the stopped apply wrote whole is not written again: only what is missing is (point 7).
                    use std::os::unix::fs::MetadataExt;
                    let plan = fx.events("project").into_iter().find(|e| e["kind"] == "assessed");
                    let files = plan.map(|e| e["payload"]["plan"]["files"].clone()).unwrap_or_default();
                    let whole: Vec<(PathBuf, u64)> = (files.as_array().into_iter().flatten())
                        .filter_map(|f| {
                            let path = fx.root.join(f["path"].as_str()?);
                            let bytes = std::fs::read(&path).ok()?;
                            let ino = std::fs::metadata(&path).ok()?.ino();
                            (bytes == f["content"].as_str()?.as_bytes()).then_some((path, ino))
                        })
                        .collect();
                    // The same apply finishes it.
                    let again = fx.apply_digest(&digest, &options, &[]);
                    assert_eq!(again.status, 0, "{what}: {again:?}");
                    for (path, ino) in &whole {
                        assert_eq!(std::fs::metadata(path).unwrap().ino(), *ino, "{what}: {}", path.display());
                    }
                    let check = fx.run(&["check"]);
                    assert_eq!(check.status, 0, "{what}: {check:?}");
                    let agents = std::fs::metadata(fx.root.join("AGENTS.md")).unwrap().permissions().mode();
                    assert_eq!(agents & 0o777, 0o755, "{what}");
                    let kinds: Vec<Value> = fx.events("project").iter().map(|e| e["kind"].clone()).collect();
                    assert_eq!(kinds, ["assessed", "publication-decided", "policy-adopted", "installed"], "{what}");
                    let repo = match mode {
                        "published" => Some(fx.root.clone()),
                        "own-repository" => Some(fx.domain.clone()),
                        _ => None,
                    };
                    if let Some(repo) = repo {
                        // Every file of the domain is committed at the branch's tip with its bytes on disk, the
                        // person's index aside: a stop after the branch moved leaves the record's own entries to the
                        // next write command (`sync_index`), as for every write command.
                        let at = ["-C", repo.to_str().unwrap()];
                        let mut files = Vec::new();
                        domain_files(&fx.domain, &mut files);
                        for f in files {
                            let rel = f.strip_prefix(&repo).unwrap().to_str().unwrap().to_owned();
                            let held = fx.git(&[&at[..], &["rev-parse", &format!("HEAD:{rel}")]].concat());
                            let disk = fx.git(&[&at[..], &["hash-object", "--no-filters", "--", &rel]].concat());
                            assert_eq!(held, disk, "{what}: {rel}");
                        }
                    }
                    n += 1;
                }
                eprintln!("apply in {mode}: {} write points stopped at", n - 1);
                assert!(n > 10, "{mode}: {n}");
                open(&snapshot);
                std::fs::remove_dir_all(&snapshot).unwrap();
            });
        }
    });
}

/// The crash suite over F-4's `apply` (D-113 point 6; part 11a): the binary is stopped at every write point before the
/// stream `bootstrap`, at each of its first event's, at each of an entry's in its middle, of its last entry's and of
/// `bootstrap-imported`'s, and at every point after it, the record commit's among them. After each stop the record
/// replays or is refused by codes, `check` reports by codes, a write command refuses `INSTALLATION_INCOMPLETE`, and the
/// same `apply` finishes it: `check` passes, the stream holds the import once, its texts joined `decisions.md`, and the
/// record branch holds every file of the domain as it is on disk.
#[test]
fn the_crash_suite_over_the_bootstrap_s_import() {
    let (fx, options) = common::import::importable("crash-import", true);
    let base = fx.root.parent().unwrap().to_path_buf();
    let snapshot = base.with_file_name(format!("{}-before", base.file_name().unwrap().to_str().unwrap()));
    copy(&base, &snapshot);
    let decisions = std::fs::read_to_string(fx.domain.join("bootstrap/decisions.md")).unwrap();
    let opts: Vec<&str> = options.iter().map(String::as_str).collect();
    let doc = fx.run(&[&["assess", "--json"][..], &opts].concat()).doc();
    let digest = doc["digest"].as_str().unwrap().to_owned();
    // The stream's events: the opening and each entry, then `bootstrap-imported`; each written at three points, the
    // first at four, its directory's made before it.
    let total = doc["plan"]["import"]["entries"].as_array().unwrap().len() + 1;
    let middle = total / 2;
    let (mut n, mut phase, mut stopped) = (1, 0, Vec::new());
    loop {
        restore(&snapshot, &base);
        let what = format!("the import stopped at point {n}");
        let ran = fx.apply_digest(&digest, &options, &[("FABREWK_FAULT_ABORT_AT", &n.to_string())]);
        if ran.status != -1 {
            assert_eq!(ran.status, 0, "{what}: {ran:?}");
            break;
        }
        let k = fx.events("bootstrap").len();
        stopped.push(k);
        if let Err(errors) = replay(&fx.domain, &PolicyVerifier) {
            assert!(errors.iter().all(|e| e.code().is_some()), "{what}: {errors:?}");
        }
        let ran = fx.run(&["check", "--json"]);
        assert!(matches!(ran.status, 0 | 3), "{what}: {ran:?}");
        coded(&ran, "findings", &what);
        let ran = fx.run(&["discard", "p-00000000", "--say", "Descártala.", "--json"]);
        assert_eq!(ran.status, 1, "{what}: {ran:?}");
        coded(&ran, "refusals", &what);
        let opened = fx.events("project").first().is_some_and(|e| e["kind"] == "assessed");
        let complete = fx.events("project").iter().any(|e| e["kind"] == "installed") && fx.run(&["check"]).status == 0;
        if opened && !complete {
            assert_eq!(ran.doc()["refusals"][0]["code"], "INSTALLATION_INCOMPLETE", "{what}: {ran:?}");
        }
        let again = fx.apply_digest(&digest, &options, &[]);
        assert_eq!(again.status, 0, "{what}: {again:?}");
        let check = fx.run(&["check"]);
        assert_eq!(check.status, 0, "{what}: {check:?}");
        let events = fx.events("bootstrap");
        assert_eq!(events.len(), total, "{what}");
        let texts: String = events[..total - 1].iter().map(|e| e["payload"]["text"].as_str().unwrap()).collect();
        assert_eq!(texts, decisions, "{what}");
        let kinds: Vec<Value> = fx.events("project").iter().map(|e| e["kind"].clone()).collect();
        assert_eq!(kinds, ["assessed", "publication-decided", "policy-adopted", "installed"], "{what}");
        // Every file of the domain committed at the branch's tip with its bytes on disk, read in two Git commands.
        let mut files = Vec::new();
        domain_files(&fx.domain, &mut files);
        let rel: Vec<String> =
            files.iter().map(|f| f.strip_prefix(&fx.root).unwrap().to_str().unwrap().to_owned()).collect();
        let held = fx.git(&["ls-tree", "-r", "--format=%(objectname) %(path)", "HEAD", "--", ".brewstillery"]);
        let held: BTreeMap<&str, &str> = held.lines().filter_map(|l| l.split_once(' ').map(|(o, p)| (p, o))).collect();
        let mut c = Command::new("git");
        isolate(&mut c, &fx.home, None);
        let out = c
            .current_dir(&fx.root)
            .args(["hash-object", "--no-filters", "--stdin-paths"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        let disk = {
            use std::io::Write;
            let mut out = out;
            out.stdin.take().unwrap().write_all(rel.join("\n").as_bytes()).unwrap();
            String::from_utf8(out.wait_with_output().unwrap().stdout).unwrap()
        };
        for (path, blob) in rel.iter().zip(disk.lines()) {
            assert_eq!(held.get(path.as_str()), Some(&blob), "{what}: {path}");
        }
        // Once the stream's first two events were covered whole, to an entry in its middle; from there, to its last.
        let jump = match phase {
            0 if k == 2 => Some(middle - 2),
            1 if k == middle + 1 => Some(total - 2 - (middle + 1)),
            _ => None,
        };
        match jump {
            Some(by) => {
                n += 3 * by;
                phase += 1;
            }
            None => n += 1,
        }
    }
    eprintln!("the import: {} write points stopped at", stopped.len());
    for k in [0, 1, middle, middle + 1, total - 2, total - 1, total] {
        assert!(stopped.contains(&k), "no stop with {k} events of the stream written: {stopped:?}");
    }
    open(&snapshot);
    std::fs::remove_dir_all(&snapshot).unwrap();
}

/// Every file under the domain but its unversioned area and its own Git directory.
fn domain_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        let (path, name) = (entry.path(), entry.file_name());
        match entry.file_type().unwrap().is_dir() {
            true if name != "local" && name != ".git" => domain_files(&path, out),
            true => {}
            false => out.push(path),
        }
    }
}
