# Phase 152 — C03e-GN Candidate Publication Current-Mesh Same-Stream Terminal Response Custody Semantics Selection Staging

Status: SEMANTICS_SELECTION_STAGING

## 1. Purpose

C03e-GN selects only the current-Mesh candidate-publication same-stream terminal-response custody semantics required after canonically CLOSED C03e-GM.

This checkpoint is docs-only.

It does not source-materialize response I/O, remove the current Agent candidate handoff barrier, invoke candidate semantic execution, populate production-owner custody, integrate a worker, activate traversal, bind a listener, publish readiness, dial, deploy, restart/recover a process, merge, delete branches, or change repository visibility.

## 2. Canonical predecessor

C03e-GM is canonically CLOSED and frozen.

Exact GM branch:
`phase-152-c03e-gm-candidate-publication-current-mesh-terminal-result-projection-frame-composition-source-materialization-staging`

Exact GM final head:
`8acd5db69397c6136e83563272419679546c19da`

Exact GM final tree:
`cb2157dfcd9c3bc7264b6889cec08739dfa84c5c`

GM PR:
`#315`

GM PR remains:
- open;
- draft;
- unmerged;
- `Status: CLOSED`.

Canonical GM immutable Drive object:
`1TfT4AZy0PJvOBvxtDfZBQ0fJHVbGO3JC`

Canonical GM audit bytes:
`12177`

Canonical GM audit SHA-256:
`33faf66d773ea369d9bb2eec95510e804f214e076eb02c011e3de6a0ad1d2093`

C03e-GN starts exactly from the GM final head and does not amend GM.

## 3. Fresh exact-GM source finding

The fresh exact-final-head audit establishes all of the following simultaneously.

### 3.1 Existing current-Mesh candidate transaction custody

`PostAuthCandidatePublicationTransaction` already retains exactly:
- one strict `CandidatePublicationMeshRequest`; and
- the exact same already-accepted `MeshControlStream` on which that request was received.

The transaction currently exposes:
- `request()` for bounded borrowing; and
- `into_parts()` for by-value custody transfer.

It currently exposes no candidate-specific current-Mesh response send surface.

### 3.2 Exact retained request correlation

`CandidatePublicationMeshRequest` retains the exact non-zero current-Mesh outer `request_id` observed on the peer request.

That request ID remains transaction correlation only.

It is not:
- publisher logical identity;
- authenticated session identity;
- transport identity;
- owner lookup identity;
- requester/rendezvous authority;
- reachability authority;
- freshness authority;
- durable state authority;
- candidate endpoint identity;
- socket identity.

### 3.3 Existing GM terminal frame composition

C03e-GM already source-materializes pure current-Mesh terminal frame composition using:
`prw_remote_transport::ControlFrame`.

GM preserves existing PRWP terminal-result bytes and maps:
- Accepted -> current-Mesh `Response`;
- Rejected -> current-Mesh `Error`.

GM performs no stream I/O.

GM frame construction remains a separate local failure layer through:
`CandidatePublicationMeshResultFrameError`.

### 3.4 Existing lower stream send semantics

`MeshControlStream::send_frame(...)` already:
1. encodes exactly one bounded current-Mesh `ControlFrame`;
2. writes the full encoded bytes under the existing operation timeout;
3. finishes the QUIC send direction;
4. reports existing `MeshQuicRuntimeError` classes for timeout/write/finish failure.

It does not perform semantic candidate interpretation.

### 3.5 Existing requester/capability precedent

The bridge already contains two same-stream response-custody precedents.

`PostAuthRequesterRendezvousTransaction::send_dr_acknowledgement_frame(...)`:
- consumes the retained transaction by value;
- sends one already-constructed frame on the exact retained stream;
- maps lower runtime failure into a requester-specific response-I/O error;
- does not reconstruct requester semantics;
- does not retry;
- does not return stream custody.

`PostAuthCapabilityTransaction::send_response_frame(...)` similarly consumes exact retained stream custody for one already-constructed response.

Candidate response custody should preserve this layering rather than invent a second stream/runtime mechanism.

### 3.6 Agent handoff barrier remains active

