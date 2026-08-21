# Phase 152 C02f-AU — Control-Plane Acquisition Evidence Public Surface Staging

## Purpose

C02f-AU materializes the minimum public Rust type surface required for a later C02f-AT semantic acquisition mapper to consume exact retained C02f-AS/C02f-AR/C02f-AQ evidence from outside `prw-control-plane`.

C02f-AU is a type-surface tranche only. It does not execute provider I/O, submit or reconcile a transaction, construct a client/runtime, activate semantic authority, or deploy anything.

## Exact base

C02f-AU starts from canonical C02f-AT:

`3d92e5d75cffb1cc5ebe0f462fb89b9faf34de3e`

C02f-AT remains the architecture decision for reconciled acquisition semantic mapping.

## Problem being closed

Before C02f-AU, the retained AQ/AR/AS source files existed and were independently validated, but `prw-control-plane::lib.rs` did not declare them as part of the library crate. A later `prw-remote-bridge` mapper therefore could not name the exact C02f-AS handoff/evidence types without duplicating those contracts or bypassing crate boundaries.

C02f-AU closes only this nameability gap.

## Selected effective-visibility design

The public entry point is a narrow facade:

`prw_control_plane::reachability_acquisition_evidence`

The facade re-exports only evidence types required to traverse the exact retained chain:

- `FenceSequenceLiveOwnerAcquisitionHandoff`;
- `FenceSequenceLiveOwnerAcquisitionPlan`;
- `FenceSequenceAllocationResolved`;
- `FenceSequenceAllocationResolvedOutcome`;
- `FenceSequenceAllocationPlan`;
- `FenceSequenceHeadObservation`;
- `FenceSequenceHead`;
- `SequenceAllocationAttemptId`;
- `FenceSequenceTxnCompare`;
- `FenceSequenceTxnOperation`;
- `RecoveryEpoch`.

The facade does not re-export provider execution or orchestration entry points.

## Internal module declarations required by the facade

`prw-control-plane::lib.rs` may privately declare the already-validated implementation modules required to make the re-exported types part of the normal library crate:

- `recovery_epoch`;
- `fence_sequence`;
- `fence_sequence_allocation_etcd`;
- `fence_sequence_allocation_orchestrator`;
- `fence_sequence_live_owner_bridge`;
- `fence_sequence_live_owner_handoff`.

Only `reachability_acquisition_evidence` is the selected new public module boundary.

Private declaration of `fence_sequence_allocation_etcd` is compile-support for the retained AQ module only; it does not make the etcd store externally nameable.

## Explicitly not public through AU

C02f-AU does not expose through the facade:

- `FenceSequenceAllocationEtcdStore`;
- AP execute/reobserve provider calls;
- AQ `FenceSequenceAllocationAuthority`;
- AQ `resolve_fence_sequence_allocation_with_reconciliation`;
- AR planning constructors;
- AS handoff constructors;
- endpoint/client/TLS/auth/RBAC configuration;
- retry/reissue execution entry points;
- recovery-epoch provider execution;
- production authority activation.

A later mapper receives already-produced evidence. C02f-AU does not grant a higher layer a new mutation capability.

## External nameability proof

An integration test compiled as an external crate must prove that the complete retained accessor chain can be named through the facade:

`AS handoff -> AR acquisition -> AQ resolved allocation -> AJ allocation plan -> predecessor head -> recovery epoch`

The test must perform no provider operation and need not construct live authority state.

## Dependency direction

C02f-AU preserves:

`prw-remote-bridge -> prw-control-plane`

It must not add the inverse `prw-control-plane -> prw-remote-bridge` dependency.

No Cargo dependency or lockfile change is selected.

## Explicit non-goals / non-activation boundary

C02f-AU does not:

- implement the C02f-AT semantic result mapper;
- call C02f-AE reconciliation;
- call C02f-AD/AP provider adapters;
- execute an etcd Get/Txn/re-observation;
- contact Spanner or issue a recovery epoch;
- allocate a real production fence sequence;
- generate attempt IDs;
- construct an etcd client or runtime;
- select endpoint/TLS/auth/RBAC/credentials/lease/Watch settings;
- activate `ReachabilityLiveOwnerAsyncAuthority`;
- implement or activate R1-R4 stale-effect rejection;
- modify deployment topology;
- merge a draft PR or deploy.

## Expected source scope

C02f-AU is bounded to:

1. `crates/prw-control-plane/src/lib.rs` — private retained-module declarations plus one public facade declaration;
2. `crates/prw-control-plane/src/reachability_acquisition_evidence.rs` — facade re-exports only;
3. `crates/prw-control-plane/tests/c02f_au_control_plane_acquisition_evidence_public_surface.rs` — external nameability harness;
4. this contract.

All predecessor implementation files remain byte-stable.

## Validation gate

Canonical Rust validation must pass on the exact final AU head, including locked dependency graph, rustfmt, full-workspace Clippy with warnings denied, all tests, and full build. Android validation is required only if repository path filters trigger it for the final AU diff.

Expected gate:

`C02F_AU_CONTROL_PLANE_ACQUISITION_EVIDENCE_PUBLIC_SURFACE_VALIDATED`

A later tranche may implement the pure C02f-AT mapper against this facade. Provider execution/runtime activation remains a separate boundary.
