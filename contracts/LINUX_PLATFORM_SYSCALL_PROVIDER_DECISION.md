# Private Remote Workspace Linux Platform Syscall Provider Decision

Version: `0.1.0`

Status: Phase 054 decision lock — dependency not yet materialized

## Decision

For the first Linux local-IPC runtime security adapter, PRW selects:

`rustix = 1.1.4`

The intended initial Cargo dependency shape is exact-version and minimal-feature scoped:

`rustix = { version = "=1.1.4", default-features = false, features = ["std", "fs", "net", "process"] }`

This contract does not itself add the dependency.

## Why rustix

The selected version exposes safe, typed APIs required by the locked Local IPC contract:

- `rustix::net::sockopt::socket_peercred` for Linux `SO_PEERCRED`;
- `rustix::net::UCred` with typed peer `pid`, `uid`, and `gid`;
- `rustix::process` UID APIs for the Agent process identity boundary;
- `rustix::fs::openat` and `OFlags::NOFOLLOW | OFlags::DIRECTORY | OFlags::CLOEXEC` for descriptor-anchored directory handling;
- `rustix::fs::statat` and `AtFlags::SYMLINK_NOFOLLOW` for final-component metadata checks without following symlinks;
- `rustix::fs::unlinkat` for directory-FD-relative removal;
- `rustix::fs::mkdirat` for descriptor-relative directory creation;
- `rustix::fs::chmodat` / `fchmod` for explicit permission handling and verification support.

The crate uses Rust I/O-safety abstractions such as `AsFd`/`OwnedFd`, so PRW can keep `unsafe_code = "forbid"` in its own workspace source.

## Feature lock

Only the following rustix features are authorized for the initial local IPC platform layer:

- `std`;
- `fs`;
- `net`;
- `process`.

`default-features = false` is required so later rustix default-feature changes cannot silently expand PRW's dependency surface.

The following are not authorized by this decision:

- `all-apis`;
- `use-libc`;
- `linux_latest` or other minimum-kernel optimization features;
- `event`, `io_uring`, `mount`, `pty`, `thread`, or unrelated modules.

A later phase must explicitly justify any feature expansion.

## Backend policy

Do not force the optional `use-libc` feature. On supported Linux targets, rustix may use its normal platform-selected backend. PRW code must depend only on the public safe rustix API and must not couple itself to backend internals.

## Version policy

The first materialization must use exact version `1.1.4` and commit the resulting Cargo lockfile update. Version changes require a later explicit dependency-review phase and full workspace validation.

## nix comparison

`nix 0.31.3` was reviewed as the principal alternative. It provides usable socket peer-credential, filesystem `*at`, and UID APIs, but its required `socket` + `fs` + `user` feature combination and dependency shape do not provide a compelling advantage over the selected rustix API for this narrow Linux security adapter.

This is not a claim that nix is unsafe; it is a scope/minimality decision for PRW.

## Runtime boundary

This decision does not authorize:

- dependency materialization without lockfile evidence;
- Unix socket bind/listen/accept/connect;
- filesystem path creation/removal;
- stale-socket unlink;
- peer credential lookup in production runtime;
- service activation;
- authentication/database/network/DNS/TUN mutation;
- deployment.

The next phase may materialize the exact dependency and validate a non-activating platform abstraction, but real listener/path mutation remains blocked until the pathname-lifecycle algorithm is separately locked.

## Primary evidence reviewed — 2026-08-15

- rustix 1.1.4 crate documentation and feature manifest on docs.rs;
- rustix 1.1.4 `net::sockopt::socket_peercred` and `UCred` documentation;
- rustix 1.1.4 `fs` module documentation for `openat`, `statat`, `unlinkat`, `mkdirat`, `chmodat`, `OFlags`, and `AtFlags`;
- nix 0.31.3 crate feature/Cargo manifest and `fs`/socket/user API documentation on docs.rs.
