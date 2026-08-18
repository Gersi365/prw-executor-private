# Phase 152 C02e — Test-Only Reachability Composition Reference Harness

Status: `SOURCE_SPEC_STAGED / TEST_ONLY_EXCLUSIVE_OWNER_REFERENCE / NO_PRODUCTION_DEPENDENCY_EDGE / FRESHNESS_REPRESENTATION_UNSELECTED / ACTUAL_PHASE141_OWNER_UNSELECTED / BUILD_GATE_CLOSED / NOT_EXECUTED / NO_NETWORK_IO`

Base C02e head: `30cf135e9974745a95a1ef84cc8a806dac29bad6`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

The linearization precedent review authorized an exclusive-mutable source/disposable reference owner, but the production crate graph still has no component that depends on all C02e admission and Phase 141 traversal domains.

This checkpoint stages the narrowest source-level reference harness that can prove the locked plan/traversal lifecycle rules **without** adding a production or test dependency edge while the Cargo/build gate is closed.

## Existing test-placement precedent

`crates/prw-remote-bridge/tests/candidate_reachability_semantic_adapter.rs` already uses:

```rust
#[path = "../src/candidate_reachability.rs"]
mod candidate_reachability;
```

This compiles the intentionally unexported C02e semantic adapter only in integration-test source and preserves the production `prw-remote-bridge` module graph.

The crate already has test dependencies required for authenticated session, registry and candidate-publication fixtures.

## Why actual Phase 141 is not imported here

`prw-remote-bridge` does not currently depend on `prw-nat-traversal`.

Adding that dependency, even as a dev-dependency, would be a Cargo manifest/dependency-graph mutation and can require root lockfile reconciliation. The current build/Cargo gate is closed, so this checkpoint does not make that change and does not manually edit `Cargo.lock`.

Therefore the harness does **not** claim to be the final owner of an actual `IceConnectivitySession`.

## Test-only traversal lifecycle marker

The staged source uses an opaque local enum only to distinguish:

- the traversal lifecycle current before a refresh;
- a separately installed replacement lifecycle.

This marker is explicitly not:

- a production generation integer;
- a nonce;
- a timestamp;
- a replay token;
- a control-plane field;
- an ICE session identifier;
- a wire value.

It is test scaffolding for ownership/currentness behavior only.

## Freshness boundary remains external

Candidate-publication freshness is still mandatory and verifier-owned.

The harness uses a zero-sized `TestOnlyFreshnessAdmission` marker only to make the already-required precondition visible in the staged method signature. It carries no value, cannot prove replay ordering, and is not a proposed production representation.

The harness therefore validates **composition after an externally successful freshness admission**, while existing C02e freshness checkpoints remain authoritative for replay/compare-and-advance requirements.

## Reference owner semantics staged

`ReachabilityCompositionReference` owns:

- one actual `PeerConnectivityPlan`;
- one optional test-only current traversal lifecycle marker.

The staged operation `commit_after_external_freshness_admission(...)`:

1. invokes the actual C02e authenticated publication consumption path;
2. therefore revalidates requester/publisher/workspace/target/transport and delegates candidate transactionality to `PeerConnectivityPlan`;
3. invalidates the current traversal marker only after that real refresh returns success;
4. leaves the traversal marker unchanged on every adapter/plan error.

Because the method owns `&mut self`, no other reference-model operation can interleave between successful plan refresh and the following infallible traversal-marker invalidation.

## Observation admission staged

The reference owner exposes observation application only through a method that first checks the supplied test lifecycle marker against the currently owned marker.

A stale lifecycle fails before `PeerConnectivityPlan::set_observation(...)`.

This proves the locked rule that candidate existence alone is insufficient currentness evidence after refresh.

## Staged source cases

`crates/prw-remote-bridge/tests/reachability_composition_reference.rs` stages these cases:

1. an exact retained candidate can be reachable before refresh, but a successful full refresh resets observations and invalidates the old traversal lifecycle;
2. a late observation from that old lifecycle is rejected even though the same `CandidateId`/path/endpoint still exists;
3. a separately installed replacement lifecycle can then apply a new current observation;
4. candidate-ID rebinding rejection preserves the complete plan and current traversal lifecycle;
5. stale target `TransportIdentity` admission rejection preserves plan and current traversal lifecycle;
6. every later successful full refresh invalidates whichever replacement lifecycle had become current.

## Production interpretation forbidden

This test reference must not be interpreted as selecting:

- `prw-remote-bridge` as the production plan-plus-traversal owner;
- a traversal lifecycle enum/generation representation;
- candidate-publication freshness encoding;
- a production lock/actor/channel/runtime;
- a network adapter;
- ICE restart behavior;
- production Agent/bootstrap wiring.

It proves only the source semantics already locked by C02e.

## Mutation surface

This checkpoint may add only:

- this contract;
- one integration-test source file under existing `prw-remote-bridge/tests`;
- one static audit record.

It must not modify:

- `crates/prw-remote-bridge/Cargo.toml`;
- root `Cargo.toml`;
- `Cargo.lock`;
- `prw-nat-traversal` source or manifest;
- production module exports;
- C02d;
- runtime/bootstrap/network state.

## Validation state

The test source is **staged specification only** while the build gate is closed.

No claim is made that it currently compiles or passes until separately authorized formatting/lint/test/build validation is executed.

Static inspection must still ensure the source imports only dependencies already present in the existing test surface and introduces no runtime/network I/O.

## Next safe seam

After staging this reference source, continue with static review of:

1. whether the test source is structurally consistent with current APIs;
2. whether a future actual Phase 141 composition test can be authorized without changing production ownership, once Cargo/lockfile validation is permitted;
3. whether the abstract publication freshness boundary can later be represented by a separately reviewed verifier-owned source authority.

No build/test dispatch or dependency mutation is authorized by this checkpoint.
