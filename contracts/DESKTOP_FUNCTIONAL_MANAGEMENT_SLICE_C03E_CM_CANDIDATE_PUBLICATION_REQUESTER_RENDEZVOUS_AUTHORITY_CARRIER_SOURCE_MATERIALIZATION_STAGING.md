# Phase 152 C03e-CM — Candidate Publication Requester/Rendezvous Authority Carrier Source Materialization

Status: `STAGED SOURCE — PENDING EXACT-HEAD VALIDATION`

Target gate:
`C03E_CM_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AUTHORITY_CARRIER_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Closed C03e-CL is the authoritative predecessor:
- branch: `phase-152-c03e-cl-candidate-publication-requester-rendezvous-authority-carrier-selection-staging`
- head: `b1c51eb6a0c07234d7d7fc6cb7e45517e81b9a24`
- tree: `8d3cbb63a001f640f4644427f57b4edcd238273b`
- gate: `C03E_CL_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AUTHORITY_CARRIER_SELECTED`
- PR #209: body `Status: CLOSED`, draft/open/unmerged

C03e-CM must remain an exact descendant of this closed CL head.

## 2. Bounded purpose

C03e-CM materializes only the CL-selected provider-neutral requester/rendezvous authority carrier and authorization port in `prw-remote-bridge`.

It does not materialize:
- a concrete requester/rendezvous provider;
- a storage/backend schema;
- a lock, channel, actor or broker;
- a clock, TTL, heartbeat or retry policy;
- post-auth PRWC Command/frame execution;
- candidate-publication semantic composition;
- reachability mutation;
- listener/runtime/product wiring;
- networking or deployment.

## 3. Selected source module

The bridge-owned source module is:

`crates/prw-remote-bridge/src/requester_rendezvous_authority.rs`

It materializes exactly these CL-selected responsibilities:
- `AuthorizedRequesterRendezvous`;
- `RequesterRendezvousAuthorityProvider`;
- `RequesterRendezvousAuthorityError`.

`crates/prw-remote-bridge/src/root.rs` exposes only this module. No existing runtime source consumes it in CM.

## 4. One-shot authority grant

`AuthorizedRequesterRendezvous` owns exactly:
- one `AuthenticatedDeviceSession` for the requester;
- one expected publisher logical `DeviceId`.

The value is deliberately not `Copy` or `Clone`. It is one-shot operation evidence intended to be consumed by at most one later candidate-publication execution attempt.

The grant exposes read-only accessors for:
- requester session;
- expected publisher logical device.

It contains no:
- PRWP payload;
- PRWC request ID;
- `TransportIdentity`;
- `CandidateId`;
- endpoint/path metadata;
- freshness token;
- IP/port/socket identity;
- clock value;
- process/thread/task identity;
- new rendezvous identifier/token.

## 5. Authority-construction boundary

The source follows the repository precedent used by `ReachabilityLiveOwnerGrant::from_authority(...)`.

`AuthorizedRequesterRendezvous::from_authority(...)` is an adapter constructor for future concrete authority implementations. Construction or possession alone is explicitly not authority.

Production composition must obtain a grant from a concrete `RequesterRendezvousAuthorityProvider` after that provider has established exactly one current server-side requester/rendezvous selection.

The constructor therefore does not:
- validate registry currentness;
- choose requester identity;
- infer workspace;
- select a provider record;
- perform I/O;
- create fallback authority.

## 6. Provider-neutral authorization port

`RequesterRendezvousAuthorityProvider` exposes exactly one synchronous provider-neutral operation:

```text
authorize_current_for_publisher(
    &mut self,
    publisher_device_id: &DeviceId
) -> Result<AuthorizedRequesterRendezvous, RequesterRendezvousAuthorityError>
```

The publisher `DeviceId` is a lookup selector only. It must later be supplied from the CK-authenticated publisher session and is not requester authority by itself.

The concrete provider remains responsible for its own currentness linearization mechanism. The trait selects no storage, lock, transaction, lease, cache or retry implementation.

## 7. Fail-closed error surface

The source materializes the CL-selected stable classes:
- `Missing`;
- `StaleOrRetired`;
- `Ambiguous`;
- `UnavailableOrIndeterminate`.

All four are terminal authorization failures for that attempt. None may become an authorized grant.

Concrete providers may retain richer internal error detail, but later adapter mapping must not convert absence, stale/retired state, multiple authority or indeterminate provider state into success.

## 8. Authorization linearization preserved

CM does not implement a linearization mechanism. It only exposes the port through which a future concrete provider must perform one bounded current-authority authorization.

The already-selected CL semantics remain authoritative:
- provider mutations that linearize before authorization must be visible to that authorization;
- successful authorization yields one owned one-shot grant;
- later provider mutation does not retroactively revoke that already-admitted single attempt;
- the grant cannot authorize another later attempt;
- no provider guard is held across unrelated frame I/O, candidate validation, durable commit or response I/O.

## 9. Later execution obligations remain unchanged

CM does not execute candidate publication.

A later execution composition must still:
1. start from a CK-authenticated publisher connection;
2. receive/strictly decode one allowed post-auth Command under separately selected stream execution;
3. derive publisher logical identity from the CK-authenticated session;
4. call existing `publish_current_candidates(...)`;
5. call the requester/rendezvous authority provider using only authenticated publisher `DeviceId` as lookup selector;
6. require exactly one one-shot grant;
7. compare expected publisher against the authenticated publisher;
8. repeat existing requester/publisher/workspace/exact-target currentness checks;
9. only then enter existing freshness/staged-validation/durable-commit ordering.

No step above is implemented by CM.

## 10. Dependency proof

At closed CL:
- `prw-remote-bridge` already has normal dependencies on `prw-core` and `prw-session`;
- manifest blob is `5fd48263be415aac28dee1c71a4031a4a02ad36c`;
- root `Cargo.lock` is `eeacde7ee776d35088f746a6d09f823f3391b82b`;
- Android native `Cargo.lock` is `cce9ca06190a196661ab38d54a747893e26af95f`.

CM therefore requires no Cargo manifest or lockfile change.

Any manifest or lock delta blocks CM closure and requires a fresh audit.

## 11. Focused source validation

Inline unit tests may validate only the provider-neutral source shape and stable fail-closed classification:
- a fail-closed reference provider implements the selected trait without provider I/O;
- absent current authority returns an error, never a grant;
- the grant adapter constructor has the selected owned input shape;
- stable error messages remain bounded and deterministic.

Tests must not introduce a concrete requester/rendezvous store, clock, network service or publication runtime.

## 12. Exact CM scope

The authorized CL -> CM source scope is exactly three paths:
1. `crates/prw-remote-bridge/src/requester_rendezvous_authority.rs`
2. `crates/prw-remote-bridge/src/root.rs`
3. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CM_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AUTHORITY_CARRIER_SOURCE_MATERIALIZATION_STAGING.md`

