//! The bubblewrap backend on this machine (part 9a; REQ-SEC-1 with S-1, S-10, S-12; REQ-SEC-2; REQ-REG-13; T-01 to
//! T-03, T-06, T-07, T-09, T-23, T-29; PD-28): real cells, as an agent's, a check's and an extension's would be
//! confined, probed from inside by `tests/probe.py`. Every fixture lives under Cargo's `CARGO_TARGET_TMPDIR`; nothing
//! of the machine's home, private folder or SSH agent is read, only named, so that a cell can be shown not to see it.
//! Keys planted in the kernel's keyrings are removed by the test that planted them.

use fabrewk_confine_bwrap::{BACKEND, FILTER, arguments};
use fabrewk_core::confine::{self, Access, Cell, End, Limits, Mount, Network, Programs, Ran, Scratch};
use fabrewk_core::detect::{Detection, Found, How, Space};
use fabrewk_core::record::sha256_hex;
use fabrewk_core::{Code, Error};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

const PROBE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/probe.py");
const HELPER: &str = "FABREWK_TEST_CELL_HELPER";

fn unique(name: &str) -> String {
    static N: AtomicUsize = AtomicUsize::new(0);
    format!("{name}-{}-{}", std::process::id(), N.fetch_add(1, Ordering::SeqCst))
}

fn which(executable: &str) -> PathBuf {
    std::env::split_paths(&std::env::var_os("PATH").unwrap())
        .map(|d| d.join(executable))
        .find(|p| p.is_file())
        .unwrap_or_else(|| panic!("{executable} is not in PATH; part 9a's tests need it"))
}

/// The backend's programs as `available` gives them, already verified by the probe's own test, which `available`
/// alone may run.
fn programs() -> Programs {
    let domain = Path::new(env!("CARGO_TARGET_TMPDIR")).join("unprobed");
    confine::available(Some(&BACKEND), &mut detection(Some("2026-11-11T00:00:00Z"), &[]), &domain).unwrap()
}

/// A repository with its domain, under the test's own directory.
struct Fx {
    repo: PathBuf,
    domain: PathBuf,
}

fn fixture(name: &str) -> Fx {
    let repo = Path::new(env!("CARGO_TARGET_TMPDIR")).join(unique(&format!("cell-{name}"))).join("repo");
    let _ = std::fs::remove_dir_all(&repo);
    let domain = repo.join(".brewstillery");
    for dir in ["data/events", "local/cache", "local/ext/probe"] {
        std::fs::create_dir_all(domain.join(dir)).unwrap();
    }
    std::fs::write(repo.join("README.md"), "the product\n").unwrap();
    std::fs::write(domain.join("local/cache/detected.json"), "{}\n").unwrap();
    std::fs::write(domain.join("local/secret"), "not for extensions\n").unwrap();
    Fx { repo, domain }
}

fn uid() -> u32 {
    use std::os::unix::fs::MetadataExt;
    std::fs::metadata(env!("CARGO_TARGET_TMPDIR")).unwrap().uid()
}

fn env() -> Vec<(OsString, OsString)> {
    [("PATH", "/usr/bin:/bin"), ("HOME", "/home/agent"), ("TMPDIR", "/tmp"), ("LANG", "C.UTF-8")]
        .map(|(k, v)| (k.into(), v.into()))
        .into()
}

fn mount(host: &Path, inside: &str, access: Access) -> Mount {
    Mount { host: host.to_path_buf(), inside: inside.into(), access }
}

/// An agent's cell (network, nested), a check's (neither), each with its `/work`; the probe bound read-only.
fn cell(scratch: &Scratch, network: Network, nested: bool, argv: &[&str]) -> Cell {
    Cell {
        home: scratch.home.clone(),
        tmp: scratch.tmp.clone(),
        mounts: vec![
            mount(&scratch.work, "/work", Access::ReadWrite),
            mount(Path::new(PROBE), "/probe/probe.py", Access::ReadOnly),
        ],
        network,
        nested,
        env: env(),
        cwd: "/work".into(),
        argv: argv.iter().map(OsString::from).collect(),
    }
}

