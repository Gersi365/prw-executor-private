# Phase 152 C03e-EX — Authenticated Agent Repeated Post-Auth Ingress Loop / Worker Source Materialization (Staging)

Status: MATERIALIZED_STAGING

## 1. Purpose

C03e-EX source-materializes only the isolated and uninvoked repeated authenticated post-authenticated ingress loop and executor-neutral cancellation-aware worker selected by C03e-EW.

This checkpoint does not integrate the new loop/worker into any active runtime collection, replace the historical capability-only loop/worker, invoke C03e-DV/provider authority, define requester response semantics, retain requester response stream custody, dial a target, publish readiness, wire the Agent binary, deploy, restart, recover, or merge.

## 2. Exact predecessor authority

Canonical predecessor:

- repository: `Gersi365/prw-executor-private`
- checkpoint: C03e-EW
- branch: `phase-152-c03e-ew-authenticated-agent-repeated-post-auth-ingress-loop-worker-ownership-selection-staging`
- head: `bdadddd5612b5735493e50d097190671ec1ee0fe`
- tree: `528e03feef3ce5d1ebcb8b244a55027ee60fb36e`
- closure: `CLOSED_AUTHENTICATED_AGENT_REPEATED_POST_AUTH_INGRESS_LOOP_WORKER_OWNERSHIP_SELECTION`
- gate: `C03E_EW_AUTHENTICATED_AGENT_REPEATED_POST_AUTH_INGRESS_LOOP_WORKER_OWNERSHIP_SELECTED`

C03e-EX remains an exact descendant of C03e-EW.

## 3. Materialized source surface

