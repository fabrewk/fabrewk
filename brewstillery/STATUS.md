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

## Waiting

- **Outside Fabrewk:** the maintainer migrates the current TickBrew repository to `tickbrew-legacy` in another
  session. Phases 1 and 3 read `tickbrew-harness`, the two pilots' receipts, the decision packet and the original
  drafting board; they should stay readable after the move, and alambique section 3 will need their new paths.
- **Then phase 1**, the audit of the maintainer's alambique (no network; its proposed text goes to a separate file
  in `brewstillery/bootstrap/`), followed by phase 2, Codex's review of the bootstrap and the audit.

Carried without blocking, with destinations, in D-013. A phase that does not pass is corrected and retried; it
never ends the project (D-003).
