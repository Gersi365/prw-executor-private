# Desktop Functional Management Slice C03e-LL — Production Durable Capability Endpoint Lifecycle Caller Migration Selection

Status: `STAGING_SELECTION`
Date: `2026-09-05`
Repository: `Gersi365/prw-executor-private`

## 1. Gate

Selected gate after exact-head validation and immutable evidence publication:

`C03E_LL_PRODUCTION_DURABLE_CAPABILITY_ENDPOINT_LIFECYCLE_CALLER_MIGRATION_SELECTED`

Selected closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_ENDPOINT_LIFECYCLE_CALLER_MIGRATION_SELECTION`

C03e-LL is documentation-only. It selects the next smallest dormant caller-adaptation boundary above the C03e-LK production-durable executor lifecycle. It does not materialize Rust/source behavior, populate a durable authority from production, invoke the Agent executable path, bind or activate a new endpoint, migrate `linux_bootstrap.rs`, widen parent-private requester lifecycle types, or change runtime behavior.

## 2. Exact predecessor authority

C03e-LL is rooted only at the closed C03e-LK source materialization:

- branch: `phase-152-c03e-lk-production-durable-capability-executor-boundary-local-finalization-source-materialization`
- exact predecessor head / merge base: `d90fdbf9068be4a4083eef7981e527c114f38ce8`
- exact predecessor tree: `a411e1e0d5958ca3e43bb4a6d8afed51e6c2e19e`
- LK changed source blob: `297cf49b235537cf9a934eca82ef30e94364eba1`
- LK endpoint-lifecycle source blob: `999fb2d2deed48e4c3ffee5af17d2b521642eff8`
- LK root executor source blob: `ef370ca500f118bc067097ddb8f5c37ab597b214`

C03e-LK is evidence-closed and remains draft/open/unmerged. No merge, deployment, runtime activation, or repository configuration mutation is implied by using it as the exact selection predecessor.

## 3. Exact LK source state relevant to this selection

On exact LK head, the durable-capability path has reached the executor lifecycle boundary but not the endpoint-lifecycle owner.

### 3.1 LK executor boundary now exists

The exact LK child source adds dormant:

`RemoteSessionExecutorRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability(...)`

with effective visibility constrained to the enclosing `remote_session_capability_runtime` module.

That executor method:

1. invokes the already-materialized LA production-durable repeated real-admission collection;
2. consumes recovered requester-aware completions through the existing FW peer disposition before outward completion observation;
3. preserves the exact `(DeviceId, requester-aware FL/join result)` completion surface;
4. moves the caller-supplied outer `Arc<ProductionDurableCapabilityAuthority>` into the durable collection lane unchanged;
5. preserves the existing endpoint finalization order after the collection returns:

```rust
transport_runtime.close(0, b"remote endpoint shutdown");
self.runtime.block_on(transport_runtime.wait_idle());
result
```

It remains dormant and has no endpoint-owner caller on exact LK source.

### 3.2 Existing endpoint-lifecycle owner still delegates to the legacy executor lane

Exact LK file:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Exact blob:

`999fb2d2deed48e4c3ffee5af17d2b521642eff8`

`RemoteSessionEndpointLifecycleRuntime` owns exactly:

- one `RemoteSessionExecutorRuntime`;
- one already-bound `AgentRemoteTransportRuntime`;
- one `RemoteSessionSupervisorShutdownSignal`.

Its existing public:

`drive_repeated_real_remote_admission_endpoint_lifecycle(...)`

consumes `self`, destructures those exact retained values, and calls the legacy executor lifecycle. That method remains valid and is not selected for mutation, replacement, removal, or semantic widening.

### 3.3 The durable callback surface is intentionally parent-module bounded

The exact LK durable completion callback contains:

`RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`

The defining requester/rendezvous module exposes this enum as `pub(super)`, making it available inside its parent `remote_session_capability_runtime` module but not as a crate-wide surface.

That visibility is authoritative for C03e-LL. A new endpoint method cannot be selected as `pub(crate)` without either:

1. widening the requester-lifecycle result type; or
2. translating the LK callback into a different outward aggregate.

Both would be additional semantic/visibility boundaries and are explicitly outside this checkpoint.

### 3.4 The next higher production/executable layer is not ready for durable migration

On exact LK source, `crates/prw-agent/src/linux_bootstrap.rs` still invokes the existing endpoint lifecycle and contains no `ProductionDurableCapabilityAuthority` durable-capability population/custody lane.

Moving directly into `linux_bootstrap.rs` would therefore combine at least three independently gated concerns:

1. endpoint-owner adaptation to the already-materialized LK executor seam;
2. projection or widening of the parent-private requester-aware terminal callback surface; and
3. production/executable durable-capability authority population/custody.

C03e-LL rejects that combined step. The endpoint-owner adaptation is selected first and remains strictly inside `remote_session_capability_runtime`.

## 4. Selected boundary

C03e-LL selects exactly one future additive dormant method on `RemoteSessionEndpointLifecycleRuntime`:

`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability`

The method must consume the existing endpoint lifecycle owner and delegate once to the exact LK executor method of the same suffix.

Selected high-level composition:

```text
RemoteSessionEndpointLifecycleRuntime {
    executor,
    transport,
    supervisor_shutdown,
}
    + existing typed repeated-admission inputs
    + requester-DR SharedCurrentCapabilityAuthority
    + Arc<ProductionDurableCapabilityAuthority>
    + requester-aware policy source
    + SharedRequesterRendezvousAuthority
    -> executor.drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability(
           ...,
           &transport,
           ...,
           supervisor_shutdown.into_shutdown(),
           ...
       )
    -> unchanged RemoteSessionPersistentCollectionConfigError result
