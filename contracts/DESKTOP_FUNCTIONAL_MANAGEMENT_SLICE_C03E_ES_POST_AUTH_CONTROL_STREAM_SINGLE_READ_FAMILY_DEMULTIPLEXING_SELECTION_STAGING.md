# Phase 152 C03e-ES — Post-Authenticated Control-Stream Single-Read Family Demultiplexing Selection (Staging)

Status: SELECTED_DOCS_ONLY

## 1. Purpose

C03e-ES selects only the ownership and compatibility boundary required to remove the control-stream
accept/read race exposed by the coexistence of the existing authenticated capability path and the
C03e-ER requester/rendezvous one-shot path.

This checkpoint does not materialize demultiplexing source and does not activate requester/rendezvous
runtime behavior.

## 2. Exact predecessor

C03e-ES begins only from the durably closed C03e-ER final state:

- predecessor head: `65334b6a4b2fc7e5bf4cedfaf65a858105cc9c10`
- predecessor tree: `88ef2418f75ca3caba51f112dc760e18ff39758b`
- predecessor gate:
  `C03E_ER_REQUESTER_RENDEZVOUS_AGENT_ONE_SHOT_TRANSACTION_SOURCE_MATERIALIZED`

No prior checkpoint is reopened.

## 3. Exact topology finding

The current Agent has two source-level operations capable of accepting a new control stream from the
same retained `AuthenticatedRemotePeerConnection`:

1. the existing capability transaction, which accepts one stream and then calls the bridge-owned
   `receive_capability_request_frame(...)`; and
2. the C03e-ER requester/rendezvous one-shot method, which accepts one stream and then calls the
   C03e-EQ `receive_requester_rendezvous_target_request(...)` adapter.

The C03e-ER method is currently uninvoked, so no runtime race exists today. Activating both as
independent loops would create an ownership race because either acceptor could receive the next
peer-initiated control stream.

Therefore a future activation must have exactly one stream-accept/read custody boundary before the
request family is selected.

## 4. Wire-family topology

Both current operation families use existing bounded PRWM `ControlFrame` transport.

The capability family uses the existing Phase 143 `PRWC` payload magic:

`BRIDGE_MAGIC = *b"PRWC"`

The requester/rendezvous target-request family uses the C03e-EK/EM `PRWZ` payload magic:

`REQUESTER_RENDEZVOUS_TARGET_REQUEST_WIRE_MAGIC = *b"PRWZ"`

Both semantic decoders independently remain authoritative for complete validation after family
selection. Family recognition is not authorization and is not full protocol validation.

## 5. Selected compatibility rule

C03e-ES selects a legacy-preserving routing rule:

- an exact first four payload bytes of `PRWZ` selects the requester/rendezvous target-request family;
- every other bounded PRWM frame remains on the existing capability-family fallback path.

This rule is deliberately asymmetric.

It preserves the existing capability error boundary for:

- valid `PRWC` requests;
- malformed `PRWC` payloads;
- short payloads;
- unknown payload magic;
- wrong application metadata that the existing `CapabilityBridge` already rejects.

C03e-ES does not introduce a new generic "unknown family" semantic error ahead of the existing
capability bridge. Only the already-reserved exact `PRWZ` prefix diverts from legacy capability
processing.

## 6. Classification is not semantic validation

The future family classifier may inspect only the exact bounded frame payload prefix needed for the
routing decision.

It must not claim that `PRWZ` prefix recognition proves:

- `ControlMessageKind::Request`;
- supported PRWZ major/minor version;
- supported operation;
- zero flags;
- valid target length;
- valid UTF-8;
- valid `DeviceId`;
- absence of trailing bytes;
- requester authorization;
- target eligibility;
- rendezvous success.

The existing strict `decode_requester_rendezvous_target_request_frame(...)` remains authoritative for
all PRWZ semantic checks.

Likewise non-PRWZ routing does not prove a valid capability request. The existing
`CapabilityBridge::authorize(...)` remains authoritative for outer request kind, authenticated lease,
current registry/transport binding, PRWC decode, capability policy, and later dispatch admission.

