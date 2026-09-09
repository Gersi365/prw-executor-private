# Desktop Functional Management Slice C03e-NP

## Production requester/rendezvous expected-device scheduling-consumption ledger owner/population integration selection — STAGING

Status: `SELECTION / DOCUMENTATION_ONLY / SOURCE_MATERIALIZATION_BLOCKED`

Gate candidate:

`C03E_NP_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_LEDGER_OWNER_POPULATION_INTEGRATION_SELECTED`

Predecessor authority:

- exact closed C03e-NO head: `98fd8d1300b08f180baf1adf03e6a7dce8b69e44`;
- exact closed C03e-NO tree: `948bd0308a834d6debda60159d55c8e597bbb673`;
- exact C03e-NO target blob: `06a6de51a488877aa4b1b6036f49c6584e5e490c`;
- C03e-NO PR: `#503`, intentionally draft/open/unmerged;
- C03e-NO durable evidence Drive ID: `1WJJ7dERpJziaZn2HAAJ2WZgxExXlTNRe`.

This checkpoint is documentation-only. It selects the exact dormant owner/population integration seam that may follow the separately evidence-closed C03e-NN Stage-A capacity source and C03e-NO Stage-B private ledger representation. It does not materialize Rust source, activate scheduling, construct an expected-device admission request, create a sender/channel, allocate a request ID, generate an admission `SessionId`, choose timing, invoke a listener, deploy, merge, or mutate production/control-plane state.

---

## 1. Closed predecessor invariants

C03e-NJ selected one bounded internal scheduling consequence of authenticated `RequesterRendezvousStart`, but raw requester policy `Allow`, candidate-publication authority, configured-peer state, transport identity, endpoint/reachability data, correlation IDs, target admission session IDs, timing values and post-authenticated identity are not scheduling authority.

C03e-NK selected scheduling-specific one-shot terminal consumption as process-local sibling state under `SharedRequesterRendezvousAuthority`, with the existing requester mutex as the sole linearization primitive. The terminal key is authenticated requester `SessionId` plus exact target logical `DeviceId`. Successful terminal insertion records authority consumption, not downstream execution success. Candidate-publication provider cleanup never removes the scheduling tombstone.

C03e-NL selected:

- private key: requester `SessionId` + target `DeviceId` only;
- private bounded `Vec<Key>` ledger;
- finite explicit `max_records`;
- duplicate check before capacity check;
- `InvalidCapacity`, `AlreadyConsumed`, `CapacityExhausted`;
- successful insertion as terminal-consumption linearization point;
- no eviction, TTL, compaction, persistence, retry, replay or restart restoration;
- dedicated non-secret configuration identity `PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS`.

C03e-NM selected a split source sequence:

1. Stage A: dedicated scheduling-consumption capacity source in `linux_bootstrap.rs`;
2. Stage B: private ledger representation in `shared_requester_rendezvous_authority.rs`;
3. Stage C owner/constructor/population integration deferred behind a fresh documentation gate.

C03e-NN completed Stage A. Its loader returns target `usize`, preserves syntactically valid zero unchanged and does not perform the later semantic non-zero check.

C03e-NO completed Stage B. The private ledger constructor alone rejects zero as `InvalidCapacity`; the ledger remains unattached to `SharedRequesterRendezvousAuthority` and the Stage-A source remains unconsumed.

No predecessor authorizes source mutation for Stage C without this selection.

---

## 2. Exact C03e-NO source facts relevant to Stage C

### 2.1 `shared_requester_rendezvous_authority.rs`

Exact C03e-NO source shows:

```text
pub struct SharedRequesterRendezvousAuthority {
    runtime_owner: Arc<Mutex<CandidatePublicationRequesterRendezvousRuntimeOwner>>,
}
```

The current constructor takes only the requester/rendezvous runtime owner and is infallible:

```text
pub fn new(runtime_owner: CandidatePublicationRequesterRendezvousRuntimeOwner) -> Self
```

All existing requester/rendezvous registration, current-grant selection and committed-record cleanup methods acquire the same `runtime_owner` mutex. Candidate-publication durable commit itself is performed after requester lock release, and post-commit requester cleanup reacquires the requester lock only afterward.

C03e-NO appended one private nested `expected_device_scheduling_consumption_ledger` module containing:

- `ExpectedDeviceSchedulingConsumptionKey`;
- `ExpectedDeviceSchedulingConsumptionLedgerError`;
- `ExpectedDeviceSchedulingConsumptionLedger`;
- `new(max_records)`;
- `commit_if_absent(key)`;
- focused representation tests.

