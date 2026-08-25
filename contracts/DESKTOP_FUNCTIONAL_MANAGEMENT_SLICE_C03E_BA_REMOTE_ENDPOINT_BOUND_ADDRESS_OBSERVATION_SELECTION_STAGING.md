# Phase 152 C03e-BA — Remote Endpoint Bound-Address Observation Selection Staging

Status: STAGED

Gate target: `C03E_BA_REMOTE_ENDPOINT_BOUND_ADDRESS_OBSERVATION_SELECTED`

Exact predecessor: closed C03e-AZ `0525c546c575857571a55c8cdba2fb74c5e3e288` / tree `bc60150698ccf35eed50d242dd69ecafaa8fb0b9`.

## 1. Purpose

C03e-BA selects one narrow library boundary that lets the already-bound remote endpoint expose the exact local `SocketAddr` reported by the existing transport endpoint.

This checkpoint is selection-only. It does not materialize Rust source, choose a production bind address, publish reachability, activate the remote operation from `main.rs`, or mutate any provider/host state.

The selected observation is configuration/runtime state only. It is never PRW logical identity, transport identity, authentication evidence, authorization evidence, readiness evidence, or provider-currentness evidence.

## 2. Audit basis

The final C03e-AZ source establishes all of the following:

1. `linux_agent_remote_process_operation(...)` still receives `bind_addr: SocketAddr` from its caller.
2. The operation constructs one private current-thread `RemoteSessionExecutorRuntime`.
3. Reachability authority bootstrap executes on that exact executor.
4. `RemoteSessionEndpointLifecycleRuntime::bind_with_executor_from_systemd_credentials(...)` binds the remote endpoint with that exact executor and admitted authority owner.
5. The lifecycle owner retains the bound `AgentRemoteTransportRuntime` privately until the existing admission/shutdown lifecycle consumes it.
6. Neither AZ nor `main.rs` selects or activates production remote inputs.

The lower transport already exposes the exact bound local address through `MeshQuicEndpoint::local_addr() -> Result<SocketAddr, io::Error>`.

The Agent wrappers do not currently project that observation:

- `AgentRemoteTransportRuntime` owns the `MeshQuicEndpoint` but exposes no bound-address getter.
- `RemoteSessionEndpointLifecycleRuntime` owns the `AgentRemoteTransportRuntime` but exposes no bound-address getter.

The control-plane live-owner authority record is a different security object. `ReachabilityLiveOwnerAuthorityRecord` contains the exact `PeerConnectivityIdentity`, lifecycle, fence, and authority attempt identifier. It does not contain a `SocketAddr`.

The provider-neutral connectivity model is also distinct. `ConnectivityEndpoint` and `ConnectivityCandidate` represent transient network path data, while `PeerConnectivityIdentity` preserves logical `DeviceId` plus independently rotatable `TransportIdentity`.

The NAT traversal crate similarly treats socket endpoints as bounded traversal/path data and owns no socket or production-network activation.

Therefore C03e-BA selects observation only. It deliberately does not collapse bound socket state, reachability authority, connectivity candidates, or publication into one boundary.

## 3. Selected source-of-truth chain

The exact observation source is the existing bound QUIC endpoint:

`MeshQuicEndpoint::local_addr()`

The selected Agent projection chain is:

`MeshQuicEndpoint::local_addr()`
→ `AgentRemoteTransportRuntime` crate-private read-only delegation
→ `RemoteSessionEndpointLifecycleRuntime::bound_addr()` public typed observation

No caller-supplied address may substitute for the lower endpoint observation once the endpoint has been bound.

The original requested bind address remains configuration input only. In particular, if the caller requested port `0`, the requested address is not sufficient evidence of the kernel-selected port. The observation must come from the already-bound endpoint.

## 4. Selected transport-layer projection

A future source-materialization checkpoint may add one crate-private read-only method to `AgentRemoteTransportRuntime` with the conceptual shape:

```text
fn bound_addr(&self) -> Result<SocketAddr, io::Error>
```

The implementation must delegate directly to the existing `MeshQuicEndpoint::local_addr()`.

It must not:

- cache caller input as if it were the observed result;
- synthesize an address;
- rewrite wildcard addresses;
- perform DNS resolution;
- probe interfaces;
- perform STUN or ICE;
- open another socket;
- rebind the endpoint;
- reconnect transport;
- retry another address;
- mutate endpoint state;
- close the endpoint;
- alter transport identity.

