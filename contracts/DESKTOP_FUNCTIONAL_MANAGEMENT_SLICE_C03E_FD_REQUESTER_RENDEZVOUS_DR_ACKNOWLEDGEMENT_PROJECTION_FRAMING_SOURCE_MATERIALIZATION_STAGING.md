# Phase 152 C03e-FD — Requester/Rendezvous DR Acknowledgement Projection/Framing Source Materialization (Staging)

Status: SOURCE_MATERIALIZATION_STAGING

## 1. Purpose

C03e-FD materializes only the C03e-FC-selected pure requester/rendezvous terminal DR acknowledgement projection and framing source.

The source boundary remains pure and no-I/O. It borrows the exact retained requester transaction only for its original PRWM request correlation and borrows the already-completed DR result only for `Ok(())` versus `Err(_)` projection. It consumes no stream custody, writes no response, performs no second read, resumes no ingress loop, selects no candidate/reachability/endpoint/relay, dials no transport, and activates no runtime/listener/bootstrap/deployment path.

## 2. Exact predecessor

Canonical predecessor C03e-FC:

- branch: `phase-152-c03e-fc-requester-rendezvous-retained-custody-dr-response-materialization-semantics-selection-staging`
- head: `e5993253df9b52f16ed2345a0b95c1d54150b710`
- tree: `308a5f42adb308343bf780d5876ca30717845279`
- contract blob: `3fc3d1078646e893047aefed0cd2ae8c2892c1eb`

FD must remain a direct source-materialization descendant of that exact closed FC checkpoint.

## 3. Audited source guards

At exact FC head:

- bridge crate root: `crates/prw-remote-bridge/src/root.rs`
  - blob: `45bad34997f4b109d68d4f28fdec2729edebed96`
- strict requester target-request codec: `crates/prw-remote-bridge/src/requester_rendezvous_target_request_wire.rs`
  - blob: `2bfb2d6119a0bc3e1278fb361955093361949db1`
- bridge retained requester transaction: `crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`
  - blob: `d5562a9587bdbde7d05e38fdd704d42f9d20f3c8`
