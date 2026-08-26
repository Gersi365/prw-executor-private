# Phase 152 C03e-BW — Candidate Publication PRWC Runtime Prerequisite Readiness

Status: STAGED READINESS AUDIT

Gate target:
`C03E_BW_CANDIDATE_PUBLICATION_PRWC_RUNTIME_PREREQUISITE_READINESS_AUDITED`

## 1. Exact predecessor

Closed C03e-BV is the authoritative predecessor:
- branch: `phase-152-c03e-bv-candidate-publication-prwp-prwc-pure-adapter-source-materialization-staging`;
- head: `f57067488a862de2f9ecf62fbc40421c8f364f16`;
- tree: `ce3debb60dc90cbb9a2f1e51d9fc8cd2ce8196dd`;
- gate: `C03E_BV_CANDIDATE_PUBLICATION_PRWP_PRWC_PURE_ADAPTER_SOURCE_MATERIALIZED`;
- PR `#191`: body `Status: CLOSED`, draft/open/unmerged.

BV materialized and canonically validated only the pure in-memory PRWP↔PRWC `Command` composition adapter plus the exact deterministic lockfile consequences. It did not select or materialize production request-ID custody, authenticated pre-mesh logical-session composition, requester/rendezvous routing, broker/dispatcher/listener execution, frame I/O, publication admission, reachability mutation, production networking or deployment.

## 2. Exact bounded purpose

BW is a docs-only readiness checkpoint.

BW audits whether the exact BV repository already contains enough authority to proceed directly from the pure adapter to a live candidate-publication runtime path.

BW may:
- identify existing authorities that can be reused;
- identify authority gaps that still block runtime wiring;
- preserve the already-selected PRWP/PRWC layering;
- define stop conditions for later selection checkpoints.

BW does not choose or implement any missing runtime mechanism.

## 3. Repository audit basis

BW is grounded in the exact BV snapshot and the following byte-stable authorities.

### 3.1 Phase 129 PRWC transport

`crates/prw-control-transport/src/lib.rs` remains the generic Phase 129 transport authority.

It currently provides:
- `ControlMessageKind`;
- `ControlFrame`;
- non-zero `u64 request_id` validation;
- bounded PRWC frame encoding/decoding;
- outbound TCP/TLS client transport;
- ALPN `prw-control/1`;
- server transport authentication only.

The crate explicitly states that message semantics remain above the transport layer and that TLS success does not enroll a device, authenticate a PRW logical session or grant capabilities.

It does not contain:
- a production request-ID allocator;
- an outstanding-request lifecycle table;
- response matching/cancellation ownership;
- authenticated logical-session storage;
- requester/recipient routing;
- a server listener/acceptor/broker for candidate publication.

### 3.2 BV pure candidate-publication adapter

`crates/prw-remote-bridge/src/candidate_publication_control_frame.rs` is now materialized and validated.

It may:
- encode an already-typed candidate-publication submission into exact PRWP bytes;
- wrap those bytes in existing `ControlMessageKind::Command` using a caller-supplied non-zero outer request ID;
- decode an already-decoded `Command` frame;
- preserve the outer request ID only as correlation metadata;
- decode exact PRWP bytes back into typed submitted values.

It does not allocate request IDs, authenticate a logical session, choose a requester/recipient, perform I/O, dispatch a command, commit publication state or activate networking.

### 3.3 Existing local IPC request tracking is a different authority

`crates/prw-agent/src/local_commands/request_tracker.rs` owns `LocalRequestTracker` for the local IPC connection boundary.

That authority is explicitly:
- per local connection;
- bounded to 64 outstanding local requests;
- keyed by `LocalIpcRequestId`;
- responsible for duplicate/unknown local request IDs and abandonment on local connection discard.

This does not establish PRWC production request-ID custody. BW rejects reuse-by-analogy between local IPC request tracking and Phase 129 remote control correlation without a separate selection proving ownership, lifecycle and restart semantics for PRWC.

### 3.4 Existing desktop local request IDs are not PRWC custody

