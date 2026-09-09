# C03e-NX — Production requester/rendezvous expected-device scheduling-authority higher-owner result custody and peer disposition selection

Status: **STAGING — SELECTION ONLY — SOURCE MATERIALIZATION BLOCKED**

## 1. Gate

`C03E_NX_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_HIGHER_OWNER_RESULT_CUSTODY_PEER_DISPOSITION_SELECTED`

Closure token reserved for validated evidence closure:

`CLOSED_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_HIGHER_OWNER_RESULT_CUSTODY_PEER_DISPOSITION_SELECTION`

## 2. Authoritative closed predecessor

This checkpoint is rooted exactly at the closed C03e-NW source-materialization boundary.

Exact C03e-NW head:

`b347da2aaaef8572256b8ebfc591adad7c5e91bd`

Exact C03e-NW tree:

`7ad6f017a86555dfb008db37413f23364450aaff`

Exact C03e-NW source blob:

`d1a68ea88d6721a622e0fd8ec54ef23ab136726f`

C03e-NW PR:

`#511`

C03e-NW durable evidence:

`C03E_NW_PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_PARALLEL_CALLER_SOURCE_MATERIALIZATION_AUDIT_2026-09-09.txt`

Drive ID:

`1slGxFuPJNEENty_1U61odaZwQ3WnXy7y`

Exact raw bytes:

`16481`

SHA-256:

`38a361cc3d7c3018334d6623c187395f1726dd396d6abbf27a6b81975ad8bf43`

C03e-NW is the only authoritative predecessor for this selection. Earlier C03e-NU blocker evidence remains explanatory history only.

## 3. Fresh successor authority check

Immediately before C03e-NX branch creation:

- C03e-NW still resolved exactly to `b347da2aaaef8572256b8ebfc591adad7c5e91bd`;
- its tree still resolved exactly to `7ad6f017a86555dfb008db37413f23364450aaff`;
- PR #511 remained draft/open/unmerged and retained its validated/evidence-recorded/closed body;
- no branch matching `phase-152-c03e-nx-*` existed;
- repository code search returned no `C03E_NX` occurrence;
- this exact contract path did not exist at the C03e-NW head.

The token `NX` is selected here as a new documentation checkpoint only after those checks. It is not inferred merely from alphabetic succession.

## 4. Closed C03e-NW semantic result

C03e-NW materialized one dormant sibling requester worker:

`run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_scheduling(...)`

Its new production-specific worker stop preserves three terminal classes:

- cancellation before a scheduling result exists;
- historical ingress/requester-response failure before a scheduling result exists;
- `SchedulingTerminal(...)` after successful requester/rendezvous DR.

The scheduling-terminal payload preserves two orthogonal channels by value:

1. `Result<ExpectedDeviceSchedulingAuthorityGrant, ExpectedDeviceSchedulingAuthorityDerivationError>`;
2. `Result<(), RequesterRendezvousTerminalDrAcknowledgementResponseCompositionError>`.

The scheduling grant remains non-`Copy` and non-`Clone`. C03e-NW does not migrate any higher owner to this sibling.

## 5. Fresh exact-head higher-owner source audit

All source findings in this checkpoint bind to exact C03e-NW head `b347da2aaaef8572256b8ebfc591adad7c5e91bd`.

### 5.1 Requester scheduling-terminal source

Path:

`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

Exact blob:

`d1a68ea88d6721a622e0fd8ec54ef23ab136726f`

Finding:

- `RequesterRendezvousProductionDurableSchedulingWorkerStop` is already `pub(super)`;
- `SchedulingTerminal(...)` owns the new terminal carrier;
- the carrier itself and its accessors are currently local/private to the requester module;
- higher-owner code can preserve the outer stop as an opaque by-value terminal result, but an acknowledgement-sensitive owner-disposition classifier cannot inspect the terminal acknowledgement channel without a narrowly selected visibility bridge.

### 5.2 Recoverable spawned / higher-owner disposer

Path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`

Exact blob:

`cb1f11d97a42a1ab754230696d2dbcc0860c128e`

Finding:

