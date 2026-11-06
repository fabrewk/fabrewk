---
name: codex-review
description: Launches and records Codex's independent review of a part of Fabrewk's bootstrap - a disposable clone under target/, a copy of the Cargo registry, codex exec in a memory-capped scope with no network, a prompt scoped to correctness and coverage of the change, and each pass recorded with its confirmed blocking findings. Use when a part's code or documents are ready for Codex's review, or for a re-check after corrections.
---

> **Fabrewk skill (part 7s, D-060).** Written by the integrator from what the bootstrap record already fixes (the
> plan's rule for every phase, D-019, R-022 to R-029); it decides nothing new. `AGENTS.md` and the maestro's
> authorization come first: the plan names the reviewer, model and effort of each part (Codex GPT-6.1 Sol, effort
> high, for phase 7). Everything here runs under `target/`; nothing is installed.

# Codex's review of a part

## Before launching

- Commit the part. Codex reviews a commit, never a working tree with uncommitted edits.
- Run `verify-part`'s checks first: a review spent on a failing suite is wasted.
- Free memory: no mutation run, build or other review at the same time.
- Use the integrator's own review first when it helps (`differential-review`, `sharp-edges`): a finding fixed before
  the pass saves a pass.

## The review directory

    d=target/review-<part>
    mkdir -p $d/cargo-home
    git clone -q . $d/repo
    cp -r $HOME/.rustup/cargo-home/registry $d/cargo-home/

so `$d/cargo-home/registry` holds the crates the lock file needs (about 35 MB) and Codex builds offline. A new pass
on corrections updates `$d/repo` with `git -C $d/repo pull -q` (or a fresh clone) and keeps the earlier reports.

## The prompt

Write `$d/prompt-<n>.md`. What has worked best (R-022): a review of correctness and coverage, scoped to the change and
to the decisions that fix it. It says:

- That Codex works only inside `$d`; the repository is `repo/`, a disposable clone; `cargo-home/` the registry copy;
  there is no network; it commits nothing, edits no tracked file, and puts scratch files under `repo/target/review/`.
- How to build and test, in bash: `export RUSTUP_HOME=$HOME/.rustup CARGO_HOME=$PWD/cargo-home
  CARGO_NET_OFFLINE=true PATH="$HOME/.rustup/cargo-home/bin:$PATH"` (the rustup shims first: Fedora's `cargo`, first
  on `PATH` otherwise, does not take `+1.99.1`; `CARGO_HOME` is the registry copy, the shims stay in the maintainer's
  rustup folder), then `cd repo && cargo +1.99.1 test --locked` and the crash suite with `--features
  fault-injection`. A binary run by hand runs in a fixture repository under `repo/target/review/`, with `HOME` and
  `XDG_CONFIG_HOME` pointing there, never in a real home.
- The change: `git diff <start>..HEAD` in `repo/`, and the decisions that fixed its scope, to be read.
- The requirements, cases and refusal codes of `core-specification.md` the part delivers, by section and key.
- What is not in the part, and which later part or phase has it, so Codex does not report it as missing.
- That an interpretation written in the specification or approved by the maestro may be challenged only if it
  contradicts a requirement, a past-defect case or an acceptance criterion, citing which.
- What to look for, in order of importance, specific to the part.
- The form of each finding: an identifier `P<part>-n` continuing the part's numbering, blocking or not, file and line,
  a concrete failing input or scenario (a command to reproduce it is best), and what it violates; no style findings;
  a last line `accept` if no blocking finding is confirmed, `reject` otherwise; the report written to
  `repo/target/review/report.md` and printed as the final answer.

A re-check pass names the earlier reports, the corrections' commits, and asks Codex to confirm each earlier finding
closed and to look for new ones in the corrections' scope.

## The launch

From `$d`, inside this repository (6.38: Codex is launched inside a repository, never in `~/Dev`):

    cd $d
    systemd-run --user --scope -p MemoryMax=3G -p MemorySwapMax=0 -p OOMPolicy=continue \
      codex exec --json -m gpt-6.1-sol -c model_reasoning_effort='"high"' -s workspace-write -C "$PWD" \
      "$(cat prompt-<n>.md)" < /dev/null > pass-<n>.jsonl 2> pass-<n>.err

Standard input is closed; `workspace-write` keeps Codex's writes inside `$d` and gives it no network. Run it in the
background and wait for it to end; a pass takes several minutes. Then read the last `agent_message` of
`pass-<n>.jsonl` and `repo/target/review/report.md`, and `pass-<n>.err` for a quota or sandbox failure: a pass cut
short is run again, never counted as an accept.

## After each pass

- Check every finding yourself before correcting: reproduce it, or show why it does not hold. A finding is confirmed
  when it reproduces or the code plainly contradicts what it cites.
- For each confirmed finding: a test that fails without the correction, then the correction, then that test passing.
  Commit the corrections of a pass together, naming the findings.
- Record per pass: the commit reviewed, the confirmed blocking findings with their keys and a few words each, and
  whether each new test failed without its correction. The plan's rule for every phase applies: before each retry the
  progress measure is fixed (confirmed blocking findings); two consecutive retries without progress produce
  alternatives for the maestro; acceptance criteria are never changed to pass.
- Codex's model and CLI version go in the session's R entry (`codex --version`; the stream names no model).

## Cleaning up

When the part is accepted, remove `$d` (a review directory with its clone and registry copy is several GB after the
builds), and anything Codex left in `/tmp`, which is memory on this machine.
