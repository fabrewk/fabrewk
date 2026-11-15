# fabrewk: command reference

Generated from the declaration of commands; it lists exactly the commands that exist.

## `fabrewk status [<id>] [--json]`

Shows the record's health, the policy in force and the state of each stage and task.

- `<id>`: A stage or a task, by its identifier (s-… or t-…): shows it with its events.
- `--json`: Gives the output as one JSON document with a schema field.
- `--help`: Shows this help.

Moves it may cause: none.

Documents with --json: `fabrewk.status/3`, `fabrewk.subject/4`

## `fabrewk config [<key>] [--json]`

Shows every effective setting with its source, the changes not adopted, the keys, what this machine has and every control.

- `<key>`: One setting, by its key path: shows only its value and source.
- `--json`: Gives the output as one JSON document with a schema field.
- `--help`: Shows this help.

Moves it may cause: none.

Documents with --json: `fabrewk.config/3`, `fabrewk.value/1`

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
| C5 | STATUS.md and RULES.md are what a fresh generation from the record gives. | `VIEW_STALE` | effective |
| C6 | Each fermento has its separator exactly once outside fenced code blocks, inside a rendering Fabrewk writes. | `SEPARATOR_MISSING`, `SEPARATOR_AMBIGUOUS`, `SEPARATOR_DAMAGED` | effective |
| C7 | The alambique is the text a maestro last approved. | `ALAMBIQUE_UNAPPROVED` | effective |
| C8 | The domain's entries, the aliases, its ignore file, and the product's ignore rule. | `DOMAIN_UNKNOWN_ENTRY`, `ALIAS_INVALID`, `GITIGNORE_CHANGED`, `DOMAIN_NOT_IGNORED` | effective |
| C9 | No file looks like a credential: a heuristic. | `CREDENTIAL_SUSPECTED` | cooperative |
| C10 | The record's files are committed where the publication mode commits them. | `RECORD_UNCOMMITTED` | effective |
| C11 | The private folder gives access to its owner only. | `PRIVATE_FOLDER_PERMISSIONS` | effective |
| C12 | Every maintainer directory has a fermento and is listed in the policy. | `MAINTAINER_UNLISTED`, `FERMENTO_MISSING` | effective |

## `fabrewk adopt --say <words> [--say-lang <tag>] [--widen <key>]… [--json]`

A maestro adopts the project file as the policy in force, naming every widening it makes.

- `--say <words>`: Your own words for this decision, kept verbatim and signed. Words copied from an example, such as “I agree.”, are refused.
- `--say-lang <tag>`: The language tag of your words; by default, your languages.agentInteraction.
- `--widen <key>`: Names a widening of authority this move makes, by its key path; give it once for each.
- `--json`: Gives the output as one JSON document with a schema field.
- `--help`: Shows this help.

Moves it may cause: `policy-adopted` (policy adopted).

Documents with --json: `fabrewk.adopted/2`

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
- `LOCK_HELD`: Another Fabrewk command holds the lock of this repository's record, or another task is running: one writer and one run at a time.
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

Documents with --json: `fabrewk.proposed/2`

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
- `PROTECTED_PATH`: The allowed files, or the agent's copy, reach a protected path (.git or .brewstillery).
- `PROTECTED_PATH_WILDCARD`: A wildcard among the allowed files matches a protected path or name.
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
- `LOCK_HELD`: Another Fabrewk command holds the lock of this repository's record, or another task is running: one writer and one run at a time.
- `RECORD_HEAD_MOVED`: Something wrote in the record without its lock while this command ran; this command wrote nothing.
- `RECORD_COMMIT_RACE`: The branch moved while Fabrewk committed the record; its events are written but not committed.
- `TOOL_FAILED`: An external tool failed; what it printed is kept as evidence, never shown here.
- `TOOL_MISSING`: A tool this needs is not on this machine.

## `fabrewk authorize <id> --seen <digest> --say <words> [--say-lang <tag>] [--widen <key>]… [--run] [--json]`

