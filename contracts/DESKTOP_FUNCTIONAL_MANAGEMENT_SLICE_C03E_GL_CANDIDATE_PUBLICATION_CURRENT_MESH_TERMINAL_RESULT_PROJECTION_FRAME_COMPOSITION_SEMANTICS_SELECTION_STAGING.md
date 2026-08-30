# Phase 152 C03e-GL — Candidate Publication Current-Mesh Terminal Result Projection / Frame Composition Semantics Selection

Status: VALIDATING

Target gate:
`C03E_GL_CANDIDATE_PUBLICATION_CURRENT_MESH_TERMINAL_RESULT_PROJECTION_FRAME_COMPOSITION_SEMANTICS_SELECTED`

## 1. Exact predecessor

Canonically CLOSED C03e-GK is the authoritative predecessor:

- branch: `phase-152-c03e-gk-candidate-publication-current-mesh-authority-owner-custody-execution-decomposition-source-materialization-staging`;
- final head: `d71533ab55105b1ef73b819303c6d3c0c41b9fa4`;
- final tree: `ec37ac9f603b5c1aebca89fa624536da8ec60a8e`;
- PR #313: `Status: CLOSED`, draft/open/unmerged;
- immutable GK Drive object: `1oC2HS9ygZufAbEcUs4nOoEDVEmEHWTA3`;
- immutable GK bytes: `12255`;
- immutable GK SHA-256: `13865fbfa931fe641ee8e0489b43a328005136b3c8e9ca6a10af5b5e6729719c`;
- closure: `CLOSED_CANDIDATE_PUBLICATION_CURRENT_MESH_AUTHORITY_OWNER_CUSTODY_EXECUTION_DECOMPOSITION_SOURCE_MATERIALIZATION`;
- gate: `C03E_GK_CANDIDATE_PUBLICATION_CURRENT_MESH_AUTHORITY_OWNER_CUSTODY_EXECUTION_DECOMPOSITION_SOURCE_MATERIALIZED`.

GK remains frozen. GL starts exactly from the GK final commit and does not amend any closed predecessor.

## 2. Fresh exact-GK source audit

The post-GK audit re-read current-Mesh ingress custody, candidate terminal-result codec source, and current-Mesh transport source rather than assuming that the historical terminal codec can be reused by type.

The audit established all of the following:

1. `CandidatePublicationMeshRequest` retains the exact non-zero current-Mesh outer `request_id` plus one strict `CandidatePublicationWireSubmission`.
2. That `request_id` is explicitly correlation only and is not publisher, requester, transport, freshness, owner, durable, or cleanup authority.
3. `PostAuthCandidatePublicationTransaction` retains the exact `CandidatePublicationMeshRequest` plus the exact same already-accepted `MeshControlStream` by value.
4. The transaction currently exposes only `request()` and by-value `into_parts()`; there is no candidate-publication current-Mesh response send surface.
5. The existing PRWP terminal-result semantics already define only two peer-visible terminal outcomes: Accepted with verifier-issued replacement freshness, or generic Rejected.
6. Existing historical terminal framing uses `prw_control_transport::ControlFrame` and historical `CandidatePublicationControlFrame` request-correlation ownership.
7. Current Mesh uses a distinct `prw_remote_transport::ControlFrame` and `MeshControlStream::send_frame(...)` accepts that current-Mesh frame type.
8. `prw_remote_transport::ControlFrame` already supports current-Mesh `Response` and `Error` kinds, requires a non-zero request ID, and preserves bounded payload bytes.
9. `MeshControlStream::send_frame(...)` writes exactly one current-Mesh frame and finishes the send direction, but response I/O is a separate ownership/failure boundary and need not be selected in GL.
10. GK semantic execution remains dormant and does not consume current-Mesh transaction, stream, or request correlation.

Therefore the next prerequisite is not current-Mesh handoff activation and not stream I/O. It is the exact pure current-Mesh terminal-result projection/frame-composition law that can later connect one completed GK semantic result to the exact current-Mesh correlation while preserving all existing PRWP peer-visible semantics.

## 3. Why historical frame reuse by type is rejected

GL rejects fabricating or translating through historical control-envelope ownership merely to reuse an existing helper signature.

A future current-Mesh result adapter must not construct:

- historical `prw_control_transport::ControlFrame` as the current-Mesh response object;
- historical `CandidatePublicationControlFrame` solely to recover request correlation;
- historical `AuthenticatedPrwcConnection`;
- a second decoded candidate command;
- a replacement current-Mesh request.

