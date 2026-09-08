# Phase 152 C03e-NM — Production requester/rendezvous expected-device scheduling consumption-state source-materialization seam selection

Status: `STAGED_SELECTION`

Target gate:

`C03E_NM_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_STATE_SOURCE_MATERIALIZATION_SEAM_SELECTED`

## 1. Purpose

C03e-NL closed the representation, capacity and bounded-failure selection for the terminal process-local expected-device scheduling-consumption ledger.

NL selected:

- a private bounded `Vec<Key>` identity set;
- exact authenticated requester `SessionId` + exact target logical `DeviceId` as the only semantic key;
- one dedicated future production configuration identity, `PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS`;
- duplicate detection before capacity enforcement;
- `InvalidCapacity`, `AlreadyConsumed` and `CapacityExhausted` as the bounded ledger failures;
- no entry removal, eviction, TTL, compaction, retry, replay or remint;
- candidate-publication provider state and capacity as independent from scheduling-consumption state and capacity;
- source materialization still blocked pending exact seam selection.

This checkpoint selects only the smallest source-materialization seams and their order. It does not materialize Rust source.

## 2. Exact predecessor authority

Authoritative predecessor branch:

`phase-152-c03e-nl-production-requester-rendezvous-expected-device-scheduling-consumption-state-representation-capacity-failure-selection`

Exact predecessor head:

`45e49ae7b6d654d44a809d10bc183dc3a1bf1e0c`

Exact predecessor tree:

`f066ce599ef40ead23dd66d4cd5fa668ad154031`

Predecessor closure remains draft/open/unmerged and documentation-only.

## 3. Fresh exact-head source evidence

### 3.1 `linux_bootstrap.rs` already owns fixed process configuration sources

At the exact NL head, `crates/prw-agent/src/linux_bootstrap.rs` owns existing fixed non-secret process configuration identities, bounded source errors, strict parsers and environment loaders.

Relevant current examples include:

- `PRW_REMOTE_BIND_ADDR_ENV`;
- `PRW_REMOTE_PEER_DEVICE_ID_ENV`;
- `PRW_REMOTE_MAX_ACTIVE_WORKERS_ENV`;
- `PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS_ENV`.

The requester/rendezvous max-records source is especially relevant because it already demonstrates the required layering:

1. fixed env-name constant;
2. bounded source error;
3. strict ASCII-decimal parser;
4. fixed-name env loader;
5. parser/source tests;
6. syntactically valid zero preserved as target `usize`;
7. semantic non-zero enforcement delegated to the downstream state constructor;
8. no default/fallback/worker-limit alias;
9. no source-to-owner population in the source-materialization checkpoint itself.

### 3.2 Direct historical precedent: MQ -> MR

Fresh repository history confirms the exact earlier pattern:

- C03e-MQ selected `PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS` as one fixed env source;
- C03e-MR materialized only `crates/prw-agent/src/linux_bootstrap.rs`;
- final MR topology remained exactly one changed Rust path;
- MR materialized only constant/error/parser/loader/focused tests;
- source-valid zero remained zero;
- provider/runtime construction remained the sole non-zero semantic authority;
- population composition and caller/runtime wiring remained separately gated.

This is a directly applicable source-custody precedent, not merely a naming analogy.

### 3.3 `shared_requester_rendezvous_authority.rs` is the exact ledger representation seam

At the exact NL head, `crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs` already imports both `SessionId` and `DeviceId` and owns requester/rendezvous mutex custody.

The file currently defines:

`SharedRequesterRendezvousAuthority { runtime_owner: Arc<Mutex<CandidatePublicationRequesterRendezvousRuntimeOwner>> }`

and `SharedRequesterRendezvousAuthority::new(...)` accepts only the existing candidate-publication requester/rendezvous runtime owner.

Therefore the future scheduling-consumption key/ledger representation belongs in this same source file. A separate new module would add an unnecessary module/export seam before any source evidence requires it.