The low-level `io::Error` remains inside the Agent implementation boundary and is not selected as a new public product error surface.

## 5. Selected lifecycle-layer projection

A future source-materialization checkpoint may add one public read-only method to `RemoteSessionEndpointLifecycleRuntime`:

```text
pub fn bound_addr(&self) -> Result<SocketAddr, RemoteSessionEndpointBoundAddressError>
```

The method borrows `&self`; it does not consume, clone, transfer, replace, close, or otherwise mutate the lifecycle owner.

It delegates to the exact retained `AgentRemoteTransportRuntime` observation selected above.

The returned `SocketAddr` is the exact lower-transport observation. No normalization or publishability claim is attached to it.

## 6. Selected stable error surface

C03e-BA selects one bounded public error classification:

```text
RemoteSessionEndpointBoundAddressError::Unavailable
```

Any lower `io::Error` while reading the bound address maps to this stable class.

The public error must not expose:

- raw OS error strings as protocol/security decisions;
- file descriptors;
- runtime/task/thread identifiers;
- endpoint object identity;
- credentials;
- certificate material;
- provider state.

Observation failure does not itself select endpoint close, retry, rebind, reconnect, process failure, readiness transition, or provider mutation. Those policies remain separately gated.

## 7. Exact meaning of the observed address

The returned `SocketAddr` means only:

> the local socket address reported by the already-bound existing remote transport endpoint at the time of observation.

It does not mean:

- globally reachable address;
- Internet-reachable address;
- LAN-reachable address;
- externally mapped NAT address;
- selected connectivity path;
- authenticated peer identity;
- authorized PRW device identity;
- live-owner authority identity;
- readiness;
- successful reachability publication.

A wildcard local address such as `0.0.0.0:<port>` or `[::]:<port>` remains an exact observation but is not thereby selected as a publishable connectivity candidate.

C03e-BA performs no `SocketAddr -> ConnectivityEndpoint` conversion.

## 8. Identity separation

C03e-BA preserves the existing identity split exactly:

- `DeviceId` / authenticated PRW session identity = logical identity.
- `TransportIdentity` = authenticated lower-transport certificate identity.
- `SocketAddr` = transient network endpoint/configuration state only.
- `SessionId` = authentication correlation only.

No IP address, port, socket, endpoint object, runtime, thread, task, channel, controller, PID, UID, GID, or process-local identifier becomes PRW identity.

The observation cannot authorize a protected request and cannot replace fresh registry/current-transport/current-policy evaluation.

## 9. Authority separation

C03e-BA does not modify the existing reachability live-owner authority model.

It does not change:

- exact-peer live-owner keys;
- `PeerConnectivityIdentity` binding;
- fence allocation;
- authority attempt identifiers;
- `Current` / `Released` lifecycle;
- dual-CAS transaction planning;
- provider currentness checks;
- reconciliation semantics;
- systemd reachability credential custody.

The observed `SocketAddr` is not inserted into `ReachabilityLiveOwnerAuthorityRecord`.

## 10. Connectivity separation

C03e-BA does not create or mutate:

- `ConnectivityEndpoint`;
- `ConnectivityCandidate`;
- `CandidateId`;
- `PeerConnectivityPlan`;
- `ReachabilityObservation`;
- selected connectivity path;
- NAT traversal state;
- STUN discovery state;
- ICE credentials/candidates/checks;
- relay state.

Future conversion of an observed local address into any connectivity candidate requires a separate explicit selection that defines address validation, path class, candidate-ID custody, publication/currentness and stale-endpoint replacement semantics.

## 11. Bind-policy separation

C03e-BA does not select the production value passed as `bind_addr`.

Specifically it does not select:

- environment-variable bind addresses;
- CLI bind addresses;
- config-file bind addresses;
- DNS names;
- arbitrary caller-controlled public/LAN addresses;
- wildcard binding as production policy;
- fixed service ports;
- ephemeral port `0` as production policy;
- IPv4 versus IPv6 production policy.

The existing caller-injected `SocketAddr` shape remains unchanged.

A future bind-policy checkpoint must be separately audited for exposure/widening risk.

## 12. Publication separation

C03e-BA does not publish the observed address anywhere.

It performs no:

- etcd write;
- Spanner write;
- registry mutation;
- discovery advertisement;
- control-plane message;
- Android/Desktop signaling message;
- NAT mapping publication;
- relay registration;
- readiness publication;
- log-based endpoint advertisement.

