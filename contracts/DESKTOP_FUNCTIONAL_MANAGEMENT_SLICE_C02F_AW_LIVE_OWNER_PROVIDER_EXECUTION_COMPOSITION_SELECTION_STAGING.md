# Phase 152 C02f-AW — Live-Owner Provider Execution Composition Selection

## Purpose

C02f-AW selects the exact evidence-continuity and failure-mapping contract for a later source tranche that may compose the already-validated C02f-AS acquisition handoff, C02f-AE bounded provider reconciliation, and C02f-AV pure semantic mapper.

C02f-AW is documentation only. It does not call C02f-AE or C02f-AD, submit an etcd transaction, perform a read/re-observation, construct a client/runtime, activate semantic authority, or deploy anything.

The remaining ambiguity after C02f-AV is ownership-sensitive: C02f-AE consumes an exact `LiveOwnerObservation` and exact `ReachabilityLiveOwnerAuthorityRecord` successor, while C02f-AV must still receive the same retained C02f-AS handoff after AE resolves so it can require complete resolved-plan equality. C02f-AW locks how that continuity must be preserved before any provider-execution source is authorized.

## Exact base

C02f-AW starts from canonical C02f-AV:

`ca5bf7135ffbf9827c34fae2a35b6c64566e61da`

C02f-AV remains frozen as the validated pure semantic mapper.

## Selected future composition inputs

A later provider-execution composition may accept only:

1. one already-created mutable `ReachabilityLiveOwnerEtcdStore`; and
2. one exact retained C02f-AS `FenceSequenceLiveOwnerAcquisitionHandoff`.

Endpoint selection, client connection, TLS/auth/RBAC/credential material, runtime ownership and handoff construction remain outside this composition.

The composition must not accept an independently supplied peer, fence, predecessor observation, successor, transaction plan, sequence-allocation attempt identifier, live-owner authority-attempt identifier, or retry policy.

## Exact evidence projection into C02f-AE

The only permitted C02f-AE execution inputs are cloned from the retained C02f-AS handoff without semantic reconstruction:

- `before` is an exact clone of `handoff.observation()`;
- `successor` is an exact clone of `handoff.acquisition().transaction().successor()`.

Cloning is selected only to satisfy C02f-AE ownership while retaining the original handoff for the subsequent C02f-AV equality check. It does not authorize mutation, re-encoding, replanning or replacement of either value.

The later source composition must call exactly the existing bounded acquisition reconciliation entry point:

`ReachabilityLiveOwnerEtcdStore::execute_acquisition_with_reconciliation(before, successor)`

It must not call `plan_acquisition` itself, submit `ReachabilityLiveOwnerEtcdStore::execute` directly, add another re-observation, or implement a second retry/reissue loop around C02f-AE.

C02f-AE remains the sole authority for the existing mandatory re-observation and at-most-one deliberate reissue semantics.

## Exact post-reconciliation mapping

If and only if C02f-AE returns one terminal `ReachabilityLiveOwnerResolvedMutation`, the later composition must pass:

- the same original retained C02f-AS handoff; and
- that exact C02f-AE resolved mutation

to the already-validated C02f-AV:

`map_reconciled_live_owner_acquisition`.

The composition must not recreate a resolved mutation, reinterpret the C02f-AE outcome, bypass C02f-AV plan equality, or manufacture a semantic grant directly.

This preserves the complete evidence chain:

`AS retained observation + AR/AQ transaction evidence -> AE terminal provider evidence -> AV semantic result`.

## Failure mapping before C02f-AV

C02f-AE errors are not terminal resolved mutations and therefore never enter C02f-AV outcome mapping.

A later composition must fail closed as semantic `ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous` for every C02f-AE reconciliation failure, including:

- provider/read/transaction unavailability represented through `ReachabilityLiveOwnerReconciliationError::Etcd`;
- deterministic transaction or authoritative-state contradiction represented through `ReachabilityLiveOwnerReconciliationError::Transaction`;
- exhaustion of the single permitted deliberate reissue represented by `ReachabilityLiveOwnerReconciliationError::ReissueLimitReached`.

