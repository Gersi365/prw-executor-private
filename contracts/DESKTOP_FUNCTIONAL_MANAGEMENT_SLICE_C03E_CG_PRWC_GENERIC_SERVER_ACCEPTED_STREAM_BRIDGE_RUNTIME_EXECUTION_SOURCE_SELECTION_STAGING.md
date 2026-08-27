# Phase 152 C03e-CG — PRWC Generic Server / Accepted-Stream + Bridge Runtime Execution Source Selection

Status: `STAGED SELECTION — PENDING EXACT-HEAD VALIDATION`

Target gate:
`C03E_CG_PRWC_GENERIC_SERVER_ACCEPTED_STREAM_BRIDGE_RUNTIME_EXECUTION_SOURCE_SELECTED`

## 1. Exact predecessor

C03e-CG is a direct docs-only successor of closed C03e-CF:

- branch: `phase-152-c03e-cf-prwa-verifier-session-id-time-source-materialization-staging`;
- head: `523c5f3a7fc98ef1c6992e3083a299421350efbc`;
- tree: `18640b58191065b564b32e693108c6ceb8e032d2`;
- gate: `C03E_CF_PRWA_VERIFIER_SESSION_ID_TIME_SOURCE_MATERIALIZED`;
- PR #203 remains draft/open/unmerged with body `Status: CLOSED`.

C03e-CF closes the verifier-side `SessionId` and verifier-time source prerequisite. C03e-CG selects only the still-missing generic Phase 129 PRWC server/accepted-stream boundary and bridge-owned connection execution/lifecycle boundary. It materializes no runtime source and activates no listener.

## 2. Preserved layering and prerequisite authority

C03e-BX remains authoritative for layering:

```text
Agent / Desktop / Android product surfaces
        -> prw-remote-bridge semantic/runtime composition
        -> prw-control-transport generic PRWC/TCP/TLS primitives
```

C03e-CG therefore keeps protocol semantics, logical authentication, requester/rendezvous authority, publication admission and reachability commit out of `prw-control-transport`.

C03e-BY remains authoritative for bridge-owned per-connection PRWC request-ID custody/lifecycle. C03e-BZ remains authoritative for one connection-local authenticated logical-session binding plus independent server-side requester/rendezvous authority. C03e-CA/CD/CF remain authoritative for the PRWA transaction, codec, and verifier source.

No PRWM request IDs, PRWM `PRWS` bytes, `MeshControlStream`, QUIC connection state, or PRWM authenticated-peer type is promoted into Phase 129 PRWC authority.

## 3. Exact source audit basis at closed CF

The selection is grounded in these byte-stable exact-CF sources:

- `crates/prw-control-transport/src/lib.rs` — blob `34b0a898572adaa2f77251ca2e9c66ea29973e95`;
- `crates/prw-control-transport/tests/tls_loopback.rs` — blob `eedebde337e0f90c920bd92740a57aa8f068a466`;
- `crates/prw-remote-bridge/Cargo.toml` — blob `5fd48263be415aac28dee1c71a4031a4a02ad36c`;
- `crates/prw-remote-bridge/src/root.rs` — blob `8fdc1f30d6be12e55e0cfa0c7624810e60466b99`;
- `crates/prw-remote-bridge/src/control_session_auth_wire.rs` — blob `77c6f401ef73c0b2a97645ae8bc83524c769a905`;
- `crates/prw-remote-bridge/src/candidate_publication_control_frame.rs` — blob `20ff7d2bc5f32596a3c0696aa387e6735f8f2031`;
- `crates/prw-remote-bridge/src/remote_server_transport_runtime.rs` — blob `14b774d11c1c123f001580be252eb036329d6d2e` (PRWM precedent only);
- `crates/prw-session/src/lib.rs` — blob `0b0b6624df93ebcf3efae632d94dfc337ee67761`;
- `crates/prw-session/src/prwa_verifier_source.rs` — blob `e34c3d452b9fd5c9787abbf1f36106e3b97e3b0b`;
- `crates/prw-registry/src/lib.rs` — blob `cd7eb52eea19354620ff8ad26c9b8aad3f9c5eb6`.

Current production Phase 129 transport is outbound-only: it owns bounded `ControlFrame` read/write, TLS 1.3-only client configuration, one bounded TCP connect attempt, exact `CONTROL_ALPN = b"prw-control/1"`, and an established client-side `ControlTlsStream`. It has no production `TcpListener`, `ServerConfig`, `ServerConnection`, server bind/accept API, or accepted server-stream type.

The existing loopback test proves only that the already-pinned rustls provider can construct disposable TLS 1.3 server primitives with exact ALPN and no client-certificate authentication. Its fixture credentials and test-thread ownership are not production authority.

