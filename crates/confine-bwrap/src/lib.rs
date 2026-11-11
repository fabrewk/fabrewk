//! The bubblewrap backend (ring `Confinement`; `confinement-design.md` 3.3, 3.5 and 3.7; part 9a). It turns the
//! core's cell into a `bwrap` command: every namespace new, a new session, no new privileges, the system read-only,
//! a minimal `/dev` and `/etc`, the cell's home, `/tmp` and mounts, the root read-only, and a seccomp filter on the
//! kernel's keys. A cell with a network gets a private one from `pasta`, which connects only outwards (S-1).
//!
//! How the two compose, settled by test in part 9a: `bwrap` is Fabrewk's direct child, started through `sh` only to
//! open the filter on descriptor 3 (`exec` keeps the identifier). Its command is a wrapper that waits for one line on
//! standard input. `pasta`, in the foreground as Fabrewk's second child, joins the namespaces of that wrapper once it
//! has replaced `bwrap`'s code (the processes of `bwrap` cannot be joined), and the line is written once the cell's
//! default route is up. `--dev` is not used: it makes `bwrap` nest a second user namespace, which `pasta` cannot
//! configure; for the same reason a cell with a network cannot be kept from nesting (`--disable-userns`), and this
//! backend refuses that cell. `pasta` cannot watch the namespace on this machine, so Fabrewk ends it after the cell;
//! after Fabrewk is killed during a run the cell dies with it (`--die-with-parent`) but an idle `pasta` remains, a
//! residual risk `config` declares.

use fabrewk_core::confine::{Access, Backend, Cell, Network, Programs, Started};
use fabrewk_core::detect::Program;
use fabrewk_core::{Code, Error};
use std::ffi::OsString;
use std::fs;
use std::io::Write;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

/// The backend, as the binary wires it on Linux.
pub const BACKEND: Backend = Backend { name: "bubblewrap", programs: &PROGRAMS, start, stop, residual: &RESIDUAL };

/// The programs it drives, detected at every start like the core's own (REQ-SET-8, S-1).
pub const PROGRAMS: [Program; 2] = [
    Program { adapter: None, executable: "bwrap", version: &["--version"], tokens: false, skills: &[] },
    Program { adapter: None, executable: "pasta", version: &["--version"], tokens: false, skills: &[] },
];

/// The residual risks it declares (`confinement-design.md` 3.10: T-04, T-08, T-10, T-24, T-29), and the idle `pasta`
/// a killed Fabrewk leaves.
pub const RESIDUAL: [&str; 6] =
    ["outbound-network", "kernel", "resources", "host-names", "key-serials", "network-helper"];

/// How long a cell may take to start, and its first process to be gone after its end.
const SETUP: Duration = Duration::from_secs(10);

/// One classic BPF instruction (`struct sock_filter`): code, jumps if true and if false, constant.
const fn op(code: u16, jt: u8, jf: u8, k: u32) -> [u8; 8] {
    let (c, k) = (code.to_le_bytes(), k.to_le_bytes());
    [c[0], c[1], jt, jf, k[0], k[1], k[2], k[3]]
}

const LOAD: u16 = 0x20; // BPF_LD | BPF_W | BPF_ABS
const JEQ: u16 = 0x15; // BPF_JMP | BPF_JEQ | BPF_K
const RET: u16 = 0x06; // BPF_RET | BPF_K
const ALLOW: u32 = 0x7fff_0000;
const KILL: u32 = 0x8000_0000; // SECCOMP_RET_KILL_PROCESS
const EPERM: u32 = 0x0005_0001; // SECCOMP_RET_ERRNO | EPERM
const AARCH64: u32 = 0xc000_00b7; // AUDIT_ARCH_AARCH64