/// An extension's cell (`confinement-design.md` 3.4): the repository read-only at its own path, `local/` hidden but
/// its own area, writable, and `local/cache/`, read-only; its home and `/tmp` inside its area; no network.
fn extension(fx: &Fx, argv: &[&str]) -> Cell {
    let area = fx.domain.join("local/ext/probe");
    for d in ["home", "tmp"] {
        std::fs::create_dir_all(area.join(d)).unwrap();
    }
    let at = |p: &Path| p.to_str().unwrap().to_owned();
    let local = fx.domain.join("local");
    let cache = local.join("cache");
    Cell {
        home: area.join("home"),
        tmp: area.join("tmp"),
        mounts: vec![
            mount(&fx.repo, &at(&fx.repo), Access::ReadOnly),
            mount(&local, &at(&local), Access::Hidden),
            mount(&area, &at(&area), Access::ReadWrite),
            mount(&cache, &at(&cache), Access::ReadOnly),
            mount(Path::new(PROBE), "/probe/probe.py", Access::ReadOnly),
        ],
        network: Network::None,
        nested: false,
        env: env(),
        cwd: at(&fx.repo).into(),
        argv: argv.iter().map(OsString::from).collect(),
    }
}

fn limits(seconds: u64) -> Limits {
    Limits { time: Duration::from_secs(seconds), bytes: 1 << 20 }
}

fn run(fx: &Fx, name: &str, make: impl FnOnce(&Scratch) -> Cell, limits: Limits) -> (Ran, String, Scratch) {
    let name = unique(name);
    let scratch = Scratch::create(&fx.domain, &name).unwrap();
    let evidence = confine::evidence(&fx.domain, &name).unwrap();
    let cell = make(&scratch);
    let ran = confine::run(&BACKEND, &programs(), &cell, &scratch, &evidence, limits).unwrap();
    let out = String::from_utf8_lossy(&std::fs::read(&ran.stdout.path).unwrap()).into_owned();
    (ran, out, scratch)
}

/// Runs the probe in the cell `make` gives and asserts that every expectation of `spec` held. The expectations reach
/// it in a file of its home, so that no argument holds them.
fn probe(fx: &Fx, name: &str, spec: Value, make: impl FnOnce(&Scratch, &[&str]) -> Cell) -> Scratch {
    let argv = ["/usr/bin/python3", "/probe/probe.py", "cell", "/home/agent/spec.json"];
    let make = |s: &Scratch| {
        let cell = make(s, &argv);
        std::fs::write(cell.home.join("spec.json"), spec.to_string()).unwrap();
        cell
    };
    let (ran, out, scratch) = run(fx, name, make, limits(60));
    let err = std::fs::read_to_string(&ran.stderr.path).unwrap();
    assert_eq!(ran.end, End::Exited(0), "{name}: {out}{err}");
    let report: Value = serde_json::from_str(out.trim()).unwrap_or_else(|_| panic!("{name}: {out}{err}"));
    assert_eq!(report["failures"], json!([]), "{name}");
    scratch
}

fn home() -> String {
    std::env::var("HOME").unwrap()
}

/// What no agent's or check's cell may see (T-01, T-02): the repository, its domain, the person's home, its SSH and
/// private folders, the runtime directory with the agent's and the session bus's sockets, the host's `/tmp`, and
/// every directory the cell does not list.
fn hidden(fx: &Fx) -> Vec<String> {
    let mut paths = vec![
        fx.repo.display().to_string(),
        fx.domain.display().to_string(),
        home(),
        format!("{}/.ssh", home()),
        format!("{}/.config/fabrewk", home()),
        format!("/run/user/{}", uid()),
    ];
    paths.extend(std::env::var("SSH_AUTH_SOCK").ok());
    paths.extend(["/run", "/var", "/sys", "/root", "/opt", "/mnt", "/media", "/srv", "/boot"].map(String::from));
    paths.extend(["/tmp/.X11-unix", "/dev/shm", "/dev/pts", "/dev/net/tun", "/etc/shadow"].map(String::from));
    paths
}

/// Where an agent or a check may write (S-10), and where not.
fn writes() -> (Vec<&'static str>, Vec<&'static str>) {
    (
        vec!["/work/w", "/home/agent/w", "/tmp/w"],
        vec!["/w", "/usr/w", "/etc/w", "/etc/passwd", "/dev/w", "/home/w", "/proc/w", "/probe/w", "/bin/w"],
    )
}

/// Listeners of the host a cell must not reach (T-03): an abstract socket, the loopback, and the host's own address.
struct Listeners {
    _held: (std::os::unix::net::UnixListener, std::net::TcpListener, std::net::TcpListener),
    targets: Vec<Value>,
}

