# C03e-LS — Linux Production Requester/Rendezvous Durable Callback Projection Operation Selection

Status: `STAGING_SELECTION`

Date: `2026-09-05`

Gate:

`C03E_LS_LINUX_PRODUCTION_REQUESTER_RENDEZVOUS_DURABLE_CALLBACK_PROJECTION_OPERATION_SELECTED`

Closure:

`CLOSED_LINUX_PRODUCTION_REQUESTER_RENDEZVOUS_DURABLE_CALLBACK_PROJECTION_OPERATION_SELECTION`

## 1. Purpose

C03e-LS is a documentation-only selection checkpoint above evidence-closed C03e-LR. It selects the smallest next Agent-owned source seam that can consume the C03e-LR production reachability endpoint durable callback projection without widening requester-private completion state, without rewriting the existing legacy Linux production operation, and without crossing into higher-owner caller migration or executable activation.

This checkpoint changes no Rust source and activates no runtime behavior.

## 2. Exact predecessor authority

The selected predecessor is the exact C03e-LR final head:

- branch: `phase-152-c03e-lr-production-reachability-endpoint-durable-callback-projection-propagation-source-materialization`
- head: `52748b8d5994e5d99d59da96ab619b446658a24e`
- tree: `8455f926876d8847e9b5b30002647f2c2ea59adb`
- final C03e-LR source path: `crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs`
- final C03e-LR source blob: `4030cac84cad1780cc37410d344ba642cb4ac6e4`

The selected immediate future source path is read from that exact predecessor head:

- `crates/prw-agent/src/linux_bootstrap.rs`
- predecessor blob: `f2a87c45bd8d96bf1555b65210531c94c722eb2f`

Any source materialization from a different predecessor head or different `linux_bootstrap.rs` blob requires a fresh audit and reselection before mutation.

## 3. Fresh source findings

### 3.1 C03e-LR already owns the projection-capable production wrapper

C03e-LR materialized one dormant crate-visible sibling on `ProductionReachabilityEndpointLifecycleRuntime`:

`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability_projection(...)`

That method already preserves the production reachability owner custody across the complete delegated lower endpoint lifecycle and accepts the C03e-LP bounded completion projection directly.

Its completion callback surface is:

`FnMut(DeviceId, RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection)`

Its rejection callback surface is:

`FnMut(RemoteSessionExpectedDeviceAdmissionRejectionReason, RemoteSessionExpectedDeviceAdmissionRequest<D, T>)`

Its admission-failure callback surface is:

`FnMut(DeviceId, RemoteSessionRealAdmissionError)`

The method additionally accepts the existing:

- `SharedCurrentCapabilityAuthority<P>` by shared reference;
- caller-owned `Arc<ProductionDurableCapabilityAuthority>` by value;
- requester/rendezvous policy source as `Arc<PS>`;
- `SharedRequesterRendezvousAuthority` by shared reference;
- session-authentication custody;
- expected requests;
- admission timing.

C03e-LS selects no change to that C03e-LR method.

### 3.2 The nearest real Linux production operation still drives the legacy lifecycle

`linux_agent_production_reachability_remote_process_operation(...)` in `linux_bootstrap.rs` is the existing dormant production operation that:

1. creates one `RemoteSessionExecutorRuntime`;
2. bootstraps production reachability runtime custody for the exact peer;
3. binds the remote endpoint with that same executor;
4. publishes the remote supervisor shutdown controller;
5. drives the production endpoint lifecycle.

Its current final stage calls the legacy production endpoint lifecycle and therefore remains typed to the legacy completion/rejection/admission-failure aggregates.

C03e-LS does not replace, edit, or reinterpret that legacy operation.

### 3.3 The existing requester/rendezvous aggregate is structurally reusable

The existing crate-private aggregate:

`LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<P, D, T, F, C, R, E>`

already retains by value:

- the existing `LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P, D, T, F, C, R, E>`;
- one `BoundedRequesterRendezvousStartPolicySource`;
- one `CandidatePublicationRequesterRendezvousRuntimeOwner`.

