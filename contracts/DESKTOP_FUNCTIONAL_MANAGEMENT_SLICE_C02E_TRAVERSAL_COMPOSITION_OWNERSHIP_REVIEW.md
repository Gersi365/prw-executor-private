# Phase 152 C02e — Traversal Composition Ownership Review

Status: `PASS_STATIC_OWNERSHIP_REVIEW / NO_CURRENT_COMPOSITION_OWNER / OWNER_AND_RUNTIME_MECHANISM_UNSELECTED / OLD_TRAVERSAL_INVALIDATION_REQUIRED / FAILED_REFRESH_PRESERVES_CURRENT_TRAVERSAL / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Review base head: `67e5f4fa2bd6b750d9083ee2fe7ec54407e10444`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

The traversal-session refresh checkpoint established that a successful full candidate refresh creates a new traversal-observation lifecycle. This review determines whether the current repository already has an authoritative composition owner that can enforce that rule without creating a new architecture edge or runtime mechanism.

It does not choose a new owner, add a Cargo dependency, introduce a generation/token, or activate traversal runtime.

## Current crate ownership evidence

The current dependency boundaries are explicit:

### `prw-nat-traversal`

`crates/prw-nat-traversal/Cargo.toml` depends on:

- `prw-connectivity`;
- the locked Sans-I/O `rtc-*` / `sansio` protocol libraries.

It does not depend on session, registry, remote bridge or Agent runtime domains.

Phase 141 explicitly assigns it protocol state only, not sockets or orchestration. The caller must move bounded datagrams through a separately controlled network adapter in a later integration phase.

### `prw-remote-bridge`

`crates/prw-remote-bridge/Cargo.toml` already composes authenticated session, registry, connectivity, transport and capability semantics, but it does **not** depend on `prw-nat-traversal`.

The C02e candidate semantic adapter in this crate is deliberately unexported and freshness-agnostic.

### `prw-agent`

`crates/prw-agent/Cargo.toml` depends on `prw-remote-bridge`, registry, session, forwarding and other local/runtime domains, but it also does **not** depend on `prw-nat-traversal`.

Production Agent/bootstrap traversal wiring is separately closed.

## Ownership conclusion

No current crate owns both:

- the mutable current `PeerConnectivityPlan` lifecycle; and
- the current `IceConnectivitySession` lifecycle.

No current source therefore provides an authoritative location where a successful candidate refresh can invalidate the old traversal session and prevent stale queued observations from crossing the refresh boundary.

Selecting such an owner now would require a new dependency/composition decision rather than reusing an existing authoritative owner. That is outside this source-only corrective and must remain unselected/fail-closed.

In particular, C02e must not silently:

- make `prw-connectivity` own ICE protocol state;
- add `prw-nat-traversal` to `prw-remote-bridge` or `prw-agent` merely to obtain a convenient owner;
- move traversal ownership into registry/session identity layers;
- activate an Agent/network adapter to solve a source-design ownership gap.

## Logical transition that the future owner must enforce

Although the concrete owner and concurrency primitive are unselected, the required ordering is now derivable and locked.

For a candidate publication that passes identity/workspace/transport/freshness admission:

1. the complete candidate vector must be validated without mutation;
2. a failed candidate refresh must leave the existing plan **and the existing traversal session lifecycle current**;
3. a successful candidate refresh must create a new traversal-observation lifecycle;
4. after successful refresh, the preceding traversal session and all queued/unapplied observations from it are stale;
5. no stale observation may be admitted after the refresh, including one for an exactly retained candidate ID/path/endpoint;
6. any continued traversal must use a replacement traversal session constructed from the refreshed current candidate state plus current authenticated coordination metadata;
7. only observations attributable to that current replacement traversal lifecycle may update the refreshed plan.

The future composition owner must make steps 3-5 indivisible with respect to observation admission: there must be no interval in which the refreshed plan is current while old traversal observations are still accepted as current evidence.

This is a logical atomicity requirement only. C02e does not choose a mutex, channel, task cancellation model, epoch counter, queue-drain implementation or async runtime.

## Relationship to publication freshness

Candidate-publication freshness and traversal-session currentness remain separate verifier-owned concerns.

Publication freshness orders accepted candidate-state transitions.

Traversal-session currentness determines whether a reachability observation belongs to the traversal lifecycle created for the current accepted candidate state.

The future owner must not reuse one mechanism as implicit proof of the other unless a later reviewed contract explicitly binds them.

## Transport rotation

`TransportIdentity` rotation remains a stronger reset:

- old plan becomes stale;
- old candidate-publication freshness lifecycle becomes stale according to its eventual exact authority;
- all traversal sessions/queued observations associated with the old plan are stale;
- replacement plan uses the same logical `DeviceId` plus the new current `TransportIdentity`;
- no endpoint, candidate ID or traversal observation carries authorization into the replacement identity.

## Dependency-direction invariant

`prw-nat-traversal -> prw-connectivity` is the current validated protocol/domain dependency direction.

This review does not authorize reversing that dependency, creating a cycle, or adding traversal protocol ownership to the connectivity domain.

A later composition owner must sit above both relevant state machines rather than making either lower-level domain impersonate the other.

The exact crate/module that provides that upper composition layer remains unselected.

## Validation boundary

Static ownership/dependency review only.

No Cargo manifest, lockfile, Rust source, test source, Agent/bootstrap source, network adapter, persistence backend or production state is changed by this checkpoint.

No build, `cargo fmt`, Clippy, tests, workflow dispatch, TCP/UDP I/O, STUN/ICE/TURN execution, QUIC activity, PTY/process I/O, signing, deployment or privileged mutation is performed.

## Next safe seam

C02e has no existing authoritative source owner for traversal invalidation across candidate refresh.

Until a later reviewed architecture/composition step selects an owner without violating current dependency and runtime boundaries, keep traversal integration unconfigured and fail-closed. The next useful work inside the current authorization is evidence synchronization/readback, not invention of a new Cargo edge or runtime lifecycle mechanism.
