# Architecture decisions (plan phase 5)

Phase 5 deliverable, one part (D-030), written by the integrator in session 8. For each decision of the packet in
`plan.md` it gives the options, their consequences and a recommendation. Nothing here is decided until the maintainer
answers in chat; each answer is then recorded in `decisions.md` with the maintainer's words, and this file marks it.
Codex reviews this file for contradictions with the alambique before and after the maintainer decides.

How to read it. Each decision (AD-1 to AD-11) states what it settles and which alambique points and earlier decisions
bind it, then the options, the recommendation and what the maintainer decides. Findings cited by key come from
`harness-verdicts.md` (H3A, observations) and `reference-study.md` (PB, CA, PR, PP, IN, OT, UC, AP, SQ); the key is a
pointer, the sentence says what it means. Names of files, keys and commands below are proposals until decided.

## 0. Inputs and what was checked

Inputs: the alambique input at `4bdb997`; `decisions.md` up to D-030; `harness-verdicts.md`; `reference-study.md`;
both settings files. `past-defect-cases.md` is phase 6's input; it is cited only where a decision here would make one
of its cases impossible to meet.

Checked on this machine in session 8, no network: Python 3.14.10 with `tomllib` and the `sqlite3` module (SQLite
3.51.2); `uv` 0.13.5 present, `pipx` and the `sqlite3` command-line tool absent; Git 2.56.0; bubblewrap 0.12.0; the
Apache-2.0 text (sections 3, 4 and 6) in `/usr/share/licenses/abseil-cpp/LICENSE` and the MIT text in
`/usr/share/licenses/uv/LICENSE-MIT`; `uv` and `rust` ship both `LICENSE-MIT` and `LICENSE-APACHE`; installed Python
packages with metadata version 2.4 carry a `License-Expression` field (PEP 639). Not checked, for lack of network:
whether the name `fabrewk` is free on PyPI; Python's end-of-support dates (taken from the release-schedule PEPs as
known, not re-read); whether GitHub can protect a ref outside `refs/heads/` and `refs/tags/`.

## Map of the packet

| # | Decision | Alambique points and decisions | Depends on |
| --- | --- | --- | --- |
| AD-1 | The single registry: its authority and where it lives | 6.26, 6.27, 6.28, 6.29, 6.37, 6.41; D-002, D-005 | — |
| AD-2 | Settings: format, levels, authority, catalogue | 6.10, 6.11, 6.13, 6.14, 6.15; D-007, D-015 | AD-1 |
| AD-3 | Language classes and their keys | 6.20, 6.21, 6.22; D-008, D-014 | AD-2 |
| AD-4 | Implementation language | Section 1, 6.2, 6.46; D-009 | — |
| AD-5 | Core boundary and size budget | 6.2, 6.36 | AD-4 |
| AD-6 | Minimal command surface | 6.5, 6.18, 6.24, 6.47; D-001 | AD-1, AD-5 |
| AD-7 | Safe installation and the assessment | Section 1, 2, 6.27, 6.38; D-006 | AD-1, AD-6, AD-8 |
| AD-8 | Local unversioned area and ignore rules | 6.4, 6.9; section 8.1.1 | AD-1 |
| AD-9 | Licence | Section 1, 6.33, 6.44; D-030 | AD-4 |
| AD-10 | How tasks are grouped, and the name of the group | Section 6 introduction, 6.16; D-019 | AD-1 |
| AD-11 | Closing a work session, permanent version | 6.44; D-022 | AD-1, AD-10 |

AD-10 and AD-11 are not in the plan's list; they were deferred to phase 5 by D-019 and D-022.

---

## AD-1. The single registry

**What it settles.** Where the one authority for tasks, authorizations, attempts, verdicts and decisions lives, and
how `STATUS.md`, the alambique's answers zone and every other view derive from it (6.26, 6.8). What must hold: a clean
clone rebuilds the state; the record does not live on one machine or depend on one interface; history is reviewable;
updates are serialized; branches and machines can collaborate; corrections never overwrite history (6.27, 6.29, 6.37).
D-002 adds that everything Fabrewk works on in a repository lives in `brewstillery/`.

