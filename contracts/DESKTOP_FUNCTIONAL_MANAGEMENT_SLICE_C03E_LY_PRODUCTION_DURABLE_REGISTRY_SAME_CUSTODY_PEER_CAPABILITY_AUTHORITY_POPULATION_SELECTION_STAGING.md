# C03e-LY — Production durable-registry same-custody peer + capability-authority population selection

Status: `SELECTION_ONLY — NO_RUST_SOURCE_MUTATION`

This checkpoint is documentation-only. It selects one later dormant production population boundary above exact evidence-closed C03e-LX. It does not itself modify Rust/source/runtime behavior, populate requester/rendezvous policy/provider state, invoke an executable caller, merge, deploy, restart, or alter repository configuration.

## 1. Exact predecessor authority

C03e-LX is the sole predecessor authority for this selection:

- branch: `phase-152-c03e-lx-production-durable-capability-higher-owner-projection-companion-assembly-source-materialization`
- exact head: `4909d1ab927b26a34dc8bf6ff743ecc0f8fcfd93`
- exact tree: `7f42eda98096fe06a8bfdd6cbea1bff6d6cb80e4`
- exact higher-owner source blob: `85c9b7b7992ca4bce3cd29a833b10b58bc72f647`
- LX status: `SOURCE_MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

Fresh pre-selection namespace audit found no `phase-152-c03e-ly*` branch.

## 2. Fresh exact-source finding

### 2.1 LX companion assembly remains dormant

Exact LX path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Exact LX blob:

`85c9b7b7992ca4bce3cd29a833b10b58bc72f647`

C03e-LX materialized the crate-private dormant companion assembly:

`run_with_production_durable_reachability_requester_rendezvous_remote_process_companion_with_production_durable_capability_projection(...)`

The assembly accepts one already-populated higher-owner aggregate by value. It does not populate the production peer, durable capability authority, requester/rendezvous policy source, requester/rendezvous provider owner, current capability authority, expected-request channel, callbacks, or executable process policy.

### 2.2 Existing production remote-process + peer population

Exact LX `crates/prw-agent/src/linux_bootstrap.rs` blob:

`7940a69e598355176a61b0bef5c7571dab9fb530`

The existing helper:

`linux_agent_production_reachability_remote_process_operation_inputs_from_production_worker_limit_and_peer(...)`

already composes the fixed worker-limit source, fixed bind-address source, fixed logical peer-device source and current durable-registry peer lookup. It returns one existing:

`LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>`

but does not populate requester/rendezvous custody or durable capability authority.

Its peer stage currently bootstraps a production durable-registry semantic store, converts it into `ProductionDurableRegistryRuntimeCustody`, resolves one current same-device `PeerConnectivityIdentity`, and then drops the remaining custody when the helper returns.

### 2.3 Existing durable capability-authority bootstrap

Exact LX path:

`crates/prw-agent/src/production_durable_registry_custody_bootstrap.rs`

Exact LX blob:

`c48003712ac20b86fc09ebdfb2ddb67afd44f649`

The existing helper:

`bootstrap_production_durable_capability_authority_from_systemd_credentials()`

performs a separate durable-registry bootstrap, converts the returned store into `ProductionDurableRegistryRuntimeCustody`, and consumes that custody into `ProductionDurableCapabilityAuthority`.

Using the existing peer helper and this authority helper independently for one future higher-owner aggregate would therefore perform two independent durable-registry/provider bootstraps and could derive peer transport authority and durable capability authority from different runtime-custody snapshots.

C03e-LY does not select that duplicated-bootstrap shape.

### 2.4 Existing runtime custody already supports one-custody derivation

Exact LX path:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

Exact LX blob:

`90b12c182d6564b42e3f22f9e3dd594ec94d2fe5`

`ProductionDurableRegistryRuntimeCustody` already supports:

`peer_connectivity_identity(device_id)`

which mutably borrows the custody, performs exactly one current same-device transport lookup and leaves the custody owned by the caller.

`ProductionDurableCapabilityAuthority::from_registry_custody(...)` already consumes that same remaining custody by value into the durable capability authority.

Therefore one store/bootstrap can safely produce the exact peer identity first and then transfer the same surviving custody into the durable capability authority without adding a generic store getter/extractor or cloning the custody.

### 2.5 Requester/rendezvous production population remains unresolved and excluded

Exact LX requester/rendezvous policy source:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_source.rs`

The concrete `BoundedRequesterRendezvousStartPolicySource` explicitly materializes only bounded in-memory policy backing and states that production custody/population is not wired.

