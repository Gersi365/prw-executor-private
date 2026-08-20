# Phase 152 C02f-AD — Real etcd Get/Txn Source Staging Audit

Status: `SOURCE_STAGING_COMPLETE / STATIC_READBACK_PASS / EXECUTABLE_VALIDATION_NOT_RUN / NO_ENDPOINT_CONTACT / NO_TLS_AUTH_RBAC / NO_RUNTIME_ACTIVATION / NO_PRODUCTION_ACTIVATION`

Date: 2026-08-20
Repository: `powercode365-dotcom/prw-executor-private`
Predecessor validated C02f-AC head: `0414e43db31231fbd6c639c8b207a9908236599a`
C02f-AD branch: `phase-152-c02f-ad-etcd-wiring-staging`
C02f-AD implementation source head: `19031baffc33c09c1dc808945b99f91512a1d58e`
Frozen C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Authorization and boundary

The user explicitly authorized `C02f-AD real etcd Get/Txn wiring staging` after C02f-AC obtained canonical full Rust validation PASS.

This tranche implements only C02f-Z source-order item 4: real `etcd-client` Get/Txn wiring against no production endpoint. It does not authorize or activate later gates.

Explicitly out of scope and not performed:

- no `Client::connect` invocation or endpoint selection;
- no production or disposable etcd endpoint contact;
- no TLS feature/configuration, trust roots, certificates, credentials or RBAC;
- no Watch, lease, TTL or clock-based currentness authority;
- no fence allocator or authority-attempt RNG materialization;
- no first-ever absent-key bootstrap;
- no recovery epoch/high-water selection or activation;
- no Agent/runtime/task ownership or background retry loop;
- no R1-R4 effect-boundary activation;
- no PR creation, merge, main-branch mutation or production deployment.

PR #46 remains the separate C02f-AC validation-only draft PR and is not reused or mutated for C02f-AD.

## Source commits

C02f-AD was created from exact validated C02f-AC head `0414e43db31231fbd6c639c8b207a9908236599a`.

Implementation commits:

1. `59b5d219df83ba7032400a0290711433b7119185`
   - `phase 152 c02f-ad: wire real etcd get and txn boundary`
   - adds `crates/prw-control-plane/src/reachability_live_owner_etcd.rs`.

2. `37ae80e5fe846c3932083ffa848898274ae1ab28`
   - `phase 152 c02f-ad: expose etcd authority store`
   - exports `reachability_live_owner_etcd` from `prw-control-plane`.

3. `19031baffc33c09c1dc808945b99f91512a1d58e`
   - `phase 152 c02f-ad: simplify error propagation`
   - source-equivalent Clippy-preventive cleanup only.

Static compare from `0414e43...` to implementation head `19031baf...` shows exactly two source paths changed:

- `crates/prw-control-plane/src/lib.rs`: one module export;
- `crates/prw-control-plane/src/reachability_live_owner_etcd.rs`: new real-etcd boundary.

No other repository path is changed by the implementation commits.

## Materialized real-etcd boundary

The new `ReachabilityLiveOwnerEtcdStore` owns an already-created `etcd_client::KvClient`.

Construction itself performs no network I/O. The module deliberately does not own endpoint bootstrap or call `Client::connect`.

### Linearizable exact-key Get

`linearizable_observation(peer)`:

1. creates the exact canonical C02f-AA key with `encode_live_owner_key(peer)`;
2. calls real `KvClient::get(key, None)`;
3. passes no serializable Get option, preserving etcd's default linearizable read semantics;
4. requires exact-key result cardinality of zero or one;
5. rejects a returned key different from the requested canonical key;
6. decodes a present KV through `LiveOwnerObservation::decode`, preserving canonical record/key binding and positive `mod_revision` validation.

`currentness(peer, fence)` performs this real linearizable read and delegates semantic provider classification to the already-validated C02f-AB `classify_currentness` function. Missing established state remains fail-closed.

### Real dual-CAS Txn

`execute(plan)` maps one already-validated C02f-AB `LiveOwnerTxnPlan` directly into a real `etcd_client::Txn`:

- compare 1: `Compare::mod_revision(key, Equal, observed_mod_revision)`;
- compare 2: `Compare::value(key, Equal, exact_observed_value)`;
- success branch: exactly one `TxnOp::put(key, canonical_successor_value, None)`;
- failure branch: exactly one `TxnOp::get(key, None)`;
- no lease is attached to the Put;
- no serializable option is attached to the Get.

A definitive successful Txn response must contain exactly one Put response before C02f-AB may classify the mutation as committed.

A definitive compare-failure response must contain exactly one Get response. That exact-key state is decoded as a `LiveOwnerObservation` and passed to C02f-AB `classify_definitive_mutation`; compare failure therefore never manufactures success.

### Indeterminate mutation rule

Any `KvClient::txn` RPC error is mapped to `ReachabilityLiveOwnerEtcdError::MutationIndeterminate`.

The transaction plan is passed by reference to `execute`, so the caller retains the exact intended plan/successor context needed for the C02f-Z mandatory re-observation path.

The module documentation explicitly requires re-observation before any retry and prohibits blind mutation retransmission. No retry loop is implemented in this tranche.

## etcd-client API verification

The repository already pins:

`etcd-client = { version = "=0.19.0", default-features = false }`

The C02f-AD implementation was statically checked against the documented `etcd-client 0.19.0` API surface, including:

- `KvClient::get`;
- `KvClient::txn`;
- `Compare::mod_revision`;
- `Compare::value`;
- `CompareOp::Equal`;
- `Txn::new().when(...).and_then(...).or_else(...)`;
- `TxnOp::put` and `TxnOp::get`;
- `TxnResponse::succeeded` and `TxnResponse::op_responses`;
- `TxnOpResponse::Put` and `TxnOpResponse::Get`;
- `GetResponse::kvs` / KV key, value and `mod_revision` accessors.

No project source was submitted to a public compiler or playground.

## Static contract checks

Static readback confirms:

- concrete real-etcd code remains in `prw-control-plane`;
- no `prw-control-plane -> prw-remote-bridge` dependency is introduced;
- exact `DeviceId + TransportIdentity` key derivation remains delegated to C02f-AA;
- no serializable safety read is requested;
- both selected CAS compares are materialized;
- success is one Put and failure is one Get;
- release delete behavior is not introduced;
- mutation RPC ambiguity is not converted into non-commit or success;
- no missing key is converted into first-ever authority;
- no endpoint/TLS/auth/RBAC/runtime/recovery behavior is introduced.

Static result: `PASS` for source shape and C02f-Z boundary conformance.

## Executable validation status

No C02f-AD executable validation is claimed by this audit.

The following have **not** yet been obtained for C02f-AD source head `19031baf...`:

- `cargo fmt --all -- --check` PASS;
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` PASS;
- `cargo test --locked --workspace --all-targets` PASS;
- `cargo build --locked --workspace --all-targets` PASS.

No C02f-AD PR has been created solely to trigger CI because PR creation is a separate GitHub mutation requiring separate explicit authorization.

Therefore the current gate is:

`C02F_AD_SOURCE_STAGING_COMPLETE -> C02F_AD_EXECUTABLE_VALIDATION_PENDING`

Later TLS/auth/RBAC/disposable integration and recovery/bootstrap work remain blocked behind their separate architectural and authorization gates.
