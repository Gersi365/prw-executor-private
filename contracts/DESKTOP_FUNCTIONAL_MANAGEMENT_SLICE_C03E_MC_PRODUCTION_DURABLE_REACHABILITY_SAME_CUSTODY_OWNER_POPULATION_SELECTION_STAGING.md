# C03e-MC — Production Durable-Reachability Same-Custody Owner Population Selection

Status: `SELECTION_STAGING`

Gate after exact-head validation and durable evidence closure:

`C03E_MC_PRODUCTION_DURABLE_REACHABILITY_SAME_CUSTODY_OWNER_POPULATION_SELECTED`

## 1. Purpose

C03e-MC is documentation-only. It selects the smallest later production-population seam for the pre-requester same-custody owner source-materialized by evidence-closed C03e-MB.

C03e-MC does not itself modify Rust/source/runtime behavior.

The selected later boundary must preserve the C03e-LZ one-custody provenance law while reusing the existing production worker-limit/bind population stage and existing logical-peer process source without invoking the legacy peer population helper that performs its own independent durable-registry bootstrap.

## 2. Exact predecessor authority

Exact predecessor checkpoint:

`C03e-MB — Production durable-reachability same-custody owner source materialization`

Exact predecessor branch:

`phase-152-c03e-mb-production-durable-reachability-same-custody-owner-source-materialization`

Exact predecessor head:

`ab6bd3f5baaf187518432232de2c9080c47068dc`

Exact predecessor tree:

`2f84f4c923cc56e65b0ebb8a2cd6272f7b9d64ec`

Exact MB owner source path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Exact MB owner source blob:

`3b1d1b3e11e0ff7278dd0bf12677e5ae9251dc60`

MB source-materialized exactly one crate-private dormant owner:

`LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>`

with two private by-value fields:

1. `LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>`;
2. raw `ProductionDurableCapabilityAuthority`.

MB intentionally added no constructor, accessor, splitter, clone/copy, tuple export, getter, generic extraction surface, population call or invocation site.

## 3. Fresh exact-source audit

### 3.1 Existing worker-limit/bind population stage

Exact MB `linux_bootstrap.rs` blob:

`7940a69e598355176a61b0bef5c7571dab9fb530`

Existing crate-private helper:

`linux_agent_remote_process_operation_inputs_from_production_worker_limit(...)`

already performs the selected production order:

1. load `PRW_REMOTE_MAX_ACTIVE_WORKERS` exactly once through `load_linux_agent_remote_max_active_workers_from_env()`;
2. after success, call `linux_agent_remote_process_operation_inputs_from_production_bind_addr(...)` exactly once;
3. return one existing `LinuxAgentRemoteProcessOperationInputs<P,D,T,F,C,R,E>`.

Its existing bounded error is:

`LinuxAgentProductionRemoteProcessInputPopulationError`.

No new worker-limit parsing, bind-address parsing, fallback, retry, host-derived default or duplicate `LinuxAgentRemoteProcessOperationInputs::new(...)` is selected.

### 3.2 Existing logical-peer process source

Existing source:

`load_linux_agent_remote_peer_device_id_from_env()`

loads the fixed process logical peer `DeviceId` without registry/provider I/O and without constructing transport identity.

The exact `DeviceId` remains process peer intent only. It is not a socket address, transport identity, capability authority, requester identity or authorization result.

### 3.3 Legacy peer population helper is not suitable for the new owner

Existing helper:

`linux_agent_production_reachability_remote_process_operation_inputs_from_production_peer(...)`

currently:

1. loads the fixed logical peer `DeviceId`;
2. calls `bootstrap_production_durable_registry_from_systemd_credentials()`;
3. creates a `ProductionDurableRegistryRuntimeCustody`;
4. resolves the current same-device `PeerConnectivityIdentity`;
5. constructs `LinuxAgentProductionReachabilityRemoteProcessOperationInputs`;
6. drops the remaining registry custody when returning.

Existing composite helper:

`linux_agent_production_reachability_remote_process_operation_inputs_from_production_worker_limit_and_peer(...)`

calls that legacy peer helper after worker-limit/bind population.

A future C03e-MB owner population must **not** call either of those peer/composite helpers, because a separate later durable-capability-authority bootstrap would then derive the retained durable authority from a different provider/runtime-custody lineage.

### 3.4 Existing C03e-LZ same-custody population seam

Exact source path:

`crates/prw-agent/src/production_durable_registry_custody_bootstrap.rs`

Exact MB blob:

`6ad990bf3b8e6536351e06d3b939370ed73c887e`

Existing helper:

`bootstrap_production_peer_and_durable_capability_authority_from_systemd_credentials(device_id)`

performs exactly one production durable-registry bootstrap, resolves exactly one current same-device `PeerConnectivityIdentity`, and then consumes the same surviving `ProductionDurableRegistryRuntimeCustody` into exactly one raw `ProductionDurableCapabilityAuthority`.

