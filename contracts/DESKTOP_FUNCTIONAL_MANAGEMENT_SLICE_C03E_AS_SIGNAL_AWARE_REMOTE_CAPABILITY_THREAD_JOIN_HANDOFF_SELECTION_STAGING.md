# Phase 152 C03e-AS — Signal-Aware Remote Capability Thread/Join Handoff Selection Staging

Status: STAGED

Target gate:

`C03E_AS_SIGNAL_AWARE_REMOTE_CAPABILITY_THREAD_JOIN_HANDOFF_SELECTED`

## Exact predecessor

Canonical predecessor is closed C03e-AR:

- branch: `phase-152-c03e-ar-single-executor-reachability-bootstrap-endpoint-startup-source-materialization-staging`
- head: `0a41aa25d9ebf017ba6546ea76b787175ef23cff`
- tree: `b609aea19969934add06b4c93ab457daa6e08a0b`
- gate: `C03E_AR_SINGLE_EXECUTOR_REACHABILITY_BOOTSTRAP_ENDPOINT_STARTUP_SOURCE_MATERIALIZED`

C03e-AR materialized the AQ-selected same-executor reachability-authority bootstrap seam and the AP endpoint-startup seam that consumes the already-created executor. AR deliberately deferred the AQ-selected OS-thread/process-lifecycle handoff because that boundary intersects the existing Linux signal-mask restoration and explicit join ordering.

C03e-AS selects only that deferred control boundary. It is documentation/selection only. It does not materialize Rust source, modify `linux_bootstrap.rs` or `main.rs`, invoke production reachability bootstrap, bind a remote endpoint, publish readiness, alter SIGTERM/SIGINT ownership, choose a production bind address, create expected-device production inputs, mutate systemd/host state, deploy, or merge.

## Exact bounded checkpoint scope

The final AR -> AS diff must contain exactly this one docs-only contract path.

No Rust source, Cargo manifest, lockfile, workflow, Android application, bridge/transport/reachability implementation, local Linux runtime implementation, Agent binary, readiness/status implementation, systemd unit, host configuration, credential payload, deployment state, or merge mutation is selected by AS itself.

## Source audit that determines the handoff boundary

The exact AR source proves four existing ordering facts that AS treats as authoritative.

### 1. `main.rs` remains a thin binary adapter

The Linux `main.rs` path performs the existing device-identity custody preflight and then delegates to `prw_agent::linux_bootstrap::run()`.

AS therefore rejects direct remote executor construction, reachability provider calls, OS-thread ownership, remote controller channels, signal-mask manipulation, endpoint bind logic, or remote terminal state machines in `main.rs`.

### 2. `linux_bootstrap::run()` delegates to the existing signal-aware runtime

The current bootstrap facade builds the fixed local runtime inputs and calls `run_signal_aware_linux_production_runtime_from_env(...)`.

Its existing local terminal classification, cleanup mapping, signal-mask restoration evidence and process exit contract remain primary.

Remote capability failure is not selected as a new local startup failure or local terminal reason.

### 3. signal ownership is established before local lifecycle assembly

`run_signal_aware_linux_production_runtime_from_env(...)` creates the one existing `LocalLinuxTerminationSignalSource` before entering local lifecycle assembly.

Any future remote capability OS thread selected by AS must be created only after this source exists so the thread inherits the already-blocked SIGTERM/SIGINT mask. The remote lane must never create another `SignalFd`, install another process signal handler, unblock termination signals as an alternate owner, or interpret thread/signal identifiers as PRW identity.

The existing local signal-aware path remains the sole SIGTERM/SIGINT interpretation authority.

### 4. there is an exact cleanup-before-mask-restore seam

`with_local_linux_production_lifecycle_from_env(...)` executes its callback while the local instance lock/listener/wake/capacity/control resources are live, then explicitly finishes listener/socket cleanup before returning `LocalLinuxProductionLifecycleExecution<R>`.

Only after that lifecycle execution returns does `run_signal_aware_linux_production_runtime_from_env(...)` read the cleanup result, recover the loop exit, and call `signal_source.restore()`.

Therefore the exact selected ordering seam is:

```text
local signal source established
    -> local lifecycle assembled
    -> local runtime loop runs
    -> local terminal reason established
    -> local workers cancelled/joined by existing loop semantics
    -> local listener/socket cleanup finishes explicitly
    -> [AS-selected companion finalization seam]
         -> request orderly remote shutdown if controller exists
         -> join the one remote capability OS thread
    -> exact prior calling-thread signal mask restored
    -> existing local terminal report returned
```

AS does not move, duplicate or weaken the existing local cleanup or signal-mask restoration logic.

## Selected below-`main.rs` lifecycle companion hook

A future source-materialization checkpoint may add one narrow Agent-internal generic companion hook around the existing signal-aware lifecycle, equivalent in responsibility to:

1. a start callback that executes after local lifecycle assembly and before the first local readiness wait and may return one owned companion value `O`;
2. the existing local runtime loop runs unchanged;
3. the companion value is carried out through the local lifecycle callback result without being detached or dropped as cleanup evidence;
4. existing listener/socket cleanup finishes and its explicit result is retained;
5. one finalizer callback consumes `O` after that local cleanup and before `signal_source.restore()`;
6. only after the finalizer returns may exact prior signal-mask restoration proceed.

The exact Rust names are not locked by AS. The ordering and ownership responsibilities are locked.

The existing public signal-aware runtime entry point should remain behaviorally stable by delegating through the new internal composition with a unit/no-op companion when no external process companion is selected.

AS does not authorize a generic arbitrary lifecycle plugin framework, public callback registry, dynamic hook list, trait-object service container, or unbounded extension surface.

## Local readiness remains independent

The companion start seam occurs only after successful local lifecycle assembly. It must not become a prerequisite for existing local IPC `Ready` semantics.

If future remote companion creation or thread spawn fails:

- the remote capability remains unavailable;
- no remote readiness is published;
- the current local Agent runtime remains eligible to continue under its existing semantics;
- no automatic local shutdown/restart is requested;
- no new `LinuxAgentBootstrapStartKind` is selected merely for remote failure.

This preserves C02f-CH and C03e-AQ.

## Selected remote capability process owner

A future source tranche may introduce one Agent-internal non-cloneable remote process-lifecycle owner equivalent in responsibility to exactly:

- at most one joinable OS-thread `JoinHandle` for the remote capability lane;
- one bounded ownership handoff receiver for the existing non-cloneable `RemoteSessionSupervisorShutdownController` after endpoint startup succeeds;
- one bounded remote-lane terminal classification after explicit join;
- no raw Tokio runtime/handle and no second signal owner.

The owner is process lifecycle state only. It is never logical identity, authentication evidence, transport identity, authorization, reachability authority, or readiness evidence.

## Exactly one joinable remote OS thread

The remote lane selected by AQ and refined by AS must use exactly one explicitly owned joinable OS thread.

Allowed responsibility inside that one lane is bounded to the remote capability lifecycle:

```text
construct one RemoteSessionExecutorRuntime
    -> same executor drives one reachability-authority bootstrap attempt
    -> on success, same executor is moved into AR/AP endpoint startup
    -> on success, move the one shutdown controller to process ownership
    -> drive the same endpoint lifecycle on the same private current-thread executor
    -> return one bounded terminal classification
```

The lane must not:

- detach;
- create a thread pool;
- create or use a Tokio multi-thread runtime;
- create a second Tokio runtime;
- replace itself after failure;
- install/read process termination signals;
- retry/rebootstrap/rebind/reconnect;
- expose thread ID as identity or evidence.

Thread creation failure must be bounded as remote-capability unavailability and must not fabricate a remote controller or convert into local readiness failure by implication.

## Selected controller ownership handoff

After successful AR/AP endpoint startup, the exact existing `RemoteSessionSupervisorShutdownController` must move once from the remote lane to the process-lifecycle owner through one bounded ownership handoff.

AS selects these invariants:

- the controller is not cloned;
- the handoff has bounded storage, sufficient for at most one controller;
- the remote thread must perform the handoff before entering the long-lived AP endpoint lifecycle drive;
- the process owner retains the receiving side until local terminal finalization;
- a controller value is never used as identity/authentication/readiness evidence.

