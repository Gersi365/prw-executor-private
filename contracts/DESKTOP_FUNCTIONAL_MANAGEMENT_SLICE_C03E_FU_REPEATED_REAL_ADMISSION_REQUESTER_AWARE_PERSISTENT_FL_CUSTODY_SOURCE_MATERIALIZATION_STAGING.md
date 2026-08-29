# Phase 152 C03e-FU — Repeated Real-Admission Requester-Aware Persistent FL Custody Source Materialization

Status: VALIDATING

## 1. Scope

C03e-FU is the source-materialization checkpoint for the exact C03e-FT-selected repeated real-admission requester-aware persistent FL custody integration.

FU may materialize only the narrow composition that carries successful existing AJ admission into exact C03e-FS recoverable persistent requester-aware custody and exact C03e-FL worker execution while preserving the existing single repeated-admission supervisor law.

FU does **not** select or materialize requester-record cleanup, peer reuse/restart, new peer-close behavior, candidate/reachability continuation, endpoint selection, target dialing, forwarding, listener/bootstrap/readiness activation, deployment, restart/recovery, or merge.

## 2. Exact predecessor

Canonical predecessor is exact C03e-FT:

- branch: `phase-152-c03e-ft-repeated-real-admission-requester-aware-persistent-fl-custody-integration-semantics-selection-staging`
- head: `9215ac6e359ce82214aa2b5c1086f0eac7b9c6e4`
- tree: `7da33180b057fbb1efa4e4c1831867a9b4cb823a`
- FT contract blob: `9a1097b32d6361086dd6eb5e924b371c0dc6d491`

FT remains frozen.

## 3. Exact source guards

FU materialization is bounded by the FT-audited exact source guards:

- repeated real-admission executor: `5d2dec029050fcc6215439bf3b377da7064b980e`
- FS recoverable persistent child: `731d97011ef17edc34ef407ed1f4e65c291875f7`
- FQ/FS recoverable parent: `c6968b4f1397ee7c6678c10927f01dcd0a7e97e4`
- exact FL worker/stop family: `bc0b9c49471d515b721c9cf47cd27ec3111f32ca`
- shared requester/rendezvous authority: `70fa4a51516ea93be47438951a27a1a15c23109c`
- requester-aware policy source: `f7377011a3ab2034c14d9018a5c0f268f6660ffa`
- AJ real-admission transaction: `812b56e9b948a41f2f746eb406ba24567efbd528`
- shared-current authority: `50356b47d3c5304b67edd424e9286beb028ace16`
- remote-session parent: `450f27574270f84a88f77276afb1618b84476035`

No guard contradiction was found before FU mutation.

## 4. Authorized source shape

FU is intentionally narrow. The materialization may add or change only:

1. this FU contract;
2. the recoverable spawned requester-aware parent registration/completion façade;
3. the FS child only to widen exact ready-reap/cancellation/drain helpers to its integration sibling;
4. one new requester-aware repeated real-admission integration child module.

FU must not mutate AJ, FL, shared-current authority, shared requester authority, requester-aware policy source, transport runtime, endpoint lifecycle, process lifecycle, registry, session authentication, Android, Cargo dependency graph, lockfiles, workflow definitions, or production bootstrap.

## 5. Single-supervisor law

FU preserves one repeated expected-device admission supervisor on the existing private current-thread executor runtime.

FU does not introduce:

- a second persistent supervisor;
- a second active worker map;
- an admission bridge channel;
- a second Tokio runtime;
- a requester-only scheduler task.

The FU integration itself owns the single loop for ready completion, shutdown observation, bounded expected-request admission, one in-flight AJ transaction, authenticated insertion, and orderly drain.

## 6. AJ law

`admit_expected_remote_device_session(...)` remains byte-stable and authoritative for:

- current-registry expected transport resolution;
- exact lower-transport peer acceptance;
- post-accept challenge preparation;
- logical-session authentication;
- post-auth binding;
- existing AJ cleanup and error taxonomy.

FU adds no requester policy or requester authority work inside AJ.

AJ success still yields exactly one `AuthenticatedRemoteSessionRuntimeOwner`.

## 7. Identity law

Pre-auth `expected_device_id` remains scheduling/preflight evidence only.

Before timing or AJ work, an expected request is rejected when the same logical device already has an active requester-aware worker.

