# Phase 152 C03e-FG — Requester/Rendezvous Terminal DR Acknowledgement Response Composition Selection (Staging)

Status: SEMANTICS_SELECTION_STAGING

## 1. Purpose

C03e-FG selects only the Agent-owned terminal composition boundary that may later consume one exact C03e-FB retained-custody DR continuation, invoke the existing C03e-FD pure requester/rendezvous acknowledgement framing boundary, and then invoke the existing C03e-FF consuming same-stream send surface.

FG is docs-only. It performs no response framing or write, consumes no live transaction, changes no Rust/Android/dependency/workflow source, resumes no ingress loop, selects no peer-close policy, performs no candidate/reachability/endpoint/relay work, dials no traffic, and activates no runtime/listener/bootstrap/deployment path.

## 2. Exact predecessor

Canonical predecessor:

- branch: `phase-152-c03e-ff-requester-rendezvous-dr-acknowledgement-consuming-send-source-materialization-staging`
- head: `c08f5033796555c303c88c3e16bca44d331c8aa2`
- tree: `ba6902c8e70f92600d0917cbd658be21f0da18ff`
- FF contract blob: `b33c53d1291930f835c596fe91af005883c02f61`
- FF bridge requester transaction/send blob: `301d8bfbd57db09ecf5922f579dc146cca151003`

FG must remain an exact docs-only descendant of that head.

## 3. Exact audited source guards

The FG selection is bounded by these exact FF-head blobs:

- `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
  - `9550148740b654a79eca8e51956bf37a351ac802`
- `crates/prw-remote-bridge/src/requester_rendezvous_dr_acknowledgement_wire.rs`
  - `71b8cd166b24268b1fd87f8f339f57200f426834`
- `crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`
  - `301d8bfbd57db09ecf5922f579dc146cca151003`
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`
  - `406cbf276c2c62a0bbd902a6ec25b8a0f93ca05c`
- `crates/prw-agent/src/remote_session_capability_runtime.rs`
  - `6b9b7bfa2445e3cbc7e713b598f67f7ec6115e8f`

No source guard may change in FG.

## 4. Exact predecessor facts

At exact FF head:

- Agent-owned `RequesterRendezvousRetainedCustodyDrContinuation` retains by value the exact bridge `PostAuthRequesterRendezvousTransaction` plus the exact terminal `Result<(), RequesterRendezvousStartCompositionError>`;
- the continuation exposes borrowed access to both exact values and an `into_parts()` custody transfer;
- FD `encode_requester_rendezvous_dr_result_for_transaction(...)` borrows the exact requester transaction only to echo the original PRWM `request_id` and borrows the exact DR result to project `Ok(())` to accepted or every `Err(_)` to generic rejected;
- FD framing performs no stream I/O and preserves frame-construction failure separately from semantic accepted/rejected;
- FF `PostAuthRequesterRendezvousTransaction::send_dr_acknowledgement_frame(self, &ControlFrame)` consumes exact transaction custody, performs exactly one same-stream bounded write, and preserves requester-specific response-I/O failure;
- FF does not invoke Agent code and remains isolated/uninvoked;
- repeated ingress remains stopped at the requester handoff barrier.

These facts constrain FG.

## 5. Selected ownership boundary

C03e-FG selects Agent ownership for the terminal composition.

This is required because Agent owns the semantic provenance of the exact completed DR result, while bridge code owns only requester/rendezvous protocol framing and retained same-stream transport custody.

A later source-materialization checkpoint should place the composition beside the existing Agent-owned retained-custody DR continuation rather than moving DR result interpretation into bridge code.

No bridge dependency on Agent error types is selected. No dependency direction changes.

## 6. Selected terminal composition sequence

A future Agent-owned composition must execute exactly this logical sequence for one exact retained continuation:

1. borrow the exact retained requester transaction and exact terminal DR result from the same continuation;
2. invoke exactly once the existing FD `encode_requester_rendezvous_dr_result_for_transaction(...)` helper;
3. if framing succeeds, consume the exact continuation by value through its existing custody-transfer boundary;
4. invoke exactly once the existing FF `send_dr_acknowledgement_frame(...)` on the exact requester transaction with the exact FD frame;
5. return terminal success only if that exact FF send succeeds.