What the earlier work leaves: the harness kept an append-only event log as the only authority, replayed into state,
every write guarded by compare-and-swap (harness, "In short" 1); the log lived on its own Git ref, which a plain clone
does not bring (observation "The state ref travels only on purpose"). Pullboard keeps SQLite as each machine's working
copy and shares a linear log of moves on a dedicated ref (PB-9). The harness's design rejected SQLite as the authority;
SQLite cannot be the channel between machines, and a database file committed to Git is unsafe unless it is closed and
checkpointed first, and Git cannot merge it in any case (SQ-2, SQ-3).

**Options.**

- **A. Event files in the tree.** Each event is one immutable JSON file under `brewstillery/data/` (the name the
  alambique proposes and `layout.reservedNames` already reserves), committed by Fabrewk in its own commits, never by
  an agent. State is derived by replaying the files. Within one machine, writes are serialized by a lock and by
  compare-and-swap on the task's last event. Across branches and machines, event files merge in Git without conflicts
  because each is a new file; two events that both claim to follow the same event of a task are a fork, which the
  replay refuses and hands to a person.
- **B. Event log on a dedicated Git ref.** The harness's model: `refs/fabrewk/…`, one commit per event, updated with
  compare-and-swap, linear by construction.
- **C. SQLite working copy and a shared log.** Pullboard's model (PB-9): a database per machine, a log on a ref, a
  fresh clone replays the log through the same engine.

| | A. Files in the tree | B. Dedicated ref | C. SQLite and log |
| --- | --- | --- | --- |
| Inside `brewstillery/` (D-002) | Yes | No: a ref is outside every directory | No, for both the database and the ref |
| A plain clone brings it (6.27) | Yes | No: needs an explicit fetch rule on each clone | No, same as B |
| Its history inspected without Fabrewk (6.27, "no depender de una interfaz") | Yes: files, in any Git host | With Git commands on the ref; hosts do not show such refs | The log, with Git commands on the ref, as B; the database, with SQLite's tools on the machine that holds it |
| Current state rebuilt | By Fabrewk replaying the files | By Fabrewk replaying the log | By Fabrewk replaying the log into the database |
| Reviewed with the work, in a PR (6.13, 6.44) | Yes | No | No |
| Order across machines | Per task; a fork needs a person | Linear; a rejected push replays and retries | Linear, same as B |
| Server-side protection against rewriting | The default branch's protection applies | Not checked whether a host can protect such a ref | Same as B |
| When `brewstillery/` is not published (6.27) | The record goes with it, and the warning says so; a separate repository for `brewstillery/` is a plain fallback | The ref could be pushed while the domain is not, which splits the record from its context | Same as B |
| Parts to build | Fewest | A ref writer, a fetch rule | A database layer, a ref writer, a replay engine |

**Recommendation: A**, with SQLite left out of the first core. If a query ever needs it, SQLite may become a local
index in the unversioned area (AD-8), rebuilt from the event files and never the only copy of anything (6.4). Option
B's linear order is what a project with many machines moving the same task at once would need; A refuses that case to
a person instead of ordering it, which is the honest behaviour for a first core with one or few maintainers.
Consequences:

- Events carry a schema version; a migration is a versioned reader or a new event, never a rewrite of old files
  (6.27, UC-6). The bootstrap record is imported as events of imported history (D-005, phase 13).
- `STATUS.md` and the answers zone are generated and committed views; a check compares them with the replay (phase 8).
- Agents never see `brewstillery/`: they work on an exported copy without `.git` or the domain (harness "In short" 5),
  so they cannot write events.
- Fabrewk commits the event files into the product's history only when the maintainers publish `brewstillery/`. When
  they do not (6.27), `brewstillery/` is ignored by the product's repository and Fabrewk never commits anything under
  it there, since a later push would publish it. The record then lives in the working tree of one machine, which is
  what the warning says; if the maintainers take the last exit the warning offers, `brewstillery/` is its own Git
  repository and Fabrewk commits there instead. The publication mode is recorded with the maintainers' words at
  installation (AD-7), and every commit rule in this file depends on it.
- Open for phase 6, stated here so it is not lost: where the events of unmerged work live when the default branch is
  protected. The candidate is that a task's events travel on the task's branch and merge with its work, and that
  views read the open task branches Fabrewk knows. Phase 6 specifies it with its refusals.

**The maintainer decides:** A, B or C; and whether `data/` keeps its name (recommended: yes, it is the alambique's
name and already reserved).

---

## AD-2. Settings: format, levels, authority and catalogue

**What it settles.** The format of settings files, their levels and precedence, how authority is protected, how
"permitted" and "present" are kept apart (D-015), and where the catalogue of agents and the profiles live (6.14,
6.15).

