# Phase 152 C03e-CS — Candidate Publication Requester/Rendezvous Bounded In-Memory Provider Representation Selection

Status: STAGED SELECTION

Target gate:
`C03E_CS_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_BOUNDED_IN_MEMORY_PROVIDER_REPRESENTATION_SELECTED`

## 1. Exact predecessor

Closed C03e-CR is the authoritative predecessor:
- branch: `phase-152-c03e-cr-candidate-publication-requester-rendezvous-authority-lifecycle-selection-staging`
- head: `337149d4f865365ba52feb80e56e32c6d33fe678`
- tree: `df93dd50d403596f421936e2463398e01a427d99`
- gate: `C03E_CR_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AUTHORITY_LIFECYCLE_SELECTED`
- PR #215: body `Status: CLOSED`, draft/open/unmerged

C03e-CS preserves exact CR source bytes and all CQ/CM authority boundaries.

## 2. Fresh post-CR prerequisite audit

Exact-CR source inspection confirms:
- no concrete `RequesterRendezvousAuthorityProvider` implementation exists;
- `prw-session` already uses a process-local `HashMap`-backed `SessionAuthenticationService` explicitly described as an in-memory authority with explicit lifecycle mutation;
- `prw-registry` already uses bounded in-memory authority state with explicit lifecycle values and explicit capacity enforcement;
- `prw-remote-bridge` already depends on `prw-core` and `prw-session`, and standard-library collections require no new dependency.

Therefore product runtime ownership is not a prerequisite for selecting a bounded library-level provider representation. The representation can be materialized and unit-tested without instantiating it in a listener, process bootstrap, accepted-stream loop, or production network path.

## 3. Selected future source boundary

A future source-materialization checkpoint may introduce one narrow module equivalent to:

`crates/prw-remote-bridge/src/requester_rendezvous_in_memory_provider.rs`

and register/export it through the existing crate root.

The public provider type shall be equivalent to:

`InMemoryRequesterRendezvousAuthorityProvider`

This type is a library-level provider representation only. Creating the type does not select or activate a product runtime owner.

## 4. Process-local and non-durable semantics

The selected provider stores authority state only in process memory.

State is intentionally not durable across process restart or provider drop. C03e-CS selects no persistence recovery, replication, snapshotting, database write, journal, file, broker, or external coordination.

Loss of the provider instance loses its requester/rendezvous authority records. A later runtime checkpoint must decide whether that process-local lifecycle is sufficient for the product runtime that owns it.

## 5. Explicit finite capacity

The provider must be bounded.

C03e-CS does not invent a product-wide numeric capacity. Instead, construction accepts a finite non-zero maximum record count equivalent to:

`new(max_records: usize) -> Result<Self, RequesterRendezvousLifecycleError>`

Rules:
- zero capacity fails before provider construction;
- registration that would exceed the configured bound fails before mutation;
- retirement does not change record count;
- removal of an explicitly retired record decreases record count;
- authorization never changes record count.

Runtime configuration of the actual capacity value remains separately gated.

## 6. Selected private record representation

Each private provider record owns exactly:
- one authenticated requester `AuthenticatedDeviceSession`;
- one expected publisher logical `DeviceId`;
- one private lifecycle value equivalent to `Current | Retired`.

No PRWP payload bytes, `TransportIdentity`, candidate endpoint, candidate ID, publication freshness, socket address, PRWC request ID, database key, timestamp, TTL, or runtime handle belongs in the authority record selected here.

## 7. Private bounded collection

The selected representation is equivalent to a private bounded `Vec<RequesterRendezvousRecord>` plus the configured maximum record count.

The collection is private implementation state. Record ordering has no authority meaning.

A source successor must inspect all relevant records before deciding authorization. It must never treat first insertion, last insertion, vector position, iteration order, or newest-looking data as authority.

## 8. Exact record identity for lifecycle mutation

C03e-CS introduces no new rendezvous identifier.

For exact lifecycle mutation only, a record is identified by the existing pair:
- requester `SessionId` from its authenticated requester session;
- expected publisher `DeviceId`.

This pair is an internal lifecycle selector, not candidate-publication authority and not a new wire field.

The candidate-publication authorization trait remains queried only by expected publisher logical `DeviceId`.

## 9. Registration seam

A future provider shall expose a bounded registration responsibility equivalent to:

`register_current(requester_session: AuthenticatedDeviceSession, expected_publisher_device_id: DeviceId)`

Registration semantics:
- the requester input must already be an `AuthenticatedDeviceSession`;
- the provider does not authenticate raw identity material;
- registration creates one `Current` record;
- exact duplicate record identity already present, whether current or retired, is rejected rather than silently overwritten;
- multiple distinct requester sessions may legitimately target the same expected publisher, so ambiguity remains observable rather than normalized away;
- capacity is checked before insertion;
- no candidate-publication command creates or refreshes a record.

