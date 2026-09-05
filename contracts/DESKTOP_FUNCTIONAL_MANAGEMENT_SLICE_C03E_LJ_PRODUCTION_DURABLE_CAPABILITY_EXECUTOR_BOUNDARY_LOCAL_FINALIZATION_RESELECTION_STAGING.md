# Phase 152 C03e-LJ — Production Durable Capability Executor-Boundary Local Finalization Reselection

Status: `SELECTION_STAGING — VALIDATION_PENDING`

## 1. Purpose

C03e-LJ reselects only the endpoint-finalization composition for the dormant production-durable executor boundary after exact C03e-LI validation disproved one C03e-LH layout assumption.

LG selected the boundary semantics. LH correctly reselected the existing small LA child as the safe physical source path, but assumed the existing `finish_remote_endpoint_shutdown(...)` helper was reachable from that descendant. LI materialized that assumption and Rust validation proved it false.

LJ does not materialize Rust source. It preserves the same observable endpoint shutdown law while selecting one exact local expression that is reachable from the already-selected small child without changing the 80,259-byte executor root, widening visibility, creating a helper, or adding a second source path.

## 2. Canonical predecessor

The canonical source predecessor for the later materialization remains the exact evidence-closed C03e-LH head, not the failed LI attempt.

Branch:

`phase-152-c03e-lh-production-durable-capability-executor-boundary-modular-source-layout-reselection`

Head:

`18a123a929f45ad8fc4b8c0a9626a01ee6c40cdf`

Tree:

`08cf0cb9bf512c875b297395a0a5c0f7c2f7da5d`

The failed LI head is retained only as negative validation evidence:

`af84b93fa3daefa0bb597501468a26a6607a129a`

No later materialization may inherit LI's failed source blob.

## 3. Exact LI failure evidence

LI changed exactly one source path and attempted to import:

`super::super::super::finish_remote_endpoint_shutdown`

PRW Rust Validation #1583, run `33963931128`, failed during Clippy/compile with:

`E0432: unresolved import super::super::super::finish_remote_endpoint_shutdown`

Formatting had already passed. Tests and workspace build were skipped after the compile failure.

Exact predecessor audit locates the helper lexically inside:

```rust
mod repeated_real_admission_supervisor {
    ...
    fn finish_remote_endpoint_shutdown<R, C, W>(...) -> R { ... }
}
```

The LA child is under the distinct subtree:

`remote_session_executor_runtime::recoverable_spawned_requester_rendezvous_worker::repeated_real_admission_requester_aware_persistent_fl_integration::production_durable_repeated_real_admission_collection`

Therefore the helper is a private sibling-module item, not an ancestor item reachable from the LA child.

## 4. Rejected corrections to LI

LJ explicitly rejects treating LI as a patchable materialization checkpoint.

Do not:

- add a second LI source path;
- widen `finish_remote_endpoint_shutdown`;
- move the helper;
- edit `remote_session_executor_runtime.rs`;
- add a new helper solely to bypass sibling privacy;
- rewrite or reconstruct the 80,259-byte root file through truncated connector transport;
- use the failed LI source blob as a successor base.

LI remains draft/open/unmerged as failure evidence only.

## 5. Physical source path remains the LH-selected child

