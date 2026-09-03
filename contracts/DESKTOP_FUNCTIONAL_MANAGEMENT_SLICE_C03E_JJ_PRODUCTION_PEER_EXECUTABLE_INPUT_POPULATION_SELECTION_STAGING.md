# C03e-JJ — Production Peer Executable Input Population Selection

Status: `STAGED_SELECTION`

Gate on successful closure:
`C03E_JJ_PRODUCTION_PEER_EXECUTABLE_INPUT_POPULATION_BOUNDARY_SELECTED`

Closure on successful validation/audit:
`CLOSED_PRODUCTION_PEER_EXECUTABLE_INPUT_POPULATION_SELECTION`

## 1. Exact predecessor

This checkpoint is based only on the exact closed C03e-JI head:

`6237fa2a9c9a5c7a704b64db4bc9ab13d27cd461`

Exact predecessor tree:

`dabd9863f42701135493535610cd1d957e0ef529`

Exact predecessor `crates/prw-agent/src/linux_bootstrap.rs` blob:

`94a7e01671c34723d2b351e33124df5eb9c1492c`

Exact predecessor `crates/prw-agent/src/production_durable_registry_custody_bootstrap.rs` blob:

`e27e4d0a34ded1002efcb14a5c9844560a2c8bf1`

Exact predecessor `crates/prw-agent/src/production_durable_registry_runtime_custody.rs` blob:

`fd78512a24b824483a101962c3c63d91ad4b2cc1`

Exact predecessor `crates/prw-registry/src/durable_registry_etcd_store.rs` blob:

`1e04b366471fe2d4433de3c383efb4108d828983`

Exact predecessor `crates/prw-agent/src/main.rs` blob:

`db6b8028c6df100a961a0fb5818347bea2fdc5c1`

JI gate:
`C03E_JI_PRODUCTION_PEER_LOGICAL_DEVICE_ENV_SOURCE_MATERIALIZED`

JI closure:
`CLOSED_PRODUCTION_PEER_LOGICAL_DEVICE_ENV_SOURCE_MATERIALIZATION`

JI immutable Drive audit:
`1xYiTMgdAHSVRvv3MsvSZ5DLQU2TnSeyc`

JI materializes only the JH-selected fixed non-secret process logical-peer source. It returns one typed `DeviceId` and deliberately does not call the durable registry, construct a production peer, populate the existing production reachability input owner, or activate an executable caller.

## 2. Why JJ is the next bounded seam

At the exact JI head, the source now contains all individual mechanisms required to derive one production peer input, but they are not joined:

1. `load_linux_agent_remote_peer_device_id_from_env()` returns one typed process-selected `DeviceId`;
2. `bootstrap_production_durable_registry_from_systemd_credentials()` returns one production `DurableRegistryEtcdStore` after existing credential/provider bootstrap;
3. `ProductionDurableRegistryRuntimeCustody::from_store(...)` adapts that store into the existing private Agent runtime custody;
4. `ProductionDurableRegistryRuntimeCustody::peer_connectivity_identity(device_id)` resolves the exact logical device through current durable-registry transport authority;
5. `LinuxAgentProductionReachabilityRemoteProcessOperationInputs::new(peer, remote_process_inputs)` already owns one typed `PeerConnectivityIdentity` beside already-built remote-process inputs.

No current source seam joins those existing mechanisms for executable-production peer input population.

JJ therefore selects only that missing population boundary. It does not materialize it, does not construct the remaining production inputs, and does not select any executable caller.

## 3. Existing process logical-peer loader remains authoritative for intent

The exact JI source exposes:

```text
load_linux_agent_remote_peer_device_id_from_env()
 -> Result<DeviceId, LinuxAgentRemotePeerDeviceSourceError>
```

and reads only:

```text
PRW_REMOTE_PEER_DEVICE_ID
```

The JH/JI law remains unchanged:

- this value is fixed non-secret process configuration;
- it represents only logical remote/process peer intent;
- it is read through the existing Agent-owned fixed-name loader;
- the exact Unicode value is passed to existing `DeviceId::new` semantics;
- no trimming, normalization, case conversion, delimiter interpretation or alternate source is added;
- no configured value is echoed through the bounded source error surface.

JJ does not modify or reinterpret the loader.

## 4. Environment intent is not production peer authority

The selected process `DeviceId` is not by itself:

- a current `PeerConnectivityIdentity`;
- a current `TransportIdentity`;
- proof that the device exists;
- proof that the device is enrolled and participating;
- proof that the device remains non-revoked and transport-bound;
- policy or workspace authorization;
- endpoint/reachability authority;
- requester/rendezvous authority;
- authenticated-session authority;
- certificate or signer authority.

JJ preserves the established provenance chain:

```text
fixed process logical DeviceId intent
 -> current durable-registry same-device transport authority
 -> typed PeerConnectivityIdentity
 -> existing production reachability input owner
```

No earlier value may bypass the durable-registry step.

## 5. Existing durable-registry production bootstrap remains authoritative

The exact JI source exposes:

```text
bootstrap_production_durable_registry_from_systemd_credentials()
 -> Result<DurableRegistryEtcdStore,
           ProductionDurableRegistryCustodyBootstrapError>
```

Calling this existing async facade:

1. loads the fixed production durable-registry systemd credential set through the existing custody layer;
2. validates bounded opaque provider bootstrap configuration;
3. performs the existing one-shot production provider bootstrap;
4. returns the existing semantic `DurableRegistryEtcdStore`;
5. performs no registry semantic Get/Txn/Put operation itself.

JJ selects no new endpoint, credential directory, trust root, certificate, private key, provider client, raw etcd client, retry policy or fallback provider.

## 6. Existing runtime custody adaptation remains authoritative

The exact JI source exposes:

```text
ProductionDurableRegistryRuntimeCustody::from_store(
    store: DurableRegistryEtcdStore,
) -> ProductionDurableRegistryRuntimeCustody
```

This constructor is side-effect-free and consumes the exact already-bootstrapped semantic store by value.

JJ selects use of this existing custody owner rather than:

- exposing the raw provider executor;
- adding a generic store getter;
- cloning the semantic store;
- introducing global registry state;
- moving registry lookup logic into `linux_bootstrap.rs`;
- bypassing the Agent-owned runtime custody abstraction.

## 7. Existing JF current-peer lookup remains authoritative

The exact JI source exposes the current JF operation:

```text
ProductionDurableRegistryRuntimeCustody::peer_connectivity_identity(
    &mut self,
    device_id: DeviceId,
) -> Result<PeerConnectivityIdentity, DurableRegistryEtcdStoreError>
```

Its exact current implementation obtains:

```text
store.current_transport_identity(&device_id)
```

and only after that authoritative read succeeds constructs:

```text
PeerConnectivityIdentity::new(device_id, current_transport)
```

The underlying `current_transport_identity(...)` performs one exact current device read and preserves semantic unknown/revoked/unbound state plus provider/currentness authority failures through `DurableRegistryEtcdStoreError`.

JJ selects this existing operation exactly once. It does not select a generic registry lookup, scan, cache, stale record, alternate device, alternate transport source or direct environment-to-peer construction.

## 8. Existing production reachability input owner remains authoritative

The exact JI `linux_bootstrap.rs` already contains:

```text
LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P, D, T, F, C, R, E>
```

with exact fields:

```text
peer: PeerConnectivityIdentity
remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>
```

and existing constructor:

```text
LinuxAgentProductionReachabilityRemoteProcessOperationInputs::new(
    peer,
    remote_process_inputs,
)
```

JJ selects no field addition, removal, visibility change, clone path or new aggregate owner.

## 9. Selected future helper

JJ selects one future crate-private async helper in:

`crates/prw-agent/src/linux_bootstrap.rs`

named exactly:

`linux_agent_production_reachability_remote_process_operation_inputs_from_production_peer`

