# Phase 152 C02e — Candidate Publication Freshness Authority Placement Static Audit

Status: `PASS_STATIC_OWNERSHIP_REVIEW / UPPER_REACHABILITY_FRESHNESS_AUTHORITY / SAME_PEER_SESSION_RENEWAL_CONTINUES_FRESHNESS / REQUESTER_INDEPENDENT / TRANSPORT_ROTATION_RESETS_BY_NEW_IDENTITY_ONLY / RESTART_FAIL_CLOSED / REPRESENTATION_UNSELECTED / BUILD_GATE_CLOSED`

Review base head: `02e8d41eb33caf1b7cbe8f53045516b1dd242619`

Frozen C02d checkpoint: `857583b25ed1206317641a93fd8f927819c954d8`

## Evidence reviewed

- current C02e candidate freshness/replay checkpoint;
- Phase 128 `REMOTE_DEVICE_SESSION_AUTH_CONTRACT.md`;
- current `prw-control-plane/src/session_auth.rs` challenge/proof/state domain;
- current `SessionAuthenticationService` pending/completed ownership;
- current `WorkspaceDeviceRegistry` identity/current-transport ownership;
- current private C02e authenticated candidate semantic adapter;
- C02e one-shot reachability composition and linearization checkpoints.

## Findings

1. Phase 128 freshness state is specifically short-lived `SessionId`-bound cryptographic authentication challenge state, including exact 32-byte nonce and bounded challenge window. Reusing that concrete type/value as candidate-publication freshness would conflate domains and silently select a representation.
2. `SessionAuthenticationService` owns authentication transactions keyed by `SessionId`; it is not the owner of ongoing reachability publication ordering.
3. `WorkspaceDeviceRegistry` owns current membership/device/transport identity and correctly revalidates candidate admission, but it does not model candidate publication history or traversal lifecycle.
4. `AuthenticatedCandidatePublication` is an immutable bounded semantic snapshot and must not own mutable verifier replay state.
5. Candidate-publication freshness therefore belongs to the upper reachability composition authority, directly or through a transactionally coupled verifier subauthority.
6. The minimum connectivity lifecycle discriminator is the exact current publisher `PeerConnectivityIdentity` (`DeviceId + TransportIdentity`) under current registry-authenticated publisher context.
7. Freshness is target/publisher state, not requester state. Same-workspace requesters must not receive separate replay namespaces for the same target peer.
8. Ordinary authenticated session renewal/reconnect with unchanged registry-current `DeviceId + TransportIdentity` must not reset publication freshness. A new `SessionId` is provenance context, not a new candidate-publication ordering lifecycle.
9. `TransportIdentity` rotation creates a replacement peer lifecycle; old plan/traversal/freshness state is stale and cannot authorize the replacement identity.
10. Initial freshness for a legitimate new peer lifecycle must be verifier-controlled; caller-selected initial baseline is not acceptable.
11. Restart/failover must not silently reset freshness for an existing peer lifecycle. If exact current authority cannot be recovered, candidate publication acceptance must fail closed until a separately reviewed recovery/re-baselining path exists.
12. Publication freshness and traversal-observation lifecycle currentness remain separate security states even though the future upper owner coordinates both.

## Mutation surface

Added only:

- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02E_CANDIDATE_PUBLICATION_FRESHNESS_AUTHORITY_PLACEMENT.md`;
- this static audit record.

No existing Rust source, Cargo manifest, lockfile, C02d source, registry/session implementation, production module graph, runtime/network state, deployment state or immutable authority is modified.

## Not executed

- build;
- rustfmt;
- Clippy;
- tests;
- workflow dispatch;
- Cargo resolution;
- TCP/UDP I/O;
- STUN/ICE/TURN activation;
- QUIC activity;
- Agent/bootstrap activation;
- deployment;
- signing;
- privileged/system mutation;
- PR creation/merge;
- Host Mirror synchronization.

## Result

`STATIC_OWNERSHIP_REVIEW_PASS / PUBLICATION_FRESHNESS_LIVES_WITH_UPPER_REACHABILITY_COMPOSITION_STATE / SESSION_AUTH_AND_REGISTRY_DOMAINS_REMAIN_SEPARATE / SAME_PEER_SESSION_RENEWAL_CANNOT_RESET_REPLAY_BASELINE / REQUESTER_DOES_NOT_DEFINE_REPLAY_NAMESPACE / RESTART_WITHOUT_CURRENT_STATE_FAILS_CLOSED / CONCRETE_FRESHNESS_REPRESENTATION_AND_PRODUCTION_OWNER_REMAIN_UNSELECTED / C02D_UNTOUCHED`
