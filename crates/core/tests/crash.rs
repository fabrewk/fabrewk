//! The crash suite on the record (core-specification 11.5; PD-08's property for the record, PD-10 for the record,
//! PD-46 for the record), built only with the feature `fault-injection`: a child process runs a scenario of three
//! write commands in `published` and aborts at its n-th write point, for every n; after each kill the record must
//! replay or be refused by a code, keep every event the child reported written and every commit it reported, keep its
//! mark whole, and the next write command must bring it back to a clean state.

mod common;

use common::*;
use fabrewk_core::record::{Code, Session, check_mark, fault, replay, uncommitted, walk_history};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::process::Command;

const DIR: &str = "FABREWK_CRASH_DIR";

/// The scenario, in the child: three write commands, each reporting what it wrote as soon as the call returns.
fn scenario(f: &Fixture) {
    let say = |what: String| println!("{what}");
    let mut s = Session::begin(&f.domain, f.repo(), &f.verifier()).unwrap();
    let mut head = None;
    for (kind, words) in [("assessed", "Evalúa."), ("publication-decided", "Publicado."), ("policy-adopted", "Adopto.")]
    {
        head = Some(
            s.append("project", head.as_ref(), &person(kind, words, project_payload(kind)), Some(&f.signer())).unwrap(),
        );
        say(format!("appended project {}", head.as_ref().unwrap().digest));
    }
    let h = s.append("project", head.as_ref(), &fabrewk("installed", project_payload("installed")), None).unwrap();
    say(format!("appended project {}", h.digest));
    say(format!("committed {}", s.finish(&f.verifier(), &[]).unwrap().unwrap()));

    let mut s = Session::begin(&f.domain, f.repo(), &f.verifier()).unwrap();
    s.commit_uncommitted(&f.verifier()).unwrap();
    let (stage, draft) = creation("stage-proposed", stage_fields());
    let h = s.append(&stage, None, &draft, None).unwrap();
    say(format!("appended {stage} {}", h.digest));
    let (stream, draft) = creation("task-proposed", task_fields(&stage));
    let h = s.append(&stream, None, &draft, None).unwrap();
    say(format!("appended {stream} {}", h.digest));
    let draft = person("task-authorized", "Autorizo.", bound(f, &stream));
    let h = s.append(&stream, Some(&h), &draft, Some(&f.signer())).unwrap();
    say(format!("appended {stream} {}", h.digest));
    let run = json!({
        "task": &stream["task-".len()..], "head": {"seq": h.seq, "digest": h.digest},
        "authorization": {"stream": stream, "seq": h.seq, "digest": h.digest}, "base": "0".repeat(40),
    });
    let h = s.append(&stream, Some(&h), &person("run-started", "Corre.", run), Some(&f.signer())).unwrap();
    say(format!("appended {stream} {}", h.digest));
    std::fs::write(f.domain.join("STATUS.md"), "view\n").unwrap();
    say(format!("committed {}", s.finish(&f.verifier(), &[PathBuf::from("STATUS.md")]).unwrap().unwrap()));

    let mut s = Session::begin(&f.domain, f.repo(), &f.verifier()).unwrap();
    s.commit_uncommitted(&f.verifier()).unwrap();
    let h = s.head(&stream).unwrap();
    let h = s.append(&stream, h.as_ref(), &fabrewk("attempt-started", attempt()), None).unwrap();
    say(format!("appended {stream} {}", h.digest));
    say(format!("committed {}", s.finish(&f.verifier(), &[]).unwrap().unwrap()));
}

#[test]
fn crash_child() {
    let Some(base) = std::env::var_os(DIR) else {
        return;
    };
    let f = Fixture::at(Path::new(&base));
    if std::env::var_os("FABREWK_FAULT_FAIL").is_some() {
        let mut s = Session::begin(&f.domain, f.repo(), &f.verifier()).unwrap();
        s.append("ext-x", None, &note(json!({})), None).unwrap();
        let e = s.finish(&f.verifier(), &[]).unwrap_err();
        println!("refused {}", e.code().unwrap());
        return;
    }
    scenario(&f);
    println!("points {}", fault::points());
}

fn child(base: &Path, env: &[(&str, &str)]) -> (bool, String) {
    let mut c = Command::new(std::env::current_exe().unwrap());
    c.args(["crash_child", "--exact", "--nocapture", "--test-threads=1"]).env(DIR, base).env_remove(ISOLATED);
    for (k, v) in env {
        c.env(k, v);
    }
    let out = c.output().unwrap();
    (out.status.success(), String::from_utf8_lossy(&out.stdout).into_owned())
}

/// The child's reports; with `--nocapture` the first one shares its line with the harness's `test … ... `.
fn reported<'a>(out: &'a str, what: &str) -> Vec<&'a str> {
    out.split('\n').filter_map(|l| l.find(what).map(|i| &l[i + what.len()..])).collect()
}