Agent source still preserves:
`AuthenticatedRemoteSessionPostAuthIngressTransactionError::CandidatePublicationHandoffNotSelected`.

Therefore current-Mesh candidate ingress remains dormant at the higher-owner seam.

C03e-GN does not remove or weaken that barrier.

## 4. Selected custody law

A future source materialization may add exactly one candidate-specific consuming same-stream terminal-response send seam on the existing:
`PostAuthCandidatePublicationTransaction`.

The seam must:
1. consume the transaction by value;
2. retain no duplicate stream custody outside the consumed value;
3. accept one already-constructed current-Mesh `ControlFrame` supplied by the higher owner;
4. require exact frame request correlation equality with the retained `CandidatePublicationMeshRequest::request_id()` before any stream I/O;
5. on correlation equality, send that exact frame once through the exact retained `MeshControlStream`;
6. delegate write + send-direction finish only to existing `MeshControlStream::send_frame(...)`;
7. consume local transaction custody regardless of terminal success/failure;
8. return no stream custody after the attempt;
9. perform no second read;
10. perform no retry, fallback send, reconnect, peer close, loop resume or alternate stream open/accept.

## 5. Exact request-correlation law

The only valid response correlation for this transaction is:
`transaction.request().request_id()`.

The future send seam must compare the supplied frame's request ID against that exact retained value before calling `send_frame(...)`.

If the supplied frame request ID differs:
- fail closed locally;
- perform zero stream writes;
- perform zero stream finish attempts;
- do not rewrite the frame request ID;
- do not allocate a replacement request ID;
- do not construct a Rejected fallback frame;
- do not replay candidate semantics;
- do not reopen or reaccept a stream;
- do not return stream custody.

Correlation mismatch remains a local response-custody invariant failure, not peer semantic rejection.

## 6. No identity promotion from request correlation

Exact request-ID equality is only transaction lineage validation.

It must never be interpreted as:
- logical device identity;
- authenticated publisher authority;
- `PeerConnectivityIdentity`;
- `DeviceId`;
- `TransportIdentity`;
- requester authority;
- reachability-owner selection;
- candidate admission authority;
- durable compare-and-commit authority.

The canonical dynamic-network identity law remains:

`logical device/session identity -> registry/discovery -> current reachable endpoint/candidates -> authenticated transport`

Not:

`device identity = static IP`

And not:

`device identity = request_id`

## 7. Selected response-I/O error taxonomy

Future source materialization should introduce one candidate-specific same-stream response-custody error classification.

It must preserve at least two distinct local classes:

### 7.1 Correlation mismatch

A supplied terminal frame does not carry the exact retained candidate request ID.

This class occurs before stream I/O.

It is not a `MeshQuicRuntimeError` and is not semantic Rejected.

### 7.2 Runtime I/O failure

Existing `MeshControlStream::send_frame(...)` returns `MeshQuicRuntimeError` for timeout/write/finish/transport failure.

The candidate-specific error should wrap/preserve that typed lower runtime cause rather than flattening it into:
- semantic candidate failure;
- owner lookup failure;
- frame construction failure;
- requester cleanup failure;
- generic bridge rejection.

## 8. Semantic / frame-construction / response-I/O separation

C03e-GN locks three separate layers.

### Layer A — semantic execution

Existing current-Mesh candidate semantic execution remains higher-owner logic.

Its success/failure determines Accepted vs Rejected projection before response I/O.

### Layer B — pure GM frame construction

GM composes one current-Mesh terminal `ControlFrame` without stream I/O.

GM local frame-construction failure remains:
`CandidatePublicationMeshResultFrameError`.

A frame-construction failure means there is no frame to send.

### Layer C — GN same-stream response custody

Only after a frame exists may the retained candidate transaction attempt one same-stream write+finish.

A response-I/O failure does not reinterpret the already-completed candidate semantic result.

These layers must not be flattened into one error enum that loses the distinction between:
- semantic failure;
- GM frame-construction failure;
- GN correlation mismatch;
- GN runtime response-I/O failure.

## 9. No semantic replay after response-I/O failure