Its existing bounded error is:

`ProductionDurablePeerCapabilityAuthorityPopulationError`.

This helper is the only selected durable-registry/bootstrap authority for the future MB-owner population lane.

### 3.5 Requester/rendezvous production provenance remains unresolved

The concrete bounded requester/rendezvous policy source remains an in-memory backing whose module explicitly does not wire production custody or population.

`CandidatePublicationRequesterRendezvousRuntimeOwner::new(...)` consumes an already-configured in-memory provider whose capacity and lifecycle state are caller-owned decisions.

Synthetic tests construct explicit provider capacities and bounded policy sources. Those test values are not production provenance.

C03e-MC therefore remains strictly before requester/rendezvous join.

## 4. Selected future source ceiling

A later separately gated source-materialization checkpoint may modify exactly one Rust path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

No second repository path is selected.

The source successor may add one crate-private async population helper conceptually named:

`linux_agent_production_durable_reachability_remote_process_operation_inputs_from_production_sources(...)`

Mechanical spelling may change only to satisfy normal Rust naming/formatting while preserving this semantic boundary.

The helper returns exactly one:

`LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>`.

It adds no invocation site.

## 5. Selected population order

The future helper must preserve this exact fail-before-next-stage order:

1. Call existing `linux_agent_remote_process_operation_inputs_from_production_worker_limit(...)` exactly once using the already-typed injected capability/session/request/timing/callback inputs.
2. If that stage succeeds, call existing `load_linux_agent_remote_peer_device_id_from_env()` exactly once.
3. If that source succeeds, call existing `bootstrap_production_peer_and_durable_capability_authority_from_systemd_credentials(device_id)` exactly once.
4. Move the returned `PeerConnectivityIdentity` and exact already-built `LinuxAgentRemoteProcessOperationInputs` into `LinuxAgentProductionReachabilityRemoteProcessOperationInputs::new(...)` exactly once.
5. In the same higher-owner module, directly construct the existing MB owner with:
   - that exact production-reachability owner; and
   - the exact raw `ProductionDurableCapabilityAuthority` returned by the same LZ invocation.
6. Return the fully populated dormant MB owner.

If worker-limit/bind population fails, no peer-device source read and no durable-registry/provider bootstrap may occur.

If peer-device source loading fails, no durable-registry/provider bootstrap may occur.

If the LZ same-custody helper fails, no MB owner may be returned.

No partial/degraded owner is selected.

## 6. Same-custody law

For every successful future population:

- the `PeerConnectivityIdentity` embedded inside the retained `LinuxAgentProductionReachabilityRemoteProcessOperationInputs`; and
- the retained raw `ProductionDurableCapabilityAuthority`

must originate from the **same exact invocation** of:

`bootstrap_production_peer_and_durable_capability_authority_from_systemd_credentials(device_id)`.

The future helper must not call:

- `linux_agent_production_reachability_remote_process_operation_inputs_from_production_peer(...)`;
- `linux_agent_production_reachability_remote_process_operation_inputs_from_production_worker_limit_and_peer(...)`;
- `bootstrap_production_durable_capability_authority_from_systemd_credentials()`;
- `bootstrap_production_durable_registry_from_systemd_credentials()` directly.

No second provider bootstrap, authority substitution, peer substitution, alternate device, retry, fallback, cache, synthetic authority or degraded authority is selected.

## 7. Selected bounded error composition

The future source checkpoint may add exactly one crate-private composite error conceptually named:

`LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError`.

Its semantic variants are ceilinged to:

1. `RemoteProcessInputs(LinuxAgentProductionRemoteProcessInputPopulationError)`;
2. `PeerDeviceSource(LinuxAgentRemotePeerDeviceSourceError)`;
3. `SameCustodyPeerCapabilityAuthority(ProductionDurablePeerCapabilityAuthorityPopulationError)`.

The exact underlying error must remain available through `std::error::Error::source()`.

`Display` must remain bounded, stage-classifying and non-sensitive. It must not expose configured worker limits, bind addresses, logical device identifiers, provider endpoints, credentials, certificate/key material or durable-registry details.

Only minimal exact `From` plumbing required by this composition is selected.

No startup/exit mapping, retry policy, fallback policy, broader bootstrap error envelope or generic provider error abstraction is selected.

## 8. Input surface law

The future helper may accept exactly the already-typed injected inputs required by existing:

`linux_agent_remote_process_operation_inputs_from_production_worker_limit(...)`

namely the current capability authority, session authentication service, expected-request receiver, admission timing source and existing completion/rejection/admission-failure callbacks.

This selection does not authorize choosing or populating those injected sources.

Their production provenance remains separately gated.

The future helper must not add endpoint, provider, credential-directory, certificate, private-key, trust, raw etcd client, socket, IP, request-id, requester/rendezvous provider or arbitrary configuration arguments.

## 9. Ownership law

The future helper constructs the MB owner directly in the same module because the MB fields are private.

