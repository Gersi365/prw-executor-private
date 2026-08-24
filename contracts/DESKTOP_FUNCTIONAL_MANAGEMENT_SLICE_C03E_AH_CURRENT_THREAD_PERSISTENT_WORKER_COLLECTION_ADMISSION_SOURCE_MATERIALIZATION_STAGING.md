# Phase 152 C03e-AH — Current-Thread Persistent Worker Collection / Admission Source Materialization Staging

Status: STAGED

Target gate:

`C03E_AH_CURRENT_THREAD_PERSISTENT_WORKER_COLLECTION_ADMISSION_SOURCE_MATERIALIZED`

## Predecessor

Canonical predecessor is closed C03e-AG:

- branch: `phase-152-c03e-ag-current-thread-persistent-worker-collection-admission-selection-staging`
- head: `0f5dc2121c2eda87479cabb9c62755fb47b7ca44`
- tree: `84da132c5b458c906e2614bf93de11eaae877e4e`
- gate: `C03E_AG_CURRENT_THREAD_PERSISTENT_WORKER_COLLECTION_ADMISSION_SELECTED`

C03e-AH preserves exact AG lineage and materializes only the AG-selected pre-listener persistent collection/supervisor seam.

## Purpose

Materialize the first bounded persistent remote capability-worker collection that can coexist cooperatively on the existing private Tokio current-thread runtime while one long-lived executor drive remains active.

This checkpoint proves collection ownership, admission backpressure, duplicate DeviceId rejection, completion reaping and orderly cancel-all-before-join-all shutdown with an injected already-authenticated admission source.

It must not bind a listener, authenticate a peer, accept transport connections, wire process signals, publish readiness, modify `main.rs`, deploy or merge.

## Exact injected admission source

C03e-AH selects a bounded Tokio `mpsc::Receiver` supplied by the caller as the first injected admission source.

The receiver item owns one already-composed worker admission consisting of:

- one `AuthenticatedRemoteSessionRuntimeOwner`;
- dispatcher `D` by value;
- verifier-time provider `T` by value.

The admission item carries no caller-supplied DeviceId. Its logical key is always derived from the retained authenticated session owner.

The receiver is not a network listener and does not perform authentication. A real accept/authentication producer remains separately gated.

The collection never polls this receiver while at active-worker capacity.

## Materialized admission item

C03e-AH adds one bounded source type equivalent in responsibility to:

`RemoteSessionWorkerAdmission<D, T>`

Construction consumes exactly the authenticated owner, dispatcher and verifier-time provider. It accepts no transport identity, DeviceId, registry snapshot, policy snapshot or authorization evidence.

An internal DeviceId lookup delegates to the authenticated owner and is used only before worker spawn.

## Materialized logical DeviceId accessor

`AuthenticatedRemoteSessionRuntimeOwner` gains one narrow Agent-internal immutable accessor equivalent in responsibility to:

`logical_device_id(&self) -> &DeviceId`

The accessor derives:

`capability_owner.bound_session.session().device_id()`

It performs no I/O, registry lookup, transport selection or authorization.

It must not expose or replace TransportIdentity and must not accept a DeviceId from the caller.

## Persistent collection configuration

The materialized executor seam accepts one `NonZeroUsize` active-worker capacity.

Capacity validation occurs before entering the long-lived runtime drive.

If the configured value exceeds `prw_registry::MAX_REGISTERED_DEVICES`, the method returns one bounded Agent-domain configuration error and spawns no task.

The value is never silently clamped.

The active map length is the only active-capacity accounting source. No second atomic, semaphore or permit collection is added.

## Materialized retained entry

The private collection is a `HashMap<DeviceId, ...>`.

Each retained entry owns exactly:

- one non-cloneable C03e-AD `RemoteSessionWorkerCancellationController`;
- one `tokio::task::JoinHandle<AuthenticatedRemoteSessionWorkerStop>`.

No raw peer, dispatcher, verifier provider, registry snapshot, policy snapshot, authorization evidence, runtime handle or TransportIdentity is retained in the map entry.

## Materialized worker spawn

Only after capacity and duplicate checks succeed:

1. create exactly one C03e-AD controller/signal pair;
2. clone exactly one outer `SharedCurrentCapabilityAuthority` Arc for that worker;
3. move the admission's authenticated owner, dispatcher, verifier-time provider and cancellation signal into one Tokio task;
4. delegate directly to existing `run_capability_request_worker(...)`;
5. insert only the controller + JoinHandle under the already-derived DeviceId key.

No request loop, authorization, dispatch or transport-close behavior is reimplemented.

## Duplicate admission rejection

Duplicate DeviceId membership is checked before worker spawn through the map entry API.

If the DeviceId is already occupied:

- existing entry remains untouched;
- no cancellation request is issued;
- no replacement task is spawned;
- no second authority clone is created for a worker;
- the rejected admission remains fully owned.

C03e-AH adds one bounded rejection reason with the first variant:

`DuplicateActiveDevice`

and one rejection wrapper that preserves the untouched admission item for explicit caller cleanup.

No peer-close code or listener diagnostic is selected here.

## Completion record

Every reaped worker produces one Agent-domain completion record containing:

- the retained DeviceId key;
- `Result<AuthenticatedRemoteSessionWorkerStop, RemoteSessionSpawnedWorkerJoinError>`.

Tokio abnormal join still maps only to existing `AbnormalTaskCompletion`.

Raw `JoinError`, panic payload, task ID, runtime ID and backtrace do not escape.

Completion records are delivered immediately through a caller-supplied synchronous callback rather than accumulated in an unbounded vector.

## Rejection callback

Duplicate rejection is delivered immediately through a caller-supplied synchronous callback that consumes the bounded rejection wrapper.

The supervisor does not accumulate rejected authenticated owners internally.

The callback boundary is lifecycle reporting only; it does not authorize requests or close the rejected peer automatically.

## Long-lived current-thread runtime drive

The new executor method remains an inherent method on the existing non-cloneable `RemoteSessionExecutorRuntime` and performs one private `Runtime::block_on(...)` around the complete persistent supervisor lifetime.

The active worker map, injected admission receiver, shutdown future and callbacks live inside this one drive.

Worker tasks make progress cooperatively while this current-thread runtime is driven.

No task collection survives return from the enclosing method.

No runtime handle or raw `block_on` surface is exposed.

## Supervisor wake ordering

Before shutdown wins, one supervisor poll obeys this exact semantic priority:

1. poll every retained JoinHandle once and reap every result already ready;
2. poll the supervisor-shutdown future;
3. only if shutdown remains pending and `active.len() < max_active`, poll the injected admission receiver;
4. receive at most one admission item for that poll;
5. derive DeviceId from that item;
6. apply duplicate membership through `HashMap::entry` before spawn;
7. spawn/register at most one worker for that admission event.

Ready completion callbacks execute before shutdown/admission handling on that poll.

No HashMap iteration order becomes protocol semantics.

## Capacity backpressure

When `active.len() == max_active`, the admission receiver is not polled.

Existing worker handles and supervisor shutdown remain polled.

This ensures the collection does not consume already-authenticated candidates it cannot retain and does not create an internal pending queue.

The caller-owned bounded mpsc may itself retain messages according to its own explicit channel capacity; C03e-AH does not add an unbounded queue.

## Admission-source closure

Injected admission-source closure is not supervisor shutdown.

If the mpsc receiver closes:

- no new admissions are possible;
- existing workers continue to run and be reaped;
- the supervisor continues to wait for the separately supplied orderly shutdown future;
- no active worker is cancelled merely because the source closed.

This is a pre-listener harness behavior only and does not define future network listener lifetime semantics.

## Orderly shutdown

When the supervisor-shutdown future becomes ready:

1. the admission receiver is never polled again;
2. every completion already ready on that same poll has already been reaped;
3. call `request_cancellation()` exactly once for every still-retained entry before awaiting any one worker;
4. after all requests are issued, poll/reap the same retained JoinHandles until the map is empty;
5. deliver one completion callback per removed entry;
6. return `Ok(())` only after the active map is empty.

The supervisor does not fabricate `AuthenticatedRemoteSessionWorkerStop::Cancelled`.

Existing C03e-S result authority remains unchanged.

## No hard abort

C03e-AH does not call or expose:

- `JoinHandle::abort()`;
- `AbortHandle`;
- `JoinSet::shutdown()`;
- runtime teardown as worker cancellation;
- forced drain timeout.

