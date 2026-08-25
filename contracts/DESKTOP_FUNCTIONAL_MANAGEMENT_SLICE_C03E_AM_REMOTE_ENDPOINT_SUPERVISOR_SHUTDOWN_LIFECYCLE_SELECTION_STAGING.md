# Phase 152 C03e-AM — Remote Endpoint + Supervisor Shutdown Lifecycle Selection Staging

Status: STAGED

Target gate:

`C03E_AM_REMOTE_ENDPOINT_SUPERVISOR_SHUTDOWN_LIFECYCLE_SELECTED`

## Exact predecessor

Canonical predecessor is closed C03e-AL:

- branch: `phase-152-c03e-al-repeated-real-admission-persistent-collection-supervisor-source-materialization-staging`
- head: `d86a6b293bceb38c9e0bc82e6c6784b7157c5bc0`
- tree: `7be5888fb4eca0ca0e0314cd4f1f28120cca675c`
- gate: `C03E_AL_REPEATED_REAL_ADMISSION_PERSISTENT_COLLECTION_SUPERVISOR_SOURCE_MATERIALIZED`

C03e-AM selects only the outer lifecycle that deterministically composes the already-materialized C03e-AL repeated real-admission/persistent-worker supervisor with the already-materialized C03e-C/D remote endpoint owner shutdown surface.

No source code is materialized by C03e-AM.

## Exact bounded selection scope

The final AL -> AM net diff must remain exactly one docs-only path: this contract.

No Rust source, manifest, lockfile, permanent workflow, Android application, bridge, transport, Agent `main.rs`, local readiness, remote readiness, packaging, host, systemd, deployment, or merge mutation is selected.

## Existing source facts this selection composes

The existing `AgentRemoteTransportRuntime`:

- owns one `RemoteServerTransportRuntime` plus the retained `ReachabilityAuthorityRuntimeOwner`;
- exposes synchronous `close(code, reason)` that closes the endpoint and all live connections;
- exposes async `wait_idle()` that waits until the underlying real QUIC endpoint has no live connections.

The existing `RemoteSessionExecutorRuntime`:

- owns one private non-cloneable Tokio current-thread runtime with I/O/time drivers;
- exposes only domain-specific drive seams;
- exposes no generic `block_on` and no runtime `Handle`;
- now materializes C03e-AL repeated expected-device admission plus persistent worker supervision entirely inside one private current-thread `Runtime::block_on` call.

The C03e-AL supervisor borrows `&AgentRemoteTransportRuntime` for the complete repeated admission / active worker lifetime and returns only after orderly supervisor shutdown has drained every in-flight AJ transaction and every retained active worker.

## Selected outer lifecycle purpose

C03e-AM selects one future Agent-internal domain-specific outer drive seam for an already-constructed executor and already-bound remote transport runtime.

Its responsibility is strictly:

1. drive the existing C03e-AL supervisor to terminal return;
2. only after that return, close the remote endpoint exactly once with the fixed normal endpoint shutdown diagnostic selected below;
3. drive the existing endpoint `wait_idle()` future to completion using the same private `RemoteSessionExecutorRuntime` current-thread runtime;
4. return only after endpoint idle is observed.

This is lifecycle composition, not readiness or production activation.

## No endpoint close while the AL supervisor is live

The endpoint must not be closed as a substitute for C03e-AL supervisor shutdown.

While C03e-AL is live:

- its supervisor-shutdown future remains the only selected outer stop input;
- active C03e-S workers retain their own code-3/code-4 terminal semantics;
- an in-flight AJ transaction remains owned by C03e-AL and must be drained, never dropped or aborted;
- C03e-AL owns the selected post-shutdown AJ-success code-4 close seam;
- the endpoint remains available to the exact already-in-flight transport/session work that C03e-AL is required to drain.

Calling endpoint `close()` before C03e-AL returns would collapse transport lifetime underneath these existing cleanup owners and could replace their typed close/cleanup semantics with transport-level endpoint closure. C03e-AM rejects that ordering.

## Supervisor shutdown source remains explicit and caller-supplied

C03e-AM does not invent a process signal source, background monitor, timer, shutdown thread, global token, channel-drop convention, or Agent-main integration.

The future source materialization keeps the existing C03e-AL `supervisor_shutdown: Future<Output = ()>` input shape or an equivalent explicitly-owned future passed into the combined domain-specific lifecycle seam.

The future may later be backed by a separately gated programmatic or signal-aware control source, but AM does not select that source.

Dropping an unrelated sender/handle is not implicitly classified as remote supervisor shutdown by this checkpoint.

## Deterministic AL terminal ordering

