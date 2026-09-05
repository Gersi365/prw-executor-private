# Desktop Functional Management Slice C03e-LQ — Production Reachability Endpoint Durable Callback Projection Propagation Selection

Status: `STAGING_SELECTION`
Date: `2026-09-05`
Repository: `Gersi365/prw-executor-private`

## 1. Gate

Selected gate after exact-head validation and immutable evidence publication:

`C03E_LQ_PRODUCTION_REACHABILITY_ENDPOINT_DURABLE_CALLBACK_PROJECTION_PROPAGATION_SELECTED`

Selected closure:

`CLOSED_PRODUCTION_REACHABILITY_ENDPOINT_DURABLE_CALLBACK_PROJECTION_PROPAGATION_SELECTION`

C03e-LQ is documentation-only. It selects the next narrow source boundary after evidence-closed C03e-LP. It does not materialize Rust/source behavior.

The selected boundary propagates the already-materialized C03e-LP requester-aware durable completion projection through the existing production reachability endpoint wrapper while preserving the wrapper's separate durable reachability-owner custody for the full lower endpoint lifecycle.

## 2. Exact predecessor authority

C03e-LQ is rooted only at evidence-closed C03e-LP:

- branch: `phase-152-c03e-lp-production-durable-capability-endpoint-lifecycle-callback-projection-source-materialization`
- exact predecessor head / merge base: `ecc3a63198860ffac5d9eed00531a4ff635f947f`
- exact predecessor tree: `db67aab4d90851db71716d1621af48dbb917db50`
- exact C03e-LP endpoint child blob: `b1afdeba4b0bc59399ff4e4dbf480479f4fb2cfe`
- exact C03e-LP parent runtime blob: `de66532f18ebbca30ac6bd6b9da4983ded4b8bbe`
- exact production reachability endpoint wrapper blob: `4be58d66dddccc03e1f0d932b2805aba524ead0c`
- exact higher-owner durable custody blob: `9ab6023eee49e5987f34409d3f37e63d753d73bd`
- exact `linux_bootstrap.rs` blob: `f2a87c45bd8d96bf1555b65210531c94c722eb2f`

C03e-LP remains draft/open/unmerged and evidence-closed. No merge, deployment, runtime activation, repository configuration mutation, branch deletion, or history rewrite is implied by using it as the exact predecessor.

## 3. Fresh source finding

### 3.1 C03e-LP raw endpoint projection exists and is already crate-visible

Exact C03e-LP source exposes the bounded crate-visible enum:

`RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection`

with exactly four variants:

- `Cancelled`
- `IngressFailure`
- `RequesterResponseFailure`
- `AbnormalTaskCompletion`

The C03e-LP parent module re-exports that enum only as `pub(crate)`.

The raw endpoint owner additionally exposes:

`RemoteSessionEndpointLifecycleRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability_projection(...)`

with visibility exactly `pub(crate)`.

That adapter consumes the raw endpoint owner, forwards all non-completion inputs unchanged, invokes the unchanged C03e-LM durable endpoint method exactly once, forwards authenticated `DeviceId` unchanged, and projects only the requester-aware completion callback into the four bounded families above.

### 3.2 The production operation does not own the raw endpoint type

Exact predecessor `linux_bootstrap.rs` creates and drives:

`ProductionReachabilityEndpointLifecycleRuntime`

through the production reachability runtime-custody bind path.

The production operation therefore does not receive a raw `RemoteSessionEndpointLifecycleRuntime` at the drive callsite. It receives the production reachability wrapper that additionally retains one `ProductionReachabilityEtcdOwnerCustody` beside the raw endpoint.

### 3.3 The production wrapper currently exposes only the legacy drive

Exact predecessor:

`crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs`

contains:

`ProductionReachabilityEndpointLifecycleRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle(...)`

That method:

- consumes the production wrapper exactly once;
- retains `ProductionReachabilityEtcdOwnerCustody` lexically across the complete delegated lower drive;
- uses the existing `drive_with_retained_custody(...)` helper;
- delegates to the legacy raw endpoint lifecycle;
- releases durable reachability-owner custody only after the lower lifecycle returns.

It does not yet expose a sibling that delegates to the C03e-LP durable requester-aware projection adapter.

### 3.4 Why higher-owner caller migration is not yet the immediate boundary

Exact predecessor `production_durable_capability_higher_owner_custody.rs` still retains the production durable capability authority only as an outer `Arc` lifetime witness and delegates to the existing requester/rendezvous production operation before dropping that `Arc`.

