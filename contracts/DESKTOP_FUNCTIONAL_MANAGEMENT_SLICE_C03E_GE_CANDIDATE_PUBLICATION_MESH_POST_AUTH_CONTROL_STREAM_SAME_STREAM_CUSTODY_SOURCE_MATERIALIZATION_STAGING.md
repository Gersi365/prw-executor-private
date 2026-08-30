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

The ingress now preserves the exact GD ordering:

1. exact payload prefix `PRWZ` -> existing requester/rendezvous path;
2. otherwise exact payload prefix `PRWP` -> candidate-publication path;
3. every other bounded frame -> existing capability fallback.

The existing constants remain authoritative:

- `REQUESTER_RENDEZVOUS_TARGET_REQUEST_WIRE_MAGIC`;
- `CANDIDATE_PUBLICATION_WIRE_MAGIC`.

No new wire magic or protocol family identifier is created.

## 5. Requester/rendezvous behavior is preserved

Exact `PRWZ` remains first in routing order.

Its behavior remains unchanged:

- existing strict `decode_requester_rendezvous_target_request_frame(...)` is called;
- successful strict decode returns existing `PostAuthRequesterRendezvousTransaction`;
- exact same already-accepted stream remains in that transaction;
- requester DR acknowledgement send surface remains unchanged;
- requester response/error/custody classifications remain unchanged.

Candidate-publication materialization must not intercept or reinterpret exact `PRWZ` traffic.

## 6. Candidate-publication strict structural decode

Exact `PRWP` prefix recognition is family selection only.

After recognition, source calls the existing:

`decode_candidate_publication_control_frame(&frame)`

That existing decoder remains authoritative for:

- outer `ControlMessageKind::Command` requirement;
- strict candidate-publication PRWP payload decoding;
- exact peer-originated outer request ID preservation;
- typed `CandidatePublicationControlFrame` production.

No candidate semantic authority is inferred from prefix recognition.

## 7. Candidate same-stream custody type

C03e-GE materializes bridge-owned:

`PostAuthCandidatePublicationTransaction`

It retains exactly:

1. one strict decoded `CandidatePublicationControlFrame`;
2. the exact same already-accepted `MeshControlStream` by value.

It exposes only narrow custody surfaces:

- `command()` — immutable borrow of the strict decoded command;
- `into_parts()` — consuming transfer of the exact command and exact stream.

It exposes no raw stream borrow/getter, clone, duplicate ownership, generic escape hatch, send API, semantic execution method, owner recovery, retry, or loop.

## 8. Candidate ingress result

`PostAuthControlStreamIngress` now has a third typed outcome:

`CandidatePublication(PostAuthCandidatePublicationTransaction)`

The requester and capability variants remain present and preserve their existing meanings.

No generic untyped candidate result is introduced.

## 9. Candidate decode failure remains distinct and terminal to this one read

`PostAuthControlStreamIngressError` now includes:

`CandidatePublicationWire(CandidatePublicationControlFrameError)`

The error preserves the existing strict candidate-control-frame error as its source.

Once exact `PRWP` selects candidate publication and strict decoding fails:

- the frame is not reinterpreted as capability;
- the frame is not reinterpreted as requester/rendezvous;
- no second frame read occurs;
- no response is written;
- no retry/resynchronization occurs;
- no semantic execution occurs.

The one-shot stream value is consumed by the failed ingress call exactly as selected by GD.

## 10. Capability fallback remains legacy-preserving

Every bounded frame whose payload begins neither exact `PRWZ` nor exact `PRWP` remains on the existing capability fallback path.

This includes:

- valid `PRWC` capability traffic;
- malformed capability traffic;
- short payloads;
- unknown payload magic;
- other non-reserved prefixes.

C03e-GE does not introduce an `UnknownFamily` error ahead of the existing capability bridge.

Existing `PostAuthCapabilityTransaction` remains unchanged in responsibility and retains the exact already-read frame plus exact same stream.

## 11. Identity authority remains separated

`PostAuthCandidatePublicationTransaction` carries structural/correlation state only.

It is not publisher identity authority.

Publisher logical identity must still come later from the current authenticated logical session owned by the higher Agent runtime owner.

It must not be derived from:

- PRWP payload fields;
- candidate endpoints;
- outer request ID;
- lower transport bytes;
- requester/rendezvous grant;
- cleanup identity.

No current authenticated-session execution adapter is selected or materialized here.

## 12. Correlation remains non-authorizing

The exact peer-originated outer request ID remains inside `CandidatePublicationControlFrame`.

C03e-GE does not:

- allocate a replacement request ID;
- insert the inbound ID into locally-originated request-ID custody;
- treat it as publisher/requester identity;
- treat it as freshness/replay authority;
- treat it as rendezvous or durable-owner authority.

