---
name: verify-part
description: Runs Fabrewk's verification of a part of its bootstrap on this machine - the suites with the rustup toolchain, the crash suite, the clean clone, the musl release build, the size counter, clippy, rustfmt, cargo deny, and cargo mutants in a memory-capped scope - and records the results the way the bootstrap record expects. Use when a part's code is ready for review or closing, after a review's corrections, or when a mutation run is due.
---

> **Fabrewk skill (part 7s, D-060).** Written by the integrator from what the bootstrap record already fixes (D-041,
> D-043, D-049, R-022, R-023, R-024, R-029); it decides nothing new. `AGENTS.md` and the maestro's authorization come
> first. Everything here runs under `target/` (on disk, ignored by Git); nothing is installed.

# Verify a part

A part of phase 7 closes only with: tests green in a clean clone, the crash suite green, the musl release build,
`budget check` passing, `clippy` and `fmt` clean, `cargo deny` clean, and every mutant caught or declared equivalent
(plan, row 7 and "Parts"). This skill is how, on this machine.

## The toolchain prefix

Fedora's `cargo` is first on `PATH`; the suites run with the rustup toolchain 1.99.1 (D-043; with Fedora's `cargo`
alone one test fails by design). Bash, for an agent:

    export RUSTUP_HOME=$HOME/.rustup CARGO_HOME=$HOME/.rustup/cargo-home PATH="$HOME/.rustup/cargo-home/bin:$PATH"

Fish, for a command the maintainer is asked to run (their shell, `AGENTS.md` "Shell"):

    env RUSTUP_HOME=$HOME/.rustup CARGO_HOME=$HOME/.rustup/cargo-home PATH="$HOME/.rustup/cargo-home/bin:$PATH" cargo +1.99.1 test --locked

## The checks, in order

With the prefix, from the repository's root:

    cargo +1.99.1 test --locked
    cargo +1.99.1 test --locked -p fabrewk-core --features fault-injection --test crash
    cargo +1.99.1 build --release --locked --target aarch64-unknown-linux-musl -p fabrewk
    cargo run --locked -p budget -- check --binary target/aarch64-unknown-linux-musl/release/fabrewk
    cargo +1.99.1 clippy --workspace --all-targets --locked -- -D warnings
    cargo +1.99.1 clippy --workspace --all-targets --locked --features fabrewk-core/fault-injection -- -D warnings
    cargo +1.99.1 fmt --check
    cargo deny --offline check licenses advisories

Also check that the release binary's `--version --json` lists no feature. Write down what `budget check` prints
(counted lines of the core against its ceiling, direct runtime crates, crates in the release build, the binary's
size): the record carries it, and a change of the ceiling is a maestro's decision (`AGENTS.md` "Code").

## The clean clone

The acceptance is "green in a clean clone", not in the working tree. Commit first, then:

    git clone -q . target/clean-<part>
    cd target/clean-<part>

and run every check above there, with the same prefix. Record the commit the clone was made from.

## Mutation

This machine has 7.3 GB of RAM and `/tmp` is memory; a mutant that loops on memory once took VS Code down (R-022),
and a run filled the disk once (R-029). So every run goes into its own user scope, with a cap per process, two jobs,
and `TMPDIR` on disk under `target/`:

    mkdir -p target/mutants-tmp
    export TMPDIR=$PWD/target/mutants-tmp
    systemd-run --user --scope -p MemoryMax=3G -p MemorySwapMax=0 -p OOMPolicy=continue \
      bash -c 'ulimit -v 1000000; exec cargo mutants --package fabrewk-core --package fabrewk --test-workspace true -j 2 --timeout 90 --features fabrewk-core/fault-injection --in-diff target/<part>.diff --output target/mutants-<part>'

and, when the part touches `tools/budget/`, a second run in the same kind of scope for the counter, without the
feature (it does not depend on the core):

      bash -c 'ulimit -v 1000000; exec cargo mutants --package budget -j 2 --timeout 90 --in-diff target/<part>.diff --output target/mutants-<part>-budget'

with the prefix exported first, and `target/<part>.diff` written by `git diff <start>..HEAD > target/<part>.diff`.
`--features fabrewk-core/fault-injection` makes the crash suite judge every mutant too (R-024: mutants in the
record's store, Git and fault code are caught only by it); without it the `crash` target is not built.
`--test-workspace true` runs every package's tests on each mutant (the binary's tests reach the core's command
surface). The counter has its own run because cargo-mutants builds the baseline for the mutated packages alone, and
`budget` refuses the core's feature (P7S-5). `--in-diff` limits a run to the part; a full run drops it. Keep `OOMPolicy=continue` (with `stop` the scope ends the
run) and 1 GB per process (with 4 GB the scope's killer once took `cargo-mutants` itself). Run one heavy job at a time:
never a mutation run beside a Codex review or another build.

For each missed mutant: add the assertion or case that kills it, or simplify the code so it no longer exists, or
declare it equivalent with the reason (why no input can tell the mutant from the original). Then run again over what
changed. Timeouts are loops caught by the timeout, not misses. The record lists every declared equivalent.

## After

- Remove what the run left: `target/mutants-tmp/` contents, old clones and review directories of earlier parts
  under `target/`, and anything a reviewer left in `/tmp`. Check the free disk space before a full run.
- Report in the session's provenance entry (`decisions.md`, an R entry): the clean clone's commit, the number of
  tests, each check's result, the measure of `budget check`, and the mutation figures (mutants, missed, timeouts,
  declared equivalents with their reasons). A check that fails is reported with its output, never retried silently.
