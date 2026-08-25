# Phase 152 C03e-AP — Remote Endpoint Startup + Supervisor Shutdown Control Source Materialization Staging

Status: STAGED

Target gate:

`C03E_AP_REMOTE_ENDPOINT_STARTUP_SUPERVISOR_SHUTDOWN_CONTROL_SOURCE_MATERIALIZED`

## Exact predecessor

Canonical predecessor is closed C03e-AO:

- branch: `phase-152-c03e-ao-remote-endpoint-startup-supervisor-shutdown-control-selection-staging`
- head: `3161e8f8291c99a36f56d45067fcd69b8ebe0938`
- tree: `25fcd26e2151c811e0e35bfcdfb7052bb78c47b0`
- gate: `C03E_AO_REMOTE_ENDPOINT_STARTUP_SUPERVISOR_SHUTDOWN_CONTROL_SELECTED`

C03e-AP materializes only the AO-selected Agent-internal endpoint startup ownership/order and explicit remote-supervisor shutdown-control seam.

## Exact bounded final scope

The final AO -> AP net diff is restricted to:

1. this AP source-materialization contract;
2. one new source module under `crates/prw-agent/src/remote_session_capability_runtime/` containing the startup owner, recoverable startup failure, remote-specific supervisor shutdown pair, and focused non-networking tests;
3. `crates/prw-agent/src/remote_session_capability_runtime.rs` only to register/re-export the bounded AP surface.

No manifest, lockfile, permanent workflow, Android application, bridge, transport implementation, reachability implementation, Agent `main.rs`, local Linux runtime, readiness, process-signal wiring, packaging, systemd unit, host configuration, deployment or merge mutation is selected.

## Materialized startup owner

The new module materializes an Agent-owned `RemoteSessionEndpointLifecycleRuntime` whose successful state retains exactly:

- one existing `RemoteSessionExecutorRuntime`;
- one existing successfully bound `AgentRemoteTransportRuntime`;
- one private remote-supervisor shutdown signal.

The constructor returns the lifecycle owner together with one separate `RemoteSessionSupervisorShutdownController`.

The owner exposes no raw Tokio runtime, runtime handle, generic future driver, endpoint close control, readiness state, retry state or detached task handle.

## Mandatory executor-before-bind ordering

The production constructor must implement the exact AO order:

1. receive one already-admitted `ReachabilityAuthorityRuntimeOwner` plus caller `SocketAddr`;
2. construct `RemoteSessionExecutorRuntime::new()` first;
3. only after executor success, call existing `AgentRemoteTransportRuntime::bind_from_systemd_credentials(...)` with the authority owner;
4. only after endpoint bind success, construct the supervisor shutdown controller/signal pair;
5. return the lifecycle owner plus external controller.

No credential read or endpoint bind may be attempted after executor-construction failure.

## Recoverable startup failure

AP may materialize one bounded startup-failure owner that always retains the exact reachability-authority owner when startup fails.

Its stable public error classification is limited to:

- the existing `RemoteSessionExecutorRuntimeCreateError` for executor construction;
- the existing `AgentRemoteTransportBindError` for credential/TLS/socket bind failure.

For endpoint bind failure, AP unwraps only the existing stable bind error and exact authority owner from `AgentRemoteTransportBindFailure`; it does not duplicate credential material, transport internals or raw provider errors.

The startup failure exposes:

- immutable access to the stable bounded error;
- consuming recovery of the exact `ReachabilityAuthorityRuntimeOwner`.

No retry, alternate address, second runtime, credential fallback, re-bootstrap or fabricated successful owner is permitted.

## Private ordering helper

To prove startup ordering without production credentials or a real endpoint, AP may materialize one private generic ownership helper used by the production constructor.

The helper must encode:

- executor construction before bind callback invocation;
- no bind callback after executor failure;
- exact authority value recovery on executor failure;
- exact authority value recovery on bind failure.

It is private implementation structure only and must not expose a generic production runtime composition API.

## Materialized supervisor shutdown pair

AP materializes one remote-specific durable shutdown pair backed by:

- one private `AtomicBool` requested state;
- one Tokio `Notify` wake;
- one `Arc` shared only between the single controller and single signal.

The public controller is non-cloneable and exposes only `request_shutdown()`.

The paired signal remains private to the endpoint lifecycle owner and exposes a consuming future internally.

## Shutdown semantics

`request_shutdown()`:

- sets durable requested state monotonically;
- is idempotent;
- wakes the one paired waiter;
- does not close the endpoint;
- does not cancel workers directly;
- does not abort tasks;
- does not mutate registry, policy, reachability or authentication state;
- does not publish readiness or terminal status.

Dropping the controller without an explicit request does not request shutdown.

The consuming signal future must satisfy `Future<Output = ()> + Send + 'static` and must observe a request made before first poll or racing waiter registration through durable state.