A future implementation may use a one-slot synchronous ownership channel or an equivalently bounded primitive. AS does not require a particular standard-library type if the exact ownership and capacity rules are preserved.

## Durable local termination intent without a second shutdown authority

The AS finalizer begins only after the existing local runtime has already established its terminal reason and local listener/socket cleanup has finished. That fact is the durable process termination intent.

If the AP shutdown controller was handed off earlier, the finalizer requests orderly shutdown through that exact controller and then joins the remote lane.

If local termination reaches the finalizer before the controller handoff exists, the process owner must wait for one of two bounded outcomes:

1. the exact controller becomes available, in which case it requests orderly shutdown once and then joins; or
2. the remote lane terminates before successful endpoint startup, in which case no controller is fabricated and the same lane is joined.

AS does not select an auxiliary atomic shutdown authority, a second remote shutdown token, a Tokio abort handle, thread cancellation, signal injection into the remote thread, or hard endpoint-drain deadline to bypass this race.

Waiting for an in-flight provider/bootstrap or endpoint-startup transaction to reach its existing terminal result is allowed; predecessor checkpoints do not prove a safe hard-cancellation boundary for those external I/O operations.

## Handoff failure cannot orphan a live endpoint

If endpoint startup succeeds but moving the controller to process ownership fails because the process-side receiver no longer exists, the remote lane must recover ownership of that exact controller from the failed handoff, request orderly supervisor shutdown through it, and continue driving the same AP lifecycle to terminal completion before returning.

It must not:

- drop the controller and leave a live endpoint unsupervised;
- detach the endpoint lifecycle;
- hard-abort the Tokio runtime;
- close the endpoint out of AN ordering;
- create a replacement controller.

Existing AP/AN/AL shutdown ordering remains authoritative.

## Join ordering is explicit evidence

The process finalizer must explicitly join the exact remote lane before returning control to signal-mask restoration.

`Drop` is not evidence of thread join, remote supervisor return, endpoint close, or endpoint idle completion.

A successful finalizer path therefore proves:

1. any available AP controller received the selected orderly request;
2. the exact one remote OS thread reached terminal completion;
3. the join result was observed explicitly;
4. only then may `signal_source.restore()` execute.

Abnormal thread completion/panic may be reduced to one bounded remote terminal classification. Panic payloads, thread IDs, OS task IDs, Tokio task IDs and backtraces are not selected as public/process identity or unbounded terminal evidence.

## Local terminal reason remains primary

Remote startup failure, remote endpoint failure, remote admission failure, remote worker failure, or abnormal remote-thread join does not overwrite the existing local `LocalLinuxSignalAwareRuntimeTerminalReason`.

AS does not select a new process exit-code taxonomy or automatic mapping from remote terminal state to local failure.

A future source tranche may retain bounded secondary remote terminal evidence for audit/testing, but existing `linux_bootstrap` report semantics remain authoritative until a separate explicit checkpoint selects any external reporting extension.

## Existing local programmatic shutdown handle is not repurposed

`LocalLinuxRuntimeShutdownHandle` remains dedicated to its existing local runtime control semantics.

AS does not permit remote provider loss, endpoint startup failure, remote worker failure or remote-thread failure to call `request_shutdown_and_wake()` automatically.

The remote lane does not need ownership of the local shutdown handle merely because both lifecycles exist in one process.

## Production bind-address source remains absent

AR's same-executor endpoint startup still requires an explicit `SocketAddr` supplied by a future process consumer.

AS does not select where a production address comes from.

No wildcard/public/LAN default, arbitrary environment/CLI widening, DNS-derived widening, firewall/NAT/route mutation, UDP relay, TUN/TAP, or forwarding behavior is authorized.

Socket address remains transient endpoint configuration only, never logical identity or authorization evidence.

## Production expected-device inputs remain absent

The AP repeated real-admission lifecycle still requires bounded expected-device/admission inputs, capability authority, session authentication, timing, dispatcher construction and callbacks.

AS does not select a production producer for those inputs and therefore does not authorize executable activation of the full remote admission lifecycle.

