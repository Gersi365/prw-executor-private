# Desktop Functional Management Slice C03e-PU — Production requester/rendezvous expected-device admission fallible verifier-time endpoint lifecycle callback projection selection

Status: STAGING — SELECTION

## 1. Purpose

C03e-PU selects the next narrow compatibility boundary above the closed C03e-PT source materialization.

C03e-PT materialized the dormant fallible verifier-time endpoint lifecycle wrapper inside `remote_session_endpoint_lifecycle_runtime`, while preserving the raw fallible persistent-worker completion carrier and its nested worker/error payloads behind the existing private-module boundaries. A fresh exact-source audit now proves that a higher owner such as `linux_bootstrap.rs` cannot safely consume that raw callback family directly without widening private executor/authenticated-session implementation types.

C03e-PU therefore selects one bounded crate-visible endpoint completion projection adapter. This checkpoint is documentation-only. It does not materialize source, install the concrete verifier-time provider, migrate a higher-owner caller, activate runtime behavior, or modify executable startup.

## 2. Exact predecessor

Predecessor checkpoint: C03e-PT.

Predecessor branch:

`phase-152-c03e-pt-production-requester-rendezvous-expected-device-admission-fallible-verifier-time-endpoint-lifecycle-runtime-composition-source-materialization`

Exact predecessor head:

`e30edafa89b8b74ab824f530734ceb07de65314f`

Exact predecessor tree:

`e1ff3cf03f5648818e05816af91d3882ee25ac0a`

Exact endpoint source blob:

`742a8caca5fb4be6c15d5c777c1f56e52b8d3e78`

Exact executor source blob:

`1539b6b9a08bf18883d7a16022f15f7c240eaf08`

Exact parent capability-runtime blob:

`de66532f18ebbca30ac6bd6b9da4983ded4b8bbe`

Canonical `main` remains outside this lineage and was freshly verified unchanged at:

`7c993fa93977a0bb84e0d030874eee7fd0cae77f`

with tree:

`63b8e59ca53797fdea6b95432e16f35eaf473604`

## 3. Exact C03e-PT source state

C03e-PT materialized:

`RemoteSessionEndpointLifecycleRuntime::drive_repeated_real_fallible_verifier_time_remote_admission_endpoint_lifecycle`

with child-module visibility:

`pub(super)`

The wrapper consumes the retained endpoint owner once, supplies the existing endpoint-owner shutdown future, forwards the existing transport/authority/session-authentication/expected-request/timing/callback inputs, and delegates exactly once to the C03e-PP executor lifecycle.

The executor lifecycle is reachable from the endpoint sibling only through the C03e-PS-selected ancestor-restricted visibility:

`pub(in crate::remote_session_capability_runtime)`

The raw completion carrier remains:

`RemoteSessionFallibleVerifierTimeRegisteredWorkerCompletion`

at executor-root `pub(super)` visibility. No parent-module re-export exists for that raw carrier.

This private layout is intentional and remains frozen by C03e-PU.

## 4. Fresh higher-owner access audit

`crates/prw-agent/src/remote_session_capability_runtime.rs` publicly re-exports the endpoint owner type, but it does not re-export the raw fallible completion carrier.

`crates/prw-agent/src/linux_bootstrap.rs` currently imports the historical infallible endpoint lifecycle types and callbacks. It does not import the fallible completion carrier, the fallible authenticated worker stop, the fallible loop error, or `PrwaVerifierSourceError` for endpoint completion handling.

The PT wrapper itself is not selected for direct crate-root exposure. Directly widening the wrapper together with its raw callback carrier would leak executor/authenticated-worker implementation representation into the higher-owner surface.

C03e-PU rejects that widening.

## 5. Existing architectural precedent

The closed C03e-LN/LO/LP lineage established the applicable pattern for requester-aware endpoint completion:

1. keep the raw worker/lifecycle result private;
2. define one bounded endpoint-owned completion projection enum;
3. expose one additive endpoint adapter that calls the existing private lifecycle exactly once and projects only completion;
4. preserve authenticated `DeviceId` unchanged;
5. re-export only the bounded projection from the parent module at crate visibility;
6. defer higher-owner migration and executable callback policy to later checkpoints.

C03e-PU selects the same layering discipline for the fallible verifier-time endpoint lifecycle. It does not reuse the requester-aware projection type because the terminal taxonomy is different.

## 6. Exact raw fallible terminal taxonomy

The exact C03e-PT source lineage contains the following nested terminal chain.

### 6.1 Persistent completion carrier

`RemoteSessionFallibleVerifierTimeRegisteredWorkerCompletion`

owns:

- authenticated logical `DeviceId`;
- `Result<AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

### 6.2 Fallible worker stop

`AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop` has exactly:

- `Cancelled`;
- `Failed(AuthenticatedRemoteSessionFallibleCapabilityRequestLoopError)`.

### 6.3 Fallible request-loop error

`AuthenticatedRemoteSessionFallibleCapabilityRequestLoopError` has exactly:

- `VerifierTime(PrwaVerifierSourceError)`;
- `Transaction(AuthenticatedRemoteSessionCapabilityTransactionError)`.

### 6.4 Join error

The existing bounded join error has:

- `RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`.

C03e-PU selects a projection that preserves these four semantically distinct terminal families while hiding all nested payload representation.

## 7. Selected bounded completion projection

The immediate future source checkpoint may add exactly one endpoint-owned crate-visible enum named:

`RemoteSessionFallibleVerifierTimeEndpointLifecycleCompletionProjection`

with exactly four variants:

- `Cancelled`
- `VerifierTimeFailure`
- `TransactionFailure`
- `AbnormalTaskCompletion`

No payload fields are selected on these variants.

### 7.1 Projection law

The future adapter must map exactly:

- `Ok(AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop::Cancelled)` -> `Cancelled`;
- `Ok(AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop::Failed(AuthenticatedRemoteSessionFallibleCapabilityRequestLoopError::VerifierTime(_)))` -> `VerifierTimeFailure`;
- `Ok(AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop::Failed(AuthenticatedRemoteSessionFallibleCapabilityRequestLoopError::Transaction(_)))` -> `TransactionFailure`;
- `Err(RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion)` -> `AbnormalTaskCompletion`.

No other mapping, fallback, default, suppression, retry, fabricated success, or catch-all terminal family is selected.

The exact nested verifier-time and transaction error payloads remain private to their existing owners.

## 8. Authenticated identity preservation

The future projection adapter must recover the raw persistent completion through its existing ownership-preserving decomposition and forward the authenticated logical `DeviceId` unchanged alongside the bounded projection.

Selected callback shape:

`FnMut(DeviceId, RemoteSessionFallibleVerifierTimeEndpointLifecycleCompletionProjection)`

The `DeviceId` is the authenticated owner-derived worker identity already retained by the persistent completion carrier. It must not be replaced by a pre-auth expected `DeviceId`, requester identity, request ID, scheduling identifier, endpoint identifier, or reconstructed value.

C03e-PU does not alter the project identity invariant:

expected pre-auth `DeviceId` is scheduling intent; authenticated owner-derived `DeviceId` is post-auth worker authority.

## 9. Selected endpoint adapter

The immediate future source checkpoint may add exactly one additive crate-visible endpoint adapter named:

`RemoteSessionEndpointLifecycleRuntime::drive_repeated_real_fallible_verifier_time_remote_admission_endpoint_lifecycle_with_completion_projection`

Selected visibility:

`pub(crate)`

The adapter must:

1. consume the same endpoint owner exactly once;
2. accept the exact existing C03e-PT lifecycle inputs;
3. call `drive_repeated_real_fallible_verifier_time_remote_admission_endpoint_lifecycle` exactly once;
4. forward every non-completion input and callback unchanged;
5. adapt only the completion callback;
6. recover authenticated `DeviceId` and exact raw completion result from the existing completion carrier;
7. map the raw result through the exact four-variant projection law above;
8. invoke the caller callback with authenticated `DeviceId` unchanged plus the bounded projection;
9. return the exact existing `Result<(), RemoteSessionPersistentCollectionConfigError>` unchanged.

The adapter must not add lifecycle behavior before or after the PT wrapper call.

## 10. Provider contract preservation

The adapter must preserve the existing verifier-time provider generic exactly:

`T: FnMut() -> Result<u64, PrwaVerifierSourceError> + Send + 'static`

It must not sample, install, wrap, cache, clone, default, clamp, retry, translate, suppress, or preflight that provider.

The concrete existing PRWA source:

`prw_session::prwa_verifier_source::current_prwa_verifier_unix_seconds()`

was freshly verified to exist and return:

`Result<u64, PrwaVerifierSourceError>`

but C03e-PU explicitly does not select its installation into production bootstrap. Concrete provider installation remains a later separately gated boundary.

## 11. Teardown and runtime preservation

The PT wrapper and C03e-PP executor lifecycle remain sole authority for endpoint shutdown ordering:

`fallible collection return -> close -> wait_idle -> exact original result`

The future projection adapter must add no:

- direct transport close;
- second `wait_idle()`;
- second shutdown source;
- runtime construction;
- runtime handle;
- generic `block_on`;
- nested runtime drive;
- worker spawn/join behavior;
- admission behavior.

The private current-thread runtime remains unchanged.

## 12. Selected immediate future source layout

The immediate future source materialization is ceilinged to exactly two paths:

1. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`
   - add the exact bounded projection enum;
   - add only the imports required to classify the existing private terminal chain;
   - add the exact additive crate-visible projection adapter;
   - call the PT wrapper exactly once;
   - project only completion.

2. `crates/prw-agent/src/remote_session_capability_runtime.rs`
   - add only one crate-private re-export of `RemoteSessionFallibleVerifierTimeEndpointLifecycleCompletionProjection`;
   - add only a mechanical lint allowance if exact source CI requires it to preserve the selected crate visibility.

No third path is selected.

If fresh future source audit proves that materialization requires any third path, parent API widening beyond this enum re-export, or changes to PT/PP lifecycle bodies, the future source checkpoint must STOP and reselect rather than widen silently.

## 13. Visibility law

Selected projection enum visibility inside the private endpoint child:

`pub(crate)`

Selected projection adapter method visibility:

`pub(crate)`

Selected parent re-export visibility:

`pub(crate) use`

No `pub` projection surface is selected.

The following raw types remain at their current visibility and are not re-exported by C03e-PU:

- `RemoteSessionFallibleVerifierTimeRegisteredWorkerCompletion`;
- `AuthenticatedRemoteSessionFallibleVerifierTimeWorkerStop`;
- `AuthenticatedRemoteSessionFallibleCapabilityRequestLoopError`.

The existing PT wrapper itself is not selected for visibility widening.

## 14. Historical infallible and requester-aware paths

C03e-PU selects no mutation of:

- `RemoteSessionEndpointLifecycleRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle`;
- `RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection`;
- `drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability_projection`;
- requester-aware durable lifecycle behavior;
- historical infallible completion callbacks.

The new projection is additive and fallible-verifier-time-specific.

## 15. Production caller boundary remains deferred

C03e-PU does not select mutation of:

`crates/prw-agent/src/linux_bootstrap.rs`

It does not select:

- replacement of `linux_agent_remote_process_operation`;
- construction of a fallible expected-request producer;
- executable completion logging/policy;
- admission rejection logging changes;
- admission failure logging changes;
- process-lifecycle handoff changes;
- readiness behavior;
- signal handling;
- endpoint startup activation.

A later checkpoint must freshly audit the exact higher-owner caller before any such migration.

## 16. Requester/rendezvous producer boundary remains deferred

C03e-PU does not install or migrate the production requester/rendezvous expected-device producer into the fallible carrier family.

It does not change:

- scheduling grant custody;
- target `SessionId` construction;
- expected-device request construction;
- expected request channel ownership;
- dispatcher custody;
- verifier-time provider placement inside request carriers;
- terminal acknowledgement behavior;
- shutdown suppression behavior.

Those boundaries require separate fresh selection/materialization checkpoints.

## 17. Frozen exclusions

C03e-PU does not select, authorize, or perform:

- Rust/source materialization;
- concrete verifier-time provider installation;
- `prw-session` changes;
- requester/rendezvous producer migration;
- higher-owner caller migration;
- `linux_bootstrap.rs` mutation;
- `main.rs` mutation;
- listener or endpoint startup invocation;
- readiness publication;
- process-signal integration;
- service/systemd changes;
- host credential changes;
- registry/DB/schema/control-plane changes;
- authentication or authorization redesign;
- retry/reconnect/rebootstrap;
- public API widening;
- runtime exposure;
- deployment;
- merge;
- ready-for-review conversion;
- PR closure;
- branch deletion;
- force push;
- reset/rebase/squash/history rewrite;
- destructive cleanup.

The accidental `tmp-never-use` branch is outside this checkpoint and must remain untouched.

## 18. Validation law

C03e-PU is documentation-only.

All validation claims must bind to the exact final C03e-PU head.

- Rust validation may be claimed PASS only if the exact final head reaches terminal `SUCCESS`.
- Android may be claimed PASS only if an exact-final-head Android workflow actually triggers and reaches terminal `SUCCESS`.
- Path-filtered `SKIPPED` workflows must be reported as `SKIPPED`, never PASS.
- An untriggered workflow must not be inherited from C03e-PT.

## 19. Evidence law

Closure requires one immutable Drive audit in canonical parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

The audit publication sequence is:

1. exact-title pre-upload search returns zero;
2. freeze local bytes and SHA-256;
3. upload exactly once as raw `text/markdown`;
4. raw Drive readback;
5. exact byte count and SHA-256 equality;
6. exact-title post-upload search returns exactly one canonical artifact;
7. re-read exact branch/contract/PR state;
8. update only the draft PR body with durable evidence and closure status.

## 20. Gate

Selected gate:

`C03E_PU_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_ENDPOINT_LIFECYCLE_CALLBACK_PROJECTION_SELECTED`

Administrative closure target:

`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

After evidence-verified C03e-PU closure: STOP.

Do not create C03e-PV in the same closure.
