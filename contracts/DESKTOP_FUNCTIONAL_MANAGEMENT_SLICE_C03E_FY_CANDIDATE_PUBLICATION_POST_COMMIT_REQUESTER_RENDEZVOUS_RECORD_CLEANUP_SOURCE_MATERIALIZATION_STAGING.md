# Phase 152 C03e-FY — Candidate Publication Post-Commit Requester/Rendezvous Record Cleanup Source Materialization

Status: VALIDATING

Target gate:

`C03E_FY_CANDIDATE_PUBLICATION_POST_COMMIT_REQUESTER_RENDEZVOUS_RECORD_CLEANUP_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Canonical predecessor:

`C03E_FX_CANDIDATE_PUBLICATION_POST_COMMIT_REQUESTER_RENDEZVOUS_RECORD_CLEANUP_SEMANTICS_SELECTED`

Exact FX branch:

`phase-152-c03e-fx-candidate-publication-post-commit-requester-rendezvous-record-cleanup-semantics-selection-staging`

Exact FX head:

`3217755d9c5d178d3177c7dfdcba77d38315aa49`

Exact FX tree:

`b6750e8201a007b41114638eb17247e52031fe86`

FX PR #300 was re-read before FY mutation and remained `Status: CLOSED`, open, draft, unmerged and mergeable. FX remains frozen.

## 2. Purpose

FY materializes only the FX-selected exact requester/rendezvous lifecycle cleanup after one definite successful durable candidate-publication commit.

FY does not activate a candidate-publication command loop or listener. It creates only dormant Agent-internal source seams that preserve the existing candidate-publication semantic ordering while preventing the shared requester-authority Tokio mutex from spanning durable reachability work or later response I/O.

## 3. Exact authorized source boundary

The exact FX -> FY diff is authorized to contain only:

1. this FY contract;
2. `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`;
3. `crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs`.

No other path is authorized.

In particular FY does not modify:

- `crates/prw-remote-bridge/src/candidate_publication_execution.rs`;
- requester/rendezvous provider representation source;
- requester/rendezvous authority carrier source;
- reachability-owner source;
- PRWC response/write source;
- Cargo manifests or lockfiles;
- workflows;
- Android/Desktop application source;
- listener/bootstrap/readiness source;
- deployment or recovery configuration.

## 4. Existing provider lifecycle remains authoritative

FY reuses the exact existing bounded in-memory provider methods:

- `authorize_current_for_publisher(...)`;
- `retire(requester SessionId, expected publisher DeviceId)`;
- `remove_retired(requester SessionId, expected publisher DeviceId)`.

No alternate provider, record key, wildcard cleanup API, TTL, timer, cleanup task, database, persistence layer or provider reset is introduced.

## 5. Runtime-owner narrow grant seam

`CandidatePublicationRequesterRendezvousRuntimeOwner` gains one crate-internal method that delegates exactly once to the existing provider-neutral current-authority port for one authenticated publisher `DeviceId`.

The method returns the existing owned `AuthorizedRequesterRendezvous` grant unchanged.

It does not retire/remove the underlying record, expose the provider, expose a guard, refresh authority, retry selection, or select an arbitrary ambiguous requester.

Existing `RequesterRendezvousAuthorityError` classifications remain exact.

## 6. Runtime-owner exact cleanup seam

The runtime owner gains one crate-internal exact-record cleanup operation accepting only:

`(requester SessionId, expected publisher DeviceId)`

It performs exactly:

1. `retire(...)`;
2. if and only if retirement succeeds, `remove_retired(...)`.

If retirement fails, removal is not attempted.

If retirement succeeds and removal fails, no rollback to `Current` occurs.

A successful return reclaims exactly the one capacity slot already defined by the existing provider.

## 7. No raw provider exposure

FY does not add:

- a provider getter;
- `provider_mut()`;
- a raw Tokio mutex getter;
- a guard-returning method;
- an Arc getter;
- a wildcard cleanup iterator;
- a publisher-wide cleanup method.

All provider custody remains private.

## 8. Non-authorizing cleanup identity

The shared-authority integration materializes one private exact cleanup-identity carrier containing only:

- requester `SessionId`;
- expected publisher `DeviceId`.

It is projected from the same exact `AuthorizedRequesterRendezvous` grant selected for the candidate-publication attempt.

The cleanup identity is not candidate-publication authority, requester authentication, retry authority, reconnect authority, dial authority, request correlation, transport identity, or reachability freshness.

## 9. Shared grant-selection custody

`SharedRequesterRendezvousAuthority` gains one private async grant-selection seam.

It:

1. acquires the existing requester-authority Tokio mutex;
2. calls the runtime owner's exact current-grant seam once;
3. releases the guard before returning the owned grant.

No durable reachability work occurs while this guard is held.

## 10. Shared exact cleanup custody

The shared authority gains one private async cleanup seam.

It:

1. acquires the requester-authority mutex;
2. consumes one exact cleanup identity;
3. calls runtime-owner exact retire+remove;
4. releases the guard before returning.

No response I/O, retry, reconnect or reachability mutation occurs inside the cleanup critical section.

## 11. Candidate-publication integration order

FY materializes one dormant crate-internal candidate-publication integration method with exact order:

1. obtain the authenticated publisher session from the existing authenticated PRWC connection;
2. construct/validate the candidate publication through existing `publish_current_candidates(...)` while no requester mutex is held;
3. acquire requester authority and select exactly one current grant for the authenticated publisher;
4. release requester authority;
5. check exact grant expected-publisher equality;
6. project exact non-authorizing cleanup identity from the grant;
7. call the existing `ProductionReachabilityOwner::commit_candidate_publication(...)` while no requester mutex guard exists;
8. if commit fails, return existing `CandidatePublicationExecutionError::Reachability` and perform no cleanup;
9. if commit succeeds, reacquire requester authority;
10. exact retire+remove the same requester record;
11. release requester authority;
12. return the definite `ReachabilityCommitOutcome` and cleanup disposition separately.

## 12. Existing execution failure taxonomy is preserved

FY reuses existing `CandidatePublicationExecutionError` classes for failures before or at durable commit:

- `Candidate(...)`;
- `RequesterAuthority(...)`;
- `ExpectedPublisherMismatch`;
- `Reachability(...)`.

No cleanup is attempted on any of those failure paths.

FY does not add cleanup failure as a new `CandidatePublicationExecutionError` variant.

## 13. Committed outcome remains authoritative

After durable commit success, cleanup failure cannot rewrite the publication as failed.

FY materializes a separate successful post-commit carrier containing:

- exact `ReachabilityCommitOutcome`;
- exact `Result<(), RequesterRendezvousLifecycleError>` cleanup disposition.

The committed reachability result remains separately projectable for existing Accepted response semantics.

## 14. No rollback or second commit

If cleanup fails after commit, FY performs no:

- durable reachability rollback;
- freshness rollback;
- second reachability commit;
- replacement requester registration;
- requester record reactivation;
- candidate-publication retry;
- implicit re-admission;
- reconnect;
- background cleanup retry.

## 15. Response I/O remains later and separate

The FY integration seam reads/writes no PRWC frame.

It does not call candidate-publication result writing.

Response composition and response write therefore occur only after requester-authority cleanup custody has been released.

A later response-write failure does not resurrect requester authority and does not govern whether cleanup should have occurred.

## 16. CQ remains byte-stable

FY deliberately does not refactor or modify the existing provider-neutral CQ execution helper.

The new Agent-specific shared-authority integration composes the same existing primitive authorities because the synchronous CQ `&mut provider` shape cannot directly model an asynchronously locked provider without either holding a mutex over durable commit or introducing a blocking lock.

FY introduces neither behavior.

## 17. No blocking Tokio lock adaptation

FY does not implement `RequesterRendezvousAuthorityProvider` for the shared async handle using:

- `blocking_lock()`;
- `try_lock()` as production authority;
- spin/poll loops;
- unsafe access;
- nested executor driving.

Shared authority remains explicitly async at its lock boundary.

## 18. Focused source tests

FY adds focused tests proving at least:

- runtime-owner current-grant method has the exact narrow publisher `DeviceId` shape;
- runtime-owner cleanup method requires exact requester `SessionId` plus expected publisher `DeviceId`;
- unknown cleanup identity preserves `RecordUnknown`;
- shared current-grant selection releases the requester mutex before returning;
- the generic commit phase can run while the requester mutex is acquirable;
- a cleanup failure after a successful commit-phase value preserves that committed value separately;
- a commit-phase failure returns before cleanup phase;
- existing clone/send/sync and shared-one-owner allocation properties remain intact.

Existing provider tests remain authoritative for exact successful retire/remove lifecycle semantics and provider capacity reclamation.

## 19. No dependency or lock change

FY uses only existing workspace dependencies and existing Tokio `rt`/`sync` features.

No Cargo manifest or lockfile mutation is selected.

## 20. No activation

FY does not wire the new integration method into:

- Agent `main`;
- PRWC listener/accept loop;
- remote-session listener;
- bootstrap;
- readiness publication;
- command loop;
- worker supervisor;
- Android/Desktop application startup.

The seam remains dormant until a separately gated runtime-integration checkpoint.

## 21. Security and identity boundary

FY preserves logical identity authority:

- requester cleanup uses requester `SessionId` from the authenticated requester grant;
- expected publisher uses logical `DeviceId` from the same grant;
- authenticated candidate publisher identity comes from the PRWC authenticated session;
- transport identity remains lower-layer candidate admission data only.

Socket addresses, task IDs, request IDs, candidate IDs, Arc addresses, mutex addresses and map slots never become cleanup authority.

## 22. Out of scope

FY does not materialize:

- automatic cleanup retry/recovery;
- idempotence flattening;
- TTL/clock cleanup;
- persistent requester state;
- publisher-wide cleanup;
- worker-completion cleanup;
- requester-session deletion/revocation;
- candidate dialing;
- direct/relay selection;
- forwarding;
- listener activation;
- production command loop;
- deployment;
- restart/recovery;
- merge.

## 23. Validation requirements

FY closes only on one exact final head where:

1. FX remains exact merge base;
2. no unauthorized path changed;
3. Rust canonical validation reaches terminal FULL PASS;
4. if Android validation triggers, it must reach terminal FULL PASS; if it does not trigger, no Android PASS is claimed;
5. non-applicable workflows are recorded as skipped;
6. no manifest/lock/workflow diff exists;
7. immutable Drive audit is uploaded and raw-read back byte-exactly;
8. PR body moves to `Status: CLOSED` only after durable evidence succeeds;
9. PR remains draft/open/unmerged.

## 24. Closure

On successful exact-head validation and immutable evidence, FY closes as:

`CLOSED_CANDIDATE_PUBLICATION_POST_COMMIT_REQUESTER_RENDEZVOUS_RECORD_CLEANUP_SOURCE_MATERIALIZATION`

Canonical gate:

`C03E_FY_CANDIDATE_PUBLICATION_POST_COMMIT_REQUESTER_RENDEZVOUS_RECORD_CLEANUP_SOURCE_MATERIALIZED`

No exact successor name is asserted by FY. A fresh post-FY prerequisite audit is required before the next checkpoint is named or materialized.
