# Phase 152 C02f-BE — Currentness Provider Execution Composition Selection Staging

## Status

Design/selection checkpoint only. This contract selects the future orchestration-side composition for authoritative currentness of one exact semantic live-owner grant.

This checkpoint performs no provider I/O and materializes no Rust source wiring.

## Exact prerequisite

The exact validated prerequisite is C02f-BD:

- head `440d3b57df2b762b051b4e68182034469ae457f5`;
- tree `fc1901332e4aecb30e218e12a3d0cd96380d1af1`;
- gate `C02F_BD_RECONCILED_RELEASE_ASYNC_AUTHORITY_COMPOSITION_VALIDATED`.

C02f-AD already owns the authoritative currentness primitive:

`ReachabilityLiveOwnerEtcdStore::currentness(peer, fence)`

That primitive performs one default-linearizable exact-key observation and then delegates to C02f-AB `classify_currentness(peer, fence, observation)`.

## Selected future composition input

A future source composition may accept only:

1. one already-created mutable `ReachabilityLiveOwnerEtcdStore`; and
2. one exact `ReachabilityLiveOwnerGrant` supplied by the semantic authority caller.

It must not accept an independently supplied peer, fence, observation, currentness result, endpoint, credential, runtime, client constructor, retry policy or cache entry.

The exact logical peer is always `grant.peer()`.

The provider fence is derived only from `grant.fence().get()` and converted to `NonZeroU128`. An impossible representation conversion fails before provider execution as semantic `ReachabilityLiveOwnerAuthorityError::FenceExhausted`.

## Selected provider execution sequence

The future composition is strictly:

1. derive `raw_fence` only from the supplied grant;
2. call only `ReachabilityLiveOwnerEtcdStore::currentness(grant.peer(), raw_fence)`;
3. map the returned exact provider classification 1:1 into semantic currentness;
4. map every provider error fail-closed.

No extra provider read is selected around `currentness`. The C02f-AD method already owns its exact linearizable observation and C02f-AB classification.

No caller-side observation rewriting, cached-state fallback, local currentness inference, advisory Watch interpretation or duplicate classification is selected.

## Exact semantic mapping

The only selected success mappings are:

- `LiveOwnerProviderCurrentness::Current` -> `ReachabilityLiveOwnerCurrentness::Current`;
- `LiveOwnerProviderCurrentness::Stale` -> `ReachabilityLiveOwnerCurrentness::Stale`.

No additional evidence capsule is required for currentness because the provider classification is produced only after exact `peer + fence` classification by C02f-AB.

A missing authoritative record does not map to `Stale`; it remains fail-closed through `LiveOwnerTxnError::MissingEstablishedState` and C02f-AD error propagation.

A cross-peer authoritative record does not map to `Stale`; it remains fail-closed through `LiveOwnerTxnError::PeerMismatch` and C02f-AD error propagation.

Malformed, non-canonical, unavailable or structurally contradictory provider state must not map to either semantic success variant.

## Selected error boundary

Every `ReachabilityLiveOwnerEtcdError` from C02f-AD maps to:

`ReachabilityLiveOwnerAuthorityError::UnavailableOrAmbiguous`

except that `FenceExhausted` remains reserved for impossible semantic/provider fence representation conversion before the provider call.

No provider error may become `Current`, `Stale`, retry authorization, local authority or cache fallback.

## No reconciliation layer for currentness

Currentness is a read-only authoritative classification and does not require the C02f-AE indeterminate-mutation reconciliation machinery.

The future composition must not call:

- `execute_release_with_reconciliation`;
- `execute_acquisition_with_reconciliation`;
- direct Txn execution;
- mutation planning;
- reissue loops; or
- release semantic mappers.

## C02f-AC legacy bridge boundary

The earlier `ReachabilityLiveOwnerProviderBridge` currentness mapping demonstrates the same provider-to-semantic shape, but the future selected composition must call the concrete C02f-AD store directly rather than route through the legacy `ReachabilityLiveOwnerDefinitiveProviderPort` abstraction.

No compatibility mutation or deletion of C02f-AC is selected in this checkpoint.

## Async/runtime boundary

A future source materialization should preserve the selected C02f-Y async contract by returning an explicit `impl Future<Output = Result<ReachabilityLiveOwnerCurrentness, ReachabilityLiveOwnerAuthorityError>> + Send`.

Provider I/O may occur only when that returned future is polled by an external executor.

The composition must not create or own a runtime, executor, task, background loop or cancellation policy.

## R1-R4 safety boundary

A successful semantic `Current` result is only an authoritative point-in-time currentness proof.

It does not authorize treating later side effects as unfenced. Future production R1-R4 effect sinks must still reject stale fences at the effect boundary according to the already-locked Phase 152 safety model.

No one-time currentness check may substitute for effect-side stale-fence rejection.

## Dependency and ownership boundary

The dependency direction remains:

`prw-remote-bridge -> prw-control-plane`

No inverse dependency is selected.

No Cargo dependency or `Cargo.lock` change is selected.

A future source composition belongs on the bridge/orchestration side because it combines one semantic grant with one already-created control-plane provider store and maps the provider result into the semantic async authority contract.

## Explicitly unselected in C02f-BE

This checkpoint does not select or activate:

- Rust source materialization of currentness composition;
- any actual etcd Get;
- endpoint or cluster selection;
- etcd client construction or `Client::connect`;
- TLS, auth, RBAC, credentials, users, roles or permissions;
- Watch, lease or TTL behavior;
- acquisition composition;
- fence allocation/reissue;
- authority attempt-ID generation;
- release mutation/reconciliation changes;
- recovery epoch issuance or Spanner contact;
- Agent/runtime integration;
- R1-R4 effect-side stale-fence rejection activation;
- deployment; or
- merge.

## Future source-materialization authorization boundary

Materializing this sequence in Rust would create a callable provider-I/O currentness composition when polled. That source step remains a separate checkpoint from this documentation-only selection.

The expected minimum future source scope is bridge-side only:

1. one new currentness execution module under `crates/prw-remote-bridge/src/`; and
2. one root module export if required.

No control-plane, Cargo, lockfile, Android or Agent mutation is currently selected.

## Selected future evidence chain

`exact semantic grant -> C02f-AD exact linearizable currentness classification -> exact semantic Current/Stale`

No intermediate stage may replace the grant peer/fence with request-controlled or cached identity.

## Validation gate

The selection gate may be claimed only after canonical executable Rust validation passes on the exact C02f-BE documentation head:

`C02F_BE_CURRENTNESS_PROVIDER_EXECUTION_COMPOSITION_SELECTED`