The composition must not reconstruct requester identity, target identity, request correlation, DR semantics, PRWZ bytes, or a replacement transaction.

## 7. DR semantic result law

The exact DR result remains semantic provenance, not a terminal-composition failure category.

Therefore:

- exact DR `Ok(())` must produce the existing FD accepted acknowledgement;
- every exact DR `Err(RequesterRendezvousStartCompositionError)` must produce the existing FD generic rejected acknowledgement;
- an exact DR error must not be returned as the future composition error merely because it is an internal semantic failure;
- the composition must not expose internal DR error discriminants, strings, registry/policy/provider detail, or retry hints;
- DR must not be re-run;
- requester registration mutation must not be repeated.

A successfully sent generic rejected acknowledgement is a successful terminal response composition, even though the underlying DR semantic result was rejection.

## 8. Frame-construction failure law

FD frame construction remains a local terminal-composition failure distinct from DR semantics and response I/O.

If the existing FD helper returns `RequesterRendezvousDrAcknowledgementWireError`:

- no FF send may be attempted;
- no fallback frame may be constructed;
- no semantic rejected frame may be fabricated to replace the local framing failure;
- no second request ID may be allocated;
- no DR retry or registration retry may occur;
- the future Agent composition must return a typed framing-failure variant.

FG selects terminal local custody resolution on this path: because the future composition consumes the exact retained continuation by value, a frame-construction failure must not return a retry-capable continuation or requester transaction to its caller.

Dropping that consumed local transaction custody is not permission to close the authenticated peer.

## 9. Response-I/O failure law

If FD framing succeeds but FF same-stream send returns `RequesterRendezvousDrAcknowledgementResponseIoError`:

- the future Agent composition must return a distinct typed response-I/O variant;
- the error must not be flattened into the FD framing error family;
- the error must not become semantic rejected;
- no retry/resend/replacement stream or second acknowledgement may occur;
- no DR or registration mutation may be repeated;
- exact requester transaction custody remains terminal because FF already consumes it by value.

FG selects no automatic whole-peer close on this failure.

## 10. Selected Agent composition error family

A later source-materialization checkpoint should expose one narrow Agent-local terminal response composition error family with exactly two semantic categories:

1. **Frame** — preserves the existing `RequesterRendezvousDrAcknowledgementWireError` from FD framing;
2. **ResponseIo** — preserves the existing `RequesterRendezvousDrAcknowledgementResponseIoError` from FF same-stream send.

Exact Rust type/variant names remain source-materialization details, but no third variant for DR `Err(_)` is selected.

The two lower errors should remain available as error sources without string translation or loss of provenance.

## 11. Terminal success law

Future terminal composition `Ok(())` may mean only:

- the exact completed DR result was projected through the existing FD semantics;
- one exact FD acknowledgement frame carrying the original requester correlation was constructed successfully; and
- that exact frame was successfully written through FF on the exact retained same stream, including successful send-direction finish.

It must not mean or imply:

- target online;
- candidate available or selected;
- reachability success;
- endpoint or relay selected;
- direct path available;
- QUIC/TCP target connection established;
- port-forward established;
- terminal established;
- remote session established;
- rendezvous complete;
- end-to-end operation success; or
- independently confirmed requester application consumption.

## 12. Correlation and identity law

The future composition must preserve the existing identity/correlation boundaries unchanged:

- the original PRWM `request_id` is echo correlation only;
- requester logical identity remains the authenticated PRW application-session lineage already consumed by DR;
- target logical identity remains the exact nominated logical `DeviceId`;
- dynamic IP/port remain transient endpoint data;
- `TransportIdentity` remains lower transport evidence only.

The composition must not derive or reconstruct identity from stream metadata, endpoint tuples, request ordering, or request correlation.

## 13. Custody-resolution law

FG selects the future terminal Agent composition as a consuming boundary for the complete retained response custody envelope.

After the future composition is invoked, no result path may return:

- `RequesterRendezvousRetainedCustodyDrContinuation`;
- `PostAuthRequesterRendezvousTransaction`;
- raw `MeshControlStream`;
- a replacement transaction;
- a retry token; or
- a second response opportunity.

This applies to:

