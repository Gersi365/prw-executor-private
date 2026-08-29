# Phase 152 C03e-FT — Repeated Real-Admission Requester-Aware Persistent FL Custody Integration Semantics Selection

Status: CLOSED

## 1. Scope

C03e-FT is a semantics-selection checkpoint only.

It selects the narrow integration law for carrying the exact C03e-FS recoverable persistent requester-aware worker custody into the existing repeated expected-device real-admission supervisor after one successful authenticated AJ transaction.

FT does **not** materialize Rust source, replace the existing capability worker in this checkpoint, alter authentication, change peer-close behavior, clean requester records, select candidate/reachability state, dial target traffic, activate a listener/bootstrap/readiness path, deploy, restart/recover the process, or merge.

## 2. Exact predecessor

Canonical predecessor is exact C03e-FS:

- branch: `phase-152-c03e-fs-recoverable-persistent-requester-aware-worker-entry-completion-custody-source-materialization-staging`
- head: `6ad3942b95b8759c03c2e8122013ee870aa79610`
- tree: `92b5fbbd9184ced4503ec55c8ed3f1c497863666`
- FS contract blob: `4021c8f8474a76460928c407f39990bba8515778`
- FQ parent registration blob: `c6968b4f1397ee7c6678c10927f01dcd0a7e97e4`
- FS child module blob: `731d97011ef17edc34ef407ed1f4e65c291875f7`

FS is frozen.

## 3. Exact source audit guards

FT is selected against the exact FS tree and these authoritative sources:

1. repeated real-admission supervisor / current capability persistent integration
   - `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`
   - blob `5d2dec029050fcc6215439bf3b377da7064b980e`

2. exact FS recoverable persistent requester-aware custody
   - `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/recoverable_persistent_requester_rendezvous_worker.rs`
   - blob `731d97011ef17edc34ef407ed1f4e65c291875f7`

3. FQ/FS parent module surface
   - `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`
   - blob `c6968b4f1397ee7c6678c10927f01dcd0a7e97e4`

