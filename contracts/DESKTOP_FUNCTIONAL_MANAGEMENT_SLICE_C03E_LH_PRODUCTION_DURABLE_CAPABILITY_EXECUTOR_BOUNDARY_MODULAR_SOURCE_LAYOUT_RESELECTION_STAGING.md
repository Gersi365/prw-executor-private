# Phase 152 C03e-LH — Production Durable Capability Executor-Boundary Modular Source Layout Reselection

Status: `SELECTION_STAGING — VALIDATION_PENDING`

## 1. Purpose

C03e-LH reselects only the physical Rust source location for the C03e-LG-selected dormant production-durable executor endpoint-lifecycle boundary.

LG selected the runtime semantics correctly but froze `remote_session_executor_runtime.rs` as the sole immediate source path. Exact LG transport audit now proves that path cannot be replaced safely through the available whole-file mutation surface without accepting truncation/reconstruction risk.

LH therefore changes no runtime semantics and materializes no Rust source. It selects one already-registered small descendant module as the sole later source-materialization path while preserving the exact LG effective visibility, callback adaptation, endpoint-finalization order, ownership law, return type, and STOP boundaries.

## 2. Exact predecessor

Canonical predecessor checkpoint:

`C03E_LG_PRODUCTION_DURABLE_CAPABILITY_EXECUTOR_ROOT_BOUNDARY_DISPOSITION_SELECTED`

Exact predecessor branch:

`phase-152-c03e-lg-production-durable-capability-executor-root-boundary-disposition-selection`

Exact predecessor head:

`b8e60855bdcb01af5d386c9793aca71466c652bc`

Exact predecessor tree:

`511f1d3c7498be41773833267f938250efb78225`

Exact LG contract blob:

`0c041f8c56dd7bd2ed85aff8d75d2e64057457cb`

LG remains frozen. LH supersedes only LG section 4 / section 19 physical source-location selection and the source-relative spelling needed to preserve the same effective visibility from that new location.

## 3. Exact transport evidence

### 3.1 Root executor file

Exact LG root source:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

Exact blob:

`ef370ca500f118bc067097ddb8f5c37ab597b214`

Exact size reported by the GitHub contents API:

`80,259 bytes`

The available high-level GitHub update operation replaces a UTF-8 file as one complete body. Exact reads of this root file are truncated by the connector response budget.

Therefore a root-file source successor would require reconstruction of a large body that cannot be independently proven byte-complete through the current mutation transport.

LH treats that as a deterministic transport STOP, not as permission to perform a best-effort rewrite.

### 3.2 Existing modular child

Exact existing LA child path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`

Exact LG blob:

`291ef3bd99d1b40daa77861af3107212eddad5a6`

Exact size:

`8,345 bytes`

The child is already registered by the existing parent declaration:

`mod production_durable_repeated_real_admission_collection;`

No module-registration edit is required.

The child already contains an `impl RemoteSessionExecutorRuntime` and the exact dormant LA durable collection selected by LG.

## 4. Privacy / reachability evidence

The selected child is nested under the existing `remote_session_executor_runtime` module root.

Rust ancestor-private visibility therefore permits this descendant to reference the already-existing root-private endpoint finalizer without widening it:

`finish_remote_endpoint_shutdown(...)`

The existing FW disposer is defined in the ancestor requester-aware worker module with parent-scoped visibility and remains reachable from the selected descendant through the executor-root visibility domain:

`dispose_recoverable_repeated_real_admission_requester_aware_worker_completion(...)`

The existing LA durable collection is already defined in the same selected child and remains callable without any visibility change.

LH selects no visibility edit to any existing item or module.

## 5. Reselected source successor

The immediate source-materialization successor after LH may change exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`

The former LG-selected root path must remain byte-identical in that successor:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

No second source path is selected.

## 6. Selected method identity

The method name remains exactly the LG-selected normative name:

`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability`

It remains an inherent method on:

`RemoteSessionExecutorRuntime`

Physical relocation does not create a trait, wrapper owner, free function, re-export, or alternate API.