After AJ success, the active-map insertion key is derived only from:

`session_owner.logical_device_id()`.

No transport identity, peer address, endpoint tuple, request ID, task ID, queue index, mutex identity, or Arc identity becomes logical worker identity.

## 8. Active-map law

FU materializes the FT-selected requester-aware active map shape:

`HashMap<DeviceId, RecoverableRequesterAwareWorkerEntry>`.

Capacity continues to use the existing `MAX_REGISTERED_DEVICES` validation.

Ready terminal entries are detached and cease occupying scheduler capacity.

No `DeviceId` tombstone or quarantine is added.

## 9. Shared dependency law

Each requester-aware FL worker uses:

- a clone of the existing `SharedCurrentCapabilityAuthority` handle;
- a clone of the exact `SharedRequesterRendezvousAuthority` handle;
- a clone of an `Arc` owning the immutable requester-aware policy source;
- the existing dispatcher and verifier-time provider moved by value;
- the existing cooperative cancellation signal.

No provider clone, requester-state snapshot, registry snapshot, mutable policy snapshot, or fallback capability policy is selected.

## 10. Recoverable entry construction

Each successful AJ insertion constructs exactly one FS-compatible recoverable entry:

- exact authenticated owner stored in `Arc<Mutex<Option<_>>>`;
- supervisor retains one owner-cell handle;
- spawned task receives another owner-cell handle;
- one existing non-cloneable cancellation controller remains supervisor-owned;
- paired cancellation signal moves into exact FL;
- exact join handle remains in the active entry.

The authenticated owner itself is never cloned.

The worker locks the cell and mutably borrows the exact owner for FL execution.

## 11. Exact worker law

The spawned worker body is exactly:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker(...)`.

FU does not run `run_capability_request_worker(...)` for the same admitted session and does not run two workers against one owner.

Exact FL cancellation ordering and exact FL terminal classification remain authoritative.

## 12. FS helper reuse

FU reuses the exact C03e-FS implementations for:

- ready recoverable worker reaping;
- cooperative cancellation request across all active entries;
- recoverable owner-bearing drain.

FU widens only their module-sibling visibility. It does not duplicate the owner recovery algorithm in the real-admission module.

The underlying order remains:

`join terminal -> detach active entry -> recover exact owner -> preserve/map exact result -> publish completion`.

## 13. Completion façade

FU materializes one parent-level opaque requester-aware completion envelope containing:

- authenticated logical `DeviceId`;
- exact recovered `AuthenticatedRemoteSessionRuntimeOwner`;
- exact `Result<RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

The façade supports borrowing exact identity/owner/result and consuming complete custody by value.

It does not close, reuse, restart, retry, clean requester state, select reachability, or dial.

## 14. Exact FL result law

Normal exact FL stop remains unchanged:

- `Cancelled`;
- `Failed(Ingress(...))`;
- `Failed(RequesterResponse(Frame(...)))`;
- `Failed(RequesterResponse(ResponseIo(...)))`.

FU does not translate a normal FL failure into AJ failure, capability-worker failure, abnormal Tokio completion, peer-close authority, or cleanup authority.

## 15. Abnormal join law

Abnormal Tokio task completion maps only to existing:

`RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`.

The exact owner is recovered before completion publication.

No raw Tokio join error, panic payload, task ID, replacement worker, synthetic FL stop, or retry token is exposed.

## 16. Duplicate-active law

Expected-device duplicate detection remains before timing sampling and AJ/network work.

FU reuses the existing:

`RemoteSessionExpectedDeviceAdmissionRejectionReason::DuplicateActiveDevice`.

It introduces no second duplicate reason family.

Because at most one AJ future exists and no second expected request is admitted while it is pending, post-auth occupied insertion remains an internal invariant contradiction.

FU does not add automatic peer disposition for that contradiction.

## 17. Ready-completion-first law

On every repeated-supervisor wake, ready requester-aware workers are reaped before shutdown/request selection.

While AJ is in flight, ready workers are also reaped before the shutdown-vs-AJ poll.

Completion publication therefore releases active scheduler occupancy before same-wake admission decisions.

## 18. Shutdown before request

If supervisor shutdown is ready before a new expected request:

- no new AJ starts;
- all active requester-aware workers receive cooperative cancellation;
- the same retained entries are drained through exact FS owner recovery;
- the supervisor returns only after the active map is empty.

