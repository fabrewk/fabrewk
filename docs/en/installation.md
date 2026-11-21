# Installing Fabrewk and bringing it into a repository

Fabrewk is one program, `fabrewk`, installed in your home. It enters a repository only through two steps you can
read before anything is written: `fabrewk assess`, which writes nothing and shows a plan, and `fabrewk apply`, which
a maestro runs in their own terminal, with their own words, to apply exactly that plan. `fabrewk onboard` asks the
questions on the terminal and runs both for you.

## What you need

- Linux on ARM64 (`aarch64`). It is the only platform Fabrewk is tested on today; the script refuses any other one
  with `PLATFORM_UNSUPPORTED` and the list of supported platforms.
- For the script: `curl`, `ssh-keygen` (OpenSSH), `sha256sum`, `tar` and `gzip`. If one is missing the script says
  `TOOL_MISSING`, names it and stops; it never installs a tool by itself.
- In the repository: Git, and an SSH key Git signs with (`gpg.format = ssh` and `user.signingkey`). Every decision
  a person makes in Fabrewk is signed with that key.
- To run agents on tasks, later: bubblewrap (`bwrap`) and passt (`pasta`), which confine each agent, and the agent's
  own tool (Claude Code or Codex). `fabrewk config` shows what this machine has.

## Install the program

The script installs one file, `~/.local/bin/fabrewk`, and nothing else. It downloads the version written in it, the
archive's checksum file `SHA256SUMS` and that file's signature `SHA256SUMS.sig`; it verifies the signature with
`ssh-keygen -Y verify` against the signers written in it, then the archive's checksum, and installs nothing if
either fails. It takes no option, so none skips the check, and it never uses `sudo`.

Trusting the release's source and its signers grants them full authority over what the script installs.

Read it first, then run it (this works in fish and in any POSIX shell):

    curl --proto '=https' --tlsv1.2 -fsSLO https://raw.githubusercontent.com/fabrewk/fabrewk/v0.1.0/install.sh
    less install.sh
    sh install.sh

Or pipe it into `sh` directly:

    curl --proto '=https' --tlsv1.2 -fsSL https://raw.githubusercontent.com/fabrewk/fabrewk/v0.1.0/install.sh | sh

The release signers, as the script writes them in OpenSSH's allowed-signers form:

    xel namespaces="fabrewk-release" ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMpNfw3yY4sb7CO5xUCEfRQuPtw6UlYial7Y/MWCHFVf xel@fabrewk

To check a release by hand, save that line as `allowed_signers` beside the three downloaded files and run:

    ssh-keygen -Y verify -f allowed_signers -I xel -n fabrewk-release -s SHA256SUMS.sig < SHA256SUMS
    sha256sum -c SHA256SUMS

If `~/.local/bin` is not on your `PATH`, the script says so and how to add it: in fish, `fish_add_path
~/.local/bin`; in a POSIX shell, `export PATH="$HOME/.local/bin:$PATH"` in `~/.profile`.

The script installs the version it names; a newer one comes only from running a newer script. Fabrewk never checks
for updates by itself and sends no telemetry.

### Completions

The archive holds the completion files in its directory's `completions/`; the script does not install them. To use
them, extract the archive (`tar -xzf fabrewk-0.1.0-linux-aarch64.tar.gz`) and copy, from
`fabrewk-0.1.0-linux-aarch64/completions/`:

- fish: `fabrewk.fish` to `~/.config/fish/completions/fabrewk.fish`;
- bash: `fabrewk.sh` to `~/.local/share/bash-completion/completions/fabrewk`;
- zsh: the same `fabrewk.sh`, loaded from `~/.zshrc` after bash's completion is enabled: `autoload -U bashcompinit;
  bashcompinit; source <the path where you put fabrewk.sh>`.

### Removing it

Delete `~/.local/bin/fabrewk`. Nothing else was installed.

## Bring Fabrewk into a repository

### On the terminal: `fabrewk onboard`

When the script runs inside a Git repository and a terminal is present, it offers to run `fabrewk onboard` there.
You can also run it yourself at the repository's root. It refuses inside an agent (`AGENT_ENVIRONMENT`) and without a
terminal (`TERMINAL_REQUIRED`). It asks, in this order:

1. the language of the interview and of the project's canonical documentation (for example `en` or `es-MX`);
2. the mode: `adopt`, for Fabrewk to administer the repository, or `review`, to only review it (review mode is not
   built yet: its plan is shown, not applied);
3. whether the maintainers publish `.brewstillery/`, Fabrewk's folder in the repository: `published`, with the
   product; `own-repository`, in a Git repository of its own inside that folder; or `unpublished`. In the last two
   the product ignores the folder through the line `/.brewstillery/` in its `.git/info/exclude`. Choosing
   `unpublished` shows a warning with its consequences and its exits, and asks again;
4. the first maestro's alias (lowercase letters, digits and single hyphens);
5. the path of the maestro's OpenSSH public key, the one Git signs with; if Git does not sign with it yet, the plan
   shows the Git commands to set that up, and runs none of them;
6. the tools to declare, among the built-in ones (`claude`, `codex`), or none;
7. when the repository has no `.brewstillery/ALAMBIQUE.md`: what the project must achieve, for whom, and what it
   leaves out. Each answer is written as you give it; an empty one leaves a line saying it is not answered yet.

Then it shows the verdict and the whole plan: every file it would create, with its exact content; the block it
would add to an existing entry file; the line it would add to `.git/info/exclude`; and the views it would generate
from the record (`STATUS.md`, `RULES.md`), by name. Only
the catalogue's yes (`yes` in English, `sí` in Spanish) followed by your own words applies it; any other answer, an
empty one, or the end of input applies nothing and says so. Your words are recorded as you write them, and signed.

### Through an agent: `fabrewk assess`, then `fabrewk apply`

An agent can prepare the same plan, but only a person applies it. The agent asks you the same questions in chat and
runs `fabrewk assess` with your answers as options, for example:

    fabrewk assess --mode adopt --publication published --alias ana --key ~/.ssh/id_ed25519.pub --tool claude --lang en --purpose 'What the project must achieve' --audience 'For whom' --excluded 'What it leaves out'

`assess` writes nothing, not even a cache, and uses no network. It prints the verdict, the plan, the plan's digest,
and the exact `apply` command, quoted for fish and for a POSIX shell. You read the plan and run that command in your
own terminal, replacing `<your words>` with yours:

    fabrewk apply <digest> --mode='adopt' --publication='published' … --say='<your words>'

`apply` refuses inside an agent (`AGENT_ENVIRONMENT`). This check is cooperative: a process can clear its
environment. What binds the act to you is the signature with your key; a key that asks for a touch closes the gap
an agent of your own user could otherwise use.

### The verdict

- `adopt`: Fabrewk can administer the repository; the plan can be applied.
- `review`: the plan only reviews the repository. Review mode is not built yet, so `apply` refuses it
  (`MODE_NOT_AVAILABLE`).
- `none`: something blocks it, for example an `AGENTS.md` or `CLAUDE.md` with changes not committed; each reason is
  listed. `apply` refuses it (`VERDICT_NONE`).

The reasons also report what other tools left that would change what agents may do: hooks in `.claude/` or
`.codex/` settings, automatic updates, telemetry, syncs that commit or push, generated files that pre-approve tools.
Fabrewk changes none of them.

### What `apply` does

It assesses again and refuses, writing nothing (`PLAN_CHANGED`), if the plan it gives now is not the one named, or
if a file the plan writes changed since. Then it writes the plan's files, never overwriting a file it did not write:
in an existing `AGENTS.md` or `CLAUDE.md` it adds a delimited block that points agents to `.brewstillery/STATUS.md`;
a `CLAUDE.md` it creates holds the single line `@AGENTS.md`. It records, signed with your key, the assessment, the
publication decision with your words, the adopted settings and the alambique's baseline; then, as Fabrewk's own
event, unsigned, the installation's receipt, with the digest of every file it wrote. In `published` it commits all of
it, the entry files included, keeping every other change you had; in `own-repository` the folder's own repository
commits the folder's files, and the entry files are left uncommitted in the product; in `unpublished` nothing is
committed. It names what it left uncommitted and the Git command that commits it, and runs none.

An `apply` that stops part-way (a crash, a full disk) leaves the repository marked `INSTALLATION_INCOMPLETE`: no
other command writes until the same `apply`, run again with the same digest and options, finishes it, writing only
what is missing.

### Publication of `.brewstillery/`

Fabrewk recommends keeping `.brewstillery/` in Git, published with the product, so that every clone carries the
project's description, its decisions and its record. Choosing not to publish it means the project's history of
decisions does not travel with the code; the warning `onboard` shows states the consequences and the exits. Your
answer is recorded in your own words.

### After the installation

- `fabrewk check` verifies the settings, the record and the domain.
- `fabrewk status` shows the state of the work.
- The project file declares no model and no profile yet: a maestro adds them to `.brewstillery/settings.toml` and
  adopts it again with `fabrewk adopt --say '<words>'`. See [Working with Fabrewk](working.md).
