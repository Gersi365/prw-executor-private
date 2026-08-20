# Phase 152 C02f-Z — etcd Storage / Transaction Contract Selection Lock

Status: `SELECTED / VERSIONED_BINARY_EXACT_PEER_KEY_V1_0 / SINGLE_VERSIONED_AUTHORITY_RECORD_V1_0 / FULL_EXACT_PEER_BINDING_IN_RECORD / FIXED_16_BYTE_BIG_ENDIAN_U128_FENCE / NONZERO_32_BYTE_AUTHORITY_ATTEMPT_ID / CURRENT_AND_RELEASED_TOMBSTONE_STATES / LINEARIZABLE_EXACT_GET / DUAL_CAS_MOD_REVISION_AND_EXACT_VALUE / TXN_FAILURE_BRANCH_AUTHORITATIVE_GET / INDETERMINATE_MUTATION_REOBSERVATION / NO_BLIND_MUTATION_RETRY / CONTROL_PLANE_TO_CONNECTIVITY_IDENTITY_BOUNDARY_SELECTED / BRIDGE_GRANT_REMAINS_PEER_PLUS_FENCE / RECOVERY_EPOCH_LAYOUT_DEFERRED / TLS_AUTH_RBAC_DEFERRED / NO_ENDPOINT_OR_RUNTIME_ACTIVATION`

Date: 2026-08-20
Repository: `powercode365-dotcom/prw-executor-private`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Frozen C02d head: `857583b25ed1206317641a93fd8f927819c954d8`
C02f-Y predecessor head: `ed95a837f766fb479bf82738cb83fab69d84b1a6`
C02f-Y predecessor tree: `9aca11b0fd2199f7e16a37db7728bca173279975`

## Purpose

This contract closes the storage/schema/normal-transaction selection gate required before a real etcd-backed live-owner adapter can be implemented.

It selects the exact physical key and authority-record encodings, the application-owned fence encoding, the internal authority-attempt identifier representation, the normal etcd compare-and-swap guard, definite compare-failure observation, indeterminate mutation reconciliation semantics, and the exact logical identity type boundary between `prw-remote-bridge` and the provider-specific `prw-control-plane` implementation.

This contract does **not** select or activate:

- the recovery-epoch bit partition or external high-water provider;
- TLS features, trust roots, certificates, credentials or RBAC;
- etcd endpoints, cluster topology, deployment platform or quorum configuration;
- a Tokio/runtime bootstrap owner;
- production network I/O;
- R1-R4 effect-boundary fencing implementation;
- Phase 153/154 production activation.

## Inherited architecture preserved

The following are inherited and remain non-negotiable:

1. the exact authority namespace is `DeviceId + TransportIdentity`;
2. `DeviceId` is logical authenticated PRW identity;
3. `TransportIdentity` is independently rotatable transport identity;
4. IP address, port, endpoint, NAT mapping, relay and path are never authority identity;
5. shared control-plane authority placement T3 is selected;
6. etcd v3.7 is selected;
7. `etcd-client = =0.19.0` is selected and dependency-materialized with default features disabled;
8. the logical PRW fence remains `ReachabilityLiveOwnerFence(NonZeroU128)`;
9. authority ambiguity/unavailability fails closed;
10. Watch, TTL, lease and clocks are not primary currentness authority;
11. stale release cannot clear newer authority;
12. stale network/task effects must later be rejected at or atomically with R1-R4 effect boundaries;
13. the C02f-X/Y asynchronous production authority port remains separate from the synchronous Sans-I/O semantic seam.

## Source facts constraining this selection

### DeviceId

`DeviceId` stores a Rust `String` and currently rejects only empty/all-whitespace values. It does not reserve a delimiter alphabet and does not normalize Unicode.

Therefore the selected key and record encodings MUST frame the exact UTF-8 bytes and MUST NOT rely on slash, colon, NUL, percent-encoding or another textual delimiter being absent from the identifier.

### TransportIdentity

`TransportIdentity` is exactly 32 opaque bytes with all-zero rejected.

The selected key and record therefore store those exact 32 bytes directly; no hex/base64 alternate spelling is admitted.