A prequeued expected request is not fabricated as AJ failure or duplicate rejection.

## 19. Shutdown during AJ

When shutdown wins while the one AJ future is pending:

1. request cooperative cancellation for every already-active requester-aware entry;
2. retain the AJ future rather than dropping it;
3. continue reaping ready active workers with exact FS owner recovery while AJ drains;
4. preserve existing AJ failure unchanged;
5. if AJ succeeds after shutdown, do not insert a worker;
6. invoke the existing authenticated-owner orderly-close seam for that never-inserted AJ success;
7. drain all previously active requester-aware workers to ownership-bearing completion;
8. return only after active custody is empty.

The post-shutdown AJ-success close is not generalized to ordinary requester-aware worker completion.

## 20. Cancellation-all-then-drain

Orderly supervisor shutdown preserves:

- stop new admission;
- request cancellation for every active requester-aware worker;
- retain all owner cells/controllers/join handles;
- drain the same entries;
- recover each exact owner;
- publish each exact owner-bearing completion;
- return only when no active entry remains.

No task abort and no whole-worker timeout/drop path is added.

## 21. Request source closure

Closing the expected-request source alone does not cancel active workers or end the supervisor.

Explicit supervisor shutdown remains authoritative.

## 22. Peer law

Requester-aware worker completion remains `retained-stopped` custody.

FU performs no automatic peer close on:

- FL cancellation;
- FL typed failure;
- abnormal Tokio join;
- ordinary ready completion;
- orderly active-worker cancellation/drain.

Existing capability worker code-3/code-4 behavior is not widened onto FL.

## 23. Requester-record law

FU performs no requester registration retirement, removal, reset, timeout cleanup, TTL cleanup, or completion-triggered cleanup.

The existing shared requester authority is used only by exact FL composition.

## 24. Endpoint lifecycle separation

Existing whole-endpoint close/idle drain after repeated supervisor return remains separately materialized and unchanged.

FU does not invoke endpoint-wide close from one requester-aware worker completion.

## 25. Focused tests

FU adds focused source-level tests for the new integration scheduler boundaries:

- duplicate expected `DeviceId` is rejected before timing sampling;
- a vacant expected request samples timing exactly once and preserves request custody;
- a full active collection does not poll the expected-request source;
- supervisor shutdown wins before a prequeued expected request.

Exact owner recovery, abnormal join recovery, duplicate active entry behavior, ready-completion-first ordering, cancellation-all-then-drain, closed admission source, and capacity ceiling remain covered by the existing FS focused tests whose implementation is reused.

## 26. Explicitly absent

FU does not materialize:

- requester-record cleanup;
- requester peer reuse or restart;
- new peer-close semantics;
- retry/resend/replacement worker;
- candidate query/selection;
- reachability evaluation;
- direct/relay selection;
- endpoint selection;
- target dialing;
- forwarding;
- rendezvous-completion claim;
- production listener/bootstrap activation;
- readiness publication;
- Android behavior change;
- desktop UI behavior change;
- deployment;
- restart/recovery;
- merge.

## 27. Validation gate

FU remains `VALIDATING` until the exact final branch head has:

- exact FT merge-base verification;
- authorized-path-only diff verification;
- Rust locked dependency graph PASS;
- rustfmt PASS;
- Clippy PASS;
- workspace tests PASS;
- workspace build PASS;
- any path-triggered Android/native application validation required by the repository workflow;
- immutable Drive audit upload and raw byte-exact readback verification.

No superseded candidate CI result may be used as canonical closure evidence.

## 28. Intended closure

After exact-final-head validation and immutable evidence, canonical closure is:

`CLOSED_REPEATED_REAL_ADMISSION_REQUESTER_AWARE_PERSISTENT_FL_CUSTODY_SOURCE_MATERIALIZATION`

Canonical gate:

`C03E_FU_REPEATED_REAL_ADMISSION_REQUESTER_AWARE_PERSISTENT_FL_CUSTODY_SOURCE_MATERIALIZED`

## 29. Next gate boundary

The next checkpoint must be separately selected from the recovered requester-aware completion custody.

FU itself does not choose whether a retained-stopped peer is closed, reused, restarted, associated requester state is cleaned, or candidate/reachability continuation begins.
