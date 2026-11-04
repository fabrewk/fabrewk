# Study of references (plan phase 4)

Phase 4 deliverable, split into two parts (D-026). Part 4a, sections 1 to 8: the sources of alambique section 7b and
below, read for Pullboard at source (5, 6.30), the catalogues and profiles of agents (6.14, 6.15), how each project
separates what is permitted from what is present on a machine (D-015), and installers that show their plan before
writing (section 1, D-006). Part 4b, sections 9 to 11: section 7a (UCI, apk-tools, SQLite) and the consolidated
source catalogue (6.30). Codex reviews both parts together.

How to read it. Each finding says what the source does at the pinned revision, what it means for Fabrewk, and where
it goes (the plan phase that decides it). "Verified" means read in the project's code at that revision, and the row
names the file; "stated" means only the project's documentation (README, specification, design record, manual page,
web page) says it. Each finding of sections 2 to 7 and 10 ends with its label. No source was run. A finding is a design input, not a decision: no
reference imposes its architecture, rules or identity (6.33), and nothing was copied into this repository.

## 1. Sources read in part 4a

All read on 2026-11-03 (America/Mexico_City) from shallow clones of the default branch in the session's scratch
directory, read-only, never executed; the clones are not kept. Licences are those GitHub reports for the repository.

| Source | Revision | Licence | What was read |
| --- | --- | --- | --- |
| `pullboard-dev/pullboard` (npm 0.9.3) | `6231e10` | MIT | `README.md`, `SPEC.md`, `DOCTRINE.md`, `docs/method.md`, `docs/lifecycle.md` (generated), `docs/architecture.md` headings and "Honest limits"; `src/trusted-policy*.js`, `src/settings.js` (head), `src/person.js` (head), `src/person-confirmation.js`, `src/init.js` (head and search), `src/run.js` and `src/board.js` (search), `src/config.js` (search), `src/commands/join.js` (search); line counts |
| `open-gsd/gsd-core` | `3730c67` (branch `next`) | MIT | `docs/adr/0003-model-catalog-module.md`, `docs/adr/2313-codex-passive-model-posture.md` (context), `docs/adr/1239-…md` (headings, appendix matrix, amendment of 2026-10-13) |
| `open-gsd/gsd-pi` | `b631874` | MIT | `README.md` (search) |
| `github/spec-kit` | `a227e38` | MIT | `src/specify_cli/integrations/` (list), `zed/__init__.py`, `manifest.py` and `_file_changes.py` (heads), `command_init.py` (agent check, URL trust, search) |
| `Fission-AI/OpenSpec` | `2bd5bc3` | MIT | `src/core/config.ts` (tool list), `src/core/shared/allowed-tools.ts`, `tool-detection.ts` (head), `project-config.ts` and `telemetry/config.ts` (search) |
| `ericrisco/rsc-harness` (npm 4.0.4) | `d9d6bb3` | MIT | `package.json`; `scripts/install-plan.js`; `scripts/detect-repo.js` (head); `scripts/lib/onboarding-apply.js` (search, lines 255–300); `scripts/lib/capabilities.js` (head); `targets/` (list), `agent-catalog.js` and `deepseek-bridge.js` (heads), `agent-sources.json` (head), `knowledge-sync.mjs` (search) |
| `omnigent-ai/omnigent` (0.22.0.dev0) | `541dd13` | Apache-2.0 | `docs/AGENT_YAML_SPEC.md` (lines 1–116, 468–540), `docs/POLICIES.md` (search), `omnigent/policies/builtins/cost.py` (`_resolve_expensive_models`), `omnigent/telemetry/client.py` (search) |
| `asheshgoplani/agent-deck` | `2e897f2` | MIT | `skills/agent-deck/references/config-reference.md` (per-group overrides, `[models]`, directory-local overrides, `[updates]`), `docs/PER-GROUP-CLAUDE-CONFIG-SPEC.md` (head) |
| `OrchestratorInc/agent-orchestrator` | `a1fe965` | Apache-2.0 | `docs/harnesses/model-catalogs.md`, `docs/harnesses/command-code.md` (permission modes) |
| `standardagents/dmux` | `8cb3d92` | MIT | `src/utils/agentLaunch.ts` (registry, permission flags), `src/utils/settingsManager.ts` (levels) |
| `autonomous-ai/openharness` | `beccfd2` | MIT | `docs/architecture.md` (ownership boundaries, daemon) |
| `openai/codex-plugin-cc` | `db52e28` | Apache-2.0 | `README.md`; `plugins/codex/scripts/lib/codex.mjs`, `agents/codex-rescue.md`, `skills/codex-cli-runtime/SKILL.md` (search) |
| `briandconnelly/amicus` (0.11.1) | `8f378d1` | MIT | `README.md`, `docs/WATCHING.md` (head) |
| `ruvnet/metaharness` | `2f415bf` | MIT | `README.md` (head), `package.json` |
| `anzihenry/HarnessBrew` (1.3.0) | `60141ca` | MIT | `README.md`, `README.en.md`, `docs/guides/using.md` (search) |
| `varun1505/awesome-software-factories` | `c39c5ce` | CC0-1.0 | `README.md` (head) |
| Claude Code settings | page read 2026-11-03 | — | `https://code.claude.com/docs/en/settings` |
| Codex configuration | page read 2026-11-03 | — | `https://learn.chatgpt.com/docs/config-file/config-advanced`, `https://learn.chatgpt.com/docs/enterprise/managed-configuration.md` (the `developers.openai.com` URLs redirect there) |
| OpenCode agents | page read 2026-11-03 | — | `https://opencode.ai/docs/agents/` |
| Factory custom droids | page read 2026-11-03 | — | `https://docs.factory.com/cli/configuration/custom-droids` (`factory.ai` and `docs.factory.ai` redirect to `factory.com`) |

Re-read in part 4b, at the same revisions, to settle Codex's review: Pullboard `src/sync.js` (lines 90–165),
`src/commands/export.js`, `src/machine.js` (head and the guard table), `src/board.js` (`headIsNew`), `src/person.js`,
`src/config.js`, `src/commands/join.js` and `src/run.js` (search); GSD Core ADR 2313 (amendment of 2026-09-04); Agent
Deck `config-reference.md` (lines 205–250); HarnessBrew `docs/guides/using.md` and `using.en.md`; amicus
`docs/WATCHING.md` and `README.md` (lines 118–130); RSC-Harness `scripts/lib/onboarding-apply.js`,
`scripts/lib/capabilities.js` and `scripts/rsc.js` (search); OpenSpec `src/core/shared/allowed-tools.ts`;
codex-plugin-cc `scripts/lib/codex.mjs` and `agents/codex-rescue.md` (search).

Pages were read through a summarizing fetch tool, so their findings below are "stated" and carry the page's own
words only where quoted. Pinned but not studied in this part (identity, licence and revision only, for the
catalogue in part 4b): `FoundationAgents/MetaGPT` `11cdf46`, `openclaw/openclaw` `bf10667`, `obra/superpowers`
`22f0b46` (cloned, not read), `storytold/photocraft` `81bd973`, `marketcalls/openalgo` `e2797fd`,
`paul-jewell/BrewStillery` `b9b8672`, `fkukuck/fabro` `ac667f5`, `danielmiessler/Fabric` `8e8511c`,
`gsd-build/get-shit-done` `bdcaab2` and `gsd-build/gsd-2` `33c00aa` (both archived, checked).

## 2. Pullboard at source (alambique 5, 7b)

Pullboard is the reference closest to what Fabrewk wants: a work board, a spec of rows, frozen criteria, a second
agent who verifies the exact commit, and a person whose decisions only they can make. The earlier research had only
the imported chat's claims about it (`CONTEXT_PROYECTO_IA_GITHUB.md`); this part answers most of the checklist that
file set, at `6231e10`.

