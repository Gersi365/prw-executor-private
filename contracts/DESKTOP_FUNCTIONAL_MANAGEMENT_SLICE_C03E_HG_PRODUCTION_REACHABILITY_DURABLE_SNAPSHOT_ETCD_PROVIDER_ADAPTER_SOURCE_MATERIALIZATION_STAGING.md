# Private Remote Workspace — Phase 152 C03e-HG Production Reachability Durable Snapshot etcd Provider Adapter Source Materialization

Status: `STAGING / SOURCE_MATERIALIZATION_ONLY / NO_RUNTIME_ACTIVATION`

Gate target:
`C03E_HG_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_ETCD_PROVIDER_ADAPTER_SOURCE_MATERIALIZED`

Canonical predecessor: C03e-HF.

Exact predecessor head:

```text
6c9f7cffb180f8441a6081e719c48de4fc8275ca
```

Exact predecessor tree:

```text
05e2ded6892a0e2cf31e263b39ea6535e2529546
```

This checkpoint materializes only the C03e-HF-selected two-layer source seam: a control-plane-owned raw exact-key etcd executor and a bridge-owned semantic implementation of the existing `ReachabilityDurableStore` trait. It does not connect to etcd, configure endpoints/TLS/auth/RBAC, load credentials, create schema, activate Agent recovery, populate owner maps, start tasks, modify networking, deploy, restart, merge, change repository visibility, or rewrite history.

## 1. Fresh exact-HF audit basis

HF remains canonically closed at exact head `6c9f7cffb180f8441a6081e719c48de4fc8275ca`.

Fresh source audit reconfirms:

- `crates/prw-control-plane/Cargo.toml` already owns `etcd-client = 0.19.0` with `default-features = false` and feature `tls`;
- `crates/prw-remote-bridge/Cargo.toml` already depends on `prw-control-plane` and does not directly depend on `etcd-client`;
- bridge-owned canonical durable value codec already exists;
- bridge-owned canonical durable key codec already exists;
- bridge-owned `ReachabilityDurableStore` trait and `ReachabilityPersistenceCommit/Error` semantics already exist;
- control-plane already contains a validated real-etcd precedent using default-linearizable exact-key Get, exact `mod_revision` + value CAS, Put success branch, linearizable Get failure branch, and indeterminate RPC failure classification.

No Cargo manifest or lockfile change is required under the exact HF topology.

## 2. Visibility correction provenance

The original immutable HF closure audit incorrectly blocked HG because current GitHub metadata reports `visibility: public` while earlier 2026-08-16 synchronization records described the repository as private.

Corrective evidence establishes that public visibility was known project state no later than 2026-08-22:

- Drive `PRW_PUBLIC_REPO_CLEANUP_ARCHIVE_2026-08-22` explicitly records `Visibility: public` and conservative cleanup while the repository remained public;
- Drive `C02F_BX_PROVIDER_CLIENT_BOOTSTRAP_SOURCE_MATERIALIZATION_VALIDATION_AUDIT` explicitly records repository ID `1334911207` as `public at validation time` and closes a source-materialization gate `PASS`;
- current GitHub metadata remains public, consistent with that known state.

Corrective audit:
`C03E_HF_REPOSITORY_VISIBILITY_GATE_CORRECTION_AUDIT_2026-08-31.md`

Drive ID:
`1_hWKmVNc6-sRTcTtsAumh9nUcfkMbnVo`

This correction authorizes no visibility mutation, history rewrite, credential publication, secret/private-key material, or deployment. HG source remains free of concrete endpoint and secret bytes.

## 3. Exact authorized path ceiling

HG may change exactly these five paths and no others:

1. `crates/prw-control-plane/src/reachability_durable_snapshot_etcd.rs`
2. `crates/prw-control-plane/src/lib.rs`
3. `crates/prw-remote-bridge/src/reachability_durable_snapshot_etcd_store.rs`
4. `crates/prw-remote-bridge/src/root.rs`
5. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_HG_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_ETCD_PROVIDER_ADAPTER_SOURCE_MATERIALIZATION_STAGING.md`

Any sixth path is a stop-and-re-audit condition.

Any Cargo manifest, lockfile, workflow, Agent composition, runtime, packaging, networking, deployment, schema, credential, or repository-visibility path is explicitly unauthorized.

## 4. Control-plane raw provider module

`crates/prw-control-plane/src/reachability_durable_snapshot_etcd.rs` owns only opaque exact-key etcd execution.

Public boundary:

- `ReachabilityDurableSnapshotEtcdExecutor`
- `ReachabilityDurableSnapshotEtcdObservation`
- `ReachabilityDurableSnapshotEtcdMutation`
- `ReachabilityDurableSnapshotEtcdError`

Construction accepts an already-created `etcd_client::KvClient` and performs no endpoint/client bootstrap.

### 4.1 Linearizable Get

`linearizable_get(key)` performs exactly one etcd Get using default options.

Selected validation:

- zero returned KV pairs -> `Ok(None)`;
- exactly one KV pair -> require exact key equality and positive `mod_revision`;
- more than one KV pair -> fail closed;
- provider RPC error -> fail closed.

The raw module does not decode the PRW key or `PRWS` value.

### 4.2 Dual-CAS mutation

`compare_and_put(key, observed_mod_revision, observed_value, replacement_value)` constructs exactly:

```text
mod_revision(key) == observed_mod_revision
AND
value(key) == exact observed_value
```

Success branch:

```text
Put(key, replacement_value)
```

Failure branch:

```text
default-linearizable Get(key)
```

The observed revision must be positive before provider I/O.

Definitive success returns `Committed` only when the response shape contains exactly the selected Put operation response.

Definitive compare failure returns `CompareFailed(...)` only when the response shape contains exactly the selected Get operation response. The optional observation may be absent.

Txn RPC failure is indeterminate and returns an error. No automatic retry occurs.

## 5. Bridge semantic durable store

`crates/prw-remote-bridge/src/reachability_durable_snapshot_etcd_store.rs` implements the existing `ReachabilityDurableStore` trait over `ReachabilityDurableSnapshotEtcdExecutor`.

The bridge owns all PRW semantic interpretation.

### 5.1 Exact binding law

Every accepted present observation must satisfy:

```text
decode(key)
  == requested_peer
  == decoded_snapshot.plan().peer()
  == decoded_snapshot.freshness().peer()
