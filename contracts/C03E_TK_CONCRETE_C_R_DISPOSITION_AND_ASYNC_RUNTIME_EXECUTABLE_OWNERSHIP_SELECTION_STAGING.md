# C03e-TK Concrete C/R Disposition and Async Runtime / Executable Ownership Selection — STAGING

Status: `SELECTION — VALIDATION PENDING`

## 1. Boundary

C03e-TK selects the concrete completion/rejection disposition and the async-runtime ownership boundary above the evidence-closed C03e-TJ caller-ready adapter.

This checkpoint is **selection only**. It does not materialize Rust source, does not modify `linux_bootstrap.rs` or `main.rs`, and does not activate any production remote listener/runtime path.

Selected law:

`C_COMPLETION_OBSERVATION_TERMINALLY_CONSUMED_AUTHORITY_FREE / R_DUPLICATE_OR_FUTURE_BOUNDED_REJECTION_TERMINALLY_CONSUMES_REASON_AND_UNTOUCHED_REQUEST / NO_LOGGING_TELEMETRY_RETRY_REQUEUE_PERSISTENCE_OR_DEAD_LETTER / DEDICATED_CALLER_OWNED_CURRENT_THREAD_TOKIO_RUNTIME / ENABLE_ALL / EXACTLY_ONE_RUNTIME_CONSTRUCTION_PER_DRIVER_INVOCATION / EXACTLY_ONE_BLOCK_ON_OF_TJ_ADAPTER / NO_REMOTE_SESSION_EXECUTOR_RUNTIME_REUSE / NO_RUNTIME_HANDLE_EXPORT / NO_ASYNC_DRIVER_THREAD / EVENTUAL_EXECUTABLE_OWNER_IS_PROCESS_MAIN_THREAD / CURRENT_LINUX_BOOTSTRAP_RUN_COMPATIBILITY_PRESERVED / ONE_PATH_FUTURE_SOURCE_CEILING / NO_MAIN_MUTATION / NO_RUNTIME_ACTIVATION`

## 2. Authoritative predecessor

Controlling predecessor is C03e-TJ final repair head:

- PR `#649`
- branch `phase-152-c03e-tj-selected-timing-failure-custody-caller-ready-adapter-source-materialization`
- head `209e2a423920e66db0f3aa6998025b8eaf5706de`
- tree `b51cb37a1e08843cc1aef4d1e8c7eb331bd4b236`
- final net diff from TI: exactly one Rust path, `+188/-1`
- final source blob: `4b7ec7fde89a72e897e502e28263c4f20e0d71eb`
- state recovered before TK: draft / open / unmerged / evidence-closed

C03e-TJ evidence chain remains two immutable Drive objects because of its recorded post-publication forward repair:

1. original intended TJ snapshot — Drive `1b5ctMDe9yNFJ_5kRVXmjHuw0gFjTWoDv`
2. controlling final-head corrective rebind — Drive `1Z3odlGZV9ScBaOYgcoMmTUQ8N7OMY6UR`

Both were freshly rechecked before TK as exactly one revision with `previousRevisionId = null`.

Stable integrated `main` remained:

- head `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`
- tree `63b8e59ca53797fdea6b95432e16f35eaf473604`

## 3. Exact source anchors audited at TJ final tree

### 3.1 TJ caller-ready adapter

Path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Exact TJ source blob:

`4b7ec7fde89a72e897e502e28263c4f20e0d71eb`

TJ materializes:

`run_with_production_durable_reachability_requester_rendezvous_configured_application_lease_companion_with_selected_timing_and_failure_custody<C, R>(on_completion, on_rejection)`

The adapter already binds:

- fresh server-local pre-AJ timing F;
- terminal pre-AJ timing K;
- terminal real-admission E;
- TH configured application-lease source wrapper.

Only C and R remain caller-supplied.

### 3.2 C observation shape

`RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffObservationProjection` is explicitly documented as a bounded higher observation that owns no scheduling grant, request, dispatcher, sender, receiver, endpoint, verifier-time provider, retry authority, continuation, stream, task handle, or raw lower error payload.

