# Phase 152 C02f-BV — Provider / Client / Bootstrap Composition Architecture Selection Staging

Status: `SELECTED / DOCUMENTATION_ONLY / SECURITY_RECONCILIATION_REQUIRED / ONE_LOGICAL_ETCD_AUTHORITY_CLUSTER / TWO_ROLE_SCOPED_AUTHENTICATED_CLIENT_CONTEXTS / ONE_PREPARATION_FACADE / BRIDGE_OWNED_BU_ASYNC_AUTHORITY_PRESERVED / BM_SINGLE_KVCLIENT_PRODUCTION_INCOMPATIBILITY_IDENTIFIED / CONTROL_PLANE_OWNS_PROVIDER_CONSTRUCTION / FAIL_CLOSED / NO_SOURCE_MATERIALIZATION / NO_TLS_FEATURE_MATERIALIZATION / NO_ENDPOINT_VALUES / NO_SECRET_MATERIAL / NO_CONNECT / NO_RUNTIME_ACTIVATION / NO_RECOVERY_EXECUTION / NO_R1_R4_ACTIVATION / NO_DEPLOYMENT / NO_MERGE`

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

C02f-BU materialized `ReachabilityLiveOwnerComposedAsyncAuthority` over exactly one already-created `ReachabilityLiveOwnerAcquisitionPreparation`. Provider/client/bootstrap construction intentionally remained outside BU.

## Purpose

BV closes the architecture gap between:

1. the already-materialized BU composed async authority; and
2. the future production provider/bootstrap path that must create the authenticated etcd clients needed by the already-selected live-owner and fence-sequence authority operations.

During BV audit, the exact closed AG, AI, BM and BU states reveal a real production-security incompatibility that must be resolved before any provider bootstrap source is materialized.

BV therefore does two things, and only these two things:

1. records the incompatibility explicitly instead of silently weakening prior security selections; and
2. selects the role-separated provider/bootstrap composition that later source materialization must implement while preserving the BU bridge-facing one-preparation-facade boundary.

## Audit-discovered prerequisite conflict

### AG selected live-owner runtime identity

C02f-AG selected the normal etcd live-owner runtime principal:

`prw-live-owner-runtime`

with least-privilege role:

`prw-live-owner-rw`

bounded to the exact live-owner prefix:

`/prw/reachability/live-owner/`

AG explicitly rejected normal-runtime root/admin credentials, broad unrelated keyspace access, plaintext fallback, and password/token fallback in addition to the selected mTLS certificate identity.

### AI selected a separate fence allocator identity

C02f-AI explicitly did **not** widen the existing AG live-owner runtime role.

AI selected a future separately bounded etcd allocator principal/role, conceptually:

`prw-fence-allocator-runtime`

limited to:

`/prw/reachability/fence-sequence/`

AI therefore preserves security-principal separation between:

- live-owner mutation/currentness/release authority; and
- within-epoch fence-sequence allocation authority.

### BM currently uses one authenticated KvClient for both roles

The materialized C02f-BM constructor currently has the shape:

```rust
ReachabilityLiveOwnerAcquisitionPreparation::new(kv: KvClient)
```

and derives from that one supplied `KvClient` both:

- `ReachabilityLiveOwnerEtcdStore`; and
- `FenceSequenceAllocationEtcdStore`.

Those internal handles originate from one authenticated etcd client context.

### Why the current production shape cannot satisfy both prior security selections

Under the selected AG certificate-CN authentication model, one authenticated client context has one normal etcd authentication identity.

Therefore the current BM one-`KvClient` production shape cannot simultaneously preserve all of the following without weakening a closed selection:

1. AG `prw-live-owner-runtime` with only live-owner-prefix privilege;
2. AI separate `prw-fence-allocator-runtime` principal/role with only fence-sequence-prefix privilege;
3. no password/token identity switching;
4. no root/admin or broad union credential;
5. one BM `KvClient` used for both live-owner and fence-allocation operations.

Granting the live-owner principal the allocator role would collapse the AI-selected principal separation.

Granting the allocator principal the live-owner role would collapse the same separation in the opposite direction.

Using root/admin or a new broad union principal would violate AG least privilege and AI role separation.

Using per-request password/token identity switching would violate the selected AG normal-runtime authentication model.

BV therefore records the current BM one-`KvClient` production bootstrap shape as **incompatible with the combined AG + AI security selections**.

This is not a claim that the current BU source is semantically invalid for its validated source tranche. It is a production-bootstrap compatibility finding discovered only when the deferred provider/security composition is now being selected.

## Production activation guard

