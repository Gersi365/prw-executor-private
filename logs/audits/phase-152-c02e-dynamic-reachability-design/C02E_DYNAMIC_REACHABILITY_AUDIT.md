# Phase 152 C02e — Dynamic Reachability Audit

Status: `IMPLEMENTATION_STAGED / AUTHENTICATED_CANDIDATE_PUBLICATION_PROVENANCE_STAGED / SESSION_WORKSPACE_TARGET_REFRESH_ORDER_STAGED / STATIC_SCOPE_VERIFIED / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Predecessor head:

`857583b25ed1206317641a93fd8f927819c954d8`

Branch:

`phase-152-c02e-dynamic-reachability-design`

## Scope

C02e captures the product requirement that a PRW device may change its current network endpoint without changing logical device identity.

This directly covers normal phone movement between Wi-Fi, 5G, another Wi-Fi, NAT/CGNAT and relay-assisted paths.

The checkpoint remains source/design only. It does not activate control-plane signaling, sockets, NAT traversal, relay traffic or production runtime wiring.

## Repository precedent reviewed

C02e reuses the existing architecture rather than inventing a parallel model.

Existing `prw-connectivity` separates:

- `DeviceId` — logical device identity;
- `TransportIdentity` — independently rotatable transport identity;
- `ConnectivityEndpoint` — explicit transient IP + port;
- `ConnectivityCandidate` — one candidate endpoint/path class;
- `PeerConnectivityPlan` — bounded candidate set and reachability observations.

Existing `prw-session` provides `AuthenticatedDeviceSession` without granting capabilities.

Existing `prw-registry` revalidates authenticated sessions against current membership/device state and owns current transport identity through `WorkspaceDeviceRegistry::validate_transport_identity(...)`.

Existing `prw-remote-bridge` already depends on session, registry and connectivity domains, so C02e can stage cross-domain admission/provenance semantics without a new Cargo edge.

Phase 139 assigns candidate exchange and coordination to the separate control plane and explicitly states that QUIC destination discovery comes from selected explicit IP/UDP candidates rather than certificate DNS SAN resolution.

Phase 141 keeps ICE/STUN Sans-I/O, requires remote ICE credentials through authenticated PRW coordination metadata, and correlates selected reachability back to existing Phase 135 candidates.

Therefore the C02e direction remains candidate refresh under authenticated logical/transport identity, not static-IP identity and not a second discovery authority.

## Connectivity source staged

Changed source:

`crates/prw-connectivity/src/lib.rs`

Public seam:

`PeerConnectivityPlan::refresh_candidates(...)`

Semantics staged:

1. validate the complete proposed candidate set before mutation;
2. preserve the existing maximum of 16 candidates;
3. reject duplicate candidate IDs;
4. reject duplicate exact `(path kind, endpoint)` values;
5. preserve peer `DeviceId` and `TransportIdentity`;
6. replace the full transient candidate set on success;
7. reset all refreshed observations to `Unknown`;
8. make removed candidate IDs unknown immediately;
9. preserve prior candidate/observation state on invalid refresh.

No dependency or Cargo mutation was introduced.

## Session/workspace/target admission ordering staged

Integration-test source:

`crates/prw-remote-bridge/tests/dynamic_reachability_registry.rs`

The current staged ordering revalidates:

1. requester `AuthenticatedDeviceSession` against current registry state;
2. target device identity from the current plan;
3. requester/target same-workspace relationship;
4. target plan `TransportIdentity` against current registry state;
5. only then candidate-set validation/mutation.

Staged fail-closed cases include:

- requester membership suspension;
- requester device/session registry mismatch through the existing validator;
- cross-workspace target access;
- target device revocation;
- target transport rotation leaving the plan stale.

Every rejection occurs before `PeerConnectivityPlan::refresh_candidates(...)` and therefore before endpoint mutation.

## Authenticated candidate publication provenance staged

New integration-test source:

`crates/prw-remote-bridge/tests/authenticated_candidate_provenance.rs`

This file stages a private source-only `AuthenticatedCandidatePublication` semantic boundary. It does not define a production protocol type or wire encoding.

Publication construction requires:

1. an already authenticated publisher device session;
2. current registry revalidation of that publisher session;
3. current registry validation of the publisher's presented `TransportIdentity`;
4. derivation of the publication `PeerConnectivityIdentity` from the authenticated publisher's own `DeviceId` plus that current `TransportIdentity`;
5. full candidate-set validation through existing `PeerConnectivityPlan` bounds before a publication can exist.

The caller cannot supply an arbitrary target `DeviceId` during publication construction.

Publication consumption requires:

1. current registry revalidation of the requester session;
2. current registry revalidation of the publisher/target session retained as source-level provenance evidence;
3. requester and publisher/target to remain in the same current workspace;
4. exact publication peer identity equality with the target `PeerConnectivityPlan` identity;
5. current registry revalidation of the target `TransportIdentity`;
6. only then transactional candidate refresh.

This stages the security property that authenticated candidate state is attributable to the device whose reachability is being advertised. A valid session for device A cannot be used to rename candidate bytes as device B merely by choosing B's plan at consumption time.

The target publisher is revalidated again at consumption so later target membership suspension/device revocation also invalidates an older publication before endpoint mutation.

## Wire/control-plane boundary remains closed

C02e does not invent or activate a candidate-update wire schema.

The repository has a bounded generic Phase 129 control-frame envelope, and Phase 139 assigns candidate exchange to authenticated control-plane coordination, but no reviewed candidate-update application payload codec or session-bound production adapter exists yet.

The source-only provenance semantics therefore lock what a later adapter must preserve without pretending that generic TLS/frame validity alone authenticates PRW candidate bytes.

No `prw-control-transport` dependency was added to `prw-remote-bridge`; no production control-plane runtime path was wired.

## Tests authored but NOT RUN

Connectivity source tests:

- `candidate_refresh_preserves_identity_and_replaces_transient_endpoints`
- `invalid_candidate_refresh_preserves_previous_state`

Registry/admission integration tests:

- `current_session_workspace_and_target_identity_allow_transient_refresh`
- `transport_rotation_rejects_stale_plan_before_endpoint_mutation`
- `target_device_revocation_rejects_refresh_before_endpoint_mutation`
- `requester_membership_suspension_rejects_refresh_before_endpoint_mutation`
- `cross_workspace_target_rejects_refresh_before_endpoint_mutation`

Authenticated publication provenance integration tests:

- `authenticated_target_publication_allows_same_workspace_refresh`
- `authenticated_requester_publication_cannot_be_retargeted_to_peer_plan`
- `target_transport_rotation_rejects_stale_publication_before_mutation`
- `target_membership_suspension_rejects_stale_publication_before_mutation`
- `cross_workspace_requester_cannot_consume_authenticated_target_publication`
- `invalid_candidate_set_is_rejected_before_publication_exists`
- `fixture_keeps_requester_and_target_identities_distinct`

These tests are source specifications only while the build/test gate remains closed.

## Static mutation review

C02e remains stacked exactly on C02d.

Current intended mutation surface is limited to:

1. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02E_DYNAMIC_REACHABILITY_GATE.md`
2. `crates/prw-connectivity/src/lib.rs`
3. `crates/prw-remote-bridge/tests/dynamic_reachability_registry.rs`
4. `crates/prw-remote-bridge/tests/authenticated_candidate_provenance.rs`
5. this audit file

