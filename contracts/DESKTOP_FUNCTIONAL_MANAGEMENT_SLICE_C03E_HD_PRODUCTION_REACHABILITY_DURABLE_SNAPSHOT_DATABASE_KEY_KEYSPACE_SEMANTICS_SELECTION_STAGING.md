# Phase 152 C03e-HD — Production Reachability Durable Snapshot Database Key / Keyspace Semantics Selection — STAGING

## 1. Purpose

C03e-HD selects only the provider-neutral canonical database **key** and keyspace semantics for the production reachability durable snapshot whose canonical `PRWS` v1.0 **value** representation was selected by C03e-HB and materialized by canonically closed C03e-HC.

HD does not select or materialize a concrete durable-store provider, provider revision/CAS syntax, transaction plan, etcd/SQL/embedded/filesystem product, schema/migrations, credentials/TLS/RBAC, leases/TTL/watch behavior, retry/reconciliation, runtime/task ownership, Agent startup activation, owner-map population, candidate handoff, traversal/listener/readiness/dialing/networking, deployment, restart, or merge.

## 2. Exact prerequisite

Canonical predecessor C03e-HC:

- branch: `phase-152-c03e-hc-production-reachability-durable-snapshot-value-record-codec-source-materialization-staging`
- exact head: `183bf3f3db561c7ba1b5c8dc5accea84bba500d5`
- exact tree: `e176f771481882736c9c11238d8e774eb4868eb2`
- gate: `C03E_HC_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_VALUE_RECORD_CODEC_SOURCE_MATERIALIZED`
- closure: `CLOSED_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_VALUE_RECORD_CODEC_SOURCE_MATERIALIZATION`

HC materialized only canonical value bytes and explicitly left database key/key prefix/keyspace, concrete provider, provider revision/CAS syntax, schema, credentials, and runtime activation separately gated.

## 3. Fresh exact-HC audit evidence

The exact-HC read-only audit establishes the next prerequisite without inferring provider selection.

### 3.1 Durable semantic store remains bridge-owned and provider-neutral

Exact HC `crates/prw-remote-bridge/src/reachability_owner.rs` blob:

`8de2e3d21224b339a7d18e926f5127838c903608`

The bridge-owned `ReachabilityDurableStore` exposes typed peer/snapshot/revision semantics. Its contract does not define database key bytes or choose a concrete database product. The owner continues to consume a semantic durable-store boundary rather than provider syntax.

Therefore database key construction must not be pushed into owner lifecycle logic or callers.

### 3.2 Existing reachability live-owner authority provides the key-codec precedent

Exact HC `crates/prw-control-plane/src/reachability_live_owner_codec.rs` blob:

`b03d4209770bf3f35fe0f5dccbeac15c5257449c`

That separate authority domain already uses a deterministic versioned binary key law:

- exact domain prefix `/prw/reachability/live-owner/`;
- `u16` major version followed by `u16` minor version;
- `u64` exact `DeviceId` UTF-8 byte length;
- exact `DeviceId` UTF-8 bytes;
- exact 32-byte `TransportIdentity`;
- unsigned multi-byte integers in network/big-endian order;
- strict decoding through existing typed constructors;
- exact peer binding and no delimiter-based identity parsing.

The shape is reusable as a repository convention. Its **domain prefix is not reusable** because live-owner authority and durable snapshot authority are distinct persistence domains.

### 3.3 Existing etcd adapter keeps provider I/O separate from key construction

Exact HC `crates/prw-control-plane/src/reachability_live_owner_etcd.rs` blob:

`a466481de12ad31f0b315928c7bca819ce3e6394`

The existing adapter performs real etcd operations only after canonical key construction and operates on an already-created client. It owns provider-specific transaction/revision behavior separately from the key representation.

This confirms the repository layering precedent:

`typed semantic state -> canonical key/value representation -> provider transaction/I/O`.

HD selects only the missing key/keyspace representation layer for durable snapshots.

### 3.4 Dependency ownership excludes provider materialization from HD

Exact HC manifests:

- `crates/prw-control-plane/Cargo.toml` blob `acf008393686c10f5b9d63605399a608737973f7` contains `etcd-client = =0.19.0`;
- `crates/prw-remote-bridge/Cargo.toml` blob `5fd48263be415aac28dee1c71a4031a4a02ad36c` has no etcd/SQL/serialization dependency;
- `crates/prw-agent/Cargo.toml` blob `4c70d6be9b56f39edc10810eefa3428314ed7559` has no direct etcd/SQL dependency;
- `crates/prw-reachability-custody/Cargo.toml` blob `f8ff3ecdac1e5cb0b580818ad6f55ae6076ff4f7` depends on `prw-control-plane` but has no direct etcd dependency.

No manifest change is required for a semantics-only key selection. Selecting or moving a concrete provider into another crate would be a different architectural checkpoint.

## 4. Selected authority domain

The production durable snapshot receives a dedicated, non-overlapping key domain:

`/prw/reachability/durable-snapshot/`

This prefix is exact ASCII bytes, including the final `/`.

It is distinct from the existing live-owner domain `/prw/reachability/live-owner/` and must never be aliased, normalized, shortened, or reused for another reachability authority class.

The key represents exactly one durable snapshot authority object for exactly one `PeerConnectivityIdentity`.

## 5. Selected key version

Initial durable-snapshot database-key version:

- major: `1` encoded as unsigned big-endian `u16`;
- minor: `0` encoded as unsigned big-endian `u16`.

A decoder for v1.0 accepts only exact major/minor `1/0`.

Unknown major or minor versions fail closed. No fallback parser, alternate legacy spelling, or best-effort normalization is selected.

The key version is independent of the `PRWS` value-record version. A future migration may evolve either side independently, but HD does not select migration mechanics.

## 6. Canonical key byte layout

For one peer `P`, the exact canonical database key is:

1. exact ASCII prefix bytes `/prw/reachability/durable-snapshot/`;
2. key major version: `u16` big-endian, value `1`;
3. key minor version: `u16` big-endian, value `0`;
4. exact `DeviceId` UTF-8 byte length: `u64` big-endian;
5. exact `DeviceId` UTF-8 bytes, with no terminator or delimiter escaping;
6. exact 32 bytes of `TransportIdentity`.

Therefore:

`key_len = prefix_len + 4 + 8 + device_id_utf8_len + 32`.

No value-derived field is part of the key.

## 7. Exact identity law

The key identity is exactly the existing typed `PeerConnectivityIdentity`:

- exact `DeviceId`;
- exact `TransportIdentity`.

The key must not contain or derive from:

- candidate IDs or endpoints;
- candidate-ID historical high-water;
- freshness lifecycle or token;
- reachability observations;
- selected path;
- provider revision;
- transaction/request/session ID;
- timestamp/clock value;
- fence sequence/live-owner record;
- hash of the peer identity;
- hostname, IP, process ID, UUID, or deployment instance.

Two distinct exact typed peers must encode to distinct canonical keys.

## 8. DeviceId law

The key uses the exact UTF-8 bytes exposed by the typed `DeviceId`.

The byte length is the UTF-8 byte count, not Unicode scalar count, grapheme count, or character count.

No Unicode normalization, case folding, slash escaping, percent encoding, NUL termination, delimiter parsing, or lossy conversion is selected.

The explicit `u64` byte length makes delimiter-like bytes in an otherwise valid typed `DeviceId` unambiguous.

On decode, the byte length must be representable in host bounds, the byte slice must be exact valid UTF-8, and reconstruction must pass through the existing `DeviceId` constructor. Invalid or unrepresentable input fails closed.

## 9. Transport identity law

The key ends with exactly the existing 32-byte transport identity.

Decode must reconstruct only through the existing `TransportIdentity` constructor/invariant boundary. Any invalid transport identity, including an all-zero identity if rejected by the existing type, fails closed.

No truncation, textual encoding, hashing, or alternate width is selected.

## 10. Strict decode law

A future canonical key decoder must reject at least:

- wrong prefix;
- unsupported major/minor version;
- truncated version or length fields;
- `DeviceId` length overflow or inconsistency;
- invalid UTF-8;
- invalid typed `DeviceId`;
- missing/truncated transport identity;
- invalid typed `TransportIdentity`;
- trailing bytes after the exact 32-byte transport identity.

Decode must consume the whole key exactly.

No prefix scan, fuzzy match, delimiter recovery, alternate version interpretation, or trailing-byte tolerance is selected.

## 11. Key/value peer-binding law

A concrete durable-store implementation must bind the database key and decoded durable value to the same exact peer.

For every successful load/CAS/commit path:

`decode(key) == decoded_snapshot.plan().peer() == decoded_snapshot.freshness().peer()`.

Any mismatch is a semantic corruption/binding failure and must fail closed before returning or installing authoritative owner state.

The adapter must not rewrite the key peer, rewrite the value peer, or choose one side as authoritative after mismatch.

This check is distinct from byte-canonicality checks and from `PeerConnectivityPlan::from_durable_state(...)` semantic restoration.

## 12. Exact-record access law

The selected key identifies one exact durable snapshot record. HD does not select prefix/range scans as an authoritative read path.

A future provider adapter should use exact-key access for one peer unless a separately reviewed checkpoint explicitly selects enumeration/recovery scanning.

The dedicated prefix reserves the durable-snapshot authority domain but does not itself authorize list/watch/range behavior.

## 13. Provider-neutrality

The canonical key is a byte string independent of any concrete provider.

HD does not select:

- etcd;
- SQL table/primary-key representation;
- Spanner;
- embedded KV;
- filesystem path mapping;
- object storage;
- provider-specific escaping;
- provider revision semantics.

