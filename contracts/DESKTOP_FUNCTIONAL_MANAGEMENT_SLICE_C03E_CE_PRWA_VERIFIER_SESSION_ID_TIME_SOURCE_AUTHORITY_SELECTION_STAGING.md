# Phase 152 C03e-CE — PRWA Verifier SessionId + Time Source Authority Selection

Status: `STAGED SELECTION — PENDING EXACT-HEAD VALIDATION`

Target gate:
`C03E_CE_PRWA_VERIFIER_SESSION_ID_TIME_SOURCE_AUTHORITY_SELECTED`

## 1. Exact predecessor authority

C03e-CE branches only from the exact closed C03e-CD checkpoint:

- branch: `phase-152-c03e-cd-prwc-pre-mesh-authentication-pure-codec-lock-synchronized-source-materialization-staging`;
- head: `4774517b9c7625a83eb0a06791e4b4f60a9487af`;
- tree: `3e967d89215c84a680fcb7214cd8f88c0e1cdd03`;
- gate: `C03E_CD_PRWC_PRE_MESH_AUTHENTICATION_PURE_CODEC_LOCK_SYNCHRONIZED_SOURCE_MATERIALIZED`;
- PR #201 body is `Status: CLOSED`, while the PR remains draft/open/unmerged.

C03e-CD materialized only the pure in-memory PRWA v1.0 codec plus the synchronized Android native lock edge. It did not select or materialize a verifier-side `SessionId` generator, verifier clock source, server/listener/accepted-stream implementation, frame loop, requester/rendezvous provider, candidate-publication execution, runtime activation, networking, deployment, restart/recovery, or merge.

## 2. Post-CD readiness evidence

The immutable post-closure readiness re-audit is:

`C03E_CD_POST_CLOSURE_PRWC_AUTH_SERVER_RUNTIME_PREREQUISITE_REAUDIT.md`

Drive ID:
`1qsCC4Uqojzu_m_xYLrDhNDdkGYoOtZj1`

The audit established two separately gated post-CD prerequisites:

1. verifier-side `SessionId` + verifier-time source authority;
2. generic Phase 129 PRWC server/listener/accepted-stream plus bridge runtime execution authority.

C03e-CE addresses only prerequisite (1). It does not select or authorize prerequisite (2).

## 3. Existing semantic authority preserved

The following existing authorities remain unchanged:

- `SessionAuthenticationService::begin_session(...)` remains the authority that accepts one already-selected typed `SessionId`, generates the exact 32-byte challenge nonce, validates the challenge lifetime, rejects duplicate session identifiers, and records pending challenge state;
- `SessionAuthenticationService::submit_proof(...)` remains the authority for nonce/session/time/public-key/signature verification and successful `AuthenticatedDeviceSession` creation;
- `WorkspaceDeviceRegistry` remains the current membership/device/identity revalidation authority;
- PRWA `Begin` carries only an untrusted `DeviceId` selector;
- PRWC `request_id` remains outer per-connection correlation only under the separately selected C03e-BY lifecycle;
- `TransportIdentity` remains a separately rotatable lower-transport certificate identity;
- successful PRWA decode remains structural/type validation only and is never authentication success.

C03e-CE does not move any of these authorities into the new source boundary.

## 4. Why an explicit verifier source is required

C03e-CA selected the semantic sequence:

1. decode `Begin`;
2. resolve the current server-side enrolled-device binding;
3. obtain a separately authorized server-side typed `SessionId` plus verifier issue/expiry times;
4. call the existing `SessionAuthenticationService::begin_session(...)`;
5. emit only the resulting typed challenge fields.

C03e-CD materialized the codec but intentionally did not close step 3.

The earlier PRWM C03e-E precedent also required a caller-supplied typed `SessionId` and verifier-owned time range. It therefore does not provide a production generator/clock authority that can be silently reused.

## 5. Selected ownership boundary

C03e-CE selects one narrow verifier source authority at the session-authentication semantic layer.

A future source materialization may expose a small server-side helper in the `prw-session` crate that returns exactly:

- one freshly generated typed `SessionId`;
- one verifier-owned `issued_at_unix_seconds`;
- one verifier-owned `expires_at_unix_seconds`.

The helper is not a session registry, connection owner, transport owner, authentication service replacement, capability authority, requester/rendezvous provider, or persistent database.

The future Phase 129 bridge runtime may invoke this source once when processing one admissible PRWA `Begin`, then pass the returned values into the unchanged `SessionAuthenticationService::begin_session(...)` path.

This preserves the C03e-BX layering:

`product surface -> prw-remote-bridge runtime composition -> semantic session authority -> generic transport primitives`

without promoting generic PRWC transport into session-identity authority.

## 6. Selected SessionId generation profile

Each new PRWA verifier session identifier is generated from exactly 32 cryptographically secure random bytes.

