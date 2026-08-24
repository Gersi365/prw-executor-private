# Phase 152 C03e-O — Single Capability Request Transaction Source Materialization Staging

Status: STAGED

Target gate:

`C03E_O_SINGLE_CAPABILITY_REQUEST_TRANSACTION_SOURCE_MATERIALIZED`

## Predecessor

Canonical predecessor is closed C03e-N:

- branch: `phase-152-c03e-n-capability-wire-adapter-source-materialization-staging`
- head: `2e09010a6134576f95ee0d4ba043d207140420a1`
- tree: `c2685eddbd29f7a995a669d305d29974ca5b40fe`
- gate: `C03E_N_CAPABILITY_WIRE_ADAPTER_SOURCE_MATERIALIZED`

C03e-O must preserve exact N lineage. It does not merge, rebase or cherry-pick any sibling or superseded checkpoint.

## Purpose

Materialize only the C03e-M-selected Agent-owned transaction for exactly one capability request on exactly one newly accepted bounded control stream of one already-authenticated remote application session.

The transaction is intentionally smaller than a session loop. It exposes no autonomous worker, no task collection and no concurrent stream admission policy.

## Ownership boundary

The transaction belongs to existing `AuthenticatedRemoteSessionRuntimeOwner`, which already retains by value:

- the same live `AuthenticatedRemotePeerConnection` established and logically authenticated before this checkpoint;
- the existing `RemoteSessionCapabilityRuntimeOwner` carrying one already-created `BoundRemoteSession`.

C03e-O does not construct a new peer, logical session, application lease or binding.

The method receives `&mut self` so one caller cannot use this API to run two transactions concurrently through the same owner. Concurrent/multi-stream session operation remains separately gated.

## Exact transaction sequence

One `process_one_capability_request(...)` call performs exactly:

1. accept one peer-initiated bounded control stream through the retained authenticated peer;
2. receive one complete bounded PRWM frame through C03e-N `receive_capability_request_frame(...)`;
3. call the retained `BoundRemoteSession::process_request(...)` exactly once with:
   - current caller-supplied `CapabilityBridge`;
   - explicit caller-supplied verifier time;
   - the received frame;
   - caller-supplied mutable `CapabilityDispatcher`;
4. only on bridge success, send exactly the returned response frame through C03e-N `send_capability_response_frame(...)` on that same stream;
5. return success only after response transmission succeeds.

There is no second request read and no next-stream acceptance inside one call.

## Dynamic authority preservation

C03e-O introduces no new authorization decision and caches no per-request authority.

Existing `BoundRemoteSession::process_request(...)` continues to supply its retained transport identity and verifier-owned session lease internally and delegates to current `CapabilityBridge` authority for every request.

Therefore every successful operation still requires current:

- lease validity at explicit verifier time;
- authenticated-session registry validity;
- logical-device to retained `TransportIdentity` binding validity;
- PRWC request decoding and exact required capability derivation;
- selected policy evaluation;
- dispatcher execution;
- bounded success-response construction with request correlation.

`DeviceId` / authenticated PRW session identity remains the logical identity. `TransportIdentity` remains transport identity only. No IP, PID, UID or GID is promoted into PRW identity.

## Wire layering

Lower `MeshControlStream`, raw PRWM frame I/O and `MeshQuicRuntimeError` remain behind `prw-remote-bridge` through the C03e-N adapter.

`prw-agent/Cargo.toml` must remain byte-stable; Agent must not add a direct `prw-remote-transport` dependency for this checkpoint.

C03e-O does not duplicate request-kind validation, PRWC decoding or response-frame construction.

## Failure taxonomy

The transaction exposes a narrow typed Agent error preserving exactly three existing classes:

- `RemoteServerTransportRuntimeError` for stream acceptance;
- `CapabilityRequestWireError` for one-frame receive/send;
- `RemoteBridgeError` for current bound-session authorization/dispatch/response construction.

Each remains available as the error source; no string-only flattening is allowed.

## Fail-closed behavior

Any failure stops the one transaction immediately.

On failure C03e-O must not:

- fabricate or send a successful response;
- retry the request;
- accept a replacement stream inside the same call;
- replace the authenticated session or application lease;
- invoke pending-session abort after authentication already succeeded;
- delete the authenticated session;
- silently widen registry or policy authority;
- automatically close the whole peer.

Whole-peer close/continue policy after a request failure remains a later session-loop/lifecycle decision.

## No concurrency or loop materialization

C03e-O does not materialize:

- a `loop` over capability requests;
- multiple requests on one stream;
- multiple accepted streams per transaction;
- concurrent stream tasks;
- a session worker/task collection;
- cancellation, draining or join semantics;
- retry/reconnect;
- request scheduling or fairness policy.

The mutable owner borrow is the only serialization property selected here.

## Source scope

Expected source scope is deliberately narrow:

1. `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`
2. this contract

No other source path is required by the selected design.

In particular, no mutation is selected for:

- `crates/prw-agent/Cargo.toml` or `Cargo.lock`;
- `crates/prw-agent/src/lib.rs` or `main.rs`;
- C03e-H session-auth transaction source;
- C03e-N bridge wire adapter source;
- remote binding/lease implementation;
- registry, policy or dispatcher implementations;
- workflows;
- Android application source;
- remote readiness;
- systemd/packaging;
- host-network/reachability activation.

## Validation requirements

Because C03e-O changes Rust source, canonical completion requires exact-head:

- PRW Rust Validation FULL PASS: locked dependency graph, rustfmt, Clippy, workspace tests and workspace build;
- PRW Android Validation FULL PASS when triggered by the source change, including exact toolchains, native adapter and Android application;
- skipped C02f-AD/C02f-AE workflows must not be counted as PASS evidence.

Any corrective formatting/lint commit must remain within the exact selected two-path scope and must be recorded in final audit evidence.

## Drive closeout requirements

After exact-head canonical CI passes:

1. publish immutable `C03E_O_SINGLE_CAPABILITY_REQUEST_TRANSACTION_SOURCE_MATERIALIZATION_AUDIT.md` in the existing evidence folder;
2. raw-readback verify its byte size and SHA-256;
3. immediately re-fetch authoritative rolling `C02E_BRANCH_STATUS.md` and require the exact post-N baseline before mutation;
4. append O evidence only, preserving the entire predecessor prefix byte-for-byte;
5. raw-readback verify final rolling size/hash and predecessor prefix hash;
6. update the O PR body to CLOSED checkpoint metadata while keeping the PR draft/open/unmerged.

## Deliberate stopping point

Even after C03e-O closes, the following remain separately gated:

- real authenticated-session request loop;
- repeated/multi-stream operation policy;
- concurrent session/request task ownership;
- peer-close/continue policy after per-request failures;
- cancellation/drain/join lifecycle;
- Agent `main.rs` runtime wiring;
- remote readiness publication;
- listener/reachability runtime activation;
- external NAT/ICE/STUN/TURN/relay integration;
- credential provisioning;
- deployment/restart/merge.

Gate on successful canonical closeout:

`C03E_O_SINGLE_CAPABILITY_REQUEST_TRANSACTION_SOURCE_MATERIALIZED`
