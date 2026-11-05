# Fabrewk construction plan

Approved by the maintainer XEL in session 1's chat ("No veo objeciones a tu propuesta de fases."), decision D-010.
Two amendments made after that approval were confirmed by the maintainer (D-012): phase 1 (audit of the
maintainer's alambique, requested in the alambique's section 8, answer 4; its proposed text is delivered as a separate
file in `.brewstillery/bootstrap/`) and phase 2 (the bootstrap review merged with the audit's review, one dispatch
instead of two). Phase 0 is closed (D-013). Phase 1 is closed (D-018). Phase 2 is closed (D-021). Phase 3 was
split into parts (D-023, section "Parts"); Codex's review accepted both (R-010) and it is closed (D-025). Phase 4 is
split into parts 4a and 4b (D-026, section "Parts"); Codex's review accepted both (R-012) and it is closed (D-029). Phase 5 is
not split (D-030): its deliverable is `.brewstillery/bootstrap/architecture-decisions.md`. AD-1 to AD-3 are decided (D-031).
The sweep of input points 6.47.a and 8.2 to 8.4 became D-032 (domain, governance, core architecture) and D-033 (the
alambique's form); a second audit gave the full new alambique and fermento, and the repository migrated to
`.brewstillery/` (R-016). Before phase 5 resumes, the alambique is rewritten concise for the maestro and the
integrator, with Codex checking its fidelity against the archived original (D-034); adopted (D-035). Phase 5 resumed
in session 11: AD-4, AD-5 and AD-7 rewritten and AD-12 to AD-15 added for what D-032 still needed; Codex accepted the
file at its second pass (R-018). The maintainer decided AD-4 to AD-15 as recommended (D-036), and Codex re-checked
the decided file (R-019). Phase 5 is closed (D-037). Phase 6 is split into parts 6a and 6b, and the size budget is frozen at phase 7's start (D-038, section "Parts"). Codex
accepted both parts at its fifth pass (R-021); the maestro approved the text and phase 6 is closed (D-040). Phase 7 is split into parts 7a to 7d (D-041, section "Parts").

**Rule for every phase (alambique 6.48, D-003).** A phase that does not pass is corrected and retried; it never
ends the project. Before each retry the progress measure is fixed (proposed: confirmed blocking findings, and
acceptance criteria met). An exhausted budget pauses the phase with its state saved and asks for more budget or a
smaller scope. Two consecutive retries without progress produce alternatives (reduce scope, change the design,
replace a component) for the maintainer. Acceptance criteria are never changed by an agent to pass.

**Splitting a phase (D-019).** A phase is split into parts (1a, 1b, …) only when it does not fit one session. The
split is decided when the phase starts, where its inputs separate: two parts that need the same large input stay one.
Each part has its own inputs and checkable criterion, ends with a commit and `STATUS.md` updated, and runs in its own
chat, started from `STATUS.md`. Codex reviews at the end of a phase that delivers documents, and at the end of each
part that delivers code. "Part" names a piece of this plan only; how Fabrewk groups work in the projects it serves is
decided in phase 5.

All architecture decisions (phase 5) come before any code. Codex model names come from the legacy receipts and
their availability is not verified.

| # | Phase | Delivers | Depends on | Acceptance (checkable) | Writes / reviews | Model and effort | Maintainer decisions first |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0 | Bootstrap | Repository, entry files, `STATUS.md`, this record, alambique answer, initial commit | — | Maintainer settings unchanged by agents; input above the separator unchanged (digest); TOML parses; `bootstrap` reserved; commit with no remote | Claude / XEL | Opus 5.5 high | Record format; activate the language key; approve the commit |
| 1 | Alambique audit | Findings (leaked TickBrew context, legacy vocabulary, duplicates, broken paths after the legacy move, contradictions) and a proposed clear text that XEL adopts by copying what they accept | 0 | Every finding cites the passage and says why; the proposal loses no expectation of the input (each point traced); no agent writes above the separator | Claude / Codex (fidelity) | Opus 5.5 high | Where the proposed text is delivered |
| 2 | Independent review of the bootstrap and the audit (merged) | Codex findings on the bootstrap files, the alambique answer's fidelity and the audit; corrections | 0, 1 | No confirmed blocking finding open; each finding recorded with its destination | Codex reviews / Claude corrects and records | Codex GPT-6.1 Sol medium; Sonnet 5.5 medium for corrections | Permission to launch Codex, its budget |
| 3 | Full reading of `tickbrew-harness` | Verdict per file (reuse as design, rewrite, discard); the defects found by past reviews turned into test cases | 0 (parallel with 4) | Each file has a verdict and reason; each confirmed past defect has a test case | Claude / Codex | Opus 5.5 high; Codex high | Read permission for the whole repository; optionally running its tests in a disposable clone |
| 4 | Study of references | Source catalogue (seed of 6.30): studied, verified, pending, with pinned revisions. Pullboard at source; catalogues and profiles of section 7b (6.14, 6.15); UCI (6.10); apk-tools; SQLite (6.26); installers that preview their plan | 0 | Each source with revision, date and what was read; each recommendation tied to an alambique point; nothing copied | Claude / Codex | Opus 5.5 high; Codex high | Network permission, list of sources, budget |
| 5 | Architecture decisions | Options, consequences and a recommendation for: single registry and where it lives (6.26, 6.27); settings format and levels (6.10, 6.11, D-007); language classes (6.20, D-008); implementation language; core boundary and size budget; minimal command surface; safe installation with the assessment of D-006; local unversioned area (ignore rules); licence | 3, 4 | The maintainer's words recorded per decision; Codex finds no blocking contradiction with the alambique | Claude / Codex | Opus 5.5 high; Codex high | Every decision of the packet; licence, possibly with a professional |
| 6 | Core specification and criteria | Frozen requirements, states, refusals, check semantics, test plan with the past-defect cases, the size-counting rule and provisional ceilings (the budget is frozen at phase 7's start, D-038) | 5 | Maintainer approves the text; every point classified indispensable maps to a requirement or an explicit deferral | Claude / Codex | Opus 5.5 high; Codex high | Approve the criteria |
| 7 | Core, part 1 | First commit: the skeleton and its line counter, then a maestro freezes the size budget (D-038); registry, transitions, effective configuration with provenance | 6 | Tests green in a clean clone; past defects do not recur; mutations caught; within size budget; clean review | Claude / Codex | Opus 5.5 high; Codex high | Budget |
| 8 | Core, part 2 | Separator checked by code, answers zone, sweep, generated `STATUS.md` | 7 | An ambiguous separator stops with nothing written; `STATUS.md` matches the registry; clean review | Claude / Codex | Sonnet 5.5 high; Codex medium | — |
| 9 | Adapters and confinement | Claude and Codex confined; authorized route when a provider is missing; start-directory check and required tools per task (alambique section 8 of phase 1) | 7 | One real dispatch per provider; a launch outside a repository is refused; clean review | Claude / Codex | Opus 5.5 high; Codex high | Permission to run the real CLIs, budget |
| 10 | Installation | Script and guided prompt reaching the same command; assessment without writing (D-006); administer and review modes; the warning for not publishing `.brewstillery/` | 5, 8, 9 | In a disposable repository nothing is written before confirmation; the three verdicts exercised; warning shown and recorded | Claude / Codex (human texts by Codex) | Opus 5.5 high; Codex high | Final warning text; publishing on GitHub (external action) |
| 11 | Pilot specification | Module, hidden criteria, the six provoked refusals, budget for both arms (D-009) | 6, the new repository | Maintainer approves budget and criteria before starting | Claude / Codex | Opus 5.5 high | Module; new repository and its alambique (written by XEL) |
| 12 | Pilot | Full cycle in the new repository and the comparison arm | 10, 11, the legacy migration | The six provoked cases refused as expected; one real rejection round completed; both arms measured the same way | Fabrewk dispatches, Claude integrates / Codex | Per profiles | Maintainer's acceptance |
| 13 | Bootstrap closing | If the pilot passes: production declared, `bootstrap/` imported into the single registry and made read-only with a check. If not: diagnosis and a new attempt | 12 | Everything imported (the importer respects fenced blocks inside entries); a check fails on any later change in `bootstrap/` | Claude / Codex | Sonnet 5.5 medium; Opus 5.5 high for a diagnosis | Maintainer's production declaration |

## Parts

**Phase 3 (D-023).** Split where its inputs separate. Both parts run on Opus 5.5, effort high; Codex, effort high,
reviews both together when phase 3 ends. Done in session 5 (D-024, R-010): three passes, accept at the third.

| Part | Delivers | Inputs | Acceptance (checkable) | State |
| --- | --- | --- | --- | --- |
| 3a | `harness-verdicts.md`: a verdict and reason per file | `tickbrew-harness` at `0000bdc`; its self-tests once in a disposable clone | Each of the 40 tracked files has a verdict and a reason | Done, session 4 (R-009) |
| 3b | The defects found by past reviews as test cases | The finding keys listed at the end of `harness-verdicts.md`; the review sections of the FZ-2 and FZ-2b receipts and the FZ-2b decision packet in `tickbrew-legacy` | Each confirmed past defect has a test case stated against Fabrewk's behaviour | Done, session 5 (R-010): `past-defect-cases.md` |

**Phase 4 (D-026).** Split where its inputs separate. Both parts run on Opus 5.5, effort high; Codex (GPT-6.1 Sol),
effort high, reviews both together when phase 4 ends. Network permission given at the start of session 6 and
confirmed in session 7 (D-027).

| Part | Delivers | Inputs | Acceptance (checkable) | State |
| --- | --- | --- | --- | --- |
| 4a | `reference-study.md`, sections 1–9: Pullboard at source; catalogues and profiles (6.14, 6.15); permitted versus present (D-015); installers that preview their plan | The sources of alambique section 7b and below, pinned; the four files of the earlier research cited in section 3 | Each source read with revision, date and what was read; each finding tied to an alambique point and a destination; nothing copied | Done, session 6 (R-011) |
| 4b | 7a in `reference-study.md` (UCI 6.10, apk-tools, SQLite 6.26 and 6.27) and the consolidated source catalogue (6.30): studied, verified, pending, with pinned revisions | The 7a sources (pinned in R-011 and re-checked); the earlier research's source list; part 4a | Same as 4a, plus every source of alambique sections 3 and 7 and of the earlier research in the catalogue with its status; then Codex's review of 4a and 4b with no confirmed blocking finding open | Done, session 7 (R-012): three Codex passes, accept at the second, minors closed at the third |

**Phase 6 (D-038).** Split where its inputs separate. Both parts run on Opus 5.5, effort high; Codex (GPT-6.1 Sol),
effort high, reviews both together when phase 6 ends; then the maintainer approves the text.

| Part | Delivers | Inputs | Acceptance (checkable) | State |
| --- | --- | --- | --- | --- |
| 6a | `core-specification.md`: every alambique point classified, each indispensable one mapped to a requirement or an explicit deferral; states and the transition table; commands and refusals; check semantics; settings keys and the items phase 5 left open | `ALAMBIQUE.md`; `architecture-decisions.md` (decided); `decisions.md`; the observations of `harness-verdicts.md` and the phase 6 findings of `reference-study.md` | Every point of section 6 has a class; every indispensable point has a requirement or a deferral with its reason; every command has its moves and refusals; every item AD-1 to AD-15 left to phase 6 is settled or deferred with a reason | Done, session 12 (R-020) |
| 6b | The test plan and the acceptance criteria, in the same file | Part 6a; `past-defect-cases.md` | Every past-defect case maps to a requirement and a test; every requirement has a checkable criterion; then Codex's review of 6a and 6b with no confirmed blocking finding open | Done, session 13 (R-021): five Codex passes, accept at the fifth; approved (D-040) |

**Phase 7 (D-041).** Split where its inputs separate. Every part runs on Opus 5.5, effort high, and delivers code, so
Codex (GPT-6.1 Sol), effort high, reviews each part. The cut between 7b, 7c and 7d was reviewed when 7b started (D-047); 7c was split into 7c1 and 7c2 when it started (D-048).

| Part | Delivers | Inputs | Acceptance (checkable) | State |
| --- | --- | --- | --- | --- |
| 7a | The skeleton (workspace, one crate per ring), the line counter `tools/budget`, the licence files, `cargo-deny`'s configuration; then the maestro's freeze of the size budget | `core-specification.md` REQ-BUD-1 to REQ-BUD-5, 11.2, 11.6; AD-4, AD-5, AD-9 | BUD's criteria of section 12; the counter's fixture tests; tests green in a clean clone and on the musl release build; mutants caught or listed as equivalent; Codex's review with no confirmed blocking finding open | Done, session 15 (R-023): ten Codex passes, accept at the tenth; P7A-1 to P7A-23 corrected; full mutation run with only the two declared equivalents missed |
| 7b | The record as a library in `fabrewk-core`: events, replay, compare-and-swap, fork detection, the history walk, the record commit; the crash suite on the record (D-047) | REG but REG-12 and REG-13; section 3; 11.3 and 11.5 for the record; 7a | REG's criteria at the library's level, but those D-047 moves to 7c and 7d; PD-01 to PD-05, PD-06's place, PD-10 and PD-46 (record); `RECORD_HEAD_MOVED`; the crash suite on the record; as 7a for clean clone, mutants and review | Done, session 16 (R-024): three Codex passes, accept at the third; P7B-1 to P7B-8 corrected; mutants caught but two declared equivalents |
| 7c1 | Settings and policy: every key declared, the project and personal files read with file, key path and line, the personal keys, the policy in force and its pending changes, widenings and `--widen`, the source of every value, language tags, the maintainers' table (D-048) | SET-1 to SET-7, SET-10, ID-1, ID-3's refusal at adoption, DOM-3; 7b | Their criteria and refusals at the library's level; as 7a | Done, session 17 (R-025): seven Codex passes, accept at the seventh; P7C1-1 to P7C1-10 corrected; mutants caught but four declared equivalents |
| 7c2 | Identity and the machine: signers from the cited policy, the role against the kind, the machine key and `instance`, the private folder, the machine file, detection, profiles, the domain's layout on fixture records (D-047, D-048) | REG-4's second half, REG-12, ID-2 to ID-5, SEC-4, SET-8, SET-9, DOM-2, DOM-4, DOM-5, DOM-7; 7b, 7c1 | Their criteria and refusals; as 7a | Planned |
| 7d | The transition table, the command surface, the message catalogue and `check`; `RECORD_TRANSITION_INVALID`, the stage and task payloads and the record's refusal tests through the command line (D-047) | TASK (but TASK-5 and TASK-6), CLI, VIEW-4, SEC-3 at `check`, SEC-5 (D-048); 7b, 7c1, 7c2 | Their criteria; PD-15, PD-17, PD-24, PD-30 to PD-32, PD-34 to PD-38; as 7a | Planned |
