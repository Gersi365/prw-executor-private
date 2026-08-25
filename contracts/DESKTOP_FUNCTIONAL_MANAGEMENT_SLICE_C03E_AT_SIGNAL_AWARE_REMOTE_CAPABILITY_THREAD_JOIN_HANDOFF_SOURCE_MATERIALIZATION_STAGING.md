# Phase 152 C03e-AT — Signal-Aware Remote Capability Thread/Join Handoff Source Materialization Staging

Status: STAGED

Target gate:

`C03E_AT_SIGNAL_AWARE_REMOTE_CAPABILITY_THREAD_JOIN_HANDOFF_SOURCE_MATERIALIZED`

## Exact predecessor

Canonical predecessor is closed C03e-AS:

- branch: `phase-152-c03e-as-signal-aware-remote-capability-thread-join-handoff-selection-staging`
- head: `61be4462cf8c6128334dd6e71c86cf71c7d98d01`
- tree: `03a655477342dd83d21d48c97afe53dd5f238d8e`
- gate: `C03E_AS_SIGNAL_AWARE_REMOTE_CAPABILITY_THREAD_JOIN_HANDOFF_SELECTED`

C03e-AT materializes only the AS-selected lifecycle companion seam and the bounded one-thread controller-handoff/join owner required to prove its ordering. It does not activate the remote capability from the production executable path.

## Exact bounded source scope

The final AS -> AT diff is restricted to exactly these four paths:

1. this contract;
2. `crates/prw-agent/src/linux_signal_aware_runtime.rs`;
3. `crates/prw-agent/src/remote_session_capability_runtime.rs`;
4. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_process_lifecycle_control.rs`.

No Cargo manifest, lockfile, workflow, Android application, `linux_bootstrap.rs`, `main.rs`, readiness/status implementation, transport/reachability implementation, systemd unit, host configuration, production credential payload, deployment state or merge mutation is allowed in AT.

## Materialized signal-aware companion seam

AT may add one crate-internal generic companion composition under the existing signal-aware runtime with exactly this responsibility:

1. create the existing single `LocalLinuxTerminationSignalSource` exactly where the current entry point creates it;
2. assemble the existing local lifecycle unchanged;
3. after successful lifecycle assembly and before the first local readiness wait, invoke the existing `on_started` callback and one companion-start callback;
4. carry the resulting owned companion value through the local lifecycle callback result;
5. run the existing local signal-aware loop unchanged;
6. retain the existing explicit listener/socket cleanup result;
7. consume the companion through one finalizer only after lifecycle cleanup has completed;
8. call `signal_source.restore()` only after that finalizer returns;
9. preserve the existing `LocalLinuxSignalAwareRuntimeTerminalReport` and startup-error semantics.

The current public `run_signal_aware_linux_production_runtime_from_env(...)` must remain behaviorally stable and delegate through the companion composition with unit/no-op companion state.

AT does not create a plugin system, public callback registry, dynamic hook collection, trait-object service container or general executor service.

## Materialized remote process-lifecycle owner

AT may add one Agent-internal non-cloneable process owner for a separately supplied remote-lane operation. The owner may contain only:

- one `std::thread::JoinHandle<()>` for exactly one joinable remote capability lane;
- one one-slot synchronous receiver for ownership handoff of the existing non-cloneable `RemoteSessionSupervisorShutdownController`;
- bounded finalization evidence classifying whether a controller was observed and whether the thread joined or panicked.

The owner is process lifecycle state only. It is not PRW identity, authentication, transport identity, authority, authorization or readiness evidence.

The spawn seam remains crate-internal and operation-injected. AT does not call production reachability bootstrap, endpoint bind or admission itself.

## One-slot controller handoff

AT materializes one one-shot publisher backed by `std::sync::mpsc::sync_channel(1)` or equivalent bounded storage.

The publisher must be consumed when publishing so a lane cannot publish multiple controllers through the same handoff authority.

If the process-side receiver exists, the exact `RemoteSessionSupervisorShutdownController` moves to process ownership.

If the receiver has already been dropped, the failed send must return the exact controller to the remote lane, which must immediately request orderly supervisor shutdown through that controller. The helper must not fabricate a replacement controller, abort the thread, close the endpoint out of AN ordering, or detach the lifecycle.

## Finalization semantics

The process owner finalizer is consuming and must:

1. wait for the one controller or sender disconnection;
2. when the controller arrives, request orderly remote supervisor shutdown exactly through the existing AP controller;
3. when the sender disconnects before a controller is published, fabricate no shutdown authority;
4. explicitly join the exact one remote OS thread;
5. reduce abnormal join/panic to a bounded classification without exposing panic payload, thread ID, OS task ID or backtrace;
6. return only bounded secondary evidence; it does not replace the local terminal reason.

Blocking while an in-flight provider/bootstrap or endpoint-startup transaction reaches its existing terminal result is allowed. AT introduces no hard cancellation boundary or deadline.

## No remote failure -> local shutdown coupling

AT does not call `LocalLinuxRuntimeShutdownHandle::request_shutdown_and_wake()` because of remote startup, provider, endpoint, worker or join failure.

The existing local shutdown handle remains dedicated to local runtime control. Existing local readiness and terminal classification remain primary and unchanged.

## Single signal owner preserved

The remote process-lifecycle module creates no `SignalFd`, installs no signal handler and unblocks no SIGTERM/SIGINT signal.

The remote lane is started only through the future companion-start seam after the existing signal source has already been created and local lifecycle assembly has succeeded. Therefore any future thread spawned there inherits the already-blocked termination mask while the existing local signal-aware path remains sole process termination interpreter.

AT itself does not production-wire the thread start seam.

## Single executor/runtime invariant preserved

AT creates no Tokio runtime and exposes no Tokio `Runtime` or `Handle`.

Future injected remote-lane work remains bound by AQ/AR:

`one RemoteSessionExecutorRuntime -> reachability bootstrap -> same executor into AP endpoint startup -> same executor for endpoint/session lifecycle`.

Forbidden remain:

- a second Tokio runtime;
- `rt-multi-thread`;
- generic `block_on`;
- detached OS thread/task;
- hard task abort;
- retry/rebootstrap/rebind/reconnect/replacement.

## Production activation remains absent

AT does not modify `linux_bootstrap.rs` or `main.rs` and does not select or supply:

- a production bind address;
- production expected-device requests;
- production capability/session-authentication inputs;
- production reachability-provider invocation;
- production endpoint bind invocation;
- remote readiness publication;
- service-manager readiness;
- systemd/host mutation;
- deployment/restart.

`SocketAddr` remains transient endpoint configuration only and never identity or authorization evidence.

## Focused non-production proof

AT tests must remain deterministic and non-networking. They should prove:

1. no-companion entry point preserves existing behavior;
2. companion finalizer runs after explicit local listener/socket cleanup;
3. companion finalizer runs before exact signal-mask restoration;
4. one injected fake lane is join-owned and never detached;
5. controller-before-finalization receives exactly one orderly request before join;
6. finalization-before-controller waits until controller publication or lane termination;
7. sender disconnect before publication fabricates no controller;
8. receiver drop returns exact controller to the lane-side publish helper and causes one orderly request;
9. abnormal join is reduced to bounded panic classification without panic payload exposure;
10. no remote failure invokes local shutdown;
11. no production credentials/provider I/O or remote endpoint bind occurs in tests.

Private generic helpers and fake controller types may be used to prove ownership behavior without constructing production remote state.

## Identity/security invariants preserved

AT changes no established identity boundary:

- `DeviceId` / authenticated PRW session identity remains logical identity;
- `TransportIdentity` remains lower-transport certificate identity only;
- IP/socket remains transient endpoint/configuration only;
- `SessionId` remains authentication correlation only;
- thread/runtime/controller/channel/join/endpoint identifiers remain implementation details only;
- reachability authority possession remains a capability-admission prerequisite, not user identity;
- endpoint startup and thread join are not authorization/readiness evidence.

Protected requests continue to use fresh current registry/current transport/current policy evaluation.

## Stop conditions

AT must stop rather than widen scope if implementation requires:

- `linux_bootstrap.rs` or `main.rs` production remote wiring;
- changing local terminal reason or exit-code semantics;
- making remote startup a prerequisite for local IPC `Ready`;
- another signal source/handler;
- signal-mask restoration before remote join;
- a detached or hard-cancelled remote lane;
- a second Tokio runtime;
- production bind-address or expected-device sources;
- retry/reconnect/rebootstrap;
- systemd/host/deployment mutation.

## Validation gate

C03e-AT may claim:

`C03E_AT_SIGNAL_AWARE_REMOTE_CAPABILITY_THREAD_JOIN_HANDOFF_SOURCE_MATERIALIZED`

only after:

1. exact AS ancestry/head/tree is reverified;
2. AS -> AT compare proves exactly the four bounded paths and no other mutation;
3. focused tests prove cleanup -> companion finalization -> signal-mask restoration ordering and one-thread/controller ownership without production I/O;
4. canonical Rust validation on the exact final AT head reaches terminal FULL PASS;
5. any actually-triggered Android validation reaches terminal verdict before closure;
6. disposable workflows are classified by actual terminal state;
7. immutable Drive evidence is uploaded and read back byte-exact;
8. rolling Drive evidence is appended with exact predecessor-prefix preservation;
9. the AT PR remains draft/open/unmerged and moves to `Status: CLOSED` only after evidence closeout.