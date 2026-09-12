# C03e-RG — Production-Durable Post-Auth Fallible Verifier-Time Requester-Rendezvous Scheduling-Aware Recoverable Repeated Real-Admission Single-Completion Poll Adapter Selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SCHEDULING_AWARE_RECOVERABLE_REPEATED_REAL_ADMISSION_SINGLE_COMPLETION_POLL_ADAPTER_SELECTION`

Selected future boundary:

`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_SCHEDULING_AWARE_RECOVERABLE_REPEATED_REAL_ADMISSION_SINGLE_COMPLETION_POLL_ADAPTER_SOURCE_MATERIALIZATION`

## Exact predecessor

Evidence-closed C03e-RF:

- branch `phase-152-c03e-rf-production-durable-post-auth-fallible-verifier-time-requester-rendezvous-scheduling-aware-recoverable-repeated-real-admission-collection-custody-adapters-source-materialization`;
- final head `d06b1e884135a0b3787dcacad759753cb381754f`;
- final tree `6c5bed3ed02fcb3118b9f0e6e634b8a05392d9fd`;
- exact selected source blob `aa183241c439e82b7b2f53a6d4437f695c3674f9`;
- PR #596 remains draft/open/unmerged/mergeable and evidence-closed;
- canonical RF audit Drive ID `1YSQoczRK4sDFHmHgnn33ljAaTNZu255M`, `17103` bytes, SHA-256 `a818277d83afe3dbea21e699839f803a9df9b81ad4903dc646db3c3d3f011334`.

Fresh post-RF authority/concurrency audit also verified:

- `main` remains head `7c993fa93977a0bb84e0d030874eee7fd0cae77f`, tree `63b8e59ca53797fdea6b95432e16f35eaf473604`;
- PR #596 remains the newest PR before RG creation;
- canonical RF evidence remains the exact singleton under parent `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
- no pre-existing `C03e-RG` PR, branch, or canonical evidence artifact existed.

## Fresh selection finding

Exact RF source inspection proves that RF completed the fallible scheduling collection-local custody family except for the cooperative single-completion polling sibling.

The historical scheduling path already owns:

`poll_one_requester_aware_scheduling_worker(...)`

with the exact law:

1. call existing generic `poll_one_ready_recoverable_worker(active, context)`;
2. return `Poll::Pending` unchanged when no completion is ready;
3. when ready, consume the generic completion by value through `into_parts()`;
4. construct exactly one owner-bearing scheduling completion envelope from the exact authenticated `DeviceId`, recovered `AuthenticatedRemoteSessionRuntimeOwner`, and exact worker/join result;
5. return that envelope as `Poll::Ready(...)` without peer disposition or any other side effect.

The historical cooperative scheduling driver uses that single-completion adapter as a prerequisite event source in both idle and admission polling. Therefore migration of the cooperative/production collection driver is not the immediate dependency after RF.

RF already materialized the parallel fallible scheduling:

- generic completion alias;
- authenticated `DeviceId` keyed active-map alias;
- completion publication adapter;
- ready-reap adapter;
- cancellation fan-out adapter;
- full-drain adapter;
- in-flight-admission drain adapter;
- QZ fallible scheduling recoverable worker-entry constructor remains available;
- RD owner-bearing fallible completion envelope and owner-disposition law remain available.

The only immediate missing collection-local custody primitive before any fallible cooperative driver migration is the parallel single-completion `poll_one` adapter.

## Selected future C03e-RH source ceiling

Future C03e-RH may change exactly one Rust path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

Required predecessor blob:

`aa183241c439e82b7b2f53a6d4437f695c3674f9`

RH may add exactly one dormant sibling function:

`poll_one_requester_aware_fallible_verifier_time_scheduling_worker(...)`

Selected exact signature law:

- input active map: mutable `ActiveRecoverableFallibleVerifierTimeSchedulingRequesterAwareWorkers`;
- input poll context: `&mut Context<'_>`;
- output: `Poll<RecoverableRepeatedRealAdmissionRequesterAwareFallibleVerifierTimeSchedulingWorkerCompletion>`.

Selected implementation law:

