# Phase 152 C03e-EP — Requester/Rendezvous PRWM Target-Request Agent Transaction Boundary Selection (Staging)

Status: SELECTED_DOCS_ONLY

## 1. Purpose

C03e-EP selects only the narrow ownership and composition boundary for a future one-shot requester/rendezvous PRWM target-request transaction after the C03e-EO decoded-target Agent adaptation source materialization.

This checkpoint does not materialize requester/rendezvous stream I/O, does not invoke the selected transaction, does not execute requester/rendezvous authority or provider state, and does not define or emit a success/error response protocol.

The selected boundary exists to preserve the already-established crate direction:

- `prw-remote-bridge` owns lower transport stream primitives and PRWZ wire semantics;
- `prw-agent` owns the authenticated application-session runtime owner and Agent-private requester/rendezvous intent types;
- no bridge -> Agent dependency is introduced.

## 2. Exact predecessor

C03e-EP is based exactly on the closed C03e-EO final head:

- branch: `phase-152-c03e-eo-requester-rendezvous-decoded-target-agent-target-intent-adaptation-source-materialization-staging`
- head: `4d4978688ae9eb6e2d80ba5efe414b1304aa2548`
- C03e-EO closure classification: `CLOSED_DECODED_TARGET_AGENT_TARGET_INTENT_ADAPTATION_SOURCE_MATERIALIZATION`
- C03e-EO target gate: `C03E_EO_REQUESTER_RENDEZVOUS_DECODED_TARGET_AGENT_TARGET_INTENT_ADAPTATION_SOURCE_MATERIALIZED`

No earlier closed checkpoint is reopened.

## 3. Verified current topology

### 3.1 Bridge-owned authenticated peer and control-stream acceptance

`crates/prw-remote-bridge/src/remote_server_transport_runtime.rs` owns:

- `AuthenticatedRemotePeerConnection`;
- exact validated lower `TransportIdentity` observation;
- `accept_control_stream()` returning one existing bounded `MeshControlStream`;
- explicit peer close;
- no logical-session authentication or requester/rendezvous authority.

The bridge-owned peer does not expose the raw Quinn connection.

