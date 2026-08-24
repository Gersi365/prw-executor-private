# Private Remote Workspace — Phase 152 C03e-M Capability Request Operation Surface Selection Staging

Status: selection staging
Date: 2026-08-24
Repository: `Gersi365/prw-executor-private`

## Exact predecessor

- branch: `phase-152-c03e-l-post-auth-session-binding-composition-source-materialization-staging`
- head: `bc7a52b31e64b91b51ed2e46ec1ccae2e5642a25`
- tree: `fbc354df4380cf26c36e7a0ec18e2347c7479c7e`
- gate: `C03E_L_POST_AUTH_SESSION_BINDING_COMPOSITION_SOURCE_MATERIALIZED`

C03e-L is closed on its immutable branch. C03e-M is a docs-only selection checkpoint created from that exact head. It does not merge, rebase, force-update or mutate any closed checkpoint.

## Selection question

C03e-L now composes one authenticated live peer with one current-registry-bound capability session under `AuthenticatedRemoteSessionRuntimeOwner`, but deliberately stops before capability request I/O.

The next source boundary must choose how one already-authenticated outer owner processes a capability request without:

- exposing the retained `BoundRemoteSession` publicly;
- allowing callers to select a second transport identity;
- caching registry or policy authority;
- giving `prw-agent` direct ownership of raw Quinn primitives;
- widening `prw-agent` to a direct `prw-remote-transport` dependency merely to name `ControlFrame`, `MeshControlStream` or `MeshQuicRuntimeError`;
- selecting request-loop/task/concurrency/readiness semantics prematurely.

## Existing authority and transport facts

Current source already establishes the required lower layers:

1. `AuthenticatedRemotePeerConnection::accept_control_stream()` accepts one peer-initiated bounded PRWM bidirectional stream and returns the existing `MeshControlStream` behind the bridge boundary.
2. `MeshControlStream::receive_frame()` reads exactly one complete bounded PRWM `ControlFrame`.
3. `MeshControlStream::send_frame()` writes exactly one bounded PRWM `ControlFrame` and finishes the send direction.
4. `BoundRemoteSession::process_request(...)` supplies its stored transport identity and lease internally and delegates authorization/dispatch to the current `CapabilityBridge`.
5. `CapabilityBridge::process_request(...)` performs current lease/registry/transport-binding/policy/dispatcher authority and produces a `Response` frame only after successful authorization and dispatch.
6. `prw-agent` currently depends on `prw-remote-bridge` and `prw-policy`, but intentionally has no direct `prw-remote-transport` dependency.

## Selected bridge-owned capability wire adapter

C03e-M selects a new bridge-owned capability wire adapter as the only Agent-facing seam for post-authentication capability frame I/O.

The adapter must remain inside `prw-remote-bridge` and may use the existing `MeshControlStream`, `ControlFrame` and `MeshQuicRuntimeError` directly.

The Agent-facing adapter surface is selected as two narrow async operations:

- receive exactly one bounded capability request frame from one existing `MeshControlStream`;
- send exactly one already-constructed capability response frame on that same stream.

The adapter must expose a bridge-owned wire error classification that preserves the existing lower transport failure as its source without exposing raw Quinn state.

The adapter does not authenticate a logical session, authorize a capability, own registry/policy/dispatcher state, choose a lease, retry I/O, accept another stream or publish readiness.

## Selected one-stream / one-request transaction

One authenticated capability transaction uses exactly one peer-initiated bidirectional control stream for exactly one request and at most one successful response.

Selected sequence:

1. the existing `AuthenticatedRemoteSessionRuntimeOwner` accepts one control stream from its retained authenticated peer;
2. the bridge-owned capability wire adapter receives exactly one bounded PRWM request frame;
3. the retained C03e-J capability owner delegates exactly once to its existing `BoundRemoteSession::process_request(...)` using:
   - the current caller-supplied `CapabilityBridge`;
   - one explicit verifier-owned `now_unix_seconds`;
   - that received frame;
   - one caller-supplied mutable dispatcher;
4. on bridge success, the bridge-owned wire adapter sends exactly the one returned response frame on the same stream;
5. the transaction returns success only after response transmission succeeds.

No second request is read from that stream and no second response is sent.

## Selected ownership boundary

The operation belongs to the existing Agent-owned `AuthenticatedRemoteSessionRuntimeOwner` because that owner already retains both:

- the same authenticated live peer required for stream acceptance; and
- the capability owner carrying the bound logical session.

