# Phase 152 C03e-BZ — Candidate Publication PRWC Pre-Mesh Authentication / Requester-Rendezvous Authority Selection

Status: STAGED SELECTION

Gate target:
`C03E_BZ_CANDIDATE_PUBLICATION_PRWC_PRE_MESH_AUTHENTICATION_REQUESTER_RENDEZVOUS_AUTHORITY_SELECTED`

## 1. Exact predecessor

Closed C03e-BY is the authoritative predecessor:
- branch: `phase-152-c03e-by-candidate-publication-prwc-request-id-custody-lifecycle-selection-staging`;
- head: `138eb8e4340acecc7ba3460d1539a4bfd5d862ef`;
- tree: `e5b6b19e9c3b83e51bdbf7aca8fde8d67a0fd1cf`;
- gate: `C03E_BY_CANDIDATE_PUBLICATION_PRWC_REQUEST_ID_CUSTODY_LIFECYCLE_SELECTED`;
- PR `#194`: body `Status: CLOSED`, draft/open/unmerged.

BY selected bridge-owned Phase 129 request-ID custody/lifecycle only. It did not select logical authentication wire composition, authenticated-session binding, requester/rendezvous authority, server/listener execution or live frame I/O.

## 2. Exact bounded purpose

BZ selects only the authority composition required before candidate-publication PRWC runtime execution may be designed:

1. how one Phase 129 connection obtains and retains an authenticated logical PRW session identity above TLS;
2. how candidate-publication admission obtains requester/rendezvous authority without accepting it from publisher-controlled payload or correlation fields.

BZ is docs-only. It does not materialize an authentication codec, provider, session store, routing table, listener, broker, dispatcher, request-ID implementation, socket I/O, publication execution, networking or deployment.

## 3. Existing authentication semantic authority

`crates/prw-session/src/lib.rs` remains authoritative for enrolled-device logical session authentication semantics.

`SessionAuthenticationService` already:
- creates fresh verifier challenges;
- validates enrolled-device identity binding;
- verifies typed session-authentication proof;
- returns `AuthenticatedDeviceSession` only after successful proof;
- keeps authentication separate from capability authorization and transport selection.

`AuthenticatedDeviceSession` already carries the authenticated `SessionId`, `WorkspaceId`, `UserId`, `DeviceId` and canonical public identity.

BZ reuses these semantics and this typed authenticated result. It does not invent a second logical identity type for Phase 129.

## 4. Currentness authority remains the registry

`crates/prw-registry/src/lib.rs` remains authoritative for current membership/device/session revalidation.

`WorkspaceDeviceRegistry::validate_authenticated_session(...)` must remain mandatory at semantic-use boundaries. A successfully proven session snapshot is not permanently current merely because authentication once succeeded.

The registry remains separately authoritative for current `TransportIdentity` binding through `validate_transport_identity(...)`.

BZ does not make transport identity part of logical session proof and does not make authenticated session proof equivalent to transport-currentness proof.

## 5. Existing PRWM authentication wire is not the PRWC pre-mesh wire

`crates/prw-remote-bridge/src/session_auth_wire.rs` remains an existing bounded adapter for PRWM `SessionAuthentication` messages.

BZ preserves BO/BW's prohibition against silently transplanting that byte representation onto Phase 129 PRWC.

The existing Phase 129 `ControlMessageKind::Authentication` is selected only as the outer semantic envelope category for a future pre-mesh authentication exchange. Its payload schema, version/magic, challenge/proof operations, response/error mapping and exact frame sequence remain unselected and require a separate checkpoint before source materialization.

Therefore:
- PRWM session-auth semantic values may be reused at the typed semantic layer;
- PRWM wire bytes are not declared PRWC wire bytes;
- Phase 129 TLS success is not logical PRW authentication;
- `ControlMessageKind::Authentication` alone is not proof that authentication succeeded.

## 6. Selected per-connection authenticated-session ownership

The future bridge-owned Phase 129 connection/runtime context owns exactly one authenticated logical-session binding after successful pre-mesh authentication.

Selected lifecycle:

```text
transport-established / logical-session-unauthenticated
    -> bounded pre-mesh authentication transaction
    -> authenticated connection context carrying AuthenticatedDeviceSession
    -> registry revalidation at each semantic admission boundary
    -> connection discard/shutdown removes the connection-local binding
```

