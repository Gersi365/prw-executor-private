# Phase 152 C03e-EU — Authenticated Agent Single-Owner Ingress Consumption Transaction Selection (Staging)

Status: SELECTED_STAGING

## 1. Purpose

C03e-EU selects only the authenticated Agent-side one-transaction ownership boundary that may consume the bridge-owned C03e-ET post-authenticated single-read ingress.

This checkpoint is documentation-only. It does not materialize or activate a combined runtime loop, does not invoke requester/rendezvous current-authority or provider logic, and does not merge or deploy anything.

## 2. Exact predecessor

C03e-EU begins only from the durably closed C03e-ET state:

- predecessor branch: `phase-152-c03e-et-post-auth-control-stream-single-read-family-demultiplexing-source-materialization-staging`
- predecessor head: `12c759253013fa19ce2e36f9be453621975b7686`
- predecessor tree: `08293a73539d14fe07e03490bb986c754ffe642f`
- predecessor gate: `C03E_ET_POST_AUTH_CONTROL_STREAM_SINGLE_READ_FAMILY_DEMULTIPLEXING_SOURCE_MATERIALIZED`

No earlier checkpoint is reopened.

## 3. Fresh exact-head topology audit

At the ET predecessor head, `AuthenticatedRemoteSessionRuntimeOwner` retains exactly:

- the already-authenticated `AuthenticatedRemotePeerConnection`; and
- the existing bound `RemoteSessionCapabilityRuntimeOwner`.

The existing capability path currently accepts its own control stream inside `process_one_capability_request(...)`.

The isolated C03e-ER requester/rendezvous one-shot seam currently accepts its own control stream inside `receive_requester_rendezvous_start_intent_once(...)`.

Those two seams must not become concurrently active independent acceptors on the same retained authenticated peer. C03e-EU therefore selects one Agent-owned acceptance/consumption transaction before any runtime integration.

## 4. Selected Agent ownership boundary

The selected future materialization boundary is one crate-private method on `AuthenticatedRemoteSessionRuntimeOwner` conceptually named:

`process_one_post_auth_control_stream_ingress(...)`

The exact implementation name may remain this name unless source-level constraints require a narrower equivalent during the successor materialization checkpoint.

The method must retain a mutable borrow of the authenticated runtime owner for the duration of one transaction.

Purpose of the mutable owner borrow:

- serialize one Agent-owned stream acceptance/ingress transaction against another operation on the same owner;
- prevent this selected seam itself from becoming a concurrent second acceptor;
- preserve the existing single-owner runtime shape.

The borrow is not claimed as a network-wide fairness mechanism, queue, global mutex, cross-process lock, or proof against incorrectly constructed duplicate owners.

## 5. Exact selected transaction sequence

A future source materialization must preserve this order:

1. borrow the exact retained `AuthenticatedRemoteSessionRuntimeOwner` mutably;
2. call the retained authenticated peer's `accept_control_stream()` exactly once;
3. transfer the accepted `MeshControlStream` by value into C03e-ET `receive_post_auth_control_stream_ingress(...)`;
4. allow C03e-ET to perform exactly one bounded PRWM frame read and family selection;
5. branch only on the typed C03e-ET result;
6. never call another stream accept/read adapter for that transaction;
7. return one typed Agent transaction outcome;
8. stop without creating a combined loop or activating a new worker.

## 6. Capability-family outcome

For `PostAuthControlStreamIngress::Capability(transaction)`:

- use `transaction.request_frame()` as the exact already-read request frame;
- retain the existing bound-session transport identity and session lease;
- perform current registry/policy authorization through the existing `SharedCurrentCapabilityAuthority` and `CapabilityBridge` path;
- dispatch through the existing authorized capability dispatch boundary;
- send the existing response frame only through `PostAuthCapabilityTransaction::send_response_frame(...)`;
- preserve the exact same control stream already retained inside the C03e-ET custody envelope;
- do not re-read the request;
- do not accept a replacement stream;
- do not expose a raw `MeshControlStream` accessor to Agent callers;
- do not redesign `BridgeCommand`, capability policy, dispatcher semantics, request correlation, or response framing.