Exact LX requester/rendezvous runtime owner:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`

`CandidatePublicationRequesterRendezvousRuntimeOwner::new(...)` consumes one already-configured `InMemoryRequesterRendezvousAuthorityProvider`. Provider capacity and lifecycle configuration are caller-owned decisions. No production population source is materialized.

The only observed `BoundedRequesterRendezvousStartPolicySource::default()` + in-memory provider construction in `linux_bootstrap.rs` is synthetic test construction, not production provenance.

C03e-LY therefore must not infer a default requester policy, positive/negative grant set, provider capacity, or provider population strategy.

## 3. Selected immediate later source boundary

The smallest safe later source boundary is a same-custody durable-registry population seam, not executable caller population and not requester/rendezvous production policy/provider selection.

A later source-materialization checkpoint may modify exactly one Rust path:

`crates/prw-agent/src/production_durable_registry_custody_bootstrap.rs`

It may add one additive crate-private async helper conceptually named:

`bootstrap_production_peer_and_durable_capability_authority_from_systemd_credentials(...)`

The exact mechanical spelling may be adjusted only if required by Rust naming/formatting constraints while preserving this semantic boundary.

## 4. Required future population law

The selected future helper must:

1. accept exactly one logical `DeviceId` by value;
2. call existing `bootstrap_production_durable_registry_from_systemd_credentials()` exactly once;
3. convert that exact returned store into one `ProductionDurableRegistryRuntimeCustody` exactly once;
4. call `peer_connectivity_identity(device_id)` on that exact custody exactly once;
5. if peer lookup succeeds, consume the same remaining custody exactly once into `ProductionDurableCapabilityAuthority::from_registry_custody(...)`;
6. return the exact `PeerConnectivityIdentity` and `ProductionDurableCapabilityAuthority` derived from that single custody lineage;
7. remain crate-private and add no invocation site.

The helper must not call `bootstrap_production_durable_capability_authority_from_systemd_credentials()` internally because that would perform a second independent bootstrap.

The helper may return the two values through either an explicit non-cloneable carrier or a direct pair only if the exact same-custody provenance and by-value ownership law remain mechanically evident. No generic store/custody extraction API is selected.

## 5. Required future error law

The selected source checkpoint may add only the bounded error surface necessary to distinguish:

- existing production durable-registry custody/provider bootstrap failure; and
- existing current same-device durable-registry peer lookup failure.

The exact underlying errors must remain preserved as sources/variants. Display text must remain bounded and must not expose endpoint, credential, certificate, private-key, provider, registry-record, transport-identity, device-value, or other sensitive details.

No retry, fallback, cached peer, alternate device, synthetic authority, degraded authority or second bootstrap is selected.

## 6. Authority and identity invariants

The selected later seam must preserve distinct authority lanes:

- `SharedCurrentCapabilityAuthority<P>`;
- `ProductionDurableRegistryRuntimeCustody`;
- `ProductionDurableCapabilityAuthority`;
- requester/rendezvous authority;
- authenticated/logical `DeviceId`;
- current `PeerConnectivityIdentity` transport binding;
- reachability/socket address;
- PRWM `request_id` correlation.

The logical `DeviceId` is only the lookup key supplied to existing current durable-registry authority. Its current transport identity must come exclusively from `peer_connectivity_identity(...)` and must not be accepted from the caller.

IP/port is not logical identity. PRWM `request_id` remains correlation only and is not authentication/authorization evidence.

## 7. Ownership and side-effect ceiling

The future helper may perform only the already-existing fixed production credential read/provider bootstrap and one existing current peer lookup required by its selected purpose.

It must add no:

- second durable-registry/provider bootstrap;
- `Arc::clone` or new generic shared custody surface;
- generic store getter/extractor;
- durable-registry write/Txn/Put or production mutation;
- requester/rendezvous policy population;
- requester/rendezvous provider construction/capacity selection;
- requester/rendezvous registration;
- current capability-authority population;
- expected-request channel creation;
- callback translation/logging/metrics/process-exit policy;
- retry/reconnect/fallback/cache behavior;
- endpoint bind/listener/readiness semantics;
- remote companion invocation;
- runtime/task spawn;
- executable caller;
- `run()` or `main.rs` migration.

## 8. Explicitly not selected

C03e-LY does not select or authorize:

- Rust/source mutation in LY itself;
- mutation of `linux_bootstrap.rs`;
- mutation of `production_durable_registry_runtime_custody.rs`;
- mutation of `production_durable_capability_higher_owner_custody.rs`;
- construction of `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs`;
- construction of `LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs`;
- requester/rendezvous policy-source production provenance;
- requester/rendezvous provider type/capacity/population decisions;
- concrete caller/input population for LX;
- current capability-authority source selection;
- session-authentication source selection;
- expected-request producer/channel source selection;
- callback source/policy selection;
- `main.rs` or `run()` mutation;
- executable entrypoint migration;
- listener/readiness/network semantic changes;
- authentication/authorization/trust weakening;
- manifest, lockfile, workflow, Android, packaging, systemd, credential, certificate, RBAC or repository-configuration changes;
- merge or ready-for-review conversion;
- deploy, restart, recovery or production activation;
- PR close, branch deletion, force update, rebase, squash or history rewrite;
- destructive cleanup.

## 9. Gate and successor discipline

C03e-LY selection gate:

`C03E_LY_PRODUCTION_DURABLE_REGISTRY_SAME_CUSTODY_PEER_CAPABILITY_AUTHORITY_POPULATION_SELECTED`

After C03e-LY is validated and evidence-recorded: **STOP**.

A later source checkpoint may materialize only the one-file same-custody peer + durable capability-authority population seam selected above, after a fresh namespace/head/source audit. It must not simultaneously wire that seam into `linux_bootstrap.rs`, construct requester/rendezvous production custody, populate the LX higher-owner aggregate, invoke the LX companion assembly, modify `main.rs`/`run()`, or activate runtime behavior.
