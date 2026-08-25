# Phase 152 C03e-AR — Single-Executor Reachability Bootstrap + Endpoint Startup Source Materialization Staging

Status: STAGED

Target gate:

`C03E_AR_SINGLE_EXECUTOR_REACHABILITY_BOOTSTRAP_ENDPOINT_STARTUP_SOURCE_MATERIALIZED`

## Exact predecessor

Canonical predecessor is closed C03e-AQ:

- branch: `phase-152-c03e-aq-single-executor-remote-capability-process-lifecycle-control-selection-staging`
- head: `8c84dc477a06c15cac3cc40dc9825423141fbcd5`
- tree: `5cfec42f79e340327814a56bde5d95722c184a9e`
- gate: `C03E_AQ_SINGLE_EXECUTOR_REMOTE_CAPABILITY_PROCESS_LIFECYCLE_CONTROL_SELECTED`

AQ selected one private current-thread executor for reachability-authority bootstrap and the complete remote endpoint/session lifecycle. AR materializes only the first source tranche required by that selection: the domain-specific same-executor reachability bootstrap seam and an AP endpoint-startup seam that consumes the already-created executor by ownership.

AR deliberately does **not** materialize the AQ-selected OS-thread/process-lifecycle handoff. That remains a separately bounded checkpoint because it intersects existing Linux signal-mask restoration and join ownership.

## Exact bounded source scope

The final AQ -> AR diff is restricted to exactly these three paths:

1. this contract;
2. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`;
3. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`.

No Cargo manifest, lockfile, workflow, Android application, bridge crate, transport implementation, reachability implementation, local Linux runtime, `linux_bootstrap.rs`, `main.rs`, readiness/status implementation, systemd unit, host configuration, deployment state or merge mutation is allowed in AR.

## Materialized executor bootstrap seam

AR may add exactly one Agent-internal domain-specific method on the existing `RemoteSessionExecutorRuntime` equivalent in responsibility to:

`bootstrap_reachability_authority_from_systemd_credentials(&mut self)`

The method must:

1. use the already-owned private Tokio current-thread `Runtime`;
2. drive exactly one call to the existing async `bootstrap_and_admit_reachability_live_owner_authority_from_systemd_credentials()` seam;
3. map success immediately through `ReachabilityAuthorityRuntimeOwner::new`;
4. return the existing `ReachabilityAuthorityCustodyBootstrapError` unchanged on custody/provider failure;
5. retain no provider client, secret bytes, future, task, runtime handle or Tokio join handle;
6. perform no retry/backoff/rebootstrap/reconnect;
7. perform no endpoint bind;
8. publish no readiness and mutate no local Agent lifecycle state.

The method is source-only in AR and must not be invoked by a production executable path.

AR forbids introducing a generic `block_on`, generic future driver, public `Runtime`, public `Handle`, second Tokio runtime or `rt-multi-thread` runtime.

## Materialized AP same-executor endpoint-startup seam

AR may add exactly one Agent-internal AP endpoint-startup seam equivalent in responsibility to:

`bind_with_executor_from_systemd_credentials(executor, authority_owner, bind_addr)`

The exact Rust visibility should remain no wider than required for a future Agent-internal process-composition consumer.

The seam must:

1. consume the already-created `RemoteSessionExecutorRuntime` by ownership;
2. consume one already-admitted `ReachabilityAuthorityRuntimeOwner`;
3. attempt the existing `AgentRemoteTransportRuntime::bind_from_systemd_credentials(...)` transaction exactly once;
4. retain/recover the exact reachability-authority owner on bind failure through the existing AP startup-failure shape;
5. classify that failure as the existing `RemoteSessionEndpointLifecycleStartupError::Transport`;
6. create the existing remote-supervisor shutdown pair only after endpoint bind succeeds;
7. return the same `RemoteSessionEndpointLifecycleRuntime` responsibility and existing non-cloneable `RemoteSessionSupervisorShutdownController`;
8. move the exact supplied executor into the resulting AP lifecycle owner without constructing a replacement executor.

