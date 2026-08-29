# Phase 152 C03e-GC — Candidate Publication Post-Commit Cleanup Terminal Result Frame Composition Source Materialization

Status: VALIDATING

## 1. Purpose

C03e-GC materializes only the C03e-GB-selected pure candidate-publication terminal-result frame-composition boundary.

GC consumes one completed GA projection and one exact already-decoded candidate-publication command, delegates semantic framing to the existing bridge codec, and preserves post-commit requester cleanup disposition separately from the local frame-construction result.

GC performs no frame write, same-stream custody, ingress classification, semantic execution, requester mutation, reachability mutation, production reachability-owner construction/recovery, command loop, listener/readiness activation, traversal/dialing, deployment, restart/recovery, or merge.

## 2. Exact predecessor

Canonical predecessor gate:

`C03E_GB_CANDIDATE_PUBLICATION_POST_COMMIT_CLEANUP_TERMINAL_RESULT_FRAME_COMPOSITION_SEMANTICS_SELECTED`

Exact predecessor branch:

`phase-152-c03e-gb-candidate-publication-post-commit-cleanup-terminal-result-frame-composition-semantics-selection-staging`

Exact GB head:

`2fc3c717db85833d5a5da66dc5820d6f49c1c13b`

Exact GB tree:

`a6b377ef905fb9a77083109cce1f83b5f11cca2d`

GB remains frozen.

## 3. Fresh source audit and narrow bridge support exception

GB selected an Agent-side carrier preserving:

- exact `Result<ControlFrame, CandidatePublicationResultWireError>`;
- exact optional `Result<(), RequesterRendezvousLifecycleError>`.

Fresh exact-GB source audit found that `prw-agent` deliberately has no direct dependency on `prw-control-transport` or `prw-remote-transport`. Adding a direct transport dependency only so Agent can name `ControlFrame` would expand both manifest/lock custody and transport surface for a pure composition checkpoint.

The existing bridge result codec already owns `ControlFrame` and the authoritative frame encoder.

Therefore GC uses GB's explicit allowance for a minimal bridge support change when byte-stability is impossible without a larger dependency expansion: the bridge adds only a generic custody carrier around an already-computed frame result plus one opaque disposition. Existing Accepted/Rejected projection and encoding functions remain unchanged.

The bridge carrier does not know requester cleanup semantics and creates no bridge -> Agent dependency.

## 4. Bridge-owned generic frame composition carrier

GC materializes:

`CandidatePublicationResultFrameComposition<D>`

inside the existing candidate-publication result-wire module.

It contains exactly:

- `Result<ControlFrame, CandidatePublicationResultWireError>`;
- generic opaque disposition `D`.

Its constructor merely pairs those already-computed values.

Its consuming `into_parts()` transfers both values unchanged.

The carrier performs no:

- encoding;
- semantic projection;
- disposition inspection;
- frame I/O;
- retry;
- mutation;
- runtime drive.

## 5. Existing codec remains authoritative

GC does not alter the behavior of:

- `project_candidate_publication_execution_result(...)`;
- `encode_candidate_publication_result_frame(...)`;
- `encode_candidate_publication_execution_result_frame(...)`;
- decoder behavior;
- Accepted/Rejected operation tags;
- outer frame kinds;
- replacement freshness bytes;
- exact request-ID echo;
- existing frame validation/error taxonomy.

Agent composition calls the existing `encode_candidate_publication_execution_result_frame(...)` directly.

## 6. Agent-owned concrete composition alias

GC materializes Agent-private concrete disposition ownership as:

`CandidatePublicationTerminalFrameComposition`

which specializes the bridge generic carrier to:

`Option<Result<(), RequesterRendezvousLifecycleError>>`.

The alias adds no direct control-transport dependency to Agent.

## 7. Agent pure frame composition helper

GC materializes:

`compose_candidate_publication_terminal_result_frame(command, projection)`

with exact inputs:

- `&CandidatePublicationControlFrame`;
- by-value `CandidatePublicationTerminalResultProjection`.

Exact order:

1. consume the GA projection into exact semantic result plus exact optional cleanup disposition;
2. call existing `encode_candidate_publication_execution_result_frame(command, semantic_result)` exactly once;
3. pair that exact frame result with the unchanged cleanup disposition using the bridge generic carrier;
4. return the carrier.

The helper performs no frame write or transport I/O.

## 8. Exact request correlation

The helper passes the exact already-decoded command reference into the existing bridge encoder.

No local request ID is allocated, replaced, normalized, incremented, or inferred from any session/device/transport/cleanup/task state.

Request ID remains correlation only.

## 9. Committed success and cleanup success

GA already proves that committed + cleanup success yields semantic `Ok(ReachabilityCommitOutcome)` plus cleanup `Some(Ok(()))`.

GC passes that semantic success unchanged to the existing bridge encoder and preserves cleanup success separately.

Existing bridge Accepted tests remain authoritative for exact Response framing and replacement freshness encoding.

## 10. Committed success and cleanup failure

GA already proves that committed + typed cleanup failure still yields semantic `Ok(ReachabilityCommitOutcome)` plus exact `Some(Err(RequesterRendezvousLifecycleError))`.

GC passes that semantic success unchanged to the existing bridge encoder and preserves the exact cleanup failure separately.

Cleanup failure cannot become:

- `CandidatePublicationExecutionError`;
- `CandidatePublicationResultWireError`;
- Rejected;
- a second peer response;
- a retry/replay trigger.

