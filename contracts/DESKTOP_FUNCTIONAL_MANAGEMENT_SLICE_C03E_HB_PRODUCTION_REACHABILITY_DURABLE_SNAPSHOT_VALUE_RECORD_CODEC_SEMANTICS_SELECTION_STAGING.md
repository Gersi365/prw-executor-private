# Phase 152 C03e-HB — Production Reachability Durable Snapshot Value-Record Codec Semantics Selection — STAGING

## 1. Purpose

C03e-HB selects only the canonical provider-neutral byte representation for one already-typed `ReachabilityDurableSnapshot` value after canonically closed C03e-HA.

This checkpoint does **not** select or materialize a database key, key prefix/keyspace, persistence product, provider revision/CAS implementation, transaction API, schema deployment, migration runner, credentials, TLS/RBAC, runtime owner, startup recovery orchestration, listener/readiness, traversal/dialing/networking, deployment, restart, or merge.

The selected boundary is deliberately narrower than the historical C02f-AA live-owner key/value codec: HB selects the **value-record codec only** because HA still leaves database key encoding/keyspace separately gated.

## 2. Exact prerequisite

Canonical predecessor C03e-HA:

- branch: `phase-152-c03e-ha-production-reachability-typed-durable-snapshot-integration-source-materialization-staging`
- exact head: `8206e83e8180413da00cd8976774b4074dadf714`
- exact tree: `5b8867039e4c126f8853dc4a16f3684fc4c711a1`
- gate: `C03E_HA_PRODUCTION_REACHABILITY_TYPED_DURABLE_SNAPSHOT_INTEGRATION_SOURCE_MATERIALIZED`

HA established that `ReachabilityDurableSnapshot` carries:

1. one `PeerConnectivityPlanDurableState`; and
2. one `CandidatePublicationFreshnessRecord` for the same exact `PeerConnectivityIdentity`.

HA also kept persistence codec/schema/version/keyspace/provider work explicitly separate.

## 3. Fresh exact-HA audit conclusion

The fresh exact-HA audit establishes:

- `ReachabilityDurableSnapshot` is already a provider-neutral typed semantic snapshot and exposes `plan()` plus `freshness()` accessors;
- `PeerConnectivityPlanDurableState` already exposes exact peer identity, complete current candidate vector in plan order, and exact optional historical candidate-ID high-watermark;
- `CandidatePublicationFreshnessRecord` already exposes exact peer and exact freshness lifecycle;
- `DeviceId`, `TransportIdentity`, `CandidateId`, connectivity path kind, endpoint address/port, and freshness token have sufficient typed construction/accessor boundaries for a pure manual codec;
- `prw-remote-bridge` and `prw-connectivity` require no serialization dependency for this representation;
- historical C02f-AA establishes repository precedent for a pure versioned fail-closed codec before concrete provider I/O;
- concrete keyspace/provider selection is not required to define one canonical durable snapshot value.

Therefore the narrowest unresolved persistence prerequisite after HA is the value-record representation itself, not provider activation.

## 4. Ownership and placement law

The future codec is bridge-owned because `ReachabilityDurableSnapshot` and freshness semantics are bridge-owned while its plan member is connectivity-owned typed durable state.

The intended future source location is conceptually:

`crates/prw-remote-bridge/src/reachability_durable_snapshot_codec.rs`

The codec is pure and runtime-independent. It must perform no:

- database/provider call;
- key construction;
- network I/O;
- filesystem I/O;
- clock read;
- randomness generation;
- retry/reconciliation;
- task/thread spawn;
- compression;
- encryption/MAC/signature construction;
- persistence mutation;
- owner recovery/install;
- candidate mutation;
- traversal operation.

No new cryptographic primitive is selected by HB.

## 5. Canonical record identity

HB selects one initial canonical durable snapshot value format:

- magic: exact four ASCII bytes `PRWS`;
- major version: unsigned big-endian `u16 = 1`;
- minor version: unsigned big-endian `u16 = 0`.

`PRWS` means only the production reachability durable **snapshot value record** selected here. It is not a network protocol frame and is not a database key prefix.

Unknown magic or any version other than exact v1.0 fails closed.

No backward/forward migration or multi-version decode policy is selected by HB.

## 6. Integer and canonicality law

Unless explicitly stated otherwise, every multi-byte integer in PRWS v1.0 is encoded unsigned big-endian.