A maestro authorizes a task, or a stage and its tasks, as status showed it, naming every widening it makes.

- `<id>`: The task or the stage, by its identifier (t-… or s-…).
- `--seen <digest>`: The digest that status printed of what you read; if the subject changed since, the move is refused.
- `--say <words>`: Your own words for this decision, kept verbatim and signed. Words copied from an example, such as “I agree.”, are refused.
- `--say-lang <tag>`: The language tag of your words; by default, your languages.agentInteraction.
- `--widen <key>`: Names a widening of authority this move makes, by its key path; give it once for each.
- `--run`: Starts the run in the same command, after the authorization.
- `--json`: Gives the output as one JSON document with a schema field.
- `--help`: Shows this help.

Moves it may cause: `task-authorized` (task authorized), `stage-authorized` (stage authorized), `run-started` (run started), `attempt-started` (attempt started), `attempt-ended` (attempt ended), `candidate-recorded` (candidate recorded), `check-recorded` (check recorded), `review-recorded` (review recorded), `task-verified` (task verified), `task-halted` (task halted).

Documents with --json: `fabrewk.authorized/3`

Refusals

- `AGENT_ENVIRONMENT`: This runs inside an agent or an extension, whose markers are in the environment, and only a person makes this move. The check is cooperative: a process can clear its environment.
- `MOVE_NOT_ALLOWED`: The transition table does not allow this move from the subject's state.
- `ROLE_FORBIDDEN`: The key signing this move holds no role in the policy that may make it.
- `WORDS_MISSING`: A person's move needs your own words, given with --say: not empty, and valid UTF-8 text.
- `WORDS_COPIED`: The words given are the example that help prints, not your own.
- `TASK_CHANGED`: The task, or its stream's head, changed after it was shown.
- `CHECK_DUPLICATE`: One check has two different definitions.
- `PROTECTED_PATH`: The allowed files, or the agent's copy, reach a protected path (.git or .brewstillery).
- `PROTECTED_PATH_WILDCARD`: A wildcard among the allowed files matches a protected path or name.
- `WIDENING_UNNAMED`: A change widens authority without being named.
- `LIMIT_UNOBSERVABLE`: A token limit is set for a tool that does not report the tokens it uses, so the limit could not be kept.
- `DEPENDENCY_OPEN`: A task it depends on is not integrated yet.
- `BASE_MOVED`: The product at the record branch's head is not the one the task's base, or the last of its dependencies' integrations, left.
- `LOCK_HELD`: Another Fabrewk command holds the lock of this repository's record, or another task is running: one writer and one run at a time.
- `ALAMBIQUE_UNAPPROVED`: The alambique is not the text a maestro last approved: it changed by hand, or it was never approved.
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
- `RECORD_HEAD_MOVED`: Something wrote in the record without its lock while this command ran; this command wrote nothing.
- `RECORD_COMMIT_RACE`: The branch moved while Fabrewk committed the record; its events are written but not committed.
- `TOOL_FAILED`: An external tool failed; what it printed is kept as evidence, never shown here.
- `TOOL_MISSING`: A tool this needs is not on this machine.

## `fabrewk accept <task> --seen <digest> --say <words> [--say-lang <tag>] [--json]`

A maestro accepts a task's candidate as status showed it, and Fabrewk integrates it into the record branch.

- `<task>`: The task, by its identifier (t-…).
- `--seen <digest>`: The digest that status printed of what you read; if the subject changed since, the move is refused.
- `--say <words>`: Your own words for this decision, kept verbatim and signed. Words copied from an example, such as “I agree.”, are refused.
- `--say-lang <tag>`: The language tag of your words; by default, your languages.agentInteraction.
- `--json`: Gives the output as one JSON document with a schema field.
- `--help`: Shows this help.

Moves it may cause: `task-accepted` (task accepted), `merge-failed` (integration failed), `task-integrated` (task integrated).

Documents with --json: `fabrewk.accepted/1`

Refusals