Current-Mesh correlation already exists in `CandidatePublicationMeshRequest::request_id()`. Current-Mesh frame construction authority already exists in `prw_remote_transport::ControlFrame::new(...)`.

GL selects semantic reuse, not historical envelope fabrication.

## 4. Existing PRWP terminal-result semantics remain authoritative

The existing candidate terminal-result law remains the semantic baseline:

- definite semantic success projects to `CandidatePublicationResultMessage::Accepted`;
- Accepted carries exactly the verifier-issued replacement freshness token from `ReachabilityCommitOutcome`;
- semantic failure projects to `CandidatePublicationResultMessage::Rejected`;
- Rejected exposes no internal error detail;
- Accepted uses outer response kind;
- Rejected uses outer error kind;
- the terminal PRWP payload remains bounded and versioned;
- request ID is echoed correlation only.

GL does not change PRWP operation numbers, magic, version, reserved fields, accepted-result size, rejected-result size, or freshness-token encoding.

## 5. GK result shape that GL must consume

GK returns:

`Result<CandidatePublicationPostCommitRequesterCleanupOutcome, CurrentMeshCandidatePublicationExecutionError>`

The successful outcome retains two conceptually distinct channels:

1. definite durable `ReachabilityCommitOutcome`;
2. independent post-commit `Result<(), RequesterRendezvousLifecycleError>` cleanup disposition.

The failure type retains two Agent-local layers:

1. `Semantic(CandidatePublicationExecutionError)`;
2. `OwnerLookup(ProductionReachabilityOwnerCustodyLookupError)`.

GL must preserve those internal distinctions for local handling while projecting only the selected generic peer-visible terminal result.

## 6. Selected peer-visible projection law

GL selects exactly this projection:

### 6.1 Definite committed success

`Ok(CandidatePublicationPostCommitRequesterCleanupOutcome { .. })`

projects to current-Mesh candidate publication Accepted.

The peer-visible Accepted result carries exactly:

`reachability_commit.replacement_freshness()`

The internal `invalidated_traversal` disposition remains unexposed exactly as in the existing historical terminal-result projection law.

### 6.2 Committed success plus cleanup success

A definite durable commit followed by requester-record cleanup success remains Accepted.

Cleanup success is not serialized.

### 6.3 Committed success plus cleanup failure

A definite durable commit followed by exact requester-record cleanup failure also remains Accepted.

The exact typed cleanup error remains internal beside the already-committed semantic success.

Cleanup failure must not:

- become generic Rejected;
- alter replacement freshness;
- trigger semantic replay;
- trigger durable rollback;
- trigger requester-record resurrection;
- trigger automatic cleanup retry;
- trigger a second frame-composition attempt.

### 6.4 Existing semantic failure

`Err(CurrentMeshCandidatePublicationExecutionError::Semantic(_))`

projects to generic Rejected.

No lower semantic error detail is serialized.

### 6.5 Exact GI owner-lookup failure

`Err(CurrentMeshCandidatePublicationExecutionError::OwnerLookup(_))`

also projects to generic Rejected for the peer-visible terminal candidate result.

Internally `Missing` and `Ambiguous` remain distinguishable before projection. GL does not flatten the Agent-local error type itself; it only selects one generic external rejection projection after semantic execution has terminated.

No lookup retry, alternate owner selection, DeviceId-only fallback, or single-owner fallback is selected.

## 7. Cleanup disposition remains a separate local channel

GL preserves post-commit cleanup state separately from the current-Mesh frame-construction result.

The future pure composition result must be able to retain:

- one current-Mesh frame-construction result; and
- optional exact post-commit cleanup disposition.

The cleanup channel is:

- absent after any pre/at-commit GK error;
- present after definite commit success;
- `Some(Ok(()))` after successful cleanup;
- `Some(Err(RequesterRendezvousLifecycleError))` after cleanup failure.

No cleanup state is serialized to the peer wire.

## 8. Exact current-Mesh correlation source

The future current-Mesh terminal frame must echo exactly:

`CandidatePublicationMeshRequest::request_id()`

from the same retained `PostAuthCandidatePublicationTransaction` lineage.

GL forbids request correlation from:

- locally allocated counters;
- historical candidate command fabrication;
- requester `SessionId`;
- publisher `DeviceId`;
- `TransportIdentity`;
- owner-map key;
- freshness token;
- candidate endpoint;
- socket address;
- any regenerated frame.

