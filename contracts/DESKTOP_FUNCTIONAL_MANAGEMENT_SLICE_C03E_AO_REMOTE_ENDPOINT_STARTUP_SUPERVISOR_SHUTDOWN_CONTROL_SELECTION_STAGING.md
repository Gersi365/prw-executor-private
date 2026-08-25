# Phase 152 C03e-AO — Remote Endpoint Startup + Supervisor Shutdown Control Selection Staging

Status: STAGED

Target gate:

`C03E_AO_REMOTE_ENDPOINT_STARTUP_SUPERVISOR_SHUTDOWN_CONTROL_SELECTED`

## Exact predecessor

Canonical predecessor is closed C03e-AN:

- branch: `phase-152-c03e-an-remote-endpoint-supervisor-shutdown-lifecycle-source-materialization-staging`
- head: `d08485c39a73d458a3edb5f22e370f423a18fa53`
- tree: `db3cb81de7bdcbe96b6829706ff990c656cf7d6a`
- gate: `C03E_AN_REMOTE_ENDPOINT_SUPERVISOR_SHUTDOWN_LIFECYCLE_SOURCE_MATERIALIZED`

C03e-AO selects only the next Agent-internal composition boundary between the already-existing real remote endpoint bind seam, the already-private current-thread remote-session executor, the C03e-AN endpoint lifecycle drive, and one explicit remote-supervisor shutdown control pair.

AO is selection-only. It does not materialize Rust source and does not invoke any production endpoint lifecycle.

## Exact bounded checkpoint scope

The final AN -> AO diff must contain exactly this one docs-only contract path.

No Rust source, manifest, lockfile, workflow, Android application, remote bridge, remote transport implementation, Agent `main.rs`, local Linux runtime, readiness, packaging, systemd unit, host configuration, deployment, or merge mutation is selected by AO.

## Existing authoritative building blocks

AO reuses without redesign:

1. `ReachabilityAuthorityRuntimeOwner`, which owns one successfully admitted reachability authority;
2. `AgentRemoteTransportRuntime::bind_from_systemd_credentials(...)`, which consumes that authority owner and binds one real QUIC endpoint from fixed systemd-delivered mesh credentials;
3. `AgentRemoteTransportBindFailure`, which already retains and can recover the exact authority owner on credential/TLS/socket bind failure;
4. `RemoteSessionExecutorRuntime::new()`, which constructs the existing private non-cloneable Tokio current-thread runtime with I/O/time drivers;
5. `RemoteSessionExecutorRuntime::drive_repeated_real_remote_admission_endpoint_lifecycle(...)`, materialized by C03e-AN, which drives the complete C03e-AL supervisor to return, then closes the endpoint once and drives `wait_idle()` before returning the exact AL result;
6. the existing shared-current registry/policy/transport authority, session-authentication service, expected-device request source, timing inputs, dispatchers and completion/rejection/failure callbacks already consumed by the AL/AN path.

AO does not replace any of these owners or duplicate their state machines.

## Selected startup ownership boundary

A future source-materialization checkpoint may introduce one Agent-internal runtime owner equivalent in responsibility to:

`RemoteSessionEndpointLifecycleRuntime`

Its successful state owns exactly:

- one `RemoteSessionExecutorRuntime`;
- one successfully bound `AgentRemoteTransportRuntime`;
- one private paired remote-supervisor shutdown signal that has not yet been consumed by the lifecycle drive.

The corresponding explicit shutdown controller is returned to the caller as a separate non-cloneable authority object.

The owner does not contain a second Tokio runtime, a raw runtime handle, a local Linux scheduler, process signal state, readiness state, retry state, or detached task ownership.

## Mandatory startup ordering

Startup ordering is selected exactly as follows:

1. receive one already-admitted `ReachabilityAuthorityRuntimeOwner` and one caller-supplied `SocketAddr` bind address;
2. construct `RemoteSessionExecutorRuntime` first;
3. only after executor construction succeeds, consume the authority owner into `AgentRemoteTransportRuntime::bind_from_systemd_credentials(...)`;
4. only after endpoint bind succeeds, construct the remote-supervisor shutdown controller/signal pair;
5. return the composed lifecycle owner plus the external shutdown controller.

The ordering is intentional and mandatory.

AO rejects binding the real endpoint before executor construction because executor-construction failure after a successful bind would otherwise leave a live endpoint without the already-selected private executor available to drive deterministic endpoint idle teardown.

## Executor-construction failure custody

If `RemoteSessionExecutorRuntime::new()` fails:

- no mesh credential read occurs;
- no UDP/QUIC endpoint is bound;
- the exact input `ReachabilityAuthorityRuntimeOwner` remains recoverable by the caller;
- no automatic reachability re-bootstrap is attempted;
- no retry or fallback runtime is constructed.

A future bounded startup-failure type may retain the exact authority owner beside the existing `RemoteSessionExecutorRuntimeCreateError` so the failure remains recoverable without exposing authority internals.