## 11. Pre/at-commit execution failure

GA already proves pre/at-commit failure yields exact `Err(CandidatePublicationExecutionError)` with cleanup absent.

GC passes that exact semantic error to the existing bridge encoder.

Existing bridge generic Rejected framing remains authoritative.

Cleanup remains absent; no cleanup disposition is fabricated.

## 12. Frame-construction failure preservation

If the existing bridge encoder returns `CandidatePublicationResultWireError`, the generic bridge carrier retains that exact frame error independently from the disposition.

GC performs no:

- semantic replay;
- cleanup retry;
- durable rollback;
- requester resurrection;
- fallback Rejected construction;
- automatic second encoding;
- frame write.

## 13. Test composition law

GC source tests preserve the proof chain across existing/new boundaries:

- existing GA generic tests prove pre-commit error -> cleanup absent;
- existing GA generic tests prove committed success + cleanup success stays semantic success;
- existing GA generic tests prove committed success + typed cleanup error stays semantic success;
- existing bridge Accepted/Rejected tests remain unchanged and prove exact wire result semantics;
- new bridge carrier tests prove an existing frame result and opaque disposition are transferred unchanged;
- new bridge carrier tests prove a frame-construction error remains exact while opaque disposition is preserved;
- new Agent test proves typed requester cleanup failure remains exact beside a frame-construction failure;
- new Agent signature test proves the real helper accepts exact decoded command + GA projection and returns the concrete GC composition alias.

No invalid private `ReachabilityCommitOutcome` is fabricated solely for testing.

## 14. No transport dependency expansion

GC adds no Agent dependency on `prw-control-transport` or `prw-remote-transport`.

No Cargo manifest or lockfile changes are authorized.

The concrete control-frame type remains bridge-owned.

## 15. No cleanup state on wire

The bridge generic disposition is opaque and is never read by candidate-publication encoding.

Agent's concrete cleanup disposition therefore cannot alter:

- PRWP magic/version;
- Accepted/Rejected operation;
- Response/Error outer kind;
- request ID;
- replacement freshness;
- payload length;
- frame validation.

No requester SessionId, expected publisher DeviceId, lifecycle error, provider state, or retry hint is serialized.

## 16. No I/O or runtime activation

GC does not:

- send a frame;
- finish a QUIC send direction;
- retain/consume a stream;
- accept/receive a stream;
- classify `PRWC` ingress;
- close a peer;
- spawn a task;
- resume a loop;
- bind/listen;
- publish readiness;
- dial or forward traffic.

## 17. No semantic or lifecycle mutation

GC does not:

- authorize requester state;
- register/retire/remove requester state;
- commit/reload/retire reachability state;
- acquire/currentness-check/release live-owner authority;
- provision/poll traversal;
- retry cleanup;
- re-execute candidate publication.

It consumes only already-completed GA evidence.

## 18. Security and privacy boundary

GC creates no authorization authority.

`ControlFrame` construction is not permission to send or dial.

`ReachabilityCommitOutcome` remains completed-commit evidence only.

Cleanup disposition remains internal and cannot leak identity/provider details onto the wire.

## 19. Authorized source paths

GC source materialization is authorized to change only:

1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_GC_CANDIDATE_PUBLICATION_POST_COMMIT_CLEANUP_TERMINAL_RESULT_FRAME_COMPOSITION_SOURCE_MATERIALIZATION_STAGING.md`;
2. `crates/prw-remote-bridge/src/candidate_publication_result_wire.rs` — only generic frame/disposition custody support and focused tests; existing encoding law must remain semantically unchanged;
3. `crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs` — concrete cleanup alias, pure composition helper, and focused tests.

No manifest, lockfile, workflow, Kotlin/Android application source, ingress/listener/runtime source, deployment path, or unrelated contract change is authorized.

## 20. Validation requirements

GC may close only if one exact final head proves:

1. GB remains exact merge base;
2. only the three authorized paths differ;
3. Rust locked graph, rustfmt, Clippy, workspace tests and workspace build pass on the exact final head;
4. Android validation, if automatically triggered by these source paths, reaches terminal success on the exact final head; if no Android run exists, no Android PASS may be claimed;
5. non-applicable AD/AE workflows are recorded as skipped rather than PASS;
6. immutable Drive audit is uploaded directly to canonical `Private Remote Workspace` folder ID `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT` and raw-read back byte-for-byte;
7. PR changes to `Status: CLOSED` only after durable evidence succeeds;
8. PR remains open/draft/unmerged.

## 21. Explicit non-goals

GC does not materialize or select:

- same-stream candidate-publication response I/O;
- candidate-publication ingress classification;
- combined command loops;
- production `ProductionReachabilityOwner` construction/recovery;
- reachability authority activation;
- target path selection/dialing;
- listener/readiness activation;
- Agent binary wiring;
- deployment;
- process restart/recovery;
- merge.

## 22. Closure target

Canonical closure:

`CLOSED_CANDIDATE_PUBLICATION_POST_COMMIT_CLEANUP_TERMINAL_RESULT_FRAME_COMPOSITION_SOURCE_MATERIALIZATION`

Canonical gate:

`C03E_GC_CANDIDATE_PUBLICATION_POST_COMMIT_CLEANUP_TERMINAL_RESULT_FRAME_COMPOSITION_SOURCE_MATERIALIZED`

After closure, GC intentionally requires a fresh exact-head prerequisite audit before any successor is named or materialized.