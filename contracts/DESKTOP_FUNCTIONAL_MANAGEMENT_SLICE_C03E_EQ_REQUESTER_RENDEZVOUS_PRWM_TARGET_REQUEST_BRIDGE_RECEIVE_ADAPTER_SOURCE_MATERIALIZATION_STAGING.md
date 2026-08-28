# Phase 152 C03e-EQ — Requester/Rendezvous PRWM Target-Request Bridge Receive Adapter Source Materialization (Staging)

Status: SOURCE_MATERIALIZED_PENDING_VALIDATION

## 1. Purpose

C03e-EQ materializes only the bridge-owned requester-specific one-frame receive adapter selected by C03e-EP.

It intentionally stops before Agent-side transaction composition. This narrower split keeps the existing capability runtime byte-stable and lets the lower I/O boundary be validated independently before a later Agent-owned one-shot composition checkpoint.

## 2. Exact predecessor

C03e-EQ begins exactly from the closed C03e-EP final head:

- predecessor head: `e870032798dae2fdb096a2b6d666bda8617176ac`
- predecessor tree: `7f426d593a8801517b2ba4b0c8e4a1d2fcce90e5`
- predecessor classification: `CLOSED_REQUESTER_RENDEZVOUS_PRWM_TARGET_REQUEST_AGENT_TRANSACTION_BOUNDARY_SELECTION`
- predecessor gate: `C03E_EP_REQUESTER_RENDEZVOUS_PRWM_TARGET_REQUEST_AGENT_TRANSACTION_BOUNDARY_SELECTED`

No earlier closed checkpoint is reopened.

## 3. Materialized source

C03e-EQ adds:

`crates/prw-remote-bridge/src/requester_rendezvous_target_request_io.rs`

The adapter exposes exactly one operation:

`receive_requester_rendezvous_target_request(&mut MeshControlStream)`

The operation:

1. receives exactly one bounded PRWM `ControlFrame` through the existing `MeshControlStream::receive_frame()` primitive;
2. delegates exactly once to the already-materialized pure `decode_requester_rendezvous_target_request_frame` codec;
3. returns only the existing `RequesterRendezvousTargetWireRequest` on success.

The adapter does not interpret requester identity, target authority, registry state, requester policy, provider state, reachability or rendezvous eligibility.

## 4. Error ownership

C03e-EQ adds `RequesterRendezvousTargetRequestIoError` with exactly two stage-preserving classifications:

- `Runtime(MeshQuicRuntimeError)` for the existing bounded stream/frame receive failure;
- `Wire(RequesterRendezvousTargetWireError)` for strict PRWM/PRWZ semantic decode failure.

The adapter does not translate either failure into a semantic response and performs no fallback, retry, second decode or peer close.

## 5. Crate boundary

Lower control-stream ownership remains in `prw-remote-bridge`.

PRWZ target-request semantics remain in the existing pure bridge codec.

The bridge root adds only:

`pub mod requester_rendezvous_target_request_io;`

No bridge -> Agent dependency is introduced.

No Agent type is named or exposed by the new bridge module.

## 6. Identity and correlation invariants

The returned existing wire request preserves:

- outer PRWM `request_id` as correlation only;
- one typed logical target `DeviceId`.

It carries no requester/session identity and no transport identity.

Successful receive/decode proves only bounded transport/frame receipt and strict wire structure. It does not prove requester authorization, target registration, workspace relationship, provider registration, reachability, transport eligibility or rendezvous success.

## 7. Capability-loop non-interference

C03e-EQ does not modify any Agent source.

In particular it does not modify or invoke:

- `AuthenticatedRemoteSessionRuntimeOwner::process_one_capability_request`;
- `AuthenticatedRemoteSessionRuntimeOwner::run_capability_request_loop`;
- the remote-session worker seam;
- any control-stream accept loop.

The new receive adapter is not invoked from production/runtime source in this checkpoint.

This preserves the C03e-EP constraint that a second concurrent semantic stream acceptor must not be activated on the same peer.

## 8. Exact changed-path ceiling

C03e-EQ is expected to contain exactly these three paths:

1. this contract;
2. `crates/prw-remote-bridge/src/requester_rendezvous_target_request_io.rs`;
3. `crates/prw-remote-bridge/src/root.rs`.

No Cargo manifest, lockfile, Agent source, Kotlin, Gradle, workflow or configuration path is authorized.

## 9. Validation requirements

Closure requires canonical exact-final-head validation:

- locked dependency graph;
- rustfmt;
- Clippy with repository-required warning discipline;
- workspace tests;
- workspace build.

Android PASS is claimed only if the canonical Android workflow actually triggers and reaches terminal success.

Any source corrective must remain inside the exact authorized C03e-EQ source paths and must be revalidated at the corrected exact head.

## 10. Preserved exclusions

C03e-EQ does not authorize or materialize:

- accepting a new control stream from `AuthenticatedRemotePeerConnection`;
- Agent-side requester/rendezvous transaction composition;
- C03e-EO or C03e-EJ invocation from I/O;
- C03e-DV invocation;
- requester registry/policy/provider execution;
- candidate selection or response construction;
- success/error response protocol;
- replay/idempotency semantics;
- concurrent stream acceptors;
- capability/requester stream demultiplexing;
- capability-loop or worker-loop modification;
- retry/reconnect/queue behavior;
- requester/rendezvous peer-close semantics;
- direct Internet, relay, SSH or traffic dialing;
- generic `BridgeCommand` redesign;
- public Agent API widening;
- bootstrap/main/listener activation;
- dependency upgrade;
- deployment, restart/recovery or merge.

## 11. Closure classification

If exact-head validation and durable evidence succeed, C03e-EQ closes as:

`CLOSED_REQUESTER_RENDEZVOUS_PRWM_TARGET_REQUEST_BRIDGE_RECEIVE_ADAPTER_SOURCE_MATERIALIZATION`

with target gate:

`C03E_EQ_REQUESTER_RENDEZVOUS_PRWM_TARGET_REQUEST_BRIDGE_RECEIVE_ADAPTER_SOURCE_MATERIALIZED`

## 12. Successor gate

Only after durable C03e-EQ closure may a separately tracked successor consider the Agent-owned isolated one-shot composition selected by C03e-EP:

1. borrow the existing authenticated remote-session owner;
2. accept exactly one new control stream;
3. call this requester-specific bridge receive adapter exactly once;
4. preserve outer `request_id` separately;
5. transfer decoded target `DeviceId` through C03e-EO and C03e-EJ;
6. return separate correlation plus `RequesterRendezvousStartIntent`;
7. stop before C03e-DV/provider/policy execution or response emission.

That successor must remain uninvoked from the existing capability loop/worker/runtime unless deterministic stream demultiplexing and lifecycle ownership are separately selected first.
