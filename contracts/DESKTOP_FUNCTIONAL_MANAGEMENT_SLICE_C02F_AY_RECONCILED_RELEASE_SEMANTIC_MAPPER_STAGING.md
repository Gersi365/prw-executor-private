# Phase 152 C02f-AY — Reconciled Release Semantic Mapper Staging

## Purpose

C02f-AY materializes the provider-neutral pure release mapper selected by C02f-AX.

The mapper consumes one exact semantic `ReachabilityLiveOwnerGrant` plus one terminal C02f-AE `ReachabilityLiveOwnerResolvedRelease` and translates only already-resolved evidence into existing `ReachabilityLiveOwnerRelease` semantics.

C02f-AY performs no provider I/O, no etcd Get/Txn/re-observation, no client/runtime construction, no retry/reissue, no endpoint or credential selection, no async-authority activation, no R1-R4 activation and no deployment.

## Exact base

C02f-AY starts from canonical C02f-AX:

`3cd105f76e949a0f73655d7248b00e60dd3aaf41`

C02f-AX remains frozen as the mapping-selection checkpoint.

## Materialized source

C02f-AY adds:

`crates/prw-remote-bridge/src/reachability_live_owner_reconciled_release.rs`

and exposes that sibling module through:

`crates/prw-remote-bridge/src/root.rs`

No Cargo manifest or lockfile change is required because `prw-remote-bridge` already depends on `prw-control-plane`.

## Public mapper

The source exposes:

`map_reconciled_live_owner_release(grant, resolved)`

with the selected inputs only:

1. one exact `ReachabilityLiveOwnerGrant`; and
2. one exact `ReachabilityLiveOwnerResolvedRelease`.

No independent peer, fence, successor, observation, transaction plan, provider status, retry policy, endpoint, credential or authority-attempt identifier is accepted.

## Exact semantic mapping

`ReachabilityLiveOwnerResolvedRelease::NotCurrent` maps directly to:

`ReachabilityLiveOwnerRelease::NotCurrent`

without constructing or inferring a mutation.

For `ReachabilityLiveOwnerResolvedRelease::Mutation(resolved)`, the mapper validates the retained transaction successor before interpreting the terminal outcome.

The successor must preserve:

- exact `grant.peer()`;
- exact non-zero raw `grant.fence()`;
- exact `LiveOwnerLifecycle::Released` lifecycle.

Any mismatch fails closed as:

`ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`

After exact context validation:

- `Committed` -> `ReachabilityLiveOwnerRelease::Released`;
- `Superseded` -> `ReachabilityLiveOwnerRelease::NotCurrent`;
- `CompareFailed(observation)` -> deterministic C02f-AB currentness classification against the exact grant peer/fence.

For compare failure:

- `Stale` -> `ReachabilityLiveOwnerRelease::NotCurrent`;
- `Current` -> `ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`;
- deterministic classifier rejection -> `ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`.

A compare failure that still proves the exact supplied grant current is contradictory release evidence and never becomes semantic release success.

## Fence representation

The mapper converts the already-authoritative semantic grant fence to `NonZeroU128` solely for deterministic provider currentness classification.

An impossible representation failure maps to:

`ReachabilityLiveOwnerAuthorityError::FenceExhausted`

No fence is allocated, incremented, replaced or accepted from request-controlled input.

## Test coverage

C02f-AY includes pure in-memory unit coverage for:

1. top-level `NotCurrent` -> semantic `NotCurrent`;
2. exact release plan + `Committed` -> `Released`;
3. exact release plan + `Superseded` -> `NotCurrent`;
4. compare failure with stale observation -> `NotCurrent`;
5. compare failure with same exact grant still `Current` -> fail closed;
6. cross-peer release-plan context -> fail closed;
7. different-fence release-plan context -> fail closed;
8. non-`Released` successor -> fail closed;
9. compare-failure observation for another peer -> fail closed.

The tests construct only deterministic in-memory records/plans through already-existing codec and planning helpers. They perform no provider I/O.

## Authority ownership preserved

C02f-AY does not replace or duplicate provider orchestration.

- C02f-AE remains authoritative for bounded indeterminate-mutation reconciliation.
- C02f-AB remains authoritative for deterministic currentness classification and transaction planning invariants.
- C02f-AC remains the earlier definitive-provider semantic bridge.
- C02f-AV remains the reconciled-acquisition pure mapper.
- C02f-AY is only the sibling reconciled-release pure mapper.

The materialized release chain is:

`exact semantic grant + AE terminal release evidence -> AY pure release semantic result`

## Explicit non-goals / non-activation boundary

C02f-AY does not:

- call `ReachabilityLiveOwnerEtcdStore::execute_release_with_reconciliation`;
- call `ReachabilityLiveOwnerEtcdStore::execute`;
- perform etcd Get/Txn/re-observation;
- connect or construct an etcd client;
- select endpoint/TLS/auth/RBAC/credential/lease/TTL/Watch/user/role/permission/cluster configuration;
- execute acquisition provider composition selected by C02f-AW;
- implement currentness provider composition;
- implement release provider composition;
- generate authority attempt IDs;
- allocate or reissue a production fence sequence;
- issue a recovery epoch or contact Spanner;
- implement or activate `ReachabilityLiveOwnerAsyncAuthority`;
- create a runtime, process lifecycle, task, timer, detached future or background retry;
- execute traversal/network effects;
- implement or activate R1-R4 stale-effect rejection;
- change Cargo manifests or `Cargo.lock`;
- deploy;
- merge a draft PR.

## Validation gate

C02f-AY is valid only if canonical repository validation is green on the exact final AY head and a fresh AX -> AY compare proves the bounded source/module/contract scope.

Expected gate:

`C02F_AY_RECONCILED_RELEASE_SEMANTIC_MAPPER_VALIDATED`

Provider-execution composition remains a separate activation boundary after C02f-AY.
