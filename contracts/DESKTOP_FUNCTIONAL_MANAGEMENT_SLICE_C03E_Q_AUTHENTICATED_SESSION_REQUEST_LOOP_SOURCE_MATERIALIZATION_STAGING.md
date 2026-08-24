# Phase 152 C03e-Q — Authenticated Session Request Loop Source Materialization Staging

Status: STAGED

Target gate:

`C03E_Q_AUTHENTICATED_SESSION_REQUEST_LOOP_SOURCE_MATERIALIZED`

## Predecessor

Canonical predecessor is closed C03e-P:

- branch: `phase-152-c03e-p-authenticated-session-request-loop-behavior-selection-staging`
- head: `3af9c88194f308bc79e9977df0b1a233f016d697`
- tree: `6eec4847fee573c15bbd0862995452b7367aa919`
- gate: `C03E_P_AUTHENTICATED_SESSION_REQUEST_LOOP_BEHAVIOR_SELECTED`

C03e-Q must preserve exact P lineage and must not merge, rebase or cherry-pick another checkpoint.

## Purpose

Materialize only the C03e-P-selected borrowed serial authenticated-session capability request loop on the existing `AuthenticatedRemoteSessionRuntimeOwner`.

This checkpoint turns the already-selected loop behavior into Rust source. It does not materialize per-session task ownership, external cancellation/drain/join, concurrent request processing, `main.rs` wiring or readiness publication.

## Existing transaction remains the operation primitive

C03e-Q does not duplicate the C03e-O transaction.

Every iteration invokes the existing:

`AuthenticatedRemoteSessionRuntimeOwner::process_one_capability_request(...)`

exactly once.

That transaction remains solely responsible for:

1. accepting one bounded peer-initiated control stream;
2. receiving one bounded PRWM request through the C03e-N bridge-owned adapter;
3. invoking retained `BoundRemoteSession::process_request(...)` exactly once with current `CapabilityBridge`, explicit verifier time and mutable dispatcher;
4. sending only the bridge-produced correlated success response on the same stream.

C03e-Q adds no second stream parser, request decoder, authorization path, dispatcher or response constructor.

## Materialized loop API

The source adds a borrowed method equivalent to:

`run_capability_request_loop(...)`

on the existing `AuthenticatedRemoteSessionRuntimeOwner`.

The method:

- receives `&mut self`;
- receives the current `&CapabilityBridge`;
- receives a caller-owned verifier-time provider with bound `FnMut() -> u64 + Send`;
- receives a caller-owned mutable `CapabilityDispatcher + Send`;
- retains existing `P: PolicyEvaluator + Sync` bounds required by the Send-safe C03e-O transaction future;
- returns the first existing `AuthenticatedRemoteSessionCapabilityTransactionError` encountered by the loop.

No new public transport type or direct `prw-remote-transport` dependency is introduced into Agent.

## Exact serial loop sequence

The method executes a plain serial loop:

1. sample the caller-owned verifier-time provider exactly once;
2. call the existing C03e-O transaction exactly once with that sampled time;
3. on `Ok(())`, begin the next iteration;
4. on `Err(error)`, explicitly close the same retained peer exactly once using the selected fixed diagnostic and return `error` unchanged.

There is no overlapping iteration, stream pipelining or per-request spawn.

## Fresh verifier time

The verifier-time provider is invoked once immediately before each transaction.

C03e-Q does not:

- cache one timestamp for the session;
- sample time after a request already begins;
- derive time from auth challenge timing;
- derive time from QUIC handshake timing;
- derive time from application lease issue time;
- derive time from request identifiers or uptime;
- call hidden process-global `SystemTime::now()` inside the loop.

The caller remains the verifier-time authority.

## Terminal failure behavior

Every C03e-O transaction failure remains session-terminal exactly as selected by C03e-P.

On the first failure the source:

- preserves the exact typed `AuthenticatedRemoteSessionCapabilityTransactionError`;
- calls `AuthenticatedRemotePeerConnection::close(...)` exactly once on the same retained peer;
- uses fixed code `3`;
- uses fixed reason bytes `b"remote capability session terminated"`;
- returns the original transaction error unchanged.

The loop does not inspect or reclassify `Accept`, `Wire` or `Bridge` errors and does not create a recoverable subset of `RemoteBridgeError` variants.

## No retry or protocol invention

C03e-Q performs no:

- retry of failed accept/read/write/dispatch;
- re-send of a success response;
- replacement stream for the failed request;
- replacement peer, transport identity or logical session;
- replacement application lease;
- reconnect or re-authentication;
- pending-session abort after completed authentication;
- authenticated-session deletion;
- negative capability-response envelope creation.

A new connection/session after terminal failure remains an outer admission concern.

## Identity and dynamic authority preservation

No logical identity rule changes.

`DeviceId` / authenticated PRW session identity remains logical identity. `TransportIdentity` remains the transport-level certificate identity already bound into the retained `BoundRemoteSession`. IP/PID/UID/GID remain non-authoritative for PRW identity.

Every successful iteration continues through retained `BoundRemoteSession::process_request(...)`, so current per-request authority still includes:

- application-session lease validity at that iteration's verifier time;
- current authenticated-session registry validity;
- current logical-device to retained transport binding validity;
- current capability policy evaluation;
- dispatcher execution;
- bounded correlated success-response construction.

The loop caches none of those decisions.

## Close-code allocation

C03e-Q materializes the C03e-P-selected fixed capability-session termination close diagnostic:

- code `3`;
- reason `remote capability session terminated`.

Existing earlier allocations remain unchanged:

- code `1`: logical-session authentication transaction failure;
- code `2`: post-authentication remote-session binding failure.

The code-3 reason is fixed, non-secret and contains no request, identity, capability, policy, path, dispatcher or lower transport diagnostic.

## Source scope

Expected final net diff relative to C03e-P is exactly two paths:

1. `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`
2. this contract

No mutation is selected for:

- `crates/prw-agent/Cargo.toml` or `Cargo.lock`;
- Agent `lib.rs` or `main.rs`;
- C03e-N capability wire adapter;
- remote-session binding/lease source;
- C03e-H logical-session authentication source;
- registry, policy or dispatcher implementation;
- workflows;
- Android application source;
- remote readiness;
- systemd/packaging;
- host-network/reachability activation.

## Explicitly unselected lifecycle surfaces

Even with the loop source present, C03e-Q does not own:

- a spawned per-session task;
- cancellation token or cancellation channel;
- graceful drain deadline;
- join handle or task collection;
- concurrent request streams;
- concurrent authenticated-session collection;
- listener accept loop;
- Agent readiness publication;
- `main.rs` runtime composition;
- retry/reconnect;
- deployment/restart/merge.

Those remain separately gated.

## Validation requirements

Because C03e-Q changes Rust source, canonical completion requires exact-head:

- PRW Rust Validation FULL PASS: locked dependency graph, rustfmt, Clippy, workspace tests and workspace build;
- PRW Android Validation FULL PASS when triggered by the source change, including exact toolchains, native adapter and Android application;
- skipped C02f-AD/C02f-AE workflows must not be counted as PASS evidence.

Any corrective formatter/lint commit must stay within the exact selected two-path scope and be recorded in final audit evidence.

## Drive closeout requirements

After exact-head canonical validation passes:

1. publish immutable `C03E_Q_AUTHENTICATED_SESSION_REQUEST_LOOP_SOURCE_MATERIALIZATION_AUDIT.md` in the existing evidence folder;
2. raw-readback verify exact byte size and SHA-256;
3. immediately re-fetch authoritative rolling `C02E_BRANCH_STATUS.md` and require the exact closed-P baseline;
4. append Q evidence only, preserving every predecessor byte;
5. raw-readback verify final rolling size/hash and the entire P prefix hash;
6. update the Q PR body to CLOSED checkpoint metadata while keeping it draft/open/unmerged.

## Deliberate stopping point

After C03e-Q closes, the immediate next boundary is not more request-loop semantics. It is explicit per-session worker/lifecycle ownership selection covering cancellation, peer close on external shutdown, drain/join semantics and how a completed/failed session worker is collected.

Still separately gated after Q:

- per-session task owner source;
- cancellation/drain/join implementation;
- concurrent authenticated-session collection;
- Agent `main.rs` runtime wiring;
- remote readiness publication;
- listener/reachability runtime activation;
- external NAT/ICE/STUN/TURN/relay integration;
- credential provisioning;
- deployment/restart/merge.

Gate on successful canonical closeout:

`C03E_Q_AUTHENTICATED_SESSION_REQUEST_LOOP_SOURCE_MATERIALIZED`
