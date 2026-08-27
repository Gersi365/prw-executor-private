# Phase 152 C03e-CW — Candidate Publication Response Write Composition Selection

Status: STAGED SELECTION

Target gate:
`C03E_CW_CANDIDATE_PUBLICATION_RESPONSE_WRITE_COMPOSITION_SELECTED`

## 1. Exact predecessor

Closed C03e-CV is the authoritative predecessor:
- branch: `phase-152-c03e-cv-candidate-publication-response-error-frame-source-materialization-staging`
- head: `7fabeedc97a5f1cc8ffbab2ae884fe05e47eab98`
- tree: `6954d58ccc83e38e0e9101325c7fe8887eb7690e`
- gate: `C03E_CV_CANDIDATE_PUBLICATION_RESPONSE_ERROR_FRAME_SOURCE_MATERIALIZED`
- PR #219: body `Status: CLOSED`, draft/open/unmerged

C03e-CW is docs-only and preserves exact CV source bytes.

## 2. Fresh post-CV prerequisite audit

Exact-CV inspection establishes:
- `AuthenticatedPrwcConnection` already owns the private `ControlTlsServerStream`, authenticated logical session, connection-local `PrwcRequestIdLifecycle`, and private candidate-publication receive health state;
- `receive_candidate_publication_command(&mut self)` already performs exactly one bounded read and strict candidate-publication Command decode per call;
- peer-originated candidate-publication correlation is preserved in `CandidatePublicationControlFrame::request_id()` and never enters the local request-ID lifecycle;
- any candidate-publication frame-read or decode failure already terminalizes the private receive side and blocks later reads;
- CV now provides pure `encode_candidate_publication_execution_result_frame(command, result)` composition which emits `Response`/Accepted or generic `Error`/Rejected using exactly `command.request_id()` and performs no I/O;
- `ControlTlsServerStream::write_frame` already performs one bounded frame write followed by flush and returns `ControlFrameError` on validation/encrypted-I/O failure; it contains no retry or loop;
- the existing PRWA authentication transaction uses that same one-shot server-stream write primitive and propagates definitive write failure rather than continuing the transaction;
- the existing guarded local terminal-response writer maintains explicit healthy/poisoned write safety: invalid in-memory response before I/O does not poison, actual write failure poisons, and a poisoned transaction is rejected before consuming further input.

Runtime/process ownership of `InMemoryRequesterRendezvousAuthorityProvider` remains broader and separately unresolved. Therefore one-shot candidate-publication result writing is the lower-level prerequisite that can be selected without provider ownership, process lifetime, synchronization, routing, listener activation, or a command loop.

## 3. Selected composition boundary

A future source checkpoint may extend `AuthenticatedPrwcConnection` with one public operation semantically equivalent to:

`write_candidate_publication_execution_result(&mut self, command: &CandidatePublicationControlFrame, result: Result<ReachabilityCommitOutcome, CandidatePublicationExecutionError>) -> Result<(), AuthenticatedPrwcCandidatePublicationResultWriteError>`

Exact naming may follow repository conventions, but the semantic boundary is fixed.

The operation performs exactly:
1. reject an already-terminal candidate-publication I/O state before result composition or stream I/O;
2. call the existing CV pure helper to construct the terminal result frame using the decoded Command and completed semantic execution result;
3. if pure frame composition fails, return the local composition error with no stream write and no I/O-health poisoning;
4. otherwise call the existing private server stream `write_frame` exactly once;
5. on write failure, terminalize candidate-publication I/O health before returning the write error;
6. on successful write, return success and leave candidate-publication I/O health ready.

No semantic execution occurs inside this operation.

## 4. One shared private candidate-publication I/O health state

C03e-CW selects one private connection-local candidate-publication I/O health state equivalent to:
- `Ready`;
- `Terminal`.

It may replace/extend the current private receive-only state rather than creating unrelated read and write authorities.

The state gates both:
- candidate-publication Command reads;
- candidate-publication terminal result writes.

This state is transport/protocol safety state only. It is not requester/rendezvous authority, publication freshness, request-ID custody, reachability-owner lifecycle, session authentication, or process readiness.

## 5. Terminalization rules

