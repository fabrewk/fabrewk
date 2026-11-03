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
- **Phase 1 delivered, not closed:** `brewstillery/bootstrap/alambique-audit.md` (D-016): findings, a proposed text
  of the maintainer's input and a traceability table. Five findings marked **Decision** need the maintainer's answer.
- **Next:** the maintainer copies into their input what they accept; the integrator sweeps the alambique, records
  the outcome and closes phase 1. Then phase 2, Codex's review of the bootstrap and the audit, in a new chat; it needs
  the maintainer's permission to launch Codex and its budget.
- **Answers zone:** replaced on every sweep by current recommendations for editing the input (D-017); earlier
  evaluations and where each is resumed are in R-006.
- **Proposed in session 2's chat, not decided:** one level of division only: a phase is split into parts when it
  does not fit one session; each part has its own inputs and checkable criterion, ends with a commit and this file
  updated, and runs in its own chat; split where inputs separate. Recorded and added to `plan.md` only if the
  maintainer accepts.

Carried without blocking, with destinations, in D-013. A phase that does not pass is corrected and retried; it
never ends the project (D-003).
