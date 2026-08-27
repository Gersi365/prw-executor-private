# Phase 152 C03e-CJ — PRWC Request-ID Lifecycle Source Materialization

Status: `STAGED SOURCE — PENDING EXACT-HEAD VALIDATION`

Target gate:
`C03E_CJ_PRWC_REQUEST_ID_LIFECYCLE_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Closed C03e-CI is the authoritative predecessor:
- branch: `phase-152-c03e-ci-prwc-generic-server-accepted-stream-source-materialization-staging`;
- final head: `3850ba841ca97bec0af1fe6e06f131d078bf813f`;
- final tree: `29f6d54f4192bbfe7a285b4e13d58a6d329adbc1`;
- gate: `C03E_CI_PRWC_GENERIC_SERVER_ACCEPTED_STREAM_SOURCE_MATERIALIZED`;
- PR `#206`: body `Status: CLOSED`, draft/open/unmerged.

C03e-CH selected the source-materialization sequence and authorized C03e-CJ after C03e-CI closure as a pure bridge-owned request-ID lifecycle unit.

## 2. Historical semantic authority

C03e-CJ materializes the request-ID lifecycle already selected by C03e-BY. The current authoritative BY branch readback is:
- branch: `phase-152-c03e-by-candidate-publication-prwc-request-id-custody-lifecycle-selection-staging`;
- head: `138eb8e4340acecc7ba3460d1539a4bfd5d862ef`;
- tree: `e5b6b19e9c3b83e51bdbf7aca8fde8d67a0fd1cf`;
- gate target in the BY contract: `C03E_BY_CANDIDATE_PUBLICATION_PRWC_REQUEST_ID_CUSTODY_LIFECYCLE_SELECTED`.

BY selected only the in-memory lifecycle contract. CJ does not broaden or reinterpret that contract.

## 3. Exact bounded purpose

CJ materializes one pure in-memory `prw-remote-bridge` module for locally originated Phase 129 PRWC request-ID custody.

CJ does not materialize:
- listener ownership or accept loops;
- PRWC stream reads/writes;
- authentication or logical-session state;
- requester/rendezvous routing;
- registry currentness or admission;
- candidate-publication execution;
- timeout, retry, cancellation, idempotency or application error policy;
- clock, RNG, persistence, database or provider state;
- Agent/Desktop/Android runtime wiring;
- production networking, activation, deployment, restart or recovery.

## 4. Envelope authority remains unchanged

At exact C03e-CI head, `crates/prw-control-transport/src/lib.rs` remains the Phase 129 envelope authority and defines each `ControlFrame` request ID as a validated non-zero `u64`.

CJ does not modify `prw-control-transport`. It materializes only the bridge-owned lifecycle above that generic envelope.

## 5. Selected connection-local namespace

One `PrwcRequestIdLifecycle` instance represents one live Phase 129 connection-local request-ID namespace.

The lifecycle is:
- process-memory only;
- connection-local only;
- independent of all other connection instances;
- restart-nonpersistent;
- not identity, authorization, routing, freshness or candidate state.

A new lifecycle instance begins allocation again at request ID `1`.

## 6. Selected originator-side allocation

CJ materializes the BY allocation rule exactly:
- first allocated ID is `1`;
- each subsequent allocated ID increments by exactly one;
- `0` is never allocated;
- an allocated ID is never reused during the same lifecycle instance, even after completion or abandonment;
- wraparound is forbidden;
- `u64::MAX` may be allocated once if reached;
- any allocation after that point fails with request-ID-space exhaustion;
- exhaustion does not mutate outstanding state.

The implementation stores the next allocatable ID as `Option<u64>` so `None` represents exhausted non-wrapping space without a sentinel request ID.

## 7. Selected outstanding-request bound

CJ locks the BY bound at:

`PRWC_MAX_OUTSTANDING_REQUESTS = 64`

Allocation checks the outstanding bound before mutating state. When 64 locally originated request IDs are already outstanding, allocation fails closed and does not consume the next ID.

The numerical equality with the local IPC tracker bound remains precedent only; CJ does not reuse the local IPC tracker or its types.

## 8. Selected outstanding collection representation

CJ chooses a private `Vec<u64>` for the bounded outstanding collection.

This is an implementation choice under BY, not a transfer of local IPC authority. The collection is bounded to 64 entries, preserves allocation order for explicit abandonment disposition, requires no dependency, and keeps all lifecycle state pure `std` memory.

No collection choice is exposed as protocol authority.

## 9. Terminal completion

`complete(request_id)`:
- succeeds only when the supplied ID is currently outstanding;
- removes that ID from the outstanding collection;
- never rolls back the monotonic allocator;
- therefore never makes a completed ID reusable;
- rejects unknown IDs fail-closed;
- rejects duplicate terminal completion as the same unknown/non-outstanding failure after the first completion.

