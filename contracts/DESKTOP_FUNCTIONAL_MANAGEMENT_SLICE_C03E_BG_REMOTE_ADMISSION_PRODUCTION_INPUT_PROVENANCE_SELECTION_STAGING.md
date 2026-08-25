# Phase 152 C03e-BG — Remote Admission Production Input Provenance Selection

Status: STAGED SELECTION

Gate target:
`C03E_BG_REMOTE_ADMISSION_PRODUCTION_INPUT_PROVENANCE_SELECTED`

## 1. Exact predecessor

Closed C03e-BF:
- branch: `phase-152-c03e-bf-remote-endpoint-production-bind-address-source-materialization-staging`;
- head: `ef215fc329f4def306ab4449e6732418319b9178`;
- tree: `97c9b9447513a8fc889c5a4fee18dfb32ee960c4`;
- gate: `C03E_BF_REMOTE_ENDPOINT_PRODUCTION_BIND_ADDRESS_SOURCE_MATERIALIZED`.

BF materialized only the fixed non-secret `PRW_REMOTE_BIND_ADDR` source and its bounded parser/validator. It did not wire `main.rs`, produce expected-device requests, construct a production dispatcher, populate registry/policy authority, select timing/correlation sources, publish readiness, deploy, or merge.

## 2. Purpose

C03e-AZ already materialized one library-owned remote-process operation that consumes fully typed injected inputs through `LinuxAgentRemoteProcessOperationInputs`.

After BF, the bind-address input now has an explicit process source. The remaining remote-admission inputs are still deliberately injected and unresolved.

BG selects only the production-input provenance ordering and non-fabrication constraints that are already required by the current source graph.

BG does **not** select a rendezvous/discovery protocol, control-plane provider, database, registry persistence mechanism, policy service, session-ID generator, request-ID allocator, concrete provider backend, system clock API wrapper, readiness policy, executable activation, or deployment mechanism.

This is a dependency/readiness selection, not an architecture redesign.

## 3. Existing composition sink remains authoritative

The current `LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>` shape remains unchanged and authoritative. It requires:

1. `bind_addr: SocketAddr`;
2. `max_active_workers: NonZeroUsize`;
3. `capability_authority: SharedCurrentCapabilityAuthority<P>`;
4. `session_authentication: SessionAuthenticationService`;
5. `expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`;
6. admission timing provider;
7. completion callback;
8. rejection callback;
9. admission-failure callback.

BG does not add, remove, reorder, or widen these source-level inputs.

## 4. Expected-device request is a pre-auth scheduling object only

The existing `RemoteSessionExpectedDeviceAdmissionRequest<D, T>` owns:

- expected logical `DeviceId`;
- `SessionId`;
- authentication PRWM correlation request identifier (`u64`);
- dispatcher `D`;
- verifier-time provider `T`.

The expected `DeviceId` is pre-authentication scheduling intent only.

It is not authenticated identity evidence, capability authority, currentness evidence, reachability evidence, or public-routability evidence.

The post-authentication worker continues to derive its logical `DeviceId` from the authenticated session owner.

## 5. TransportIdentity remains registry-derived

BG explicitly preserves the existing AJ ordering:

1. expected logical `DeviceId` is supplied to the AJ transaction;
2. one fresh shared-current registry read resolves that device's current `TransportIdentity`;
3. current device lifecycle and transport binding are validated;
4. only that exact current `TransportIdentity` is passed to lower transport acceptance.

No expected-request producer may supply an independent `TransportIdentity`.

No IP address, `SocketAddr`, `ConnectivityEndpoint`, candidate, thread/task/runtime/channel identifier, PID, UID, GID, or request identifier may substitute for `DeviceId` or `TransportIdentity`.

## 6. Pre-handshake expected-device provenance remains separately gated

The current lower remote transport exposes only exact-expected-identity acceptance: `accept_authenticated_peer(expected TransportIdentity)`.

It does not expose an accept-any-authenticated-peer operation that could safely discover a logical device after handshake.

Therefore a production expected-device producer must have authoritative provenance for the intended logical `DeviceId` **before** the AJ lower-transport accept begins.

BG does not choose where that pre-handshake scheduling intent comes from.

Specifically, BG does not select:
- control-plane rendezvous;
- discovery advertisement consumption;
- durable queue/provider semantics;
- client-initiated local IPC scheduling;
- registry enumeration as an implicit listener strategy;
- polling every registered device;
- transport-identity guessing;
- IP-based device inference;
- candidate-based identity inference.

