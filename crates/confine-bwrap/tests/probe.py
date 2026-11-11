"""The probe of REQ-SEC-1 (part 9a), run as a confined process would run, and its fixtures on the host.

`probe.py cell <file>` runs inside a cell, reads its expectations from the file, and prints one JSON object, `{"failures": [...]}`: every expectation of the
argument that does not hold. `probe.py plant <description>` adds a key to the host's session keyring and one of
default permissions to its user keyring, and prints their serial numbers; `probe.py remove <description> <serial>...`
invalidates them. The system call numbers are aarch64's, the one architecture the backend has a filter for.
"""

import ctypes
import json
import os
import socket
import subprocess
import sys

KEYCTL, ADD_KEY, REQUEST_KEY, IO_URING_SETUP = 219, 217, 218, 425
KEYCTL_LINK, KEYCTL_SEARCH, KEYCTL_READ, KEYCTL_INVALIDATE = 8, 10, 11, 21
SESSION, USER = -3, -4
EPERM = 1

libc = ctypes.CDLL(None, use_errno=True)
libc.syscall.restype = ctypes.c_long


def call(number, *args):
    """A system call's result and errno."""
    ctypes.set_errno(0)
    result = libc.syscall(ctypes.c_long(number), *[ctypes.c_long(a) if isinstance(a, int) else a for a in args])
    return result, ctypes.get_errno()


def plant(description):
    serials = []
    for ring in (SESSION, USER):
        serial, errno = call(ADD_KEY, b"user", description.encode() + (b"-s" if ring == SESSION else b"-u"), b"planted", 7, ring)
        if serial < 0:
            sys.exit(f"add_key failed: errno {errno}")
        buffer = ctypes.create_string_buffer(64)
        length, errno = call(KEYCTL, KEYCTL_READ, serial, buffer, 64)
        if length != 7:
            sys.exit(f"a planted key cannot be read on the host: errno {errno}")
        serials.append(serial)
    print(" ".join(map(str, serials)))


def remove(description, serials):
    """Invalidates the planted keys, and the one a cell that was not refused would have made."""
    made, _ = call(KEYCTL, KEYCTL_SEARCH, SESSION, b"user", description.encode() + b"-made", 0)
    for serial in [int(s) for s in serials] + ([made] if made > 0 else []):
        call(KEYCTL, KEYCTL_INVALIDATE, serial)


def refused(failures, what, result):
    """Expects a call refused by the filter: -1 with EPERM."""
    value, errno = result
    if value != -1 or errno != EPERM:
        failures.append(f"{what}: not refused by the filter (result {value}, errno {errno})")


def keys(failures, serials, description, where):
    buffer = ctypes.create_string_buffer(64)
    for serial in serials:
        refused(failures, f"{where}: keyctl read {serial}", call(KEYCTL, KEYCTL_READ, serial, buffer, 64))
        refused(failures, f"{where}: keyctl link {serial}", call(KEYCTL, KEYCTL_LINK, serial, SESSION))
    for suffix in (b"-s", b"-u"):
        found = call(KEYCTL, KEYCTL_SEARCH, SESSION, b"user", description.encode() + suffix, 0)
        refused(failures, f"{where}: keyctl search", found)
        refused(failures, f"{where}: request_key", call(REQUEST_KEY, b"user", description.encode() + suffix, None, 0))
    made = description.encode() + b"-made"
    refused(failures, f"{where}: add_key", call(ADD_KEY, b"user", made, b"x", 1, SESSION))
    params = ctypes.create_string_buffer(256)
    refused(failures, f"{where}: io_uring_setup", call(IO_URING_SETUP, 1, params))
    try:
        socket.socket(38, socket.SOCK_SEQPACKET, 0).close()
        failures.append(f"{where}: a cryptographic socket opened")
    except OSError as e:
        if e.errno != EPERM:
            failures.append(f"{where}: a cryptographic socket failed with errno {e.errno}, not the filter's")
    for family in (socket.AF_INET, socket.AF_UNIX):
        try:
            socket.socket(family, socket.SOCK_STREAM).close()
        except OSError as e:
            failures.append(f"{where}: a socket of family {family} failed: {e}")


def routed():
    with open("/proc/net/route") as f:
        v4 = any(line.split()[0] != "lo" and line.split()[1] == "00000000" for line in f.readlines()[1:])
    with open("/proc/net/ipv6_route") as f:
        v6 = any(line.split()[0] == "0" * 32 and line.split()[1] == "00" and line.split()[9] != "lo" for line in f)
    return v4 or v6