fn listeners() -> Listeners {
    use std::os::linux::net::SocketAddrExt;
    let name = unique("fabrewk-abstract");
    let addr = std::os::unix::net::SocketAddr::from_abstract_name(name.as_bytes()).unwrap();
    let unix = std::os::unix::net::UnixListener::bind_addr(&addr).unwrap();
    let lo = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let any = std::net::TcpListener::bind("0.0.0.0:0").unwrap();
    let probe = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
    probe.connect("192.0.2.1:9").expect("the host has a route outwards, which a cell's network needs");
    let own = probe.local_addr().unwrap().ip().to_string();
    let targets = vec![
        json!({"abstract": name}),
        json!({"tcp": ["127.0.0.1", lo.local_addr().unwrap().port()]}),
        json!({"tcp": [own, any.local_addr().unwrap().port()]}),
    ];
    Listeners { _held: (unix, lo, any), targets }
}

/// Keys planted in the host's session keyring and, with default permissions, in its user keyring (T-29, S-12),
/// removed when dropped.
struct Keys {
    description: String,
    serials: Vec<i64>,
}

impl Keys {
    fn plant() -> Keys {
        let description = unique("fabrewk-key");
        let out = Command::new("python3").args([PROBE, "plant", &description]).output().unwrap();
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        let serials = String::from_utf8(out.stdout).unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        Keys { description, serials }
    }

    fn spec(&self) -> Value {
        json!([self.serials, self.description])
    }
}

impl Drop for Keys {
    fn drop(&mut self) {
        let serials: Vec<String> = self.serials.iter().map(i64::to_string).collect();
        let _ = Command::new("python3").args([PROBE, "remove", &self.description]).args(&serials).status();
    }
}

// ---- REQ-SEC-1: the cells, probed from inside -----------------------------------------------------------------------

#[test]
fn an_agents_cell_sees_and_writes_only_what_it_lists_and_reaches_nothing_of_the_host() {
    let fx = fixture("agent");
    let (writable, unwritable) = writes();
    let net = listeners();
    let keys = Keys::plant();
    let spec = json!({
        "absent": hidden(&fx), "writable": writable, "unwritable": unwritable, "unmakeable": ["/x", "/usr/x"],
        "present": ["/etc/passwd", "/etc/resolv.conf", "/usr/bin/python3"],
        "listing": {"/": ["bin", "dev", "etc", "home", "lib", "lib64", "probe", "proc", "sbin", "tmp", "usr", "work"],
                    "/dev": ["fd", "full", "null", "random", "stderr", "stdin", "stdout", "tty", "urandom", "zero"],
                    "/home": ["agent"]},
        "uid": uid(), "routed": true, "unreachable": net.targets, "keys": keys.spec(), "nested": true,
    });
    let scratch = probe(&fx, "agent", spec, |s, argv| cell(s, Network::Outbound, true, argv));
    // The three writable places held what the cell wrote, and are gone with the attempt.
    for d in [&scratch.work, &scratch.home, &scratch.tmp] {
        assert_eq!(std::fs::read_to_string(d.join("w")).unwrap(), "written\n");
    }
    let root = scratch.root.clone();
    scratch.remove().unwrap();
    assert!(!root.exists());
}

#[test]
fn a_checks_cell_has_no_network_and_cannot_nest() {
    let fx = fixture("check");
    let (writable, unwritable) = writes();
    let net = listeners();
    let keys = Keys::plant();
    let spec = json!({
        "absent": hidden(&fx), "writable": writable, "unwritable": unwritable, "uid": uid(), "routed": false,
        "unreachable": net.targets, "keys": keys.spec(),
    });
    probe(&fx, "check", spec, |s, argv| cell(s, Network::None, false, argv)).remove().unwrap();
    // A sandbox of its own starts in an agent's cell and cannot in a check's.
    let nest = "/usr/bin/bwrap --unshare-all --ro-bind /usr /usr --symlink usr/lib /lib --symlink usr/lib64 /lib64 \
                --symlink usr/bin /bin --proc /proc --dev /dev /usr/bin/true";
    let nest: Vec<&str> = nest.split_whitespace().collect();
    for (network, nested, end) in [(Network::Outbound, true, true), (Network::None, false, false)] {
        let (ran, _, scratch) = run(&fx, "nest", |s| cell(s, network, nested, &nest), limits(30));
        assert_eq!(
            ran.end == End::Exited(0),
            end,
            "{network:?}: {}",
            std::fs::read_to_string(&ran.stderr.path).unwrap()
        );
        scratch.remove().unwrap();
    }
}