The aggregate itself is generic over `C`, `R`, and `E`. The legacy callback shapes are imposed by existing operation-function bounds, not by a second callback-specific input owner.

Therefore the immediate successor does not need a new aggregate, a duplicate input owner, or a second source path.

### 3.4 Existing shared requester/rendezvous authority is the selected ownership adaptation

The existing `SharedRequesterRendezvousAuthority::new(...)` consumes one `CandidatePublicationRequesterRendezvousRuntimeOwner` by value and retains it behind the existing shared async synchronization boundary.

The selected successor may therefore adapt the already-owned requester/rendezvous runtime owner exactly once through this existing constructor. It must not obtain or expose the raw provider, clone provider state, or create a second requester/rendezvous provider authority.

### 3.5 Durable capability authority already has a higher-owner custody lane

`production_durable_capability_higher_owner_custody.rs` already retains exactly one outer `Arc<ProductionDurableCapabilityAuthority>` beside the existing production/reachability/requester-rendezvous aggregate.

The immediate successor therefore accepts that existing `Arc<ProductionDurableCapabilityAuthority>` as an explicit input. It must not construct, bootstrap, populate, replace, clone underlying authority state, or introduce a new durable-capability authority type.

Higher-owner invocation of the selected sibling remains a later separately gated migration.

## 4. Selected immediate source successor

The immediate source-materialization checkpoint is ceilinged to exactly one source path:

`crates/prw-agent/src/linux_bootstrap.rs`

The selected additive sibling is:

`linux_agent_production_reachability_requester_rendezvous_remote_process_operation_with_production_durable_capability_projection`

Selected visibility:

`pub(crate)`

The sibling remains dormant and is not selected as a new public or executable Agent entrypoint.

## 5. Selected input shape

The sibling must reuse the existing aggregate directly:

`LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<P, D, T, F, C, R, E>`

and accept exactly one additional explicit durable-capability input:

`Arc<ProductionDurableCapabilityAuthority>`

No new input-owner struct is selected.

The selected generic callback bounds are the C03e-LR projection shapes:

```rust
C: FnMut(
        DeviceId,
        RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection,
    ) + Send
    + 'static,
R: FnMut(
        RemoteSessionExpectedDeviceAdmissionRejectionReason,
        RemoteSessionExpectedDeviceAdmissionRequest<D, T>,
    ) + Send
    + 'static,
E: FnMut(DeviceId, RemoteSessionRealAdmissionError) + Send + 'static,
```

The operation continues to preserve the existing production-process lifetime requirements for the remaining generic inputs, including the existing `Send + 'static` process-closure requirements where required by `linux_bootstrap.rs`.

If exact source compilation proves that a strictly mechanical bound adjustment is required inside this same one-file sibling declaration, that adjustment is permitted only when it does not alter ownership, callback semantics, or any frozen existing function. Any need for a second source path requires STOP and reselection.

## 6. Selected ownership and stage law

The sibling must perform only the following ownership composition.

### 6.1 Factory construction

Before the returned one-shot process operation is invoked, the sibling may:

1. destructure the existing requester/rendezvous aggregate exactly once;
2. retain its existing production inputs unchanged;
3. wrap the exact existing `BoundedRequesterRendezvousStartPolicySource` in one `Arc` exactly once;
4. consume the exact existing `CandidatePublicationRequesterRendezvousRuntimeOwner` through `SharedRequesterRendezvousAuthority::new(...)` exactly once;
5. retain the caller-supplied `Arc<ProductionDurableCapabilityAuthority>` for the returned operation.

Factory construction must perform no credential read, provider I/O, endpoint bind, requester registration, durable capability mutation, listener activation, readiness publication, task spawn, retry, fallback, or peer disposition.

### 6.2 One-shot process operation

When the returned operation is invoked, it must reuse the existing `run_remote_process_operation_composition(...)` ordering exactly once:

1. `RemoteSessionExecutorRuntime::new`;
2. existing production reachability runtime-custody bootstrap for the exact retained peer;
3. existing same-executor production endpoint bind;
4. existing shutdown-controller publication;
5. exactly one C03e-LR projection-capable production endpoint lifecycle drive.

No alternate executor, second bootstrap, second bind, duplicate publication, fallback lifecycle, retry, reconnect, or secondary drive is selected.

## 7. Exact selected LR drive

The lifecycle stage must call exactly:

`ProductionReachabilityEndpointLifecycleRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability_projection(...)`

The call must forward unchanged:

- `max_active_workers`;
- the existing shared current capability authority;
- the caller-owned durable capability authority `Arc`;
- the exact requester/rendezvous policy-source `Arc`;
- the exact shared requester/rendezvous authority;
- session authentication;
- expected requests;
- admission timing;
- projection completion callback;
- projection rejection callback;
- projection admission-failure callback.

No callback translation layer is selected in `linux_bootstrap.rs`.

## 8. Callback law

### 8.1 Completion

The selected sibling receives and forwards unchanged:

`(DeviceId, RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection)`

It must not reconstruct requester-private stop/error payloads, map back into `RemoteSessionRegisteredWorkerCompletion`, or introduce executable exit/logging/counter policy.

### 8.2 Rejection

The selected sibling receives and forwards unchanged:

`(RemoteSessionExpectedDeviceAdmissionRejectionReason, RemoteSessionExpectedDeviceAdmissionRequest<D, T>)`

It must not rebuild the legacy rejection aggregate, discard the untouched expected request, or reinterpret rejection reasons.

### 8.3 Admission failure

The selected sibling receives and forwards unchanged:

`(DeviceId, RemoteSessionRealAdmissionError)`

It must not rebuild `RemoteSessionRepeatedAdmissionFailure` or add a new failure classifier.

## 9. Distinct custody lanes remain distinct

The selected sibling must preserve all of the following as separate ownership roles:

1. production reachability durable-owner custody remains inside `ProductionReachabilityEndpointLifecycleRuntime`;
2. production durable capability authority remains the explicit `Arc<ProductionDurableCapabilityAuthority>` lower authorization input;
3. requester/rendezvous runtime custody is adapted through the existing `SharedRequesterRendezvousAuthority`;
4. requester/rendezvous policy remains the existing bounded policy source under one `Arc`;
5. shared-current registry/policy authority remains the existing `SharedCurrentCapabilityAuthority<P>`.

No extraction, substitution, merging, aliasing shortcut, provider getter, or duplicate source of truth is selected between those lanes.

## 10. Permitted one-file mechanical changes in the future source checkpoint

Within `crates/prw-agent/src/linux_bootstrap.rs` only, the immediate source successor may add the selected sibling plus only the mechanical imports, documentation, dead-code allowance, type references, and tests necessary to compile and prove that sibling.

Permitted imported/referenced existing types include only those required by the selected shape, including:

- `std::sync::Arc`;
- `ProductionDurableCapabilityAuthority`;
- `RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection`;
- `RemoteSessionExpectedDeviceAdmissionRejectionReason`;
- `RemoteSessionRealAdmissionError`;
- `SharedRequesterRendezvousAuthority`.

A same-file compile-time shape test or side-effect-free construction test is permitted if it proves only the selected sibling and does not execute production bootstrap/network/provider I/O.

## 11. Frozen existing `linux_bootstrap.rs` surfaces

The immediate source successor must leave unchanged in behavior and signature:

- `linux_agent_remote_process_operation(...)`;
- `linux_agent_production_reachability_remote_process_operation(...)`;
- `linux_agent_production_reachability_requester_rendezvous_remote_process_operation(...)`;
- `LinuxAgentRemoteProcessOperationInputs<...>`;
- `LinuxAgentProductionReachabilityRemoteProcessOperationInputs<...>`;
- `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<...>`;
- existing production worker-limit/bind/peer population helpers;
- `run_with_production_reachability_requester_rendezvous_remote_process_companion(...)`;
- public `run()`;
- public `run_with_remote_process_companion(...)`;
- existing callback counters, report types, finalization policy, and process-exit surfaces.

