# Core specification (plan phase 6)

Phase 6 deliverable, split into two parts (D-038). Part 6a, written by the integrator in session 12, holds sections 1
to 10: the classification of the alambique, the requirements, the record's format, the states and the transition
table, the commands, the refusals, the semantics of `fabrewk check`, the settings keys and what phase 5 left open.
Part 6b adds the test plan, with every past-defect case, and the acceptance criteria. Codex reviews both parts
together; then the maintainer approves the text (plan row 6). Until that approval nothing here is frozen. After it, a
change to a requirement is a maestro's recorded decision that names the work it affects (6.25).

**How to read it.** "The first core" is what phases 7 to 10 build; "the pilot" is phases 11 and 12. A requirement is
`REQ-<area>-<n>` and says what must hold; it names a mechanism only where a decided AD already fixed it. A refusal code
is `UPPER_SNAKE` and is listed once, in section 7. Event kinds are `kebab-case`. Each requirement cites the alambique
points and the architecture decisions (AD-n, in `architecture-decisions.md`) it serves. Names of commands, keys,
codes, events and files are part of the specification and are approved with it.

**State (session 12).** Part 6a delivered; part 6b pending.

## 0. Inputs

Session 12, at `919886d`: `.brewstillery/ALAMBIQUE.md` (sha256 `f2cac00b…`); `architecture-decisions.md` as decided
(`9e45b1bc…`); `decisions.md` up to D-038; `.brewstillery/settings.toml` (`96635397…`); the observations and "In short"
of `harness-verdicts.md`; the findings of `reference-study.md` sent to phase 6 (PB-1 to PB-4, PB-7, UC-5, AP-7, SQ-5);
the case titles and sections A to C of `past-defect-cases.md` (`b8bacf1c…`), read so that the refusals here already
answer them. Part 6b reads that file whole. No network; nothing run beyond reading and digests.

## 1. Classification of the alambique

The alambique's section 6 asks that every point be classified as indispensable for the code, a later stage, or an
alternative to evaluate, keeping every expectation and justifying every deferral. The classes:

- **I, indispensable:** the first core implements it before the pilot. When part of the point is deferred, the row says
  which part and why.
- **L, later stage:** not in the first core. The row says why, and what the first core leaves ready so that the
  later stage needs no rewrite.
- **A, alternative to evaluate:** the point asks to evaluate an option; the row says when.

Sections 3, 4, 5 and 7 give context and method, not capabilities; they have no requirement. Section 1's intention of
names, organization and domain, and the trademark check, are not code.

### Sections 1 and 2

| Point | Class | Requirements | Deferred part and reason |
| --- | --- | --- | --- |
| §1 Configure a new project | I | REQ-INS-1 to 4, REQ-DOM-1 to 7 | — |
| §1 Adopt a cloned repository, or only review it | I | REQ-INS-2, REQ-INS-5 | Review mode: after the pilot. The pilot exercises administration; review mode reuses the same record, checks and evidence, and its directory name is reserved now (REQ-DOM-3). Until then `assess` can still conclude "review only" and `apply` refuses that plan (`MODE_NOT_AVAILABLE`). |
| §1 Assess before writing [decidido] | I | REQ-INS-1, REQ-INS-2 | — |
| §1 Install by script or by an agent's prompt | I | REQ-INS-6, REQ-INS-7 | Built in phase 10, inside the first core. |
| §1 Licence [decidido] | I | REQ-BUD-5 | Licence files come with the first code (phase 7). |
| §2 Structure in each repository [decidido] | I | REQ-DOM-1 to 7 | — |

### Section 6

| Point | Class | Requirements | Deferred part and reason |
| --- | --- | --- | --- |
| 6.1 Development apart from the product's operation | I | REQ-SEC-1, REQ-SEC-2, REQ-RUN-8 | Actions on real environments (deployments, production data): after the pilot. The first core grants none: agents and checks run confined, with no product credential. |
| 6.2 Small, independent core in Rust [decidido] | I | REQ-BUD-1 to 5 | — |
| 6.3 Own domain in each repository | I | REQ-DOM-1, REQ-DOM-2 | — |
| 6.4 Internal areas and file classes | I | REQ-DOM-5, REQ-SEC-3 | — |
| 6.5 Command name [decidido] | I | REQ-CLI-1 to 4 | Validation on macOS and Windows follows 6.46's order. |
| 6.6 Space per maintainer | I | REQ-DOM-3, REQ-ID-1 | — |
| 6.7 No configuration in the alambique or fermentos; the separator | I | REQ-FER-1, REQ-FER-2, REQ-SET-1 | — |
| 6.8 Answers in the fermento; proposal state | I | REQ-FER-3 to 8 | — |
| 6.9 Support material | I | REQ-FER-9, REQ-DOM-5 | Rights of use and provenance of material are the maintainer's; the first core only keeps unversioned material in `<alias>/local/` and reads what is cited. |
| 6.10 Separate TOML settings [decidido] | I | REQ-SET-1, REQ-SET-2 | — |
| 6.11 Preferences and policy; effective configuration [decidido] | I | REQ-SET-3 to 8 | Agents changing their own settings at a maestro's request: after the pilot; no adapter offers it yet in a way Fabrewk can check. |
| 6.12 Three roles [decidido] | I | REQ-ID-1 to 6 | Supervisor enforcement: when a second maintainer needs it (decided). Until then a policy that lists a supervisor is refused at adoption (`ROLE_NOT_ENFORCED`), so a role is never shown as enforced when it is not. |
| 6.13 Protecting what grants authority | I | REQ-SET-4, REQ-SET-5, REQ-GATE-9, REQ-SEC-5 | Path ownership from the code-owners file: with the supervisor. |
| 6.14 Catalogue of tools, providers and models | I | REQ-SET-9 | — |
| 6.15 Profiles | I | REQ-SET-9, REQ-RUN-2 | Profiles shared between projects: after the pilot; first core profiles live in the project file. |
| 6.16 Stage plan | I | REQ-TASK-3, REQ-TASK-6 | Parallel steps: only after serial execution and its recovery are shown to work (decided, AD-10). |
| 6.17 Product documentation | I | REQ-DOM-6 | Fabrewk's own README and manuals are written in phase 10; generating a served project's documentation is not Fabrewk's job. |
| 6.18 Self-sufficient human communication | I | REQ-CLI-2, REQ-FER-4, REQ-VIEW-1 | — |
| 6.19 References explained in context | I | REQ-CLI-2 | — |
| 6.20 Languages per class [decidido] | I | REQ-CLI-2, REQ-SET-10 | — |
| 6.21 Localization of documentation | L | — | Translation workflow, paths and checks: after the pilot, when there is documentation stable enough to translate. The first core validates the two keys in use (REQ-SET-10). |
| 6.22 Freshness of translations | L | — | With 6.21. |
| 6.23 Ordered incorporation of ideas | I | REQ-FER-3 to 7 | — |
| 6.24 Bounded automation | I | REQ-TASK-2, REQ-TASK-4, REQ-RUN-2 | — |
| 6.25 Stable requirements | I | REQ-TASK-2, REQ-GATE-7 | — |
| 6.26 Single state [decidido] | I | REQ-REG-1 to 12, REQ-VIEW-3 | SQLite index: only if a query needs it (decided, AD-1). |
| 6.27 Persistence and collaboration | I | REQ-REG-8 to 11, REQ-DOM-7 | The warning's text: phase 10. Pushing, pull requests and events of unmerged work on a protected branch: with 6.44's deferred part (section 9, item 1). |
| 6.28 Tasks and transitions controlled by code | I | REQ-TASK-1 to 6, section 4 | — |
| 6.29 Traceability | I | REQ-REG-2, REQ-REG-13, REQ-GATE-3, REQ-GATE-7 | — |
| 6.30 Catalogue of external sources | L | — | A feature for projects that follow sources, not needed to build or pilot the core. During the bootstrap the catalogue is `reference-study.md` section 11; the first core records each crate's reason (REQ-BUD-4), which covers Fabrewk's own dependencies. |
| 6.31 Following sources (`followBuildSources`) | A | — | Evaluated with 6.30, as an extension (AD-5). |
| 6.32 Notices of external changes | L | — | With 6.31. A failed lookup is reported as unknown, never as "no change" (kept from phase 5). |
| 6.33 Selective use of references | L | — | A working rule for agents, already in force (`AGENTS.md`); code support comes with 6.30. |
| 6.34 Orchestration | I | REQ-RUN-1 to 8 | — |
| 6.35 Separation of responsibilities | I | REQ-RUN-8, REQ-GATE-4 | — |
| 6.36 Provider independence | I | REQ-GATE-4, REQ-SEC-1 | OpenCode's adapter: after the pilot (alambique section 4). |
| 6.37 One writer; concurrency | I | REQ-REG-5, REQ-FER-3 | Separate databases and ports for parallel work: with parallel steps (6.16). |
| 6.38 Security, confinement, extensions | I | REQ-SEC-1 to 7, REQ-EXT-1 to 5 | Docker as alternative backend: A, evaluated when a second platform starts. The per-sweep proposal of each project's tools: after the pilot. |
| 6.39 Continuity from `STATUS.md` | I | REQ-VIEW-1 | — |
| 6.40 Consumption | I | REQ-RUN-7, REQ-VIEW-1 | Reports beyond the per-task figures: after the pilot. |
| 6.41 Recovery | I | REQ-RUN-1, REQ-RUN-5, REQ-RUN-6, REQ-GATE-8 | — |
| 6.42 Independent review | I | REQ-GATE-4, REQ-GATE-6 | Specialists beyond one reviewer: after the pilot. |
| 6.43 Effective verification | I | REQ-GATE-1 to 5, REQ-GATE-10 | — |
| 6.44 Integration and delivery | I | REQ-GATE-8, REQ-REG-10, REQ-RUN-4 | Push and pull requests: after the pilot. They are external actions; in the first core the maestro pushes by hand. |
| 6.45 Documentation for people and agents | I | REQ-CLI-1, REQ-VIEW-2 | Fabrewk's manuals: phase 10. |
| 6.46 Portability | I | REQ-BUD-3, REQ-INS-7 | Linux x86-64, macOS ARM64, Windows: each when its tests pass on it (decided, AD-4). Project containers: A, after the pilot. |
| 6.47 Supervision and outside collaboration | I | REQ-VIEW-1, REQ-VIEW-4, REQ-REG-6 | — |
| 6.48 The pilot | I | Part 6b; phases 11 and 12 | Not code: the pilot's specification (phase 11) uses the requirements here. |
| 6.49 Extensions [decidido] | I | REQ-EXT-1 to 5 | Built in phase 9, with confinement, which the contract needs. |
| 6.50 Central server [proyectado] | L | REQ-REG-12 | Decided as a later stage. The first core records the instance in every event and keeps the schema versioned. |
| 6.51 Private folder [decidido] | I | REQ-SEC-4, REQ-REG-12 | Preferences common to a machine's projects: deferred (decided). |
| 6.52 Semantic search [proyectado] | L | — | After the pilot, as an extension (decided). |

