# Phase 152 C03e-CL — Candidate Publication Requester/Rendezvous Authority Carrier Selection

Status: STAGED SELECTION

Target gate:
`C03E_CL_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AUTHORITY_CARRIER_SELECTED`

## 1. Exact predecessor

Closed C03e-CK is the authoritative predecessor:
- branch: `phase-152-c03e-ck-prwc-connection-authentication-execution-source-materialization-staging`
- head: `139a1a049139c05d3cb8677eea5cf81d78dfecf4`
- tree: `6cd1f2c61288af2b45b08dd65fb591c25cdd1a6e`
- gate: `C03E_CK_PRWC_CONNECTION_AUTHENTICATION_EXECUTION_SOURCE_MATERIALIZED`
- PR #208: body `Status: CLOSED`, draft/open/unmerged

C03e-CL preserves exact CK lineage. It does not amend CK authentication semantics or expose CK's retained `ControlTlsServerStream`.

## 2. Historical semantic authority retained

C03e-BZ remains authoritative for candidate-publication requester/rendezvous semantics:
- branch: `phase-152-c03e-bz-candidate-publication-prwc-pre-mesh-authentication-requester-rendezvous-authority-selection-staging`
- head: `cc226a27b2e404024e4ef6fd8ea089ffff33c2d6`
- gate: `C03E_BZ_CANDIDATE_PUBLICATION_PRWC_PRE_MESH_AUTHENTICATION_REQUESTER_RENDEZVOUS_AUTHORITY_SELECTED`

BZ requires server-side authority produced independently of publisher-controlled data and carrying semantically:
1. one authenticated requester session snapshot;
2. one expected publisher logical `DeviceId`;
3. provenance that this requester currently requested/awaits reachability for that expected publisher in requester workspace context.

BZ explicitly leaves concrete provider/storage representation, staleness/abandonment representation, routing, broker and database unselected.

C03e-CL resolves only the provider-neutral authority-carrier and authorization boundary needed before candidate-publication execution source may be designed. It does not select a concrete provider.

## 3. Fresh post-CK gap audit

At exact closed CK head, the required surrounding primitives exist:
- `AuthenticatedPrwcConnection::session()` exposes the server-produced authenticated publisher session while retaining the post-auth stream privately;
- C03e-BV's `candidate_publication_control_frame` strictly decodes/encodes PRWP inside existing Phase 129 `Command` frames and treats PRWC request ID as outer correlation only;
- `publish_current_candidates(...)` derives publisher logical identity from an authenticated session and revalidates presented `TransportIdentity`;
- `validate_authenticated_publication_admission(...)` requires requester session separately and revalidates requester/publisher/workspace/exact-target currentness;
- `ProductionReachabilityOwner::commit_candidate_publication(...)` owns freshness, staged candidate validation and durable compare-and-commit ordering.

No current `prw-remote-bridge` or `prw-control-plane` source object represents BZ's requester-awaits-publisher rendezvous authority. The Phase 143 capability bridge authorizes capability requests but does not establish requester reachability rendezvous provenance and is not reusable as this authority.

Therefore candidate-publication execution is still blocked by one explicit authority carrier. Direct source execution before this carrier exists would either invent hidden requester state or accept requester authority from publisher-controlled data, both forbidden by BZ.

## 4. Selected ownership boundary

The provider-neutral requester/rendezvous semantic boundary is bridge-owned because it is consumed by the bridge's candidate-publication composition between authenticated PRWC connection state and existing reachability semantics.

Future source materialization shall introduce responsibilities equivalent to:
- `AuthorizedRequesterRendezvous`
- `RequesterRendezvousAuthorityProvider`
- `RequesterRendezvousAuthorityError`

Names may remain exact unless compiler/repository conventions require a narrowly equivalent form. Their semantics below are authoritative.

The concrete provider implementation remains outside C03e-CL and is not selected by this checkpoint.

## 5. Selected owned authorization grant

`AuthorizedRequesterRendezvous` is one owned, server-produced authorization grant for exactly one candidate-publication execution attempt.

Its semantic content is exactly:
- one `AuthenticatedDeviceSession` for the requester;
- one expected publisher logical `DeviceId`.

The fact that the value was produced by the selected provider authorization boundary is the rendezvous provenance. No publisher-supplied provenance string, route field, correlation token or opaque byte claim is added.

The grant does not contain or derive authority from:
- PRWP payload bytes;
- PRWC `request_id`;
- `TransportIdentity`;
- `CandidateId`;
- endpoint or path kind;
- publication freshness token;
- IP/port/socket identity;
- clock time;
- process/thread/task identity.

The requester workspace remains the workspace carried by the authenticated requester session and must be revalidated against current registry state during semantic admission. C03e-CL does not duplicate a separate workspace field.

## 6. One-shot semantics

The authorized rendezvous value is operation evidence, not a reusable lease or bearer credential.

