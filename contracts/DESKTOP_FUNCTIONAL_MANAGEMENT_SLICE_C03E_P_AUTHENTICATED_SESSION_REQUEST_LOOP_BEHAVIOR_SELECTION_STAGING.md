# Phase 152 C03e-P — Authenticated Session Request Loop Behavior Selection Staging

Status: STAGED SELECTION ONLY

Target gate:

`C03E_P_AUTHENTICATED_SESSION_REQUEST_LOOP_BEHAVIOR_SELECTED`

## Predecessor

Canonical predecessor is closed C03e-O:

- branch: `phase-152-c03e-o-single-capability-request-transaction-source-materialization-staging`
- head: `31fd797c5c647a0e24c37505d247bac76aed51f1`
- tree: `3ceda6aedda84adeb220fb5e98adfcc01603ac5b`
- gate: `C03E_O_SINGLE_CAPABILITY_REQUEST_TRANSACTION_SOURCE_MATERIALIZED`

C03e-P is a sibling-free continuation of exact O lineage. It does not merge, rebase or cherry-pick any other checkpoint.

## Purpose

Select, but do not yet materialize, the first authenticated remote application-session request-loop behavior over the C03e-O one-stream/one-request transaction.

The selection deliberately resolves:

- whether successful transactions continue;
- whether each existing transaction failure is request-local or session-terminal;
- whole-peer close behavior on terminal failure;
- verifier-time sampling across repeated transactions;
- whether retries, replacement streams or re-authentication occur;
- what remains outside this loop boundary.

No Rust source is changed by C03e-P.

## Selected loop owner

The future loop belongs to the existing Agent-owned `AuthenticatedRemoteSessionRuntimeOwner` that already retains:

- one live `AuthenticatedRemotePeerConnection`;
- one `RemoteSessionCapabilityRuntimeOwner` carrying exactly one existing `BoundRemoteSession`.

The loop is a borrowed operation on that same owner. It does not create a second peer, logical session, capability owner, application-session lease or binding.

The future source seam must use `&mut self`, preserving strict serialization through this owner API. C03e-P does not select multiple simultaneous request loops for one owner.

## Selected success behavior

One loop iteration invokes exactly one C03e-O `process_one_capability_request(...)` transaction.

Only `Ok(())` from that transaction permits another iteration.

A successful iteration therefore means all of the following already completed:

1. one bounded control stream was accepted;
2. one bounded PRWM request frame was received;
3. current retained binding/lease/registry/transport/policy authority admitted the request;
4. the dispatcher completed;
5. one correlated bounded success response frame was sent and its send direction was finished.

Only after those conditions does the loop proceed to accept another stream.

There is no pipelining and no overlapping iteration.

## Selected verifier-time behavior

The loop must not cache one verifier timestamp for the connected session and must not read hidden process-global wall-clock state internally.

The future source seam must receive an explicit caller-owned verifier-time provider with the equivalent contract of:

`FnMut() -> u64 + Send`

The loop calls that provider exactly once immediately before each C03e-O one-request transaction and passes that returned Unix-seconds value as the transaction's explicit verifier time.

The loop never derives verifier time from:

- authentication challenge issue/expiry time;
- QUIC handshake time;
- application-session lease issue time;
- request identifiers;
- system uptime;
- a cached prior iteration value;
- hidden `SystemTime::now()` inside the session loop.

The caller remains responsible for supplying authoritative verifier-owned Unix time. C03e-P adds no clock authority of its own.

## Selected failure rule — every transaction error is session-terminal

C03e-P deliberately selects a conservative rule:

**every `AuthenticatedRemoteSessionCapabilityTransactionError` returned by C03e-O is terminal for the connected authenticated remote application session.**

The loop does not attempt to classify an error as recoverable and does not continue to another stream after any transaction error.

This applies equally to all three C03e-O classes:

