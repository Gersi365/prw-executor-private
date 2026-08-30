# Phase 152 C03e-GM — Candidate Publication Current-Mesh Terminal Result Projection / Frame Composition Source Materialization

Status: VALIDATING

Target gate:
`C03E_GM_CANDIDATE_PUBLICATION_CURRENT_MESH_TERMINAL_RESULT_PROJECTION_FRAME_COMPOSITION_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Canonically CLOSED C03e-GL is the authoritative predecessor:

- branch: `phase-152-c03e-gl-candidate-publication-current-mesh-terminal-result-projection-frame-composition-semantics-selection-staging`;
- final head: `562d5db7a34f4e08f060a284e2bc44769c39d7ab`;
- final tree: `a0110be7a4badbf7bc5ce1ebbf6b8c1470d3ffd2`;
- PR #314: `Status: CLOSED`, draft/open/unmerged;
- canonical immutable GL Drive object: `1jz8kUUw5ygZwYWmysouMReaXj1B9T03h`;
- immutable GL bytes: `11938`;
- immutable GL SHA-256: `0ea2c56cd36ee1c2946d510bf6f250dbba0560dc73d695fa5aa8b7cb083caf63`;
- closure: `CLOSED_CANDIDATE_PUBLICATION_CURRENT_MESH_TERMINAL_RESULT_PROJECTION_FRAME_COMPOSITION_SEMANTICS_SELECTION`;
- gate: `C03E_GL_CANDIDATE_PUBLICATION_CURRENT_MESH_TERMINAL_RESULT_PROJECTION_FRAME_COMPOSITION_SEMANTICS_SELECTED`.

GL remains frozen. GM starts exactly from the GL final commit and does not amend a closed predecessor.

## 2. Fresh exact-GL source audit

The post-GL source audit re-read:

- `crates/prw-remote-bridge/Cargo.toml`;
- `crates/prw-remote-bridge/src/root.rs`;
- `crates/prw-remote-bridge/src/candidate_publication_result_wire.rs`;
- current `prw_remote_transport::ControlFrame` / `ControlMessageKind` / `RemoteTransportError`;
- GK Agent result decomposition and cleanup custody source.

The audit established:

1. `prw-remote-bridge` already depends directly on `prw-remote-transport`; no manifest or lockfile change is required.
2. Existing historical candidate result codec owns `prw_control_transport::ControlFrame`, which must remain unchanged.
3. Existing public PRWP terminal constants and `CandidatePublicationResultMessage` are sufficient to reproduce the same bounded PRWP terminal payload bytes under current-Mesh frame ownership.
4. Current `prw_remote_transport::ControlFrame::new(...)` already owns non-zero request-correlation and bounded-payload validation.
5. Current Mesh already defines `Response` and `Error` outer kinds.
6. `ReachabilityCommitOutcome::replacement_freshness()` is the existing public accessor for the verifier-issued committed freshness value.
7. No new bridge -> Agent dependency is required to preserve post-commit cleanup state: a generic opaque disposition carrier is sufficient.
8. A generic semantic error parameter can remain distinguishable to the higher caller before pure projection while every consumed pre/at-commit error maps to generic Rejected without serialized detail.
9. Same-stream `MeshControlStream` ownership and send/finish behavior are not required for pure composition and remain separately gated.

## 3. Exact GM source ownership

GM materializes a dedicated bridge-owned module:

`crates/prw-remote-bridge/src/candidate_publication_mesh_result_wire.rs`

and exposes it through one module declaration in:

`crates/prw-remote-bridge/src/root.rs`

No existing historical candidate terminal codec implementation is rewritten.

## 4. Current-Mesh frame error layer

GM introduces:

`CandidatePublicationMeshResultFrameError`

with a distinct local frame-construction layer over current `RemoteTransportError`.

This error is not:

- `CandidatePublicationExecutionError`;
- `CurrentMeshCandidatePublicationExecutionError`;
- `ProductionReachabilityOwnerCustodyLookupError`;
- `RequesterRendezvousLifecycleError`;
- a durable rollback signal;
- retry authorization;
- fallback-frame authorization.

A frame-construction failure remains local and fail-closed.

## 5. Opaque disposition carrier

GM introduces:

`CandidatePublicationMeshResultFrameComposition<D>`

which retains exactly:

- one `Result<prw_remote_transport::ControlFrame, CandidatePublicationMeshResultFrameError>`; and
- one opaque higher-owner disposition `D`.

The bridge does not inspect, flatten or serialize `D`.

This mirrors the previously proven historical composition pattern without reusing the historical frame type.

## 6. PRWP terminal payload byte stability

GM keeps the existing PRWP terminal payload law byte-stable:

- `CANDIDATE_PUBLICATION_WIRE_MAGIC`;
- `CANDIDATE_PUBLICATION_WIRE_MAJOR`;
- `CANDIDATE_PUBLICATION_WIRE_MINOR`;
- accepted operation `OP_PUBLISHER_CANDIDATE_SET_ACCEPTED`;
- rejected operation `OP_PUBLISHER_CANDIDATE_SET_REJECTED`;
- zero reserved field;
- accepted replacement-freshness bytes;
- rejected header-only payload;
- existing accepted/rejected exact lengths.

No new PRWP version, operation, metadata, error detail or endpoint material is introduced.

## 7. Exact current-Mesh outer frame law

GM materializes:

`encode_candidate_publication_mesh_result_frame(request_id, message)`

with exact outer mapping:

- `CandidatePublicationResultMessage::Accepted` -> `prw_remote_transport::ControlMessageKind::Response`;
- `CandidatePublicationResultMessage::Rejected` -> `prw_remote_transport::ControlMessageKind::Error`.

The frame type is exactly:

`prw_remote_transport::ControlFrame`

and never historical:

`prw_control_transport::ControlFrame`.

## 8. Request correlation remains correlation only

`request_id` is caller-supplied and echoed unchanged through current-Mesh `ControlFrame::new(...)`.

GM allocates/registers no request ID.

The request ID is not:

- publisher identity;
- requester identity;
- transport identity;
- owner key;
- freshness authority;
- durable authority;
- cleanup authority;
- candidate endpoint authority.

A zero request ID remains rejected by existing current-Mesh frame validation.

## 9. Pure semantic projection law

GM materializes:

`compose_candidate_publication_current_mesh_terminal_result(request_id, result)`

where the completed result is generic over:

- success: `(ReachabilityCommitOutcome, D)`;
- failure: `E`.

Exact projection:

### 9.1 Definite durable success

`Ok((committed, disposition))`

projects to Accepted using exactly:

`committed.replacement_freshness()`

and preserves:

`Some(disposition)`

beside the frame-construction result.

### 9.2 Any pre/at-commit error

`Err(error)`

projects to generic Rejected and carries:

`None`

for post-commit disposition.

The higher caller can distinguish its concrete error before passing it into pure projection. GM serializes no internal error detail.

## 10. Cleanup success/failure law

For the future Agent caller, `D` may be the exact post-commit requester cleanup disposition.

Because GM treats `D` as opaque:

- cleanup success cannot alter Accepted bytes;
- cleanup failure cannot alter Accepted bytes;
- cleanup failure cannot become Rejected;
- cleanup failure cannot trigger durable rollback;
- cleanup failure cannot trigger semantic replay;
- cleanup failure cannot trigger cleanup replay;
- cleanup failure cannot trigger fallback frame composition.

Only the existence of definite durable success controls whether `Some(D)` exists.

## 11. Owner-lookup error privacy

A future caller may pass either Agent-local semantic or exact GI owner-lookup errors as the generic `E` channel.

Before projection those types remain locally distinguishable.

After pure projection all consumed errors become the same generic peer-visible Rejected payload.

GM adds no:

- Missing/ambiguous error detail serialization;
- owner fallback;
- alternate transport selection;
- DeviceId-only lookup;
- single-owner fallback;
- retry.

## 12. Historical codec remains frozen

GM does not modify:

- `candidate_publication_result_wire.rs`;
- `CandidatePublicationResultFrameComposition<D>`;
- `encode_candidate_publication_result_frame(...)`;
- `decode_candidate_publication_result_frame(...)`;
- `project_candidate_publication_execution_result(...)`;
- `encode_candidate_publication_execution_result_frame(...)`.

Tests compare current-Mesh Accepted/Rejected payload bytes against the existing historical encoder to prove semantic byte-equivalence while retaining distinct frame ownership.

## 13. Focused validation expectations

GM source tests must prove at minimum:

1. current-Mesh Accepted uses outer `Response`;
2. current-Mesh Rejected uses outer `Error`;
3. exact non-zero caller request correlation is echoed unchanged;
4. Accepted current-Mesh PRWP payload bytes equal existing historical Accepted payload bytes for the same typed message;
5. Rejected current-Mesh PRWP payload bytes equal existing historical Rejected payload bytes;
6. zero request correlation fails through the distinct current-Mesh frame error layer;
7. no fallback frame is produced after local frame-construction failure;
8. generic pre/at-commit failure produces Rejected with no success disposition.

Definite-success projection itself uses the existing public `ReachabilityCommitOutcome::replacement_freshness()` accessor and does not widen that type merely to create synthetic test instances.

## 14. No stream I/O

GM does not call:

- `MeshControlStream::send_frame(...)`;
- `MeshControlStream::receive_frame(...)`;
- `accept_control_stream()`;
- `open_control_stream()`;
- stream finish directly.

No stream is consumed, cloned, replaced, reaccepted, reread or retried by GM.

## 15. No current-Mesh candidate handoff activation

`AuthenticatedRemoteSessionPostAuthIngressTransactionError::CandidatePublicationHandoffNotSelected`

remains unchanged.

GM does not consume `PostAuthCandidatePublicationTransaction`.

GM does not invoke GK semantic execution from ingress.

## 16. No production-owner runtime ownership decision

GM does not select or materialize:

- startup owner-map population;
- recovery scheduling;
- transport-rotation insertion/removal;
- map synchronization;
- mutex/actor/mailbox policy;
- concurrent candidate execution policy;
- owner background tasks.

## 17. No cancellation semantics

GM selects no cancellation behavior around:

- GK semantic execution;
- durable commit;
- requester cleanup;
- current-Mesh frame composition;
- later same-stream send/finish.

## 18. Dynamic-network identity invariant remains unchanged

GM assigns no identity or authorization meaning to IP addresses, socket endpoints, candidate endpoints or request IDs.

The canonical identity chain remains:

`logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

