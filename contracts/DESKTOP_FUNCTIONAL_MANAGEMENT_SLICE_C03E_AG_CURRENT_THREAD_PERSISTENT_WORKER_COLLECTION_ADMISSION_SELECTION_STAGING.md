# Phase 152 C03e-AG — Current-Thread Persistent Worker Collection / Admission Selection Staging

Status: STAGED

Target gate:

`C03E_AG_CURRENT_THREAD_PERSISTENT_WORKER_COLLECTION_ADMISSION_SELECTED`

## Predecessor

Canonical predecessor is closed C03e-AF:

- branch: `phase-152-c03e-af-current-thread-single-worker-supervisor-source-materialization-staging`
- head: `a53417a5d77085006acd02c15d0c611177a4c7e3`
- tree: `459e9c2ce79059b7b0b34df2afe2c787881f7cc9`
- gate: `C03E_AF_CURRENT_THREAD_SINGLE_WORKER_SUPERVISOR_SOURCE_MATERIALIZED`

C03e-AG preserves exact AF lineage. It is a selection-only checkpoint.

## Purpose

Select the first persistent remote-worker collection and admission boundary after C03e-AF proved one lexically supervised current-thread worker.

The selected boundary must answer, before any multi-worker source is introduced:

1. how a collection can remain live while Tokio current-thread tasks continue to make progress;
2. which logical identity keys an active worker;
3. whether two active workers for the same logical device are allowed;
4. how active-worker capacity is bounded and how capacity backpressure works;
5. how completed workers are reaped and classified without silently dropping results;
6. how orderly shutdown reaches every retained worker without controller cloning or hard abort;
7. what an admission rejection owns and what it must not destroy implicitly;
8. what remains separately gated before a real listener/accept loop or Agent bootstrap can run.

This checkpoint does not materialize the collection, spawn a second worker, accept a peer, bind a listener, add a dependency, wire `main.rs`, publish readiness, deploy or merge.

## Existing facts constraining the selection

### C03e-R lifecycle contract

C03e-R already selected the semantic relationship for a future retained worker entry:

- one task/join handle;
- one matching cancellation-controller authority;
- explicit completion reaping;
- stop admission before shutdown;
- request cancellation for all retained workers before joining them;
- classify every completion;
- do not intentionally drop a live managed handle.

C03e-R deliberately deferred the collection key, capacity, duplicate-session policy, fairness and concrete async runtime ownership.

### C03e-AF current-thread constraint

C03e-AF proved that a Tokio current-thread task does not become an independently progressing background worker merely because a `JoinHandle` exists.

The private Agent runtime makes task progress only while that current-thread runtime is actively driven.

Therefore a persistent worker collection must live inside one long-lived future that is itself driven by the existing private `Runtime::block_on(...)`. The collection must not be stored across synchronous executor calls while expecting its tasks to continue progressing after `block_on` returns.

### Logical DeviceId is already authoritative

`AuthenticatedDeviceSession` carries `DeviceId`, and `BoundRemoteSession::session()` preserves that authenticated logical session.

`WorkspaceDeviceRegistry` stores registered devices in `HashMap<DeviceId, RegisteredDevice>`, so the current registry already treats `DeviceId` as the device key.

TransportIdentity remains a separately rotatable transport binding and is revalidated per protected request. It is not a worker-collection key.

### DeviceId is hashable, not ordered

The current `DeviceId` domain type implements `Clone`, `Eq` and `Hash`, but not `Ord`.

The selected first keyed collection is therefore a `HashMap<DeviceId, ...>` or an equivalent hash-keyed Agent-private map. C03e-AG does not add an ordering requirement to DeviceId merely for collection convenience.

### Local Linux precedent

The existing local Linux worker registry retains each handle beside its matching cancellation authority, reaps finished workers explicitly and performs cancel-all before join-all during shutdown.

Its capacity owner is caller-bounded and refuses acquisition at capacity rather than creating an unbounded queue.

C03e-AG adopts those lifecycle properties only; it does not reuse OS-thread/scoped-handle types for Tokio remote workers.

