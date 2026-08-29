# Phase 152 C03e-ET — Post-Authenticated Control-Stream Single-Read Family Demultiplexing Source Materialization (Staging)

Status: MATERIALIZED_SOURCE_STAGING

## 1. Purpose

C03e-ET source-materializes only the bridge-owned single-read ingress/family-custody boundary selected by C03e-ES.

It does not activate a combined Agent request loop and does not invoke requester/rendezvous authority, registry, policy, provider, candidate, response, dialing, deployment, restart, recovery, or merge behavior.

## 2. Exact predecessor

C03e-ET begins only from the durably closed C03e-ES final state:

- predecessor branch: `phase-152-c03e-es-post-auth-control-stream-single-read-family-demultiplexing-selection-staging`
- predecessor head: `34c63fafe9346494a41011241061684f2929daf3`
- predecessor tree: `c6295a4bb45548498f7cb7c1797bfdefd3253f68`
- predecessor gate: `C03E_ES_POST_AUTH_CONTROL_STREAM_SINGLE_READ_FAMILY_DEMULTIPLEXING_SELECTED`

No prior checkpoint is reopened.

## 3. Materialized source surface

C03e-ET adds one bridge module:

`crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`

The production bridge root exposes that module lexically only. No Agent source is modified.

The source introduces:

- `PostAuthCapabilityTransaction`
- `PostAuthControlStreamIngress`
- `PostAuthControlStreamIngressError`
- `receive_post_auth_control_stream_ingress(...)`

## 4. Exact one-read ingress

`receive_post_auth_control_stream_ingress(...)` consumes one already-accepted `MeshControlStream` by value.

Its bounded I/O sequence is exactly:

1. receive exactly one existing PRWM `ControlFrame`;
2. inspect only whether the payload starts with the exact four bytes `PRWZ`;
3. if yes, delegate exactly once to the existing strict requester/rendezvous PRWZ decoder;
4. otherwise preserve the already-read frame plus the same stream inside capability custody;
5. return one typed family result;
6. stop.

It does not perform a second read.

## 5. Legacy-preserving asymmetric family rule

The C03e-ES compatibility rule is materialized exactly:

- exact first four payload bytes `PRWZ` -> requester/rendezvous family;
- every other bounded PRWM frame -> existing capability family.

The capability fallback therefore still receives:

- valid `PRWC` requests;
- malformed `PRWC` requests;
- short payloads;
- unknown magic;
- non-PRWZ application payloads;
- wrong outer message kinds that the existing capability bridge must reject.

No generic unknown-family error is introduced before the legacy capability boundary.

## 6. Family recognition remains non-authoritative

A `PRWZ` prefix match proves only family selection.

It does not prove:

- `ControlMessageKind::Request`;
- supported PRWZ version;
- supported operation;
- zero flags;
- valid target length;
- valid UTF-8;
- valid logical `DeviceId`;
- absence of trailing bytes;
- requester authentication;
- requester authorization;
- target registration;
- target workspace relationship;
- provider eligibility;
- candidate availability;
- rendezvous success.

The existing `decode_requester_rendezvous_target_request_frame(...)` remains authoritative for strict PRWZ wire semantics.

## 7. Requester/rendezvous outcome

When exact `PRWZ` family recognition succeeds and the strict decoder passes, the ingress returns:

`PostAuthControlStreamIngress::RequesterRendezvous(RequesterRendezvousTargetWireRequest)`

The returned wire request preserves:

- outer PRWM `request_id` as correlation only;
- exact decoded logical target `DeviceId` as caller-nominated target intent only.

No requester identity is taken from the wire.

Requester identity remains owned by the authenticated PRW application-session context at the later Agent composition boundary.

## 8. Capability transaction custody

Every non-PRWZ frame returns:

`PostAuthControlStreamIngress::Capability(PostAuthCapabilityTransaction)`

The bridge-owned capability envelope retains exactly:

- the already-received `ControlFrame`;
- the exact same `MeshControlStream`.

It exposes only:

- immutable borrowing of the request frame through `request_frame()`;
- one consuming same-stream response operation through `send_response_frame(...)`.

No raw stream accessor is materialized.

## 9. Same-stream capability response preservation

The capability envelope delegates its response write to the existing:

`send_capability_response_frame(...)`

Therefore ET does not create a second response encoder or change existing capability response semantics.

The existing capability bridge remains authoritative for:

- outer request-kind validation;
- current lease validity;
- registry-revalidated principal;
- current transport binding;
- strict PRWC decode;
- exact capability policy;
- typed dispatch;
- response construction.

## 10. Stream ownership precision

The ingress function takes `MeshControlStream` by value.

After the caller transfers the stream into the bridge ingress, the caller cannot simultaneously retain that stream value while family selection occurs.

For capability traffic the bridge returns the stream only inside opaque capability transaction custody.

For requester/rendezvous traffic the stream is not returned from the decoded outcome. ET defines no requester/rendezvous response semantics.

This is source-level custody, not a claim of full runtime serialization or fairness.

## 11. Error classification

ET introduces only:

- `Runtime(MeshQuicRuntimeError)` for the single bounded PRWM receive;
- `RequesterRendezvousWire(RequesterRendezvousTargetWireError)` when exact `PRWZ` selection occurs but strict PRWZ decode fails.

A non-PRWZ frame is not decoded as requester/rendezvous and does not produce the new PRWZ error class.