Any such source requires a separately reviewed selection consistent with existing Phase 152 and production mutation gates.

## 7. SessionId production remains separately gated

`prw_core::SessionId` currently validates only that the supplied string is non-empty/non-whitespace. It does not allocate, randomize, persist, sequence, or guarantee uniqueness.

The Phase 128 `SessionAuthenticationService` rejects duplicate pending/authenticated `SessionId` values, but it does not generate identifiers.

Therefore production `SessionId` custody/production remains a separate input-source responsibility.

BG does not select UUIDs, randomness, monotonic counters, provider-issued IDs, persistence, restart behavior, reuse windows, or cross-process uniqueness semantics.

`SessionId` remains authentication correlation only and never replaces logical `DeviceId`.

## 8. Authentication request-id production remains separately gated

The existing AJ/authentication transaction consumes a `u64` PRWM correlation request identifier and enforces exact response correlation.

No source-level allocator or production custody policy is selected by the current remote process composition.

BG therefore keeps authentication request-id production separate from expected-device provenance and separate from `SessionId` production.

The expected-device producer must not invent a request-id policy implicitly merely because the request object stores one.

## 9. Dispatcher construction precedes a usable expected-request producer

Each `RemoteSessionExpectedDeviceAdmissionRequest<D, T>` owns its dispatcher before AJ starts and later transfers it unchanged into the authenticated worker admission.

Consequently a production expected-request producer cannot be considered complete unless the dispatcher construction/custody boundary is separately selected and materialized.

BG does not select a concrete `CapabilityDispatcher` implementation.

The existing Agent local-management provider machinery is not silently promoted into a remote production dispatcher:
- local management authority is crate-internal and tied to its reviewed local admission model;
- the local provider lifecycle explicitly owns no production terminal/forwarding backend implementation;
- the local management dispatcher contract itself records that production provider wiring is absent.

Any remote adapter must preserve the existing Phase 143 authorization result and existing provider principal/authority invariants without bypassing current registry/policy checks.

## 10. Current registry/policy authority may not be fabricated

`SharedCurrentCapabilityAuthority<P>` requires a concrete `WorkspaceDeviceRegistry` and policy evaluator.

The current registry implementation is bounded in-memory state. `WorkspaceDeviceRegistry::new()` creates an empty registry; that is not production registry population.

BG explicitly rejects treating an empty/default registry as a successful production authority source.

Likewise a convenient fail-open or synthetic policy must not be introduced merely to satisfy the generic type.

Production registry population/watch/persistence and policy load/mutation remain separately gated.

No database/provider architecture is selected by BG.

## 11. SessionAuthenticationService construction is not session provenance

`SessionAuthenticationService::new()` may construct the existing in-memory authentication transaction owner without network or provider side effects.

That constructor does not solve:
- expected-device provenance;
- `SessionId` production;
- authentication request-id production;
- current registry population;
- policy source;
- dispatcher construction;
- timing source.

BG therefore does not conflate service construction with a complete production session-authentication input assembly.

## 12. Verifier timing remains verifier-owned

The repeated admission supervisor retains the existing separation between:
- per-request verifier-time provider used by the capability request loop; and
- fresh AJ admission timing sampled only when one admission attempt actually starts.

BG selects no wall-clock wrapper, time synchronization provider, skew policy, persistence, retry deadline, reconnect timer, or lease refresh policy.

Timing values must remain verifier-owned and must not be accepted from an unauthenticated remote peer as authority.

## 13. Worker capacity and callbacks remain independent configuration/observation inputs

`max_active_workers` and the completion/rejection/admission-failure callbacks do not establish identity, authentication, authorization, reachability, readiness, or provider authority.

BG does not use worker capacity as a substitute for expected-device discovery policy.

BG does not select logging, metrics, retry, replacement, process-exit, or readiness consequences for callback events.

Those remain separately gated.

## 14. Dependency ordering selected by BG

For a future production-quality remote-operation input assembly, the following responsibilities must remain explicit and non-fabricated:

1. authoritative current registry + policy source capable of producing `SharedCurrentCapabilityAuthority`;
2. concrete typed capability-provider/dispatcher custody suitable for remote authorized requests;
3. production `SessionId` custody/production;
4. production authentication request-id custody/production;
5. verifier-owned request/admission timing sources;
6. authoritative pre-handshake expected-`DeviceId` scheduling provenance;
7. bounded worker-capacity configuration and observation callbacks;
8. only then composition into `RemoteSessionExpectedDeviceAdmissionRequest` values and `LinuxAgentRemoteProcessOperationInputs`.