Until a separately authorized source tranche corrects the provider-preparation construction boundary, the current BM/BU one-`KvClient` composition must **not** be activated as the production AG+AI-authenticated authority bootstrap path.

No production endpoint, credential, certificate, etcd user/role, runtime or deployment may be attached to that incompatible shape merely to avoid the correction.

## Selected reconciliation

BV selects:

**one logical etcd authority cluster, two role-scoped authenticated etcd client contexts, one outward preparation facade, and one BU bridge-owned async authority.**

The two authenticated client contexts are:

1. **live-owner client context**
   - authentication identity: AG-selected `prw-live-owner-runtime`;
   - role: AG-selected `prw-live-owner-rw`;
   - authority scope: exact live-owner namespace required by AD/AE/BF/BD/BP/BQ/BU semantics;

2. **fence allocator client context**
   - authentication identity: AI-selected separate allocator principal, conceptually `prw-fence-allocator-runtime`;
   - role: separately bounded allocator role selected by AI;
   - authority scope: exact `/prw/reachability/fence-sequence/` namespace required by AN/AO/AP/AQ/BM allocation semantics.

The two client contexts must target the **same exact logical etcd authority cluster configuration** while using distinct client identities and least-privilege role bindings.

## Same-cluster invariant replaces same-authenticated-session assumption

The safety property required by BT/BU is that acquisition preparation, fence allocation, live-owner mutation, currentness and release cannot silently diverge onto unrelated authority backends.

BV preserves that safety property while correcting the credential boundary.

The production invariant is now expressed as:

```text
one immutable authority-cluster configuration
        |
        +-- live-owner authenticated client context
        |      identity: prw-live-owner-runtime
        |      scope: /prw/reachability/live-owner/
        |
        +-- fence-allocator authenticated client context
               identity: prw-fence-allocator-runtime
               scope: /prw/reachability/fence-sequence/

both contexts -> one preparation facade -> one BU composed async authority
```

“Same authority backend” means the same configured logical etcd cluster and trust domain, not one shared security principal.

No caller may independently supply an arbitrary live-owner cluster and an arbitrary allocator cluster.

## Selected cluster-configuration ownership

Provider-specific etcd client construction remains owned by **`prw-control-plane`**.

A future control-plane bootstrap boundary must receive one validated immutable cluster configuration and derive both role-scoped client contexts from that same cluster configuration.

The caller must not provide two unrelated endpoint sets.

At minimum, both role-scoped client contexts must inherit the same selected:

- logical authority-cluster configuration object;
- production client endpoint set;
- AF stable-member-FQDN constraints;
- bounded server trust domain / explicit CA roots selected by AG;
- server-name verification policy;
- transport-security policy.

Only the client authentication identity/private-key material differs by role.

Concrete endpoint values, DNS zone, ports, CA bytes, certificates, private keys and secret-loading mechanisms remain deferred.

## Cluster identity proof remains a later runtime gate

BV prevents configuration-level split authority by deriving both role-scoped clients from one immutable cluster configuration.

BV does not yet select an active provider-I/O proof such as etcd cluster-ID/status verification, because this docs-only checkpoint authorizes no endpoint contact or readiness protocol.

A later runtime/readiness selection may require explicit cluster-ID consistency proof before activation. Such a proof may strengthen but may not weaken the BV same-cluster invariant.

## Selected corrected preparation boundary

A later separately authorized source tranche must correct the current BM construction boundary so the preparation facade can own the two role-scoped provider contexts without exposing them to bridge callers.

Conceptually, a future provider-specific construction path is equivalent to:

```text
ReachabilityAuthorityEtcdProviderContext {
    live_owner_kv: role-scoped KvClient,
    fence_allocator_kv: role-scoped KvClient,
}
        -> ReachabilityLiveOwnerAcquisitionPreparation
```

Exact Rust naming is not selected by BV.

The important invariants are:

- the live-owner store receives only the live-owner authenticated client context;
- the fence-sequence allocation store receives only the allocator authenticated client context;
- both contexts derive from one immutable same-cluster configuration;
- the preparation object remains the only outward owner passed into BU bridge composition;
- neither raw client escapes through the public bridge-facing API.

The later source tranche may alter the BM constructor or add a replacement provider-specific constructor/factory, but it must preserve the existing provider-neutral preparation/orchestration semantics and tests.

## BU bridge boundary remains selected and unchanged

BV does not reopen the BU bridge constructor:

```rust
ReachabilityLiveOwnerComposedAsyncAuthority::new(preparation)
```

The bridge still receives one preparation facade by value.

The bridge must not receive:

- endpoint strings;
- `etcd_client::Client`;
- `etcd_client::KvClient`;
- TLS configuration;
- CA/certificate/private-key material;
- etcd usernames/roles;
- retry/backoff policy;
- Spanner recovery provider;
- runtime/executor handles.

BU remains the selected full async live-owner lifecycle composition. BV introduces no parallel acquisition/currentness/release state machine.

## Selected provider-bootstrap ownership

The future provider bootstrap boundary remains in `prw-control-plane`, because that crate already owns the etcd provider-specific stores and `etcd-client` dependency.

`prw-remote-bridge` must not add a direct `etcd-client` dependency merely to construct authority clients.

The bootstrap boundary must not be implemented inside `ReachabilityLiveOwnerComposedAsyncAuthority`.

The selected layering is:

```text
future external composition/runtime caller        (still deferred)
        |
        | provider-neutral validated bootstrap material
        v
prw-control-plane provider bootstrap boundary     (BV selected; source deferred)
        |
        +-- live-owner role-scoped etcd client
        +-- fence-allocator role-scoped etcd client
        |
        v
corrected ReachabilityLiveOwnerAcquisitionPreparation
        |
        v
prw-remote-bridge ReachabilityLiveOwnerComposedAsyncAuthority
        |
        v
higher-level runtime/effect integration           (still deferred)
```

## Selected bootstrap input categories

BV selects only categories of future input, never concrete values.

One immutable authority-cluster configuration may contain validated representations of:

- a non-empty production client endpoint set for one exact logical etcd cluster;
- the bounded private server trust material required by AG;
- provider-native connection options strictly required to apply the selected transport-security profile.

Role-specific client identity material is separate:

- live-owner runtime client certificate/private-key identity;
- fence-allocator runtime client certificate/private-key identity.

The bootstrap boundary must not accept peer-request data as provider configuration.

`DeviceId`, `TransportIdentity`, file paths, forwarding targets, terminal commands, remote request payloads and user-controlled network destinations must never select authority endpoints or credentials.

## Endpoint constraints

Future materialization must reject invalid production endpoint configuration before returning usable provider contexts.

Selected constraints:

- endpoint set is non-empty;
- production authority client endpoints use `https://` only;
- host identities conform to the AF stable-member-FQDN model;
- loopback-only, wildcard and empty host identities are invalid for production authority bootstrap;
- no plaintext `http://` fallback;
- no fallback list spanning unrelated clusters;
- both role-scoped client contexts derive from the exact same immutable endpoint set;
- no peer/request-controlled endpoint enters provider bootstrap;
- no DNS or endpoint mutation occurs in this boundary.

BV does not select concrete endpoints, endpoint ordering, load-balancing behavior, dial timeout, keepalive, service discovery or reconnect policy.

## TLS and credential constraints

BV inherits AG without materializing it.

A future source tranche may apply the AG-selected TLS profile only under separate explicit authorization that includes dependency/lockfile materialization.

Both role-scoped client contexts must preserve:

- explicit bounded private trust anchors;
- normal server-name verification;
- production HTTPS;
- dedicated mTLS client identity;
- no native-root fallback;
- no plaintext fallback;
- no root/admin credential substitution;
- no automatic/self-generated etcd TLS;
- no secret logging.

The live-owner and allocator contexts must not reuse the same client private key merely to simplify bootstrap.

Certificate issuance, key custody, secret storage, rotation, reload and filesystem/secret-manager integration remain later security/runtime bindings.

## Connection operation semantics

The future provider bootstrap boundary may perform only the provider construction needed to produce the two role-scoped clients and corrected preparation facade.

Selected sequence:

1. validate one immutable authority-cluster configuration;
2. validate presence/shape of the live-owner client identity input;
3. validate presence/shape of the fence-allocator client identity input;
4. construct the live-owner TLS/provider connection options from the shared cluster configuration plus live-owner identity;
5. construct the allocator TLS/provider connection options from the same shared cluster configuration plus allocator identity;
6. perform one logical etcd client connection operation for the live-owner role context;
7. perform one logical etcd client connection operation for the allocator role context;
8. derive only the role-appropriate `KvClient` handle from each successful context;
9. construct one corrected preparation facade from those two role-scoped handles;
10. return only the preparation facade across the bridge-facing composition boundary.

No live-owner Get/Txn, fence allocation, currentness proof, release, recovery issuance, sequence initialization or R1-R4 effect activation is part of provider construction itself.

## Partial-bootstrap failure semantics

Bootstrap succeeds only if **both** required role-scoped provider contexts are successfully constructed.

If live-owner client construction succeeds but allocator client construction fails, the bootstrap attempt fails closed and returns no usable preparation.

