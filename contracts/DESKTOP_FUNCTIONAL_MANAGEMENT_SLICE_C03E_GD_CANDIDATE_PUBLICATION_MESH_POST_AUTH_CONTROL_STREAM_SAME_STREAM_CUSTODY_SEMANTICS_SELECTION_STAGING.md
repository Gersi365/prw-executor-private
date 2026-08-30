# Phase 152 C03e-GD — Candidate Publication Mesh Post-Auth Control-Stream Same-Stream Custody Semantics Selection

Status: SELECTED_DOCS_ONLY

Target gate:
`C03E_GD_CANDIDATE_PUBLICATION_MESH_POST_AUTH_CONTROL_STREAM_SAME_STREAM_CUSTODY_SEMANTICS_SELECTED`

## 1. Purpose

C03e-GD selects only the current Mesh post-authenticated single-read family/custody extension required to make one strict candidate-publication `PRWP` Command available as an isolated same-stream custody handoff.

This checkpoint does not materialize Rust source and does not execute candidate publication, compose or write a terminal result frame, construct/recover a production reachability owner, activate a command loop, or activate runtime/network behavior.

## 2. Exact predecessor

Closed C03e-GC is the authoritative predecessor:

- branch: `phase-152-c03e-gc-candidate-publication-post-commit-cleanup-terminal-result-frame-composition-source-materialization-staging`
- head: `82c643dbbba6e32d38bde0b86329d99bccda6973`
- tree: `e7fb3e4f43b35edfcf5e73d9d1aad3ab8289807f`
- closure: `CLOSED_CANDIDATE_PUBLICATION_POST_COMMIT_CLEANUP_TERMINAL_RESULT_FRAME_COMPOSITION_SOURCE_MATERIALIZATION`
- gate: `C03E_GC_CANDIDATE_PUBLICATION_POST_COMMIT_CLEANUP_TERMINAL_RESULT_FRAME_COMPOSITION_SOURCE_MATERIALIZED`
- PR #305: `Status: CLOSED`, draft/open/unmerged.

GC remains frozen. C03e-GD does not amend GC source or any earlier checkpoint.

## 3. Fresh post-GC prerequisite audit

The exact GC head already contains two distinct post-auth candidate-publication eras:

1. historical pre-Mesh `AuthenticatedPrwcConnection` / `ControlTlsServerStream` seams from C03e-CN through C03e-CX; and
2. the newer Mesh `AuthenticatedRemotePeerConnection` / `MeshControlStream` requester-aware single-read ingress lineage from C03e-ES onward.

The historical pre-Mesh lineage already provides:

- strict one-frame candidate-publication receive on `AuthenticatedPrwcConnection`;
- provider-neutral candidate-publication execution composition;
- generic Accepted/Rejected result framing;
- a pre-Mesh result write on `ControlTlsServerStream`.

Those seams do not own or transfer the current Mesh `MeshControlStream` used by the requester-aware Agent ingress lifecycle.

The current Mesh ingress at exact GC head instead recognizes only exact `PRWZ` requester/rendezvous payload prefix. Every non-`PRWZ` bounded frame is preserved on the legacy capability fallback path. Therefore a `PRWP` candidate-publication Command currently has no current-Mesh typed ingress/custody outcome.

Separately, FY/GA/GC materialize dormant semantic execution, post-commit cleanup projection, and terminal frame composition, but no current-Mesh caller or same-stream response custody.

The first non-duplicative prerequisite is therefore current-Mesh candidate-publication family selection plus exact same-stream custody. Production reachability-owner custody/recovery and actual execution remain separate later gates.

## 4. Historical pre-Mesh seams are not the selected current owner

C03e-GD does not reuse `AuthenticatedPrwcConnection` as the current Mesh stream owner.

The C03e-CN/C03e-CO receive seam and C03e-CW/C03e-CX write seam are retained historical/provider-neutral source and remain byte-stable. They use `ControlTlsServerStream`, while the current requester-aware topology owns `MeshControlStream` through bridge-owned post-auth ingress transactions.

