# Phase 152 C03e-FZ — Candidate Publication Post-Commit Cleanup Terminal Result Projection Semantics Selection

Status: VALIDATING

## 1. Purpose

C03e-FZ selects only the terminal-result projection semantics required after C03e-FY materialized candidate-publication execution with exact post-commit requester/rendezvous cleanup.

FZ is docs-only. It does not materialize Rust source, route candidate-publication commands, write a response frame, construct a production reachability owner, activate reachability authority, bind or accept a listener, dial a target, deploy, restart/recover, or merge.

The exact C03e-FY source state is the immutable predecessor.

## 2. Exact predecessor

Canonical predecessor gate:

`C03E_FY_CANDIDATE_PUBLICATION_POST_COMMIT_REQUESTER_RENDEZVOUS_RECORD_CLEANUP_SOURCE_MATERIALIZED`

Exact predecessor branch:

`phase-152-c03e-fy-candidate-publication-post-commit-requester-rendezvous-record-cleanup-source-materialization-staging`

Exact FY head commit:

`c99fb601f184b8d283a33062d50c0fc39df577fe`

Exact FY tree:

`65c05c32f9fdc752765330e3656620cea0bd9666`

FY remains frozen.

## 3. Fresh post-FY audit facts

The fresh post-FY source audit establishes four independent existing boundaries.

First, the provider-neutral candidate-publication execution seam in `prw-remote-bridge` returns:

`Result<ReachabilityCommitOutcome, CandidatePublicationExecutionError>`

and preserves the existing pre-commit error taxonomy:

- candidate admission failure;
- requester-authority failure;
- expected-publisher mismatch;
- reachability-owner failure.

Second, the existing pure candidate-publication result codec projects exactly that semantic result into one terminal peer-visible message:

- `Ok(ReachabilityCommitOutcome)` -> `Accepted { replacement_freshness }`;
- `Err(CandidatePublicationExecutionError)` -> generic `Rejected`.

Third, FY materialized an Agent-owned execution seam whose successful result is richer:

`CandidatePublicationPostCommitRequesterCleanupOutcome`

containing independently:

- exact committed `ReachabilityCommitOutcome`;
- exact post-commit `Result<(), RequesterRendezvousLifecycleError>` cleanup disposition.

Fourth, FY explicitly preserves the existing `CandidatePublicationExecutionError` only for failures before or at durable reachability commit and performs no requester cleanup on those failure paths.

## 4. The first missing prerequisite is projection, not activation

The post-FY audit does not justify jumping directly to command-loop, listener, process, or reachability-authority activation.

The first semantic mismatch is narrower: FY can now represent a successful durable candidate publication followed by failed requester-record cleanup, while the existing wire projection accepts only the original commit/error semantic result.

FZ therefore selects how the richer FY outcome is reduced for the existing terminal result codec while preserving cleanup disposition separately.

## 5. Durable commit remains peer-visible success authority

FZ preserves the existing candidate-publication protocol law:

**a definite successful durable reachability commit is the sole success fact needed for candidate-publication Accepted eligibility.**

If FY returns a `CandidatePublicationPostCommitRequesterCleanupOutcome`, its embedded `ReachabilityCommitOutcome` proves that the durable publication commit has already succeeded.

The later cleanup disposition cannot revoke that fact.

## 6. Cleanup failure must not rewrite Accepted eligibility

If FY returns:

- committed `ReachabilityCommitOutcome`; and
- `Err(RequesterRendezvousLifecycleError)` from exact post-commit cleanup,

FZ selects that the candidate-publication terminal projection remains `Accepted` using the replacement freshness from the committed reachability outcome.

The cleanup failure remains an internal lifecycle-maintenance disposition.

It must not be converted into:

- `CandidatePublicationExecutionError`;
- generic wire `Rejected`;
- a new candidate-publication wire error status;
- a freshness rollback;
- a second reachability commit;
- requester reactivation;
- implicit retry or reconnect.