Every point classified I has at least one requirement; every deferral has its reason.

## 2. Requirements

### DOM: the domain and its layout

- **REQ-DOM-1.** Every file Fabrewk writes in a repository is under `.brewstillery/`, except the entry block of
  REQ-DOM-6 and, in the publication modes that need it, one line in `.git/info/exclude` (REQ-DOM-7). (§2, 6.3, AD-12)
- **REQ-DOM-2.** The domain holds only: `ALAMBIQUE.md`, `STATUS.md`, `RULES.md`, `settings.toml`, `.gitignore`,
  `data/`, `local/`, one directory per alias, and `review/` in review mode. Fabrewk creates nothing else. `check` reports
  any other entry at the domain's top as a warning (`DOMAIN_UNKNOWN_ENTRY`); it does not delete it. (§2, AD-8, AD-12)
- **REQ-DOM-3.** Reserved names: `ALAMBIQUE.md`, `STATUS.md`, `RULES.md`, `settings.toml`, `.gitignore`, `data`,
  `local`, `review`, `bootstrap`. An alias matches `^[a-z][a-z0-9]*(-[a-z0-9]+)*$`, has at most 32 characters and is
  not reserved (`ALIAS_INVALID`). The list lives in the code; `layout.reservedNames` is not a production key (section
  8). (§2, 6.6, AD-12)
- **REQ-DOM-4.** `.brewstillery/.gitignore` holds exactly the line `local/`. Fabrewk writes it, records its digest in
  the installation receipt, and `check` reports any difference. (AD-8)
- **REQ-DOM-5.** The local area follows AD-8's classification: `local/evidence/` (never deleted automatically, listed
  by `status`, its files' digests in the record), `local/cache/` (rebuildable), `local/tmp/` (one directory per
  attempt, deleted when the attempt ends), `local/ext/<name>/`, `local/lock`. Fabrewk uses no machine-wide temporary
  directory. A maintainer's unversioned material lives in `<alias>/local/`. (6.4, 6.9, AD-8)
- **REQ-DOM-6.** In administer mode, the root `AGENTS.md` receives a block between the lines
  `<!-- fabrewk:entry begin -->` and `<!-- fabrewk:entry end -->`, whose text comes from the catalogue in the
  canonical documentation language and says that the repository is administered by Fabrewk and that an agent starts at
  `.brewstillery/STATUS.md`. In an existing file Fabrewk changes nothing outside the block. A missing `CLAUDE.md` is
  created as the single line `@AGENTS.md`; an existing one gets the same block. Other tools' entry files only when the
  catalogue declares the tool and the maestro accepts the file in a plan. Review mode touches no entry file. (§2, 6.17,
  AD-12)
- **REQ-DOM-7.** The publication mode is one of `published`, `own-repository`, `unpublished`, recorded with the
  maintainers' words in the `publication-decided` event at `apply`, and changed only by a new signed event. In
  `own-repository` and `unpublished`, `apply` adds `/.brewstillery/` to the product's `.git/info/exclude` (declared in
  the plan), and every write command first checks with `git check-ignore` that the domain is ignored by the product's
  repository; if not, it refuses (`DOMAIN_NOT_IGNORED`), since a later commit would publish it. In `own-repository`,
  `.brewstillery/` is a Git repository of its own and Fabrewk commits there. (6.27, AD-1, AD-7 step 6)

### REG: the record

- **REQ-REG-1.** The record is a set of event files under `.brewstillery/data/events/<stream>/`, named
  `<seq>-<digest16>.json`, where `seq` is the event's position in its stream, six digits with leading zeros, and
  `digest16` the first 16 hexadecimal digits of the file's SHA-256. Streams: `project`, `alambique`,
  `fermento-<alias>`, `stage-<id>`, `task-<id>`, `ext-<name>`. A stage, task or proposal identifier is a letter, a
  hyphen and the first 8 hexadecimal digits of the digest of the payload that creates it, which holds a random nonce of
  16 bytes (`s-…`, `t-…`, `p-…`); `check` refuses two subjects with one identifier (`RECORD_ID_COLLISION`). (6.26,
  AD-1)
- **REQ-REG-2.** Every event (schema version 1) has these fields: `schema`; `stream`; `seq`; `prev` (the full digest of
  the stream's previous event, `null` only at `seq` 1); `kind`; `at` (UTC, RFC 3339, informative: order comes from
  `prev`, never from time); `instance` (the fingerprint of the machine key, REQ-REG-12); `fabrewk` (the binary's
  version); `actor` (kind `person`, `agent`, `extension` or `fabrewk`; for a person, alias and role; for an agent, the
  tool, provider and model as requested and as observed; for an extension, its name and digest); `policy` (digest of
  the policy in force that the event was checked against); `say` (a person's words verbatim and their language tag,
  only on person moves); `payload`. (6.26, 6.29, AD-1, AD-14, AD-15)
- **REQ-REG-3.** An event file is canonical JSON: UTF-8, object keys sorted by code point, no insignificant
  whitespace, integers only, the shortest string escapes, one final newline. Any Unicode character, line and paragraph
  separators included, is stored and replayed intact; nothing splits a file into lines. A file not in canonical form is
  refused (`RECORD_NONCANONICAL`). (6.26; PD-04)
- **REQ-REG-4.** A person's event has a detached signature `<file>.sig`, made with `ssh-keygen -Y sign -n fabrewk` over
  the event file's bytes and verified with `ssh-keygen -Y verify` against the allowed signers derived from the policy
  the event cites. The signer must hold, in that policy, the role the event claims, and the role must allow the event's
  kind (section 4). The signature is written before the event file is moved into place. (6.12, AD-14)
- **REQ-REG-5.** One writer per repository: every command that writes holds an operating-system advisory lock on
  `.brewstillery/local/lock` for its whole run; a second writer is refused at once (`LOCK_HELD`), never queued. Each
  append re-reads the stream's head under the lock and fails if it moved (compare-and-swap). A file is written to a
  temporary name starting with `.` in the same directory, synced, renamed, and the directory synced. Readers take no
  lock and skip temporary names. (6.37, 6.41, AD-1)
- **REQ-REG-6.** Replay reads every stream and refuses, each by its code and never with a raw error: an unknown schema
  version (`RECORD_VERSION_UNKNOWN`), a name whose digest does not match the file (`RECORD_DIGEST_MISMATCH`), a
  non-canonical file, a gap or a `prev` that names no event (`RECORD_BROKEN`), two events at one position of a stream
  (`RECORD_FORK`, handed to a person: Fabrewk does not choose), a move the transition table does not allow from the
  replayed state (`RECORD_TRANSITION_INVALID`), a missing or failing signature (`RECORD_SIGNATURE_INVALID`). (6.26,
  6.47; PD-03)
