# Phase 152 C03e-AI — Expected-Device Real Remote Admission Transaction Selection Staging

Status: STAGED

Target gate:

`C03E_AI_EXPECTED_DEVICE_REAL_REMOTE_ADMISSION_TRANSACTION_SELECTED`

## Predecessor

Canonical predecessor is closed C03e-AH:

- branch: `phase-152-c03e-ah-current-thread-persistent-worker-collection-admission-source-materialization-staging`
- head: `4f32cdb37774a58356562dcf0e9c811581e28399`
- tree: `9f73f488c48be4c28751c18d7020651f43e33cb1`
- gate: `C03E_AH_CURRENT_THREAD_PERSISTENT_WORKER_COLLECTION_ADMISSION_SOURCE_MATERIALIZED`

C03e-AI preserves exact AH lineage. It is a selection-only checkpoint.

## Purpose

Select one bounded real remote admission transaction for one caller/injected expected logical `DeviceId`.

The transaction bridges already-materialized lower-level seams without selecting a listener loop or production activation:

1. resolve the current expected lower-transport identity for the intended logical device from current registry authority;
2. accept exactly one real QUIC/TLS peer through the existing exact-expected-transport-identity primitive;
3. revalidate current registry/device/transport state before preparing a logical-session challenge;
4. execute the existing challenge/proof authentication transaction;
5. compose the authenticated session and same live peer into one `AuthenticatedRemoteSessionRuntimeOwner` using a separately supplied application-lease interval;
6. return that owner for a later separately gated collection-integration seam.

C03e-AI does not create a listener/accept loop, does not insert into the C03e-AH worker collection, does not spawn a worker, does not wire Agent `main.rs`, does not publish readiness and does not deploy or merge.

## Existing constraints that determine this selection

### No blind accept-any primitive exists

The existing real transport API is intentionally:

`AgentRemoteTransportRuntime::accept_authenticated_peer(expected_peer: TransportIdentity)`

The expected peer transport identity is required before lower-transport acceptance. Successful return proves only the locked QUIC/TLS/ALPN and exact certificate-derived `TransportIdentity` checks.

C03e-AI therefore must not invent an accept-any path and must not weaken the bridge/transport contract merely to create a conventional listener loop.

### TransportIdentity is current registry state

`WorkspaceDeviceRegistry` already retains one separately rotatable optional `TransportIdentity` for each registered `DeviceId`.

The expected transport identity for a real accept must be resolved from the current registered device state. A caller may select the intended logical `DeviceId`, but must not supply a second independent `TransportIdentity` that could disagree with current registry authority.

### Current-thread collection integration is later

C03e-AH drives its injected already-authenticated admission receiver inside one long-lived private Tokio current-thread `Runtime::block_on(...)`.

A real async admission producer cannot be treated as an independently progressing background producer after that drive returns, and an unconstrained producer task could run ahead of collection capacity.

Therefore C03e-AI selects only one bounded admission transaction. A later checkpoint must explicitly integrate repeated real admissions and C03e-AH capacity/backpressure inside one shared current-thread supervisor lifetime.

## Selected transaction inputs

A later source seam may be equivalent in responsibility to:

`admit_expected_remote_device_session(...)`

It receives only the ownership/verification inputs required for one transaction:

- borrowed `AgentRemoteTransportRuntime`;
- borrowed `SharedCurrentCapabilityAuthority<P>`;
- mutable `SessionAuthenticationService`;
- expected logical `DeviceId`;
- typed `SessionId`;
- logical-session challenge validity `Range<u64>`;
- one bounded PRWM authentication correlation request ID `u64`;
- authentication verifier time `u64`;
- separate application-lease `Range<u64>`.

The later exact API may choose ordinary references/owned values consistent with existing seams, but must preserve these semantic inputs.

The transaction does not accept:

- caller-supplied `TransportIdentity`;
- caller-supplied `DeviceIdentityBinding`;
- capability set or authorization evidence;
- dispatcher;
- worker verifier-time provider;
- cancellation controller/signal;
- runtime handle;
- task/join ID;
- listener socket/address selection.

## Phase 1 — resolve current expected transport identity

Before awaiting network acceptance, the transaction takes one fresh read through `SharedCurrentCapabilityAuthority<P>` and uses only its current registry component.

Inside that bounded read:

1. look up the exact expected `DeviceId` with `WorkspaceDeviceRegistry::device(...)`;
2. fail with the existing `RegistryError::DeviceUnknown` if absent;
3. read the current `RegisteredDevice::transport_identity()`;
4. fail with existing `RegistryError::TransportIdentityMissing` if no current transport is bound;
5. call existing `WorkspaceDeviceRegistry::validate_transport_identity(device_id, transport_identity)` so current device lifecycle and exact current binding remain authoritative;
6. return the owned/copy `TransportIdentity` from the closure.

No registry or policy object is cloned or snapshotted.

The shared-current read guard must be released before any network wait.

