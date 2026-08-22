# Phase 152 C02f-BR — Common Acquisition Composition Selection Staging

## Status

Documentation-only architecture-selection checkpoint after fully validated C02f-BQ.

C02f-BR selects the common acquisition composition that may later join the already-materialized C02f-BM preparation facade with the separately validated C02f-BP first-owner provider path and C02f-BQ replacement provider path.

This checkpoint does not add or modify Rust source, execute provider I/O, create clients/endpoints/runtime state, activate `ReachabilityLiveOwnerAsyncAuthority`, activate R1-R4 effects, deploy, or merge.

## Approval basis

The user explicitly authorized:

`Autorizoj C02f-BR common acquisition composition architecture selection.`

That authorization is limited to this documentation-only architecture selection. A later source-materialization checkpoint requires a separate explicit authorization.

## Exact base

C02f-BR starts from exact validated C02f-BQ:

- branch `phase-152-c02f-bq-replacement-provider-execution-materialization-staging`;
- head `8ea007644803f98dea81db5d0bd9dae6bdd9ff82`;
- tree `be000f28be5c3066b511ee2df9b199a520a4ddfc`;
- gate `C02F_BQ_REPLACEMENT_PROVIDER_EXECUTION_MATERIALIZED`.

C02f-BP remains the validated first-owner provider-execution/mapping checkpoint and C02f-BQ remains the validated replacement provider-execution/mapping checkpoint. C02f-BR composes them; it does not redesign either closed path.

## Authoritative inherited contracts

C02f-BR preserves these already-selected boundaries:

1. C02f-X: `prw-remote-bridge` owns asynchronous reachability/live-owner orchestration; `prw-control-plane` owns provider-specific etcd behavior; dependency direction remains `prw-remote-bridge -> prw-control-plane`; no inverse dependency is allowed.
2. C02f-BJ/BM: one preparation operation accepts only exact `PeerConnectivityIdentity`, retains the initial live-owner predecessor-or-absence context, performs the bounded AQ fence-sequence allocation protocol, generates typed attempt IDs internally, and returns exactly `Replacement | FirstOwner | Superseded` prepared evidence.
3. C02f-BJ/BM: the same provider context must back fence-sequence and live-owner operations; callers must not independently assemble stores that may target different authority backends.
4. C02f-AW/BQ: replacement execution consumes the exact retained AS handoff, projects only its retained predecessor and successor into AE, performs no replanning or outer retry, and maps the exact AE terminal evidence through AV.
5. C02f-BO/BP: first-owner execution consumes the exact retained first-owner handoff, uses the bounded BO create-only reconciliation protocol, and maps exact terminal evidence through the BP mapper.
6. A committed AQ allocation is consumed even if later live-owner preparation/execution contends or fails. No common layer may allocate another fence or regenerate attempt IDs inside the same logical acquisition.

## Remaining composition problem

At C02f-BQ, both live-owner provider branches exist, but they are intentionally separate.

C02f-BM currently owns its live-owner store privately inside `ReachabilityLiveOwnerAcquisitionPreparation`. C02f-BQ accepts an already-created mutable `ReachabilityLiveOwnerEtcdStore`, while C02f-BP first-owner execution is also a method on that store.

The common acquisition layer therefore needs a way to execute the selected BP/BQ branch against the exact live-owner provider context already owned by the BM preparation facade without:

- exposing an unrestricted `live_owner_mut()`-style accessor;
- constructing or accepting a second independent live-owner store;
- constructing or cloning a second endpoint/client authority context in the bridge;
- moving bridge semantic mapping into `prw-control-plane`;
- introducing an inverse `prw-control-plane -> prw-remote-bridge` dependency;
- reimplementing BP/BQ provider state machines.

C02f-BR selects that ownership seam below.

## Selected common public acquisition input

The later bridge-side common composition accepts only:

1. one mutable `ReachabilityLiveOwnerAcquisitionPreparation`; and
2. one exact borrowed `PeerConnectivityIdentity`.

Conceptually:

```rust
fn acquire_prepared_live_owner<'a>(
    preparation: &'a mut ReachabilityLiveOwnerAcquisitionPreparation,
    peer: &'a PeerConnectivityIdentity,
) -> impl Future<
    Output = Result<ReachabilityLiveOwnerAcquisition, ReachabilityLiveOwnerAuthorityError>,
> + Send + 'a;
```

Exact source naming may be adjusted mechanically at materialization, but the input/ownership semantics may not be widened.

The common composition must not accept independently supplied:

- predecessor observation or absence assertion;
- fence/recovery epoch/sequence;
- sequence-allocation attempt ID;
- authority-attempt ID;
- replacement or first-owner transaction plan;
- retained handoff from an unrelated preparation instance;
- provider result;
- retry/reissue budget;
- semantic grant;
- endpoint/client/TLS/auth/RBAC configuration.

## Selected orchestration order

For one logical common acquisition operation, the bridge must perform exactly this order:

1. call the existing BM `preparation.prepare(peer)` once;
2. await that preparation to one terminal `ReachabilityLiveOwnerPreparedAcquisition` or preparation error;
3. do not refresh/re-read/re-plan the retained preparation context in the common layer;
4. branch only on that exact BM terminal prepared result;
5. if `Superseded`, return semantic `ReachabilityLiveOwnerAcquisition::Contended` immediately;
6. if `Replacement(handoff)`, execute exactly the already-selected BQ replacement chain against the BM-owned live-owner provider and return its semantic result unchanged;
7. if `FirstOwner(handoff)`, execute exactly the BO/BP first-owner provider reconciliation against the BM-owned live-owner provider, then pass that exact resolved evidence to the BP semantic mapper and return its semantic result unchanged.

No common-layer retry, second preparation, second fence allocation, second attempt-ID generation, provider fallback, or branch conversion is selected.

## Selected BM-owned provider borrowing seam

C02f-BR selects a **non-escaping scoped async execution borrow** from `ReachabilityLiveOwnerAcquisitionPreparation`.

The future source may expose one narrow operation conceptually equivalent to:

```rust
preparation.with_live_owner_execution(|store| async move {
    // bridge-selected BP or BQ execution only for this awaited scope
}).await
```

The semantic requirements are exact:

1. the borrowed store is the same `ReachabilityLiveOwnerEtcdStore` already owned by the BM preparation facade and used for its initial authoritative live-owner observation;
2. the borrow exists only for the awaited execution scope;
3. the facade retains ownership before and after the scoped call;
4. the API must not return `&mut ReachabilityLiveOwnerEtcdStore`, `KvClient`, endpoint/configuration state, or another handle that can escape the scope;
5. there is no `Deref`/`DerefMut` provider escape hatch and no unrestricted `live_owner_mut()` accessor;
6. the common bridge composition cannot independently replace the store with a different provider context;
7. no second `ReachabilityLiveOwnerEtcdStore` is constructed merely to execute the prepared branch;
8. no second `KvClient`, `Client::connect`, endpoint selection, TLS/auth/RBAC, or credential lookup is introduced by this seam.

The exact Rust spelling of the non-escaping async callback/capability may be adjusted mechanically to satisfy the repository toolchain, but it must preserve all eight properties above. If the source checkpoint cannot express the non-escaping scoped borrow under the selected native `impl Future + Send` model without weakening these properties or adding mandatory boxed/dynamic futures, implementation must stop for a compiler/API selection checkpoint rather than exposing a raw mutable store accessor.

## Replacement branch continuity

For `ReachabilityLiveOwnerPreparedAcquisition::Replacement(handoff)`, the common composition must reuse the C02f-BQ behavior without semantic reconstruction.

The provider execution remains exactly:

- `before = handoff.observation().clone()`;
- `successor = handoff.acquisition().transaction().successor().clone()`;
- one call to existing AE `execute_acquisition_with_reconciliation(before, successor)`;
- no `plan_acquisition`;
- no direct generic lower `execute` call;
- no extra live-owner Get/re-observation;
- no outer retry/reissue;
- exact AE terminal evidence + the same original handoff into AV `map_reconciled_live_owner_acquisition`.

A later source tranche may mechanically factor BQ internals only if the existing public BQ behavior, evidence continuity and tests remain byte-for-byte/semantically equivalent at the boundary. BR does not authorize replacement state-machine redesign.

## First-owner branch continuity

For `ReachabilityLiveOwnerPreparedAcquisition::FirstOwner(handoff)`, the common composition must:

1. pass that exact retained handoff to the already-materialized BO/BP provider execution on the BM-owned live-owner store;
2. permit only BO's bounded create-only reconciliation semantics, including at most one deliberate identical reissue after authoritative `ProvenNotCommitted`;
3. retain the exact provider-resolved first-owner evidence returned by that execution;
4. pass that exact resolved evidence directly to `map_resolved_first_owner_acquisition`;
5. return the BP mapper result unchanged.

The common layer must not manufacture a predecessor, change the `version == 0` compare, create a new successor, regenerate an attempt ID, reinterpret CompareFailed/Superseded provenance, or manufacture `Granted`/`Contended` itself for this branch.

## Superseded preparation semantics

BM `Superseded` means the exact AQ sequence allocation lost authority before a live-owner mutation was prepared.

C02f-BR selects the later semantic composition:

`ReachabilityLiveOwnerPreparedAcquisition::Superseded -> ReachabilityLiveOwnerAcquisition::Contended`

This mapping performs no live-owner mutation and no live-owner provider read. It must never produce `Granted`, allocate another sequence, or recursively call `prepare(peer)` within the same logical acquisition.

The superseded allocation/sequence attempt remains consumed according to BJ/BM rules.

## Failure mapping

C02f-BR selects fail-closed common failure mapping:

- every `ReachabilityLiveOwnerPreparationError` -> `ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`;
- every replacement AE/provider/reconciliation error remains BQ fail-closed `UnavailableOrAmbiguous`;
- every first-owner provider/reconciliation error -> `UnavailableOrAmbiguous`;
- AV/BP semantic mapper results/errors are returned unchanged;
- `FenceExhausted` may arise only from an already-selected semantic fence-representation conversion in the validated semantic mappers, not from provider unavailability, preparation failure, retry exhaustion, compare failure, or supersession.

