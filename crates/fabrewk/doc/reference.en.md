# fabrewk: command reference

Generated from the declaration of commands; it lists exactly the commands that exist.

## `fabrewk status [<id>] [--json]`

Shows the record's health, the policy in force and the state of each stage and task.

- `<id>`: A stage or a task, by its identifier (s-… or t-…): shows it with its events.
- `--json`: Gives the output as one JSON document with a schema field.
- `--help`: Shows this help.

Moves it may cause: none.

Documents with --json: `fabrewk.status/1`, `fabrewk.subject/1`

## `fabrewk config [<key>] [--json]`

Shows every effective setting with its source, the changes not adopted, the keys, what this machine has and every control.

- `<key>`: One setting, by its key path: shows only its value and source.
- `--json`: Gives the output as one JSON document with a schema field.
- `--help`: Shows this help.

Moves it may cause: none.

Documents with --json: `fabrewk.config/1`, `fabrewk.value/1`

Refusals

- `POLICY_ABSENT`: No policy is adopted, so there is no effective configuration and no maintainer.
- `SETTINGS_INVALID`: A settings file does not read or validate.
- `SETTINGS_KEY_NOT_PERSONAL`: A personal file holds a key that only the project file may set.
- `ALIAS_MISMATCH`: The personal file names another maintainer than its directory.
- `PRIVATE_FOLDER_PERMISSIONS`: The private folder gives access to someone other than its owner, so Fabrewk does not read it.
- `KEY_UNKNOWN`: No setting is declared at this key path.

## `fabrewk check [--json]`

Verifies the settings, the policy, the record, the domain, the credentials and the private folder.

- `--json`: Gives the output as one JSON document with a schema field.
- `--help`: Shows this help.

Moves it may cause: none.

Documents with --json: `fabrewk.check/1`

| | Verifies | Codes | Control |
| --- | --- | --- | --- |
| C1 | The settings files read and validate, and each maintainer's file holds only personal keys. | `SETTINGS_INVALID`, `SETTINGS_KEY_NOT_PERSONAL`, `ALIAS_MISMATCH` | effective |
| C2 | A policy is adopted, and the project file equals it or its change is pending. | `POLICY_ABSENT`, `POLICY_PENDING` | effective |
| C3 | The record replays: versions, digests, canonical form, chains, forks, transitions, signatures, identifiers. | `RECORD_VERSION_UNKNOWN`, `RECORD_BROKEN`, `RECORD_DIGEST_MISMATCH`, `RECORD_NONCANONICAL`, `RECORD_FORK`, `RECORD_TRANSITION_INVALID`, `ATTEMPT_ENDED`, `RECORD_SIGNATURE_INVALID`, `RECORD_ID_COLLISION` | effective |
| C4 | The record is present and has not rolled back: effective where Git's history is walked, cooperative on one machine. | `RECORD_ABSENT`, `RECORD_ROLLBACK`, `MARK_INVALID` | effective |
| C8 | The domain's entries, the aliases, its ignore file, and the product's ignore rule. | `DOMAIN_UNKNOWN_ENTRY`, `ALIAS_INVALID`, `GITIGNORE_CHANGED`, `DOMAIN_NOT_IGNORED` | effective |
| C9 | No file looks like a credential: a heuristic. | `CREDENTIAL_SUSPECTED` | cooperative |
| C10 | The record's files are committed where the publication mode commits them. | `RECORD_UNCOMMITTED` | effective |
| C11 | The private folder gives access to its owner only. | `PRIVATE_FOLDER_PERMISSIONS` | effective |

## `fabrewk adopt --say <words> [--say-lang <tag>] [--widen <key>]… [--json]`

A maestro adopts the project file as the policy in force, naming every widening it makes.

- `--say <words>`: Your own words for this decision, kept verbatim and signed. Words copied from an example, such as “I agree.”, are refused.
- `--say-lang <tag>`: The language tag of your words; by default, your languages.agentInteraction.
- `--widen <key>`: Names a widening of authority this move makes, by its key path; give it once for each.
- `--json`: Gives the output as one JSON document with a schema field.
- `--help`: Shows this help.