- **REQ-REG-7.** If the domain exists and `data/` does not, `status` says the record is absent and `check` fails
  (`RECORD_ABSENT`); absence is never read as an empty record. A record exists only from `apply`'s first event. (6.27;
  PD-05)
- **REQ-REG-8.** Rollback is detected two ways. Each machine keeps the last head it saw of every stream in
  `local/cache/heads.json`, replaced whole (temporary file, sync, rename); a stream whose head does not descend from the
  mark is refused (`RECORD_ROLLBACK`), and a mark that exists but is empty or invalid is refused (`MARK_INVALID`) and
  only a person removes it, the message naming the file. In `published` and `own-repository`, `check` walks the Git
  history of `data/` on the record branch with replacement objects and grafts disabled: an event file present in any
  ancestor commit and missing or different now is a rollback. (6.26, 6.29; PD-01, PD-02)
- **REQ-REG-9.** Any change to an event's fields raises `schema`. The binary reads every version ever written; a
  migration is a reader or a new event, never a rewrite of a file. (6.27; PD-03)
- **REQ-REG-10.** In `published` and `own-repository`, each write command ends with one commit holding only the files
  Fabrewk owns that the command changed (event files, signatures, views), built through a temporary index so the
  person's staged and unstaged changes are untouched, with one trailer `Fabrewk-Event: <digest>` per event; the branch
  moves by compare-and-swap from the head read at the start. If the head moved meanwhile, the commit is rebuilt once on
  the new head when no file it holds changed there; otherwise the command refuses (`RECORD_COMMIT_RACE`), the events
  stay written and uncommitted, and `check` reports them. In `unpublished` nothing is committed. (6.44, AD-1, AD-11;
  PD-41)
- **REQ-REG-11.** The record branch is the branch checked out in the repository's main working tree; tasks integrate
  into it. With a detached head, write commands refuse (`HEAD_DETACHED`). (6.27, AD-1)
- **REQ-REG-12.** At first run Fabrewk creates the machine key in the private folder (`ssh-keygen -t ed25519`, no
  passphrase) and records its fingerprint in every event; the effective configuration says that nothing verifies it
  yet. (6.26, 6.50, AD-15)
- **REQ-REG-13.** Raw output of tools and providers, their error texts included, never enters an event: it is kept in
  `local/evidence/`, and the event holds a code, the file's digest and the agent's structured result fields. (6.29,
  6.38; PD-28, PD-29)

### SET: settings

- **REQ-SET-1.** Three files, all TOML 1.0 and never rewritten by Fabrewk: the project's `.brewstillery/settings.toml`;
  each maintainer's `<alias>/settings.toml`; the machine's `machine.toml` in the private folder, which only declares
  data. Fabrewk reads no configuration from the alambique or a fermento. (6.7, 6.10, AD-2)
- **REQ-SET-2.** Every key is declared once in the code with its type, the levels allowed to set it, its default, its
  direction (whether a larger value widens or narrows authority) and its description. An unknown key, a wrong type or
  a value outside the allowed set is refused with file, key path and line (`SETTINGS_INVALID`). (6.10, AD-2; UC-5)
- **REQ-SET-3.** A personal file may set only the keys section 8 marks personal; any other key is refused
  (`SETTINGS_KEY_NOT_PERSONAL`). Its `maintainer.alias` equals its directory's name (`ALIAS_MISMATCH`). (6.11)
- **REQ-SET-4.** The policy in force is the content of the project file carried by the last `policy-adopted` event,
  signed by a maestro, in every publication mode. The file on disk is a proposal until adopted: `config` shows its
  differences as pending, and nothing uses them. In `published` and `own-repository`, `adopt` takes the file as
  committed at the record branch's head and refuses an uncommitted edit (`POLICY_UNCOMMITTED`), so the adopted text
  is also in Git's history. Commands that need a policy refuse while none is adopted (`POLICY_ABSENT`). (6.11, 6.13,
  AD-2, AD-14; PB-4)
- **REQ-SET-5.** `adopt` prints every change between the policy in force and the file, each classed as widening,
  narrowing or neutral by its key's direction, and refuses unless every widening is named with `--widen <key>`
  (`WIDENING_UNNAMED`). Adding, removing or changing a maintainer, or a role, is a widening. (6.13, AD-2, AD-14)
- **REQ-SET-6.** Precedence, lowest first: defaults, project, stage plan, task exception. Personal values apply only to
  personal keys and are not changed by a stage or a task. A stage or task value may narrow freely; a widening needs
  the authorization to name it with `--widen` and is shown as a widening. (6.11, 6.16, AD-2)
- **REQ-SET-7.** `fabrewk config [<key>]` prints every value with its source: `default`, `project` with the adopted
  digest, `stage <id>`, `task <id>`, `personal <alias>`, `detected` with how and when, or `declared` (from
  `machine.toml`); pending changes come apart, with their file and line. (6.11, AD-2)
- **REQ-SET-8.** What is present is detected at the start of every command that needs it (`git`, `ssh-keygen`,
  `bwrap`, every catalogued tool, with versions), kept with its date in `local/cache/detected.json` and never written
  to a settings file. Each value says whether it was observed, verified by use, or only declared. The effective set is
  what is both permitted and present; naming something grants nothing. (6.11, 6.14, D-015, AD-2)
- **REQ-SET-9.** The catalogue has three tables, `tools`, `providers` and `models`; profiles name catalogue entries and
  never copy them. Before a run Fabrewk validates the profile: tool permitted and present, provider permitted, model of
  that provider, effort among the model's, required tools present; the model and effort observed are recorded apart
  from those requested. An unsupported permission mode refuses and never falls back to the tool's default. (6.14,
  6.15, AD-2; PR-5, PR-6, CA-6)
- **REQ-SET-10.** Language keys follow AD-3's table; tags are BCP 47; a regional variant falls back to its base and then
  to English. (6.20, AD-3)

### ID: identity and roles

- **REQ-ID-1.** Maintainers are tables `[maintainers.<alias>]` in the project file with `role` (`maestro`,
  `supervisor`, `contributor`), `key` (one public key in the allowed-signers form), and optionally `hostAccount` and,
  for a supervisor, `area`. (6.6, 6.12, AD-14)
- **REQ-ID-2.** Fabrewk derives `local/cache/allowed_signers` from the policy in force, for `ssh-keygen` and Git; it is
  never edited. (AD-14)
- **REQ-ID-3.** In the first core a maestro may make every person move of section 4; a contributor makes none, and its
  fermento is swept like any other. A policy listing a supervisor is refused at adoption (`ROLE_NOT_ENFORCED`) until
  that role is enforced. A move by a key with no role, or with a role that may not make it, is refused
  (`ROLE_FORBIDDEN`). (6.12, 6.48, AD-14)
- **REQ-ID-4.** Every signed event records the signer's alias and role; roles are read only from the policy in force,
  so editing a personal file or the private folder changes none. (6.12, 6.48, AD-14)
- **REQ-ID-5.** `config` reports each maintainer key's type, and whether a touch is enforced as `yes` only when the
  verification itself requires it, otherwise `unknown`. (6.38, AD-14)
- **REQ-ID-6.** The first maestro is whoever applies the installation plan, with the key they give: trust on first
  use, stated in the plan. (AD-7, AD-14)

### TASK: stages, tasks and transitions

- **REQ-TASK-1.** One declarative table in the code holds every state, move, actor, role, guard and refusal of section
  4. Commands, help, the replay's validation and the reference documentation derive from it; a test compares the table
  with section 4. (6.28; PB-1)
- **REQ-TASK-2.** A task holds: identifier, stage, goal, requirements (each with its checkable criterion), base commit,
  dependencies, allowed files (glob patterns), the profiles for building and reviewing, checks, limits (`rounds`,
  `dispatches`, `minutes`, and `tokens` only where the tool reports them) and the delegation language. An authorization
  binds the task's identifier, the digest of these fields and the stream's head; any change to the fields needs a new
  authorization (`TASK_CHANGED`). (6.24, 6.25, 6.28; PD-06; PB-2)
- **REQ-TASK-3.** A stage holds: identifier, goal, its tasks in order, its budget and the checks it approves.
  Authorizing a stage authorizes each of its tasks by binding each task's digest and head. In the first core, tasks run
  one at a time in a repository, and only when their dependencies are integrated (`DEPENDENCY_OPEN`). (6.16, AD-10)
- **REQ-TASK-4.** A person's move takes `--say` with their words: not empty after trimming (`WORDS_MISSING`), not equal
  to any example the catalogue prints (`WORDS_COPIED`), recorded verbatim with a language tag, signed. It is refused
  where the environment shows an agent's or extension's markers (`AGENT_ENVIRONMENT`); that refusal is cooperative and
  says so. Fabrewk never prints words for the person to copy. (6.20, 6.24, AD-6; PD-30 to PD-32)
