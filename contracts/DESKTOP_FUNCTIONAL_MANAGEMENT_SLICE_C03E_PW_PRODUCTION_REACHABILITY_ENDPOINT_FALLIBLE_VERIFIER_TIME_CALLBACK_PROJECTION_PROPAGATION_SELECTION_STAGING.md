# Desktop Functional Management Slice C03e-PW — Production reachability endpoint fallible verifier-time callback projection propagation selection

Status: STAGING — SELECTION

## 1. Purpose

C03e-PW selects the next narrow compatibility boundary above the closed C03e-PV source materialization.

C03e-PV materialized one bounded crate-visible fallible verifier-time completion projection and one raw endpoint adapter on `RemoteSessionEndpointLifecycleRuntime`. A fresh exact-source audit now proves that the next production caller cannot skip directly from `linux_bootstrap.rs` to that raw endpoint owner: the production process owns `ProductionReachabilityEndpointLifecycleRuntime`, which retains a distinct `ProductionReachabilityEtcdOwnerCustody` beside the lower endpoint and releases that custody only after the delegated lifecycle drive returns.

C03e-PW therefore selects only propagation of the already-materialized C03e-PV projection through that existing production reachability endpoint wrapper.

This checkpoint is documentation-only. It does not materialize Rust source, install the concrete verifier-time provider, compose requester/rendezvous production request construction, migrate a Linux higher-owner caller, activate an executable path, merge, or deploy.

## 2. Exact predecessor

Predecessor checkpoint: C03e-PV.

Predecessor branch:

`phase-152-c03e-pv-production-requester-rendezvous-expected-device-admission-fallible-verifier-time-endpoint-lifecycle-callback-projection-source-materialization`

Exact predecessor head:

`a45d705e300f26eb2bac98b44b36eae943038777`

Exact predecessor tree:

`fce0f05d423da053159e298616fd732248126d0e`

Exact predecessor endpoint source blob:

`3b5537d7c176cc8ca1d34f65d47a71c7297effac`

Exact predecessor parent capability-runtime blob:

`b79f6dfb33d9c29a229a581061cb3e30dd5a8d77`

Exact production reachability endpoint wrapper blob:

`4030cac84cad1780cc37410d344ba642cb4ac6e4`

Exact Linux higher-owner blob:

`5984af5e0f3db211e484d4a6dd4935a3ffc8a630`

Exact PRWA verifier-source blob:

`e34c3d452b9fd5c9787abbf1f36106e3b97e3b0b`

Canonical `main` remains outside this lineage and was freshly verified unchanged at:

`7c993fa93977a0bb84e0d030874eee7fd0cae77f`

with tree:

`63b8e59ca53797fdea6b95432e16f35eaf473604`

## 3. Exact C03e-PV source state

C03e-PV materialized the bounded endpoint-owned enum:

`RemoteSessionFallibleVerifierTimeEndpointLifecycleCompletionProjection`

with visibility:

`pub(crate)`

and exact variants:

- `Cancelled`
- `VerifierTimeFailure`
- `TransactionFailure`
- `AbnormalTaskCompletion`

C03e-PV also materialized:

`RemoteSessionEndpointLifecycleRuntime::drive_repeated_real_fallible_verifier_time_remote_admission_endpoint_lifecycle_with_completion_projection`

with visibility:

`pub(crate)`

The adapter invokes the existing C03e-PT fallible endpoint lifecycle exactly once, projects only completion, forwards authenticated owner-derived `DeviceId` unchanged, preserves every non-completion input/callback, and returns the existing `RemoteSessionPersistentCollectionConfigError` unchanged.

The parent `remote_session_capability_runtime` re-exports only the bounded completion projection at `pub(crate)` visibility. Raw fallible worker/error payload types remain private.

## 4. Fresh production-wrapper audit

The real production reachability endpoint owner is:

`ProductionReachabilityEndpointLifecycleRuntime`

in:

`crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs`