The retained `BoundRemoteSession` remains private. C03e-M does not select a public accessor for transport identity, lease, authenticated session or the raw binding.

A source implementation may delegate internally through the private capability-owner/binding relationship, but must not expose that binding as new caller authority.

## Dynamic authorization remains mandatory

Every transaction must invoke current `BoundRemoteSession::process_request(...)` exactly once for the received request.

Therefore no C03e-M/N implementation may cache or replace:

- application lease validity at verifier time;
- authenticated-session registry state;
- current device lifecycle/membership;
- current transport-identity binding;
- current policy decision;
- bounded request decoding;
- dispatcher admission after authorization.

Transport identity remains the immutable snapshot already bound in C03e-L. The caller cannot provide or replace it per request.

## Selected failure taxonomy

The future Agent single-request transaction should preserve three primary failure classes without translation into an unrelated taxonomy:

1. **stream accept** — existing bridge-owned `RemoteServerTransportRuntimeError`;
2. **capability wire I/O** — the new bridge-owned capability wire error preserving the existing `MeshQuicRuntimeError` as its source;
3. **capability bridge** — existing `RemoteBridgeError` from `BoundRemoteSession::process_request(...)`.

No cleanup error exists at this boundary because there is no pending authentication challenge to abort and no new registry state is created by the transaction.

## Selected per-request failure behavior

C03e-M deliberately does **not** select whole-peer closure for every per-request failure.

On stream-accept, wire, authorization, policy, lease, registry, dispatch or response-send failure:

- the single transaction fails closed and returns its typed error;
- no success response is fabricated;
- no retry occurs inside the transaction;
- no replacement stream/request/session/lease is invented;
- no pending-session abort is called;
- no authenticated-session deletion is invented;
- the outer owner retains peer/session lifetime authority for a later separately gated session-loop/lifecycle policy to decide whether the whole peer should close or continue.

This preserves error evidence without prematurely equating a denied capability request with logical-session destruction.

## Request/response correlation

The existing `CapabilityBridge::process_request(...)` remains responsible for constructing the success `Response` frame using the authorized request correlation identifier.

C03e-M does not introduce a second request-id parser, a new response envelope or a parallel application protocol.

Bridge failures currently produce no successful response frame. C03e-M does not invent a remote error-response protocol.

## Dependency boundary

C03e-M explicitly selects that `prw-agent/Cargo.toml` remains byte-stable for the next source materialization.

The Agent must not add a direct `prw-remote-transport` dependency solely to name post-auth capability transport primitives. Lower-transport frame/stream mechanics stay behind `prw-remote-bridge`, matching the existing C03d session-auth wire pattern.

## Protected boundaries

The selection does not authorize changes to:

- `Cargo.toml` or lockfiles;
- Agent `main.rs`;
- reachability authority/runtime;
- authentication challenge/proof transaction semantics;
- C03e-L binding composition;
- registry or policy rules;
- dispatcher capability semantics;
- Android application source;
- local runtime/readiness;
- systemd/packaging/firewall/NAT/routes/DNS/TUN/TAP;
- credential provisioning;
- deploy/restart/merge/rebase/force-push.

## Still separately gated

C03e-M does not select or materialize:

- a multi-request loop;
- concurrent request streams;
- concurrent authenticated-session collection ownership;
- spawned session workers/tasks;
- cancellation/join semantics;
- peer-close policy after repeated/permanent request failures;
- retry/backoff/reconnect/session refresh;
- Agent `main.rs` wiring;
- remote readiness publication.

## Expected source sequence after selection

The minimal follow-on source sequence is intentionally split:

1. materialize the bridge-owned one-frame capability wire adapter without Agent runtime mutation beyond imports/tests required by that bridge checkpoint;
2. then materialize the Agent-owned one-stream/one-request transaction on the authenticated outer owner using the adapter and existing dynamic `BoundRemoteSession::process_request(...)`;
3. only after both are independently validated may a later checkpoint select a request/session loop.

## Validation requirement

This checkpoint is documentation-only. Canonical Rust validation on the exact head is required. Android validation is not required unless its workflow is actually triggered; no PASS claim may be made for a non-triggered or skipped workflow.

## Completion gate

After exact-scope verification, exact-head validation, immutable Drive audit publication and append-only rolling closeout:

`C03E_M_CAPABILITY_REQUEST_OPERATION_SURFACE_SELECTED`