- FB retained-custody DR continuation: `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
  - blob: `9550148740b654a79eca8e51956bf37a351ac802`
- DR composition: `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_composition.rs`
  - blob: `8ef66c9bd9e2ca65e2b21291a445ddeebbbf4090`
- candidate-publication terminal result precedent only: `crates/prw-remote-bridge/src/candidate_publication_result_wire.rs`
  - blob: `b7aa6b05a037cdc5db1bcd454f5c076890d95c96`

FD intentionally changes only the bridge crate root to declare the new isolated module. All other guards above must remain byte-stable.

## 4. Materialized ownership split

FD preserves the FC ownership law:

- Agent owns provenance of the exact completed `Result<(), RequesterRendezvousStartCompositionError>`;
- bridge protocol code owns requester/rendezvous acknowledgement projection/framing semantics;
- `PostAuthRequesterRendezvousTransaction` continues to own the exact same already-accepted control stream;
- requester identity remains authenticated PRW application-session identity already captured before DR;
- target identity remains the nominated logical `DeviceId` lineage;
- PRWM `request_id` remains echo correlation only.

The bridge projector is generic over error type `E` and examines only whether the completed result is `Ok(())` or `Err(_)`. This prevents bridge code from depending on Agent-internal DR error taxonomy and prevents accidental error-detail leakage.

## 5. Materialized requester-visible semantic type

FD materializes one requester-specific acknowledgement enum with exactly two states:

- `Accepted`
- `Rejected`

`Accepted` means only that the already-completed DR stage succeeded: current validation, requester-aware authorization and requester registration mutation completed successfully.

`Accepted` does not mean target online, candidate available/selected, reachability established, endpoint/relay selected, transport connected, port-forward active, terminal active, remote session established, rendezvous complete, or end-to-end success.

`Rejected` is the only requester-visible DR failure semantic. All internal DR failures map to it without inspecting, formatting or serializing the internal error.

## 6. Materialized PRWZ framing

FD keeps requester/rendezvous acknowledgements inside the existing requester-specific `PRWZ` v1.0 protocol family.

Exact framing materialized by FD:

- magic: existing `PRWZ`
- major: existing `1`
- minor: existing `0`
- existing request operation remains `1`
- terminal DR accepted operation: `2`
- terminal DR rejected operation: `3`
- flags: exactly `0`
- acknowledgement payload size: exactly `12` bytes
- no acknowledgement result body
- accepted outer PRWM kind: `Response`
- rejected outer PRWM kind: `Error`

The accepted frame therefore carries no fabricated candidate/reachability/endpoint/relay/transport/session data. The rejected frame carries no internal DR error detail.

## 7. Exact correlation law

FD materializes a helper that borrows the exact retained `PostAuthRequesterRendezvousTransaction` and echoes only:

`transaction.request().request_id()`

into the acknowledgement frame.

The helper does not allocate, replace, derive or register another request ID. It does not interpret request ID as identity or authority.

The transaction is borrowed, not consumed. Its retained stream remains untouched for a later separately gated same-stream response-write checkpoint.

## 8. Failure-boundary preservation

FD preserves three distinct layers:

1. completed DR semantic result;
2. coarse requester-visible `Accepted` / `Rejected` projection;
3. local PRWM frame-construction failure.

An internal DR error becomes a valid semantic `Rejected` acknowledgement. It is not converted into a local codec error.

A local frame-construction error does not become `Rejected`, does not fabricate a frame, and does not rerun DR or registration.

## 9. Strict decode law

FD also materializes strict pure decode for the acknowledgement family so protocol structure is testable without I/O.

Decode rejects:

- any payload length other than exactly 12 bytes;
- wrong PRWZ magic;
- unsupported major/minor version;
- non-zero flags;
- unknown acknowledgement operation;
- trailing bytes; or
- mismatch between operation and outer PRWM `Response`/`Error` kind.

Successful decode proves only bounded wire structure and exact outer-kind pairing. It proves no DR provenance, identity, reachability or rendezvous success.

## 10. Source surface

FD materializes only:

1. `crates/prw-remote-bridge/src/requester_rendezvous_dr_acknowledgement_wire.rs`
2. one module declaration in `crates/prw-remote-bridge/src/root.rs`
3. this FD contract

No Agent source modification is required because the bridge projector is generic over the completed error type and the retained transaction type already belongs to the bridge crate.

## 11. Test surface

The isolated bridge module tests must cover at minimum:

- accepted round-trip with exact request correlation;
- rejected round-trip with outer `Error` kind;
- accepted/rejected exact 12-byte payload size;
- generic `Err(E)` projection to one `Rejected` semantic across different `E` types;
- outer-kind mismatch rejection;
- non-zero flags rejection;
- unknown operation rejection; and
- trailing-data rejection.

These tests perform no stream I/O.

## 12. Repeated-ingress barrier

FD preserves the existing terminal requester path while response custody is unresolved.

No second accept/read or second frame receive may occur before a later gate deliberately consumes or otherwise resolves retained response custody.

FD creates no task, queue, channel, worker, concurrent reader, speculative accept, fairness policy, backpressure policy, retry policy or loop-resume policy.

## 13. Explicit non-goals

C03e-FD does not materialize or authorize:

- response stream write;
- transaction send adapter consuming stream custody;
- second accept/read;
- mixed-family loop resume;
- retry/fallback/replacement/fabricated success;
- stream/peer-close policy;
- detailed external DR error taxonomy;
- candidate-publication result-codec reuse;
- candidate selection;
- reachability selection;
- endpoint selection;
- relay selection;
- QUIC/TCP dialing;
- port-forward activation;
- terminal activation;
- remote-session activation;
- runtime/listener/bootstrap activation;
- Android behavior changes;
- dependency/workflow widening;
- packaging;
- deployment;
- restart;
- recovery; or
- merge.

## 14. Validation contract

Closure requires validation on the exact final FD head.

Required checks:

- exact FC merge base;
- ahead-only lineage, behind zero;
- exact changed-path accounting;
- all non-root source guards in section 3 byte-stable;
- bridge root differs only by the isolated FD module declaration;
- locked dependency graph PASS;
- rustfmt PASS;
- Clippy with warnings denied PASS;
- workspace tests PASS;
- workspace build PASS;
- Android PASS only if Android validation actually triggers for the exact final head;
- immutable Drive audit with raw byte-exact readback.

Any validation contradiction is not permission to widen FD beyond the selected source boundary.

## 15. Next-gate recommendation

If FD closes cleanly, the next separately gated checkpoint should audit/select ownership for consuming the exact retained requester transaction to write exactly one already-materialized DR acknowledgement on the same control stream.

That next gate must decide write-error handling and custody termination before any second ingress read or loop resume. It must not infer candidate/reachability/dialing/runtime activation permission from FD.

## 16. Canonical closure target

Intended closure marker:

`CLOSED_REQUESTER_RENDEZVOUS_DR_ACKNOWLEDGEMENT_PROJECTION_FRAMING_SOURCE_MATERIALIZATION`

Intended gate marker:

`C03E_FD_REQUESTER_RENDEZVOUS_DR_ACKNOWLEDGEMENT_PROJECTION_FRAMING_SOURCE_MATERIALIZED`
