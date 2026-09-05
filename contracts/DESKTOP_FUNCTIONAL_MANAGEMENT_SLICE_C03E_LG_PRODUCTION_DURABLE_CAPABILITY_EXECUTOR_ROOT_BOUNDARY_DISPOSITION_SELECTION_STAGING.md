# Phase 152 C03e-LG — Production Durable Capability Executor-Root Boundary Disposition Selection

Status: `SELECTION_STAGING — VALIDATION_PENDING`

## 1. Purpose

C03e-LG selects only the first dormant executor-root boundary that can combine the already-materialized production-durable repeated-real-admission collection with the already-materialized requester-aware completion peer disposition while preserving the existing endpoint shutdown law.

LG does not materialize Rust source. It freezes the exact one-path source successor shape so a later independently gated materialization can be deterministic.

The exact C03e-LF source state is the immutable predecessor for this selection.

## 2. Exact predecessor

Canonical predecessor checkpoint:

`C03E_LF_PRODUCTION_DURABLE_CAPABILITY_OPERATION_BOUNDARY_LIFETIME_CUSTODY_SOURCE_MATERIALIZED`

Exact predecessor branch:

`phase-152-c03e-lf-production-durable-capability-operation-boundary-lifetime-custody-source-materialization`

Exact predecessor head:

`ed7763f6579efcf38de4e1578391b12d10307742`

Exact predecessor tree:

`3a66146773296ad94ba2ee469173b3675ca5e9f6`

LF remains frozen.

## 3. Source facts constraining LG

### 3.1 Existing LA durable repeated-admission collection

The existing dormant C03e-LA overload is:

`RemoteSessionExecutorRuntime::drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_capability(...)`

It lives in:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`

Its visibility is restricted to the `remote_session_executor_runtime` module root through:

`pub(in super::super::super)`

It returns exactly:

`Result<(), RemoteSessionPersistentCollectionConfigError>`

and publishes each recovered requester-aware worker completion only through its callback:

`C: FnMut(RecoverableRepeatedRealAdmissionRequesterAwareWorkerCompletion)`

The completion envelope is not a valid endpoint-sibling interface.

### 3.2 Existing FW completion disposition

The current LF source contains the already-materialized canonical disposer:

`dispose_recoverable_repeated_real_admission_requester_aware_worker_completion(...)`

The disposer consumes exactly one:

`RecoverableRepeatedRealAdmissionRequesterAwareWorkerCompletion`

and returns exactly:

```rust
(
    DeviceId,
    Result<
        RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop,
        RemoteSessionSpawnedWorkerJoinError,
    >,
)
```

Before returning that tuple it consumes the recovered authenticated-session owner according to the existing FV/FW terminal partition:

- `Cancelled` -> existing orderly-shutdown consuming close;
- typed requester-aware FL failure -> existing requester-aware terminal-failure consuming close;
- abnormal spawned-task completion -> the same requester-aware terminal-failure consuming close.

The exact FL/join result remains unchanged.

No requester-record retirement/removal is performed by this disposer.

### 3.3 Visibility fact

Both the LA overload and the FW disposer are callable from the `remote_session_executor_runtime` module root without changing the visibility of either private completion type or nested implementation helper.

Therefore LG does not select any visibility widening of:

- `RecoverableRepeatedRealAdmissionRequesterAwareWorkerCompletion`;
- the LA source module;
- the FW disposer;
- requester/rendezvous worker internals.

### 3.4 Existing endpoint-finalization law

The existing executor endpoint lifecycle computes the collection result first and then calls the existing helper:

`finish_remote_endpoint_shutdown(...)`

That helper preserves the exact order:

1. invoke endpoint close with the existing fixed shutdown code/reason;
2. synchronously drive the existing `wait_idle()` future on the retained executor runtime;
3. return the original collection result unchanged.

The existing endpoint lifecycle therefore closes and drains the bound endpoint even when the collection result is an existing configuration error.

LG must preserve this law exactly.

## 4. Selected boundary location

LG selects exactly one future Rust source path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

No other Rust path is selected for the immediate source-materialization successor.

In particular, the immediate successor must not modify:

- `remote_session_endpoint_lifecycle_runtime.rs`;
- the LA child module;
- `recoverable_spawned_requester_rendezvous_worker.rs`;
- `requester_rendezvous_retained_custody_dr_continuation.rs`;
- `remote_session_capability_runtime.rs` re-exports;
- `production_durable_capability_higher_owner_custody.rs`;
- `linux_bootstrap.rs`;
- durable bootstrap/custody source.

If the selected seam cannot be materialized by changing only `remote_session_executor_runtime.rs`, the source successor must STOP and a new selection/layout gate is required.

## 5. Selected executor-root overload

LG selects one new dormant method on `RemoteSessionExecutorRuntime` with the exact conceptual role:

`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability`

The exact method name above is normative for the immediate source successor.

Its visibility must be exactly parent-scoped:

`pub(super)`

This is a new narrow executor-to-endpoint sibling seam. It does not change visibility of any existing item and must not be re-exported from `remote_session_capability_runtime.rs`.

## 6. Selected input shape

The new overload must accept the existing endpoint-lifecycle inputs required by LA plus only the already-existing durable/requester-aware authority inputs:

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
- one boundary-safe completion callback `C`;
- the existing LA rejection callback `R`;
- the existing LA admission-failure callback `E`.

No additional runtime owner, raw transport, Tokio handle, requester provider, durable registry store, bootstrap credential, callback channel, queue, receipt, retry token, or restart token is selected.

## 7. Selected generic bounds

The overload must preserve the existing LA bounds exactly except for the intentionally adapted completion callback:

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

No stronger `Send`, `Sync`, `Clone`, `Copy`, `Unpin`, `'static`, or error-conversion bound may be added without a new gate.

