# Phase 152 C03e-BX — Candidate Publication PRWC Runtime Ownership / Prerequisite Ordering Selection

Status: STAGED SELECTION

Gate target:
`C03E_BX_CANDIDATE_PUBLICATION_PRWC_RUNTIME_OWNERSHIP_PREREQUISITE_ORDERING_SELECTED`

## 1. Exact predecessor

Closed/revalidated C03e-BW is the authoritative predecessor:
- branch: `phase-152-c03e-bw-candidate-publication-prwc-runtime-prerequisite-readiness-staging`;
- head: `a4a02a6d754fc2b67dbb5d6a512341a76bc78119`;
- tree: `ed00b5aadb4f75f18ca8ece0e3f57dcce398f9b7`;
- gate: `C03E_BW_CANDIDATE_PUBLICATION_PRWC_RUNTIME_PREREQUISITE_READINESS_AUDITED`;
- PR `#192`: body `Status: CLOSED — CURRENT HEAD REVALIDATED`, draft/open/unmerged.

BW established that live candidate-publication PRWC runtime wiring is not yet ready because three authority groups remain separately unselected: PRWC request-ID custody/lifecycle, pre-mesh logical authentication plus requester/rendezvous authority, and network execution ownership.

BX selects only the ownership layering and dependency ordering among those prerequisites. It does not materialize any runtime source.

## 2. Exact repository audit basis

BX is grounded in the exact BW snapshot.

### 2.1 Generic Phase 129 transport

`crates/prw-control-transport/src/lib.rs` blob:
`34b0a898572adaa2f77251ca2e9c66ea29973e95`

It owns generic PRWC framing, non-zero envelope request-ID validation, bounded read/write codec primitives, and a bounded outbound TLS client. It explicitly keeps message semantics above transport and states TLS success is transport authentication only.

It does not own candidate-publication semantics, logical PRW session authentication, requester routing, request lifecycle/custody, a candidate-publication broker, or a server-side Phase 129 listener/acceptor.

### 2.2 Higher bridge/integration layer

`crates/prw-remote-bridge/Cargo.toml` blob:
`a0e80460c8c101f72dc8b95b77b7ee33aae1f179`

The bridge now depends downward on `prw-control-transport` and already owns the validated BV candidate-publication PRWP↔PRWC adapter.

`crates/prw-remote-bridge/src/candidate_publication_control_frame.rs` blob:
`20ff7d2bc5f32596a3c0696aa387e6735f8f2031`

The adapter accepts a caller-supplied non-zero outer request ID but deliberately owns no allocator, request table, authentication, routing or I/O.

### 2.3 Existing bridge runtime patterns are different protocols

`crates/prw-remote-bridge/src/remote_server_transport_runtime.rs` blob:
`14b774d11c1c123f001580be252eb036329d6d2e`

It is a bridge-owned wrapper above the lower PRWM/QUIC runtime and explicitly distinguishes lower transport authentication from logical-session authentication and capability authority.

`crates/prw-remote-bridge/src/remote_session_binding.rs` blob:
`fcaa4960c7ec150d317e8aea197b5e936f3529a4`

It binds a lower authenticated PRWM transport identity to an already-authenticated logical session lease, but owns no socket/task/dispatcher lifecycle.

These files demonstrate an existing layering pattern only. They are PRWM authorities and must not be reused directly as Phase 129 PRWC implementations.

### 2.4 Product/runtime consumers remain above the bridge

Current manifests remain:
- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`;
- `apps/desktop/Cargo.toml`: `28c8c628651b92c5e62ed0ee97fb059b6037918e`;
- `apps/android/native/Cargo.toml`: `63e32c75ed8d700e2430481d4bc6c0ae89cc7e34`.

These surfaces consume `prw-remote-bridge`; they do not directly depend on `prw-control-transport` for candidate-publication runtime ownership.

## 3. Selected runtime ownership layering

BX selects this layering for later Phase 129 candidate-publication runtime work:

```text
Agent / Desktop / Android product surfaces
        -> prw-remote-bridge semantic/runtime composition
        -> prw-control-transport generic PRWC/TCP/TLS primitives
