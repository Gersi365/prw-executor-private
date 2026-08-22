# Phase 152 C02f-BO — First-Owner Provider Execution Composition Selection Staging

## Status

Documentation-only architecture-selection checkpoint after validated C02f-BN.

C02f-BO selects the exact provider-execution, reconciliation, evidence-continuity and semantic-mapping boundary for the already-prepared first-owner branch. It does not materialize Rust source, provider I/O, transaction execution, endpoint/client construction, runtime activation, deployment or merge.

This checkpoint is intentionally separate from C02f-AW replacement provider-execution materialization. It does not authorize either provider-I/O source checkpoint.

## Exact prerequisite

Validated C02f-BN:

- branch `phase-152-c02f-bn-first-owner-provider-execution-readiness-staging`;
- head `90efaabd0d31a80360aaee907a421fa526edc191`;
- tree `f4fb30839cccd8720155edca811c5cccb881ecf5`;
- gate `C02F_BN_FIRST_OWNER_PROVIDER_EXECUTION_READINESS_COMPLETE`;
- exact BM merge base;
- one documentation-only commit over BM;
- canonical PRW Rust Validation #868 FULL PASS.

BN proved that the first-owner branch cannot safely reuse the existing replacement AD/AE execution entry points because:

1. the first-owner handoff retains a dedicated `ReachabilityLiveOwnerFirstOwnerTxnPlan` using exact-key etcd `version == 0` creation semantics;
2. `ReachabilityLiveOwnerEtcdStore::execute(...)` accepts only `LiveOwnerTxnPlan` and materializes the replacement/release dual-CAS shape;
3. `execute_acquisition_with_reconciliation(...)` requires a concrete predecessor `LiveOwnerObservation` and reconstructs a replacement plan;
4. authoritative first-owner state is `None`, and C02f-BI forbids manufacturing a synthetic predecessor observation.

## Existing boundaries preserved

C02f-BO preserves without redesign:

- C02f-BI first-owner absence/bootstrap and indeterminate-reconciliation semantics;
- C02f-BK provider-neutral first-owner handoff and create-only transaction plan;
- C02f-BM preparation facade and its `Replacement | FirstOwner | Superseded` prepared evidence;
- C02f-AV pure deterministic semantic-mapper pattern in `prw-remote-bridge`;
- C02f-AW selected replacement execution chain `AS handoff -> AE reconciliation -> AV mapper`;
- one already-created etcd provider context, with endpoint selection and `Client::connect` outside the acquisition layer.

No BO decision permits callers to supply a fence, attempt ID, predecessor, absence assertion, provider outcome, transaction plan, retry budget or semantic grant.

## Selected provider adapter placement

C02f-BO selects the existing private `ReachabilityLiveOwnerEtcdStore` as the owner of first-owner etcd execution.

The future source checkpoint must add the first-owner transaction/reconciliation implementation as a dedicated private submodule beneath the existing `reachability_live_owner_etcd` provider boundary rather than creating a second public store or independent provider context.

The selected shape is conceptually:

```text
ReachabilityLiveOwnerEtcdStore
  -> private first_owner execution/reconciliation module
  -> exact retained ReachabilityLiveOwnerFirstOwnerHandoff
```

The same already-created `KvClient` owned by `ReachabilityLiveOwnerEtcdStore` must execute the first-owner operation. The future implementation must not accept a separately constructed KV client for first-owner execution and must not select endpoints or call `Client::connect`.

The existing general `execute(&LiveOwnerTxnPlan)` method is not widened to accept a union of transaction types. First-owner creation remains a dedicated operation so the `version == 0` protocol cannot be confused with replacement/release dual-CAS semantics.

## Selected first-owner execution input

The future first-owner execution/reconciliation entry point consumes one exact provider-neutral:

`ReachabilityLiveOwnerFirstOwnerHandoff`

prepared by the validated BM path.

It must not accept the transaction plan and allocation as independently pairable public arguments. Consuming the retained handoff preserves the exact committed AQ allocation, exact canonical fence, exact intended `Current` successor and exact create-only transaction as one evidence unit.

The future source may borrow internal pieces from the handoff while executing, but terminal evidence must remain bound to the same logical handoff. No request-controlled reconstruction is permitted.

## Exact provider transaction selected

For one first-owner submission, the future private adapter materializes only the exact BK plan:

1. one compare: exact canonical live-owner key `version == 0`;
2. success branch: exactly one Put of the exact canonical intended `Current` PRWL successor bytes retained by the handoff;
3. failure branch: exactly one default-linearizable Get of that same exact key.

No `mod_revision` compare, predecessor-value compare, lease, TTL, Watch, extra Get, alternate Put, delete, second key or synthetic predecessor belongs to this transaction.

A definitive provider response must be structurally validated against the selected branch shape before it can become terminal evidence.

## Selected terminal provider evidence

C02f-BO selects a dedicated first-owner resolved-evidence capsule rather than reusing `ReachabilityLiveOwnerResolvedMutation`, whose retained plan type is `LiveOwnerTxnPlan`.