### Existing PRW binary precedent

The existing `reachability_freshness_wire` contract uses a fixed 4-byte magic, `u16` major/minor versions, fixed-width big-endian integers, reserved-zero fields, exact-length validation and raw 32-byte identity/token fields.

C02f-Z deliberately follows those encoding disciplines without reusing the PRWF wire format itself.

## Selected exact etcd key format — K1 v1.0

One exact peer live-owner namespace maps to exactly one binary etcd key.

### Domain prefix

The exact fixed ASCII prefix is:

```text
/prw/reachability/live-owner/
```

The prefix is part of the physical key bytes and permits operational prefix scans without making the identity suffix textual.

### Key fields after the prefix

After the exact prefix, bytes are encoded in this exact order:

| Field | Encoding |
| --- | --- |
| key major | unsigned `u16`, big-endian, exact value `1` |
| key minor | unsigned `u16`, big-endian, exact value `0` |
| DeviceId byte length | unsigned `u64`, big-endian |
| DeviceId | exact `DeviceId::as_str().as_bytes()` UTF-8 bytes |
| TransportIdentity | exact 32 bytes from `TransportIdentity::as_bytes()` |

There is no delimiter, textual escaping, hash, checksum, terminal marker or trailing field.

### Key canonicality rules

- only major `1`, minor `0` is accepted by the v1 decoder;
- `DeviceId` byte length is the exact number of following UTF-8 bytes before the fixed 32-byte transport suffix;
- the length is encoded through checked conversion to `u64`; no truncation is permitted;
- DeviceId bytes must decode as UTF-8 and satisfy the existing `DeviceId` constructor;
- no Unicode normalization, case folding, trimming or alternate spelling is performed;
- TransportIdentity must satisfy the existing non-zero 32-byte constructor;
- trailing bytes are invalid;
- malformed/non-canonical keys fail closed;
- provider request-size limits may reject an otherwise source-valid identifier, but the codec must never truncate or rewrite it.

## Selected authority value record — PRWL v1.0

Each exact authority key stores one complete versioned authority record. Current-owner state and released/tombstoned state are two lifecycle values of the same record; normal release never deletes the key.

### Exact record header

The first 12 bytes are:

| Offset | Field | Encoding |
| ---: | --- | --- |
| 0..4 | magic | exact ASCII bytes `PRWL` |
| 4..6 | record major | unsigned `u16`, big-endian, exact value `1` |
| 6..8 | record minor | unsigned `u16`, big-endian, exact value `0` |
| 8..10 | lifecycle | unsigned `u16`, big-endian |
| 10..12 | flags/reserved | unsigned `u16`, big-endian, exact value `0` |

Selected lifecycle values:

- `1` = `Current`;
- `2` = `Released`.

Any other lifecycle value or non-zero flags field is invalid and fails closed.

### Exact record body

Immediately after the 12-byte header:

| Field | Encoding |
| --- | --- |
| DeviceId byte length | unsigned `u64`, big-endian |
| DeviceId | exact UTF-8 bytes |
| TransportIdentity | exact 32 bytes |
| PRW fence | exact 16-byte unsigned big-endian `u128` |
| authority attempt ID | exact 32 opaque bytes |

The record length is therefore exactly `100 + DeviceId UTF-8 byte length` bytes.

No trailing bytes are permitted.

### Exact-peer binding rule

The record duplicates the complete logical namespace intentionally.

A record loaded from key K is valid only when:

1. the key decodes canonically;
2. the record decodes canonically;
3. record DeviceId exactly equals key DeviceId;
4. record TransportIdentity exactly equals key TransportIdentity.

A byte-valid record copied to another exact-peer key is therefore rejected rather than silently acquiring the destination namespace.

## Selected fence external encoding

The logical `ReachabilityLiveOwnerFence(NonZeroU128)` is encoded as exactly 16 unsigned big-endian bytes using the full raw `u128` value.

Rules:

- zero is invalid on decode;
- no decimal/textual alternate form exists;
- the full 16-byte value is persisted unchanged;
- etcd revision/version/lease ID never becomes the PRW fence;
- this selection intentionally does **not** decompose the raw value into recovery epoch/sequence fields;
- a later recovery-epoch selection may interpret bits inside the same `u128` without changing this storage encoding.

