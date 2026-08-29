# Phase 152 C03e-EY — Requester/Rendezvous Handoff Continuation + Response-Stream Custody Selection (Staging)

Status: SELECTED_STAGING

## 1. Purpose

C03e-EY selects only the ownership contract required to carry one requester/rendezvous transaction from the existing C03e-EX handoff barrier toward the already-existing authenticated Agent C03e-DV authoritative start-execution seam while preserving the exact accepted control stream for a later requester response transaction.

This checkpoint is selection-only. It does not change Rust source, invoke provider authority, construct or write a requester response, select a new requester response wire schema, dial a target, resume the repeated ingress loop, activate a listener/runtime, deploy, restart, recover, or merge.

## 2. Exact predecessor authority

Canonical predecessor:

- repository: `Gersi365/prw-executor-private`
- checkpoint: C03e-EX
- branch: `phase-152-c03e-ex-authenticated-agent-repeated-post-auth-ingress-loop-worker-source-materialization-staging`
- head: `de34c9f75f771ba6baf3238b7d727449431bd3f4`
- tree: `d639a40455699821be045462b662ccc04ece75a9`
- closure: `CLOSED_AUTHENTICATED_AGENT_REPEATED_POST_AUTH_INGRESS_LOOP_WORKER_SOURCE_MATERIALIZATION`
- gate: `C03E_EX_AUTHENTICATED_AGENT_REPEATED_POST_AUTH_INGRESS_LOOP_WORKER_SOURCE_MATERIALIZED`

C03e-EY must remain an exact descendant of that predecessor.

## 3. Verified current requester ingress topology

### 3.1 C03e-ET one-read family ingress

`receive_post_auth_control_stream_ingress(stream)` consumes one already-accepted `MeshControlStream` by value and receives exactly one bounded PRWM frame.

Family classification remains exact:

- exact `PRWZ` payload prefix selects requester/rendezvous strict decode;
- every non-PRWZ bounded frame remains on the capability path.

Current custody is asymmetric:

- capability outcome retains the exact already-read frame together with the exact same stream for response I/O;
- requester/rendezvous outcome returns the strict decoded requester request but does not retain the stream in its typed outcome.

No requester response transaction can therefore be safely selected without first repairing typed stream custody.

### 3.2 Strict requester request shape

The existing requester/rendezvous target wire request contains only:

- outer `request_id: u64` as transaction correlation;
- strict target `DeviceId` as logical target identity.

`request_id` is not requester identity, target identity, authenticated-session identity, transport identity, registry authority, policy authority, provider authority, or reachability identity.

### 3.3 C03e-EV transaction seam

`AuthenticatedRemoteSessionRuntimeOwner::process_one_post_auth_control_stream_ingress(...)` is the current single Agent-owned accept/ET transaction seam.

For requester/rendezvous traffic it currently:

1. preserves outer `request_id` only as correlation;
2. consumes strict target `DeviceId` through the existing C03e-EO adaptation;
3. composes the target through C03e-EJ using the retained authenticated session as requester identity authority;
4. returns the existing correlated non-authoritative `RequesterRendezvousStartIntent`;
5. stops before C03e-DV/provider execution, requester response mapping/write, candidate transport selection, or dialing.

### 3.4 C03e-EX repeated handoff barrier

The isolated EX repeated loop continues only after `CapabilityProcessed`.

When requester/rendezvous is observed, EX returns the correlated start intent as a typed handoff barrier and stops before accepting another stream. This barrier is required precisely because requester response-stream custody and continuation are not yet selected/materialized.

## 4. Verified existing authoritative continuation boundary

C03e-EY does not select a new provider-authority path.

The authenticated Agent already contains the crate-private C03e-DV caller seam:

`AuthenticatedRemoteSessionRuntimeOwner::execute_requester_rendezvous_start_intent(...)`

It delegates to the existing C03e-DR composite:

`validate_authorize_and_register_requester_rendezvous_start(...)`

The DR sequence is fail-closed and ordered:

1. C03e-DI current registry validation of requester and target;
2. C03e-DP current requester-rendezvous policy snapshot acquisition;
3. existing requester-rendezvous start authorization;
4. C03e-DN live provider registration.

The DV/DR terminal type is currently:

`Result<(), RequesterRendezvousStartCompositionError>`

C03e-EY selects this exact existing boundary for later continuation. It does not bypass, duplicate, widen, or parallelize it.

## 5. Response-wire audit result

No existing requester/rendezvous-specific result encoder or requester/rendezvous response-stream transaction was found in the current canonical bridge source.

The repository contains candidate-publication terminal result codecs and general PRWC request-ID/authentication support, but those are not requester/rendezvous result semantics and must not be reused by analogy without a separate contract.

Therefore EY does not invent a requester success payload, rejection payload, operation tag, external error mapping, or response frame encoder.

Requester response projection/encoding/write semantics remain separately gated.

## 6. Selected bridge requester custody repair

A future source-materialization checkpoint must change only the typed requester branch custody shape so that C03e-ET can return one by-value requester transaction envelope containing:

- the strict decoded `RequesterRendezvousTargetWireRequest`;
- the exact same already-accepted `MeshControlStream` from which that request frame was received.

The selected custody law is:

`accepted stream -> exactly one ET read -> strict requester decode + same-stream custody`

The future requester envelope must not:

- accept another stream;
- clone or replace the stream;
- re-read the requester request;
- create a detached response task;
- fabricate another request ID;
- interpret stream/transport identity as logical requester identity.

## 7. Selected Agent handoff custody shape

A future EV/EX-compatible requester handoff must preserve by value, as one transaction custody unit:

- outer requester `request_id` as correlation only;
- existing C03e-EJ session-derived `RequesterRendezvousStartIntent`;
- exact requester `MeshControlStream` custody.

The logical identity rules remain:

`authenticated PRW session identity -> requester identity`

`target DeviceId -> logical rendezvous target identity`

`request_id -> correlation only`

`TransportIdentity -> lower transport evidence only`

`IP/port -> transient reachability only`

No request ID, IP, port, PID, UID, GID, transport handle, or stream handle may fabricate PRW logical identity.

## 8. Selected requester handoff continuation owner

After EX returns a requester/rendezvous handoff barrier, exactly one future Agent-owned continuation transaction may consume that handoff by value.

That continuation owns, for the duration of the requester transaction:

- the correlated start intent;
- the exact requester stream;
- access to the existing authenticated owner needed to invoke C03e-DV;
- caller-supplied current registry, current policy source, requester-rendezvous runtime/provider owner, and verifier-time inputs required by the existing DV API.

The continuation must invoke the existing C03e-DV seam exactly once for the handed-off start intent.

It must not invoke C03e-DN directly and must not bypass C03e-DI/C03e-DP/authorization ordering.

## 9. Correlation and authority separation through DV

The retained `request_id` and retained response stream travel alongside DV execution only for eventual response correlation/custody.

They are not inputs that grant requester-rendezvous authority.

Authority continues to come from:

- authenticated requester identity already embedded through the existing session-derived start intent;
- current registry validation;
- current policy source and authorization;
- the existing requester-rendezvous live provider mutation boundary.

A successful ET decode or retained stream does not imply successful authorization or registration.

## 10. Selected DV terminal-result custody

The future continuation must preserve the exact terminal result of one DV invocation:

- `Ok(())`; or
- the exact `RequesterRendezvousStartCompositionError` failure class.

EY does not flatten all DV failures before a future response-mapping contract decides what, if anything, is externally disclosed.

No DV error may be translated into fabricated provider success, requester completion, candidate selection, transport selection, or dialing success.

## 11. Selected response handoff law

EY selects only the ownership boundary for a later response mapper:

`retained exact request_id + retained exact requester stream + exact DV terminal result -> separately selected requester response mapper -> exactly one response attempt`

The response mapper itself is not selected in EY.

In particular, EY does not select:

- requester response payload bytes;
- requester response operation tags;
- Response versus Error outer-kind mapping;
- detailed versus generic external rejection policy;
- response frame codec API;
- response-write helper API;
- stream shutdown after response;
- response-write retry semantics.

## 12. Repeated-loop resume barrier

A requester/rendezvous handoff remains a barrier after EY selection.

The EX repeated accept loop must not resume merely because DV returned.

A future integration checkpoint may resume authenticated stream acceptance only after a separately selected requester continuation reaches an explicit response-terminal ownership state, or after a separately selected session/peer termination path.

This prevents later streams from being accepted while the requester response transaction remains unresolved.

EY does not activate such resume behavior.

## 13. Failure and close ownership

EY selects no new peer-close code or reason.

The existing capability-specific code-3 and code-4 diagnostics must not be silently widened to requester/rendezvous continuation failures.

Current typed DV failures remain typed continuation results.

Future requester response codec failure, response-write failure, peer-close ownership, and session termination semantics remain separately gated.

No retry, replay, reconnect, replacement stream, or replacement session is selected.

## 14. Serialization and backpressure

The selected continuation remains strictly serial per authenticated owner:

- one requester handoff outstanding at a time;
- no new EX accept while the handoff is unresolved;
- no family-specific queue;
- no detached provider task;
- no detached response writer;
- no speculative acceptance of a later stream.

Backpressure remains the absence of another accept while requester continuation/response custody is outstanding.

## 15. Provider semantics preserved

The existing requester-rendezvous provider registration boundary represents authoritative live registration only.

EY does not claim that DV success returns:

- a selected network candidate;
- a reachable endpoint;
- a relay allocation;
- a transport connection;
- a dial result;
- a requester response payload.

The current DV success shape is `Ok(())`, and EY preserves that fact.

## 16. Candidate-publication codec non-reuse rule

Existing candidate-publication result wire types are scoped to candidate-publication semantic execution.

Their accepted/rejected payloads, freshness token semantics, operation tags, and external projection rules are not requester/rendezvous response semantics.

EY explicitly prohibits reusing those result codecs merely because they also produce Phase 129 `Response`/`Error` control frames.

## 17. Security boundaries preserved

EY does not authorize:

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

## 18. Explicitly excluded from C03e-EY

C03e-EY does not materialize or activate:

- ET requester custody source changes;
- EV requester outcome source changes;
- EX handoff source changes;
- C03e-DV invocation;
- provider execution;
- requester response codec or payload schema;
- requester response frame construction;
- requester response write;
- requester response close semantics;
- response retry/replay/idempotency;
- candidate selection;
- transport selection;
- reachability resolution;
- relay allocation;
- dialing;
- repeated-loop resume;
- runtime integration;
- second acceptor or second read;
- task spawning;
- listener/bootstrap activation;
- Agent `main.rs` activation;
- dependency changes;
- workflow changes;
- Android application changes;
- packaging/systemd changes;
- deployment/restart/recovery;
- merge.

## 19. Selected next source/response checkpoints

The next safe source checkpoint should materialize only the requester response-stream custody repair through ET -> EV -> EX handoff while keeping the continuation uninvoked.

Only after exact source validation should a separate selection checkpoint define requester-rendezvous terminal result projection and exact retained-stream response transaction semantics around the existing DV terminal result.

No checkpoint should activate an endless mixed-family runtime loop until requester continuation reaches a selected response-terminal state.

## 20. Selection closure

C03e-EY is selected when this contract is the only net repository change over exact C03e-EX, exact-head validation is terminal as applicable, durable evidence is recorded, and no runtime/provider/response activation has occurred.

Closure marker:

`CLOSED_REQUESTER_RENDEZVOUS_HANDOFF_CONTINUATION_RESPONSE_STREAM_CUSTODY_SELECTION`

Gate marker:

`C03E_EY_REQUESTER_RENDEZVOUS_HANDOFF_CONTINUATION_RESPONSE_STREAM_CUSTODY_SELECTED`