- **REQ-TASK-5.** A task halts and waits for a person when: rounds, dispatches, minutes or tokens are spent
  (`ROUNDS_EXHAUSTED`, `BUDGET_EXHAUSTED`); two rounds in a row do not lower the confirmed blocking findings
  (`NO_PROGRESS`, with the alternatives of D-003: reduce the scope, change the design, replace a component); a required
  provider is missing (`PROVIDER_MISSING`); an attempt's outcome is unknown (`OUTCOME_UNKNOWN`). Only `authorize` or
  `reject` moves a halted task. (6.41, 6.48, D-003; PD-07)
- **REQ-TASK-6.** A stage closes only by a maestro's signed move, when every task is integrated or rejected, every
  maintainer's fermento was swept after the last of them ended (each sweep records the heads it saw), the alambique
  matches its last approved digest, and every file Fabrewk owns is committed where the mode commits (`STAGE_OPEN_TASKS`,
  `SWEEP_MISSING`, `ALAMBIQUE_UNAPPROVED`, `RECORD_UNCOMMITTED`). (6.16, AD-10, AD-11, AD-13)

### RUN: dispatch, attempts and recovery

- **REQ-RUN-1.** An attempt is recorded (`attempt-started`: role, profile, requested model and effort) before its
  dispatch and ends exactly once (`attempt-ended`: outcome `success`, `failure` or `unknown`, the observed model and
  usage). A second end is refused (`ATTEMPT_ENDED`) and the first kept. (6.41; PD-10)
- **REQ-RUN-2.** Before each dispatch Fabrewk checks: the profile is valid (REQ-SET-9); the required tools are present
  (`TOOL_MISSING`); budget remains (`BUDGET_EXHAUSTED`); and, before any review starts, a revalidation included, the
  round limit (`ROUNDS_EXHAUSTED`). (6.15, 6.24; PD-07)
- **REQ-RUN-3.** The agent works on an exported copy of the base tree in `local/tmp/<attempt>/work`, without `.git` and
  without `.brewstillery`, at any depth; symbolic links are copied as links, never followed, whatever their target.
  Fabrewk computes the change set itself (added, modified and deleted files, executable bit, links), through a
  temporary index, with paths delimited by NUL and the base's ignore rules applied. Every changed path must match the
  task's allowed files by an exact matcher that cannot backtrack (`LANE_VIOLATION`); a path with a `.git` or
  `.brewstillery` component, exact or nested, is refused even when a pattern allows it (`PROTECTED_PATH`). (6.37,
  6.38; PD-15 to PD-22)
- **REQ-RUN-4.** The candidate is a commit whose parent is the task's base, on the branch `fabrewk/<task-id>`. It names
  the agent (tool, provider, model) as co-author and carries the sign-off the project requires (`git.signOff`). (6.29,
  6.44; PD-45)
- **REQ-RUN-5.** Every command that writes first ends, as `unknown`, each attempt left open by a process that no longer
  runs, counts it as spent and halts its task (`OUTCOME_UNKNOWN`); nothing is retried without a person. A result for an
  attempt that a later one replaced is refused (`ATTEMPT_REPLACED`). (6.41; PD-08 to PD-10)
- **REQ-RUN-6.** A tool's error in one task is recorded in that task, with a code and words for the person; other tasks
  go on. (6.41; PD-11)
- **REQ-RUN-7.** Usage (tokens, time) is recorded only as the tool reports it; what is not reported is recorded as
  unknown, never as zero. A token limit on a profile whose tool does not report tokens is refused at authorization
  (`LIMIT_UNOBSERVABLE`). (6.39, 6.40; PD-39)
