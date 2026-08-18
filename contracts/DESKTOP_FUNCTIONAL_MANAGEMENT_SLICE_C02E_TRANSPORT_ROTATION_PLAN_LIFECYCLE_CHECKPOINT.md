# Phase 152 C02e — Transport Rotation / Connectivity Plan Lifecycle Checkpoint

Status: `DESIGN_LOCK / DEVICE_ID_STABLE / TRANSPORT_ROTATION_INVALIDATES_OLD_PLAN / REPLACEMENT_PLAN_REQUIRED / NO_IN_PLACE_PEER_IDENTITY_REBIND / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Base C02e head: `17743249807dd39ddd14748e19810b6bcc1a8760`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

This checkpoint locks the lifecycle distinction between transient endpoint change and transport-identity rotation.

A phone moving Wi-Fi -> mobile data -> another Wi-Fi may change IP/port candidates without changing its logical PRW `DeviceId` or, by itself, requiring a new `TransportIdentity`. That is an endpoint refresh inside one current `PeerConnectivityPlan` identity.

A transport-key/certificate rotation is different. It creates a new `TransportIdentity`. The old connectivity plan is then stale and must not be repaired by mutating its peer identity in place.

## Repository precedent

The existing architecture already determines the safe behavior:

1. Phase 135 `PeerConnectivityIdentity` contains a logical `DeviceId` plus a separately observable `TransportIdentity` and explicitly states that changing transport identity must not imply changing logical device identity.
2. `PeerConnectivityPlan` owns a fixed `peer` identity. Its refresh API replaces only transient candidates and deliberately preserves that peer identity; there is no peer-identity mutation API.
3. Phase 130 registry rotation is compare-and-rotate: `rotate_transport_identity(...)` atomically replaces the current transport identity only when the expected old identity is current.
4. C02e admission revalidates the exact plan/publication target `TransportIdentity` against current registry state before endpoint mutation.
5. Phase 139 locks transport-key rotation as producing a new `TransportIdentity` with atomic registry update and old-identity retirement/revocation semantics.
6. Phase 139 also disables active QUIC connection migration in the initial profile; path changes may establish a new authenticated connection instead of silently rebinding transport identity.

No new lifecycle mechanism is needed.

## Locked lifecycle rule

### Endpoint-only change

When `DeviceId` and current `TransportIdentity` are unchanged:

- the existing plan identity remains valid;
- authenticated/current candidate publication may replace transient IP/port candidates through `PeerConnectivityPlan::refresh_candidates(...)`;
- the refresh remains transactional;
- candidate observations reset to `Unknown`;
- plan-scoped `CandidateId` non-rebinding remains enforced.

### Transport-identity rotation

After the registry atomically rotates a device from transport identity A to transport identity B:

- the logical `DeviceId` remains the same;
- transport identity A is stale for new admission;
- a `PeerConnectivityPlan` whose peer still contains A is stale and must not receive endpoint refresh;
- a candidate publication under A fails exact current transport validation;
- a candidate publication under B has a different `PeerConnectivityIdentity` and therefore must not be applied to the stale A plan;
- the caller must construct a replacement `PeerConnectivityPlan` using the same logical `DeviceId` and current transport identity B;
- candidate/observation/selected-path state from the stale plan must not be transferred as authenticated reachability evidence into the replacement plan;
- candidate identifiers are scoped to their plan lifetime; no authorization or reachability continuity may be inferred merely from numeric ID reuse across replacement plans.

The replacement is an identity/lifecycle transition, not an endpoint mutation.

## Authenticated-session relationship

Transport rotation alone does not redefine the logical authenticated PRW device identity.

The current `AuthenticatedDeviceSession` remains governed by the session and registry binding rules for workspace/user/device/public device identity. Candidate publication still has to present and revalidate the device's newly current `TransportIdentity` separately.

A later runtime may additionally bind an application session or live QUIC connection to a concrete transport connection lifetime, but C02e does not invent that runtime policy here.

## Fail-closed ordering

For an old plan after transport rotation:

1. requester session must still be registry-current;
2. publisher session must still be registry-current;
3. workspace equality must still hold;
4. publication peer must exactly equal the target plan peer;
5. the exact target transport identity must be current in registry;
6. only then may candidate mutation occur.

Thus either an old-A publication fails at current transport validation, or a new-B publication fails exact publication-to-old-plan identity equality. Neither path can mutate the stale plan.

## Security invariants

C02e must not:

- mutate a plan's `PeerConnectivityIdentity` from transport A to B in place;
- treat endpoint refresh as transport-key rotation;
- treat transport-key rotation as a mere IP/port update;
- change logical `DeviceId` merely because `TransportIdentity` rotates;
- transfer stale plan reachability observations to the replacement plan;
- accept old transport identity because its certificate/key was previously valid;
- infer transport currentness from IP address, candidate ID, generic frame request ID or TLS transport success alone;
- activate QUIC migration, sockets, ICE/STUN/TURN, Agent runtime wiring or production networking.

## Validation state

This checkpoint is source/design review only.

Existing staged source already covers the key negative edge: rotation causes stale target transport validation to fail before `PeerConnectivityPlan` mutation.

No build, `cargo fmt`, Clippy, test, workflow dispatch or runtime/network execution is performed here.

## Next safe seam

With plan lifecycle now explicit, the next C02e design seam is candidate-publication freshness/replay semantics. Phase 129 generic `request_id` is not sufficient precedent. Reuse an existing verifier-owned freshness pattern if one cleanly fits candidate publication; otherwise keep the production candidate wire adapter unselected/fail-closed rather than inventing values.