Rules:
- a candidate-publication `Command` must not be semantically admitted before the connection has a completed authenticated logical session binding;
- the authenticated session is server/verifier-produced state, never reconstructed from PRWP fields;
- the publisher cannot replace the bound logical session by supplying another `SessionId`, `DeviceId`, `WorkspaceId`, user, public key or transport identity inside candidate-publication payloads;
- one live Phase 129 connection does not switch to a different authenticated logical session in place; authenticating a different logical session requires a new connection context or explicit future reauthentication lifecycle selected by a separate checkpoint;
- connection discard removes the binding but does not revoke the underlying account/device/session globally;
- registry currentness must still be checked when candidate publication is processed.

BZ does not select storage beyond this connection-local binding and does not select persistence across process restart.

## 7. Publisher identity source for candidate publication

For an inbound candidate-publication command, the logical publisher session supplied to `publish_current_candidates(...)` must be the current connection's server-side authenticated `AuthenticatedDeviceSession` binding.

The decoded PRWP submission continues to supply only:
- untrusted presented `TransportIdentity`;
- untrusted presented verifier freshness token;
- bounded typed candidate vector.

The logical publisher `DeviceId` is derived from the authenticated session and current registry authority, exactly as existing `publish_current_candidates(...)` requires.

The presented `TransportIdentity` remains separately checked against that authenticated logical device. It is not the source of logical identity.

## 8. Existing candidate-admission requester authority

`crates/prw-remote-bridge/src/candidate_reachability.rs` and `reachability_owner.rs` already require requester identity as a separate authenticated input:

- `validate_authenticated_publication_admission(...)` takes `requester_session: &AuthenticatedDeviceSession` separately from the publisher publication;
- `ProductionReachabilityOwner::commit_candidate_publication(...)` likewise receives requester session separately;
- both requester and publisher are revalidated against current registry state;
- workspace equality and exact publication target identity are checked before mutation.

BZ preserves this separation. Requester identity is not a field of PRWP and is not inferred from PRWC `request_id`, `TransportIdentity`, endpoint, freshness or candidate metadata.

## 9. Selected requester/rendezvous authority

Candidate-publication execution must consume a **server-side rendezvous selection** produced independently of the publisher submission.

The selected semantic content of that authority is:
- one authenticated requester session snapshot (`AuthenticatedDeviceSession`), obtained from an authenticated PRW control-plane context or an equivalently authoritative server-side session provider;
- one expected publisher logical `DeviceId`, selected by server-side scheduling/rendezvous state;
- provenance that the requester requested/awaits reachability for that expected publisher within the requester's logical workspace context.

The rendezvous authority is not serialized inside PRWP candidate publication and is not supplied by the publisher.

A later provider/representation checkpoint must define how this server-side selection is stored/looked up and how staleness/abandonment is represented. BZ does not choose a database, broker, table key, persistence product or network service.

## 10. Required rendezvous admission checks

Before `ProductionReachabilityOwner::commit_candidate_publication(...)` may be invoked, later runtime composition must fail closed unless all of the following hold:

1. publisher connection has a completed authenticated logical session binding;
2. publisher authenticated session is registry-current;
3. publisher-presented `TransportIdentity` is current for that authenticated publisher device;
4. an independent server-side rendezvous selection exists;
5. rendezvous requester session is registry-current;
6. rendezvous expected publisher `DeviceId` equals the publisher device derived from the authenticated publisher session;
7. requester and publisher are currently in the same workspace;
8. the selected `ProductionReachabilityOwner` / current plan refers to the exact publisher peer identity produced by existing publication semantics;
9. only after these authority checks may existing freshness/current-plan/durable-commit ordering continue.

Existing candidate admission may repeat some of these checks. Revalidation is intentional and must not be optimized away merely because rendezvous selection was valid earlier.

## 11. Requester/rendezvous provider failure semantics

A later rendezvous provider must fail closed on at least:
- no current rendezvous selection;
- stale/retired rendezvous selection;
- requester session no longer current;
- expected publisher mismatch;
- cross-workspace mismatch;
- ambiguous/multiple authoritative selection where exactly one is required;
- provider unavailability when current authority cannot be established.

Provider absence or ambiguity must not be interpreted as permission to derive requester identity from publisher input.

## 12. Identity separation remains explicit

