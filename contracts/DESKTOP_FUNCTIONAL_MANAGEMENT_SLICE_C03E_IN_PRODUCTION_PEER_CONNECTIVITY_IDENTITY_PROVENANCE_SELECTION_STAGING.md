# Phase 152 C03e-IN — Production Peer Connectivity Identity Provenance Selection

Status: `STAGED_SELECTION`

Gate on closure:
`C03E_IN_PRODUCTION_PEER_CONNECTIVITY_IDENTITY_PROVENANCE_SELECTED`

Canonical closure token on successful exact-head validation and evidence recording:
`CLOSED_PRODUCTION_PEER_CONNECTIVITY_IDENTITY_PROVENANCE_SELECTION`

## 1. Scope

C03e-IN is a documentation-only prerequisite selection checkpoint after closed C03e-IM.

It selects the provenance law for the `PeerConnectivityIdentity` consumed by the already-materialized production reachability/process-operation path. It does **not** create a production identity source, materialize Rust source, activate the production remote companion, alter runtime readiness, or change any production state.

The exact predecessor is C03e-IM head:

`ca7ba379d480eac802264a29f6e060c572fed4f3`

C03e-IM materialized only production bind-address population and explicitly left production `PeerConnectivityIdentity` provenance separately gated.

## 2. Exact source state observed at the IM predecessor

### 2.1 Existing production process-operation owner

`crates/prw-agent/src/linux_bootstrap.rs`

Exact IM blob:

`3f7c2214452b4f61ae0c0c77495f60f70f742709`

The existing crate-private owner is:

```text
LinuxAgentProductionReachabilityRemoteProcessOperationInputs<...>
    peer: PeerConnectivityIdentity
    remote_process_inputs: LinuxAgentRemoteProcessOperationInputs<...>
```

Its constructor accepts an already-typed `PeerConnectivityIdentity` and performs ownership composition only.

The production operation later passes that exact peer to:

```text
RemoteSessionExecutorRuntime::bootstrap_production_reachability_runtime_custody_from_systemd_credentials(&peer)
```

No production source for that peer is selected or invoked in IM.

### 2.2 Existing bind-address population does not establish identity

The C03e-IM helper:

```text
linux_agent_remote_process_operation_inputs_from_production_bind_addr(...)
```

loads only `PRW_REMOTE_BIND_ADDR` through the existing exact `SocketAddr` source and constructs the existing non-peer remote-process input owner.

The bind address, IP address and port are transient endpoint/reachability configuration only. They are not logical identity, transport identity, session identity, requester identity, rendezvous target identity, or proof of any of those identities.

### 2.3 Exact peer type

`crates/prw-connectivity/src/lib.rs`

Exact IM blob:

`fefb8459e73bd0a92e87e8d7600282c7f515b159`

`PeerConnectivityIdentity` contains exactly:

```text
DeviceId
TransportIdentity
```

`DeviceId` is the logical device identity. `TransportIdentity` is an independently rotatable 32-byte mesh transport identity.

The existing constructor:

```text
PeerConnectivityIdentity::new(device, transport)
```

accepts already-validated components. Constructor existence is not provenance or authorization to fabricate either component.

### 2.4 Durable production reachability is keyed by the exact two-part peer lifecycle

`crates/prw-agent/src/production_reachability_owner_custody.rs`

At IM, authoritative durable recovery and custody lookup use exact `PeerConnectivityIdentity` equality.

The existing semantics explicitly reject:

- `DeviceId`-only matching;
- `TransportIdentity`-only matching;
- alternate transport identity for the same logical device;
- arbitrary single-entry fallback;
- missing or ambiguous exact peer ownership.

Therefore production peer provenance must preserve the exact logical-device plus current-transport lifecycle pair before durable recovery begins.

### 2.5 Production reachability bootstrap consumes but does not source the peer

`crates/prw-agent/src/production_reachability_custody_bootstrap.rs`

Exact IM blob:

`ba1e9bb318a4d64206eb745ccb33a00d587f87a3`

The existing production systemd custody facade accepts only:

```text
&PeerConnectivityIdentity
```

It loads reachability-provider credentials and delegates to the production composition path. It does not infer or obtain the peer identity from provider endpoint, certificate file path, credential directory, request ID, IP address, or another incidental value.

`crates/prw-agent/src/production_reachability_bootstrap.rs`

Exact IM blob:

`8de308229d072272b96e4217f8ebf6484e666f23`

The bootstrap similarly consumes the already-typed peer for durable owner recovery. It does not own peer-identity discovery or production registry lookup.

## 3. Existing identity authority semantics that constrain provenance

### 3.1 Current registry keeps logical device and transport identity distinct

`crates/prw-registry/src/lib.rs`

Exact IM blob:

`cd7eb52eea19354620ff8ad26c9b8aad3f9c5eb6`

The existing registry model stores a registered logical device and a separately rotatable optional current `TransportIdentity`.

Existing registry operations include:

- bind first current transport identity to one exact enrolled device;
- rotate transport identity only by exact compare-and-replace;
- reject unknown or revoked devices;
- reject absent transport identity;
- reject stale/mismatched transport identity.

The registry-current principal snapshot exposes the validated logical `DeviceId` while the current registry separately owns the transport binding.

This model establishes the required semantic relationship: a production peer identity must join one exact logical `DeviceId` with the current transport identity bound to that same device under current authority.

### 3.2 Current capability authorization already proves the same-device binding law

`crates/prw-remote-bridge/src/lib.rs`

Exact IM blob:

`7b1c5c62339983da6ae2556f73510d7582ec0c5b`

The existing authorization sequence is:

1. revalidate the authenticated application session against the current registry;
2. obtain the current-registry principal and its logical `DeviceId`;
3. validate the presented `TransportIdentity` against the current transport identity bound to that exact `DeviceId`;
4. only then construct an authorized request.

The resulting authorized request keeps the current-registry principal and the current verified transport identity as separate typed values.

This is evidence of the already-selected identity-binding law. It is **not** a production peer-source callsite for reachability bootstrap, because request authorization occurs in a different lifecycle stage and cannot be retroactively treated as the bootstrap source.

### 3.3 Transport identity is authenticated transport-key identity, not logical device identity

`crates/prw-remote-transport/src/lib.rs`

At IM, `TransportIdentity` is derived from the canonical leaf certificate SPKI using SHA-256, and established transport identity can be checked against an expected registry value.

Transport identity therefore cannot replace `DeviceId`; likewise `DeviceId` cannot replace transport authentication.

## 4. Selected production peer provenance law

C03e-IN selects the following law and no broader implementation:

> The production `PeerConnectivityIdentity` supplied to production reachability/bootstrap ownership must originate from one authoritative, current same-device binding of logical `DeviceId` and current `TransportIdentity`. Both components must refer to the same exact device lifecycle at the moment the production peer value is admitted for use. Neither component may be inferred from transient reachability, request correlation, session correlation, process configuration, or requester/rendezvous target intent alone.

The exact semantic construction is:

```text
current authoritative device record
    -> exact enrolled/non-revoked DeviceId
    -> exact current transport binding for that same DeviceId
    -> PeerConnectivityIdentity::new(device_id, current_transport_identity)
```

The construction step is pure. The authority that proves the two inputs are current and same-device remains separately gated.

## 5. Required fail-closed conditions

A future production peer-authority source must fail closed before returning a `PeerConnectivityIdentity` when any of these conditions applies:

- logical device is unknown;
- device is not currently eligible/enrolled;
- device is revoked;
- current transport binding is absent;
- transport binding is stale or mismatched;
- more than one authoritative current binding is returned for the same requested lifecycle;
- the current source cannot prove that the logical and transport identities belong to the same device;
- source state is unavailable or ambiguous;
- a caller presents only an endpoint, bind address, candidate, request ID, session ID, or unvalidated target and asks the source to infer identity.

No fallback `PeerConnectivityIdentity` may be fabricated.

## 6. Explicit non-sources

