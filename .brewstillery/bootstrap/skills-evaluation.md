# Skills for the agents: search and evaluation (part 7s)

Part 7s of the plan (D-059). The alambique asks it in 6.38 (skills, MCP, plugins and other extensions are evaluated
for permissions, provenance, versions and effects before they are incorporated; skills by the project's languages,
tools and approach and to spare repeated work; the maestro decides which are adopted and only those are installed,
in `.brewstillery/skills/`) and section 2 (`.brewstillery/skills/` holds the approved skills, read as `AGENTS.md`
says; a file a tool demands elsewhere is declared and kept minimal). Inputs: `reference-study.md` PP-10 (a generated
skill's `allowed-tools` only pre-approves, it does not restrict) and C-60, C-61. Written by the integrator in session
22; nothing here is installed until the maestro approves it.

## 1. What the project needs

The core is Rust (`unsafe_code = "forbid"`, no FFI, no async, one runtime thread), tested with `proptest` (dev
dependency, choice 11 of the core specification), `cargo mutants` in a capped scope, a crash suite, `clippy`,
`rustfmt`, `cargo deny` and the size counter. Work repeated every part: running the suites with the rustup prefix,
the clean clone, the musl build and `budget check`; the mutation run in its own scope (R-022, R-024); launching Codex
in a disposable clone and recording each pass (R-022 to R-029); closing a session. Review cost is the largest: part
7d2 took ten Codex passes, most on what a reading command may open (links, non-regular files, other domains), the
kind of misuse-resistance a skill on API and configuration footguns targets.

## 2. How the two tools find skills (read 2026-11-06)

- Claude Code (`code.claude.com/docs/en/skills`, no revision date shown): personal `~/.claude/skills/`, project
  `.claude/skills/` from the start directory up to the repository root, nested `.claude/skills/`, `--add-dir`,
  plugins, managed settings, and `~/.claude/skills/synced/`. A skill entry may be a symbolic link to a directory
  elsewhere. No key or flag names another directory. `allowed-tools` lists "tools Claude can use without asking
  permission during the turn that invokes this skill … It does not restrict which tools are available", and
  "workspace trust doesn't gate this field": a project skill's grant applies even in a folder never trusted. A skill
  may carry `hooks`, registered when it is invoked, and `` !`command` `` lines, run before its text reaches the model
  without a permission prompt.
- Codex (`developers.openai.com/codex/skills`, redirected to `learn.chatgpt.com/docs/build-skills`, no revision date
  shown): `.agents/skills/` from the working directory up to the repository root, `~/.agents/skills/`,
  `/etc/codex/skills/`, the bundled system skills; symbolic links to skill folders are followed. No key names
  another directory; `[[skills.config]]` in `~/.codex/config.toml` only enables or disables. Codex reads `name` and
  `description`; `agents/openai.yaml` may set `policy.allow_implicit_invocation`. The page is silent on
  `allowed-tools` and hooks.

Neither tool reads `.brewstillery/skills/` by itself. Two routes follow (section 5).

**Present on this machine (D-015, present is not permitted).** `~/.agents/skills/` holds five Trail of Bits skills
adapted for TickBrew's OP-1 on 2026-10-23 (`differential-review`, `mutation-testing`, `property-based-testing`,
`sharp-edges`, `supply-chain-risk-auditor`; their `MANIFEST.sha256` verifies), linked from `~/.claude/skills/`, so
both tools already load them in every repository, this one included. Each carries a notice that sends TickBrew work to
`docs/automation/AGENT_SKILLS_POLICY.md`, a file of the frozen legacy that this repository does not cite. Their
bodies equal Trail of Bits' head `470419e` except the notice and the removed `allowed-tools` and `effort` keys
(checked file by file). `~/.codex/skills/.system/` holds Codex's bundled skills (`imagegen`, `openai-docs`,
`review-agent`, `skill-creator`, `skill-installer`); `skill-installer` fetches skills from the network into the user's
folder. These are the maestro's machine, outside the repository; nothing here changes them.

## 3. Sources searched

| Source | Revision read | Licence | Why searched |
| --- | --- | --- | --- |
| `trailofbits/skills` | `470419e` (2026-11-04) | CC BY-SA 4.0 | Security and testing skills, the origin of the five present; Rust review |
| `openai/skills` | `49f948f` | Apache-2.0 per skill | Codex's official catalogue |
| `anthropics/skills` | `ca45fc4` | Apache-2.0 per skill (`skill-creator`) | Claude's official catalogue |
| `obra/superpowers` | `22f0b46`, the commit C-60 checked | MIT | Development-flow skills (C-60) |
| `leonardomso/rust-skills` | `fd2a861` (1.5.1) | MIT | The most cited community Rust skill |
| `actionbook/rust-skills` (`zhanghandong/rust-skills` redirects there) | `5c40d3a` | none detected | Community Rust skills |
| Registries (`claudskills.com`, `skills.sh`, `skills.lc`) and `SHA888/cargo-skill`, `AssetsArt/skills`, `udapy/rust-agentic-skills` | search results only | — | Search for Rust, Cargo and mutation skills |

