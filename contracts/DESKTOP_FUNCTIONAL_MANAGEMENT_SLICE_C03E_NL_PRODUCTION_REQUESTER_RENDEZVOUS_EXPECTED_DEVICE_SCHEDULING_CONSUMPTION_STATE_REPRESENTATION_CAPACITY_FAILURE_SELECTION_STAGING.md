# Phase 152 C03e-NL — Production requester/rendezvous expected-device scheduling consumption-state representation, capacity and failure selection

Status: `STAGED_SELECTION`

Target gate:

`C03E_NL_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_STATE_REPRESENTATION_CAPACITY_AND_FAILURE_SELECTED`

## 1. Purpose

C03e-NK closed ownership and linearization for the requester-driven expected-device scheduling consumption state.

NK selected:

- sibling process-local scheduling-consumption state under `SharedRequesterRendezvousAuthority`;
- authenticated requester `SessionId` plus exact target logical `DeviceId` as the terminal operation key;
- the existing requester mutex as the sole scheduling-state linearization primitive;
- requester-mutex -> shared-current-authority lock ordering;
- lazy terminal insertion only at the fresh authority-mint commit point;
- no rollback, remint, retry, replay, eviction or candidate-provider cleanup coupling after commit;
- process-local drop with the final shared requester-authority owner;
- no persistence, timer, TTL, background compaction or restart restoration.

NK intentionally did not select the concrete private state representation, finite capacity source, bounded failure surface or future source-materialization seam.

C03e-NL selects only those remaining representation/capacity/failure semantics.

No Rust/source/runtime materialization is authorized by this checkpoint.

## 2. Exact predecessor

Exact C03e-NK head:

`3966bdacd4f4b66ac4d1afe9d79561f28b2f2a55`

Exact C03e-NK tree:

`8067c2ec111532080215f4e351c3d845dc5562c3`

Exact C03e-NK contract blob:

`e397802f65c86c85e3666530e8257614d9d28bd5`

C03e-NL must remain rooted exactly at this closed predecessor unless a fresh concurrency audit proves a newer authoritative successor.

## 3. Source evidence re-read at exact NK head

### 3.1 Strongly typed operation-key components already support exact equality

`prw-core` defines `SessionId` and `DeviceId` as strongly typed owned identifiers with exact `PartialEq`, `Eq`, `Clone` and `Hash` semantics.

No new textual identifier grammar is required for the NK-selected key.

The scheduling consumption state must not decompose either identifier into raw strings for authority comparison.

### 3.2 Existing requester/rendezvous provider establishes a bounded in-memory precedent

`InMemoryRequesterRendezvousAuthorityProvider` owns:

- one explicit finite `max_records: usize`;
- a private `Vec<RequesterRendezvousRecord>`;
- exact duplicate detection before capacity enforcement;
- fail-before-mutation capacity exhaustion;
- explicit constructor rejection of zero capacity;
- no timer, persistence or implicit lifecycle task.

Its record ordering has no authority meaning.

That provider is candidate-publication authority state, not scheduling-consumption state, so NL may reuse only the bounded representation law, not the provider record lifecycle or capacity source.

### 3.3 Existing requester/rendezvous capacity is semantically incompatible with terminal scheduling tombstones

`PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS` bounds concurrently retained requester/rendezvous provider records.

Those provider records may be retired and removed after definite successful candidate publication, reclaiming provider capacity.

NK selected the scheduling-consumption tombstone as terminal for the lifetime of the process-local shared requester authority.

Therefore provider capacity and scheduling-consumption capacity are not the same quantity.

Reusing `PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS` would silently couple a recyclable current-record bound to a non-recyclable lifetime-consumption bound and is rejected.

### 3.4 Active-worker capacity is also semantically incompatible

`PRW_REMOTE_MAX_ACTIVE_WORKERS` bounds active remote workers.

Worker completion reclaims worker capacity.

A terminal scheduling-consumption tombstone is not an active worker and is not reclaimed on worker completion.

