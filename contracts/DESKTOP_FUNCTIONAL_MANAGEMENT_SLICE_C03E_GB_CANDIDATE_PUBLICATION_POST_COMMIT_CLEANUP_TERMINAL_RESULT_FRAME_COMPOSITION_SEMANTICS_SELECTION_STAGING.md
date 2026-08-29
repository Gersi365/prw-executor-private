# Phase 152 C03e-GB — Candidate Publication Post-Commit Cleanup Terminal Result Frame Composition Semantics Selection

Status: VALIDATING

## 1. Purpose

C03e-GB selects only the pure terminal-result **frame-composition** semantics that apply after C03e-GA has already reduced one completed candidate-publication execution into bridge-compatible semantic result plus independent post-commit requester cleanup disposition.

GB is docs-only. It does not materialize Rust source, receive or write a control frame, accept a stream, route candidate-publication ingress, construct or recover a production reachability owner, activate reachability authority, start traversal or dialing, bind or accept a listener, publish readiness, deploy, restart/recover, or merge.

The exact C03e-GA source state is the immutable predecessor.

## 2. Exact predecessor

Canonical predecessor gate:

`C03E_GA_CANDIDATE_PUBLICATION_POST_COMMIT_CLEANUP_TERMINAL_RESULT_PROJECTION_SOURCE_MATERIALIZED`

Exact predecessor branch:

`phase-152-c03e-ga-candidate-publication-post-commit-cleanup-terminal-result-projection-source-materialization-staging`

Exact GA head commit:

`b66e30a3cabf68ffcb4fcce1abfa70ec92f4e662`

Exact GA tree:

`b8a50df4ef4f4ad03ebfe97a020a6fa4be65645b`

GA remains frozen.

## 3. Fresh post-GA audit facts

The fresh post-GA exact-head audit establishes the following existing boundaries.

First, GA materializes an Agent-side `CandidatePublicationTerminalResultProjection` carrying by value:

- bridge-compatible `Result<ReachabilityCommitOutcome, CandidatePublicationExecutionError>`;
- optional exact post-commit `Result<(), RequesterRendezvousLifecycleError>`.

GA exposes an Agent-private `into_parts()` transfer preserving both channels without flattening either classification.

Second, the existing `prw-remote-bridge` candidate-publication result codec already owns the authoritative pure terminal framing helper:

`encode_candidate_publication_execution_result_frame(command, semantic_result)`

That helper:

- accepts one already-decoded `CandidatePublicationControlFrame` by reference;
- accepts exactly the bridge-compatible semantic result shape emitted by GA;
- echoes `command.request_id()` exactly;
- maps committed semantic success to the existing Accepted payload carrying only replacement freshness;
- maps existing candidate-publication execution failure to generic Rejected;
- returns `CandidatePublicationResultWireError` if local frame construction fails;
- allocates/registers no local request ID;
- performs no frame write or stream I/O.

Third, the current Agent GA source does not yet compose its projection with that bridge framing helper.

Fourth, current post-auth control-stream ingress is a later boundary: exact `PRWZ` traffic selects requester/rendezvous custody, while other traffic including `PRWC` remains on the legacy capability path. GB does not alter that classifier.

Fifth, production `ProductionReachabilityOwner<S,T>` construction/custody is also a later independent prerequisite. GB consumes only completed semantic evidence and therefore requires no owner construction or provider authority.

## 4. The first missing prerequisite is pure frame composition

After GA, the first missing source boundary is narrower than ingress routing, same-stream response custody, response I/O, command looping, or production reachability-owner composition.

The missing boundary is a pure Agent-side adapter that consumes one completed GA projection, passes only its bridge-compatible semantic channel into the existing bridge terminal framing helper, and preserves the cleanup disposition separately.

GB therefore selects only that composition law.

## 5. Existing bridge codec remains the sole wire authority

GB selects that no Agent-local duplicate Accepted/Rejected encoder may be introduced.

Future source must call the existing `prw-remote-bridge` terminal result helper rather than reconstructing:

- PRWP magic/version fields;
- Accepted/Rejected operation tags;
- outer `Response`/`Error` kind;
- replacement freshness bytes;
- request-ID echo;
- bounded frame validation.

The bridge codec remains byte-stable unless a fresh contradiction proves a minimal bridge change is unavoidable.

## 6. Exact input custody

Future frame composition consumes exactly:

1. one already-decoded `CandidatePublicationControlFrame` by immutable reference for correlation and existing codec semantics;
2. one completed GA `CandidatePublicationTerminalResultProjection` by value.

The GA projection must be consumed exactly once into:

- exact bridge-compatible semantic result;
- exact optional cleanup disposition.

No full FY grant, requester provider, cleanup identity, mutex guard, reachability owner, stream, task, or transport owner enters this pure composition boundary.

## 7. Exact request correlation law

The decoded candidate-publication command owns the exact peer-originated request ID.

GB selects that future composition passes the exact same command reference to the existing bridge encoder.

It must not:

- allocate a local request ID;
- increment or normalize the request ID;
- substitute a capability request ID;
- derive correlation from session, device, transport, candidate, cleanup, or task identity;
- fabricate a fallback request ID.

Request ID remains correlation only and creates no authority.

## 8. Committed plus cleanup success

If GA contains:

- semantic `Ok(ReachabilityCommitOutcome)`; and
- cleanup `Some(Ok(()))`,

future composition must invoke the existing bridge encoder with that exact semantic success.

The result therefore remains eligible for the existing Accepted frame carrying the exact committed replacement freshness.

Cleanup success remains internal and adds no peer-visible field.

## 9. Committed plus cleanup failure

If GA contains:

- semantic `Ok(ReachabilityCommitOutcome)`; and
- cleanup `Some(Err(RequesterRendezvousLifecycleError))`,

future composition must still invoke the existing bridge encoder with the exact semantic success.

Therefore the frame-construction path remains Accepted-eligible regardless of cleanup failure.

The exact cleanup error must remain separately preserved for higher-owner observation and must not become:

- `CandidatePublicationExecutionError`;
- `CandidatePublicationResultWireError`;
- generic Rejected;
- a second result frame;
- a new wire status or diagnostic payload.

## 10. Pre-commit execution error

If GA contains:

- semantic `Err(CandidatePublicationExecutionError)`; and
- cleanup `None`,

future composition must pass that exact semantic error to the existing bridge framing helper.

The existing codec therefore remains responsible for generic Rejected framing.

GB selects no fabricated cleanup result on this path.

## 11. Cleanup disposition is not serialized

Future frame composition must preserve cleanup disposition separately from the constructed frame result.

Cleanup state must never affect candidate-publication wire bytes.

No peer-visible payload may include:

- cleanup success/failure bit;
- `RequesterRendezvousLifecycleError` discriminant;
- requester SessionId;
- expected publisher DeviceId;
- provider state;
- capacity state;
- retry/recovery hint;
- mutex/lock state;
- raw internal diagnostic text.

## 12. Selected two-channel frame-composition result

Future source must preserve two logically independent output channels:

1. **terminal frame construction result** — exact `Result<ControlFrame, CandidatePublicationResultWireError>` from the existing bridge codec;
2. **post-commit cleanup disposition** — exact optional `Result<(), RequesterRendezvousLifecycleError>` transferred from GA.

Conceptually:

- pre-commit failure -> existing Rejected frame construction result + cleanup absent;
- committed + cleanup success -> existing Accepted frame construction result + cleanup success;
- committed + cleanup failure -> existing Accepted frame construction result + exact cleanup failure.

Exact Rust carrier/helper naming is deferred to source materialization.

## 13. Frame-construction failure remains distinct

The existing bridge framing helper exposes `CandidatePublicationResultWireError` even though a strictly decoded non-zero request ID and fixed bounded result payload make failure tightly constrained.

GB does not erase that typed failure surface.

If local frame construction returns `Err(CandidatePublicationResultWireError)`:

- preserve that exact frame-construction failure;
- preserve cleanup disposition independently if one exists;
- do not translate it into `CandidatePublicationExecutionError`;
- do not rerun semantic execution;
- do not rerun requester cleanup;
- do not roll back durable reachability state;
- do not resurrect requester authority;
- do not manufacture generic Rejected as a fallback;
- do not retry encoding automatically.

The framing error is post-semantic local composition failure, not evidence that an already-completed semantic operation changed result.

## 14. Frame construction does not write the frame

GB selects pure construction only.

Future source must not:

- call `send_frame`;
- call capability response send helpers;
- finish a QUIC send direction;
- accept or receive a stream;
- close a peer;
- retry response I/O;
- retain or return stream custody.

Same-stream response custody and response I/O remain later gates.

## 15. No ingress classification change

GB does not alter `post_auth_control_stream_ingress`.

In particular, GB does not make `PRWC` a candidate-publication ingress discriminator and does not move any existing non-`PRWZ` traffic off the legacy capability path.

A later separately selected checkpoint must decide exact family recognition and same-stream custody before any candidate-publication wire activation.

## 16. No reachability mutation or owner construction

The GA semantic result is already completed evidence.

GB composition must not:

- call `commit_candidate_publication`;
- construct/recover/reload a `ProductionReachabilityOwner`;
- acquire/currentness-check/release live-owner authority;
- provision or poll traversal;
- retire reachability state.

Production owner custody remains a separate prerequisite.

## 17. No requester mutation

Requester cleanup has already been attempted before GA projection exists.

GB composition must not call:

- current grant selection;
- registration;
- `retire`;
- `remove_retired`;
- provider reset/sweep;
- cleanup retry or recovery.

It only preserves the completed cleanup disposition.

## 18. No duplicate projection

One GA projection may be consumed into at most one terminal frame-composition result.

GB does not authorize:

- emitting both Accepted and Rejected for the same semantic result;
- constructing a second peer result because cleanup failed;
- projecting the same completed FY execution twice;
- replaying `ReachabilityCommitOutcome` as authority for another commit.

## 19. Error taxonomy remains layered

GB preserves three distinct categories:

1. `CandidatePublicationExecutionError` — pre/at-commit semantic failure;
2. `RequesterRendezvousLifecycleError` — post-commit cleanup disposition, present only after definite semantic success;
3. `CandidatePublicationResultWireError` — local terminal frame-construction failure.

No category is silently flattened into another.

The higher owner may later choose an operational disposition, but GB selects no logging, metrics, peer-close, process-failure, retry, or shutdown policy.

## 20. Security boundary

GB creates no new authorization fact.

In particular:

- request ID remains correlation only;
- `ControlFrame` construction is not permission to send or dial;
- `ReachabilityCommitOutcome` is completed-commit evidence, not reusable commit authority;
- cleanup disposition is not requester authority;
- frame construction success does not activate ingress, traversal, connectivity selection, or target dialing.

## 21. Privacy boundary

The existing candidate-publication peer wire remains unchanged.

Future composition must not serialize internal cleanup/provider/identity details because framing delegates to the existing bridge codec with only its existing semantic result.

Typed internal errors may be retained for higher-owner handling but no new raw error text reaches the peer.

## 22. Pure-before-I/O sequencing

GB selects explicit layering:

1. completed FY semantic execution and cleanup;
2. GA semantic/cleanup projection;
3. GB pure terminal frame composition;
4. separately gated same-stream custody and response I/O;
5. separately gated ingress/loop/runtime activation.

This prevents response transport concerns from changing completed semantic truth.

## 23. Focused source proofs required next

The next source-materialization checkpoint must prove at minimum:

- pre-commit semantic error uses existing bridge generic Rejected construction with exact command request ID and cleanup absent;
- committed + cleanup success uses existing bridge Accepted construction with exact replacement freshness/request ID and preserves cleanup success separately;
- committed + typed cleanup failure still uses existing bridge Accepted construction and preserves the exact cleanup error separately;
- cleanup failure cannot alter constructed candidate-publication result bytes;
- composition delegates to existing bridge codec rather than duplicating wire logic;
- GA projection is consumed without cloning/replaying operation authority;
- no frame write, stream custody, ingress classification, reachability mutation, requester mutation, retry, activation, or dialing is introduced.

If exact source topology permits deterministic exercise of `CandidatePublicationResultWireError`, focused tests should also prove that frame-construction failure remains distinct while cleanup disposition is preserved. If strict typed inputs make that failure unreachable in the pure higher-level helper, source materialization must preserve the typed lower-layer error surface without fabricating invalid state solely for a test.

## 24. Exact GB diff boundary

GB is docs-only.

The exact GA -> GB diff is authorized to contain only:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_GB_CANDIDATE_PUBLICATION_POST_COMMIT_CLEANUP_TERMINAL_RESULT_FRAME_COMPOSITION_SEMANTICS_SELECTION_STAGING.md`

Any Rust/Kotlin source, manifest, lockfile, workflow, provider state, runtime/listener source, networking configuration, deployment path, or unrelated contract change blocks GB closure.

## 25. Validation and durable evidence requirements

GB may close only if one exact final head proves:

1. GA head `b66e30a3cabf68ffcb4fcce1abfa70ec92f4e662` remains the exact merge base;
2. GB is ahead only by the bounded docs-only contract commit;
3. exactly one changed path exists;
4. automatically triggered relevant validation reaches terminal non-failing verdict; non-applicable workflows are recorded as skipped rather than called PASS;
5. immutable Drive audit is uploaded directly under canonical `Private Remote Workspace` folder ID `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`, never My Drive root;
6. raw Drive readback matches local bytes and SHA-256 exactly;
7. PR body changes to `Status: CLOSED` only after durable evidence succeeds;
8. PR remains draft/open/unmerged with exact base/head unchanged.

The source contract may remain `Status: VALIDATING` after exact-head validation; canonical closure may be carried by immutable Drive evidence plus PR body/readback so the validated branch remains byte-stable.

## 26. Selected next source-materialization boundary

After canonical GB closure, the next separately gated source checkpoint may materialize only the minimum Agent-side pure frame-composition adapter selected above, conceptually:

1. one bounded carrier preserving exact bridge terminal frame-construction result separately from optional exact cleanup disposition;
2. one pure helper consuming GA projection and delegating semantic framing to the existing bridge encoder with the exact decoded command;
3. focused ownership/projection/framing tests from Section 23.

That source checkpoint must not add same-stream response I/O, candidate-publication ingress classification, production reachability-owner construction/recovery, command loops, listeners, readiness, traversal/dialing, deployment, restart/recovery, or merge.

The expected sequential label is `C03e-GC`, subject to exact-head prerequisite verification after GB closure.

## 27. Closure statement

C03e-GB is complete only when its exact docs-only head, validation evidence, immutable Drive audit, and draft/open/unmerged PR prove the selected law:

**GA semantic truth is framed only through the existing bridge candidate-publication result codec; durable-commit success remains Accepted-eligible independent of post-commit cleanup disposition, cleanup remains internal, and local frame-construction failure remains a distinct post-semantic error without replay, rollback, retry, or response-I/O activation.**