### 3.4 No module export change is needed for private ledger representation

`crates/prw-agent/src/remote_session_capability_runtime.rs` already declares the private `shared_requester_rendezvous_authority` module and re-exports only `SharedRequesterRendezvousAuthority` at crate visibility.

The NL-selected scheduling-consumption key, ledger and error can remain private to `shared_requester_rendezvous_authority.rs` while being unit-tested from that file's test module. No `remote_session_capability_runtime.rs` modification is required merely to materialize the dormant representation.

### 3.5 Immediate composite-owner integration would widen the diff

The current shared authority stores only `Arc<Mutex<CandidatePublicationRequesterRendezvousRuntimeOwner>>`. The NL/NK-selected future model instead requires candidate-publication runtime state and scheduling-consumption ledger state to serialize under the same requester mutex.

Actually integrating the ledger into `SharedRequesterRendezvousAuthority` will therefore require a later structural composition decision, such as an internal composite state under the existing mutex or another exact equivalent that preserves the same single linearization primitive.

Further, current production composition in `linux_bootstrap.rs` constructs the shared authority through:

`SharedRequesterRendezvousAuthority::new(requester_rendezvous_runtime_owner)`

Changing that constructor immediately to require scheduling-consumption capacity would force cross-file call-site/population changes. That would mix representation materialization, configuration custody and production owner population before each seam is independently validated.

NM therefore rejects immediate multi-file integration.

## 4. Candidate source-materialization strategies

### 4.1 One combined multi-file source checkpoint

Candidate:

- add the dedicated capacity env source in `linux_bootstrap.rs`;
- add ledger types in `shared_requester_rendezvous_authority.rs`;
- modify `SharedRequesterRendezvousAuthority` storage/constructor;
- feed the configured capacity into production construction;
- update all affected tests/call sites in one checkpoint.

Classification:

`REJECTED_AS_TOO_BROAD_FOR_FIRST_MATERIALIZATION`

Reason:

This combines independent syntactic configuration custody, state representation, owner structure and population composition. A failure could not be cleanly attributed to one seam and would unnecessarily widen the source diff.

### 4.2 Ledger representation first, capacity source second

Candidate:

- first materialize private key/ledger/error with test-only explicit capacities;
- then materialize the production env source.

Classification:

`VALID_BUT_NOT_SELECTED`

Reason:

This could compile safely, but it establishes a state constructor before the already-selected production capacity custody exists. The repository's MQ->MR precedent establishes configuration-source materialization before source-to-state population, so capacity-source first provides the cleaner dependency direction.

### 4.3 Dedicated capacity source first, private ledger representation second

Candidate:

1. one `linux_bootstrap.rs` source-only checkpoint;
2. one `shared_requester_rendezvous_authority.rs` private representation-only checkpoint;
3. only after both are evidence-closed, a separately selected owner/constructor/population integration checkpoint.

Classification:

`SELECTED`

Reason:

- each first-stage source diff is one file;
- each has a single semantic responsibility;
- both can remain dormant;
- capacity syntax and capacity semantics remain separated;
- source failures can be attributed precisely;
- no runtime or networking activation is necessary;
- existing candidate-publication owner behavior remains untouched;
- later composite integration can be audited with both lower-level primitives already proven.

## 5. Selected source-materialization sequence

NM selects the following exact sequence.

### Stage A — dedicated production capacity-source materialization

Immediate source successor may change exactly one Rust path:

`crates/prw-agent/src/linux_bootstrap.rs`

It may materialize only:

1. crate-private fixed constant:
   `PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS_ENV`;
2. exact env value:
   `PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS`;
3. one bounded source error containing exactly:
   - `Missing`;
   - `NonUnicode`;
   - `InvalidValue`;
4. one private strict ASCII-decimal parser returning target `usize`;
5. one fixed-name environment loader returning target `usize`;
6. focused parser/source tests.

The Stage-A parser contract is:

