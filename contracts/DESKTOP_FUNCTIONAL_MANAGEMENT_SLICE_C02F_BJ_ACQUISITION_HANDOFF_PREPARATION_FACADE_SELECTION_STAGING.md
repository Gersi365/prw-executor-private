# Phase 152 C02f-BJ — Acquisition Handoff Preparation Facade Selection Staging

## Status

Documentation-only selection checkpoint after validated C02f-BI.

C02f-BJ selects a narrow `prw-control-plane` facade that will prepare one acquisition attempt from only an exact `PeerConnectivityIdentity`, while keeping fence-sequence, bootstrap, live-owner transaction and reconciliation provider modules private. It does not add Rust source, dependencies, provider I/O, randomness generation, fence allocation, runtime activation, deployment or merge.

## Exact prerequisite

Validated C02f-BI:

- head `6cdfcc28bd18678267d5cfb0a328eb8c9a1538ab`;
- tree `41b8043c87c6ff459842a5cda9494fa327aa11de`;
- gate `C02F_BI_FIRST_OWNER_LIVE_OWNER_BOOTSTRAP_SEMANTICS_SELECTED`.

C02f-BG remains the broader acquisition-readiness checkpoint, C02f-BH owns attempt-ID generation selection, and C02f-BI owns first-owner bootstrap semantics.

## Purpose of the facade

Current acquisition primitives are deliberately split across private control-plane modules. Publishing those modules wholesale would expose provider internals and permit callers to assemble unsafe or contradictory flows.

C02f-BJ therefore selects one purpose-built control-plane preparation boundary that owns the exact orchestration needed before provider execution evidence reaches semantic mapping.

The bridge caller must continue to supply only the exact peer identity.

## Public input boundary

The selected preparation call accepts only:

`&PeerConnectivityIdentity`

It must not accept request-controlled:

- live-owner predecessor or absence assertion;
- fence;
- recovery epoch;
- sequence;
- PRWF head observation;
- sequence-allocation attempt ID;
- live-owner authority-attempt ID;
- transaction plan;
- bootstrap mode selector;
- retry/reissue budget;
- provider outcome;
- semantic grant.

## Provider construction boundary

The future facade may be constructed from one already-created `etcd_client::KvClient` and must not select endpoints or call `Client::connect`.

The same provider context must back both fence-sequence and live-owner etcd operations for one preparation instance. The facade must not require callers to provide independently constructed sequence and live-owner stores that could accidentally point at different authority backends.

The currently private store modules remain private.

Endpoint selection, TLS/auth/RBAC, credential lookup and runtime ownership remain later boundaries.

## Attempt-ID generation ownership

The facade owns production attempt-ID generation internally according to C02f-BH.

Future source materialization may add the repository-selected `aws-lc-rs` dependency to `prw-control-plane` and use independent `SystemRandom` fills for the two typed domains.

No raw attempt-ID input may be added to the public facade.

## Selected preparation order

For one logical `prepare(peer)` operation, C02f-BJ selects this order:

1. perform one exact linearizable live-owner Get for `peer` and retain the authoritative result as either `Some(observation)` or `None`;
2. perform one exact linearizable read of the initialized PRWF sequence head;
3. generate one fresh C02f-BH `SequenceAllocationAttemptId`;
4. deterministically plan one AJ fence-sequence allocation from that exact head;
5. resolve that exact retained allocation through the existing AQ bounded reconciliation state machine;
6. if AQ resolves `Superseded`, stop this logical preparation as a non-ready/contended terminal result; do not create another allocation attempt inside the same call;
7. if AQ resolves `Committed`, retain that exact allocation evidence and compose its canonical 64/64 non-zero live-owner fence;
8. generate one fresh, independent C02f-BH `AuthorityAttemptId`;
9. branch only on the exact live-owner result retained in step 1:
   - `Some(observation)` => use existing AR planning plus AS handoff retention;
   - `None` => use the dedicated BI first-owner bootstrap plan/evidence path;
10. return only a bounded prepared outcome/evidence capsule to the later execution layer.

No provider write other than the already-selected AQ allocation protocol belongs to preparation source until its exact source checkpoint authorizes it. In particular, BJ itself is documentation only.

## Why live-owner observation comes first

The first linearizable live-owner read establishes the exact predecessor-or-absence context that the later live-owner CAS must defend.

Any state change after that read is handled by the selected transaction compare:

- replacement path: exact mod-revision + exact-value CAS;
- bootstrap path: exact-key `version == 0` CAS.

The facade must not silently refresh the live-owner observation after sequence allocation and then bind a different context without a separately selected replanning policy.

## Fence-sequence allocation outcome

A committed AQ allocation authorizes one canonical fence but does not itself grant live-owner authority.

If AQ resolves `Superseded`, this logical preparation attempt terminates without a prepared live-owner mutation. There is no loop that automatically generates a new sequence-allocation attempt ID and tries again within the same preparation call.

A future semantic composition may map that bounded non-ready outcome to `Contended`; it must never map it to `Granted`.

## Committed-but-unused sequence rule

Once AQ resolves an allocation `Committed`, that sequence/fence is consumed even if subsequent preparation or live-owner execution fails, contends or becomes unavailable.

The facade must never reuse a committed allocation for a later logical acquisition operation.

This includes failure after:

- live-owner attempt-ID generation;
- AR/AS deterministic preparation;
- BI bootstrap preparation;
- later live-owner compare failure;
- indeterminate live-owner mutation reconciliation.

Skipped sequence values are acceptable; fence reuse is not.

## Replacement preparation branch

For `Some(observation)`, the facade must use existing validated primitives rather than reimplement them:

1. retain the exact committed AQ allocation;
2. use the exact BH-generated live-owner `AuthorityAttemptId`;
3. call existing AR `plan_live_owner_acquisition_from_allocation(...)` with the retained observation, exact peer, committed allocation and attempt ID;
4. call existing AS `retain_live_owner_acquisition_handoff(...)` with that same observation and exact AR plan;
5. return the retained AS handoff as the replacement prepared evidence.

No alternate replacement transaction shape is selected by BJ.

## First-owner preparation branch

For authoritative `None`, the facade must use BI semantics rather than manufacturing a predecessor.

The prepared bootstrap evidence must retain:

- exact peer;
- exact committed AQ allocation;
- exact canonical fence;
- exact fresh `AuthorityAttemptId`;
- exact intended canonical `Current` PRWL successor;
- exact create-only `version == 0` plan;
- explicit absence provenance.

The bootstrap evidence is distinct from existing AS replacement evidence.

## Selected public prepared outcome shape

C02f-BJ selects a narrow provider-neutral prepared result conceptually equivalent to:

```rust
pub enum ReachabilityLiveOwnerPreparedAcquisition {
    Replacement(FenceSequenceLiveOwnerAcquisitionHandoff),
    FirstOwner(ReachabilityLiveOwnerFirstOwnerHandoff),
    Superseded,
}
```

Exact source names may be adjusted mechanically in the source checkpoint, but semantics must remain unchanged.

`Replacement` and `FirstOwner` contain provider-owned retained evidence only. `Superseded` is a non-grant preparation terminal state caused by the exact AQ allocation losing authority.

No variant carries endpoint, credentials, client handles, random provider state or semantic grants.

## Preparation errors

Preparation fails closed for:

- live-owner read failure or invalid state;
- PRWF head read failure/missing initialization;
- sequence-attempt randomness failure or zero rejection;
- AJ planning failure;
- AQ provider/domain error or reissue-limit exhaustion;
- canonical fence composition failure;
- live-owner attempt randomness failure or zero rejection;
- AR replacement planning failure;
- AS evidence-retention mismatch;
- BI bootstrap plan/evidence construction contradiction.

A later bridge mapper must map these failures to `ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`, except an actual canonical fence-representation exhaustion case may use the existing `FenceExhausted` semantic where already selected.

Preparation error never means `Granted`.

## Private-module boundary

C02f-BJ rejects broad public exposure of:

- `fence_sequence`;
- `fence_sequence_allocation_etcd`;
- `fence_sequence_allocation_orchestrator`;
- `fence_sequence_live_owner_bridge`;
- `fence_sequence_live_owner_handoff`;
- future bootstrap provider internals;
- recovery provider internals.

Only the narrow facade and evidence types required by the later execution/semantic layer may become public.

## Source-materialization scope selected for later checkpoint

A later source checkpoint should prefer the smallest practical control-plane scope:

1. one new acquisition-preparation facade module;
2. one dedicated provider-neutral BI bootstrap plan/evidence module if not already materialized separately;
3. `prw-control-plane/src/lib.rs` narrow public facade export;
4. `reachability_acquisition_evidence` updates only for the new retained first-owner/prepared evidence types actually required downstream;
5. `prw-control-plane/Cargo.toml` addition of the already-selected `aws-lc-rs = 1.18.0` profile used elsewhere in the repository;
6. `Cargo.lock` only if dependency graph materialization requires a lockfile byte change.

No bridge/runtime/Agent/Android source belongs in the preparation-source checkpoint unless compile coupling proves otherwise.

## Relationship to provider execution

Preparation is not live-owner mutation execution.

After preparation:

- replacement evidence continues through the already-selected AW path `AS -> AE -> AV`;
- first-owner evidence will require a separately materialized BI etcd execution/reconciliation path plus evidence-bound semantic mapper;
- `Superseded` remains non-grant.

The full `ReachabilityLiveOwnerAsyncAuthority::acquire(peer)` composition must wait until both prepared branches have validated provider-execution mappings.

## Runtime and recovery boundary

The facade must assume the surrounding lifecycle has already established a valid initialized PRWF head for the current recovery epoch.

It must not issue a recovery epoch, initialize PRWF state, contact Spanner recovery authority or fallback into recovery when normal preparation sees missing/unavailable PRWF state.

No endpoint/client bootstrap, task ownership or R1-R4 effect enforcement is selected here.

## Explicitly not authorized / not activated

C02f-BJ does not:

- add or modify Rust source;
- add `aws-lc-rs` to control-plane yet;
- generate attempt IDs;
- perform etcd/Spanner I/O;
- allocate a production fence;
- create or mutate live-owner state;
- publish private provider modules;
- construct endpoints/clients/TLS/auth/RBAC/runtime state;
- activate full `acquire(peer)`;
- activate R1-R4;
- deploy;
- merge.

## Validation gate

The gate is:

`C02F_BJ_ACQUISITION_HANDOFF_PREPARATION_FACADE_SELECTED`

It may be claimed only after canonical executable Rust validation passes on the exact final documentation-only BJ head and exact BI ancestry/scope are reverified.
