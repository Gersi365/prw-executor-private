# Desktop Functional Management Slice C03e-PO — Production Requester/Rendezvous Expected-Device Admission Fallible Verifier-Time Endpoint Lifecycle Composition Selection

Status: `SELECTION — SOURCE MATERIALIZATION DEFERRED`

Selected later boundary:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_ENDPOINT_LIFECYCLE_COMPOSITION_SOURCE_MATERIALIZATION`

## 1. Exact predecessor

C03e-PO is a documentation-only selection checkpoint whose authoritative predecessor is closed C03e-PN.

PN branch:

`phase-152-c03e-pn-production-requester-rendezvous-expected-device-admission-fallible-verifier-time-repeated-real-admission-persistent-collection-source-materialization`

Exact final PN head:

`23b733e0486891b2833a2453865e06a7ef33173d`

Exact final PN tree:

`abe32a1d93d33266787f2e041b38240e9ae619c8`

Exact final PN executor source blob:

`aa2ee037d698d2c37b41dfd6aefa4b089131e2f3`

Closed PN PR:

`#553 — C03e-PN: materialize fallible verifier-time repeated real-admission persistent collection`

PN remains draft/open/unmerged. Its administrative closure does not merge or activate the source.

Canonical immutable PN audit:

- Drive ID: `1RXYzpGc0LQRSms7idAJzynah2K9hF6Tk`
- filename: `C03E_PN_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_REPEATED_REAL_ADMISSION_PERSISTENT_COLLECTION_SOURCE_MATERIALIZATION_AUDIT_2026-09-11.md`
- canonical parent: `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`
- MIME: `text/markdown`
- bytes: `11216`
- SHA-256: `67ae0124b8427bea9c0ce2f54184f85e8ee5686bf10be7da7470325609dae06e`

## 2. Fresh exact-source observation

The exact final PN executor source contains both sides required to identify the next narrow compatibility seam.

PN now materializes the dormant fallible repeated-real-admission persistent collection sibling:

`RemoteSessionExecutorRuntime::drive_repeated_real_fallible_verifier_time_remote_admission_collection`

Its selected provider/completion law is already fixed in source:

- `T: FnMut() -> Result<u64, PrwaVerifierSourceError> + Send + 'static`;
- `C: FnMut(RemoteSessionFallibleVerifierTimeRegisteredWorkerCompletion)`;
- authenticated owner-derived `DeviceId` remains active-worker authority;
- worker `Cancelled` and `Failed(exact PB error)` remain normal completion values;
- abnormal Tokio completion remains the existing bounded join-error path.

The same exact source still contains only the historical infallible outer endpoint lifecycle:

`RemoteSessionExecutorRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle`

That historical method still composes:

`drive_repeated_real_remote_admission_collection(...)`

and therefore cannot consume the PN fallible collection without one separately gated sibling composition.

## 3. Existing endpoint teardown primitive is already sufficient

The exact PN source already contains the private generic helper:

`finish_remote_endpoint_shutdown<R, C, W>(executor, result, close_endpoint, wait_idle) -> R`

Its behavior is fixed and sufficient:

1. call endpoint close exactly once with the existing fixed diagnostic;
2. drive the supplied idle future on the same private current-thread executor;
3. return the original result unchanged.

Existing close diagnostic remains:

- code: `0`;
- reason: `b"remote endpoint shutdown"`.

No new endpoint teardown helper, close code, close reason, error type, runtime, task or readiness mechanism is required for the fallible verifier-time migration.

## 4. Immediate fixed compatibility gap after PN

The immediate gap is only the outer composition between:

`drive_repeated_real_fallible_verifier_time_remote_admission_collection(...)`

and the already-existing:

`finish_remote_endpoint_shutdown(...)`.

The gap is not:

- verifier-time provider installation;
- requester/rendezvous producer migration;
- expected-device source production wiring;
- endpoint bind/listener acceptance;
- startup/readiness integration;
- signal integration;
- `main.rs` integration;
- service/systemd integration;
- deployment;
- merge.

Those remain later boundaries.

