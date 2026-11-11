# Phase 9 opening: threat model, confinement design and the cut into parts

Phase 9's opening deliverable (plan row 9, D-071), written by the integrator in session 29 before any code. It holds
the threat model of dispatching agents, the design of the confinement backend, what that design asks the frozen
specification to change, and the cut of phase 9 into parts. It decides nothing by itself: section 5 lists what the
maestro decides, and each decision is recorded in `decisions.md`.

**How to read it.** "Agent" is a dispatched tool (Claude Code, Codex) running a model; "the integrator" is the agent
a person runs in their own terminal, unconfined. "Cell" is what one confined process may reach. A control is
`effective` or `cooperative` as REQ-SEC-5 defines them; a **residual risk** is a threat no control of the first core
closes, which `config` must declare (6.38: no written instruction presented as a demonstrated protection).

**State (session 29).** The maestro decided section 5 as recommended (D-075). Codex's first pass confirmed six
blocking findings (P9O-1 to P9O-6), corrected here: T-02, T-28, 3.3, 3.4, 3.8 and S-4, S-5, S-8 to S-11. The review
is recorded in R-038.

## 0. Inputs and what was probed

Read at `ae80923`: `STATUS.md`, `plan.md`, `ALAMBIQUE.md`, `xel/fermento.md`, both settings files;
`core-specification.md` (TASK, RUN, GATE, SEC, EXT, BUD, DOM-1 to DOM-5, REG-5, REG-13, SET-8, SET-9, sections 3 to 8,
11.2 to 11.6, 12 RUN to BUD, 13); `architecture-decisions.md` AD-5, AD-6, AD-8; `decisions.md` D-032, D-067 to D-071,
D-074, R-037; `past-defect-cases.md` PD-20 to PD-30; `harness-verdicts.md` (the observations); the workspace's
manifests, `main.rs`, `lib.rs`, `detect.rs`'s contract and `tools/budget/src/budget.rs`. No network beyond the
probes below; no model called; no file of the private folder or of a tool's credentials read.

Probed on this machine (Fedora Asahi 45, kernel 7.2.12 aarch64 with 16k pages, SELinux enforcing, 7.5 GB of RAM, 8
cores; bubblewrap 0.13.1, not setuid; `pasta` 2026-10-02 from `passt`; Claude Code 2.1.316; `codex-cli` 0.166.0; no
Docker or Podman), in the session's scratch directory:

- **P-1.** `bwrap` runs unprivileged with new user, mount, PID, IPC, UTS, cgroup and network namespaces. Inside: no
  capability, `NoNewPrivs` set, no seccomp filter, only its own processes, only a loopback interface.
- **P-2.** Inside, `/proc/1/cmdline` shows `bwrap`'s whole command line and `/proc/self/mountinfo` the host path of
  every bound directory: names are visible, contents are not.
- **P-3.** The host holds abstract sockets (`@/tmp/.X11-unix/X0`, `X1`) and loopback listeners (`sshd`, three
  ephemeral ports on `127.0.0.1`, the stub resolver). A process that shares the host's network namespace reaches all
  of them; the X server admits a local process of the same user.
- **P-4.** Under `pasta` the abstract sockets are gone. By default it still maps the host's loopback and gateway
  address (port 22 answered). With `-t none -u none -T none -U none --no-map-gw` the host's loopback and own address
  are closed; the local network's gateway and the Internet answer.
- **P-5.** The host resolves names through a loopback stub. With `--dns-forward 169.254.0.53 --dns-host 127.0.0.53`
  and a `resolv.conf` naming that address, names resolve inside `bwrap` inside `pasta`; the address answers nothing
  else. TLS was not exercised (no certificate store was bound).
- **P-6.** `bwrap` nests inside `bwrap` when the outer one has a `/tmp`; `--disable-userns` (with `--unshare-user`)
  refuses the inner one.
- **P-7.** The exit status passes through `pasta` and `bwrap` (7 as 7, a kill as 137).
- **P-8.** Killing `pasta` leaves `bwrap` and everything it confines running. Killing the outer `bwrap` ends every
  process, one in its own session that ignores signals included.
- **P-9.** A guard namespace placed around `pasta` (`bwrap` or `unshare`, three variants) makes the inner `bwrap` or
  `pasta` fail to map its user. Not solved here; see 3.7.
- **P-10.** Claude Code is one dynamically linked executable, `~/.local/share/claude/versions/<version>`; its
  credential is `~/.claude/.credentials.json` (`0600`). Codex is a release directory under
  `~/.codex/packages/standalone/releases/<version>/` with three executables in `bin/`, statically linked; its
  credential is `~/.codex/auth.json` (`0600`). Both print, in their help, flags for a headless run with a result
  schema and without user configuration (`claude --print --output-format stream-json --json-schema --bare --settings
  --setting-sources --strict-mcp-config --permission-mode --tools`; `codex exec --json --output-schema --sandbox
  --ephemeral --ignore-user-config --ignore-rules --skip-git-repo-check`, and `codex sandbox`). What each flag does
  was not verified.

- **P-11 (after P9O-1).** A key planted in the session's kernel keyring is found and read inside a `bwrap` cell with
  every namespace new. With `keyctl session -` run first, inside or outside the cell, it is not found, and neither is
  the user keyring's.