## 4. Selected generic server primitive boundary

A later separately gated source-materialization checkpoint may extend `prw-control-transport` with generic server-side Phase 129 primitives only.

Selected responsibilities of the generic transport layer are:

1. construct a rustls server configuration from caller-supplied server certificate chain/private-key material;
2. enforce TLS 1.3 only using the already-pinned aws-lc rustls provider;
3. configure exactly `CONTROL_ALPN` and disable any protocol fallback;
4. use server-authenticated TLS with no client-certificate authentication in this initial PRWC profile;
5. bind one explicit caller-supplied `SocketAddr` through a generic TCP listener primitive;
6. accept one TCP socket at a time when requested by its owner;
7. apply caller-supplied non-zero read/write socket timeouts bounded by existing `MAX_CONTROL_TIMEOUT` and enable TCP no-delay;
8. complete the rustls `ServerConnection` handshake before yielding the accepted stream;
9. verify the negotiated protocol is TLS 1.3 and the negotiated ALPN is exactly `CONTROL_ALPN` before return;
10. expose only a bounded accepted server-side PRWC stream abstraction with `read_frame(...)` / `write_frame(...)` over the existing exact `ControlFrame` codec.

The accepted stream proves only successful generic Phase 129 server-side TCP/TLS mechanics. It does not prove logical PRW authentication, enrollment, requester authority, capability authorization, candidate freshness, publication authority or reachability currentness.

## 5. Credential boundary

C03e-CG selects no credential source, provisioning mechanism, filesystem path, secret store, database row, certificate rotation workflow, deployment mechanism or product setting.

Generic transport constructors may receive owned/borrowed certificate-chain and private-key bytes from their caller and validate/build a server configuration. They must not discover credentials, persist them, generate a CA, mint production certificates, infer paths, or become credential-custody authority.

Disposable test fixture certificate/key bytes remain test-only.

## 6. Listener ownership split

The generic transport layer owns the mechanics and representation of a bound Phase 129 listener and one accepted TLS stream.

The bridge layer owns each live listener instance's semantic lifetime and every decision to invoke accept again. In particular:

- `prw-control-transport` does not spawn an accept loop, worker, broker, dispatcher or authentication task;
- `prw-remote-bridge` later owns the accept-loop/runtime context that holds the generic listener primitive;
- product surfaces do not own raw PRWC sockets or protocol-state machines;
- listener bind address remains explicit input to the bridge/runtime constructor and is not discovered from DNS, rendezvous data, candidate endpoints or product payloads;
- bind/listen success is not readiness, logical authentication, capability authorization or publication authority.

C03e-CG selects ownership only. It does not select thread-vs-async scheduling, worker count, backoff, retry, concurrency framework or process supervision.

## 7. Selected bridge per-connection execution context

A later bridge source-materialization checkpoint may create one connection-local PRWC execution context in `prw-remote-bridge` for each accepted generic PRWC server stream.

That context is the owner of:

- the accepted PRWC stream handle;
- connection-local BY request-ID lifecycle state for locally originated requests;
- exactly one pre-mesh PRWA authentication transaction state while unauthenticated;
- at most one completed BZ `AuthenticatedDeviceSession` binding for that connection;
- explicit pending-session cleanup metadata sufficient to call `abort_pending_session(...)` exactly once when required;
- injected references/ports to existing session-authentication and current-registry authorities;
- an injected requester/rendezvous authority port selected by BZ, without selecting its storage/provider implementation.

The context does not persist any of these connection-local states across connection discard or process restart.

## 8. Initial connection state and frame admission

Every newly accepted PRWC stream begins in logical state `Unauthenticated` regardless of TLS success.

Before terminal PRWA `Authenticated` delivery has succeeded:

- only the CA-selected PRWA authentication transaction is admissible for semantic processing;
- candidate-publication `Command` frames are not decoded into publication admission/execution;
- other command semantics receive no authorization from transport success;
- a protocol/order violation fails closed and the connection is discarded;
- if a pending PRWA session exists at that point, its pending session is aborted exactly once before connection teardown.

C03e-CG does not select a new general-purpose pre-auth command error payload. CA's generic PRWA `Rejected` remains the only selected authentication terminal failure when an authentication transaction is active.

## 9. Exact PRWA server execution ordering

A later runtime implementation must preserve the already-selected CA/CF ordering:

1. read one bounded `ControlFrame` from the accepted stream;
2. strictly decode PRWA Begin and treat its `DeviceId` only as an untrusted selector;
3. resolve the current server-side registered enrolled-device binding from existing registry authority;
4. obtain one fresh C03e-CF verifier context (`SessionId`, issued time, expiry time);
5. call `SessionAuthenticationService::begin_session(...)` exactly once;
6. record enough connection-local pending metadata to guarantee explicit cleanup;
7. encode/write the CA/CD Challenge on the same peer-originated request ID;
8. read exactly one correlated Proof and require exact request-ID/session correlation;
9. reconstruct the existing typed `SessionAuthProof`;
10. obtain a fresh C03e-CF verifier-time observation;
11. call `SessionAuthenticationService::submit_proof(...)` exactly once;
12. call `WorkspaceDeviceRegistry::validate_authenticated_session(...)` on the returned session;
13. encode/write terminal PRWA Authenticated on the same request ID;
14. only after that write succeeds, install the returned `AuthenticatedDeviceSession` as the usable BZ connection-local logical-session binding.

No `Command` becomes admissible between steps 11 and 14.

The service remains nonce/replay/proof authority. The registry remains currentness authority. The bridge merely composes their already-reviewed APIs at the selected connection lifecycle boundary.

## 10. Authentication failure and cleanup lifecycle

Before pending service state exists, decode, lookup, verifier-source, or `begin_session(...)` failure creates no pending cleanup obligation; the bridge may attempt the CA-selected generic Rejected terminal response when a valid transaction correlation exists, then discards the connection.

Once `begin_session(...)` succeeds and pending state exists:

- every terminal failure before successful `submit_proof(...)` commit calls `abort_pending_session(...)` exactly once;
- after that cleanup, the bridge attempts one generic PRWA Rejected write on the transaction request ID when it can form that response;
- Rejected write success or failure does not permit retry or continued use; the connection is discarded;
- no replacement SessionId, replacement challenge, replacement request ID, second Begin or same-connection reauthentication is selected.

After `submit_proof(...)` succeeds, pending state no longer exists. If current-registry revalidation fails, Authenticated encoding/writing fails, or terminal Authenticated delivery is unavailable/fails:

- the connection is discarded;
- the returned connection-local authenticated-session candidate is not installed/used;
- no call to `abort_pending_session(...)` is made for that completed SessionId;
- C03e-CG does not invent a new API that deletes the already-completed internal `SessionAuthenticationService` authenticated record.

This preserves the actual existing service state machine instead of pretending completed authentication can be rolled back through the pending cleanup API.

## 11. BY request-ID source placement

The BY-selected pure request-ID lifecycle source remains unmaterialized at C03e-CG, but its placement is now selected exactly: it belongs as connection-local in-memory state owned by the future bridge PRWC connection execution context.

It stays above the generic frame codec and below product callers.

For peer-originated PRWA and candidate-publication requests, the bridge preserves the received non-zero request ID for correlated responses and does not allocate a replacement. Locally originated future requests use the BY allocator only after its separate source materialization.

Request-ID state is not authentication, authorization, routing, freshness or candidate identity.

## 12. Post-authentication Command boundary

Only after successful terminal Authenticated delivery may the connection-local authenticated binding be used to consider `ControlMessageKind::Command` frames.

For candidate publication, a later separately gated execution checkpoint must still:

- decode PRWP through the existing pure `candidate_publication_control_frame` adapter;
- derive publisher logical identity from the connection-local authenticated session, not PRWP fields;
- revalidate publisher session/current registry authority at semantic use time;
- separately validate publisher-presented `TransportIdentity` under existing authority;
- obtain an independent current requester/rendezvous selection through the BZ-selected injected authority;
- validate requester currentness, expected publisher, workspace and target-plan relationships;
- preserve freshness/current-plan/admission/compare-and-commit authorities already selected elsewhere.

C03e-CG does not materialize or authorize those candidate-publication execution calls. It only selects where the injected requester/rendezvous authority enters the future bridge runtime.

## 13. Requester/rendezvous injection boundary

The BZ requester/rendezvous authority is injected into the bridge semantic/runtime layer as an interface/port supplied by a higher server-side composition root.

It is not supplied by:

- `prw-control-transport`;
- the accepted socket or peer address;
- TLS/ALPN state;
- PRWC request ID;
- PRWA fields;
- PRWP fields;
- candidate endpoints;
- freshness tokens.

C03e-CG selects no concrete provider, key, table, schema, database, cache, broker, persistence semantics or product API for that port.

## 14. Connection discard and shutdown

On accepted-stream failure, malformed frame, protocol-order violation, authentication terminal failure, terminal write failure, explicit runtime shutdown or listener/runtime teardown:

- no further semantic frames are processed on that connection;
- any still-pending PRWA session is aborted exactly once;
- BY outstanding locally originated request IDs are abandoned as connection-discard state, not completed successfully;
- the connection-local authenticated binding is dropped;
- requester/rendezvous handles/references are released without mutating provider authority;
- the accepted stream/socket is dropped/closed;
- no automatic reconnect, retry, reauthentication, request replay or candidate republish is selected.

Listener-level shutdown prevents future accepts under that bridge runtime instance. C03e-CG does not select process supervision, restart policy or deployment orchestration.

## 15. Server transport identity and logical identity separation

The initial Phase 129 server profile remains server-authenticated TLS only. `with_no_client_auth()` is therefore selected for the generic initial server primitive; client logical identity is proven only by PRWA above the transport.

Consequently:

- TLS success does not identify a PRW `DeviceId`;
- peer socket address is not identity;
- absence of a client certificate is not an authentication bypass because pre-mesh PRWA remains mandatory before Command admission;
- `TransportIdentity` remains an independently rotatable lower transport identity used only where separately selected semantic checks require it; it is not inferred from this server-authenticated PRWC socket.

No mutual-TLS client-certificate identity profile is selected by C03e-CG.

## 16. Selected future source-materialization boundary

After C03e-CG closure, the smallest safe source-materialization work is still separately gated and must remain non-activated/disposable.

A future source checkpoint may touch only explicitly re-audited paths needed for:

- generic `prw-control-transport` server config/listener/accepted-stream primitives and focused loopback tests;
- a bridge-owned pure/bounded request-ID lifecycle implementation matching BY;
- a bridge-owned PRWC connection execution state machine/composition source plus focused tests;
- minimal crate exports/manifests/lock synchronization only if exact dependency analysis proves they are required.

The exact path list must be selected by that future source audit before mutation. C03e-CG itself does not pre-authorize any source path.

## 17. Explicit non-materializations

C03e-CG does not create or activate:

- production server/listener/accepted-stream Rust source;
- request-ID allocator/outstanding-table Rust source;
- socket/frame-loop Rust source;
- live authentication runtime;
- candidate-publication execution;
- requester/rendezvous provider implementation;
- credential provisioning/storage/rotation;
- Agent/Desktop/Android runtime wiring;
- capability authorization/dispatch;
- STUN/ICE/TURN/relay/QUIC activation;
- host/systemd/firewall/NAT/route/DNS/TUN/TAP mutation;
- provider/database mutation;
- deployment/restart/recovery;
- rebase or merge.

No runtime/network/deployment authority is granted by this docs-only selection.

## 18. C03e-CG exact repository scope

C03e-CG itself is limited to exactly one docs-only repository path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CG_PRWC_GENERIC_SERVER_ACCEPTED_STREAM_BRIDGE_RUNTIME_EXECUTION_SOURCE_SELECTION_STAGING.md`

Any second changed path requires a fresh scope audit before mutation.

Root `Cargo.lock`, Android native `Cargo.lock`, all Cargo manifests, Rust/Kotlin source and workflows must remain byte-stable.

## 19. Closure requirements

C03e-CG may close only if all are true at the exact final head:

- parent is exact closed CF head `523c5f3a7fc98ef1c6992e3083a299421350efbc`;
- merge base with CF is exact CF head;
- branch is ahead only and not behind CF;
- final net diff contains exactly the single docs-only contract path in section 18;
- root `Cargo.lock` remains blob `eeacde7ee776d35088f746a6d09f823f3391b82b`;
- Android native `Cargo.lock` remains blob `cce9ca06190a196661ab38d54a747893e26af95f`;
- all automatically triggered canonical workflows are terminal and non-failing, with skipped workflows reported only as skipped;
- immutable Drive audit is published and raw-readback verified;
- rolling `C02E_BRANCH_STATUS.md` is appended only after a fresh exact predecessor size/SHA concurrency guard and post-write prefix/suffix/full-image verification;
- candidate PR remains draft/open/unmerged after its body becomes `Status: CLOSED`.

## 20. Completion gate and next boundary

Upon exact validation and evidence closure:

`C03E_CG_PRWC_GENERIC_SERVER_ACCEPTED_STREAM_BRIDGE_RUNTIME_EXECUTION_SOURCE_SELECTED`

After C03e-CG, BX prerequisite item 4 is selected but still not materialized. The next safe checkpoint is an audit-first, separately gated source-materialization plan for the exact generic server/accepted-stream and bridge connection execution surfaces described here, including BY request-ID source placement and disposable validation.

No live production listener, authentication cutover, candidate-publication execution, product wiring or deployment is authorized until later explicit gates close.