Its variants are bounded terminal observations:

- `Cancelled`
- `VerifierTimeFailure`
- `IngressFailure`
- `RequesterResponseFailure`
- `SchedulingDerivationFailure { acknowledgement }`
- `AbnormalTaskCompletion`
- `EligibleTerminal { acknowledgement, disposition }`

Therefore C does not own continuation authority requiring another action.

### 3.3 R rejection shape

`RemoteSessionExpectedDeviceAdmissionRejectionReason` is a bounded non-exhaustive reason family. Current materialized reason is `DuplicateActiveDevice`.

R receives the reason plus the exact untouched `RemoteSessionExpectedDeviceAdmissionRequest<D, T>` by value before AJ/network work.

That request owns:

- expected logical `DeviceId`;
- `SessionId`;
- authentication request ID;
- dispatcher;
- request-owned verifier-time provider.

No transport identity is supplied by the request.

### 3.4 Existing async/runtime separation

`RemoteSessionExecutorRuntime` already owns a private current-thread Tokio runtime for lower remote transport/session work. Its documentation intentionally exposes only bounded worker-driving seams and explicitly does **not** expose a generic `block_on` or runtime handle.

TK does not repurpose it as the higher-owner configured-source async driver.

`linux_bootstrap::run()` remains the current executable-facing synchronous local Linux runtime facade and is still the only function called by current `main.rs` after device-identity preflight.

The existing remote process companion uses one separately joined OS thread via `RemoteSessionProcessLifecycleOwner::spawn`. That thread remains lower runtime custody; TK does not add another driver thread.

## 4. Selected concrete C sink

Future helper name:

`dispose_production_remote_session_handoff_completion_observation`

Semantic signature:

```text
fn dispose_production_remote_session_handoff_completion_observation(
    requester_device_id: DeviceId,
    observation: RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffObservationProjection,
)
```

Selected behavior:

1. receive both values by ownership exactly once;
2. treat `DeviceId` as bounded requester correlation only;
3. consume/release both values naturally;
4. return unit.

The sink must not:

- branch into retry/requeue policy;
- reconstruct a request;
- alter scheduling authority;
- emit logs;
- emit metrics or telemetry;
- persist or enqueue an event;
- dead-letter the observation;
- restart/exit the process;
- expose requester identity or lower details;
- synthesize success/failure semantics beyond the already-bounded projection.

Dropping this observation is selected because it is already terminal and authority-free.

## 5. Selected concrete R sink

Future helper name:

`dispose_production_remote_session_expected_device_admission_rejection`

Semantic signature:

```text
fn dispose_production_remote_session_expected_device_admission_rejection(
    reason: RemoteSessionExpectedDeviceAdmissionRejectionReason,
    request: RemoteSessionExpectedDeviceAdmissionRequest<
        LinuxAgentProductionRemoteCapabilityDispatcher,
        fn() -> Result<u64, PrwaVerifierSourceError>,
    >,
)
```

Selected behavior:

1. receive the exact bounded reason and untouched request by ownership once;
2. do not inspect or transform request identity/correlation fields for side effects;
3. terminally release the reason and request;
4. allow normal ownership drop to release the request-owned dispatcher/provider;
5. return unit.

The sink must not:

- retry or requeue the request;
- clone or recreate the dispatcher/provider;
- create a replacement channel/sender;
- resample timing;
- enter AJ/session/transport work;
- remint request IDs/session IDs;
- emit logs/telemetry;
- persist/dead-letter the request;
- map the rejection to K or E;
- turn rejection into success;
- exit/restart the process.

The selection applies to the bounded non-exhaustive rejection family without assuming `DuplicateActiveDevice` is forever the only variant.

## 6. Selected no-argument async composition

Future helper name:

`run_with_production_durable_reachability_requester_rendezvous_configured_application_lease_companion_with_selected_disposition`

It remains crate-private and async.

It must call the existing TJ adapter exactly once with:

1. `dispose_production_remote_session_handoff_completion_observation` as C;
2. `dispose_production_remote_session_expected_device_admission_rejection` as R.