Any fourth path requires a fresh scope audit before mutation.

No Cargo manifest, lockfile, workflow, Agent/Desktop/Android implementation, concrete provider/database, transport implementation, networking configuration or deployment path is authorized.

## 13. Validation and closure

C03e-CM may close only after:
- exact closed CL predecessor remains unchanged;
- final CL -> CM compare contains exactly the three authorized paths and no fourth path;
- root and Android Cargo locks remain byte-stable;
- all automatically triggered workflows reach terminal non-failing verdicts;
- Rust exact-head validation proves locked graph, formatting, Clippy, tests and workspace build;
- Android PASS is claimed only if Android validation actually triggers and succeeds on the exact final head;
- skipped workflows are recorded as SKIPPED, never PASS;
- immutable Drive audit raw-readback passes;
- rolling Drive predecessor guard and append-only byte-prefix proof pass;
- PR body moves `STAGED -> CLOSED` only after durable Drive verification;
- PR remains draft/open/unmerged.

Any source correction before closure must remain inside the same three authorized paths. A required fourth path stops CM and triggers a fresh scope audit.

## 14. Safe successor rule

After durable C03e-CM closure, no candidate-publication execution source is automatically authorized.

The next checkpoint must begin with a fresh read-only audit from exact closed CM to determine the remaining prerequisite ordering between:
- post-authenticated PRWC stream/Command execution seam selection;
- concrete requester/rendezvous provider representation/lifecycle selection;
- later candidate-publication execution composition.

No successor may jump directly to product runtime activation, listener cutover, production networking, deployment or merge.

## 15. Completion meaning

Closure means only that the CL-selected provider-neutral requester/rendezvous authority carrier and fail-closed authorization port exist in validated bridge source.

It does not mean a requester/rendezvous provider exists, CK exposes Command I/O, candidate publication executes, reachability mutates, a listener is active, product runtime is wired, or anything is deployed.

Target gate:
`C03E_CM_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AUTHORITY_CARRIER_SOURCE_MATERIALIZED`
