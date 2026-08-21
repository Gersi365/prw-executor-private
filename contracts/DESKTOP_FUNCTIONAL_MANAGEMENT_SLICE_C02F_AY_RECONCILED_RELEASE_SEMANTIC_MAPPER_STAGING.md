# Phase 152 C02f-AY — Reconciled Release Semantic Mapper Staging

## Purpose

C02f-AY materializes the provider-neutral pure release mapper selected by C02f-AX, with one bounded fail-closed safety correction discovered during manual evidence-binding review before AY validation.

The mapper consumes one exact semantic `ReachabilityLiveOwnerGrant` plus one terminal C02f-AE `ReachabilityLiveOwnerResolvedRelease` and translates only already-resolved evidence into existing `ReachabilityLiveOwnerRelease` semantics.

C02f-AY performs no provider I/O, no etcd Get/Txn/re-observation, no client/runtime construction, no retry/reissue, no endpoint or credential selection, no async-authority activation, no R1-R4 activation and no deployment.

## Exact base

C02f-AY starts from canonical C02f-AX:

`3cd105f76e949a0f73655d7248b00e60dd3aaf41`

C02f-AX remains frozen as the prior mapping-selection checkpoint.

## Newly discovered AX contradiction

C02f-AX selected top-level C02f-AE `ReachabilityLiveOwnerResolvedRelease::NotCurrent` to map directly to semantic `ReachabilityLiveOwnerRelease::NotCurrent`.

Manual review before AY validation found a concrete evidence-continuity defect in that selected mapping:

- the existing C02f-AE `NotCurrent` variant is a unit variant;
- it retains no peer identity;
- it retains no fence generation;
- therefore, once detached from the provider call that produced it, AY cannot prove that the `NotCurrent` result belongs to the semantic grant supplied to the mapper;
- a terminal `NotCurrent` produced for one peer/fence could otherwise be rebound to another grant.

This is a new concrete contradiction, not a reopening of closed reconciliation work.

C02f-AY therefore supersedes only that one unsafe AX mapping rule and fails the unbound top-level `NotCurrent` variant closed as:

`ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`

No C02f-AE predecessor source is changed in AY. A later separately selected evidence-binding tranche may retain exact peer/fence context in release terminal evidence and restore a safely provable semantic `NotCurrent` mapping.

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

### Unbound top-level NotCurrent

`ReachabilityLiveOwnerResolvedRelease::NotCurrent` has no retained exact peer/fence evidence in the current C02f-AE type.

AY therefore maps it to:

`ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`

It does not manufacture semantic `NotCurrent` from unbound evidence.

### Resolved mutation

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

1. unbound top-level `NotCurrent` -> fail closed;
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

The safe materialized release chain is:

`exact semantic grant + AE terminal mutation evidence -> AY pure release semantic result`

The current unbound AE top-level `NotCurrent` path is intentionally excluded from semantic success until exact release-result binding is materialized separately.

## CI infrastructure status

C02f-AY validation is currently blocked by repository/account-specific GitHub Actions execution failure before the first job step.

On corrected AY head `a108c07237ca293d8f36f3b5b1ecdb64b8948bf8`, canonical zero-step failures were:

- Rust #823 / run `32484718387` / attempt 1 job `96778551783`;
- Android #376 / run `32484718430` / attempt 1 job `96778551911`.

Canonical rerun attempt 2 on that same corrected head reproduced the same pre-step failure pattern:

- Rust #823 / run `32484718387` / attempt 2 job `96786548995`: `failure`, `steps=null`, logs URL `null`;
- Android #376 / run `32484718430` / attempt 2 job `96786565715`: `failure`, `steps=null`, logs URL `null`.

Neither workflow reached checkout or any repository validation command. Direct job-log retrieval for both attempt-2 jobs returned `404 BlobNotFound`, consistent with no job log artifact having been materialized.

GitHub public Actions status is operational, and the repository workflows use GitHub-hosted Ubuntu runners. The observed pattern is therefore an execution/account/quota/budget-class infrastructure blocker rather than a source diagnostic. This contract does not claim a precise billing cause without account billing evidence.

No validation gate is claimed until canonical CI actually executes and passes on the exact final AY head.

## Explicit non-goals / non-activation boundary

C02f-AY does not:

- change C02f-AE predecessor source;
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

Expected gate, explicitly not yet claimed:

`C02F_AY_RECONCILED_RELEASE_SEMANTIC_MAPPER_VALIDATED`

Provider-execution composition and exact release-`NotCurrent` evidence binding both remain separate later boundaries.