No skill for `cargo mutants` and no Rust skill from the Rust project or a known organization was found. The
registries' scores and counts are self-reported; their skills were not read.

## 4. Evaluation per candidate

Permissions are what the skill declares and what it runs; effects are what it writes. "Adapted" means the upstream
files unchanged except the frontmatter keys that grant tools or set effort, removed, and a notice added, as section 6
says.

### Recommended for adoption

| Skill | Provenance and version | What it brings | Permissions and effects | Verdict |
| --- | --- | --- | --- | --- |
| `property-based-testing` | `trailofbits/skills` `470419e`, `plugins/property-based-testing/skills/property-based-testing/`, plugin 1.2.2, 9 files | When a property earns its place, generators, shrinking, and how to tell a weak property; `proptest` is its Rust library, already this core's | Upstream `effort: low`. No script, no hook, no network. Writes test code only where the task does | Adopt, adapted |
| `sharp-edges` | Same repository, `plugins/sharp-edges/skills/sharp-edges/`, 1.1.3, 19 files with `references/lang-rust.md` | Misuse-resistant APIs and configuration: defaults, dangerous options, silent fallbacks; the class most of 7d2's passes found | Upstream `allowed-tools: Read Grep Glob` (pre-approves reading). No script, no hook. Writes nothing | Adopt, adapted |
| `differential-review` | Same repository, `plugins/differential-review/skills/differential-review/`, 1.1.4, 7 files | A review of a diff by risk: history (`git blame`), blast radius by callers, coverage of changed code, adversarial pass. For the integrator before each Codex pass, to cut passes | Upstream `allowed-tools: Read Write Grep Glob Bash` (pre-approves shell and writing). Runs read-only Git commands. Writes its report in the working directory by default (`reporting.md`, "File Naming and Location"), against the domain rule; names an `adversarial-modeler` agent and an `issue-writer` skill that are not installed | Adopt, adapted: the notice sends the report to the session's scratch directory, never the tree, and says the agent and `issue-writer` are absent |

### Recommended to write, for the bootstrap's repeated work

Two project skills by the integrator, in English, under the repository's licence, built from what the record already
fixes; nothing new is decided by writing them.

| Skill | Content | Permissions and effects |
| --- | --- | --- |
| `verify-part` | The rustup prefix (fish and POSIX); tests, crash suite with `fault-injection`, musl release build, `budget check --binary`, `clippy` with and without the feature, `fmt --check`, `cargo deny --offline`; the clean clone under `target/`; the mutation run with `--in-diff` in a user scope (`MemoryMax=3G`, `ulimit -v`, `-j 2`, `TMPDIR` under `target/`), declaring equivalents; sweeping leftovers (R-022, R-023, R-024, R-029) | Runs Cargo and `systemd-run --user`; writes under `target/` only; no `allowed-tools` |
| `codex-review` | Launching Codex for a part: disposable clone under `target/`, a copy of the Cargo registry, `codex exec --json`, standard input closed, sandbox `workspace-write`, no network, in a capped scope; the prompt worded as a review of correctness and coverage, scoped to the change; recording each pass (confirmed blocking findings, their keys, each new test failing without its correction); one heavy job at a time | Runs `git clone`, `codex`, `systemd-run --user`; writes under `target/`; no `allowed-tools` |

A third one for closing a session was considered and is not proposed: `AGENTS.md` ("Closing", "Handover") already
holds its rules, and a skill would repeat them.

### Not recommended now, with a later use

| Skill | Provenance | Why not now | Later |
| --- | --- | --- | --- |
| `spec-to-code-compliance` | `trailofbits/skills`, 2.0.2 | Runs only as a Claude Code plugin: a JavaScript workflow (`Workflow` tool) dispatching one agent per requirement, writes `spec-compliance/` at the repository's root; told not to check requirements inline | The closest match to this project (103 frozen requirements): a one-off check at a phase's close, as a plugin in a disposable clone, a maestro's decision then |
| `rust-review` | `trailofbits/skills`, 1.1.2 | A plugin with three named subagents and `uv` scripts, about 13 workers for safe Rust, `allowed-tools` with `Agent` and `Bash`, writes `.rust-review-results/` at the root; its weight is on `unsafe`, FFI, async and concurrency, all absent here | A one-off audit before release (phase 10), same terms |
| `security-threat-model` | `openai/skills` `.curated`, Apache-2.0 | Explicit-only; writes a threat model in Markdown | Phase 9 (confinement), if the maestro wants one |

### Not recommended

