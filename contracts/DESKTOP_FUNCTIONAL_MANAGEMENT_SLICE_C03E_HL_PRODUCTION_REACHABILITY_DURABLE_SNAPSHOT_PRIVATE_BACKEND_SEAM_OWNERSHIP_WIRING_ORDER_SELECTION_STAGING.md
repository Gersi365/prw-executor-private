# Private Remote Workspace — C03e-HL Production Reachability Durable Snapshot Private Backend Seam / Ownership / Wiring Order Selection Staging

Status: `STAGED_SELECTION_ONLY — DOCS_ONLY — NO_RUNTIME_AUTHORIZATION`

## Purpose

This checkpoint selects the missing private backend seam, ownership boundary, and source-materialization order required before the C03e-HK durable-snapshot etcd client topology can be wired safely.

It is a direct successor to C03e-HK. It preserves the C03e-HJ durable-snapshot application contract and every C03e-HK security-principal, RBAC, endpoint/trust, credential, connection-topology, and lifecycle selection unchanged.

C03e-HL exists because the exact C03e-HK source tree does not yet contain a durable-snapshot runtime backend/store or a reachable durable-snapshot consumer seam. Adding a third etcd connection directly to the existing bootstrap at this point would therefore create an unused/orphaned provider connection and would invent wiring not represented by the current control-plane composition boundary.

This checkpoint is documentation-only. It does **not** authorize Rust source modification, provider connection attempts, credential loading, etcd authentication/RBAC mutation, deployment, service activation, database migration, production rollout, or privileged host mutation.

## Authoritative predecessors

### C03e-HJ application contract

C03e-HJ locked the durable-snapshot etcd application contract:

- key prefix: `b"/prw/reachability/durable-snapshot/"`;
- exact key: `prefix || peer_id.as_bytes()`;
- canonical fixed-width value: 112 bytes;
- application read: exact-key `Get(raw_key)` only;
- application write: exact-key `Put(raw_key, raw_value, None)` only;
- no lease attachment;
- no `Delete`;
- no prefix/range scan;
- no arbitrary range read;
- no `Watch`;
- no compaction operation;
- fail-closed canonical decode/profile/domain/hash validation;
- provider construction boundary receives only a preconnected role-scoped `etcd_client::KvClient`.

C03e-HL changes none of these semantics.

### C03e-HK provider/security contract

C03e-HK selected:

- the same logical three-member reachability-authority etcd cluster;
- the same validated runtime-supplied three-HTTPS-endpoint vector;
- the same runtime-supplied trust bundle;
- the same pinned TLS server name, `reachability-etcd.prw.internal`;
- dedicated durable-snapshot principal `prw-reachability-durable-snapshot`;
- dedicated RBAC role `prw-reachability-durable-snapshot-rw` scoped only to `/prw/reachability/durable-snapshot/`;
- provider-level read/write permission as the smallest etcd-native grant while PRW remains limited to exact Get/Put operations;
- a dedicated durable-snapshot certificate/private-key identity;
- three separately established role connections to the same validated authority-cluster inputs;
- bootstrap ownership of connection construction/lifecycle;
- only the durable role's `KvClient` crossing into the durable backend;
- no new process, service, global raw etcd client, dependency, or public provider API.

C03e-HL does not reopen any of those selections.

## Exact audited current source boundary

At the C03e-HK head, `crates/prw-control-plane/src/reachability_acquisition_evidence/` contains exactly:

- `attempt_id_generation.rs`;
- `bootstrap.rs`;
- `first_owner.rs`;
- `preparation.rs`.

There is no durable-snapshot module/store/backend in that directory.

`crates/prw-control-plane/src/reachability_acquisition_evidence.rs` currently exposes the existing acquisition/preparation facade and contains no durable-snapshot runtime module declaration.

`bootstrap.rs` currently:

- accepts one validated three-endpoint HTTPS vector;
- accepts one trust bundle;
- accepts two distinct role identities, live-owner and fence allocator;
- performs two separate `etcd_client::Client::connect(...)` calls;
- derives one `KvClient` per role;
- drops each broad `Client` handle;
- constructs `ReachabilityLiveOwnerAcquisitionPreparation::from_role_scoped_clients(live_owner_kv, fence_allocator_kv)`.

`preparation.rs` currently owns exactly two provider stores inside `ReachabilityLiveOwnerAcquisitionPreparation`:

- `live_owner: ReachabilityLiveOwnerEtcdStore`;
- `allocation: FenceSequenceAllocationEtcdStore`.

Its crate-private constructor accepts exactly two role-scoped `KvClient` values and has no durable-snapshot input or field.

Therefore the current source provides no legitimate destination for a third durable-snapshot `KvClient`.

## Problem statement

C03e-HK correctly selected a future third role connection, but its "next checkpoint readiness" text assumed source materialization could proceed directly to bootstrap wiring.

The exact HK source audit shows one prerequisite must be materialized first: a narrow private durable-snapshot backend and a reachable ownership/injection seam.

C03e-HL refines only the **implementation order**. It does not weaken, replace, or contradict C03e-HK's topology/security selections.

## Selection 1 — Private durable-snapshot module boundary

The durable-snapshot provider implementation belongs inside the existing reachability-acquisition-evidence control-plane boundary.

The selected source location for later materialization is:

`crates/prw-control-plane/src/reachability_acquisition_evidence/durable_snapshot.rs`

This is a **new private implementation module** selected for future source work. C03e-HL does not claim that the file already exists.

The module must remain private or crate-private according to the minimum visibility required by its immediate control-plane composition owner. It must not become a new public provider surface.

No new process, service, executable, listener, worker daemon, or independently deployed component is selected.

## Selection 2 — Dedicated concrete backend/store, not a generic storage abstraction

The future module must contain a dedicated durable-snapshot etcd backend/store whose scope is only the HJ durable-snapshot contract.

The implementation must not introduce a generic key-value repository, arbitrary etcd wrapper, shared storage trait, or provider-agnostic catch-all abstraction merely to host this slice.

The durable backend may own exactly one injected role-scoped `etcd_client::KvClient` for the dedicated durable-snapshot principal.

It must never own or accept:

- a raw `etcd_client::Client`;
- endpoint configuration;
- TLS trust configuration;
- certificate/private-key material;
- live-owner `KvClient`;
- fence-allocation `KvClient`;
- a global/shared multi-role provider client.

Connection construction remains outside this module.

## Selection 3 — Domain-specific operation surface

The durable backend operation surface is restricted to domain-specific operations corresponding exactly to C03e-HJ.

The future implementation may expose internally only the minimum equivalent of:

1. read the canonical durable snapshot for one exact `PeerConnectivityIdentity` / peer-id-derived key;
2. put the canonical durable snapshot for one exact `PeerConnectivityIdentity` / peer-id-derived key.

The implementation must derive the raw etcd key internally from the exact HJ prefix and peer identity. Callers must not supply arbitrary raw keys or ranges.

The implementation must encode/decode the canonical 112-byte durable-snapshot value internally through the already-selected HJ representation/validation boundary. Callers must not gain an arbitrary raw-value write surface.

The backend must not expose methods that permit:

- delete;
- prefix/range scan;
- arbitrary range read;
- watch;
- lease creation/attachment/revocation;
- compaction;
- authentication administration;
- role administration;
- member/cluster administration;
- generic transactions unrelated to the exact snapshot Get/Put contract.

## Selection 4 — Injection boundary

The concrete durable backend constructor must accept only the dedicated role-scoped `etcd_client::KvClient` produced by the reachability bootstrap.

The selected dependency direction is:

`reachability bootstrap -> dedicated durable KvClient -> private durable_snapshot backend/store -> narrow internal durable-snapshot operation surface`

The reverse dependency is forbidden. The durable backend must not call `Client::connect`, discover endpoints, load credentials, or construct TLS options.

This preserves the C03e-HK rule that connection authority and credential material remain centralized in the existing reachability bootstrap.

## Selection 5 — Composition ownership

The existing reachability preparation/composition boundary remains the nearest lifecycle owner for provider handles produced by the bootstrap.

For the minimum later implementation, `ReachabilityLiveOwnerAcquisitionPreparation` may be extended privately to own one durable-snapshot backend/store alongside the existing live-owner and fence-allocation stores.

This is selected as an implementation-minimizing ownership extension; it does **not** change the domain meaning of existing public acquisition operations and does not require renaming the facade.

Rules:

1. durable backend ownership must be private to the existing control-plane composition boundary;
2. no raw durable `KvClient` accessor may be added;
3. no raw durable backend/store accessor may be public;
4. any capability exposed to internal consumers must remain narrow and domain-specific;
5. existing live-owner and fence-allocation ownership remains unchanged;
6. no role's `KvClient` may be cloned/reused as another role's authenticated context.

If the exact later consumer requires a scoped execution capability, that capability must follow the existing preparation pattern: borrow only the durable backend it needs, expose only durable-snapshot domain operations, and provide no provider/configuration handle.

## Selection 6 — No orphan connection

A third durable-snapshot etcd connection is forbidden until the private durable backend and its composition destination exist in source.

A valid future bootstrap change must be able to move the resulting dedicated durable `KvClient` immediately into a reachable durable backend owned by the returned control-plane composition object.

The following states are explicitly invalid:

- connecting the durable role and then dropping its `KvClient` unused;
- retaining a third raw `Client` only to keep a connection alive;
- storing the durable client in a global/static singleton;
- creating an unused optional field with no reachable domain consumer;
- wiring the durable role into live-owner or fence stores as a placeholder;
- sharing an existing role's client until the durable seam is implemented.

## Selection 7 — Mandatory source-materialization order

Future implementation must be split so the architecture exists before provider connection wiring.

The required order is:

### Step A — private backend/seam materialization

1. add the private `durable_snapshot` module under `reachability_acquisition_evidence`;
2. materialize the dedicated durable-snapshot backend/store with an injected `KvClient` constructor;
3. implement only the HJ exact-key Get/Put application operations and canonical mapping;
4. add narrow fail-closed tests for key derivation, canonical value handling, and forbidden-surface absence where statically/testably applicable;
5. add the minimum private composition ownership/injection seam needed to hold and reach the backend;
6. do **not** add a third `Client::connect(...)` in this step.

### Step B — bootstrap third-role wiring

Only after Step A exists and is validated:

1. extend the existing bootstrap configuration with dedicated durable certificate/private-key identity material;
2. preserve the same already-validated endpoint vector and trust bundle;
3. preserve the pinned TLS server name from C03e-HK;
4. validate durable identity material fail-closed before provider use;
5. establish a third, distinct `Client::connect(...)` using the durable identity;
6. derive only that connection's `KvClient`;
7. drop the broad durable `Client` handle consistent with the existing bootstrap pattern;
8. inject the durable `KvClient` directly into the private durable backend/composition destination created in Step A;
9. return no raw provider handle.

Step B must not be materialized first.

## Selection 8 — Credential and role isolation remains mandatory

The future durable backend is associated only with the C03e-HK principal:

`prw-reachability-durable-snapshot`

and the C03e-HK role:

`prw-reachability-durable-snapshot-rw`.

No application source may treat live-owner, fence allocator, and durable snapshot as interchangeable credentials or clients.

Future bootstrap validation must reject exact credential reuse across the durable identity and either existing role to the extent deterministically checkable from the supplied material, without logging credential bytes.

No secret value may be embedded in source, contracts, tests intended for production configuration, CI output, or audit reports.

## Selection 9 — Endpoint/trust authority remains singular

C03e-HL does not introduce any new endpoint or trust configuration.

The future durable connection must consume the same validated three-member HTTPS endpoint set and the same runtime-supplied reachability-authority trust bundle selected by C03e-HK.

Forbidden:

- a durable-specific endpoint vector;
- fallback endpoints;
- dynamic endpoint discovery added by this slice;
- system/native-root fallback;
- plaintext transport;
- TLS verification bypass;
- a second independently configurable CA/trust source.

## Selection 10 — Fail-closed behavior

Durable-snapshot provider and canonicalization failures must fail closed.

No failure path may fall back to:

- live-owner storage;
- fence-allocation storage;
- an in-memory authoritative substitute;
- a second key prefix;
- a legacy/non-canonical encoding;
- a request-controlled host/path/provider target.

Absence of a durable snapshot may be represented only according to the HJ-selected exact-key read semantics; malformed or non-canonical stored bytes must not be treated as an absent valid record.

## Selection 11 — Public API ceiling

No new public control-plane API is selected by C03e-HL.

The durable module, concrete backend/store, constructor, and provider types remain private implementation details unless a later source audit proves a minimal crate-visible capability is required.

Even in that case:

- visibility must be no broader than necessary;
- raw `KvClient`/`Client` access remains forbidden;
- arbitrary key/value provider operations remain forbidden;
- existing public live-owner acquisition semantics must remain unchanged.

## Selection 12 — Dependency ceiling

No dependency change is selected.

The existing control-plane etcd client/TLS dependency set is sufficient for this architecture. C03e-HL does not authorize:

- dependency upgrades;
- new storage frameworks;
- new async runtimes;
- new credential libraries;
- unrelated feature expansion.

## Source-materialization checkpoint split

C03e-HL intentionally replaces the single-step implementation assumption at the end of C03e-HK with two bounded source checkpoints.

The next source checkpoint should materialize **Step A only**: the private durable-snapshot backend/store and composition ownership seam, with no third provider connection.

A subsequent checkpoint may materialize **Step B only**: the dedicated durable identity/config validation and third bootstrap connection, injecting its `KvClient` into the already-existing seam.

This sequencing is mandatory to avoid speculative or orphaned provider wiring.

## Validation requirements for Step A

Before the private backend/seam source checkpoint can be considered complete, evidence must show at minimum:

1. the new durable module is private or minimally crate-visible;
2. the durable store/backend accepts only an injected `KvClient`;
3. it does not call `Client::connect`;
4. it derives only the HJ exact per-peer key under `/prw/reachability/durable-snapshot/`;
5. it preserves the HJ canonical 112-byte value contract;
6. its provider calls are limited to exact-key Get and Put without lease;
7. no delete/range/watch/lease/compaction/admin API is exposed by the durable seam;
8. the composition owner has a concrete reachable destination for the durable backend;
9. no third connection is added yet;
10. existing live-owner and fence behavior remains unchanged;
11. formatting, clippy, tests, and workspace build succeed, or environment/tooling failure is explicitly separated from source defects.

## Validation requirements for Step B

Before the later bootstrap-wiring checkpoint can be considered complete, evidence must show at minimum:

1. HK's three-endpoint HTTPS validation remains intact;
2. the same endpoint vector and trust bundle feed all three role connections;
3. durable certificate/private-key material is a distinct input and fails closed when absent;
4. durable credential reuse with existing roles is rejected where exact byte comparison is applicable;
5. a distinct third `Client::connect(...)` is used for the durable identity;
6. only its `KvClient` reaches the durable backend;
7. the broad durable `Client` is not exposed or retained as a new global owner;
8. no orphan/unused durable connection exists;
9. HJ key/value and Get/Put-only semantics remain unchanged;
10. no public raw etcd API is introduced;
11. formatting, clippy, tests, and workspace build succeed, or environment/tooling failure is explicitly separated from source defects.

## Explicitly not authorized by C03e-HL

This checkpoint does not authorize any of the following:

- Rust source modification;
- third etcd connection creation;
- runtime wiring;
- provider network I/O;
- runtime/service/listener/background-worker activation;
- etcd authentication enablement or disablement;
- etcd user creation/deletion/password mutation;
- etcd role creation/deletion/grant/revoke operations;
- certificate generation, issuance, rotation, installation, or distribution;
- CA/trust-bundle mutation;
- endpoint mutation;
- etcd member/cluster mutation;
- service deployment or restart;
- DNS mutation;
- firewall/network mutation;
- privileged host mutation;
- database migration;
- production rollout;
- remote-access activation;
- repository visibility mutation;
- branch deletion/cleanup;
- merge;
- unrelated refactor, rename, or dependency upgrade.

## Audit interpretation

C03e-HL is a prerequisite-selection checkpoint only.

Successful completion means that the missing durable-snapshot provider destination and the safe implementation order are locked tightly enough that later source work does not need to invent ownership or create an orphaned third connection.

It must not be interpreted as evidence that:

- the durable backend exists in Rust yet;
- the third etcd connection exists;
- etcd credentials or RBAC are provisioned;
- any production cluster was contacted;
- any service was deployed or activated.

## Next checkpoint readiness

After C03e-HL is reviewed and accepted, the next safe checkpoint is a **source-only private backend/seam materialization** checkpoint corresponding to Step A above.

That next checkpoint must not add the durable etcd connection yet.

Only after the private durable backend and reachable composition destination exist and pass validation may a subsequent source-only checkpoint materialize the C03e-HK third-role bootstrap connection and inject its dedicated `KvClient`.