Moves it may cause: `policy-adopted` (policy adopted).

Documents with --json: `fabrewk.adopted/1`

Refusals

- `AGENT_ENVIRONMENT`: This runs inside an agent or an extension, whose markers are in the environment, and only a person makes this move. The check is cooperative: a process can clear its environment.
- `ROLE_FORBIDDEN`: The key signing this move holds no role in the policy that may make it.
- `WORDS_MISSING`: A person's move needs your own words, given with --say: not empty, and valid UTF-8 text.
- `WORDS_COPIED`: The words given are the example that help prints, not your own.
- `SETTINGS_INVALID`: A settings file does not read or validate.
- `SETTINGS_KEY_NOT_PERSONAL`: A personal file holds a key that only the project file may set.
- `ALIAS_MISMATCH`: The personal file names another maintainer than its directory.
- `ROLE_NOT_ENFORCED`: The policy lists a supervisor, a role Fabrewk does not enforce yet.
- `POLICY_UNCOMMITTED`: The project file is not as committed at the record branch's head.
- `WIDENING_UNNAMED`: A change widens authority without being named.
- `RECORD_ABSENT`: There is no record here.
- `POLICY_ABSENT`: No policy is adopted, so there is no effective configuration and no maintainer.
- `RECORD_BROKEN`: The record is broken: its files are not one unbroken chain.
- `RECORD_DIGEST_MISMATCH`: An event file's content is not the one its name records.
- `RECORD_NONCANONICAL`: An event file is not in canonical form, so its digest cannot be trusted.
- `RECORD_FORK`: Two events stand at the same place of one stream: two clones wrote there.
- `RECORD_TRANSITION_INVALID`: An event stands where the transition table does not allow it.
- `ATTEMPT_ENDED`: The record ends an attempt that is not open.
- `RECORD_SIGNATURE_INVALID`: A person's event is not signed by the key and role the policy gives its signer.
- `RECORD_VERSION_UNKNOWN`: An event is not of a schema this version of Fabrewk knows.
- `RECORD_ID_COLLISION`: Two stages or two tasks were given the same identifier.
- `RECORD_ROLLBACK`: The record lost events it held before: a rollback.
- `MARK_INVALID`: This machine's mark of the record (local/cache/heads.json) is damaged or names an event the record does not hold there.
- `CHECK_UNKNOWN`: A verification could not run, so its result is unknown, and an unknown result is never a pass.
- `DOMAIN_NOT_IGNORED`: The product's repository does not ignore .brewstillery/, so a commit there would publish it.
- `NOT_A_REPOSITORY`: This is not the root of the repository's main working tree.
- `HEAD_DETACHED`: Git's head names no branch, so there is no record branch.
- `PRIVATE_FOLDER_PERMISSIONS`: The private folder gives access to someone other than its owner, so Fabrewk does not read it.
- `LOCK_HELD`: Another Fabrewk command holds the lock of this repository's record.
- `RECORD_HEAD_MOVED`: Something wrote in the record without its lock while this command ran; this command wrote nothing.
- `RECORD_COMMIT_RACE`: The branch moved while Fabrewk committed the record; its events are written but not committed.
- `TOOL_FAILED`: An external tool failed; what it printed is kept as evidence, never shown here.
- `TOOL_MISSING`: A tool this needs is not on this machine.

## `fabrewk propose <file> [--json]`

Proposes a stage and its tasks, or tasks for a stage of the record, from a JSON file; it authorizes nothing.

- `<file>`: The JSON file that describes the stage and its tasks; its format is below.
- `--json`: Gives the output as one JSON document with a schema field.
- `--help`: Shows this help.

Moves it may cause: `stage-proposed` (stage proposed), `task-proposed` (task proposed).

Documents with --json: `fabrewk.proposed/1`

The file's format

