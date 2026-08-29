# Phase 152 C03e-FI — Requester/Rendezvous Post-Terminal Response Lifecycle Selection (Staging)

Status: SEMANTICS_SELECTION_STAGING

## 1. Purpose

C03e-FI selects only the Agent-owned lifecycle law that applies after one exact C03e-FH requester/rendezvous terminal DR acknowledgement response composition resolves the retained requester response-stream custody.

FI is docs-only. It performs no repeated-ingress source mutation, invokes no live requester/rendezvous path, changes no Rust/Android/dependency/workflow source, closes no peer, retries no request or response, selects no candidate/reachability/endpoint/relay state, dials no traffic, activates no runtime/listener/bootstrap path, deploys nothing, and merges nothing.

## 2. Exact predecessor

Canonical predecessor:

- branch: `phase-152-c03e-fh-requester-rendezvous-terminal-dr-acknowledgement-response-composition-source-materialization-staging`
- head: `2b3045b999402fdfc57cfe09f344d02c89a0b1af`
- tree: `e2cbffb0097850b357942e739beac9fc6e1a28fe`
- FH contract blob: `ef3218f1bd1695a001cb12450ef0abbe6c8c6707`
- FH Agent retained-custody/terminal-composition blob: `29073f1b9129001f1644f977469c50f0a97bd917`

FI must remain an exact docs-only descendant of that head.

## 3. Exact audited source guards

The FI selection is bounded by these exact FH-head blobs:

- FH retained-custody DR continuation + terminal response composition:
  - `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
  - blob `29073f1b9129001f1644f977469c50f0a97bd917`
- EV/EX requester-aware one-transaction and repeated-ingress seams:
  - `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`
  - blob `406cbf276c2c62a0bbd902a6ec25b8a0f93ca05c`
- Agent post-auth outcome/error carrier source:
  - `crates/prw-agent/src/remote_session_capability_runtime.rs`
  - blob `6b9b7bfa2445e3cbc7e713b598f67f7ec6115e8f`
- historical capability loop/worker lifecycle precedent:
  - `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`
  - blob `083bf83fd1827f6175c9eb62ff93b40147fa9271`
- FD requester/rendezvous acknowledgement codec:
  - `crates/prw-remote-bridge/src/requester_rendezvous_dr_acknowledgement_wire.rs`
  - blob `71b8cd166b24268b1fd87f8f339f57200f426834`
- FF requester transaction consuming same-stream send:
  - `crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`
  - blob `301d8bfbd57db09ecf5922f579dc146cca151003`

No source guard may change in FI.

## 4. Exact predecessor facts

At exact FH head:

1. C03e-EV processes exactly one accepted authenticated control stream and one bounded frame.
2. Capability-family success returns `AuthenticatedRemoteSessionPostAuthIngressOutcome::CapabilityProcessed` only after existing capability authorization, dispatch and same-stream response I/O succeed.
3. Requester/rendezvous-family success returns one `RequesterRendezvousResponseStreamCustodyHandoff` retaining the exact requester transaction/response stream plus the session-derived start intent.
4. C03e-EX `run_repeated_post_auth_control_stream_ingress(...)` reaches the next iteration only after `CapabilityProcessed`.
5. C03e-EX returns immediately when one requester/rendezvous handoff appears; therefore it accepts no second stream while requester response custody is unresolved.
6. The first C03e-EV transaction failure terminates the C03e-EX repeated loop unchanged; no retry or suppression is performed.
7. The C03e-EX executor-neutral worker returns requester handoff, cancellation, or the exact first EV transaction failure as distinguishable outcomes and deliberately performs no mixed-family peer close.
8. FH consumes one exact retained DR continuation, invokes existing FD acknowledgement framing exactly once, then invokes existing FF same-stream send exactly once when framing succeeds.
9. FH `Ok(())` means only that the exact FD acknowledgement frame was successfully written on the retained requester stream and its send direction finished.
10. FH has exactly two local terminal-composition failure categories: `Frame(...)` and `ResponseIo(...)`.
11. A completed DR `Err(RequesterRendezvousStartCompositionError)` is not an FH lifecycle failure. Existing FD projects it to a valid generic rejected acknowledgement.
12. FH returns no requester transaction, raw stream, retained continuation, retry token, or second response opportunity on any result path.
13. The outer `AuthenticatedRemoteSessionRuntimeOwner` owns the authenticated peer separately from the requester response-stream transaction and remains the authority for accepting any later control stream.
14. Historical capability-only `run_capability_request_loop(...)` closes the peer with code 3 on capability-loop failure, but C03e-EX explicitly did not widen that capability-specific close behavior to mixed-family traffic.
15. Historical capability cancellation uses code 4, but C03e-EX explicitly did not widen that close behavior to mixed-family cancellation.

These facts constrain FI.

## 5. Selected lifecycle principle

C03e-FI selects **transaction-complete resume on terminal response success; fail-stop propagation on terminal response composition failure**.

The selection is deliberately asymmetric:

- a successfully completed requester acknowledgement resolves the requester transaction and permits the same authenticated session owner to return to serial mixed-family ingress;
- a local FH terminal-composition failure stops the repeated mixed-family ingress lifecycle and must be propagated distinctly to the caller;
- neither failure category authorizes automatic whole-peer close in FI.

No result permits retrying the consumed requester response transaction.

## 6. Successful acknowledgement resume law

If exact FH terminal response composition returns `Ok(())`:

1. exact requester response-stream custody is terminally resolved;
2. the exact requester transaction is consumed and must not reappear;
3. the outer authenticated-session peer owner remains available for later stream acceptance;
4. the future combined requester-aware lifecycle may start the next C03e-EV serial iteration on that same authenticated session owner;
5. the next stream acceptance must occur only **after** FH `Ok(())` is observed;
6. no speculative accept, prefetch, queue, concurrent transaction, or second in-flight requester response is permitted;
7. verifier time for a later EV iteration must be freshly sampled according to the existing EV/EX law;
8. fresh current registry/policy evaluation remains required for any later capability request under existing authority semantics.

This resume law applies equally when FH successfully sends:

- the FD accepted acknowledgement derived from exact DR `Ok(())`; or
- the FD generic rejected acknowledgement derived from exact DR `Err(_)`.

Requester-visible rejection therefore completes one requester transaction but does not terminate the authenticated session by itself.

## 7. Success does not authorize candidate/reachability continuation

Successful terminal acknowledgement does not itself authorize any target-side continuation.

FI does not convert FD `Accepted` into:

- candidate query or selection;
- reachability evaluation;
- endpoint or relay selection;
- direct-path attempt;
- target transport establishment;
- port-forward activation;
- terminal activation;
- remote-session establishment;
- rendezvous completion.

Those remain separate gates.

The only FI success consequence is permission to return the **requester-side authenticated control ingress owner** to its existing serial mixed-family acceptance lifecycle after the acknowledgement write has completed.

## 8. Frame-construction failure law

If FH returns `RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError::Frame(...)`:

1. no requester response write occurred;
2. requester transaction/stream custody is nevertheless terminal locally because FH consumed the complete retained continuation by value;
3. the future mixed-family lifecycle must **not** silently resume another EV iteration;
4. the exact FH `Frame(...)` failure must be propagated as a distinct terminal lifecycle failure to the caller;
5. no retry, fallback frame, replacement transaction, replacement stream, second request ID, second acknowledgement, DR rerun, or requester-registration retry is permitted;
6. no automatic peer close is selected;
7. no capability close code may be reused merely because the mixed-family lifecycle stops.

The stop requirement prevents local protocol/framing failure from being suppressed by accepting unrelated later traffic.

## 9. Response-I/O failure law

If FH returns `RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError::ResponseIo(...)`:

1. FD framing already succeeded;
2. FF consumed the exact requester transaction and attempted exactly one bounded same-stream send;
3. no retry-capable response-stream custody remains;
4. the future mixed-family lifecycle must **not** accept another stream after this failure inside the same lifecycle invocation;
5. the exact FH `ResponseIo(...)` failure must be propagated distinctly to the caller;
6. no retry, resend, replacement stream, duplicate acknowledgement, DR rerun, or requester-registration retry is permitted;
7. no automatic whole-peer close is selected by FI;
8. no capability-specific close code or reason may be widened to this requester response-I/O failure.

A higher lifecycle owner may later be given an explicitly selected peer-close policy, but FI does not select one.

## 10. Selected combined lifecycle error shape

A later source-materialization checkpoint should preserve two top-level failure families for the requester-aware serial lifecycle:

1. **Ingress** — preserves the existing `AuthenticatedRemoteSessionPostAuthIngressTransactionError` from C03e-EV/C03e-EX;
2. **RequesterResponse** — preserves the existing FH `RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError`.

The requester-response family already preserves its lower `Frame` versus `ResponseIo` distinction.

No additional error category for the completed DR semantic `Err(_)` is selected because that outcome remains one valid generic rejected acknowledgement.

Exact Rust type and variant names remain source-materialization details.

## 11. Serial orchestration law

A future source seam materializing FI must preserve one strictly serial transaction lifecycle:

1. run the existing repeated EV/EX ingress until either an ingress failure, cancellation boundary, or requester handoff is produced;
2. if an EV/EX ingress failure occurs, propagate the existing ingress failure unchanged under the selected combined lifecycle wrapper and stop;
3. if a requester handoff is produced, run exactly one existing FB retained-custody DR continuation;
4. run exactly one existing FH terminal response composition on that exact continuation;
5. if FH returns `Ok(())`, start the next serial EV/EX ingress cycle on the same authenticated-session owner;
6. if FH returns `Frame(...)` or `ResponseIo(...)`, propagate that exact requester-response failure and stop.

No second EV transaction may overlap the requester DR/response interval.

## 12. Ownership law

FI preserves ownership boundaries exactly:

- `AuthenticatedRemoteSessionRuntimeOwner` continues to own the authenticated peer and later stream-accept authority;
- the requester handoff owns the exact accepted requester stream only until FB/FH continuation consumes it;
- Agent owns completed DR semantic provenance;
- FD owns requester acknowledgement projection/framing;
- FF owns requester same-stream write mechanics;
- no raw `MeshControlStream` is returned to Agent orchestration code;
- no bridge code gains Agent semantic ownership;
- no dependency direction changes.

After FH success, only the **outer authenticated peer owner** is reused for a new stream acceptance. The consumed requester transaction itself is never reused.

## 13. Peer lifecycle selection

FI selects **no automatic authenticated-peer close** for requester terminal response completion or failure.

Specifically:

- FH success does not close the peer;
- FH `Frame(...)` does not close the peer automatically;
- FH `ResponseIo(...)` does not close the peer automatically;
- capability-only code 3 / `remote capability session terminated` is not reused;
- capability-only code 4 / `remote capability session shutdown` is not reused;
- no requester-specific close code or reason is invented.

On requester terminal response failure the FI lifecycle stops and returns control to its caller with the exact typed failure. A later explicit peer-lifecycle checkpoint may decide whether that caller closes, retains, drains, or otherwise disposes of the authenticated peer.

## 14. Cancellation boundary remains separate

FI does not widen or redefine cancellation semantics.

The existing C03e-EX worker cancellation law applies only while the repeated ingress loop future is pending and currently performs no whole-peer close.

FI does not yet select:

- cancellation racing FB DR continuation;
- cancellation racing FH frame construction;
- cancellation racing FF response I/O;
- cancellation after FH success but before the next EV iteration;
- a combined requester-aware worker stop enum;
- cancellation-owned peer close.

Those concerns remain separately gated unless a later source-materialization audit proves they must be composed atomically with FI.

## 15. Error ordering and no suppression

FI requires exact failure ordering:

- existing EV/EX ingress failure wins if it occurs before a requester handoff;
- once requester handoff custody is returned, no later EV ingress exists until requester response lifecycle completes;
- FH frame failure wins before any response write attempt;
- FH response-I/O failure wins only after successful frame construction and exactly one FF send attempt;
- FH success is the only requester-response outcome that permits a later EV iteration.

No failure may be replaced by fabricated success, generic rejection, retry, cancellation, peer-close status, or an unrelated later transaction result.

## 16. Identity and correlation law

FI preserves all established identity/correlation boundaries:

- requester logical identity remains the authenticated PRW application-session lineage;
- target logical identity remains the exact nominated logical `DeviceId`;
- dynamic IP/port remain transient endpoint data;
- `TransportIdentity` remains lower transport evidence only;
- PRWM `request_id` remains correlation only.

Resuming ingress after FH success does not carry requester identity or request correlation into a later independent transaction except through the authenticated session owner and the later request's own strict protocol data.

## 17. Fresh-authority law after resume

FI resume does not cache prior transaction authority.

Any later capability transaction must continue to use the existing fresh shared-current registry/policy evaluation in C03e-EV.

Any later requester/rendezvous transaction must produce its own new strict request, session-derived start intent, requester-aware DR evaluation, registration mutation, acknowledgement framing and response custody lifecycle.

No prior accepted/rejected result, registry read, policy decision, requester registration result, request ID, endpoint, or transport evidence may authorize a later transaction.

## 18. Backpressure and ordering non-selection

FI preserves strict serialization and therefore selects no queueing or concurrency policy.

Not selected:

- parallel accepted streams;
- requester transaction concurrency;
- capability/requester reordering;
- queued requests while requester response is pending;
- speculative accept;
- multi-stream fairness;
- per-family priorities;
- backpressure buffers;
- timeout/retry policy.

A later performance/concurrency checkpoint may consider these only after the serial lifecycle is materialized and validated.

## 19. Runtime activation non-selection

FI does not integrate the selected lifecycle into:

- existing production capability worker ownership;
- persistent worker collections;
- admission loops;
- listener lifecycle;
- process lifecycle control;
- Agent `main.rs`;
- readiness;
- systemd/service management;
- host mutation;
- deployment.

The future source seam remains isolated until separately gated runtime integration.

## 20. Android and protocol non-selection

FI changes no Android behavior and no wire protocol.

Unchanged:

- PRWZ request/acknowledgement format;
- FD accepted/rejected operations;
- outer PRWM kinds;
- request correlation;
- FF write mechanics;
- Android client/application behavior.

No Android application assumption is introduced by FI.

## 21. Source-materialization target

The next source-materialization checkpoint should implement only the FI-selected isolated Agent orchestration needed to re-enter the existing serial mixed-family ingress after successful FH completion and to stop/propagate on existing ingress or FH response failure.

Preferred scope:

- Agent-owned source only;
- existing EV/EX ingress reused, not duplicated;
- existing FB continuation reused exactly once per requester handoff;
- existing FH completion reused exactly once per requester handoff;
- combined lifecycle failure wrapper preserving existing ingress versus requester-response provenance;
- no peer-close behavior;
- no cancellation widening unless exact source constraints prove it unavoidable;
- no active runtime/worker collection/listener integration.

Exact module/function/type names remain source-materialization details.

## 22. Explicit non-goals

C03e-FI does not authorize:

- source implementation;
- second accept before FH success;
- loop resume after FH failure;
- DR retry;
- requester-registration retry;
- response retry/resend;
- replacement stream;
- duplicate acknowledgement;
- peer close;
- close-code reuse or invention;
- cancellation policy widening;
- candidate/reachability/endpoint/relay selection;
- target dialing;
- port-forward or terminal startup;
- runtime/listener/bootstrap activation;
- Android mutation;
- workflow/dependency mutation;
- packaging;
- deployment;
- restart/recovery;
- merge.

## 23. Closure condition

C03e-FI may close only if exact-head verification proves:

1. exact FH ancestry;
2. docs-only net diff;
3. all audited source guards remain byte-stable;
4. successful FH completion alone is selected to permit serial mixed-family ingress resume;
5. FH `Frame` and `ResponseIo` failures are selected as fail-stop propagated lifecycle failures;
6. no requester response retry or custody return is selected;
7. no automatic peer close or capability close-code reuse is selected;
8. candidate/reachability/runtime/deployment boundaries remain closed;
9. validation required by repository policy is successful for the exact FI head;
10. durable immutable audit evidence is recorded.

## 24. Canonical closure and next gate

On successful closure:

`CLOSED_REQUESTER_RENDEZVOUS_POST_TERMINAL_RESPONSE_LIFECYCLE_SELECTION`

Gate:

`C03E_FI_REQUESTER_RENDEZVOUS_POST_TERMINAL_RESPONSE_LIFECYCLE_SELECTED`

Next separately gated checkpoint:

**C03e-FJ — requester/rendezvous post-terminal response serial lifecycle source materialization**.

FJ should materialize only the isolated Agent orchestration selected here. It must not activate runtime ownership, widen cancellation, add peer-close policy, select candidate/reachability behavior, dial traffic, deploy, or merge without separate authorization.