- `Accept(RemoteServerTransportRuntimeError)`;
- `Wire(CapabilityRequestWireError)`;
- `Bridge(RemoteBridgeError)`.

This strict selection is intentional because the current protocol has no separately selected negative capability-response envelope, request retry contract or safe per-stream recovery protocol.

## Exact `RemoteBridgeError` consequence

Under this selection, every existing `RemoteBridgeError` variant is session-terminal when it emerges from C03e-O, including:

- `InvalidSessionLease`;
- `SessionNotYetValid`;
- `SessionExpired`;
- `WrongControlMessageKind`;
- `RegistryRejected`;
- `TransportIdentityRejected`;
- `InvalidRequestPayload`;
- `CapabilityDenied`;
- `DispatchFailed`;
- `DispatchResponseTooLarge`;
- `ResponseFrameRejected`.

C03e-P intentionally does **not** create a special continue-after-`CapabilityDenied` or continue-after-dispatch-failure path.

A later protocol version may separately select a typed negative response and recoverable per-request semantics, but this checkpoint does not assume one.

## Selected whole-peer close behavior

On the first transaction failure, the future loop must:

1. retain the exact primary C03e-O typed error;
2. explicitly close the same retained authenticated peer exactly once;
3. use fixed non-secret application close code `3`;
4. use fixed non-secret reason bytes `b"remote capability session terminated"`;
5. return the original C03e-O transaction error unchanged after issuing close.

The close reason must not interpolate request content, device identity, path, capability, policy decision, dispatcher output or lower transport diagnostics.

Existing close-code allocation is preserved:

- `1`: logical-session authentication transaction failure;
- `2`: post-authentication remote-session binding failure;
- `3`: authenticated capability-session request-loop terminal failure selected here.

`AuthenticatedRemotePeerConnection::close(...)` is synchronous and infallible at the current API boundary, so C03e-P selects no secondary cleanup-error envelope.

## Accept failure semantics

Every stream-accept failure is terminal.

This includes the currently reachable lower cases wrapped by `RemoteServerTransportRuntimeError`, including operation timeout and accept-stream failure.

The loop does not retry `accept_control_stream()`, does not wait for a replacement peer and does not re-enter logical-session authentication.

An accept timeout therefore acts as a bounded idle/accept failure for this first loop shape and terminates the connected session.

## Wire failure semantics

Every C03e-N receive/send/finish failure is terminal.

The loop does not:

- retry a frame read;
- re-send a success response;
- accept a replacement stream for the same request;
- attempt to infer whether a lower transport error was malicious or transient;
- continue after malformed/oversized/invalid PRWM input.

The same peer is closed once with the selected generic capability-session termination diagnostic.

## Authorization and identity semantics remain unchanged

C03e-P creates no new authorization rule.

Each successful iteration still delegates through C03e-O to retained `BoundRemoteSession::process_request(...)`, which supplies its retained transport identity and application-session lease internally and invokes current `CapabilityBridge` authority.

Every request therefore remains subject to current:

- lease validation at the explicit verifier time for that iteration;
- authenticated-session registry validity;
- logical-device to retained `TransportIdentity` binding validity;
- PRWC decoding and exact capability derivation;
- selected policy evaluation;
- dispatcher execution;
- bounded correlated success-response construction.

`DeviceId` / authenticated PRW session identity remains logical identity. `TransportIdentity` remains transport identity only. IP, PID, UID and GID remain non-authoritative for logical identity.

## No retry or replacement authority

C03e-P selects no automatic retry anywhere in the loop.

After a transaction failure there is no:

- repeated execution of the same capability request;
- new request identifier fabrication;
- replacement control stream;
- replacement transport identity;
- replacement logical session;
- new application-session lease;
- pending-session abort after authentication already succeeded;
- authenticated-session deletion;
- reconnect;
- re-authentication.

A new connection/session, if ever desired after termination, must originate from a separately owned outer admission flow.

## No error-response protocol invention

