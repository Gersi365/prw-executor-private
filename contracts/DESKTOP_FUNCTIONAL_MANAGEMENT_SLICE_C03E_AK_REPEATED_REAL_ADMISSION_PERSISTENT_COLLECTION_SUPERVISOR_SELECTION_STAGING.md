# Phase 152 C03e-AK — Repeated Real Admission + Persistent Collection Supervisor Selection Staging

Status: STAGED

Target gate:

`C03E_AK_REPEATED_REAL_ADMISSION_PERSISTENT_COLLECTION_SUPERVISOR_SELECTED`

## Predecessor

Canonical predecessor is closed C03e-AJ:

- branch: `phase-152-c03e-aj-expected-device-real-remote-admission-transaction-source-materialization-staging`
- final head: `ce90d4a3583c4ec3f6bf8604fb7c4eb730369162`
- final tree: `a4e94288925230259ab4429a032ca794599c00f6`
- gate: `C03E_AJ_EXPECTED_DEVICE_REAL_REMOTE_ADMISSION_TRANSACTION_SOURCE_MATERIALIZED`

C03e-AK selects only the first repeated expected-device real admission integration against the already-materialized C03e-AH persistent worker collection under the same private current-thread runtime.

No source is materialized in this checkpoint.

## Existing boundaries retained

C03e-AH already materializes:

- one private long-lived current-thread `Runtime::block_on(...)` collection lifetime;
- `HashMap<DeviceId, ...>` active-worker ownership;
- exactly one active worker per authenticated logical `DeviceId`;
- active-map length as the sole active-worker capacity accounting source;
- bounded caller-supplied `NonZeroUsize` capacity not exceeding `MAX_REGISTERED_DEVICES`;
- ready-worker reap before shutdown/admission work;
- no polling of already-authenticated admission input while full;
- orderly shutdown cancellation of all retained workers before drain;
- no hard abort, detached task, multi-thread runtime, listener activation or readiness.

C03e-AJ already materializes exactly one async expected-device real admission transaction that:

1. resolves current `DeviceId -> TransportIdentity` from fresh registry state;
2. releases the current-authority guard before lower-transport accept;
3. accepts only the exact expected lower-transport identity;
4. performs a second fresh current-registry revalidation after accept;
5. releases the second authority guard before challenge/proof wire I/O;
6. owns code-5 cleanup only for post-accept challenge-preparation failure;
7. delegates pending-session abort + code-1 cleanup exclusively to the existing authentication transaction;
8. delegates code-2 binding-failure cleanup exclusively to existing authenticated-owner composition;
9. returns one `AuthenticatedRemoteSessionRuntimeOwner` on success.

AK must compose those boundaries without weakening either one.

## Selected combined supervisor boundary

A later source-materialization checkpoint may add one Agent-internal domain-specific drive seam conceptually equivalent to:

`RemoteSessionExecutorRuntime::drive_repeated_real_remote_admission_collection(...)`

The raw Tokio runtime remains private.

The complete repeated-admission + persistent-worker lifetime executes inside exactly one private current-thread `Runtime::block_on(async supervisor { ... })`.

The combined seam must not call the existing public `drive_persistent_remote_worker_collection(...)` from inside that runtime, because that would require nested `block_on`.

Later source may refactor/reuse the existing private C03e-AH collection helpers/state machine, but the existing public AH seam must remain behaviorally unchanged.

No generic public `block_on`, runtime handle clone, multi-thread runtime or independent network producer is selected.

## Expected-device request source

The first repeated source is an injected bounded Tokio mpsc receiver of owned **pre-authentication expected-device admission requests**.

A request conceptually owns only the non-transport inputs needed to schedule one AJ attempt and later construct one worker admission, including:

- expected logical `DeviceId`;
- typed `SessionId`;
- authentication PRWM request ID;
- owned capability dispatcher `D` intended for the successful worker;
- owned worker verifier-time provider `T` intended for the successful worker.

The request must not contain:

- `TransportIdentity`;
- IP address as logical identity;
- registry snapshot;
- policy snapshot;
- authenticated-session owner;
- peer/connection handle;
- Tokio task/join/runtime handle;
- cancellation controller/signal;
- active-worker capacity permit.

The source is bounded. No unbounded pending request queue is selected.

Closing the expected-device request source alone does not fabricate supervisor shutdown. It only means no further expected-device requests can arrive; active workers continue until explicit supervisor shutdown.

## Fresh admission timing at actual start

Challenge validity, authentication verifier time and application lease timing must not be frozen merely because an expected-device request sat in the bounded queue while the worker collection was full.

Therefore the combined supervisor selects one injected timing provider/factory sampled **only when a request is actually allowed to start AJ**.

