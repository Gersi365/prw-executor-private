# Phase 152 — C03e-GR Production Reachability Awaitable Durable-Store / Owner / Candidate-Execution Source Materialization Staging

Status: SOURCE_MATERIALIZATION_STAGING

Target gate:
`C03E_GR_PRODUCTION_REACHABILITY_AWAITABLE_DURABLE_STORE_OWNER_CANDIDATE_EXECUTION_SOURCE_MATERIALIZED`

## 1. Purpose

C03e-GR materializes only the provider-neutral awaitable durable-I/O source boundary selected by canonically CLOSED C03e-GQ.

GR evolves the existing `ReachabilityDurableStore` authority, the durable-I/O methods on the existing `ProductionReachabilityOwner`, the existing candidate-publication semantic composition, the authenticated freshness resynchronization durable reload, and the existing Agent production-owner recovery wrapper so an eventual asynchronous persistence provider can be polled by a caller-owned executor without hiding blocking inside a synchronous durable-store API.

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

The fresh exact-GQ audit found the production owner/candidate call graph below. Exact-head compiler validation later exposed one additional existing durable-load callsite in authenticated freshness resynchronization. That compiler finding is part of GR scope evidence rather than being hidden or treated as an unrelated failure.

### 3.1 Existing durable authority

`crates/prw-remote-bridge/src/reachability_owner.rs` at GQ blob
`8d0e65c3fc0bd646c257199d4f55be65fa3f792d` defined synchronous:

- `ReachabilityDurableStore::load_current(...)`;
- `ReachabilityDurableStore::compare_and_commit(...)`.

Those calls were used by exactly the owner operations whose semantics require durable I/O:

- `ProductionReachabilityOwner::recover(...)`;
- `ProductionReachabilityOwner::commit_candidate_publication(...)`;
- `ProductionReachabilityOwner::retire_noncurrent_lifecycle(...)`;
- `ProductionReachabilityOwner::reload_from_store(...)`.

Pure accessors, exact peer identity comparison, candidate staging, traversal construction and observation do not require durable I/O merely because the durable authority becomes awaitable.

### 3.2 Existing candidate composition

`crates/prw-remote-bridge/src/candidate_publication_execution.rs` at GQ blob
`4a0f3a23cdc45e5d076d152e84724e997c789b1b` reached the production owner's durable commit through a synchronous private `CandidatePublicationCommit` seam.

The existing ordering was:

`authenticated publication construction -> requester/rendezvous authority -> exact publisher equality -> production-owner durable candidate commit`.

GR preserves this order and only makes the final durable-dependent path awaitable.

### 3.3 Existing production-owner integration test

`crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs` at GQ blob
`de0772e8f8198b168fcfb47c4b5b66771ab40d22` used a test-only in-memory `ReachabilityDurableStore` and directly exercised recovery, commit, reload and retirement.

This is the focused executable regression surface for preserving C02e durability and fail-closed semantics while changing only I/O execution shape.

### 3.4 Existing Agent custody

`crates/prw-agent/src/production_reachability_owner_custody.rs` at GQ blob
`4d1da871f7451302ead98e16c7d37b006c6d7ded` delegated one authoritative recovery to `ProductionReachabilityOwner::recover(...)`.

The existing `with_owner_mut` and exact-peer custody-map association/lookup are synchronous bounded ownership operations. They do not themselves perform durable I/O and therefore are not made asynchronous by GR.

A later Agent orchestration checkpoint may require a separately selected ownership shape for awaiting a future while retaining mutable owner custody. GR does not select or activate that higher composition.

### 3.5 Repository async precedent

The exact repository already contains `reachability_live_owner_async.rs`, which exposes provider-neutral production async ports as `impl Future + Send` while leaving executor/runtime ownership outside the bridge.

GR follows that established source pattern. It does not add `async-trait`, Tokio or another runtime/dependency.

### 3.6 Compiler-discovered authenticated freshness resynchronization propagation

The original five-path GR candidate omitted one existing `ReachabilityDurableStore::load_current(...)` call in:

`crates/prw-remote-bridge/src/reachability_freshness_wire.rs`.

