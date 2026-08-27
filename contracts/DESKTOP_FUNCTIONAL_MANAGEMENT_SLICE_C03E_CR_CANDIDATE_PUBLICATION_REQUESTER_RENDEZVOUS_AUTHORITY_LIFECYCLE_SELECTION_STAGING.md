# Phase 152 C03e-CR — Candidate Publication Requester/Rendezvous Authority Lifecycle Selection

Status: STAGED SELECTION

Target gate:
`C03E_CR_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AUTHORITY_LIFECYCLE_SELECTED`

## 1. Exact predecessor

Closed C03e-CQ is the authoritative predecessor:
- branch: `phase-152-c03e-cq-candidate-publication-provider-neutral-execution-composition-source-materialization-staging`
- head: `231cc3a313a798534875a20a71c236a8ca945f9f`
- tree: `72890ef9fb0b29a5bda42793d48b82173f23a8c9`
- gate: `C03E_CQ_CANDIDATE_PUBLICATION_PROVIDER_NEUTRAL_EXECUTION_COMPOSITION_SOURCE_MATERIALIZED`
- PR #214: body `Status: CLOSED`, draft/open/unmerged

C03e-CR preserves exact CQ source bytes. It does not amend the CQ candidate-publication execution helper, the CM requester/rendezvous authority carrier, the CO post-auth receive seam, reachability-owner semantics, request-ID custody, or any runtime/listener behavior.

## 2. Fresh post-CQ prerequisite-order audit

At exact closed CQ head, one bounded candidate-publication semantic execution is already provider-neutral and complete once a trustworthy current requester/rendezvous authority provider is supplied:

1. authenticated publisher logical identity comes only from `AuthenticatedPrwcConnection::session()`;
2. typed transport/freshness/candidates come only from the already-received candidate-publication submission;
3. `publish_current_candidates(...)` performs current publisher/session/transport/candidate admission;
4. `RequesterRendezvousAuthorityProvider::authorize_current_for_publisher(...)` is called exactly once using the authenticated logical publisher `DeviceId`;
5. the returned owned `AuthorizedRequesterRendezvous` must name that exact publisher;
6. the grant requester session is the only requester authority passed to `ProductionReachabilityOwner::commit_candidate_publication(...)`;
7. the existing reachability owner retains durable admission/currentness/commit authority.

The remaining prerequisite before product runtime can instantiate that composition is not response encoding or frame looping. It is a trustworthy server-owned lifecycle/representation capable of deciding whether a current requester/rendezvous authority exists for the authenticated publisher selector.

Historical BZ, CL and CM deliberately left the concrete provider/lifecycle unresolved while fixing the authority semantics. Fresh exact-CQ repository inspection finds the generic authority port and carrier but no concrete requester-awaits-publisher provider/lifecycle implementation to reuse.

Therefore C03e-CR selects the minimum lifecycle/authority responsibilities required before later source materialization. It does not select a storage technology, runtime owner, wall clock, TTL, lock, broker, schema, cleanup scheduler or deployment topology.

## 3. Selected authority record meaning

A requester/rendezvous authority record represents server-owned evidence that one authenticated requester is currently waiting for one expected publisher logical device.

Its authority-bearing semantic fields are exactly:
- an authenticated requester `AuthenticatedDeviceSession`;
- an expected publisher logical `DeviceId`.

The expected publisher `DeviceId` is the lookup/selection dimension used by the existing `RequesterRendezvousAuthorityProvider` port.

No candidate-publication payload field is permitted to create, replace or strengthen requester authority.

## 4. Logical identities remain separated

C03e-CR preserves these distinct meanings:
- requester `AuthenticatedDeviceSession` = requester logical PRW authority;
- expected publisher `DeviceId` = logical publisher target selected by the server-owned rendezvous lifecycle;
- authenticated publication publisher `DeviceId` = logical publisher actually authenticated on the candidate-publication connection;
- PRWP `TransportIdentity` = lower transport identity requiring current registry validation;
- `CandidatePublicationFreshnessToken` = verifier-owned candidate-publication currentness/replay state;
- `CandidateId` = candidate-plan correlation identity only;
- PRWC `request_id` = peer-originated message correlation only.

None may substitute for another.

## 5. Current-authority selection responsibility

A later lifecycle provider must be able to evaluate the authoritative requester/rendezvous state for one expected publisher logical `DeviceId` and classify the result into the existing CM fail-closed surface:
- exactly one usable current authority -> return one owned `AuthorizedRequesterRendezvous`;
- no matching authority -> `Missing`;
- matching authority exists but is no longer usable/current -> `StaleOrRetired`;
- more than one state candidate prevents a unique current authority decision -> `Ambiguous`;
- provider cannot determine current authority reliably -> `UnavailableOrIndeterminate`.

C03e-CR does not weaken or merge these classifications.

## 6. Ambiguity remains fail-closed

C03e-CR does not invent a repository-wide invariant that only one requester record can ever exist for a publisher.