## 13. FY/GA/GC remain dormant

C03e-GE does not invoke:

- `SharedRequesterRendezvousAuthority::execute_authenticated_candidate_publication_with_post_commit_cleanup(...)`;
- `project_candidate_publication_terminal_result(...)`;
- `compose_candidate_publication_terminal_result_frame(...)`.

The candidate transaction only makes the decoded command and exact stream available for later separately gated ownership composition.

No execution error, cleanup result, or result-wire error is flattened or reclassified by GE.

## 14. Production reachability-owner custody remains separately gated

C03e-GE does not construct, recover, store, or expose:

`ProductionReachabilityOwner<S,T>`

It performs no reachability commit and no durable store operation.

Existing live-owner bootstrap authority is not treated as interchangeable with the production candidate-publication owner.

A fresh post-GE audit remains required before any such owner custody/recovery materialization.

## 15. No current-Mesh candidate response write

The new candidate transaction intentionally has no send method.

C03e-GE does not materialize:

- terminal result frame send;
- `MeshControlStream::send_frame(...)` for candidate publication;
- send-direction finish policy;
- result-write error classification;
- fallback Rejected;
- retry/re-encode;
- peer-close policy;
- repeated-ingress continuation after response.

GC remains pure frame composition and is not invoked here.

## 16. No runtime activation

C03e-GE does not modify any Agent caller to consume the new candidate variant.

Therefore this source exists as a validated bridge-owned ingress/custody capability only.

It does not activate candidate-publication runtime traffic.

No command loop, listener, readiness, bootstrap, network cutover, traversal, dialing, deployment, restart/recovery, or merge is authorized.

## 17. Focused tests

Source includes focused structural tests proving at least:

- the single stream-consuming ingress surface remains the only receive operation;
- requester custody transfer signature remains present;
- candidate custody transfer signature consumes the candidate transaction and returns exact command + Mesh stream ownership;
- classifier partitions exact `PRWZ`, exact `PRWP`, and capability fallback prefixes;
- requester family recognition still requires strict requester decoding;
- candidate family recognition still requires strict candidate `Command` decoding.

No live network test, provider test, reachability commit, or runtime activation test is introduced.

## 18. Exact dependency boundary

C03e-GE uses only types already present in `prw-remote-bridge`:

- `CandidatePublicationControlFrame`;
- `CandidatePublicationControlFrameError`;
- `decode_candidate_publication_control_frame(...)`;
- `CANDIDATE_PUBLICATION_WIRE_MAGIC`;
- existing `MeshControlStream` and `ControlFrame` dependencies.

No dependency or lockfile change is required or authorized.

## 19. Validation gate

Canonical closure requires exact-final-head validation after all formatter/lint corrections, if any.

Required Rust verdict:

- checkout success;
- native prerequisites success;
- exact toolchain success;
- locked dependency graph success;
- rustfmt success;
- Clippy success with warnings denied;
- workspace tests success;
- workspace build success.

If Android validation automatically triggers because of the Rust source delta, both native adapter and Android application must reach terminal success on the exact final GE head.

Path-filtered workflows that skip must be recorded as `skipped`, never PASS.

## 20. Durable closure evidence

C03e-GE closes only after:

1. exact GD predecessor remains unchanged;
2. GD...GE compare is ahead-only with exact GD merge base;
3. only the authorized source + contract paths are present, except narrowly mechanical formatter/lint corrections within the same source path;
4. exact-final-head CI is terminal and non-failing;
5. one immutable GE audit is uploaded to the canonical Private Remote Workspace Drive folder;
6. raw Drive byte/hash readback passes;
7. PR body moves from `Status: VALIDATING` to `Status: CLOSED` only after Drive verification;
8. PR remains draft/open/unmerged.

## 21. Intended canonical closure

Closure:

`CLOSED_CANDIDATE_PUBLICATION_MESH_POST_AUTH_CONTROL_STREAM_SAME_STREAM_CUSTODY_SOURCE_MATERIALIZATION`

Gate:

`C03E_GE_CANDIDATE_PUBLICATION_MESH_POST_AUTH_CONTROL_STREAM_SAME_STREAM_CUSTODY_SOURCE_MATERIALIZED`

## 22. Successor rule

GE does not pre-authorize a specific successor checkpoint.

After exact-head validation and durable GE closure, a fresh prerequisite audit must choose the next narrow boundary among at least:

- current authenticated-session -> dormant FY/GA/GC execution adaptation;
- production `ProductionReachabilityOwner<S,T>` custody/recovery;
- current-Mesh candidate terminal-result write custody;
- any still-earlier prerequisite exposed by exact current source topology.

No successor may jump directly to runtime activation, listener/readiness cutover, traversal/dialing, deployment, restart/recovery, or merge.