The sibling is additive only.

## 12. Frozen external source surfaces

The immediate source successor must not modify:

- `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`;
- `crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs`;
- `crates/prw-agent/src/production_reachability_runtime_custody.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime.rs`;
- the C03e-LP raw endpoint projection adapter or enum;
- requester-private stop/error types or their visibility;
- shared requester/rendezvous authority implementation;
- durable capability authority bootstrap/population code;
- requester/rendezvous runtime-owner population code;
- durable-registry peer population code;
- `lib.rs`;
- `main.rs`;
- manifests;
- lockfiles;
- workflows;
- Android sources or packaging.

If any of those paths must change, STOP and reselect.

## 13. Explicit non-selection

C03e-LS does not select or authorize:

- higher-owner invocation of the new sibling;
- mutation of `production_durable_capability_higher_owner_custody.rs`;
- replacement of the existing legacy higher-owner operation;
- a new input aggregate or authority owner;
- production durable capability-authority bootstrap/population;
- requester/rendezvous provider bootstrap/population changes;
- current-peer source changes;
- conversion between legacy callback aggregates and the new projection callbacks;
- callback logging, counters, telemetry, process-exit or restart policy;
- candidate-publication activation;
- listener/readiness activation;
- public Agent `run()` activation;
- merge;
- ready-for-review conversion;
- deploy;
- restart/recovery activation;
- repository configuration mutation;
- branch deletion;
- force update or history rewrite;
- destructive cleanup.

## 14. Immediate successor validation requirements

A future source-materialization checkpoint based on C03e-LS must prove on its exact final head:

1. changed-path ceiling is exactly `crates/prw-agent/src/linux_bootstrap.rs`;
2. the new sibling is additive and dormant;
3. the sibling consumes the existing requester/rendezvous aggregate without a new aggregate type;
4. the exact existing runtime owner is converted once into `SharedRequesterRendezvousAuthority`;
5. the exact existing bounded policy source is wrapped once in `Arc`;
6. the caller-supplied durable capability authority `Arc` is forwarded unchanged to C03e-LR;
7. the same executor is preserved across production reachability bootstrap and endpoint bind;
8. controller publication ordering remains unchanged;
9. C03e-LR projection drive is called exactly once;
10. projection completion/rejection/admission-failure callbacks are forwarded without remapping;
11. existing legacy Linux operation factories remain unchanged;
12. no executable caller is activated;
13. Rust validation passes on the exact final head;
14. Android validation is classified only from exact-final-head workflow evidence if the path triggers it;
15. path-filtered skips remain `SKIPPED`, not PASS;
16. immutable Drive evidence is frozen, uploaded, raw-read back, hash-verified, and uniquely rediscovered before closure.

## 15. Selection gate

C03e-LS selects exactly one future source boundary:

`linux_agent_production_reachability_requester_rendezvous_remote_process_operation_with_production_durable_capability_projection(...)`

in exactly:

`crates/prw-agent/src/linux_bootstrap.rs`

with the ownership, stage, callback, and freeze laws above.

Gate:

`C03E_LS_LINUX_PRODUCTION_REQUESTER_RENDEZVOUS_DURABLE_CALLBACK_PROJECTION_OPERATION_SELECTED`

Closure marker after exact-head validation and immutable evidence publication:

`CLOSED_LINUX_PRODUCTION_REQUESTER_RENDEZVOUS_DURABLE_CALLBACK_PROJECTION_OPERATION_SELECTION`

## 16. STOP

After C03e-LS selection closure: **STOP**.

The next checkpoint may materialize only this one-file dormant sibling after a fresh namespace/head/source audit. It must not automatically migrate the higher-owner durable-capability custody caller, activate the Agent executable, define callback policy, merge, deploy, restart, or perform destructive cleanup.
