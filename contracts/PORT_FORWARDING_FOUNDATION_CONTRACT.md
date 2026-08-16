# Private Remote Workspace Port Forwarding Foundation Contract

Version: `0.1.0`

Status: Phase 134 implementation lock

## Purpose

Phase 134 establishes the typed, bounded lifecycle and backend boundary for PRW TCP port forwarding.

This phase does **not** open a real listener, accept a socket, connect to a target, expose a public port, mutate a firewall, configure a router, create a tunnel, or activate production forwarding.

## Security separation

A port-forward session is distinct from:

- the authenticated PRW device session;
- current workspace/device registry state;
- terminal authority;
- file authority;
- control-transport connection identity;
- future private-mesh transport identity.

Opening a real forward must eventually require current registry revalidation plus an explicit forwarding capability decision. Workspace role metadata alone does not grant forwarding access.

## Initial forwarding mode

Phase 134 models **TCP only**.

The initial bind side is deliberately restricted to loopback-only named families:

- IPv4 loopback (`127.0.0.1` semantics);
- IPv6 loopback (`::1` semantics).

The domain does not accept a caller-supplied bind IP address. Wildcard/public/LAN bind addresses are not representable in the Phase 134 API.

UDP forwarding, SOCKS, transparent proxying, reverse/public listeners, Unix-domain forwarding and arbitrary socket options are deferred.

## Forward identifier

`PortForwardId` is a non-zero unsigned 64-bit broker-scoped identifier.

It is not a user, device, authenticated-session, network or transport identity.

## Bind endpoint

A loopback bind request consists only of:

- `LoopbackFamily::{Ipv4, Ipv6}`;
- one explicit non-zero TCP port.

Port zero is rejected so Phase 134 does not introduce implicit dynamic allocation semantics.

## Target endpoint

A target consists only of:

- one explicit IP address;
- one explicit non-zero TCP port.

No hostname or resolver input is accepted. Phase 134 therefore has no DNS dependency.

Unspecified and multicast target addresses are rejected. IPv4 limited broadcast is rejected.

Loopback targets are allowed because forwarding to an authenticated host-local service is a valid product use case.

## Session bounds

One broker may track at most 32 active/failed forwarding records.

Duplicate `PortForwardId` values fail before backend mutation.

## Lifecycle

Initial states:

- `Opening`;
- `Active`;
- `Closing`;
- `Closed`;
- `Failed`.

A successful backend open produces `Active`.

Backend open failure produces no tracked session. Backend close failure retains the tracked record as `Failed`, preventing silent reuse. Successful close returns a terminal `Closed` record and removes it from the broker.

## Backend boundary

The provider-neutral `PortForwardBackend` accepts only:

- one already-validated `TcpForwardSpec`;
- one backend-owned handle for close.

It does not accept raw shell commands, executable paths, environment variables, firewall instructions, interface names, DNS names, arbitrary bind addresses, arbitrary socket options or privilege-escalation instructions.

A future Linux/network adapter owns real socket creation and must preserve this boundary.

## Identity binding

Every forwarding record immutably snapshots:

- `WorkspaceId`;
- `UserId`;
- `DeviceId`;
- authenticated PRW `SessionId`.

The snapshot is derived from a current `RegistryValidatedPrincipal` boundary plus the authenticated session identifier. The forwarding broker does not mutate identity after open.

## Required tests

Tests must prove at least:

- zero forwarding identifier rejected;
- zero bind port rejected;
- zero target port rejected;
- unspecified/multicast/IPv4 broadcast targets rejected;
- loopback targets accepted;
- only named loopback bind families are representable;
- duplicate identifiers rejected before backend call;
- 32-session capacity enforced before backend call;
- validated bind/target spec is passed unchanged to backend;
- backend open failure creates no tracked session;
- successful close returns `Closed` and removes the record;
- backend close failure retains a `Failed` record;
- immutable workspace/user/device/authenticated-session identity is preserved;
- no API accepts a caller-supplied hostname, raw command, arbitrary bind address or socket-option bag.

## Explicitly deferred

- real TCP listener/accept/connect implementation;
- byte pumping/backpressure/half-close handling;
- per-connection limits/timeouts/accounting;
- production forwarding capability policy;
- authenticated remote protocol framing;
- public/LAN bind support;
- reverse/public forwarding;
- UDP;
- SOCKS;
- DNS targets;
- Android/Desktop forwarding UI;
- private-mesh path selection (Phase 135);
- relay transport integration (Phase 136).

## Production boundary

Phase 134 source/disposable work may proceed under the user's authorization through Phase 137.

No production port forwarding becomes active until an audited real socket adapter, explicit capability policy, authenticated transport/session integration, current registry revalidation and deployed Agent validation are complete.
