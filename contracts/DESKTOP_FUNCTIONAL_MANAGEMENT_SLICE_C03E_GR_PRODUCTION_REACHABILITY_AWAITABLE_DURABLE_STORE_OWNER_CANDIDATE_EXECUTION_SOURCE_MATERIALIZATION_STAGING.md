# Phase 152 — C03e-GR Production Reachability Awaitable Durable-Store / Owner / Candidate-Execution Source Materialization Staging

Status: SOURCE_MATERIALIZATION_STAGING

Target gate:
`C03E_GR_PRODUCTION_REACHABILITY_AWAITABLE_DURABLE_STORE_OWNER_CANDIDATE_EXECUTION_SOURCE_MATERIALIZED`

## 1. Purpose

C03e-GR materializes only the provider-neutral awaitable durable-I/O source boundary selected by canonically CLOSED C03e-GQ.

GR evolves the existing `ReachabilityDurableStore` authority, durable-I/O methods on the existing `ProductionReachabilityOwner`, candidate-publication semantic composition, authenticated freshness resynchronization durable reload, Agent production-owner recovery/custody, and the existing dormant Agent current-authority/requester composition only as far as exact compiler-discovered awaitability propagation requires.

GR does not choose or construct a persistence provider, database, schema, serializer, credential set, async runtime, listener, task, worker, watcher, Agent startup population path, candidate ingress handoff, network path, deployment or merge.

## 2. Exact predecessor

The exact predecessor is canonically CLOSED C03e-GQ.

- GQ branch: `phase-152-c03e-gq-production-reachability-durable-store-execution-model-prerequisite-semantics-selection-staging`
- GQ final head: `4065c9e75a6e711914f0f6f042f544a788f5eefa`
- GQ final tree: `7b957505c1fd1734d866f0d8e5df29e0288daf01`
- GQ contract blob: `2d6c9138c27d7d0c47ef444036a29ae66ac9efb9`
- GQ PR: `#319`
- GQ remains draft/open/unmerged with `Status: CLOSED`.

GR begins exactly from the GQ final head and does not amend GQ.

## 3. Fresh exact-GQ source audit and compiler-discovered propagation boundary

The initial exact-GQ audit identified the production owner/candidate durable call graph. Automatic exact-head compiler validation then exposed additional existing callsites that necessarily inherit awaitability from the same durable store. Those findings are recorded as scope evidence rather than hidden or treated as unrelated failures.

### 3.1 Existing durable authority

`crates/prw-remote-bridge/src/reachability_owner.rs` at GQ blob
`8d0e65c3fc0bd646c257199d4f55be65fa3f792d` defined synchronous:

- `ReachabilityDurableStore::load_current(...)`;
- `ReachabilityDurableStore::compare_and_commit(...)`.

Those calls were used by owner operations whose semantics require durable I/O:

- `ProductionReachabilityOwner::recover(...)`;
- `ProductionReachabilityOwner::commit_candidate_publication(...)`;
- `ProductionReachabilityOwner::retire_noncurrent_lifecycle(...)`;
- `ProductionReachabilityOwner::reload_from_store(...)`.

Pure accessors, exact peer identity comparison, candidate staging, traversal construction and observation do not become asynchronous merely because durable authority becomes awaitable.

### 3.2 Existing candidate composition

`crates/prw-remote-bridge/src/candidate_publication_execution.rs` at GQ blob
`4a0f3a23cdc45e5d076d152e84724e997c789b1b` reached the production owner's durable commit through a synchronous private candidate commit seam.

The existing ordering remains:

`authenticated publication construction -> requester/rendezvous authority -> exact publisher equality -> production-owner durable candidate commit`.

GR preserves this order and makes only durable-dependent execution awaitable.

### 3.3 Existing production-owner integration test

`crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs` at GQ blob
`de0772e8f8198b168fcfb47c4b5b66771ab40d22` used a test-only in-memory `ReachabilityDurableStore` and directly exercised recovery, commit, reload and retirement.

This remains the focused executable regression surface for C02e durability/fail-closed semantics while execution shape changes.

### 3.4 Existing Agent custody

`crates/prw-agent/src/production_reachability_owner_custody.rs` at GQ blob
`4d1da871f7451302ead98e16c7d37b006c6d7ded` delegated authoritative recovery to `ProductionReachabilityOwner::recover(...)` and exposed bounded exact-owner custody operations.

At GQ those operations were synchronous. GR initially propagated awaitability only into custody recovery, then exact-head compiler validation proved that dormant Agent candidate composition also needs a bounded async owner-custody counterpart so the exact selected mutable owner can remain lexically borrowed across one durable commit await.

