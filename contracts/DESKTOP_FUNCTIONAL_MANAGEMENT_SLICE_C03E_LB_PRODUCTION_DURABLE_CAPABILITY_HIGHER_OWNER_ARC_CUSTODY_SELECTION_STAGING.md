# C03e-LB Production Durable Capability Higher-Owner Arc Custody Selection Staging

Status: `SELECTION_STAGING`
Date: `2026-09-05`

Gate:

`C03E_LB_PRODUCTION_DURABLE_CAPABILITY_HIGHER_OWNER_ARC_CUSTODY_SELECTED`

Intended closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_HIGHER_OWNER_ARC_CUSTODY_SELECTION`

## 1. Purpose

C03e-LB is the first selection checkpoint after the evidence-closed C03e-LA modular source materialization.

LA materialized the KX/KZ-selected dormant repeated real-admission overload that accepts one higher-owner:

`Arc<ProductionDurableCapabilityAuthority>`

but deliberately did not select where the first production outer Arc is created or retained, which production aggregate owns it, how it is propagated to the executor, or when any runtime caller migrates to the new overload.

LB closes only the first of those remaining ownership questions. It selects one additive, dormant production aggregate whose sole new responsibility is to consume one already-bootstrapped raw `ProductionDurableCapabilityAuthority`, wrap it in exactly one outer `Arc`, and retain that Arc beside the already-materialized production/reachability/requester-rendezvous process inputs.

LB does not select propagation of that Arc to endpoint lifecycle or executor code. It does not select a caller migration, executable assembly, credential bootstrap callsite, runtime activation, network activation, readiness change, deployment, restart, or merge.

## 2. Exact predecessor authority

Repository:

`Gersi365/prw-executor-private`

Exact LA branch:

`phase-152-c03e-la-production-durable-capability-repeated-real-admission-collection-modular-source-materialization`

Exact LA head:

`ad6d8a18a81181c2a3b79b15ce81c15bd7d9d3ec`

Exact LA tree:

`045cc6a8db4fa07ec38f04c07ba360d9bd7ba372`

LA PR #437 remains draft/open/unmerged.

LA Rust validation #1573 and Android validation #1463 are successful. LA is evidence-closed without runtime activation.

## 3. Frozen source evidence at exact LA

The following LA blobs are authoritative for this selection.

### 3.1 Linux production composition surface

Path:

`crates/prw-agent/src/linux_bootstrap.rs`

Exact blob:

`f2a87c45bd8d96bf1555b65210531c94c722eb2f`

The file already contains the non-cloneable staged aggregate:

`LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<P, D, T, F, C, R, E>`

That aggregate joins the existing production/reachability process inputs with requester-rendezvous start-policy/runtime-owner custody before separately gated executable assembly.

Its existing operation path remains staged and does not carry production durable capability authority to LA.

### 3.2 Durable capability-authority ownership source

Path:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

Exact blob:

`90b12c182d6564b42e3f22f9e3dd594ec94d2fe5`

The source defines:

`ProductionDurableCapabilityAuthority`

which owns the durable-registry runtime custody behind its existing internal:

`Arc<Mutex<ProductionDurableRegistryRuntimeCustody>>`

and retains the concrete production deny-all policy baseline.

This internal registry-custody Arc is not the KX/KZ/LA outer authority Arc. LB does not alter it.

### 3.3 Durable capability-authority population source

Path:

`crates/prw-agent/src/production_durable_registry_custody_bootstrap.rs`

Exact blob:

`c48003712ac20b86fc09ebdfb2ddb67afd44f649`

The existing async helper:

`bootstrap_production_durable_capability_authority_from_systemd_credentials()`

returns one raw:

`ProductionDurableCapabilityAuthority`

It does not create an outer `Arc<ProductionDurableCapabilityAuthority>` and it does not wire any runtime caller.

### 3.4 LA durable repeated-admission overload

Path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`

Exact blob:

`291ef3bd99d1b40daa77861af3107212eddad5a6`

The dormant LA overload receives:

`capability_authority: Arc<ProductionDurableCapabilityAuthority>`

while the existing:

`SharedCurrentCapabilityAuthority<P>`

remains the AJ admission authority and requester-DR authority.

LA does not construct the outer Arc and remains uninvoked by production runtime source.

## 4. Correction of the ownership model

LB explicitly distinguishes three different ownership layers that must not be conflated.

1. `ProductionDurableRegistryRuntimeCustody` owns the semantic durable-registry store.
2. `ProductionDurableCapabilityAuthority` owns that registry custody through its existing internal `Arc<Mutex<_>>` and owns the deny-all production policy.
3. KX/KZ/LA require a separate outer `Arc<ProductionDurableCapabilityAuthority>` so one authority object can be shared across repeated persistent-worker recreation.

LB selects only layer 3's first production owner.

LB does not add another registry-custody mutex, does not alter the authority's internal Arc, and does not wrap `ProductionDurableRegistryRuntimeCustody` itself in the new outer Arc.

