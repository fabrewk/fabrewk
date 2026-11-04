# Study of references (plan phase 4)

Phase 4 deliverable, split into two parts (D-026). This file holds part 4a: the sources of alambique section 7b and
below, read for Pullboard at source (5, 6.30), the catalogues and profiles of agents (6.14, 6.15), how each project
separates what is permitted from what is present on a machine (D-015), and installers that show their plan before
writing (section 1, D-006). Part 4b adds section 7a (UCI, apk-tools, SQLite) and the consolidated source catalogue
(6.30), and Codex reviews both parts together.

How to read it. Each finding says what the source does at the pinned revision, what it means for Fabrewk, and where
it goes (the plan phase that decides it). "Verified" means read in the source at that revision; "stated" means only
the project's own documentation says it; nothing was run. A finding is a design input, not a decision: no
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

| # | What it does (verified unless marked) | For Fabrewk | Destination |
| --- | --- | --- | --- |
| PB-1 | One declaration (`src/machine.js`) holds every state, move, guard and refusal; commands, help and the generated `docs/lifecycle.md` derive from it; every move into `verified` passes the same four guards whatever command gets there; each refusal has a code and a next step. | The strongest model found for 6.28 (transitions controlled by code) and for the legacy lesson that a refusal must name its next step. A declaration that docs are generated from also serves 6.45 (no duplicated documentation). | Phase 6 (states, refusals), phase 7 |
| PB-2 | The criterion, title and cited spec rows freeze at the first claim; submit and verify refuse when they changed; a refreeze is a logged coordinator move. A sign-off keeps the row text it approved and goes stale when the row changes. | Direct answer to 6.25 (requirements an agent cannot move to declare success) and part of 6.29 (a change makes the affected evidence obsolete). | Phase 6, phase 7 |
| PB-3 | A verdict binds the submitted commit and the frozen digest; the verifier's checkout must contain that commit; submit runs the gate itself on the exact tree and never trusts an earlier stamp; the builder never verifies its own work; a rejected commit can never be resubmitted. | Matches 6.29 and 6.43 and several past-defect cases (verdicts on the exact work, no stale evidence counted). | Phase 6 test plan, phase 7 |
| PB-4 | The coordinator's policy (`pullboard.json`) is read from a pinned Git object of the trunk or the claim base, never from the candidate's files (`src/trusted-policy-read.js`: "never from the candidate filesystem"). | The mechanism 6.13 asks for: a change cannot approve its own widening of permissions or weaken the check that verifies it, because the check reads the policy from the base, not from the change. | Phase 5 (settings and authority), phase 6 |
| PB-5 | Settings have one home each: board rules in the repository, machine capacity and tools in `~/.pullboard`, flags override (SPEC O5, O6). | One answer to 6.10/6.11 and D-015: what belongs to the machine is not stored in the project file. | Phase 5 |
| PB-6 | A person's moves (approving rows, answering decisions) are refused in a terminal carrying the environment markers of Claude Code or Codex (`CLAUDECODE`, `CODEX_THREAD_ID`, …). Sign-offs can instead carry the signer's SSH signature over the row text, and a change to the signer list counts only when a key already on it signs that change. | Two kinds of control for 6.12/6.13: the environment check is cooperative (an agent with the same OS user can unset a variable); the signature is effective. Fabrewk should state which of its controls is which (6.13, 6.38). | Phase 5 (roles and authority), phase 11 (provoked refusals) |
| PB-7 | Agents declare their model and, optionally, a model family on joining; `verify.family` can be `off`, `prefer` or `require` a verifier from a family that built none of the work. Both values are self-declared. | A cheap way to encode 6.42 (review preferably by a different provider) as a checked rule, with the limit that the identity is declared, not attested, as in the legacy pilots. | Phase 5, phase 6 |
| PB-8 | Items route by tier (`light`, `mid`, `strong`); an agent claims and verifies its tier and below; `pullboard run` builds with the command the person configured for each tier and escalates a failed attempt one tier up; the core names no model or vendor (SPEC P5). | A simple profile model for 6.15 (task difficulty → agent command) that keeps providers out of the core (6.36). | Phase 5 (profiles) |
| PB-9 | The board is one SQLite file in the Git common directory, shared by every worktree and never committed; submitted work is pinned under `refs/pullboard/items`; an event log is append-only; `pullboard export` writes the whole board out. Sharing across machines goes through an optional hosted relay. | Evidence for 6.26/6.27: SQLite as the single authority works for one machine, but a clean clone does not carry the board; Pullboard needs an export or a hosted service for that. This is the exact tension 6.27 names. | Phase 4b (SQLite), phase 5 (registry) |
| PB-10 | `init` is idempotent and never overwrites a file it did not write, but it writes first and reports after; it edits `.claude/` settings (a `SessionStart` hook), `.codex/config.toml` (an MCP server), `.gitignore`, `AGENTS.md` and `CLAUDE.md`. | Not the model for D-006: Fabrewk assesses and shows its plan before writing anything. Its rule "never overwrite a file it did not write" is worth keeping. | Phase 10 |
| PB-11 | Size: about 84,300 lines of non-test source in `src/` plus 6,900 in `relay/`, and 156,800 lines of tests, built by its own agents (the README reports 490 agents cycled). Its own doctrine already flags eleven files as conflict hotspots. | A measured warning for 6.2 (a small core) and for the legacy lesson that the core grew: the method works, and the code it produces still grows fast. Fabrewk's size budget (phase 5) should be set before building. | Phase 5 (size budget) |
| PB-12 | Stated limits (`docs/architecture.md`, README): worktree identity is not a security boundary; `--no-verify` skips a hook, with the pre-push gate and the verifier as backstops; every agent runs as the same OS user and can read the board key; the relay sees metadata. | Pullboard declares the real limits of its isolation, as 6.38 asks of Fabrewk. | Phase 5 (safe installation), phase 9 |