#[test]
fn an_extensions_cell_reads_the_repository_and_writes_only_its_own_area() {
    let fx = fixture("extension");
    let at = |p: PathBuf| p.display().to_string();
    let area = fx.domain.join("local/ext/probe");
    let spec = json!({
        "absent": [home() + "/.ssh", home() + "/.config/fabrewk", at(fx.domain.join("local/secret")), "/run".to_owned(),
                   format!("/run/user/{}", uid())],
        "readable": [at(fx.repo.join("README.md")), at(fx.domain.join("local/cache/detected.json"))],
        "writable": [at(area.join("w")), "/home/agent/w".to_owned(), "/tmp/w".to_owned()],
        "unwritable": [at(fx.repo.join("w")), at(fx.domain.join("data/w")), at(fx.domain.join("local/w")),
                       at(fx.domain.join("local/cache/w")), "/w".to_owned()],
        "listing": {at(fx.domain.join("local")): ["cache", "ext"], at(fx.domain.join("local/ext")): ["probe"]},
        "uid": uid(), "routed": false,
    });
    probe(&fx, "extension", spec, |_, argv| extension(&fx, argv));
    assert_eq!(std::fs::read_to_string(area.join("w")).unwrap(), "written\n");
    assert_eq!(std::fs::read_to_string(area.join("home/w")).unwrap(), "written\n");
}

/// Runs this test binary again as the person's process would be, with a variable planted in its environment and, in
/// `mode`, a cell to run; gives the child.
fn helper(test: &str, mode: &str, planted: &str) -> std::process::Child {
    Command::new(std::env::current_exe().unwrap())
        .args([test, "--exact", "--nocapture", "--test-threads=1"])
        .env(HELPER, mode)
        .env("FABREWK_PLANTED", planted)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
}

#[test]
fn nothing_of_fabrewks_own_environment_or_arguments_reaches_a_cell() {
    let planted = unique("planted-value");
    if std::env::var(HELPER).as_deref() == Ok("environment") {
        let fx = fixture("environment");
        let names: Vec<&str> = ["PATH", "HOME", "TMPDIR", "LANG"].into();
        let spec = json!({"planted": ["FABREWK_PLANTED", std::env::var("FABREWK_PLANTED").unwrap()],
                          "environment": names, "uid": uid()});
        probe(&fx, "environment", spec, |s, argv| cell(s, Network::Outbound, true, argv));
        return;
    }
    let out = helper("nothing_of_fabrewks_own_environment_or_arguments_reaches_a_cell", "environment", &planted)
        .wait_with_output()
        .unwrap();
    let text = String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success() && text.contains("... ok"), "{text}");
    // No value of the cell's environment is in `bwrap`'s arguments, which a cell can read (T-23, P-2).
    let fx = fixture("arguments");
    let scratch = Scratch::create(&fx.domain, &unique("arguments")).unwrap();
    let mut c = cell(&scratch, Network::Outbound, true, &["/usr/bin/true"]);
    c.env.push(("TOKEN".into(), planted.clone().into()));
    let args = arguments(&c, &scratch.root.join("etc"));
    assert!(args.iter().all(|a| !a.to_string_lossy().contains(&planted)));
    assert!(args.iter().any(|a| a == "--die-with-parent") && args.iter().any(|a| a == "--new-session"));
}

// ---- T-09: a cell ends at its limit and with Fabrewk ----------------------------------------------------------------

/// The host's processes whose command line holds `marker`, with those lines.
fn holding(marker: &str) -> Vec<(u32, String)> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir("/proc").unwrap().flatten() {
        let Ok(pid) = entry.file_name().to_string_lossy().parse::<u32>() else { continue };
        let line = std::fs::read(entry.path().join("cmdline")).unwrap_or_default();
        let line = String::from_utf8_lossy(&line).replace('\0', " ");
        if line.contains(marker) && pid != std::process::id() {
            found.push((pid, line));
        }
    }
    found
}

