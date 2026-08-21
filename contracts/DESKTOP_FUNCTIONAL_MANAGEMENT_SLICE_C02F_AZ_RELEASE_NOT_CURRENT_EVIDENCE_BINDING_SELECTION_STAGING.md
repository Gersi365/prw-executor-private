# Phase 152 C02f-AZ — Release NotCurrent Evidence Binding Selection

## Purpose

C02f-AZ selects the minimum provider-owned evidence binding required to make the existing C02f-AE top-level release `NotCurrent` terminal result safely consumable by a higher-level semantic mapper.

C02f-AY already established the concrete defect: `ReachabilityLiveOwnerResolvedRelease::NotCurrent` is currently a unit variant. After it is detached from the provider call that produced it, it retains neither the exact `PeerConnectivityIdentity` nor the exact non-zero fence generation that was classified as not current. C02f-AY therefore correctly fails that unbound value closed rather than permitting cross-grant evidence rebinding.

C02f-AZ is documentation only. It does not modify C02f-AE or C02f-AY source, perform provider I/O, issue an etcd read or transaction, construct a client/runtime, activate async authority, perform R1-R4 effects, or deploy anything.

## Exact base

C02f-AZ starts from the final staged C02f-AY head:

`7c3723de5e0b166bc79545053b20757b0e1e9939`

C02f-AY remains the immediately preceding reconciled-release semantic mapper checkpoint. Its mapper and root export are not modified by this selection tranche.

## Concrete evidence-continuity defect

The current C02f-AE release orchestration performs:

1. an authoritative release pre-read supplied as `LiveOwnerObservation`;
2. deterministic `plan_release(peer, fence, Some(&before))`;
3. if `into_transaction()` returns `None`, it returns `ReachabilityLiveOwnerResolvedRelease::NotCurrent` without a mutation.

`plan_release` already proves the authoritative observation belongs to the requested exact peer and returns no transaction only when the persisted authority state is not `Current` with the exact requested fence.

The defect is therefore not missing provider classification. The defect is loss of the exact peer/fence binding when that already-proven result crosses the C02f-AE terminal-result boundary.

C02f-AZ selects restoration of only that lost binding.

## Selected provider-owned evidence capsule

A later source tranche should introduce one dedicated provider-neutral evidence type conceptually equivalent to:

```text
ReachabilityLiveOwnerResolvedNotCurrent {
    peer: PeerConnectivityIdentity,
    fence: NonZeroU128,
}
```

The exact Rust layout may follow repository formatting conventions, but the selected semantics are fixed:

- `peer` is the exact `PeerConnectivityIdentity` supplied to the validated release classification;
- `fence` is the exact non-zero fence supplied to the validated release classification;
- both fields are retained together as one indivisible terminal-evidence capsule;
- the type is provider-neutral and contains no etcd client, endpoint, response object, credential, retry state, runtime handle, timer, or network resource.

The existing terminal enum should then carry this evidence instead of an unbound unit value, conceptually:

```text
ReachabilityLiveOwnerResolvedRelease::NotCurrent(ReachabilityLiveOwnerResolvedNotCurrent)
```

The existing `Mutation(Box<ReachabilityLiveOwnerResolvedMutation>)` branch remains unchanged.

## Construction authority

The evidence capsule must not be freely forgeable by higher-level consumers.

Selected construction rule:

- construction occurs only inside the C02f-AE release reconciliation module after `plan_release(peer, fence, Some(&before))` has succeeded and produced no transaction;
- public struct fields are not selected;
- a public arbitrary constructor is not selected;
- `Default`, tuple conversion, raw-string conversion, deserialization, or request-controlled construction are not selected;
- public consumers may receive the evidence and inspect read-only accessors only.

The minimal read-only public surface is:

- `peer() -> &PeerConnectivityIdentity`;
- `fence() -> NonZeroU128`.

Whether the internal constructor is a private function or direct same-module struct construction is an implementation detail; it must not become a general public evidence-minting API.

## Why the full observation is not retained

C02f-AZ deliberately does not select retention of the entire `LiveOwnerObservation` in top-level `NotCurrent` evidence.

The higher-level semantic decision requires proof that this terminal result belongs to the same exact grant peer/fence. It does not require the encoded key/value bytes, `mod_revision`, lifecycle record, authority-attempt identifier, or another provider read.

