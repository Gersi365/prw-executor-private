# Phase 152 C03e-FX — Candidate Publication Post-Commit Requester/Rendezvous Record Cleanup Semantics Selection

Status: VALIDATING

## 1. Purpose

C03e-FX selects only the requester/rendezvous record lifecycle semantics that apply after one exact requester authority grant has been used by one candidate-publication execution attempt.

FX is docs-only. It does not materialize Rust source, expose provider internals, activate candidate-publication runtime wiring, start dialing, deploy, restart/recover, or merge.

The exact C03e-FW source state is the immutable predecessor.

## 2. Exact predecessor

Canonical predecessor gate:

`C03E_FW_RECOVERED_REQUESTER_AWARE_WORKER_COMPLETION_PEER_DISPOSITION_SOURCE_MATERIALIZED`

Exact predecessor branch:

`phase-152-c03e-fw-recovered-requester-aware-worker-completion-peer-disposition-source-materialization-staging`

Exact FW head:

`6c6ba541cdcefa20b500210519a210be92e348f9`

Exact FW tree:

`6d657e0df47628800fd639f53d6fcc7d438e4356`

FW remains frozen.

## 3. Existing lifecycle facts

The bounded in-memory requester/rendezvous provider already owns exact records containing:

- one authenticated requester session;
- one expected publisher logical `DeviceId`;
- lifecycle `Current` or `Retired`.

Its exact lifecycle mutation identity is:

`(requester SessionId, expected publisher DeviceId)`

The provider already exposes synchronous exact-record operations:

- `retire(requester_session_id, expected_publisher_device_id)`;
- `remove_retired(requester_session_id, expected_publisher_device_id)`.

`remove_retired` frees exactly one bounded capacity slot. No timer, background cleanup task, TTL, wildcard cleanup, or implicit retirement exists.

## 4. Existing authority grant facts

`AuthorizedRequesterRendezvous` is an owned one-shot operation grant. It carries exactly:

- authenticated requester `AuthenticatedDeviceSession`;
- expected publisher `DeviceId`.

Therefore one exact grant contains enough logical provenance to derive the provider lifecycle identity:

- `grant.requester_session().session_id()`;
- `grant.expected_publisher_device_id()`.

The grant remains operation authority for only the bounded candidate-publication attempt. FX does not make it cloneable, replayable, or reusable.

## 5. Existing candidate-publication ordering

The existing provider-neutral candidate-publication execution order is:

1. authenticate/validate publisher candidate publication;
2. call requester/rendezvous current-authority selection exactly once using the authenticated publisher `DeviceId`;
3. verify exact expected-publisher equality;
4. call `ProductionReachabilityOwner::commit_candidate_publication(...)` exactly once using the grant requester session;
5. return `ReachabilityCommitOutcome` only after the existing durable reachability owner succeeds.

`ReachabilityCommitOutcome` is evidence that the durable publication commit completed and the new local authoritative state was installed according to the existing reachability-owner law.

## 6. Existing one-shot grant does not already consume lifecycle state

Earlier authority selection deliberately kept operation-grant issuance distinct from underlying lifecycle consumption.

`authorize_current_for_publisher(...)` is non-consuming. The concrete in-memory provider may return fresh owned grants repeatedly while the exact record remains `Current`.

FX is therefore the separate lifecycle gate that decides when a successfully used requester authority record stops being current.

## 7. Cleanup is not publisher-worker-completion authority

FV/FW already established that publisher worker completion is insufficient requester cleanup authority.

FW completion carries publisher `DeviceId`, recovered publisher owner, and exact FL/join terminal result, but it does not carry requester `SessionId`.

FX preserves that boundary.

No requester record may be retired or removed because of:

- FW cancellation;
- typed FL failure;
- abnormal join;
- publisher peer close;
- publisher session ID;
- publisher worker map key;
- Tokio task identity;
- completion order.

## 8. Cleanup is not DR acknowledgement authority

Requester DR registration becomes current before the terminal DR acknowledgement is sent.

A successful DR acknowledgement does not mean the expected publisher has published candidates or that durable reachability state has committed.

A DR acknowledgement write failure also does not identify whether the later publisher-side candidate publication will occur.

FX therefore selects no retirement/removal at DR acknowledgement success or failure.

## 9. Selected cleanup trigger

FX selects exactly one normal cleanup trigger:

**successful durable candidate-publication commit using the exact requester authority grant.**