## Endpoint-bind failure custody

If executor construction succeeds but `AgentRemoteTransportRuntime::bind_from_systemd_credentials(...)` fails:

- no lifecycle owner is fabricated;
- the existing `AgentRemoteTransportBindFailure` remains authoritative for credential/TLS/socket failure classification;
- the exact authority owner remains recoverable through that existing failure owner;
- the newly constructed executor contains no spawned work and may be dropped normally;
- no second bind, alternate address, credential fallback, re-bootstrap, reconnect or retry is attempted.

AO does not broaden the existing bind error taxonomy or expose credential material.

## Bind address semantics

The caller-supplied `SocketAddr` is endpoint configuration only.

It is not:

- logical `DeviceId` identity;
- authenticated PRW session identity;
- `TransportIdentity`;
- authentication evidence;
- capability authorization evidence;
- readiness evidence.

The existing transport layer remains authoritative for the actual kernel-selected local endpoint address returned by `local_addr()`.

## Selected remote-supervisor shutdown pair

A future source-materialization checkpoint may introduce one remote-specific pair equivalent in responsibility to:

- `RemoteSessionSupervisorShutdownController`;
- `RemoteSessionSupervisorShutdownSignal`;
- `remote_session_supervisor_shutdown_pair()`.

The pair is independent of the existing single-worker cancellation pair. Worker cancellation remains owned by C03e-AD/AH/AL and is not reused as process/supervisor shutdown authority.

The selected supervisor pair uses one private durable monotonic requested-state plus one Tokio async wake mechanism, matching the already-proven lost-wake-safe lifecycle pattern without sharing the worker-cancellation type.

## Shutdown-controller semantics

The controller has one explicit operation equivalent in responsibility to:

`request_shutdown()`

Its semantics are exact:

- transition to requested state is monotonic;
- repeated requests are idempotent;
- explicit request wakes the single paired supervisor waiter;
- controller drop without an explicit request does not request shutdown;
- the controller does not close the endpoint;
- the controller does not cancel workers directly;
- the controller does not abort tasks;
- the controller does not mutate registry, policy, reachability or session state;
- the controller does not publish readiness or terminal status.

Neither public half is selected as `Clone`.

A later process-signal integration may transfer the one controller to an explicitly selected signal-owning path, but AO does not select that integration.

## Shutdown-signal semantics

The paired signal is a single-consumer future source.

It may expose one consuming operation equivalent in responsibility to:

`into_shutdown()`

The produced future must satisfy the existing AN/AL supervisor bound:

`Future<Output = ()> + Send + 'static`

The signal completes only after explicit shutdown request. Durable requested-state, not wake delivery itself, is the lifecycle truth, so a request occurring before first poll or racing waiter registration cannot be permanently missed.

Dropping the external controller without requesting shutdown does not make the signal ready.

## Lifecycle drive composition

A successful startup owner may expose one domain-specific drive operation that delegates to the existing C03e-AN method rather than copying its state machine.

The drive operation:

1. consumes the one stored supervisor shutdown signal exactly once;
2. borrows the stored `AgentRemoteTransportRuntime`;
3. mutably borrows the stored `RemoteSessionExecutorRuntime`;
4. forwards the existing bounded AL/AN inputs unchanged;
5. calls `drive_repeated_real_remote_admission_endpoint_lifecycle(...)`;
6. returns exactly the existing `Result<(), RemoteSessionPersistentCollectionConfigError>` from that method.

The future source materialization may choose an ownership shape that makes a second lifecycle drive impossible after the signal has been consumed and the endpoint has been closed/drained.

No generic future-driving or endpoint-control API is selected.

## Shutdown ordering remains owned by AL + AN

`request_shutdown()` means only that the paired supervisor-shutdown future becomes ready.

After that readiness is observed, existing code remains authoritative:

- C03e-AL stops expected-request polling;
- C03e-AL requests cancellation on all active workers;
- C03e-AL retains and drains any in-flight AJ rather than dropping/aborting it;
- post-shutdown AJ success follows its existing code-4 authenticated-owner close path and is not spawned;
- all active worker handles are drained;
- only after AL fully returns does C03e-AN close the whole endpoint once with code `0` / `remote endpoint shutdown`;
- C03e-AN drives endpoint `wait_idle()` on the same private current-thread runtime;
- the exact AL result is returned unchanged.

The AO controller never bypasses this ordering by calling endpoint `close()` directly.

## Configuration-error behavior

If the lifecycle drive rejects worker capacity before entering AL supervisor runtime work:

- the paired shutdown signal is irrelevant to the already-terminal configuration result;
- C03e-AN still closes the bound endpoint exactly once;
- C03e-AN still drives endpoint idle to completion;
- the original `RemoteSessionPersistentCollectionConfigError` is returned unchanged.

AO does not retry with corrected capacity and does not keep the endpoint open after the drive returns.

## Endpoint and authority lifetime