```

No second executor lifecycle, endpoint bind, endpoint close, idle drain, worker loop, peer disposition, durable authorization implementation, requester-rendezvous mutation, callback projection, or callback adaptation is selected at this owner layer. Those semantics remain delegated to already-materialized lower seams or separately gated above this boundary.

## 5. Selected ownership and authority forwarding

The source successor must preserve distinct authority roles.

### 5.1 Requester-DR current authority

The existing:

`&SharedCurrentCapabilityAuthority<P>`

continues to represent the current requester-DR/session admission authority lane required by the lower durable path. It must not be replaced by the durable capability authority.

### 5.2 Durable capability authority

Exactly one caller-supplied:

`Arc<ProductionDurableCapabilityAuthority>`

is accepted by value and moved unchanged into the LK executor method.

This endpoint owner must not:

- construct the durable authority;
- clone the underlying authority type;
- reload durable state;
- perform authorization itself;
- convert durable authority into requester-DR authority;
- cache or globalize the authority;
- fabricate a fallback authority.

Any later production population/custody of this outer `Arc` remains separately gated.

### 5.3 Requester/rendezvous authority and policy source

The existing typed requester-aware inputs are forwarded unchanged:

- `Arc<PS>` where `PS: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static`;
- `&SharedRequesterRendezvousAuthority`.

The endpoint owner performs no requester registration, cleanup, lookup, mutation, or policy evaluation.

## 6. Selected lifecycle custody

The future method must consume `self` exactly once and destructure:

```text
executor
transport
supervisor_shutdown
```

It then:

1. borrows the retained `transport` only for the one LK executor invocation;
2. converts the retained `supervisor_shutdown` through the existing `into_shutdown()` future exactly once;
3. delegates to the LK executor method exactly once;
4. returns that exact result unchanged.

The endpoint owner must not duplicate LK finalization. In particular, it must not call:

- `transport.close(...)`;
- `transport.wait_idle()`;
- another executor finalizer;
- another shutdown signal conversion.

Endpoint close and idle drain remain owned by the exact LK executor method.

## 7. Selected callback and error surfaces

The source successor must preserve the exact LK durable callback surfaces rather than remap them into legacy wrappers or widen their types.

### Completion

Forward an existing typed callback with the semantic shape:

```text
FnMut(
    DeviceId,
    Result<
        RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop,
        RemoteSessionSpawnedWorkerJoinError,
    >,
)
```

The endpoint owner receives no recovered session owner and performs no peer disposition. FW/LK already consume peer custody before this callback.

`RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop` remains parent-module bounded exactly as on LK. C03e-LL does not widen it.

### Rejection

Forward the exact lower durable rejection shape:

```text
FnMut(
    RemoteSessionExpectedDeviceAdmissionRejectionReason,
    RemoteSessionExpectedDeviceAdmissionRequest<D, T>,
)
```

No legacy rejection aggregate translation is selected.

### Admission failure

Forward the exact lower durable admission-failure shape:

```text
FnMut(DeviceId, RemoteSessionRealAdmissionError)
```

No legacy repeated-admission failure aggregate translation is selected.

### Return error

Return the existing:

`RemoteSessionPersistentCollectionConfigError`

unchanged.

No new endpoint-lifecycle error enum, startup mapping, retry policy, fallback, logging envelope, exit mapping, or error suppression is selected.

## 8. Selected visibility

The future additive endpoint method is selected with parent-module visibility only:

`pub(super)`

when defined inside `remote_session_endpoint_lifecycle_runtime`.

Effective visibility is therefore limited to the enclosing:

`crate::remote_session_capability_runtime`

This matches the current visibility ceiling of the requester-aware terminal callback type and the LK executor seam.

Rationale:

- the exact LK callback surface contains a `pub(super)` requester-lifecycle result type;
- the endpoint sibling can consume the LK executor seam entirely within the same parent module;
- `pub(crate)` would exceed the current callback-type visibility and would require a separately selected widening or projection boundary;
- a public API is unnecessary and unauthorized.

C03e-LL does not authorize visibility widening of the requester lifecycle type, the endpoint method, the LK executor method, or any other owner/type.

## 9. Exact first source-successor ceiling

After C03e-LL closes, the immediate source successor may change only:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`