The decoder must reject:

- truncation;
- trailing bytes;
- arithmetic/host-size conversion overflow;
- unknown enum/tag values;
- non-zero reserved fields/bits;
- inconsistent explicit lengths/counts;
- non-canonical IPv4 padding;
- invalid typed identity/candidate/endpoint/freshness values.

There is exactly one canonical byte encoding for one representable typed snapshot value.

No alternative text/JSON/CBOR/MessagePack/serde representation is selected.

## 7. Exact PRWS v1.0 layout

The fixed header is exactly **72 bytes**:

| Offset | Width | Field | Canonical value / meaning |
|---:|---:|---|---|
| 0 | 4 | magic | exact `PRWS` |
| 4 | 2 | major | `1` |
| 6 | 2 | minor | `0` |
| 8 | 2 | freshness lifecycle | tag defined in §8 |
| 10 | 2 | reserved | exactly zero |
| 12 | 8 | total record length | exact complete encoded byte length |
| 20 | 8 | `DeviceId` UTF-8 byte length | exact length of following device bytes |
| 28 | 2 | candidate count | exact current candidate count, `0..=16` |
| 30 | 2 | state flags | only bit 0 defined; all other bits zero |
| 32 | 8 | historical candidate-ID high-water | canonical rule in §9 |
| 40 | 32 | freshness token field | canonical rule in §8 |

After the fixed header, fields occur in this exact order:

1. exact `DeviceId` UTF-8 bytes, length from offset 20;
2. exact 32-byte `TransportIdentity`;
3. exactly `candidate_count` fixed-width candidate entries, each exactly 32 bytes, in the exact order retained by `PeerConnectivityPlanDurableState::candidates()`.

Therefore the exact total length is:

`104 + device_id_utf8_len + (32 * candidate_count)`.

The explicit total-length field must equal both that computed value and the actual input byte length.

## 8. Freshness lifecycle and token encoding

The exact freshness lifecycle tags are:

- `1` = `NewLifecycleEligible(token)`;
- `2` = `Established(token)`;
- `3` = `RecoveryRequired`;
- `4` = `Retired`.

For tags 1 and 2:

- the fixed 32-byte freshness-token field contains the exact opaque token bytes;
- all-zero token bytes are invalid;
- decoding must construct the existing `CandidatePublicationFreshnessToken` through its existing typed constructor;
- no numeric, timestamp, ordering-counter, UUID, identity-derived, or provider-revision meaning is assigned to those bytes.

For tags 3 and 4:

- the entire fixed 32-byte freshness-token field must be exactly zero;
- any non-zero byte is non-canonical and fails closed;
- zero bytes here are an absence sentinel in the record format only and never construct a valid freshness token.

The decoder reconstructs the existing `CandidatePublicationFreshnessRecord` variant for the exact decoded peer.

HB does not activate the separately gated new-lifecycle/bootstrap issuance callsite merely because v1.0 can represent `NewLifecycleEligible`.

## 9. Historical candidate-ID high-water encoding

`state_flags` bit 0 means **historical candidate-ID high-water present**.

All bits 1 through 15 are reserved and must be zero.

Canonical rules:

- bit 0 clear => raw high-water field at offset 32 must be exactly zero;
- bit 0 set => raw high-water field must be non-zero and is decoded through existing `CandidateId::new(...)`;
- no second presence encoding is accepted;
- absence versus presence is preserved exactly.

HB does not move connectivity semantic restoration authority into the codec.

In particular, vector-level semantic conditions such as active-candidate/high-water consistency remain authoritatively validated by the existing `PeerConnectivityPlan::from_durable_state(...)` restoration boundary when the owner recovers/reloads. The codec must not silently repair, lower, synthesize, or recompute historical high-water state.

## 10. Exact peer identity encoding

The record carries one exact peer identity shared by both typed snapshot members.

### DeviceId

- encoded as exact UTF-8 bytes from `DeviceId::as_str()`;
- length encoded as the header `u64` field;
- decoder requires valid UTF-8;
- decoder reconstructs through the existing `DeviceId::new(...)` validation boundary;
- no normalization, case folding, trimming, alternate textual form, hash, numeric substitution, or provider key substitution is allowed.

### TransportIdentity

- encoded as exactly 32 bytes from the existing opaque transport identity;
- decoder reconstructs through existing `TransportIdentity::new(...)`;
- the prohibited all-zero transport identity therefore fails closed.