The trigger exists only when `ProductionReachabilityOwner::commit_candidate_publication(...)` returns `Ok(ReachabilityCommitOutcome)` for the publication attempt that used that exact grant.

Grant issuance alone is insufficient. Expected-publisher equality alone is insufficient. Response-frame composition or response write is not required to establish the trigger.

## 10. No cleanup on pre-commit candidate failure

If authenticated candidate publication construction fails before requester authority selection, no exact requester record was selected for the attempt.

FX selects no requester lifecycle mutation.

## 11. No cleanup on requester-authority failure

If `authorize_current_for_publisher(...)` returns `Missing`, `StaleOrRetired`, `Ambiguous`, or `UnavailableOrIndeterminate`, no successful exact current grant exists for the attempt.

FX selects no requester lifecycle mutation.

## 12. No cleanup on expected-publisher mismatch

If an obtained grant names an expected publisher different from the authenticated candidate publisher, candidate publication fails closed before reachability commit.

FX selects no requester lifecycle mutation on this path.

The mismatching grant is not treated as authority to mutate some other exact requester record merely because its fields are structurally available.

## 13. No cleanup on reachability commit failure

If `ProductionReachabilityOwner::commit_candidate_publication(...)` returns any existing `ReachabilityOwnerError`, FX selects no requester record retirement/removal.

This includes definite pre-commit failure, stale durable state, recovery-required state, or ambiguous persistence classification.

The existing reachability owner remains authoritative for its own recovery semantics. FX does not guess whether an ambiguous durable operation should consume requester authority.

## 14. Selected exact lifecycle cleanup sequence

After one definite successful durable candidate-publication commit, FX selects exact-record cleanup in this order:

1. reacquire requester/rendezvous lifecycle authority for the exact registration identity;
2. `retire(exact requester SessionId, exact expected publisher DeviceId)`;
3. after successful retirement, `remove_retired(the same exact requester SessionId, the same exact expected publisher DeviceId)`;
4. release requester/rendezvous authority custody.

The two mutations belong to one bounded cleanup critical section for one exact record.

No unrelated requester record is inspected for mutation, retired, removed, reset, or swept.

## 15. Why retire precedes remove

The existing provider explicitly forbids removal of a current record.

FX preserves that lifecycle invariant rather than adding a direct delete-current shortcut.

Retirement records the selected terminal transition inside the existing provider lifecycle before bounded capacity is reclaimed by `remove_retired`.

## 16. Immediate bounded capacity reclamation

FX selects immediate removal after successful exact retirement inside the same cleanup operation.

Therefore a successful post-commit cleanup frees exactly one provider capacity slot.

FX does not retain retired records indefinitely merely for diagnostic history, because the current provider is explicitly bounded and non-durable and already exposes removal as caller-driven lifecycle completion.

After successful removal, a later provider lookup for that publisher may classify absence according to the existing provider law. FX does not create a historical tombstone store or persistence layer.

## 17. No publisher-wide or wildcard cleanup

Distinct requester sessions may have distinct current records for the same expected publisher.

Therefore FX explicitly rejects:

- retire-all-for-publisher;
- remove-all-for-publisher;
- wildcard requester cleanup;
- provider reset;
- capacity sweep;
- iteration-order selection;
- newest-record selection;
- cleanup by publisher `DeviceId` alone.

Exactly one successful candidate-publication attempt consumes only the exact requester registration whose grant authorized that attempt.

## 18. Cleanup identity is non-authorizing provenance

Future materialization may preserve the exact lifecycle key across the durable commit as a narrow cleanup receipt/provenance carrier.

Such a carrier may contain only what is needed to identify the exact record, conceptually:

`(requester SessionId, expected publisher DeviceId)`

Possession of that cleanup identity is not candidate-publication authority, requester authorization, session authentication, reachability authority, retry authority, or dial authority.

It must not contain or expose a raw provider reference, mutex guard, transport identity, candidate set, freshness token, peer socket, or worker handle.

## 19. Operation grant must not become reusable cleanup authority

Future materialization must not retain the full `AuthorizedRequesterRendezvous` as a replayable post-commit authority object.

The grant's requester session may be borrowed for the existing durable reachability commit, and exact lifecycle identity may be projected for cleanup custody. After the bounded attempt, the operation grant itself is consumed/dropped according to existing one-shot semantics.

A cleanup receipt cannot authorize a second candidate-publication attempt.