## Selected authority-attempt identifier

Each acquisition/replacement logical mutation creates one fresh internal `AuthorityAttemptId` represented as exactly 32 opaque bytes.

Rules:

1. all-zero is invalid;
2. the identifier is generated internally, never accepted from remote/request-controlled payloads;
3. it is fresh for each new logical acquisition/replacement attempt;
4. retries of the **same** logical mutation after authoritative reconciliation reuse the same attempt ID and intended fence;
5. a new attempt after definite contention/re-observation uses a new attempt ID;
6. the selected generation requirement is cryptographically strong process-local randomness with negligible collision probability; the exact Rust RNG crate/API remains an implementation-materialization detail and is not selected by this contract;
7. the attempt ID is not a secret credential and does not independently grant authority;
8. the attempt ID remains provider/internal metadata and is not added to `ReachabilityLiveOwnerGrant`;
9. a successful `Current` record stores the acquisition attempt ID;
10. release writes `Released` while preserving the exact same fence and attempt ID from the owner being released.

This makes the attempt ID sufficient to reconcile an indeterminate acquisition mutation while preserving the public semantic rule that the grant is exact-peer identity plus authority fence.

## Selected identity boundary between bridge and control-plane

The provider implementation remains owned by `prw-control-plane` and MUST NOT depend on `prw-remote-bridge`.

For later source materialization, the selected lower-level dependency direction is:

```text
prw-core <- prw-connectivity <- prw-control-plane
                               ^
                               |
                     prw-remote-bridge orchestration
```

More precisely:

- `prw-control-plane` may add a normal dependency on the lower-level `prw-connectivity` crate;
- control-plane key/record/provider operations take the strongly typed `PeerConnectivityIdentity` or its exact typed components, not unchecked strings/raw request fields;
- `prw-control-plane` returns provider-owned outcomes/raw fence values and never constructs `ReachabilityLiveOwnerGrant`;
- `prw-remote-bridge` remains the layer that maps an unambiguous provider result into `ReachabilityLiveOwnerFence` and `ReachabilityLiveOwnerGrant`;
- the existing bridge-to-control-plane dependency may be promoted/materialized in a later source tranche;
- no inverse `prw-control-plane -> prw-remote-bridge` dependency is permitted.

This preserves an acyclic dependency graph and exact type validation.

## Selected authoritative read rule

Every safety-relevant direct read of live-owner authority uses a normal latest etcd Range/Get with default linearizable semantics.

Explicit serializable/member-local reads are prohibited for:

- acquisition pre-read;
- currentness proof;
- release pre-read;
- indeterminate mutation reconciliation;
- established-state bootstrap/recovery checks.

Watch remains advisory only and cannot prove Current.

## Selected normal CAS guard — dual compare

For an existing valid authority record, the provider first performs a linearizable exact-key Get and captures:

- the exact canonical value bytes;
- the key's `mod_revision`;
- the decoded/validated authority state.

Any mutation based on that observation uses one etcd Txn with **both** compares against the same exact key:

1. `mod_revision == observed_mod_revision`;
2. `value == exact_observed_record_bytes`.

The compares are conjunctive: both must pass.

The success branch contains exactly one Put of the complete canonical successor record for that key.

Why both are selected:

- `mod_revision` proves no intervening modification of the key within the active cluster lineage;
- exact value comparison proves the complete authoritative application record remains byte-identical to the validated observation;
- the application-owned fence remains inside the record and is not replaced by provider revision metadata;
- the extra compare cost is acceptable for this low-cardinality authority operation and buys a stronger fail-closed guard against unexpected byte/state drift.

## Selected Txn failure branch

The Txn failure branch performs an exact-key Range/Get and performs no write.

This is selected because etcd evaluates the compare set and one branch atomically, so a compare failure can return the authoritative state observed at that transaction linearization point without a write.

The returned failure-branch state is decoded and classified before any new logical mutation attempt is constructed.

No compare failure is converted into success.

## Acquisition/replacement transaction protocol

For an existing exact namespace:

1. linearizable Get exact key;
2. decode key/value and require exact identity match;
3. reject malformed, absent-established or impossible state fail-closed;
4. obtain a strictly newer safe fence from the separately selected/approved fence-allocation policy;
5. generate one fresh non-zero 32-byte authority attempt ID;
6. build canonical `Current` successor bytes;
7. Txn compare `mod_revision` **and** exact prior value bytes;
8. success branch: Put exactly the canonical `Current` successor;
9. failure branch: Get exact key, no mutation;
10. definitive `succeeded=true` response may map to provider acquisition success;
11. definitive compare failure maps to contention/stale observation and never manufactures a grant;
12. only `prw-remote-bridge` may construct the semantic grant from an unambiguous successful provider outcome.

This contract does not select the recovery epoch/sequence allocator. The provider codec accepts the full selected non-zero `u128` fence produced by that future allocator.

## Currentness protocol

For a previously issued semantic grant:

1. require the grant peer to match the requested exact peer;
2. linearizable Get the exact key;
3. decode and require exact key/record identity binding;
4. `Current` is returned only when lifecycle is `Current` and persisted fence exactly equals the grant fence;
5. `Released` or a different valid fence is `Stale`;
6. missing established state, corrupt state, impossible state or authority unavailability is an error/fail-closed result, never `Current`.

The internal attempt ID is not a second grant credential. Fence non-reuse plus exact-peer binding remain the public ownership generation semantics.

## Release transaction protocol

Release is a liveness operation and never weakens high-water state.

1. linearizable Get exact key;
2. decode exact current state;
3. if lifecycle is not `Current` or fence differs from the supplied grant, return `NotCurrent`;
4. preserve the exact peer identity, fence and authority attempt ID;
5. build canonical `Released` successor bytes with lifecycle `2` and all other authority identity/generation fields unchanged;
6. Txn compare both observed `mod_revision` and exact observed Current value;
7. success branch Put the Released successor;
8. failure branch Get exact key and perform no write;
9. a stale release can therefore never clear or overwrite a newer owner;
10. the record is not deleted.

A later acquisition from Released state must use a strictly newer safe fence and a fresh attempt ID.

## First-ever absent-key bootstrap rule

C02f-Z deliberately does not treat key absence as permission to manufacture fence 1.

For an established/recovered namespace, absence is fail-closed/recovery-required.

A first-ever creation transaction may use `version == 0` as the etcd absence CAS only after a separately approved bootstrap/recovery authority proves that the exact namespace is eligible for first creation under the selected recovery high-water regime.

Until the recovery epoch/high-water contract is selected and materialized, production first-ever creation from absence remains disabled.

## Selected indeterminate mutation reconciliation

A transport timeout, connection loss, leader transition or other error without a definitive Txn response does **not** prove non-commit.

### Common rule

1. discard any permission to infer Current from the failed RPC result;
2. retain the exact intended successor bytes, intended fence and attempt ID in operation-local state;
3. when authority becomes available, perform a new linearizable exact-key Get;
4. decode and validate the full record;
5. never blindly retransmit the mutation before this re-observation.

### Indeterminate acquisition

After re-observation:

- exact `Current` record with intended peer + intended fence + same attempt ID => the original mutation committed and, if still exact-current, may map to acquisition success;
- a valid different/newer Current record or Released successor => the original attempt is not current and cannot produce a grant;
- the exact pre-mutation record still present => the original mutation did not commit; the same logical mutation may be deliberately reissued using the same intended fence/attempt ID only after this proof;
- absent-established, corrupt, impossible older state or still-unavailable authority => fail closed.

If the intended acquisition committed and was then superseded before reconciliation, the later authoritative record wins; the original operation does not regain authority merely because it may have committed transiently.

### Indeterminate release

The provider retains the pre-release Current record, including its attempt ID.

After re-observation:

- matching `Released` record with same peer + fence + attempt ID => release committed;
- exact pre-release `Current` record still present => release did not commit; the same release may be deliberately reissued only after this proof;
- different/newer valid state => supplied grant is no longer current and release cannot overwrite it;
- absent/corrupt/impossible/unavailable state => fail closed.