The worker bound must not be aliased into scheduling-consumption capacity.

### 3.5 Existing requester mutex already supplies atomicity

NK selected the existing requester mutex as the sole scheduling-state linearization primitive.

Consequently the future representation requires no internal mutex, atomic counter, lock-free structure or concurrent collection.

The representation may remain an ordinary private mutable value accessed only while the established requester mutex is held.

## 4. Candidate representation classes

NL compares the following representation classes.

### A. Unbounded `Vec<Key>`

Rejected.

Reason:

- violates the explicit bounded-memory discipline;
- terminal tombstones are never removed during process lifetime;
- repeated distinct requester-session/target operations could grow memory without a configured ceiling.

### B. Bounded `Vec<Key>` identity set

Selected.

Properties:

- stores only exact terminal operation identities;
- linear duplicate lookup is bounded by explicit finite capacity;
- insertion order has no authority meaning;
- no hash seed, map value, lifecycle enum or timestamp is required;
- duplicate-before-capacity ordering is explicit and deterministic;
- no independent synchronization primitive is introduced.

### C. `HashSet<Key>`

Not selected.

`SessionId` and `DeviceId` are hashable, so this is mechanically possible, but it adds no authority semantic needed by the selected bounded state.

A bounded `Vec` is sufficient, simpler, deterministic in mutation behavior and consistent with the existing bounded requester/rendezvous provider precedent.

### D. `HashMap<Key, Metadata>` or record map

Rejected.

No authoritative metadata value is required after terminal consumption.

Adding timestamps, policy snapshots, registry snapshots, execution disposition, retry counters or downstream request/session correlation would create unselected semantics.

### E. Reuse or extension of `InMemoryRequesterRendezvousAuthorityProvider`

Rejected.

Candidate-publication provider records have `Current -> Retired -> removed` lifecycle and recyclable capacity.

Scheduling-consumption tombstones are terminal process-lifetime state under NK.

The two lifecycles must remain independent.

## 5. Selected exact private operation-key shape

Selected future private semantic shape:

```text
ExpectedDeviceSchedulingConsumptionKey {
    requester_session_id: SessionId,
    target_device_id: DeviceId,
}
```

The exact Rust name remains subject only to mechanical naming during a later materialization checkpoint; the semantic fields are fixed here.

The key contains exactly:

1. the authenticated requester application `SessionId` selected by NK; and
2. the exact logical target `DeviceId` selected by NI/NK.

No additional field is authorized.

Specifically excluded from the key:

- requester `DeviceId`;
- requester `UserId`;
- requester `WorkspaceId`;
- PRWC outer request ID;
- PRWM authentication request ID;
- target-side admission `SessionId`;
- `TransportIdentity`;
- endpoint, IP address or port;
- candidate/reachability data;
- task, worker or channel ID;
- wall-clock or monotonic time;
- policy revision/version;
- registry revision/version;
- candidate-publication grant identity;
- provider slot/index;
- process configured-peer identity;
- synthetic operation-generation ID.

The key is authority-consumption identity only.

## 6. Selected state representation

Selected future semantic representation:

```text
ExpectedDeviceSchedulingConsumptionLedger {
    max_records: usize,
    consumed: Vec<ExpectedDeviceSchedulingConsumptionKey>,
}
```

This is a private bounded identity set represented by a `Vec`.

The name `Ledger` denotes process-local terminal bookkeeping only.

It does not imply persistence, durable storage, audit log, database or append-only file.

### 6.1 Entry content

Each terminal entry stores only the exact operation key.

No additional non-authorizing metadata is selected.

The ledger does not retain:

- the full authenticated requester session;
- requester user/workspace identity beyond the selected session key;
- target registry binding;
- policy decision or evaluator;
- requester/rendezvous candidate-publication grant;
- reachability publication;
- transport identity;
- endpoint/candidate state;
- downstream scheduling event/request;
- target-side authentication material;
- execution outcome;
- error reason;
- retry count;
- timestamps.

