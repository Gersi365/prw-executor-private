# Phase 152 C03e-W — Shared Current Capability Authority Selection Staging

Status: STAGED

Target gate:

`C03E_W_SHARED_CURRENT_CAPABILITY_AUTHORITY_SELECTED`

## Predecessor

Canonical predecessor is closed C03e-V:

- branch: `phase-152-c03e-v-borrowed-single-worker-drive-seam-source-materialization-staging`
- head: `a6bb340964e2d8e45625972523ea378b64beaa14`
- tree: `136b3d45efa25beac5e0563d89cf773614212bb9`
- gate: `C03E_V_BORROWED_SINGLE_WORKER_DRIVE_SEAM_SOURCE_MATERIALIZED`

C03e-W preserves exact V lineage.

## Purpose

Select the shared-current authorization ownership boundary required before any authenticated remote-session worker may become a spawned `'static` task.

This checkpoint selects architecture only. It does not materialize synchronization source, change the existing C03e-O/Q/S/V worker signatures, spawn tasks, wire Agent `main.rs`, publish readiness, or activate remote transport.

## Problem that must be solved

The existing remote-session worker path deliberately borrows one `CapabilityBridge<'_, P>` for current per-request authorization.

The production bridge in turn borrows:

- `&WorkspaceDeviceRegistry`; and
- `&P` where `P: PolicyEvaluator`.

The registry is mutable process authority. Membership suspension/removal, device revocation, and transport-identity rotation must be visible to subsequent protected-operation authorization. A per-task cloned registry snapshot is therefore invalid.

The current borrowed bridge is correct for the C03e-V single borrowed worker but cannot simply be leaked, cloned, converted to a process-global static, or captured as a stale snapshot to satisfy a future spawned task lifetime.

## Existing bridge split reused by this selection

The current bridge already exposes the exact split needed for safe synchronization:

- `CapabilityBridge::authorize(...)` performs lease validation, current registry session validation, current transport-binding validation, request decoding and policy evaluation, returning an owned `AuthorizedCapabilityRequest`;
- `CapabilityBridge::process_request(...)` calls `authorize(...)`, then invokes the dispatcher and constructs the bounded response frame.

`BoundRemoteSession::authorize(...)` also already delegates to `CapabilityBridge::authorize(...)` while supplying its stored transport identity and lease internally.

C03e-W therefore does not select a new authorization protocol. It selects shared ownership around these existing primitives.

## Selected shared-current owner

Future source materialization shall introduce one Agent-owned generic owner equivalent in responsibility to:

`SharedCurrentCapabilityAuthority<P>`

with one shared state equivalent in responsibility to:

`CurrentCapabilityAuthorityState<P>`

The shared owner contains exactly one:

`Arc<tokio::sync::RwLock<CurrentCapabilityAuthorityState<P>>>`

The state owns together:

- one current `WorkspaceDeviceRegistry` by value; and
- one current policy evaluator `P` by value.

The registry and policy are intentionally protected by the **same lock**, not independent locks.

## Why one combined lock is selected

One combined state/lock provides one authorization snapshot boundary and avoids:

- registry-lock then policy-lock ordering;
- policy-lock then registry-lock ordering;
- lock-order deadlocks;
- observing a registry state from one authority generation and a policy state from another during one authorization decision;
- two independent poison/failure models;
- per-task cached authority combinations.

A request authorization sees one coherent current registry/policy state under one read guard.

## Why Tokio RwLock is selected

The selected lock is `tokio::sync::RwLock`, using the exact Tokio dependency already materialized by C03e-U with the `sync` feature.

This selection adds no new lock/dependency family.

Compared with `std::sync::RwLock`, Tokio's lock avoids a synchronous thread-blocking acquisition path and has no lock-poisoning state that would require recovery or reinterpretation after a writer panic.

Compared with adding a new synchronization crate, this keeps the dependency boundary unchanged.

C03e-W does not select custom fairness, lock sharding, lock-free snapshots, `ArcSwap`, actor/message-passing authority, or a second synchronization primitive.

## Clone semantics

Cloning the future shared-current owner may clone only the outer `Arc`.

It must not clone:

- `WorkspaceDeviceRegistry` into per-worker snapshots;
- policy state into per-worker snapshots;
- validated principals as reusable authorization;
- transport-binding results;
- capability decisions.

Every clone must refer to the same current authority state.

## Per-request authorization sequence

Future source materialization must preserve this sequence for every protected request:

1. perform stream/frame receipt without holding an authority lock;
2. obtain fresh verifier time through the existing caller-owned provider;
3. asynchronously acquire one shared-authority read guard;
4. construct an ephemeral `CapabilityBridge` borrowing only the registry and policy inside that guard;
5. call the retained `BoundRemoteSession::authorize(...)` exactly once with the current frame and verifier time;
6. obtain one owned `AuthorizedCapabilityRequest` or the existing `RemoteBridgeError`;
7. release the read guard before dispatcher execution;
8. only after successful authorization, dispatch that exact owned authorized request;
9. construct/send the bounded success response using existing bridge-owned transport framing.

No lock guard may escape the authorization operation.

## Bridge-owned dispatch helper selected

To preserve the existing boundary that keeps `ControlFrame`/lower transport framing inside `prw-remote-bridge`, future source materialization shall add a bridge-owned helper equivalent in responsibility to:

`dispatch_authorized_request(...)`

The helper accepts:

- one already-owned `AuthorizedCapabilityRequest`; and
- the mutable existing `CapabilityDispatcher`.

It performs the same existing post-authorization behavior currently embedded in `CapabilityBridge::process_request(...)`:

- invoke dispatcher exactly once;
- map dispatcher failure to existing `RemoteBridgeError::DispatchFailed`;
- enforce `MAX_CONTROL_PAYLOAD_BYTES`;
- construct exactly one `ControlMessageKind::Response` frame with the original request ID;
- preserve existing transport-error mapping.

Existing `CapabilityBridge::process_request(...)` should delegate to `authorize(...)` plus this helper so behavior remains one canonical implementation.

This helper does **not** widen Agent dependencies: `prw-agent` must still not add a direct `prw-remote-transport` dependency.

## Lock hold boundary

A shared-authority read or write guard must never be held across:

- control-stream accept;
- frame receive;
- dispatcher execution;
- response send;
- cancellation wait;
- worker join/drain;
- arbitrary network I/O;
- filesystem/terminal/forwarding side effects;
- task spawn;
- readiness publication.

The read guard exists only during synchronous current authorization.

This is a hard selected boundary, not a performance suggestion.

## Authorization linearization semantics

Current authorization linearizes while the shared-authority read guard is held.

A registry/policy mutation linearizes while the shared-authority write guard is held.

Therefore:

- if a completed mutation obtains the write lock before a later request authorization, that authorization must observe the new current state;
- if a request completes authorization before a competing mutation linearizes, that request is already admitted and may proceed to dispatcher execution after the guard is released;
- mutation does not retroactively cancel an operation already authorized;
- no authorization decision may be reused for a later request.

This defines the protected-operation authorization point without holding authority locks across potentially long side effects.

## Registry currentness requirements

The existing `WorkspaceDeviceRegistry` remains the source of current membership/device/transport-binding state.

Every request authorization must continue to call the existing current checks through `CapabilityBridge::authorize(...)`, including:

- `validate_authenticated_session(...)`; and
- `validate_transport_identity(...)`.

Consequently, after a mutation linearizes:

- suspended/removed membership is rejected on later authorization;
- revoked device is rejected on later authorization;
- stale rotated transport identity is rejected on later authorization.

No authenticated-session snapshot overrides these current checks.

## Policy currentness requirements

Policy is stored in the same shared state as the registry.

Future policy replacement or mutation for the same concrete policy type must occur under the same write lock before it can be considered current.

A worker must not retain a copied policy decision across requests.

The existing `PolicyEvaluator::evaluate(...)` remains the decision primitive during each authorization.

C03e-W does not select a trait-object policy registry, remote policy service, policy cache, generation token, or dynamic plugin system.

## Mutation ownership

The future shared-current authority owner is the only selected synchronization boundary through which registry/policy state may be shared with remote worker tasks.

Future mutation APIs must:

- acquire the same state write lock;
- perform one bounded synchronous state mutation;
- release the write guard before any unrelated await or external I/O;
- preserve existing registry transition/error semantics rather than inventing silent recovery.

C03e-W does not yet select the exact public mutation method list or Agent management wiring; those may remain internal/materialized only as needed by later runtime integration.

Raw `RwLockWriteGuard` or `RwLockReadGuard` exposure is not selected as public domain API.

## Cancellation while waiting for authority

A worker may be cancelled while its request path is waiting to acquire a shared-authority read lock.

The existing C03e-S worker race remains authoritative:

- if cancellation wins while authorization acquisition is pending, dropping the request-loop future drops that pending lock acquisition;
- after the request-loop mutable borrow is released, the existing code-4 shutdown close occurs exactly once;
- no authority guard is retained by a cancelled future;
- no lock-acquisition failure is reclassified as a remote bridge success or fabricated response.

Once the read guard has been acquired, authorization is synchronous and bounded; no `.await` occurs while that guard is held.

## Bounds selected for future spawned use

The future shared-current owner is intended to be cloneable into spawned worker contexts while sharing the same state.

The future materialized path may therefore require the concrete policy type to satisfy:

`P: PolicyEvaluator + Send + Sync + 'static`

The authority state and registry must satisfy the compiler-required Send/Sync bounds for the selected Tokio lock/Arc composition.

C03e-W does not add unsafe Send/Sync implementations.

## Existing worker authority inputs that change later

The current C03e-O/Q/S/V path accepts a borrowed `CapabilityBridge`.

A later source-materialization checkpoint must replace that long-lived borrowed bridge input with the selected shared-current authority owner/reference so that the bridge is created **ephemerally per authorization** under the current read guard.

This later refactor must preserve:

- one request at a time per authenticated-session owner;
- verifier-time sampling once per request;
- existing stream/wire semantics;
- existing C03e-Q code-3 failure semantics;
- existing C03e-S code-4 cancellation semantics;
- V's bounded executor drive behavior until task spawn is separately materialized.

C03e-W itself performs no such source mutation.

## No spawned task yet

Even after this architecture selection, C03e-W does not introduce:

- `tokio::spawn`;
- `spawn_local`;
- `JoinHandle`;
- `JoinSet`;
- cancellation channel/token ownership;
- worker registry/collection;
- panic/join-failure classification source;
- concurrent authenticated-session admission;
- duplicate-DeviceId admission rules;
- capacity/fairness policy;
- `main.rs` wiring;
- readiness.

Those remain later checkpoints.

## Identity invariants

Shared-authority synchronization changes no identity meaning.

- `DeviceId` / authenticated PRW session identity remain logical identity;
- `TransportIdentity` remains lower-transport identity;
- IP remains a transient endpoint;
- PID/UID/GID/thread/task/runtime/lock identities are not PRW identity.

## Explicitly rejected alternatives

C03e-W rejects for this first shared-current boundary:

- per-worker cloned registry state;
- per-worker cached policy decisions;
- `static mut` or process-global mutable authority;
- leaked `'static` references;
- unsafe pointer lifetime extension;
- one long-held registry read lock around an entire worker/session loop;
- holding authority locks across dispatcher execution or network I/O;
- independent registry and policy locks with ambiguous ordering;
- `std::sync::Mutex`/`RwLock` as the selected async-worker lock;
- new `parking_lot`, `arc-swap`, actor/mailbox or lock-free dependency families;
- silent policy/registry snapshotting merely to satisfy `'static` task bounds.

## Expected source-materialization split after W

The next source checkpoint should materialize only the selected shared-current authority and bridge authorized-dispatch split, then refactor the existing request/worker chain to consume that authority while remaining non-spawned.

Only after that source is validated should a later checkpoint select/materialize:

- cancellation-controller construction;
- spawned worker task ownership;
- retained join/completion collection.

Concurrent authenticated-session admission remains later still.

## Exact W diff boundary

C03e-W is docs-only.

Expected V -> W diff is exactly one path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_W_SHARED_CURRENT_CAPABILITY_AUTHORITY_SELECTION_STAGING.md`

No Rust source, Cargo manifest/lockfile, workflow, Android application source, Agent `main.rs`, readiness, packaging/systemd or host-network path may change in W.

## Validation requirements

Closure requires on final exact W head:

- exact V merge base;
- exact one-path docs-only scope;
- canonical Rust validation FULL PASS;
- Android PASS is not claimed unless Android validation actually triggers on the exact docs-only head;
- skipped workflows are recorded as skipped, never PASS;
- immutable Drive audit raw-readback verification;
- append-only rolling Drive update preserving the complete post-V prefix byte-for-byte;
- PR remains draft/open/unmerged.

## Completion meaning

Closure of C03e-W means only that the shared-current registry/policy ownership and authorization-lock boundary required before future spawned remote workers is selected.

It does not mean the authority wrapper exists in source, worker signatures have changed, any task is spawned, multiple sessions are admitted, Agent `main.rs` is wired, readiness is published, or remote runtime is activated.

Target gate:

`C03E_W_SHARED_CURRENT_CAPABILITY_AUTHORITY_SELECTED`