Not checked in this part, from that checklist: the relay's cryptography, background processes and update behaviour
in detail, and Fedora Asahi ARM64 support (the README says CI runs on Linux and macOS; nothing was run here).

## 3. Catalogues of agents, tools and providers (6.14)

6.14 asks for a configuration that tells tool, provider and model apart, where a tool may take several providers and
a provider several models, and where naming an agent never authorizes running it.

| # | Source | What it does | For Fabrewk |
| --- | --- | --- | --- |
| CA-1 | Omnigent | An agent's `executor` has `harness` (the tool: `claude-sdk`, `codex`, `opencode-native`, …), `auth` (the route to the provider), `model` and `reasoning_effort`; the effort is "validated against the harness's effort vocabulary at launch". | The clearest split of the three names 6.14 asks for, with effort validated before running (6.15). |
| CA-2 | OpenCode (stated) | Per-agent `model` in `provider/model-id` form; a subagent with no model uses the model of the primary agent that invoked it. | Provider and model in one identifier; the silent inheritance is a case 6.35 forbids unless it is shown. |
| CA-3 | GSD Core | One machine-readable model catalogue is the single source for runtimes, tier defaults per runtime, the agent registry, and the profiles `quality`, `balanced`, `budget`, `adaptive` (ADR 0003). Four drifting copies before it made about 15 of 33 agents fall silently to one model. | Evidence for 6.10 ("each datum has one canonical place") applied to the catalogue, and of the cost of not doing it. |
| CA-4 | GSD Core | A per-host matrix records, for each tool, how it exposes commands, dispatch, model choice, hooks and MCP, with a "degradation ladder" when a capability is missing; for Codex it records that model choice is "passive (session-only)". | A model for 6.36: each adapter declares what its tool can do and how Fabrewk degrades when it cannot, instead of assuming parity. |
| CA-5 | Spec Kit, OpenSpec | Registries of about 50 tools (Spec Kit: key, name, folder, whether a CLI is required, install URL; OpenSpec: name, skills directory, detection paths). | Tool catalogues exist mostly to place files for each tool; they do not hold providers, models or permissions. |
| CA-6 | dmux | A registry maps one permission vocabulary (`plan`, `acceptEdits`, `bypassPermissions`) to each tool's flags; when a tool lacks the requested mode, no flag is added and the tool runs with its own default (`getPermissionFlags` returns an empty string). | A sharp edge to avoid: an unsupported permission must refuse, not fall back silently (6.35, 6.38). |
| CA-7 | Agent Orchestrator | The same common vocabulary (`default`, `accept-edits`, `auto`, `bypass-permissions`) maps per tool, and each harness document declares where the mapping loses precision (for one tool, `auto` becomes `accept-edits`). | The correct form of CA-6: a lossy mapping is declared. |
| CA-8 | amicus | One MCP server fronts Codex, Kimi and Claude; a table says which verbs each backend may run (Claude only reviews), and "A `no` is amicus's routing policy, not a model limitation"; a backend outside a tool's list is refused before anything is spent. | Separates what a tool can do from what the project allows it to do (6.14, 6.24), and refuses before spending. |
| CA-9 | RSC-Harness | Agent roles carry a tool set: reviewers `read`, `search`; build resolvers add `edit`, `shell`. `agent-sources.json` records, per agent, the URLs its instructions were researched from and the date. | Role → permitted tools is the core of a profile (6.15); the per-agent provenance file is a small precedent for 6.30. |
| CA-10 | Factory (stated) | A custom droid is a Markdown file with `model` (default `inherit`), `reasoningEffort`, `tools` and `mcpServers`; when a project droid and a personal droid share a name, "the project definition wins". | A project file overriding a personal one is the right direction for 6.11; the question for Fabrewk is the reverse case, which it already forbids (a personal file never widens). |