/// The filter for aarch64 (S-12, P-13, P-14): another architecture's call ends the process; `keyctl`, `add_key`,
/// `request_key` and the three `io_uring` calls are refused, and so is a socket of the cryptographic family
/// (`AF_ALG`, 38, the first argument's low half); everything else is allowed.
pub const FILTER: [[u8; 8]; 15] = [
    op(LOAD, 0, 0, 4),
    op(JEQ, 1, 0, AARCH64),
    op(RET, 0, 0, KILL),
    op(LOAD, 0, 0, 0),
    op(JEQ, 9, 0, 219),
    op(JEQ, 8, 0, 217),
    op(JEQ, 7, 0, 218),
    op(JEQ, 6, 0, 425),
    op(JEQ, 5, 0, 426),
    op(JEQ, 4, 0, 427),
    op(JEQ, 0, 2, 198),
    op(LOAD, 0, 0, 16),
    op(JEQ, 1, 0, 38),
    op(RET, 0, 0, ALLOW),
    op(RET, 0, 0, EPERM),
];

/// The architectures a filter exists for; a platform counts only when tested.
const ARCHES: [&str; 1] = ["aarch64"];

/// What of `/etc` a cell sees as the host has it, when it exists: the certificate stores, the cryptographic policy
/// they read, the dynamic loader's cache and configuration, the alternatives.
const ETC: [&str; 9] = [
    "/etc/pki/ca-trust",
    "/etc/pki/tls",
    "/etc/ssl",
    "/etc/ca-certificates",
    "/etc/crypto-policies",
    "/etc/ld.so.cache",
    "/etc/ld.so.conf",
    "/etc/ld.so.conf.d",
    "/etc/alternatives",
];

/// The address a cell asks for names, which `pasta` forwards to the host's resolver (P-5).
const DNS: &str = "169.254.0.53";

/// The cell's command: it waits for one line on standard input, then becomes the command with standard input on
/// `/dev/null`, without the variables the shells set.
const WRAPPER: &str = "read -r _ && exec /usr/bin/env -u PWD -u SHLVL -u _ \"$@\" </dev/null";

fn unavailable(path: Option<&Path>, detail: impl Into<String>) -> Error {
    Error::Refused { code: Code::ConfinementUnavailable, path: path.map(Path::to_path_buf), detail: detail.into() }
}

fn io(path: &Path) -> impl FnOnce(std::io::Error) -> Error + '_ {
    move |error| Error::Io { path: path.to_path_buf(), error }
}