The policy component is not evaluated for admission and is not capability authorization evidence. It is present only because the existing shared-current owner jointly owns current registry and policy state.

## Phase 2 — exact lower-transport acceptance

After the first authority guard is released, call exactly once:

`AgentRemoteTransportRuntime::accept_authenticated_peer(expected_transport_identity).await`

No accept-any, wildcard identity, IP-based selection, retry, reconnect or replacement peer is selected.

Acceptance failure returns the existing bounded `AgentRemotePeerAcceptError`; no accepted peer exists for C03e-AI cleanup in that case.

Successful acceptance yields exactly one `AuthenticatedRemotePeerConnection` whose `TransportIdentity` has already been revalidated by the lower transport.

Holding that peer is not logical-session authentication and is not capability authorization.

## Phase 3 — fresh current registry revalidation before challenge

Transport identity may rotate or the logical device may be revoked while lower-transport acceptance is pending.

Therefore, after one peer is accepted and before logical challenge preparation, C03e-AI requires a second independent fresh current-authority read.

Inside that read, delegate to the existing:

`AgentRemoteTransportRuntime::begin_registry_bound_session_challenge(...)`

using:

- the accepted peer;
- the fresh current registry;
- the same expected logical `DeviceId`;
- the typed `SessionId`;
- the selected challenge-validity interval;
- the same mutable `SessionAuthenticationService` transaction owner.

This existing seam validates the accepted peer's already-revalidated `TransportIdentity` against current registry state before preparing the Phase 128 challenge.

The second shared-current guard must be released before logical-session wire I/O begins.

No authority guard may cross the challenge/proof exchange.

## Challenge-preparation failure cleanup

If a real peer has already been accepted but `begin_registry_bound_session_challenge(...)` fails, no existing later transaction owns that accepted peer cleanup.

C03e-AI therefore selects one fixed non-secret whole-peer diagnostic for this exact boundary:

- close code: `5`;
- close reason: `remote session admission preparation failed`.

The wrapper closes the accepted peer exactly once before returning the existing bounded challenge-preparation failure.

This code is distinct from the existing remote-session diagnostics:

- code 1: session authentication transaction failure;
- code 2: remote session binding failure;
- code 3: remote capability session terminated;
- code 4: remote capability session shutdown.

C03e-AI does not call `abort_pending_session` when challenge preparation itself returns an error. The existing `begin_session` result remains authoritative for whether a pending challenge was created; a failed preparation must not fabricate pending-session ownership.

## Phase 4 — existing authentication transaction owns its failures

After successful challenge preparation, delegate exactly once to:

`complete_registry_bound_session_authentication(...)`

using the same accepted peer, same session-authentication service, prepared challenge, correlation request ID and authentication verifier time.

On any terminal failure after the pending challenge exists, that existing transaction remains the sole cleanup authority:

- it calls `abort_pending_session` exactly once;
- it explicitly closes the peer with existing code 1 / `session authentication transaction failed`;
- it preserves primary and cleanup failures separately.

The C03e-AI wrapper must not double-abort or double-close an authentication failure.

Successful authentication yields exactly one existing `AuthenticatedDeviceSession`.

## Phase 5 — existing post-authenticated binding owns its failures

After authentication success, consume the same accepted peer and authenticated session into existing:

`compose_authenticated_remote_session(...)`

using the separately supplied application-lease interval.

On binding failure, that existing seam remains the sole cleanup authority and closes the same peer with existing code 2 / `remote session binding failed`.

The C03e-AI wrapper must not issue another close or delete the authenticated session on binding failure.

Successful composition returns exactly one `AuthenticatedRemoteSessionRuntimeOwner`.

## Selected transaction result

The later source seam returns one bounded result equivalent in responsibility to:

`Result<AuthenticatedRemoteSessionRuntimeOwner, RemoteSessionRealAdmissionError>`

The bounded error preserves the existing failure classes without exposing raw implementation identity:

- `Registry(RegistryError)` for pre-accept current expected-transport resolution;
- `Accept(AgentRemotePeerAcceptError)` for lower-transport acceptance;
- `Challenge(AgentRemoteSessionChallengeError)` for post-accept challenge preparation;
- `Authentication(AgentRemoteSessionAuthenticationFailure)` for challenge/proof transaction failure;
- `Binding(RemoteBridgeError)` for post-authentication owner composition.

Exact enum/type names may be finalized during source materialization, but the error must retain these semantic boundaries and must not flatten away authentication cleanup evidence.

## Fresh-current authority semantics

C03e-AI performs two separate current-registry checks:

1. immediately before real lower-transport acceptance to resolve the expected current `TransportIdentity`;
2. after lower-transport acceptance and immediately before challenge preparation to revalidate the accepted peer against current registry state.

Neither result is reusable capability authorization evidence.

After the transaction succeeds, C03e-X/Z and C03e-AH remain authoritative: every protected capability request in the eventual worker independently rechecks fresh current registry membership, current transport binding and current policy.