- **P-12 (after P9O-2).** In a repository whose configuration sets `core.hooksPath` to a tracked directory, `git
  update-ref` runs that directory's `reference-transaction`; with `-c core.hooksPath=<an empty directory>` it runs
  nothing.

The workspace forbids `unsafe` code and has four runtime crates, so the backend can call no system call of its own
(Landlock, seccomp, `openat`, `kill`, `statvfs`): it drives programs.

## 1. What is protected, from whom

**Assets.**

1. The person's authority: the key they sign with (reached through the SSH agent's socket), the record, the policy,
   the alambique and the fermentos.
2. Credentials: the private folder, the home (`~/.ssh`, cloud and tool logins, browser profiles), the environment's
   variables, the operating system's key store.
3. The repository: `.git` (its configuration and hooks run as the person), the working tree outside a task's allowed
   files, other repositories on the machine.
4. The host: its processes, its desktop session (display, clipboard, session bus), its local services and its network.
5. The code's confidentiality: it is sent only to a provider the policy allows.
6. The evidence: that a check ran on the candidate, that a reviewer examined it, that an attempt ended as recorded.
7. Availability: disk, memory, time and the budget of dispatches.

**Actors.**

| Actor | Runs | Trusted for |
| --- | --- | --- |
| The person (a maestro) | Their terminal; signs | Everything their role allows |
| The integrator | Unconfined, as the person, in their terminal | Nothing by code: every control on it is cooperative (AD-14's stated gap); a person's move needs their signature and words |
| A building agent | Confined | Nothing. It is treated as hostile: it may be mistaken, lazy, or steered by text in the task, the tree or the network |
| A reviewing agent | Confined | Its verdict, only when the audit shows it examined the candidate, and only as one of three (checks, review, the person's acceptance) |
| A check | Confined; runs code the builder wrote | Nothing but its exit status on the tree Fabrewk exported |
| An extension | Confined; permitted by the policy, pinned by digest | Its requests, validated by the outer core |
| A provider | Remote | Receiving the copy (`providers.<name>.allowed`); nothing else |
| Another process of the same user | Unconfined | Out of scope: it can already write the domain (REQ-SEC-5's definition of cooperative) |
| The kernel, `bwrap`, `pasta`, `git`, `ssh-keygen` | The host | The trusted base; their defects are residual |

**Boundaries.** B1, the host and the cell (namespaces). B2, what a cell leaves behind (files, a stream, an exit
status) and the unconfined code that reads it. B3, a candidate and the record branch (lanes, protected paths, checks,
review, acceptance). B4, the cell and the network. B5, an extension and the core. B6, the person and every agent
(words and signature).

**Out of scope for the first core.** A defect of the kernel or of the trusted programs; side channels; another user
of the machine with root; macOS and Windows (no backend, so dispatch refuses there, REQ-SEC-2).

## 2. Threats and controls

| # | Threat | Control | Label | Shown by |
| --- | --- | --- | --- | --- |
| T-01 | An agent or a check reads the repository, `.git`, the domain, another repository, the home or the private folder | Only what the cell lists exists inside; the copy holds no `.git` and no `.brewstillery` at any depth | effective | REQ-SEC-1's probes, PD-22 |
| T-02 | It reaches the person's signature or session: the SSH agent's socket, the session bus, the key store, the display's socket, the kernel keyring its process inherits | No `/run`, no host `/tmp`, no host home; the environment is built from empty; a new session keyring before the cell's command starts (3.3; P-11) | effective | Probes for each socket, for a planted variable and for a key planted in the session keyring |
| T-03 | Through a shared network namespace it reaches the host's abstract sockets (the X server: keys read and typed into the person's terminal) and loopback services (the editor's, `sshd`) | A private network namespace with outbound connections only (section 3.5) | effective with option B; residual with option A | Probes: no abstract socket, the host's loopback closed |
| T-04 | It reaches the local network and the Internet: sends the copy anywhere, attacks other machines | None in the first core | residual | Declared by `config` |
| T-05 | It reads its tool's credential and sends it out | The credential bound is the tool's only; the tool's own sandbox keeps the model's commands from the network (3.6) | cooperative (the tool's control); residual | Declared; the preflight shows the tool's sandbox starts |
| T-06 | It writes outside its copy, or leaves something that outlives the attempt | Only the attempt's directories are writable; they are deleted when it ends; everything else is read-only or absent | effective | Probes; nothing of the attempt remains |
| T-07 | It gains privileges or touches host processes: setuid programs, `ptrace`, signals, devices | No new privileges, a PID namespace, a minimal `/dev` | effective | Probes |
| T-08 | It exploits the kernel through what a user namespace opens, or through system calls no filter refuses | Nested namespaces only where a tool's own sandbox needs them; no seccomp filter (a later hardening) | residual | Declared |
| T-09 | It outlives its time limit or its end | Ending Fabrewk's direct child ends every process of the cell; it dies with Fabrewk (3.7) | effective | A process in its own session that ignores signals is gone after the limit, and after Fabrewk is killed |
| T-10 | It exhausts memory, processors or disk | The time limit; a cap on the stream kept; free space checked before each copy (S-3). None for memory | residual for memory and for disk during a run | Declared; R-022's lesson |
| T-11 | The tree it leaves attacks what reads it: links out of the copy, special files, a nested `.git`, `.gitattributes` that convert or filter, `.gitignore` edited to hide files, names that are not text | Section 3.8's rules: read after the cell is dead, nothing followed, nothing in the tree read as configuration | effective | PD-16, PD-18 to PD-21 and S-2's new cases |
| T-12 | The stream it leaves attacks what reads it: without end, malformed, with control characters | A cap; one strict grammar per tool, any anomaly voids; raw output only in evidence (REQ-REG-13); escapes on every human output | effective | PD-26, PD-28, PD-29 |
| T-13 | The model's own commands forge events in the stream the audit reads | The tool's own sandbox | cooperative | Declared: the audit detects a lazy or failed review, not a hostile one (PD-26's stated limit) |
| T-14 | Text an agent wrote (a finding, a summary) steers a person or the integrator when shown | Shown verbatim, labelled, escaped and bounded; a person's move needs a signature and words | effective for the move; cooperative for the integrator | PD-37, PD-30 to PD-32 |
| T-15 | A candidate changes files outside its lane, protected names, or what defines its checks | The lane guard, the change-set guard and the integration guard; protected paths only when listed literally | effective | PD-15, PD-16, `PROTECTED_PATH_WILDCARD` |
| T-16 | A candidate plants instructions or automatic configuration for the next agent, the integrator or the person's Git, at the root or in any directory (`src/AGENTS.md`, `src/.claude/`, a nested `.gitattributes`, a hook in a tracked hooks directory) | Those names protected at any depth and the hooks directory protected, by default (S-4): changed only when a maestro listed the path literally; `accept` shows every file | effective as REQ-GATE-9 is | S-4's tests, `src/**` with a nested instruction file among them |
| T-17 | A candidate makes its checks pass without proving anything (it edits the tests, or ends them early) | Protected paths, lanes, the review, the person's acceptance | residual: a check proves what its protected files make it prove | Declared |
| T-18 | A candidate's text steers its reviewer | A reviewer of another provider preferred (REQ-GATE-4); checks independent of both; the person's acceptance | residual | Declared |
| T-19 | A check, which runs the builder's code, attacks the host or sends the copy out | Confined as an agent, without network or credential, on a fresh copy never read back | effective | REQ-GATE-2's probe |
| T-20 | A reviewer changes the candidate | Its copy is never read back | effective | PD-27's statement, a test |
| T-21 | An extension writes the record or reaches a key | Confined: the repository read-only, one writable area; its requests validated by the outer core, never signed as a person | effective on Linux | REQ-EXT-3, REQ-EXT-4 |
| T-22 | The tool obeys what it finds: user settings, hooks, MCP servers, skills, the tree's instruction files | An empty home (effective); each adapter turns off the tool's discovery as far as the tool allows; the packet is the only instruction (REQ-RUN-8) | effective for the home; cooperative for the tool's flags | PD-22's markers; REQ-SEC-6 |
| T-23 | Fabrewk's own launch leaks: a secret in a command line, the person's environment inherited | No secret in any argument; the environment built from empty; credentials are files | effective | A probe for a planted variable; a test on the arguments |
| T-24 | Host paths are visible inside (the person's name, where the repository is) | None | residual, names only | Declared |
| T-25 | An unconfined process of the same user races a write (P8C-8) or edits the domain | None by code | cooperative | Declared (M-3) |
| T-26 | No backend, a broken one, or a tool's new version that does not fit it | Dispatch refuses (`CONFINEMENT_UNAVAILABLE`); a preflight per tool and version | effective | REQ-SEC-2, PD-27 |
| T-28 | After integration, Fabrewk's own Git commands run or obey what the candidate brought: a hook in a tracked hooks directory run by the record commit's `update-ref` (P-12), a filter or conversion chosen by the candidate's `.gitattributes` when the working tree is written | Every Git command Fabrewk runs names an empty hooks directory and no file-system monitor; the working tree is written from the objects' bytes, with no filter or conversion (3.8 rule 9; S-11) | effective | P-12 as a test through `accept`; a candidate's `.gitattributes` with a configured filter leaves the bytes |
| T-27 | Fabrewk is interrupted during a dispatch | The cell dies with it; the next write command ends the attempt as unknown (REQ-RUN-5) | effective | The crash suite, PD-08 |

## 3. The confinement design

### 3.1 Principles

1. **Nothing exists unless listed.** A cell starts empty: no file, variable, socket or network that its description
   does not name. Nothing is inherited from the person's session.
2. **One description, one backend.** The core describes a cell and names no mechanism; the backend turns the
   description into the operating system's. Agents, checks, the preflight and extensions are four descriptions, not
   four mechanisms.
3. **What comes back is data.** No program runs outside a cell on what a cell wrote, except those that read it as
   bytes. Nothing in an agent's tree is configuration for Git or for Fabrewk, in the copy and, for Fabrewk's own
   commands, after integration too: once a person accepted a candidate its files are theirs, and what their own
   tools then run is what they accepted.
4. **Dead before read.** Fabrewk reads what a cell left only when every process of the cell has ended.
5. **Refuse, never fall back.** A backend that is absent or does not pass its own probe, a program it needs that is
   missing, a tool whose preflight fails: each refuses the dispatch. No role runs with less confinement than its
   description.
6. **Two layers, two owners.** Fabrewk's cell protects the host from the tool. The tool's own sandbox protects the
   tool's credential and network from the model's commands. The first is Fabrewk's and tested; the second is the
   tool's, started and checked by the preflight, and declared as the tool's.

### 3.2 The contract the core declares

The core gains a module that holds only data and one runner; `main.rs` wires the backend of the target system, as it
wires `MODULES` today (D-051). Sketch, to be fixed with the code:

```rust
pub enum Access { ReadOnly, ReadWrite }
pub struct Mount { pub host: PathBuf, pub inside: PathBuf, pub access: Access }
pub enum Network { None, Outbound }

/// Everything a confined process may reach; nothing else exists inside.
pub struct Cell {
    pub mounts: Vec<Mount>,                   // beyond the system directories the backend shows read-only
    pub network: Network,
    pub nested: bool,                         // whether it may make namespaces of its own (a tool's sandbox)
    pub env: Vec<(OsString, OsString)>,       // the whole environment
    pub cwd: PathBuf,                         // inside
    pub argv: Vec<OsString>,                  // inside
}

/// A confinement backend: the programs it drives, how it turns a cell into a command, and what it cannot close.
pub struct Backend {
    pub name: &'static str,
    pub programs: &'static [Program],         // detected like every program (REQ-SET-8)
    pub command: fn(&Cell, &Detected) -> Result<Command, Error>,
    pub residual: &'static [&'static str],    // catalogue keys `config` lists
}
```

The core's runner starts the command, keeps its two output channels in `local/evidence/` (REQ-REG-13) up to a cap,
ends it at its time limit, and returns the exit status and the evidence digests. The backend module is
`crates/confine-bwrap` (ring `Confinement`, 400 lines); a second backend (Docker, D-032 point 14) fulfils the same
contract and is not built in phase 9.

### 3.3 What every cell holds (Linux, bubblewrap)

- New user, mount, PID, IPC, UTS and cgroup namespaces; a new session, so no terminal is shared; no new privileges; a
  fixed host name. A new session keyring, joined before the cell's command starts: namespaces do not detach the one a
  process inherits (P-11). The standard library cannot join one, so a program does (`keyctl session` of `keyutils`
  here, one more program the backend detects and without which it refuses); part 9a fixes it with the planted-key
  probe. The user's identifier inside is the person's own, so Fabrewk can read and delete what the cell wrote.
- `/usr` read-only, and `/bin`, `/sbin`, `/lib`, `/lib64` as the host has them (links recreated, directories bound
  read-only). A new `/proc` and a minimal `/dev`.
- Of `/etc`, a list: the certificate stores, the dynamic loader's cache and configuration, the alternatives. A
  `passwd`, `group`, `hosts`, `nsswitch.conf` and `resolv.conf` written by Fabrewk into the attempt's directory and
  bound from there. The list is fixed in part 9a by the probes; a tool or a check that needs more fails visibly.
- A home (`/home/agent`), a `/tmp` and, where the role has one, `/work`: three directories of the attempt, under
  `local/tmp/<attempt>/`, on disk (AD-8: no machine-wide temporary directory; on this machine `/tmp` is memory). They
  are the only places an agent or a check can write, and all three are deleted when the attempt ends (S-10 says so
  in REQ-SEC-1's criterion, which names `/work` alone). An extension's home and `/tmp` are directories inside its own
  area, `local/ext/<name>/`, so it writes nowhere else (REQ-EXT-4, AD-8).
- Nothing else: no `/run`, `/var`, `/sys`, `/home/<person>`, `/root`, `/opt`, `/mnt` or `/media`.
- The environment the cell lists and no other: `PATH`, `HOME`, `TMPDIR`, a language, and what the adapter adds.
  `bwrap` receives it by inheritance from a cleared command, so no value appears in an argument.

### 3.4 The cells, by role

| Role | `/work` | Read-only besides the system | Credential | Network | Nested namespaces |
| --- | --- | --- | --- | --- | --- |
| Building agent | The attempt's copy of the base, writable | The tool's release, as its adapter derives it from the executable found (REQ-SEC-1, PD-27) | The tool's own, where the tool looks for it | Outbound | As the adapter declares for the tool's own sandbox |
| Reviewing agent | A fresh copy of the candidate, writable, never read back | The same | The same | Outbound | The same |
| Check | A fresh copy of the candidate, writable, never read back | — | None | None; outbound only when the check declares it and the policy permits it (REQ-GATE-2) | No |
| Preflight | An empty directory with the role's permissions | The tool's release | None | None | As the role's |
| Extension | The repository read-only at its own path; `local/` hidden except `local/ext/<name>/`, writable (its home and `/tmp` inside it), and what a reading command reads (`local/cache/`), read-only | The `fabrewk` executable; the extension's program, as found, at its own path (S-9) | None | None, unless `extensions.<name>.network` | No |

A required tool (`profiles.<name>.requiredTools`, `checks.<name>.tools`) is present for a dispatch only when the
cell shows it: its executable, as it resolves, lies under the system directories. A tool that lives in the person's
home is absent inside and refused (`TOOL_MISSING`), saying why (S-6). The pilot's module is Python, whose tools are
system packages; a machine key that declares a tool's directory is proposed when a project needs it. The same holds
for an extension: its program is bound, and an interpreter or a support file it needs must lie under the system
directories or in the repository; otherwise it fails to start, and Fabrewk says why (S-9).

### 3.5 The network

REQ-SEC-1 gives an agent "network only for its provider" and states "the shared network" as a residual risk, as the
harness did. P-3 shows what sharing the host's network namespace costs on a desktop: the X server's abstract socket
and the editor's loopback ports are reachable, and with them the keys the person types and the terminal where they
sign. That is a way around B6, the boundary every other control rests on.

| Option | Closes | Leaves | Cost |
| --- | --- | --- | --- |
| A. Share the host's network (the specification as written) | Nothing | T-03, T-04, T-05 | None |
| **B. A private network namespace with outbound connections only** (`pasta`, hardened as in P-4 and P-5) | T-03: the host's abstract sockets, loopback and own address | T-04, T-05: the local network and the Internet are reachable | One more program the backend drives and detects; dispatch refuses where it is absent; name resolution written per attempt; the composition of 3.7 |
| C. No network; outbound connections only through a proxy of Fabrewk's that admits the provider's hosts | T-03, most of T-04 and T-05 | Sending to another account of the same provider | A proxy that parses what the agent sends, in the unconfined process; a bridge inside the cell; each tool must honour a proxy; about 300 to 400 lines |

**Recommendation: B now; C designed after the pilot.** B closes the one path from an agent to the person's
authority at a small cost, with a program the distribution already ships. C makes REQ-SEC-1's sentence true and
belongs with the pilot's measurements of what the tools need. With B, `config` declares T-04 and T-05.

### 3.6 Credentials, and the tool's own sandbox

- Each adapter declares its tool's credential: the file, where the tool looks for it inside the cell's home, and
  whether the tool must rewrite it. Fabrewk copies no session (6.51): it binds the file.
- **Open, for parts 9c and 9d:** whether each tool rewrites its credential when it refreshes it, and what a
  read-only bind then does to the person's own login. It is tested before the first real dispatch. If a tool cannot
  work with a bound credential, the alternatives (a writable bind; a token of the tool's made for unattended use, kept
  by the person) go to the maestro with what the test showed.
- An agent can read the credential its tool uses and, with an open network, send it out (T-05). The control is the
  tool's own sandbox: the adapter starts the tool in the mode where the model's commands run without network and
  inside the copy, and never with the tool's sandbox off. Approvals cannot be asked in a headless run, so the mode
  that needs none is chosen per role; a mode the tool's version does not support refuses (REQ-SET-9). What each flag
  really does is verified with the real tool in 9c and 9d, and what cannot be verified is declared.
- The home is empty, so the tool finds no user settings, hooks, MCP servers, plugins or skills (T-22, REQ-SEC-6). The
  adapter also turns off the tool's discovery of instruction files in the copy, as far as the tool allows: the
  product's `AGENTS.md` points at a `STATUS.md` the copy does not hold, and the packet is the only instruction.

### 3.7 Ending a cell, and its limits

- **Requirement.** Ending Fabrewk's direct child ends every process of the cell, and the cell dies when Fabrewk
  does. The standard library can end only a direct child, and P-8 shows that with `pasta` as that child the cell
  survives.
- **Open, for part 9a.** How `bwrap` and `pasta` compose so that the requirement holds. Candidates, in order: (a)
  `bwrap` is the direct child and makes the network namespace; Fabrewk attaches `pasta` to it, and the tool starts
  only once the network is up; (b) a guard namespace around `pasta` (P-9: three variants failed); (c) `pasta`
  outermost and `bwrap` ended by its process identifier through a program, rejected unless (a) and (b) fail, for the
  race on the identifier. Part 9a chooses by test. If none holds, the choice between option A declared and refusing
  the dispatch returns to the maestro.
- **Limits.** Time: the profile's or the check's minutes. Output: a cap on each channel kept; past it the cell is
  ended and the attempt fails. Disk: free space checked before each copy (S-3); what a cell writes during its run is
  not bounded. Memory, processors and the number of processes: not bounded in the first core. The machine's lessons
  applied as limits at launch are projected (6.51, 6.38); until then this is declared, and R-022 is the incident.

### 3.8 What comes back, and how it is read

1. Nothing is read from a cell's directories until its direct child has been waited for.
2. Fabrewk walks the copy itself and follows nothing: a directory is entered, a regular file is read, a link's target
   is taken as written. Anything else (a pipe, a socket, a device) is never opened and, outside the base's ignored
   paths, is refused as a change no candidate can hold.
3. An entry named `.git` or `.brewstillery`, of any kind and at any depth, under ASCII case folding, is refused
   before any Git command sees the copy (`PROTECTED_PATH`), so no nested repository becomes a submodule entry. A name
   Git's protections for other file systems refuse is refused too.
4. Contents are hashed without filters or conversions and the temporary index is built from those identifiers.
   No command that reads the copy's `.gitattributes`, `.gitignore` or `.gitmodules` runs on it.
5. The base's ignore rules are the base tree's own files, not the copy's: an agent that edits `.gitignore` changes a
   file, not what Fabrewk sees.
6. A changed path that is not valid text cannot match a pattern and is refused (`LANE_VIOLATION`).
7. A reviewer's and a check's copies are deleted unread. Only an exit status, the kept stream and, for an agent, the
   result its role's schema admits come back from them.
8. The stream is read by its adapter's one grammar; the result is validated against the role's schema by the core
   (REQ-RUN-8); texts are bounded in length, recorded verbatim and escaped wherever shown.
9. After integration: Fabrewk writes a touched path of the working tree from the object's bytes, with no filter,
   conversion or checkout command; every Git command Fabrewk runs, in every command and phase, names an empty hooks
   directory and turns the file-system monitor off, so that no file a candidate brought runs as a hook of Fabrewk's
   own ref updates (P-12; S-11).

### 3.9 Preflight and detection

`bwrap`, `pasta` and the program that joins a session keyring are the backend's programs, detected at every start
like `git` (REQ-SET-8). The backend has a
probe of its own, run before a dispatch when its programs' versions are not yet verified by use: an empty cell that
reads a planted file it must not see, and must fail to. Each adapter's preflight (REQ-SEC-1, PD-27) runs, under each
role's cell without network or credential, the tool's version, its helper programs and its own sandbox creating a
temporary file. A pass is kept as "verified by use" in `local/cache/detected.json` for that path and version, which
is what that cache already carries; a failure refuses (`CONFINEMENT_UNAVAILABLE`) and names its evidence file. Each
adapter's machine rules are checked at every launch: the launch directory is the attempt's, never one that holds
other repositories, and the copy holds no repository.

### 3.10 What `config` declares

Beside the controls and their labels (REQ-SEC-5), `config` lists the residual risks of the backend and of each
adapter, worded in the catalogue: the open outbound network (T-04); the tool's credential readable by its agent
(T-05); the kernel and the trusted programs (T-08); no limit on memory, processors or disk during a run (T-10); the
audit's limit (T-13); what a check proves (T-17); a reviewer steered by the candidate (T-18); the tool's own flags
(T-22); host paths visible by name (T-24); an unconfined process of the same user (T-25).

## 4. What this design asks the specification to change

Each is a change to a frozen requirement, so a maestro's recorded decision (6.25). The work each affects is phase 9's
own; nothing built in phases 7 and 8 changes but where named.

- **S-1 (with M-1, option B).** REQ-SEC-1: an agent's network is a private namespace with outbound connections only;
  nothing of the host's loopback, addresses or abstract sockets is reachable from a cell; the open outbound network
  is the stated residual risk. Its criterion gains the two probes. `pasta` joins `bwrap` among the programs detected.
- **S-2.** REQ-RUN-3 gains section 3.8's rules 1 to 6, and its tests: a nested `.git` directory and a `.git` file; a
  `.gitattributes` that asks for a conversion (the candidate keeps the bytes); a `.gitignore` edited to hide a new
  file (the file is in the change set); a pipe and a socket left in the copy; a path that is not valid text.
- **S-3 (D-067).** A new requirement, REQ-RUN-10: before each copy (an attempt's, a review's, a check's) Fabrewk
  reads the free space of the file system that holds `local/` and refuses when it is below
  `limits.freeDiskMegabytes` (integer, default 2048, direction N); a free space that cannot be read refuses too. New
  code `DISK_LOW`; before a dispatch it halts the task as REQ-TASK-5's codes do. The free space is asked of `df -Pk`,
  a POSIX program, which joins the core's programs. REQ-DOM-5 gains: `status` shows how much `local/evidence/` and
  `local/tmp/` hold.
- **S-4 (corrected after P9O-2 and P9O-3).** A new key, `gate.protectedNames` (a list of names, direction N): a
  changed path with a component equal to one of them must match an allowed-files entry that holds no wildcard, at
  any depth; otherwise the change set refuses it, and the integration's recomputation again
  (`PROTECTED_PATH_WILDCARD`). Its default is `.gitattributes`, `.gitmodules` and the names each built-in adapter
  declares for its tool's instructions (Claude Code: `CLAUDE.md`, `.claude`; Codex: `AGENTS.md`, `.codex`,
  `.agents`). The guard is on paths, not on patterns, so `*.md` and `src/**` stay valid lanes (PD-15) and refuse
  only the change that reaches such a name. `gate.protectedPaths`' default gains the directory the repository's
  `core.hooksPath` names when it lies inside the working tree, read at each guard. Tests: `src/**` with a fake that
  writes `src/AGENTS.md`, `src/.claude/settings.json` and `src/.gitattributes`; the same paths listed literally.
- **S-5 (corrected after P9O-6).** `attempt-ended`'s `usage` is exactly `seconds` and `tokens`. `seconds` is
  Fabrewk's own measure, from the start of the attempt's process to the end Fabrewk observed: an integer, or `null`
  when Fabrewk did not observe the end (an attempt ended as `unknown` by a later command, REQ-RUN-5). `tokens` is
  `null` when the tool reports none, or exactly `input`, `output` and `cached`, each an integer or `null` (PD-39).
  REQ-RUN-7 is amended to say that time is Fabrewk's measure and tokens the tool's report, each unknown, never zero,
  when not observed. Against the `minutes` limit an unknown duration counts as the whole time the attempt was
  allowed, which it cannot have passed, since its cell ends at that limit or with Fabrewk. A view shows a sum with an
  unknown part as unknown (R-037, interpretation 6). Fixed with the code in part 9e.
- **S-6.** REQ-RUN-2: a required tool is present for a dispatch only when the role's cell shows it (3.4); otherwise
  `TOOL_MISSING`, whose words say that the tool lives outside what the confinement shows.
- **S-7.** REQ-SEC-5's criterion gains: `config` lists the residual risks of 3.10, and a test fails when a backend or
  an adapter declares one with no words in every catalogue.
- **S-8 (with M-3).** P8C-8 is closed as declared: the race needs an unconfined process of the same user, so the
  lock's control is labelled cooperative, as `LOCK_HELD` already is, and no crate is added.
- **S-9 (after P9O-4).** REQ-EXT-4 gains: the extension's cell holds its program, read-only, at the path where it was
  found; an interpreter or support file outside the system directories and the repository is absent, and an
  extension that then fails to start is `TOOL_FAILED`, whose words say so. Its criterion gains a permitted extension
  installed outside the repository and the system directories that runs, and one whose interpreter is outside both
  that fails so.
- **S-10 (after P9O-5).** REQ-SEC-1's text and criterion: the writable places of an agent's or a check's cell are
  `/work`, its home and its `/tmp`, the attempt's three directories, all deleted when it ends; a probe fails to write
  anywhere else. REQ-EXT-4 is unchanged: an extension's home and `/tmp` lie inside `local/ext/<name>/`.
- **S-11 (after P9O-2).** A new sentence in REQ-REG-10 and REQ-GATE-8: every Git command Fabrewk runs names an
  empty hooks directory and turns the file-system monitor off; the working tree is brought to a candidate by
  writing each touched path from its object's bytes. This changes `record/git.rs`'s one command builder, built in
  phase 7; part 9b makes the change with P-12 as its test, before any candidate can exist.
- **S-12 (after P9O-1).** REQ-SEC-1 gains: a cell's command starts with a session keyring of its own; its criterion
  gains the planted-key probe.

## 5. What the maestro decides

- **M-1. The agents' network:** A, B or C of 3.5. Recommended: B.
- **M-2. The core's ceiling** (D-069). Measured 7,630 of 8,000. Section 7 projects about 10,500 after phase 9.
  Recommended: 11,000, measured again at each part.
- **M-3. P8C-8:** declare it cooperative with no crate (recommended), or approve a crate that opens each path
  component without following links.
- **M-4. S-2 to S-7** as drafted, or changed.
- **M-5. Review and models.** Whether Codex reviews this design before part 9a starts (recommended: it fixes the
  boundary every part builds on, and a finding costs less here than in code); and part 9a on Fable 5.2, the other
  parts on Opus 5.5 (recommended: 9a is the boundary itself, and gives D-071's calibration a part measured like
  phase 7's).

**Decided (D-075), as recommended.** The maestro decided: B for the network; 11,000 for the core; P8C-8 declared with
no crate; S-2 to S-7 as drafted; Codex reviews this file in session 29; part 9a on Fable 5.2 and the others on
Opus 5.5. What the review changed in S-4 and S-5, and S-9 to S-12, which it added, are the maestro's to approve
again (recorded with R-038).

Not asked now: the permission and the budget to run the real tools (plan row 9), due when part 9c starts.

## 6. The cut into parts

Split where the inputs separate (D-019). Every part delivers code and tests, ends as phase 7's parts did (clean
clone, mutation, Codex's review at effort high, a commit, `STATUS.md`), and starts by reading this file's sections it
names. Codex is GPT-6.1 Sol.

| Part | Delivers | Inputs | Acceptance (checkable) | Model |
| --- | --- | --- | --- | --- |
| 9a | The cell's contract and the runner in the core; the bubblewrap backend with the network of M-1; its own probe; the evidence store; `CONFINEMENT_UNAVAILABLE` for an absent or failing backend; the residual risks in `config` | Sections 3.1 to 3.5, 3.7, 3.9, 3.10; REQ-SEC-1 (the cell), REQ-SEC-2, REQ-REG-13, REQ-SEC-5; S-1, S-7 | REQ-SEC-1's probes as an agent, a check and an extension would be confined (T-01 to T-03, the planted key included, T-06, T-07, T-23; S-10, S-12); T-09's test; REQ-SEC-2 at the library's level; PD-28 with a fixture program; 3.7's open point settled by test | Fable 5.2 high |
| 9b | The attempt's copy and the candidate: the export, the walk and the change set, the three guards on it, the credential scan in a candidate, the candidate commit, the free-space refusal | Section 3.8; REQ-RUN-3, REQ-RUN-4, REQ-SEC-3 (change set); S-2, S-3, S-4 (the protected names on the change set), S-11 (Fabrewk's Git runs no hook); `record/git.rs`, `task.rs`, `scan.rs` | PD-16 (change set), PD-18 to PD-21, PD-22 (the export), PD-45; `LANE_VIOLATION`, `CREDENTIAL_SUSPECTED` in a candidate, `DISK_LOW`; the tests of S-2, S-4 and S-11 | Opus 5.5 high |
| 9c | The adapter's contract and the Claude Code module: launch per role, the packet and the role's result schema, usage, markers, protected instruction files, its preflight and its stream audit | Sections 3.6, 3.9; REQ-RUN-7, REQ-RUN-8, REQ-SEC-1 (adapter), REQ-SEC-7, REQ-SET-9 (dispatch), REQ-GATE-4 (audit); S-4; the tool's own documentation; one recorded real stream (the maestro's permission and budget) | PD-25, PD-26 and PD-27 for this adapter; `REVIEW_INVALID`; the credential question of 3.6 answered by test; the fixture home's settings byte-identical (REQ-SEC-6) | Opus 5.5 high |
| 9d | The Codex module, the same contract: launch per role, its machine rule, its preflight and its stream audit | As 9c, for Codex; 6.38's rule of this machine | PD-25, PD-26 and PD-27 for this adapter; the launch refused in a directory that holds other repositories | Opus 5.5 high |
| 9e | The run, building side: `run` past confinement, attempts and their recovery, the checks before dispatch, the halts, the events' payloads, exit status 4 | REQ-RUN-1, REQ-RUN-2, REQ-RUN-5 to REQ-RUN-7, REQ-TASK-3 (`BASE_MOVED` at `run`), REQ-TASK-5 (building side), REQ-SET-9; S-5, S-6; 9a to 9c | PD-08 to PD-10, PD-21 (the failed export recorded), PD-28, PD-29, PD-39 (record); `TOOL_MISSING`, `BUDGET_EXHAUSTED`, `OUTCOME_UNKNOWN`, `ATTEMPT_REPLACED`; a fake builder reaches a recorded candidate | Opus 5.5 high |
| 9f | Checks, review and verification: the gate, evidence identities, the reviewer's dispatch, rounds, revalidation | REQ-GATE-1 to REQ-GATE-6, REQ-GATE-10, REQ-RUN-9, REQ-TASK-5 (the rest); 9d, 9e | PD-07, PD-23, PD-25 and PD-26 through the flow, PD-33 (moves), PD-46 (gate); `EVIDENCE_STALE`, `NO_PROGRESS`, `ROUNDS_EXHAUSTED`, `PROVIDER_MISSING`, `REVIEW_UNEXAMINED` | Opus 5.5 high |
| 9g | Acceptance and integration: `accept`, the three checks of the preconditions, the move, the working tree, `reject` saying where the branch is | REQ-GATE-7, REQ-GATE-8; D-068 point 3; S-4 and S-11 at integration (3.8 rule 9); 9f | PD-11 to PD-14, PD-16 (integration), PD-37 (accept), PD-44; T-28's two tests; `BASE_MOVED`, `WORKTREE_DIRTY`, `MERGE_IN_PROGRESS`; `MOVE_NOT_ALLOWED` for `accept` | Opus 5.5 high |
| 9h | Extensions: the runner, the variables, the requests, their cell, exit status 10 | REQ-EXT-1 to REQ-EXT-5, REQ-CLI-3; AD-5's contract; S-9; 9a | Their criteria, S-9's two cases included; `EXTENSION_DIGEST`, `EXTENSION_OUTPUT_INVALID`; REQ-SEC-2 for extensions | Opus 5.5 high |
| 9i | The whole: the flows F-1 to F-3 with fake tools, the crash suite over F-1, one real dispatch per provider with its preflight first, the phase's close | 11.5; plan row 9; every part | 11.5's flows and crash suite; the real dispatches' receipts; every criterion of section 13 for phase 9 met; D-071's comparison reported | Opus 5.5 high |

Order: 9a, 9b, 9c, 9d, 9e, 9f, 9g, 9h, 9i. 9b needs no confinement and may run before 9a. `run` keeps refusing
`CONFINEMENT_UNAVAILABLE` at the command line until part 9e, the declaration growing by part as it grew by phase
(D-053).

## 7. Size projection

Counted lines (REQ-BUD-1). The modules have their own frozen ceilings: the backend 400, each adapter 600 with its
stream audit; each new crate is a ring in `tools/budget` at its kind's ceiling (D-041, D-042), which is no change to
a ceiling.

| Part | Core | Module |
| --- | --- | --- |
| 9a | +250 (the contract, the runner, the evidence store, the residual risks) | backend, about 300 |
| 9b | +400 | — |
| 9c | +150 (the adapter's contract, the packet, the result schema) | Claude Code, about 500 |
| 9d | +30 | Codex, about 500 |
| 9e | +600 | — |
| 9f | +550 | — |
| 9g | +550 | — |
| 9h | +300 | — |
| 9i | +50 | — |
| Total | about +2,880: 7,630 to about 10,500 | |

Phase 7's projections ran short by a third where the file system was adversarial (7d2); the same error here would
reach about 11,400. An adapter with its audit of twenty anomalies may press its 600; it is measured in part 9c.
