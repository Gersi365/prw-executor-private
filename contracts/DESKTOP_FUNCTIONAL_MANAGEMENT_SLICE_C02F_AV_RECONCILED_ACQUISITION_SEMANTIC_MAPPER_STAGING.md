# Phase 152 C02f-AV — Reconciled Live-Owner Acquisition Semantic Mapper Staging

## Purpose

C02f-AV materializes the pure Rust mapping selected by C02f-AT after C02f-AU made the retained C02f-AS acquisition evidence externally nameable.

The tranche translates exactly one retained C02f-AS acquisition handoff plus exactly one terminal C02f-AE resolved mutation into the already-selected `prw-remote-bridge` semantic acquisition result. It performs no provider I/O and does not activate authority.

## Exact base

C02f-AV starts from canonical C02f-AU:

`627dc4f0e847e4abb6d062fb2ba7fe1bbfb0c744`

C02f-AU remains frozen and is the source of the narrow public evidence facade.

## Exact inputs

The public mapper accepts only:

1. one exact `prw_control_plane::reachability_acquisition_evidence::FenceSequenceLiveOwnerAcquisitionHandoff`; and
2. one exact terminal `prw_control_plane::reachability_live_owner_etcd::reconciliation::ReachabilityLiveOwnerResolvedMutation`.

The mapper does not accept a request-controlled peer, fence, successor, attempt identifier or provider client.

## Mapping order and fail-closed checks

Before interpreting the terminal outcome, the mapper must:

1. compare the complete C02f-AE resolved `LiveOwnerTxnPlan` with the exact transaction retained by the C02f-AS handoff;
2. reject any mismatch as semantic `ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`;
3. obtain the exact expected peer from the authoritative observation retained by the handoff;
4. require the retained transaction successor to be bound to that exact peer;
5. require the retained successor lifecycle to be exactly `LiveOwnerLifecycle::Current`.

This preserves the C02f-AS/AR/AB evidence chain rather than reconstructing authority from higher-layer input.

A C02f-AE `CompareFailed` observation must also remain bound to the same exact peer. Contradictory cross-peer context fails closed as `UnavailableOrAmbiguous`.

## Terminal mapping

### `Committed`

Only after all exact-plan and peer/lifecycle checks pass:

- convert the retained successor's exact non-zero provider fence to `ReachabilityLiveOwnerFence`;
- construct `ReachabilityLiveOwnerGrant::from_authority` from the retained exact peer and exact fence;
- return `ReachabilityLiveOwnerAcquisition::Granted`.

An impossible semantic fence conversion maps to `ReachabilityLiveOwnerAuthorityError::FenceExhausted`.

### `CompareFailed(authoritative_observation)`

After exact-plan and context validation, return:

`ReachabilityLiveOwnerAcquisition::Contended`

No grant is constructed.

### `Superseded`

After exact-plan and context validation, return:

`ReachabilityLiveOwnerAcquisition::Contended`

No grant is constructed.

## Evidence construction boundary

C02f-AV does not add a public constructor for `FenceSequenceLiveOwnerAcquisitionHandoff` or `ReachabilityLiveOwnerResolvedMutation`.

Those types remain evidence produced by their already-validated lower-layer paths. Unit tests exercise a private mapping core using canonical `LiveOwnerTxnPlan` values and terminal outcome variants; they do not make provider-owned resolved evidence forgeable through a new public API.

## Dependency direction

C02f-AV preserves:

`prw-remote-bridge -> prw-control-plane`

No inverse dependency is added. No Cargo dependency or lockfile change is selected.

## Exact source scope

C02f-AV is bounded to:

1. `crates/prw-remote-bridge/src/reachability_live_owner_reconciled_acquisition.rs` — pure mapper and unit tests;
2. `crates/prw-remote-bridge/src/root.rs` — one public module declaration only;
3. this contract.

All AU/control-plane predecessor implementation files remain byte-stable.

## Required test coverage

The mapper test surface must prove at minimum:

1. exact committed plan maps to the exact semantic peer/fence grant;
2. exact compare failure maps to `Contended`;
3. exact superseded result maps to `Contended`;
4. resolved-plan mismatch fails closed before terminal outcome mapping;
5. cross-peer successor context fails closed;
6. non-`Current` successor fails closed;
7. compare-failure observation bound to another peer fails closed.

Tests perform no provider I/O.

## Explicit non-goals / non-activation boundary

C02f-AV does not:

- execute C02f-AD/AЕ provider operations;
- call `ReachabilityLiveOwnerEtcdStore::execute`;
- call `execute_acquisition_with_reconciliation`;
- perform an etcd Get, Txn or re-observation;
- allocate or reissue a fence sequence;
- issue a recovery epoch or contact Spanner;
- construct/connect an etcd client;
- select endpoints, TLS, authentication, RBAC, credentials, leases, TTL, Watch, users, roles, permissions or cluster membership;
- generate sequence-allocation or live-owner attempt IDs;
- implement a provider port;
- implement or activate `ReachabilityLiveOwnerAsyncAuthority`;
- create a runtime, task, timer, process lifecycle or detached future;
- perform traversal/network side effects;
- implement or activate R1-R4 stale-effect rejection;
- modify Cargo manifests or `Cargo.lock`;
- deploy;
- merge a draft PR.

## Validation gate

Canonical Rust validation must pass on the exact final AV head, including locked dependency graph, canonical rustfmt, full-workspace Clippy with warnings denied, all tests and full build. Repository-triggered Android validation must also pass when selected by path filters.

Expected gate:

`C02F_AV_RECONCILED_ACQUISITION_SEMANTIC_MAPPER_VALIDATED`

A later separately reviewed tranche may compose this pure mapper with provider orchestration. C02f-AV itself does not cross that boundary.