Selected rules:
- one grant is consumed by at most one candidate-publication execution attempt;
- it must not be cached and reused for a later PRWC Command;
- it must not be serialized onto PRWP/PRWC wire;
- it must not be accepted from a remote peer;
- source materialization should not make the grant `Copy` or otherwise encourage implicit duplication;
- cloning the underlying authenticated requester session elsewhere does not clone rendezvous authority;
- failure of the candidate-publication attempt does not silently manufacture a replacement grant.

C03e-CL does not select a reusable rendezvous token, generation number or new correlation identifier.

## 7. Selected provider-neutral authorization port

The future provider-neutral port performs one current server-side authorization lookup/linearization for an already-authenticated logical publisher device.

Its responsibility is equivalent to:

```text
authorize_current_for_publisher(
    publisher_device_id: &DeviceId
) -> Result<AuthorizedRequesterRendezvous, RequesterRendezvousAuthorityError>
```

The publisher `DeviceId` argument is a lookup selector only. It must come from the CK authenticated publisher session. Supplying a `DeviceId` to the provider does not itself create requester authority.

The provider must return a grant only when exactly one current authoritative requester/rendezvous selection exists for that publisher according to the concrete provider's later-selected lifecycle semantics.

The returned grant's `expected_publisher_device_id` must still be compared with the publisher device derived from the authenticated publisher session by later runtime composition. Provider lookup does not eliminate that independent equality check.

## 8. Fail-closed provider classification

The provider-neutral error surface must distinguish at least these stable authority outcomes:
- no current requester/rendezvous selection;
- stale, abandoned or retired selection;
- ambiguous/multiple current authoritative selections where exactly one is required;
- provider authority unavailable or indeterminate.

All four classes fail closed before reachability mutation.

A concrete provider may have richer internal failure detail, but it must not map absence, staleness, ambiguity or indeterminate authority to an authorized grant.

C03e-CL does not select retries, fallback provider order, eventual-consistency acceptance or optimistic default authority.

## 9. Authorization linearization

C03e-CL reuses the already-selected Phase 152 shared-current authorization principle: current authority is checked at one bounded authorization linearization point, producing owned operation evidence; internal authority guards are not carried into unrelated side effects.

Selected semantics:
- a concrete provider determines currentness atomically according to its own later-selected synchronization/storage boundary;
- successful provider authorization linearizes one requester/rendezvous admission and returns the owned one-shot grant;
- a provider mutation that linearizes before authorization must be observed by that authorization;
- once authorization has successfully produced the one-shot owned grant, a later provider mutation does not retroactively revoke that already-admitted single execution attempt;
- the grant cannot authorize a second or later candidate-publication attempt;
- no provider lock/transaction guard is selected to remain held across PRWC frame I/O, candidate validation, durable reachability commit, response I/O or network side effects.

This defines operation-level authority without selecting a lock, database transaction, broker reservation or lease implementation.

## 10. Required future candidate-publication ordering

A later execution checkpoint must preserve this order:

1. begin from one CK-authenticated PRWC connection;
2. receive one bounded Phase 129 frame under a separately selected post-auth stream execution seam;
3. require the existing `Command` kind and strictly decode the existing PRWP payload through the BV adapter;
4. obtain the publisher `AuthenticatedDeviceSession` from the CK connection-local authenticated binding;
5. call existing `publish_current_candidates(...)` so publisher session and presented transport identity are registry-current before an authenticated publication exists;
6. call the selected requester/rendezvous authority provider using only the authenticated publisher logical `DeviceId` as lookup selector;
7. require exactly one returned one-shot `AuthorizedRequesterRendezvous`;
8. require grant expected publisher `DeviceId` equals the authenticated publisher device;
9. revalidate requester and publisher currentness, same-workspace membership and exact publication target through existing candidate/reachability authorities;
10. only then allow existing publication freshness, staged candidate validation and durable compare-and-commit ordering to execute;
11. any failure before durable commit produces no reachability mutation.

Provider authorization does not replace existing registry/current-plan/freshness checks. Revalidation overlap is intentional.

## 11. CK stream boundary remains closed

C03e-CK intentionally retains its `ControlTlsServerStream` privately after authentication and states that later `Command` execution is separately gated.

C03e-CL does not expose that stream, add read/write methods, add a frame loop, or choose whether candidate-publication execution consumes, borrows or owns the authenticated connection.

That post-auth connection execution seam remains a separate selection/materialization checkpoint after the authority carrier exists in source.

## 12. No concrete provider selected

C03e-CL does not select:
- in-memory map, `Mutex`, `RwLock`, actor or channel;
- etcd, Spanner, PostgreSQL, CockroachDB, Redis or another database;
- broker/topic/queue/service;
- persistence key/schema;
- TTL, wall-clock expiry or heartbeat policy;
- multi-region replication;
- requester scheduling algorithm;
- fairness/priority policy;
- route discovery;
- network API;
- process placement;
- retry/reconciliation policy.

