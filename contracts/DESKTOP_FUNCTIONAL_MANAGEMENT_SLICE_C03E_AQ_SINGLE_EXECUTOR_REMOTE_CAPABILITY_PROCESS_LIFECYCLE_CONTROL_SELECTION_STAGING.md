# Phase 152 C03e-AQ — Single-Executor Remote Capability Process-Lifecycle Control Selection Staging

Status: STAGED

Target gate:

`C03E_AQ_SINGLE_EXECUTOR_REMOTE_CAPABILITY_PROCESS_LIFECYCLE_CONTROL_SELECTED`

## Exact predecessor

Canonical predecessor is closed C03e-AP:

- branch: `phase-152-c03e-ap-remote-endpoint-startup-supervisor-shutdown-control-source-materialization-staging`
- head: `aa8ba849178ddf59095df6557bea28e582995207`
- tree: `3aa223e760007240844d3f8a1ff23166215b83e1`
- gate: `C03E_AP_REMOTE_ENDPOINT_STARTUP_SUPERVISOR_SHUTDOWN_CONTROL_SOURCE_MATERIALIZED`

C03e-AQ selects only the next Agent-internal process-lifecycle control boundary needed to compose the already-existing reachability-authority bootstrap, the already-private current-thread remote executor, and the closed AP endpoint lifecycle without broadening local Agent readiness or creating a second async runtime.

AQ is documentation/selection only. It does not invoke reachability bootstrap, bind a remote endpoint, materialize Rust source, modify `main.rs`, publish readiness, consume production process signals differently, provision expected-device requests, mutate systemd/host state, deploy, or merge.

## Exact bounded checkpoint scope

The final AP -> AQ diff must contain exactly this one docs-only contract path.

No Rust source, Cargo manifest, lockfile, workflow, Android application, remote bridge, remote transport implementation, local Linux runtime implementation, Agent binary, readiness/status implementation, systemd unit, host configuration, credential payload, deployment state, or merge mutation is selected by AQ itself.

## Authoritative readiness predecessor: C02f-CH

AQ explicitly preserves closed C02f-CH:

`C02F_CH_AGENT_REACHABILITY_AUTHORITY_RUNTIME_READINESS_ORDERING_SELECTED`

CH selected reachability-authority bootstrap as a prerequisite for remote/reachability admission, **not** as a prerequisite for the base local Agent process bootstrap or the existing local IPC `Ready` state.

Therefore AQ locks these consequences:

1. existing local Agent startup and local IPC readiness remain semantically independent of reachability-provider availability;
2. authority bootstrap failure keeps the remote/reachability capability not admitted but does not retroactively invalidate the existing local `Ready` state;
3. no remote surface requiring the authority may be advertised, admitted, or used before authority bootstrap succeeds;
4. no retry/reconnect/background policy from CH is inherited implicitly — AQ must explicitly select any new long-lived execution ownership it needs;
5. remote capability health is not encoded into `LocalAgentRuntimeState` by this checkpoint.

AQ therefore rejects any design in which `linux_bootstrap::run()` must synchronously complete provider bootstrap before the validated local Agent runtime becomes available.

## Existing authoritative building blocks

AQ reuses without redesign:

1. `bootstrap_and_admit_reachability_live_owner_authority_from_systemd_credentials()`, the existing async fail-closed Agent admission seam;
2. `ReachabilityAuthorityRuntimeOwner`, the existing opaque Agent-owned lifetime boundary for one admitted authority;
3. `RemoteSessionExecutorRuntime`, the existing non-cloneable Tokio current-thread runtime owner built with I/O/time drivers;
4. `RemoteSessionEndpointLifecycleRuntime`, the closed AP owner for one executor + one authority-gated real remote endpoint + one private supervisor-shutdown signal;
5. `RemoteSessionSupervisorShutdownController`, the existing non-cloneable explicit authority for orderly AP/AL/AN supervisor shutdown;
6. the existing C03e-AN lifecycle ordering: supervisor return -> whole endpoint close once -> endpoint `wait_idle()` on the same private runtime;
7. `LocalLinuxTerminationSignalSource`, which already owns the process SIGTERM/SIGINT `SignalFd` path;
8. `LocalLinuxRuntimeShutdownHandle`, which preserves monotonic local shutdown state-before-wake ordering;
9. the Phase 101/102 narrow `linux_bootstrap` facade and thin `main.rs` boundary.

