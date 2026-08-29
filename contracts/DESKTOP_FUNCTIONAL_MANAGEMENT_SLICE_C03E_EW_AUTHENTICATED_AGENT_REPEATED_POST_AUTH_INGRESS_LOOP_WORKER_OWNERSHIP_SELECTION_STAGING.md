# Phase 152 C03e-EW — Authenticated Agent Repeated Post-Auth Ingress Loop / Worker Ownership Selection (Staging)

Status: SELECTED_STAGING

## 1. Purpose

C03e-EW selects the ownership and stop/handoff semantics for a future repeated post-authenticated control-stream ingress loop and its executor-neutral worker body.

This checkpoint is selection-only. It does not activate a repeated loop, replace the existing capability loop/worker, invoke requester/rendezvous provider authority, define requester response wire semantics, dial a target, publish readiness, wire the Agent binary, deploy, restart, recover, or merge.

The selected direction extends the already-materialized C03e-EV one-transaction seam without creating a second authenticated control-stream acceptor.

## 2. Exact predecessor authority

Canonical predecessor:

- repository: `Gersi365/prw-executor-private`
- predecessor checkpoint: C03e-EV
- predecessor branch: `phase-152-c03e-ev-authenticated-agent-single-owner-ingress-transaction-source-materialization-staging`
- predecessor head: `77a3d2af3ddc08964edf26890b32b09466af5816`
- predecessor tree: `8dff7bab312ae20dab007c7e05508027fcd1e4c7`
- predecessor closure: `CLOSED_AUTHENTICATED_AGENT_SINGLE_OWNER_INGRESS_TRANSACTION_SOURCE_MATERIALIZATION`
- predecessor gate: `C03E_EV_AUTHENTICATED_AGENT_SINGLE_OWNER_INGRESS_TRANSACTION_SOURCE_MATERIALIZED`

C03e-EW must remain an exact descendant of that predecessor. No earlier capability-only loop is an authority base for EW.

## 3. Verified current topology

### 3.1 Retained authenticated owner

`AuthenticatedRemoteSessionRuntimeOwner` retains:

- one `AuthenticatedRemotePeerConnection`;
- one `RemoteSessionCapabilityRuntimeOwner` containing the already-bound authenticated application session.

Logical requester identity continues to derive from the authenticated PRW session. Transport identity remains lower-transport evidence. IP/port remain transient reachability data. PRWM `request_id` remains transaction correlation only.

### 3.2 Existing capability-only transaction and loop

The current source still contains the historical capability-only path:

- `process_one_capability_request(...)`
  - accepts one new control stream;
  - reads one capability frame;
  - performs current registry/policy authorization;
  - dispatches one authorized capability request;
  - writes one response on the same stream.

- `run_capability_request_loop(...)`
  - repeatedly samples verifier time;
  - invokes `process_one_capability_request(...)`;
  - closes the peer with the existing capability-session termination diagnostic on the first transaction error.

- `run_capability_request_worker(...)`
  - races the capability-only loop against caller-owned cancellation;
  - owns the current code-4 capability-session shutdown close when cancellation wins.

These methods remain real source but are capability-family-only. They do not consume C03e-ET family ingress and therefore must not become concurrent acceptors beside C03e-EV.

### 3.3 C03e-ET family ingress

`receive_post_auth_control_stream_ingress(stream)`:

- consumes one already-accepted `MeshControlStream` by value;
- reads exactly one bounded PRWM frame;
- recognizes only exact `PRWZ` prefix as requester/rendezvous family;
- keeps all non-PRWZ frames on the legacy capability path;
- capability outcome retains the exact already-read frame plus the same stream for response custody;
- requester/rendezvous outcome returns only the strict decoded request and does not retain stream custody for a future requester response.

This asymmetry is important to EW ownership selection.

### 3.4 C03e-EV one-transaction seam

`process_one_post_auth_control_stream_ingress(...)` is the selected single Agent-owned acceptance point for one isolated post-authenticated transaction:

1. one authenticated-peer `accept_control_stream()`;
2. accepted stream transferred by value into C03e-ET;
3. capability branch reuses the already-read frame and same-stream response custody;
4. requester branch preserves outer `request_id` only as correlation and composes target `DeviceId` through C03e-EO then C03e-EJ;
5. requester branch returns one correlated non-authoritative `RequesterRendezvousStartIntent`;
6. requester branch stops before C03e-DV, provider execution, candidate selection, response construction/write, or dialing.

The EV method deliberately takes `&mut self`. The exclusive mutable borrow is transaction-custody architecture, not incidental mutation.

## 4. Problem EW must solve

A future authenticated session must eventually accept more than one post-authenticated control stream while preserving deterministic family ownership.

Naively leaving `run_capability_request_loop(...)` active while adding a second repeated EV consumer would create competing calls to `accept_control_stream()` on the same retained authenticated peer. That is prohibited.

Naively replacing the capability loop with an endless mixed-family loop is also premature because the current requester/rendezvous branch has no selected provider execution, response semantics, or requester response stream custody.

