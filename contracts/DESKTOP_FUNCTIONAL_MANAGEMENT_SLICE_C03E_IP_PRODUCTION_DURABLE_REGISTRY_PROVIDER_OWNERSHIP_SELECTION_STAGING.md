# Phase 152 C03e-IP — Production Durable Registry Provider / Ownership Selection

Status: `STAGED_SELECTION`

Gate on closure:
`C03E_IP_PRODUCTION_DURABLE_REGISTRY_PROVIDER_OWNERSHIP_SELECTED`

Canonical closure token on successful exact-head validation and evidence recording:
`CLOSED_PRODUCTION_DURABLE_REGISTRY_PROVIDER_OWNERSHIP_SELECTION`

## 1. Scope

C03e-IP is the documentation-only prerequisite after closed C03e-IO.

C03e-IO proved that the exact repository contains the Phase 130 semantic current-registry model but no already-selected durable production authority source capable of supplying the current same-device binding of logical `DeviceId` plus current `TransportIdentity` required by the C03e-IN production `PeerConnectivityIdentity` provenance lock.

C03e-IP selects only:

1. the durable provider family for this current-state registry authority;
2. the crate ownership split between provider execution, registry semantics, and production composition;
3. the provider/semantic boundary required to avoid a dependency cycle and semantic duplication.

The exact predecessor is C03e-IO head:

`d04415cac126c11a390eb32517badedc14044bb6`

C03e-IP does not materialize Rust/Kotlin/Cargo/lockfile/workflow/runtime source, create provider resources, define a concrete key namespace or value schema, create credentials, select endpoints, configure TLS/auth/RBAC, migrate registry state, populate production records, activate the production remote companion, mutate `run()`/`main.rs`, or change production state.

## 2. Exact predecessor authority

C03e-IP inherits without broadening the C03e-IO finding:

> Production peer population remains blocked until a dependency-safe durable current-device/current-transport authority source is selected and later materialized without weakening Phase 130 semantics.

