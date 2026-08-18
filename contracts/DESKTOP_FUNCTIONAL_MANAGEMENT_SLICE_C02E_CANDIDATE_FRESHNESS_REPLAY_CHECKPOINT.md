# Phase 152 C02e — Candidate Publication Freshness / Replay Checkpoint

Status: `DESIGN_LOCK / VERIFIER_OWNED_FRESHNESS_REQUIRED / EXACT_REPRESENTATION_UNSELECTED / WIRE_UNSELECTED / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Base C02e head: `2e684ed138baf27162ae7989d394d62d049842b7`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

This checkpoint locks the freshness/replay semantics required before the source-only authenticated candidate publication adapter may ever be serialized or connected to a production control-plane path.

C02e already proves provenance/currentness and transactional endpoint refresh. Those checks are insufficient by themselves if an older, otherwise authentic publication can be replayed after a newer candidate set has been accepted.

## Repository precedent reviewed

The repository provides several compatible fail-closed state-transition patterns:

1. Phase 114/128 enrollment and session authentication keep verifier-owned current challenge state, bind submissions to immutable authenticated identity context, reject stale/replayed context before effect, and consume verified state exactly once.
2. The device-identity verifier orders challenge/replay validation before cryptographic verification and single-use consumption; invalid verification does not consume freshness state.
3. Phase 130 transport rotation uses compare-and-rotate against the exact registry-current `TransportIdentity`; stale expected state cannot mutate the registry.
4. Phase 132 upload transactions require the caller's offset to equal verifier-observed committed state before appending; out-of-order/stale offsets fail before the next state is acknowledged.
5. Phase 129 control-frame `request_id` is only a non-zero correlation identifier. No current contract makes it a uniqueness, monotonicity or replay-prevention authority for candidate publications.

These precedents determine the security shape, but they do not determine a candidate-publication counter width, nonce format, timestamp window, persistence backend, initial value or wire field.

## Locked freshness semantics

Any future candidate-publication transport/runtime adapter must maintain verifier-owned freshness state for the candidate state it accepts.

At minimum, admission must preserve this order:

1. authenticate/revalidate the publisher session against current registry state;
2. derive and revalidate the exact current publisher `PeerConnectivityIdentity` (`DeviceId + TransportIdentity`);
3. authenticate/revalidate the requester and same-workspace relationship when consuming for a requester;
4. require exact publication-to-target-plan peer identity equality;
5. compare the publication's future freshness proof/token against verifier-owned current freshness state for the exact publication identity/lifecycle scope;
6. reject stale, duplicate or replayed publication state before candidate mutation;
7. transactionally validate the complete candidate vector, including candidate-ID lifetime freshness;
8. commit candidate refresh and freshness-state advance as one logical successful transition.

A failure in identity, workspace, freshness or candidate validation must not make an older/stale publication become current and must not partially mutate the target connectivity plan.

## Atomicity rule

Freshness state is authorization-critical coordination state.

A later implementation must provide equivalent compare-and-advance semantics so that two publications racing from the same previous freshness state cannot both become current.

The accepted transition must behave conceptually as:

`expected current freshness + authenticated exact peer + complete valid candidate set -> one new current candidate state + one advanced freshness state`

If the expected freshness is stale, or candidate validation fails, neither candidate state nor freshness state may advance.

This checkpoint does not choose a database, lock, transaction primitive or distributed coordination mechanism.

## Identity and lifecycle scope

Freshness must be bound to authenticated candidate state, never to an IP address or port.

It must not be shared in a way that allows state from one logical/transport peer identity to authorize candidate state for another.

Transport-identity rotation remains the separate C02e plan-lifecycle transition:

- old `TransportIdentity` -> old plan and old publication lifecycle stale;
- new `TransportIdentity` -> replacement plan / new current peer identity;
- no numeric freshness continuity, candidate-ID continuity or endpoint continuity may itself authorize the replacement identity.

The exact rule for preserving or resetting a future freshness token across authenticated session renewal, durable service restart or multi-node control-plane failover is not determined by current repository precedent and remains unselected.

## Deliberately unselected representation

C02e does **not** select any of the following:

- a generation/counter integer type or width;
- an initial generation value;
- random nonce length or encoding;
- timestamp/clock window;
- replay-window size;
- candidate payload magic/version;
- a control message-kind allocation;
- reuse of Phase 129 `request_id` as freshness;
- persistence/durability format;
- restart/failover recovery semantics;
- a production wire codec.

Choosing any of those without an existing reviewed contract would invent production protocol values and is outside this checkpoint.

## Relationship to the source-only semantic adapter

`crates/prw-remote-bridge/src/candidate_reachability.rs` remains unexported and intentionally carries no freshness field.

That source continues to specify authenticated publisher derivation, current requester/publisher/workspace/transport admission and exact target correlation only.

It must not be exported or connected to production candidate signaling as though those semantics alone provide replay protection.

A later adapter may wrap the existing semantic publication only after a reviewed freshness representation and atomic verifier state boundary exist.

## Security invariants

C02e must not:

- accept an older authentic candidate publication after newer candidate state became current;
- treat duplicate delivery as a second successful state transition;
- infer freshness from IP address, port, `CandidateId`, TLS connection success or generic frame `request_id`;
- advance freshness after a rejected candidate vector;
- advance candidate state without the corresponding successful freshness transition;
- permit freshness state from another `DeviceId` or `TransportIdentity` to authorize this publication;
- weaken the replacement-plan rule after transport rotation;
- invent a production counter/nonce/timestamp/wire format in this source/design checkpoint;
- export or activate candidate signaling, sockets, ICE/STUN/TURN, QUIC runtime or Agent/bootstrap wiring.

## Validation boundary

This checkpoint is static design evidence only.

No source runtime API is added by this checkpoint and no build, `cargo fmt`, Clippy, tests, workflow dispatch, network I/O or production mutation is authorized or performed.

## Next safe seam

With freshness semantics locked but representation intentionally unselected, the next safe C02e work is static integration review of the complete source-only chain:

`authenticated publisher -> current registry identity -> bounded candidate publication -> freshness gate (required, representation unselected) -> same-workspace requester/target admission -> transactional lifetime-fresh candidate refresh -> Phase 141 reachability correlation`

Any production wire or runtime activation remains closed until the freshness representation and transaction authority are separately reviewed.