```

Any malformed key/value or mismatch maps to:

```text
ReachabilityPersistenceError::UnavailableOrAmbiguous
```

No repair, normalization, fallback parsing, alternate key spelling, or peer substitution is permitted.

### 5.2 `load_current`

The semantic load path:

1. encodes the exact canonical key for `peer`;
2. calls one raw default-linearizable exact-key Get;
3. maps provider absence to `Ok(None)`;
4. decodes and validates present key/value bytes;
5. enforces exact requested-peer/key/value binding;
6. returns the typed `ReachabilityDurableSnapshot` only after full validation.

Provider unavailability or malformed/ambiguous state maps fail closed.

### 5.3 `compare_and_commit`

The semantic CAS path:

1. takes the exact replacement peer from the typed replacement snapshot;
2. encodes canonical replacement key/value bytes before provider mutation;
3. performs one raw default-linearizable exact-key Get;
4. maps absence to definite `StaleExpected` and performs no Put;
5. validates/binds a present observation;
6. compares the decoded current PRW freshness token to `expected_current`;
7. maps non-matching/no-current-token state to definite `StaleExpected` without a Put;
8. only when expected PRW freshness is current, submits exact observed raw bytes + positive observed `mod_revision` + canonical replacement value to the raw executor;
9. maps definitive Put success to `Committed`;
10. maps a definitive absent/different-freshness failure observation to `StaleExpected`;
11. maps malformed/ambiguous failure observations fail closed.

Provider revision is transaction evidence only and never becomes PRW freshness state.

## 6. Same-token compare-failure invariant

HF closure requires a focused test for failed CAS followed by a failure-read that still carries the same exact `expected_current` PRW freshness token while durable bytes differ.

Such a result is not normal stale currentness.

Required mapping:

```text
ReachabilityPersistenceError::UnavailableOrAmbiguous
```

Required behavior:

- fail closed;
- do not report normal `StaleExpected`;
- do not install the failure-read snapshot as authority;
- do not blindly retransmit the transaction;
- require the owner/recovery path to re-observe authority.

HG implementation treats any definitive failure observation still carrying the exact expected current token as ambiguous rather than claiming stale semantic authority. This is intentionally fail-closed and includes the required same-token/different-bytes case.

## 7. Absence and creation boundary

HG materializes no create-if-absent protocol.

No `version(key) == 0` transaction is added.

`load_current` may return `None`.

`compare_and_commit` sees absent current state as definite non-commit and returns `StaleExpected`.

Bootstrap/new-lifecycle durable creation remains separately gated.

## 8. Retry/reconciliation/watch boundary

HG adds no:

- retry loop;
- backoff policy;
- reconciliation worker;
- queue;
- prefix/range scan;
- Watch;
- lease;
- TTL;
- compaction/GC policy;
- periodic task;
- executor/runtime ownership.

All provider operations remain bounded to one exact key per call.

## 9. Client/security boundary

The raw control-plane executor receives an already-created `KvClient`.

HG does not materialize:

- `Client::connect`;
- endpoint discovery/configuration;
- TLS roots/client certificates;
- usernames/passwords/tokens;
- RBAC;
- secret loading/storage/rotation;
- cluster provisioning;
- firewall/routing changes.

No concrete endpoint or secret/private-key byte is introduced.

## 10. Registration-only existing-file changes

`crates/prw-control-plane/src/lib.rs` changes only by registering:

```rust
pub mod reachability_durable_snapshot_etcd;
```

`crates/prw-remote-bridge/src/root.rs` changes only by registering:

```rust
pub mod reachability_durable_snapshot_etcd_store;
```

No unrelated root/module reorganization is authorized.

## 11. Focused tests

Control-plane module-local tests cover at minimum:

- selected dual-CAS transaction materialization without contacting an endpoint;
- rejection of non-positive observed provider revisions before provider I/O.

Bridge module-local tests cover at minimum:

- exact peer/key/value round-trip binding;
- requested-peer mismatch fail-closed behavior;
- same-token/different-bytes compare-failure ambiguity;
- different-current-token compare failure mapping to `StaleExpected`.

No focused test opens a socket or connects to etcd.

## 12. Validation requirements

The exact final HG head must pass all automatically triggered permanent validation relevant to the changed paths.

At minimum:

- locked dependency graph;
- rustfmt;
- Clippy with `-D warnings`;
- workspace/all-target tests;
- workspace/all-target build;
- focused new module tests as part of workspace validation.

Root `Cargo.lock` and `apps/android/native/Cargo.lock` must remain byte-identical to HF because HG changes no manifest/dependency.

Path-filtered disposable-etcd workflows may be skipped. A skipped workflow is recorded as skipped and is not claimed as PASS.

No real etcd validation or production infrastructure is required or authorized by HG.

## 13. Explicit exclusions

HG does not select/materialize/activate:

- another persistence trait/model;
- another durable key/value representation;
- schema/table/migration behavior;
- create-if-absent bootstrap;
- endpoint/client connection bootstrap;
- credentials/TLS/auth/RBAC;
- Watch/lease/TTL/scan/retry/reconciliation;
- Agent owner-map population;
- startup durable recovery orchestration;
- bootstrap freshness issuance;
- candidate publication/current-Mesh response activation;
- traversal/listener/readiness/dialing/network activation;
- Android/desktop runtime activation;
- systemd mutation;
- deployment/restart;
- merge/branch deletion;
- history rewrite/force-push;
- repository-visibility mutation.

## 14. Closure condition

HG may close only after exact-final-head evidence proves:

- exact HF parent/merge-base lineage;
- no path outside the exact five-path ceiling;
- no manifest/lockfile/workflow/runtime/deployment/visibility mutation;
- focused semantics remain consistent with HF;
- exact-head permanent CI has no failing or pending automatically triggered validation;
- immutable Drive audit evidence records final head/tree/path/blob/CI/lockfile results.

Target closure:

```text
CLOSED_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_ETCD_PROVIDER_ADAPTER_SOURCE_MATERIALIZATION
```

Until those conditions are met, this file remains staging evidence only.