Candidate endpoints remain transient reachability data.

## 19. Exact authorized path ceiling

GM authorizes exactly three paths:

1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_GM_CANDIDATE_PUBLICATION_CURRENT_MESH_TERMINAL_RESULT_PROJECTION_FRAME_COMPOSITION_SOURCE_MATERIALIZATION_STAGING.md`;
2. `crates/prw-remote-bridge/src/candidate_publication_mesh_result_wire.rs`;
3. `crates/prw-remote-bridge/src/root.rs`.

No other path is authorized.

In particular no:

- `Cargo.toml`;
- `Cargo.lock`;
- workflow;
- Android/Kotlin/Gradle;
- Agent source;
- ingress;
- current-Mesh transport implementation;
- listener;
- bootstrap;
- readiness;
- persistence;
- deployment;
- configuration;
- unrelated contract path

may change.

## 20. Validation protocol

Before canonical GM closure require exact-final-head evidence for:

- exact predecessor / merge base;
- ahead-only compare;
- exactly three authorized changed paths;
- exact blob identities;
- locked dependency graph;
- rustfmt;
- Clippy with warnings denied;
- workspace tests;
- workspace build;
- Android only if automatically triggered for the exact final head;
- AD/AE and other path-filtered workflows recorded exactly as observed;
- immutable Drive raw byte/SHA equality.

Any correction creates a new exact final head and supersedes earlier validation evidence.

No manual validation workflow dispatch is authorized.

## 21. Immutable evidence procedure

After exact-final-head CI is terminal and acceptable:

1. freeze final GM source/contract state;
2. record exact final commit/tree/compare/path/blob evidence;
3. create one immutable GM audit locally;
4. compute exact local bytes and SHA-256;
5. upload directly to canonical Drive folder `Private Remote Workspace` / `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
6. raw-fetch that exact Drive object;
7. recompute byte count and SHA-256 and require equality;
8. only then update the GM PR body to `Status: CLOSED` while keeping it draft/open/unmerged;
9. independently re-read PR and branch state.

No My Drive root upload is authorized.

## 22. Explicit non-activation boundary

GM does not authorize:

- same-stream response I/O;
- higher-owner candidate handoff activation;
- worker-loop integration;
- cancellation semantics;
- production-owner runtime population/synchronization;
- traversal activation;
- listener/readiness activation;
- dialing;
- deployment;
- process restart/recovery;
- merge;
- branch deletion;
- repository visibility change.

## 23. Successor rule

After canonical GM closure, perform a fresh exact-final-head source audit before selecting any successor.

Likely remaining boundaries include exact current-Mesh transaction/request-correlation adaptation plus same-stream response custody and, separately, production-owner runtime custody/population required before candidate handoff activation. Their ordering and source scope are not pre-authorized by GM.