- `AGENT_ENVIRONMENT`: This runs inside an agent or an extension, whose markers are in the environment, and only a person makes this move. The check is cooperative: a process can clear its environment.
- `MOVE_NOT_ALLOWED`: The transition table does not allow this move from the subject's state.
- `ROLE_FORBIDDEN`: The key signing this move holds no role in the policy that may make it.
- `WORDS_MISSING`: A person's move needs your own words, given with --say: not empty, and valid UTF-8 text.
- `WORDS_COPIED`: The words given are the example that help prints, not your own.
- `TASK_CHANGED`: The task, or its stream's head, changed after it was shown.
- `EVIDENCE_STALE`: A check's identity no longer matches: the candidate's product tree, the check's definition, the policy, a named tool's version or the platform changed.
- `ALAMBIQUE_UNAPPROVED`: The alambique is not the text a maestro last approved: it changed by hand, or it was never approved.
- `PROTECTED_PATH`: The allowed files, or the agent's copy, reach a protected path (.git or .brewstillery).
- `PROTECTED_PATH_WILDCARD`: A wildcard among the allowed files matches a protected path or name.
- `LANE_VIOLATION`: The agent's copy changes a file outside the task's allowed files, a path that is not valid text, or leaves an entry that is not a file, a link or a directory.
- `BASE_MOVED`: The product at the record branch's head is not the one the task's base, or the last of its dependencies' integrations, left.
- `WORKTREE_DIRTY`: A file the integration would write holds, in the working tree or the index, neither the base's content nor the candidate's: an edit of the person's, which is never overwritten.
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
- `LOCK_HELD`: Another Fabrewk command holds the lock of this repository's record, or another task is running: one writer and one run at a time.
- `RECORD_HEAD_MOVED`: Something wrote in the record without its lock while this command ran; this command wrote nothing.
- `RECORD_COMMIT_RACE`: The branch moved while Fabrewk committed the record; its events are written but not committed.
- `TOOL_FAILED`: An external tool failed; what it printed is kept as evidence, never shown here.
- `TOOL_MISSING`: A tool this needs is not on this machine.

## `fabrewk reject <task> --say <words> [--say-lang <tag>] [--json]`

A maestro rejects a task, saying why.

- `<task>`: The task, by its identifier (t-…).
- `--say <words>`: Your own words for this decision, kept verbatim and signed. Words copied from an example, such as “I agree.”, are refused.
- `--say-lang <tag>`: The language tag of your words; by default, your languages.agentInteraction.
- `--json`: Gives the output as one JSON document with a schema field.
- `--help`: Shows this help.

Moves it may cause: `task-rejected` (task rejected).

Documents with --json: `fabrewk.rejected/2`

Refusals

- `AGENT_ENVIRONMENT`: This runs inside an agent or an extension, whose markers are in the environment, and only a person makes this move. The check is cooperative: a process can clear its environment.
- `MOVE_NOT_ALLOWED`: The transition table does not allow this move from the subject's state.
- `ROLE_FORBIDDEN`: The key signing this move holds no role in the policy that may make it.
- `WORDS_MISSING`: A person's move needs your own words, given with --say: not empty, and valid UTF-8 text.
- `WORDS_COPIED`: The words given are the example that help prints, not your own.
- `MERGE_IN_PROGRESS`: The task's current candidate is accepted, and its integration may be halfway done.
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
- `LOCK_HELD`: Another Fabrewk command holds the lock of this repository's record, or another task is running: one writer and one run at a time.
- `RECORD_HEAD_MOVED`: Something wrote in the record without its lock while this command ran; this command wrote nothing.
- `RECORD_COMMIT_RACE`: The branch moved while Fabrewk committed the record; its events are written but not committed.
- `TOOL_FAILED`: An external tool failed; what it printed is kept as evidence, never shown here.
- `TOOL_MISSING`: A tool this needs is not on this machine.

## `fabrewk run <id> --say <words> [--say-lang <tag>] [--json]`

A maestro starts the confined run of an authorized task, or of a stage's next one, or finishes the integration of an accepted task.

