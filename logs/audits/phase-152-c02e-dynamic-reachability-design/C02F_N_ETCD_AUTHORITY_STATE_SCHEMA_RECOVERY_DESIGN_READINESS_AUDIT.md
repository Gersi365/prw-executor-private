# Phase 152 C02f-N — etcd Authority State / Schema / Recovery Design Readiness Audit

Status: `DESIGN_READINESS_COMPLETE / ETCD_V3_7_SELECTED / ETCD_CLIENT_0_19_0_MATERIALIZED / EXACT_PEER_NAMESPACE_PRESERVED / SINGLE_RECORD_CAS_MODEL_PREFERRED_FOR_SELECTION_REVIEW / VERSIONED_BINARY_KEY_FRAMING_PREFERRED_FOR_SELECTION_REVIEW / FIXED_WIDTH_BIG_ENDIAN_U128_PREFERRED_FOR_SELECTION_REVIEW / INDETERMINATE_COMMIT_REOBSERVATION_REQUIRED / STALE_RELEASE_MUST_PRESERVE_HIGH_WATER / SAME_SNAPSHOT_HIGH_WATER_INSUFFICIENT / ETCD_REVISION_BUMP_INSUFFICIENT_FOR_PRW_FENCE / RECOVERY_EPOCH_OR_EXTERNAL_HIGH_WATER_DECISION_REQUIRED / SCHEMA_ENCODING_NOT_SELECTED / RECOVERY_MECHANISM_NOT_SELECTED / TLS_PROFILE_DEFERRED / CLUSTER_DEPLOYMENT_DEFERRED / RUNTIME_ACTIVATION_DEFERRED / DOCS_ONLY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Frozen C02d head: `857583b25ed1206317641a93fd8f927819c954d8`
C02f-M predecessor head: `134c8260a02fdf385904e34635c0070286f7ed56`
C02f-M predecessor tree: `57678449f6ef47d04c246a8c1e276a3821555aec`
Review date: `2026-08-19`

## Purpose

C02f-M closed executable dependency materialization for the selected `etcd-client = =0.19.0` Rust dependency while leaving key/value schema, external `u128` fence encoding, transaction mapping, recovery high-water mechanism, TLS profile, deployment topology and runtime activation deferred.

C02f-N does not select any of those deferred architecture choices. It narrows the design space using:

- the already locked PRW identity/fencing semantics;
- the exact active repository source types and existing PRW encoding precedents;
- the selected etcd v3.7 KV/Txn guarantees;
- the selected `etcd-client 0.19.0` compare/transaction API;
- the failure and disaster-recovery semantics documented by etcd v3.7.

The goal is to separate choices that are mechanically constrained from choices that still require explicit architecture approval before production adapter implementation.

## Inherited non-negotiable semantics

The following are inherited and are not reopened:

1. logical live-owner namespace is exactly `DeviceId + TransportIdentity`;
2. IP/port/endpoint/NAT/relay/path are transient location data, never identity;
3. T3 shared control-plane authority placement is selected;
4. etcd v3.7 is selected as the T3 live-owner authority backend;
5. `etcd-client 0.19.0` is selected and dependency-materialized;
6. live-owner fence is `ReachabilityLiveOwnerFence(NonZeroU128)` logically in memory;
7. every accepted replacement must receive a strictly newer non-zero fence for the exact namespace;
8. fence reuse/rollback after restart, failover or restore is prohibited;
9. ambiguous/unavailable/no-quorum authority fails closed;
10. stale release cannot clear or weaken newer authority;
11. Watch, TTL, heartbeat and clocks are not primary currentness authority;
12. R1-R4 effect sinks must reject stale fencing at or atomically with the effect boundary.

## Repository source evidence

### DeviceId

`crates/prw-core/src/lib.rs` currently defines `DeviceId(String)`.

Its constructor rejects only empty or all-whitespace values. It does not impose a restricted path alphabet, delimiter exclusion, hexadecimal form, UUID form, ASCII-only form, maximum length, slash exclusion, percent-encoding rule or NUL exclusion.

Therefore a future etcd key format must not concatenate raw `DeviceId` text with a delimiter and assume that the delimiter is impossible inside the identifier.

### TransportIdentity

`crates/prw-connectivity/src/lib.rs` currently defines:

`TransportIdentity([u8; 32])`

with all-zero rejected and `as_bytes()` exposing the exact 32 bytes.

This gives the storage design a canonical source byte sequence for transport identity, but does not itself select textual hex/base64 or raw-binary persistence representation.

### Existing PRW wire precedent

`crates/prw-remote-bridge/src/reachability_freshness_wire.rs` uses:

- fixed magic/version fields;
- fixed-width integer fields;
- big-endian integer encoding via `to_be_bytes()` / `from_be_bytes()`;
- exact fixed-size raw 32-byte identity/token fields;
- exact-length validation;
- zero reserved-field validation;
- fail-closed decoding for unknown/invalid encodings.

This is a useful repository precedent for a future binary authority record, but it does not authorize reuse of the PRWF format or silently select a live-owner etcd schema.

### Existing durable-state precedent

`crates/prw-remote-bridge/src/reachability_owner.rs` already models durable mutation as expected-current compare-and-commit and treats any ambiguous persistence result as recovery-required.

This precedent supports a live-owner adapter API that distinguishes:

- authoritative commit;
- definite stale compare rejection;
- unavailable/ambiguous mutation outcome;
- invalid/corrupt state.

It also supports preserving one atomic transition rather than a non-atomic read-then-write sequence.

## etcd v3.7 authoritative evidence

Primary references reviewed on `2026-08-19`:

- etcd v3.7 API guarantees: `https://etcd.io/docs/v3.7/learning/api_guarantees/`
- etcd v3.7 API/data model: `https://etcd.io/docs/v3.7/learning/api/`
- etcd v3.7 data model: `https://etcd.io/docs/v3.7/learning/data_model/`
- etcd v3.7 failure modes: `https://etcd.io/docs/v3.7/op-guide/failures/`
- etcd v3.7 disaster recovery: `https://etcd.io/docs/v3.7/op-guide/recovery/`
- etcd v3.7 maintenance: `https://etcd.io/docs/v3.7/op-guide/maintenance/`
- `etcd-client 0.19.0` Rust API docs for `Compare`, `Txn`, `TxnOp` and `Client::txn`.

