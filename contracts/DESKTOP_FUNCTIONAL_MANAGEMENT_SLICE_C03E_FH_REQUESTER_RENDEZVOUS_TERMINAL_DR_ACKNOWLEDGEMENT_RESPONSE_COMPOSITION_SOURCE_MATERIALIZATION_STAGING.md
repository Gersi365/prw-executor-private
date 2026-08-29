# Phase 152 C03e-FH — Requester/Rendezvous Terminal DR Acknowledgement Response Composition Source Materialization (Staging)

Status: SOURCE_MATERIALIZATION_STAGING

## 1. Purpose

C03e-FH materializes only the C03e-FG-selected Agent-owned terminal response composition from one exact retained requester/rendezvous DR continuation through the existing C03e-FD pure acknowledgement framing boundary and existing C03e-FF consuming same-stream send surface.

FH does not resume repeated ingress, select peer-close policy, select candidate/reachability/endpoint/relay state, dial target traffic, activate port-forward/terminal/session/runtime/listener/bootstrap behavior, deploy, restart, recover, or merge.

## 2. Exact predecessor

Canonical predecessor:

- branch: `phase-152-c03e-fg-requester-rendezvous-terminal-dr-acknowledgement-response-composition-selection-staging`
- head: `f59a2f02ce35d55a0cd2251f80df97fad34ba8ab`
- tree: `357fa93d18ea24dd9579f30f9facb9b068b2e2e2`
- FG contract blob: `f8e995ce81c63fd1b1f63ed579283a50d89007d3`

FH must remain an exact descendant of that head.

## 3. Exact source guards

FH may modify only the Agent retained-custody DR continuation module plus this contract unless exact-head validation exposes a concrete contradiction.

These predecessor source guards must remain byte-stable:

- C03e-FD acknowledgement codec:
  - `crates/prw-remote-bridge/src/requester_rendezvous_dr_acknowledgement_wire.rs`
  - blob `71b8cd166b24268b1fd87f8f339f57200f426834`
- C03e-FF bridge requester transaction/send:
  - `crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`
  - blob `301d8bfbd57db09ecf5922f579dc146cca151003`
- ET/EV/EX requester path:
  - `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`
  - blob `406cbf276c2c62a0bbd902a6ec25b8a0f93ca05c`
- Agent runtime carrier/error source:
  - `crates/prw-agent/src/remote_session_capability_runtime.rs`
  - blob `6b9b7bfa2445e3cbc7e713b598f67f7ec6115e8f`

The predecessor Agent retained-custody DR continuation blob is:

- `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
- blob `9550148740b654a79eca8e51956bf37a351ac802`

That is the sole intended Rust mutation path.

## 4. Materialized Agent ownership

FH keeps terminal response composition Agent-owned because Agent owns the exact completed DR semantic provenance.

Bridge ownership remains unchanged:

- FD owns pure requester/rendezvous acknowledgement projection/framing;
- FF owns exact same-stream consuming response write custody.

FH introduces no bridge -> Agent dependency and no crate dependency widening.

## 5. Materialized terminal composition function

FH materializes one crate-internal Agent function:

`complete_requester_rendezvous_terminal_dr_acknowledgement_response(...)`

The function:

1. consumes one exact `RequesterRendezvousRetainedCustodyDrContinuation` by value;
2. borrows the exact retained requester transaction and exact completed DR result only long enough to invoke existing FD `encode_requester_rendezvous_dr_result_for_transaction(...)` exactly once;
3. if framing succeeds, consumes the continuation through the existing `into_parts()` custody transfer;
4. transfers the exact requester transaction into existing FF `send_dr_acknowledgement_frame(...)` exactly once;
5. returns `Ok(())` only if the exact FF send succeeds.

No replacement transaction or stream is created.

## 6. DR semantic law

FH does not reinterpret DR semantics.

Exact existing FD projection remains authoritative:

- exact DR `Ok(())` -> requester-visible `Accepted` acknowledgement;
- every exact DR `Err(RequesterRendezvousStartCompositionError)` -> one generic requester-visible `Rejected` acknowledgement.

A DR `Err(_)` is not an FH composition failure and does not enter the FH error family.

FH does not inspect, format, serialize or leak internal DR error detail.

FH does not rerun DR and does not repeat requester registration mutation.

## 7. Materialized two-category error family

FH materializes one Agent-local error enum:

`RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError`

It contains exactly two variants:

1. `Frame(RequesterRendezvousDrAcknowledgementWireError)`
2. `ResponseIo(RequesterRendezvousDrAcknowledgementResponseIoError)`

Both lower errors remain available through `std::error::Error::source()`.

FH adds `From` conversions for the exact two lower error types.

No third DR-semantic error variant exists.

## 8. Frame-construction failure custody

If FD framing fails:

- FH returns `Frame(...)`;
- FF is not invoked;
- no response write is attempted;
- no fallback frame is built;
- no generic rejection is fabricated to replace local framing failure;
- no second request ID is allocated;
- no DR or registration mutation is retried;
- because FH owns the complete continuation by value, no retry-capable continuation, requester transaction or raw stream custody is returned.

Dropping consumed local transaction custody is not whole-peer close authority.

## 9. Response-I/O failure custody

If FD framing succeeds and FF send fails:

- FH returns `ResponseIo(...)`;
- the error remains distinct from DR semantics and framing;
- FF has already consumed exact requester transaction custody by value;
- no retry, resend, replacement stream or duplicate acknowledgement occurs;
- no DR or registration mutation is repeated;
- no retry-capable custody is returned.

FH selects no automatic authenticated-peer close.

## 10. Success meaning

FH `Ok(())` proves only that:

- the exact already-completed DR result was projected by unchanged FD semantics;
- one exact bounded acknowledgement frame carrying original PRWM correlation was constructed;
- that exact frame was written once through FF on the exact retained same stream; and
- the send direction finished successfully.

FH success does not prove:

- target online;
- candidate availability or selection;
- reachability success;
- endpoint or relay selection;
- target transport establishment;
- port-forward establishment;
- terminal establishment;
- remote-session establishment;
- rendezvous completion;
- end-to-end operation success; or
- independently confirmed requester application consumption.

## 11. Identity and correlation

FH preserves all existing identity/correlation laws:

- PRWM `request_id` is correlation only;
- requester logical identity remains authenticated PRW application-session lineage;
- target logical identity remains exact nominated `DeviceId`;
- dynamic IP/port remain transient endpoint data;
- `TransportIdentity` remains lower transport evidence only.

FH does not reconstruct identity from stream metadata or request ordering.

## 12. Repeated-ingress barrier

FH does not authorize or implement:

- second `accept_control_stream()`;
- second `receive_frame()`;
- mixed-family/capability-loop resume;
- speculative pre-accept;
- concurrent requester transaction;
- queue/channel/task creation;
- fairness/backpressure policy.

The repeated-ingress barrier remains a separately gated lifecycle decision after terminal response custody resolution.

## 13. Peer lifecycle non-selection

FH does not:

- close the authenticated peer on success;
- close the authenticated peer on frame failure;
- close the authenticated peer on response-I/O failure;
- reuse capability close codes;
- invent requester-specific close codes or reasons.

Higher-level peer lifecycle remains separately gated.

## 14. Candidate/reachability non-selection

FH does not authorize candidate query/selection, reachability evaluation, endpoint resolution, relay selection, target dialing, target transport startup, port-forward, terminal, remote-session or rendezvous continuation.

FD accepted remains accepted-for-continuation only.

## 15. Source placement and visibility

The composition and its error family live beside the existing retained-custody DR continuation in:

`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