Retaining the full observation would broaden the public terminal evidence surface without adding proof needed by the semantic mapper. The existing deterministic planner remains authoritative for classifying the observation before the terminal result is created.

## Future semantic mapper rule

After the provider-owned evidence binding is materialized, a later semantic-mapping checkpoint may restore a safe top-level `NotCurrent` success mapping only when both retained values match the supplied semantic grant exactly.

Selected future rule:

1. convert the supplied semantic grant fence to the exact non-zero provider representation using the already-established fail-closed conversion;
2. require `evidence.peer() == grant.peer()`;
3. require `evidence.fence() == converted grant fence`;
4. only then map to `ReachabilityLiveOwnerRelease::NotCurrent`.

Any peer mismatch, fence mismatch, impossible fence representation, or otherwise contradictory evidence must fail closed. A mismatched evidence capsule must never be rebound to the supplied grant.

C02f-AZ does not modify the C02f-AY mapper itself. Semantic success consumption of the new evidence remains a later separately validated source boundary.

## Compile-coupling constraint discovered during AZ preflight

A source-materialization preflight found one concrete compatibility constraint that must be preserved explicitly.

Changing the public enum shape from the current unit variant:

```text
ReachabilityLiveOwnerResolvedRelease::NotCurrent
```

to a payload variant:

```text
ReachabilityLiveOwnerResolvedRelease::NotCurrent(ReachabilityLiveOwnerResolvedNotCurrent)
```

is a compile-time API change for every exhaustive consumer. The existing C02f-AY bridge mapper currently matches the unit variant directly, so a control-plane-only enum mutation would make the workspace fail to compile before semantic behavior could be validated.

Therefore the evidence-binding source checkpoint must include the minimum mechanical consumer adaptation required to preserve compilation:

- update the C02f-AY mapper pattern from the unit form to the payload form;
- continue to return `ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous` for that branch;
- do not inspect or semantically trust the new payload in that compatibility checkpoint except as required by the type pattern;
- do not restore semantic `ReachabilityLiveOwnerRelease::NotCurrent` yet;
- preserve all mutation-backed mapping behavior unchanged.

This mechanical fail-closed adaptation is not the later semantic-consumption checkpoint. It exists only because the public enum signature and its existing exhaustive consumer cannot be changed independently while keeping the workspace buildable.

## Existing mutation-backed release semantics remain unchanged

C02f-AZ does not change the validated meaning of C02f-AE mutation evidence or the C02f-AY mutation-backed mapping:

- exact retained release successor peer must match the semantic grant peer;
- exact retained release successor fence must match the semantic grant fence;
- successor lifecycle must be `Released`;
- `Committed` maps to semantic `Released`;
- `Superseded` maps to semantic `NotCurrent`;
- `CompareFailed` remains classified by the existing deterministic C02f-AB currentness semantics;
- contradictions fail closed.

No transaction is manufactured for the no-mutation `NotCurrent` path.

## Dependency direction

The selected dependency direction remains unchanged:

`prw-remote-bridge -> prw-control-plane`

The evidence capsule belongs with the C02f-AE provider-owned terminal result in `prw-control-plane`. The bridge mapper may consume the public evidence but `prw-control-plane` must not depend on `prw-remote-bridge`.

No Cargo dependency or lockfile change is selected.

## Future source-materialization scope

A later evidence-binding source checkpoint may change only the minimum surfaces required to bind the no-mutation terminal result while keeping the workspace compilable, expected to include:

1. `crates/prw-control-plane/src/reachability_live_owner_etcd/reconciliation.rs` for the new evidence type, read-only accessors, enum payload, and provider-owned construction;
2. `crates/prw-control-plane/src/reachability_live_owner_etcd/reconciliation/tests.rs` for focused control-plane evidence tests;
3. `crates/prw-remote-bridge/src/reachability_live_owner_reconciled_release.rs` only for the mechanically required payload-pattern compatibility change and any directly coupled test adjustment, while preserving top-level bound `NotCurrent` as fail-closed;
4. only if required by existing module visibility conventions, a bounded existing public facade/export surface in `prw-control-plane`.