If allocator client construction succeeds but live-owner client construction fails, the bootstrap attempt fails closed and returns no usable preparation.

A partially created client must not be exposed as degraded authority.

Cleanup/drop behavior may rely on ordinary provider object destruction at this architecture stage; no background compensation task is selected.

## Retry and fallback policy

BV selects no outer connection retry scheduler.

A later source implementation may return failure from one bounded bootstrap attempt to its caller. Process-level startup retry/backoff, reconnect, circuit breaking and replacement remain later runtime/liveness selections.

The bootstrap boundary must not:

- retry forever;
- fall back to plaintext;
- fall back to another cluster;
- fall back to a broad union credential;
- fall back to root/admin;
- collapse both roles onto whichever credential connected successfully;
- fall back to local/in-memory authority;
- manufacture a preparation after partial or ambiguous bootstrap failure.

## Fail-closed bootstrap errors

Any of the following must produce no usable preparation:

- invalid/empty shared endpoint configuration;
- mismatched role/configuration shape;
- missing live-owner identity input;
- missing allocator identity input;
- TLS validation failure;
- mTLS client-authentication failure;
- inability to construct either provider context;
- provider/client construction ambiguity;
- any condition that would require weakening AG or AI to continue.

Bootstrap failure is not `Granted`, `Current`, `Released`, recovered authority, sequence initialization, or production readiness.

## Readiness is separate from provider construction

Successful construction of both role-scoped clients does not prove:

- they reached the same actual cluster instance at runtime;
- the current recovery epoch is proven;
- PRWF sequence state is initialized/current;
- live-owner state is valid;
- R1-R4 stale-fence rejection is active;
- remote effects are safe to expose;
- production service readiness.

Those proofs remain later authority/runtime gates.

## Recovery boundary remains separate

Normal live-owner provider bootstrap must not:

- construct the Spanner recovery provider;
- issue or select a recovery epoch;
- reset or initialize PRWF state;
- restore or replace an etcd cluster;
- interpret missing fence state as permission to create production authority;
- activate under a locally invented epoch.

The existing fail-closed missing-sequence-head behavior remains the safe normal-runtime behavior until explicit recovery/initialization has been completed by its separately selected orchestration.

## Process/runtime owner remains deferred

BV does not select which final process/module invokes provider bootstrap and retains/injects the resulting BU authority.

Still deferred:

- `prw-agent` production startup integration;
- dedicated control-plane process selection;
- runtime/executor creation and ownership;
- task spawning;
- shutdown ordering;
- restart/reconnect policy;
- health/readiness endpoints;
- dependency-injection container;
- singleton/global storage;
- concurrency/sharding of authority instances.

No `tokio::main`, nested runtime, `block_on`, detached worker, retry task, `Arc<Mutex<_>>` authority container, systemd activation, deployment or production connection is selected.

## Security material lifetime boundary

BV does not select how raw secret bytes are obtained or retained.

Future materialization must minimize duplicate plaintext secret retention and must not expose credentials, private keys, trust bundles or provider client handles through the BU async-authority API.

Role-specific client identities remain separate security material even though both clients target one authority cluster.

## Error ownership

BV selects a bootstrap-error boundary distinct from semantic live-owner authority results.

Future errors may distinguish configuration validation, live-owner client construction, allocator client construction and provider failures only as needed for safe diagnostics.

Errors must not contain private keys, full secret/certificate payloads or authorization tokens.

No bootstrap error may be converted into semantic authority success.

## Later source-materialization requirements

A separately authorized source tranche must treat the current BM single-`KvClient` constructor as a compatibility/test surface unless it is proven safe for a non-production use case.

The production provider-specific construction path must use the BV role-separated shape.

That tranche must validate at minimum:

- one immutable cluster configuration feeds both role-specific client constructors;
- distinct client identity inputs are required;
- live-owner operations cannot receive the allocator client;
- fence allocation cannot receive the live-owner client;
- raw provider clients do not escape to `prw-remote-bridge`;
- BU bridge constructor remains one-preparation-by-value;
- no direct `etcd-client` dependency is added to `prw-remote-bridge` merely for bootstrap;
- no automatic recovery/sequence initialization;
- no runtime/task creation;
- no fallback to broad credentials or unrelated clusters;
- canonical Rust/Android regressions remain clean.

Disposable provider integration, if later used, remains non-production evidence only.

## Cargo/dependency boundary

BV changes no dependency.

This checkpoint does not:

- enable `etcd-client` feature `tls`;
- enable `tls-roots`;
- add `etcd-client` to `prw-remote-bridge`;
- add `etcd-client` directly to `prw-agent`;
- add runtime/executor dependencies;
- add secret-manager/KMS dependencies;
- add DNS/service-discovery dependencies.

