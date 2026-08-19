# Phase 152 C02e — Tranche 6 Live-Owner Fencing Authority Lock

Status: `DESIGN_LOCK / ACCEPTED_STATE_CAS_DISTINCT_FROM_LIVE_OWNER_FENCING / EXACT_PEER_LIFECYCLE_BOUND / MONOTONIC_FENCE_REQUIRED / STALE_OWNER_SIDE_EFFECTS_FORBIDDEN / CONCRETE_LEASE_BACKEND_UNSELECTED / NO_NETWORK_IO / NO_AGENT_BOOTSTRAP_ACTIVATION`

Starting C02e head: `78daf5b02ed359762eba0cfb5afcd0effbc86bc6`

Frozen predecessor C02d: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

Tranche 5 closed accepted-state freshness-token wire delivery and authenticated durable resynchronization. The remaining production-reachability boundary is different: a durable accepted-state CAS does not prove that only one transient runtime owner may currently drive one peer's traversal/network lifecycle.

This tranche locks that distinction before any real socket/network adapter or Agent/bootstrap integration is allowed.

It selects no persistence product, distributed-lock service, wall-clock lease duration, heartbeat cadence, async runtime, task model, socket adapter, STUN/TURN/ICE runtime or deployment mechanism.

## Existing authority retained

The existing `prw-remote-bridge::reachability_owner::ProductionReachabilityOwner` remains the selected upper composition owner for:

- current authenticated peer identity;
- current `PeerConnectivityPlan`;
- verifier-owned candidate-publication freshness;
- durable accepted-state compare-and-commit;
- at most one local Phase 141 Sans-I/O traversal session.

The existing `ReachabilityDurableStore::compare_and_commit(...)` remains the accepted-state linearization seam.

It is **not** reinterpreted as distributed runtime-owner tenancy.

## Why CAS is insufficient for transient ownership

`&mut self` serializes only one in-process Rust owner instance.

The durable accepted-state CAS prevents two writers from both committing the same expected freshness transition, but it does not stop two processes/replicas that loaded the same accepted snapshot from simultaneously holding local traversal state, polling observations or later driving network side effects.

Therefore these are separate authorities:

1. **accepted-state authority** — which candidate/freshness snapshot is durably current;
2. **live-owner authority** — which transient runtime owner is currently allowed to act for that exact peer lifecycle.

Neither authority substitutes for the other.

## Exact tenancy namespace

Live-owner authority is keyed by the exact current peer lifecycle:

`DeviceId + TransportIdentity`

It is not keyed by:

- IP address;
- UDP/TCP port;
- candidate ID;
- endpoint/path kind;
- session ID;
- control-frame request ID;
- candidate-publication freshness token;
- process ID;
- host UID/GID;
- wall-clock timestamp supplied by a requester.

A `TransportIdentity` rotation creates a different live-owner namespace even when the logical `DeviceId` remains the same.

## Fencing generation requirement

A correct distributed live-owner authority must issue an authority-owned fencing generation for one exact peer lifecycle.

The fencing generation must be:

- created only by the live-owner authority;
- bound to the exact `DeviceId + TransportIdentity` lifecycle;
- strictly newer than every fencing generation previously granted for that same lifecycle;
- durably ordered so authority restart/failover cannot reuse an older generation as current;
- impossible for request payloads, endpoint state, candidate IDs or freshness-token bytes to select directly;
- compared as an ordering/fencing value, not treated as a secret authentication credential.

The exact integer width/serialization is not selected by this design lock. Production representation must preserve a practically non-wrapping strict ordering and fail closed before reuse/wrap could occur.

## Grant replacement semantics

Acquiring a newer live-owner grant for an exact peer lifecycle invalidates every older grant for that same lifecycle.

Correctness must not depend on a cooperative release from the old owner because processes may crash, pause, partition or lose execution indefinitely.

Explicit release may exist as a liveness optimization, but it is not the safety mechanism.

## Required current-owner checks

A future production composition must require current live-owner authority before transient traversal/runtime work including:

1. provisioning a Phase 141 traversal session for the current plan;
2. polling or applying traversal reachability observations;
3. constructing or replacing a real network-adapter session;
4. emitting any future network/runtime side effect attributable to that peer lifecycle;
5. continuing a long-lived runtime operation after an authority loss/reacquisition boundary.

Accepted-state recovery alone does not restore live-owner authority. After any owner recovery/restart, the runtime must reacquire or revalidate live-owner authority separately.

## Side-effect fencing requirement