| Skill or set | Reason |
| --- | --- |
| `mutation-testing` (Trail of Bits, present) | Built for `mewt`/`muton`; this project uses `cargo mutants` with its own recipe, which `verify-part` keeps |
| `supply-chain-risk-auditor` (Trail of Bits, present) | npm, PyPI and Go only; its scripts run through `uv` with network. `cargo deny` covers this core's advisories and licences |
| `second-opinion` (Trail of Bits, 1.8.3) | Launches Codex read-only in the repository with temporary files from `mktemp` (memory on this machine) and its own default model and effort, against the bootstrap's recipe (R-022, R-024); `codex-review` keeps the recipe |
| `gh-cli`, `github-triage` (Trail of Bits); `gh-address-comments`, `gh-fix-ci`, `yeet` (OpenAI) | This repository has no remote |
| `cli-creator` (OpenAI) | Builds a CLI from an API for Codex; this command surface is specified |
| `security-best-practices` (OpenAI) | References cover Python, JavaScript and Go, not Rust |
| `skill-creator` (Anthropic, OpenAI) | Already available to both agents (Claude's synced copy, Codex's system skill); nothing to install |
| Other Trail of Bits skills (smart contracts, C and C++, Python, Burp, YARA, Lean, fuzzing, Semgrep and CodeQL, DWARF, Trailmark) | Outside this project's languages and tools; `cargo-fuzz` may return if fuzzing enters the test plan |
| Superpowers as a plugin (C-60) | Still installs a `SessionStart` hook that injects its method into every session (`hooks/hooks.json`), the reason TickBrew declined it; that would override `AGENTS.md`'s flow |
| Superpowers' `verification-before-completion`, `receiving-code-review`, `systematic-debugging`, `test-driven-development` taken alone | Sound, but the bootstrap already requires evidence before a claim and checks every review finding before correcting; their imperative wording ("YOU MUST") competes with `AGENTS.md` |
| `leonardomso/rust-skills` | 265 rules, one author, ranks memory and performance optimization as critical, against this core's priorities (size budget, plain code); last push 2026-06-14 |
| `actionbook/rust-skills` | No licence detected: not adoptable |
| Registry skills (`claudskills.com` and others), `cargo-skill`, `AssetsArt/skills`, `udapy/rust-agentic-skills` | Provenance unverified or a single author; `cargo-skill` installs into `.claude/skills/`, outside the domain |

## 5. Where they are read from

- **Route A (recommended): the domain only.** Each approved skill is a folder in `.brewstillery/skills/`. `AGENTS.md`
  lists them, one line each saying when to read which `SKILL.md`. No entry under `.claude/` or `.agents/`: neither
  tool loads them by itself, so no frontmatter grants a tool, no hook registers, and nothing is loaded twice beside
  the five present in the user's folders. The cost: the agent reads a skill because `AGENTS.md` says so, not because
  the tool matched it.
- **Route B: native discovery.** As route A, plus `.claude/skills/<name>` and `.agents/skills/<name>`, each a
  symbolic link to `../../.brewstillery/skills/<name>`, declared as entry files. Both tools then match skills by
  their description. Two directories at the root beside `AGENTS.md` and `CLAUDE.md`; links, which the core never
  follows in the domain; and, on this machine, two skills of the same name for each of the three Trail of Bits ones
  while the user's copies stay.

## 6. Licence and changes

Trail of Bits' skills are CC BY-SA 4.0. The criterion of D-030 (open source, beside any product, obliging only
credit) was fixed for Fabrewk's own licence. These files are not part of the product or its binary: they keep their
own licence, the licence text goes in each skill's folder, and an adaptation stays CC BY-SA 4.0 (share-alike) with
its changes stated. Adopting them is the maestro's to accept under that condition. Changes made in the adapted copies:
the frontmatter keys `allowed-tools` and `effort` removed; a notice after the frontmatter naming the source, revision,
licence and changes, and saying that this repository's `AGENTS.md` and the maestro's authorization come first, that
reports go to the session's scratch directory and never into the tree, and that no tool is installed by the skill.

## 7. The reserved name

Section 2 makes `skills/` part of the domain. The exact change proposed for the core:

- `crates/core/src/settings/mod.rs`: `RESERVED` gains `"skills"` (ten names), so no alias may be `skills`.
- `crates/core/src/domain.rs`: `DIRS` gains `"skills"`, so a `skills/` directory at the domain's top is listed and
  `check` reports nothing; a file or a link named `skills` stays `DOMAIN_UNKNOWN_ENTRY`.
- `crates/core/tests/domain.rs`, `req_dom_2_…`: a `skills/` directory gives no finding; a file `skills` is an unknown
  entry. `ALIAS_INVALID`'s test already iterates `RESERVED` and so covers `skills`.
- `core-specification.md`: REQ-DOM-2 lists `skills/`; REQ-DOM-3 lists `skills`; the paragraph "The domain, as part
  7c2b fixed it" counts `skills/` among the directories. The core reads nothing inside `skills/`.
- `.brewstillery/settings.toml`'s `layout.reservedNames` is a bootstrap key that no code reads; adding `skills` there is
  the maestro's edit, if wanted.