The request ID remains correlation only and grants no identity or authorization.

## 9. Current-Mesh frame type is mandatory

The future frame-composition result must use:

`prw_remote_transport::ControlFrame`

and not historical:

`prw_control_transport::ControlFrame`.

The existing current-Mesh constructor remains authoritative for:

- non-zero request correlation;
- bounded payload length;
- exact current-Mesh outer message kind;
- PRWM encoding validity.

No cross-transport frame wrapper is selected.

## 10. Current-Mesh outer kind law

The selected current-Mesh outer kind is aligned with the existing terminal semantics:

- Accepted -> `prw_remote_transport::ControlMessageKind::Response`;
- Rejected -> `prw_remote_transport::ControlMessageKind::Error`.

GL does not select `Request`, `Event`, `Heartbeat`, or `SessionAuthentication` as a candidate terminal result kind.

## 11. PRWP payload bytes remain semantically stable

GL selects reuse of the existing PRWP terminal-result payload law, not a new current-Mesh-specific semantic payload protocol.

A later source materialization may factor a pure payload encoder or otherwise reuse the existing constants/semantic projection, provided that it proves byte-equivalence for:

- terminal magic;
- major/minor version;
- accepted operation;
- rejected operation;
- reserved field;
- accepted replacement-freshness bytes;
- rejected header-only payload;
- exact accepted/rejected lengths.

GL does not pre-select the internal refactor shape.

## 12. Historical codec behavior must remain unchanged

Any later source materialization must preserve existing historical tests and behavior for:

- `encode_candidate_publication_result_frame(...)`;
- `decode_candidate_publication_result_frame(...)`;
- `project_candidate_publication_execution_result(...)`;
- `encode_candidate_publication_execution_result_frame(...)`;
- `CandidatePublicationResultFrameComposition<D>`.

A current-Mesh adapter may share lower pure payload/projection logic, but it must not mutate historical envelope ownership or change historical wire bytes.

## 13. Current-Mesh frame construction failure is a distinct local failure layer

A future current-Mesh frame constructor may fail locally because current `prw_remote_transport::ControlFrame::new(...)` rejects its supplied kind/request ID/payload.

That frame-construction failure is not:

- `CandidatePublicationExecutionError`;
- `ProductionReachabilityOwnerCustodyLookupError`;
- `RequesterRendezvousLifecycleError`;
- a durable commit rollback signal;
- a reason to rerun GK semantics;
- a reason to rerun cleanup;
- a reason to fabricate a generic fallback frame automatically.

GL selects a distinct current-Mesh frame-construction error/result channel.

## 14. No fallback frame after local composition failure

If current-Mesh terminal frame construction fails, GL selects fail-closed local termination of that composition attempt.

It does not authorize:

- automatic fallback Rejected construction;
- a second request ID;
- alternate payload encoding;
- historical frame fallback;
- semantic replay;
- cleanup replay;
- second durable commit;
- stream replacement;
- response retry.

The later same-stream I/O checkpoint may separately select peer/session disposition after such a local failure.

## 15. Same-stream custody is preserved but not consumed

GL requires preservation of the exact original `PostAuthCandidatePublicationTransaction` lineage so later I/O can consume the exact same accepted `MeshControlStream`.

GL itself does not call:

- `PostAuthCandidatePublicationTransaction::into_parts()` for active I/O;
- `MeshControlStream::send_frame(...)`;
- `receive_frame()`;
- `accept_control_stream()`;
- `open_control_stream()`.

No stream is cloned, replaced, reaccepted, reread, or finished by GL.

## 16. No response I/O semantics are selected

Although exact source shows `MeshControlStream::send_frame(...)` writes one frame and finishes the send direction, GL deliberately stops before response I/O.

GL does not select:

- send retry;
- write retry;
- finish retry;
- response timeout handling above the existing transport taxonomy;
- peer close behavior after write/finish failure;
- loop continuation after response success;
- loop termination after response failure;
- stream reuse after finish.

Those remain a later separately gated same-stream response-custody checkpoint.

## 17. No current-Mesh candidate handoff activation

`AuthenticatedRemoteSessionPostAuthIngressTransactionError::CandidatePublicationHandoffNotSelected`

remains unchanged and active after GL.

GL does not wire the current-Mesh candidate transaction into GK execution.

