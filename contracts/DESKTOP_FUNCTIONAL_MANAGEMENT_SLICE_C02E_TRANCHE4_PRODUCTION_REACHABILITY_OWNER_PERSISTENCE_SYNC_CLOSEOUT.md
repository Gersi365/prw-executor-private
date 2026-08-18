# Phase 152 C02e — Tranche 4 Production Reachability Owner / Persistence-Synchronization Closeout

Status: `PASS / PRODUCTION_UPPER_OWNER_SELECTED / PERSISTENCE_CAS_SEAM_LOCKED / PHASE141_NORMAL_DEPENDENCY / RECOVERY_FAIL_CLOSED / RETIREMENT_TOMBSTONE_ENFORCED / NO_PERSISTENCE_BACKEND_SELECTION / NO_DISTRIBUTED_RUNTIME_TENANCY / NO_WIRE_KIND / NO_NETWORK_IO / NO_AGENT_BOOTSTRAP_ACTIVATION`

Tranche 3 closeout head: `6168d500b25627190aa272ff34fdc186465ebc04`
Exact validated Tranche 4 head: `d8c2171ea3a07cc485ce0153f6687009eac80adb`
Authoritative validation evidence child: `1732119a8895188b105e7362492e293267d8b06d`
Validation report: `logs/audits/phase-152-c02e-dynamic-reachability-design/C02E_TRANCHE4_PRODUCTION_OWNER_VALIDATION_d8c2171ea3a07cc485ce0153f6687009eac80adb.txt`
Validation report blob: `63e3f37c71e1e8bc5f9215a439f740cf77afb01b`
Frozen C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Closed authority decision

Tranche 4 selects `prw-remote-bridge::reachability_owner::ProductionReachabilityOwner` as the concrete production upper composition owner for one exact peer reachability lifecycle.

This placement sits above the existing lower authorities it composes:

- authenticated session / registry currentness;
- authenticated candidate publication admission;
- verifier-owned candidate-publication freshness;
- `PeerConnectivityPlan` candidate/observation state;
- Phase 141 `IceConnectivitySession` Sans-I/O traversal state.

The owner does not replace any of those lower state machines with a parallel model.

## Production dependency edge

`prw-remote-bridge -> prw-nat-traversal` is now exactly one normal local Cargo dependency.

The previous test-only/dev edge is removed; no duplicate dev dependency remains. `Cargo.lock` remains byte-identical to the Tranche 3 locked state:

`becbd46de66354591afd3a4d755a9b4ba06f9c9c15045069b85e04a99525423a`

The new edge therefore changes production composition ownership without changing resolved package versions.

## Durable accepted-state boundary

The selected persistence seam is the typed `ReachabilityDurableStore` abstraction.

Its production semantic obligation is an exact-current linearizable compare-and-commit for one exact `DeviceId + TransportIdentity` peer lifecycle. The owner stages candidate mutation on a cloned plan, validates identity/workspace/transport/freshness/candidate state, issues a distinct verifier token, and only then asks the store to compare the expected current token and commit the complete replacement durable snapshot.

A successful durable commit is the accepted-state linearization point. Only after a definite `Committed` result does the in-process owner install the new plan/freshness state and invalidate the preceding traversal session.

A definite stale expected value or any ambiguous/unavailable storage outcome invalidates transient traversal authority and enters `RecoveryRequired`; the implementation never assumes an uncertain commit failed.

## Recovery and retirement

Recovery is authoritative-load only. Missing storage for an established lifecycle does not bootstrap a new lifecycle.

Durable `RecoveryRequired` remains fail-closed. Durable `Retired` remains a historical tombstone and cannot become `NewLifecycleEligible` merely because identical transport bytes later reappear.

After registry-authoritative transport rotation, the old exact peer lifecycle may be durably CAS-committed to `Retired`. This retirement does not create or authorize the replacement transport lifecycle.

## Traversal lifecycle ownership

The production owner holds at most one current Phase 141 `IceConnectivitySession`.

Successful candidate publication commit always makes the preceding traversal lifecycle stale. Replacement traversal construction is forward-only: if construction fails after accepted state committed, the new plan and freshness remain current and the system remains with no current traversal session rather than resurrecting old traversal state.

Observation polling/application remains inside one mutable owner operation and revalidates current transport identity before applying Phase 141 reachability evidence.

## Deliberately unselected production mechanisms

This tranche does not select or activate:

- a concrete database or persistence product;
- persistence serialization/schema encoding;
- replication technology;
- a concrete mutex/actor/channel/task primitive;
- distributed live-owner lease/fencing or runtime tenancy;
- freshness token wire message or resynchronization endpoint;
- socket/network adapter;
- STUN/TURN/ICE network I/O;
- Agent/bootstrap runtime wiring;
- deployment, PR, merge or signing.

The absence of distributed live-owner fencing is specifically why network/runtime activation remains closed even though accepted-state CAS is now selected.

## Executable validation

Exact-head validation on `d8c2171ea3a07cc485ce0153f6687009eac80adb` passed every gate:

- locked Cargo hash audit;
- dependency-kind audit;
- locked metadata;
- `cargo fmt --all -- --check`;
- focused production-owner test;
- focused `prw-remote-bridge` Clippy with `-D warnings`;
- focused `prw-connectivity` + `prw-nat-traversal` Clippy;
- full workspace Clippy;
- full workspace tests;
- full workspace build;
- tracked-target normalization;
- source/hash stability;
- final zero tracked diff.

The authoritative report records `FIRST_FAILURE=NONE` and `STATUS=PASS`.

## Corrective history retained as evidence

Two earlier exact-head validator failures remain preserved rather than rewritten:

1. `f34f59a0f8324026c784bf68f118f4399f8c07c3` failed only rustfmt; formatter output was applied mechanically.
2. `c30bef8b4059c4cabbc681384549e684ff7208f8` passed format and focused tests, then failed only Clippy `missing_const_for_fn`; `require_current` was changed mechanically to `const fn`.

Neither corrective changed the owner architecture, persistence semantics, dependency resolution, runtime boundary or network behavior.

## Final Tranche 4 authority state

Tranche 4 is closed as a validated production composition/persistence seam.

The next architecture gates remain separate:

1. freshness-token wire delivery / authenticated resynchronization contract; and/or
2. distributed live-owner tenancy/fencing plus eventual runtime/network activation.

Neither gate is opened by this closeout.