It owns exactly:

- one `RemoteSessionEndpointLifecycleRuntime`;
- one `ProductionReachabilityEtcdOwnerCustody`.

The wrapper already centralizes the custody law through:

`drive_with_retained_custody(endpoint, custody, drive)`

which:

1. invokes the lower endpoint drive;
2. captures its exact output;
3. drops the retained production custody only after that drive returns;
4. returns the exact lower output unchanged.

C03e-PW must preserve that law.

## 5. Why direct Linux caller migration is not selected

`crates/prw-agent/src/linux_bootstrap.rs` does not drive the raw endpoint owner in the production-reachability path. It binds through production reachability custody and later receives a `ProductionReachabilityEndpointLifecycleRuntime`.

The existing Linux production requester/rendezvous operation currently calls the production wrapper sibling materialized by C03e-LR, not a raw endpoint method.

Directly migrating Linux to the C03e-PV raw endpoint adapter would bypass or reconstruct the distinct retained `ProductionReachabilityEtcdOwnerCustody` lifecycle role.

C03e-PW rejects that shortcut.

Higher-owner caller migration remains separately gated after the production wrapper exposes the corresponding fallible projection seam.

## 6. Historical architectural precedent

The closed C03e-LP/LQ/LR lineage establishes the applicable layering rule.

C03e-LP materialized a bounded raw endpoint completion projection.

C03e-LQ then selected propagation through `ProductionReachabilityEndpointLifecycleRuntime` instead of directly migrating the higher owner.

C03e-LR materialized exactly one production-wrapper sibling in one source path using `drive_with_retained_custody(...)` and delegated exactly once to the raw C03e-LP adapter.

Only later checkpoints crossed into Linux/higher-owner operation composition.

C03e-PW applies the same layering discipline to the distinct C03e-PV fallible verifier-time projection.

## 7. Selected immediate future source ceiling

The immediate future source-materialization checkpoint is ceilinged to exactly one Rust path:

`crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs`

No second source path is selected.

The future materialization may add only:

1. the existing crate-visible C03e-PV projection type to the current import list; and
2. one dormant crate-visible sibling method on `ProductionReachabilityEndpointLifecycleRuntime`.

If canonical source validation requires any second path, parent-module mutation, endpoint-child mutation, executor mutation, authenticated-session mutation, `prw-session` mutation, Linux mutation, or broader visibility, the future source checkpoint must STOP and reselect rather than silently widening the ceiling.

## 8. Selected production-wrapper sibling

Selected future method:

`ProductionReachabilityEndpointLifecycleRuntime::drive_repeated_real_fallible_verifier_time_remote_admission_endpoint_lifecycle_with_completion_projection`

Selected visibility:

`pub(crate)`

The future sibling must consume the production wrapper exactly once.

It must destructure only:

- `endpoint`;
- `owner_custody`.

It must invoke the existing `drive_with_retained_custody(...)` exactly once.

Inside that retained-custody delegation, it must invoke the existing C03e-PV raw endpoint adapter exactly once.

No alternate lower lifecycle is selected.

## 9. Exact selected input shape

The future production-wrapper sibling must preserve the exact C03e-PV non-custody input family:

- `max_active_workers: NonZeroUsize`;
- `authority: &SharedCurrentCapabilityAuthority<P>`;
- `session_authentication: &mut SessionAuthenticationService`;
- `expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`;
- `admission_timing: F`;
- `on_completion: C`;
- `on_rejection: R`;
- `on_admission_failure: E`.

Selected generic bounds:

`P: PolicyEvaluator + Send + Sync + 'static`

`D: CapabilityDispatcher + Send + 'static`

`T: FnMut() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError> + Send + 'static`

`F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming`

`C: FnMut(DeviceId, RemoteSessionFallibleVerifierTimeEndpointLifecycleCompletionProjection)`

`R: FnMut(RemoteSessionExpectedDeviceAdmissionRejection<D, T>)`

