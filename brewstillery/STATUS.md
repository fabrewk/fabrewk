# Fabrewk — status

Manual, provisional view, written by the integrator until Fabrewk's single registry exists; then it becomes a
view generated from that registry. Technical files are in English by default (D-008).

## Current state

**Phase 0 (bootstrap) is closed (D-013). Nothing of Fabrewk is built yet.** No code, no licence, no `.gitignore`,
no `data/`: each is a pending decision. The repository holds the maintainer's alambique and settings, this file,
the root entry files (`AGENTS.md`, `CLAUDE.md`) and the bootstrap record.

## Read first

1. `brewstillery/xel/alambique.md`: the maintainer's input (above the separator) and the agents' answers
   (below it). It is the source of what Fabrewk must be; it authorizes nothing by itself.
2. `brewstillery/bootstrap/decisions.md`: the decisions in force, with their origin, and what each session read.
3. `brewstillery/bootstrap/plan.md`: the approved construction plan.
4. `brewstillery/settings.toml` (project) and `brewstillery/xel/settings.toml` (personal; never grants authority).

## Evidence

`brewstillery/bootstrap/` is the provisional record of Fabrewk's self-construction (D-005). It exists only in this
repository, is imported into the single registry at production and then stays read-only.

## Latest and next

- **Done:** the legacy TickBrew repository now lives at `/home/xel/Dev/TickBrew/tickbrew-legacy`, frozen at tag
  `legacy-final` (R-004). Documentation publication languages: `en`, `es-MX` (D-014).
- **Phase 1 closed (D-018):** the maintainer adopted the audited input (`brewstillery/bootstrap/alambique-audit.md`),
  with a usage note and section 8 as their channel. Phases are split into parts only when needed (D-019, in
  `plan.md`). The answers zone holds only current recommendations (D-017).
- **Phase 2 closed (D-021, R-008):** Codex's review had one blocking point (6.38), settled by the maintainer: it
  stays "propone". Plan sentence on append-only clarified in `AGENTS.md`.
- **Phase 3 closed (D-025).** Part 3a (R-009): every file of `tickbrew-harness` has a verdict in
  `brewstillery/bootstrap/harness-verdicts.md` (25 reuse as design, 7 rewrite, 8 discard). Part 3b (R-010): every
  confirmed defect found by past reviews, and the three found in 3a, is a test case against Fabrewk's behaviour in
  `brewstillery/bootstrap/past-defect-cases.md` (46 cases). Codex (GPT-6.1 Sol, high) reviewed both: reject, two
  scoped re-checks, accept; every confirmed finding corrected.
- **Phase 4 in progress, split into 4a and 4b (D-026).** Part 4a done (R-011): `brewstillery/bootstrap/reference-study.md`
  covers the sources of section 7b and below, pinned: Pullboard at source, catalogues and profiles of agents (6.14,
  6.15), what is permitted apart from what is present (D-015) and installers that preview their plan. Not yet reviewed
  by Codex.
- **Next: part 4b** (`plan.md`, section "Parts"): section 7a (UCI, apk-tools, SQLite) in the same file, the
  consolidated source catalogue (6.30), then Codex's review of 4a and 4b together. Start prompt:
  `Eres el integrador de Fabrewk; empieza por brewstillery/STATUS.md`. Model and effort: Claude Code on Opus 5.5,
  effort high; Codex GPT-6.1 Sol, effort high, reviews. At its start the maintainer confirms network permission and
  Codex's launch with its budget.

Carried without blocking, with destinations, in D-013. A phase that does not pass is corrected and retried; it
never ends the project (D-003).