The relevant provider properties are:

1. KV operations are strictly serializable / linearizable by default;
2. Range becomes potentially stale only when explicitly requested as serializable;
3. one Txn atomically evaluates compares and executes one branch;
4. etcd keys and values are byte strings;
5. etcd store revision is a cluster-wide monotonic signed 64-bit logical counter for the lifetime of that cluster lineage;
6. Watch is ordered but not a linearizable currentness authority;
7. lease expiry is wall-clock TTL behavior;
8. a client can be uncertain whether a mutating request committed after timeout/network disruption;
9. majority loss stops consensus progress and therefore writes;
10. snapshot restore may move apparent keyspace revision backwards unless operational revision bumping is used;
11. revision bumping changes etcd revision behavior, not application-owned values inside the restored snapshot.

## Key-space design problem

etcd exposes a flat lexicographically ordered binary key space.

PRW needs an injective mapping from the logical pair:

`(DeviceId UTF-8 bytes, TransportIdentity[32])`

to exactly one physical authority key.

The mapping must be:

- deterministic;
- unambiguous;
- collision-free for all values currently admitted by PRW types;
- versionable;
- recoverably decodable or at least diagnostically inspectable;
- independent of IP/port/endpoint;
- stable across host/process restart;
- non-lossy with respect to exact logical identity.

### Unsafe naive concatenation

A shape such as:

`/prw/live-owner/<DeviceId>/<TransportIdentity>`

is not currently safe unless it also specifies a canonical escaping/length rule because `DeviceId` has no source-level delimiter restrictions.

This audit therefore rejects unframed delimiter concatenation as `NOT_READY_FOR_SELECTION`.

### Candidate K1 — versioned binary framed key

Classification: `PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`.

Conceptual shape:

- fixed product/domain prefix bytes;
- key-schema version;
- fixed-width big-endian length of the UTF-8 DeviceId byte sequence;
- exact DeviceId bytes;
- exact 32 TransportIdentity bytes.

Properties:

- injective without assuming a restricted DeviceId alphabet;
- fixed transport length needs no delimiter;
- explicit schema version permits future migration;
- direct mapping from current source types;
- binary key bytes are natively supported by etcd.

Open choices that remain selection work:

- exact prefix bytes;
- exact schema version field width;
- DeviceId length field width and maximum admitted encoded length;
- whether a terminal checksum/type marker is useful;
- whether operational tooling requires a separate printable diagnostic renderer.

### Candidate K2 — canonical escaped text key

Classification: `ELIGIBLE / NOT_SELECTED`.

A printable key could encode DeviceId using a fully specified escaping or base encoding and TransportIdentity as lower-case hex/base64url.

Benefits:

- easier manual inspection with etcdctl;
- easier prefix diagnostics.

Costs/risks:

- must define one canonical escaping/base alphabet;
- must reject alternate spellings for the same logical bytes;
- longer keys;
- more parser surface;
- accidental use of non-canonical forms can split one logical namespace into multiple physical keys.

### Candidate K3 — hash-derived key identity

Classification: `NOT_RECOMMENDED_FOR_INITIAL_SELECTION`.

Hashing DeviceId/TransportIdentity can produce compact printable keys but introduces collision/domain-separation and diagnostic reversibility concerns without solving a current scale problem.

The source types already provide exact bytes that can be framed without hashing.

## Value-record design problem

A live-owner record must carry enough information to:

- prove the exact namespace binding;
- expose the current PRW fence;
- distinguish current ownership state from released/tombstoned state;
- support stale-release CAS;
- support indeterminate-commit reconciliation;
- reject malformed/non-canonical state;
- permit schema evolution;
- preserve high-water history rather than resetting it by deletion.

### Single-record state versus split current/high-water keys

#### Candidate V1 — one versioned authority record per exact namespace

Classification: `PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`.

One physical key carries the complete currently authoritative state required for CAS, including at minimum conceptually:

- record magic/domain marker;
- record schema version;
- logical fence value;
- current/released lifecycle discriminator;
- one authority-attempt/owner identifier sufficient to reconcile an ambiguous mutation;
- reserved/versioning room with canonical zero/known-value rules.

The exact owner/attempt token type and encoding remain unselected.

Benefits:

- one etcd Txn compare can protect the entire state;
- one Put atomically replaces all authority fields;
- stale release can compare the exact prior state before writing a released successor;
- no torn state between separate current-owner and local high-water keys;
- easier corruption validation.

This does **not** solve disaster-recovery rollback by itself because a snapshot can restore the entire record to an older fence.

#### Candidate V2 — separate owner-state key + same-cluster high-water key

Classification: `ELIGIBLE_FOR_NORMAL_OPERATION / INSUFFICIENT_FOR_DISASTER_RECOVERY_HIGH_WATER`.

Both keys could be updated atomically in one etcd Txn during normal operation.

However, when both keys are contained in the same etcd snapshot, restoring an old snapshot restores both to their old values.

Therefore V2 does not satisfy the already locked recovery guarantee unless paired with a high-water source outside the rollback domain or an equivalent recovery epoch mechanism.

## Fence external encoding analysis

The logical fence is a native unsigned `u128` with total numeric ordering.

### Candidate F1 — fixed 16-byte unsigned big-endian

Classification: `PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`.

Properties:

- exact one-to-one representation for every `u128`;
- zero can be rejected on decode;
- fixed width avoids alternate textual spellings;
- big-endian byte ordering preserves unsigned numeric lexical ordering among equal-length values;
- aligns with existing PRW network-order integer precedent;
- simple checked `u128::from_be_bytes` / `to_be_bytes` mapping;
- no decimal parsing or signed-conversion risk.

The fact that F1 is preferred does not authorize using etcd value-byte ordering as the fence allocator. The allocator still must obey the PRW state machine and checked arithmetic.

### Candidate F2 — canonical decimal UTF-8

Classification: `ELIGIBLE / NOT_PREFERRED`.

It is human-readable but requires canonical no-leading-zero rules and numeric parsing; ordinary lexicographic ordering of variable-width decimal strings does not preserve numeric ordering.

### Candidate F3 — little-endian fixed 16 bytes

Classification: `ELIGIBLE_FOR_ROUNDTRIP / NOT_PREFERRED`.

It preserves the value on roundtrip but does not preserve ordinary lexicographic numeric ordering and conflicts with the existing PRW big-endian wire precedent.

### Candidate F4 — etcd revision/version/lease ID as the fence

