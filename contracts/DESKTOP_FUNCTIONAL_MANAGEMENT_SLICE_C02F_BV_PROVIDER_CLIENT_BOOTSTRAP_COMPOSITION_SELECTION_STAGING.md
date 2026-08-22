# Phase 152 C02f-BV — Provider / Client / Bootstrap Composition Architecture Selection Staging

Status: `SELECTED / DOCUMENTATION_ONLY / CONTROL_PLANE_OWNS_ETCD_PROVIDER_CONSTRUCTION / SINGLE_ETCD_PROVIDER_CONTEXT / PREPARATION_RETURN_BOUNDARY / BRIDGE_OWNS_BU_ASYNC_AUTHORITY_COMPOSITION / AF_AG_SECURITY_CONSTRAINTS_INHERITED / FAIL_CLOSED / NO_TLS_FEATURE_MATERIALIZATION / NO_ENDPOINT_VALUES / NO_SECRET_MATERIAL / NO_CONNECT / NO_RUNTIME_ACTIVATION / NO_RECOVERY_EXECUTION / NO_R1_R4_ACTIVATION / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-22
Repository: `Gersi365/prw-executor-private`
Repository ID: `1334911207`

## Approval basis

The user explicitly authorized:

`Autorizoj C02f-BV provider/client/bootstrap composition selection (docs-only).`

This checkpoint selects architecture only. It does not materialize Rust source, mutate Cargo features or lockfiles, create an etcd client, contact an endpoint, load or create certificates/private keys, mutate etcd authentication/RBAC, construct a Spanner client, execute recovery, initialize fence state, create a runtime/task, activate R1-R4 effects, deploy, retarget, or merge any pull request.

## Exact prerequisite

The exact validated prerequisite is closed C02f-BU:

- branch: `phase-152-c02f-bu-full-async-authority-lifecycle-composition-source-materialization-staging`;
- head: `c7cad1e581ff00e72d4c746d2e5f2d52aae0d2c3`;
- tree: `d926ca34902e65c2bb9b4d431cd899415436ad6e`;
- gate: `C02F_BU_FULL_ASYNC_AUTHORITY_LIFECYCLE_COMPOSITION_MATERIALIZED`;
- PR #92 remains draft/open/unmerged.

C02f-BU materialized `ReachabilityLiveOwnerComposedAsyncAuthority` over exactly one already-created `ReachabilityLiveOwnerAcquisitionPreparation`. Acquisition, currentness, and release all use provider state owned by that same preparation facade. Provider/client/bootstrap construction intentionally remained outside BU.

## Purpose

BV closes only the architecture gap between:

1. the already-materialized BU composed async authority, which accepts one already-created preparation facade; and
2. a future production composition root, which must obtain one authenticated etcd provider context without leaking provider-specific construction into bridge orchestration.

BV selects the provider-construction boundary, bootstrap data flow, ownership handoff, failure behavior, and non-activation limits for that gap.

BV does **not** select a process startup owner, deployment platform, concrete endpoint set, concrete secret source, retry scheduler, health/readiness protocol, or production activation sequence.

## Existing validated seams BV must compose, not redesign

### C02f-AD provider seam

`ReachabilityLiveOwnerEtcdStore` already accepts an already-created `etcd_client::KvClient`. Its constructor performs no network I/O. Endpoint selection and `Client::connect` remain outside AD.

BV preserves AD unchanged.

### C02f-BM preparation seam

`ReachabilityLiveOwnerAcquisitionPreparation::new(kv)` already accepts exactly one `KvClient` and internally derives the live-owner and fence-sequence provider handles from that same provider context.

Its internal cloning is a same-context handle split, not permission to supply independently constructed stores or independently connected clusters.

BV preserves BM unchanged.

### C02f-BU bridge composition seam

`ReachabilityLiveOwnerComposedAsyncAuthority::new(preparation)` already accepts exactly one prepared control-plane provider facade and exposes the existing async authority port.

The BU bridge constructor deliberately accepts no endpoint, `Client`, `KvClient`, TLS configuration, credential, retry policy, recovery provider, runtime, or executor.

BV preserves that constructor unchanged.

## Inherited architecture constraints

BV inherits prior closed selections without reopening them.

### Identity boundary

Logical peer identity remains exactly the selected `DeviceId + TransportIdentity` namespace.

The following are not logical peer identity:

- etcd endpoint address;
- stable member FQDN;
- certificate subject or SAN;
- etcd authentication username;
- host UID/GID;
- process identity;
- cloud/service-account identity.

Provider bootstrap may authenticate the PRW workload to the authority backend, but it must not redefine the peer namespace.

### C02f-AF topology boundary

The selected production authority topology remains:

- exactly three voting etcd members initially;
- one voter per independent low-latency failure domain;
- one region / one low-latency consensus locality;
- stable member FQDN identity;
- reachable client/peer endpoint roles;
- no advertised `localhost`, loopback-only, wildcard, or ephemeral-container identity.

BV does not choose concrete FQDNs, ports, DNS zone, platform, region, or member addresses.

### C02f-AG transport/authentication boundary

The selected later production security profile remains:

- `etcd-client = 0.19.0`;
- `default-features = false`;
- only the `tls` feature selected for later materialization;
- `tls-roots` not selected;
- explicit bounded private trust anchors;
- HTTPS only for production authority client traffic;
- server-name verification against the selected stable member FQDN;
- client mTLS using the dedicated runtime identity;
- normal etcd runtime principal `prw-live-owner-runtime`;
- least-privilege role `prw-live-owner-rw` over the exact selected live-owner/fence namespaces as separately selected;
- no root/admin credential on the normal runtime path;
- no plaintext fallback.

The current BU `prw-control-plane` manifest still has `etcd-client = { version = "=0.19.0", default-features = false }` with no TLS feature materialized. BV does not change that manifest.

### Recovery boundary

Recovery-epoch issuance, Spanner provider construction, sequence-head initialization/reconciliation, and disaster-recovery activation remain separate authority operations.

Normal live-owner provider bootstrap must not:

- construct a Spanner `DatabaseClient`;
- issue a recovery epoch;
- select a recovery epoch;
- reset or initialize a fence-sequence head;
- interpret a missing PRWF head as permission to bootstrap one;
- activate authority under a locally invented epoch;
- bypass current-epoch proof.

The existing BM acquisition preparation remains fail closed when the required initialized sequence state is unavailable.

## Selected provider-construction ownership

Provider-specific etcd client construction is selected to remain owned by **`prw-control-plane`**.

A future provider-bootstrap module in `prw-control-plane` may use `etcd-client` provider types internally, including the future AG-selected TLS configuration path.

`prw-remote-bridge` must not add a direct `etcd-client` dependency merely to connect or configure the authority provider.

The provider bootstrap boundary must not be implemented inside `ReachabilityLiveOwnerComposedAsyncAuthority`.

This preserves the layering:

```text
future external composition/bootstrap caller
        |
        | provider-neutral validated bootstrap inputs
        v