This list is a dependency/provenance ordering. It does not select concrete providers for items 1–7.

An implementation may stage independently reviewable prerequisites in a different textual order only if it preserves all dependencies and does not fabricate missing authority.

## 15. Why expected-device producer is not materialized in BG

A producer that emits only `DeviceId` is insufficient for the current request type because each request also owns `SessionId`, request-id, dispatcher, and verifier-time provider.

A producer that fabricates those additional values would silently select several still-unreviewed production policies at once.

A producer that accepts an independent `TransportIdentity` would violate the already-closed AI/AJ identity ordering.

Therefore BG intentionally stops at the provenance/dependency selection and does not create a queue producer or source adapter.

## 16. Why main.rs activation remains blocked

Even with BF's bind-address source, `main.rs` still lacks reviewed production sources for the remaining remote-operation inputs.

Calling the remote process operation from `main.rs` now would require one or more of:
- default/empty authority fabrication;
- synthetic expected-device requests;
- arbitrary session/request identifiers;
- test/disposable dispatcher substitution;
- invented timing/callback semantics.

BG explicitly forbids those shortcuts.

## 17. Existing provider and authorization boundaries remain authoritative

Phase 143 authorization remains the only remote capability admission path:
- valid PRWM request framing;
- valid remote application-session lease;
- fresh current registry revalidation;
- exact current transport binding;
- typed PRWC command decode;
- exact capability policy allow;
- only then dispatcher invocation.

BG does not weaken or duplicate this chain.

Phase 152 typed provider code remains reusable only through separately reviewed adapters that preserve its authority and lifecycle constraints.

## 18. Explicit non-selections

BG does not select or materialize:
- a discovery/rendezvous protocol or provider;
- a concrete expected-device producer;
- a concrete remote `CapabilityDispatcher`;
- terminal/file/transfer/forwarding production backend construction;
- filesystem root selection;
- registry persistence, database schema, watch protocol, or control-plane sync;
- policy persistence/load/mutation;
- `SessionId` generator/custody;
- authentication request-id allocator/custody;
- timing provider or clock policy;
- worker-capacity source;
- callback logging/metrics/process policy;
- candidate ID/path-kind/priority/candidate construction/publication;
- STUN/ICE/TURN/relay activation;
- Agent `main.rs` remote-lane wiring;
- remote readiness or local-readiness changes;
- remote failure -> process-exit policy;
- retry/reconnect/rebind/rebootstrap/replacement;
- systemd/packaging/host/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart;
- recovery/PRWF/R1-R4 activation;
- merge.

## 19. Identity and security invariants

- `DeviceId` / authenticated PRW session identity remains logical identity.
- `TransportIdentity` remains lower-transport certificate identity only.
- `SocketAddr` / `ConnectivityEndpoint` remains transient endpoint/configuration state only.
- `SessionId` remains authentication correlation only.
- PRWM request-id remains message correlation only.
- expected `DeviceId` remains pre-auth scheduling intent until authentication succeeds.
- runtime/task/thread/controller/channel/lock/candidate/endpoint identifiers are never PRW identity.
- successful configuration, bind, handshake, request correlation, or queue delivery is not capability authorization.
- protected operations continue to require fresh-current registry/transport/policy evaluation.

## 20. Exact intended BF -> BG scope

BG is docs-only.

The exact branch must differ from closed BF only by:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BG_REMOTE_ADMISSION_PRODUCTION_INPUT_PROVENANCE_SELECTION_STAGING.md`

Any source, Cargo, lockfile, workflow, `main.rs`, packaging/systemd, provider, registry, auth, or networking path change blocks closure.

## 21. Validation and closure condition

BG can close only after:
- exact BF predecessor lineage remains unchanged;
- exact BF -> BG diff is one docs-only path;
- canonical Rust validation on the exact final BG head reaches terminal success;
- any automatically triggered workflow reaches a terminal non-failing verdict before closure;
- immutable Drive audit is uploaded and raw-readback verified;
- rolling Drive evidence passes a fresh predecessor guard, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No production source materialization is authorized merely by BG closure.

Gate target remains:
`C03E_BG_REMOTE_ADMISSION_PRODUCTION_INPUT_PROVENANCE_SELECTED`