Any non-`PRWZ` frame remains on this capability-family path. Malformed/unknown capability payload semantics continue to fail at the existing capability bridge boundary; EU does not add a second family fallback.

## 7. Requester/rendezvous-family outcome

For `PostAuthControlStreamIngress::RequesterRendezvous(request)`:

- copy the outer PRWM `request_id` only into a separate correlation value;
- consume the strict decoded logical target `DeviceId`;
- adapt that target through the existing C03e-EO decoded-target helper;
- adapt the resulting target intent through the existing C03e-EJ authenticated-session helper;
- return the resulting non-authoritative `RequesterRendezvousStartIntent` together with the separate correlation value;
- stop before C03e-DV/current registry requester policy/provider execution;
- perform no candidate selection;
- construct/write no requester/rendezvous response;
- perform no dialing.

Malformed exact-`PRWZ` traffic remains a requester/rendezvous wire failure from C03e-ET and must not fall back into capability decoding.

## 8. Selected transaction outcome shape

The future source seam should return one typed enum-equivalent outcome with exactly these semantic branches:

- `CapabilityProcessed`
- `RequesterRendezvous(RequesterRendezvousCorrelatedStartIntent)`

The capability branch carries no raw stream because same-stream response completion occurs before successful return.

The requester/rendezvous branch carries only correlation plus the existing typed start intent. It carries no transport stream, endpoint, IP address, provider record, candidate, authorization verdict, or response frame.

## 9. Selected failure shape

The future source seam must preserve distinguishable existing failure classes without fabricating success:

- authenticated peer stream acceptance failure;
- C03e-ET ingress receive / strict requester-rendezvous wire failure;
- existing capability bridge authorization/dispatch failure;
- existing same-stream capability response I/O failure.

No retry, replacement stream, fallback family decode, request replay, peer reconnect, error-response invention, provider mutation, or whole-peer close policy is selected here.

## 10. Identity and correlation invariants

C03e-EU preserves the existing identity separation exactly:

- requester logical identity comes only from the retained authenticated PRW application session;
- target identity is the strict decoded logical `DeviceId` nomination;
- `TransportIdentity` remains lower transport evidence only;
- endpoint/IP/port data remains transient reachability data only;
- PRWM `request_id` remains transaction correlation only;
- family classification is not authentication;
- family classification is not authorization;
- successful PRWZ decoding is not requester/target eligibility or rendezvous-success authority.

No PID, UID, GID, transport address, request ID, process identity, or dynamic IP may become PRW logical identity.

## 11. Existing capability loop preservation

C03e-EU does not change or invoke the existing:

- `process_one_capability_request(...)`;
- `run_capability_request_loop(...)`;
- `run_capability_request_worker(...)`.

Those seams remain byte-stable in this selection checkpoint.

A successor source-materialization checkpoint may add the selected one-transaction seam, but replacement/integration of the existing capability loop is a separate gate.

## 12. Existing C03e-ER seam preservation

C03e-EU does not delete, mutate, invoke, or claim activation of `receive_requester_rendezvous_start_intent_once(...)`.

That ER seam remains isolated and uninvoked. It is historical source evidence for the requester composition path, not a second runtime acceptor to be activated beside the selected EU transaction.

Any later cleanup, deprecation, or removal of superseded isolated seams is separately destructive/behavioral work and is excluded here.

## 13. No combined-loop activation

C03e-EU specifically does not select runtime activation of a loop that repeatedly accepts and routes both families.

Not selected here:

- repeated combined accept loop;
- worker integration;
- task spawning;
- cancellation race semantics for the combined loop;
- fairness between capability and requester traffic;
- per-family concurrency limits;
- queueing/backpressure policy;
- drain/shutdown ordering for a combined loop;
- requester-response lifecycle;
- replay/idempotency;
- peer-close rules for requester failures.

