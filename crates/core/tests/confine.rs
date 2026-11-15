//! The cell's contract and its runner at the core's level (part 9a; REQ-SEC-2, REQ-REG-13; `confinement-design.md`
//! 3.2, 3.7, 3.9), with backends that confine nothing: what refuses before any cell starts, the probe that must fail
//! where a planted file is visible, the evidence store's modes, the limits, and the removal of what a cell left. The
//! bubblewrap backend's own tests run real cells (`crates/confine-bwrap/tests/cell.rs`).

mod common;

use fabrewk_core::confine::{
    self, Access, Backend, Cell, End, Limits, Mount, Network, Programs, Scratch, Started, Unfinished,
};
use fabrewk_core::detect::{Detection, Found, How, Program, Space};
use fabrewk_core::record::{Code, Error, sha256_hex};
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const PROGRAMS: [Program; 1] = [Program { adapter: None, executable: "sh", version: &[], tokens: false, skills: &[] }];

/// Runs the cell's command on the host, unconfined, with `/work` in its arguments read as the host directory mounted
/// there.
fn unconfined(cell: &Cell, _: &Programs, _: &Path, _: &Path) -> Result<Started, Error> {
    let work = cell.mounts[0].host.to_str().unwrap();
    let argv: Vec<String> = cell.argv.iter().map(|a| a.to_str().unwrap().replace("/work", work)).collect();
    let child = Command::new(&argv[0])
        .args(&argv[1..])
        .env_clear()
        .envs(cell.env.iter().map(|(k, v)| (k, v)))
        .current_dir(work)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    Ok(Started { child, beside: Vec::new(), first: None, failed: None })
}

fn stop(s: &mut Started) -> Result<(), Error> {
    let _ = s.child.kill();
    s.child.wait().map(|_| ()).map_err(|error| Error::Io { path: "sh".into(), error })
}

fn never(_: &Cell, _: &Programs, _: &Path, _: &Path) -> Result<Started, Error> {
    panic!("a verified backend is not probed again")
}

const UNCONFINED: Backend = Backend {
    name: "unconfined",
    programs: &PROGRAMS,
    system: &["/usr"],
    start: unconfined,
    stop,
    idle: |_| {},
    residual: &["kernel"],
};