C03e-EX changes the existing isolated authenticated-session child source:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`

The historical C03e-ER method and C03e-EV one-transaction method remain present. EX adds two crate-private methods on the same `AuthenticatedRemoteSessionRuntimeOwner`.

### 3.1 Repeated loop

Materialized method:

`run_repeated_post_auth_control_stream_ingress(...)`

The loop:

1. exclusively borrows `&mut AuthenticatedRemoteSessionRuntimeOwner`;
2. samples the caller-supplied verifier-time provider exactly once immediately before each C03e-EV transaction;
3. invokes `process_one_post_auth_control_stream_ingress(...)` exactly once per iteration;
4. never calls `accept_control_stream()` directly;
5. on `CapabilityProcessed`, begins the next serialized iteration;
6. on `RequesterRendezvous(handoff)`, returns the exact correlated non-authoritative start intent and stops before another accept;
7. on the first C03e-EV transaction error, returns that error unchanged and stops.

The loop return surface is intentionally minimal:

`Result<RequesterRendezvousCorrelatedStartIntent, AuthenticatedRemoteSessionPostAuthIngressTransactionError>`

An `Ok(handoff)` is the C03e-EW requester/rendezvous handoff barrier. An `Err(error)` is the first exact C03e-EV failure. Capability success is internal continue-only state and does not produce a terminal return.

## 4. Worker materialization

Materialized method:

`run_repeated_post_auth_control_stream_ingress_worker(...)`

The worker:

- owns one repeated EX loop future;
- owns one caller-supplied cancellation future;
- spawns no task;
- polls the repeated loop before cancellation on each wake;
- preserves an already-ready requester handoff or EV failure over simultaneous cancellation;
- lets cancellation win only while the repeated loop is pending;
- lexically drops the in-flight repeated-loop future before the cancellation result leaves the method, releasing the exclusive mutable owner borrow first.

The worker return surface is:

`Result<Option<RequesterRendezvousCorrelatedStartIntent>, AuthenticatedRemoteSessionPostAuthIngressTransactionError>`

Semantics are fixed:

- `Ok(Some(handoff))` — requester/rendezvous handoff barrier;
- `Ok(None)` — caller-owned cancellation;
- `Err(error)` — first unchanged C03e-EV transaction failure.

These three classes remain distinguishable without adding a new public or crate-level enum.

## 5. Accept ownership and serialization

C03e-EX introduces no direct `accept_control_stream()` call in the new repeated loop or worker.

The only accept path inside EX is inherited through C03e-EV:

`AuthenticatedRemoteSessionRuntimeOwner -> process_one_post_auth_control_stream_ingress -> one authenticated-peer accept -> C03e-ET one-read family ingress`

Therefore:

- at most one post-authenticated control-stream transaction is in flight per owner;
- no second capability acceptor is introduced;
- no second requester acceptor is introduced;
- no family-specific queue or task pool exists;
- no speculative pre-accept exists;
- transport arrival order plus one serialized owner remains the only selected fairness model.

## 6. Capability outcome

`CapabilityProcessed` means the exact C03e-EV capability transaction has already completed:

- current registry/policy authorization;
- authorized dispatch;
- response transmission on the same retained capability stream.

EX then continues to the next serialized iteration and samples verifier time again immediately before the next EV call.

No replay, retry, queue or concurrent capability work is introduced.

## 7. Requester/rendezvous handoff barrier

`RequesterRendezvous((request_id, start_intent))` remains a barrier.

EX returns it immediately and does not accept another control stream because:

- C03e-ET requester ingress does not retain response stream custody;
- C03e-EV stops before C03e-DV;
- requester provider execution remains excluded;
- requester candidate selection remains excluded;
- requester success/error response construction and write semantics remain excluded;
- dialing remains excluded.

The returned `request_id` remains transaction correlation only. Authenticated requester identity remains session-derived and target identity remains the logical target `DeviceId`.

## 8. Failure ownership

The first `AuthenticatedRemoteSessionPostAuthIngressTransactionError` stops the repeated loop and is returned unchanged.

EX adds no retry, fallback, suppression, replacement stream, replacement session, fabricated success, requester response, or whole-peer close.

The historical capability-only code-3 termination diagnostic is not reused or widened for mixed-family EV failure.

## 9. Cancellation and peer-close boundary

Cancellation in EX returns `Ok(None)` only.

EX deliberately performs no whole-peer close when cancellation wins.

Reason:

- the existing code-4 shutdown diagnostic is capability-specific;
- C03e-EW did not authorize silent widening of that diagnostic to mixed-family traffic;
- EX does not invent a replacement close code;
- close ownership remains separately gated.

The repeated-loop future is dropped before cancellation returns from the worker. No detached accepted-stream task or dispatcher task remains because EX creates no task.

## 10. Historical Q/S loop and worker

The existing historical methods remain source-stable and unmodified:

- `process_one_capability_request(...)`;
- `run_capability_request_loop(...)`;
- `run_capability_request_worker(...)`.

EX does not call them and does not replace their active invocation anywhere.

A future integration checkpoint must ensure the historical capability-only repeated accept path and the EX EV-backed repeated accept path are never activated concurrently for the same authenticated owner.

## 11. C03e-ER one-shot seam

`receive_requester_rendezvous_start_intent_once(...)` remains present and unchanged in behavior.

EX does not call it. It remains an isolated historical source seam and must not become a concurrent acceptor beside a future EX-backed loop.

## 12. Identity and authority invariants

The canonical identity model remains unchanged:

`authenticated PRW session identity -> logical requester identity`

`target DeviceId -> logical rendezvous target identity`

`TransportIdentity -> lower transport evidence only`

`IP/port -> transient reachability only`

`PRWM request_id -> transaction correlation only`

No PID, UID, GID, IP address, port, request ID or transport handle fabricates logical PRW identity.

Successful stream acceptance and family classification grant no requester provider authority.

## 13. Explicit exclusions

C03e-EX does not materialize or activate:

- active runtime integration of the EX loop;
- active runtime integration of the EX worker;
- capability-loop replacement;
- worker-loop replacement;
- a second authenticated acceptor;
- concurrent stream handling;
- family-specific queues;
- task spawning;
- C03e-DV invocation;
- requester registry/policy/provider execution;
- candidate enumeration or selection;
- requester response stream custody;
- requester success/error response semantics;
- requester response writes;
- replay/idempotency;
- retry/reconnect;
- new peer-close diagnostics;
- dialing;
- readiness;
- listener/bootstrap wiring;
- Agent `main.rs` activation;
- dependency changes;
- workflow changes;
- Android application changes;
- packaging/systemd changes;
- deployment/restart/recovery;
- merge.

## 14. Validation requirement

Because EX modifies Rust source, closure requires exact-final-head canonical validation:

- PRW Rust Validation FULL PASS;
- Android Validation FULL PASS if triggered by the canonical source delta;
- disposable C02f-AD/C02f-AE runs are interpreted only according to their expected skip/pass behavior and never substitute for canonical Rust/Android validation.

Any superseded candidate validation is not closure evidence.

## 15. Durable evidence requirement

Closure requires:

- immutable Drive audit for exact final EX head;
- raw byte-exact audit readback;
- append-only update of rolling `C02E_BRANCH_STATUS.md` from exact post-EW predecessor state;
- raw byte-exact rolling readback;
- predecessor prefix preservation;
- closure marker, gate marker and immutable audit ID each exactly once in the rolling state.

## 16. Closure markers

Closure marker:

`CLOSED_AUTHENTICATED_AGENT_REPEATED_POST_AUTH_INGRESS_LOOP_WORKER_SOURCE_MATERIALIZATION`

Gate marker:

`C03E_EX_AUTHENTICATED_AGENT_REPEATED_POST_AUTH_INGRESS_LOOP_WORKER_SOURCE_MATERIALIZED`

C03e-EX is source-materialization evidence only. It does not authorize merge, deployment or runtime activation.