Recommendation for phase 5: one catalogue with three separate tables (tools, providers, models), each tool declaring
its capabilities and its degradation; profiles referring to catalogue entries; and the rule that a catalogue entry
grants nothing. Every source above keeps "configured" and "authorized to run" apart only by convention, except amicus
and Codex requirements (section 5), which refuse by code.

## 4. Profiles of work (6.15)

| # | Source | What it does | For Fabrewk |
| --- | --- | --- | --- |
| PR-1 | GSD Core | Profiles are views over one agent registry; `quality` equals the "golden" choice per agent and cheaper profiles differ per agent where needed. | A profile as a named view over the catalogue, not a copy of it. |
| PR-2 | Pullboard | Tiers `light`, `mid`, `strong` per item and per agent; the person configures the command for each tier on the machine; escalation is a recorded move. | Profiles tied to the kind of task, with escalation recorded (6.15, 6.41). |
| PR-3 | Agent Deck | Model and effort resolve "per-session > conductor > group (ancestor-walking) > global", recomputed at every start; `group show --resolved` prints the resolved values. | An inheritance order that can be printed, as 6.11 asks ("show the effective configuration"). |
| PR-4 | Codex (stated) | Profiles are named files (`~/.codex/<name>.config.toml`) holding only what differs from the base, selected with `--profile`; the older `[profiles.<name>]` tables stopped working in 0.134.0. | Profiles as layers; and a reminder that a tool's own profile format changes under us (6.36: Fabrewk keeps its profiles in its own format). |
| PR-5 | GSD Core | Codex on a ChatGPT account exposes only the session model; a per-agent model pin made requests fail with a 400 error, and the install check "validates presence, not correctness". | 6.15 "validate the configuration before running" must check that the model is usable on that account, not only that it is named. |
| PR-6 | OpenHarness, GSD Pi | OpenHarness re-reads each engine's config and pane footer every five minutes "to keep model and effort right"; GSD Pi surfaces the "provider-reported child model". | 6.15 "record which profile was really used": the model and effort observed, kept apart from those requested, as the legacy cases already require. |
| PR-7 | codex-plugin-cc | With no `--model` or `--effort`, "Codex chooses its own defaults". | A silent model choice; Fabrewk passes them explicitly or records that the tool chose (6.35). |

## 5. What is permitted, apart from what is present (D-015, 6.11)

D-015 took as a starting idea that the project file says what is permitted, that what exists on each machine is
detected at start and shown in the effective configuration without being stored as another configuration, and that
Fabrewk's defaults sit below the project. Every source studied that addresses the question supports that split; none
contradicts it. What they add:

