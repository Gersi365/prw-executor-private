# Phase 152 C02e — Transport Rotation / Plan Lifecycle Static Audit

Status: `PASS_STATIC_DESIGN_REVIEW / DEVICE_ID_STABLE / TRANSPORT_ROTATION_REQUIRES_REPLACEMENT_PLAN / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Audit base head: `17743249807dd39ddd14748e19810b6bcc1a8760`

Frozen C02d checkpoint: `857583b25ed1206317641a93fd8f927819c954d8`

## Authoritative source reviewed

- `contracts/PRIVATE_MESH_CONNECTIVITY_FOUNDATION_CONTRACT.md`;
- current `crates/prw-connectivity/src/lib.rs`;
- current `crates/prw-registry/src/lib.rs`;
- `contracts/REMOTE_TRANSPORT_ARCHITECTURE_DECISION.md`;
- current C02e dynamic-reachability gate;
- current C02e authenticated candidate provenance / semantic-adapter source.

## Findings

1. Phase 135 explicitly separates logical `DeviceId` from `TransportIdentity`.
2. `PeerConnectivityPlan` has a fixed peer identity; candidate refresh does not and cannot mutate that identity.
3. Registry transport rotation is an explicit compare-and-rotate current-state transition.
4. Phase 139 defines transport-key rotation as producing a new `TransportIdentity` and requires atomic registry update with retirement/revocation semantics for the old identity.
5. Phase 139's initial profile does not use active QUIC migration; candidate/path changes may create a new authenticated connection.
6. Current C02e admission already rejects a stale old-transport plan before endpoint mutation after registry rotation.
7. A publication under the new transport identity necessarily has a different `PeerConnectivityIdentity`, so exact publication/plan equality prevents it from being retargeted into the old plan.
8. The minimal compatible rule is therefore replacement-plan lifecycle, not a new peer-rebinding API.

## Locked conclusion

`ENDPOINT_CHANGE -> SAME_CURRENT_PEER_IDENTITY -> TRANSACTIONAL_CANDIDATE_REFRESH`

`TRANSPORT_IDENTITY_ROTATION -> OLD_PLAN_STALE -> SAME_DEVICE_ID_PLUS_NEW_TRANSPORT_IDENTITY -> REPLACEMENT_PLAN`

No candidate observation, selected path, endpoint or candidate identifier from the old plan is authorization evidence for the replacement plan.

## Mutation surface

Added only:

- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02E_TRANSPORT_ROTATION_PLAN_LIFECYCLE_CHECKPOINT.md`;
- this static audit record.

No Rust source, Cargo manifest, lockfile, Agent source, runtime source, C02d source or immutable Drive authority is modified by this checkpoint.

## Explicitly not executed

- build;
- `cargo fmt`;
- Clippy;
- tests;
- workflow dispatch;
- real TCP/UDP I/O;
- STUN/ICE/TURN activity;
- QUIC connection/migration;
- PTY/process I/O;
- Agent/bootstrap activation;
- deployment;
- signing;
- privileged/system mutation;
- Host Mirror source synchronization.

## Result

`STATIC_DESIGN_REVIEW_PASS / TRANSPORT_ROTATION_INVALIDATES_STALE_PLAN / IN_PLACE_PEER_REBIND_FORBIDDEN / DEVICE_ID_PRESERVED / C02D_UNTOUCHED`
