# Phase 152 C03e-IU — Production Durable Registry Semantic Adapter Boundary Selection

Status: `STAGED_SELECTION`

Gate on closure:
`C03E_IU_PRODUCTION_DURABLE_REGISTRY_SEMANTIC_ADAPTER_BOUNDARY_SELECTED`

Canonical closure token on successful exact-head validation and evidence recording:
`CLOSED_PRODUCTION_DURABLE_REGISTRY_SEMANTIC_ADAPTER_BOUNDARY_SELECTION`

## 1. Scope

C03e-IU is the documentation-only prerequisite after closed C03e-IT.

C03e-IT materialized the bounded raw etcd executor in `prw-control-plane` and deliberately left semantic durable registry I/O in `prw-registry` separately gated. C03e-IU now selects only the semantic adapter boundary that may consume that raw executor while preserving Phase 130 registry semantics, canonical C03e-IQ codecs, the C03e-IS ownership split, and the C03e-IT provider-mechanics boundary.

Exact predecessor C03e-IT head:

`2b75dd56022e179e480d73be7964eebf6400602d`

Exact predecessor C03e-IT tree:

`11a6e91f3d426b4c87cedf14d28e9b6b99f7775d`

C03e-IU changes documentation only. It does not materialize Rust/Kotlin/Cargo/lockfile/workflow/runtime source, connect to etcd, select endpoints, create credentials, configure TLS/auth/RBAC, create or mutate production registry records, migrate registry state, populate production peer identity, activate Agent/bootstrap/runtime behavior, or deploy anything.

## 2. Exact predecessor provider executor authority

Exact materialized raw provider source:

`crates/prw-control-plane/src/durable_registry_etcd.rs`

Exact C03e-IT blob:

`b8339c075f28d290889551aad09f29f2462700ef`

Exact minimal control-plane module export:

`crates/prw-control-plane/src/lib.rs`

Exact C03e-IT blob:

`ec34a88989d7ba68c4db142d1e20e969c1db7683`

The raw executor already exposes the complete bounded provider-mechanics primitive set selected by C03e-IQ/C03e-IS:

- `linearizable_get` for one exact key;
- `linearizable_pair_get` for two exact keys in one transaction response;
- `create_if_absent` using exact key `version == 0`;
- `compare_and_put` using exact positive observed `mod_revision + exact observed raw value`;
- `register_device_if_membership_unchanged` using membership revision + exact membership value + device `version == 0`;
- structurally validated raw observations and compare-failure observations;
- `ReadUnavailable` versus `MutationIndeterminate` provider failure distinction.

That module does not decode `PRWM`/`PRWD`, classify registry lifecycle state, construct successor semantic records, or create `PeerConnectivityIdentity`.

C03e-IU preserves that boundary exactly.

## 3. Exact semantic codec authority

Exact canonical codec source:

`crates/prw-registry/src/durable_registry_codec.rs`

Exact retained blob:

`c1aafcc80dea3a6d06f11b0d50418e25e1437473`

Exact registry module export source:

`crates/prw-registry/src/lib.rs`

Exact retained blob:

`f76b5daa664d5272ed97c9f76c6792b105825eb0`

The codec module owns:

- canonical membership key encoding/decoding;
- canonical device key encoding/decoding;
- canonical `PRWM` membership value encoding/decoding;
- canonical `PRWD` device value encoding/decoding;
- exact v1.0 bounds/version/profile validation;
- exact membership key/value/request binding through `decode_bound_membership_record`;
- exact device key/value/request binding through `decode_bound_device_record`.

It deliberately performs no provider I/O or transaction planning.

C03e-IU does not move codec authority into `prw-control-plane` and does not duplicate those formats in another crate.

## 4. Phase 130 semantic authority remains unchanged

Exact Phase 130 source:

`crates/prw-registry/src/lib.rs`

Exact Phase 130 contract:

`contracts/DEVICE_REGISTRY_WORKSPACE_MEMBERSHIP_CONTRACT.md`

Exact contract blob:

`2faf8b014d43583e9180b907f94f40f093e5125b`

The durable adapter must preserve the existing semantic laws, including:

- membership key is exact `(WorkspaceId, UserId)`;
- membership lifecycle is `Active`, `Suspended`, or terminal `Removed`;
- only `Active` membership participates in current registry validation;
- role remains metadata and is not a capability grant;
- device key is exact `DeviceId`;
- device immutable tuple is workspace/user/device/canonical public identity;
- only `Enrolled` is registrable;
- `Revoked` is terminal for participation;
- a `DeviceId` may never be rebound;
- initial transport bind is non-overwriting;
- transport rotation is compare-before-mutate against the exact expected current identity;
- session validation revalidates current membership and current device state;
- repeated terminal transitions are not new successful authorization events.

The durable path must not silently reinterpret provider behavior as a different registry model.

## 5. C03e-IQ record/currentness law remains unchanged

Exact retained selection contract:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_IQ_PRODUCTION_DURABLE_REGISTRY_RECORD_KEY_CAS_SEMANTICS_SELECTION_STAGING.md`

C03e-IU preserves its exact selected durable model:

- membership and device are separate durable current-state record kinds;
- device lifecycle + immutable tuple + optional current transport occupy one complete device value;
- `/prw/registry/membership/` and `/prw/registry/device/` remain the exact authority-domain prefixes;
- canonical v1.0 `PRWM` / `PRWD` formats remain mandatory;
- authoritative single reads are default-linearizable exact-key reads;
- authenticated-session current validation uses one paired transactional membership/device read;
- initial create uses exact `version == 0` absence compare;
- existing-record mutation uses exact observed positive `mod_revision + exact observed raw value` dual CAS;
- device registration uses exact active-membership observation + device absence in one bounded transaction;
- compare-failure branches return authoritative exact-key observation(s);
- mutation ambiguity is never success;
- no Delete, Watch, Lease, TTL, prefix scan, range discovery, stale/serializable read, or cache authority is selected.

## 6. Exact dependency topology

Exact registry manifest:

`crates/prw-registry/Cargo.toml`

Exact C03e-IT blob:

`ec9215d9bad86ac9601e2f2d1bc0ed8461e724c1`

It already contains:

`prw-registry -> prw-control-plane`

and does not depend directly on `etcd-client`.

C03e-IU selects preservation of this topology.

The semantic adapter may depend on public bounded raw-executor types from `prw-control-plane`, but it MUST NOT:

- add direct `etcd-client` to `prw-registry`;
- accept or return `etcd_client::KvClient`;
- construct raw etcd transactions in `prw-registry`;
- expose endpoint/TLS/auth/RBAC/bootstrap provider objects through the semantic API;
- create `prw-control-plane -> prw-registry` reverse dependency;
- create a dependency cycle;
- create a new shared crate merely to bypass existing dependency direction.

No Cargo or lockfile change is selected for the first semantic-adapter source checkpoint.

## 7. Selected concrete semantic adapter seam

C03e-IU selects one concrete dormant semantic adapter in `prw-registry` over the already-materialized raw executor.

Selected semantic ownership pattern:

- `DurableRegistryEtcdExecutor` remains the raw provider executor owned by `prw-control-plane`;
- a later `prw-registry` concrete semantic store/adapter owns one `DurableRegistryEtcdExecutor` instance;
- adapter construction accepts the already-created raw executor and performs no network I/O;
- optional `into_inner`-style custody return may return the raw executor, never a raw `KvClient` directly;
- all canonical key construction, value construction, decode/binding and registry semantic classification happen in `prw-registry` before/after bounded raw executor calls.

The selected concrete source name for the first semantic materialization checkpoint is:

`crates/prw-registry/src/durable_registry_etcd_store.rs`

The selected concrete type name may be:

`DurableRegistryEtcdStore`

This naming identifies the concrete provider adapter without moving provider mechanics into the registry crate.

C03e-IU does not select a generic provider trait, dynamic dispatch layer, plugin system, mock-provider abstraction, or new crate. A future provider-neutral abstraction requires a separate architecture gate if it becomes necessary.

## 8. Selected public failure taxonomy

The semantic adapter must not collapse provider/currentness failures into Phase 130 semantic errors.

C03e-IU selects a provider-neutral durable-adapter failure surface with distinct categories equivalent to:

1. `Semantic(RegistryError)` — an exact Phase 130 semantic failure that can be proven from canonical authoritative current state;
2. `ReadUnavailable` — authoritative read could not be obtained;
3. `MutationIndeterminate` — provider mutation outcome was not definitive;
4. `InvalidAuthority` — provider response structure, canonical durable bytes, key/value/request binding, immutable-state invariant, or another authority invariant is invalid;
5. `CurrentnessConflict` — a definitive compare failure returned canonical current state but no exact Phase 130 semantic transition error can be safely inferred without retrying or ignoring revision/currentness movement.

Exact Rust variant/type spelling remains source-materialization detail, but these distinctions are mandatory.

The public semantic adapter API MUST NOT expose `etcd_client::Error` as its semantic contract and MUST NOT require callers to classify provider-specific errors.

`RegistryError::MembershipCapacity` and `RegistryError::DeviceCapacity` are source/disposable in-memory bounds and MUST NOT be fabricated by the durable adapter. C03e-IQ deliberately did not select a provider-global count/quota protocol.

## 9. Provider error mapping law

Raw executor results map upward as follows:

- `DurableRegistryEtcdError::ReadUnavailable` -> semantic adapter `ReadUnavailable`;
- provider-invalid structural results such as impossible cardinality, wrong key, non-positive revision, or unexpected transaction response shape -> `InvalidAuthority`;
- `DurableRegistryEtcdError::MutationIndeterminate` -> semantic adapter `MutationIndeterminate`;
- canonical codec/decode/binding failure -> `InvalidAuthority`.

A semantic adapter MUST NOT turn provider unavailability into `MembershipUnknown`, `DeviceUnknown`, duplicate, stale expected, removed, revoked, already-bound, or success.

## 10. Exact observation binding law

Every existing-record provider observation must be validated before semantic use.

For membership observations the adapter must:

1. construct the exact canonical membership key from the requested `(WorkspaceId, UserId)`;
2. require the raw observation key to be that exact key through the existing codec/binding helper;
3. decode the canonical membership value;
4. verify decoded key, decoded value and requested identifiers agree exactly;
5. retain the exact observed raw value bytes and positive `mod_revision` as the CAS pre-observation.

For device observations the adapter must perform the equivalent exact `DeviceId` binding and retain the exact observed raw value bytes + positive `mod_revision`.

Caller/request fields may not repair malformed authority bytes or replace fields from the durable value.

## 11. Semantic mutation pipeline

Every existing-record durable mutation uses the same selected sequence:

1. exact authoritative pre-read;
2. canonical decode and exact request binding;
3. Phase 130 semantic precondition validation;
4. construction of one complete semantic successor record preserving all immutable fields and all unrelated current fields;
5. canonical successor value encoding before provider mutation;
6. raw `compare_and_put` using the exact pre-read key, positive `mod_revision`, and exact raw pre-read value bytes;
7. on `Committed`, return semantic success;
8. on definitive `CompareFailed`, canonical-decode and reclassify only from the exact authoritative failure observation;
9. on provider mutation uncertainty, return `MutationIndeterminate` without retry or success inference.

No partial provider field update, blind Put, last-write-wins, mutation retransmission, or hidden loop is selected.

## 12. Membership lookup/add law

Exact membership lookup uses one `linearizable_get` of the canonical membership key.

- authoritative absence -> no membership / Phase 130 unknown when a membership is required;
- canonical bound record -> exact `WorkspaceMembership` semantic state;
- malformed or mismatched authority -> `InvalidAuthority`.

Membership add validates before provider mutation:

- exact identifiers are canonically representable;
- role is one existing selected role;
- proposed lifecycle is exactly `Active`.

It encodes the complete canonical `PRWM` record and calls `create_if_absent`.

- `Committed` -> success;
- definitive compare failure with one canonical bound existing membership -> `RegistryError::MembershipAlreadyExists`;
- compare failure with absent/malformed/impossible failure observation -> `InvalidAuthority`;
- mutation uncertainty -> `MutationIndeterminate`.

A retained `Removed` membership remains an existing duplicate and is never silently resurrected.

## 13. Membership suspension/removal law

Suspension precondition:

- absence -> `MembershipUnknown`;
- `Active` -> eligible for `Suspended` successor;
- `Suspended` -> `InvalidMembershipTransition`;
- `Removed` -> `MembershipRemoved`.

Removal precondition:

- absence -> `MembershipUnknown`;
- `Active` or `Suspended` -> eligible for `Removed` successor;
- `Removed` -> `MembershipRemoved`.

The successor preserves exact workspace/user/role and changes only lifecycle.

After a definitive CAS compare failure, the adapter must validate the failure observation before classification.

If current canonical state now proves a Phase 130 terminal/ineligible condition, that semantic error may be returned. If the state remains semantically eligible but the compare failed because provider currentness/revision moved, the adapter returns `CurrentnessConflict`; it MUST NOT silently retry the transition.

A byte-identical value at a different revision remains `CurrentnessConflict`, not success and not an authorization to retry automatically.

## 14. Device registration law

Device registration preserves the exact Phase 130 precondition order:

1. supplied `DeviceIdentityBinding.lifecycle` must be exactly `Enrolled`, otherwise `DeviceNotEnrolled` before provider mutation;
2. exact membership must exist;
3. exact membership must be `Active`;
4. device registration then requires exact `DeviceId` absence.

The adapter first performs an authoritative membership read and canonical binding check. It builds the canonical initial device record with:

- exact immutable workspace/user/device/public identity from the supplied enrolled binding;
- lifecycle `Enrolled`;
- no current transport identity.

It then calls `register_device_if_membership_unchanged` using the exact observed membership key/revision/raw bytes plus exact canonical device key/value.

On definitive compare failure the returned membership/device pair is one semantic classification snapshot.

Classification order is:

1. validate provider shape and canonical binding of every returned existing record;
2. missing membership -> `MembershipUnknown`;
3. inactive membership -> `MembershipNotActive`;
4. if membership remains active but its exact provider currentness no longer matches the pre-observation without a Phase 130 semantic explanation -> `CurrentnessConflict`;
5. canonical existing device at the exact requested `DeviceId` -> `DeviceAlreadyExists`;
6. if membership remains exact and device remains absent despite definitive compare failure -> `InvalidAuthority` or `CurrentnessConflict`, never silent retry/success.

An existing canonical device record is a duplicate even when its immutable tuple happens to equal the proposed tuple. Initial registration is non-idempotent under Phase 130 semantics.

## 15. Initial transport bind law

The adapter reads one exact current device record.

Preconditions:

- absence -> `DeviceUnknown`;
- lifecycle other than `Enrolled` -> `DeviceRevoked` for the registered durable states;
- current transport already present -> `TransportIdentityAlreadyBound`;
- replacement transport must already be a valid non-zero `TransportIdentity` typed value.

The successor preserves the complete immutable tuple and lifecycle and changes only the optional current transport from absent to the supplied identity.

It is committed with exact observed revision + raw value dual CAS.

On compare failure:

- absence -> `DeviceUnknown`;
- current revoked state -> `DeviceRevoked`;
- current transport present -> `TransportIdentityAlreadyBound`;
- semantically still eligible enrolled/unbound state at moved currentness -> `CurrentnessConflict`;
- malformed/binding/immutable-state violation -> `InvalidAuthority`.

No second initial bind becomes rotation implicitly.

## 16. Transport rotation law

Before mutation the adapter requires:

- exact current device exists;
- lifecycle is `Enrolled`;
- current transport is present;
- current transport equals caller `expected_current` exactly;
- replacement differs from `expected_current`.

The existing Phase 130 semantic failures remain:

- absence -> `DeviceUnknown`;
- revoked -> `DeviceRevoked`;
- absent transport -> `TransportIdentityMissing`;
- stale/mismatched expected identity -> `TransportIdentityMismatch`;
- unchanged replacement -> `TransportIdentityUnchanged` before provider mutation.

The successor preserves immutable tuple + lifecycle and changes only current transport.

After definitive CAS compare failure, the adapter reclassifies the authoritative current device:

- current semantic stale/mismatch/revocation/missing state -> corresponding Phase 130 error;
- still-eligible exact expected transport at moved currentness -> `CurrentnessConflict`;
- immutable tuple changed relative to the valid pre-observation -> `InvalidAuthority`;
- malformed/binding failure -> `InvalidAuthority`.

No last-write-wins or retry is selected.

## 17. Device revocation law

The adapter reads and canonically binds one exact current device record.

- absence -> `DeviceUnknown`;
- `Enrolled` -> eligible;
- already `Revoked` -> `DeviceRevoked`;
- `PendingEnrollment` is invalid canonical durable registered state and therefore `InvalidAuthority`, not a normal durable transition state.

The successor preserves exact immutable tuple and optional current transport and changes only lifecycle to `Revoked`.

The successor is committed under exact dual CAS.

A compare-failure observation proving current `Revoked` may return `DeviceRevoked`. A canonical still-enrolled state at moved currentness returns `CurrentnessConflict`; malformed or immutable-state violation returns `InvalidAuthority`.

No Delete is selected.

## 18. Device/current transport read law

Exact device lookup uses one canonical device key and one `linearizable_get`.

A later semantic helper used for production peer provenance may return the exact current transport only after proving from that one current device observation:

- exact requested/key/value `DeviceId` binding;
- canonical supported `PRWD` record;
- lifecycle exactly `Enrolled`;
- transport exactly present and valid.

Semantic failures remain:

- absence -> `DeviceUnknown`;
- revoked -> `DeviceRevoked`;
- no current transport -> `TransportIdentityMissing`.

C03e-IU does not authorize Agent production `PeerConnectivityIdentity` population. The later composition checkpoint may consume this semantically proven current transport and construct peer identity without re-reading provider state.

## 19. Current transport validation law

`validate_transport_identity` equivalent durable behavior uses one authoritative current device read and canonical binding.

It succeeds only when:

- exact device exists;
- current device lifecycle is `Enrolled`;
- current transport is present;
- presented transport equals current transport exactly.

It preserves the existing Phase 130 failure meanings `DeviceUnknown`, `DeviceRevoked`, `TransportIdentityMissing`, and `TransportIdentityMismatch`.

No stale local cache, authenticated-session snapshot, endpoint, address, or expected-device hint may substitute for the current durable read.

## 20. Authenticated-session validation law

The durable adapter must use exactly one raw `linearizable_pair_get` containing:

1. the exact canonical membership key from the authenticated session workspace/user;
2. the exact canonical device key from the authenticated session device.

Both raw observations must come from that one transaction response.

The semantic adapter then preserves Phase 130 validation order:

1. missing membership -> `MembershipUnknown`;
2. inactive membership -> `MembershipNotActive`;
3. missing device -> `DeviceUnknown`;
4. non-enrolled/revoked device -> `DeviceRevoked`;
5. registered workspace/user/device/public identity must equal the authenticated session snapshot exactly, otherwise `SessionBindingMismatch`;
6. on success return the existing `RegistryValidatedPrincipal` identity + role snapshot only.

Canonical decode/binding failure of either record is `InvalidAuthority`, not `MembershipUnknown`, `DeviceUnknown`, or a repaired session binding.

No two sequential independent Gets are permitted for this production current-registry validation primitive.

## 21. Immutable tuple integrity law

The device immutable tuple is semantic authority:

- workspace;
- user;
- `DeviceId`;
- canonical public identity.

All adapter-created successors must preserve it byte-for-semantic-value exactly.

If a definitive compare-failure observation is canonical for the requested `DeviceId` but changes an immutable tuple field relative to the previously valid current observation, C03e-IU selects `InvalidAuthority` rather than retry, rebinding, or treating the changed tuple as an ordinary concurrent lifecycle/transport transition.

No caller field may overwrite durable immutable tuple authority after registration.

## 22. Compare-failure currentness law

A definitive compare failure proves only that the proposed success branch did not commit.

The semantic adapter MUST NOT claim success merely because:

- current bytes equal intended successor bytes;
- current bytes equal the prior observed bytes;
- current semantic lifecycle/transport now matches the requested end state;
- another writer may have performed an equivalent transition.

For existing-record updates, if the authoritative failure record is canonical and proves a specific Phase 130 semantic error, that exact error may be returned.

If the record remains semantically eligible for the original transition but the exact CAS failed, the result is `CurrentnessConflict`.

This includes exact byte-identical current value at a different `mod_revision`.

C03e-IU does not select operation identifiers, reconciliation markers, mutation journals, or idempotent-success inference.

## 23. Mutation indeterminacy law

If the raw executor returns `MutationIndeterminate`, the semantic adapter returns `MutationIndeterminate`.

It MUST NOT:

- retry automatically;
- retransmit automatically;
- read and infer success automatically;
- convert successor equality into success;
- convert current terminal state into proof that this call committed;
- suppress the ambiguity.

A later automatic reconciliation protocol requires a separately gated contract with explicit operation identity and outcome rules.

A caller may make a later new semantic decision only after fresh authoritative observation; that is not retroactive proof of the indeterminate call's outcome.

## 24. Malformed/invalid authority law

The semantic adapter fails closed as `InvalidAuthority` on at least:

- wrong key prefix/version/length/trailing bytes;
- wrong value magic/version/length/reserved/code/profile;
- unsupported durable value version;
- invalid identifier encoding/bounds;
- invalid public identity profile/bytes;
- invalid transport presence/bytes;
- key/value/request mismatch;
- impossible provider exact-key cardinality;
- provider key mismatch;
- non-positive provider revision for an existing record;
- unexpected transaction response branch/count/order/type;
- required failure-branch observation missing where the selected transaction semantics make that state impossible;
- `PendingEnrollment` encoded as a durable registered device;
- immutable device tuple mutation across an existing-record currentness conflict.

Malformed authority is never repaired, normalized, skipped, treated as absence, replaced from caller state, or rewritten automatically.

## 25. No provider-global capacity inference

The existing in-memory `WorkspaceDeviceRegistry` retains its `4096` membership and `4096` device entry limits.

The durable semantic adapter does not use prefix scans, provider counts, local counters, cached counts, or approximate counts to emulate those source/disposable bounds.

Therefore the first durable adapter source checkpoint must not emit:

- `MembershipCapacity`;
- `DeviceCapacity`;

from provider-global state.

A production durable quota requires a separate contract selecting exact counting and atomicity semantics.

## 26. No role mutation law

Phase 130 exposes no role transition operation and C03e-IQ treats current stored role as immutable under the selected mutation set.

The durable adapter may read and return role metadata but must not add a role-change mutation, infer capability grants, or change role while performing lifecycle updates.

A later role-change feature requires its own semantic and authorization contract.

## 27. Existing semantic-store precedent

Exact existing semantic-over-raw-executor precedent:

`crates/prw-remote-bridge/src/reachability_durable_snapshot_etcd_store.rs`

Exact C03e-IT blob:

`a381963986c79f8a314088839316d47595ba8686`

That source demonstrates the selected architectural pattern:

- semantic owner accepts an already-created control-plane raw executor;
- semantic owner constructs canonical keys/values;
- raw provider executor performs exact etcd mechanics;
- semantic owner validates key/value/request binding;
- compare-failure raw observations are classified above the provider layer;
- provider errors map to bounded semantic persistence failures;
- endpoint/bootstrap/TLS/auth/RBAC/retry/runtime ownership stays outside the semantic store.

C03e-IU adopts that ownership pattern only. Registry-specific Phase 130 semantics and C03e-IQ record laws remain independently authoritative.

## 28. Selected first semantic source ceiling

After C03e-IU closes, the next separately gated source-materialization checkpoint may add only:

1. new file:
   `crates/prw-registry/src/durable_registry_etcd_store.rs`
2. minimal module declaration/export only in:
   `crates/prw-registry/src/lib.rs`

No Cargo/lockfile change is selected.

That first semantic source checkpoint may materialize only:

- concrete custody of `DurableRegistryEtcdExecutor`;
- provider-neutral durable adapter error categories selected here;
- canonical exact membership/device reads;
- membership add/suspend/remove semantics;
- device registration semantics;
- initial transport bind;
- transport rotation;
- device revocation;
- current device/transport semantic read/validation;
- authenticated-session paired-read validation;
- exact successor construction using existing `durable_registry_codec` functions;
- authoritative compare-failure classification;
- pure/focused semantic helper tests that require no endpoint and prove binding/transition/failure precedence where possible.

The checkpoint may add only minimal `pub mod durable_registry_etcd_store;`-style exposure required by existing crate structure.

It must not change `WorkspaceDeviceRegistry` in-memory behavior except for a minimal module declaration/export. It must not replace that source/disposable authority path.

## 29. First semantic source test matrix

The later source checkpoint must prove at least, using endpoint-free pure helpers where possible and normal workspace compilation/CI for executor wiring:

### Read/binding

- canonical membership observation binds exact workspace/user;
- canonical device observation binds exact `DeviceId`;
- malformed value and key/value/request mismatch fail as invalid authority;
- exact absence remains distinct from malformed state.

### Membership

- add constructs Active canonical membership and duplicate compare failure remains duplicate;
- removed membership cannot be resurrected;
- Active -> Suspended successor preserves role/key tuple;
- Active|Suspended -> Removed successor preserves role/key tuple;
- repeated/ineligible transitions map to existing semantic errors;
- same bytes at moved revision after compare failure is currentness conflict.

### Device registration

- non-enrolled input fails before provider mutation planning;
- active membership is required;
- initial canonical device is Enrolled/unbound;
- duplicate device compare failure is duplicate, not success;
- changed/inactive membership failure state blocks registration;
- ambiguous unchanged-active membership currentness movement is not retried.

### Transport/device lifecycle

- first bind only from Enrolled/unbound;
- second bind is already-bound;
- rotation requires exact expected current and distinct replacement;
- successor preserves immutable tuple;
- revocation preserves immutable tuple + transport and changes only lifecycle;
- repeated revocation remains rejected;
- compare failure with exact semantic stale state maps to the Phase 130 error;
- compare failure still semantically eligible maps to currentness conflict;
- immutable tuple movement fails as invalid authority.

### Session/current transport

- paired membership/device current snapshot validation preserves Phase 130 precedence;
- inactive membership blocks an otherwise valid device;
- revoked device blocks an otherwise valid session;
- session workspace/user/public identity mismatch fails closed;
- current transport lookup requires Enrolled + bound current device;
- role remains metadata only.

No provider endpoint, production credential, production record, deployment, or Agent runtime is required for this test matrix.

## 30. Still separately gated after semantic source materialization

Even after the later semantic adapter source exists, the following remain independently blocked:

- raw `KvClient` ownership in `prw-registry`;
- etcd endpoint/cluster/bootstrap selection;
- provider connection creation policy;
- credentials, TLS, auth, RBAC, secret loading;
- retry/backoff/reconciliation protocol;
- Watch/Lease/TTL/Delete/prefix scan/range discovery/cache authority;
- production registry migration/population/provenance;
- production membership/device record creation;
- Agent production registry-executor composition;
- Agent production `PeerConnectivityIdentity` population;
- runtime `run()`/`main.rs` activation;
- requester/rendezvous/candidate/traversal/dialing activation;
- listener/readiness/network activation;
- systemd/package/certificate/database/schema mutation;
- deployment/restart.

## 31. Production population provenance remains blocked

C03e-IU does not authorize using any of the following as production registry authority:

- empty provider state;
- the Phase 130 in-memory registry;
- test/disposable fixtures;
- environment values;
- authenticated-session snapshots;
- request fields;
- device enrollment input without durable registration;
- network endpoint/address;
- certificate alone;
- cached/watch state.

Production membership/device population and migration provenance require a later explicit gate and immutable evidence.

## 32. Production peer composition remains blocked

C03e-IU selects only that the semantic durable adapter is the layer that must prove current canonical device lifecycle + current transport from one exact current device observation.

It does not select where/how Agent obtains the already-created raw executor, how provider endpoints are configured, or how production peer objects are inserted into Agent runtime structures.

A later composition checkpoint must preserve one-device-observation provenance and must not re-derive transport identity from network address, environment, stale session, fixture, or expected-device hint.

## 33. Security and operational invariants

C03e-IU preserves:

- user identity != device identity;
- logical `DeviceId` != `TransportIdentity`;
- transport identity is current separately rotatable state inside the device record;
- IP/port is reachability, not identity;
- immutable public identity cannot be rebound through transport mutation;
- removed membership and revoked device remain terminal participation states;
- role metadata is not a capability grant;
- provider mechanics are not registry semantic authority;
- malformed/ambiguous/unavailable state fails closed;
- mutation indeterminacy is not success.

No secret/private key, credential, certificate, trust root, provider resource, database/schema, registry record, systemd unit/package, network route/firewall/DNS/TUN/TAP state, listener/readiness state, runtime state, or deployment state is introduced.

## 34. Exact-head validation requirement

C03e-IU may be semantically closed only after the exact final C03e-IU head proves:

- predecessor/base/merge-base is exact C03e-IT head `2b75dd56022e179e480d73be7964eebf6400602d`;
- branch is ahead only by the bounded docs-only selection commit;
- exactly one contract path changed;
- no Rust/Kotlin/Cargo/lockfile/workflow/runtime/security/deployment path changed;
- automatically triggered Rust validation passes on the exact final head;
- path-filtered workflows are recorded accurately;
- immutable canonical Drive evidence is uploaded, raw-read back, byte-counted and SHA-256 verified;
- PR remains draft/open/unmerged.

No validation result from another head may be inherited.

## 35. Repository/deployment non-authorization

C03e-IU does not authorize or perform:

- repository visibility/configuration mutation;
- merge;
- PR close;
- ready-for-review transition;
- branch deletion;
- history rewrite;
- production deployment;
- service restart;
- systemd/package mutation;
- credential/certificate/trust/RBAC/auth mutation;
- provider resource creation;
- production registry write;
- migration/schema population;
- firewall/routing/DNS/TUN/TAP mutation;
- listener/readiness activation;
- production-state mutation.

Repository visibility remains the pre-existing repository state and is not changed by this checkpoint.

## 36. Closure meaning

If exact-head validation and immutable evidence recording pass, C03e-IU closure means only:

`PRODUCTION_DURABLE_REGISTRY_SEMANTIC_ADAPTER_BOUNDARY_SELECTED`

Specifically it means:

- `prw-registry` is selected as the semantic durable adapter owner over the C03e-IT raw executor;
- the adapter owns `DurableRegistryEtcdExecutor`, not raw `KvClient`;
- canonical C03e-IQ codecs and Phase 130 semantic transitions remain authoritative;
- semantic/provider/currentness failure categories remain distinct;
- exact pre-read -> semantic validation -> complete successor encode -> dual CAS -> failure reclassification is selected for existing-record transitions;
- active-membership guarded device registration preserves Phase 130 ordering and cross-record currentness;
- currentness movement without a provable Phase 130 semantic outcome fails as conflict and is never retried automatically;
- immutable tuple movement is invalid authority;
- authenticated-session validation uses one paired transactional read;
- durable provider-global capacity is not invented;
- first semantic source materialization is bounded to `crates/prw-registry/src/durable_registry_etcd_store.rs` plus a minimal module export only;
- production bootstrap, provider security/configuration, registry population, Agent peer composition and deployment remain separately gated.