For the selected request, immediately after capacity/duplicate gating and immediately before constructing the AJ future, the timing provider yields the concrete values required by the existing AJ API:

- challenge-validity `Range<u64>`;
- authentication `now_unix_seconds: u64`;
- application-lease `Range<u64>`.

The existing AJ/session/binding validation remains authoritative for the supplied ranges and verifier time.

No timing snapshot is retained as capability authorization evidence.

## Capacity-aware start

C03e-AK preserves C03e-AG/AH active capacity semantics.

The supervisor may poll one expected-device request and begin one AJ transaction only when all of the following are true:

1. orderly shutdown has not been latched;
2. no AJ transaction is currently in flight;
3. `active.len() < max_active_workers`.

The active worker map length remains the sole active-worker capacity accounting source.

AK does not add:

- semaphore permits;
- atomic reservation counters;
- second capacity maps;
- admission leases;
- channel length as worker capacity authority.

At most one pre-authentication AJ transaction may be in flight at any time in this first integration.

The one-in-flight rule is a control-state invariant, not a second worker-capacity counter.

Because no second AJ transaction can start while the first is in flight, a transaction that starts while `active.len() < max_active_workers` cannot lose its available worker slot to another admission. Active workers may only complete and reduce occupancy while that AJ attempt is running.

This intentionally under-utilizes spare capacity when more than one slot is free; parallel pre-authentication admission remains separately gated.

## Duplicate-active preflight

After one expected-device request is received and before any network accept/authentication begins, the supervisor checks the request's expected `DeviceId` against the current active-worker map.

If that expected logical DeviceId already has an active worker:

- reject the expected-device request before constructing AJ;
- perform no lower-transport accept;
- create no pending logical-session challenge;
- spawn no worker;
- do not cancel or replace the existing active worker;
- return/preserve the untouched request through a bounded duplicate-rejection callback/surface.

This pre-auth duplicate check is scheduling/backpressure only. It is not authentication or capability authorization.

On later AJ success, the persistent collection key must still be re-derived from the returned authenticated owner, never trusted from the pre-auth request as authority evidence.

AJ semantics guarantee that the successful authenticated owner corresponds to the expected logical DeviceId used for its challenge transaction.

## Ordinary AJ success handoff

When one in-flight AJ transaction succeeds before shutdown is latched:

1. retain the returned `AuthenticatedRemoteSessionRuntimeOwner`;
2. combine it with the request-owned dispatcher `D` and worker verifier-time provider `T` into the existing `RemoteSessionWorkerAdmission<D, T>` ownership shape or an equivalent internal shape;
3. derive the collection key from `AuthenticatedRemoteSessionRuntimeOwner::logical_device_id()`;
4. create exactly one existing C03e-AD cancellation pair;
5. spawn exactly one existing C03e-S worker body;
6. retain exactly one controller + one JoinHandle in the active map.

No caller-supplied DeviceId is used as the post-auth collection key.

No shared authorization snapshot crosses into the worker. Protected requests continue fresh current registry/transport/policy evaluation through `SharedCurrentCapabilityAuthority`.

## Ordinary AJ failure

One AJ attempt failure does not terminate the repeated supervisor.

The supervisor reports one bounded admission-failure event/callback containing at least:

- the expected logical DeviceId used for the attempt;
- the original `RemoteSessionRealAdmissionError` unchanged.

The failed request is consumed. No automatic retry/reconnect/replacement is selected.

No extra peer/session cleanup is added by the supervisor because AJ and its existing delegated transactions already own the exact cleanup boundaries:

- Registry/Accept failure: no accepted authenticated logical session exists;
- Challenge failure: AJ owns exact code-5 peer close;
- Authentication failure: existing authentication transaction owns pending-session abort + code-1 peer close;
- Binding failure: existing composition owns code-2 peer close.

The supervisor must not double-close or double-abort.

## Deterministic supervisor poll order before shutdown

On each normal supervisor wake, later materialization must preserve this ordering:

1. reap all ready active worker completions first;
2. poll supervisor shutdown;
3. if shutdown did not latch, poll the one existing in-flight AJ future if present;
4. only when no AJ future exists and active capacity exists, poll at most one expected-device request;
5. duplicate-preflight the received request before constructing AJ;
6. if accepted for start, sample fresh admission timing and construct exactly one AJ future.

No second request is polled merely to queue additional pre-auth work while one AJ transaction is in flight.

If supervisor shutdown and an AJ completion become ready on the same wake, shutdown is observed/latching first. This avoids spawning a new worker after shutdown has become ready.

Worker completion reaping remains first so already-completed worker ownership is never hidden by shutdown/admission work.

## Why AJ future drop-cancellation is not selected

The existing logical-session authentication transaction owns pending-session abort only on its terminal failure paths.