No provider credentials are read merely to observe the already-bound endpoint.

## 13. Lifecycle and shutdown preservation

The existing AP/AN/AL endpoint/session lifecycle remains authoritative.

Observation must not alter:

- supervisor shutdown controller/signal semantics;
- repeated real-admission loop;
- worker capacity;
- worker cancellation;
- idle waiting;
- endpoint close ordering;
- private current-thread runtime ownership;
- exact lifecycle result propagation.

No detached task or second runtime is introduced.

No hard abort, timeout/deadline, fallback runtime, `Handle`, generic `block_on`, or multi-thread Tokio runtime is selected.

## 14. Startup/failure preservation

C03e-BA does not change endpoint startup ordering:

1. executor construction;
2. reachability authority bootstrap;
3. endpoint bind;
4. shutdown-controller publication;
5. existing lifecycle drive.

The new observation accessor is not selected as a mandatory startup-success condition.

If observation fails after a successful bind, C03e-BA selects only the bounded observation error. It does not silently transform that event into a bind failure or invent new authority-owner recovery semantics after custody has already moved into the lifecycle owner.

## 15. Public API bound

The only new public product-facing concept selected by BA is:

- `RemoteSessionEndpointBoundAddressError`;
- `RemoteSessionEndpointLifecycleRuntime::bound_addr(&self)`.

`AgentRemoteTransportRuntime` remains an internal Agent implementation detail for this seam.

The lower `MeshQuicEndpoint` API remains unchanged.

No public Tokio runtime/handle is exposed.

## 16. Future source-materialization path bound

A future C03e-BB source-materialization checkpoint, if opened, is expected to remain limited to the existing Agent files required for this projection:

1. `crates/prw-agent/src/remote_transport_runtime.rs`
2. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`
3. `crates/prw-agent/src/remote_session_capability_runtime.rs`
4. the C03e-BB materialization contract itself.

`crates/prw-remote-transport/src/runtime.rs` should not require modification because `MeshQuicEndpoint::local_addr()` already exists.

`crates/prw-agent/src/linux_bootstrap.rs` is not selected for modification by BB.

`crates/prw-agent/src/main.rs` is not selected for modification by BB.

Any need to widen this path set requires a new contradiction/audit before mutation.

## 17. Future materialization test expectations

A future source checkpoint should prove, without activating production remote networking, at least:

- lifecycle observation delegates to the retained transport observation rather than caller bind input;
- successful observation returns the exact `SocketAddr` unchanged;
- lower observation failure maps to exactly `RemoteSessionEndpointBoundAddressError::Unavailable`;
- the lifecycle accessor borrows and leaves lifecycle ownership intact;
- no `ConnectivityEndpoint`/candidate is created by the observation path;
- public method shape remains read-only and synchronous;
- no second runtime/task/retry/rebind path is introduced.

Tests must not read production systemd credentials or perform provider mutation.

If direct lower-endpoint behavior is already covered by the transport crate, BA does not require duplicating network integration tests merely to prove the Agent projection.

## 18. Explicit non-goals

C03e-BA does not select or authorize:

- Rust source materialization;
- executable invocation from `main.rs`;
- production remote input assembly;
- production bind address/source;
- expected-device request producer;
- concrete `CapabilityDispatcher`;
- registry/policy population;
- session/timing source;
- reachability candidate publication;
- provider state mutation;
- NAT/STUN/ICE activation;
- readiness changes;
- new process-signal ownership;
- remote failure -> local fail-stop policy;
- retry/reconnect/rebootstrap/rebind;
- arbitrary public/LAN listener widening;
- firewall/route/TUN/TAP/SOCKS/UDP forwarding expansion;
- systemd unit mutation;
- host mutation;
- deployment;
- recovery/PRWF/R1-R4 activation;
- merge.

## 19. Selection verdict

C03e-BA selects one minimal missing observation boundary:

> expose the exact local `SocketAddr` of the already-bound existing remote endpoint through a read-only Agent lifecycle accessor, preserving all identity, authority, connectivity, bind-policy, publication, readiness and lifecycle gates.

This is intentionally narrower than choosing how the Agent binds, how it discovers externally reachable addresses, or how it publishes a current transient endpoint.

Gate target remains:

`C03E_BA_REMOTE_ENDPOINT_BOUND_ADDRESS_OBSERVATION_SELECTED`