## 10. Retirement seam

A future provider shall expose explicit retirement equivalent to:

`retire(requester_session_id: &SessionId, expected_publisher_device_id: &DeviceId)`

Rules:
- exactly one matching `Current` record transitions to `Retired`;
- absent identity fails without mutation;
- already-retired identity fails distinctly;
- retirement is explicit lifecycle mutation, not wall-clock expiry;
- retirement does not issue a candidate-publication grant and does not write a network response.

## 11. Explicit retired-record removal

Because retired records are retained to preserve `StaleOrRetired` classification and the provider is bounded, the representation selects an explicit maintenance operation equivalent to:

`remove_retired(requester_session_id: &SessionId, expected_publisher_device_id: &DeviceId)`

Rules:
- only an exactly identified `Retired` record may be removed;
- a still-`Current` record cannot be removed through this operation;
- absent identity fails without mutation;
- removal is caller-driven and synchronous;
- no timer, scheduler, background cleanup, TTL, or automatic compaction is selected.

The runtime flow that decides when to retire or remove remains separately gated.

## 12. Lifecycle mutation error surface

Future source materialization shall use a stable provider-lifecycle mutation error surface distinguishing at least:
- invalid zero capacity;
- capacity exhausted;
- exact authority record already exists;
- authority record unknown;
- authority record already retired;
- attempted retired-record removal while record is still current.

These lifecycle mutation errors remain separate from the existing `RequesterRendezvousAuthorityError`, which classifies authorization attempts.

## 13. Existing authorization trait remains authoritative

The provider must implement the existing:

`RequesterRendezvousAuthorityProvider::authorize_current_for_publisher(&mut self, &DeviceId)`

No alternate candidate-publication authorization port is selected.

The authenticated publisher logical `DeviceId` supplied by CQ remains the only provider lookup selector used by candidate-publication execution.

## 14. Exact authorization classification

For one publisher selector, authorization must inspect all provider records for that expected publisher and classify them as follows:

- exactly one `Current` match -> issue one owned `AuthorizedRequesterRendezvous` operation grant;
- more than one `Current` match -> `Ambiguous`;
- zero `Current` matches and one or more `Retired` matches -> `StaleOrRetired`;
- zero matching records -> `Missing`.

The in-memory representation has no selected external dependency that can become indeterminate during a normal intact call, so `UnavailableOrIndeterminate` need not be manufactured artificially. The existing trait variant remains valid for other provider implementations or later representation changes.

## 15. Ambiguity scan precedes grant construction

The provider must not construct or return a grant until it has proven that exactly one current record exists for the publisher selector.

In particular it must not:
- return the first current match;
- return the last current match;
- choose by requester session ID ordering;
- choose by vector position;
- mutate records to make the result unique;
- retire competing records automatically.

Ambiguity remains fail-closed exactly as selected by CR.

## 16. Fresh operation grant from retained authority state

On exactly one current match, the provider may clone the immutable `AuthenticatedDeviceSession` and expected publisher `DeviceId` from its retained authoritative record solely to construct a fresh owned `AuthorizedRequesterRendezvous` operation grant.

This does not weaken the CM rule that the grant itself is neither `Copy` nor `Clone`.

The retained lifecycle record and the one-shot operation grant have different ownership roles:
- provider record = current server-owned lifecycle authority;
- grant = bounded authority snapshot for one CQ execution attempt.

## 17. Authorization is non-consuming

Calling `authorize_current_for_publisher(...)` does not alter record lifecycle, remove a record, decrement capacity, refresh authority, or extend any lifetime.

Successful authorization and failed CQ execution both leave the provider lifecycle record unchanged.

Any later decision to consume/retire authority after a successful publication requires a separate explicit contract and is not selected here.

## 18. No requester registry substitution

Storing an authenticated requester session does not make the in-memory provider authoritative for current workspace/device membership.

The existing reachability owner continues to revalidate the grant requester session against the current registry during candidate-publication commit.

The provider does not replace `WorkspaceDeviceRegistry` and does not cache registry-validation outcomes.

## 19. No publisher authentication substitution

The expected publisher `DeviceId` stored in a provider record is a server-side target binding, not proof that the publisher is authenticated.

CQ continues to obtain the actual publisher identity only from the authenticated connection and continues to require grant expected-publisher equality before reachability commit.

## 20. No clock or TTL

The selected provider contains no clock source, timestamp, deadline, TTL, expiration duration, timer wheel, scheduled job, or background retirement.