## Selected long-lived current-thread supervisor

The first persistent remote-worker collection exists only inside one long-lived Agent-owned supervisor future driven through the existing private current-thread runtime.

A later source seam may be equivalent in responsibility to:

`drive_persistent_remote_worker_collection(...)`

The outer method remains on the non-cloneable `RemoteSessionExecutorRuntime` and keeps one private `Runtime::block_on(...)` active until the collection supervisor reaches orderly terminal shutdown.

Within that one active runtime drive, multiple Tokio worker tasks may coexist and make progress cooperatively on the same current thread.

The collection is persistent across multiple admissions inside that supervisor lifetime, but is not persisted across return from the enclosing `block_on` call.

No runtime handle is exposed or stored in domain state.

## Selected collection key

The active-worker collection is keyed only by authenticated logical `DeviceId`.

The key must be derived from the already-composed `AuthenticatedRemoteSessionRuntimeOwner` / retained bound authenticated session.

A later source materialization may add one narrow Agent-internal accessor equivalent in responsibility to:

`logical_device_id(&self) -> &DeviceId`

That accessor must derive the value from the retained authenticated session and must not accept a caller-supplied DeviceId.

No admission API may receive a second independent DeviceId that could disagree with the authenticated owner.

The collection must not use:

- TransportIdentity;
- IP address;
- SessionId alone;
- Tokio task ID;
- runtime/thread ID;
- PID/UID/GID;
- pointer/Arc identity;
- local integer worker IDs

as logical collection authority.

## One active worker per DeviceId

C03e-AG selects exactly one active remote capability worker per logical DeviceId in the first persistent collection.

If an already-composed candidate refers to a DeviceId that still has a retained active entry:

- the existing worker remains authoritative and is not cancelled or replaced;
- no second worker is spawned;
- no replacement race is started;
- the candidate is rejected as duplicate-active-device admission.

A new worker for that DeviceId may be admitted only after the old worker is terminal, its completion is explicitly reaped/classified, and its entry is removed.

C03e-AG does not select “latest connection wins”, automatic replacement, migration or parallel sessions for one DeviceId.

## Rejection ownership is fail-closed

Duplicate admission must be detected before ownership-consuming worker spawn.

A later source API must preserve ownership of the rejected `AuthenticatedRemoteSessionRuntimeOwner` rather than silently dropping an already-authenticated live owner merely because admission failed.

The exact internal wrapper type may be selected during source materialization, but it must retain:

- a bounded rejection class; and
- the untouched rejected authenticated runtime owner for explicit outer cleanup.

C03e-AG does not select a new peer-close code or network rejection diagnostic. Listener/protocol-facing rejection cleanup remains separately gated.

## Selected active-worker capacity

The persistent collection is caller-bounded at construction by one `NonZeroUsize` maximum active-worker count.

The configured bound must not exceed the existing absolute registered-device bound `MAX_REGISTERED_DEVICES` because every admitted worker is keyed by an authenticated registered DeviceId.

A later constructor must reject an invalid larger configured bound rather than silently clamp it.

The collection's retained-entry count is the active-capacity accounting source for this current-thread owner. C03e-AG does not select a second AtomicUsize/permit system for the same entries.

An entry continues to consume one active slot until its worker is terminal and the completion is reaped/removed.

## Capacity backpressure

When the retained-entry count equals the configured maximum:

- the long-lived supervisor must continue polling/reaping existing worker completions and shutdown;
- it must not poll the next already-authenticated admission source merely to receive work that cannot be retained;
- it must not create an unbounded pending candidate queue;
- it must not spawn over capacity.

This is backpressure at the collection-admission seam.

A later real network accept/authentication scheduler may apply an earlier pre-authentication backpressure boundary, but that listener behavior is not selected by C03e-AG.

## Reap-before-admit rule

Before accepting a new candidate on a supervisor wake, the supervisor must first poll retained worker handles for terminal completion and reap every completion already ready on that wake.

