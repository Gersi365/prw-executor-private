# Phase 152 C03e-AJ — Expected-Device Real Remote Admission Transaction Source Materialization Staging

Status: STAGED

Target gate:

`C03E_AJ_EXPECTED_DEVICE_REAL_REMOTE_ADMISSION_TRANSACTION_SOURCE_MATERIALIZED`

## Predecessor

Canonical predecessor is closed C03e-AI:

- branch: `phase-152-c03e-ai-expected-device-real-remote-admission-transaction-selection-staging`
- head: `889e5131250d29521f4f9af8d85b96cd605601f1`
- tree: `cb87d44b0041f0a91ba0dd5a9c970cf9da765a35`
- gate: `C03E_AI_EXPECTED_DEVICE_REAL_REMOTE_ADMISSION_TRANSACTION_SELECTED`

C03e-AJ materializes only the one bounded transaction selected by C03e-AI.

## Materialized source shape

Add one child module under `remote_session_capability_runtime` that exposes one bounded async Agent transaction equivalent in responsibility to:

`admit_expected_remote_device_session(...)`

The function accepts:

- `&AgentRemoteTransportRuntime`;
- `&SharedCurrentCapabilityAuthority<P>`;
- `&mut SessionAuthenticationService`;
- expected logical `&DeviceId`;
- owned typed `SessionId`;
- challenge-validity `Range<u64>`;
- authentication PRWM request ID `u64`;
- authentication verifier time `u64`;
- application-lease `Range<u64>`.

It returns:

`Result<AuthenticatedRemoteSessionRuntimeOwner, RemoteSessionRealAdmissionError>`.

No dispatcher, worker verifier-time factory, cancellation pair, JoinHandle, collection sender/receiver, runtime handle or caller-supplied TransportIdentity is accepted.

## Phase 1 — fresh expected transport resolution

Use existing internal `SharedCurrentCapabilityAuthority::with_current_authority(...)` without widening its visibility.

Within the first read:

1. `registry.device(expected_device_id)`;
2. `RegisteredDevice::transport_identity()`;
3. `registry.validate_transport_identity(expected_device_id, identity)`;
4. return the owned/copy `TransportIdentity`.

Map failure to `RemoteSessionRealAdmissionError::Registry` using existing `RegistryError` values.

The authority read guard must be released before awaiting peer acceptance.

No policy evaluation, registry clone or policy clone is introduced.

## Phase 2 — exact lower-transport acceptance

Call existing:

`AgentRemoteTransportRuntime::accept_authenticated_peer(expected_transport_identity).await`

exactly once.

Map failure to `RemoteSessionRealAdmissionError::Accept`.

Do not add an accept-any transport API, retry, wildcard identity or IP-based selection.

## Phase 3 — second fresh registry check and challenge preparation

After a peer is accepted, take another independent fresh shared-current authority read and delegate to existing:

`AgentRemoteTransportRuntime::begin_registry_bound_session_challenge(...)`.

Pass:

- the same accepted peer;
- fresh current registry;
- the same expected logical DeviceId;
- the owned typed SessionId;
- challenge-validity interval;
- the same mutable `SessionAuthenticationService`.

The existing seam must revalidate the accepted peer's TransportIdentity against current registry state.

The second authority guard must be released before authentication wire I/O.

## Post-accept challenge-preparation failure cleanup

If challenge preparation fails after peer acceptance, close the accepted peer exactly once with:

- code `5`;
- reason `remote session admission preparation failed`.

Then return `RemoteSessionRealAdmissionError::Challenge(existing_error)`.

Do not call `abort_pending_session` for a challenge-preparation failure.

## Phase 4 — existing authentication transaction

After challenge preparation succeeds, call existing:

`complete_registry_bound_session_authentication(...)`

exactly once with the accepted peer, mutable session service, prepared challenge, request ID and verifier time.

Map failure to `RemoteSessionRealAdmissionError::Authentication` without additional cleanup.

Existing authentication transaction remains sole owner of:

- pending-session abort;
- code-1 peer close;
- preservation of primary + cleanup errors.

AJ must not double-abort or double-close.

## Phase 5 — existing post-auth binding

