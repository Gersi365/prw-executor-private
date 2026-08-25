# Phase 152 C03e-BK — Candidate ID High-Water Observation Source Materialization

Status: STAGED SOURCE MATERIALIZATION

Gate target:
`C03E_BK_CANDIDATE_ID_HIGH_WATER_OBSERVATION_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Closed C03e-BJ:
- branch: `phase-152-c03e-bj-candidate-id-high-water-observation-selection-staging`;
- head: `6d1cac944e36974812c7f4ae9c3d06a065e1399d`;
- tree: `f916d37c89a23b6e7e2632ee584375531acbd993`;
- gate: `C03E_BJ_CANDIDATE_ID_HIGH_WATER_OBSERVATION_SELECTED`.

BJ selected only read-only observation of the existing `PeerConnectivityPlan` candidate-ID high-water state. It did not select an allocator, custody/persistence policy, path-kind classifier, publication behavior, networking activation, readiness, deployment or merge.

## 2. Materialized source

BK changes only:

`crates/prw-connectivity/src/lib.rs`

Predecessor source blob:
`bdefd6302fde130330be0c51073aa07345501249`

BK source blob:
`2ca071988d9e6aa90bb6b77957e27f4a95bfac12`

The source diff is additive only: `+91/-0`.

No existing source line was removed or rewritten by the source commit.

## 3. Materialized accessor

BK materializes:

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

The accessor:
- reads only the plan-owned existing high-water field;
- maps the internal zero sentinel to `None`;
- maps any non-zero value to the exact typed `CandidateId`;
- performs no mutation;
- performs no allocation or reservation;
- performs no provider/registry lookup;
- performs no socket, DNS, traversal, relay or other I/O.

## 4. Existing anti-reuse authority remains unchanged

`PeerConnectivityPlan::new(...)` remains authoritative for initial high-water initialization.

`PeerConnectivityPlan::refresh_candidates(...)` remains authoritative for:
- candidate-count validation;
- duplicate candidate-ID rejection;
- duplicate `(path kind, endpoint)` rejection;
- exact retained-candidate identity;
- candidate-ID rebound/reuse rejection;
- monotonic high-water advancement;
- validation-before-mutation transactional behavior.

BK does not modify those methods or their acceptance rules.

A high-water observation is advisory input to a future separately reviewed producer only; the plan continues to fail closed independently on invalid proposed candidates.

## 5. Focused tests materialized

BK adds pure tests proving:

1. an empty plan reports `None`;
2. initial candidates report the maximum accepted identifier;
3. removing the higher active candidate does not reduce historical high-water state;
4. accepting a higher new identifier advances high-water state;
5. a failed refresh preserves the previous high-water observation.

The existing Phase 135/C02e connectivity tests remain in place and are not weakened.

## 6. Observation is not allocation

BK does not create a candidate-ID producer.

The accessor does not define:
- `high_water + 1` as an allocation policy;
- overflow behavior;
- counter ownership;
- persistence across process restart;
- cross-process or distributed uniqueness;
- database sequencing;
- random or UUID allocation;
- reservation or batching;
- recovery after state loss.

Those responsibilities remain separately gated.

## 7. Path-kind provenance remains unresolved

`ConnectivityPathKind` remains explicit product classification only.

BK does not infer `LocalDirect`, `InternetDirect`, or `Relay` from:
- IP address scope;
- interface name;
- `SocketAddr`;
- STUN/server-reflexive results;
- ICE protocol candidate classes;
- relay-route existence;
- reachability success.

Discovery/provider-owned classification provenance remains separately gated.

## 8. Relationship to BI candidate assembly

Closed BI remains authoritative for typed assembly:

```text
CandidateId + ConnectivityPathKind + ConnectivityEndpoint
    -> ConnectivityCandidate
```

BK only makes historical ID floor state observable. It does not invoke BI assembly, create a candidate vector, mutate a connectivity plan through a new path, or publish candidates.

## 9. Publication and reachability remain separate

BK does not invoke or alter:
- authenticated candidate publication;
- publication freshness/currentness checks;
- durable provider CAS;
- reachability-owner state;
- `set_observation` generation;
- STUN/ICE connectivity checks;
- TURN/relay provider activation.

A high-water observation is not publication or reachability evidence.

## 10. Identity and security invariants

BK preserves:
- `DeviceId` / authenticated PRW session identity as logical identity;
- `TransportIdentity` as lower-transport certificate identity only;
- `CandidateId` as plan-scoped candidate correlation only;
- `ConnectivityEndpoint` as transient endpoint/configuration state only;
- `ConnectivityPathKind` as product path classification only;
- `SessionId` as authentication correlation only;
- request IDs as message correlation only.

High-water observation is not authentication, authorization, membership, currentness, public-routability, liveness or readiness evidence.

## 11. Explicit non-selections

BK does not select or materialize:
- candidate-ID allocator/custody;
- allocator persistence/restart/recovery semantics;
- candidate-ID overflow/reservation/batching policy;
- path-kind classifier/provenance;
- endpoint discovery/interface enumeration;
- address-scope/public-routability inference;
- candidate construction beyond closed BI;
- candidate publication/freshness/currentness source;
- reachability observation generation;
- STUN/ICE/TURN/relay activation;
- registry/database/provider mutation;
- expected-device provenance;
- SessionId/request-id production;
- remote dispatcher/provider backends;
- Agent `main.rs` activation;
- readiness/process-exit policy;
- retry/reconnect/rebind/rebootstrap/replacement;
- systemd/host/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## 12. Exact intended BJ -> BK scope

The final BK branch must differ from closed BJ only by:

1. `crates/prw-connectivity/src/lib.rs` — selected accessor + focused pure tests;
2. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BK_CANDIDATE_ID_HIGH_WATER_OBSERVATION_SOURCE_MATERIALIZATION_STAGING.md` — this contract.

Any Cargo manifest, lockfile, workflow, Agent, remote-bridge, registry/provider, Android/Kotlin, packaging/systemd, networking or deployment change blocks closure.

## 13. Validation requirements

BK can close only after:
- exact BJ predecessor lineage remains unchanged;
- exact BJ -> BK compare is exactly the two intended paths;
- source diff remains additive and bounded to the selected accessor/tests;
- canonical Rust validation on exact final BK head reaches terminal success;
- Android validation triggered by the source change reaches terminal success for both native adapter and Android application;
- every other automatically triggered workflow reaches a terminal non-failing verdict;
- immutable audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-readback verified;
- rolling Drive evidence passes fresh predecessor guard, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No allocator, production networking, readiness or deployment materialization is authorized merely by BK closure.

Gate target remains:
`C03E_BK_CANDIDATE_ID_HIGH_WATER_OBSERVATION_SOURCE_MATERIALIZED`
