# Private Remote Workspace Local IPC Contract

Version: `0.1.0`

Status: Phase 006 Ubuntu local IPC baseline

## Scope

This contract defines the future local IPC boundary between same-user Ubuntu PRW client processes, such as the desktop client or local CLI, and the unprivileged headless PRW Agent.

It does not define the separate privileged-helper IPC boundary.

Phase 006 records contracts and typed constants only. It does not create, bind, listen on, connect to, chmod, unlink, or otherwise mutate any socket or filesystem object.

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

The Phase 006 authorization baseline is:

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
6. a future bounded and versioned application protocol.

No single filesystem permission bit is treated as a substitute for application protocol validation.

## Protocol framing

Phase 006 does not select:

- frame format;
- serialization format;
- request/response envelope;
- protocol version negotiation;
- request identifiers;
- maximum message size;
- timeout semantics;
- cancellation semantics;
- capability payload schemas.

Those belong to a subsequent provider-neutral phase.

## Privileged helper separation

Any future privileged helper is a separate trust boundary.

This Desktop/CLI-to-Agent contract must not be interpreted as authorization to expose privileged helper commands directly to local clients or to make the full Agent permanently privileged.

## Forbidden interpretation

Phase 006 does not authorize or implement:

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

The design boundary is based on:

- the XDG Base Directory Specification for `$XDG_RUNTIME_DIR` ownership, mode, lifetime, locality, and runtime-object use;
- Linux `unix(7)` for Unix-domain stream sockets and `SO_PEERCRED` peer credentials.