## 7. Selected ownership split

### Agent ownership

`AuthenticatedRemoteSessionRuntimeOwner` remains the orchestration owner because it already retains
the exact authenticated peer and exact authenticated application-session capability context.

A future Agent transaction may request exactly one post-authenticated control-stream ingress from the
bridge and branch on the bridge-owned family result.

Agent does not gain direct `prw-remote-transport` dependency or raw transport protocol ownership.

### Bridge ownership

The bridge remains owner of:

- `MeshControlStream`;
- bounded PRWM frame receive/write mechanics;
- `ControlFrame` wire representation;
- `PRWC` and `PRWZ` magic constants;
- strict PRWZ decode;
- capability response send mechanics.

The future single-read ingress should therefore be bridge-owned and should hide lower transport
custody from Agent source as much as practical.

## 8. Selected single-read ingress shape

C03e-ES selects a future bridge-owned ingress transaction that receives exactly one already-accepted
post-authenticated control stream **by value**, reads exactly one bounded PRWM frame, and then returns
one of two bridge-owned typed outcomes:

1. requester/rendezvous target request — after exact `PRWZ` prefix recognition and existing strict
   PRWZ decode; or
2. capability transaction custody — preserving both the already-received raw `ControlFrame` and the
   same stream required for the eventual existing capability response write.

The exact Rust type names remain a source-materialization choice, but the ownership invariants above
are selected.

The capability outcome must retain same-stream response custody because the current capability path
writes its success response on the same control stream after authorization and dispatch.

The requester/rendezvous outcome does not select a response write; its stream may be dropped after
strict request decode unless a later response-semantics checkpoint selects otherwise.

## 9. Why a bridge-owned capability transaction envelope is selected

The Agent crate intentionally has no direct `prw-remote-transport` dependency.

A bridge-owned opaque capability transaction envelope can retain:

- the already-received `ControlFrame`; and
- the exact same `MeshControlStream`.

It may expose only the narrow operations required by the existing capability path, for example:

- borrow the request frame for existing `BoundRemoteSession` / `CapabilityBridge` authorization;
- consume or mutably use the retained stream only to send the already-constructed existing response.

C03e-ES does not authorize a broader raw stream API.

## 10. Capability behavior preservation

A future implementation must preserve the current capability semantics:

1. current registry/transport binding and lease checks remain inside the existing capability bridge;
2. PRWC decode remains inside the existing capability bridge;
3. capability policy remains inside the existing capability bridge;
4. dispatch remains through the existing authorized request dispatch path;
5. only bridge success produces the existing same-stream response;
6. existing typed capability errors remain unchanged unless a separately selected checkpoint
   explicitly changes them.

Family demultiplexing must not reinterpret malformed non-PRWZ frames as requester/rendezvous
requests.

## 11. Requester/rendezvous behavior preservation

A future implementation may route exact `PRWZ` frames through the existing strict requester codec,
then through the already-materialized Agent composition lineage:

- C03e-EO decoded target → target-intent adaptation;
- C03e-EJ authenticated-session target-intent → start-intent adaptation.

The resulting requester/rendezvous transaction still stops before C03e-DV.

No requester registry, requester-aware policy, provider execution, candidate selection, response
construction, or dialing is selected here.

## 12. Request correlation remains separate

For both families the outer PRWM `request_id` remains transaction correlation.

C03e-ES does not convert request correlation into:

- requester identity;
- authenticated-session identity;
- target identity;
- transport identity;
- authorization;
- replay/idempotency authority.

Requester/rendezvous correlation remains separate from `RequesterRendezvousStartIntent` exactly as
closed by C03e-ER.

## 13. No active demultiplexer in ES

C03e-ES is documentation-only selection.

It does not:

- change `process_one_capability_request(...)`;
- change `run_capability_request_loop(...)`;
- change `run_capability_request_worker(...)`;
- invoke the C03e-ER one-shot method;
- add a combined request loop;
- accept a runtime stream;
- read a runtime frame;
- classify a runtime frame;
- write a response;
- close a peer.