- accept only non-empty ASCII decimal digits `[0-9]+`;
- no trimming;
- no sign;
- no separators;
- no alternative radix;
- no normalization;
- fail closed on target-`usize` overflow;
- preserve syntactically valid zero exactly as `0`;
- disclose no configuration value in bounded errors.

The Stage-A source must not:

- use `NonZeroUsize` as the source return type;
- reject zero semantically;
- alias or fall back to `PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS`;
- alias or fall back to `PRW_REMOTE_MAX_ACTIVE_WORKERS`;
- derive capacity from expected-request channel capacity;
- derive capacity from active workers, provider records, registry size or runtime state;
- construct a scheduling ledger;
- modify `SharedRequesterRendezvousAuthority`;
- feed the value into any constructor or production operation.

The future ledger constructor remains the sole non-zero semantic authority.

### Stage B — private scheduling-consumption ledger representation materialization

Only after Stage A is separately validated and evidence-closed may the next source checkpoint change exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs`

It may materialize only a dormant private representation equivalent to the NL contract:

1. a private exact key with:
   - owned requester `SessionId`;
   - owned target logical `DeviceId`;
2. a private bounded error with exactly:
   - `InvalidCapacity`;
   - `AlreadyConsumed`;
   - `CapacityExhausted`;
3. a private ledger with:
   - one stored positive `max_records: usize`;
   - one `Vec<Key>` containing only terminal identities;
4. a constructor that rejects zero before creating the ledger;
5. one synchronous `commit_if_absent(key)`-equivalent mutation;
6. duplicate lookup before capacity lookup;
7. successful insertion as the only mutation;
8. focused representation tests.

Stage B must not yet add the ledger as a field of `SharedRequesterRendezvousAuthority`, change its constructor, expose a mint method, derive a scheduling grant, or read environment configuration.

The dormant private ledger may be instantiated only in focused unit tests during Stage B.

### Stage C — composite owner / constructor / population composition

Stage C is not authorized by NM for source materialization.

After Stages A and B are evidence-closed, a new documentation-only checkpoint must select the exact composition by which:

- the candidate-publication runtime owner and scheduling-consumption ledger become sibling state under the same existing requester mutex;
- configured scheduling-consumption capacity reaches that owner exactly once;
- existing candidate-publication APIs continue to operate on the runtime-owner component without changing their authority semantics;
- the current `SharedRequesterRendezvousAuthority::new(...)` constructor/call sites are migrated or supplemented without creating a default/fallback capacity path;
- no second mutex or alternate authority owner is introduced.

No exact internal composite-state struct name or constructor signature is selected here because those details must be audited after Stages A and B exist in source.

## 6. Private type/module custody selection

Selected custody:

- capacity source: `linux_bootstrap.rs`;
- key/error/ledger representation: `shared_requester_rendezvous_authority.rs`;
- no new module;
- no `remote_session_capability_runtime.rs` export change for Stage B;
- no public API exposure;
- no bridge/core/policy/registry crate type added.

Rationale:

The key and ledger are Agent-local scheduling-consumption implementation state, not reusable domain identity or transport protocol. `SessionId` and `DeviceId` remain existing core identities; the combined pair is not promoted into `prw-core`.

## 7. Constructor and population boundary

### 7.1 Capacity source constructor boundary

Stage A ends at a typed `usize` loader result. It performs syntax acquisition only.

The source does not own the non-zero invariant.

### 7.2 Ledger constructor boundary

Stage B constructor owns exactly the semantic invariant:

`max_records > 0`

and returns `InvalidCapacity` for zero.

No environment read occurs inside the ledger constructor.

### 7.3 Production population boundary

No production population is selected for source materialization in NM.

A later Stage-C selection must establish exact one-time source->ledger population and failure layering before any production owner is constructed with scheduling-consumption state.

## 8. Atomic mutation shape ceiling

Stage B may materialize one synchronous mutation equivalent to:

`commit_if_absent(key) -> Result<(), SchedulingConsumptionError>`

Required semantic order under future requester-mutex custody:

1. exact key lookup;
2. if present: `AlreadyConsumed`;
3. otherwise capacity check;
4. if full: `CapacityExhausted`;
5. otherwise insert exact key;
6. return success.

No mutation may occur on either failure path.

The Stage-B method itself does not acquire a mutex; synchronization remains the responsibility of the future owning `SharedRequesterRendezvousAuthority` composition selected by NK.

## 9. Entry data ceiling

Each future terminal ledger entry stores only:

- requester `SessionId`;
- target logical `DeviceId`.

It must not retain:

- authenticated session object;
- requester `DeviceId` separately unless already encoded only through the selected session identity semantics;
- workspace/user identity snapshot;
- policy decision;
- capability enum;
- registry binding/lifecycle snapshot;
- transport identity;
- endpoint/IP/port;
- candidate/reachability state;
- request/correlation ID;
- target admission `SessionId`;
- authentication PRWM request ID;
- verifier/admission time;
- channel/worker/task identity;
- downstream success/failure;
- retry/reconnect counters.

## 10. Capacity independence proof obligation

The selected scheduling-consumption capacity is terminal-process-lifetime memory capacity.

The existing requester/rendezvous provider capacity is reusable provider-record capacity because retired records can be removed and free slots.

Therefore:

`SCHEDULING_CONSUMPTION_MAX_RECORDS != REQUESTER_RENDEZVOUS_MAX_RECORDS_BY_SEMANTIC_DEFINITION`

The values may happen to be numerically equal in some deployment, but there is no authority to infer, alias, copy or default one from the other.

The same independence applies to `PRW_REMOTE_MAX_ACTIVE_WORKERS` and any channel bound.

## 11. Failure-domain separation

Stage A source failures are syntactic configuration failures only:

- missing;
- non-Unicode;
- invalid decimal/overflow.

Stage B ledger failures are semantic state failures only:

- invalid zero capacity at construction;
- already-consumed key;
- distinct-key capacity exhaustion.

These error domains must not be flattened into each other before the future Stage-C population-selection checkpoint.

Neither domain is a peer-visible protocol error in these dormant stages.

## 12. Focused test ceiling

### Stage A tests

Tests should prove at minimum:

- exact env constant spelling;
- loader signature returns `usize`;
- missing -> `Missing`;
- non-Unicode -> `NonUnicode` where platform test support exists;
- empty/malformed/signed/whitespace/non-decimal -> `InvalidValue`;
- overflow -> `InvalidValue`;
- positive decimal preserved exactly;
- zero preserved exactly;
- no fallback source is consulted.

### Stage B tests

Tests should prove at minimum:

- zero constructor capacity -> `InvalidCapacity`;
- positive constructor starts empty;
- first exact key commits successfully;
- exact duplicate -> `AlreadyConsumed`;
- duplicate remains `AlreadyConsumed` even when full;
- new distinct key at full capacity -> `CapacityExhausted`;
- failure paths preserve state;
- multiple distinct keys commit until exact finite bound;
- order has no authority meaning;
- no removal API exists in the selected surface.

No network/runtime integration test is required or authorized for either stage.

## 13. Why no `HashSet`

NL already selected bounded `Vec<Key>`.

NM does not reopen that representation choice.

The exact key count is explicitly finite, ordering has no authority meaning, and the existing requester/rendezvous provider already demonstrates bounded linear search over a `Vec`. No hashing dependency or alternate representation is introduced here.

## 14. Why no new core/domain type

The pair `(SessionId, DeviceId)` is private operation-consumption state, not a globally reusable identity.

Creating a public/core operation ID would risk being mistaken for a new authority identity and would contradict NK's explicit refusal to invent a new operation-generation identifier.

The future private key therefore remains module-local.

## 15. Locking remains unchanged

NM materializes no lock and changes no lock.

The future owner remains governed by NK:

- one requester mutex only;
- requester mutex first;
- shared-current authority read second when authority mint eventually exists;
- no network I/O/channel send/response write while authority locks are held.

Stage B's private ledger mutation is synchronous specifically so it can later execute entirely inside that already-selected requester critical section.

## 16. Candidate-publication provider remains unchanged

Neither Stage A nor Stage B may change:

- `CandidatePublicationRequesterRendezvousRuntimeOwner`;
- `InMemoryRequesterRendezvousAuthorityProvider`;
- provider max-records configuration;
- provider `Current -> Retired -> removed` lifecycle;
- candidate grant authorization;
- durable candidate commit;
- post-commit requester cleanup.

The scheduling-consumption ledger remains sibling future state, never embedded in or cleaned by the candidate provider.

## 17. Source-materialization selection result

Selected result:

`SPLIT_ONE_FILE_SOURCE_MATERIALIZATION_SEQUENCE_SELECTED / DEDICATED_SCHEDULING_CONSUMPTION_CAPACITY_SOURCE_FIRST_IN_LINUX_BOOTSTRAP / PRIVATE_LEDGER_REPRESENTATION_SECOND_IN_SHARED_REQUESTER_RENDEZVOUS_AUTHORITY / NO_MODULE_EXPORT_CHANGE_FOR_LEDGER_REPRESENTATION / COMPOSITE_OWNER_CONSTRUCTOR_AND_POPULATION_INTEGRATION_DEFERRED / MULTI_FILE_SOURCE_MATERIALIZATION_NOT_AUTHORIZED`

This selection preserves the source boundary:

`CONFIGURATION_SYNTAX_CUSTODY -> PRIVATE_STATE_REPRESENTATION -> LATER_EXPLICIT_OWNER_POPULATION_COMPOSITION`

No stage is allowed to skip ahead.

## 18. Immediate next blocking prerequisite

Selected immediate next boundary:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_CAPACITY_ENV_SOURCE_MATERIALIZATION`

