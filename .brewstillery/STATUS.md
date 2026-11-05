# Fabrewk — status

Manual, provisional view, written by the integrator until Fabrewk's single registry exists; then it becomes a
view generated from that registry. Technical files are in English by default (D-008).

## Current state

**Phases 0 to 5 are closed (D-037). Nothing of Fabrewk is built yet.** No code, no licence files (the licence is
decided, D-036), no `.gitignore`, no `data/` yet. The repository holds the project's alambique, the maintainer's
fermento and settings, this file, the root entry files (`AGENTS.md`, `CLAUDE.md`) and the bootstrap record. The domain is `.brewstillery/`, hidden
(D-032).

## Read first

1. `.brewstillery/ALAMBIQUE.md`: the description of what Fabrewk must be, approved by its maestros, with status keys
   defined in its opening. Nobody edits it directly; it authorizes nothing by itself.
2. `.brewstillery/xel/fermento.md`: the maintainer's input (above the separator) and the agents' answers (below it).
3. `.brewstillery/bootstrap/decisions.md`: the decisions in force, with their origin, and what each session read.
4. `.brewstillery/bootstrap/plan.md`: the approved construction plan.
5. `.brewstillery/settings.toml` (project) and `.brewstillery/xel/settings.toml` (personal; never grants authority).

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
  `.gitignore`. The fermento's answer recommends removing points 1 and 2.
- **Next: phase 6, core specification and criteria (plan row 6).** Frozen requirements, states, refusals, check
  semantics, the test plan with the past-defect cases and the size budget frozen after measuring a skeleton (AD-5);
  every alambique point classified indispensable maps to a requirement or an explicit deferral. Phase 6 decides at its
  start whether it splits into parts (D-019).
- **Start prompt:** `Eres el integrador de Fabrewk; empieza por .brewstillery/STATUS.md`. Model and effort: Claude
  Code on Opus 5.5, effort high; Codex GPT-6.1 Sol, effort high, reviews (plan row 6).

Carried without blocking, with destinations, in D-013. A phase that does not pass is corrected and retried; it
never ends the project (D-003).
