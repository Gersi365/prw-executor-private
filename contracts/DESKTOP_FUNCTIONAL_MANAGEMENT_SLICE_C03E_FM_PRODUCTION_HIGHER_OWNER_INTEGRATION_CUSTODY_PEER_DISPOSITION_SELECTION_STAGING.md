# Phase 152 C03e-FM — Production/Higher-Owner Integration Custody and Peer Disposition Selection (Staging)

Status: SEMANTICS_SELECTION_STAGING

## 1. Purpose

C03e-FM selects only the higher-owner custody and authenticated-peer disposition law around the already-materialized isolated C03e-FL mixed-family cancellation-aware serial lifecycle worker.

FM does not integrate FL into the production persistent worker collection. It does not add synchronization, spawn a mixed-family worker, change listener/admission behavior, close a peer, select a new close code, activate candidate/reachability continuation, dial target traffic, deploy, restart, recover, or merge.

## 2. Exact predecessor

FM is based exactly on C03e-FL:

- predecessor branch: `phase-152-c03e-fl-higher-owner-mixed-family-cancellation-aware-serial-lifecycle-worker-source-materialization-staging`
- predecessor head: `bd80bd8c77ef2f94889df369c7065a19a0b12878`
- predecessor tree: `00dcb00bd6c803641f941fb948713298e9be1e2c`
- predecessor semantic status: `CLOSED`
- predecessor gate: `C03E_FL_HIGHER_OWNER_MIXED_FAMILY_CANCELLATION_AWARE_SERIAL_LIFECYCLE_WORKER_SOURCE_MATERIALIZED`

C03e-FL remains frozen.

## 3. Exact audited source guards

FM relies on these exact FL-head source blobs and does not mutate them:

1. FL requester-aware serial lifecycle / cancellation-aware worker
   - `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
   - blob `2a4bcbf48965b8ef5fa3202b3bb3ef46b3f96f31`

2. authenticated-session owner and historical capability-only worker/close precedent
   - `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`
   - blob `083bf83fd1827f6175c9eb62ff93b40147fa9271`

3. existing executor / spawned / supervised / persistent collection ownership
   - `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`
   - blob `47c41735de3c153cde8794b46479e09da7cfba18`

4. process-local requester/rendezvous authority runtime owner
   - `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs`
   - blob `68ba74e82cf703664b7ee090a10fc1c6cce1609d`

5. process-local requester/rendezvous provider
   - `crates/prw-remote-bridge/src/requester_rendezvous_in_memory_provider.rs`

6. authenticated peer bridge owner
   - `crates/prw-remote-bridge/src/remote_server_transport_runtime.rs`
   - blob `14b774d11c1c123f001580be252eb036329d6d2e`

7. lower QUIC runtime
   - `crates/prw-remote-transport/src/runtime.rs`
   - blob `d03bcf642aeb2576656437a8b3d2ddf148a50e30`

## 4. Audited FL worker ownership facts

The exact FL worker:

- accepts `&mut AuthenticatedRemoteSessionRuntimeOwner` rather than consuming the owner;
- accepts `&mut CandidatePublicationRequesterRendezvousRuntimeOwner` rather than consuming it;
- returns exactly `RequesterRendezvousPostTerminalResponseSerialLifecycleWorkerStop`;
- preserves `Cancelled` separately from `Failed(existing FJ lifecycle error)`;
- performs no whole-peer close on either stop path;
- performs no retry, replay, replacement stream, duplicate acknowledgement, DR rerun, requester-registration retry, candidate/reachability continuation, or runtime integration.

Therefore, at the isolated FL function boundary, both higher-owned mutable owners survive every normal FL terminal stop.

## 5. Audited existing persistent-collection facts

The current capability-only persistent executor path uses a different ownership model:

- `RemoteSessionWorkerAdmission` owns `AuthenticatedRemoteSessionRuntimeOwner` by value;
- persistent admission consumes that owner into one spawned task;
- the task runs the historical capability-only worker;
- completion reporting returns logical `DeviceId` plus worker/join terminal result;
- completion reporting does not return the consumed authenticated-session owner;
- capability-only worker cancellation closes the peer using the existing code-4 diagnostic;
- capability-only request-loop failure closes the peer using the existing code-3 diagnostic.

This model is safe for the existing capability-only worker because the worker itself performs its selected peer close before ownership is released.

It is not automatically valid for FL because FL deliberately performs no peer close.

## 6. Audited requester-authority ownership facts

`CandidatePublicationRequesterRendezvousRuntimeOwner`:

- owns one process-local `InMemoryRequesterRendezvousAuthorityProvider` by value;
- exposes requester registration only through `&mut self`;
- is not clone-backed shared authority;
- exposes no synchronization primitive;
- exposes no task/thread ownership model;
- exposes no raw provider sharing surface.

The underlying provider itself explicitly retains no synchronization primitive.

Therefore, exact FL cannot simply be inserted into multiple concurrent persistent worker tasks while sharing one requester/rendezvous runtime owner without a new synchronization/custody selection.

## 7. Selected integration principle

FM selects:

**borrowed higher-owner custody first; retained-stopped peer disposition on every normal FL stop; no direct FL insertion into the existing spawned/persistent capability collection.**

This selection preserves exact FL semantics while preventing implicit authenticated-peer disposal and unsafely invented requester-authority sharing.

## 8. Selected borrowed higher-owner custody

The first higher-owner integration shape for FL must keep these owners outside the FL future:

- one `AuthenticatedRemoteSessionRuntimeOwner`;
- one `CandidatePublicationRequesterRendezvousRuntimeOwner`.

The higher owner may lend mutable borrows into one FL invocation, but FL must not consume either owner.

The higher owner also retains responsibility for:

- `SharedCurrentCapabilityAuthority`;
- requester-aware policy source custody;
- dispatcher custody;
- verifier-time provider custody;
- cancellation-source provenance.

The first integration source seam should therefore be executor-neutral or a borrowed executor drive seam. It must not require moving the authenticated-session owner into a detached or persistent task merely to invoke FL.

## 9. Selected peer disposition: retained-stopped

For every normal FL terminal stop, FM selects one common immediate authenticated-peer disposition:

**retain the exact `AuthenticatedRemoteSessionRuntimeOwner` under the higher caller, but mark the FL invocation stopped and do not automatically resume or close the peer.**

The retained owner is not equivalent to an active worker.

Retention means only that explicit ownership survives for a later separately selected disposition.

It does not authorize:

- another ingress cycle;
- worker restart;
- session reuse;
- reconnect;
- replacement session;
- endpoint migration;
- candidate/reachability continuation.

## 10. `Cancelled` disposition

When FL returns `Cancelled`:

- the FL invocation is terminal;
- exact authenticated-session owner remains retained by the higher caller;
- exact requester/rendezvous runtime owner remains retained by the higher caller;
- no code-4 capability shutdown close is reused;
- no new mixed-family close is sent;
- no peer drop is intentionally used as a substitute for a selected close policy;
- no worker restart or next ingress is authorized;
- cancellation remains local lifecycle control only.

Cancellation provenance remains relevant to a later gate. FM does not assume every FL cancellation means process shutdown, peer fault, logout, authentication invalidation, or connection failure.

## 11. `Failed(Ingress(...))` disposition

When FL returns `Failed(Ingress(...))`:

- the exact nested ingress failure is preserved unchanged;
- the FL invocation is terminal;
- authenticated-session owner remains retained-stopped;
- requester/rendezvous runtime owner remains retained;
- no code-3 capability termination close is reused;
- no new close code/reason is selected;
- no retry, suppression, worker restart, or next ingress is authorized.

Ingress failure does not by itself select whole-peer retirement because its nested classes include bounded stream/wire/bridge conditions whose connection-wide fatality has not been separately selected for mixed-family traffic.

## 12. `Failed(RequesterResponse(Frame(...)))` disposition

When FL returns requester-response framing failure:

- exact nested FH frame failure is preserved;
- authenticated-session owner remains retained-stopped;
- requester/rendezvous runtime owner remains retained;
- no response retry/fallback is authorized;
- no whole-peer close is selected;
- no next ingress or worker restart is authorized.

Local frame construction failure is not automatically promoted to authenticated-peer failure.

## 13. `Failed(RequesterResponse(ResponseIo(...)))` disposition

When FL returns requester-response I/O failure:

- exact nested FH/FF response-I/O failure is preserved;
- authenticated-session owner remains retained-stopped;
- requester/rendezvous runtime owner remains retained;
- no acknowledgement retry/resend/replacement stream is authorized;
- no automatic whole-peer close is selected;
- no next ingress or worker restart is authorized.

The preserved lower `MeshQuicRuntimeError` may later support a more specific peer-fatality policy, but FM does not infer one from the broad response-I/O family.

## 14. Explicit no-drop-as-policy rule

FM does not select implicit Rust/Quinn handle destruction as an authenticated-peer lifecycle policy.

The existence of a `Drop` path for lower handles is not treated as a protocol-level close decision, diagnostic, retry rule, or peer-retirement classification.

A future by-value/spawned FL integration must therefore not simply discard the session owner when FL stops.

If a future task owns the session owner by value, its normal completion surface must return that owner or consume it through an explicitly selected peer disposition before task completion.

## 15. Spawned/persistent integration barrier

FM explicitly does not authorize replacing the current capability-only worker body in:

- `drive_spawned_capability_request_worker`;
- `drive_supervised_capability_request_worker`;
- `drive_persistent_remote_worker_collection`.

A direct substitution would create at least two unresolved ownership problems:

1. the authenticated-session owner would be consumed into the task while FL itself no longer closes it;
2. multiple worker tasks have no selected safe sharing mechanism for the single mutable process-local requester/rendezvous runtime owner.

These must be solved before persistent mixed-family activation.

## 16. Selected normal completion custody if spawning is later introduced

If a later checkpoint selects an owned/spawned FL worker, normal worker completion must preserve a recoverable custody result conceptually containing:

- exact authenticated logical `DeviceId` key;
- exact FL stop;
- exact `AuthenticatedRemoteSessionRuntimeOwner` by value.

Dispatcher/verifier-time provider recovery is not required by FM unless a later operational policy needs them.

The exact Rust type/name is deferred.

No completion callback may silently flatten FL stop or discard the live authenticated-session owner.

## 17. Abnormal task completion boundary

FM does not claim recoverable authenticated-session ownership after an abnormal spawned-task termination because no such FL spawning surface is selected here.

A later checkpoint that introduces FL task spawning must explicitly decide abnormal-join custody and whether panic/task abort can destroy peer custody without an explicit close.

Until then, abnormal join behavior is outside FM activation scope.

## 18. Requester-authority concurrency barrier

FM selects no `Arc`, mutex, RwLock, actor, channel, per-session provider clone, sharding scheme, or other synchronization primitive for `CandidatePublicationRequesterRendezvousRuntimeOwner`.

The owner remains one higher-owned mutable process-local authority.

Consequently:

- only one borrowed FL invocation may hold its mutable requester-authority borrow at a time;
- production multi-session FL concurrency remains blocked;
- this does not reduce the existing capability-only persistent collection behavior because FM does not activate FL there.

A dedicated synchronization/custody selection must precede concurrent production FL integration.

## 19. No hidden serialization through Tokio current-thread execution

The existing executor uses a current-thread Tokio runtime, but single-thread execution does not by itself solve Rust ownership or authority synchronization.

Multiple spawned futures can interleave on the same thread.

FM therefore does not treat current-thread scheduling as a substitute for explicit requester-authority synchronization/custody.

## 20. Existing capability-only behavior remains unchanged

FM does not change the historical capability-only path:

- capability loop failure may still use its selected code-3 close;
- capability worker cancellation may still use its selected code-4 close;
- orderly capability shutdown may still use its existing code-4 helper;
- current persistent capability worker collection remains unchanged.

FM only prevents those capability-specific peer-close semantics from being silently widened to FL.

## 21. Session identity law

FM preserves:

- authenticated PRW application-session lineage as logical requester identity;
- exact authenticated owner `DeviceId` as logical session key where already selected;
- dynamic IP/port as transient endpoint data only;
- `TransportIdentity` as lower transport evidence only;
- PRWM `request_id` as correlation only.

Worker cancellation, task identity, executor slot, map key ordering, stream ID, endpoint tuple, error class, or completion callback does not become identity.

## 22. Requester authority law

The requester/rendezvous runtime owner remains process-local authority custody, not per-transport or per-worker identity.

Registration state must not be cloned into per-session worker snapshots merely to satisfy task ownership.

No stale cloned provider state is selected.

## 23. No restart/reuse law after FL stop

FM selects no automatic worker restart or same-peer ingress resumption after any FL terminal stop.

A retained-stopped owner may only be reused under a later explicit policy that establishes:

- which stop classes permit reuse;
- whether transport/session state is still valid;
- whether requester authority state requires cleanup/retirement;
- whether new cancellation state is required;
- whether a fresh worker may be started.

Until such a gate closes, retained means custody only.

## 24. No requester-authority cleanup implied by worker stop

FL stop does not automatically retire or remove requester/rendezvous authority records.

FM does not select:

- registration rollback;
- record retirement;
- retired-record removal;
- TTL cleanup;
- session-stop-driven provider mutation.

Requester authority cleanup remains a separate lifecycle policy.

## 25. Candidate/reachability boundary remains closed

FM does not authorize:

- candidate query/selection;
- reachability evaluation;
- endpoint resolution;
- relay selection;
- direct-path attempt;
- target QUIC/TCP establishment;
- port-forward activation;
- terminal activation;
- remote-session establishment;
- rendezvous completion.

Requester acknowledgement `Accepted` remains accepted-for-continuation only.

## 26. Runtime/deployment boundary remains closed

FM does not:

- add an executor drive method for FL;
- spawn an FL task;
- change persistent worker admission/completion types;
- add requester-authority synchronization;
- wire listener/admission lifecycle;
- change process lifecycle control;
- publish readiness;
- alter `main.rs`;
- change Android behavior;
- widen dependencies/workflows;
- package;
- deploy;
- restart/recover;
- merge.

## 27. Selected next source seam

The next source-materialization checkpoint should remain narrow and non-production.

It should materialize only a **borrowed executor drive seam** for FL that:

- borrows `AuthenticatedRemoteSessionRuntimeOwner` mutably;
- borrows `CandidatePublicationRequesterRendezvousRuntimeOwner` mutably;
- reuses exact FL worker once;
- returns exact FL worker stop unchanged;
- preserves both owner objects with the caller after return;
- performs no peer close/drop/restart;
- performs no task spawn/channel/queue;
- does not change persistent collection behavior.

Only after that source seam is validated should a separate gate select requester-authority synchronization and owned/persistent FL integration.

## 28. Explicit non-goals

No Rust source mutation.
No Android mutation.
No dependency/workflow mutation.
No direct persistent FL integration.
No task spawn.
No task abort policy.
No implicit owner drop policy.
No code-3/code-4 widening.
No new mixed-family close code.
No peer close.
No peer restart/reuse.
No requester-authority synchronization primitive.
No requester record cleanup.
No candidate/reachability/endpoint/relay selection.
No target dialing.
No port-forward/terminal/session activation.
No listener/process activation.
No packaging.
No deployment.
No restart/recovery.
No merge.

## 29. Canonical closure target

`CLOSED_PRODUCTION_HIGHER_OWNER_INTEGRATION_CUSTODY_PEER_DISPOSITION_SELECTION`

## 30. Canonical gate target

`C03E_FM_PRODUCTION_HIGHER_OWNER_INTEGRATION_CUSTODY_PEER_DISPOSITION_SELECTED`

## 31. Next separately gated checkpoint

**C03e-FN — borrowed executor drive seam for FL source materialization**.

FN should materialize only the non-spawned borrowed executor integration selected by FM. Requester-authority synchronization, persistent multi-worker FL integration, peer close/reuse policy, candidate/reachability continuation, target dialing, deployment, restart/recovery and merge remain later gates.
