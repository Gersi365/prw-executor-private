# Phase 152 C03e-BB — Remote Endpoint Bound-Address Observation Source Materialization

Status: STAGED

Gate target:
`C03E_BB_REMOTE_ENDPOINT_BOUND_ADDRESS_OBSERVATION_SOURCE_MATERIALIZED`

Exact predecessor: closed C03e-BA `521d8b95698bef2662be304e59a0e88a8cc3dd28` / tree `e404e3b37616b373963a053039a9b8b2bfa89d8d`.

This checkpoint materializes only the corrected BA-selected read-only lifecycle observation of the exact local `SocketAddr` reported by the already-bound existing remote endpoint.

It does not select or activate a production bind-address source, connectivity-candidate conversion, reachability publication, endpoint retry/rebind, executable invocation, readiness, systemd/host mutation, deployment or merge.

## 1. Corrected inherited source state

C03e-BA closure recorded one audit-basis correction: the exact inherited source already contains the required lower Agent transport observation.

The existing source chain through BA is:

`MeshQuicEndpoint::local_addr()`
→ existing remote-server transport observation
→ existing `AgentRemoteTransportRuntime::local_addr()`.

Therefore BB must not duplicate or rewrite the lower Agent getter merely to match the wording of the original BA selection document.

`crates/prw-agent/src/remote_transport_runtime.rs` is explicitly outside normal BB mutation scope and must remain byte-stable unless a new concrete contradiction is recorded before mutation.

## 2. Exact BB source scope

The BA-corrected BB source scope is exactly three paths:

1. this contract;
2. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`;
3. `crates/prw-agent/src/remote_session_capability_runtime.rs`.

No other repository path is authorized by BB.

## 3. Materialized public observation

`RemoteSessionEndpointLifecycleRuntime` exposes exactly one new synchronous read-only method:

```text
pub fn bound_addr(&self) -> Result<SocketAddr, RemoteSessionEndpointBoundAddressError>
```

The method:
- borrows `&self`;
- delegates to the retained existing `AgentRemoteTransportRuntime::local_addr()` observation;
- returns the exact successful lower `SocketAddr` unchanged;
- does not consume, replace, close or mutate the endpoint lifecycle owner;
- performs no bind, retry, reconnect, rebootstrap, rebind, provider I/O, discovery, DNS resolution, STUN/ICE operation, readiness publication or task/runtime construction.

The caller's original bind input is not retained as an authoritative substitute. The value comes from the already-bound retained transport observation.

## 4. Materialized public failure class

BB materializes the BA-selected stable error:

```text
RemoteSessionEndpointBoundAddressError::Unavailable
```

Any lower `AgentRemoteTransportRuntime::local_addr()` error maps to exactly this public lifecycle observation class.

The lower `AgentRemoteTransportBindError` is not re-exported as the lifecycle observation API and BB does not widen its credential/transport details into a new public contract.

Observation failure:
- returns synchronously;
- does not request shutdown;
- does not close the endpoint;
- does not cancel workers;
- does not retry or rebind;
- does not mutate reachability authority;
- does not change readiness or process-exit policy.

## 5. Parent module surface

`crates/prw-agent/src/remote_session_capability_runtime.rs` re-exports `RemoteSessionEndpointBoundAddressError` beside the existing endpoint-lifecycle public types.

No other parent-module behavior changes.

## 6. Identity and authorization separation

BB preserves the existing identity model exactly:

- `DeviceId` / authenticated PRW session identity remains logical identity;
- `TransportIdentity` remains lower-transport certificate identity only;
- `SocketAddr` remains transient network endpoint/configuration state only;
- `SessionId` remains authentication correlation only;
- runtime/task/thread/controller/channel/lock/endpoint identifiers remain implementation details only.

A successful bound-address observation is never:
- account/session authentication;
- authorization;
- fresh-current registry/policy evidence;
- reachability-authority ownership/currentness evidence;
- readiness evidence;
- proof that the observed address is externally routable or publication-ready.

## 7. Connectivity and publication separation

BB does not materialize:
- `SocketAddr -> ConnectivityEndpoint` conversion;
- `ConnectivityCandidate` creation;
- NAT mapping inference;
- STUN/ICE/TURN state;
- interface enumeration;
- DNS resolution;
- wildcard-address expansion;
- reachability provider mutation;
- etcd/Spanner/registry publication;
- discovery advertisement;
- path selection or relay selection.

A wildcard observed address remains the exact local observation and is not silently transformed into a candidate.

## 8. Bind-policy separation

BB does not select:
- fixed versus ephemeral production port policy;
- IPv4 versus IPv6 production exposure policy;
- wildcard versus interface-specific production bind policy;
- privileged port policy;
- listener retry/backoff/fallback policy;
- multi-address or multi-endpoint policy.

Those remain separately gated.

## 9. Lifecycle and process exclusions

BB does not change:
- executor ownership;
- endpoint startup ordering;
- reachability-authority custody;
- supervisor-shutdown ownership;
- worker admission/cancellation/collection;
- endpoint close/idle-drain semantics;
- Linux signal ownership;
- local IPC lifecycle;
- local readiness;
- Agent `main.rs`;
- process-exit policy.

No new Tokio runtime, runtime handle, `block_on`, task spawn, OS thread, hard abort or hard deadline is introduced.

## 10. Focused non-networking proof

Focused tests in the lifecycle module must prove without production network operations that:

1. a successful synthetic lower observation returns the exact `SocketAddr` unchanged;
2. a synthetic lower error maps to exactly `RemoteSessionEndpointBoundAddressError::Unavailable`;
3. the public `bound_addr` method has the selected synchronous `&self` signature;
4. the mapping helper performs no retry or secondary operation;
5. existing production constructor/bootstrap signature assertions remain unchanged.

The focused tests must not invoke real credential reads, TLS construction, socket bind, endpoint startup, reachability provider I/O, discovery, file/terminal/forwarding capability side effects, systemd mutation or host mutation.

## 11. Validation gate

BB cannot close until the exact final head passes the repository's canonical Rust validation including:
- locked dependency graph;
- rustfmt;
- Clippy with warnings denied;
- workspace tests;
- workspace build.

If Android validation is triggered on the exact final source head, its native adapter/application terminal result must also be recorded before closure. No Android PASS may be claimed if no exact-final-head Android workflow exists.

Any corrective head invalidates older workflow results for closure evidence.

## 12. Drive evidence ordering

Closure requires:
1. immutable BB audit upload and raw byte-exact readback;
2. fresh rolling `C02E_BRANCH_STATUS.md` predecessor guard;
3. append-only rolling closure with exact predecessor-prefix preservation;
4. rolling raw byte-exact readback;
5. only then PR body transition from `Status: STAGED` to `Status: CLOSED` while remaining draft/open/unmerged;
6. final GitHub race checks.

## 13. Explicit non-authorization

C03e-BB does not authorize:
- production bind-address source selection;
- production expected-device/discovery source;
- concrete production capability side effects;
- registry/policy persistence/watch/mutation;
- account enrollment/authentication mutation;
- readiness changes;
- executable activation;
- `main.rs` wiring;
- systemd or host configuration mutation;
- firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart;
- recovery/PRWF initialization;
- R1-R4 activation;
- merge.

Gate target remains:

`C03E_BB_REMOTE_ENDPOINT_BOUND_ADDRESS_OBSERVATION_SOURCE_MATERIALIZED`
