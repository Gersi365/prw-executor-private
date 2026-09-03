# Phase 152 C03e-JE — Production Peer Connectivity Identity Runtime Custody Lookup Boundary Selection

Status: `STAGED_SELECTION`

Gate on closure:
`C03E_JE_PRODUCTION_PEER_CONNECTIVITY_IDENTITY_RUNTIME_CUSTODY_LOOKUP_BOUNDARY_SELECTED`

Canonical closure token:
`CLOSED_PRODUCTION_PEER_CONNECTIVITY_IDENTITY_RUNTIME_CUSTODY_LOOKUP_BOUNDARY_SELECTION`

## 1. Scope

C03e-JE is a documentation-only prerequisite after closed C03e-JD.

It selects the first operation-specific use of the Agent-owned durable-registry runtime custody: authoritative production `PeerConnectivityIdentity` lookup for one explicitly supplied logical `DeviceId`.

C03e-JE does not materialize Rust source, call the provider, read production records, populate an executable input owner, wire startup/readiness/runtime, create a background task, change provider/security configuration, deploy, merge, close a pull request, mark a pull request ready, or delete a branch.

Exact predecessor C03e-JD head:

`c8ae7dc5d8295002488ddeb3a7bba578029b9346`

## 2. Exact predecessor custody state

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

Exact C03e-JD blob:

`a86faeb43becbd49b6380e85be8bb40357d94bbe`

C03e-JD materialized only:

```text
ProductionDurableRegistryRuntimeCustody {
    store: DurableRegistryEtcdStore
}
```

with a by-value side-effect-free constructor:

```text
ProductionDurableRegistryRuntimeCustody::from_store(store)
```

The semantic store is private. There is no generic store getter, executor getter, service locator, readiness hook, startup hook, or registry operation method.

C03e-JD explicitly requires future registry use to be operation-specific and separately gated.

## 3. Exact semantic authority already materialized

`crates/prw-registry/src/durable_registry_etcd_store.rs`

At the exact C03e-JD head the semantic durable-registry store already exposes:

```text
current_transport_identity(
    &mut self,
    device_id: &DeviceId,
) -> Result<TransportIdentity, DurableRegistryEtcdStoreError>
```

This method performs one authoritative current-device lookup and returns a transport identity only when the requested logical device is current, enrolled and transport-bound.

The existing semantic failure surface preserves at least:

- unknown device;
- revoked/non-enrolled device;
- absent transport binding;
- provider read unavailable;
- invalid durable authority.

It does not infer identity from an endpoint, request, IP address, process configuration, environment value, or another device.

## 4. Exact peer identity type

`crates/prw-connectivity/src/lib.rs`

At the exact C03e-JD head:

```text
PeerConnectivityIdentity::new(
    device: DeviceId,
    transport: TransportIdentity,
) -> PeerConnectivityIdentity
```

is a pure typed constructor.

`DeviceId` and `TransportIdentity` remain distinct identities. The constructor does not prove provenance; provenance must be established before construction.

## 5. C03e-IN provenance law remains controlling

Exact retained contract:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_IN_PRODUCTION_PEER_CONNECTIVITY_IDENTITY_PROVENANCE_SELECTION_STAGING.md`

Exact blob:

`979c5fc9cc088b32ed0af5677841627fdc2644f4`

C03e-IN selected that production `PeerConnectivityIdentity` must originate from one authoritative current same-device binding:

```text
requested logical DeviceId
    -> authoritative current device record
    -> exact current transport bound to that same DeviceId
    -> PeerConnectivityIdentity::new(device_id, current_transport)