## 5. Historical decomposition confirmation

Historical C03e-AM selected remote endpoint supervisor shutdown lifecycle only after C03e-AL had materialized the repeated real-admission persistent collection.

Historical C03e-AN then materialized the endpoint-lifecycle composition separately.

That historical lifecycle established the ordering which remains authoritative:

- the repeated supervisor runs to full return before endpoint close;
- caller-supplied supervisor shutdown remains the supervisor's stop input while the collection is live;
- endpoint close occurs exactly once only after collection return;
- endpoint `wait_idle()` is driven only after close;
- the same private current-thread executor drives the idle future;
- a pre-drive collection configuration error is preserved unchanged while close and idle teardown still occur;
- no nested `block_on`, second runtime, runtime handle exposure, detached idle-drain task, readiness activation or deployment is introduced.

PO preserves that decomposition exactly for the fallible verifier-time path.

## 6. PO selection decision

C03e-PO selects one later dormant sibling:

`RemoteSessionExecutorRuntime::drive_repeated_real_fallible_verifier_time_remote_admission_endpoint_lifecycle`

The sibling exists only to compose the already-materialized PN collection with the already-materialized endpoint teardown primitive.

The historical infallible:

`drive_repeated_real_remote_admission_endpoint_lifecycle`

remains untouched.

The future sibling should remain no more visible than required for the next separately gated same-crate composition. `pub(super)` is the selected initial ceiling while no production caller is selected.

Any later visibility widening required by a production caller must be justified by that later caller checkpoint, not by PO.

## 7. Selected later source ceiling

A later C03e-PP source materialization may modify exactly one existing Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

No second Rust path is selected.

No contract, Cargo manifest, lockfile, workflow, Android application source, requester/rendezvous module, transport implementation, database, authentication, authorization, endpoint startup, `main.rs`, readiness, packaging, host or deployment path is selected.

Narrow same-file tests/documentation strictly necessary to prove the selected composition are allowed only in the same Rust path.

## 8. Selected conceptual signature

The later sibling should preserve the exact PN collection input surface while changing only the outer lifecycle composition:

```rust
pub(super) fn drive_repeated_real_fallible_verifier_time_remote_admission_endpoint_lifecycle<
    P,
    D,
    T,
    S,
    F,
    C,
    R,
    E,
>(
    &mut self,
    max_active_workers: NonZeroUsize,
    transport_runtime: &AgentRemoteTransportRuntime,
    authority: &SharedCurrentCapabilityAuthority<P>,
    session_authentication: &mut SessionAuthenticationService,
    expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>,
    supervisor_shutdown: S,
    admission_timing: F,
    on_completion: C,
    on_rejection: R,
    on_admission_failure: E,
) -> Result<(), RemoteSessionPersistentCollectionConfigError>
```

Selected semantic bounds remain:

- `P: PolicyEvaluator + Send + Sync + 'static`;
- `D: CapabilityDispatcher + Send + 'static`;
- `T: FnMut() -> Result<u64, PrwaVerifierSourceError> + Send + 'static`;
- `S: Future<Output = ()> + Send`;
- `F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming`;
- `C: FnMut(RemoteSessionFallibleVerifierTimeRegisteredWorkerCompletion)`;
- `R: FnMut(RemoteSessionExpectedDeviceAdmissionRejection<D, T>)`;
- `E: FnMut(RemoteSessionRepeatedAdmissionFailure)`.

No extra `Clone`, `Copy`, `Default`, `Unpin`, unrelated `Sync`, or other widening is selected.

## 9. Selected composition law

The later source sibling must be a narrow composition equivalent in structure to:

```rust
let result = self.drive_repeated_real_fallible_verifier_time_remote_admission_collection(
    max_active_workers,
    transport_runtime,
    authority,
    session_authentication,
    expected_requests,
    supervisor_shutdown,
    admission_timing,
    on_completion,
    on_rejection,
    on_admission_failure,
);

finish_remote_endpoint_shutdown(
    self,
    result,
    |code, reason| transport_runtime.close(code, reason),
    transport_runtime.wait_idle(),
)
```