This rule ensures:

- completed entries release capacity promptly;
- a completed DeviceId entry is removed before duplicate admission is evaluated;
- no ready worker result is silently deferred behind a stream of new admissions.

Reaping maps each completed handle to the existing bounded result:

`Result<AuthenticatedRemoteSessionWorkerStop, RemoteSessionSpawnedWorkerJoinError>`

and associates that completion with the DeviceId key that owned the entry.

No raw Tokio `JoinError` or task identifier becomes completion identity.

## Completion accounting

Every removed worker entry must produce one explicit Agent-domain completion record containing:

- the retained logical DeviceId key; and
- the existing bounded worker/join result.

The exact later type may be equivalent in responsibility to:

`RemoteSessionRegisteredWorkerCompletion`

No completion may be discarded merely because the worker ended between admission polls.

Completion order among different DeviceIds is not protocol semantics and C03e-AG does not add ordering requirements to DeviceId.

A later source implementation may return a batch of ready completions from one poll/reap pass; tests must compare by DeviceId rather than relying on HashMap iteration order.

## Selected supervisor wake priority

Before orderly shutdown has begun, one supervisor wake uses this semantic priority:

1. poll/reap retained worker completions;
2. poll the supervisor-shutdown future;
3. only if shutdown remains pending and collection capacity exists, poll at most one new already-composed admission candidate;
4. if a candidate is received, derive its DeviceId from the owner and apply duplicate admission checks before spawn;
5. register at most one new worker for that admission event.

This ordering prevents a same-wake shutdown request from admitting new work and prevents ready worker completions from being starved by admissions.

C03e-AG does not select a throughput/fairness policy beyond this bounded single-admission-per-poll-cycle rule.

## Worker entry ownership

Each retained map entry owns exactly:

- one matching non-cloneable C03e-AD `RemoteSessionWorkerCancellationController`;
- one Tokio `JoinHandle<AuthenticatedRemoteSessionWorkerStop>`.

The task itself owns:

- its `AuthenticatedRemoteSessionRuntimeOwner`;
- one shared-current authority outer-Arc clone;
- its dispatcher;
- its verifier-time provider;
- its one C03e-AD cancellation signal future.

The map entry does not retain:

- a raw peer;
- a second authenticated-session owner;
- TransportIdentity as a key;
- registry/policy snapshots;
- capability authorization evidence;
- dispatcher mutable state outside the task;
- raw runtime handles.

## Shared-current authority remains per request

Multi-worker collection ownership does not weaken C03e-X/Z authority semantics.

Every protected request in every worker still obtains current registry/policy authority independently.

No worker admission result, collection key, DeviceId duplicate check or active-capacity check is authorization evidence for a later request.

Registry suspension/removal, device revocation and transport-identity rotation remain visible through the existing per-request current-authority path.

## Orderly multi-worker shutdown

C03e-AG now selects the first collection-level orderly shutdown sequence required by the persistent owner:

1. once supervisor shutdown wins, stop polling the admission source permanently;
2. retain every current entry and handle;
3. call `request_cancellation()` exactly once for every currently retained entry before awaiting any one worker to completion;
4. after all cancellation requests are issued, continue driving the same current-thread runtime;
5. await/reap every remaining handle;
6. classify every completion with its DeviceId;
7. return only after the active map is empty.

The cancellation controllers remain non-cloneable. Fan-out is an iteration over individually owned controllers, not a broadcast token or cloned authority.

If a worker had already completed but had not yet been reaped when shutdown begins, its handle is reaped/classified as completion; shutdown must not fabricate a Cancelled stop for it.

## No hard abort or shutdown deadline

C03e-AG does not select:

- `JoinHandle::abort()`;
- `AbortHandle`;
- `JoinSet::shutdown()`;
- runtime teardown as worker cancellation;
- a forced-drain timeout;
- detached tasks.

Orderly shutdown may remain pending until every managed worker reaches its existing C03e-S terminal path.

Any future forced deadline requires a separate contract proving cancellation safety of in-flight dispatcher/domain operations.