Classification: `REJECTED_BY_INHERITED_ARCHITECTURE`.

These are provider metadata with different width/lifecycle semantics and are already prohibited from silently replacing the PRW logical `u128` fence.

## Normal-operation acquisition mapping

The following transaction shape is technically compatible with locked semantics and is the preferred mapping for later selection review. It remains `NOT_IMPLEMENTED / NOT_SELECTED` in this checkpoint.

### Existing namespace with valid authority record

1. perform an authoritative linearizable Get of the exact key;
2. decode and validate the complete record;
3. reject corruption or impossible state as fail-closed;
4. calculate `next_fence = current_fence.checked_add(1)`;
5. fail closed on exhaustion;
6. construct a replacement record containing the next fence and a new attempt/owner identity;
7. execute one etcd Txn whose compare proves the previously observed record is still current;
8. on compare success, Put the complete replacement record in the success branch;
9. on compare failure, classify as definite contention/stale observation and re-observe before any new attempt;
10. only an unambiguous successful Txn response can directly grant effect authority.

### Choice of compare operand

Two etcd compare strategies are mechanically viable:

- exact `Compare::value(..., Equal, prior_record_bytes)`;
- `Compare::mod_revision(..., Equal, prior_mod_revision)` after decoding the corresponding authoritative record.

Both can prevent lost-update replacement when used correctly.

Using `mod_revision` as a CAS guard does **not** redefine the etcd revision as the PRW fence; it is merely provider-native concurrency metadata guarding the application-owned record.

Exact value comparison additionally validates that all authoritative bytes remain exactly as observed, while mod-revision comparison can produce a smaller compare payload and explicitly identifies the exact observed version.

The exact compare target remains deferred for transaction-mapping selection.

### Absent key

Absence must not automatically mean `fence = 1` is safe for an established namespace.

For an already-established/recovering lifecycle, absent state is `RECOVERY_REQUIRED` unless a separate bootstrap authority proves this is the first-ever grant for the exact namespace under a safe high-water regime.

This follows the existing PRW rule that storage absence must not silently manufacture authority.

## Release mapping

Deleting the only authority record is not preferred because deletion discards the immediately visible application high-water value and makes accidental `version == 0` bootstrap logic dangerous.

Preferred selection direction:

1. read/hold the exact current state expected by the releasing owner;
2. Txn-compare the exact current fence/owner state or its exact mod revision;
3. on success, write a `Released/Tombstoned` successor record that **retains the last issued fence**;
4. do not decrement/rebase the fence;
5. do not create a new lower generation on later acquisition;
6. if compare fails, stale release becomes a definite no-op/rejection;
7. ambiguity fails closed and requires authoritative re-observation.

This preserves normal-operation high-water history inside the authority record and directly satisfies stale-release isolation.

It still does not solve old-snapshot rollback.

## Indeterminate transaction outcome protocol

etcd explicitly documents that a client may be uncertain whether an operation completed after timeout or network disruption.

Therefore a mutating `etcd-client` error cannot be blindly mapped to `NotCommitted`.

The future adapter must use a stable attempt identity in the proposed replacement record and reconcile as follows:

1. mutation request returns an outcome that is not definitively authoritative;
2. caller immediately loses permission to infer Current from the request result;
3. perform a new authoritative linearizable Get when backend authority becomes available;
4. if the exact committed state contains the same attempt identity and intended fence/state, classify the original mutation as committed;
5. if state is definitively different/newer, classify the original attempt as not current/stale;
6. if state is absent, corrupt, older in an impossible way, or still unavailable, remain `RecoveryRequired`;
7. do not issue a conflicting newer transition merely as a blind transport retry.

The exact attempt/owner token type remains a deferred schema decision.

## Watch and cache role

Watch may be used later to accelerate stale-owner invalidation or cache refresh.

It must not grant authority because etcd does not guarantee Watch linearizability.

The safe pattern is:

- Watch/cache = advisory invalidation/observation acceleration;
- linearizable KV read/Txn = authority;
- effect-boundary fence rejection = final stale-effect barrier.

A watch disconnect, compaction event or delayed event cannot make cached ownership authoritative.

## Lease / TTL role

Lease expiry is wall-clock TTL behavior and cannot be the primary safety transition.

This audit therefore does not recommend attaching live-owner safety solely to automatic lease deletion.