If a future provider can accept arbitrary byte keys, it must use these exact bytes. If a future provider requires another native key type, an explicit separately reviewed mapping would be required; HD does not silently authorize one.

## 14. Separation from CAS / revision semantics

HD does not change the existing `ReachabilityDurableStore` semantic revision/CAS contract.

It does not select how a concrete database maps:

- absent record;
- expected durable revision;
- provider mod revision/version;
- create revision;
- transaction compare predicates;
- conflict results;
- indeterminate mutation results.

Those are transaction/provider semantics, not database key identity semantics.

## 15. Separation from value codec

C03e-HB/HC own the canonical `PRWS` v1.0 value bytes.

HD does not alter:

- `PRWS` magic/version;
- header layout;
- candidate encoding;
- freshness encoding;
- high-water encoding;
- value decode failure taxonomy.

The future key codec and existing value codec are separate pure functions that can be tested independently and then bound together by a future store adapter.

## 16. Source ownership selected for a bounded successor

The future provider-neutral canonical durable-snapshot key codec belongs alongside the bridge-owned durable snapshot value codec because:

- `ReachabilityDurableStore` is bridge-owned;
- the key represents the bridge-owned `PeerConnectivityIdentity` authority key;
- `PRWS` value representation is already bridge-owned;
- no concrete provider dependency is needed for pure key bytes.

A future source-materialization successor is expected to require only:

1. one bridge-owned durable-snapshot key-codec source module, or a bounded extension of the existing durable snapshot codec module if fresh-head audit proves that is cleaner;
2. one bridge root registration/export only if a new module is used;
3. one source-materialization contract.

No Cargo manifest or lockfile change is expected.

If fresh-HD-head audit requires a provider, Agent, custody, workflow, manifest, lockfile, or runtime path merely to materialize pure key bytes, stop and re-audit rather than widening scope automatically.

## 17. Future focused source tests

A source successor should cover at least:

- deterministic exact bytes for a known peer;
- exact v1.0 round-trip;
- delimiter-like and Unicode DeviceId bytes without ambiguity;
- distinct exact peers produce distinct keys;
- wrong prefix rejection;
- wrong major/minor rejection;
- truncation at every fixed field boundary;
- inconsistent/overflowing DeviceId length rejection;
- invalid UTF-8 / typed DeviceId rejection;
- invalid transport identity rejection;
- trailing-byte rejection;
- proof that no candidate/freshness/value field changes the key for the same exact peer;
- exact key/value peer-binding helper or adapter-boundary failure, if that binding is materialized in the same separately authorized successor scope.

## 18. Explicit exclusions

C03e-HD does not select or materialize:

- source code;
- database schema/table names/migrations;
- concrete durable-store implementation;
- etcd endpoint/client/TLS/RBAC;
- SQL/Spanner/embedded/filesystem provider;
- provider revision/CAS mapping;
- transaction planning;
- key enumeration/range scans;
- leases/TTL/watch;
- retry/reconciliation;
- credentials;
- owner-map population;
- Agent startup recovery activation;
- bootstrap/freshness issuance;
- candidate publication handoff;
- current-Mesh response activation;
- traversal/listener/readiness/dialing/networking;
- executor/task/cancellation ownership;
- deployment/restart;
- branch merge/deletion.

## 19. Exact diff ceiling

HD itself is docs-only and limited to exactly this one contract path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_HD_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_DATABASE_KEY_KEYSPACE_SEMANTICS_SELECTION_STAGING.md`

Any second changed path is a scope contradiction and blocks canonical closure pending re-audit.

No manifest, lockfile, source, workflow, provider, deployment, or runtime path is authorized in HD.

## 20. Closure requirements

Canonical HD closure requires all of:

1. exact predecessor/merge-base remains HC head `183bf3f3db561c7ba1b5c8dc5accea84bba500d5`;
2. branch is ahead exactly one commit and behind zero;
3. changed-file set is exactly the one authorized contract path;
4. no source/manifest/lockfile/workflow/provider/runtime path appears;
5. all automatically triggered exact-head CI is terminal with no failure/pending workflow;
6. immutable audit evidence is stored in the canonical project Drive folder and raw-read back byte-exact;
7. PR body may then move to canonical `Status: CLOSED` while GitHub PR remains draft/open/unmerged.

## 21. Target closure and gate

Target canonical closure:

`CLOSED_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_DATABASE_KEY_KEYSPACE_SEMANTICS_SELECTION`

Target canonical gate:

`C03E_HD_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_DATABASE_KEY_KEYSPACE_SEMANTICS_SELECTED`

Until all closure requirements pass, HD remains STAGING.

## 22. Safe successor

After canonical HD closure, begin with a fresh exact-HD-head read-only audit.

The likely bounded successor is provider-neutral source materialization of the selected durable-snapshot key codec, but HD does not pre-authorize its final path count or API shape.

A concrete provider, provider CAS/revision mapping, schema, credentials, or runtime activation remains a separate prerequisite and must not be bundled by inference.
