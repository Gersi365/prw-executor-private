# Phase 152 C02e — Corrected Freshness Bootstrap Reference Static Source Review

Status: `PASS_STATIC_SOURCE_REVIEW / NEW_ESTABLISHED_RECOVERY_REQUIRED_STATES_COHERENT / REPLACEMENT_FIXTURE_CORRECTED / ADMISSION_BEFORE_BOOTSTRAP / CANDIDATE_STAGING_NONCONSUMING_ON_FAILURE / BUILD_GATE_CLOSED / NOT_COMPILED / NO_NETWORK_IO`

Reviewed head: `2ad4bf035856324112404004d1ae430cfc96b1fd`

Initial staging commit: `214b091d0e9171131e37ac37bf8db2a52dd1bc06`

Fixture corrective commit: `2ad4bf035856324112404004d1ae430cfc96b1fd`

Frozen C02d checkpoint: `857583b25ed1206317641a93fd8f927819c954d8`

## Source reviewed

`crates/prw-remote-bridge/tests/reachability_freshness_bootstrap_reference.rs`

## Lifecycle-state review

The source keeps three explicit test-only lifecycle conditions:

- `NewLifecycleEligible(test bootstrap)`;
- `Established(test freshness)`;
- `RecoveryRequired`.

The commit operation always executes the actual current identity/workspace/target/transport admission helper before interpreting these test lifecycle states.

`RecoveryRequired` cannot alias `NewLifecycleEligible`, and an already `Established` lifecycle cannot replay the first-publication path.

## Candidate validation / bootstrap consumption review

For a matching new-lifecycle bootstrap, the complete candidate vector is validated against a private cloned `PeerConnectivityPlan`.

Only successful staged validation replaces the authoritative plan and changes lifecycle to `Established`.

A target-plan `CandidateIdRebound` error therefore leaves both the authoritative plan and exact test bootstrap lifecycle unchanged.

The staged corrected publication then demonstrates retry under the same still-current verifier bootstrap.

## Session renewal review

The test creates a second authenticated target `SessionId` using the same enrolled binding and unchanged current `TransportIdentity`.

The new session publishes against the already-existing new-peer bootstrap lifecycle; no second bootstrap authority is created.

This matches the locked rule that session renewal changes provenance context but not peer publication freshness lifecycle.

## Transport-rotation fixture corrective

Static inspection of the initially staged transport-rotation test found that:

- the replacement publication used `CandidateId(1)` at port `4001`;
- the replacement plan used the same `CandidateId(1)` at port `4000`.

With a correct bootstrap this would fail for `CandidateIdRebound`, obscuring the intended replacement-bootstrap behavior.

The corrective changes only the replacement-plan fixture endpoint from port `4000` to `4001`, making the retained replacement candidate exact.

The test now cleanly stages:

1. old test bootstrap -> `BootstrapMismatch` before candidate staging;
2. replacement test bootstrap -> exact candidate refresh succeeds and establishes replacement lifecycle freshness.

No production semantic or candidate-ID rule was weakened.

## Static type/API surface

By inspection:

- current registry transport rotation signature matches fixture use;
- the authenticated target session remains valid logical identity context after transport rotation, while the presented replacement `TransportIdentity` is separately revalidated by publication construction;
- replacement `PeerConnectivityPlan` identity exactly matches the new current registry transport identity;
- source imports are already present in the `prw-remote-bridge` test dependency surface;
- no actual Phase 141 dependency or runtime operation is introduced.

No obvious remaining ownership/move contradiction was found in the reviewed source.

## Evidence limitation

This is static inspection, not compiler/rustfmt/Clippy/test/build evidence. The source remains staged until the closed validation gate is separately opened.

## Result

`STATIC_SOURCE_REVIEW_PASS / NEW_PEER_BOOTSTRAP_RECOVERY_REQUIRED_AND_ESTABLISHED_STATES_REMAIN_DISTINCT / FAILED_FIRST_CANDIDATE_VALIDATION_PRESERVES_BOOTSTRAP / REPLACEMENT_TRANSPORT_TEST_NOW_ISOLATES_BOOTSTRAP_IDENTITY_FROM_CANDIDATE_REBINDING / NO_PRODUCTION_EDGE / C02D_UNTOUCHED`
