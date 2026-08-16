# Private Remote Workspace Relay Fallback Foundation Contract

Version: `0.1.0`

Status: Phase 136 implementation lock

## Purpose

Phase 136 establishes a provider-neutral, bounded relay-fallback session and opaque-frame boundary.

It does **not** deploy a relay server, create a socket, contact a relay, decrypt application traffic, terminate end-to-end encryption, configure public networking, mutate routes/firewalls or activate production relay traffic.

## Fallback-only rule

Relay is fallback, not the preferred path.

A Phase 136 relay session specification may be created only from a Phase 135 `SelectedConnectivityPath::Candidate` whose `ConnectivityPathKind` is exactly `Relay`.

`LocalDirect`, `InternetDirect` and `Offline` selection results cannot create a relay-session specification.

This preserves the product ordering:

`LocalDirect -> InternetDirect -> Relay -> Offline`.

## Identity separation

The relay session records both:

- logical `DeviceId`;
- distinct opaque `TransportIdentity` from Phase 135.

Neither value is a relay encryption key.

## Relay route token

`RelayRouteToken` is exactly 32 opaque bytes and must not be all zero.

It is a bounded routing/session token, not a private device identity key, transport private key or cryptographic primitive.

Phase 136 does not derive, encrypt, sign or decrypt this token.

## Relay endpoint

The relay endpoint is the explicit-IP/non-zero-port `ConnectivityEndpoint` selected by Phase 135.

No hostname or DNS resolver input is added by Phase 136.

## Opaque frame boundary

`OpaqueRelayFrame` contains an opaque byte vector only.

Initial bounds:

- minimum payload: 1 byte;
- maximum one frame: 64 KiB.

The relay abstraction exposes no method to parse terminal/file/forwarding payload semantics and no method to decrypt application content.

The caller is responsible for supplying application data already protected by the audited end-to-end transport layer. The relay provider receives only the opaque frame plus bounded routing/session metadata.

Phase 136 does not claim that arbitrary bytes are cryptographically valid ciphertext; it enforces architectural opacity, not cryptographic verification.

## Session bounds and lifecycle

- non-zero `RelaySessionId`;
- maximum 32 tracked relay sessions per broker;
- duplicate session identifiers fail before backend mutation.

Lifecycle:

- `Opening`;
- `Active`;
- `Closing`;
- `Closed`;
- `Failed`.

Backend open failure creates no tracked session. Backend transmit failure retains the record as `Failed`. Backend close failure retains the record as `Failed`. Successful close returns a terminal `Closed` record and removes it from the broker.

## Backend boundary

The provider-neutral `RelayBackend` accepts only:

- one validated `RelaySessionSpec`;
- one backend-owned handle;
- one already-bounded `OpaqueRelayFrame`;
- close of the same handle.

It accepts no plaintext-specific command, no private key, no DNS name, no raw shell command, no firewall/TUN instruction and no arbitrary socket-option bag.

## Required tests

Tests must prove at least:

- zero relay session identifier rejected;
- all-zero route token rejected;
- empty and oversized opaque frames rejected;
- direct/offline selected paths cannot construct relay specs;
- selected relay candidate constructs the exact relay spec;
- logical device and transport identity remain distinct;
- duplicate session identifiers rejected before backend open;
- 32-session capacity enforced before backend open;
- backend open failure creates no record;
- exact opaque frame bytes reach backend unchanged;
- backend transmit failure makes the session `Failed`;
- successful close returns `Closed` and removes the session;
- backend close failure retains `Failed`;
- no API parses/decrypts application payload or accepts a private key/hostname/raw command.

## Explicitly deferred

- relay server implementation/deployment;
- relay authentication service;
- real relay dialing and sockets;
- production route-token issuance;
- bandwidth accounting/rate limiting;
- multi-region relay selection;
- reconnect/migration;
- application transport encryption integration;
- production capability policy;
- optional private DNS (Phase 137).

## Production boundary

Phase 136 source/disposable work may proceed under the user's authorization through Phase 137.

No production relay server, connection, route token, public endpoint, firewall rule or user-impacting networking change is authorized by this contract.
