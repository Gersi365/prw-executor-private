# Private Remote Workspace — Phase 152 C03e-G Session Authentication Transaction Failure Semantics Selection Staging

Status: architecture/behavior selection staging
Date: 2026-08-23
Repository: `Gersi365/prw-executor-private`

## Exact predecessor

- branch: `phase-152-c03e-f-pending-session-abort-source-materialization-staging`
- head: `b66baa028b0278e578ef740cc5450f7a8249b0a3`
- tree: `e94332dc469f16f81ef1038fdbf72029d5ce9644`
- gate: `C03E_F_EXPLICIT_PENDING_SESSION_ABORT_SOURCE_MATERIALIZED`

## Purpose

C03e-G selects the deterministic transaction/failure semantics required before source may execute the already-existing C03d logical-session challenge/proof wire exchange after C03e-E has created one pending Phase 128 challenge.

This checkpoint is documentation-only. It does not add wire I/O, peer handling, proof execution, runtime tasks, or session-capability ownership source.

## Existing boundaries composed by the later transaction

The later source checkpoint must compose only these already-materialized boundaries:

1. C03e-D `AuthenticatedRemotePeerConnection` — lower-transport authenticated peer with exact revalidated `TransportIdentity`;
2. C03e-E `AgentRemoteTransportRuntime::begin_registry_bound_session_challenge(...)` — current-registry-bound pending challenge creation;
3. C03d `SessionAuthenticationWireMessage`, `send_session_authentication_message(...)`, and `receive_session_authentication_message(...)` — bounded PRWS-over-PRWM wire encoding and one-frame stream I/O;
4. C03e-F `SessionAuthenticationService::abort_pending_session(...)` — explicit pending-state cleanup;
5. existing `SessionAuthenticationService::submit_proof(...)` — Phase 128 proof authority and authenticated-session commit.

The later transaction must not create a parallel registry, signer, verifier, transport-identity authority, proof verifier, or capability decision path.

## Selected transaction sequence

After C03e-E has successfully returned one `SessionAuthChallenge`, the later Agent-level transaction must perform exactly this sequence on the same accepted peer and the same `SessionAuthenticationService`:

1. accept exactly one peer-initiated bounded control stream through `AuthenticatedRemotePeerConnection::accept_control_stream()`;
2. convert the typed challenge to `SessionAuthenticationWireChallenge`;
3. send exactly one C03d `Challenge` message using one non-zero PRWM request identifier selected by the transaction caller/owner solely as a correlation token;
4. receive exactly one C03d message from the same bidirectional stream;
5. require the returned PRWM request identifier to equal the challenge request identifier exactly;
6. require the decoded message variant to be exactly `Proof`;
7. require `wire_proof.session_id()` to equal the expected typed `SessionId::as_str()` exactly before constructing a typed proof;
8. construct `SessionAuthProof` with the already-selected expected typed `SessionId`, the wire proof nonce, and the wire proof public signature;
9. call the existing `SessionAuthenticationService::submit_proof(expected_session_id, proof, now_unix_seconds)` exactly once;
10. return the resulting existing `AuthenticatedDeviceSession` only after proof verification succeeds.

Nonce/replay/time/signature verification remains owned by the existing Phase 128 service. C03e-G does not select a duplicate nonce or cryptographic verification layer in Agent code.

## Request identifier semantics

The PRWM request identifier is only a bounded wire-correlation value.

It is not:

- a logical `SessionId`;
- a transport identity;
- a capability grant;
- a registry generation;
- an authorization token;
- a retry token.

The later source seam may accept one caller-selected non-zero `u64` request identifier because the existing C03d/PRWM frame constructor already rejects invalid request identifiers. No new request-ID allocator is selected by C03e-G.

## Selected failure matrix

Once C03e-E has successfully created the pending challenge, **every terminal failure before `submit_proof` succeeds** owns the same cleanup obligation.

Failures requiring cleanup include:

- peer control-stream acceptance failure;
- challenge wire-frame construction/encoding failure;
- challenge send/write/finish failure;
- proof receive/read failure;
- PRWM/PRWS decode/validation failure;
- request-identifier mismatch;
- receiving a `Challenge` or any non-`Proof` message where proof is required;
- wire proof session-identifier mismatch;
- Phase 128 proof rejection, including nonce, time, replay-context, public-key, or signature rejection represented by existing session-service errors.

For each such primary failure, the later source must:

1. call `abort_pending_session(expected_session_id)` exactly once;
2. explicitly close the entire `AuthenticatedRemotePeerConnection` before returning, because the current `MeshControlStream` public surface exposes bounded send/receive but no explicit stream reset/stop primitive while the peer handle exposes explicit connection close;
3. return a failure that preserves the primary failure classification;
4. if abort itself fails, preserve and surface that cleanup failure in addition to the primary failure rather than hiding it.

The peer connection is closed on both cleanup success and cleanup failure. Connection close currently returns no fallible result, so C03e-G selects no invented close-success claim beyond invoking the existing explicit close method.

## Successful transaction semantics

If and only if `submit_proof(...)` succeeds:

- the existing Phase 128 service has already removed the pending challenge and committed one `AuthenticatedDeviceSession`;
- no C03e-F abort call occurs;
- the authenticated peer connection remains open for a later separately gated post-authentication lifecycle;
- the completed authentication stream may fall out of scope after its one challenge/send and one proof/receive exchange; no additional message is selected here;
- no capability is authorized merely because authentication succeeded.

## Cleanup-failure representation

A later source checkpoint must use an explicit failure value capable of retaining both:

- the primary transaction failure; and
- an optional `SessionServiceError` returned by `abort_pending_session`.

C03e-G does not permit cleanup failure to be logged-and-discarded, replaced by the primary error, or converted into success.

A concrete Rust enum/struct name is intentionally deferred to the source-materialization checkpoint, but the two-part information requirement is locked by this selection.

## Retry and replacement-challenge policy

The transaction itself performs **no retry**.

After a failure:

- the peer is closed;
- no replacement challenge is generated by the transaction;
- no automatic reconnect occurs;
- no new `SessionId` is allocated;
- no same-connection retry is permitted.

A higher-level future owner may decide whether to accept a new peer and begin a fresh challenge only after the prior cleanup outcome is explicit. If cleanup failed, C03e-G selects no retry eligibility; the ambiguous/remaining pending state must be handled by a separately reviewed owner.

## Failures before pending challenge creation

C03e-E registry/challenge-preparation failures occur before this transaction owns a pending challenge and therefore do not call C03e-F abort through this selected wire-transaction failure path.

This prevents an Agent wrapper from fabricating cleanup for a challenge that was never successfully created.

## Post-authentication stopping point

C03e-G stops at one existing `AuthenticatedDeviceSession`.

It does not yet select or materialize:

- `RemoteSessionLease` construction timing;
- C03e `BoundRemoteSession` construction;
- C03f-selected `RemoteSessionCapabilityRuntimeOwner` source;
- capability processing loop;
- concurrent stream model;
- session registry/task ownership;
- session refresh/re-authentication;
- remote readiness.

Those remain separately gated downstream work.

## Fixed peer-close reason ownership

The later source checkpoint should use one private, fixed application close code and one private, fixed bounded reason for logical-session transaction failure. These values are diagnostics only and must not encode secrets, proof bytes, session identifiers, device identifiers, registry data, or user-controlled strings.

The exact numeric code/string may be selected in the source checkpoint together with tests; C03e-G locks only that they are fixed, private, bounded, and non-secret rather than caller-controlled.

## Protected boundaries

C03e-G is documentation-only and must keep byte-stable:

- all Rust source;
- all Cargo manifests;
- root `Cargo.lock`;
- `apps/android/native/Cargo.lock`;
- Agent `main.rs`;
- workflows;
- Android application source;
- systemd/networking/packaging source.

## Negative guarantees

C03e-G does not:

- accept a network peer or stream;
- send or receive PRWM/PRWS bytes;
- abort a real pending session;
- close a real peer;
- verify a proof;
- create `AuthenticatedDeviceSession`, `RemoteSessionLease`, or `BoundRemoteSession`;
- materialize C03f ownership source;
- authorize or dispatch capabilities;
- add retry/reconnect/session refresh;
- wire `main.rs`;
- spawn tasks/workers/executors;
- publish remote readiness;
- activate ICE/STUN/TURN/relay;
- provision credentials;
- mutate systemd/firewall/NAT/routes/DNS/TUN/TAP;
- initialize PRWF, run recovery epochs, or activate R1-R4 effects;
- deploy, restart, rebase, or merge.

## Completion gate

After exact-head documentation validation and Drive closeout:

`C03E_G_SESSION_AUTH_TRANSACTION_FAILURE_SEMANTICS_SELECTED`
