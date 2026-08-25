# Phase 152 C03e-AZ — Linux Bootstrap Injected Remote Process Operation Composition Source Materialization Staging

Status: STAGED

Target gate:

`C03E_AZ_LINUX_BOOTSTRAP_INJECTED_REMOTE_PROCESS_OPERATION_COMPOSITION_SOURCE_MATERIALIZED`

## Exact predecessor

Canonical predecessor is closed C03e-AY:

- branch: `phase-152-c03e-ay-linux-bootstrap-injected-remote-process-operation-composition-selection-staging`
- head: `b3afc5aee8b29368ff6a5357a24931d9e7d01ada`
- tree: `15acc87ba89d528546585543bb3896e0e1bbdbfd`
- gate: `C03E_AY_LINUX_BOOTSTRAP_INJECTED_REMOTE_PROCESS_OPERATION_COMPOSITION_SELECTED`

AY selected one library-owned injected remote-process operation composition in `linux_bootstrap.rs`. The purpose is to keep AR same-executor reachability/bootstrap and endpoint-startup seams crate-internal while exposing only an ownership-complete operation compatible with the already-closed AX companion facade.

AZ materializes only that selected boundary. It does not activate it from the executable.

## Exact bounded source scope

The final AY -> AZ diff is restricted to exactly these two paths:

1. this contract;
2. `crates/prw-agent/src/linux_bootstrap.rs`.

No Cargo manifest, lockfile, workflow, Android application, bridge crate, transport implementation, reachability implementation, remote-session implementation module, local Linux runtime, `main.rs`, readiness/status implementation, systemd unit, host configuration, deployment state or merge mutation is allowed in AZ.

## Materialized bootstrap-facing input owner

AZ may add one public, non-cloneable bootstrap-facing owner equivalent in responsibility to:

`LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>`

It owns only already-typed injected values required by the existing closed runtime seams:

- caller-supplied `SocketAddr` bind configuration;
- `NonZeroUsize` maximum active-worker bound;
- one `SharedCurrentCapabilityAuthority<P>`;
- one owned `SessionAuthenticationService`;
- one bounded `mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`;
- one admission-timing callback `F`;
- one registered-worker completion callback `C`;
- one pre-authentication expected-device rejection callback `R`;
- one repeated-admission failure callback `E`.

Construction is ownership-only. It must perform no credential read, provider I/O, endpoint bind, listener activation, session authentication, authorization, worker spawn, Tokio task spawn, readiness publication or process-lifecycle mutation.

The owner must not implement `Clone` and must not expose a raw Tokio `Runtime`, `Handle`, transport owner, reachability provider client, secret material, lock guard, authenticated-session owner or remote peer.

## Materialized operation factory

AZ may add one public factory equivalent in responsibility to:

`linux_agent_remote_process_operation(inputs)`

The returned value must satisfy:

`FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static`

Factory construction must be side-effect-free and ownership-only. The returned closure is the only new executable-compatible object; the AR `pub(crate)` executor/reachability/endpoint seams remain crate-internal.

The existing AX `run_with_remote_process_companion<F>` signature and semantics must remain unchanged. The existing no-companion `linux_bootstrap::run()` signature and semantics must remain unchanged.

## Exact returned-operation sequence

Only invocation of the returned operation may execute remote startup/lifecycle work. Its selected order is exact:

1. construct exactly one existing `RemoteSessionExecutorRuntime`;
2. on that same executor, invoke `bootstrap_reachability_authority_from_systemd_credentials()` exactly once;
3. retain the exact resulting `ReachabilityAuthorityRuntimeOwner` on success;
4. move the exact same executor and authority owner into `RemoteSessionEndpointLifecycleRuntime::bind_with_executor_from_systemd_credentials(...)` with the injected `SocketAddr`;
5. only after endpoint startup succeeds, publish the exact returned AP `RemoteSessionSupervisorShutdownController` through the consumed `LinuxAgentRemoteSupervisorShutdownPublisher`;
6. irrespective of whether publication reports `Published` or `ReceiverGoneShutdownRequested`, drive the exact resulting endpoint lifecycle through `drive_repeated_real_remote_admission_endpoint_lifecycle(...)` using the injected worker bound, current authority, session-auth service, expected-device receiver, timing callback and callbacks;
7. return only after that existing endpoint lifecycle returns.

No replacement executor, alternate endpoint constructor, parallel bootstrap or parallel pre-authentication lane is allowed.