The composed lifecycle owner keeps `AgentRemoteTransportRuntime` alive for the complete lifecycle drive.

Therefore its retained `ReachabilityAuthorityRuntimeOwner` remains alive through:

- repeated lower-transport admission;
- logical-session authentication;
- worker execution;
- supervisor shutdown and worker drain;
- whole-endpoint close;
- endpoint idle completion.

After the AN drive returns, normal owner drop may release the closed/idle endpoint and reachability authority. AO does not select authority extraction or reuse after successful lifecycle completion.

## Identity and authorization invariants

AO preserves all established identity boundaries:

- `DeviceId` / authenticated PRW session identity is logical identity;
- `TransportIdentity` is lower-transport certificate identity only;
- IP/socket address is transient endpoint/configuration data only;
- `SessionId` is authentication correlation only;
- shutdown-controller identity, wake state, Tokio runtime identity, task IDs and endpoint IDs are implementation details only;
- endpoint close diagnostic code/reason is lifecycle data only.

Protected requests continue to use fresh current registry/current transport/current policy evaluation through the existing shared-current authority path. Neither startup success nor shutdown-controller possession is capability authorization evidence.

## Runtime constraints

AO preserves the existing private current-thread runtime model.

Allowed:

- construct exactly one `RemoteSessionExecutorRuntime` before endpoint bind;
- use the existing AN domain-specific lifecycle drive;
- sequential internal private runtime drives already selected by AN.

Forbidden:

- a second Tokio runtime;
- `rt-multi-thread`;
- exposing `Runtime` or `Handle`;
- generic public/crate-public `block_on`;
- detached endpoint or worker drain task;
- local Linux runtime driving the remote endpoint;
- hard task abort or hard endpoint-drain deadline.

## Readiness remains separately gated

Successful startup composition means only:

- executor construction succeeded;
- fixed mesh credentials were loaded successfully;
- one real endpoint bind succeeded;
- one shutdown pair exists.

It does not publish or imply:

- local Agent readiness;
- remote endpoint readiness;
- reachability health;
- registry health;
- policy health;
- authentication readiness;
- deployment success.

No `sd_notify`, readiness file/state, local readiness counter, or remote readiness publication is selected.

## Process signals remain separately gated

AO does not connect the new remote shutdown controller to:

- SIGTERM;
- SIGINT;
- the existing local Linux termination-signal path;
- eventfd/local runtime wake;
- systemd stop behavior;
- `main.rs`.

The controller is selected only as an explicit in-process authority seam suitable for a later separately gated process-lifecycle integration.

## Production activation remains absent

Although the future constructor will perform a real bind when called, AO does not call it from any executable path.

Therefore AO does not activate a production remote listener and does not change runtime behavior of the current Agent binary.

## Focused future source-level proof

The next source-materialization checkpoint must be able to prove without activating a production endpoint that:

- shutdown requested before signal poll is observed from durable state;
- pending shutdown signal is woken and then completes;
- repeated shutdown requests are idempotent;
- controller drop without request leaves the signal pending;
- the consumed shutdown future satisfies `Future<Output = ()> + Send + 'static`;
- startup ordering constructs executor before attempting endpoint bind;
- executor-construction failure retains the exact authority owner and performs no bind attempt;
- bind failure delegates to the existing retained-authority bind failure without retry;
- no second runtime, generic `block_on`, readiness or process-signal integration is introduced.

Tests must use injected/private helpers or compile-time shape checks where necessary. They must not read production systemd credentials, perform provider bootstrap, bind a production endpoint, publish readiness, or mutate host state merely to prove ordering.

## Explicitly still absent

C03e-AO does not select or materialize:

- Rust source implementation;
- Agent `main.rs` wiring;
- local/remote readiness publication;
- process-signal integration;
- automatic reachability bootstrap invocation from the executable;
- automatic expected-device request production/discovery;
- retry/reconnect/rebind/replacement;
- parallel pre-auth AJ attempts;
- multiple workers per `DeviceId`;
- hard shutdown deadline/task abort;
- second runtime / `rt-multi-thread`;
- generic `block_on` / runtime Handle exposure;
- systemd unit or host mutation;
- deployment;
- merge.

## Validation and closure

Because AO is docs-only, canonical closure requires on the exact final AO head:

- PRW Rust Validation FULL PASS: locked dependency graph, rustfmt, Clippy, workspace tests, workspace build;
- no Android PASS is claimed unless the canonical Android workflow actually triggers;
- disposable C02f workflows are recorded as SKIPPED and never counted as PASS;
- exact AN merge base and exact one-path docs-only net scope;
- immutable Drive audit with raw byte/hash verification;
- append-only rolling Drive update preserving the complete post-AN predecessor prefix byte-for-byte;
- draft/open/unmerged PR metadata updated to CLOSED only after evidence is final.

No production endpoint activation, Agent `main.rs`, readiness, process signals, deployment, host mutation or merge is authorized by this gate.