Dropping an AJ future after challenge preparation could abandon that explicit cleanup ownership.

Therefore C03e-AK does **not** select:

- dropping the in-flight AJ future on shutdown;
- hard abort of the AJ future;
- task abort;
- cancellation token injected into AJ;
- endpoint close as a substitute for AJ cleanup;
- fabricated authentication failure.

A future checkpoint may select cancellation-aware partial-authentication semantics only after every phase has an explicit cancellation-safe cleanup contract.

## Orderly shutdown while no AJ transaction is in flight

When supervisor shutdown becomes ready and no AJ transaction is in flight:

1. latch shutdown;
2. stop polling the expected-device request source;
3. request cancellation for every retained active worker controller before waiting for any one worker;
4. continue polling/reaping every retained JoinHandle until the active map is empty;
5. return from the combined supervisor.

Queued pre-auth expected-device requests own no accepted peer or pending session and may be dropped with the receiver when the supervisor returns.

The transport runtime itself is not closed by this combined supervisor; its owner remains responsible for endpoint close/wait-idle in a separately selected outer lifecycle boundary.

## Orderly shutdown while AJ transaction is in flight

When supervisor shutdown becomes ready while one AJ transaction is in flight:

1. latch shutdown immediately;
2. stop polling the expected-device request source permanently for this supervisor run;
3. request cancellation for every currently retained active worker controller before waiting on any one worker;
4. **do not drop or abort the AJ future**;
5. keep the same private current-thread runtime actively polling both active-worker completion and the same in-flight AJ transaction until AJ reaches its existing terminal result;
6. never start another AJ transaction after shutdown latches.

The in-flight AJ transaction continues under its existing transport/wire bounded failure semantics. AK introduces no new drain deadline or hard abort.

### In-flight AJ terminal failure after shutdown

If the drained AJ transaction returns `Err(RemoteSessionRealAdmissionError)` after shutdown latched:

- report the same bounded admission failure;
- perform no additional peer/session cleanup;
- continue draining active workers until the map is empty.

Existing AJ/authentication/binding cleanup remains sole owner.

### In-flight AJ success after shutdown

If the drained AJ transaction returns a new `AuthenticatedRemoteSessionRuntimeOwner` after shutdown latched:

- do **not** insert it into the persistent collection;
- do **not** spawn a worker merely for cleanup;
- do **not** poll capability request work;
- consume that owner through one later narrow Agent-internal orderly-shutdown close seam using the existing code-4 remote capability-session shutdown diagnostic exactly once;
- then drop the consumed owner and continue draining previously active workers.

Later source materialization may add a consuming internal method conceptually equivalent to:

`AuthenticatedRemoteSessionRuntimeOwner::close_for_orderly_shutdown(self)`

That method is selected only for an authenticated owner that completed AJ after supervisor shutdown had already latched.

It must reuse the existing fixed code-4 diagnostic:

- code `4`;
- reason `remote capability session shutdown`.

It performs no capability authorization, no request-loop poll, no task spawn, no authenticated-session deletion, no retry and no replacement.

This consuming close seam exists solely to avoid both a silent peer drop and a post-shutdown worker spawn.

## Active-worker shutdown remains C03e-AH/C03e-S

Existing active workers remain governed by the existing C03e-AH/C03e-S semantics:

- every retained controller is requested before drain;
- worker Q is polled before cancellation on each worker wake;
- Q failure still owns code-3 terminal close/classification;
- cancellation while Q remains pending still owns code-4 shutdown close/classification;
- no hard abort or forced deadline is introduced;
- completion callbacks retain authenticated DeviceId plus existing bounded worker/join result.

The new post-AJ-success-after-shutdown close seam does not change worker race ordering because no worker exists for that owner.

## SessionAuthenticationService ownership

The combined supervisor borrows one mutable `SessionAuthenticationService` for the repeated admission lifetime.

Only one AJ transaction is in flight, so no concurrent mutable session-service admission transaction is selected.

Worker capability processing does not borrow this service and therefore may continue concurrently on the same current-thread runtime while one AJ transaction is in flight.

No session-service clone or secondary pending-session store is introduced.

## Transport runtime ownership

The combined supervisor borrows the existing `AgentRemoteTransportRuntime` for repeated AJ attempts.

It does not:

- bind a new endpoint;
- clone/expose the underlying endpoint;
- close the endpoint automatically on one admission failure;
- close the endpoint automatically on collection duplicate rejection;
- treat endpoint ownership as logical identity;
- publish readiness.

Outer endpoint shutdown/idle waiting remains separately gated.

## Failure and callback boundary

The combined supervisor itself should retain only bounded configuration failure comparable to existing `RemoteSessionPersistentCollectionConfigError` for invalid active-worker capacity.