- `<id>`: The task or the stage, by its identifier (t-… or s-…).
- `--say <words>`: Your own words for this decision, kept verbatim and signed. Words copied from an example, such as “I agree.”, are refused.
- `--say-lang <tag>`: The language tag of your words; by default, your languages.agentInteraction.
- `--json`: Gives the output as one JSON document with a schema field.
- `--help`: Shows this help.

Moves it may cause: `run-started` (run started), `attempt-started` (attempt started), `attempt-ended` (attempt ended), `candidate-recorded` (candidate recorded), `check-recorded` (check recorded), `review-recorded` (review recorded), `task-verified` (task verified), `task-halted` (task halted), `merge-failed` (integration failed), `task-integrated` (task integrated).

Documents with --json: `fabrewk.run/2`, `fabrewk.accepted/1`

Refusals

- `AGENT_ENVIRONMENT`: This runs inside an agent or an extension, whose markers are in the environment, and only a person makes this move. The check is cooperative: a process can clear its environment.
- `MOVE_NOT_ALLOWED`: The transition table does not allow this move from the subject's state.
- `ROLE_FORBIDDEN`: The key signing this move holds no role in the policy that may make it.
- `WORDS_MISSING`: A person's move needs your own words, given with --say: not empty, and valid UTF-8 text.
- `WORDS_COPIED`: The words given are the example that help prints, not your own.
- `DEPENDENCY_OPEN`: A task it depends on is not integrated yet.
- `BASE_MOVED`: The product at the record branch's head is not the one the task's base, or the last of its dependencies' integrations, left.
- `LOCK_HELD`: Another Fabrewk command holds the lock of this repository's record, or another task is running: one writer and one run at a time.
- `WIDENING_UNNAMED`: A change widens authority without being named.
- `ALAMBIQUE_UNAPPROVED`: The alambique is not the text a maestro last approved: it changed by hand, or it was never approved.
- `ROUNDS_EXHAUSTED`: The task's review rounds since its authorization are spent; no round more is started.
- `BUDGET_EXHAUSTED`: The task's or its stage's dispatches, minutes or tokens are spent, or what was spent cannot be known; nothing more is dispatched.
- `PROVIDER_MISSING`: review.differentProvider is require, and the review profile's provider is the builder's, or its tool is absent.
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
- `RECORD_HEAD_MOVED`: Something wrote in the record without its lock while this command ran; this command wrote nothing.
- `RECORD_COMMIT_RACE`: The branch moved while Fabrewk committed the record; its events are written but not committed.
- `TOOL_FAILED`: An external tool failed; what it printed is kept as evidence, never shown here.
- `TOOL_MISSING`: A tool this needs is not on this machine.

## `fabrewk sweep <alias> --answer <file> [--json]`

Records the integrator's answer to one maintainer's fermento: what it recommends and the edits it drafts, and writes the answer below the separator.

- `<alias>`: The maintainer, by alias.
- `--answer <file>`: The JSON file with the integrator's answer; its format is below.
- `--json`: Gives the output as one JSON document with a schema field.
- `--help`: Shows this help.

Moves it may cause: `sweep-recorded` (sweep recorded), `proposal-recorded` (proposal recorded), `proposal-withdrawn` (proposal withdrawn).

Documents with --json: `fabrewk.swept/2`

The file's format

```text
The answer file: an object with these fields
  agent: an object with these fields
    tool: text
    provider: text
    model: text
  points: a list, each item an object with these fields
    number: a whole number, 1 or more
    digest: hexadecimal digits
    recommendation: an object with these fields
      point: text
      clarify: text
      destination: text
    edit (optional): an object with these fields
      target: text
      base: hexadecimal digits
      units: a list, each item an object with these fields
        kind: text
        n (optional): a whole number, 1 or more
        m (optional): a whole number, 1 or more
        text: text, or null
      keys: a list, each item an object with these fields
        point: text
        key: text
      result: hexadecimal digits

An edit's unit is the opening (kind opening), a section (kind section, n), a point (kind point, n and m) or a rule (kind rule, n; a text of null removes it); target is alambique or rules, and a rules edit holds rule units only.```

