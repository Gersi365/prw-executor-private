# Phase 152 C03e-IQ — Production Durable Registry Record / Key / CAS Semantics Selection

Status: `STAGED_SELECTION`

Gate on closure:
`C03E_IQ_PRODUCTION_DURABLE_REGISTRY_RECORD_KEY_CAS_SEMANTICS_SELECTED`

Canonical closure token on successful exact-head validation and evidence recording:
`CLOSED_PRODUCTION_DURABLE_REGISTRY_RECORD_KEY_CAS_SEMANTICS_SELECTION`

## 1. Scope

C03e-IQ is the documentation-only prerequisite after closed C03e-IP.

C03e-IP selected etcd as the production durable current-state provider family for the current device/current transport registry authority, with:

- provider-specific raw etcd execution owned by `prw-control-plane`;
- Phase 130 membership/device/transport semantics retained by `prw-registry`;
- narrow production composition owned by `prw-agent`.

C03e-IQ selects only the exact provider-neutral durable registry record split, canonical key/value representation, current-state atomicity/CAS law, malformed/ambiguous failure law, and the first source-materialization ceiling required before any Rust provider or semantic adapter is created.

The exact predecessor is C03e-IP head:

`65fdb7659263c4963c16d3a1b74c728a0805aa2e`

C03e-IQ does not materialize Rust/Kotlin/Cargo/lockfile/workflow/runtime source, connect to etcd, create provider resources, create credentials, configure TLS/auth/RBAC, migrate registry state, populate production records, activate production bootstrap, mutate `run()`/`main.rs`, or change production state.

## 2. Exact predecessor authority