### 6.2 No lifecycle enum

The ledger has one semantic state per stored key: consumed.

No `Eligible`, `Pending`, `Committed`, `Sent`, `Accepted`, `Failed`, `Retired` or `Removed` variant is selected.

Absence means no committed scheduling-consumption tombstone exists for that exact key.

Presence means terminal consumption has committed.

This absence/presence law is valid only while the existing requester mutex supplies the NK-selected linearization custody.

## 7. Selected finite capacity

The ledger has an explicit finite positive `max_records` bound.

The bound is independent from:

- requester/rendezvous provider `max_records`;
- active-worker count;
- candidate/reachability capacity;
- channel capacity;
- registry size;
- process configured-peer count.

There is no implicit multiplication, minimum, maximum or derivation from another bound.

## 8. Selected authoritative capacity source

NL selects a new dedicated non-secret production process configuration source:

`PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS`

This exact configuration name is scheduling-consumption specific.

It must not alias or fall back to:

- `PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS`;
- `PRW_REMOTE_MAX_ACTIVE_WORKERS`;
- any candidate/reachability configuration;
- CPU count, memory size or runtime auto-sizing;
- a hard-coded production default.

### 8.1 Future source parsing law

A later source-materialization checkpoint may materialize a strict parser/loader with the same configuration discipline already used by Linux bootstrap sources:

- exact environment variable only;
- Unicode required;
- non-empty ASCII decimal digits only;
- no trimming;
- no sign;
- no decimal point;
- no underscore;
- no exponent;
- no alternate variable;
- no fallback;
- no retry or dynamic refresh;
- fail closed on target-`usize` overflow.

The future source may return `usize` and preserve syntactically valid zero unchanged.

The ledger constructor remains the semantic authority that rejects zero capacity.

This separation mirrors the existing requester/rendezvous capacity-source/provider-constructor law without reusing that source or provider state.

## 9. Selected constructor invariant

Selected future semantic constructor shape:

```text
new(max_records: usize) -> Result<Ledger, InvalidCapacity>
```

Exact Rust error-type organization remains a later materialization detail, but the semantic invariant is fixed:

`max_records == 0` -> fail closed before ledger construction.

A successful constructor creates:

- exact configured positive capacity;
- empty consumed-entry vector;
- no preallocated authority entries;
- no registration side effect;
- no policy/registry read;
- no task/channel/runtime side effect.

No default constructor with implicit capacity is selected.

## 10. Selected bounded failure classes

NL selects three stable semantic failure classes for the future state boundary.

### 10.1 `InvalidCapacity`

Construction requested zero ledger capacity.

This is a construction/configuration invariant failure.

No state exists after failure.

### 10.2 `AlreadyConsumed`

The exact requester-session/target key is already present.

This is the canonical duplicate/terminal-consumption failure.

It is not success, idempotent coalescing, retry eligibility or downstream duplicate-active-device classification.

No mutation occurs.

### 10.3 `CapacityExhausted`

The ledger is at its configured finite capacity and the requested key is not already present.

No mutation occurs.

No tombstone is evicted, compacted or replaced.

No older entry is removed to admit a newer operation.

A process with an exhausted terminal ledger fails closed for new distinct scheduling-consumption keys until the process-local authority lifetime ends.

NL does not add automatic restart, persistence rollover or operator remediation behavior.

## 11. Selected duplicate-before-capacity precedence

For one future atomic commit attempt under the requester mutex, evaluation order is fixed:

1. test exact-key presence;
2. if present -> `AlreadyConsumed`;
3. otherwise test `len >= max_records`;
4. if full -> `CapacityExhausted`;
5. otherwise insert exactly one key and return success.

Therefore a duplicate remains classified as `AlreadyConsumed` even when the ledger is otherwise full.

This preserves terminal identity semantics and prevents capacity state from obscuring a known duplicate.

## 12. Selected atomic commit API semantic

