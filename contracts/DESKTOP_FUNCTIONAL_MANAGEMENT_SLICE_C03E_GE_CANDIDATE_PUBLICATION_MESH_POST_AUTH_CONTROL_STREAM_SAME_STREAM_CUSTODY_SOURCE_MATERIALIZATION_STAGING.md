# Phase 152 C03e-GE — Candidate Publication Mesh Post-Auth Control-Stream Same-Stream Custody Source Materialization

Status: VALIDATING

Target gate:
`C03E_GE_CANDIDATE_PUBLICATION_MESH_POST_AUTH_CONTROL_STREAM_SAME_STREAM_CUSTODY_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Closed C03e-GD is the authoritative predecessor:

- branch: `phase-152-c03e-gd-candidate-publication-mesh-post-auth-control-stream-same-stream-custody-semantics-selection-staging`;
- head: `ace3b377f9589a23f9e0e8843c47acd155dfb434`;
- tree: `7dafb70a4ca2ef462f3982f393caef6e6dccb0cf`;
- closure: `CLOSED_CANDIDATE_PUBLICATION_MESH_POST_AUTH_CONTROL_STREAM_SAME_STREAM_CUSTODY_SEMANTICS_SELECTION`;
- gate: `C03E_GD_CANDIDATE_PUBLICATION_MESH_POST_AUTH_CONTROL_STREAM_SAME_STREAM_CUSTODY_SEMANTICS_SELECTED`;
- PR #306: `Status: CLOSED`, draft/open/unmerged.

C03e-GE begins exactly from that head and does not amend GD or any prior closed checkpoint.

## 2. Scope

C03e-GE source-materializes only the GD-selected current-Mesh candidate-publication family/custody extension inside the existing bridge-owned single-read ingress.

The materialized delta is intentionally bounded to:

1. `crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`;
2. this source-materialization contract.

No Cargo manifest, lockfile, module root, workflow, Kotlin/Gradle, Agent runtime, configuration, deployment, listener, readiness, reachability-owner, or unrelated source path is authorized.

## 3. Existing ingress remains the owner

The current bridge-owned function remains:

`receive_post_auth_control_stream_ingress(MeshControlStream)`

It still:

- consumes exactly one already-accepted `MeshControlStream` by value;
- performs exactly one bounded `receive_frame().await`;
- returns exactly one typed custody result;
- never accepts a second stream;
- never retries or performs a second read;
- never authenticates a session;
- never executes semantic policy/provider logic.

No second candidate-specific accept/read loop is introduced.

## 4. Materialized three-way family routing

The ingress preserves the exact GD routing order:

1. exact payload prefix `PRWZ` -> existing requester/rendezvous path;
2. otherwise exact payload prefix `PRWP` -> candidate-publication path;
3. every other bounded frame -> existing capability fallback.

The existing constants remain authoritative:

- `REQUESTER_RENDEZVOUS_TARGET_REQUEST_WIRE_MAGIC`;
- `CANDIDATE_PUBLICATION_WIRE_MAGIC`.

No new wire magic or protocol family identifier is created.

## 5. Exact-head compatibility correction discovered during source validation

The first GE candidate attempted to call historical:

`decode_candidate_publication_control_frame(...)`

directly with the current-Mesh ingress frame.

Exact-head Rust validation proved that this is not a valid source boundary:

- historical `candidate_publication_control_frame.rs` consumes `prw_control_transport::ControlFrame`;
- current Mesh ingress consumes `prw_remote_transport::ControlFrame`;
- those are distinct Rust types and are not interchangeable;
- historical `prw_control_transport::ControlMessageKind` has candidate `Command` semantics;
- current `prw_remote_transport::ControlMessageKind` has no `Command` variant and represents peer request operations with `Request`.

This is a transport-era compatibility distinction, not authority to reopen or mutate GD.

GE therefore preserves the GD semantic intent — exact PRWP family recognition, strict bounded structural decode, exact outer correlation preservation, and exact same-stream custody — through the current-Mesh-native representation described below.

GE explicitly rejects:

- casting one frame type into the other;
- fabricating an `AuthenticatedPrwcConnection`;
- wrapping `MeshControlStream` as historical `ControlTlsServerStream`;
- importing historical `Command` kind into the current Mesh enum;
- treating the two transport frame representations as identity-equivalent.

## 6. Current-Mesh candidate strict structural decode

Exact `PRWP` prefix recognition is family selection only.

After recognition, GE performs current-Mesh-native strict decoding with this order:

1. require outer `prw_remote_transport::ControlMessageKind::Request`;
2. decode the complete payload using existing pure `CandidatePublicationWireSubmission::decode(...)`;
3. preserve the exact outer `ControlFrame::request_id()` unchanged;
4. return one typed current-Mesh candidate request carrier.

The existing pure PRWP decoder remains authoritative for:

- PRWP magic;
- version;
- operation;
- reserved fields;
- transport identity construction;
- freshness token construction;
- candidate count/bounds;
- candidate IDs/path kinds/endpoints;
- truncation/trailing data.

No candidate semantic authority is inferred from prefix recognition or structural decode.

## 7. Current-Mesh candidate request carrier

C03e-GE materializes bridge-owned:

`CandidatePublicationMeshRequest`

It retains exactly:

1. the exact non-zero peer-originated outer PRWM `request_id`;
2. one strict decoded `CandidatePublicationWireSubmission`.

It exposes only narrow structural/correlation access:

- `request_id()`;
- `submission()`;
- consuming `into_submission()`.

This carrier is not the historical `CandidatePublicationControlFrame` and does not pretend to be one.

It is not publisher identity, requester authority, replay authority, freshness currentness, reachability authority, or durable commit authority.

## 8. Candidate same-stream custody type

C03e-GE materializes bridge-owned:

`PostAuthCandidatePublicationTransaction`

It retains exactly:

1. one strict `CandidatePublicationMeshRequest`;
2. the exact same already-accepted `MeshControlStream` by value.

It exposes only:

- `request()` — immutable borrow of the strict current-Mesh candidate request;
- `into_parts()` — consuming transfer of exact request plus exact stream.

It exposes no raw stream borrow/getter, clone, duplicate ownership, generic escape hatch, send API, semantic execution method, owner recovery, retry, or loop.

## 9. Candidate ingress result

`PostAuthControlStreamIngress` has a third typed outcome:

`CandidatePublication(PostAuthCandidatePublicationTransaction)`

The requester and capability variants remain present and preserve their existing meanings.

No generic untyped candidate result is introduced.

## 10. Candidate decode failure remains distinct and terminal to this one read

GE materializes:

`CandidatePublicationMeshRequestError`

with at least:

- `InvalidOuterKind`;
- `Wire(CandidatePublicationWireError)`.

`PostAuthControlStreamIngressError::CandidatePublicationWire(...)` preserves that current-Mesh candidate decoding failure as its source.

Once exact `PRWP` selects candidate publication and strict current-Mesh decoding fails:

- the frame is not reinterpreted as capability;
- the frame is not reinterpreted as requester/rendezvous;
- no second frame read occurs;
- no response is written;
- no retry/resynchronization occurs;
- no semantic execution occurs.

The one-shot stream value is consumed by the failed ingress call exactly as selected by GD.

## 11. Requester/rendezvous behavior is preserved

Exact `PRWZ` remains first in routing order.

Its behavior remains unchanged:

- existing strict `decode_requester_rendezvous_target_request_frame(...)` is called;
- successful strict decode returns existing `PostAuthRequesterRendezvousTransaction`;
- exact same already-accepted stream remains in that transaction;
- requester DR acknowledgement send surface remains unchanged;
- requester response/error/custody classifications remain unchanged.

Candidate-publication materialization cannot intercept or reinterpret exact `PRWZ` traffic.

## 12. Capability fallback remains legacy-preserving

Every bounded frame whose payload begins neither exact `PRWZ` nor exact `PRWP` remains on the existing capability fallback path.

This includes:

- valid `PRWC` capability traffic;
- malformed capability traffic;
- short payloads;
- unknown payload magic;
- other non-reserved prefixes.

C03e-GE introduces no `UnknownFamily` error ahead of the existing capability bridge.

Existing `PostAuthCapabilityTransaction` remains unchanged in responsibility and retains the exact already-read frame plus exact same stream.

## 13. Identity authority remains separated

`CandidatePublicationMeshRequest` and `PostAuthCandidatePublicationTransaction` carry structural/correlation/custody state only.

Publisher logical identity must still come later from the current authenticated logical session owned by the higher Agent runtime owner.

It must not be derived from:

- PRWP payload fields;
- candidate endpoints;
- outer request ID;
- lower transport bytes;
- requester/rendezvous grant;
- cleanup identity.

No current authenticated-session execution adapter is selected or materialized here.

## 14. Correlation remains non-authorizing

The exact peer-originated outer request ID is preserved in `CandidatePublicationMeshRequest`.

C03e-GE does not:

- allocate a replacement request ID;
- insert the inbound ID into locally-originated request-ID custody;
- treat it as publisher/requester identity;
- treat it as freshness/replay authority;
- treat it as rendezvous or durable-owner authority.

## 15. Historical candidate control frame remains byte-stable

C03e-GE does not modify:

- `candidate_publication_control_frame.rs`;
- historical pre-Mesh `AuthenticatedPrwcConnection` receive/write APIs;
- provider-neutral C03e-CQ execution helper;
- historical candidate result framing.

The historical `CandidatePublicationControlFrame` remains valid only at its own transport boundary.

A later separately gated current-Mesh execution adapter must consume current-Mesh request fields without violating publisher-session authority or manufacturing historical transport ownership.

## 16. FY/GA/GC remain dormant

C03e-GE does not invoke:

- `SharedRequesterRendezvousAuthority::execute_authenticated_candidate_publication_with_post_commit_cleanup(...)`;
- `project_candidate_publication_terminal_result(...)`;
- `compose_candidate_publication_terminal_result_frame(...)`.

The candidate transaction only makes the decoded current-Mesh request and exact stream available for later separately gated ownership composition.

No execution error, cleanup result, or result-wire error is flattened or reclassified by GE.

## 17. Production reachability-owner custody remains separately gated

C03e-GE does not construct, recover, store, or expose:

`ProductionReachabilityOwner<S,T>`

It performs no reachability commit and no durable store operation.

Existing live-owner bootstrap authority is not treated as interchangeable with the production candidate-publication owner.

A fresh post-GE audit remains required before any such owner custody/recovery materialization.

## 18. No current-Mesh candidate response write

The candidate transaction intentionally has no send method.

C03e-GE does not materialize:

- terminal result frame send;
- `MeshControlStream::send_frame(...)` for candidate publication;
- send-direction finish policy;
- result-write error classification;
- fallback Rejected;
- retry/re-encode;
- peer-close policy;
- repeated-ingress continuation after response.

GC remains pure historical-frame composition and is not invoked here.

## 19. No runtime activation

C03e-GE does not modify any Agent caller to consume the new candidate variant.

Therefore this source exists as a validated bridge-owned ingress/custody capability only.

It does not activate candidate-publication runtime traffic.

No command loop, listener, readiness, bootstrap, network cutover, traversal, dialing, deployment, restart/recovery, or merge is authorized.

## 20. Focused tests

Source includes focused structural tests proving at least:

- the single stream-consuming ingress surface remains the only receive operation;
- requester custody transfer signature remains present;
- candidate custody transfer consumes the candidate transaction and returns exact current-Mesh request + Mesh stream ownership;
- classifier partitions exact `PRWZ`, exact `PRWP`, and capability fallback prefixes;
- requester family recognition still requires strict requester decoding;
- candidate family recognition requires current-Mesh outer `Request`;
- exact PRWP prefix alone is insufficient for successful payload decode;
- valid current-Mesh candidate structural decode preserves exact request ID and decoded submission.

No live network test, provider test, reachability commit, or runtime activation test is introduced.

## 21. Exact dependency boundary

C03e-GE uses only types already present in `prw-remote-bridge`:

- `CandidatePublicationWireSubmission`;
- `CandidatePublicationWireError`;
- `CANDIDATE_PUBLICATION_WIRE_MAGIC`;
- existing `prw_remote_transport::{ControlFrame, ControlMessageKind}`;
- existing `MeshControlStream`.

No dependency or lockfile change is required or authorized.

## 22. Validation history before final candidate

Initial GE candidate:

`074e24539b901ac59a9faa60e4d2727dcaa6f6aa`

Rust #1344 / run `33295463675` / job `99214248173`:

- locked dependency graph: PASS;
- rustfmt: FAIL;
- Clippy/tests/build: not run after formatter boundary.

The exact formatter correction was one mechanical line-wrap hunk in the focused candidate custody signature test.

Formatter-normalized candidate:

`1bb7cbc8c6669c056adbda7082cbdf111c80a738`

Rust #1345 / run `33295528406` / job `99214417160`:

- locked dependency graph: PASS;
- rustfmt: PASS;
- Clippy/compile: FAIL at the historical-vs-Mesh frame-type incompatibility described in section 5;
- tests/build: skipped after failure.

Those failures are superseded evidence and are not final GE verdicts.

## 23. Final validation gate

Canonical closure requires exact-final-head validation after the current-Mesh compatibility correction and any later strictly mechanical formatter/lint corrections.

Required Rust verdict:

- checkout success;
- native prerequisites success;
- exact toolchain success;
- locked dependency graph success;
- rustfmt success;
- Clippy success with warnings denied;
- workspace tests success;
- workspace build success.

Because GE changes Rust source and Android validation triggers, exact-final-head Android validation must also reach terminal success for:

- native adapter;
- Android application.

Path-filtered workflows that skip must be recorded as `skipped`, never PASS.

## 24. Durable closure evidence

C03e-GE closes only after:

1. exact GD predecessor remains unchanged;
2. GD...GE compare is ahead-only with exact GD merge base;
3. only the authorized source + contract paths are present;
4. exact-final-head CI is terminal and non-failing;
5. one immutable GE audit is uploaded to the canonical Private Remote Workspace Drive folder;
6. raw Drive byte/hash readback passes;
7. PR body moves from `Status: VALIDATING` to `Status: CLOSED` only after Drive verification;
8. PR remains draft/open/unmerged.

## 25. Intended canonical closure

Closure:

`CLOSED_CANDIDATE_PUBLICATION_MESH_POST_AUTH_CONTROL_STREAM_SAME_STREAM_CUSTODY_SOURCE_MATERIALIZATION`

Gate:

`C03E_GE_CANDIDATE_PUBLICATION_MESH_POST_AUTH_CONTROL_STREAM_SAME_STREAM_CUSTODY_SOURCE_MATERIALIZED`

## 26. Successor rule

GE does not pre-authorize a specific successor checkpoint.

After exact-head validation and durable GE closure, a fresh prerequisite audit must choose the next narrow boundary among at least:

- current authenticated-session + current-Mesh candidate request -> dormant FY semantic execution adaptation;
- production `ProductionReachabilityOwner<S,T>` custody/recovery;
- current-Mesh candidate terminal-result composition/write adaptation;
- any still-earlier prerequisite exposed by exact current source topology.

Any successor must preserve the discovered transport-era distinction: current-Mesh request correlation/submission must not be converted into historical transport custody merely to reuse older APIs.

No successor may jump directly to runtime activation, listener/readiness cutover, traversal/dialing, deployment, restart/recovery, or merge.