Exact predecessor contract:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_IO_PRODUCTION_CURRENT_DEVICE_TRANSPORT_AUTHORITY_SOURCE_GAP_SELECTION_STAGING.md`

Exact C03e-IO blob:

`609d373407852e325cb58369c00343cb475ea381`

No C03e-IP selection may reinterpret source/disposable registry state as production state or invent a current binding from network addresses, request/session identifiers, expected-device intent, Android presentation state, or test fixtures.

## 3. Registry semantic requirements from exact source

Exact source:

`crates/prw-registry/src/lib.rs`

Exact C03e-IO blob:

`cd7eb52eea19354620ff8ad26c9b8aad3f9c5eb6`

The current Phase 130 model already establishes the semantic law that a durable implementation must preserve.

### 3.1 Device registration

A registered device is keyed by exact `DeviceId` and retains its immutable workspace/user/device/public-identity tuple.

Registration is non-overwriting:

- only `Enrolled` devices may be registered;
- exact active workspace/user membership is required;
- duplicate `DeviceId` is rejected;
- capacity failure occurs before mutation in the source/disposable implementation.

A future durable provider does not obtain authority to redefine these rules.

### 3.2 Current transport binding

The first transport bind requires:

- exact existing device;
- current `Enrolled` lifecycle;
- no already-bound transport identity.

A second initial bind is rejected rather than silently replacing current state.

### 3.3 Transport rotation

`rotate_transport_identity` is already a compare-before-mutate semantic transition:

- the replacement must differ from the expected current identity;
- the device must exist and remain `Enrolled`;
- a current transport identity must exist;
- the exact expected current identity must match;
- only then may the replacement become current.

A production durable implementation therefore needs an authoritative current-state compare-and-mutate primitive. A last-write-wins blind overwrite is not semantically equivalent.

### 3.4 Revocation and current validation

Device revocation is a terminal participation transition from `Enrolled` to `Revoked` while retaining the immutable tuple.

Transport validation rejects:

- unknown device;
- revoked device;
- missing current transport binding;
- stale/mismatched presented transport identity.

Authenticated-session validation separately revalidates current membership/device state and the immutable identity tuple.

The production durable provider must support these currentness decisions without allowing stale snapshots to become authoritative by default.

## 4. Exact dependency topology

Exact registry manifest:

`crates/prw-registry/Cargo.toml`

Exact C03e-IO blob:

`ec9215d9bad86ac9601e2f2d1bc0ed8461e724c1`

Existing dependency direction includes:

`prw-registry -> prw-control-plane`

Exact control-plane manifest:

`crates/prw-control-plane/Cargo.toml`

Exact C03e-IO blob:

`acf008393686c10f5b9d63605399a608737973f7`

The control-plane crate already contains the selected `etcd-client` dependency and provider-specific authority executors for other Phase 152 domains.

Exact Agent manifest:

`crates/prw-agent/Cargo.toml`

Exact C03e-IO blob:

`4c70d6be9b56f39edc10810eefa3428314ed7559`

The Agent already consumes both `prw-registry` and `prw-control-plane`.

C03e-IP MUST NOT introduce or require the reverse edge:

`prw-control-plane -> prw-registry`

while the current `prw-registry -> prw-control-plane` dependency exists.

Such a reverse edge would create a dependency cycle and is not selected.

## 5. Exact etcd current-state precedent

Exact provider executor:

`crates/prw-control-plane/src/reachability_durable_snapshot_etcd.rs`

Exact C03e-IO blob:

`77fc9f345c17c5722c5240f3cead7ea68cb55cac`

This source is the closest exact repository precedent for the required current-state provider mechanism.

It establishes a control-plane-owned raw etcd executor that:

- accepts an already-created `KvClient`;
- performs default-linearizable exact-key reads;
- retains exact observed raw key/value bytes and positive `mod_revision` evidence;
- performs a dual compare on exact observed `mod_revision` plus exact observed value;
- commits exactly one replacement `Put` only on compare success;
- performs an authoritative exact-key failure read when the compare branch fails;
- reports provider read unavailability separately from mutation indeterminacy;
- rejects impossible exact-key cardinality, key mismatch, invalid revisions, and unexpected transaction response shapes;
- does not decode PRW semantic records;
- does not configure endpoints, TLS/auth/RBAC, credentials, scans, Watch, leases, TTLs, retries, background tasks, or deployment.

This mechanism matches the shape needed by the Phase 130 current-state compare-before-mutate rules without assigning registry semantics to the provider layer.

## 6. Exact semantic-adapter precedent

Exact semantic adapter:

`crates/prw-remote-bridge/src/reachability_durable_snapshot_etcd_store.rs`

Exact C03e-IO blob:

`a381963986c79f8a314088839316d47595ba8686`

This source proves an established Phase 152 layering rule:

- the control-plane provider executor owns opaque etcd execution;
- a semantic owner above that executor owns canonical key/value encoding and decoding;
- the semantic owner verifies requested-key/value binding;
- provider failures and ambiguous definitive outcomes fail closed;
- compare failure is classified from authoritative returned bytes rather than guessed from RPC shape alone.

The semantic adapter explicitly does not own provider connection bootstrap, TLS/auth/RBAC/credentials, deployment, retries, or background execution.

C03e-IP selects the same separation principle for the production registry domain. It does not copy reachability codecs or types into the registry.

## 7. Exact Agent composition precedent

Exact Agent composition source:

`crates/prw-agent/src/production_reachability_owner_composition.rs`

Exact C03e-IO blob:

`6a338b43995ecc069383e8aee63d7b53a35bc6ff`

This source demonstrates an existing production composition law:

- provider bootstrap is completed/narrowed before Agent semantic composition;
- the Agent receives a dedicated typed provider executor rather than a broad generic provider client;
- the executor is moved into the semantic store;
- no raw `Client`/`KvClient` is exposed as general application authority;
- the Agent does not duplicate provider connection logic or semantic record rules.

C03e-IP selects this composition shape for the registry domain at the ownership level only.

## 8. Spanner precedent and non-selection

Exact Spanner recovery-epoch adapter:

`crates/prw-control-plane/src/recovery_epoch_spanner.rs`

Exact C03e-IO blob:

`986e17dc781aa9b6f7c5576f43b88acc491a5e20`

The existing Spanner adapter is appropriate to a different authority shape: a strongly read recovery-epoch ledger with a selected head plus append-only issuance history and transactional submission semantics.

Phase 130 registry semantics currently require authoritative current state, immutable identity binding, terminal lifecycle transitions, and compare-before-mutate transport rotation. They do not establish an append-only production registry history ledger as a prerequisite for current device/transport authority.

Therefore C03e-IP does **not** select Google Cloud Spanner for the initial production durable current-registry authority.

This is a bounded selection for the current C03e-IP requirements. It does not declare Spanner unsuitable for all future registry history, analytics, audit, or unrelated authority domains.

A future separately authorized requirement for an append-only registry history ledger would require a new decision rather than silently expanding this current-state selection.

## 9. Provider selection

C03e-IP selects:

**etcd as the production durable current-state provider family for the current device / current transport registry authority.**

The selection is limited to the provider mechanism required for authoritative exact-key current-state reads and compare-before-mutate transitions.

The selection does not authorize a concrete endpoint, cluster, namespace, key prefix, value encoding, record schema, credential source, TLS configuration, RBAC policy, lease, Watch stream, TTL, retry policy, migration, resource provisioning, or runtime connection.

## 10. Selected ownership topology

C03e-IP selects the following ownership split.

### 10.1 `prw-control-plane` owns provider-specific etcd execution

The control-plane layer is the selected owner for the concrete etcd mechanism required by the registry domain.

Its future registry-specific provider seam may own only the bounded provider mechanics established by the exact current-state precedent, including:

- an already-created/narrowed dedicated etcd KV capability;
- exact-key linearizable reads;
- exact provider observation evidence required for safe compare-and-mutate;
- exact compare-and-put execution;
- provider-specific definitive/indeterminate error classification.

It MUST NOT own or invent:

- workspace membership semantics;
- device lifecycle semantics;
- immutable device identity binding rules;
- transport binding/rotation semantic classification;
- role-to-capability mapping;
- authenticated-session registry validation;
- PRW registry key/value semantic codecs unless a later architecture gate explicitly moves that authority.

### 10.2 `prw-registry` retains registry semantic authority

`prw-registry` remains the selected semantic owner for:

- workspace membership lifecycle;
- immutable registered-device tuple semantics;
- current `DeviceLifecycle` participation rules;
- current transport bind/rotation rules;
- current transport validation;
- session-to-current-registry revalidation;
- semantic classification of provider observations/results;
- future canonical registry record/key codec ownership, subject to a separately gated exact contract.

The existence of a durable provider must not create a second registry model beside Phase 130.

Because `prw-registry` already depends on `prw-control-plane`, a future narrow registry semantic adapter may consume a control-plane-owned provider executor without adding the prohibited reverse dependency.

C03e-IP does not materialize that adapter or freeze its Rust type names.

### 10.3 `prw-agent` owns production composition only

The Agent is selected as the production composition boundary that may later receive already-narrowed provider capability and compose it into the registry semantic authority required by production bootstrap/request paths.

The Agent MUST NOT:

- become the registry semantic source of truth;
- duplicate membership/device/transport transition rules;
- manufacture registry records from configuration or process state;
- expose a raw generic etcd client across unrelated production code;
- silently fall back to in-memory/test state when production durable authority is unavailable.

## 11. Dependency-cycle prohibition

The selected topology preserves the existing dependency direction:

`prw-registry -> prw-control-plane`

and composition direction:

`prw-agent -> prw-registry`

`prw-agent -> prw-control-plane`

C03e-IP does not authorize:

- `prw-control-plane -> prw-registry`;
- moving Phase 130 semantics into control-plane to avoid the cycle;
- introducing a new shared crate merely to bypass the existing dependency graph;
- copying semantic record definitions between crates;
- dependency inversion/refactoring beyond the selected ownership boundary.

If future exact source proves the selected topology cannot be materialized without a cycle, implementation must stop and return to a separately authorized architecture checkpoint.

## 12. Selected provider/semantic handoff law

The future provider-to-registry handoff must remain narrow and typed.

Provider execution may return only the bounded evidence needed for semantic classification, such as:

- exact requested/observed raw key bytes;
- exact observed raw value bytes;
- positive provider revision/currentness evidence;
- definitive compare-commit versus definitive compare-failure observation;
- provider read unavailable;
- provider mutation indeterminate;
- invalid/unexpected provider response shape.

The provider layer must not declare a device `Enrolled`, `Revoked`, transport-current, stale, or semantically valid merely from raw provider success.

The registry semantic layer must validate and classify those results against canonical PRW registry records and the exact requested identity before returning authoritative application state.

## 13. Current-state consistency requirement

The selected durable registry source must support authoritative currentness for at least:

- current membership lifecycle when required for protected current-registry validation;
- current device lifecycle;
- current immutable identity binding;
- current transport identity binding.

C03e-IP does not yet select whether these semantic fields occupy one exact provider key or multiple keys/records.

That question is deliberately deferred because it affects atomicity and record/schema design and therefore requires its own exact contract before source materialization.

No implementation may infer atomicity across multiple keys unless the later key/record contract proves the exact transaction boundary.

## 14. Compare-before-mutate requirement

Any later materialization must preserve the semantic effect of Phase 130 compare-before-mutate operations.

At minimum:

- initial transport bind must not overwrite an already-current binding;
- transport rotation must compare the exact expected current semantic identity before replacement;
- device lifecycle transition must reject stale/non-participating current state;
- membership lifecycle transition must reject invalid current state;
- provider mutation ambiguity must never be converted to success;
- a definitive provider compare failure may be classified semantically only from an authoritative failure observation.

The exact etcd comparison/key/value formulation is not selected by C03e-IP and belongs to the next record/CAS contract.

## 15. No Watch/lease/TTL selection

C03e-IP selects etcd as a durable current-state provider, not an etcd coordination feature set.

It does not select:

- Watch as an authorization source;
- lease expiry as device revocation;
- TTL expiry as membership removal;
- a live-owner lease model for registry records;
- prefix scans as record discovery;
- background cache synchronization;
- event-stream-derived currentness.

Protected operations must ultimately rely on authoritative current state according to the later selected read/record contract, not merely on a potentially stale local watch/cache snapshot.

## 16. No provider bootstrap/security selection

C03e-IP does not authorize or select:

- etcd endpoint addresses;
- production cluster identity;
- systemd credential names/paths;
- client certificates;
- private keys;
- CA/trust roots;
- username/password or token authentication;
- RBAC users/roles/policies;
- TLS server-name policy;
- credential rotation;
- network ACL/firewall changes;
- retry/reconnect loops;
- health/readiness activation.

Those are separate production security/operation decisions and must not be inferred from the provider-family selection.

## 17. No namespace/key/value/schema selection

C03e-IP deliberately does not define:

- etcd key prefix;
- workspace/user/device key shape;
- whether membership and device/transport state share a record;
- binary/text encoding;
- version bytes;
- maximum provider key/value lengths;
- canonical ordering;
- schema migration/version upgrade behavior;
- tombstone/history records;
- indexing/discovery layout.

No production registry record may be written until a separately gated contract selects these semantics and proves exact identity binding, atomicity, bounds, malformed-data failure behavior, and compatibility with Phase 130.

## 18. No migration/population selection

C03e-IP does not authorize:

- copying current in-memory fixture state into etcd;
- manufacturing an initial owner/member/device record;
- enrolling a production device;
- binding a production transport identity;
- changing an existing production device lifecycle;
- changing an existing production transport binding;
- importing Android/client snapshots;
- creating bootstrap defaults when records are absent.

An absent durable production record remains absent and must fail closed according to the later materialized authority path.

## 19. Failure law preserved

The future production durable registry path must fail closed on at least:

- provider unavailable;
- read result unavailable or ambiguous;
- mutation outcome indeterminate;
- malformed provider key/value;
- key/value semantic binding mismatch;
- unknown membership/device;
- inactive/removed membership;
- non-enrolled/revoked device;
- absent transport binding;
- stale expected current state;
- mismatched current transport identity;
- impossible duplicate/ambiguous current record state;
- unsupported record version;
- any state for which same-device currentness cannot be proved.

No default, first-match, cache-as-authority, stale snapshot, old authenticated session, environment variable, endpoint address, request ID, expected-device hint, or test fixture may convert one of these failures into production authority.

## 20. Provider neutrality versus provider selection

Phase 130 originally remained provider-neutral because durable persistence was deferred.

C03e-IP now selects etcd only for the production durable current-state provider mechanism needed by the productization path.

This does not erase provider-neutral semantic boundaries:

- registry semantics remain provider-independent;
- provider-specific errors are narrowed before application authority decisions;
- no etcd primitive becomes a PRW identity/capability concept;
- future tests must be able to prove semantic behavior independently from a live production cluster.

The selection is therefore a provider implementation choice beneath an unchanged semantic security boundary, not a rewrite of the registry model.

## 21. Spanner and other providers explicitly not selected

For this initial durable current-registry authority, C03e-IP does not select:

- Google Cloud Spanner;
- PostgreSQL;
- SQLite;
- filesystem records;
- object storage;
- embedded KV engines;
- Redis;
- client-side/Android state;
- environment variables;
- a new remote registry microservice.

No dependency or infrastructure change follows from this non-selection.

## 22. Source-materialization ceiling

C03e-IP authorizes no source materialization.

In particular it does not authorize:

- a registry etcd executor module;
- a registry store trait/adapter;
- registry key/value codecs;
- Cargo dependency changes;
- lockfile changes;
- new crates;
- provider connection code;
- environment/config variables;
- systemd credentials;
- production data writes;
- tests that contact production provider state;
- runtime callsite activation.

The first source path ceiling must be selected only after the record/key/CAS contract is closed.

## 23. Next prerequisite — durable registry record/key/CAS contract

The next separately gated checkpoint must remain documentation-only initially and select the exact durable registry record/key/CAS semantics required to materialize this provider/ownership topology safely.

That checkpoint must prove from exact source and existing bounds at least:

1. exact authoritative lookup key(s) for membership/device/current transport state;
2. exact semantic record boundary and atomicity requirement;
3. canonical versioned key/value encoding ownership;
4. exact field/bounds preservation from Phase 130 identifiers and identity material;
5. immutable tuple representation and rebinding prohibition;
6. current device lifecycle representation;
7. current transport optional/bound representation;
8. initial bind compare law;
9. transport rotation compare law;
10. lifecycle transition compare law;
11. multi-record transaction law if more than one key is required;
12. malformed/unknown-version/provider ambiguity classification;
13. namespace collision isolation from existing Phase 152 etcd authority domains;
14. exact focused test matrix;
15. exact first Rust source path ceiling;
16. continued prohibition on production record creation or runtime activation.

C03e-IP does not pre-authorize the outcome of that record/key/CAS decision.

## 24. Frozen executable/runtime surfaces

The following remain unchanged and uninvoked by C03e-IP:

- production reachability bootstrap;
- production durable-owner recovery;
- production requester/rendezvous composition;
- process-companion wrapper;
- public `run()`;
- `main.rs`;
- listeners/readiness;
- candidate publication;
- NAT traversal;
- relay activation;
- peer dialing;
- production retry/reconnect/rebootstrap loops;
- Android production connection/enrollment/revocation;
- production registry population.

## 25. Security and identity invariants

C03e-IP preserves:

- logical user identity != logical device identity;
- logical `DeviceId` != `TransportIdentity`;
- transport rotation does not replace logical device identity;
- IP/port is reachability, not identity;
- certificate transport identity alone does not prove current logical-device binding;
- request/session IDs are correlation, not durable registry authority;
- role metadata does not create capabilities;
- stale authenticated-session snapshots require current-registry revalidation;
- provider success alone does not establish semantic authority.

No secret, credential, private key, certificate, trust root, RBAC rule, account token, registry record, provider resource, or production database value is introduced.

## 26. Exact-head validation requirement

C03e-IP may be semantically closed only after the exact final C03e-IP head proves:

- predecessor/base/merge-base is exact C03e-IO head `d04415cac126c11a390eb32517badedc14044bb6`;
- branch is ahead only by the bounded docs-only selection commit;
- exactly one contract path changed;
- no Rust/Kotlin/Cargo/lockfile/workflow/runtime/security path changed;
- automatically triggered Rust validation passes on the exact final head;
- path-filtered workflows are recorded accurately;
- immutable canonical Drive evidence is written and raw-read back;
- PR remains draft/open/unmerged.

No validation result from another head may be inherited.

## 27. Repository and deployment non-authorization

C03e-IP does not authorize or perform:

- merge;
- branch deletion;
- history rewrite;
- repository visibility/configuration mutation;
- production deployment;
- service restart;
- provider resource creation;
- etcd data mutation;
- database/schema/migration mutation;
- systemd/package mutation;
- credential/certificate/trust/RBAC/auth mutation;
- firewall/routing/DNS/TUN/TAP mutation;
- listener/readiness activation;
- production-state mutation.

Repository visibility remains whatever exact repository metadata reports; C03e-IP does not change it.

## 28. Closure meaning

If exact-head validation and immutable evidence recording pass, C03e-IP closure means only:

`PRODUCTION_DURABLE_REGISTRY_PROVIDER_OWNERSHIP_SELECTED`

Specifically:

- etcd is selected as the durable current-state provider family for the production registry authority;
- `prw-control-plane` owns provider-specific raw etcd execution/bootstrap boundary;
- `prw-registry` retains Phase 130 semantic authority and future semantic record/codec classification;
- `prw-agent` owns narrow production composition only;
- the existing dependency direction is preserved without a `prw-control-plane -> prw-registry` cycle;
- no concrete key/value schema, namespace, credential, endpoint, record, source adapter, production write, runtime activation, or deployment has been authorized.

The production peer population path remains blocked until the separately gated durable registry record/key/CAS contract and subsequent source materialization/validation checkpoints are closed.