- **REQ-RUN-8.** The packet an agent receives holds only: the task's goal, requirements, allowed files, the rules its
  role needs (from `RULES.md`'s source), the delegation and code languages, and the output schema of its role. The
  result is validated against that schema by the core: a review's verdict is `accept` or `reject`, its findings have a
  severity (`blocking`, `minor`) and a status (`confirmed`, `suspected`); a reject needs at least one confirmed blocking
  finding (`REVIEW_INVALID`). (6.34, 6.35, 6.42)

### GATE: checks, review, verification and integration

- **REQ-GATE-1.** A candidate's required checks are the union of its profile's checks and those its stage approved.
  Two definitions with one name are refused (`CHECK_DUPLICATE`). (6.43; PD-23, PD-24)
- **REQ-GATE-2.** Each check runs confined on a fresh export of the candidate's tree, with no network unless the check
  declares it and the policy permits it, and with its time limit. Only a check that finished with exit status 0
  counts; failed, timed out, interrupted or unknown does not. (6.43)
- **REQ-GATE-3.** A check result's identity is the digest of: the candidate's tree, the check's definition, the policy
  digest, the versions of the tools the check names, and the platform. It is recomputed whenever the result is used;
  a mismatch makes the result stale (`EVIDENCE_STALE`). (6.29, 6.43; PB-3)
- **REQ-GATE-4.** The reviewer is dispatched after the checks pass, on the same candidate, never by the attempt that
  built it. `review.differentProvider` is `off`, `prefer` or `require`: with `prefer` unmet, the limitation is
  recorded and shown; with `require` unmet, the task halts (`PROVIDER_MISSING`) and only an authorization naming
  `--widen review.differentProvider` lets it continue, the limitation recorded. The provider's identity is what the
  adapter declares, recorded as declared. An accept counts only when the audit of the reviewer's stream shows it
  examined the candidate; a stream the audit cannot read counts as not examined (`REVIEW_UNEXAMINED`). (6.35, 6.36,
  6.42; PB-7; PD-25, PD-26)
- **REQ-GATE-5.** A task is verified when every required check is satisfactory with a current identity and a review
  accept is bound to the same candidate tree and the same requirements digest. (6.43; PB-3)
- **REQ-GATE-6.** A reject moves the task to its next round within the authorization, without asking the person. The
  progress measure is the number of confirmed blocking findings. (6.24, 6.42, D-003)
- **REQ-GATE-7.** Before signing, `accept` prints what it freezes: the candidate, its files, the checks and their
  identities, the review and the requirements. The signed event binds the task's identifier, its head, the candidate
  commit and tree, the requirements digest and the evidence identities, and carries the summary shown. (6.25, 6.29;
  PD-06, PD-37)
- **REQ-GATE-8.** Integration moves the record branch only from the task's base to the candidate, by one
  compare-and-swap (`git update-ref` with the old value), refusing a moved base (`BASE_MOVED`). Its preconditions
  (evidence current, base unmoved, working tree clean at the touched paths) are checked before the signature, again
  just before the move, and again with the branch at the candidate. A checked-out working tree is brought to the
  candidate only if it is clean (`WORKTREE_DIRTY`). A failure records `merge-failed` with its reason and the task stays
  accepted. `run` on an accepted task finishes its merge under the same checks. A task in any waiting state whose
  record branch is already at its candidate can be finished by `accept` or stopped by `reject`, the record saying where
  the branch is; `reject` is refused while a merge is accepted and the branch is at its candidate (`MERGE_IN_PROGRESS`).
  (6.41, 6.44; PD-12 to PD-14, PD-44, PD-46)
- **REQ-GATE-9.** A path in `gate.protectedPaths` (the files that define the checks, by default the project's build
  and test configuration) can be in a task's allowed files only when listed literally, never through a wildcard
  (`PROTECTED_PATH_WILDCARD`), so a change cannot weaken the control that verifies it unless the maestro named it.
  Agents never see `.brewstillery/`, so no task changes the settings. (6.13)
- **REQ-GATE-10.** Every identity is recomputed on the exact final state before the branch moves; nothing integrates
  on an earlier stamp. (6.43; PB-3)

### FER: fermentos, sweeps, proposals and the alambique

- **REQ-FER-1.** The separator is the line `<!-- fabrewk:fermento-separator v1 -->`, exactly, at the start of a line,
  outside every fenced code block (CommonMark fences of backticks or tildes, open and closed by CommonMark's rules),
  found exactly once, with the banner lines around it identical to one of the catalogue's renderings. Zero, two or a
  damaged banner stop the command with nothing written (`SEPARATOR_MISSING`, `SEPARATOR_AMBIGUOUS`,
  `SEPARATOR_DAMAGED`). (6.7, AD-12)
- **REQ-FER-2.** The points of a fermento are the top-level numbered items (`<n>.` at the start of a line, outside
  fences) above the separator; a point's text runs to the next point or heading. (6.8, AD-13)
- **REQ-FER-3.** `fabrewk sweep <alias> --answer <file>` takes the integrator's answer as JSON: for each point, its
  number, the digest of its text, the recommendation (point, what to clarify and how, destination) and, when there is
  consensus, the drafted edit (section 4). Fabrewk refuses an answer whose point is missing or whose digest differs
  (`POINT_CHANGED`); records `sweep-recorded` and one `proposal-recorded` per drafted edit; records
  `proposal-withdrawn` for each open proposal whose point is gone; and rewrites only the text below the separator.
  Where the mode commits, the input must be committed (`INPUT_UNCOMMITTED`). The input's digest is taken before and
  after the write; if it changed, nothing is written (`INPUT_CHANGED`). (6.8, 6.23, 6.37, AD-13)
- **REQ-FER-4.** The answer is generated from the events, in the maintainer's interaction language: the agent's
  recommendations as written, and for each proposal its state and the command a maestro runs to approve or discard it,
  the proposal's identifier only as a secondary reference. (6.8, 6.18)
- **REQ-FER-5.** `approve <proposal>` is a maestro's signed move. It is refused when the proposal is stale: the point
  with its number is missing or its text changed (the same words elsewhere do not count), the alambique's digest is not
  the one the edit starts from, or the drafted text changed (`PROPOSAL_STALE`, `ALAMBIQUE_CHANGED`). Fabrewk applies the
  edit, checks that the result's digest is the one predicted, records `proposal-incorporated` and, where the mode
  commits, commits the alambique alone with the maestro as author. `discard <proposal>` records `proposal-discarded`.
  (6.8, AD-11, AD-13)
- **REQ-FER-6.** An edit replaces whole units: the opening (before `## 1.`), a section (`## <n>.`) or a point
  (`**<n>.<m>.`). A unit not found exactly once refuses (`UNIT_AMBIGUOUS`). Status keys change only inside an edit.
  (AD-13)
- **REQ-FER-7.** `check` compares the alambique with the last incorporated digest (the first is recorded at `apply`;
  in this repository, at the bootstrap's import). A difference is `ALAMBIQUE_UNAPPROVED`: stages do not close and tasks
  are not accepted until a maestro signs `approve alambique` (recording the difference as an edit) or reverts it. (6.8,
  AD-13)
- **REQ-FER-8.** No proposal state is written by hand anywhere; fermento and alambique only show what the record says.
  (6.8)
- **REQ-FER-9.** Fabrewk reads from a maintainer's space only the fermento and the files the fermento or the alambique
  cite. (6.9; alambique, agents' rules)

### VIEW: generated views

- **REQ-VIEW-1.** `STATUS.md` is generated from the replay in the canonical documentation language: the open stage,
  tasks by state, what waits for whom (for each person, only the decisions that wait for them), the last integrated
  work, the record's health, the warning when the mode is `unpublished`, and next steps as real commands. A figure it
  cannot know is shown as unknown; a person's response time counts only while a decision waited for that person; it
  states nothing that later events would make false without regenerating it. (6.39, 6.40, 6.47; OT-6; PD-39, PD-40,
  PD-43)
- **REQ-VIEW-2.** `RULES.md` is generated from the binary's base rules and the project rules incorporated through
  approved proposals targeting it. (6.45, AD-12)
- **REQ-VIEW-3.** Every write command regenerates the views it affects; `check` compares each view with a fresh
  generation (`VIEW_STALE`). (6.26)
- **REQ-VIEW-4.** `status` and `config` take no lock and work while a run is in progress. (6.47, AD-6)

### CLI: commands and messages

- **REQ-CLI-1.** One declaration lists every command, its arguments, the moves it may cause and its refusals; the
  parser, help, shell completions for fish and POSIX shells, and the reference documentation are generated from it.
  (6.5, 6.45, AD-6; PB-1)
- **REQ-CLI-2.** Every human message comes from a catalogue (English and Spanish first; `es-MX` falls back to `es`, then
  `en`); every key has a description; no bare key, unfilled placeholder or raw error reaches a person; an identifier is
  shown only after a sentence that says what it is. A refusal prints its code, the reason and the next step as a real
  command. A test fails when any message names a command or flag that does not exist, or uses syntax that fish or a
  POSIX shell would not run. (6.18, 6.19, 6.20, AD-6; PD-34 to PD-36, PD-38)
- **REQ-CLI-3.** Exit status: 0 done; 1 refused (a code was printed); 2 usage error; 3 `check` found a problem; 4 the
  run stopped waiting for a person; 70 internal error. An extension that does not support the protocol exits with 10.
  (AD-6, AD-5)
- **REQ-CLI-4.** `--json` gives every command's output as a stable document with a `schema` field; the refusal's code
  is in it. (AD-6)

### INS: assessment, installation and onboarding

- **REQ-INS-1.** `fabrewk assess` reads only Git's state, the root files, an existing `AGENTS.md`, the licence, another
  harness's files, and the side effects of other tools (hooks in `.claude/` or `.codex/` settings, automatic updates,
  telemetry, knowledge syncs that commit or push, generated files that pre-approve tools); it uses no network and
  writes nothing, not even the cache. (§1, D-006, AD-7)
- **REQ-INS-2.** It ends in a verdict (`adopt`, `review`, `none`) with its reasons and a plan: every file it would
  create or change with its exact content or difference, the publication mode, the first maestro's alias and key, the
  catalogued tools, and the local area it would declare. The plan's identity is the digest of its content. (§1, 6.4,
  AD-7; IN-1)
- **REQ-INS-3.** `fabrewk apply <plan-digest> --say` reassesses and refuses if the new plan's digest differs
  (`PLAN_CHANGED`); never overwrites a file it did not write; writes the plan; records, signed by the first maestro,
  `assessed` (verdict and plan), `publication-decided` (with the maintainers' words; the warning shown when
  `unpublished`), `policy-adopted` and the installation receipt with the digest of every file written. It refuses a
  verdict of `none` (`VERDICT_NONE`) and, in the first core, of `review` (`MODE_NOT_AVAILABLE`). (§1, 6.27, AD-7)
- **REQ-INS-4.** `fabrewk onboard` asks AD-7's questions in the language the person chooses, reading from the
  terminal, runs the assessment, shows verdict and plan, and applies it only on the person's explicit words; no answer
  is a no. (AD-7; AP-2, IN-3)
- **REQ-INS-5.** Review mode, when built, writes only under `.brewstillery/review/` and changes neither code nor the
  project's `AGENTS.md`. (§1, AD-7)
- **REQ-INS-6.** The installation script follows AD-7 steps 1 to 3: POSIX `sh`, a release verified by `ssh-keygen -Y
  verify` against the signers written in it, no flag to skip the check, one file `~/.local/bin/fabrewk`, no `sudo`,
  nothing else installed; the agent's route ends at `assess`. (§1, AD-7)
- **REQ-INS-7.** A platform not yet supported is refused with the list of those that are (`PLATFORM_UNSUPPORTED`).
  (6.46, AD-4)

### SEC: confinement, credentials and controls

- **REQ-SEC-1.** On Linux every agent and check runs under bubblewrap: system read-only, an empty home, only the tool's
  binary and its own credential, the exported copy writable at `/work`, the repository, `.brewstillery/`, the private
  folder and the SSH agent's socket invisible; network only for an agent's provider, never for a check unless REQ-GATE-2
  allows it. The shared network is a stated residual risk. Each adapter carries its tool's machine rules and checks them
  at every launch (Codex never starts in a directory that holds other repositories). (6.1, 6.38, AD-5)
- **REQ-SEC-2.** Where no backend exists, dispatch refuses (`CONFINEMENT_UNAVAILABLE`). (6.38, 6.46)
- **REQ-SEC-3.** A file that looks like a credential is refused in `.brewstillery/` (by `check`) and in a candidate's
  change set (by `run`), with `CREDENTIAL_SUSPECTED`. Patterns: a private-key header (`-----BEGIN` followed by `PRIVATE
  KEY-----` on the same line, OpenSSH's included); `AKIA` and 16 uppercase letters or digits; `ghp_`, `gho_`, `ghu_`,
  `ghs_`, `ghr_` or `github_pat_` followed by at least 20 token characters; `sk-ant-`; `sk-` followed by at least 20
  token characters; `xox` with `b`, `a`, `p`, `r` or `s` and a hyphen; file names `.env`, `id_rsa`, `id_ecdsa`,
  `id_ed25519` (without `.pub`), and `*.pem`, `*.p12`, `*.pfx`, `*.key`. The match is a heuristic, a cooperative
  control, and Fabrewk says so; the patterns are matched by hand, with no regular-expression crate. (6.4, 6.38, AD-8)
- **REQ-SEC-4.** The private folder is created with access for its owner only, and Fabrewk refuses to read it if its
  permissions are wider (`PRIVATE_FOLDER_PERMISSIONS`). (6.51, AD-15)
- **REQ-SEC-5.** Every control in the transition table and in `check` is labelled `effective` (enforced by code with
  confinement, by a signature or by the Git host) or `cooperative` (an unconfined process of the same user could bypass
  it); `config` lists them. (6.13, 6.38, AD-2; AP-7)
- **REQ-SEC-6.** Fabrewk installs no hook into an agent's settings, turns on no automatic update, checks for no update
  and sends no telemetry unless a plan the maestro accepted names that item. (AD-7)
- **REQ-SEC-7.** Agents' environment markers are declared by each adapter (for Claude Code, `CLAUDECODE`; for Codex, its
  sandbox variables; not re-checked) and, with `FABREWK_EXTENSION`, drive REQ-TASK-4's refusal. (AD-6)

### EXT: extensions

- **REQ-EXT-1.** Extension `<name>` is the program `fabrewk-<name>`, run as `fabrewk <name> …`, only when
  `[extensions.<name>]` permits it and it is present; a pinned `digest` that differs is refused
  (`EXTENSION_DIGEST`); a core command's name cannot be an extension's. (6.49, AD-5)
- **REQ-EXT-2.** The core sets `FABREWK_PROTOCOL=1`, `FABREWK_BIN`, `FABREWK_ROOT` and `FABREWK_EXTENSION=<name>`.
  The extension reads state through `"$FABREWK_BIN" <command> --json`. (AD-5)
- **REQ-EXT-3.** Its standard output carries only requests, one JSON object per line: `{"protocol":1,"request":<kind>,
  …}`. In the first core the kinds are `note` (an informational event in `ext-<name>`) and `propose` (a task proposal,
  which grants nothing). If any line is not a valid request, none is applied (`EXTENSION_OUTPUT_INVALID`). Requests are
  validated and applied by the outer core, with the extension's name and digest as actor, never with a person's
  signature. Standard error goes to the person. (AD-5)
- **REQ-EXT-4.** Confinement as AD-5 point 4; on a platform without a backend extensions refuse to run, as dispatch
  does. (AD-5)
- **REQ-EXT-5.** An extension runs when called and ends; it opens no port. (6.49, AD-5)

### BUD: size, dependencies and build

- **REQ-BUD-1.** Size is counted in non-blank lines of Rust that are not only comment (`//`, `///`, `//!`, and lines
  inside `/* */`), outside `#[cfg(test)]` modules, `tests/`, `benches/` and `examples/`, per ring, by a counter kept in
  the workspace as a development tool. A ring is its directory: the core, each built-in module, the installer. (AD-5)
- **REQ-BUD-2.** AD-5's limits are provisional ceilings (core 4,000; each tool adapter 600; each confinement backend
  400; installer and assessment 1,200; 8 direct runtime crates; 50 crates in the release lock file; a stripped release
  binary of 8 MB). A maestro freezes the budget after measuring phase 7's skeleton, before any feature; raising it later
  is a maestro's recorded decision with its reason. (AD-5, D-038)
- **REQ-BUD-3.** The MSRV is in `Cargo.toml` as `rust-version`; edition 2024; release builds for
  `aarch64-unknown-linux-musl`. (AD-4)
- **REQ-BUD-4.** Every runtime crate has a recorded reason approved by a maestro; `Cargo.lock` is committed; every
  build uses `--locked`; licences and advisories of the whole tree are checked, the licence allowlist being MIT,
  Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Unicode-3.0, Zlib and MIT-0. (AD-4, AD-9)
- **REQ-BUD-5.** `Cargo.toml` declares `license = "MIT OR Apache-2.0"`; `LICENSE-MIT`, `LICENSE-APACHE` and `NOTICE`
  come with the first code; each release ships a third-party notices file generated from the lock file. (AD-9)

## 3. The record's events

Every kind below is a move of section 4 or, where marked, a record that changes no state. Payload fields beyond those
named are fixed in phase 7 with the code and recorded in this file.

| Stream | Kind | Actor | Payload (main fields) |
| --- | --- | --- | --- |
| `project` | `assessed` | person, signed | verdict, reasons, plan and its digest |
| `project` | `publication-decided` | person, signed | mode, whether the warning was shown |
| `project` | `policy-adopted` | person, signed | the project file's content and digest, the widenings named |
| `project` | `installed` | fabrewk | the receipt: every file written and its digest |
| `alambique` | `alambique-baseline` | person, signed | the alambique's digest at `apply` (or at the bootstrap's import) |
| `fermento-<alias>` | `sweep-recorded` | agent | the input's digest, per point its number, text digest and recommendation, the stage and task heads seen |
| `fermento-<alias>` | `proposal-recorded` | agent | identifier, point number and text digest, target (`alambique` or `rules`), base digest, units and their new text, status-key changes, predicted result digest |
| `fermento-<alias>` | `proposal-withdrawn` | fabrewk | identifier, why (point gone) |
| `alambique` | `proposal-incorporated` | person, signed | identifier, resulting digest |
| `fermento-<alias>` | `proposal-discarded` | person, signed | identifier |
| `alambique` | `direct-edit-approved` | person, signed | old and new digests, the difference |
| `stage-<id>` | `stage-proposed` | any | goal, tasks in order, budget, approved checks |
| `stage-<id>` | `stage-authorized` | person, signed | each task's identifier, digest and head, widenings named |
| `stage-<id>` | `stage-closed` | person, signed | the sweeps it relied on |
| `task-<id>` | `task-proposed` | any | the fields of REQ-TASK-2 |
| `task-<id>` | `task-authorized` | person, signed | task digest and head, limits, widenings named |
| `task-<id>` | `run-started` | person, signed | who runs it, the authorization it relies on |
| `task-<id>` | `attempt-started` | fabrewk | role, profile, requested model and effort |
| `task-<id>` | `attempt-ended` | fabrewk | outcome, observed model, usage, evidence digests |
| `task-<id>` | `candidate-recorded` | fabrewk | commit, tree, change set |
| `task-<id>` | `check-recorded` | fabrewk | check name, identity, outcome, evidence digest |
| `task-<id>` | `review-recorded` | fabrewk | verdict, findings, audit result |
| `task-<id>` | `task-verified` | fabrewk | candidate, evidence identities |
| `task-<id>` | `task-halted` | fabrewk | code, reason |
| `task-<id>` | `task-accepted` | person, signed | REQ-GATE-7's fields and summary |
| `task-<id>` | `merge-failed` | fabrewk | precondition that failed, reason |
| `task-<id>` | `task-integrated` | fabrewk | old and new branch heads |
| `task-<id>` | `task-rejected` | person, signed | reasons in their words, where the branch is |
| `ext-<name>` | `extension-note` | extension | text (records nothing else) |

