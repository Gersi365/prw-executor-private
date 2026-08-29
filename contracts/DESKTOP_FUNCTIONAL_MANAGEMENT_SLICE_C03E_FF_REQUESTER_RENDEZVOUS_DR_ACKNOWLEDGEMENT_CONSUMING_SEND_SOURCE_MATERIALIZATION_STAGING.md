# Phase 152 C03e-FF — Requester/Rendezvous DR Acknowledgement Consuming Send Source Materialization (Staging)

Status: SOURCE_MATERIALIZATION_STAGING

## 1. Purpose

C03e-FF materializes only the C03e-FE-selected bridge-owned consuming send surface for exactly one already-constructed requester/rendezvous DR acknowledgement on the exact control stream retained by `PostAuthRequesterRendezvousTransaction`.

FF also materializes the requester-specific local response-I/O failure classification selected by FE.

FF does not compose the Agent-owned retained DR result with the C03e-FD projector/framer, does not invoke the new send surface from Agent code, does not resume repeated ingress, and does not select any peer-close, retry, candidate, reachability, endpoint, relay, dialing, runtime, deployment, or merge behavior.

## 2. Exact predecessor

Canonical predecessor:

- branch: `phase-152-c03e-fe-requester-rendezvous-same-stream-dr-acknowledgement-write-custody-selection-staging`
- head: `d993d66435b1ba560940e008308c596517c4b7a1`
- tree: `d71cc4658403877e7cdb67fc150bd225446bf242`
- FE contract blob: `bad62642932ee680e87fa67486e587a7e3698de9`

FF must remain an exact narrow source-materialization descendant of that head.

## 3. Exact predecessor guards

The FF source change is constrained by these exact FE-head blobs:

- `crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`
  - `d5562a9587bdbde7d05e38fdd704d42f9d20f3c8`
- `crates/prw-remote-bridge/src/requester_rendezvous_dr_acknowledgement_wire.rs`
  - `71b8cd166b24268b1fd87f8f339f57200f426834`
- `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
  - `9550148740b654a79eca8e51956bf37a351ac802`
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`
  - `406cbf276c2c62a0bbd902a6ec25b8a0f93ca05c`
- `crates/prw-agent/src/remote_session_capability_runtime.rs`
  - `6b9b7bfa2445e3cbc7e713b598f67f7ec6115e8f`
- `crates/prw-remote-bridge/src/capability_request_wire.rs`
  - `4a24af6316e2c17c0980c12e787791848174be9b`
- `crates/prw-remote-bridge/src/requester_rendezvous_target_request_io.rs`
  - `b86dfb8ebea963693eaa9a5107b91e919c21f9a6`
- `crates/prw-remote-transport/src/runtime.rs`
  - `d03bcf642aeb2576656437a8b3d2ddf148a50e30`

Only `post_auth_control_stream_ingress.rs` may change among these guards in FF. All other listed guards must remain byte-stable.

## 4. Materialized ownership surface

FF extends `PostAuthRequesterRendezvousTransaction` with one public asynchronous consuming method:

`send_dr_acknowledgement_frame(self, acknowledgement_frame: &ControlFrame)`

The receiver is `self`, not `&self` or `&mut self`.

This preserves FE's sole-custody law:

- the exact strict requester request and exact retained `MeshControlStream` enter the method under one bridge-owned transaction;
- the transaction is destructured internally;
- only the retained stream is used for the lower send;
- the transaction is not returned;
- raw stream custody is not transferred into Agent code;
- no second transaction envelope is fabricated.

## 5. Exactly-one write law

The method delegates exactly once to the existing lower:

`MeshControlStream::send_frame(...)`

The existing lower operation:

- encodes exactly one bounded `ControlFrame`;
- performs one bounded write operation under the existing timeout;
- finishes the QUIC send direction;
- returns the existing `MeshQuicRuntimeError` on timeout, write failure, finish failure, or lower transport validation failure.

FF adds no loop around that call.

FF adds no:

- retry;
- resend;
- fallback;
- replacement stream;
- second write;
- second acknowledgement;
- speculative send;
- alternate transport path.

## 6. Already-constructed frame law

The FF method accepts one borrowed `ControlFrame` that must already have been produced by the existing C03e-FD requester/rendezvous DR acknowledgement framing boundary in the later caller composition.