No task is intentionally detached or forgotten.

## Shared-current authority remains per request

One outer authority Arc clone is created per admitted worker only after duplicate/capacity checks.

Within each worker, every protected request still obtains fresh current registry/policy authority through the existing C03e-Z path.

Admission success, DeviceId keying, capacity availability and collection membership are not request authorization evidence.

No registry or policy snapshot is introduced.

## Focused injected-admission tests

The private persistent-supervisor primitive must include focused current-thread tests proving at least:

1. duplicate DeviceId is rejected before a second spawn while the first worker remains active;
2. a ready completion is reaped before a same-DeviceId candidate is evaluated, so the completed key may be admitted again;
3. ready shutdown wins before a prequeued admission is consumed/spawned;
4. shutdown requests cancellation for all retained workers and drains/classifies all handles;
5. invalid capacity above `MAX_REGISTERED_DEVICES` fails before runtime work;
6. admission-source closure does not fabricate worker shutdown.

Tests may use generic private candidate keys/payloads and C03e-AD signals so they do not require real network peers.

No Tokio macros feature is added.

## Dependency boundary

C03e-AH uses only existing dependencies/features:

- `std::collections::HashMap`;
- `std::num::NonZeroUsize`;
- `prw_core::DeviceId`;
- `prw_registry::MAX_REGISTERED_DEVICES`;
- existing Tokio `rt` + `sync` features.

These files remain byte-stable:

- `Cargo.lock`;
- `crates/prw-agent/Cargo.toml`;
- `crates/prw-remote-bridge/Cargo.toml`.

Tokio `macros` and `rt-multi-thread` remain absent.

## Identity invariants

- DeviceId / authenticated PRW session identity remain logical identity;
- DeviceId is the collection key only because it is derived from the authenticated owner;
- TransportIdentity remains lower-transport identity only;
- IP remains transient endpoint;
- SessionId alone is not collection identity;
- task/join/runtime/thread/controller/signal/channel/Arc/PID/UID/GID/lock identities are not PRW logical identity.

The collection key is lifecycle/scheduling metadata and never request authorization evidence.

## Existing seams remain stable

C03e-AH must not behavior-change:

- `drive_capability_request_worker(...)`;
- `drive_spawned_capability_request_worker(...)`;
- `drive_supervised_capability_request_worker(...)`;
- C03e-AD cancellation pair semantics;
- C03e-S/Q/Z worker/request semantics.

The persistent seam is additive.

## Exact source scope

The intended final AG -> AH net diff contains exactly four paths:

1. this contract;
2. `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`;
3. `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`;
4. `crates/prw-agent/src/remote_session_capability_runtime.rs`.

No manifest, lockfile, permanent workflow, bridge, transport, Android application, Agent binary, packaging or host path belongs to final AH scope.

## Validation requirements

Closure requires on the final exact AH head:

- exact AG merge base;
- exact four-path final net scope;
- no manifest/lockfile change;
- permanent PRW Rust validation FULL PASS: locked dependency graph, rustfmt, Clippy, workspace tests and workspace build;
- canonical Android native/application validation FULL PASS because Rust source changes are present;
- skipped workflows recorded only as skipped;
- immutable Drive audit with raw-readback byte verification;
- append-only rolling Drive update preserving the complete post-AG prefix byte-for-byte;
- PR remains draft/open/unmerged.

## Explicit non-selection

C03e-AH does not materialize or select:

- real QUIC listener accept;
- transport authentication;
- session proof exchange;
- network retry/reconnect;
- network duplicate-rejection close code;
- process-signal wiring;
- Agent `main.rs` wiring;
- readiness publication;
- systemd/host mutation;
- deployment;
- merge;
- hard abort;
- forced-drain timeout;
- unbounded admission queue;
- multiple active workers for one DeviceId;
- multi-thread Tokio runtime.

## Completion meaning

Closure means only that the AG-selected persistent current-thread worker collection/admission architecture exists in Agent Rust source behind an injected already-authenticated bounded mpsc source and is canonically validated.

It does not mean remote peers are accepted by production code, process shutdown is wired, readiness may be published, or deployment/merge may occur.

Target gate:

`C03E_AH_CURRENT_THREAD_PERSISTENT_WORKER_COLLECTION_ADMISSION_SOURCE_MATERIALIZED`