AQ does not replace these owners or duplicate their state machines.

## Integration blocker discovered after AP

The closed source surfaces expose a concrete ordering constraint:

- reachability authority bootstrap is async and performs provider network I/O;
- AP endpoint startup requires an already-materialized `ReachabilityAuthorityRuntimeOwner`;
- AP's existing public constructor constructs `RemoteSessionExecutorRuntime` only after that authority owner is already available.

A future executable integration therefore cannot correctly drive the async authority bootstrap by constructing one temporary Tokio runtime and then construct AP's second Tokio runtime afterward.

That would violate the already-selected single private current-thread runtime model.

AQ resolves this integration constraint by selecting **one remote-capability executor for both authority bootstrap and the complete remote endpoint lifecycle**.

This is an integration extension only; it does not reopen or invalidate AP's closed constructor semantics.

## Selected single-executor ownership

A future source-materialization checkpoint may introduce a narrow Agent-internal process composition equivalent in responsibility to one remote-capability lane owning exactly:

- one `RemoteSessionExecutorRuntime`;
- at most one successfully admitted `ReachabilityAuthorityRuntimeOwner` during startup transition;
- at most one AP `RemoteSessionEndpointLifecycleRuntime` after endpoint startup succeeds;
- the one remote lifecycle drive and its terminal result/evidence.

There is never one runtime for bootstrap plus another runtime for endpoint/session work.

The raw Tokio `Runtime` and `Handle` remain private and are not returned to the process orchestration layer.

## Domain-specific authority-bootstrap drive

A future source tranche may add one domain-specific method to `RemoteSessionExecutorRuntime`, equivalent in responsibility to:

`bootstrap_reachability_authority_from_systemd_credentials()`

Its only purpose is to use the already-owned private current-thread runtime to drive the existing async Agent admission seam to one terminal result and convert successful admission immediately into `ReachabilityAuthorityRuntimeOwner`.

The method must:

- call the existing fixed-credential reachability admission path exactly once;
- preserve the existing bounded custody/provider-bootstrap error unchanged or in one bounded wrapper;
- return no raw provider client, store, Tokio handle, future driver, or secret material;
- perform no retry/backoff/reconnect loop;
- perform no endpoint bind;
- publish no readiness;
- execute no R1-R4 effect.

AQ does **not** select a generic `block_on`, generic future executor, public runtime handle, or crate-wide async execution service.

## Reusing the same executor in AP startup

Because the one executor must exist before authority bootstrap, a future source tranche may add a narrow Agent-internal AP startup path equivalent in responsibility to:

`bind_with_executor_from_systemd_credentials(executor, authority_owner, bind_addr)`

The exact Rust name is not locked by AQ; the responsibility is.

That path must:

1. receive the already-constructed `RemoteSessionExecutorRuntime` by ownership;
2. receive one successfully admitted `ReachabilityAuthorityRuntimeOwner`;
3. attempt the existing fixed mesh-credential/TLS/socket bind exactly once;
4. construct the existing AP supervisor-shutdown pair only after bind success;
5. return the same AP lifecycle owner responsibility plus the existing non-cloneable controller.

The original AP public constructor may remain byte-stable or delegate through a common private helper. AQ does not authorize semantic weakening of its executor-before-bind rule.

No second executor may be constructed on the process-integration path.

## Startup failure semantics under the single-executor path

### Executor construction failure

If the one remote executor cannot be constructed:

- no reachability credential read occurs;
- no provider bootstrap occurs;
- no mesh credential read occurs;
- no remote endpoint is bound;
- the remote capability remains unavailable;
- existing local Agent lifecycle/readiness semantics remain unaffected.

No retry or fallback executor is selected.

### Reachability authority bootstrap failure

If authority custody/provider bootstrap fails:

- no authority admission token/runtime owner is fabricated;
- no AP endpoint bind is attempted;
- no remote readiness is published;
- no fallback authority is constructed;
- no plaintext/weaker provider path is used;
- the remote capability lane terminates with bounded internal failure evidence;
- the already-running local Agent may remain available for its existing local IPC surface, exactly as C02f-CH requires.

AQ does not select remote bootstrap failure as a process-wide fail-stop cause.

### Endpoint startup failure

If authority bootstrap succeeds but AP endpoint startup fails:

- existing AP/transport error and authority-custody semantics remain authoritative;
- no second bind, alternate address, retry, rebootstrap or reconnect occurs;
- the remote capability lane terminates unavailable;
- local IPC `Ready` is not reclassified by this checkpoint.

## Selected process-concurrency boundary

A future source-materialization checkpoint may run the remote capability lane on **exactly one joinable OS thread** after the existing local Linux signal source has been established.

The thread is selected because the validated local Linux runtime is synchronous/blocking while the remote lane must continuously drive one private Tokio current-thread runtime.

The thread must be:

- explicitly owned by the Agent process composition;
- joinable;
- never detached;
- never converted into an identity/authentication principal;
- never used as a second SIGTERM/SIGINT signal owner;
- bounded to the one remote capability lifecycle.

AQ does not select a thread pool, Tokio multi-thread runtime, detached worker, background retry worker, or replacement thread after failure.

## Local readiness ordering

The remote lane must not become a prerequisite for current local IPC `Ready`.

The selected future sequence is conceptually:

```text
existing process/device-identity preflight
    -> existing local SignalFd setup
    -> existing local lifecycle/socket assembly
    -> current local runtime remains locally Ready
    -> start one separately-owned remote capability lane
         -> construct one remote executor
         -> drive reachability authority bootstrap
         -> if successful, bind AP endpoint using the same executor
         -> if later separately supplied remote-admission inputs exist, drive AP lifecycle
```

Starting the remote lane after local lifecycle assembly does not mean that remote capability is ready. It only begins a separately gated capability admission attempt.

AQ selects no global/service readiness publication and no `sd_notify` behavior.

## Process signal ownership remains singular

The existing `LocalLinuxTerminationSignalSource` remains the sole SIGTERM/SIGINT owner.

AQ forbids the remote lane from:

- installing a signal handler;
- creating another `SignalFd`;
- unblocking SIGTERM/SIGINT for itself as an alternate owner;
- interpreting thread ID or signal delivery as PRW identity.

When the remote thread is created after local signal-mask installation, inheriting the blocked termination mask is expected process-lifecycle behavior. Signal interpretation remains on the existing local signal-aware path.

## Selected local-process termination -> remote shutdown direction

AQ selects only this cross-lifecycle shutdown direction:

**existing local process termination/lifecycle completion may request orderly shutdown of an already-started remote endpoint supervisor, then join the one remote lane.**

The existing AP `RemoteSessionSupervisorShutdownController` remains the only authority that makes the AP/AL supervisor-shutdown future ready after endpoint startup.

A future source tranche may use a bounded ownership handoff to make that one non-cloneable controller available to the process-lifecycle owner after AP startup succeeds.

No controller clone is selected.

If local termination is already requested before the AP controller exists, the process-lifecycle owner must retain that terminal intent and apply it once a controller becomes available, or observe that the remote lane terminated before endpoint startup. It must not create a second shutdown authority to bypass AP.

AQ does not authorize hard abort of an in-flight provider bootstrap or endpoint transaction merely to accelerate process termination. Cancellation semantics for such external I/O are not inferred where predecessors do not prove them.