FF does not:

- call the DR projector;
- inspect `Result<(), E>`;
- choose accepted versus rejected;
- allocate a new request ID;
- rewrite request correlation;
- decode/revalidate the acknowledgement;
- reconstruct PRWZ bytes;
- create a second `ControlFrame`;
- reinterpret identity.

The bridge send primitive is deliberately provenance-neutral with respect to the supplied already-constructed frame. Provenance composition remains a later Agent-owned gate.

## 7. Requester-specific response-I/O error

FF materializes:

`RequesterRendezvousDrAcknowledgementResponseIoError`

with the sole current variant:

`Runtime(MeshQuicRuntimeError)`

The type is requester/rendezvous-specific and does not reuse `CapabilityRequestWireError` as its public semantic family.

The lower exact `MeshQuicRuntimeError` remains available as the error source.

No lower failure is translated into:

- semantic `Rejected`;
- semantic `Accepted`;
- retry permission;
- fabricated delivery success;
- whole-peer close;
- requester registration rollback;
- DR rerun.

## 8. Failure-layer preservation

FF preserves the four FE-selected layers:

1. exact completed DR semantic result;
2. C03e-FD accepted/rejected requester-visible projection;
3. C03e-FD local frame-construction failure;
4. C03e-FF response write/finish failure.

FF implements only layer 4.

In particular:

- a DR `Err(_)` is not an FF error;
- a C03e-FD frame construction error means FF is never invoked;
- an FF I/O error does not become semantic rejection;
- an FF I/O error does not rerun DR;
- an FF I/O error does not repeat requester registration mutation.

## 9. Custody termination law

Calling the FF method consumes the exact `PostAuthRequesterRendezvousTransaction`.

On successful lower send, transaction custody terminates.

On lower timeout/write/finish failure, transaction custody also terminates because the method still consumed `self` and returns only the requester-specific error.

FF intentionally provides no API that returns:

- the transaction;
- the raw stream;
- the strict request;
- a retry token;
- a replacement stream handle.

This makes retry impossible through the FF surface without a separate new source boundary.

## 10. Send-direction completion semantics

FF inherits the exact existing lower `MeshControlStream::send_frame(...)` semantics.

Successful FF return proves only that the bounded response frame write succeeded and the local QUIC send direction was successfully finished.

It does not prove:

- peer application consumed the acknowledgement;
- requester accepted the acknowledgement semantically;
- target online;
- candidate available or selected;
- target reachable;
- endpoint resolved;
- relay selected;
- direct path available;
- QUIC/TCP path to target established;
- port-forward established;
- terminal established;
- remote session established;
- rendezvous complete;
- end-to-end success.

## 11. Identity and correlation invariants

FF changes no identity law.

- requester logical identity remains the authenticated PRW application-session lineage;
- target logical identity remains the exact nominated `DeviceId`;
- dynamic IP/port remain transient endpoint data;
- `TransportIdentity` remains lower transport evidence only;
- PRWM `request_id` remains correlation only.

The FF method does not read identity from transport metadata and does not interpret the acknowledgement frame's request ID as identity.

## 12. Existing capability precedent

The existing `PostAuthCapabilityTransaction::send_response_frame(...)` remains a useful same-stream consuming-send precedent.

FF does not reuse capability-specific failure naming because FE explicitly selected a requester-specific response-I/O classification.

FF does not modify capability response behavior.

## 13. Existing requester receive precedent

The existing requester-specific receive adapter remains unchanged.

FF does not add a second receive operation and does not reuse the receive-error type for response write failure.

Receive/decode failure and terminal response-send failure remain distinct transaction stages.

## 14. Repeated-ingress barrier

The repeated-ingress barrier remains in force after FF source materialization.

FF does not authorize or implement:

- second `accept_control_stream()`;
- second `receive_frame()`;
- mixed-family loop resume;
- speculative pre-accept;
- concurrent requester transaction;
- queue/channel/task creation;
- fairness/backpressure policy.

The new send surface remains isolated and uninvoked by the repeated ingress worker.

## 15. Peer lifecycle non-selection

FF does not close the retained authenticated peer on write failure or success.

It does not reuse capability code-3 or code-4 close diagnostics.

It does not invent requester-specific close codes or reasons.