The future resolved capsule must retain the exact first-owner handoff or an ownership-equivalent exact retained copy of all of its evidence and expose inspection without public arbitrary construction.

Conceptually the terminal provider outcome is:

```rust
pub enum ReachabilityLiveOwnerFirstOwnerResolvedOutcome {
    Committed,
    CompareFailed(LiveOwnerObservation),
    Superseded(LiveOwnerObservation),
}
```

and the resolved capsule is conceptually:

```rust
pub struct ReachabilityLiveOwnerResolvedFirstOwner {
    handoff: ReachabilityLiveOwnerFirstOwnerHandoff,
    outcome: ReachabilityLiveOwnerFirstOwnerResolvedOutcome,
}
```

Exact source names may be adjusted mechanically in the later source checkpoint, but the semantic distinctions and evidence ownership are fixed by BO.

The resolved evidence constructor remains private to the provider execution/reconciliation implementation. Downstream code may inspect the exact retained evidence and terminal outcome but may not manufacture a resolved first-owner result.

## Why `CompareFailed` and `Superseded` remain distinct

The provider evidence layer must not prematurely collapse provider history into semantic `Contended`.

`CompareFailed(observation)` means the initial or deliberate reissue returned a definitive etcd transaction compare failure with the authoritative exact-key observation from the selected failure branch.

`Superseded(observation)` means an indeterminate submission was followed by a mandatory fresh linearizable re-observation that found another valid exact-key record rather than the exact intended successor.

Both later map to semantic `Contended`, but retaining the distinction preserves auditability and prevents a semantic layer from pretending a re-observed supersession was a definitive transaction response.

## Selected first-owner indeterminate reconciliation

C02f-BO materializes no source, but selects the exact future state machine from the already-validated BI semantics.

For the exact retained first-owner handoff:

1. submit the exact retained create-only plan once;
2. definitive success => `Committed`;
3. definitive compare failure with one valid authoritative exact-key observation => `CompareFailed(observation)` and stop;
4. non-definitive mutation RPC outcome => do not retransmit immediately;
5. perform one fresh default-linearizable exact-key Get for the handoff peer;
6. if the observation equals the exact intended canonical successor retained by the handoff => `Committed`;
7. if another valid exact-key record exists => `Superseded(observation)`;
8. if the exact key remains absent => internal `ProvenNotCommitted`;
9. only `ProvenNotCommitted` authorizes one deliberate reissue of the exact same retained plan;
10. the reissue must preserve the identical key, compare, intended Put bytes, canonical fence and authority attempt ID;
11. definitive reissue success => `Committed`;
12. definitive reissue compare failure => `CompareFailed(observation)` and stop;
13. second non-definitive RPC outcome => perform one final fresh linearizable exact-key Get;
14. exact intended successor => `Committed`;
15. another valid exact-key record => `Superseded(observation)`;
16. exact-key absence => fail closed with reissue exhaustion;
17. no third transaction submission is permitted.

Any malformed record, wrong key, unexpected transaction response shape or provider read failure fails closed. None permits another submission or semantic grant.

## Exact intended-successor equality

A re-observation counts as `Committed` only when the canonical decoded record is exactly the intended successor retained by the first-owner handoff.

The proof must therefore preserve and compare the complete canonical authority record, including:

- exact `PeerConnectivityIdentity`;
- exact `LiveOwnerLifecycle::Current`;
- exact canonical non-zero fence derived from the retained committed AQ allocation;
- exact `AuthorityAttemptId` already embedded in the intended successor.

Peer-only or fence-only equality is insufficient to prove commit of the retained logical mutation.

## Reissue and allocation-consumption rule

The committed AQ allocation retained by the handoff is burned regardless of the later first-owner outcome.

The first-owner reconciliation path may reissue only the exact same create-only plan after authoritative `ProvenNotCommitted`. It must never generate another authority attempt ID, allocate another fence, replace the intended successor, refresh the prepared handoff, or return to BM preparation within the same logical operation.

A later logical acquisition must start with a fresh BM preparation and a fresh sequence allocation attempt.

## Selected provider error boundary

The future first-owner execution/reconciliation implementation uses a dedicated fail-closed error surface for provider/codec/transaction-shape/reconciliation failures that are not terminal provider evidence.

At minimum this includes:

- etcd submit unavailability or an indeterminate outcome that cannot be resolved safely;
- exact-key read unavailability;
- malformed/corrupt/non-canonical observed state;
- unexpected Get cardinality or key mismatch;
- unexpected transaction branch/response shape;
- deterministic contradiction between retained handoff and materialized transaction;
- second authoritative `ProvenNotCommitted` after the one permitted reissue.

Those failures are not `CompareFailed`, are not `Superseded`, and are never semantic `Contended` merely because the provider could not prove a grant.

A later composition maps every such error to:

`ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`

unless an already-validated semantic fence representation conversion alone produces the existing `FenceExhausted` condition.

## Selected first-owner semantic mapper placement