## 4. States and transitions

**Tasks.** States: `proposed`, `authorized`, `running`, `halted`, `verified`, `accepted`, `integrated` (final),
`rejected` (final).

| Move (event) | From | To | Who | Guards (refusal) |
| --- | --- | --- | --- | --- |
| `task-proposed` | — | `proposed` | anyone; grants nothing | stage exists and is open (`STAGE_CLOSED`); fields valid (`TASK_INVALID`); protected paths literal (`PROTECTED_PATH_WILDCARD`) |
| `task-authorized`, or covered by `stage-authorized` | `proposed`, `halted` | `authorized` | maestro, `--say` | role (`ROLE_FORBIDDEN`); words (`WORDS_MISSING`, `WORDS_COPIED`); digest and head current (`TASK_CHANGED`); widenings named (`WIDENING_UNNAMED`); observable limits (`LIMIT_UNOBSERVABLE`) |
| `run-started`, then `attempt-started` | `authorized` | `running` | maestro (signed) | policy adopted (`POLICY_ABSENT`); dependencies integrated (`DEPENDENCY_OPEN`); no other task running (`LOCK_HELD`); REQ-RUN-2; confinement (`CONFINEMENT_UNAVAILABLE`); alambique approved (`ALAMBIQUE_UNAPPROVED`) |
| `attempt-ended`, `candidate-recorded`, `check-recorded`, `review-recorded` | `running` | `running` | fabrewk | one end per attempt (`ATTEMPT_ENDED`); lanes (`LANE_VIOLATION`, `PROTECTED_PATH`); credentials (`CREDENTIAL_SUSPECTED`); review schema (`REVIEW_INVALID`) |
| `task-verified` | `running` | `verified` | fabrewk | REQ-GATE-5 |
| `task-halted` | `authorized`, `running` | `halted` | fabrewk | one of REQ-TASK-5's codes |
| `task-accepted` | `verified`; or any waiting state with the branch at the candidate | `accepted` | maestro, `--say` | REQ-GATE-7; evidence current (`EVIDENCE_STALE`); alambique approved (`ALAMBIQUE_UNAPPROVED`) |
| `task-integrated` | `accepted` | `integrated` | fabrewk | REQ-GATE-8 (`BASE_MOVED`, `WORKTREE_DIRTY`, `EVIDENCE_STALE`) |
| `merge-failed` | `accepted` | `accepted` | fabrewk | — |
| `task-rejected` | `proposed`, `authorized`, `halted`, `verified`, `accepted` | `rejected` | maestro, `--say` | not while accepted with the branch at the candidate (`MERGE_IN_PROGRESS`) |

