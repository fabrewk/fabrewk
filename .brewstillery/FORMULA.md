# Fabrewk formula

The formula is the project's roadmap: the phases, subphases and cuts the integrator proposes from the alambique, which
the maestro approves with their signature and the agents follow to the letter (alambique section 2, 6.24). The
alambique says what Fabrewk must be; this file says in what order it is built, by whom, within what bounds, and how
each piece is known to be done. A change to it is the integrator's proposal, approved again by the maestro.

State: drafted by the integrator on 2026-11-22; not approved.

## How it works

- **Approval.** The maestro's signature on this file authorizes every phase in it. Inside a phase, the agents build,
  review, verify and integrate without another signature or command from the maestro. What changes scope, risk,
  resources or providers comes back as an edit of this file or of the alambique, signed again.
- **Cuts.** Each subphase ends at a cut. At a cut the integrator reports in chat what was done, its evidence and its
  measures; the maestro audits and accepts, rejects or asks for changes, in chat or in their fermento. Work past a cut
  waits for that audit.
- **Acceptance.** A piece is accepted by verification against its criteria: its tests in a clean clone (the skill
  `verify-part`), the past-defect cases, and an independent review by an agent of another provider. An agent never
  changes a criterion to make it pass. A cut that does not pass is corrected and retried; it never ends the project.
  Before each retry the progress measure is fixed (confirmed blocking findings and criteria met); two retries without
  progress bring the maestro alternatives (reduce scope, change the design, replace a component).
- **Requirements.** `bootstrap/core-specification.md` stays the reference specification. Where a phase changes a
  requirement, the phase states the new one here with its criterion, and it supersedes the requirement it names.
- **Who.** The integrator is Claude Code (Opus 5.5, effort high). The reviewer is Codex (GPT-6.1 Sol, effort high),
  launched outside Fabrewk (skill `codex-review`) until Fabrewk dispatches its own reviews. Each subphase runs in its
  own chat, started from `STATUS.md`.

## Approving this formula before phase 1 exists