BZ preserves these non-interchangeable identities:
- `AuthenticatedDeviceSession` / `DeviceId` = logical authenticated PRW identity;
- `SessionId` = authentication-session correlation within the typed auth authority;
- `TransportIdentity` = independently rotatable lower transport certificate identity;
- PRWC `request_id` = one-connection outer message correlation only;
- `CandidateId` = candidate-plan correlation only;
- candidate endpoint/path kind = connectivity metadata only;
- publication freshness token = verifier-owned replay/currentness state only.

None of the lower-level or correlation identifiers may be promoted to logical authentication or rendezvous authority.

## 13. Selected dependency order after BZ

The pre-execution dependency order is now:

```text
Phase 129 transport connection
    -> future PRWC-specific pre-mesh authentication wire exchange
    -> SessionAuthenticationService semantic verification
    -> connection-local AuthenticatedDeviceSession binding
    -> current registry revalidation
    -> candidate-publication Command / exact PRWP decode
    -> publish_current_candidates(...) using bound publisher session
    -> independent server-side requester/rendezvous selection
    -> requester/publisher/expected-device/workspace/target admission
    -> verifier freshness comparison
    -> staged candidate validation
    -> durable compare-and-commit
```

PRWC request-ID custody selected in BY remains independent correlation state and is not an authentication/rendezvous dependency input.

## 14. Explicit non-selections

BZ does not select or materialize:
- PRWC authentication payload codec/schema/version/magic;
- challenge/proof operation codes or exact frame sequence;
- authentication response/error payloads;
- `SessionId` allocation/custody changes;
- request-ID source materialization;
- authenticated-session persistence/store/provider beyond connection-local ownership semantics;
- rendezvous provider storage schema or database;
- routing/broker/listener implementation;
- server socket or accepted-stream implementation;
- frame read/write loop;
- timeout/retry/idempotency/deduplication;
- publication response/ack/error protocol;
- publication admission execution source;
- reachability mutation beyond already-existing semantic authorities;
- Agent/Desktop/Android runtime wiring;
- STUN/ICE/TURN/relay/QUIC production activation;
- host/systemd/firewall/NAT/route/DNS/TUN/TAP changes;
- deployment/restart/recovery;
- merge.

## 15. Safe successor rule

After BZ closure, the next safe checkpoint is a docs-only **PRWC pre-mesh authentication wire schema / transaction selection** that maps the existing typed session-auth challenge/proof semantics into bounded Phase 129 `Authentication` envelopes without reusing PRWM bytes by assumption.

That successor must remain pure protocol selection: no listener, socket I/O, session-provider persistence, requester routing, publication execution, Agent activation or networking.

A later checkpoint must separately select requester/rendezvous provider representation/lifecycle if implementation requires more than the semantic authority fixed here.

## 16. Exact BZ source scope

The final BY→BZ diff is authorized to contain exactly one path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BZ_CANDIDATE_PUBLICATION_PRWC_PRE_MESH_AUTHENTICATION_REQUESTER_RENDEZVOUS_AUTHORITY_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, Agent/Desktop/Android implementation, transport implementation, provider/database file, networking configuration or deployment path blocks BZ closure.

## 17. Audit basis

Exact BY source authorities used by this selection must remain byte-stable through BZ:
- `crates/prw-session/src/lib.rs` — `845d40a3c7879f4ee813e140123d945fa1e72aff`;
- `crates/prw-registry/src/lib.rs` — `cd7eb52eea19354620ff8ad26c9b8aad3f9c5eb6`;
- `crates/prw-remote-bridge/src/session_auth_wire.rs` — `492d3e938fcbc75907b345750928717c957204e8`;
- `crates/prw-remote-bridge/src/candidate_reachability.rs` — `51b294cfb3772925651a05bdcb034cd051204efb`;
- `crates/prw-remote-bridge/src/reachability_owner.rs` — `8d0e65c3fc0bd646c257199d4f55be65fa3f792d`;
- `crates/prw-control-transport/src/lib.rs` — `34b0a898572adaa2f77251ca2e9c66ea29973e95`.

## 18. Validation and closure

BZ may close only after:
- exact closed BY predecessor lineage remains unchanged;
- exact BY→BZ compare contains one docs-only path;
- all audit-basis source files remain byte-stable;
- every automatically triggered workflow reaches terminal non-failing verdict;
- immutable Drive audit is uploaded under project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-readback verified;
- rolling Drive predecessor guard, append-only prefix proof and raw post-write verification pass;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No source/runtime/networking/deployment mutation is authorized by BZ closure.