| # | What it does | For Fabrewk | Destination |
| --- | --- | --- | --- |
| PB-1 | One declaration (`src/machine.js`) holds every state, move, guard and refusal; commands, help and the generated `docs/lifecycle.md` derive from it; every move into `verified` passes the same four guards whatever command gets there; each refusal has a code and a next step. Verified (`src/machine.js`, head; read in part 4b). | The strongest model found for 6.28 (transitions controlled by code) and for the legacy lesson that a refusal must name its next step. A declaration that docs are generated from also serves 6.45 (no duplicated documentation). | Phase 6 (states, refusals), phase 7 |
| PB-2 | The criterion, title and cited spec rows freeze at the first claim; submit and verify refuse when they changed; a refreeze is a logged coordinator move. A sign-off keeps the row text it approved and goes stale when the row changes. The freeze verified (`src/machine.js`); the stale sign-off, stated. | Direct answer to 6.25 (requirements an agent cannot move to declare success) and part of 6.29 (a change makes the affected evidence obsolete). | Phase 6, phase 7 |
| PB-3 | A verdict binds the submitted commit and the frozen digest; the verifier's checkout must contain that commit; submit runs the gate itself on the exact tree and never trusts an earlier stamp; the builder never verifies its own work; a commit rejected under a frozen criterion cannot be submitted again under the same criterion (after a reopen with no verdict, the coordinator refreezes the bar first, a recorded move). Verified (`src/board.js` `headIsNew`, `src/machine.js`; re-read in part 4b). | Matches 6.29 and 6.43 and several past-defect cases (verdicts on the exact work, no stale evidence counted). | Phase 6 test plan, phase 7 |
| PB-4 | The coordinator's policy (`pullboard.json`) is read from a pinned Git object of the trunk or the claim base, never from the candidate's files (`src/trusted-policy-read.js`: "never from the candidate filesystem"). Verified (`src/trusted-policy-read.js`). | The mechanism 6.13 asks for: a change cannot approve its own widening of permissions or weaken the check that verifies it, because the check reads the policy from the base, not from the change. | Phase 5 (settings and authority), phase 6 |
| PB-5 | Settings have one home each: board rules in the repository, machine capacity and tools in `~/.pullboard`, flags override (SPEC O5, O6). Stated (SPEC). | One answer to 6.10/6.11 and D-015: what belongs to the machine is not stored in the project file. | Phase 5 |
| PB-6 | A person's moves (approving rows, answering decisions) are refused in a terminal carrying the environment markers of Claude Code or Codex (`CLAUDECODE`, `CODEX_THREAD_ID`, …). Sign-offs can instead carry the signer's SSH signature over the row text, and a change to the signer list counts only when a key already on it signs that change. The environment check verified (`src/person.js`); the signatures, stated. | Two kinds of control for 6.12/6.13: the environment check is cooperative (an agent with the same OS user can unset a variable); the signature is effective. Fabrewk should state which of its controls is which (6.13, 6.38). | Phase 5 (roles and authority), phase 11 (provoked refusals) |
| PB-7 | Agents declare their model and, optionally, a model family on joining; `verify.family` can be `off`, `prefer` or `require` a verifier from a family that built none of the work. Both values are self-declared. Verified (`src/commands/join.js`, `src/config.js`, `src/board.js`). | A cheap way to encode 6.42 (review preferably by a different provider) as a checked rule, with the limit that the identity is declared, not attested, as in the legacy pilots. | Phase 5, phase 6 |
| PB-8 | Items route by tier (`light`, `mid`, `strong`); an agent claims and verifies its tier and below; `pullboard run` builds with the command the person configured for each tier and escalates a failed attempt one tier up; the core names no model or vendor (SPEC P5). The escalation verified (`src/run.js`); the tiers and SPEC P5, stated. | A simple profile model for 6.15 (task difficulty → agent command) that keeps providers out of the core (6.36). | Phase 5 (profiles) |
| PB-9 | The board is one SQLite file in the Git common directory, shared by every worktree and never committed; submitted work is pinned under `refs/pullboard/items`; an event log is append-only; `pullboard export` prints the board as JSON. Without an account, `pullboard sync` (SPEC H11) shares one board across machines through the repository's own remote: a dedicated ref, `refs/pullboard/board`, holds a linear log of data-only commits (a baseline snapshot, then one commit per move), published with a compare-and-swap lease; a fresh clone imports the baseline and replays the moves through the same engine; a non-contiguous log is refused, and so is a rewrite that removes a prefix this machine already acknowledged (a fresh clone cannot tell a rewritten history from the original). A hosted relay is the other, optional, route. Sync and export verified (`src/sync.js`, `src/commands/export.js`; read in part 4b); the relay, stated. | Evidence for 6.26 and 6.27: SQLite as the local working copy, and an ordered append-only log in Git as the shared history from which a clean clone rebuilds it. The database is never committed and the log is never edited by hand. | Phase 5 (registry) |
| PB-10 | `init` is idempotent and never overwrites a file it did not write, but it writes first and reports after; it edits `.claude/` settings (a `SessionStart` hook), `.codex/config.toml` (an MCP server), `.gitignore`, `AGENTS.md` and `CLAUDE.md`. Verified (`src/init.js`). | Not the model for D-006: Fabrewk assesses and shows its plan before writing anything. Its rule "never overwrite a file it did not write" is worth keeping. | Phase 10 |
| PB-11 | Size: about 84,300 lines of non-test source in `src/` plus 6,900 in `relay/`, and 156,800 lines of tests, built by its own agents (the README reports 490 agents cycled). Its own doctrine already flags eleven files as conflict hotspots. Line counts measured; the 490 agents, stated. | A measured warning for 6.2 (a small core) and for the legacy lesson that the core grew: the method works, and the code it produces still grows fast. Fabrewk's size budget (phase 5) should be set before building. | Phase 5 (size budget) |
| PB-12 | Stated limits (`docs/architecture.md`, README): worktree identity is not a security boundary; `--no-verify` skips a hook, with the pre-push gate and the verifier as backstops; every agent runs as the same OS user and can read the board key; the relay sees metadata. Stated. | Pullboard declares the real limits of its isolation, as 6.38 asks of Fabrewk. | Phase 5 (safe installation), phase 9 |

Not checked in this part, from that checklist: the relay's cryptography, background processes and update behaviour
in detail, and Fedora Asahi ARM64 support (the README says CI runs on Linux and macOS; nothing was run here).

## 3. Catalogues of agents, tools and providers (6.14)

6.14 asks for a configuration that tells tool, provider and model apart, where a tool may take several providers and
a provider several models, and where naming an agent never authorizes running it.

