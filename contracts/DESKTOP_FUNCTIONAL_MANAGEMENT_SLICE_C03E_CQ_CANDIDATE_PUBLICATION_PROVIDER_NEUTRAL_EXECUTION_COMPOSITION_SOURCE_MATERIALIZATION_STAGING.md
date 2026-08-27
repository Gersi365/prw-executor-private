# Phase 152 C03e-CQ — Candidate Publication Provider-Neutral Execution Composition Source Materialization

Status: STAGED SOURCE MATERIALIZATION

Target gate:
`C03E_CQ_CANDIDATE_PUBLICATION_PROVIDER_NEUTRAL_EXECUTION_COMPOSITION_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Durably closed C03e-CP is the authoritative predecessor:
- branch: `phase-152-c03e-cp-candidate-publication-provider-neutral-execution-composition-selection-staging`
- head: `460d35790aa10b74edcfbfa0413571692cabd8d4`
- tree: `7ddec0f940a2083e3a88278e0c3f6c0472fc0064`
- gate: `C03E_CP_CANDIDATE_PUBLICATION_PROVIDER_NEUTRAL_EXECUTION_COMPOSITION_SELECTED`
- PR #213 body: `Status: CLOSED`, draft/open/unmerged
- immutable Drive audit ID: `1ZqMda7nwRM5w5zpePLImyVVLnT1jyWto`
- immutable audit raw SHA-256: `8d0e3f5f9095a8e86418835a2d4342ec7887e671cb87f9133b05d4db0339fa39`
- rolling Drive image after CP: `855533` bytes / SHA-256 `38388718758cfc296d8db69d0e1aa77d4650c7d78c3dd31add333dcba465c8ef`

CQ preserves exact CP lineage and materializes only the CP-selected provider-neutral semantic execution composition.

## 2. Authorized source boundary

The exact authorized CP -> CQ source boundary is:
1. this source-materialization contract;
2. new `crates/prw-remote-bridge/src/candidate_publication_execution.rs`;
3. `crates/prw-remote-bridge/src/root.rs` only to register/export `candidate_publication_execution`.

No other path is authorized.

In particular CQ authorizes no modification to:
- `crates/prw-remote-bridge/src/prwc_connection_authentication.rs`;
- `crates/prw-remote-bridge/src/requester_rendezvous_authority.rs`;
- `crates/prw-remote-bridge/src/candidate_reachability.rs`;
- `crates/prw-remote-bridge/src/candidate_publication_control_frame.rs`;
- `crates/prw-remote-bridge/src/candidate_publication_wire.rs`;
- `crates/prw-remote-bridge/src/candidate_publication_freshness.rs`;
- `crates/prw-remote-bridge/src/reachability_owner.rs`;
- any Cargo manifest or lockfile;
- Android/Desktop/Agent application source;
- workflows;
- provider/database implementations;
- listener/runtime/networking/deployment configuration.

## 3. Materialized public execution seam

CQ materializes one public bridge-owned helper equivalent to:

```text
execute_authenticated_candidate_publication<S, T, P>(
    connection: &AuthenticatedPrwcConnection,
    command: &CandidatePublicationControlFrame,
    registry: &WorkspaceDeviceRegistry,
    requester_authority: &mut P,
    owner: &mut ProductionReachabilityOwner<S, T>,
) -> Result<ReachabilityCommitOutcome, CandidatePublicationExecutionError>
where
    S: ReachabilityDurableStore,
    T: CandidatePublicationFreshnessTokenSource,
    P: RequesterRendezvousAuthorityProvider