Its sole responsibility is to consume one already-built existing:

```text
LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>
```

and populate only the existing production `peer` field through the existing JI/JB/JD/JF provenance chain.

Selected semantic shape:

```text
async fn linux_agent_production_reachability_remote_process_operation_inputs_from_production_peer<
    P, D, T, F, C, R, E,
>(
    remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
) -> Result<
    LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
    LinuxAgentProductionPeerInputPopulationError,
>
```

The helper remains crate-private and dormant. JJ does not select any callsite for it.

## 10. Exact population order

When a later separately gated caller invokes the selected helper, the successful sequence must be exactly:

```text
load_linux_agent_remote_peer_device_id_from_env()
 -> device_id
 -> bootstrap_production_durable_registry_from_systemd_credentials().await
 -> store
 -> ProductionDurableRegistryRuntimeCustody::from_store(store)
 -> runtime_custody
 -> runtime_custody.peer_connectivity_identity(device_id).await
 -> peer
 -> LinuxAgentProductionReachabilityRemoteProcessOperationInputs::new(
        peer,
        remote_process_inputs,
    )
```

The helper must:

1. load the process logical `DeviceId` exactly once;
2. perform no durable-registry/provider bootstrap when that source load fails;
3. call the existing production durable-registry bootstrap exactly once after source success;
4. adapt the returned exact store through `ProductionDurableRegistryRuntimeCustody::from_store(...)` exactly once;
5. call current JF `peer_connectivity_identity(device_id)` exactly once;
6. perform no production reachability recovery when peer lookup fails;
7. construct the existing `LinuxAgentProductionReachabilityRemoteProcessOperationInputs::new(...)` exactly once only after all provenance stages succeed;
8. move the existing `remote_process_inputs` unchanged into the existing owner;
9. return the constructed owner without invoking any operation factory or runtime.

## 11. Selected bounded failure envelope

Unlike IL/IM's single-source bind-address helper, this population boundary joins three already-existing fallible stages and therefore requires one Agent-local bounded composition error.

JJ selects one crate-private error equivalent to:

```text
LinuxAgentProductionPeerInputPopulationError
```

with exactly three semantic classes:

```text
PeerDeviceSource(LinuxAgentRemotePeerDeviceSourceError)
DurableRegistryBootstrap(ProductionDurableRegistryCustodyBootstrapError)
DurableRegistryLookup(DurableRegistryEtcdStoreError)
```

The future materialization may use equivalent variant names only if compilation/style requires a mechanical naming adjustment; it must not collapse or widen the three failure stages.

The error surface must:

- preserve the exact underlying typed error as `source()` where available;
- avoid echoing configured peer identifiers, provider endpoints, credentials, certificate/private-key material or raw provider responses;
- add no retry/fallback/degraded-success state;
- remain distinct from `LinuxAgentBootstrapStartFailure` and executable exit policy.

## 12. Fail-before-next-stage law

Failure ordering is part of the selected contract.

```text
peer-device source failure
 -> return PeerDeviceSource(...)
 -> zero durable-registry bootstrap
 -> zero registry read
 -> zero owner construction
```

```text
durable-registry bootstrap failure
 -> return DurableRegistryBootstrap(...)
 -> zero registry semantic read
 -> zero peer construction
 -> zero owner construction
```

```text
JF current-peer lookup failure
 -> return DurableRegistryLookup(...)
 -> zero fallback peer
 -> zero reachability recovery
 -> zero owner construction
```

No later stage may run after an earlier stage fails.

## 13. No fallback, retry or alternate peer

The selected helper must not:

- re-read `PRW_REMOTE_PEER_DEVICE_ID`;
- select another environment variable;
- choose another registry device;
- scan the durable registry for a usable peer;
- reuse a stale/cached transport identity;
- read a transport identity from environment or config;
- derive transport identity from certificate, signer, endpoint, IP address or request data;
- use requester/rendezvous target intent as the process peer;
- use the first/next expected admission request as the process peer;
- retry provider bootstrap with another endpoint/configuration;
- substitute an in-memory/test registry;
- fabricate a `PeerConnectivityIdentity` after a failed registry read.

