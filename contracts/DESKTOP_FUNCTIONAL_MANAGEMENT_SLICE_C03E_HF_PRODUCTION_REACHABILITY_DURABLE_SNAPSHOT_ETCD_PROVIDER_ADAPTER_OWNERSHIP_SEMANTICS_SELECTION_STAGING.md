# Private Remote Workspace — Phase 152 C03e-HF Production Reachability Durable Snapshot etcd Provider Adapter Ownership Semantics Selection

Status: `STAGING / SEMANTICS_SELECTION_ONLY / NO_SOURCE_MATERIALIZATION / NO_RUNTIME_ACTIVATION`

Gate target: `C03E_HF_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_ETCD_PROVIDER_ADAPTER_OWNERSHIP_SEMANTICS_SELECTED`

Canonical predecessor: C03e-HE.

Exact predecessor head:

```text
09f1a14da5614402e8fd89361799297ea4294d22
```

This checkpoint selects only the ownership and transaction semantics for a future concrete etcd-backed implementation of the already-existing provider-neutral `ReachabilityDurableStore` boundary. It does not materialize Rust source, change a Cargo manifest or lockfile, connect to etcd, configure endpoints/TLS/RBAC, create schema, activate recovery, start a task/runtime, mutate networking, deploy, restart, merge, or modify production service state.

## 1. Exact C03e-HE audit basis

C03e-HE leaves the following source topology authoritative:

- bridge production reachability owner: `crates/prw-remote-bridge/src/reachability_owner.rs`, exact blob `8de2e3d21224b339a7d18e926f5127838c903608`;
- bridge durable snapshot value codec: `crates/prw-remote-bridge/src/reachability_durable_snapshot_codec.rs`, exact blob `3c66fcfa35c1104f5762c1431ea6200eb9daaf4b`;
- bridge durable snapshot key codec: `crates/prw-remote-bridge/src/reachability_durable_snapshot_key_codec.rs`, exact blob `12b65ccefd089505266658086af70092945d8f7f`;
- bridge manifest: `crates/prw-remote-bridge/Cargo.toml`, exact blob `5fd48263be415aac28dee1c71a4031a4a02ad36c`;
- control-plane manifest: `crates/prw-control-plane/Cargo.toml`, exact blob `acf008393686c10f5b9d63605399a608737973f7`;
- existing live-owner real-etcd precedent: `crates/prw-control-plane/src/reachability_live_owner_etcd.rs`, exact blob `a466481de12ad31f0b315928c7bca819ce3e6394`;
- existing live-owner codec precedent: `crates/prw-control-plane/src/reachability_live_owner_codec.rs`, exact blob `b03d4209770bf3f35fe0f5dccbeac15c5257449c`;
- existing live-owner deterministic transaction precedent: `crates/prw-control-plane/src/reachability_live_owner_txn.rs`, exact blob `9f0daccd3e0c066a63f0d40fa940368ca24c84f6`.

The exact HE dependency topology is significant:

```text
prw-remote-bridge -> prw-control-plane
prw-control-plane -> etcd-client = 0.19.0
```

`prw-remote-bridge` does not directly depend on `etcd-client`, while `prw-control-plane` already owns the concrete etcd client dependency and existing exact-key linearizable Get / Txn wiring precedent.

C03e-HF therefore must not invert this dependency direction and must not introduce a `prw-control-plane -> prw-remote-bridge` edge.

## 2. Existing semantic persistence boundary remains authoritative

The bridge-owned trait remains the production semantic contract:

```text
ReachabilityDurableStore
  load_current(peer)
  compare_and_commit(expected_current, replacement)
```

The durable semantic record remains:

```text
ReachabilityDurableSnapshot
  PeerConnectivityPlanDurableState
  CandidatePublicationFreshnessRecord
```

The existing owner semantics remain unchanged:

- `load_current` is an authoritative current-record read, not new-lifecycle authorization;
- recovery treats durable absence according to the already-existing owner law rather than silently creating state;
- `compare_and_commit` is linearizable for one exact peer lifecycle;
- `ReachabilityPersistenceCommit::Committed` means the replacement is definitely durable current;
- `ReachabilityPersistenceCommit::StaleExpected` means definite non-commit because the expected current state was not current;
- any unavailable or indeterminate provider outcome fails closed through the existing persistence-error boundary and requires authoritative recovery/re-observation.

C03e-HF does not change these types or widen their public API.

## 3. Selected concrete provider family