- historical completion custody is fixed to `RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`;
- historical peer disposition is exactly two-way:
  - `Cancelled` -> `OrderlyShutdown`;
  - historical `Failed(...)` or abnormal join -> `TerminalFailure`;
- the existing disposer consumes the recovered authenticated-session owner before returning the historical terminal result;
- this file is the existing semantic locus for requester-aware peer/session-owner disposition and is therefore selected for the scheduling-specific parallel disposer.

### 5.3 Repeated real-admission persistent integration

Path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`

Exact blob:

`eaec3c982353f776e60473e1393ae751d3bf6762`

Finding:

- the existing production-durable spawn helper calls the historical production-durable requester worker;
- its active-map specialization is fixed to the historical worker stop;
- ready reaping/draining delegates to generic custody machinery;
- a parallel scheduling-specific specialization can be defined locally without modifying the generic primitive.

### 5.4 Production-durable repeated/endpoint collection

Path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`

Exact blob:

`297cf49b235537cf9a934eca82ef30e94364eba1`

Finding:

- the existing dormant production-durable collection calls the historical production-durable spawn helper;
- its completion callback receives the historical completion envelope;
- its endpoint lifecycle consumes that envelope through the historical FW disposer before invoking its boundary-safe callback;
- this is the transitive owner boundary where a scheduling-specific parallel collection/callback surface must terminate for this propagation gate.

### 5.5 Generic persistent primitive remains sufficient

