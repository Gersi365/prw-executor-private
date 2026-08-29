# Phase 152 C03e-ER — Requester/Rendezvous Agent One-Shot Transaction Source Materialization (Staging)

Status: SOURCE_MATERIALIZED_PENDING_VALIDATION

## 1. Purpose

C03e-ER materializes only the isolated Agent-owned one-shot transaction boundary selected by
C03e-EP after C03e-EQ supplied the bridge-owned requester-specific PRWM/PRWZ receive adapter.

The transaction is deliberately not integrated into the existing capability request loop, worker,
listener lifecycle, admission loop, bootstrap, or product runtime. It is source-only staging behind
an uninvoked crate-private method.

## 2. Exact predecessor

C03e-ER starts only from the durably closed C03e-EQ final head:

- predecessor head: `0c69f51605a6543dc1be749be31304e78069f419`
- predecessor tree: `81b3e0e16400a2610252f5a512e2ecec6133480c`
- predecessor gate:
  `C03E_EQ_REQUESTER_RENDEZVOUS_PRWM_TARGET_REQUEST_BRIDGE_RECEIVE_ADAPTER_SOURCE_MATERIALIZED`

No earlier checkpoint is reopened.

## 3. Materialized boundary

The existing `AuthenticatedRemoteSessionRuntimeOwner` remains the only owner used for this
transaction because it already retains:

1. the exact lower-transport-authenticated peer; and
2. the exact Agent-owned authenticated application-session capability owner.

C03e-ER adds one crate-private asynchronous method:

`receive_requester_rendezvous_start_intent_once(&mut self)`

The method performs exactly this ordered composition:

1. accept exactly one new bounded control stream from the retained authenticated peer;
2. invoke the C03e-EQ bridge-owned
   `receive_requester_rendezvous_target_request(...)` exactly once;
3. copy the decoded outer PRWM `request_id` only into a separate correlation tuple element;
4. consume the exact decoded logical target `DeviceId` through the existing C03e-EO adaptation;
5. consume that target intent through the existing C03e-EJ authenticated-session adaptation;
6. return only `(request_id, RequesterRendezvousStartIntent)`.

The transaction stops there.

## 4. Correlation and identity invariants

The outer PRWM `request_id` remains transaction correlation only.

It is not:

- requester identity;
- authenticated-session identity;
- target identity;
- transport identity;
- registry identity;
- policy authority;
- provider identity;
- replay/idempotency authority.

Requester identity continues to come only from the exact retained authenticated application
session through the existing C03e-EJ lineage.

Target identity continues to come only from the caller-nominated logical `DeviceId` decoded by the
existing strict bridge PRWZ codec and transferred by value through C03e-EO.

Lower `TransportIdentity` remains transport evidence only and is not copied into the returned
requester/rendezvous intent.

## 5. Error ownership

C03e-ER introduces one Agent-owned typed transaction error with only two classes:

- `Accept(RemoteServerTransportRuntimeError)` — exact authenticated-peer control-stream acceptance
  failed;
- `Wire(RequesterRendezvousTargetRequestIoError)` — the existing C03e-EQ receive/decode boundary
  failed.

Neither class is translated into:

- a response frame;
- a requester/rendezvous authority result;
- a provider result;
- a fabricated success;
- a peer-close policy.

No retry, fallback decode, replacement stream, replacement session, or alternate target is selected.

## 6. C03e-EQ ownership remains unchanged

The bridge continues to own:

- `MeshControlStream` frame receive mechanics;
- the requester/rendezvous PRWM/PRWZ target-request wire codec;
- strict decode validation;
- bridge receive/decode error classification.

C03e-ER does not move PRWZ codec logic into the Agent and does not introduce a bridge-to-Agent
dependency.

## 7. C03e-EO and C03e-EJ reuse remains exact

C03e-ER introduces no alternate target constructor and no alternate requester constructor.

The decoded target flows through the already-materialized C03e-EO helper:

`adapt_decoded_requester_rendezvous_target_device_id(...)`

and then through the already-materialized C03e-EJ helper:

`adapt_post_auth_requester_rendezvous_target_intent(...)`.

No requester identity is accepted from the wire.

## 8. Authority boundary remains closed

C03e-ER does not invoke C03e-DV.

Therefore successful ER return does not prove or grant:

- requester authorization;
- current requester registration;
- current target registration;
- requester/target workspace relationship;
- requester-aware policy admission;
- provider registration;
- current reachability;
- transport eligibility;
- candidate availability;
- rendezvous success.

