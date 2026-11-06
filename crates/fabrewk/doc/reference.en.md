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
