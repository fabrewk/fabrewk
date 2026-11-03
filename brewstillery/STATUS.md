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
  `legacy-final`; the files Fabrewk read are unchanged there (R-004). Documentation publication languages: `en`,
  `es-MX` (D-014).
- **Next: phase 1**, the audit of the maintainer's alambique (no network; its proposed text goes to a separate file
  in `brewstillery/bootstrap/`), followed by phase 2, Codex's review of the bootstrap and the audit.

Carried without blocking, with destinations, in D-013. A phase that does not pass is corrected and retried; it
never ends the project (D-003).