This is conceptual source shape, not authorization to alter another path or activate a caller.

The future implementation must invoke the PN fallible collection exactly once and `finish_remote_endpoint_shutdown(...)` exactly once.

## 10. No early propagation before endpoint teardown

The future sibling must capture the exact PN collection result before endpoint teardown.

It must not use an early `?` or equivalent return path which bypasses close + idle when:

`RemoteSessionPersistentCollectionConfigError::CapacityExceedsRegisteredDeviceLimit`

is returned before runtime collection work.

Even a pre-drive configuration failure must still pass unchanged through the existing endpoint teardown ordering.

## 11. Endpoint close and idle ordering law

The selected ordering is strict:

1. the PN repeated collection reaches full return;
2. endpoint close occurs exactly once;
3. `transport_runtime.wait_idle()` is driven after close;
4. the exact captured PN result is returned unchanged.

Endpoint close must not occur while PN collection work is live.

`wait_idle()` must not start before close.

No detached teardown task, parallel close/idle race, timeout, retry, fallback or cancellation of `wait_idle()` is selected.

## 12. Existing helper and constants remain authoritative

The future sibling must reuse unchanged:

- `finish_remote_endpoint_shutdown(...)`;
- `REMOTE_ENDPOINT_SHUTDOWN_CODE`;
- `REMOTE_ENDPOINT_SHUTDOWN_REASON`.

No duplicate helper or duplicate shutdown constants are selected.

No new close reason based on verifier-time failure, worker cancellation, AJ failure or collection configuration failure is selected.

## 13. Error law

The endpoint-lifecycle sibling introduces no new error enum.

Its return remains:

`Result<(), RemoteSessionPersistentCollectionConfigError>`.

The exact collection result is preserved unchanged through endpoint teardown.

Verifier-time failures remain worker completion values inside:

`RemoteSessionFallibleVerifierTimeRegisteredWorkerCompletion`.

AJ failures remain callback values through:

`RemoteSessionRepeatedAdmissionFailure`.

Neither is promoted into a new endpoint-lifecycle error.

## 14. Worker terminal and completion law

PO does not modify PN worker semantics.

Normal fallible worker terminals remain:

- `Cancelled`;
- `Failed(exact PB error)`.

Abnormal Tokio completion remains the existing bounded join error.

The endpoint lifecycle does not reinterpret, retry, stringify, aggregate or transform those completions.

The exact `on_completion` callback is passed through to PN collection ownership unchanged.

## 15. Supervisor shutdown law

The endpoint-lifecycle sibling does not add a second shutdown source.

The supplied `supervisor_shutdown` future remains the only selected stop input while the PN collection is live.

PN remains responsible for:

- cancellation request to active workers;
- retained in-flight AJ drain;
- post-shutdown AJ-success orderly owner close without worker spawn;
- retained worker completion drain;
- final collection return.

Only after that return may the outer endpoint close + idle teardown occur.

No signal source, global cancellation token, endpoint-close-driven supervisor cancellation or forced task abort is selected.

## 16. Identity and authority law

PO introduces no new identity or authorization authority.

The canonical model remains:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

Authenticated owner-derived `DeviceId` remains worker authority after AJ success.

Expected pre-authentication `DeviceId` remains scheduling intent only.

PRWM `request_id` remains transaction correlation only.

Endpoint close state, verifier timestamp, collection capacity, transport address and task identity do not become logical identity or authorization evidence.

## 17. Verifier-time provider law

PO does not select a concrete verifier-time provider.

The generic fallible provider remains owned by the existing expected-device request and passed through PN unchanged until the worker samples it through the already-established PD/PB path.

The endpoint lifecycle:

- does not sample the provider;
- does not cache it;
- does not retry it;
- does not replace it;
- does not supply a default timestamp;
- does not clamp or normalize values;
- does not convert provider failure into endpoint close reason.

## 18. Transport teardown is not authorization

Calling `transport_runtime.close(...)` after collection return is endpoint lifecycle cleanup only.