Therefore EW must select one repeated owner while also defining a safe barrier when requester/rendezvous traffic appears.

## 5. Selected repeated ownership model

C03e-EW selects exactly one repeated post-authenticated ingress owner per `AuthenticatedRemoteSessionRuntimeOwner`.

The future repeated ingress loop must:

1. borrow the authenticated owner exclusively through `&mut AuthenticatedRemoteSessionRuntimeOwner` for the loop lifetime;
2. invoke C03e-EV `process_one_post_auth_control_stream_ingress(...)` exactly once per iteration;
3. never call `accept_control_stream()` directly outside EV;
4. never invoke `process_one_capability_request(...)` inside the same repeated loop;
5. permit at most one accepted post-authenticated control stream transaction in flight for that owner;
6. preserve arrival-order serialization rather than introducing family-specific queues or parallel workers.

There is exactly one authoritative accept path while the future EW loop is active:

`AuthenticatedRemoteSessionRuntimeOwner -> C03e-EV one transaction -> C03e-ET one read -> typed family outcome`

No second capability acceptor and no second requester acceptor may coexist.

## 6. Capability outcome selection

When C03e-EV returns `CapabilityProcessed`:

- the capability request has already completed its same-stream response transaction;
- the repeated owner may begin the next iteration;
- verifier time may be sampled again immediately before the next EV transaction;
- no additional capability queue, task, retry, replay, or concurrency is introduced.

Capability success is the only currently selected outcome that automatically reaches another accept iteration.

## 7. Requester/rendezvous handoff barrier

When C03e-EV returns `RequesterRendezvous((request_id, start_intent))`, the future EW repeated ingress loop must stop accepting additional streams and return a typed handoff outcome to its caller.

The requester/rendezvous outcome is therefore a **handoff barrier**, not a normal continue-loop result.

The barrier is required because, at the current source boundary:

- C03e-ET requester/rendezvous ingress returns only the decoded request;
- the requester control stream is not retained for future response custody;
- C03e-EV intentionally stops before C03e-DV/provider execution;
- requester success/error response wire semantics are not selected;
- candidate selection and dialing remain excluded.

EW must not hide these missing semantics by accepting later streams as if the requester transaction had reached a selected terminal state.

The handoff preserves:

- authenticated requester identity through the existing session-derived start intent;
- target logical `DeviceId` as logical target identity;
- outer `request_id` as correlation only.

It must not convert `request_id`, transport identity, IP, port, PID, UID or GID into logical authorization identity.

## 8. Selected loop stop surface

At selection level, the future repeated loop requires distinguishable terminal/handoff classes equivalent to:

- `RequesterRendezvous(RequesterRendezvousCorrelatedStartIntent)` — successful family handoff barrier;
- `Failed(AuthenticatedRemoteSessionPostAuthIngressTransactionError)` — first EV transaction failure.

The exact Rust type name and enum placement are source-materialization details for a later checkpoint, but these semantic classes are fixed by EW.

A requester/rendezvous handoff is not classified as a failure and is not classified as a completed rendezvous operation.

## 9. Failure ownership selection

The first C03e-EV transaction error terminates the repeated ingress loop and is returned without retry, suppression, fallback, stream replacement, session replacement, or fabricated success.

EW does **not** authorize silent reuse of the existing capability-only code-3 termination diagnostic for mixed-family EV failure. The historical constant/reason is capability-specific.

A future source-materialization checkpoint must either:

- select/materialize an appropriately scoped post-authenticated session termination diagnostic, or
- explicitly retain failure close ownership outside the repeated loop.

Until that separate source choice is made, EW selects error propagation and loop termination, not a new numeric close code.

## 10. Worker ownership selection

The future executor-neutral repeated post-auth ingress worker must own exactly one repeated EW loop future and one caller-supplied cancellation future.

It must not spawn its own task.

The worker must preserve exclusive session ownership while the loop future is alive. It must not run the historical capability-only worker concurrently with the future EW worker.

Selected worker stop classes must remain distinguishable:

- cancellation;
- requester/rendezvous handoff barrier;
- repeated-ingress transaction failure.

The exact type names remain source-materialization details.

## 11. Cancellation selection

Cancellation may stop the future repeated loop only through the one Agent-owned worker race.

When cancellation wins:

1. the in-flight repeated-loop future must be dropped first;
2. dropping it releases the exclusive mutable owner borrow;
3. only then may the worker perform any selected whole-peer shutdown action;
4. no detached accepted-stream or dispatcher task may remain.

EW does not automatically widen or rename the existing capability-specific code-4 shutdown diagnostic. Reuse or replacement of that diagnostic remains a source-materialization detail that must preserve one-close ownership.

No cancellation path may create a second acceptor, retry an accepted request, or fabricate requester/rendezvous completion.

## 12. Serialization, fairness and backpressure

EW selects strict per-session serialization:

- one accepted post-authenticated control stream transaction at a time;
- no concurrent capability transactions;
- no concurrent capability/requester transactions;
- no family-specific task pools;
- no queue owned by EW;
- no speculative pre-accept of the next stream.