`apps/desktop/src/ipc.rs` uses fixed local startup-probe IDs and validates local response correlation over the Phase 151 Unix-domain local Agent boundary.

That implementation is not a production PRWC allocator and has a different trust, transport, framing and lifecycle boundary.

### 3.5 Existing logical-session wire adapter is on PRWM

`crates/prw-remote-bridge/src/session_auth_wire.rs` is an existing logical-session challenge/proof wire adapter over PRWM `SessionAuthentication`.

It explicitly does not authenticate a transport peer, create a remote-session lease, evaluate policy or grant capabilities merely by encoding/decoding wire messages.

Earlier C03e-BO explicitly forbids silently reusing PRWM/mesh session-authentication or freshness delivery as the Phase 129 pre-mesh control-plane authentication/bootstrap path.

Therefore its existence does not prove that pre-mesh PRWC logical authentication is ready.

## 4. Preserved correlation semantics

BW preserves BO/BR/BS/BV correlation ownership exactly:

> Candidate-publication application semantics own no independent request identifier. PRWC `request_id` is outer envelope correlation only.

The PRWC request ID remains:
- non-zero;
- non-authenticating;
- non-authorizing;
- non-routing;
- non-freshness evidence;
- non-candidate identity;
- non-session identity.

BW does not add an inner request ID to PRWP and does not derive authority from the outer ID.

## 5. Runtime prerequisite A — PRWC request-ID custody remains missing

Before a live runtime path may call the BV adapter without an arbitrary caller-provided correlation value, a separate checkpoint must select exact production PRWC request-ID custody.

That future selection must decide at least:
- owner crate/layer;
- allocation API boundary;
- non-zero guarantee;
- uniqueness scope;
- reuse policy after completion;
- collision behavior;
- outstanding-request bound;
- response/error completion semantics;
- abandonment/cancellation semantics;
- connection-discard behavior;
- persistence or deliberate non-persistence;
- process-restart semantics;
- whether IDs are monotonic, random or otherwise generated;
- exact prohibition on deriving IDs from `DeviceId`, `SessionId`, `TransportIdentity`, `CandidateId`, freshness tokens, endpoints or wall-clock timestamps unless separately justified.

BW does not select any answer to those questions.

## 6. Runtime prerequisite B — pre-mesh logical authentication and requester/rendezvous authority remain missing

BO established that logical publisher identity must come from a current authenticated PRW control-plane session context above Phase 129 TLS and must not be reconstructed from PRWP fields.

BO also established that requester/recipient routing must come from separately authoritative current rendezvous/routing context rather than from publisher-submitted fields or PRWC correlation.

The exact BV repository does not yet prove a Phase 129 pre-mesh runtime path that provides both authorities to candidate publication.

A future selection must therefore decide, without weakening existing semantics:
- how a Phase 129 control connection becomes associated with a current authenticated PRW logical session;
- which existing session-authentication authorities are reused and which wire adaptation is required before mesh establishment;
- authenticated-session storage/provider ownership;
- session-currentness lookup and invalidation;
- requester/recipient rendezvous representation;
- expected-device scheduling provenance;
- workspace/target validation ownership;
- broker/routing table/provider ownership;
- explicit separation between transport/TLS identity and logical `DeviceId`/session identity.

BW does not select a new authentication protocol, session store, routing schema or broker.

## 7. Runtime prerequisite C — network execution ownership remains missing

BV and its predecessors deliberately stopped before frame I/O and runtime activation.

A future checkpoint must separately select the bounded execution owner for any Phase 129 candidate-publication runtime path, including:
- connection ownership;
- listener/acceptor versus outbound connection roles;
- broker/dispatcher ownership;
- read/write ordering;
- backpressure and connection bounds;
- timeout policy;
- malformed-frame disposition;
- authenticated-session binding to a connection;
- request lifecycle integration;
- response/error framing ownership;
- failure behavior on disconnect;
- shutdown/recovery boundaries.

No such runtime execution selection is made by BW.

## 8. Publication admission and durable reachability remain later authorities

Even after future PRWC runtime prerequisites are selected, successful frame decode must not directly commit reachability.

