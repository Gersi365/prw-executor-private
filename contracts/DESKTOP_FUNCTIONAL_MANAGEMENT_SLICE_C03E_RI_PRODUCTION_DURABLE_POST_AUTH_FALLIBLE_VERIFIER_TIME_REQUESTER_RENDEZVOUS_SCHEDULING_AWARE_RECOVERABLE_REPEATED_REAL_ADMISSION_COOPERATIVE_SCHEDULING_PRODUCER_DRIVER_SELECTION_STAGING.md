# C03e-RI — Fallible verifier-time cooperative scheduling producer-driver selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SCHEDULING_AWARE_RECOVERABLE_REPEATED_REAL_ADMISSION_COOPERATIVE_SCHEDULING_PRODUCER_DRIVER_RECEIPT_PROPAGATION_SOURCE_SEAM_SELECTION`

Selected future boundary:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SCHEDULING_AWARE_RECOVERABLE_REPEATED_REAL_ADMISSION_COOPERATIVE_SCHEDULING_PRODUCER_DRIVER_SOURCE_MATERIALIZATION`

## Exact predecessor authority

Evidence-closed C03e-RH is authoritative:

- head `7e146c1a12afb0b3f67b92d2bbee94965fb8cd55`;
- tree `3f5a2a208efef362d44e26e2669a76cd3830fdca`;
- selected integration source blob `304513eaf4ac72720a96c265f72dd8a56f5ac87f`;
- nested production/cooperative collection blob `08dc8160d72c41bc9a211c2c2e15f0e65d067b74`;
- PR #598 remains draft/open/unmerged/mergeable and evidence-closed;
- canonical RH audit Drive ID `1mzU8b5jNKBD_oIF1ppPuuk6TuKphbazo`, 12779 bytes, SHA-256 `18c68eedd82c4ac0b19c4a1006e96ce9ffede9c831609949f8e5a3262254b47b`.

Fresh post-RH authority audit also confirmed `main` remains `7c993fa93977a0bb84e0d030874eee7fd0cae77f` / tree `63b8e59ca53797fdea6b95432e16f35eaf473604`, #598 is the newest PR, RH evidence has exactly one revision, and no RI branch/PR/canonical evidence artifact existed before RI creation.

## Selection finding

Exact RH source now owns the full fallible-verifier-time scheduling collection-local custody prerequisites:

- `ActiveRecoverableFallibleVerifierTimeSchedulingRequesterAwareWorkers`;
- `RecoverableFallibleVerifierTimeSchedulingRequesterAwareWorkerCompletion`;
- `publish_recoverable_fallible_verifier_time_scheduling_completion(...)`;
- `poll_one_requester_aware_fallible_verifier_time_scheduling_worker(...)`;
- ready-reap, cancellation fan-out, full-drain and in-flight-admission drain adapters;
- `spawn_recoverable_fallible_verifier_time_requester_aware_worker_with_production_durable_scheduling(...)`;
- exact owner-bearing fallible scheduling completion envelope;
- exact fallible scheduling completion disposer.

The nested production file remains byte-stable at `08dc8160...` and still contains only the historical infallible scheduling cooperative producer-driver lane. Its historical lane is typed over:

- `ActiveRecoverableSchedulingRequesterAwareWorkers`;
- `RequesterRendezvousProductionDurableSchedulingWorkerStop`;
- `T: FnMut() -> u64 + Send + 'static`;
- the historical scheduling completion disposer/spawn constructor.

The historical cooperative driver already proves the required concurrency and shutdown law. The OI→OJ→OK precedent established that once a scheduling single-completion adapter exists, the next separately gated boundary is one dormant cooperative producer-driver sibling plus same-file private progress/shutdown helpers.

Therefore the immediate missing dependency after RH is the parallel fallible-verifier-time cooperative scheduling producer driver in the same nested production file. Endpoint producer forwarding, concrete receipt/channel ownership, request construction and runtime activation remain later gates.

## Selected future C03e-RJ source ceiling

Future C03e-RJ may change exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`

Exact required predecessor blob:

`08dc8160d72c41bc9a211c2c2e15f0e65d067b74`

No second Rust path is selected.

## Selected future driver law

RJ may materialize one dormant sibling method:

`drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_fallible_verifier_time_scheduling_producer(...)`

The future method must remain generic over receipt type and borrow exactly one caller-owned lending async producer. It may retain at most one lexical producer-call future at a time and must never spawn/detach that future.

Exact terminal result input to producer and suppression mapper:

`Result<RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>`

Exact verifier-time provider bound:

`T: FnMut() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError> + Send + 'static`

Normal recovered completion law:

1. recover exactly one fallible scheduling completion through `poll_one_requester_aware_fallible_verifier_time_scheduling_worker(...)`;
2. consume recovered owner custody first through `dispose_recoverable_repeated_real_admission_requester_aware_fallible_verifier_time_scheduling_worker_completion(...)`;
3. only then invoke the borrowed producer with exact authenticated `DeviceId` plus unchanged exact terminal result;
4. preserve the producer future lexically until exactly one generic receipt is produced;
5. synchronously observe that receipt exactly once.