Individual repeated admission failures are reported through a callback/event surface and do not tear down the supervisor.

Pre-auth duplicate request rejection is reported separately from AJ failure because no AJ transaction was attempted.

Worker completions continue through the existing bounded completion surface.

No raw Tokio/Quinn task, connection, runtime, panic or join identifiers enter the domain error model.

## Identity and authority invariants

C03e-AK preserves:

- `DeviceId` / authenticated PRW session identity as logical identity;
- `TransportIdentity` as lower-transport identity derived from fresh current registry state;
- IP as transient endpoint data;
- `SessionId` as transaction correlation, not worker/collection identity;
- expected DeviceId as pre-auth scheduling intent, not post-auth authority evidence;
- authenticated owner-derived DeviceId as collection key;
- collection membership/capacity/duplicate status as lifecycle state only, not authorization;
- fresh current registry / current transport binding / current policy evaluation for every protected capability request;
- no authority guard across accept, authentication wire I/O, binding, worker task lifecycle or collection lifecycle.

PID/UID/GID/thread/task/join/runtime/controller/signal/channel/lock identities remain non-logical implementation details.

## No nested or parallel executor expansion

AK explicitly does not select:

- nested `Runtime::block_on`;
- runtime Handle clone/exposure;
- `rt-multi-thread`;
- second Agent Tokio runtime;
- detached admission producer task;
- multiple simultaneous AJ transactions;
- per-device admission tasks;
- JoinSet ownership;
- unbounded admission queue;
- semaphore-based capacity authority.

The same existing private current-thread runtime remains actively driven for network admission and worker tasks.

## Focused materialization tests required later

The later source-materialization checkpoint must use injected/non-networking focused tests where possible to prove at least:

- expected request source is not polled while active map is full;
- at most one pre-auth admission future exists;
- duplicate expected DeviceId is rejected before AJ/network start;
- fresh timing provider is sampled only when an attempt actually starts;
- worker completions are reaped before shutdown/admission work;
- shutdown wins same-wake against a ready admission completion for spawn/no-spawn decision;
- shutdown stops new expected-request polling;
- active worker controllers are all requested before drain;
- an in-flight admission future is not dropped when shutdown latches;
- in-flight terminal failure after shutdown preserves the original admission error and adds no cleanup;
- in-flight success after shutdown uses the consuming code-4 owner-close seam and never spawns/inserts a worker;
- ordinary success before shutdown becomes exactly one existing worker admission;
- ordinary admission failure does not terminate the supervisor;
- request-source closure alone does not fabricate supervisor shutdown.

Do not require a production real-network listener integration test in the first materialization checkpoint.

## Exact expected first materialization scope

The next source-materialization checkpoint should remain bounded to the minimum Agent source needed for this selected supervisor, likely including only:

1. its new materialization contract;
2. `crates/prw-agent/src/remote_session_capability_runtime.rs` for bounded re-exports if needed;
3. `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs` only for the selected consuming orderly-shutdown owner close seam;
4. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs` for the combined supervisor and request/callback ownership types.

`real_remote_admission_transaction.rs` should remain semantically unchanged unless compilation proves a concrete integration-boundary contradiction.

No manifest, lockfile, permanent workflow, Android application, Agent `main.rs`, packaging or host path is expected.

## Explicit non-selection

C03e-AK does not select/materialize:

- source code in AK itself;
- blind accept-any transport;
- parallel pre-authentication attempts;
- cancellation/drop/abort of partial AJ authentication;
- automatic retry/reconnect/replacement;
- replacement of an active same-DeviceId worker;
- multiple active workers per DeviceId;
- hard shutdown deadline;
- task abort;
- transport endpoint close/wait-idle lifecycle;
- process-signal wiring;
- Agent `main.rs`;
- readiness;
- systemd/host mutation;
- deployment;
- merge.

## Validation requirements

AK is docs-only selection.

Closure requires on the exact final AK head:

- exact AJ merge base;
- exactly one net changed path: this contract;
- permanent PRW Rust validation FULL PASS;
- Android validation only if canonical workflow triggers; no Android PASS may be claimed when it does not run;
- disposable workflows recorded only as skipped;
- immutable Drive audit with raw-readback byte verification;
- append-only rolling Drive update preserving the complete post-AJ prefix byte-for-byte;
- PR remains draft/open/unmerged.

## Completion meaning

Closure means only that repeated expected-device real admission and the existing persistent worker collection now have one selected deterministic same-current-thread integration contract, including capacity-aware start and shutdown ownership for every AJ phase.

No production listener loop is active until a later source-materialization checkpoint implements and validates this contract.

Target gate:

`C03E_AK_REPEATED_REAL_ADMISSION_PERSISTENT_COLLECTION_SUPERVISOR_SELECTED`