## 8. Selected return shape

The overload must return exactly:

`Result<(), RemoteSessionPersistentCollectionConfigError>`

The result must be the exact LA collection result after the existing endpoint close/idle-drain law has run.

LG selects no new endpoint lifecycle error enum and no error flattening, wrapping, retry classification, logging side channel, or panic conversion.

## 9. Completion adaptation law

The new executor-root overload must pass a local adapter callback to LA.

For each exact LA completion, the adapter must perform exactly this semantic order:

1. receive one `RecoverableRepeatedRealAdmissionRequesterAwareWorkerCompletion` by value;
2. immediately pass it by value to the existing FW disposer;
3. receive the exact `(DeviceId, Result<...>)` returned by FW;
4. only after FW has consumed/closed the recovered peer, invoke the caller-supplied LG completion callback with that exact `DeviceId` and exact unchanged FL/join result.

Conceptually:

```rust
|completion| {
    let (device_id, result) =
        dispose_recoverable_repeated_real_admission_requester_aware_worker_completion(completion);
    on_completion(device_id, result);
}
```

The source successor may perform ordinary borrow/mutability syntax required by Rust, but it must not alter this ordering.

## 10. Peer disposition precedes external completion observation

The caller-supplied LG completion callback must never receive the recovered `AuthenticatedRemoteSessionRuntimeOwner` or the private FU completion envelope.

Peer disposition must complete before the callback is invoked.

This prevents a higher sibling caller from:

- reusing a terminal peer;
- bypassing the FV/FW close partition;
- retaining raw session-owner custody;
- performing requester-record cleanup from publisher completion;
- observing a half-disposed completion envelope.

The returned FL/join result remains exact and is not reclassified by LG.

## 11. Endpoint shutdown law

After the LA collection returns, the new overload must call the existing `finish_remote_endpoint_shutdown(...)` helper exactly once.

The selected order is therefore:

1. LA repeated-real-admission collection runs;
2. every published recovered requester-aware completion is consumed through FW before its external callback;
3. LA returns its exact `Result<(), RemoteSessionPersistentCollectionConfigError>`;
4. existing endpoint close runs with the existing shutdown code/reason;
5. existing endpoint `wait_idle()` is driven to completion on the retained executor runtime;
6. the original LA collection result is returned unchanged.

No endpoint close may occur before LA returns through this overload.

No second close, alternate close code, requester-aware close reason, timeout, abort, or replacement idle-drain mechanism is selected at the endpoint boundary.

## 12. Durable capability ownership law

The LG overload receives the existing outer:

`Arc<ProductionDurableCapabilityAuthority>`

by value and transfers it unchanged into LA.

LG selects:

- no new `Arc::new`;
- no new `Arc::clone` in the executor-root adapter;
- no durable-authority accessor;
- no extraction/return of the Arc;
- no global/static storage;
- no callback exposure of the Arc.

Existing Arc cloning already internal to LA for admitted workers remains unchanged and is not widened by LG.

LF operation-boundary outer-Arc custody remains separately dormant and unchanged. LG does not connect LF's process-operation owner to this executor seam.

## 13. Requester-aware authority semantics remain unchanged

LG performs no requester/rendezvous registration, retirement, removal, rollback, provider reset, bulk cleanup, or capacity sweep outside existing lower seams.

The shared requester/rendezvous authority is forwarded unchanged to LA.

FW remains the sole selected peer-disposition consumer for recovered repeated-admission completions.

Publisher `DeviceId` remains insufficient authority for requester-record cleanup.

## 14. Admission semantics remain unchanged

LG must not change LA's existing:

- max-active-worker validation;
- ready-completion-first reaping;
- duplicate expected-device preflight;
- at-most-one in-flight expected-device admission;
- AJ authentication/admission semantics;
- authenticated `DeviceId` map key;
- production-durable worker spawn lane;
- supervisor cancellation-all-then-drain behavior;
- post-shutdown in-flight admission drain;
- orderly close of a post-shutdown successful admission that is never inserted;
- exact rejection callback shape;
- exact admission-failure callback shape.

