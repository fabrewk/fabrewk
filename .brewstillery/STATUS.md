# Fabrewk — status

Manual, provisional view, written by the integrator until Fabrewk's single registry exists; then it becomes a
view generated from that registry. Technical files are in English by default (D-008).

## Current state

**Phases 0 to 6 are closed (D-037, D-040). Phase 7 (the first code) is split into parts 7a, 7b, 7c1, 7c2a, 7c2b,
7d1, 7d2, 7s and 7d3 (D-041, D-047, D-048, D-050, D-053, D-059); parts 7a to 7d2 are closed, 7d2 the command
surface and the reading commands (R-029); 7s, skills for the agents, is next, then 7d3, the write commands.** The repository holds the first code at its root: a
Cargo workspace with one crate per ring (`crates/core`, `crates/fabrewk`, the binary, which runs the core's command
surface) and the size counter `tools/budget`, the licence files, `deny.toml` and a root `.gitignore` with `/target`.
The record is built in `crates/core/src/record/`, the settings in `crates/core/src/settings/`, identity in
`identity.rs`, the private folder and the machine file in `machine.rs`, detection and profiles in `detect.rs`, the
domain's layout in `domain.rs`, the transition table in `table.rs`, the payloads and pattern guards in `task.rs`, the
credential scan in `scan.rs`, and the command surface in `crates/core/src/cli/` with its catalogue in
`crates/core/catalogue/` (`en`, `es`); `fabrewk status`, `fabrewk config` and `fabrewk check` exist, and no command
writes yet. The core counts 4,530 of 7,500 lines, formatted at width 120 (D-049, D-054). No `data/` exists in this
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
- **Next: part 7s (plan, "Phase 7").** Search and evaluate skills for Claude Code and Codex (Rust, Git, mutation
  testing, review, the bootstrap's flow), each with its permissions, provenance, version, licence and effects (6.38;
  `reference-study.md` PP-10 on skills that pre-approve tools); propose which to adopt; install only the approved, in
  `.brewstillery/skills/`, with `AGENTS.md` pointing there; reserve the name `skills` in the core's domain layout
  with its test, the exact change approved by the maestro; Codex reviews. Network to read sources of skills only.
- **Then: part 7d3 (plan, "Phase 7").** The write commands: `adopt`, `propose`, `authorize` (PD-37 for `authorize`),
  `reject`, `run` up to confinement (`CONFINEMENT_UNAVAILABLE`), `--say` and `--say-lang`, `AGENT_ENVIRONMENT`, the
  guards every write command applies (REQ-RUN-5, REQ-DOM-7), signing with Git's signing key (D-055 point 1), and the
  record's refusal tests through the command line (D-047); TASK-4, PD-30 to PD-32, PD-37. Open from R-028:
  `run-started`'s words, and how `TASK_CHANGED` arises. The declaration, catalogue, completions and reference grow
  with each command (D-053 point 3); next steps that today are words become commands.
- **Start prompt:** `Eres el integrador de Fabrewk; fase 7, parte 7s: búsqueda, evaluación e instalación de skills
  para los agentes; empieza por .brewstillery/STATUS.md`. Model and effort: Claude Code on Opus 5.5, effort high;
  Codex GPT-6.1 Sol, effort high, reviews the evaluation and the code (plan row 7).

Carried without blocking, with destinations, in D-013. A phase that does not pass is corrected and retried; it
never ends the project (D-003).
