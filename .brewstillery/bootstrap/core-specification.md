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
| §1 Install by script or by an agent's prompt | I | REQ-INS-6, REQ-INS-7 | Built in phase 10, in the installer's ring (AD-5, D-097). |
| §1 The first alambique in the onboarding | I | REQ-INS-8 | Added at phase 10's opening (D-066, D-097). |
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
  `data/`, `local/`, `skills/` (the skills the maestros approved, §2; amended by D-060), one directory per alias,
  `review/` in review mode, and, in `own-repository`, `.git`, the directory of its own repository (REQ-DOM-7; amended
  by D-091). Fabrewk creates nothing else; the core reads nothing inside `skills/`. `check` reports
  any other entry at the domain's top as a warning (`DOMAIN_UNKNOWN_ENTRY`); it does not delete it. (§2, AD-8, AD-12)
- **REQ-DOM-3.** Reserved names: `ALAMBIQUE.md`, `STATUS.md`, `RULES.md`, `settings.toml`, `.gitignore`, `data`,
  `local`, `review`, `bootstrap`, `skills` (D-060). An alias matches `^[a-z][a-z0-9]*(-[a-z0-9]+)*$`, has at most 32 characters and is
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
files as files, `data/`, `local/`, `skills/` (D-060) and each alias as directories, and `.git` as a directory only
while the publication mode in force is `own-repository`, D-091); a link is never followed and is an unknown entry. A
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
  16 bytes (`s-…`, `t-…`, `p-…`); `check` refuses two subjects with one identifier (`RECORD_ID_COLLISION`). A proposal's identifier is instead
  the first 8 hexadecimal digits of the digest of its fermento's alias and its payload without `id`, so that the same
  edit swept again is the proposal already recorded and two maintainers' identical edits are two (part 8a, P8A-3). (6.26,
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

**The dispatch, as phase 9's opening amended it (D-075, D-076; `confinement-design.md`, sections 3.8 and 4).**

- **REQ-RUN-2 (S-6).** A required tool is present for a dispatch only when the role's cell shows it: its executable,
  as it resolves, lies under the system directories the cell holds. Otherwise `TOOL_MISSING`, whose words say that the
  tool lives outside what the confinement shows.
- **REQ-RUN-3 (S-2).** Nothing is read from an attempt's directories until every process of its cell has ended.
  Fabrewk walks the copy itself and follows nothing: a directory is entered, a regular file read, a link's target taken
  as written; any other entry (a pipe, a socket, a device) is never opened and, outside the base's ignored paths, is
  refused (`LANE_VIOLATION`). An entry named `.git` or `.brewstillery`, of any kind and at any depth, is refused
  before any Git command sees the copy (`PROTECTED_PATH`), and so is a name Git's protections for other file systems
  refuse. Contents are hashed without filters or conversions, and the temporary index is built from those
  identifiers; no command that reads the copy's `.gitattributes`, `.gitignore` or `.gitmodules` runs on it. The
  base's ignore rules are the base tree's own files, never the copy's. A changed path that is not valid text is
  refused (`LANE_VIOLATION`). A reviewer's and a check's copies are deleted unread.
- **REQ-RUN-7 (S-5).** Time is Fabrewk's own measure, from the start of the attempt's process to the end Fabrewk
  observed; tokens are the tool's report; each is unknown, never zero, when not observed. `attempt-ended`'s `usage`
  is exactly `seconds` (an integer, or `null` when Fabrewk did not observe the end, as for an attempt ended `unknown`
  by a later command) and `tokens` (`null`, or exactly `input`, `output` and `cached`, each an integer or `null`).
  Against the `minutes` limit an unknown duration counts as the whole time the attempt was allowed. A view shows a
  sum with an unknown part as unknown.
- **REQ-RUN-10 (S-3; 6.4, 6.41, D-067).** Before each copy (an attempt's, a review's, a check's) Fabrewk reads the
  free space of the file system that holds `local/`, as the POSIX program `df -Pk` reports it, and refuses when it is
  below `limits.freeDiskMegabytes` (`DISK_LOW`); a free space that cannot be read refuses too. Before a dispatch
  `DISK_LOW` halts the task, as REQ-TASK-5's codes do. `df` joins the core's programs (REQ-SET-8).

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

**The gate, as phase 9's opening amended it (D-076; `confinement-design.md`, section 4).**

- **REQ-GATE-9 (S-4).** `gate.protectedNames` lists names protected at any depth: a changed path with a component
  equal to one of them must match an allowed-files entry that holds no wildcard; otherwise the change set refuses
  it, and the integration's recomputation again (`PROTECTED_PATH_WILDCARD`). The guard is on paths, not on patterns:
  `*.md` and `src/**` stay valid lanes and refuse only the change that reaches such a name. Its default is
  `.gitattributes`, `.gitmodules` and the names each built-in adapter declares for its tool's instructions (Claude
  Code: `CLAUDE.md`, `.claude`; Codex: `AGENTS.md`, `.codex`, `.agents`). `gate.protectedPaths`' default gains the
  directory the repository's `core.hooksPath` names when it lies inside the working tree, read at each guard.
- **REQ-REG-10, REQ-GATE-8 (S-11).** Every Git command Fabrewk runs names an empty hooks directory and turns the
  file-system monitor off. The working tree is brought to a candidate by writing each touched path from its object's
  bytes, with no filter, conversion or checkout command.

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
- **REQ-FER-10.** A sweep records, in `sweep-recorded`, the skills the tools load from outside the project: for each
  tool of the machine's profile, the names, never the content, found in the user folders its module declares in its
  contract (D-052; fake modules until phase 9). The answer shows them from the event and warns for each one that bears
  the name of a skill in `.brewstillery/skills/`; whether one contradicts the project's rules is for the integrator to
  say in its recommendation, since no code can judge it. Fabrewk touches none of them, and a folder that cannot be
  listed is shown as unknown, never as empty. (6.38; D-061, D-072)