That module is currently dormant and explicitly not attached to `SharedRequesterRendezvousAuthority`.

### 2.2 `linux_bootstrap.rs`

Exact C03e-NO source already contains the C03e-NN Stage-A fixed source:

`PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS`

and the crate-private loader:

`load_linux_agent_remote_expected_device_scheduling_consumption_max_records_from_env()`

The source returns exact target `usize`. Missing, non-Unicode, malformed and target-`usize` overflow values fail at the source boundary. Valid zero passes through unchanged.

The production requester/rendezvous operation input currently stores:

- existing production/reachability inputs;
- `BoundedRequesterRendezvousStartPolicySource`;
- raw `CandidatePublicationRequesterRendezvousRuntimeOwner`.

The existing production operation assembly then constructs:

`SharedRequesterRendezvousAuthority::new(requester_rendezvous_runtime_owner)`.

Thus the current shared requester owner is created only after the raw requester/rendezvous runtime owner has already been populated and joined into the production input carrier.

### 2.3 `production_durable_capability_higher_owner_custody.rs`

The current production population chain already has the requester/rendezvous capacity pattern selected in C03e-MQ through C03e-MT:

- fixed requester/rendezvous max-records source;
- explicit-capacity provider/runtime-owner construction;
- existing combined population error;
- configured population wrapper;
- no executable caller activation.

The existing explicit-capacity combined population error preserves two lanes:

- `ProductionSources(...)`;
- `RequesterRendezvousRuntime(...)`.

The existing configured population error preserves:

- `RequesterRendezvousMaxRecordsSource(...)`;
- `Population(...)`.

The configured wrapper loads the requester/rendezvous capacity source and then delegates into the existing explicit-capacity population chain. This is the controlling precedent for preserving source errors separately from semantic constructor errors and for nesting existing historical errors rather than flattening them.

---

## 3. Integration options considered

### Option A — add a second scheduling mutex beside the requester runtime-owner mutex

Rejected.

C03e-NK selected the existing requester mutex as the sole scheduling-state linearization primitive. A second mutex would permit independent lock acquisition and would create an avoidable lock-order/concurrency domain not authorized by NK.

### Option B — keep the existing constructor and use an implicit/default scheduling capacity

Rejected.

No default, fallback, worker-limit alias, requester-provider-capacity alias, channel-capacity alias or derived host/runtime bound was selected by NL/NN. Silent use of `usize::MAX`, requester provider capacity, worker capacity or any constant would violate explicit capacity provenance.

### Option C — keep the existing constructor and attach `Option<Ledger>` / uninitialized state

Rejected for the selected production integration.

A production owner that can exist without its selected terminal-consumption ledger would introduce an additional uninitialized state and an alternate construction path. That would require a new runtime failure class or a later hidden initialization transition not selected by NL/NK. Staging must not obtain compile convenience by weakening the final fail-closed owner invariant.

### Option D — pre-validate zero outside the ledger constructor

Rejected.

C03e-NN deliberately preserves syntactically valid zero, and C03e-NL/C03e-NO keep the ledger constructor as the sole semantic non-zero authority. A separate `NonZeroUsize` conversion, manual `== 0` precheck or alternate validation helper would duplicate or move that semantic authority.

### Option E — leave the ledger outside `SharedRequesterRendezvousAuthority` and attach it in a separate wrapper owner

Rejected.

NK selected scheduling consumption as sibling state under the existing shared requester authority and requester mutex. A second top-level wrapper owner would create an alternate custody object and would make exact one-mutex serialization less direct.

### Option F — atomic bounded dormant integration across the minimum coordinated source seam

Selected.

The Stage-A source, Stage-B representation, current shared-owner shape and existing production population chain live in three exact Rust paths. A compile-valid integration that preserves all selected invariants necessarily coordinates those paths. Splitting the actual owner integration into a one-file source checkpoint would require at least one rejected temporary state: implicit capacity, optional/uninitialized ledger, second mutex, duplicate semantic validation, or an alternate owner type.

Therefore the source successor is permitted a tightly bounded three-file dormant integration seam and no fourth path.

---

## 4. Selected owner state

The integrated `SharedRequesterRendezvousAuthority` must own exactly one shared mutex-protected composite state containing both:

1. the existing `CandidatePublicationRequesterRendezvousRuntimeOwner`;
2. the C03e-NO `ExpectedDeviceSchedulingConsumptionLedger`.

The exact internal Rust spelling remains an implementation detail, but the semantics are fixed:

```text
Arc<Mutex<COMPOSITE_STATE>>

COMPOSITE_STATE:
- requester/rendezvous runtime owner
- expected-device scheduling-consumption ledger
```

There must not be:

- two `Arc`s representing independent requester and scheduling ownership;
- two mutexes;
- a global/static ledger;
- a lock-free side table;
- an `Option<Ledger>` production mode;
- provider-owned scheduling tombstones;
- candidate-publication cleanup-driven ledger removal.

`Clone` must continue to clone only the outer shared `Arc`, so all clones observe the same requester/rendezvous provider state and the same terminal scheduling-consumption state.

Existing requester/rendezvous and candidate-publication operations must acquire the same shared mutex and project only the runtime-owner field needed by the existing operation. They must not read or mutate the scheduling ledger unless a later separately gated scheduling-authority method explicitly requires it.

The existing requester mutex remains the sole linearization primitive for future `commit_if_absent`.

---

## 5. Selected constructor invariant

The final production shared requester authority must not be constructible without an explicit scheduling-consumption capacity.

The owner constructor must receive:

- one already-populated `CandidatePublicationRequesterRendezvousRuntimeOwner` by value;
- one exact caller-supplied target `usize expected_device_scheduling_consumption_max_records` by value.

The exact source `usize` must be passed unchanged into the C03e-NO ledger constructor exactly once.

The ledger constructor remains the sole semantic non-zero authority:

- positive capacity -> construct empty ledger;
- zero -> `InvalidCapacity`;
- no clamp/default/fallback/retry.

Only after successful ledger construction may the requester runtime owner and ledger be joined into one shared composite state.

If ledger construction fails, no partially initialized `SharedRequesterRendezvousAuthority` may escape.

The source successor may introduce one bounded shared-owner construction error or one exact mapping lane required to carry `InvalidCapacity` out of construction. It must not collapse source errors, requester-provider errors and ledger semantic errors into one generic failure.

`AlreadyConsumed` and `CapacityExhausted` are future mutation-time ledger outcomes; they must not be misrepresented as constructor failures.

---

## 6. Selected production population order

The configured production population chain must preserve deterministic fail-closed stage order.

Selected order:

1. load the existing requester/rendezvous max-records source exactly once;
2. load `PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS` exactly once through the existing C03e-NN loader;
3. only after both source reads succeed, run the existing pre-requester production population stages exactly once;
4. construct the requester/rendezvous provider/runtime owner exactly once from the unchanged requester/rendezvous capacity;
5. construct the shared requester authority exactly once from that exact runtime owner plus the unchanged scheduling-consumption capacity;
6. move the successfully constructed shared requester authority into the production requester/rendezvous input carrier exactly once.

This order preserves the existing requester/rendezvous source as the first configured capacity source while preventing durable/provider population after a missing/malformed scheduling-consumption source.

Source-valid zero is not rejected in step 2. It reaches step 5 unchanged and fails only through the ledger constructor's selected `InvalidCapacity` authority.

A failure at any stage short-circuits all later stages.

No partial requester/rendezvous runtime owner, ledger or shared authority is returned after a later failure.

No stage is retried.

No source is reread dynamically.

---

## 7. Selected production input migration

The production requester/rendezvous operation input that currently stores a raw `CandidatePublicationRequesterRendezvousRuntimeOwner` must, after successful configured population, retain the already-constructed `SharedRequesterRendezvousAuthority` by value instead.

This moves fallible scheduling-capacity/ledger construction into the population boundary rather than deferring it to operation invocation.

Consequently the later production operation assembly must not perform another shared requester authority construction. It receives the exact already-populated shared authority and only moves/clones it according to existing operation custody semantics.

This prevents:

- environment access during operation invocation;
- repeated ledger construction per publisher/session/operation;
- panic/expect-based zero handling;
- a second owner instance;
- a second ledger instance;
- operation-time partial initialization.

The exact production operation remains dormant until a later caller/runtime activation gate. This checkpoint does not authorize invoking it from `run()`, `main.rs`, listener ownership or readiness publication.

---

## 8. Selected failure preservation

Existing historical errors must remain semantically distinguishable.

The source successor may extend the configured population error surface only enough to preserve these stages without flattening:

1. existing requester/rendezvous max-records source error;
2. scheduling-consumption max-records source error from C03e-NN;
3. existing production-source population error;
4. existing requester/rendezvous provider/runtime construction error;
5. shared requester authority / scheduling-ledger construction `InvalidCapacity` error.

Existing error variants must retain their current meaning.