Selected future semantic operation:

```text
commit_if_absent(key) -> Result<(), ConsumptionStateError>
```

The method must execute entirely under the NK-selected existing requester mutex.

It performs exactly one synchronous state transition:

`Absent(key) -> Present(key)`

or fails before mutation.

No public/raw membership-query API is required for production authority composition.

No two-step `contains()` then `insert()` authority sequence may escape the mutex boundary.

No reservation/pending token is selected.

No rollback API is selected.

No remove API is selected.

## 13. Linearization point

The successful vector insertion is the exact scheduling-consumption state linearization point.

It remains nested inside the broader NJ/NK authority-mint commit composition after all selected fresh authority gates succeed.

The terminal state must not be inserted:

- during requester/rendezvous registration;
- during policy evaluation alone;
- during provider grant selection alone;
- during candidate publication;
- when a target becomes reachable;
- when a transport endpoint appears;
- when an expected-device request is constructed;
- when a channel send succeeds;
- when target authentication succeeds.

## 14. Capacity failure relative to fresh authority gates

NK selected lazy state creation at the authority-mint commit point.

NL preserves that law.

The future composition may perform fresh requester/target registry currentness, exact policy reauthorization and requester/rendezvous provider uniqueness before entering the terminal insertion step.

If the terminal ledger is full, `CapacityExhausted` rejects the mint before any scheduling grant is returned.

No terminal entry is created for that failed new key.

No downstream event/request construction or send occurs.

## 15. Post-commit failure remains terminal

After a successful terminal insertion:

- grant-carrier construction failure;
- scheduling event/request construction failure;
- expected-request channel closure;
- backpressure/future send failure;
- receiver rejection;
- receiver `DuplicateActiveDevice`;
- AJ registry failure;
- AJ transport identity failure;
- target authentication failure;
- worker failure/completion;
- candidate/reachability change;
- requester response failure;
- shutdown/drop

must not remove the key or authorize remint.

The ledger records authority consumption, not downstream success.

## 16. Candidate-publication provider independence

The future ledger must remain outside `CandidatePublicationRequesterRendezvousRuntimeOwner` and outside `InMemoryRequesterRendezvousAuthorityProvider`.

Candidate provider operations remain unchanged:

- `register_current`;
- `authorize_current_for_publisher`;
- `retire`;
- `remove_retired`.

Provider record cleanup never mutates the terminal scheduling ledger.

Scheduling ledger capacity never changes provider capacity.

Provider capacity exhaustion never means scheduling ledger capacity exhaustion, and vice versa.

## 17. Locking law preserved

NL introduces no synchronization primitive.

The future state is mutable only while held under the existing `SharedRequesterRendezvousAuthority` mutex selected by NK.

The existing selected authority lock order remains:

1. requester mutex;
2. nested shared-current registry/policy read when the authority-mint composition requires it.

No ledger lock exists to invert this order.

No network I/O, channel send, peer accept or response write may occur while the authority locks are held.

## 18. Determinism and bounded complexity

For capacity `N`, duplicate detection performs at most `N` exact key comparisons.

The configured finite bound makes this linear scan memory- and work-bounded.

No randomized hash state or iteration order is authority-relevant.

Vector order is implementation order only and must never select one authority over another.

## 19. No stored authority snapshot

The terminal entry is not evidence that requester/target registry or policy state remains current later.

It proves only that one scheduling authority commit for that exact key already occurred.

Future downstream operations must continue to perform whatever fresh checks their own contracts require.

No later code may treat ledger membership as:

- current registry eligibility;
- current policy authorization;
- transport authorization;
- candidate/reachability authorization;
- target authentication;
- active-session evidence.

## 20. Process lifetime and restart

The ledger is process-local and non-durable as selected by NK.

All entries disappear when the final shared requester-authority owner drops or the process terminates.

NL selects no persistence, restart replay, crash recovery or cross-process deduplication.

A later process instance receives a newly constructed empty ledger with its own explicit positive capacity.

