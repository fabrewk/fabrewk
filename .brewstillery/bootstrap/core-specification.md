# Core specification (plan phase 6)

Phase 6 deliverable, split into two parts (D-038). Part 6a, written by the integrator in session 12, holds sections 1
to 10: the classification of the alambique, the requirements, the record's format, the states and the transition
table, the commands, the refusals, the semantics of `fabrewk check`, the settings keys and what phase 5 left open.
Part 6b, written in session 13, adds sections 11 to 13: the test plan, with every past-defect case and every refusal,
the acceptance criterion of every requirement, and what each later phase must show; writing the tests found gaps in
part 6a, corrected in place and listed in section 11.1. Codex reviews both parts
together; then the maintainer approves the text (plan row 6). Until that approval nothing here is frozen. After it, a
change to a requirement is a maestro's recorded decision that names the work it affects (6.25).

**How to read it.** "The first core" is what phases 7 to 10 build; "the pilot" is phases 11 and 12. A requirement is
`REQ-<area>-<n>` and says what must hold; it names a mechanism only where a decided AD already fixed it. A refusal code
is `UPPER_SNAKE` and is listed once, in section 7. Event kinds are `kebab-case`. Each requirement cites the alambique
points and the architecture decisions (AD-n, in `architecture-decisions.md`) it serves. Names of commands, keys,
codes, events and files are part of the specification and are approved with it.

**State (session 13).** Parts 6a and 6b delivered; Codex accepted both at its fifth pass (R-021); the maestro
approved the text and every choice of section 10 as recommended (D-040). The text is frozen.

## 0. Inputs

Session 12, at `919886d`: `.brewstillery/ALAMBIQUE.md` (sha256 `f2cac00b…`); `architecture-decisions.md` as decided
(`9e45b1bc…`); `decisions.md` up to D-038; `.brewstillery/settings.toml` (`96635397…`); the observations and "In short"
of `harness-verdicts.md`; the findings of `reference-study.md` sent to phase 6 (PB-1 to PB-4, PB-7, UC-5, AP-7, SQ-5);
the case titles and sections A to C of `past-defect-cases.md` (`b8bacf1c…`), read so that the refusals here already
answer them. No network; nothing run beyond reading and digests.

Session 13, at `8e3f8c2`: this file as part 6a left it (`24c94c05…`) and `past-defect-cases.md` whole (`b8bacf1c…`);
AD-4 and AD-5's passages on tests, crates and the budget in `architecture-decisions.md` (`9e45b1bc…`). No network.

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
| 6.28 Tasks and transitions controlled by code | I | REQ-TASK-1 to 7, section 4 | — |
| 6.29 Traceability | I | REQ-REG-2, REQ-REG-13, REQ-GATE-3, REQ-GATE-7 | — |
| 6.30 Catalogue of external sources | L | — | A feature for projects that follow sources, not needed to build or pilot the core. During the bootstrap the catalogue is `reference-study.md` section 11; the first core records each crate's reason (REQ-BUD-4), which covers Fabrewk's own dependencies. |
| 6.31 Following sources (`followBuildSources`) | A | — | Evaluated with 6.30, as an extension (AD-5). |
| 6.32 Notices of external changes | L | — | With 6.31. A failed lookup is reported as unknown, never as "no change" (kept from phase 5). |
| 6.33 Selective use of references | L | — | A working rule for agents, already in force (`AGENTS.md`); code support comes with 6.30. |
| 6.34 Orchestration | I | REQ-RUN-1 to 8 | — |
| 6.35 Separation of responsibilities | I | REQ-RUN-8, REQ-GATE-4 | — |
| 6.36 Provider independence | I | REQ-GATE-4, REQ-SEC-1 | OpenCode's adapter: after the pilot (alambique section 4). |
| 6.37 One writer; concurrency | I | REQ-REG-5, REQ-FER-3, REQ-TASK-7 | Separate databases and ports for parallel work: with parallel steps (6.16). |
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
| 6.48 The pilot | I | Sections 11 to 13; phases 11 and 12 | Not code: the pilot's specification (phase 11) uses the requirements here. |
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

**The domain, as part 7c2b fixed it (D-051).** At the domain's top a listed name counts only with its kind (the five
files as files, `data/`, `local/` and each alias as directories); a link is never followed and is an unknown entry. A
directory whose name is neither an alias nor reserved is `ALIAS_INVALID`; anything else not listed is
`DOMAIN_UNKNOWN_ENTRY`, `bootstrap/` and, while review mode is not built, `review/` included, so `check` does not fail
in this repository. `.gitignore` must be a file, not a link, holding exactly `local/` and a newline. The publication
mode in force is that of the last `publication-decided`. `DOMAIN_NOT_IGNORED`'s check first requires the domain's
parent to be the root of the product's main working tree (`NOT_A_REPOSITORY` otherwise: a repository found further up
is not the product's), then asks `git check-ignore -q -- .brewstillery` there; a domain the product tracks is not
ignored. The exclude line is `/.brewstillery/`, added once to the file `git rev-parse --git-path info/exclude` names.

### REG: the record

- **REQ-REG-1.** The record is a set of event files under `.brewstillery/data/events/<stream>/`, named
  `<seq>-<digest16>.json`, where `seq` is the event's position in its stream, six digits with leading zeros, and
  `digest16` the first 16 hexadecimal digits of the file's SHA-256. Streams: `project`, `alambique`,
  `rules`, `fermento-<alias>`, `stage-<id>`, `task-<id>`, `ext-<name>`. A stage, task or proposal identifier is a letter, a
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
  the event file's bytes. It is verified without any allowed-signers file, so that a reading command writes nothing
  (D-048): the public key the signature carries must be exactly the key the policy the event cites gives the event's
  alias, and `ssh-keygen -Y check-novalidate -n fabrewk` must accept the signature over the file's bytes. The signer
  must hold, in that policy, the role the event claims, and the role must allow the event's kind (section 4). The
  same holds in the policy in force at the event's place (D-050): in the `project` stream, the stream's own order
  (the events `apply` writes before the first adoption cite it, the first adoption cites itself, a later one the
  adoption before it, any other event the policy in force at its place); in any other stream, the last adoption whose
  seal holds that stream below the event's position, so that a change of a maintainer, their key or their role takes
  effect for every event written after the adoption. The signature is written before the event file is moved into
  place. (6.12, AD-14)
- **REQ-REG-5.** One writer per repository: every command that writes holds an operating-system advisory lock on
  `.brewstillery/local/lock` for its whole run; a second writer is refused at once (`LOCK_HELD`), never queued. Each
  append re-reads the stream's head under the lock and fails if it moved (compare-and-swap; `RECORD_HEAD_MOVED`,
  nothing written, D-047). A file is written to a
  temporary name starting with `.` in the same directory, synced, renamed, and the directory synced. Readers take no
  lock and skip temporary names. (6.37, 6.41, AD-1)
- **REQ-REG-6.** Replay reads every stream and refuses, each by its code and never with a raw error: an unknown schema
  version (`RECORD_VERSION_UNKNOWN`), a name whose digest does not match the file (`RECORD_DIGEST_MISMATCH`), a
  non-canonical file, a gap or a `prev` that names no event (`RECORD_BROKEN`), two events at one position of a stream
  (`RECORD_FORK`, handed to a person: Fabrewk does not choose), a move the transition table does not allow from the
  replayed state (`RECORD_TRANSITION_INVALID`), a missing or failing signature (`RECORD_SIGNATURE_INVALID`). An event
  whose `stream` or `seq` field disagrees with the file's place is `RECORD_BROKEN`. A person's event whose bound
  identifier or head (REQ-TASK-2, REQ-GATE-7) is not its own stream's, or whose `say` is empty after trimming, is
  `RECORD_TRANSITION_INVALID`. (6.26, 6.47; PD-03, PD-06, PD-31)
- **REQ-REG-7.** If the domain exists and `data/` does not, `status` says the record is absent and `check` fails
  (`RECORD_ABSENT`); absence is never read as an empty record. A record exists only from `apply`'s first event. (6.27;
  PD-05)
- **REQ-REG-8.** Rollback is detected two ways. Each machine keeps the last head it saw of every stream in
  `local/cache/heads.json`, replaced whole (temporary file, sync, rename); a stream whose head does not descend from the
  mark is refused (`RECORD_ROLLBACK`), and a mark that exists but is empty or invalid is refused (`MARK_INVALID`) and
  only a person removes it, the message naming the file. Only write commands update the mark. In `published` and
  `own-repository`, `check` walks the whole Git history of `data/` on the record branch, every parent of every commit,
  with replacement objects and grafts disabled: every commit must hold, unchanged, every event file each of its parents
  holds. A commit that lacks an event file, or holds it changed, is a rollback even when a later commit restores it
  exactly. (6.26, 6.29; PD-01, PD-02)
- **REQ-REG-9.** Any change to an event's fields raises `schema`. The binary reads every version ever written; a
  migration is a reader or a new event, never a rewrite of a file. (6.27; PD-03)
- **REQ-REG-10.** In `published` and `own-repository`, each write command ends with one record commit holding only
  the files Fabrewk owns that the command changed (event files, signatures, views), built through a temporary index so
  the person's staged and unstaged changes are untouched, with one trailer `Fabrewk-Event: <digest>` per event; the
  branch moves by compare-and-swap from the head read at the start. Two commands make one commit before it: `approve`
  of an alambique proposal commits the alambique alone, with the maestro as author and no co-author (REQ-FER-5), and
  integration makes its move (REQ-GATE-8). An event a commit depends on is written before that commit
  (`proposal-incorporated` before the alambique's commit, `task-accepted` before the integration move); an event that
  records a commit's outcome is written after it (`task-integrated`, once the move and the checks with the branch at
  the candidate have passed); all go into the command's record commit. If the head moved
  meanwhile, a commit is rebuilt once on the new head when no file it holds changed there; otherwise the command
  refuses (`RECORD_COMMIT_RACE`) and its events stay written and uncommitted. Events left uncommitted, by a race or by a
  command stopped between its commits, are reported by `check` (`RECORD_UNCOMMITTED`), and the next write command first
  commits them in a record commit of their own. Before a record commit, the command replays every stream it appended
  to and regenerates the views; if either fails, nothing is committed and the refusal says why. In `unpublished`
  nothing is committed. (6.44, AD-1, AD-11; PD-41, PD-46)
- **REQ-REG-11.** The record branch is the branch checked out in the repository's main working tree; tasks integrate
  into it. With a detached head, write commands refuse (`HEAD_DETACHED`). (6.27, AD-1)
- **REQ-REG-12.** The first write command on a machine creates the machine key in the private folder (`ssh-keygen -t
  ed25519`, no passphrase); every event records its fingerprint; the effective configuration says that nothing
  verifies it yet, or that there is none yet. Reading commands (`status`, `config`, `check`, `assess`, and `onboard`
  before the person's words) write nothing anywhere, the key, the mark and the caches included: they detect in memory
  and compare with the mark without updating it. (6.26, 6.50, AD-7, AD-15)
- **REQ-REG-13.** Raw output of tools and providers, their error texts included, never enters an event: it is kept in
  `local/evidence/`, and the event holds a code, the file's digest and the agent's structured result fields. Evidence
  is readable and writable by its owner only (directories `0700`, files `0600` on Unix), and every message about a
  failed tool names its evidence file. (6.29, 6.38; PD-28, PD-29)

### SET: settings

- **REQ-SET-1.** Three files, all TOML 1.1 (a superset of TOML 1.0, D-048) and never rewritten by Fabrewk: the project's `.brewstillery/settings.toml`;
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
  `bwrap`, every catalogued tool, with versions), kept by write commands with its date in `local/cache/detected.json`
  (REQ-REG-12) and never written
  to a settings file. Each value says whether it was observed, verified by use, or only declared. The effective set is
  what is both permitted and present; naming something grants nothing. (6.11, 6.14, D-015, AD-2)
- **REQ-SET-9.** The catalogue has three tables, `tools`, `providers` and `models`; profiles name catalogue entries and
  never copy them. Before a run Fabrewk validates the profile: tool permitted and present, provider permitted, model of
  that provider, effort among the model's, required tools present; the model and effort observed are recorded apart
  from those requested. An unsupported permission mode refuses and never falls back to the tool's default. A profile
  invalid for any reason but an absent tool is refused `SETTINGS_INVALID`, naming the profile's key path and its line,
  or the stage or task that set the value; a tool or required tool absent is `TOOL_MISSING` (D-051). (6.14, 6.15,
  AD-2; PR-5, PR-6, CA-6)
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
  with section 4. A row's guards are evaluated in the order it lists them; the first that fails is the refusal reported,
  and nothing is written (`check` reports every finding instead). A command whose move the table does not allow from
  its subject's state is refused (`MOVE_NOT_ALLOWED`), naming the state and the moves that are allowed. (6.28; PB-1)
- **REQ-TASK-2.** A task holds: identifier, stage, goal, requirements (each with its checkable criterion), base commit
  (none for a task with dependencies, REQ-TASK-3), dependencies, allowed files (glob patterns), the profiles for building and reviewing, checks, limits (`rounds`,
  `dispatches`, `minutes`, and `tokens` only where the tool reports them) and the delegation language. An authorization
  binds the task's identifier, the digest of these fields and the stream's head; any change to the fields needs a new
  authorization (`TASK_CHANGED`). (6.24, 6.25, 6.28; PD-06; PB-2)
- **REQ-TASK-3.** A stage holds: identifier, goal, its tasks in order, its budget and the checks it approves.
  Authorizing a stage authorizes each of its tasks by binding each task's digest and head. In the first core, tasks run
  one at a time in a repository, and only when their dependencies are integrated (`DEPENDENCY_OPEN`). A task with
  dependencies is proposed and authorized without a base commit; its authorization binds its dependencies'
  identifiers. Its base is fixed at `run-started` as the record branch's head, whose product tree (REQ-GATE-8) must
  equal the one the last of its dependencies' integrations left (`BASE_MOVED` otherwise), and `run-started` records it.
  (6.16, AD-10)
- **REQ-TASK-4.** A person's move takes `--say` with their words: not empty after trimming (`WORDS_MISSING`), not equal
  to any example the catalogue prints (`WORDS_COPIED`), recorded verbatim with a language tag, signed. It is refused
  where the environment shows an agent's or extension's markers (`AGENT_ENVIRONMENT`); that refusal is cooperative and
  says so. Fabrewk never prints words for the person to copy. A missing `--say`, or one that is not valid UTF-8, is
  `WORDS_MISSING`, not a usage error, so the person reads why. `propose` grants nothing and is no person's move: it
  takes no `--say`, and the words that make a task workable are those of `authorize`. (6.20, 6.24, AD-6; PD-30 to
  PD-32)