It returns the exact TJ adapter result unchanged:

```text
Result<
    LinuxAgentBootstrapWithRemoteReport,
    LinuxAgentProductionDurableReachabilityRequesterRendezvousConfiguredApplicationLeaseCompanionError,
>
```

It adds no new error mapping, source read, clock read, channel, retry or runtime.

## 7. Selected async runtime owner

The caller-ready async composition is driven by a new, dedicated higher-owner synchronous driver.

Future helper name:

`run_with_production_durable_reachability_requester_rendezvous_configured_application_lease_companion_with_selected_executable_custody`

Selected behavior:

1. construct exactly one Tokio runtime per driver invocation with:
   - `tokio::runtime::Builder::new_current_thread()`
   - `.enable_all()`
   - `.build()`
2. do not retry runtime construction;
3. call `runtime.block_on(...)` exactly once;
4. block on only the selected no-argument async composition from section 6;
5. expose no runtime handle;
6. clone no runtime handle;
7. spawn no task solely to drive the top-level future;
8. spawn no new OS thread for this async driver;
9. drop the runtime after the top-level composition returns;
10. return a bounded selected-driver result.

The runtime is process-caller-owned. It is not the lower `RemoteSessionExecutorRuntime` and must not replace or enter that owner's private worker runtime.

## 8. Why current-thread + enable-all is selected

Configured higher-owner population is async and may traverse existing production source/provider operations before the synchronous Linux companion begins.

A dedicated current-thread runtime:

- keeps async source population on the same eventual executable caller thread;
- avoids a new independent driver thread;
- avoids a multi-thread scheduling authority change;
- avoids leaking a Tokio `Handle` into lower layers;
- leaves the existing remote companion thread and its lower executor ownership unchanged.

`enable_all()` is selected so existing I/O/time-backed async production source operations retain their current Tokio driver requirements without source-specific runtime branching.

TK does not select `Handle::current`, `spawn_blocking`, `block_in_place`, a global runtime, a static runtime, runtime reuse across invocations, or nested use of `RemoteSessionExecutorRuntime`.

## 9. Selected driver failure family

Future crate-private error:

`LinuxAgentProductionConfiguredApplicationLeaseSelectedExecutableCustodyError`

Selected variants:

```text
RuntimeConstruction
Companion(
    LinuxAgentProductionDurableReachabilityRequesterRendezvousConfiguredApplicationLeaseCompanionError
)
```

`RuntimeConstruction` is bounded and does not expose raw Tokio/std I/O construction details.

`Companion(...)` preserves the exact existing TJ/TH configured-source / configured-population / Linux companion error chain via `std::error::Error::source()`.

Ordering law:

- runtime construction failure occurs before the selected async composition is polled;
- therefore it occurs before application-lease source read, expected-request channel creation, configured production population, K/E/C/R, or Linux/remote companion startup;
- no retry/fallback runtime is attempted.

## 10. Selected synchronous driver result

Semantic result:

```text
Result<
    LinuxAgentBootstrapWithRemoteReport,
    LinuxAgentProductionConfiguredApplicationLeaseSelectedExecutableCustodyError,
>
```

On async composition failure, map exactly once to `Companion(exact_error)`.

No exit-code policy, user-facing log policy, telemetry policy, restart policy or service-manager policy is selected by TK.

## 11. Executable ownership boundary

TK selects the **process main thread** as the eventual owner of the synchronous driver.

Reason:

- current `main.rs` invokes `linux_bootstrap::run()` synchronously on the process main thread;
- the selected driver ultimately enters the existing synchronous signal-aware Linux bootstrap/remote-companion path;
- keeping the future activation on the process main thread preserves that existing process-lifecycle ownership rather than moving the local runtime to an invented helper thread.

However, TK does **not** select or materialize the concrete `main.rs` invocation.

The current executable remains unchanged and still calls only:

`prw_agent::linux_bootstrap::run()`

A later separately authorized activation checkpoint must decide the public executable-facing facade, exact `main.rs` replacement/callsite, exit mapping and any user-visible diagnostic policy.

