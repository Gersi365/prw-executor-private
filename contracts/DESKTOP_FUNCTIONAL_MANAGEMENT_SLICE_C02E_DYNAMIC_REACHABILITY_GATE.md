# Phase 152 C02e — Dynamic Reachability Gate

Status: `DESIGN_LOCK / IDENTITY_STABLE_ENDPOINTS_TRANSIENT / CANDIDATE_ID_NON_REBINDABLE / AUTHENTICATED_CANDIDATE_PUBLICATION_PROVENANCE_STAGED / SESSION_WORKSPACE_REGISTRY_REFRESH_ORDER_LOCKED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Predecessor: `phase-152-c02d-provider-backend-design`

Predecessor head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

C02e locks the product rule that a PRW device is identified by authenticated PRW identity, not by the current IP address through which the device happens to be reachable.

This is required for normal mobile operation where one enrolled phone can move between home Wi-Fi, 5G, another Wi-Fi network, NAT/CGNAT domains, or relay-assisted connectivity without becoming a different PRW device merely because its current endpoint changed.

C02e remains provider-neutral and Sans-I/O. It does not open sockets, gather candidates, perform STUN/ICE, activate TURN/relay, change routes/firewall/DNS, wire the Agent runtime, or select a production endpoint.

## Existing architecture reused

C02e does not introduce a second reachability model.

It reuses the existing `prw-connectivity` Phase 135 types:

- `PeerConnectivityIdentity`
- `DeviceId`
- `TransportIdentity`
- `ConnectivityCandidate`
- `ConnectivityEndpoint`
- `ConnectivityPathKind`
- `ReachabilityObservation`
- `PeerConnectivityPlan`

It reuses `AuthenticatedDeviceSession` and `WorkspaceDeviceRegistry` as the existing authenticated-session and current device/transport authority boundaries.

It also preserves the existing Phase 139 transport architecture:

`DeviceId -> current TransportIdentity -> candidate exchange -> selected explicit IP/UDP candidate -> authenticated QUIC/TLS transport`

The deterministic preference remains:

`LocalDirect -> InternetDirect -> Relay -> Offline`

Phase 139 already assigns candidate exchange and coordination to the separate control plane. Phase 141 remains the Sans-I/O ICE/STUN layer. Successful ICE selection produces reachability evidence correlated to an existing candidate; it does not become a PRW identity or authorization authority.

## Identity rule

A connectivity endpoint is not a PRW identity.

The following are explicitly separate:

1. logical enrolled-device identity (`DeviceId`);
2. current transport identity (`TransportIdentity`);
3. transient reachability candidates (`ConnectivityEndpoint` values).

Changing a candidate IP address or port does not change the logical device identity.

A transport-key/certificate rotation may legitimately produce a new `TransportIdentity`; that lifecycle remains governed by the registry/transport identity contract and is not modeled as a mere endpoint refresh.

## Candidate refresh rule

`PeerConnectivityPlan` may replace its transient candidate set while preserving its existing `PeerConnectivityIdentity`.

The refresh operation must:

- validate the complete proposed candidate set before mutating the plan;
- preserve the existing logical/transport peer identity;
- enforce the existing maximum of 16 candidates;
- reject duplicate candidate IDs;
- reject duplicate exact `(path kind, endpoint)` candidates;
- treat each `CandidateId` as stable for the lifetime of the plan;
- permit an existing `CandidateId` to survive a refresh only when it still denotes the exact same path kind and endpoint;
- require a fresh `CandidateId` when path kind or endpoint changes;
- reject any attempt to rebind an existing `CandidateId` to another candidate before mutation;
- replace the prior candidate set atomically on success;
- discard stale candidate observations;
- initialize every refreshed candidate observation to `Unknown`;
- make removed candidate IDs immediately unknown to subsequent observation updates;
- leave the complete prior candidate/observation state unchanged if validation fails.

Resetting observations is required because reachability evidence for an old network path must not be silently transferred to a newly signaled endpoint set.

The non-rebinding rule additionally prevents a late Phase 141 reachability update correlated only by an older `CandidateId` from being applied to a different newly signaled endpoint that reused that same identifier.

## Authenticated requester and workspace admission rule

A later runtime integration must not apply a target candidate refresh merely because a candidate vector is syntactically valid.

The required admission order is locked as:

1. revalidate the requester's `AuthenticatedDeviceSession` through `WorkspaceDeviceRegistry::validate_authenticated_session(...)`;
2. require the candidate publication to identify the exact target `PeerConnectivityIdentity` already held by the plan;
3. revalidate the authenticated publisher/target session against current registry state;
4. require requester and publisher/target to belong to the same current `WorkspaceId`;
5. revalidate the publication/plan's exact target `TransportIdentity` through `WorkspaceDeviceRegistry::validate_transport_identity(...)`;
6. only after all current identity/workspace/provenance checks pass may `PeerConnectivityPlan::refresh_candidates(...)` validate and mutate transient endpoints.

Any failure before step 6 must leave the complete plan state unchanged.

This ordering closes these stale-authority cases before endpoint mutation:

- requester membership suspension/removal;
- requester device revocation or authenticated-session/registry mismatch;
- target publisher membership suspension/removal;
- target publisher device revocation or authenticated-session/registry mismatch;
- cross-workspace target access;
- a publication created by one authenticated device being retargeted to another peer plan;
- target transport identity rotation leaving the publication/plan identity stale.

C02e stages this order in `prw-remote-bridge` integration-test source because that crate already depends on session, registry and connectivity domains. No Cargo edge or runtime wiring is added.

## Authenticated candidate publication provenance boundary

Registry-current admission alone does not authenticate arbitrary candidate bytes. C02e therefore stages a source-only publication boundary in `crates/prw-remote-bridge/tests/authenticated_candidate_provenance.rs`.

The staged publication semantics are:

1. a publisher must already hold an `AuthenticatedDeviceSession`;
2. that publisher session is revalidated against the current registry before publication;
3. the publisher's presented `TransportIdentity` is revalidated for that authenticated publisher device;
4. the publication target identity is derived from the authenticated publisher's own `DeviceId` plus that current `TransportIdentity`; the caller does not supply an arbitrary target `DeviceId`;
5. the complete candidate vector is validated using the existing `PeerConnectivityPlan` candidate bounds before a publication can exist;
6. consumption revalidates both requester and publisher/target sessions, requires the same workspace, requires exact publication-to-plan peer identity equality, and revalidates the exact target transport identity before endpoint mutation.

This means an authenticated requester cannot publish candidates under its own identity and then inject them into another target's plan merely by presenting a syntactically valid candidate vector.

The staged type exists only in integration-test source. It is a design/provenance specification, not a production control-plane object and not a new discovery authority.

## Phase 141 correlation and refresh boundary

Phase 141's `IceConnectivitySession` owns one bounded set of remote candidate correlations and its selected-pair update carries the correlated Phase 135 `CandidateId`.

C02e therefore requires candidate identity stability across refresh:

- an endpoint/path change must receive a new candidate ID;
- a removed old candidate ID is rejected by the refreshed plan;
- an old ID cannot be rebound to a new endpoint;
- observations are reset on every successful refresh.

This prevents old ICE correlation from becoming reachability evidence for a different endpoint merely because an identifier was reused.

C02e does not claim that this solves general signaling replay/freshness. A late update for the exact same unchanged candidate still requires the later authenticated coordination adapter to preserve appropriate freshness semantics.

## Wire and control-transport boundary

C02e deliberately does not invent a candidate-update wire encoding.

The existing Phase 129 control transport already provides a bounded generic frame envelope and Phase 139 assigns candidate exchange to the authenticated control-plane coordination path, but the repository does not yet contain a reviewed candidate-update application payload schema or a production adapter that binds such a payload to an authenticated PRW session.

Phase 128 provides server-owned, single-use freshness for session authentication, but Phase 129 generic control-frame `request_id` is only specified as non-zero and is not a candidate-publication replay authority. C02e therefore must not infer replay protection from the transport envelope alone.

Therefore C02e locks the semantic requirement without activating transport:

`authenticated publisher session -> current publisher DeviceId + TransportIdentity -> bounded validated candidate publication -> current same-workspace requester admission -> transactional target plan refresh`

A later separately reviewed adapter may serialize/deserialize this semantic object only if it preserves the same identity derivation, current-registry checks, bounds, freshness/replay semantics and fail-closed ordering. Raw unauthenticated endpoint injection remains forbidden.

## Registry/discovery relationship

The higher-level product flow is:

`authenticated PRW target/publisher session`

`-> current registry target principal + current TransportIdentity`

`-> bounded authenticated candidate publication`

`-> authenticated PRW requester session`

`-> current registry requester principal`

`-> same-workspace + exact target identity revalidation`

`-> transactional transient candidate refresh`

`-> provider reachability observations`

`-> deterministic path selection`

`-> authenticated transport establishment`

The production source/transport of candidate publications remains outside C02e. Existing control-plane transport, NAT traversal and relay components retain their respective transport/protocol responsibilities.

## Forwarding relationship

The C02d `ExactForwardingEgressPolicy` remains a low-level exact IP+TCP-port primitive for explicitly fixed service targets.

It must not be interpreted as the primary device-to-device identity mechanism.

Device-to-device PRW connectivity should resolve the current network endpoint from authenticated device/transport state. A mobile client changing IP addresses therefore does not require editing a static identity allowlist.

This C02e gate does not widen C02d forwarding to DNS names, CIDRs, wildcards, arbitrary destinations, or runtime-selected untrusted targets.

## Security invariants

C02e must not:

- derive PRW identity from IP address;
- persist a candidate IP as a substitute for `DeviceId`;
- rebind an existing plan-scoped `CandidateId` to a different path or endpoint;
- let a candidate publisher name an arbitrary target device instead of deriving identity from the authenticated publisher;
- accept a publication from a publisher session that is no longer registry-current;
- consume a publication when the requester session is no longer registry-current;
- apply a cross-workspace target refresh;
- apply a publication whose exact peer identity differs from the target plan;
- apply a candidate refresh after current-registry target device revocation;
- apply a candidate refresh after the target transport identity becomes stale due to rotation;
- carry reachability observations across candidate refresh;
- accept more than 16 candidates;
- preserve removed candidates as still selectable;
- treat generic TLS/control-frame validity alone as PRW session authentication or candidate-publication freshness;
- expose a raw unauthenticated endpoint injection path;
- introduce DNS/hostname resolution into `prw-connectivity` candidate endpoints;
- perform socket I/O;
- activate STUN/ICE/TURN;
- open a listener;
- add firewall, route, TUN/TAP or DNS mutation;
- wire production Agent runtime or bootstrap;
- modify production deployment/service-manager state.

## Validation state

Source tests are authored but are not execution evidence while the build/test gate is closed.

Current staged test coverage includes:

Connectivity:

- successful refresh preserves `PeerConnectivityIdentity`;
- successful refresh replaces stale endpoints;
- refreshed observations begin `Unknown`;
- removed candidate IDs fail closed;
- rebinding an existing candidate ID to a different endpoint fails before mutation;
- invalid refresh leaves the previous plan state unchanged.

Bridge admission ordering:

- current requester session + same workspace + current target transport permits refresh;
- target transport identity rotation rejects a stale plan before endpoint mutation;
- target device revocation rejects refresh before endpoint mutation;
- requester membership suspension rejects refresh before endpoint mutation;
- cross-workspace target rejects refresh before endpoint mutation.

Authenticated candidate publication provenance:

- a registry-current authenticated target can publish a bounded candidate set for its own current identity;
- an authenticated requester's own publication cannot be retargeted to another peer plan;
- target transport rotation rejects a previously published stale target identity before mutation;
- target publisher membership suspension rejects a stale publication before mutation;
- a cross-workspace requester cannot consume an authenticated target publication;
- an invalid candidate set is rejected before a publication object exists.

Required future separately-authorized validation:

- run the staged connectivity and bridge integration tests;
- confirm existing path-selection ordering remains unchanged;
- confirm registry/session/provenance error ordering remains fail-closed;
- confirm candidate ID non-rebinding prevents old correlation from targeting a changed endpoint;
- workspace formatting/lints/tests/build remain clean in the authorized scope.

## Current classification

`C02E_DYNAMIC_REACHABILITY_DESIGN_LOCKED / DEVICE_IDENTITY_NOT_IP_BOUND / CANDIDATE_REFRESH_TRANSACTIONAL / CANDIDATE_ID_NON_REBINDABLE / AUTHENTICATED_PUBLISHER_IDENTITY_DERIVED / SESSION_WORKSPACE_TARGET_REGISTRY_ORDER_LOCKED / STALE_CROSS_WORKSPACE_RETARGETED_OR_REBOUND_STATE_FAILS_BEFORE_MUTATION / CANDIDATE_WIRE_AND_REPLAY_ADAPTER_UNSELECTED / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`