It does not grant capability access, validate a peer, authorize requester/rendezvous operations, or establish logical identity.

Likewise, successful `wait_idle()` completion is cleanup evidence only and must not be interpreted as authentication, authorization, application success, merge, deployment or runtime activation evidence.

## 19. Production/provider/caller graph explicitly deferred

PO does not select or authorize:

- concrete production verifier-time provider installation;
- production expected-device producer construction;
- requester/rendezvous scheduling mutation;
- requester/rendezvous provider mutation;
- dispatcher production population changes;
- capability-authority population changes;
- session-authentication population changes;
- listener/bind acceptance;
- readiness publication;
- signal handling;
- startup orchestration;
- `main.rs` invocation;
- service/systemd mutation;
- host credential changes;
- database/schema/control-plane mutation;
- deployment;
- merge.

The future endpoint-lifecycle sibling remains dormant until a separately gated caller composition reaches it.

## 20. Historical infallible path preservation

The historical methods remain untouched:

- `drive_repeated_real_remote_admission_collection`;
- `drive_repeated_real_remote_admission_endpoint_lifecycle`.

PO does not select broad genericization of the two lifecycle methods merely to reduce duplication.

A later PP should prefer one narrow additive fallible sibling which reuses the existing teardown helper.

No historical behavior deletion or refactor is selected.

## 21. Selected later tests

A later PP may add only focused same-file tests needed to prove the outer composition.

Selected test obligations are limited to evidence that:

- the exact PN result is preserved through teardown;
- close occurs before idle;
- close occurs exactly once;
- a collection configuration error does not bypass close + idle;
- no source outside the selected executor path changes.

Existing tests of `finish_remote_endpoint_shutdown(...)` may be reused as evidence where exact source shape makes a new duplicate test unnecessary.

No process-global environment mutation is selected for these tests.

## 22. Selected later validation law

A later PP materialization must be validated only on its exact final head.

Minimum acceptance:

- final PO -> PP compare contains exactly the selected Rust path and no unrelated historical hunks;
- Rust formatting passes;
- Clippy passes;
- workspace tests pass;
- workspace build passes;
- Android validation is reported according to actual exact-head workflow state;
- path-filtered workflows are reported as `SKIPPED`, not PASS, when skipped;
- exact source blob/tree/head are re-read after validation;
- PR remains draft/open/unmerged;
- immutable Drive evidence is frozen, uploaded once, raw-readback verified and uniquely titled;
- successor namespace remains empty at closure.

Any formatting/lint corrective commit must remain source-local and preserve full forward history. No force push, squash, rebase or history rewrite is selected.

## 23. PO exact scope

C03e-PO itself is documentation-only.

Its allowed repository delta is exactly one new contract path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_PO_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_FALLIBLE_VERIFIER_TIME_ENDPOINT_LIFECYCLE_COMPOSITION_SELECTION_STAGING.md`

PO must contain zero Rust/source/runtime/manifest/lockfile/workflow/Android/requester-rendezvous/transport/packaging/host/deployment changes.

## 24. PO explicit non-actions

C03e-PO does not:

- modify Rust/source;
- materialize C03e-PP;
- modify PN source;
- alter the PN fallible collection;
- alter the historical endpoint lifecycle;
- alter endpoint close code or reason;
- add a second runtime or runtime handle surface;
- install a verifier-time provider;
- sample verifier time in the endpoint lifecycle;
- add retry/fallback/cache/default/clamp/saturation;
- change authentication or authorization;
- change requester/rendezvous behavior;
- activate listener/startup/readiness;
- modify `main.rs`;
- change dependencies, database or security architecture;
- merge any PR;
- deploy;
- mark a draft PR ready;
- delete branches;
- force-push, squash, rebase or rewrite history.

## 25. Stop rule

C03e-PO closes only a selection decision.

After PO exact-head validation and immutable evidence are published, its draft PR may be administratively marked:

`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

Then stop at PO.

C03e-PP source materialization is a separate checkpoint requiring a fresh exact-head, exact-source, concurrency and evidence audit before any Rust mutation.