The boundary is fail-closed.

## 14. Point-in-time currentness and custody lifetime

A successful JF lookup establishes one point-in-time current production peer consisting of:

- the exact process-selected logical `DeviceId`; and
- the durable registry's current transport identity for that same device at lookup time.

After the typed `PeerConnectivityIdentity` is constructed and moved into the existing production reachability input owner, the temporary durable-registry runtime custody used only for this population transaction need not be retained by that owner.

JJ does not select:

- background registry watching;
- transport refresh;
- automatic re-key;
- hidden owner replacement;
- retry/rebootstrap on later transport rotation;
- registry polling during an active reachability owner lifecycle.

Any automatic reaction to later transport rotation requires a separate lifecycle checkpoint.

## 15. No mutation of durable-registry authority

The selected JF path is read-only.

JJ does not authorize:

- membership creation/suspension/removal;
- device registration/revocation;
- transport bind/rotation;
- registry transaction mutation;
- production registry population;
- provider reconciliation;
- Watch/lease/TTL additions.

The future helper may only obtain current peer authority through the existing read path.

## 16. No reachability bootstrap or network activation

JJ stops after construction of the existing production reachability input owner.

The selected helper must not invoke:

```text
linux_agent_production_reachability_remote_process_operation(...)
```

or:

```text
RemoteSessionExecutorRuntime::bootstrap_production_reachability_runtime_custody_from_systemd_credentials(...)
```

and must not:

- create a remote executor;
- recover durable reachability custody;
- bind a remote endpoint;
- listen or accept;
- publish readiness;
- publish candidates;
- traverse NAT;
- dial peers;
- consume expected admission requests;
- create or drive a remote session lifecycle.

Provider I/O selected here is limited to the existing durable-registry bootstrap plus one exact current-peer registry read if the future helper is explicitly invoked by a separately gated caller.

## 17. Existing requester/rendezvous custody remains untouched

JJ does not construct, mutate or invoke:

```text
BoundedRequesterRendezvousStartPolicySource
CandidatePublicationRequesterRendezvousRuntimeOwner
LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<...>
```

and does not select:

- requester-aware policy evaluation;
- requester/rendezvous provider capacity;
- provider registration or cleanup;
- target-intent ingestion;
- candidate publication;
- traversal or dialing.

The existing requester/rendezvous aggregate remains a later composition layer.

## 18. Existing lower-level remote inputs remain caller custody

JJ does not select new production sources for any field already contained in:

```text
LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>
```

That includes:

- `bind_addr` beyond the already-materialized IM seam;
- `max_active_workers`;
- capability authority;
- session-authentication state;
- expected-request producer/channel lifecycle;
- dispatcher/verifier-time ownership carried by expected requests;
- admission timing;
- completion callback;
- rejection callback;
- repeated-admission-failure callback.

The future JJ successor consumes one already-built lower-level owner by value and changes none of its contents.

## 19. Existing operation factories and executable assembly remain frozen

JJ selects no mutation or invocation of:

```text
linux_agent_remote_process_operation(...)
linux_agent_production_reachability_remote_process_operation(...)
linux_agent_production_reachability_requester_rendezvous_remote_process_operation(...)
run_with_production_reachability_requester_rendezvous_remote_process_companion(...)
run_with_remote_process_companion(...)
```

The selected population helper returns data ownership only.

No closure is built or executed by the population boundary.

## 20. `run()` and `main.rs` remain frozen

At exact JI head:

- `linux_bootstrap::run()` still enters only the existing local signal-aware runtime;
- `main.rs` still performs the existing device-identity signer preflight and then calls only `prw_agent::linux_bootstrap::run()`.

JJ preserves both paths exactly.

The first source successor after JJ must not modify `run()` or `main.rs` and must not make the selected helper reachable from service startup.