C02f-BO selects a pure deterministic first-owner semantic mapper in `prw-remote-bridge`, parallel to the existing C02f-AV reconciled-acquisition mapper.

The mapper performs no provider I/O, re-observation, transaction execution, retry, randomness generation, allocation, endpoint/client construction or runtime activation.

It accepts only the exact resolved first-owner provider evidence required for deterministic proof. It must not accept a request-controlled peer, fence, attempt ID, provider client, retry signal or semantic result.

The narrow `prw-control-plane::reachability_acquisition_evidence` facade remains the route by which first-owner handoff/transaction evidence is externally nameable. The private `first_owner` implementation module is not published wholesale.

## Mandatory mapper evidence checks

Before interpreting the terminal outcome, the future first-owner mapper must fail closed unless all retained evidence is internally exact:

1. the resolved capsule retains the exact first-owner handoff produced for this logical operation;
2. the retained transaction successor is exactly `LiveOwnerLifecycle::Current`;
3. the successor peer equals the exact peer represented by the retained first-owner handoff;
4. the transaction key is the canonical live-owner key for that exact peer;
5. the success Put value is the canonical encoding of that exact retained successor;
6. the transaction compare remains exact-key `version == 0`;
7. the retained AQ allocation outcome is exactly `Committed`;
8. the successor fence equals the canonical 64/64 fence derived from that exact retained AQ allocation;
9. the retained authority attempt ID is the exact attempt ID embedded in the canonical successor;
10. any `CompareFailed` or `Superseded` observation belongs to the same exact peer and is valid canonical provider evidence.

The mapper must not repair, re-plan or normalize a contradictory capsule.

## Selected terminal semantic mapping

Only after all mandatory evidence checks pass:

- `Committed` -> `ReachabilityLiveOwnerAcquisition::Granted` with the exact retained peer and exact retained non-zero fence;
- `CompareFailed(...)` -> `ReachabilityLiveOwnerAcquisition::Contended`;
- `Superseded(...)` -> `ReachabilityLiveOwnerAcquisition::Contended`;
- evidence contradiction -> `ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`;
- impossible conversion of the retained provider fence into the existing semantic fence representation -> `ReachabilityLiveOwnerAuthorityError::FenceExhausted`.

No provider failure maps to `Granted`. No absent re-observation maps to `Contended` before the selected bounded reconciliation rule has reached a terminal provider result.

## Public-surface selection

The future source checkpoints may expose only the narrow resolved first-owner evidence/outcome types that the bridge semantic mapper must name.

They must not make public:

- raw etcd transaction builders;
- first-owner submit/reobserve helpers;
- provider retry/reissue state;
- mutable provider client internals;
- endpoint/client bootstrap;
- independent constructors for terminal evidence;
- broad private fence-sequence or first-owner modules.

Any public resolved evidence type must remain observation-only outside its provider-owned constructor path.

## Relationship to C02f-AW replacement execution

C02f-BO does not alter the AW-selected replacement chain:

`AS retained handoff -> AE acquisition reconciliation -> AV semantic mapper`

Replacement provider execution and first-owner provider execution remain separate source-materialization checkpoints so each real-provider boundary can be validated independently.

Neither source checkpoint may silently compose the full BM `Replacement | FirstOwner | Superseded` result into `ReachabilityLiveOwnerAsyncAuthority::acquire(peer)`.

## Safe continuation after BO

After BO is canonically validated, a later separately authorized source checkpoint may materialize the dedicated first-owner etcd execution/reconciliation boundary selected here.

That source checkpoint will cross the real provider-I/O execution boundary when invoked and therefore is not authorized by BO itself.

A separate later source checkpoint may materialize the already-selected AW replacement execution composition only under its own provider-I/O authorization.

Only after both branches have validated provider-execution plus evidence-bound semantic mapping may a later documentation-selection checkpoint choose their common `ReachabilityLiveOwnerAsyncAuthority::acquire(peer)` composition.

## Explicit non-activation boundary

C02f-BO does not:

- add or modify Rust source;
- modify Cargo manifests or lockfiles;
- execute an etcd Get, Txn or re-observation;
- invoke AD, AE, AQ, BM or any other provider operation;
- construct or connect an etcd client;
- select endpoints, TLS, authentication, RBAC, credentials, leases, TTL, Watch, users, roles, permissions or cluster membership;
- issue a recovery epoch or contact Spanner;
- initialize PRWF state;
- allocate or reissue a production fence;
- generate an attempt ID;
- materialize first-owner provider execution;
- materialize AW replacement provider execution;
- map live provider evidence at runtime;
- activate `ReachabilityLiveOwnerAsyncAuthority::acquire(peer)`;
- activate runtime/R1-R4 effects;
- deploy;
- merge.

## Validation gate

The C02f-BO gate is:

`C02F_BO_FIRST_OWNER_PROVIDER_EXECUTION_COMPOSITION_SELECTED`

It may be claimed only after canonical executable Rust validation passes on the exact final documentation-only BO head and exact BN ancestry/scope are reverified.