| # | Source | What it does | For Fabrewk |
| --- | --- | --- | --- |
| CA-1 | Omnigent | An agent's `executor` has `harness` (the tool: `claude-sdk`, `codex`, `opencode-native`, …), `auth` (the route to the provider), `model` and `reasoning_effort`; the effort is "validated against the harness's effort vocabulary at launch". Stated (`AGENT_YAML_SPEC.md`). | The clearest split of the three names 6.14 asks for, with effort validated before running (6.15). |
| CA-2 | OpenCode (stated) | Per-agent `model` in `provider/model-id` form; a subagent with no model uses the model of the primary agent that invoked it. | Provider and model in one identifier; the silent inheritance is a case 6.35 forbids unless it is shown. |
| CA-3 | GSD Core | One machine-readable model catalogue is the single source for runtimes, tier defaults per runtime, the agent registry, and the profiles `quality`, `balanced`, `budget`, `adaptive` (ADR 0003). Four drifting copies before it made about 15 of 33 agents fall silently to one model. Stated (ADR 0003). | Evidence for 6.10 ("each datum has one canonical place") applied to the catalogue, and of the cost of not doing it. |
| CA-4 | GSD Core | A per-host matrix records, for each tool, how it exposes commands, dispatch, model choice, hooks and MCP, with a "degradation ladder" when a capability is missing; for Codex it recorded model choice as "passive (session-only)". ADR 2313's amendment of 2026-09-04 narrows that: model and effort are passed per invocation when Codex's spawn schema advertises them, and the passive posture remains the fallback for older schemas and for the files written at install. Stated (ADRs 1239 and 2313). | A model for 6.36: each adapter declares what its tool can do and how Fabrewk degrades when it cannot, instead of assuming parity. |
| CA-5 | Spec Kit, OpenSpec | Registries of about 50 tools (Spec Kit: key, name, folder, whether a CLI is required, install URL; OpenSpec: name, skills directory, detection paths). Verified (Spec Kit `integrations/`, OpenSpec `src/core/config.ts`). | Tool catalogues exist mostly to place files for each tool; they do not hold providers, models or permissions. |
| CA-6 | dmux | A registry maps one permission vocabulary (`plan`, `acceptEdits`, `bypassPermissions`) to each tool's flags; when a tool lacks the requested mode, no flag is added and the tool runs with its own default (`getPermissionFlags` returns an empty string). Verified (`src/utils/agentLaunch.ts`). | A sharp edge to avoid: an unsupported permission must refuse, not fall back silently (6.35, 6.38). |
| CA-7 | Agent Orchestrator | The same common vocabulary (`default`, `accept-edits`, `auto`, `bypass-permissions`) maps per tool, and each harness document declares where the mapping loses precision (for one tool, `auto` becomes `accept-edits`). Stated. | The correct form of CA-6: a lossy mapping is declared. |
| CA-8 | amicus | One MCP server fronts Codex, Kimi and Claude; a table says which verbs each backend may run (Claude only reviews), and "A `no` is amicus's routing policy, not a model limitation"; a backend outside a tool's list is refused before anything is spent. Stated (README). | Separates what a tool can do from what the project allows it to do (6.14, 6.24), and refuses before spending. |
| CA-9 | RSC-Harness | Agent roles carry a tool set: reviewers `read`, `search`; build resolvers add `edit`, `shell`. `agent-sources.json` records, per agent, the URLs its instructions were researched from and the date. Verified (`targets/agent-catalog.js`, `agent-sources.json`). | Role → permitted tools is the core of a profile (6.15); the per-agent provenance file is a small precedent for 6.30. |
| CA-10 | Factory (stated) | A custom droid is a Markdown file with `model` (default `inherit`), `reasoningEffort`, `tools` and `mcpServers`; when a project droid and a personal droid share a name, "the project definition wins". | A project file overriding a personal one is the right direction for 6.11; the question for Fabrewk is the reverse case, which it already forbids (a personal file never widens). |

Recommendation for phase 5: one catalogue with three separate tables (tools, providers, models), each tool declaring
its capabilities and its degradation; profiles referring to catalogue entries; and the rule that a catalogue entry
grants nothing. In the code read, every source above keeps "configured" and "authorized to run" apart only by
convention. amicus (its README) and Codex requirements (section 5, its documentation) say they refuse what is not
allowed; their code was not read, so that is stated.

## 4. Profiles of work (6.15)

| # | Source | What it does | For Fabrewk |
| --- | --- | --- | --- |
| PR-1 | GSD Core | Profiles are views over one agent registry; `quality` equals the "golden" choice per agent and cheaper profiles differ per agent where needed. Stated (ADR 0003). | A profile as a named view over the catalogue, not a copy of it. |
| PR-2 | Pullboard | Tiers `light`, `mid`, `strong` per item and per agent; the person configures the command for each tier on the machine; escalation is a recorded move. The escalation verified (`src/run.js`); the rest, stated. | Profiles tied to the kind of task, with escalation recorded (6.15, 6.41). |
| PR-3 | Agent Deck | The model resolves per session > conductor > group (nearest ancestor) > no flag, the tool's own default; the global `default_model` only pre-fills the new-session dialog. Effort travels as an environment variable, merged per key from ancestor groups to the group to the conductor. Both are recomputed at every start, and `group show --resolved` prints the resolved values. Stated (`config-reference.md`). | An inheritance order that can be printed, as 6.11 asks ("show the effective configuration"). |
| PR-4 | Codex (stated) | Profiles are named files (`~/.codex/<name>.config.toml`) holding only what differs from the base, selected with `--profile`; the older `[profiles.<name>]` tables stopped working in 0.134.0. | Profiles as layers; and a reminder that a tool's own profile format changes under us (6.36: Fabrewk keeps its profiles in its own format). |
| PR-5 | GSD Core | On a ChatGPT-account Codex, a per-agent pin to a model the account does not expose (an Anthropic alias, or a real Codex model the account lacks) made the spawn fail with a 400 error, while the session model always works; the install check "validates presence, not correctness". Stated (ADR 2313). | 6.15 "validate the configuration before running" must check that the model is usable on that account, not only that it is named. |
| PR-6 | OpenHarness, GSD Pi | OpenHarness re-reads each engine's config and pane footer every five minutes "to keep model and effort right"; GSD Pi surfaces the "provider-reported child model". Stated. | 6.15 "record which profile was really used": the model and effort observed, kept apart from those requested, as the legacy cases already require. |
| PR-7 | codex-plugin-cc | With no `--model` or `--effort`, "Codex chooses its own defaults". Stated (README). | A silent model choice; Fabrewk passes them explicitly or records that the tool chose (6.35). |

## 5. What is permitted, apart from what is present (D-015, 6.11)

D-015 took as a starting idea that the project file says what is permitted, that what exists on each machine is
detected at start and shown in the effective configuration without being stored as another configuration, and that
Fabrewk's defaults sit below the project. Every source studied that addresses the question supports that split; none
contradicts it. What they add:

| # | Source | What it does | What it adds to D-015 |
| --- | --- | --- | --- |
| PP-1 | amicus | `AMICUS_BACKENDS` says which backends are enabled; an enabled backend whose CLI is missing or not logged in "still appears", with `status.installed` and `status.authenticated` saying so; discovery is free and verbs spend. Stated (README). | Present has two parts, installed and authenticated, and both are shown beside "permitted". |
| PP-2 | Agent Deck | `[models] probe = true` asks installed CLIs for their model lists (`codex debug models`, `opencode models`), caches the answer for a day and falls back to a built-in list; "The lists are suggestions, not an allowlist". Stated. | Detection is cached and can be stale; a detected list never becomes a permission. |
| PP-3 | Agent Orchestrator | A "Configured" marker "means AO observed credential configuration; it does not prove that the provider accepts it"; for cloud tasks a model list is labelled either "Verified on your Cloud workers" or as the list a pinned CLI version supports. Stated. | Each "present" value carries how it was known: observed, verified by use, or only declared by a release. |
| PP-4 | RSC-Harness | `capabilities.js` reads installed skills and agents "from DISK, never from the install state file", because a stale state entry would answer "covered" for something deleted. Verified (`scripts/lib/capabilities.js`). | Present is read from the machine each time, never from a record of an earlier install. |
| PP-5 | Spec Kit | `init` checks that the chosen tool's CLI exists and stops with its install URL, unless `--ignore-agent-tools`. Verified (`command_init.py`). | A check at start, with the reason and the next step. |
| PP-6 | Pullboard | Repository rules and machine settings live in different files and never duplicate each other. Stated (SPEC). | Confirms the two homes. |
| PP-7 | Claude Code (stated) | Precedence managed > command line > project local > shared project > user; permission lists merge across files; a committed project file's `allow` rules wait until the user trusts the folder, while `deny` and `ask` apply at once; some values (`bypassPermissions`, `auto`) never take effect from a project file; for some security keys the stricter value wins from any level. `/status` lists the files loaded but "doesn't show which file supplied each key". | Concrete rules for 6.11 and 6.13: a project file may restrict at once and may widen only after an explicit trust step; stricter wins. Fabrewk's effective configuration should show per value which file supplied it, which this tool does not. |
| PP-8 | Codex (stated) | Project `.codex/` layers load "only when the project is trusted"; some keys are ignored even then. Requirements (`/etc/codex/requirements.toml` on Linux, or managed) are "admin-enforced constraints that users can't override"; when a user asks for a disallowed value, the client "falls back to a compatible value and notifies the user". "Requirements constrain … Defaults provide starting values." | The same split Fabrewk needs between project policy (constraints) and Fabrewk's defaults (starting values); for Fabrewk the fallback should be a refusal or at least a recorded notice (6.35). |
| PP-9 | Agent Deck | A directory-local `.agent-deck/config.toml` may override only three allowlisted keys, because such a file "can come from" a repository. Stated. | A repository-supplied file is not trusted by default; only named keys may come from it. |
| PP-10 | OpenSpec | Generated skills carry `allowed-tools: Bash(openspec:*)`, which "only pre-approves — it does not restrict". Verified (`src/core/shared/allowed-tools.ts`). | A tool's install can widen an agent's permissions through a generated file; Fabrewk's assessment (D-006) should look for such grants and its own install should not add them silently (6.38). |

