# Phase 152 C03e-AY — Linux Bootstrap Injected Remote Process Operation Composition Selection Staging

Status: STAGED

Target gate:

`C03E_AY_LINUX_BOOTSTRAP_INJECTED_REMOTE_PROCESS_OPERATION_COMPOSITION_SELECTED`

## Exact predecessor

Canonical predecessor is closed C03e-AX:

- branch: `phase-152-c03e-ax-linux-bootstrap-remote-process-companion-public-facade-source-materialization-staging`
- head: `fd090bd2e8eb6c437d59c2ae264edbe0e349e2f0`
- tree: `179b8881a5a0722cc036a96a727ee42fa259f880`
- gate: `C03E_AX_LINUX_BOOTSTRAP_REMOTE_PROCESS_COMPANION_PUBLIC_FACADE_SOURCE_MATERIALIZED`

AX exposes a public injected `run_with_remote_process_companion<F>` bootstrap facade while preserving the existing no-companion `linux_bootstrap::run()` path. AR already materialized the crate-internal same-executor reachability bootstrap and endpoint-startup seams, but those seams remain inaccessible to the binary crate by design. AY selects only the library-owned operation-composition boundary needed to bridge that visibility gap without activating the executable.

## Exact bounded selection scope

AY is selection-only.

The exact AX -> AY diff is restricted to this one documentation contract path.

AY does not authorize Rust/source materialization. The immediately following C03e-AZ source-materialization checkpoint is selected to remain bounded to exactly:

1. the C03e-AZ contract; and
2. `crates/prw-agent/src/linux_bootstrap.rs`.

No Cargo manifest, lockfile, workflow, Android application, bridge crate, transport implementation, reachability implementation, remote-session implementation module, local Linux runtime, `main.rs`, readiness/status implementation, systemd unit, host configuration, deployment state or merge mutation is selected by AY.

## Problem boundary selected

After AX, the standalone binary can call the public injected companion facade, but it must not directly reach the AR crate-internal methods:

- `RemoteSessionExecutorRuntime::bootstrap_reachability_authority_from_systemd_credentials(...)`;
- `RemoteSessionEndpointLifecycleRuntime::bind_with_executor_from_systemd_credentials(...)`.

Those methods intentionally remain `pub(crate)` so raw executor/reachability startup composition does not become general executable API.

AY therefore selects one library-owned, injected remote-process operation factory in `linux_bootstrap.rs`. The future factory hides AR internals, performs no work at factory-construction time, and returns one `FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static` operation compatible with the already-closed AX facade.

The returned operation remains uninvoked until a separately gated caller passes it to `run_with_remote_process_companion(...)`.

## Selected injected input owner

AZ may materialize one non-cloneable public bootstrap-facing input owner equivalent in responsibility to:

`LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>`

The owner may contain only already-typed injected values required by the closed AR/AP/AL seams:

- one caller-supplied `SocketAddr` bind configuration;
- one `NonZeroUsize` maximum active-worker bound;
- one `SharedCurrentCapabilityAuthority<P>` containing the already-supplied current registry/policy state;
- one owned `SessionAuthenticationService`;
- one bounded Tokio `mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`;
- one injected admission-timing function `F`;
- one completion callback `C`;
- one pre-authentication rejection callback `R`;
- one repeated-admission failure callback `E`.

The input owner is custody/composition only. Its constructor must perform no credential read, provider I/O, endpoint bind, session authentication, authorization, worker spawn, task spawn, readiness publication or process-lifecycle mutation.

The owner must not be `Clone` and must not expose a generic Tokio runtime, runtime handle, transport owner, reachability provider client, secret material, lock guard, authenticated-session owner or remote peer.

## Selected operation factory

AZ may materialize one public factory equivalent in responsibility to:

`linux_agent_remote_process_operation(inputs) -> impl FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static`

Factory construction itself must be ownership-only and side-effect-free.

Only invocation of the returned operation on the existing AT join-owned remote OS thread may perform the selected remote startup and lifecycle work.

The factory must not alter the existing AX `run_with_remote_process_companion<F>` signature or the existing no-companion `run()` signature.

## Exact returned-operation sequence

When, and only when, a separately gated caller invokes the returned operation, it must execute this exact sequence:

1. construct exactly one existing private `RemoteSessionExecutorRuntime`;
2. on that exact executor, execute the existing AR reachability-authority bootstrap from fixed systemd credentials exactly once;
3. on success, retain the resulting exact `ReachabilityAuthorityRuntimeOwner`;
4. move the exact same executor and authority owner into the existing AR same-executor endpoint startup with the injected `SocketAddr`;
5. only after endpoint startup succeeds, publish the exact existing non-cloneable AP shutdown controller through the AX bootstrap-facing publisher;
6. drive the exact resulting `RemoteSessionEndpointLifecycleRuntime` through the existing AP/AN/AL repeated real-admission endpoint lifecycle using the injected worker bound, current authority, session-authentication service, expected-device receiver, admission-timing function and callbacks;
7. return from the remote operation only after the existing endpoint lifecycle returns.

