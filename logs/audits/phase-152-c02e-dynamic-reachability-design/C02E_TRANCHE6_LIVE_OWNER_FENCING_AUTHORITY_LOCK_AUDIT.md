# C02e Tranche 6 — Live-Owner Fencing Authority Lock Audit

Status: `PASS_STATIC_DESIGN_LOCK / EXACT_SCOPE_VERIFIED / NO_SOURCE_RUNTIME_MUTATION / NO_BUILD_EXECUTION`

Starting head: `78daf5b02ed359762eba0cfb5afcd0effbc86bc6`

Design-lock commit: `a20e323a80c6f3da69b6d697d50035a0adbdbb4a`

Frozen predecessor C02d: `857583b25ed1206317641a93fd8f927819c954d8`

## Authoritative state verified before mutation

GitHub repository:

`powercode2026/prw-executor-private`

Active branch:

`phase-152-c02e-dynamic-reachability-design`

Pre-mutation branch head was verified as:

`78daf5b02ed359762eba0cfb5afcd0effbc86bc6`

The branch was exactly `165` commits ahead of frozen C02d, `0` behind, with merge base equal to the frozen C02d head.

Google Drive mutable evidence `C02E_BRANCH_STATUS.md` independently reported the same Tranche 5 closeout head and frozen C02d predecessor.

PR state remained unchanged:

- PR #1 open, draft, unmerged;
- PR #2 closed, draft, unmerged.

## Source evidence inspected

The review inspected the current production upper owner in:

`crates/prw-remote-bridge/src/reachability_owner.rs`

The source explicitly states:

- `&mut self` serializes one in-process owner operation;
- `ReachabilityDurableStore::compare_and_commit(...)` is the cross-writer accepted-state arbitration seam;
- traversal observations are transient local state;
- persistence ambiguity enters fail-closed recovery.

The review also inspected the Tranche 5 closeout and Drive status, both of which explicitly preserve distributed live-owner tenancy/fencing as an unselected boundary after freshness-wire/resynchronization closure.

## Gap confirmed

The current source can prevent conflicting accepted-state commits using expected-current CAS, but it does not prove exclusive transient runtime ownership across processes/replicas.

Two owners may load the same durable accepted snapshot and independently hold local traversal state unless a separate runtime-owner authority is added.

Therefore accepted-state CAS and live-owner tenancy are correctly classified as distinct authorities.

## Decision locked

The new contract locks the following without selecting a backend:

1. live-owner authority is keyed by exact `DeviceId + TransportIdentity`;
2. endpoint/IP/candidate/session/request/freshness identifiers are not tenancy identity;
3. live-owner fencing generation is authority-issued and strictly newer for each replacement grant in the same exact-peer lifecycle;
4. durable ordering must prevent reuse of an older generation after authority restart/failover;
5. a newer grant invalidates all older grants for that exact peer;
6. cooperative release is not required for safety;
7. current live-owner authority is required before traversal provisioning, observation application and future network/runtime side effects;
8. a single pre-check is insufficient for eventual real I/O; the future concrete side-effect boundary must reject stale generations;
9. authority loss/ambiguity invalidates transient runtime authority and fails closed;
10. `CandidatePublicationFreshnessToken` remains a distinct accepted-publication ordering type and cannot be reused as a live-owner fence.

## Mutation boundary

The exact first-step delta from `78daf5b...` to `a20e323...` is:

- `1` commit ahead;
- `0` behind;
- exactly `1` changed file;
- `209` additions;
- `0` deletions.

Changed path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02E_TRANCHE6_LIVE_OWNER_FENCING_AUTHORITY_LOCK.md`

No Rust source, Cargo manifest, `Cargo.lock`, workflow, Agent/bootstrap, systemd, Android, desktop, deployment, signing or privileged-system source changed in that design-lock commit.

## Execution boundary

No build, formatting, Clippy, tests, workflow dispatch, socket I/O, STUN/TURN/ICE traffic, Agent restart, service-manager action, signing, deployment or privileged mutation was performed for this static design lock.

## Next bounded source tranche

The next safe source step is an additive pure `prw-remote-bridge` fencing seam plus reference tests. It must remain:

- provider/backend neutral;
- exact-peer bound;
- type-distinct from publication freshness;
- free of clocks, sockets, tasks and runtime activation;
- unable to claim production stale-side-effect blocking until a concrete runtime adapter honors the fence.

## Result

`PASS / C02E_TRANCHE6_LIVE_OWNER_FENCING_DESIGN_LOCKED / C02D_UNTOUCHED / PRODUCTION_NETWORK_RUNTIME_STILL_CLOSED`
