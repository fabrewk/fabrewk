# Fabrewk construction plan

Approved by the maintainer XEL in session 1's chat ("No veo objeciones a tu propuesta de fases."), decision D-010.
Two amendments made after that approval were confirmed by the maintainer (D-012): phase 1 (audit of the
maintainer's alambique, requested in the alambique's section 8, answer 4; its proposed text is delivered as a separate
file in `brewstillery/bootstrap/`) and phase 2 (the bootstrap review merged with the audit's review, one dispatch
instead of two). Phase 0 is closed (D-013). Phase 1 is closed (D-018). Phase 2 is closed (D-021). Phase 3 is
split into parts (D-023, section "Parts").

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
| 6 | Core specification and criteria | Frozen requirements, states, refusals, check semantics, test plan with the past-defect cases, size budget | 5 | Maintainer approves the text; every point classified indispensable maps to a requirement or an explicit deferral | Claude / Codex | Opus 5.5 high; Codex high | Approve the criteria |
| 7 | Core, part 1 | Registry, transitions, effective configuration with provenance | 6 | Tests green in a clean clone; past defects do not recur; mutations caught; within size budget; clean review | Claude / Codex | Opus 5.5 high; Codex high | Budget |
| 8 | Core, part 2 | Separator checked by code, answers zone, sweep, generated `STATUS.md` | 7 | An ambiguous separator stops with nothing written; `STATUS.md` matches the registry; clean review | Claude / Codex | Sonnet 5.5 high; Codex medium | — |
| 9 | Adapters and confinement | Claude and Codex confined; authorized route when a provider is missing; start-directory check and required tools per task (alambique section 8) | 7 | One real dispatch per provider; a launch outside a repository is refused; clean review | Claude / Codex | Opus 5.5 high; Codex high | Permission to run the real CLIs, budget |
| 10 | Installation | Script and guided prompt reaching the same command; assessment without writing (D-006); administer and review modes; the warning for not publishing `brewstillery/` | 5, 8, 9 | In a disposable repository nothing is written before confirmation; the three verdicts exercised; warning shown and recorded | Claude / Codex (human texts by Codex) | Opus 5.5 high; Codex high | Final warning text; publishing on GitHub (external action) |
| 11 | Pilot specification | Module, hidden criteria, the six provoked refusals, budget for both arms (D-009) | 6, the new repository | Maintainer approves budget and criteria before starting | Claude / Codex | Opus 5.5 high | Module; new repository and its alambique (written by XEL) |
| 12 | Pilot | Full cycle in the new repository and the comparison arm | 10, 11, the legacy migration | The six provoked cases refused as expected; one real rejection round completed; both arms measured the same way | Fabrewk dispatches, Claude integrates / Codex | Per profiles | Maintainer's acceptance |
| 13 | Bootstrap closing | If the pilot passes: production declared, `bootstrap/` imported into the single registry and made read-only with a check. If not: diagnosis and a new attempt | 12 | Everything imported (the importer respects fenced blocks inside entries); a check fails on any later change in `bootstrap/` | Claude / Codex | Sonnet 5.5 medium; Opus 5.5 high for a diagnosis | Maintainer's production declaration |

## Parts

**Phase 3 (D-023).** Split where its inputs separate. Both parts run on Opus 5.5, effort high; Codex, effort high,
reviews both together when phase 3 ends.

| Part | Delivers | Inputs | Acceptance (checkable) | State |
| --- | --- | --- | --- | --- |
| 3a | `harness-verdicts.md`: a verdict and reason per file | `tickbrew-harness` at `0000bdc`; its self-tests once in a disposable clone | Each of the 40 tracked files has a verdict and a reason | Done, session 4 (R-009) |
| 3b | The defects found by past reviews as test cases | The finding keys listed at the end of `harness-verdicts.md`; the review sections of the FZ-2 and FZ-2b receipts and the FZ-2b decision packet in `tickbrew-legacy` | Each confirmed past defect has a test case stated against Fabrewk's behaviour | Next |