No replacement executor, alternate endpoint startup path or parallel pre-authentication lane is selected.

## Same-executor invariant

AY preserves the closed AQ/AR invariant exactly:

```text
one RemoteSessionExecutorRuntime
  -> drives reachability custody/provider bootstrap
  -> moves into endpoint startup
  -> remains inside RemoteSessionEndpointLifecycleRuntime
  -> drives repeated admission/workers/shutdown/idle drain
```

The executor is implementation custody only. It is never PRW identity, authentication evidence, authorization authority or readiness evidence.

No second Tokio runtime, `rt-multi-thread`, raw `Runtime`, raw `Handle`, generic `block_on` or detached Tokio task surface is selected.

## Shutdown-controller publication semantics

The returned operation receives exactly one existing AX `LinuxAgentRemoteSupervisorShutdownPublisher` by ownership.

Endpoint startup must succeed before any publication attempt.

The exact AP controller is published once through the existing AX wrapper. No controller clone, synthetic replacement controller or second shutdown authority is selected.

If process-side ownership has disappeared, the existing AT/AX publication semantics remain authoritative: the recovered controller is asked to request orderly shutdown. The remote operation must still enter the existing endpoint lifecycle so the already-requested AP shutdown signal can drive worker cancellation, endpoint close and idle drain through the existing AN/AL ordering.

Publication outcome does not become local process exit policy or readiness evidence in AY.

## Failure semantics before controller publication

### Executor construction failure

If the one executor cannot be constructed:

- no reachability credential read occurs;
- no provider I/O occurs;
- no endpoint credential read or bind occurs;
- no shutdown controller exists;
- no expected-device request is consumed;
- no retry/fallback runtime is attempted;
- the returned operation simply terminates its remote lane.

The existing AT/AX process-finalization evidence remains the only selected process-level observation. AY does not widen local bootstrap startup errors.

### Reachability custody/provider failure

If the existing AR bootstrap fails:

- no endpoint bind occurs;
- no controller is published;
- no expected-device request is consumed;
- no retry/rebootstrap/reconnect occurs;
- local IPC readiness remains unchanged;
- the remote operation terminates without requesting local process shutdown.

### Endpoint startup failure

If the existing AR/AP endpoint startup fails:

- the exact existing startup transaction owns its failure cleanup;
- no second bind or alternate address is attempted;
- no replacement executor is constructed;
- no controller is published;
- no expected-device request is consumed;
- the remote operation terminates without widening local bootstrap startup failure classes.

AY does not add authority-recovery API to the public bootstrap surface merely because the internal AP failure retains exact authority custody.

## Endpoint lifecycle result semantics

The existing AP/AN/AL lifecycle remains authoritative after controller publication.

A returned `RemoteSessionPersistentCollectionConfigError` may terminate only the injected remote operation. AY does not select:

- remote failure -> local runtime shutdown;
- remote failure -> process exit failure;
- automatic restart/retry;
- a hard drain deadline;
- task/thread abort;
- detached fallback cleanup.

A later observability or process-exit-policy checkpoint is required before any such semantics can be added.

## Expected-device request source remains injected

AY explicitly does not select a production producer for `RemoteSessionExpectedDeviceAdmissionRequest`.

The bounded receiver is supplied from outside the selected operation owner. AY adds no:

- discovery polling;
- registry watch loop;
- controller queue producer;
- automatic expected-device enumeration;
- retry/reconnect/replacement admission;
- parallel pre-authentication attempts.

Each request continues to carry a logical expected `DeviceId`; lower-transport identity is resolved fresh from current registry state inside the existing AJ transaction and is never caller-selected.

## Dispatcher remains injected

AY does not select or materialize a concrete production `CapabilityDispatcher`.

The dispatcher value remains carried inside each injected expected-device request exactly as required by the existing AL/AJ path.

No file, terminal, forwarding, DNS, device-management, policy-management or host-operation backend is activated by AY.

A separately reviewed dispatcher/materialization checkpoint is required before any concrete remote capability side effect can be enabled.

## Registry and policy remain injected current authority

AY does not select how a production `WorkspaceDeviceRegistry` is populated, refreshed or mutated, and does not select a production policy source.

The operation consumes an already-constructed `SharedCurrentCapabilityAuthority<P>`.

Existing fresh-current request authorization remains authoritative. No per-worker registry or policy snapshot may replace it.

## Session-authentication service remains injected

AY does not select account authentication, enrollment mutation, registry persistence or session-ID production.

The operation consumes one already-created `SessionAuthenticationService` and delegates all session challenge/proof behavior to existing closed seams.