Once the caller-supplied supervisor-shutdown future becomes ready, the existing C03e-AL rules remain authoritative:

1. stop expected-device request polling;
2. request cancellation for every retained active worker;
3. retain and drain any one in-flight AJ future rather than dropping or aborting it;
4. preserve exact AJ terminal error cleanup, or consume a post-shutdown AJ success with existing code `4` / `remote capability session shutdown`;
5. drain every retained worker JoinHandle to terminal completion;
6. return from the C03e-AL drive only after the active worker map is empty.

C03e-AM adds no second worker registry, task collection, cancellation fan-out owner, or AJ cleanup authority.

## Selected endpoint-level normal close diagnostic

After C03e-AL has fully returned, the outer lifecycle closes the bound remote endpoint exactly once with:

- application close code: `0`;
- reason: `remote endpoint shutdown`.

This fixed diagnostic is endpoint-level normal lifecycle shutdown, not a logical-session error classification.

The selection intentionally does not reuse C03e session close codes `1` through `5`, because those codes belong to logical-session/authentication/capability lifecycle owners and must remain semantically distinct from whole-endpoint teardown.

The fixed reason is a bounded literal and is not caller-controlled data.

## Endpoint close happens before endpoint idle wait

After C03e-AL returns:

1. invoke `AgentRemoteTransportRuntime::close(0, b"remote endpoint shutdown")` exactly once;
2. immediately begin driving `AgentRemoteTransportRuntime::wait_idle()`;
3. keep the endpoint owner and its retained reachability-authority owner alive until `wait_idle()` completes;
4. only then return the outer lifecycle result and permit normal ownership drop.

The endpoint is not rebound, reopened, replaced, or retried during this sequence.

## Same private current-thread runtime drives idle completion

`AgentRemoteTransportRuntime::wait_idle()` is async and therefore requires a live Tokio runtime driver.

C03e-AM selects reuse of the existing private `RemoteSessionExecutorRuntime` current-thread runtime. It rejects:

- a second Tokio runtime;
- `rt-multi-thread`;
- a generic public `block_on`;
- runtime `Handle` exposure or clone;
- spawning a detached endpoint-drain task;
- handing endpoint idle completion to the local Phase 096–102 runtime.

The future materialization may use either:

- one bounded outer domain-specific method that first invokes the existing C03e-AL drive and then performs endpoint close plus a second private `Runtime::block_on(transport_runtime.wait_idle())`; or
- an equivalent private refactoring preserving the same externally observable ordering.

Two sequential domain-specific uses of the same private current-thread runtime are selected. A nested `block_on` is not selected.

## Configuration-error shutdown ownership

The existing C03e-AL drive may return `RemoteSessionPersistentCollectionConfigError` before entering its supervisor runtime when `max_active_workers` exceeds the registered-device ceiling.

For the future AM materialization, an already-bound endpoint must not be leaked merely because this pre-drive configuration validation fails.

Therefore the combined outer lifecycle preserves the original configuration error but still performs the selected endpoint teardown before returning it:

1. retain the exact `RemoteSessionPersistentCollectionConfigError`;
2. close the endpoint once with code `0` / `remote endpoint shutdown`;
3. drive `wait_idle()` to completion on the same private executor runtime;
4. return the unchanged configuration error.

No fabricated supervisor success, retry, corrected capacity, or fallback configuration is selected.

## Normal supervisor completion shutdown ownership

When C03e-AL returns `Ok(())` after its explicit supervisor-shutdown path:

1. the active worker map is already empty;
2. any in-flight AJ transaction has already reached terminal completion;
3. all logical-session-specific close semantics have already been owned by their existing transaction/worker owners;
4. AM closes the endpoint once with the selected endpoint-level normal diagnostic;
5. AM drives endpoint idle to completion;
6. AM returns successful outer lifecycle completion.

The endpoint close does not retroactively classify any worker or admission result.

## Admission failure and worker completion events remain non-terminal to AM

Existing C03e-AL callbacks remain unchanged:

- repeated AJ admission failures are reported through the existing bounded admission-failure callback and do not by themselves stop the supervisor;
- worker completion/join outcomes are reported through the existing completion callback;
- duplicate expected-device requests are reported through the existing rejection callback.

AM does not promote these events into endpoint shutdown triggers and does not add retry/reconnect/replacement policy.

## Endpoint idle completion is not readiness

Successful `wait_idle()` completion proves only that the selected endpoint has no remaining live QUIC connections after close.

It does not prove or publish:

- local Agent readiness;
- remote Agent readiness;
- reachability registration health;
- current device registry health;
- policy health;
- reconnect availability;
- deployment success.

No readiness bit, status snapshot, service-manager notification, or public API field is selected.