The existing parent module already carries a bounded `dead_code` allowance for this separately gated requester/rendezvous continuation module. FH therefore requires no parent-module mutation or new export.

The FH composition remains crate-internal and uninvoked by active runtime ownership.

## 16. Tests

FH adds narrow source/compile tests proving:

- the terminal composition function surface exists;
- the Agent-local error family accepts exact FD frame error conversion;
- the Agent-local error family accepts exact FF response-I/O error conversion.

No test activates listeners, network runtime, repeated ingress, candidate selection or dialing.

Existing FD and FF tests remain authoritative for framing semantics and same-stream send ownership.

## 17. Intended changed paths

Exactly two paths are intended:

1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_FH_REQUESTER_RENDEZVOUS_TERMINAL_DR_ACKNOWLEDGEMENT_RESPONSE_COMPOSITION_SOURCE_MATERIALIZATION_STAGING.md`
2. `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

No `Cargo.toml`, `Cargo.lock`, workflow, Android, bridge, transport, Agent runtime parent, packaging or deployment path should change.

A format-only correction inside the intended Agent Rust path is permitted only if exact-head rustfmt requires it and may not widen semantics.

## 18. Validation contract

Closure requires exact-final-head validation.

Rust validation must pass:

- checkout;
- native prerequisites;
- toolchain record;
- locked dependency graph;
- rustfmt;
- Clippy with warnings denied;
- workspace tests;
- workspace build.

Android validation may be claimed only if an Android workflow actually triggers and passes on the exact final FH head.

Closure also requires:

- exact FG merge base;
- ahead only by intended FH materialization commit(s);
- exact intended changed-path scope;
- FD and FF source guards byte-stable;
- immutable Drive audit with raw byte-exact readback;
- PR left draft/open/unmerged with semantic `Status: CLOSED`.

## 19. Explicit non-goals

C03e-FH does not materialize or authorize:

- repeated-ingress loop resume;
- second accept/read/frame receive;
- peer-close policy;
- DR retry;
- requester-registration retry;
- response retry/resend;
- replacement stream;
- duplicate acknowledgement;
- candidate selection;
- reachability selection;
- endpoint selection;
- relay selection;
- target QUIC/TCP dialing;
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

## 20. Closure marker

On exact-head validation and durable evidence completion, canonical closure is:

`CLOSED_REQUESTER_RENDEZVOUS_TERMINAL_DR_ACKNOWLEDGEMENT_RESPONSE_COMPOSITION_SOURCE_MATERIALIZATION`

Canonical gate:

`C03E_FH_REQUESTER_RENDEZVOUS_TERMINAL_DR_ACKNOWLEDGEMENT_RESPONSE_COMPOSITION_SOURCE_MATERIALIZED`

## 21. Next separately gated seam

After FH closure, the next checkpoint may audit/select only the lifecycle transition after exact terminal requester response custody has been resolved.

That later gate must decide whether and under which success/failure outcomes the repeated post-authenticated mixed-family ingress loop may resume, remain stopped, or trigger a separately selected peer lifecycle action.

Candidate/reachability continuation, target dialing, runtime activation, deployment and merge remain separate from that lifecycle gate.