Higher-level peer lifecycle policy remains separately gated.

## 16. Agent-source non-mutation

FF must not modify Agent Rust source.

In particular:

- retained-custody DR continuation remains byte-stable;
- ET/EV/EX requester handoff/loop remains byte-stable;
- Agent runtime error carrier remains byte-stable;
- no Agent error variant is added for FF;
- no Agent method calls the FF send surface;
- no DR result is consumed by FF in this checkpoint.

Agent terminal response composition remains the next separate decision/materialization boundary.

## 17. FD codec byte stability

The C03e-FD pure requester/rendezvous acknowledgement codec must remain byte-stable.

FF does not modify:

- accepted/rejected operation numbers;
- outer PRWM kinds;
- PRWZ magic/version;
- flags;
- payload size;
- correlation handling;
- pure projector;
- pure frame encoder/decoder.

Any need to alter FD bytes is a contradiction requiring a new explicit gate.

## 18. Dependency and module stability

FF adds no crate dependency.

FF adds no new root module declaration.

The send surface and requester-specific response-I/O type live beside the exact stream custody owner in `post_auth_control_stream_ingress.rs`.

This keeps lower `MeshControlStream` ownership bridge-local and avoids exposing a parallel public raw-stream send helper.

## 19. Tests

FF source tests must at minimum prove at compile/source level that:

- the requester transaction exposes the new consuming send method;
- the requester-specific response-I/O type converts the exact existing `MeshQuicRuntimeError` without translation.

Existing family classification and custody-transfer tests must remain passing.

No test should require runtime/listener activation or widen network behavior.

## 20. Exact source scope

Intended FF changed paths:

1. this FF contract;
2. `crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`.

No other path should change unless exact-head validation identifies a concrete contradiction.

Any format-only correction required by rustfmt is permitted only inside the intended changed Rust path and must not widen semantics.

## 21. Explicit non-goals

C03e-FF does not materialize or authorize:

- Agent retained-result -> FD frame -> FF send composition;
- DR rerun;
- requester registration retry;
- response retry/resend;
- replacement stream;
- second acknowledgement;
- second accept/read/frame receive;
- repeated-ingress or mixed-family loop resume;
- peer-close policy;
- candidate selection;
- reachability selection;
- endpoint selection;
- relay selection;
- new QUIC/TCP dialing;
- port-forward activation;
- terminal activation;
- remote-session activation;
- runtime/listener/bootstrap activation;
- Android behavior changes;
- dependency/workflow widening;
- packaging;
- deployment;
- restart;
- recovery;
- merge.

## 22. Validation contract

Closure requires exact-final-head validation.

Rust validation must pass on the exact final FF head:

- checkout;
- native prerequisites;
- toolchain record;
- locked dependency graph;
- rustfmt;
- Clippy with warnings denied;
- workspace tests;
- workspace build.

Android validation is claimed only if an Android workflow actually triggers and passes on the exact final FF head.

Closure also requires:

- exact FE merge base;
- ahead only by the intended FF source-materialization commit;
- behind zero;
- exact changed-path scope;
- byte-stability of every non-mutated guard;
- durable Drive audit recording exact head/tree/blobs/workflow evidence;
- raw Drive byte-exact readback.

## 23. Canonical closure target

Intended closure marker:

`CLOSED_REQUESTER_RENDEZVOUS_DR_ACKNOWLEDGEMENT_CONSUMING_SEND_SOURCE_MATERIALIZATION`

Intended gate marker:

`C03E_FF_REQUESTER_RENDEZVOUS_DR_ACKNOWLEDGEMENT_CONSUMING_SEND_SOURCE_MATERIALIZED`

## 24. Next separately gated seam

If FF closes cleanly, the next checkpoint should audit/select the Agent-owned terminal response composition that:

1. consumes the exact retained FB DR continuation;
2. borrows its exact transaction and DR result only long enough to call the existing FD pure frame materializer;
3. if frame construction succeeds, consumes the exact requester transaction through the new FF send surface;
4. preserves internal DR provenance separately from local frame-construction and response-I/O failure;
5. resolves exact requester response custody before any later repeated-ingress resume is considered.

That next gate must remain separate from loop resume, peer-close policy, candidate/reachability continuation, dialing, runtime activation, deployment, and merge.
