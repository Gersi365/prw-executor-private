# Private Remote Workspace Local IPC Contract

Version: `0.2.0`

Status: Phase 007 Ubuntu local IPC framing baseline

## Scope

This contract defines the future local IPC boundary between same-user Ubuntu PRW client processes, such as the desktop client or local CLI, and the unprivileged headless PRW Agent.

It does not define the separate privileged-helper IPC boundary.

Phase 006 locked the transport and local peer-authentication boundary. Phase 007 adds bounded stream framing and version metadata. Neither phase activates the socket runtime.

## Transport

The Ubuntu client-to-Agent baseline uses:

- address family: `AF_UNIX`;
- socket type: `SOCK_STREAM`;
- namespace: filesystem pathname;
- endpoint: `$XDG_RUNTIME_DIR/private-remote-workspace/agent.sock`.

The baseline does not use:

- TCP;
- loopback TCP;
- a public network listener;
- Linux abstract Unix-domain socket namespace;
- `/tmp` as a fallback socket directory.

There is no TCP fallback for Desktop/CLI-to-Agent local IPC.

## XDG runtime directory boundary

A future runtime implementation must treat `$XDG_RUNTIME_DIR` as a security boundary rather than merely a convenient path.

Before creating the PRW runtime subdirectory or socket, it must validate the required runtime-directory properties, including:

- the variable exists;
- the path is absolute;
- the runtime directory belongs to the current user;
- the runtime directory is not accessible by other users under the XDG runtime-directory security model;
- the environment is suitable for Unix-domain runtime sockets.

The XDG Base Directory Specification requires `$XDG_RUNTIME_DIR` to be owned by the user and to have Unix access mode `0700`.

If the required security properties cannot be established, local PRW IPC must fail closed. The implementation must not silently fall back to `/tmp`, a world-accessible directory, TCP, or another weaker endpoint.

## PRW-owned runtime path

The future implementation must create the application runtime subdirectory as:

`$XDG_RUNTIME_DIR/private-remote-workspace`

Required mode:

`0700`

The future Agent socket pathname is:

`$XDG_RUNTIME_DIR/private-remote-workspace/agent.sock`

Required filesystem mode after binding:

`0600`

The runtime implementation must validate actual ownership and permissions rather than assuming that requested creation modes were preserved.

## Local peer authentication

Filesystem permissions alone are not sufficient as the entire peer-authentication mechanism.

After accepting a connected Unix-domain stream socket, the future Linux implementation must obtain kernel-reported peer credentials using `SO_PEERCRED`.

The authorization baseline is:

- the peer kernel-reported UID must equal the Agent process UID expected by the local security policy;
- otherwise the connection is rejected before processing an application request.

The contract deliberately says kernel-reported UID. It does not redefine Linux credential semantics in application-specific terms.

## Defense in depth

The local boundary combines:

1. XDG runtime-directory isolation;
2. PRW runtime-subdirectory ownership/mode;
3. socket filesystem ownership/mode;
4. kernel-backed peer credentials;
5. same-UID authorization;
6. bounded and versioned application framing;
7. a future typed command/response payload protocol.

No single filesystem permission bit is treated as a substitute for application protocol validation.

## Stream framing

The Phase 007 local IPC stream is split into frames with a fixed 24-byte header followed by an opaque payload.

All multi-byte integer fields are unsigned and encoded in network byte order (big-endian).

Fixed header layout:

| Offset | Size | Field |
| --- | ---: | --- |
| 0 | 4 | magic bytes `PRW\0` |
| 4 | 2 | protocol major version |
| 6 | 2 | protocol minor version |
| 8 | 1 | message kind |
| 9 | 1 | flags, must be zero in version 1.0 |
| 10 | 2 | reserved, must be zero in version 1.0 |
| 12 | 8 | non-zero request ID |
| 20 | 4 | payload length in bytes |

The fixed header is exactly 24 bytes.

The payload begins immediately after the fixed header and contains exactly the declared number of bytes.

The future decoder must reject a frame before payload processing if:

- the magic bytes do not match;
- the protocol version is unsupported;
- the message-kind value is unknown;
- flags or reserved fields are non-zero when not defined by the active protocol version;
- request ID is zero;
- payload length exceeds the configured protocol maximum;
- EOF occurs before the full declared header or payload is received.

Phase 007 records this wire layout but does not implement byte encoding/decoding or socket reads/writes.

## Protocol version

The initial local IPC protocol version is:

- major: `1`;
- minor: `0`.

The current implementation contract accepts exactly version `1.0`.

A later phase may define compatibility rules for newer minor versions. Until those rules are explicitly locked, clients and the Agent must fail closed on any other version rather than guessing compatibility.

## Message kinds

Version 1.0 reserves these message-kind codes:

- `1` — Request;
- `2` — Response;
- `3` — Error.

Every Response or Error must carry the same non-zero request ID as the Request to which it corresponds.

Unsolicited event/notification messages are not part of the Phase 007 baseline.

## Request identifiers

Request ID is an unsigned 64-bit integer.

Rules:

- value `0` is reserved and invalid;
- the client assigns request IDs;
- a request ID correlates one Request with its Response or Error;
- a client must not have two simultaneously outstanding requests with the same request ID on one connection;
- request IDs are correlation values, not authentication tokens, capabilities, secrets, or persistent object identifiers.

## Payload bound

Maximum Phase 007 payload length:

`1,048,576 bytes` (1 MiB)

The limit applies to the opaque payload only; total maximum frame length is the fixed 24-byte header plus this payload.

The local IPC channel is a bounded control channel. Large file contents and bulk transfer data must not be tunneled through this control frame merely to bypass the dedicated file/transfer architecture.

A future phase may define smaller per-command limits beneath this global ceiling.

## Payload serialization

Phase 007 deliberately leaves payload bytes opaque.

It does not select:

- JSON;
- CBOR;
- MessagePack;
- Protocol Buffers;
- another schema/serialization format;
- command IDs or schemas;
- error-code schemas.

The next typed protocol layer must fit within the framing contract and remain bounded.

## Privileged helper separation

Any future privileged helper is a separate trust boundary.

This Desktop/CLI-to-Agent contract must not be interpreted as authorization to expose privileged helper commands directly to local clients or to make the full Agent permanently privileged.

## Forbidden interpretation

Phase 007 does not authorize or implement:

- arbitrary shell execution;
- privileged host-management commands;
- production network listeners;
- SSH listeners;
- account authentication;
- cryptographic private-key operations;
- DNS mutation;
- TUN/WireGuard mutation;
- systemd activation;
- database changes;
- deployment.

## Primary platform references

The transport/security boundary remains based on:

- the XDG Base Directory Specification for `$XDG_RUNTIME_DIR` ownership, mode, lifetime, locality, and runtime-object use;
- Linux `unix(7)` for Unix-domain stream sockets and `SO_PEERCRED` peer credentials.

The Phase 007 frame format itself is a PRW application-protocol contract. It is not a cryptographic primitive and provides no authentication or confidentiality beyond the local transport/security boundary defined above.