## 7. Cleanup success also preserves normal Accepted projection

If FY returns a committed reachability outcome and cleanup succeeds, the terminal projection is the same existing Accepted projection.

Requester cleanup success adds no new peer-visible candidate-publication field or status.

The peer-visible freshness token remains exactly the verifier-issued replacement freshness from `ReachabilityCommitOutcome`.

## 8. Pre-commit execution error remains generic Rejected

If the FY execution seam returns `Err(CandidatePublicationExecutionError)`, FZ selects the existing generic Rejected projection unchanged.

There is no cleanup disposition on this path because FY already guarantees no cleanup after:

- candidate construction/admission failure;
- requester-authority selection failure;
- expected-publisher mismatch;
- reachability commit failure.

FZ does not fabricate an absent cleanup result.

## 9. Selected two-channel projection shape

Future source materialization must preserve two logically separate channels:

1. **wire semantic result** — exactly the existing `Result<ReachabilityCommitOutcome, CandidatePublicationExecutionError>` shape consumed by the existing pure candidate-publication result codec;
2. **post-commit cleanup disposition** — present only after definite commit success and preserving the exact `Result<(), RequesterRendezvousLifecycleError>`.

Conceptually:

- FY `Err(execution_error)` -> wire `Err(execution_error)`, cleanup disposition absent;
- FY `Ok { commit, cleanup: Ok(()) }` -> wire `Ok(commit)`, cleanup disposition present-success;
- FY `Ok { commit, cleanup: Err(cleanup_error) }` -> wire `Ok(commit)`, cleanup disposition present-failure.

Exact Rust type and helper names are deferred to source materialization. The separation itself is normative.

## 10. No lossy flattening of cleanup state

FZ rejects a projection that simply discards cleanup disposition before the higher owner can observe it.

The future adapter may consume the FY outcome by value, but it must preserve the exact cleanup success/failure state separately from the wire semantic result.

No boolean-only conversion is selected when it would erase the existing typed `RequesterRendezvousLifecycleError`.

## 11. No cleanup state on the peer wire

Cleanup disposition is internal and must not be serialized into the candidate-publication response.

FZ selects no:

- cleanup error code;
- cleanup-success bit;
- requester lifecycle field;
- requester SessionId;
- expected publisher DeviceId;
- provider state;
- mutex/lock state;
- retry hint.

The existing Accepted/Rejected wire vocabulary remains unchanged.

## 12. Exact request correlation remains unchanged

The existing result-framing helper echoes the decoded candidate-publication command's exact peer-originated request ID.

FZ does not allocate, replace, normalize, increment, or reinterpret request correlation.

Future composition must pass the projected existing semantic result to the existing result codec with the exact decoded command that already owns correlation.

## 13. Existing result codec remains authoritative

FZ does not fork or duplicate candidate-publication result encoding.

The existing `prw-remote-bridge` pure codec remains authoritative for:

- Accepted versus Rejected outer kind;
- accepted replacement freshness encoding;
- generic rejected payload;
- exact request-ID echo;
- bounded frame validation.

Future Agent materialization must adapt FY state into that existing codec contract rather than introduce a parallel wire encoder.

## 14. Cross-crate dependency direction remains unchanged

`prw-remote-bridge` must not depend on `prw-agent` merely to understand FY's Agent-private cleanup outcome.

FZ selects the adapter/projection ownership on the Agent side of the dependency boundary.

The bridge codec continues to consume only its existing provider-neutral semantic result shape.

## 15. Projection performs no response I/O

The future source adapter selected by FZ is pure ownership/typing projection only.

It must not:

- receive a command frame;
- send a result frame;
- accept a stream;
- finish a QUIC send direction;
- close a peer;
- resume a loop;
- spawn a task;
- block on a runtime.

Response I/O remains separately gated.

## 16. Projection performs no requester mutation