Existing requester acknowledgement controls scheduling-terminal owner disposition; scheduling derivation success/failure never controls peer disposition.

## Selected same-file helper adaptation

RJ may add only private fallible siblings required by the one driver. Prefer reusing type-agnostic historical helpers instead of duplicating them.

Fallible-specific siblings are selected only where the historical helper is statically bound to the infallible scheduling lane, including as necessary:

- idle event carrying exact fallible scheduling completion custody;
- admission event carrying exact fallible scheduling completion custody;
- idle poll using the RH fallible `poll_one` adapter;
- admission poll using the RH fallible `poll_one` adapter;
- shutdown-begin helper over `ActiveRecoverableFallibleVerifierTimeSchedulingRequesterAwareWorkers`;
- admission-finisher using the QZ fallible worker constructor and fallible verifier-time `T` bound;
- worker drain with suppression using the RD fallible disposer and exact fallible terminal result;
- pending borrowed-producer driver over the fallible active map/result lane.

The existing generic producer event, generic producer+admission event, generic producer-drive outcome, generic producer polling, generic producer+admission polling, shutdown-admission finisher, expected-request preparation and AJ admission primitive should be reused unchanged where their types already permit it.

## Cooperative progress and shutdown law

RJ must preserve the existing OJ/OK cooperative law exactly:

- explicit supervisor shutdown has highest priority;
- while a producer future is pending, eligible expected-request receiver progress continues;
- while a producer future and AJ admission are both pending, both remain lexical and progress cooperatively without hidden tasks;
- a direct producer await that blocks receiver/admission progress remains forbidden;
- on explicit shutdown, close the expected-request receiver only after shutdown is observed and retire buffered unadmitted requests;
- no new producer call may begin after quiescence;
- preserve any already-started producer future to one terminal receipt;
- preserve any already-started admission to exact terminal disposition;
- request cancellation for active fallible scheduling workers and drain them;
- each drained completion must run existing RD owner disposition before synchronous shutdown suppression mapping;
- each suppression mapping returns the same generic receipt type and each receipt is observed exactly once.

`try_send`, `blocking_send`, callback `block_on`, detached/hidden spawn, cloned sender, second/unbounded queue, retry, defaulting, grant remint/replay, terminal-result reconstruction and scheduling-consumption rollback remain forbidden.

## Exact identity and provenance law

- completion `DeviceId` remains the authenticated requester-side identity/correlation value;
- no target expected `DeviceId` is synthesized from it;
- exact recovered terminal result remains by value and unflattened;
- exact fallible verifier-time source failure provenance remains nested unchanged;
- requester response/acknowledgement provenance remains nested unchanged;
- scheduling derivation and acknowledgement channels remain independent;
- abnormal join remains exact `RemoteSessionSpawnedWorkerJoinError`;
- no clone/copy/remint/stringification/reconstruction/default/cache/retry of terminal custody.

## Explicitly deferred after RJ

RJ is not authorized to add or migrate:

- endpoint-lifecycle producer forwarding;
- concrete `HandoffReceipt` or another concrete receipt family;
- production channel/sender owner or sender clone;
- actual request construction/send/enqueue policy;
- target admission `SessionId` allocation;
- PRWM authentication request-ID allocation;
- NB dispatcher production wiring;
- verifier-time source/interface mutation outside the already-selected fallible provider type;
- producer/executor/endpoint/higher-owner caller migration;
- requester cleanup;
- candidate/reachability continuation;
- target dialing;
- listener/bootstrap/readiness/runtime/executable activation;
- authentication/database/control-plane mutation;
- Cargo manifest/lockfile mutation;
- GitHub workflow mutation;
- Android source mutation;
- packaging/service/repository configuration;
- merge/deployment/ready-for-review transition;
- branch deletion/reset/rebase/squash/force/history rewrite;
- destructive evidence cleanup.

If correct RJ compilation requires a second Rust path, parent integration mutation, RD parent mutation, QX lifecycle mutation, generic persistent custody mutation, visibility widening, endpoint forwarding or any deferred surface above, RJ must STOP and return to selection.

## Byte-stable future guards

RJ must keep unchanged:

- RH integration source `304513eaf4ac72720a96c265f72dd8a56f5ac87f`;
- RD parent recoverable-worker source `0c45ffa31c9e3263264544ae149ba29a6991fdc1`;
- QX lifecycle `39f91dc8510df49620cf3336e98656532b46c158`;
- generic persistent custody `4b33440ee2ddeceb2e62d016f42a3bcf332a37c0`.

Historical infallible scheduling cooperative producer driver and endpoint lifecycle must remain behaviorally unchanged; RJ adds only a parallel dormant fallible lane in the selected nested source ceiling.

## Validation and closure law

RI is selection-only. It may add only this contract document.

After exact-final-head CI and immutable evidence publication, keep the RI PR draft/open/unmerged and STOP. Do not create or materialize RJ inside RI closure.