## 7. Effective visibility preservation

At the LG root location, the selected method visibility was:

`pub(super)`

which exposes the method only to the parent `remote_session_capability_runtime` visibility domain and its descendants.

From the reselected nested child, source-relative `pub(super)` would be too narrow and would not preserve the LG endpoint-sibling boundary.

LH therefore selects the ancestor-qualified spelling:

`pub(in super::super::super::super)`

for the method when materialized in the selected child.

From the selected child, that path resolves to the same `remote_session_capability_runtime` module domain that LG's root-level `pub(super)` selected.

This is an effective-scope preservation, not a visibility widening.

No existing item changes visibility.

If compiler/module resolution proves this spelling does not resolve to exactly that existing parent visibility domain, the source successor must STOP rather than choose a broader visibility.

## 8. Input shape remains exact LG

The reselected method must accept exactly the LG-selected input surface:

- `&mut self`;
- `max_active_workers: NonZeroUsize`;
- `transport_runtime: &AgentRemoteTransportRuntime`;
- `authority: &SharedCurrentCapabilityAuthority<P>`;
- `capability_authority: Arc<ProductionDurableCapabilityAuthority>`;
- `policy_source: Arc<PS>`;
- `requester_rendezvous_authority: &SharedRequesterRendezvousAuthority`;
- `session_authentication: &mut SessionAuthenticationService`;
- `expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`;
- `supervisor_shutdown: S`;
- `admission_timing: F`;
- boundary-safe completion callback `C`;
- existing LA rejection callback `R`;
- existing LA admission-failure callback `E`.

No additional owner, runtime, raw transport, bootstrap input, provider, store, queue, channel, receipt, retry token, restart token, or callback object is selected.

## 9. Generic bounds remain exact LG

The source successor must preserve:

```rust
P: PolicyEvaluator + Send + Sync + 'static,
D: CapabilityDispatcher + Send + 'static,
T: FnMut() -> u64 + Send + 'static,
PS: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static,
S: Future<Output = ()> + Send,
F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming,
C: FnMut(
    DeviceId,
    Result<
        RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop,
        RemoteSessionSpawnedWorkerJoinError,
    >,
),
R: FnMut(
    RemoteSessionExpectedDeviceAdmissionRejectionReason,
    RemoteSessionExpectedDeviceAdmissionRequest<D, T>,
),
E: FnMut(DeviceId, RemoteSessionRealAdmissionError),
```

No stronger bound is selected.

## 10. Return shape remains exact LG

The method must return exactly:

`Result<(), RemoteSessionPersistentCollectionConfigError>`

No wrapper, mapping, error enum, retry classification, or side-channel result is selected.

## 11. Completion adaptation law remains exact LG

For every completion produced by LA, the reselected method must preserve exactly:

1. receive `RecoverableRepeatedRealAdmissionRequesterAwareWorkerCompletion` by value;
2. pass it immediately by value to existing FW disposition;
3. receive the exact existing `(DeviceId, Result<...>)` tuple;
4. invoke the outward completion callback only after FW has consumed/disposed the recovered peer;
5. forward the exact unchanged `DeviceId` and FL/join result.

Conceptually:

```rust
|completion| {
    let (device_id, result) =
        dispose_recoverable_repeated_real_admission_requester_aware_worker_completion(completion);
    on_completion(device_id, result);
}
```

No private session owner or FU completion envelope crosses the boundary.

## 12. Endpoint-finalization law remains exact LG

The reselected method must preserve this exact order:

1. run the existing LA durable repeated-real-admission collection;
2. dispose every published requester-aware completion through FW before outward observation;
3. retain the exact LA `Result<(), RemoteSessionPersistentCollectionConfigError>`;
4. invoke the existing `finish_remote_endpoint_shutdown(...)` exactly once;
5. existing endpoint close uses its unchanged fixed shutdown code/reason;
6. existing `wait_idle()` is driven to completion on the retained executor runtime;
7. return the original LA result unchanged.

No endpoint close occurs before LA returns.

No duplicate close or alternate idle-drain mechanism is selected.