The later source-materialization successor may change exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`

Exact LH predecessor blob:

`291ef3bd99d1b40daa77861af3107212eddad5a6`

Exact predecessor size:

`8,345 bytes`

The file is already module-registered and already contains the LA durable repeated-real-admission `impl RemoteSessionExecutorRuntime`.

No module-registration change is selected.

## 6. Existing executor-root file remains frozen

The following path must remain byte-identical to LH:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

Exact LH blob:

`ef370ca500f118bc067097ddb8f5c37ab597b214`

Exact size:

`80,259 bytes`

No import, visibility, helper, constant, method, test, module declaration, or formatting edit is selected in that file.

## 7. Dormant boundary method remains unchanged

The later source successor still selects the exact dormant method name:

`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability`

on:

`RemoteSessionExecutorRuntime`

Its effective sibling-facing visibility remains exactly the LH-selected:

`pub(in super::super::super::super)`

from the selected descendant location, resolving to the existing `remote_session_capability_runtime` parent domain.

No existing item visibility changes.

## 8. Input and return shape remain LG/LH exact

The method must retain the LG/LH-selected inputs:

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
- rejection callback `R`;
- admission-failure callback `E`.

Return type remains exactly:

`Result<(), RemoteSessionPersistentCollectionConfigError>`

No new wrapper, enum, trait, channel, aggregate, or error conversion is selected.

## 9. Completion adaptation remains exact

For every completion published by the existing LA durable collection, the new boundary must perform exactly:

```rust
|completion| {
    let (device_id, result) =
        dispose_recoverable_repeated_real_admission_requester_aware_worker_completion(completion);
    on_completion(device_id, result);
}
```

The existing FW disposer must consume the recovered authenticated-session owner before the outward completion callback runs.

The callback receives only the authenticated `DeviceId` and the exact unchanged requester-aware FL/join result.

No requester-record cleanup or raw peer/session-owner exposure is selected.

## 10. Durable capability ownership remains exact

The caller-owned outer:

`Arc<ProductionDurableCapabilityAuthority>`

moves by value into the existing LA durable collection.

The boundary selects:

- no new `Arc::new`;
- no boundary-level `Arc::clone`;
- no accessor or extraction;
- no callback exposure;
- no global/static storage.

Existing Arc cloning internal to LA for admitted workers remains unchanged.

## 11. Existing endpoint-finalization law

The canonical existing helper has exact observable behavior:

1. endpoint close with code `0` and reason `b"remote endpoint shutdown"`;
2. synchronous drive of the exact `transport_runtime.wait_idle()` future through the retained executor runtime;
3. return the original collection result unchanged.

The helper itself remains private and unchanged inside `repeated_real_admission_supervisor`.

LJ does not select helper reuse because sibling privacy makes that composition unreachable from the modular child.

## 12. Selected local finalization expression

After the existing LA durable collection returns its exact result, the later source successor must express the same law locally and directly, without defining a new helper or constants:

```rust
transport_runtime.close(0, b"remote endpoint shutdown");
self.runtime.block_on(transport_runtime.wait_idle());
result
```

This exact three-step expression is normative.

It is not permission to create a generalized finalization API or alternate shutdown path.

## 13. Exact ordering law

The selected boundary order is:

1. run the existing LA durable repeated-real-admission collection;
2. for each recovered requester-aware completion, run FW peer disposition before outward callback observation;
3. wait until LA has fully returned its exact collection result;
4. call `transport_runtime.close(0, b"remote endpoint shutdown")` exactly once;
5. synchronously drive `transport_runtime.wait_idle()` exactly once using `self.runtime.block_on(...)`;
6. return the exact original LA collection result unchanged.

No endpoint close may occur before LA returns from this boundary.

No callback may run with an undisposed recovered peer.

## 14. Why direct executor runtime access is selected

The selected LA child already compiles existing code that uses:

`self.runtime.block_on(...)`

inside its `impl RemoteSessionExecutorRuntime`.

Therefore the descendant already has lexical access to the ancestor-private executor runtime field. LJ does not require a new runtime accessor, Tokio handle, generic `block_on` API, or visibility widening.

The direct `self.runtime.block_on(transport_runtime.wait_idle())` expression is limited solely to reproducing the existing endpoint idle-drain law after the durable collection returns.

## 15. No new shutdown constants

The later source successor must not define replacement constants for the existing private sibling constants.

It must use the exact selected literals directly:

- code: `0`;
- reason: `b"remote endpoint shutdown"`.

This prevents a second mutable naming surface while preserving the exact existing diagnostic bytes.

## 16. No semantic expansion

LJ does not select:

- a timeout;
- alternate close code/reason;
- retry;
- abort;
- second close;
- asynchronous detached idle drain;
- logging side channel;
- metrics side channel;
- panic conversion;
- result wrapping or flattening;
- requester-record cleanup;
- peer reuse;
- replacement worker/session admission;
- runtime restart.

## 17. Existing LA semantics remain unchanged

The later source successor must not alter the existing LA method body or its semantics, including:

- capacity validation;
- ready completion reaping;
- duplicate expected-device preflight;
- one in-flight expected-device admission;
- authentication/admission flow;
- authenticated `DeviceId` active-map key;
- durable worker spawn lane;
- shutdown cancellation and drain;
- post-shutdown in-flight admission drain;
- orderly close of a successful post-shutdown admission not inserted;
- exact rejection callback;
- exact admission-failure callback.

Only imports plus the new dormant boundary method are permitted in the selected child.

## 18. Existing FW semantics remain unchanged

The existing FW disposer remains the sole selected consumer of each recovered requester-aware repeated-admission completion before outward callback observation.

No visibility widening or duplication of the FW terminal peer-disposition classifier is selected.

## 19. No endpoint sibling caller

The later source successor remains dormant.

Do not modify:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

The existing active endpoint method and callers remain unchanged.

A later independent selection gate must decide whether to add a durable-capability endpoint sibling overload.

## 20. No process/bootstrap propagation

LJ does not select or materialize:

- a `linux_bootstrap.rs` caller;
- LF durable operation caller wiring;
- LD aggregate population at a real process callsite;
- durable-capability bootstrap credential loading;
- outer Arc propagation from LF into this executor boundary;
- listener/readiness activation;
- process signal wiring;
- deployment or restart.

## 21. Immediate materialization ceiling

A later source-materialization checkpoint based on this LJ selection may change exactly one Rust file: the LH-selected LA child path in section 5.

Permitted changes are limited to:

- imports required by the existing FW disposer and exact callback result types;
- the dormant boundary overload;
- narrowly scoped lint/dead-code annotations required because the overload has no caller;
- the exact local finalization expression in section 12.

Any need for a second source path, root-file visibility edit, helper migration, endpoint caller, bootstrap edit, manifest/workflow/lockfile change, or runtime activation is a STOP condition.

## 22. Validation requirements

The later source successor must prove at minimum:

- exact predecessor is this LJ selection head;
- source content begins from the exact LH child blob `291ef3bd99d1b40daa77861af3107212eddad5a6`;
- exactly one selected Rust path changes;
- root executor blob remains `ef370ca500f118bc067097ddb8f5c37ab597b214`;
- endpoint sibling remains byte-identical;
- `linux_bootstrap.rs` remains byte-identical;
- LF higher-owner custody remains byte-identical;
- `cargo fmt --check` passes;
- Clippy passes with repository `-D warnings`;
- tests pass;
- workspace build passes;
- path-triggered Android validation is regression evidence only;
- no caller exists for the new overload.

## 23. Security and privacy boundary

No new authorization authority is introduced.

Authenticated logical `DeviceId` remains sourced by existing authenticated admission/session ownership. Requester/rendezvous authority remains governed by existing lower seams. Transport address, callback order, task identity, Arc identity, close code, shutdown reason, or completion timing must not become authorization authority.

No secret or user-derived content may be added to the fixed shutdown reason.

## 24. Explicitly rejected alternatives

LJ rejects:

- rewriting the 80,259-byte root executor file;
- widening `finish_remote_endpoint_shutdown` to `pub(super)` or broader;
- moving the existing helper;
- creating a second helper abstraction;
- importing the private helper through unsupported sibling paths;
- defining duplicate shutdown constants;
- changing the shutdown code/reason;
- skipping idle drain;
- returning before idle drain;
- closing before LA returns;
- exposing the private FU completion envelope;
- migrating endpoint/process callers in the same checkpoint.

## 25. Next-gate boundary

After a later source checkpoint successfully materializes this exact one-file dormant boundary and closes validation evidence, a subsequent independent selection gate may examine only the next narrow question: whether `RemoteSessionEndpointLifecycleRuntime` should gain a dormant durable-capability endpoint-drive overload that delegates to the validated executor boundary.

LF Arc propagation, durable bootstrap population, and real process caller migration remain later gates.

## 26. STOP

Keep LJ docs-only.

Do not materialize Rust source in this checkpoint.

Do not mark any PR ready, merge, deploy, restart, add endpoint/process callers, populate durable bootstrap, connect LF custody, widen private helper visibility, modify the root executor file, or activate listener/network/runtime behavior without a later independent gate.
