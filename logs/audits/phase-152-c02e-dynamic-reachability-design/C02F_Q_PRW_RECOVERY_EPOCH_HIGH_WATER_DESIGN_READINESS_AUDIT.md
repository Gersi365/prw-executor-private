# Phase 152 C02f-Q — PRW Recovery Epoch / High-Water Design Readiness Audit

Status: `RECOVERY_HIGH_WATER_DESIGN_READINESS_COMPLETE / STALE_SNAPSHOT_FENCE_REUSE_PROBLEM_ISOLATED / SECOND_ONLINE_AUTHORITY_NOT_REQUIRED_BY_PREFERRED_DIRECTION / EXTERNAL_DISASTER_RECOVERY_EPOCH_REGISTER_PREFERRED_FOR_SELECTION_REVIEW / STRUCTURED_U128_EPOCH_SEQUENCE_PREFERRED_FOR_SELECTION_REVIEW / 64_64_BIT_PARTITION_PREFERRED_FOR_SELECTION_REVIEW / EPOCH_REGISTER_MUST_BE_OUTSIDE_ETCD_ROLLBACK_DOMAIN / EPOCH_RESERVATION_MUST_BE_MONOTONIC_CAS / RESERVED_EPOCHS_MAY_BE_SKIPPED_NEVER_REUSED / RESTORED_CLUSTER_MUST_REMAIN FAIL_CLOSED_UNTIL_NEW_EPOCH_ACTIVATED / NORMAL_OPERATION_DOES_NOT_REQUIRE_EXTERNAL_EPOCH_IO / SAME_NAMESPACE_MONOTONICITY_PRESERVED / CROSS_NAMESPACE_NUMERIC_ORDER_NOT_AUTHORITY / FENCE_EXHAUSTION_FAIL_CLOSED / RECOVERY_ATTEMPT_AMBIGUITY_REQUIRES_REOBSERVATION / RECOVERY_PROVIDER_NOT_SELECTED / BIT_PARTITION_NOT_SELECTED / EPOCH_SCOPE_NOT_SELECTED / SCHEMA_ENCODING_NOT_SELECTED / NO_RUNTIME_ACTIVATION / DOCS_ONLY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Frozen C02d head: `857583b25ed1206317641a93fd8f927819c954d8`
C02f-P predecessor head: `69a7c89210bcf49cf05101e965bc3da5710a9880`
C02f-P predecessor tree: `0799368245561529efa4aa32f9d475f4be12a3f9`
Review date: `2026-08-19`

## Purpose

C02f-N proved that a stale etcd snapshot can restore an application-owned live-owner fence below generations that were successfully issued after the snapshot. It also proved that:

- a high-water key stored only inside the same etcd snapshot rolls back with that snapshot;
- `etcd --bump-revision` advances etcd revision lineage, not the application-owned PRW `u128` fence;
- blindly incrementing the restored fence can therefore reuse a generation already granted before disaster.

C02f-Q deepens only this recovery problem.

The objective is to determine whether PRW can preserve the locked `NonZeroU128` fence and stale-snapshot safety **without adding a second online authority to every normal acquisition**.

This checkpoint does not select a recovery provider, fence bit layout or deployment mechanism. It produces a concrete recovery package for explicit architecture selection.

## Inherited fence semantics

The following remain locked:

1. `ReachabilityLiveOwnerFence` is logically a non-zero `u128`;
2. for one exact `DeviceId + TransportIdentity` namespace, every accepted replacement must carry a strictly newer fence than every previously accepted fence for that namespace;
3. fence reuse/rollback is prohibited after restart, failover and disaster restore;
4. clocks, TTLs, lease IDs and etcd revisions do not become the PRW fence;
5. authority ambiguity/unavailability fails closed;
6. stale side effects are rejected by fence at R1-R4 boundaries;
7. external storage encoding is still not selected, though C02f-N prefers fixed 16-byte big-endian encoding for review.

A recovery mechanism must satisfy those rules rather than redefining them.

## Repository evidence relevant to structured fencing

### Current in-memory type

`crates/prw-remote-bridge/src/reachability_live_owner.rs` represents the fence as:

`ReachabilityLiveOwnerFence(NonZeroU128)`.

The type exposes ordering on the full `u128` and does not currently prescribe internal bit fields.

Therefore an epoch/sequence composition can remain representation-compatible with the public logical type if the resulting raw value remains a non-zero `u128` and preserves required ordering.

No source change is made here.

### Current reference authority

`crates/prw-remote-bridge/tests/reachability_live_owner_peer_namespace.rs` uses a test-only `u128 last_issued` and `checked_add(1)`.

That reference model proves:

- later replacement for the same exact peer has a larger fence;
- stale grants become stale;
- independent peer namespaces do not invalidate one another.

Its single global `last_issued` counter is a test implementation detail, not a locked requirement that production must allocate one flat global sequence.

The locked semantic requirement is strict monotonicity for the exact authority namespace.

## The stale-snapshot impossibility for a flat local counter

Assume the production authority uses only an etcd-resident flat fence value.

1. Namespace P is at fence `F`.
2. Snapshot S is captured.
3. Production issues `F+1`, `F+2`, ... `F+n`.
4. Some effect sink observes `F+n`.
5. The cluster is catastrophically lost.
6. Snapshot S is restored.
7. Restored etcd says last fence is `F`.

Without information outside S, the restored authority cannot distinguish:

- world A: no fence after F was ever issued;
- world B: fences through F+n were issued.

Those worlds have identical restored state.

Therefore no deterministic allocator using only the stale snapshot can safely know whether `F+1` is unused.

This is an information problem, not an etcd transaction problem.

At least one monotonic fact outside the rollback domain is required if stale snapshot restoration is supported.

## Two broad solution families

### Family H1 — external absolute high-water updated during normal operation

Maintain a second durable system that tracks every issued fence or an absolute global floor.

Classification: `SAFETY_CAPABLE / ONLINE_SECOND_AUTHORITY_COST / NOT_PREFERRED_INITIAL`.

This can solve recovery, but normal authority mutations may need synchronous coordination with a second system or a carefully designed dual-write protocol.

That expands the live acquisition path and creates a new distributed transaction/ambiguity problem.

C02f-Q therefore does not prefer H1 unless another requirement makes an online second authority necessary.

### Family H2 — externally monotonic recovery epoch + bounded intra-epoch sequence

Classification: `PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`.

The key idea is to use the high bits of the existing PRW `u128` to identify a monotonic recovery epoch and the low bits for ordinary issuance inside that epoch.

The external recovery system then needs to know only the highest epoch ever activated, not every fence issued.

If a disaster restore advances to an epoch numerically greater than every previous epoch, every fence in the new epoch is greater than every fence from all prior epochs, regardless of how stale the restored per-namespace sequence is.

This can keep the external recovery register off the normal acquisition hot path.

## Preferred conceptual fence function

For selection review, define conceptually:

`raw_fence = (recovery_epoch << SEQUENCE_BITS) | sequence`

with constraints:

- `recovery_epoch > 0`;
- `sequence > 0` for an issued fence;
- each exact namespace's sequence increases monotonically inside one epoch;
- no sequence wrap;
- no epoch wrap;
- a later recovery epoch is strictly numerically greater than any earlier epoch because epoch occupies the high-order bits;
- the complete result remains a `NonZeroU128`.

The exact bit partition is not selected by this checkpoint.

## Why high-order epoch bits solve stale snapshot rollback

Suppose an old epoch is E and the low sequence range is bounded by the bit partition.

Every fence in epoch E is less than every valid fence in epoch E+1 because the epoch occupies higher-order bits.

If snapshot S contains `(E, sequence=x)` but production later issued any values `(E, sequence>x)`, then disaster recovery can reserve `E+1` outside the stale snapshot domain.

The first valid new-epoch fence `(E+1, 1)` is numerically greater than **all** possible values in epoch E.

Therefore the system does not need to know the exact post-snapshot low sequence high-water.

It needs only a monotonic external proof that epoch `E+1` has never been used before and is greater than every previously activated epoch.

## Bit-partition candidates

The partition must balance:

- number of possible disaster-recovery epochs;
- number of possible normal acquisitions per namespace inside one epoch;
- simple checked arithmetic/encoding;
- ease of audit and test.

### B1 — 64-bit epoch / 64-bit sequence

Classification: `PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`.

Layout conceptually:

- high 64 bits: non-zero recovery epoch;
- low 64 bits: non-zero intra-epoch sequence.

Capacity:

- up to `2^64 - 1` non-zero epoch values;
- up to `2^64 - 1` non-zero sequence values inside an epoch.

Even at one million acquisitions per second for one namespace, consuming a 64-bit sequence would take roughly 584,000 years.

Advantages:

- symmetric fixed-width fields;
- easy `u64` checked arithmetic;
- simple compose/decompose logic;
- very large margin on both dimensions;
- maps naturally into a fixed 16-byte big-endian full fence if C02f-N F1 is later selected.

Costs:

- formally reduces per-epoch sequence space from 128 bits to 64 bits, though the practical capacity remains enormous for the live-owner use case.

### B2 — 32-bit epoch / 96-bit sequence

Classification: `ELIGIBLE / NOT_PREFERRED`.

Advantages:

- even larger sequence space;
- `2^32 - 1` recovery epochs is already operationally enormous.

Costs:

- 96-bit sequence has no native Rust integer type;
- composition/increment code becomes less direct;
- additional parsing/arithmetic implementation surface with no practical capacity requirement.

### B3 — 48-bit epoch / 80-bit sequence

Classification: `ELIGIBLE / NOT_PREFERRED`.

It offers vast capacity but again introduces non-native integer widths without a demonstrated benefit.

### B4 — 128-bit random epoch/token

Classification: `REJECTED_AS_ORDERING_MECHANISM`.

Randomness can give uniqueness probability but cannot guarantee that a recovery value sorts strictly above every prior fence.

The contract requires deterministic ordering, not probabilistic uniqueness.

### B5 — wall-clock timestamp as epoch

Classification: `REJECTED_AS_PRIMARY_SAFETY AUTHORITY`.

Clock rollback/skew/restore semantics make time unsuitable as the monotonic safety source, and clocks are already excluded by inherited architecture.

## Preferred epoch scope

There are two meaningful scopes.

### E1 — one recovery epoch for the whole PRW live-owner authority domain

Classification: `PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`.

All namespaces in one authority deployment share the same high-order recovery epoch.

Within that epoch, each exact namespace may maintain its own low sequence in its authority record.

Advantages:

- only one external monotonic register per authority domain;
- disaster restoration advances one value before any namespace can reactivate;
- avoids maintaining an external high-water entry for every DeviceId/TransportIdentity pair;
- cleanly invalidates all pre-disaster authority generations in one operation.

Important semantic note:

A globally shared epoch does **not** make fences from different namespaces interchangeable authority.

The exact namespace remains part of every grant/currentness check. Numeric ordering across unrelated namespaces may exist as a representation artifact and must not grant cross-namespace authority.

### E2 — separate external epoch per exact namespace

Classification: `SAFETY_CAPABLE / NOT_PREFERRED`.

This preserves the most local possible ordering domain but creates unbounded external recovery metadata cardinality and a much harder restore procedure.

There is no current requirement justifying that complexity.

## External recovery epoch register requirements

The preferred H2 direction still requires one monotonic fact outside the etcd snapshot rollback domain.

The future register must provide at minimum:

1. durable persistence independent of the etcd snapshot being restored;
2. monotonic compare-and-set or equivalent single-winner allocation;
3. authoritative read after ambiguous allocation result;
4. no rollback to an earlier register value during the lifetime of the PRW authority domain;
5. access control separate from ordinary runtime etcd authority credentials;
6. integrity/audit evidence for recovery operations;
7. failure closed when the register is unavailable or ambiguous during disaster recovery;
8. enough durability/failure separation that one disaster cannot silently roll back both etcd and the epoch register to the same stale point.

The exact technology/provider is not selected.

Examples of technology classes that could later be evaluated include:

- a strongly consistent external metadata store with conditional writes;
- a separately protected recovery database/key;
- an append-only/WORM recovery metadata ledger with an authoritative current value;
- a provider control-plane primitive with proven monotonic CAS semantics.

Naming those classes does not select any service.

## Why this is not a second online authority in normal operation

Under the preferred H2 design:

- normal acquisition/replacement/release uses only etcd and the current epoch stored in the live authority state/domain metadata;
- the external epoch register is not synchronously touched for every fence;
- ordinary member restart/failover inside the healthy cluster does not advance the recovery epoch;
- only disaster restoration/recreation or exceptionally rare deliberate epoch rollover needs the external register.

Therefore the normal live-owner hot path keeps one online consensus authority: etcd.

The recovery register is a disaster-recovery safety dependency, not a per-request authority dependency.

## Required normal-operation state

If H2 is selected later, etcd must retain enough canonical state to know the currently activated epoch.

Conceptually the domain needs:

- current activated recovery epoch E;
- a domain/schema version;
- optionally a recovery activation identifier/status;
- per-namespace authority records containing the full fence or E + sequence information.

The exact etcd key/value layout remains unselected.

The full externally visible/stored fence should remain sufficient to compare grants without requiring effect sinks to query the external epoch register.

## Preferred per-namespace normal allocation

Inside activated epoch E:

1. linearizably read the exact namespace authority record;
2. verify the record belongs to epoch E and is canonical;
3. obtain prior sequence S from the stored fence/record;
4. calculate `S+1` with checked arithmetic;
5. compose `(E, S+1)` into the full `u128`;
6. execute the C02f-N single-record etcd Txn CAS;
7. only unambiguous Txn success grants authority;
8. compare failure re-observes;
9. mutation ambiguity follows attempt-ID reconciliation.

For a namespace first seen in an already active safe epoch, sequence starts from the canonical first non-zero value only under a bootstrap rule that proves no older record in the same epoch exists.

For a restored namespace whose record is from an older epoch, the first post-recovery acquisition can use `(E_new, 1)` because the epoch itself proves the full fence is above every older epoch value.

## Release semantics under H2

Release should retain the last full fence in a tombstoned/released authority record as already preferred by C02f-N.

This gives normal-operation continuity and diagnostics.

It is not needed to reconstruct post-snapshot low high-water once a new epoch is safely activated, but it remains useful to:

- reject stale release;
- continue monotonic allocation within the same epoch;
- detect corrupt or impossible state;
- avoid accidental absence/bootstrap semantics.

## Fresh-cluster bootstrap protocol

A truly first-ever authority deployment also needs a defined initial epoch.

Preferred selection direction:

1. external register is in explicit uninitialized state or zero sentinel;
2. a recovery/bootstrap operator identity atomically reserves epoch 1;
3. etcd domain activation record is initialized to epoch 1;
4. schema/security/topology health is verified;
5. runtime authority is enabled only after both sides agree on the activated epoch;
6. epoch 1 is never returned to uninitialized state.

The exact initialization ceremony is not selected.

## Disaster recovery activation protocol

Preferred selection direction:

### Phase R0 — fail closed

A restored/recreated cluster starts with live-owner runtime activation disabled.

No owner grant, replacement or currentness-sensitive effect may be newly authorized from restored state.

### Phase R1 — restore etcd data

Restore the selected snapshot under the reviewed C02f-P membership topology.

Verify cluster health, TLS/auth/RBAC and application record decodability.

Do not infer that restored fence values are globally current.

### Phase R2 — observe external epoch high-water

Read the authoritative recovery epoch register outside the etcd rollback domain.

Let the highest reserved/activated external epoch be `E_ext`.

### Phase R3 — reserve a strictly newer epoch

Atomically reserve `E_new = E_ext + 1` using checked arithmetic and compare-and-set semantics.

If the operation is rejected because another recovery won, re-observe.

If the mutation result is ambiguous, do not guess; re-read the register and reconcile the recovery attempt.

If the register is unavailable or corrupt, remain fail closed.

### Phase R4 — activate the new epoch inside etcd

Write a canonical domain activation state for `E_new` using an authoritative etcd transaction under recovery/admin credentials.

The old restored per-namespace fences remain historical values in older epochs.

### Phase R5 — validate

Verify:

- etcd domain epoch == E_new;
- external register proves E_new is the highest reserved/activated recovery epoch;
- selected schema version is valid;
- no record contains an epoch above E_new;
- credentials/topology are healthy;
- recovery transition is auditable.

### Phase R6 — permit runtime reactivation

Only after R0-R5 are complete can the normal runtime authority adapter become eligible to issue new fences in E_new.

The first replacement in each restored namespace is numerically greater than all pre-recovery fences because its high-order epoch is newer.

## Reservation versus activation distinction

The external register should treat a reserved epoch as consumed even if cluster activation later fails.

Example:

1. recovery reserves epoch 42;
2. process crashes before etcd activation;
3. next recovery attempt observes 42 as already consumed;
4. it reserves 43 rather than reusing 42.

Skipping an epoch is harmless.

Reusing an epoch after an ambiguous reservation is dangerous.

Therefore the safety rule is:

`RESERVED_EPOCH_MAY_BE_UNUSED / RESERVED_EPOCH_MUST_NEVER_BE_REUSED`

## Ambiguous external epoch mutation

The same indeterminate-commit principle already applied to etcd must apply to the external epoch register.

A timeout after requesting `41 -> 42` cannot be treated as proof that 42 was not reserved.

A stable recovery attempt identifier is preferred so re-observation can distinguish:

- our reservation committed;
- another recovery reserved a newer epoch;
- no reservation committed;
- state remains ambiguous/corrupt.

No blind retry of the same expected value is allowed without authoritative re-observation.

## Multiple simultaneous disaster-recovery operators

The external epoch CAS acts as a serialization point.

If two recovery controllers start simultaneously from epoch E:

- only one may atomically reserve E+1;
- the loser must re-observe and either stop because a recovery is already active or reserve a later epoch under an explicit takeover protocol;
- both must not activate independent restored clusters that can issue fences in the same epoch.

The exact leader/operator takeover policy remains unselected.

## Split-brain disaster recovery prevention

A dangerous scenario is two separately restored etcd clusters both believing they are the authority successor.

The preferred epoch design helps only if cluster activation is tied to the external register.

Requirements:

1. a restored cluster cannot activate solely because it has a locally larger value than its stale snapshot;
2. activation must prove control of a newly reserved external epoch;
3. the recovery system must not authorize two clusters as active for the same epoch;
4. ordinary runtime credentials should not have permission to reserve recovery epochs;
5. operational routing/discovery must move clients only to the selected activated authority domain.

A future deployment design may require an explicit authority-domain activation record/lease outside etcd in addition to epoch history if concurrent disaster sites are supported. That choice remains deferred.

## Normal failover does not advance epoch

Loss/replacement of one etcd member while quorum survives is not a PRW disaster epoch transition.

Reasons:

- the existing etcd consensus history remains authoritative;
- application records retain the actual latest fences;
- replacing a member under C02f-P membership rules does not roll back the logical keyspace;
- advancing the recovery epoch for ordinary member churn would create unnecessary global discontinuities.

The epoch is for authority-lineage rollback boundaries, not machine identity.

## Region failover distinction

If a future topology uses one regional etcd cluster plus snapshot-based recovery in another region, regional disaster reconstruction **does** cross an authority-lineage rollback boundary and therefore needs a new recovery epoch.

If instead a future multi-region voting etcd cluster retains quorum and continues the same consensus history through one region loss, there is no snapshot rollback and therefore no recovery epoch change merely because leadership moved regions.

This cleanly separates:

- consensus failover within one live etcd lineage;
- disaster recovery that creates/restores a new lineage from potentially stale state.

## Sequence allocation scope

### S1 — per exact namespace sequence within a global epoch

Classification: `PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`.

Each `DeviceId + TransportIdentity` record increments only its own low sequence.

Advantages:

- no global hot counter for every authority acquisition;
- exact namespace remains the contention scope;
- etcd transaction contention remains localized;
- global recovery epoch still guarantees post-disaster ordering.

### S2 — one global sequence across all namespaces

Classification: `SAFETY_CAPABLE / NOT_PREFERRED`.

This matches the simple current test reference but creates an unnecessary shared allocation hotspot and couples unrelated namespaces.

Nothing in the locked architecture requires fences from different namespaces to form one authority order.

## Cross-namespace fence comparisons

If S1 is selected, two different namespaces may both have the same numeric fence `(epoch E, sequence 7)`.

This is safe because a live-owner grant is scoped by exact peer identity plus fence.

An effect sink must never compare only the raw fence while discarding the namespace.

The authority invariant is:

`(namespace, fence)`

not:

`fence globally identifies an owner across all namespaces`.

Any future sink API that loses the namespace would require separate review.

## Sequence exhaustion

No wrap is allowed.

If the low sequence reaches its maximum inside an epoch, the authority must fail closed rather than reuse zero or wrap to a smaller value.

With B1 64-bit sequence, this is operationally remote, but correctness must still specify it.

Possible future actions at exhaustion:

- perform a deliberate global epoch rollover through the external register;
- permanently retire the affected authority domain;
- migrate to a future schema.

No automatic wrap is permitted.

## Epoch exhaustion

If the external recovery epoch reaches its maximum representable value, no later epoch can be represented in the selected layout.

The recovery system must fail closed.

No rollback/reuse is permitted.

B1 makes this limit practically unreachable but not semantically ignorable.

## Deliberate epoch rollover without disaster

The same external mechanism could theoretically reserve a new epoch for planned maintenance/schema migration.

Classification: `POSSIBLE / NOT_REQUIRED_FOR_INITIAL DESIGN`.

A planned rollover would need an authority-wide transition that prevents old-epoch grants from continuing to create effects while the new epoch activates.

C02f-Q does not select such a procedure.

## Interaction with fixed 16-byte big-endian encoding

If C02f-N F1 and C02f-Q B1 are later selected:

- bytes 0..8 conceptually encode epoch as big-endian `u64`;
- bytes 8..16 conceptually encode sequence as big-endian `u64`;
- the concatenated 16 bytes are exactly the big-endian encoding of the composed `u128`;
- lexicographic comparison of canonical 16-byte values matches numeric full-fence ordering.

This is a useful compatibility property, not a selection in this checkpoint.

## Interaction with the one-record authority schema

C02f-N prefers one retained versioned authority record per exact namespace.

Under H2/S1, that record can contain the full composed fence.

The domain epoch may also have a separate metadata record because it applies across all namespaces.

Normal acquisition validates that:

- the namespace record is canonical;
- its fence epoch is not greater than the active domain epoch;
- issuance uses the active domain epoch;
- an old-epoch record can only transition to the active newer epoch, never backwards.

Exact record fields remain unselected.

## Corruption/inconsistent epoch handling

The future adapter must fail closed on impossible states such as:

- namespace record epoch > active domain epoch;
- active domain epoch == 0;
- sequence == 0 in a state that claims an issued grant;
- malformed full-fence bytes;
- restored cluster activation record that disagrees with recovery proof;
- external register value lower than an epoch already proven active by trusted evidence;
- epoch/sequence overflow.

These are `RecoveryRequired`/corruption outcomes, not bootstrap opportunities.

## Security and credential separation

C02f-O's credential separation becomes more important here.

Preferred privilege model:

### Ordinary runtime authority identity

May:

- read/write selected live-owner authority keys through the selected etcd RBAC range.

Must not:

- allocate recovery epochs;
- change cluster membership;
- restore snapshots;
- administer etcd auth/RBAC.

### Recovery controller/operator identity

May, under controlled procedure:

- access the external epoch register;
- restore/validate etcd;
- write domain activation metadata;
- perform recovery-only reconciliation.

The exact credential provider remains unselected.

## External register availability requirements

Because the external register is not on the normal hot path, it does not need to meet every normal acquisition latency objective.

It does need:

- strong correctness;
- durable monotonicity;
- availability during disaster recovery;
- independent backup/failure-domain design;
- clear operator observability.

If it is unavailable during a disaster, recovery remains fail closed. That is an availability cost accepted to preserve safety.

## External register durability boundary

The register must not be recoverable only from the same snapshot set as etcd.

Examples of invalid deployment coupling:

- epoch file stored on one etcd member disk;
- epoch key inside the same etcd cluster only;
- epoch metadata copied only inside the same stale etcd backup artifact;
- epoch value derived from an etcd revision in the restored cluster.

The mechanism is only useful if the disaster that rolls back etcd cannot silently roll back the authoritative epoch register to the same old value.

## External register backup/restore rule

The register's own disaster-recovery procedure must preserve monotonicity.

If its platform can restore old snapshots, then merely moving the epoch into another ordinary snapshot-capable database reproduces the same problem at another layer.

Selection review must therefore prove one of:

- the provider itself offers non-rollback monotonic/append-only semantics sufficient for the register;
- its recovery process has an independent immutable high-water/audit source;
- operational custody guarantees a restored value cannot become lower than a previously activated epoch.

The recovery register is small, but its correctness requirement is stronger than ordinary backup retention.

## Comparison with H1 external absolute high-water

H1 must observe every relevant fence issuance if it is to know the absolute latest low-level value after an arbitrary snapshot.

H2 needs to observe only epoch transitions because the bit partition creates a numeric gap above all possible earlier-epoch values.

Therefore H2 materially reduces coupling:

- no cross-system write on every acquisition;
- no normal-path distributed transaction between etcd and external store;
- less latency;
- fewer indeterminate dual-write states;
- one small disaster metadata value rather than per-namespace live high-water state.

This is the principal reason H2 is preferred for review.

## Comparison with operator-supplied absolute floor

C02f-N H3 proposed a human/operator recovery floor.

A structured recovery epoch improves that idea because the operator need not know the exact highest per-namespace fence issued after the snapshot.

The recovery system only needs a reliably monotonic epoch register.

Human memory, timestamp guesses and manual “pick a large number” procedures remain unacceptable.

## Migration concern: existing pre-etcd test fences

No production etcd live-owner fence has been activated yet.

The current live-owner seam and tests are provider-neutral/in-memory references; there is no deployed durable etcd fence history that must be migrated into a new encoding.

Therefore, if a structured epoch fence is explicitly selected before production adapter activation, the architecture can start at its canonical initial epoch without converting existing production authority records.

This reduces migration risk.

## Compatibility with existing public semantics

A structured implementation remains compatible with the current logical API if:

- callers continue to receive an opaque `ReachabilityLiveOwnerFence`;
- callers do not gain authority to manufacture epoch/sequence values;
- full `u128` ordering remains correct;
- namespace identity remains required;
- no caller is allowed to treat epoch or sequence independently as authority.

Exposing epoch/sequence accessors publicly is not required for initial implementation.

## Required tests if selected later

If H2/B1/E1/S1 is selected, implementation validation should cover at minimum:

### Composition/encoding

- `(epoch=1, seq=1)` produces non-zero full fence;
- same epoch, higher sequence -> higher full fence;
- higher epoch, lowest sequence -> higher than lower epoch, maximum sequence;
- no zero epoch for active authority;
- no zero issued sequence;
- sequence overflow rejected;
- epoch overflow rejected;
- 16-byte big-endian roundtrip if F1 is selected.

### Namespace semantics

- same namespace replacement increments low sequence;
- unrelated namespaces can progress independently;
- equal raw fence values across different namespaces do not make grants interchangeable;
- stale release cannot clear newer same-namespace grant.

### Recovery

- restore snapshot from old epoch, reserve new epoch, first new fence > maximum old-epoch fence;
- reserved-but-not-activated epoch is skipped on retry;
- ambiguous epoch CAS requires re-observation;
- external register unavailable -> restored authority stays disabled;
- two concurrent recovery attempts cannot activate the same epoch;
- old restored cluster cannot issue while activation gate is closed;
- namespace record with future epoch -> corruption/recovery-required;
- region/member failover without snapshot rollback does not advance epoch.

### Security

- runtime role cannot mutate epoch register/recovery metadata;
- recovery role cannot accidentally become the ordinary application runtime identity;
- recovery secrets do not appear in logs/evidence.

## Decision candidates for explicit selection

### Q-D1 — fence structure

Recommended: structured recovery epoch + sequence inside the existing `u128`.

Status: `NOT_SELECTED`.

### Q-D2 — bit partition

Recommended: 64-bit epoch / 64-bit sequence.

Status: `NOT_SELECTED`.

### Q-D3 — epoch scope

Recommended: one global recovery epoch for the live-owner authority domain.

Status: `NOT_SELECTED`.

### Q-D4 — sequence scope

Recommended: per exact `DeviceId + TransportIdentity` namespace sequence inside the global epoch.

Status: `NOT_SELECTED`.

### Q-D5 — external register role

Recommended: disaster-recovery-only monotonic epoch register, not normal-request high-water storage.

Status: `NOT_SELECTED`.

### Q-D6 — external register provider

No provider recommendation is selected by this checkpoint. A separate evaluation must prove non-rollback monotonic CAS and independent failure-domain custody.

Status: `UNSELECTED`.

### Q-D7 — recovery activation gate

Recommended: restored cluster cannot enable live-owner runtime until it reserves a newer external epoch and atomically installs/validates that epoch in etcd.

Status: `NOT_SELECTED`.

## Preferred coherent package for selection review

C02f-Q recommends, but does not select:

1. preserve `ReachabilityLiveOwnerFence(NonZeroU128)` as the logical fence type;
2. structure the raw `u128` as high-order recovery epoch + low-order sequence;
3. use 64 high bits for epoch and 64 low bits for sequence;
4. use one recovery epoch per live-owner authority domain;
5. allocate sequence independently per exact `DeviceId + TransportIdentity` namespace;
6. retain full fences in C02f-N-style versioned authority records;
7. keep an externally monotonic epoch register outside the etcd snapshot rollback domain;
8. touch that external register only during first bootstrap, disaster recovery and exceptional planned epoch rollover;
9. consume every reserved epoch permanently, even if activation fails;
10. reconcile ambiguous epoch allocation by authoritative re-observation, never blind retry;
11. restore etcd fail closed, reserve a newer epoch, install/validate it, then reactivate runtime;
12. fail closed on epoch/sequence exhaustion or inconsistent/corrupt recovery metadata.

This package removes the need for a second online per-request authority while solving the stale-snapshot information gap, provided the external register's own non-rollback semantics are proven.

## What remains unresolved after C02f-Q

The largest remaining recovery choice is no longer the logical mechanism; it is **where the external monotonic epoch register lives and how its non-rollback guarantee is proven**.

Still deferred:

- provider/service for external epoch register;
- exact CAS API;
- key/schema for the register;
- exact 64/64 selection;
- exact initial epoch ceremony;
- recovery operator identity/custody;
- concurrent disaster-site takeover semantics;
- etcd domain activation record schema;
- runtime adapter implementation;
- cluster deployment/security selections from C02f-O/P.

## Why production adapter implementation remains blocked

Even with Q's stronger recovery recommendation, implementation would still force unapproved choices for:

- key/value representation;
- 16-byte fence encoding;
- 64/64 structure;
- attempt identity;
- exact CAS guard;
- recovery register provider;
- TLS feature/PKI;
- cluster topology/endpoints.

Therefore C02f-Q does not authorize production adapter source mutation.

## Production byte-stability requirement

C02f-Q is a docs-only recovery-design audit.

It must not modify:

- Cargo manifests;
- `Cargo.lock`;
- production Rust source;
- GitHub workflow behavior;
- etcd cluster resources;
- external recovery resources;
- endpoints;
- credentials;
- runtime/bootstrap behavior.

No build/rustfmt/Clippy/test run is required solely for C02f-Q because executable bytes remain unchanged from the canonically validated C02f-M state.

## Final classification

C02f-Q closes recovery-epoch **design readiness**, not recovery architecture selection.

The material conclusion is:

> PRW can preserve its locked non-zero `u128` fencing model and avoid a second online per-acquisition authority by reserving a monotonic high-order recovery epoch in a durable register outside the etcd snapshot rollback domain, while allocating ordinary per-namespace low-order sequences inside etcd. A 64-bit epoch / 64-bit sequence layout is the preferred review candidate. Disaster restore remains fail closed until a strictly newer epoch has been reserved and activated. The external register provider and the complete package remain unselected.

Final status:

`C02F_Q_RECOVERY_HIGH_WATER_DESIGN_READINESS_COMPLETE / EXTERNAL_DR_EPOCH_REGISTER_PLUS_STRUCTURED_U128_PREFERRED_FOR_SELECTION_REVIEW / NO_RECOVERY_SELECTION / NO_EXTERNAL_PROVIDER_SELECTION / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION / C02D_UNTOUCHED`
