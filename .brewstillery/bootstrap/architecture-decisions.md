# Architecture decisions (plan phase 5)

Phase 5 deliverable, one part (D-030), written by the integrator in session 8. For each decision of the packet in
`plan.md` it gives the options, their consequences and a recommendation. Nothing here is decided until the maintainer
answers in chat; each answer is then recorded in `decisions.md` with the maintainer's words, and this file marks it.
Codex reviews this file for contradictions with the alambique before and after the maintainer decides.

**State (session 8).** Codex accepted this file at its third pass (R-015). The maintainer decided AD-1 (option A), AD-2
and AD-3 as recommended (D-031) and paused the rest for a sweep of input points 8.2 (a Rust core with extensions in
other languages and a local API) and 8.3 (an optional central server). AD-4 to AD-11 stay open; AD-4 and AD-5 will be
rewritten with those proposals as options, and a decision on the central server added, once the maintainer has acted on
the sweep.

**State (session 9).** The sweep became D-032 and D-033 (R-016). D-032 fixes the Rust core (point 12), extensions as
separate programs (13), confinement per operating system (14), no always-on process (15) and the central server as a
later stage (17), and supersedes two things here: AD-1's path is now `.brewstillery/data/`, and AD-2's rule of no
user-level settings outside the repository gives way to the private folder per machine (10). The repository moved to
`.brewstillery/`; the paths below are updated, the rest of the text is as Codex accepted it.

**State (session 11).** The alambique was rewritten concise and adopted (D-034, D-035); its point numbers did not
change. This session rewrote AD-4 (Rust), AD-5 (core boundary and the extension contract) and AD-7 (a binary
installation, its two routes and how the script checks what it downloads), and added AD-12 to AD-15 for what D-032
points 1 to 10 and 17 need: the domain's layout and the agents' rules file, alambique edits and the state of proposals,
identity and roles, the private folder and the instance identity. AD-6, AD-8 to AD-11 and the closing list were
brought in line with D-032 and the fermento (the alambique no longer has an answers zone). AD-1 to AD-3 keep their
decided text; a note under each says what D-032 changed. AD-4 to AD-15 wait for the maintainer.

How to read it. Each decision states what it settles and which alambique points and earlier decisions bind it, then
the options, the recommendation and what the maintainer decides. Findings cited by key come from `harness-verdicts.md`
(H3A, observations) and `reference-study.md` (PB, CA, PR, PP, IN, OT, UC, AP, SQ); the key is a pointer, the sentence
says what it means. Names of files, keys and commands below are proposals until decided.

## 0. Inputs and what was checked

Inputs (session 8): the alambique input at `4bdb997`; `decisions.md` up to D-030; `harness-verdicts.md`;
`reference-study.md`; both settings files. `past-defect-cases.md` is phase 6's input; it is cited only where a decision
here would make one of its cases impossible to meet. Inputs (session 11): `.brewstillery/ALAMBIQUE.md` as adopted in
D-035 (sha256 `4f6a1e6b…a3bf2`); `decisions.md` up to D-035; both settings files; the same two study files.

Checked on this machine in session 8, no network: Python 3.14.10 with `tomllib` and the `sqlite3` module (SQLite
3.51.2); `uv` 0.13.5 present, `pipx` and the `sqlite3` command-line tool absent; Git 2.56.0; bubblewrap 0.12.0; the
Apache-2.0 text (sections 3, 4 and 6) in `/usr/share/licenses/abseil-cpp/LICENSE` and the MIT text in
`/usr/share/licenses/uv/LICENSE-MIT`; `uv` and `rust` ship both `LICENSE-MIT` and `LICENSE-APACHE`; installed Python
packages with metadata version 2.4 carry a `License-Expression` field (PEP 639). Not checked, for lack of network:
whether the name `fabrewk` is free on PyPI; Python's end-of-support dates (taken from the release-schedule PEPs as
known, not re-read); whether GitHub can protect a ref outside `refs/heads/` and `refs/tags/`.

Checked on this machine in session 11, no network: `rustc` and `cargo` 1.99.1, packaged by Fedora (`1.99.1-2.fc44`),
no `rustup`; `rustc --print target-list` names 27 musl targets (which standard libraries are installed was not
checked); OpenSSH 10.2p1, whose `ssh-keygen -Y sign` and `-Y verify` exist (each asks for a namespace); this
repository's Git has no signing configured (`gpg.format`, `user.signingkey`, `commit.gpgsign` unset); `codex-cli`
0.166.0; `gh` 2.97.0; `curl` 8.18.0; `sha256sum` (coreutils 9.10). Absent: `minisign`, `cosign`, `docker`, `podman`,
`tokei`, `cargo-deny`. Not checked, for lack of network: the current stable Rust and crate versions; whether the name
`fabrewk` is free on crates.io; the paths where Git hosts look for a code-owners file. Statements about Rust, Cargo,
Git and OpenSSH that are not in the list above come from the integrator's knowledge and are marked "not re-checked".

## Map of the packet

| # | Decision | Alambique points and decisions | Depends on |
| --- | --- | --- | --- |
| AD-1 | The single registry: its authority and where it lives | 6.26, 6.27, 6.28, 6.29, 6.37, 6.41; D-002, D-005 | — |
| AD-2 | Settings: format, levels, authority, catalogue | 6.10, 6.11, 6.13, 6.14, 6.15; D-007, D-015 | AD-1 |
| AD-3 | Language classes and their keys | 6.20, 6.21, 6.22; D-008, D-014 | AD-2 |
| AD-4 | Rust: toolchain, targets, dependencies, tests | 6.2, 6.46; D-032 point 12 | — |
| AD-5 | Core boundary, the extension contract and the size budget | 6.2, 6.36, 6.38, 6.49; D-032 points 13 to 15 | AD-4 |
| AD-6 | Minimal command surface | 6.5, 6.8, 6.18, 6.24, 6.47; D-001 | AD-1, AD-5 |
| AD-7 | Installation: the binary, its two routes, the assessment | Section 1, 2, 6.27, 6.38; D-006, D-033 | AD-1, AD-6, AD-8, AD-14 |
| AD-8 | Local unversioned area and ignore rules | 6.4, 6.9 | AD-1 |
| AD-9 | Licence | Section 1, 6.33, 6.44; D-030 | AD-4 |
| AD-10 | How tasks are grouped, and the name of the group | Section 6 introduction, 6.16; D-019 | AD-1 |
| AD-11 | Closing a work session, permanent version | 6.44; D-022 | AD-1, AD-10 |
| AD-12 | The domain's layout, entry files and the agents' rules file | Section 2, 6.3, 6.7, 6.17; D-032 points 1 to 3 | AD-1 |
| AD-13 | Alambique edits and the state of proposals | 6.7, 6.8, 6.23; D-032 points 4 and 5 | AD-1, AD-14 |
| AD-14 | Identity, roles and signatures | 6.6, 6.12, 6.13, 6.48; D-032 points 6 to 9 | AD-1, AD-2 |
| AD-15 | The private folder and the instance identity | 6.4, 6.26, 6.50, 6.51; D-032 points 10 and 17 | AD-2, AD-14 |