Path:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/recoverable_persistent_requester_rendezvous_worker.rs`

Exact blob:

`264d18d57aafe9c6f67683843ded656e40d2d8cb`

Finding:

- `RecoverablePersistentWorkerEntry<O, T>` is generic over terminal result `T`;
- `RecoverablePersistentWorkerCompletion<K, O, T>` is generic over terminal result `T`;
- reap, cancellation and drain machinery is generic over `T`;
- only historical aliases fix `T` to the historical stop.

Selection consequence:

**this generic primitive file is explicitly not selected for source modification by the immediate source successor.**

## 6. Selected authenticated-session owner disposition law

`SchedulingTerminal(...)` is a new terminal class and requires an explicit owner-disposition law before higher-owner propagation can be materialized.

The selected law is based only on requester acknowledgement disposition after successful requester/rendezvous DR:

| scheduling worker terminal result | selected owner disposition |
| --- | --- |
| `Cancelled` | existing orderly-shutdown close seam |
| `Failed(...)` | existing requester-aware terminal-failure close seam |
| abnormal spawned/join failure | existing requester-aware terminal-failure close seam |
| `SchedulingTerminal { acknowledgement_result: Ok(()), ... }` | existing orderly-shutdown close seam |
| `SchedulingTerminal { acknowledgement_result: Err(_), ... }` | existing requester-aware terminal-failure close seam |

Scheduling derivation success or failure **must not affect peer/session-owner disposition**.

Rationale:

- requester/rendezvous DR has already succeeded before `SchedulingTerminal` exists;
- scheduling derivation is local scheduling-authority state and is not peer authorization;
- derivation failure must not rewrite a successful requester/rendezvous registration as a peer failure;
- a successful requester acknowledgement permits orderly terminal close after the selected one-shot scheduling handoff;
- acknowledgement framing/I/O failure preserves the existing requester-response failure meaning at the owner boundary and therefore selects the existing requester-aware terminal-failure close seam;
- no new close code, peer-visible scheduling frame, or scheduling-specific wire status is selected.

## 7. Selected propagation strategy

A parallel scheduling-aware higher-owner path is selected.

The immediate source successor must **not** widen or replace historical shared completion/alias/function signatures merely to carry the non-Clone scheduling grant.

Selected strategy:

1. retain all historical stop/completion/collection surfaces unchanged;
2. add narrowly named production-scheduling-specific custody aliases/envelopes/functions beside the historical surfaces;
3. move `RequesterRendezvousProductionDurableSchedulingWorkerStop` upward by value exactly once through each owner layer;
4. keep the inner scheduling-terminal result intact while owner disposition occurs;
5. invoke the boundary-safe scheduling-specific endpoint callback only after authenticated-session owner disposition;
6. return no owner/reuse/restart capability from that callback boundary.

## 8. Selected exact source ceiling for the immediate successor

The immediate source materialization may modify exactly these four Rust source paths:

1. `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
2. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`
3. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`
4. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`

No fifth source path is authorized.

If implementation requires modification of the generic persistent primitive, Cargo/lockfile/workflow/Android/packaging/runtime-bootstrap code, or any other source path, the successor must STOP and record a blocker instead of broadening scope.

## 9. Selected requester-module visibility bridge

Within source path 1, the immediate successor may make only the minimum visibility change required for the higher-owner disposition classifier to observe acknowledgement disposition while preserving the terminal payload by value.

Selected permission:

- expose the scheduling-terminal carrier to the parent/sibling scope no more broadly than `pub(super)`;
- expose only the accessor or consuming decomposition needed to observe/preserve the acknowledgement channel and scheduling result;
- keep fields private;
- do not alter scheduling derivation order, acknowledgement construction/I/O order, cancellation law, or terminal return semantics;
- do not add request construction/send behavior.

No authority is widened by this visibility change.

## 10. Selected higher-owner custody shapes

Within source path 2, the successor may materialize only scheduling-specific parallel custody/disposition shapes needed to carry:

`Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>`

with the exact recovered authenticated-session owner and authenticated `DeviceId`.

The scheduling-specific disposer must:

1. consume the exact owner once;
2. classify `Cancelled`, historical pre-scheduling `Failed`, abnormal join, and `SchedulingTerminal` without flattening them;
3. for `SchedulingTerminal`, inspect only requester acknowledgement disposition for peer close selection;
4. ignore scheduling derivation success/failure for peer close selection;
5. close through one of the two already-existing owner close seams;
6. return authenticated `DeviceId` plus the exact unchanged scheduling stop/join result by value;
7. return no peer/owner/restart/requester-cleanup authority.

Historical FW completion/disposer shapes must remain unchanged.

## 11. Selected persistent specialization

Within source path 3, the successor may add only a scheduling-specific production-durable specialization over the already-generic persistent primitive.

The selected specialization must:

- use `RequesterRendezvousProductionDurableSchedulingWorkerStop` as terminal result `T`;
- call `run_requester_rendezvous_post_terminal_response_serial_lifecycle_worker_with_production_durable_scheduling(...)` only in the new scheduling-specific spawn helper;
- retain exact owner-cell recovery, cancellation-controller custody, active-map `DeviceId` keying, ready-reap ordering and drain semantics;
- publish a scheduling-specific repeated-admission completion envelope without loss or clone of the scheduling grant;
- leave the historical production-durable spawn helper and historical aliases unchanged.

The generic persistent primitive file must remain unchanged.

## 12. Selected production-durable repeated/endpoint propagation

Within source path 4, the successor may add one parallel scheduling-aware production-durable repeated-admission collection/lifecycle surface that preserves the existing dormant production-durable supervisor law while changing only terminal result custody to the scheduling-specific path.

It must preserve:

- existing expected admission request source semantics;
- duplicate-active-device rejection;
- exact AJ transaction and authentication behavior;
- authenticated `DeviceId` active-map keying;
- existing timing input semantics;
- existing shutdown race handling;
- cancellation/drain law;
- endpoint close + idle-drain law where applicable;
- historical callbacks and functions unchanged.

The scheduling-specific endpoint callback may receive only:

- authenticated `DeviceId`;
- exact `Result<RequesterRendezvousProductionDurableSchedulingWorkerStop, RemoteSessionSpawnedWorkerJoinError>` after owner disposition.

It may not receive authenticated-session owner custody.

## 13. Non-Clone / exact-transfer law

Across the selected higher-owner path, a successful `ExpectedDeviceSchedulingAuthorityGrant`:

- moves by value exactly once through each enclosing result/completion;
- is never cloned;
- is never copied;
- is never reconstructed;
- is never reminted;
- is never replaced by a side-channel token;
- is never hidden behind `Arc`, global/static storage, or a second ownership lane;
- survives requester acknowledgement failure;
- survives owner disposition and reaches the boundary-safe callback inside the exact scheduling-terminal result.

The exact typed scheduling derivation error is preserved by the same law when derivation fails.

## 14. Historical compatibility law

The immediate successor must leave historical behavior/signatures byte- or semantic-equivalent for at least:

- `RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop` and its Copy/Clone semantics;
- historical requester lifecycle workers;
- existing production-durable non-scheduling requester worker;
- `RecoverableSpawnedRequesterRendezvousWorkerCompletion`;
- `RecoverableRepeatedRealAdmissionRequesterAwareWorkerCompletion`;
- historical requester-aware peer disposition classifier/disposer;
- historical `RecoverableRequesterAwareWorkerEntry` and completion aliases;
- existing production-durable repeated-admission collection;
- existing production-durable endpoint lifecycle.

No historical caller is silently migrated by this source checkpoint.

## 15. Explicitly blocked work

This selection does **not** authorize:

- construction of `RemoteSessionExpectedDeviceAdmissionRequest` from the scheduling grant;
- expected-device scheduling sender/channel ownership or send;
- target admission `SessionId` selection or generation;
- authentication/PRWM request-ID selection or generation;
- new admission timing selection or timing source;
- peer-visible scheduling-specific response/error frame;
- scheduling-grant retry, replay, remint, persistence or timer;
- requester-record cleanup;
- candidate/reachability continuation;
- dial target selection;
- listener/bootstrap/readiness caller migration;
- runtime activation;
- Cargo/lockfile/workflow modification;
- Android source/packaging change;
- deployment;
- merge;
- branch deletion;
- repository configuration or permissions changes.

## 16. Validation requirements for the immediate source successor

The source successor must prove at minimum:

1. exact diff is limited to the four selected source paths;
2. the generic persistent primitive file remains unchanged at blob `264d18d57aafe9c6f67683843ded656e40d2d8cb` unless a fresh blocker is recorded instead of closure;
3. historical stop retains Copy/Clone and historical variants/signatures remain unchanged;
4. new scheduling-specific persistent entry carries the scheduling stop without clone/copy/reconstruction;
5. ready reap and drain recover the exact authenticated-session owner before publication;
6. `Cancelled` selects orderly shutdown;
7. pre-scheduling `Failed(...)` selects terminal failure;
8. abnormal join selects terminal failure;
9. scheduling terminal + acknowledgement success selects orderly shutdown;
10. scheduling terminal + acknowledgement failure selects terminal failure;
11. the two scheduling-terminal tests above produce identical owner disposition for scheduling derivation success and derivation failure when acknowledgement disposition is held constant;
12. owner disposition completes before the scheduling-specific endpoint callback;
13. callback receives the exact unchanged scheduling/join terminal result and no owner;
14. no expected-device request is constructed or sent;
15. no new sender/channel, SessionId, PRWM request ID, timing source or runtime activation is introduced;
16. exact-head Rust validation passes before closure;
17. any workflow reported `SKIPPED` is recorded as `SKIPPED`, never `PASS`.

## 17. Immediate successor boundary

After this selection closes, the next source step is separately gated as:

`PRODUCTION_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_SCHEDULING_AUTHORITY_HIGHER_OWNER_RESULT_CUSTODY_PEER_DISPOSITION_SOURCE_MATERIALIZATION`

A likely branch token after a fresh authority/concurrency check is `C03e-NY`, but this document does not pre-create, pre-authorize by name alone, or assume such a branch exists.

The source successor must re-read the exact closed C03e-NX head and every selected target blob before mutation.

## 18. Closure discipline

This C03e-NX document is selection-only.

Before declaring this selection closed:

- exact branch head/tree/contract blob must be frozen;
- compare against exact C03e-NW must prove one docs path only;
- exact-head validation must be recorded according to workflows actually observed;
- a durable `.txt` audit must be published under canonical Drive evidence parent `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
- raw Drive readback must match frozen bytes and SHA-256 exactly;
- exact-title canonical-parent post-search must resolve exactly one artifact;
- branch/PR must be re-read after evidence publication;
- PR must remain draft/open/unmerged;
- no source/runtime/configuration/deployment mutation may occur in this checkpoint.

`STOP`