def connects(target):
    if "abstract" in target:
        s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        address = "\0" + target["abstract"]
    else:
        host, port = target["tcp"]
        s = socket.socket(socket.AF_INET6 if ":" in host else socket.AF_INET, socket.SOCK_STREAM)
        address = (host, port)
    s.settimeout(3)
    try:
        s.connect(address)
        return True
    except OSError:
        return False
    finally:
        s.close()


def cell(spec):
    failures = []
    for path in spec.get("absent", []):
        if os.path.lexists(path):
            failures.append(f"{path} exists inside")
    for path in spec.get("present", []):
        if not os.path.exists(path):
            failures.append(f"{path} is missing inside")
    for path in spec.get("readable", []):
        try:
            open(path, "rb").read()
        except OSError as e:
            failures.append(f"{path} cannot be read: {e}")
    for path in spec.get("writable", []):
        try:
            with open(path, "w") as f:
                f.write("written\n")
        except OSError as e:
            failures.append(f"{path} cannot be written: {e}")
    for path in spec.get("unwritable", []):
        try:
            with open(path, "w") as f:
                f.write("written\n")
            failures.append(f"{path} was written")
        except OSError:
            pass
    for path in spec.get("unmakeable", []):
        try:
            os.mkdir(path)
            failures.append(f"{path} was made")
        except OSError:
            pass
    if "listing" in spec:
        for path, wanted in spec["listing"].items():
            got = sorted(os.listdir(path))
            if got != sorted(wanted):
                failures.append(f"{path} holds {got}, not {sorted(wanted)}")
    if "planted" in spec:
        name, value = spec["planted"]
        if name in os.environ:
            failures.append(f"the variable {name} is inside")
        for proc in ("/proc/1/cmdline", "/proc/self/environ"):
            if value.encode() in open(proc, "rb").read():
                failures.append(f"{proc} holds the planted value")
        if sorted(os.environ) != sorted(spec["environment"]):
            failures.append(f"the environment is {sorted(os.environ)}")
    status = dict(line.split(":", 1) for line in open("/proc/self/status").read().splitlines() if ":" in line)
    for field, wanted in (("NoNewPrivs", "1"), ("CapEff", "0000000000000000"), ("Seccomp", "2")):
        if status.get(field, "").strip() != wanted:
            failures.append(f"{field} is {status.get(field, '').strip()}, not {wanted}")
    processes = [p for p in os.listdir("/proc") if p.isdigit()]
    if len(processes) > 4:
        failures.append(f"{len(processes)} processes are visible")
    if os.getuid() != spec["uid"]:
        failures.append(f"the user is {os.getuid()}, not the person's {spec['uid']}")
    if "routed" in spec and routed() != spec["routed"]:
        failures.append(f"a default route {'is missing' if spec['routed'] else 'exists'}")
    for target in spec.get("unreachable", []):
        if connects(target):
            failures.append(f"{target} answered")
    if "keys" in spec:
        serials, description = spec["keys"]
        keys(failures, serials, description, "the cell")
        if spec.get("nested"):
            nested = subprocess.run(
                ["bwrap", "--unshare-all", "--ro-bind", "/usr", "/usr", "--symlink", "usr/bin", "/bin",
                 "--symlink", "usr/lib", "/lib", "--symlink", "usr/lib64", "/lib64", "--proc", "/proc",
                 "--dev", "/dev", "--ro-bind", sys.argv[0], "/probe.py",
                 "python3", "/probe.py", "nested", json.dumps(spec["keys"])],
                capture_output=True, text=True)
            try:
                failures += json.loads(nested.stdout)["failures"]
            except ValueError:
                failures.append(f"the nested sandbox did not run: {nested.stderr.strip()}")
    print(json.dumps({"failures": failures}))


if __name__ == "__main__":
    mode = sys.argv[1]
    if mode == "plant":
        plant(sys.argv[2])
    elif mode == "remove":
        remove(sys.argv[2], sys.argv[3:])
    elif mode == "nested":
        found = []
        serials, description = json.loads(sys.argv[2])
        keys(found, serials, description, "a sandbox nested in the cell")
        print(json.dumps({"failures": found}))
    else:
        with open(sys.argv[2]) as f:
            cell(json.load(f))