That checkpoint may materialize only Stage A in:

`crates/prw-agent/src/linux_bootstrap.rs`

and must remain dormant.

After exact-head validation and durable evidence closure of Stage A, the next allowed source boundary is:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_LEDGER_REPRESENTATION_MATERIALIZATION`

which may materialize only Stage B in:

`crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs`

After both source primitives are closed, a fresh documentation-only checkpoint is required before any Stage-C owner/population integration.

## 19. Explicitly not selected

NM does not select or authorize:

- any Rust source change in this checkpoint;
- a new public module;
- a new `prw-core` identity;
- a new capability;
- policy mutation;
- `SharedRequesterRendezvousAuthority` field/storage mutation;
- constructor signature mutation;
- production source->ledger population;
- scheduling authority mint method;
- owned scheduling grant carrier;
- expected-device scheduling event/request construction;
- `RemoteSessionExpectedDeviceAdmissionRequest` producer construction;
- expected-request `mpsc` channel/sender creation or custody;
- target admission `SessionId` generation;
- authentication PRWM request-ID allocation/reuse;
- verifier/admission timing selection;
- dispatcher/caller migration;
- listener/readiness/main/systemd/package wiring;
- retry/reconnect/replay behavior;
- TTL/timer/background compaction;
- persistence/database/etcd state;
- restart restoration;
- deployment;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- force update, rebase, squash or history rewrite.

## 20. Closure condition

C03e-NM may close only if:

- it remains documentation-only;
- exact NL -> NM topology is one commit and one contract path only;
- exact-final-head Rust validation is successful;
- path-filtered skips are reported as `SKIPPED`, not PASS;
- no Android PASS is claimed without an exact-head Android workflow;
- immutable Drive evidence is published with byte-exact raw readback;
- branch/PR topology is re-read before closure metadata;
- PR remains draft/open/unmerged.

Closure token:

`CLOSED_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_STATE_SOURCE_MATERIALIZATION_SEAM_SELECTION`

**STOP after C03e-NM durable closure.**