`Retired` is explicit lifecycle state only.

A later checkpoint must separately select any time-based staleness policy before source may depend on one.

## 21. No synchronization primitive selected

The provider representation itself selects no mutex, RW lock, channel, actor, atomic protocol, transaction, or async synchronization primitive.

C03e-CS imposes no `Send`/`Sync` requirement beyond what naturally follows from its fields. Runtime concurrency ownership must be selected later.

No global/static singleton is selected.

## 22. No runtime ownership selected

C03e-CS does not choose which product process owns the provider instance, how long that instance lives, or how requester registration and candidate-publication execution reach the same instance.

It selects no:
- listener;
- accepted-stream loop;
- Tokio runtime/task;
- daemon/service;
- Desktop/Agent/Android owner;
- dependency-injection container;
- bootstrap global;
- production credentials;
- networking endpoint.

That wiring remains a later prerequisite before product activation.

## 23. No response or frame-loop composition

C03e-CS does not select candidate-publication Response/Error mapping, response payload, request-ID echo behavior, write failure handling, retry, reconnect, or command loop.

Outer PRWC request ID remains peer-originated correlation only.

## 24. No new dependency

The representation uses existing `prw-core`, `prw-session`, existing requester/rendezvous carrier types, and standard-library collection facilities.

C03e-CS selects no Cargo manifest or lockfile change.

Any later dependency request blocks source materialization unless a fresh compiler/source audit proves it unavoidable.

## 25. Focused source-materialization tests selected for later checkpoint

A later source checkpoint must include focused tests proving at least:
1. zero capacity is rejected;
2. configured capacity is enforced before mutation;
3. exact duplicate identity is rejected;
4. one current record returns one grant with exact requester session and expected publisher;
5. two distinct current requesters for one publisher return `Ambiguous` without record mutation;
6. only retired matches return `StaleOrRetired`;
7. no matches return `Missing`;
8. authorization does not consume/retire/remove current state;
9. explicit retirement transitions only the exact record;
10. current records cannot be removed through `remove_retired`;
11. retired removal frees one capacity slot;
12. vector/insertion order never resolves ambiguity.

Tests must not require network listeners, clocks, databases, threads, or product runtime startup.

## 26. Exact C03e-CS diff boundary

C03e-CS is docs-only.

The exact CR -> CS diff is authorized to contain only:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CS_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_BOUNDED_IN_MEMORY_PROVIDER_REPRESENTATION_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, runtime/listener, networking, deployment, database/provider source implementation, or unrelated contract blocks CS closure.

## 27. Explicitly rejected shortcuts

C03e-CS rejects:
- unbounded authority record growth;
- a hardcoded product capacity selected without evidence;
- arbitrary-current-record selection under ambiguity;
- hidden replacement of an existing record on registration;
- automatic retirement on authorization;
- automatic deletion on grant issuance;
- clock-derived staleness;
- background cleanup;
- requester authority derived from publisher payload or transport state;
- bypassing current requester registry revalidation in the reachability owner;
- weakening CQ expected-publisher equality;
- adding a new rendezvous wire identifier;
- global mutable provider state;
- selecting concurrency/runtime ownership implicitly;
- response/frame-loop integration;
- listener/runtime activation;
- production networking;
- deployment or merge.

## 28. Safe successor after durable CS closure

After C03e-CS is durably closed, a fresh exact-head audit may authorize one bounded source-materialization checkpoint equivalent to:
1. a source-materialization contract;
2. `crates/prw-remote-bridge/src/requester_rendezvous_in_memory_provider.rs`;
3. `crates/prw-remote-bridge/src/root.rs` only for module registration/export.

That successor should require no manifest/lock change and must keep runtime ownership unselected.

After provider source validates, a new prerequisite audit is required before choosing runtime ownership/wiring versus candidate-publication Response/Error-frame composition.

No successor may jump directly to listener/runtime activation, production networking, deployment or merge.

## 29. Closure requirements

C03e-CS may close only if one exact final head proves:
1. exact CR merge base and one bounded docs-only commit;
2. no source/manifest/lock/workflow/runtime/network/deployment path changed;
3. canonical automatically-triggered validation is terminal and non-failing; non-applicable workflows are recorded as `SKIPPED`, not `PASS`;
4. root and Android-native lock blobs remain byte-stable;
5. immutable Drive audit is raw-read back exactly;
6. rolling Drive predecessor is freshly guarded and preserved byte-for-byte as prefix;
7. PR body changes to `Status: CLOSED` only after durable evidence succeeds;
8. PR remains draft/open/unmerged.

Until those conditions are satisfied, C03e-CS remains staged and selects no production behavior.