A future source proof for the thread/join control must use injected private helpers/fakes or deterministic bounded test lanes. Tests must not read production reachability/mesh credentials, contact providers, bind a production remote endpoint, or manufacture production expected-device discovery.

## Focused future source proof

The next source-materialization checkpoint should be able to prove without production activation that:

1. the signal source is established before the companion/thread start seam;
2. companion start occurs only after successful local lifecycle assembly;
3. local listener/socket cleanup finishes before companion finalization;
4. companion finalization finishes before exact prior signal-mask restoration;
5. the existing no-companion signal-aware entry point preserves its terminal/cleanup/mask-restoration behavior;
6. one fake remote lane is explicitly join-owned and never detached;
7. a pre-existing controller handoff receives one orderly shutdown request before join;
8. local terminal intent that occurs before controller availability waits for the controller-or-lane-terminal outcome without fabricating authority;
9. failed controller handoff returns the exact controller to the remote lane for orderly shutdown;
10. abnormal thread completion is bounded without panic/thread identity exposure;
11. remote failure does not request local shutdown or alter local readiness;
12. no second signal source, Tokio runtime, retry worker, production bind, or expected-device producer appears.

Private generic composition helpers, deterministic standard-library channels, fake controllers and compile-time shape checks are preferred.

## Identity/security invariants preserved

AS changes no established identity boundary:

- `DeviceId` / authenticated PRW session identity remains logical identity;
- `TransportIdentity` remains lower-transport certificate identity only;
- IP/socket remains transient endpoint/configuration data only;
- `SessionId` remains authentication correlation only;
- thread/runtime/controller/channel/join/endpoint identifiers remain implementation details only;
- reachability authority possession is a capability-admission prerequisite, not user identity;
- endpoint startup success and thread join are not authorization/readiness evidence.

Protected request authorization remains fresh-current through existing registry/transport/policy evaluation.

## Explicitly absent

C03e-AS does not authorize or materialize:

- Rust/source implementation;
- `main.rs` changes;
- `linux_bootstrap.rs` production remote activation;
- production invocation of reachability bootstrap or endpoint bind;
- production bind-address selection;
- production expected-device/discovery machinery;
- new process signal handlers or a second `SignalFd`;
- local or remote readiness publication;
- remote failure -> local shutdown/restart coupling;
- retry/backoff/reconnect/rebootstrap/replacement;
- hard thread/task abort or hard endpoint-drain deadline;
- second Tokio runtime or `rt-multi-thread`;
- generic `block_on`/Runtime/Handle exposure;
- systemd unit/drop-in mutation;
- host firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart;
- recovery/PRWF initialization;
- R1-R4 activation;
- merge.

## Stop conditions

AS must stop for another explicit selection rather than widen scope if future source materialization requires:

- changing the existing local terminal reason or exit-code contract;
- making remote startup a prerequisite for local IPC `Ready`;
- changing SIGTERM/SIGINT ownership;
- restoring the signal mask before remote thread join;
- detaching or hard-cancelling the remote lane;
- creating a second Tokio runtime;
- production bind-address or expected-device input sources;
- automatic retry/reconnect/rebootstrap;
- `main.rs` orchestration growth;
- systemd/host/deployment mutation.

## Validation gate

C03e-AS may claim:

`C03E_AS_SIGNAL_AWARE_REMOTE_CAPABILITY_THREAD_JOIN_HANDOFF_SELECTED`

only after:

1. exact AR ancestry/head/tree is reverified;
2. AR -> AS compare proves exactly this one docs-only path;
3. the selected ordering is audited against exact AR `main.rs`, `linux_bootstrap.rs`, `linux_signal_aware_runtime.rs`, `linux_production_lifecycle.rs`, and remote endpoint lifecycle source;
4. canonical Rust validation on the exact final AS head reaches terminal PASS;
5. any actually-triggered Android validation reaches its terminal verdict before closure;
6. disposable workflows are classified by their actual terminal states;
7. immutable Drive evidence is uploaded and read back byte-exact;
8. rolling Drive evidence is appended with exact predecessor-prefix preservation;
9. the AS PR remains draft/open/unmerged and moves to `Status: CLOSED` only after evidence closeout.