### 3.2 Existing Agent owner already retains the peer

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs` owns `AuthenticatedRemoteSessionRuntimeOwner` containing:

- one `AuthenticatedRemotePeerConnection`; and
- one `RemoteSessionCapabilityRuntimeOwner` retaining the same authenticated application-session binding.

This is the narrowest existing owner that simultaneously has:

1. custody of the exact authenticated peer from which a future requester/rendezvous control stream may be accepted; and
2. custody of the exact authenticated application session from which requester identity must be derived.

Therefore C03e-EP selects `AuthenticatedRemoteSessionRuntimeOwner` as the future Agent-side transaction owner.

### 3.3 Existing capability I/O proves the crate direction

The existing capability path already demonstrates the intended layering:

1. Agent-owned `AuthenticatedRemoteSessionRuntimeOwner` accepts one control stream from its retained bridge peer;
2. a bridge-owned one-frame adapter receives one bounded PRWM frame;
3. Agent performs higher-level application composition without owning lower transport implementation.

C03e-EP preserves this direction for requester/rendezvous instead of exposing or importing lower transport implementation into Agent code.

### 3.4 Existing PRWZ target-request codec remains pure bridge ownership

`crates/prw-remote-bridge/src/requester_rendezvous_target_request_wire.rs` owns the pure PRWZ v1.0 target-request codec and `RequesterRendezvousTargetWireRequest`.

The decoded value contains only:

- outer PRWM `request_id` as correlation; and
- one typed logical target `DeviceId`.

It contains no requester/session/transport identity and confers no authority.

### 3.5 Existing Agent adaptation remains crate-private

`crates/prw-agent/src/remote_session_capability_runtime.rs` owns the C03e-EO helper:

`adapt_decoded_requester_rendezvous_target_device_id(DeviceId) -> RequesterRendezvousTargetIntent`

and the C03e-EJ caller seam:

`adapt_post_auth_requester_rendezvous_target_intent(&AuthenticatedRemoteSessionRuntimeOwner, RequesterRendezvousTargetIntent) -> RequesterRendezvousStartIntent`.

These seams preserve the identity split:

- requester identity comes only from the retained authenticated application session;
- target identity comes only from the caller-nominated decoded logical `DeviceId`.

## 4. Critical stream-ownership constraint

The same `AuthenticatedRemoteSessionRuntimeOwner` already owns the existing capability request transaction and loop. That capability path accepts newly opened control streams from the retained peer.

Therefore C03e-EP explicitly rejects any design that introduces an independently running requester/rendezvous accept loop or second concurrent acceptor on the same peer.

Two concurrent semantic acceptors would create an unselected race in which either path could consume the next control stream before application semantics are known.

C03e-EP consequently selects only a one-shot, non-invoked requester/rendezvous transaction seam for later source materialization. It does not select loop integration, concurrent acceptance, stream demultiplexing, scheduling, or runtime activation.

Any later activation that combines capability and requester/rendezvous traffic on the same peer must separately select deterministic stream transaction ownership/demultiplexing before execution.

## 5. Selected future transaction shape

A later source-materialization checkpoint may add exactly one isolated Agent-owned one-shot requester/rendezvous target-request transaction with the following semantic order:

1. borrow `&mut AuthenticatedRemoteSessionRuntimeOwner` so stream acceptance remains serialized by the exact owner;
2. accept exactly one new control stream from the retained `AuthenticatedRemotePeerConnection`;
3. delegate exactly one bounded PRWM frame receive plus strict requester/rendezvous PRWZ decode to a bridge-owned requester-specific receive adapter;
4. retain the decoded outer PRWM `request_id` as transaction correlation;
5. consume the decoded target `DeviceId` by value;
6. adapt that target exactly once through the existing C03e-EO helper into `RequesterRendezvousTargetIntent`;
7. adapt that target intent exactly once through the existing C03e-EJ authenticated-session caller seam into `RequesterRendezvousStartIntent`;
8. return a crate-private Agent transaction value that keeps `request_id` and `RequesterRendezvousStartIntent` as separate fields.

The transaction stops there.

It must not invoke C03e-DV, requester policy, registry validation, provider mutation, candidate selection, dialing, or response emission.

## 6. Selected bridge I/O boundary

C03e-EP selects a requester-specific bridge-owned one-frame receive adapter rather than rebranding or semantically widening the existing capability adapter.

The later bridge receive adapter must:

- own any reference to `MeshControlStream` and lower transport I/O errors;
- perform exactly one bounded frame receive;
- delegate strict semantic decode to the existing `decode_requester_rendezvous_target_request_frame` codec;
- return only `RequesterRendezvousTargetWireRequest` on success;
- preserve transport-read failure separately from PRWZ decode failure;
- perform no retry, loop, response write, peer close, identity lookup, registry access, policy evaluation, provider mutation, or networking beyond the single already-selected stream read.

The existing pure codec semantics remain unchanged.

A later materialization should prefer a separate requester-specific I/O module so the existing pure codec module remains conceptually pure and its tests remain side-effect-free.

## 7. Selected Agent transaction value

The future Agent-side one-shot composition should preserve correlation without contaminating intent identity.

Conceptually, the crate-private result contains:

- `request_id: u64`; and
- `start_intent: RequesterRendezvousStartIntent`.

The exact Rust type name remains a source-materialization detail, but these invariants are mandatory:

- `request_id` is not copied into `RequesterRendezvousTargetIntent`;
- `request_id` is not copied into `RequesterRendezvousStartIntent`;
- requester identity is not copied into bridge wire types;
- transport identity is not copied into the logical target intent;
- target nomination grants no requester authorization or target eligibility.

## 8. Error selection

The later one-shot transaction must fail closed and preserve stage ownership.

C03e-EP selects distinct error classes for:

1. control-stream acceptance failure from the existing bridge runtime; and
2. requester/rendezvous one-frame receive/decode failure from the future bridge requester-specific adapter.

No requester/rendezvous authority/provider error is part of this transaction because authority/provider execution remains excluded.

No wire error is translated into a fabricated semantic response.

No retry, replacement stream, fallback decode, secondary codec attempt, peer close, or response write is selected by C03e-EP.

## 9. Response and correlation boundary

C03e-EP preserves the outer PRWM `request_id` only so a later separately selected response checkpoint can correlate a terminal requester/rendezvous result.

C03e-EP does not select:

- success response payload semantics;
- error response payload semantics;
- response wire magic/version/operation;
- candidate list/result encoding;
- replay protection;
- idempotency semantics;
- request-ID allocation for locally originated requests;
- duplicate-request handling;
- timeout-generated response behavior.

A later response checkpoint must consume the preserved correlation value explicitly rather than reconstructing it from requester or target identity.

## 10. Capability-loop non-interference

The existing capability transaction/loop remains byte-for-byte and behaviorally authoritative.

C03e-EP does not authorize modifying:

- `process_one_capability_request`;
- `run_capability_request_loop`;
- capability request wire semantics;
- capability response semantics;
- capability dispatcher behavior;
- capability current-registry/policy evaluation.

A future requester/rendezvous one-shot seam remains uninvoked and must not run concurrently with the existing capability loop unless a separate deterministic demultiplexing/worker-ownership checkpoint is first selected and materialized.

## 11. Crate/dependency invariants

The following dependency direction is mandatory:

- Agent may depend on public bridge APIs already permitted by the current workspace;
- bridge must not depend on Agent;
- no new circular dependency;
- no new dependency solely to move requester/session identity into the bridge;
- no public exposure of Agent-private `RequesterRendezvousTargetIntent` or `RequesterRendezvousStartIntent` from the bridge.

C03e-EP selects no manifest or lockfile changes.

## 12. Identity and authority invariants

### Requester

Requester identity is only the `AuthenticatedDeviceSession` already retained by the exact `AuthenticatedRemoteSessionRuntimeOwner` path.

A PRWZ payload cannot nominate or replace requester identity.

### Target

Target identity is only the decoded logical `DeviceId` from the PRWZ request payload.

Target nomination is intent only.

### Transport

`TransportIdentity` remains lower-transport authentication evidence and is not interchangeable with either requester logical identity or target logical identity.

### Correlation

Outer PRWM `request_id` is transaction correlation only.

### Authority

Successful decode/adaptation does not prove:

- requester authorization;
- current requester registration;
- current target registration;
- requester/target workspace relationship;
- requester policy approval;
- provider registration;
- target reachability;
- transport eligibility;
- candidate freshness/currentness;
- rendezvous success.

## 13. Exact future source boundary selected

A successor source-materialization checkpoint may touch only the minimum source needed to implement the selected isolated seam, expected to be:

1. one new requester-specific bridge one-frame receive adapter module;
2. `crates/prw-remote-bridge/src/root.rs` only to expose that bridge-owned adapter within the existing crate API surface; and
3. `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs` for the Agent-owned one-shot transaction composition.

If exact-head source inspection at the successor shows that fewer paths are sufficient, the successor must use fewer paths.

Any need for a broader path set, manifest change, public API widening, worker-loop modification, or runtime invocation is a new selection problem rather than implicit authorization from C03e-EP.

## 14. Validation contract for C03e-EP

Because C03e-EP is documentation-only, closure requires:

- exact branch based on the C03e-EO final head;
- exactly one changed contract path;
- no source/configuration/manifest/lock drift;
- canonical Rust validation terminal success for the exact final head;
- Android validation claimed only if it actually triggers and reaches terminal success;
- immutable Drive audit readback;
- append-only rolling status update with predecessor prefix preserved.

## 15. Preserved exclusions

C03e-EP does not authorize or materialize:

- requester/rendezvous stream acceptance or read/write source;
- invocation of PRWZ decode from I/O;
- invocation of C03e-EO/EJ from I/O;
- C03e-DV invocation;
- requester-aware registry/policy execution;
- requester/rendezvous provider execution or mutation;
- candidate selection or candidate response construction;
- success/error response protocol;
- request-ID replay/idempotency policy;
- concurrent control-stream acceptors;
- capability/requester stream demultiplexing activation;
- worker-loop modification or integration;
- queue/retry/reconnect behavior;
- peer-close semantics for requester/rendezvous errors;
- direct Internet, relay, SSH or traffic dialing;
- generic `BridgeCommand` redesign;
- Agent bootstrap/main wiring;
- listener/runtime activation;
- dependency upgrade;
- deployment, restart/recovery or merge.

## 16. Closure classification and successor gate

If exact-head validation and durable evidence succeed, C03e-EP closes as:

`CLOSED_REQUESTER_RENDEZVOUS_PRWM_TARGET_REQUEST_AGENT_TRANSACTION_BOUNDARY_SELECTION`

with target gate:

`C03E_EP_REQUESTER_RENDEZVOUS_PRWM_TARGET_REQUEST_AGENT_TRANSACTION_BOUNDARY_SELECTED`

Only after that durable closure may a separately tracked successor consider source-materializing the isolated one-shot bridge receive + Agent composition seam selected here.

That successor still must not activate the seam from the capability loop, worker lifecycle, bootstrap, or production runtime, and must not execute requester/rendezvous authority/provider logic or emit a response unless those boundaries are separately selected first.