Those remain separately gated authority/execution semantics.

## 9. Stream-lifecycle constraint

The existing authenticated capability transaction and serial capability loop already accept control
streams from the same retained peer.

C03e-ER does not authorize a second active accept loop.

The new one-shot method remains uninvoked by runtime source. A future checkpoint must separately
select deterministic control-stream demultiplexing/dispatch ownership before any capability and
requester/rendezvous transaction can compete for streams.

No claim is made that stream-kind selection, protocol-family dispatch, fairness, concurrency,
backpressure, cancellation, or terminal peer-close behavior is solved by C03e-ER.

## 10. Response semantics remain unselected

C03e-ER writes no response frame and selects no requester/rendezvous response protocol.

Specifically absent:

- success response wire format;
- error response wire format;
- request-ID response correlation rules beyond preserving the inbound value locally;
- candidate payload encoding;
- semantic error projection;
- replay/idempotency handling;
- timeout response behavior;
- retry behavior.

## 11. Source scope ceiling

C03e-ER is limited to:

1. Agent root typing/error support for the one-shot transaction;
2. lexical child-module declaration under the existing authenticated owner;
3. the new isolated child source implementing the one-shot method;
4. this staging contract.

No bridge source is changed in C03e-ER.

No Cargo manifest, lockfile, Kotlin, Gradle, workflow, configuration, bootstrap, listener, systemd,
packaging, or deployment source is authorized.

## 12. Runtime non-activation

The new method is intentionally crate-private and uninvoked.

C03e-ER does not modify:

- `process_one_capability_request(...)`;
- `run_capability_request_loop(...)`;
- `run_capability_request_worker(...)`;
- executor worker collection;
- expected-device admission;
- endpoint lifecycle startup;
- process-lifecycle handoff;
- Agent `main`;
- readiness publication.

Therefore source materialization does not itself create a competing stream acceptor at runtime.

## 13. Validation gate

Closure requires exact-final-head canonical validation.

Required evidence:

- PRW Rust Validation full success: locked graph, rustfmt, Clippy with warnings denied, workspace
  tests, and workspace build;
- PRW Android Validation full success when triggered by this source delta: exact toolchains, native
  adapter, and Android application;
- specialized C02f-AD/C02f-AE workflows may be skipped according to their existing path filters and
  must not be claimed as PASS when skipped;
- no required exact-head workflow may remain pending or failing.

No closure evidence may be reused from a predecessor head.

## 14. Durable evidence gate

After canonical exact-head validation succeeds, closure requires:

1. one immutable C03e-ER audit file in the existing Drive audit folder;
2. raw byte-exact readback of that immutable audit;
3. append-only update of the rolling `C02E_BRANCH_STATUS.md` file;
4. exact preservation of the complete post-EQ predecessor prefix;
5. raw byte-exact rolling readback;
6. one occurrence each of the ER closure classification and target gate.

## 15. Preserved exclusions

C03e-ER explicitly excludes:

- capability-loop integration;
- worker-loop integration;
- concurrent stream acceptors;
- deterministic stream demultiplexing activation;
- C03e-DV invocation;
- requester registry/policy/provider execution;
- candidate selection;
- requester/rendezvous response construction or write;
- success/error response wire semantics;
- replay/idempotency policy;
- queue/retry/reconnect behavior;
- requester/rendezvous peer-close policy;
- direct Internet dialing;
- relay dialing;
- SSH dialing;
- traffic forwarding activation;
- generic `BridgeCommand` redesign;
- public API widening;
- bootstrap/main/listener activation;
- dependency upgrade;
- deployment;
- restart/recovery;
- merge.

## 16. Intended closure classification

On exact-head validation and durable evidence completion, C03e-ER may close only as:

`CLOSED_REQUESTER_RENDEZVOUS_AGENT_ONE_SHOT_TRANSACTION_SOURCE_MATERIALIZATION`

with target gate:

`C03E_ER_REQUESTER_RENDEZVOUS_AGENT_ONE_SHOT_TRANSACTION_SOURCE_MATERIALIZED`

A successor must begin with a fresh topology audit. It may select deterministic stream
family/demultiplexing ownership or another narrower prerequisite, but it does not inherit authority
to activate competing acceptors, invoke C03e-DV/provider/policy execution, define response semantics,
dial, deploy, or merge.