Exact predecessor contract:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_IP_PRODUCTION_DURABLE_REGISTRY_PROVIDER_OWNERSHIP_SELECTION_STAGING.md`

Exact C03e-IP blob:

`a01df8287087496f28b9509802196a01992d7718`

Exact predecessor head:

`65fdb7659263c4963c16d3a1b74c728a0805aa2e`

Exact predecessor tree:

`11b3f803ddb84790690990f191dbaccf819f0d77`

C03e-IP deliberately deferred:

- etcd key prefix/key layout;
- membership/device record split;
- binary/text value encoding;
- value/versioning layout;
- exact multi-key atomicity;
- exact initial-create and update compare law;
- malformed/unknown-version mapping;
- first Rust source ceiling.

C03e-IQ resolves those items without authorizing source materialization.

## 3. Phase 130 source model remains semantic authority

Exact source:

`crates/prw-registry/src/lib.rs`

Exact blob:

`cd7eb52eea19354620ff8ad26c9b8aad3f9c5eb6`

Exact Phase 130 contract:

`contracts/DEVICE_REGISTRY_WORKSPACE_MEMBERSHIP_CONTRACT.md`

Exact blob:

`2faf8b014d43583e9180b907f94f40f093e5125b`

The source model contains two separate authority maps:

1. memberships keyed by exact `(WorkspaceId, UserId)`;
2. registered devices keyed by exact `DeviceId`.

A `WorkspaceMembership` contains:

- exact `WorkspaceId`;
- exact `UserId`;
- immutable `WorkspaceRole` in the current Phase 130 API;
- mutable `MembershipLifecycle`.

A `RegisteredDevice` contains:

- one immutable `DeviceIdentityBinding` with exact workspace/user/device/public-identity tuple plus current device lifecycle;
- one separately mutable optional current `TransportIdentity`.

C03e-IQ MUST preserve this semantic split rather than introduce a competing registry model.

## 4. Selected durable record split

C03e-IQ selects exactly two durable current-state semantic record kinds.

### 4.1 Membership record

One durable membership record exists for one exact Phase 130 key:

`(WorkspaceId, UserId)`

Its semantic payload is:

- exact `WorkspaceId`;
- exact `UserId`;
- exact current `WorkspaceRole`;
- exact current `MembershipLifecycle`.

The durable record is retained after `Removed` and MUST NOT be deleted merely because participation became terminal.

### 4.2 Device record

One durable device record exists for one exact Phase 130 key:

`DeviceId`

Its semantic payload is:

- exact immutable `WorkspaceId`;
- exact immutable `UserId`;
- exact immutable `DeviceId`;
- exact immutable canonical public identity profile + bytes;
- exact current `DeviceLifecycle`;
- optional exact current `TransportIdentity`.

The device lifecycle and optional current transport binding MUST occupy the same durable device value. C03e-IQ does not split them into separate keys.

This is required so transport bind/rotation and device revocation can compare and replace one complete current device state atomically without a cross-key currentness gap.

The durable device record is retained after `Revoked` and MUST NOT be deleted merely because participation became terminal.

## 5. Rejected mega-record design

C03e-IQ does not select one workspace-wide, user-wide, or registry-wide mega-record.

A mega-record would:

- merge independently keyed Phase 130 authority domains;
- broaden write contention;
- require unrelated membership/device rewrites;
- complicate exact `DeviceId` lookup required by production peer provenance;
- create a new aggregate model not present in Phase 130.

The selected two-record-kind model preserves existing semantic key boundaries while using explicit etcd transactions only when one operation genuinely spans those boundaries.

## 6. Exact key namespace selection

C03e-IQ selects a registry-only namespace that was not present in the exact C03e-IP repository search:

- membership prefix: `/prw/registry/membership/`
- device prefix: `/prw/registry/device/`

These prefixes are byte strings, not directory semantics.

They are distinct from existing reachability authority namespaces such as `/prw/reachability/...`.

No C03e-IQ operation may use prefix scan, first-match discovery, or broad range discovery as registry authority. Exact typed identifiers are required to construct exact keys.

## 7. Common key versioning law

Both registry key kinds use the same binary version envelope immediately after their exact prefix:

1. unsigned big-endian `u16` major version;
2. unsigned big-endian `u16` minor version.

Initial selected key version:

- major: `1`
- minor: `0`

Decoders MUST require exact `1.0`.

Unknown major or minor versions fail closed as unsupported. No automatic fallback, reinterpretation, migration, or best-effort parsing is selected.

## 8. Membership key layout

Exact membership key bytes are:

1. exact prefix bytes `/prw/registry/membership/` — 25 bytes;
2. big-endian `u16` major = `1`;
3. big-endian `u16` minor = `0`;
4. big-endian `u64` workspace UTF-8 byte length;
5. exact `WorkspaceId` UTF-8 bytes;
6. big-endian `u64` user UTF-8 byte length;
7. exact `UserId` UTF-8 bytes.

No delimiter parsing or Unicode normalization is permitted.

The exact identifier bytes are preserved. Delimiter-like characters, `/`, `:`, Unicode, and embedded NUL bytes remain data when admitted by the existing typed identifier constructor and selected bounds.

No trailing bytes are permitted.

## 9. Device key layout

Exact device key bytes are:

1. exact prefix bytes `/prw/registry/device/` — 21 bytes;
2. big-endian `u16` major = `1`;
3. big-endian `u16` minor = `0`;
4. big-endian `u64` device UTF-8 byte length;
5. exact `DeviceId` UTF-8 bytes.

No delimiter parsing or Unicode normalization is permitted.

No transport identity appears in the durable device key because transport identity is current mutable state of the exact logical device, not a second durable device namespace.

No trailing bytes are permitted.

## 10. Selected production persistence bounds

Exact core identifier constructors at C03e-IP reject only empty/whitespace-only identifiers and do not themselves impose a byte-length maximum.

Exact source:

`crates/prw-core/src/lib.rs`

Exact blob:

`665afdb5f2627a7d84f09b476302503e66e121e2`

The production authenticated-session canonical boundary already requires:

- each identifier: `1..=1024` UTF-8 bytes;
- public identity: `1..=256` bytes for the locked initial P-256 SPKI profile.

Exact source:

`crates/prw-control-plane/src/session_auth.rs`

Exact blob:

`1dbd06d8d9741844e4d8bbb235d27431921a1650`

C03e-IQ selects those existing protected-operation compatibility bounds for durable production registry key/value encoding:

- `WorkspaceId`: `1..=1024` UTF-8 bytes;
- `UserId`: `1..=1024` UTF-8 bytes;
- `DeviceId`: `1..=1024` UTF-8 bytes;
- public identity bytes: `1..=256` bytes.

This does not alter `prw-core` constructors. It selects what may be represented as canonical production durable registry state for the existing authenticated production path.

Any out-of-bounds typed value fails before provider mutation.

## 11. Exact maximum selected key sizes

Under the selected bounds:

Membership key maximum:

`25 + 4 + 8 + 1024 + 8 + 1024 = 2093 bytes`

Device key maximum:

`21 + 4 + 8 + 1024 = 1057 bytes`

Encoding uses checked size arithmetic. Overflow or a non-canonical length fails closed before provider I/O.

## 12. Membership value format

C03e-IQ selects one canonical provider-neutral membership value format.

Magic:

`PRWM`

Initial value version:

- major `1`;
- minor `0`.

Exact bytes, in order:

1. 4-byte magic `PRWM`;
2. big-endian `u16` major = `1`;
3. big-endian `u16` minor = `0`;
4. big-endian `u64` total record byte length;
5. big-endian `u64` workspace byte length;
6. big-endian `u64` user byte length;
7. big-endian `u16` role code;
8. big-endian `u16` membership lifecycle code;
9. big-endian `u32` reserved = `0`;
10. exact workspace UTF-8 bytes;
11. exact user UTF-8 bytes.

Fixed membership value bytes before variable identifiers:

`40`

Maximum selected membership value length:

`40 + 1024 + 1024 = 2088 bytes`

The declared total length MUST equal the canonical computed length and the exact byte-slice length.

No trailing bytes are permitted.

## 13. Membership enum codes

Selected `WorkspaceRole` codes:

- `Owner` = `1`;
- `Admin` = `2`;
- `Member` = `3`.

Selected `MembershipLifecycle` codes:

- `Active` = `1`;
- `Suspended` = `2`;
- `Removed` = `3`.

All other codes are invalid/unsupported and fail closed.

The reserved field MUST be zero.

Phase 130 currently exposes no role-transition operation; C03e-IQ therefore treats the stored role as immutable under the current mutation set. A later role-change feature requires its own semantic transition contract.

## 14. Device value format

C03e-IQ selects one canonical provider-neutral device value format.

Magic:

`PRWD`

Initial value version:

- major `1`;
- minor `0`.

Exact bytes, in order:

1. 4-byte magic `PRWD`;
2. big-endian `u16` major = `1`;
3. big-endian `u16` minor = `0`;
4. big-endian `u64` total record byte length;
5. big-endian `u64` workspace byte length;
6. big-endian `u64` user byte length;
7. big-endian `u64` device byte length;
8. big-endian `u64` public-identity byte length;
9. big-endian `u16` public-identity algorithm code;
10. big-endian `u16` public-key encoding code;
11. big-endian `u16` device lifecycle code;
12. big-endian `u16` transport-presence code;
13. big-endian `u32` reserved = `0`;
14. exact 32-byte transport slot;
15. exact workspace UTF-8 bytes;
16. exact user UTF-8 bytes;
17. exact device UTF-8 bytes;
18. exact public-identity bytes.

Fixed device value bytes before variable identifiers/public identity:

`92`

Maximum selected device value length:

`92 + 1024 + 1024 + 1024 + 256 = 3420 bytes`

The declared total length MUST equal the canonical computed length and exact byte-slice length.

No trailing bytes are permitted.

## 15. Device identity profile codes

The existing locked initial public-identity profile remains authoritative.

Selected algorithm code:

- `DeviceIdentityAlgorithm::EcdsaP256Sha256` = `1`.

Selected public-key encoding code:

- `DeviceIdentityPublicKeyEncoding::SubjectPublicKeyInfoDer` = `1`.

These are the same code values used by the existing canonical Phase 128 session-authentication message.

Unknown algorithm or encoding codes fail closed.

C03e-IQ does not add another device identity algorithm or encoding.

## 16. Device lifecycle and transport codes

A durable registered-device record may encode only the Phase 130 registered lifecycle states:

- `Enrolled` = `1`;
- `Revoked` = `2`.

`PendingEnrollment` is invalid in a durable registered-device record because Phase 130 registration rejects non-enrolled devices before insertion.

Selected transport-presence codes:

- `0` = no current transport identity bound;
- `1` = current transport identity present.

When transport presence is `0`:

- the 32-byte transport slot MUST be all zero.

When transport presence is `1`:

- the 32-byte transport slot MUST construct an existing valid non-zero `TransportIdentity`.

Exact transport type source:

`crates/prw-connectivity/src/lib.rs`

Exact blob:

`fefb8459e73bd0a92e87e8d7600282c7f515b159`

Any other transport-presence code or non-canonical zero/non-zero combination fails closed.

Revocation does not clear a previously bound transport identity. The durable record preserves the complete last current device record while lifecycle controls participation.

## 17. Key/value binding law

Canonical decoding is not sufficient by itself.

For every authoritative provider observation, the semantic owner MUST verify exact key/value binding.

### Membership binding

The decoded `(WorkspaceId, UserId)` in the membership value MUST equal the exact pair decoded from the membership key and the exact pair requested by the caller.

### Device binding

The decoded `DeviceId` in the device value MUST equal:

- the exact `DeviceId` decoded from the device key;
- the exact `DeviceId` requested by the caller.

The complete immutable device tuple in the value is semantic authority and may not be substituted from caller/request fields.

Any key/value/request mismatch fails closed as unavailable/invalid durable authority state rather than being repaired or rebound.

## 18. Canonical codec precedent

Exact existing reachability key codec:

`crates/prw-remote-bridge/src/reachability_durable_snapshot_key_codec.rs`

Exact blob:

`12b65ccefd089505266658086af70092945d8f7f`

Exact existing reachability value codec:

`crates/prw-remote-bridge/src/reachability_durable_snapshot_codec.rs`

Exact blob:

`3c66fcfa35c1104f5762c1431ea6200eb9daaf4b`

C03e-IQ reuses their established design laws, not their domain types:

- binary exact prefixes;
- explicit major/minor versions;
- checked lengths;
- exact UTF-8 preservation;
- explicit enum codes;
- reserved fields required to be zero;
- exact total-length validation;
- typed constructor validation;
- rejection of trailing bytes;
- semantic key/value binding above provider execution.

## 19. Provider observation law

A future control-plane raw registry etcd executor may return only bounded provider evidence such as:

- exact requested/observed key bytes;
- exact observed raw value bytes;
- positive `mod_revision`;
- definitive mutation commit;
- definitive compare failure plus authoritative failure-branch observation(s);
- read unavailable;
- mutation indeterminate;
- invalid provider response shape.

It MUST NOT decode `PRWM`/`PRWD` records or classify workspace/device/transport semantics.

The `prw-registry` semantic layer remains responsible for canonical decode, key/value binding, transition validation, and semantic failure classification.

## 20. Single-key authoritative read law

Single-record currentness operations use a default-linearizable exact-key Get.

This applies at least to:

- exact membership lookup when no paired device snapshot is needed;
- exact device lookup;
- production current `DeviceId` + current transport lookup for later `PeerConnectivityIdentity` construction;
- current transport validation;
- mutation pre-observation for one-record membership/device transitions.

A serializable/stale read, Watch cache, prefix scan, process cache, test fixture, or old session snapshot is not authoritative current registry state.

An exact-key Get must return either zero or one exact matching key. More than one result or another key is provider-invalid state and fails closed.

## 21. Paired membership/device snapshot law

Authenticated-session current-registry validation spans two independently keyed records.

C03e-IQ selects one authoritative paired-read operation implemented as one etcd transaction containing exact membership-key Get and exact device-key Get operations.

The semantic layer MUST receive and validate both observations as one transactionally consistent provider snapshot before admitting the session as current.

Two unrelated sequential reads are not selected as the production validation primitive because membership could change between observations and Phase 130 validates both under one current registry view.

The paired read performs no writes.

## 22. Initial create-if-absent law

Phase 130 membership insertion and device registration are non-overwriting.

C03e-IQ selects an atomic exact-key absence compare for initial creation.

The provider-level absence guard is:

`exact key version == 0`

or the provider API's exact semantic equivalent proving that the key does not exist at transaction evaluation.

On compare success:

- exactly one canonical `Put` for that record is permitted, except the cross-record device-registration transaction described separately below.

On compare failure:

- the failure branch MUST return an authoritative exact-key Get of the current record needed for semantic classification.

A definitive create compare failure is not converted to success when the existing record happens to equal the proposed record. Phase 130 duplicate creation remains a duplicate/terminal semantic event, not idempotent overwrite.

No unconditional Put is allowed for initial record creation.

## 23. Single-record update CAS law

All existing-record membership/device transitions use the exact observed provider state as a conjunctive guard.

Required compares for the exact key:

1. `mod_revision == observed positive mod_revision`;
2. `value == exact observed raw canonical value bytes`.

On compare success:

- exactly one complete canonical replacement `Put` commits.

On compare failure:

- the failure branch MUST return the authoritative exact-key current observation.

This law applies to at least:

- membership `Active -> Suspended`;
- membership `Active|Suspended -> Removed`;
- first transport bind from exact enrolled/unbound device record;
- transport rotation from exact enrolled/bound expected-current device record;
- device `Enrolled -> Revoked`.

No partial-field provider update is selected. Replacement writes the complete canonical semantic record.

## 24. Membership add semantics

Before provider mutation the semantic layer MUST validate:

- exact identifiers are representable by the selected canonical format;
- role is one selected Phase 130 role;
- proposed lifecycle is exactly `Active`.

The provider transaction compares exact membership key absence and commits the canonical membership value only on absence.

If the key exists, semantic classification returns the existing Phase 130 duplicate-membership meaning rather than replacing or resurrecting it.

A retained `Removed` membership therefore blocks silent re-add under the same exact key.

## 25. Membership suspension/removal semantics

The semantic layer first obtains one authoritative exact current membership observation and validates key/value binding.

For suspension:

- only `Active -> Suspended` is valid;
- already `Suspended` remains an invalid transition;
- `Removed` remains terminal.

For removal:

- `Active -> Removed` is valid;
- `Suspended -> Removed` is valid;
- `Removed` remains terminal.

The complete successor membership value is written under the selected dual CAS.

If CAS fails and authoritative failure state is returned, semantic classification is performed from that exact state. The provider layer does not guess the Phase 130 error.

## 26. Device registration cross-record atomicity

Phase 130 device registration requires an exact active membership and an absent `DeviceId` in one logical mutation decision.

C03e-IQ selects one bounded multi-key etcd transaction for production durable device registration.

Before the mutation, the semantic layer obtains and validates an authoritative active membership observation for the binding's exact `(WorkspaceId, UserId)`.

The transaction compares conjunctively:

1. membership key `mod_revision == observed membership mod_revision`;
2. membership key `value == exact observed active membership bytes`;
3. device key `version == 0` proving exact `DeviceId` absence.

On success:

- exactly one canonical unbound `Enrolled` device record is Put under the exact device key.

The membership record is not rewritten.

On compare failure:

- the failure branch MUST return authoritative exact Gets for both the membership key and device key so the semantic owner can distinguish missing/inactive/changed membership, duplicate device, malformed state, or an ambiguous provider condition.

This prevents a stale active membership observation from authorizing device registration after concurrent suspension/removal.

## 27. Device initial record law

A newly registered durable device record MUST contain:

- exact immutable workspace/user/device/public-identity binding supplied by the validated enrolled binding;
- lifecycle exactly `Enrolled`;
- transport presence `0`;
- all-zero transport slot.

A `PendingEnrollment` or `Revoked` proposed initial durable device record is rejected before provider mutation.

An existing device key is never rebound to another immutable tuple.

## 28. Initial transport bind law

The semantic layer obtains one exact authoritative device observation.

Bind is valid only when:

- key/value/requested `DeviceId` bind exactly;
- current device lifecycle is `Enrolled`;
- transport presence is `0`;
- proposed transport identity is a valid non-zero existing `TransportIdentity`.

The complete successor device record differs only by setting:

- transport presence to `1`;
- exact 32-byte transport slot to the proposed identity.

The immutable tuple and device lifecycle are preserved exactly.

The successor is committed under the selected dual CAS.

A second initial bind fails semantically rather than replacing current transport identity.

## 29. Transport rotation law

Rotation is valid only when:

- exact device record exists and is canonically bound;
- current lifecycle is `Enrolled`;
- current transport is present;
- exact current transport equals the caller's expected current identity;
- replacement transport is valid and differs from expected current.

The complete successor device value preserves the immutable tuple/lifecycle and changes only the current transport slot.

The replacement is committed under the exact observed `mod_revision + value` dual CAS.

A stale expected identity or a concurrent device/transport change fails closed and is never last-write-wins.

## 30. Device revocation law

Revocation is valid only from exact current `Enrolled` device state.

The successor:

- preserves immutable workspace/user/device/public-identity tuple;
- preserves the optional current transport slot exactly;
- changes only device lifecycle to `Revoked`.

The successor is committed under the selected dual CAS.

`PendingEnrollment` is invalid durable state; repeated `Revoked` remains terminal and is not a new successful authorization event.

No provider Delete is selected for revocation.

## 31. Current transport lookup for production peer provenance

The later production `PeerConnectivityIdentity` authority lookup for one known logical `DeviceId` uses exactly one authoritative device-key read.

Before returning `PeerConnectivityIdentity`, the semantic layer MUST prove:

- exact key/value/request `DeviceId` binding;
- canonical supported device value;
- current device lifecycle exactly `Enrolled`;
- transport presence exactly `1`;
- valid non-zero current `TransportIdentity`.

Only then may it construct:

`PeerConnectivityIdentity::new(exact_device_id, exact_current_transport_identity)`

No membership record is required merely to prove this exact current device/transport pair because Phase 130 `validate_transport_identity` is device-state scoped and C03e-IN selected current same-device binding as the peer-provenance requirement.

Membership remains separately authoritative for protected authenticated-session validation.

## 32. Compare-failure classification law

A provider compare failure is definitive only about non-commit of that transaction branch; semantic meaning depends on authoritative failure observation(s).

The semantic layer MUST:

1. validate exact failure key(s);
2. validate positive provider revision evidence when a record exists;
3. canonical-decode returned value(s);
4. validate key/value/request binding;
5. classify the Phase 130 transition from that exact current state.

If a compare fails while the current raw value is byte-identical to the pre-mutation observed value but `mod_revision` changed, C03e-IQ selects fail-closed ambiguous/currentness-conflict behavior rather than silently retrying or claiming success. An intervening provider history cannot be ignored merely because semantic bytes returned to an earlier value.

If current state equals the intended successor after a failed or indeterminate attempt, the operation is not automatically declared successful unless a later separately selected reconciliation contract proves that exact operation identity/outcome. Phase 130 repeated transitions often have different semantics from first execution.

## 33. Provider mutation indeterminacy law

A mutation RPC transport/provider failure that does not return a definitive transaction response is:

`INDETERMINATE`

It is never converted directly to:

- success;
- stale expected;
- duplicate;
- revoked;
- removed;
- already-bound.

No hidden automatic mutation retry is selected by C03e-IQ.

A later retry/reconciliation checkpoint may be required if production operation semantics need safe automatic recovery from an indeterminate outcome. Until then, callers fail closed and require authoritative re-observation before any new mutation decision.

## 34. Malformed durable-state failure law

The semantic owner fails closed on at least:

- wrong key prefix;
- unsupported key major/minor version;
- invalid/truncated/overflowed key lengths;
- invalid identifier UTF-8;
- identifier outside selected persistence bounds;
- non-canonical trailing key bytes;
- wrong value magic;
- unsupported value major/minor version;
- invalid total record length;
- non-zero reserved field;
- invalid role/lifecycle/transport-presence code;
- `PendingEnrollment` encoded as registered device;
- invalid identity algorithm/encoding code;
- empty/oversized public identity;
- absent transport with non-zero transport slot;
- present transport with all-zero/invalid transport identity;
- key/value identifier mismatch;
- requested/key/value identifier mismatch;
- impossible exact-key cardinality;
- non-positive provider `mod_revision` for an existing observation;
- unexpected transaction branch/result shape.

Malformed authoritative bytes are not repaired in place, normalized, skipped, replaced from caller input, or interpreted as absent.

## 35. Unknown version / migration law

C03e-IQ selects exact v1.0 key and value decoding only.

Unknown version means unsupported durable state and fails closed.

C03e-IQ does not select:

- lazy migration;
- dual-read old/new formats;
- background conversion;
- automatic rewrite after read;
- provider-side schema migration;
- compatibility fallback.

Any future format upgrade requires a separate version/migration checkpoint with exact upgrade and rollback semantics.

## 36. No Delete / tombstone/history authority

The initial durable registry model is current-state only.

C03e-IQ does not select provider Delete for membership or device lifecycle transitions.

Terminal state is represented by retained canonical records:

- membership `Removed`;
- device `Revoked`.

No append-only registry history ledger, audit-event stream, tombstone table, or historical query API is selected.

Provider MVCC history is not promoted into PRW semantic history authority by this contract.

## 37. No Watch / lease / TTL / scan authority

C03e-IQ does not select:

- Watch for authorization/currentness;
- lease ownership for membership/device records;
- TTL expiration for removal/revocation;
- prefix scans for registry discovery;
- background cache synchronization;
- stale local cache as a production authority fallback.

All selected protected/currentness decisions use exact current KV operations or exact bounded etcd transactions.

## 38. Capacity bounds remain source/disposable, not provider-global

The Phase 130 contract explicitly describes `4096` membership and `4096` device limits as initial hard bounds for one bounded in-memory registry instance.

It separately requires future durable implementations to preserve equivalent:

- uniqueness;
- immutable binding;
- terminal revocation/removal;
- compare-before-mutate semantics.

C03e-IQ therefore does not invent a provider-global count record, prefix scan, approximate count, or cross-cluster quota to reinterpret the source/disposable per-instance capacities.

The existing `WorkspaceDeviceRegistry` retains its exact `4096` in-memory limits unchanged.

A future production durable population quota, if required, must be selected separately with exact counting/atomicity semantics. No durable adapter may fabricate `MembershipCapacity` or `DeviceCapacity` from an approximate provider count.

## 39. Provider transaction capability required by the selected model

The exact C03e-IP manifest pins:

`etcd-client = 0.19.0`

Exact manifest blob:

`acf008393686c10f5b9d63605399a608737973f7`

C03e-IQ requires only bounded KV transaction capabilities consistent with the selected provider family:

- exact-key Get;
- transaction compare on exact key version for absence;
- transaction compare on exact `mod_revision`;
- transaction compare on exact raw value;
- multiple conjunctive compares across exact membership/device keys for registration;
- bounded exact-key Get operations in transaction branches;
- complete exact-value Put.

It does not require Watch, Lease, Lock, Election, prefix scans, Delete, or a broad raw `Client` at the semantic boundary.

## 40. Dependency and ownership law preserved

C03e-IQ preserves C03e-IP ownership exactly.

### `prw-registry`

Owns:

- `PRWM` / `PRWD` canonical semantic key/value codecs;
- selected semantic record types or equivalent typed carriers;
- key/value/request binding;
- Phase 130 transition validation;
- semantic classification of authoritative provider observations.

### `prw-control-plane`

Owns only provider-specific raw etcd execution needed by the selected operations.

It MUST NOT depend on `prw-registry` or decode registry semantic formats.

### `prw-agent`

Owns later production composition only and does not own registry records/codecs/provider logic.

No reverse dependency `prw-control-plane -> prw-registry`, new shared crate, or copied registry semantics is selected.

## 41. First source-materialization ceiling

After C03e-IQ closes, the next separately gated checkpoint may materialize only the provider-neutral registry codec layer first.

Selected first Rust source ceiling:

1. new file:
   `crates/prw-registry/src/durable_registry_codec.rs`
2. minimal module declaration/export only in:
   `crates/prw-registry/src/lib.rs`

That first source checkpoint may implement/test only:

- selected membership/device key codecs;
- selected membership/device value codecs;
- selected bounds/version/magic/enum mappings;
- canonical decode validation;
- key/value binding helpers that require no provider I/O.

It may not yet add:

- etcd executor code;
- `KvClient` use in `prw-registry`;
- Cargo dependencies;
- provider connection/bootstrap;
- semantic durable store/adapters performing I/O;
- production records;
- environment/configuration;
- runtime activation.

The path `crates/prw-registry/src/durable_registry_codec.rs` was absent at the exact C03e-IP predecessor and is therefore a new bounded source seam rather than an overwrite of existing implementation.

## 42. Focused first source test matrix

The codec-only source checkpoint must prove at least:

### Keys

- exact v1.0 membership key bytes and roundtrip;
- exact v1.0 device key bytes and roundtrip;
- delimiter-like Unicode and embedded NUL identifier bytes roundtrip without normalization;
- distinct exact identities produce distinct keys;
- wrong prefix rejected;
- wrong major/minor rejected;
- truncated/overflowed length rejected;
- invalid UTF-8 rejected;
- over-1024-byte identifiers rejected;
- trailing bytes rejected.

### Membership values

- all selected role/lifecycle combinations canonically roundtrip;
- exact total length enforced;
- wrong magic/version/reserved/code rejected;
- workspace/user bounds enforced;
- key/value binding mismatch rejected;
- removed record remains representable and retained.

### Device values

- enrolled/unbound roundtrip;
- enrolled/bound roundtrip;
- revoked/unbound and revoked/bound roundtrip;
- `PendingEnrollment` rejected;
- exact immutable workspace/user/device/public-identity tuple roundtrip;
- public identity `1..=256` enforced;
- unsupported algorithm/encoding rejected;
- absent transport with non-zero bytes rejected;
- present all-zero transport rejected;
- invalid transport-presence code rejected;
- exact total length/reserved/trailing bytes enforced;
- key/value/request `DeviceId` mismatch rejected.

No provider endpoint is required for these tests.

## 43. Later provider source prerequisite

After codec materialization is separately validated, another checkpoint must select/materialize the raw control-plane registry etcd executor operations needed for:

- exact linearizable single-key read;
- paired exact-key transactional read;
- exact create-if-absent;
- exact dual-CAS update;
- exact active-membership + device-absence registration transaction;
- authoritative failure-branch observation decoding at provider shape only.

That checkpoint must remain independent of endpoint/credential/bootstrap activation.

C03e-IQ does not pre-authorize its Rust type names or implementation details beyond the provider mechanics selected here.

## 44. Later semantic adapter prerequisite

After raw provider execution is separately materialized and validated, another checkpoint must compose it inside `prw-registry` so provider observations are classified through Phase 130 semantics.

That future checkpoint must preserve:

- no raw provider leakage;
- canonical key/value binding;
- provider unavailable/indeterminate errors distinct from Phase 130 semantic errors;
- no automatic mutation retry unless separately reconciled;
- no production data creation.

C03e-IQ does not materialize or name that final adapter type.

## 45. Production bootstrap/population remains blocked

C03e-IQ does not make production peer population executable.

Still blocked after C03e-IQ closure are at least:

- codec source materialization;
- raw registry etcd executor materialization;
- semantic durable registry adapter materialization;
- provider bootstrap/security/credential selection;
- production registry population/migration provenance;
- exact current-device/current-transport lookup composition into Agent bootstrap;
- production testing deployment and operational validation.

No empty registry, source/disposable map, environment value, request/session state, certificate alone, or test fixture may be treated as production durable registry population.

## 46. Security and operational invariants

C03e-IQ preserves:

- user identity != device identity;
- logical `DeviceId` != `TransportIdentity`;
- transport identity remains separately rotatable current state;
- IP/port remains reachability, not identity;
- immutable public identity cannot be rebound through transport rotation;
- removed membership and revoked device remain terminal participation states;
- role metadata remains non-capability metadata;
- provider mechanics do not become semantic authority;
- malformed/ambiguous/unavailable durable state fails closed.

No private key, secret, credential, certificate, trust root, provider resource, database migration, registry record, systemd unit/package, network route/firewall/DNS/TUN/TAP state, listener/readiness state, runtime state, or deployment state is introduced.

## 47. Exact-head validation requirement

C03e-IQ may be semantically closed only after the exact final C03e-IQ head proves:

- predecessor/base/merge-base is exact C03e-IP head `65fdb7659263c4963c16d3a1b74c728a0805aa2e`;
- branch is ahead only by the bounded docs-only selection commit;
- exactly one contract path changed;
- no Rust/Kotlin/Cargo/lockfile/workflow/runtime/security path changed;
- automatically triggered Rust validation passes on the exact final head;
- path-filtered workflows are recorded accurately;
- immutable canonical Drive evidence is written and raw-read back;
- PR remains draft/open/unmerged.

No validation result from another head may be inherited.

## 48. Repository and deployment non-authorization

C03e-IQ does not authorize or perform:

- repository visibility/configuration mutation;
- merge;
- branch deletion;
- history rewrite;
- production deployment;
- service restart;
- systemd unit/package mutation;
- credential creation/replacement;
- certificate issuance/installation;
- trust/RBAC/auth mutation;
- provider resource creation;
- production registry write;
- migration/schema population;
- firewall/routing/DNS/TUN/TAP mutation;
- listener/readiness activation;
- production-state mutation.

Repository visibility remains whatever the exact repository metadata reports; C03e-IQ does not change it.

## 49. Closure meaning

If exact-head validation and immutable evidence recording pass, C03e-IQ closure means only:

`PRODUCTION_DURABLE_REGISTRY_RECORD_KEY_CAS_SEMANTICS_SELECTED`

Specifically it means:

- membership and device are selected as separate durable record kinds matching Phase 130 keys;
- device lifecycle + immutable tuple + optional current transport are selected in one atomic device value;
- exact `/prw/registry/membership/` and `/prw/registry/device/` key namespaces are selected;
- canonical binary key/value v1.0 formats, bounds and enum/profile codes are selected;
- exact key/value/request binding is mandatory;
- single-key linearizable reads, paired transactional reads, absence create, dual CAS update and cross-record registration transaction laws are selected;
- malformed/unknown/ambiguous/provider-indeterminate state fails closed;
- source/disposable 4096 per-instance capacities are not silently promoted to a provider-global quota;
- first source materialization is bounded to provider-neutral registry codecs only.

It does not mean codecs exist, an etcd registry executor exists, production registry records exist, credentials/endpoints are selected, production peer lookup is wired, runtime networking is active, or deployment is complete.