The following values are not selected as production `PeerConnectivityIdentity` provenance:

- `PRW_REMOTE_BIND_ADDR`;
- local or remote `SocketAddr`;
- candidate IP address or port;
- `ConnectivityEndpoint`;
- `CandidateId`;
- `ConnectivityPathKind`;
- current selected path;
- STUN/TURN/relay observation;
- DNS name;
- interface address;
- public-IP discovery;
- PRWM `request_id`;
- `SessionId`;
- terminal or transfer IDs;
- requester/rendezvous request correlation IDs;
- expected-device scheduling intent by itself;
- requester identity by itself;
- rendezvous target `DeviceId` by itself;
- transport certificate identity by itself;
- static environment literals invented only to satisfy a constructor;
- test/disposable seed identities.

## 7. Relationship to expected-device scheduling intent

Earlier Phase 152 production-admission selection preserves an authoritative pre-handshake expected `DeviceId` as scheduling intent.

C03e-IN does not upgrade that expected `DeviceId` into a complete production peer identity by itself.

A future authoritative source may use an already-authorized expected `DeviceId` as the lookup key for current device/transport binding, but successful lookup must still return the current same-device `TransportIdentity` under a production authority. A stale, missing or ambiguous lookup must fail closed.

Therefore:

```text
expected DeviceId
    != PeerConnectivityIdentity
```

and:

```text
expected DeviceId
    + guessed transport identity
    != authorized production PeerConnectivityIdentity
```

## 8. Relationship to authenticated remote admission

Remote application-session authorization already validates one current-registry principal and one current same-device transport identity before capability dispatch.

C03e-IN preserves that identity law but does not move request-time authorization earlier, reuse a post-handshake authorized request as pre-bootstrap authority, or introduce a lifecycle cycle.

Reachability bootstrap peer provenance and later capability request authorization remain distinct gates even when they eventually consume identity facts from the same authoritative registry domain.

## 9. Current source gap

The exact IM source does **not** contain a selected production current-registry persistence/bootstrap/provider path that can safely be declared the executable source for `PeerConnectivityIdentity`.

The existing `WorkspaceDeviceRegistry` is explicitly an in-memory Phase 130 registry model. C03e-IN does not silently reinterpret that in-memory model as production durable authority, does not add persistence, and does not choose a database/provider.

Consequently, C03e-IN intentionally selects provenance semantics without selecting a Rust source-materialization successor for the production peer field.

This is a fail-closed result of the audit, not a missing implementation to be bypassed.

## 10. Next separately gated prerequisite

Before any source checkpoint may populate the production `peer` field, a new selection checkpoint must identify the authoritative production current-device/transport-binding source and prove all of the following from exact source/topology:

1. who owns the current device lifecycle authority;
2. who owns the current transport binding authority;
3. whether those authorities are one source or require a bounded composition seam;
4. how currentness/revocation/rotation are observed;
5. how ambiguity/unavailability fail closed;
6. how the selected source reaches `prw-agent` without dependency inversion or raw provider leakage;
7. how expected-device scheduling intent may be used only as a lookup key, not as authority;
8. how the result remains an exact two-part `PeerConnectivityIdentity` lifecycle key;
9. the exact source-path ceiling for the first materialization;
10. focused no-fabrication/stale-transport/revocation/ambiguity tests.

Until that prerequisite closes, production peer field population remains blocked.

## 11. No source-materialization authorization from IN

C03e-IN authorizes no Rust/Kotlin/Cargo/workflow/systemd/package/security source materialization.

In particular, IN does not authorize adding a helper that:

- reads a `DeviceId` or transport identity from a new environment variable;
- hard-codes a production identity;
- creates a test/disposable identity in executable code;
- treats bind address or candidate endpoint as identity;
- reads certificate bytes and assumes a logical device without registry binding;
- constructs a new in-memory registry and treats it as current production state;
- introduces database, control-plane, file, network, or provider access;
- activates production reachability merely because a typed peer value can be constructed.

## 12. Frozen executable/runtime surfaces

The following remain unchanged and uninvoked by this selection:

- `linux_agent_production_reachability_remote_process_operation(...)`;
- `linux_agent_production_reachability_requester_rendezvous_remote_process_operation(...)`;
- `run_with_production_reachability_requester_rendezvous_remote_process_companion(...)`;
- public `run()`;
- `main.rs`;
- local readiness semantics;
- listener activation;
- requester/rendezvous policy evaluation;
- requester/rendezvous provider mutation;
- candidate publication;
- NAT traversal;
- relay activation;
- peer dialing;
- production retry/reconnect/rebind/rebootstrap loops.

## 13. Security and identity invariants

C03e-IN preserves all established separation:

- user identity is not device identity;
- logical `DeviceId` is not `TransportIdentity`;
- `TransportIdentity` rotation does not replace logical device identity;
- endpoint/IP/port is not identity;
- `SessionId` is not device identity;
- PRWM request ID is correlation only;
- expected-device scheduling intent is not proof of transport identity;
- requester identity and requester/rendezvous target identity remain separately validated;
- a durable reachability key is exact two-part peer lifecycle identity, not a network address.

No private key, certificate, trust root, account credential, service credential, secret, or token value is introduced by this checkpoint.

## 14. Repository and operational non-authorization

C03e-IN does not authorize or perform:

- repository visibility/configuration mutation;
- merge;
- branch deletion;
- history rewrite;
- production deployment;
- service restart;
- systemd unit/package mutation;
- `LoadCredential=` mutation;
- credential creation/replacement;
- certificate issuance/installation;
- trust-store mutation;
- RBAC/auth mutation;
- database/schema/migration change;
- host firewall/routing/DNS/TUN mutation;
- runtime listener activation;
- production-state mutation.

Repository visibility observed in the current project lineage is `public`; this checkpoint does not change it.

## 15. Exact-head validation requirement

C03e-IN may be closed only after the exact final IN head proves:

- exact predecessor is C03e-IM head `ca7ba379d480eac802264a29f6e060c572fed4f3`;
- merge base is that exact predecessor;
- ahead `1`, behind `0` unless a separately documented mechanical correction is required;
- exactly one changed path, this Markdown contract;
- zero Rust/Kotlin/Cargo/lockfile/workflow/runtime changes;
- `PRW Rust Validation` succeeds on the exact final head;
- path-filtered disposable etcd workflows are skipped as expected for a docs-only change;
- Android validation is claimed only if an exact-head Android run actually occurs;
- no exact-head required check remains failing or pending at closure.

Superseded-head validation, if any, is diagnostic history only.

## 16. Durable evidence requirement

After exact-head validation succeeds, preserve one immutable Markdown audit artifact in the canonical Private Remote Workspace Drive parent.

The audit must record at minimum:

- exact predecessor/head/tree;
- exact changed-path topology;
- contract blob;
- exact-head workflow run/job identities and conclusions;
- the selected provenance law;
- the current production-source gap;
- the explicit block on production peer source materialization;
- repository visibility observation without mutation;
- Drive file ID, byte size and SHA-256;
- raw readback equality.

No audit evidence may claim a production identity source, runtime activation, deployment, or test result that did not occur.

## 17. Closure meaning

Successful C03e-IN closure means only:

`PRODUCTION_PEER_CONNECTIVITY_IDENTITY_PROVENANCE_SELECTED`

It means the project now has an explicit fail-closed rule for what may constitute the production reachability peer identity and a documented proof that the current exact source lacks an authorized production authority source for constructing that value.

It does **not** mean:

- production peer identity source materialized;
- production registry persistence selected;
- remote runtime activated;
- reachability established;
- candidate publication activated;
- listener exposed;
- transport authenticated in production;
- deployment completed.

## 18. Safe successor

The safe successor begins with a fresh exact-IN-head read-only audit and selects the authoritative production current-device/transport-binding source required to produce one exact `PeerConnectivityIdentity` without fabrication.

That successor must remain a selection checkpoint unless exact source proves an already-existing production authority seam narrow enough to materialize safely under an explicit source ceiling.
