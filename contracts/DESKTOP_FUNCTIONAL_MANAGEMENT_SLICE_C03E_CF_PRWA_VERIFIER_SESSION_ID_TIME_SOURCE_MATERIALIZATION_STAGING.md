# Phase 152 C03e-CF — PRWA Verifier SessionId + Time Source Materialization

Status: `STAGED SOURCE — PENDING EXACT-HEAD VALIDATION`

Target gate:
`C03E_CF_PRWA_VERIFIER_SESSION_ID_TIME_SOURCE_MATERIALIZED`

## 1. Exact predecessor

C03e-CF is a direct successor of the closed C03e-CE authority-selection checkpoint:

- branch: `phase-152-c03e-ce-prwa-verifier-session-id-time-source-authority-selection-staging`;
- head: `69d39def00e9ec3165102c60b7d8c14886b73f38`;
- tree: `36f357fe17a7002f0f0882194d985188534d65c6`;
- gate: `C03E_CE_PRWA_VERIFIER_SESSION_ID_TIME_SOURCE_AUTHORITY_SELECTED`;
- PR #202 remains draft/open/unmerged with body status `CLOSED`.

C03e-CE selected the verifier-side PRWA `SessionId` and time-source authority only. C03e-CF materializes exactly that selected pure/server-local source and does not broaden into network execution.

## 2. Selected CE authority preserved exactly

The source materialized here preserves all C03e-CE selections:

- one fresh SessionId uses exactly 32 bytes / 256 bits of cryptographically secure randomness;
- randomness uses the existing production `aws_lc_rs::rand::SystemRandom` provider already depended on by `prw-session`;
- the 32 random bytes are encoded as exactly 64 lowercase hexadecimal ASCII bytes;
- the encoded opaque value is reconstructed through existing `prw_core::SessionId::new(...)`;
- SessionId is not derived from PRWC request ID, DeviceId, verifier time, TransportIdentity, WorkspaceId, UserId, challenge nonce, candidate/freshness state, requester/rendezvous state, endpoint, socket address, process/host identity, or product input;
- exactly one randomness acquisition/generation attempt occurs; this source contains no collision retry or replacement-ID loop;
- SessionId remains non-persistent across restart; no database, durable counter, cross-host allocator, lease, or distributed lock is introduced;
- verifier time comes from server `SystemTime::now()` using checked `duration_since(UNIX_EPOCH)` and whole `u64` seconds;
- challenge lifetime is exactly the existing Phase 128 maximum, 300 seconds;
- expiry is computed with checked `issued + 300` arithmetic;
- pre-epoch verifier time or expiry overflow fails closed;
- proof submission must later obtain a fresh observation from the same verifier wall-clock helper;
- no client-supplied time/lifetime, skew grace, timestamp rewriting, monotonic-clock substitution, retry extension, or fallback is introduced.

## 3. Exact source boundary

C03e-CF is limited to exactly three repository paths:

1. `crates/prw-session/src/prwa_verifier_source.rs` — new pure/server-local verifier source plus focused unit tests;
2. `crates/prw-session/src/lib.rs` — one public module declaration only;
3. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CF_PRWA_VERIFIER_SESSION_ID_TIME_SOURCE_MATERIALIZATION_STAGING.md` — this contract.

Any fourth changed path requires a fresh scope audit before mutation.

No `Cargo.toml`, root `Cargo.lock`, Android native `Cargo.lock`, workflow, Agent/Desktop/Android application source, remote-bridge runtime, control-transport source, registry source, provider/database source, networking configuration, packaging, systemd, deployment, restart/recovery, or merge path is authorized.

## 4. Dependency and lockfile proof basis

At exact C03e-CE:

- `crates/prw-session/Cargo.toml` already has production `aws-lc-rs = 1.18.0`, `prw-control-plane`, `prw-core`, and `prw-device-identity` dependencies needed by this source;
- `crates/prw-control-plane/src/session_auth.rs` already exposes `MAX_SESSION_AUTH_CHALLENGE_LIFETIME_SECONDS: u64 = 300`;
- root `Cargo.lock` blob is `eeacde7ee776d35088f746a6d09f823f3391b82b`;
- `apps/android/native/Cargo.lock` blob is `cce9ca06190a196661ab38d54a747893e26af95f`.

C03e-CF requires no dependency or lockfile mutation. Both lock blobs must remain byte-stable at closure.

## 5. Materialized API

The new `prw_session::prwa_verifier_source` module exposes:

- `PrwaVerifierSessionContext` containing exactly a typed `SessionId`, verifier issue Unix seconds, and verifier expiry Unix seconds;
- `PrwaVerifierSourceError` with bounded non-secret classifications for SessionId randomness, SessionId construction, verifier-time representation, and expiry overflow;
- `new_prwa_verifier_session_context()` for one fresh server-local SessionId plus exact 300-second challenge window;
- `current_prwa_verifier_unix_seconds()` for a fresh proof-submission verifier-time observation from the same wall-clock authority;
- constants locking the 32-byte random input, 64-byte lowercase-hex representation, and 300-second challenge lifetime.

The source contains no mutable global state and no internal transaction table.

## 6. Authority separation

This source does not call or replace `SessionAuthenticationService::begin_session(...)`.

It does not call or replace `SessionAuthenticationService::submit_proof(...)`.

Future bounded composition must still:

1. resolve current enrolled-device authority under the separately selected registry/transport rules;
2. obtain one C03e-CF verifier context;
3. pass its typed SessionId and exact issue/expiry values into existing `begin_session(...)` exactly once;
4. rely on the existing service to reject a duplicate live SessionId;
5. later obtain a fresh `current_prwa_verifier_unix_seconds()` observation;
6. pass that verifier time into existing `submit_proof(...)` exactly once.

Therefore C03e-CF does not become challenge-nonce authority, pending/authenticated state authority, replay authority, identity-verification authority, registry authority, request-ID authority, transport authority, routing authority, or capability authority.

## 7. Collision and failure semantics

The materialized source performs one SessionId randomness acquisition only.

If a future call to `SessionAuthenticationService::begin_session(...)` rejects the generated SessionId as already pending/authenticated, that authentication attempt fails closed under C03e-CE semantics. C03e-CF does not generate a replacement ID, retry on the same transaction, allocate a new PRWC request ID, or silently reauthenticate the connection.

Before pending state exists:

- RNG failure returns `SessionIdRandomness`;
- typed SessionId construction failure returns `SessionIdConstruction`;
- pre-epoch/non-representable verifier time returns `VerifierTime`;
- checked expiry overflow returns `ExpiryOverflow`.

No pending-session cleanup is required because this source never creates pending state.

## 8. Focused validation

Unit tests in the new module prove:

- fixed bytes `[0xab; 32]` encode exactly 64 lowercase hexadecimal characters and reconstruct a typed SessionId;
- a fixed issue time yields an exact 300-second expiry window;
- `UNIX_EPOCH` maps to zero Unix seconds;
- a representable pre-epoch time fails closed;
- `u64` expiry overflow fails closed;
- the public source returns a 64-character lowercase-hex SessionId and exact 300-second window.

No probabilistic distinct-ID assertion is used.

## 9. Explicit non-materializations

C03e-CF does not materialize or activate:

- PRWC server TLS config;
- listener/bind/accept source;
- accepted-stream source;
- socket/frame read-write loop;
- bridge-owned PRWC connection lifecycle;
- request-ID allocator/outstanding table source;
- requester/rendezvous provider;
- candidate-publication admission/execution;
- reachability mutation;
- authenticated-session durable persistence;
- Agent/Desktop/Android runtime wiring;
- capability authorization/dispatch;
- STUN/ICE/TURN/relay/QUIC activation;
- host/systemd/firewall/NAT/routes/DNS/TUN/TAP changes;
- provider/database mutation;
- credential provisioning;
- deployment/restart/recovery;
- rebase or merge.

## 10. Closure requirements

C03e-CF may close only if all are true at the exact final head:

- parent lineage begins from exact closed CE head `69d39def00e9ec3165102c60b7d8c14886b73f38`;
- merge base with CE is exact CE head;
- branch is ahead only and not behind CE;
- final net diff contains exactly the three paths listed in section 3 and no fourth path;
- `crates/prw-session/src/lib.rs` net change is only the module declaration;
- root `Cargo.lock` remains blob `eeacde7ee776d35088f746a6d09f823f3391b82b`;
- Android native `Cargo.lock` remains blob `cce9ca06190a196661ab38d54a747893e26af95f`;
- canonical automatically triggered workflows are terminal and non-failing, with skipped workflows reported only as skipped;
- any formatting/lint/test correction stays inside the already authorized three paths; a fourth path requires fresh audit;
- immutable Drive audit is published and raw-readback verified;
- rolling `C02E_BRANCH_STATUS.md` is appended only after a fresh exact predecessor-size/SHA concurrency guard and post-write prefix/suffix/full-image verification;
- candidate PR remains draft/open/unmerged after closure body status becomes `CLOSED`.

## 11. Completion gate and next boundary

Upon successful exact-head validation and evidence closure:

`C03E_CF_PRWA_VERIFIER_SESSION_ID_TIME_SOURCE_MATERIALIZED`

After C03e-CF, the verifier-side semantic source prerequisite is closed. The next safe checkpoint is a separately gated docs-only selection/readiness checkpoint for the still-missing generic Phase 129 PRWC server TLS/listener/accepted-stream primitive plus bridge-owned PRWC connection/frame-loop/failure-lifecycle execution authority.

No live candidate-publication PRWC runtime wiring is authorized by C03e-CF.