`E: FnMut(RemoteSessionRepeatedAdmissionFailure)`

No new aggregate parameter type is selected.

## 10. Projection ownership law

C03e-PW selects no new completion mapping.

C03e-PV remains sole owner of the four-family projection:

- worker cancellation -> `Cancelled`;
- verifier-time source failure -> `VerifierTimeFailure`;
- capability transaction failure -> `TransactionFailure`;
- abnormal task completion -> `AbnormalTaskCompletion`.

The production reachability wrapper must forward the projected callback unchanged.

It must not inspect, reconstruct, translate, merge, split, suppress, retry, cache, or relabel any completion.

## 11. Authenticated identity preservation

The future production wrapper must not create or substitute any logical identity.

Authenticated owner-derived `DeviceId` supplied by the C03e-PV projection remains unchanged through the wrapper.

The project identity invariant remains:

expected pre-auth `DeviceId` is scheduling intent; authenticated owner-derived `DeviceId` is post-auth worker authority.

The wrapper must not replace that identity with:

- requester `DeviceId`;
- expected pre-auth `DeviceId`;
- request ID;
- endpoint identity;
- scheduling correlation;
- reconstructed identity.

## 12. Production reachability custody law

`ProductionReachabilityEtcdOwnerCustody` remains a distinct lifecycle-custody lane.

The future sibling must retain that exact owner for the complete lower C03e-PV endpoint drive and release it only after the lower drive returns.

It may not:

- extract internal provider/store state;
- clone or recreate custody;
- release custody before the endpoint drive returns;
- replace custody with `Arc<ProductionDurableCapabilityAuthority>`;
- bootstrap a new provider;
- perform durable owner operations;
- synthesize a degraded custody state.

## 13. Provider preservation

The verifier-time provider remains generic and caller-supplied through each expected request.

C03e-PW does not select concrete provider installation.

The existing production-capable source:

`prw_session::prwa_verifier_source::current_prwa_verifier_unix_seconds`

was freshly verified to have exact shape:

`fn() -> Result<u64, PrwaVerifierSourceError>`

but it remains uninstalled by this checkpoint and by the immediate one-file production-wrapper propagation successor.

The future wrapper must not sample, preflight, cache, default, clamp, retry, translate, or otherwise invoke the provider itself.

## 14. Requester/rendezvous and durable capability separation

C03e-PW does not yet combine the C03e-PV fallible endpoint path with the separate requester-aware production-durable capability lifecycle.

The existing C03e-LR sibling remains unchanged:

`ProductionReachabilityEndpointLifecycleRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability_projection`

Its distinct inputs remain unchanged, including:

- `Arc<ProductionDurableCapabilityAuthority>`;
- requester/rendezvous policy source;
- `SharedRequesterRendezvousAuthority`;
- the requester-aware durable completion projection.

C03e-PW selects no substitution between the C03e-LR durable lane and the C03e-PV fallible verifier-time lane.

Their later composition, if selected, remains a separate checkpoint.

## 15. Rejection and admission-failure preservation

The future wrapper forwards unchanged:

`RemoteSessionExpectedDeviceAdmissionRejection<D, T>`

and:

`RemoteSessionRepeatedAdmissionFailure`.

It adds no rejection projection, admission-failure projection, recovery, retry, fallback, or error envelope.

## 16. Teardown/runtime preservation

C03e-PW selects no teardown or runtime mutation.

The future wrapper adds no:

- endpoint close;
- `wait_idle()`;
- shutdown source;
- runtime construction;
- generic `block_on`;
- runtime handle;
- nested runtime drive;
- worker spawn/join behavior;
- error translation.

C03e-PP/PT remain authoritative below PV for endpoint close -> idle drain -> exact result preservation.

`drive_with_retained_custody(...)` remains the production-wrapper-only custody envelope around that existing lower behavior.

## 17. Historical-path preservation

The future source checkpoint must leave unchanged:

- `ProductionReachabilityEndpointLifecycleRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle`;
- `ProductionReachabilityEndpointLifecycleRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_capability_projection`;
- C03e-PV raw endpoint projection adapter;
- C03e-LP requester-aware durable projection adapter;
- all executor lifecycle bodies;
- authenticated-session worker bodies;
- production reachability startup/bind behavior;
- `drive_with_retained_custody(...)` itself.

## 18. Higher-owner boundary deferred

C03e-PW does not select mutation of:

`crates/prw-agent/src/linux_bootstrap.rs`

It does not select:

- a Linux production operation factory;
- callback logging/counter/process-exit policy;
- production input-owner migration;
- concrete provider installation;
- expected-device request construction;
- requester/rendezvous producer wiring;
- durable capability/requester-aware recomposition;
- executable companion assembly;
- public `run()` activation.

Those remain separately gated.

## 19. Source-layout stop conditions

The immediate future source checkpoint must STOP and reselect if any of the following becomes necessary:

- more than `production_reachability_endpoint_lifecycle.rs`;
- raw endpoint source mutation;
- parent capability-runtime mutation;
- executor/authenticated-session mutation;
- `prw-session` mutation;
- Linux bootstrap mutation;
- a new callback mapping;
- a new custody abstraction;
- a new provider invocation;
- broader than `pub(crate)` visibility;
- a second teardown path;
- requester/durable lifecycle fusion.

No silent widening is permitted.

## 20. Exact C03e-PW branch target

Selected branch:

`phase-152-c03e-pw-production-reachability-endpoint-fallible-verifier-time-callback-projection-propagation-selection`

The branch must remain based directly on exact C03e-PV head:

`a45d705e300f26eb2bac98b44b36eae943038777`

C03e-PW itself changes only this contract file.

## 21. Validation law

C03e-PW is documentation-only.

Only CI attached to the exact final C03e-PW head may be claimed.

Rust validation must reach terminal SUCCESS before closure.

Android may be reported only if an exact-final-head Android run actually triggers and reaches terminal SUCCESS.

Path-filtered `SKIPPED` workflows are not PASS.

No predecessor PASS may be inherited as exact-head evidence.

## 22. Immutable evidence law

After exact-final-head validation, publish one immutable raw Markdown audit under canonical Drive parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

with exact-title precheck zero, frozen local byte count/SHA-256, one upload, raw Drive readback, exact byte/hash equality, and exact-title postcheck uniqueness.

Do not overwrite or mutate an existing evidence object.

## 23. Frozen exclusions

C03e-PW does not modify, select, or authorize:

- Rust/source materialization in PW itself;
- more than one source path for the immediate successor;
- concrete verifier-time provider installation;
- `prw-session` mutation;
- requester/rendezvous production producer migration;
- production expected-device request construction;
- production durable capability/requester-aware fusion;
- higher-owner caller migration;
- `linux_bootstrap.rs` mutation;
- executable callback/logging/counter/process-exit policy;
- listener/startup/readiness activation;
- process-signal integration;
- `main.rs` mutation;
- public API widening;
- generic runtime exposure;
- service/systemd/host credential mutation;
- DB/schema/control-plane mutation;
- authentication/authorization redesign;
- retry/reconnect/rebootstrap;
- merge;
- deployment;
- restart/recovery;
- ready-for-review conversion;
- PR close;
- branch deletion;
- force update;
- reset/rebase/squash/history rewrite;
- destructive cleanup.

The previously disclosed accidental `tmp-never-use` branch remains outside this checkpoint and must remain untouched.

## 24. Gate / closure

Gate:

`C03E_PW_PRODUCTION_REACHABILITY_ENDPOINT_FALLIBLE_VERIFIER_TIME_CALLBACK_PROJECTION_PROPAGATION_SELECTED`

Expected administrative closure after exact-head validation and immutable evidence publication:

`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

After C03e-PW closure: **STOP**.

Do not create C03e-PX in the same closure.