The future handoff must remain separately gated because it also requires concrete production-owner map custody and runtime ownership that GK/GL intentionally do not populate or synchronize.

## 18. No owner-map runtime population or synchronization

GL does not select:

- startup recovery schedule;
- construction of all production-owner custodies;
- insertion/removal after transport rotation;
- lifecycle retirement scheduling;
- `reload_from_store()` orchestration;
- `Arc<Mutex<ProductionReachabilityOwnerCustodyMap<...>>>`;
- Tokio mutex around the map;
- per-peer locks;
- actors/mailboxes;
- owner background tasks;
- concurrent candidate execution policy.

Current-Mesh framing semantics must not be used as implicit authorization for those runtime ownership choices.

## 19. No cancellation semantics

GL selects no cancellation point before, during, or after:

- GK semantic execution;
- definite durable commit;
- requester cleanup;
- frame composition;
- later same-stream response I/O.

Cancellation may affect committed-but-not-yet-responded lifecycle handling and therefore remains separately gated.

## 20. Error privacy and peer-visible rejection

Generic Rejected remains intentionally non-diagnostic.

GL does not expose to the publisher peer whether rejection arose from:

- registry/session currentness;
- presented transport currentness;
- candidate validation;
- requester authority;
- expected-publisher mismatch;
- owner lookup Missing;
- owner lookup Ambiguous;
- stale freshness;
- token-source failure;
- durable CAS conflict;
- persistence ambiguity/failure.

Detailed internal typed errors remain local evidence/diagnostic state only.

## 21. Dynamic-network identity invariant remains unchanged

GL does not assign identity or authorization meaning to an IP address, socket endpoint, candidate endpoint, or request ID.

Candidate endpoint information remains transient dynamic reachability data under exact logical `DeviceId` + current `TransportIdentity` lifecycle authority.

Nothing in terminal framing changes the canonical identity chain:

`logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`.

## 22. Pure-composition boundary selected

The future source materialization selected by GL must remain pure with respect to network/runtime side effects.

Its permitted responsibilities are limited to:

- projecting one completed GK result into Accepted/Rejected semantics;
- preserving exact post-commit cleanup disposition separately;
- taking exact current-Mesh request correlation as explicit correlation input;
- composing one current `prw_remote_transport::ControlFrame`;
- preserving existing PRWP terminal payload semantics;
- returning typed local frame-construction failure without performing I/O.

## 23. Source ownership expectation

GL does not pre-authorize exact source paths for the later source checkpoint beyond requiring a fresh audit.

The likely minimum seam spans bridge-owned terminal-result semantics and current-Mesh frame ownership, but any materialization must first determine whether the cleanest bounded source location is:

- an extension/refactor inside `candidate_publication_result_wire.rs`;
- a dedicated current-Mesh result adapter in `prw-remote-bridge`;
- an Agent-side composition wrapper over a bridge-owned pure current-Mesh encoder;
- another already-existing pure seam found by the fresh source audit.

No broad bridge/transport redesign is selected.

## 24. No dependency inversion

GL preserves layering:

- `prw-remote-bridge` may depend on existing transport/domain crates according to current dependency direction;
- Agent-local cleanup types must not be pulled into lower transport ownership solely for framing;
- opaque/generic disposition carriers may be used to preserve higher-owner state without lower-layer semantic inspection;
- no bridge -> Agent dependency is authorized.

## 25. Future source validation expectations

A later source materialization must prove at minimum:

1. current-Mesh Accepted uses exact GK committed replacement freshness;
2. cleanup success and cleanup failure both remain Accepted after definite commit;
3. pre/at-commit semantic error becomes generic Rejected;
4. GI Missing and Ambiguous both become generic peer-visible Rejected while remaining internally distinguishable before projection;
5. no internal error detail appears in the peer payload;
6. exact `CandidatePublicationMeshRequest::request_id()` is echoed unchanged;
7. request ID never becomes identity/authorization authority;
8. result frame type is `prw_remote_transport::ControlFrame`;
9. historical `prw_control_transport::ControlFrame` is not fabricated for current Mesh;
10. historical `CandidatePublicationControlFrame` is not fabricated;
11. existing historical result codec tests remain byte-stable;
12. current-Mesh accepted/rejected payload bytes match existing PRWP terminal semantics;
13. local frame-construction failure remains distinct from semantic/cleanup failures;
14. no semantic replay, cleanup retry, fallback frame, stream write, worker integration, listener activation, or dialing occurs.

