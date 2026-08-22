# Phase 152 C02f-BI — First-Owner Live-Owner Bootstrap Semantics Staging

## Status

Documentation-only selection checkpoint after validated C02f-BH.

C02f-BI selects the provider-neutral semantics for creating the first retained `PRWL` live-owner record when one exact peer key is authoritatively absent. It does not add Rust source, perform provider I/O, generate attempt IDs, allocate a fence, initialize or issue a recovery epoch, construct a runtime, activate R1-R4 effects, deploy, or merge anything.

## Exact prerequisite

The exact validated prerequisite is C02f-BH:

- head `23f6a478a90639ff33f37c0463d79357d23b40cf`;
- tree `e3ddb68aa594768f6208173900cf0cfe2422e785`;
- gate `C02F_BH_ATTEMPT_ID_GENERATION_OWNERSHIP_SELECTED`.

C02f-BG remains authoritative for broader acquisition readiness, and C02f-BH remains authoritative for attempt-ID generation ownership/domain separation.

## Existing boundary that creates the gap

The current C02f-AB `plan_acquisition(...)` is replacement-only. It requires an existing `LiveOwnerObservation` and validates a strictly newer successor against that predecessor.

The real C02f-AD store returns `Option<LiveOwnerObservation>` from a linearizable exact-key Get. `None` is therefore a real authoritative provider observation, not a malformed `LiveOwnerObservation`.

C02f-BI explicitly forbids manufacturing a synthetic predecessor, revision, value, fence, lifecycle or attempt ID when the exact key is absent.

## Existing repository precedent for absent-key CAS

C02f-AM/C02f-AN already materialize the repository's canonical absent-key initialization shape for the PRWF fence-sequence head:

1. exact-key linearizable read may return `None`;
2. deterministic plan retains `predecessor = None`;
3. mutation compare is etcd key `version == 0`;
4. success branch is exactly one canonical `Put`;
5. failure branch is exactly one default-linearizable exact-key `Get`;
6. an indeterminate submission requires fresh linearizable re-observation before any reissue;
7. an absent fresh re-observation proves non-commit only for the retained create plan.

C02f-BI selects the same provider-level absence/CAS principle for first live-owner creation rather than inventing another absence mechanism.

## Selected entry condition

Normal acquisition preparation starts with one exact linearizable live-owner Get for the requested `PeerConnectivityIdentity`.

- `Some(observation)` => use the already-materialized replacement path; first-owner bootstrap is not eligible.
- `None` => bootstrap may be planned only if all other normal-acquisition prerequisites are already authoritative and valid.

The bootstrap path must not reinterpret malformed provider state, read failure, cardinality mismatch, key mismatch or decode failure as absence.

## Required upstream authority before bootstrap

An absent live-owner key does not authorize local fence creation.

Before one first-owner bootstrap plan may exist, the normal acquisition flow must already hold:

1. an initialized PRWF fence-sequence head for the surrounding valid recovery epoch;
2. one exact C02f-AQ sequence allocation resolved `Committed`;
3. the canonical non-zero live-owner fence composed from that committed allocation as `(epoch << 64) | sequence`;
4. one fresh `AuthorityAttemptId` generated under the C02f-BH ownership/domain rules;
5. the exact requested `PeerConnectivityIdentity` supplied by the semantic authority call.

Normal bootstrap must not issue a recovery epoch, initialize a missing PRWF head, or manufacture a sequence allocation as a fallback.

## Selected bootstrap successor

The only permitted intended first-owner record is one canonical C02f-Z `ReachabilityLiveOwnerAuthorityRecord::current(...)` containing:

- the exact requested peer;
- the exact canonical fence from the committed AQ allocation;
- the fresh C02f-BH live-owner `AuthorityAttemptId`;
- lifecycle exactly `Current`.

No request-controlled fence, epoch, sequence, attempt ID, record bytes or transaction plan is admitted.

## Dedicated bootstrap plan

C02f-BI selects a dedicated provider-neutral bootstrap plan rather than weakening or overloading the existing two-CAS `LiveOwnerTxnPlan`.

The future bootstrap plan must retain at least:

- exact canonical live-owner key;
- exact canonical intended `Current` successor record and encoded value;
- exact committed AQ allocation evidence authorizing the fence;
- absence provenance (`predecessor = None` or an equivalently explicit create-only marker);
- exactly one absence compare;
- exactly one success Put;
- exactly one failure-branch linearizable Get.

The selected compare is:

`exact live-owner key version == 0`

The success branch is:

`Put(exact key, exact canonical intended successor bytes)`

The failure branch is:

`LinearizableGet(exact key)`

No mod-revision compare, synthetic expected value, delete, lease, TTL, Watch, range request or second mutation is permitted in the bootstrap transaction.

## Why bootstrap is separate from AS

C02f-AS `FenceSequenceLiveOwnerAcquisitionHandoff` deliberately retains a real `LiveOwnerObservation` and validates it by replaying replacement planning.

A first-owner path has no predecessor observation to retain or replay.

C02f-BI therefore selects a distinct first-owner evidence capsule/plan boundary rather than making `LiveOwnerObservation` optional inside the existing AS type or fabricating a placeholder observation.

The existing AS/AW/AE/AV replacement path remains unchanged.

## Definitive transaction classification

For one retained bootstrap plan:

### Compare success

If etcd returns a definitive successful Txn response with exactly the selected Put response, the exact intended first-owner record is `Committed`.

