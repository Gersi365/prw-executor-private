# Phase 152 C03e-BJ — Candidate ID High-Water Observation Selection

Status: STAGED SELECTION

Gate target:
`C03E_BJ_CANDIDATE_ID_HIGH_WATER_OBSERVATION_SELECTED`

## 1. Exact predecessor

Closed C03e-BI:
- branch: `phase-152-c03e-bi-explicit-connectivity-candidate-assembly-source-materialization-staging`;
- head: `02fee2e5c1e097f8e16794af248bacd79c787f13`;
- tree: `b31d4570d343ce64b4359be30f3027d396a8a800`;
- gate: `C03E_BI_EXPLICIT_CONNECTIVITY_CANDIDATE_ASSEMBLY_SOURCE_MATERIALIZED`.

BI materialized only typed candidate assembly from an already-supplied `CandidateId`, `ConnectivityPathKind`, and `ConnectivityEndpoint`. It intentionally did not select candidate-ID allocation/custody or path-kind classification/provenance.

## 2. Purpose

The existing Phase 135 `PeerConnectivityPlan` already owns the anti-reuse authority for candidate identifiers through its private `candidate_id_high_watermark` state.

`refresh_candidates(...)` already enforces that:
- an exact retained candidate may keep its existing identifier;
- an identifier cannot be rebound to another kind/endpoint;
- a newly introduced candidate whose identifier is at or below the prior high-water mark fails closed;
- the stored high-water mark never decreases across refresh.

A future candidate-ID producer must preserve those rules. However, the current public API does not expose the plan's historical high-water state.

BJ selects only a read-only observation boundary for that already-existing state. It does not select an allocator.

## 3. Existing Phase 135 authority remains unchanged

The following source remains authoritative:

`crates/prw-connectivity/src/lib.rs`

Current exact-BI blob:
`bdefd6302fde130330be0c51073aa07345501249`

The current plan stores:

```text
candidate_id_high_watermark: u64
```

Its initial value is the maximum candidate identifier in the initial candidate set, or zero when the plan has never contained a candidate.

`refresh_candidates(...)` advances the value to the maximum of:
- the prior high-water mark; and
- the maximum identifier in the new candidate set.

BJ does not change any of these semantics.

## 4. Selected observation semantic

BJ selects one read-only semantic operation on `PeerConnectivityPlan`:

```text
candidate_id_high_watermark(&self) -> Option<CandidateId>
```

Required meaning:
- `None` means the plan has never observed any non-zero candidate identifier;
- `Some(id)` means `id` is the highest candidate identifier ever accepted by that plan lifetime;
- the returned value is historical allocation/anti-reuse state, not necessarily a currently active candidate;
- the operation performs no mutation;
- the operation performs no allocation;
- the operation performs no I/O.

The exact Rust method name may be mechanically adjusted during source materialization only if required by existing naming conventions while preserving this semantic contract.

## 5. Why `Option<CandidateId>` is selected

`CandidateId::new(0)` is invalid, while the internal high-water state uses zero only as the sentinel for "no candidate identifier has ever been accepted".

Returning `Option<CandidateId>` keeps that sentinel out of the public candidate-ID domain:
- internal zero -> `None`;
- any non-zero high-water value -> `Some(CandidateId)`.

BJ therefore does not expose zero as if it were a usable candidate identifier.

The returned `CandidateId` is still plan-scoped correlation state only. It is not logical device identity, authentication evidence, transport identity, authorization evidence, or reachability evidence.

## 6. Observation is not allocation

A high-water observation does not determine how the next identifier is produced.

BJ does not select:
- `high_water + 1` as a mandatory allocator;
- monotonic counter custody outside the existing plan;
- random identifiers;
- UUIDs;
- database sequences;
- persistence across process restart;
- cross-process or distributed uniqueness;
- overflow policy;
- recovery after allocator-state loss;
- broker restart semantics;
- candidate-ID reservation;
- batch allocation.

Any producer that creates new candidate identifiers remains separately gated.

## 7. No path-kind provenance is inferred

The high-water value says nothing about `ConnectivityPathKind`.

BJ does not classify or infer:
- `LocalDirect`;
- `InternetDirect`;
- `Relay`.

Phase 135 remains explicit that endpoint scope classification belongs to discovery/provider logic.

No IP address, socket address, interface name, STUN result, relay route, or ICE candidate class may be converted into a product path kind merely because a candidate-ID floor is known.

## 8. Relationship to C03e-BI

BI remains authoritative for:

```text
CandidateId + ConnectivityPathKind + ConnectivityEndpoint
    -> ConnectivityCandidate
```

BJ only makes historical candidate-ID floor state observable to a future separately reviewed producer.

A future producer may use the observation as one input to its custody/allocation logic, but BJ does not define that logic and does not call BI assembly.