/// A command whose child leaves its session and ignores every signal it can, marked by `marker`.
fn stubborn(marker: &str) -> String {
    format!("setsid sh -c 'trap \"\" TERM INT HUP QUIT; while :; do sleep 1; done' {marker} & sleep 600")
}

#[test]
fn a_process_that_ignores_signals_in_its_own_session_is_gone_after_the_time_limit() {
    let fx = fixture("limit");
    for network in [Network::Outbound, Network::None] {
        let marker = unique("stubborn");
        let script = stubborn(&marker);
        let begun = Instant::now();
        let (ran, _, scratch) = run(
            &fx,
            "limit",
            |s| cell(s, network, network == Network::Outbound, &["/bin/sh", "-c", &script]),
            limits(2),
        );
        assert_eq!(ran.end, End::TimedOut);
        assert!(begun.elapsed() < Duration::from_secs(15));
        assert_eq!(holding(&marker), vec![], "{network:?}");
        scratch.remove().unwrap();
    }
}

#[test]
fn a_cell_dies_with_fabrewk_and_its_idle_pasta_remains() {
    if let Ok(marker) = std::env::var(HELPER).as_deref().map(str::to_owned)
        && marker.starts_with("killed-")
    {
        let fx = fixture("killed");
        let script = stubborn(&marker);
        run(&fx, "killed", |s| cell(s, Network::Outbound, true, &["/bin/sh", "-c", &script]), limits(600));
        unreachable!("the parent kills this process first");
    }
    let marker = unique("killed-stubborn");
    let mut child = helper("a_cell_dies_with_fabrewk_and_its_idle_pasta_remains", &marker, "x");
    // The cell's command is the process that became `sh -c <script>`; its loop, the one that left its session.
    let deadline = Instant::now() + Duration::from_secs(20);
    let command = loop {
        let found = holding(&marker);
        let command = found.iter().find(|(_, l)| l.starts_with("/bin/sh -c setsid")).map(|(p, _)| *p);
        if let Some(c) = command.filter(|_| found.iter().any(|(_, l)| l.starts_with("sh -c trap"))) {
            break c;
        }
        assert!(Instant::now() < deadline, "the cell did not start: {found:?}");
        std::thread::sleep(Duration::from_millis(20));
    };
    let pasta = loop {
        let netns = format!("/proc/{command}/ns/net");
        if let Some((p, _)) = holding(&netns).into_iter().find(|(_, l)| l.contains("pasta")) {
            break p;
        }
        assert!(Instant::now() < deadline, "no pasta for the cell");
        std::thread::sleep(Duration::from_millis(20));
    };
    child.kill().unwrap();
    child.wait().unwrap();
    let gone = Instant::now() + Duration::from_secs(5);
    while !holding(&marker).is_empty() && Instant::now() < gone {
        std::thread::sleep(Duration::from_millis(20));
    }
    assert_eq!(holding(&marker), vec![], "the cell outlived Fabrewk");
    // The residual risk `network-helper`: the cell's pasta stays, idle; the test ends it, by its identifier, now.
    assert!(Path::new(&format!("/proc/{pasta}")).exists(), "pasta ended with the cell; the residual risk is void");
    assert!(Command::new("kill").arg(pasta.to_string()).status().unwrap().success());
}

// ---- PD-28, REQ-REG-13: the evidence of each run ----------------------------------------------------------------------

fn mode(path: &Path) -> u32 {
    std::fs::metadata(path).unwrap().permissions().mode() & 0o777
}

#[test]
fn pd_28_fake_tools_that_fail_succeed_and_time_out_leave_their_output_as_evidence() {
    let fx = fixture("evidence");
    let cases = [
        ("fails", "echo marker-out; echo marker-err >&2; exit 3", 30, End::Exited(3), "marker-out\n", "marker-err\n"),
        ("succeeds", "echo marker-done; echo marker-note >&2", 30, End::Exited(0), "marker-done\n", "marker-note\n"),
        (
            "times-out",
            "echo marker-on; echo marker-wait >&2; exec sleep 600",
            1,
            End::TimedOut,
            "marker-on\n",
            "marker-wait\n",
        ),
    ];
    for (name, script, seconds, end, out, err) in cases {
        let (ran, kept, scratch) =
            run(&fx, name, |s| cell(s, Network::None, false, &["/bin/sh", "-c", script]), limits(seconds));
        assert_eq!((ran.end, kept.as_str()), (end, out), "{name}");
        assert_eq!(std::fs::read_to_string(&ran.stderr.path).unwrap(), err, "{name}");
        for k in [&ran.stdout, &ran.stderr] {
            assert_eq!(k.digest, sha256_hex(&std::fs::read(&k.path).unwrap()), "{name}");
            assert_eq!(mode(&k.path), 0o600, "{name}");
            assert!(!k.capped);
        }
        let dir = ran.stdout.path.parent().unwrap();
        assert_eq!((mode(dir), mode(&fx.domain.join("local/evidence"))), (0o700, 0o700));
        if end != End::Exited(0) {
            let Error::Refused { code, path, detail } = ran.failure("fake") else { panic!() };
            assert_eq!((code, path.as_deref()), (Code::ToolFailed, Some(dir)));
            assert!(detail.contains(&dir.display().to_string()) && !detail.contains("marker"), "{detail}");
        }
        scratch.remove().unwrap();
    }
}

