# Fabrewk — status

Manual, provisional view, written by the integrator until Fabrewk's single registry exists; then it becomes a
view generated from that registry. Technical files are in English by default (D-008).

## Current state

**Phases 0 to 7 are closed (D-037, D-040, D-070). Phase 7 (the first code) was split into parts 7a, 7b, 7c1, 7c2a,
7c2b, 7d1, 7d2, 7s, 7d3a and 7d3b (D-041, D-047, D-048, D-050, D-053, D-059, D-062), the last the task commands
(D-068, D-069, R-034); phase 8, the core's part 2, was split into 8a, 8b and 8c (D-072) and all three are closed, and so is the phase (D-077); phase 9 is open: its design is accepted and it is cut into parts 9a to 9i (D-075, D-076); parts 9a to 9h are closed and 9i is next.** The repository holds the first code at its root: a
Cargo workspace with one crate per ring (`crates/core`, `crates/fabrewk`, the binary, which runs the core's command
surface, `crates/confine-bwrap`, the confinement backend for Linux, `crates/adapter-claude`, Claude Code's adapter, and `crates/adapter-codex`, Codex's) and the size counter `tools/budget`, the licence files, `deny.toml` and a root `.gitignore` with `/target`.
The record is built in `crates/core/src/record/`, the settings in `crates/core/src/settings/`, identity in
`identity.rs`, the private folder and the machine file in `machine.rs`, detection and profiles in `detect.rs`, the
domain's layout in `domain.rs`, the transition table in `table.rs`, the payloads and pattern guards in `task.rs`, the
credential scan in `scan.rs`, the fermento in `fermento.rs`, the alambique and the rules in `alambique.rs`, the cell's contract and its runner in `confine.rs`, the attempt's copy, its change set and the candidate in `candidate.rs`, the tool adapter's contract in `adapter.rs`, the checks, the identities and the rounds' measure in `gate.rs`, the run in `crates/core/src/cli/run.rs`, the integration in `merge.rs` and `crates/core/src/cli/accept.rs`, the extensions' runner in `crates/core/src/cli/ext.rs`, and the command surface in `crates/core/src/cli/` with its catalogue in
`crates/core/catalogue/` (`en`, `es`); `fabrewk status`, `fabrewk config` and `fabrewk check` read, and `fabrewk
adopt`, `fabrewk propose`, `fabrewk authorize`, `fabrewk reject`, `fabrewk sweep`, `fabrewk discard`, `fabrewk approve`, `fabrewk close`, `fabrewk run` (round after round of the builder, the checks and the reviewer, to a verified task or a halt) and `fabrewk accept` (an acceptance and its integration) write, and `fabrewk <name>` runs a permitted extension confined and applies its requests, through one
write path in `crates/core/src/cli/write.rs`. The skills the maestro approved are in `.brewstillery/skills/`, listed in `AGENTS.md`. The core counts 11,607 of 12,000 lines, the backend 278 of 400, Claude Code's adapter 269 of 600 and Codex's 319 of 600, formatted at width 120 (D-049, D-054, D-074, D-075). No `data/` exists in this
repository, whose project file still holds bootstrap keys, so `fabrewk check` here reports them. The domain is
`.brewstillery/`, hidden (D-032).

## Read first

1. `.brewstillery/ALAMBIQUE.md`: the description of what Fabrewk must be, approved by its maestros, with status keys
   defined in its opening. Nobody edits it directly; it authorizes nothing by itself.
2. `.brewstillery/xel/fermento.md`: the maintainer's input (above the separator) and the agents' answers (below it).
3. `.brewstillery/bootstrap/decisions.md`: the decisions in force, with their origin, and what each session read.
4. `.brewstillery/bootstrap/plan.md`: the approved construction plan.
5. `.brewstillery/settings.toml` (project) and `.brewstillery/xel/settings.toml` (personal; never grants authority).
6. `.brewstillery/bootstrap/core-specification.md`: the frozen specification phases 7 to 10 build, its tests and
   acceptance criteria (D-040).
7. `.brewstillery/bootstrap/confinement-design.md`: phase 9's threat model, the confinement's design, what it
   amended in the specification and the cut into parts (D-075, D-076). Every part of phase 9 starts from the sections
   its row names.

## Evidence

`.brewstillery/bootstrap/` is the provisional record of Fabrewk's self-construction (D-005). It exists only in this
repository, is imported into the single registry at production and then stays read-only.

## Latest and next

- **Done:** the legacy TickBrew repository now lives at `/home/xel/Dev/TickBrew/tickbrew-legacy`, frozen at tag
  `legacy-final` (R-004). Documentation publication languages: `en`, `es-MX` (D-014).
- **Phase 1 closed (D-018):** the maintainer adopted the audited input (`.brewstillery/bootstrap/alambique-audit.md`),
  with a usage note and section 8 as their channel. Phases are split into parts only when needed (D-019, in
  `plan.md`). The answers zone holds only current recommendations (D-017).
- **Phase 2 closed (D-021, R-008):** Codex's review had one blocking point (6.38), settled by the maintainer: it
  stays "propone". Plan sentence on append-only clarified in `AGENTS.md`.
- **Phase 3 closed (D-025).** Part 3a (R-009): every file of `tickbrew-harness` has a verdict in
  `.brewstillery/bootstrap/harness-verdicts.md` (25 reuse as design, 7 rewrite, 8 discard). Part 3b (R-010): every
  confirmed defect found by past reviews, and the three found in 3a, is a test case against Fabrewk's behaviour in
  `.brewstillery/bootstrap/past-defect-cases.md` (46 cases). Codex (GPT-6.1 Sol, high) reviewed both: reject, two
  scoped re-checks, accept; every confirmed finding corrected.
- **Phase 4 delivered (D-026, D-027).** `.brewstillery/bootstrap/reference-study.md`: part 4a (R-011), Pullboard at
  source, catalogues and profiles of agents (6.14, 6.15), permitted versus present (D-015) and installers that preview
  their plan; part 4b (R-012), section 7a (UCI, apk-tools, SQLite, with three storage options for 6.26 and 6.27) and
  the consolidated source catalogue (6.30, 61 sources: studied, checked or pending). Codex (GPT-6.1 Sol, high) reviewed
  both: reject, two scoped re-checks, accept; every confirmed finding corrected. **Phase 4 closed (D-029).** UCI and
  apk-tools stay as design patterns only; their schema is out of scope (D-028).
