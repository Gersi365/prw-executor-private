# Phase 152 C02e — Dynamic Reachability Audit

Status: `IMPLEMENTATION_STAGED / STATIC_SCOPE_VERIFIED / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

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

Phase 139 also explicitly states that actual QUIC destination discovery comes from selected explicit IP/UDP candidates and does not use the transport certificate DNS SAN for endpoint discovery.

Therefore the correct C02e change is candidate refresh under stable peer identity, not static-IP identity and not a second registry model.

## Source staged

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

## Tests authored but NOT RUN

Two new source tests are staged:

- `candidate_refresh_preserves_identity_and_replaces_transient_endpoints`
- `invalid_candidate_refresh_preserves_previous_state`

They cover:

- stable peer identity across endpoint replacement;
- removal of stale candidate IDs;
- observation reset to `Unknown`/Offline after refresh;
- new candidate observation and selection;
- transactional preservation of prior state on invalid refresh.

These tests are source specifications only while the build/test gate remains closed.

## Static mutation review

At the first implementation checkpoint the branch is stacked exactly on C02d and changes only:

1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02E_DYNAMIC_REACHABILITY_GATE.md`
2. `crates/prw-connectivity/src/lib.rs`
3. this audit file

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

`authenticated PRW device/session identity -> current registry/control-plane candidate state -> current candidate selection -> authenticated transport`

A mobile client's IP changing is therefore a candidate refresh event, not a device re-enrollment or static allowlist identity change.

## Explicit non-claims

C02e does not claim:

- production candidate discovery is wired;
- STUN/ICE/TURN is activated;
- real sockets are opened;
- a QUIC connection is established;
- Agent runtime integration exists;
- forwarding policy has been widened;
- build/fmt/Clippy/tests have run;
- deployment or host sync is authorized.

## Current classification

`C02E_DYNAMIC_REACHABILITY_IMPLEMENTATION_STAGED / IDENTITY_STABLE_ENDPOINTS_TRANSIENT / TRANSACTIONAL_REFRESH / STATIC_SCOPE_VERIFIED / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`
