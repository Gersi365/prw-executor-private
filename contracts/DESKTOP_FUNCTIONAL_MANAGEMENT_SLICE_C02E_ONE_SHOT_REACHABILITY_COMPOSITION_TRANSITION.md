# Phase 152 C02e — One-Shot Reachability Composition Transition

Status: `DESIGN_LOCK / ONE_SHOT_COMPOSITION_SEMANTICS / SUCCESSFUL_REFRESH_INVALIDATES_PRIOR_TRAVERSAL / OBSERVATION_REFRESH_LINEARIZATION_REQUIRED / CONCRETE_OWNER_UNSELECTED / FRESHNESS_REPRESENTATION_UNSELECTED / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Base C02e head: `11c4eea14dd8b48049f634d42034aeb163667014`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

The upper-composition precedent review established that C02e should first lock a bounded one-shot composition transition before selecting a concrete crate, dependency edge, runtime loop, synchronization primitive, or network adapter.

This checkpoint defines that one-shot transition semantically.

It does not add a Rust API because the exact candidate-publication freshness representation and the concrete owner of plan-plus-traversal lifecycle remain intentionally unselected.

## State domains composed

The future one-shot owner coordinates existing authorities without replacing them:

1. `WorkspaceDeviceRegistry` / `AuthenticatedDeviceSession` — current logical-device, workspace and transport-identity admission;
2. authenticated candidate publication semantics — publisher-derived exact peer identity plus bounded candidate vector;
3. verifier-owned candidate-publication freshness — exact-current compare-and-advance authority, representation still unselected;
4. `PeerConnectivityPlan` — current peer plan, transactional candidate refresh, plan-lifetime candidate-ID freshness and observations;
5. Phase 141 traversal lifecycle — the current `IceConnectivitySession`, if one exists, plus any queued/unapplied observation produced by that lifecycle.

No raw endpoint vector, IP address, port, generic `request_id`, or ICE-selected pair may bypass these authorities.

## One-shot publication transition

A future owner must process one candidate publication with a single logically ordered transition.

### Stage A — current identity/admission precheck

Before any candidate-plan or traversal-lifecycle mutation:

1. revalidate requester authenticated session against current registry state;
2. revalidate publisher authenticated session against current registry state;
3. require requester and publisher to share the same current workspace;
4. require publication peer identity to exactly match the current target plan peer;
5. revalidate the publication/plan `TransportIdentity` as registry-current for the publisher device.

Any failure ends the transition with no plan mutation, no traversal invalidation and no freshness advance.

### Stage B — exact-current publication freshness check

The publication must present whatever separately reviewed freshness proof/token is eventually selected.

The owner compares it against verifier-owned current freshness state for the exact publication identity/lifecycle scope.

Stale, duplicate, replayed or concurrently-lost expected state fails before candidate mutation.

Generic Phase 129 `request_id`, `CandidateId`, endpoint equality and current TLS/ICE success are not freshness authority.

### Stage C — complete candidate refresh validation

The complete proposed candidate vector must satisfy the current `PeerConnectivityPlan` refresh rules before any accepted-state commit is exposed:

- global candidate bound;
- duplicate-ID rejection;
- duplicate exact path/endpoint rejection;
- plan-lifetime candidate-ID non-rebinding;
- candidate-ID private high-water freshness;
- exact retained-candidate semantics.

A validation failure must not advance publication freshness or invalidate the current traversal lifecycle.

The implementation mechanism for combining this validation with the future freshness transaction remains unselected; the required external behavior is atomic/fail-closed.

## Commit linearization rule

A successful publication acceptance has one logical commit point.

At that point all of the following become true together from the perspective of later admissions:

1. the refreshed candidate vector is the current `PeerConnectivityPlan` candidate state;
2. all plan reachability observations have the existing refresh-defined post-commit state (`Unknown`);
3. verifier-owned candidate-publication freshness has advanced so the accepted publication cannot commit again;
4. every traversal session established for the preceding candidate-state lifecycle is stale;
5. every queued/unapplied observation produced by such a stale traversal lifecycle is stale and inadmissible.

The concrete lock, transaction, CAS primitive, generation representation, database operation or scheduler mechanism that provides this logical atomicity is not selected here.

## Refresh versus observation race

Candidate refresh and traversal observation admission must be mutually ordered by the future owner.

For any observation racing a successful refresh, exactly one of these outcomes is permitted:

- the observation is admitted against the pre-refresh current lifecycle **before** the refresh commit, after which the successful refresh resets the plan observations to `Unknown`; or
- the refresh commits first, making the producing traversal lifecycle stale, and the observation is rejected before `PeerConnectivityPlan::set_observation(...)`.

The following outcome is forbidden:

`refresh commits -> old traversal observation later repopulates the refreshed plan`

This rule applies even when the candidate is retained exactly with the same `CandidateId`, path kind and endpoint.

`CandidateId` therefore remains candidate correlation only and cannot prove traversal-lifecycle currentness.

## Post-commit traversal state

A successful candidate refresh never leaves the pre-refresh traversal session current.

After commit:

- the old traversal session is stale;
- its queued observations are stale;
- continued traversal requires a replacement session built from the exact committed candidate state and current authenticated coordination metadata.

A replacement traversal session becomes authoritative only after the future owner has associated it with the exact current post-refresh lifecycle.

No previous selected pair or reachability observation may be carried forward as current evidence.

## Replacement traversal construction failure

Replacement traversal establishment is a separately fallible protocol/composition step and must not create rollback ambiguity for an already accepted candidate publication.

If candidate publication freshness and plan refresh have committed successfully but a replacement traversal session cannot be constructed, configured or promoted:

- the accepted candidate plan remains current;
- accepted publication freshness remains advanced;
- the old traversal session remains stale and must not be reactivated;
- no traversal-derived `Reachable` observation is invented;
- the system remains without a current traversal session until a later separately valid replacement attempt succeeds;
- existing Phase 135 selection behavior remains authoritative over the resulting observation state.

The runtime must not roll back to the old candidate publication or old traversal lifecycle merely to regain connectivity.

A future implementation may preconstruct purely in-memory replacement traversal state before the commit if that can be done without making it authoritative or mutating current state. Preconstruction does not alter the commit/invalidation rules above.

## Failed publication transition

If any identity, workspace, target, freshness or candidate validation stage fails before the commit point:

- the existing `PeerConnectivityPlan` remains byte-for-byte semantically current;
- existing candidate observations remain unchanged;
- candidate-ID high-water state remains unchanged;
- verifier publication freshness remains unchanged;
- the current traversal lifecycle remains current;
- queued observations from that still-current lifecycle are not invalidated merely because a rejected publication was attempted.

This preserves the existing C02e rule that failed refresh is non-destructive.

## Concurrent candidate publications

Two publications that race from the same verifier-owned prior freshness state must not both commit.

Exactly one may win the compare-and-advance transition. Any loser must observe stale expected freshness and fail before plan/traversal mutation.

This requirement is independent from transport-frame correlation and from candidate-ID allocation.

## Transport-identity rotation

Transport rotation remains a stronger, separate lifecycle boundary.

If the registry-current `TransportIdentity` differs from the plan/publication identity:

- the old publication transition fails before endpoint mutation;
- the old plan is stale;
- the old traversal lifecycle is stale;
- a replacement plan for the same logical `DeviceId` plus new current `TransportIdentity` is required;
- no candidate freshness, candidate ID, endpoint or traversal state from the stale transport authorizes the replacement identity.

This one-shot endpoint-refresh transition must not mutate the plan peer identity in place.

## Evidence-bearing conceptual outcomes

A future source API should expose enough structured outcome evidence to distinguish at least:

### `RejectedBeforeCommit`

No plan, freshness or traversal-lifecycle state changed.

### `CommittedTraversalReset`

The candidate publication committed, freshness advanced, the prior traversal lifecycle became stale, and no replacement traversal is currently authoritative.

### `CommittedReplacementCurrent`

The candidate publication committed and a separately valid replacement traversal lifecycle for the exact committed state is current.

This checkpoint locks the semantic distinctions only. It does not select Rust enum names or public API shape.

## Security invariants

The future composition must not:

- apply stale traversal observations after a successful refresh;
- reactivate old traversal state after replacement construction failure;
- advance freshness when candidate validation fails;
- mutate candidate state when freshness validation fails;
- allow two publications from one prior freshness state to both commit;
- infer traversal lifecycle from `CandidateId` alone;
- infer publication freshness from `request_id`, endpoint, TLS or ICE success;
- rebind plan peer identity on `TransportIdentity` rotation;
- create a second connectivity plan or second ICE authority;
- choose runtime/thread/async/network/deployment behavior implicitly.

## Deliberately unselected implementation details

This checkpoint does not select:

- concrete upper-owner crate/module;
- candidate-publication freshness type or encoding;
- persistence/transaction backend;
- mutex/RwLock/atomic/channel/task primitive;
- traversal generation/session-ID representation;
- queue implementation or drain mechanism;
- replacement-session retry policy;
- ICE restart API;
- control-plane wire schema;
- socket/network adapter;
- async runtime;
- long-running orchestration loop;
- Agent/bootstrap integration.

## Validation boundary

Static design evidence only.

No Rust source, Cargo manifest, lockfile, build, formatting, lint, test, workflow, network I/O, STUN/ICE/TURN activity, QUIC activity, Agent/bootstrap mutation, deployment, signing, privileged mutation, PR, or merge is performed by this checkpoint.

## Next safe seam

Review repository precedent for **linearizable multi-state one-shot transitions and failure recovery** so a later source-only owner can preserve the commit rules above without inventing a synchronization/transaction architecture.

Until that precedent is reviewed, concrete source placement and synchronization mechanisms remain closed.
