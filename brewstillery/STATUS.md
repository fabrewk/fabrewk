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
- **Phase 3 delivered (D-023, D-024).** Part 3a (R-009): every file of `tickbrew-harness` has a verdict in
  `brewstillery/bootstrap/harness-verdicts.md` (25 reuse as design, 7 rewrite, 8 discard). Part 3b (R-010): every
  confirmed defect found by past reviews, and the three found in 3a, is a test case against Fabrewk's behaviour in
  `brewstillery/bootstrap/past-defect-cases.md` (46 cases). Codex (GPT-6.1 Sol, high) reviewed both: reject, two
  scoped re-checks, accept; every confirmed finding corrected. **The phase's closure waits for the maintainer's word.**
- **Next: phase 4**, the study of references (`plan.md`, row 4), which can start once phase 3 is closed (it does not
  depend on 3). Start prompt: `Eres el integrador de Fabrewk; empieza por brewstillery/STATUS.md`. Model and effort
  (`plan.md`): Claude Code on Opus 5.5, effort high; Codex, effort high, reviews it. Decisions the maintainer gives
  at its start: network permission, the list of sources and the budget; the integrator proposes a split into parts if
  it does not fit one session (D-019). If the maintainer has not closed phase 3 in session 5's chat, the next session
  asks first.

Carried without blocking, with destinations, in D-013. A phase that does not pass is corrected and retried; it
never ends the project (D-003).