```text
stage: an object with these fields, or the identifier (s-…) of a stage of the record
  lang: a language tag (BCP 47)
  goal: text
  budget: an object with these fields
    dispatches: a whole number, 1 or more
    minutes: a whole number, 1 or more
    tokens (optional): a whole number, 1 or more
  checks: an object whose fields each hold an object with these fields
    command: a list, each item text
    network: true or false
    minutes: a whole number, 1 or more
    tools: a list, each item text
  settings: an object whose fields each hold a setting's value
tasks: a list of one task or more, each an object with these fields
  name (optional): a name for the task, which a later task of the file may give among its dependencies
  lang: a language tag (BCP 47)
  goal: text
  requirements: a list, each item an object with these fields
    text: text
    criterion: text
  base: a commit (40 or 64 hexadecimal digits), or null
  dependencies: a list, each item a task's identifier (t-…), or the name of a task above it in the file
  allowed: a list, each item text
  profiles: an object with these fields
    build: text
    review: text
  checks: a list, each item text
  limits: an object with these fields
    rounds: a whole number, 1 or more
    dispatches: a whole number, 1 or more
    minutes: a whole number, 1 or more
    tokens (optional): a whole number, 1 or more
  delegationLanguage: a language tag (BCP 47)
  settings: an object whose fields each hold a setting's value
```

Refusals

- `STAGE_CLOSED`: The stage is closed, so no task can be added to it.
- `TASK_INVALID`: A stage's or a task's fields are not valid.
- `PROTECTED_PATH`: The allowed files reach a protected path.
- `PROTECTED_PATH_WILDCARD`: A wildcard among the allowed files matches a protected path.
- `CHECK_DUPLICATE`: One check has two different definitions.
- `RECORD_ABSENT`: There is no record here.
- `POLICY_ABSENT`: No policy is adopted, so there is no effective configuration and no maintainer.
- `RECORD_BROKEN`: The record is broken: its files are not one unbroken chain.
- `RECORD_DIGEST_MISMATCH`: An event file's content is not the one its name records.
- `RECORD_NONCANONICAL`: An event file is not in canonical form, so its digest cannot be trusted.
- `RECORD_FORK`: Two events stand at the same place of one stream: two clones wrote there.
- `RECORD_TRANSITION_INVALID`: An event stands where the transition table does not allow it.
- `ATTEMPT_ENDED`: The record ends an attempt that is not open.
- `RECORD_SIGNATURE_INVALID`: A person's event is not signed by the key and role the policy gives its signer.
- `RECORD_VERSION_UNKNOWN`: An event is not of a schema this version of Fabrewk knows.
- `RECORD_ID_COLLISION`: Two stages or two tasks were given the same identifier.
- `RECORD_ROLLBACK`: The record lost events it held before: a rollback.
- `MARK_INVALID`: This machine's mark of the record (local/cache/heads.json) is damaged or names an event the record does not hold there.
- `CHECK_UNKNOWN`: A verification could not run, so its result is unknown, and an unknown result is never a pass.
- `DOMAIN_NOT_IGNORED`: The product's repository does not ignore .brewstillery/, so a commit there would publish it.
- `NOT_A_REPOSITORY`: This is not the root of the repository's main working tree.
- `HEAD_DETACHED`: Git's head names no branch, so there is no record branch.
- `PRIVATE_FOLDER_PERMISSIONS`: The private folder gives access to someone other than its owner, so Fabrewk does not read it.
- `SETTINGS_INVALID`: A settings file does not read or validate.
- `LOCK_HELD`: Another Fabrewk command holds the lock of this repository's record.
- `RECORD_HEAD_MOVED`: Something wrote in the record without its lock while this command ran; this command wrote nothing.
- `RECORD_COMMIT_RACE`: The branch moved while Fabrewk committed the record; its events are written but not committed.
- `TOOL_FAILED`: An external tool failed; what it printed is kept as evidence, never shown here.
- `TOOL_MISSING`: A tool this needs is not on this machine.

## The program alone

- `fabrewk --help`: Shows this help.
- `fabrewk --version`: Shows Fabrewk's version.

Documents with --json: `fabrewk.refused/1`, `fabrewk.usage/1`, `fabrewk.internal/1`, `fabrewk.version/1`

## Exit statuses

- `0`: done.
- `1`: refused: a code was printed, and nothing was written.
- `2`: usage error: the command line was not accepted.
- `3`: the check found a problem.
- `70`: internal error.