Fabrewk 0.2.0 cannot record a formula's approval. Until subphase 1a builds it, the approval is the maestro's signed
commit of this file, alone, made with their key (D-108's), verifiable with `git verify-commit` against the policy's
allowed signers. Subphase 1a imports that commit into the record as the formula's first approval, with no new
signature.

## Phase 1: the authority model in Fabrewk

The model the alambique now holds (section 2, 6.7, 6.8, 6.12, 6.16, 6.24, 6.53) is built into Fabrewk. Fabrewk cannot
build it with its own tools, since their guards are what changes, so, as 6.53 builds a toolchain, the integrator
builds it by hand in this repository, in commits of its own outside the domain, and each cut is measured as the
pilot measures (the manual arm, D-111 point 2). Its release, 0.3.0, needs an extraordinary authorization: the maestro
signs its tag; the integrator prepares everything else.

### 1a. The formula in the record

1. `FORMULA.md` is part of the domain: `fabrewk check` no longer reports it as an unknown entry. A project has no
   formula until its integrator proposes one; the installation writes none.
2. A formula edit is a proposal, as an alambique edit is, and a maestro approves it with one signed move. The record
   keeps each approved version by its digest.
3. The approval made by the bridge above is imported as the formula's first approval, verified against the maestro's
   key in the policy in force; a commit not signed by that key is refused.
4. A formula changed since its last approval stops every run until it is approved (as `ALAMBIQUE_UNAPPROVED` does);
   `check` reports it.
5. `fabrewk status` and `STATUS.md` show the formula's state, the phase and cut in progress and what waits for the
   maestro; `STATUS.md` names `FORMULA.md` as what an agent reads next.

Criterion: tests for each point, the import tested with a signed commit, an unsigned one and one signed by another
key; `check` clean on this repository with `FORMULA.md` approved.

### 1b. Autonomy inside the approved formula

1. Each phase of the formula declares bounds Fabrewk can check: the paths it may change, its budget (dispatches,
   minutes, tokens where the tool reports them), its providers, its build and review profiles and its checks.
2. Inside those bounds an agent proposes, authorizes, runs and integrates tasks. The record names the agent as the
   actor and cites the formula approval that allows the move; no words and no signature are asked.
3. A move beyond the bounds is refused with a code of its own, naming the bound, and waits for a signed edit.
4. A task is accepted when its required checks pass and an independent review accepts the same candidate; it is
   then integrated with no person's move.
5. The person's moves that remain are approving the alambique and the formula, adopting the policy, and
   extraordinary authorizations (a widening, a release). Each is one short command with its subject, which asks for
   the person's words when they are not given, and is refused inside an agent as today.
6. The base rules (`RULES.md`) say the same: rule 1 names the alambique as the authority of substance and the approved
   formula as what agents follow; rule 7 lists only the moves of point 5.

Supersedes: REQ-TASK-4 and the signed `authorize`, `run`, `accept`, `reject` and `close` of section 5 of the
specification, for work inside an approved formula. Criterion: the whole task cycle in a fixture under an approved
formula with no person's move; each bound exceeded refused; the remaining person's moves still refused inside an
agent.

### 1c. The simplified fermento

1. The fermento is written and read by both: the maintainer's part under a line `MAESTRO:`, the agent's under a line
   `AGENTE:`, as the catalogue renders them in the maintainer's interaction language, each alone on its line and once
   outside code blocks; otherwise the sweep stops without writing (6.7).
2. The header the installation writes is reduced to what a maintainer needs to start.
3. The first sweep of a fermento with the current separator converts it once, the maintainer's input kept byte for
   byte.
4. The answer is plain prose; a proposal is shown with what it changes and what approving it does, with no command
   line full of flags.

Criterion: sweep tests on both forms, the conversion included; this repository's fermento converted with its input
unchanged.

### 1d. Commands for agents and people

1. Every command a person is shown pastes as it is in their declared shell, fish included: no placeholders between
   angle brackets, nothing to copy from another output (the import's frictions, fermento point 1).
2. Each reading command gives agents a compact form with stable fields; its size is measured and reported against
   the current `--json`.
3. Help says, for each person's move, what it does and what the words are for.

Criterion: a test renders every printed command and parses it in sh and fish; the sizes measured.

### 1e. Checks that build Rust, and the profiles

1. A confined check can build and test this workspace with no network, its dependencies available read-only.
2. The project file declares the build and review profiles the phases name; the maestro adopts it (a signed move).

Criterion: `cargo test` of this workspace passes as a confined check on this machine; `fabrewk config` shows both
profiles from the adopted policy.

At 1e's cut, 0.3.0 is released and installed in this repository, as 0.2.0 was.

## Phase 2: the pilot, Fabrewk builds itself

Fabrewk dispatches its own work under this formula (alambique 6.48, 6.53; `bootstrap/plan.md` row 12).

1. The pilot's specification is an edit of this formula, approved before the phase starts: budget, hidden criteria,
   the six provoked refusals and the measures of both arms.
2. Tasks dispatched by Fabrewk, each built and reviewed by its profiles: the three defects of the stage already
   proposed (the import's entry block in the canonical documentation language; a clear refusal for a signing key
   protected by a passphrase; the evidence `ssh-keygen` never keeps), the import's remaining frictions, and
   `run <stage>` as section 5 of the specification states it (D-111 point 5).
3. Accepted as above; the external oracles hold for every accepted task: tests in a clean clone, the past-defect
   cases, Codex launched outside Fabrewk on the integrated commit, and the maestro's audit at each cut.
4. The pilot's report compares both arms by the same measures and names what is not comparable.

## Phase 3: production and the first outside project

If the pilot passes: production declared by the maestro and its release; TickBrew adopted as the first outside
project when its alambique, written by XEL, settles (`bootstrap/plan.md` row 13). If not: a diagnosis, corrections
and a new attempt with its own budget (6.48).
