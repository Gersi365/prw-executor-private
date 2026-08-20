# Phase 152 C02f-Y — Async Authority Port Source Staging Closeout Audit

Status: `SOURCE_STAGING_COMPLETE / C02F_X_SELECTION_MATERIALIZED / SEPARATE_ASYNC_PORT / IMPL_FUTURE_PLUS_SEND / STATIC_DISPATCH / MUTABLE_RECEIVER / SYNC_SEAM_PRESERVED / FULL_RUST_VALIDATION_PASS / PR43_MERGED / NO_ETCD_ENDPOINT / NO_SCHEMA_OR_CAS_SELECTION / NO_TLS_OR_RUNTIME_ACTIVATION`

Date: 2026-08-20
Repository: `powercode365-dotcom/prw-executor-private`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Frozen C02d head: `857583b25ed1206317641a93fd8f927819c954d8`
C02f-X selection head: `839caa2d4343a8cf455bc2e3991b817b3f6b901e`
C02f-X tree: `a9fb96bce6d96d428cba7578b0194d159e46b334`
C02f-Y source commit: `73ff17f73974c69e63bb51a3ae596d9c9ae8548a`
C02f-Y source tree: `a772f04766f6cb9fea2bb3276db9c24c40f6de0e`

## Purpose

C02f-X locked the explicitly approved asynchronous live-owner authority API/orchestration architecture. C02f-Y is the first narrow source materialization of that lock.

The tranche intentionally stages only the provider-neutral async authority port and its crate-root exposure. It does not implement provider-specific etcd operations and does not activate any runtime, endpoint, socket, task, TLS profile, deployment or R1-R4 network effect.

## Exact source scope

The squash-merged source commit changes exactly two paths relative to C02f-X:

1. `crates/prw-remote-bridge/src/reachability_live_owner_async.rs`
   - new file;
   - 166 additions;
   - merged blob `6ded4e006d30e71c548ea985339d2435eaa5a741`.
2. `crates/prw-remote-bridge/src/root.rs`
   - 3 additions / 1 deletion;
   - merged blob `4a5b982b3267b0a5b3b8b67984c87881c1b9425d`.

No Cargo manifest, lockfile, control-plane source, Agent source, transport source, NAT traversal source or existing synchronous live-owner authority source changed in the source commit.

## Materialized API

C02f-Y adds the public provider-neutral trait:

`ReachabilityLiveOwnerAsyncAuthority`

The trait is deliberately separate from the existing synchronous `ReachabilityLiveOwnerAuthority` semantic/reference seam.

Its initial operation surface mirrors the minimum existing live-owner semantics:

- `acquire`;
- `currentness`;
- `release`.

Each operation:

- takes `&mut self`;
- borrows its exact peer/grant input for the Future lifetime;
- returns native Rust `impl Future<Output = Result<...>> + Send`;
- uses static dispatch;
- does not require `async-trait`;
- does not require boxed/dyn Future allocation;
- does not create or own a runtime.

## Preserved synchronous semantic seam

The existing file:

`crates/prw-remote-bridge/src/reachability_live_owner.rs`

remains byte-stable at blob:

`ad21a7cc4369e1f5b9953f72c5b12bf64e50a404`

Therefore C02f-Y does not disguise provider I/O behind the old synchronous API and does not force deterministic reference/Sans-I/O semantics to depend on an executor.

## Crate/dependency preservation

`crates/prw-remote-bridge/Cargo.toml` remains byte-stable at blob:

`5e59862f0a2ee120e05c5b4569ebe25d85ffd79d`

Consequences:

- `prw-control-plane` remains a dev-dependency of `prw-remote-bridge` at this checkpoint;
- no new normal bridge-to-control-plane dependency has yet been required;
- no Tokio dependency was added to bridge;
- no Tokio/etcd dependency was added to Agent;
- no `async-trait` dependency was added;
- no Cargo.lock mutation was required.

Provider-specific etcd adapter materialization therefore remains a later source tranche after the unresolved schema/CAS and identity-boundary decisions are selected.

## Fail-closed reference coverage

The new module includes a runtime-independent test-only `FailClosedReferenceAuthority` implementation.

It returns `UnavailableOrAmbiguous` for acquire/currentness/release using `std::future::ready` and exists only to prove the selected port shape without a runtime/backend.