The source checkpoint must not activate provider-execution composition, broaden runtime ownership, or restore semantic success for top-level `NotCurrent`. The subsequent semantic mapper checkpoint will separately inspect exact evidence peer/fence and may map it to semantic `NotCurrent` only after exact grant binding is proven.

## Required future tests for evidence materialization

The evidence-binding source checkpoint should include deterministic coverage proving at least:

1. a release classification for an already stale fence creates bound `NotCurrent` evidence with the exact requested peer and fence;
2. a release classification for an already `Released` exact fence creates bound `NotCurrent` evidence with the exact requested peer and fence;
3. a peer mismatch still fails in deterministic planning and never mints `NotCurrent` evidence;
4. an exact-current owner still produces the existing release mutation path rather than top-level `NotCurrent` evidence;
5. mutation-backed reconciliation behavior remains semantically stable outside the enum signature change;
6. the evidence type exposes no public arbitrary constructor capable of minting a different peer/fence binding;
7. the bridge remains buildable against the payload enum while still failing the top-level bound `NotCurrent` branch closed;
8. existing mutation-backed C02f-AY mapper tests remain unchanged in meaning.

Because a deliberately non-forgeable evidence capsule cannot be arbitrarily constructed from another crate, the compatibility checkpoint is not required to manufacture fake provider-owned evidence merely to execute the bridge fail-closed arm. Control-plane tests own evidence construction coverage; the workspace build owns cross-crate signature compatibility. Exact semantic evidence matching is tested later when semantic consumption is selected.

No new network I/O is required to test the evidence data model itself beyond any already-existing bounded provider harness used by the selected source checkpoint.

## Deliberately unselected neighboring work

C02f-AZ does not select or implement:

- C02f-AE provider reconciliation redesign;
- another linearizable read for semantic mapping;
- another release transaction or blind retry;
- semantic success consumption of bound top-level `NotCurrent` in this selection tranche;
- release provider-execution composition;
- currentness provider-execution composition;
- complete `ReachabilityLiveOwnerAsyncAuthority` implementation;
- provider/client construction or ownership lifecycle;
- endpoint/TLS/auth/RBAC/credential configuration;
- recovery-epoch or fence-sequence runtime allocation;
- authority-attempt-ID generation;
- R1-R4 stale-effect enforcement;
- Agent/runtime integration;
- deployment or merge.

## Explicit non-activation boundary

C02f-AZ does not:

- modify Rust source;
- modify C02f-AE or C02f-AY files;
- perform etcd Get / Txn / re-observation;
- call `execute_release_with_reconciliation`;
- call `plan_release` at runtime;
- construct or connect an etcd client;
- select endpoints, TLS, authentication, RBAC, credentials, leases, TTL, Watch, users, roles, permissions, or cluster membership;
- allocate or reissue a production fence sequence;
- issue a recovery epoch or contact Spanner;
- implement or activate `ReachabilityLiveOwnerAsyncAuthority`;
- create a runtime, task, timer, detached future, or background retry;
- execute traversal/network effects;
- implement or activate R1-R4 stale-effect rejection;
- modify Cargo manifests or `Cargo.lock`;
- deploy;
- merge a draft PR.

## Exact source scope

C02f-AZ adds exactly this one documentation contract.

No Rust, workflow, manifest, lockfile, Android, Agent, runtime, client, provider, credential, network, or deployment file is selected for mutation in this tranche.

## Validation gate

C02f-AZ is selected only if a fresh AY -> AZ compare proves:

- exact AY merge base `7c3723de5e0b166bc79545053b20757b0e1e9939`;
- exactly one added documentation contract;
- zero Rust/source/runtime/workflow/manifest/lockfile changes.

Canonical repository CI should still be requested on the exact final AZ head. If the established GitHub Actions zero-step infrastructure restriction persists, the tranche must remain explicitly staged / infrastructure-blocked / not validated rather than treating the absence of source diagnostics as a pass.

Expected gate after executable canonical validation:

`C02F_AZ_RELEASE_NOT_CURRENT_EVIDENCE_BINDING_SELECTED`

A later source checkpoint may materialize the selected provider-owned peer/fence evidence capsule plus only the mechanically required fail-closed bridge compatibility adaptation. Semantic success consumption remains separately gated.