Permitted scope is limited to:

1. imports required solely by the new durable endpoint-lifecycle method;
2. one additive dormant `pub(super)` method named exactly `drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability`;
3. exact consumption/destructuring of the existing endpoint lifecycle owner;
4. exact forwarding of the existing requester-DR authority;
5. exact by-value forwarding of one `Arc<ProductionDurableCapabilityAuthority>`;
6. exact forwarding of requester-aware policy/rendezvous inputs;
7. exact forwarding of session authentication, expected requests, timing and LK callback shapes;
8. exactly one `supervisor_shutdown.into_shutdown()` conversion;
9. exactly one call to the LK durable executor lifecycle method;
10. unchanged return of `RemoteSessionPersistentCollectionConfigError`;
11. focused compile/source-shape tests only if they fit the same file without fabricating runtime authority or widening private types.

The source successor must stop if implementation requires any second repository path, crate-wide/public API, requester-lifecycle type visibility change, callback projection/translation, new owner type, new durable provider/source, new error envelope, legacy method mutation, root executor mutation, `linux_bootstrap.rs` mutation, `main.rs` mutation, runtime activation, or deployment configuration.

## 10. Explicitly unchanged legacy path

The existing public:

`RemoteSessionEndpointLifecycleRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle(...)`

must remain source- and behavior-unchanged in the immediate successor.

C03e-LL does not select replacement, deprecation, call-site migration, callback translation, or removal of the legacy endpoint lifecycle.

The new durable method is an additive dormant parent-module lane only.

## 11. Identity invariant

Continue to preserve:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

The following remain distinct and may not substitute for logical identity or authority:

- bind address;
- observed bound address;
- connectivity candidates;
- worker-limit configuration;
- PRWM `request_id` correlation;
- requester identity;
- target device identity;
- transport endpoint;
- durable capability authority;
- requester-DR current authority.

C03e-LL introduces no identity source, lookup, re-selection, or endpoint interpretation.

## 12. Explicit exclusions

C03e-LL does not perform or authorize:

- Rust/source mutation in this selection checkpoint;
- mutation of the existing endpoint lifecycle method;
- mutation of the LK executor source;
- requester-lifecycle result visibility widening;
- callback projection or legacy-wrapper translation;
- `linux_bootstrap.rs` mutation;
- `main.rs` mutation;
- production durable-capability authority population;
- durable provider bootstrap or credential loading;
- capability authorization invocation changes;
- requester/rendezvous provider population;
- peer lookup or peer re-selection;
- bind-address or worker-limit source changes;
- environment mutation;
- listener/bind/readiness/network activation;
- additional endpoint close or idle drain;
- worker spawn/cancel redesign;
- callback aggregation redesign;
- startup/exit error policy;
- registry, database, schema or control-plane mutation;
- service/systemd/package/credential/certificate/private-key/trust/RBAC mutation;
- restart, recovery activation or deployment;
- repository visibility/configuration mutation;
- merge, PR close, ready-for-review conversion, branch deletion or history rewrite.

## 13. Validation rule

The C03e-LL documentation-only selection is valid only when tied to one exact final LL head.

Required validation authority:

- exact LL branch head after all selection/corrective contract commits;
- exact LK -> final LL comparison with merge base equal to LK head;
- exactly one changed path: this contract;
- zero Rust/source/runtime/manifest/lockfile/workflow/Android/packaging/host changes;
- `PRW Rust Validation` terminal success on the exact final LL head;
- path-filtered workflow skips recorded as `SKIPPED`, not PASS;
- no Android PASS claimed unless an exact-head Android workflow actually runs and succeeds.

Earlier LL heads and their CI become non-authoritative if a later corrective contract commit changes the branch head.

## 14. Immutable evidence rule

Closure may be claimed only after:

1. exact-final-head validation is complete;
2. a frozen markdown audit is published under the canonical Private Remote Workspace Drive parent;
3. exact-title pre-upload search confirms no duplicate;
4. raw Drive readback confirms exact frozen byte count and SHA-256;
5. exact-title post-upload search returns exactly one canonical artifact;
6. the LL branch is re-read after publication and remains at the exact validated head;
7. the LL PR is re-read after publication and remains draft/open/unmerged at the exact validated head.

## 15. STOP rule

After C03e-LL closes, **STOP**.

The next checkpoint may only materialize the one-file parent-module endpoint-lifecycle durable caller adaptation selected here.

After that source materialization closes, perform a fresh exact-head audit before selecting any boundary that projects/widens the requester-aware terminal callback outside `remote_session_capability_runtime`, any production durable-authority population, `linux_bootstrap.rs` caller migration, executable aggregate assembly, startup error policy, runtime activation, merge, deployment, or destructive cleanup.
