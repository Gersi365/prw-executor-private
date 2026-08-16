# Private Remote Workspace — Disposable Opaque Relay Provider Contract

Status: Phase 142 source/disposable implementation lock
Date: 2026-08-16
Repository: `Gersi365/prw-executor-private`
Input baseline: `112a5997953580ef0c7630307ab69c17b3e22325`
Parent relay foundation: `contracts/RELAY_FALLBACK_FOUNDATION_CONTRACT.md`
Parent transport architecture: `contracts/REMOTE_TRANSPORT_ARCHITECTURE_DECISION.md`
Parent traversal validation: `contracts/NAT_TRAVERSAL_CONNECTIVITY_CHECKS_CONTRACT.md`

## Purpose

Phase 142 implements a disposable PRW relay protocol/service/provider around the already validated Phase 136 fallback-only relay abstraction. The relay carries bounded opaque bytes only. Those bytes are the same end-to-end protected QUIC/TLS mesh payloads used on a direct path; the relay does not terminate PRW mesh TLS, parse PRW application messages, authenticate a user/device session, or grant a capability.

This phase remains source/disposable only. It does not deploy a production relay, bind a production listener, create persistent TURN traffic, alter firewall/NAT/router/TUN/TAP/routes/DNS, provision production credentials, replace/restart the production Agent, or distribute Android/Desktop clients.

## Provider choice

The Phase 141 contract permits Phase 142 to implement TURN allocation or a PRW relay provider so long as the Phase 136 opacity/fallback boundary and end-to-end QUIC security association are preserved.

The initial Phase 142 implementation uses a dependency-free PRW disposable relay provider/service rather than adding a new third-party relay-server dependency. TURN remains the standardized external relay architecture for future provider integration, but this phase does not claim to implement a production TURN service.

No new cryptographic primitive is introduced. The relay route token is routing correlation material, not an encryption key, transport identity, device identity, account credential or capability credential.

## Component ownership

Phase 142 introduces one crate:

`crates/prw-relay-service`

Ownership boundaries:

- `prw-relay` remains the provider-neutral Phase 136 policy/lifecycle boundary and continues to require an already-selected `Relay` path;
- `prw-relay-service` owns the initial bounded relay wire envelope, disposable in-memory relay route table, peer-leg registration, bounded per-client delivery queues and a `RelayBackend` adapter used only for disposable validation;
- `prw-remote-transport` remains the owner of QUIC/TLS encryption and must not move certificate/private-key material into the relay service;
- `prw-nat-traversal` remains the ICE/STUN owner and does not absorb relay service policy;
- the Agent is not integrated with the Phase 142 relay provider in this phase;
- Phase 143 owns the authenticated capability bridge and end-to-end integration.

## Relay route token confidentiality

`RelayRouteToken` is a 32-byte non-zero opaque routing token. Although it is not a cryptographic key, disclosure can reveal active routing correlation material.

Phase 142 therefore requires:

- `RelayRouteToken` custom `Debug` output must be redacted;
- derived `Debug` for `RelaySessionSpec` must not reveal token bytes through the nested token representation;
- relay wire-message `Debug` must redact the route token and payload bytes, exposing only safe metadata such as message kind, request identifier, leg and payload length;
- errors and audit evidence must not include raw route-token bytes.

The existing explicit `as_bytes()` accessor remains available only because the wire codec needs the token value for routing. This does not upgrade the token into authorization or encryption material.

## Initial relay wire protocol

The disposable relay protocol is versioned independently of QUIC and uses a fixed 60-byte network-byte-order header followed by an optional bounded payload.

Header:

1. magic — 4 bytes: `PRWR`;
2. protocol major — `u16`, value `1`;
3. protocol minor — `u16`, value `0`;
4. message kind — `u16`;
5. flags — `u16`, value `0`;
6. request identifier — non-zero `u64`;
7. payload length — `u32`;
8. relay leg — `u8`, `1` for leg A and `2` for leg B;
9. reserved — 3 zero bytes;
10. relay route token — 32 bytes, non-zero.

Message kinds:

- `1` — register;
- `2` — register acknowledgement;
- `3` — opaque data;
- `4` — unregister;
- `5` — unregister acknowledgement.

Rules:

- unknown magic/version/kind, non-zero flags/reserved bytes, zero request identifier, invalid leg, zero route token, truncation, trailing bytes or oversized payload fail closed;
- register/register-ack/unregister/unregister-ack require zero payload bytes;
- data requires a non-empty payload;
- the initial Phase 142 data payload ceiling is 2048 bytes per relay packet;
- the 2048-byte provider packet ceiling is intentionally tighter than the Phase 136 generic `OpaqueRelayFrame` ceiling and does not widen any existing bound;
- no wire message carries a shell command, filesystem path, application capability, user credential, private key or decrypted PRW application message.