The existing synchronous `with_owner_mut` and `with_owner_mut_for_peer` remain available for non-durable bounded access. GR adds `with_owner_mut_async` and `with_owner_mut_for_peer_async` only for explicitly awaitable operations; neither returns an owner reference, map entry, guard, task or runtime.

### 3.5 Repository async precedent

The exact repository already contains provider-neutral async ports using `impl Future + Send` while leaving executor/runtime ownership outside the bridge.

GR follows that source pattern. It does not add `async-trait`, Tokio to `prw-remote-bridge`, or another bridge runtime/dependency.

### 3.6 Compiler-discovered authenticated freshness resynchronization propagation

The original five-path GR candidate omitted one existing `ReachabilityDurableStore::load_current(...)` call in:

`crates/prw-remote-bridge/src/reachability_freshness_wire.rs`.

Automatic exact-head Rust validation #1379 on head `de926bb85e3242fa6f0ea28c6d01a3b4fe48e9e8` passed the locked dependency graph and rustfmt, then Clippy failed because `authenticated_current_token_resynchronization(...)` still called the now-awaitable durable load synchronously.

The correction makes only that authenticated resynchronization function awaitable and awaits exactly one durable `load_current` after existing registry/session/transport currentness validation. No registry ordering, peer derivation, freshness lifecycle interpretation, failure taxonomy or token-delivery semantics change.

The corresponding integration test `crates/prw-remote-bridge/tests/reachability_freshness_wire.rs` necessarily follows the same trait/callsite shape. Its deterministic in-memory store remains test-only.

### 3.7 Compiler-discovered Agent durable-commit propagation

Automatic exact-head Rust validation #1382 on head `a9ad236a30ff169ed57f2ba3b1d8f75664c7d6b7` passed the locked dependency graph and rustfmt, then Clippy/compiler diagnostics exposed two existing Agent requester/rendezvous callsites that invoked `ProductionReachabilityOwner::commit_candidate_publication(...)` without awaiting the now-awaitable durable commit.

The required propagation remains inside already-staged dormant Agent authority composition:

- `SharedCurrentCapabilityAuthority` gains `with_current_authority_async(...)`, retaining the same existing Tokio `RwLock` read guard lexically across one supplied async operation;
- `ProductionReachabilityOwnerCustody` / its map gain bounded async closure counterparts retaining exactly one mutable production owner custody across one supplied async operation;
- `SharedRequesterRendezvousAuthority` propagates awaitability through its existing commit/cleanup composition and current-Mesh candidate semantic decomposition;
- the current-authority read and exact owner mutable custody cross only the durable commit await;
- requester cleanup still occurs only after definite durable commit success and after owner/current-authority custody is released.

These additions do not spawn a worker, activate a listener, populate the production owner map, select a provider, send a frame, retry, reconnect, dial, traverse or publish readiness.

## 4. Materialized durable-store API law

The existing `ReachabilityDurableStore` remains the one provider-neutral candidate reachability durable authority.

Its two persistence operations return caller-polled `impl Future + Send` values:

- exact-peer authoritative `load_current`;
- exact-current linearizable `compare_and_commit`.

The trait creates no runtime, task, thread, timer, watcher or socket. GR adds no parallel store trait and no second durable snapshot authority.

## 5. Durable semantic outcomes remain unchanged

Awaitability changes execution shape only.

The existing result law remains authoritative:

- `ReachabilityPersistenceCommit::Committed` means the replacement snapshot is durably current;
- `ReachabilityPersistenceCommit::StaleExpected` is a definite non-commit because durable current freshness no longer matches the expected token;
- `ReachabilityPersistenceError::UnavailableOrAmbiguous` remains ambiguous/unavailable and forces fail-closed recovery in the owner.

GR does not reinterpret cancellation, provider timeout or reconciliation because no concrete provider/runtime behavior is selected.

## 6. Owner awaitability law

Only existing `ProductionReachabilityOwner` methods that call durable storage become awaitable:

1. `recover` awaits authoritative load;
2. `commit_candidate_publication` awaits exactly one durable compare-and-commit after existing pre-commit checks/staging;
3. `retire_noncurrent_lifecycle` awaits exactly one durable compare-and-commit after the existing exact transport-currentness gate;
4. `reload_from_store` awaits authoritative load.

The following remain synchronous:

- `mode`;
- `plan`;
- `freshness`;
- `has_current_traversal`;
- `selected_path`;
- `provision_current_traversal`;
- `poll_and_apply_current_reachability`;
- internal exact-current checks and recovery-state helpers.

GR does not make pure or Sans-I/O operations async without durable-I/O evidence.

## 7. Same-owner authority across await

One durable candidate attempt retains exclusive `&mut ProductionReachabilityOwner` authority across its awaited durable operation.

The source sequence remains:

1. require current owner;
2. validate authenticated publication against the exact current owner plan;
3. require exact presented freshness;
4. stage replacement candidate plan;
5. issue one verifier-owned replacement freshness token;
6. build one complete replacement durable snapshot;
7. await one exact-current durable compare-and-commit;
8. on definite `Committed`, install staged plan/freshness and invalidate prior traversal;
9. on definite stale or ambiguous persistence, invalidate traversal and enter recovery exactly as before.

GR does not release mutable owner authority and later apply a durable result to another owner instance.

## 8. Candidate semantic composition law

Bridge candidate execution becomes awaitable only because the existing production-owner commit is awaitable.

The authority ordering remains:

1. derive publisher identity from authenticated session/connection;
2. construct authenticated candidate publication;
3. authorize requester/rendezvous authority exactly once;
4. require exact expected publisher equality;
5. await exactly one production-owner candidate commit.

PRWC/current-Mesh request correlation remains correlation only and is not device identity, owner identity or persistence key.

No terminal response composition/send, retry, reconnect, second request read or candidate ingress loop is activated here.

## 9. Agent authority and owner-custody awaitability law

`ProductionReachabilityOwnerCustody::recover(...)` becomes awaitable because authoritative owner recovery is awaitable.

Existing synchronous bounded operations remain for non-durable work:

- `with_owner_mut`;
- `ProductionReachabilityOwnerCustodyMap::try_new`;
- `with_owner_mut_for_peer`.

Compiler-required counterparts add only bounded lexical awaitability:

- `with_owner_mut_async`;
- `with_owner_mut_for_peer_async`;
- `SharedCurrentCapabilityAuthority::with_current_authority_async`.

The async current-authority operation holds the already-existing shared-current `RwLock` read guard only for the supplied awaited operation. The async owner-custody operation holds only the exact selected mutable owner borrow for the supplied awaited operation. Neither seam exposes its guard/owner or creates a runtime/task/channel/new synchronization primitive.

The dormant requester/rendezvous candidate composition uses these seams so one fresh current-authority read and one exact peer-keyed production owner remain authoritative across only the durable commit await. They are released before post-commit requester cleanup.

GR does not populate the owner map or activate the dormant candidate entrypoint.

## 10. Authenticated freshness resynchronization law

`authenticated_current_token_resynchronization(...)` becomes awaitable only because authoritative durable `load_current` is awaitable.

The exact ordering remains:

1. validate authenticated publisher session;
2. validate exact current `TransportIdentity` binding for that authenticated `DeviceId`;
3. construct exact `PeerConnectivityIdentity` from logical device identity plus current transport identity;
4. await exactly one authoritative durable load for that peer;
5. reject missing/mismatched/recovery-required/retired durable state exactly as before;
6. re-deliver the exact authoritative current freshness token without generation or commit.

Resynchronization performs no compare-and-commit, freshness advance, rebaseline, candidate mutation, traversal mutation or runtime/network I/O.

## 11. Runtime-independent focused-test law

Bridge-focused test durable stores compute deterministic in-memory results and return ready futures where appropriate. Known-ready futures are resolved with a safe single-poll helper using `Waker::noop()`, `Context`, `Poll` and `std::pin::pin!`.

The helper:

- performs no `block_on`;
- owns no async runtime;
- spawns no thread/task;
- performs no busy wait/polling loop;
- fails if a supposedly ready test future returns `Pending`.

The freshness resynchronization test retains a fail-fast compare-and-commit implementation solely to prove that resynchronization never invokes mutation. Its explicit future form is allowed because the test store is intentionally `!Send` while the returned operation future itself must satisfy the production `Send` contract without capturing an `Rc`-backed mutable store borrow.

Existing Agent tests may use their already-present Tokio test/runtime infrastructure; GR adds no production runtime ownership through those tests.

## 12. No hidden sync-to-async bridge

GR contains no new:

- `Runtime::block_on` in production paths;
- `Handle::block_on`;
- `block_in_place`;
- private bridge Tokio runtime;
- helper thread whose purpose is to make async durable I/O look synchronous;
- channel round-trip to an async persistence task;
- busy wait;
- process-global executor assumption.