**Format.** Options: TOML (today's files), JSON, YAML, a UCI-like format. UCI is out of scope (D-028); YAML needs a
dependency and has well-known type surprises; JSON has no comments, which a person editing policy needs. **Recommended:
TOML**, read with the standard library (`tomllib`). Fabrewk never rewrites a settings file: a rewrite would lose the
maintainer's comments and order (UC-3). When Fabrewk wants a change, it proposes the edit to the maintainer, as the
alambique does; agents keep not editing these files. Keys are addressed by a stable path, `section.key`, used by the
effective configuration and by every message that names a value (UC-1). Keys keep the style already in use
(`camelCase` inside `[section]`).

**Levels, lowest to highest.**

1. **Fabrewk's defaults**, in code. Every key is declared once, with its type, the levels allowed to set it and its
   description. Proposed rule: an unknown key in any file is refused, not ignored; it carries to settings the lesson
   of CA-6, where an unsupported value was silently dropped.
2. **Project**, `brewstillery/settings.toml`: what is permitted (D-007), the catalogue, the profiles, the technical
   languages, limits and budgets. For anything that grants authority, Fabrewk reads this file from the base commit, not
   from the working tree, so an uncommitted or proposed change cannot authorize itself (PB-4; the harness observation
   "Configuration read from the working tree"). An uncommitted edit is shown as pending and not used. The policy a
   task uses is bound to it by revision or digest, in every publication mode (AD-1):
   - `brewstillery/` published: the file at the product's base commit.
   - `brewstillery/` in its own repository: the file at a commit of that repository, recorded with the task.
   - `brewstillery/` neither published nor in its own repository: there is no commit to read. The policy in force is
     the file's digest that the responsible maintainer confirmed with their words, recorded as an event; Fabrewk
     reads the file, refuses to use it if its digest differs from the confirmed one, and a changed file takes effect
     only when the maintainer confirms its new digest. The comparison of the old and new policy below applies to that
     confirmation. This control is weaker than a commit protected by a Git host, since one machine holds both the file
     and the record, and Fabrewk says so with the warning of 6.27.
3. **Personal**, `brewstillery/<alias>/settings.toml`: preferences only, from an allowlist of keys (interaction
   language, presentation). A key outside the allowlist is refused with its code (PP-9). A preference can never widen
   permissions or change criteria, providers or limits (6.11).
4. **Stage values**: not a file. The plan of a stage (AD-10): its steps, profiles, models, efforts, files per writer
   and budget, fixed by the stage's authorization in the registry (6.16). They override project values for that
   stage's tasks.
5. **Per-task exceptions**: not a file either. They are part of a task's authorization in the registry, given by the
   responsible maintainer with their words (6.24), and override the stage's values for that task only.

For levels 4 and 5 the same rule holds: a value may narrow what the level below allows at once; a widening needs the
responsible maintainer's explicit authorization naming it, recorded, and is shown as a widening in the effective
configuration. Personal preferences apply only to preference keys and are not changed by levels 4 and 5.

**Present, apart from permitted (D-015).** What exists on the machine (tools, their versions, logins, MCP servers) is
detected at each start from the machine itself, never from a record of an earlier install (PP-4), cached with its date
in the unversioned area (AD-8) and never written into a settings file. Each detected value says how it was known:
observed, verified by use, or only declared (PP-1, PP-3). The effective set is what is both permitted and present;
naming something in the catalogue grants nothing (6.14, CA-8).

**Effective configuration.** One command prints every value with its source: file and key at a revision, a default,
a task exception, or "detected" with how and when (6.11; PP-7 notes that Claude Code does not show this per key).

**Protecting authority (6.13).** At integration, Fabrewk compares the policy at the base with the policy in the
candidate. A candidate that widens authority (roles, permissions, providers, limits, the checks that guard them) is
refused unless a recorded authorization of the responsible maintainer names that widening. Fabrewk labels each of its
controls as effective (enforced by code, by the Git host or by a signature) or cooperative (an agent running as the
same user could bypass it), and says which is which (PB-6, AP-7, 6.38).

**Catalogue and profiles.** One catalogue in the project file with three separate tables, tools, providers and models;
each tool declares what it can do and how Fabrewk degrades when a capability is missing (CA-1, CA-4). Profiles refer to
catalogue entries and declare the task kind, role, model, effort, checks, required tools and limits (6.15); a profile
is a named view over the catalogue, not a copy (PR-1). Validation before a run checks that the model is usable on that
account when that can be observed, not only that it is named (PR-5); the model and effort observed are recorded apart
from those requested (PR-6). An unsupported permission mode refuses; it never falls back to the tool's default (CA-6).

**Machine rules (6.38).** A rule such as "Codex starts inside a repository, never in a parent directory" belongs to the
Codex adapter and is checked at every launch; it is not a setting a file could turn off. No user-level settings file
outside the repository in the first core: what is personal travels in the alias's file, what belongs to the machine is
detected.

**The maintainer decides:** TOML; the five levels and their rule; reading authority from the base commit; the
catalogue's three tables in the project file. Key names are fixed in phase 6.

---

## AD-3. Language classes and their keys

**What it settles.** Which classes of content have their own language, which level sets each, and the final names of
the keys (6.20, 6.21; D-008 and D-014 left the names to this phase).

**Recommendation.** Every class has a key; only the three in use are required. The others are optional and inherit from
a parent, so nothing is hard-coded and nothing must be configured twice (6.10). Language tags are BCP 47 (`en`,
`es-MX`); a regional variant falls back to its base and then to English (as the harness's `i18n.py` did).

| Class | Level | Key | If absent |
| --- | --- | --- | --- |
| Technical files: agent files, `STATUS.md`, the record, canonical documents | Project | `languages.canonicalDocumentationLanguage` (in use, D-008) | `en` |
| Publication languages of the documentation | Project | `languages.documentationLanguages` (in use, D-014) | the canonical language only |
| Code and comments | Project | `languages.codeLanguage` | the canonical language |
| Evidence and attempt reports | Project | `languages.evidenceLanguage` | the canonical language |
| Packets delegated to agents | Project | `languages.delegationLanguage` | the canonical language |
| Interaction with a maintainer: agents' answers, alambique answers | Personal | `languages.agentInteraction` (in use for `xel`) | the canonical language |
| Fabrewk's interface messages to a person | Personal | `languages.interfaceLanguage` | that maintainer's `agentInteraction` |

The alambique's input is free text in whatever language the maintainer writes; its answers follow `agentInteraction`.
A person's authorization is recorded in their own words with its language tag and bound to the action it authorizes;
a translation never creates or widens an authorization (6.20). Identifiers, states, refusal codes, commands and flags
never change with the language. The three keys in use keep their names, though they do not share one style, because
renaming a decided key costs more than its inconsistency.

**The maintainer decides:** the class table and the four new key names.

---

## AD-4. Implementation language

**What it settles.** The language of Fabrewk's own code.

| Option | For | Against |
| --- | --- | --- |
| **Python** | The maintainer's stated intent (section 1); the pilot is Python (D-009); the harness's mechanisms, reused as design, are Python; the standard library has TOML reading, SQLite, JSON, hashing, subprocesses and archives, so the core can have no runtime dependency; `uv` installs a tool in one command | Needs an interpreter on the machine; startup slower than a compiled binary; types are checked only by an external tool |
| Go | One static binary per platform, simple `curl` installation, fast start | A second language next to the Python pilot; nothing of the harness carries over; the maintainer's stated intent is Python |
| Rust | Same as Go, with stronger guarantees | Highest cost to write and review for a small team; same carry-over problem |
| TypeScript / Node | Several references use it (Pullboard, RSC-Harness, OpenSpec) | Runtime dependency on Node, a large package ecosystem to audit (6.38), no carry-over |

**Recommendation: Python**, with these rules. The supported floor is Python 3.12: 3.11 reaches its end of support in
October 2027 (from its release-schedule PEP, not re-checked), before Fabrewk would be in wide use; the machine runs
3.14.10. The core uses only the standard library; any runtime dependency needs a recorded reason (6.2). Tests use the
standard library's `unittest`, as the harness did. The package and command are `fabrewk` (D-001); whether the PyPI
name is free is unchecked (no network this session) and is checked before publication (phase 10).

**The maintainer decides:** the language and the floor version.

---

## AD-5. Core boundary and size budget

**What it settles.** What belongs to the core, what goes in adapters, and how much code each may have (6.2: a small
core, clear responsibilities, justified dependencies, extensions through adapters).

**Boundary.**

- **Core:** the registry (event files, replay, the one declarative table of transitions, compare-and-swap, fork
  detection); digests and input identities; settings (levels, provenance, validation); the gate of checks; the
  alambique's separator and the writer of its answers zone; the views (`STATUS.md`, status); the mechanism of message
  catalogues; the command dispatcher.
- **Adapters, each behind a contract the core declares:** one per tool (Claude Code, Codex, later OpenCode), with its
  stream audit; one per confinement backend (bubblewrap on Linux first); the installer (assessment, plan, apply);
  later, the following of external sources (6.31).
- **Data, not code:** the message catalogues' texts.

The core names no tool, provider or model (PB-8); adapters depend on the core's contracts, never the reverse.

**Budget.** The harness's core reached 3,089 of its 3,500 limit for one task, one maintainer and one machine, and its
size count passed (harness-verdicts, "What was read and run"); Pullboard shows how fast agent-built code grows (PB-11).
The budget is set before building, checked by a tool from the first commit of code, and raised only by the maintainer's
recorded decision with its reason, never by an agent (6.25).

A caution on the unit: `harness-verdicts.md` lists 1,569 physical lines for the core's Python files, while the
harness's counter reported 3,089 for its core, so that counter's unit is not a physical line and the figures cannot be
compared directly. **Recommended:** count non-blank lines that are neither comments nor docstrings, with Fabrewk's own
tool, and freeze these limits in phase 6 after measuring the harness's core with that same tool: core 2,500; each tool
adapter, its stream audit included, 400; each confinement backend 300; the installer 800. Tests and catalogue texts are
not counted.

**The maintainer decides:** the boundary; the unit; the four limits, or that phase 6 fixes them after the measurement.

---

## AD-6. Minimal command surface

**What it settles.** The rules every command follows and the first set of commands (6.5, D-001). Final names and
arguments are fixed in phase 6 with the table of transitions.

**Options for the shape.**

| Option | For | Against |
| --- | --- | --- |
| **Flat verbs** (`fabrewk run`, `fabrewk accept`), as Git's porcelain | Short; one word per move of the transition table, so the table and the commands stay one list | The list grows with every move; it must stay small by design |
| Noun groups (`fabrewk task run`, `fabrewk config show`) | Scales to many objects | Longer commands, and two words to remember for every move; with few objects it adds a level that buys nothing |
| One command per role (`fabrewk-review`, `fabrewk-install`) | Each tool small | Several names to install and document; breaks D-001's single command `fabrewk` |
| Conversational only, through an agent | Nothing to learn | Nothing a person can run or a test can check; a person's words would pass through the agent (6.24) |

**Recommendation: flat verbs**, with one exception kept for reading: `fabrewk config` takes an optional key to show one
value. If a later stage adds many objects, groups can come then, under 6.5's rule for an ordered change.

**Rules.**

- One declaration lists every command, its arguments, the moves it may cause and its refusals; help, the reference
  documentation and shell completion are generated from it (PB-1, 6.45).
- English verbs, one meaning each, no aliases and no translations; long options in `--kebab-case`; `--help` and
  `--version` everywhere; `--json` for machine output with stable codes; stable exit codes.
- Every refusal prints its code, the reason in the person's language and the next step as a real command. A test fails
  if any message names a command or flag that does not exist (the legacy pilots' `tbh` lesson).
- A person's move takes `--say "<their words>"`, kept verbatim; Fabrewk never prints words for the person to copy
  (FZ2B-C4-02). It is refused when the environment shows an agent's markers; that control is cooperative and is
  declared as such (PB-6).
- Reading never waits for a writer: `status` works during a dispatch (harness observation "No view during a
  dispatch"; 6.47).
- Every command Fabrewk prints works the same in fish and in a POSIX shell, with no shell-specific syntax, and the
  examples are tested in both.

**First set** (provisional names):

| Command | Does | Writes |
| --- | --- | --- |
| `fabrewk status` | The state, what waits for whom | No |
| `fabrewk config` | The effective configuration with the source of each value | No |
| `fabrewk check` | The record replays, the views match it, the settings parse and validate | No |
| `fabrewk assess` | Assesses a repository and prints a verdict and a plan with its identity (D-006) | No |
| `fabrewk apply <plan> --say` | Applies an assessed plan the maintainer accepted | Yes |
| `fabrewk propose` | Records a task proposal: goal, requirements, base, dependencies, files, profile (6.28) | Yes |
| `fabrewk authorize <task> --say` | A person authorizes a task or a group of tasks (AD-10) | Yes |
| `fabrewk run <task>` | Runs the authorized plan: attempt recorded first, dispatch, review, checks (6.41) | Yes |
| `fabrewk accept <task> --say` | A person accepts the verified result | Yes |
| `fabrewk reject <task> --say` | A person rejects it, with their reasons | Yes |
| `fabrewk sweep <alias>` | Prepares an alambique sweep and writes only its answers zone, stopping if the separator is not found exactly once (6.7) | Yes |

**The maintainer decides:** the rules, and the first set as the starting list for phase 6.

---

## AD-7. Safe installation and the assessment

**What it settles.** How Fabrewk is installed (section 1: a script fetched with `curl`, or a prompt the maintainer's
agent can follow, both reaching the same command) and how it enters a repository without writing first (D-006, 6.38).

**Options for installing the tool.**

| Option | For | Against |
| --- | --- | --- |
| **`uv tool install`**, called by the script | One command; an isolated environment per tool; `uv` can also download a Python when the machine lacks a suitable one (its `--no-python-downloads` option, seen in `uv tool install --help`); present on this machine (0.13.5) | `uv` must be installed first |
| `pipx install` | Same isolation, an older and widely packaged tool | Absent on this machine; whether it can fetch a Python itself depends on its version and was not checked (no network) |
| `pip install` into the user's or the system's Python | Nothing else needed | Mixes Fabrewk's packages with the machine's; PEP 668 lets a distribution refuse it (an externally managed environment); this machine's Python 3.14 carries no such marker |
| A standalone binary per platform | No Python needed | Only practical with a compiled language (AD-4), or with a bundler that adds a large build dependency |
| Distribution packages (Homebrew, AUR, Fedora) | Native updates | One recipe per system to maintain; later, once there are users who ask for them |

**Options for entering a repository.** Writing first and reporting after, even idempotently (Pullboard, PB-10), is
excluded by D-006. A dry run that is optional (apk, AP-2) leaves the preview to the person's memory. A plan with its
identity, confirmed and then applied only if unchanged (RSC-Harness, IN-1), is the one that meets D-006 as written.

**Recommendation.**

1. **Install the tool, outside any repository.** The script is short, readable POSIX `sh`; it installs a pinned release
   with `uv tool install` and verifies the downloaded package against a published checksum. If `uv` is missing it stops
   and says how to get it; it never installs another tool on its own (IN-6, PP-5). The page shows two routes: piping
   into `sh`, and downloading, reading, then running the script; both work in fish. Trusting the release source is a
   grant of full authority over what the script installs, and the page says so in one sentence (AP-3).
2. **Assess, writing nothing.** `fabrewk assess` reads only what D-006 lists (Git state, root files, an existing
   `AGENTS.md`, the licence, another harness present) plus the side effects other tools leave that would change what
   agents may do: hooks in `.claude/` or `.codex/` settings, automatic updates, telemetry, knowledge syncs that commit
   or push, generated files that pre-approve tools (reference-study section 6, PP-10). No network during the
   assessment. It ends in a verdict, adopt, review only or do not enter, with its reasons, and a plan: every file it
   would create or change, with its exact content or difference, and the plan's identity as a digest of its content
   (IN-1).
3. **Apply only what was accepted.** `fabrewk apply <plan> --say` refuses if the plan changed since the maintainer saw
   it (IN-1); proceeds only on an explicit yes and refuses on no answer (AP-2's sharp edge; IN-3); never overwrites a
   file it did not write: in an existing `AGENTS.md` it adds a delimited reference block (D-002, IN-4); records the
   digest of every file it wrote, so an uninstall removes only files that still match (IN-3, AP-5). The verdict and
   that receipt are the repository's first record entry (D-006).
4. **Publication of `brewstillery/`.** The plan asks whether the maintainers will publish `brewstillery/`. If not, the
   warning of 6.27 is shown, its consequences stated, the exits offered (a separate repository for `brewstillery/` as
   the last one), and the maintainers' answer recorded in their own words. The warning's text is phase 10's.
5. **Review mode** writes only its own evidence in a subdirectory of `brewstillery/` and changes neither code nor the
   project's `AGENTS.md` (section 1). Its directory name is fixed in phase 6.

Fabrewk itself installs no hook into an agent's settings, turns on no automatic update and sends no telemetry, unless
the maintainer accepts that exact item in a plan.

**The maintainer decides:** the five steps; `uv` as the installer the script relies on.

---

## AD-8. Local unversioned area and ignore rules

**What it settles.** Where Fabrewk keeps what must not be versioned, how it is kept out of Git without cluttering the
root, and how each kind of file is classified (6.4: versioned content, local state, conserved evidence and temporary
files; nothing needed to continue or audit lost as cache; no credentials anywhere; declared persistent memory inside
`brewstillery/`).

**Where the ignore rule lives.**

| Option | For | Against |
| --- | --- | --- |
| The root `.gitignore` | Usual place | Writes outside `brewstillery/` and edits a file the project owns (D-002) |
| **`brewstillery/.gitignore`** | Inside the domain; Git honours ignore files in subdirectories; travels with every clone | One more file in `brewstillery/` |
| `.git/info/exclude` | Touches no tracked file | Per clone: a fresh clone lacks it and could commit the local area by mistake |

**Recommendation:** `brewstillery/.gitignore`, written and owned by Fabrewk (its digest is in the installation
receipt), with one rule, `local/`, which matches `brewstillery/local/` and a `local/` inside any maintainer's space.
`local` becomes a reserved name (the maintainer adds it to `layout.reservedNames`; the file is theirs).

**Classification.**

| Kind | Where | Rule |
| --- | --- | --- |
| Versioned | Settings, alambiques and their material, `data/`, `STATUS.md`, `brewstillery/.gitignore` | In the product's Git when `brewstillery/` is published; otherwise only in the working tree, or in `brewstillery/`'s own repository if the maintainers took that exit (AD-1) |
| Conserved local evidence | `brewstillery/local/evidence/` | Providers' raw streams and logs that informed a verdict: never deleted automatically; `fabrewk status` lists them; the record holds their digests (harness "In short" 7) |
| Cache | `brewstillery/local/cache/` | Rebuildable, safe to delete: detection results with their date, a future SQLite index |
| Temporary | `brewstillery/local/tmp/` | Agents' exported copies and scratch files, deleted when the attempt ends; no machine-wide temporary directory (section 8.1.1) |
| Lock | `brewstillery/local/lock` | The single writer's lock |
| A maintainer's private material | `brewstillery/<alias>/local/` | Material cited from the alambique that must not be versioned (6.9) |

The assessment's plan declares this area before creating it (6.4, "declararla al mantenedor y ser autorizada"). A check
refuses files that look like credentials anywhere in `brewstillery/` (patterns fixed in phase 6). An agent's exported
copy inside the repository is confined to that copy (phase 9); a directory boundary alone is not isolation (6.6).

**The maintainer decides:** the ignore file's place; `local` as the area's name and a reserved name; the classification.

---

## AD-9. Licence

**What it settles.** Fabrewk's licence, against the criterion of D-030: open source; able to live beside any product
under an open or closed licence; obliging those who use or fork Fabrewk only to credit it.

**What a licence can require.** A software licence binds whoever copies, modifies or redistributes the software. It
cannot bind a project that only runs Fabrewk on its code, and an open-source licence must not try: that would make
Fabrewk part of the product's terms, which 6.1 forbids. So "credit us" holds by licence for forks and for anyone who
ships Fabrewk's code; for projects that only use it, the credit is a courtesy (a line in the entry files Fabrewk writes,
which the project may remove) and the name is protected by trademark, not by the licence.

**Options.**

| Licence | Obligation | Patents | Fits the criterion |
| --- | --- | --- | --- |
| MIT | Keep the copyright and permission notice in copies or substantial portions | No explicit grant | Yes. Compatible with GPL-2.0, GPL-3.0 and closed products |
| Apache-2.0 | Keep the notices; a distributed derivative work includes a readable copy of the attribution notices of the `NOTICE` file, in a `NOTICE` file, its source or documentation, or its display (section 4 d), the strongest credit among permissive licences; mark modified files (4 b) | Explicit grant, ended for whoever brings a patent suit claiming that the Work or a contribution in it infringes (section 3) | Yes, with one gap: the FSF considers it incompatible with GPL-2.0-only code (stated, not re-read) |
| BSD-2-Clause, ISC | As MIT | No | Yes, as MIT |
| **MIT OR Apache-2.0** | The recipient chooses either; both keep the credit | The Apache grant, for whoever chooses it | Yes, and closes Apache's GPL-2.0 gap; used by `uv` and Rust (their licence files are on this machine) |
| GPL, LGPL, AGPL, MPL | Share source of modified files or works | Varies | No: more than credit |
| 0BSD, MIT-0, Unlicense, CC0 | None | No | No: not even credit |

**Recommendation.** Fabrewk's code and documentation under **MIT OR Apache-2.0**, with a `NOTICE` file naming Fabrewk
and its authors; the package declares the expression `MIT OR Apache-2.0` (PEP 639). What Fabrewk writes into a
repository it serves (the reference block in entry files, the skeleton of `brewstillery/`, settings templates, generated
views) is the project's and carries no obligation: those templates are under **MIT-0**, so a closed-source project owes
nothing for them. Further consequences:

- The licence grants no right to the name or logo (Apache section 6 says so). The trademark check with a professional
  before commercial use stays (alambique section 1); a short trademark note in the README says the name is not licensed.
- Contributions come in under the same licence they go out under; whether to ask for a sign-off (DCO) is decided when
  Fabrewk accepts outside contributions (phase 10).
- `tickbrew-harness` has no licence and is the maintainer's; any code carried over literally comes in under Fabrewk's
  licence by the maintainer's decision. The default stays "reuse as design" (harness-verdicts, "The three verdicts").
- The licence files are created with the first code (phase 7), not before (`AGENTS.md`: no licence yet).

**The maintainer decides:** the licence of the code and documentation; MIT-0 for what Fabrewk writes into served
repositories.

---

## AD-10. How tasks are grouped, and the name of the group

**What it settles.** The alambique leaves to the architecture how tasks are grouped and what the group is called (section
6, introduction); TickBrew's "phase", "gate" and "cut" are out. 6.16 describes the group: an execution plan with steps,
profiles, models and efforts, files per writer and dependencies, authorized once and run within a budget; section 2 says
the alambique is swept when it closes.

**Options for the name** (an English identifier; the person sees it in their own language through the catalogue,
"etapa" in Spanish):

| Name | For | Against |
| --- | --- | --- |
| **stage** | The alambique's own word, "etapa"; plain English | Git calls adding to the index "staging"; Fabrewk never uses that sense in its commands, but a reader might |
| batch | No clash | Sounds mechanical; says nothing of a plan or an authorization |
| milestone | Common | GitHub uses it for issues, with another meaning |
| iteration, sprint | Common | Imply fixed time boxes, which Fabrewk does not have |

**Recommendation: `stage`.** A stage is a set of tasks under one authorization, one plan and one budget; it closes with
the alambique's sweep. In the first core its tasks run one after another; parallel steps come only after serial
execution and its recovery are shown to work (6.16).

**The maintainer decides:** the name, and that the first core runs a stage's tasks serially.

---

## AD-11. Closing a work session, permanent version

**What it settles.** D-022 left to this phase the permanent form of "a session closes with a clean tree, and the
maintainer's edits go in their own commit with no AI co-author".

**Recommendation.** With AD-1 A and `brewstillery/` published, or in its own repository (AD-1), Fabrewk commits its
own events and views as they happen, so a clean close becomes a check: a stage does not close while a file Fabrewk owns
is uncommitted. When `brewstillery/` is neither published nor in its own repository, there is nothing to commit and the
close says once more that the record lives only on this machine. Files the maintainer owns (alambique input,
settings, their material) are committed by Fabrewk only on the maintainer's word, alone, authored by the maintainer and
with no AI co-author line (6.44, preserving authorship). The project's own Git rules (6.44) decide whether those commits
go to a branch or a pull request. During the bootstrap, D-022 stays in force.

**The maintainer decides:** this rule as the permanent version of D-022.

---

## Other destinations of phase 5

Findings the study sent to phase 5 that need no separate decision, with where they land:

- Maintainer roles (6.12): the first core has one responsible maintainer per project, named in the project file and
  protected as authority (AD-2); a hierarchy of roles is added only if it brings a concrete improvement (6.12), and the
  pilot tests that editing a personal file or opening a pull request cannot raise permissions (6.48).
- Review by a different provider (6.42, PB-7): a profile rule with `off`, `prefer` or `require`; the provider's
  identity is declared by the adapter, not attested, and is recorded as such. Phase 6.
- Following external sources (6.31, 6.32, OT-2): a later stage, outside the first core; its setting's name and
  behaviour are designed then. A failed lookup is reported as unknown, never as "no change".
- The person's view lists only the decisions that wait for that person (OT-6): part of `fabrewk status` (AD-6).