## Join ordering and signal-mask restoration

A future source-materialization checkpoint must preserve explicit join ownership.

The preferred lifecycle ordering selected by AQ is:

1. existing local loop establishes its terminal reason;
2. local listener/session worker cleanup proceeds under the existing Linux runtime semantics;
3. if a remote AP controller exists, request orderly remote supervisor shutdown;
4. join the one remote capability thread;
5. only after the joined remote lane has relinquished its inherited blocked-signal context may the calling thread finalize exact prior signal-mask restoration and process-level terminal evidence.

If implementing this ordering requires a narrow below-`main.rs` lifecycle hook around the existing signal-aware runtime, that hook must be separately materialized and preserve the existing public facade/result semantics.

AQ explicitly forbids using `Drop` as fake evidence that remote join or endpoint idle completion occurred.

## Remote failure does not become local shutdown by implication

AQ does **not** select remote startup failure, provider loss, endpoint failure, admission failure, or remote worker failure as an automatic call to `LocalLinuxRuntimeShutdownHandle::request_shutdown_and_wake()`.

Doing so would couple external reachability availability to the existing local Agent lifetime and would contradict C02f-CH's selected local-readiness independence unless a later explicit degradation/restart policy checkpoint authorizes it.

The existing local shutdown handle remains available for its current local semantics and controlled tests. AQ does not repurpose it as remote failure propagation.

## Remote terminal evidence

A future source tranche may retain one bounded remote-lane terminal classification sufficient to distinguish at least:

- executor construction failure;
- reachability custody/provider-bootstrap failure;
- AP endpoint startup failure;
- AP lifecycle terminal return;
- abnormal OS-thread join/panic classification without exposing panic payloads or thread identity.

AQ does not select a new public log protocol, new process exit-code taxonomy, local status transition, or unbounded error object graph.

Remote terminal evidence must not overwrite the existing local runtime's primary terminal reason.

## Bind-address custody

The existing AP/C03e-C constructor receives an explicit caller-supplied `SocketAddr`.

AQ preserves that address strictly as endpoint configuration, never identity, authentication, authority, authorization or readiness evidence.

AQ does not select a production bind-address source or default.

In particular AQ does not authorize:

- wildcard/public/LAN binding by default;
- arbitrary CLI/environment bind widening;
- DNS-derived bind widening;
- firewall/NAT/route mutation;
- UDP forwarding or relay activation.

A concrete production bind-address source remains separately gated.

## Expected-device and capability inputs remain separately gated

AP's repeated real admission lifecycle still requires already-selected/injected bounded inputs, including the shared-current capability authority, session-authentication service, expected-device request receiver, timing inputs, dispatchers and callbacks.

AQ does not manufacture or select a production source for expected-device requests.

AQ does not select:

- discovery polling;
- registry watch tasks;
- controller-generated expected-device queues;
- remote dispatcher construction from untrusted input;
- automatic reconnection/replacement admission;
- parallel pre-auth transactions.

Until those inputs are separately materialized for a production process path, future AQ source proof must use injected/focused non-production helpers rather than activate a real executable remote admission loop.

## Identity and authorization invariants

AQ preserves all established boundaries:

- `DeviceId` / authenticated PRW session identity is logical identity;
- `TransportIdentity` is lower-transport certificate identity only;
- IP/socket address is transient endpoint/configuration data only;
- `SessionId` is authentication correlation only;
- thread/runtime/controller/channel/join/endpoint identifiers are implementation details only;
- reachability authority possession is a capability-admission prerequisite, not authenticated user identity;
- endpoint startup success is not capability authorization evidence.

Protected requests continue to use fresh current registry/current transport/current policy evaluation through the existing shared-current authority path.

## Runtime constraints

AQ locks the following for future source materialization.

Allowed:

- exactly one `RemoteSessionExecutorRuntime` for remote authority bootstrap and endpoint/session lifecycle;
- one joinable OS thread for the remote capability lane;
- private domain-specific executor methods only;
- existing AP/AN/AL orderly shutdown and endpoint idle sequencing.

Forbidden:

- a second Tokio runtime;
- `rt-multi-thread`;
- generic public or crate-public `block_on`;
- exposing Tokio `Runtime` or `Handle`;
- detached thread/task ownership;
- hard task abort;
- hard endpoint-drain deadline;
- retry/reconnect/rebootstrap/replacement loop;
- process signal duplication.

## Phase 101/102 thin-binary boundary preserved

AQ does not select direct remote orchestration in `main.rs`.

Any future source materialization must remain below the thin binary adapter, preferably by extending the existing narrow Linux bootstrap/process facade or an Agent-internal lifecycle composition called by that facade.

`main.rs` must not gain direct Tokio runtime construction, thread lifecycle mechanics, reachability provider calls, endpoint bind logic, signal-mask manipulation, remote worker state machines, or raw remote terminal objects merely because AQ exists.

## Focused future source-level proof

The next source-materialization checkpoint must be able to prove without production activation that:

- one executor is constructed before authority bootstrap and reused for the endpoint path;
- authority bootstrap failure performs no endpoint bind and does not alter local readiness;
- the process-integration path cannot construct a second executor;
- the AP same-executor startup path preserves executor-before-bind ordering;
- one remote lane is join-owned and not detached;
- the remote lane creates no second process signal owner;
- local termination intent is durable across the race where AP controller handoff has not yet occurred;
- an available AP controller receives at most the selected orderly shutdown request semantics;
- remote lane completion is joined explicitly;
- existing local terminal reason/readiness semantics remain primary and unchanged;
- bind address stays injected configuration with no wildcard/public default;
- no production expected-device producer is introduced.

Tests should use private injected helpers, compile-time shape checks and deterministic thread/control fakes where necessary. They must not read production reachability/mesh credentials, contact production providers, bind a production remote endpoint, mutate host state, or publish service readiness merely to prove ownership/ordering.

## Explicitly still absent

C03e-AQ does not select or materialize:

- Rust/source implementation;
- `main.rs` wiring;
- production reachability bootstrap invocation;
- production mesh endpoint bind invocation;
- production bind address;
- local or remote readiness publication;
- status degradation/recovery representation;
- automatic remote failure -> local process shutdown;
- expected-device request production/discovery;
- retry/backoff/reconnect/rebootstrap/replacement;
- authority health polling;
- quorum-loss policy;
- service-manager readiness;
- systemd unit/drop-in mutation;
- credential provisioning;
- host firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart;
- recovery/PRWF initialization;
- R1-R4 effect activation;
- merge.

## Stop conditions

Future source materialization must stop for re-selection instead of widening AQ if it discovers that correctness requires:

- more than one Tokio runtime;
- a generic runtime/handle exposure;
- making local IPC startup depend on provider/quorum availability;
- changing existing local `Ready` semantics;
- automatic remote failure -> process restart/shutdown policy;
- cancelling provider bootstrap without an already-proven cancellation contract;
- hard abort/deadline semantics;
- detached runtime/thread/task ownership;
- wildcard/public bind selection;
- production expected-device/discovery machinery;
- systemd/host/deployment mutation.

## Validation gate

AQ may claim:

`C03E_AQ_SINGLE_EXECUTOR_REMOTE_CAPABILITY_PROCESS_LIFECYCLE_CONTROL_SELECTED`

only after:

1. exact AP ancestry is reverified;
2. AP -> AQ compare proves exactly one documentation file addition and no source/manifest/lock/workflow mutation;
3. canonical repository validation on the exact final AQ head reaches its actual terminal verdicts;
4. Drive immutable evidence is written and read back exactly;
5. rolling Drive evidence is append-preserved byte-for-byte;
6. the AQ pull request remains draft/open/unmerged and its status changes to CLOSED only after evidence closeout.