fn domain(name: &str) -> PathBuf {
    let d = common::tmp(name).join(".brewstillery");
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn detection(verified: Option<&str>, present: bool) -> Detection {
    let found = present.then(|| Found {
        path: "/bin/sh".into(),
        version: None,
        how: How::Observed,
        verified: verified.map(String::from),
    });
    Detection { at: "2026-11-11T00:00:00Z".into(), found: BTreeMap::from([((Space::Program, "sh".to_owned()), found)]) }
}

fn cell(scratch: &Scratch, script: &str) -> Cell {
    Cell {
        home: scratch.home.clone(),
        tmp: scratch.tmp.clone(),
        mounts: vec![Mount { host: scratch.work.clone(), inside: "/work".into(), access: Access::ReadWrite }],
        network: Network::None,
        nested: false,
        env: vec![("PATH".into(), "/usr/bin:/bin".into())],
        cwd: "/work".into(),
        argv: ["/bin/sh", "-c", script].map(OsString::from).into(),
    }
}

fn run(d: &Path, name: &str, script: &str, limits: Limits) -> Result<confine::Ran, Unfinished> {
    let scratch = Scratch::create(d, name).unwrap();
    let evidence = confine::evidence(d, name).unwrap();
    let programs =
        confine::available(Some(&UNCONFINED), &mut detection(Some("2026-11-10T00:00:00Z"), true), d).unwrap();
    let ran = confine::run(&UNCONFINED, &programs, &cell(&scratch, script), &scratch, &evidence, limits);
    scratch.remove().unwrap();
    ran
}

fn limits(seconds: u64, bytes: u64) -> Limits {
    Limits { time: Duration::from_secs(seconds), bytes }
}

#[test]
fn no_backend_or_an_absent_program_refuses_before_any_cell() {
    let d = domain("confine-refusals");
    let e = confine::available(None, &mut detection(None, true), &d).unwrap_err();
    assert_eq!(e.code(), Some(Code::ConfinementUnavailable));
    let e = confine::available(Some(&UNCONFINED), &mut detection(None, false), &d).unwrap_err();
    assert_eq!(e.code(), Some(Code::ConfinementUnavailable));
    assert!(e.detail().contains("sh: not present") && e.detail().contains("unconfined"), "{}", e.detail());
    assert!(!d.join("local").exists(), "nothing is written before a backend is found whole");
}

#[test]
fn a_verified_backend_is_not_probed_again() {
    let d = domain("confine-verified");
    let b = Backend { start: never, ..UNCONFINED };
    let programs = confine::available(Some(&b), &mut detection(Some("2026-11-10T00:00:00Z"), true), &d).unwrap();
    assert_eq!(programs.get("sh"), Some(Path::new("/bin/sh")));
    assert_eq!(programs.get("bwrap"), None);
}

#[test]
fn a_backend_that_drives_no_program_is_probed_at_every_use() {
    let d = domain("confine-no-program");
    let b = Backend { programs: &[], ..UNCONFINED };
    let e = confine::available(Some(&b), &mut detection(Some("2026-11-10T00:00:00Z"), true), &d).unwrap_err();
    assert_eq!(e.code(), Some(Code::ConfinementUnavailable), "probed, and the unconfined cell sees the planted file");
}

#[test]
fn a_backend_whose_cell_sees_the_planted_file_fails_its_probe_and_names_its_evidence() {
    let d = domain("confine-probe");
    let mut found = detection(None, true);
    let e = confine::available(Some(&UNCONFINED), &mut found, &d).unwrap_err();
    let Error::Refused { code, path: Some(evidence), detail } = e else { panic!("{e:?}") };
    assert_eq!(code, Code::ConfinementUnavailable);
    assert!(detail.contains(&evidence.display().to_string()), "{detail}");
    assert_eq!(std::fs::read_to_string(evidence.join("stdout")).unwrap(), "seen\nwritten\n");
    assert!(found.get(Space::Program, "sh").unwrap().verified.is_none(), "a failed probe verifies nothing");
    assert_eq!(std::fs::read_dir(d.join("local/tmp")).unwrap().count(), 0, "the probe's directories are removed");
}

#[test]
fn the_runner_keeps_both_channels_with_their_digests_and_modes() {
    let d = domain("confine-kept");
    let ran = run(&d, "kept", "printf one; printf two >&2; exit 4", limits(30, 100)).unwrap();
    assert_eq!(ran.end, End::Exited(4));
    for (k, text) in [(&ran.stdout, "one"), (&ran.stderr, "two")] {
        assert_eq!(std::fs::read_to_string(&k.path).unwrap(), text);
        assert_eq!((k.digest.clone(), k.bytes, k.capped), (sha256_hex(text.as_bytes()), 3, false));
        assert_eq!(std::fs::metadata(&k.path).unwrap().permissions().mode() & 0o777, 0o600);
    }
    for dir in [ran.stdout.path.parent().unwrap(), &d.join("local/evidence")] {
        assert_eq!(std::fs::metadata(dir).unwrap().permissions().mode() & 0o777, 0o700);
    }
    let Error::Refused { code, path, detail } = ran.failure("fake") else { panic!() };
    assert_eq!((code, path.as_deref()), (Code::ToolFailed, ran.stdout.path.parent()));
    assert!(detail.contains("exited with status 4") && !detail.contains("one"), "{detail}");
    // A run's name is its own: a second evidence directory of the same name refuses.
    assert!(confine::evidence(&d, "kept").is_err());
}

#[test]
fn a_run_past_its_time_or_its_bytes_is_ended_and_says_so() {
    let d = domain("confine-limits");
    let begun = Instant::now();
    let ran = run(&d, "slow", "printf begun; exec sleep 600", limits(1, 100)).unwrap();
    assert!(begun.elapsed() < Duration::from_secs(10));
    assert_eq!((ran.end, std::fs::read_to_string(&ran.stdout.path).unwrap().as_str()), (End::TimedOut, "begun"));
    assert!(ran.failure("fake").detail().contains("time limit"));
    let begun = Instant::now();
    let ran = run(&d, "loud", "printf 0123456789abcdef; exec sleep 600", limits(30, 10)).unwrap();
    assert!(begun.elapsed() < Duration::from_secs(10), "ended when its bytes passed, not at its time");
    assert_eq!((ran.end, ran.stdout.bytes, ran.stdout.capped), (End::OutputCapped, 10, true));
    assert_eq!(std::fs::read_to_string(&ran.stdout.path).unwrap(), "0123456789");
    assert!(ran.failure("fake").detail().contains("bytes kept"));
    // A channel exactly at its bytes is whole, not cut.
    let ran = run(&d, "exact", "printf 0123456789", limits(30, 10)).unwrap();
    assert_eq!((ran.end, ran.stdout.capped), (End::Exited(0), false));
}

#[test]
fn a_cell_with_no_command_is_refused_before_it_starts() {
    let d = domain("confine-empty");
    let scratch = Scratch::create(&d, "empty").unwrap();
    let evidence = confine::evidence(&d, "empty").unwrap();
    let b = Backend { start: never, ..UNCONFINED };
    let programs = confine::available(Some(&b), &mut detection(Some("2026-11-10T00:00:00Z"), true), &d).unwrap();
    let mut c = cell(&scratch, "true");
    c.argv.clear();
    let u = confine::run(&b, &programs, &c, &scratch, &evidence, limits(30, 10)).unwrap_err();
    assert_eq!((u.error.code(), u.ended), (Some(Code::ToolFailed), true));
}

#[test]
fn a_run_ended_by_a_signal_says_so() {
    let d = domain("confine-signal");
    let ran = run(&d, "signal", "kill -KILL $$", limits(30, 10)).unwrap();
    assert_eq!(ran.end, End::Signalled);
    assert!(ran.failure("fake").detail().contains("signal"));
}

#[test]
fn removing_a_scratch_follows_no_link_and_reopens_what_a_cell_closed() {
    let d = domain("confine-remove");
    let outside = d.parent().unwrap().join("outside");
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(outside.join("kept"), "kept").unwrap();
    let scratch = Scratch::create(&d, "left").unwrap();
    let root = scratch.root.clone();
    for dir in [&scratch.home, &scratch.tmp, &scratch.work] {
        assert_eq!(std::fs::metadata(dir).unwrap().permissions().mode() & 0o777, 0o700);
    }
    std::os::unix::fs::symlink(&outside, scratch.work.join("link")).unwrap();
    let closed = scratch.work.join("closed");
    std::fs::create_dir(&closed).unwrap();
    std::fs::write(closed.join("file"), "x").unwrap();
    std::fs::set_permissions(&closed, std::fs::Permissions::from_mode(0o500)).unwrap();
    assert!(Scratch::create(&d, "left").is_err(), "a name in use is not reused");
    scratch.remove().unwrap();
    assert!(!root.exists());
    assert_eq!(std::fs::read_to_string(outside.join("kept")).unwrap(), "kept");
}

/// Backends that cannot show their cell ended, by a refusal or by an operating-system error: the process runs on.
fn left_running(_: &mut Started) -> Result<(), Error> {
    Err(Error::Refused { code: Code::ToolFailed, path: None, detail: "the cell did not end".into() })
}

fn left_unknown(_: &mut Started) -> Result<(), Error> {
    Err(Error::Io { path: "bwrap".into(), error: std::io::Error::other("wait failed") })
}

#[test]
fn p9a_1_a_cell_the_backend_cannot_show_ended_is_neither_waited_for_nor_read_nor_removed() {
    for (n, stop) in [left_running as fn(&mut Started) -> Result<(), Error>, left_unknown].into_iter().enumerate() {
        let d = domain(&format!("confine-unproven-{n}"));
        let b = Backend { stop, ..UNCONFINED };
        let scratch = Scratch::create(&d, "running").unwrap();
        let evidence = confine::evidence(&d, "running").unwrap();
        let programs = confine::available(Some(&b), &mut detection(Some("2026-11-10T00:00:00Z"), true), &d).unwrap();
        let begun = Instant::now();
        // The process holds both channels open for five seconds; the runner does not wait for them.
        let c = cell(&scratch, "exec sleep 5");
        let u = confine::run(&b, &programs, &c, &scratch, &evidence, limits(0, 10)).unwrap_err();
        assert!(!u.ended, "{n}");
        assert!(begun.elapsed() < Duration::from_secs(3), "{:?}", begun.elapsed());
        // Its probe keeps its directories, unread, and refuses.
        let e = confine::available(Some(&b), &mut detection(None, true), &d).unwrap_err();
        assert_eq!(e.code(), Some(Code::ConfinementUnavailable));
        assert!(e.detail().contains("did not end"), "{}", e.detail());
        let left: Vec<_> = std::fs::read_dir(d.join("local/tmp")).unwrap().flatten().map(|e| e.file_name()).collect();
        assert_eq!(left.len(), 2, "the run's and the probe's directories stay: {left:?}");
    }
}

/// A backend that starts its cell's process, then fails to finish starting it.
fn half_started(cell: &Cell, programs: &Programs, scratch: &Path, evidence: &Path) -> Result<Started, Error> {
    let mut started = unconfined(cell, programs, scratch, evidence)?;
    // The cell wrote before the backend's failure, as `bwrap` does before `pasta` fails.
    std::thread::sleep(Duration::from_millis(300));
    started.failed = Some(Error::Refused { code: Code::ConfinementUnavailable, path: None, detail: "no net".into() });
    Ok(started)
}

#[test]
fn p9a_3_a_cell_that_failed_to_start_keeps_its_output_and_is_ended() {
    let d = domain("confine-half");
    let b = Backend { start: half_started, ..UNCONFINED };
    let programs = confine::available(Some(&b), &mut detection(Some("2026-11-10T00:00:00Z"), true), &d).unwrap();
    let scratch = Scratch::create(&d, "half").unwrap();
    let evidence = confine::evidence(&d, "half").unwrap();
    let c = cell(&scratch, "echo early-diagnostic >&2; exec sleep 600");
    let begun = Instant::now();
    let u = confine::run(&b, &programs, &c, &scratch, &evidence, limits(30, 100)).unwrap_err();
    assert!(u.ended && begun.elapsed() < Duration::from_secs(10));
    assert_eq!(u.error.code(), Some(Code::ConfinementUnavailable));
    assert_eq!(std::fs::read_to_string(evidence.join("stderr")).unwrap(), "early-diagnostic\n");
    // And when its end cannot be shown either, the failure says so.
    let b = Backend { start: half_started, stop: left_unknown, ..UNCONFINED };
    let scratch = Scratch::create(&d, "half-unknown").unwrap();
    let evidence = confine::evidence(&d, "half-unknown").unwrap();
    let u = confine::run(&b, &programs, &cell(&scratch, "exec sleep 3"), &scratch, &evidence, limits(30, 10));
    assert!(!u.unwrap_err().ended);
}

#[test]
fn the_cores_residual_risks_are_three() {
    assert_eq!(confine::RESIDUAL, ["check-proves", "reviewer-steered", "same-user"]);
}
