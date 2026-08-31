# Phase 152 C03e-HE — Production Reachability Durable Snapshot Database Key Codec Source Materialization — STAGING

## 1. Purpose

C03e-HE materializes only the provider-neutral canonical durable-snapshot database-key codec selected by canonically closed C03e-HD.

The checkpoint adds pure typed peer -> canonical key bytes -> typed peer source behavior and crate-root registration. It does not select or materialize a concrete durable-store provider, provider revision/CAS mapping, transaction plan, schema/migration, credentials/TLS/RBAC, retry/reconciliation, runtime/task ownership, Agent startup activation, owner-map population, candidate handoff, networking, deployment, restart, or merge.

## 2. Exact predecessor

Canonical predecessor C03e-HD:

- PR: #334
- branch: `phase-152-c03e-hd-production-reachability-durable-snapshot-database-key-keyspace-semantics-selection-staging`
- exact head: `1fd1358c11aab3c2a72ce221752f4c217cb49e28`
- exact tree: `1dc75f206219e60fee1b74cd59019039b245dd33`
- gate: `C03E_HD_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_DATABASE_KEY_KEYSPACE_SEMANTICS_SELECTED`
- closure: `CLOSED_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_DATABASE_KEY_KEYSPACE_SEMANTICS_SELECTION`

HD selected the dedicated exact authority-domain prefix `/prw/reachability/durable-snapshot/`, independent key version 1.0, exact typed peer identity law, strict fail-closed whole-key decoding, and future key/value peer binding while explicitly leaving source materialization and provider/CAS/schema/runtime behavior separately gated.

## 3. Fresh exact-HD audit

### 3.1 HE branch started byte-identical to exact HD

Fresh branch readback before mutation showed the HE staging branch at exact commit `1fd1358c11aab3c2a72ce221752f4c217cb49e28`, with tree `1dc75f206219e60fee1b74cd59019039b245dd33`.

No pre-existing HE source commit was present.

### 3.2 Existing live-owner codec establishes the pure binary-key precedent

Exact HD `crates/prw-control-plane/src/reachability_live_owner_codec.rs` blob:

`b03d4209770bf3f35fe0f5dccbeac15c5257449c`

Its key codec is pure/runtime-independent and already demonstrates the repository pattern required here:

- exact binary domain prefix;
- independent `u16` major/minor key version;
- `u64` DeviceId UTF-8 byte length;
- exact DeviceId UTF-8 bytes;
- exact 32-byte TransportIdentity;
- big-endian unsigned integer encoding;
- checked allocation/length arithmetic;
- strict total-length equality;
- reconstruction through existing typed constructors;
- no delimiter parsing or identity normalization;
- no provider I/O in the codec.

HE reuses that repository convention only as a structural precedent. It does not reuse the live-owner authority prefix or authority-record semantics.

### 3.3 Durable value codec remains a distinct representation domain

Exact HD `crates/prw-remote-bridge/src/reachability_durable_snapshot_codec.rs` remains the bridge-owned canonical `PRWS` value codec.

Its module-level contract explicitly scopes it to durable snapshot value representation and says it performs no database/key construction.

HD also selected key versioning as independent of `PRWS` value-record versioning. Therefore HE does not append key semantics into the value codec module. A separate key-codec module preserves the already-selected separation and makes future key/value binding an explicit store-adapter concern rather than a codec conflation.

### 3.4 Root registration is sufficient; no manifest change is required

Exact HD `crates/prw-remote-bridge/src/root.rs` blob:

`de87cbc222e1ec69d5e7b2e229d3544ce405d594`

The bridge root already registers the durable snapshot value codec as a public pure module. One adjacent module registration is sufficient for the new key codec.

The key codec requires only `std`, existing `prw-connectivity`, and existing `prw-core` dependencies already present in `prw-remote-bridge`. No Cargo manifest or lockfile change is required.

## 4. Materialized source API

HE adds:

`crates/prw-remote-bridge/src/reachability_durable_snapshot_key_codec.rs`

The module exports the exact selected constants:

- `REACHABILITY_DURABLE_SNAPSHOT_KEY_PREFIX = b"/prw/reachability/durable-snapshot/"`
- `REACHABILITY_DURABLE_SNAPSHOT_KEY_MAJOR = 1`
- `REACHABILITY_DURABLE_SNAPSHOT_KEY_MINOR = 0`
- `REACHABILITY_DURABLE_SNAPSHOT_KEY_TRANSPORT_BYTES = 32`

It exports pure functions:

- `encode_reachability_durable_snapshot_key(&PeerConnectivityIdentity) -> Result<Vec<u8>, ReachabilityDurableSnapshotKeyCodecError>`
- `decode_reachability_durable_snapshot_key(&[u8]) -> Result<PeerConnectivityIdentity, ReachabilityDurableSnapshotKeyCodecError>`

No provider handle, revision, transaction object, runtime context, credential, clock, random source, owner state, durable value, or database client enters either function signature.

## 5. Exact materialized key layout

For exact typed peer `P`, encoder output is:

1. exact ASCII prefix bytes `/prw/reachability/durable-snapshot/`;
2. big-endian `u16` key major = 1;
3. big-endian `u16` key minor = 0;
4. big-endian `u64` exact DeviceId UTF-8 byte length;
5. exact DeviceId UTF-8 bytes;
6. exact 32 bytes of TransportIdentity.

The encoder uses checked arithmetic for host allocation/capacity and fails closed on overflow.

No candidate, freshness, high-water, observation, selected path, value-record version, provider revision, transaction ID, request/session ID, timestamp, fence, hostname, process ID, UUID, or hash-derived identity contributes to the key.

## 6. Strict decode behavior

The decoder:

- requires the exact prefix;
- requires exact key version 1.0;
- reads the DeviceId byte length as big-endian `u64`;
- requires representability in host `usize`;
- computes exact total key length with checked arithmetic;
- requires total input length to match exactly;
- requires exact UTF-8 DeviceId bytes;
- reconstructs through existing `DeviceId::new`;
- reads exactly 32 transport bytes;
- reconstructs through existing `TransportIdentity::new`;
- consumes the whole key with no trailing-byte tolerance;
- returns exact typed `PeerConnectivityIdentity` only after all checks succeed.

It performs no fallback version parse, delimiter recovery, Unicode normalization, case folding, slash escaping, percent decoding, peer substitution, repair, or provider lookup.

## 7. Error boundary

HE materializes a key-specific fail-closed error taxonomy:

- `InvalidKeyPrefix`
- `UnsupportedKeyVersion`
- `InvalidKeyLength`
- `InvalidDeviceId`
- `InvalidTransportIdentity`
- `LengthOverflow`

The error boundary remains pure representation validation. It does not expose provider-specific status codes, revisions, transaction outcomes, retries, reconciliation states, or runtime failures.

## 8. Focused source tests

The new module includes focused tests covering:

1. exact known v1.0 byte layout and round-trip;
2. delimiter-like, Unicode, and NUL DeviceId bytes without normalization ambiguity;
3. distinct exact peers produce distinct keys;
4. wrong prefix rejection;
5. unsupported major rejection;
6. unsupported minor rejection;
7. inconsistent/overflowing DeviceId length rejection;
8. invalid UTF-8 rejection;
9. all-zero invalid TransportIdentity rejection through the typed constructor;
10. truncation rejection;
11. trailing-byte rejection;
12. truncation at selected fixed-field boundaries fails closed.

These tests exercise only pure canonical key behavior and do not instantiate a provider.

## 9. Key/value peer binding remains separately gated

HD selected the future durable-store adapter law:

`decode(key) == decoded_snapshot.plan().peer() == decoded_snapshot.freshness().peer()`.

HE does not prematurely materialize that adapter boundary because no concrete durable-store provider/transaction implementation is yet selected.

