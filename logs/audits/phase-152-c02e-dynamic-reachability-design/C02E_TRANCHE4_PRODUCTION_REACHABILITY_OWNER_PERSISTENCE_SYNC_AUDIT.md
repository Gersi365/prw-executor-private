# C02e Tranche 4 — Production Reachability Owner / Persistence-Synchronization Audit

Status: `IMPLEMENTATION_STAGED / EXECUTABLE_VALIDATION_PENDING`

Tranche 3 closeout head: `6168d500b25627190aa272ff34fdc186465ebc04`
Frozen C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Audited precedent

The implementation was derived from the existing C02e ownership, upper-composition, one-shot-transition and linearization/failure-recovery contracts. Those precedents require an owner above `PeerConnectivityPlan` and `IceConnectivitySession`, validate-before-commit semantics, non-destructive precommit failure, forward-only postcommit recovery and a later durable implementation that preserves equivalent linearizable expected-current semantics.

## Concrete decisions in Tranche 4

- owner placement: `prw-remote-bridge::reachability_owner::ProductionReachabilityOwner`;
- Phase 141 edge: promoted from dev-only to normal local dependency;
- existing Phase 143 bridge source preserved through `src/root.rs` re-export wrapper;
- authenticated publication adapter and Tranche 3 freshness representation exported as production owner inputs;
- persistence abstraction: typed `ReachabilityDurableStore` with `load_current` and exact-current `compare_and_commit`;
- token issuance abstraction: verifier-owned `CandidatePublicationFreshnessTokenSource`;
- traversal replacement abstraction: `ReachabilityTraversalFactory` returning an actual Sans-I/O `IceConnectivitySession`;
- explicit owner modes: `Current`, `RecoveryRequired`, `Retired`.

## Commit proof obligations

The source stages candidate mutation on a cloned plan. It does not mutate the current plan before identity/admission, exact-current freshness, candidate validation and token issuance succeed. The durable CAS is the accepted-state linearization seam. Only a definite durable commit installs the staged plan/freshness and invalidates prior traversal locally.

A definite stale durable expected token or any ambiguous persistence result drops traversal and puts the owner in `RecoveryRequired`. The source never assumes that an uncertain commit failed.

## Recovery and retirement

Recovery loads authoritative durable state for the same exact peer. Absence does not bootstrap. Peer mismatch fails closed. A durable `RecoveryRequired` remains blocked; a durable `Retired` remains terminal for that peer lifecycle.

After registry transport rotation, the old exact peer lifecycle may be CAS-committed to `Retired`; this is a tombstone only and does not bootstrap the replacement transport lifecycle.

## Traversal observations

The owner holds at most one Phase 141 session. A successful candidate commit always removes the previous session. Replacement factory failure is forward-only and leaves the accepted plan/freshness intact with no current traversal.

Observation polling/application stays inside one mutable owner operation and revalidates exact transport currentness immediately before Phase 141 polling/application.

## Deliberate activation boundary

`&mut self` serializes one owner instance and durable CAS arbitrates accepted-state writers. This tranche does not implement distributed live-owner leasing/fencing for transient observations. Therefore network/runtime activation remains closed until a separate runtime-tenancy contract prevents competing live traversal owners for one exact peer lifecycle.

No database product, persistence serialization, wire format, socket, network adapter, async task, Agent/bootstrap path or deployment is selected here.

## Staged executable checks

`crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs` covers:

1. successful durable commit advances freshness and invalidates traversal;
2. candidate validation failure preserves store/freshness/traversal;
3. stale durable CAS forces recovery and authoritative reload;
4. ambiguous persistence result forces fail-closed recovery;
5. postcommit traversal-construction failure does not roll back accepted state;
6. transport rotation permits durable retirement of the old peer lifecycle.

The final classification remains pending until exact-head Cargo metadata, formatting, focused tests/Clippy and full locked workspace validation all pass with drift normalization.