Selected representation:

- entropy input: exactly 32 bytes / 256 bits;
- encoding: lowercase hexadecimal ASCII;
- encoded length: exactly 64 bytes;
- alphabet: `0-9a-f` only;
- no prefix, suffix, timestamp, counter, host name, process ID, device identifier, request identifier, transport identifier, workspace/user identifier, candidate identifier, endpoint, or freshness token is embedded;
- the encoded value must be reconstructed through the existing `SessionId::new(...)` constructor before use.

The value is opaque. Consumers must not parse semantic meaning from its textual representation.

## 7. Selected randomness authority

The SessionId randomness source must be cryptographically secure and OS-backed.

The preferred implementation boundary is the already-established production cryptographic provider family used by `prw-session` for challenge nonces (`aws_lc_rs::rand::SystemRandom`). C03e-CE does not authorize a second RNG family, custom PRNG, timestamp-derived identifier, deterministic hash-derived identifier, or product/UI-supplied identifier.

Because `prw-session` already has the production `aws-lc-rs` dependency, a later narrowly scoped source materialization should not require introducing a new RNG dependency solely for this SessionId source.

Randomness acquisition failure is terminal fail-closed for that authentication attempt.

## 8. SessionId collision and retry semantics

The verifier source performs one generation attempt per admissible PRWA `Begin`.

If the generated typed `SessionId` is rejected by the current `SessionAuthenticationService` as already pending or authenticated, the authentication transaction fails closed.

C03e-CE selects:

- no hidden collision retry loop;
- no replacement SessionId on the same authentication transaction;
- no new request ID;
- no same-connection reauthentication;
- no fallback to a counter/time/device-derived identifier.

This preserves C03e-CA's no-retry transaction semantics and keeps failure behavior deterministic.

## 9. Persistence and restart semantics

The selected SessionId source is non-persistent.

Rationale:

- SessionId is authentication-session correlation, not a durable account/device identifier;
- Phase 128 pending/completed session state remains in-memory at the current authority boundary;
- BZ selects connection-local authenticated-session ownership;
- a restarted verifier does not retain old pending authentication transactions or connection-local bindings;
- 256-bit cryptographic randomness provides a fresh namespace without a persistent sequence counter.

C03e-CE therefore does not authorize a SessionId table, database sequence, durable counter, cross-host allocator, lease service, distributed lock, or restart recovery protocol.

No claim of mathematically impossible cross-restart collision is made. The authority is probabilistic 256-bit uniqueness plus fail-closed duplicate rejection inside the live session service.

## 10. Selected verifier clock source

Verifier authentication time is sourced only from the server/verifier system wall clock as Unix time in whole seconds.

Selected representation:

`SystemTime::now()` -> checked duration since `UNIX_EPOCH` -> checked `u64` whole seconds.

The client/publisher never supplies verifier issue time, expiry time, or proof-verification `now` authority.

Failures to obtain a representable Unix timestamp, including a pre-epoch value, fail closed before pending session state is created.

## 11. Selected challenge lifetime

C03e-CE selects one fixed PRWA verifier challenge lifetime:

`300 seconds`

This reuses the existing locked Phase 128 maximum and introduces no new configuration surface.

For each accepted authentication attempt:

- `issued_at_unix_seconds` is one server clock observation immediately before `begin_session(...)`;
- `expires_at_unix_seconds = issued_at_unix_seconds + 300` using checked arithmetic;
- overflow fails closed;
- the existing `SessionAuthenticationService` remains responsible for validating that the resulting lifetime is positive and within the locked Phase 128 bound.

No client-supplied lifetime, per-device lifetime, per-request lifetime, UI-configured lifetime, environment-variable override, adaptive lifetime, or retry extension is selected.

## 12. Proof-verification time

Later PRWA transaction execution must obtain `now_unix_seconds` for `SessionAuthenticationService::submit_proof(...)` from the same verifier wall-clock authority selected here.

A fresh clock observation is required at proof submission. The issue timestamp is not reused as proof-verification `now`.

If the wall clock moves backward such that `now` is before the recorded issue time, or forward to/after expiry, the existing Phase 128 validation fails closed. C03e-CE does not add clock-skew tolerance, grace windows, timestamp rewriting, monotonic-clock substitution, or retry.

## 13. Exact authority ordering after C03e-CE

A later runtime composition must preserve this order:

1. receive one PRWA `Begin` under one non-zero PRWC request ID;
2. treat its `DeviceId` only as an untrusted lookup selector;
3. resolve the exact current server-side enrolled-device binding under the separately selected registry/transport rules;
4. invoke the C03e-CE verifier source to obtain one fresh typed SessionId and the exact 300-second verifier time window;
5. call `SessionAuthenticationService::begin_session(...)` exactly once;
6. emit the resulting PRWA `Challenge` under the same request ID;
7. receive exactly one correlated PRWA `Proof`;
8. construct the existing typed `SessionAuthProof`;
9. obtain a fresh verifier wall-clock `now` from the same C03e-CE clock authority;
10. call `SessionAuthenticationService::submit_proof(...)` exactly once;
11. revalidate the returned authenticated session against current registry authority;
12. only after all selected semantic checks succeed may PRWA `Authenticated` be emitted and connection-local authenticated-session state become usable.

## 14. Explicit forbidden derivations

Neither SessionId nor verifier time may be derived from, controlled by, or replaced by:

- PRWC `request_id`;
- PRWA payload bytes;
- `DeviceId`;
- `WorkspaceId`;
- `UserId`;
- `TransportIdentity`;
- public-key bytes or signatures;
- challenge nonce;
- candidate IDs/endpoints/path kinds;
- publication freshness tokens;
- requester/rendezvous state;
- socket addresses;
- product/UI identifiers;
- local IPC request IDs;
- PRWM request IDs.

## 15. Failure semantics

Before pending state exists, any SessionId randomness failure, SessionId construction failure, verifier-time failure, or checked expiry overflow fails closed with no pending-session cleanup required.

After `begin_session(...)` succeeds, C03e-CA's existing failure rule remains authoritative:

- every terminal failure before successful proof commit must abort the pending session exactly once;
- no internal retry or replacement challenge is permitted;
- terminal failure attempts generic PRWA `Rejected`, then discards the unauthenticated connection;
- detailed external authentication-oracle errors remain forbidden.

C03e-CE does not select the concrete I/O cleanup implementation.

## 16. Source-materialization boundary for a later checkpoint

C03e-CE itself is docs-only.

A separately authorized source-materialization successor may be narrowly scoped to a pure/server-local verifier source, preferably inside `prw-session`, plus focused tests and one successor contract.

Expected implementation properties:

- no network I/O;
- no listener/socket/stream ownership;
- no database or persistence;
- no product/UI integration;
- no request-ID allocator;
- no registry mutation;
- no capability authorization;
- no candidate-publication execution;
- no runtime activation;
- no deployment/restart/recovery behavior.

Any source materialization must be re-audited against the exact C03e-CE head before mutation; this contract does not pre-authorize a concrete source diff.

## 17. Remaining prerequisite after C03e-CE

C03e-CE does not close the remaining C03e-BX network-execution prerequisite.

Still separately required before live candidate-publication PRWC execution:

- exact generic Phase 129 server TLS/listener/accepted-stream primitive selection;
- exact bridge-owned PRWC connection/frame-loop/failure-lifecycle execution selection;
- only then bounded source materialization and disposable validation;
- only after all semantic/runtime prerequisites, separately authorized product/runtime wiring.

The current production `prw-control-transport` remains outbound-client oriented. Disposable server-side TLS test code is precedent only and is not promoted into production authority by C03e-CE.

## 18. Explicit non-selections

C03e-CE does not select or materialize:

- Rust/Kotlin source changes;
- Cargo manifest or lockfile changes;
- PRWC listener/server/accepted-stream implementation;
- TLS server credential custody/provisioning;
- bridge frame read/write loop;
- request-ID source materialization;
- authenticated-session persistence;
- requester/rendezvous provider representation;
- candidate-publication execution;
- reachability mutation;
- capability dispatch;
- Agent/Desktop/Android runtime wiring;
- production networking;
- host/systemd/firewall/NAT/routes/DNS/TUN/TAP mutation;
- database/provider mutation;
- deployment/restart/recovery;
- rebase or merge.

## 19. Closure requirements

C03e-CE may close only if all of the following remain true at its exact head:

- exact predecessor is closed C03e-CD head `4774517b9c7625a83eb0a06791e4b4f60a9487af`;
- compare is one commit ahead / zero behind with exact CD merge base;
- exactly one changed path exists: this contract;
- no production source, manifest, lockfile, workflow, runtime, networking, provider/database, deployment, Agent/Desktop/Android application path changed;
- automatically triggered canonical workflows are terminal and non-failing;
- immutable Drive audit is raw-readback verified;
- rolling Drive evidence is appended only after a fresh predecessor hash guard;
- PR remains draft/open/unmerged after body status becomes `CLOSED`.

## 20. Selected gate

Upon successful closure evidence:

`C03E_CE_PRWA_VERIFIER_SESSION_ID_TIME_SOURCE_AUTHORITY_SELECTED`

The next safe checkpoint is a separately gated bounded source-materialization selection/readiness step for the C03e-CE verifier source, or—if source materialization is intentionally deferred—a docs-only selection for the remaining generic Phase 129 server/accepted-stream and bridge runtime execution prerequisite.