A one-time pre-check is insufficient for a distributed fencing design because an old owner may pause after checking and resume after a newer owner has been granted authority.

Therefore the future concrete side-effect boundary must reject stale fencing generations.

At minimum, every long-lived network/runtime side-effect channel must be attributable to the exact current live-owner generation, and replacement ownership must make older-generation side effects inadmissible or terminate them before they can continue as current authority.

The concrete mechanism is deliberately unselected here. It may be implemented only in a later reviewed runtime/protocol tranche once the actual network adapter and distributed authority backend are chosen.

## Authority-loss behavior

Any of these conditions fail closed for transient runtime authority:

- live-owner authority unavailable;
- authority result ambiguous;
- presented grant belongs to another peer lifecycle;
- presented generation is older than current;
- currentness cannot be proven;
- transport identity is no longer registry-current;
- the peer lifecycle is durably `RecoveryRequired` or `Retired`.

Loss or ambiguity must invalidate/cancel local traversal/runtime authority. It must not silently continue because the last accepted candidate snapshot is still locally cached.

## Relationship to candidate-publication freshness

`CandidatePublicationFreshnessToken` remains accepted-publication ordering state.

It must not be reused as a live-owner fencing generation.

Reason:

- publication freshness advances when accepted candidate state changes;
- live ownership may change without any candidate publication;
- candidate publication may change while the same live owner remains current;
- requester reconnect/session renewal does not necessarily change either state;
- transport rotation creates a replacement exact-peer lifecycle for both domains but still does not make the two tokens interchangeable.

## Relationship to durable accepted state

The existing durable reachability snapshot remains:

- exact current `PeerConnectivityPlan`;
- exact verifier-owned freshness lifecycle.

This tranche does not add live-owner tenancy fields to `ReachabilityDurableSnapshot` and does not widen the existing CAS API.

A future concrete backend may colocate accepted-state and live-owner records transactionally if justified, but the semantic states remain distinct and independently auditable.

## Runtime placement direction

The live-owner proof/check seam belongs at or above the selected `prw-remote-bridge` production reachability composition boundary because that is the first current source layer that owns both accepted reachability state and traversal lifecycle.

Lower domains must not absorb this responsibility:

- `prw-connectivity` remains provider-neutral candidate/observation state;
- `prw-nat-traversal` remains Sans-I/O traversal protocol state;
- registry/session identity layers remain identity/currentness authorities, not traversal-runtime tenancy managers.

No Cargo dependency change is required by this design lock.

## Concrete mechanism deliberately unselected

This tranche does **not** select:

- etcd/Consul/Redis/PostgreSQL/SQLite or any other backend;
- TTL length;
- heartbeat/renew interval;
- wall-clock vs monotonic-clock implementation;
- consensus technology;
- process/thread/task ownership;
- cancellation primitive;
- network protocol field encoding for a fence;
- peer-side fence persistence;
- socket/session shutdown implementation.

Those choices depend on the later concrete runtime and deployment topology.

## Security invariants

Tranche 6 must not allow:

- endpoint/IP identity to become owner identity;
- a request to choose its own fencing generation;
- publication freshness to masquerade as live-owner fencing;
- a stale owner to remain authoritative merely because its local plan is current;
- a stale owner to regain authority by replaying an older grant;
- release success to be required for safety;
- transport rotation to preserve the old exact-peer live-owner grant;
- authority ambiguity to be treated as ownership success;
- live-owner loss to resurrect stale traversal state;
- distributed tenancy design to activate sockets, STUN/TURN/ICE traffic or Agent bootstrap.

## Next implementation seam

The next bounded source tranche may add a pure `prw-remote-bridge` live-owner authority/fencing type seam and reference tests that prove:

- non-zero/ordered authority-issued fencing representation;
- exact-peer binding;
- newer-grant invalidation of older grants;
- stale-grant rejection;
- cross-peer rejection;
- accepted-state freshness remains type-distinct;
- no socket/runtime I/O is introduced.

Production runtime integration remains a later gate because pure source fencing semantics alone cannot prove stale network side effects are physically blocked.

## Classification

`C02E_TRANCHE6_LIVE_OWNER_FENCING_AUTHORITY_LOCKED / ACCEPTED_STATE_CAS_NOT_TENANCY / EXACT_DEVICE_TRANSPORT_NAMESPACE / MONOTONIC_DURABLE_FENCE_REQUIRED / STALE_SIDE_EFFECTS_MUST_BE_FENCED / CONCRETE_BACKEND_AND_RUNTIME_UNSELECTED / C02D_UNTOUCHED`