| # | Source | What it does | What it adds to D-015 |
| --- | --- | --- | --- |
| PP-1 | amicus | `AMICUS_BACKENDS` says which backends are enabled; an enabled backend whose CLI is missing or not logged in "still appears", with `status.installed` and `status.authenticated` saying so; discovery is free and verbs spend. | Present has two parts, installed and authenticated, and both are shown beside "permitted". |
| PP-2 | Agent Deck | `[models] probe = true` asks installed CLIs for their model lists (`codex debug models`, `opencode models`), caches the answer for a day and falls back to a built-in list; "The lists are suggestions, not an allowlist". | Detection is cached and can be stale; a detected list never becomes a permission. |
| PP-3 | Agent Orchestrator | A "Configured" marker "means AO observed credential configuration; it does not prove that the provider accepts it"; for cloud tasks a model list is labelled either "Verified on your Cloud workers" or as the list a pinned CLI version supports. | Each "present" value carries how it was known: observed, verified by use, or only declared by a release. |
| PP-4 | RSC-Harness | `capabilities.js` reads installed skills and agents "from DISK, never from the install state file", because a stale state entry would answer "covered" for something deleted. | Present is read from the machine each time, never from a record of an earlier install. |
| PP-5 | Spec Kit | `init` checks that the chosen tool's CLI exists and stops with its install URL, unless `--ignore-agent-tools`. | A check at start, with the reason and the next step. |
| PP-6 | Pullboard | Repository rules and machine settings live in different files and never duplicate each other. | Confirms the two homes. |
| PP-7 | Claude Code (stated) | Precedence managed > command line > project local > shared project > user; permission lists merge across files; a committed project file's `allow` rules wait until the user trusts the folder, while `deny` and `ask` apply at once; some values (`bypassPermissions`, `auto`) never take effect from a project file; for some security keys the stricter value wins from any level. `/status` lists the files loaded but "doesn't show which file supplied each key". | Concrete rules for 6.11 and 6.13: a project file may restrict at once and may widen only after an explicit trust step; stricter wins. Fabrewk's effective configuration should show per value which file supplied it, which this tool does not. |
| PP-8 | Codex (stated) | Project `.codex/` layers load "only when the project is trusted"; some keys are ignored even then. Requirements (`/etc/codex/requirements.toml` on Linux, or managed) are "admin-enforced constraints that users can't override"; when a user asks for a disallowed value, the client "falls back to a compatible value and notifies the user". "Requirements constrain … Defaults provide starting values." | The same split Fabrewk needs between project policy (constraints) and Fabrewk's defaults (starting values); for Fabrewk the fallback should be a refusal or at least a recorded notice (6.35). |
| PP-9 | Agent Deck | A directory-local `.agent-deck/config.toml` may override only three allowlisted keys, because such a file "can come from" a repository. | A repository-supplied file is not trusted by default; only named keys may come from it. |
| PP-10 | OpenSpec | Generated skills carry `allowed-tools: Bash(openspec:*)`, which "only pre-approves — it does not restrict". | A tool's install can widen an agent's permissions through a generated file; Fabrewk's assessment (D-006) should look for such grants and its own install should not add them silently (6.38). |

Recommendation for phase 5: keep D-015's three levels. Show each effective value with its source (file and key, or
"detected", with how and when). Detection is cached with its date and never written into a settings file. A project
file may restrict at once; any widening needs the responsible maintainer's confirmation, recorded (D-007 b, 6.13).

## 6. Installers that preview their plan (section 1, D-006, 6.38)

| # | Source | What it does | For Fabrewk |
| --- | --- | --- | --- |
| IN-1 | RSC-Harness | `onboard` produces a plan whose id is a hash of its content; the user accepts that id (`--accept-plan <id>`); apply refuses with `RSC_PLAN_CHANGED` if the plan differs; it snapshots the governed paths first, then writes a receipt with the accepted id and the digest of every governed artifact; its doctor compares disk, manifest and receipt and says "Do not trust the receipt" when they disagree. | The closest model to D-006: assessment, a plan the maintainer confirms by its identity, a refusal if it changed, and a receipt that becomes the first record entry. |
| IN-2 | HarnessBrew | Every changing command takes `--dry-run`; `apply-plan <id>` executes a saved plan "without re-planning"; sources must be trusted explicitly (`tap trust`). | The same pattern in a package manager; explicit trust of a source before installing from it (6.38). |
| IN-3 | Spec Kit | Each installed integration records the files it created with their SHA-256; uninstall removes only those still matching and reports the rest. Installing into a non-empty folder asks for confirmation; an extension from an external URL is denied by default when no one can answer the prompt. | Ownership of written files by digest, so Fabrewk can undo its own install without touching a maintainer's edits; default-deny when no human can confirm. |
| IN-4 | Pullboard | Idempotent and "never overwrites a file it did not write", but no preview (PB-10). | Keep the rule, not the order. |
| IN-5 | amicus | A free dry run shows "what a review or delegate call would send" before any model call. | Preview applies to dispatches too: what is sent to a provider (6.38). |
| IN-6 | codex-plugin-cc | `/codex:setup` checks install and login and "can offer to install Codex for you". | An installer that installs another tool must say so and ask (6.38). |