AD-10 and AD-11 are not in the plan's list; they were deferred to phase 5 by D-019 and D-022. AD-12 to AD-15 come from
D-032's consequences.

---

## AD-1. The single registry

**Decided (D-031): option A.** Text as Codex accepted it; see the note at the end for what changed since.

**What it settles.** Where the one authority for tasks, authorizations, attempts, verdicts and decisions lives, and
how `STATUS.md`, the alambique's answers zone and every other view derive from it (6.26, 6.8). What must hold: a clean
clone rebuilds the state; the record does not live on one machine or depend on one interface; history is reviewable;
updates are serialized; branches and machines can collaborate; corrections never overwrite history (6.27, 6.29, 6.37).
D-002 adds that everything Fabrewk works on in a repository lives in `.brewstillery/`.

What the earlier work leaves: the harness kept an append-only event log as the only authority, replayed into state,
every write guarded by compare-and-swap (harness, "In short" 1); the log lived on its own Git ref, which a plain clone
does not bring (observation "The state ref travels only on purpose"). Pullboard keeps SQLite as each machine's working
copy and shares a linear log of moves on a dedicated ref (PB-9). The harness's design rejected SQLite as the authority;
SQLite cannot be the channel between machines, and a database file committed to Git is unsafe unless it is closed and
checkpointed first, and Git cannot merge it in any case (SQ-2, SQ-3).

**Options.**

