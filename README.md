[Español (México)](README.es-MX.md)

# Fabrewk

Fabrewk is an open-source meta-harness for building software with AI agents. It keeps a project's description
(the alambique), each maintainer's input (the fermentos), its decisions and its agents' work in `.brewstillery/`.
Agents work confined on tasks a person authorized; every person's decision is signed with their SSH key.

The names come from a distillery: Brewstillery is the project's distillery, each maestro prepares a fermento,
and the alambique distils what the project must be. Only maestros approve what enters it.

Version **0.2.0** is a pre-release. It supports **Linux on ARM64** only. Review mode is not built yet: Fabrewk can
show a plan for it, but cannot apply it.

## Install

You need `curl`, OpenSSH's `ssh-keygen`, `sha256sum`, `tar` and `gzip`. Download the script, read it, then run it;
these commands work in fish and in a POSIX shell:

```sh
curl --proto '=https' --tlsv1.2 -fsSLO https://raw.githubusercontent.com/fabrewk/fabrewk/v0.2.0/install.sh
less install.sh
sh install.sh
```

Or pipe it into `sh`:

```sh
curl --proto '=https' --tlsv1.2 -fsSL https://raw.githubusercontent.com/fabrewk/fabrewk/v0.2.0/install.sh | sh
```

The script verifies the signature of `SHA256SUMS` against its allowed signers, then the archive's checksum. It
installs one file, `~/.local/bin/fabrewk`, without `sudo`, and prints how to add its directory to `PATH` if needed.
Inside a Git repository with a terminal, it offers `fabrewk onboard`: questions, a plan to read, then your explicit
yes and your own words before it applies anything.

Trusting the release's source and its signers grants them full authority over what the script installs.

### Through your agent

Copy this prompt into any agent's chat:

```text
Install Fabrewk 0.2.0 and prepare its onboarding plan for my repository. Stop before applying it.

Install the binary outside the repository, in a temporary directory. Download the script, read it in full, and
only then run it; never pipe it into a shell:
    curl --proto '=https' --tlsv1.2 -fsSLO https://raw.githubusercontent.com/fabrewk/fabrewk/v0.2.0/install.sh
    less install.sh
    sh install.sh

Ask me in chat for the language tag (en or es-MX), used for canonical documentation and my agent interactions;
the mode (adopt or review; review cannot be applied yet); publication of .brewstillery/ (published with the product,
own-repository, or unpublished); my alias as the first maestro; the absolute path to my OpenSSH public key, the one
Git signs with; and which tools to declare (claude, codex, or none).

Explain that unpublished keeps the record on this computer: collaborators cannot see it or continue the work,
and maintainers accept that at their own risk and responsibility. Offer publication with the product first,
or a separate repository as a last resort. My decision is recorded in my own words only when I apply the plan.

If .brewstillery/ALAMBIQUE.md is absent, also ask what the project must achieve, for whom, and what it leaves out.
Pass every answer exactly as given, including empty answers; draft nothing and add no ideas of your own.

At the root of my repository, run the following template with my answers substituted and safely quoted for the
shell, preserving their exact values. Repeat --tool once per chosen tool, or omit it for none. Omit --purpose,
--audience and --excluded when an alambique already exists; otherwise pass those answers, even when empty:
    fabrewk assess --mode='<mode>' --publication='<publication>' --alias='<alias>' --key='<public-key-path>' --lang='<language-tag>' --tool='<tool>' --purpose='<purpose>' --audience='<audience>' --excluded='<excluded>'

assess writes nothing. Show me the verdict and the whole plan, including the first alambique if any and the
suggested apply command. Stop there. Never run fabrewk apply or fabrewk onboard. I will run fabrewk apply in my
own terminal with my own words after reading the plan.
```

### Release signers

The release is signed by maestro `xel`. The script's exact allowed-signers line is:

```ssh
xel namespaces="fabrewk-release" ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMpNfw3yY4sb7CO5xUCEfRQuPtw6UlYial7Y/MWCHFVf xel@fabrewk
```

See the [installation manual](docs/en/installation.md#install-the-program) for checking a release by hand.

### Completions and removal

Completion files stay in the archive; the script does not install them. After extracting
`fabrewk-0.2.0-linux-aarch64.tar.gz`, use its `fabrewk-0.2.0-linux-aarch64/completions/` directory:

- Copy `fabrewk.fish` to `~/.config/fish/completions/fabrewk.fish`.
- Copy `fabrewk.sh` to `~/.local/share/bash-completion/completions/fabrewk` for bash; for zsh, source that file
  after `autoload -U bashcompinit; bashcompinit`.

Fabrewk checks for no update by itself and sends no telemetry. Remove the program by deleting
`~/.local/bin/fabrewk`.

## Documentation

- [Installation and onboarding](docs/en/installation.md)
- [Working with Fabrewk](docs/en/working.md)
- [Commands](docs/en/commands.md)
- [Generated command reference](crates/fabrewk/doc/reference.en.md)

## Licence and contributions

Fabrewk's code and documentation are licensed under **MIT OR Apache-2.0**, at your option:
[LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE), [NOTICE](NOTICE). Templates Fabrewk writes into
served repositories are under MIT-0. The licence grants no right to the Fabrewk name or logo.

The release archive carries `THIRD-PARTY-NOTICES` and `THIRD-PARTY-NOTICES-std.html`, covering its crates and the
linked Rust standard library, musl libc and LLVM libunwind.

Outside contributions are not accepted yet. When they open, contributions will come in under the same licence
they go out under; whether to require a DCO sign-off will be decided then.

Report vulnerabilities privately through "Report a vulnerability" on the "Security" tab of
[github.com/fabrewk/fabrewk](https://github.com/fabrewk/fabrewk/security). Do not open a public issue for a
vulnerability.