## 13. Durable Arc law remains exact LG

The outer `Arc<ProductionDurableCapabilityAuthority>` is received by value and transferred unchanged into LA.

The modular adapter selects:

- no `Arc::new`;
- no adapter-level `Arc::clone`;
- no Arc extraction/return;
- no global/static storage;
- no callback exposure.

Existing internal LA worker cloning remains unchanged.

LH still does not connect LF process-level custody to this executor seam.

## 14. Requester-aware semantics remain unchanged

FW remains the sole selected higher-owner peer disposition for LA completions.

LH performs no requester-record retirement, removal, rollback, wildcard cleanup, provider reset, or capacity sweep.

The exact requester/rendezvous authority is forwarded unchanged into LA.

## 15. Active paths remain frozen

LH selects no caller.

The following remain byte-stable and behaviorally unchanged:

- existing active non-durable executor endpoint lifecycle;
- `remote_session_endpoint_lifecycle_runtime.rs`;
- `production_reachability_endpoint_lifecycle.rs`;
- `production_durable_capability_higher_owner_custody.rs`;
- `linux_bootstrap.rs`;
- durable bootstrap/custody source;
- manifests, lockfiles, workflows, Android and systemd source.

No endpoint sibling is migrated by LH or its immediate source-materialization successor.

## 16. No module-layout mutation required

Because the selected LA child already exists and is already registered, the immediate source successor must not add or change:

- a `mod` declaration;
- `remote_session_capability_runtime.rs` registration/re-export;
- parent module registration;
- filesystem module layout;
- crate-level public API.

Any need for such a second-path layout edit is a STOP condition.

## 17. Immediate source-materialization ceiling

A later source-materialization checkpoint may change only the selected 8,345-byte LA child path.

Permitted edits are limited to:

- imports/path qualifications required by the LG-selected composition;
- the new dormant inherent method;
- narrowly scoped lint/dead-code annotations required because the method has no caller yet;
- tests in the same file only if required to prove signature/order without behavior activation.

The source successor must not modify the 80,259-byte root executor file.

If materialization requires any second Rust path, any visibility widening, any new type/wrapper, or any caller change, it must STOP and return to a new selection gate.

## 18. Validation requirements for the later source successor

A later materialization must prove at minimum:

- exact predecessor is the evidence-closed LH selection head;
- exactly one selected Rust path changed;
- root executor blob remains exactly `ef370ca500f118bc067097ddb8f5c37ab597b214`;
- LG contract remains unchanged;
- endpoint sibling source remains unchanged;
- LF higher-owner source remains unchanged;
- `linux_bootstrap.rs` remains unchanged;
- no caller of the new method exists;
- formatting passes;
- Clippy passes under existing strict settings;
- tests pass;
- workspace build passes;
- any path-triggered Android workflow is regression evidence only.

## 19. Explicitly rejected alternatives

LH rejects:

- reconstructing/replacing the truncated 80,259-byte root file;
- changing root + child together;
- adding a second module-registration path;
- widening LA or FW visibility;
- exposing the private FU completion envelope;
- introducing a public completion wrapper;
- moving FW disposition into the endpoint sibling;
- duplicating FW peer-close logic;
- changing endpoint shutdown ordering;
- adding adapter-level durable Arc clone/new;
- durable bootstrap population;
- LF operation caller wiring;
- endpoint sibling migration;
- process activation/deployment.

## 20. Next-gate boundary

After LH is evidence-closed, the next checkpoint may materialize only this modular one-path executor-boundary source seam.

Only after that independent source checkpoint is evidence-closed may another selection gate examine a dormant endpoint-sibling durable lifecycle overload.

Process-level Arc propagation, durable bootstrap population, and real caller activation remain later independent questions.

## 21. STOP

Keep LH docs-only.

Do not materialize Rust source in LH.

Do not mark ready, merge, deploy, restart, mutate production state, add a caller, migrate the endpoint sibling, populate durable bootstrap, connect LF custody, expose private completion/session custody, or activate listener/network/runtime behavior without a later independent gate.