Refusals

- `ALIAS_INVALID`: A name that is not a valid alias: lowercase letters, digits and single hyphens, starting with a letter, at most 32 characters, and not a reserved name.
- `MAINTAINER_UNLISTED`: That alias is no maintainer of the policy in force.
- `FERMENTO_MISSING`: The maintainer has no fermento.md.
- `SEPARATOR_MISSING`: The fermento has no separator line outside fenced code blocks.
- `SEPARATOR_AMBIGUOUS`: The separator's marker stands on more than one line outside fenced code blocks.
- `SEPARATOR_DAMAGED`: The lines around the separator's marker are not a rendering Fabrewk writes, or the file is not valid text.
- `ANSWER_INVALID`: The answer file does not have the shape sweep reads, or one of its texts would break the separator.
- `POINT_CHANGED`: A point of the answer does not exist in the fermento, changed since it was read, or has no recommendation.
- `INPUT_UNCOMMITTED`: The fermento's input differs from the committed one, and the sweep's commit would carry your edit.
- `INPUT_CHANGED`: The fermento's input changed while the sweep ran.
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
- `LOCK_HELD`: Another Fabrewk command holds the lock of this repository's record, or another task is running: one writer and one run at a time.
- `RECORD_HEAD_MOVED`: Something wrote in the record without its lock while this command ran; this command wrote nothing.
- `RECORD_COMMIT_RACE`: The branch moved while Fabrewk committed the record; its events are written but not committed.
- `TOOL_FAILED`: An external tool failed; what it printed is kept as evidence, never shown here.
- `TOOL_MISSING`: A tool this needs is not on this machine.

## `fabrewk discard <proposal> --say <words> [--say-lang <tag>] [--json]`

A maestro discards a pending proposal, saying why.

- `<proposal>`: The proposal, by its identifier (p-…).
- `--say <words>`: Your own words for this decision, kept verbatim and signed. Words copied from an example, such as “I agree.”, are refused.
- `--say-lang <tag>`: The language tag of your words; by default, your languages.agentInteraction.
- `--json`: Gives the output as one JSON document with a schema field.
- `--help`: Shows this help.

Moves it may cause: `proposal-discarded` (proposal discarded).

Documents with --json: `fabrewk.discarded/2`

Refusals

- `AGENT_ENVIRONMENT`: This runs inside an agent or an extension, whose markers are in the environment, and only a person makes this move. The check is cooperative: a process can clear its environment.
- `PROPOSAL_UNKNOWN`: No proposal of the record has that identifier.
- `MOVE_NOT_ALLOWED`: The transition table does not allow this move from the subject's state.
- `ROLE_FORBIDDEN`: The key signing this move holds no role in the policy that may make it.
- `WORDS_MISSING`: A person's move needs your own words, given with --say: not empty, and valid UTF-8 text.
- `WORDS_COPIED`: The words given are the example that help prints, not your own.
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
- `LOCK_HELD`: Another Fabrewk command holds the lock of this repository's record, or another task is running: one writer and one run at a time.
- `RECORD_HEAD_MOVED`: Something wrote in the record without its lock while this command ran; this command wrote nothing.
- `RECORD_COMMIT_RACE`: The branch moved while Fabrewk committed the record; its events are written but not committed.
- `TOOL_FAILED`: An external tool failed; what it printed is kept as evidence, never shown here.
- `TOOL_MISSING`: A tool this needs is not on this machine.

## `fabrewk approve <proposal> --say <words> [--say-lang <tag>] [--json]`

A maestro approves a proposal, which applies its edit and commits the alambique alone, or approves the alambique as it stands, saying why.

- `<proposal>`: A proposal, by its identifier (p-…), or the word alambique.
- `--say <words>`: Your own words for this decision, kept verbatim and signed. Words copied from an example, such as “I agree.”, are refused.
- `--say-lang <tag>`: The language tag of your words; by default, your languages.agentInteraction.
- `--json`: Gives the output as one JSON document with a schema field.
- `--help`: Shows this help.