The source test proves that Futures returned from all three operations satisfy the selected `Send` contract while borrowing a mutable concrete authority and exact peer/grant inputs.

No test constructs an etcd endpoint or performs network I/O.

## PR and validation evidence

Source staging used branch:

`phase-152-c02f-y-async-authority-port-staging`

Pull request:

- PR #43 — `Phase 152 C02f-Y async authority port staging`;
- base: `phase-152-c02e-dynamic-reachability-design` at C02f-X;
- final validated PR head: `ee0cf365d9fd494dcf13e2c3bd9e48e71b0225bd`;
- final PR diff: exactly two source paths, 169 additions / 1 deletion.

The first canonical run (#728) stopped only at rustfmt and therefore did not reach Clippy/tests/build. The corrective changed formatting only.

Final canonical PRW Rust Validation:

- workflow: `PRW Rust Validation`;
- run number: `729`;
- run ID: `32334417370`;
- validated head: `ee0cf365d9fd494dcf13e2c3bd9e48e71b0225bd`;
- conclusion: `SUCCESS`.

Successful stages:

- desktop native prerequisites;
- Rust/toolchain recording;
- locked dependency graph verification;
- `cargo fmt --all -- --check`;
- full workspace Clippy with warnings denied;
- full workspace tests;
- full workspace build.

The validated PR was then marked ready and squash-merged with expected head protection.

Squash merge result:

- merged: true;
- source commit: `73ff17f73974c69e63bb51a3ae596d9c9ae8548a`;
- parent: exact C02f-X head `839caa2d4343a8cf455bc2e3991b817b3f6b901e`;
- tree content matches the validated final PR tree for the changed source paths.

## Safety invariants retained

C02f-Y does not weaken any inherited live-owner invariant:

- exact namespace remains `DeviceId + TransportIdentity`;
- `ReachabilityLiveOwnerFence(NonZeroU128)` remains the logical ordered fence type;
- ambiguity/unavailability fails closed;
- advisory watches/caches cannot prove currentness;
- stale release cannot clear a newer owner;
- release remains a liveness operation, not the safety foundation;
- dropping or cancelling a pending async authority operation cannot be interpreted as successful ownership;
- bridge-level currentness alone remains insufficient for real effects;
- R1-R4 must ultimately reject stale fencing at or atomically with their actual effect boundaries.

## Explicitly not implemented by C02f-Y

This checkpoint does not implement or select:

- `etcd_client::Client::connect`;
- production etcd Get/Txn/Put calls;
- real etcd endpoint discovery/configuration;
- authority key prefix/framing/version bytes;
- exact DeviceId encoding/length policy;
- authority record value schema;
- owner/attempt identifier representation;
- fence byte encoding as persisted provider state;
- exact CAS compare target (`value`, `mod_revision`, or another proven guard);
- indeterminate Txn reconciliation algorithm;
- bridge-to-control-plane production dependency edge;
- exact identity representation crossing into control-plane;
- TLS feature selection;
- CA/certificate/client identity configuration;
- etcd authentication/RBAC;
- cluster member topology or placement;
- recovery epoch bit split/provider;
- external immutable epoch ledger provider;
- executor/bootstrap runtime ownership;
- Agent Tokio expansion;
- R1-R4 effect-side fencing implementation;
- runtime/network/deployment activation.

## Next safe gate

The async API representation and bridge orchestration boundary are no longer blockers.

The next real architecture gate before a provider-specific etcd adapter can be implemented safely is the exact provider state/transaction contract, especially:

1. exact key encoding for `DeviceId + TransportIdentity`;
2. exact versioned authority record/value representation;
3. persisted fence encoding;
4. stable owner/attempt identity representation;
5. exact etcd Txn compare/CAS guard;
6. indeterminate mutation reconciliation semantics;
7. exact identity type boundary between bridge and control-plane.

C02f-N/S contain preferred directions for several of these, but those directions remain unselected until separately approved.

## Closeout conclusion

C02f-Y successfully materializes the C02f-X-selected async production authority port as a separate, statically dispatched, `impl Future + Send`, `&mut self` API in `prw-remote-bridge` while preserving the synchronous semantic seam and all production/runtime/network gates.

The tranche passed canonical full Rust validation and was merged as a single clean source commit. No etcd endpoint was contacted and no runtime/network effect was activated.

C02d remains frozen and untouched.