## 9. Relationship to candidate publication

Candidate publication remains separately gated.

BJ does not invoke or alter:
- `publish_current_candidates`;
- `refresh_from_authenticated_publication`;
- durable provider CAS;
- publication freshness/currentness verification;
- reachability ownership;
- signaling transport.

Reading high-water state is not publication authority.

## 10. Relationship to refresh anti-reuse

The existing `refresh_candidates(...)` validation remains the final fail-closed authority even after a future producer exists.

A producer's use of the high-water observation cannot bypass:
- duplicate-ID rejection;
- duplicate `(path kind, endpoint)` rejection;
- exact retained-candidate requirement;
- candidate-ID rebound/reuse rejection;
- candidate-capacity bound;
- transactional no-mutation-on-error behavior.

The plan remains authoritative over whether a proposed candidate vector is acceptable.

## 11. Identity and security invariants

BJ preserves:
- `DeviceId` / authenticated PRW session identity as logical identity;
- `TransportIdentity` as lower-transport certificate identity only;
- `CandidateId` as plan-scoped candidate correlation only;
- `ConnectivityEndpoint` as transient endpoint/configuration state only;
- `ConnectivityPathKind` as product path classification only;
- `SessionId` as authentication correlation only;
- request IDs as message correlation only.

High-water observation is not authentication, authorization, membership, publication freshness/currentness, reachability, public-routability, readiness, or liveness evidence.

## 12. Selected future source ownership

If BJ closes cleanly, the first source tranche belongs only in:

`crates/prw-connectivity/src/lib.rs`

Rationale:
- the high-water state is already owned by `PeerConnectivityPlan`;
- exposing it elsewhere would duplicate hidden plan state;
- no Agent or remote-bridge dependency widening is required;
- no network/provider layer should become authoritative over the plan's own anti-reuse history.

## 13. Intended source materialization shape

The next source tranche may add only the read-only accessor and focused pure tests.

Expected semantic shape:

```rust
#[must_use]
pub const fn candidate_id_high_watermark(&self) -> Option<CandidateId> {
    if self.candidate_id_high_watermark == 0 {
        None
    } else {
        Some(CandidateId(self.candidate_id_high_watermark))
    }
}
```

Equivalent source that preserves the exact semantics is allowed.

The source tranche must not:
- mutate the high-water mark;
- allocate a new candidate identifier;
- expose a mutable reference to high-water state;
- reset/reduce high-water state;
- change `refresh_candidates(...)` acceptance rules;
- infer path kind;
- create or publish candidates;
- perform I/O.

## 14. Focused validation requirements

Future source validation must prove at minimum:
1. a newly created empty plan reports `None`;
2. an initial plan reports `Some(max_initial_id)`;
3. retaining/removing candidates cannot lower the observed high-water mark;
4. accepting a new higher identifier advances the observation;
5. failed refresh leaves the observation unchanged;
6. the accessor is read-only and requires no network/provider operation;
7. existing candidate selection and refresh tests remain green.

## 15. Explicit non-selections

BJ does not select or materialize:
- candidate-ID allocator/custody;
- allocator persistence/restart behavior;
- path-kind classifier/provenance;
- endpoint discovery/interface enumeration;
- address-scope policy;
- candidate construction beyond already-closed BI;
- candidate publication/freshness/currentness source;
- reachability observation generation;
- STUN/ICE/TURN/relay activation;
- registry/database/provider mutation;
- production bind-interface policy;
- externally-routable inference;
- expected-device provenance;
- SessionId/request-id production;
- remote dispatcher/provider backends;
- Agent `main.rs` activation;
- readiness/process-exit policy;
- retries/reconnect/rebind/rebootstrap/replacement;
- systemd/host/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## 16. Exact intended BI -> BJ scope

BJ is docs-only.

The exact branch must differ from closed BI only by:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BJ_CANDIDATE_ID_HIGH_WATER_OBSERVATION_SELECTION_STAGING.md`

Any Rust source, Cargo manifest, lockfile, workflow, Agent, provider, registry, networking, packaging/systemd or deployment change blocks BJ closure.

## 17. Closure conditions

BJ can close only after:
- exact BI predecessor lineage remains unchanged;
- exact BI -> BJ compare is one docs-only path;
- canonical Rust validation on the exact final BJ head reaches terminal success;
- every automatically triggered workflow reaches a terminal non-failing verdict;
- immutable Drive audit is uploaded inside the project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-readback verified;
- rolling Drive evidence passes a fresh predecessor guard, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No candidate-ID allocator or production networking materialization is authorized merely by BJ closure.

Gate target remains:
`C03E_BJ_CANDIDATE_ID_HIGH_WATER_OBSERVATION_SELECTED`
