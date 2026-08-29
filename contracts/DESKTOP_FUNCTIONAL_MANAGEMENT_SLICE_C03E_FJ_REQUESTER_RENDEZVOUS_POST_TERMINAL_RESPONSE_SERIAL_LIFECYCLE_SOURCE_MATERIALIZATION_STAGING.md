# Phase 152 C03e-FJ — Requester/Rendezvous Post-Terminal Response Serial Lifecycle Source Materialization (Staging)

Status: SOURCE_MATERIALIZATION_STAGING

## 1. Purpose

C03e-FJ materializes only the C03e-FI-selected isolated Agent-owned serial lifecycle after requester/rendezvous terminal response custody resolution.

The source seam reuses the already-materialized EV/EX mixed-family ingress, FB retained-custody DR continuation, and FH terminal requester acknowledgement composition. It resumes the next serial mixed-family ingress cycle only after FH returns success. Existing ingress failure or FH requester-response failure stops the isolated lifecycle and propagates a typed failure.

FJ does not integrate this seam into the active runtime, worker collection, listener lifecycle, process lifecycle, Agent binary, readiness path, deployment, restart/recovery, or merge. It selects no candidate/reachability/endpoint/relay continuation and performs no target dialing.

## 2. Exact predecessor

Canonical predecessor:

- branch: `phase-152-c03e-fi-requester-rendezvous-post-terminal-response-lifecycle-selection-staging`
- head: `781ae57b84fe650a5a89d400d1fc2b29abaa66aa`
- tree: `c3fe1e44fde1417ace7208c547a7e9741dda1090`
- FI contract blob: `804b004eeca1cfe2689a0c018c4cbf885c09b9e8`

FJ must remain an exact descendant of that head.

## 3. Selected source location

FJ extends only the existing Agent requester/rendezvous retained-custody lifecycle module:

`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

This module already owns the FB continuation and FH terminal response composition. Keeping FJ there avoids:

- a second stream acceptor;
- a new bridge helper;
- a parent-module export widening;
- a new dependency;
- a raw `MeshControlStream` escape into Agent;
- duplicated DR/framing/send logic.

## 4. Existing source reused unchanged

FJ must reuse the exact existing behavior of:

### EV/EX mixed-family ingress

`AuthenticatedRemoteSessionRuntimeOwner::run_repeated_post_auth_control_stream_ingress(...)`

Existing law:

- exactly one EV transaction is in flight per iteration;
- capability success alone continues internally to the next EV iteration;
- requester/rendezvous ingress returns one exact requester response-stream custody handoff;
- the first ingress failure returns unchanged;
- requester handoff occurs before any second stream accept/read;
- no requester response is constructed or sent inside EV/EX.

### FB retained-custody DR continuation

`continue_requester_rendezvous_retained_custody_through_dr(...)`

Existing law:

- consumes one exact requester handoff;
- uses current registry authority once;
- uses the requester-aware policy source;
- performs the existing DR validation/authorization/registration composition exactly once;
- retains exact requester transaction custody on both DR `Ok(())` and DR `Err(...)`;
- returns the exact completed DR result without flattening semantics.

### FH terminal response composition

`complete_requester_rendezvous_terminal_dr_acknowledgement_response(...)`

Existing law:

- consumes one exact FB continuation by value;
- invokes existing FD framing exactly once;
- exact DR `Ok(())` maps to accepted acknowledgement;
- exact DR `Err(_)` maps to generic rejected acknowledgement and is not an FH composition failure;
- successful framing invokes existing FF same-stream send exactly once;
- no result path returns retry-capable requester transaction/stream custody;
- errors remain exactly `Frame(...)` or `ResponseIo(...)`.

## 5. Materialized FJ lifecycle error family

FJ materializes one Agent-local two-family error boundary:

`RequesterRendezvousPostTerminalResponseSerialLifecycleError`

with exactly:

1. `Ingress(AuthenticatedRemoteSessionPostAuthIngressTransactionError)`
2. `RequesterResponse(RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError)`

The exact lower errors remain available through `std::error::Error::source()`.

FJ adds no DR-semantic error category. A completed DR `Err(_)` remains a valid generic rejected requester acknowledgement through existing FD/FH semantics.

## 6. Materialized serial orchestration

FJ materializes:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle(...)`

