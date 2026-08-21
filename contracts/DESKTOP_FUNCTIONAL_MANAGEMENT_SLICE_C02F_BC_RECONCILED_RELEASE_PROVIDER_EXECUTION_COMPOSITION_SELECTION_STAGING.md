# Phase 152 C02f-BC — Reconciled Release Provider Execution Composition Selection Staging

## Status

Design/selection checkpoint only. This contract selects the future composition from one exact semantic live-owner grant through the already-materialized C02f-AD/C02f-AE provider boundary into the validated C02f-BB release semantic mapper.

This checkpoint performs no provider I/O and materializes no Rust source wiring.

## Exact prerequisite chain

The selected composition depends on already-validated boundaries and does not replace their ownership:

- C02f-AD owns real etcd exact-key linearizable Get and canonical transaction execution over an already-created `KvClient`;
- C02f-AE owns bounded reconciliation for indeterminate mutation outcomes, including mandatory fresh linearizable re-observation and at most one deliberate exact transaction reissue;
- C02f-BA owns provider-created top-level release `NotCurrent` evidence bound to the exact requested peer and non-zero fence;
- C02f-BB owns semantic interpretation of that bound evidence and maps it to semantic `NotCurrent` only after exact peer/fence equality with the supplied semantic grant.

The exact C02f-BB prerequisite head is:

`17a1b7d71372b52ad19184101dd4b9bd72455ee2`

The exact C02f-BB prerequisite tree is:

`7d6d1cdd3ee1c7097e535dd4f8008f67b13a66f3`

## Selected future composition input

A future source composition may accept only:

1. one already-created mutable `ReachabilityLiveOwnerEtcdStore`; and
2. one exact `ReachabilityLiveOwnerGrant` supplied by the semantic authority caller.

The composition must not accept an independently supplied peer, fence, pre-read observation, transaction plan, retry policy, endpoint, credential, runtime, client constructor or semantic release result.

The exact logical peer is always `grant.peer()`.

The exact provider fence is derived only from `grant.fence().get()` and converted to `NonZeroU128`. An impossible representation conversion fails before provider execution as semantic `ReachabilityLiveOwnerAuthorityError::FenceExhausted`.

## Selected provider execution sequence

The future composition is strictly ordered as follows.

### 1. Exact initial linearizable observation

Call only:

`ReachabilityLiveOwnerEtcdStore::linearizable_observation(grant.peer())`

The returned `Option<LiveOwnerObservation>` is retained exactly as returned by C02f-AD and is passed directly into C02f-AE release reconciliation.

No caller-side currentness classification, observation rewriting, re-encoding, peer substitution, fence substitution or release planning is selected before C02f-AE.

Any C02f-AD read/codec/shape/transaction-classification error from this initial observation maps fail-closed to:

`ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`

No initial-read error may become `Released`, `NotCurrent`, retry authorization or local fallback authority.

### 2. Sole release mutation/reconciliation entry point

Call only:

`ReachabilityLiveOwnerEtcdStore::execute_release_with_reconciliation(grant.peer(), raw_fence, observation)`

where:

- `grant.peer()` is the exact peer from the same semantic grant;
- `raw_fence` is the exact non-zero provider representation of the same grant fence; and
- `observation` is the exact result of step 1.

Direct use of `ReachabilityLiveOwnerEtcdStore::execute` is not selected for this composition.

No outer release planning, transaction reconstruction, direct Txn submission, extra provider re-observation, blind retry, additional reissue loop, successor reconstruction or semantic result manufacture is selected.

C02f-AE remains the sole owner of:

- deterministic release planning from the supplied exact observation;
- deciding whether the release is already not current and requires no mutation;
- provider-owned bound `NotCurrent` evidence construction;
- exact retained release transaction context;
- mandatory fresh linearizable re-observation after an indeterminate transaction result;
- the at-most-one deliberate exact transaction reissue; and
- terminal `Committed`, `CompareFailed` or `Superseded` resolved mutation evidence.

A missing initial observation remains fail-closed through the existing C02f-AE/C02f-AB `MissingEstablishedState` transaction error. This composition does not reinterpret absence as semantic `NotCurrent`.

A peer mismatch in the observed authoritative record remains a C02f-AE/C02f-AB error and must not mint bound `NotCurrent` evidence.

### 3. Exact semantic mapping

After C02f-AE returns one terminal `ReachabilityLiveOwnerResolvedRelease`, pass:

- the same original `ReachabilityLiveOwnerGrant`; and
- that exact returned `ReachabilityLiveOwnerResolvedRelease`

directly to:

`map_reconciled_live_owner_release(grant, &resolved)`

