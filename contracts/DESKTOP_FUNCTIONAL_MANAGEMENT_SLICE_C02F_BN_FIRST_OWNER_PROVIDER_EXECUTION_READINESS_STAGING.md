# Phase 152 C02f-BN — First-Owner Provider Execution Readiness Staging

## Status

Documentation-only readiness checkpoint after validated C02f-BM.

C02f-BN does not select or materialize a new provider executor. It audits the exact provider-execution gap that remains after BM materialized the narrow acquisition-preparation facade, and it preserves the already-selected replacement execution path without duplication.

No Rust source, dependency, lockfile, workflow, Android, Agent, bridge, runtime, provider I/O, deployment or merge mutation is authorized by BN.

## Exact prerequisite

Validated C02f-BM:

- branch `phase-152-c02f-bm-acquisition-handoff-preparation-facade-staging`;
- head `dfa29c54dbdfa23dcfe77c3dc3bf5ac0e0f3c6fb`;
- tree `5d1abbb15da6c726d16394688d6ef886dfb19e24`;
- gate `C02F_BM_ACQUISITION_HANDOFF_PREPARATION_FACADE_MATERIALIZED`;
- canonical Rust #867 FULL PASS;
- canonical Android #442 FULL PASS.

C02f-BM returns only bounded preparation evidence:

```rust
pub enum ReachabilityLiveOwnerPreparedAcquisition {
    Replacement(FenceSequenceLiveOwnerAcquisitionHandoff),
    FirstOwner(ReachabilityLiveOwnerFirstOwnerHandoff),
    Superseded,
}
```

BM itself performs no live-owner mutation execution or semantic grant mapping.

## Already-selected replacement execution path

C02f-AW already selected the replacement provider-execution composition and BN must not redesign it.

The selected replacement evidence chain remains:

`AS retained handoff -> AE acquisition reconciliation -> AV semantic mapper`

The future replacement source composition is bounded to:

1. one already-created mutable `ReachabilityLiveOwnerEtcdStore`;
2. one exact retained `FenceSequenceLiveOwnerAcquisitionHandoff`;
3. exact projection of `handoff.observation()` as AE `before`;
4. exact projection of `handoff.acquisition().transaction().successor()` as AE `successor`;
5. exactly one call to `ReachabilityLiveOwnerEtcdStore::execute_acquisition_with_reconciliation(before, successor)`;
6. exact retention of the original AS handoff for `map_reconciled_live_owner_acquisition`.

AW forbids direct `execute(...)`, replanning, extra provider re-observation, outer retry/reissue loops, reconstructed resolved mutations, bypass of AV evidence equality, and direct manufacturing of semantic outcomes.

AW also explicitly states that materializing that selected source composition crosses the real provider-I/O execution boundary when invoked and requires a separately authorized source checkpoint.

BN therefore treats replacement execution as selected-but-not-yet-materialized and does not alter it.

## First-owner execution gap proven by current source

The first-owner branch cannot safely reuse the existing replacement execution path.

### Dedicated first-owner transaction type

C02f-BK materialized a distinct provider-neutral transaction type:

`ReachabilityLiveOwnerFirstOwnerTxnPlan`

Its selected transaction shape is:

- exact-key etcd `version == 0` compare;
- exactly one canonical `Current` PRWL Put on success;
- exactly one default-linearizable exact-key Get on compare failure.

The retained `ReachabilityLiveOwnerFirstOwnerHandoff` binds one committed AQ allocation to that exact create-only transaction plan.

This is deliberately separate from `LiveOwnerTxnPlan` and from the AS replacement handoff.

### Existing AD execution API mismatch

`ReachabilityLiveOwnerEtcdStore::execute(...)` currently accepts only:

`&LiveOwnerTxnPlan`

and materializes the C02f-AB dual-CAS replacement/release transaction shape using:

- `mod_revision == observed`;
- exact-value equality;
- one success Put;
- one compare-failure linearizable Get.

It does not accept `ReachabilityLiveOwnerFirstOwnerTxnPlan` and does not materialize `version == 0` first-owner creation.

### Existing AE reconciliation API mismatch

`ReachabilityLiveOwnerEtcdStore::execute_acquisition_with_reconciliation(...)` currently requires:

- a concrete `LiveOwnerObservation` predecessor; and
- a `ReachabilityLiveOwnerAuthorityRecord` successor.

Internally it reconstructs the C02f-AB replacement acquisition plan from that predecessor.

The first-owner branch begins from authoritative `None`. C02f-BI explicitly forbids manufacturing a synthetic predecessor observation. Therefore the existing AE replacement entry point is not a valid first-owner execution API.

## Already-selected first-owner execution semantics

C02f-BI already selected the semantic behavior that any future first-owner provider executor/reconciler must preserve:

1. authoritative live-owner `None` remains explicit absence;
2. submit the exact retained create-only `version == 0` plan;
3. definitive success proves the exact intended successor committed;
4. definitive compare failure returns one authoritative exact-key observation and is semantic contention, never grant;
5. an indeterminate submission requires mandatory fresh linearizable exact-key re-observation before any reissue;
6. exact retained intended successor on re-observation => `Committed`;
7. any other valid exact-key record => `Superseded` / semantic `Contended`;
8. exact-key absence => `ProvenNotCommitted`;
9. only `ProvenNotCommitted` may authorize one exact retained-plan reissue;
10. a second indeterminate result is re-observed again;
11. second authoritative `ProvenNotCommitted` => fail-closed reissue exhaustion / semantic `UnavailableOrAmbiguous`;
12. no third transaction submission;
13. the committed AQ sequence/fence is consumed regardless of later contention or execution failure and is never reused.

BN does not change those BI semantics.

## Evidence continuity required before semantic mapping

A future first-owner provider-execution path must retain enough evidence to prove that any semantic result derives from the exact BM-prepared handoff.

At minimum the future terminal provider evidence must remain bound to:

- the exact `ReachabilityLiveOwnerFirstOwnerHandoff` or an exactly validated projection of it;
- the exact committed AQ allocation already retained by that handoff;
- the exact create-only transaction plan;
- the exact intended canonical `Current` successor;
- the exact requested `PeerConnectivityIdentity`;
- the exact canonical non-zero fence;
- the exact authority attempt ID already encoded in the intended successor;
- the terminal provider resolution (`Committed`, definitive contention/supersession, or fail-closed error).

A semantic mapper must not accept an unbound provider outcome that can be paired with a different handoff, peer, fence, successor or transaction.

## Decisions still unresolved after BM

The following are not yet selected and must be resolved by a separate documentation-only selection checkpoint before first-owner executor source is materialized:

1. **Provider adapter placement**
   - extend the existing private `ReachabilityLiveOwnerEtcdStore` with a dedicated first-owner execution/reconciliation entry point; or
   - place the first-owner provider executor in a separate private module that still owns the same already-created KV context.

2. **Terminal provider evidence type**
   - define the exact first-owner resolved evidence capsule and its visibility;
   - preserve transaction/handoff equality strongly enough for later semantic mapping.

3. **Definitive compare-failure representation**
   - retain the authoritative exact-key observation without collapsing it prematurely into a semantic result.

4. **Indeterminate reconciliation implementation seam**
   - materialize BI's `Committed | Superseded | ProvenNotCommitted` classification for the dedicated create-only plan;
   - enforce exactly one retained-plan reissue and no third submit.

5. **First-owner semantic mapper**
   - select the exact evidence-equality checks before mapping terminal provider evidence to `Granted`, `Contended`, `UnavailableOrAmbiguous`, or an already-existing fence-representation exhaustion semantic where applicable.

6. **Composition order relative to AW replacement materialization**
   - replacement and first-owner provider execution must remain independently auditable source checkpoints;
   - neither should silently activate full `ReachabilityLiveOwnerAsyncAuthority::acquire(peer)`.

## Safe continuation sequence

The safe next design step after BN is a separate documentation-only first-owner provider-execution/reconciliation composition selection checkpoint.

Only after that selection is canonically validated may a later source checkpoint materialize the dedicated first-owner etcd transaction/reconciliation path.

Separately, AW's already-selected replacement composition may be materialized only under the separate authorization AW explicitly requires for crossing the provider-I/O execution boundary.

Full semantic `acquire(peer)` remains blocked until both prepared branches have validated provider-execution mappings and a later composition checkpoint explicitly selects their common semantic authority boundary.

## Explicit non-activation boundary

C02f-BN does not:

- add or modify Rust source;
- change Cargo manifests or lockfiles;
- publish private provider modules;
- execute an etcd Get, Txn or re-observation;
- construct or connect an etcd client;
- select endpoints, TLS, authentication, RBAC, credentials, leases, TTL, Watch, users, roles, permissions or cluster membership;
- issue a recovery epoch or initialize PRWF state;
- allocate a production fence;
- generate attempt IDs;
- materialize AW replacement provider execution;
- materialize first-owner provider execution;
- map first-owner evidence to a semantic grant;
- activate `ReachabilityLiveOwnerAsyncAuthority::acquire(peer)`;
- activate runtime/R1-R4 effects;
- deploy;
- merge.

## Readiness gate

The BN readiness gate is:

`C02F_BN_FIRST_OWNER_PROVIDER_EXECUTION_READINESS_COMPLETE`

It may be claimed only after canonical executable Rust validation passes on the exact final documentation-only BN head and exact BM ancestry/scope are reverified.