The lifecycle may internally prevent ambiguity, but the externally authoritative behavior remains fail-closed if a unique current requester authority cannot be established.

A source successor must not pick an arbitrary requester from multiple plausible current records.

Ordering, map iteration, insertion order, newest-looking metadata, request ID, transport identity or candidate data must not break an authority tie.

## 7. Current versus stale/retired is provider-owned authority state

The lifecycle must distinguish a usable current authority from authority that has been retired, abandoned, superseded, expired according to a later-selected policy, or otherwise made unusable by authoritative server state.

C03e-CR selects only the semantic distinction, not the mechanism that creates it.

This checkpoint does not choose:
- a TTL duration;
- a wall-clock or monotonic-clock source;
- timestamp serialization;
- cleanup cadence;
- abandonment timeout;
- process-lifetime semantics;
- persistence duration;
- a database expiration feature.

A later checkpoint must make any such mechanism explicit before relying on it.

## 8. Lifecycle mutation authority is separate from candidate publication

Candidate publication itself must not create or refresh requester/rendezvous authority.

The requester/rendezvous lifecycle is established by a separately authenticated server-side flow. The existing candidate-publication execution path only queries/linearizes that authority through `authorize_current_for_publisher(...)`.

Consequently a later provider must not treat a publisher command, candidate endpoint, presented transport identity, freshness token or outer request ID as registration/refresh input for requester authority.

## 9. Authenticated requester requirement

Any state eligible to become an `AuthorizedRequesterRendezvous` must carry a requester session that was authenticated through the existing PRW session authority model.

A bare `DeviceId`, `UserId`, workspace ID, transport identity, connection address, token string or unvalidated session-like payload is insufficient.

The generic CM grant continues to carry an `AuthenticatedDeviceSession`, and CQ continues to pass only that grant session into reachability admission.

## 10. Expected publisher binding

Each eligible requester/rendezvous authority must bind to one exact expected publisher logical `DeviceId`.

Provider lookup receives the authenticated candidate publisher `DeviceId` only as a selector. Selection success produces a grant whose `expected_publisher_device_id()` remains explicit so CQ can perform the existing equality check before any reachability commit.

C03e-CR does not remove that CQ equality check even if a future provider is internally keyed by publisher `DeviceId`.

## 11. Linearization boundary

The provider must linearize its own current-authority decision sufficiently that one call to `authorize_current_for_publisher(...)` either:
- returns one owned grant representing a determinate current authority decision; or
- returns one existing fail-closed authority error.

The returned grant is the operation-level authority snapshot consumed by the CQ execution attempt.

No provider lock/guard/transaction handle is exposed through the grant or retained across the later reachability-owner durable commit.

C03e-CR selects the semantic linearization requirement only. It does not select a mutex, RW lock, actor, transaction, compare-and-swap primitive, database isolation level or distributed consensus mechanism.

## 12. One-shot operation grant remains distinct from lifecycle consumption

`AuthorizedRequesterRendezvous` remains an owned one-shot operation grant and remains neither `Copy` nor `Clone`.

C03e-CR does not infer that issuing one grant must automatically delete, retire or consume the underlying requester/rendezvous lifecycle record. Underlying record consumption semantics require a separately explicit selection because BZ/CL/CM fixed only the one-shot operation grant semantics.

A source successor must therefore not silently add record consumption as a side effect of authorization unless a later contract selects it.

## 13. No guard survives authorization

The provider must not require CQ to retain a mutable provider guard across:
- expected-publisher equality checking;
- reachability-owner validation;
- durable candidate publication commit;
- local reachability installation;
- traversal invalidation;
- any future response write.

Authority is transferred into the owned grant for the bounded execution attempt. This preserves the CM/BZ ownership boundary and prevents unrelated durable work from extending provider synchronization scope.

## 14. Existing CQ ordering remains authoritative

CR does not reorder execution.

The required order remains:
1. current publisher/session/transport/candidate construction;
2. exactly one requester/rendezvous current-authority selection;
3. exact expected-publisher equality;
4. exactly one existing reachability-owner commit.

Requester authority must not be requested before publisher/transport publication construction succeeds.

No later stage runs after an earlier failure.

## 15. No response/Error-frame semantics selected

C03e-CR does not select:
- PRWC Response versus Error outer kind;
- response payload representation;
- status/error code mapping;
- whether replacement freshness is returned;
- response write timing;
- connection terminalization after write failure;
- retry/reconnect behavior.

Those remain a separate post-provider prerequisite.

The inbound PRWC `request_id` remains available only as peer-originated correlation for that future response composition.

## 16. Request ID remains non-authoritative

The requester/rendezvous lifecycle must not use inbound candidate-publication `request_id` as:
- requester identity;
- publisher identity;
- rendezvous identity;
- lifecycle version;
- currentness marker;
- replay authority;
- freshness state;
- storage primary key selected from publisher input.

