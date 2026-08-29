# Phase 152 C03e-FE — Requester/Rendezvous Same-Stream DR Acknowledgement Write Custody Selection (Staging)

Status: SEMANTICS_SELECTION_STAGING

## 1. Purpose

C03e-FE selects only the ownership, one-write, custody-termination, and failure-boundary semantics for a later separately gated requester/rendezvous DR acknowledgement write on the exact same control stream retained by C03e-EZ/FB and framed purely by C03e-FD.

FE is docs-only. It performs no response write, does not consume the retained requester transaction, does not modify the FD acknowledgement codec, does not resume repeated ingress, and does not activate candidate/reachability/endpoint/relay/dialing/runtime/deployment behavior.

## 2. Exact predecessor

Canonical predecessor:

- branch: `phase-152-c03e-fd-requester-rendezvous-dr-acknowledgement-projection-framing-source-materialization-staging`
- head: `9d38f1d2ab09dbc29bdd7f60fbc8d95b317a7902`
- tree: `ecd79945c518a0f05215636f037f3694a4ed1ac5`
- FD acknowledgement codec blob: `71b8cd166b24268b1fd87f8f339f57200f426834`
- FD contract blob: `73b0c459a3e340b8d99ebeca7ec5f1b3535c6a9a`

FE must remain an exact docs-only descendant of that head.

## 3. Audited exact-source guards

The FE selection is bounded by these exact FD-head blobs:

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

All listed source must remain byte-stable in FE.

## 4. Audited predecessor facts

At the exact FD head:

- `PostAuthRequesterRendezvousTransaction` owns by value the exact strict requester/rendezvous request plus the exact already-accepted `MeshControlStream`;
- that requester transaction currently exposes only `request()` and consuming `into_parts()` and performs no response I/O;
- the capability-family sibling `PostAuthCapabilityTransaction` already establishes the architectural precedent of a transaction-consuming `send_response_frame(...)` operation that writes exactly one already-constructed response on its retained same stream;
- FD already constructs one requester-specific bounded `ControlFrame` from the exact transaction correlation and exact completed DR result without consuming or writing the transaction;
- FD `Ok(())` projects to requester-visible accepted-for-continuation and every DR `Err(_)` projects to one generic requester-visible rejected acknowledgement;
- `MeshControlStream::send_frame(...)` writes exactly one bounded PRWM frame and then finishes the QUIC send direction;
- `send_frame(...)` preserves timeout/write/finish failures as existing `MeshQuicRuntimeError` classes;
- the isolated repeated post-auth ingress loop stops at the requester/rendezvous handoff barrier and does not accept another control stream after that handoff;
- existing capability-family success may continue its loop only after its same-stream response has completed.

These facts determine FE.

## 5. Selected write owner

C03e-FE selects the bridge-owned `PostAuthRequesterRendezvousTransaction` as the sole owner of the future same-stream requester/rendezvous acknowledgement write.

The later source boundary must not extract the raw `MeshControlStream` into Agent code merely to perform the write. The transaction already owns the exact stream whose request produced the correlation used by FD; keeping the write on that transaction preserves single-owner same-stream custody and keeps lower transport I/O inside `prw-remote-bridge`.

A later source-materialization checkpoint may add one requester-specific consuming send surface on that transaction or an equivalently narrow bridge-owned operation. Exact Rust symbol names remain implementation details for that later gate.

## 6. Already-constructed-frame law

The selected write boundary receives one already-materialized bounded `ControlFrame`.

It must not:

- re-project the DR result;
- reconstruct accepted/rejected semantics;
- allocate a second request ID;
- rewrite correlation;
- decode and re-encode the FD acknowledgement;
- invent candidate/reachability/session fields; or
- reuse candidate-publication result framing.

FD remains the sole selected requester/rendezvous DR acknowledgement projection/framing source. FE selects only transport write ownership for that already-constructed frame.

## 7. Exact one-write-attempt law

The future requester transaction send boundary must perform exactly one call-equivalent bounded frame write on the retained same `MeshControlStream`.

There is no selected:

- second response frame;
- retry;
- resend;
- fallback stream;
- replacement stream;
- alternate peer connection;
- write queue; or
- duplicated stream custody.

The acknowledgement write is terminal for that exact requester transaction.

## 8. Consuming custody law

The future send operation must consume the exact `PostAuthRequesterRendezvousTransaction` by value.

This is deliberate terminal custody transfer:

- the exact request correlation and exact same stream cannot remain simultaneously usable by the caller after the write attempt begins;
- a successful write ends local transaction custody after the underlying `send_frame(...)` has written the one frame and finished the send direction;
- a write/finish/timeout failure also ends local transaction custody when the consuming operation returns failure;
- no failed write returns the transaction for retry;
- no stream clone or second owner is created.

FE selects no explicit whole-peer close as part of this operation. Local transaction/stream custody termination is not permission to close the authenticated peer.

## 9. Success law

A successful future same-stream send means only:

- exactly one already-constructed FD acknowledgement frame was accepted by the existing bounded transport write path; and
- the QUIC send direction was successfully finished by the existing `MeshControlStream::send_frame(...)` behavior.

Write success does not strengthen the meaning of FD accepted. In particular it does not mean:

- target online;
- candidate available or selected;
- target reachable;
- endpoint or relay selected;
- transport path established;
- port-forward/terminal/session established;
- rendezvous complete; or
- peer application consumption of the acknowledgement was independently confirmed.

## 10. Failure-boundary selection

FE preserves four conceptual layers:

1. exact completed DR semantic result;
2. FD coarse accepted/rejected projection;
3. FD local frame-construction failure; and
4. future same-stream response write/finish failure.

These layers must remain distinguishable.

In particular:

- DR `Err(_)` is not a response-write failure; it becomes one valid generic rejected frame before I/O;
- FD frame-construction failure causes no response write attempt;
- response write/finish failure must not be reclassified as semantic rejected;
- response write/finish failure must not fabricate success;
- no write failure may re-run DR or repeat requester registration mutation.

The later write source should preserve the existing lower `MeshQuicRuntimeError` as the source of the local I/O failure while exposing a requester-specific bridge failure classification rather than coupling requester/rendezvous to the capability-family `CapabilityRequestWireError` name/semantics.

Exact future error type and variant names remain source-materialization details.

## 11. Capability precedent — behavior only

The existing capability transaction is precedent for ownership shape only:

- transaction owns same stream;
- send operation consumes transaction;
- caller supplies one already-constructed frame;
- exactly one lower bounded write is attempted.

FE does not select requester/rendezvous reuse of capability protocol semantics, capability error naming, capability authorization, capability dispatcher behavior, or capability loop lifecycle.

## 12. Requester-specific I/O ownership

Requester/rendezvous already uses requester-specific bridge-owned receive/error boundaries elsewhere. FE therefore keeps the future acknowledgement send classification requester-specific as well.

This avoids turning `CapabilityRequestWireError` into a cross-family umbrella merely because the lower `MeshControlStream` mechanism is shared.

No dependency widening is selected.

## 13. Correlation and identity law

The frame supplied to the future consuming write operation must already carry the exact FD-preserved original PRWM `request_id`.

That value remains correlation only. It is never requester identity, target identity, authenticated-session identity, transport identity, authorization evidence, or rendezvous ownership evidence.

Logical requester identity remains the authenticated PRW session lineage. Logical target identity remains the exact nominated `DeviceId`. Dynamic IP/port and `TransportIdentity` remain lower transport evidence only.

## 14. Repeated-ingress barrier

FE does not lift the existing repeated-ingress barrier.

No second control-stream accept/read, second frame receive, or mixed-family loop resume is authorized by this docs-only selection or by a future isolated write adapter alone.

The barrier may be reconsidered only after a separately gated composition proves that the exact retained requester transaction has reached terminal response custody resolution and explicitly selects subsequent loop behavior.

## 15. No peer-close policy

FE selects no requester-specific whole-peer close code, close reason, automatic connection teardown, or capability close-code reuse.

A local acknowledgement write failure remains a typed local transaction failure. Whether a higher lifecycle later closes the authenticated peer is a separate policy gate.

## 16. No retry/recovery policy

FE explicitly selects fail-closed one-attempt behavior for the exact requester response transaction.

There is no retry, recovery stream, replacement transaction, duplicate acknowledgement, replay, delayed resend, or fabricated delivery success.

A future consuming send failure terminates that local transaction custody and returns its typed failure to the caller.

## 17. Source-stability guard

C03e-FE is docs-only.

All Rust, Android, dependency, workflow, protocol implementation, and runtime source must remain byte-stable. In particular every exact guard in section 3 must remain unchanged.

Any incompatibility discovered by validation is a contradiction requiring a new explicit decision; it is not permission to widen FE into implementation.

## 18. Explicit non-goals

C03e-FE does not materialize or authorize:

- requester transaction send Rust source;
- requester-specific response-I/O error Rust source;
- Agent composition from FB retained DR result through FD framing into the future send operation;
- response retry/resend;
- second acknowledgement frame;
- second accept/read or second frame receive;
- mixed-family loop resume;
- peer-close policy;
- capability error-family reuse;
- candidate/reachability/endpoint/relay selection;
- QUIC/TCP dialing beyond the already-existing response stream I/O;
- port-forward activation;
- terminal activation;
- remote-session activation;
- runtime/listener/bootstrap activation;
- Android behavior mutation;
- dependency/workflow widening;
- packaging;
- deployment;
- restart;
- recovery; or
- merge.

## 19. Next-gate recommendation

If FE closes cleanly, the next separately gated checkpoint may materialize only the selected bridge-owned requester transaction consuming send surface and requester-specific local response-I/O failure classification.

That source gate should keep the FD acknowledgement codec byte-stable and remain uninvoked by Agent runtime composition unless its own contract explicitly proves that composition belongs in the same narrow step. Preferred decomposition keeps the bridge write primitive isolated first, then separately selects/materializes the Agent terminal response composition and only afterward considers any repeated-ingress resume policy.

## 20. Validation contract

Closure requires exact-final-head validation.

Because FE is docs-only:

- source byte-stability must be verified against every section-3 guard;
- branch ancestry must show exact FD merge base, ahead only by the FE docs commit(s), behind zero;
- Rust validation is closure evidence only if actually triggered on the exact final FE head and must pass its complete required job;
- Android validation is closure evidence only if actually triggered on the exact final FE head and must pass its complete required job;
- expected workflow skips must not be misreported as failures; and
- durable audit evidence must record final head/tree, exact changed paths, source guards, CI evidence actually observed, closure marker, and gate marker.

## 21. Canonical closure target

Intended closure marker:

`CLOSED_REQUESTER_RENDEZVOUS_SAME_STREAM_DR_ACKNOWLEDGEMENT_WRITE_CUSTODY_SELECTION`

Intended gate marker:

`C03E_FE_REQUESTER_RENDEZVOUS_SAME_STREAM_DR_ACKNOWLEDGEMENT_WRITE_CUSTODY_SELECTED`