## Reachability authority custody remains live through endpoint idle

`AgentRemoteTransportRuntime` retains its `ReachabilityAuthorityRuntimeOwner` for the full endpoint lifetime.

C03e-AM requires that this owner remain retained until after endpoint `wait_idle()` completes. The lifecycle must not extract, drop, replace, re-bootstrap, or mutate the authority owner as an endpoint shutdown shortcut.

After idle completion, ordinary ownership drop may destroy the endpoint owner and its retained authority owner. AM does not select a reusable authority hand-back or automatic rebind path.

## No cross-runtime lifecycle ownership

The existing local Linux Phase 096–102 lifecycle remains unchanged and separate.

Its historical patterns are precedent only for explicit resource teardown ordering; AM does not:

- pass the remote Tokio runtime into the local scheduler;
- pass local worker registry/capacity state into the remote supervisor;
- reuse local runtime wake descriptors as remote shutdown state;
- reuse local `LocalLinuxRuntimeShutdownHandle` as a remote logical identity or automatic remote shutdown source;
- alter local listener cleanup or signal-mask restoration ordering.

Any later integration between local process lifecycle and the AM-selected remote lifecycle is separately gated.

## Panic/unwind boundary

C03e-AM selects deterministic normal-return and typed-error endpoint teardown only.

It does not claim a new async `wait_idle()` guarantee during Rust panic unwind, does not catch panics, and does not add a panic-specific runtime driver. Existing Rust/Quinn drop behavior remains unchanged on panic unless a later checkpoint explicitly selects a stronger unwind contract.

A future materialization may use a private synchronous close guard only if needed to preserve best-effort endpoint close on unwind, but such a guard must not claim async idle completion from `Drop`.

## Identity and authority invariants

C03e-AM preserves:

- DeviceId / authenticated PRW session identity as logical identity;
- TransportIdentity as lower-transport certificate identity only;
- IP/socket address as transient endpoint data;
- SessionId as authentication correlation only;
- endpoint close code/reason as lifecycle diagnostics, never logical identity or authorization evidence;
- current registry membership/current transport binding/current policy evaluation on every protected request while workers are live;
- no authority guard across accept, authentication wire I/O, capability dispatch, task lifetime, endpoint close, or idle drain.

PID/UID/GID/thread/runtime/task/join/controller/channel/lock/endpoint identifiers remain non-logical implementation details.

## Explicitly still absent

C03e-AM does not select or materialize:

- source code;
- endpoint bind/startup composition;
- a concrete remote supervisor shutdown controller or process-signal source;
- Agent `main.rs` wiring;
- local or remote readiness publication;
- concurrent/parallel pre-auth AJ transactions;
- automatic reconnect/rebind/retry/replacement;
- reachability authority re-bootstrap after endpoint shutdown;
- a hard endpoint drain deadline or task abort;
- a second Tokio runtime or `rt-multi-thread`;
- generic `block_on` or runtime `Handle` exposure;
- systemd/host mutation;
- deployment;
- merge.

## Future source-materialization requirements

A later source checkpoint materializing AM must remain narrow and prove at least:

- the existing C03e-AL drive returns before endpoint close is invoked;
- the fixed endpoint diagnostic is exactly code `0` / `remote endpoint shutdown`;
- endpoint close occurs exactly once on normal and configuration-error returns;
- `wait_idle()` is driven only after close;
- `wait_idle()` uses the same private current-thread executor runtime;
- no nested `block_on`, second runtime, Handle exposure, detached drain task, or local-runtime coupling appears;
- the original `RemoteSessionPersistentCollectionConfigError` survives endpoint cleanup unchanged;
- the endpoint/reachability owner remains live until idle completion;
- existing AL source and tests remain semantically stable except for the minimal composition seam required by that checkpoint.

Focused source tests may use compile-time signatures and non-networking orchestration helpers where possible. A disposable real-loopback endpoint test may be added only if source materialization requires proof that close-before-wait reaches idle; no production listener activation is selected.

## Validation and closure

Because C03e-AM is docs-only, canonical closure requires on the exact final head:

- exact AL merge base and one-path docs-only diff;
- PRW Rust Validation FULL PASS on the exact AM head;
- disposable C02f workflows, if present, recorded as SKIPPED and never counted as PASS;
- no Android PASS claim unless the canonical Android workflow actually triggers;
- immutable Drive audit with raw byte/hash verification;
- append-only rolling Drive update preserving the complete post-AL prefix byte-for-byte;
- draft/open/unmerged PR metadata updated to CLOSED only after evidence is final.

No merge, deployment, readiness, Agent `main.rs`, process-signal wiring, systemd/host mutation, or remote runtime activation is authorized by this gate.