## No JoinSet in the first keyed collection

The first persistent collection is selected as a DeviceId-keyed map rather than `JoinSet` because one-active-worker-per-DeviceId admission requires direct logical-key membership testing and paired cancellation custody.

A JoinSet may not be added merely as a second hidden handle owner beside the map.

There must be one authoritative retained handle per active DeviceId.

## Admission input remains pre-listener

C03e-AG selects collection admission for already-composed authenticated runtime owners only.

It does not select or materialize:

- QUIC listener accept;
- transport authentication;
- session proof exchange;
- application lease selection;
- peer bind loop;
- retry/reconnect;
- process signal handling.

A later source checkpoint may use a caller-supplied/injected async admission source solely to prove collection behavior without activating real network admission.

## Dependency boundary

C03e-AG selects no new crate, package version or Cargo feature.

The first source materialization must use existing standard collections and existing Tokio `rt` support.

Tokio `rt-multi-thread` and `macros` remain unselected.

These files remain byte-stable in AG:

- `Cargo.lock`;
- `crates/prw-agent/Cargo.toml`;
- `crates/prw-remote-bridge/Cargo.toml`.

## Identity invariants

Collection ownership creates no new PRW identity.

- DeviceId / authenticated PRW session identity remain logical identity;
- DeviceId is the active-worker collection key;
- TransportIdentity remains lower-transport identity only;
- IP remains transient endpoint;
- SessionId is not substituted for DeviceId collection authority;
- task/join/runtime/thread/controller/signal/Arc/PID/UID/GID/lock identities are not logical identity.

A DeviceId map key is scheduling/lifecycle ownership metadata only. It does not itself authorize a capability request.

## Explicit non-selection

C03e-AG does not select or perform:

- source materialization of the collection;
- real listener/accept/authentication loops;
- a network rejection close code;
- replacement of an active same-DeviceId worker;
- multiple active workers for one DeviceId;
- unbounded worker count or admission queue;
- separate atomic capacity accounting;
- controller/signal Clone;
- broadcast/watch cancellation tokens;
- hard abort or forced drain deadline;
- `JoinSet` as a second handle owner;
- multi-thread Tokio runtime;
- process-signal wiring;
- Agent `main.rs` wiring;
- readiness publication;
- systemd/host mutation;
- deployment;
- merge.

## Exact selection scope

C03e-AG is docs-only.

Its final AF -> AG net diff must contain exactly this contract path and no source, manifest, lockfile, workflow, Android application, bridge, transport, Agent binary, packaging or host mutation.

## Validation requirements

Closure requires on the final exact AG head:

- exact AF merge base;
- exact one-path docs-only net scope;
- permanent PRW Rust validation FULL PASS: locked dependency graph, rustfmt, Clippy, workspace tests and workspace build;
- Android validation need not trigger for the docs-only exact head; no Android PASS may be claimed if it does not run;
- skipped workflows recorded only as skipped;
- immutable Drive audit with raw-readback byte verification;
- append-only rolling Drive update preserving the complete post-AF prefix byte-for-byte;
- PR remains draft/open/unmerged.

## Completion meaning

Closure of C03e-AG means only that the first persistent current-thread remote-worker collection/admission architecture is selected:

- one long-lived current-thread supervisor owns the collection;
- active entries are keyed by authenticated DeviceId;
- one active worker per DeviceId;
- caller-bounded capacity with no over-capacity admission polling;
- explicit completion reaping;
- cancel-all-before-join-all orderly shutdown;
- no hard abort and no network activation.

It does not mean the collection exists in Rust source, a listener accepts peers, multiple sessions are running in production, process shutdown is wired, readiness is published or deployment may occur.

The next checkpoint may materialize only this pre-listener persistent collection/supervisor seam and focused injected-admission tests. Real listener admission and Agent bootstrap remain separately gated.

Target gate:

`C03E_AG_CURRENT_THREAD_PERSISTENT_WORKER_COLLECTION_ADMISSION_SELECTED`