C03e-AI does not cache registry or policy snapshots inside the returned owner.

## Identity invariants

C03e-AI preserves the project's identity split:

- `DeviceId` / authenticated PRW session identity are logical identity;
- the expected `DeviceId` identifies which logical device the one admission transaction intends to accept;
- `TransportIdentity` is lower-transport identity only and is derived from current registry authority;
- IP remains a transient endpoint and is not admission identity;
- SessionId remains a typed logical-session correlation identity but is not substituted for DeviceId;
- request ID is wire correlation only;
- task/runtime/thread/PID/UID/GID/pointer/Arc/lock identities are not logical identity.

A successful lower-transport accept is not logical authentication. A successful logical authentication is not capability authorization. A successful worker-collection admission is not capability authorization.

## No retry or replacement

One C03e-AI invocation performs at most one exact lower-transport accept and one logical authentication transaction.

It does not:

- retry acceptance;
- retry a failed proof;
- rotate transport identity;
- replace an active same-DeviceId worker;
- choose another DeviceId;
- reconnect;
- queue another candidate.

Any retry/reconnect policy remains separately gated.

## No collection insertion

The successful output is only `AuthenticatedRemoteSessionRuntimeOwner`.

C03e-AI does not construct:

- `RemoteSessionWorkerAdmission`;
- dispatcher;
- worker verifier-time provider;
- collection callback;
- mpsc sender/receiver;
- persistent worker entry;
- JoinHandle;
- cancellation pair.

A later checkpoint must select how a successfully authenticated owner becomes one C03e-AH admission without violating capacity/backpressure or current-thread runtime-driving constraints.

## Cancellation and supervisor shutdown are explicitly deferred

C03e-AI does not select cancellation of an in-flight pre-authentication transaction.

This is deliberate. Cancelling while a logical-session challenge is pending would require explicit proof of pending-session cleanup and accepted-peer cleanup ownership on every cancellation point.

A later combined listener/collection integration checkpoint must define orderly supervisor-shutdown behavior for:

- no peer accepted yet;
- peer accepted but challenge not prepared;
- challenge prepared and proof transaction pending;
- authenticated owner produced but not yet handed to collection admission.

Until that contract exists, C03e-AI remains one bounded transaction that is simply awaited to its existing terminal success/failure.

## Dependency boundary

C03e-AI selects no new crate, package version or Cargo feature.

The later source materialization is expected to compose only existing Agent/registry/session/remote-bridge seams.

These remain byte-stable in the selection checkpoint:

- `Cargo.lock`;
- `crates/prw-agent/Cargo.toml`;
- `crates/prw-remote-bridge/Cargo.toml`.

Tokio `rt-multi-thread` and `macros` remain unselected.

## Explicit non-selection

C03e-AI does not select or perform:

- source materialization of this transaction;
- blind/accept-any QUIC admission;
- a repeated listener/accept loop;
- persistent pre-auth task storage;
- parallel authentication attempts;
- worker collection insertion;
- a buffered real-network producer running ahead of C03e-AH capacity;
- nested `Runtime::block_on`;
- runtime handle exposure;
- in-flight authentication cancellation;
- retry/reconnect/replacement;
- process-signal wiring;
- Agent `main.rs` wiring;
- readiness publication;
- systemd/host mutation;
- deployment;
- merge.

## Exact selection scope

C03e-AI is docs-only.

Its final AH -> AI net diff must contain exactly this contract path and no source, manifest, lockfile, workflow, Android application, bridge, transport, Agent binary, packaging or host mutation.

## Validation requirements

Closure requires on the final exact AI head:

- exact AH merge base;
- exact one-path docs-only net scope;
- permanent PRW Rust validation FULL PASS: locked dependency graph, rustfmt, Clippy, workspace tests and workspace build;
- Android validation need not trigger for this docs-only exact head; no Android PASS may be claimed if absent;
- skipped workflows recorded only as skipped;
- immutable Drive audit with raw-readback byte verification;
- append-only rolling Drive update preserving the complete post-AH prefix byte-for-byte;
- PR remains draft/open/unmerged.

## Completion meaning

Closure of C03e-AI means only that the first bounded expected-device real remote admission transaction is architecturally selected:

- current registry resolves expected transport identity before accept;
- no authority guard crosses network I/O;
- exact lower-transport identity is verified by existing transport;
- current registry revalidates again before challenge preparation;
- existing authentication and binding seams retain their own cleanup semantics;
- a successful transaction yields one authenticated remote-session runtime owner;
- no collection insertion or listener loop exists yet.

The next checkpoint may materialize only this one-transaction seam and focused injected/source-level tests. Repeated real listener admission and C03e-AH collection integration remain separately gated.

Target gate:

`C03E_AI_EXPECTED_DEVICE_REAL_REMOTE_ADMISSION_TRANSACTION_SELECTED`