#[test]
fn crash_suite_on_the_record() {
    isolated("crash_suite_on_the_record", || {
        let pristine = Fixture::new("crash", true);
        let base = pristine.root.parent().unwrap().to_path_buf();
        let whole = base.with_file_name(format!("{}-whole", base.file_name().unwrap().to_str().unwrap()));
        copy_tree(&base, &whole);
        let (ok, out) = child(&whole, &[]);
        assert!(ok, "{out}");
        let points: usize = reported(&out, "points ")[0].parse().unwrap();
        let all = reported(&out, "appended ").len();
        let mut before = 0;
        assert!(points > 50, "{points} write points");
        for n in 1..=points {
            let run = base.with_file_name(format!("{}-{n}", base.file_name().unwrap().to_str().unwrap()));
            copy_tree(&base, &run);
            let (ok, out) = child(&run, &[("FABREWK_FAULT_ABORT_AT", &n.to_string())]);
            assert!(!ok, "point {n} did not stop the child");
            // The child stops at the point named, not earlier: what it reports grows with n, and all of it by the last.
            let appended = reported(&out, "appended ").len();
            assert!(appended >= before && (n < points || appended == all), "point {n}: {appended} of {all}");
            before = appended;
            after_kill(&Fixture::at(&run), &out, n);
            std::fs::remove_dir_all(&run).unwrap();
        }
    });
}

fn after_kill(f: &Fixture, out: &str, n: usize) {
    let appended = reported(out, "appended ");
    // Replay is valid, or refused by a code: here only the absence of a record not yet begun.
    let r = match replay(&f.domain, &f.verifier()) {
        Ok(r) => Some(r),
        Err(errors) => {
            assert!(appended.is_empty(), "point {n}: {errors:?}");
            assert!(errors.iter().all(|e| e.code() == Some(Code::RecordAbsent)), "point {n}: {errors:?}");
            None
        }
    };
    // Every event the child was told is written is in place, signed if a person's (the replay verified it).
    for line in &appended {
        let (stream, digest) = line.split_once(' ').unwrap();
        let events = &r.as_ref().unwrap().streams[stream];
        assert!(events.iter().any(|e| e.digest == digest), "point {n}: {line} lost");
    }
    // Every reported commit is on the branch; no committed event is lost; the mark is the old or the new one.
    let repo = f.repo().unwrap();
    for commit in reported(out, "committed ") {
        git(&f.root, &["merge-base", "--is-ancestor", commit, "main"]);
    }
    walk_history(&repo, "refs/heads/main").unwrap_or_else(|e| panic!("point {n}: {e:?}"));
    if let Some(r) = &r {
        check_mark(&f.domain, r).unwrap_or_else(|e| panic!("point {n}: {e:?}"));
    }
    // A killed write leaves only temporary names starting with `.`, or a signature whose event is not in place.
    for dir in std::fs::read_dir(f.domain.join("data/events")).into_iter().flatten() {
        for file in std::fs::read_dir(dir.unwrap().path()).unwrap() {
            let name = file.unwrap().file_name().into_string().unwrap();
            assert!(
                name.starts_with('.') || name.ends_with(".json") || name.ends_with(".json.sig"),
                "point {n}: {name}"
            );
        }
    }
    // The next write command brings the record back: leftovers removed, events committed, index in step.
    let mut s = Session::begin(&f.domain, f.repo(), &f.verifier()).unwrap_or_else(|e| panic!("point {n}: {e:?}"));
    s.commit_uncommitted(&f.verifier()).unwrap_or_else(|e| panic!("point {n}: {e:?}"));
    if r.is_some() {
        s.finish(&f.verifier(), &[]).unwrap_or_else(|e| panic!("point {n}: {e:?}"));
        let r = replay(&f.domain, &f.verifier()).unwrap();
        check_mark(&f.domain, &r).unwrap();
    } else {
        drop(s);
    }
    assert!(uncommitted(&repo).unwrap().is_empty(), "point {n}");
    walk_history(&repo, "refs/heads/main").unwrap();
    assert_eq!(git(&f.root, &["status", "--porcelain", "--", ".brewstillery/data"]), "", "point {n}");
    for dir in std::fs::read_dir(f.domain.join("data/events")).into_iter().flatten() {
        for file in std::fs::read_dir(dir.unwrap().path()).unwrap() {
            let name = file.unwrap().file_name().into_string().unwrap();
            assert!(!name.starts_with('.'), "point {n}: {name} left after recovery");
        }
    }
    assert!(!f.domain.join("local/tmp/record-index").exists(), "point {n}");
}

#[test]
fn pd_46_a_failed_replay_before_the_commit_commits_nothing() {
    isolated("pd_46_a_failed_replay_before_the_commit_commits_nothing", || {
        let f = Fixture::new("pd46", true);
        let base = f.root.parent().unwrap();
        let head = git(&f.root, &["rev-parse", "HEAD"]);
        let (ok, out) = child(base, &[("FABREWK_FAULT_FAIL", "replay-before-commit")]);
        assert!(ok, "{out}");
        assert_eq!(reported(&out, "refused "), ["RECORD_BROKEN"]);
        assert_eq!(git(&f.root, &["rev-parse", "HEAD"]), head);
        assert_eq!(uncommitted(&f.repo().unwrap()).unwrap().len(), 1);
        assert!(!f.domain.join("local/cache/heads.json").exists());
    });
}