4. exact requester-aware FL worker and stop family
   - `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
   - blob `bc0b9c49471d515b721c9cf47cd27ec3111f32ca`

5. exact shared requester/rendezvous authority
   - `crates/prw-agent/src/remote_session_capability_runtime/shared_requester_rendezvous_authority.rs`
   - blob `70fa4a51516ea93be47438951a27a1a15c23109c`

6. requester-aware policy source
   - `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_source.rs`
   - blob `f7377011a3ab2034c14d9018a5c0f268f6660ffa`

7. exact AJ real-admission transaction
   - `crates/prw-agent/src/remote_session_capability_runtime/real_remote_admission_transaction.rs`
   - blob `812b56e9b948a41f2f746eb406ba24567efbd528`

8. shared-current registry/policy authority
   - `crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`
   - blob `50356b47d3c5304b67edd424e9286beb028ace16`

9. remote-session parent ownership/export surface
   - `crates/prw-agent/src/remote_session_capability_runtime.rs`
   - blob `450f27574270f84a88f77276afb1618b84476035`

Any contradiction with these guards requires a new gate rather than silent FT widening.

## 4. Existing repeated-supervisor law remains the integration locus

FT selects the existing `drive_repeated_real_remote_admission_collection(...)` scheduling model as the integration locus.

A future source checkpoint must not introduce a second persistent supervisor task, a second active-worker map, a new worker-admission channel, or a separate Tokio runtime merely to carry requester-aware workers.

The current one-loop structure remains authoritative:

- ready worker completion processing;
- supervisor shutdown observation;
- bounded expected-request polling;
- at most one in-flight AJ admission future;
- authenticated post-AJ worker insertion;
- orderly shutdown/drain.

Requester-aware custody is integrated into that same lifecycle rather than wrapped around it as an independent scheduler.

## 5. Existing AJ transaction remains unchanged

FT does not alter `admit_expected_remote_device_session(...)`.

The transaction continues to own exactly:

- current-registry expected transport resolution;
- exact expected lower-transport peer acceptance;
- post-accept current-registry challenge preparation;
- logical-session authentication;
- post-authenticated binding composition;
- its existing code-5, code-1 and code-2 failure cleanup boundaries.

AJ still returns exactly one `AuthenticatedRemoteSessionRuntimeOwner` on success.

No requester/rendezvous policy, requester authority, FL execution, persistent worker spawn, candidate/reachability continuation, or target dialing is moved into AJ.

## 6. Pre-auth expected DeviceId remains scheduling evidence only

The pre-authentication `expected_device_id` continues to gate whether an AJ attempt may start while an active worker already occupies that logical key.

It remains a scheduling and expected-device input, not a substitute for post-authenticated identity authority.

The existing preflight law remains:

- if the current active map contains the expected `DeviceId`, reject the untouched request before timing sampling and before AJ/network work;
- otherwise sample timing and begin the one AJ transaction.

FT does not promote IP address, port, endpoint tuple, `TransportIdentity`, request ID, task ID, Arc identity, mutex identity, or expected-request queue position into logical worker identity.

## 7. Post-authenticated DeviceId is the insertion authority

After AJ succeeds, the future requester-aware integration derives the active-map key from:

`session_owner.logical_device_id()`.

That authenticated logical `DeviceId` is authoritative for insertion.

The exact AJ expected-device transaction is already required to authenticate the intended device; therefore the existing equality invariant between authenticated and expected logical device remains preserved.

FT selects no runtime fallback that would reinterpret a post-authentication mismatch as a transport identity, silently rewrite the key, or insert under the pre-auth expected value.

## 8. One active map changes custody shape, not identity law

The future integrated active map is conceptually:

`HashMap<DeviceId, RecoverableRequesterAwareWorkerEntry>`.

This replaces only the active entry custody used by the repeated real-admission requester-aware path.

The logical key remains the exact authenticated `DeviceId`.

FT does not select a second requester-aware active map beside the capability map inside the same repeated supervisor.

The standalone existing capability persistent collection may remain as a separately usable historical/pre-production seam; FT does not require deleting it.

## 9. Existing ownership-only admission carrier may be reused

`RemoteSessionWorkerAdmission<D, T>` is already an ownership-only carrier containing:

- authenticated session owner;
- dispatcher;
- verifier-time provider.

Its logical key is derived from the contained authenticated owner and it carries no caller-supplied transport identity.

FT permits the future integration to reuse this carrier between AJ success and requester-aware entry construction.

No second requester-specific admission carrier is required unless a source-level visibility constraint proves otherwise.

## 10. New supervisor-scoped requester-aware inputs

Requester-aware real-admission integration requires exactly two additional shared inputs at the repeated-supervisor boundary:

1. one `SharedRequesterRendezvousAuthority` handle referring to the single process-local requester/rendezvous runtime owner;
2. one requester-aware policy source held through shared immutable `'static` custody suitable for worker task lifetime, conceptually `Arc<S>` where `S: RequesterRendezvousStartPolicySource + Send + Sync + ?Sized + 'static`.

The existing `SharedCurrentCapabilityAuthority<P>` remains a separate shared-current source.

FT selects no per-worker provider clone, no requester-authority snapshot, no mutable policy-source snapshot, and no process-global fallback evaluator.

## 11. Shared requester authority remains one source of truth

The repeated supervisor borrows or otherwise retains one exact `SharedRequesterRendezvousAuthority` handle.

Each spawned requester-aware worker may receive only a clone of that existing shared handle.

Cloning the handle does not clone or snapshot the underlying requester/rendezvous runtime owner or provider.

The existing lock order remains:

`requester/rendezvous authority -> shared-current registry/policy read`.

FT does not add an active-map lock or any requester-authority lock around worker insertion.

## 12. Requester-aware policy remains requester-specific and fail-closed

Each worker receives shared immutable custody of the exact requester-aware policy source selected by DP/DX.

At request time, the source continues to resolve policy only from the exact authenticated requester session dimensions.

FT selects no conversion to the principal-agnostic capability policy, no default evaluator, no cross-requester fallback, no policy cache inside the worker entry, and no live policy mutation semantics.

## 13. Exact requester-aware entry construction

For each successful AJ admission selected for insertion, the future source materialization must construct exactly one FS-equivalent requester-aware persistent entry containing:

- one recoverable owner cell holding the exact `AuthenticatedRemoteSessionRuntimeOwner`;
- one existing non-cloneable `RemoteSessionWorkerCancellationController`;
- one Tokio join handle whose normal result is exact `RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`.

The authenticated-session owner itself is not cloned.

## 14. Exact worker spawn law

The future spawn sequence is selected as:

1. derive authenticated logical `DeviceId` from the exact AJ-success owner;
2. retain the dispatcher and verifier-time provider from the admitted request;
3. create one existing cancellation controller/signal pair;
4. place the exact authenticated-session owner into one recoverable owner cell;
5. retain one owner-cell handle in supervisor custody;
6. clone only the existing shared-current authority handle required by task lifetime;
7. clone only the existing shared requester/rendezvous authority handle;
8. clone only the shared immutable requester-aware policy-source handle;
9. move the worker owner-cell handle, dispatcher, verifier-time provider, authority handles, policy-source handle and cancellation signal into exactly one Tokio task;
10. the task locks the owner cell and borrows the exact owner mutably;
11. the task runs exact FL with the existing cancellation signal;
12. the supervisor retains owner-cell recovery custody, the sole cancellation controller and the exact join handle in the active entry.

No task abort handle, retry token, replacement-worker token, peer-close token, candidate token, reachability token, or requester-cleanup authority is added.

## 15. Exact FL remains the worker body

The spawned task delegates directly to:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker(...)`.

