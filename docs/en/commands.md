# The commands

Every command, with what it does. The [command reference](../../crates/fabrewk/doc/reference.en.md) is generated
from the program's own declaration of its commands, so it lists exactly the commands that exist, each with its
options, the record's moves it may cause, its JSON documents and its refusals; `fabrewk <command> --help` prints its
summary, its options, its file's format where it takes one, and its refusals. Commands and options are in English in every language.

## Reading, writing nothing

| Command | What it does |
| --- | --- |
| `fabrewk status [<id>]` | The record's health, the policy in force and the state of each stage and task; with an identifier, that stage or task with its events and the digest a decision on it cites. |
| `fabrewk config [<key>]` | Every effective setting with its source, the changes not adopted, the maintainers' keys, what this machine has and every control. |
| `fabrewk check` | Verifies the settings, the policy, the record, the domain, the credentials and the private folder; exits 3 on an error, 0 on warnings alone. |
| `fabrewk assess [<options>]` | Assesses the repository, writing nothing: the verdict, its reasons and the plan, with its digest and the `apply` command. |

## The installation

| Command | What it does |
| --- | --- |
| `fabrewk onboard` | Asks the installation's questions on the terminal, shows the plan, and applies it on your yes and your words. |
| `fabrewk apply <plan> <options> --say <words>` | A maestro applies the plan `assess` printed, with the same options. |

## A maestro's decisions

Each takes `--say '<words>'`, your own words, recorded as written and signed with your key, and optionally
`--say-lang <tag>`, their language; each refuses inside an agent.

| Command | What it does |
| --- | --- |
| `fabrewk adopt --say <words> [--widen <key>]…` | Adopts the project file as the policy in force. |
| `fabrewk authorize <id> --seen <digest> --say <words> [--widen <key>]… [--run]` | Authorizes a task, or a stage and its tasks, as `status` showed it; with `--run`, runs it at once. |
| `fabrewk run <id> --say <words>` | Runs an authorized task, or a stage's next one, confined, round after round, to a verified task or a halt; finishes the integration of an accepted task. |
| `fabrewk accept <task> --seen <digest> --say <words>` | Accepts a task's candidate as `status` showed it and integrates it. |
| `fabrewk reject <task> --say <words>` | Rejects a task, saying why. |
| `fabrewk approve <proposal> --say <words>` | Approves a proposal, applying its edit to the alambique; `approve alambique` approves the alambique as it stands. |
| `fabrewk discard <proposal> --say <words>` | Discards a pending proposal, saying why. |
| `fabrewk close <stage> --say <words>` | Closes a stage whose work has ended. |

## Recording, granting nothing

| Command | What it does |
| --- | --- |
| `fabrewk propose <file>` | Records a stage and its tasks, or tasks for a stage, from a JSON file. |
| `fabrewk sweep <alias> --answer <file>` | Records the integrator's answer to a maintainer's fermento and writes it below the separator. |

## The program alone, and extensions

- `fabrewk --help` and `fabrewk --version` (with `--json`, the version as a document).
- `fabrewk <name>` runs the extension `fabrewk-<name>` when the settings permit it; see
  [Working with Fabrewk](working.md#extensions).

## Output and exit statuses

Every built-in command but `onboard` takes `--json`, which prints one JSON document with a `schema` field; a refusal
with `--json` is a document too, and `--help` stays text. An extension's arguments are its own. A refusal prints its code first, for example `PLAN_CHANGED: …`, with the reason and the next step.

| Status | Meaning |
| --- | --- |
| `0` | Done. |
| `1` | Refused: a code was printed, with its reason and next step. A refusal before the command's events are written leaves nothing written; one after says what it left and how it is finished. |
| `2` | Usage error: the command line was not accepted. |
| `3` | The check found a problem. |
| `4` | The run stopped and waits for a person: the task is halted. |
| `10` | The extension does not support the protocol of this version of Fabrewk. |
| `70` | Internal error. |

## Completions

The release archive carries completion files for fish and for bash (zsh with `bashcompinit`), generated from the
same declaration; [Installing Fabrewk](installation.md#completions) says where to put them.