Recommendation for phase 5: keep D-015's three levels. Show each effective value with its source (file and key, or
"detected", with how and when). Detection is cached with its date and never written into a settings file. A project
file may restrict at once; any widening needs the responsible maintainer's confirmation, recorded (D-007 b, 6.13).

## 6. Installers that preview their plan (section 1, D-006, 6.38)

| # | Source | What it does | For Fabrewk |
| --- | --- | --- | --- |
| IN-1 | RSC-Harness | `onboard` produces a plan whose id is a hash of its content; the user accepts that id (`--accept-plan <id>`); apply refuses with `RSC_PLAN_CHANGED` if the plan differs; it snapshots the governed paths first, then writes a receipt with the accepted id and the digest of every governed artifact; its doctor compares disk, manifest and receipt and says "Do not trust the receipt" when they disagree. Verified (`scripts/install-plan.js`, `scripts/rsc.js`, `scripts/lib/onboarding-apply.js`). | The closest model to D-006: assessment, a plan the maintainer confirms by its identity, a refusal if it changed, and a receipt that becomes the first record entry. |
| IN-2 | HarnessBrew | Changing commands take `--dry-run`, except `recover`, `apply-plan`, `apply-maintenance` and `recover-maintenance`, and `maintenance` requires it; `apply-plan <id>` applies a saved plan "without replanning"; sources must be trusted explicitly (`tap trust`). Stated (`docs/guides/using.en.md`, README). | The same pattern in a package manager; explicit trust of a source before installing from it (6.38). |
| IN-3 | Spec Kit | Each installed integration records the files it created with their SHA-256; uninstall removes only those still matching and reports the rest. Installing into a non-empty folder asks for confirmation; an extension from an external URL is denied by default when no one can answer the prompt. Verified (`integrations/manifest.py`, `_file_changes.py`, `command_init.py`). | Ownership of written files by digest, so Fabrewk can undo its own install without touching a maintainer's edits; default-deny when no human can confirm. |
| IN-4 | Pullboard | Idempotent and "never overwrites a file it did not write", but no preview (PB-10). Verified (`src/init.js`). | Keep the rule, not the order. |
| IN-5 | amicus | A free dry run shows "what a review or delegate call would send" before any model call. Stated (README). | Preview applies to dispatches too: what is sent to a provider (6.38). |
| IN-6 | codex-plugin-cc | `/codex:setup` checks install and login and "can offer to install Codex for you". Stated (README). | An installer that installs another tool must say so and ask (6.38). |

Side effects found that the assessment of D-006 should detect in a repository and that Fabrewk's own install must not
cause without consent: automatic updates on by default (Agent Deck `auto_install = true` and a self-healing update
timer; Pullboard asks once, SPEC O1), knowledge sync that commits and pushes by default (RSC-Harness
`targets/knowledge-sync.mjs`, "On by default", opt-out file `.no-knowledge-sync`; still so at `d9d6bb3`), telemetry
on by default with opt-out (Omnigent; OpenSpec has a telemetry setting), and hooks installed into the agents' own
settings (Pullboard, RSC-Harness, Agent Orchestrator, codex-plugin-cc's optional review gate, which its README warns
"may drain usage limits quickly"). The knowledge sync, Omnigent's telemetry client and OpenSpec's telemetry setting
were read in code; the rest is stated.

## 7. Other findings tied to alambique points

| # | Source | Finding | Point and destination |
| --- | --- | --- | --- |
| OT-1 | codex-plugin-cc | The code starts Codex with `sandbox: "read-only"` and `approvalPolicy: "never"` by default, but the rescue agent's instructions tell the model to "Default to a write-capable Codex run by adding `--write`". Write access is decided by text the model reads, not by a grant. Verified (`scripts/lib/codex.mjs`, `agents/codex-rescue.md`). | 6.24, 6.38: a widening is an explicit authorization, never prose. Phase 9. |
| OT-2 | amicus | `docs/WATCHING.md`: one command compares the Kimi, Claude, FastMCP and MCP streams with their shipped baselines and opens one issue per new minor, except Claude, which keeps one rolling issue keyed by full version; Codex is watched by a separate routine that opens a draft support pull request per new minor. The command "never installs, validates, widens a cap or changes a supported set"; a failed lookup is `unknown`, which "is never read as `current`" and fails the run. Stated. | 6.31, 6.32 almost as written in the alambique; and 6.30 (version used versus latest). Phase 5. |
| OT-3 | amicus | Discloses per backend how prompts travel and where copies stay: Kimi's CLI keeps its own session files that amicus cannot switch off; Codex and Claude run with session persistence disabled. Stated (README). | 6.38: what each agent may send, and what the provider keeps. Phase 9. |
| OT-4 | Omnigent | At `541dd13` the documentation still says `expensive_models: []` "disables the hard limit", while `_resolve_expensive_models` maps `None` or `[]` to "all models blocked". The prior research found the same at `0675793`. Verified (code and documentation). | 6.43 (documentation that disagrees with the code); the prior finding is verified again. Catalogue, part 4b. |
| OT-5 | ruvnet/metaharness | Stated: when a host cannot enforce a deny-list, a non-empty deny-list ships a `SANDBOX-REQUIRED.md` "instead of silently dropping your posture". README only; the project's many performance claims were not checked. | 6.36 (declare the limitation when a provider is missing a capability). Phase 9. |
| OT-6 | Pullboard | The person's view lists only "your calls"; questions go one step up (agent → coordinator → person); a coordinator answers or passes a decision up with its note. Stated (SPEC). | 6.18, 6.34: reduce repetitive questions without losing the person's decisions. Phase 5 (minimal command surface). |
| OT-7 | awesome-software-factories | A list for discovery; its own field report says a team that ran a "lights-off" factory for three months abandoned it "because the codebase degraded". Stated. | Section 7c; claims to check in their sources before use (6.33). Part 4b catalogue. |

## 8. Corrections to the earlier research and to section 7