"Waiting state" is `authorized`, `halted` or `verified`. `run` on an `accepted` task finishes its merge. A `running`
task with no live process is halted by the next write command (REQ-RUN-5).

**Stages.** `proposed` (`stage-proposed`, anyone) → `authorized` (`stage-authorized`, maestro, `--say`) → `closed`
(`stage-closed`, maestro, `--say`, REQ-TASK-6). Tasks may be added to an authorized stage; each needs its own
`task-authorized`.

**Proposals.** `pending` (`proposal-recorded`) → `incorporated` (`proposal-incorporated`, maestro) or `discarded`
(`proposal-discarded`, maestro) or `withdrawn` (`proposal-withdrawn`, at a sweep). `stale` is derived on every replay,
never recorded: a pending proposal whose bound point, alambique digest or drafted text no longer matches.

**Project.** `apply` writes, in order, `assessed`, `publication-decided`, `policy-adopted`, `alambique-baseline` and
`installed`. Later `policy-adopted` and `publication-decided` events need a maestro.

## 5. Commands

The first set of AD-6, with two moves AD-6's list lacked: `adopt` (AD-2 and AD-14 need a maestro's signed adoption of
the project file) and `close` (REQ-TASK-6). Every command takes `--help`; every reading command takes `--json`;
`fabrewk --version` prints the version. A person's move takes `--say "<words>"` and optionally `--say-lang <tag>`
(default: that maintainer's `agentInteraction`).

| Command | Writes | Moves | Main refusals |
| --- | --- | --- | --- |
| `fabrewk status [<id>]` | no | — | `RECORD_ABSENT` (as a statement, exit 0 with the absence shown) |
| `fabrewk config [<key>]` | no | — | `SETTINGS_INVALID`, `KEY_UNKNOWN` |
| `fabrewk check` | no | — | section 6 |
| `fabrewk onboard` | after the person's words | as `apply` | as `assess` and `apply`; `TERMINAL_REQUIRED` |
| `fabrewk assess [--mode adopt\|review] [--publication <mode>] [--alias <alias>] [--key <file>] [--tool <name>]…` | no | — | `NOT_A_REPOSITORY`, `ALIAS_INVALID` |
| `fabrewk apply <plan-digest> --say` | yes | project events | `PLAN_CHANGED`, `VERDICT_NONE`, `MODE_NOT_AVAILABLE`, `AGENT_ENVIRONMENT` |
| `fabrewk adopt --say [--widen <key>]…` | yes | `policy-adopted` | `POLICY_UNCOMMITTED`, `WIDENING_UNNAMED`, `ROLE_NOT_ENFORCED`, `SETTINGS_INVALID` |
| `fabrewk propose <file>` | yes | `stage-proposed`, `task-proposed` | `TASK_INVALID`, `STAGE_CLOSED` |
| `fabrewk authorize <stage\|task> --say [--widen <key>]…` | yes | `stage-authorized`, `task-authorized` | section 4 |
| `fabrewk run <task\|stage>` | yes | `run-started` and the run's events | section 4 |
| `fabrewk accept <task> --say` | yes | `task-accepted`, `task-integrated` | section 4 |
| `fabrewk reject <task> --say` | yes | `task-rejected` | `MERGE_IN_PROGRESS` |
| `fabrewk sweep <alias> --answer <file>` | yes | `sweep-recorded`, `proposal-*` | REQ-FER-1, REQ-FER-3 |
| `fabrewk approve <proposal\|alambique> --say` | yes | `proposal-incorporated`, `direct-edit-approved` | `PROPOSAL_STALE`, `ALAMBIQUE_CHANGED`, `UNIT_AMBIGUOUS` |
| `fabrewk discard <proposal> --say` | yes | `proposal-discarded` | `PROPOSAL_UNKNOWN` |
| `fabrewk close <stage> --say` | yes | `stage-closed` | REQ-TASK-6 |

`propose` and `sweep` read a JSON file the integrator wrote; their schemas are printed by `--help` and published with
the reference documentation. `run <stage>` runs the stage's authorized tasks one after another and stops at the first
that waits for a person. Every write command first applies REQ-RUN-5 and REQ-DOM-7.

## 6. What `fabrewk check` verifies

Read-only; prints every finding with its code and severity, and exits 3 if any is an error.

| # | Verifies | Code | Severity | Control |
| --- | --- | --- | --- | --- |
| C1 | Settings parse and validate; personal files keep to their keys | `SETTINGS_INVALID`, `SETTINGS_KEY_NOT_PERSONAL`, `ALIAS_MISMATCH` | error | effective |
| C2 | A policy is adopted; the file matches it or its change is pending | `POLICY_ABSENT`, `POLICY_PENDING` | error, warning | effective |
| C3 | The record replays: versions, digests, canonical form, chains, forks, transitions, signatures, identifiers | REQ-REG-6's codes, `RECORD_ID_COLLISION` | error | effective |
| C4 | The record is present and has not rolled back | `RECORD_ABSENT`, `RECORD_ROLLBACK`, `MARK_INVALID` | error | effective with Git history; cooperative on one machine |
| C5 | Views equal a fresh generation | `VIEW_STALE` | error | effective |
| C6 | Each fermento's separator | REQ-FER-1's codes | error | effective |
| C7 | The alambique equals its last approved digest | `ALAMBIQUE_UNAPPROVED` | error | effective for the record; nobody can be stopped from typing |
| C8 | The domain's layout, aliases and the ignore file | `DOMAIN_UNKNOWN_ENTRY`, `ALIAS_INVALID`, `GITIGNORE_CHANGED`, `DOMAIN_NOT_IGNORED` | warning, error, error, error | effective |
| C9 | No file looks like a credential | `CREDENTIAL_SUSPECTED` | error | cooperative (heuristic) |
| C10 | Files Fabrewk owns are committed where the mode commits | `RECORD_UNCOMMITTED` | error | effective |
| C11 | The private folder's permissions | `PRIVATE_FOLDER_PERMISSIONS` | error | effective |
| C12 | Every maintainer directory has a fermento and is listed in the policy | `MAINTAINER_UNLISTED`, `FERMENTO_MISSING` | error | effective |

A check that cannot run (a tool missing, Git history unreadable) reports `CHECK_UNKNOWN` as an error; an unknown
result is never a pass (6.43).

## 7. Refusals

Each code has a catalogue entry with its reason and next step; part 6b gives each a test. Codes not described in the
requirements above:

| Code | When | Next step |
| --- | --- | --- |
| `KEY_UNKNOWN` | `config <key>` names no declared key | `fabrewk config` |
| `TERMINAL_REQUIRED` | `onboard` without a terminal | the agent route: `fabrewk assess` |
| `NOT_A_REPOSITORY` | not inside a Git working tree | run it at the repository's root |
| `PROPOSAL_UNKNOWN` | no such proposal | `fabrewk status` |
| `POLICY_PENDING` | the file differs from the adopted policy (a warning) | `fabrewk adopt --say …` |
| `GITIGNORE_CHANGED` | `.brewstillery/.gitignore` differs from what Fabrewk wrote | restore it |
| `MAINTAINER_UNLISTED`, `FERMENTO_MISSING` | a maintainer directory and the policy disagree | `fabrewk adopt` or create the fermento |
| `CHECK_UNKNOWN` | a verification could not run | the missing tool or reason, named |

All codes, in one list: `AGENT_ENVIRONMENT`, `ALAMBIQUE_CHANGED`, `ALAMBIQUE_UNAPPROVED`, `ALIAS_INVALID`,
`ALIAS_MISMATCH`, `ATTEMPT_ENDED`, `ATTEMPT_REPLACED`, `BASE_MOVED`, `BUDGET_EXHAUSTED`, `CHECK_DUPLICATE`,
`CHECK_UNKNOWN`, `CONFINEMENT_UNAVAILABLE`, `CREDENTIAL_SUSPECTED`, `DEPENDENCY_OPEN`, `DOMAIN_NOT_IGNORED`,
`DOMAIN_UNKNOWN_ENTRY`, `EVIDENCE_STALE`, `EXTENSION_DIGEST`, `EXTENSION_OUTPUT_INVALID`, `FERMENTO_MISSING`,
`GITIGNORE_CHANGED`, `HEAD_DETACHED`, `INPUT_CHANGED`, `INPUT_UNCOMMITTED`, `KEY_UNKNOWN`, `LANE_VIOLATION`,
`LIMIT_UNOBSERVABLE`, `LOCK_HELD`, `MAINTAINER_UNLISTED`, `MARK_INVALID`, `MERGE_IN_PROGRESS`, `MODE_NOT_AVAILABLE`,
`NOT_A_REPOSITORY`, `NO_PROGRESS`, `OUTCOME_UNKNOWN`, `PLAN_CHANGED`, `PLATFORM_UNSUPPORTED`, `POINT_CHANGED`,
`POLICY_ABSENT`, `POLICY_PENDING`, `POLICY_UNCOMMITTED`, `PRIVATE_FOLDER_PERMISSIONS`, `PROPOSAL_STALE`,
`PROPOSAL_UNKNOWN`, `PROTECTED_PATH`, `PROTECTED_PATH_WILDCARD`, `PROVIDER_MISSING`, `RECORD_ABSENT`,
`RECORD_BROKEN`, `RECORD_COMMIT_RACE`, `RECORD_DIGEST_MISMATCH`, `RECORD_FORK`, `RECORD_ID_COLLISION`,
`RECORD_NONCANONICAL`, `RECORD_ROLLBACK`, `RECORD_SIGNATURE_INVALID`, `RECORD_TRANSITION_INVALID`,
`RECORD_UNCOMMITTED`, `RECORD_VERSION_UNKNOWN`, `REVIEW_INVALID`, `REVIEW_UNEXAMINED`, `ROLE_FORBIDDEN`,
`ROLE_NOT_ENFORCED`, `ROUNDS_EXHAUSTED`, `SEPARATOR_AMBIGUOUS`, `SEPARATOR_DAMAGED`, `SEPARATOR_MISSING`,
`SETTINGS_INVALID`, `SETTINGS_KEY_NOT_PERSONAL`, `STAGE_CLOSED`, `STAGE_OPEN_TASKS`, `SWEEP_MISSING`, `TASK_CHANGED`,
`TASK_INVALID`, `TERMINAL_REQUIRED`, `TOOL_MISSING`, `UNIT_AMBIGUOUS`, `VERDICT_NONE`, `VIEW_STALE`,
`WIDENING_UNNAMED`, `WORDS_COPIED`, `WORDS_MISSING`, `WORKTREE_DIRTY`.

## 8. Settings keys

Style: `camelCase` inside `[section]` (AD-2). "Dir." is the key's direction: W (a larger or added value widens
authority), N (narrows), — (neutral).