Automatic exact-head Rust validation #1379 on head `de926bb85e3242fa6f0ea28c6d01a3b4fe48e9e8` passed the locked dependency graph and rustfmt, then Clippy failed because `authenticated_current_token_resynchronization(...)` still called the now-awaitable durable load synchronously. The diagnostic pointed to the `load_current(&peer).map_err(...)` chain and required awaiting the store future before persistence error mapping.

The source correction therefore makes only that authenticated resynchronization function awaitable and awaits exactly the durable `load_current` operation after existing registry/session/transport currentness validation. No registry ordering, peer derivation, freshness lifecycle interpretation, failure taxonomy or token-delivery semantics change.

The corresponding integration test `crates/prw-remote-bridge/tests/reachability_freshness_wire.rs` is also necessarily propagated because it implements the same `ReachabilityDurableStore` trait and directly invokes authenticated resynchronization. Its deterministic in-memory store uses the same std-only ready-future pattern already selected by GR.

Exact source/reference audit found no additional production durable-store callsite requiring GR propagation. The older freshness authority/bootstrap/composition reference tests remain independent test-local models and do not implement this durable-store port.

## 4. Materialized durable-store API law

The existing `ReachabilityDurableStore` remains the one provider-neutral candidate reachability durable authority.

Its two persistence operations now return caller-polled futures:

- exact-peer authoritative `load_current`;
- exact-current linearizable `compare_and_commit`.

The futures are required to be `Send` so a future concrete provider is not forced into a thread-affine store operation merely by the bridge contract.

The trait itself creates no runtime, task, thread, timer, watcher or socket.

GR does not add a parallel `ReachabilityDurableStoreAsync`, a second owner model, or a second durable snapshot authority.

## 5. Durable semantic outcomes remain unchanged

Awaitability changes execution shape only.

The existing result law remains authoritative:

- `ReachabilityPersistenceCommit::Committed` means the replacement snapshot is durably current;
- `ReachabilityPersistenceCommit::StaleExpected` is a definite non-commit because durable current freshness no longer matches the caller's expected token;
- `ReachabilityPersistenceError::UnavailableOrAmbiguous` remains an ambiguous/unavailable durable result and forces fail-closed recovery in the owner.

GR does not reinterpret cancellation, runtime failure, provider timeout or transaction reconciliation because no concrete provider/runtime behavior is selected here.

## 6. Owner awaitability law

Only existing `ProductionReachabilityOwner` methods that call durable storage become awaitable:

1. `recover` awaits authoritative load;
2. `commit_candidate_publication` awaits exactly one durable compare-and-commit after all existing pre-commit checks and staging;
3. `retire_noncurrent_lifecycle` awaits exactly one durable compare-and-commit after the existing exact transport-currentness gate;
4. `reload_from_store` awaits authoritative load.

These methods continue to operate on the same owner instance and preserve the same local state transition law after the awaited durable result.

The following remain synchronous:

- `mode`;
- `plan`;
- `freshness`;
- `has_current_traversal`;
- `selected_path`;
- `provision_current_traversal`;
- `poll_and_apply_current_reachability`;
- internal exact-current token checks and recovery-state transition helpers.

GR does not make pure or Sans-I/O operations asynchronous without durable-I/O evidence.

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

`execute_authenticated_candidate_publication(...)` and the private candidate commit composition are now awaitable only because the existing production-owner commit is awaitable.

The existing authority ordering remains unchanged:

1. derive publisher identity from authenticated session/connection;
2. construct authenticated candidate publication;
3. authorize current requester/rendezvous authority exactly once;
4. require exact expected publisher equality;
5. await exactly one production-owner candidate commit.

PRWC/current-Mesh request correlation remains correlation only and is not used as device identity, owner identity or persistence key.

No frame I/O, terminal response composition, same-stream send, retry, reconnect, second request read or candidate ingress loop is added here.

## 9. Agent recovery custody law

`ProductionReachabilityOwnerCustody::recover(...)` becomes awaitable because its sole authoritative construction operation now awaits durable owner recovery.

It still:

- constructs exactly one custody from exactly one successful `ProductionReachabilityOwner::recover` result;
- preserves missing, ambiguous, mismatched, recovery-required and retired classifications from the owner;
- creates no default/rebaseline owner;
- does not clone or expose raw owner/store/token-source custody.

`with_owner_mut`, `ProductionReachabilityOwnerCustodyMap::try_new`, and exact-peer lookup remain synchronous.

GR does not invent an async closure/guard/map/mutex/actor/channel seam for later candidate orchestration.

## 10. Authenticated freshness resynchronization law

`authenticated_current_token_resynchronization(...)` becomes awaitable only because authoritative durable `load_current` is awaitable.

The exact ordering remains:

1. validate authenticated publisher session;
2. validate exact current `TransportIdentity` binding for that authenticated `DeviceId`;
3. construct exact `PeerConnectivityIdentity` from logical device identity plus current transport identity;
4. await exactly one authoritative durable load for that peer;
5. reject missing/mismatched/recovery-required/retired durable state exactly as before;
6. re-deliver the exact authoritative current freshness token without generation or commit.

Resynchronization still performs no compare-and-commit, freshness advance, rebaseline, candidate mutation, traversal mutation or runtime/network I/O.

## 11. Runtime-independent test law

GR updates only existing focused tests required by the API-shape change.

Test durable-store implementations compute deterministic in-memory results synchronously and return `std::future::ready(...)` where the operation can complete normally. Focused tests resolve only known-ready futures with a safe single-poll helper built from `std::task::Wake`, `Waker`, `Context` and `std::pin::pin!`.

The helper:

- performs no `block_on`;
- owns no async runtime;
- spawns no thread/task;
- performs no busy wait or polling loop;
- fails if a supposedly ready test future returns `Pending`.

The freshness resynchronization test retains a fail-fast compare-and-commit implementation solely to prove that the resynchronization path never invokes mutation.

This test mechanism is not a production executor or provider adapter.

## 12. No hidden sync-to-async bridge

GR contains no:

- `Runtime::block_on`;
- `Handle::block_on`;
- `block_in_place`;
- private Tokio runtime;
- helper thread whose purpose is to make async I/O look synchronous;
- channel round-trip to an async persistence task;
- busy wait;
- process-global executor assumption.

The durable futures are returned/awaited honestly through the existing provider-neutral call chain.

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

The separate production freshness-token prerequisite remains future work.

## 15. Dynamic-network invariant

GR preserves the project-wide dynamic-network invariant.

Durable snapshots and production owner authority continue to use exact logical/current peer identity, including `DeviceId + TransportIdentity`, rather than IP address, socket address, port, DNS answer, relay endpoint, candidate endpoint or request correlation.

Canonical composition remains:

`logical device/session identity -> registry/discovery -> current reachable endpoint/candidates -> authenticated transport`.

Not:

`device identity = static IP`.

## 16. Candidate source blobs after seven-path propagation

The six source/test paths materialized before this contract correction are:

- `crates/prw-remote-bridge/src/reachability_owner.rs`
  - candidate blob `fb7543361ea3a144ae9275284b41bf0ef63df2ad`
- `crates/prw-remote-bridge/src/candidate_publication_execution.rs`
  - candidate blob `584f5fd17bfcc68c822c104893977d501e284c4f`
- `crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs`
  - candidate blob `f12c22f1c97850b359630fe0964600fd9f539336`
- `crates/prw-agent/src/production_reachability_owner_custody.rs`
  - candidate blob `5b42a29157b0efccc5bfeba15048a3b348599a6e`
- `crates/prw-remote-bridge/src/reachability_freshness_wire.rs`
  - candidate blob `05873b0a86ef761155be65be27fb15b6d7d3f7fd`
- `crates/prw-remote-bridge/tests/reachability_freshness_wire.rs`
  - candidate blob `155dbe1a97a2f19d65087ee8b7db824a5c43a272`

The contract's own final blob is fixed only after this update. If later exact-head validation requires another correction, all candidate blobs and all prior validation are superseded for canonical closure purposes.

## 17. GR authorized changed-path ceiling

GR authorizes exactly seven paths:

1. `crates/prw-remote-bridge/src/reachability_owner.rs`
2. `crates/prw-remote-bridge/src/candidate_publication_execution.rs`
3. `crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs`
4. `crates/prw-agent/src/production_reachability_owner_custody.rs`
5. `crates/prw-remote-bridge/src/reachability_freshness_wire.rs`
6. `crates/prw-remote-bridge/tests/reachability_freshness_wire.rs`
7. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_GR_PRODUCTION_REACHABILITY_AWAITABLE_DURABLE_STORE_OWNER_CANDIDATE_EXECUTION_SOURCE_MATERIALIZATION_STAGING.md`

The original five-path candidate ceiling is explicitly superseded because exact-head Clippy proved the existing freshness durable-load callsite had not been propagated.

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
- activate a worker, task, listener, readiness signal, dialing, traversal networking or reconnect loop;
- deploy or restart a service;
- merge or delete a branch;
- change repository visibility.

No peer-visible runtime behavior is activated by this checkpoint.

## 19. Validation and correction law

Only the exact final GR head may provide closure evidence.

Automatic CI may run because the GR PR is draft/open. No workflow is manually dispatched.

Superseded validation history includes:

- initial candidate `b1f82411beaf4965f96ef04c667f985d740d62ff`: Rust #1377 passed locked graph and failed rustfmt only;
- two formatter-only corrections produced later candidate `de926bb85e3242fa6f0ea28c6d01a3b4fe48e9e8`;
- exact head `de926bb85e3242fa6f0ea28c6d01a3b4fe48e9e8`: Rust #1379 passed locked graph and rustfmt, then Clippy exposed the unawaited freshness durable-load callsite;
- Android #1192 associated with that superseded candidate is not closure evidence;
- AD #631 and AE #622 on that superseded candidate were skipped and are recorded only as skipped.

The compiler-discovered scope correction was then materialized as:

- `82b5e9ca20e240102350d54ce814d10d66cfff2b` — await exact freshness durable reload;
- `eb00fe2f68210b7f5eca195823c88ba9d2464705` — adapt focused freshness tests to the awaitable store/resynchronization surface.

If this contract or any later correction changes the GR head:

- every validation result for every superseded head is non-canonical;
- final path/blob/compare evidence must be recomputed;
- final closure uses only workflows tied to the exact corrected head.

Skipped path-filtered workflows are recorded as skipped, never promoted to PASS.

## 20. Closure law

GR may become canonically CLOSED only when all of the following hold:

1. exact predecessor remains GQ final head `4065c9e75a6e711914f0f6f042f544a788f5eefa`;
2. GQ -> exact final GR compare is ahead-only with behind 0;
3. merge base is exact GQ final head;
4. changed-path set is exactly the seven authorized paths;
5. final exact tree and all seven final blobs are recorded;
6. exact-final-head required automatic CI reaches terminal acceptable conclusions;
7. no superseded validation is reused;
8. immutable GR audit is created from exact final source/CI state;
9. local audit bytes and SHA-256 are recorded;
10. duplicate guard is clean in the canonical `Private Remote Workspace` Drive folder;
11. audit is uploaded directly to the canonical folder;
12. exact uploaded object is raw-fetched;
13. raw readback bytes/SHA equal local bytes/SHA exactly;
14. PR and branch are re-read immediately before closure metadata mutation;
15. PR body is updated to `Status: CLOSED` while PR remains draft/open/unmerged;
16. PR and branch are independently re-read after closure.

## 21. Successor rule

Canonical GR closure will materialize execution shape only; it will not make GP owner-map population production-ready by itself.

After closure, perform a fresh exact-final-head audit before choosing the next checkpoint.

The expected next boundary is selection/materialization of a concrete candidate-reachability durable-store adapter/backing, including exact key/schema/CAS/outcome/recovery/retirement semantics, unless fresh source evidence exposes a narrower prerequisite.

The concrete production freshness-token source remains separately gated and may precede or follow provider materialization only after fresh source evidence.

Agent map population and candidate-publication handoff remain blocked until their production recovery inputs are concretely available and validated.
