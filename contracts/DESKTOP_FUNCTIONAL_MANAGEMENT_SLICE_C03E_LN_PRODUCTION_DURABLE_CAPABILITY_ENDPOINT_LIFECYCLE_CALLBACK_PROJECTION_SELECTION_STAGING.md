# Desktop Functional Management Slice C03e-LN — Production Durable Capability Endpoint Lifecycle Callback Projection Selection

Status: `STAGING_SELECTION`
Date: `2026-09-05`
Repository: `Gersi365/prw-executor-private`

## 1. Gate

Selected gate after exact-head validation and immutable evidence publication:

`C03E_LN_PRODUCTION_DURABLE_CAPABILITY_ENDPOINT_LIFECYCLE_CALLBACK_PROJECTION_SELECTED`

Selected closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_ENDPOINT_LIFECYCLE_CALLBACK_PROJECTION_SELECTION`

C03e-LN is documentation-only. It selects the next smallest dormant callback-projection boundary above the evidence-closed C03e-LM endpoint lifecycle caller seam. It does not materialize Rust/source behavior, widen requester-private lifecycle types, mutate the C03e-LM method, migrate `linux_bootstrap.rs`, change the existing higher-owner durable-capability custody wrapper, populate production durable authority, assemble executable callbacks, activate runtime behavior, merge, deploy, restart, or perform destructive cleanup.

## 2. Exact predecessor authority

C03e-LN is rooted only at the closed C03e-LM source materialization:

- branch: `phase-152-c03e-lm-production-durable-capability-endpoint-lifecycle-caller-migration-source-materialization`
- exact predecessor head / merge base: `a83c49dc3e9da48ae916621115b17bf0c0ffb7f2`
- exact predecessor tree: `e10fd094a4c4fcdbd97637999b66fe84723063d7`
- exact LM endpoint source blob: `59859c2659b94f68267eae105e3bcce928b77dc9`
- requester retained-custody source blob: `a8cb82f4eda44a207ba889bacd60c3f24c1901e7`
- LK durable executor source blob inherited unchanged: `297cf49b235537cf9a934eca82ef30e94364eba1`

C03e-LM remains draft/open/unmerged and evidence-closed. No merge, deployment, runtime activation, or repository configuration mutation is implied by using it as the exact LN predecessor.

## 3. Fresh source findings that constrain this selection

### 3.1 C03e-LM already owns the endpoint lifecycle adaptation

Exact LM path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

The file now contains dormant:

`RemoteSessionEndpointLifecycleRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability(...)`

with visibility exactly `pub(super)`.

That method consumes the existing endpoint owner, forwards the requester-DR current authority and the distinct `Arc<ProductionDurableCapabilityAuthority>` lane, forwards the requester-aware policy/rendezvous inputs, converts the retained supervisor shutdown once, delegates once to the exact LK durable executor lifecycle, and returns the existing `RemoteSessionPersistentCollectionConfigError` unchanged.

The legacy endpoint lifecycle remains unchanged.

### 3.2 Exact LM completion surface cannot be widened directly

The C03e-LM completion callback contains:

`RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`

The type is defined in:

`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

and is visible only as `pub(super)` to the enclosing `remote_session_capability_runtime` module.

Its exact terminal partition is:

- `Cancelled`;
- `Failed(RequesterRendezvousPostTerminalResponseSerialLifecycleError)`.

The nested requester lifecycle error is also parent-module bounded and contains two typed families:

- `Ingress(...)`;
- `RequesterResponse(...)`.

Therefore changing the LM method itself to `pub(crate)` would begin a recursive requester-private visibility widening chain. C03e-LN rejects that approach.

### 3.3 Existing FW peer disposition is already consumed below LM

The evidence-closed FW semantics and the exact LK durable executor method already consume every recovered requester-aware completion through the existing terminal peer-disposition seam before the outward completion callback is invoked.

The resulting exact endpoint callback still carries the unchanged requester-aware FL/join terminal result, but the authenticated peer owner has already been consumed through the selected orderly-shutdown or requester-aware terminal-failure disposition.

C03e-LN therefore does not select any FW classifier/disposer visibility widening, duplicate peer disposition, new close code, peer reuse, restart, reconnect, or requester-record cleanup.

### 3.4 Existing abnormal join is already a bounded crate-visible class

`RemoteSessionSpawnedWorkerJoinError` is crate-visible and currently contains the bounded `AbnormalTaskCompletion` variant.

C03e-LN does not widen or replace this type. The selected projection converts that existing abnormal-join family into one bounded completion projection variant while keeping the requester-private FL error payloads private.

### 3.5 Higher-owner durable custody already exists

Exact LM source already retains production durable capability authority in the separately materialized higher-owner custody path. The higher-owner operation wrapper currently delegates to the legacy Linux production requester/rendezvous operation and then releases the retained outer durable authority after that delegated operation returns.

C03e-LN therefore does not select a new durable-authority owner, provider, aggregate, `Arc::new`, `Arc::clone`, bootstrap population lane, or higher-owner caller mutation.

### 3.6 Linux callback policy remains a later boundary