`SessionId` remains authentication correlation only and never becomes logical identity.

## Timing remains injected

AY does not select wall-clock acquisition, NTP policy, clock recovery, timestamp persistence or retry timing.

The existing AL admission-timing callback remains caller-injected and sampled only when one expected-device admission attempt actually starts.

## Bind-address custody remains unresolved

The `SocketAddr` is injected endpoint configuration only.

AY does not select a production source or default for the bind address and does not authorize:

- wildcard/public/LAN widening;
- arbitrary CLI/environment bind widening;
- DNS-derived bind widening;
- firewall/NAT/route mutation;
- UDP/relay/SOCKS/TUN/TAP activation.

IP/socket data remains transient endpoint/configuration data only, never PRW identity, authentication, authorization, authority or readiness evidence.

## Existing local readiness remains independent

AY preserves C02f-CH exactly.

The local Linux lifecycle and local IPC `Ready` meaning remain independent of reachability-provider availability, remote endpoint startup and remote session admission.

The selected remote operation publishes no local readiness and does not mutate `LocalAgentStatusSnapshot`.

## Identity/security invariants preserved

AY changes no established identity boundary:

- `DeviceId` / authenticated PRW session identity remains logical identity;
- `TransportIdentity` remains lower-transport certificate identity only;
- IP/socket remains transient endpoint configuration only;
- `SessionId` remains authentication correlation only;
- runtime/task/thread/controller/channel/lock/endpoint identifiers remain implementation details only;
- reachability authority possession is an admission prerequisite, not user identity;
- endpoint startup success is not authorization or readiness evidence.

Protected requests continue to use fresh current registry, transport and policy evaluation through the existing worker path.

## Explicitly absent

C03e-AY does not authorize or materialize:

- Rust/source changes;
- `main.rs` wiring;
- automatic executable invocation of the AX companion path;
- production bind-address selection;
- production expected-device/discovery producer;
- concrete production capability dispatcher;
- production registry population/watch/persistence;
- production policy loading/mutation;
- production timing source;
- account authentication or enrollment mutation;
- local or remote readiness publication changes;
- new signal ownership or another `SignalFd`;
- remote failure -> local fail-stop;
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

## C03e-AZ source scope selected

The immediately following source-materialization checkpoint may modify exactly:

1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_AZ_LINUX_BOOTSTRAP_INJECTED_REMOTE_PROCESS_OPERATION_COMPOSITION_SOURCE_MATERIALIZATION_STAGING.md`;
2. `crates/prw-agent/src/linux_bootstrap.rs`.

AZ should use injected/fake values for focused non-networking tests. No test may read production reachability credentials, mesh credentials, bind a production endpoint or execute real file/terminal/forwarding capability side effects.

## AZ focused proof requirements selected

AZ validation should prove at minimum:

1. input-owner construction performs no remote work;
2. the returned operation has the exact `FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static` shape;
3. private composition helpers enforce executor -> reachability bootstrap -> endpoint startup -> controller publication -> lifecycle drive ordering;
4. executor failure suppresses bootstrap, bind, publication and lifecycle;
5. reachability failure suppresses bind, publication and lifecycle;
6. endpoint-startup failure suppresses publication and lifecycle;
7. publication occurs exactly once after endpoint success;
8. receiver-gone publication semantics still lead into the same orderly endpoint lifecycle with shutdown already requested;
9. the exact same synthetic executor value is preserved through fake bootstrap/bind composition;
10. no production credential/network I/O is invoked by tests;
11. no path outside the exact AZ two-path scope changes.

## Stop conditions

AY/AZ must stop for a new selection checkpoint rather than widen scope if implementation requires:

- `main.rs` mutation;
- a concrete production dispatcher;
- a production expected-device/discovery producer;
- a production bind-address source;
- registry/policy production loading or mutation;
- a second Tokio runtime;
- raw runtime/handle exposure;
- local readiness changes;
- new process-signal ownership;
- remote failure -> local fail-stop or process-exit policy;
- retry/reconnect/rebootstrap/rebind;
- hard abort/deadline;
- systemd/host/deployment mutation.

## Validation gate

AY may claim:

`C03E_AY_LINUX_BOOTSTRAP_INJECTED_REMOTE_PROCESS_OPERATION_COMPOSITION_SELECTED`

only after:

1. exact AX ancestry is reverified;
2. AX -> AY compare proves exactly this one docs-only path and no other mutation;
3. canonical Rust validation on the exact final AY head reaches terminal FULL PASS;
4. any actually-triggered Android validation reaches its terminal verdict before closure;
5. disposable validations are classified by their actual terminal states;
6. immutable Drive evidence is uploaded and read back byte-exact;
7. rolling Drive evidence is appended with exact predecessor-prefix preservation;
8. the AY PR remains draft/open/unmerged and moves to `Status: CLOSED` only after evidence closeout.
