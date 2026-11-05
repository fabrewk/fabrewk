//! `budget count` prints each ring's measure; `budget check --binary <path>` checks the whole
//! budget: rings, crates and the stripped release binary. Exit status: 0 within, 1 over, 2 usage, 70 error.

use budget::budget::RINGS;
use budget::{deps, rings};
use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "usage: budget count | budget check --binary <path>";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let result = match args.as_slice() {
        ["count"] => count(),
        ["check", "--binary", binary] => check(PathBuf::from(binary)),
        _ => {
            eprintln!("{USAGE}");
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) => {
            eprintln!("budget: {e}");
            ExitCode::from(70)
        }
    }
}

fn count() -> Result<bool, String> {
    print_counts(&rings::count_rings(&budget::workspace_root(), RINGS).map_err(|e| e.to_string())?);
    Ok(true)
}

fn print_counts(counts: &[rings::RingCount]) {
    for c in counts {
        println!(
            "ring {}: {} of {} lines, {} files",
            c.name,
            c.lines,
            c.limit(),
            c.files
        );
    }
}

fn check(binary: PathBuf) -> Result<bool, String> {
    let root = budget::workspace_root();
    let counts = rings::count_rings(&root, RINGS).map_err(|e| e.to_string())?;
    print_counts(&counts);
    let report = budget::release_report(&root, &counts)?;
    println!(
        "direct runtime crates: {} {:?}",
        report.direct.len(),
        report.direct.values().collect::<Vec<_>>()
    );
    println!(
        "crates in the release build, every platform counted: {}",
        report.all.len()
    );
    let bytes = std::fs::metadata(&binary)
        .map_err(|e| format!("{}: {e}", binary.display()))?
        .len();
    let dep_info_path = binary.with_extension("d");
    let dep_info = std::fs::read_to_string(&dep_info_path)
        .map_err(|e| format!("{}: {e}", dep_info_path.display()))?;
    println!("release binary {}: {bytes} bytes", binary.display());
    let mut problems = budget::ring_problems(&counts);
    problems.extend(budget::dependency_problems(&report));
    problems.extend(budget::binary_problem(bytes));
    problems.extend(budget::source_problems(&dep_info, &counts)?);
    problems.extend(deps::config_problems(&root, std::env::vars()));
    for p in &problems {
        println!("over budget: {p}");
    }
    Ok(problems.is_empty())
}