## Disposable service model

The initial service is Sans-I/O/in-memory. A separately controlled future network adapter would supply the authenticated/known client transport handle; Phase 142 itself opens no socket.

Service bounds:

- maximum active routes: 32;
- maximum route legs: two per route, A and B;
- maximum queued peer data packets per registered client: 16;
- maximum payload per packet: 2048 bytes;
- client identifiers are non-zero service-generated values;
- one client identifier may occupy only one route leg at a time;
- one route leg may have only one active client;
- the same client cannot occupy both legs;
- route removal occurs when both legs are absent;
- no migration/rebinding semantics are introduced in the initial profile.

Registration and forwarding fail closed on occupied legs, unknown routes, wrong legs, unregistered clients, unavailable peer leg, queue capacity and malformed packets.

## Disposable provider adapter

`DisposableRelayProvider` implements the existing Phase 136 `RelayBackend` interface against a shared disposable service instance.

- construction assigns the provider to exactly one relay leg (`A` or `B`);
- `open()` registers the route token from an already-validated `RelaySessionSpec` and requires an exact register acknowledgement;
- `transmit()` encodes the `OpaqueRelayFrame` as one bounded data message and preserves payload bytes exactly;
- `receive()` is a Phase 142 test/provider extension that decodes only the relay envelope and returns one `OpaqueRelayFrame`; it does not parse the opaque payload;
- `close()` unregisters the route and requires an exact unregister acknowledgement;
- request identifiers are non-zero and monotonically advanced within the provider, with deterministic wrap back to `1` after `u64::MAX`;
- provider/service failures map to the existing generic `RelayError::Backend` at the Phase 136 boundary.

## End-to-end opacity

The relay must be unable to derive PRW application plaintext from the bytes it forwards.

Phase 142 proves opacity structurally and behaviorally:

- the service has no dependency on `prw-remote-transport`, application-session, file, terminal, forwarding or policy crates;
- the service codec knows only relay header metadata and an opaque payload vector;
- data forwarded from one leg to the other is byte-identical;
- no payload parser or decryption API exists in the relay service;
- Phase 143 will connect actual authenticated encrypted transport/capability flows and remains responsible for end-to-end integration validation.

## Required disposable validation

Phase 142 validation must prove at minimum:

1. `RelayRouteToken` debug output does not expose raw token bytes;
2. relay wire-message debug output redacts token/payload bytes;
3. fixed-header round trip succeeds for every message kind;
4. malformed magic/version/kind/flags/request-id/leg/reserved/token/length/truncation/trailing bytes fail closed;
5. zero-payload control messages and non-empty bounded data rules are enforced;
6. the 2048-byte provider payload ceiling is enforced;
7. two clients can register opposite legs of one route and forward an opaque payload byte-for-byte;
8. same-leg collision, same-client dual-leg registration, unknown route/client, wrong leg, absent peer and queue-full conditions fail closed;
9. unregister removes the correct leg and empty routes are reclaimed;
10. the provider implements `RelayBackend` open/transmit/close against the disposable service and can receive the peer payload without parsing it;
11. Phase 136 continues to reject direct/offline paths before provider open;
12. source has no socket, async runtime, DNS lookup, process/shell, TUN/TAP, route or firewall ownership;
13. focused rustfmt, Clippy `-D warnings`, tests and build pass;
14. full locked workspace rustfmt, Clippy, tests and build pass;
15. no production network/runtime state is mutated.

## Phase 143 handoff

Phase 143 may use the validated Phase 142 relay provider boundary as one fallback transport path when constructing the end-to-end authenticated capability bridge. Phase 143 must still require current transport identity, authenticated application session, current registry/workspace membership and capability authorization.

Relay registration or successful relay forwarding must never be interpreted as PRW application authorization.

## Production boundary

Until the exact Phase 154 production transaction is separately approved, Phase 142 MUST NOT:

- deploy or expose a production relay endpoint;
- start persistent TURN/STUN/ICE relay traffic;
- change public/LAN listeners;
- alter firewall/NAT/router/TUN/TAP/routes/DNS;
- provision production relay or transport credentials;
- replace/restart the production Agent for relay integration;
- distribute/sign Android/Desktop clients.

## Completion classification

Target final state:

`PHASE_142_DONE / DISPOSABLE_OPAQUE_RELAY_PROVIDER_VALIDATED / END_TO_END_QUIC_OPACITY_PRESERVED / PHASE136_FALLBACK_ONLY_POLICY_PRESERVED / NO_PRODUCTION_SIDE_EFFECT / READY_FOR_PHASE_143`