No adapter may fabricate an `AuthenticatedPrwcConnection` around a `MeshControlStream`, expose raw stream internals, or substitute one transport owner type for the other.

## 5. Exact candidate-publication family marker

Candidate publication uses the existing inner wire magic:

`CANDIDATE_PUBLICATION_WIRE_MAGIC = *b"PRWP"`

The outer bounded control frame must still be strictly validated by the existing:

`decode_candidate_publication_control_frame(...)`

which requires the existing `Command` outer kind and strict PRWP payload semantics.

C03e-GD does not rename PRWP as PRWC. The outer control-frame transport and the inner candidate-publication PRWP protocol remain distinct concepts.

## 6. Selected three-way Mesh family routing rule

A future source materialization shall extend the existing bridge-owned single-read Mesh ingress to exactly three outcomes using this compatibility order:

1. exact first-four payload bytes `PRWZ` → existing requester/rendezvous family;
2. otherwise exact first-four payload bytes `PRWP` → candidate-publication family;
3. every other bounded frame → existing capability fallback family.

The order is explicit and deterministic.

`PRWZ` behavior remains unchanged.

Non-`PRWZ`, non-`PRWP` capability fallback remains unchanged.

A frame whose payload begins exact `PRWP` is not allowed to fall back to capability after candidate-family recognition.

## 7. Family recognition is not candidate semantic authorization

Exact `PRWP` prefix recognition proves only routing family selection.

It does not prove:

- outer `ControlMessageKind::Command`;
- supported PRWP major/minor version;
- supported candidate-publication operation;
- reserved-field validity;
- candidate bounds or endpoint validity;
- authenticated publisher authority;
- transport-identity equality;
- requester/rendezvous authority;
- publication freshness;
- reachability-owner authority;
- durable commit eligibility.

The existing `decode_candidate_publication_control_frame(...)` remains authoritative for strict structural Command decoding after family recognition.

All later candidate execution authorities remain separately authoritative.

## 8. Selected bridge-owned candidate transaction custody

On exact `PRWP` prefix recognition and successful strict existing candidate-publication decode, the bridge must return one opaque candidate-publication transaction envelope retaining exactly:

1. the decoded `CandidatePublicationControlFrame`; and
2. the exact same already-accepted `MeshControlStream` by value.

The exact Rust type name is left to source materialization, but it must be equivalent to a bridge-owned candidate transaction custody type.

The stream cannot simultaneously remain owned by the ingress function, capability transaction, requester transaction, Agent owner, or any second handler.

## 9. Candidate transaction surface

The future candidate transaction may expose only narrow custody operations sufficient for later separately gated composition, including:

- immutable borrow of the strict decoded `CandidatePublicationControlFrame`;
- consuming transfer of the exact decoded command plus exact same stream to a bridge-owned or opaque higher-layer handoff.

C03e-GD does not select a raw `MeshControlStream` getter, clone, duplicate handle, generic stream escape hatch, or direct Agent dependency on `prw-remote-transport` merely to name the stream.

A later response-I/O checkpoint may add a consuming send surface to this bridge-owned candidate transaction, but GD does not select that write operation.

## 10. Strict decode failure behavior

Once exact `PRWP` prefix selects the candidate-publication family, strict decode failure must:

- return a distinct candidate-publication wire/control-frame ingress error;
- not reinterpret the frame as a capability request;
- not reinterpret the frame as requester/rendezvous;
- not perform a second frame read;
- not write a fallback response;
- not retry or resynchronize;
- not execute candidate semantics.

The consumed stream remains consumed by the failed single-read ingress attempt. Peer-close policy remains separately gated.

## 11. Existing capability compatibility

C03e-GD preserves the existing capability path for every bounded frame whose payload does not begin exact `PRWZ` or exact `PRWP`.

The capability transaction continues to retain:

- the exact already-read `ControlFrame`; and
- the exact same `MeshControlStream` required by its existing response path.

C03e-GD does not alter capability authorization, capability wire semantics, dispatch, response I/O, error projection, or peer disposition.

## 12. Existing requester/rendezvous compatibility