## 21. First source-successor ceiling

After JJ closes, the first source-materialization successor is authorized to modify exactly one source file:

`crates/prw-agent/src/linux_bootstrap.rs`

Authorized source change is limited to:

1. imports required only for the existing production durable-registry bootstrap/custody/error types;
2. one crate-private bounded `LinuxAgentProductionPeerInputPopulationError` carrying exactly the three selected stage failures;
3. bounded `Display`/`Error`/conversion plumbing for that error without sensitive value disclosure;
4. one crate-private async helper named exactly `linux_agent_production_reachability_remote_process_operation_inputs_from_production_peer`;
5. one exact existing JI peer-device loader call;
6. one exact existing production durable-registry bootstrap call;
7. one exact `ProductionDurableRegistryRuntimeCustody::from_store(...)` adaptation;
8. one exact existing JF current-peer lookup call;
9. one exact existing `LinuxAgentProductionReachabilityRemoteProcessOperationInputs::new(...)` construction after success;
10. focused source-shape/signature/error-mapping tests that do not require real production credential/provider/registry I/O;
11. local lint-shape acknowledgement only if required by the exact helper signature.

The successor must not modify any other source path.

If implementation requires a second source file, new public API, new registry/provider abstraction, new owner, retry/fallback/cache, reachability bootstrap, requester/rendezvous population or executable invocation, it must stop and select another checkpoint.

## 22. Test and validation obligations

The first source successor must prove, without production mutation:

1. the helper success type is exactly the existing `LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P, D, T, F, C, R, E>`;
2. the helper consumes exactly one existing `LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>`;
3. the bounded composition error retains distinct peer-source, registry-bootstrap and registry-lookup classes;
4. source inspection shows the selected one-time loader/bootstrap/custody/lookup/constructor sequence;
5. no fallback/retry/alternate peer path exists;
6. `run()`, `main.rs`, operation factories and requester/rendezvous aggregate assembly remain unchanged;
7. no second source path changes;
8. workspace formatting, lint, tests and build remain green.

Tests must not mutate process-global environment solely to exercise the production helper and must not require real systemd credentials, etcd access or production registry data. Existing JI parser tests, JB bootstrap tests, JD custody-constructor tests, JF semantic-store tests and source-shape inspection remain authoritative for the individual stages.

## 23. Explicit non-authorization

C03e-JJ does not authorize or perform:

- Rust source materialization itself;
- environment or service-unit configuration mutation;
- production durable-registry bootstrap or registry I/O during this selection checkpoint;
- production registry data mutation;
- reachability custody recovery;
- endpoint bind/listen/readiness activation;
- remote worker/session execution;
- requester/rendezvous invocation;
- expected-request production or consumption;
- candidate publication;
- NAT traversal;
- peer dialing;
- capability/session authority production population;
- worker-limit production source selection;
- full production aggregate input construction;
- executable caller activation;
- `run()` or `main.rs` mutation;
- systemd/package/security provisioning;
- credential/certificate/private-key/trust/RBAC mutation;
- deployment/restart;
- repository visibility/configuration mutation;
- merge, PR close, ready-for-review transition, branch deletion or history rewrite.

The JJ PR must remain draft/open/unmerged after closure.

## 24. Closure meaning and stop condition

C03e-JJ closes only:

`PRODUCTION_PEER_EXECUTABLE_INPUT_POPULATION_BOUNDARY_SELECTED`

It selects one exact fail-closed composition boundary that converts already-materialized process logical-peer intent into the existing typed production reachability peer input through existing production durable-registry current same-device authority.

No source is materialized and no runtime is activated by JJ.

After JJ closes, the next authorized step is only the one-file source-materialization successor within Section 21.

After that source successor closes, work must stop again and select the next still-missing executable-production provenance seam from the exact resulting source state. No assumption is made that capability/session authority, worker-limit source, expected-request production, requester/rendezvous provider population, full aggregate assembly or executable activation is automatically next.