None of those errors may be converted to `Contended`, `Granted`, a retry authorization, or local fallback ownership.

`ReachabilityLiveOwnerAuthorityError::FenceExhausted` remains reserved for the already-selected C02f-AV semantic fence conversion failure. The provider-execution composition does not infer `FenceExhausted` from provider/reconciliation failure.

## Definitive outcomes remain owned by C02f-AV

The composition itself does not select or duplicate terminal acquisition semantics.

After successful C02f-AE resolution, C02f-AV remains authoritative for:

- `Committed` -> exact retained semantic `Granted`;
- `CompareFailed` -> `Contended` after exact context checks;
- `Superseded` -> `Contended` after exact context checks;
- resolved-plan, peer or lifecycle contradiction -> `UnavailableOrAmbiguous`;
- impossible semantic fence conversion -> `FenceExhausted`.

No semantic result may be constructed before C02f-AV accepts the exact retained evidence pair.

## Dependency and ownership direction

The selected dependency direction remains:

`prw-remote-bridge -> prw-control-plane`.

C02f-AW does not authorize `prw-control-plane` to depend on `prw-remote-bridge`.

A later source composition belongs on the bridge/orchestration side that already depends downward on the public C02f-AE and C02f-AS evidence types. C02f-AW does not require a new inverse dependency, Cargo dependency or lockfile change.

## Cancellation and retry ownership

A later composition must add no detached task, background retry, timer, retry budget or cleanup mutation around C02f-AE.

Dropping/cancelling the outer future must not spawn follow-up provider work. Any in-call indeterminate-mutation handling remains exactly the bounded C02f-AE behavior already validated; C02f-AW selects no additional mutation submission after C02f-AE returns or is dropped.

## Deliberately unselected neighboring operations

C02f-AW selects acquisition composition only.

It does not select or implement:

- currentness composition;
- release composition;
- a complete `ReachabilityLiveOwnerAsyncAuthority` implementation;
- provider/client construction or ownership lifecycle;
- endpoint/TLS/auth/RBAC/credential configuration;
- recovery-epoch or fence-sequence runtime allocation;
- attempt-ID generation;
- R1-R4 stale-effect enforcement;
- Agent/runtime integration.

Those remain separate boundaries and must not be pulled into an acquisition-only source tranche implicitly.

## Explicit non-goals / non-activation boundary

C02f-AW does not:

- add or modify Rust source;
- call `execute_acquisition_with_reconciliation` at runtime;
- call `ReachabilityLiveOwnerEtcdStore::execute`;
- perform an etcd Get, Txn or re-observation;
- call C02f-AV at runtime;
- construct or connect an etcd client;
- choose endpoints, TLS, authentication, RBAC, credentials, leases, TTL, Watch, users, roles, permissions or cluster membership;
- issue a recovery epoch or contact Spanner;
- allocate or reissue a production fence sequence;
- generate sequence-allocation or live-owner attempt IDs;
- implement or activate `ReachabilityLiveOwnerAsyncAuthority`;
- construct an async runtime, process lifecycle, task, timer or detached future;
- execute traversal/network effects;
- implement or activate R1-R4 stale-effect rejection;
- modify Cargo manifests or `Cargo.lock`;
- deploy or merge a draft PR.

## Exact source scope

C02f-AW adds exactly this one documentation contract. No Rust, workflow, manifest, lockfile, runtime or deployment file is selected.

## Validation gate

C02f-AW is valid only if canonical repository validation remains green on the exact final AW head and a fresh AV -> AW compare proves exactly one documentation-only addition with AV as the exact merge base.

Expected gate after validation:

`C02F_AW_LIVE_OWNER_PROVIDER_EXECUTION_COMPOSITION_SELECTED`

A later separately authorized source tranche may materialize this composition and will, by definition, cross the real provider-I/O execution boundary when invoked. C02f-AW itself does not cross that boundary.
