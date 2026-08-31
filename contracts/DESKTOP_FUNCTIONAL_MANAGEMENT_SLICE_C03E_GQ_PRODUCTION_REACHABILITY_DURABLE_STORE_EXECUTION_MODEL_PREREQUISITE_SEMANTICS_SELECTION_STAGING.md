# Phase 152 — C03e-GQ Production Reachability Durable-Store Execution-Model Prerequisite Semantics Selection Staging

Status: SEMANTICS_SELECTION_STAGING

Target gate:
`C03E_GQ_PRODUCTION_REACHABILITY_DURABLE_STORE_EXECUTION_MODEL_PREREQUISITE_SELECTED`

## 1. Purpose

C03e-GQ selects the narrow production execution-model prerequisite exposed by a fresh exact-C03e-GP source audit before the GP-selected startup/recovery `ProductionReachabilityOwnerCustodyMap` population can be materialized.

The audit shows that the already-selected production reachability owner is parameterized by a provider-neutral durable-store seam, but the current seam is synchronous while the repository's concrete distributed persistence precedents are asynchronous. Current Agent source contains only generic production-owner custody plus test-only store/token implementations and does not construct a production durable store.

GQ therefore selects one provider-neutral law: production durable reachability load/CAS operations that can perform network/database I/O must become explicitly awaitable through the existing owner/execution composition before any asynchronous production provider may be wired. Hidden sync-to-async blocking inside the current synchronous `ReachabilityDurableStore` contract is forbidden.

GQ does not select a persistence product, schema, serializer, concrete adapter, freshness-token source, Agent map population, candidate-publication handoff, network runtime, deployment or merge.

## 2. Exact predecessor

The exact predecessor is canonically CLOSED C03e-GP.

- GP branch: `phase-152-c03e-gp-production-reachability-owner-custody-runtime-population-synchronization-semantics-selection-staging`
- GP final head: `55fde94b8b9ca66c22501ea2b4169013f7071047`
- GP final tree: `ad643aa42d7ff2cb38e8b5657a36eab83c0bc8e4`
- GP contract blob: `65a747ed372c9f56e9547792b86914006af79a4f`

GQ begins exactly from the GP final head and does not amend GP.

## 3. Why GP's expected population successor is not yet source-ready

GP selected the population law:

`already-recovered production owner custodies -> validate exact peer uniqueness -> construct complete owner-custody map -> expose complete map`

and explicitly required a fresh exact-final-head audit before source materialization. GP's successor rule permits a different prerequisite when fresh source evidence exposes one.

The exact GP audit exposes that prerequisite:

1. `ProductionReachabilityOwner<S, T>` requires concrete `S: ReachabilityDurableStore` and `T: CandidatePublicationFreshnessTokenSource` inputs;
2. Agent `ProductionReachabilityOwnerCustody<S, T>` recovery requires those same concrete inputs;
3. current Agent production source does not supply a production `S` or `T`;
4. the only concrete `ReachabilityDurableStore` and `CandidatePublicationFreshnessTokenSource` implementations in the Agent custody module are `#[cfg(test)]` fixtures;
5. Linux bootstrap does not construct `ProductionReachabilityOwner`, `ProductionReachabilityOwnerCustody`, or `ProductionReachabilityOwnerCustodyMap`;
6. therefore GP population cannot yet consume an actual production recovered-custody snapshot.

GQ addresses only the first execution-model prerequisite inside that missing recovery-input chain. It does not collapse all remaining recovery-input work into one checkpoint.

## 4. Existing C02e durable-state law remains authoritative

GQ does not redefine candidate-publication durable-state semantics.

C02e Tranche 3 already selected:

- exact 32-byte opaque verifier freshness tokens;
- durable state keyed by exact `PeerConnectivityIdentity` (`DeviceId + TransportIdentity`);
- `NewLifecycleEligible`, `Established`, `RecoveryRequired`, and `Retired` lifecycle dispositions;
- no missing-row bootstrap authority;
- linearizable accepted publication state transition;
- fail-closed restart/failover when exact authoritative freshness cannot be recovered;
- no automatic same-identity rebaseline;
- retained historical retirement semantics sufficient to prevent stale identity reuse from creating a new replay namespace.

C02e Tranche 4 already selected:

- `ProductionReachabilityOwner` as the production upper reachability owner;
- `ReachabilityDurableStore` as the provider-neutral durable arbitration seam;
- `compare_and_commit(expected_current, replacement)` as the exact-current linearizable CAS semantic boundary;
- `Committed`, `StaleExpected`, and ambiguous/unavailable persistence outcomes with their existing fail-closed meanings;
- authoritative reload as the only recovery path after stale/ambiguous persistence;
- no concrete persistence product, schema, serialization or replication selection.

GQ changes none of those semantic outcomes.

## 5. Fresh exact-GP source evidence

### 5.1 Current durable store seam is synchronous

At exact GP head, `crates/prw-remote-bridge/src/reachability_owner.rs` defines synchronous:

- `ReachabilityDurableStore::load_current(...)`;
- `ReachabilityDurableStore::compare_and_commit(...)`.

`ProductionReachabilityOwner::recover(...)`, accepted candidate commit, authoritative reload and retirement call that seam synchronously.

The same module states that concrete database, serialization, replication and transaction implementation remain outside the tranche and that the module performs no Agent bootstrap activation.

There is no concrete `impl ReachabilityDurableStore for ...` in that exact source module.

### 5.2 Candidate semantic execution is synchronous at the durable commit boundary

At exact GP head, `crates/prw-remote-bridge/src/candidate_publication_execution.rs` exposes synchronous candidate-publication semantic execution.

The execution path reaches `ProductionReachabilityOwner::commit_candidate_publication(...)` synchronously after authenticated publication construction and requester/rendezvous authorization.

Therefore a future network-backed durable implementation hidden under the current trait would perform blocking durable I/O on whichever caller thread executes candidate semantics unless a separate execution mechanism were invented.

### 5.3 Current Agent custody has test-only recovery inputs

At exact GP head, `crates/prw-agent/src/production_reachability_owner_custody.rs` is generic over `S` and `T` and recovers by calling the existing production owner.

Its concrete store/token-source implementations are test-only fixtures under `#[cfg(test)]`.

The production module remains crate-internal and staged; it is not a proof that Linux runtime constructs a concrete recovery input pair.

### 5.4 Existing production reachability persistence precedent is asynchronous and semantically distinct

`crates/prw-control-plane/src/reachability_live_owner_etcd.rs` contains a real production `ReachabilityLiveOwnerEtcdStore` using `etcd-client` and asynchronous etcd operations.

That store is authority for C02f live-owner currentness/fencing. It is not the C02e/C03e candidate reachability durable snapshot store and must not be substituted for `ReachabilityDurableStore` merely because both concern reachability.

Its value as precedent is narrower:

- etcd is an existing production distributed persistence family in this repository;
- current etcd operations are asynchronous;
- exact-current/CAS style transactions already exist in source;
- provider-specific I/O is owned outside the provider-neutral reachability owner.

GQ does not select etcd as the candidate reachability durable backend.

### 5.5 No existing synchronous embedded durable backend is established in the exact workspace graph

The exact GP `Cargo.lock` includes `etcd-client` and the existing cloud/provider graph, but no current package named `rusqlite`, `rocksdb`, `redb`, or `sled`.

This is not a universal claim that no synchronous persistence technology exists. It is a source-topology finding that the current repository does not provide a ready, already-authorized synchronous durable backend that can be silently substituted into `ReachabilityDurableStore`.

## 6. Selected execution-model law

Production durable reachability operations that may perform external persistence I/O must be explicitly awaitable at the provider-neutral owner boundary.

The selected semantic direction is:

`async production persistence adapter -> awaitable provider-neutral durable-store operation -> awaitable ProductionReachabilityOwner durable operation -> awaitable candidate semantic composition`

not:

`async provider -> hidden block_on/blocking wait inside synchronous ReachabilityDurableStore -> synchronous owner/candidate execution`.

This selection is about I/O execution ownership, not about changing the durable CAS meaning.

## 7. Existing durable-store authority must evolve; no parallel owner model

A later source-materialization checkpoint should evolve the existing `ReachabilityDurableStore` / `ProductionReachabilityOwner` durable-I/O path into an awaitable form rather than introducing a second production reachability owner or a second durable-state authority.

Equivalent Rust source shapes may be considered during exact-head materialization, including native async trait methods or explicit typed future-returning methods, provided the resulting source preserves the laws in this contract.

GQ does not require a particular macro crate or syntax.

The materialization must not create:

- `ProductionReachabilityOwnerAsync` as a second semantic owner;
- a second `ReachabilityDurableSnapshot` model;
- a second freshness lifecycle;
- a shadow in-memory accepted-state authority;
- a provider-specific candidate-execution implementation that bypasses the existing owner.

## 8. Operations that become awaitable

Only operations whose existing semantic path depends on durable-store I/O need to become awaitable.

The expected affected semantic boundaries are:

- initial `ProductionReachabilityOwner::recover(...)` authoritative load;
- `reload_from_store()` authoritative reload;
- accepted candidate-publication `compare_and_commit(...)` path;
- durable retirement `compare_and_commit(...)` path;
- higher candidate-publication execution composition that calls the durable commit.

Pure value construction, identity comparison, exact peer lookup, staged candidate validation, accessor methods and other non-I/O operations must not become background tasks merely because the durable seam becomes awaitable.

A source-materialization audit must determine the exact call graph and changed-path ceiling before mutation.

## 9. No hidden blocking bridge

A conforming future production implementation must not hide asynchronous persistence by calling or inventing, inside the durable-store/owner semantic path:

- `Runtime::block_on(...)`;
- `Handle::block_on(...)`;
- `block_in_place(...)` as an implicit provider adapter;
- a synchronous channel round-trip to an async persistence task while holding candidate owner authority;
- a per-call private Tokio runtime;
- a per-call helper thread whose only purpose is to make async persistence appear synchronous;
- busy-waiting or polling loops;
- process-global executor assumptions not represented in the API.

Any later runtime scheduling/offload mechanism must be separately justified by exact source evidence. GQ selects explicit awaitability instead of hidden blocking.

## 10. Caller-owned runtime law

Making the durable path awaitable does not make `ProductionReachabilityOwner` a runtime owner.

The owner must not create or retain:

- a Tokio runtime;
- executor handle;
- background worker;
- task set;
- listener;
- socket;
- watch stream;
- retry timer;
- cancellation supervisor.

The later Agent/current-Mesh orchestration layer that already owns asynchronous execution remains responsible for polling/awaiting the returned durable operation.

GQ does not select that higher orchestration activation.

## 11. Borrow/authority law across await

Awaitability must not weaken authority semantics.

A future source materialization must preserve that one candidate commit operates against exactly one mutable `ProductionReachabilityOwner` for the exact peer lifecycle. It must not release owner authority midway and then apply a stale durable result to a different local owner instance.

The exact Rust borrow shape is a source concern, but semantic requirements remain:

1. all pre-commit identity/freshness/candidate validation uses the exact owner state for that attempt;
2. one replacement durable snapshot is staged;
3. one authoritative durable compare-and-commit is awaited;
4. only a definite `Committed` result installs staged local state;
5. stale or ambiguous persistence transitions the same local owner fail-closed as already selected;
6. no second candidate operation may interleave against the same mutable owner through an unauthorized alias.

GQ does not select an `Arc`, mutex, actor or channel.

## 12. Error law remains unchanged

Async execution must not flatten or reinterpret existing persistence/owner outcomes.

The future awaitable seam must preserve distinctions including:

- definite `StaleExpected`;
- persistence unavailable/error where commit status cannot be safely inferred;
- `DurableStateMissing` on authoritative recovery;
- snapshot peer mismatch;
- `RecoveryRequired`;
- `Retired` lifecycle restrictions.

A transport/runtime cancellation or task failure must not be silently translated into `Committed` or a candidate semantic `Rejected` result.

GQ does not select cancellation semantics for an in-flight durable transaction; that requires provider/runtime evidence once a concrete adapter is selected.

## 13. Persistence product remains unselected

GQ deliberately does not choose between etcd, Spanner, another distributed durable service, a future synchronous backend, or another provider.

A later provider-selection checkpoint must prove at least:

- exact-key representation for `PeerConnectivityIdentity`;
- authoritative durable snapshot serialization/versioning;
- exact-current freshness comparison semantics;
- atomic commit of the full replacement snapshot required by C02e;
- unambiguous mapping of provider outcomes to `Committed`, `StaleExpected`, and ambiguous persistence failure;
- recovery/load semantics;
- retirement/tombstone retention semantics;
- credential/configuration custody;
- runtime cancellation/timeout behavior;
- no collision with the distinct live-owner authority keyspace/semantics.

GQ does not pre-authorize any of those source changes.