## 5. Selected higher-owner aggregate boundary

The future source materialization selected by LB may add exactly one new dormant aggregate to:

`crates/prw-agent/src/linux_bootstrap.rs`

Selected type name:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<P, D, T, F, C, R, E>`

The new aggregate is additive and sits one ownership layer outside the existing:

`LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<P, D, T, F, C, R, E>`

It must not replace, rename, widen, or mutate the existing aggregate.

## 6. Exact selected custody shape

The selected new aggregate owns exactly two logical fields:

1. the existing production/reachability/requester-rendezvous aggregate by value; and
2. one `Arc<ProductionDurableCapabilityAuthority>`.

The intended semantic shape is:

```rust
pub(crate) struct LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<
    P,
    D,
    T,
    F,
    C,
    R,
    E,
> {
    requester_rendezvous_inputs:
        LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<
            P,
            D,
            T,
            F,
            C,
            R,
            E,
        >,
    capability_authority: Arc<ProductionDurableCapabilityAuthority>,
}
```

Exact formatting and line wrapping may be determined by rustfmt, but the ownership semantics are fixed by this contract.

No third authority field, registry field, policy field, provider field, credential field, endpoint field, channel, queue, task handle, or retry state is selected.

## 7. Outer Arc construction law

The new aggregate constructor is the selected first production creation site for the KX/KZ/LA outer authority Arc.

It must consume by value:

- one existing `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<...>`; and
- one raw `ProductionDurableCapabilityAuthority`.

It must perform exactly one ownership adaptation:

`Arc::new(capability_authority)`

and retain the resulting Arc in the new aggregate.

The constructor must not call the async durable-registry/bootstrap helper itself.

It must not read credentials, perform provider I/O, perform registry I/O, acquire a mutex, authorize a request, evaluate policy, spawn a task, bind a listener, publish readiness, or activate networking.

## 8. No outer-Arc clone in the higher owner

The LB-selected aggregate must not clone its `Arc<ProductionDurableCapabilityAuthority>`.

No accessor returning an Arc clone is selected in LB.

No `into_parts`/extraction method is selected in LB.

No runtime propagation method is selected in LB.

The later mechanism that moves the retained outer Arc toward endpoint/executor code remains a separate gate.

This keeps the KZ/LA worker-insertion law untouched: LA remains the first currently materialized place that clones the received outer authority Arc, and it clones only for successful vacant-slot worker insertion.

## 9. Non-cloneable process-lifetime custody law

The new aggregate must not derive or implement `Clone` or `Copy`.

Its existing nested production/reachability/requester-rendezvous aggregate remains owned by value and unchanged.

The new aggregate represents process-lifetime production custody only. It is not a per-request, per-AJ, per-session, per-worker, or per-recovery object.

Worker crash/recreation must not reconstruct the raw `ProductionDurableCapabilityAuthority` or create a new outer Arc.

## 10. Population boundary

The existing helper:

`bootstrap_production_durable_capability_authority_from_systemd_credentials()`

remains the selected population source for one raw production durable capability authority.

LB does not call it.

A later executable/bootstrap composition gate may decide whether and where a real process invocation awaits that helper and passes its successful raw return value into the LB-selected aggregate constructor.

No credential name, credential path, provider endpoint, TLS material, raw client, registry record, fallback authority, or synthetic authority enters the LB aggregate constructor.

## 11. Shared-current authority separation law

The existing:

`SharedCurrentCapabilityAuthority<P>`

remains unchanged and distinct from:

`ProductionDurableCapabilityAuthority`.

LB does not alter `LinuxAgentRemoteProcessOperationInputs::authority`.

LB does not add production durable authority to AJ.

LB does not substitute production durable authority for requester-DR authority.

LB preserves:

`ProductionDurableCapabilityAuthority != SharedCurrentCapabilityAuthority<P>`

## 12. Existing aggregate preservation

The existing:

`LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<...>`

must remain textually and behaviorally unchanged in the immediate source successor except for unavoidable line-number displacement caused by additive source placed elsewhere in the same file.

Its constructor, fields, staged operation functions, tests, and existing requester-rendezvous identity checks are not selected for mutation.

The new durable aggregate wraps it by value rather than adding a required field to it.

This prevents LB materialization from changing any existing constructor callsite or staged operation behavior.

## 13. Active call graph remains unchanged

LB selects no change to the active path:

`linux_bootstrap -> RemoteSessionEndpointLifecycleRuntime -> RemoteSessionExecutorRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle -> legacy repeated real-admission collection`

The LA durable requester-aware repeated-admission overload remains dormant.

No existing function is selected to call the new LB aggregate constructor.

No existing function is selected to receive the retained outer Arc.

No endpoint lifecycle method, executor lifecycle method, repeated collection method, binary entrypoint, or service-manager integration is selected for caller migration.

## 14. Executor/endpoint visibility boundary remains closed

The LA durable repeated-admission overload remains scoped inside `remote_session_executor_runtime`.

Its internal requester-aware completion envelope and disposal helpers remain implementation details of that module.

LB does not widen visibility, export internal completion types, add an endpoint wrapper, or select an executor-to-endpoint integration API.

That interface/caller boundary remains independently gated after higher-owner custody is materialized.

## 15. Identity and authorization laws remain unchanged

LB changes no identity semantics.

- Authenticated PRW application-session `DeviceId` remains the active worker-map key.
- Expected `DeviceId` remains preflight intent until authenticated AJ success.
- Static IP is never identity.
- Transport identity/evidence is not logical device identity.
- Outer PRWM `request_id` remains correlation only.
- Authority custody is not identity.

LB changes no authorization semantics.

The existing production durable authority retains its fixed deny-all policy baseline and existing durable bridge method unchanged.

## 16. Exact future source ceiling

The immediate source successor to LB may change only:

`crates/prw-agent/src/linux_bootstrap.rs`

and only to add the LB-selected dormant higher-owner aggregate, its side-effect-free constructor, required imports, and narrowly scoped compile-time/unit tests proving the selected ownership shape.

The following remain byte-identical unless a later gate explicitly selects them:

- `crates/prw-agent/src/production_durable_registry_runtime_custody.rs`;
- `crates/prw-agent/src/production_durable_registry_custody_bootstrap.rs`;
- all `remote_session_capability_runtime` source, including LA;
- `main.rs`;
- manifests and lockfile;
- workflows;
- Android source;
- systemd/service-manager configuration;
- deployment/restart configuration.

If compilation requires a second source path, an existing aggregate mutation, a new bootstrap callsite, an Arc propagation method, visibility widening, endpoint/executor caller change, or executable activation, STOP and open a new gate.

## 17. Immediate source successor proof obligations

A later LB source-materialization checkpoint must prove all of the following:

1. exact predecessor is LB selection head and its merge-base is exact LA/LB predecessor chain;
2. exactly one Rust source path changed: `crates/prw-agent/src/linux_bootstrap.rs`;
3. the source change is additive and dormant;
4. the new aggregate owns the existing requester-rendezvous production aggregate by value;
5. the new aggregate owns exactly one outer `Arc<ProductionDurableCapabilityAuthority>`;
6. its constructor receives one raw `ProductionDurableCapabilityAuthority` by value;
7. exactly one `Arc::new(capability_authority)` creates the outer Arc;
8. no `Arc::clone` of that outer authority is added at this layer;
9. no `Clone` or `Copy` implementation/derive is added to the aggregate;
10. no durable bootstrap helper is invoked;
11. no existing aggregate constructor/callsite is changed;
12. no existing operation function calls the new aggregate;
13. no endpoint/executor source changes;
14. no existing authority lane changes;
15. no listener/readiness/network/runtime behavior changes;
16. no manifest, lockfile, workflow, Android, deployment, or service-manager change occurs;
17. Rust formatting, Clippy, tests, and workspace build remain successful;
18. Android validation is regression evidence only and authorizes no Android mutation.

## 18. Later gates left open deliberately

LB does not select any of the following:

- where executable/bootstrap code awaits production durable capability-authority population;
- how the LB aggregate is populated by a real executable caller;
- how the retained Arc is moved out of the LB aggregate;
- how the Arc crosses the production operation boundary;
- how endpoint lifecycle receives durable-authority custody;
- how executor lifecycle invokes the LA durable repeated-admission overload;
- whether an additive executor wrapper is required;
- any visibility widening of LA internals;
- any production caller migration;
- readiness semantics;
- listener semantics;
- network activation;
- deployment, restart, recovery, or service-manager behavior.

Each remains independently gated.

## 19. Selection rationale

The additive wrapper is selected instead of mutating the existing production/reachability/requester-rendezvous aggregate because:

1. the existing aggregate already has staged constructors, identity checks, operation functions, and tests;
2. adding a required durable field to it would alter existing callsites before durable caller migration is selected;
3. a new outer aggregate can retain the durable authority without changing any existing behavior;
4. the raw authority returned by the existing bootstrap helper has exactly one clear ownership adaptation still missing: the KX/KZ/LA outer `Arc`;
5. wrapping the raw authority once at the process aggregate layer preserves one authority object across later worker recreation without creating per-worker authority state;
6. later propagation can be selected independently without reopening durable-registry/bootstrap semantics.

## 20. Exact STOP boundary

C03e-LB is selection-only.

Do not mutate `linux_bootstrap.rs` in LB.

Do not call the production durable capability-authority bootstrap helper from any runtime/executable path.

Do not add an outer-Arc accessor, clone, extraction method, endpoint wrapper, executor wrapper, or caller migration in LB.

Do not modify LA, KZ, KY, KX, main, manifests, workflows, Android, systemd, deployment, or runtime configuration.

Do not mark any PR ready, merge, deploy, restart, activate networking, or rewrite history.