- **REQ-TASK-5.** A task halts and waits for a person when: rounds, dispatches, minutes or tokens are spent
  (`ROUNDS_EXHAUSTED`, `BUDGET_EXHAUSTED`); two rounds in a row do not lower the confirmed blocking findings, that is,
  round n's count is not below round n−1's and round n−1's is not below round n−2's (`NO_PROGRESS`, with the
  alternatives of D-003: reduce the scope, change the design, replace a component); a required provider is missing
  (`PROVIDER_MISSING`); an attempt's outcome is unknown (`OUTCOME_UNKNOWN`); a candidate is refused (`LANE_VIOLATION`,
  `PROTECTED_PATH`, `CREDENTIAL_SUSPECTED`); a review is invalid or unexamined (`REVIEW_INVALID`,
  `REVIEW_UNEXAMINED`); a tool fails (`TOOL_FAILED`); confinement is missing or its preflight fails
  (`CONFINEMENT_UNAVAILABLE`). At a round's end `NO_PROGRESS` is evaluated first; the round limit is checked before the
  next dispatch. Only `authorize` or `reject` moves a halted task, and `accept` when the record branch is at its
  candidate (section 4, REQ-GATE-8). (6.41, 6.48, D-003; PD-07, PD-25, PD-44)
- **REQ-TASK-6.** A stage closes only by a maestro's signed move, when every task is integrated or rejected, every
  maintainer's fermento was swept after the last of them ended (each sweep records the heads it saw), the alambique
  matches its last approved digest, and every file Fabrewk owns is committed where the mode commits (`STAGE_OPEN_TASKS`,
  `SWEEP_MISSING`, `ALAMBIQUE_UNAPPROVED`, `RECORD_UNCOMMITTED`). (6.16, AD-10, AD-11, AD-13)
- **REQ-TASK-7.** Allowed files are patterns relative to the repository's root, segments separated by `/`: `*`
  matches any run of characters inside one segment, `?` one character inside a segment, and a segment that is exactly
  `**` matches zero or more whole segments; every other character is literal (no classes, braces or escapes). Empty
  and `.` segments are dropped; a pattern with a `..` segment, a leading `/`, more than 256 bytes or more than 32
  segments is refused (`TASK_INVALID`). The lane guard, at `propose` and again at `authorize`, refuses (`PROTECTED_PATH`)
  a pattern that can match a path whose first component is `.git` or `.brewstillery` under ASCII case folding, or any of
  whose segments other than a bare `**` can match one of those names under ASCII case folding. A bare `**` is not
  refused for what it could reach deeper, because REQ-RUN-3 refuses those components at any depth in the change set.
  The guard decides from the pattern language itself, never from sample paths; the matcher of REQ-RUN-3 implements the
  same language, case-sensitively, in steps bounded by (pattern length + 1) × (path length + 1). (6.3, 6.13, 6.37;
  PD-15, PD-17)

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
  task's allowed files by the matcher of REQ-TASK-7, which cannot backtrack (`LANE_VIOLATION`); a path with a `.git`
  or `.brewstillery` component, exact or nested, compared under ASCII case folding, is refused even when a pattern
  allows it (`PROTECTED_PATH`), a symbolic link at such a name included. A tracked link left untouched is not a change,
  whatever it points to. A new or changed link is a change like a file: its path must match the allowed files, its
  target is stored as written, never followed, and `accept` shows it. A change of the executable bit alone is a change.
  If the export fails, the attempt is recorded as ended with `TOOL_FAILED` before any dispatch and the task halts; no
  raw error reaches the person. (6.37, 6.38; PD-15 to PD-22)
- **REQ-RUN-4.** The candidate is a commit whose parent is the task's base, on the branch `fabrewk/<task-id>`. It names
  the agent (tool, provider, model) as co-author and carries the sign-off the project requires (`git.signOff`). (6.29,
  6.44; PD-45)
- **REQ-RUN-5.** Every command that writes first ends, as `unknown`, each attempt left open by a process that no longer
  runs, counts it as spent and halts its task (`OUTCOME_UNKNOWN`); nothing is retried without a person. A result for an
  attempt that a later one replaced is refused (`ATTEMPT_REPLACED`). (6.41; PD-08 to PD-10)
- **REQ-RUN-6.** A tool's error in one task (`git`, `ssh-keygen`, `bwrap`, an agent's CLI) is recorded in that task as
  `TOOL_FAILED`, with the tool, its exit status, the digest of its raw output (REQ-REG-13) and words for the person;
  other tasks go on and reading commands keep working. (6.41; PD-11, PD-21, PD-36)
- **REQ-RUN-7.** Usage (tokens, time) is recorded only as the tool reports it; what is not reported is recorded as
  unknown, never as zero. A token limit on a profile whose tool does not report tokens is refused at authorization
  (`LIMIT_UNOBSERVABLE`). (6.39, 6.40; PD-39)
- **REQ-RUN-8.** The packet an agent receives holds only: the task's goal, requirements, allowed files, the rules its
  role needs (from `RULES.md`'s source), the delegation and code languages, the output schema of its role and, from the
  second round on, the previous round's confirmed findings and the output of the checks that failed. The
  result is validated against that schema by the core: a review's verdict is `accept` or `reject`, its findings have a
  severity (`blocking`, `minor`) and a status (`confirmed`, `suspected`); a reject needs at least one confirmed blocking
  finding (`REVIEW_INVALID`). (6.34, 6.35, 6.42)
- **REQ-RUN-9.** A revalidation re-runs the required checks and the review on an existing candidate and dispatches no
  builder. `run` starts one on a `verified` task whose evidence is stale and on an `authorized` task that keeps a
  candidate (after `authorize` from `accepted`). It is a round: it counts against the round limit (REQ-RUN-2) and ends
  as a round does, verified or with a reject that starts a building round. (6.41, 6.43; PD-07)

### GATE: checks, review, verification and integration

- **REQ-GATE-1.** A candidate's required checks are the union of its profile's checks and those its stage approved.
  Two definitions with one name are refused (`CHECK_DUPLICATE`). (6.43; PD-23, PD-24)
- **REQ-GATE-2.** Each check runs confined on a fresh export of the candidate's tree, with no network unless the check
  declares it and the policy permits it, and with its time limit. Only a check that finished with exit status 0
  counts; failed, timed out, interrupted or unknown does not. (6.43)
- **REQ-GATE-3.** A check result's identity is the digest of: the candidate's product tree (REQ-GATE-8), the check's definition, the policy
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
- **REQ-GATE-6.** A review's reject, or a required check that does not pass, ends the round and moves the task to its
  next round within the authorization, without asking the person; each check that does not pass counts as one
  confirmed blocking finding, and the reviewer is not dispatched in that round. The progress measure is the number of
  confirmed blocking findings. (6.24, 6.42, D-003; PD-33)
- **REQ-GATE-7.** Before signing, `accept` prints what it freezes: the candidate, its files, the checks and their
  identities, the review and the requirements. The signed event binds the task's identifier, its head, the candidate
  commit and tree, the requirements digest and the evidence identities, and carries the summary shown. The same holds
  for `authorize`: `status <task>` shows verbatim, with the language tag of each text, everything an authorization or
  an acceptance would bind, and the signed event carries the digest of that rendering, so the person reads what they
  freeze before signing it. Fabrewk translates none of it (6.21 is a later stage). (6.25, 6.29; PD-06, PD-37)
- **REQ-GATE-8.** Integration brings the candidate's change set into the record branch. The product tree of a commit
  is its tree outside `.brewstillery/` (in `own-repository` and `unpublished`, its whole tree). The base is unmoved when
  the product tree at the branch's head equals the base's, so Fabrewk's own record commits never move it; otherwise
  integration refuses (`BASE_MOVED`). If the head is the base commit itself, the branch moves to the candidate;
  otherwise Fabrewk makes an integration commit with the head as its only parent and the head's tree with the
  candidate's change set applied, so that its product tree equals the candidate's; it carries the candidate's message,
  co-author and sign-off trailers, and a trailer `Fabrewk-Candidate: <commit>`. Either way the branch moves by one
  compare-and-swap (`git update-ref` with the old value). The branch is at the candidate when its product tree equals
  the candidate's. The preconditions (evidence current; base unmoved, or the branch already at the candidate; working
  tree clean at the touched paths) are checked before the signature, again just before the move, and again with the
  branch at the candidate. Before the move Fabrewk also recomputes the change
  set from the base to the candidate under REQ-RUN-3's rules and refuses a protected path (`PROTECTED_PATH`), a third
  guard independent of the lane guard and the change-set guard. A checked-out working tree is brought to the candidate
  only if it is clean (`WORKTREE_DIRTY`): at every touched path the working tree and the index hold the base's content
  or, after an interrupted move, the candidate's. A failure before the move records `merge-failed` with its reason;
  the product tree, the index and the working tree are untouched, and only the record commit of REQ-REG-10 advances
  the branch. A failure after the move (the working tree not brought to the candidate, or a check with the branch at
  the candidate failing) records `merge-failed` saying that the branch is at the candidate, and nothing is moved back.
  A working tree or index path holding neither the base's nor the candidate's content is never written. Either way the task stays accepted. `run` on an accepted task finishes its merge under the same checks. A task in any waiting state whose
  record branch is already at its candidate can be finished by `accept` or stopped by `reject`, the record saying where
  the branch is; `reject` is refused, whatever the task's state, while a `task-accepted` for its current candidate is
  on record and the branch is at that candidate (`MERGE_IN_PROGRESS`). An accepted task whose integration failed, its
  evidence gone stale included, is revalidated: `authorize` returns it to `authorized` keeping its candidate, and `run`
  then revalidates it (REQ-RUN-9); so may a `verified` task whose revalidation the round limit stops. (6.41, 6.44; PD-12 to PD-14, PD-44, PD-46)
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
  The above is for a proposal whose target is `alambique`. A proposal whose target is `rules` edits the project rules,
  which live in the record, not in a file: numbered rules `R<n>`, each added, replaced or removed whole. Its base digest
  is the digest of the project rules the replay gives; `approve` refuses when it differs or when the point is stale
  (`PROPOSAL_STALE`), never checks or commits the alambique, records `proposal-incorporated` in the stream `rules`
  with the rules' new text, and regenerates `RULES.md` (REQ-VIEW-2). The two targets' digests are independent. (AD-12,
  AD-13)