## 14. Live-owner etcd authority remains distinct

The existing `ReachabilityLiveOwnerEtcdStore` must not be type-aliased, wrapped or renamed into candidate durable snapshot authority without a separately selected adapter/schema contract.

The two authorities differ:

- live-owner authority controls current mutable runtime tenancy/fencing;
- candidate reachability durable state controls accepted plan + freshness lifecycle recovery and replay safety.

They may eventually use the same provider family, but sharing a provider does not make their records, keys, transactions, credentials, retention or authorization semantics interchangeable.

## 15. Freshness-token generation remains a separate prerequisite

`CandidatePublicationFreshnessTokenSource` remains a distinct production recovery/execution input.

C02e already requires:

- exactly 32 random bytes;
- non-zero token;
- verifier-owned generation;
- cryptographically secure entropy;
- replacement distinct from the expected/current token.

C03e-CE/CF provide a repository precedent for OS-backed `aws_lc_rs::rand::SystemRandom` in another verifier-owned 256-bit random source, but that SessionId source is not candidate freshness authority and must not be reused by type substitution.

GQ does not select or materialize the concrete production freshness-token source. A later checkpoint may use the established cryptographic-provider precedent only after a fresh exact-head ownership/dependency audit.

## 16. GP population remains blocked

GP's startup/recovery map population law remains authoritative but not yet source-materializable as a production runtime path.

The ordering after GQ is:

1. close the provider-neutral durable-store execution-model prerequisite selected here;
2. materialize and validate that awaitable durable-I/O seam without choosing a provider unless separately authorized;
3. select/materialize a concrete production durable-store adapter/backing;
4. select/materialize the concrete production freshness-token source if still absent;
5. construct/recover exact production owner custodies from authoritative durable state;
6. only then materialize GP's complete startup/recovery custody-map population;
7. only after population is authoritative may a later checkpoint consider Agent candidate-publication handoff orchestration.

A fresh exact-head audit after every closed checkpoint may reorder steps only when concrete source evidence justifies it.

## 17. Candidate-publication ingress remains dormant

GQ does not remove or weaken the existing Agent `CandidatePublicationHandoffNotSelected` barrier.

GQ does not invoke:

- candidate semantic execution from current-Mesh ingress;
- requester/rendezvous authorization from candidate ingress;
- reachability durable mutation;
- GM terminal frame composition;
- GO same-stream response send;
- repeated candidate ingress loop behavior.

No peer-visible behavior changes.

## 18. No map population or recovery activation

GQ does not:

- create a `ProductionReachabilityOwnerCustodyMap` in Linux bootstrap;
- construct concrete `ProductionReachabilityOwnerCustody` values;
- perform production durable recovery;
- create bootstrap freshness state;
- reload owners;
- retire lifecycles;
- populate or mutate owner-map membership;
- add live synchronization/watchers/reconcilers;
- publish the map to candidate execution.

GP remains the authority for eventual map-population semantics.

## 19. Dynamic-network invariant

GQ preserves the project-wide dynamic-network law.

Durable reachability state and owner authority remain keyed by logical/current peer identity, not static network coordinates.

No persistence key, executor selection or provider mapping may treat IP address, socket address, port, relay endpoint, DNS answer or current candidate endpoint as `PeerConnectivityIdentity`.

Canonical composition remains:

`logical device/session identity -> registry/discovery -> current reachable endpoint/candidates -> authenticated transport`.

## 20. Source-materialization target after GQ

After canonical GQ closure, a fresh exact-final-head audit may authorize a bounded source-materialization checkpoint for the provider-neutral awaitable durable-store execution path.

Expected materialization shape, subject to exact audit:

- evolve the existing durable-store operation signatures to an explicit awaitable form;
- propagate awaitability through only existing production-owner methods that perform durable I/O;
- propagate it through the existing candidate semantic execution boundary that invokes durable commit;
- update focused tests to await those same semantics;
- preserve all existing durable outcome/error/ordering laws;
- add no concrete provider, database schema, runtime owner, worker, network listener or Agent activation.

The exact file set must be re-derived before mutation. GQ does not pre-authorize a changed-path count.

## 21. Validation implications for source successor

A later source-materialization checkpoint must prove at minimum:

- existing focused production-owner recovery/commit/retirement semantics still pass;
- candidate-publication ordering remains admission -> requester authority -> exact durable commit;
- no hidden `block_on`/private runtime/thread/channel bridge is introduced;
- no second owner/store/freshness model is added;
- full locked workspace format/Clippy/tests/build pass on the exact final head;
- any manifest/lockfile change is separately justified and exact-head audited.

GQ itself is docs-only and does not claim executable validation of a future async shape.

## 22. Explicit non-selections

GQ does not select or materialize:

- etcd as candidate reachability durable backend;
- Spanner as candidate reachability durable backend;
- embedded SQL/KV storage;
- durable key encoding;
- snapshot byte serialization;
- schema versioning/migration;
- replication/consensus technology;
- provider timeout/retry/cancellation policy;
- provider credential custody changes;
- concrete `ReachabilityDurableStore` adapter;
- concrete `CandidatePublicationFreshnessTokenSource` adapter;
- live-owner/candidate-owner transactional coupling;
- distributed runtime tenancy;
- Agent owner-map population;
- candidate handoff/orchestration;
- traversal activation;
- listener/bind/accept behavior;
- readiness publication;
- production dialing;
- deployment/restart/process recovery action;
- merge;
- branch deletion;
- repository visibility mutation.

## 23. GQ changed-path ceiling

GQ is docs-only semantics selection.

Authorized changed path ceiling is exactly one path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_GQ_PRODUCTION_REACHABILITY_DURABLE_STORE_EXECUTION_MODEL_PREREQUISITE_SEMANTICS_SELECTION_STAGING.md`

No Rust source, Cargo manifest, lockfile, workflow, Agent/Linux runtime, remote transport, Android/Kotlin/Gradle, provider/database source, deployment/configuration or unrelated contract may change in GQ.

## 24. Exact-head validation law

Only the exact final GQ head may provide closure evidence.

If this contract is corrected, all validation attached only to a superseded head is superseded for closure purposes.

No workflow may be manually dispatched.

Path-filtered workflows must be recorded with their actual conclusion; skipped is not PASS.

## 25. Closure law

GQ may become canonically CLOSED only after all of the following hold:

1. exact predecessor is GP final head `55fde94b8b9ca66c22501ea2b4169013f7071047`;
2. GP -> GQ compare is ahead-only with exact GP merge base and zero behind commits;
3. changed-path set is exactly the one authorized GQ contract path;
4. exact final contract blob is recorded;
5. automatically triggered required CI on the exact final GQ head reaches terminal acceptable conclusions;
6. an immutable GQ audit is created from the exact final state;
7. local audit byte count and SHA-256 are recorded;
8. the audit is uploaded directly to the canonical `Private Remote Workspace` Drive folder;
9. the exact uploaded object is raw-fetched and byte/hash equality is verified;
10. PR/head state is re-read immediately before closure metadata mutation;
11. PR body becomes `Status: CLOSED` while the PR remains draft/open/unmerged;
12. PR and branch are independently re-read after closure.

## 26. Canonical selected law

Upon canonical closure, GQ selects:

**the GP-selected production owner-custody map cannot be populated from real production recovery inputs while durable reachability persistence remains a synchronous provider-neutral seam with no concrete production store; because the repository's established distributed persistence precedents are asynchronous, any network/database-backed `ReachabilityDurableStore` path must first become explicitly awaitable through the existing `ProductionReachabilityOwner` and candidate-execution composition, without hidden blocking, private runtimes, parallel owner models or changes to the already-locked durable CAS semantics.**

Canonical closure target:
`CLOSED_PRODUCTION_REACHABILITY_DURABLE_STORE_EXECUTION_MODEL_PREREQUISITE_SEMANTICS_SELECTION`

Canonical gate:
`C03E_GQ_PRODUCTION_REACHABILITY_DURABLE_STORE_EXECUTION_MODEL_PREREQUISITE_SELECTED`

## 27. Successor rule

After canonical GQ closure, perform a fresh exact-final-head source audit.

The expected successor is a bounded source-materialization checkpoint for the provider-neutral awaitable durable-store/owner/candidate-execution path selected here, if exact source topology confirms that it can be done without choosing a concrete backend or activating runtime behavior.

If exact source topology exposes a smaller prerequisite, choose that prerequisite instead.

GP startup/recovery custody-map population, concrete durable provider selection, concrete freshness-token source, Agent candidate handoff, runtime activation and deployment remain separately gated.