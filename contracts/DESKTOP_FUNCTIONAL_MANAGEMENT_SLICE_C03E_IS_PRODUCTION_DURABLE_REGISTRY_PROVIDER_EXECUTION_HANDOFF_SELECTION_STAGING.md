# Phase 152 C03e-IS — Production Durable Registry Provider Execution Handoff Selection

Status: `STAGED_SELECTION`

Gate on closure:
`C03E_IS_PRODUCTION_DURABLE_REGISTRY_PROVIDER_EXECUTION_HANDOFF_SELECTED`

Canonical closure token on successful exact-head validation and evidence recording:
`CLOSED_PRODUCTION_DURABLE_REGISTRY_PROVIDER_EXECUTION_HANDOFF_SELECTION`

## 1. Scope

C03e-IS is the documentation-only prerequisite after closed C03e-IR.

C03e-IR materialized only the provider-neutral durable registry codec seam in `prw-registry`. C03e-IS now selects the dependency-safe handoff between those semantic codecs and a later raw etcd executor without materializing that executor, adding dependencies, selecting provider bootstrap/security configuration, composing the semantic durable adapter, creating production registry records, or activating production runtime behavior.

Exact predecessor C03e-IR head:

`72c50a33bd291b756a85886e198fe723aa31fb1a`

Exact predecessor C03e-IR tree:

`8652b8f8be6f4bb846a7b1199aea9995e8b63513`

C03e-IS changes documentation only.

## 2. Exact predecessor codec authority

Exact materialized codec source:

`crates/prw-registry/src/durable_registry_codec.rs`

Exact predecessor blob:

`c1aafcc80dea3a6d06f11b0d50418e25e1437473`

Exact minimal module export:

`crates/prw-registry/src/lib.rs`

Exact predecessor blob:

`f76b5daa664d5272ed97c9f76c6792b105825eb0`

The codec source owns canonical `PRWM` / `PRWD` key/value representation and exact key/value/request binding. It deliberately performs no provider I/O, transaction planning, retry, credential handling, runtime work, registry population, migration, or production activation.

C03e-IS does not move those semantics into `prw-control-plane`.

## 3. Exact C03e-IQ CAS/currentness authority