- RSC-Harness moved from 3.5.0 (`34acd5e`, studied 2026-10-27) to 4.0.4 (`d9d6bb3`); its knowledge sync is still on
  by default. Omnigent moved from 0.21.0.dev0 to 0.22.0.dev0; OT-4 still holds.
- `openai/codex-plugin-cc` and `briandconnelly/amicus`, marked "Sin enlace todavía; por comprobar" in section 7b,
  exist at `https://github.com/openai/codex-plugin-cc` and `https://github.com/briandconnelly/amicus`. amicus
  describes itself as replacing "codex-in-claude, moonbridge, or claude-in-codex".
- Factory's site and documentation now answer at `factory.com` and `docs.factory.com`; `factory.ai` redirects there.
- HarnessBrew (7d) is "a Git-native package manager for AI Agent assets" (skills, agents, workflows, prompts,
  instructions and MCP configuration) for OpenAI Codex, at source version 1.3.0; it is a reference for installing
  agent assets with preview and trusted sources, not for resource management in general.
- `gsd-build/get-shit-done` and `gsd-build/gsd-2` are archived (checked), as the earlier research said; their
  successors are `open-gsd/gsd-core` and `open-gsd/gsd-pi`.
- Codex's documentation moved: `developers.openai.com/codex/...` redirects to `learn.chatgpt.com/docs/...`.

## 9. Sources read in part 4b

Read on 2026-11-03 (America/Mexico_City). The three 7a heads were re-checked at 2026-11-04 01:03 UTC and are those
pinned in part 4a (R-011); `git.openwrt.org/project/uci.git` and its GitHub mirror `openwrt/uci` have the same head.
Shallow clones in the session's scratch directory, read-only, never executed, not kept. Run on this machine, outside
any source: a Python 3.14.8 probe against an in-memory SQLite database (version, compile options, module API,
`STRICT` tables), and `bwrap --version` and `git --version`.

