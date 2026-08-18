# Phase 152 C02e — Test-Only Freshness Lifecycle-State Reference

Status: `SOURCE_SPEC_STAGED / NEW_VS_ESTABLISHED_VS_RECOVERY_REQUIRED_DISTINCT / TEST_ONLY_VERIFIER_BOOTSTRAP / FAILED_FIRST_PUBLICATION_NONCONSUMING / SESSION_RENEWAL_CONTINUES_BOOTSTRAP / TRANSPORT_ROTATION_DISTINCT_BOOTSTRAP / PRODUCTION_REPRESENTATION_UNSELECTED / BUILD_GATE_CLOSED / NOT_EXECUTED`

Base C02e head: `47155cf9f5f615acf3f690609b8e74d417b4d449`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

The bootstrap/re-baseline lifecycle checkpoint requires the upper freshness authority to distinguish a legitimately new peer lifecycle from an existing lifecycle whose current verifier state is unavailable.

This checkpoint stages that distinction in integration-test source using only local non-normative markers. It does not select any production freshness or bootstrap representation.

## Test-only lifecycle states

`crates/prw-remote-bridge/tests/reachability_freshness_bootstrap_reference.rs` models three semantic conditions:

- `NewLifecycleEligible(test bootstrap)`;
- `Established(test current freshness)`;
- `RecoveryRequired`.

These test enum values are not production lifecycle database states, protocol values, counters, nonces, timestamps, or wire fields. They exist only to make the security distinction executable in a later authorized test run.

## Verifier-owned bootstrap reference

For `NewLifecycleEligible`, the reference owner already holds one test bootstrap value. The publisher presents an expected bootstrap value; it does not choose the owner's next state.

The staged first-publication operation preserves the C02e order:

1. current requester/publisher/workspace/target/transport admission through the actual source-only admission helper;
2. require lifecycle to be eligible for new-peer bootstrap rather than established/recovery-required;
3. compare the presented test bootstrap with owner-current test bootstrap;
4. stage complete candidate validation on a private `PeerConnectivityPlan` clone;
5. if validation succeeds, atomically replace the plan and transition the test lifecycle to `Established` inside one exclusive-owner operation.

No fallible candidate operation remains after the staged plan succeeds.

## Failed first publication

A candidate vector may be internally valid as a standalone publication yet violate the target plan's plan-lifetime candidate-ID rules.

The staged source proves that such a first-publication failure:

- leaves the authoritative plan unchanged;
- leaves the owner in `NewLifecycleEligible`;
- preserves the exact test bootstrap state;
- permits a later corrected candidate vector to be evaluated against that same current bootstrap.

This is the test analogue of non-consuming verifier bootstrap failure.

## Recovery-required state

`RecoveryRequired` represents an existing peer lifecycle for which the verifier cannot prove current freshness.

Even if caller input happens to present a value equal to the test bootstrap used by a different new lifecycle, `RecoveryRequired` cannot enter the first-publication path and fails before candidate staging.

This directly proves that:

`missing/unavailable current freshness != legitimate new lifecycle`

No automatic re-baselining is staged.

## Session renewal during bootstrap

The source creates a new authenticated target `SessionId` for the same enrolled device and unchanged current `TransportIdentity` while the reference owner remains in the same `NewLifecycleEligible` state.

The renewed session publishes against the existing verifier-owned bootstrap rather than creating another bootstrap namespace.

Successful first commit moves the one peer lifecycle to `Established`.

## Transport rotation

The source rotates the registry-current target transport identity, constructs a replacement plan for the same `DeviceId + new TransportIdentity`, and gives that replacement owner a distinct test bootstrap marker.

The old test bootstrap marker is rejected for the replacement owner; only the replacement owner's own verifier-held test bootstrap can establish its first publication.

This stages identity-lifecycle separation without selecting how production bootstrap state is encoded or delivered.

## Production interpretation forbidden

This source must not be interpreted as selecting:

- a production bootstrap enum;
- a production initial value;
- a fixed number of freshness states;
- an incrementing generation model;
- a nonce size or lifetime;
- `prw-remote-bridge` as the production owner;
- plan cloning as the production transaction implementation;
- a persistence/recovery schema;
- an automatic same-identity re-baseline path;
- a wire/control message format.

## Mutation surface

This checkpoint may add only:

- this contract;
- `crates/prw-remote-bridge/tests/reachability_freshness_bootstrap_reference.rs`;
- one static audit record.

It must not change Cargo manifests, `Cargo.lock`, production exports, registry/session ownership, Phase 141 source, C02d, runtime/network state, deployment state, or immutable authority.

## Validation state

The source is staged specification only. No rustfmt, compiler, Clippy, test or build evidence exists while the gate remains closed.

## Next safe seam

Perform static API/lint/source review of this lifecycle-state reference. Then review whether C02e has now closed all source/design authorization gaps needed before a separately authorized build/Cargo validation tranche, or whether another semantic gap remains in publication/traversal lifecycle composition.