However, replacing that operation call directly would skip the existing production reachability endpoint wrapper's separate durable reachability-owner custody law. The production operation's endpoint value is the wrapper, not the raw LP endpoint owner.

C03e-LQ therefore selects one intermediate propagation seam first. Higher-owner caller migration remains separately gated.

## 4. Selected immediate source successor

The immediate future source materialization may mutate exactly one Rust path:

`crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs`

Exact predecessor blob:

`4be58d66dddccc03e1f0d932b2805aba524ead0c`

No second source path is selected.

## 5. Selected production-wrapper sibling

The future source checkpoint may add exactly one new dormant method on:

`ProductionReachabilityEndpointLifecycleRuntime`

with name exactly:

`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability_projection`

and visibility exactly:

`pub(crate)`

The method must consume `self` exactly once and preserve the existing production wrapper's ownership split:

- raw endpoint owner: `RemoteSessionEndpointLifecycleRuntime`;
- retained durable production reachability owner: `ProductionReachabilityEtcdOwnerCustody`.

The method must use the existing `drive_with_retained_custody(...)` helper exactly once so `ProductionReachabilityEtcdOwnerCustody` remains alive until the C03e-LP raw endpoint adapter returns.

The method must invoke the C03e-LP raw endpoint projection adapter exactly once and must not invoke the legacy raw endpoint drive.

## 6. Exact selected input and callback surface

The new wrapper sibling must mirror the already-materialized C03e-LP durable projection inputs without creating a new aggregate.

It must accept and forward unchanged:

- `max_active_workers: NonZeroUsize`;
- `authority: &SharedCurrentCapabilityAuthority<P>`;
- `capability_authority: Arc<ProductionDurableCapabilityAuthority>`;
- `policy_source: Arc<PS>`;
- `requester_rendezvous_authority: &SharedRequesterRendezvousAuthority`;
- `session_authentication: &mut SessionAuthenticationService`;
- `expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`;
- `admission_timing: F`;
- `on_completion: C`;
- `on_rejection: R`;
- `on_admission_failure: E`.

The selected generic bounds must preserve the C03e-LP shapes, including:

- `P: PolicyEvaluator + Send + Sync + 'static`;
- `D: CapabilityDispatcher + Send + 'static`;
- `T: FnMut() -> u64 + Send + 'static`;
- `PS: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static`;
- `F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming`;
- `C: FnMut(DeviceId, RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection)`;
- `R: FnMut(RemoteSessionExpectedDeviceAdmissionRejectionReason, RemoteSessionExpectedDeviceAdmissionRequest<D, T>)`;
- `E: FnMut(DeviceId, RemoteSessionRealAdmissionError)`.

The wrapper must return the existing:

`Result<(), RemoteSessionPersistentCollectionConfigError>`

unchanged.

No new error enum, completion enum, callback aggregate, provider type, policy type, owner type, or public API is selected.

## 7. Projection semantics remain solely owned by C03e-LP

C03e-LQ selects no new callback mapping.

The production wrapper must forward the C03e-LP completion projection unchanged. The exact already-materialized mapping remains authoritative:

- requester worker `Cancelled` -> `Cancelled`;
- requester lifecycle `Failed(Ingress(_))` -> `IngressFailure`;
- requester lifecycle `Failed(RequesterResponse(_))` -> `RequesterResponseFailure`;
- join `AbnormalTaskCompletion` -> `AbnormalTaskCompletion`.

The authenticated `DeviceId` remains unchanged beside that projection.

Requester-private stop/error payloads remain private. The production wrapper must not inspect, recreate, stringify, translate, re-export, or widen those lower private types.

## 8. Two distinct durable custody lanes must remain distinct

C03e-LQ explicitly preserves two different ownership roles:

1. `ProductionReachabilityEtcdOwnerCustody` retained by `ProductionReachabilityEndpointLifecycleRuntime`;
2. `Arc<ProductionDurableCapabilityAuthority>` forwarded by value into the C03e-LP durable endpoint projection adapter.

The future wrapper must not merge these types, derive one from the other, expose either through a generic getter, clone the reachability owner custody, or use the reachability owner as capability authorization authority.

`ProductionReachabilityEtcdOwnerCustody` remains a lifecycle-retention obligation around the lower endpoint drive. `ProductionDurableCapabilityAuthority` remains the explicit capability-authorization authority input consumed by the lower requester-aware durable lifecycle.

## 9. Existing lifecycle and shutdown law remains authoritative

The new wrapper sibling must not independently:

- destructure or alter the raw endpoint beyond passing it to `drive_with_retained_custody(...)`;
- convert the supervisor shutdown signal;
- close the remote transport;
- wait for idle;
- invoke the executor lifecycle directly;
- duplicate persistent-worker collection behavior;
- retry endpoint work;
- rebind an endpoint;
- bootstrap or recover reachability authority;
- publish readiness.

Those behaviors remain below the existing raw endpoint/executor boundaries.

The existing legacy `ProductionReachabilityEndpointLifecycleRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle(...)` must remain unchanged.

## 10. Permitted one-file mechanical changes

The immediate source materialization may add only what is mechanically required in the selected file for the sibling above:

- `Arc` import;
- `RequesterRendezvousStartPolicySource` import;
- `ProductionDurableCapabilityAuthority` import;
- `RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection` import;
- `RemoteSessionExpectedDeviceAdmissionRejectionReason` import;
- `RemoteSessionRealAdmissionError` import;
- `SharedRequesterRendezvousAuthority` import;
- the selected method itself;
- doc comments;
- exact signature-only test scaffolding in the same file if useful;
- formatting or narrow lint attributes required for exact-source CI while the seam remains dormant.

No behavioral helper duplication is selected. The existing `drive_with_retained_custody(...)` helper must be reused rather than replaced.

## 11. Frozen source and behavior outside the selection

The immediate future source checkpoint must leave unchanged:

- `crates/prw-agent/src/remote_session_capability_runtime.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`;
- the C03e-LM durable raw endpoint method;
- the C03e-LP raw endpoint projection adapter;
- the C03e-LP projection enum and parent re-export;
- requester retained-custody stop/error visibility;
- `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`;
- `crates/prw-agent/src/linux_bootstrap.rs`;
- `crates/prw-agent/src/production_reachability_runtime_custody.rs`;
- `crates/prw-agent/src/lib.rs`;
- `crates/prw-agent/src/main.rs`;
- manifests, lockfile, workflows, Android source and packaging.

If a second source path becomes necessary, STOP and open a separately selected boundary rather than widening this checkpoint.

## 12. Explicit non-selection

C03e-LQ does not select or authorize:

- Rust/source materialization in LQ itself;
- higher-owner caller migration;
- mutation of `production_durable_capability_higher_owner_custody.rs`;
- mutation of `linux_bootstrap.rs`;
- legacy callback aggregate conversion;
- executable callback/logging/counter policy;
- durable capability-authority bootstrap or population;
- requester/rendezvous runtime-owner population changes;
- production peer population changes;
- requester-private type visibility widening;
- new completion mapping or error translation;
- candidate/reachability continuation changes;
- target dialing, retry, reconnect or peer reuse;
- endpoint/listener/readiness/runtime activation;
- merge, ready-for-review conversion, deploy, restart/recovery;
- repository configuration mutation;
- PR close, branch deletion, force update or history rewrite;
- destructive cleanup.

## 13. Validation requirements

C03e-LQ closure authority is only its exact final documentation-only head.

Before closure:

1. verify exact predecessor remains C03e-LP head `ecc3a63198860ffac5d9eed00531a4ff635f947f`;
2. verify LQ is ahead only by documentation commit(s), behind by zero;
3. verify exactly one changed path: this contract;
4. verify zero Rust/source/runtime/manifest/lockfile/workflow/Android/packaging/executable changes;
5. require exact-final-head `PRW Rust Validation` success;
6. classify path-inapplicable workflows as `SKIPPED`, never PASS;
7. claim Android only if an exact-final-head Android workflow actually runs and succeeds;
8. re-read exact branch/PR/source authority before immutable evidence publication;
9. freeze audit bytes and SHA-256 before Drive upload;
10. require exact-title pre-upload uniqueness, raw Drive byte/hash readback, and exact-title post-upload uniqueness;
11. keep the PR draft/open/unmerged.

## 14. Gate and successor boundary

After exact-final-head validation and immutable evidence publication, record:

`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

Gate:

`C03E_LQ_PRODUCTION_REACHABILITY_ENDPOINT_DURABLE_CALLBACK_PROJECTION_PROPAGATION_SELECTED`

Closure:

`CLOSED_PRODUCTION_REACHABILITY_ENDPOINT_DURABLE_CALLBACK_PROJECTION_PROPAGATION_SELECTION`

After C03e-LQ closure: STOP.

The immediate later source checkpoint may materialize only the one-file production-wrapper sibling selected above. Higher-owner operation migration remains a later separately selected boundary and must not be combined with that source checkpoint.