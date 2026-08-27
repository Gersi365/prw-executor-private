# Phase 152 C03e-CT — Candidate Publication Requester/Rendezvous Bounded In-Memory Provider Source Materialization

Status: STAGED SOURCE MATERIALIZATION

Target gate:
`C03E_CT_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_BOUNDED_IN_MEMORY_PROVIDER_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Closed C03e-CS is the authoritative predecessor:
- branch: `phase-152-c03e-cs-candidate-publication-requester-rendezvous-bounded-in-memory-provider-representation-selection-staging`
- head: `3704f3a3d83266cdc0b399fc8ad3f4741d31f792`
- tree: `14eef8653866f437feba8f0ca3d7fb3c777e3789`
- gate: `C03E_CS_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_BOUNDED_IN_MEMORY_PROVIDER_REPRESENTATION_SELECTED`
- PR #216: body `Status: CLOSED`, draft/open/unmerged

CT must preserve all exact CS source bytes except the explicitly authorized source registration and new provider module below.

## 2. Fresh post-CS prerequisite audit

Exact closed-CS inspection confirms:
- `requester_rendezvous_authority.rs` already owns the one-shot `AuthorizedRequesterRendezvous`, stable authorization errors, and the provider-neutral `RequesterRendezvousAuthorityProvider` trait;
- no concrete provider implementation exists at exact CS;
- the CS contract explicitly selected a future module `requester_rendezvous_in_memory_provider.rs` and root registration;
- `prw-remote-bridge` already depends on `prw-core` and `prw-session` and already has dev dependencies used to construct authenticated test sessions;
- no manifest or lock change is required;
- runtime/process ownership is not a prerequisite for unit-testing the library-level provider representation.

Therefore CT materializes only the selected bounded in-memory representation and focused tests. It does not select product runtime wiring.

## 3. Exact authorized paths

CT is authorized to change exactly three paths:
1. this contract;
2. new `crates/prw-remote-bridge/src/requester_rendezvous_in_memory_provider.rs`;
3. `crates/prw-remote-bridge/src/root.rs` only to add `pub mod requester_rendezvous_in_memory_provider;` in canonical module order.

No other path is authorized absent a new concrete compiler contradiction.

## 4. Materialized public surface

The source module shall expose:
- `InMemoryRequesterRendezvousAuthorityProvider`;
- `RequesterRendezvousLifecycleError`;
- `InMemoryRequesterRendezvousAuthorityProvider::new(max_records)`;
- `register_current(requester_session, expected_publisher_device_id)`;
- `retire(requester_session_id, expected_publisher_device_id)`;
- `remove_retired(requester_session_id, expected_publisher_device_id)`;
- implementation of the existing `RequesterRendezvousAuthorityProvider` trait.

No alternate candidate-publication authorization trait is introduced.

## 5. Exact bounded representation

Provider state remains private and process-local:
- configured finite non-zero `max_records`;
- private bounded `Vec` of authority records;
- each record owns authenticated requester `AuthenticatedDeviceSession`, exact expected publisher `DeviceId`, and private lifecycle `Current | Retired`.

The provider contains no persistence handle, clock, timestamp, TTL, lock, runtime handle, transport identity, request ID, PRWP payload, candidate endpoint, or reachability state.

## 6. Constructor and capacity semantics

`new(0)` fails with a distinct lifecycle error before construction.

Registration:
- rejects an exact record identity already present, whether current or retired;
- rejects capacity exhaustion before insertion;
- otherwise inserts one new `Current` record;
- never overwrites or retires another record to make room.

The product value of `max_records` remains runtime configuration and is not selected by CT.

## 7. Exact lifecycle identity

Lifecycle mutation identifies one record only by:
- requester authenticated `SessionId`;
- expected publisher logical `DeviceId`.

This pair is internal lifecycle selection only and is not a wire identifier or candidate-publication authority input.

## 8. Retirement and retired removal

`retire(...)`:
- exact current record -> `Retired`;
- exact retired record -> distinct already-retired error;
- absent identity -> unknown-record error;
- no other record changes.

`remove_retired(...)`:
- exact retired record -> removed synchronously, freeing one capacity slot;
- exact current record -> distinct current-cannot-be-removed error;
- absent identity -> unknown-record error;
- no implicit retirement or cleanup occurs.

## 9. Full-scan authorization

`authorize_current_for_publisher(&mut self, publisher_device_id)` must inspect the complete bounded set relevant to that publisher before classification.

Classification remains exactly:
- exactly one current match -> fresh owned `AuthorizedRequesterRendezvous`;
- more than one current match -> `Ambiguous`;
- zero current plus one or more retired matches -> `StaleOrRetired`;
- no match -> `Missing`.

The provider must not use first/last insertion, vector position, session-ID ordering, or mutation as an ambiguity tie-breaker.

The selected in-memory implementation does not manufacture `UnavailableOrIndeterminate` during an intact ordinary lookup. That stable trait variant remains available for other providers.

## 10. Grant ownership and non-consumption

Exactly one current record may be cloned only to build one fresh existing one-shot `AuthorizedRequesterRendezvous`.

Authorization itself does not:
- consume or remove the retained record;
- retire it;
- extend any lifetime;
- change capacity;
- bypass later requester registry revalidation;
- weaken expected-publisher equality.

No provider borrow or guard survives into downstream reachability-owner durable work.

## 11. Focused tests required

CT source includes focused tests proving at least:
1. zero capacity rejection;
2. capacity enforcement before mutation;
3. exact duplicate rejection;
4. one current record returns the exact requester/publisher grant;
5. two distinct current requesters for one publisher fail `Ambiguous` without mutation;
6. retired-only matches fail `StaleOrRetired`;
7. absent publisher fails `Missing`;
8. authorization is non-consuming/repeatable as fresh grants;
9. retirement changes only the exact record and distinguishes already-retired/unknown;
10. current records cannot be removed through retired removal;
11. retired removal frees one capacity slot;
12. insertion order never resolves ambiguity.

Tests use only existing disposable signer/session-auth dev dependencies. No network, clock, database, thread, listener or product runtime is started.

## 12. Preserved authority chain

CT preserves without modification:
- CM `AuthorizedRequesterRendezvous` carrier and generic authority port;
- CQ provider-neutral execution ordering;
- authenticated publisher identity origin from `AuthenticatedPrwcConnection`;
- expected-publisher equality before commit;
- peer-originated request-ID custody;
- `WorkspaceDeviceRegistry` current requester revalidation inside reachability ownership;
- `ProductionReachabilityOwner` freshness/durable mutation authority.

## 13. Explicit exclusions

CT does not authorize:
- database, durable storage, schema, serialization or recovery;
- TTL, clock, expiry or time-based staleness;
- background cleanup;
- mutex/RW lock/channel/actor/transaction or distributed coordination;
- global/static provider state;
- runtime/process owner or bootstrap wiring;
- listener/socket/task/frame-loop activation;
- response/Error-frame mapping;
- request-ID authority changes;
- credentials;
- production networking;
- deployment;
- merge.

## 14. Dependency and lock rule

No Cargo manifest or lockfile change is expected or authorized.

If canonical compiler evidence proves an additional dependency unavoidable, CT stops and requires a fresh bounded audit rather than silently expanding scope.

## 15. Validation and closure

CT closes only if one exact final head proves:
- exact CS merge base;
- only the three authorized paths changed;
- canonical Rust validation FULL PASS;
- Android validation is recorded according to actual trigger/result and never inferred;
- non-applicable workflows are recorded as SKIPPED, not PASS;
- root and Android-native lock blobs remain byte-stable;
- immutable Drive audit raw-readback exact;
- rolling Drive predecessor freshly guarded and preserved byte-for-byte as prefix;
- PR body changes to `Status: CLOSED` only after durable evidence;
- PR remains draft/open/unmerged.

## 16. Safe successor

After durable CT closure, perform a fresh exact-head prerequisite audit before choosing among runtime/process ownership/wiring and candidate-publication response/Error-frame composition.

Neither is automatically authorized by CT. No direct jump to listener/runtime activation, production networking, deployment or merge is allowed.