The function owns no peer and creates no task. It borrows the existing authenticated-session owner and repeatedly performs exactly this serial sequence:

1. invoke existing EX mixed-family ingress using the existing authenticated-session owner;
2. if EX returns ingress failure, map it to `Ingress(...)` and return immediately;
3. if EX returns one requester/rendezvous handoff, invoke existing FB exactly once;
4. pass the resulting exact FB continuation into existing FH exactly once;
5. if FH returns `Frame(...)` or `ResponseIo(...)`, map it to `RequesterResponse(...)` and return immediately;
6. only after FH returns `Ok(())`, begin the next EX cycle.

There is no overlap between requester DR/response custody and a later EV/EX accept/read cycle.

## 7. Verifier-time law

The caller-supplied verifier-time function remains one mutable source across the isolated FJ lifecycle.

FJ passes a fresh mutable reborrow into each existing EX cycle. EX remains authoritative for sampling verifier time exactly once immediately before each EV transaction.

FJ does not cache, replay, precompute, synthesize, or reinterpret verifier time.

## 8. Success resume law

FH `Ok(())` is the only requester branch that reaches another EX cycle.

FH success means only:

- existing FD framing succeeded;
- existing FF sent one exact acknowledgement on the retained requester stream; and
- the stream send direction finished successfully.

After that terminal requester transaction completes, the outer authenticated-session owner may accept later traffic serially.

This law applies equally when the successfully sent acknowledgement was:

- accepted, from DR `Ok(())`; or
- generic rejected, from DR `Err(_)`.

A generic requester rejection therefore completes that control transaction but does not by itself terminate the authenticated session.

## 9. Ingress fail-stop law

If existing EV/EX returns `AuthenticatedRemoteSessionPostAuthIngressTransactionError`:

- FJ immediately returns `Ingress(...)`;
- no FB continuation runs;
- no FH response composition runs;
- no next EX cycle begins;
- no retry, suppression, replacement stream, or fabricated requester response occurs;
- no automatic whole-peer close is added.

FJ does not widen the historical capability-only code-3 close behavior into mixed-family ingress.

## 10. Requester-response fail-stop law

If FH returns either requester terminal response failure:

- FJ immediately returns `RequesterResponse(...)`;
- no next EX cycle begins;
- no second accept/read occurs inside this lifecycle invocation;
- no retry, resend, replacement stream, duplicate acknowledgement, DR rerun, or requester-registration retry occurs;
- no automatic whole-peer close is added.

The exact FH distinction between `Frame(...)` and `ResponseIo(...)` remains preserved inside the lower source error.

## 11. No speculative resume

FJ explicitly forbids:

- accepting the next control stream while FB is running;
- accepting the next control stream while FH framing is running;
- accepting the next control stream while FF response I/O is running;
- queueing or prefetching a later stream;
- concurrent EV transactions;
- multiple requester transactions in flight;
- a second response opportunity for the same requester request.

Resume is strictly post-FH-success.

## 12. Ownership law

FJ preserves these ownership boundaries:

- `AuthenticatedRemoteSessionRuntimeOwner` remains owner of the authenticated peer and later accept authority;
- EV/EX owns one stream acceptance/read transaction at a time;
- requester handoff retains exact requester transaction/response-stream custody;
- FB consumes the handoff and preserves exact requester transaction across DR;
- Agent owns DR semantic provenance;
- FD owns pure acknowledgement projection/framing;
- FF owns exact same-stream requester write mechanics;
- FH consumes the requester transaction terminally;
- FJ reuses only the outer authenticated-session owner after FH success.

The consumed requester transaction is never reused.

## 13. No new peer-close law

FJ performs no peer close on:

- EX ingress failure;
- FH frame failure;
- FH response-I/O failure;
- FH success.

FJ does not reuse:

- capability termination code 3;
- capability shutdown code 4;
- any capability close reason.

FJ invents no requester-specific close code or reason.

A later explicit peer-lifecycle gate may decide how a higher owner handles a returned FJ failure.

## 14. Cancellation remains outside FJ

FJ materializes no cancellation race around FB/FH and does not call the EX cancellation-aware worker seam.

Not selected:

- cancellation racing DR continuation;
- cancellation racing FD framing;
- cancellation racing FF send;
- cancellation between FH success and the next EX cycle;
- cancellation-owned peer close;
- a combined requester-aware worker stop enum.