#[test]
fn a_channel_past_its_bytes_ends_the_cell_and_keeps_only_the_bytes() {
    let fx = fixture("capped");
    let small = Limits { time: Duration::from_secs(30), bytes: 1000 };
    let (ran, out, scratch) = run(&fx, "capped", |s| cell(s, Network::None, false, &["/usr/bin/yes"]), small);
    assert_eq!((ran.end, ran.stdout.bytes, out.len(), ran.stdout.capped), (End::OutputCapped, 1000, 1000, true));
    assert_eq!(ran.stdout.digest, sha256_hex(out.as_bytes()));
    scratch.remove().unwrap();
}

// ---- REQ-SEC-2, 3.9: the probe and what refuses -----------------------------------------------------------------------

fn detection(verified: Option<&str>, absent: &[&str]) -> Detection {
    let mut found = BTreeMap::new();
    for name in ["bwrap", "pasta"] {
        let path = which(name);
        let at = Found { path, version: Some("v".into()), how: How::Observed, verified: verified.map(String::from) };
        found.insert((Space::Program, name.to_owned()), (!absent.contains(&name)).then_some(at));
    }
    Detection { at: "2026-11-11T00:00:00Z".into(), found }
}

#[test]
fn the_backends_probe_passes_here_and_is_kept_as_verified_by_use() {
    let fx = fixture("probe");
    let mut d = detection(None, &[]);
    let paths = confine::available(Some(&BACKEND), &mut d, &fx.domain).unwrap();
    assert_eq!(paths, programs());
    assert_eq!(paths.get("pasta"), Some(which("pasta").as_path()));
    for p in ["bwrap", "pasta"] {
        assert!(d.get(Space::Program, p).unwrap().verified.is_some(), "{p}");
    }
    // The probe left its evidence and no directory of its run.
    let tmp: Vec<_> = std::fs::read_dir(fx.domain.join("local/tmp")).unwrap().collect();
    assert!(tmp.is_empty());
    assert_eq!(std::fs::read_dir(fx.domain.join("local/evidence")).unwrap().count(), 1);
}

#[test]
fn an_absent_program_refuses_and_a_cell_with_a_network_that_may_not_nest_refuses() {
    let fx = fixture("refusals");
    for absent in ["bwrap", "pasta"] {
        let e = confine::available(Some(&BACKEND), &mut detection(None, &[absent]), &fx.domain).unwrap_err();
        assert_eq!(e.code(), Some(Code::ConfinementUnavailable), "{absent}");
        assert!(e.detail().contains(absent));
    }
    let name = unique("not-nested");
    let scratch = Scratch::create(&fx.domain, &name).unwrap();
    let evidence = confine::evidence(&fx.domain, &name).unwrap();
    let c = cell(&scratch, Network::Outbound, false, &["/usr/bin/true"]);
    let e = confine::run(&BACKEND, &programs(), &c, &scratch, &evidence, limits(30)).unwrap_err();
    assert_eq!(e.code(), Some(Code::ConfinementUnavailable));
    scratch.remove().unwrap();
}

#[test]
fn the_filter_is_fifteen_instructions_for_aarch64() {
    assert_eq!(FILTER.concat().len(), 15 * 8);
    // The architecture's check comes first, and ends any other architecture's process.
    assert_eq!(FILTER[1][4..], 0xc000_00b7u32.to_le_bytes());
    assert_eq!(FILTER[2][4..], 0x8000_0000u32.to_le_bytes());
}
