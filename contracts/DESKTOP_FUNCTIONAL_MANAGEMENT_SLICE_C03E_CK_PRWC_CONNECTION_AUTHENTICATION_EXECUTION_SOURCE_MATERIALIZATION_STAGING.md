# Phase 152 C03e-CK — PRWC Connection Authentication Execution Source Materialization

Status: `STAGED SOURCE — PENDING EXACT-HEAD VALIDATION`

Target gate:
`C03E_CK_PRWC_CONNECTION_AUTHENTICATION_EXECUTION_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Closed C03e-CJ is the authoritative predecessor:

- branch: `phase-152-c03e-cj-prwc-request-id-lifecycle-source-materialization-staging`;
- head: `78d6e922e9e9f540bede020d56e2205e4ad2257b`;
- tree: `7c0138520702ba78d6b555795cd31359fc95f707`;
- gate: `C03E_CJ_PRWC_REQUEST_ID_LIFECYCLE_SOURCE_MATERIALIZED`;
- PR `#207`: body `Status: CLOSED`, draft/open/unmerged.

C03e-CH selected the sequence CI -> CJ -> CK. CI materialized the generic server accepted stream; CJ materialized bridge-owned BY request-ID custody. CK is the first separately gated bridge source allowed to compose those prerequisites with the already-closed PRWA authentication authorities.

## 2. Historical semantic authorities

CK materializes, but does not broaden, these closed decisions:

- C03e-BZ: one completed `AuthenticatedDeviceSession` is the connection-local logical-session binding above transport; current registry revalidation remains mandatory.
- C03e-CA: one PRWA transaction uses Begin -> Challenge -> Proof -> terminal Authenticated/Rejected, preserving one peer-originated PRWC request ID and forbidding same-connection retry/parallel authentication.
- C03e-CE/CF: verifier-owned fresh `SessionId` and checked verifier time source.
- C03e-CG: bridge owns accepted-stream authentication execution ordering, pending-session cleanup, and post-proof delivery semantics.
- C03e-CI: generic `ControlTlsServerStream` proves only TCP/TLS1.3/exact-ALPN mechanics.
- C03e-CJ: bridge-owned connection-local request-ID lifecycle exists independently from peer-originated authentication correlation.

No CK rule may infer logical identity from TLS, socket address, PRWC request ID, `TransportIdentity`, requester/rendezvous state, or candidate-publication data.

## 3. Exact bounded purpose

CK materializes one bridge-owned authentication execution module for one already-accepted generic PRWC server stream.

The module may:

1. take ownership of one `ControlTlsServerStream`;
2. create one fresh `PrwcRequestIdLifecycle` for the same connection namespace;
3. execute exactly one pre-mesh PRWA authentication transaction;
4. compose the existing PRWA codec, verifier source, `SessionAuthenticationService`, and `WorkspaceDeviceRegistry`;
5. preserve the Begin request ID through Challenge, Proof correlation, Authenticated, and best-effort pre-proof Rejected;
6. return one authenticated connection wrapper only after terminal Authenticated write success.

CK does not bind a listener, call `ControlTlsListener::accept`, spawn an accept loop/task/thread, admit Command semantics, select requester/rendezvous state, execute candidate publication, dispatch capabilities, provision credentials, or activate product runtime.

## 4. Initial connection state

Every CK wrapper begins logically unauthenticated regardless of successful generic TLS establishment.

Construction performs no frame I/O and no authentication call.

The unauthenticated wrapper is consumed by `authenticate(...)`. Failure consumes/drops the connection wrapper, preventing same-connection retry or reauthentication through the CK API.

## 5. Exact authentication ordering

CK must preserve the C03e-CG ordering:

1. read one bounded `ControlFrame`;
2. decode exactly PRWA Begin;
3. treat Begin `DeviceId` as an untrusted selector;
4. resolve one current registered device binding and require its lifecycle to be enrolled;
5. obtain one fresh CF verifier context;
6. call `SessionAuthenticationService::begin_session(...)` exactly once;
7. record the verifier `SessionId` as pending cleanup metadata;
8. encode and write Challenge on the Begin request ID;
9. read one next bounded frame;
10. require exact outer request-ID correlation;
11. decode exactly PRWA Proof;
12. require exact verifier-issued `SessionId` correlation;
13. reconstruct typed `SessionAuthProof`;
14. obtain a fresh verifier-time observation;
15. call `submit_proof(...)` exactly once;
16. call `WorkspaceDeviceRegistry::validate_authenticated_session(...)`;
17. encode and write terminal Authenticated on the original request ID;
18. only after that write succeeds, return/install the `AuthenticatedDeviceSession` in the authenticated connection wrapper.

No Command frame is processed by this source.

## 6. Pending cleanup boundary

No pending cleanup obligation exists before `begin_session(...)` succeeds.

After successful `begin_session(...)`, every terminal failure before successful `submit_proof(...)` must call `abort_pending_session(...)` exactly once before return.

That includes:

- Challenge encode failure;
- Challenge write failure;
- Proof read failure;
- request-ID mismatch;
- Proof decode/order failure;
- PRWA `SessionId` mismatch;
- fresh verifier-time failure;
- `submit_proof(...)` failure.

After successful cleanup, CK may attempt exactly one generic PRWA Rejected write on the original transaction request ID. Rejected delivery is best effort and never permits retry or continued connection use.

If explicit pending cleanup itself fails, CK fails closed with a distinct cleanup classification and does not pretend cleanup succeeded.

## 7. Post-proof failure boundary