## Error-mapping rule for the already-selected async bridge port

C02f-Y currently exposes provider-neutral semantic errors:

- `UnavailableOrAmbiguous`;
- `FenceExhausted`.

C02f-Z does not expand that public enum.

The future control-plane provider may use richer internal classifications for missing-established state, corrupt encoding, invalid identity binding, ambiguous outcome and provider failure, but the initial bridge mapping remains fail closed:

- all unresolved/missing-established/corrupt/ambiguous provider states -> `UnavailableOrAmbiguous`;
- selected fence-allocation exhaustion -> `FenceExhausted`;
- no internal failure maps to Current/Granted.

A richer public error taxonomy requires a separate API decision if later operational requirements justify it.

## Canonical validation requirements for the first codec/Txn source tranche

Before any real etcd endpoint is contacted, implementation must prove at minimum:

1. key encode/decode exact roundtrip for DeviceId values containing delimiter-like Unicode/text bytes;
2. distinct `(DeviceId, TransportIdentity)` pairs never encode to the same key;
3. key decoder rejects wrong version, invalid UTF-8, zero transport identity, impossible length and trailing bytes;
4. record encode/decode exact roundtrip for Current and Released;
5. record decoder rejects wrong magic/version/lifecycle, non-zero flags, zero fence, zero attempt ID, invalid identity and trailing bytes;
6. record/key exact-peer mismatch fails closed;
7. `u128` fence encodes to exactly 16 big-endian bytes and zero is rejected;
8. Released preserves fence and attempt ID;
9. dual-CAS request construction includes both `mod_revision` and exact value compares;
10. failure branch contains Get only and no mutation;
11. definitive compare failure cannot yield Granted/Current;
12. indeterminate acquisition with matching intended attempt reconciles to committed only when still exact-current;
13. indeterminate acquisition superseded by a newer state cannot yield authority;
14. indeterminate release cannot clear a newer owner;
15. no serializable read is used for safety decisions;
16. no Watch/lease/TTL result proves currentness;
17. no runtime, endpoint, TLS or network activation is introduced by codec/state-transition tests.

## Source/dependency materialization boundary after this lock

After C02f-Z, the next authorized implementation can be split safely into narrow source tranches:

1. pure control-plane key/value codec and validation types;
2. provider-owned deterministic transaction-plan/result mapping using a mock/in-memory scripted KV boundary;
3. bridge wrapper that maps definitive provider outcomes into the already-selected async semantic port;
4. only after those pass, real `etcd-client` Get/Txn wiring against no production endpoint;
5. TLS/auth/RBAC and disposable integration remain later gates;
6. recovery epoch/high-water must be selected before production absence/bootstrap or stale-snapshot recovery can be activated.

## Explicitly not selected by C02f-Z

C02f-Z does **not** select:

- 64/64 or another recovery epoch/sequence split;
- global recovery epoch scope;
- external immutable epoch ledger provider/custody/schema;
- AWS/Azure/GCS;
- TLS feature flags or crypto-provider acceptance;
- CA/cert/credential/RBAC values;
- etcd cluster voter count, region, AZ or storage platform;
- Agent/executor runtime ownership;
- production endpoints;
- background retry policy or detached authority tasks;
- R1-R4 effect activation;
- Phase 153/154 production mutation.

## Selection conclusion

The normal-operation authority storage/transaction contract is now selected:

- exact versioned binary key framed from `DeviceId` UTF-8 bytes plus raw 32-byte `TransportIdentity`;
- one exact-peer-bound `PRWL` v1.0 record per namespace;
- fixed 16-byte big-endian full `u128` fence;
- non-zero 32-byte internal authority attempt ID preserved through release;
- `Current` and `Released` lifecycle states, never delete-on-release;
- linearizable exact-key reads;
- conjunctive `mod_revision + exact value` CAS;
- authoritative Get in the Txn failure branch;
- explicit re-observation before any retry after an indeterminate mutation;
- strong typed identity boundary through `prw-connectivity` on the control-plane side;
- public bridge grant remains exact peer + fence.

These selections close the normal storage/Txn design ambiguity without activating the separately deferred recovery, TLS, deployment, runtime or network gates.
