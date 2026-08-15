# Phase 006 Ubuntu Local Agent IPC

Status: approved under standing project authorization for build-phase implementation

## Purpose

Lock the minimum secure local IPC boundary between Ubuntu PRW client processes and the unprivileged PRW Agent without activating a listener or introducing a platform binding dependency.

## Decision

The future Ubuntu client-to-Agent local IPC baseline is:

- filesystem-backed Unix-domain stream socket (`AF_UNIX`, `SOCK_STREAM`);
- socket path `$XDG_RUNTIME_DIR/private-remote-workspace/agent.sock`;
- PRW runtime subdirectory mode `0700`;
- socket filesystem mode `0600`;
- Linux kernel peer credentials obtained with `SO_PEERCRED`;
- same-kernel-reported-UID authorization baseline;
- no TCP or loopback-TCP fallback.

## Why XDG runtime storage

The XDG Base Directory Specification defines `$XDG_RUNTIME_DIR` specifically for runtime communication objects such as sockets and requires it to be user-owned with access mode `0700`.

This gives PRW a standard per-user runtime location instead of inventing a fixed `/tmp` pathname or a home-directory socket location with different lifecycle semantics.

The implementation must fail closed if the required runtime-directory security properties cannot be validated. It must not weaken the boundary by falling back to a less protected location or transport.

## Why Unix-domain stream sockets

Linux Unix-domain sockets provide a local-only stream transport and expose peer credentials through `SO_PEERCRED` on connected sockets.

For the desktop/CLI-to-Agent relationship, this permits authorization based on kernel-reported local process credentials rather than introducing a second network authentication protocol merely for same-host IPC.

The filesystem namespace is selected instead of Linux abstract sockets because the pathname also participates in the XDG runtime-directory and filesystem-permission boundary.

## Defense-in-depth model

The intended future runtime sequence is:

1. Resolve and validate `$XDG_RUNTIME_DIR`.
2. Validate runtime-directory ownership and security properties.
3. Create or validate `$XDG_RUNTIME_DIR/private-remote-workspace` as owner-only (`0700`).
4. Safely handle the expected socket pathname without following unsafe substitutions.
5. Bind an `AF_UNIX` `SOCK_STREAM` socket.
6. Enforce the socket filesystem mode `0600` and validate the resulting metadata.
7. Accept a connection.
8. Retrieve `SO_PEERCRED`.
9. Reject the connection unless the kernel-reported UID matches the Agent's expected local UID policy.
10. Only then process the future bounded application protocol.

Phase 006 implements none of these runtime steps. It records the contract required before they can be implemented safely.

## Privilege separation

This contract is for unprivileged client processes talking to the unprivileged Agent.

Any future privileged helper must use a separately reviewed, narrower command boundary. It must not inherit this interface wholesale and must not make the full Agent permanently privileged.

## Locked typed boundary

`prw-agent` exposes provider-neutral types/constants for:

- Unix-domain stream transport;
- Linux `SO_PEERCRED` credential source;
- same-user-ID authorization;
- required runtime directory and socket modes;
- deterministic socket-path construction beneath a supplied XDG runtime directory.

The library performs no socket I/O and creates no filesystem objects.

## Explicit deferrals

Phase 006 does not select or implement:

- Unix socket Rust crate/API bindings;
- actual bind/listen/accept/connect logic;
- stale socket cleanup mechanics;
- symlink/race-resistant filesystem implementation details;
- application framing;
- serialization;
- protocol versioning;
- request IDs;
- maximum frame size;
- timeouts/cancellation;
- privileged-helper IPC;
- systemd unit activation;
- cryptographic provider selection;
- account authentication;
- remote networking.

## Primary references

- XDG Base Directory Specification, `$XDG_RUNTIME_DIR` requirements.
- Linux `unix(7)`, Unix-domain sockets and `SO_PEERCRED`.

## Validation requirements

Phase 006 source changes must continue to pass:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets`
- `cargo build --workspace --all-targets`

Focused tests must prove the baseline transport/credential/authorization values, required permission modes, and deterministic socket path.