C03e-CR allocates no new request ID and changes no local `PrwcRequestIdLifecycle` behavior.

## 17. No transport-derived authority

`TransportIdentity`, candidate addresses/endpoints, socket peer address and connection ownership do not establish requester/rendezvous authority.

Transport identity remains separately validated by existing publisher reachability admission. The requester authority provider remains keyed by logical expected publisher `DeviceId` and returns an authenticated requester session.

## 18. No storage/backend technology selected

C03e-CR deliberately does not select:
- in-memory versus durable storage;
- SQL/NoSQL/etcd/Redis or another product;
- a table/key/value schema;
- serialization format;
- database key layout;
- transaction model;
- lock primitive;
- broker/topic/queue;
- cache;
- filesystem state;
- distributed coordination service.

A later source/materialization checkpoint may only choose a representation that is justified by a fresh audit and the product lifecycle that will own it.

## 19. No clock/TTL/cleanup policy selected

The semantic error `StaleOrRetired` remains valid, but C03e-CR does not define staleness using time.

No TTL, timestamp, deadline, timer wheel, periodic cleanup task, background worker or scheduled retirement is authorized here.

A provider may only classify stale/retired according to authoritative lifecycle state selected and materialized by a later checkpoint.

## 20. No process/runtime ownership selected

C03e-CR does not decide which process/component owns requester-awaits-publisher state at product runtime.

It does not activate or select:
- a listener;
- an accepted-stream loop;
- a Tokio/async task;
- a desktop process;
- an Agent process;
- an Android component;
- a daemon/service;
- bootstrap dependency wiring;
- runtime credentials;
- production networking.

Runtime ownership remains separately gated after a concrete lifecycle representation exists and validates.

## 21. No dependency selected

C03e-CR is docs-only and selects no Cargo dependency, feature, manifest edit or lockfile change.

The existing generic carrier/provider port remains sufficient to express the selected lifecycle semantics.

Any later dependency request must be justified by a fresh source-materialization audit rather than assumed from this contract.

## 22. Exact C03e-CR diff boundary

The exact CQ -> CR diff is authorized to contain only:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CR_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AUTHORITY_LIFECYCLE_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, concrete database/provider implementation, runtime/listener source, networking configuration, deployment path or unrelated contract blocks CR closure.

## 23. Explicitly rejected shortcuts

C03e-CR rejects:
- deriving requester authority from PRWP bytes;
- treating publisher authentication as requester authority;
- deriving requester authority from `TransportIdentity` or candidate endpoints;
- using outer PRWC `request_id` as rendezvous/currentness authority;
- accepting an arbitrary caller-supplied requester session;
- selecting one arbitrary record when provider state is ambiguous;
- suppressing `StaleOrRetired` into `Missing` solely for convenience;
- holding a provider guard across reachability-owner durable work;
- automatically consuming provider lifecycle state merely because the operation grant is one-shot;
- selecting a hidden TTL/clock/cleanup policy;
- choosing a database or broker without a separate gate;
- combining provider lifecycle, response encoding and frame loop into one mutation;
- listener/runtime activation;
- production networking changes;
- deployment or merge.

## 24. Safe successor after durable CR closure

After C03e-CR is durably closed, the next checkpoint must begin with a fresh exact-head audit before any source mutation.

The intended question for that audit is whether the selected lifecycle can be materialized as a bounded bridge-owned provider representation using existing dependencies and authority types, or whether product runtime ownership must first be selected more explicitly.

A source-materialization successor, if justified, must preserve:
- the existing CM `RequesterRendezvousAuthorityProvider` public authority port;
- the existing one-shot `AuthorizedRequesterRendezvous` carrier semantics;
- CQ execution ordering and expected-publisher equality;
- existing reachability-owner authority;
- request-ID custody;
- fail-closed ambiguity/unavailable behavior.

Response/Error-frame composition remains separately gated after the requester/rendezvous authority prerequisite is trustworthy enough to instantiate CQ in a product runtime.

No successor may jump directly to listener/runtime activation, production networking, deployment or merge.

## 25. Closure requirements

C03e-CR may close only if all of the following hold on one exact final head:
1. CQ remains the exact merge base and CR is ahead only by the bounded docs-only contract commit;
2. no source, manifest, lockfile, workflow, runtime, networking or deployment path changed;
3. canonical automatically-triggered validation is terminal and non-failing for the exact head; non-applicable workflows are recorded as `SKIPPED`, not `PASS`;
4. root and Android-native lock blobs remain byte-stable;
5. an immutable Drive audit is written and raw-read back exactly;
6. the rolling Drive ledger is freshly guarded, appended only if its predecessor bytes remain exact, and raw-read back with exact predecessor-prefix proof;
7. the CR PR body is changed to `Status: CLOSED` only after durable evidence succeeds;
8. the PR remains draft/open/unmerged.

Until those conditions are satisfied, this checkpoint remains staged and selects no production behavior.