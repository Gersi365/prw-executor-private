# Phase 152 C02f-L — etcd-client 0.19.0 Selection Lock

Status: `CLIENT_LIBRARY_SELECTION_LOCK / ETCD_V3_7_PROVIDER_LOCK_INHERITED / ETCD_CLIENT_0_19_0_SELECTED / TOKIO_TONIC_ASYNC_CLIENT_MODEL_ACCEPTED / KV_TXN_SURFACE_REQUIRED / ETCD_3_7_COMPATIBILITY_PROOF_REQUIRED_BEFORE_EXECUTABLE_ACCEPTANCE / DEPENDENCY_NOT_MATERIALIZED / CARGO_MANIFEST_BYTE_STABLE_REQUIRED / FEATURE_FLAGS_DEFERRED / TLS_PROFILE_DEFERRED / KEY_SCHEMA_ENCODING_DEFERRED / TRANSACTION_MAPPING_DEFERRED / CLUSTER_DEPLOYMENT_DEFERRED / RUNTIME_ACTIVATION_DEFERRED / DOCS_ONLY / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Exact predecessor head: `750e50bfbaebdc66c8258643f8569764117403a0`
Exact predecessor tree: `dea72119fccf7886845f400ca88b2280312fbf68`
Predecessor checkpoint: `C02f-K etcd Rust client dependency selection readiness audit`
Approval date: `2026-08-19`

## Purpose

C02f-J selected etcd v3.7 as the T3 shared control-plane live-owner authority backend. C02f-K evaluated Rust client/dependency options and identified `etcd-client 0.19.0` as the preferred candidate for selection review while explicitly leaving client selection open.

The architecture owner has now approved continuing with that recommendation. C02f-L records the exact client-library selection and closes that decision gate without adding or activating the dependency.

This checkpoint is intentionally docs-only. It selects the client library and version target, but it does not mutate Cargo manifests, `Cargo.lock`, production Rust source, runtime wiring, endpoints, credentials, cluster topology, schema, encoding, feature flags or network behavior.

## Selected client

### `etcd-client 0.19.0`

Classification: `SELECTED`.

The selected Rust client library for the initial etcd v3.7 T3 shared control-plane live-owner authority implementation is:

`etcd-client = 0.19.0`

Future implementation work for this authority must target the semantics and API surface of `etcd-client 0.19.0` unless a separately approved architecture/dependency change supersedes this checkpoint.

Selection here means the library and version target are fixed for the next implementation gates. It does **not** mean the crate has been added to any production manifest or that any transitive dependency has been accepted into the lockfile.

## Selection rationale inherited from C02f-K

The selected client is accepted because it provides the direct Rust etcd v3 API surface required by the chosen backend and matches the existing async Rust direction of the repository:

- asynchronous client model backed by Tokio;
- Tonic-based transport stack;
- KV operations;
- transaction (`Txn`) operations;
- cluster and maintenance APIs available for later operational tooling if separately authorized;
- TLS support available behind features;
- no sidecar/service boundary required merely to access etcd.

These properties make it materially simpler than maintaining PRW-owned generated Tonic bindings and avoid the extra process/deployment boundary that an official Go `clientv3` sidecar would introduce.

## Compatibility proof remains mandatory

`etcd-client 0.19.0` selection does not itself prove compatibility with the selected etcd v3.7 server line.

Before executable acceptance, PRW must validate the exact operations it depends on against an approved etcd v3.7 test target, including at minimum:

1. linearizable Get/Range behavior used for authoritative observation;
2. transaction compare/success/failure behavior used for atomic replacement;
3. stale-owner rejection;
4. stale-release isolation;
5. mutation timeout / indeterminate-outcome handling;
6. re-observation after ambiguous outcomes;
7. member or leader failover behavior;
8. no-quorum fail-closed behavior;
9. TLS/mTLS behavior after the trust profile is separately selected;
10. prevention of serializable/stale reads from acting as currentness authority;
11. prevention of blind retries for ownership-changing mutations.

A successful build against the crate is insufficient compatibility evidence.

## Inherited authority semantics

Client selection cannot weaken any previously locked authority rule.

The following remain authoritative:

- T3 shared control-plane authority placement is selected;
- etcd v3.7 is the selected backend;
- cross-host replacement is required;
- the exact namespace is `DeviceId + TransportIdentity`;
- `TransportIdentity` remains separately rotatable from logical `DeviceId` identity;
- IP address, port, NAT mapping, relay path and transient endpoint are not identity;
- `ReachabilityLiveOwnerFence` remains a PRW-owned strictly ordered non-zero logical `u128` generation;
- the fence must be strictly monotonic and never reused or rolled back;
- live-owner replacement must be atomic;
- stale owners must remain permanently rejected once superseded;
- stale release must not clear a newer owner;
- recovery must preserve a durable high-water safety invariant;
- authority ambiguity, no-quorum and unknown authoritative outcomes fail closed;
- clocks, TTL, heartbeat, Lease and Watch are not primary stale-owner safety authority;
- R1-R4 reachability side effects must reject stale fencing at or atomically with their effect boundary.

## Required API usage constraints

When implementation begins, `etcd-client 0.19.0` must be wrapped behind a PRW-owned authority adapter rather than exposed broadly across product modules.

The adapter must enforce the domain rules above and must not let raw etcd concepts redefine PRW semantics.

In particular:

- etcd revision numbers must not automatically become the PRW `u128` live-owner fence;
- Lease IDs must not become owner generations;
- Watch delivery must not establish currentness;
- serializable reads must not establish currentness;
- connection success must not imply authority success;
- retryable transport errors must not automatically imply that a mutating transaction failed;
- unknown mutation outcome must enter explicit recovery/re-observation logic;
- raw client errors must be mapped into bounded PRW authority error categories.

## Deferred dependency materialization

C02f-L does not add `etcd-client` to `crates/prw-control-plane/Cargo.toml` or any other Cargo manifest.

It does not authorize a `Cargo.lock` update.

A later dependency-materialization checkpoint must explicitly decide:

- exact manifest placement;
- exact version pin syntax;
- default-features policy;
- required `etcd-client` feature flags;
- TLS feature selection;
- whether any direct Tokio/Tonic/Prost dependencies are needed or remain transitive only;
- resulting dependency graph review;
- MSRV/build compatibility;
- supply-chain/license review as required by repository policy;
- executable validation after the dependency graph changes.

## Deferred TLS and trust profile

No TLS feature profile is selected here.

The later security/trust checkpoint must decide:

- server authentication requirements;
- client certificate authentication requirements;
- CA/trust-anchor ownership;
- certificate rotation;
- endpoint hostname verification;
- credential loading and storage;
- failure behavior for expired, missing or ambiguous credentials;
- whether the `tls-aws-lc` path is selected.

No credentials or endpoints are introduced in C02f-L.

## Deferred key/value schema and external encoding

C02f-L does not select:

- etcd key prefix;
- namespace serialization;
- `DeviceId` encoding;
- `TransportIdentity` encoding;
- value record schema;
- owner token representation;
- `u128` fence external byte/string representation;
- schema versioning;
- migration format;
- recovery high-water record placement.

Those decisions must preserve the already locked logical namespace and monotonicity semantics.

## Deferred transaction mapping

The exact mapping from PRW live-owner operations to etcd KV transactions remains deferred.

The next state-machine design must specify deterministic transaction forms for at least:

- observe current owner/fence;
- acquire initial owner;
- replace owner with strictly newer fence;
- reject stale owner;
- release only when the exact owner/fence still matches;
- re-observe after indeterminate mutation outcome;
- recover while preserving high-water monotonicity.

No code path may use a non-authoritative read followed by an unfenced write as an ownership transition.

## Deferred cluster deployment and runtime activation

C02f-L selects no cluster member count, AZ/region layout, managed/self-hosted topology, endpoint set, DNS policy, backup/restore procedure or runtime bootstrap.

It performs no:

- outbound etcd connection;
- DNS resolution;
- credential loading;
- client construction;
- background watch;
- lease keepalive;
- network I/O;
- runtime task creation;
- production authority operation.

## Source mutation boundary

Production Rust source, Cargo manifests and `Cargo.lock` must remain byte-stable relative to C02f-K.

No build, rustfmt, Clippy, test or workflow run is required solely for this docs-only client-selection lock because executable source and the dependency graph remain unchanged.

The latest executable validation evidence remains the already closed C02e Tranche 6 canonical PASS until a later executable checkpoint changes production source or dependencies.

## Next gates after C02f-L

The client-library selection gate is closed. The next work must not reopen `etcd-client 0.19.0` selection absent contradictory evidence or explicit redesign.

Remaining implementation gates include:

1. dependency materialization / exact Cargo feature policy;
2. exact key/value schema and external fence encoding;
3. transaction/CAS state-machine mapping;
4. indeterminate-outcome/retry/re-observation behavior;
5. recovery high-water persistence and restore procedure;
6. cluster deployment/availability topology;
7. TLS/auth/credential trust boundary;
8. concrete R1-R4 sink-side fence propagation/rejection;
9. executable etcd v3.7 compatibility tests;
10. runtime activation only after preceding gates pass.

## Locked conclusion

C02f-L closes the Rust client-library selection gate with the authoritative result:

`ETCD_CLIENT_0_19_0_SELECTED_FOR_T3_SHARED_CONTROL_PLANE_LIVE_OWNER_AUTHORITY`

The selection is constrained by all inherited T3/etcd v3.7 safety semantics. Dependency materialization, Cargo feature flags, TLS profile, key/schema/encoding, transaction mapping, cluster deployment and runtime activation remain explicitly deferred.