The candidate-publication I/O state becomes `Terminal` on:
- existing bounded frame-read failure;
- existing candidate-publication Command decode failure;
- actual `ControlTlsServerStream::write_frame` failure while sending an already-composed candidate-publication result frame.

Once terminal, later candidate-publication read or result-write attempts must fail immediately before another stream read/write.

C03e-CW deliberately treats an actual write failure as potentially stream-desynchronizing. Continuing candidate-publication traffic after an uncertain partial/encrypted write is not selected.

## 6. Pure composition failure does not poison

Failure from the existing CV pure result-frame constructor occurs before stream I/O.

Therefore a `CandidatePublicationResultWireError` returned before `write_frame`:
- causes no stream write attempt;
- does not by itself transition candidate-publication I/O health to `Terminal`;
- is returned to the caller as a local composition failure.

This follows the existing guarded terminal-response precedent where an invalid in-memory response rejected before I/O does not poison write state.

C03e-CW does not select an automatic retry after such a failure.

## 7. Successful write does not create a loop

A successful terminal result write leaves the candidate-publication I/O state `Ready`.

This means only that the stream has not encountered a selected terminal read/decode/write failure. It does not:
- create a command loop;
- automatically call `receive_candidate_publication_command` again;
- authorize another publication;
- allocate a request ID;
- imply keepalive policy;
- imply connection persistence or shutdown policy.

Any later repeated command processing requires a separately gated composition.

## 8. Exact request-ID behavior

The future write operation must use the existing CV helper that derives correlation only from:

`command.request_id()`

It must not:
- call `PrwcRequestIdLifecycle::allocate` or equivalent;
- register the peer-originated ID in local request-ID state;
- increment or mutate local request-ID state;
- derive correlation from session, freshness, publisher, requester, candidate, or transport fields.

The connection-local request-ID lifecycle remains untouched by candidate-publication result writing.

## 9. Semantic result ordering remains external

The write operation accepts an already-completed semantic execution result.

It must not call:
- `execute_authenticated_candidate_publication`;
- requester/rendezvous authorization;
- candidate publication admission;
- freshness rotation;
- reachability durable commit;
- owner lookup or recovery.

CV already ensures `Ok(ReachabilityCommitOutcome)` maps to Accepted and every execution `Err(_)` maps to generic Rejected. CW only performs one final frame write after that pure projection.

## 10. No success before durable commit

Because the write operation consumes the existing completed execution result, an Accepted response is possible only when the earlier semantic execution has already returned `Ok(ReachabilityCommitOutcome)`.

CW selects no optimistic/pre-commit acknowledgement and no separate success code.

## 11. Receive/decode failure behavior remains unchanged

CW does not add candidate-publication Error responses for malformed/unreadable Commands.

Existing receive/decode failures:
- terminalize candidate-publication I/O health;
- return `AuthenticatedPrwcCommandReceiveError`;
- do not flow through the CV post-decode result codec.

This preserves CU/CV's explicit boundary that terminal result framing begins only after successful candidate-publication Command decode.

## 12. Selected write error surface

A future source checkpoint should expose a stable fail-closed error surface that distinguishes at least:
- already-terminal candidate-publication I/O state;
- pure CV result-frame composition failure;
- actual Phase 129 result-frame write failure.

An equivalent shape is:
- `Terminal`;
- `Result(CandidatePublicationResultWireError)`;
- `Frame(ControlFrameError)`.

Exact names may follow repository conventions. Internal errors must not be serialized into a second fallback frame.

## 13. No fallback write after write failure

If the one selected `write_frame` attempt fails, CW authorizes no second Error frame, retry, reconnect, alternate stream, or best-effort fallback.

Reason: write completion may be partial/ambiguous and candidate-publication I/O health is terminalized before returning.

## 14. No stream exposure

The private `ControlTlsServerStream` must remain private.

CW does not authorize:
- a public stream getter;
- transfer of the raw stream to external callers;
- direct external writes bypassing connection-local I/O health.

The write operation remains a narrow method on the authenticated connection boundary or an equivalent private helper invoked only through that boundary.

## 15. Future source boundary

