# Phase 152 C02e — Tranche 5 Freshness-Token Wire Delivery / Authenticated Resynchronization

Status: `DESIGN_AND_SOURCE_SELECTION / PRWM_REQUEST_RESPONSE_ERROR_REUSED / PRWF_V1_PAYLOAD_SELECTED / AUTHENTICATED_DURABLE_RESYNC_SELECTED / TOKEN_REDELIVERY_NON_MUTATING / NO_NEW_CONTROL_KIND / NO_DISTRIBUTED_RUNTIME_TENANCY / NO_NETWORK_IO / NO_AGENT_BOOTSTRAP_ACTIVATION`

Tranche 4 closeout base head:

`eea6b8743eebf21002ae173dfcfd5cbbf93378a8`

Frozen predecessor C02d:

`857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

This tranche selects the exact bounded wire representation for verifier-owned candidate-publication freshness token delivery and the authenticated current-token resynchronization protocol that Tranche 3 deliberately left open.

It does **not** open the separate distributed live-owner tenancy/fencing gate, real socket/network activation, candidate-vector wire encoding, concrete database/schema/replication selection, Agent/bootstrap runtime wiring, deployment, signing, PR or merge.

## Preserved authority

The protocol preserves all earlier C02e authority locks:

- `DeviceId` from a current authenticated PRW session is the logical publisher identity;
- `TransportIdentity` is independently rotatable and must be registry-current for that exact device;
- IP/port, request ID, candidate ID and session ID are not freshness identity;
- candidate-publication freshness remains verifier-owned replay-ordering state scoped to exact `DeviceId + TransportIdentity`;
- session renewal for the same exact peer lifecycle does not reset freshness;
- `NewLifecycleEligible(token)` and `Established(token)` carry exact current verifier state;
- `RecoveryRequired` and `Retired` disclose no token and fail closed;
- accepted candidate publication rotates freshness only at the already-selected durable CAS commit point;
- resynchronization never generates, advances, resets or re-baselines freshness.

## Transport placement

The selected outer transport is the existing Phase 140 PRWM remote transport from `prw-remote-transport`.

No new PRWM `ControlMessageKind` value is allocated. Tranche 5 reuses:

- `Request` for a current-token resynchronization request;
- `Response` for successful token delivery;
- `Error` for bounded fail-closed semantic failure.

The existing non-zero PRWM `request_id` remains correlation only. It is not freshness, identity, authentication or replay authority.

Tranche 5 does not use the older outbound-only Phase 129 TLS control transport as the C02e peer freshness authority surface, and it does not extend the Phase 143 capability `BridgeCommand` operation namespace. Freshness delivery is protocol coordination above current session/registry identity, not a file/terminal/forwarding capability grant.

## Selected PRWF payload

The inner payload magic is exact ASCII `PRWF` (`[0x50, 0x52, 0x57, 0x46]`).

Version is exactly `1.0`.

All integer fields are unsigned big-endian. Reserved fields must be zero. Unknown operations, unknown enum codes, wrong version/magic, invalid all-zero transport identity/token, truncation or trailing bytes fail closed.

Fixed header, 12 bytes:

| Offset | Width | Field |
|---|---:|---|
| 0 | 4 | magic `PRWF` |
| 4 | 2 | major = `1` |
| 6 | 2 | minor = `0` |
| 8 | 2 | operation |
| 10 | 2 | reserved flags = `0` |

### Operation 1 — `CurrentTokenResynchronizationRequest`

Outer PRWM kind: `Request`.

Body is exactly 32 bytes:

`TransportIdentity[32]`

No `DeviceId` is accepted from the payload. Logical device identity is derived from the already-authenticated current session. This prevents caller-controlled retargeting of the replay namespace.

### Operation 2 — `TokenDelivery`

Outer PRWM kind: `Response`.

Body is exactly 68 bytes:

| Width | Field |
|---:|---|
| 2 | delivery reason |
| 2 | reserved = `0` |
| 32 | exact `TransportIdentity` |
| 32 | exact non-zero verifier freshness token |

Delivery-reason codes:

1. `Bootstrap` — initial delivery from an already-durable `NewLifecycleEligible(token)` record;
2. `AcceptedPublication` — replacement token obtained from a definitely committed `ReachabilityCommitOutcome`;
3. `Resynchronization` — non-mutating re-delivery of exact authoritative current durable state.

The reason is metadata only. Token bytes have no embedded counter/time/reason semantics.

### Operation 3 — `Failure`

Outer PRWM kind: `Error`.

Body is exactly 4 bytes:

| Width | Field |
|---:|---|
| 2 | stable failure code |
| 2 | reserved = `0` |

Failure codes:

1. `CurrentnessRejected` — authenticated session/device/transport currentness failed;
2. `StalePublicationFreshness` — candidate publication presented a consumed/non-current token;
3. `DurableStateMissing` — no authoritative snapshot exists for the established exact peer lifecycle;
4. `RecoveryRequired` — exact freshness state is ambiguous/unrecoverable or snapshot consistency fails;
5. `Retired` — exact peer lifecycle is a durable tombstone;
6. `PersistenceUnavailable` — authoritative durable load/transaction outcome is unavailable;
7. `ProtocolRejected` — bounded generic protocol/semantic rejection.

Registry-internal detail is not required on the wire. Authentication/currentness failures collapse to `CurrentnessRejected`.

## Bootstrap token delivery

Wire delivery cannot create bootstrap authority.

A `Bootstrap` `TokenDelivery` may be constructed only from an already-authoritative `CandidatePublicationFreshnessRecord::NewLifecycleEligible(token)` for the exact peer. Storage absence, an authenticated request, reconnect, restart or caller-supplied bytes cannot synthesize bootstrap eligibility.

## Accepted-publication token delivery

A successful candidate publication may return `TokenDelivery(reason = AcceptedPublication)` only from the `ReachabilityCommitOutcome` returned after the existing Tranche 4 owner receives definite durable `Committed` from `ReachabilityDurableStore::compare_and_commit`.

Therefore a response containing the replacement token is downstream of the accepted-state linearization point. A pre-commit or ambiguous commit path has no valid `ReachabilityCommitOutcome` and must not emit success/token delivery.

This tranche does not select candidate-vector request encoding; it selects the success token payload that an eventual candidate-publication request path must use after commit.

## Authenticated current-token resynchronization

Resynchronization is selected as a read/re-delivery protocol over the existing durable authority.

Required order:

1. receive PRWM `Request` carrying PRWF `CurrentTokenResynchronizationRequest`;
2. require an already-authenticated current PRW publisher session;
3. revalidate that session with `WorkspaceDeviceRegistry::validate_authenticated_session`;
4. derive logical `DeviceId` from that authenticated session, never from payload bytes;
5. revalidate the presented 32-byte `TransportIdentity` as current for that exact device;
6. construct exact `PeerConnectivityIdentity(DeviceId, TransportIdentity)`;
7. use the already-selected `ReachabilityDurableStore::load_current` authoritative read for that exact peer;
8. require snapshot plan peer and freshness peer to equal the exact lookup peer;
9. return the same exact current token only for `NewLifecycleEligible(token)` or `Established(token)`;
10. return fail-closed failure for missing state, `RecoveryRequired`, `Retired`, peer mismatch or persistence ambiguity.

No compare-and-commit occurs during resynchronization. No token source is invoked. No freshness lifecycle transition occurs. No candidate or traversal mutation occurs.

A subsequent resynchronization must perform a new authoritative durable load; it must not assume a previously cached token remains current. This handles the lost-response case where another accepted commit has already advanced durable freshness.

## Security and replay properties

- PRWM transport establishment alone is insufficient; current authenticated session and registry validation remain mandatory.
- The request contains no caller-selected `DeviceId`, workspace, user, token baseline or current token guess.
- Token delivery includes the exact transport identity so recipients can bind returned state to the independently rotatable transport lifecycle.
- Request ID is correlation only and cannot make a stale token current.
- Replaying a resynchronization request may cause re-delivery of the same current token to the same authenticated current peer, but cannot rotate or reset freshness.
- Replaying an old accepted-publication request still fails at the existing current-token comparison after its token is consumed.
- A rotated/stale transport identity is rejected before durable token lookup.
- `Retired` historical identity cannot obtain a token merely by replaying old wire traffic.

## Source placement

Selected source module:

`crates/prw-remote-bridge/src/reachability_freshness_wire.rs`

It reuses:

- `prw-remote-transport::ControlFrame` and existing `Request`/`Response`/`Error` kinds;
- `WorkspaceDeviceRegistry` and `AuthenticatedDeviceSession` for currentness;
- Tranche 3 freshness representation;
- Tranche 4 `ReachabilityDurableStore` authoritative load and `ReachabilityCommitOutcome` commit evidence.

No new Cargo dependency is required.

## Deliberately unselected / closed

Tranche 5 does not select or activate:

- distributed live-owner tenancy, lease, fencing, leader election or observation-writer arbitration;
- concrete database, record encoding, schema migration, replication or consensus technology;
- candidate-vector wire serialization;
- runtime handler/task/channel/queue/cancellation ownership;
- real QUIC stream opening, socket I/O, STUN/TURN/ICE packets, relay traffic or network adapter;
- Agent/bootstrap integration;
- same-identity rebaseline;
- deployment, system mutation, signing, PR creation/merge or production activation.

## Validation boundary

Executable validation may compile/lint/test the pure codec, PRWM frame wrapping and in-memory authenticated durable-resynchronization semantics. Validation must perform no network I/O and must preserve the locked Cargo dependency graph and `Cargo.lock` hash.

## Exit condition

Tranche 5 closes when:

1. PRWF v1.0 message/field/operation/failure allocation is present in production source;
2. bootstrap delivery is only constructible from authoritative bootstrap lifecycle state;
3. accepted-publication delivery is tied to definite commit evidence;
4. authenticated resync revalidates session + exact transport before authoritative durable load;
5. resync returns the same exact current durable token without CAS/token generation/mutation;
6. recovery/retired/missing/currentness paths fail closed;
7. focused and full locked workspace validation pass;
8. temporary validation harnesses are removed;
9. Google Drive mutable branch evidence mirror is synchronized after final GitHub closeout.