These remain separately gated.

## 14. Concurrency boundary

The selected one-transaction seam is deliberately serial at the retained authenticated owner boundary.

C03e-EU does not authorize:

- two independent `accept_control_stream()` consumers on one authenticated peer;
- parallel capability and requester accept loops;
- detached requester tasks;
- detached capability tasks;
- per-stream fan-out before a separately selected bounded concurrency policy;
- fairness claims based only on QUIC stream arrival order.

## 15. Authority boundaries

The selected ingress transaction may only compose already-existing authority seams.

Capability branch authority remains:

`authenticated bound session -> current registry/transport revalidation -> capability policy -> typed dispatcher`

Requester branch remains only:

`authenticated PRW application session + caller-nominated target DeviceId -> non-authoritative RequesterRendezvousStartIntent`

C03e-DV and all requester current-registry/policy/provider execution remain excluded.

## 16. Security invariants preserved

C03e-EU must not introduce:

- PID/UID/GID -> PRW identity fabrication;
- request-selected host roots;
- request-selected terminal executable/argv/env/cwd;
- arbitrary shell fragments;
- PRW identity -> Linux user mapping;
- setuid/setgid/sudo/su/pkexec behavior;
- public/LAN forwarding bind widening;
- hostname/DNS widening of exact-target forwarding primitives;
- firewall/route/TUN/TAP expansion;
- arbitrary socket-option control;
- detached terminal/forward workers;
- dynamic IP as logical identity;
- PRWM request ID as logical identity;
- requester target nomination as authorization;
- ambient privilege assumptions.

## 17. Source mutation ceiling for EU

C03e-EU itself is selection-only.

Allowed mutation ceiling:

- exactly one new C03e-EU contract document;
- PR metadata describing the selection.

Not allowed in EU:

- Rust source changes;
- Cargo or lockfile changes;
- Kotlin/Gradle/Android source changes;
- workflow changes;
- configuration changes;
- packaging/systemd changes;
- listener/bootstrap changes;
- deployment/restart/recovery;
- merge.

## 18. Validation expectation

Because EU is documentation-only:

- canonical Rust validation may trigger through PR path filters and, if triggered, must pass on the exact final EU head before closure;
- Android validation may legitimately not trigger for a docs-only delta;
- no Android PASS may be claimed unless an exact-head Android workflow actually runs successfully;
- skipped disposable etcd workflows remain expected if their path gates are not matched.

## 19. Closure requirements

C03e-EU may close only after all of the following are true:

1. branch head is re-read after mutation;
2. predecessor/head ancestry is exact;
3. changed-path ceiling is exactly the selected docs-only contract;
4. any required exact-head workflow has terminal success;
5. immutable Drive audit is written and raw-read back byte-exact;
6. rolling Drive status is append-only from the exact post-ET predecessor;
7. rolling raw readback preserves the full post-ET predecessor prefix;
8. EU closure, gate, and immutable audit ID markers each occur exactly once;
9. PR remains draft/open/unmerged.

## 20. Target closure classification

On successful closure, the classification is:

`CLOSED_AUTHENTICATED_AGENT_SINGLE_OWNER_INGRESS_CONSUMPTION_TRANSACTION_SELECTION`

Target gate:

`C03E_EU_AUTHENTICATED_AGENT_SINGLE_OWNER_INGRESS_CONSUMPTION_TRANSACTION_SELECTED`

## 21. Successor direction

The preferred successor after durable EU closure is a narrowly bounded source-materialization checkpoint that adds only the selected one-transaction Agent seam while preserving:

- one stream acceptance;
- ET one-read family custody;
- legacy capability authorization/dispatch/response behavior;
- EO/EJ requester intent composition;
- stop-before-DV requester behavior;
- no combined loop activation.

Any repeated combined loop, worker replacement, cancellation/fairness/backpressure policy, requester response, provider execution, dialing, deployment, restart/recovery, or merge remains separately gated.
