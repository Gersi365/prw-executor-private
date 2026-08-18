# Phase 152 C02e — Traversal Session / Candidate Refresh Lifecycle Static Audit

Status: `PASS_STATIC_CORRECTIVE_REVIEW / OLD_TRAVERSAL_SESSION_INVALID_AFTER_REFRESH / RETAINED_CANDIDATE_STALE_OBSERVATION_GAP_LOCKED / RUNTIME_CORRELATION_UNSELECTED / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Checkpoint commit: `dd54cd337907a5d7685ae9722d4ba3e9dc05fea1`

Review base: `4b250701b53dc0cf690f4e87e751a28ed0ddab1b`

Frozen C02d checkpoint: `857583b25ed1206317641a93fd8f927819c954d8`

## Concrete finding

The preceding source-only integration review correctly verified that Phase 141 cannot invent a new candidate and that an observation for a candidate absent from the refreshed plan fails closed.

Static inspection found an additional retained-candidate case:

- `PeerConnectivityPlan::refresh_candidates(...)` resets every observation to `Unknown` after success;
- a candidate may be retained exactly across refresh with the same plan-lifetime `CandidateId`;
- Phase 141 `CandidateReachabilityUpdate` carries only that `CandidateId` plus `ReachabilityObservation`;
- `CandidateReachabilityUpdate::apply(...)` checks only whether the ID exists in the current plan;
- therefore an unapplied update produced by the pre-refresh `IceConnectivitySession` can still apply after refresh when that exact candidate remains present.

Candidate-ID lifetime freshness does not distinguish traversal-session age for an exactly retained candidate.

## Required lifecycle correction

A successful full candidate refresh must invalidate the preceding traversal-observation lifecycle.

All old Phase 141 session objects and queued/unapplied observations produced from them become stale at the refresh boundary, regardless of whether one or more candidates are retained exactly.

A continuing traversal integration must establish a replacement traversal session from the refreshed candidate state before accepting new authoritative observations.

The later composition owner must prevent any race in which an old traversal session publishes an observation after the candidate refresh has become current.

## Existing source remains unchanged

No `prw-nat-traversal` source mutation was made.

This is deliberate:

- Phase 141 remains the bounded Sans-I/O protocol/correlation authority it already is;
- C02e plan/traversal lifecycle composition does not currently have a production owner;
- choosing a traversal epoch/generation, cancellation channel, async-task owner or queue-drain primitive would invent runtime architecture not fixed by repository precedent.

The correct current state is a locked requirement plus fail-closed runtime boundary, not premature runtime wiring.

## Separation from publication replay

Candidate-publication freshness prevents stale candidate **sets** from becoming current.

Traversal-session freshness prevents stale reachability **observations** from a superseded traversal lifecycle from repopulating current plan state.

They are independent state-currentness requirements and must both be enforced in future runtime composition.

## Transport rotation

Transport rotation already forces replacement of the entire connectivity plan. That transition also invalidates every traversal session and observation associated with the old transport identity.

No candidate number, endpoint equality or old traversal success can bridge the old transport lifecycle into the replacement plan.

## Mutation surface

Added only:

- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02E_TRAVERSAL_SESSION_REFRESH_LIFECYCLE_CHECKPOINT.md`;
- this audit record.

No existing Rust source, Cargo manifest, lockfile, Phase 141 source, Agent/bootstrap source, C02d source or production state was modified by this corrective checkpoint.

## Explicitly not executed

- build;
- `cargo fmt`;
- Clippy;
- tests;
- workflow dispatch;
- real TCP/UDP I/O;
- STUN/ICE/TURN execution;
- QUIC connection/migration;
- PTY/process I/O;
- runtime/bootstrap activation;
- deployment;
- signing;
- privileged/system mutation;
- Host Mirror synchronization;
- PR creation/merge.

## Result

`STATIC_CORRECTIVE_REVIEW_PASS / CANDIDATE_ID_FRESHNESS_NOT_TREATED_AS_TRAVERSAL_SESSION_FRESHNESS / SUCCESSFUL_REFRESH_INVALIDATES_OLD_TRAVERSAL_OBSERVATIONS / RETAINED_CANDIDATE_OLD_UPDATE_NOT_AUTHORIZED / ENFORCEMENT_OWNER_AND_MECHANISM_UNSELECTED / C02D_UNTOUCHED`