Moves it may cause: `proposal-incorporated` (proposal incorporated), `direct-edit-approved` (direct edit approved), `alambique-baseline` (alambique's baseline).

Documents with --json: `fabrewk.approved/2`

Refusals

- `AGENT_ENVIRONMENT`: This runs inside an agent or an extension, whose markers are in the environment, and only a person makes this move. The check is cooperative: a process can clear its environment.
- `PROPOSAL_UNKNOWN`: No proposal of the record has that identifier.
- `MOVE_NOT_ALLOWED`: The transition table does not allow this move from the subject's state.
- `ROLE_FORBIDDEN`: The key signing this move holds no role in the policy that may make it.
- `WORDS_MISSING`: A person's move needs your own words, given with --say: not empty, and valid UTF-8 text.
- `WORDS_COPIED`: The words given are the example that help prints, not your own.
- `PROPOSAL_STALE`: The fermento's point is gone or changed, or another proposal changed the project rules since, or the drafted text does not give the result it predicts.
- `ALAMBIQUE_CHANGED`: The alambique, in the working tree, in the index or in the last commit, is not the text the proposal was drafted from.
- `ALAMBIQUE_UNAPPROVED`: The alambique is not the text a maestro last approved: it changed by hand, or it was never approved.
- `ALAMBIQUE_MISSING`: There is no ALAMBIQUE.md, or it is not a regular file reached without a link, or it is not valid text.
- `UNIT_AMBIGUOUS`: A unit of the edit is found twice or not at all in the alambique, or two units overlap, or a rule it removes does not exist.
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
- `LOCK_HELD`: Another Fabrewk command holds the lock of this repository's record, or another task is running: one writer and one run at a time.
- `RECORD_HEAD_MOVED`: Something wrote in the record without its lock while this command ran; this command wrote nothing.
- `RECORD_COMMIT_RACE`: The branch moved while Fabrewk committed the record; its events are written but not committed.
- `TOOL_FAILED`: An external tool failed; what it printed is kept as evidence, never shown here.
- `TOOL_MISSING`: A tool this needs is not on this machine.

## `fabrewk close <stage> --say <words> [--say-lang <tag>] [--json]`

A maestro closes an authorized stage once every task has ended, every fermento was swept after the last of them, the alambique is as last approved and the record is committed.

- `<stage>`: The stage, by its identifier (s-…).
- `--say <words>`: Your own words for this decision, kept verbatim and signed. Words copied from an example, such as “I agree.”, are refused.
- `--say-lang <tag>`: The language tag of your words; by default, your languages.agentInteraction.
- `--json`: Gives the output as one JSON document with a schema field.
- `--help`: Shows this help.

Moves it may cause: `stage-closed` (stage closed).

Documents with --json: `fabrewk.closed/2`

Refusals

- `AGENT_ENVIRONMENT`: This runs inside an agent or an extension, whose markers are in the environment, and only a person makes this move. The check is cooperative: a process can clear its environment.
- `MOVE_NOT_ALLOWED`: The transition table does not allow this move from the subject's state.
- `ROLE_FORBIDDEN`: The key signing this move holds no role in the policy that may make it.
- `WORDS_MISSING`: A person's move needs your own words, given with --say: not empty, and valid UTF-8 text.
- `WORDS_COPIED`: The words given are the example that help prints, not your own.
- `STAGE_OPEN_TASKS`: A task of the stage has not ended: it is neither integrated nor rejected.
- `SWEEP_MISSING`: A maintainer's fermento was not swept after the last task of the stage ended.
- `ALAMBIQUE_UNAPPROVED`: The alambique is not the text a maestro last approved: it changed by hand, or it was never approved.
- `RECORD_UNCOMMITTED`: Some of the record's files are not committed on the record branch.
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
- `LOCK_HELD`: Another Fabrewk command holds the lock of this repository's record, or another task is running: one writer and one run at a time.
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
- `4`: the run stopped and waits for a person: the task is halted.
- `10`: the extension does not support the protocol of this version of Fabrewk.
- `70`: internal error.
