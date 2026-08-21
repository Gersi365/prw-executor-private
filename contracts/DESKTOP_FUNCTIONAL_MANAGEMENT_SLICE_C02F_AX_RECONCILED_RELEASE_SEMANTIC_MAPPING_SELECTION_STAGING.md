# Phase 152 C02f-AX — Reconciled Release Semantic Mapping Selection

## Purpose

C02f-AX selects the exact provider-neutral semantic mapping from the already-validated C02f-AE bounded release reconciliation result into the existing `ReachabilityLiveOwnerRelease` authority semantics.

C02f-AX is documentation only. It performs no provider I/O, does not execute a release transaction, does not perform a linearizable read or re-observation, does not construct an etcd client/runtime, does not activate the async authority, and does not deploy anything.

The gap after C02f-AW is release-specific: C02f-AE already exposes `ReachabilityLiveOwnerResolvedRelease`, including reconciled `Committed`, `CompareFailed`, and `Superseded` mutation outcomes, while the existing C02f-AC bridge maps only its older definitive release wrapper. A later pure mapper must interpret the reconciled release evidence without replaying provider logic or weakening stale-owner safety.

## Exact base

C02f-AX starts from canonical C02f-AW:

`95d495eb28874ed577e9a60615bfacfdfc175046`

C02f-AW remains frozen as the validated acquisition provider-execution composition selection.

## Selected future mapper inputs

A later pure release semantic mapper may accept only:

1. one exact existing `ReachabilityLiveOwnerGrant`; and
2. one exact C02f-AE `ReachabilityLiveOwnerResolvedRelease`.

The mapper must not accept an independently supplied peer, fence, lifecycle, release successor, transaction plan, observation, attempt identifier, retry policy, provider status, endpoint, or credential.

The grant supplies the semantic peer/fence expectation. The C02f-AE result supplies the provider-owned terminal release evidence.

## `NotCurrent` mapping

If C02f-AE returns:

`ReachabilityLiveOwnerResolvedRelease::NotCurrent`

then the pure mapper returns:

`ReachabilityLiveOwnerRelease::NotCurrent`

No transaction plan exists in this branch, so the mapper must not construct one, infer one, perform another read, or reinterpret `NotCurrent` as a successful release.

`NotCurrent` means the supplied grant is already stale/not exact-current. It is not an authority error and does not prove that this caller performed a release mutation.

## Mutation context validation

If C02f-AE returns:

`ReachabilityLiveOwnerResolvedRelease::Mutation(resolved)`

then the mapper must validate the retained resolved transaction context before interpreting the terminal outcome.

The resolved transaction successor must satisfy all of the following:

- successor peer equals `grant.peer()` exactly;
- successor fence equals the non-zero raw value of `grant.fence()` exactly;
- successor lifecycle is exactly `LiveOwnerLifecycle::Released`.

Any mismatch fails closed as:

`ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`

The mapper must not reconstruct a release plan, call `plan_release`, substitute another grant, or accept a successor for another peer/fence/lifecycle.

## `Committed` mapping

After exact mutation-context validation:

`ReachabilityLiveOwnerResolvedMutationOutcome::Committed`

maps to:

`ReachabilityLiveOwnerRelease::Released`

This is the only C02f-AE reconciled mutation outcome that maps to semantic `Released`.

A `Committed` result must not be inferred from cancellation, provider unavailability, compare failure, supersession, or a local assumption.

## `Superseded` mapping

After exact mutation-context validation:

`ReachabilityLiveOwnerResolvedMutationOutcome::Superseded`

maps to:

`ReachabilityLiveOwnerRelease::NotCurrent`

The superseding authoritative state means the supplied grant is no longer exact-current. It must not map to `Released`, because the retained release mutation itself is not being claimed as the authoritative terminal writer.

No retry, replacement release, local cleanup authority, or success inference is selected by this mapping.

## `CompareFailed` mapping

After exact mutation-context validation, a resolved compare failure carries one authoritative failure-branch `LiveOwnerObservation`.

The mapper must classify that observation using the already-validated deterministic C02f-AB currentness classifier against the exact grant peer and fence:

`classify_currentness(grant.peer(), grant_fence, Some(observation))`

The mapping is:

- `LiveOwnerProviderCurrentness::Stale` -> `ReachabilityLiveOwnerRelease::NotCurrent`;
- `LiveOwnerProviderCurrentness::Current` -> fail closed as `ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`;
- any deterministic classification failure -> fail closed as `ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`.

A compare failure while the authoritative failure observation still classifies the same peer/fence as `Current` is contradictory for semantic release completion and must never become `Released` or `NotCurrent` silently.

## Fence conversion

The mapper may convert the grant's semantic fence to `NonZeroU128` only for deterministic provider currentness classification.

A zero value is structurally impossible for a valid `ReachabilityLiveOwnerFence`, but if conversion cannot be represented safely, the mapper must return:

`ReachabilityLiveOwnerAuthorityError::FenceExhausted`

No new fence is allocated and no fence arithmetic is performed.

## Error ownership

The pure mapper consumes only a terminal C02f-AE result. C02f-AE reconciliation errors are outside this mapper and must be mapped by a later provider-execution composition boundary.

This tranche therefore does not select provider-error retry behavior, etcd error conversion, cancellation semantics around I/O, or any additional reconciliation loop.

All semantic contradictions inside the mapper fail closed. No contradiction may be converted into `Released`, `NotCurrent`, local cleanup success, or a retry authorization.

## Dependency direction

The selected dependency direction remains:

`prw-remote-bridge -> prw-control-plane`

The future pure mapper belongs on the bridge/orchestration side and may consume public C02f-AE evidence types plus the existing provider-neutral live-owner semantic types.

C02f-AX does not authorize `prw-control-plane` to depend on `prw-remote-bridge` and does not select a new Cargo dependency or lockfile mutation.

## Relationship to C02f-AC and C02f-AV

C02f-AC remains the earlier definitive-provider bridge and is not modified by C02f-AX.

C02f-AV remains the validated pure reconciled acquisition semantic mapper and is not modified by C02f-AX.

A future release mapper should be a sibling pure mapping surface, not a rewrite of C02f-AC and not an extension of acquisition-specific C02f-AV semantics.

The selected release chain is:

`exact semantic grant + AE terminal release evidence -> pure release semantic result`

No provider call exists inside that chain.

## Deliberately unselected neighboring work

C02f-AX does not select or implement:

- acquisition provider execution selected by C02f-AW;
- currentness provider execution composition;
- release provider execution composition;
- a complete `ReachabilityLiveOwnerAsyncAuthority` implementation;
- provider/client construction or ownership lifecycle;
- endpoint/TLS/auth/RBAC/credential configuration;
- recovery-epoch or fence-sequence runtime allocation;
- attempt-ID generation;
- R1-R4 stale-effect enforcement;
- Agent/runtime integration.

Those remain separate boundaries.

## Explicit non-goals / non-activation boundary

C02f-AX does not:

- add or modify Rust source;
- perform etcd Get/Txn/re-observation;
- call `execute_release_with_reconciliation`;
- call `ReachabilityLiveOwnerEtcdStore::currentness`;
- connect or construct an etcd client;
- choose endpoints, TLS, authentication, RBAC, credentials, leases, TTL, Watch, users, roles, permissions, or cluster membership;
- issue a recovery epoch or contact Spanner;
- allocate or reissue a production fence sequence;
- generate attempt IDs;
- implement or activate `ReachabilityLiveOwnerAsyncAuthority`;
- construct an async runtime, process lifecycle, task, timer, or detached future;
- execute traversal/network effects;
- implement or activate R1-R4 stale-effect rejection;
- modify Cargo manifests or `Cargo.lock`;
- deploy or merge a draft PR.

## Exact source scope

C02f-AX adds exactly this one documentation contract. No Rust, workflow, manifest, lockfile, runtime, client, provider, or deployment file is selected.

## Validation gate

C02f-AX is valid only if canonical repository validation remains green on the exact final AX head and a fresh AW -> AX compare proves exactly one documentation-only addition with AW as the exact merge base.

Expected gate after validation:

`C02F_AX_RECONCILED_RELEASE_SEMANTIC_MAPPING_SELECTED`

A later non-activating source tranche may materialize this pure mapper without crossing the provider-I/O boundary. Provider-execution composition remains separately gated.