## Same-executor invariant

AZ preserves AQ/AR exactly:

```text
one RemoteSessionExecutorRuntime
  -> reachability custody/provider bootstrap
  -> same executor + admitted authority move into endpoint startup
  -> same executor retained by RemoteSessionEndpointLifecycleRuntime
  -> same executor drives repeated admission/workers/shutdown/idle drain
```

The executor is implementation custody only. Runtime, task, thread, join, controller, channel and endpoint identifiers are never PRW identity or authorization evidence.

AZ must not introduce a second Tokio runtime, `rt-multi-thread`, raw runtime/handle exposure or generic future-driving surface.

## Controller publication semantics

The returned operation consumes exactly one AX `LinuxAgentRemoteSupervisorShutdownPublisher`.

No publication occurs before successful endpoint startup.

The exact AP controller is published once. No clone, replacement or second shutdown authority is created.

If the process-side receiver has disappeared, the existing AT/AX publisher requests orderly shutdown on the recovered AP controller and reports `ReceiverGoneShutdownRequested`. AZ must still enter the exact endpoint lifecycle; the already-requested AP shutdown signal remains authoritative for cancellation, endpoint close and idle drain.

Publication outcome remains secondary remote-lane behavior. AZ does not map it to local process exit policy or readiness.

## Failure semantics

### Executor construction failure

If `RemoteSessionExecutorRuntime::new()` fails:

- no reachability credential read occurs;
- no provider I/O occurs;
- no endpoint credential read or bind occurs;
- no controller exists or is published;
- no expected-device request is consumed;
- no retry or fallback runtime is attempted;
- the injected remote operation returns.

No existing local bootstrap startup-failure class is widened.

### Reachability bootstrap failure

If the existing AR bootstrap returns its existing custody/provider error:

- no endpoint bind occurs;
- no controller is published;
- no expected-device request is consumed;
- no retry/rebootstrap/reconnect occurs;
- local IPC readiness is unchanged;
- the injected remote operation returns.

### Endpoint startup failure

If existing AR/AP endpoint startup fails:

- its existing transaction owns failure cleanup and exact authority custody;
- no second bind or alternate address is attempted;
- no replacement executor is created;
- no controller is published;
- no expected-device request is consumed;
- the injected remote operation returns.

AZ does not widen the public bootstrap surface with an authority-recovery API.

### Endpoint lifecycle result

The existing AP/AN/AL lifecycle remains authoritative. If it returns `RemoteSessionPersistentCollectionConfigError`, AZ may ignore that value at this composition boundary and simply terminate the remote operation.

AZ does not select or materialize:

- remote failure -> local runtime shutdown;
- remote failure -> process exit failure;
- automatic restart/retry;
- hard drain deadline;
- task/thread abort;
- detached cleanup.

## Expected-device source remains injected

AZ accepts only an already-created bounded receiver. It does not create a production producer for `RemoteSessionExpectedDeviceAdmissionRequest` and does not add:

- discovery polling;
- registry watch loops;
- automatic device enumeration;
- controller queue production;
- retry/reconnect/replacement admission;
- parallel pre-authentication attempts.

The logical expected `DeviceId` remains scheduling input only; the existing AJ transaction resolves current lower-transport identity from current registry state.

## Dispatcher remains injected

AZ does not implement a concrete production `CapabilityDispatcher`.

The dispatcher remains carried inside each injected expected-device request. No file, terminal, forwarding, DNS, device-management, policy-management or host-operation backend is activated merely because AZ exists.

## Current registry/policy authority remains injected

AZ consumes an already-created `SharedCurrentCapabilityAuthority<P>`.

It does not select registry persistence, production population, refresh/watch behavior or production policy loading/mutation. Existing fresh-current registry/transport/policy evaluation remains authoritative for protected requests.

## Session authentication and timing remain injected

AZ consumes one already-created `SessionAuthenticationService` and the selected timing callback.

It adds no account authentication, enrollment mutation, session-ID producer, wall-clock acquisition, NTP policy, clock-recovery policy, timestamp persistence or timing retry behavior.

`SessionId` remains authentication correlation only.

## Bind-address custody remains injected

`SocketAddr` is endpoint configuration only.

AZ does not select a production bind-address source, wildcard/public/LAN default, arbitrary environment/CLI widening, DNS-derived widening, firewall/NAT/route mutation, UDP/relay/SOCKS/TUN/TAP activation or any interpretation of IP/socket data as identity/auth/readiness evidence.