Any such behavior remains separately gated.

## 15. Identity and correlation invariants

FJ does not alter identity semantics:

- authenticated PRW application-session lineage = requester logical identity;
- exact nominated `DeviceId` = target logical identity;
- dynamic IP/port = transient endpoint data only;
- `TransportIdentity` = lower transport evidence only;
- PRWM `request_id` = correlation only.

FJ does not infer identity from stream order, endpoint tuple, connection metadata, or request correlation.

## 16. Fresh-authority law after resume

FJ reuses no previous transaction authority as authorization for later traffic.

A later EV transaction still receives its own fresh verifier time.

A later capability request still performs fresh current registry/policy authorization.

A later requester/rendezvous request still performs a new requester-aware DR composition, registration mutation when authorized, acknowledgement framing, and requester response-stream lifecycle.

Previous request ID, DR result, registry view, policy result, registration mutation, stream, endpoint, or transport evidence does not authorize a later transaction.

## 17. Candidate/reachability boundary remains closed

FJ does not query or select:

- candidate state;
- reachability state;
- endpoint state;
- relay state;
- direct-path state.

FJ does not establish:

- target QUIC/TCP connectivity;
- port-forward connectivity;
- terminal connectivity;
- remote-session connectivity;
- rendezvous completion.

FD accepted remains accepted-for-continuation only.

## 18. Runtime boundary remains closed

FJ remains an isolated, uninvoked Agent source seam.

It is not integrated into:

- the existing production capability loop;
- a production mixed-family worker;
- persistent worker collections;
- real remote admission;
- endpoint lifecycle startup;
- process lifecycle control;
- Agent `main.rs`;
- listener startup;
- readiness publication;
- service management.

## 19. Deployment boundary remains closed

FJ performs no:

- packaging;
- artifact publication;
- host mutation;
- service installation;
- deployment;
- restart;
- recovery;
- merge.

## 20. Validation requirements

Closure requires exact final-head validation after any formatter-only correction:

- locked dependency graph PASS;
- rustfmt PASS;
- Clippy PASS;
- workspace tests PASS;
- workspace build PASS.

If Android validation is triggered for the exact final head, its terminal verdict must be recorded. No Android PASS may be claimed without an exact-head run.

## 21. Expected source guards

Other than the exact Agent lifecycle source file above, these source paths must remain byte-stable from FI unless a concrete compiler contradiction proves a minimal change necessary:

- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`
  - expected blob `406cbf276c2c62a0bbd902a6ec25b8a0f93ca05c`
- `crates/prw-agent/src/remote_session_capability_runtime.rs`
  - expected blob `6b9b7bfa2445e3cbc7e713b598f67f7ec6115e8f`
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`
  - expected blob `083bf83fd1827f6175c9eb62ff93b40147fa9271`
- `crates/prw-remote-bridge/src/requester_rendezvous_dr_acknowledgement_wire.rs`
  - expected blob `71b8cd166b24268b1fd87f8f339f57200f426834`
- `crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`
  - expected blob `301d8bfbd57db09ecf5922f579dc146cca151003`

No Cargo/dependency/workflow/Android source change is expected.

## 22. Closure target

Canonical closure target:

`CLOSED_REQUESTER_RENDEZVOUS_POST_TERMINAL_RESPONSE_SERIAL_LIFECYCLE_SOURCE_MATERIALIZATION`

Canonical gate target:

`C03E_FJ_REQUESTER_RENDEZVOUS_POST_TERMINAL_RESPONSE_SERIAL_LIFECYCLE_SOURCE_MATERIALIZED`

## 23. Next separately gated seam

After FJ closure, the next gate should audit/select only the lifecycle above FJ failure returns and cancellation interaction before any active runtime integration.

At minimum it must decide:

- what higher owner does with `Ingress(...)` failure;
- what higher owner does with `RequesterResponse(Frame(...))`;
- what higher owner does with `RequesterResponse(ResponseIo(...))`;
- whether and how caller-owned cancellation may race the now-composed serial lifecycle;
- whether any peer close is selected, and if so under a new explicit requester/mixed-family lifecycle law rather than capability code reuse.

No such higher lifecycle behavior is authorized by FJ itself.