```

This is an ownership selection, not an implementation.

### Lower generic transport responsibility

A later separately authorized transport checkpoint may extend `prw-control-transport` only with generic Phase 129 server/accepted-stream mechanics required by the selected runtime topology.

That lower layer must remain ignorant of:
- candidate-publication PRWP semantics;
- DeviceId/workspace/user authority;
- authenticated PRW session leases;
- requester/recipient rendezvous semantics;
- freshness authority;
- publication admission or reachability commit.

### Higher bridge runtime-composition responsibility

A later separately authorized bridge checkpoint may own the Phase 129 candidate-publication connection/request/session orchestration above generic transport.

The bridge runtime layer may later compose already-selected authorities, but must not weaken or replace them.

Agent/Desktop/Android are not selected as owners of the PRWC request lifecycle or protocol runtime.

## 4. Why runtime ownership must be selected before request-ID custody

BW requires a future request-ID selection to decide uniqueness scope, outstanding-request bound, completion, abandonment, connection-discard behavior and restart semantics.

Those rules require a concrete ownership/lifetime boundary for one Phase 129 runtime connection context.

Therefore BX selects this dependency order:

> runtime ownership/lifetime boundary first; exact PRWC request-ID custody/lifecycle selection second.

This does not allocate any request ID in BX.

## 5. Request-ID custody placement selected at layer level only

BX selects that future PRWC request-ID lifecycle authority belongs above the generic frame codec and below product UI/runtime callers, within the bridge-owned Phase 129 runtime context.

`prw-control-transport` continues to validate only the envelope-level non-zero field and must not become an application request-table authority.

A successor may select the exact bridge-owned request-ID lifecycle representation, bounds and reuse rules, but BX itself does not choose:
- generator algorithm;
- initial value;
- random versus monotonic generation;
- concrete table/container;
- timeout values;
- retry semantics;
- response/error schema.

## 6. Authentication/rendezvous dependency remains separate

Logical publisher identity must still come from a current authenticated PRW control-plane session above TLS. Requester/recipient rendezvous remains separately authoritative.

BX selects no pre-mesh auth wire, session store, routing schema or broker.

The future bridge runtime context must eventually bind those authorities explicitly; neither PRWC request ID nor TLS connection identity may substitute for them.

## 7. Network execution materialization remains last

BX selects the following safe dependency ordering for candidate-publication runtime materialization:

```text
1. runtime ownership/lifetime layering selection  [this BX checkpoint]
2. exact bridge-owned PRWC request-ID custody/lifecycle selection
3. exact pre-mesh logical authentication + requester/rendezvous authority selection
4. exact generic transport server/accepted-stream and bridge runtime execution source selection
5. only then bounded source materialization and disposable validation
```

Steps 2 and 3 are semantic prerequisites. Their internal order after BX may be adjusted only by a later docs-only dependency audit, but neither may be skipped before live execution materialization.

No production networking is authorized by this ordering.

## 8. Existing PRWM runtime is not a shortcut

The existing QUIC `RemoteServerTransportRuntime`, `BoundRemoteSession`, PRWM request IDs and PRWM session-auth wire must not be transplanted into Phase 129 by structural similarity.

They may inform layering discipline only.

Any shared abstraction requires a separately demonstrated repository need and an explicit contract before refactoring.

## 9. Preserved identity/security boundaries

BX preserves:
- current authenticated PRW session / DeviceId as logical identity;
- TransportIdentity as lower rotatable transport certificate identity only;
- SessionId as authentication correlation only;
- PRWC request ID as outer request correlation only;
- CandidateId as plan-scoped candidate correlation only;
- freshness token as verifier-owned publication currentness/replay authority only.

No selected runtime owner may derive authority from a socket endpoint, TLS success, request ID, candidate endpoint or freshness-token possession alone.

## 10. Explicit non-selections

BX does not select or materialize:
- request-ID generator/source code;
- outstanding request table/source code;
- response/ack/error protocol;
- timeout/retry/idempotency implementation;
- Phase 129 server listener/acceptor source;
- TLS server credential source/custody;
- logical authentication wire protocol;
- authenticated-session provider/store;
- requester/recipient routing schema/provider;
- broker/dispatcher source;
- Agent/Desktop/Android runtime wiring;
- publication admission execution;
- reachability mutation;
- provider/database mutation;
- STUN/ICE/TURN/relay/QUIC activation;
- host/systemd/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## 11. Safe successor rule

After BX closure, the next safe checkpoint is a bounded docs-only selection for exact bridge-owned PRWC request-ID custody/lifecycle, grounded in the runtime/lifetime ownership selected here.

That successor must remain in-memory/lifecycle-only and must not select authentication/routing, frame I/O, listener/acceptor mechanics or networking activation.

If exact request-ID semantics cannot be fixed without first resolving a concrete contradiction in connection ownership, work must stop and reopen dependency ordering rather than silently broadening scope.

## 12. Exact BX source scope

The final BW→BX diff is authorized to contain exactly one path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BX_CANDIDATE_PUBLICATION_PRWC_RUNTIME_OWNERSHIP_PREREQUISITE_ORDERING_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, Agent/Desktop/Android implementation, transport implementation, provider/database file, networking configuration or deployment path blocks BX closure.

## 13. Validation and closure

BX may close only after:
- exact current-head BW predecessor lineage remains unchanged;
- exact BW→BX compare contains one docs-only path;
- audit-basis source/manifests remain byte-stable;
- every triggered workflow reaches terminal non-failing verdict;
- immutable Drive audit raw readback passes;
- rolling Drive predecessor guard, append-only prefix proof and post-write raw verification pass;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No source/runtime/networking/deployment mutation is authorized by BX closure.