For this bounded adapter sequence, the selected concrete provider family is the repository's already-pinned etcd client boundary:

```text
etcd-client = 0.19.0
```

This selection reuses the existing control-plane dependency and live-owner provider precedent. It is not authorization to choose endpoints, create a client connection, configure TLS/auth/RBAC, provision an etcd cluster, create credentials, deploy infrastructure, or activate production persistence.

No SQL, Spanner, filesystem, embedded database, object store, or second durable-store provider is selected by this checkpoint.

## 4. Selected two-layer ownership model

C03e-HF selects exactly two ownership layers.

### 4.1 Bridge-owned semantic adapter

`prw-remote-bridge` owns the future concrete type that implements the bridge-local `ReachabilityDurableStore` trait.

That semantic adapter owns all PRW-specific meaning:

- canonical durable snapshot key encoding and decoding;
- canonical `PRWS` value encoding and decoding;
- exact requested-peer validation;
- exact key/value peer-binding validation;
- freshness-token comparison;
- mapping definite etcd compare failure to the existing `StaleExpected` semantic result;
- mapping provider/codec/shape/indeterminate failures into the existing fail-closed persistence error boundary;
- preserving the existing owner recovery semantics.

The bridge adapter does not own raw etcd protocol construction, endpoint selection, credentials, connection establishment, runtime/task ownership, retries, watches, leases, TTLs, scans, or deployment.

### 4.2 Control-plane-owned raw etcd executor

`prw-control-plane` owns the future provider-specific executor built around an already-created `etcd_client::KvClient`.

That executor operates on opaque exact key/value bytes plus etcd revision evidence. It does not parse PRW `PRWS` values, does not construct `PeerConnectivityIdentity`, does not understand publication freshness semantics, and does not implement the bridge-owned `ReachabilityDurableStore` trait.

The executor owns only:

- default-linearizable exact-key Get;
- exact-key response cardinality/key-shape validation;
- exact observed `mod_revision` carriage;
- one bounded dual-CAS Txn against the exact observed key/value;
- exact Put of replacement bytes on Txn success;
- one default-linearizable exact-key Get in the compare-failure branch;
- provider response-shape validation;
- explicit indeterminate RPC failure classification.

Construction accepts an already-created `KvClient`. It performs no endpoint selection or `Client::connect` operation.

## 5. Canonical key/value binding law

The exact C03e-HD/HE key law remains:

```text
/prw/reachability/durable-snapshot/
+ u16-be major = 1
+ u16-be minor = 0
+ u64-be DeviceId UTF-8 byte length
+ exact DeviceId UTF-8 bytes
+ exact 32-byte TransportIdentity
```

The exact C03e-HB/HC value law remains canonical `PRWS` v1.0.

Every successful semantic read or mutation preparation must enforce:

```text
decode(key)
  == requested_peer
  == decoded_snapshot.plan().peer()
  == decoded_snapshot.freshness().peer()
```

For `compare_and_commit`, the replacement snapshot must also satisfy the same exact-peer binding before provider mutation is attempted.

No peer normalization, fallback parsing, alternate key spelling, prefix search, repair, substitution, or value-side identity override is permitted.

A key/value mismatch is fail-closed corruption/authority ambiguity, never a successful load and never a stale-success shortcut.

## 6. Selected linearizable load semantics

A future bridge semantic adapter `load_current(peer)` performs the following bounded transaction-free path:

1. encode the exact canonical key from `peer` using the HE key codec;
2. request one default-linearizable exact-key Get from the control-plane etcd executor;
3. accept provider cardinality zero or one only;
4. zero records maps to `Ok(None)` at the `ReachabilityDurableStore` boundary;
5. one record must carry the exact requested key;
6. decode the returned key canonically and require exact equality with `peer`;
7. decode the returned `PRWS` value canonically;
8. require exact key/value/requested-peer binding;
9. return the typed `ReachabilityDurableSnapshot` only after all validation succeeds.

Provider unavailability, malformed/non-canonical bytes, impossible cardinality, wrong key, or peer mismatch fails closed through the existing persistence error boundary. None of those conditions creates new lifecycle authority.

No serializable Get is selected.

## 7. Selected compare-and-commit preparation semantics

The existing semantic method accepts:

```text
expected_current: CandidatePublicationFreshnessToken
replacement: &ReachabilityDurableSnapshot
```

The future bridge adapter prepares a mutation as follows:

1. validate and canonically encode the replacement key and value;
2. perform one default-linearizable exact-key Get for that key;
3. absence means the caller's expected current token cannot be current and therefore produces a definite `StaleExpected` non-commit without creating a record;
4. for a present record, validate exact key and decode/validate the exact current `PRWS` value;
5. enforce exact key/current-value/replacement peer binding;
6. compare the decoded current freshness token/lifecycle against `expected_current` under the already-existing freshness semantics;
7. if the exact expected freshness is not current, return definite `StaleExpected` without issuing a Put Txn;
8. if it is current, retain the exact observed raw value bytes and exact positive provider `mod_revision` solely as Txn compare evidence;
9. pass those opaque bytes/revision plus the canonical replacement bytes to the control-plane etcd executor.

The adapter must never infer currentness from revision alone. PRW freshness remains the semantic expected-current authority; etcd revision is only provider-local race-closing evidence between the linearizable observation and the subsequent Txn.

## 8. Selected etcd dual-CAS transaction shape

The future raw etcd executor uses one exact-key Txn with both compares:

```text
mod_revision(exact_key) == observed_mod_revision
AND
value(exact_key) == exact_observed_raw_value
```

Success branch:

```text
Put(exact_key, canonical_replacement_value)
```

Failure branch:

```text
default-linearizable Get(exact_key)
```

There is no Delete success branch. Retirement remains a canonical replacement snapshot/tombstone according to existing reachability freshness semantics.

No transaction may use prefix/range compares, lease compares, clock values, request IDs, candidate IDs, transport observations, or a provider revision as PRW freshness authority.

## 9. Definitive versus indeterminate result law

### 9.1 Txn success

A structurally valid Txn success response with exactly the selected Put operation maps to:

```text
ReachabilityPersistenceCommit::Committed
```

### 9.2 Txn compare failure

A structurally valid Txn response with `succeeded == false` proves the Put branch did not commit.

The selected failure branch must contain exactly one exact-key Get response. The bridge adapter validates any returned record with the same canonical key/value peer-binding law.

A valid compare-failure response maps to:

```text
ReachabilityPersistenceCommit::StaleExpected
```

whether the failure observation is missing or contains a newer valid exact-peer record. The result is a definite non-commit and the production owner already treats stale durable currentness as requiring recovery/reload before further mutation.

Malformed failure-branch response shape, wrong key, invalid bytes, or peer mismatch fails closed as persistence unavailability/ambiguity rather than being accepted as a valid current record.

### 9.3 RPC/transport error

If the Txn RPC returns no definitive etcd Txn response, commit status is indeterminate.

It maps only to the existing fail-closed provider error classification:

```text
ReachabilityPersistenceError::UnavailableOrAmbiguous
```

The caller must re-observe authoritative durable state before any subsequent mutation decision. Blind retransmission of the same Put/TXN is not selected.

## 10. Provider revision semantics

`mod_revision` is provider-local transactional evidence only.

It is not:

- a PRW freshness token;
- a fence sequence;
- a candidate ID;
- a transport identity generation;
- a replay nonce;
- a user-visible revision;
- part of the canonical `PRWS` value;
- part of the canonical durable-snapshot key.

The revision may be retained only for the bounded read-to-Txn interval required to construct the exact dual-CAS operation. It is not restored into the reachability owner after recovery.

## 11. Absence and creation boundary

C03e-HF explicitly does not select an absent-record creation protocol.

Consequences:

- `load_current` may return `None` exactly as the existing trait permits;
- owner recovery continues to interpret absence through its existing fail-closed lifecycle law;
- `compare_and_commit` does not create a missing record merely because a caller supplied `expected_current`;
- bootstrap/new-lifecycle creation remains separately gated and must not be smuggled through the etcd adapter.

No `version(key) == 0` create-if-absent transaction is selected here.

## 12. Retry and reconciliation boundary

No automatic mutation retry is selected.

A future caller may perform a fresh authoritative `load_current` after a definite stale result or an indeterminate provider failure according to existing owner recovery semantics. That is a new observation/decision, not blind retransmission.

C03e-HF does not select a retry loop, backoff policy, reconciliation worker, watch stream, queue, background task, or periodic scanner.

## 13. Client construction and security boundary

The future control-plane executor accepts an already-created `KvClient` and performs no connection bootstrap.

Separately gated and not selected here:

- endpoint discovery or configuration;
- `Client::connect` ownership;
- TLS roots/client certificates;
- username/password or token auth;
- RBAC policy;
- credential storage/loading;
- secret rotation;
- health checking;
- cluster provisioning;
- network routing/firewall changes.

No private key or credential material is introduced by this checkpoint.

## 14. Keyspace operation boundary

The dedicated durable-snapshot prefix identifies an exact-record authority domain only.

C03e-HF selects no:

- prefix scan;
- range query;
- enumeration;
- Watch;
- lease;
- TTL;
- compaction policy;
- historical replay;
- garbage collection;
- secondary index;
- multi-peer transaction.

All selected provider operations are one exact key at a time.

## 15. Dependency and package boundary

A source-materialization successor must preserve the existing dependency direction.

Selected law:

```text
prw-remote-bridge
  owns PRW semantic store adapter
  uses existing bridge key/value codecs
  consumes a control-plane-owned raw etcd executor

prw-control-plane
  owns etcd-client provider executor
  owns no bridge semantic types
```

Therefore the successor must not:

- add `etcd-client` to `prw-remote-bridge`;
- add `prw-remote-bridge` to `prw-control-plane`;
- move the bridge-owned `ReachabilityDurableStore` trait into control-plane;
- duplicate the `PRWS` or durable-key codecs in control-plane;
- create a second persistence trait/model.

Under the exact HE topology, no dependency or lockfile change is expected for this adapter materialization.

## 16. Expected source-materialization ceiling

A future source-materialization checkpoint must begin with a fresh exact-HF-head audit. If repository topology remains unchanged, the expected maximum source scope is:

1. one new control-plane provider module for opaque exact-key etcd Get/dual-CAS execution;
2. `crates/prw-control-plane/src/lib.rs` only for module/export registration required by that provider seam;
3. one new bridge module implementing the existing `ReachabilityDurableStore` semantic adapter over the control-plane executor;
4. `crates/prw-remote-bridge/src/root.rs` only for module/export registration required by that bridge seam;
5. one source-materialization contract.

Focused tests should live inside those new modules when practical so that a separate test path is not required.

Any Cargo manifest, lockfile, workflow, Agent composition, owner-map, startup, credential, runtime, networking, deployment, schema, scan/watch, or sixth path is a stop-and-re-audit condition rather than authority for silent expansion.

The ceiling is an audit expectation, not automatic authorization if fresh topology contradicts it.

## 17. Validation expectations for this selection checkpoint

Because C03e-HF is documentation-only, the expected repository validation remains the automatically triggered permanent Rust validation for the exact HF head.

No Android PASS is claimed unless the exact HF head actually triggers and completes Android validation.

No disposable-etcd workflow result is required merely because this checkpoint selects semantics; provider source and real disposable-provider behavior remain later source/validation work.

The source successor, if authorized by fresh audit, must validate at minimum:

- locked dependency graph;
- rustfmt;
- Clippy with `-D warnings`;
- workspace/all-target tests;
- workspace/all-target build;
- focused provider/adapter tests;
- unchanged lockfiles if no dependency mutation is required.

A later real-etcd validation checkpoint may use only disposable/non-production infrastructure unless separately authorized.

## 18. Explicit non-selection / non-activation

C03e-HF does not select or activate:

- a new persistence trait;
- a new durable semantic record;
- a different key or value format;
- an absent-record bootstrap/create path;
- a schema/migration system;
- SQL/Spanner/filesystem/object-store persistence;
- endpoints or connection bootstrap;
- TLS/auth/RBAC/credentials;
- retries, reconciliation workers, scans, Watch, lease, TTL, compaction, or GC;
- owner-map population;
- Agent startup recovery orchestration;
- candidate publication/current-Mesh response activation;
- traversal/listener/readiness/dialing/network activation;
- Android/desktop runtime activation;
- systemd mutation;
- deployment or restart;
- merge, branch deletion, history rewrite, or repository-visibility mutation.

## 19. Closure condition

C03e-HF may close only after the exact final HF head proves all of the following:

- exact HE parent/merge-base lineage;
- exactly one documentation path changed;
- no source/manifest/lockfile/workflow/runtime/deployment mutation;
- exact-head permanent CI has no failing or pending automatically triggered validation;
- immutable project evidence records the exact head/tree/path/blob and CI result.

Target canonical closure:

```text
CLOSED_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_ETCD_PROVIDER_ADAPTER_OWNERSHIP_SEMANTICS_SELECTION
```

Until those closure conditions are satisfied, this file remains staging evidence only.
