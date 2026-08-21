# Phase 152 C02f-AT — Reconciled Live-Owner Acquisition Semantic Mapping Selection

## Purpose

C02f-AT selects the provider-neutral semantic mapping required after the already-validated C02f-AS acquisition handoff and the already-validated C02f-AE bounded provider reconciliation have produced one terminal, authoritative acquisition outcome.

C02f-AT is a documentation-only architecture selection. It does not call C02f-AE, C02f-AD, C02f-AC, an etcd client, a runtime, an endpoint or any network provider.

The purpose is to remove one remaining ambiguity before a later source tranche: how one exact C02f-AS handoff plus one exact C02f-AE resolved mutation may be translated into the already-selected C02f-Y/C02f-AC semantic acquisition contract without manufacturing authority.

## Exact base

C02f-AT starts from the exact canonical C02f-AS head:

`ddaff332b685b5ab086d945822b33b9b4365f780`

C02f-AS remains canonical and byte-stable.

## Retained inputs

A future mapping implementation may accept only:

1. one exact C02f-AS `FenceSequenceLiveOwnerAcquisitionHandoff`; and
2. one terminal C02f-AE `ReachabilityLiveOwnerResolvedMutation` produced for that same logical mutation.

The mapper must not perform provider I/O, re-observe authority, allocate another sequence, choose another fence or rebuild a different successor.

Before any semantic mapping, the complete C02f-AE resolved `LiveOwnerTxnPlan` must equal the exact transaction retained by the C02f-AS handoff.

A plan mismatch is fail-closed and must become semantic `UnavailableOrAmbiguous`; it can never produce `Granted` or `Contended` by inference.

## Exact peer and successor binding

The retained transaction successor must continue to satisfy the already-validated C02f-AB/C02f-AR invariants:

- lifecycle is exactly `Current`;
- peer is exactly the requested `DeviceId + TransportIdentity` lifecycle;
- fence is the exact non-zero canonical AR fence `(epoch << 64) | sequence`;
- live-owner `AuthorityAttemptId` is the exact retained mutation attempt identifier;
- no request-controlled or reconstructed fence/peer/attempt identity may replace retained evidence.

Malformed, cross-peer, non-`Current` or plan-mismatched context fails closed as `UnavailableOrAmbiguous`.

## Terminal acquisition mapping

### C02f-AE `Committed`

If and only if:

- the complete AE resolved plan exactly equals the AS retained transaction;
- the retained successor is exact-peer and `Current`; and
- the successor fence converts to the already-selected semantic `ReachabilityLiveOwnerFence` representation;

then the semantic result is:

`ReachabilityLiveOwnerAcquisition::Granted(exact_peer, exact_successor_fence)`

This does not mean C02f-AT itself grants authority. It means a later mapper may construct the existing semantic grant only from a terminal C02f-AE result that already proves the exact intended mutation committed.

### C02f-AE `CompareFailed(authoritative_observation)`

A definitive compare failure never constructs a grant. Its semantic acquisition result is:

`ReachabilityLiveOwnerAcquisition::Contended`

This preserves the existing C02f-AC definitive compare-failure mapping.

### C02f-AE `Superseded`

A C02f-AE `Superseded` result means a fresh authoritative observation proved that a later valid authority state superseded the unresolved logical acquisition. It is therefore a definitive non-grant outcome, not an unavailable/ambiguous provider result.

Its semantic acquisition result is selected as:

`ReachabilityLiveOwnerAcquisition::Contended`

`Superseded` must never produce `Granted`.

## Errors remain errors

C02f-AE execution/reconciliation failures are not terminal resolved mutations and are not normalized into `Contended`.

In particular, a future orchestration adapter must preserve fail-closed error behavior for:

- provider/read/transaction unavailability;
- malformed or contradictory authoritative state;
- deterministic transaction validation failure;
- exhausted one-reissue bound (`ReissueLimitReached`);
- handoff/resolved-plan mismatch;
- invalid peer/lifecycle binding;
- impossible semantic fence conversion.

Those conditions map to the existing semantic authority error domain (`UnavailableOrAmbiguous`, or `FenceExhausted` where the ordered-generation representation itself cannot be represented safely), never to successful ownership.

## Dependency direction

C02f-AT preserves the selected dependency direction:

- concrete etcd/provider execution remains owned by `prw-control-plane`;
- semantic async authority remains owned by `prw-remote-bridge`;
- `prw-control-plane` must not gain a dependency on `prw-remote-bridge`;
- any later source mapper belongs on the orchestration/bridge side or in another layer that already depends downward on the control-plane types.

No dependency edge is changed by this documentation-only tranche.

## Explicit non-goals / non-activation boundary

C02f-AT does not:

- implement the mapping in Rust;
- modify C02f-AS, AR, AQ, AP, AE, AD, AC, AB or PRWL source;
- call `ReachabilityLiveOwnerEtcdStore::execute`;
- call `execute_acquisition_with_reconciliation`;
- perform a live-owner etcd read, Txn or re-observation;
- allocate or issue a fence sequence;
- issue a recovery epoch or contact Spanner;
- create/connect an etcd client;
- select or materialize endpoints, TLS, authentication, RBAC, credentials, leases, TTL, Watch, users, roles, permissions or cluster membership;
- generate sequence-allocation or live-owner attempt IDs;
- activate `ReachabilityLiveOwnerAsyncAuthority` in a production/runtime composition;
- construct an async runtime, process lifecycle, task, timer or detached future;
- execute traversal/network effects;
- implement or activate R1-R4 stale-fence effect rejection;
- modify Cargo manifests or `Cargo.lock`;
- deploy or merge a draft PR.

## Exact source scope

C02f-AT adds exactly this one documentation contract and no Rust/workflow/dependency/runtime file.

## Validation gate

C02f-AT is valid only if canonical repository validation remains green on the exact AT head and a fresh AS -> AT compare proves the tranche is exactly one documentation-only addition.

Expected gate after validation:

`C02F_AT_RECONCILED_ACQUISITION_SEMANTIC_MAPPING_SELECTED`

A later source tranche may materialize this pure result mapper. A still-later separately authorized boundary may compose real provider execution. C02f-AT itself crosses neither boundary.
