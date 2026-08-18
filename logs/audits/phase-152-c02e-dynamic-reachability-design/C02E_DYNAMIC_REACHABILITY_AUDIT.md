# Phase 152 C02e — Dynamic Reachability Audit

Status: `IMPLEMENTATION_STAGED / REGISTRY_CURRENT_REFRESH_ORDER_STAGED / STATIC_SCOPE_VERIFIED / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Predecessor head:

`857583b25ed1206317641a93fd8f927819c954d8`

Branch:

`phase-152-c02e-dynamic-reachability-design`

## Scope

C02e captures the product requirement that a PRW device may change its current network endpoint without changing logical device identity.

This directly covers normal phone movement between Wi-Fi, 5G, another Wi-Fi, NAT/CGNAT and relay-assisted paths.

## Repository precedent reviewed

C02e reuses the existing architecture rather than inventing a parallel model.

Existing `prw-connectivity` already separates:

- `DeviceId` — logical device identity;
- `TransportIdentity` — independent transport identity;
- `ConnectivityEndpoint` — explicit transient IP + port;
- `ConnectivityCandidate` — one candidate endpoint/path class;
- `PeerConnectivityPlan` — bounded candidate set and reachability observations.

Existing `prw-registry` already owns current enrolled-device lifecycle and current transport-identity validation through `WorkspaceDeviceRegistry::validate_transport_identity(...)`.

Existing `prw-remote-bridge` already depends on both `prw-connectivity` and `prw-registry`, so C02e can stage their required admission ordering without adding a new dependency edge or parallel registry model.

Phase 139 also explicitly states that actual QUIC destination discovery comes from selected explicit IP/UDP candidates and does not use the transport certificate DNS SAN for endpoint discovery.

Therefore the correct C02e direction is candidate refresh under stable peer identity plus current-registry revalidation, not static-IP identity and not a second discovery authority.

## Connectivity source staged

Changed source:

`crates/prw-connectivity/src/lib.rs`

New public seam:

`PeerConnectivityPlan::refresh_candidates(...)`

Semantics staged:

1. proposed candidate set is validated before mutation;
2. existing maximum remains 16 candidates;
3. duplicate candidate IDs fail closed;
4. duplicate exact `(path kind, endpoint)` values fail closed;
5. peer `DeviceId` and `TransportIdentity` are not modified;
6. successful refresh replaces the full transient candidate set;
7. all refreshed observations start `Unknown`;
8. removed candidate IDs become unknown immediately;
9. invalid refresh leaves prior candidate and observation state unchanged.

No new dependency or Cargo mutation was introduced.

## Registry-current admission ordering staged

New integration-test source:

`crates/prw-remote-bridge/tests/dynamic_reachability_registry.rs`

It stages the required pure admission ordering:

1. read `DeviceId` and `TransportIdentity` from the current `PeerConnectivityPlan`;
2. revalidate that exact transport identity through `WorkspaceDeviceRegistry::validate_transport_identity(...)`;
3. on registry failure, return before calling `refresh_candidates(...)`;
4. only a registry-current peer identity may proceed to candidate-set validation/mutation.

This specifically means:

- device revocation blocks refresh before endpoint mutation;
- transport identity rotation makes a plan carrying the old identity stale and blocks refresh before endpoint mutation;
- a normal endpoint change with the same current device/transport identity remains admissible;
- registry validation does not by itself authenticate arbitrary candidate bytes; authenticated control-plane/session candidate provenance remains a later integration boundary.

The staged helper exists only inside integration-test source. No production bridge/runtime API was activated by this checkpoint.

## Tests authored but NOT RUN

Connectivity source tests:

- `candidate_refresh_preserves_identity_and_replaces_transient_endpoints`
- `invalid_candidate_refresh_preserves_previous_state`

Bridge integration tests:

- `current_registry_identity_allows_transient_candidate_refresh`
- `transport_rotation_rejects_stale_plan_before_endpoint_mutation`
- `device_revocation_rejects_refresh_before_endpoint_mutation`

Together they specify:

- stable peer identity across endpoint replacement;
- removal of stale candidate IDs;
- observation reset to `Unknown`/Offline after refresh;
- transactional preservation of prior state on invalid refresh;
- current registry identity is checked before endpoint mutation;
- stale transport identity after rotation fails before mutation;
- revoked device fails before mutation.

These tests are source specifications only while the build/test gate remains closed.

## Static mutation review

C02e remains stacked exactly on C02d.

Current intended mutation surface is limited to:

1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02E_DYNAMIC_REACHABILITY_GATE.md`
2. `crates/prw-connectivity/src/lib.rs`
3. `crates/prw-remote-bridge/tests/dynamic_reachability_registry.rs`
4. this audit file

No changes are intended to:

- root `Cargo.toml`;
- `Cargo.lock`;
- Agent Cargo manifest;
- Agent `main.rs`;
- production bootstrap/runtime loop;
- Android application source;
- desktop application source;
- systemd packaging;
- signing/credential source;
- NAT traversal runtime adapters;
- relay service runtime;
- firewall/routes/DNS/TUN/TAP;
- deployment.

## Relationship to C02d forwarding egress

C02d exact IP+TCP-port egress policy remains a valid low-level primitive for explicitly fixed service targets.

It is not the device identity model.

C02e locks the higher-level device flow as:

`authenticated PRW device/session identity -> current registry/control-plane candidate state -> registry-current transport identity revalidation -> current candidate selection -> authenticated transport`

A mobile client's IP changing is therefore a candidate refresh event, not a device re-enrollment or static allowlist identity change.

## Explicit non-claims

C02e does not claim:

- production candidate discovery is wired;
- arbitrary candidate bytes are authenticated by registry validation alone;
- STUN/ICE/TURN is activated;
- real sockets are opened;
- a QUIC connection is established;
- Agent runtime integration exists;
- forwarding policy has been widened;
- build/fmt/Clippy/tests have run;
- deployment or host sync is authorized.

## Current classification

`C02E_DYNAMIC_REACHABILITY_IMPLEMENTATION_STAGED / IDENTITY_STABLE_ENDPOINTS_TRANSIENT / TRANSACTIONAL_REFRESH / REGISTRY_CURRENT_REFRESH_ORDER_STAGED / STALE_TRANSPORT_AND_REVOKED_DEVICE_FAIL_BEFORE_MUTATION / STATIC_SCOPE_VERIFIED / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`