```

No endpoint, bind address, IP/port, candidate, request ID, session ID, expected-device scheduling hint, transport identity alone, environment literal, or fixture may substitute for the authoritative same-device binding.

The C03e-IP through C03e-JD chain has now materialized the durable authority source and private Agent custody needed to satisfy this earlier gap without exposing a generic provider/store handle.

## 6. Selected operation-specific runtime custody method

C03e-JE selects one and only one first operation-specific method on:

`ProductionDurableRegistryRuntimeCustody`

with a shape equivalent to:

```text
async fn peer_connectivity_identity(
    &mut self,
    device_id: DeviceId,
) -> Result<PeerConnectivityIdentity, DurableRegistryEtcdStoreError>
```

The exact source checkpoint may choose an equivalent crate-private name, but the ownership and behavior below are fixed.

The method consumes the requested `DeviceId` by value so the exact same typed logical identity can be moved into the returned peer after authoritative validation.

## 7. Selected exact operation order

The method must perform exactly this semantic sequence:

1. receive one already-typed `DeviceId` from the caller;
2. call the privately held semantic store's existing `current_transport_identity(&device_id)` exactly once;
3. propagate the exact existing `DurableRegistryEtcdStoreError` without fallback, remapping to success, or retry;
4. on success only, construct exactly one `PeerConnectivityIdentity::new(device_id, current_transport)`;
5. return that peer.

No second device read, separate transport source, prefix scan, cache lookup, environment read, or provider client access is selected.

## 8. Fail-closed law

The operation returns no peer when the semantic store reports any failure, including:

- device unknown;
- device revoked/non-enrolled;
- current transport absent;
- provider read unavailable;
- invalid durable authority;
- any future non-success variant of the bounded durable-registry semantic error surface.

A provider failure, malformed record, unknown device, revoked device, or unbound device cannot yield a fabricated or fallback peer.

## 9. Same-device binding law

The returned `PeerConnectivityIdentity` must contain:

- exactly the requested `DeviceId` passed into the method; and
- exactly the `TransportIdentity` returned by `current_transport_identity(&requested_device_id)`.

The method must not accept a caller-supplied transport identity.

The method must not substitute another logical device discovered from a record, session, request or provider response.

No `DeviceId`-only or `TransportIdentity`-only fallback is selected.

## 10. Currentness and rotation law

Every invocation performs a new authoritative current device lookup through the existing semantic store.

A peer value returned by one invocation is a point-in-time result. C03e-JE does not declare it permanently current after return.

A later transport rotation changes the authoritative current transport for subsequent invocations. C03e-JE selects no peer cache and no stale-peer reuse authority.

Callers that need freshness at a later lifecycle point must invoke the separately owned operation at that lifecycle point rather than treating an old peer as current authority by default.

## 11. Error ownership

C03e-JE introduces no new provider or registry error taxonomy.

The first source method returns the existing provider-neutral:

`DurableRegistryEtcdStoreError`

directly.

This preserves exact distinctions such as semantic failure, read unavailable, invalid authority and currentness/provider state without leaking raw `etcd_client::Error`.

No Agent-level error wrapper is required for this pure operation-specific delegation.

## 12. No generic durable-registry access escape

C03e-JE does not authorize:

- `store()` / `store_mut()`;
- `into_store()`;
- `executor()` / `into_executor()`;
- raw provider handle access;
- a generic closure/callback that receives `&mut DurableRegistryEtcdStore`;
- trait-object service-location access;
- global/static registry state.

The runtime custody remains an owner with exact operation-specific methods only.

## 13. Relation to existing production input owner

`crates/prw-agent/src/linux_bootstrap.rs`

At the exact C03e-JD head, the already-materialized crate-private owner:

```text
LinuxAgentProductionReachabilityRemoteProcessOperationInputs<...> {
    peer: PeerConnectivityIdentity,
    remote_process_inputs: ...
}
```

still requires an already-typed `PeerConnectivityIdentity`.

C03e-JE does not populate that field and does not modify `linux_bootstrap.rs`.

A later separately gated checkpoint must select how an exact logical `DeviceId` is supplied and when this operation-specific lookup result may be moved into that existing input owner.

## 14. No startup/runtime activation

The first source successor remains dormant and uncalled.

C03e-JE does not authorize changes to:

- `run()`;
- `main.rs`;
- Linux production bootstrap callsites;
- readiness state;
- listener lifecycle;
- requester/rendezvous execution;
- candidate publication;
- NAT traversal;
- peer dialing;
- worker/session lifecycle;
- background tasks;
- service restart/deployment.

## 15. No production registry mutation

The selected first operation is read-only.

It does not authorize:

- membership creation/suspension/removal;
- device registration/revocation;
- transport bind/rotation;
- production record population/migration;
- provider resource creation;
- RBAC/security mutation.

## 16. First source-materialization ceiling

After C03e-JE closure, the next separately gated source checkpoint may change only:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

No `lib.rs` change is expected because the runtime-custody module is already declared.

Allowed source delta only:

- imports needed for `DeviceId`, `PeerConnectivityIdentity`, and `DurableRegistryEtcdStoreError`;
- one crate-private operation-specific async method implementing the exact selected sequence;
- focused provider-free/type-shape tests where possible.

Still prohibited in that source checkpoint:

- generic store/executor getter;
- Cargo/Cargo.lock change;
- `linux_bootstrap.rs` change;
- runtime/startup callsite;
- provider/security/custody change;
- production record mutation;
- deployment.

## 17. Focused next-source validation matrix

The next source checkpoint must prove at least:

- exact method signature returns `PeerConnectivityIdentity` through the existing durable-store error surface;
- caller cannot provide a `TransportIdentity` separately;
- method uses the private store rather than exposing it;
- no generic getter/extraction method appears;
- method remains uncalled by executable/runtime source;
- no Cargo/lockfile/path expansion beyond the exact runtime-custody file.

Provider-error/unknown/revoked/unbound behavioral semantics are already covered by the semantic store and must remain propagated without alteration.

## 18. Explicit non-authorization

C03e-JE does not authorize or perform:

- Rust source materialization itself;
- production provider I/O;
- registry record mutation/population/migration;
- provider endpoint/security/RBAC/provisioning changes;
- credential/systemd changes;
- peer input-owner population;
- logical DeviceId source selection for the executable path;
- startup/readiness/runtime wiring;
- requester/rendezvous invocation;
- candidate publication/traversal/dialing;
- deployment/restart;
- repository visibility/configuration mutation;
- merge;
- PR close;
- ready-for-review transition;
- branch deletion/history rewrite.

## 19. Closure meaning

C03e-JE closure means only:

`PRODUCTION_PEER_CONNECTIVITY_IDENTITY_RUNTIME_CUSTODY_LOOKUP_BOUNDARY_SELECTED`

It selects the narrow operation-specific bridge from one requested logical `DeviceId` to the durable registry's authoritative current same-device transport binding and then to one typed `PeerConnectivityIdentity`.

It does not mean production peer population is wired into the executable path or that runtime activation is authorized.