The seam must not read reachability credentials, bootstrap provider authority, create a second executor, retry bind, choose an alternate address, publish readiness or begin remote admission.

## Existing AP constructor remains authoritative

The already-public AP constructor:

`RemoteSessionEndpointLifecycleRuntime::bind_from_systemd_credentials(authority_owner, bind_addr)`

must retain its closed AP semantics:

- executor construction occurs before endpoint bind;
- executor-construction failure prevents bind and returns exact authority custody;
- endpoint-bind failure returns exact authority custody;
- one bind attempt only;
- one private shutdown pair only after bind success;
- no retry/fallback/rebootstrap.

AR may share a private bind-composition helper between the original AP constructor and the new same-executor seam, but it must not weaken or reorder the AP constructor contract.

## Same-executor invariant

The intended future integration sequence after AR is:

```text
RemoteSessionExecutorRuntime::new()
    -> same executor drives reachability authority bootstrap
    -> success becomes ReachabilityAuthorityRuntimeOwner
    -> exact same executor is moved into AP same-executor endpoint startup
    -> exact same executor remains inside RemoteSessionEndpointLifecycleRuntime
    -> later AP/AN/AL lifecycle drives use that same executor
```

At no point may the process-integration path construct a temporary bootstrap runtime and then construct a second endpoint/session runtime.

The executor is ownership state, never PRW identity, authentication evidence, policy authority or readiness evidence.

## Failure semantics

### Executor construction failure

Executor construction remains outside the new domain-specific bootstrap method. If `RemoteSessionExecutorRuntime::new()` fails:

- no reachability credential read occurs;
- no provider network I/O occurs;
- no mesh credential read occurs;
- no endpoint bind occurs;
- no authority owner exists;
- no retry/fallback runtime is selected.

### Reachability bootstrap failure

If the same-executor bootstrap method returns an existing custody/provider error:

- no authority owner is fabricated;
- no endpoint bind occurs merely because AR exists;
- no remote readiness is published;
- local IPC `Ready` is not reclassified;
- no automatic local shutdown/restart is requested;
- no retry/rebootstrap/reconnect is performed.

This preserves C02f-CH and C03e-AQ.

### Endpoint bind failure

If the same-executor AP startup seam fails:

- the exact authority owner remains recoverable through `RemoteSessionEndpointLifecycleStartupFailure`;
- the failure remains `Transport(...)`;
- no second bind or alternate address is attempted;
- no replacement executor is constructed;
- no remote-supervisor shutdown pair is treated as successfully published;
- no local readiness/lifecycle state changes.

The supplied executor is allowed to be dropped on failed bind; AR does not invent executor-recovery API where AQ did not select it.

## Bind-address custody

`SocketAddr` remains caller-supplied endpoint configuration only.

AR does not select or materialize:

- a production bind-address source;
- wildcard/public/LAN defaults;
- arbitrary CLI/environment bind widening;
- DNS-derived widening;
- firewall/NAT/route mutation;
- UDP/relay/TUN/TAP activation.

No socket address may become logical identity, authentication, authorization, authority or readiness evidence.

## Readiness and local Agent isolation

AR preserves C02f-CH exactly:

- the existing local Agent lifecycle and local IPC `Ready` semantics do not depend on provider bootstrap availability;
- remote authority/bootstrap failure leaves the remote capability unavailable/not admitted;
- AR adds no local status transition and no service-manager readiness behavior;
- AR does not call `LocalLinuxRuntimeShutdownHandle` on remote failure.

## Process-lifecycle boundary intentionally deferred

AR does not materialize:

- an OS thread;
- a `JoinHandle` for a remote process lane;
- cross-thread controller handoff;
- local termination-intent latching;
- signal-mask restoration hooks;
- process signal consumption;
- remote failure -> local shutdown propagation;
- executable process composition.