The existing semantic order remains authoritative:

```text
PRWC frame
    -> require Command
    -> exact PRWP decode
    -> current authenticated publisher session
    -> publish_current_candidates(...)
    -> separately authoritative requester/rendezvous context
    -> exact admission checks
    -> verifier-owned freshness comparison
    -> staged plan validation
    -> durable compare-and-commit
```

BW does not alter or materialize publication admission, provider persistence or reachability commit.

## 9. Existing authorities that remain reusable

Later checkpoints should reuse rather than duplicate:
- `ControlFrame` / PRWC codec and Phase 129 transport bounds;
- BV `candidate_publication_control_frame` pure adapter;
- BQ PRWP v1.0 codec;
- existing `publish_current_candidates(...)` authenticated publisher semantics;
- existing candidate-set validation and freshness authorities;
- existing admission and production reachability-owner semantics;
- existing typed identity separation between `DeviceId`, `SessionId`, `TransportIdentity`, `CandidateId`, endpoint and freshness token.

None of these authorities by themselves fills the missing runtime custody/routing/execution boundaries.

## 10. Authorities that must not be silently transplanted

BW explicitly blocks direct reuse-by-analogy of:
- local IPC `LocalRequestTracker` as PRWC request lifecycle authority;
- desktop startup-probe request IDs as production PRWC allocation semantics;
- PRWM request IDs as PRWC request IDs;
- PRWM `session_auth_wire` as the pre-mesh PRWC authentication path without explicit selection;
- TLS connection identity as logical PRW identity;
- `TransportIdentity` as requester routing identity;
- PRWP freshness token possession as authenticated-session proof;
- socket endpoint identity as logical device identity;
- candidate endpoint or `CandidateId` as rendezvous routing authority.

## 11. Readiness verdict

BW selects only this readiness verdict:

> The repository is **not yet ready for live candidate-publication PRWC runtime wiring** after BV. The pure adapter is ready and validated, but production runtime wiring remains blocked by separately unselected request-ID custody, pre-mesh logical authentication/requester-rendezvous authority, and network execution ownership.

This is a readiness result, not a redesign and not a selection of the missing implementations.

## 12. Safe successor rule

After BW closure, the next checkpoint must remain a bounded selection/readiness checkpoint for one missing runtime prerequisite or for an explicitly justified dependency ordering among them.

No successor may jump directly to:
- Agent/Desktop/Android runtime wiring;
- socket/TLS frame execution;
- broker/listener activation;
- publication admission execution;
- reachability mutation;
- provider/database mutation;
- production networking;
- deployment/restart/recovery;
- merge.

If a future checkpoint selects request-ID custody first, that selection must remain representation/lifecycle-only and must not silently select authentication, routing or networking.

If a future checkpoint selects authentication/routing first, it must not silently allocate PRWC request IDs or activate frame I/O.

If a future checkpoint selects network execution first, it must remain unusable for candidate publication until request-ID custody and authenticated routing authorities are separately satisfied.

## 13. Exact BW source scope

The final BV→BW diff is authorized to contain exactly one path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BW_CANDIDATE_PUBLICATION_PRWC_RUNTIME_PREREQUISITE_READINESS_STAGING.md`

Any Rust/Kotlin source, Cargo manifest, lockfile, workflow, Agent/Desktop/Android implementation, transport implementation, provider/database file, networking configuration or deployment file blocks BW closure.

## 14. Validation and closure requirements

BW may close only after:
- exact BV predecessor lineage remains unchanged;
- exact BV→BW compare contains one docs-only path;
- audited source files remain byte-stable on BW;
- automatically triggered workflows, if any, reach terminal non-failing verdicts;
- immutable Drive audit is uploaded into project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-readback verified;
- rolling Drive evidence `C02E_BRANCH_STATUS.md` passes a fresh predecessor guard, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No source/runtime/networking/deployment mutation is authorized by BW closure.

Gate target remains:
`C03E_BW_CANDIDATE_PUBLICATION_PRWC_RUNTIME_PREREQUISITE_READINESS_AUDITED`