Once higher-owner candidate semantics have already completed and GM frame composition has already completed, any GN response-custody failure must not authorize:
- a second candidate semantic execution;
- a second durable reachability commit;
- freshness reissue;
- requester cleanup replay;
- owner lookup replay;
- requester grant replay;
- fallback Rejected construction;
- second GM composition attempt;
- alternate request correlation;
- new stream open;
- new stream accept;
- reconnect;
- automatic peer close;
- traversal activation.

## 10. Cleanup disposition remains higher-owner state

GM may preserve an opaque higher-owner success disposition beside frame composition.

GN response custody must not inspect, serialize, modify, retry or replay that disposition.

In particular, requester cleanup outcome remains distinct from response I/O.

A response-I/O failure after definite durable commit does not:
- roll back durable state;
- restore prior freshness;
- reactivate requester state;
- convert committed success into semantic Rejected;
- cause cleanup replay.

## 11. Frame payload and kind are not reparsed by GN

GN selects no second candidate terminal codec.

The future same-stream send seam receives one already-constructed current-Mesh frame.

It should not:
- decode PRWP terminal payload again;
- inspect replacement freshness;
- reclassify Accepted/Rejected;
- rebuild `ControlMessageKind`;
- convert to historical `prw_control_transport::ControlFrame`;
- fabricate historical `CandidatePublicationControlFrame`;
- fabricate `AuthenticatedPrwcConnection`;
- rewrite payload bytes.

The only transaction-specific invariant enforced at this custody boundary is exact retained request-ID equality before I/O.

## 12. Same exact stream law

The response must use the exact `MeshControlStream` already retained by the exact `PostAuthCandidatePublicationTransaction` produced by the one-read ingress.

Forbidden substitutions include:
- a newly opened control stream;
- a newly accepted control stream;
- another transaction's stream;
- historical TLS/PRWC stream ownership;
- a cloned stream handle;
- a reconstructed stream wrapper.

No second request read occurs.

## 13. Consuming ownership law

The future response seam must consume:
`PostAuthCandidatePublicationTransaction`
by value.

This proves there is no simultaneous bridge-level custody of the same candidate transaction after send begins.

On success:
- exactly one send attempt completed;
- send direction is finished by existing lower runtime behavior;
- transaction custody is consumed.

On correlation mismatch:
- transaction custody is consumed;
- no stream I/O occurs.

On runtime send failure:
- transaction custody is consumed;
- no retry occurs;
- no stream is returned.

## 14. No loop/cancellation policy selection

C03e-GN does not decide what a higher owner should do after candidate terminal response success or failure.

It does not select:
- repeated mixed-family ingress continuation;
- worker termination;
- worker cancellation;
- peer-close policy;
- supervisor restart;
- connection reuse policy;
- endpoint lifecycle changes.

Those remain separately gated.

## 15. No Agent candidate execution activation

C03e-GN does not wire together:
- `PostAuthCandidatePublicationTransaction`;
- authenticated publisher session;
- GK semantic execution;
- GM result composition;
- GN response send.

It only selects the bridge-owned response-custody primitive required before such higher-owner orchestration could later be considered.

`CandidatePublicationHandoffNotSelected` remains unchanged.

## 16. No production-owner runtime population

C03e-GN does not create/populate/recover/synchronize production owner maps at runtime.

It does not add:
- `Arc` custody;
- mutexes;
- Tokio mutexes;
- actors/mailboxes;
- task-per-owner synchronization;
- owner-map population loops;
- owner-map transport-rotation logic.

GI/GK ownership semantics remain unchanged.

## 17. No transport-runtime changes

Existing `MeshControlStream::send_frame(...)` is sufficient for the selected custody semantics.

C03e-GN therefore selects no modification to:
- `prw-remote-transport` runtime;
- Quinn stream mechanics;
- operation timeout;
- frame encoding bounds;
- endpoint open/accept logic;
- mTLS peer identity validation.

## 18. Expected source-materialization scope

After canonical GN closure, a fresh exact-final-head audit may authorize a narrow source checkpoint that modifies only the minimum bridge source needed to materialize the selected primitive.

The expected source focus is the existing:
`crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`

