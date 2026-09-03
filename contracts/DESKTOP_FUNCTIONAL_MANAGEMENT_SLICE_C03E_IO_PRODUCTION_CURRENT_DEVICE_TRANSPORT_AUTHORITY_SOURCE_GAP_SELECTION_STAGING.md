# Phase 152 C03e-IO — Production Current Device / Transport Authority Source-Gap Selection

Status: `STAGED_SELECTION`

Gate on closure:
`C03E_IO_PRODUCTION_CURRENT_DEVICE_TRANSPORT_AUTHORITY_SOURCE_GAP_SELECTED`

Canonical closure token on successful exact-head validation and evidence recording:
`CLOSED_PRODUCTION_CURRENT_DEVICE_TRANSPORT_AUTHORITY_SOURCE_GAP_SELECTION`

## 1. Scope

C03e-IO is the documentation-only prerequisite after closed C03e-IN.

C03e-IN established that the production `PeerConnectivityIdentity` supplied to production reachability/bootstrap ownership must come from one authoritative current same-device binding of logical `DeviceId` plus current `TransportIdentity`, and that no exact post-IN source is yet authorized to produce that value.

C03e-IO audits the exact C03e-IN source/dependency state to determine whether an existing production current-device/current-transport authority provider can be selected without invention.

The exact predecessor is C03e-IN head:

`adb7ad674638a57473eba5c33c69375841f7549a`

C03e-IO does not materialize Rust/Kotlin/Cargo/workflow/runtime source, choose a database/provider, persist a registry, alter enrollment or revocation, activate the production remote companion, mutate `run()`/`main.rs`, or change production state.

## 2. Exact source evidence at the C03e-IN predecessor

### 2.1 Registry implementation remains explicitly in-memory and non-persistent

Exact file:

`crates/prw-registry/src/lib.rs`

Exact C03e-IN blob:

`cd7eb52eea19354620ff8ad26c9b8aad3f9c5eb6`

The crate-level contract states that the registry revalidates current membership/device lifecycle state and deliberately does not:

- authenticate accounts;
- map roles to capabilities;
- persist a database;
- select a transport.

The implementation uses bounded in-memory maps and remains the Phase 130 source/disposable registry model.

This source is semantic authority for the current-binding rules exercised inside one registry instance. It is not evidence that a durable production registry exists.

### 2.2 Phase 130 contract explicitly defers durable production registry authority

Exact file:

`contracts/DEVICE_REGISTRY_WORKSPACE_MEMBERSHIP_CONTRACT.md`

Exact C03e-IN blob:

`2faf8b014d43583e9180b907f94f40f093e5125b`

The Phase 130 implementation lock permits bounded in-memory maps and explicitly defers:

- durable persistence;
- multi-process transactional coordination;
- replication;
- server-side database schema;
- distributed revocation propagation.

Its production causal boundary states that real production registry population remains dependent on a later durable registry deployment decision for production persistence.

Therefore C03e-IO must not reinterpret the current in-memory registry as an already-selected production durable source.

### 2.3 Phase 146 also refuses to invent production device authority

Exact file:

`contracts/ANDROID_ENROLLMENT_DEVICE_MANAGEMENT_CONTRACT.md`

Exact C03e-IN blob:

`ffcbd6924e635fe96fb0e61daa024927bafa07c3`

Phase 146 remains non-production. It may consume injected/disposable authoritative device snapshots for presentation but explicitly does not:

- persist a production registry;
- create a production device-list fetch endpoint;
- propagate production revocation;
- treat client presentation state as enrollment authority.

This confirms that no Android-side or JNI-side surface may be promoted into the missing production registry source.

## 3. Dependency topology constraining future composition

### 3.1 Existing registry dependency direction

Exact manifest:

`crates/prw-registry/Cargo.toml`

Exact C03e-IN blob:

`ec9215d9bad86ac9601e2f2d1bc0ed8461e724c1`

`prw-registry` currently depends on:

- `prw-connectivity`;
- `prw-control-plane`;
- `prw-core`;
- `prw-session`.

The existing direction therefore includes:

`prw-registry -> prw-control-plane`

### 3.2 Existing control-plane provider dependencies do not establish registry ownership

Exact manifest:

`crates/prw-control-plane/Cargo.toml`

Exact C03e-IN blob:

`acf008393686c10f5b9d63605399a608737973f7`

`prw-control-plane` already owns selected provider dependencies used by other authority domains, including `etcd-client` and Google Cloud Spanner.

Provider dependency availability does not mean those providers are selected for the device registry.

Because `prw-registry` already depends on `prw-control-plane`, adding a direct `prw-control-plane -> prw-registry` dependency to implement the missing registry authority would create a dependency cycle and is not selected by C03e-IO.

C03e-IO likewise does not select moving registry semantic ownership into `prw-control-plane`, splitting crates, reversing dependencies, adding a shared provider crate, or otherwise redesigning the architecture.