## 20. Shared-authority lock ordering

The existing Agent shared requester/rendezvous authority uses one Tokio mutex around the process-local runtime owner.

FX selects the following lock/custody ordering for future integration:

1. acquire requester/rendezvous authority only long enough to linearize exact current grant selection;
2. derive/preserve exact cleanup identity from the selected grant;
3. release requester/rendezvous authority before durable reachability commit;
4. perform the existing reachability-owner commit with no requester-authority guard held;
5. only after definite commit success, reacquire requester/rendezvous authority;
6. perform exact retire + exact remove in one bounded cleanup critical section;
7. release requester/rendezvous authority before result-frame composition or response I/O.

No shared-current authority lock is needed for post-commit cleanup because the cleanup identity was already established by authenticated requester registration and exact grant selection.

## 21. Existing provider-neutral helper cannot justify a long-held mutex

The existing provider-neutral helper accepts `&mut RequesterRendezvousAuthorityProvider` across its bounded authorize -> reachability-commit call.

That synchronous borrowing shape is valid for a directly owned provider but must not be implemented in Agent integration by holding the shared requester/rendezvous Tokio mutex across durable reachability work.

Future materialization must preserve the existing semantic ordering while introducing a narrow split/adaptation that allows:

- grant selection under requester-authority custody;
- durable commit after requester-authority custody is released;
- exact cleanup after successful commit under newly reacquired requester-authority custody.

FX does not select a second provider or weaken the existing provider-neutral execution semantics.

## 22. Post-commit cleanup failure is not publication rollback

Once `ReachabilityCommitOutcome` exists, the candidate publication is already durably committed under the existing reachability-owner authority.

If later requester-record retirement/removal fails, FX selects:

- no durable reachability rollback;
- no replacement commit;
- no second publication attempt;
- no freshness rollback;
- no peer reconnect;
- no implicit re-admission;
- no target-dial retry.

Cleanup failure is a post-commit lifecycle-maintenance fault, not evidence that the candidate publication failed to commit.

## 23. Post-commit cleanup failure must remain distinct from `CandidatePublicationExecutionError`

The existing candidate-publication response codec maps `Ok(ReachabilityCommitOutcome)` to Accepted and execution `Err(_)` to generic Rejected.

Therefore a requester cleanup fault discovered after successful durable commit must not be rewritten into existing `CandidatePublicationExecutionError` in a way that would report the already-committed publication as Rejected.

Future materialization must preserve the successful `ReachabilityCommitOutcome` and report cleanup disposition separately to its higher owner.

Exact source type naming is deferred, but the semantic distinction is mandatory.

## 24. Peer-visible result remains governed by durable commit

FX selects that candidate-publication protocol success eligibility remains based on the existing semantic execution result:

- durable commit success remains eligible for Accepted;
- pre-commit execution failure remains Rejected under existing codec law.

A post-commit cleanup fault does not fabricate a rejected semantic execution result.

FX does not add a new wire status, error code, fallback frame, or diagnostic payload for requester cleanup.

## 25. Response write does not govern cleanup

Requester cleanup occurs after definite durable commit and does not wait for candidate-publication result-frame write success.

If response write later fails, the existing candidate-publication I/O terminalization law remains authoritative. The requester record is not resurrected merely because the peer may not have observed the Accepted response.

This avoids replaying requester authority after durable reachability state already moved.

## 26. No automatic cleanup retry selected

FX selects no background retry task, timer, queue, TTL sweep, process restart hook, or reconnect loop for cleanup failure.

A future checkpoint may select bounded recovery/idempotence behavior if operational evidence requires it.

Until then, cleanup failure remains separately observable post-commit state and must not be silently retried.

## 27. Partial cleanup failure

The exact sequence may theoretically observe:

- retirement failure before removal;
- retirement success followed by removal failure.

FX selects no rollback from `Retired` back to `Current`.

If retirement succeeds and removal fails, the record remains non-current according to the existing provider lifecycle and capacity may remain occupied. That condition is a post-commit cleanup fault for a separately gated recovery policy; it is not authority to reactivate the requester record.

## 28. Idempotence is not fabricated

Existing `retire` and `remove_retired` expose explicit lifecycle errors such as unknown/already-retired/current-cannot-be-removed.

FX does not flatten those states into unconditional success and does not assume that `RecordUnknown` always means an earlier cleanup succeeded.