## Existing local readiness and signal ownership remain unchanged

AZ preserves C02f-CH and AT/AX:

- local IPC `Ready` remains independent of remote provider/bootstrap/endpoint/session availability;
- no local status transition is added;
- no new `SignalFd` or signal handler is added;
- the existing local signal owner remains sole SIGTERM/SIGINT owner;
- remote startup/lifecycle failure does not request local shutdown;
- cleanup/mask-restoration ordering remains owned by existing signal-aware bootstrap code.

## Identity/security invariants preserved

AZ changes no established boundary:

- `DeviceId` / authenticated PRW session identity remains logical identity;
- `TransportIdentity` remains lower-transport certificate identity only;
- IP/socket remains transient endpoint configuration only;
- `SessionId` remains auth correlation only;
- runtime/task/thread/controller/channel/lock/endpoint identifiers remain implementation details only;
- reachability authority possession is an admission prerequisite, not user identity;
- endpoint startup success is not authorization or readiness evidence.

Protected requests continue to use existing fresh-current registry, current transport and policy evaluation.

## Focused non-networking proof

AZ tests must not execute the real returned production operation because that would read production credentials/provider state and bind the real remote endpoint.

Private generic composition helpers may model the selected sequence with synthetic values and injected fake stages.

Focused proof must establish:

1. input-owner construction itself performs no remote work;
2. the public factory returns the exact `FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static` shape;
3. helper ordering is executor -> bootstrap -> endpoint -> publication -> lifecycle;
4. executor failure suppresses bootstrap, endpoint, publication and lifecycle;
5. bootstrap failure suppresses endpoint, publication and lifecycle;
6. endpoint failure suppresses publication and lifecycle;
7. publication occurs exactly once only after endpoint success;
8. a receiver-gone-equivalent publication result still enters the same lifecycle stage;
9. the same synthetic executor value survives fake bootstrap/endpoint composition;
10. no production credential/provider/network operation is invoked by tests;
11. no source path outside the exact two-path AZ scope changes.

## Explicitly absent

C03e-AZ does not authorize or materialize:

- `main.rs` wiring;
- automatic executable invocation;
- production bind-address selection;
- production expected-device/discovery producer;
- concrete production capability dispatcher;
- production registry persistence/population/watch;
- production policy loading/mutation;
- production timing source;
- account authentication/enrollment mutation;
- readiness changes;
- new process-signal ownership;
- remote failure -> local fail-stop or process-exit policy;
- retry/backoff/reconnect/rebootstrap/rebind/replacement;
- hard abort or hard drain deadline;
- second Tokio runtime or `rt-multi-thread`;
- generic runtime/handle/block_on exposure;
- systemd unit/drop-in mutation;
- host firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart;
- recovery/PRWF initialization;
- R1-R4 activation;
- merge.

## Stop conditions

AZ must stop for a new selection checkpoint rather than widen scope if implementation requires:

- `main.rs` mutation;
- any source path beyond this contract and `linux_bootstrap.rs`;
- a concrete production dispatcher;
- a production expected-device/discovery producer;
- a production bind-address source;
- registry/policy production loading or mutation;
- a second Tokio runtime;
- raw runtime/handle exposure;
- local readiness changes;
- new signal ownership;
- remote failure -> local fail-stop/process-exit policy;
- retry/reconnect/rebootstrap/rebind;
- hard abort/deadline;
- systemd/host/deployment mutation.

## Validation gate

AZ may claim:

`C03E_AZ_LINUX_BOOTSTRAP_INJECTED_REMOTE_PROCESS_OPERATION_COMPOSITION_SOURCE_MATERIALIZED`

only after:

1. exact AY ancestry is reverified;
2. AY -> AZ compare proves exactly the two bounded paths and no other mutation;
3. focused source tests preserve the selected ordering/failure semantics without production I/O;
4. canonical Rust validation on the exact final AZ head reaches terminal FULL PASS;
5. any actually-triggered Android validation reaches terminal verdict before closure;
6. disposable validations are classified by actual terminal state;
7. immutable Drive evidence is uploaded and read back byte-exact;
8. rolling Drive evidence is appended with exact predecessor-prefix preservation;
9. the AZ PR remains draft/open/unmerged and moves to `Status: CLOSED` only after evidence closeout.
