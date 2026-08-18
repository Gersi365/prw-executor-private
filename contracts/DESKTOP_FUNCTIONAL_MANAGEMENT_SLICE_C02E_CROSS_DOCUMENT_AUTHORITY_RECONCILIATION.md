# Phase 152 C02e — Cross-Document Authority Reconciliation

Status: `DESIGN_AUTHORITY_RECONCILIATION / HISTORICAL_CHECKPOINTS_PRESERVED / LATER_LOCKS_TAKE_PRECEDENCE_WHERE_EXPLICITLY_SUPERSEDING / ADMISSION_THEN_FRESHNESS_ORDER_CURRENT / SAME_PEER_SESSION_RENEWAL_CONTINUES_FRESHNESS / EXISTING_PEER_STATE_LOSS_FAILS_CLOSED / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Reconciliation base head: `f1e106bf9496050778d6e6479128e5f8c8a497bd`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

C02e intentionally accumulated narrow design checkpoints over time. Earlier files accurately record what was known or deliberately unselected at their own review heads, while later checkpoints resolve some of those previously open questions.

A cross-document read at the current head found several earlier statements that are valid historical evidence but are no longer the current authority if read without their chronology. This reconciliation preserves those historical files unchanged and records explicit precedence for the superseded points.

It does not alter C02e semantics, choose a production freshness representation, add a wire protocol, select a production composition owner, change Cargo state, or activate runtime/network behavior.

## Authority rule

C02e checkpoint files remain immutable historical evidence of the state reviewed at their declared base/review heads.

When a later C02e checkpoint explicitly resolves a question that an earlier checkpoint described as unselected, or explicitly corrects an ordering/placement statement, the later lock is the current authority for that question. The earlier wording remains evidence of the earlier state and must not be treated as a competing current contract.

The current consolidated source/design authority is summarized by:

- `DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02E_SOURCE_DESIGN_GAP_CLOSURE_REVIEW.md`;
- `DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02E_CANDIDATE_PUBLICATION_FRESHNESS_AUTHORITY_PLACEMENT.md`;
- `DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02E_CANDIDATE_FRESHNESS_BOOTSTRAP_REBASELINE_LIFECYCLE.md`;
- the later test-only/static corrective checkpoints that implement/reference those locks without selecting production representation.

Future implementation work must follow the re-derivation guard in the source/design gap-closure review rather than reopening resolved semantics merely because a concrete mechanism is later chosen.

## Reconciled point 1 — admission precedes freshness

Some earlier narrative chains place the mandatory publication-freshness gate before requester/workspace/target admission. In particular, the historical `CANDIDATE_FRESHNESS_REPLAY_CHECKPOINT` "Next safe seam" chain and the historical `SOURCE_ONLY_INTEGRATION_REVIEW` top-level chain use that ordering.

Those narrative chains are superseded on ordering by the later freshness-authority/bootstrap work and the source/design gap-closure review.

The current locked order is:

1. revalidate current requester and publisher authenticated identity;
2. require current same-workspace authorization;
3. require exact publication/target-plan peer identity and current target `TransportIdentity`;
4. compare the exact expected verifier-owned publication freshness for the publisher peer lifecycle;
5. stage/validate the complete candidate-plan transition;
6. commit accepted candidate state and freshness advance as one logical transition;
7. make the previous traversal lifecycle stale as part of the accepted lifecycle transition.

Equivalently:

`current requester/publisher/workspace/target/transport admission`

`-> exact expected verifier freshness`

`-> complete candidate validation`

`-> accepted authoritative commit`

Identity/workspace/current-target failure must therefore occur before freshness is disclosed, consumed or compared as an authorization effect.

## Reconciled point 2 — same-peer session renewal does not reset freshness

The historical `CANDIDATE_FRESHNESS_REPLAY_CHECKPOINT` states that the rule for preserving/resetting freshness across authenticated session renewal was then unselected.

That question is resolved by `CANDIDATE_PUBLICATION_FRESHNESS_AUTHORITY_PLACEMENT`.

Current authority is:

- publication freshness is publisher peer-lifecycle state keyed to the exact current `DeviceId + TransportIdentity`;
- requester identity is authorization context, not a freshness namespace;
- a new authenticated `SessionId`, reconnect, control-transport reconnect or reauthentication for the same current peer lifecycle does **not** reset publication freshness;
- ordinary session replacement cannot make an older authentic candidate publication current again.

Any earlier statement that session-renewal preservation/reset is still wholly unselected is historical and superseded by this lock.

## Reconciled point 3 — restart/state-loss posture is fail-closed

Earlier checkpoints list restart/failover recovery semantics as unselected. The exact recovery representation, persistence architecture, replication mechanism and same-identity recovery/re-baselining protocol do remain unselected.

However, the security posture is no longer unselected.

`CANDIDATE_PUBLICATION_FRESHNESS_AUTHORITY_PLACEMENT` and `CANDIDATE_FRESHNESS_BOOTSTRAP_REBASELINE_LIFECYCLE` lock that:

- service restart, process replacement or failover must not silently reset an established peer lifecycle to an initial freshness state;
- missing/unrecoverable freshness for a peer lifecycle known or conservatively assumed to be established is `RecoveryRequired`, not `NewLifecycleEligible`;
- candidate publication fails closed while exact current freshness cannot be proven/recovered;
- a publisher/requester cannot claim that missing verifier state means "first publication";
- automatic same-identity re-baselining remains forbidden without a separately reviewed recovery contract.

Thus the **mechanism** remains unselected, while the **fail-closed behavior** is current authority.

## Reconciled point 4 — new-lifecycle bootstrap is verifier-owned

Earlier freshness checkpoints intentionally did not choose an initial generation/value or restart rule.

The later bootstrap lifecycle checkpoint resolves the semantic initialization boundary without choosing a concrete representation:

- only an authoritatively legitimate new `DeviceId + TransportIdentity` peer lifecycle may enter bootstrap;
- verifier/upper-owner state establishes the bootstrap baseline;
- publisher/requester does not choose the verifier baseline;
- invalid first candidate validation does not consume bootstrap eligibility/state;
- at most one racing first publication from one current bootstrap state may commit;
- transport rotation may create a legitimate replacement peer lifecycle, but old freshness/candidate/traversal/session state does not authorize the replacement identity.

Concrete bootstrap token/proof/wire/persistence values remain unselected.

## Reconciled point 5 — source placement evolved without production activation

The early `DYNAMIC_REACHABILITY_GATE` describes the staged authenticated publication type as existing only in integration-test source.

The later `CANDIDATE_SEMANTIC_ADAPTER_CHECKPOINT` and source-only integration review supersede that statement only as source placement: the semantic adapter now exists in the private unexported `crates/prw-remote-bridge/src/candidate_reachability.rs` source seam and is exercised by integration tests.

The security/runtime boundary remains unchanged:

- the module is not a production wire protocol;
- candidate signaling remains unactivated;
- no production network/runtime ownership is implied by source placement;
- exact freshness representation remains absent from the production semantic object.

## Reconciled point 6 — earlier "next safe step" statements are historical

Earlier checkpoint files contain forward-looking "Next safe seam/step" text based on the authority available at those review heads. Later C02e work safely closed additional runtime-neutral semantics without choosing a production freshness representation by using explicit precedent reviews, test-only opaque reference models and fail-closed lifecycle locks.

Those earlier next-step statements are therefore historical planning evidence, not permanent gates that override later reviewed work.

At the current C02e authority boundary, the remaining gates are the ones recorded by the source/design gap-closure and implementation-validation-tranche reviews:

1. concrete publication-freshness representation/protocol/persistence/recovery mechanism;
2. actual Phase 141 dev-dependency/Cargo materialization and validation;
3. production upper composition owner, synchronization, cancellation, runtime and network adapter;
4. executable implementation validation.

## Preserved invariants

This reconciliation does not change the already locked C02e invariants:

- logical `DeviceId` is not endpoint identity;
- `TransportIdentity` is separately rotatable;
- transient candidate endpoints are not authorization identity;
- candidate refresh is transactional;
- candidate IDs are plan-lifetime non-rebindable/fresh;
- successful full refresh invalidates the previous traversal lifecycle;
- publication freshness and traversal currentness are distinct;
- rejected admission does not advance freshness;
- stale freshness does not mutate candidate state;
- rejected candidate validation does not consume/advance freshness;
- accepted publication state is not rolled back merely because later replacement-traversal construction fails;
- stale traversal state is never revived.

## Deliberately unselected and unchanged

This reconciliation does **not** select:

- freshness counter/nonce/token/timestamp representation;
- bootstrap proof/canonical message/wire field;
- persistence schema or transaction primitive;
- multi-node consensus/replication/recovery mechanism;
- same-identity recovery/re-baselining protocol;
- production composition-owner crate/module;
- mutex/async/task/cancellation/queue primitive;
- socket/network adapter;
- Phase 141 production dependency edge;
- deployment/bootstrap integration.

## Validation boundary

Documentation reconciliation only.

No Cargo resolution, `cargo fmt`, compiler/type check, Clippy, tests, build, workflow dispatch, TCP/UDP I/O, STUN/ICE/TURN activation, QUIC activity, production runtime/bootstrap wiring, deployment, signing, privileged/system mutation, PR creation/merge or Host Mirror synchronization is performed by this checkpoint.

## Result

`C02E_CROSS_DOCUMENT_AUTHORITY_RECONCILED / HISTORICAL_EVIDENCE_PRESERVED / CURRENT_ORDER_ADMISSION_THEN_FRESHNESS_THEN_CANDIDATE_VALIDATION_THEN_COMMIT / SAME_PEER_SESSION_RENEWAL_CONTINUES_FRESHNESS / EXISTING_PEER_STATE_LOSS_RECOVERY_REQUIRED_FAIL_CLOSED / PRODUCTION_REPRESENTATION_RUNTIME_AND_EXECUTION_GATES_STILL_CLOSED / C02D_UNTOUCHED`