Fairness is therefore transport arrival order plus the single serialized owner. No weighted family fairness policy is selected.

Backpressure is the absence of another accept while one EV transaction or requester handoff barrier is outstanding. No additional buffering policy is selected.

## 13. Verifier-time ownership

For compatibility with existing capability authorization, the future repeated loop may retain a caller-supplied `FnMut() -> u64` verifier-time provider.

The selected sampling point is once immediately before each EV transaction invocation.

Requester/rendezvous composition must not reinterpret this capability verifier-time sample as requester identity, target identity, provider freshness authority, or rendezvous policy authority.

No wall-clock source is selected or widened by EW.

## 14. Legacy capability loop/worker migration rule

The existing:

- `run_capability_request_loop(...)`;
- `run_capability_request_worker(...)`;
- `process_one_capability_request(...)`

remain unchanged in EW selection source.

Future source materialization must not activate both the historical capability-only repeated accept path and the new EV-backed repeated accept path for the same authenticated owner.

The safe staging rule is:

- materialize any new EW loop/worker seam isolated and uninvoked first;
- prove exact source validation;
- only a separately gated integration checkpoint may replace historical runtime invocation.

EW does not itself perform that replacement.

## 15. Requester continuation remains separately gated

After the requester handoff barrier, all of the following remain outside EW:

- C03e-DV current registry/requester-policy/provider execution;
- provider authority acquisition or mutation;
- candidate enumeration or selection;
- reachability resolution;
- requester success/error response construction;
- requester response stream custody;
- requester response write semantics;
- replay/idempotency semantics;
- retry/reconnect;
- dialing;
- peer-close semantics specific to requester response failure.

A future checkpoint must address requester continuation and response custody before any claim of an endless mixed-family session loop.

## 16. Unknown and malformed family behavior

EW inherits C03e-ET family classification exactly:

- exact `PRWZ` prefix selects requester/rendezvous strict decode;
- malformed PRWZ fails as requester/rendezvous wire error;
- every non-PRWZ bounded frame remains on the legacy capability path.

EW introduces no fallback decoder, no generic command family, no protocol probing and no second read.

## 17. Identity and authority invariants

The following remain mandatory:

`authenticated PRW session identity -> logical requester identity`

`target DeviceId -> logical rendezvous target identity`

`TransportIdentity -> lower transport evidence only`

`IP/port -> transient reachability only`

`PRWM request_id -> transaction correlation only`

No PID/UID/GID, IP address, port, request ID or transport handle may fabricate logical PRW identity.

No requester/rendezvous provider authority is granted by successful stream acceptance or family classification.

## 18. Security boundaries preserved

EW does not authorize:

- request-selected host roots;
- request-selected terminal executable/argv/env/cwd;
- arbitrary shell fragments;
- PRW identity to Linux-user mapping;
- setuid/setgid/sudo/su/pkexec behavior;
- ambient privilege widening;
- public/LAN forwarding binds by default;
- arbitrary bind addresses;
- DNS/hostname widening of exact-target primitives;
- firewall, route, TUN/TAP expansion;
- arbitrary socket options;
- detached terminal/forward workers;
- failed-provider reopen;
- cross-principal ID reuse;
- dynamic IP as identity;
- request ID as identity;
- runtime/listener/deployment activation.

## 19. Explicitly excluded from C03e-EW

C03e-EW does not materialize or activate:

- a Rust repeated post-auth ingress loop;
- a Rust repeated post-auth worker;
- capability-loop replacement;
- worker-loop replacement;
- task spawning;
- concurrent stream handling;
- requester provider/policy execution;
- C03e-DV invocation;
- requester candidate selection;
- requester response stream retention;
- requester success/error response semantics;
- requester response writes;
- requester replay/idempotency;
- retry/reconnect;
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

## 20. Selected source-materialization target

The next source checkpoint, if separately authorized, should materialize only an isolated and uninvoked EV-backed repeated loop/worker seam consistent with EW:

- one exclusive authenticated owner;
- one EV transaction per iteration;
- capability success -> continue;
- requester/rendezvous outcome -> typed handoff barrier and stop;
- first EV error -> typed failure and stop;
- cancellation remains worker-owned;
- no second acceptor;
- no requester provider/response/dialing activation.

It should not yet integrate the new worker into any active runtime collection or Agent bootstrap.

## 21. Selection closure

C03e-EW is selected when this contract is the only net repository change over exact C03e-EV, exact-head validation is terminal as applicable, durable evidence is recorded, and no runtime activation has occurred.

Closure marker:

`CLOSED_AUTHENTICATED_AGENT_REPEATED_POST_AUTH_INGRESS_LOOP_WORKER_OWNERSHIP_SELECTION`

Gate marker:

`C03E_EW_AUTHENTICATED_AGENT_REPEATED_POST_AUTH_INGRESS_LOOP_WORKER_OWNERSHIP_SELECTED`
