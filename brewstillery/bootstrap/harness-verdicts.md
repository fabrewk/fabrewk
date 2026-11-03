# tickbrew-harness: verdict per file

Plan phase 3, part 3a (D-023), written by the integrator in session 4. Each tracked file of
`/home/xel/Dev/TickBrew/tickbrew-harness` at `0000bdc` gets a verdict and its reason. Part 3b turns the defects found
by past reviews into test cases; Codex reviews both parts together when phase 3 ends. Provenance is in R-009.

## What was read and run

- All 40 tracked files, whole (5,927 physical lines); the subjects of the 21 commits of `gate/fz-2b` and the three
  latest messages whole.
- The self-tests, once, in a disposable clone in the session's scratch directory, with fake CLIs and no provider
  contacted: 198 tests passed in 63 s (Python 3.14.8, Git 2.56.0, bubblewrap 0.12.0). The size count passes: core
  3,089 of 3,500 logical lines, adapters 905 of 1,000.
- Three probes written by the integrator in the same clone, to confirm what the reading suggested (see "Defects found
  by this reading"). The clone was deleted afterwards.

## The three verdicts

- **Reuse as design.** Fabrewk keeps the file's mechanism. Its code is written again inside Fabrewk's structure
  (`brewstillery/`, the `fabrewk` command, the decided settings) and may follow this file closely. Whether any code is
  carried over literally is decided with the implementation language and the licence (phase 5).
- **Rewrite.** Fabrewk needs what the file does, but the mechanism has to change; the reason says what.
- **Discard.** Fabrewk does not need the file. It stays in its origin as reference (alambique section 4).

Totals: 25 reuse as design, 7 rewrite, 8 discard.

## In short

What Fabrewk takes from tickbrew-harness:

1. An append-only event log as the only authority, the state derived by replaying it, every write guarded and
   serialized by compare-and-swap (6.26, 6.37). Where the log lives is still open (6.27).
2. Transitions guarded in code by a pure reducer; a person's event needs a person and words, bound to a digest of the
   exact action (6.24, 6.28).
3. Checks and verdicts bound to the identity of their exact input, recomputed whenever used and stale on any change
   (6.29, 6.43).
4. Attempts recorded before they run and ended exactly once; an unknown outcome waits for the person and is never
   retried blindly (6.41).
5. Agents work on an exported copy without `.git` or the domain; the harness computes the change set and commits;
   lanes are checked by an exact matcher that cannot backtrack (6.37, 6.38).
6. Human messages only from catalogues, machine tokens in English, every key shown with its description (6.18, 6.20).
7. A fail-closed audit of what a reviewer ran, kept as evidence; the providers' raw output stays on the machine
   (6.29, 6.38).
8. A size budget in logical lines, checked by a tool (6.2).

What must change: loading the configuration and its levels, the command line, the provider adapters, the message
texts, the installation guide and the configuration example. Each is tied to `.meta-harness/`, `tickbrew_harness`,
TickBrew's vocabulary or settings that Fabrewk has decided differently.

## Verdict per file

Lines are physical lines at `0000bdc`.

### Core (`tickbrew_harness/`)

| File | Lines | Verdict | Reason |
| --- | ---: | --- | --- |
| `state.py` | 132 | Reuse as design | The log on its own Git ref, one commit per transition, compare-and-swap, a hash chain, a rollback mark replaced atomically, and only a newline ends an event (FZ2B-C3-05). Meets 6.26 and 6.37; its location leaves 6.27 open (observations). |
| `machine.py` | 297 | Reuse as design | Pure reducer with every guard in code, refusing before any change; person events need words and bind to digests. Fabrewk needs one declarative table and the halt code apart from its reason (observations), and the task fields of 6.28 that are missing: dependencies, allowed files, profile. |
| `identity.py` | 42 | Reuse as design | Canonical JSON digests, the input identity of a check or a verdict, stale fields recomputed on use (6.29, 6.43). The policy part must bind to a revision (observations). |
| `checks.py` | 48 | Reuse as design | Only a finished, successful check counts; the gate takes the union of the configured and the approved checks and recomputes their identity (6.43, FZ2-C5-03). |
| `orchestrate.py` | 386 | Reuse as design | The run loop: attempt recorded before the dispatch and ended once, a lost result sent to the person, copies without the domain, the change set computed by the harness, the merge revalidated before and after a compare-and-swap move. Holds H3A-01 and H3A-02, which the new code must not repeat. |
| `gitops.py` | 95 | Reuse as design | Git plumbing only, objects read as stored (no replacement refs or grafts), commits through a temporary index, paths delimited by NUL. Its `export` holds H3A-03. |
| `config.py` | 121 | Rewrite | Loading and validation are tied to `.meta-harness/config/harness.toml` and a single level; Fabrewk has `brewstillery/settings.toml`, personal files and levels still to design (D-002, D-007, D-015; 6.10, 6.11). The lane matcher and write guard in the same file (lines 44–121) are reused as design: exact intersection of glob languages, no backtracking (FZ2-C8-01, FZ2B-C1-01). |
| `schemas.py` | 61 | Reuse as design | One output schema per role, checked again by the core; a rejection needs a confirmed blocking finding and an acceptance may carry none (6.42). Standard library only. |
| `errors.py` | 18 | Reuse as design | Refusals as language-neutral codes with parameters; an adapter error keeps the provider's text out of the log (FZ2R-4). |
| `i18n.py` | 60 | Reuse as design | A language per content class with precedence, a regional variant falling back to its base and then English, keys rendered with their description and a test against bare keys (6.18, 6.20). Class names follow the schema of phase 5. |
| `views.py` | 161 | Reuse as design | Status, the pending decision and the report generated from the log; a missing count is never a zero; a person's time counts only while a decision waited (6.39, 6.40). `STATUS.md` as a view is new for Fabrewk (6.26). |
| `cli.py` | 140 | Rewrite | The command surface is a phase 5 decision; `status` cannot run during a dispatch; the person channel is an interactive-terminal test, a cooperative control. Kept as design: `--say` required and recorded verbatim, the machine code shown with every refusal, work continuing in the same process. |
| `catalog/en.toml` | 180 | Rewrite | The texts name `python3 -m tickbrew_harness`, `.meta-harness` and the board; Fabrewk speaks of the alambique, `brewstillery/` and `fabrewk`. The rules stay: the subject first, no bare key, no words offered to copy (FZ2B-C4-02). |
| `catalog/es.toml` | 180 | Rewrite | Same as `en.toml`; both have the same keys (checked). |
| `__init__.py` | 3 | Discard | Package stub of `tickbrew_harness`. |
| `__main__.py` | 5 | Discard | Package entry point of `tickbrew_harness`. |

### Adapters (`tickbrew_harness/adapters/`)

| File | Lines | Verdict | Reason |
| --- | ---: | --- | --- |
| `confine.py` | 41 | Reuse as design | bubblewrap arguments: read-only system, an empty home, only the CLI and its own credential, the copy at `/work`, no network for checks. Linux only, and bubblewrap itself is still to be evaluated (7c, 6.46). |
| `agents.py` | 205 | Rewrite | Tool, provider and model are one name and the router keys on it (6.14); there is no route when a provider is missing (6.36); the flags are pinned to CLI versions of October 2026, not rechecked; Claude runs with its own permissions bypassed inside the confinement. Kept as design: headless runs with a schema, raw channels kept locally, the CLI's release directory bound read-only. |
| `audit.py` | 177 | Reuse as design | A fail-closed audit of a Codex or Claude stream: any anomaly voids the examination count and an unknown exit code never counts as success (AW1BR-1 to 4, FZ2B-C3-R4, FZ2B-C4-01). It depends on each CLI's stream format. |
| `__init__.py` | 8 | Discard | A one-function indirection. |

### Tests (`tests/`)

| File | Lines | Verdict | Reason |
| --- | ---: | --- | --- |
| `support.py` | 141 | Reuse as design | A disposable repository and a scripted adapter that plays every role with no dispatch: the pattern for Fabrewk's tests. Its fixtures use the old domain and board names. |
| `test_state.py` | 84 | Reuse as design | Integrity of the log: one commit per transition, competing and interrupted writes, edited, removed and unknown-version events. Input of 3b. |
| `test_machine.py` | 79 | Reuse as design | Reducer guards and attempt accounting (AW1BR-2, AW1BR-3). Input of 3b. |
| `test_flow.py` | 359 | Reuse as design | The pilot's refusal list (DESIGN.md section 12): lanes, maintainers' spaces, symlinks, budget, rounds, disputes, checks, stale verdicts, moved base, interruption. Input of 3b. |
| `test_cli.py` | 103 | Reuse as design | Person channel, lock, words kept with the goal, refusal codes, `verify` without a log. Input of 3b. |
| `test_language.py` | 140 | Reuse as design | Precedence, fallback, catalogue parity, bare keys, an authorization independent of its wording. Input of 3b. |
| `test_review_fixes.py` | 223 | Reuse as design | One test per finding of the C5 and C6 reviews (FZ2-C5-01 to 07, FZ2-C6-01 to 03), with the reviews' own variants. Input of 3b. |
| `test_hardening.py` | 601 | Reuse as design | FZ2-C8-01 to 04, with variants that try to beat each fix, a property check of the lane guard, and a crash before and after every event of a whole cycle. Input of 3b. |
| `test_disposition.py` | 520 | Reuse as design | FZ2B-C1-01 to 03, FZ2B-C3-05, FZ2B-C4-02 and 03, the empty-words and ignored-name findings, and the report view. Input of 3b. |
| `adapters/test_adapters.py` | 640 | Reuse as design | The real bubblewrap driven by fake CLIs, and stream audits with each spoiler at each position (FZ2R-1 to 5, C8F-4). Input of 3b. |
| `__init__.py` | 0 | Discard | Empty. |
| `adapters/__init__.py` | 0 | Discard | Empty. |

### Tools, documents and pilot inputs

| File | Lines | Verdict | Reason |
| --- | ---: | --- | --- |
| `tools/size_count.py` | 70 | Reuse as design | Logical lines without comments, blank lines or docstrings, a limit per part, `--check` failing over it; plan phases 6 and 7 need it. |
| `docs/DESIGN.md` | 402 | Reuse as design | Sections 4, 6, 7, 9 and 10 describe the mechanisms reused above, with the reasons for the alternatives rejected (SQLite as the authority, porcelain commits, a tracked file). Section 2 (the old board's 46 points) and section 12 (FZ-2's inputs) are superseded by the alambique and phase 6; section 15's limits are carried in the observations. |
| `docs/INSTALL.md` | 57 | Rewrite | The requirement table and the clean-clone check have the right shape (6.46); the target preparation writes `.meta-harness/` and needs `PYTHONPATH`, while Fabrewk installs by a script or a guided prompt after an assessment (section 1, D-006). |
| `docs/harness.example.toml` | 38 | Rewrite | A useful seed of envelope, profiles and checks, but in one file, at one level and with key names Fabrewk has decided otherwise (`brewstillery/settings.toml`, `canonicalDocumentationLanguage`; 6.10, 6.20). |
| `README.md` | 24 | Discard | Status and paths of TickBrew's pilot; read in session 1 (R-001). |
| `.gitignore` | 1 | Discard | Fabrewk's ignore rules are a phase 5 decision. |
| `pilot/goal.es.md` | 9 | Discard | FZ-2's pilot task; Fabrewk's pilot is a TickBrew module (D-004, D-009). Its shape, a closed rule set and a first step that asks for the specification, is a model for phase 11. |
| `pilot/oracle/oracle.py` | 76 | Reuse as design | A hidden oracle, fixed and digested before the pilot and never shown to the author or the reviewer: the pattern for phase 11's hidden criteria (6.48). Its content belongs to FZ-2. |

## Defects found by this reading

Confirmed in the disposable clone. No test covers them, and no commit message read here names them; part 3b checks
them against the receipts and turns them into test cases.

- **H3A-01. A symlink already tracked in the base stops every task.** `changes()` in `orchestrate.py` (lines 300–312)
  reports every symlink in the author's copy as a change, even an untouched one, so the attempt halts with
  `LANE_VIOLATION` or `SYMLINK_REFUSED`. Probe: a base with `link -> ok.txt`; the task blocked on `link`.
- **H3A-02. A change of the executable bit alone is lost without notice.** `changes()` compares bytes only, so a file
  made executable with the same content is not in the change set; the candidate keeps mode 100644 and the task is
  accepted. Probe: the author runs `chmod 755` on a tracked `tool.sh`; the candidate's tree shows 100644.
- **H3A-03. A symlink to an absolute path in the base raises a raw error before any dispatch.** `Git.export`
  (`gitops.py` lines 63–69) extracts with tarfile's `data` filter, which refuses such a link with `AbsoluteLinkError`.
  Nothing is recorded, the task stays proposed and `run` fails the same way on every call. Probe: a base with
  `abs -> /etc/hostname`.

## Observations for phase 5

Not defects of the pilot: places where the harness's design meets a decision Fabrewk still has to take.

- **Configuration read from the working tree.** `load_config` reads `.meta-harness/config/` from the checkout, not
  from a commit, and the identity keeps only digests of it. DESIGN.md section 6 says the policy digest is the tree hash
  of the configuration at the base; the code differs. So the configuration a dispatch used may be uncommitted and
  cannot be rebuilt from the log (6.11, effective configuration with provenance; 6.29).
- **The state ref travels only on purpose.** A plain clone does not bring `refs/meta-harness/state` (known in the
  harness, FX-7). That fits 6.26 but leaves 6.27 open: a record that does not live on one machine needs the ref pushed
  and fetched explicitly, or another place.
- **No view during a dispatch.** `status` takes the same lock as a dispatch (DESIGN.md section 15); 6.47 needs a read
  path that does not wait for the writer.
- **Confinement.** The Claude adapter bypasses Claude's own permission prompts and relies on the outer `bwrap` alone;
  the shared network and the readable credential are the residual risk the maintainer accepted for FZ-2; `/etc` is
  readable; only Linux has a confinement (6.38, 6.46; bubblewrap still to be evaluated, 7c).
- **Tool, provider and model.** `provider = "claude"` names all three, and the router keys on it; 6.14 asks to tell
  them apart and 6.36 for a route when a provider is missing; neither exists.
- **What the examination count proves.** It guards against a lazy or failed run, not a hostile agent (DESIGN.md
  section 7). Fabrewk must state that limit as plainly (6.38).
- **One table for the transitions.** The allowed moves are spread over `FROM`, `ROLE_FROM` and the handlers, two
  states (`checked`, `merging`) are missing from DESIGN.md section 5's table, and one field mixes refusal codes
  (`ROUNDS_EXHAUSTED`) with reasons (`rounds`), which forces special cases in `_decided`. A single declarative table
  would let a test hold the code and the document together (6.28).
- **Licence.** The repository carries none and belongs to the maintainer; Fabrewk is always open source (alambique
  section 1). Copying code literally is the maintainer's decision, with the licence (phase 5).

## For part 3b

Inputs: the finding keys in the tests and commit messages (FZ2-C5-01 to 07, FZ2-C6-01 to 03, FZ2-C8-01 to 04,
FZ2B-C1-01 to 03, FZ2B-C3-05, FZ2B-C3-R4 and R5, FZ2B-C4-01 to 03, FZ2R-1 to 5, AW1BR-1 to 4, C7C-5, C8F-3, C8F-4,
FX-1 to 8), H3A-01 to 03 above, and the review sections of the receipts in `tickbrew-legacy`. Each confirmed defect
becomes a test case stated against what Fabrewk must do, not against this code.
