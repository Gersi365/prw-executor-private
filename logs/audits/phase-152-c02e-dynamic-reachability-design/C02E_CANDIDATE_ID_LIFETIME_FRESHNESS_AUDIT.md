# Phase 152 C02e — Candidate ID Lifetime Freshness Static Audit

Status: `PASS_STATIC_SOURCE_REVIEW / CANDIDATE_ID_LIFETIME_FRESHNESS_STAGED / BOUNDED_HIGH_WATERMARK / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Source corrective commit: `d387abfe8f5cb9df5731d766da1472f6c41389e7`

Parent lifecycle checkpoint: `383d54f17ff9950cd6fc661a29b67b8311208eba`

Frozen C02d checkpoint: `857583b25ed1206317641a93fd8f927819c954d8`

## Concrete contradiction found

The first C02e candidate-ID non-rebinding corrective compared a proposed refreshed candidate only with the candidates currently present in `PeerConnectivityPlan`.

That prevented immediate rebinding, but it did not remember identifiers removed by an earlier successful refresh.

A sequence such as:

`candidate 1 active -> refresh to candidate 2 -> later reuse candidate 1`

could therefore reintroduce identifier `1` during the same plan lifetime.

Phase 141 reachability updates are correlated back to Phase 135 by `CandidateId`. Reusing a retired identifier inside the same plan could allow delayed correlation from the earlier candidate lifetime to alias a later candidate state.

That contradicted the C02e contract statement that a plan-scoped candidate identifier is stable for the complete plan lifetime and that old reachability evidence must not transfer to newly signaled state.

## Minimal corrective staged

`PeerConnectivityPlan` now retains one private bounded value:

`candidate_id_high_watermark: u64`

Construction initializes it to the maximum candidate identifier in the initial validated set, or zero for an empty set.

During refresh:

1. an identifier currently present may survive only when it still denotes the exact same `ConnectivityCandidate`;
2. a newly introduced identifier must be strictly greater than the existing plan high-water mark;
3. an identifier at or below the high-water mark that is not an exact retained current candidate fails with `ConnectivityError::CandidateIdRebound`;
4. complete candidate-set validation finishes before mutation;
5. only a successful refresh advances the high-water mark;
6. failed refresh preserves candidates, observations and high-watermark state atomically.

This uses O(1) retained freshness state rather than an unbounded tombstone/history collection.

## Staged source test — NOT RUN

Added in `crates/prw-connectivity/src/lib.rs`:

`candidate_refresh_rejects_reuse_after_candidate_removal`

The source test specifies:

- initial candidate ID `1`;
- successful replacement with fresh ID `2`;
- later attempt to reintroduce retired ID `1`;
- fail-closed `CandidateIdRebound` result;
- complete plan state unchanged after rejection.

Existing immediate-rebinding and transactional-refresh source tests remain staged.

## Interaction with authenticated candidate publication

The unexported C02e semantic adapter continues to delegate final candidate refresh to `PeerConnectivityPlan`.

Therefore authenticated provenance, current session/workspace/transport admission and lifetime-fresh candidate-ID enforcement compose in one order:

`authenticated/current publication admission -> exact target identity -> lifetime-fresh transactional candidate validation -> endpoint mutation`

An authenticated publication does not bypass candidate lifetime freshness.

## Interaction with transport rotation

Transport-identity rotation is a separate plan-lifecycle boundary locked by the preceding checkpoint.

A replacement plan for a new `TransportIdentity` begins a new plan lifetime. Numeric candidate identifiers from an old plan are not authorization or reachability evidence for the replacement plan, and old traversal/correlation state must not be applied merely because a numeric identifier matches.

This corrective does not create cross-plan identity authority and does not weaken the replacement-plan rule.

## Mutation surface

The corrective modified only:

- `crates/prw-connectivity/src/lib.rs`.

This audit adds only this branch audit record.

It did not modify:

- C02d;
- Cargo manifests or `Cargo.lock`;
- `prw-nat-traversal`;
- control transport framing;
- `prw-remote-bridge` runtime/module exports;
- Agent/bootstrap source;
- Android/Desktop runtime source;
- relay/DNS/forwarding runtime;
- deployment or system state.

## Explicitly not executed

- build;
- `cargo fmt`;
- Clippy;
- tests;
- workflow dispatch;
- real TCP/UDP I/O;
- STUN/ICE/TURN activity;
- QUIC connection/migration;
- PTY/process I/O;
- Agent/bootstrap activation;
- deployment;
- signing;
- privileged/system mutation;
- Host Mirror synchronization.

## Result

`STATIC_SOURCE_REVIEW_PASS / REMOVED_CANDIDATE_ID_CANNOT_RETURN_WITHIN_PLAN / HIGH_WATERMARK_STATE_BOUNDED / TRANSACTIONAL_FAILURE_PRESERVED / TRANSPORT_ROTATION_REPLACEMENT_PLAN_RULE_PRESERVED / WIRE_AND_REPLAY_ADAPTER_UNSELECTED / C02D_UNTOUCHED`