## 14. No concurrency policy selected

Single-read ownership removes the ambiguous double-consumer topology, but C03e-ES does not select a
full concurrency policy.

Still separately gated:

- serial versus concurrent request execution;
- fairness between capability and requester/rendezvous families;
- cancellation behavior;
- backpressure;
- per-family queueing;
- request deadlines;
- peer-close rules;
- drain/shutdown interaction.

A source-materialization successor may build only the selected isolated ingress/classification
boundary unless additional behavior is separately selected.

## 15. No response semantics selected

C03e-ES does not define requester/rendezvous response semantics.

Specifically absent:

- success response wire format;
- error response wire format;
- candidate payload response;
- semantic error projection;
- timeout response behavior;
- replay/idempotency response behavior.

The existing capability response path is preserved but not redesigned.

## 16. Source/dependency guards

At the C03e-ER predecessor:

- capability one-frame bridge adapter blob:
  `4a24af6316e2c17c0980c12e787791848174be9b`
- legacy capability bridge implementation blob:
  `7b1c5c62339983da6ae2556f73510d7582ec0c5b`
- strict requester/rendezvous PRWZ codec blob:
  `2bfb2d6119a0bc3e1278fb361955093361949db1`
- C03e-EQ requester/rendezvous receive adapter blob:
  `b86dfb8ebea963693eaa9a5107b91e919c21f9a6`
- ER Agent authenticated runtime root blob:
  `083bf83fd1827f6175c9eb62ff93b40147fa9271`
- ER requester/rendezvous one-shot child blob:
  `438dc88761bf2eb7424a71c0c503bfdc95c9cde3`
- Agent Cargo blob:
  `18ed32b080cac9b4540b33f870388499d7e5bc52`

C03e-ES itself must change no Rust, Cargo, lockfile, Kotlin, Gradle, workflow, configuration, or runtime
source.

## 17. Validation gate

Because ES is docs-only, canonical closure requires exact-final-head Rust validation when triggered:

- locked dependency graph;
- rustfmt;
- Clippy with warnings denied;
- workspace tests;
- workspace build.

Android validation is not required to trigger for a docs-only delta and no Android PASS may be claimed
unless it actually runs successfully on the exact final head.

Path-filtered specialized workflows may be skipped and must not be described as PASS when skipped.

## 18. Durable evidence gate

Closure requires:

1. one immutable C03e-ES audit file in the existing Drive audit folder;
2. raw byte-exact audit readback;
3. append-only update of rolling `C02E_BRANCH_STATUS.md`;
4. exact preservation of the full post-ER predecessor prefix;
5. raw byte-exact rolling readback;
6. unique ES closure classification and target-gate markers.

## 19. Preserved exclusions

C03e-ES explicitly excludes:

- demultiplexing source materialization in ES itself;
- active combined request loop;
- multiple concurrent stream acceptors;
- capability execution redesign;
- C03e-DV invocation;
- requester registry/policy/provider execution;
- candidate selection;
- requester/rendezvous response protocol;
- replay/idempotency policy;
- queue/retry/reconnect behavior;
- peer-close semantics changes;
- direct Internet, relay, SSH, or traffic dialing;
- generic `BridgeCommand` redesign;
- new Agent transport dependency;
- public API widening beyond a separately reviewed bridge-owned ingress surface;
- Agent bootstrap/main/listener activation;
- dependency upgrade;
- deployment;
- restart/recovery;
- merge.

## 20. Intended closure classification

On exact-head validation and durable evidence completion, C03e-ES may close only as:

`CLOSED_POST_AUTH_CONTROL_STREAM_SINGLE_READ_FAMILY_DEMULTIPLEXING_SELECTION`

with target gate:

`C03E_ES_POST_AUTH_CONTROL_STREAM_SINGLE_READ_FAMILY_DEMULTIPLEXING_SELECTED`

A successor may source-materialize only the selected bridge-owned single-read ingress/family custody
boundary. Activation of a combined Agent request loop, requester/rendezvous authority execution,
response semantics, concurrency policy, dialing, deployment, and merge remain separately gated.
