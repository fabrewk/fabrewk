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
    cargo +1.99.1 test --locked -p fabrewk-core -p fabrewk --features fault-injection --test crash --test faults
    cargo +1.99.1 build --release --locked --target aarch64-unknown-linux-musl -p fabrewk
    cargo run --locked -p budget -- check --binary target/aarch64-unknown-linux-musl/release/fabrewk
    cargo +1.99.1 clippy --workspace --all-targets --locked -- -D warnings
    cargo +1.99.1 clippy --workspace --all-targets --locked --features fabrewk-core/fault-injection -- -D warnings
    cargo +1.99.1 fmt --check
    cargo deny --offline check licenses advisories

On this machine each of them runs, as the mutation run does, in a user scope of 2 GB with no debug information and
two build jobs (`systemd-run --user --scope -p MemoryMax=2G -p MemorySwapMax=0`, `CARGO_PROFILE_DEV_DEBUG=0`,
`CARGO_PROFILE_TEST_DEBUG=0`, `-j 2`; R-045); the run's tests and the launch tests need `bwrap`, `pasta`, `socat`,
`python3` and the network. Also check that the release binary's `--version --json` lists no feature. Write down what
`budget check` prints (counted lines of the core against its ceiling, direct runtime crates, crates in the release
build, the binary's size): the record carries it, and a change of the ceiling is a maestro's decision (`AGENTS.md`
"Code").

## The clean clone

The acceptance is "green in a clean clone", not in the working tree. Commit first, then:

    git clone -q . target/clean-<part>
    cd target/clean-<part>

and run every check above there, with the same prefix. Record the commit the clone was made from.

## Mutation

This machine has 7.3 GB of RAM, of which the editor leaves about 2 GB free, and `/tmp` is memory; a mutant that
loops on memory once took VS Code down (R-022), a run filled the disk once (R-029), and a run in a 3 GB scope ran the
machine out of memory (R-044). So every run goes into its own user scope of 2 GB, with a cap per process of 1.6 GB,
two jobserver tasks, no debug information, and `TMPDIR` on disk under `target/`.

Before starting, look for what earlier runs left. `systemctl --user list-units --type=scope` lists the scopes of
`systemd-run`; stop each one an earlier run or suite left (`systemctl --user stop <scope>`), never one in use. Then
`pgrep -a pasta`: a `pasta` whose `--netns` is `/proc/<pid>/ns/net` with no `/proc/<pid>` left is the idle helper of a
cell that was killed (the backend's declared residual `network-helper`, part 9a); end it with `kill`. Any other `pasta`
(a container's, for example) is not Fabrewk's and is left alone.

Until the recovery of a stopped run ends those helpers (D-092, part 9i), each test the timeout kills leaves one, and
they pile up and slow every suite (R-045: 72 in one run, about 1 GB). So the run carries a watcher in its own scope
that ends each idle `pasta` of that scope whose cell's process is gone; write it once as `target/watch-pasta.sh`:

    #!/bin/bash
    # Ends each idle pasta of this scope whose cell's process is gone (D-093; until D-092 is built).
    own=$(cat /proc/self/cgroup)
    while sleep 10; do
      for p in $(pgrep -x pasta); do
        [ "$(cat /proc/$p/cgroup 2>/dev/null)" = "$own" ] || continue
        ns=$(tr '\0' '\n' < /proc/$p/cmdline 2>/dev/null | grep -m1 '^/proc/[0-9]*/ns/net$') || continue
        pid=${ns#/proc/}; pid=${pid%%/*}
        [ -d /proc/$pid ] || kill $p
      done
    done

Then, with the prefix exported first:

    mkdir -p target/mutants-tmp
    export TMPDIR=$PWD/target/mutants-tmp CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
    systemd-run --user --scope -p MemoryMax=2G -p MemorySwapMax=0 -p OOMPolicy=continue \
      bash -c 'ulimit -v 1600000; bash target/watch-pasta.sh & w=$!; cargo mutants --package fabrewk-core --package fabrewk --test-workspace true -j 1 --jobserver-tasks 2 --timeout 400 --build-timeout 600 --features fabrewk-core/fault-injection --in-diff target/<part>.diff --output target/mutants-<part>; s=$?; kill $w; exit $s'

and, when the part touches `tools/budget/`, a second run in the same kind of scope for the counter, without the
feature (it does not depend on the core):

      bash -c 'ulimit -v 1600000; exec cargo mutants --package budget -j 2 --jobserver-tasks 2 --timeout 90 --build-timeout 600 --in-diff target/<part>.diff --output target/mutants-<part>-budget'

with `target/<part>.diff` written by `git diff <start>..HEAD > target/<part>.diff`.

A passing suite of the core and the binary takes about 190 s even alone (R-045), hence `-j 1 --timeout 400`. Two jobs
(`-j 2 --timeout 180`) finish sooner, but a mutant that only lost the processor to the other job ends as a timeout:
run the timeouts of such a run again alone (`--iterate -j 1 --timeout 400`, same scope) before counting them.
`--iterate` skips the mutants caught before; when the code changed since the last run (a correction of a missed
mutant), write the part's diff again first, against the working tree (`git diff <start> > target/<part>.diff`), so
that `--in-diff` finds the lines where they now are.
`--features fabrewk-core/fault-injection` makes the crash suite judge every mutant too (R-024: mutants in the
record's store, Git and fault code are caught only by it), and builds the binary's fault test `faults` (part 7d3a,
R-031); without it neither target is built.
`--test-workspace true` runs every package's tests on each mutant (the binary's tests reach the core's command
surface). The counter has its own run because cargo-mutants builds the baseline for the mutated packages alone, and
`budget` refuses the core's feature (P7S-5). `--in-diff` limits a run to the part; a full run drops it. Keep `OOMPolicy=continue` (with `stop` the scope ends the
run) and a cap per process below the scope's (with 4 GB per process the scope's killer once took `cargo-mutants`
itself). Run one heavy job at a time:
never a mutation run beside a Codex review or another build.

For each missed mutant: add the assertion or case that kills it, or simplify the code so it no longer exists, or declare
it equivalent with the reason (why no input can tell the mutant from the original). Then run again over what changed.
Timeouts are loops caught by the timeout, not misses. A build that reaches the cap per process can wait forever instead
of failing (R-034: one compiler waited 1 h 45 min); with `--build-timeout 600` it ends as a timeout, and such a mutant
is run again alone (`--re` naming it, `-j 1`, in the same kind of scope without `ulimit -v`, the scope's 2 GB still
capping it) before it is counted. The record lists every declared equivalent.

## After

- Remove what the run left: any `pasta` and scope the run left (as before starting), `target/mutants-tmp/` contents,
  old clones and review directories of earlier parts under `target/`, and anything a reviewer left in `/tmp`. Check
  the free disk space before a full run.
- Report in the session's provenance entry (`decisions.md`, an R entry): the clean clone's commit, the number of
  tests, each check's result, the measure of `budget check`, and the mutation figures (mutants, missed, timeouts,
  declared equivalents with their reasons). A check that fails is reported with its output, never retried silently.
