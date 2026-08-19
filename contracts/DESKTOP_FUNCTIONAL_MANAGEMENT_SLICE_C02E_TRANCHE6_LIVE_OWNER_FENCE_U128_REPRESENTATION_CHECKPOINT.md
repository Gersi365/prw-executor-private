# Phase 152 C02e — Tranche 6 Live-Owner Fence `u128` Representation Checkpoint

Status: `REPRESENTATION_DECISION_LOCKED / NONZERO_U128_LOGICAL_IN_MEMORY_FENCE / STRICT_MONOTONIC_ORDER_REQUIRED / PERSISTENCE_ENCODING_UNSELECTED / WIRE_ENCODING_UNSELECTED / BACKEND_UNSELECTED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Starting C02e head: `1664c27b6999f986f634537cbb7d9c5dc9374c83`

Frozen predecessor C02d: `857583b25ed1206317641a93fd8f927819c954d8`

Prior static authority reconciliation:

`logs/audits/phase-152-c02e-dynamic-reachability-design/C02E_TRANCHE6_STATIC_AUTHORITY_RECONCILIATION_AUDIT.md`

## Purpose

The original Tranche 6 authority lock deliberately left exact fencing-generation integer width and serialization unselected. The staged provider-neutral source subsequently used `NonZeroU128`, creating an authority-provenance mismatch that was explicitly reopened by the static reconciliation checkpoint.

This checkpoint resolves only that representation question.

It does not select a concrete distributed authority backend, persistence schema, wire protocol encoding, lease TTL, heartbeat cadence, clock source, runtime/task model, network adapter, Agent/bootstrap integration or deployment mechanism.

## Decision

`ReachabilityLiveOwnerFence` retains a non-zero `u128` as the reviewed logical in-memory fencing-generation representation.

The source representation:

`ReachabilityLiveOwnerFence(NonZeroU128)`

is therefore architecture-authorized for the provider-neutral Tranche 6 seam.

This decision is intentionally narrower than selecting a storage or wire representation.

## Semantics

For one exact live-owner namespace:

`DeviceId + TransportIdentity`

a fencing generation is an authority-issued ordered value with these required properties:

1. zero is invalid and never denotes authority;
2. a successful replacement grant must use a value strictly greater than every previously issued grant for the same exact peer lifecycle;
3. an older value can never become current again after a newer value has been established;
4. restart/failover must not permit generation reuse or rollback;
5. exhaustion or inability to prove a strictly newer safe value fails closed;
6. the value is compared as an ordering/fencing generation, not treated as a secret authentication credential.

The logical ordering is the native total ordering of non-zero unsigned 128-bit values.

## Why `u128`

A non-zero `u128` is retained because it provides:

- a simple total ordering suitable for fencing comparisons;
- an extremely large practical monotonic generation space;
- an explicit zero-invalid invariant through `NonZeroU128`;
- direct checked arithmetic for fail-closed exhaustion handling;
- no dependency on wall-clock time, randomness, UUID ordering, endpoint state or requester-controlled bytes;
- a provider-neutral representation that does not force a database, distributed-lock service or wire format.

This choice minimizes source churn because the already staged seam uses this exact representation and its semantics already match the Tranche 6 monotonic-fencing requirements.

## What this decision does not mean

Retaining `u128` does **not** mean:

- the fence is a random token;
- the fence is an authentication secret;
- callers may choose fence values;
- a timestamp may substitute for authority allocation;
- candidate-publication freshness may be reused as live-owner fencing;
- `u128` byte order or wire serialization is selected;
- a persistence column type or schema is selected;
- a concrete backend may reset the sequence after restart;
- one global sequence is required across unrelated peer namespaces.

A concrete authority may allocate from a wider or globally ordered durable mechanism internally, but the value exposed through this seam must preserve the reviewed non-zero `u128` strict-order semantics and must fail closed before truncation, reuse or wrap can occur.

## Persistence boundary remains open

No durable encoding is selected here.

A future backend may represent the logical fence using a database-native integer/numeric type, two machine words, fixed-width bytes or another reviewed encoding, provided decode/encode preserves the exact unsigned `u128` value and its ordering without truncation or ambiguity.

The concrete backend must durably preserve the monotonic non-reuse invariant across restart, failover and replacement ownership.

## Wire boundary remains open

No network protocol field for the live-owner fence is selected by this checkpoint.

If a later side-effect fencing design needs the generation to cross a process or network boundary, that protocol tranche must independently specify:

- versioning;
- byte order/encoding;
- length and canonical representation;
- peer binding;
- stale-generation rejection behavior;
- compatibility and rollout semantics.

No existing PRWM/PRWF field is reinterpreted as a live-owner fence by this decision.

## Namespace isolation

The fence is meaningful only together with its exact peer lifecycle.

Independent peer namespaces may each hold current grants simultaneously. A higher numeric fence for peer B does not stale a lower numeric fence for peer A.

Replacement invalidation applies only within the same exact:

`DeviceId + TransportIdentity`

namespace.

Registry/session currentness remains a separate authority that may reject an obsolete transport lifecycle even if a historical grant object still exists locally.

## Exhaustion

`u128::MAX` is the final representable logical generation.

A concrete authority must never wrap to zero or reuse an older generation. If it cannot establish a strictly newer representable generation for the exact peer lifecycle, acquisition must fail closed as `FenceExhausted` or an equivalent authority failure.

No rollover/rebase protocol is authorized by this checkpoint.

## Relationship to staged source

The currently staged source blob for:

`crates/prw-remote-bridge/src/reachability_live_owner.rs`

is:

`ad21a7cc4369e1f5b9953f72c5b12bf64e50a404`

Its existing representation uses `NonZeroU128`, exposes ordered comparison semantics, rejects zero, and includes a `FenceExhausted` authority error class.

Therefore no production source rewrite is required solely to resolve the representation decision.

Executable validation is still required for the exact representation-resolved source state before Tranche 6 closeout.

## Security invariants preserved

This checkpoint must not permit:

- caller-controlled fence generation;
- random-token semantics to replace strict ordering;
- wall-clock timestamps to become ownership proof;
- candidate freshness to become live-owner fencing;
- IP/port/endpoint identity to become owner identity;
- stale generation reuse after restart/failover;
- cross-peer numeric comparison to create global tenancy coupling;
- serialization/storage choices to be inferred implicitly from the in-memory type;
- a one-time currentness pre-check to be treated as sufficient future side-effect fencing.

## Next gate

With representation authority now resolved, the next permitted Tranche 6 action is observable executable validation of the representation-resolved exact source state.

That validation must include at least:

- locked Cargo metadata;
- rustfmt;
- focused live-owner fencing tests, including peer-namespace isolation coverage;
- focused `prw-remote-bridge` Clippy with `-D warnings`;
- workspace Clippy with `-D warnings`;
- workspace tests;
- workspace build;
- `Cargo.lock` hash stability;
- final zero tracked drift.

Only after authoritative executable PASS evidence may Tranche 6 be closed and later work consider a concrete distributed live-owner backend or runtime/network side-effect fencing.

## Classification

`TRANCHE6_U128_REPRESENTATION_LOCKED / LOGICAL_IN_MEMORY_ONLY / STRICT_MONOTONIC_NON_REUSE_REQUIRED / STORAGE_AND_WIRE_UNSELECTED / EXISTING_SOURCE_REPRESENTATION_AUTHORIZED / EXECUTABLE_VALIDATION_NEXT / C02D_UNTOUCHED / PRODUCTION_RUNTIME_CLOSED`
