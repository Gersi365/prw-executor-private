# Private Remote Workspace — Phase 152 C03e-H Session Authentication Transaction Source Materialization Staging

Status: source-materialization staging
Date: 2026-08-23
Repository: `Gersi365/prw-executor-private`

## Exact predecessor

- branch: `phase-152-c03e-g-session-auth-transaction-failure-semantics-selection-staging`
- head: `55406e09786aa8048bdd7eae27af05566f1fc8ad`
- tree: `a05452dfa072e53f64158d4ee498f824a19801a9`
- gate: `C03E_G_SESSION_AUTH_TRANSACTION_FAILURE_SEMANTICS_SELECTED`

## Purpose

C03e-H materializes the C03e-G-selected Agent-owned execution seam for one already-prepared logical-session challenge/proof transaction.

The new source is not runtime activation. Nothing in C03e-H calls it from `main.rs`, spawns a worker, publishes remote readiness, creates a `RemoteSessionLease`, constructs C03e `BoundRemoteSession`, or materializes the C03f-selected session-capability owner.

## Source placement

C03e-H adds one dedicated Agent module:

`crates/prw-agent/src/remote_session_authentication_transaction.rs`

and exports that module from `crates/prw-agent/src/lib.rs`.

The existing C03e-C/D/E `remote_transport_runtime.rs` remains byte-stable. This keeps endpoint ownership, peer acceptance, and registry-bound challenge preparation separate from the one-stream logical-session transaction.

## Materialized entry point

The source adds:

`complete_registry_bound_session_authentication(...)`

Its inputs are exactly:

- `&AgentRemoteTransportRuntime` — retains the authority-gated Agent endpoint owner for the transaction lifetime;
- `&AuthenticatedRemotePeerConnection` — the same lower-transport-authenticated peer from C03e-D;
- `&mut SessionAuthenticationService` — the same Phase 128 service that owns the pending challenge;
- `&SessionAuthChallenge` — the already-prepared C03e-E challenge;
- caller-selected non-zero-intended PRWM `u64` request identifier used only for correlation;
- verifier-owned `now_unix_seconds` supplied to existing Phase 128 proof verification.

The function returns the existing `AuthenticatedDeviceSession` only after successful proof verification.

C03e-H does not accept a caller-supplied `DeviceIdentityBinding`, `TransportIdentity`, public key, verifier implementation, policy evaluator, capability set, dispatcher, lease, or bound remote session.

## Exact transaction sequence

The materialized function performs the C03e-G sequence directly:

1. retain the supplied authority-gated Agent runtime borrow for the transaction;
2. derive the expected typed `SessionId` from the supplied prepared challenge;
3. accept exactly one peer-initiated bounded control stream through the C03e-D peer handle;
4. convert the prepared typed challenge to the existing C03d wire challenge;
5. send exactly one C03d `Challenge` message with the supplied PRWM request identifier;
6. receive exactly one C03d logical-session message from the same bidirectional stream;
7. require exact returned request-identifier equality;
8. require the returned C03d message variant to be `Proof`;
9. require the wire proof session identifier to equal the expected typed challenge session identifier exactly;
10. construct the existing `SessionAuthProof` with the already-selected expected typed `SessionId`, wire nonce and wire signature;
11. call existing `SessionAuthenticationService::submit_proof(...)` exactly once;
12. return its existing `AuthenticatedDeviceSession` on success.

No duplicate nonce, replay, verifier-time, public-key, signature, or canonical-message validation is added in Agent source. Those checks remain owned by the existing Phase 128 service and device-identity verifier.

## Request identifier behavior

C03e-H does not add a new request-ID type or allocator.

The supplied `u64` is passed unchanged to the existing C03d/PRWM constructor. Existing PRWM validation rejects zero. Because C03e-H executes only after a pending challenge already exists, an invalid request identifier is a C03d wire construction failure and therefore follows the selected C03e-G cleanup path.

The request identifier is not interpreted as a session identifier, transport identity, authorization token, retry token, registry generation, or capability grant.

## Materialized failure model

C03e-H adds `AgentRemoteSessionAuthenticationPrimaryError` with narrow primary classes:

- lower transport/control-stream acceptance failure;
- C03d wire construction/I/O/decode failure;
- PRWM request-identifier mismatch;
- unexpected logical-session message variant;
- wire logical-session identifier mismatch;
- existing Phase 128 session-service proof failure.

C03e-H also adds `AgentRemoteSessionAuthenticationFailure`, which retains separately:

- the primary failure; and
- optional `SessionServiceError` from explicit C03e-F cleanup.

The cleanup error is never converted into success or discarded.

## Materialized cleanup behavior