All requester lifecycle mutation selected by FX and materialized by FY has already completed before the FY successful outcome exists.

FZ projection therefore must not call:

- `retire`;
- `remove_retired`;
- registration;
- current-grant selection;
- provider reset/sweep;
- cleanup retry.

It observes/preserves the already-produced cleanup disposition only.

## 17. Projection performs no reachability mutation

The embedded `ReachabilityCommitOutcome` is post-commit evidence.

FZ projection must not call any reachability-owner mutation or authority operation, including:

- `commit_candidate_publication`;
- recovery/reload;
- traversal provisioning;
- traversal polling;
- retirement;
- live-owner acquire/currentness/release.

It only preserves the existing committed outcome for the existing terminal codec.

## 18. Reachability authority custody is a later prerequisite

The fresh audit also confirms that the Agent's staged `ReachabilityAuthorityRuntimeOwner` currently owns a `ReachabilityLiveOwnerComposedAsyncAuthority`, while FY's semantic execution seam consumes an existing `ProductionReachabilityOwner<S,T>`.

FZ does not claim these are interchangeable and does not invent a conversion between them.

Any production composition that must establish or recover the exact `ProductionReachabilityOwner` for candidate publication requires its own fresh prerequisite audit and separately gated semantics before runtime integration.

## 19. Current post-auth ingress is not candidate-publication activation authority

The current staged post-auth single-read ingress classifies exact `PRWZ` requester/rendezvous traffic and otherwise preserves the legacy capability path.

It does not expose a candidate-publication branch in the requester-aware retained-custody worker outcome.

FZ therefore does not alter that classifier and does not treat generic `PRWC` recognition as authorization to activate candidate-publication handling.

## 20. Cleanup error observability stays higher-owner internal

A future higher owner may need to record or surface an internal operational classification for post-commit cleanup failure.

FZ requires only that the exact typed cleanup disposition remain available after wire-semantic projection.

It does not select logging payloads, metrics labels, retry policy, process failure, shutdown policy, or recovery automation.

Those are later operational gates.

## 21. Response-write failure does not resurrect requester authority

After FY has returned committed success, requester cleanup has already been attempted according to FX/FY ordering.

If a later response write fails, FZ selects no requester-record resurrection and no semantic rollback.

The peer may have missed the Accepted frame, but durable reachability state has already advanced and requester lifecycle disposition has already been produced.

## 22. No duplicate terminal projection

One FY execution attempt must yield at most one candidate-publication terminal semantic projection for a future response owner.

FZ does not authorize projecting the same successful outcome into both Accepted and Rejected, nor emitting a second terminal result because cleanup failed.

Cleanup disposition is not a second peer response.

## 23. Privacy boundary

FZ introduces no new peer-visible diagnostics.

Typed cleanup errors remain internal and must not cause serialization of requester SessionId, expected publisher DeviceId, user/workspace identity, provider internals, transport addresses, panic/task identity, mutex state, or raw error text onto the candidate-publication wire.

## 24. Security boundary

FZ preserves all logical-identity authority already established by prior checkpoints.

Projection does not create new authorization facts. In particular:

- request ID remains correlation only;
- transport identity remains transport identity, not requester cleanup authority;
- cleanup disposition is not candidate-publication authority;
- `ReachabilityCommitOutcome` proves a completed commit but is not reusable authority for another commit;
- an Accepted frame is an observation of completed semantics, not authority to dial or start traversal.

## 25. Selected exact future adapter law

The future source adapter must be equivalent to this bounded law:

1. consume one completed FY execution result;
2. if it is an existing `CandidatePublicationExecutionError`, preserve that exact error in the bridge-compatible semantic result and produce no cleanup disposition;
3. if it is a FY successful outcome, consume that outcome into exact `ReachabilityCommitOutcome` plus exact cleanup `Result<(), RequesterRendezvousLifecycleError>`;
4. preserve the reachability outcome as bridge-compatible `Ok(...)` regardless of cleanup success/failure;
5. preserve cleanup disposition separately for higher-owner observation;
6. perform no I/O, mutation, retry, runtime drive, response framing, activation or dialing.

