# Phase 152 C03e-AL — Repeated Real Admission + Persistent Collection Supervisor Source Materialization Staging

Status: STAGED

Target gate:

`C03E_AL_REPEATED_REAL_ADMISSION_PERSISTENT_COLLECTION_SUPERVISOR_SOURCE_MATERIALIZED`

## Exact predecessor

Canonical predecessor is closed C03e-AK:

- branch: `phase-152-c03e-ak-repeated-real-admission-persistent-collection-supervisor-selection-staging`
- head: `f065ac299876473ca504d4d443ab774eb3db6991`
- tree: `f41b5cec8b2ed38b976a62e381b1738d01c23dec`
- gate: `C03E_AK_REPEATED_REAL_ADMISSION_PERSISTENT_COLLECTION_SUPERVISOR_SELECTED`

C03e-AL materializes only the C03e-AK-selected repeated expected-device AJ admission integration against the existing C03e-AH persistent worker collection under the same private Tokio current-thread runtime.

## Exact bounded source scope

The final AK -> AL net diff must remain bounded to:

1. this materialization contract;
2. `crates/prw-agent/src/remote_session_capability_runtime.rs` only for bounded re-exports;
3. `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs` only for the selected consuming orderly-shutdown owner close seam;
4. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs` for the repeated real-admission supervisor, request/timing/event ownership types, and focused orchestration tests.

`real_remote_admission_transaction.rs` must remain semantically and byte stable unless compilation proves a concrete contradiction.

No manifest, lockfile, permanent workflow, Android application, bridge, transport, Agent `main.rs`, packaging, host, readiness, deployment, or merge mutation is selected.

## Materialized combined supervisor

The existing `RemoteSessionExecutorRuntime` gains one domain-specific drive seam equivalent in responsibility to:

`drive_repeated_real_remote_admission_collection(...)`

The complete repeated-admission plus active-worker lifetime stays inside exactly one private `Runtime::block_on(async supervisor { ... })`.

The implementation must not nest `block_on`, expose/clone a runtime handle, create a second Tokio runtime, enable `rt-multi-thread`, detach an admission producer, or use `JoinSet` as a second handle owner.

The existing `drive_persistent_remote_worker_collection(...)` behavior remains unchanged.

## Expected-device request ownership

The repeated input is one bounded Tokio mpsc receiver of owned expected-device requests.

Each request owns exactly the pre-auth scheduling/worker-handoff values selected by AK:

- expected logical `DeviceId`;
- typed `SessionId`;
- authentication PRWM request ID;
- owned capability dispatcher `D`;
- owned worker verifier-time provider `T`.

The request contains no caller-supplied `TransportIdentity`, IP-derived logical identity, registry/policy snapshot, authenticated owner, peer, cancellation pair, JoinHandle, runtime handle, or capacity permit.

A rejected duplicate request must remain recoverable intact through a bounded pre-auth rejection surface.

Closing the request source alone is not supervisor shutdown.

## Fresh timing materialization

A bounded timing value/factory is materialized for actual AJ start only.

It supplies exactly:

- challenge-validity `Range<u64>`;
- authentication verifier `now_unix_seconds`;
- application-lease `Range<u64>`.

The timing provider is sampled only after capacity and duplicate-active preflight permit the request to start. Queued requests do not freeze these times.

The resulting timing is transaction input only and never capability authorization evidence.

## Capacity and duplicate preflight

Before polling a new expected request, the supervisor:

1. reaps all ready active workers;
2. polls supervisor shutdown;
3. polls no request while `active.len() >= max_active_workers`.

After receiving one request and before constructing AJ:

- derive only the request's expected logical DeviceId for scheduling preflight;
- if that DeviceId already exists in the active map, reject the untouched request;
- do not accept transport, create a pending session, sample timing, cancel/replace the existing worker, or spawn a worker.

The active map length remains the only active-worker capacity authority.

No semaphore, atomic reservation counter, second capacity map, or admission lease is introduced.

At most one AJ transaction exists at a time.

## One in-flight AJ transaction

When capacity exists and pre-auth duplicate preflight passes:

1. sample fresh admission timing;
2. split the request ownership into AJ inputs plus retained worker dispatcher/verifier ownership;
3. construct exactly one existing `admit_expected_remote_device_session(...)` future;
4. continue driving active workers and that exact AJ future on the same current-thread runtime.

No second expected request is polled until the in-flight AJ transaction reaches terminal completion.

## Deterministic normal poll order

While one AJ future is in flight and shutdown has not latched, each supervisor wake must preserve:

1. reap ready active workers;
2. poll supervisor shutdown;
3. only if shutdown remains pending, poll the in-flight AJ future.

Thus shutdown wins a same-wake tie against a ready AJ completion for the spawn/no-spawn decision.

Worker completion remains first so already-completed worker ownership is reaped before lifecycle decisions.

## Ordinary AJ failure

If AJ returns `Err(RemoteSessionRealAdmissionError)` before shutdown:

- report one bounded failure event containing the expected logical DeviceId and the exact original AJ error;
- consume the failed request;
- add no peer/session cleanup;
- perform no retry/reconnect/replacement;
- continue the repeated supervisor.

AJ and its delegated existing transactions remain sole cleanup owners for codes 1, 2, and 5 and pending-session abort.

## Ordinary AJ success

If AJ succeeds before shutdown:

1. retain the returned `AuthenticatedRemoteSessionRuntimeOwner`;
2. re-derive the collection key from that authenticated owner, not the pre-auth request;
3. combine the owner with the request-owned dispatcher and worker verifier-time provider;
4. create exactly one existing C03e-AD cancellation pair;
5. spawn exactly one existing C03e-S worker;
6. retain exactly one cancellation controller and JoinHandle in the active map.

The AJ contract guarantees that the successful authenticated owner corresponds to the expected logical DeviceId; the implementation must still derive the post-auth key from the owner.

No authorization result is reused across worker requests.

## Orderly shutdown without in-flight AJ

When shutdown latches with no AJ transaction in flight:

1. stop polling expected requests permanently;
2. request cancellation for every retained active worker before waiting for any one worker;
3. continue driving/reaping all retained workers;
4. return only after the active map is empty.

Queued expected requests may be dropped because they own no accepted peer or pending logical session.

The real transport endpoint itself is not closed by this supervisor.

## Orderly shutdown with in-flight AJ

When shutdown becomes ready while AJ is in flight:

1. latch shutdown immediately;
2. stop expected-request polling permanently;
3. request cancellation for all currently active worker controllers before worker drain;
4. do not drop, abort, cancel, replace, or fabricate failure for the AJ future;
5. keep polling active-worker completion and the same AJ future under the same current-thread runtime until AJ reaches its existing terminal result;
6. never start another AJ attempt after shutdown.

There is no new hard drain deadline.

### AJ terminal failure after shutdown

If the drained AJ returns its existing error:

- report that exact admission failure;
- add no cleanup;
- continue draining already-active workers.

### AJ success after shutdown

If the drained AJ succeeds after shutdown latched:

- do not spawn or insert a worker;
- consume the authenticated owner through the selected orderly-shutdown owner-close seam;
- close the retained peer exactly once with existing code 4 / `remote capability session shutdown`;
- perform no capability request poll, authorization, authenticated-session deletion, retry, or replacement;
- continue draining previously active workers.

## Authenticated owner close seam

`AuthenticatedRemoteSessionRuntimeOwner` materializes one Agent-internal consuming method equivalent to:

`close_for_orderly_shutdown(self)`

This method reuses the existing fixed C03e-S shutdown diagnostic:

- code `4`;
- reason `remote capability session shutdown`.

It exists only for an AJ-success owner obtained after supervisor shutdown had already latched.

It performs no task spawn and no request-loop work.

## Active-worker semantics preserved

Existing active workers retain all C03e-AH/C03e-S semantics:

- every retained controller requested before drain;
- request loop Q polled before cancellation inside each worker;
- Q failure preserves code 3 and the original typed failure;
- cancellation while Q pending preserves code 4 and `Cancelled`;
- no hard abort;
- completion accounting retains authenticated DeviceId plus existing bounded worker/join result.

## Session service and transport ownership

The combined supervisor borrows one mutable `SessionAuthenticationService` for repeated AJ attempts.

Because only one AJ transaction exists at a time, no concurrent mutable session-service admission transaction is introduced.

The supervisor borrows the existing `AgentRemoteTransportRuntime`; it does not bind, clone/expose, close, wait-idle, or publish readiness for the endpoint.

Outer endpoint lifecycle remains separately gated.

## Bounded public/domain surfaces

Materialization may add only the bounded ownership types required by AK, including equivalents of:

- expected-device admission request;
- fresh admission timing;
- duplicate-active expected-device rejection;
- repeated admission failure event.

No raw Tokio/Quinn task, runtime, connection, panic, join, or pointer identity enters the PRW domain error model.

## Identity and authority invariants

C03e-AL preserves:

- DeviceId / authenticated PRW session identity as logical identity;
- expected DeviceId as pre-auth scheduling intent only;
- post-auth authenticated-owner DeviceId as the active-worker collection key;
- TransportIdentity as current registry-derived lower-transport identity only;
- IP as transient endpoint data;
- SessionId as typed authentication correlation only;
- collection/capacity/duplicate state as lifecycle state, not capability authorization;
- fresh current registry membership, current transport binding, and current policy evaluation on every protected request;
- no authority read guard across transport acceptance, authentication wire I/O, binding, task lifecycle, or collection lifecycle.

PID/UID/GID/thread/task/join/runtime/controller/signal/channel/lock identities remain non-logical implementation details.

## Focused validation required

Source-level non-networking tests must prove the orchestration rules where possible, including:

- full active capacity prevents expected-request polling;
- request-source closure alone does not terminate the supervisor;
- duplicate expected DeviceId is rejected before timing/admission start;
- timing is sampled only for an actually-started attempt;
- shutdown is polled before an in-flight admission future;
- a ready shutdown wins a same-wake tie against a ready admission result;
- shutdown does not drop the in-flight admission future;
- shutdown requests all active worker cancellations before final drain;
- drained admission failure remains unchanged;
- source-level consuming code-4 owner-close seam has the selected signature/diagnostic;
- existing AH tests remain green.

A production real-network listener test is not required here.

## Validation and closure

Because C03e-AL changes Rust Agent source, canonical closure requires on the exact final head:

- PRW Rust Validation FULL PASS: locked dependency graph, rustfmt, Clippy, workspace tests, workspace build;
- PRW Android Validation FULL PASS when the canonical Android workflow triggers for the source change;
- disposable C02f workflows, if present, recorded as SKIPPED and never counted as PASS evidence;
- exact AK merge base and bounded final path scope;
- immutable Drive audit with raw byte/hash verification;
- append-only rolling Drive update preserving the complete post-AK prefix byte-for-byte;
- draft/open/unmerged PR metadata updated to CLOSED only after evidence is final.

No merge, deployment, listener activation, Agent `main.rs`, readiness, process-signal wiring, endpoint close/wait-idle lifecycle, or host mutation is authorized by this gate.