On authentication success, call existing:

`compose_authenticated_remote_session(...)`

with the same accepted peer, authenticated session and application-lease interval.

Map existing `RemoteBridgeError` to `RemoteSessionRealAdmissionError::Binding` without additional peer close.

Existing composition remains sole owner of code-2 binding failure close.

Success returns one `AuthenticatedRemoteSessionRuntimeOwner`.

## Bounded error surface

Materialize:

`RemoteSessionRealAdmissionError`

with exactly these semantic variants:

- `Registry(RegistryError)`;
- `Accept(AgentRemotePeerAcceptError)`;
- `Challenge(AgentRemoteSessionChallengeError)`;
- `Authentication(AgentRemoteSessionAuthenticationFailure)`;
- `Binding(RemoteBridgeError)`.

Implement bounded `Display`, `Error::source`, and `From` mappings where appropriate.

Do not expose raw Quinn/Tokio identifiers or flatten authentication cleanup evidence.

## Focused source tests

Tests must remain source-level and non-networking. Prove at least:

- transaction function has the selected typed input/output surface;
- each existing bounded error maps to the correct AJ error variant;
- code 5 is nonzero and reason is nonempty/fixed;
- no caller-supplied TransportIdentity appears in the transaction signature.

Do not add a fake accept-any transport or production listener test.

## Current-thread and collection boundary

AJ does not call `Runtime::block_on`, spawn a task, create an mpsc channel or insert into C03e-AH collection.

The function is an ordinary async transaction that a later combined current-thread supervisor may await.

Repeated real listener admission, capacity-aware producer integration and shutdown/cancellation of partial authentication remain separately gated.

## Authority and identity invariants

- DeviceId remains logical identity.
- TransportIdentity remains transport-only identity derived from current registry state.
- IP remains transient endpoint data.
- SessionId remains session correlation identity, not collection/device authority.
- successful lower-transport acceptance is not logical authentication;
- successful logical authentication is not capability authorization;
- every later protected capability request continues fresh current registry/transport/policy evaluation.

No authority guard crosses network accept, challenge/proof I/O, binding, task lifecycle or collection lifecycle.

## Dependency boundary

No dependency, package version or feature change is expected.

These must remain absent from the final AI -> AJ diff:

- `Cargo.lock`;
- `crates/prw-agent/Cargo.toml`;
- `crates/prw-remote-bridge/Cargo.toml`;
- permanent workflows;
- Android application source;
- Agent `main.rs`;
- readiness/host/package paths.

## Exact intended final scope

The final AI -> AJ net diff should contain exactly three paths:

1. this AJ contract;
2. `crates/prw-agent/src/remote_session_capability_runtime.rs` for child-module declaration/re-export only;
3. `crates/prw-agent/src/remote_session_capability_runtime/real_remote_admission_transaction.rs`.

No other final path is authorized without a concrete source contradiction.

## Validation requirements

Closure requires on the exact final AJ head:

- exact AI merge base;
- exact three-path final net scope;
- permanent PRW Rust validation FULL PASS;
- canonical Android native/application validation FULL PASS because Rust source changes are present;
- skipped workflows recorded only as skipped;
- immutable Drive audit with raw-readback byte verification;
- append-only rolling Drive update preserving the complete post-AI prefix byte-for-byte;
- PR remains draft/open/unmerged.

## Explicit non-selection

C03e-AJ does not materialize:

- repeated listener/accept loop;
- blind accept-any;
- worker collection insertion;
- dispatcher or worker admission construction;
- partial-auth cancellation/shutdown policy;
- retry/reconnect/replacement;
- process-signal wiring;
- Agent `main.rs`;
- readiness;
- systemd/host mutation;
- deployment;
- merge.

## Completion meaning

Closure means only that the C03e-AI-selected one expected-device real remote admission transaction exists in source and is canonically validated.

The next checkpoint must separately select repeated real listener admission + C03e-AH collection integration under the same actively driven current-thread runtime before any production listener loop is materialized.

Target gate:

`C03E_AJ_EXPECTED_DEVICE_REAL_REMOTE_ADMISSION_TRANSACTION_SOURCE_MATERIALIZED`