No changes are intended to:

- root `Cargo.toml`;
- `Cargo.lock`;
- Agent Cargo manifest;
- Agent `main.rs`;
- production bootstrap/runtime loop;
- control-plane transport runtime source;
- NAT traversal runtime adapters;
- relay service runtime;
- Android application source;
- desktop application source;
- systemd packaging;
- signing/credential source;
- firewall/routes/DNS/TUN/TAP;
- deployment.

## Relationship to C02d forwarding egress

C02d exact IP+TCP-port egress policy remains a valid low-level primitive for explicitly fixed service targets.

It is not the device identity model and was not modified by C02e.

C02e locks the higher-level device flow as:

`authenticated target session -> current target DeviceId + TransportIdentity -> bounded candidate publication -> current same-workspace requester -> transactional refresh -> reachability observation -> selected candidate -> authenticated transport`

A mobile client's IP changing is therefore a candidate refresh event, not a device re-enrollment or static allowlist identity change.

## Explicit non-claims

C02e does not claim:

- a production candidate-update wire schema exists;
- production candidate discovery/signaling is wired;
- generic control-plane TLS or frame validity alone is PRW session authentication;
- STUN/ICE/TURN is activated;
- real sockets are opened;
- a QUIC connection is established by this checkpoint;
- Agent runtime integration exists;
- forwarding policy has been widened;
- build, rustfmt, Clippy or tests have run;
- workflow dispatch occurred;
- deployment, signing, privileged mutation or Host Mirror source sync is authorized.

## Current classification

`C02E_DYNAMIC_REACHABILITY_IMPLEMENTATION_STAGED / IDENTITY_STABLE_ENDPOINTS_TRANSIENT / TRANSACTIONAL_REFRESH / AUTHENTICATED_PUBLISHER_IDENTITY_DERIVED / REQUESTER_AND_TARGET_REGISTRY_CURRENT / SAME_WORKSPACE_REQUIRED / RETARGETED_OR_STALE_PUBLICATION_FAILS_BEFORE_MUTATION / CANDIDATE_WIRE_ADAPTER_UNSELECTED / STATIC_SCOPE_VERIFIED / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`