plus one new checkpoint contract.

No manifest or lockfile change is expected because the bridge already depends on current remote transport and already imports `MeshControlStream`, `MeshQuicRuntimeError`, and `ControlFrame` in this module.

Any broader path requirement must be justified by a fresh exact-head audit rather than assumed by GN.

## 19. Focused future tests selected

A later GN source-materialization successor should prove at minimum:

1. the candidate response method consumes the exact transaction type by value;
2. exact retained request ID + matching frame request ID reaches the lower send seam;
3. mismatched request ID fails before any write;
4. mismatch does not rewrite or regenerate request correlation;
5. lower `MeshQuicRuntimeError` maps into the candidate-specific response-I/O class;
6. no semantic candidate result is reconstructed by the send seam;
7. no stream is returned after success/failure;
8. existing requester/capability behavior is unchanged;
9. candidate ingress decode/custody behavior remains unchanged;
10. no historical frame type is introduced.

Tests must not require listener/readiness activation or production network deployment.

## 20. Exact GN authorized path ceiling

C03e-GN itself authorizes exactly one docs-only path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_GN_CANDIDATE_PUBLICATION_CURRENT_MESH_SAME_STREAM_TERMINAL_RESPONSE_CUSTODY_SEMANTICS_SELECTION_STAGING.md`

No Rust source, `Cargo.toml`, `Cargo.lock`, workflow, Android/Kotlin/Gradle, Agent runtime source, bridge runtime source, transport runtime source, listener, readiness, persistence, deployment, configuration or unrelated contract path may change in GN.

## 21. Validation law

GN validation must bind only to the exact final GN head.

Required observation:
- PRW Rust Validation must complete successfully on exact final head;
- any path-filtered workflows must be recorded accurately as PASS/skipped/failure rather than inherited;
- Android is not required merely because prior source checkpoints used it; if no Android workflow is triggered for this docs-only head, no Android PASS may be claimed.

No manually dispatched validation is authorized.

## 22. Immutable evidence closure law

After exact-final-head validation:
1. freeze GN source/contract state;
2. record commit/tree/compare/path/blob evidence;
3. create an immutable GN audit locally;
4. record exact local bytes and SHA-256;
5. upload those exact bytes directly into canonical Drive folder `Private Remote Workspace / 1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
6. raw-fetch that exact Drive object;
7. recompute readback bytes and SHA-256;
8. require exact byte/hash equality;
9. reread GN branch/PR state;
10. only then set PR metadata to `Status: CLOSED` while keeping it draft/open/unmerged;
11. independently reread PR and branch after closure.

No My Drive root staging is authorized.

## 23. Canonical GN closure target

Canonical closure:
`CLOSED_CANDIDATE_PUBLICATION_CURRENT_MESH_SAME_STREAM_TERMINAL_RESPONSE_CUSTODY_SEMANTICS_SELECTION`

Canonical gate:
`C03E_GN_CANDIDATE_PUBLICATION_CURRENT_MESH_SAME_STREAM_TERMINAL_RESPONSE_CUSTODY_SEMANTICS_SELECTED`

## 24. Explicit exclusions

C03e-GN does not authorize:
- source materialization beyond a later separately gated successor;
- Agent candidate higher-owner handoff activation;
- GK execution invocation from ingress;
- GM composition invocation from ingress;
- same-stream response I/O during GN itself;
- repeated command-loop integration;
- cancellation semantics;
- peer-close policy;
- production-owner population/synchronization;
- traversal activation;
- listener/readiness activation;
- dialing;
- deployment;
- process restart/recovery;
- merge;
- branch deletion;
- repository visibility change.

## 25. Successor rule

After canonical GN closure, perform a fresh exact-final-head audit before any successor mutation.

The likely next checkpoint is narrow source materialization of only the selected candidate-specific consuming same-stream terminal-response custody primitive, including exact request-correlation equality and typed response-I/O failure preservation.

That successor must not automatically include Agent handoff activation, semantic execution orchestration, result composition orchestration, worker integration, owner-map runtime population, traversal, listener/readiness, dialing, deployment, restart/recovery or merge.