For every terminal failure after the prepared pending challenge exists, C03e-H calls:

`SessionAuthenticationService::abort_pending_session(expected_session_id)`

exactly once, then invokes the existing authenticated peer `close(...)` with one private fixed non-secret code/reason.

The fixed close diagnostic is not caller-controlled and contains no session identifier, device identifier, proof bytes, signature bytes, registry data, credential material, or other user-controlled content.

The peer is explicitly closed whether abort succeeds or fails. The existing peer close method is infallible at this API boundary, so C03e-H makes no invented asynchronous close-acknowledgement claim.

No retry, replacement challenge, reconnect, new `SessionId`, same-connection retry, or cleanup loop is performed.

## Success behavior

If existing `submit_proof(...)` succeeds:

- the existing Phase 128 service has committed exactly one `AuthenticatedDeviceSession` and removed the pending challenge;
- C03e-H does not call abort;
- C03e-H does not close the authenticated peer;
- the one challenge/proof stream is no longer used by this function;
- the peer remains available for a later separately gated post-authentication lifecycle;
- no capability is authorized merely because authentication succeeded.

## Authority and identity separation

C03e-H preserves the existing separation:

- reachability authority remains owned by `AgentRemoteTransportRuntime`;
- lower transport identity remains inside `AuthenticatedRemotePeerConnection`;
- logical device/session identity remains Phase 128 session-authentication state;
- current registry binding remains C03e-E challenge-preparation responsibility;
- capability authorization remains the existing `CapabilityBridge` per-request responsibility downstream.

C03e-H does not re-validate registry state after the pending challenge was prepared, because the current checkpoint only materializes the selected challenge/proof transaction. Current-registry capability authorization remains independently revalidated later by the existing bridge.

## Validation scope

The source checkpoint adds compile/API tests that lock:

- the transaction function remains a distinct Agent module surface;
- primary and cleanup errors remain separately observable;
- the private peer-close diagnostic remains fixed, non-zero and non-empty.

C03e-H does not claim a new full real-loopback end-to-end session-authentication integration test. A separately gated integration-validation checkpoint may exercise success/failure network paths before any runtime wiring or post-authentication owner composition.

## Exact intended diff

Relative to exact C03e-G head `55406e09786aa8048bdd7eae27af05566f1fc8ad`, C03e-H is restricted to exactly these three paths:

1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_H_SESSION_AUTH_TRANSACTION_SOURCE_MATERIALIZATION_STAGING.md`
2. `crates/prw-agent/src/lib.rs`
3. `crates/prw-agent/src/remote_session_authentication_transaction.rs`

The `lib.rs` change is exactly one public module export.

## Protected byte-stable boundaries

C03e-H must not modify:

- `crates/prw-agent/src/remote_transport_runtime.rs`;
- C03d `crates/prw-remote-bridge/src/session_auth_wire.rs`;
- C03e `crates/prw-remote-bridge/src/remote_session_binding.rs`;
- `crates/prw-session/src/lib.rs` after C03e-F;
- registry source;
- any Cargo manifest;
- root `Cargo.lock`;
- `apps/android/native/Cargo.lock`;
- Agent `main.rs`;
- workflows;
- Android application source;
- local Linux runtime/readiness source;
- reachability-authority source;
- packaging/systemd/networking source.

## Deliberate stopping point

C03e-H stops at one successful existing `AuthenticatedDeviceSession` or one explicit transaction failure.

Still not materialized:

- full real-loopback transaction integration validation;
- `RemoteSessionLease` construction timing;
- C03e `BoundRemoteSession` construction;
- C03f `RemoteSessionCapabilityRuntimeOwner` source on the authoritative integration line;
- capability request loop;
- concurrency/task/session registry model;
- re-authentication/session refresh;
- remote readiness.

## Negative guarantees

C03e-H does not:

- wire the transaction into `main.rs`;
- create a peer accept loop;
- spawn tasks/workers/executors;
- retry authentication;
- create a replacement challenge;
- implement reconnect/session refresh;
- create a `RemoteSessionLease`;
- construct C03e `BoundRemoteSession`;
- materialize or compose C03f ownership;
- authorize or dispatch capabilities;
- publish remote readiness;
- activate ICE/STUN/TURN/relay;
- provision or rotate credentials;
- mutate systemd/firewall/NAT/routes/DNS/TUN/TAP;
- initialize PRWF, execute recovery epochs, or activate R1-R4 effects;
- deploy, restart, rebase, or merge.

## Completion gate

After exact-scope verification, exact-head canonical validation, immutable Drive audit publication, and append-only rolling-status closeout:

`C03E_H_SESSION_AUTH_TRANSACTION_SOURCE_MATERIALIZED`
