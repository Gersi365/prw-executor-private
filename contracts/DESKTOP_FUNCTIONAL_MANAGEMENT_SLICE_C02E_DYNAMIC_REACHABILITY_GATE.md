# Phase 152 C02e — Dynamic Reachability Gate

Status: `DESIGN_LOCK / IDENTITY_STABLE_ENDPOINTS_TRANSIENT / SESSION_WORKSPACE_REGISTRY_REFRESH_ORDER_LOCKED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

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

It reuses `WorkspaceDeviceRegistry` as the authority for current session/device/transport state.

It also preserves the existing Phase 139 transport architecture:

`DeviceId -> current TransportIdentity -> candidate exchange -> selected explicit IP/UDP candidate -> authenticated QUIC/TLS transport`

The deterministic preference remains:

`LocalDirect -> InternetDirect -> Relay -> Offline`

Phase 141 remains the Sans-I/O ICE/STUN layer. Successful ICE selection produces reachability evidence correlated to an existing candidate; it does not become a PRW identity or authorization authority.

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
- replace the prior candidate set atomically on success;
- discard stale candidate observations;
- initialize every refreshed candidate observation to `Unknown`;
- make removed candidate IDs immediately unknown to subsequent observation updates;
- leave the complete prior candidate/observation state unchanged if validation fails.

Resetting observations is required because reachability evidence for an old network path must not be silently transferred to a newly signaled endpoint set.

## Authenticated requester and workspace admission rule

A later runtime integration must not apply a target candidate refresh merely because a candidate vector is syntactically valid.

The required admission order is now locked as:

1. revalidate the requester's `AuthenticatedDeviceSession` through `WorkspaceDeviceRegistry::validate_authenticated_session(...)`;
2. locate the target `DeviceId` from the existing `PeerConnectivityPlan` in the current registry;
3. require the target registered device to belong to the same `WorkspaceId` as the current registry-validated requester;
4. revalidate the plan's exact target `TransportIdentity` through `WorkspaceDeviceRegistry::validate_transport_identity(...)`;
5. only after all current identity/workspace checks pass may `PeerConnectivityPlan::refresh_candidates(...)` validate and mutate transient endpoints.

Any failure before step 5 must leave the complete plan state unchanged.

This ordering closes these stale-authority cases before endpoint mutation:

- requester membership suspension/removal;
- requester device revocation or authenticated-session/registry mismatch;
- cross-workspace target access;
- target device revocation;
- target transport identity rotation leaving the plan's old transport identity stale.

C02e stages this order in `prw-remote-bridge` integration-test source because that crate already depends on session, registry and connectivity domains. No Cargo edge or runtime wiring is added.

## Candidate provenance boundary

Current session/workspace/target registry validation answers **who may participate and which target identity is current**.

It does not, by itself, prove that arbitrary candidate bytes came from an authenticated control-plane signaling channel.

Production candidate provenance therefore remains a separate later boundary. A future signaling adapter must correlate a bounded candidate update to the already admitted workspace/target identity and must not expose a raw unauthenticated endpoint injection path.

C02e deliberately does not invent a new wire format because the current control transport supplies a generic bounded frame envelope while the repository does not yet contain a reviewed candidate-update application payload schema.

## Registry/discovery relationship

The higher-level product flow is:

`authenticated PRW requester session`

`-> current registry principal`

`-> same-workspace current target device + target TransportIdentity`

`-> authenticated control-plane candidate state`

`-> bounded transient candidate refresh`

`-> provider reachability observations`

`-> deterministic path selection`

`-> authenticated transport establishment`

The source of refreshed production candidates remains outside C02e. Existing control-plane transport, NAT traversal and relay components own their respective transport/protocol responsibilities.

## Forwarding relationship

The C02d `ExactForwardingEgressPolicy` remains a low-level exact IP+TCP-port primitive for explicitly fixed service targets.

It must not be interpreted as the primary device-to-device identity mechanism.

Device-to-device PRW connectivity should resolve the current network endpoint from authenticated device/transport state. A mobile client changing IP addresses therefore does not require editing a static identity allowlist.

This C02e gate does not widen C02d forwarding to DNS names, CIDRs, wildcards, arbitrary destinations, or runtime-selected untrusted targets.

## Security invariants

C02e must not:

- derive PRW identity from IP address;
- persist a candidate IP as a substitute for `DeviceId`;
- carry reachability observations across candidate refresh;
- accept more than 16 candidates;
- preserve removed candidates as still selectable;
- apply a refresh when the requester session is no longer registry-current;
- apply a cross-workspace target refresh;
- apply a candidate refresh after current-registry target device revocation;
- apply a candidate refresh after the plan's target transport identity becomes stale due to rotation;
- treat registry identity validation alone as authentication of arbitrary candidate bytes;
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
- invalid refresh leaves the previous plan state unchanged.

Bridge admission ordering:

- current requester session + same workspace + current target transport permits refresh;
- target transport identity rotation rejects a stale plan before endpoint mutation;
- target device revocation rejects refresh before endpoint mutation;
- requester membership suspension rejects refresh before endpoint mutation;
- cross-workspace target rejects refresh before endpoint mutation.

Required future separately-authorized validation:

- run the staged connectivity and bridge integration tests;
- confirm existing path-selection ordering remains unchanged;
- confirm registry/session error ordering remains fail-closed;
- workspace formatting/lints/tests/build remain clean in the authorized scope.

## Current classification

`C02E_DYNAMIC_REACHABILITY_DESIGN_LOCKED / DEVICE_IDENTITY_NOT_IP_BOUND / CANDIDATE_REFRESH_TRANSACTIONAL / SESSION_WORKSPACE_TARGET_REGISTRY_ORDER_LOCKED / STALE_OR_CROSS_WORKSPACE_AUTHORITY_FAILS_BEFORE_MUTATION / CANDIDATE_WIRE_PROVENANCE_STILL_UNSELECTED / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`
