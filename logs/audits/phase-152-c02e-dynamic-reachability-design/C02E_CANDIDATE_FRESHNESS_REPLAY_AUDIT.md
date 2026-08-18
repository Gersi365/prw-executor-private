# Phase 152 C02e — Candidate Publication Freshness / Replay Static Audit

Status: `PASS_STATIC_DESIGN_REVIEW / VERIFIER_OWNED_FRESHNESS_REQUIRED / EXACT_REPRESENTATION_UNSELECTED / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Checkpoint commit: `c0850b7219b39d782cd008770bba78f6e06e3738`

Checkpoint base: `2e684ed138baf27162ae7989d394d62d049842b7`

Frozen C02d checkpoint: `857583b25ed1206317641a93fd8f927819c954d8`

## Evidence reviewed

- Phase 114 enrollment proof challenge state and compare-and-consume semantics;
- Phase 128 authenticated device-session challenge state and single-use consumption;
- `prw-device-identity` verification ordering: replay/context validation -> canonical identity-bound message -> cryptographic verification -> consume;
- Phase 130 exact-current `TransportIdentity` compare-and-rotate behavior;
- Phase 132 upload sequencing against verifier-observed committed offset;
- Phase 129 generic control-frame request correlation semantics;
- current C02e authenticated candidate semantic adapter, candidate-ID lifetime freshness and transport-rotation replacement-plan lifecycle.

## Static conclusions

1. Repository precedent consistently keeps freshness/currentness authority on the verifier/state-owner side rather than trusting caller-selected endpoint data.
2. Stale expected state is rejected before mutation in registry rotation and file-transfer sequencing.
3. Enrollment/session replay protection binds freshness state to immutable authenticated context and consumes/advances only after complete verification.
4. Invalid enrollment/session proof does not become a successful freshness transition.
5. Phase 129 `request_id` is correlation only; no current contract grants it candidate-publication uniqueness, ordering or replay authority.
6. No repository precedent fixes the exact candidate-publication freshness representation, counter width, initial value, nonce format, timestamp window, replay window, durable storage model or wire field.
7. Selecting any of those now would invent production protocol values rather than reuse an authoritative precedent.

## Locked semantic requirement

A future candidate signaling adapter must provide verifier-owned exact-current freshness state bound to the authenticated candidate publication identity/lifecycle scope.

The complete accepted transition must be logically atomic:

`current authenticated peer + expected current freshness + valid bounded candidate set -> refreshed candidate plan + advanced freshness state`

A stale/duplicate/replayed freshness presentation must fail before endpoint mutation.

A candidate validation failure must not advance verifier freshness state.

Two updates racing from the same previous freshness state must not both commit successfully.

## Deliberate non-selection

No Rust freshness token type or runtime authority was added.

No candidate wire schema, generation integer, nonce, timestamp, replay window, persistence schema, message kind or Phase 129 request-id reinterpretation was selected.

`crates/prw-remote-bridge/src/candidate_reachability.rs` remains unexported and freshness-agnostic by design; it cannot be treated as a production replay-safe signaling adapter.

## Mutation surface

Added only:

- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02E_CANDIDATE_FRESHNESS_REPLAY_CHECKPOINT.md`;
- this static audit record.

No existing Rust source, Cargo manifest, lockfile, NAT traversal source, control transport source, Agent/bootstrap source, C02d source or production system state was modified by this checkpoint.

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
- Host Mirror synchronization.

## Result

`STATIC_DESIGN_REVIEW_PASS / FRESHNESS_MUST_BE_VERIFIER_OWNED_AND_IDENTITY_BOUND / STALE_OR_DUPLICATE_PUBLICATION_MUST_FAIL_BEFORE_MUTATION / SUCCESSFUL_REFRESH_AND_FRESHNESS_ADVANCE_REQUIRE_ONE_LOGICAL_TRANSITION / REPRESENTATION_AND_WIRE_REMAIN_UNSELECTED / C02D_UNTOUCHED`
