# Private Remote Workspace Linux Local Peer Identity Adapter Contract

Version: `0.1.0`

Status: Phase 057 non-activating Linux identity adapter

## Scope

Phase 057 introduces the first PRW source that calls the Phase 054/055 selected rustix provider. It is limited to read-only Linux identity operations over an already-existing file descriptor.

## Agent identity

The Agent-side UID used for the local same-user security boundary is the process **effective UID** returned by `rustix::process::geteuid()`.

Using the effective UID aligns the comparison with the identity under which the Agent is actually executing access-controlled operations.

## Peer identity

For an already-connected Unix-domain socket file descriptor, peer identity is obtained only through Linux kernel `SO_PEERCRED` via `rustix::net::sockopt::socket_peercred`.

The adapter converts the returned rustix `UCred` into a PRW-local typed record containing:

- peer PID;
- peer UID;
- peer GID.

No user-controlled protocol field may substitute for these kernel credentials.

## Error bounding

A peer-credential syscall failure is mapped to a bounded PRW-local error classification. Raw operating-system error strings are not exposed through the application protocol.

## Authorization boundary

Phase 057 only retrieves identity information. It does **not** authorize the peer and does not read application-protocol bytes.

A later phase must compose:

`peer credentials -> same-effective-UID decision -> only then application Request processing`.

## Test boundary

Linux tests may use `std::os::unix::net::UnixStream::pair()` solely to supply already-connected anonymous Unix-domain file descriptors for `SO_PEERCRED` validation.

Tests must not:

- bind a filesystem pathname;
- create a listener;
- accept network connections;
- create/remove XDG runtime paths.

## Forbidden interpretation

Phase 057 does not authorize or implement:

- filesystem-backed Unix socket bind/listen/accept/connect;
- XDG runtime-directory mutation;
- stale-socket unlink;
- peer authorization or policy-context construction;
- protocol reads/writes from the tested socket pair;
- service activation;
- network/DNS/TUN mutation;
- database changes;
- private-key operations;
- deployment.