A later liveness design may use leases/heartbeats for hints or failure detection only if:

- authoritative replacement still passes the locked Txn/CAS rules;
- stale sink fencing remains enforced;
- lease expiry does not permit fence rollback/reuse;
- authority ambiguity remains fail closed.

Exact lease usage remains deferred.

## Disaster-recovery high-water proof

This is the principal unresolved blocker after C02f-N.

Assume:

1. snapshot S is taken when exact namespace P has last issued fence `F`;
2. production continues and successfully issues `F+1 ... F+n`;
3. those newer fences can reach effect sinks;
4. cluster later suffers disaster and only snapshot S is available;
5. restoring S restores application value `F`.

If the restored system simply increments the restored record, it can issue `F+1` again.

That violates the locked permanent non-reuse rule because an old effect sink may already have observed `F+n`.

### Same-cluster high-water key does not solve this

If a second high-water key is stored in the same etcd snapshot, S restores that key to `F` as well.

Atomic normal-operation updates are useful but do not create an independent disaster-recovery monotonic source.

Classification: `INSUFFICIENT_AS_SOLE_RECOVERY_PROOF`.

### etcd `--bump-revision` does not solve PRW fence rollback

etcd v3.7 recovery supports revision bumping and compaction marking so clients/watchers do not observe a revision lineage moving backwards.

This changes etcd's internal store revision lineage after restore.

It does not rewrite the application-owned `u128` fence value stored inside an old snapshot and does not prove how large a PRW fence may have been issued after that snapshot.

Classification: `USEFUL_ETCD_RECOVERY_TOOL / INSUFFICIENT_AS_PRW_FENCE_HIGH_WATER_PROOF`.

## Recovery mechanism candidates

### H1 — independent durable monotonic high-water service/store

Classification: `SAFETY_CAPABLE / ARCHITECTURE_EXPANDING / NOT_SELECTED`.

Persist a monotonic floor in a durability domain that is not rolled back with the etcd authority snapshot. Recovery reconciles restored etcd state against that floor and allocates strictly above it before reactivation.

Advantages:

- direct safety proof;
- conceptually simple.

Costs:

- introduces a second durable authority dependency;
- deployment/availability/trust semantics must be selected;
- may partially undermine the simplicity gained by selecting etcd as the shared authority backend.

Requires explicit architecture approval.

### H2 — PRW recovery epoch encoded into the logical u128

Classification: `PROMISING_FOR_SELECTION_REVIEW / NOT_SELECTED`.

Conceptually partition the logical `u128` into a recovery epoch plus an intra-epoch sequence, for example an ordered high-order epoch and lower-order counter.

After a disaster restore, production authority may not resume until an independently monotonic epoch is advanced above every prior epoch. Every post-recovery fence then compares numerically above every pre-recovery fence regardless of the restored lower-order sequence.

Advantages:

- preserves the already locked external `u128` type;
- makes recovery invalidation explicit in the fence ordering itself;
- can avoid per-namespace external high-water storage if one safe authority epoch covers the authority domain.

Costs/open questions:

- exact bit partition is a new architecture decision;
- epoch source/custody must itself be monotonic and survive etcd snapshot rollback;
- epoch exhaustion and sequence exhaustion rules must be specified;
- scope must be decided: per cluster/domain versus per namespace;
- changing from a conceptually flat generation to structured generation must be reviewed against all existing tests/contracts.

A random epoch is not sufficient because the fence contract requires strict ordering, not probabilistic uniqueness.

A wall-clock epoch is not sufficient as primary authority because clocks are already excluded as the safety source.

### H3 — disaster restore prohibited until operator-supplied monotonic recovery floor is proven

Classification: `SAFETY_CAPABLE / OPERATIONALLY_STRICT / NOT_SELECTED`.

The system can remain permanently fail closed after stale snapshot restore until an operator supplies auditable evidence/metadata that establishes a floor strictly above every previously issued fence, then writes a new safe state.

This minimizes automatic machinery but moves critical correctness into a recovery ceremony/runbook.

It still requires a durable source of truth for the supplied floor/epoch; human memory or timestamp guessing is not sufficient.

### H4 — use etcd cluster revision as the PRW fence

Classification: `REJECTED`.

This conflicts with the existing explicit PRW-owned `u128` fence lock and does not automatically survive cluster restore/recreation with the required application semantics.

