# Private Remote Workspace — Phase 152 C03e-F Explicit Pending Session Abort Source Materialization Staging

Status: source/materialization staging
Date: 2026-08-23
Repository: `Gersi365/prw-executor-private`
Exact predecessor branch: `phase-152-c03e-e-registry-bound-session-challenge-source-materialization-staging`
Exact predecessor head: `4537342fc65767b72dfe0c9624119726e351777d`
Exact predecessor tree: `cfac7d5d1893d1824d56e6a50632465343ce26f1`
Predecessor gate: `C03E_E_REGISTRY_BOUND_SESSION_CHALLENGE_SOURCE_MATERIALIZED`

## Purpose

C03e-F materializes the minimal explicit cleanup primitive required before a later C03d challenge/proof wire transaction can safely own a Phase 128 pending challenge across fallible network I/O.

The existing `SessionAuthenticationService` currently removes pending state only after successful proof verification. A send, receive, decode, correlation, or proof failure after C03e-E challenge preparation would otherwise leave the pending `SessionId` resident and duplicate-blocking with no explicit cleanup API.

C03e-F adds only an explicit abort operation to the existing Phase 128 session authority. It does not execute wire I/O, choose retry timing, create a remote session, or use destructor behavior as cleanup evidence.

## Selected primitive

`SessionAuthenticationService` gains:

`abort_pending_session(&mut self, session_id: &SessionId) -> Result<(), SessionServiceError>`

Exact semantics:

1. if `session_id` already names an authenticated session, return the existing `SessionServiceError::SessionAlreadyAuthenticated` and do not mutate authenticated state;
2. otherwise remove exactly that identifier from the private pending map;
3. if no pending entry existed, return the existing `SessionServiceError::UnknownSession`;
4. on success return `Ok(())` after the pending entry has been removed.

No pending challenge, binding, nonce, signature, or internal state is returned to the caller.

## Cleanup invariants

C03e-F selects these invariants:

- cleanup is explicit, synchronous, and verifier-owned;
- `Drop` is not treated as proof that cleanup happened;
- authenticated session state is never removed by this primitive;
- aborting an unknown identifier is visible as an error rather than silently reported as successful cleanup;
- successful abort removes the duplicate-session blocker owned by the current in-memory pending map;
- no tombstone, automatic retry, retry delay, reconnect policy, or new session identifier is created by abort;
- no network resource is closed by this primitive;
- no capability or policy state is changed.

The existing `begin_session` behavior remains authoritative after a successful abort. C03e-F does not itself retry or decide whether a higher-level caller may later submit the same or a different typed `SessionId` for a fresh challenge.

## Failure-path intent for the later wire checkpoint

A separately gated C03d wire-execution owner may later call this primitive after it has successfully prepared a C03e-E challenge but then fails before `submit_proof` commits an authenticated session.

That later checkpoint must still explicitly define:

- exactly which wire/correlation/proof failures trigger abort;
- how an abort failure is surfaced instead of being hidden by the original I/O error;
- when the peer stream/connection is explicitly closed or reset;
- whether any later attempt is allowed and, if so, by which higher-level owner.

C03e-F does not select those orchestration details.

## Expected source mutation

Relative to exact C03e-E, exactly two paths are expected:

1. this contract;
2. `crates/prw-session/src/lib.rs`.

No Agent source, remote-bridge source, registry source, manifest, lockfile, workflow, readiness, or Android application change is required.

## Focused validation

Source validation must prove at minimum:

- only pending state can be removed;
- authenticated state cannot be aborted and remains queryable;
- unknown identifiers return `UnknownSession`;
- successful abort decrements pending count and leaves authenticated count unchanged;
- a proof rejected before abort leaves the pending transaction present, and explicit abort then removes it;
- no challenge/binding/nonce internals are exposed by the abort API;
- existing successful proof semantics remain unchanged;
- canonical Rust and Android validation pass on the exact final source head.

## Protected boundaries

C03e-F must not change:

- `crates/prw-agent/src/remote_transport_runtime.rs`;
- `crates/prw-remote-bridge/src/session_auth_wire.rs`;
- C03e-D accepted-peer implementation;
- `crates/prw-registry/src/lib.rs`;
- C03e `BoundRemoteSession` implementation;
- C03f contract;
- any `Cargo.toml`;
- root `Cargo.lock`;
- `apps/android/native/Cargo.lock`;
- `crates/prw-agent/src/main.rs`;
- existing local Linux runtime/readiness modules;
- workflows;
- Android application source.

## Negative guarantees

C03e-F does not:

- accept or close a network peer;
- open, send, receive, reset, or close a control stream;
- encode/decode PRWM/PRWS;
- retry session authentication;
- generate a replacement challenge itself;
- verify a proof;
- remove authenticated sessions;
- create `AuthenticatedDeviceSession`, `RemoteSessionLease`, or `BoundRemoteSession`;
- materialize the C03f session-capability owner;
- authorize or dispatch capabilities;
- wire `main.rs`;
- spawn tasks/workers/executors;
- publish remote readiness;
- run ICE/STUN/TURN or relay;
- provision credentials;
- mutate systemd/firewall/NAT/routes/DNS/TUN/TAP;
- deploy, restart, rebase, or merge.

## Completion gate

After exact-head canonical Rust/Android validation and Drive closeout:

`C03E_F_EXPLICIT_PENDING_SESSION_ABORT_SOURCE_MATERIALIZED`