## Materialized lifecycle drive

`RemoteSessionEndpointLifecycleRuntime` exposes one domain-specific consuming drive equivalent in responsibility to:

`drive_repeated_real_remote_admission_endpoint_lifecycle(...)`

The method consumes the lifecycle owner, so the same owner cannot drive a second lifecycle after endpoint close/drain.

It forwards the existing bounded AL/AN inputs unchanged, supplies its private shutdown future, and delegates to the existing C03e-AN `RemoteSessionExecutorRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle(...)` method.

It returns exactly the existing `Result<(), RemoteSessionPersistentCollectionConfigError>`.

AP does not copy the AL supervisor, worker map, AJ state machine, endpoint-close logic or idle-drain logic.

## Existing AL + AN ordering remains authoritative

Explicit AP shutdown request only makes the supervisor future ready.

Existing source remains authoritative for all terminal work:

- AL stops expected-request polling;
- AL requests active-worker cancellations;
- AL retains/drains in-flight AJ;
- post-shutdown AJ success uses existing code-4 close semantics and is not spawned;
- AL drains all worker handles;
- AN waits for AL to return before whole-endpoint close;
- AN closes endpoint once with code `0` / `remote endpoint shutdown`;
- AN drives `wait_idle()` on the same private current-thread executor;
- AN returns the exact AL result unchanged.

AP adds no competing shutdown state machine.

## Configuration-error behavior

The AP consuming lifecycle drive delegates configuration error handling to AN unchanged.

If persistent-worker capacity is rejected before AL runtime work:

- the endpoint is still closed once by AN;
- endpoint idle is still driven to completion;
- the original `RemoteSessionPersistentCollectionConfigError` is returned unchanged.

No capacity correction/retry is introduced.

## Focused non-networking tests

AP tests must prove at least:

- explicit shutdown requested before first signal poll completes from durable state;
- pending signal is woken and completes after explicit request;
- repeated shutdown requests are idempotent;
- controller drop without request leaves the signal pending;
- the consuming shutdown future satisfies `Future<Output = ()> + Send + 'static`;
- the private startup helper invokes executor construction before bind;
- executor failure prevents bind invocation and returns the exact authority value;
- bind failure returns the exact authority value without retry;
- the public constructor has the exact authority-owner + `SocketAddr` to lifecycle-owner/controller-or-recoverable-failure shape.

Tests must not read production systemd credentials, perform provider bootstrap, bind a real endpoint, publish readiness or mutate host state.

## Identity and authority invariants

AP preserves:

- `DeviceId` / authenticated PRW session identity as logical identity;
- `TransportIdentity` as lower-transport certificate identity only;
- IP/socket address as transient endpoint/configuration data only;
- `SessionId` as authentication correlation only;
- shutdown-controller/wake/runtime/task/endpoint identifiers as implementation details only;
- endpoint close diagnostic as lifecycle data only;
- fresh current registry/current transport/current policy evaluation for protected requests through the existing shared-current authority.

Startup success and shutdown-controller possession are not capability authorization or readiness evidence.

## Runtime constraints

AP must use exactly the existing `RemoteSessionExecutorRuntime` current-thread runtime.

Forbidden:

- second Tokio runtime;
- `rt-multi-thread`;
- public/crate-public generic `block_on`;
- raw `Runtime` or `Handle` exposure;
- detached endpoint/worker drain;
- local Linux runtime driving the remote endpoint;
- hard task abort;
- hard endpoint-drain deadline.

## Explicitly still absent

C03e-AP does not materialize or activate:

- Agent `main.rs` wiring;
- automatic reachability bootstrap from the executable;
- local or remote readiness publication;
- SIGTERM/SIGINT integration;
- local Linux termination-signal integration;
- expected-device discovery/request production;
- retry/reconnect/rebind/replacement;
- parallel pre-auth AJ;
- systemd unit or host mutation;
- deployment;
- merge.

The real endpoint constructor exists but is not invoked from an executable path by AP.

## Canonical validation and closure

Because AP changes Rust Agent source, closure requires on the exact final AP head:

- PRW Rust Validation FULL PASS: locked dependency graph, rustfmt, Clippy, workspace tests, workspace build;
- PRW Android Validation FULL PASS if the canonical Android workflow triggers for these source changes;
- disposable C02f workflows recorded only as SKIPPED;
- exact AO merge base and bounded final path scope;
- no permanent helper workflow path;
- immutable Drive audit with raw byte/hash verification;
- append-only rolling Drive update preserving the complete post-AO prefix byte-for-byte;
- draft/open/unmerged PR metadata updated to CLOSED only after evidence is final.

No production activation, `main.rs`, readiness, process signals, systemd/host mutation, deployment or merge is authorized by this gate.
