# C03e-LE Production Durable Capability Operation-Boundary Lifetime Custody Selection Staging

Status: `SELECTION_STAGING`
Date: `2026-09-05`

Gate:

`C03E_LE_PRODUCTION_DURABLE_CAPABILITY_OPERATION_BOUNDARY_LIFETIME_CUSTODY_SELECTED`

Intended closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_OPERATION_BOUNDARY_LIFETIME_CUSTODY_SELECTION`

## 1. Purpose

C03e-LE selects the next ownership-only step after evidence-closed C03e-LD.

LD materialized one dormant higher-owner aggregate that owns:

1. the existing production/reachability/requester-rendezvous remote-process operation inputs by value; and
2. exactly one outer `Arc<ProductionDurableCapabilityAuthority>` created from one raw authority.

LE selects only how that already-materialized outer Arc is retained at the existing remote-process operation boundary before any separately gated propagation into endpoint/executor/runtime code.

LE does not populate the aggregate from executable/bootstrap source, does not expose the Arc, does not clone it, does not pass it to LA, and does not activate any runtime behavior.

## 2. Exact predecessor authority

Repository:

`Gersi365/prw-executor-private`

Exact LD branch:

`phase-152-c03e-ld-production-durable-capability-higher-owner-arc-custody-modular-source-materialization`

Exact LD head:

`45b9376182e0b84cf12b46a9f15afef6c90ae338`

Exact LD tree:

`8d35aeaa5288e30c16f564282f05c34128427a15`

LD PR #440 remains draft/open/unmerged and is evidence-closed.

Final LD validation evidence on the exact head:

- `PRW Rust Validation` #1578 / run `33958902902`: success, including locked dependency graph, formatting, Clippy, tests, workspace build and cleanup;
- `PRW Android Validation` #1469 / run `33958902908`: success, including native adapter, Android application and cleanup.

Final namespace audit before LE creation found no `phase-152-c03e-le*` successor.

## 3. Frozen LD source evidence

### 3.1 Higher-owner custody module

Path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Exact LD blob:

`690ebff362c2dd99ae3c4932f8b4f0b0b00a7bcc`

The module contains the dormant non-cloneable:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<P, D, T, F, C, R, E>`

with exactly two fields:

- existing production/reachability/requester-rendezvous operation inputs by value;
- one `Arc<ProductionDurableCapabilityAuthority>`.

Its constructor performs the only selected higher-owner allocation:

`Arc::new(capability_authority)`

The LC-selected `pub(crate)` aggregate spelling remains frozen.

### 3.2 Existing production/requester-rendezvous operation factory

Path:

`crates/prw-agent/src/linux_bootstrap.rs`

Exact LD blob:

`f2a87c45bd8d96bf1555b65210531c94c722eb2f`

The existing dormant factory:

`linux_agent_production_reachability_requester_rendezvous_remote_process_operation(...)`

consumes the existing production/requester-rendezvous aggregate, constructs the unchanged production/reachability operation, and returns one:

`FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static`

The existing function, its aggregate, its callers, tests and active runtime behavior are not selected for mutation by LE.

### 3.3 Crate-root registry

Path:

`crates/prw-agent/src/lib.rs`

Exact LD blob:

`53e6b9c33d1a3be644fb6645289f6854cc096eee`

The new higher-owner custody module is already Linux-gated and registered. LE selects no crate-root mutation.

## 4. Selected operation-boundary custody seam

LE selects one new dormant crate-private operation factory in:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Conceptual name:

`linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation`

The new factory consumes exactly one existing LD higher-owner aggregate by value.

It must destructure that aggregate into:

- `requester_rendezvous_inputs`; and
- `capability_authority`.

It then constructs exactly one existing operation by invoking:

`linux_agent_production_reachability_requester_rendezvous_remote_process_operation(requester_rendezvous_inputs)`

No alternate operation factory is selected.

## 5. Full-operation lifetime law

The returned one-shot closure must retain the exact existing outer:

`Arc<ProductionDurableCapabilityAuthority>`

by value for the entire invocation of the existing production/requester-rendezvous remote-process operation.

The selected order is semantically equivalent to:

```rust
move |publisher| {
    operation(publisher);
    drop(capability_authority);
}
```

The durable capability authority must therefore remain alive while `operation(publisher)` is executing and may be released only after that call returns normally.

LE specifically does **not** select dropping the Arc before delegating to the existing operation.

This checkpoint is lifetime custody only. The retained Arc remains inaccessible to the delegated operation.

## 6. No-clone and identity law

The LE-selected factory must contain no:

`Arc::clone`

and no additional:

`Arc::new`

for the durable capability authority.

The only outer higher-owner allocation remains the LD constructor.

The exact Arc moved into the LE closure is the same Arc owned by the LD aggregate.

No accessor, borrowed getter, mutable getter, extraction method, `into_parts`, propagation method, callback injection, operation parameter, endpoint parameter or executor parameter is selected.

## 7. Existing operation preservation

The existing:

`linux_agent_production_reachability_requester_rendezvous_remote_process_operation(...)`

must remain byte-identical in `linux_bootstrap.rs`.

LE selects no change to how requester/rendezvous custody values are currently handled by that existing operation.

The new LE factory is a wrapper around the existing factory; it does not duplicate its operation body, shutdown semantics, endpoint lifecycle, requester/rendezvous behavior or production/reachability behavior.

## 8. Generic-bound law

The new factory must use the same semantic generic bounds already required by the existing production/requester-rendezvous operation factory for:

- `P: PolicyEvaluator + Send + Sync + 'static`;
- `D: CapabilityDispatcher + Send + 'static`;
- `T: FnMut() -> u64 + Send + 'static`;
- `F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming + Send + 'static`;
- `C: FnMut(RemoteSessionRegisteredWorkerCompletion) + Send + 'static`;
- `R: FnMut(RemoteSessionExpectedDeviceAdmissionRejection<D, T>) + Send + 'static`;
- `E: FnMut(RemoteSessionRepeatedAdmissionFailure) + Send + 'static`.

Only imports required to express those existing bounds, the existing operation factory, and `LinuxAgentRemoteSupervisorShutdownPublisher` may be added to the selected module.

No new trait abstraction or generic authority interface is selected.

## 9. Factory side-effect law

Constructing the new LE-selected operation factory must be side-effect-free except for ownership moves.

Factory construction performs no:

- credential read;
- provider bootstrap;
- registry operation;
- mutex acquisition;
- capability authorization;
- policy evaluation;
- request read/decode;
- endpoint bind;
- listener activation;
- readiness publication;
- task/thread spawn;
- retry/fallback;
- network I/O;
- durable-authority mutation.

The returned closure remains dormant unless a later separately gated caller invokes it.

## 10. Caller boundary remains closed

LE selects no caller of the new factory.

In particular, LE does not change:

- executable/bootstrap assembly;
- `main.rs`;
- `run_with_remote_process_companion` or its callers;
- production operation construction call sites;
- endpoint lifecycle construction;
- executor lifecycle construction;
- LA visibility or invocation.

The current active runtime path remains unchanged.

## 11. Durable bootstrap population remains closed

The existing:

`bootstrap_production_durable_capability_authority_from_systemd_credentials()`

is not invoked by LE.

LE does not select where executable/bootstrap source obtains a raw `ProductionDurableCapabilityAuthority`, when the LD aggregate is constructed, or which real caller eventually consumes it.

No systemd credential path/name, provider endpoint, TLS material, registry store, synthetic authority, fallback authority or process-global state enters the LE-selected factory.

## 12. LA propagation remains closed

The existing LA production-durable repeated-admission overload remains dormant and uninvoked.

LE does not pass the retained outer Arc into:

- `RemoteSessionEndpointLifecycleRuntime`;
- `RemoteSessionExecutorRuntime`;
- LA's durable repeated-admission overload;
- requester-DR authority;
- AJ admission;
- shared-current capability authority.

`ProductionDurableCapabilityAuthority` remains distinct from `SharedCurrentCapabilityAuthority<P>`.

A later independent gate must select any real propagation/interface change.

## 13. Exact immediate source ceiling

The immediate source materialization after LE may change exactly one path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Selected mutation inside that file is limited to:

1. imports required by the existing operation signature/bounds;
2. one dormant crate-private operation factory implementing the lifetime law above;
3. a narrowly scoped `dead_code` allowance if required because no caller is selected.

The existing LD aggregate and constructor must remain textually unchanged except for rustfmt movement that is mechanically required by added imports and does not alter either item.

No second source path is selected.

If compilation requires `linux_bootstrap.rs`, `lib.rs`, LA, endpoint/executor source, a manifest, workflow, Android source, systemd source or any other path to change, STOP and open a new gate.

## 14. Immediate successor proof obligations

A later LE source materialization successor must prove:

1. exact predecessor is LE head;
2. exactly one selected Rust path changed;
3. LD aggregate still owns exactly two fields;
4. LD constructor still performs exactly one `Arc::new(capability_authority)`;
5. aggregate remains `pub(crate)` and non-Clone/non-Copy;
6. new factory consumes the aggregate by value;
7. new factory constructs exactly one existing production/requester-rendezvous operation;
8. the exact outer Arc is moved into the returned closure by value;
9. `operation(publisher)` occurs before `drop(capability_authority)`;
10. no durable-authority `Arc::clone` occurs;
11. no additional durable-authority `Arc::new` occurs;
12. no Arc accessor/extraction/propagation interface is added;
13. no caller of the new factory is added;
14. `linux_bootstrap.rs` remains exact blob `f2a87c45bd8d96bf1555b65210531c94c722eb2f`;
15. no LA/endpoint/executor/runtime caller mutation occurs;
16. no durable bootstrap helper is invoked;
17. no listener/readiness/network/runtime behavior changes;
18. no manifest/lock/workflow/Android/systemd/deployment change occurs;
19. Rust formatting, Clippy, tests and workspace build succeed;
20. Android validation, if triggered, is regression evidence only and authorizes no Android mutation.

## 15. Later gates left open

LE deliberately leaves all of the following for separate checkpoints:

- executable/bootstrap construction of the LD aggregate;
- durable-authority bootstrap population;
- migration of a real operation caller to the LE factory;
- outer-Arc extraction or propagation;
- operation parameter changes that expose the Arc downstream;
- endpoint lifecycle durable-authority custody;
- executor lifecycle migration to LA;
- LA caller population;
- readiness/listener/network behavior changes;
- runtime activation;
- deployment/restart/recovery/service-manager behavior.

## 16. Exact STOP boundary

C03e-LE is selection-only.

Do not mutate Rust source in LE.

Do not add a real caller.

Do not expose, clone, extract or propagate the durable-authority Arc.

Do not call the durable capability-authority bootstrap helper from executable/runtime source.

Do not modify `linux_bootstrap.rs`, `lib.rs`, LA, endpoint/executor source, `main.rs`, manifests, workflows, Android, systemd, deployment or runtime configuration.

Do not mark any PR ready, merge, deploy, restart or activate networking/runtime behavior.