- **Phase 5 in progress, one part (D-030).** `.brewstillery/bootstrap/architecture-decisions.md` gives options, consequences
  and a recommendation for AD-1 to AD-11; Codex (GPT-6.1 Sol, high) accepted it at its third pass (R-015). Decided
  (D-031): AD-1, the registry as immutable event files under `.brewstillery/data/`; AD-2, settings in TOML with five
  levels and authority read from a committed or confirmed policy; AD-3, the language classes. Licence criterion
  (D-030): open source, able to live beside any open or closed product, obliging only credit; decided without a
  professional.
- **Session 9 (D-032, D-033, R-016).** The sweep of 6.47.a and section 8 became eighteen decisions on the domain,
  governance and core architecture (D-032): hidden domain `.brewstillery/`; one project `ALAMBIQUE.md`, never edited
  directly, and one `fermento.md` per maintainer; maestro, supervisor and contributor roles with the SSH signing key as
  identity; a private folder per machine; a Rust core with extensions as separate programs; one confinement backend
  per operating system; semantic search and the central server as later stages. The maintainer annotated the proposed
  text (D-033: title, opening analogy, usage list, status keys, installation). The second audit
  (`.brewstillery/bootstrap/alambique-audit.md`) holds the full proposed alambique (Part 2) and fermento (Part 3). The
  integrator's half of the migration is committed: the domain renamed, the old alambique moved to `ALAMBIQUE.md`,
  entry files and this file updated.
- **Session 10 (D-034, R-017).** Migration check passed. The fermento's answer now also carries, for each proposal
  with consensus, the drafted alambique text. The alambique was rewritten concise for the maestro and the integrator
  (−18 % in words; section 7 moved to the source catalogue, which gained C-62 to C-66 and the maintainer's notes); the
  original is archived as `.brewstillery/bootstrap/alambique-original.md`. Codex accepted the rewrite at its second
  pass. The preview is in XEL's fermento answer, between the markers.
- **Adopted (D-035).** The maintainer validated the rewrite in chat and copied it over `.brewstillery/ALAMBIQUE.md`
  (sha256 checked against the preview). The fermento's answer recommends removing point 2.
- **Session 11 (R-018).** Phase 5 resumed. `architecture-decisions.md` now has AD-4 (Rust: MSRV, platforms, crates,
  tests), AD-5 (three rings and the extension contract), AD-7 (release binaries, the `curl` route with onboarding in
  the binary, the agent route ending at `assess`, downloads checked by an SSH signature) and the new AD-12 (domain
  layout, `RULES.md`, separator marker), AD-13 (alambique edits and proposal state), AD-14 (identity, roles, signed
  events) and AD-15 (private folder, instance identity). Codex accepted it at its second pass.
- **Decided (D-036, R-019).** The maintainer adopted AD-4 to AD-15 as recommended; Codex re-checked the decided file
  and accepted. Every decision of the phase 5 packet is decided. The licence is MIT OR Apache-2.0 (files come with the
  first code).
- **Phase 5 closed (D-037).** On the maestro's authorization the integrator applied the two drafted alambique edits
  (licence in section 1, `RULES.md` in section 2). `layout.reservedNames` now includes `local`, `RULES.md` and
  `.gitignore`. The maintainer then removed fermento points 1 and 2; the answer has no current recommendation.
- **Phase 6 started (D-038, R-020).** Split into two parts. The size budget is frozen at phase 7's start, after
  measuring the skeleton, since no code exists before then. Part 6a is delivered in
  `.brewstillery/bootstrap/core-specification.md`: every alambique point classified, 101 requirements, the record's
  events, the transition tables, 16 commands, `fabrewk check`, 83 refusal codes, the settings keys, the items phase 5
  left open, and nine choices for the maestro to approve with the text (its section 10).
- **Phase 6 closed (D-039, D-040, R-021).** Part 6b added to `core-specification.md` the test plan (every past-defect
  case and every refusal code with its test, whole flows, a crash suite, mutation and size rules), an acceptance
  criterion for each of the 103 requirements and the phase each is due in, and corrected part 6a where the tests
  showed gaps (section 11.1). Codex (GPT-6.1 Sol, high) accepted at its fifth pass, confirmed blocking findings 13, 5,
  2, 1, 0. The maestro approved the text and its 13 choices; it is frozen. Start prompts now name their phase
  (`AGENTS.md`, handover). The previous closing commit had folded the maintainer's fermento edit into the integrator's
  commit; recorded in R-021.
- **Phase 7 started; part 7a paused (session 14; D-041 to D-046, R-022).** Four parts (plan, "Phase 7"). The skeleton
  and its counter are committed; the maestro froze the budget at AD-5's ceilings after measuring it (core 12 of
  4,000 lines, 0 crates, the musl binary 397,600 bytes of 8,000,000). The suites run with the rustup toolchain 1.99.1,
  which builds musl (D-043). D-044 fixes what the counter guarantees: it fails closed on every route Cargo or the
  compiler record (dep-info, one unfiltered `cargo metadata`, a closed world of local packages, no `.cargo` directory,
  no `RUSTFLAGS`, no wrapped compiler); a hiding only the diff shows is the review's. Codex reviewed nine times,
  confirmed blocking findings 4, 3, 3, 4, 4, 1, 1, 1, 1; all 23 corrected; the maestro chose the alternatives twice
  (D-045). The last correction (`a40c769`, any backslash in the dep-info refuses) is not reviewed yet. The alambique
  gained two [proyectado] sentences on a memory of lessons per machine (D-046); the maestro then removed the
  fermento's point 1, and its answer has no current recommendation.
- **How to run the suites (fish):**
  `env RUSTUP_HOME=$HOME/.rustup CARGO_HOME=$HOME/.rustup/cargo-home PATH="$HOME/.rustup/cargo-home/bin:$PATH" cargo +1.99.1 test --locked`;
  the musl release build is the same prefix with `cargo +1.99.1 build --release --locked --target
  aarch64-unknown-linux-musl -p fabrewk`, then `cargo run --locked -p budget -- check --binary
  target/aarch64-unknown-linux-musl/release/fabrewk`. With Fedora's `cargo` alone, one test fails by design (D-043).
- **This machine's lesson on memory (R-022).** 7.3 GB of RAM, and `/tmp` lives in it. `cargo mutants` runs only in its
  own scope with a cap per process: the same prefix, then `systemd-run --user --scope -p MemoryMax=3G -p
  MemorySwapMax=0 -p OOMPolicy=continue bash -c 'ulimit -v 1000000; exec cargo mutants --workspace -j 2 --timeout 90'`,
  with `TMPDIR` under `target/` (on disk): the session's scratch directory is memory with a per-user quota, which
  stopped a mutation run in session 16 (R-024); Codex's disposable clone goes there too. Run Codex in the same kind of scope, one heavy job at a time, and
  remove reviewers' leftovers from `/tmp`. Without this, a mutant that loops on memory took VS Code down.