The decoded exact peer is used for both:

- `PeerConnectivityPlanDurableState`; and
- `CandidatePublicationFreshnessRecord`.

No `DeviceId`-only or transport-only persisted identity is selected.

## 11. Candidate entry encoding

Each current candidate entry is exactly **32 bytes**:

| Relative offset | Width | Field |
|---:|---:|---|
| 0 | 8 | candidate ID |
| 8 | 2 | connectivity path-kind tag |
| 10 | 2 | address-family tag |
| 12 | 2 | port |
| 14 | 2 | reserved |
| 16 | 16 | canonical address bytes |

Candidate ID:

- unsigned big-endian `u64`;
- zero is invalid;
- decoder uses existing `CandidateId::new(...)`.

Path-kind tags:

- `1` = `LocalDirect`;
- `2` = `InternetDirect`;
- `3` = `Relay`;
- all other values fail closed.

Address-family tags:

- `1` = IPv4;
- `2` = IPv6;
- all other values fail closed.

Port:

- exact unsigned big-endian `u16`;
- zero is invalid through the existing endpoint constructor.

Reserved candidate field:

- exactly zero.

Address bytes:

- IPv4: first four bytes are the exact IPv4 octets and remaining twelve bytes are exactly zero;
- IPv6: all sixteen bytes are the exact IPv6 octets;
- IPv4 records with any non-zero padding byte fail closed.

Decoded endpoints are constructed only through existing `ConnectivityEndpoint::new(...)`, preserving current rejection of zero ports, unspecified/multicast addresses and IPv4 limited broadcast.

Decoded candidates are constructed only through existing typed connectivity constructors.

Candidate order is semantically retained as stored plan order; the codec must neither sort nor deduplicate.

## 12. Candidate count and boundedness

The candidate-count field must be at most existing `MAX_CONNECTIVITY_CANDIDATES = 16`.

The decoder must verify exact byte-count consistency before accepting the record.

The codec does not widen connectivity capacity and does not invent a second storage-specific candidate bound.

Vector-level duplicate-ID, duplicate-endpoint and historical high-water semantic validation remains the existing connectivity restoration authority; HB does not create an alternate semantic validator or repair path.

## 13. Decode result law

A successful PRWS v1.0 decode reconstructs exactly one provider-neutral typed `ReachabilityDurableSnapshot` using existing domain constructors.

The decode path conceptually performs:

1. strict framing/version/length/reserved validation;
2. exact peer reconstruction;
3. exact current-candidate typed reconstruction in encoded order;
4. exact optional high-water reconstruction without recomputation;
5. `PeerConnectivityPlanDurableState::from_parts(...)` from decoded provider-neutral fields;
6. exact freshness record reconstruction for the same peer;
7. existing `ReachabilityDurableSnapshot::new(...)` exact-peer consistency construction.

It does **not** call `PeerConnectivityPlan::from_durable_state(...)` as a replacement for owner recovery/reload authority. Existing HA owner recovery/reload remains responsible for semantic durable-plan restoration and `ReachabilitySnapshotError::PlanRestoration(...)` classification.

## 14. Encode result law

Encoding reads only the existing typed snapshot accessors.

It must:

- preserve exact `DeviceId` bytes;
- preserve exact `TransportIdentity` bytes;
- preserve candidate vector order;
- preserve exact candidate IDs, kinds, addresses and ports;
- preserve exact optional historical high-water presence/value;
- preserve exact freshness lifecycle;
- preserve exact current token bytes where the lifecycle carries a token;
- emit canonical zero token bytes where the lifecycle carries no token;
- compute exact count/length fields;
- emit all reserved fields/bits as zero.

Encoding performs no state mutation and no semantic rebaselining.

## 15. Error taxonomy selection

The future pure codec must expose a codec-local fail-closed error family sufficient to distinguish at least:

- invalid magic;
- unsupported version;
- invalid/truncated/trailing/overflowing record length;
- invalid reserved fields/flags;
- invalid lifecycle/token canonicality;
- invalid `DeviceId`;
- invalid `TransportIdentity`;
- invalid candidate count;
- invalid candidate ID;
- invalid path-kind tag;
- invalid address-family/padding;
- invalid endpoint;
- invalid high-water canonicality;
- snapshot construction/peer-binding failure where applicable.

