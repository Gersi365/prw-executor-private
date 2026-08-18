# Phase 152 C02e — Candidate Publication Semantic Adapter Checkpoint

Status: `SOURCE_ONLY_SEMANTIC_ADAPTER_STAGED / WIRE_UNSELECTED / REPLAY_FRESHNESS_UNSELECTED / UNEXPORTED / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Base C02e head: `cb41389d16267e1a733c865689cacdc2a06fda13`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

This checkpoint continues C02e from the authenticated candidate-publication provenance and candidate-correlation non-rebinding design without inventing a production candidate-update wire format.

The repository precedent is sufficient to factor the already-reviewed provenance and admission semantics into reusable source, but it is not sufficient to allocate a candidate application message kind, candidate payload magic/version, or replay/freshness mechanism.

## Repository precedent reviewed

The existing boundaries remain authoritative:

- Phase 128 `AuthenticatedDeviceSession` establishes authenticated PRW session identity;
- Phase 129 `PRWC` control transport supplies only a bounded generic envelope and a non-zero `request_id`;
- Phase 130 `WorkspaceDeviceRegistry` owns current membership/device/transport revalidation;
- Phase 135 `PeerConnectivityPlan` owns bounded candidates, transactional refresh and deterministic path selection;
- Phase 139 assigns candidate exchange to authenticated control-plane coordination while preserving `DeviceId`, `TransportIdentity` and transient endpoints as distinct concepts;
- Phase 141 correlates reachability observations to stable plan-scoped `CandidateId` values and does not become an identity authority;
- Phase 143 demonstrates the project pattern of keeping transport framing distinct from application authorization semantics.

The Phase 129 generic `request_id` is not specified as a candidate-publication anti-replay sequence and is therefore not promoted into one here.

## Staged semantic source

`crates/prw-remote-bridge/src/candidate_reachability.rs` now contains the source-level semantic object and admission operations previously expressed only as local helpers in C02e integration-test source.

The adapter preserves this order:

1. revalidate the authenticated publisher session through the current registry;
2. revalidate the publisher's exact current `TransportIdentity`;
3. derive publication `PeerConnectivityIdentity` from that authenticated publisher device plus current transport identity;
4. validate the complete candidate vector before a publication object exists;
5. on consumption, revalidate the requester session;
6. revalidate the publisher session again;
7. require current same-workspace membership;
8. require exact publication-to-plan peer identity equality;
9. revalidate exact target `TransportIdentity` currentness;
10. only then call transactional `PeerConnectivityPlan::refresh_candidates(...)`.

`PeerConnectivityPlan` remains authoritative for candidate capacity, duplicate detection, plan-scoped `CandidateId` non-rebinding, observation reset and atomic mutation.

## Deliberate non-export boundary

The new source module is intentionally not added to the production `prw-remote-bridge` module graph.

`crates/prw-remote-bridge/tests/candidate_reachability_semantic_adapter.rs` includes that source by explicit test-only path so a future authorized test run can compile and exercise the semantic adapter without making it a production API or runtime entrypoint today.

This checkpoint therefore supersedes only the earlier C02e placement statement that the staged semantic type existed solely inside one integration-test file. The security semantics of the main C02e gate remain unchanged.

## Wire and freshness boundary remains fail-closed

No candidate wire message code, magic, protocol version, serialized device identity, serialized transport authority, generation counter, nonce, timestamp, or replay window is invented here.

A production wire adapter remains forbidden until a separately reviewed freshness/replay contract exists. In particular:

- generic TLS/control-frame validity is not PRW session authentication;
- Phase 129 `request_id` alone is not candidate-publication freshness;
- raw wire bytes must never be allowed to name an arbitrary target `DeviceId` that bypasses authenticated publisher identity derivation;
- replay of an otherwise unchanged old candidate publication must not be able to reset newer reachability state once production signaling exists.

## Mutation boundary

This checkpoint adds only:

- one unexported source semantic adapter;
- one source-level integration-test compile/behavior surface;
- this checkpoint contract;
- one static audit record.

It does not modify C02d, Cargo manifests, `Cargo.lock`, `main.rs`, Agent bootstrap/runtime wiring, control-transport framing, NAT traversal, relay, DNS, forwarding policy, deployment or system state.

## Validation boundary

No build, `cargo fmt`, Clippy, test, workflow dispatch, socket I/O or runtime execution is authorized by this checkpoint.

The staged source is therefore classified as authored/static-reviewed evidence only until the build/test gate is explicitly opened.

## Next safe seam

Before any candidate-update serialization is added, inspect existing repository replay/freshness precedents and lock a candidate-publication freshness semantic that cannot be inferred from IP/port or generic control-frame `request_id` alone.

If no existing precedent safely determines the exact production freshness state machine, leave candidate wire serialization unselected rather than inventing production values.