| Source | Revision | Licence | What was read |
| --- | --- | --- | --- |
| `openwrt/uci` (6,149 lines of C) | `74f6277` (commit 2026-03-12) | LGPL-2.1, from the file headers; GitHub detects none | `uci.h` (defaults, errors), `cli.c` (usage), `file.c` (tokenizer, `uci_file_commit`, `uci_file_load`), `delta.c` (save directory, delta paths, `uci_save`, `uci_load_delta`), the message of `fb3c234` (override directory), `tests/` (list) |
| `openwrt/procd` (`git.openwrt.org/project/procd.git`) | `fb61168` | LGPL-2.1 for its own sources, with others (`COPYING`) | `service/validate.c` (head), `service/service.c` (search for validation and triggers) |
| `openwrt/openwrt` | `f8e7af4` | GitHub cannot classify its licence file; not read | `package/base-files/files/etc/init.d/boot` (`uci_apply_defaults`) |
| OpenWrt UCI wiki page | — | — | Not read: `openwrt.org` serves its pages only after a JavaScript bot check; it was not circumvented. The code above replaces it. |
| `alpine/apk-tools` (GitLab) | `f419028`, the commit of tag `v3.0.9` (2026-11-02) | GPL-2.0 (`LICENSE`) | `README.md`, `SECURITY.md`; in `doc/`: `apk.8` (description, global options, files), `apk-world.5`, `apk-audit.8`, `apk-protected_paths.5`, `apk-policy.8`, `apk-fix.8`, `apk-keys.5`, `apk-add.8` and `apk-repositories.5` (search); `src/commit.c` lines 420–470, `src/database.c` (search) |
| `sqlite/sqlite` (mirror of SQLite's Fossil repository) | `38b4005` (2026-11-03), version 3.55.0 in development | Public domain; build scripts under BSD-style licences (`LICENSE.md`) | `LICENSE.md`; help of `tool/sqldiff.c` and `tool/dbhash.c`; presence of `tool/sqlite3_rsync.c` |
| SQLite documentation | pages read 2026-11-03, each with its own "last updated" | — | `useovernet.html` (2022-06-22), `howtocorrupt.html` (2026-04-13), `wal.html` (2026-08-25), `sessionintro.html` (2025-05-31), `appfileformat.html` (no date), `index.html` (latest release 3.54.1, 2026-10-16) |

The documentation pages were read through the same summarizing fetch tool as in part 4a, so their findings are
"stated".

## 10. Configuration, modularity and storage (7a)

### UCI (6.10, 6.11, 6.37)

| # | What it does | For Fabrewk | Destination |
| --- | --- | --- | --- |
| UC-1 | One plain-text file per package in one directory (`/etc/config`), three statements (`config <type> ['<name>']`, `option`, `list`), and one address per value, `package.section.option`, used by `get`, `set`, `show` and `delete`. Verified (`cli.c`, `file.c`). | A format a person reads and a tool addresses by a stable path. The address, not the syntax, is what UCI adds over TOML: one way to name every value, which the effective configuration of 6.11 can use to say where each value came from. | Phase 5 (settings format and levels) |
| UC-2 | `set` does not touch the configuration file: it appends a delta line to a save directory (`/tmp/.uci`, created with the mode of `/etc/config`, or 0700). Every later load through UCI applies the saved deltas on top of the file (`uci_file_load` calls `uci_load_delta`), so other UCI readers already see the change. `changes` lists the deltas and `revert` drops them; `commit` locks the file, re-reads the deltas other processes saved, writes a temporary file, syncs it and renames it over the original. Verified (`cli.c`, `delta.c`, `file.c`). | Staging for persistence, not an approval boundary: a pending change can be shown and undone and is written by one serialized writer (6.37), but readers already use it. A widening of permissions that waits for the responsible maintainer's confirmation (D-007, 6.13) needs more: no reader may use it until it is confirmed. | Phase 5, phase 7 |
| UC-3 | The tokenizer ends a line at `#` and `commit` regenerates the file from the parsed model (`uci_export`), so a `uci commit` does not keep comments. Verified (`file.c`), not run. | A tool that rewrites a settings file a person edits loses that person's words. Fabrewk's settings writer, if it has one, must keep comments and order, or never rewrite those files (the rule today: agents do not edit them). | Phase 5 (settings format) |
| UC-4 | Since `fb3c234` (2025-01-17), a file of the same name in an override directory (`/var/run/uci`) is read instead of the one in `/etc/config`, whole, and commits go to it. The commit message: so a configuration manager can generate files at run time "without overwriting existing config files on flash". Verified (`file.c`, commit `fb3c234`). | A layer that replaces whole files, not values. Each value can still be attributed to the one file selected (UCI keeps its path with the package). The limit for D-015 and 6.11 is granularity: to change or restrict one value, a lower level must carry the whole file, which copies every other value (against 6.10, one canonical place per datum). | Phase 5 |
| UC-5 | UCI has no schema. Each service declares to procd its validation rules (package, section type, option, rule), procd keeps them and lists all of them on request. Verified (procd `service/validate.c`, `service/service.c`). | Validation declared next to the values it checks and listable in one place: the shape of 6.15's "validate the configuration before running". | Phase 5, phase 6 |
| UC-6 | At boot, each script in `/etc/uci-defaults` runs; only the ones that exit with success are deleted, then one `uci commit`. A failed script runs again at the next boot. Verified (`etc/init.d/boot`). | A run-once migration that retries until it succeeds (6.27, 6.41). Its only record is the deleted file; Fabrewk would record each migration and its result (6.29). | Phase 5 (registry migrations) |

### apk-tools (D-015, 6.30, 6.31, 6.38, section 1)

Each row says which part was verified in `src/` and which is only stated by apk's manual pages or `SECURITY.md`.

| # | What it does | For Fabrewk | Destination |
| --- | --- | --- | --- |
| AP-1 | `/etc/apk/world` holds only constraints, one per line: a name, an optional version (`=`, `<`, `>`, `~`…), an optional repository tag, or `!name` to forbid. What is installed is a separate database under `lib/apk/db/`; the solver derives the changes from one to the other. A hand edit of `world` takes effect only through `apk fix`. Both paths verified (`database.c`); the format and `apk fix`, stated. | The closest precedent for D-015: what is wanted lives in one file, what is present in another, and neither is mistaken for the other. A prohibition is a first-class line, not the absence of a permission. | Phase 5 |
| AP-2 | Before committing, apk prints the plan only when it is interactive or run with more verbosity: the packages removed and downgraded, and, when there are any of those, when it is interactive or at the highest verbosity, the packages new, upgraded and reinstalled with the download and disk sizes. Interactive mode asks "Do you want to continue [Y/n]?", and `--simulate` opens the database read-only and changes nothing. apk is non-interactive by default, and in `commit.c` an empty line or end of input counts as yes. Verified (`commit.c`). | The preview D-006 asks for, with two sharp edges: the preview is optional, and the confirmation proceeds when nobody answers. Fabrewk always shows its plan before writing and refuses on no answer (default deny, as IN-3). | Phase 10 |
| AP-3 | Trust is a file: a public key in `/etc/apk/keys` makes every package it signs acceptable; `--allow-untrusted` is an explicit flag. `SECURITY.md`: "If a package signing key is trusted, it is approved to modify the system." The two key directories verified (`context.c`); the rest, stated. | The trust model stated in one sentence, as 6.38 asks of Fabrewk: trusting a source is a grant of full authority, and the installer says so. | Phase 5 (safe installation), phase 10 |
| AP-4 | A tagged repository (`@edge`) is never used by default; a package comes from it only when `world` names the tag, and its dependencies only when they need it. `apk policy` lists, per package, the versions each repository offers. Stated. | Following newer sources is opt-in per item (6.31), and the version used is shown beside the versions available (6.30). | Phase 5 |
| AP-5 | Protected paths: a configuration file the administrator changed is not overwritten; the new version is written beside it as `.apk-new`. `apk audit` compares the disk with the database's checksums and lists files added, modified, deleted or with changed metadata. The `.apk-new` suffix verified (`context.c`, `database.c`); the audit, stated. | Never overwrite a maintainer's edit, and find drift by digest (IN-3, PP-4, 6.37). | Phase 10 |
| AP-6 | Configuration by whole files: only the first of `/etc/apk/config` and `/lib/apk/config` is read; in the key and repository directories a file name seen once hides the same name in later directories. The first, verified (`apk.c`); the second, stated. | The same whole-file precedence as UC-4: simple to explain, each value attributable to its file, and the same limit of granularity. | Phase 5 |
| AP-7 | Commit hooks run before and after a commit, and a failing pre-commit hook aborts it; `--no-scripts` or `--no-commit-hooks` skips them. Every command that changes the database is logged to `/var/log/apk.log`. The log path verified (`context.c`); the rest, stated. | A check its operator can switch off by a flag is a cooperative control (6.13): Fabrewk states which of its checks can be bypassed and by whom. | Phase 6 |

### SQLite (6.26, 6.27)

| # | What it does | For Fabrewk | Destination |
| --- | --- | --- | --- |
| SQ-1 | Stated: one file, atomic transactions, concurrent processes on one machine, queryable with common tools. Verified here: Python 3.14.8 ships the `sqlite3` module, linked to SQLite 3.51.2, and accepts `STRICT` tables. | The structured store 6.26 asks to evaluate is available with no new dependency in the language section 1 proposes. | Phase 5 (registry) |
| SQ-2 | Stated: SQLite over a network filesystem risks corruption; WAL mode needs every process on the same host. SQLite recommends a client/server database or a proxy on the database's machine. | The database file cannot be the channel of collaboration between machines (6.27); that has to be Git (text) or a service. | Phase 5 |
| SQ-3 | Stated: copying the file during a transaction, or without its `-wal` or `-journal` file, can corrupt the copy; safe copies are the backup API, `VACUUM INTO` and `sqlite3_rsync`. WAL mode is persistent and keeps `-wal` and `-shm` files while a connection is open. | Committing the database file to Git is unsafe unless it is closed and checkpointed, and Git cannot merge it. A copy for evidence or backup is made by one of the safe routes (6.27 "copias consistentes"). | Phase 5 |
| SQ-4 | Stated: the session extension records the changes to tables with a declared primary key as a changeset that can be applied to another database, inverted, or applied with a conflict handler that omits, aborts or replaces; rows with NULL in the key and virtual tables are not captured; it is off unless compiled in. Verified here: the local library is compiled with it, but Python's `sqlite3` module exposes no session or changeset call. | Branch and machine collaboration through changesets is possible but would need another binding (a dependency, 6.2) and a primary key on every table. | Phase 5 |
| SQ-5 | `sqldiff` prints the SQL that turns one database into another, or writes a changeset. `dbhash` hashes (SHA1) the schema and the content of tables, independent of page layout; it leaves out the content, not the schema entries, of every table whose name starts with `sqlite_` (among them `sqlite_sequence`, which holds the next automatic key) and of virtual tables. Verified (`tool/sqldiff.c` help, `tool/dbhash.c`). | A reviewable difference, and a digest of the content it covers: two databases with the same digest can still differ in state that changes behaviour, such as the next automatic key. Binding evidence to the exact state (6.29) needs that state covered too, or a digest of a canonical text export. | Phase 5, phase 6 |
| SQ-6 | Latest release 3.54.1 (2026-10-16, stated); development tree 3.55.0 at `38b4005`; the library Python uses here, 3.51.2. | A concrete case of 6.30: the version used is not the latest, and both are recorded. | Catalogue (section 11) |

Recommendation for phase 5 (6.26, 6.27), with PB-9: three options to put to the maintainer. (a) The registry's authority
is text files in Git, and SQLite, if used, is a local index rebuilt from them, so a clean clone rebuilds it (6.27) and
it is never the only copy of a record (6.4). (b) SQLite is each machine's working copy, and an append-only log of
moves is shared through Git, as Pullboard's `sync` does (PB-9): a clean clone rebuilds the database by replaying the
log, which needs the same deterministic engine everywhere and a refusal when the log is rewritten. (c) SQLite with
changesets exchanged between machines, which needs a binding and a primary key on every table. Options (a) and (b)
both rebuild from a clean clone: (a) keeps the records themselves as reviewable text with the fewest parts; (b) orders
moves between machines and makes the state depend on replaying them. Phase 5 decides; this study leans to (a) for a
first core, with (b) as the model if concurrent moves from several machines must be ordered.

Recommendation for phase 5 (6.10, 6.11): UCI and apk show two patterns worth taking and one to avoid. Take a stable
address for every value (UC-1), and a pending change shown before it is written (UC-2, AP-2), with a separate
approval step where a widening must not be used before it is confirmed. Take "wanted" and "present" as separate files
that are never mixed (AP-1). Avoid precedence by whole files (UC-4, AP-6) for settings with several levels, because
changing one value at a lower level would copy the whole file. Neither changes the decision to keep TOML for the bootstrap files.

## 11. Consolidated source catalogue (6.30)

Every source of alambique sections 3 and 7, and every external source of the earlier research, with its status:

- **Studied**: its content was read at a pinned revision, or on the date given for a page, and the findings are in
  this file or in the earlier research.
- **Checked**: its identity was confirmed (it exists at that address, with that revision and licence) and its content
  was not studied. This is the plan's "verified" and the alambique's "comprobó"; the name differs so it is not confused
  with the "verified" of a single finding.
- **Pending**: not located, or not readable, with the reason.

Revisions are default-branch heads: part 4a's were pinned at 2026-11-04 00:33 UTC, part 4b's between 01:03 and
01:12 UTC. The relation of every external source to Fabrewk is "design reference": Fabrewk incorporates and adapts no
component yet (6.33). Tools this bootstrap uses to work are marked as such; using one does not make it a dependency.
The earlier research's verdicts (`EVALUATION.md`, `comparison.csv`) are cited, not repeated.

### Section 3: our own earlier work (local)

| # | Source | Status | Revision | What was read, where | Points |
| --- | --- | --- | --- | --- | --- |
| C-01 | `tickbrew-harness` | Studied | `0000bdc` (no licence, no remote) | Every tracked file, phase 3: `harness-verdicts.md` | 6.2, 6.28; phases 5–6 |
| C-02 | `tickbrew-legacy`: `FZ-2.md`, `FZ-2b.md`, `FZ2B_DECISION_PACKET.md` | Studied | tag `legacy-final` (`8a9e1bf`) | The three files whole, phase 3b: `past-defect-cases.md` | 6.43, 6.48; phase 6 |
| C-03 | `tickbrew-legacy`: `docs/development/XEL-DRAFTINGBOARD.md` | Pending | `legacy-final` | Not opened: the read permission (D-023, D-024) names three files only | Phase 11 (pilot module) |
| C-04 | The earlier research, `/home/xel/Dev/TickBrew/meta-harness/` | Studied | Not a Git repository; files of 2026-10-27, sha256 `EVALUATION.md` `9185b472…f6eb`, `comparison.csv` `9341bc14…bf3a`, `SOURCES.md` `3dff14f5…f03a`, `CONTEXT_PROYECTO_IA_GITHUB.md` `556889b3…ad49` | The four files whole, part 4a | 5, 6.30 |
| C-05 | The imported chat "Proyecto IA en GitHub" (`evidence/proyecto-ia-github-transcript.md`) | Studied, by search only | sha256 `44e2078d…3093` | Searched in part 4b for the addresses of the 7c case studies | 7c |

The earlier research's local anchors (`SOURCES.md`, "Local source anchors") point to `/home/xel/Dev/TickBrew/tickbrew`,
now `tickbrew-legacy` (R-004). They are TickBrew's own files, not external sources, and were not read in phase 4.

### Section 7a

| # | Source | Kind | Status | Revision | Licence | What was read, where | Points |
| --- | --- | --- | --- | --- | --- | --- | --- |
| C-06 | UNIX and Linux | Conceptual reference | Studied as a principle; nothing to pin | — | — | Composition and bounded responsibilities, applied in UC-1 to UC-6 and AP-1 to AP-7 | 6.2 |
| C-07 | OpenWrt UCI, `git.openwrt.org/project/uci.git` (mirror `openwrt/uci`) | Library and tool | Studied | `74f6277` | LGPL-2.1 | Section 9; UC-1 to UC-6 | 6.10, 6.11, 6.37 |
| C-08 | OpenWrt procd | Service manager | Studied, in part | `fb61168` | LGPL-2.1 and others | Section 9; UC-5 | 6.15 |
| C-09 | OpenWrt main tree, `openwrt/openwrt` | Distribution | Studied, one file | `f8e7af4` | GitHub cannot classify its licence file; not read | Section 9; UC-6 | 6.27 |
| C-10 | `https://openwrt.org/` and its UCI page | Documentation | Pending | — | — | Bot check that requires JavaScript; not circumvented | 6.10 |
| C-11 | apk-tools, `gitlab.alpinelinux.org/alpine/apk-tools` | Development tool | Studied | `f419028` (= `v3.0.9`) | GPL-2.0 | Section 9; AP-1 to AP-7 | D-015, 6.30, 6.31, 6.38 |
| C-12 | SQLite, `https://www.sqlite.org/` (mirror `sqlite/sqlite`) | Library | Studied | `38b4005` (3.55.0 in development); latest release 3.54.1; used here 3.51.2 | Public domain | Section 9; SQ-1 to SQ-6 | 6.26, 6.27 |

### Section 7b

| # | Source | Kind | Status | Revision | Licence | What was read, where | Points |
| --- | --- | --- | --- | --- | --- | --- | --- |
| C-13 | Pullboard, `pullboard-dev/pullboard` | Meta-harness candidate | Studied | `6231e10` (npm 0.9.3) | MIT | Section 1; PB-1 to PB-12 | 5, 6.13, 6.25, 6.26–6.29 |
| C-14 | `https://pullboard.dev` | Project site | Checked (answers; not read) | — | — | — | 7b |
| C-15 | RSC-Harness, `ericrisco/rsc-harness` | Meta-harness candidate | Studied | `d9d6bb3` (4.0.4); earlier research `34acd5e` (3.5.0) | MIT | Section 1; CA-9, PP-4, IN-1; earlier research | 6.14, 6.15, section 1 |
| C-16 | Agent Deck, `asheshgoplani/agent-deck` | Meta-harness candidate | Studied | `2e897f2` | MIT | Section 1; PR-3, PP-2, PP-9 | 6.11, 6.15 |
| C-17 | Omnigent, `omnigent-ai/omnigent` | Meta-harness candidate | Studied | `541dd13` (0.22.0.dev0); earlier research `0675793` | Apache-2.0 | Section 1; CA-1, OT-4; earlier research | 6.14, 6.43 |
| C-18 | OpenHarness, `autonomous-ai/openharness` | Meta-harness candidate | Studied | `beccfd2`; earlier research `0fe0778` | MIT | Section 1; PR-6; earlier research | 6.15 |
| C-19 | dmux, `standardagents/dmux` | Development tool | Studied | `8cb3d92` | MIT | Section 1; CA-6 | 6.35, 6.38 |
| C-20 | Agent Orchestrator, `OrchestratorInc/agent-orchestrator` | Meta-harness candidate | Studied | `a1fe965` | Apache-2.0 | Section 1; CA-7, PP-3 | 6.14, 6.11 |
| C-21 | OpenSpec, `Fission-AI/OpenSpec` | Specification tool | Studied | `2bd5bc3` | MIT | Section 1; CA-5, PP-10 | 6.14, 6.38 |
| C-22 | Spec Kit, `github/spec-kit` | Specification tool | Studied | `a227e38` | MIT | Section 1; CA-5, PP-5, IN-3 | 6.14, section 1 |
| C-23 | Open GSD Core, `open-gsd/gsd-core` | Meta-harness candidate | Studied | `3730c67` (branch `next`) | MIT | Section 1; CA-3, CA-4, PR-1, PR-5 | 6.10, 6.15, 6.36 |
| C-24 | GSD Pi, `open-gsd/gsd-pi` | Meta-harness candidate | Studied, by search | `b631874` | MIT | Section 1; PR-6 | 6.15 |
| C-25 | `gsd-build/get-shit-done` and `gsd-build/gsd-2` (historical names) | Archived predecessors | Checked | `bdcaab2`, `33c00aa`; both archived | — | Earlier research read their READMEs | 7b |
| C-26 | `ruvnet/metaharness` | Harness generator | Studied, README head | `2f415bf` | MIT | Section 1; OT-5; its performance claims not checked | 6.36 |
| C-27 | MetaGPT, `FoundationAgents/MetaGPT` | Secondary reference | Checked | `11cdf46` (last push 2026-01-21) | MIT | — | 6.35 |
| C-28 | OpenClaw, `openclaw/openclaw` | Secondary reference | Checked | `bf10667` in part 4a; head `754f04f` in part 4b | MIT | — | 7b |
| C-29 | Factory and Droid, `https://factory.com` (`factory.ai` redirects) | Commercial tool | Studied, one page (stated) | Page read 2026-11-03; repository `Factory-AI/factory` checked at `892925c`, no licence detected | Commercial | Section 1; CA-10 | 6.14, 6.15 |
| C-30 | `openai/codex-plugin-cc` | Agent bridge | Studied | `db52e28` | Apache-2.0 | Section 1; PR-7, IN-6, OT-1 | 6.24, 6.38 |
| C-31 | amicus, `briandconnelly/amicus` | Agent bridge | Studied | `8f378d1` (0.11.1) | MIT | Section 1; CA-8, PP-1, IN-5, OT-2, OT-3 | 6.14, 6.31, 6.32, 6.38 |

### Section 7c

| # | Source | Kind | Status | Revision | Licence | What was read, where | Points |
| --- | --- | --- | --- | --- | --- | --- | --- |
| C-32 | OpenAI Harness Engineering, `https://openai.com/index/harness-engineering/` (address from the imported chat) | Case study | Pending | — | — | The page refuses automated reads (HTTP 403); only secondary coverage was found, not used | 6.48 |
| C-33 | Stripe Minions, `https://stripe.dev/blog/minions-stripes-one-shot-end-to-end-coding-agents` | Case study | Checked (answers; not read) | — | — | — | 6.48 |
| C-34 | StrongDM Software Factory, `https://factory.strongdm.ai/` | Case study | Checked (answers; not read) | Repositories `strongdm/attractor` `fb57a55` and `strongdm/leash` `fd8e0c1`, both Apache-2.0, listed by awesome-software-factories as unmaintained (not checked) | — | — | 6.48 |
| C-35 | Gailleur Labs / Domi, `https://www.gailleur.com/ai-first-software-factory` (address from the imported chat) | Case study | Checked (answers; not read) | — | — | A web search found no other source | 6.48 |
| C-36 | CodePrism, `rustic-ai/codeprism` | Case study | Checked | `8115b77` (last push 2026-02-14) | None detected | Its blog posts are marked as sponsored; not read | 6.48 |
| C-37 | awesome-software-factories, `varun1505/awesome-software-factories` | Discovery list | Studied, README head | `c39c5ce` | CC0-1.0 | Section 1; OT-7; searched in part 4b for the case studies | 6.33 |
| C-38 | Diátaxis, `https://diataxis.fr/` | Documentation reference | Checked | Repository `evildmp/diataxis-documentation-framework` `957c09c` | `LICENSE.rst`, not read | — | 6.17, 6.45 |
| C-39 | Write the Docs, `https://www.writethedocs.org/` | Documentation reference | Checked | Repository `writethedocs/www` `f33cfda` | `LICENSE.md`, not read | — | 6.17 |
| C-40 | Mermaid, `mermaid-js/mermaid` | Development tool | Checked | `ea53556` (branch `develop`) | MIT | — | 6.45 |
| C-41 | markdownlint-cli2, `DavidAnson/markdownlint-cli2` | Development tool | Checked | `341e2a3` | MIT | — | 6.43 |
| C-42 | Lychee, `lycheeverse/lychee` | Development tool | Checked | `d119b68` | Apache-2.0 | — | 6.43 |
| C-43 | Vale, `vale-cli/vale` (`errata-ai/vale` redirects there) | Development tool | Checked | `0d9e802` (branch `v3`) | MIT | — | 6.43 |
| C-44 | W3C WAI, `https://www.w3.org/WAI/` | Accessibility reference | Checked | Repository `w3c/wai-website` `321da62` | Use with attribution, per its `LICENSE.md` | — | 6.17 |
| C-45 | Git, `git/git` | Work tool, used in this bootstrap | Checked | `750f5de`; here 2.56.0 | GitHub cannot classify its licence file; not read | — | 6.44 |
| C-46 | GitHub and its CLI, `cli/cli` | Service and work tool, used in this bootstrap | Checked | `eeb44bc`; earlier research read three GitHub documentation pages | MIT | Earlier research (`SOURCES.md`) | 6.13, 6.44 |
| C-47 | VS Code, `microsoft/vscode` | Editor | Checked | `7ea5245`; earlier research read three documentation pages | MIT | Earlier research (`SOURCES.md`) | 6.36 |
| C-48 | Claude Code, `anthropics/claude-code` | Work tool, the integrator of this bootstrap | Studied, one page (stated) | Settings page read 2026-11-03; repository `26382c0` | Proprietary ("All rights reserved", commercial terms) | Section 1; PP-7 | 6.11, 6.13, 6.36 |
| C-49 | OpenAI Codex, `openai/codex` | Work tool, the reviewer of this bootstrap | Studied, pages (stated) | Configuration pages read 2026-11-03; repository `f39e45e`; here 0.166.0; earlier research read three more pages | Apache-2.0 | Section 1; PR-4, PP-8; earlier research | 6.11, 6.15, 6.36 |
| C-50 | OpenCode, `anomalyco/opencode` (`sst/opencode` redirects there) | Work tool, considered (alambique 4) | Studied, one page (stated) | Agents page read 2026-11-03; repository `27dd925` (branch `dev`) | MIT | Section 1; CA-2 | 6.14 |
| C-51 | bubblewrap, `containers/bubblewrap` | Isolation tool | Checked | `1fb0aa2`; here 0.12.0 | LGPL-2.1 (`COPYING`) | Its use in `tickbrew-harness`: `harness-verdicts.md` | 6.38; phase 9 |

### Section 7d

| # | Source | Kind | Status | Revision | Licence | What was read, where | Points |
| --- | --- | --- | --- | --- | --- | --- | --- |
| C-52 | HarnessBrew, `anzihenry/HarnessBrew` | Agent-asset installer | Studied | `60141ca` (1.3.0) | MIT | Section 1; IN-2 | 6.38, section 1 |
| C-53 | BrewStillery, `paul-jewell/BrewStillery` | Name coincidence | Checked | `b9b8672` (last push 2022-07-19) | AGPL-3.0 | — | Section 2 (name) |
| C-54 | Fabrika, `https://tatiyants.com/fabrika-software-factory/` | Name neighbour | Checked (answers; not read) | — | — | — | 1 (name) |
| C-55 | fabriqa.ai, `https://fabriqa.ai/` | Name neighbour | Checked (answers; not read) | — | — | — | 1 (name) |
| C-56 | Fabro, `fkukuck/fabro` | Name neighbour | Checked | `ac667f5` | MIT | — | 1 (name) |
| C-57 | Fabric, `danielmiessler/Fabric` | Name neighbour | Checked | `8e8511c` | MIT | `fabric-ai` is the name of its Homebrew, AUR and Scoop packages (its README); PyPI has no package of that name (checked) | 1 (name) |

### In the earlier research, not in alambique section 7

| # | Source | Kind | Status | Revision | Licence | What was read, where | Points |
| --- | --- | --- | --- | --- | --- | --- | --- |
| C-58 | PhotoCraft, `storytold/photocraft` | Case study (public method) | Studied by the earlier research | `dacecbf` (2026-10-27); head `81bd973` in part 4a, `205db62` in part 4b | Apache-2.0 (and `LICENSE-MIT`, per the earlier research) | Earlier research (`SOURCES.md`, AW-1 prelude) | 6.17 |
| C-59 | OpenAlgo, `marketcalls/openalgo` | Case study lead | Checked | `e2797fd` | AGPL-3.0 | Imported chat only | 6.48 |
| C-60 | Superpowers, `obra/superpowers` | Skill set | Checked | `22f0b46` | MIT | TickBrew declined it in OP-1 (earlier research); no new audit | 6.38 |
| C-61 | sharp-edges skill, `~/.agents/skills/sharp-edges/SKILL.md` | Review method, local | Checked (present; not read) | — | — | Admitted in TickBrew, used by the earlier research | 6.38 |