A later materialization/recovery checkpoint must preserve exact lifecycle error information sufficiently to distinguish expected bounded transitions from invariant contradictions or prior cleanup.

## 29. Requester session lifetime

Cleanup uses only the exact requester `SessionId` plus expected publisher `DeviceId` lifecycle identity.

It does not require the requester transport connection to remain open after durable candidate publication commit.

It does not delete or invalidate the authenticated requester session itself. Requester/rendezvous record lifecycle remains distinct from general PRW authenticated-session lifecycle.

## 30. Security boundary

FX preserves logical identity authority:

- requester identity originates from an authenticated requester session;
- expected publisher identity is the exact logical `DeviceId` bound by requester/rendezvous registration;
- authenticated candidate publisher must still match the grant expected publisher before commit;
- transport identities, socket addresses, QUIC IDs, task IDs, map positions, request IDs, and candidate IDs are never lifecycle cleanup authority.

## 31. Privacy boundary

FX introduces no new peer-visible cleanup diagnostic.

Future internal cleanup classification must not serialize user IDs, workspace IDs, device IDs, session IDs, network addresses, panic payloads, policy details, or provider internals onto the candidate-publication wire.

## 32. No reachability/dialing activation

A successful candidate-publication commit updates existing reachability authority only.

FX cleanup does not itself:

- select a connectivity path beyond existing reachability-owner state;
- start traversal I/O;
- dial a target;
- establish forwarding;
- bind or accept listeners;
- publish product readiness;
- activate Android/Desktop behavior;
- deploy;
- restart/recover a process;
- merge.

## 33. Exact FX diff boundary

FX is docs-only.

The exact FW -> FX diff is authorized to contain only:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_FX_CANDIDATE_PUBLICATION_POST_COMMIT_REQUESTER_RENDEZVOUS_RECORD_CLEANUP_SEMANTICS_SELECTION_STAGING.md`

Any Rust/Kotlin source, manifest, lockfile, workflow, provider data, runtime/listener source, networking configuration, deployment path, or unrelated contract change blocks FX closure.

## 34. Validation and durable evidence requirements

FX may close only if one exact final head proves:

1. FW remains the exact merge base;
2. FX is ahead only by the bounded docs-only contract commit;
3. exactly one changed path exists;
4. automatically triggered relevant validation reaches terminal non-failing verdict; non-applicable workflows are recorded as skipped rather than called PASS;
5. immutable Drive audit is uploaded and raw-read back byte-for-byte;
6. PR body changes to `Status: CLOSED` only after durable evidence succeeds;
7. PR remains draft/open/unmerged with exact head/base unchanged.

The source contract itself need not be mutated after exact-head validation merely to replace `VALIDATING`; canonical closure may be carried by immutable audit plus PR body/readback.

## 35. Selected future source-materialization boundary

After durable FX closure, the next separately gated source checkpoint may materialize only the minimum seams required by this selection, including conceptually:

1. exact requester lifecycle cleanup identity/receipt custody derived from one selected grant;
2. runtime-owner exact retire+remove operation using the existing provider APIs;
3. shared requester-authority bounded cleanup operation;
4. a grant/commit adaptation that does not hold requester-authority mutex custody across durable reachability commit;
5. a post-commit disposition surface that preserves `ReachabilityCommitOutcome` independently from cleanup status;
6. focused tests for exact identity, lock/call ordering, no cleanup on pre-commit failure, capacity reclamation, and no rollback/rejected-result rewrite after post-commit cleanup fault.

That source checkpoint must not activate listener/runtime networking, target dialing, deployment, restart/recovery, or merge.

## 36. Canonical closure

Upon exact-head validation, immutable Drive readback, and PR administrative closure, FX closes as:

`CLOSED_CANDIDATE_PUBLICATION_POST_COMMIT_REQUESTER_RENDEZVOUS_RECORD_CLEANUP_SEMANTICS_SELECTION`

Canonical gate:

`C03E_FX_CANDIDATE_PUBLICATION_POST_COMMIT_REQUESTER_RENDEZVOUS_RECORD_CLEANUP_SEMANTICS_SELECTED`

## 37. Exact next checkpoint

Provided FX closes without contradiction, the selected source-materialization successor is:

**C03e-FY — candidate publication post-commit requester/rendezvous record cleanup source materialization**

FY must remain separately gated and may materialize only the FX-selected cleanup custody, lock ordering, exact-record lifecycle mutation, post-commit disposition, and focused tests.