After successful `submit_proof(...)`, the service has completed authentication and removed pending state.

Therefore:

- registry currentness failure;
- Authenticated encoding failure;
- Authenticated write/delivery failure

discard the connection without calling `abort_pending_session(...)` and without inventing a rollback API for the completed service session.

The authenticated connection wrapper is returned only after the terminal Authenticated frame has been written successfully.

## 8. Request-ID authority separation

PRWA authentication is peer-originated. CK preserves the inbound Begin request ID for every correlated response and never allocates a replacement from C03e-CJ custody.

The connection still owns one fresh `PrwcRequestIdLifecycle` so the selected future bridge connection context has the already-materialized BY state. CK itself allocates no locally originated request ID.

On failed authentication the lifecycle is explicitly abandoned before the consumed wrapper is dropped.

Request IDs remain correlation only; they are not identity, authorization, routing, freshness, requester, candidate, or session identifiers.

## 9. Current registry authority

Begin device selection uses the current registered-device table only as an untrusted lookup target and requires the stored binding to remain enrolled.

After successful proof verification, CK performs the mandatory full `validate_authenticated_session(...)` revalidation before Authenticated delivery.

This preserves current membership/device authority at semantic commit time. CK does not mutate registry state.

## 10. Accepted-stream authority

`ControlTlsServerStream` remains generic transport authority from C03e-CI.

CK consumes that stream but does not alter `prw-control-transport`, expose socket addresses as identity, select mTLS client identity, or modify TLS/ALPN behavior.

No listener ownership or accept-loop scheduling is materialized by CK.

## 11. Validation seam

The production public wrapper owns an actual `ControlTlsServerStream`.

Inside the module, one private frame-I/O seam mirrors only `read_frame` / `write_frame` so focused deterministic inline tests can execute the exact authentication state machine without adding a second socket fixture or production test hook.

The seam is private and is not a protocol/provider authority.

## 12. Focused validation requirements

Inline tests must prove at least:

- valid Begin -> Challenge -> Proof -> Authenticated ordering;
- exact request ID preserved through the successful transaction;
- request-ID mismatch aborts pending state and rejects the original transaction;
- PRWA `SessionId` mismatch aborts pending state before proof submission;
- Challenge write failure cleans pending state;
- terminal Authenticated write failure after successful proof does not call pending rollback and leaves the service-completed session intact;
- unknown Begin device fails before pending state and emits only generic Rejected when possible.

Canonical workspace validation remains authoritative over inline tests.

## 13. Dependency and lockfile rule

Current C03e-CJ `prw-remote-bridge` already depends on every crate required by CK:

- `prw-control-plane`;
- `prw-control-transport`;
- `prw-core`;
- `prw-registry`;
- `prw-session`.

Its existing dev-dependencies already include the disposable signer/crypto material required by inline tests.

Therefore CK authorizes no Cargo manifest or lockfile mutation.

Exact predecessor blobs that must remain byte-stable:

- `crates/prw-remote-bridge/Cargo.toml`: `5fd48263be415aac28dee1c71a4031a4a02ad36c`;
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`;
- Android native `Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`.

## 14. Exact authorized source scope

The final C03e-CJ -> C03e-CK net diff is authorized to contain exactly these three paths:

1. `crates/prw-remote-bridge/src/prwc_connection_authentication.rs`;
2. `crates/prw-remote-bridge/src/root.rs`;
3. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CK_PRWC_CONNECTION_AUTHENTICATION_EXECUTION_SOURCE_MATERIALIZATION_STAGING.md`.

The root change is limited to one module export.

Any fourth path requires a fresh scope audit before mutation.

## 15. Explicit exclusions

CK does not materialize or activate:

- changes to `prw-control-transport`;
- changes to `prw-session`, `prw-registry`, or `prw-control-plane`;
- manifest or lockfile edits;
- listener bind/accept loops;
- background tasks, async runtime ownership, worker pools, retry/backoff, reconnect or reauthentication;
- requester/rendezvous provider or storage;
- candidate-publication Command decode/admission/execution;
- `TransportIdentity` inference from PRWC;
- capability authorization or dispatch;
- Agent/Desktop/Android runtime wiring;
- credential source/provisioning/storage/rotation;
- production bind address or network configuration;
- database/provider mutation;
- deployment, restart, recovery, merge, rebase, or branch cleanup.

Source materialization is not runtime activation.

## 16. Closure requirements

CK may close only after:

- exact CJ predecessor lineage remains unchanged;
- exact CJ -> CK compare contains only the three authorized paths;
- remote-bridge manifest and both Cargo lockfiles remain byte-stable;
- every automatically triggered workflow reaches terminal non-failing verdict;
- Rust validation proves locked graph, formatting, Clippy, tests, and workspace build;
- Android validation, if triggered, reaches terminal success for native adapter and application;
- immutable Drive audit is uploaded and raw-readback verified;
- rolling Drive predecessor guard, append-only prefix proof, and raw post-write verification pass;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

Formatter/lint corrections are permitted only within the same three authorized paths. Any required fourth path blocks correction pending a fresh audit.

## 17. Safe successor rule

CK closes only pre-mesh PRWA connection authentication execution.

Candidate-publication Command execution, requester/rendezvous injection/selection use, publication admission, reachability compare-and-commit, capability dispatch, product listener ownership, production credentials, and production network activation remain later separately gated work.

No successor may treat CK source availability as authorization to deploy, merge, bind a production listener, cut over authentication, or execute candidate publication.