The preferred shape is to preserve existing population errors nested intact and add a bounded scheduling-source lane plus a bounded shared-owner construction lane, rather than rewriting older source/provider errors into new generic text.

No error value may expose environment contents, requester `SessionId`, target `DeviceId`, credentials, transport identity or endpoint data.

No failure authorizes retry, fallback, alternate capacity, degraded owner construction or runtime activation.

---

## 9. Lock and concurrency invariants

The future shared composite mutex must preserve the already-selected lock order:

1. requester/shared-owner mutex first when requester/scheduling state must be linearized;
2. shared-current registry/policy read only afterward when required by the existing requester-start or future scheduling-authority composition;
3. release authority locks before network I/O, frame writes, channel sends or downstream durable execution unless a separately closed contract proves a narrower existing await boundary.

C03e-NP does not authorize the future scheduling mint/commit method itself.

When that later method is selected, `commit_if_absent` must execute while the shared requester mutex is held and successful insertion remains the terminal-consumption linearization point.

Candidate-publication authorization/cleanup continues to project only the existing runtime-owner field. Candidate-publication `Current -> Retired -> removed` lifecycle does not mutate scheduling terminal state.

---

## 10. Exact source-successor path ceiling

A separately gated source-materialization successor may modify exactly these three Rust paths and no others:

1. `crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs`
2. `crates/prw-agent/src/linux_bootstrap.rs`
3. `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Allowed changes are limited to the selected dormant integration:

### Path 1 — shared requester authority

- integrate the existing private C03e-NO ledger into one mutex-protected shared composite state;
- adapt `Clone` to share the same composite `Arc`;
- make owner construction explicitly scheduling-capacity aware and fallible only as required by ledger `InvalidCapacity`;
- adapt existing requester/candidate methods to project the existing runtime-owner field under the same mutex;
- update focused same-file tests for clone/shared-lock/constructor invariants;
- no scheduling mint/grant method yet.

### Path 2 — Linux bootstrap

- preserve the C03e-NN source/parser/loader unchanged in semantics;
- migrate the production requester/rendezvous operation input from raw runtime owner custody to already-constructed `SharedRequesterRendezvousAuthority` custody;
- adapt the corresponding dormant production operation assembly so it does not construct a second shared authority;
- update focused same-file type/operation-shape tests only as necessary;
- no executable caller wiring.

### Path 3 — higher-owner production population

- consume the existing C03e-NN scheduling-consumption capacity loader exactly once in the selected order;
- thread exact unchanged requester and scheduling capacities into the existing population/join stages;
- construct the shared requester authority exactly once after requester runtime-owner success;
- preserve old population errors and add only bounded source/construction error lanes required by this integration;
- return the existing higher-owner lineage containing the integrated shared requester authority;
- no companion invocation, listener/readiness/network activation or executable caller migration.

If the implementation requires a fourth source path, manifest/lockfile/workflow change, Android source change, packaging/service change, database/control-plane change, or runtime activation, the source checkpoint must STOP and return to a new documentation gate.

---

## 11. Explicitly forbidden implementation shortcuts

The source successor must not introduce:

- `Option<ExpectedDeviceSchedulingConsumptionLedger>` as a production construction mode;
- `usize::MAX` or any constant default scheduling capacity;
- reuse of `PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS` as scheduling capacity;
- reuse of `PRW_REMOTE_MAX_ACTIVE_WORKERS`;
- channel capacity as scheduling-ledger capacity;
- capacity derived from CPU count, memory, peer count, registry size or provider state;
- `NonZeroUsize` conversion before the ledger constructor;
- manual `max_records == 0` semantic precheck outside the ledger constructor;
- panic/`expect`/`unwrap` for production scheduling capacity;
- second scheduling mutex;
- global/static scheduling ledger;
- persistence, etcd/database record or restart restore;
- tombstone eviction/TTL/cleanup;
- provider cleanup deleting a scheduling tombstone;
- candidate-publication authority reuse as scheduling authority;
- new policy capability;
- scheduling authority mint/grant method;
- expected-device admission request construction;
- expected-request sender/channel creation;
- target-side admission `SessionId` allocation;
- PRWM request-ID allocation/reuse;
- verifier/admission timing selection;
- retry/reconnect/remint/replay;
- listener/readiness/network activation;
- executable `run()`/`main.rs` caller wiring;
- deployment, restart, merge, PR close, ready-for-review conversion, branch deletion, force update, rebase/squash or destructive evidence cleanup.

---

## 12. Alternatives rejected by compile-boundary analysis

A one-file Stage-C source checkpoint is not selected.

`shared_requester_rendezvous_authority.rs` alone cannot enforce the final explicit-capacity constructor invariant without breaking the current production construction call site or introducing a default/uninitialized compatibility path.

`linux_bootstrap.rs` alone cannot attach the private ledger to the shared owner.

`production_durable_capability_higher_owner_custody.rs` alone cannot alter the shared owner representation or operation input custody.

A two-file integration still leaves either the population source unconsumed at the actual configured population boundary or the operation input/shared-owner construction call site inconsistent.

The three selected paths are therefore the minimum coherent dormant integration seam proven by the exact C03e-NO source and the established MR→MS→MT population precedent.

This is an exception to the earlier Stage-A/Stage-B one-file materialization discipline, not a general authorization for multi-file source changes.

---

## 13. Validation requirements for the source successor

The source successor must validate only an exact final head after all formatter/lint corrections.

At minimum:

- locked dependency graph: PASS;
- `cargo fmt --all -- --check`: PASS;
- workspace Clippy with warnings denied: PASS;
- workspace tests: PASS;
- workspace build: PASS;
- exact-head Android validation if the workflow is triggered; if not triggered, no Android PASS claim;
- path-filtered AD/AE results recorded exactly as returned; `SKIPPED` is not PASS.

Focused Rust tests must prove at least:

1. zero scheduling capacity fails owner construction through the selected ledger semantic authority;
2. positive scheduling capacity constructs one shared owner;
3. clone shares the exact same composite mutex/state allocation;
4. existing requester/rendezvous grant-selection lock release behavior remains intact;
5. existing candidate-publication commit/cleanup behavior remains intact;
6. configured population source ordering is deterministic;
7. scheduling source failure occurs before production/provider population;
8. exact scheduling `usize` is forwarded unchanged;
9. valid zero reaches the owner/ledger constructor unchanged rather than being source-rejected;
10. no duplicate shared-owner construction occurs in production operation assembly.

No runtime scheduling behavior test is required or authorized yet because no scheduling mint/event/request path is selected by NP.

---

## 14. Selected source successor

After NP durable closure, the next separately gated boundary is:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_LEDGER_OWNER_POPULATION_INTEGRATION_SOURCE_MATERIALIZATION`