Exact retained selection contract:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_IQ_PRODUCTION_DURABLE_REGISTRY_RECORD_KEY_CAS_SEMANTICS_SELECTION_STAGING.md`

Exact blob at the C03e-IR head:

`ef7162ae2dc3a318d23ca1d283c729eda64f8542`

C03e-IS preserves its provider laws exactly:

- authoritative single-record reads are default-linearizable exact-key Gets;
- paired membership/device validation uses one transactionally consistent transaction containing both exact-key Gets;
- initial create uses exact key `version == 0` absence compare;
- existing-record replacement uses conjunctive exact observed positive `mod_revision` and exact observed raw value bytes;
- device registration compares exact active-membership `mod_revision + raw value` and exact device-key `version == 0` in one transaction;
- compare-failure branches return authoritative exact-key observation(s);
- mutation RPC/provider failure without a definitive transaction response is `INDETERMINATE`;
- no hidden automatic mutation retry is selected;
- malformed, ambiguous, impossible-cardinality, non-positive-revision, or unexpected provider response shape fails closed;
- no Watch, Lease, TTL, Delete, prefix scan, range discovery, cache authority, or stale/serializable read is selected.

C03e-IS narrows how those laws cross crate/provider boundaries; it does not alter them.

## 4. Existing real-etcd precedent

Exact existing control-plane precedent:

`crates/prw-control-plane/src/reachability_live_owner_etcd.rs`

Exact blob:

`a466481de12ad31f0b315928c7bca819ce3e6394`

That source already demonstrates the selected provider-mechanics pattern:

- caller supplies an already-created `etcd_client::KvClient`;
- construction itself does no network I/O;
- default-linearizable exact-key Get is used without serializable mode;
- exact dual CAS is translated to an etcd transaction;
- success branch performs the exact selected Put;
- compare-failure branch performs the exact selected Get;
- mutation RPC failure maps to an indeterminate outcome;
- endpoint selection, connection/bootstrap, TLS/auth/RBAC, retry and runtime activation stay outside the store.

A second exact precedent is:

`crates/prw-control-plane/src/fence_sequence_allocation_etcd.rs`

Exact blob:

`3741936a16c4a5877b1a837e521dc9e32f9099fa`

It likewise consumes an already-created `KvClient`, performs canonical default-linearizable exact-key reads, validates bounded transaction shape, uses failure-branch exact Gets, and does not retry an indeterminate mutation.

These precedents validate provider mechanics only. They are domain-specific and are not registry semantic authority.

## 5. Exact dependency topology

Exact control-plane manifest:

`crates/prw-control-plane/Cargo.toml`

Exact blob:

`acf008393686c10f5b9d63605399a608737973f7`

It already pins:

`etcd-client = "=0.19.0"`

Exact registry manifest:

`crates/prw-registry/Cargo.toml`

Exact blob:

`ec9215d9bad86ac9601e2f2d1bc0ed8461e724c1`

It already depends on:

`prw-registry -> prw-control-plane`

Therefore C03e-IS selects preservation of the existing dependency direction. It does not authorize:

- `prw-control-plane -> prw-registry`;
- a dependency cycle;
- a new shared crate merely to route registry semantics;
- direct `etcd-client` ownership in `prw-registry`;
- a new Cargo or lockfile dependency for the first raw-executor source checkpoint.

## 6. Exact source-gap finding

The exact `crates/prw-control-plane/src` inventory at C03e-IR contains domain-specific etcd modules such as fence-sequence and reachability-live-owner executors, but no generic/raw durable-registry exact-key/CAS executor seam.

No existing source at the exact C03e-IR head can be silently reinterpreted as the registry provider executor without mixing an unrelated domain or inventing a new semantic owner.

Therefore C03e-IS is selection-only. It does not materialize Rust provider execution.

## 7. Selected ownership boundary

C03e-IS selects exactly this ownership split.

### `prw-registry`

Remains the semantic owner of:

- canonical membership/device key construction and decode;
- canonical `PRWM` / `PRWD` value encode/decode;
- exact key/value/request binding;
- Phase 130 membership/device/current-transport transition validation;
- semantic classification of authoritative provider observations;
- construction of complete canonical replacement bytes before a provider mutation request;
- interpretation of compare-failure observations into Phase 130 semantic outcomes.

### `prw-control-plane`

Owns only raw provider-specific etcd execution:

- exact-key default-linearizable Get;
- paired exact-key transactional Get;
- exact create-if-absent transaction;
- exact dual-CAS replacement transaction;
- exact active-membership + device-absence registration transaction;
- structural validation of provider response cardinality/key/branch shape;
- raw provider result/evidence transport back to the semantic owner.

It does not decode `PRWM` / `PRWD`, validate registry identifiers, classify lifecycle/role/transport semantics, or construct `PeerConnectivityIdentity`.

### `prw-agent`

Remains a later composition owner only. It does not own registry key/value formats or raw etcd transaction semantics.

## 8. Selected handoff shape

The dependency-safe handoff is raw and bounded.

A future `prw-registry` semantic adapter may construct canonical raw key/value bytes and pass only provider-mechanics inputs into a `prw-control-plane` executor.

The raw provider boundary may receive only data needed to execute the selected exact operation, including as applicable:

- exact key bytes;
- exact replacement value bytes;
- exact observed positive `mod_revision`;
- exact observed raw value bytes used as a compare operand;
- exact second key/value/revision operands for the bounded registration transaction.

The raw provider boundary may return only bounded provider evidence, including as applicable:

- exact requested/observed key bytes;
- exact observed raw value bytes;
- positive `mod_revision` for an existing record;
- authoritative absence;
- definitive mutation commit;
- definitive compare failure plus authoritative exact-key failure-branch observation(s);
- read unavailable;
- mutation indeterminate;
- invalid provider response shape.

Registry semantic types, lifecycle classification, role classification, transport-currentness classification and `PeerConnectivityIdentity` MUST NOT be required by the raw executor.

## 9. Exact-key read law

A later raw executor performs only an exact-key default-linearizable Get for a single-record read.

It must validate provider shape before returning evidence:

- zero exact entries means authoritative absence;
- one entry with the exact requested key may be returned as raw evidence;
- more than one entry fails closed;
- another key fails closed;
- an existing observation with non-positive `mod_revision` fails closed.

It must not:

- use prefix/range discovery;
- return the first result from a broader query;
- use serializable/stale reads;
- substitute a cache, Watch snapshot, fixture or process-local state.

## 10. Paired exact-key read law

Authenticated-session current-registry validation requires the C03e-IQ paired snapshot primitive.

The raw executor must perform both exact-key Gets inside one etcd transaction and return both raw observations from that one transaction response.

The raw layer validates only provider structure and exact requested keys. `prw-registry` later decodes and semantically binds both records.

Two sequential independent Gets are not the selected paired-read primitive.

## 11. Create-if-absent execution law

Initial membership/device creation uses exactly the C03e-IQ absence compare:

`exact key version == 0`

Success branch:

- one exact complete canonical Put for the selected record, except the separately selected device-registration multi-key transaction.

Failure branch:

- one authoritative exact-key Get for semantic classification by `prw-registry`.

A compare failure is never converted by `prw-control-plane` into semantic duplicate/success/idempotence.

No unconditional Put is selected.

## 12. Dual-CAS update execution law

Existing-record replacement uses exactly both compares:

1. exact key `mod_revision == observed positive mod_revision`;
2. exact key raw `value == exact observed raw value bytes`.

Success branch:

- exactly one complete replacement Put.

Failure branch:

- exactly one authoritative exact-key Get.

The raw executor may state only whether the compare branch committed and return provider evidence. It may not classify suspended/removed/revoked/already-bound/stale-transport or another registry semantic outcome.

## 13. Device-registration transaction execution law

The bounded cross-record registration transaction preserves exact C03e-IQ compares:

1. membership key `mod_revision == observed membership mod_revision`;
2. membership key `value == exact observed active membership raw bytes`;
3. device key `version == 0`.

Success branch:

- exactly one Put of the complete canonical enrolled/unbound device value under the exact device key;
- no membership rewrite.

Failure branch:

- authoritative exact Gets for both membership and device keys.

`prw-control-plane` validates only response shape and exact key identity. The semantic owner distinguishes changed/inactive/missing membership, duplicate device, malformed authority, or ambiguous state.

## 14. Compare-failure law

A definitive compare failure proves only that the success branch did not commit.

The raw executor must return the authoritative failure-branch observation(s) selected by C03e-IQ.

It must not:

- infer semantic success because current bytes equal proposed successor bytes;
- infer semantic success because current bytes equal prior observed bytes after a revision change;
- silently reissue the mutation;
- suppress revision movement;
- repair or normalize provider bytes.

`prw-registry` later validates canonical bytes and classifies the exact current semantic state.

## 15. Mutation indeterminacy law

If the provider/RPC does not return a definitive transaction response, the raw result is:

`INDETERMINATE`

No automatic mutation retry, retransmission, idempotent-success inference, or semantic classification is selected.

A new semantic mutation decision requires later authoritative re-observation. Any automatic reconciliation/retry protocol requires its own separately gated contract.

## 16. Provider-invalid failure law

The raw executor fails closed on provider-mechanics violations including at least:

- impossible exact-key cardinality;
- returned key different from requested key;
- non-positive `mod_revision` for an existing record;
- unexpected transaction branch;
- unexpected operation count/order/type in a transaction response;
- missing failure-branch observation required by the selected operation;
- extra response operations outside the selected bounded transaction;
- provider read failure;
- mutation response indeterminacy.

It does not decode or repair malformed registry value bytes; those bytes are returned only when provider shape is valid so `prw-registry` can perform canonical semantic validation.

## 17. Caller-owned `KvClient` law

The first raw-executor implementation must follow existing control-plane precedent: it accepts an already-created `etcd_client::KvClient` or equivalently receives provider custody from a caller-owned already-created boundary.

That executor does not select or perform:

- endpoint/cluster discovery;
- `Client::connect` policy;
- DNS/provider discovery policy;
- TLS certificate/trust selection;
- authentication credentials;
- RBAC provisioning;
- secret loading;
- retry/backoff policy;
- timeout policy beyond existing provider defaults unless separately selected;
- service/runtime activation.

Construction of the raw executor itself must perform no network I/O.

## 18. No raw `KvClient` leakage into `prw-registry`

C03e-IS explicitly rejects adding `etcd_client::KvClient`, raw etcd transaction construction, provider response decoding, endpoint/bootstrap configuration, or etcd errors as the semantic implementation mechanism inside `prw-registry`.

`prw-registry` may later depend on a bounded control-plane raw executor API because that dependency direction already exists. It may not become the raw etcd provider owner.

## 19. Semantic decode stays above provider execution

After a structurally valid raw observation returns, `prw-registry` remains responsible for:

- canonical key decode;
- canonical `PRWM` / `PRWD` decode;
- exact key/value/request binding;
- lifecycle/role/transport profile validation;
- immutable tuple preservation;
- Phase 130 transition validation;
- semantic error/outcome mapping.

Provider absence is not automatically a semantic success. Provider malformed bytes are not automatically absence. Provider compare failure is not automatically a registry-domain error until the semantic owner validates the returned current state.

## 20. Production peer provenance remains blocked

C03e-IS does not populate production `PeerConnectivityIdentity`.

Even after a later raw executor exists, production peer lookup still requires the separately gated semantic durable registry adapter and then a later production composition that proves from one canonical current device observation:

- exact requested/key/value `DeviceId` binding;
- lifecycle exactly `Enrolled`;
- transport presence exactly current/present;
- valid exact current same-device `TransportIdentity`.

No raw provider bytes may be handed directly to Agent bootstrap as peer identity.

## 21. No production population or migration

C03e-IS does not authorize:

- etcd registry key creation;
- production membership/device population;
- migration from the Phase 130 in-memory registry;
- bootstrap seed records;
- fallback fixtures;
- environment-derived registry records;
- background conversion;
- provider resource creation;
- schema/version migration.

The retained `Removed` / `Revoked` current-state semantics and v1.0 codec law remain unchanged.

## 22. Selected first raw-executor source ceiling

Because no generic raw registry executor exists at the exact C03e-IR head, the next separately gated source-materialization checkpoint may add only a dormant raw provider executor seam in `prw-control-plane`.

Selected maximum path ceiling:

1. new file:
   `crates/prw-control-plane/src/durable_registry_etcd.rs`
2. minimal module declaration/export only in:
   `crates/prw-control-plane/src/lib.rs`

No Cargo/lockfile change is selected because `prw-control-plane` already pins `etcd-client = 0.19.0`.

That first raw-executor source checkpoint may materialize/test only:

- caller-owned already-created `KvClient` custody;
- exact default-linearizable single-key Get;
- paired exact-key transactional Get;
- create-if-absent Txn translation;
- dual-CAS update Txn translation;
- active-membership + device-absence registration Txn translation;
- exact provider response shape validation;
- bounded raw observation/result carriers;
- read-unavailable vs mutation-indeterminate distinction;
- provider-free transaction-shape tests and, where existing disposable infrastructure permits without broadening scope, focused disposable provider-mechanics tests.

It must not decode registry records or depend on `prw-registry`.

If implementation requires another source path, new Cargo dependency, provider bootstrap/security logic, registry semantic import, runtime activation, migration/population, or broader architecture, stop and select a separate extension checkpoint.

## 23. Source-successor type-name non-authorization

C03e-IS selects the source path and behavior ceiling, not exact public Rust type or method names.

A later implementation may choose bounded names consistent with existing control-plane conventions, but must not use naming freedom to widen authority or semantics.

The module is a raw registry-provider mechanics seam, not a new registry semantic model.

## 24. Focused first raw-executor test matrix

The separately gated source successor must prove at least:

### Exact reads

- one exact key -> one raw present observation with exact key/value/positive `mod_revision`;
- absence -> exact absence;
- another key rejected;
- impossible cardinality rejected;
- no serializable/prefix/range option introduced.

### Paired read

- exactly two requested exact-key Gets are issued in one transaction;
- response order/key identity is validated exactly;
- absent/present combinations remain raw provider evidence, not semantic classification;
- unexpected branch/response count/type is rejected.

### Create-if-absent

- exact `version == 0` compare;
- one exact Put on success;
- one exact Get on compare failure;
- definitive compare failure remains distinct from RPC indeterminacy.

### Dual CAS

- exact positive `mod_revision` compare;
- exact observed raw-value compare;
- one complete replacement Put on success;
- one exact Get on compare failure;
- changed revision with same bytes is not auto-success and is not retried.

### Device registration

- membership revision compare + exact membership value compare + device version-zero compare;
- one device Put only on success;
- membership and device exact Gets on compare failure;
- no membership rewrite.

### Failure law

- read RPC failure -> read unavailable;
- mutation RPC failure without definitive response -> indeterminate;
- unexpected provider response shape -> fail closed;
- no hidden retry/reissue.

No test may require production credentials, production provider records, production endpoints, Agent runtime activation, or production networking.

## 25. Later semantic adapter prerequisite

After the raw control-plane executor is separately materialized and validated, another checkpoint must select/materialize the `prw-registry` semantic durable adapter that:

- constructs raw provider operations from canonical codecs and validated transition state;
- consumes raw provider evidence;
- decodes canonical records;
- validates key/value/request binding;
- maps provider unavailable/indeterminate/provider-invalid outcomes distinctly from Phase 130 semantic outcomes;
- performs no automatic mutation retry unless separately reconciled.

C03e-IS does not materialize that adapter or select its exact Rust type names/path ceiling.

## 26. Provider bootstrap/security remains separate

The later raw executor remains inert until separately composed with already-created provider custody.

C03e-IS does not select:

- concrete etcd endpoint or cluster;
- TLS trust roots/client certificates;
- authentication credentials;
- RBAC rules;
- secret source or `systemd LoadCredential=` wiring;
- endpoint failover;
- retry/backoff;
- health/readiness semantics;
- service lifecycle;
- production deployment.

## 27. Runtime and networking non-authorization

C03e-IS does not authorize or perform:

- `run()` / `main.rs` mutation;
- production registry bootstrap/population;
- Agent production peer lookup wiring;
- listener/readiness activation;
- requester/rendezvous execution;
- candidate publication;
- NAT traversal;
- peer dialing;
- firewall/routing/DNS/TUN/TAP mutation;
- background provider task/watch/cache;
- service restart/deployment.

## 28. Security and identity invariants

C03e-IS preserves:

- user identity != device identity;
- logical `DeviceId` != current `TransportIdentity`;
- IP/port remains reachability, not identity;
- immutable public identity cannot be rebound by raw provider execution;
- removed membership and revoked device remain terminal semantic states;
- provider mechanics do not become semantic authority;
- malformed/ambiguous/unavailable/indeterminate provider state fails closed;
- exact same-device current transport proof remains required before later `PeerConnectivityIdentity` construction.

No private key, credential, certificate, trust root, RBAC/auth rule, production provider record, schema/migration, systemd unit/package, network state, runtime state, repository visibility/configuration, or deployment state is changed.

## 29. Exact-head validation requirement

C03e-IS may be semantically closed only after the exact final C03e-IS head proves:

- predecessor/base/merge-base is exact final C03e-IR head `72c50a33bd291b756a85886e198fe723aa31fb1a`;
- branch is ahead only by the bounded docs-only selection commit;
- exactly one contract path changed;
- no Rust/Kotlin/Cargo/lockfile/workflow/runtime/security path changed;
- automatically triggered Rust validation passes on the exact final head;
- any Android/path-filtered workflows are reported only if they actually exist on that exact head;
- immutable canonical Drive evidence is uploaded and raw-read back with exact byte/hash equality;
- PR remains draft/open/unmerged.

No validation result from another head may be inherited.

## 30. Repository and lifecycle non-authorization

C03e-IS does not authorize or perform:

- repository visibility/configuration mutation;
- merge;
- PR close;
- ready-for-review transition;
- branch deletion;
- history rewrite;
- production deployment;
- service restart;
- systemd/package mutation;
- credential/certificate/trust/RBAC mutation;
- provider resource creation;
- production registry write;
- migration/schema population.

## 31. Closure meaning

If exact-head validation and immutable evidence recording pass, C03e-IS closure means only:

`PRODUCTION_DURABLE_REGISTRY_PROVIDER_EXECUTION_HANDOFF_SELECTED`

Specifically:

- existing dependency direction remains `prw-registry -> prw-control-plane`;
- registry semantics/codecs remain in `prw-registry`;
- raw etcd mechanics remain in `prw-control-plane`;
- caller-owned already-created `KvClient` custody is selected for the first raw executor;
- raw handoff consists only of exact provider-mechanics inputs and bounded provider evidence;
- exact linearizable read, paired read, version-zero create, dual-CAS update and bounded registration Txn laws are preserved;
- compare failure returns authoritative failure-branch observation(s);
- mutation RPC uncertainty remains `INDETERMINATE` with no hidden retry;
- a two-path first raw-executor source ceiling is selected in `prw-control-plane` with no Cargo change.

It does not mean the raw executor exists, the semantic durable adapter exists, provider bootstrap/security is selected, production records exist, production peer population is wired, runtime networking is active, or deployment is complete.