The future integration must not translate FL back into the capability-only `run_capability_request_worker(...)` path.

It must not run both workers for one authenticated session.

It must not create a capability worker followed by a requester-aware worker restart.

## 16. FL cancellation law remains unchanged

The same existing cancellation signal is supplied to FL unchanged.

Therefore exact FL remains authoritative for:

- ingress-first polling before cancellation;
- cancellation while ingress is pending;
- cancellation deferral across requester FB/FH critical custody;
- requester-response failure precedence during that protected interval;
- post-FH cancellation observation before the next ingress cycle.

Persistent shutdown must not abort the Tokio task or drop the FL future to bypass these boundaries.

## 17. Duplicate-active barrier remains before new work

Before AJ starts, the existing expected-device duplicate-active check remains authoritative.

After AJ success, insertion uses the authenticated `DeviceId` and the exact same active map.

Because the supervisor admits at most one AJ future at a time and does not poll a second expected request while that AJ is in flight, an occupied post-authenticated key after a successful preflight is an internal invariant contradiction rather than a selected normal rejection path.

FT does not invent post-auth mismatch peer cleanup or a second duplicate-rejection family.

## 18. Post-auth invariant failure is not a peer-disposition gate

A future source checkpoint may preserve an internal assertion/unreachable invariant for an impossible occupied post-auth key under the exact single-in-flight scheduling law.

FT does not authorize automatic close, reuse, restart, requester cleanup, or key rewriting to recover from that contradiction.

If a real source contradiction proves the invariant can occur, work must stop at a new semantic gate.

## 19. Ready-completion-first ordering is preserved

The repeated supervisor continues to reap ready active workers before same-wake shutdown or expected-request polling.

While one AJ future is in flight, ready active workers are still reaped before the shutdown-vs-AJ decision is polled.

For requester-aware entries, reaping means the full FS terminal custody sequence:

**join terminal -> detach active entry -> recover exact owner -> preserve/map exact result -> publish ownership-bearing completion**.

## 20. FS owner recovery is reused, not duplicated

Future source materialization must reuse the exact FS recoverable owner-cell and terminal reaping law.

It must not create a second independent owner-recovery algorithm in the repeated real-admission module.

Only the minimum executor-hierarchy visibility/re-export required to reuse the FS requester-aware entry/completion/reaping/cancellation/drain primitives is selected.

FT does not select public exposure of generic owner-cell internals, mutex handles, raw join handles, or raw Tokio errors.

## 21. Ownership-bearing completion replaces capability-only completion for this path

A requester-aware repeated-admission worker completion must own:

- exact authenticated logical `DeviceId`;
- exact recovered `AuthenticatedRemoteSessionRuntimeOwner`;
- exact `Result<RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

The existing capability-only `RemoteSessionRegisteredWorkerCompletion` does not satisfy this law because it does not return authenticated-session owner custody and carries the capability-worker stop taxonomy.

FT therefore selects a distinct requester-aware completion boundary for the integrated path.

## 22. Public-boundary hygiene may use an opaque completion envelope

Because the existing repeated supervisor methods are public while exact FL stop taxonomy remains crate-internal, future source materialization may introduce one public opaque requester-aware completion carrier solely to keep the method signature well-formed.

That envelope must retain the exact FS completion fields internally without translation.

Crate-internal higher owners must be able to recover the exact `DeviceId`, exact session owner and exact FL/join result by value.

FT does not select public exposure of FL internals, panic payloads, raw Tokio join errors, owner-cell handles, or requester provider internals.

## 23. Exact FL stop taxonomy is preserved

Normal requester-aware completion preserves exact FL stop categories unchanged:

- `Cancelled`;
- `Failed(Ingress(...))`;
- `Failed(RequesterResponse(Frame(...)))`;
- `Failed(RequesterResponse(ResponseIo(...)))`.

A normal FL `Failed(...)` remains a normal Tokio join result.

It is not converted to AJ failure, capability worker failure, endpoint shutdown, requester cleanup, or peer-close result.

## 24. Abnormal join remains bounded

Abnormal Tokio task completion maps only to existing:

`RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`.

Exact session-owner recovery precedes completion publication.

No synthetic FL stop, raw `JoinError`, panic payload, Tokio task ID, retry token, or replacement worker is exposed.

## 25. Recovered peer remains retained-stopped

A requester-aware worker terminal result does not automatically close the recovered authenticated peer.

This remains true for:

- FL `Cancelled`;
- FL typed failure;
- abnormal Tokio completion;
- ordinary ready-worker reaping;
- active-worker cancellation during orderly supervisor shutdown.

Capability code 3/code 4 behavior is not widened onto FL completion.

Possession of recovered owner custody does not authorize peer reuse, restart, next ingress, or requester cleanup.

## 26. Existing AJ failure handling remains unchanged

An AJ `Err(RemoteSessionRealAdmissionError)` continues to publish the existing `RemoteSessionRepeatedAdmissionFailure` containing the pre-auth expected logical `DeviceId` and exact AJ error.

AJ failure never constructs a requester-aware worker entry.

Requester-aware policy and requester/rendezvous authority are not consulted merely because AJ failed.

## 27. Shutdown before a new request remains unchanged

If supervisor shutdown is ready before polling a new expected request:

- no new AJ attempt starts;
- no new worker is spawned;
- all currently active requester-aware entries receive cooperative cancellation;
- the exact retained active entries are drained to owner-bearing completion.

Prequeued expected requests are not reclassified as rejection or admission failure merely because shutdown won.

## 28. Shutdown during in-flight AJ preserves AJ custody

The existing shutdown-first race around one in-flight AJ remains authoritative.

If shutdown wins while AJ is pending:

1. request cooperative cancellation for every already-active requester-aware worker;
2. retain and continue polling the same AJ future rather than dropping it;
3. while AJ drains, continue reaping active requester-aware workers through exact FS owner recovery;
4. if AJ fails, publish the existing AJ failure;
5. if AJ succeeds after shutdown latched, do **not** spawn or insert a requester-aware worker;
6. preserve the existing orderly-shutdown handling for that newly authenticated AJ-success owner;
7. drain all previously active requester-aware entries to exact ownership-bearing completion before supervisor return.

FT does not reinterpret the post-shutdown AJ-success close as a worker-completion peer-disposition policy.

## 29. Existing post-shutdown AJ success close remains narrow

Current repeated admission already closes an AJ-success owner through `close_for_orderly_shutdown()` when shutdown latched before AJ completed.

FT preserves that existing boundary unchanged because the session was never inserted as an active requester-aware worker.

This does not authorize closing owners recovered from terminal requester-aware workers.

No new close code or close reason is selected.

## 30. Cancellation-all-then-drain remains authoritative

When supervisor shutdown begins with active requester-aware workers:

- admission stops;
- cancellation is requested for every still-active entry before any retained handle is intentionally discarded;
- no task is aborted;
- the same active map remains driven;
- each terminal entry is detached;
- each exact owner is recovered;
- each exact FL/join result is preserved;
- each ownership-bearing completion is published;
- supervisor return occurs only after the active map is empty.

## 31. Active capacity law remains unchanged

The existing `MAX_REGISTERED_DEVICES` capacity validation remains authoritative.

FT selects no requester-specific second active-worker ceiling.

Terminal entries stop consuming active capacity after detachment.

Recovered retained-stopped completion custody is not an active scheduler tombstone.

## 32. Expected-request source closure remains unchanged

Closing the expected-request source alone does not cancel active requester-aware workers and does not terminate the repeated supervisor.

Explicit supervisor shutdown remains the orderly termination authority.

## 33. No DeviceId tombstone or quarantine

FT does not add a permanent active-key reservation after requester-aware completion.

A terminal active-map vacancy is a scheduler fact only.

It is not authority to reuse the recovered peer.

A later same-`DeviceId` session must arrive through a separately authoritative expected-device AJ admission and logical authentication transaction.

## 34. Existing endpoint lifecycle remains separately authoritative

FT does not alter the existing whole-endpoint shutdown contract that executes after the repeated supervisor has fully returned in the existing endpoint lifecycle wrapper.

No requester-aware worker completion is converted into an endpoint-wide close request.

Global endpoint teardown at supervisor lifecycle termination remains distinct from per-worker peer disposition.

## 35. Requester-record lifecycle remains closed

FT selects no requester/rendezvous record retirement, removal, rollback, reset, TTL cleanup, cancellation cleanup, abnormal-join cleanup, completion cleanup, or shutdown cleanup.

A future requester lifecycle gate must decide those semantics explicitly.

## 36. Candidate/reachability continuation remains closed

FT performs no candidate query, candidate selection, reachability evaluation, endpoint choice, direct/relay decision, target dial, forwarding, terminal activation, or rendezvous-completion claim.

A successful requester acknowledgement remains only the already-selected requester transaction result; this checkpoint does not continue it into data-plane establishment.

## 37. No listener/bootstrap/readiness activation

FT does not wire the requester-aware repeated-admission path into production listener startup, Agent process bootstrap, readiness publication, Android behavior, desktop UI behavior, deployment, restart/recovery, or merge.

The checkpoint remains below activation.

## 38. Source materialization boundary for the next checkpoint

A future source-materialization checkpoint may change only the narrow integration surfaces required to realize this selection, expected to include:

- the repeated real-admission executor module;
- the minimum FS/FQ module visibility or requester-aware helper façade needed to reuse exact recoverable entry/reaping/drain custody;
- the requester-aware completion carrier required by the repeated supervisor signature;
- exact focused tests for AJ-success requester-aware spawn, owner recovery, duplicate/preflight ordering and shutdown/AJ coexistence;
- the FT/FU contract artifact itself.

It must not modify AJ authentication semantics, FL semantics, requester-policy semantics, requester-authority provider semantics, candidate/reachability logic, peer-disposition logic, requester cleanup, listener/bootstrap activation, deployment, or merge.

## 39. Materialization must preserve capability-only historical seams unless required

The existing generic/capability persistent collection and its capability completion type are not selected for deletion by FT.

The future integration may stop using capability `spawn_registered_worker(...)` inside the repeated requester-aware real-admission path, but unrelated capability-only test seams must remain byte-stable unless compilation requires a narrowly justified adjustment.

No broad executor refactor is selected.

## 40. Focused validation requirements for the future source checkpoint

Future source materialization should prove at minimum:

1. AJ success spawns exactly one requester-aware FL worker, not the capability worker;
2. active key comes from authenticated owner `DeviceId`;
3. duplicate expected `DeviceId` is rejected before timing/AJ work while active;
4. exact owner is recovered after normal FL completion;
5. exact owner is recovered after abnormal join;
6. exact FL stop is preserved without translation;
7. ready completion is processed before same-wake shutdown/request admission;
8. shutdown cancels all active requester-aware workers then drains the same handles;
9. shutdown during AJ drains AJ rather than dropping it;
10. post-shutdown AJ success is not inserted as a worker;
11. requester-aware policy source and shared requester authority remain shared single-source handles;
12. no requester cleanup, peer reuse, candidate/reachability, dialing or activation appears in the diff.

## 41. FT acceptance criteria

FT can close only if:

- exact FS is the merge base;
- FT changes only this semantics-selection contract;
- no Rust, Android, Cargo, lockfile, workflow, deployment or runtime source changes occur;
- exact source guards above remain byte-stable;
- permanent Rust validation passes on the exact FT head;
- Android validation is not claimed unless the repository actually runs it for this docs-only head;
- immutable Drive audit raw readback is byte-exact;
- the FT PR remains open, draft and unmerged with semantic `Status: CLOSED`.

## 42. Canonical closure target

`CLOSED_REPEATED_REAL_ADMISSION_REQUESTER_AWARE_PERSISTENT_FL_CUSTODY_INTEGRATION_SEMANTICS_SELECTION`

## 43. Canonical gate target

`C03E_FT_REPEATED_REAL_ADMISSION_REQUESTER_AWARE_PERSISTENT_FL_CUSTODY_INTEGRATION_SEMANTICS_SELECTED`

## 44. Next separately gated checkpoint after successful FT closure

The next checkpoint is:

**C03e-FU — repeated real-admission requester-aware persistent FL custody source materialization**.

FU may materialize only the FT-selected integration law and focused ownership/scheduling tests.

Peer disposition, requester-record cleanup, candidate/reachability continuation, target dialing, listener/bootstrap/readiness activation, deployment, restart/recovery, and merge remain later gates.
