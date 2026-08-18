# Phase 152 C02e — One-Shot Reachability Composition Static Audit

Status: `PASS_STATIC_DESIGN_REVIEW / ONE_SHOT_COMMIT_RULE_LOCKED / OLD_TRAVERSAL_INVALID_AFTER_SUCCESSFUL_REFRESH / OBSERVATION_REFRESH_RACE_FAIL_CLOSED / REPLACEMENT_FAILURE_NO_ROLLBACK / OWNER_AND_FRESHNESS_REPRESENTATION_UNSELECTED / BUILD_GATE_CLOSED / NO_NETWORK_IO`

Design base head: `11c4eea14dd8b48049f634d42034aeb163667014`

Frozen C02d checkpoint: `857583b25ed1206317641a93fd8f927819c954d8`

## Evidence reviewed

- current C02e upper reachability composition precedent review;
- current C02e traversal-session/candidate-refresh lifecycle checkpoint;
- current candidate-publication freshness/replay checkpoint;
- current candidate-ID lifetime freshness behavior;
- current transport-rotation replacement-plan lifecycle;
- `PeerConnectivityPlan::refresh_candidates(...)` transactional/reset semantics;
- Phase 141 `CandidateReachabilityUpdate` correlation/application semantics;
- current registry/session authenticated candidate publication admission ordering.

## Findings

1. A successful full candidate refresh is necessarily a traversal-observation lifecycle boundary, including for exactly retained candidates.
2. Candidate refresh and old traversal observation admission therefore require a single logical ordering boundary owned above `PeerConnectivityPlan` and `IceConnectivitySession`.
3. If an old observation wins before refresh, refresh resets the resulting observation state; if refresh wins first, the old observation must be rejected before plan mutation.
4. `CandidateId` cannot act as traversal-session freshness because an exactly retained candidate keeps the same ID across a successful refresh.
5. Publication freshness, plan refresh and old-traversal invalidation must appear as one accepted commit to later operations, while the concrete transaction primitive remains unselected.
6. Any pre-commit identity/workspace/target/freshness/candidate failure preserves current plan, observations, candidate-ID high-water state, publication freshness and traversal lifecycle.
7. Replacement traversal construction is separate from acceptance of the candidate publication. If replacement establishment fails after commit, the accepted candidate state remains current and freshness remains advanced; the old traversal stays stale and cannot be reactivated.
8. Two candidate publications racing from the same prior verifier freshness state must not both commit.
9. Transport-identity rotation remains outside endpoint-refresh semantics and requires replacement-plan handling.
10. Current evidence supports semantic outcome classes (`rejected before commit`, `committed with traversal reset`, `committed with current replacement`) but does not authorize concrete Rust enum/API naming or source placement.

## Fail-closed recovery rule

After an accepted publication commit, replacement-traversal failure leaves the system with:

- current refreshed candidate plan;
- advanced publication freshness;
- old traversal lifecycle stale;
- no authoritative replacement traversal until a later valid establishment succeeds;
- no invented reachability observation.

Rollback to the old publication/traversal is forbidden because it would make already superseded reachability state authoritative again.

## Mutation surface

Added only:

- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02E_ONE_SHOT_REACHABILITY_COMPOSITION_TRANSITION.md`;
- this static audit record.

No existing Rust source, Cargo manifest, lockfile, C02d source, Phase 141 source, Agent/bootstrap source, runtime/network state, deployment state, or immutable authority is changed by this checkpoint.

## Not executed

- build;
- `cargo fmt`;
- Clippy;
- tests;
- workflow dispatch;
- TCP/UDP I/O;
- STUN/ICE/TURN activation;
- QUIC connection/migration;
- runtime/bootstrap wiring;
- deployment;
- signing;
- privileged/system mutation;
- PR creation/merge;
- Host Mirror synchronization.

## Result

`STATIC_DESIGN_REVIEW_PASS / ONE_SHOT_REACHABILITY_COMMIT_SEMANTICS_LOCKED / REFRESH_AND_OBSERVATION_MUST_LINEARIZE / SUCCESS_INVALIDATES_OLD_TRAVERSAL / REPLACEMENT_FAILURE_PRESERVES_NEW_PLAN_AND_NEVER_REACTIVATES_OLD_TRAVERSAL / CONCRETE_OWNER_SYNC_PRIMITIVE_AND_FRESHNESS_REPRESENTATION_REMAIN_UNSELECTED / C02D_UNTOUCHED`