Likely branch name, subject to a fresh GitHub successor check:

`phase-152-c03e-nq-production-requester-rendezvous-expected-device-scheduling-consumption-ledger-owner-population-integration-source-materialization`

That successor may materialize only the exact three-path dormant integration selected here.

It must STOP immediately if source inspection proves the integration requires a fourth path or an invariant not selected in NP.

---

## 15. Boundary after the source successor

Even after owner/population integration source materialization is evidence-closed, scheduling authority derivation remains separately blocked.

A fresh documentation gate will still be required before adding any method that:

- rechecks requester/target currentness;
- reauthorizes requester policy;
- checks current requester/rendezvous provider uniqueness;
- calls `commit_if_absent` for terminal scheduling consumption;
- returns a derived one-shot scheduling authority grant;
- constructs a scheduling event;
- constructs `RemoteSessionExpectedDeviceAdmissionRequest`;
- sends to expected-device admission;
- activates production caller/listener behavior.

Owner/population integration is state custody only, not scheduling authority activation.

---

## 16. Selected result

`ATOMIC_THREE_PATH_DORMANT_OWNER_POPULATION_INTEGRATION_SEAM_SELECTED / SHARED_REQUESTER_RENDEZVOUS_AUTHORITY_SINGLE_MUTEX_COMPOSITE_STATE_SELECTED / EXACT_REQUESTER_AND_SCHEDULING_CAPACITY_SOURCES_PRESERVED_SEPARATELY / SCHEDULING_CAPACITY_FORWARDED_UNCHANGED_TO_LEDGER_CONSTRUCTOR / LEDGER_CONSTRUCTOR_REMAINS_SOLE_NONZERO_SEMANTIC_AUTHORITY / RAW_RUNTIME_OWNER_TO_PRECONSTRUCTED_SHARED_AUTHORITY_INPUT_MIGRATION_SELECTED / EXISTING_PROVIDER_AND_CANDIDATE_PUBLICATION_SEMANTICS_PRESERVED / RUNTIME_SCHEDULING_AUTHORITY_ACTIVATION_DEFERRED / SOURCE_MATERIALIZATION_BLOCKED`

---

## STOP

C03e-NP is documentation-only.

Do not materialize Rust source in this checkpoint.
Do not merge.
Do not deploy.
Do not activate runtime behavior.
Keep the future NP PR draft/open/unmerged after evidence closure.