A later concrete-provider checkpoint is required before a production provider implementation can be materialized.

## 13. No new dependency required for the carrier

The selected provider-neutral source carrier needs only already-present typed dependencies:
- `prw_core::DeviceId`;
- `prw_session::AuthenticatedDeviceSession`.

`prw-remote-bridge` already depends on both crates at closed CK. No Cargo manifest or lockfile change is selected for carrier source materialization.

## 14. Identity separation

C03e-CL preserves all existing non-interchangeable identities:
- `AuthenticatedDeviceSession` / `DeviceId` = logical authenticated PRW identity;
- `SessionId` = authentication/session identity inside its own authority;
- `TransportIdentity` = lower transport certificate identity;
- PRWC `request_id` = one-connection outer correlation only;
- `CandidateId` = candidate-plan correlation only;
- publication freshness token = verifier-owned currentness/replay state only.

The requester/rendezvous grant is server-side authorization evidence for one operation. It is not a new user/device/session/transport identity.

## 15. Explicitly rejected shortcuts

C03e-CL rejects:
- deriving requester identity from publisher PRWP fields;
- adding requester `DeviceId`, `SessionId`, workspace or rendezvous authority to PRWP;
- treating PRWC request ID as requester/rendezvous correlation authority;
- treating `TransportIdentity` as requester or logical publisher identity;
- using the publisher's own authenticated session as requester authority;
- accepting the first/current workspace member as implicit requester;
- selecting a reachability owner solely from publisher input without independent server-side rendezvous authority;
- caching an authorized grant for multiple publications;
- holding an unspecified provider guard across durable reachability mutation;
- inventing a rendezvous bearer token solely to bridge the gap;
- directly exposing CK's retained stream in this docs-only checkpoint.

## 16. Audit-basis source remains byte-stable

The CL selection is based on these exact closed-CK authorities:
- BZ selection contract blob `9d08f0d98857f564a642d3f9cb4c3a3f0a699fc3`;
- `crates/prw-remote-bridge/src/prwc_connection_authentication.rs` blob `952ad7e8d0027e2acc8d05b6526b4ebaf8212e69`;
- `crates/prw-remote-bridge/src/candidate_publication_control_frame.rs` blob `20ff7d2bc5f32596a3c0696aa387e6735f8f2031`;
- `crates/prw-remote-bridge/src/candidate_reachability.rs` blob `51b294cfb3772925651a05bdcb034cd051204efb`;
- `crates/prw-remote-bridge/src/reachability_owner.rs` blob `8d0e65c3fc0bd646c257199d4f55be65fa3f792d`;
- `crates/prw-session/src/lib.rs` blob `0b0b6624df93ebcf3efae632d94dfc337ee67761`;
- `crates/prw-registry/src/lib.rs` blob `cd7eb52eea19354620ff8ad26c9b8aad3f9c5eb6`.

No source authority above may change in CL.

## 17. Exact CL diff boundary

C03e-CL is docs-only.

The exact CK -> CL diff is authorized to contain only:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CL_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AUTHORITY_CARRIER_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, Agent/Desktop/Android implementation, provider/database file, transport implementation, networking configuration or deployment path blocks CL closure.

## 18. Safe successor rule

After durable C03e-CL closure, the next safe checkpoint may materialize only the provider-neutral requester/rendezvous authority carrier/port in `prw-remote-bridge` plus its root exposure and contract, using existing dependencies and no concrete provider.

Candidate-publication PRWC Command execution remains separately gated after that carrier source exists and is validated.

Concrete rendezvous provider/storage/runtime placement remains separately gated still.

No successor may jump directly to product runtime activation, live listener cutover, networking, deployment or merge.

## 19. Validation and closure

C03e-CL may close only after:
- exact closed CK predecessor lineage remains unchanged;
- CK -> CL compare is ahead 1 / behind 0 with exact CK merge base and exactly one docs-only path;
- all audit-basis source blobs remain byte-stable;
- root and Android Cargo locks remain byte-stable;
- every automatically triggered workflow reaches a terminal non-failing verdict;
- skipped workflows are recorded as SKIPPED, never PASS;
- immutable Drive audit is uploaded and raw-readback verified;
- rolling Drive predecessor guard and append-only byte-prefix proof pass;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged.

## 20. Completion meaning

Closure means only that the provider-neutral, one-shot requester/rendezvous authorization carrier and its fail-closed current-authority semantics are selected.

It does not mean the carrier exists in Rust source, a concrete rendezvous provider exists, CK exposes Command I/O, candidate publication executes, reachability state mutates, a listener is activated, product runtime is wired, or anything is deployed.

Target gate:
`C03E_CL_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AUTHORITY_CARRIER_SELECTED`
