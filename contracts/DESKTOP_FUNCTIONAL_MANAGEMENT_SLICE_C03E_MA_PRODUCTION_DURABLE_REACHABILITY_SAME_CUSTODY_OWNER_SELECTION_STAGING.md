# C03e-MA — Production durable-reachability same-custody owner selection

Status: `SELECTION_ONLY — NO_RUST_SOURCE_MUTATION`

This checkpoint is documentation-only. It selects one later dormant ownership boundary above exact evidence-closed C03e-LZ. It does not itself modify Rust/source/runtime behavior, populate requester/rendezvous policy/provider state, invoke a production source, assemble an executable caller, merge, deploy, restart, or alter repository configuration.

## 1. Exact predecessor authority

C03e-LZ is the sole predecessor authority for this selection:

- branch: `phase-152-c03e-lz-production-durable-registry-same-custody-peer-capability-authority-population-source-materialization`
- exact head: `782616669f213487e32c8b0f454772cf268c981a`
- exact tree: `8227bfeda54d3888f3cb2f37b8ca413d52f3307e`
- exact LZ population source blob: `6ad990bf3b8e6536351e06d3b939370ed73c887e`
- exact higher-owner source blob: `85c9b7b7992ca4bce3cd29a833b10b58bc72f647`
- exact Linux bootstrap blob: `7940a69e598355176a61b0bef5c7571dab9fb530`
- LZ status: `SOURCE_MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

Fresh pre-selection namespace audit found no `phase-152-c03e-ma*` branch. Fresh PR search found no existing PR titled `C03e-MA: select production durable reachability same-custody owner`. Fresh canonical Drive search found no exact-title MA audit artifact. The exact MA contract path did not exist at the predecessor head.

## 2. Fresh exact-source finding

### 2.1 LZ now produces one same-custody peer + durable-authority pair

Exact LZ path:

`crates/prw-agent/src/production_durable_registry_custody_bootstrap.rs`

Exact LZ blob:

`6ad990bf3b8e6536351e06d3b939370ed73c887e`

C03e-LZ materialized:

`bootstrap_production_peer_and_durable_capability_authority_from_systemd_credentials(device_id)`

The helper:

1. accepts one logical `DeviceId`;
2. performs exactly one existing production durable-registry bootstrap;
3. converts that exact store into one runtime custody;
4. resolves one current same-device `PeerConnectivityIdentity` from that exact custody;
5. consumes the same surviving custody into one `ProductionDurableCapabilityAuthority`;
6. returns the peer and durable authority from that single custody lineage.

It adds no invocation site and remains dormant.

### 2.2 Existing Linux production reachability owner already retains the peer beside remote-process inputs

Exact LZ path:

`crates/prw-agent/src/linux_bootstrap.rs`

Exact LZ blob:

`7940a69e598355176a61b0bef5c7571dab9fb530`

Existing owner:

`LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>`

already retains:

- one current `PeerConnectivityIdentity`; and
- one existing `LinuxAgentRemoteProcessOperationInputs<P,D,T,F,C,R,E>`.

Its constructor is side-effect-free. Existing production worker-limit/bind population and the legacy peer-only population helpers remain separate from requester/rendezvous custody and executable assembly.

### 2.3 Existing higher-owner begins only after requester/rendezvous custody has already been joined

Exact LZ path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Exact LZ blob:

`85c9b7b7992ca4bce3cd29a833b10b58bc72f647`

Existing higher-owner:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<P,D,T,F,C,R,E>`

retains:

- one already-constructed `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<P,D,T,F,C,R,E>`; and
- one outer `Arc<ProductionDurableCapabilityAuthority>` created from a raw durable authority in its constructor.

That owner is therefore too late in the composition graph to act as the immediate retention boundary for the new LZ pair without simultaneously requiring requester/rendezvous custody.

### 2.4 Requester/rendezvous production provenance is still unresolved

Exact LZ Linux source confirms that requester/rendezvous operation custody requires:

- `BoundedRequesterRendezvousStartPolicySource`; and
- `CandidatePublicationRequesterRendezvousRuntimeOwner`.

The observed construction using `BoundedRequesterRendezvousStartPolicySource::default()` and an in-memory provider with capacity `1` occurs only in synthetic test assembly. It is not production provenance and must not be promoted into production policy/provider population.

Therefore the immediate next safe boundary must remain before requester/rendezvous join.

### 2.5 A free pair is not the safest long-lived composition surface

LZ necessarily returns two values from the same custody lineage. A later caller could mechanically separate those values and accidentally recombine one peer-bearing production owner with a durable authority derived by a different bootstrap/custody lineage.

The next safe step is therefore not caller population. It is a narrow non-cloneable ownership carrier that keeps the production reachability owner and its corresponding raw durable authority together before requester/rendezvous custody is introduced.

No such pre-requester durable-reachability owner exists in exact LZ `linux_bootstrap.rs` or `production_durable_capability_higher_owner_custody.rs`.

## 3. Selected immediate later source boundary

A later source-materialization checkpoint may modify exactly one Rust path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

It may add exactly one additive crate-private, non-cloneable dormant owner conceptually named:

`LinuxAgentProductionDurableReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>`

The exact mechanical spelling may change only if required by Rust naming/formatting constraints while preserving this semantic boundary.

## 4. Required future owner law

The selected future owner must retain exactly:

1. one `LinuxAgentProductionReachabilityRemoteProcessOperationInputs<P,D,T,F,C,R,E>` by value; and
2. one raw `ProductionDurableCapabilityAuthority` by value.

The two fields must remain private to the module.

The source checkpoint must add no public or crate-public general-purpose constructor, accessor, splitter, clone, copy, tuple-export, authority getter, production-input getter, or generic extraction surface. The owner may remain constructible only from later separately gated logic in the same module.

The selected source checkpoint must not itself call the C03e-LZ population helper. It materializes only the custody shape needed to preserve the pair across a future population/join boundary.

The raw durable authority must not be wrapped in `Arc` at this pre-requester stage. Existing `Arc` construction remains deferred to the already-materialized final higher-owner boundary unless a later separately gated checkpoint explicitly selects a different ownership transition.

## 5. Same-custody provenance law

The selected owner is not permission to combine arbitrary production reachability inputs with arbitrary durable authority.

Future construction of this owner must be separately gated and must derive:

- the `PeerConnectivityIdentity` embedded in `LinuxAgentProductionReachabilityRemoteProcessOperationInputs`; and
- the retained `ProductionDurableCapabilityAuthority`

from the exact same single invocation of the C03e-LZ helper:

`bootstrap_production_peer_and_durable_capability_authority_from_systemd_credentials(device_id)`.

No independent call to `bootstrap_production_durable_capability_authority_from_systemd_credentials()` may be used to populate the retained authority for that future owner.

No second durable-registry/provider bootstrap, peer substitution, authority substitution, retry, fallback, cached value, alternate device, or degraded authority is selected.

## 6. Authority and identity invariants

The selected later owner must preserve distinct lanes:

- `SharedCurrentCapabilityAuthority<P>`;
- `ProductionDurableRegistryRuntimeCustody`;
- `ProductionDurableCapabilityAuthority`;
- requester/rendezvous authority;
- authenticated/logical `DeviceId`;
- current `PeerConnectivityIdentity`;
- reachability/socket address;
- PRWM `request_id` correlation.

The current `PeerConnectivityIdentity` remains current durable-registry authority output. IP/port is not logical identity. PRWM `request_id` remains correlation only and is not authentication, authorization, identity, or transport-authority evidence.

## 7. Ownership and side-effect ceiling

The selected source checkpoint is ownership-only and side-effect-free. It must add no:

- env/process configuration read;
- systemd credential read;
- durable-registry/provider bootstrap;
- durable-registry read/write/Txn/Put;
- peer lookup;
- `Arc::new` or `Arc::clone` for the durable authority;
- generic shared custody surface;
- requester/rendezvous policy evaluation or population;
- requester/rendezvous provider construction, capacity choice, mutation or registration;
- current capability-authority population;
- session-authentication source selection;
- expected-request channel/producer creation;
- callback source/policy/translation/logging/metrics;
- endpoint bind, listener, readiness or publication behavior;
- retry/reconnect/fallback/cache behavior;
- remote operation construction or invocation;
- remote companion invocation;
- runtime/task/thread spawn;
- process-exit policy;
- executable caller;
- `run()` or `main.rs` mutation.

## 8. Explicitly not selected

C03e-MA does not select or authorize:

- Rust/source mutation in MA itself;
- mutation of `linux_bootstrap.rs`;
- mutation of `production_durable_registry_custody_bootstrap.rs`;
- mutation of `production_durable_registry_runtime_custody.rs`;
- invocation of the LZ helper;
- production population of the selected owner;
- construction of requester/rendezvous custody;
- requester/rendezvous policy-source production provenance;
- requester/rendezvous provider type/capacity/population decisions;
- construction of the existing final `LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs`;
- LX companion invocation;
- current capability-authority/session-authentication/expected-request/callback source selection;
- executable caller/input assembly;
- `main.rs` or `run()` migration;
- listener/readiness/network semantic changes;
- authentication/authorization/trust weakening;
- manifest, lockfile, workflow, Android, packaging, systemd, credential, certificate, trust, RBAC or repository-configuration changes;
- merge or ready-for-review conversion;
- deploy, restart, recovery or production activation;
- PR close, branch deletion, force update, rebase, squash or history rewrite;
- destructive cleanup.

## 9. Gate and successor discipline

C03e-MA selection gate:

`C03E_MA_PRODUCTION_DURABLE_REACHABILITY_SAME_CUSTODY_OWNER_SELECTED`

After C03e-MA is validated and evidence-recorded: **STOP**.

A later source checkpoint may materialize only the one-file dormant pre-requester ownership carrier selected above, after a fresh namespace/head/source audit. It must not simultaneously populate that owner from LZ, construct requester/rendezvous custody, construct the existing final higher-owner aggregate, invoke LX, modify `linux_bootstrap.rs`, `main.rs` or `run()`, or activate runtime behavior.
