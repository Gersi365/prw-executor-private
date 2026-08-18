# C02e Tranche 5 — Freshness-Token Wire / Authenticated Resynchronization Audit

Status: `IMPLEMENTATION_STAGED / RUSTFMT_CORRECTED / EXACT_HEAD_REVALIDATION_TRIGGERED`

Tranche 4 closeout head: `eea6b8743eebf21002ae173dfcfd5cbbf93378a8`
Frozen C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Audited precedents

The tranche was derived from:

- Tranche 3 freshness representation/persistence/recovery, especially the explicit authenticated same-token resynchronization rule;
- Tranche 4 production owner and `ReachabilityDurableStore` exact-current durable seam;
- Phase 140 `prw-remote-transport` PRWM frame contract;
- Phase 143 remote bridge ordering, which proves transport establishment is not application-session/registry authority.

## Placement decision

The wire codec belongs in `prw-remote-bridge`, adjacent to the C02e freshness and owner modules. It reuses Phase 140 PRWM `Request`, `Response` and `Error` kinds rather than allocating a new transport kind.

The payload magic is `PRWF`, version 1.0. It is separate from the Phase 143 capability-command `PRWC` namespace because freshness synchronization is reachability protocol coordination rather than a file/terminal/forwarding capability operation.

## Identity minimization

The resync request carries only the exact 32-byte `TransportIdentity`. It does not accept `DeviceId`, workspace, user, session ID, endpoint or token baseline from the payload.

`DeviceId` is derived from the authenticated current session and revalidated through `WorkspaceDeviceRegistry`. Current transport identity is checked before durable storage is read.

## Durable resynchronization decision

The implementation calls the existing `ReachabilityDurableStore::load_current` after currentness validation and returns a token only from exact-peer `NewLifecycleEligible` or `Established` durable state.

It performs no compare-and-commit and never invokes the verifier token source. Repeated resync calls re-read authoritative durable state instead of trusting a cached token.

`RecoveryRequired`, `Retired`, missing durable state, snapshot peer mismatch and persistence ambiguity do not disclose token material.

## Delivery provenance

Bootstrap delivery is constructible only from an existing `NewLifecycleEligible(token)` record.

Accepted-publication delivery is constructible from `ReachabilityCommitOutcome`, which is emitted by the Tranche 4 production owner only after definite durable commit. This keeps success-token delivery downstream of the accepted-state linearization point.

## Error/privacy boundary

Wire failures use bounded stable codes. Registry-internal currentness detail is collapsed to `CurrentnessRejected`. Request IDs remain transport correlation only.

## Deliberate non-activation

No candidate-vector wire encoding, distributed owner fencing, database/schema, async runtime, socket, stream, STUN/TURN/ICE traffic, Agent bootstrap or deployment is selected or activated.

## Planned executable checks

The focused Tranche 5 test will prove:

1. exact PRWF request/delivery/failure round trips and outer PRWM kind agreement;
2. malformed/reserved/all-zero/wrong-kind inputs fail closed;
3. bootstrap delivery requires authoritative bootstrap state;
4. authenticated resync returns the exact durable current token without commit;
5. each resync re-reads durable state and observes a newer committed token;
6. wrong/stale transport currentness fails before durable lookup;
7. `RecoveryRequired`, `Retired` and missing durable state disclose no token;
8. full locked workspace format/Clippy/tests/build and tracked-drift checks remain clean.

## Rustfmt corrective and exact-head revalidation

The staged source/test candidate was normalized by the repository-pinned formatter using a one-shot workflow that ran `cargo fmt --all`, rejected any diff outside the two Tranche 5 Rust files, verified the frozen Cargo.lock SHA-256, guarded the branch head before push and self-deleted.

Rustfmt corrective commit: `63db80674e30f85b79d40fd07690c7f332afbf50`.

The corrective patch contains formatting only: line wrapping, brace layout and whitespace. It changes no PRWF field, operation, failure code, authentication/currentness ordering, durable-read semantics, token disclosure rule, dependency graph, Cargo.lock, runtime boundary or network activation.

Because commits pushed by a workflow token do not recursively trigger the validator workflow, this audit-only user commit intentionally retriggers the existing exact-head Tranche 5 validator without changing source semantics. Final classification remains pending the new authoritative validation evidence child/report.