Side effects found that the assessment of D-006 should detect in a repository and that Fabrewk's own install must not
cause without consent: automatic updates on by default (Agent Deck `auto_install = true` and a self-healing update
timer; Pullboard asks once, SPEC O1), knowledge sync that commits and pushes by default (RSC-Harness
`targets/knowledge-sync.mjs`, "On by default", opt-out file `.no-knowledge-sync`; still so at `d9d6bb3`), telemetry
on by default with opt-out (Omnigent; OpenSpec has a telemetry setting), and hooks installed into the agents' own
settings (Pullboard, RSC-Harness, Agent Orchestrator, codex-plugin-cc's optional review gate, which its README warns
"may drain usage limits quickly").

## 7. Other findings tied to alambique points

| # | Source | Finding | Point and destination |
| --- | --- | --- | --- |
| OT-1 | codex-plugin-cc | The code starts Codex with `sandbox: "read-only"` and `approvalPolicy: "never"` by default, but the rescue agent's instructions tell the model to "Default to a write-capable Codex run by adding `--write`". Write access is decided by text the model reads, not by a grant. | 6.24, 6.38: a widening is an explicit authorization, never prose. Phase 9. |
| OT-2 | amicus | `docs/WATCHING.md`: a command compares each dependency with the version the code declares as supported and opens one issue per new minor; it "never installs, validates, widens a cap or changes a supported set"; a failed lookup is `unknown`, which "is never read as `current`" and fails the run. | 6.31, 6.32 almost as written in the alambique; and 6.30 (version used versus latest). Phase 5. |
| OT-3 | amicus | Discloses per backend how prompts travel and where copies stay: Kimi's CLI keeps its own session files that amicus cannot switch off; Codex and Claude run with session persistence disabled. | 6.38: what each agent may send, and what the provider keeps. Phase 9. |
| OT-4 | Omnigent | At `541dd13` the documentation still says `expensive_models: []` "disables the hard limit", while `_resolve_expensive_models` maps `None` or `[]` to "all models blocked". The prior research found the same at `0675793`. | 6.43 (documentation that disagrees with the code); the prior finding is verified again. Catalogue, part 4b. |
| OT-5 | ruvnet/metaharness | Stated: when a host cannot enforce a deny-list, a non-empty deny-list ships a `SANDBOX-REQUIRED.md` "instead of silently dropping your posture". README only; the project's many performance claims were not checked. | 6.36 (declare the limitation when a provider is missing a capability). Phase 9. |
| OT-6 | Pullboard | The person's view lists only "your calls"; questions go one step up (agent → coordinator → person); a coordinator answers or passes a decision up with its note. | 6.18, 6.34: reduce repetitive questions without losing the person's decisions. Phase 5 (minimal command surface). |
| OT-7 | awesome-software-factories | A list for discovery; its own field report says a team that ran a "lights-off" factory for three months abandoned it "because the codebase degraded". | Section 7c; claims to check in their sources before use (6.33). Part 4b catalogue. |

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

## 9. Left for part 4b

Section 7a: UCI (6.10) at `openwrt/uci` `74f6277` (the GitHub mirror and `git.openwrt.org/project/uci.git` have the
same head), apk-tools at `gitlab.alpinelinux.org/alpine/apk-tools` `f419028`, SQLite (6.26, 6.27) at the
`sqlite/sqlite` mirror `38b4005` and its documentation. The consolidated catalogue of every source in sections 3 and
7 (6.30): studied, verified, pending, each with revision, date, licence and what was read, including the 7c case
studies and tools and the 7d neighbours, without repeating evaluations that exist. Then Codex's review of 4a and 4b.