CJ does not inspect response/error payload semantics. A later runtime may call completion only after its own protocol checks.

## 10. Connection discard / shutdown abandonment

`abandon_all()`:
- atomically removes every currently outstanding ID from this custody instance;
- returns the abandoned IDs in allocation order for later upper-layer disposition;
- is not successful terminal completion;
- does not roll back or reuse previously allocated IDs.

The later owning connection context is responsible for discarding the lifecycle instance when the connection is retired. CJ itself owns no connection/socket lifetime.

## 11. Fail-closed error classes

CJ exposes distinct pure lifecycle errors for at least:
- outstanding bound reached;
- request-ID space exhausted;
- unknown/non-outstanding terminal completion;
- internal allocator collision.

The internal-collision branch is defensive only: monotonic non-reuse allocation makes collisions impossible under valid internal state. If detected, allocation fails before mutation rather than overwriting state.

## 12. Explicit prohibited derivations

Request IDs are not derived from:
- `DeviceId`;
- `SessionId`;
- `TransportIdentity`;
- `CandidateId`;
- freshness tokens;
- requester/workspace/user/target identifiers;
- socket addresses or ports;
- wall-clock time;
- PRWP payload bytes;
- random values.

The lifecycle requires no clock or RNG.

## 13. Focused validation surface

Inline focused tests in the new module must prove:
- fresh allocation begins at 1 and is monotonic/non-zero;
- completion removes outstanding state without enabling reuse;
- the 64-outstanding bound fails before mutation;
- completing one outstanding request frees capacity for the next new monotonic ID;
- unknown and duplicate completion fail closed;
- abandon-all returns remaining outstanding IDs and does not reset the allocator;
- `u64::MAX` is allocatable once and subsequent allocation fails exhausted;
- defensive internal collision fails before mutation.

Tests may directly construct otherwise unreachable private states only inside the module's `#[cfg(test)]` child module to exercise exhaustion/collision failure branches. No production test hook is authorized.

## 14. Dependency and lockfile rule

CJ requires only `std` and existing crate structure.

No manifest or lockfile mutation is authorized. At the C03e-CI predecessor:
- `crates/prw-remote-bridge/Cargo.toml` blob is `5fd48263be415aac28dee1c71a4031a4a02ad36c`;
- root `Cargo.lock` blob is `eeacde7ee776d35088f746a6d09f823f3391b82b`;
- Android native `Cargo.lock` blob is `cce9ca06190a196661ab38d54a747893e26af95f`.

All three must remain byte-stable through CJ.

## 15. Exact authorized source scope

The final C03e-CI → C03e-CJ net diff is authorized to contain exactly these three paths:

1. `crates/prw-remote-bridge/src/prwc_request_id_lifecycle.rs`;
2. `crates/prw-remote-bridge/src/root.rs`;
3. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CJ_PRWC_REQUEST_ID_LIFECYCLE_SOURCE_MATERIALIZATION_STAGING.md`.

The root change is limited to exposing the new module.

Any fourth path — including Cargo manifest/lockfile, workflow, transport implementation, auth/session/registry/runtime file, Agent/Desktop/Android implementation, provider/database, networking configuration or deployment path — requires a fresh scope audit before mutation.

## 16. Runtime non-activation rule

The new lifecycle module must remain unused by live runtime source in this checkpoint.

CJ does not wire it into:
- `ControlTlsListener` or `ControlTlsServerStream`;
- `remote_server_transport_runtime`;
- candidate-publication framing/execution;
- authentication transaction state;
- product surfaces.

Source availability is not runtime activation.

## 17. Validation and closure

CJ may close only after:
- exact closed CI predecessor lineage remains unchanged;
- exact CI→CJ compare contains only the three authorized paths;
- remote-bridge manifest and both Cargo lockfiles remain byte-stable;
- every automatically triggered workflow reaches a terminal non-failing verdict;
- Rust validation proves locked graph, formatting, Clippy, tests, and workspace build;
- Android validation, if triggered, must reach terminal success for both native adapter and application;
- immutable Drive audit is uploaded and raw-readback verified;
- rolling Drive predecessor guard, append-only prefix proof and raw post-write verification pass;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

Formatter/lint corrections are allowed only inside the same three authorized paths. Any required fourth path blocks correction pending fresh audit.

## 18. Safe successor rule

After!CJ closure, C03e-CK may audit and select the exact bridge PRWA connection-execution source paths using the already-closed CI server primitive, CE verifier-source authority, CF verifier source, and CJ request-ID lifecycle.

C03e-CK path selection must be freshly audited against the exact closed CJ head. CJ does not pre-authorize listener activation, frame-loop activation, authentication cutover, candidate publication, product runtime wiring, credential provisioning, production networking, deployment, merge or branch cleanup.
