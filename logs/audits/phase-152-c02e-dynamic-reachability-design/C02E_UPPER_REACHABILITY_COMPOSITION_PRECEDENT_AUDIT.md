# Phase 152 C02e — Upper Reachability Composition Precedent Static Audit

Status: `PASS_STATIC_PRECEDENT_REVIEW / PROGRESSIVE_UPPER_ORCHESTRATION_PATTERN_CONFIRMED / CONCRETE_OWNER_UNSELECTED / ONE_SHOT_COMPOSITION_SEAM_SAFE / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Review base head: `904b7c63c07fae1bde409459d89a65b12b498ddc`

Frozen C02d checkpoint: `857583b25ed1206317641a93fd8f927819c954d8`

## Evidence reviewed

- `contracts/LOCAL_BOUNDARY_SERVER_CONNECTION_STATE_CONTRACT.md` (Phase 051);
- `docs/architecture/PHASE_052_BOUNDED_SERVER_CONNECTION_LOOP.md`;
- `contracts/LINUX_AGENT_AUTHENTICATED_SESSION_BRIDGE_CONTRACT.md` (Phase 071);
- `contracts/LINUX_AGENT_CONNECTION_PROCESSING_BOUNDS_CONTRACT.md` (Phase 072);
- `contracts/LINUX_AGENT_WORKER_THREAD_LIFECYCLE_CONTRACT.md` (Phase 077);
- `docs/architecture/PHASE_080_LINUX_SCOPED_WORKER_REGISTRY.md`;
- `contracts/LINUX_AGENT_WORKER_CANCELLATION_CONTRACT.md` (Phase 081);
- `contracts/LINUX_AGENT_BOUNDED_SCHEDULING_CYCLE_CONTRACT.md` (Phase 085);
- `crates/prw-agent/src/linux_runtime_orchestration.rs` (Phase 092);
- current C02e candidate-publication provenance, publication-freshness, candidate-ID lifetime, transport-rotation, traversal-refresh lifecycle and traversal-composition ownership checkpoints.

## Findings

1. Repository precedent does not solve a missing composition owner by pushing unrelated ownership into an existing lower state machine.
2. Phase 051/052 keep aggregate/request processing caller-controlled and runtime-neutral.
3. Phase 071 performs state composition while explicitly deferring scheduling, shutdown, policy binding and bootstrap.
4. Phase 072 locks runtime obligations before activation.
5. Phase 077/080/081 separately establish lifetime, result-accounting and cancellation ownership before scheduler activation.
6. Phase 085 introduces only a caller-bounded scheduling composition and still defers the long-running outer runtime.
7. Phase 092 materializes a finite upper orchestrator only after the prerequisite ownership seams exist, while preserving lower APIs and still avoiding production outer-loop/bootstrap activation.
8. This sequence is directly applicable to C02e: `PeerConnectivityPlan` and `IceConnectivitySession` should retain their local authority; the missing plan-plus-traversal lifecycle coordination belongs to a later upper composition boundary.
9. No current repository evidence uniquely authorizes `prw-agent`, `prw-remote-bridge`, `prw-nat-traversal`, or a new crate as that final owner.
10. The safe next step is therefore to lock a one-shot, representation-neutral reachability composition transition before choosing concrete source/API placement or adding a dependency edge.

## Required C02e upper-owner invariants

The future upper boundary must preserve all of the following:

- no second/shadow connectivity-plan model;
- no duplicated ICE protocol authority;
- publication freshness remains verifier-owned and identity-bound;
- failed identity/workspace/freshness/candidate validation leaves plan, traversal lifecycle and freshness state unchanged;
- successful candidate refresh makes the old traversal session and queued observations stale before further observation admission;
- only observations attributable to the current traversal lifecycle may reach `PeerConnectivityPlan::set_observation(...)`;
- transport rotation invalidates the old plan/traversal lifecycle and requires replacement-plan semantics;
- no runtime/network behavior is inferred from the design boundary.

## Mutation surface

Added only:

- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02E_UPPER_REACHABILITY_COMPOSITION_PRECEDENT_REVIEW.md`;
- this static audit record.

No existing Rust source, Cargo manifest, Cargo lockfile, C02d source, Phase 141 source, production Agent/bootstrap source, network/runtime state, deployment state, or immutable Drive authority is modified by this checkpoint.

## Not executed

- build;
- `cargo fmt`;
- Clippy;
- tests;
- workflow dispatch;
- TCP/UDP I/O;
- STUN/ICE/TURN activation;
- QUIC connection/migration;
- Agent/bootstrap activation;
- deployment;
- signing;
- privileged/system mutation;
- PR creation/merge;
- Host Mirror synchronization.

## Result

`STATIC_PRECEDENT_REVIEW_PASS / REPOSITORY_PATTERN_REQUIRES_PROGRESSIVE_UPPER_COMPOSITION / LOWER_PLAN_AND_TRAVERSAL_AUTHORITIES_PRESERVED / FINAL_OWNER_NOT_YET_AUTHORIZED / ONE_SHOT_REACHABILITY_COMPOSITION_DESIGN_IS_NEXT_SAFE_SEAM / C02D_UNTOUCHED`
