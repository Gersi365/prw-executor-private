# Phase 152 C03e-FK — Higher-Owner Mixed-Family Serial Lifecycle Failure/Cancellation Disposition Selection (Staging)

Status: SEMANTICS_SELECTION_STAGING

## 1. Purpose

C03e-FK selects only the higher-owner disposition law around the isolated C03e-FJ requester-aware mixed-family serial lifecycle.

FK is docs-only. It does not materialize a cancellation-aware worker, alter FJ/EX/FB/FH source, close a live peer, invent or reuse a close code, integrate the lifecycle into a persistent worker/runtime/listener, select candidate/reachability/endpoint/relay state, dial target traffic, activate port-forward/terminal/session behavior, deploy, restart, recover, or merge.

## 2. Exact predecessor

Canonical predecessor:

- branch: `phase-152-c03e-fj-requester-rendezvous-post-terminal-response-serial-lifecycle-source-materialization-staging`
- exact head: `9f6152c88fe43b6e52c9844117fde76e7e19df23`
- exact tree: `a1bda4e1916d352a7cde5439595c329bd6a76aeb`
- FJ contract blob: `ddb527fc93d3c3cac15ad040f61e95868b711fac`
- FJ Agent requester lifecycle blob: `2bab48c68b63f1e5b2058c40fc3539e7841d5a32`

FK must remain an exact docs-only descendant of that head.

## 3. Exact audited source guards

The FK selection is bounded by these exact FJ-head blobs:

1. FJ requester-aware serial lifecycle / FB / FH source
   - `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
   - blob `2bab48c68b63f1e5b2058c40fc3539e7841d5a32`

2. EV/EX one-transaction/repeated-ingress/cancellation precedent
   - `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`
   - blob `406cbf276c2c62a0bbd902a6ec25b8a0f93ca05c`

3. Shared-current authority
   - `crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`
   - blob `50356b47d3c5304b67edd424e9286beb028ace16`

4. Historical capability-only loop/worker close precedent
   - `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`
   - blob `083bf83fd1827f6175c9eb62ff93b40147fa9271`

5. FF requester same-stream consuming send
   - `crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`
   - blob `301d8bfbd57db09ecf5922f579dc146cca151003`

6. Lower bounded QUIC stream write/read runtime
   - `crates/prw-remote-transport/src/runtime.rs`
   - blob `d03bcf642aeb2576656437a8b3d2ddf148a50e30`

## 4. Exact predecessor facts

At exact FJ head:

- FJ runs indefinitely after successful transactions and returns only on typed lifecycle failure.
- FJ preserves exactly two top-level failure families:
  - `Ingress(AuthenticatedRemoteSessionPostAuthIngressTransactionError)`;
  - `RequesterResponse(RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError)`.
- requester-response failure remains internally split into exact `Frame(...)` versus exact `ResponseIo(...)`.
- existing EX remains the sole post-authenticated stream accept/read loop.
- EX already polls its ingress loop before caller cancellation, so an already-ready handoff or ingress failure wins over simultaneous cancellation.
- existing EX cancellation drops the in-flight ingress future before returning and deliberately performs no whole-peer close or capability code-4 reuse.
- FJ itself introduces no cancellation race and no peer-close action.
- FB acquires current authority asynchronously, but the actual requester DR validation/authorization/registration operation is synchronous once the read guard is acquired.
- therefore caller cancellation can safely precede that synchronous operation, but must not be modeled as interrupting it halfway through its registry mutation.
- FH consumes exact requester response custody and resolves it only through exact frame failure, exact response-I/O failure, or terminal response success.
- lower FF/send custody is by-value; no retry-capable requester transaction or raw stream is returned.
- historical capability-only code-3 termination and code-4 cancellation shutdown belong to the capability-only lifecycle and were explicitly not widened by EX/FJ.

## 5. Selected higher-owner stop model

A later source-materialization checkpoint should expose exactly two higher-owner stop classes:

1. `Cancelled` — caller-owned cancellation won at a selected cancellation-safe ingress boundary;
2. `Failed(existing FJ RequesterRendezvousPostTerminalResponseSerialLifecycleError)` — the exact FJ lifecycle failed.

The future exact Rust type/variant names remain source-materialization details.

No clean `Completed` class is selected. Current FJ has no clean terminal success branch: successful requester/capability transactions continue serial ingress.

## 6. Failure disposition

All FJ failures are higher-owner terminal for the current isolated worker invocation and are propagated without flattening, suppression, retry, fallback, or conversion into cancellation.

### 6.1 FJ `Ingress(...)`

- preserve the exact existing `AuthenticatedRemoteSessionPostAuthIngressTransactionError` beneath FJ `Ingress`;
- stop the current mixed-family lifecycle invocation;
- do not begin another EX ingress cycle;
- do not retry accept/read/authorization/dispatch/response;
- do not fabricate requester response semantics;
- do not automatically close the authenticated peer.

### 6.2 FJ `RequesterResponse(Frame(...))`

- preserve the exact FH frame-construction failure;
- stop the current mixed-family lifecycle invocation;
- do not start another EX ingress cycle;
- do not fabricate a replacement rejected acknowledgement;
- do not retry DR, requester registration, framing, or response write;
- do not automatically close the authenticated peer.

### 6.3 FJ `RequesterResponse(ResponseIo(...))`

- preserve the exact FH/FF response-I/O failure;
- stop the current mixed-family lifecycle invocation;
- do not start another EX ingress cycle;
- do not retry/resend/replace the stream or duplicate the acknowledgement;
- do not rerun DR or requester registration;
- do not automatically close the authenticated peer.

A pending cancellation must not erase or relabel an already-resolved FJ failure.

## 7. Cancellation placement selection

FK deliberately does **not** select a direct race between one opaque whole-FJ future and cancellation.

Reason: after requester handoff, FJ may complete requester DR registration before terminal acknowledgement response custody is resolved. Allowing cancellation to drop the whole FJ future at an arbitrary point after that side effect could create a locally induced state where requester registration committed but the terminal acknowledgement was abandoned solely due caller cancellation.

Instead, FK selects cancellation only at explicit safe boundaries.

### 7.1 Before requester handoff / while ingress is pending

Caller cancellation may race the existing EX mixed-family ingress future while that future is pending.

- ingress/handoff/failure is polled first, preserving existing EX precedence;
- if ingress produces a requester handoff or an ingress failure on the same wake as cancellation, the ingress result wins;
- if ingress remains pending and cancellation becomes ready, cancellation wins;
- the in-flight ingress future is dropped before `Cancelled` is returned;
- this releases the exclusive mutable session-owner borrow first;
- no retry or replacement stream is created;
- no automatic whole-peer close occurs.

This preserves the already-selected EX cancellation ordering.

### 7.2 After requester handoff

Once EX has produced an exact requester/rendezvous handoff, caller cancellation is deferred/masked until the requester transaction reaches one terminal FH outcome.

The higher-owner sequence is then non-cancellable with respect to the caller cancellation signal:

1. run exact FB continuation once;
2. run exact FH terminal acknowledgement composition once;
3. observe exact FH success or exact FH `Frame`/`ResponseIo` failure.

This does not mean underlying bounded transport timeouts are disabled. Existing FB/FH/FF lower errors and timeouts remain authoritative.

### 7.3 Why the requester critical section is cancellation-deferred

- requester response-stream custody is already owned;
- FB may perform the one authorized requester-registration mutation;
- once current-authority acquisition completes, that mutation is synchronous and cannot be interrupted midway by an await;
- FH is the only selected terminal acknowledgement path for the retained requester transaction;
- deferring caller cancellation prevents a local shutdown signal from becoming an implicit response-abandonment path after requester registration has already committed.

No new rollback is selected or required.

## 8. Outcome precedence inside the requester critical section

After requester handoff, exact requester transaction outcome has precedence over caller cancellation.

- FH `Frame(...)` -> `Failed(RequesterResponse(Frame(...)))` even if cancellation became ready while FB/FH was running;
- FH `ResponseIo(...)` -> `Failed(RequesterResponse(ResponseIo(...)))` even if cancellation became ready while FB/FH was running;
- FH `Ok(())` completes the requester transaction before cancellation is reconsidered.

Cancellation does not reclassify an already-resolved requester transaction failure as `Cancelled`.

## 9. Post-FH-success cancellation boundary

After exact FH `Ok(())`, but **before accepting another control stream**, the higher owner must reconsider the caller cancellation signal.

If cancellation became ready while requester FB/FH processing was in progress:

- return `Cancelled` before starting the next EX/EV ingress accept;
- do not consume another verifier-time sample for a transaction that will not start;
- do not accept/read another control stream first;
- do not close the peer automatically.

Only if cancellation is still pending may the next serial EX ingress cycle begin.

This is stricter than simply polling an opaque FJ future first forever and is selected specifically to avoid cancellation starvation by immediately-ready subsequent traffic.

## 10. Cancellation during capability-only transaction work

Before any requester handoff, cancellation continues to follow the selected EX race boundary.

If the current EX/EV transaction is already ready with capability success, requester handoff, or exact ingress failure when polled, that transaction result wins before cancellation is observed, matching existing EX precedent.

FK does not introduce preemption inside synchronous capability authorization/dispatch work.

## 11. Peer lifecycle selection

FK selects **no automatic whole-peer close** for either higher-owner stop class.

Therefore:

- `Cancelled` does not close the peer;
- `Failed(Ingress(...))` does not close the peer;
- `Failed(RequesterResponse(Frame(...)))` does not close the peer;
- `Failed(RequesterResponse(ResponseIo(...)))` does not close the peer.

FK does not reuse:

- capability-only close code 3 / `remote capability session terminated`;
- capability-only close code 4 / `remote capability session shutdown`.

FK invents no new mixed-family close code or close reason.

The authenticated peer remains owned by the outer `AuthenticatedRemoteSessionRuntimeOwner` after the borrowed isolated worker invocation releases its mutable borrow. A later explicit owner/integration gate may decide whether that higher owner closes, retires, reuses, or drops the peer.

## 12. Cancellation does not equal protocol failure

`Cancelled` is local caller-owned lifecycle control, not:

- requester rejection;
- requester response frame failure;
- transport authentication failure;
- capability denial;
- session identity failure;
- target offline state;
- reachability failure;
- rendezvous failure.

No cancellation marker is written onto requester/capability wire protocol.

## 13. Identity/correlation preservation

FK preserves all existing identity boundaries:

- authenticated PRW application-session lineage = requester logical identity;
- exact nominated `DeviceId` = target logical identity;
- dynamic IP/port = transient endpoint data only;
- `TransportIdentity` = lower transport evidence only;
- PRWM `request_id` = correlation only.

Cancellation timing, stream ordering, FJ error class, endpoint tuple, or request ID does not become identity.

## 14. No retry/replay law

Neither `Cancelled` nor any `Failed(...)` result authorizes:

- request retry;
- response retry/resend;
- replacement stream;
- duplicate acknowledgement;
- DR rerun;
- requester-registration retry;
- capability re-dispatch;
- reuse of a consumed requester transaction;
- reuse of an old request ID as authority.

Any later transaction must begin as a fresh post-authenticated request under a separately active higher-owner lifecycle.

## 15. Candidate/reachability boundary remains closed

FK does not authorize:

- candidate query/selection;
- reachability evaluation;
- endpoint resolution;
- relay selection;
- direct-path attempt;
- target transport establishment;
- port-forward activation;
- terminal activation;
- remote-session establishment;
- rendezvous completion.

Requester `Accepted` remains accepted-for-continuation only.

## 16. Runtime/integration boundary remains closed

FK does not:

- materialize the cancellation-aware mixed-family worker;
- register that worker in the persistent worker collection;
- replace the historical capability-only worker;
- integrate a production listener/admission path;
- alter process lifecycle control;
- publish readiness;
- alter Android behavior;
- change dependencies/workflows;
- package/deploy/restart/recover;
- merge.

## 17. Source-materialization target for the next checkpoint

The next source gate should materialize only an isolated Agent-owned cancellation-aware higher-owner worker implementing this exact boundary law.

The preferred shape is to reuse existing EX + FB + FH primitives at explicit transaction boundaries rather than race cancellation against the opaque infinite FJ future.

Required properties:

- one caller-supplied cancellation future;
- EX ingress polled before cancellation while no requester handoff is owned;
- `Cancelled` only when EX is pending or at the post-FH-success pre-next-accept boundary;
- requester handoff masks/defer cancellation through FB+FH;
- requester FH failure wins over pending cancellation;
- FJ-equivalent typed `Ingress` versus `RequesterResponse` error preservation;
- no automatic peer close;
- no task/channel/queue creation;
- no active runtime integration.

Exact refactoring/helper placement remains a source-materialization detail and must be kept minimal.

## 18. Canonical closure target

When exact-head validation and durable audit evidence are complete, the semantic closure marker is:

`CLOSED_HIGHER_OWNER_MIXED_FAMILY_SERIAL_LIFECYCLE_FAILURE_CANCELLATION_DISPOSITION_SELECTION`

Canonical gate:

`C03E_FK_HIGHER_OWNER_MIXED_FAMILY_SERIAL_LIFECYCLE_FAILURE_CANCELLATION_DISPOSITION_SELECTED`

## 19. Next separately gated seam

**C03e-FL — higher-owner mixed-family cancellation-aware serial lifecycle worker source materialization**.

FL may materialize only the isolated cancellation-aware worker selected here. Persistent worker collection/runtime/listener integration, explicit peer retirement/close policy, candidate/reachability continuation, target dialing, deployment, and merge remain later gates.