Only this committed result may map directly to semantic `Granted` from a definitive transaction response.

The semantic grant must use the exact peer and fence retained in the intended canonical successor.

### Compare failure

If `version == 0` fails, the failure-branch Get must return exactly one valid record for the exact key.

A valid exact-key record means another writer established live-owner state before this retained create transaction could commit. This is a definitive non-grant result and maps to semantic `Contended`.

Even if the returned record bytes happen to equal the retained intended successor, a definitive compare failure does not prove that this retained transaction committed and must not map to `Granted`.

If a definitive compare-failure branch returns no key, another key, malformed state or impossible cardinality, the result is contradictory and fails closed as `UnavailableOrAmbiguous` at the semantic boundary.

## Indeterminate submission reconciliation

An indeterminate bootstrap Txn result never authorizes immediate retransmission.

The exact key must first be re-read through one fresh linearizable Get and classified against the exact retained create plan.

Selected classifications are:

1. `Committed` — the fresh observation is exactly the retained intended canonical successor, including peer, lifecycle, fence and authority-attempt ID;
2. `Superseded` — a valid exact-key record exists but is not the exact retained intended successor;
3. `ProvenNotCommitted` — the exact key remains absent;
4. contradiction/error — malformed, cross-binding or otherwise impossible provider state.

`Committed` may map to semantic `Granted` only while the exact observed retained successor is still `Current`.

`Superseded` maps to semantic `Contended`, never to a grant.

Contradiction/provider failure maps fail-closed to `UnavailableOrAmbiguous`.

## Bounded reissue policy

C02f-BI adopts the existing C02f-AE bounded mutation discipline.

Only `ProvenNotCommitted` after the first indeterminate submission permits one deliberate reissue.

The reissue must use the exact same retained bootstrap plan, including:

- same exact key;
- same intended successor bytes;
- same committed allocation/fence;
- same `AuthorityAttemptId`;
- same `version == 0` compare.

No new fence, allocation, attempt ID or replanning is permitted for that same logical bootstrap attempt.

A second indeterminate result is re-observed again but can never cause a third transaction submission.

If that second re-observation again proves non-commit, the one-reissue budget is exhausted and the operation fails closed as `UnavailableOrAmbiguous`, consistent with the existing C02f-AE `ReissueLimitReached` policy.

## Fence monotonicity and stale-owner safety

First-owner creation does not weaken the global fencing invariant.

The bootstrap fence still comes exclusively from the already-authoritative PRWF recovery-epoch/sequence machinery. No local zero-based or per-key counter is introduced.

The normal live-owner protocol releases by writing `Released` state rather than deleting the key. Bootstrap is therefore an absent-key create path, not a normal reacquisition path for released owners.

If an exact live-owner key is present in either `Current` or `Released` lifecycle, acquisition remains on the existing replacement path and must satisfy the existing strict newer-fence checks.

## Semantic mapping

The future first-owner semantic mapper must be evidence-bound and fail closed:

- exact bootstrap `Committed` => `ReachabilityLiveOwnerAcquisition::Granted(exact peer/fence)`;
- definitive compare failure with valid exact-key state => `Contended`;
- reconciled `Superseded` => `Contended`;
- provider error, evidence mismatch, contradictory state, exhausted reissue bound or malformed context => `ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`;
- fence representation failure => existing `FenceExhausted` where applicable.

No local inference may manufacture `Granted` from absence alone.

## Public-surface boundary

The semantic caller remains:

`acquire(peer)`

It must not gain request fields for bootstrap mode, absence assertions, fence, epoch, sequence, attempt ID, transaction plan or retry policy.

The future narrow control-plane acquisition-preparation facade decides internally between replacement and first-owner paths after the authoritative live-owner read.

## Relationship to remaining acquisition work

C02f-BI closes only the first-owner semantic selection gap identified by C02f-BG.

After BI, the principal remaining work is:

1. select the narrow control-plane acquisition-handoff preparation facade that internally branches between existing replacement preparation and the new first-owner bootstrap preparation;
2. materialize attempt-ID generation and bootstrap/provider-neutral source behind that facade in bounded source checkpoints;
3. materialize real etcd bootstrap execution/reconciliation using the selected `version == 0` Txn shape;
4. materialize a first-owner evidence mapper without weakening existing AS/AV replacement evidence;
5. integrate the already-selected AW replacement execution path plus the new bootstrap path into complete `acquire(peer)` composition;
6. only later consider production owner/runtime assembly and R1-R4 effect-side enforcement.

## Explicitly not authorized / not activated

C02f-BI does not:

- add or modify Rust source;
- alter `LiveOwnerTxnPlan`, AS, AE or AV source;
- generate production randomness or attempt IDs;
- allocate or reissue a production fence sequence;
- perform etcd Get/Txn or Spanner I/O;
- create any live-owner record;
- initialize PRWF state;
- issue or contact a recovery-epoch provider;
- expose private control-plane modules;
- construct endpoint/client/TLS/auth/RBAC/runtime state;
- activate R1-R4;
- deploy;
- merge.

## Validation gate

The gate is:

`C02F_BI_FIRST_OWNER_LIVE_OWNER_BOOTSTRAP_SEMANTICS_SELECTED`

It may be claimed only after canonical executable Rust validation passes on the exact final documentation-only BI head and exact BH ancestry/scope are reverified.