Those responsibilities remain separately gated after AR so they can be reviewed against the existing single `SignalFd` owner and exact Linux cleanup/mask-restoration ordering.

## Expected-device inputs still absent

AR does not create a production source for `RemoteSessionExpectedDeviceAdmissionRequest`.

It does not add discovery polling, registry watch loops, controller queues, automatic remote-session creation, parallel pre-authentication attempts, retry/reconnect or replacement admission.

Consequently AR does not invoke the full AP repeated real-admission endpoint lifecycle from a production path.

## Focused non-networking source proof

AR validation should prove without production credential/network activation that:

1. the new executor bootstrap method has the exact bounded return shape;
2. the method remains domain-specific and keeps the raw Tokio runtime private;
3. the AP same-executor seam accepts one `RemoteSessionExecutorRuntime` by ownership;
4. a private composition helper preserves the supplied executor unchanged on successful fake bind;
5. fake bind failure occurs exactly once and retains the exact fake authority value;
6. the existing AP constructor still proves executor-before-bind and bind suppression after executor failure;
7. no test invokes production reachability credentials/provider I/O;
8. no test binds the production remote endpoint;
9. no source path outside the exact AR three-path scope changes.

Compile-time function-shape tests and private fake composition helpers are preferred over production I/O.

## Identity/security invariants preserved

AR changes no established identity boundary:

- `DeviceId` / authenticated PRW session identity remains logical identity;
- `TransportIdentity` remains lower-transport certificate identity only;
- IP/socket remains transient endpoint/configuration data only;
- `SessionId` remains authentication correlation only;
- runtime/task/thread/controller/channel/endpoint identifiers remain implementation details only;
- reachability authority possession is an admission prerequisite, not user identity;
- endpoint startup success is not authorization or readiness evidence.

Protected request authorization remains fresh-current through existing registry/transport/policy evaluation.

## Explicitly absent

C03e-AR does not authorize or materialize:

- `main.rs` wiring;
- Linux process-thread composition;
- SIGTERM/SIGINT changes;
- local or remote readiness publication;
- production invocation of reachability bootstrap;
- production endpoint bind invocation;
- production bind-address selection;
- expected-device production/discovery;
- retry/backoff/reconnect/rebootstrap/replacement;
- provider health/quorum policy;
- hard abort or hard drain deadline;
- second Tokio runtime or `rt-multi-thread`;
- generic `block_on`/Runtime/Handle exposure;
- systemd unit/drop-in mutation;
- host firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart;
- recovery/PRWF initialization;
- R1-R4 activation;
- merge.

## Stop conditions

AR must stop for a new selection checkpoint rather than widen scope if implementation requires:

- a second Tokio runtime;
- exposing the raw runtime/handle;
- changing local `Ready` semantics;
- modifying `linux_bootstrap.rs`, `main.rs` or signal-aware runtime code;
- a production bind-address source;
- production expected-device/discovery machinery;
- automatic retry/reconnect/rebootstrap;
- hard cancellation/abort semantics;
- systemd/host/deployment mutation.

## Validation gate

AR may claim:

`C03E_AR_SINGLE_EXECUTOR_REACHABILITY_BOOTSTRAP_ENDPOINT_STARTUP_SOURCE_MATERIALIZED`

only after:

1. exact AQ ancestry is reverified;
2. AQ -> AR compare proves exactly the three bounded paths and no other mutation;
3. focused source tests preserve the selected ownership/failure ordering without production I/O;
4. canonical Rust validation on the exact final AR head reaches terminal FULL PASS;
5. any actually-triggered Android validation reaches its terminal verdict before closure;
6. disposable validations are classified by their actual terminal states;
7. Drive immutable evidence is uploaded and read back byte-exact;
8. rolling Drive evidence is appended with exact predecessor-prefix preservation;
9. the AR PR remains draft/open/unmerged and moves to `Status: CLOSED` only after evidence closeout.