### H5 — same-snapshot high-water key only

Classification: `REJECTED_AS_SOLE_DISASTER_RECOVERY_MECHANISM`.

Useful for normal operation, insufficient against stale snapshot restore.

## Preferred design package for explicit selection review

This audit recommends, but does not select, the following coherent package:

1. one versioned binary authority key per exact `DeviceId + TransportIdentity` namespace using injective length framing;
2. one versioned binary authority record per namespace;
3. fixed-width 16-byte big-endian external fence encoding;
4. authority record retained across release/tombstone so normal-operation high-water is never reset by deletion;
5. linearizable Get plus single etcd Txn CAS for acquisition/replacement/release;
6. either exact-value or mod-revision compare as the provider-native CAS guard, without treating revision as the PRW fence;
7. stable attempt/owner identity in the record for ambiguous-commit reconciliation;
8. no serializable read for authority currentness;
9. Watch only as advisory invalidation;
10. TTL/lease only as optional liveness support, never safety authority;
11. explicit recovery epoch or independent monotonic high-water mechanism before stale-snapshot activation;
12. fail-closed recovery when that high-water proof is unavailable.

Items 1-7 and 11 contain still-deferred representation/architecture decisions and therefore remain `NOT_SELECTED`.

## Why adapter implementation must not start yet

The crate dependency is available, but writing the production etcd adapter before schema/recovery selection would force implicit choices in source code for:

- key framing;
- value record versioning;
- `u128` bytes;
- owner/attempt token;
- stale-release representation;
- CAS compare target;
- recovery epoch/high-water mechanism.

That would violate C02f-J/C02f-M deferment boundaries and risk making an implementation detail the accidental architecture.

Therefore production adapter mutation remains blocked until the representation/recovery package is explicitly selected.

## Decisions that can be made independently later

The next explicit architecture review can separate the following decisions rather than approving one monolithic schema:

### D1 — key encoding

Recommended: `K1 versioned binary length-framed namespace key`.

### D2 — fence encoding

Recommended: `F1 fixed 16-byte big-endian u128`.

### D3 — authority record shape

Recommended: `V1 one versioned record retained across release/tombstone`.

### D4 — CAS guard

Recommended shortlist: exact prior value or exact prior mod revision; both preserve application-owned fence semantics.

### D5 — ambiguous mutation identity

Need explicit owner/attempt identifier type and generation rule.

### D6 — disaster-recovery high-water

Recommended shortlist: H2 recovery epoch if its independent monotonic custody can be proven; otherwise H1 independent high-water store/domain.

D6 is the highest-risk remaining decision.

## Production byte-stability requirement

C02f-N is a design-readiness audit only.

It must not modify:

- Cargo manifests;
- `Cargo.lock`;
- Rust production source;
- GitHub workflow behavior;
- endpoints;
- credentials;
- TLS features;
- cluster deployment;
- runtime/bootstrap behavior.

No build/rustfmt/Clippy/test workflow is required solely for this docs-only audit because executable bytes remain unchanged from the canonically validated C02f-M state.

## Locked conclusion of this audit

C02f-N closes the schema/recovery **readiness analysis**, not the schema/recovery selection.

The material conclusions are:

- naive delimiter-based DeviceId keys are unsafe under the current source type;
- versioned injective binary key framing is preferred for selection review;
- fixed 16-byte big-endian `u128` is preferred for selection review;
- one retained versioned authority record is preferred over delete/recreate state;
- etcd Txn can implement the locked atomic CAS state machine;
- exact-value or mod-revision compare can guard state without redefining the PRW fence;
- indeterminate mutations require linearizable re-observation and stable attempt identity;
- same-cluster high-water keys do not solve stale-snapshot rollback;
- etcd revision bump does not solve PRW logical fence rollback;
- a separately monotonic recovery epoch/high-water proof is required before stale-snapshot reactivation;
- production etcd adapter implementation remains blocked pending those explicit selections.

Final classification:

`C02F_N_DESIGN_READINESS_COMPLETE / REPRESENTATION_RECOMMENDATIONS_READY / RECOVERY_HIGH_WATER_ARCHITECTURE_DECISION_REQUIRED / NO_SCHEMA_SELECTION / NO_RUNTIME_ACTIVATION / C02D_UNTOUCHED`
