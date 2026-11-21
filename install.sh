#!/bin/sh
# Installs Fabrewk's command, one file: ~/.local/bin/fabrewk. POSIX sh; read it before you run it.
#
#   curl --proto '=https' --tlsv1.2 -fsSL https://raw.githubusercontent.com/fabrewk/fabrewk/v0.1.0/install.sh | sh
#
# It installs the version written below and nothing else. It downloads that version's archive, its checksum file
# and the checksum file's signature; it verifies the signature with `ssh-keygen -Y verify` against the signers
# written below, then the archive's checksum, and on any failure installs nothing. It takes no option, so none skips
# the check, and it never uses sudo. Trusting this release's source and its signers grants them full authority over
# what this script installs. Inside a Git repository with a terminal, it offers `fabrewk onboard`, which writes
# nothing before your yes. To remove Fabrewk, delete ~/.local/bin/fabrewk.
#
# Fabrewk is licensed under MIT OR Apache-2.0.

set -eu

version=0.1.0
release_url=https://github.com/fabrewk/fabrewk/releases/download/v0.1.0
allowed_signers='xel namespaces="fabrewk-release" ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMpNfw3yY4sb7CO5xUCEfRQuPtw6UlYial7Y/MWCHFVf xel@fabrewk'
namespace=fabrewk-release
supported=linux-aarch64
newline='
'

say() {
    printf 'fabrewk install: %s\n' "$*"
}

refuse() {
    printf '%s\n' "$*" >&2
    exit 1
}

need() {
    command -v "$1" >/dev/null 2>&1 ||
        refuse "TOOL_MISSING: $1 is not installed; install it with your system's package manager ($2), then run this script again. It installs no tool by itself."
}

main() {
    if [ "$#" -ne 0 ]; then
        printf 'usage: sh install.sh\nIt takes no option; the version, its source and its signers are written in it.\n' >&2
        exit 2
    fi
    os=$(uname -s)
    arch=$(uname -m)
    case "$os $arch" in
        'Linux aarch64' | 'Linux arm64') platform=linux-aarch64 ;;
        *) refuse "PLATFORM_UNSUPPORTED: $os on $arch is not supported yet; supported: $supported." ;;
    esac
    need curl 'the package curl'
    need ssh-keygen 'OpenSSH, the package openssh or openssh-client'
    need sha256sum 'coreutils'
    need tar 'the package tar'
    need gzip 'the package gzip'
    [ -n "${HOME:-}" ] && [ -d "$HOME" ] || refuse "HOME is not a directory, so there is no ~/.local/bin to install into."

    archive="fabrewk-$version-$platform.tar.gz"
    bindir="$HOME/.local/bin"
    # The traps come before the first temporary file, so that a signal or an output closed under it leaves nothing.
    tmp=
    new=
    trap '[ -z "$tmp" ] || rm -rf "$tmp"; [ -z "$new" ] || rm -f "$new"' EXIT
    trap 'exit 1' HUP INT PIPE TERM
    tmp=$(mktemp -d "${TMPDIR:-/tmp}/fabrewk-install.XXXXXX")

    say "downloading Fabrewk $version for $platform from $release_url"
    proto=${release_url%%://*}
    for file in "$archive" SHA256SUMS SHA256SUMS.sig; do
        curl --proto "=$proto" --proto-redir "=$proto" --tlsv1.2 --fail --silent --show-error --location \
            --output "$tmp/$file" "$release_url/$file" ||
            refuse "the download of $release_url/$file failed; nothing was installed."
    done

    printf '%s\n' "$allowed_signers" >"$tmp/allowed_signers"
    signer=
    while read -r principal _; do
        if ssh-keygen -Y verify -f "$tmp/allowed_signers" -I "$principal" -n "$namespace" -s "$tmp/SHA256SUMS.sig" \
            <"$tmp/SHA256SUMS" >/dev/null 2>&1; then
            signer=$principal
            break
        fi
    done <"$tmp/allowed_signers"
    [ -n "$signer" ] ||
        refuse "the signature of SHA256SUMS is not valid for any signer this script names; nothing was installed."
    say "the signature of SHA256SUMS is $signer's"

    expected=$(awk -v f="$archive" '$2 == f || $2 == "*" f { n++; h = $1 } END { if (n == 1) print h }' \
        "$tmp/SHA256SUMS")
    actual=$(sha256sum <"$tmp/$archive")
    actual=${actual%% *}
    case "$expected" in
        '' | *[!0-9a-f]*) refuse "SHA256SUMS does not list $archive exactly once; nothing was installed." ;;
    esac
    [ "$expected" = "$actual" ] ||
        refuse "the checksum of $archive is not the one SHA256SUMS lists; nothing was installed."
    say "the checksum of $archive matches"

    # Inside the temporary directory, by relative names: GNU tar reads escapes in the names it is given (P10D-5).
    (cd "$tmp" && tar -xzf "$archive" "fabrewk-$version-$platform/fabrewk") ||
        refuse "the archive holds no program fabrewk; nothing was installed."
    binary="$tmp/fabrewk-$version-$platform/fabrewk"
    [ -f "$binary" ] && [ ! -L "$binary" ] || refuse "the archive holds no program fabrewk; nothing was installed."
    mkdir -p "$bindir"
    [ ! -d "$bindir/fabrewk" ] || refuse "$bindir/fabrewk is a directory; nothing was installed."
    new=$(mktemp "$bindir/.fabrewk.XXXXXX")
    cp "$binary" "$new"
    chmod 755 "$new"
    mv -f "$new" "$bindir/fabrewk"
    new=
    rm -rf "$tmp"
    said=$("$bindir/fabrewk" --version) ||
        refuse "$bindir/fabrewk is installed but does not run on this machine; delete it to remove it."
    say "installed $bindir/fabrewk: $said"

    case ":${PATH:-}:" in
        *":$bindir:"*) ;;
        *)
            say "$bindir is not on your PATH; add it in fish with:"
            say "  fish_add_path ~/.local/bin"
            say "or in a POSIX shell, in ~/.profile, with:"
            say '  export PATH="$HOME/.local/bin:$PATH"'
            ;;
    esac

    agent=
    for marker in CLAUDECODE CODEX_SANDBOX CODEX_SANDBOX_NETWORK_DISABLED CODEX_THREAD_ID FABREWK_EXTENSION; do
        eval "set_=\${$marker+set}"
        [ -z "$set_" ] || agent=$marker
    done
    if [ -n "$agent" ]; then
        say "$agent is set, so this runs inside an agent: the next step is fabrewk assess with the maintainer's answers."
    elif command -v git >/dev/null 2>&1 && [ "$(git rev-parse --is-inside-work-tree 2>/dev/null)" = true ] &&
        (exec </dev/tty >/dev/tty) 2>/dev/null; then
        # The top as Git prints it, its own trailing line ends kept: only the one Git adds is removed.
        top=$(git rev-parse --show-toplevel && echo x) || refuse "git could not name the repository's top."
        top=${top%x}
        top=${top%"$newline"}
        printf 'Run fabrewk onboard in %s now? It asks its questions here and writes nothing before your yes. [y/N] ' \
            "$top" >/dev/tty
        answer=
        read -r answer </dev/tty || answer=
        case "$answer" in
            y | Y | yes | Yes | YES) cd "$top" && exec "$bindir/fabrewk" onboard ;;
        esac
        say "when you want it, run fabrewk onboard inside the repository."
    else
        say "inside a Git repository, run fabrewk onboard to bring Fabrewk into it."
    fi
}

main "$@"