This does not authorize reuse of a previous process's correlation IDs or transport state.

## 21. Alternatives rejected explicitly

NL rejects all of the following:

- unbounded vector/set;
- provider-capacity aliasing;
- worker-capacity aliasing;
- automatic capacity derivation;
- hard-coded fallback production capacity;
- zero-capacity successful construction;
- eviction of oldest tombstone;
- LRU behavior;
- TTL expiration;
- timer-based cleanup;
- removal after successful downstream admission;
- removal after failed downstream admission;
- removal on candidate-publication cleanup;
- retry/remint after send failure;
- duplicate coalescing;
- duplicate-as-success;
- map values containing status or metadata;
- persistent database/etcd ledger;
- raw request/correlation IDs as keys;
- a new operation-generation ID;
- a second scheduling mutex;
- lock-free or atomic-counter authority.

## 22. Selected result

C03e-NL selects:

`BOUNDED_VEC_IDENTITY_SET_SELECTED / REQUESTER_SESSION_ID_PLUS_TARGET_DEVICE_ID_PRIVATE_KEY_SELECTED / DEDICATED_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS_CONFIGURATION_SELECTED / DUPLICATE_BEFORE_CAPACITY_ATOMIC_COMMIT_SELECTED / INVALID_CAPACITY_ALREADY_CONSUMED_AND_CAPACITY_EXHAUSTED_FAILURES_SELECTED / CANDIDATE_PUBLICATION_PROVIDER_STATE_REMAINS_INDEPENDENT / SOURCE_MATERIALIZATION_BLOCKED`

## 23. Selected exact configuration identity

The dedicated future production capacity source is:

`PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS`

This token is non-secret process configuration only.

It is not authorization, registry state, identity, reachability, readiness or scheduling evidence.

## 24. Source-materialization ceiling

C03e-NL authorizes no Rust/source changes.

A later separately gated checkpoint may select the exact source-materialization seam for:

1. the private key;
2. bounded ledger representation;
3. bounded error classes;
4. atomic `commit_if_absent` mutation;
5. dedicated capacity environment constant/parser/loader;
6. constructor/population composition under `SharedRequesterRendezvousAuthority`.

The likely audited source families are:

- `crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs` for shared-owner sibling state and mutation custody;
- `crates/prw-agent/src/linux_bootstrap.rs` for production process configuration source custody.

This checkpoint does not authorize editing either path.

The later seam-selection checkpoint must decide whether dormant source materialization is split across separate one-file checkpoints or can be proven safely as one bounded multi-file materialization.

## 25. Next blocking prerequisite

Selected next boundary:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_STATE_SOURCE_MATERIALIZATION_SEAM_SELECTION`

That checkpoint must remain documentation-only.

It must select:

- exact source path(s);
- exact private type/module custody;
- exact constructor/population boundary;
- exact capacity source layering;
- whether source materialization must be sequenced config-source-first or ledger-first;
- exact test seam;
- the smallest permissible source diff;
- proof that candidate-publication behavior remains byte/semantic-equivalent outside the new dormant state;
- proof that no authority mint, grant carrier, request construction or runtime activation occurs merely because the state type exists.

## 26. Frozen exclusions / STOP

No Rust scheduling-consumption key, ledger, error enum, parser, loader, environment constant, owner field, constructor parameter, mint method, scheduling-authority grant carrier, policy mutation, scheduling event/request construction, expected-request sender/channel, target admission `SessionId`, authentication PRWM request ID, verifier/admission timing, dispatcher migration, listener/runtime activation, task spawn, persistence, timer, retry/reconnect, deployment, merge, PR close, ready-for-review conversion, branch deletion, force update, rebase, squash or history rewrite is authorized by C03e-NL.

Runtime activation: `0%`.

Scheduling authority grant materialization: `0%`.

Expected-device request producer materialization: `0%`.

Keep the future C03e-NL PR draft/open/unmerged after durable checkpoint closure.