Exact `PRWZ` routing, strict decode, requester response-stream custody, DR continuation, requester acknowledgement composition/write, and requester-aware repeated ingress behavior remain unchanged.

C03e-GD does not alter requester/rendezvous authority, registration, cleanup, lifecycle, cancellation, or peer disposition.

Candidate-publication family recognition cannot use requester `SessionId`, target `DeviceId`, requester record state, or requester response correlation as a classifier.

## 13. Publisher identity authority remains session-owned

The candidate transaction carries structural command/correlation state only.

Publisher logical identity must later come from the existing authenticated logical session owned by the current authenticated Agent session owner. It must not be derived from:

- PRWP payload fields;
- candidate endpoints;
- outer request ID;
- lower transport bytes;
- requester/rendezvous grant;
- cleanup identity.

C03e-GD does not select the exact future session-to-FY execution adapter; that remains separately gated.

## 14. Request correlation remains non-authorizing

`CandidatePublicationControlFrame::request_id()` remains the exact peer-originated outer correlation value.

It is not:

- publisher identity;
- requester identity;
- transport identity;
- freshness authority;
- rendezvous authority;
- replay authority;
- durable owner key.

No new request ID may be allocated during Mesh ingress classification or custody transfer.

## 15. FY/GA/GC remain dormant after GD

C03e-GD does not invoke:

- `SharedRequesterRendezvousAuthority::execute_authenticated_candidate_publication_with_post_commit_cleanup(...)`;
- `project_candidate_publication_terminal_result(...)`;
- `compose_candidate_publication_terminal_result_frame(...)`.

Those seams remain dormant after GD selection.

A later integration checkpoint must explicitly select how the current authenticated Agent session supplies publisher-session authority to the FY semantic path without fabricating or reusing historical pre-Mesh `AuthenticatedPrwcConnection` custody.

## 16. ProductionReachabilityOwner custody remains unresolved and separately gated

C03e-GD does not select or materialize an Agent higher owner for:

`ProductionReachabilityOwner<S, T>`

Current Agent reachability bootstrap source exposes live-owner authority composition but does not, by itself, prove current ownership/recovery of a `ProductionReachabilityOwner<S,T>` suitable for FY candidate commit execution.

A later fresh audit must select exact production reachability-owner representation, peer keying, recovery/construction, synchronization, lifecycle, and failure custody before real candidate execution can be wired.

No current live-owner authority may be treated as equivalent to `ProductionReachabilityOwner<S,T>` merely because both relate to reachability.

## 17. No response write selected

GC already materializes pure terminal frame composition, but GD selects no current-Mesh candidate response write.

Specifically GD does not select:

- `MeshControlStream::send_frame(...)` for candidate results;
- send-direction finish behavior;
- response I/O error classification;
- whether frame-construction failure consumes stream custody;
- peer close after result write failure;
- automatic Rejected fallback;
- retry/re-encode behavior.

Those decisions require a later dedicated response-custody/write gate.

## 18. No command loop or repeated candidate execution selected

GD is one single-read custody selection only.

It does not select:

- repeated candidate Commands on one stream;
- repeated candidate transactions across streams;
- candidate/capability/requester fairness;
- queueing;
- concurrent candidate execution;
- backpressure;
- cancellation after candidate handoff;
- retry/reconnect;
- post-response loop resumption.

A future source-materialization successor may only materialize the isolated third-family ingress/custody boundary unless additional behavior is separately selected.

## 19. Audit-basis exact GC source

C03e-GD selection is based on exact closed-GC source including:

- `crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs` — blob `301d8bfbd57db09ecf5922f579dc146cca151003`;
- `crates/prw-remote-bridge/src/candidate_publication_control_frame.rs` — blob `20ff7d2bc5f32596a3c0696aa387e6735f8f2031`;
- `crates/prw-remote-bridge/src/candidate_publication_wire.rs` — blob `299042938b38b65b78f737926f74b8567e5046fb`;
- `crates/prw-remote-bridge/src/prwc_connection_authentication.rs` — exact GC source containing historical pre-Mesh CN/CO/CX seams;
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs` — blob `638e4035404ed1fc6f178adcdc620b9a50b24dad`;
- `crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs` — blob `0c53f16d1e1dde8a7c1a328a8ac20c4c6a42311b`;
- `crates/prw-remote-bridge/src/reachability_owner.rs` — blob `8d0e65c3fc0bd646c257199d4f55be65fa3f792d`;
- `crates/prw-agent/src/reachability_authority_custody_bootstrap.rs` — blob `2843cbf9cfed7ae26e336ec4a2ead6a97855b2c0`.

All audit-basis source remains byte-stable in this docs-only checkpoint.

## 20. Exact GD diff boundary

C03e-GD is documentation-only selection.

The exact GC -> GD diff is authorized to contain only:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_GD_CANDIDATE_PUBLICATION_MESH_POST_AUTH_CONTROL_STREAM_SAME_STREAM_CUSTODY_SEMANTICS_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, Agent/Desktop/Android source, runtime configuration, deployment path, or second contract path blocks GD closure.

## 21. Validation gate

C03e-GD may close only after:

- exact GC predecessor remains unchanged;
- GC -> GD compare is ahead 1 / behind 0 with exact GC merge base;
- exactly one authorized docs-only path changed;
- every automatically triggered workflow reaches a terminal non-failing verdict;
- skipped workflows are recorded as SKIPPED, never PASS;
- PR remains draft/open/unmerged;
- immutable Drive audit is uploaded to the canonical Private Remote Workspace folder and raw byte/hash readback passes;
- PR body changes to `Status: CLOSED` only after durable Drive verification.

No Android PASS may be claimed unless Android actually runs successfully on the exact GD head.

## 22. Safe successor rule

After durable GD closure, the next safe checkpoint may source-materialize only the selected third-family current-Mesh candidate-publication ingress/custody extension.

That source materialization must:

- preserve exact `PRWZ` requester behavior;
- divert exact `PRWP` prefix to existing strict candidate decode;
- preserve all other frames on capability fallback;
- retain exact decoded candidate Command plus exact same `MeshControlStream` in bridge-owned custody;
- perform one read only;
- perform no candidate execution;
- perform no response write;
- perform no reachability-owner recovery/construction;
- activate no loop/listener/readiness/network behavior.

After that materialization validates, a fresh audit must choose the next prerequisite among current authenticated-session execution adaptation, `ProductionReachabilityOwner` custody/recovery, and current-Mesh terminal response write custody. No ordering is pre-authorized here.

## 23. Explicitly rejected shortcuts

C03e-GD rejects:

- treating C03e-CN/CX historical `ControlTlsServerStream` as current Mesh custody;
- fabricating an `AuthenticatedPrwcConnection` around a Mesh stream;
- routing `PRWP` through capability after exact family recognition;
- using request ID as publisher or requester authority;
- using transport identity as logical `DeviceId`;
- exposing raw `MeshControlStream` broadly to Agent;
- holding requester mutex while waiting on frame I/O;
- executing FY during family classification;
- flattening cleanup failure into semantic Rejected;
- constructing/recovering production reachability owner in ingress;
- response write/retry/re-encoding;
- candidate execution loop;
- listener/readiness activation;
- traversal/dialing;
- deployment;
- restart/recovery;
- merge.

## 24. Completion meaning

Closure means only that the current-Mesh bridge-owned post-auth single-read ingress extension for strict `PRWP` candidate-publication Command plus exact same-stream custody is selected.

It does not mean the source exists, candidate publication executes, FY/GA/GC are invoked, a production reachability owner exists in Agent runtime, a result is written, a loop is active, a listener is activated, traffic is dialed, anything is deployed, or any PR is merged.

Canonical closure target:

`CLOSED_CANDIDATE_PUBLICATION_MESH_POST_AUTH_CONTROL_STREAM_SAME_STREAM_CUSTODY_SEMANTICS_SELECTION`

Target gate:

`C03E_GD_CANDIDATE_PUBLICATION_MESH_POST_AUTH_CONTROL_STREAM_SAME_STREAM_CUSTODY_SEMANTICS_SELECTED`