## 12. No second async-driver thread

The selected driver must not call:

- `std::thread::spawn`;
- `tokio::spawn` for the top-level future;
- `spawn_blocking`;
- `block_in_place`;
- a detached task/thread mechanism.

The existing `RemoteSessionProcessLifecycleOwner::spawn` remains the sole separately joined remote capability OS-thread custody when lower Linux bootstrap begins.

## 13. Runtime lifetime

The dedicated current-thread Tokio runtime remains owned by the synchronous driver for the entire `block_on` call.

The selected async composition may eventually enter the existing synchronous Linux bootstrap/remote companion and remain there until process lifecycle termination. The runtime object may therefore remain alive while that synchronous lower call is in progress, but TK assigns it no further authority during that period.

No runtime handle is exported to the remote companion or lower runtime owners.

## 14. C/R failure and panic policy

C and R sinks are pure ownership-consuming functions with no fallible return surface.

TK introduces no `catch_unwind`, panic conversion or callback panic recovery. Normal Rust panic semantics remain unchanged; the selected sinks themselves contain no intentional panic path.

## 15. Existing failure custody remains unchanged

- application-lease configuration/source failure remains TH process-configuration failure;
- configured production population/bootstrap failure remains existing companion failure;
- pre-AJ F failure remains K;
- real-admission failure remains E;
- C remains bounded terminal handoff observation disposition;
- R remains pre-AJ duplicate/other bounded rejection disposition.

No domain is remapped into another.

## 16. Current compatibility surfaces remain untouched

TK preserves:

- `linux_bootstrap::run()`;
- all historical remote timing/admission lanes;
- TJ caller-ready adapter with caller-supplied C/R;
- TH configured lease source wrapper;
- TF typed-policy propagation;
- TD fresh proof/post-auth lease transaction;
- existing remote process companion thread ownership;
- existing lower `RemoteSessionExecutorRuntime` ownership.

No API is replaced or reinterpreted by this selection.

## 17. First future source-materialization ceiling

If separately authorized, the immediate TK successor should modify exactly one Rust path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

It may add only:

1. concrete C terminal disposal helper;
2. concrete R terminal disposal helper;
3. no-argument async selected-disposition wrapper;
4. crate-private selected executable-custody error;
5. crate-private synchronous current-thread Tokio driver;
6. bounded tests/type assertions.

It must not modify:

- `crates/prw-agent/src/linux_bootstrap.rs`;
- `crates/prw-agent/src/main.rs`;
- `crates/prw-agent/src/lib.rs`;
- lower remote-session runtime paths;
- Cargo manifests or lockfile;
- workflows;
- service/deployment files.

If compilation demonstrates a required second source path, stop and review the ceiling instead of silently widening it.

## 18. Future tests / source-validation expectations

The source successor should prove, where practical:

- C exact function-pointer shape;
- R exact function-pointer shape;
- selected no-argument async wrapper compiles against TJ C/R bounds;
- selected driver result/error type shape;
- driver uses one current-thread runtime construction and one `block_on` path;
- no `RemoteSessionExecutorRuntime` reuse;
- no driver `tokio::spawn`, OS thread spawn, runtime handle export or retry;
- no logging macros in the new C/R/error/driver code;
- no `linux_bootstrap.rs` / `main.rs` changes;
- exactly one changed Rust path.

Full canonical workspace CI remains required on the exact final source head.

## 19. Explicit non-actions

TK performs no:

- Rust/source materialization;
- concrete `main.rs` invocation;
- public executable-facing facade selection/materialization;
- executable exit-code mapping for new driver errors;
- user-visible logging policy;
- telemetry/metrics sink;
- retry/requeue/dead-letter policy;
- listener/readiness/network activation;
- service-manager mutation;
- deployment/restart;
- merge/ready conversion/PR close/branch deletion;
- history rewrite/force push;
- repository configuration mutation;
- destructive evidence cleanup.

## 20. STOP

C03e-TK stops after selecting concrete C/R terminal disposition and caller-owned async runtime/executable ownership semantics.

Source materialization and executable activation remain separately gated.