`linux_bootstrap.rs` currently owns generic/injected legacy completion, rejection and admission-failure callbacks. The requester/rendezvous operation aggregate still delegates to the legacy production remote-process operation while requester-aware custody values remain non-executable.

C03e-LN does not select construction of legacy `RemoteSessionRegisteredWorkerCompletion`, `RemoteSessionExpectedDeviceAdmissionRejection`, or `RemoteSessionRepeatedAdmissionFailure` aggregates from the requester-aware path. It also does not select logging, counter, startup-error, executable callback, or caller policy.

Those policies remain independently gated after a bounded crate-visible endpoint completion surface exists.

## 4. Selected boundary

C03e-LN selects one future additive dormant projection adapter in exactly the existing LM endpoint source file:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

The future source successor may add exactly:

1. one bounded crate-visible completion projection enum; and
2. one additive crate-visible endpoint lifecycle adapter method that delegates exactly once to the unchanged C03e-LM method and performs only completion projection at the callback boundary.

No second source path is selected.

## 5. Selected bounded completion projection

The selected future type is:

`RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection`

Selected effective visibility:

`pub(crate)`

Selected shape:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection {
    Cancelled,
    IngressFailure,
    RequesterResponseFailure,
    AbnormalTaskCompletion,
}
```

This enum is a boundary projection, not a new execution/error authority. It carries no peer owner, requester identity, capability authority, session owner, transport, retry token, cleanup capability, raw private error payload, or reconnect/restart authority.

### 5.1 Exact selected mapping

The future adapter must map the exact unchanged LM completion result as follows:

- `Ok(RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Cancelled)` -> `Cancelled`;
- `Ok(RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Failed(RequesterRendezvousPostTerminalResponseSerialLifecycleError::Ingress(_)))` -> `IngressFailure`;
- `Ok(RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Failed(RequesterRendezvousPostTerminalResponseSerialLifecycleError::RequesterResponse(_)))` -> `RequesterResponseFailure`;
- `Err(RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion)` -> `AbnormalTaskCompletion`.

The authenticated `DeviceId` supplied by the existing LM callback must be forwarded unchanged beside the projection.

### 5.2 Deliberate payload ceiling

Requester-private ingress and requester-response error payloads remain private. The projection exposes only the exact terminal family needed to cross the crate-visible endpoint boundary safely.

C03e-LN does not select:

- `pub(crate)` visibility for `RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`;
- `pub(crate)` visibility for `RequesterRendezvousPostTerminalResponseSerialLifecycleError`;
- visibility changes to either nested payload error family;
- stringification/debug formatting as an API surface;
- cloning/copying private payload values beyond what the existing result already performs;
- conversion into a legacy authenticated-worker stop/error value.

## 6. Selected endpoint projection adapter

The selected future additive method is:

`RemoteSessionEndpointLifecycleRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability_projection(...)`

Selected effective visibility:

`pub(crate)`

The adapter must preserve the C03e-LM input/ownership surface, except that its completion callback receives only:

```text
(DeviceId, RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection)
```

instead of the requester-private FL/join result.

The rejection and admission-failure callback surfaces remain the already crate-visible exact LM forms and are forwarded unchanged.

Conceptual composition:

```text
crate-visible projection adapter
    -> unchanged LM pub(super) durable endpoint lifecycle exactly once
       -> completion callback only:
          exact private requester-aware result
          -> bounded four-family projection
          -> unchanged DeviceId + projection to caller
       -> rejection callback unchanged
       -> admission-failure callback unchanged
    -> unchanged RemoteSessionPersistentCollectionConfigError