### 3.3 Agent can consume both domains but is not durable registry authority by default

Exact manifest:

`crates/prw-agent/Cargo.toml`

Exact C03e-IN blob:

`4c70d6be9b56f39edc10810eefa3428314ed7559`

`prw-agent` depends on both `prw-registry` and `prw-control-plane`.

This topology could permit a future dependency-safe composition at the Agent boundary if a separately selected typed production registry provider/handoff exists. It does not authorize the Agent to invent durable registry storage, manufacture current state, connect to an arbitrary provider, or duplicate registry semantics.

## 4. Exact search result

A repository search for existing production persistence/provider use of `WorkspaceDeviceRegistry`, `bind_transport_identity`, current device registry persistence, or an etcd-backed device-registry binding found only:

- the in-memory `prw-registry` implementation;
- request-time `prw-remote-bridge` consumers/tests of that implementation;
- contracts that explicitly defer durable production registry persistence.

No exact C03e-IN source path was identified that already implements or selects:

- durable production `WorkspaceDeviceRegistry` persistence;
- a production device-record database key/value model;
- production registry provider connection/bootstrap;
- production current-device lookup;
- production current transport-binding lookup;
- transactional device lifecycle + transport rotation persistence;
- distributed revocation/currentness propagation;
- a typed durable registry executor/provider handoff to `prw-registry` or `prw-agent`.

Absence of a discovered source seam is treated fail-closed. It is not permission to choose one from available dependencies.

## 5. Closed source-gap selection

C03e-IO selects the following fact and no broader architecture:

> The exact post-C03e-IN repository contains the semantic in-memory current-registry model required to define correct device/transport binding behavior, but it does not contain an already-selected production durable authority source that can supply current `DeviceId` plus current same-device `TransportIdentity`. Production peer population therefore remains blocked at an explicit authority-source gap.

The gap comprises at least these unresolved production responsibilities:

1. durable current device lifecycle authority;
2. durable current transport-binding authority;
3. authoritative same-device join between them;
4. production currentness/revocation/rotation observation;
5. ambiguity/unavailability classification;
6. provider/bootstrap ownership;
7. dependency-safe typed handoff into the existing semantic registry/Agent composition.

## 6. Preserved semantic authority

The existing Phase 130 rules remain the semantic requirements for any future durable implementation:

- `DeviceId` remains the logical device key;
- immutable workspace/user/device/public-identity binding cannot be silently rebound;
- only `Enrolled` devices participate as current devices;
- `Revoked` is terminal for participation;
- current transport identity remains separately rotatable;
- initial bind is exact and non-overwriting;
- rotation is compare-before-mutate and requires a distinct replacement;
- missing or mismatched transport identity fails closed;
- authenticated-session snapshots must be revalidated against current registry state;
- role metadata does not imply capability authority.

A future production provider must preserve these semantics rather than define a competing registry model.

## 7. Provider neutrality remains locked

C03e-IO does not choose any concrete registry persistence provider.

Specifically not selected:

- etcd;
- Google Cloud Spanner;
- PostgreSQL;
- SQLite;
- embedded key/value stores;
- filesystem records;
- object storage;
- another remote service;
- an environment-variable registry;
- Android/client-side state as authority.

The fact that `prw-control-plane` already has etcd and Spanner dependencies is insufficient authority to reuse either one for the registry.

Provider selection must be driven by a separate exact-source/contract decision that preserves Phase 130 semantics and current dependency direction.

## 8. Dependency-cycle prohibition

A future checkpoint must not solve the gap by introducing:

`prw-control-plane -> prw-registry`

while the existing exact dependency remains:

`prw-registry -> prw-control-plane`

unless a separately authorized architecture checkpoint first changes that dependency topology.

C03e-IO does not authorize such an architecture change.

Likewise, a future implementation must not copy `WorkspaceDeviceRegistry` semantics into `prw-control-plane` merely to avoid the cycle.

## 9. No bootstrap-from-request substitution

The existing remote capability bridge can later revalidate an authenticated session against a `WorkspaceDeviceRegistry` and validate a presented transport identity for the same device.

That request-time validation remains a later protected-operation gate.

C03e-IO does not authorize:

- moving that gate earlier as a bootstrap source;
- deriving pre-bootstrap peer identity from an `AuthorizedCapabilityRequest`;
- requiring a completed protected request to create the identity needed to bootstrap reachability;
- creating a lifecycle cycle between reachability bootstrap and request authorization.

## 10. No endpoint or certificate substitution

The missing production registry authority may not be replaced by:

- bind/local/remote `SocketAddr`;
- IP address or port;
- connectivity candidates;
- DNS/public-IP observation;
- transport certificate identity alone;
- systemd credential filename/path alone;
- request/session IDs;
- expected-device scheduling intent alone;
- hard-coded or environment-provided logical device identity without current registry authority.