**The alambique and the rules, as part 8b fixed them (D-073).** (1) A unit is found outside fenced code blocks and
spans from its first line to the end of its last line that is not blank: the opening from the start to the heading
`## 1.`; a section from `## <n>.` to the next heading of level 1 or 2; a point from `**<n>.<m>.` to the next point or
heading. Zero or several places, two units that overlap, or a rule to remove that does not exist, are
`UNIT_AMBIGUOUS`; every unit is found in the text as it stands, then all are replaced. (2) The text last approved is
the baseline, an incorporated proposal's result or a direct edit's new digest, whichever is last. With no file and
nothing approved, C7 passes; a file nothing approved, a changed one, or an approved one that is gone, is
`ALAMBIQUE_UNAPPROVED`; a file that cannot be read is `CHECK_UNKNOWN`. The stream `alambique` opens with its baseline,
once; `approve alambique` records it when there is none (the first text approved) and otherwise a
`direct-edit-approved` with the old and new digests and the difference, one hunk: the first line that differs (from 1),
the lines removed and those put, the equal ones at both ends left out; the old text is the record branch's when it is
the text approved, and `removed` is `null` (with all the new lines in `added`) when it is not. An alambique equal to
its last approved text is `MOVE_NOT_ALLOWED`. (3) `approve <proposal>` evaluates, after `MOVE_NOT_ALLOWED`, the role and
the words: `PROPOSAL_STALE` (the point with its number and the digest of its text in the fermento as it stands now;
for `rules`, the digest of the rules the replay gives), `ALAMBIQUE_CHANGED` (the file, the record branch's tip and the
person's index all hold the text the edit starts from), `ALAMBIQUE_UNAPPROVED` (that text is the one last approved),
`UNIT_AMBIGUOUS`, and `PROPOSAL_STALE` again when the drafted text does not give the result it predicted. The
alambique is written, and committed alone through a temporary index, as the person's Git names the author and with no
co-author, after the event and before the record commit; where the branch, and the index, already hold the file as it
stands (a direct edit the person committed) there is no such commit. The status keys of a proposal are shown with it;
the predicted digest binds the whole text. (4) The rules are `R<n>` and their text; their digest is that of their
canonical JSON, `{}` when there are none. A rules proposal adds, replaces (a text) or removes (`null`) whole rules, and
its `proposal-incorporated` in the stream `rules` carries the id, the result and all the rules after it; `RULES.md`
follows with the views (8c). (5) The replay holds both streams to this: an incorporated proposal belongs to the stream
of its target, starts from the digest that stream holds and gives its predicted result, and the rules recorded are the
units' (`RECORD_TRANSITION_INVALID`). (6) `ALAMBIQUE_UNAPPROVED` is evaluated by `run` before confinement; `accept`
(phase 9) and `close` (8c) call the same function.

**The views and the stage's close, as part 8c fixed them (D-074).** (1) `STATUS.md` and `RULES.md` are pure functions of
the replay and the catalogue, in the project's canonical documentation language (`en` with no policy): the same record
gives the same bytes, so a view holds no clock, no branch head and nothing read from outside the record; a time shown is
the `at` of the event that began a wait. (2) Every write command, once its events are written and the record replays,
writes both views (only a file that differs) and the record commit holds them with the events; a failure after the events
leaves them written and says the next write command regenerates the views. A view that is absent, not a regular file,
reached through a link or different from a fresh generation by one byte is `VIEW_STALE` at `check` (C5, effective); a
record that does not replay leaves C5 `CHECK_UNKNOWN`; the reading commands never regenerate. (3) `STATUS.md` holds the
record's counts, the policy in force, the publication mode and, only when it is `unpublished`, the warning; each stage
with its tasks, a halted task with its code; what waits, under each maestro and under the integrator; the last
integrated work; and each task's dispatches with its tokens as the tool reported them, unknown if any attempt did not
report or has not ended, zero only when reported. What waits: a proposed stage (a maestro authorizes it); a task
proposed into an authorized stage; an authorized task (run or reject); a halted or verified task (reject; accepting is
phase 9's); a pending proposal (approve or discard); a stage whose tasks have all ended: one sweep per maintainer whose
last sweep did not see every task's last event (the integrator) and, when none is missing, its close (a maestro). Each
wait lists the commands that exist for it, without words, and never `authorize`, which needs the digest `status <id>`
shows. (4) `status` lists the same waits, for whom and since when, in the person's language, and offers as next steps
only the commands of the person who reads: a maestro's to a maestro, the integrator's to whoever is no maintainer, none
to a maintainer with nothing to decide; its document is `fabrewk.status/2`, with `waiting` (`fabrewk.status/3` from part 9b, with `local`). (5) `RULES.md` holds the twelve
base rules of the binary's catalogue (D-074) then the project rules of the stream `rules`, in the order of their numbers.
(6) `close <stage> --say` is the maestro's move on an authorized stage: `MOVE_NOT_ALLOWED`, `ROLE_FORBIDDEN`, the words,
then the table's guards in order: a task neither integrated nor rejected (`STAGE_OPEN_TASKS`); a maintainer of the policy
whose last sweep did not record the last event of every task of the stage as its head (`SWEEP_MISSING`); the alambique
as last approved (`ALAMBIQUE_UNAPPROVED`); events of the record not committed where the mode commits
(`RECORD_UNCOMMITTED`). A stage with no task needs no sweep. The `stage-closed` payload binds the stage and its head, as
a rejection does, and lists, for each maintainer, the sweep it relied on (`stage`, `head`, `sweeps`: `alias` and the
sweep event's `head`). A task's identifier is a usage error, as is one the record does not hold. Its document is
`fabrewk.closed/1`.

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
a tool's own output) and the next step, with the command to run when there is one. Documents: `fabrewk.status/1` (`/2` since part 8c),
`fabrewk.subject/1` (`/2` since part 7d3b), `fabrewk.config/1` (`/2` since part 9a), `fabrewk.value/1`, `fabrewk.check/1`, `fabrewk.refused/1`,
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

**The write path, as part 7d3a fixed it (D-062).** The declaration gains a required argument (`propose <file>`, a
usage error without it) and the options that take a value, `--say <words>`, `--say-lang <tag>` and `--widen <key>`
(only `--widen` repeats; another repeated, a tag that is not BCP 47, or a missing value is a usage error). A value is
attached with `=` or is the next argument unless that one starts with `-`, so `--say --json` gives no words rather
than the words `--json`, and words that start with `-` are given as `--say=-…`; `--say` absent, given no value,
empty after trimming or not UTF-8 is `WORDS_MISSING`. Write commands take `--json` too (`fabrewk.adopted/1`,
`fabrewk.proposed/1`, each with the events written and the record commit); help and the reference list, after a
write command's own refusals, those of the write path. Every write command runs in this order: for a person's move,
`AGENT_ENVIRONMENT`; the record read without the lock (`RECORD_ABSENT` and the replay's codes); this machine's mark
(`RECORD_ROLLBACK`, `MARK_INVALID`); the policy in force (`POLICY_ABSENT`); the publication mode (a record with a
policy and no `publication-decided` is `RECORD_BROKEN`: `apply` writes both); `DOMAIN_NOT_IGNORED`; the product's
checked-out branch in every mode (`HEAD_DETACHED`, P7D3A-5); where the mode commits, a lock a stopped Git left
(`TOOL_FAILED`, P7D3A-7) and a Git that cannot name the commit's author or committer (`TOOL_FAILED`, P7D3A-14); the private folder's place and permissions, a folder not made yet placed where the links of
its existing ancestors lead (P7D3A-8), and the machine file (`SETTINGS_INVALID`, P7D3A-2); the command's own guards
on that record; then the session (`NOT_A_REPOSITORY`, the
record branch where the mode commits, the history walk, `LOCK_HELD`), the record replayed again under the lock and
the command's guards again on it; on a machine with no machine key yet, the key made under a temporary name; every
event made and signed in memory, an adoption's seal read just before it; then the machine key put in place
(REQ-REG-12), so that a key that cannot sign is `TOOL_FAILED` with the private folder, a key literal's copy and its
directory all as they were (P7D3A-1); the record commit of what an earlier command left uncommitted, taken only from
the streams replay reads (P7D3A-9); the events, each written by compare-and-swap after the head the plan saw
(`RECORD_HEAD_MOVED`); the replay before the record commit (PD-46), the commit (`RECORD_COMMIT_RACE`; rebuilt on a new head only when it
descends from the command's first one and its history is walked as the first one was, `RECORD_ROLLBACK` otherwise,
P7D3A-16, P7D3A-19) and, once it is made, the
mark (P7D3A-10); then, still under the lock (P7D3A-3), the other caches: `allowed_signers` from the
policy in force (REQ-ID-2) and `detected.json` with `git` and `ssh-keygen` verified by use (REQ-SET-8). A refusal decided before the session writes nothing; one decided after it,
only when the record changed in between, may leave the empty `local/lock`. Two things are no write in this sense
(D-063): cleaning what an interrupted write left (temporary files, signatures whose event never reached its place,
the record commit's temporary index, a machine key that does not read), which every write command does even when it
then refuses; and, after a race with a writer that ignores the lock (`RECORD_HEAD_MOVED`, `RECORD_COMMIT_RACE`), the
machine key the refused command made, which the next command uses. A record commit that fails, whatever the cause
(a race, a Git that refuses the ref's update), leaves the command's events written and not committed, as REQ-REG-10
says of a race; the refusal says so, `check` reports them, and the next write command commits them first (D-065). A person is the maintainer whose key Git's `user.signingkey`
names (D-055), refused `ROLE_FORBIDDEN` when Git names none, the key is no maintainer's, or the role may not make the
move; the signature is `ssh-keygen -Y sign -n fabrewk -f` over the private key file, the public key file (its
private key beside it or in the SSH agent) or, for a `key::` literal, a copy of it in `local/tmp/` removed after
(the agent). Every file `ssh-keygen` may open is held to D-055's rule, a regular file outside this repository and
every domain, as written and as it resolves: the file named, and, for a public key file, the private key beside it
when one exists; otherwise the key names nobody (P7D3A-4). The words are not an example any catalogue prints (`words.example`, which `--say`'s help shows),
compared after trimming (`WORDS_COPIED`); their tag is `--say-lang`, else the person's `languages.agentInteraction`.
`AGENT_ENVIRONMENT` is refused when any marker is set in the environment, whatever its value: those the binary
declares on each adapter's behalf (`CLAUDECODE`; `CODEX_SANDBOX`, `CODEX_SANDBOX_NETWORK_DISABLED`) and the core's
`FABREWK_EXTENSION` (REQ-SEC-7); `propose`, which grants nothing, is not refused there. REQ-RUN-5 is phase 9's: no
command of phase 7 opens an attempt. An internal error of a write command says that what it wrote stays in the
record. `adopt` cites the policy in force, checks the move's role in it, and records the file's content, digest, the
seal of every other stream's head read just before the append, and the widenings named; it may adopt a file equal
to the policy in force. `propose <file>` reads a regular file outside every Fabrewk domain, at most 1 MiB, as JSON
holding exactly `stage` (a stage's payload without its nonce, or the identifier of a stage of the record) and `tasks`
(one or more, each a task's payload without its nonce and stage, plus an optional `name` that is not shaped as a
task's identifier, by which a later task of the file names it among its dependencies); `propose` draws the nonces, again
for an identifier the record or the file already holds, and refuses `RECORD_ID_COLLISION` with nothing written after
eight draws (P7D3A-15); it records Fabrewk as the actor, and refuses every failure of the file's shape or content as `TASK_INVALID`; the
protected paths a task's wildcards are held to are the effective `gate.protectedPaths` with its stage's and its own
layer; a stage that is closed is `STAGE_CLOSED`. Its format is printed by `propose --help` and the reference. The
fault tests of the write path (`RECORD_HEAD_MOVED`, `RECORD_COMMIT_RACE`, PD-46) are in the binary's test `faults`,
built with the binary's feature `fault-injection`, which enables the core's.

**The task commands, as part 7d3b fixed it (D-053 point 4, D-062, D-068).** `authorize <id> --seen <digest> --say
<words> [--say-lang <tag>] [--widen <key>]… [--run]`, `reject <task> --say <words>` and `run <id> --say <words>` are
write commands (`fabrewk.authorized/1`, `fabrewk.rejected/1`; `run` gives none of its own until phase 9). An
identifier the record does not hold, or a stage's given to `reject`, is a usage error, and so is `authorize` without
`--seen`. After the write path's own guards each runs, in this order, the move from the subject's state
(`MOVE_NOT_ALLOWED`), the person (`ROLE_FORBIDDEN`) and their words (`WORDS_MISSING`, `WORDS_COPIED`), then the
row's guards in section 4's order. `status <id>` shows, for a subject the table lets an authorization move, what that
authorization binds: a title, then the subject's identifier and fields' digest, its stream's head and every field,
each text verbatim with its language tag (control characters, bidirectional marks and line separators written as
escapes, `\u` and four hex digits, every further line of a text indented; a catalogue's placeholder in a text is
never filled), the words around them in the person's language as the record planned on and their file give it, so
under the lock as they are then (P7D3B-1); for a stage, its
own fields and then each of its tasks that waits for an authorization (from `proposed`, `halted`, `verified` or
`accepted`), each after the tasks of the stage it depends on, else in the order of their identifiers. Its digest is
the SHA-256 of that text, printed after it with the command `fabrewk authorize <id> --seen <digest>` (without words,
PD-32) and given in `fabrewk.subject/2` as `shown` (`text`, `digest`); `TASK_CHANGED` is the rendering made under the
lock differing from the one `--seen` names, so a change of the fields, of a head, of a stage's tasks or of the
person's language refuses; its detail never gives the digest of what is shown now. The authorization binds each task's
identifier, fields' digest, head and limits as proposed, and records only the widenings its layers make (a key named
with `--widen` that widens nothing records nothing). `PROTECTED_PATH` holds each bound task's allowed files to the
lane guard; `PROTECTED_PATH_WILDCARD` to the protected paths in force with its stage's and its own layer;
`WIDENING_UNNAMED` checks the stage's layer and each task's against the policy in force; `LIMIT_UNOBSERVABLE` applies
a task's `tokens` or its stage's budget in tokens to both of its profiles; `CHECK_DUPLICATE`, for a stage, its checks
against the policy in force. `run <stage>` runs the stage's first task, in that order, that the table lets start
(`MOVE_NOT_ALLOWED` when none may); `run`'s guards are its dependencies integrated (`DEPENDENCY_OPEN`), no task
`running` (`LOCK_HELD`, REQ-TASK-3), then confinement, which refuses `CONFINEMENT_UNAVAILABLE` until phase 9, so
nothing is written; the policy in force is the write path's guard, `BASE_MOVED` and REQ-RUN-2 are evaluated from
phase 9 with the modules and the integration, and the alambique's approval, after confinement, with phase 8 (D-068).
`authorize --run` evaluates the run's guards on the states the authorization leaves before writing either, so until
phase 9 it writes nothing. `reject` refuses `MERGE_IN_PROGRESS` whenever a `task-accepted` follows the task's last
`candidate-recorded`, which closes by default until phase 9 can tell where the record branch is (D-068). Every human
output reaches the terminal with each such character but the line break written as that escape, whatever field of the
record it comes from (P7D3B-4), and the goals `status`, `propose` and `authorize` print are indented as the rendering's
texts are (P7D3B-2); the documents carry each text as it is, JSON escaping it.

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
- **REQ-INS-8 (added at phase 10's opening, D-066, D-097).** When `.brewstillery/ALAMBIQUE.md` is absent, the plan
  creates it from the catalogue's skeleton in the canonical documentation language: a title naming the project; an
  opening that explains, pedagogically, the analogy that names everything (Brewstillery, the distillery, is Fabrewk's
  domain in the project; each maestro prepares a fermento with what they want to add, remove or discuss; the alambique
  is where the fermentos are poured to distil what the project must be; only maestros approve what enters); then
  numbered sections with stable titles, what the project must achieve, for whom, and what it leaves out, each holding
  the person's answer exactly as given, or the catalogue's placeholder line when the answer is empty. Fabrewk adds
  nothing of its own. The whole text is in the plan, so the person reads it before confirming, and `apply`'s
  `alambique-baseline` records its digest: it is the approved alambique of departure. An alambique present is kept
  byte for byte and its digest is the baseline. On the agent's route the agent asks the same three questions in chat
  and passes the answers as given, drafting nothing. (§1, D-066)

**Installation, as phase 10's opening fixed it (D-097).** What the parts of phase 10 build; Codex reviews it before any
code. Where a point below and a sentence above differ, the point holds.

1. **Where it lives.** The installation is a built-in module in its own ring, as AD-5 decided: `crates/install`, the
   ring `install` of kind `Installer` (1,200 counted lines), wired by the binary as the adapters are. The core keeps
   what every module relies on: the declaration of `assess`, `apply` and `onboard` (help, completions, the reference
   documentation and the catalogue come from it, REQ-CLI-1), their dispatch to the module through a contract the core
   declares, the record (the events, their payloads' validation, the signatures, the lock, the record commit, through
   the one write path of part 7d3a), the views and the codes. The module holds what is the installation's own: what
   `assess` reads, the verdict's rules, the plan, its files' contents and its digest, and `onboard`'s interview. The
   core's ceiling is 12,500 (D-097).
2. **The commands.** `fabrewk assess [--mode adopt|review] [--publication published|own-repository|unpublished]
   [--alias <alias>] [--key <file>] [--tool <name>]… [--lang <tag>] [--purpose <text>] [--audience <text>] [--excluded
   <text>] [--json]` reads; `--mode` defaults to `adopt`, `--lang` (the canonical documentation language and the first
   maestro's `agentInteraction`) to `en`; `--tool` names a built-in adapter (`claude`, `codex`), any other name a usage
   error that lists them; `--key` is a file holding one OpenSSH public key. `fabrewk apply <plan-digest> <the same
   options> --say <words> [--say-lang <tag>] [--json]` writes; there `--publication`, `--alias` and `--key` are
   required (a usage error without them, exit 2). `fabrewk onboard` takes no option but `--help`. `assess` ends with
   the exact `apply` command its plan corresponds to, each value in single quotes. When no value holds a single
   quote or a backslash, that one command serves fish and a POSIX shell alike; otherwise two are printed, each named:
   for fish, with `\` written `\\` and `'` written `\'` inside the quotes; for a POSIX shell, with `'` written `'\''`
   (fish reads a backslash inside single quotes, a POSIX shell does not; P10O-9). Section 11.2 rule 9 and PD-38
   check a command named for one shell in that shell alone (P10O-13).
3. **What `assess` reads** (REQ-INS-1, a closed list the module declares). It runs at the top of the main working
   tree of a repository that is not bare (`NOT_A_REPOSITORY` otherwise, as REQ-DOM-7's check). It reads: Git's state
   (the checked-out branch or a detached head; an operation in progress, merge, rebase, cherry-pick, revert or bisect;
   whether tracked files have changes); the names and kinds of the root's entries; the contents of `AGENTS.md`,
   `CLAUDE.md` and the licence file (`LICENSE*`, `COPYING*`, `UNLICENSE`); the top entries of `.brewstillery/` and its
   `ALAMBIQUE.md`; another harness by its names (a list the module declares, such as `.claude/`, `.codex/`, `.cursor/`,
   `.github/copilot-instructions.md`, `GEMINI.md`, `.aider.conf.yml`, `.clinerules`); the side effects in the settings
   of the catalogued tools, in the repository (`.claude/settings.json`, `.claude/settings.local.json`, `.mcp.json`,
   `.codex/config.toml`) and in the home (`~/.claude/settings.json`, `~/.codex/config.toml`): hooks, automatic
   updates, telemetry and error reports, MCP servers, tools approved in advance (permission allow lists, an approval
   policy that never asks, a sandbox turned off) and hooks or commands that run `git commit` or `git push`; the
   `--key` file; Git's `gpg.format` and `user.signingkey`, to say whether Git signs with that key; whether `AGENTS.md`
   or `CLAUDE.md` has staged or unstaged changes; and every target of point 5, by kind (absent, regular file, link,
   directory) and, for a regular file, by content (P10O-7). Each side
   effect is reported by file and key path, with a count where it is a list, never with a value, which may hold a
   credential. A file over 1 MiB is reported by name and size and not read. `.brewstillery/local/` is never an input
   (it is unversioned, and `apply`'s lock lives there), and `.brewstillery/data/` is one only when it holds an event:
   the root's entries and the domain's top entries are read without `local/` and without a `data/` that holds only
   directories and signatures with no event beside them (what a killed `apply` leaves before its first event), and a
   `.brewstillery/` left with no other entry reads as absent (P10O-10). Under the lock, the session removes those
   leftovers as it removes any orphan signature. Nothing else: not the private folder, not the network, no
   cache, no file written.
4. **The verdict** (REQ-INS-2). `none` when any of these holds, each a reason: a detached head; an operation in
   progress; a record in `.brewstillery/data/`, that is, at least one event (Fabrewk is already there; point 7 says
   what `apply` does with its own); a target of point 5 to `create` present in any form (a file, a link, a directory), the alambique excepted,
   which is kept; `AGENTS.md` or `CLAUDE.md` present but not a regular file, not UTF-8, over 1 MiB (P10O-8), or with
   staged or unstaged changes, which the person commits or puts aside first (P10O-3). Otherwise the verdict is `--mode`'s, `adopt` or `review`. Another
   harness, side effects, changes in tracked files and a missing licence are reasons shown so that the maestro decides
   informed, never blockers. A plan missing `--publication`, `--alias` or `--key` is shown as incomplete, naming what
   it lacks; its verdict stands, and `apply` requires them (point 2). An alias that is invalid or reserved is
   `ALIAS_INVALID`; a key file that is not one public key makes the project file invalid, which `apply` refuses
   (`SETTINGS_INVALID`) and `assess` shows as a reason.
5. **The plan** (`fabrewk.plan/1`): a canonical JSON document, as the record's events are (REQ-REG-3); its identity is
   the SHA-256 of those bytes. It holds no time, no nonce and no path outside the repository but the home's files
   read, written `~/…`. It holds the verdict and its reasons; the answers (the options); every input read, by path,
   with its digest or `absent`; the side effects; the warning of point 9 in the plan's language when the publication is
   `unpublished`; the next steps (Git's commands to sign with the key when Git does not, shown and never run); and,
   for `adopt`, every file with its action. The `apply` command of point 2 is printed beside the plan and is not part
   of its content, since it holds the digest:
   - `create`, with its exact content: `.brewstillery/settings.toml` (`[project] name`, the repository directory's
     name; `[maintainers.<alias>]` with `role = "maestro"` and the key; `[languages] canonicalDocumentationLanguage`;
     for each `--tool`, `[tools.<name>]` with its `adapter` and `providers`, and `[providers.<provider>] allowed =
     true`; no model and no profile, which a later `adopt` adds, as the next steps say); `.brewstillery/.gitignore`
     (`local/`, REQ-DOM-4); `.brewstillery/<alias>/settings.toml` (`[maintainer] alias`, `[languages]
     agentInteraction`); `.brewstillery/<alias>/fermento.md` (the catalogue's empty input in `--lang` and REQ-FER-1's
     separator block); `.brewstillery/ALAMBIQUE.md` when absent (REQ-INS-8); `CLAUDE.md` when absent (`@AGENTS.md`
     and a newline, REQ-DOM-6);
   - `keep`: an alambique present;
   - `block`: `AGENTS.md` (created holding only the block when absent) and a `CLAUDE.md` present, with the file's
     bytes after and the lines added or replaced; the block goes at the end after one blank line, or replaces the one
     present in place (REQ-DOM-6);
   - `exclude`, in `own-repository` and `unpublished`: the line `/.brewstillery/` added to the file `git rev-parse
     --git-path info/exclude` names (REQ-DOM-7);
   - `repository`, in `own-repository`: `.brewstillery/` made a Git repository of its own on the product's branch
     name;
   - the views `STATUS.md` and `RULES.md`, by name only: they are generated from the events `apply` writes, as by every
     write command (REQ-VIEW-3).
   A plan for `review` holds no file: review mode is not built in the first core (REQ-INS-5).
6. **`apply`** (REQ-INS-3). It refuses inside an agent (`AGENT_ENVIRONMENT`), then reassesses with its options and
   refuses a digest other than the one given (`PLAN_CHANGED`), a verdict `none` (`VERDICT_NONE`) and `review`
   (`MODE_NOT_AVAILABLE`); the key Git signs with must be the plan's (`ROLE_FORBIDDEN` otherwise, D-055); the words
   are checked as for every move (`WORDS_MISSING`, `WORDS_COPIED`); the project file must validate
   (`SETTINGS_INVALID`). Then, under the domain's lock, in this order: `assessed`, with the verdict, the reasons, the
   plan and its digest, and `publication-decided`, with the mode, `warningShown` true when the plan held the warning,
   and the person's words; the plan's files, each written whole and atomically; the entry blocks; the domain's own
   repository or the exclude line; `policy-adopted`, as `adopt` writes it; `alambique-baseline`, with the alambique's
   digest; `installed`, with the receipt, every file written or changed and its digest after; then the views and the
   record commit. The four person's events (`assessed`, `publication-decided`, `policy-adopted`,
   `alambique-baseline`) are signed with the plan's key; `installed` is Fabrewk's and unsigned, as section 3 has it
   (P10O-2). In `published` the record commit holds everything `apply` wrote, the entry files included, whose working
   tree and index equal `HEAD` before (point 4), so that the commit keeps every other staged or unstaged change as
   REQ-REG-10 does; in `own-repository` the domain's repository's first commit holds the domain's files and the entry
   files are left in the product's working tree, uncommitted; in `unpublished` nothing is committed (F-2). `apply` says
   what it left uncommitted and the Git command that commits it, running none.

   **The core's contract for it (P10O-4).** The one write path gains an installation entry, used by `apply` alone: it
   runs with no record, or with one holding only an installation of the same plan (point 7); the publication mode,
   the policy and the signer come from the plan, not from the record; `policy-adopted`'s check that the project file
   is committed (`POLICY_UNCOMMITTED`) does not apply to the first adoption, whose file the same record commit holds;
   and the record commit takes the receipt's files besides the record's and the views. Every other guard of part
   7d3a stands: the lock, the mark, the record branch, the machine key, and the mode's checks once the exclude line or
   the domain's repository is in place.

   **Events before the first adoption (P10O-5).** `assessed` and `publication-decided` cite the digest of the project
   file the plan creates. Replay verifies them against the maintainers of the project file carried in the `assessed`
   event's plan, whose digest the citation must equal; once `policy-adopted` follows, part 7c2a's rule holds as
   before. A project stream with no `policy-adopted` replays only in that form, and only as an installation not
   complete.
7. **An interrupted `apply`.** A killed `apply` is finished by running it again with the same options. When the
   record holds an `assessed` with the given digest and the installation is not complete, `apply` does not reassess:
   it takes the plan from that event, checks that each target is either as the plan's inputs found it or as the plan
   writes it (`PLAN_CHANGED` otherwise, writing nothing), and writes only what is missing, the events included. The
   installation is complete once `installed` is recorded, `STATUS.md` and `RULES.md` are written (P10O-11), and, in a
   mode that commits, the record commit holding `installed` is made. Those are facts of the record, the domain's files
   and the record branch, never of the person's index, so that no later edit, staged or committed, makes a complete
   installation incomplete (P10O-15). The same `apply` finishes each of them: it writes a missing view and makes the
   missing commit. The receipt's committed paths are, in `published`, every path of the receipt; in `own-repository`,
   those under the domain, whose repository commits them, the product's entry files and its exclude file being left
   as point 6 says; in `unpublished`, none (P10O-14). Their entries in the person's index are written before the
   branch moves, never after (P10O-12, P10O-17, P10O-18): just before the record commit, `apply` stages each committed
   path of the receipt whose index entry carries no staged change (equal to `HEAD`'s, or absent where `HEAD` has
   none), with the bytes the commit takes; an entry that carries a staged change is the person's and is kept, staged
   against the new commit. The commit is then made as REQ-REG-10 makes it, its tree built apart from the person's
   index, and once the branch has moved `apply` changes no entry of those paths. A kill before the branch moves leaves
   the installation's files staged and not committed, which is what they are, and the same `apply` makes the commit; a
   kill after it leaves the index already at the commit for every path `apply` staged. So no recovery has to tell a
   stale entry from a staged one, and `sync_index` keeps covering the record's own files only. Until it is complete every other write command refuses (`INSTALLATION_INCOMPLETE`, new) and `check` reports
   it, so that no other command commits a part of it (P10O-6). Once it is complete, the same `apply` again says so,
   writes nothing, and exits 0;
   another plan meets point 4's record and is `VERDICT_NONE`. Section 12's "a second `apply` changing nothing" for
   REQ-DOM-6 is read so (P10O-1). Before `assessed` is written nothing is, but the lock under `local/` and what point
   3 reads as before (P10O-10). The crash suite runs over `apply`'s write points, the record commit's between the
   branch's move and the index's update among them.
8. **`onboard`** (REQ-INS-4). It opens the terminal itself (`/dev/tty`), since standard input is the script when it
   is piped into `sh`, and refuses when there is none (`TERMINAL_REQUIRED`: no controlling terminal; section 11.4's
   case is read so). It asks first the language (`en`, `es`), then, in that language: adopt or only review; whether
   the maintainers will publish `.brewstillery/`, showing point 9's warning for `unpublished` and offering the exits;
   the alias; the public key's file, showing Git's commands to sign with it when Git does not, running none; the
   catalogued tools; and, when no alambique exists, REQ-INS-8's three questions. Then it assesses with those answers,
   shows the verdict and the whole plan, the first alambique included, and asks whether to apply it: only the
   catalogue's word for yes (`yes`; `sí` or `si`) is a yes; then it asks the person's words, which are `apply`'s
   `--say`. Any other answer, an empty one, or the end of input, at any question, applies nothing, says so, and exits
   0. A yes runs `apply`'s own path in the same process with the same answers, so the two routes reach the same
   command (plan row 10).
9. **The publication warning** (6.27, AD-7 step 6; section 9 item 9). Shown in the plan and by `onboard` when the
   answer is `unpublished`: that the record stays on one computer, that other collaborators can neither see nor
   continue the work, that the maintainers take that on at their own risk and responsibility; the exits, publishing
   `.brewstillery/` with the product, and, as the last, a repository of its own for it (`own-repository`); and that
   their answer is recorded in their own words. Its text is in the catalogue, `en` and `es`, drafted by Codex in part
   10a; the maestro decides the final text (plan row 10).
10. **The script and the release** (REQ-INS-6, REQ-INS-7, BUD-5; part 10d). `install.sh` at the repository's root,
    POSIX `sh`. The supported platforms are those whose tests pass on them (6.46, AD-4): Linux on ARM64 alone today, so
    any other `uname` is `PLATFORM_UNSUPPORTED` with that list. A release is `fabrewk-<version>-linux-aarch64.tar.gz`
    (the musl binary, `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`, the third-party notices generated from `Cargo.lock`
    by a development tool under `tools/`, and the completion files), `SHA256SUMS`, and its signature by a maestro with
    `ssh-keygen -Y sign -n fabrewk-release`. The script names the version, the release's address and the allowed
    signers; it checks the signature, then the checksum; it installs `~/.local/bin/fabrewk` through a temporary file in
    that directory renamed into place, and nothing else (the completion files stay in the archive, and the README says
    where to put them); inside a repository with a terminal it offers `fabrewk onboard`. Its tests run a copy of the
    script whose address points to a local release signed by a fixture key. Publishing the repository and the release
    on GitHub is the maestro's decision and act.
11. **Documentation** (6.17, 6.45; part 10d). `README.md` (English) and `README.es-MX.md` at the root: what Fabrewk is,
    the two installation routes with the agent's preset prompt (AD-7 step 3), and the sentence of AP-3 on trusting the
    release source; the manuals in `docs/en/` and `docs/es-MX/`: installation and onboarding, working with Fabrewk, and
    the commands, linking the reference documentation generated in `crates/fabrewk/`. The first-line texts (the
    README, the warning, the interview's questions) are written by Codex, the technical ones by Claude, each checked
    for fidelity by the other (6.45, plan row 10).

**Amended in part 10a (D-100).** How `assess` reads Git's state, points 3 and 4 above read so, after Codex's second
pass (P10A-8 to P10A-12): `assess` runs no `git status`. First, before any other Git, it lists Git's includes without
following them (`git config --no-includes`); one that names a file of the private folder stops it
(`PRIVATE_FOLDER_INCLUDED`); an include nested inside an included file is a declared residual. Every Git it runs
fetches no missing object and runs no filter. `AGENTS.md` and `CLAUDE.md` are compared by Fabrewk: the index and
`HEAD` by object id, the working tree's bytes, at most 1 MiB, hashed by Git from standard input with the path's
attributes but no filter, so that an index flag (`assume-unchanged`, `skip-worktree`) hides no edit; one that goes
through a Git filter blocks with its own reason (`filtered`), since whether it holds changes cannot be told without
running the filter. Whether other tracked files have changes, a reason that informs, is told from the index's staged
changes and from each tracked file's size and time against what the index recorded, so no file's content is read.
The check of the private folder covers every file Git's configuration names that Git opens while `assess` runs, its
includes and its attributes file (`core.attributesFile`), in each file and in the `-c` values of the environment
(P10A-14, P10A-15). Point 3's limit of 1 MiB is on the files `assess` reads for their content; what Git reads to
answer about its own state (its index, objects, configuration and attributes) is Git's state (D-101). After Codex's
fourth pass (P10A-22 to P10A-27) every Git `assess` runs is isolated (D-102): only the repository's own
configuration, no global or system file, no `-c` value or trace the environment carries, no attributes file; that
replaces the checks of the global, system and environment's includes and attributes above. Before any Git in the
repository, the repository's configuration files are refused when they lie in the private folder or include a file
of it, and the person's global files when they lie there; from those global files, each read alone with no include
followed, `assess` takes only `user.signingkey`, `gpg.format` and `core.autocrlf`, the repository's own value
winning. A setting only an include of the global configuration holds, and a global attribute, are not seen: a
declared residual, which can make `assess` see a change that is not one, never hide one. What lies inside the Git
directory (its objects, alternates, index and hooks) is the person's own state, which a clone cannot plant: a link
there to the private folder is a declared residual, but for the repository's configuration and attributes file,
checked as above (D-103). Every file `assess` opens by itself is checked by where it really lies, and one in the
private folder is reported as `private` and not read; a key file there is unusable (`key-inside`).

### SEC: confinement, credentials and controls

- **REQ-SEC-1.** On Linux every agent and check runs under bubblewrap: system read-only, an empty home, only the tool's
  binary and its own credential, the exported copy writable at `/work`, the repository, `.brewstillery/`, the private
  folder and the SSH agent's socket invisible; network only for an agent's provider, never for a check unless REQ-GATE-2
  allows it. The shared network is a stated residual risk (amended by D-075: a private network, below). Each adapter carries its tool's machine rules and checks them
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

**Confinement, as phase 9's opening amended it (D-075, D-076; `confinement-design.md`, sections 3 and 4).**

- **REQ-SEC-1 (S-1).** An agent's network is a private namespace with outbound connections only: nothing of the
  host's loopback, addresses or abstract sockets is reachable from a cell. The open outbound network (the local
  network, the Internet) is the stated residual risk; this replaces "the shared network". `pasta` joins `bwrap`
  among the programs detected, and dispatch refuses where either is absent.
- **REQ-SEC-1 (S-10).** The writable places of an agent's or a check's cell are `/work`, its home and its `/tmp`, the
  attempt's three directories under `local/tmp/<attempt>/`, all deleted when it ends. An extension's home and `/tmp`
  lie inside `local/ext/<name>/`.
- **REQ-SEC-1 (S-12).** No process of a cell can call the kernel's keyring system calls, open a cryptographic socket
  or use `io_uring`; a filter loaded with the cell refuses them, in the cell and in what nests in it. What another
  kernel interface may do with a key's serial number is a stated residual risk.
- **REQ-SEC-5 (S-7, S-8).** Beside the controls and their labels, `config` lists the residual risks each backend and
  each adapter declares, worded in the catalogue. The lock's control against a directory swapped by an unconfined
  process of the same user (P8C-8) is cooperative and declared; no crate is added for it.

**The cell and its backend, as part 9a fixed them (`confinement-design.md` 3.2, 3.3 and 3.7's open point).**

- The contract is `crates/core/src/confine.rs`: a `Cell` (its home and `/tmp`, mounts read-only, writable or hidden,
  the network, whether it may nest, the whole environment, the directory and the command), a `Backend` (its programs,
  how it starts and ends a cell, its residual risks), and `Programs`, made only by `available`, which refuses
  `CONFINEMENT_UNAVAILABLE` where no backend exists, where a program it drives is absent, or where its probe fails,
  and runs the probe whenever a program is not verified by use (a backend that drives none is probed at every use).
  The runner keeps both channels in `local/evidence/<name>/stdout` and `stderr` (files `0600`, directories `0700`),
  each up to a number of bytes past which the cell is ended, ends the cell at its time, and returns only once every
  process of the cell has ended; a run that did not succeed is `TOOL_FAILED` naming its evidence directory, never its
  output. A run's directories are `local/tmp/<name>/home`, `tmp` and `work` (`0700`), removed following no link; when
  the backend cannot show that the cell ended, nothing of them is read or removed.
- How `bwrap` and `pasta` compose, settled by test (candidate (a)): `bwrap` is Fabrewk's direct child, started
  through `sh -c 'exec "$@" 3<"$0"'` only to open the filter on descriptor 3 (3.3 had it on standard input, which
  instead carries one line that releases the command), with `--die-with-parent`. Its command is a wrapper that reads
  that line, takes the cell's environment from a file Fabrewk wrote (`0600`, read-only inside at
  `/etc/cell-environment`, one single-quoted assignment per variable; a name that is not a shell variable's refuses),
  and becomes the command, with standard input on `/dev/null` and without `PWD`, `SHLVL` and `_`. `sh` and `bwrap`
  run on the host with an empty environment: 3.3's inheritance would have given the cell's variables, the dynamic
  loader's among them, to processes of the host before any confinement (P9A-4). For a cell
  with a network, `pasta` runs in the foreground as Fabrewk's second child, joins the wrapper's namespaces once it has
  replaced `bwrap`'s code, and the line is written once a default route through a device other than `lo` exists;
  Fabrewk ends `pasta` after the cell. After the direct child is ended, Fabrewk waits for the cell's first process to
  be gone, since its descriptors close before the other processes of its namespace are ended.
- Three consequences of that composition. `--dev` is not used, since it nests a second user namespace that `pasta`
  cannot configure: `/dev` holds `null`, `zero`, `full`, `random`, `urandom`, `tty` and the descriptor links, with no
  `/dev/pts` and no `/dev/shm`. A cell with a network cannot be kept from nesting, since `--disable-userns` nests one
  too: this backend refuses it (`CONFINEMENT_UNAVAILABLE`), which leaves a check's or an extension's network
  (REQ-GATE-2, `extensions.<name>.network`) to the parts that build them. `pasta` cannot watch the namespace on this
  machine (SELinux refuses it, and clears a parent-death signal across `pasta`'s change of domain), so after Fabrewk
  is killed during a run the cell ends with it but its `pasta` stays, idle: a residual risk `config` declares.
- What of `/etc` a cell sees, fixed by the probes: the certificate stores (`/etc/pki/ca-trust`, `/etc/pki/tls`,
  `/etc/ssl`, `/etc/ca-certificates`), `/etc/crypto-policies`, the dynamic loader's cache and configuration, the
  alternatives, each where it exists; and `passwd`, `group`, `hosts`, `nsswitch.conf` and `resolv.conf` written by
  Fabrewk, the last naming `169.254.0.53`, which `pasta` forwards to the host's resolver.
- `config` lists the residual risks (`fabrewk.config/2`, with `backend` and `residual`): the core's three (T-17,
  T-18, T-25 with P8C-8) and the backend's six (T-04, T-08, T-10, T-24, T-29, and the idle `pasta`). The adapters'
  (T-05, T-13, T-22) come with parts 9c and 9d. `run` keeps refusing `CONFINEMENT_UNAVAILABLE` at the command line
  until part 9e, which is also the first to keep a probe's pass in `local/cache/detected.json`.

**The copy and the candidate, as part 9b fixed them (`confinement-design.md` 3.8; REQ-RUN-3, REQ-RUN-4, REQ-RUN-10,
REQ-SEC-3, REQ-GATE-9, S-2, S-3, S-4, S-11).**

- The library is `crates/core/src/candidate.rs`: `export`, `changes`, `commit`, `free_space`, `protected_paths` and
  `protected_names`. `run` reaches none of them at the command line until part 9e, which records a failed export
  (`TOOL_FAILED`, PD-21) and `candidate-recorded`; the integration's recomputation is part 9g's.
- Every Git command Fabrewk runs names `/dev/null` as its hooks directory (no file can exist under it, so no hook is
  found) and `core.fsmonitor=false` (S-11). Tests that had a hook make Git fail now lock a ref's directory or pause the
  command (`record-moved`) while another Git takes the index's lock.
- The export writes the tree from the objects' bytes (`git cat-file --batch`), files `0644` or `0755` as the tree
  records them, links as links; a submodule's entry is an empty directory, unchanged while it stays empty. An entry of
  a mode Git does not write, a path with an empty, `.` or `..` component, or a path below a non-directory is
  `TOOL_FAILED`; so is a commit that is not 40 or 64 hexadecimal digits.
- A protected component (`.git`, `.brewstillery`, at any depth, in the export and in the copy) is compared under ASCII
  case folding, with HFS+'s ignorable code points removed, NTFS's trailing dots and spaces and alternate stream
  dropped, in each part between backslashes, and NTFS's short names `git~1` and `brewst~1`.
- The walk makes a directory or file the cell closed readable by its owner again, keeping the executable bit. The base's
  ignore rules are only the base tree's `.gitignore` files, written apart under the attempt's directory and read by
  `git check-ignore --no-index` in a repository of their own made with no template, `core.excludesFile` at
  `/dev/null` and `core.ignoreCase` off: neither the copy's rules nor the person's `info/exclude` or global excludes.
- The guards, in order: a protected component (`PROTECTED_PATH`, in the walk, before any Git command); an entry that
  is not a file, a link or a directory outside the ignored paths, a changed path that is not valid text, or one no
  allowed file matches, a deletion included (`LANE_VIOLATION`); a change that reaches a protected name or path and
  matches no allowed file without a wildcard (`PROTECTED_PATH_WILDCARD`); a change Git refuses to hold in a tree (a
  `.gitmodules` that is a link, or a name Windows or macOS reads as one under `core.protectNTFS` or
  `core.protectHFS`), found by writing the change set over the base into an index apart and reading it back, since
  Git leaves such a path out and still succeeds (`PROTECTED_PATH`); a changed file or link whose name, or a changed
  file whose content, looks like a credential (`CREDENTIAL_SUSPECTED`). Nothing is written into the object store
  until they pass: `commit` reads its own index back the same way first (`TOOL_FAILED`), then writes the objects.
- `gate.protectedNames` is compared with each component under ASCII case folding (a case-insensitive file system
  reads `claude.md` as `CLAUDE.md`). Its default lists the seven names of D-076, declared in the core's key on the
  adapters' behalf until parts 9c and 9d, as `main.rs` declares their markers (D-062).
- `gate.protectedPaths` gains the hooks directory always, not only at its default, so that no layer drops it: as
  written and where its links lead, read from the working tree's root as the kernel reads the path when Git runs a
  hook there (a link replaced before a `..` that follows it; a part that does not exist yet kept as written; the
  root's own links spending none of the 40, a root given relative read as the same root), each when inside the
  working tree, with each path its reading passes through, a link or a directory a `..` leaves (changing one to a
  link moves the directory; past the limit, those met up to the 41st link, since the kernel meets the same ones
  first), but those above the directory, which the guard covers already, and a component that is not text
  protecting the directory above it (`/` alone when it is the root, which protects
  every path). On paths, a change at a protected directory, under it, or at a path above it (a link or a file in the
  directory's place) reaches it. `propose` and `authorize` read it too.
- The candidate's message is `fabrewk: candidate for <task-id>`, a blank line, `Co-authored-by: <tool> (<provider>,
  <model>) <noreply@fabrewk.invalid>` (an address no mail reaches) and, with `git.signOff`, `Signed-off-by:` with the
  committer's name and address as Git gives them. A name with a character outside printable ASCII, or one of `<`, `>`,
  `(`, `)` and `,`, empty or longer than 100 bytes, is `TOOL_FAILED`, since a stream may report it. The branch
  `fabrewk/<task-id>` moves from the candidate the caller names, or from absent, by compare-and-swap; otherwise
  `TOOL_FAILED`.
- `DISK_LOW` reads `df -Pk -- <dir>` with an empty environment and `LC_ALL=C`: one line after the header, whose
  capacity follows three counts, the third the space available. A count is anything `df` may print there (digits,
  signed or not and of any length, or `-`) and a capacity digits with `%` or `-`, so that the real columns are
  always a reading; a line that reads so in more than one place (a name or a mount point that looks like them), or
  whose space available is not a count of kibibytes, cannot be read, and refuses. `limits.freeDiskMegabytes` is a key
  of the project file only. `status` shows the bytes `local/evidence/` and `local/tmp/` hold (`fabrewk.status/3`, with
  `local`: `evidence` and `tmp`, each a number or `null` when a directory cannot be listed).

**The adapter's contract and Claude Code's module, as part 9c fixed them (`confinement-design.md` 3.6 and 3.9;
REQ-RUN-7, REQ-RUN-8, REQ-SEC-1 for the adapter, REQ-SEC-6, REQ-SEC-7, REQ-GATE-4's audit, S-4, S-5).**

- The contract is `crates/core/src/adapter.rs`: an `Adapter` declares its tool's program (D-051), the provider's
  identity, its markers, its protected names, its residual risks, its credential, the variables its tool runs with,
  how to tell its release from the executable, its command per role, its preflight's commands and the reading of its
  stream. The core builds from it each role's cell, the packet, the role's result schema and its validation, the
  usage and the preflight; `resolve`, `dispatch`, `preflight`, `schema`, `validate`, `rules`, `packet` and `usage`.
  `run` reaches none of them at the command line until parts 9e and 9f. The first adapter is
  `crates/adapter-claude` (ring `ToolAdapter`, 600 lines), wired by the binary with its program, markers and residual
  risks; part 9d added Codex's and changed the contract (the next paragraph).
- The roles are `build` and `review`. A role's cell holds the copy writable at `/work`, the tool's release read-only at
  its own path, nested namespaces for the tool's sandbox, the variables `PATH`, `HOME`, `TMPDIR`, `LANG` and the
  adapter's; a dispatch adds the credential, bound read-only at the same place under `/home/agent`, and the private
  outbound network; the preflight has neither.
- The release is the directory `resolve` derives from the executable as it resolves; it must hold the executable and
  must not hold the person's home or the repository (`TOOL_MISSING`), so that no misplaced installation shows them to
  a cell. For Claude Code it is the directory named `versions` that holds the executable, as its installer lays it out;
  an executable elsewhere has none.
- The credential is a regular file at the path the adapter declares under the person's home (Claude Code:
  `.claude/.credentials.json`), reached through no link below the home, its own or a folder's; otherwise the dispatch is
  `TOOL_FAILED` before any process starts. The question of 3.6, by test: with the read-only bind, four real dispatches
  worked and the person's file kept its date, size and digest; the tool rewrites its credential when it renews it (the
  file's date is exactly its validity before its expiry), and inside a cell neither an append nor a replacement by
  rename can reach the file ("Read-only file system"). What a renewal inside a cell, which the tool can attempt only
  when the token is near its end, would do to the person's own login was not provoked, since it could end that login.
  The maestro closed it by refusing (D-079): before a dispatch Fabrewk reads only the credential's `expiresAt` and
  refuses (`TOOL_FAILED`, saying to renew the login) when it falls within the attempt's minutes; built with the dispatch
  in part 9e.
- The packet is a JSON document holding exactly `role`, `goal`, `requirements`, `allowed`, `rules`, `languages`
  (`delegation`, `code`) and `schema`, and from the second round on `previous` (`findings`, `failedChecks`); nothing
  else of the task reaches it. Its rules are the role's own instruction (catalogue `packet.<role>`), the base rules 8,
  11 and 12, the only ones that bind a dispatched agent (the others govern the domain, which no copy holds), and the
  project's own rules, in the delegation language with REQ-SET-10's fallback.
- A build's result is exactly `summary`, a text; a review's is exactly `verdict` (`accept`, `reject`) and `findings`,
  at most 50, each exactly `severity` (`blocking`, `minor`), `status` (`confirmed`, `suspected`), `summary` and
  `path`, a text or `null` (part 9d; optional until then); every text 1 to 4,000 characters. The core validates the result whatever the tool checked: a
  review that does not fit, a reject with no confirmed blocking finding, or an accept that holds one, is
  `REVIEW_INVALID`; a build's that does not fit is `TOOL_FAILED`. An accept with a confirmed blocking finding is
  refused because the verdict and the progress measure (REQ-GATE-6) would then disagree.
- Usage (S-5): `usage(seconds, tokens)`. Claude Code's tokens are its final result's `usage`: `input` is
  `input_tokens` plus `cache_creation_input_tokens`, `output` is `output_tokens`, `cached` is
  `cache_read_input_tokens`, each unknown unless a whole number. Its program reports tokens, so a token limit on its
  profiles is observable (REQ-RUN-7).
- Claude Code's launch, verified with 2.1.316: `--print --output-format stream-json --verbose`; its discovery off by
  `--safe-mode` (no `CLAUDE.md`, `AGENTS.md`, skills of the person's, plugins installed, hooks or MCP servers; its
  built-in plugin reported "project memory is off"), `--restricted` (no user, project or local settings file: the
  tool reported a project's settings in the copy ignored; file tools confined to the working directory),
  `--strict-mcp-config`, `--disable-slash-commands`, `--no-session-persistence`; `--permission-prompts none`; per
  role the tools (build: `Bash,Read,Edit,Write,Glob,Grep`, permission mode `acceptEdits`; review:
  `Bash,Read,Glob,Grep`, `dontAsk`); `--settings` with its sandbox enabled and required (`failIfUnavailable`), no
  command outside it, no domain allowed, the credential's folder unreadable to commands and to its file tools, no web
  tool; `--json-schema` with the role's schema; `--model` and `--effort`; the packet as the last argument, after
  `--`. Its variables turn off updates, telemetry, error reports and other traffic of its own (REQ-SEC-6). Inside a
  real dispatch the model's commands had a seccomp filter, no new privileges, only a loopback interface, the direct
  network unreachable and its proxy refusing every host, the credential unreadable, the home and `.gitconfig` not
  writable and `/work` writable. Its sandbox leaves an empty `/work/.claude/` directory, which no change set holds.
- The preflight runs each of the adapter's commands in each role's cell, with no network and no credential, each to end
  with status 0, or `CONFINEMENT_UNAVAILABLE` naming its evidence; it calls no model. Claude Code starts its sandbox
  only at the model's first command, so no command of the tool's makes it start without a model: the preflight runs
  the tool's version, `socat`, and a sandbox of the kind the tool makes, nested in the cell, creating a temporary file
  in the copy and in its own `/tmp`; at a dispatch, `failIfUnavailable` makes the tool refuse to run when its sandbox
  cannot start. Keeping a pass as verified by use is part 9e's.
- Claude Code's stream has one grammar (`crates/adapter-claude/src/stream.rs`): lines ended by a newline alone, a final
  newline not a blank line, each line one compact JSON object; `system` with subtype `init` first and only first, and
  later only with subtype `thinking_tokens` (the estimate a run that thinks writes); a `message` only on the assistant's
  and the user's turns; `assistant` turns of `text` and `thinking` blocks whose texts are texts and of `tool_use`
  blocks, each use with a new text identifier, a text name, an object input and, for `Bash`, a command that is a text
  with a simple command; `user` turns of one `tool_result` block, answering a use not answered yet by its text
  identifier, its content a text or a list of text blocks, with a boolean `is_error`, present when the use examines;
  `rate_limit_event`; one `result`, last, after every use was answered, its `is_error` a boolean and its `subtype` a
  text. Any other event, block, field type or order voids the whole stream: no result, model, tokens or examination. The
  result is the final event's `structured_output` when its subtype is `success` and `is_error` false; another subtype
  keeps the model and tokens and gives no result. Examination counts the uses answered without an error of `Bash` only
  when the command completed and printed something, as both the answer and the run's own record beside it (the user
  turn's `tool_use_result`: `interrupted` false, no `backgroundTaskId`, a `stdout` or `stderr` that is not blank) show
  it: the tool answers a silent command with `(Bash completed with no output)`, and an uncalled function, an empty loop
  or an unmatched branch runs nothing that shows, which no reading of the shell's grammar can tell; never when started
  in the background (`run_in_background`), whose answer says it started, not what it ran; a user turn holds one answer,
  so that the record is that answer's. Each counts by its simple commands (split on unquoted newlines, `;`, `&&`, `||`
  and `|`, a `#` that starts a word commenting out the rest of its line, a backslash before a newline joining two
  lines), and of `Read`, `Grep` and `Glob`; never `StructuredOutput`, the provider's answer tool; and only on success. A
  heredoc's lines count as commands: the count's size decides nothing, only whether it is zero. PD-26.2's sub-case is
  that split, which changes the count and voids nothing; PD-26.4's is every stream cut before its end; PD-26.6's a
  result of another subtype; PD-26.15 and PD-26.20 an error flag that is not a boolean. The recorded streams are
  `crates/adapter-claude/tests/streams/review.jsonl`, a real review, `build.jsonl`, a real build, and `silent.jsonl`, a
  real review that ran a silent command and one that printed, each with the account's rate-limit figures removed, their
  digests pinned; a fourth real stream, of the probe of the tool's sandbox, reads whole too and is not kept, since it
  holds the sandbox's proxy credential for that session.
- Claude Code's marker is `CLAUDECODE`; its protected names `CLAUDE.md` and `.claude`, which a test holds within
  `gate.protectedNames`' default, still declared whole in the core's key so that the key's default reads as one
  value; its residual risks T-05 (`tool-credential`), T-13 (`audit-limit`) and T-22 (`tool-flags`), in `config`. Its
  skills folder `.claude/skills` is listed by a sweep (REQ-FER-10).

**Codex's module, and what it changed in the contract, as part 9d fixed them (`confinement-design.md` 3.6 and 3.9;
REQ-SEC-1 for the adapter, REQ-SEC-6, REQ-SEC-7, REQ-GATE-4's audit; 6.38's rule of this machine; D-081).**

- The contract gains three things. An adapter declares its machine rule (`machine`): names that may not be found at
  any depth of the launch directory, under ASCII case folding and following no link; a dispatch whose copy holds one is
  `CONFINEMENT_UNAVAILABLE`, naming it, before the credential is looked at and before any process starts. Both adapters
  declare `.git`: a tool is never launched where another repository lies (6.38, where Codex's sandbox once left `.git`,
  `.codex`, `.agents` and `.aws` empty in a folder of repositories; 0.166.0 creates none of them, tested). A dispatch
  writes the role's output schema at `/home/agent/schema.json` in the cell (`adapter::SCHEMA`), for a tool that reads
  it from a file. And a review's finding holds `path` always, a text or `null`: every property of a role's schema is
  required, as OpenAI's strict structured output demands (the first real dispatch was refused for the optional
  `path`); the core's validation follows. The markers declared in the binary on Codex's behalf (D-062) are gone: each
  adapter declares its own, and `cli::main` takes the adapters and the backend only. The split of a command into its
  simple commands (`segments`) moved into the core, shared by both audits. Both grammars read each line, and Codex's
  its result's text, with `adapter::json`: JSON as `serde_json` reads it, except that an object naming a member twice
  is no value, since `serde_json` keeps only the last and a stream could hide an anomaly behind it (P9D-1); it uses
  `serde`'s traits, already approved and in the release build (D-036).
- The module is `crates/adapter-codex` (ring `ToolAdapter`, 600 lines). Program `codex`, adapter name `codex`, version
  by `--version`, tokens reported; provider `openai`; markers `CODEX_SANDBOX` (its sandbox's on macOS, read from the
  binary), `CODEX_SANDBOX_NETWORK_DISABLED` and `CODEX_THREAD_ID` (both seen inside its sandbox in a real dispatch);
  protected names `AGENTS.md`, `.codex`, `.agents`, held within `gate.protectedNames`' default by a test; skills folders
  `.codex/skills` and `.agents/skills`, listed by a sweep; residual risks T-05, T-13 and T-22, as Claude Code's;
  credential `.codex/auth.json`, bound read-only at `/home/agent/.codex/auth.json`, the tool's home being its default
  `~/.codex` (an explicit `CODEX_HOME` that does not exist yet, as in a preflight's cell, stops the tool).
- The release is the directory its installer keeps per version, `releases/<version>/`, holding `bin/codex` and beside
  it `codex-resources/` (its own `bwrap`) and `codex-path/` (`rg`); an executable not at `releases/<version>/bin/` has
  none. On this machine it lies under the person's `~/.codex/packages/`, and only that directory is bound.
- Codex's launch, verified with 0.166.0: `exec --json --color never --strict-config` (a setting it does not know stops
  it, so that a version that renamed one cannot run with it silently ignored), `--skip-git-repo-check` (the copy holds
  no repository), `--ephemeral`, `--ignore-user-config`, `--ignore-rules`, `--cd /work`; the model's commands under a
  permission profile of the role (`default_permissions="fabrewk"`, extending `:workspace` for a builder and
  `:read-only` for a reviewer, with `network.enabled=false` and the credential's folder `/home/agent/.codex` at
  `none`); `approval_policy="never"`, since a headless run cannot ask; its discovery off (`project_doc_max_bytes=0`:
  no `AGENTS.md` or `AGENTS.override.md`; `skills.include_instructions=false` and `skills.bundled.enabled=false`: no
  skill of the copy's `.agents/skills` or `.codex/skills`; a project's `.codex/config.toml` is not loaded, the project
  not being trusted in an empty home), shown by `codex debug prompt-input` with each of them planted in a copy: none
  reached what the model sees; the web, updates, analytics, feedback, telemetry, history and memories off; and its
  features that reach beyond the copy off (hooks, plugins, apps, browser and computer use, image generation and
  viewing, goals, tool suggestions, skill search and installation, workspace dependencies, the shell's snapshot, its
  daemon, unbounded connection retries, and the unified execution that leaves processes running between calls);
  `--model`, `model_reasoning_effort`, `--output-schema /home/agent/schema.json`; the packet as the last argument,
  after `--`. Its environment is the cell's alone. Inside a real dispatch the model's commands had a seccomp filter, no
  new privileges, only a loopback interface, a direct socket refused ("Operation not permitted"), no name resolution,
  the credential's folder unreadable and unlistable, the home and, for a reviewer, `/work` not writable. What the tool
  still offers its model and no setting removes in 0.166.0 is declared (T-22): a JavaScript `exec` that calls its
  tools, and the tools to start other agents, whose use voids the stream.
- The preflight runs, in each role's cell with no network and no credential: the tool's version; `codex sandbox` under
  the builder's profile creating a temporary file in the copy and in its `/tmp`; and under the reviewer's profile, where
  only a write refused with "Read-only file system" passes, never the failure of something else. Unlike Claude Code,
  Codex starts its own sandbox without a model; when it cannot make its namespaces it ends with status 1 and runs
  nothing (tested with user namespaces disabled), so a failure of its sandbox fails closed.
- Codex's stream has one grammar (`crates/adapter-codex/src/stream.rs`): lines ended by a newline alone, a final
  newline not a blank line, each line one compact JSON object; `thread.started` with a text `thread_id` first,
  `turn.started` second; then `item.started`, `item.updated` and `item.completed`, each holding exactly `type` and
  `item`, and `error` notices holding exactly a text `message`; one `turn.completed` with an object `usage`, last, after
  every item started was completed. A `turn.failed` is outside the grammar: a failed turn gives no result, usage or
  examination, as a void stream does, so it is read as one (the mutation run showed the two readings the same). An item
  holds exactly the fields of its kind, of their types: `agent_message` and `reasoning` a text `text`;
  `command_execution` a text `command` and `aggregated_output`, an `exit_code` null while `in_progress`, a whole
  number when `completed`, either when `failed` or `declined`, and that `status`; `file_change` its `changes` (`path`,
  `kind` among `add`, `delete`, `update`) and a status; `todo_list` its `items` (`text`, `completed`); `error` a text
  `message`. An item starts once, running; is updated or completed only while not completed, keeping its kind and its
  command; may complete without a start. Any other event, item kind (an MCP call, another agent's, a web search),
  field, type or order voids the whole stream: no result, model, tokens or examination. The `error` notices are what
  the tool recovered from (a real run without network wrote "Reconnecting... n/5"); a fatal one ends in `turn.failed`.
  The result is the last `agent_message`'s text read as a JSON object; a last message that is not an object gives
  none and examines nothing. Examination counts each command that
  completed with status 0 and a non-blank output, by the simple commands of its script: Codex shows a command as its
  words quoted for a shell, so a shell's `-c` or `-lc` with one script counts the script's, any other command its own
  line's, and one whose quoting does not close none; messages, reasoning, plans and edits never count. The stream does
  not say which model answered: the model observed is unknown. Tokens are `turn.completed`'s `usage`: `input` is
  `input_tokens` less `cached_input_tokens` (OpenAI counts the cache within the input; unknown if the cache exceeds
  it), `output` is `output_tokens`, which holds the reasoning, `cached` is `cached_input_tokens`. The recorded streams
  are `crates/adapter-codex/tests/streams/review.jsonl`, `build.jsonl`, `probe.jsonl` (the probe of the sandbox and a
  silent command, which shows an empty `aggregated_output`) and `failed.jsonl` (the provider's refusal of the first
  schema, ending in `turn.failed`), kept whole: they hold no figure of the account.
- The credential (3.6), by test: with the read-only bind, four real dispatches worked and the person's file kept its
  date, size and digest; inside a cell the tool cannot rewrite it, and its own folder in the cell's home takes what the
  tool writes (its state databases, its logs, its model catalogue), removed with the attempt. Codex renews a ChatGPT
  login when its `last_refresh` is older than about eight days (its documentation), rewriting the file; a renewal
  inside a cell was not provoked. As D-079 closed it for Claude Code, part 9e refuses a dispatch whose credential
  would be renewed within the attempt's minutes, reading only `last_refresh` (D-081).

**The run, building side, as part 9e fixed it (`confinement-design.md` 3.7 to 3.9; REQ-RUN-1, REQ-RUN-2, REQ-RUN-5
to REQ-RUN-7, REQ-RUN-10, REQ-TASK-3, REQ-TASK-5, REQ-CLI-3; S-5, S-6; D-079, D-081, D-082).**

- `run` is `crates/core/src/cli/run.rs`. `run-started`'s guards are evaluated in the table's order: `DEPENDENCY_OPEN`,
  `BASE_MOVED`, `LOCK_HELD`, REQ-RUN-2 (the profile, `SETTINGS_INVALID`; its tool and required tools, `TOOL_MISSING`;
  the budget; the packet's size) and `ALAMBIQUE_UNAPPROVED` before anything is written; then, under the lock, which a
  run holds to its end (REQ-REG-5), `CONFINEMENT_UNAVAILABLE` (the backend, its probe and the tool's preflight) and
  `DISK_LOW`. `BASE_MOVED` reads the last of a dependent task's dependencies' integrations from the record branch's
  history, never from the clock: going back from the head along first parents, the first commit that is a
  dependency's candidate or names it in a `Fabrewk-Candidate` trailer (REQ-GATE-8); none is `BASE_MOVED` (P9E-9). A
  guard whose code is one of REQ-TASK-5's (with `DISK_LOW`, REQ-RUN-10) halts the task instead of
  refusing: `task-halted` from `authorized`, with no `run-started`, exit 4, since only an authorization moves a halted
  task and grants it more. The others refuse with nothing written. `authorize --run` evaluates the run's refusing
  guards before writing either move (D-062 point 1) and writes the authorization before a halt.
- The layers' widenings (P9E-12): a run reads the stage's and the task's layers against the policy in force with the
  widenings the authorization in force named; a layer a later, stricter policy makes a widening is refused
  (`WIDENING_UNNAMED`, nothing written) until an authorization names it.
- REQ-RUN-2's tools (S-6): the tool resolves to its release (`TOOL_MISSING`, part 9c), and each of the profile's
  required tools resolves under the system directories the backend shows (`Backend::system`; for bubblewrap `/usr`,
  `/bin`, `/sbin`, `/lib`, `/lib64`), else `TOOL_MISSING` saying it lives outside what a cell shows.
- The budget (D-082): the task's `limits` count the attempts since its last `task-authorized` (a stage's authorization
  counts from the start), so an authorization grants them anew; its stage's `budget` counts every attempt of the
  stage's tasks, a cap no authorization raises. Dispatches are attempts started; minutes are attempts' `seconds`, an
  unknown one counting the whole time its attempt was allowed (S-5); tokens are `input` plus `output`, never `cached`,
  and an unknown part with a token limit is spent (`BUDGET_EXHAUSTED`, saying so). An attempt is allowed the profile's
  `minutes`, less when the task's or the stage's minutes left are fewer; a builder ended at a time so lessened halts
  `BUDGET_EXHAUSTED`, at its own `TOOL_FAILED`. Minutes are sixty seconds, or with `fault-injection` the seconds
  `FABREWK_FAULT_MINUTE` names, for tests.
- The packet is the tool's last argument: one of more than 120,000 bytes halts `TOOL_FAILED` before any attempt.
- The preflight's pass is kept as verified by use for the tool's path and version in `local/cache/detected.json`, as
  the backend's probe is; a tool not verified is preflighted at its run.
- Each adapter declares `expiry`, read from the credential's bytes: Claude Code's `claudeAiOauth.expiresAt` (in
  milliseconds), Codex's `last_refresh` (RFC 3339 in UTC, `Z`) plus eight days. `adapter::dispatch` refuses
  `TOOL_FAILED` before any process starts when that time falls within the attempt's allowed time, or cannot be read
  (D-079, D-081); no token appears in the words.
- The events, under the lock and in one record commit: `run-started` (the person's, exactly `task`, `head`,
  `authorization`, the `stream`, `seq` and `digest` of the `task-authorized` or `stage-authorized` it relies on, and
  `base`, 40 or 64 hexadecimal digits), `attempt-started` (exactly `role`, `profile`, `tool`, `version`, the tool's
  version as it reported it or `null` (PD-29, P9E-10), `model`, the provider's name of the model requested, `effort`
  and `allowedSeconds`), `attempt-ended` (exactly `attempt`, the place of its
  start, `outcome`, `model`, observed or `null`, `usage` as S-5 fixed it, and `evidence`, `null` or exactly the
  `stdout` and `stderr` digests), `candidate-recorded` (exactly `commit`, `tree` and `changes`, each exactly `path`,
  `before` and `after`, `null` or a `mode` and an `oid`) and `task-halted` (exactly `code`, one of REQ-TASK-5's or
  `DISK_LOW`, and `reason`, the core's words with every path relative to the repository's root or, under the
  person's home, written from `~/`, so that the record holds nothing of the machine's layout, P9E-2). What these
  payloads copy from elsewhere (a changed path, the model a stream names, the tool's version, the catalogue's names) is
  any text, white space and the empty text included, so that no name a tool or a file system gives leaves an event the
  replay refuses. The replay holds
  `run-started` to its stream's head and to the authorization in force: the task's last `task-authorized` or its
  stage's last `stage-authorized` that binds it, whichever comes later in the task's events, which `run` names and
  from which the task's limits count (P9E-3; `authorize <stage> --run` relies on the stage's authorization only for a
  task it binds); an `attempt-ended` names its attempt: one already ended is `ATTEMPT_ENDED`, one a later attempt replaced
  `ATTEMPT_REPLACED`, both before the move is checked, and an end naming no attempt while one is open
  `RECORD_TRANSITION_INVALID`.
- The attempt: its directories `local/tmp/attempt-<task>-<seq>/` and its evidence `local/evidence/attempt-<task>-<seq>/`;
  the base exported into `work` (a failure ends the attempt `failure` with no evidence digests, keeps the core's words
  in the evidence file `export`, `0600`, and halts `TOOL_FAILED` naming it, PD-21); the builder dispatched with the
  role's packet and schema; its end: `success` when it exited 0 and its stream's result fits the build's schema, else
  `failure` (a run ended at its time, its output past 32 MiB, a stream with no result, a result that does not fit:
  `TOOL_FAILED` naming the evidence), `unknown` when the backend cannot show its cell ended (`OUTCOME_UNKNOWN`, its
  directories kept unread). On success the change set's refusals halt with their codes (`LANE_VIOLATION`,
  `PROTECTED_PATH`, `PROTECTED_PATH_WILDCARD`, `CREDENTIAL_SUSPECTED`), naming no evidence, and a failure of Git while
  the change set or the candidate is made halts `TOOL_FAILED` naming an evidence file of its own, `candidate`, `0600`,
  holding the core's words, never the builder's output, whichever Git command failed, the reading of where the
  candidate's branch is included (P9E-11, P9E-14; Git's own output and exit status are kept nowhere in
  the core until part 9g keeps them for every command, D-083); else the candidate is committed on
  `fabrewk/<task>` from where the branch is, its co-author the catalogue's tool name, the adapter's provider and the
  model observed (the one requested when the stream names none, or only white space), and recorded. The attempt's directories are removed;
  its evidence is kept.
- Until part 9f runs the checks and the review, a recorded candidate halts the task `CONFINEMENT_UNAVAILABLE`, its
  reason saying where the candidate is and that part 9f builds the rest (D-082). `run <stage>` runs its first task
  that may run and stops there.
- REQ-RUN-5: every write command, before its own plan, finds the tasks left `running` (none can have a live process,
  since a run holds the lock to its end; a second writer meanwhile is `LOCK_HELD`), in the record it read and again in
  the one it reads under the lock, where a task found running sends it back to recover first and plan again (P9E-13), ends each open attempt `unknown`
  with `seconds` and `tokens` `null`, halts the task `OUTCOME_UNKNOWN` and removes its last attempt's directories, open
  or ended, before its events, so that a recovery stopped anywhere is made again whole (P9E-4, P9E-8), in a record
  commit of its own; then the command plans on that record. `status <id>` shows a halted task's code, its
  catalogue reason and the halt's words (`fabrewk.subject/3`, with `halt`); `status` walks no directory of `local/tmp/`,
  which only cells make, running or kept unread: while one is there its size is unknown (REQ-RUN-3, P9E-1, P9E-7); `STATUS.md` shows a task's dispatches,
  seconds and tokens read, written and read from the cache, each sum unknown when a part is.
- `run`'s document is `fabrewk.run/1` (`id`, `attempt`, `candidate`, `halt`, and the events and commit written);
  `authorize`'s is `fabrewk.authorized/2`, with `run`, `null` or the run's document. Exit status 4 is the run's halt.

**Checks, review and verification, as part 9f fixed them (`confinement-design.md` 3.4 and 3.8; REQ-GATE-1 to REQ-GATE-6,
REQ-RUN-8, REQ-RUN-9, REQ-TASK-5; D-085).**

- The library is `crates/core/src/gate.rs` (the required checks, a check's identity, its cell and outcome, whether a
  verification holds, the rounds and their progress measure, what a round leaves for the next); the rounds are
  `crates/core/src/cli/run.rs`'s. Part 9f replaces D-082's halt on a candidate: a run goes on to the checks and the
  review, round after round, until a round verifies the task or a halt stops it, all under the lock the run holds.
- REQ-RUN-3, as D-085 amended it: a building round on a task that has a candidate (after a review's reject, a failed
  check, or an authorization that followed either) exports the task's last candidate into its copy, so that the builder
  corrects rather than rebuilds; the change set is still computed against the base and the new candidate's parent is
  still the base.
- A round: the round limit first (the task's `limits.rounds`, counted since the authorization in force as D-082 counts
  the other limits; `ROUNDS_EXHAUSTED`); then the builder's attempt, but in a revalidation's first round; each
  required check on a fresh export of the candidate; and, once every one passed, the reviewer's attempt on another fresh
  export. A check that does not pass ends the round with no reviewer, one confirmed blocking finding each; a reject ends
  it with its own; an examined accept verifies the task (`task-verified`, exit 0). At a round's end with findings,
  `NO_PROGRESS` is evaluated first, over the counts of the rounds since the authorization in force: the last not below
  the one before it, nor that one below the one before it; then the next round starts with no person's move (PD-33),
  carrying the previous round's confirmed findings and the checks that did not pass. Both roles' packets carry it.
- The required checks (REQ-GATE-1): the task's own `checks`, its build profile's `profiles.<name>.checks` and every check
  its stage approved, by name, each defined by the stage or by the policy; a stage's definition that the policy in force
  defines otherwise, as one adopted after the stage may, is `CHECK_DUPLICATE` at the run too (P9F-2), and a name
  neither defines is `SETTINGS_INVALID`. A tool a check names must be found on the cell's own `PATH` (`/usr/bin:/bin`),
  else `TOOL_MISSING` (S-6); its version is the first line its `--version` prints, `null` when none.
- A check's cell (3.4): its copy writable at `/work`, its home and `/tmp`, no network, no credential, no namespace of its
  own, the cell's four variables, its command run without a shell, limited by its own `minutes`, which spend nothing of
  the task's or the stage's limits (D-085 point 3). A check with `network = true` halts the task
  `CONFINEMENT_UNAVAILABLE` before any dispatch (D-085 point 1). Its outcome: `passed` for exit status 0 only; `failed`
  for another status (a signal inside the cell reaches Fabrewk as the status 128 and the signal's, P-7); `timed-out` at
  its limit; `interrupted` when its cell was signalled or its output passed 32 MiB; `unknown` when the backend cannot
  show its cell ended, which halts `OUTCOME_UNKNOWN` with its directories kept unread. A check whose cell could not be
  started halts `TOOL_FAILED` with no event of its own. Its directories and evidence are `check-<task>-<seq>`, `<seq>` the
  place its `check-recorded` takes; its copy is deleted unread.
- A check result's identity (REQ-GATE-3) is the SHA-256 of the canonical JSON of exactly `product` (the SHA-256 of the
  candidate's product tree as `git ls-tree --full-tree` lists its root, `.brewstillery` left out), `definition`
  (`command`, `network`, `minutes`, `tools`), `policy` (the digest in force), `tools` (each named tool's `path`, as it
  resolves, and `version`) and `platform` (the operating system, the processor and the backend's name, as
  `linux-aarch64-bubblewrap`; with `fault-injection`, `FABREWK_FAULT_PLATFORM`). A verification holds while its
  candidate's tree and the requirements' digest are the task's, its review is an accept bound to both, and its checks
  are exactly those required now, each with the identity recomputed now; otherwise its evidence is stale. `accept` and
  integration refuse stale evidence (`EVIDENCE_STALE`, REQ-GATE-10) in part 9g; `run` revalidates it.
- The review (REQ-GATE-4): a new attempt of the review profile's agent, its result validated (`REVIEW_INVALID`); a
  stream that gives no result, the audit having voided it, is `REVIEW_UNEXAMINED` with no review recorded; a review
  that fits is recorded, and one whose audit counted no examination is `REVIEW_UNEXAMINED` whatever its verdict
  (PD-25: "invalid output whatever its verdict"). `review.differentProvider`: with the reviewer's provider, as its
  adapter declares it, the builder's, `require` halts `PROVIDER_MISSING` before any dispatch, and so does an absent
  reviewer's tool, which otherwise refuses `TOOL_MISSING`; an authorization naming `--widen review.differentProvider`
  records it in `widened` though no layer widens it, and the run goes on with the limitation (only the authorization's
  naming counts, never a layer that sets the key, P9F-1); `prefer` records the
  limitation; `off` none. `status <id>` says when the last review has it.
- Revalidation (REQ-RUN-9): `run` on a `verified` task whose evidence is stale, or on an `authorized` task authorized
  from `verified`, `accepted` or after `merge-failed` (the event before the authorization in force), keeps the task's
  last candidate and dispatches no builder in its first round, which counts against the rounds. A verified task whose
  evidence is current is `MOVE_NOT_ALLOWED`, evaluated with REQ-RUN-2, whose readings its identities need. Since the
  table halts no `verified` task, what would halt it before the lock refuses instead (`ROUNDS_EXHAUSTED`,
  `BUDGET_EXHAUSTED`, `PROVIDER_MISSING`), nothing written, as REQ-RUN-9's criterion asks of the first; under the
  lock its `run-started` is written before the confinement guard, so that `CONFINEMENT_UNAVAILABLE` or `DISK_LOW`
  halts it from `running`.
- The payloads. `check-recorded`: exactly `check` (any text), `round` (a count), `identity` (64 hexadecimal digits),
  `outcome` (`passed`, `failed`, `timed-out`, `interrupted`, `unknown`) and `evidence` (`null` or exactly the `stdout`
  and `stderr` digests). `review-recorded`: exactly `attempt` (its attempt's place), `round`, `verdict` (`accept`,
  `reject`), `findings` (each exactly `severity`, `status`, `path`, any text or `null`, and `summary`, any text),
  `examined` (the audit's count), `tree` (the candidate's tree), `requirements` (the SHA-256 of the requirements'
  canonical JSON), `provider` (the reviewer's, as declared) and `limited` (a boolean). `task-verified`: exactly
  `candidate`, `tree`, `requirements`, `checks` (each required check's name and identity) and `review` (the place of its
  `review-recorded`).
- What a round carries to the next (REQ-RUN-8): `previous.findings`, the confirmed findings, blocking or minor, of a
  reject that examined; `previous.failedChecks`, each exactly `check`, `outcome`, `stdout` and `stderr`, the last 4,000
  characters of each channel the check's evidence kept, read as UTF-8 with replacements. Read from the record for a
  run's first round: the events after the last candidate or `run-started`, whichever is later.
- REQ-RUN-5 for checks: a recovery also removes the directories of the checks the stopped run made (their places after
  its `run-started`); an earlier run's, left unread, stay.
- The run's document stays `fabrewk.run/1`: `halt` is `null` once the task is verified, and `attempt` names the last
  attempt. `run` and `authorize --run` record `check-recorded`, `review-recorded` and `task-verified`; `run` from
  `verified` may refuse `ROUNDS_EXHAUSTED`, `BUDGET_EXHAUSTED` and `PROVIDER_MISSING`, and any run `CHECK_DUPLICATE`.

**Acceptance and integration, as part 9g fixed them (`confinement-design.md` 3.8 rule 9; REQ-GATE-7, REQ-GATE-8,
REQ-GATE-10, REQ-REG-10, REQ-RUN-6, REQ-REG-13; S-4, S-11; D-083, D-086, D-087).**

- The command is `crates/core/src/cli/accept.rs`; the library, `crates/core/src/merge.rs` (a commit's product tree, the
  change set between two commits and its third guard, what the index and the working tree hold at the touched paths,
  the integration commit, the move, the working tree and the index brought to the candidate).
- `fabrewk accept <task> --seen <digest> --say <words>` (D-086 point 2). `status <task>` shows, when the table allows an
  acceptance from the task's state and, but from `verified`, the record branch's product is the candidate's, what an
  acceptance binds: the task as an authorization shows it, then the candidate and its tree, its files (added, changed,
  removed), the checks with the identities its last verification of that tree recorded and that verification's review
  (event, verdict, findings, examinations), or that none is on record; in the person's language, with its digest, which
  `accept` takes with `--seen` (`TASK_CHANGED` otherwise; a usage error without it). `fabrewk.subject/4` carries it as
  `accepting` (`text`, `digest`), beside `shown`.
- `accept`'s guards, in order: the move (`MOVE_NOT_ALLOWED`: from `verified`, or from `authorized` or `halted` with the
  branch at the candidate; a task with no candidate has none to accept); the person and their words; REQ-GATE-7's
  `--seen`; `EVIDENCE_STALE`; `ALAMBIQUE_UNAPPROVED`; then the integration's preconditions in `task-integrated`'s order,
  each refusing before the signature, with nothing written (D-087 point 1): REQ-GATE-8 (Git's locks, `TOOL_FAILED`; the
  third guard), `BASE_MOVED`, `WORKTREE_DIRTY`, `EVIDENCE_STALE`.
- Evidence current: the task's last `task-verified` holds (REQ-GATE-5) with every required check's identity recomputed
  now on the candidate (a check whose identity cannot be recomputed, its tool gone, counts as stale), and its review was
  recorded under the policy in force (D-086 point 4), which `run`'s revalidation reads alike.
- The base is the candidate's parent (REQ-RUN-4). The base is unmoved when the product tree at the branch's head is
  the base's; the branch is at the candidate when it is the candidate's; neither is `BASE_MOVED`.
- The third guard: the change set recomputed from the base's and the candidate's whole trees (`git diff-tree -r`), a
  path with a protected component, as `candidate::protected` reads it, `PROTECTED_PATH`; a path that is not valid text,
  and the lanes, `LANE_VIOLATION` and `PROTECTED_PATH_WILDCARD`, with the protected paths read now (S-4).
- Clean: at each touched path, the index's stage-0 entry, and the working tree's entry read without stat data or filter
  (a file hashed as its bytes with its executable bit, a link's target as written; a path below a link or a file, a
  directory, a special file or an unmerged entry being no entry, and an index entry the path would displace, one below
  it or a file above it, each one that is no touched path, making it none either, P9G-5, P9G-7), equal the base's or, with the branch at the candidate,
  the candidate's. A submodule's path in the working tree is Git's own checkout and is never written but made as an
  empty directory where none is; its index entry is.
- `task-accepted`'s payload is exactly `task`, `head` (bound as an authorization's), `candidate`, `tree` (the task's last
  `candidate-recorded`'s, which the replay holds it to), `requirements`, `checks` (each name's identity) and `review`
  (the verification's), and `shown` (the digest of what `status` showed).
- Under the lock, after `task-accepted` is written: the preconditions again, just before the move; the move: none when
  the branch is at the candidate, to the candidate itself when the head is the base commit, else to an integration
  commit (the head's tree with the change set applied through a temporary index, the head its only parent, the
  candidate's message with `Fabrewk-Candidate: <commit>` after its trailers, the person's Git as author and committer;
  its product tree must be the candidate's), by `update-ref` with the old value (a branch moved meanwhile,
  `BASE_MOVED`); the record commit then starts from the commit the integration left. The working tree and the index
  are brought to the candidate at each path that holds the base's content, read again after the move: removals first
  (a file or a link, never a directory; then each directory they emptied), then each entry written from its object's
  bytes into `local/tmp/merge-<task>/` and renamed into place, then the index by `update-index --index-info` (removals
  first), with no filter, conversion or checkout command (S-11). Each path, and the index, is read again just before
  it is written, and one that changed since is `WORKTREE_DIRTY` (P9G-1); the instant between that reading and the write
  is left, a cooperative control as P8C-8. A file and a directory may trade places: a path below a touched one that is
  not a directory holds nothing, and a directory at a touched path above others holds no entry of its own. Then, with the branch at the candidate: its product the candidate's,
  every touched path the candidate's, the evidence current; then `task-integrated`, exactly `from` (the head moved from,
  `null` when no move was made) and `to` (the head with the branch at the candidate).
- A failure records `merge-failed`, exactly `code` (`BASE_MOVED`, `WORKTREE_DIRTY`, `EVIDENCE_STALE`, `PROTECTED_PATH`,
  `PROTECTED_PATH_WILDCARD`, `LANE_VIOLATION` or `TOOL_FAILED`, any other failure being `TOOL_FAILED`), `reason` (the
  core's words, with no path of the machine), `moved` (whether the branch is at the candidate, or `null`; nothing is
  moved back)
  and `evidence` (`null`, or when a Git command failed, whatever the code, a compare-and-swap Git refused included
  (P9G-4), exactly `status`, its exit status or `null` for a signal, and `stderr`, the digest of its standard error).
  `moved` is read when the failure is recorded, whoever moved the branch (P9G-2, P9G-6), after the failure's evidence
  is taken; `null` when where the branch is cannot be read, the reason saying why (P9G-8). The task stays `accepted`, exit 4, and waits for a maestro (`task-merge` in the
  views): `run <task> --say` finishes it under the same checks, its words checked and not recorded (D-087 point 2), or
  `authorize` returns it to `authorized` keeping its candidate. A command killed after the move leaves `task-accepted`
  written and the branch moved; the next write command commits the events, and `run` finishes the integration.
- `accept`'s document, and `run`'s on an accepted task, is `fabrewk.accepted/1` (`id`, `integrated`, the commit or
  `null`, `failed`, `null` or `code`, `reason` and `moved`, and the events and commit written).
- `reject`: `MERGE_IN_PROGRESS` while a `task-accepted` follows the task's last `candidate-recorded` and the branch is at
  that candidate; `task-rejected` gains `candidate` (the last, or `null`) and `atCandidate` (D-068 point 3), and its
  document is `fabrewk.rejected/2` with both.
- `BASE_MOVED` at `run` (part 9e) also takes the commit a dependency's `task-integrated` names as `to` for its
  integration, so that a branch a person brought to a candidate by hand (PD-44) counts.
- Git's own output (D-083, D-086 point 3): every Git command the core runs has its standard error captured; one that
  fails is `TOOL_FAILED` with its first words and its exit status, and in a write command (from the start of its write
  path) its standard error and exit status are kept in `local/evidence/git-<seconds>-<process>-<n>/` (`stderr`,
  `status`; `0700`, `0600`), which the words name. A committed or a staged file is read by asking first whether the tip
  or the index holds it, so that a Git that fails is never taken for its absence (P9G-3); a ref that exists and names
  no commit keeps what its reading printed too (P9G-9); only exit status 1 of `rev-parse --verify -q` says that a name
  resolves to no commit, any other is a failed Git (P9G-11, P9G-12); and where the record branch is, read by a write
  command, is never taken from a Git that failed (P9G-10). The same holds wherever a write command reads a state from
  Git (P9G-13, P9G-14): the branch checked out (exit status 1 of `symbolic-ref -q` alone is `HEAD_DETACHED`), the key
  Git signs with (exit status 1 of `git config --get` alone is an unset key), the main working tree (`NOT_A_REPOSITORY`
  naming Git's evidence), the base's ignore rules, the candidate's branch, and what an acceptance binds; `status` says
  such a failure with Git's exit status, keeping nothing. A failure whose own reading of the state fails too keeps both
  outputs, and the event's `evidence` is the first's (P9G-15); the record commit's compare-and-swap that a moved branch
  refuses keeps Git's output, named by `RECORD_COMMIT_RACE`; and a Git failure a write command goes on from (that
  compare-and-swap, rebuilt on the new head; the person's index that cannot follow the record commit, P7D3A-21) is said
  in its words, the commit reported made, with its evidence (D-088). The key Git signs with is read once.
- D-089: no state outside a failure says which evidence it names. A failed Git's words name its directory, and a
  failure that follows another names both, in order, the first one's first; `merge-failed`'s `evidence` is read from
  the first directory its failure's words name. What failed in Git that a write command went on from is held by its
  session, survives the recovery of a stopped task that the command makes first, and is named in its words and in its
  document as `recovered` (texts, each naming its evidence); a refusal after it names it too ("before it, …"). Every
  write command's document gains `recovered`, under a new schema: `fabrewk.adopted/2`, `fabrewk.proposed/2`,
  `fabrewk.authorized/3`, `fabrewk.run/2`, `fabrewk.swept/2`, `fabrewk.discarded/2`, `fabrewk.approved/2`,
  `fabrewk.closed/2`, `fabrewk.rejected/2` and `fabrewk.accepted/1`. The recovery's own case is held by the design and
  not provoked by a test: the person's index update and the temporary index's have the same words, so
  `FABREWK_FAULT_GIT` cannot fail the one alone. A refusal after a recovery names what the recovery went on from too
  (P9G-20); the evidence is read from its directory's own name, the last component of the path the words give
  (P9G-21). Declared (D-090, P9G-22): an internal error (exit 70, `fabrewk.internal/1`) says only its path, by design;
  what failed in Git before it stays kept in `local/evidence/`, named by no word. Such a refusal writes that directory and nothing else, a fourth kind of
  refusal that writes (11.2 rule 4). Reading commands keep nothing. With `fault-injection`, `FABREWK_FAULT_GIT` makes a
  Git command whose words hold its text fail as Git fails on an option it does not know.
- With `fault-injection`, the pauses `before-move` (before the second check), `before-update-ref` and `after-move`.
- D-091, in its closing: `check`'s layout (C8) takes the publication mode in force, and in `own-repository` the
  domain's own `.git`, as a directory, is part of the domain (REQ-DOM-2 amended); with another mode or none on record,
  or as a file or a link, it is still a finding (`ALIAS_INVALID` as a directory, `DOMAIN_UNKNOWN_ENTRY` otherwise).

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

**Extensions, as phase 9's opening amended it (D-076, S-9).** An extension's cell holds its program, read-only, at
the path where it was found. An interpreter or support file outside the system directories and the repository is
absent; an extension that then fails to start is `TOOL_FAILED`, whose words say so.

**Extensions, as part 9h fixed them (REQ-EXT-1 to REQ-EXT-5, REQ-CLI-3, S-9; D-094).**

- The runner is `crates/core/src/cli/ext.rs`. `fabrewk <name> …`, for a first word that is no core command's, is an
  extension's, every word after it its own (`--json` included). A name outside the alias pattern, or with no program
  `fabrewk-<name>` in `PATH`, or one the policy in force does not permit (no table `[extensions.<name>]`), is a usage
  error, as an unknown command is; a present program whose permission cannot be read (no record that replays) is the
  record's refusal. The program is the executable `PATH` finds, resolved; its digest is the SHA-256 of its bytes, and
  a pinned `digest` that differs refuses `EXTENSION_DIGEST` before it runs. Then `extensions.<name>.network` refuses
  `CONFINEMENT_UNAVAILABLE` (D-094 point 1), and so does a backend that is absent or does not pass its probe.
- Its cell: the repository read-only at its own path, `.brewstillery/local/` hidden but for its area `local/ext/<name>/`
  (writable, `0700`, kept between runs, its home and `/tmp` the directories `home` and `tmp` inside it, each checked to
  be reached through no link before it is made and again after; its lock `local/ext/<name>.lock`, which no cell
  reaches, held for the whole run, so that two runs of one extension never overlap, `LOCK_HELD`, P9H-1) and
  `local/cache/` (read-only, where it exists, reached through no link); the running `fabrewk` and the program,
  read-only at their resolved paths; no network, no namespace of its own; the directory is the repository's root; the
  environment is a cell's (`PATH`, `HOME`, `TMPDIR`, `LANG`) and the four variables of REQ-EXT-2. Its run is limited by
  `extensions.<name>.minutes` (D-094 point 2) and by 1 MiB kept of each channel; its evidence is
  `local/evidence/ext-<name>-<process>-<nanoseconds>/`. Its standard error reaches the person once it has ended, every
  hidden character written as an escape.
- Exit 10 applies nothing and exits 10; 126 and 127 are `TOOL_FAILED` whose words say that an interpreter or a support
  file may lie outside the cell (S-9); any other status but 0, a time limit or a cap passed, `TOOL_FAILED` naming its
  evidence (D-094 point 3). On 0, every line of its standard output (UTF-8, split at line breaks, the last one's
  optional) must be one JSON object with no member named twice: `{"protocol":1,"request":"note","text":<text>}` or
  `{"protocol":1,"request":"propose","stage":…,"tasks":…}`, whose two fields are those of `propose`'s file; otherwise
  `EXTENSION_OUTPUT_INVALID`, naming the line and why, never its bytes. With no line, nothing is written.
- The requests are applied in one write command, all or none, planned on the record as it is once the extension has
  ended, since another command may have written meanwhile (P9H-3): a note is `extension-note` in `ext-<name>`; a proposal
  is planned as `propose` plans its file, every request drawing from one set of identifiers (P9H-2), with the
  extension as the events' actor (`stage-proposed` and
  `task-proposed` take an extension, D-094; never a person), and its own refusals keep their codes, their detail
  naming the line. Under the lock the policy in force must still permit the extension at the digest it ran with, else
  `EXTENSION_DIGEST`. Applying them keeps the backend's programs as verified by use; a run with no request writes
  nothing, so its probe runs again next time.
- `config` (`fabrewk.config/3`) lists each permitted extension with its resolved path and digest, or as absent, without
  running it (D-094 point 4). `extension-note`'s payload is `{"text"}` (schema 1).

**The whole, as part 9i fixed it (11.5, REQ-RUN-5, REQ-SEC-1, REQ-SEC-5; D-092, D-095).**

- The idle `pasta` (D-092). As soon as the helper exists, before it brings the cell's network up and before the cell
  is released, the bubblewrap backend writes in the cell's scratch directory a file `beside`: the helper's identifier
  and start time (`/proc/<pid>/stat`, field 22), the cell's first process's, and the program the helper was started
  as, its first argument (`pasta` makes `/proc/<pid>/exe` unreadable to its owner); only the instant between the
  helper's start and that write stays uncovered, and the residual stays declared. A backend's `idle` ends the helper
  only while a program named `pasta` runs with the start time and the program written, and once the cell's first
  process is gone or another; then, or when the helper is gone, it removes the file and says nothing is left; a file
  it cannot read whole, a cell still running or a signal not sent leave it. Every write command calls it on each
  directory of `local/tmp/` first, whatever it then does, so that the helper of a probe or a preflight killed before
  `run-started`, which no recovery reaches, is ended too; the residual `network-helper` says it lasts until the next
  write command. The recovery of a stopped run removes a cell's directories only once nothing is left beside it,
  waiting up to ten seconds for the cell to end; otherwise it keeps them, with the file, for a later command (P9I-2).
  A helper whose file could not be written is ended at once, and the cell fails.
- The flows of 11.5 are `crates/fabrewk/tests/flows.rs`, with F-1's commands in `tests/common/flow.rs`: the project
  events from the test helper (before phase 10); `run <stage>` takes the first task, rejected in round 1 and accepted
  in round 2; the second waits (`DEPENDENCY_OPEN` at its `run`) until the first is integrated. In `own-repository`
  and `unpublished` the product's branch moves only by an integration, to the candidate itself.
- The crash suite over F-1 is `crates/fabrewk/tests/crash.rs`: each write command of F-1 runs again from the fixture
  as it stood before it, stopped at its n-th write point for every n, until it ends by itself. The points are those of
  the record (each event, signature, mark, view, record commit and ref update), of the integration (its commit, ref
  and working tree) and of the candidate (its objects, its commit and its branch, P9I-4); a cell's own files, the
  export and the evidence are not the record's and have none: a kill while a cell runs is `pd_08_…`'s and the
  backend's tests. The next write command after a stop is a person's `discard` of a proposal that does not exist,
  which recovers a task left `running` and dispatches nothing; every event the branch held before the command, or
  after the stop, keeps its bytes on the branch and on disk.

### BUD: size, dependencies and build

- **REQ-BUD-1.** Size is counted in non-blank lines of Rust that are not only comment (`//`, `///`, `//!`, and lines
  inside `/* */`), outside `#[cfg(test)]` modules, `tests/`, `benches/` and `examples/`, per ring, by a counter kept in
  the workspace as a development tool. A ring is its directory: the core, each built-in module, the installer. (AD-5)
- **REQ-BUD-2.** AD-5's limits are provisional ceilings (core 4,000; each tool adapter 600; each confinement backend
  400; installer and assessment 1,200; 8 direct runtime crates; 50 crates in the release lock file; a stripped release
  binary of 8 MB). A maestro freezes the budget after measuring phase 7's skeleton, before any feature; raising it later
  is a maestro's recorded decision with its reason. The core's ceiling was raised to 7,500 after part 7d1 was
  measured (D-054), then to 8,000 (D-074), 11,000 (D-075), 12,000 (D-086) and 12,500 (D-097). (AD-5, D-038)
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
| `alambique` | `alambique-baseline` | person, signed | the alambique's digest at `apply` (or at the bootstrap's import, or at the first `approve alambique`) |
| `fermento-<alias>` | `sweep-recorded` | agent | the input's digest, per point its number, text digest and recommendation, the stage and task heads seen, the skills the tools load from outside the project (names only, REQ-FER-10) |
| `fermento-<alias>` | `proposal-recorded` | agent | identifier, point number and text digest, target (`alambique` or `rules`), base digest, units and their new text, status-key changes, predicted result digest |
| `fermento-<alias>` | `proposal-withdrawn` | fabrewk | identifier, why (point gone) |
| `alambique` or `rules` (its target) | `proposal-incorporated` | person, signed | identifier, resulting digest; for `rules`, all the rules after it |
| `fermento-<alias>` | `proposal-discarded` | person, signed | identifier |
| `alambique` | `direct-edit-approved` | person, signed | old and new digests, the difference |
| `stage-<id>` | `stage-proposed` | any | goal, tasks in order, budget, approved checks |
| `stage-<id>` | `stage-authorized` | person, signed | each task's identifier, digest and head, widenings named |
| `stage-<id>` | `stage-closed` | person, signed | the stage and its head, and the sweeps it relied on (D-074) |
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
  just before it), `limits`, `shown` (the digest of what `status <id>` showed, D-062 point 3) and `widened`.
  `stage-authorized`: exactly `stage`, `digest`, `head`, `tasks` (each `task`, `digest`, `head`, `limits`), `shown`
  and `widened`. `task-accepted` holds at least `task` and `head`, bound the same way; the rest of its payload is
  phase 9's. `task-rejected`: exactly `task` and `head`, bound the same way; where the record branch is, phase 9 adds
  with the candidate (part 7d3b, D-068).
- A payload of one of these kinds missing a field, holding an unknown one or a field of the wrong JSON type is
  `RECORD_VERSION_UNKNOWN`, as schema 1 says, and wins over any failure of content. Content (a blank text, a tag that
  is not one, a count below 1, a malformed identifier or commit, no requirement or allowed file, a pattern REQ-TASK-7
  refuses, a base commit together with dependencies or neither, a dependency named twice, a check with an empty
  command, a layer key that level may not set or a value not of its type) is `TASK_INVALID` at a command and
  `RECORD_TRANSITION_INVALID` on replay. The replay holds every stage and task stream to the transition table: a
  move from a state the table does not allow, a task naming a stage or a dependency the record does not hold, an
  authorization, acceptance or rejection whose `task` (or `stage`), `digest` or `head` is not its own stream's, and a stage's
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

- `attempt-ended`'s `usage`, as phase 9's opening fixed it (D-076, S-5): exactly `seconds` (an integer or `null`) and
  `tokens` (`null`, or exactly `input`, `output` and `cached`, each an integer or `null`). The rest of the payloads
  of phase 9's kinds is fixed by the part that first builds each.

## 4. States and transitions

**Tasks.** States: `proposed`, `authorized`, `running`, `halted`, `verified`, `accepted`, `integrated` (final),
`rejected` (final).

| Move (event) | From | To | Who | Guards (refusal) |
| --- | --- | --- | --- | --- |
| `task-proposed` | — | `proposed` | anyone; grants nothing | stage exists and is open (`STAGE_CLOSED`); fields and patterns valid (`TASK_INVALID`, REQ-TASK-7); no pattern reaches a protected name (`PROTECTED_PATH`); protected paths literal (`PROTECTED_PATH_WILDCARD`) |
| `task-authorized`, or covered by `stage-authorized` | `proposed`, `halted`; `verified` and `accepted` for a revalidation, keeping the candidate (REQ-GATE-8, REQ-RUN-9) | `authorized` | maestro, `--say` | role (`ROLE_FORBIDDEN`); words (`WORDS_MISSING`, `WORDS_COPIED`); digest and head current (`TASK_CHANGED`); lane guard again (`PROTECTED_PATH`); protected paths literal again (`PROTECTED_PATH_WILDCARD`, D-068); widenings named (`WIDENING_UNNAMED`); observable limits (`LIMIT_UNOBSERVABLE`) |
| `run-started`, then `attempt-started` | `authorized`; `verified` with stale evidence, as a revalidation (REQ-RUN-9) | `running` | maestro, `--say` (D-062) | policy adopted (`POLICY_ABSENT`); dependencies integrated (`DEPENDENCY_OPEN`); a dependent task's base fixed (`BASE_MOVED`, REQ-TASK-3); no other task running (`LOCK_HELD`); REQ-RUN-2; alambique approved (`ALAMBIQUE_UNAPPROVED`, before confinement, D-073); confinement (`CONFINEMENT_UNAVAILABLE`) |
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
incorporated (`PROPOSAL_STALE`, `ALAMBIQUE_CHANGED`, `ALAMBIQUE_UNAPPROVED` for an alambique target, `UNIT_AMBIGUOUS`; D-073) or discarded by a maestro with words, and
withdrawn by Fabrewk. Each guard is labelled (REQ-SEC-5): cooperative are the guards of a move that grants nothing
(`task-proposed`, `stage-proposed`, whose events any process can write and whose fields the authorization checks
again), `WORDS_COPIED` (the signer can always type the example), `LOCK_HELD` (an advisory lock), REQ-RUN-2 (the
tools found on the machine) and `CREDENTIAL_SUSPECTED` (a heuristic); every other guard is effective, enforced by the
replay, by the signature of the person who moves after it was checked, by the confinement that keeps agents from the
domain, or by the Git host. `run-started` takes the person's words like every person's move (D-062 point 2), and the
authorizations check again that protected paths are listed literally (D-068).

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
| `fabrewk assess [--mode adopt\|review] [--publication <mode>] [--alias <alias>] [--key <file>] [--tool <name>]… [--lang <tag>] [--purpose <text>] [--audience <text>] [--excluded <text>]` (D-097) | no | — | `NOT_A_REPOSITORY`, `ALIAS_INVALID` |
| `fabrewk apply <plan-digest> <assess's options> --say` (D-097) | yes | project events | `PLAN_CHANGED`, `VERDICT_NONE`, `MODE_NOT_AVAILABLE`, `AGENT_ENVIRONMENT`, `ROLE_FORBIDDEN`, `SETTINGS_INVALID` |
| `fabrewk adopt --say [--widen <key>]…` | yes | `policy-adopted` | `POLICY_UNCOMMITTED`, `WIDENING_UNNAMED`, `ROLE_NOT_ENFORCED`, `SETTINGS_INVALID` |
| `fabrewk propose <file>` | yes | `stage-proposed`, `task-proposed` | `TASK_INVALID`, `STAGE_CLOSED` |
| `fabrewk authorize <stage\|task> --seen <digest> --say [--widen <key>]… [--run]` | yes | `stage-authorized`, `task-authorized`; with `--run`, also as `run` | section 4 |
| `fabrewk run <task\|stage> --say` | yes | `run-started` and the run's events | section 4 |
| `fabrewk accept <task> --seen <digest> --say` | yes | `task-accepted`, `merge-failed`, `task-integrated` | section 4 (D-086 point 2) |
| `fabrewk reject <task> --say` | yes | `task-rejected` | `MERGE_IN_PROGRESS` |
| `fabrewk sweep <alias> --answer <file>` | yes | `sweep-recorded`, `proposal-*` | REQ-FER-1, REQ-FER-3 |
| `fabrewk approve <proposal\|alambique> --say` | yes | `proposal-incorporated`, `direct-edit-approved`, `alambique-baseline` | `PROPOSAL_STALE`, `ALAMBIQUE_CHANGED`, `ALAMBIQUE_UNAPPROVED`, `ALAMBIQUE_MISSING`, `UNIT_AMBIGUOUS` |
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

As part 7d2 fixed it (D-053, D-055): `check` verifies C1 to C4 and C8 to C11; C5, C6, C7 and C12 were declared with
phase 8 (C6 and C12 came with part 8a, C7 with 8b, C5 with 8c), when views, separators and the alambique's approval exist. Warnings (`DOMAIN_UNKNOWN_ENTRY`,
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
folder that is one is `PRIVATE_FOLDER_PERMISSIONS`; Git's signing key is read only from a regular file outside this
repository and outside every Fabrewk domain, as written and as it resolves, and never when Git takes the setting itself from a file in a domain (P7D2-17,
P7D2-22, P7D2-30); what Git itself reads in order to run, its configuration and what it includes, is Git's and the
person's, outside this boundary (D-058), and `--version` reads nothing (P7D2-25). What cannot be
examined on the way to the private folder, `machine.toml` or the person's own file is an error, never an absence
(P7D2-24, P7D2-26), and a domain that cannot be examined is `CHECK_UNKNOWN`, not absent (P7D2-27). A private
folder inside this repository or inside any Fabrewk domain (a path with a `.brewstillery` component), written there or
reached through a link, is `PRIVATE_FOLDER_PERMISSIONS`, so that no maintainer's material is read as it (AD-15,
P7D2-28, P7D2-29); one inside another repository that holds no domain is not detected (6.51 says where the folder
is; it does not ask Fabrewk to search every repository, and the suites' fixtures live inside this one). A domain or a maintainer's directory that cannot be listed leaves C1 `CHECK_UNKNOWN`.

## 7. Refusals

Each code has a catalogue entry with its reason and next step; section 11.4 gives each a test. Codes not described in
the requirements above:

| Code | When | Next step |
| --- | --- | --- |
| `KEY_UNKNOWN` | `config <key>` names no declared key | `fabrewk config` |
| `TERMINAL_REQUIRED` | `onboard` without a terminal | the agent route: `fabrewk assess` |
| `NOT_A_REPOSITORY` | not inside a Git working tree | run it at the repository's root |
| `PROPOSAL_UNKNOWN` | no such proposal | `fabrewk status` |
| `ALAMBIQUE_MISSING` | `approve alambique` finds no `ALAMBIQUE.md`, or one that is not a regular file reached without a link, or not valid text (D-073) | restore the alambique |
| `ANSWER_INVALID` | a sweep's answer file does not have the shape `--help` prints, or one of its texts would break the separator (D-072) | correct the file; `fabrewk sweep --help` prints its format |
| `POLICY_PENDING` | the file differs from the adopted policy (a warning) | `fabrewk adopt --say …` |
| `GITIGNORE_CHANGED` | `.brewstillery/.gitignore` differs from what Fabrewk wrote | restore it |
| `MAINTAINER_UNLISTED`, `FERMENTO_MISSING` | a maintainer directory and the policy disagree | `fabrewk adopt` or create the fermento |
| `CHECK_UNKNOWN` | a verification could not run | the missing tool or reason, named |
| `RECORD_HEAD_MOVED` | a stream's head moved under the lock, so something wrote without it; nothing was written (D-047) | run the command again |

All codes, in one list: `AGENT_ENVIRONMENT`, `ALAMBIQUE_CHANGED`, `ALAMBIQUE_MISSING`, `ALAMBIQUE_UNAPPROVED`, `ALIAS_INVALID`,
`ALIAS_MISMATCH`, `ANSWER_INVALID`, `ATTEMPT_ENDED`, `ATTEMPT_REPLACED`, `BASE_MOVED`, `BUDGET_EXHAUSTED`, `CHECK_DUPLICATE`,
`CHECK_UNKNOWN`, `CONFINEMENT_UNAVAILABLE`, `CREDENTIAL_SUSPECTED`, `DEPENDENCY_OPEN`, `DISK_LOW`, `DOMAIN_NOT_IGNORED`,
`DOMAIN_UNKNOWN_ENTRY`, `EVIDENCE_STALE`, `EXTENSION_DIGEST`, `EXTENSION_OUTPUT_INVALID`, `FERMENTO_MISSING`,
`GITIGNORE_CHANGED`, `HEAD_DETACHED`, `INPUT_CHANGED`, `INPUT_UNCOMMITTED`, `KEY_UNKNOWN`, `LANE_VIOLATION`,
`LIMIT_UNOBSERVABLE`, `LOCK_HELD`, `MAINTAINER_UNLISTED`, `MARK_INVALID`, `MERGE_IN_PROGRESS`, `MODE_NOT_AVAILABLE`,
`MOVE_NOT_ALLOWED`, `NOT_A_REPOSITORY`, `NO_PROGRESS`, `OUTCOME_UNKNOWN`, `PLAN_CHANGED`, `PLATFORM_UNSUPPORTED`, `POINT_CHANGED`,
`POLICY_ABSENT`, `POLICY_PENDING`, `POLICY_UNCOMMITTED`, `PRIVATE_FOLDER_INCLUDED`, `PRIVATE_FOLDER_PERMISSIONS`, `PROPOSAL_STALE`,
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
| `limits.freeDiskMegabytes` | integer | 2048 | N | REQ-RUN-10 (D-075) |
| `checks.<name>.command` | list of strings | required | — | The command, without a shell |
| `checks.<name>.network`, `.minutes`, `.tools` | boolean, integer, list | `false`, 30, empty | W, W, — | REQ-GATE-2, REQ-GATE-3 |
| `review.differentProvider` | `off`, `prefer`, `require` | `prefer` | N | REQ-GATE-4 (`off` is the widest) |
| `gate.protectedPaths` | list of paths | `Cargo.toml`, `Cargo.lock`, `.github/`, every file a check's command names and the hooks directory inside the working tree (D-076) | N | REQ-GATE-9 |
| `gate.protectedNames` | list of names | `.gitattributes`, `.gitmodules` and each adapter's instruction names (D-076) | N | REQ-GATE-9 |
| `git.signOff` | boolean | `false` | — | REQ-RUN-4 |
| `extensions.<name>.digest`, `.network`, `.minutes` | string, boolean, integer | none, `false`, 30 | —, W, W | REQ-EXT-1, AD-5, D-094 |

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
| `ANSWER_INVALID` | REQ-FER-3 | A sweep's answer that is not JSON, holds a field of the wrong type or an unknown one, names a point twice, holds an edit with no unit or a unit its target does not take, or holds the separator's marker as a line of a text (D-072) | 8 |
| `ATTEMPT_ENDED` | REQ-RUN-1 | A second `attempt-ended` for one attempt | 7 |
| `ATTEMPT_REPLACED` | REQ-RUN-5 | A result for an attempt a later one replaced (PD-08) | 9 |
| `BASE_MOVED` | REQ-GATE-8, REQ-TASK-3 | The product tree changed after acceptance (PD-13); a dependent task's `run` after another product change followed its dependency's integration | 9 |
| `BUDGET_EXHAUSTED` | REQ-TASK-5, REQ-RUN-2 | `dispatches` spent; `minutes` spent by a fake that sleeps past the limit | 9 |
| `CHECK_DUPLICATE` | REQ-GATE-1 | A stage and the project defining one check name differently (PD-24) | 7 |
| `CHECK_UNKNOWN` | Section 6 | `check` with `git` absent from `PATH`; with the Git history unreadable; with a domain file it must scan unreadable (C9) | 7 |
| `CONFINEMENT_UNAVAILABLE` | REQ-SEC-2, REQ-SEC-1 | `run` with `bwrap` absent; a failed preflight (PD-27) | 9 |
| `CREDENTIAL_SUSPECTED` | REQ-SEC-3 | Each pattern of REQ-SEC-3 in a file under the domain (`check`) and in a candidate (`run`); near misses (`AKIA` with 15 characters, `id_ed25519.pub`, `sk-` with 19) are not refused | 7, 9 |
| `DEPENDENCY_OPEN` | REQ-TASK-3 | `run` on a task whose dependency is not integrated | 7 |
| `DISK_LOW` | REQ-RUN-10 | `limits.freeDiskMegabytes` above the free space, before an attempt's copy and before a check's; a `df` that fails (D-075) | 9 |
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
| `INSTALLATION_INCOMPLETE` | REQ-INS-3 (D-097 point 7) | `propose` after an `apply` killed before `installed`; `check` reports it; the same `apply` run again finishes it | 10 |
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
| `PRIVATE_FOLDER_INCLUDED` | REQ-INS-1 (D-100) | `assess` with Git's `include.path` naming a file of the private folder: it stops before any other Git | 10 |
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
| `TERMINAL_REQUIRED` | Section 7 | `onboard` with no controlling terminal (`/dev/tty` cannot be opened, D-097 point 8) | 10 |
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
| REQ-FER-10 | A fake module whose user folder holds a skill named like an approved one: the sweep records its name and the answer warns; a folder made unreadable is shown as unknown; no file under the folder is written or changed and no content of one reaches an event or a view | 8 |

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
| REQ-INS-8 | `apply` with the three answers and with none, in `en` and `es`: the alambique's bytes equal the catalogue's skeleton holding the answers as given; an alambique present is byte-identical after and its digest is the baseline's; `check` passes C7 after `apply` (D-097) | 10 |

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

### Amended at phase 9's opening (D-075, D-076)

Each row adds to its requirement's criterion above; the part is the one of `plan.md`, "Phase 9".

| Requirement | Also accepted when | Part |
| --- | --- | --- |
| REQ-SEC-1 | From a cell with a network: no abstract socket of the host and nothing on its loopback or its own address answers; `config` states the open outbound network as a residual risk. A probe writes in `/work`, its home and its `/tmp` and nowhere else, and the three are gone when the attempt ends. A key planted in the host's session keyring and one of default permissions in its user keyring, their serial numbers given to the probe, are neither found, read, linked nor used through a cryptographic socket, from the cell and from a sandbox nested in it. A process in its own session that ignores signals is gone after the time limit, and after Fabrewk is killed | 9a |
| REQ-SEC-5 | `config` lists the residual risks; a test fails when a backend or an adapter declares one with no words in every catalogue | 9a |
| REQ-RUN-2 | A required tool that resolves outside the cell's system directories: `TOOL_MISSING`, the fake CLI uncalled | 9e |
| REQ-RUN-3 | A nested `.git` directory and a `.git` file (`PROTECTED_PATH`, no submodule entry); a `.gitattributes` that asks for a conversion (the candidate keeps the bytes); a `.gitignore` edited to hide a new file (the file is in the change set); a pipe and a socket left in the copy; a path that is not valid text | 9b |
| REQ-RUN-7 | An attempt ended `unknown` by a later command records `seconds` as `null`, counts its whole allowed time against `minutes`, and the view shows the sum as unknown | 9e |
| REQ-RUN-10 | `DISK_LOW`'s test; `status` shows the sizes of `local/evidence/` and `local/tmp/` | 9b |
| REQ-GATE-9 | Allowed files `src/**` with a fake that writes `src/AGENTS.md`, `src/.claude/settings.json` and `src/.gitattributes`: each refused `PROTECTED_PATH_WILDCARD` at the change set, and at integration with the change-set guard off; the same paths listed literally are kept; a tracked hooks directory under a wildcard is refused | 9b, 9g |
| REQ-REG-10, REQ-GATE-8 | A repository whose `core.hooksPath` is a tracked directory with a `reference-transaction` hook: no write command runs it; through `accept`, a candidate that changes it does not run; a candidate's `.gitattributes` naming a configured filter leaves the working tree's bytes those of the objects | 9b, 9g |
| REQ-EXT-4 | A permitted extension installed outside the repository and the system directories runs; one whose interpreter lies outside both fails `TOOL_FAILED`, saying so | 9h |

## 13. What each later phase must show

A phase's acceptance (plan rows 7 to 10) includes every criterion of section 12 assigned to it, every past-defect case
and refusal assigned to it in 11.3 and 11.4, the mutation rule of 11.6 for its code, and a clean clone (11.2 rule 7).
A criterion assigned to two phases is first met with what the earlier one builds and met whole in the later one.

- **Phase 7 (core, part 1).** First the skeleton and its counter, and the maestro's freeze of the budget. Then the
  record, settings, identity, the transition table and the command surface: REG, SET, ID, TASK (but TASK-5 and TASK-6),
  CLI, BUD, DOM-2 to DOM-5 and DOM-7 on fixture records, VIEW-4, SEC-3 at `check`, SEC-4, SEC-5; the crash suite on the
  record; PD-01 to PD-06, PD-10 (record), PD-15, PD-17, PD-24, PD-30 to PD-32, PD-34 to PD-38, PD-46 (record).
- **Phase 8 (core, part 2).** FER (REQ-FER-10 included, D-072), VIEW-1 to VIEW-3, TASK-6; C5, C6, C7 and C12; the views of PD-33 and PD-39; PD-40 to PD-43. Cut into 8a, 8b and 8c (D-072).
- **Phase 9 (adapters and confinement).** RUN (RUN-9 included), GATE, TASK-5, SEC-1, SEC-2, SEC-3 in the change set, SEC-7, EXT, SET-9's
  dispatch; PD-07 to PD-14, PD-16, PD-18 to PD-29, PD-33, PD-44 to PD-46; the flows F-1 to F-3 with fake tools; one real
  dispatch per provider, preflight first. REQ-RUN-10 and the amendments of D-075 and D-076. Cut into 9a to 9i (D-075).
- **Phase 10 (installation).** INS (REQ-INS-8 included, D-097), DOM-1, DOM-6, SEC-6, BUD-5's release notices; F-1 and F-2
  from a real `apply`, the test helper's project events retired; the crash suite over `apply`. Cut into 10a to 10d (D-097).
- **Phases 11 and 12 (the pilot).** The pilot's specification picks its six provoked refusals (6.48, D-009) from 11.4
  and holds both arms to the same measures.