- **A. Event files in the tree.** Each event is one immutable JSON file under `.brewstillery/data/` (the name the
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
| Inside `.brewstillery/` (D-002) | Yes | No: a ref is outside every directory | No, for both the database and the ref |
| A plain clone brings it (6.27) | Yes | No: needs an explicit fetch rule on each clone | No, same as B |
| Its history inspected without Fabrewk (6.27, "no depender de una interfaz") | Yes: files, in any Git host | With Git commands on the ref; hosts do not show such refs | The log, with Git commands on the ref, as B; the database, with SQLite's tools on the machine that holds it |
| Current state rebuilt | By Fabrewk replaying the files | By Fabrewk replaying the log | By Fabrewk replaying the log into the database |
| Reviewed with the work, in a PR (6.13, 6.44) | Yes | No | No |
| Order across machines | Per task; a fork needs a person | Linear; a rejected push replays and retries | Linear, same as B |
| Server-side protection against rewriting | The default branch's protection applies | Not checked whether a host can protect such a ref | Same as B |
| When `.brewstillery/` is not published (6.27) | The record goes with it, and the warning says so; a separate repository for `.brewstillery/` is a plain fallback | The ref could be pushed while the domain is not, which splits the record from its context | Same as B |
| Parts to build | Fewest | A ref writer, a fetch rule | A database layer, a ref writer, a replay engine |

**Recommendation: A**, with SQLite left out of the first core. If a query ever needs it, SQLite may become a local
index in the unversioned area (AD-8), rebuilt from the event files and never the only copy of anything (6.4). Option
B's linear order is what a project with many machines moving the same task at once would need; A refuses that case to
a person instead of ordering it, which is the honest behaviour for a first core with one or few maintainers.
Consequences:

- Events carry a schema version; a migration is a versioned reader or a new event, never a rewrite of old files
  (6.27, UC-6). The bootstrap record is imported as events of imported history (D-005, phase 13).
- `STATUS.md` and the answers zone are generated and committed views; a check compares them with the replay (phase 8).
- Agents never see `.brewstillery/`: they work on an exported copy without `.git` or the domain (harness "In short" 5),
  so they cannot write events.
- Fabrewk commits the event files into the product's history only when the maintainers publish `.brewstillery/`. When
  they do not (6.27), `.brewstillery/` is ignored by the product's repository and Fabrewk never commits anything under
  it there, since a later push would publish it. The record then lives in the working tree of one machine, which is
  what the warning says; if the maintainers take the last exit the warning offers, `.brewstillery/` is its own Git
  repository and Fabrewk commits there instead. The publication mode is recorded with the maintainers' words at
  installation (AD-7), and every commit rule in this file depends on it.
- Open for phase 6, stated here so it is not lost: where the events of unmerged work live when the default branch is
  protected. The candidate is that a task's events travel on the task's branch and merge with its work, and that
  views read the open task branches Fabrewk knows. Phase 6 specifies it with its refusals.

**The maintainer decides:** A, B or C; and whether `data/` keeps its name (recommended: yes, it is the alambique's
name and already reserved).

**Since D-032.** The path is `.brewstillery/data/` (D-032 point 1). "The alambique's answers zone" now reads "each
fermento's answer" (D-032 point 3: the alambique has no answers zone); it stays a view generated from the registry
(6.8). Every event also names the instance that wrote it (D-032 point 17, AD-15), and an event that records a person's
act carries that person's signature (AD-14).

---

## AD-2. Settings: format, levels, authority and catalogue

**Decided (D-031) as written.** Text as Codex accepted it; see the note at the end for what changed since.

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
2. **Project**, `.brewstillery/settings.toml`: what is permitted (D-007), the catalogue, the profiles, the technical
   languages, limits and budgets. For anything that grants authority, Fabrewk reads this file from the base commit, not
   from the working tree, so an uncommitted or proposed change cannot authorize itself (PB-4; the harness observation
   "Configuration read from the working tree"). An uncommitted edit is shown as pending and not used. The policy a
   task uses is bound to it by revision or digest, in every publication mode (AD-1):
   - `.brewstillery/` published: the file at the product's base commit.
   - `.brewstillery/` in its own repository: the file at a commit of that repository, recorded with the task.
   - `.brewstillery/` neither published nor in its own repository: there is no commit to read. The policy in force is
     the file's digest that the responsible maintainer confirmed with their words, recorded as an event; Fabrewk
     reads the file, refuses to use it if its digest differs from the confirmed one, and a changed file takes effect
     only when the maintainer confirms its new digest. The comparison of the old and new policy below applies to that
     confirmation. This control is weaker than a commit protected by a Git host, since one machine holds both the file
     and the record, and Fabrewk says so with the warning of 6.27.
3. **Personal**, `.brewstillery/<alias>/settings.toml`: preferences only, from an allowlist of keys (interaction
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

**Since D-032.** The core is Rust (D-032 point 12), so TOML is read with a crate, not with Python's `tomllib` (AD-4);
the decision on the format stands. The last rule of "Machine rules" gives way to the private folder per machine
(D-032 point 10, AD-15): it holds machine data that detection cannot find and Fabrewk's own keys, never authority and
never a sixth level of these settings; what it declares is shown on the "present" side, marked as declared. The
"responsible maintainer" of this text is a maestro (D-032 point 7, AD-14), and the roles and keys live in a section of
the project file (D-032 point 8). Every policy change that touches that section needs a maestro's signature (AD-14).

---

## AD-3. Language classes and their keys

**Decided (D-031) as written.** Text as Codex accepted it; see the note at the end for what changed since.

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

**Since D-032.** "Alambique answers" and "the alambique's input" read "fermento answers" and "the fermento's input"
(D-032 points 2 and 3). The alambique is the project's description, approved by its maestros; it is not
documentation (AGENTS.md) and is written in the language its maestros write it in. The agents' rules file (AD-12) is a
technical file: `canonicalDocumentationLanguage`.

---

## AD-4. Rust: toolchain, targets, dependencies, tests

**What it settles.** D-032 point 12 fixes the language: the core is written in Rust, every crate is justified and a
minimum Rust version is fixed. 6.2 adds one program per platform, no interpreter, easy to install and port. Left to
decide: the minimum version and the edition, the first platforms, the rules for dependencies and the first set of
crates, the external tools the core calls, and how the code is tested. What does not change: the harness's mechanisms
are reused as design, not as code (harness-verdicts, "The three verdicts"), and the pilot module stays in Python
(D-009), because it is a product, not Fabrewk.

**Minimum Rust version (MSRV).**

| Option | For | Against |
| --- | --- | --- |
| **The version Fedora ships on the maintainer's machine when phase 7 starts** (today 1.99.1) | The first platform builds with the distribution's own toolchain, no `rustup`; one clear number, written as `rust-version` in `Cargo.toml` | Lags the newest stable by a few weeks or months; a newer feature waits for the distribution |
| The newest stable, always | Every feature at once | No fixed floor: a contributor or packager cannot know what builds it; needs `rustup` on this machine |
| A fixed number several releases back | Wider reach for packagers | Gains nothing for a tool that ships its own binary (AD-7) |

**Recommendation:** the first option, raised only by a recorded decision of a maestro with its reason. Edition 2024
(it needs Rust 1.85 or newer, not re-checked). The build checks the MSRV and the newest stable once a build service
exists (phase 7).

**First platforms (6.46, D-032 point 14).** Linux on ARM64 first, the maintainer's machine. Release binaries are linked
statically against musl (`aarch64-unknown-linux-musl`), so one file needs no system libraries on a Linux of that
architecture; it still needs a minimum kernel and processor baseline, fixed when each platform is validated;
development builds may use the GNU target. Then Linux x86-64 and macOS ARM64; Windows after them.
A platform counts as supported only once its tests pass on it; until then the installer refuses it with a message
(AD-7). Building for another target needs that target's standard library, which this machine was not checked for.

**Dependencies.** Rules:

- Each runtime crate has a recorded reason: what it does, why the standard library is not enough, its licence, and
  what it pulls in. A new crate is a maestro's decision, never an agent's (6.2, 6.38).
- `Cargo.lock` is committed and every build uses `--locked`; the licences of the whole tree are checked against an
  allowlist that fits AD-9, and known advisories are checked, with a tool chosen in phase 6 (`cargo-deny` is the
  candidate; absent on this machine).
- A crate with a build script or a procedural macro runs code at build time; it needs its own line in the reason.
- No async runtime, no HTTP client, no database, no cryptographic signature crate in the core: the core has no service,
  no port and no network (D-032 points 13 and 15), the registry is files (AD-1), and signatures are made and checked by
  OpenSSH (below).

First set, proposed (versions unchecked, no network):

| Crate | Does | Why not the standard library |
| --- | --- | --- |
| `serde` with its derive | Types of events, settings and `--json` output | Rust's standard library has no serialization; derive avoids hand-written readers that drift (a procedural macro, so it needs its line) |
| `serde_json` | Event files and machine output | No JSON in the standard library |
| `toml` | Reading the settings (AD-2) | No TOML in the standard library; Fabrewk never writes TOML, so no editing crate |
| `sha2` | Digests of plans, events, policies and texts | No hashing beyond hash tables in the standard library |

A command-line parser is left to phase 6, with AD-6's single declaration of commands: `clap` gives help and shell
completion for free but brings its own crates; `lexopt` is small and leaves help and completion to Fabrewk's
declaration; a parser by hand is a third choice. Development-only crates (for example `proptest`, for property tests)
do not ship and are not in the runtime budget.

**External tools the core calls.** `git` for every Git operation, instead of a Git library: it behaves as the person's
own Git (hooks, configuration, signing) and adds no crate. `ssh-keygen` for every signature (AD-14), as Git itself does
for SSH signatures (not re-checked). Both are detected as present at start (D-015), with their versions; their absence
refuses the commands that need them, with the next step.

**Tests.** `cargo test`; the past defects (`past-defect-cases.md`) become tests in phase 6; property tests where a pair
or a replay exists (an event written and read back, a replay that equals the state); mutation testing with
`cargo-mutants` (plan row 7, "mutations caught"; absent on this machine). Every test runs in a clean clone.

**Names.** The crate and the binary are `fabrewk` (D-001). Whether the name is free on crates.io is unchecked and is
checked before publication (phase 10).

**The maintainer decides:** the MSRV rule and the edition; the first platforms and static linking on Linux; the
dependency rules and the four first crates (the parser in phase 6); `git` and `ssh-keygen` as external tools.

---

## AD-5. Core boundary, the extension contract and the size budget

**What it settles.** What belongs to the core, what is built into the same program as a separate module, what is an
extension (6.2, 6.49), the contract an extension follows, and how much code each part may have. D-032 points 13 to 15
fix that extensions are separate programs in any language that talk to the `fabrewk` command through stable
structured output, that the project settings declare which may run, that none writes the registry, and that there is
no always-on process or open port until the central server exists.

**Three rings.**

- **Core:** the registry (event files, replay, the one declarative table of transitions, compare-and-swap, fork
  detection); digests and input identities; settings (levels, provenance, validation, the authority read of AD-2);
  identity and signature checks (AD-14); the gate of checks; the fermento's separator and the writer of its answer;
  applying approved alambique edits (AD-13); the views (`STATUS.md`, the rules file of AD-12, each fermento's answer);
  the message catalogues' mechanism; the command dispatcher; the extension runner.
- **Built-in modules**, in the same program, each behind a contract the core declares: one per tool (Claude Code and
  Codex first, OpenCode later), with its stream audit; one per confinement backend (bubblewrap on Linux first, Docker
  as the alternative, D-032 point 14); the installer and the assessment (AD-7). The core names no tool, provider or
  model (PB-8); modules depend on the core's contracts, never the reverse.
- **Extensions**, separate programs: optional features that grant no authority, such as semantic search (6.52), the
  following of external sources (6.31) and a project's own tools.

Why tool adapters and confinement are built in and not extensions: they launch agents, apply the confinement and audit
what a reviewer ran. As separate programs each would be a second root of trust running outside the confinement it
should apply, and the core would lose its one-program advantage (6.2). An adapter for a tool the core does not ship
may become an extension later, run inside the same confinement as the agent, once the contract has held in use.

**The extension contract (6.49).**

1. **Name and permission.** Extension `<name>` is the program `fabrewk-<name>`, run as `fabrewk <name> …`, as Git runs
   its subcommands. It runs only if the project settings permit it (`[extensions.<name>]`) and it is present on the
   machine (D-015). A permitted entry may pin the program's digest; a pinned extension whose digest differs is refused.
   An extension cannot take the name of a core command. The effective configuration shows each extension's path,
   version and digest, and every run records them (6.29).
2. **Channel.** The core starts the program with the person's arguments and three variables: `FABREWK_PROTOCOL` (the
   contract's version), `FABREWK_BIN` (the running core) and `FABREWK_ROOT` (the repository). The extension reads
   state by calling `"$FABREWK_BIN" <command> --json`, which only reads and never waits for a writer (AD-6). It writes nothing
   under `.brewstillery/` and asks for no change from inside: a change it wants is a structured request on its standard
   output, and the outer core, which started it and is not confined, validates each request against the contract and
   the authorization in force and applies it, or refuses it with a code. Standard error carries the extension's
   messages to the person. An extension that does not support the protocol version exits with a reserved code. The
   request format, exact codes and the variables' final names are fixed in phase 6.
3. **Authority.** A change an extension requests is recorded with the extension's name and digest as the actor, under
   the authorization of the person or task that ran it; it never carries a person's signature. A person's move
   (`--say`) refuses when called from an extension.
4. **Confinement.** Where a backend exists (Linux first), the extension runs confined: the repository read-only except
   its own area under `.brewstillery/local/` (AD-8), `.brewstillery/data/` read-only, the private folder (AD-15) and
   the person's SSH agent not reachable, no network unless the project settings grant it to that extension (6.38).
   That makes "none writes the registry" an effective control there; on a platform without a backend it is
   cooperative and Fabrewk says so.
5. **Runtime.** An extension in Python, TypeScript or another interpreted language brings its interpreter, which is
   detected as present (6.49); the one-program advantage is the core's.
6. **No service.** An extension runs when called and ends; it opens no port and stays in no background (D-032 point
   15). The central server (6.50) is a later program of its own (AD-15).

**Budget.** The harness's core reached 3,089 of its 3,500 limit for one task, one maintainer and one machine, and its
size count passed (harness-verdicts, "What was read and run"); Pullboard shows how fast agent-built code grows (PB-11).
The budget is set before building, checked by a tool from the first commit of code, and raised only by a maestro's
recorded decision with its reason, never by an agent (6.25).

The harness's figure is in Python, whose counter's unit is not a physical line (`harness-verdicts.md` lists 1,569
physical lines where the counter reported 3,089), so it cannot be carried over to Rust by measurement. **Recommended
unit:** non-blank lines of Rust that are not comments, outside test code (`#[cfg(test)]` modules and `tests/`), counted
by Fabrewk's own tool. **Recommended limits**, estimates for phase 6 to freeze after the first skeleton is measured
with that tool: core 4,000; each tool adapter, its stream audit included, 600; each confinement backend 400; the
installer and the assessment 1,200. Two more measures that Rust makes cheap to check: at most 8 direct runtime crates
and at most 50 crates in the release build's lock file; a stripped release binary of at most 8 MB. Tests, catalogue
texts and extensions are not counted in the core's budget; each extension sets its own when it is proposed.

**The maintainer decides:** the three rings and why adapters and confinement are built in; the six points of the
extension contract; the unit; the limits, or that phase 6 freezes them after the measurement.

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
value. If a later stage adds many objects, groups can come then, under 6.5's rule for an ordered change. A name that is
not a core command runs a permitted extension (AD-5), never the reverse.

**Rules.**

- One declaration lists every command, its arguments, the moves it may cause and its refusals; help, the reference
  documentation and shell completion are generated from it (PB-1, 6.45).
- English verbs, one meaning each, no aliases and no translations; long options in `--kebab-case`; `--help` and
  `--version` everywhere; `--json` for machine output with stable codes; stable exit codes.
- Every refusal prints its code, the reason in the person's language and the next step as a real command. A test fails
  if any message names a command or flag that does not exist (the legacy pilots' `tbh` lesson).
- A person's move takes `--say "<their words>"`, kept verbatim, and is signed with that person's key (AD-14); Fabrewk
  never prints words for the person to copy (FZ2B-C4-02). It is refused when the environment shows an agent's markers
  or an extension's variables; that refusal is cooperative and declared as such (PB-6), the signature is what AD-14
  makes effective.
- Reading never waits for a writer: `status` works during a dispatch (harness observation "No view during a
  dispatch"; 6.47).
- Every command Fabrewk prints works the same in fish and in a POSIX shell, with no shell-specific syntax, and the
  examples are tested in both.

**First set** (provisional names):

| Command | Does | Writes |
| --- | --- | --- |
| `fabrewk status` | The state, what waits for whom | No |
| `fabrewk config` | The effective configuration with the source of each value | No |
| `fabrewk check` | The record replays and its signatures verify; the views and the alambique match it; the settings parse and validate | No |
| `fabrewk onboard` | The guided interview of AD-7: asks, assesses, shows the plan, and applies it only on the person's explicit yes | Only after that yes |
| `fabrewk assess` | Assesses a repository and prints a verdict and a plan with its identity (D-006) | No |
| `fabrewk apply <plan> --say` | Applies an assessed plan the maintainer accepted | Yes |
| `fabrewk propose` | Records a task proposal: goal, requirements, base, dependencies, files, profile (6.28) | Yes |
| `fabrewk authorize <task> --say` | A person authorizes a task or a group of tasks (AD-10) | Yes |
| `fabrewk run <task>` | Runs the authorized plan: attempt recorded first, dispatch, review, checks (6.41) | Yes |
| `fabrewk accept <task> --say` | A person accepts the verified result | Yes |
| `fabrewk reject <task> --say` | A person rejects it, with their reasons | Yes |
| `fabrewk sweep <alias>` | Records a sweep of that maintainer's fermento and writes only its answer, stopping if the separator is not found exactly once (6.7, 6.8) | Yes |
| `fabrewk approve <proposal> --say` | A maestro approves a drafted alambique edit; Fabrewk applies it (AD-13) | Yes |
| `fabrewk discard <proposal> --say` | A maestro discards a proposal | Yes |

**The maintainer decides:** the rules, and the first set as the starting list for phase 6.

---

## AD-7. Installation: the binary, its two routes, the assessment

**What it settles.** How Fabrewk is installed and how it enters a repository. Section 1 and D-033 point 5 ask for two
routes: a script fetched with `curl` from Fabrewk's own GitHub repository that starts a guided onboarding with the
options the maintainer chooses for the repository, or a preset prompt that any of the maintainer's agents can run for
the same onboarding; in both, nothing is written before the maintainer confirms. D-033 adds that this decision says how
the script checks what it downloads. D-006 (section 1's assessment item) fixes that Fabrewk assesses a repository,
writing nothing, before it enters. AD-4 makes the tool one binary per platform.

**Options for distributing the tool.**

| Option | For | Against |
| --- | --- | --- |
| **Release binaries fetched by the script** | One file per platform, no interpreter and no toolchain on the machine; the script stays short | Fabrewk must build, sign and publish a binary per platform; the curl route needs Fabrewk's repository published on GitHub (an external action, phase 10) |
| `cargo install fabrewk` from crates.io | Builds from source; natural for Rust developers | Needs a Rust toolchain and a build on every machine; no onboarding; the name on crates.io is unchecked |
| Distribution packages (Homebrew, AUR, Fedora) | Native updates | One recipe per system to maintain; later, when users ask for them |
| A build from a clone | Nothing published needed; what the maintainer uses until the first release | For developers only |

**Options for checking what the script downloads.**

| Option | Detects | Against |
| --- | --- | --- |
| A checksum file from the same release | A broken download | Whoever can replace the binary can replace the checksum beside it: no protection against a replaced release |
| **A signature checked against a key written in the script, with `ssh-keygen -Y verify`** | A binary replaced after it was signed, unless the script itself was replaced | OpenSSH must be present (it is on Linux and macOS; an optional feature on Windows, not re-checked) |
| `minisign` | Same as the signature | Another tool to install first (IN-6); absent here |
| Sigstore (`cosign`) or the Git host's build attestations (`gh attestation verify`) | That the binary came from Fabrewk's own build service | Needs `cosign` or `gh` and the network to a public log; useful later as an extra check, not as the only one |

**Recommendation.**

1. **Install the binary, outside any repository.** The script is short, readable POSIX `sh`. It detects the system and
   architecture and refuses one that is not supported yet, with the list of those that are (6.46). It downloads the
   pinned release's archive, its checksum file and the checksum file's signature; it verifies the signature with
   `ssh-keygen -Y verify` against the release signers written in the script (AD-14: the keys of the maestros who sign
   releases, also listed in the project settings and the README), then the archive's checksum; on any failure it
   deletes what it downloaded and stops. It has no flag to skip the check. If `curl` or `ssh-keygen` is missing it
   stops and says how to get it; it never installs another tool on its own (IN-6, PP-5). It writes one file,
   `~/.local/bin/fabrewk`, never uses `sudo`, and prints how to add that directory to the path in fish and in a POSIX
   shell if it is not there. It installs the version it names; a newer one comes only from running a newer script.
   The page shows two ways to run it: piping it into `sh`, and downloading it, reading it, then running it; both work
   from fish. It says in one sentence that trusting the release source grants it full authority over what the script
   installs (AP-3).
2. **Guided onboarding, the curl route.** When the script runs inside a Git repository and a terminal is present, it
   ends by offering `fabrewk onboard`, reading the answers from the terminal (`/dev/tty`), because when the script is
   piped into `sh` its standard input is the script itself. The interview lives in the binary, where it is tested, not
   in the script. It asks, in the language the person chooses first: adopt the repository or only review it; whether
   the maintainers will publish `.brewstillery/` (step 6); the maestro's alias (AD-12) and the SSH key they sign with
   (AD-14), showing the exact Git commands to set up signing if they lack it and running none of them; the tools of
   the catalogue to declare. Then it runs the assessment (step 4), shows the verdict and the plan, and applies it only
   on the person's explicit yes in their own words (step 5).
3. **The preset prompt, the agent route.** A fixed prompt in Fabrewk's README for any agent: install the binary with
   the script by downloading, reading and running it (not piped); ask the maintainer the onboarding questions in chat;
   run `fabrewk assess` with the answers as options, which writes nothing; show the maintainer the verdict and the
   plan; and stop there. The plan is applied only when the maintainer runs `fabrewk apply <plan> --say "<their
   words>"` in their own terminal. The command refuses an agent's environment, a cooperative control (AD-6), and the
   act is signed with the maestro's key, which binds it to that key but, as AD-14 says, does not by itself prove that
   an unconfined agent of the same user did not use it; a key that asks for a touch closes that gap. Both routes reach
   the same two commands,
   `assess` and `apply` (plan row 10).
4. **Assess, writing nothing.** `fabrewk assess` reads only what D-006 lists (Git state, root files, an existing
   `AGENTS.md`, the licence, another harness present) plus the side effects other tools leave that would change what
   agents may do: hooks in `.claude/` or `.codex/` settings, automatic updates, telemetry, knowledge syncs that commit
   or push, generated files that pre-approve tools (reference-study section 6, PP-10). No network during the
   assessment. It ends in a verdict, adopt, review only or do not enter, with its reasons, and a plan: every file it
   would create or change, with its exact content or difference, and the plan's identity as a digest of its content
   (IN-1).
5. **Apply only what was accepted.** `fabrewk apply <plan> --say` refuses if the plan changed since the maintainer saw
   it (IN-1); proceeds only on an explicit yes and refuses on no answer (AP-2's sharp edge; IN-3); never overwrites a
   file it did not write: in an existing `AGENTS.md` it adds a delimited reference block (AD-12, IN-4); records the
   digest of every file it wrote, so an uninstall removes only files that still match (IN-3, AP-5). The verdict and
   that receipt are the repository's first record entry (D-006), signed by the first maestro's key (AD-14).
6. **Publication of `.brewstillery/`.** The plan asks whether the maintainers will publish `.brewstillery/`. If not, the
   warning of 6.27 is shown, its consequences stated, the exits offered (a separate repository for `.brewstillery/` as
   the last one), and the maintainers' answer recorded in their own words. The warning's text is phase 10's.
7. **Review mode** writes only its own evidence in a subdirectory of `.brewstillery/` and changes neither code nor the
   project's `AGENTS.md` (section 1). Its directory name is fixed in phase 6 and is reserved (AD-12).

Fabrewk itself installs no hook into an agent's settings, turns on no automatic update, checks for no update by itself
and sends no telemetry, unless the maintainer accepts that exact item in a plan (D-032 point 15: with no central server
there is nothing to call home to). Removing the binary is deleting `~/.local/bin/fabrewk`; removing Fabrewk from a
repository uses the receipt of step 5.

**The maintainer decides:** release binaries fetched by the script, with `cargo install` and a build from a clone as
developer routes; the signature check with `ssh-keygen` and no flag to skip it; the onboarding in the binary, reading
from the terminal; the agent route ending at `assess`, with `apply` run by the maestro; `~/.local/bin` as the place;
the seven steps.

---

## AD-8. Local unversioned area and ignore rules

**What it settles.** Where Fabrewk keeps what must not be versioned, how it is kept out of Git without cluttering the
root, and how each kind of file is classified (6.4: versioned content, local state, conserved evidence and temporary
files; nothing needed to continue or audit lost as cache; no credentials anywhere in these areas, they go in the
private folder (6.51, AD-15); persistent memory that Fabrewk needs is declared, authorized and kept in `.brewstillery/`).

**Where the ignore rule lives.**

| Option | For | Against |
| --- | --- | --- |
| The root `.gitignore` | Usual place | Writes outside `.brewstillery/` and edits a file the project owns (section 2) |
| **`.brewstillery/.gitignore`** | Inside the domain; Git honours ignore files in subdirectories; travels with every clone | One more file in `.brewstillery/` |
| `.git/info/exclude` | Touches no tracked file | Per clone: a fresh clone lacks it and could commit the local area by mistake |

**Recommendation:** `.brewstillery/.gitignore`, written and owned by Fabrewk (its digest is in the installation
receipt), with one rule, `local/`, which matches `.brewstillery/local/` and a `local/` inside any maintainer's space.
`local` becomes a reserved name (AD-12).

**Classification.**

| Kind | Where | Rule |
| --- | --- | --- |
| Versioned | Settings, the alambique, the fermentos and their material, `data/`, `STATUS.md`, the rules file (AD-12), `.brewstillery/.gitignore` | In the product's Git when `.brewstillery/` is published; otherwise only in the working tree, or in `.brewstillery/`'s own repository if the maintainers took that exit (AD-1) |
| Conserved local evidence | `.brewstillery/local/evidence/` | Providers' raw streams and logs that informed a verdict: never deleted automatically; `fabrewk status` lists them; the record holds their digests (harness "In short" 7) |
| Cache | `.brewstillery/local/cache/` | Rebuildable, safe to delete: detection results with their date, a future SQLite index, a future semantic index (6.52) |
| Temporary | `.brewstillery/local/tmp/` | Agents' exported copies and scratch files, deleted when the attempt ends; no machine-wide temporary directory (6.4, the rule drawn from the maintainer's `~/Dev/tmp`) |
| Extensions | `.brewstillery/local/ext/<name>/` | The only place a confined extension may write (AD-5) |
| Lock | `.brewstillery/local/lock` | The single writer's lock |
| A maintainer's private material | `.brewstillery/<alias>/local/` | Material cited from the fermento that must not be versioned (6.9) |

The assessment's plan declares this area before creating it (6.4: Fabrewk declares the memory it needs and the
maintainer authorizes it). A check refuses files that look like credentials anywhere in `.brewstillery/` (patterns
fixed in phase 6). An agent's exported copy inside the repository is confined to that copy (phase 9); a directory
boundary alone is not isolation (6.6).

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
| **MIT OR Apache-2.0** | The recipient chooses either; both keep the credit | The Apache grant, for whoever chooses it | Yes, and closes Apache's GPL-2.0 gap; used by `uv` and Rust (their licence files are on this machine) and common across Rust crates |
| GPL, LGPL, AGPL, MPL | Share source of modified files or works | Varies | No: more than credit |
| 0BSD, MIT-0, Unlicense, CC0 | None | No | No: not even credit |

**Recommendation.** Fabrewk's code and documentation under **MIT OR Apache-2.0**, with a `NOTICE` file naming Fabrewk
and its authors; `Cargo.toml` declares `license = "MIT OR Apache-2.0"`, an SPDX expression (not re-checked). What
Fabrewk writes into a repository it serves (the reference block in entry files, the skeleton of `.brewstillery/`,
settings templates, generated views) is the project's and carries no obligation: those templates are under **MIT-0**,
so a closed-source project owes nothing for them. Further consequences:

- A release binary links its crates statically (AD-4), so it is a copy of each of them: MIT and Apache-2.0 crates
  require their notices to travel with it. Each release ships a file of third-party notices generated from the lock
  file, and the licence allowlist of AD-4 keeps out any crate whose terms ask more than notices.
- The licence grants no right to the name or logo (Apache section 6 says so). The trademark check with a professional
  before commercial use stays (alambique section 1); a short trademark note in the README says the name is not licensed.
- Contributions come in under the same licence they go out under; whether to ask for a sign-off (DCO) is decided when
  Fabrewk accepts outside contributions (phase 10).
- `tickbrew-harness` has no licence and is the maintainer's; any code carried over literally comes in under Fabrewk's
  licence by the maintainer's decision. The default stays "reuse as design" (harness-verdicts, "The three verdicts"),
  and a port from Python to Rust is a rewrite in any case.
- The licence files are created with the first code (phase 7), not before (`AGENTS.md`: no licence yet).

**The maintainer decides:** the licence of the code and documentation; MIT-0 for what Fabrewk writes into served
repositories.

---

## AD-10. How tasks are grouped, and the name of the group

**What it settles.** The alambique leaves to the architecture how tasks are grouped and what the group is called (section
6, introduction); TickBrew's "phase", "gate" and "cut" are out. 6.16 describes the group: an execution plan with steps,
profiles, models and efforts, files per writer and dependencies, authorized once and run within a budget; section 2 says
each maintainer's fermento is the one file swept when it closes.

**Options for the name** (an English identifier; the person sees it in their own language through the catalogue,
"etapa" in Spanish):

| Name | For | Against |
| --- | --- | --- |
| **stage** | The alambique's own word, "etapa"; plain English | Git calls adding to the index "staging"; Fabrewk never uses that sense in its commands, but a reader might |
| batch | No clash | Sounds mechanical; says nothing of a plan or an authorization |
| milestone | Common | GitHub uses it for issues, with another meaning |
| iteration, sprint | Common | Imply fixed time boxes, which Fabrewk does not have |

**Recommendation: `stage`.** A stage is a set of tasks under one authorization, one plan and one budget; it closes with
the sweep of the fermentos. In the first core its tasks run one after another; parallel steps come only after serial
execution and its recovery are shown to work (6.16).

**The maintainer decides:** the name, and that the first core runs a stage's tasks serially.

---

## AD-11. Closing a work session, permanent version

**What it settles.** D-022 left to this phase the permanent form of "a session closes with a clean tree, and the
maintainer's edits go in their own commit with no AI co-author".

**Recommendation.** With AD-1 A and `.brewstillery/` published, or in its own repository (AD-1), Fabrewk commits its
own events and views as they happen, so a clean close becomes a check: a stage does not close while a file Fabrewk owns
is uncommitted. When `.brewstillery/` is neither published nor in its own repository, there is nothing to commit and the
close says once more that the record lives only on this machine. Files the maintainer owns (their fermento input,
settings, their material) are committed by Fabrewk only on the maintainer's word, alone, authored by the maintainer and
with no AI co-author line (6.44, preserving authorship); an approved alambique edit is committed the same way, on the
approving maestro's word (AD-13). The project's own Git rules (6.44) decide whether those commits go to a branch or a
pull request. During the bootstrap, D-022 stays in force.

**The maintainer decides:** this rule as the permanent version of D-022.

---

## AD-12. The domain's layout, entry files and the agents' rules file

**What it settles.** D-032 points 1 to 3: the domain `.brewstillery/`, one alambique and one fermento per maintainer,
the root entry files reduced to a delimited reference to `STATUS.md`, and the rules Fabrewk administers for agents
living inside the domain, "its file name and form fixed by the design". Also the names the domain reserves and the
form of the fermento's separator, which the core must find exactly once (6.7).

**The rules file.** Today the bootstrap's rules are in the root `AGENTS.md`. In production they move into the domain.

| Option | For | Against |
| --- | --- | --- |
| **`.brewstillery/RULES.md`, a generated view** | One authority (6.26): Fabrewk's base rules come from the binary's catalogue, the project's own rules from the registry, where a maestro approved them as AD-13 approves alambique edits; a check compares the file with them, as with `STATUS.md`; a name no tool loads by itself | A project rule cannot be added by editing the file; it goes through a proposal and a maestro |
| `.brewstillery/AGENTS.md`, same content | The standard name | Some tools load every `AGENTS.md` on the path to the directory they work in (Codex does, not re-checked), which would mix it with the root file; two files with one name confuse a person |
| A Markdown file the maestros edit by hand | Simple | A second editable source beside the decisions, which drifts (6.26); nobody edits the alambique directly, and rules deserve no less |
| A section of `settings.toml` | Already protected as authority (AD-2) | Settings hold values; rules are sentences an agent reads |

**Recommendation: `.brewstillery/RULES.md`, generated.** Its reader is the integrator: the agent that works with a
maestro on the project, which starts at `STATUS.md`, and `STATUS.md` names `RULES.md`. A dispatched agent never reads
it: it works on an exported copy without the domain (AD-1) and receives in its packet only the rules its task needs,
taken from the same source. The file is in the project's canonical documentation language (AD-3).

**Root entry files.** `fabrewk apply` writes, or adds to an existing file, a block between two marker lines (an HTML
comment opening and closing it, invisible when rendered) that says the repository is administered or reviewed by
Fabrewk and that an agent starts at `.brewstillery/STATUS.md`; in administer mode, an optional credit line (AD-9).
Nothing else from Fabrewk goes outside the domain. `CLAUDE.md`, when the project has none, is the one line
`@AGENTS.md` (Claude Code's import, as Pullboard writes it, R-016); when it exists, it gets the same block. Entry files
for other tools are written only when the catalogue declares the tool and the maestro accepts that file in a plan. In
review mode Fabrewk touches no entry file (section 1). In this repository, the root `AGENTS.md` keeps the bootstrap's
rules until production (phase 13), when they move to `RULES.md` and the root file becomes the block.

**Reserved names.** Inside `.brewstillery/`, an alias may not be: `ALAMBIQUE.md`, `STATUS.md`, `RULES.md`,
`settings.toml`, `.gitignore`, `data`, `local`, `bootstrap` (only in Fabrewk's own repository, but reserved
everywhere), and the review-mode directory (AD-7, its name in phase 6). Aliases use lowercase letters, digits and
single hyphens, start with a letter and have at most 32 characters (section 2 fixes the characters; the start and the
length are proposed). The list lives in Fabrewk's code, and `layout.reservedNames` in this repository's settings is
then no longer needed; until the core exists it stays as the maestro's record.

**The fermento's separator (6.7).** Fabrewk writes it, from its message catalogue, in the maintainer's interaction
language: a very visible banner, as today, plus one marker line that never changes with the language (an HTML
comment such as `<!-- fabrewk:fermento-separator v1 -->`, final text in phase 6). The core finds the marker line by
reading lines and skipping fenced code blocks, so an example quoted in a fence does not count. Exactly one marker line
outside fences, with the banner around it as written, is the only accepted case; zero, two or a damaged banner stops
the sweep with nothing written (6.7, past-defect cases). This repository's fermento gains the marker line when it is
migrated at production; until then the bootstrap's check (the banner line found once) stays.

**The maintainer decides:** `RULES.md` as a generated view and who reads it; the entry block and `CLAUDE.md`'s form;
the reserved names and the alias rule; the separator's marker line and how it is found.

---

## AD-13. Alambique edits and the state of proposals

**What it settles.** D-032 points 4 and 5: nobody edits the alambique directly, an agent drafts each edit and a maestro
approves it; the state of each proposal (pending, approved, incorporated into the alambique, discarded) is recorded by
Fabrewk with the identity and words of who decides, bound to the exact text, and goes stale when that text changes; the
fermento and the alambique only show it (6.8). Today the maestro does this by hand: the drafted text is in the
fermento's answer and the maestro copies it in their own commit (D-034, D-035).

**Recommendation.**

1. **A sweep records proposals.** `fabrewk sweep <alias>` records, for each point of the fermento it answers, which
   point of the maintainer's input it is (its number above the separator) and that point's exact text (the text and
   its digest), the recommendation, and, when
   there is consensus, the drafted alambique edit: the full new text of each alambique point it changes, the digest of
   the alambique it starts from, and the status key changes it proposes. The fermento's answer is generated from these
   events (6.8). A proposal's identity is the digest of all of that.
2. **A maestro approves or discards.** `fabrewk approve <proposal> --say` and `fabrewk discard <proposal> --say`, signed
   with the maestro's key (AD-14). Approval is refused when the proposal is stale: the point with that number above
   the separator is missing or its text differs from the recorded one (the same words found elsewhere, in the answer
   or inside another point, do not count), the alambique's digest is not the one the edit starts from, or the drafted
   text changed.
   A stale proposal is shown as stale and redrafted at the next sweep; it is never applied.
3. **Fabrewk applies the approved edit.** It replaces exactly the points named in the edit, checks that the result's
   digest is the one the proposal predicted, and records the proposal as incorporated with that digest. Where
   `.brewstillery/` is versioned, the change is committed alone, on the approving maestro's word, with the maestro as
   author (AD-11). A point removed from the fermento by its maintainer withdraws its open proposals at the next sweep.
4. **A check catches a direct edit.** `fabrewk check` compares the alambique with the last incorporated digest. An
   alambique changed outside an approved edit is reported, and a stage does not close and a candidate is not integrated
   until a maestro either approves that difference as an edit, signed, or reverts it. Nobody can be stopped from typing
   in a file; what is effective is that an unapproved text never becomes the approved one in the record.
5. **Status keys.** A key's change ([decidido] and the others) is part of an edit, approved with it (alambique opening).
   The check of point 4 also reports a key that disagrees with the registry, which prevails.

**The maintainer decides:** the five points, which replace the manual copy once the core exists (phase 8).

---

## AD-14. Identity, roles and signatures

**What it settles.** D-032 points 6 to 9: a maintainer's identity is the SSH key they sign with, listed in the project
settings, and the list changes only with the signature of a key already on it; three roles (maestro, supervisor,
contributor), the first core enforcing maestro and contributor, every act recorded with its role from the start; roles
in a section of the project settings; ownership of paths read from the Git host's code-owners file. 6.48 asks the pilot
to prove that editing personal settings or the private folder, or opening a PR, raises no permission, and that a
contributor cannot launch agents.

**The list.** One table per maintainer in the project settings: alias, role, the public key in the form Git's allowed
signers file uses, and, for a supervisor, their area. Fabrewk derives an allowed-signers file from it in the local area
(AD-8), for `ssh-keygen` and for Git; it is a cache, never edited.

**What is signed, and how.**

| Option | For | Against |
| --- | --- | --- |
| **Each person's act is an event with a detached SSH signature** (`ssh-keygen -Y sign`, namespace `fabrewk`) | Works in every publication mode, also when `.brewstillery/` is not committed (AD-1); binds the person's words and the exact action; checked on every replay | One more file per act; the person needs SSH signing set up |
| Signed Git commits only | Git hosts show and can require them | Nothing to sign when `.brewstillery/` is not versioned; a commit binds a tree, not one act and its words |
| No signatures, the alias is the identity | Nothing to set up | An alias is a name anyone can type (6.6) |

**Recommendation: signed events**, verified with `ssh-keygen -Y verify` against the allowed signers taken from the
settings in force for that act (AD-2's authority read). Where `.brewstillery/` is published, Git's SSH-signed commits
are recommended too, and a host's rule that requires signed commits is the extra effective control (6.13).

- **Changing the list.** A change to the maintainers' table takes effect only when a maestro listed in the version in
  force signs an event adopting the new version's digest, in every publication mode; until then Fabrewk keeps using
  the old list and shows the change as pending. That is AD-2's comparison of policies, made specific.
- **The first maestro.** At onboarding the person who applies the plan gives their public key; the first entry of the
  registry (AD-7, step 5) is signed with it and the key becomes the first maestro. This is trust on first use, said in
  the plan.
- **Roles in the first core.** A maestro authorizes, runs, accepts, rejects, approves alambique edits and adopts
  settings changes. A contributor's fermento is swept like any other, and nothing else: `fabrewk run` and every move
  that dispatches an agent require an authorization signed by a maestro (the task's or the stage's, 6.16), and Fabrewk
  refuses to dispatch when the person running it holds no role that may, so a contributor can launch no agent through
  Fabrewk, not even read-only (D-032 point 7 d). What a contributor runs on their own machine outside Fabrewk is beyond any tool's
  reach; it never enters the record as authorized work, because its events would not verify. Every event records the
  role the signer held. A supervisor is recorded as a role now and enforced when a second maintainer needs it.
- **Areas and code owners (D-032 point 9).** Where the project has a code-owners file, a supervisor's area is the paths
  it gives them (each maintainer's table names their account on the Git host); where it has none, the area is in the
  table, or the whole project. Fabrewk keeps no second map. The places hosts look for that file are checked in phase 6
  (not re-checked here).

**What is effective and what is not (6.38).** A signature proves that the key was used, not that the person was at the
keyboard. An interactive agent that runs as the same user, as the integrator does today, can reach the person's SSH
agent and sign. So: confined agents and extensions never see the SSH agent or the keys (AD-5, phase 9); Fabrewk
recommends a key that asks for a touch for every signature (a hardware key, type `ed25519-sk`) or an SSH agent that
asks for confirmation on each use. The effective configuration reports the key's type, which its public key shows,
apart from whether a touch is enforced: a hardware key can be made with touch not required, so enforcement is reported
as unknown unless the signature check itself requires it (OpenSSH's allowed-signers file can, not re-checked); an
agent's confirmation setting cannot be seen and is reported as unknown. Editing a
personal file or the private folder changes no role, because roles are read only from the project settings in force;
the pilot proves it (6.48).

**The maintainer decides:** the table in the project settings; signed events and the namespace; adopting a new list
only by a listed maestro's signature; trust on first use for the first maestro; the first core's roles; areas from the
code-owners file; the recommendation of keys that ask for presence.

---

## AD-15. The private folder and the instance identity

**What it settles.** D-032 point 10 and 6.51: a private folder per machine, outside every repository, for Fabrewk's own
keys and tokens and the machine data detection cannot find; never authority, never the sessions of other tools, never
copied between machines, hidden from agents. D-032 point 17 and 6.26: every registry entry names the instance that
wrote it, and 6.50 asks that an instance be identified by a verifiable identity, not a declared name.

**Where the folder is.**

| System | Place | Reason |
| --- | --- | --- |
| Linux | `$XDG_CONFIG_HOME/fabrewk/`, which is `~/.config/fabrewk/` when the variable is unset | 6.51 |
| macOS | `~/Library/Application Support/fabrewk/` | The user's configuration folder there |
| Windows | `%LOCALAPPDATA%\fabrewk\` | The local, not the roaming, application data: a domain may copy the roaming folder between machines, which 6.51 forbids (not re-checked) |

Fabrewk creates it with access for its owner only and refuses to read it if its permissions are wider, as SSH does with
its keys. Secrets (the central server's token, later) go first in the system's key store and in an owner-only file
when there is none; a headless Linux often has no key store (not re-checked), so the fallback is the usual path there.

**What it holds.** `machine.toml`: machine data that detection cannot find, each value shown in the effective
configuration as "declared" (D-015, AD-2), never as permitted; the instance key (below); later, the central server's
token. Nothing in it is read for permissions, roles or criteria. Preferences common to every project of a machine
stay deferred (6.51).

**The instance identity.**

| Option | For | Against |
| --- | --- | --- |
| **A key pair per machine user, made with `ssh-keygen -t ed25519` at first run; the instance is named by its public key's fingerprint** | A name derived from a key, which becomes verifiable once events are signed with it; the same tool as AD-14; the central server (6.50) can then check signatures without changing any recorded name | A private key on disk, without a passphrase, since it is used unattended |
| A random identifier | Nothing to protect | A declared name, which 6.50 rejects; it would change when the central server arrives |
| The host name | Readable | Declared, not unique, can change |

**Recommendation: the key pair.** From the first core every event records the machine key's fingerprint. An instance
is Fabrewk in one repository on one machine (6.50); since every event lives in one project's registry, the fingerprint
and that project name the instance together, and no event needs to carry the project's own identity. Projects on one
machine share the key, not the instance. Two clones of the same project on one machine count as one instance; telling
them apart, if it is ever needed, is left to the central server's stage. Until events are signed with the instance key,
the fingerprint is a key-based identifier that nothing verifies, and Fabrewk says so; signing every event with it, and
so verifying which instance wrote it, waits for the central server stage, since nothing checks it before then. The key
exists now so the name never changes. Even signed, the instance key would prove only that the event came from a
process able to read that machine's private folder: confined agents cannot (AD-5, 6.38), an unconfined process of the
same user can.

**What the first core leaves ready for the central server (D-032 point 17).** The machine key's fingerprint in every event,
the event schema's version, and the warning that connecting a repository publishes its record (6.27, 6.50).
Nothing listens, nothing calls out (D-032 point 15). The server, when its stage comes, is a separate program in this
repository, built in the same Cargo workspace until it needs its own release cycle (6.50).

**The maintainer decides:** the three places, Windows' local folder included; what the folder holds; the instance key
and its fingerprint in every event; what is left ready for the central server.

---

## Other destinations of phase 5

Findings the study sent to phase 5 that need no separate decision, with where they land:

- Maintainer roles (6.12): decided in D-032 points 6 to 9 and designed in AD-14; the pilot tests that editing a personal
  file or the private folder, or opening a pull request, cannot raise permissions, and that a contributor cannot launch
  agents (6.48).
- Review by a different provider (6.42, PB-7): a profile rule with `off`, `prefer` or `require`; the provider's
  identity is declared by the adapter, not attested, and is recorded as such. Phase 6.
- Following external sources (6.31, 6.32, OT-2): a later stage, outside the first core, as an extension (AD-5); its
  setting's name and behaviour are designed then. A failed lookup is reported as unknown, never as "no change".
- Semantic code search (6.52): a later extension, off by default, its index in the cache area (AD-8), built after the
  pilot (D-032 point 16).
- The person's view lists only the decisions that wait for that person (OT-6): part of `fabrewk status` (AD-6).