C03e-P does not create an application-level negative response frame.

On failure, the first loop shape terminates the connected session instead of synthesizing a success-shaped or ad-hoc error payload.

Therefore no capability error string, numeric bridge-error code or dispatcher diagnostic is placed on the capability stream by this selection.

## No concurrency selection

The first loop remains strictly serial:

- one owner mutable borrow;
- one accepted control stream at a time;
- one C03e-O transaction at a time;
- one dispatcher mutation at a time through the supplied dispatcher borrow;
- next accept only after prior success response send completes.

C03e-P does not select per-stream tasks, parallel dispatch, request ordering across tasks, maximum in-flight request count or fairness scheduling.

## Cancellation and draining deliberately remain separate

C03e-P does not select task ownership, external cancellation token shape, graceful drain deadline or join semantics.

A future source loop may be a borrowed async operation that runs until its first transaction failure; caller-driven cancellation of that future is not given a new protocol meaning by this checkpoint.

Before production task ownership/readiness wiring, a separate checkpoint must select explicit cancellation/drain/close ownership so cancellation cannot silently become remote readiness or lifecycle authority.

## Selected next source shape

A later source-materialization checkpoint may add a method equivalent to:

- borrowed `&mut AuthenticatedRemoteSessionRuntimeOwner`;
- current `&CapabilityBridge`;
- caller-owned `FnMut() -> u64 + Send` verifier-time provider;
- caller-owned mutable `CapabilityDispatcher + Send`;
- serial `loop` around exactly one existing C03e-O transaction;
- continue only on `Ok(())`;
- on first `Err(error)`, close same peer once using code `3` / `remote capability session terminated` and return `error` unchanged.

That source checkpoint must not add task spawning, `main.rs`, readiness or a direct `prw-remote-transport` dependency to Agent.

## Source scope of this checkpoint

C03e-P itself is docs-only.

Expected diff is exactly this contract file.

No mutation is selected for:

- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`;
- any Agent or bridge Cargo manifest;
- `Cargo.lock`;
- Agent `lib.rs` or `main.rs`;
- C03e-N wire adapter;
- remote-session binding/lease;
- logical-session authentication;
- registry/policy/dispatcher implementations;
- workflows;
- Android application source;
- remote readiness;
- systemd/packaging;
- host-network/reachability activation.

## Validation requirement

Because C03e-P is docs-only, canonical completion requires exact-head repository validation that is actually triggered by this diff.

Any skipped workflow must remain recorded as skipped and must not be claimed as PASS.

No Android PASS is required or claimed if the Android workflow is not triggered by the docs-only path.

## Drive closeout requirement

After exact-head canonical validation passes:

1. publish immutable `C03E_P_AUTHENTICATED_SESSION_REQUEST_LOOP_BEHAVIOR_SELECTION_AUDIT.md` in the existing evidence folder;
2. raw-readback verify its exact byte size and SHA-256;
3. immediately re-fetch authoritative rolling `C02E_BRANCH_STATUS.md` and require the exact closed-O baseline;
4. append P evidence only, preserving every predecessor byte;
5. raw-readback verify post-P size/hash and the entire closed-O prefix hash;
6. update the P PR to CLOSED checkpoint metadata while keeping it draft/open/unmerged.

## Deliberate stopping point

Even after this selection closes, these remain separately gated:

- actual request-loop source materialization;
- explicit cancellation/drain/join lifecycle;
- concurrent request/session task ownership;
- typed negative capability response protocol;
- recoverable per-request error semantics;
- Agent `main.rs` runtime wiring;
- remote readiness publication;
- listener/reachability runtime activation;
- external NAT/ICE/STUN/TURN/relay integration;
- credential provisioning;
- deployment/restart/merge.

Gate on successful canonical closeout:

`C03E_P_AUTHENTICATED_SESSION_REQUEST_LOOP_BEHAVIOR_SELECTED`
