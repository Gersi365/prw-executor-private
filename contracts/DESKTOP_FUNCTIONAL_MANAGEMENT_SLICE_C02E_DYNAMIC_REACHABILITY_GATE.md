# Phase 152 C02e — Dynamic Reachability Gate

Status: `DESIGN_LOCK / IDENTITY_STABLE_ENDPOINTS_TRANSIENT / REGISTRY_CURRENT_REFRESH_ORDER_LOCKED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

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

It also preserves the existing Phase 139 transport architecture:

`DeviceId -> current TransportIdentity -> candidate exchange -> selected explicit IP/UDP candidate -> authenticated QUIC/TLS transport`

The deterministic preference remains:

`LocalDirect -> InternetDirect -> Relay -> Offline`

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

## Registry-current refresh admission rule

Before a later runtime integration may apply a refreshed candidate set to an existing peer plan, the plan's current identity must be revalidated against the current registry state.

The locked ordering is:

1. read the existing plan's `DeviceId` and `TransportIdentity`;
2. call the current registry transport-identity validation boundary;
3. if the device is unknown, revoked, unbound, or the transport identity is stale/mismatched, fail before candidate mutation;
4. only after successful registry revalidation may `PeerConnectivityPlan::refresh_candidates(...)` run;
5. candidate validation failure must still preserve the complete pre-refresh plan state.

This ordering prevents a stale network update from surviving either device revocation or transport-key/certificate rotation.

C02e stages this ordering in `prw-remote-bridge` integration-test source because that crate already depends on both `prw-registry` and `prw-connectivity`. It does not add a new dependency cycle or modify runtime wiring.

The production source of candidate updates still requires authenticated current control-plane/session signaling before runtime activation. C02e does not claim that arbitrary candidate vectors are authenticated merely because registry identity revalidation succeeds.

## Registry/discovery relationship

The higher-level product flow is:

`authenticated PRW device/session identity`

`-> current registry/control-plane state`

`-> bounded current candidate set`

`-> provider reachability observations`

`-> deterministic path selection`

`-> authenticated transport establishment`

The source of refreshed production candidates remains outside C02e. Existing control-plane signaling, NAT traversal and relay components own candidate discovery/exchange according to their respective contracts.

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
- apply a candidate refresh after current-registry device revocation;
- apply a candidate refresh after the plan's transport identity becomes stale due to rotation;
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

- successful refresh preserves `PeerConnectivityIdentity`;
- successful refresh replaces stale endpoints;
- refreshed observations begin `Unknown`;
- removed candidate IDs fail closed;
- invalid refresh leaves the previous plan state unchanged;
- current registry transport identity permits refresh;
- transport identity rotation rejects a stale plan before endpoint mutation;
- device revocation rejects refresh before endpoint mutation.

Required future separately-authorized validation:

- run the staged connectivity and bridge integration tests;
- confirm existing path-selection ordering remains unchanged;
- confirm registry error ordering remains fail-closed;
- workspace formatting/lints/tests/build remain clean in the authorized scope.

## Current classification

`C02E_DYNAMIC_REACHABILITY_DESIGN_LOCKED / DEVICE_IDENTITY_NOT_IP_BOUND / CANDIDATE_REFRESH_TRANSACTIONAL / REGISTRY_CURRENT_REFRESH_ORDER_LOCKED / STALE_TRANSPORT_AND_REVOKED_DEVICE_FAIL_BEFORE_MUTATION / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`