## 26. GL authorized path set

GL is docs-only.

Exactly one path is authorized:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_GL_CANDIDATE_PUBLICATION_CURRENT_MESH_TERMINAL_RESULT_PROJECTION_FRAME_COMPOSITION_SEMANTICS_SELECTION_STAGING.md`

No Rust source, `Cargo.toml`, `Cargo.lock`, workflow, Android/Kotlin/Gradle, bridge source, transport source, Agent runtime source, GI custody-map source, worker, listener, bootstrap, readiness, persistence, deployment, configuration, or other contract path is authorized.

## 27. GL validation requirements

Canonical closure requires exact-final-head evidence for:

- exact GK merge base;
- ahead-only branch state;
- exactly one changed path;
- contract-only delta;
- no source/manifest/lock/workflow/runtime changes;
- locked dependency graph PASS when Rust workflow is triggered;
- rustfmt PASS when Rust workflow is triggered;
- Clippy PASS when Rust workflow is triggered;
- workspace tests PASS when Rust workflow is triggered;
- workspace build PASS when Rust workflow is triggered;
- Android recorded only if actually triggered;
- AD/AE and any path-filtered workflow recorded exactly as observed;
- immutable Drive raw byte/SHA equality.

Any correction creates a new exact final head and supersedes earlier validation evidence.

## 28. Immutable evidence procedure

After exact-final-head CI is terminal and acceptable:

1. freeze GL contract/source state;
2. record final commit/tree/compare/path/blob evidence;
3. create one immutable GL audit locally;
4. compute exact local bytes and SHA-256;
5. upload directly to canonical Drive folder `Private Remote Workspace` / `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
6. raw-fetch that exact Drive object;
7. recompute byte count and SHA-256 and require equality;
8. only then update the GL PR body to `Status: CLOSED` while keeping it draft/open/unmerged;
9. independently re-read PR and branch state.

No My Drive root upload is permitted.

## 29. Explicit GL non-goals

GL does not:

- source-materialize the selected projection/frame composition;
- remove `CandidatePublicationHandoffNotSelected`;
- invoke GK from current-Mesh ingress;
- consume current-Mesh transaction custody;
- write/send/finish a candidate response;
- define response I/O retry or peer-close policy;
- resume or integrate the repeated mixed-family loop;
- define candidate cancellation semantics;
- populate or synchronize the production-owner map;
- recover owners per command;
- activate traversal;
- bind/accept a listener;
- publish readiness;
- dial;
- deploy;
- restart/recover a process;
- merge;
- delete branches;
- change repository visibility.

## 30. Canonical selected law

**For one completed dormant GK current-Mesh candidate-publication semantic attempt, definite durable commit success projects to the existing PRWP Accepted semantics with exactly the committed replacement freshness regardless of exact post-commit requester-cleanup success or failure, while any pre/at-commit semantic or exact GI owner-lookup failure projects to generic Rejected with no internal error detail; cleanup disposition remains a separate local channel, exact correlation comes only from the retained `CandidatePublicationMeshRequest::request_id()`, the terminal frame is composed as a current `prw_remote_transport::ControlFrame` using existing PRWP terminal payload semantics and Response/Error kind pairing, historical PRWC/Command frame ownership is never fabricated, local frame-construction failure remains distinct and triggers no replay/fallback, and no stream I/O, candidate handoff activation, runtime integration, owner-map population, listener/readiness, traversal, dialing, deployment, restart/recovery, or merge is selected.**

## 31. Canonical closure target

Canonical closure:

`CLOSED_CANDIDATE_PUBLICATION_CURRENT_MESH_TERMINAL_RESULT_PROJECTION_FRAME_COMPOSITION_SEMANTICS_SELECTION`

Canonical gate:

`C03E_GL_CANDIDATE_PUBLICATION_CURRENT_MESH_TERMINAL_RESULT_PROJECTION_FRAME_COMPOSITION_SEMANTICS_SELECTED`

## 32. Successor rule

After canonical GL closure, perform a fresh exact-final-head source audit before naming or materializing the source successor.

The likely next checkpoint is a narrow source materialization of the pure current-Mesh candidate terminal-result projection/frame-composition law. It must not automatically include same-stream send I/O, higher-owner handoff activation, worker/cancellation integration, production-owner runtime population/synchronization, traversal, listener/readiness, dialing, deployment, restart/recovery, or merge.