**Project file** (`.brewstillery/settings.toml`):

| Key | Type | Default | Dir. | Meaning |
| --- | --- | --- | --- | --- |
| `project.name` | string | required | — | The project's name |
| `maintainers.<alias>.role` | `maestro`, `supervisor`, `contributor` | required | W | REQ-ID-1 |
| `maintainers.<alias>.key` | allowed-signers line | required | W | REQ-ID-1 |
| `maintainers.<alias>.hostAccount` | string | none | — | Account on the Git host |
| `maintainers.<alias>.area` | list of paths | whole project | W | Supervisor's area |
| `languages.canonicalDocumentationLanguage` | tag | `en` | — | AD-3 |
| `languages.documentationLanguages` | list of tags | the canonical one | — | AD-3 |
| `languages.codeLanguage`, `.evidenceLanguage`, `.delegationLanguage` | tag | the canonical one | — | AD-3 |
| `tools.<name>.adapter` | a built-in adapter (`claude-code`, `codex`) | required | W | Which module drives the tool |
| `tools.<name>.providers` | list of provider names | required | W | Providers the tool may use |
| `providers.<name>.allowed` | boolean | `false` | W | Whether code may be sent to it |
| `models.<name>.provider`, `.id`, `.efforts` | string, string, list | required | W | The provider's model and its efforts |
| `profiles.<name>.kind` | `build`, `review`, `sweep`, `document` | required | — | Task kind |
| `profiles.<name>.tool`, `.model`, `.effort` | names from the catalogue | required | W | REQ-SET-9 |
| `profiles.<name>.checks`, `.requiredTools` | lists | empty | N | Checks run; tools verified at start |
| `profiles.<name>.rounds`, `.dispatches`, `.minutes`, `.tokens` | integers | from `limits` | W | Limits |
| `limits.rounds`, `.dispatches`, `.minutes` | integers | 3, 6, 120 | W | Defaults for profiles |
| `checks.<name>.command` | list of strings | required | — | The command, without a shell |
| `checks.<name>.network`, `.minutes`, `.tools` | boolean, integer, list | `false`, 30, empty | W, W, — | REQ-GATE-2, REQ-GATE-3 |
| `review.differentProvider` | `off`, `prefer`, `require` | `prefer` | N | REQ-GATE-4 (`off` is the widest) |
| `gate.protectedPaths` | list of paths | `Cargo.toml`, `Cargo.lock`, `.github/` and every file a check's command names | N | REQ-GATE-9 |
| `git.signOff` | boolean | `false` | — | REQ-RUN-4 |
| `extensions.<name>.digest`, `.network` | string, boolean | none, `false` | —, W | REQ-EXT-1, AD-5 |

Adding a table under `maintainers`, `tools`, `providers`, `models`, `profiles`, `checks` or `extensions` is a widening;
removing one is a narrowing.

**Personal file** (`.brewstillery/<alias>/settings.toml`): `maintainer.alias`, `languages.agentInteraction`,
`languages.interfaceLanguage`, `environment.shell` (`fish`, `sh`, `bash`, `zsh`; commands printed for the person are
valid in it and in POSIX `sh`).

**Machine file** (`machine.toml`, private folder): `tools.<name>.path` (where a tool is, when detection cannot find
it). Every value is shown as declared.

**This repository.** `project.command`, `project.domain`, `maintainers.aliases` and `layout.reservedNames` are bootstrap
keys: the first core does not declare them, so they are refused there. They are replaced at production (phase 13),
by a maestro's edit of the settings, with `[maintainers.xel]` and its key.

## 9. What phase 5 left to phase 6

| # | Item | Settled here |
| --- | --- | --- |
| 1 | Where the events of unmerged work live when the default branch is protected (AD-1) | Deferred with pushing and pull requests (6.44), after the pilot. The first core commits the record on the local record branch (REQ-REG-10, REQ-REG-11); the question appears only when pushing to a protected branch. The candidate stays AD-1's: a task's events travel on its branch. |
| 2 | Settings key names (AD-2, AD-3) | Section 8 |
| 3 | The command-line parser (AD-4) | Recommended: `lexopt`. AD-6 already requires Fabrewk's own declaration from which help and completions are generated, so `clap`'s would be a second one and would bring several crates against the limit of 8. A new crate is a maestro's decision: asked at the approval of this text. |
| 4 | The tool that checks licences and advisories (AD-4) | Recommended: `cargo-deny`, as a development tool, never shipped, installed in phase 7 with the network permission that phase asks for. |
| 5 | The extension's request format, codes and variables (AD-5) | REQ-EXT-2, REQ-EXT-3, REQ-CLI-3 |
| 6 | The size budget (AD-5) | Moved to phase 7's start (D-038); the counting rule is REQ-BUD-1 |
| 7 | Final command names and arguments (AD-6) | Section 5 |
| 8 | Review mode's directory (AD-7) | `review`, reserved |
| 9 | The publication warning's text (AD-7) | Phase 10 |
| 10 | Credential patterns (AD-8) | REQ-SEC-3 |
| 11 | The separator's marker line (AD-12) | REQ-FER-1 |
| 12 | Where Git hosts look for a code-owners file (AD-14) | Deferred with the supervisor. Known places (GitHub: `.github/`, the root, `docs/`), not re-checked. |
| 13 | Review by a different provider (phase 5's other destinations) | REQ-GATE-4 |

## 10. What this text asks of the maestro

When Codex has reviewed parts 6a and 6b, the maestro approves the text, and with it these choices, which the
architecture did not settle:

1. `lexopt` as the fifth runtime crate (section 9, item 3) and `cargo-deny` as a development tool (item 4).
2. Review mode after the pilot; `apply` refuses a review-only plan until then (§1 row 2).
3. Pushing and pull requests after the pilot (6.44); the maestro pushes by hand.
4. A policy that lists a supervisor refused until that role is enforced (6.12).
5. Two commands beyond AD-6's list: `adopt` and `close` (section 5).
6. The policy in force carried in the `policy-adopted` event, in every mode (REQ-SET-4).
7. `.git/info/exclude` as the one place outside the domain Fabrewk writes, in the two modes that do not publish the
   domain inside the product's repository (REQ-DOM-7).
8. `run` signed by whoever runs it (`run-started`), so Fabrewk can refuse a key with no role that may dispatch (AD-14);
   an integrator agent that runs an authorized task uses the maestro's key, the gap AD-14 already states.
9. No change to the alambique's status keys: no point passes [decidido] while there is no code.

Part 6b receives this file, `past-defect-cases.md` whole, and the list of refusal codes in section 7; it maps every
case to a requirement and a test, and gives every requirement its acceptance criterion.