After durable CW closure, a bounded source-materialization checkpoint may modify only:
1. one source-materialization contract;
2. `crates/prw-remote-bridge/src/prwc_connection_authentication.rs` to extend the private candidate-publication I/O health state, add the one-shot result-write operation/error surface, delegate to existing CV composition, and add focused no-socket tests.

No new public module or Cargo dependency is required by the selected design.

Any additional source path requires a fresh audit and explicit justification.

## 16. Focused future tests

A later source checkpoint must prove at least:
1. Ready + valid generic rejected result performs exactly one write and remains Ready;
2. Ready + valid accepted result performs exactly one write and preserves the decoded Command request ID;
3. already-Terminal result write fails before composition-visible I/O and writes zero frames;
4. actual result-frame write failure transitions state to Terminal before return;
5. after write failure, a later result-write attempt performs zero I/O and returns Terminal;
6. after write failure, a later `receive_candidate_publication_command` performs zero read I/O and returns existing terminal receive classification;
7. pure result-frame composition failure before I/O does not terminalize state and performs zero writes;
8. successful write does not mutate `PrwcRequestIdLifecycle`;
9. no result-write path invokes semantic candidate-publication execution;
10. no retry/fallback second write occurs after a write failure.

Tests should use a private fake frame-I/O seam and require no real socket, listener, database, thread, async runtime, or product bootstrap.

## 17. Runtime/process ownership remains separately gated

CW does not choose:
- which process owns `InMemoryRequesterRendezvousAuthorityProvider`;
- provider instance lifetime;
- how requester registrations/retirements/removals reach that instance;
- synchronization primitive or actor/task ownership;
- `ProductionReachabilityOwner` lookup/routing;
- shared/global state;
- listener/accept-loop ownership;
- runtime shutdown behavior.

Those remain separate later prerequisites.

## 18. No command loop or listener activation

CW does not authorize:
- receive -> execute -> write looping;
- repeated automatic command reads;
- task spawning;
- thread creation;
- listener binding/accept loop;
- bootstrap wiring;
- production networking activation.

It selects only one caller-invoked result-write operation on one already-authenticated connection.

## 19. No connection close/keepalive policy

CW does not select explicit socket close, TLS shutdown, keepalive, idle timeout, retry, or reconnect behavior after either successful or failed result writing.

`Terminal` means only that candidate-publication operations on this connection fail closed. Later connection-lifecycle policy remains separately gated.

## 20. No dependency or lock change

All selected semantics use existing:
- `AuthenticatedPrwcConnection` and its private server stream;
- `ControlTlsServerStream::write_frame`;
- `ControlFrameError`;
- CV result composition types/functions;
- existing candidate-publication execution result types;
- standard-library facilities.

No Cargo manifest or lockfile change is selected.

## 21. Exact C03e-CW diff boundary

C03e-CW is docs-only.

The exact CV -> CW diff is authorized to contain only:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CW_CANDIDATE_PUBLICATION_RESPONSE_WRITE_COMPOSITION_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, runtime/listener, provider-state, networking, deployment, or unrelated contract change blocks CW closure.

## 22. Safe successor after durable CW closure

After CW is durably closed, a fresh exact-head audit may authorize bounded source materialization in `prwc_connection_authentication.rs` only, plus its source-materialization contract.

After that source validates, another fresh prerequisite audit is required before selecting runtime/process ownership or any receive-execute-write transaction/loop composition.

No direct jump to listener activation, production networking, deployment, or merge is allowed.

## 23. Closure requirements

C03e-CW may close only if one exact final head proves:
1. exact CV merge base and one bounded docs-only commit;
2. only the CW contract path changed;
3. canonical automatically-triggered validation is terminal and non-failing; non-applicable workflows are recorded as `SKIPPED`, not `PASS`;
4. root and Android-native lock blobs remain byte-stable;
5. immutable Drive audit is raw-read back exactly;
6. rolling Drive predecessor is freshly guarded and preserved byte-for-byte as prefix;
7. PR body changes to `Status: CLOSED` only after durable evidence succeeds;
8. PR remains draft/open/unmerged.

Until those conditions are satisfied, C03e-CW remains staged and selects no production behavior.
