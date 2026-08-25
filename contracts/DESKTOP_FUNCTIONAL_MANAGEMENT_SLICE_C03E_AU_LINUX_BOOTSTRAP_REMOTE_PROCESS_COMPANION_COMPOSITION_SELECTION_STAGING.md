# Phase 152 C03e-AU — Linux Bootstrap Remote Process Companion Composition Selection — STAGING

Completion gate:

`C03E_AU_LINUX_BOOTSTRAP_REMOTE_PROCESS_COMPANION_COMPOSITION_SELECTED`

## Canonical predecessor

C03e-AU starts only from the closed C03e-AT source-materialization checkpoint:

- repository: `Gersi365/prw-executor-private`
- predecessor branch: `phase-152-c03e-at-signal-aware-remote-capability-thread-join-handoff-source-materialization-staging`
- predecessor head: `6900459e1000e27184a01c46c9916c89b0859f29`
- predecessor tree: `62639c3cd17cea35366262b2df295aedb1fd0956`
- predecessor gate: `C03E_AT_SIGNAL_AWARE_REMOTE_CAPABILITY_THREAD_JOIN_HANDOFF_SOURCE_MATERIALIZED`

C03e-AT is already closed with canonical Rust and Android validation plus immutable/rolling Drive evidence. AU does not reopen AS/AT, AR, endpoint lifecycle, reachability, identity, transport, worker, or local-runtime audits.

## Purpose

C03e-AU selects the narrow Agent-library composition boundary that may later connect the already-validated Linux signal-aware local lifecycle to the already-materialized AT join-owned remote process companion, while keeping the actual remote capability operation injected and non-production.

This checkpoint is selection-only. It does not materialize Rust source and does not activate any remote endpoint.

## Existing source facts being composed

The selection is grounded in the current AT source:

1. `linux_bootstrap::run()` still builds the locked local bootstrap inputs and calls `run_signal_aware_linux_production_runtime_from_env(...)` with no remote companion.
2. `run_signal_aware_linux_production_runtime_from_env_with_companion(...)` already guarantees:
   - one existing `LocalLinuxTerminationSignalSource` is created first;
   - local lifecycle assembly succeeds before the companion starts;
   - local runtime terminal reason remains authoritative;
   - local worker teardown and listener/socket cleanup complete before companion finalization;
   - companion finalization completes before exact prior signal-mask restoration.
3. `RemoteSessionProcessLifecycleOwner::spawn(...)` already creates exactly one explicitly join-owned OS thread around an injected remote operation.
4. `RemoteSessionProcessLifecycleOwner::finalize(...)` already waits for the exact shutdown controller or sender disconnect, requests orderly shutdown when the controller exists, and explicitly joins the same thread.
5. Receiver-drop publication failure already recovers the exact `RemoteSessionSupervisorShutdownController` and requests orderly shutdown through it.
6. The AR same-executor reachability/bootstrap + endpoint-startup source exists, but no executable/production path invokes it yet.

AU must compose only these existing boundaries. It must not copy their state machines.

## Selected future materialization boundary

The immediately following source-materialization checkpoint may add one private, non-production helper in `crates/prw-agent/src/linux_bootstrap.rs` that composes an injected remote operation with the existing signal-aware companion seam.

The helper must remain below the public executable entry point and must not change the behavior of existing `linux_bootstrap::run()`.

The selected operation shape is one ownership-moving callback equivalent to:

`FnOnce(RemoteSessionSupervisorShutdownPublisher) + Send + 'static`

The callback is supplied by the caller of the private helper. The helper itself must not construct production reachability/bootstrap inputs, choose a bind address, create expected-device requests, construct a dispatcher, or invoke AR endpoint startup.

## Selected startup ordering

For the injected companion path, ordering is fixed:

1. construct the existing local bootstrap inputs exactly as already selected by the local bootstrap path;
2. call the existing `run_signal_aware_linux_production_runtime_from_env_with_companion(...)` seam;
3. let that seam establish the existing termination-signal source;
4. let that seam complete local lifecycle assembly;
5. only then attempt `RemoteSessionProcessLifecycleOwner::spawn(injected_operation)`;
6. continue the existing local runtime regardless of whether the remote OS-thread spawn succeeds;
7. preserve the existing local terminal reason and local cleanup as primary evidence;
8. after local listener/socket cleanup, finalize the remote companion state;
9. only after that finalization may the existing signal-aware seam restore the prior signal mask.

No remote thread may start before signal-source establishment and successful local lifecycle assembly.

## Selected remote spawn-failure semantics

The AT process-owner constructor is fallible. AU deliberately does not turn that failure into a new local bootstrap start failure.

The future private composition helper must carry remote companion construction as owned secondary state, equivalent in meaning to:

- `Running(RemoteSessionProcessLifecycleOwner)`, or
- `Unavailable(RemoteSessionProcessLifecycleSpawnError)`.

