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
- **Phase 3 split into 3a and 3b (D-023).** Part 3a is done (R-009): every file of `tickbrew-harness` has a verdict
  in `brewstillery/bootstrap/harness-verdicts.md` (25 reuse as design, 7 rewrite, 8 discard), its 198 self-tests pass
  in a clean clone, and the reading found three new defects (H3A-01 to 03).
- **Next: part 3b**, the defects found by past reviews turned into test cases; its inputs are listed at the end of
  `harness-verdicts.md` and in `plan.md`, section "Parts". Start prompt:
  `Eres el integrador de Fabrewk; empieza por brewstillery/STATUS.md`. Model and effort (`plan.md`): Claude Code on
  Opus 5.5, effort high. At the start, confirm with the maintainer the read permission of D-023 (`tickbrew-harness`
  and the three named files of `tickbrew-legacy`). When 3b ends, Codex (effort high) reviews phase 3, with the
  maintainer's permission and budget.

Carried without blocking, with destinations, in D-013. A phase that does not pass is corrected and retried; it
never ends the project (D-003).