No resolved-release reconstruction, evidence copying, peer/fence rebinding, outcome rewriting or direct semantic result manufacture is selected.

C02f-BB remains the sole owner of the top-level bound-evidence semantic rule:

- exact bound evidence peer == grant peer AND exact evidence fence == grant fence -> semantic `NotCurrent`;
- any peer/fence mismatch -> `UnavailableOrAmbiguous`.

The existing mutation-backed C02f-AY rules remain authoritative:

- exact Released successor + `Committed` -> semantic `Released`;
- exact Released successor + `Superseded` -> semantic `NotCurrent`;
- exact Released successor + `CompareFailed` that classifies the supplied grant stale -> semantic `NotCurrent`;
- contradictory/current/ambiguous mutation context -> `UnavailableOrAmbiguous`.

## Selected error boundary

The future composition must map every C02f-AE `ReachabilityLiveOwnerReconciliationError` variant fail-closed:

- `Etcd(_)` -> `ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`;
- `Transaction(_)` -> `ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`;
- `ReissueLimitReached` -> `ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`.

No C02f-AE reconciliation error may become `Released`, `NotCurrent`, retry authorization, local authority or fallback success.

`FenceExhausted` remains reserved for impossible semantic/provider fence representation conversion at the semantic boundary; it is not a generic provider-error mapping.

## C02f-AC legacy definitive bridge boundary

The earlier `ReachabilityLiveOwnerProviderBridge` / `ReachabilityLiveOwnerDefinitiveProviderPort` path from C02f-AC remains unchanged but is **not selected** as the future reconciled release composition.

In particular, the future C02f-BC source composition must not:

- translate C02f-AE `ReachabilityLiveOwnerResolvedRelease` into `ReachabilityLiveOwnerDefinitiveRelease`;
- discard C02f-BA peer/fence binding into the older unit `ReachabilityLiveOwnerDefinitiveRelease::NotCurrent` variant;
- route the reconciled release result through `map_definitive_release`; or
- otherwise bypass C02f-BB exact evidence matching.

This separation preserves the provider-owned peer/fence provenance introduced by C02f-BA and validated semantically by C02f-BB.

No deletion or compatibility mutation of the C02f-AC bridge is selected in C02f-BC.

## Dependency and ownership boundary

The dependency direction remains:

`prw-remote-bridge -> prw-control-plane`

No inverse dependency is selected.

No Cargo dependency or `Cargo.lock` change is selected.

A future source composition, if separately authorized, belongs on the bridge/orchestration side because it combines:

- an already-created control-plane store;
- an exact semantic grant;
- C02f-AD initial provider observation;
- C02f-AE reconciled provider execution; and
- C02f-BB semantic mapping.

Provider endpoint selection, client connection/bootstrap, TLS/auth/RBAC/credential ownership, runtime/executor ownership and process lifecycle remain outside this composition.

## Explicitly unselected in C02f-BC

This checkpoint does not select or activate:

- Rust source materialization of the release composition;
- any actual `linearizable_observation` call;
- any actual `execute_release_with_reconciliation` call;
- any etcd Get, Txn or re-observation;
- any direct C02f-AD `execute` call;
- any extra retry/reissue or background task;
- endpoint or cluster selection;
- etcd client construction or `Client::connect`;
- TLS, auth, RBAC, credentials, users, roles or permissions;
- lease/TTL/Watch behavior;
- currentness provider composition;
- acquisition source composition selected earlier by C02f-AW;
- a complete concrete `ReachabilityLiveOwnerAsyncAuthority` implementation;
- recovery epoch issuance or Spanner contact;
- fence allocation/reissue or attempt-ID generation;
- Agent/runtime integration;
- R1-R4 effect-side stale-fence rejection activation;
- deployment; or
- merge.

## Future source-materialization authorization boundary

Materializing the selected sequence in Rust would create a callable composition that performs real provider I/O when polled. That source step therefore remains a separate authorization boundary, consistent with C02f-AW.

Until separately authorized, this checkpoint claims only a deterministic composition selection and evidence/error ownership contract.

## Selected future evidence chain

`exact semantic grant -> C02f-AD exact initial linearizable observation -> C02f-AE bound/reconciled terminal release evidence -> C02f-BB exact semantic release result`

No intermediate stage may weaken, replace or reconstruct the peer/fence provenance required by the next stage.

## Validation gate

The selection gate may be claimed only after canonical executable Rust validation passes on the exact C02f-BC documentation head:

`C02F_BC_RECONCILED_RELEASE_PROVIDER_EXECUTION_COMPOSITION_SELECTED`