Capability response I/O continues to use the existing `CapabilityRequestWireError` through the existing response adapter.

No new error-response wire protocol is materialized.

## 12. No Agent integration

C03e-ET intentionally does not modify:

- `AuthenticatedRemoteSessionRuntimeOwner`;
- `process_one_capability_request(...)`;
- `run_capability_request_loop(...)`;
- `run_capability_request_worker(...)`;
- the C03e-ER one-shot requester/rendezvous method;
- executor/admission/lifecycle source.

The new bridge ingress currently has no production Agent caller.

Therefore C03e-ET does not activate a combined request loop and does not create a second active acceptor.

## 13. No requester/rendezvous authority execution

C03e-ET stops before:

- C03e-DV invocation;
- requester registry lookup;
- requester-aware policy;
- provider lookup or mutation;
- candidate selection;
- candidate filtering;
- success response construction;
- semantic error response construction;
- response write;
- dialing.

The existing C03e-EO/EJ/ER Agent composition lineage is not invoked here.

## 14. Identity and correlation invariants

C03e-ET preserves:

- `DeviceId` / authenticated PRW session identity as logical identity;
- `TransportIdentity` as lower transport evidence only;
- endpoint/IP data as transient reachability information only;
- PRWM `request_id` as transaction correlation only.

No IP, port, PID, UID, GID, or request ID is promoted into logical identity or authorization.

## 15. No concurrency policy materialization

ET materializes only one-stream/one-read family custody.

Still separately gated:

- single authoritative peer accept-loop integration;
- serial versus concurrent execution after classification;
- fairness between capability and requester/rendezvous traffic;
- cancellation;
- backpressure;
- queue bounds;
- deadlines;
- drain/shutdown behavior;
- peer-close behavior.

## 16. No requester/rendezvous response semantics

ET does not define:

- requester success response format;
- requester error response format;
- candidate payload format;
- provider-error projection;
- timeout response behavior;
- replay/idempotency response behavior.

The requester/rendezvous stream is therefore not exposed for response writing by this checkpoint.

## 17. Existing source guards

At the ES predecessor, the relevant existing source blobs are:

- capability one-frame I/O adapter: `4a24af6316e2c17c0980c12e787791848174be9b`
- legacy capability bridge: `7b1c5c62339983da6ae2556f73510d7582ec0c5b`
- strict requester/rendezvous PRWZ codec: `2bfb2d6119a0bc3e1278fb361955093361949db1`
- requester-specific EQ receive adapter: `b86dfb8ebea963693eaa9a5107b91e919c21f9a6`
- bridge root: `d54e82cf7e511ff3d74cae6593de2e4bed48f676`

ET must not mutate the legacy capability bridge, strict PRWZ codec, EQ receive adapter, Cargo manifests, lockfile, Agent source, Android source, workflows, packaging, or deployment configuration.

## 18. Expected diff ceiling

The intended ET delta is limited to three paths:

1. this C03e-ET contract;
2. new `crates/prw-remote-bridge/src/post_auth_control_stream_ingress.rs`;
3. `crates/prw-remote-bridge/src/root.rs` only to declare the new module.

Any additional path requires explicit contradiction analysis before closure.

## 19. Validation requirements

Canonical closure requires exact-final-head Rust validation:

- exact toolchain recording;
- locked dependency graph;
- rustfmt;
- Clippy with `-D warnings`;
- workspace tests;
- workspace build.

Because bridge Rust source changes, Android validation must be evaluated from actual workflow triggering and may be claimed only if it runs successfully on the exact final ET head.

Path-filtered specialized workflows may be skipped and must not be reported as PASS when skipped.

Superseded candidate validation never counts as final closure evidence.

## 20. Durable evidence requirements

Closure requires:

1. immutable C03e-ET audit file in the existing Drive audit folder;
2. raw byte-exact audit readback;
3. append-only update of rolling `C02E_BRANCH_STATUS.md`;
4. exact preservation of the full post-ES predecessor prefix;
5. raw byte-exact rolling readback;
6. unique ET closure classification, gate, and audit-ID markers.

## 21. Preserved exclusions

C03e-ET explicitly excludes:

- Agent integration of the new ingress;
- active combined request loop;
- multiple concurrent stream acceptors;
- capability behavior redesign;
- C03e-DV invocation;
- requester registry/policy/provider execution;
- candidate selection;
- requester/rendezvous response protocol;
- replay/idempotency policy;
- queue/retry/reconnect behavior;
- peer-close semantic changes;
- direct Internet, relay, SSH, or traffic dialing;
- generic `BridgeCommand` redesign;
- new Agent transport dependency;
- arbitrary raw stream exposure;
- Agent bootstrap/main/listener activation;
- dependency upgrade;
- deployment;
- restart/recovery;
- merge.

## 22. Intended closure classification

On exact-head validation and durable evidence completion, C03e-ET may close only as:

`CLOSED_POST_AUTH_CONTROL_STREAM_SINGLE_READ_FAMILY_DEMULTIPLEXING_SOURCE_MATERIALIZATION`

with target gate:

`C03E_ET_POST_AUTH_CONTROL_STREAM_SINGLE_READ_FAMILY_DEMULTIPLEXING_SOURCE_MATERIALIZED`

A successor may select how the authenticated Agent runtime consumes exactly one bridge-owned family ingress result while preserving one authoritative accept owner. Runtime integration itself remains separately gated.