A remote thread spawn failure therefore means:

- no remote lane exists;
- no controller can be published;
- no remote join is required;
- the existing local Agent lifecycle continues unchanged;
- no local programmatic shutdown is requested;
- no retry, replacement thread, fallback runtime, process exit, or readiness mutation occurs.

This preserves the established rule that remote capability availability is not a prerequisite for the already-validated local IPC lifecycle.

## Selected finalization semantics

After local runtime termination and explicit local listener/socket cleanup:

- `Running(owner)` must be consumed by exactly one `owner.finalize()` call;
- `Unavailable(spawn_error)` performs no join and no fabricated controller action;
- the resulting remote evidence remains secondary and bounded;
- a remote thread panic is represented only through the existing bounded `RemoteSessionProcessThreadFinalization::Panicked` class;
- controller absence before endpoint startup remains the existing `UnavailableBeforeEndpointStartup` class;
- no panic payload, thread ID, runtime ID, task ID, PID, UID, GID, socket endpoint, or transport identity becomes PRW identity or public evidence.

The private helper may return or capture bounded secondary remote evidence for focused tests, but AU does not widen `LinuxAgentBootstrapReport` and does not change `LinuxAgentBootstrapReport::is_success()`.

## Selected local/remote authority relationship

Local lifecycle remains primary:

- local SIGTERM/SIGINT handling remains owned by the existing single signal source;
- remote failure must not call `LocalLinuxRuntimeShutdownHandle::request_shutdown_and_wake()`;
- local terminal reason is not replaced by remote spawn/join/startup outcomes;
- local IPC readiness is not widened to represent remote readiness;
- remote finalization evidence is secondary only.

Remote process/thread/socket identifiers are not logical identity. `DeviceId` and authenticated PRW session identity remain the logical identity model when a separately gated real remote operation is eventually supplied.

## Future source scope selected by AU

The source-materialization checkpoint immediately following AU is restricted to exactly these paths unless a concrete contradiction is first proven:

1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_AV_LINUX_BOOTSTRAP_REMOTE_PROCESS_COMPANION_COMPOSITION_SOURCE_MATERIALIZATION_STAGING.md`
2. `crates/prw-agent/src/linux_bootstrap.rs`

No other path is required by this selection.

The materialization must reuse, without modifying:

- `crates/prw-agent/src/linux_signal_aware_runtime.rs`
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_process_lifecycle_control.rs`
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`

## Focused non-production test obligations for the future materialization

Tests in `linux_bootstrap.rs` may use injected no-network operations only. They must prove at minimum:

1. remote operation does not start when local lifecycle assembly fails before the companion start point;
2. remote thread spawn failure remains secondary and does not manufacture a local startup failure;
3. a successfully spawned injected remote lane is explicitly finalized/joined after local lifecycle cleanup;
4. the existing public `run()` path remains the no-companion path unless a later activation gate changes it;
5. no production reachability provider, systemd remote credential, network bind, expected-device admission, or real dispatcher is invoked by the focused tests.

If proving cleanup-before-finalizer ordering directly through the existing signal-aware runtime is already covered by AT tests, AV should reference/reuse that fact rather than duplicate the signal-aware state machine.

## Explicit non-claims / retained gates

C03e-AU does not authorize or claim:

- Rust/source materialization;
- any change to `main.rs`;
- changing existing `linux_bootstrap::run()` behavior;
- production invocation of the AR reachability/bootstrap seam;
- production endpoint bind;
- production bind-address selection;
- production expected-device source/discovery;
- production dispatcher construction;
- production session-authentication input construction;
- remote readiness publication;
- local readiness widening;
- a second `SignalFd` or signal handler;
- a second Tokio runtime beyond the already-selected private remote current-thread runtime when a real operation is separately gated;
- generic `block_on` or Tokio `Handle` exposure;
- detached threads/tasks;
- retry/reconnect/rebootstrap/rebind/replacement;
- hard abort or shutdown deadline;
- systemd, host, firewall, route, DNS, TUN/TAP, NAT or deployment mutation;
- recovery/PRWF/R1-R4 activation;
- merge.

## Closure criteria for AU

AU may be declared closed only when:

1. this contract is the sole AT→AU repository change;
2. AT is the exact merge base, with AU ahead by one and behind by zero;
3. the AU PR remains draft/open/unmerged and mergeable;
4. canonical Rust validation on the exact AU head is terminal success;
5. disposable C02f workflows are terminal skipped as applicable;
6. no Android PASS is claimed unless an Android workflow actually runs on the docs-only head;
7. immutable Drive audit is uploaded and raw-readback verified;
8. rolling evidence is appended only after a fresh predecessor-byte guard;
9. the PR body changes from `Status: STAGED` to `Status: CLOSED` only after Drive evidence is complete.

Closure of AU selects only the private non-production Linux-bootstrap remote-process companion composition. Source materialization remains separately gated as C03e-AV.