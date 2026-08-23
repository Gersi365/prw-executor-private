# Private Remote Workspace — Phase 152 C03e-I Post-Authentication Remote Session Lifecycle Ownership Selection Staging

Status: architecture/lifecycle ownership selection staging
Date: 2026-08-23
Repository: `Gersi365/prw-executor-private`

## Exact predecessor

- branch: `phase-152-c03e-h-session-auth-transaction-source-materialization-staging`
- head: `5e37edf1b44e202202ac9e646f5470c6ffe28004`
- tree: `009c0f60843cbb063863896cc886e87ed1237c06`
- gate: `C03E_H_SESSION_AUTH_TRANSACTION_SOURCE_MATERIALIZED`

## Purpose

C03e-I selects the narrow Agent ownership and construction ordering required immediately after one C03e-H logical-session authentication transaction succeeds.

C03e-H returns the existing Phase 128 `AuthenticatedDeviceSession` while leaving the already-authenticated C03e-D `AuthenticatedRemotePeerConnection` open and still owned by its caller. The existing C03e `BoundRemoteSession` can bind that authenticated logical session to the peer's already-revalidated `TransportIdentity`, and the separately closed C03f selection requires a future Agent-owned `RemoteSessionCapabilityRuntimeOwner` to consume that `BoundRemoteSession` by value.

Those existing boundaries do not yet select who retains the live peer connection for the same admitted application-session lifetime. C03e-I closes only that lifecycle ownership gap. It does not activate a capability loop, task, readiness state, listener, retry policy, or deployment.

## Authoritative existing facts

C03e-I relies on these already-materialized facts without changing them:

1. C03e-D `AuthenticatedRemotePeerConnection` owns one established lower-transport-authenticated QUIC connection and exposes the already-revalidated peer `TransportIdentity`, bounded peer-initiated control-stream acceptance, and explicit connection close.
2. C03e-H borrows that peer while executing one prepared challenge/proof transaction. On success it performs no abort and no peer close, returning one existing `AuthenticatedDeviceSession`.
3. C03e `BoundRemoteSession::new(...)` consumes one `AuthenticatedDeviceSession` by value, receives one `TransportIdentity` snapshot plus verifier-owned lease issue/expiry times, constructs the existing `RemoteSessionLease` internally, and fails with the existing `RemoteBridgeError` surface when the lease interval is invalid.
4. The existing `RemoteSessionLease` lifetime is independently bounded to at most 3,600 seconds.
5. C03f already selected an Agent-owned `RemoteSessionCapabilityRuntimeOwner` that consumes exactly one `BoundRemoteSession` by value and retains it for one application-layer remote-session capability lifetime.
6. C03f explicitly forbids `RemoteSessionCapabilityRuntimeOwner` from owning a UDP socket, Quinn endpoint, QUIC connection, QUIC stream, session-authentication service, registry, policy evaluator, dispatcher, task/executor/worker, retry state, or remote readiness.
7. Existing `CapabilityBridge` remains authoritative on every future request for lease time, current registry/session state, current transport binding, bounded request decoding, exact policy capability, and dispatcher admission.

## Selected ownership chain

C03e-I selects the following post-authentication chain:

`C03e-D AuthenticatedRemotePeerConnection`
+
`C03e-H successful AuthenticatedDeviceSession`
→ `C03e BoundRemoteSession::new(peer TransportIdentity snapshot, authenticated session, verifier lease window)`
→ `C03f RemoteSessionCapabilityRuntimeOwner consumes BoundRemoteSession`
→ `AuthenticatedRemoteSessionRuntimeOwner consumes the live peer + capability owner`
→ future separately gated request/session operation seam
→ existing `CapabilityBridge` remains per-request authority.

The selected future outer Agent type name is:

`AuthenticatedRemoteSessionRuntimeOwner`

Its pure ownership contract is:

- consume exactly one existing `AuthenticatedRemotePeerConnection` by value;
- consume exactly one C03f-selected `RemoteSessionCapabilityRuntimeOwner` by value;
- retain those two values together for one connected authenticated remote application-session lifetime;
- perform no network I/O, registry lookup, policy evaluation, capability authorization, task spawn, retry, or readiness mutation during pure construction;
- prevent ordinary callers from separating the live peer from the capability-session context after successful composition except through later narrowly reviewed Agent-internal lifecycle seams.

## Why two ownership layers remain distinct

C03f's capability owner and C03e-I's outer session owner have different responsibilities and must not be collapsed.

`RemoteSessionCapabilityRuntimeOwner` owns only the application-layer `BoundRemoteSession`. It must remain independent of raw or live transport ownership so that authorization semantics stay reusable and do not acquire connection/task responsibilities.

`AuthenticatedRemoteSessionRuntimeOwner` owns the connected-session lifetime. It retains the live peer together with the already-selected capability owner so that the connection cannot accidentally fall out of scope while its bound application session remains retained elsewhere.

This preserves the existing separation:

- lower transport identity/authentication remains transport-owned;
- logical device-session authentication remains Phase 128 session-auth owned;
- `BoundRemoteSession` owns the immutable transport snapshot plus bounded lease;
- C03f capability owner retains only that bound application context;
- C03e-I outer owner retains live connection lifetime plus the capability owner;
- `CapabilityBridge` remains the dynamic request authority.

## Selected construction ordering

A later source checkpoint must preserve this exact order after C03e-H success:

1. retain ownership of the same successful `AuthenticatedRemotePeerConnection`;
2. read its already-revalidated `TransportIdentity` snapshot;
3. receive a separately verifier-owned remote-session lease interval;
4. call existing `BoundRemoteSession::new(...)` exactly once with that transport snapshot, the C03e-H `AuthenticatedDeviceSession` by value, and the selected lease issue/expiry values;
5. only if binding succeeds, consume the resulting `BoundRemoteSession` into the C03f-selected `RemoteSessionCapabilityRuntimeOwner`;
6. only then consume the live peer and capability owner into `AuthenticatedRemoteSessionRuntimeOwner`;
7. return the composed outer owner without performing capability I/O or publishing readiness.

No capability owner may exist before `BoundRemoteSession::new(...)` succeeds. No outer authenticated-session owner may exist before both the live peer and capability owner exist.

## Lease-window separation

C03e-I explicitly keeps the remote application-session lease window distinct from the earlier C03e-E/C03e-H authentication challenge window.

The later composition caller/verifier must supply the remote-session lease issue/expiry values separately. The implementation must not silently derive the application lease from:

- the C03e-E challenge issue time;
- the C03e-E challenge expiry time;
- the C03e-H proof-verification `now_unix_seconds` value alone;
- QUIC handshake time;
- system wall-clock reads hidden inside the owner constructor.

The existing `BoundRemoteSession::new(...)` / `RemoteSessionLease::new(...)` validation remains authoritative. C03e-I does not widen the existing 3,600-second maximum lease lifetime.

This separation prevents the bounded proof-challenge validity window from becoming an implicit capability lifetime policy.

## Binding failure after successful authentication

If `BoundRemoteSession::new(...)` fails after C03e-H has already committed the `AuthenticatedDeviceSession`, the later composition source must fail closed:

- do not construct the C03f capability owner;
- do not construct `AuthenticatedRemoteSessionRuntimeOwner`;
- explicitly close the same authenticated peer before returning;
- return the existing `RemoteBridgeError` classification, either directly or inside a narrowly reviewed Agent composition error that preserves it unchanged;
- do not attempt C03e-F pending-session abort, because C03e-H success has already consumed and removed the pending challenge;
- do not invent deletion of the already-authenticated Phase 128 session, because the existing session service exposes no such lifecycle operation;
- do not retry, regenerate a session identifier, or create a replacement lease internally.

The authenticated identity remaining in the Phase 128 service after a post-auth lease-construction failure is not treated as capability admission. No `BoundRemoteSession` means no capability owner and no remote capability context.

## Successful construction semantics

Successful construction of `AuthenticatedRemoteSessionRuntimeOwner` proves only that:

- the lower transport peer was previously authenticated by the existing C03c/C03e-D boundary;
- C03e-H successfully authenticated the logical device session;
- C03e `BoundRemoteSession` accepted the verifier-owned remote lease interval;
- the resulting bound application-session context has been transferred into the C03f-selected Agent capability owner;
- the live peer and capability owner are now retained together by one outer Agent lifecycle owner.

It does **not** prove that any capability request is currently authorized.

Current registry state, current transport binding, lease time, request decoding, policy and dispatcher admission remain re-evaluated through the existing `CapabilityBridge` for every later request.

## Outer owner does not absorb endpoint authority

`AuthenticatedRemoteSessionRuntimeOwner` must not own or replace `AgentRemoteTransportRuntime` or `ReachabilityAuthorityRuntimeOwner`.

The endpoint-level Agent runtime continues to own the bound server endpoint and admitted reachability-authority lifetime. C03e-H requires an immutable borrow of that runtime during authentication, and C03e-I does not transfer endpoint authority into a per-session owner.

This preserves the existing ordering:

`ReachabilityAuthorityRuntimeOwner`
→ `AgentRemoteTransportRuntime`
→ accepted `AuthenticatedRemotePeerConnection`
→ authenticated logical session
→ bound capability session
→ per-session outer owner.

A later listener/session-manager checkpoint may determine how many per-session owners coexist under one endpoint owner. C03e-I does not select concurrency or collection semantics.

## Future operation seam constraint

A later separately gated checkpoint may expose Agent-internal operations from `AuthenticatedRemoteSessionRuntimeOwner`, but must preserve these constraints:

- callers cannot replace the retained peer with an independently selected connection;
- callers cannot replace the `BoundRemoteSession` transport identity per request;
- current registry and policy context are not cached as successful authorization;
- request time remains verifier/caller owned;
- dispatch occurs only after existing `CapabilityBridge` authorization;
- peer close/shutdown remains explicit;
- capability request stream ownership, request loop shape and concurrency remain separately selected;
- no local Unix same-UID session type is reused as the remote outer owner.

## Source-materialization order after C03e-I

C03e-I does not require one broad source patch.

The preferred minimal sequence is:

1. materialize the already-selected C03f `RemoteSessionCapabilityRuntimeOwner` on the current C03e-H descendant line, constructor/ownership only;
2. separately materialize `AuthenticatedRemoteSessionRuntimeOwner` and the minimal post-auth composition seam that constructs `BoundRemoteSession`, transfers it into the C03f owner, and retains that owner with the live peer;
3. only after those source-only boundaries validate may a later checkpoint select request-loop/task/session-manager behavior.

This sequencing avoids combining ownership wrappers with runtime wiring.

## Protected boundaries

C03e-I is documentation-only. Relative to exact C03e-H it must change exactly one path: this contract.

It must not change:

- any Rust source;
- any Cargo manifest;
- root `Cargo.lock`;
- `apps/android/native/Cargo.lock`;
- Agent `main.rs`;
- C03d wire source;
- C03e binding source;
- C03e-F session-service source;
- C03e-H transaction source;
- registry/policy/dispatcher source;
- workflow files;
- Android application source;
- systemd/networking/packaging source.

## Negative guarantees

C03e-I does not:

- create a real `RemoteSessionLease` or `BoundRemoteSession`;
- materialize either selected Agent owner in Rust source;
- accept, open, read, write, reset or close a real connection/stream;
- authorize or dispatch a capability;
- select a request loop or concurrent stream model;
- select a session registry/collection;
- spawn a task, worker or executor;
- select retry/reconnect/session-refresh behavior;
- publish remote readiness;
- activate ICE/STUN/TURN/relay;
- provision credentials;
- mutate systemd/firewall/NAT/routes/DNS/TUN/TAP;
- initialize PRWF, execute recovery epochs, or activate R1-R4 effects;
- deploy, restart, rebase or merge.

## Completion gate

After exact-head documentation validation and Drive closeout:

`C03E_I_POST_AUTH_REMOTE_SESSION_LIFECYCLE_OWNERSHIP_SELECTED`