Durable futures are returned/awaited honestly through the existing provider-neutral call chain.

## 13. No provider or schema selection

GR does not select or materialize:

- etcd as candidate durable storage;
- Spanner as candidate durable storage;
- embedded database storage;
- key encoding;
- durable snapshot serialization/versioning;
- transaction reconciliation;
- timeout/cancellation semantics;
- credentials/TLS/RBAC;
- retention/compaction policy;
- bootstrap record creation.

The existing C02f live-owner etcd authority remains a distinct authority domain and is not aliased into candidate reachability durability.

## 14. Freshness-token source remains separate

`CandidatePublicationFreshnessTokenSource` remains synchronous because token generation is not the external durable-I/O problem selected by GQ.

GR does not construct a production freshness-token source and does not reuse a different random identifier type as candidate freshness authority.

The production freshness-token prerequisite remains future work.

## 15. Dynamic-network invariant

GR preserves the project-wide dynamic-network invariant.

Durable snapshots and production owner authority continue to use exact logical/current peer identity, including `DeviceId + TransportIdentity`, rather than IP address, socket address, port, DNS answer, relay endpoint, candidate endpoint or request correlation.

Canonical composition remains:

`logical device/session identity -> registry/discovery -> current reachable endpoint/candidates -> authenticated transport`.

Not:

`device identity = static IP`.

## 16. Candidate source blobs after nine-path propagation

The eight source/test paths immediately before this contract correction are:

- `crates/prw-remote-bridge/src/reachability_owner.rs`
  - candidate blob `fb7543361ea3a144ae9275284b41bf0ef63df2ad`
- `crates/prw-remote-bridge/src/candidate_publication_execution.rs`
  - candidate blob `eaca3c8b61ed5c7d3f8ca92b19c81fa062bd7a6f`
- `crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs`
  - candidate blob `648ca46c791f7f80d1d8571a3da5dde3f761047e`
- `crates/prw-agent/src/production_reachability_owner_custody.rs`
  - candidate blob `4b3d51fd4019074513632d0b4c1a7e58c24048ef`
- `crates/prw-remote-bridge/src/reachability_freshness_wire.rs`
  - candidate blob `05873b0a86ef761155be65be27fb15b6d7d3f7fd`
- `crates/prw-remote-bridge/tests/reachability_freshness_wire.rs`
  - candidate blob `6bcc8695fffec073676d030cb50b69c4334ff50b`
- `crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`
  - candidate blob `60307fff4dd0fd573192ba6e6fab9dedd3321dda`
- `crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs`
  - candidate blob `d550ec8d5aa18ed6885ebed42c52ee742498e9d2`

The contract's own final blob is fixed only after this update. Any later source/contract correction supersedes all candidate blob and CI evidence for closure purposes.

## 17. GR authorized changed-path ceiling

GR authorizes exactly nine paths:

1. `crates/prw-remote-bridge/src/reachability_owner.rs`
2. `crates/prw-remote-bridge/src/candidate_publication_execution.rs`
3. `crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs`
4. `crates/prw-agent/src/production_reachability_owner_custody.rs`
5. `crates/prw-remote-bridge/src/reachability_freshness_wire.rs`
6. `crates/prw-remote-bridge/tests/reachability_freshness_wire.rs`
7. `crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`
8. `crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs`
9. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_GR_PRODUCTION_REACHABILITY_AWAITABLE_DURABLE_STORE_OWNER_CANDIDATE_EXECUTION_SOURCE_MATERIALIZATION_STAGING.md`

The original five-path ceiling was superseded by #1379 freshness propagation. The seven-path ceiling was superseded by #1382 Agent durable-commit propagation. Both expansions are compiler-discovered consequences of the already-selected awaitable durable-store execution shape; they do not authorize unrelated runtime integration.

No Cargo manifest, lockfile, workflow, control-plane provider, Linux bootstrap/main, current-Mesh transport, Android/Kotlin/Gradle, deployment/configuration, unrelated contract or other source path is authorized.

## 18. Explicit non-activation boundary

GR does not:

- instantiate a concrete durable provider;
- execute production durable recovery at process startup;
- construct production owner custodies in Linux bootstrap;
- populate `ProductionReachabilityOwnerCustodyMap`;
- add owner-map synchronization;
- remove `CandidatePublicationHandoffNotSelected`;
- invoke candidate semantics from production current-Mesh ingress;
- compose/send candidate terminal responses from Agent ingress;
- activate a worker, listener, readiness signal, dialing, traversal networking or reconnect loop;
- deploy or restart a service;
- merge or delete a branch;
- change repository visibility.

The two Agent authority files are touched only to propagate lexical awaitability through already-dormant candidate semantic composition. No peer-visible runtime behavior is activated.

## 19. Validation and correction law

Only the exact final GR head may provide closure evidence. Automatic CI may run because the GR PR is draft/open. No workflow is manually dispatched.

Superseded validation/correction history includes:

- initial candidate `b1f82411beaf4965f96ef04c667f985d740d62ff`: Rust #1377 passed locked graph and failed rustfmt only;
- formatter-only corrections produced `de926bb85e3242fa6f0ea28c6d01a3b4fe48e9e8`;
- Rust #1379 on `de926bb85e3242fa6f0ea28c6d01a3b4fe48e9e8`: locked graph + rustfmt PASS, Clippy exposed unawaited freshness durable load; tests/build skipped;
- source corrections `82b5e9ca20e240102350d54ce814d10d66cfff2b` and `eb00fe2f68210b7f5eca195823c88ba9d2464705` propagated freshness awaitability;
- contract correction `a9ad236a30ff169ed57f2ba3b1d8f75664c7d6b7` recorded the temporary seven-path boundary;
- Rust #1382 on `a9ad236a30ff169ed57f2ba3b1d8f75664c7d6b7`: locked graph + rustfmt PASS, then Clippy/compiler diagnostics exposed manual test wakers, a test-only explicit-future lint, and two unawaited Agent durable-commit callsites;
- Agent propagation then added bounded async current-authority and exact-owner-custody closure seams and propagated requester/rendezvous commit composition; the key requester propagation commit is `6e93f3ec29b691a8d3af8b62d41206ac4b56c411`;
- Rust #1385 on `6e93f3ec29b691a8d3af8b62d41206ac4b56c411` failed rustfmt only on one Agent custody test hunk;
- commits `6e00fa3a1f91a7a97e821050b6b17520bd52f019`, `9086d1e9324820ca0c626fdc95e412668f80b3b0`, and `29dfca12c1e301a248adaa0798a8d68735934f03` normalized compiler/Clippy-discovered test helper issues without widening the nine-path set;
- Rust #1387 on `9086d1e9324820ca0c626fdc95e412668f80b3b0` and Rust #1388 on `29dfca12c1e301a248adaa0798a8d68735934f03` both failed only at the same previously known Agent custody rustfmt hunk;
- `bcdbaeda24e7ec82ce2905e6ec51e7e1269b047d` applies only that formatter-required Agent custody layout correction.

All Android PASS results attached to superseded heads are superseded as closure evidence. AD/AE skipped runs remain recorded only as skipped.

If this contract or any later correction changes GR head:

- every validation result for every superseded head is non-canonical;
- final path/blob/compare evidence must be recomputed;
- closure uses only workflows tied to the exact corrected head.

## 20. Closure law

GR may become canonically CLOSED only when all of the following hold:

1. exact predecessor remains GQ final head `4065c9e75a6e711914f0f6f042f544a788f5eefa`;
2. GQ -> exact final GR compare is ahead-only with behind 0;
3. merge base is exact GQ final head;
4. changed-path set is exactly the nine authorized paths;
5. final exact tree and all nine final blobs are recorded;
6. exact-final-head required automatic Rust and Android validation reaches terminal success;
7. no superseded validation is reused;
8. immutable GR audit is created from exact final source/CI state;
9. local audit bytes and SHA-256 are recorded;
10. duplicate guard is clean in canonical `Private Remote Workspace` Drive folder;
11. audit is uploaded directly to that canonical folder;
12. exact uploaded object is raw-fetched;
13. raw readback bytes/SHA equal local bytes/SHA exactly;
14. PR and branch are re-read immediately before closure metadata mutation;
15. PR body is updated to `Status: CLOSED` while PR remains draft/open/unmerged;
16. PR and branch are independently re-read after closure.

## 21. Successor rule

Canonical GR closure materializes execution shape only; it does not make GP owner-map population production-ready by itself.

After closure, perform a fresh exact-final-head audit before choosing the next checkpoint.

The expected remaining prerequisites include a concrete candidate-reachability durable-store adapter/backing and a concrete production `CandidatePublicationFreshnessTokenSource`, but ordering is not pre-authorized and must follow fresh exact-GR source evidence.

Agent map population and candidate-publication handoff remain blocked until production recovery inputs are concretely available and validated.
