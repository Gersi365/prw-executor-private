# Phase 152 C03e-HC — Production Reachability Durable Snapshot Value-Record Codec Source Materialization — STAGING

## 1. Purpose

C03e-HC materializes only the pure bridge-owned `PRWS` v1.0 durable snapshot **value** codec selected by canonically closed C03e-HB.

It does not select or materialize database key bytes/keyspace, a concrete durable-store provider, provider revision/CAS syntax, schema/migrations, credentials/TLS/RBAC, runtime/task ownership, Agent startup recovery orchestration, bootstrap issuance, candidate handoff, traversal/listener/readiness/dialing/networking, deployment, restart, or merge.

## 2. Exact prerequisite

Canonical predecessor C03e-HB:

- branch: `phase-152-c03e-hb-production-reachability-durable-snapshot-value-record-codec-semantics-selection-staging`
- exact head: `e17dda3c5718cdfd04f5b0c8ef7ac07e40d27142`
- exact tree: `7b6eceaf88759e66c61341963843cb37b6e6aad0`
- gate: `C03E_HB_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_VALUE_RECORD_CODEC_SEMANTICS_SELECTED`

HB selected a provider-neutral `PRWS` v1.0 value representation and explicitly bounded its source successor to one bridge codec source, bridge root registration, and one source-materialization contract, with no Cargo/lockfile change expected.

## 3. Fresh exact-HB audit conclusion

The exact-HB source audit confirms:

- `ReachabilityDurableSnapshot` publicly exposes `plan()` and `freshness()` and constructs only through exact-peer `ReachabilityDurableSnapshot::new(...)`;
- `PeerConnectivityPlanDurableState` publicly exposes exact peer, ordered candidate slice, optional historical candidate-ID high-water, and `from_parts(...)` for decoded provider-neutral fields;
- `CandidatePublicationFreshnessRecord` exposes every selected lifecycle constructor and exact token accessor;
- `DeviceId`, `TransportIdentity`, `CandidateId`, `ConnectivityEndpoint`, and connectivity candidates already provide strict typed reconstruction boundaries;
- `prw-remote-bridge` already depends on `prw-core` and `prw-connectivity`; no new serialization or other dependency is required;
- root registration is one existing module-export seam;
- no keyspace/provider/runtime code is needed to implement canonical value bytes.

Therefore HC may proceed as a three-path source-only materialization directly above exact HB.

## 4. Exact intended diff ceiling

HC is limited to exactly:

1. `crates/prw-remote-bridge/src/reachability_durable_snapshot_codec.rs` — new pure `PRWS` codec plus focused tests;
2. `crates/prw-remote-bridge/src/root.rs` — one module registration/export;
3. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_HC_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_VALUE_RECORD_CODEC_SOURCE_MATERIALIZATION_STAGING.md` — this contract.

Any manifest/lockfile/workflow/provider/keyspace/runtime path is scope contradiction and blocks canonical closure.

## 5. Materialized codec surface

The new bridge module exposes:

- canonical `PRWS` v1.0 constants;
- `encode_reachability_durable_snapshot(&ReachabilityDurableSnapshot) -> Result<Vec<u8>, ReachabilityDurableSnapshotCodecError>`;
- `decode_reachability_durable_snapshot(&[u8]) -> Result<ReachabilityDurableSnapshot, ReachabilityDurableSnapshotCodecError>`;
- one codec-local fail-closed error family.

The module is pure: no database/provider/key operation, I/O, runtime/task, clock, randomness, retry/reconciliation, state mutation, owner recovery/install, candidate mutation, traversal, compression, encryption, MAC, signature, checksum, or production activation.

## 6. Exact `PRWS` v1.0 law retained

HC implements HB without changing its semantics:

- magic exact `PRWS`;
- major/minor exact `1.0`;
- unsigned big-endian multi-byte integers;
- fixed 72-byte header;
- exact total length `104 + DeviceId UTF-8 bytes + 32 * candidate_count`;
- exact peer identity (`DeviceId` + 32-byte `TransportIdentity`);
- exact ordered current candidate vector;
- exact optional historical candidate-ID high-water presence/value;
- exact freshness lifecycle and fixed 32-byte token field;
- fixed 32-byte candidate records;
- IPv4 first four octets plus twelve zero padding bytes;
- IPv6 exact sixteen octets;
- all reserved fields/bits zero.

No database key representation is introduced.

## 7. Decode fail-closed law

Decode rejects at least:

- wrong magic or unsupported version;
- truncation, trailing bytes, inconsistent explicit lengths, host-size overflow;
- non-zero header/candidate reserved fields or unknown state flags;
- unknown freshness/path/address-family tags;
- token canonicality violations;
- invalid UTF-8/`DeviceId`;
- all-zero/invalid `TransportIdentity`;
- candidate count above existing `MAX_CONNECTIVITY_CANDIDATES`;
- zero candidate ID;
- invalid endpoint/zero port;
- non-zero IPv4 padding;
- non-canonical historical high-water presence/value.

Decoded identity/candidate/token values pass only through existing typed constructors.

## 8. Semantic restoration separation

HC deliberately reconstructs `PeerConnectivityPlanDurableState::from_parts(...)` and then `ReachabilityDurableSnapshot::new(...)`.

It does **not** invoke `PeerConnectivityPlan::from_durable_state(...)` during byte decode. Duplicate candidate IDs/endpoints and active/high-water semantic consistency remain the existing owner recovery/reload restoration authority. The codec neither repairs nor recomputes semantic high-water state.

This preserves HA/HB's boundary: byte canonicality in the codec; durable plan semantics in owner recovery.

## 9. Encoding law

Encoding reads existing typed accessors only and preserves:

- exact `DeviceId` and transport identity bytes;
- exact candidate order/IDs/kinds/endpoints;
- exact high-water presence/value;
- exact freshness lifecycle/token bytes.

It emits canonical zero sentinel fields where selected by HB and rejects a typed durable carrier whose candidate count cannot fit the existing product capacity. It does not mutate or rebaseline state.

## 10. Focused tests

The new source tests cover:

- exact deterministic v1.0 bytes;
- encode/decode round-trip for every freshness lifecycle;
- IPv4 and IPv6 handling;
- candidate order preservation;
- empty candidate vector;
- high-water absent/present preservation;
- empty-current plus historical high-water preservation;
- proof that codec decode does not repair a semantically low high-water;
- wrong magic/version;
- truncation/trailing/total-length failures;
- non-zero reserved fields/unknown flags;
- invalid lifecycle/token canonicality;
- invalid peer identity bytes;
- zero candidate ID/unknown path tag;
- invalid IPv4 padding/endpoint;
- invalid high-water encoding;
- candidate-count bounds on decode and encode.

## 11. Non-activation boundary

C03e-HC does not:

- modify Cargo manifests or lockfiles;
- add dependencies;
- change workflows;
- construct or consume database keys;
- call a durable provider;
- choose etcd/SQL/embedded/filesystem persistence;
- select provider revision/CAS details;
- define schema/migrations/credentials/TLS/RBAC;
- alter `ReachabilityDurableStore` semantics;
- populate the production owner map;
- run Agent startup recovery;
- authorize new-lifecycle/bootstrap token issuance;
- activate candidate handoff/current-Mesh responses;
- activate executor/task/cancellation ownership;
- activate traversal/listener/readiness/dialing/networking;
- deploy/restart/recover processes;
- merge or delete branches.

## 12. Closure requirements

Canonical closure requires all of:

1. exact parent/merge-base remains HB head;
2. exact diff remains the three authorized paths only;
3. no manifest/lockfile/workflow/provider/keyspace/runtime path appears;
4. exact-head automatically triggered CI is clean;
5. immutable audit evidence is uploaded to the canonical project Drive folder and raw-read back byte-exact;
6. PR remains draft/open/unmerged while its body moves to `Status: CLOSED`.

## 13. Safe successor

After closure, start with a fresh exact-HC-head read-only audit.

The next persistence prerequisite is not automatically authorized by HC. A likely narrow question is database key/keyspace semantics because HB/HC intentionally materialize only value bytes, but exact-HC evidence must determine whether keyspace selection, transaction planning, or another prerequisite is actually next.

No provider/schema/runtime activation may be bundled by inference.

## 14. Target closure

Target canonical closure:

`CLOSED_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_VALUE_RECORD_CODEC_SOURCE_MATERIALIZATION`

Target canonical gate:

`C03E_HC_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_VALUE_RECORD_CODEC_SOURCE_MATERIALIZED`

Until exact-head CI and immutable evidence closure complete, HC remains STAGING.