- FD framing success + FF send success;
- FD framing failure; and
- FF response-I/O failure.

The purpose is to make one terminal response attempt a one-way custody transition without replay ambiguity.

## 14. Repeated-ingress barrier remains

FG does not lift the repeated-ingress barrier.

The future terminal composition may resolve exact requester response custody, but loop behavior after that terminal result remains a separate decision.

FG does not authorize:

- a second `accept_control_stream()`;
- a second `receive_frame()`;
- mixed-family loop resume;
- capability-loop resume;
- speculative pre-accept;
- concurrent requester handling;
- queue/channel/task creation;
- fairness/backpressure policy.

A later gate must separately decide whether terminal response success, frame failure, or response-I/O failure may resume, stop, or close a higher lifecycle.

## 15. No peer-close policy

FG selects no requester-specific close code, close reason, automatic peer teardown, or reuse of capability code-3/code-4 diagnostics.

A future terminal composition returns typed local failures only.

Any whole-peer close decision remains a separately gated lifecycle policy.

## 16. No candidate/reachability continuation

FG does not invoke or authorize any post-DR candidate/reachability work.

In particular it does not select:

- candidate query or selection;
- target reachability evaluation;
- endpoint resolution;
- relay selection;
- direct-vs-relay selection;
- QUIC/TCP target dialing;
- port-forward activation;
- terminal activation;
- remote-session activation.

FD accepted remains only accepted-for-continuation semantics and is not strengthened by terminal response delivery.

## 17. Source stability

C03e-FG is docs-only.

All Rust, Android, dependency, workflow, protocol implementation, and runtime source must remain byte-stable from exact FF head. In particular every exact source guard in section 3 must remain unchanged.

No `Cargo.toml`, module declaration, error enum, Agent function, bridge function, test, workflow, or Android path may change in FG.

## 18. Explicit non-goals

C03e-FG does not materialize or authorize:

- Agent terminal response composition Rust source;
- Agent terminal response composition error Rust source;
- changes to FB retained-custody DR continuation source;
- changes to FD acknowledgement codec;
- changes to FF consuming send surface;
- DR retry or requester-registration retry;
- response retry/resend/replacement stream;
- second acknowledgement;
- second accept/read/frame receive;
- repeated-ingress or mixed-family loop resume;
- peer-close policy;
- candidate/reachability/endpoint/relay selection;
- target dialing;
- port-forward/terminal/remote-session activation;
- runtime/listener/bootstrap activation;
- Android behavior mutation;
- dependency/workflow widening;
- packaging;
- deployment;
- restart;
- recovery; or
- merge.

## 19. Next-gate recommendation

If FG closes cleanly, the next separately gated checkpoint may materialize only the selected Agent-owned terminal response composition and its narrow two-variant local error family.

Preferred source scope is the existing Agent retained-custody DR continuation module plus only the minimum parent-module dead-code/export adjustment if exact Rust visibility requires it.

The next materialization should reuse unchanged:

- FB continuation custody;
- FD pure framing helper;
- FF consuming same-stream send method.

It should not alter any of those lower semantics.

Loop resume, peer-close policy, candidate/reachability continuation, dialing, runtime activation, deployment, and merge remain later gates.

## 20. Validation contract

Closure requires exact-final-head validation.

Because FG is docs-only:

- exact FF merge base must be preserved;
- FG must be ahead only by its docs commit(s), behind zero;
- the exact changed-path set must contain only this FG contract;
- all exact source guards must remain byte-stable;
- Rust validation is claimed only if the repository workflow triggers and passes on exact final FG head;
- Android validation is claimed only if the workflow actually triggers and passes on exact final FG head;
- durable audit evidence must record exact final head/tree, contract blob, ancestry/scope, source guards, workflow evidence, closure marker, gate marker, and raw byte-exact Drive readback.

## 21. Canonical closure target

Intended closure marker:

`CLOSED_REQUESTER_RENDEZVOUS_TERMINAL_DR_ACKNOWLEDGEMENT_RESPONSE_COMPOSITION_SELECTION`

Intended gate marker:

`C03E_FG_REQUESTER_RENDEZVOUS_TERMINAL_DR_ACKNOWLEDGEMENT_RESPONSE_COMPOSITION_SELECTED`