prw-control-plane provider bootstrap boundary
        |
        | one authenticated etcd provider context
        v
ReachabilityLiveOwnerAcquisitionPreparation
        |
        | ownership transfer by value
        v
prw-remote-bridge ReachabilityLiveOwnerComposedAsyncAuthority
```

The diagram selects ownership and call direction only. It does not activate any runtime path.

## Selected provider bootstrap result boundary

The selected successful output of the control-plane provider bootstrap boundary is **one already-created `ReachabilityLiveOwnerAcquisitionPreparation`**, not a raw provider handle exported for arbitrary use.

Conceptually:

```text
validated authority bootstrap inputs
        -> one etcd connection context
        -> one KvClient derived from that context
        -> ReachabilityLiveOwnerAcquisitionPreparation::new(kv)
        -> return preparation
```

Exact Rust naming and error type remain mechanical details for a separately authorized source-materialization tranche, but the ownership semantics are fixed by BV.

The future boundary must not return or expose to bridge callers:

- `etcd_client::Client`;
- `etcd_client::KvClient`;
- `ReachabilityLiveOwnerEtcdStore`;
- `FenceSequenceAllocationEtcdStore`;
- generic Get/Put/Txn clients;
- raw TLS provider objects;
- credentials or private-key bytes;
- an independently clonable arbitrary provider escape hatch.

This is a composition boundary, not a general-purpose etcd client factory.

## Single-provider-context invariant

One successful provider-bootstrap operation creates exactly one logical etcd provider context for the BU authority instance.

The context may contain the selected set of client endpoints for one logical three-member authority cluster. “Single provider context” does **not** mean “single member endpoint.” It means one connected/configured authority context with one trust/credential profile and one logical cluster target.

From that one context:

1. exactly one `KvClient` is derived for preparation construction;
2. BM may clone that handle internally only to split the already-selected fence-sequence and live-owner stores;
3. BU acquisition/currentness/release continue to use the exact preparation-owned context;
4. no second `Client::connect` is performed merely for currentness, release, allocation, first-owner, or replacement execution;
5. no second cluster, endpoint policy, credential profile, or trust bundle may be substituted for one lifecycle operation.

## Selected bootstrap input categories

BV selects only the categories of input that a future provider bootstrap boundary may consume. Concrete values and loading mechanisms remain deferred.

The boundary may consume validated, caller-supplied provider-bootstrap material representing:

- a non-empty production client endpoint set for the one selected etcd cluster;
- the bounded private CA/trust material required by AG;
- the dedicated runtime client certificate and private-key identity required by AG;
- any provider-native connection options that are strictly required to apply the selected AG security profile.

The boundary must not discover or invent these values from peer requests.

The boundary must not accept logical `DeviceId`, `TransportIdentity`, remote file paths, forwarding targets, terminal commands, or other per-request data as authority provider configuration.

## Endpoint constraints at the bootstrap boundary

A future materialization must reject invalid production endpoint configuration before treating the provider as usable.

Selected constraints:

- endpoint set must be non-empty;
- production client endpoints must use `https://`;
- endpoint host identity must be compatible with the AF stable-member-FQDN model;
- loopback-only, wildcard, and empty host identities are invalid for production authority bootstrap;
- plaintext `http://` fallback is not permitted;
- endpoint configuration must represent one logical authority cluster rather than a fallback list of unrelated clusters;
- no peer-request-controlled endpoint may enter provider bootstrap;
- no DNS or endpoint mutation is performed by this boundary.

