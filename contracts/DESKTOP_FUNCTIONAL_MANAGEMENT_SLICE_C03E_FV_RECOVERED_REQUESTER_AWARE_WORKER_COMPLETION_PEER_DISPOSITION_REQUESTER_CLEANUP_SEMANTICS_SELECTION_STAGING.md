# Phase 152 C03e-FV — Recovered Requester-Aware Worker Completion Peer-Disposition / Requester-Cleanup Semantics Selection

Status: CLOSED

## 1. Purpose

C03e-FV selects only the higher-owner semantics that apply after C03e-FU publishes one exact `RecoverableRepeatedRealAdmissionRequesterAwareWorkerCompletion`.

FV does not materialize Rust source. It selects how the recovered authenticated peer is disposed according to the exact FL/join terminal class and explicitly decides whether requester/rendezvous authority records may be retired or removed from this worker-completion boundary.

The exact C03e-FU source state is the immutable predecessor for this selection.

## 2. Exact predecessor

Canonical predecessor checkpoint:

`C03E_FU_REPEATED_REAL_ADMISSION_REQUESTER_AWARE_PERSISTENT_FL_CUSTODY_SOURCE_MATERIALIZED`

Exact predecessor branch:

`phase-152-c03e-fu-repeated-real-admission-requester-aware-persistent-fl-custody-source-materialization-staging`

Exact predecessor head:

`cf04813db9bd6c633e33e777ca2e1c097362f79c`

Exact predecessor tree:

`082579f4bf3ecb60f650cdab6637df23a4cbba12`

FU remains frozen.

## 3. Source facts that constrain this selection

### 3.1 FU completion custody

FU publishes one completion containing exactly:

- authenticated logical publisher `DeviceId`;
- exact recovered `AuthenticatedRemoteSessionRuntimeOwner`;
- exact `Result<RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

The completion contains no requester session identifier, requester-registration receipt, requester lifecycle token, target-registration handle, candidate set, reachability state, dial target, or restart token.

### 3.2 Exact FL terminal classes

Normal exact FL worker stop is one of:

- `Cancelled`;
- `Failed(Ingress(...))`;
- `Failed(RequesterResponse(Frame(...)))`;
- `Failed(RequesterResponse(ResponseIo(...)))`.

Abnormal Tokio completion remains only:

`RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`.

### 3.3 Existing requester/rendezvous provider lifecycle identity

The existing in-memory provider already supports explicit:

- `retire(requester_session_id, expected_publisher_device_id)`;
- `remove_retired(requester_session_id, expected_publisher_device_id)`.

Therefore requester-record retirement/removal requires the exact composite lifecycle identity:

`(requester SessionId, expected publisher DeviceId)`.

A publisher `DeviceId` alone is not sufficient authority.

### 3.4 Existing provider cardinality

Distinct requester sessions may register distinct current records for the same expected publisher `DeviceId`.

Consequently one publisher worker may have served zero, one, or multiple requester registrations over its serial FL lifetime.

Worker completion cannot infer which requester record or records, if any, should be retired.

### 3.5 Existing peer-close semantics

The authenticated-session owner already has the existing consuming orderly-shutdown close seam using the fixed code-4 shutdown diagnostic.

The capability-only worker separately uses its existing code-3 termination diagnostic on capability-loop failure.

Exact FL deliberately did not automatically widen either close behavior onto requester-aware failure paths.

FV therefore selects the higher-owner disposition law explicitly instead of inheriting capability-only closure accidentally.

## 4. Selected ownership law

A FU requester-aware completion is terminal ownership custody.

The higher owner MUST consume the recovered session owner exactly once through one selected disposition branch.

The completion MUST NOT be dropped while silently relying on peer object destruction as lifecycle semantics.

The completion MUST NOT be converted into a reusable active-worker entry without a separately selected restart/reuse checkpoint.

## 5. Selected terminal-class partition

FV selects three disposition classes:

1. orderly supervisor cancellation;
2. typed requester-aware FL failure;
3. abnormal spawned-task completion.

The exact original FL/join result remains observable to the higher owner until peer disposition has been selected.

No result is flattened into a boolean success/failure flag.

## 6. `Cancelled` disposition

For FU repeated-real-admission custody, `RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop::Cancelled` is produced by the cooperative cancellation path used when the repeated supervisor is shutting down.

FV selects:

- consume the recovered `AuthenticatedRemoteSessionRuntimeOwner`;
- close the retained authenticated peer through the existing orderly-shutdown consuming seam;
- reuse the existing fixed code-4 shutdown diagnostic;
- perform no peer reuse;
- perform no worker restart;
- perform no requester-record retirement/removal from this completion;
- perform no candidate/reachability continuation.

No new cancellation close code is selected.

## 7. Typed FL failure disposition

For:

- `Failed(Ingress(...))`;
- `Failed(RequesterResponse(Frame(...)))`;
- `Failed(RequesterResponse(ResponseIo(...)))`;

FV selects a fail-stop peer disposition.

The recovered authenticated peer MUST NOT return to the active requester-aware collection and MUST NOT be reused for another ingress cycle after the worker has already declared terminal failure.

The higher owner must consume the exact recovered owner through a dedicated requester-aware terminal-failure close seam in the future source-materialization checkpoint.

This dedicated failure seam must use one fixed non-secret terminal diagnostic selected for requester-aware worker failure. It MUST NOT misclassify failure as orderly shutdown.

FV does not allocate a numeric transport close code. Future materialization must verify the repository's existing close-code namespace before choosing the concrete fixed value.

FV does not reuse the existing code-4 shutdown diagnostic for typed failure.

FV also does not silently widen the capability-only code-3 diagnostic without an explicit implementation-time namespace check.

## 8. Abnormal join disposition

For:

`RemoteSessionSpawnedWorkerJoinError::AbnormalTaskCompletion`

FV selects the same fail-stop peer-disposition category as typed FL failure.

Rationale:

- the exact authenticated owner is recoverable;
- worker execution terminated abnormally;
- post-worker peer protocol state cannot be proven safe for reuse;
- restart/recovery has not been selected.

Therefore the recovered peer MUST be consumed through the same dedicated requester-aware terminal-failure close seam selected for typed FL failure.

The abnormal join classification itself remains unchanged and must remain available to the completion consumer before/while disposition is applied.

No panic payload, Tokio task identity, retry, replacement worker, or automatic reconnect is introduced.

## 9. No clean-success peer-reuse branch

Exact FL has no ordinary clean-success terminal stop.

Its normal terminal enum contains only cancellation or typed failure.

FV therefore selects no `CompletedSuccessfully -> reuse peer` branch.

A future new FL success taxonomy would require a new semantics-selection checkpoint before peer reuse could be considered.

## 10. Requester-record cleanup is NOT coupled to publisher worker completion

FV explicitly rejects automatic requester-record retirement or removal from FU worker completion.

The FU completion does not contain the exact requester lifecycle identity required by the provider.

Publisher `DeviceId` is not a requester-record key.

The recovered publisher authenticated session is not a requester-registration receipt.

The worker's terminal FL result is not requester-record authority.

Therefore no FV/FW path may call requester-record `retire` or `remove_retired` based only on:

- publisher `DeviceId`;
- publisher session ID;
- peer transport identity;
- FL terminal result;
- Tokio task identity;
- worker entry identity;
- active-map key;
- completion order.

## 11. Exact requester cleanup authority

Any later requester-record cleanup checkpoint must carry or recover the exact composite identity required by the provider:

`(requester SessionId, expected publisher DeviceId)`.

A later design may choose an exact registration receipt/token if it preserves this identity by construction, but FV does not select or materialize such a token.

Cleanup must remain requester-registration-lifecycle authority, not publisher-worker-completion authority.

## 12. Multi-requester safety

The provider permits distinct requester sessions for one expected publisher.

Therefore a publisher-level cleanup such as "retire all requester records for this publisher" is explicitly rejected.

Such a cleanup could invalidate unrelated current requester authority and would violate the existing exact-record lifecycle API.

FV selects no bulk retirement, wildcard removal, publisher-wide reset, provider reset, or capacity sweep.

## 13. Capacity semantics

Because FV does not retire or remove requester records, provider capacity remains governed by the existing explicit requester-record lifecycle.

FV does not fabricate capacity recovery from publisher worker completion.

If requester records need lifecycle cleanup to avoid permanent capacity consumption, that cleanup must be selected and materialized through a separate exact-requester-key checkpoint.

## 14. Peer disposition happens before completion custody is discarded

Future materialization must preserve this order:

1. receive exact FU completion;
2. inspect/preserve exact FL/join terminal class;
3. select the corresponding FV peer-disposition branch;
4. consume the exact recovered session owner through that branch;
5. only then discard any remaining completion envelope fields.

No peer close is performed before owner recovery.

No owner is cloned to perform disposition.

## 15. No reuse/restart

FV selects no peer reuse and no worker restart for any current FU terminal class.

After cancellation, typed FL failure, or abnormal join, the recovered authenticated peer is terminally disposed.

A new authenticated session, if later desired, must enter through the existing real-admission path as a new ownership event.

FV does not transfer the old peer into a replacement worker.

## 16. No implicit re-admission

Peer disposition does not itself enqueue a new expected-device request.

It does not bypass duplicate-active checks, AJ, authentication, binding, lease validation, or requester-aware policy authority.

No reconnect/retry loop is selected.

## 17. No candidate/reachability continuation from worker completion

FU worker completion is not a rendezvous-success signal.

FV does not interpret:

- worker cancellation;
- typed FL failure;
- abnormal join;
- peer close;

as authority to start candidate publication, candidate selection, reachability evaluation, direct/relay choice, endpoint selection, target dialing, or forwarding.

Those behaviors require separately gated inputs and lifecycle authority.

## 18. No requester-registration success inference from final worker result

One FL worker may have completed many requester DR acknowledgement transactions before later terminal failure or cancellation.

The final worker stop therefore does not identify the outcome of any particular earlier requester registration.

FV does not use final worker stop to retire, roll back, commit, or compensate earlier requester records.

## 19. No compensation rollback

A requester DR registration that already committed and whose acknowledgement path later failed is not automatically rolled back by worker completion.

The current source does not return an exact registration receipt from FL to the higher owner.

FV therefore selects no speculative compensation rollback.

A future exact-record lifecycle design may select compensation only with explicit provenance and idempotence semantics.

## 20. Lock ordering remains unchanged

FV introduces no new requester-authority lock operation.

Existing shared requester authority continues to acquire requester/rendezvous authority first and then nested shared-current authority only for registration composition.

Peer disposition must not occur while a requester-authority guard is held.

Future requester cleanup, if selected separately, must define its own lock-order contract before materialization.

## 21. Failure taxonomy preservation

Future materialization must not rewrite:

- `Ingress(...)` into requester-response failure;
- requester-response `Frame(...)` into `ResponseIo(...)`;
- typed FL failure into abnormal join;
- abnormal join into typed FL failure;
- cancellation into failure;
- failure into cancellation.

Peer disposition is side-effect handling after terminal classification, not result reclassification.

## 22. Dedicated requester-aware terminal-failure close seam

FV selects a future narrow consuming helper on `AuthenticatedRemoteSessionRuntimeOwner`, conceptually:

`close_for_requester_aware_terminal_failure(self)`

The exact name is not normative; the ownership shape is normative.

The seam must:

- consume the owner by value;
- close exactly the retained authenticated peer once;
- use a fixed non-secret requester-aware terminal-failure diagnostic;
- expose no raw peer;
- perform no requester cleanup;
- perform no retry/reconnect;
- perform no session deletion;
- perform no candidate/reachability work.

## 23. Orderly-shutdown seam reuse

The existing `close_for_orderly_shutdown(self)` seam is already sufficient for the selected cancellation branch.

Future materialization should reuse it rather than duplicate code-4 closure.

No change to the existing post-shutdown AJ-success closure semantics is selected.

## 24. Higher-owner completion handler shape

Future materialization should add one narrow completion consumer above FU, not a second worker supervisor.

Conceptually the handler consumes:

`RecoverableRepeatedRealAdmissionRequesterAwareWorkerCompletion`

and performs only FV-selected peer disposition.

It must not own AJ, active-map scheduling, requester registration, candidate selection, target dialing, or runtime activation.

## 25. FU scheduling remains byte-stable

FV does not change:

- one repeated expected-device AJ supervisor;
- one active map keyed by authenticated `DeviceId`;
- ready-completion-first ordering;
- duplicate expected-device preflight;
- at most one in-flight AJ;
- FS owner recovery;
- cancellation-all-then-drain;
- post-shutdown AJ drain;
- post-shutdown never-inserted AJ-success close.

## 26. Existing provider lifecycle methods remain dormant from this boundary

Although the concrete provider already implements `retire` and `remove_retired`, FV does not expose those operations through `CandidatePublicationRequesterRendezvousRuntimeOwner` or `SharedRequesterRendezvousAuthority` for use by FU completion handling.

That exposure would require a separate semantics gate defining exact caller provenance, exact record identity, error handling, idempotence, lock ordering, and lifecycle timing.

## 27. Error handling for peer close

Existing peer close is a local consuming disposition with no retry-result surface selected by the current runtime owner.

FV preserves that bounded model.

Future materialization must not add a peer-close retry loop, reconnect loop, or asynchronous cleanup task merely to dispose a recovered owner.

## 28. Security boundary

FV preserves logical identity authority:

- `DeviceId` comes from the authenticated bound session;
- requester cleanup requires exact requester session identity plus expected publisher `DeviceId`;
- transport identity is never substituted for either logical identity.

No IP address, socket tuple, QUIC connection ID, task ID, Arc address, mutex address, or map slot becomes cleanup or lifecycle authority.

## 29. Privacy / diagnostic boundary

Future requester-aware terminal-failure close reason must be fixed and non-secret.

It must not include:

- user IDs;
- workspace IDs;
- device IDs;
- session IDs;
- requester IDs;
- target IDs;
- policy results;
- exception text;
- panic payloads;
- network addresses.

## 30. Out of scope

FV does not materialize or select:

- requester-record retirement/removal implementation;
- requester cleanup trigger timing;
- requester cleanup idempotence;
- requester cleanup retry;
- requester TTL/clock behavior;
- provider persistence;
- peer reuse;
- worker restart;
- reconnect;
- new AJ behavior;
- new authentication behavior;
- candidate publication;
- candidate selection;
- reachability evaluation;
- direct/relay selection;
- target dialing;
- forwarding;
- listener/bootstrap/readiness activation;
- Android behavior change;
- desktop UI behavior change;
- deployment;
- restart/recovery;
- merge.

## 31. Future source-materialization boundary

The next separately gated source-materialization checkpoint may materialize only:

1. one dedicated requester-aware terminal-failure consuming peer-close seam;
2. one higher-owner FU completion disposition helper/consumer;
3. exact branch mapping:
   - `Cancelled` -> existing orderly-shutdown consuming close;
   - typed FL failure -> dedicated requester-aware terminal-failure consuming close;
   - abnormal join -> dedicated requester-aware terminal-failure consuming close;
4. focused ownership/branching tests.

It must not expose requester cleanup or candidate/reachability behavior.

## 32. Selected closure

C03e-FV is closed as:

`CLOSED_RECOVERED_REQUESTER_AWARE_WORKER_COMPLETION_PEER_DISPOSITION_REQUESTER_CLEANUP_SEMANTICS_SELECTION`

Canonical gate:

`C03E_FV_RECOVERED_REQUESTER_AWARE_WORKER_COMPLETION_PEER_DISPOSITION_REQUESTER_CLEANUP_SEMANTICS_SELECTED`

## 33. Exact next checkpoint

The exact next checkpoint is:

**C03e-FW — recovered requester-aware worker completion peer-disposition source materialization**

FW may materialize only the FV-selected peer-disposition seams and focused tests.

Requester-record cleanup remains separately gated because FU completion does not carry the exact requester lifecycle identity required by the existing provider.