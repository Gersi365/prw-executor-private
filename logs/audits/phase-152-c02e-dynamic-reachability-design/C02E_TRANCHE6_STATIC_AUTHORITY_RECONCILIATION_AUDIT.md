# C02e Tranche 6 — Static Authority Reconciliation Audit

Status: `STATIC_RECONCILIATION_COMPLETE / PEER_NAMESPACE_REFERENCE_COVERAGE_CORRECTED / FENCE_WIDTH_AUTHORITY_MISMATCH_IDENTIFIED / EXECUTABLE_VALIDATION_STILL_UNRESOLVED / NO_PRODUCTION_RUNTIME_MUTATION`

Prior unresolved-validation head: `616768553a72f3984e8b6414ff565ebdf26aa03a`

Peer-namespace reference coverage commit: `b19be45bf66aa8c670a7adcc82fbd99c5f953877`

Frozen predecessor C02d: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

This audit resumes Tranche 6 from the exact unresolved-validation checkpoint and performs only static authority reconciliation. It does not reinterpret absent executable evidence as PASS or FAIL, does not activate runtime behavior, and does not make a new architecture/security representation decision.

## Finding 1 — exact-peer namespace reference coverage

The authoritative Tranche 6 design lock states that live-owner tenancy is keyed by the exact peer lifecycle:

`DeviceId + TransportIdentity`

It also states that acquiring a newer grant invalidates older grants for that **same exact lifecycle**.

The original inline test-only `ReferenceAuthority` in `reachability_live_owner.rs` keeps one global `Option<ReachabilityLiveOwnerGrant>`. Its existing tests exercise same-peer replacement and exact-peer grant binding, but that helper is not capable of demonstrating simultaneous current grants for independent peer namespaces.

If that helper were treated as a general distributed-authority reference model, acquiring peer B would make peer A appear stale. That would contradict the exact-peer namespace rule.

### Bounded correction

Commit `b19be45bf66aa8c670a7adcc82fbd99c5f953877` adds only:

`crates/prw-remote-bridge/tests/reachability_live_owner_peer_namespace.rs`

The new test-only `PeerScopedReferenceAuthority` tracks current grants independently by exact `PeerConnectivityIdentity` and adds source coverage proving:

1. acquiring another peer does not stale the existing peer;
2. replacement fences only the same exact peer namespace;
3. same `DeviceId` with a replacement `TransportIdentity` is a distinct authority namespace;
4. stale release in one namespace cannot clear authority in another namespace.

The transport-rotation test is intentionally namespace-only. Registry currentness remains a distinct production authority and may separately reject the obsolete transport identity.

No production trait/type/root/Cargo/runtime source changed in this correction.

## Finding 2 — fence-width authority mismatch

The Tranche 6 design lock explicitly says:

- the exact integer width/serialization is not selected by that design lock;
- a later production representation must preserve practically non-wrapping strict ordering and fail closed before reuse/wrap.

The subsequently staged production seam currently defines:

`ReachabilityLiveOwnerFence(NonZeroU128)`

and the staging audit explicitly records a non-zero `u128` backing representation.

That is a concrete source representation choice beyond what the preceding design lock selected.

This audit does **not** decide whether `u128` should be retained or replaced. Either outcome is an architecture/security representation decision and must be made explicitly before Tranche 6 can be classified as authority-complete.

### Required resolution options

A later explicitly authorized representation checkpoint must choose one of these classes of resolution:

1. retain `u128` as the reviewed in-memory fencing-generation representation while keeping persistence/wire serialization separately unselected; or
2. replace the concrete width with another reviewed representation/seam that preserves strict ordering, non-reuse and fail-closed exhaustion semantics.

The resolution must not silently couple the fence to candidate-publication freshness, request IDs, endpoint state, clocks or caller-controlled bytes.

## Executable-validation status

The prior classification remains authoritative:

`EXECUTABLE_VALIDATION_UNRESOLVED`

No compiler, rustfmt, Clippy, focused test, workspace test or workspace build result is fabricated by this static review.

Because the fence-width authority mismatch now precedes closeout, executable validation should target the representation-resolved exact head rather than treating the currently staged `u128` seam as already architecture-closed.

## Boundaries preserved

This reconciliation performs no:

- Cargo manifest or `Cargo.lock` mutation;
- `reachability_owner.rs` mutation;
- Agent/bootstrap integration;
- persistence/backend selection;
- lease TTL/heartbeat/clock selection;
- socket/network/STUN/TURN/ICE/QUIC activation;
- PR creation/merge;
- deployment/signing/service-manager mutation.

C02d remains untouched.

## Result

`TRANCHE6_STATIC_AUTHORITY_RECONCILED / PEER_NAMESPACE_REFERENCE_GAP_CLOSED_TEST_ONLY / FENCE_WIDTH_DECISION_EXPLICITLY_REOPENED / EXECUTABLE_VALIDATION_PENDING_AFTER_REPRESENTATION_RESOLUTION / PRODUCTION_RUNTIME_CLOSED`