No error may authorize local ownership, fallback ownership, another fence allocation, or another live-owner attempt within the same common acquisition call.

## Evidence continuity

The selected evidence chains remain distinct and exact:

```text
Replacement:
BM -> AS retained replacement handoff -> AE terminal provider evidence -> AV semantic result

FirstOwner:
BM -> retained first-owner handoff -> BO/BP terminal provider evidence -> BP semantic result

Superseded:
BM authoritative AQ-superseded terminal -> Contended
```

The common composition may dispatch among these chains but may not collapse their evidence types, erase first-owner CompareFailed/Superseded provenance, synthesize resolved evidence, or convert one branch's evidence into another branch's type.

## Dependency and ownership direction

C02f-BR preserves:

```text
executor/bootstrap lifecycle (still deferred)
        |
        v
prw-remote-bridge common acquisition orchestration
        |
        v
prw-control-plane BM preparation + etcd provider execution
        |
        v
etcd-client
```

`prw-control-plane` must not depend on `prw-remote-bridge`.

The scoped provider borrow is supplied downward-owned provider state to an already-authorized bridge orchestration call; it does not transfer provider lifecycle/endpoint ownership into the bridge and does not authorize bridge code to construct provider internals independently.

## Cancellation and runtime ownership

The later common composition remains an ordinary borrowed Future under the C02f-X/Y `impl Future + Send` model.

It must add no:

- detached task;
- background worker;
- timer;
- retry scheduler;
- channel-driven provider owner;
- `Arc<Mutex<_>>` provider wrapper;
- nested/blocking runtime;
- `block_on`;
- mandatory `async-trait`;
- mandatory boxed/dynamic future.

Dropping the outer future must not spawn or schedule follow-up provider work. Any in-flight AE/BO indeterminate-mutation reconciliation remains solely the bounded behavior already selected inside those provider paths while the future is actually polled.

## Relationship to `ReachabilityLiveOwnerAsyncAuthority::acquire`

C02f-BR selects the complete **acquisition sub-composition semantics** needed by a later production async authority implementation, but it does not yet implement or activate the full `ReachabilityLiveOwnerAsyncAuthority` object.

A future `acquire(peer)` implementation may delegate to the selected common composition only after its authority/provider lifecycle owner is separately selected/materialized.

Currentness and release composition remain separate boundaries. Process-level executor/bootstrap ownership also remains separate.

Therefore BR does not authorize an `impl ReachabilityLiveOwnerAsyncAuthority for ...` merely because the acquisition subpath is now selected.

## Source-materialization scope selected for a later checkpoint

A separately authorized source checkpoint should prefer the smallest practical scope:

1. add the non-escaping scoped live-owner execution-borrow seam to the existing BM preparation facade/control-plane public surface;
2. add one bridge-side common acquisition composition module/function implementing the exact `prepare -> Replacement | FirstOwner | Superseded` dispatch;
3. reuse the validated BQ replacement execution/mapping path and BP first-owner execution/mapping path without state-machine redesign;
4. add focused deterministic tests for branch dispatch and fail-closed mapping where provider I/O can be abstracted without weakening exact evidence checks;
5. update narrow module exports required by those additions;
6. modify no Cargo manifest or `Cargo.lock` unless compiler-proven coupling makes it unavoidable and separately re-audited before commit.

The source checkpoint must stop rather than widening into currentness, release, provider construction, runtime ownership or R1-R4 effect activation.

## Explicitly not selected / not authorized

C02f-BR does not authorize:

- Rust source mutation in this checkpoint;
- a raw public `live_owner_mut()`/store getter;
- a second independently constructed live-owner store;
- a second etcd client/provider context for common acquisition;
- endpoint selection or `Client::connect`;
- TLS/auth/RBAC/credentials;
- a new fence allocation or attempt-ID generation policy;
- extra provider reads/re-observations;
- new retry/reissue loops;
- replacement or first-owner transaction redesign;
- provider-result/evidence reconstruction;
- currentness composition;
- release composition;
- full async-authority object activation;
- process runtime/executor ownership;
- recovery activation or recovery-provider changes;
- R1-R4 effect enforcement/activation;
- Agent/Android integration changes;
- deployment;
- merge.

## Validation gate

C02f-BR is valid only after canonical repository validation passes on the exact final documentation-only BR head and a fresh BQ -> BR compare proves:

- exact merge base is BQ head `8ea007644803f98dea81db5d0bd9dae6bdd9ff82`;
- BR is ahead only by the intended documentation commit(s);
- changed scope is only this C02f-BR contract;
- no Rust, manifest, lockfile, workflow, Android, Agent, runtime or deployment file changed.

Expected gate after successful validation:

`C02F_BR_COMMON_ACQUISITION_COMPOSITION_SELECTED`

A later source-materialization checkpoint requires separate explicit user authorization.