Exact Rust variant naming remains a source-shape detail, provided these fail-closed distinctions are preserved.

The codec does not create provider/network/retry errors and does not map codec corruption to storage absence or new-lifecycle eligibility.

## 16. No checksum, compression, encryption or authentication envelope

PRWS v1.0 contains no codec-level checksum, compression, encryption, MAC, signature, nonce, timestamp or revision field.

This is intentional:

- storage integrity/durability belongs to the later provider boundary;
- transport security/credentials belong to later provider/security composition;
- freshness replay authority is already represented by the opaque verifier token;
- HB introduces no new cryptographic primitive.

A future provider may use its own authenticated transport/storage integrity mechanisms without changing canonical PRWS value bytes unless a separately authorized migration changes the codec version.

## 17. Explicit keyspace separation

HB selects **no database key bytes**.

It selects no:

- key prefix;
- key version;
- key delimiter/escaping rule;
- hashed key;
- etcd prefix;
- SQL primary key;
- embedded-store path;
- filesystem name;
- namespace/range rule.

A later keyspace checkpoint must bind whatever key representation it selects to the exact peer decoded from PRWS and must fail closed on mismatch; HB itself does not choose that key representation.

This explicit separation is the principal scope distinction from historical C02f-AA.

## 18. Provider/CAS separation

HB selects no concrete implementation of `ReachabilityDurableStore`.

It does not select:

- etcd versus SQL versus embedded/filesystem storage;
- provider revision/mod_revision/version compare;
- provider transaction syntax;
- lease/TTL/watch;
- retry/reconciliation;
- connection/client construction;
- endpoint/topology;
- credentials/TLS/RBAC;
- schema DDL/migration tooling;
- replication/durability settings.

Existing semantic store law remains authoritative: exact expected-current freshness comparison, definite `Committed` versus definite `StaleExpected`, and ambiguous/unavailable fail-closed recovery.

## 19. Historical precedent retained without scope import

C02f-AA is precedent only for the repository pattern of a pure deterministic versioned fail-closed codec before real provider I/O.

HB does **not** import C02f-AA's live-owner key prefix, `PRWL` record shape, fence/attempt-ID fields, etcd provider choice, or key/value bundling into production reachability snapshots.

The new snapshot record has its own exact `PRWS` v1.0 value identity and remains provider-neutral.

## 20. Non-activation boundary

C03e-HB is documentation-only.

It does not:

- add Rust/Kotlin source;
- change Cargo manifests or lockfiles;
- add serialization dependencies;
- change workflows;
- encode/decode production data;
- write/read a persistence provider;
- populate/synchronize the production owner map;
- perform Agent startup recovery;
- authorize bootstrap/new-lifecycle freshness issuance;
- activate candidate handoff/current-Mesh response;
- activate traversal/listener/readiness/dialing/networking;
- deploy/restart/recover a process;
- merge or delete branches.

## 21. Expected narrow source successor

A source successor must start with a fresh exact-HB-head audit.

If topology remains consistent, the expected narrow source-materialization ceiling is:

1. one new pure bridge codec source, conceptually `crates/prw-remote-bridge/src/reachability_durable_snapshot_codec.rs`;
2. `crates/prw-remote-bridge/src/root.rs` registration/export only;
3. one source-materialization contract.

No Cargo manifest or lockfile change is expected from the exact HA/HB dependency audit.

Focused tests should prove:

- exact v1.0 byte layout;
- Accepted typed round trips for every freshness lifecycle;
- IPv4/IPv6 canonicality;
- candidate order preservation;
- empty and populated candidate sets;
- historical high-water present/absent preservation;
- empty-current + historical-high-water preservation;
- exact truncation/trailing/version/reserved/tag/length failures;
- invalid token/identity/candidate/endpoint failures;
- no semantic high-water recomputation/repair.

Any fourth path, manifest/lockfile change, keyspace/provider/schema selection, or runtime/network activation is a stop-and-re-audit condition rather than automatic scope expansion.

## 22. Target closure

Target canonical closure:

`CLOSED_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_VALUE_RECORD_CODEC_SEMANTICS_SELECTION`

Target canonical gate:

`C03E_HB_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_VALUE_RECORD_CODEC_SEMANTICS_SELECTED`

Until exact-head validation and immutable audit closure are complete, this file remains a staging selection artifact only.