- **REQ-FER-7.** `check` compares the alambique with the last incorporated digest (the first is recorded at `apply`;
  in this repository, at the bootstrap's import). A difference is `ALAMBIQUE_UNAPPROVED`: stages do not close and tasks
  are not accepted until a maestro signs `approve alambique` (recording the difference as an edit) or reverts it. (6.8,
  AD-13)
- **REQ-FER-8.** No proposal state is written by hand anywhere; fermento and alambique only show what the record says.
  (6.8)
- **REQ-FER-9.** Fabrewk reads from a maintainer's space only its `fermento.md`, its `settings.toml` and the files the
  fermento or the alambique cite. Of the rest it reads names alone, for `check`'s credential scan (REQ-SEC-3), never
  their content. (6.9; alambique, agents' rules)

### VIEW: generated views

- **REQ-VIEW-1.** `STATUS.md` is generated from the replay in the canonical documentation language: the open stage,
  tasks by state, what waits for whom (for each person, only the decisions that wait for them), the last integrated
  work, the record's health, the warning when the mode is `unpublished`, and next steps as real commands. A figure it
  cannot know is shown as unknown; a person's response time counts only while a decision waited for that person; it
  states nothing that later events would make false without regenerating it. (6.39, 6.40, 6.47; OT-6; PD-39, PD-40,
  PD-43)
- **REQ-VIEW-2.** `RULES.md` is generated from the binary's base rules and the project rules that the stream `rules`
  holds, incorporated through approved proposals targeting them (REQ-FER-6). (6.45, AD-12)
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

**The command surface, as part 7d2 fixed it (D-053, D-055).** The declaration lists the commands of each phase only
(in 7d2 `status`, `config` and `check`); every command takes `--help`, the reading commands `--json`, and the program
alone `--help` and `--version [--json]`; anything else (an unknown command or option, a value given to an option, a
second argument, bytes that are not UTF-8, no command) is a usage error, exit 2, whose next step is the command's
help. The person is the maintainer of the policy in force whose key is the public key Git's `user.signingkey` names
in the repository (a `key::` literal, a public key file, or a private key file with its `.pub` beside it); that
maintainer's `languages.interfaceLanguage` chooses the catalogue, else the policy's canonical documentation language,
else `en`; a personal file that does not read is not used for the language. The catalogue is one TOML file per
language in `crates/core/catalogue/` (`en`, the canonical one, holds every table's description in `about`; `es`); a
command appears in it only in a `run` field, whole. A refusal prints its code and the catalogue's reason, where (a
path relative to where the command ran), the core's detail (its technical note: a key path and line, a count; never
a tool's own output) and the next step, with the command to run when there is one. Documents: `fabrewk.status/1`,
`fabrewk.subject/1`, `fabrewk.config/1`, `fabrewk.value/1`, `fabrewk.check/1`, `fabrewk.refused/1`,
`fabrewk.usage/1`, `fabrewk.internal/1`, `fabrewk.version/1`; their fields are pinned by a test. An error no refusal
names (a file that does not read where a command needs it, a panic) is exit 70 with the catalogue's words. Help,
`crates/fabrewk/completions/fabrewk.fish`, `crates/fabrewk/completions/fabrewk.sh` (POSIX `sh` syntax registering
bash's completion, zsh's through `bashcompinit`) and `crates/fabrewk/doc/reference.<lang>.md` are generated from the
declaration and the catalogue, and a test holds each file to a fresh generation. `status` shows the record's health
(it replays, it does not, with each code, or it is absent, which is a statement and exit 0), the policy in force and
whether the file has pending changes, the publication mode, and each stage and task with its state; `status <id>`
takes `s-…`, `t-…`, `stage-s-…` or `task-t-…`, shows the subject's events and the moves the table allows next, and an
identifier the record does not hold is a usage error. `config` refuses `POLICY_ABSENT` when no policy is adopted and
a record's own code when it does not replay; it lists each effective value with its source, the pending changes with
their effect, each maintainer's key (REQ-ID-5), what detection finds, the effective tools and every guard of section
4 and every check of section 6 with its label (REQ-SEC-5).

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
  at every launch (Codex never starts in a directory that holds other repositories). Before a provider's first
  dispatch on a machine, and whenever its tool's version changes, the adapter runs a preflight that calls no model:
  under the confinement and the copy permissions of each role, the tool, its helper programs and its own sandbox start
  and create a temporary file. The tool's release directory is bound whole and read-only. A failed preflight refuses
  the dispatch (`CONFINEMENT_UNAVAILABLE`) and names its evidence file. (6.1, 6.38, AD-5; PD-27)
- **REQ-SEC-2.** Where no backend exists, dispatch refuses (`CONFINEMENT_UNAVAILABLE`). (6.38, 6.46)
- **REQ-SEC-3.** A file that looks like a credential is refused in `.brewstillery/` (by `check`) and in a candidate's
  change set (by `run`), with `CREDENTIAL_SUSPECTED`. In the domain, `check` matches the content of every file but a
  maintainer's uncited material (anything in `<alias>/` other than `fermento.md`, `settings.toml` and the files the
  fermento or the alambique cite), whose names alone it matches, so that AD-8's "anywhere in `.brewstillery/`" and
  6.9's "only what is cited is read" both hold; a file it must read and cannot is `CHECK_UNKNOWN`. Token characters
  are ASCII letters, digits, `_` and `-`. Patterns: a line that is exactly five hyphens, `BEGIN`, a space, optional
  uppercase words each followed by a space, `PRIVATE KEY` and five hyphens (OpenSSH's included); `AKIA` followed by 16
  uppercase letters or digits; `ghp_`, `gho_`, `ghu_`, `ghs_`, `ghr_` or `github_pat_` followed by at least 20 token
  characters; `sk-ant-` followed by at least 20 token characters; `sk-` followed by at least 20 token characters;
  `xoxb-`, `xoxa-`, `xoxp-`, `xoxr-` or `xoxs-` followed by at least 10 token characters; file names `.env`, `id_rsa`,
  `id_ecdsa`, `id_ed25519` (without `.pub`), and `*.pem`, `*.p12`, `*.pfx`, `*.key`. A pattern's description, as in
  this file, is no match. The match is a heuristic, a cooperative control, and Fabrewk says so; the patterns are
  matched by hand, with no regular-expression crate. (6.4, 6.9, 6.38, AD-8; P6-15)
  As part 7d1 fixed it (D-053): a private key's line may end in a carriage return; the token patterns match anywhere
  in a file, with no boundary before the prefix; the domain is walked whole, `local/` included, and a link or any file
  that is not regular is matched by its name only, never followed or read; a maintainer's top directory is any
  directory at the domain's top whose name is an alias; which files the fermento or the alambique cite is given to the
  scan by `check` (part 7d2).
  As part 7d2 fixed it (D-055, D-056): a citation is read lexically, with no Markdown structure: in `ALAMBIQUE.md`
  and in each `<alias>/fermento.md`, every `[`, then on the same line `](`, followed by a destination, cites it,
  wherever it stands (in code, in a title, in another link's text). The destination is `<…>` up to its `>` on the same
  line, or every character up to the first white space (as Unicode defines it) or the `)` that closes it, parentheses
  balanced; a backslash
  before punctuation escapes it. A destination with a scheme, an absolute path, only a fragment, or empty, cites
  nothing; a fragment or a query after the path is dropped; it is resolved from the citing file's directory with `.`
  and `..` read as written. A citing file is read only when it is a regular file in a maintainer's directory that is
  not a link, and as far as it is UTF-8. The cost accepted (D-056): a file a maintainer names in an example is read too.
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
  is a maestro's recorded decision with its reason. The core's ceiling was raised to 7,500 after part 7d1 was
  measured (D-054). (AD-5, D-038)
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
| `project` | `policy-adopted` | person, signed | the project file's content and digest, the seal of every other stream's head (D-050), the widenings named |
| `project` | `installed` | fabrewk | the receipt: every file written and its digest |
| `alambique` | `alambique-baseline` | person, signed | the alambique's digest at `apply` (or at the bootstrap's import) |
| `fermento-<alias>` | `sweep-recorded` | agent | the input's digest, per point its number, text digest and recommendation, the stage and task heads seen |
| `fermento-<alias>` | `proposal-recorded` | agent | identifier, point number and text digest, target (`alambique` or `rules`), base digest, units and their new text, status-key changes, predicted result digest |
| `fermento-<alias>` | `proposal-withdrawn` | fabrewk | identifier, why (point gone) |
| `alambique` or `rules` (its target) | `proposal-incorporated` | person, signed | identifier, resulting digest; for `rules`, the rules' new text |
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

**Schema 1's envelope, as part 7b fixed it (D-047).** Recorded here as this section asks; the payloads beyond the
creating nonce are fixed by the part that first builds each kind.

- Fields: exactly `actor`, `at`, `fabrewk`, `instance`, `kind`, `payload`, `policy`, `prev`, `schema`, `seq`, `stream`,
  and `say` on a person's move only. `schema` is the integer 1; `seq` a positive integer; `prev` 64 lowercase
  hexadecimal digits or `null`; `at` `YYYY-MM-DDTHH:MM:SS`, an optional fraction of up to nine digits, `Z`; `instance`
  and `fabrewk` non-empty strings; `policy` 64 lowercase hexadecimal digits; `payload` an object.
- `actor`: `{"kind":"person","alias","role"}` (alias as REQ-DOM-3's pattern, role `maestro`, `supervisor` or
  `contributor`); `{"kind":"agent","tool","provider","modelRequested","modelObserved"}` (`modelObserved` a string or
  `null`); `{"kind":"extension","name","digest"}`; `{"kind":"fabrewk"}`. A kind whose actor is "any" takes an agent or
  Fabrewk, never a person (PD-30). `say`: `{"text","lang"}`, both strings, `lang` not empty.
- `stage-proposed` and `task-proposed` carry `payload.nonce`, 32 lowercase hexadecimal digits (16 random bytes); the
  identifier is the first 8 hexadecimal digits of the SHA-256 of the canonical payload, without the final newline.
- Codes: an unknown or missing field, a field of the wrong type, an unknown kind or a malformed actor or `say` is
  `RECORD_VERSION_UNKNOWN` (the file is not schema 1 as this binary knows it); `stream` or `seq` unlike the file's
  place, `prev` set at `seq` 1 or missing after it, a stream name that is not valid, or a stage or task stream whose
  identifier is not its creating payload's, is `RECORD_BROKEN`; a kind recorded in a stream that does not record it,
  an actor the kind does not allow, words on a move no person records or a person's move without words, words empty
  after trimming, a stage or task stream that does not begin with its creation or holds a second one, is
  `RECORD_TRANSITION_INVALID`; a signature beside an event no person records is `RECORD_SIGNATURE_INVALID`.
- The mark `local/cache/heads.json` is canonical JSON `{"heads":{"<stream>":{"digest","seq"}},"schema":1}`. A marked
  stream that is gone or shorter than its mark is `RECORD_ROLLBACK`; a mark that is empty, not canonical, of another
  shape, or naming at a position a digest its stream does not hold there, is `MARK_INVALID` (PD-02). In `unpublished`
  a cut is made by deleting files (PD-01), so a rewrite of the same length is told as `MARK_INVALID`; where the
  history is walked, it is `RECORD_ROLLBACK`, and the walk runs before the mark.
- The record commit's message is `fabrewk: record`, a blank line, and one `Fabrewk-Event: <digest>` line per event, the
  full 64-digit digest.
- `policy-adopted`'s payload is exactly `content` (the project file's text), `digest` (its SHA-256), `heads` (the seal:
  for every stream but `project` that holds an event, `{"digest","seq"}` of its head when the adoption was written)
  and `widened` (the key paths named with `--widen`) (parts 7c1 and 7c2a, D-050). A malformed payload is
  `RECORD_VERSION_UNKNOWN`; a digest that is not its content's is `RECORD_BROKEN`; a seal naming a position its stream
  does not reach, or a digest its stream does not hold there, is `RECORD_ROLLBACK`: the seal is a signed mark.
- `publication-decided`'s payload is exactly `mode` (`published`, `own-repository` or `unpublished`) and `warningShown`
  (a boolean: whether the publication warning was shown); the maintainers' words are the move's `say` (part 7c2b,
  D-051). Any other payload is `RECORD_VERSION_UNKNOWN`.
- The stage and task payloads, as part 7d1 fixed them (D-053). `stage-proposed`: exactly `nonce`, `lang` (the tag
  of its texts), `goal`, `budget` (`dispatches`, `minutes`, optionally `tokens`), `checks` (by name, each exactly
  `command`, `network`, `minutes`, `tools`, its defaults filled) and `settings` (the stage's layer: key path to value).
  `task-proposed`: exactly `nonce`, `stage` (its identifier), `lang`, `goal`, `requirements` (each `text` and
  `criterion`), `base` (a commit of 40 or 64 hexadecimal digits, or `null`), `dependencies` (task identifiers),
  `allowed` (patterns), `profiles` (`build`, `review`), `checks` (names), `limits` (`rounds`, `dispatches`,
  `minutes`, optionally `tokens`), `delegationLanguage` and `settings` (the task's layer). A stage's or a task's
  digest is the SHA-256 of its creating payload, canonical, without the final newline; its identifier is that
  digest's first 8 digits. `task-authorized`: exactly `task`, `digest`, `head` (`seq`, `digest`: the stream's head
  just before it), `limits` and `widened`. `stage-authorized`: exactly `stage`, `digest`, `head`, `tasks` (each
  `task`, `digest`, `head`, `limits`) and `widened`. `task-accepted` holds at least `task` and `head`, bound the same
  way; the rest of its payload is phase 9's.
- A payload of one of these kinds missing a field, holding an unknown one or a field of the wrong JSON type is
  `RECORD_VERSION_UNKNOWN`, as schema 1 says, and wins over any failure of content. Content (a blank text, a tag that
  is not one, a count below 1, a malformed identifier or commit, no requirement or allowed file, a pattern REQ-TASK-7
  refuses, a base commit together with dependencies or neither, a dependency named twice, a check with an empty
  command, a layer key that level may not set or a value not of its type) is `TASK_INVALID` at a command and
  `RECORD_TRANSITION_INVALID` on replay. The replay holds every stage and task stream to the transition table: a
  move from a state the table does not allow, a task naming a stage or a dependency the record does not hold, an
  authorization or acceptance whose `task` (or `stage`), `digest` or `head` is not its own stream's, and a stage's
  authorization binding a task of another stage, another digest, or a task that waits for no authorization, are
  `RECORD_TRANSITION_INVALID`; a bound head its task's stream does not hold is `RECORD_ROLLBACK`, as for the seal. A
  stage's authorization makes each task it binds `authorized` right after the head it names. An `attempt-ended`
  ends the last `attempt-started`; one with no open attempt is `ATTEMPT_ENDED` on replay (PD-10). The pattern guards
  (`PROTECTED_PATH`, `PROTECTED_PATH_WILDCARD`), `STAGE_CLOSED`, `CHECK_DUPLICATE` and `LIMIT_UNOBSERVABLE` are
  evaluated by the commands, not by the replay. Proposals are held to the table with the sweep (phase 8), the
  `project` stream's order with `apply` (phase 10).
- A person's event is checked, after the record is read field by field, against the policy it cites and the policy in
  force at its place (REQ-REG-4, D-050); every failure is `RECORD_SIGNATURE_INVALID`: a citation of a digest no
  adoption holds, an event of `project` citing another policy than the one in force at its place, an event outside
  `project` that every seal holds (written before any adoption), an alias either policy does not list, a role other
  than the one either gives, a key that differs between them or from the one the `.sig` carries, a role that may not
  make the move (only a maestro makes a person's move in the first core, REQ-ID-3), and `ssh-keygen -Y
  check-novalidate -n fabrewk` refusing. A write refuses the same move before signing (`ROLE_FORBIDDEN`), and checks
  the signature it made against the signer's own key before the event exists.
- Canonical JSON nests arrays and objects at most 64 deep, the event itself being 1; a deeper value is not written
  (`RECORD_NONCANONICAL`, nothing written) and a deeper file is not canonical. REQ-REG-3's property holds for every
  value within that depth.
- In the modes that commit, a write command runs at the root of the repository's main working tree; from a linked
  working tree or a subdirectory it refuses `NOT_A_REPOSITORY`, since there the domain, its lock and the index are
  another copy.

## 4. States and transitions

**Tasks.** States: `proposed`, `authorized`, `running`, `halted`, `verified`, `accepted`, `integrated` (final),
`rejected` (final).

| Move (event) | From | To | Who | Guards (refusal) |
| --- | --- | --- | --- | --- |
| `task-proposed` | — | `proposed` | anyone; grants nothing | stage exists and is open (`STAGE_CLOSED`); fields and patterns valid (`TASK_INVALID`, REQ-TASK-7); no pattern reaches a protected name (`PROTECTED_PATH`); protected paths literal (`PROTECTED_PATH_WILDCARD`) |
| `task-authorized`, or covered by `stage-authorized` | `proposed`, `halted`; `verified` and `accepted` for a revalidation, keeping the candidate (REQ-GATE-8, REQ-RUN-9) | `authorized` | maestro, `--say` | role (`ROLE_FORBIDDEN`); words (`WORDS_MISSING`, `WORDS_COPIED`); digest and head current (`TASK_CHANGED`); lane guard again (`PROTECTED_PATH`); widenings named (`WIDENING_UNNAMED`); observable limits (`LIMIT_UNOBSERVABLE`) |
| `run-started`, then `attempt-started` | `authorized`; `verified` with stale evidence, as a revalidation (REQ-RUN-9) | `running` | maestro (signed) | policy adopted (`POLICY_ABSENT`); dependencies integrated (`DEPENDENCY_OPEN`); a dependent task's base fixed (`BASE_MOVED`, REQ-TASK-3); no other task running (`LOCK_HELD`); REQ-RUN-2; confinement (`CONFINEMENT_UNAVAILABLE`); alambique approved (`ALAMBIQUE_UNAPPROVED`) |
| `attempt-ended`, `candidate-recorded`, `check-recorded`, `review-recorded` | `running` | `running` | fabrewk | one end per attempt (`ATTEMPT_ENDED`); lanes (`LANE_VIOLATION`, `PROTECTED_PATH`); credentials (`CREDENTIAL_SUSPECTED`); review schema (`REVIEW_INVALID`) |
| `task-verified` | `running` | `verified` | fabrewk | REQ-GATE-5 |
| `task-halted` | `authorized`, `running` | `halted` | fabrewk | one of REQ-TASK-5's codes |
| `task-accepted` | `verified`; or any waiting state with the branch at the candidate | `accepted` | maestro, `--say` | REQ-GATE-7; evidence current (`EVIDENCE_STALE`); alambique approved (`ALAMBIQUE_UNAPPROVED`) |
| `task-integrated` | `accepted` | `integrated` | fabrewk | REQ-GATE-8 (`BASE_MOVED`, `WORKTREE_DIRTY`, `EVIDENCE_STALE`) |
| `merge-failed` | `accepted` | `accepted` | fabrewk | — |
| `task-rejected` | `proposed`, `authorized`, `halted`, `verified`, `accepted` | `rejected` | maestro, `--say` | not while a `task-accepted` for the current candidate is on record and the branch is at it, whatever the state (`MERGE_IN_PROGRESS`) |

**The table, as part 7d1 fixed it (D-053).** The code holds section 4 as one declarative table, in
`crates/core/src/table.rs`, with the stage and proposal moves of the paragraphs below it: a stage is proposed by anyone (guards
`TASK_INVALID`, `CHECK_DUPLICATE`), authorized by a maestro with words (the task authorization's guards and
`CHECK_DUPLICATE` again) and closed by one (REQ-TASK-6's codes); a proposal is recorded by an agent (REQ-FER-3),
incorporated (`PROPOSAL_STALE`, `ALAMBIQUE_CHANGED`, `UNIT_AMBIGUOUS`) or discarded by a maestro with words, and
withdrawn by Fabrewk. Each guard is labelled (REQ-SEC-5): cooperative are the guards of a move that grants nothing
(`task-proposed`, `stage-proposed`, whose events any process can write and whose fields the authorization checks
again), `WORDS_COPIED` (the signer can always type the example), `LOCK_HELD` (an advisory lock), REQ-RUN-2 (the
tools found on the machine) and `CREDENTIAL_SUSPECTED` (a heuristic); every other guard is effective, enforced by the
replay, by the signature of the person who moves after it was checked, by the confinement that keeps agents from the
domain, or by the Git host. `run-started`'s row takes no `--say` in section 4 while schema 1 gives every person's
move words: how `run` records them is decided with 7d3.

"Waiting state" is `authorized`, `halted` or `verified`. `run` on an `accepted` task finishes its merge. A `running`
task with no live process is halted by the next write command (REQ-RUN-5). A command whose move this table does not
allow from the task's state is refused (`MOVE_NOT_ALLOWED`); the same holds for stages and proposals.

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
| `fabrewk authorize <stage\|task> --say [--widen <key>]… [--run]` | yes | `stage-authorized`, `task-authorized`; with `--run`, also as `run` | section 4 |
| `fabrewk run <task\|stage>` | yes | `run-started` and the run's events | section 4 |
| `fabrewk accept <task> --say` | yes | `task-accepted`, `task-integrated` | section 4 |
| `fabrewk reject <task> --say` | yes | `task-rejected` | `MERGE_IN_PROGRESS` |
| `fabrewk sweep <alias> --answer <file>` | yes | `sweep-recorded`, `proposal-*` | REQ-FER-1, REQ-FER-3 |
| `fabrewk approve <proposal\|alambique> --say` | yes | `proposal-incorporated`, `direct-edit-approved` | `PROPOSAL_STALE`, `ALAMBIQUE_CHANGED`, `UNIT_AMBIGUOUS` |
| `fabrewk discard <proposal> --say` | yes | `proposal-discarded` | `PROPOSAL_UNKNOWN` |
| `fabrewk close <stage> --say` | yes | `stage-closed` | REQ-TASK-6 |

`propose` and `sweep` read a JSON file the integrator wrote; their schemas are printed by `--help` and published with
the reference documentation. `run <stage>` runs the stage's authorized tasks one after another and stops at the first
that waits for a person. `authorize … --run` signs the authorization and the `run-started` that follows it, and runs at
once, so a person's decision continues the work without another command (PD-33; choice 10 in section 10); `accept`
integrates in the same command. Every write command first applies REQ-RUN-5 and REQ-DOM-7.

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

As part 7d2 fixed it (D-053, D-055): `check` verifies C1 to C4 and C8 to C11; C5, C6, C7 and C12 are declared with
phase 8, when views, separators and the alambique's approval exist. Warnings (`DOMAIN_UNKNOWN_ENTRY`,
`POLICY_PENDING`) exit 0 when they are the only findings. C1 reads the project file, every `<alias>/settings.toml`
and `machine.toml`; C2 compares the file with the policy in force; C3 reports each code the replay gives; C4 reports
`RECORD_ABSENT`, the history walk where the mode commits, and the mark; C10 reports the record's files the record
branch's tip does not hold. A tool missing or failing, or a file that does not read, is `CHECK_UNKNOWN` for the check
that needed it, and so is a history that cannot be walked (`HEAD_DETACHED`, `NOT_A_REPOSITORY`) and a verifier that
fails: `ssh-keygen -Y check-novalidate` refuses a signature only by exiting 255, and any other failure, a signal
included, is the tool's (`TOOL_FAILED` in the library). With a record that does not replay, C2, the mark, C8's ignore
rule and C10 are `CHECK_UNKNOWN`; with a record and no `publication-decided`, C4's walk, C8's ignore rule and C10.
A project file that does not read leaves C2 `CHECK_UNKNOWN` besides C1's finding. With no domain at all, the one
finding is `RECORD_ABSENT`. C4 is labelled cooperative in `unpublished` and effective otherwise, in `check`'s list
and in `config`'s; C9 is cooperative. `config` words every guard of section 4 from the catalogue (`guard.<name>`).
`status` and `config` give exit 70 when the record's files cannot be read at all; `status <id>` with a record that
does not replay shows the record's state instead. A ref that exists and names no commit, or a Git that cannot say, is
`TOOL_FAILED`, never an unborn branch (P7D2-12). Nothing a reading command reads is opened unless it is a regular file,
and no link is followed, on the file or on its way from the domain (or the private folder) to it: a settings file that is not one is `SETTINGS_INVALID`, a mark `MARK_INVALID`, and the
record's `data/`, `data/events/` or a stream that is not a directory `RECORD_BROKEN` (P7D2-14); a citing file that is
not UTF-8 is read as far as it is (P7D2-15). A domain that is a link is `RECORD_BROKEN` (C3 in `check`); a private
folder that is one is `PRIVATE_FOLDER_PERMISSIONS`; Git's signing key is read only from a regular file outside the domain,
written or reached there through a link (P7D2-17, P7D2-22), and `--version` reads nothing (P7D2-25). What cannot be
examined on the way to the private folder or `machine.toml` is an error, never an absence (P7D2-24). A domain or a maintainer's directory that cannot be listed leaves C1 `CHECK_UNKNOWN`.

## 7. Refusals

Each code has a catalogue entry with its reason and next step; section 11.4 gives each a test. Codes not described in
the requirements above:

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
| `RECORD_HEAD_MOVED` | a stream's head moved under the lock, so something wrote without it; nothing was written (D-047) | run the command again |

All codes, in one list: `AGENT_ENVIRONMENT`, `ALAMBIQUE_CHANGED`, `ALAMBIQUE_UNAPPROVED`, `ALIAS_INVALID`,
`ALIAS_MISMATCH`, `ATTEMPT_ENDED`, `ATTEMPT_REPLACED`, `BASE_MOVED`, `BUDGET_EXHAUSTED`, `CHECK_DUPLICATE`,
`CHECK_UNKNOWN`, `CONFINEMENT_UNAVAILABLE`, `CREDENTIAL_SUSPECTED`, `DEPENDENCY_OPEN`, `DOMAIN_NOT_IGNORED`,
`DOMAIN_UNKNOWN_ENTRY`, `EVIDENCE_STALE`, `EXTENSION_DIGEST`, `EXTENSION_OUTPUT_INVALID`, `FERMENTO_MISSING`,
`GITIGNORE_CHANGED`, `HEAD_DETACHED`, `INPUT_CHANGED`, `INPUT_UNCOMMITTED`, `KEY_UNKNOWN`, `LANE_VIOLATION`,
`LIMIT_UNOBSERVABLE`, `LOCK_HELD`, `MAINTAINER_UNLISTED`, `MARK_INVALID`, `MERGE_IN_PROGRESS`, `MODE_NOT_AVAILABLE`,
`MOVE_NOT_ALLOWED`, `NOT_A_REPOSITORY`, `NO_PROGRESS`, `OUTCOME_UNKNOWN`, `PLAN_CHANGED`, `PLATFORM_UNSUPPORTED`, `POINT_CHANGED`,
`POLICY_ABSENT`, `POLICY_PENDING`, `POLICY_UNCOMMITTED`, `PRIVATE_FOLDER_PERMISSIONS`, `PROPOSAL_STALE`,
`PROPOSAL_UNKNOWN`, `PROTECTED_PATH`, `PROTECTED_PATH_WILDCARD`, `PROVIDER_MISSING`, `RECORD_ABSENT`,
`RECORD_BROKEN`, `RECORD_COMMIT_RACE`, `RECORD_DIGEST_MISMATCH`, `RECORD_FORK`, `RECORD_HEAD_MOVED`, `RECORD_ID_COLLISION`,
`RECORD_NONCANONICAL`, `RECORD_ROLLBACK`, `RECORD_SIGNATURE_INVALID`, `RECORD_TRANSITION_INVALID`,
`RECORD_UNCOMMITTED`, `RECORD_VERSION_UNKNOWN`, `REVIEW_INVALID`, `REVIEW_UNEXAMINED`, `ROLE_FORBIDDEN`,
`ROLE_NOT_ENFORCED`, `ROUNDS_EXHAUSTED`, `SEPARATOR_AMBIGUOUS`, `SEPARATOR_DAMAGED`, `SEPARATOR_MISSING`,
`SETTINGS_INVALID`, `SETTINGS_KEY_NOT_PERSONAL`, `STAGE_CLOSED`, `STAGE_OPEN_TASKS`, `SWEEP_MISSING`, `TASK_CHANGED`,
`TASK_INVALID`, `TERMINAL_REQUIRED`, `TOOL_FAILED`, `TOOL_MISSING`, `UNIT_AMBIGUOUS`, `VERDICT_NONE`, `VIEW_STALE`,
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

**Detection, as part 7c2b fixed it (D-051, D-052).** Each built-in module declares, for the program it drives, the
executable's name and the arguments that print its version on the first line of standard output without calling a
model; `git` (`--version`) and `ssh-keygen` (no version) are the core's. Names are looked for in three spaces, apart,
so that a name in one is never found as the same name in another: programs, by executable (the core's and each
module's program that serves no adapter, a backend's); tools (every catalogued tool, by its adapter's program, and
every name `machine.toml` declares); and required tools (every profile's, each by its own name). A machine file's path
must be absolute; it takes the place of the search of `PATH` (present there or absent, `declared`), except for the
core's own programs, which are always found in `PATH`, as the core runs them. `PATH` is searched in order, absolute
directories only; a name holding `/` is never searched. A file is executable when it is a regular file with an execute
bit that `/bin/sh -c 'test -x "$1"'`, run with no environment, accepts (D-052). A version is read only when its
arguments exit 0. `local/cache/detected.json` is canonical JSON
`{"at","programs":{…},"required":{…},"schema":1,"tools":{…}}`, each entry `null` or
`{"how","path","verified","version"}` (`how` is `observed` or `declared`; a path that is not UTF-8 is `null`), written
only by write commands; the next detection reads from it only `verified`, the date of the last successful use, kept
while the path and version are the same (never for a path that is not UTF-8); a cache that does not read is none. The
effective set is the catalogued tools that are present.

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
10. `authorize … --run`, so that a person's authorization continues into the run without another command (PD-33);
    without it, `authorize` records the decision and `fabrewk run` is the next step.
11. The test tooling of section 11.2: `proptest` as a development crate, never shipped; a Cargo feature
    `fault-injection`, never enabled in a release build; `fish`, `dash`, `bwrap`, `cargo-mutants` and `cargo-deny` on
    the machine that runs the suites, installed in phase 7 with the network permission that phase asks for.
12. The corrections part 6b made to part 6a (section 11.1), among them the requirements REQ-TASK-7 and REQ-RUN-9 and the codes
    `MOVE_NOT_ALLOWED` and `TOOL_FAILED`; and the phase in which each group of tests is due (section 13).
13. The credential scan's scope (REQ-SEC-3): content everywhere in the domain except a maintainer's uncited
    material, which is matched by name only. AD-8 asks for the check "anywhere in `.brewstillery/`" and the alambique's
    6.9 lets only cited material be read; this is how the text reconciles them, and it is the maestro's to approve.

## 11. Test plan

### 11.1 Corrections made to part 6a

Mapping every past-defect case to a requirement and a test found places where part 6a did not say what a test could
check. Each was corrected in its requirement, in place, and is listed here so that the review and the approval see it.

| Where | Correction | Case |
| --- | --- | --- |
| REQ-TASK-7 (new) | The pattern language of allowed files, its limits, and a lane guard at `propose` and `authorize`; part 6a refused protected paths only in the change set | PD-15, PD-17 |
| REQ-RUN-3 | Protected names compared under ASCII case folding; the rule for new, changed and untouched links; a mode change alone is a change; a failed export is `TOOL_FAILED`, recorded before dispatch | PD-16, PD-19 to PD-21 |
| REQ-GATE-8 | A third guard on protected paths at integration; "clean" defined against the base or, after an interrupted move, the candidate | PD-12, PD-16 |
| REQ-RUN-6, section 7 | New code `TOOL_FAILED` for an external tool's error, so none reaches the person raw | PD-11, PD-21, PD-36 |
| REQ-TASK-1, section 4 | Guards evaluated in their listed order, the first failure reported; new code `MOVE_NOT_ALLOWED` for a move the table does not allow from the current state (part 6a had no code for it) | — |
| REQ-TASK-5 | What a task does after a refused candidate, an invalid or unexamined review, a tool's failure or a failed preflight: it halts; the no-progress rule stated over rounds n−2, n−1, n, and its order with the round limit | PD-07, PD-25 |
| REQ-GATE-6, REQ-RUN-8 | A failed required check ends the round like a reject, counted as one confirmed blocking finding; the next round's packet carries the previous round's findings and the failed checks' output (part 6a gave the next round nothing to work from) | PD-33 |
| REQ-GATE-7 | `status <task>` shows verbatim what an authorization or an acceptance binds, with each text's language tag, and the event carries that rendering's digest | PD-37 |
| REQ-TASK-4 | A missing or non-UTF-8 `--say` is `WORDS_MISSING`, not a usage error; `propose` takes no `--say` | PD-30, PD-31 |
| REQ-REG-6 | An event out of its place is `RECORD_BROKEN`; a person's event bound to another stream, or with empty words, is `RECORD_TRANSITION_INVALID` | PD-06, PD-31 |
| REQ-REG-10 | The record is replayed and the views regenerated before Fabrewk commits; a failure commits nothing | PD-46 |
| REQ-REG-13 | Evidence readable by its owner only; messages name the evidence file | PD-28 |
| REQ-SEC-1 | A preflight per provider that calls no model, before its first dispatch and after a version change; the release directory bound whole | PD-27 |
| Section 5 | `authorize … --run` (choice 10) | PD-33 |

Codex's first review of both parts (R-021) found more, corrected the same way:

| Where | Correction | Finding |
| --- | --- | --- |
| REQ-GATE-8, REQ-GATE-3 | The base is compared by its product tree (outside `.brewstillery/`), so Fabrewk's own record commits never move it; when the head is not the base itself, integration makes a single-parent commit on the head carrying the candidate's change set and trailers | P6-1 |
| REQ-TASK-2, REQ-TASK-3, section 4 | A dependent task has no base when authorized; its base is fixed at `run-started`, after its dependencies' integrations, and recorded | P6-2 |
| REQ-REG-10 | `approve` and integration each make one commit before the record commit; events left uncommitted are committed first by the next write command | P6-3, P6-7 |
| REQ-REG-8 | The history walk requires every commit to hold every event file of each parent, so a truncation later restored exactly is still a rollback | P6-9 |
| REQ-REG-12, REQ-SET-8 | Only write commands create the machine key or write the mark and caches; reading commands, `assess` among them, write nothing | P6-8 |
| REQ-RUN-9 (new), REQ-GATE-8, REQ-TASK-5, section 4 | Revalidation of an existing candidate: `run` on a verified task with stale evidence, `authorize` from `accepted`; `MERGE_IN_PROGRESS` holds in every state while an acceptance of the candidate is on record and the branch is at it; `accept` moves a halted task whose branch is at its candidate | P6-6, P6-13 |
| REQ-FER-6, REQ-VIEW-2, REQ-REG-1, section 3 | Proposals targeting `rules`: their units, base digest and stream, independent of the alambique's | P6-12 |
| REQ-FER-9, REQ-SEC-3 | The credential scan matches content everywhere in the domain except a maintainer's uncited material, whose names alone it matches and whose content nothing reads; an unreadable file it must read is `CHECK_UNKNOWN` (choice 13) | P6-5 |

Its second review found four more, corrected the same way:

| Where | Correction | Finding |
| --- | --- | --- |
| REQ-REG-10 | Events a commit depends on are written before it, events recording its outcome after it; `task-integrated` only once the move and its checks passed | P6-14 |
| REQ-GATE-8, 11.2 rule 4 | Integration failures before and after the move, each with what it may change | P6-7 |
| Section 4, REQ-GATE-8 | `authorize` from `verified` as well, so a revalidation stopped by the round limit has a way out | P6-13 |
| REQ-SEC-3 | Patterns tightened so that describing them is no match (`sk-ant-` and the private-key line need what a real key has); this repository's domain passes the scan | P6-15 |

The specification now holds 103 requirements and 85 refusal codes.

### 11.2 Rules for every test

1. **Levels.** Unit tests live beside the code in `#[cfg(test)]` modules. Integration tests, in `tests/`, run the built
   `fabrewk` binary as a person or an agent would, in a disposable repository the test creates. Property tests generate
   their inputs with `proptest`, a development crate (choice 11). The crash suite stops the binary at every write
   (11.5). Mutation testing uses `cargo-mutants` (11.6). Real dispatches, from phase 9, call the real CLIs with the
   maestro's permission and budget and are never part of `cargo test`.
2. **Fixtures are built by the test**: `git init`, commits, SSH keys made with `ssh-keygen` inside the fixture, and a
   home and private folder of the fixture's own (the test sets `HOME` and `XDG_CONFIG_HOME`). Nothing is read from the
   machine's home, private folder or SSH agent. Temporary files go under Cargo's `CARGO_TARGET_TMPDIR`. Until phase 10
   builds `apply`, a test helper writes the same signed project events that `apply` would.
3. **Fake tools.** An agent's CLI is replaced by a fixture program placed on `PATH` under the real tool's name, which
   replays a recorded stream or a scripted behaviour (write files, change a mode, make a link, print a marker, exit
   with a code, sleep, crash). The built-in adapters are the code under test. The recorded streams for the audit
   (PD-26) come from phase 9's first real dispatch per provider, with every credential, account name and provider text
   the audit does not need removed, and their digests pinned in the test.
4. **What a refusal test asserts**, all of: the exit status of REQ-CLI-3; the code in the human output and in `--json`;
   the catalogue's reason, and a next step that names an existing command, with no unfilled placeholder; and that
   nothing was written: the working tree, the index, the refs, `.git/info/exclude`, the domain and the private folder
   are byte-identical before and after. Three kinds of refusal write, and their tests assert exactly what: a halt
   writes its `task-halted` event, the `attempt-ended` it implies and the views they affect; a refused integration
   (`BASE_MOVED`, `WORKTREE_DIRTY`, `EVIDENCE_STALE`, `PROTECTED_PATH`, `TOOL_FAILED`) writes `task-accepted` when the
   command was `accept`, then `merge-failed` and the views, in one record commit; before the move, the product tree, the
   index and the working tree are byte-identical and, in `published`, the branch differs only by that record commit;
   after the move (PD-12's variants), the branch holds the integration and the record commit, and the working tree
   and the index are byte-identical to before the command except at touched paths that held the base's content,
   which may now hold the candidate's (an interrupted sync); a path that held anything else, a person's edit, is
   byte-identical. "Before" is the state after the fixture's last injection: a test that injects a person's edit
   while the command runs, as PD-12's variants do after the move, compares with a snapshot taken right after that
   injection, so the edit must survive byte-identical in the working tree and in the index; `RECORD_COMMIT_RACE` leaves the command's
   events written and uncommitted, and nothing else.
5. **Names.** A requirement's tests are `req_<area>_<n>_<what>`; a past-defect case's, `pd_<nn>_<what>`; a refusal's,
   `refusal_<code in lower case>`. A search for the name finds the test from this file and this file from the test.
6. **A past-defect test fails without its fix.** The phase's receipt names one mutation of the guard the case protects
   (a `cargo-mutants` mutant or a written patch) and shows the test killing it.
7. **Clean clone.** Every suite runs in a new clone: `cargo test --locked`, then the crash suite with
   `--features fault-injection`. A test that needs a tool the machine lacks (`git`, `ssh-keygen`, `bwrap`, `fish`,
   `dash`) fails naming it; it is never skipped as passed. A phase is accepted only with every test of its groups run.
8. **Release builds carry no test seam.** `fabrewk --version --json` lists the build's features; a test on the release
   binary requires that `fault-injection` is absent.
9. **Shells.** Every command a message prints passes `fish --no-execute` and `dash -n` (REQ-CLI-2, PD-38).
10. **Platforms.** The suites run on Linux ARM64 first (AD-4); a platform is supported only when every suite passes
    on it.

### 11.3 Past-defect cases

Every case of `past-defect-cases.md`, with the requirements it maps to, the test that keeps it from recurring, and the
phase that must pass it (section 13). Case variants are written there; a test exercises every variant. The cases name
the domain as `brewstillery/`, from before it was hidden (D-032); here it is `.brewstillery/`, which REQ-RUN-3
protects at any depth.

| Case | Requirements | Test: what it builds and asserts | Phase |
| --- | --- | --- | --- |
| PD-01 | REQ-REG-8, REQ-REG-6 | In `published`, after a mark exists: (a) a commit whose `data/` is a prefix of its parent's; (b) the branch reset to an ancestor; (c) a merge whose first parent is the old state and the second the watched one; (d) events seen, cut and regrown; (e) a `git replace` object hiding a commit; (f) a graft file; (g) a commit that drops the last two events and a later one that restores them byte-identical and appends a new event. `check` reports `RECORD_ROLLBACK` for each (by the mark for (b), by the history walk for the rest) and every write command refuses. In `unpublished`, (a), (b) and (d) are made by deleting event files and are refused by the mark | 7 |
| PD-02 | REQ-REG-8 | `heads.json` empty, holding invalid JSON, or naming a digest no stream holds: every command that reads it refuses `MARK_INVALID`, naming the file, and nothing deletes it; once a person removes it, the next command writes a fresh mark. The crash suite stops the binary during the mark's replacement: the old or the new mark remains, never an empty one | 7 |
| PD-03 | REQ-REG-9, REQ-REG-6 | A test compares every event kind's fields with a pinned description of schema 1, so a changed field without a new `schema` fails it; an event with `schema` 2 is refused `RECORD_VERSION_UNKNOWN`; a record holding every version the binary knows replays; no replay test ends in a panic or a raw parse error | 7 |
| PD-04 | REQ-REG-3 | Events whose agent text, person's words, finding text and file names hold U+0085, U+2028, U+2029 and a lone CR are written, replayed, re-serialized byte-identical and shown whole by `status`; a truncated file and one with a changed byte are still refused (`RECORD_DIGEST_MISMATCH`, `RECORD_NONCANONICAL`); a source test fails if the record or the stream audit calls a line-splitting function on recorded text | 7 |
| PD-05 | REQ-REG-7 | A clone whose domain has no `data/`, and one with an empty `data/events/`: `status` shows the record absent (exit 0), `check` exits 3 with `RECORD_ABSENT`, every write command refuses | 7 |
| PD-06 | REQ-TASK-2, REQ-REG-6, REQ-GATE-7 | Two tasks with identical fields: the signed `task-authorized` of one, copied into the other's stream, is refused on replay (`RECORD_BROKEN` for its place, `RECORD_TRANSITION_INVALID` for its binding); re-appended later in its own stream, refused; `authorize` after a change to any field refuses `TASK_CHANGED`. The same for `task-accepted` | 7 |
| PD-07 | REQ-RUN-2, REQ-TASK-5 | `rounds = 2` and a fake reviewer that always rejects: no third review is dispatched, a revalidation of stale evidence (REQ-RUN-9) counted as a round (counted by the fake's invocations); the task halts `ROUNDS_EXHAUSTED`; only `authorize --say` grants another round | 9 |
| PD-08 | REQ-RUN-5, REQ-RUN-1, REQ-TASK-5, REQ-VIEW-1 | The fake agent ends successfully and the binary is killed before `attempt-ended`, for the build and the review roles: the next write command ends the attempt `unknown`, counts it and halts `OUTCOME_UNKNOWN`; `status` says the attempt's result is unknown and that a retry spends a dispatch; a late result for the replaced attempt is refused `ATTEMPT_REPLACED`. Variants: a second loss on the retry; no dispatch left (halts `BUDGET_EXHAUSTED`, nothing dispatched); an earlier halt already answered is not what `status` shows; another task runs to integration meanwhile. Property: the crash suite (11.5) | 9; the crash suite on the record, 7 |
| PD-09 | REQ-RUN-5, REQ-RUN-1 | A task whose attempt's process died, in `authorized` (before dispatch), `running` and `halted`: `reject --say` ends the open attempt `unknown`, counted, then rejects; a later `attempt-ended` for it is refused `ATTEMPT_ENDED`; a second task then runs | 9 |
| PD-10 | REQ-RUN-1, REQ-RUN-5 | `attempt-started` is on disk before the fake CLI starts (the fake reads the record and fails otherwise); a second `attempt-ended` is refused `ATTEMPT_ENDED`, at the command and on replay, and the first is kept; a killed attempt stays counted and nothing dispatches again without `authorize` | 7 (record), 9 (dispatch) |
| PD-11 | REQ-RUN-6, REQ-GATE-8 | A leftover `.git/index.lock` during task A's integration: A records `merge-failed` with `TOOL_FAILED` and words, and stays `accepted`; `status` and `check` work; task B builds its candidate meanwhile (its export uses a temporary index); once the lock is removed, `run` integrates A | 9 |
| PD-12 | REQ-GATE-8 | Each precondition (evidence current, base unmoved, touched paths clean, reviewer profile unchanged in the adopted policy) made false at each of the three points (before signing, just before the move, with the branch at the candidate): no `task-integrated`, a `merge-failed` naming it, the task `accepted`. Variants: the branch already at the candidate, moved by someone else; the binary killed between `update-ref` and the working tree's update, after which `run` finishes it | 9 |
| PD-13 | REQ-GATE-8 | The branch moved after acceptance by a commit that changes a file outside the domain: `BASE_MOVED`, branch untouched; record commits alone in between do not count as a move, and integration then makes a single-parent commit whose product tree is the candidate's; a race test moves it between Fabrewk's read and its `update-ref`, and the compare-and-swap fails; a checked-out tree dirty at a touched path: `WORKTREE_DIRTY`, the tree untouched | 9 |
| PD-14 | REQ-GATE-8 | An accepted task whose branch reached the candidate and whose integration then failed: `reject` refuses `MERGE_IN_PROGRESS`, `run` finishes it. The reviewer's variant: accepted, failed before moving, run back to accepted, then the branch brought to the candidate by hand as in PD-44: `reject` refused, `run` finishes under every precondition. With no acceptance on record, `reject` is allowed whatever the branch and records where it is. After `authorize` returns such a task to `authorized`, `reject` is still `MERGE_IN_PROGRESS` | 9 |
| PD-15 | REQ-TASK-7 | Refused at `propose`, and at `authorize` for a proposal edited in the record by a fixture: `**/*.png`, `*/xel/**`, `?brewstillery/settings.toml`, `**/attachments/**`, `*`, `**`, `**/*.py`, `*/config/*`, `*.git`, `.G*`, `./.brewstillery/x`, `tests/fixtures/.brewstillery/**` (`PROTECTED_PATH`); `src/../.git/config` and a 257-byte pattern (`TASK_INVALID`). Kept: `*.md`, `src/**`, `.github/**`, `.gitignore`, `.git*ignore`, `tests/fixtures/brewstillery/**`, `.brewstillery-old/x`, `brewstillery-old/x`. Properties: over the tokens `a`, `G`, `.`, `/`, `*`, `?`, `**`, `.git`, `.brewstillery`, for patterns and paths of up to six tokens, the guard refuses a pattern exactly when a reference built on the matcher finds a protected match as REQ-TASK-7 defines it, both ways; random long patterns the guard keeps never match a protected path at the first component | 7 |
| PD-16 | REQ-RUN-3, REQ-GATE-8 | The fake agent writes `.brewstillery`, `.brewstillery/x`, `.BrewStillery/x`, `a/.git/config`, `.GIT` and a link named `.brewstillery`, with the lane guard switched off by `fault-injection`: each refused `PROTECTED_PATH` at the change set; with the change-set guard also off, integration refuses `PROTECTED_PATH`. Run with the domain committed in the base and not, and with the record branch checked out and not. What Git refuses surfaces as `TOOL_FAILED`, never raw | 9 |
| PD-17 | REQ-TASK-7, REQ-RUN-3 | `src/` followed by eight `**a` and a `b` against a 60-character path, and 150 `**` segments against a 4,000-character path: the matcher's step count, exposed to tests, stays within REQ-TASK-7's bound. Property: the matcher equals a recursive reference definition over random small pairs, paths with newlines included | 7 |
| PD-18 | REQ-RUN-3 | The base ignores `*.log` and `out/`; the fake agent creates ignored files whose names hold a space, a double quote, a backslash, a newline, a tab, an accented letter and U+2028: none enters the candidate; tracked files with the same names, edited inside the lane, are kept byte-exact | 9 |
| PD-19 | REQ-RUN-3 | A base with tracked links to a relative file, to `/etc/passwd`, to a missing target and to a directory, left untouched: none is in the change set and the task proceeds; a new link inside the lane is in the candidate as a link with its target as written; a new link outside it is `LANE_VIOLATION` | 9 |
| PD-20 | REQ-RUN-3 | The fake sets the executable bit on a tracked file and changes nothing else: the change set holds the mode change and the candidate has mode `100755`; with another file also edited, both are in the candidate; never "no change" | 9 |
| PD-21 | REQ-RUN-3, REQ-RUN-6 | A base holding a link to an absolute path: the export holds it as a link and the task proceeds. An export made to fail by `fault-injection`: `attempt-ended` with `TOOL_FAILED` is recorded before any fake CLI starts, the task halts, the message names the evidence file and holds no raw error | 9 |
| PD-22 | REQ-RUN-3, REQ-RUN-8, REQ-SEC-1 | A base holding `.git` and `.brewstillery` at the root and nested: the exports for an agent and for a check hold neither at any depth; the fake agent and a check probe for the repository's real path, the domain and the private folder and find none; marker strings planted in the settings, the alambique and a fermento appear nowhere in what the fake receives | 9 |
| PD-23 | REQ-GATE-1, REQ-GATE-3, REQ-GATE-5 | A task accepted, then a check removed from its profile and the policy adopted: integration refuses `EVIDENCE_STALE`; a check the stage approved but the profile lacks is still required and run | 9 |
| PD-24 | REQ-GATE-1, REQ-SET-2 | A stage approving a check `test` defined differently from the project's `test`: `CHECK_DUPLICATE` at `propose`, before any check runs; a duplicate TOML table: `SETTINGS_INVALID`. The legacy pair (a required `false` and an optional `true` under one name) cannot be stated without one of the two refusals | 7 |
| PD-25 | REQ-GATE-4, REQ-TASK-5 | A fake reviewer returns a well-formed `accept` after running no command: the task halts `REVIEW_UNEXAMINED`, with no `task-verified` | 9 |
| PD-26 | REQ-GATE-4 | For each provider's adapter, its recorded valid stream (11.2 rule 3) audits as examined. Each of PD-26.1 to PD-26.20 is a generator that inserts its anomaly at every position of that stream, and every result audits as not examined, except PD-26.16, whose stream reads whole and counts as the valid one does. One stream per sub-case goes through the whole flow and halts `REVIEW_UNEXAMINED`. PD-26.2: quoted separators do not split, unquoted `;`, `&&`, `\|\|`, `\|` and newlines do. PD-26.19: a final newline is not a blank line | 9 |
| PD-27 | REQ-SEC-1 | A fake CLI that needs a sibling helper and starts its own sandbox: the preflight passes only with the release directory bound whole; a fake whose sandbox cannot start in a role's copy fails the preflight with `CONFINEMENT_UNAVAILABLE` before any dispatch. In the real dispatches of phase 9, the preflight runs first | 9 |
| PD-28 | REQ-REG-13 | Fake CLIs that fail, succeed and time out each leave their standard output and error in `local/evidence/` (files `0600`, directories `0700`), with their digests in the attempt's event; the message names the file | 9 |
| PD-29 | REQ-REG-13 | A fake CLI's error text, error subtype, standard error and a free-form usage field each carry a marker: no file under `data/` holds a marker, the evidence files do, and the event keeps the model and CLI version as reported | 9 |
| PD-30 | REQ-TASK-4 | For every person move of section 4, the words given are in the signed event byte-identical, with their language tag; `propose --say` is a usage error and `propose` records no person | 7 |
| PD-31 | REQ-TASK-4, REQ-REG-6 | For every person move: `--say` absent, `""`, `" "`, a tab and a newline, U+2028 alone, and bytes that are not UTF-8: `WORDS_MISSING`, nothing written; a hand-made, validly signed event with empty `say`: `RECORD_TRANSITION_INVALID` on replay | 7 |
| PD-32 | REQ-TASK-4, REQ-CLI-2 | Every catalogue message rendered with every parameter offers no words: a command it names that needs `--say` is printed without it, so running it as printed refuses `WORDS_MISSING`, nothing written; the catalogue's example words given as `--say` refuse `WORDS_COPIED` | 7 |
| PD-33 | REQ-GATE-6, REQ-GATE-8, REQ-VIEW-1, REQ-CLI-2 | After `accept --say` the same command integrates; after a review's reject or a failed check, the next round starts with no command; `authorize --run --say` runs the task with no other command. `status`, in every waiting state, names who acts next and shows no command to a person with nothing to decide | 9 (moves), 8 (views) |
| PD-34 | REQ-CLI-2 | Every catalogue key rendered in every language with every value the code can pass (each state, event kind, role and code): no bare key, no internal token without its words, no doubled punctuation; every key has a description; every state, event kind, role and code has words in every catalogue | 7 |
| PD-35 | REQ-CLI-2, REQ-CLI-4 | Every refusal test (11.4) asserts the code in the human output and in `--json` | 7 |
| PD-36 | REQ-CLI-2, REQ-RUN-6 | No rendered message holds a `{name}` placeholder; failures provoked in `git` and `ssh-keygen` reach the person as `TOOL_FAILED` with words, their raw text only in evidence | 7 |
| PD-37 | REQ-GATE-7 | `status <task>` before `authorize` and before `accept` shows verbatim every requirement and its criterion, the allowed files and the limits, and before `accept` the candidate and its evidence, each text with its language tag; the signed event carries the digest of exactly that rendering; for a maintainer whose interaction language is `es-MX`, the surrounding words are in Spanish and the requirements in the language they were written, labelled | 7 (authorize), 9 (accept) |
| PD-38 | REQ-CLI-2, REQ-CLI-1 | Every command and flag that any message names is accepted by the command declaration's parser; every printed command passes `fish --no-execute` and `dash -n` | 7 |
| PD-39 | REQ-RUN-7, REQ-VIEW-1 | A fake CLI that reports no cached tokens, one that reports 0, and a mix of both across attempts: the events record unknown, 0, and both; the view shows "unknown", "0", and a sum with an unknown part as unknown | 9 (record), 8 (view) |
| PD-40 | REQ-VIEW-1 | A dispatch interrupted for an hour, then halted: the person's response time starts at the halt, not at the dispatch; with nothing pending for them, the view says there is none | 8 |
| PD-41 | REQ-REG-10, REQ-FER-3, REQ-FER-5 | A maintainer's edit to their fermento staged, another unstaged: every write command's commit holds only Fabrewk's files and leaves both edits as they were; `sweep` refuses `INPUT_UNCOMMITTED`; `approve` commits the alambique alone, with the maestro as author and no co-author | 8 |
| PD-42 | REQ-VIEW-1, REQ-FER-4 | Every count and state in `STATUS.md` and in the fermento's answer follows the replay: events are added and the views regenerated, and every figure changes as the replay says; no view takes text from anywhere but the replay and the catalogue | 8 |
| PD-43 | REQ-VIEW-1, REQ-VIEW-3 | `STATUS.md` holds no branch head or other fact a later event could make false without a regeneration; a hand edit of a view is `VIEW_STALE`; after every write command, a fresh generation equals the committed text | 8 |
| PD-44 | REQ-GATE-8 | A task in `authorized`, `halted` and `verified`, each with the record branch brought to its candidate by hand (a cherry-pick of the candidate in `published`, so the record's commits stay; a move to the candidate commit in the other modes): `accept --say` integrates it under every precondition of PD-12, and `reject --say` stops it, recording where the branch is; no hand move of the branch is needed | 9 |
| PD-45 | REQ-RUN-4 | The candidate commit read back: its parent is the base; one co-author trailer names the tool, provider and model the adapter observed; with `git.signOff = true` a sign-off trailer, with `false` none | 9 |
| PD-46 | REQ-GATE-5, REQ-GATE-10, REQ-REG-10 | A required check fails: no `task-verified`, the branch never moves, no commit but the record's own; a write command whose replay before commit fails (made to fail by `fault-injection`) commits nothing and says so | 9 (gate), 7 (record) |

Of the items `past-defect-cases.md` did not turn into cases, C8F-5 and FX-8 (refusal codes with no test) are answered
by 11.4, which gives every code a test; the others keep their destinations there.

### 11.4 Refusals

One test per code (11.2 rule 4), with the requirement that defines the code and what the test does to provoke it.
`check`'s findings exit 3; a halt exits 4; every other refusal exits 1. What each writes follows 11.2 rule 4.

| Code | Requirement | Provoked by | Phase |
| --- | --- | --- | --- |
| `AGENT_ENVIRONMENT` | REQ-TASK-4, REQ-SEC-7 | A person's move with `CLAUDECODE=1`, with Codex's sandbox variable, and with `FABREWK_EXTENSION` set; the message says the control is cooperative | 7 |
| `ALAMBIQUE_CHANGED` | REQ-FER-5 | `approve` after the alambique changed from the proposal's base digest | 8 |
| `ALAMBIQUE_UNAPPROVED` | REQ-FER-7 | A byte of the alambique changed by hand, then `check`, `run`, `accept` and `close` | 8 |
| `ALIAS_INVALID` | REQ-DOM-3 | Aliases `Xel`, `9a`, `a--b`, `a-`, 33 characters, and each reserved name; a domain directory with such a name at `check` (a reserved name at the domain's top is a listed entry or `DOMAIN_UNKNOWN_ENTRY`, D-051) | 7 |
| `ALIAS_MISMATCH` | REQ-SET-3 | `maintainer.alias = "ana"` in `xel/settings.toml` | 7 |
| `ATTEMPT_ENDED` | REQ-RUN-1 | A second `attempt-ended` for one attempt | 7 |
| `ATTEMPT_REPLACED` | REQ-RUN-5 | A result for an attempt a later one replaced (PD-08) | 9 |
| `BASE_MOVED` | REQ-GATE-8, REQ-TASK-3 | The product tree changed after acceptance (PD-13); a dependent task's `run` after another product change followed its dependency's integration | 9 |
| `BUDGET_EXHAUSTED` | REQ-TASK-5, REQ-RUN-2 | `dispatches` spent; `minutes` spent by a fake that sleeps past the limit | 9 |
| `CHECK_DUPLICATE` | REQ-GATE-1 | A stage and the project defining one check name differently (PD-24) | 7 |
| `CHECK_UNKNOWN` | Section 6 | `check` with `git` absent from `PATH`; with the Git history unreadable; with a domain file it must scan unreadable (C9) | 7 |
| `CONFINEMENT_UNAVAILABLE` | REQ-SEC-2, REQ-SEC-1 | `run` with `bwrap` absent; a failed preflight (PD-27) | 9 |
| `CREDENTIAL_SUSPECTED` | REQ-SEC-3 | Each pattern of REQ-SEC-3 in a file under the domain (`check`) and in a candidate (`run`); near misses (`AKIA` with 15 characters, `id_ed25519.pub`, `sk-` with 19) are not refused | 7, 9 |
| `DEPENDENCY_OPEN` | REQ-TASK-3 | `run` on a task whose dependency is not integrated | 7 |
| `DOMAIN_NOT_IGNORED` | REQ-DOM-7 | `unpublished` with the exclude line removed: every write command, and `check` | 7 |
| `DOMAIN_UNKNOWN_ENTRY` | REQ-DOM-2 | A file `notes.md` at the domain's top: a warning, `check` exits 0 if it is the only finding, the file kept | 7 |
| `EVIDENCE_STALE` | REQ-GATE-3 | Each input of a check's identity changed after the check: tree, definition, policy, a named tool's version, platform | 9 |
| `EXTENSION_DIGEST` | REQ-EXT-1 | An extension whose file differs from its pinned digest | 9 |
| `EXTENSION_OUTPUT_INVALID` | REQ-EXT-3 | An extension printing one valid request and one invalid line: none applied | 9 |
| `FERMENTO_MISSING` | Section 6, C12 | A listed maintainer's directory without `fermento.md` | 8 |
| `GITIGNORE_CHANGED` | REQ-DOM-4 | `.brewstillery/.gitignore` with a line added | 7 |
| `HEAD_DETACHED` | REQ-REG-11 | A write command with a detached head | 7 |
| `INPUT_CHANGED` | REQ-FER-3 | The input edited between the sweep's two digests (`fault-injection` pauses the write) | 8 |
| `INPUT_UNCOMMITTED` | REQ-FER-3 | `sweep` in `published` with the input edited and not committed | 8 |
| `KEY_UNKNOWN` | Section 7 | `config nosuch.key` | 7 |
| `LANE_VIOLATION` | REQ-RUN-3 | The fake agent changes a file outside the allowed files | 9 |
| `LIMIT_UNOBSERVABLE` | REQ-RUN-7 | A `tokens` limit on a profile whose adapter declares no token report, at `authorize` | 7 |
| `LOCK_HELD` | REQ-REG-5 | A second write command while the first holds the lock | 7 |
| `MAINTAINER_UNLISTED` | Section 6, C12 | A directory `ana/` with a fermento, not in the policy | 8 |
| `MARK_INVALID` | REQ-REG-8 | An empty `heads.json` (PD-02) | 7 |
| `MERGE_IN_PROGRESS` | REQ-GATE-8 | `reject` on an accepted task with the branch at its candidate (PD-14) | 9 |
| `MODE_NOT_AVAILABLE` | REQ-INS-3 | `apply` of a plan whose verdict is `review` | 10 |
| `MOVE_NOT_ALLOWED` | REQ-TASK-1 | `run` on a `proposed` task, `accept` on a `running` one, `authorize` on an `integrated` one, `approve` of a discarded proposal, `close` of a closed stage; each with its command's phase (D-053) | 7, 8, 9 |
| `NOT_A_REPOSITORY` | Section 7 | `assess` outside a Git working tree, and in a bare repository | 10 |
| `NO_PROGRESS` | REQ-TASK-5 | Confirmed blocking findings 2, 2, 2 over three rounds; 2, 3, 1 does not halt | 9 |
| `OUTCOME_UNKNOWN` | REQ-TASK-5, REQ-RUN-5 | The binary killed during an attempt (PD-08) | 9 |
| `PLAN_CHANGED` | REQ-INS-3 | A root file changed between `assess` and `apply`; `apply` with a key other than the plan's | 10 |
| `PLATFORM_UNSUPPORTED` | REQ-INS-7 | The installation script with a fake `uname` reporting an unlisted platform | 10 |
| `POINT_CHANGED` | REQ-FER-3 | An answer whose point's digest differs; one naming a point that does not exist | 8 |
| `POLICY_ABSENT` | REQ-SET-4 | `authorize` in a record with no `policy-adopted` | 7 |
| `POLICY_PENDING` | Section 6, C2 | The project file edited after adoption: a warning at `check`, the change pending in `config` | 7 |
| `POLICY_UNCOMMITTED` | REQ-SET-4 | `adopt` in `published` with the file edited and not committed | 7 |
| `PRIVATE_FOLDER_PERMISSIONS` | REQ-SEC-4 | The private folder with mode `0755` | 7 |
| `PROPOSAL_STALE` | REQ-FER-5, REQ-FER-6 | `approve` after its point's text changed, after the point's number changed with the same words, and after the drafted text changed; a `rules` proposal after another rules proposal was incorporated | 8 |
| `PROPOSAL_UNKNOWN` | Section 7 | `discard p-00000000` | 8 |
| `PROTECTED_PATH` | REQ-TASK-7, REQ-RUN-3, REQ-GATE-8 | The patterns of PD-15 and the writes of PD-16 | 7, 9 |
| `PROTECTED_PATH_WILDCARD` | REQ-GATE-9 | Allowed files `*.toml` with `Cargo.toml` in `gate.protectedPaths`; `Cargo.toml` listed literally is kept | 7 |
| `PROVIDER_MISSING` | REQ-TASK-5, REQ-GATE-4 | `review.differentProvider = "require"` with one provider permitted; a required provider's tool absent | 9 |
| `RECORD_ABSENT` | REQ-REG-7 | PD-05 | 7 |
| `RECORD_BROKEN` | REQ-REG-6 | A gap in `seq`; a `prev` naming no event; an event in another stream's directory | 7 |
| `RECORD_COMMIT_RACE` | REQ-REG-10 | The branch moved during a write command, changing a file the commit holds | 7 |
| `RECORD_DIGEST_MISMATCH` | REQ-REG-6 | A file renamed with another digest; a byte changed | 7 |
| `RECORD_FORK` | REQ-REG-6 | Two events at one `seq` of one stream, from two clones merged | 7 |
| `RECORD_HEAD_MOVED` | REQ-REG-5 | A stream's head moved between the command's read and its append under the lock (`fault-injection`); nothing written (D-047) | 7 |
| `RECORD_ID_COLLISION` | REQ-REG-1 | Two creation payloads made to share an identifier, with nonces chosen by the fixture | 7 |
| `RECORD_NONCANONICAL` | REQ-REG-3 | Keys out of order; a space added; `1.0` for an integer; a needless escape; no final newline | 7 |
| `RECORD_ROLLBACK` | REQ-REG-8 | PD-01 | 7 |
| `RECORD_SIGNATURE_INVALID` | REQ-REG-4, REQ-REG-6 | A person's event without `.sig`; signed by a key absent from the cited policy; a signature over other bytes; another namespace; and after D-050: citing no adopted policy or, in `project`, another than the one in force at its place; a role the policy does not give; a contributor's move; written after an adoption that removed its signer, changed their key or made them a contributor; outside `project`, held by every seal | 7 |
| `RECORD_TRANSITION_INVALID` | REQ-REG-6 | A hand-made `task-integrated` from `verified`; PD-06; PD-31 | 7 |
| `RECORD_UNCOMMITTED` | REQ-TASK-6, section 6 C10 | Events left uncommitted by `RECORD_COMMIT_RACE`: `check` reports them and `close` refuses | 7, 8 |
| `RECORD_VERSION_UNKNOWN` | REQ-REG-6 | An event with `schema` 2 (PD-03) | 7 |
| `REVIEW_INVALID` | REQ-RUN-8 | A reject with no confirmed blocking finding; a verdict `maybe`; a finding without a severity | 9 |
| `REVIEW_UNEXAMINED` | REQ-GATE-4 | PD-25 and PD-26 | 9 |
| `ROLE_FORBIDDEN` | REQ-ID-3 | `authorize` signed by a contributor; by a key with no role | 7 |
| `ROLE_NOT_ENFORCED` | REQ-ID-3 | `adopt` of a policy that lists a supervisor | 7 |
| `ROUNDS_EXHAUSTED` | REQ-TASK-5, REQ-RUN-2 | PD-07 | 9 |
| `SEPARATOR_AMBIGUOUS` | REQ-FER-1 | Two marker lines outside fences | 8 |
| `SEPARATOR_DAMAGED` | REQ-FER-1 | The marker with an edited banner line | 8 |
| `SEPARATOR_MISSING` | REQ-FER-1 | No marker; the marker only inside a fenced block (backticks, tildes, an unclosed fence); the marker indented | 8 |
| `SETTINGS_INVALID` | REQ-SET-2 | An unknown key, a wrong type, a value outside its set, a duplicate table: each with file, key path and line | 7 |
| `SETTINGS_KEY_NOT_PERSONAL` | REQ-SET-3 | `review.differentProvider` in a personal file | 7 |
| `STAGE_CLOSED` | Section 4 | `propose` of a task into a closed stage | 7 |
| `STAGE_OPEN_TASKS` | REQ-TASK-6 | `close` with a task `verified` | 8 |
| `SWEEP_MISSING` | REQ-TASK-6 | `close` when a maintainer's last sweep predates the end of the stage's last task | 8 |
| `TASK_CHANGED` | REQ-TASK-2 | `authorize` after the task's fields changed or its stream's head moved | 7 |
| `TASK_INVALID` | Section 4, REQ-TASK-7 | A task with no requirement; a requirement with no criterion; a pattern with `..` (PD-15) | 7 |
| `TERMINAL_REQUIRED` | Section 7 | `onboard` with standard input not a terminal | 10 |
| `TOOL_FAILED` | REQ-RUN-6 | `git` failing on an index lock (PD-11); a failed export (PD-21); `ssh-keygen` failing to sign | 7, 9 |
| `TOOL_MISSING` | REQ-RUN-2 | A profile's `requiredTools` entry absent | 9 |
| `UNIT_AMBIGUOUS` | REQ-FER-6 | An edit whose section heading occurs twice in the alambique | 8 |
| `VERDICT_NONE` | REQ-INS-3 | `apply` of a plan whose verdict is `none` | 10 |
| `VIEW_STALE` | REQ-VIEW-3 | A byte of `STATUS.md` or `RULES.md` changed by hand | 8 |
| `WIDENING_UNNAMED` | REQ-SET-5 | `adopt` raising `limits.rounds` without `--widen limits.rounds`; adding a maintainer | 7 |
| `WORDS_COPIED` | REQ-TASK-4 | The catalogue's example words given as `--say` (PD-32) | 7 |
| `WORDS_MISSING` | REQ-TASK-4 | PD-31 | 7 |
| `WORKTREE_DIRTY` | REQ-GATE-8 | A touched path edited in the checked-out tree (PD-13) | 9 |

### 11.5 Whole flows and the crash suite

- **F-1, administer, `published`.** Project events (from `apply`, or from the test helper before phase 10); `propose`
  a stage with two tasks, the second depending on the first; `authorize` the stage; `run` it: task 1 rejected by the
  review in round 1, accepted in round 2 and integrated; task 2 waits for it, then runs and integrates; `sweep`;
  `approve` a proposal to the alambique and one to the rules; `close`. After every command `check` exits 0, the views
  equal a fresh generation, and the record branch has one record commit per write command with its trailers, preceded
  by the alambique's own commit for `approve` and by the integration's for `accept`.
- **F-2.** F-1 in `own-repository` (commits in the domain's repository, none in the product's) and in `unpublished`
  (no commit; the exclude line in place).
- **F-3, two maintainers.** A maestro and a contributor in two clones: the contributor's moves refused
  `ROLE_FORBIDDEN`, their fermento swept like the maestro's; events written in both clones and merged are
  `RECORD_FORK`, handed to a person.
- **Crash suite.** With `fault-injection`, the binary is killed before and after every file write of F-1: each event,
  signature, mark, view, commit, ref update and working tree update. After each kill: replay is valid or refused by a
  code, never by a raw error; no task stays `running` after the next write command; nothing is dispatched without a
  person's move recorded after the kill; no event already committed is lost. This is PD-08's property and 6.41's test.

### 11.6 Mutation, size and dependencies

- **Mutation.** `cargo mutants` over the core ring and the built-in modules, in a clean clone, from phase 7. Every
  missed mutant is killed by a new test or listed in the phase's receipt as equivalent, with its reason; the
  reviewer checks that list. Timeouts are listed; unviable mutants are not counted.
- **Size.** The counter of REQ-BUD-1 has fixture tests of its own: blank lines, `//`, `///` and `//!` comments,
  nested `/* */`, a `//` inside a string, `#[cfg(test)]` modules, `tests/`, `benches/`, `examples/`. A test fails when a
  ring passes its ceiling.
- **Dependencies.** `cargo build --release --locked`; `cargo deny check licenses advisories` with REQ-BUD-4's allowlist;
  the stripped binary's size; the crates of the release build counted by `cargo tree --edges normal,build` for the
  release target, the package itself not counted. Development crates are outside the runtime budget (AD-4) and inside
  the licence and advisory check.

## 12. Acceptance criteria

A requirement is met when its criterion holds in the suites of its phase (section 13), run under the rules of 11.2.
Where a criterion names a case or a code, its test in 11.3 or 11.4 is part of the criterion.

### DOM

| Requirement | Accepted when | Phase |
| --- | --- | --- |
| REQ-DOM-1 | After every flow of 11.5, comparing a snapshot of the repository from before shows changes outside `.git/` only under `.brewstillery/`, in the entry block of REQ-DOM-6, and at the paths of the integrated candidates' change sets, each with the candidate's content; and inside `.git/` only Git objects, the record branch's and `fabrewk/<task>` refs and their logs, the index when a checked-out tree is brought to a candidate and, in `own-repository` and `unpublished`, the one exclude line | 7, 10 |
| REQ-DOM-2 | After every flow the domain's top holds only the listed names; an extra entry is reported `DOMAIN_UNKNOWN_ENTRY` as a warning and left in place | 7 |
| REQ-DOM-3 | `xel`, `ana-maria` and `a1` are accepted; the aliases of `ALIAS_INVALID`'s test are refused; `layout.reservedNames` in a project file is `SETTINGS_INVALID` | 7 |
| REQ-DOM-4 | After project creation the file is exactly `local/` and a newline, its digest in the receipt; any change is `GITIGNORE_CHANGED` | 7, 10 |
| REQ-DOM-5 | After F-1, `local/tmp/` is empty, each attempt's directory having existed during it; `status` lists the evidence with digests equal to the record's; removing `local/cache/` and running any write command rebuilds it, while reading commands leave it absent; with `TMPDIR` set to an empty directory, the directory is still empty after F-1 | 9 |
| REQ-DOM-6 | `apply` in administer mode with no `AGENTS.md`, an existing one, one already holding the block (replaced in place, a second `apply` changing nothing), no `CLAUDE.md` (created as `@AGENTS.md` and a newline) and an existing one (block added): bytes outside the block unchanged in each | 10 |
| REQ-DOM-7 | In each mode `publication-decided` holds the mode and the words; in `own-repository` and `unpublished` the plan declares the exclude line and `apply` adds it; `DOMAIN_NOT_IGNORED`'s test; in `own-repository` the domain's repository receives the commits and the product's none | 7, 10 |

### REG

| Requirement | Accepted when | Phase |
| --- | --- | --- |
| REQ-REG-1 | Every event file of the flows is named by its six-digit `seq` and the first 16 hexadecimal digits of its SHA-256, in its stream's directory; identifiers match `^[pst]-[0-9a-f]{8}$` and derive from the creating payload; `RECORD_ID_COLLISION`'s test | 7 |
| REQ-REG-2 | A schema test finds exactly the listed fields in every event of the flows, `prev` null only at `seq` 1, `at` in RFC 3339 UTC, `say` only on person moves; replay gives the same state when the `at` values of a fixture are permuted | 7 |
| REQ-REG-3 | Property: any event value, its strings drawn from every Unicode scalar value, serializes canonically, parses back equal and re-serializes byte-identical; PD-04; `RECORD_NONCANONICAL`'s test | 7 |
| REQ-REG-4 | Every person's event of the flows has a `.sig` that carries the key the cited policy gives its alias and that `ssh-keygen -Y check-novalidate -n fabrewk` accepts (D-048); the same key and role in the policy in force at its place, a maestro's concurrent work citing either policy accepted, and a seal its stream does not hold `RECORD_ROLLBACK` (D-050); the crash suite finds no event file in place without its signature; `RECORD_SIGNATURE_INVALID`'s test | 7 |
| REQ-REG-5 | Two write commands started together: one runs, the other refuses `LOCK_HELD` at once; a head moved between read and append (`fault-injection`) fails the append; a kill during a write leaves only a temporary file whose name starts with `.`, which readers skip | 7 |
| REQ-REG-6 | One fixture per code it lists, each refused by that code; property: random byte damage to any event file is refused by one of its codes, never accepted and never a panic | 7 |
| REQ-REG-7 | PD-05 | 7 |
| REQ-REG-8 | PD-01, PD-02; reading commands leave the mark byte-identical | 7 |
| REQ-REG-9 | PD-03 | 7 |
| REQ-REG-10 | In `published`, each write command adds exactly one record commit, holding only the Fabrewk files it changed, one trailer per event, preceded only by the alambique's commit for `approve` and the integration's; killed between the two, the events are `RECORD_UNCOMMITTED` at `check` and the next write command commits them first; PD-41's staged and unstaged edits byte-identical after; a head moved at an untouched file: the commit rebuilt on it; at a touched file: `RECORD_COMMIT_RACE`, events kept, `RECORD_UNCOMMITTED` at `check`; PD-46's replay failure commits nothing; in `unpublished`, no commit | 7 |
| REQ-REG-11 | With branch `trunk` checked out, F-1 integrates into `trunk`; `HEAD_DETACHED`'s test | 7, 9 |
| REQ-REG-12 | The first write command with an empty private folder creates an ed25519 key with no passphrase; every event's `instance` is its fingerprint; `config` says nothing verifies it; `status`, `config`, `check` and `assess` run first leave the private folder and `local/` byte-identical | 7 |
| REQ-REG-13 | PD-28, PD-29 | 9 |

### SET

| Requirement | Accepted when | Phase |
| --- | --- | --- |
| REQ-SET-1 | The three files are read as TOML 1.1 and are byte-identical after every flow; TOML-like text in a fermento or the alambique changes no effective value | 7 |
| REQ-SET-2 | A test walks every declared key and finds its type, levels, default, direction and description; for each key, a wrong type, a value outside its set and the key at each level not allowed are refused with file, key path and line; an unknown key is refused | 7 |
| REQ-SET-3 | Every non-personal key in a personal file is `SETTINGS_KEY_NOT_PERSONAL`; every personal key is accepted; `ALIAS_MISMATCH`'s test | 7 |
| REQ-SET-4 | In each mode the policy in force equals the last `policy-adopted` content whatever the file says; an edit raising `limits.rounds` grants no round until adopted and shows as pending; `POLICY_UNCOMMITTED`'s and `POLICY_ABSENT`'s tests, the latter for every command that needs a policy | 7 |
| REQ-SET-5 | For every declared key, a change each way is classed by its direction; adding, removing or changing a maintainer or a role is a widening; `adopt` refuses until every widening is named (`WIDENING_UNNAMED`) and the event lists those named | 7 |
| REQ-SET-6 | Property over random values at every level: the effective value follows the precedence; a stage or task widening without `--widen` is `WIDENING_UNNAMED`, and with it is shown as a widening in `config` and in the event; personal values are untouched by stages and tasks | 7 |
| REQ-SET-7 | `config` and `config <key>` print one listed source per value, the adopted digest for `project`, and pending changes apart with file and line; `--json` carries the same | 7 |
| REQ-SET-8 | A tool removed from `PATH` between two commands leaves the effective set at the second; `detected.json` is dated; no settings file is written; each value is marked observed, verified by use or declared; permitted but absent, and present but not permitted, are both outside the effective set | 7 |
| REQ-SET-9 | Each invalid profile (tool not permitted or absent, provider not permitted, model of another provider, effort not the model's, required tool absent) is refused before any dispatch; observed model and effort are recorded apart from the requested; an unsupported permission mode refuses and the fake CLI shows it was never called | 7 (validation), 9 (dispatch) |
| REQ-SET-10 | A key missing in `es-MX` falls back to `es`, then `en`; a tag that is not BCP 47 is `SETTINGS_INVALID`; every AD-3 key is read where AD-3 says | 7 |

### ID

| Requirement | Accepted when | Phase |
| --- | --- | --- |
| REQ-ID-1 | Maintainer tables with each field parse; a missing `role` or `key` is `SETTINGS_INVALID`; `area` outside a supervisor is `SETTINGS_INVALID` | 7 |
| REQ-ID-2 | After every adoption, `allowed_signers` equals a fresh derivation from the policy in force; a key added to it by hand verifies nothing | 7 |
| REQ-ID-3 | A maestro makes every person move of section 4; a contributor and a key with no role make none (`ROLE_FORBIDDEN`, each move tested); `ROLE_NOT_ENFORCED`'s test | 7 |
| REQ-ID-4 | Every signed event of the flows records alias and role; editing a personal file and the private folder, then replaying, changes no role | 7 |
| REQ-ID-5 | `config` prints each key's type, and touch enforcement as `unknown` for ed25519, `sk-ssh-ed25519` and `sk-ecdsa-sha2-nistp256` keys unless the verification itself requires touch | 7 |
| REQ-ID-6 | `apply` records the plan's first maestro and key, the plan states trust on first use, and `apply` with another key is `PLAN_CHANGED` | 10 |

### TASK

| Requirement | Accepted when | Phase |
| --- | --- | --- |
| REQ-TASK-1 | A test compares the code's table with a rendering of section 4 pinned when phase 7 starts; a move added only to the table appears in help, replay and the reference documentation; for each row, two guards made to fail at once report the first listed; `MOVE_NOT_ALLOWED`'s test | 7 |
| REQ-TASK-2 | A task missing any field is `TASK_INVALID`; an authorization binds the identifier, the fields' digest and the head; a change to each field in turn makes `authorize` refuse `TASK_CHANGED`; PD-06 | 7 |
| REQ-TASK-3 | Authorizing a stage records each task's digest and head, and for the dependent task its dependencies; its `run-started` records the base fixed after the dependency's integration; F-1 runs its tasks one at a time; `DEPENDENCY_OPEN`'s test; `run` of a second task while one runs is `LOCK_HELD` | 7, 9 |
| REQ-TASK-4 | PD-30, PD-31, PD-32; `AGENT_ENVIRONMENT`'s test | 7 |
| REQ-TASK-5 | Each halting code provoked leaves the task `halted` with exit 4; every command but `authorize`, `reject` and, with the branch at the candidate, `accept` on a halted task is `MOVE_NOT_ALLOWED`; `NO_PROGRESS`'s test; with `rounds = 3` and findings 2, 2, 2, the halt is `NO_PROGRESS`, not `ROUNDS_EXHAUSTED` | 9 |
| REQ-TASK-6 | `close` refuses for each unmet condition with its code and succeeds when all are met; `stage-closed` lists the sweeps it relied on | 8 |
| REQ-TASK-7 | PD-15, PD-17; a pattern of 33 segments, one with a leading `/` and one with `..` are `TASK_INVALID`; empty and `.` segments are dropped before matching | 7 |

### RUN

| Requirement | Accepted when | Phase |
| --- | --- | --- |
| REQ-RUN-1 | PD-10; every attempt of the flows has exactly one start and one end | 9 |
| REQ-RUN-2 | Each check before dispatch provoked in turn (invalid profile, `TOOL_MISSING`, `BUDGET_EXHAUSTED`, `ROUNDS_EXHAUSTED`) leaves the fake CLI uncalled | 9 |
| REQ-RUN-3 | PD-15 to PD-22 | 9 |
| REQ-RUN-4 | PD-45; the candidate is on `fabrewk/<task-id>` | 9 |
| REQ-RUN-5 | PD-08, PD-09, PD-10; the crash suite | 9 |
| REQ-RUN-6 | PD-11, PD-21; `TOOL_FAILED`'s test | 9 |
| REQ-RUN-7 | PD-39; `LIMIT_UNOBSERVABLE`'s test | 9 |
| REQ-RUN-8 | The fake agent stores the packet it received: a schema check finds only the listed parts, and in round 2 the previous findings and the failed checks' output; `REVIEW_INVALID`'s test | 9 |
| REQ-RUN-9 | A verified task whose evidence went stale, and an accepted task returned by `authorize`: `run` dispatches checks and a review on the same candidate and no builder (counted by the fakes' invocations), and the round counts against the limit; with the branch at the candidate, a revalidation that spends the last round, then stale evidence again: `run` refuses `ROUNDS_EXHAUSTED`, `authorize` grants a round, and the task reaches `integrated` | 9 |

### GATE

| Requirement | Accepted when | Phase |
| --- | --- | --- |
| REQ-GATE-1 | PD-23, PD-24 | 9 |
| REQ-GATE-2 | A probe check finds a fresh export and no network unless its definition declares it; a check that sleeps past its limit is killed and counts as not passed; exit 1, a timeout, a signal and an unknown result each count as not passed | 9 |
| REQ-GATE-3 | `EVIDENCE_STALE`'s test, one input at a time | 9 |
| REQ-GATE-4 | The reviewer is dispatched only after every check passes, on the same candidate, as a new attempt; `off`, `prefer` and `require` each with one and with two providers give the recorded limitation, the halt or the review as REQ-GATE-4 says; `--widen review.differentProvider` lets a halted task continue with the limitation recorded; PD-25, PD-26 | 9 |
| REQ-GATE-5 | A task becomes `verified` only with every required check current and a review accept bound to the same tree and requirements digest; an accept bound to another tree or digest does not count | 9 |
| REQ-GATE-6 | A reject and a failed check each start the next round with no person's move; a failed check dispatches no reviewer in its round and counts one finding | 9 |
| REQ-GATE-7 | PD-37; the `task-accepted` event binds every field listed | 9 |
| REQ-GATE-8 | PD-11 to PD-14, PD-44, and the integration guard of PD-16; in `published`, the integration commit has the head as its only parent, the candidate's product tree, the domain of the head, and the trailers listed; with the head at the base itself, the branch moves to the candidate | 9 |
| REQ-GATE-9 | `PROTECTED_PATH_WILDCARD`'s test; the default list holds every file a check's command names | 7 |
| REQ-GATE-10 | A policy adopted, or a named tool's version changed (`fault-injection`), between verification and the move: the move refuses `EVIDENCE_STALE` | 9 |

### FER

| Requirement | Accepted when | Phase |
| --- | --- | --- |
| REQ-FER-1 | The three separator codes' tests; property: over random documents with backtick and tilde fences (closed, unclosed, longer closers, indented up to three spaces) and markers, the finder agrees with a reference fence scanner written from CommonMark's rules; every refusal writes nothing | 8 |
| REQ-FER-2 | Points are found as top-level numbered items outside fences; a numbered line inside a fence, a nested item and text under a heading are not points; `10.` follows `9.` | 8 |
| REQ-FER-3 | A valid answer records the events and changes only the text below the separator (the input's digest equal before and after); points gone withdraw their open proposals; the tests of `POINT_CHANGED`, `INPUT_UNCOMMITTED` and `INPUT_CHANGED` | 8 |
| REQ-FER-4 | The answer regenerated from the events equals the one written; it is in the maintainer's interaction language; each proposal shows its state and the command to approve or discard it, its identifier only after that | 8 |
| REQ-FER-5 | `approve` applies the edit, the result's digest is the predicted one, `proposal-incorporated` is recorded and the alambique is committed alone with the maestro as author; the tests of `PROPOSAL_STALE` and `ALAMBIQUE_CHANGED`; `discard` records `proposal-discarded` | 8 |
| REQ-FER-6 | The opening, a section and a point are each replaced whole; `UNIT_AMBIGUOUS`'s test; a status key changed by hand outside an edit is `ALAMBIQUE_UNAPPROVED`; a `rules` proposal adds, replaces and removes whole rules, is recorded in the stream `rules`, leaves the alambique and its digest untouched, and `RULES.md` follows | 8 |
| REQ-FER-7 | `ALAMBIQUE_UNAPPROVED`'s test; `approve alambique` records the difference as `direct-edit-approved` and the check passes after it | 8 |
| REQ-FER-8 | A proposal state typed by hand into the fermento's answer is overwritten at the next sweep, and typed into the alambique is `ALAMBIQUE_UNAPPROVED`; the replay's states are unchanged in both | 8 |
| REQ-FER-9 | Uncited files of a maintainer's space made unreadable (mode `000`): every command, `check` included, succeeds; no event or view holds any text of an uncited file | 8 |

### VIEW

| Requirement | Accepted when | Phase |
| --- | --- | --- |
| REQ-VIEW-1 | For each task state and each mode, `STATUS.md` shows what REQ-VIEW-1 lists, the warning only in `unpublished`, and next steps that pass 11.2 rule 9; PD-33, PD-39, PD-40, PD-42, PD-43 | 8 |
| REQ-VIEW-2 | `RULES.md` equals the base rules plus every incorporated proposal that targets it, regenerated after each `approve` | 8 |
| REQ-VIEW-3 | After every write command of the flows each view equals a fresh generation; `VIEW_STALE`'s test | 8 |
| REQ-VIEW-4 | `status` and `config` succeed while another process holds the lock and a fake dispatch sleeps | 7 |

### CLI

| Requirement | Accepted when | Phase |
| --- | --- | --- |
| REQ-CLI-1 | Every command of the declaration appears in help, in the fish and POSIX completions and in the reference documentation, and the parser accepts exactly what the declaration lists; the fish completion file passes `fish --no-execute` | 7 |
| REQ-CLI-2 | PD-32, PD-34, PD-35, PD-36, PD-38; every key exists in `en` and `es` | 7 |
| REQ-CLI-3 | A test provokes each exit status, 10 included from a fake extension | 7, 9 |
| REQ-CLI-4 | Every reading command's `--json` validates against a pinned schema carrying a `schema` field; each refusal's code is in it; changing a document's fields without a new `schema` fails a test | 7 |

### INS

| Requirement | Accepted when | Phase |
| --- | --- | --- |
| REQ-INS-1 | `assess` with the repository, the home and the private folder mounted read-only and no network succeeds, and gives the plan it gives unconfined; fixtures holding each side effect listed (a hook in `.claude/` settings, a Codex setting, automatic updates, telemetry, a sync that commits, generated pre-approvals) are each reported | 10 |
| REQ-INS-2 | The plan holds every item listed; its identity is its content's digest; two assessments of an unchanged repository give one digest | 10 |
| REQ-INS-3 | `apply` writes the plan's files and records the events in the order of section 4, the receipt's digests equal to the files written; a file present that the plan did not write is never overwritten; the tests of `PLAN_CHANGED`, `VERDICT_NONE` and `MODE_NOT_AVAILABLE` | 10 |
| REQ-INS-4 | `onboard` on a pseudo-terminal with scripted answers in Spanish and English applies the plan only after explicit words; end of input or an empty answer applies nothing; `TERMINAL_REQUIRED`'s test | 10 |
| REQ-INS-5 | Not built in the first core: `MODE_NOT_AVAILABLE`'s test. When built: in review mode, a snapshot shows writes only under `.brewstillery/review/` | 10 |
| REQ-INS-6 | In `dash`, with a local release signed by a fixture key: exactly one file, `~/.local/bin/fabrewk`, is installed; a changed binary or signature installs nothing; the script has no option that skips verification and no `sudo`; the agent route's last command is `fabrewk assess` | 10 |
| REQ-INS-7 | `PLATFORM_UNSUPPORTED`'s test; the message lists the supported platforms | 10 |

### SEC

| Requirement | Accepted when | Phase |
| --- | --- | --- |
| REQ-SEC-1 | A probe run as an agent and as a check fails to read the repository's path, the domain, the private folder, the real home and the SSH agent's socket, and to write outside `/work`; a check has no network; `config` states the shared network as a residual risk; each adapter's machine rules are checked at launch (Codex refuses a directory holding other repositories); PD-22, PD-27 | 9 |
| REQ-SEC-2 | With `bwrap` absent, agents, checks and extensions each refuse `CONFINEMENT_UNAVAILABLE` and nothing is dispatched | 9 |
| REQ-SEC-3 | `CREDENTIAL_SUSPECTED`'s test at both places; this repository's `.brewstillery/` at the commit phase 7 starts from, this file included, passes the scan, while fixtures holding real-shaped keys fail it; uncited files in `<alias>/` made unreadable change nothing, and one named `id_rsa` is refused by its name; the message calls the control a heuristic; `Cargo.lock` holds no regular-expression crate | 7, 9 |
| REQ-SEC-4 | The private folder is created `0700`; `PRIVATE_FOLDER_PERMISSIONS`'s test | 7 |
| REQ-SEC-5 | Every guard of section 4 and every check C1 to C12 has a label, `config` lists them, and a test fails when a guard has none | 7 |
| REQ-SEC-6 | The fixture home's `.claude/` and `.codex/` settings are byte-identical after every flow; the suites, except the dispatch tests, pass with no network | 9, 10 |
| REQ-SEC-7 | Each adapter declares its markers and each drives `AGENT_ENVIRONMENT` | 7 (Claude Code), 9 |

### EXT

| Requirement | Accepted when | Phase |
| --- | --- | --- |
| REQ-EXT-1 | A permitted, present extension runs; not permitted or absent, `fabrewk <name>` is a usage error; `EXTENSION_DIGEST`'s test; a program `fabrewk-status` never runs for `fabrewk status` | 9 |
| REQ-EXT-2 | A fake extension sees the four variables and reads `"$FABREWK_BIN" status --json` | 9 |
| REQ-EXT-3 | A `note` and a `propose` request are applied with the extension's name and digest as actor and no signature; `EXTENSION_OUTPUT_INVALID`'s test; its standard error reaches the person | 9 |
| REQ-EXT-4 | A probe extension can read the repository and `.brewstillery/data/` and run `"$FABREWK_BIN" status --json`; it cannot write anywhere but `.brewstillery/local/ext/<name>/`, reach the private folder or the SSH agent's socket, or use the network unless permitted; with no backend, `CONFINEMENT_UNAVAILABLE` | 9 |
| REQ-EXT-5 | The core returns only after the extension exits, and the extension's confinement has no network unless `extensions.<name>.network` permits it | 9 |

### BUD

| Requirement | Accepted when | Phase |
| --- | --- | --- |
| REQ-BUD-1 | The counter's fixture tests of 11.6 pass; it reports per ring | 7 |
| REQ-BUD-2 | The maestro's freeze is recorded at phase 7's start after the skeleton is measured; a test fails when a ring, the crate counts or the binary's size passes its frozen limit | 7 |
| REQ-BUD-3 | `rust-version` equals the Fedora `rustc` version recorded at phase 7's start (AD-4), or a later one a maestro recorded; `edition = "2024"`; the suites pass built with that compiler, and with the newest stable once a build service exists; a release build for `aarch64-unknown-linux-musl` passes them | 7 |
| REQ-BUD-4 | A test compares the direct runtime dependencies with the list of approved reasons; every build uses `--locked`; `cargo deny check licenses advisories` passes with the allowlist | 7 |
| REQ-BUD-5 | `license = "MIT OR Apache-2.0"`, `LICENSE-MIT`, `LICENSE-APACHE` and `NOTICE` exist from the first code; a release's third-party notices equal a fresh generation from `Cargo.lock` | 7, 10 |

## 13. What each later phase must show

A phase's acceptance (plan rows 7 to 10) includes every criterion of section 12 assigned to it, every past-defect case
and refusal assigned to it in 11.3 and 11.4, the mutation rule of 11.6 for its code, and a clean clone (11.2 rule 7).
A criterion assigned to two phases is first met with what the earlier one builds and met whole in the later one.

- **Phase 7 (core, part 1).** First the skeleton and its counter, and the maestro's freeze of the budget. Then the
  record, settings, identity, the transition table and the command surface: REG, SET, ID, TASK (but TASK-5 and TASK-6),
  CLI, BUD, DOM-2 to DOM-5 and DOM-7 on fixture records, VIEW-4, SEC-3 at `check`, SEC-4, SEC-5; the crash suite on the
  record; PD-01 to PD-06, PD-10 (record), PD-15, PD-17, PD-24, PD-30 to PD-32, PD-34 to PD-38, PD-46 (record).
- **Phase 8 (core, part 2).** FER, VIEW-1 to VIEW-3, TASK-6; the views of PD-33 and PD-39; PD-40 to PD-43.
- **Phase 9 (adapters and confinement).** RUN (RUN-9 included), GATE, TASK-5, SEC-1, SEC-2, SEC-3 in the change set, SEC-7, EXT, SET-9's
  dispatch; PD-07 to PD-14, PD-16, PD-18 to PD-29, PD-33, PD-44 to PD-46; the flows F-1 to F-3 with fake tools; one real
  dispatch per provider, preflight first.
- **Phase 10 (installation).** INS, DOM-1, DOM-6, SEC-6, BUD-5's release notices; F-1 from a real `apply`.
- **Phases 11 and 12 (the pilot).** The pilot's specification picks its six provoked refusals (6.48, D-009) from 11.4
  and holds both arms to the same measures.