BV does not select the concrete endpoint count supplied to the client library, endpoint ordering, load-balancing policy, dial timeout, keepalive values, reconnect policy, or service-discovery mechanism.

## TLS and credential application boundary

A future source tranche may apply the already-selected AG TLS profile while constructing the provider context, but BV itself does not authorize that dependency or source materialization.

The future composition must preserve:

- explicit bounded CA/trust anchors rather than host-native root fallback;
- normal server-name verification;
- dedicated runtime mTLS client identity;
- no password/token fallback on the normal runtime path;
- no root/admin credential substitution;
- no automatic/self-generated etcd TLS;
- no logging of private-key, certificate-private material, or secret contents.

Certificate acquisition, parsing, storage, rotation, reload, filesystem permissions, secret-manager choice, and key custody remain separate security/runtime bindings.

## Connection operation semantics

The future provider bootstrap boundary may perform exactly the provider connection operation necessary to establish one authenticated etcd client context.

BV selects these semantics:

1. validate the supplied bootstrap configuration shape;
2. construct the selected provider connection/TLS options from those validated inputs;
3. perform one logical etcd client connection operation for the selected cluster context;
4. derive one `KvClient` from that successful context;
5. immediately wrap that `KvClient` in one `ReachabilityLiveOwnerAcquisitionPreparation`;
6. return only the preparation facade across the selected boundary.

No live-owner Get/Txn, fence allocation, currentness proof, release, recovery operation, or R1-R4 effect is part of provider construction itself.

Whether the underlying provider library internally maintains channels across multiple configured cluster members is provider behavior inside the one logical context and does not violate the single-context invariant.

## Retry and fallback policy

BV selects **no outer connection retry scheduler**.

A future materialization may expose one async connection attempt whose failure is returned to its caller. Process-level retry/backoff, startup retry budget, reconnection, circuit breaking, and health-driven replacement remain a later runtime/liveness selection.

The provider bootstrap boundary must not:

- silently retry forever;
- fall back to plaintext;
- fall back to a different cluster;
- fall back to a local in-memory authority;
- reuse stale cached authority proof as a substitute for connection;
- manufacture a successful preparation after connection/authentication failure.

