# Phase 152 C03e-BA — Remote Endpoint Bound-Address Observation Selection Corrective Addendum

Status: CORRECTIVE / AUTHORITATIVE FOR C03e-BA CLOSURE

Gate target remains: `C03E_BA_REMOTE_ENDPOINT_BOUND_ADDRESS_OBSERVATION_SELECTED`

Exact predecessor remains closed C03e-AZ `0525c546c575857571a55c8cdba2fb74c5e3e288` / tree `bc60150698ccf35eed50d242dd69ecafaa8fb0b9`.

This addendum corrects one audit-basis inconsistency discovered by readback of the exact C03e-BA candidate tree. It does not change the selected product boundary, materialize Rust source, select production bind policy, publish reachability, activate `main.rs`, or mutate provider/host state.

## 1. Exact contradiction

The original C03e-BA selection contract states that `AgentRemoteTransportRuntime` owns the lower transport but exposes no bound-address getter, and therefore describes a future crate-private transport-layer delegation as part of the projected observation chain.

That statement is not true for the exact inherited source at the C03e-BA candidate tree.

The canonical inherited file `crates/prw-agent/src/remote_transport_runtime.rs` already contains:

```text
pub fn local_addr(&self) -> Result<SocketAddr, AgentRemoteTransportBindError>
```

and that method already delegates directly to the retained `RemoteServerTransportRuntime::local_addr()` result.

Therefore the existing Agent transport getter is authoritative inherited source and must not be duplicated, renamed, narrowed, or rewritten merely to satisfy the wording of the original BA document.

## 2. Corrected selected source-of-truth chain

The corrected selected observation chain is:

`MeshQuicEndpoint::local_addr()`
→ existing remote-server transport projection
→ existing `AgentRemoteTransportRuntime::local_addr()`
→ future public `RemoteSessionEndpointLifecycleRuntime::bound_addr()` typed observation.

The exact lower observation remains the already-bound endpoint address. Caller bind input still must not substitute for that observation.

## 3. Corrected C03e-BB materialization bound

A future C03e-BB source-materialization checkpoint is now bounded to exactly:

1. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`;
2. `crates/prw-agent/src/remote_session_capability_runtime.rs`;
3. the C03e-BB materialization contract.

`crates/prw-agent/src/remote_transport_runtime.rs` is explicitly excluded from normal BB mutation because the required read-only transport observation already exists.

Any future need to modify that transport file requires a new concrete contradiction and a separately recorded scope decision before mutation.

## 4. Public error selection unchanged

The BA-selected public lifecycle error remains exactly:

```text
RemoteSessionEndpointBoundAddressError::Unavailable
```

A future lifecycle accessor must map any existing lower Agent transport-address error to that bounded public class. The lower `AgentRemoteTransportBindError` is not promoted as the new lifecycle observation API.

## 5. Identity, authority, connectivity and publication separation unchanged

This correction does not change any BA invariant:

- `DeviceId` / authenticated PRW session identity remains logical identity;
- `TransportIdentity` remains lower-transport certificate identity;
- `SocketAddr` remains transient network endpoint/configuration state only;
- `SessionId` remains authentication correlation only;
- bound-address observation is not authentication, authorization, readiness, live-owner authority, provider-currentness or publication evidence;
- no `SocketAddr -> ConnectivityEndpoint` conversion is selected;
- no connectivity candidate, NAT/STUN/ICE state, provider mutation, discovery advertisement or readiness publication is selected.

## 6. Lifecycle and activation bounds unchanged

The future lifecycle accessor remains synchronous and read-only, borrows `&self`, and must not consume or mutate the lifecycle owner.

No retry, reconnect, rebind, alternate bind, second runtime, task spawn, hard abort, timeout, endpoint close, process failure policy, `main.rs` wiring, systemd mutation, host mutation, deployment or merge is authorized by this correction.

## 7. Focused future proof corrected

C03e-BB focused tests should prove without production networking that:

- lifecycle observation delegates to the existing retained Agent transport observation;
- a successful lower observation is returned byte/field-equivalent as the exact `SocketAddr`;
- any lower observation error maps to exactly `RemoteSessionEndpointBoundAddressError::Unavailable`;
- the lifecycle accessor is synchronous, borrows `&self`, and leaves lifecycle ownership intact;
- no connectivity candidate/publication/runtime/retry path is introduced.

No duplicate test is required merely to re-prove the already-existing lower transport getter.

## 8. Closure interpretation

For C03e-BA closure, this addendum is authoritative wherever the original selection contract conflicts with the exact inherited source state.

The selection verdict remains:

> expose the exact local `SocketAddr` of the already-bound existing remote endpoint through a read-only Agent lifecycle accessor, preserving all identity, authority, connectivity, bind-policy, publication, readiness and lifecycle gates.

The correction only removes an unnecessary future lower-layer mutation from that plan.

Gate target remains:

`C03E_BA_REMOTE_ENDPOINT_BOUND_ADDRESS_OBSERVATION_SELECTED`
