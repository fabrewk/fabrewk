# Working with Fabrewk

Fabrewk keeps a project's description, its decisions and the work of its agents in one folder of the repository,
`.brewstillery/`, and lets agents work only on what a person authorized, confined, with every decision signed. This
manual explains the parts and the cycle of work. Each command is described in full, with its options and refusals,
in the [command reference](../../crates/fabrewk/doc/reference.en.md).

## The parts, named after a distillery

- **Brewstillery**, the distillery, is Fabrewk's domain in the repository: the folder `.brewstillery/`. Everything
  Fabrewk writes lives there, but for a delimited block in `AGENTS.md` and `CLAUDE.md` and, when the folder is not
  published, one line in `.git/info/exclude`.
- **The alambique**, `.brewstillery/ALAMBIQUE.md`, describes what the project must be. Nobody edits it directly: its
  changes come from the fermentos, and only a maestro approves what enters it.
- **A fermento**, `.brewstillery/<alias>/fermento.md`, is each maintainer's own file, with what they want to add,
  remove or discuss. The maintainer writes above its separator; agents answer below it.
- **The record**, under `.brewstillery/data/`, holds every event as an immutable file: proposals, authorizations,
  attempts, checks, reviews, acceptances. A person's event is signed with their SSH key. `STATUS.md` and `RULES.md`
  are views generated from it; never edit them.
- **The settings**: `.brewstillery/settings.toml` is the project's, in force only once a maestro adopts it;
  `.brewstillery/<alias>/settings.toml` holds a maintainer's preferences, which never widen any permission.
- **The local area**, `.brewstillery/local/`, is never versioned: evidence, caches, temporary copies and the lock.
- **The private folder**, `~/.config/fabrewk/`, is outside every repository and hidden from every agent.

## Who does what

- A **maestro** makes the decisions: adopts the settings, authorizes and accepts work, approves the alambique. Each
  of these moves takes the maestro's own words (`--say`), recorded as written and signed with their key.
- A **contributor** proposes, and grants nothing.
- An **agent** works only inside a confined copy, on a task a maestro authorized. A person's move refuses to run
  inside an agent (`AGENT_ENVIRONMENT`): a cooperative check, since a process can clear its environment; the
  signature is what binds the move to the maestro.

Words copied from an example that help prints are refused (`WORDS_COPIED`). A move that changes what agents may do
names each widening it makes with `--widen <key>`.

## The settings

The installation writes the first `settings.toml` with the maestro, the languages and the tools chosen. It declares
no model and no profile: before any run, a maestro adds them (the models a tool may use and the profiles that join a
tool, a model and an effort for building and for reviewing), then adopts the file:

    fabrewk config          # the effective settings, with their source, and what is not adopted yet
    fabrewk adopt --say '<your words>'

`fabrewk config` also shows what this machine has: the tools found, the confinement and every control, each marked
as effective or cooperative.

## The cycle of the alambique

1. A maintainer writes points in their fermento, above the separator.
2. The integrator, an agent, reads them and records its answer with `fabrewk sweep <alias> --answer <file>`: what it
   recommends for each point and, where there is consensus, the alambique's text it drafts. The answer is written
   below the separator, and each drafted edit is a pending proposal.
3. A maestro reads the proposal in `fabrewk status` and approves it, `fabrewk approve <proposal> --say '<words>'`,
   which applies the edit and commits the alambique alone, or discards it, `fabrewk discard <proposal> --say
   '<words>'`. A proposal goes stale when its point, the alambique or its text change.
4. An alambique edited by hand is not approved until a maestro approves it as it stands: `fabrewk approve alambique
   --say '<words>'`. While it is not approved, no run starts and no task is accepted (`ALAMBIQUE_UNAPPROVED`).

## The cycle of a task

1. **Propose.** Anyone writes a stage and its tasks as a JSON file and records it with `fabrewk propose <file>`. A
   task names its goal, its requirements with their criteria, the files it may change, its checks, its build and
   review profiles and its limits; no task may reach `.git` or `.brewstillery/` (`PROTECTED_PATH`). Proposing grants
   nothing. `fabrewk propose --help` prints the file's format.
2. **Authorize.** A maestro reads the task or the stage in `fabrewk status <id>`, which prints a digest of what was
   shown, and authorizes it: `fabrewk authorize <id> --seen <digest> --say '<words>'`. If the subject changed since it
   was read, the move is refused (`TASK_CHANGED`). With `--run`, the run starts in the same command.
3. **Run.** `fabrewk run <id> --say '<words>'` runs rounds: the builder agent works in a confined copy of the base
   tree, without the repository's `.git`, with no access to the private folder and only the network its provider
   needs; Fabrewk records its candidate, runs the checks, and dispatches the reviewer on the same candidate. The run
   ends with a verified task, or halts and waits for a person (exit status 4), naming why. `run <stage>` runs the
   stage's first task that may start a run.
4. **Accept or reject.** A maestro reads the verified candidate in `fabrewk status <task>` and accepts it, `fabrewk
   accept <task> --seen <digest> --say '<words>'`, which integrates it into the branch in the same command, or
   rejects it, `fabrewk reject <task> --say '<words>'`.
5. **Close.** When every task of a stage has ended, every fermento was swept after the last of them, the alambique
   is as last approved and the record is committed, a maestro closes the stage: `fabrewk close <stage> --say
   '<words>'`.

A task that changes a path it may not change, or that leaves a suspected credential in its candidate, is halted
(`LANE_VIOLATION`, `CREDENTIAL_SUSPECTED`). A run killed part-way is halted by the next command that writes.

## Checking the repository

- `fabrewk status` shows the record's health, the policy in force and the state of each stage and task.
- `fabrewk check` verifies the settings, the policy, the record, the domain, the credentials and the private folder;
  it exits 3 when it finds an error; warnings alone, such as settings not adopted yet, exit 0.

`.brewstillery/bootstrap/` records Fabrewk's own construction by hand. It exists only in Fabrewk's own repository,
never in a project Fabrewk serves. C13 verifies: "After the import, bootstrap/ contains exactly the imported files,
byte for byte, with no links or empty directories." Differences are reported as `BOOTSTRAP_CHANGED`.

Every built-in command but `onboard` takes `--json`, which prints one document with a `schema` field, for agents and
scripts. Every built-in command takes `--help`, which prints text. [The commands](commands.md) lists them all.

## Extensions

A program named `fabrewk-<name>` runs as `fabrewk <name>` when `[extensions.<name>]` in the settings permits it. It
runs confined, writes nothing in the repository itself, and prints only requests (a note, or a task proposal, which
grants nothing), which the core applies after it exits successfully. `fabrewk config` lists each extension with its
path and digest.