## Fail-closed bootstrap result

Provider bootstrap succeeds only when the selected provider connection context has been constructed successfully under the supplied validated security configuration.

Any of the following must fail closed and return no usable preparation:

- invalid/empty endpoint configuration;
- invalid security-profile input shape;
- inability to establish the provider client context;
- TLS validation failure;
- client-certificate authentication failure;
- provider configuration error;
- provider/client construction error whose success cannot be proven.

A bootstrap error is not `Granted`, `Current`, `Released`, or a recovered authority state.

## Composition into BU

After and only after successful provider bootstrap, the future composition caller may pass the returned preparation by value into the already-materialized BU constructor:

```text
preparation
    -> ReachabilityLiveOwnerComposedAsyncAuthority::new(preparation)
    -> existing ReachabilityLiveOwnerAsyncAuthority surface
```

No additional provider handle is passed to the bridge.

No bridge-side reconnect, endpoint selection, TLS configuration, or provider construction is selected.

BU remains the only selected full async live-owner lifecycle composition. BV does not introduce a parallel acquisition/currentness/release implementation.

## Process/runtime ownership remains deferred

BV deliberately does not select which final process/module owns the production startup call that invokes provider bootstrap and stores/injects the resulting BU authority.

Specifically deferred:

- `prw-agent` production startup integration;
- a dedicated control-plane daemon/process;
- executor/runtime creation and ownership;
- task spawning;
- shutdown ordering;
- restart policy;
- reconnect policy;
- health/readiness endpoints;
- dependency-injection container shape;
- global singleton storage;
- concurrency/sharding of authority instances.

No `tokio::main`, `block_on`, nested runtime, detached task, background worker, `Arc<Mutex<_>>`, or service activation is selected by BV.

## Readiness is not provider construction

A successful etcd connection does not by itself prove PRW authority readiness.

BV therefore rejects interpreting “provider connected” as any of:

- current recovery epoch proven;
- PRWF sequence head initialized/current;
- live-owner namespace valid;
- R1-R4 effect fencing active;
- remote reachability effects safe to expose;
- production service ready.

Those proofs and activation gates remain owned by their separately selected authority/recovery/runtime layers.

## Recovery and normal-start separation

Normal provider bootstrap and disaster-recovery bootstrap are separate operations.

BV selects no automatic behavior that, on missing or malformed authority state, issues a new epoch, initializes a new sequence head, restores etcd state, or starts a replacement cluster.

A normal runtime encountering missing required initialized state must fail closed through the existing provider/orchestration semantics.

Recovery remains explicit and must satisfy the already-selected external epoch and sequence initialization contracts before normal live-owner acquisition can succeed.

## Security material lifetime boundary

BV does not select ownership or lifetime of raw secret bytes after provider construction.

A future security/materialization tranche must minimize secret exposure and ensure that bootstrap composition does not retain duplicate plaintext private-key material merely for convenience.

The bridge authority must not expose credentials, trust bundles, endpoints, or provider client internals through its public async authority interface.

## Error ownership

BV selects a bounded bootstrap error boundary distinct from semantic live-owner authority results.

Future provider-bootstrap errors may distinguish only what is operationally necessary for safe startup diagnostics, but they must not be converted into semantic authority success.

The bootstrap error surface must not contain secret material or full private-key/certificate payloads.

Exact Rust enum variants remain a source-materialization detail.

## Testability requirements for later source materialization

A future source tranche must preserve deterministic validation without requiring production endpoints or secrets.

At minimum, the materialized boundary should be testable for:

- configuration-shape rejection before connection;
- exact one-context/one-preparation ownership shape;
- absence of raw `Client`/`KvClient` escape across the public bootstrap result;
- BU composition from the returned preparation without adding `etcd-client` to `prw-remote-bridge`;
- fail-closed propagation of provider connection failure;
- no automatic recovery/fence initialization;
- no runtime/task creation;
- no plaintext or unrelated-cluster fallback.

Disposable provider integration, if later used, must remain non-production and separately validated.

## Cargo/dependency boundary

BV does not mutate dependencies.

In particular, this checkpoint does not:

- enable `etcd-client` feature `tls`;
- enable `tls-roots`;
- add `etcd-client` to `prw-remote-bridge`;
- add `etcd-client` directly to `prw-agent`;
- add a runtime/executor dependency;
- add a secret-manager/KMS dependency;
- add a DNS/service-discovery dependency.

Any future source tranche that needs the AG-selected `tls` feature must receive separate explicit authorization that includes dependency/lockfile materialization.

## Explicitly rejected designs

BV rejects the following as the selected first production composition:

1. bridge constructor accepts endpoints/TLS/credentials directly;
2. bridge creates its own `etcd_client::Client`;
3. acquisition, currentness, and release each establish independent provider connections;
4. fence-sequence and live-owner stores are supplied from independently constructed clients;
5. provider bootstrap returns a raw general-purpose `KvClient` to bridge callers;
6. normal startup silently initializes recovery/fence state;
7. connection failure falls back to local or stale authority;
8. provider bootstrap owns an infinite retry/background loop;
9. production bootstrap permits HTTP or native-root fallback contrary to AG;
10. normal runtime uses root/admin credentials;
11. provider connection success is treated as production readiness;
12. peer/request identity controls provider endpoints or credentials.

## Selected layering after BV

The selected non-activated architecture is:

```text
future process/runtime composition root          (still deferred)
        |
        | validated config + security material
        v
prw-control-plane provider bootstrap boundary    (BV selected; source not materialized)
        |
        | one authenticated etcd provider context
        | -> one KvClient -> one preparation
        v
ReachabilityLiveOwnerAcquisitionPreparation      (BM/BU existing)
        |
        +-- acquisition execution                 (BS/BQ/BP existing)
        +-- lifecycle currentness                 (BF/BU existing)
        +-- lifecycle release                     (BD/BU existing)
        |
        v
ReachabilityLiveOwnerComposedAsyncAuthority      (BU existing)
        |
        v
higher-level runtime/effect integration           (still deferred)
```

The recovery-epoch Spanner authority and recovery/sequence initialization orchestration remain separate from this normal live-owner provider bootstrap path.

## Source-stability requirement

Because BV is documentation-only:

- no Rust source file may change;
- no Android source file may change;
- no Agent source file may change;
- no Cargo manifest or lockfile may change;
- no workflow may change;
- no secret/config file may be added;
- no endpoint value may be added;
- no deployment/runtime configuration may be added.

The intended BV net diff is exactly one new contract file.

## Next dependent tranche

After BV is validated and frozen, a later separately authorized tranche may materialize the selected provider/client/bootstrap boundary in source.

That later tranche must reverify:

- exact BV head and gate;
- current `etcd-client` manifest/lock state;
- AG security selection and any explicit authorization to materialize the `tls` feature;
- AF endpoint/FQDN constraints;
- BU single-provider-context ownership;
- absence of runtime/deployment authorization.

The later tranche may not infer permission to create endpoints, certificates, private keys, etcd users/roles, Spanner resources, runtime tasks, production connections, or deployment from BV.

## Authorization boundary

`C02F_BV_PROVIDER_CLIENT_BOOTSTRAP_COMPOSITION_SELECTION_ONLY / DOCUMENTATION_ONLY / NO_SOURCE_MATERIALIZATION / NO_TLS_FEATURE_MATERIALIZATION / NO_SECRET_CREATION / NO_ENDPOINT_CONTACT / NO_AUTH_RBAC_MUTATION / NO_RECOVERY_EXECUTION / NO_RUNTIME_ACTIVATION / NO_R1_R4_ACTIVATION / NO_DEPLOYMENT / NO_MERGE`

Any Rust/Cargo source materialization, TLS feature enablement, endpoint/client connection, certificate/secret creation, etcd auth/RBAC mutation, recovery execution, runtime activation, Agent integration, deployment, retargeting, or merge requires separate explicit authorization.

## Gate

After exact diff verification, draft-PR validation/evidence capture, immutable audit persistence, rolling-status append/readback, and final read-only repository recheck complete without contradiction, the checkpoint gate is:

`C02F_BV_PROVIDER_CLIENT_BOOTSTRAP_COMPOSITION_SELECTED`