```

The source successor must not duplicate endpoint/executor lifecycle behavior around that call.

## 7. Ownership and authority law

The selected adapter preserves every existing C03e-LM ownership and authority boundary.

### 7.1 Endpoint owner

`self` is consumed exactly once by the new adapter and moved into exactly one invocation of the existing C03e-LM method.

The adapter must not separately destructure the endpoint owner, convert supervisor shutdown itself, close transport, wait idle, bind another endpoint, or invoke a second executor lifecycle.

### 7.2 Requester-DR current authority

The existing `&SharedCurrentCapabilityAuthority<P>` remains the requester-DR/current admission authority lane and is forwarded unchanged.

### 7.3 Production durable capability authority

Exactly one caller-owned `Arc<ProductionDurableCapabilityAuthority>` is accepted by value and moved unchanged into the existing C03e-LM method.

The adapter must not perform a new `Arc::new`, `Arc::clone`, authority bootstrap, provider lookup, cache insertion, authority conversion, reload, fallback, or direct authorization.

### 7.4 Requester/rendezvous policy and authority

The existing typed requester-aware policy source and `SharedRequesterRendezvousAuthority` are forwarded unchanged. No new requester lifecycle owner, record cleanup, publication continuation, candidate state, reachability state, or provider call is selected.

## 8. Completion ordering and side-effect law

The selected future adapter is projection-only.

For every completion observed by its caller:

1. the lower FW peer disposition has already consumed the recovered authenticated peer owner;
2. the unchanged LM callback receives the exact requester-aware FL/join result;
3. the adapter maps that result to exactly one selected projection variant;
4. the caller callback receives the unchanged `DeviceId` and that projection;
5. no additional peer, requester, transport, durable-authority, restart, retry, reconnect, cleanup, reachability, candidate, or publication side effect occurs in the projection closure.

No projection callback may run before the lower existing peer disposition.

## 9. Legacy and requester-private surfaces frozen

The immediate future source materialization must leave the following source behavior unchanged:

- existing legacy `drive_repeated_real_remote_admission_endpoint_lifecycle(...)`;
- C03e-LM `drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability(...)` body, signature and `pub(super)` visibility;
- requester retained-custody stop/error visibility;
- FW disposer and peer-disposition classifier;
- LK durable executor lifecycle;
- legacy executor completion/rejection/failure aggregates;
- `production_durable_capability_higher_owner_custody.rs`;
- `linux_bootstrap.rs`;
- `lib.rs` module registration;
- `main.rs` and executable startup path.

If source materialization requires any of those mutations, STOP and open a new selection gate.

## 10. Exact source ceiling for the immediate successor

The immediate source successor may mutate exactly one path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Allowed additions are limited to the enum and adapter selected above, plus only mechanical imports/attributes/doc comments in that same file required for compilation and canonical formatting.

No second source path, test path, manifest, lockfile, workflow, Android, systemd, deployment, packaging, generated, or executable path is selected.

The exact LM predecessor blob for this one source path is:

`59859c2659b94f68267eae105e3bcce928b77dc9`

A future source checkpoint must re-read that exact path from its exact predecessor branch before writing. If the blob differs or any successor already exists, STOP and re-audit.

## 11. Explicit non-selection

C03e-LN does not select or authorize:

- Rust/source materialization in LN itself;
- raw requester-lifecycle visibility widening;
- callback projection outside the selected endpoint file;
- mutation of FW disposition semantics or close codes;
- requester-record retirement/removal;
- conversion into legacy worker completion/rejection/failure aggregates;
- `linux_bootstrap.rs` caller migration;
- mutation of the higher-owner operation wrapper;
- production durable-authority bootstrap/population;
- executable aggregate assembly;
- startup error policy;
- callback logging/counters/metrics policy;
- candidate/reachability continuation;
- target dialing, reconnect, retry or peer reuse;
- endpoint/listener/readiness/runtime activation;
- `run()` or `main.rs` mutation;
- manifest/lockfile/workflow/Android source mutation;
- merge, ready-for-review conversion, deploy, restart/recovery;
- repository configuration mutation;
- PR close, branch deletion, force update or history rewrite;
- destructive cleanup.

## 12. Validation requirements

C03e-LN closure authority is only its exact final docs-only head.

Before evidence closure:

1. verify exact predecessor remains C03e-LM head `a83c49dc3e9da48ae916621115b17bf0c0ffb7f2`;
2. verify the LN branch is ahead only by its documentation commit(s) and behind by zero;
3. verify exactly one changed path, the selected contract;
4. verify zero Rust/source/runtime/manifest/lockfile/workflow/Android/packaging/executable changes;
5. require exact-final-head `PRW Rust Validation` success, including locked graph, formatting, Clippy, tests and workspace build;
6. classify path-inapplicable workflow skips as `SKIPPED`, never PASS;
7. claim Android validation only if an exact-final-head Android workflow actually runs and succeeds;
8. re-read the exact final branch head after CI before evidence publication;
9. freeze the immutable audit bytes and SHA-256 before Drive upload;
10. verify exact-title uniqueness before upload, raw Drive byte/hash readback after upload, and exact-title uniqueness after publication;
11. update the draft PR only after exact evidence is known;
12. re-read final PR/branch state and keep the PR draft/open/unmerged.

## 13. Determinism and retry law

Before every write, re-audit the latest C03e namespace and exact branch head.

After LN branch creation, each write must require the exact expected current head. If branch drift or an independently created successor appears, STOP rather than overwrite.

Ordinary retry must not create a duplicate contract file, duplicate branch, duplicate PR, duplicate immutable Drive audit, second semantic contract commit, or history rewrite.

Any correction required by validation must stay documentation-only and preserve the exact selected boundary. If a correction would change the selected semantic/source ceiling, STOP and open a separately audited reselection rather than silently widening LN.

## 14. Closure and successor boundary

After exact-final-head validation and immutable evidence publication, record:

`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

Gate:

`C03E_LN_PRODUCTION_DURABLE_CAPABILITY_ENDPOINT_LIFECYCLE_CALLBACK_PROJECTION_SELECTED`

Closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_ENDPOINT_LIFECYCLE_CALLBACK_PROJECTION_SELECTION`

After C03e-LN closure, STOP.

The immediate later source checkpoint may only materialize the selected one-file bounded endpoint completion projection adapter. It must not at the same time migrate the higher-owner operation, construct executable callback aggregates, change `linux_bootstrap.rs`, populate production durable authority, activate runtime behavior, merge, deploy, restart, or widen any requester-private type.

Only after that source materialization is independently validated and evidence-closed may another fresh selection audit decide whether the next smallest boundary is higher-owner operation caller migration or a separately required callback-policy/aggregate adapter.