No general-purpose constructor, accessor, splitter, tuple export, clone/copy, authority getter, production-input getter or generic extraction surface is selected.

The raw `ProductionDurableCapabilityAuthority` remains raw inside the MB owner.

No new `Arc::new` or `Arc::clone` is selected at this pre-requester stage.

The existing later requester/rendezvous durable higher-owner remains the separate boundary where the existing outer `Arc<ProductionDurableCapabilityAuthority>` adaptation already occurs.

## 10. Requester/rendezvous exclusion

The future source successor selected by MC must not:

- construct `BoundedRequesterRendezvousStartPolicySource`;
- infer allow/deny bindings;
- choose provider type or capacity;
- construct `InMemoryRequesterRendezvousAuthorityProvider`;
- construct `CandidatePublicationRequesterRendezvousRuntimeOwner`;
- register requester/rendezvous lifecycle state;
- build `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs`;
- build the existing final durable requester/rendezvous higher-owner;
- invoke requester/rendezvous policy or provider behavior.

Requester/rendezvous production provenance requires a separate reviewed gate.

## 11. Authority and identity invariants

Remain distinct:

- `SharedCurrentCapabilityAuthority<P>`;
- `ProductionDurableRegistryRuntimeCustody`;
- raw `ProductionDurableCapabilityAuthority`;
- requester/rendezvous authority;
- authenticated/logical `DeviceId`;
- current `PeerConnectivityIdentity`;
- transient reachability/socket address;
- PRWM `request_id` correlation.

`DeviceId` is a logical lookup key.

`PeerConnectivityIdentity` is current transport-authority output for that logical device.

IP/port is not logical identity.

PRWM `request_id` is correlation only and is not requester identity, target identity, authentication, authorization or transport-authority evidence.

## 12. Side-effect ceiling for the future source checkpoint

Source materialization may contain the dormant async helper described above, but adds no invocation site.

Calling that helper later would perform only the already-selected production source/bootstrap operations in the exact order above.

The source checkpoint itself must add no:

- requester/rendezvous population;
- endpoint bind/listener/readiness/publication behavior;
- remote operation construction/invocation;
- LX companion invocation;
- runtime/task/thread spawn;
- process-exit policy;
- `main.rs` or `run()` caller;
- merge/deploy/restart/recovery/activation behavior.

No networking/runtime activation occurs merely by materializing the dormant helper.

## 13. Focused validation allowance

The future source checkpoint may add same-file tests limited to:

- composite error variant/source/Display shape;
- helper signature/source-shape assertions that do not execute process-global environment mutation or real systemd/provider I/O;
- compile-time ownership/non-cloneability shape.

No test may invent production requester/rendezvous defaults or perform destructive environment/provider mutation.

A narrowly-scoped lint acknowledgement is permitted only if exact source shape requires it.

## 14. C03e-MC exact scope

C03e-MC itself changes only this documentation contract:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_MC_PRODUCTION_DURABLE_REACHABILITY_SAME_CUSTODY_OWNER_POPULATION_SELECTION_STAGING.md`

C03e-MC authorizes no Rust/source/runtime mutation in the MC branch.

## 15. Explicitly not selected

C03e-MC does not authorize:

- owner population in MC itself;
- any Rust path change in MC;
- mutation of `linux_bootstrap.rs`;
- mutation of `production_durable_registry_custody_bootstrap.rs`;
- mutation of `production_durable_registry_runtime_custody.rs`;
- mutation of requester/rendezvous policy/runtime modules;
- legacy JR peer/composite-helper reuse for the new owner;
- requester/rendezvous production provenance;
- requester/rendezvous provider type/capacity/population;
- final requester/rendezvous join;
- LX invocation;
- current capability-authority source selection;
- session-authentication source selection;
- expected-request producer/channel source selection;
- callback source/policy selection;
- executable caller/input assembly;
- `main.rs` or `run()` mutation;
- listener/readiness/network semantic changes;
- authentication/authorization/trust weakening;
- identity/address/correlation conflation;
- manifest/lockfile/workflow/Android/packaging/systemd/credential/certificate/trust/RBAC/repository-configuration changes;
- merge;
- ready-for-review conversion;
- deploy;
- restart/recovery;
- runtime activation;
- PR close;
- branch deletion;
- force update;
- rebase;
- squash;
- history rewrite;
- destructive cleanup.

## 16. Validation and closure discipline

Validation authority is the exact final C03e-MC head only.

Skipped workflows are `SKIPPED`, not PASS.

A successful workflow on a superseded head cannot close MC.

After exact-final-head validation, publish one immutable canonical Google Drive audit with exact-title pre-upload uniqueness and raw-byte/hash readback verification.

Only after verified durable evidence publication may PR metadata be updated to:

`SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

Closure token:

`CLOSED_PRODUCTION_DURABLE_REACHABILITY_SAME_CUSTODY_OWNER_POPULATION_SELECTION`

After closure: **STOP**.