## 26. Focused source tests required by the next checkpoint

The next source checkpoint must prove at minimum:

- pre-commit `CandidatePublicationExecutionError` is preserved exactly and has no cleanup disposition;
- committed outcome + cleanup success projects to bridge-compatible success plus exact cleanup success;
- committed outcome + cleanup failure projects to bridge-compatible success plus the exact typed cleanup failure;
- cleanup failure cannot cause a Rejected semantic result;
- adapter consumes FY outcome without cloning/replaying operation authority;
- no bridge-to-Agent dependency inversion is introduced;
- existing bridge result-codec source remains byte-stable unless a fresh contradiction proves a minimal change is required.

## 27. Explicit non-goals

FZ does not select or materialize:

- candidate-publication command ingestion;
- candidate-publication control-loop routing;
- same-stream response custody;
- response frame I/O;
- production `ProductionReachabilityOwner` construction/recovery;
- async reachability live-owner acquisition/currentness/release composition;
- requester cleanup retry/recovery;
- traversal provisioning or polling;
- target candidate selection or dialing;
- listener bind/accept;
- readiness publication;
- Agent binary wiring;
- Android/Desktop activation;
- deployment;
- process restart/recovery;
- merge.

## 28. Exact FZ diff boundary

FZ is docs-only.

The exact FY -> FZ diff is authorized to contain only:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_FZ_CANDIDATE_PUBLICATION_POST_COMMIT_CLEANUP_TERMINAL_RESULT_PROJECTION_SEMANTICS_SELECTION_STAGING.md`

Any Rust/Kotlin source, manifest, lockfile, workflow, provider data, runtime/listener source, networking configuration, deployment path, or unrelated contract change blocks FZ closure.

## 29. Validation and durable evidence requirements

FZ may close only if one exact final head proves:

1. FY head commit `c99fb601f184b8d283a33062d50c0fc39df577fe` remains the exact merge base;
2. FZ is ahead only by the bounded docs-only contract commit;
3. exactly one changed path exists;
4. automatically triggered relevant validation reaches terminal non-failing verdict; non-applicable workflows are recorded as skipped rather than called PASS;
5. immutable Drive audit is uploaded under the canonical `Private Remote Workspace` folder, not My Drive root, and raw-read back byte-for-byte;
6. PR body changes to `Status: CLOSED` only after durable evidence succeeds;
7. PR remains draft/open/unmerged with exact head/base unchanged.

The source contract itself need not be mutated after exact-head validation merely to replace `VALIDATING`; canonical closure may be carried by immutable audit plus PR body/readback.

## 30. Selected next source-materialization boundary

After durable FZ closure, the next separately gated source checkpoint may materialize only the minimum Agent-side adapter required by this selection, conceptually:

1. one bounded projection/disposition carrier preserving bridge-compatible candidate-publication semantic result separately from post-commit cleanup disposition;
2. one pure adapter consuming the FY execution result according to Section 25;
3. focused projection/ownership tests from Section 26.

That source checkpoint must not wire candidate-publication ingress, result-frame I/O, production reachability-owner acquisition/recovery, command loops, listeners, readiness, target dialing, deployment, restart/recovery, or merge.

The expected sequential label is `C03e-GA`, but the exact successor title and path remain subject to exact-head prerequisite verification after FZ closure.

## 31. Closure statement

C03e-FZ is complete only when its exact docs-only head, validation evidence, immutable Drive audit, and draft/open/unmerged PR prove the following selected law:

**candidate-publication terminal wire success remains governed by definite durable reachability commit; FY post-commit requester cleanup disposition stays separate and internal, and cleanup failure cannot rewrite an already-committed publication into Rejected.**