```

The helper borrows the authenticated connection and already-received command. It never exposes or consumes the private `ControlTlsServerStream` and performs no frame I/O.

## 4. Exact execution order

One invocation materializes the CP-selected fail-closed order:
1. publisher logical identity is obtained only from `AuthenticatedPrwcConnection::session()`;
2. presented transport identity, presented freshness and candidates are obtained only from `CandidatePublicationControlFrame::submission()`;
3. `publish_current_candidates(...)` is called first to revalidate current publisher session, current lower transport identity and candidate-set semantics;
4. only after successful publication construction, `RequesterRendezvousAuthorityProvider::authorize_current_for_publisher(...)` is called exactly once with the authenticated publication's logical publisher `DeviceId`;
5. the returned `AuthorizedRequesterRendezvous` remains an owned one-shot grant for this attempt and is neither cloned nor cached;
6. `grant.expected_publisher_device_id()` must equal the authenticated publication publisher `DeviceId` exactly;
7. only after that equality, the existing `ProductionReachabilityOwner::commit_candidate_publication(...)` is called exactly once using the grant requester session, authenticated publication and PRWP presented freshness;
8. the existing `ReachabilityCommitOutcome` is returned only when the existing durable owner succeeds.

No later stage executes after an earlier failure.

## 5. Stable execution error surface

CQ materializes `CandidatePublicationExecutionError` with distinct fail-closed classes for:
- existing `CandidateReachabilityError` during publisher/session/transport/candidate publication construction;
- existing `RequesterRendezvousAuthorityError` from the CM authority provider;
- exact expected-publisher mismatch;
- existing `ReachabilityOwnerError` from the durable reachability owner.

Nested source errors remain available through `std::error::Error::source()` where an underlying error exists. Expected-publisher mismatch has no fabricated nested source.

`AuthenticatedPrwcCommandReceiveError` is deliberately absent because CO receive has already completed before CQ execution begins.

## 6. Testability without a second production authority

The materialization may use a private, module-local commit adapter solely to test execution ordering without constructing or exposing transport state. Production execution remains statically bound to `ProductionReachabilityOwner<S, T>` and its existing `commit_candidate_publication(...)` implementation.

The private adapter:
- is not public API;
- does not select a second durable authority;
- does not expose storage or plan mutation;
- does not alter the production function signature;
- exists only so focused unit tests can prove no commit follows prerequisite failure and authority precedes commit.

## 7. Focused required tests

CQ source includes focused tests proving at least:
- publisher/current-registry admission failure invokes neither requester/rendezvous authority nor commit;
- requester/rendezvous authority failure is preserved and prevents commit;
- expected-publisher mismatch fails closed before commit;
- successful authority selection occurs exactly once before exactly one commit attempt;
- the commit attempt receives the authenticated publisher identity and exact PRWP presented freshness;
- nested authority/reachability errors remain discoverable as error sources.

These tests do not activate sockets, listeners, async runtimes, production networking or a concrete rendezvous backend.

## 8. PRWC request-ID custody remains unchanged

CQ deliberately never reads `CandidatePublicationControlFrame::request_id()`.

The outer request ID remains peer-originated correlation only and is not:
- allocated locally;
- inserted into `PrwcRequestIdLifecycle`;
- used as publisher/requester authority;
- used as publication freshness;
- used as a durable expected version;
- used as candidate identity.

Because CQ borrows the command, later separately gated response/Error-frame composition may still use the preserved correlation value.

## 9. Existing authority ownership remains unchanged

CQ does not duplicate validation owned by existing modules.

`publish_current_candidates(...)` remains authoritative for:
- current publisher authenticated-session validation;
- current publisher transport-identity validation;
- bounded candidate-set semantic construction.

`RequesterRendezvousAuthorityProvider` remains authoritative for:
- exactly one current server-side requester/rendezvous selection for the authenticated publisher lookup selector.

`ProductionReachabilityOwner::commit_candidate_publication(...)` remains authoritative for:
- requester currentness;
- publisher currentness;
- workspace equality;
- exact target-plan identity;
- current target transport;
- presented freshness equality;
- staged candidate refresh validation;
- replacement verifier freshness;
- durable expected-current CAS;
- recovery transition on stale/ambiguous durable state;
- local install and old-traversal invalidation after commit.

CQ performs no direct durable-store call and no direct connectivity-plan mutation.

## 10. No concrete requester/rendezvous provider

CQ remains generic over `RequesterRendezvousAuthorityProvider` and selects no concrete provider lifecycle or representation.

No database/storage product, schema, TTL, clock, synchronization mechanism, server process, cleanup schedule, provider credentials or bootstrap wiring is introduced.

A concrete requester/rendezvous provider remains mandatory before production runtime activation can supply authoritative grants.

## 11. No response or loop semantics

CQ does not:
- read another PRWC frame;
- write any PRWC frame;
- select Response versus Error framing;
- encode replacement freshness for a peer;
- define write-failure terminalization;
- loop over commands;
- retry authority or commit;
- reconnect;
- spawn a task/thread;
- poll a runtime;
- bind or accept a listener.

Each invocation is one bounded semantic execution attempt over one already-received typed command.

## 12. No dependency change

All production and focused-test dependencies required by CQ already exist in `prw-remote-bridge` at CP:
- `prw-connectivity`;
- `prw-control-plane`;
- `prw-core`;
- `prw-registry`;
- `prw-session`;
- existing remote-bridge modules;
- existing dev dependencies `aws-lc-rs` and `prw-device-identity-signer` used by established disposable session fixtures.

CQ requires no manifest or lockfile change.

Any manifest/lock mutation blocks closure unless a concrete compiler contradiction is separately audited and authorized.

## 13. CP audit-basis source preservation

At the CQ base, the following CP/CO source blobs are authoritative and must remain byte-stable:
- `prwc_connection_authentication.rs` — `1af3aa2851e87e3a4f7990c98e105e62141d8db1`;
- `candidate_publication_control_frame.rs` — `20ff7d2bc5f32596a3c0696aa387e6735f8f2031`;
- `candidate_publication_wire.rs` — `299042938b38b65b78f737926f74b8567e5046fb`;
- `candidate_publication_freshness.rs` — `fd7c2f095999b6a6479be79c562637fe5f46634c`;
- `requester_rendezvous_authority.rs` — `260024b7aca2aea6109dc72e778bcda3dcca8038`;
- `candidate_reachability.rs` — `51b294cfb3772925651a05bdcb034cd051204efb`;
- `reachability_owner.rs` — `8d0e65c3fc0bd646c257199d4f55be65fa3f792d`;
- `prw-remote-bridge/Cargo.toml` — `5fd48263be415aac28dee1c71a4031a4a02ad36c`;
- root `Cargo.lock` — `eeacde7ee776d35088f746a6d09f823f3391b82b`;
- Android native `Cargo.lock` — `cce9ca06190a196661ab38d54a747893e26af95f`.

Only `root.rs` is authorized to change among the prior source paths, and only by one module registration/export line for the new CQ module.

## 14. Validation requirements

CQ closure requires exact final-head validation through every automatically triggered relevant workflow.

For Rust, closure requires terminal success of the canonical locked graph / formatting / Clippy / tests / build workflow on the exact final CQ head.

If Android validation triggers, it must reach terminal success before closure. If it does not trigger for the bounded path set, CQ must explicitly record that no Android PASS is claimed.

Disposable provider workflows that do not apply must be recorded as `SKIPPED`, never `PASS`.

Any failure is classified before correction; source defects may be corrected only within this exact CQ boundary. Environment/tooling failures must not be disguised as source defects.

## 15. Closure evidence

CQ may close only after:
- exact CP predecessor guard remains unchanged;
- CP -> CQ compare has exact CP merge base and no unauthorized path;
- audit-basis source/locks remain byte-stable except authorized `root.rs` registration;
- exact-head workflow verdicts are terminal and non-failing;
- immutable Drive audit is uploaded and raw-readback verified byte-for-byte;
- rolling Drive predecessor guard and append-only byte-prefix proof pass;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged.

## 16. Safe successor after CQ

After durable CQ closure, perform a fresh read-only prerequisite audit before selecting the next candidate-publication prerequisite.

The audit must choose explicitly among still-unmaterialized prerequisites such as:
- concrete requester/rendezvous provider lifecycle/representation;
- response/Error-frame semantic composition;
- later bounded command-loop/runtime assembly.

No successor may jump directly to listener activation, production networking, deployment or merge.

## 17. Completion meaning

Closure means only that the CP-selected provider-neutral candidate-publication semantic execution composition exists in validated Rust source with focused fail-closed tests.

It does not mean a concrete requester/rendezvous provider exists, a response is written, a frame loop runs, a listener is active, production networking is activated, anything is deployed, or any PR is merged.

Target gate:
`C03E_CQ_CANDIDATE_PUBLICATION_PROVIDER_NEUTRAL_EXECUTION_COMPOSITION_SOURCE_MATERIALIZED`