The current BU `prw-control-plane` manifest remains `etcd-client = { version = "=0.19.0", default-features = false }`.

Any source tranche that enables the AG-selected `tls` feature requires separate explicit authorization covering Cargo/lockfile materialization.

## Explicitly rejected designs

BV rejects:

1. one broad/root/admin client used for live-owner and fence allocation;
2. binding both AG and AI permissions to `prw-live-owner-runtime` merely to preserve the current BM constructor;
3. binding both permissions to `prw-fence-allocator-runtime`;
4. password/token role switching on the selected normal mTLS path;
5. live-owner and allocator clients targeting independently supplied cluster configurations;
6. bridge constructor accepting endpoints/TLS/credentials;
7. bridge owning `Client::connect`;
8. acquisition/currentness/release establishing independent clients;
9. provider bootstrap returning raw general-purpose `KvClient` handles to bridge callers;
10. normal startup silently issuing recovery epochs or initializing fence state;
11. connection failure falling back to local/stale authority;
12. provider bootstrap owning infinite retry/background loops;
13. production HTTP/native-root fallback contrary to AG;
14. treating provider connection success as production readiness;
15. peer/request identity selecting provider endpoints or credentials.

## Relationship to BT/BU same-provider selection

BT/BU selected a critical anti-divergence invariant: the full lifecycle must not silently use unrelated authority backends.

BV preserves the **anti-divergence intent** but refines the deferred production provider construction required to satisfy the later-composed AG+AI credential model.

The bridge-facing invariant remains one preparation facade and one composed async authority.

The provider-internal production implementation must use two role-scoped authenticated clients derived from one same-cluster configuration.

This BV refinement is necessary because the previously deferred security/bootstrap layer makes one shared authenticated `KvClient` incompatible with the closed AG+AI least-privilege selections.

## Source-stability requirement

BV is documentation-only.

The intended final net diff from BU is exactly one new contract file.

No Rust source, Android source, Agent source, Cargo manifest, lockfile, workflow, secret/config file, endpoint value, deployment file or runtime configuration may appear in the BV net diff.

The initial BV documentation commit is preserved in history as the pre-reconciliation draft. The corrective documentation commit records the audit-discovered AG/AI/BM incompatibility without force-push or history rewriting.

## Next dependent tranche

After BV is validated and frozen, the next source boundary requires separate explicit authorization.

That source tranche must include the **provider-preparation credential-separation corrective** before any production provider bootstrap can be considered activatable.

Before mutation it must reverify:

- exact BV head and gate;
- current BM/BU preparation and bridge source;
- AG live-owner principal/role/prefix selection;
- AI separate allocator principal/role/prefix selection;
- current `etcd-client` Cargo/lock state;
- AF stable-FQDN/topology constraints;
- absence of deployment/runtime authorization.

If the same tranche also enables the AG-selected `tls` feature, its authorization must explicitly cover dependency/lockfile materialization.

It may not infer permission to create endpoints, certificates/private keys, etcd users/roles, Spanner resources, runtime tasks, production connections, recovery operations, deployment or merge.

## Authorization boundary

`C02F_BV_PROVIDER_CLIENT_BOOTSTRAP_COMPOSITION_SELECTION_ONLY / DOCUMENTATION_ONLY / ROLE_SEPARATED_PROVIDER_CONTEXTS_SELECTED / SAME_LOGICAL_CLUSTER_REQUIRED / BM_SINGLE_KVCLIENT_PRODUCTION_ACTIVATION_BLOCKED_PENDING_CORRECTIVE / NO_SOURCE_MATERIALIZATION / NO_TLS_FEATURE_MATERIALIZATION / NO_SECRET_CREATION / NO_ENDPOINT_CONTACT / NO_AUTH_RBAC_MUTATION / NO_RECOVERY_EXECUTION / NO_RUNTIME_ACTIVATION / NO_R1_R4_ACTIVATION / NO_DEPLOYMENT / NO_MERGE`

Any Rust/Cargo source materialization, BM constructor correction, TLS feature enablement, endpoint/client connection, certificate/secret creation, etcd auth/RBAC mutation, recovery execution, runtime activation, Agent integration, deployment, retargeting or merge requires separate explicit authorization.

## Gate

After exact diff verification, draft-PR validation/evidence capture, immutable audit persistence, rolling-status append/readback and final read-only repository recheck complete without contradiction, the checkpoint gate is:

`C02F_BV_PROVIDER_CLIENT_BOOTSTRAP_COMPOSITION_SELECTED`