LG is composition only above those existing semantics.

## 15. No endpoint sibling mutation yet

Although `pub(super)` makes the selected executor-root overload available to the parent module boundary, LG does not select any caller in `remote_session_endpoint_lifecycle_runtime.rs`.

The existing active endpoint method remains byte-stable and continues to call the existing non-durable executor endpoint lifecycle.

A later independent gate must select any endpoint caller migration.

## 16. No process caller or bootstrap population

LG does not select or materialize:

- a `linux_bootstrap.rs` caller;
- a caller of the LF durable operation factory;
- construction of the LD higher-owner aggregate at a real process callsite;
- invocation of `bootstrap_production_durable_capability_authority_from_systemd_credentials()`;
- propagation of the LF retained outer Arc into the executor root;
- listener/readiness activation;
- process signal wiring;
- runtime startup/restart;
- deployment.

The durable executor overload remains dormant after its future source materialization until a later explicit caller gate.

## 17. No visibility widening

The immediate source successor must not change visibility of existing types/functions.

Specifically it must not widen:

- `RecoverableRepeatedRealAdmissionRequesterAwareWorkerCompletion`;
- `RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`;
- the FW disposer;
- the LA overload;
- `SharedRequesterRendezvousAuthority`;
- nested worker modules.

The only new visibility surface selected by LG is the new overload itself at exactly `pub(super)`.

No crate-level re-export is selected.

## 18. No new type or wrapper

LG selects no new completion struct, enum, error wrapper, trait, callback object, channel message, boxed trait object, or ownership aggregate.

The existing `(DeviceId, Result<...>)` produced by FW is sufficient for the selected parent-scoped callback boundary.

If Rust visibility or lint constraints prove that this exact existing tuple cannot cross the selected `pub(super)` method boundary without a new type or visibility change, the source successor must STOP rather than inventing one.

## 19. Immediate source-materialization ceiling

The immediate successor may change exactly one source path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

Permitted changes in that one file are limited to:

- imports required by the selected overload;
- the selected dormant `pub(super)` overload;
- narrowly scoped lint/dead-code annotations required solely because the seam has no caller yet;
- tests in that same file only if needed to prove the selected callback ordering/signature without changing runtime behavior.

Any need for a second source path, module re-export, endpoint caller, nested visibility edit, production owner edit, bootstrap edit, manifest edit, workflow edit, lockfile edit, Android edit, or deployment edit is a STOP condition.

## 20. Validation requirements for the source successor

A later source-materialization checkpoint must prove at minimum:

- exact predecessor is this LG selection head;
- exactly one selected Rust path changed;
- `cargo fmt --check` passes;
- Clippy passes under the repository's existing strict workflow;
- tests pass;
- workspace build passes;
- any path-triggered Android validation remains regression evidence only;
- `linux_bootstrap.rs` remains byte-identical;
- `remote_session_endpoint_lifecycle_runtime.rs` remains byte-identical;
- LF higher-owner custody source remains byte-identical;
- LA child source remains byte-identical;
- no caller exists for the new overload.

## 21. Security and privacy boundary

LG introduces no new identity authority.

- authenticated logical `DeviceId` remains sourced by existing admission/session ownership;
- requester authority remains governed by existing exact requester/rendezvous semantics;
- no transport address, task identity, Arc address, map slot, close code, callback order, or completion timing becomes authorization authority.

The existing fixed endpoint shutdown diagnostic and existing fixed requester-aware peer-close diagnostics remain unchanged.

No device/session/requester/policy/network secret is added to endpoint close reasons or new logs.

## 22. Explicitly rejected alternatives

LG rejects:

- exposing the FU completion type to the endpoint sibling;
- widening FW or LA visibility;
- adding a new public completion wrapper;
- moving FW disposition into the endpoint sibling;
- duplicating FV/FW peer-close logic;
- performing endpoint close inside the per-worker completion callback;
- returning before endpoint idle drain;
- cloning the durable authority merely for the adapter;
- invoking durable bootstrap from the executor root;
- migrating the active endpoint caller in the same checkpoint;
- modifying `linux_bootstrap.rs` in the same checkpoint.

## 23. Next-gate boundary

After a source successor materializes this exact executor-root overload and closes validation evidence, a later selection gate may examine one narrow next question:

whether the dormant `RemoteSessionEndpointLifecycleRuntime` sibling should gain a separately selected durable-capability endpoint-drive overload that delegates to this new executor-root seam.

That later gate must independently select the required durable/requester-aware inputs and must not be inferred from LG.

Process-level LF Arc propagation, durable bootstrap population, and real caller activation remain later still.

## 24. STOP

Keep LG selection docs-only.

Do not materialize Rust source in this checkpoint.

Do not mark any PR ready, merge, deploy, restart, mutate production state, populate durable bootstrap, add a real caller, migrate the endpoint sibling, expose private completion/session custody, clone/extract the retained outer Arc, or activate listener/network/runtime behavior without a later independent gate.