/// The files of `/etc` Fabrewk writes for a cell: its one user, the person's identifiers; names resolved from
/// `hosts`, then through `pasta` when it has a network.
fn etc_files(network: Network, uid: u32, gid: u32) -> [(&'static str, String); 5] {
    let resolv = if network == Network::Outbound { format!("nameserver {DNS}\n") } else { String::new() };
    [
        ("passwd", format!("agent:x:{uid}:{gid}:agent:/home/agent:/bin/sh\n")),
        ("group", format!("agent:x:{gid}:\n")),
        ("hosts", "127.0.0.1 localhost cell\n::1 localhost cell\n".to_owned()),
        ("nsswitch.conf", "passwd: files\ngroup: files\nhosts: files dns\n".to_owned()),
        ("resolv.conf", resolv),
    ]
}

/// `bwrap`'s arguments for `cell`, with the written files of `/etc` in `etc`. No value of the environment is in them
/// (T-23): it reaches `bwrap` by inheritance.
pub fn arguments(cell: &Cell, etc: &Path) -> Vec<OsString> {
    let mut a: Vec<OsString> = Vec::new();
    let mut put = |items: &[&dyn AsRef<std::ffi::OsStr>]| a.extend(items.iter().map(|i| i.as_ref().to_owned()));
    put(&[&"--unshare-all", &"--die-with-parent", &"--new-session", &"--hostname", &"cell"]);
    if !cell.nested {
        put(&[&"--unshare-user", &"--disable-userns"]);
    }
    put(&[&"--ro-bind", &"/usr", &"/usr"]);
    for dir in ["/bin", "/sbin", "/lib", "/lib64"] {
        match fs::read_link(dir) {
            Ok(target) => put(&[&"--symlink", &target, &dir]),
            Err(_) if Path::new(dir).is_dir() => put(&[&"--ro-bind", &dir, &dir]),
            Err(_) => {}
        }
    }
    put(&[&"--proc", &"/proc"]);
    // `tty` too, which a sandbox nested in the cell binds; with no controlling terminal it opens nothing.
    for dev in ["null", "zero", "full", "random", "urandom", "tty"] {
        let dev = format!("/dev/{dev}");
        put(&[&"--dev-bind-try", &dev, &dev]);
    }
    for (n, name) in ["stdin", "stdout", "stderr"].iter().enumerate() {
        put(&[&"--symlink", &format!("/proc/self/fd/{n}"), &format!("/dev/{name}")]);
    }
    put(&[&"--symlink", &"/proc/self/fd", &"/dev/fd"]);
    for path in ETC {
        put(&[&"--ro-bind-try", &path, &path]);
    }
    for (name, _) in etc_files(cell.network, 0, 0) {
        put(&[&"--ro-bind", &etc.join(name), &format!("/etc/{name}")]);
    }
    put(&[&"--bind", &cell.home, &"/home/agent", &"--bind", &cell.tmp, &"/tmp"]);
    let mut hidden = Vec::new();
    for m in &cell.mounts {
        match m.access {
            Access::ReadOnly => put(&[&"--ro-bind", &m.host, &m.inside]),
            Access::ReadWrite => put(&[&"--bind", &m.host, &m.inside]),
            Access::Hidden => {
                put(&[&"--tmpfs", &m.inside]);
                hidden.push(&m.inside);
            }
        }
    }
    for inside in hidden {
        put(&[&"--remount-ro", inside]);
    }
    put(&[&"--remount-ro", &"/", &"--chdir", &cell.cwd, &"--seccomp", &"3"]);
    put(&[&"--", &"/bin/sh", &"-c", &WRAPPER, &"cell"]);
    a.extend(cell.argv.iter().cloned());
    a
}

/// The first child of process `pid`, as the host's `/proc` lists it.
fn first_child(pid: u32) -> Option<u32> {
    fs::read_to_string(format!("/proc/{pid}/task/{pid}/children")).ok()?.split_whitespace().next()?.parse().ok()
}

/// Whether the network namespace of process `pid` has a default route, IPv4 or IPv6, through a device that is not
/// the loopback (a new namespace holds IPv6's unreachable default on `lo`).
fn routed(pid: u32) -> bool {
    let table = |name: &str| fs::read_to_string(format!("/proc/{pid}/net/{name}")).unwrap_or_default();
    let fields = |l: &str| l.split_whitespace().map(str::to_owned).collect::<Vec<_>>();
    let v4 = table("route").lines().skip(1).map(fields).any(|f| f.len() > 1 && f[0] != "lo" && f[1] == "00000000");
    let v6 = table("ipv6_route")
        .lines()
        .map(fields)
        .any(|f| f.len() == 10 && f[0] == "0".repeat(32) && f[1] == "00" && f[9] != "lo");
    v4 || v6
}

/// Waits, until `deadline`, for `what`; `false` when the deadline passed.
fn until(deadline: Instant, mut what: impl FnMut() -> bool) -> bool {
    loop {
        if what() {
            return true;
        }
        if Instant::now() >= deadline {
            return false;
        }
        std::thread::sleep(Duration::from_millis(2));
    }
}

/// Gives the cell of process `first` a private network (`confinement-design.md` 3.5, P-4, P-5): `pasta` joins the
/// namespaces of the cell's command once it is the wrapper, maps none of the host's ports, addresses or loopback, and
/// forwards names to the host's resolver; ready once the default route is up.
fn attach(first: u32, pasta: &Path, evidence: &Path, deadline: Instant) -> Result<Child, Error> {
    let mut command = None;
    let joinable = until(deadline, || {
        command = first_child(first).filter(|c| fs::read_link(format!("/proc/{c}/ns/net")).is_ok());
        command.is_some()
    });
    let Some(command) = command.filter(|_| joinable) else {
        return Err(unavailable(None, "the cell's command did not start"));
    };
    let log = evidence.join("network");
    let err = fs::File::options().write(true).create_new(true).mode(0o600).open(&log).map_err(io(&log))?;
    let ns = |kind: &str| format!("/proc/{command}/ns/{kind}");
    let fixed = ["-f", "--quiet", "--config-net", "-t", "none", "-u", "none", "-T", "none", "-U", "none"];
    let mut helper = Command::new(pasta)
        .args(fixed)
        .args(["--no-map-gw", "--no-netns-quit", "--dns-forward", DNS, "--netns", &ns("net"), "--userns", &ns("user")])
        .env_clear()
        .current_dir("/")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(err)
        .spawn()
        .map_err(io(pasta))?;
    let mut ended = false;
    let up = until(deadline, || {
        ended = !matches!(helper.try_wait(), Ok(None));
        ended || routed(command)
    });
    if up && !ended {
        return Ok(helper);
    }
    let _ = helper.kill();
    let _ = helper.wait();
    Err(unavailable(Some(&log), format!("pasta gave the cell no network; its output is kept in {}", log.display())))
}

/// Starts `cell` (`confinement-design.md` 3.3, 3.7).
fn start(cell: &Cell, programs: &Programs, scratch: &Path, evidence: &Path) -> Result<Started, Error> {
    if !ARCHES.contains(&std::env::consts::ARCH) {
        return Err(unavailable(None, format!("no seccomp filter exists for {}", std::env::consts::ARCH)));
    }
    if cell.network == Network::Outbound && !cell.nested {
        return Err(unavailable(None, "a cell with a network cannot be kept from nesting by this backend"));
    }
    let program = |name: &str| programs.get(name).ok_or_else(|| unavailable(None, format!("{name}: not detected")));
    let (bwrap, pasta) = (program("bwrap")?, program("pasta")?);
    let meta = fs::metadata(scratch).map_err(io(scratch))?;
    let etc = scratch.join("etc");
    fs::create_dir(&etc).map_err(io(&etc))?;
    for (name, text) in etc_files(cell.network, meta.uid(), meta.gid()) {
        fs::write(etc.join(name), text).map_err(io(&etc.join(name)))?;
    }
    let filter = scratch.join("filter");
    fs::write(&filter, FILTER.concat()).map_err(io(&filter))?;
    let mut child = Command::new("/bin/sh")
        .args(["-c", "exec \"$@\" 3<\"$0\""])
        .arg(&filter)
        .arg(bwrap)
        .args(arguments(cell, &etc))
        .env_clear()
        .envs(cell.env.iter().map(|(k, v)| (k, v)))
        .current_dir("/")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(io(bwrap))?;
    let deadline = Instant::now() + SETUP;
    // `bwrap`'s first process in the new namespaces; none when `bwrap` ended first, whose output says why.
    let mut first = None;
    until(deadline, || {
        first = first_child(child.id());
        first.is_some() || !matches!(child.try_wait(), Ok(None))
    });
    let mut started = Started { child, beside: Vec::new(), first, failed: None };
    if let (Network::Outbound, Some(first)) = (cell.network, first) {
        match attach(first, pasta, evidence, deadline) {
            Ok(helper) => started.beside.push(helper),
            // The command stays held: its standard input closes with no line, and the runner ends the cell.
            Err(e) => {
                started.failed = Some(e);
                drop(started.child.stdin.take());
                return Ok(started);
            }
        }
    }
    if let Some(mut stdin) = started.child.stdin.take() {
        // A cell that ended already has closed it; its end is the runner's to see.
        let _ = stdin.write_all(b"\n");
    }
    Ok(started)
}

/// Ends a started cell: its direct child, then, once the cell's first process is gone (only after every process of
/// its namespace has ended), what was started beside it.
fn stop(s: &mut Started) -> Result<(), Error> {
    let _ = s.child.kill();
    let waited = s.child.wait();
    let gone = s.first.is_none_or(|p| until(Instant::now() + SETUP, || !PathBuf::from(format!("/proc/{p}")).exists()));
    for helper in &mut s.beside {
        let _ = helper.kill();
        let _ = helper.wait();
    }
    waited.map_err(io(Path::new("bwrap")))?;
    if gone {
        return Ok(());
    }
    let detail = "bwrap: the cell's first process had not ended 10 seconds after the cell's end";
    Err(Error::Refused { code: Code::ToolFailed, path: None, detail: detail.into() })
}