- call existing `poll_one_ready_recoverable_worker(active, context)` exactly once;
- `Poll::Pending` maps directly to `Poll::Pending`;
- `Poll::Ready(completion)` consumes the generic completion exactly once through existing `into_parts()`;
- preserve exact `DeviceId` by value;
- preserve recovered `AuthenticatedRemoteSessionRuntimeOwner` by value;
- preserve exact `Result<RequesterRendezvousFallibleVerifierTimeProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>` by value and unflattened;
- construct the already-existing `RecoverableRepeatedRealAdmissionRequesterAwareFallibleVerifierTimeSchedulingWorkerCompletion::new(...)` exactly once;
- return that envelope as `Poll::Ready(...)`;
- no callback invocation, peer disposition, owner close/reuse, suppression, verifier-time sampling, worker spawn, request polling, admission polling, cancellation, retry, queue/channel/task creation, or producer interaction occurs in this function.

The future function may be annotated only as needed to keep this deliberately dormant seam compile-clean before separately gated caller migration.

No new import should be required: `Context`, `Poll`, the generic poll primitive, the fallible active-map alias, and the owner-bearing fallible completion envelope are already in the exact predecessor source scope. If correct RH compilation requires a second Rust path, a visibility widening, a generic persistent custody mutation, or a production collection-driver mutation, RH must STOP and return to selection.

## Explicitly deferred after RH

RG does not select or authorize RH to materialize:

- any mutation of `production_durable_repeated_real_admission_collection.rs`;
- fallible cooperative driver event enums;
- fallible `poll_cooperative_*` driver/admission/producer helpers;
- migration of `drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_scheduling`;
- callback/disposer wiring inside the production collection driver;
- producer future/lending-producer migration;
- shutdown suppression/receipt observation migration;
- endpoint/executor/higher-owner propagation;
- expected-device request construction or send;
- SessionId, request-ID, or real-admission timing allocation;
- verifier-time sampling outside the existing worker admission/worker seam;
- new queue/channel/task topology;
- runtime/listener/network/auth/database/control-plane activation or mutation;
- Cargo manifest/lockfile/workflow/Android-source mutation;
- packaging/service/repository configuration;
- merge/deployment/ready-for-review transition;
- branch deletion/reset/rebase/squash/force/history rewrite;
- destructive evidence cleanup.

The production collection driver remains a separately gated checkpoint after RH.

## Exact byte-stable future guards

Future RH must keep unchanged:

- production-durable repeated-admission collection blob `08dc8160d72c41bc9a211c2c2e15f0e65d067b74`;
- RD parent recoverable-worker source blob `0c45ffa31c9e3263264544ae149ba29a6991fdc1`;
- QX lifecycle blob `39f91dc8510df49620cf3336e98656532b46c158`;
- generic persistent custody blob `4b33440ee2ddeceb2e62d016f42a3bcf332a37c0`.

Within the selected target source, existing RF aliases and publish/reap/cancel/drain/in-flight-drain adapters, QZ fallible worker-entry constructor, historical scheduling adapters, historical `poll_one_requester_aware_scheduling_worker(...)`, and historical non-fallible collection surfaces must remain semantically unchanged.

## Validation and evidence requirements for RG

RG itself is selection-only and must remain docs-only.

Before RG closure:

- direct RF -> RG topology must prove exact RF merge base, ahead-only lineage, exactly one documentation path, and zero Rust/source/runtime/Cargo/lockfile/workflow/Android-source/packaging/deployment/repository-config changes;
- exact-final-head CI may claim only workflows actually registered on RG head;
- `SKIPPED` is never PASS;
- immutable canonical RG audit bytes must be frozen once with internal status `SELECTION — VALIDATED — EVIDENCE PUBLICATION PENDING`;
- exact-title Drive pre-upload search must return zero;
- upload must occur once under canonical parent `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
- raw Drive readback must match exact frozen bytes/SHA-256/final-LF;
- exact-title post-upload search must return exactly one canonical artifact;
- revision lineage must contain exactly one current revision with `previousRevisionId = null`;
- only then may PR metadata bind external status `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`.

Historical C03e-QS duplicate evidence remains untouched.

## Stop boundary

C03e-RG is selection-only.

Do not materialize C03e-RH inside RG closure.

Do not mutate either selected Rust source or production collection driver inside RG.

Do not merge, deploy, activate runtime/network/service behavior, mark ready for review, rewrite history, delete branches, or destructively clean evidence.

**C03e-RG must STOP after validated immutable selection evidence is recorded.**