The key codec returns the exact decoded peer required for a future adapter to enforce the binding. The existing durable value codec independently returns the typed durable snapshot. A later separately reviewed store-adapter checkpoint must bind the two before authoritative owner installation.

## 10. Provider neutrality and dependency law

HE adds no dependency and changes no manifest or lockfile.

The module performs no:

- etcd call;
- SQL/Spanner call;
- embedded KV call;
- filesystem/object-store mapping;
- provider-specific escaping;
- transaction/CAS mapping;
- revision conversion;
- range/prefix scan;
- lease/TTL/watch;
- retry/reconciliation;
- credential/TLS/RBAC handling.

If a later provider cannot consume these exact canonical byte keys directly, any mapping to a provider-native key type requires a separately reviewed checkpoint. HE does not authorize silent transformation.

## 11. Source ownership

The module is bridge-owned because:

- `ReachabilityDurableStore` semantic authority is bridge-owned;
- the durable snapshot value codec is bridge-owned;
- the key represents the same exact `PeerConnectivityIdentity` authority object;
- the pure codec requires no control-plane provider dependency;
- keeping canonical key and canonical value representations in adjacent bridge modules preserves explicit layering before provider I/O.

This ownership decision does not move or duplicate the existing live-owner authority codec in `prw-control-plane`.

## 12. Exact authorized changed paths

HE authorizes exactly these three paths:

1. `crates/prw-remote-bridge/src/reachability_durable_snapshot_key_codec.rs`
2. `crates/prw-remote-bridge/src/root.rs`
3. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_HE_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_DATABASE_KEY_CODEC_SOURCE_MATERIALIZATION_STAGING.md`

Any fourth changed path is a scope contradiction and blocks canonical closure pending re-audit.

No Cargo manifest, lockfile, workflow, provider, custody, Agent runtime, deployment, or networking path is authorized.

## 13. Explicit exclusions

C03e-HE does not select or materialize:

- concrete `ReachabilityDurableStore` provider;
- etcd/SQL/Spanner/embedded/filesystem/object-store choice;
- provider revision/CAS/Txn semantics;
- absent-record/create semantics;
- schema/table/migration behavior;
- key enumeration/range scans;
- leases/TTL/watch;
- retry/reconciliation;
- credentials/TLS/RBAC;
- key/value peer-binding adapter implementation;
- owner-map population;
- Agent startup recovery activation;
- bootstrap/freshness issuance;
- candidate publication handoff;
- current-Mesh response activation;
- traversal/listener/readiness/dialing/networking;
- executor/task/cancellation ownership;
- deployment/restart;
- branch merge/deletion.

## 14. Closure requirements

Canonical HE closure requires all of:

1. exact predecessor/merge-base remains HD head `1fd1358c11aab3c2a72ce221752f4c217cb49e28`;
2. canonical HE branch is ahead exactly one atomic source-materialization commit and behind zero;
3. changed-file set is exactly the three authorized paths;
4. no manifest or lockfile mutation is present;
5. all automatically triggered exact-head CI is terminal with no failure/pending workflow;
6. immutable audit evidence is stored in the canonical project Drive folder and raw-read back byte-exact;
7. PR body may then move to canonical `Status: CLOSED` while GitHub PR remains draft/open/unmerged.

## 15. Target closure and gate

Target canonical closure:

`CLOSED_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_DATABASE_KEY_CODEC_SOURCE_MATERIALIZATION`

Target gate:

`C03E_HE_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_DATABASE_KEY_CODEC_SOURCE_MATERIALIZED`

Until those closure requirements are proven, this checkpoint remains `STAGING`.

## 16. Safe successor boundary

After canonical HE closure, the next action must begin with a fresh exact-HE-head read-only audit.

HE does not pre-authorize a provider. The audit should determine the next actual prerequisite among provider selection, provider revision/CAS mapping, key/value binding adapter semantics, credential/client construction, or another missing seam. No later checkpoint should be inferred solely from sequence naming.