Transport-certificate verification can prove transport-key identity, but it cannot by itself prove the current logical-device binding required by C03e-IN.

## 11. Failure law for a future authority source

Before returning a production `PeerConnectivityIdentity`, a future authoritative source must fail closed on at least:

- unknown logical device;
- inactive or revoked device;
- absent current transport binding;
- stale expected transport binding;
- mismatched presented/current transport identity;
- ambiguous multiple current records;
- same-device relationship not provable;
- provider state unavailable;
- transaction/read state indeterminate;
- durability/currentness result that cannot be distinguished from stale state.

No default, cached-as-authoritative, first-match, single-entry, old-session, network-address, or test-seed fallback is allowed.

## 12. Next prerequisite — provider/ownership decision only

The next separately gated checkpoint must select the production durable registry provider/ownership topology before source materialization.

That selection must prove from exact source and established contracts:

1. concrete persistence/provider choice or a previously existing provider seam;
2. which crate owns provider connection/bootstrap;
3. which crate retains registry semantic ownership;
4. dependency direction without a cycle;
5. typed provider-to-semantic handoff with no broad/raw provider leakage;
6. durable record/key semantics for current device lifecycle and current transport binding;
7. transactional compare-before-mutate law for transport rotation and lifecycle changes;
8. currentness/revocation visibility across process boundaries;
9. ambiguity/unavailability/provider-failure mapping;
10. recovery/bootstrap behavior without fabricating missing records;
11. credential/TLS/RBAC requirements, if any, kept separately gated from mere provider selection;
12. exact first source-materialization path ceiling and focused validation tests.

C03e-IO does not pre-authorize the outcome of that provider/ownership decision.

## 13. No source-materialization authorization from C03e-IO

C03e-IO authorizes no Rust/Kotlin/Cargo/lockfile/workflow/systemd/package/security source materialization.

In particular, it does not authorize:

- adding a database client dependency;
- adding a registry store trait/adapter;
- adding an etcd or Spanner implementation;
- adding schema/key codecs;
- adding registry environment variables;
- adding systemd credentials;
- persisting registry state;
- migrating current in-memory state;
- changing enrollment/revocation behavior;
- changing transport-identity rotation behavior;
- changing crate dependency direction;
- adding production peer construction;
- activating any production callsite.

## 14. Frozen executable/runtime surfaces

The following remain unchanged and uninvoked by C03e-IO:

- production reachability bootstrap;
- production durable-owner recovery;
- production remote process operation;
- requester/rendezvous production composition;
- process-companion wrapper;
- public `run()`;
- `main.rs`;
- listener/readiness activation;
- candidate publication;
- NAT traversal;
- relay activation;
- peer dialing;
- production retry/reconnect/rebootstrap loops.

## 15. Security and operational invariants

C03e-IO preserves:

- user identity != logical device identity;
- logical `DeviceId` != `TransportIdentity`;
- transport rotation does not replace logical device identity;
- IP/port is reachability, not identity;
- request/session IDs are correlation, not device authority;
- expected-device intent is at most a lookup key for a future authority, not proof;
- requester identity and rendezvous target identity remain separately validated;
- durable reachability ownership remains keyed by exact two-part peer lifecycle.

No private key, secret, credential, certificate, trust root, account token, registry record, or production database value is introduced.

## 16. Exact-head validation requirement

C03e-IO may be semantically closed only after the exact final C03e-IO head proves:

- predecessor/base/merge-base is exact C03e-IN head `adb7ad674638a57473eba5c33c69375841f7549a`;
- branch is ahead only by the bounded docs-only selection commit;
- exactly one contract path changed;
- no source/Cargo/lockfile/workflow/runtime/security path changed;
- automatically triggered Rust validation passes on the exact final head;
- path-filtered workflows are recorded accurately;
- immutable canonical Drive evidence is written and raw-read back;
- PR remains draft/open/unmerged.

No validation result from another head may be inherited.

## 17. Repository and deployment non-authorization

C03e-IO does not authorize or perform:

- repository visibility/configuration mutation;
- merge;
- branch deletion;
- history rewrite;
- production deployment;
- service restart;
- systemd unit/package mutation;
- credential creation/replacement;
- certificate issuance/installation;
- trust/RBAC/auth mutation;
- database/schema/migration mutation;
- firewall/routing/DNS/TUN/TAP mutation;
- listener/readiness activation;
- production-state mutation.

Repository visibility remains whatever the exact repository metadata reports; C03e-IO does not change it.

## 18. Closure meaning

If exact-head validation and evidence recording pass, C03e-IO closure means only:

`PRODUCTION_CURRENT_DEVICE_TRANSPORT_AUTHORITY_SOURCE_GAP_SELECTED`

It means the missing durable production registry authority source has been made explicit and bounded.

It does not mean a provider has been chosen, a durable registry exists, a production peer can now be populated, production networking is active, or deployment is complete.
