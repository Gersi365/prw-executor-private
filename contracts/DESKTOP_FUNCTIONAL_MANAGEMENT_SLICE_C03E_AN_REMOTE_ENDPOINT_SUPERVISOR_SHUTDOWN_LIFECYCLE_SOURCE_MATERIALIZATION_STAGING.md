# Phase 152 C03e-AN — Remote Endpoint + Supervisor Shutdown Lifecycle Source Materialization Staging

Status: STAGED

Target gate:

`C03E_AN_REMOTE_ENDPOINT_SUPERVISOR_SHUTDOWN_LIFECYCLE_SOURCE_MATERIALIZED`

## Exact predecessor

Canonical predecessor is closed C03e-AM:

- branch: `phase-152-c03e-am-remote-endpoint-supervisor-shutdown-lifecycle-selection-staging`
- head: `f9b286f4200213bd796fd0c1ec74433af0e7e214`
- tree: `66dc31b425da9b4fe5127db9babb5ca5ce4674bb`
- gate: `C03E_AM_REMOTE_ENDPOINT_SUPERVISOR_SHUTDOWN_LIFECYCLE_SELECTED`

C03e-AN materializes only the AM-selected domain-specific composition of the existing C03e-AL supervisor with deterministic whole-endpoint close and idle drain.

## Exact bounded source scope

The final AM -> AN net diff must remain bounded to:

1. this materialization contract;
2. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs` for the fixed endpoint shutdown diagnostic, one endpoint-specific private finish helper, one domain-specific outer lifecycle method, and focused non-networking tests.

No manifest, lockfile, permanent workflow, Android application, bridge, transport implementation, Agent `main.rs`, local/remote readiness, packaging, host, systemd, deployment, or merge mutation is selected.

Temporary formatting/patch helper workflows may exist only transiently and must self-remove before canonical validation/closure.

## Materialized endpoint shutdown diagnostic

The executor module materializes fixed private constants equivalent to:

- close code: `0_u32`;
- reason: `b"remote endpoint shutdown"`.

The diagnostic is not caller-configurable and is never logical identity, authentication evidence, or capability authorization evidence.

Logical-session close codes 1 through 5 remain owned by their existing session/authentication/capability lifecycle paths and are not reused by endpoint teardown.

## Endpoint-specific finish helper

The executor module may materialize one private helper whose responsibility is strictly:

1. receive an already-computed outer lifecycle result without changing it;
2. invoke a supplied endpoint-close operation exactly once with the fixed AM diagnostic;
3. drive one supplied endpoint-idle future to completion through the same private `RemoteSessionExecutorRuntime` current-thread runtime;
4. return the original result unchanged.

The helper exists to make close-before-idle ordering and result preservation directly testable without binding a production endpoint.

It must remain private and endpoint-specific. It must not expose a generic future-driving API or raw Tokio runtime/Handle.

## Materialized outer lifecycle method

`RemoteSessionExecutorRuntime` gains one domain-specific method equivalent in responsibility to:

`drive_repeated_real_remote_admission_endpoint_lifecycle(...)`

The method accepts the same bounded C03e-AL repeated-admission/supervisor inputs, including the already-bound borrowed `AgentRemoteTransportRuntime` and caller-supplied supervisor-shutdown future.

Its ordering is exact:

1. invoke the existing `drive_repeated_real_remote_admission_collection(...)` and capture its `Result` without `?`-returning;
2. only after that method has fully returned, close the endpoint exactly once with code `0` / `remote endpoint shutdown`;
3. drive `transport_runtime.wait_idle()` on the same private current-thread runtime;
4. return the captured C03e-AL result unchanged.

No endpoint close is invoked while the AL supervisor's private `block_on` is active.

## Configuration-error preservation

The outer method must not use early `?` propagation for the C03e-AL result.

If AL returns `RemoteSessionPersistentCollectionConfigError` before entering supervisor runtime work:

- retain that exact error value;
- still execute endpoint close once;
- still drive endpoint idle to completion;
- return the unchanged error.

No capacity correction, retry, fallback or fabricated success is permitted.

## Normal supervisor-return preservation

If AL returns `Ok(())` after explicit supervisor shutdown:

- all active workers are already drained;
- any in-flight AJ is already terminal;
- logical-session-specific cleanup is already complete;
- endpoint close then occurs once;
- idle is driven to completion;
- `Ok(())` is returned unchanged.

AM/AN do not add a second terminal result classification.

## Runtime-drive constraints

Source must use only the existing private `RemoteSessionExecutorRuntime::runtime` current-thread runtime.

Allowed:
- existing AL private `block_on` completes and returns;
- then a second sequential endpoint-specific private `block_on(wait_idle)` executes.

Forbidden:
- nested `block_on`;
- second Tokio runtime;
- `rt-multi-thread`;
- public or crate-public generic `block_on`;
- runtime Handle clone/exposure;
- detached endpoint-drain task;
- local Linux runtime driving the remote endpoint.

## Endpoint/reachability ownership

The outer method borrows the existing `AgentRemoteTransportRuntime` throughout both AL supervision and endpoint idle drain.

Because the endpoint owner itself retains `ReachabilityAuthorityRuntimeOwner`, that authority custody necessarily remains live until after `wait_idle()` returns.

AN does not extract, replace, re-bootstrap, return, or mutate reachability authority during shutdown.

## Existing C03e-AL semantics remain unchanged

AN must delegate the full repeated admission and worker lifecycle to the existing C03e-AL method without copying or reimplementing its state machine.

Therefore AL continues to own:

- capacity and duplicate preflight;
- fresh AJ timing;
- at most one in-flight AJ;
- shutdown-before-AJ same-wake ordering;
- active worker cancellation and drain;
- in-flight AJ retain/drain semantics;
- post-shutdown AJ-success code-4 close;
- repeated admission failure, duplicate rejection, and worker completion callbacks.

AN adds no worker map, join collection, retry state, or cleanup authority.

## Focused non-networking tests

Tests must prove at least:

- fixed endpoint diagnostic is exactly code `0` and reason `remote endpoint shutdown`;
- endpoint close callback is invoked exactly once;
- close is observed before idle-future completion;
- a representative `RemoteSessionPersistentCollectionConfigError` result is returned unchanged after close + idle;
- the same private current-thread executor can drive a non-Send test idle future, demonstrating no detached/multi-thread requirement.

The test helper must not bind a production endpoint, read systemd credentials, mutate reachability state, or publish readiness.

## Identity and authority invariants

C03e-AN preserves:

- DeviceId/authenticated PRW session identity as logical identity;
- TransportIdentity as lower-transport certificate identity only;
- IP/socket address as transient endpoint data;
- SessionId as authentication correlation only;
- endpoint close diagnostic as lifecycle data only;
- fresh current registry/current transport/current policy evaluation for every protected request while workers are live;
- no authority guard across accept, authentication, capability dispatch, task lifecycle, endpoint close, or idle drain.

PID/UID/GID/thread/runtime/task/join/controller/channel/lock/endpoint identifiers remain non-logical implementation details.

## Explicitly still absent

C03e-AN does not materialize:

- endpoint bind/startup composition;
- concrete supervisor-shutdown controller or process signals;
- Agent `main.rs` wiring;
- local/remote readiness;
- parallel pre-auth AJ;
- reconnect/rebind/retry/replacement;
- hard drain deadline/task abort;
- second runtime / rt-multi-thread;
- generic block_on / Handle exposure;
- systemd/host mutation;
- deployment;
- merge.

## Validation and closure

Because C03e-AN changes Rust Agent source, canonical closure requires on the exact final source head:

- PRW Rust Validation FULL PASS: locked dependency graph, rustfmt, Clippy, workspace tests, workspace build;
- PRW Android Validation FULL PASS when the canonical Android workflow triggers for the source change;
- disposable C02f workflows recorded as SKIPPED and never counted as PASS;
- exact AM merge base and bounded final path scope;
- immutable Drive audit with raw byte/hash verification;
- append-only rolling Drive update preserving the complete post-AM prefix byte-for-byte;
- draft/open/unmerged PR metadata updated to CLOSED only after evidence is final.

No merge, deployment, Agent `main.rs`, readiness, process-signal wiring, endpoint startup activation, or host mutation is authorized by this gate.