- **Part 7a closed (session 15, R-023).** Codex accepted at its tenth pass (P7A-23 closed, no new finding in scope;
  one residual, a file named with a tab that Cargo's dep-info omits, reachable only as data). Full mutation run: 492
  mutants, only the two declared equivalents missed, 24 timeouts that are loops. Clean clone: 61 tests green, musl
  build, `budget check`, clippy, fmt and `cargo deny` clean; the measure of D-042 unchanged. Codex's prompts work best
  worded as a review of correctness and coverage, scoped to the change and to D-044 (R-022).
- **Part 7b closed (session 16, D-047, R-024).** The maestro approved the cut before any code (D-047): 7b is the
  record as a library tested at its level; REQ-REG-12 and the signers derived from the policy go to 7c;
  `RECORD_TRANSITION_INVALID` from the table, the stage and task payloads and the command-line refusal tests to 7d; a
  new code `RECORD_HEAD_MOVED`. Codex accepted at its third pass (confirmed blocking findings 3, 1, 0; P7B-1 to P7B-8
  corrected). Mutation: every mutant caught but two declared equivalents. Clean clone: 118 tests and the crash suite
  green, musl build, `budget check`, clippy, fmt, `cargo deny` clean. Interpretations the maestro may want to read
  are in `core-specification.md` section 3, "Schema 1's envelope". With `rustfmt`'s default width the record counts
  212 lines more than at 120 (settled by D-049).
- **Part 7c1 closed (session 17, D-048, D-049, R-025).** Before any code the maestro split 7c into 7c1 and 7c2 (SEC-5
  moves to 7d), chose to verify a signature with the key it carries against the cited policy and
  `ssh-keygen -Y check-novalidate` (no allowed-signers file, so reading commands write nothing; REQ-REG-4 amended),
  and `toml` 1.2 reading TOML 1.1 without `serde` (REQ-SET-1 amended). 7c1 delivers every key of section 8 declared
  once, the three files read with file, key path and line in every refusal, the policy in force from the last
  `policy-adopted` (its payload is now `content`, `digest`, `widened`), pending changes located, each change classed
  by direction, adoption's refusals, and the effective values with their sources and the stage and task layers. With
  7c1 the core reached 2,915 counted lines at `rustfmt`'s default; the maestro chose width 120 with "Max" heuristics
  (D-049, `rustfmt.toml`): 2,102 now. Codex accepted at its seventh pass (confirmed blocking 5, 2, 1, 2, 1, 1, 0; most
  passes were on how a TOML syntax error names its key path, now resolved by the parser itself). Mutation: caught but
  four declared equivalents. Clean clone: 144 tests and the crash suite green, musl build, `budget check`, clippy,
  fmt, `cargo deny` clean.
- **Part 7c2a closed (session 18, D-050, R-026).** Before any code the maestro split 7c2 into 7c2a and 7c2b, fixed
  the policy the events before the first adoption cite (the first adoption's own digest), and chose a seal: every
  `policy-adopted` records each other stream's head, and a person's event must hold its key and role both in the policy
  it cites and in the one in force at its place, so removing or re-keying a maintainer takes effect even against a
  hand-forged event. 7c2a delivers the policy verifier (`identity.rs`), `ROLE_FORBIDDEN`, the derived
  `allowed_signers`, the key report, the private folder (`0700`, `PRIVATE_FOLDER_PERMISSIONS`) and the machine key
  with its fingerprint (`machine.rs`). Codex accepted at its second pass (confirmed blocking 1, 0). Mutation: caught
  but one declared equivalent. Clean clone: 159 tests and the crash suite green, musl build, `budget check`, clippy,
  fmt, `cargo deny` clean.
- **Part 7c2b closed (session 19, D-051, D-052, R-027).** Before any code the maestro decided that an invalid
  profile is `SETTINGS_INVALID` (an absent tool `TOOL_MISSING`), that the core learns a program's executable and
  version arguments through a contract each built-in module fulfils (fake modules until phase 9), and the scope.
  7c2b delivers `machine.toml` read from the private folder and shown as declared; detection in memory at every start,
  in three spaces apart (programs, catalogued tools, required tools); `local/cache/detected.json`, written only by
  write commands, carrying only "verified by use"; the effective tools; `check_profile`; the domain's layout
  (`DOMAIN_UNKNOWN_ENTRY`, `ALIAS_INVALID`; `bootstrap/` is an unknown entry, a warning); the exact `.gitignore`
  (`GITIGNORE_CHANGED`); `publication-decided`'s payload (`mode`, `warningShown`); `DOMAIN_NOT_IGNORED` with `git
  check-ignore` at the product's root. After Codex's first pass the maestro chose `sh`'s `test -x` to tell whether a
  file is executable (D-052). Codex accepted at its second pass (confirmed blocking 4, 0). Mutation: caught but two
  declared equivalents. Clean clone: 172 tests and the crash suite green, musl build, `budget check`, clippy, fmt,
  `cargo deny` clean.
- **Part 7d1 closed (session 20, D-053, D-054, R-028).** Before any code the maestro split 7d into 7d1 (the table and
  the payloads, as a library), 7d2 (the command surface and the reading commands) and 7d3 (the write commands); the
  declaration of commands grows by phase, so `MOVE_NOT_ALLOWED`'s tests of `accept`, `approve` and `close` move to
  their phases; `run` evaluates its guards up to confinement, which refuses until phase 9. 7d1 delivers section 4 as
  one declarative table with each guard labelled effective or cooperative, the replay held to it (bindings, attempts),
  the four stage and task payloads, the pattern language with its guards, `CHECK_DUPLICATE`, `LIMIT_UNOBSERVABLE`
  and the credential scan. Codex accepted at its third pass (confirmed blocking 3, 1, 0). Mutation: caught, none
  equivalent. Clean clone: 197 tests and the crash suite green, musl build, `budget check`, clippy, fmt, `cargo deny`
  clean. Measured at +548 lines (estimated 750), the maestro raised the core's ceiling to 7,500 (D-054).
- **Part 7d2 closed (session 21, D-055 to D-058, R-029).** Before any code the maestro decided that a command knows
  the person by Git's signing key (`user.signingkey`, matched with the policy; their file gives the language), that
  the fermento and the alambique cite by Markdown links, that completions and the reference are generated files, and
  the scope. 7d2 delivers one declaration of commands (`lexopt` as a lexer) with the parser, help, the fish and POSIX
  completions and the reference in `en` and `es` generated from it; the catalogue in `en` and `es` as data; `--json`
  documents with a pinned `schema`; exit statuses 0, 1, 2, 3 and 70; `status`, `config` and `check` (C1 to C4, C8 to
  C11), reading without a lock and writing nothing. Codex reviewed ten times (confirmed blocking 9, 7, 5, 4, 1, 2, 1,
  1, 1, 1), mostly on what a reading command may open: no link followed on the way to a file, only regular files,
  nothing read from any Fabrewk domain where the person names a place, what cannot be examined never taken for absent.
  The maestro decided citations are read lexically (D-056), a tenth pass after two retries without progress (D-057),
  and that what Git reads to run is Git's (D-058), which accepted the part. Mutation: caught but one declared
  equivalent. Clean clone: 257 tests and the crash suite green, musl build, `budget check`, clippy, fmt, `cargo deny`
  clean. Measured at +1,231 lines (projected 650 to 900); corrected projection in R-029: the ceiling of 7,500 may not
  hold through phase 9, the maestro's to decide when 7d3 is measured. A mutation run filled the disk once: the test
  fixtures are now swept when their process ends. The maestro's fermento edit (point 1, skills for the agents) was
  folded into an integrator's commit, as in R-021; its answer is current.
- **Skills for the agents (D-059).** After the closing commit the maestro made the fermento's point 1 a proposal and
  approved the two alambique edits the integrator drafted: `.brewstillery/skills/` for approved skills, read as
  `AGENTS.md` says (section 2), and the search and evaluation of skills in every project, asked by each alambique and
  done in the bootstrap before the core continues (6.38). A part 7s runs before 7d3.
- **Part 7s closed (session 22, D-060, R-030).** `.brewstillery/bootstrap/skills-evaluation.md` evaluates the
  candidates (Trail of Bits, OpenAI, Anthropic, Superpowers, community Rust skills) by permissions, provenance,
  version, licence and effects. The maestro adopted `property-based-testing`, `sharp-edges` and `differential-review`
  from Trail of Bits, adapted (CC BY-SA 4.0, no key that pre-approves tools, a notice that puts `AGENTS.md` first), and
  two skills written from the record, `verify-part` (the checks, clean clone and mutation on this machine) and
  `codex-review` (launching and recording Codex). They live only in `.brewstillery/skills/`; `AGENTS.md` says when to
  read each, and no tool loads them by itself. The core reserves `skills` and lists `skills/` in the domain
  (REQ-DOM-2, REQ-DOM-3 amended). Codex accepted at its third pass (confirmed blocking 3, 1, 0). Clean clone: 257
  tests and the crash suite green, musl build, `budget check`, clippy, fmt, `cargo deny` clean. After the close the
  maintainer removed the five TickBrew copies from their user folders, added `skills` to `layout.reservedNames`, and
  approved a sentence in 6.38: Fabrewk reports skills loaded from outside the project and warns on a duplicate name
  or a contradiction, never touching them (D-061; a requirement is due when phase 8 or 10 starts). The disk had 4.9 GB free at the start; old clones under `target/` were removed.
- **Part 7d3a closed (session 23, D-062 to D-065, R-031).** Before any code the maestro split 7d3 into 7d3a and 7d3b,
  gave `run` the person's words, made `authorize` take `--seen <digest>` (the digest of what `status <id>` showed,
  `TASK_CHANGED` otherwise, carried as `shown` in the authorizations' payloads), and had the binary declare the
  agents' markers (D-062). 7d3a delivers the declaration's required argument and options with a value (`--say`,
  `--say-lang`, `--widen`); one write path (guards on the record read without the lock, the session, the guards again
  under the lock, every event made and signed in memory, the machine key put in place after, the leftovers' commit,
  the events by compare-and-swap, the record commit, the caches under the lock); signing with Git's signing key;
  `AGENT_ENVIRONMENT`, `WORDS_MISSING`, `WORDS_COPIED`, `STAGE_CLOSED`; `adopt` and `propose`; the record's refusals
  through the command line and the fault tests (`crates/fabrewk/tests/faults.rs`). Codex reviewed six times
  (confirmed blocking 5, 5, 5, 3, 2, 0): after three passes without progress the maestro settled two findings as the
  design (D-063: what a write command cleans and keeps before it refuses), then asked a fifth pass (D-064) and settled
  that a failed record commit leaves its events, saying so (D-065). Mutation: caught but none equivalent. Clean clone
  of `9d91a59`: 300 tests, `crash` 3 and `faults` 8 green, musl build, `budget check`, clippy, fmt, `cargo deny` clean.
  Measured at +837 lines; the projection in R-031 puts the core at about 7,650 to 8,550 after phase 9, so the ceiling
  of 7,500 is the maestro's to decide when 7d3b is measured. The maestro approved two edits to `verify-part` (its fault
  suites include `faults`). The maestro's fermento edit went in its own commit (`2577784`). After the close the maestro
  proposed in their fermento that the onboarding guide a new maintainer to a first alambique; their edit was folded
  into the integrator's commit `972ebe5` (R-032, the slip of R-021). After the sweep the maestro approved the item,
  with a pedagogical explanation of the names' analogy, now in the alambique's section 1 (D-066; a requirement is due
  when phase 10 starts). The fermento's answer says the proposal may be removed.
- **Session 24, before 7d3b (R-033, D-067).** The machine's root disk went from 2.9 to 33 GiB free: caches and
  superseded tool versions removed, inactive folders moved to the external disk (`/home/xel/Dev/external`, compressed
  per folder), old snapper snapshots removed by the maestro. `~/Dev` holds only `Fabrewk`, `TickBrew` (still cited and
  needed by the pilot) and `external`; `~/Dev/tmp` now lives at `/home/xel/Dev/external/tmp`. A `git bundle` of the
  repository is kept at `/home/xel/Dev/external/backups/fabrewk/`. The maestro's cleanup scripts live outside the
  project (`~/Dev/tools/dev-clean.sh`, `~/System/Asahi/Fedora/system_maintenance.sh`). The maestro approved two
  alambique edits (D-067): Fabrewk's own disk discipline in 6.4, 6.41 and 6.46 (a requirement due when phases 9 and 10
  start), and 6.53, a staged self-rebuild after the pilot, `[proyectado]`. Check the free disk space (15 GiB or more)
  before a clean clone or a mutation run.
- **Part 7d3b closed; phase 7 closed (session 25, D-068 to D-070, R-034).** Before any code the maestro decided that
  `run`'s `BASE_MOVED` and REQ-RUN-2 are evaluated from phase 9, that `authorize` without `--seen` is a usage error,
  that `reject` binds its task and head and refuses `MERGE_IN_PROGRESS` by default until phase 9, and that both
  authorizations check `PROTECTED_PATH_WILDCARD` again (D-068). 7d3b delivers `authorize <stage|task> --seen <digest>
  --say [--widen <key>]… [--run]`, with what it binds shown by `status <id>` with its digest (`fabrewk.subject/2`,
  `shown` in both authorizations' payloads), `reject <task> --say` and `run <task|stage> --say` up to confinement
  (`CONFINEMENT_UNAVAILABLE`, nothing written). Codex reviewed three times (confirmed blocking 2, 1, 0): the rendering
  is now worded in the language the record under the lock gives, and every human output reaches the terminal with
  control characters, bidirectional marks and separators written as escapes. A first mutation run was void (the
  fault suite already failed; the integrator had skipped its step); the valid run missed 8 mutants, all killed; a
  compiler held at the cap per process waited 1 h 45 min, so `verify-part` gained a build timeout (D-069). Clean clone
  of `29c4f25`: 324 tests, `crash` 3 and `faults` 9 green, musl build, `budget check`, clippy, fmt, `cargo deny` clean.
  Measured at +482 lines (5,859); the maestro kept the ceiling at 7,500 and decides it again when phase 9 starts, with
  phase 8's measure (projected 6,650 to 6,950 after phase 8, 7,650 to 8,350 after phase 9's core). The maestro's
  fermento edit went in its own commit (`230ff56`); the answer has no current recommendation. Then the maestro closed
  phase 7 (D-070).
- **Part 8a closed (session 26, D-072, R-035).** Before any code the maestro cut phase 8 into 8a, 8b and 8c and chose
  to record REQ-FER-10 (the skills loaded from outside the project) in `sweep-recorded` (D-072). 8a delivers the
  separator finder outside fenced code (REQ-FER-1; an ambiguous or damaged one stops with nothing written), the
  points and their digests, the proposals in the replay and the table, `sweep --answer <file>` and `discard <id>`
  with the answer zone generated from the events in the maintainer's language, `check`'s C6 and C12, and ten codes
  (`ANSWER_INVALID` to ratify). Codex reviewed three times (confirmed blocking 4, 1, 0): a CRLF fence, a staged edit
  of the fermento, two maintainers' identical edits, a printed command carrying words, a hash collision across aliases.
  Mutation: 15 of 318 survived the first run and were killed; none equivalent. Clean clone of `cf3db8d`: 358 tests,
  `crash` 3 and `faults` 11 green, musl build, `budget check`, clippy, fmt, `cargo deny` clean. Measured at +803
  lines (6,662): phase 8 will likely pass 7,300 and phase 9's core the ceiling, which the maestro decides when phase 9
  starts (D-069). `verify-part`'s `ulimit -v 1000000` no longer fits the core's build (1,600,000 was used); the skill
  is the maestro's to change.
- **Part 8b closed (session 27, D-073, R-036).** Before any code the maestro decided that with no baseline C7 passes for
  no file and refuses a file nobody approved (`approve alambique` then records the baseline), that `approve alambique`
  also commits the alambique alone, that a direct edit records its difference as one hunk, and that
  `ALAMBIQUE_UNAPPROVED` is evaluated by `run` before confinement (D-073). 8b delivers `approve <proposal|alambique>
  --say`: an alambique proposal applies its edit by whole units (opening, section, point; `UNIT_AMBIGUOUS`), checks
  `PROPOSAL_STALE`, `ALAMBIQUE_CHANGED` (file, tip and index) and `ALAMBIQUE_UNAPPROVED`, and commits the alambique
  alone with the person's Git as author and no co-author; a `rules` proposal records all the rules in the stream
  `rules`; `alambique-baseline`, `direct-edit-approved`, C7, the replay's checks of both streams, and five codes
  (`ALAMBIQUE_MISSING` to ratify). Codex reviewed three times (confirmed blocking 5, 1, 0): the index not checked
  again before the file is written, a section that ended only at `# ` or `## `, a refusal after the machine key was
  put in place, an index the commit could not follow, a replay failing after the alambique's commit, and the fix
  printed for the wrong repository in `own-repository`. Mutation: 197 mutants, all missed ones killed, none
  equivalent. Clean clone of `2debde4`: 380 tests, `crash` 3 and `faults` 17 green, musl build, `budget check`,
  clippy, fmt, `cargo deny` clean. Measured at +473 lines (7,135): 8c will likely cross the ceiling of 7,500 before
  phase 9; the maestro decides it (D-069), preferably before 8c starts.
- **Part 8c closed (session 28, D-074, R-037).** Before any code the maestro raised the core's ceiling to 8,000, approved
  the twelve base rules of `RULES.md` with the first rule worded as they dictated (Git's permissions are the supreme
  authority; the alambique with the fermentos is the workbench that gives the plan its form; the chat stands above
  them for extraordinary changes, always sketched in the alambique; no change without the maestro's signature), and
  proposed `distill` for `sweep` after the pilot (D-074). 8c delivers `STATUS.md` and `RULES.md` as pure functions
  of the replay and the catalogue (`crates/core/src/view.rs`), written by every write command once its events replay,
  held in the record commit and compared by `check` (C5, `VIEW_STALE`); what waits and for whom, with the time of
  the event that began each wait, in the views and in `status` (`fabrewk.status/2`), which offers each person only
  their commands; and `fabrewk close <stage> --say` (`STAGE_OPEN_TASKS`, `SWEEP_MISSING`, `ALAMBIQUE_UNAPPROVED`,
  `RECORD_UNCOMMITTED`; `stage-closed` binds the stage's head and the sweeps it relied on). Codex reviewed four times
  (confirmed blocking 3, 2, 1, 0), mostly on what a write may follow or restart: a link at a temporary name or at the
  lock, a wait restarted by a later sweep, a stage with no task. P8C-8, left to the maestro, is the one residual: a
  directory above the lock swapped in the instant between check and open; closing it takes `openat` per component, a
  crate no maestro approved. Mutation: every missed mutant killed or removed, none equivalent. Clean clone of `a1bb6bb`:
  404 tests, `crash` 3 and `faults` 19 green, musl build, `budget check`, clippy, fmt, `cargo deny` clean. Measured at
  +495 lines (7,630): phase 9's core will cross 8,000, which the maestro decides when it starts (D-069). A flake of
  8b's library tests (a lock held by another fixture's `fork`) was fixed. An untracked `.vscode/settings.json` made by
  the editor is not in any commit.
- **Phase 9 opened (session 29, D-075, D-076, R-038).** `.brewstillery/bootstrap/confinement-design.md` holds the
  threat model (29 threats, each with its control, its label and what shows it), the confinement's design and the cut.
  Probed on this machine before designing: with the host's network shared, a confined process reaches the X server's
  abstract socket and the loopback listeners, a way to the terminal where the person signs; under `pasta` it reaches
  neither. The maestro decided (D-075): a private network with outbound connections only for agents (`pasta`; the
  local network and the Internet stay open, declared); the core's ceiling at 11,000; P8C-8 declared cooperative with
  no crate; nine parts; part 9a on Fable 5.2 and the others on Opus 5.5 (part 9a moved to Opus 5.5 after the session, D-078). Codex (GPT-6.1 Sol, high) reviewed the
  design four times (confirmed blocking 6, 1, 1, 0): the kernel's keyrings, which namespaces do not isolate (a filter
  now refuses the keyring calls, cryptographic sockets and `io_uring`; the rest is declared); a tracked Git hook run
  by Fabrewk's own ref update after integration (Fabrewk's Git now runs no hook); instruction files nested under a
  wildcard lane (`gate.protectedNames`, at any depth); an extension's program missing from its cell; the writable
  places of a cell; an unknown duration after a crash. The maestro approved the twelve amendments (S-1 to S-12, D-075
  and D-076), now in `core-specification.md` under "as phase 9's opening amended it" and in section 12's last table;
  new: REQ-RUN-10, `DISK_LOW`, `limits.freeDiskMegabytes`, `gate.protectedNames`. No ring's code changed; only the
  ceiling in `tools/budget` (its tests green, and the core's tests that read the specification). One point is left
  to part 9a by test: how `bwrap` and `pasta` compose so that ending Fabrewk's direct child ends the cell (killing
  `pasta` does not; killing `bwrap` does); if no composition holds, the choice returns to the maestro. An untracked
  `.vscode/` made by the editor is in no commit.
- **Part 9a closed (session 30, R-039).** The cell's contract and its runner (`crates/core/src/confine.rs`) and the
  bubblewrap backend (`crates/confine-bwrap`, ring `Confinement`): every namespace new, the system read-only, a
  minimal `/dev` and `/etc`, a seccomp filter on the kernel's keys, a private network from `pasta`, the evidence in
  `local/evidence/<run>/` (`0600`, `0700`), `CONFINEMENT_UNAVAILABLE` where no backend exists, a program is absent or
  the probe fails, and `config` listing the residual risks (`fabrewk.config/2`). How `bwrap` and `pasta` compose was
  settled by test (the design's 3.7, candidate (a)), with three consequences the maestro may want to read in R-039:
  no `--dev` (no `/dev/pts`, no `/dev/shm`), a cell with a network that may not nest refused, and an idle `pasta`
  left when Fabrewk is killed during a run, declared as a residual risk. The cell's environment reaches it through a
  file read inside, never through a process of the host (P9A-4). Real cells probed as an agent's, a check's and an
  extension's would be confined; a process in its own session that ignores signals is gone at the limit and when
  Fabrewk is killed. Codex reviewed three times (confirmed blocking 3, 1, 0). Mutation: of 133 mutants the 16 missed were killed, none
  equivalent; the 28 of those corrections were all caught. Clean clone of `8faed54`: 431 tests, `crash` 3 and `faults`
  19 green, musl build, `budget check`, clippy, fmt, `cargo deny` clean. Measured at +335
  lines in the core (projected 250) and 276 in the backend (projected 300). `run` keeps refusing at the command line
  until part 9e. An untracked `.vscode/` made by the editor is in no commit.
- **Part 9b closed (session 31, R-040).** The attempt's copy and the candidate (`crates/core/src/candidate.rs`): the
  export of a commit into `work` from the objects' bytes, never through a link and with no `.git` or `.brewstillery`
  at any depth; the walk of what a cell leaves, following nothing and opening no special file; the change set against
  the base and its guards (`PROTECTED_PATH`, `LANE_VIOLATION`, `PROTECTED_PATH_WILDCARD` for `gate.protectedNames` and
  the hooks directory, `CREDENTIAL_SUSPECTED`), with nothing written into the object store until they pass; a path
  Git refuses to hold is refused, since Git drops it silently; the candidate commit on `fabrewk/<task-id>` by
  compare-and-swap; `DISK_LOW` from `df` (`limits.freeDiskMegabytes`); and S-11: every Git command Fabrewk runs names
  `/dev/null` as its hooks directory and turns the file-system monitor off. `status` shows what `local/evidence/` and
  `local/tmp/` hold (`fabrewk.status/3`). Codex reviewed five times (confirmed blocking 3, 5, 3, 1, 0; P9B-1 to
  P9B-12); three launches were cut short by Codex's provider ("flagged for possible cybersecurity risk"), and at the
  maestro's choice the last two passes were narrow re-checks. Mutation: of 221 mutants the 18 missed were killed or
  removed but one declared equivalent (the check that `cat-file` answers with the identifier asked); every run over
  the corrections caught all or was closed the same way. Clean clone of `ca8bc8a`: 452 tests, `crash` 3 and `faults`
  20 green, musl build, `budget check`, clippy, fmt, `cargo deny` clean. Measured at +636 lines in the core (projected
  400). Three interpretations the maestro may want to read are in R-040 (Git's silent refusal, the hooks directory's
  resolution, `df`'s reading). A test that fails by chance with `LOCK_HELD`, older than this part, waits for the
  maestro's destination (R-040); it once made a missed mutant look caught.
- **Part 9c closed (session 32, R-041, D-079).** The adapter's contract (`crates/core/src/adapter.rs`): each role's
  cell, the packet of REQ-RUN-8 (exactly its listed parts, as JSON), each role's result schema and its validation
  (`REVIEW_INVALID`), the usage of S-5, the preflight (`CONFINEMENT_UNAVAILABLE`), a release that may not hold the
  home or the repository. Claude Code's module (`crates/adapter-claude`): its launch per role with its discovery off
  (`--safe-mode`, `--restricted`) and its own sandbox required with no network for the model's commands, its
  credential bound read-only, its preflight, its marker, protected names and residual risks (T-05, T-13, T-22 in
  `config`), and the one grammar of its stream with the audit of a reviewer's examinations (PD-25, PD-26's twenty
  sub-cases at every position, PD-27). With the maestro's permission the real tool ran four dispatches (Sonnet 5.5,
  low, about 0.13 USD at list price): three streams recorded and pinned, the tool's sandbox probed from inside, the
  person's credential unchanged. Codex reviewed six times (confirmed blocking 3, 2, 1, 1, 1, 0; P9C-1 to P9C-9);
  after two retries without progress the maestro chose a structural rule for the audit (a command examines only when
  its answer and the run's own record show it completed and printed) and a narrow last pass. The credential's
  renewal inside a cell is closed by refusing a dispatch whose token expires within the attempt (D-079, built in
  part 9e). Mutation: 155 mutants, the 3 missed killed, none equivalent. Clean clone of `c64b9c1`: 498 tests, `crash`
  3 and `faults` 20 green, musl build, `budget check`, clippy, fmt, `cargo deny` clean. Measured: the core +275
  (projected 150), the adapter 295 (projected 500). Seven interpretations the maestro may want to read are in R-041.
  The chance failure with `LOCK_HELD` of R-040 is closed: a session releases its lock on the description a forked
  child shares (D-080).
- **Part 9d closed (session 33, R-042, D-081).** Codex's module (`crates/adapter-codex`): its release per version
  (`releases/<version>/` with its own `bwrap` and `rg`), its launch per role (`exec` with `--strict-config`, so that a
  setting a version renamed stops it; a permission profile per role, `:workspace` or `:read-only`, with no network and
  the credential's folder unreadable; approvals, `AGENTS.md`, skills, rules, hooks, plugins, the web, updates,
  telemetry and the unified execution off, each shown by `codex debug prompt-input` or a real run), its preflight (its
  own sandbox under each role's profile, which Codex starts without a model and which fails closed), its markers moved
  from the binary (plus `CODEX_THREAD_ID`), its protected names, and the one grammar of its stream with the audit of a
  reviewer's commands. The contract gained each adapter's machine rule (a launch directory holding `.git` anywhere
  refuses, `CONFINEMENT_UNAVAILABLE`, before any process: 6.38), the role's schema written in the cell, every property
  of a role's schema required (a finding's `path` a text or `null`, as OpenAI's strict output demands), and
  `adapter::json`, which refuses a JSON member named twice. With the maestro's permission the real Codex ran four
  dispatches (`gpt-6-luna`, low; the first refused by the provider for its schema): four streams recorded and kept
  whole, the sandbox probed from inside, the person's credential unchanged. A renewal of its login inside a cell is
  closed by refusing in part 9e (D-081), as D-079 did for Claude Code. Codex reviewed twice (confirmed blocking 1, 0):
  P9D-1, a member named twice hiding an anomaly, also in Claude Code's grammar of 9c, corrected in both. Mutation: 169
  mutants, the 16 missed tested or removed but one declared equivalent. Clean clone of `e2e3465`: 542 tests, `crash` 3
  and `faults` 20 green, musl build, `budget check`, clippy, fmt, `cargo deny` clean. Measured: the core +109
  (projected 30), Codex's adapter 313 (projected 500). Six interpretations the maestro may want to read are in R-042.
- **Part 9e closed (session 34, R-043, D-082 to D-084).** The run, building side (`crates/core/src/cli/run.rs`):
  `run-started`'s guards in the table's order, a halting code halting the task from `authorized` (exit 4); REQ-RUN-2
  with S-6's required tools under the cell's system directories; the budget per authorization and the stage's as a
  cap (D-082); the packet bounded; the preflight's pass kept as verified by use; the credential refused when it
  expires or would be renewed within the attempt (D-079, D-081); then, under the lock the run holds to its end, the
  attempt's copy, the builder's dispatch, its end, the change set and the candidate on `fabrewk/<task>`, and until
  part 9f a halt `CONFINEMENT_UNAVAILABLE` on it (D-082). Every write command first recovers a task left `running`,
  before the lock and under it (REQ-RUN-5): its attempt ended `unknown`, the task halted `OUTCOME_UNKNOWN`, its copy
  removed. The replay binds a run to the authorization in force and each end to its attempt (`ATTEMPT_REPLACED`);
  the five payloads are fixed; `BASE_MOVED` reads the last integration from the branch's history; `status <id>` shows
  a halt (`fabrewk.subject/3`); the view shows S-5's usage. Codex reviewed five times, the fifth narrow at the
  maestro's choice after two retries without progress (D-084; two launches cut by its provider): confirmed blocking 4,
  2, 2, 2, 0, P9E-1 to P9E-14 corrected; keeping Git's own output as evidence for every command went to part 9g
  (D-083). Mutation: 335 mutants, the 4 missed removed or tested, none equivalent. Clean clone of `c495157`: 570
  tests, `crash` 3 and `faults` 23 green, musl build, `budget check`, clippy, fmt, `cargo deny` clean. Measured: the
  core +766 (projected 600); 1,249 lines remain for 9f to 9i against about 1,450 projected. Eight interpretations the
  maestro may want to read are in R-043. A view's temporary file left by a stopped write (part 8c's) waits for part
  9i's crash suite. No credential of the person's was read.
- **Part 9f closed (session 35, R-044, D-085).** Checks, review and verification (`crates/core/src/gate.rs`, new, and
  the rounds in `cli/run.rs`): a run goes on past its candidate, round after round under its lock: the round limit
  (`ROUNDS_EXHAUSTED`), the builder on the task's last candidate (D-085), each required check confined on a fresh
  export (REQ-GATE-1, REQ-GATE-2; no network, its own minutes outside the budget; a check with network halts
  `CONFINEMENT_UNAVAILABLE`, D-085), the reviewer on another (REQ-GATE-4, `REVIEW_UNEXAMINED`,
  `review.differentProvider` and `PROVIDER_MISSING`), `NO_PROGRESS` at a round's end, and `task-verified` on an
  examined accept (exit 0). A check result's identity (REQ-GATE-3), whether a verification holds (REQ-GATE-5), and
  revalidation (REQ-RUN-9): a verified task whose evidence is current is `MOVE_NOT_ALLOWED`, a stale one is checked
  and reviewed again on its candidate. Three payloads (`check-recorded`, `review-recorded`, `task-verified`); a
  recovery removes the stopped run's checks' directories. Codex reviewed twice (confirmed blocking 2, 0): P9F-1, an
  `authorize --run` taking a layer's key as the authorization's naming, and P9F-2, a stage's check the policy later
  defined otherwise, corrected. Mutation: 248 mutants, every viable one caught, tested or removed, none equivalent. Clean
  clone of `5428a62`: 584 tests, `crash` 3 and `faults` 23 green, musl build, `budget check`, clippy, fmt, `cargo
  deny` clean. Measured: the core +568 (projected 550); 681 lines remain for 9g to 9i against about 900 projected,
  so the maestro may want to measure the ceiling again before part 9g (`AGENTS.md` "Code"). Eight interpretations
  the maestro may want to read are in R-044. The machine ran out of memory during the mutation reruns (the scope's
  3 GB cap above the 2 GB the editor leaves free) and was restarted; nothing was lost. No credential of the person's
  was read.
- **Part 9g delivered (session 36, R-045, D-086 to D-090; closed in session 37, below).** Acceptance and integration (`crates/core/src/merge.rs` and
  `crates/core/src/cli/accept.rs`, new): `status <id>` shows what an acceptance binds with its digest
  (`fabrewk.subject/4`), which `accept` takes with `--seen` (REQ-GATE-7, PD-37); the preconditions at three points,
  before the signature (refusing, D-087), just before the move and with the branch at the candidate (`merge-failed`,
  exit 4, waiting for a maestro); the third guard on the whole trees, the touched paths clean in the index and the
  working tree, an integration commit with a `Fabrewk-Candidate` trailer or none when the branch is already at the
  candidate, the move by compare-and-swap, the working tree and the index brought from the objects' bytes with no
  filter (S-11); `run` finishes an accepted task (D-087 point 2); `reject` says where the record branch is
  (`fabrewk.rejected/2`); a verification is bound to the policy its review was recorded under (D-086 point 4). Git's
  standard error and exit status are kept by write commands in `local/evidence/git-*` and named in the words; the
  evidence travels in the error's text, and what a write command went on from is named in `recovered` and in a
  refusal that follows (D-083, D-086, D-089). The core's ceiling is 12,000 (D-086). Codex reviewed nine times, the
  last two narrow (D-088 to D-090; one launch cut by its provider): confirmed blocking 4, 2, 2, 1, 3, 2, 1, 4, 3;
  P9G-1 to P9G-21 corrected, P9G-22 declared (D-090). Mutation: 432 mutants, every viable one caught, tested or
  removed, one equivalent, two loops caught by the timeout. Clean clone of `7ffb3a2`: 611 tests, `crash` 3 and `faults`
  37 green, musl build, `budget check`, clippy, fmt, `cargo deny` clean. Measured: the core +957 (projected 550); 724
  lines remain for 9h and 9i against about 350 projected. Nine interpretations the maestro may want to read are in
  R-045; among them, `check` reports an `own-repository` domain's own `.git` as `ALIAS_INVALID`, older than this part.
  No credential of the person's was read.
- **Part 9g closed (session 37, R-046, D-091 to D-093).** `check` counts the domain's own `.git` in `own-repository`
  (REQ-DOM-2 amended, D-091): `layout` in `crates/core/src/domain.rs` takes the publication mode in force, and `.git`
  is part of the domain only as a directory and only in that mode; tests in the four modes, as a file and as a link,
  `check` in both modes of the fast-forward test, and with an `own-repository` record that does not replay (P9G-23).
  Codex, one narrow pass: accept, P9G-23 not blocking, covered. Mutation: 7 mutants, 5 caught, 2 unviable. Clean clone
  of `77a9071`: 613 tests, `crash` 3 and `faults` 37 green, musl build, `budget check`, clippy, fmt, `cargo deny`
  clean; the core +2, 722 lines remain for 9h and 9i. `verify-part` amended for this machine with the text the
  maestro approved (D-093): every check in a scope of 2 GB, the mutation run's parameters, leftovers looked for before
  starting, and the watcher `target/watch-pasta.sh` until D-092 (part 9i) is built. No credential of the person's was
  read.
- **Part 9h closed (session 38, R-047, D-094).** Extensions (`crates/core/src/cli/ext.rs`, new): `fabrewk <name> …`,
  for a name no core command has, runs `fabrewk-<name>` from `PATH` when `[extensions.<name>]` permits it, every word
  after the name its own; absent, not permitted or outside the alias pattern, a usage error; a pinned digest that
  differs, `EXTENSION_DIGEST`; with network, `CONFINEMENT_UNAVAILABLE` (D-094). Its cell: the repository read-only,
  `local/` hidden but for its area `local/ext/<name>/` (writable, its home and `/tmp` inside, its lock beside it,
  `LOCK_HELD` for a second run) and `local/cache/`, `fabrewk` and a copy of the digested program at their paths, no
  network, the four variables of REQ-EXT-2, `extensions.<name>.minutes` (new key, 30). Exit 10 exits 10; 126 and 127
  `TOOL_FAILED` saying an interpreter may lie outside the cell (S-9); any other but 0 `TOOL_FAILED`. Its standard
  output's `note` and `propose` requests are applied all or none (`EXTENSION_OUTPUT_INVALID`), on the record as it is
  once it ends, with the extension as actor and no signature; its standard error reaches the person escaped; `config`
  lists each permitted extension's path and digest (`fabrewk.config/3`). Codex reviewed three times (confirmed
  blocking 2, 1, 0): P9H-1 to P9H-3 corrected. Mutation: 112 mutants, 82 caught, 30 unviable, none missed. Clean clone
  of `e9cf866`: 626 tests, `crash` 3 and `faults` 39 green, musl build, `budget check`, clippy, fmt, `cargo deny`
  clean. Measured: the core +329 (projected 300); 393 lines remain for part 9i. Eleven interpretations the maestro may
  want to read are in R-047. No credential of the person's was read.
- **Next: part 9i (plan, "Phase 9").** The whole: the flows F-1 to F-3 with fake tools, the crash suite over F-1, one
  real dispatch per provider with its preflight first, the idle `pasta` a killed Fabrewk leaves ended by the recovery
  (D-092), the phase's close. Inputs: the plan's row 9i; 11.5; plan row 9; every part; D-092; R-047 (a `bwrap` left
  by a mutated binary, for the crash suite to look at). Acceptance: the flows and the crash suite; the real dispatches'
  receipts; every criterion section 13 assigns to phase 9; D-071's comparison reported. The permission and the budget
  to run the real tools are asked before the real dispatches (D-079, D-081).
- **Start prompt:** `Eres el integrador de Fabrewk; fase 9, parte 9i: el conjunto (los flujos F-1 a F-3, la suite de
  fallas, un despacho real por proveedor, el pasta ocioso, el cierre de la fase); empieza por
  .brewstillery/STATUS.md`. Model and effort: Claude Code on Opus 5.5 (`claude-opus-5-5`), effort high; Codex GPT-6.1
  Sol, effort high, reviews. Read the skills `verify-part` and `codex-review` before the checks and the review, and
  `differential-review` before a pass. The run's and the extensions' tests (`crates/fabrewk/tests/run.rs`,
  `extensions.rs`) and the backend's and the adapters' launch tests run real cells and need `bwrap`, `pasta`, `socat`,
  `python3` and the network; Codex's sandbox cannot run them nor bind sockets, which is no finding. This machine's
  memory, the mutation run's parameters and the watcher of idle `pasta` are in `verify-part` (D-093); a full suite
  leaves about two idle `pasta`, ended by hand until D-092 is built. When waiting for Codex, wait on the launching
  command itself. Codex's prompt asks it not to read `.brewstillery/skills/` and to report each finding as an input,
  an expected result and an actual one.

Carried without blocking, with destinations, in D-013. A phase that does not pass is corrected and retried; it
never ends the project (D-003).
