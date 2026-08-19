# Phase 152 C02f-B — Durable Live-Owner Backend Preselection Audit

Status: `PRESELECTION_AUDIT_COMPLETE / NO_CONCRETE_BACKEND_SELECTED / EXISTING_ACCEPTED_STATE_CAS_NOT_RECLASSIFIED_AS_LIVE_OWNER_TENANCY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Repository: `powercode365-dotcom/prw-executor-private`

Repository ID: `1334911207`

Active branch: `phase-152-c02e-dynamic-reachability-design`

Exact predecessor head: `e40251b9b1400a355e6f21506517b0df68a4caa2`

Exact predecessor tree: `db21ec25eb6dccbe6c91f5f906f6ef016df5e7dc`

Predecessor checkpoint: `C02f-A durable live-owner backend authority gate`

## Audit purpose

C02f-B performs only repository-native preselection due diligence for the future durable live-owner authority backend required by C02f-A.

This checkpoint does not select a database, key-value store, consensus service, lease service, persistence encoding, migration strategy, wire protocol, runtime topology, network path, or deployment target. It does not add a dependency and does not mutate production Rust source.

The goal is to determine whether the repository already contains a concrete backend that can satisfy the C02f-A safety contract without architecture invention.

## C02f-A safety requirements inherited unchanged

Any later concrete live-owner backend must preserve all of the following:

1. exact authority namespace = `DeviceId + TransportIdentity`;
2. non-zero ordered `u128` logical fencing generation;
3. every replacement grant for one exact peer receives a strictly greater generation than every previously issued generation for that namespace;
4. acquisition/replacement linearizes against authoritative durable state;
5. stale owners remain permanently stale for authority purposes;
6. stale release cannot clear a newer grant;
7. restart/failover/recovery cannot reuse or roll back the fencing generation;
8. ambiguous or unavailable authority fails closed;
9. future side effects must reject stale fences at, or atomically with, the side-effect boundary;
10. clock/TTL/heartbeat behavior may assist liveness later but is not the primary stale-owner safety authority.

C02f-B does not weaken, reinterpret, or implement these requirements.

## Existing production live-owner seam

Authoritative source:

`crates/prw-remote-bridge/src/reachability_live_owner.rs`

Blob at predecessor head:

`ad21a7cc4369e1f5b9953f72c5b12bf64e50a404`

The current source already defines the provider-neutral:

- `ReachabilityLiveOwnerFence`;
- `ReachabilityLiveOwnerGrant`;
- `ReachabilityLiveOwnerAcquisition`;
- `ReachabilityLiveOwnerCurrentness`;
- `ReachabilityLiveOwnerRelease`;
- `ReachabilityLiveOwnerAuthorityError`;
- `ReachabilityLiveOwnerAuthority` trait.

The seam explicitly leaves acquisition/replacement policy, lease/TTL/heartbeat mechanics if any, persistence, and failover to a future concrete implementation.

Therefore no concrete backend can be inferred merely from the existence of this trait.

## Accepted reachability-state persistence precedent

Authoritative source:

`crates/prw-remote-bridge/src/reachability_owner.rs`

Blob at predecessor head:

`8d0e65c3fc0bd646c257199d4f55be65fa3f792d`

The current production owner already defines a distinct persistence seam:

`ReachabilityDurableStore`

with:

- `load_current(peer)`;
- `compare_and_commit(expected_current, replacement)`;
- linearizable expected-current compare-and-commit for one exact peer lifecycle;
- `UnavailableOrAmbiguous` fail-closed persistence classification;
- `StaleExpected` as a definite non-commit.

This is valuable architectural precedent for atomic durable mutation and bounded ambiguity handling.

It is not, however, a live-owner tenancy backend. Its comparison authority is `CandidatePublicationFreshnessToken` over accepted reachability snapshots. C02e/C02f-A explicitly keeps candidate-publication freshness separate from distributed live-owner fencing.

Reclassifying `ReachabilityDurableStore` or its freshness token as the live-owner authority would collapse two separately reviewed authority domains and is not authorized by this audit.

## Existing executable persistence harness is test-only

Authoritative test source:

`crates/prw-remote-bridge/tests/reachability_owner_production_seam.rs`

Blob at predecessor head:

`de0772e8f8198b168fcfb47c4b5b66771ab40d22`

The test implements `ReachabilityDurableStore` with `MemoryStore` backed by `Rc<RefCell<Option<ReachabilityDurableSnapshot>>>` and a test-controlled ambiguous-commit flag.

This harness proves contract behavior only. It is in-memory, process-local, non-durable, non-replicated, and not a production backend candidate.

It cannot preserve fencing history across process restart, host restart, or failover and therefore cannot satisfy the C02f-A live-owner durability contract.

## Registry precedent is explicitly in-memory

Authoritative source:

`crates/prw-registry/src/lib.rs`

Blob at predecessor head:

`cd7eb52eea19354620ff8ad26c9b8aad3f9c5eb6`

The registry states that it deliberately does not persist a database and stores bounded membership/device state in `HashMap` values.

Its manifest contains only internal PRW dependencies plus test crypto support.

Manifest blob:

`ec9215d9bad86ac9601e2f2d1bc0ed8461e724c1`

The registry therefore remains current-identity authority, not a durable live-owner persistence provider.

## Control-plane precedent does not select persistence

Authoritative source:

`crates/prw-control-plane/src/lib.rs`

Blob at predecessor head:

`668619338b1e085a4ac42bc27f793014e8a03df2`

The control-plane contract explicitly states that its enrollment/session semantics do not select a persistence layer.

Its manifest depends only on `prw-core`.

Manifest blob:

`a940a7eb23764452b9ef1fb24b8d20a91ba712c9`

No control-plane database/provider implementation can therefore be treated as an existing live-owner backend.

## Remote-bridge dependency surface

Authoritative manifest:

`crates/prw-remote-bridge/Cargo.toml`

Blob at predecessor head:

`5e59862f0a2ee120e05c5b4569ebe25d85ffd79d`

Production dependencies are only internal PRW crates:

- `prw-connectivity`;
- `prw-file-service`;
- `prw-file-transfer`;
- `prw-forwarding`;
- `prw-nat-traversal`;
- `prw-policy`;
- `prw-registry`;
- `prw-remote-transport`;
- `prw-session`;
- `prw-terminal`.

There is no current production persistence/database/coordination dependency on this crate.

## Exact lockfile dependency preflight

Authoritative `Cargo.lock` blob at predecessor head:

`4d69f7c6ec5a779615595f7dac8e02b2a660dc5d`

Exact lockfile inspection found no package entries matching the following common persistence/coordination crates or families checked during this bounded audit:

- `rusqlite` / SQLite;
- `sqlx`;
- `redb`;
- `sled`;
- `rocksdb`;
- `tokio-postgres`;
- `redis`;
- `etcd-client`.

This negative result is not a universal survey of every possible backend technology. It is sufficient to establish that no already-locked obvious backend from those families can be promoted mechanically into the live-owner seam.

Adding any new external backend dependency would be a separately reviewable dependency and architecture decision.

## Preselection conclusion

The repository contains useful semantics but no concrete durable live-owner backend ready for direct reuse.

Reusable precedent:

- exact-peer identity via `PeerConnectivityIdentity`;
- bounded fail-closed error classifications;
- linearizable compare-and-commit semantics for accepted state;
- recovery-required behavior when persistence outcome is ambiguous;
- provider-neutral live-owner fencing seam;
- executable exact-peer namespace/fencing tests from C02e Tranche 6.

Not reusable as a concrete backend:

- the test-only `MemoryStore`;
- `WorkspaceDeviceRegistry` HashMaps;
- control-plane typed domain state;
- candidate-publication freshness tokens as live-owner fences.

## What remains before provider selection

A separately authorized provider-selection checkpoint must compare candidate implementation families against the C02f-A contract and must document, at minimum:

1. durable record keyed by exact `DeviceId + TransportIdentity`;
2. atomic monotonic generation allocation/replacement mechanism;
3. exact behavior under process restart, host restart and backend failover;
4. stale-release compare condition;
5. ambiguity semantics for interrupted/unknown commits;
6. fence-exhaustion behavior;
7. side-effect fencing integration boundary;
8. storage representation and migration implications;
9. dependency/build/runtime footprint;
10. whether the topology is single-host, multi-process, or distributed and what safety claim that topology can actually support.

No provider should be selected solely because it supports transactions or compare-and-swap. The selected mechanism must preserve strictly monotonic non-reused fencing generations and side-effect fencing under the intended failure model.

## Production-source byte-stability baseline

This audit-only checkpoint requires the following predecessor blobs to remain unchanged:

- `crates/prw-remote-bridge/src/reachability_live_owner.rs`
  - `ad21a7cc4369e1f5b9953f72c5b12bf64e50a404`
- `crates/prw-remote-bridge/src/reachability_owner.rs`
  - `8d0e65c3fc0bd646c257199d4f55be65fa3f792d`
- `crates/prw-remote-bridge/src/root.rs`
  - `591320cbba4b7c3bdfcfd37a8176d82db33c1db6`
- `crates/prw-remote-bridge/Cargo.toml`
  - `5e59862f0a2ee120e05c5b4569ebe25d85ffd79d`
- root `Cargo.toml`
  - `fbbd220348e3008b38d4cfb1ec5721f8c12199e2`
- `Cargo.lock`
  - `4d69f7c6ec5a779615595f7dac8e02b2a660dc5d`
- `crates/prw-registry/src/lib.rs`
  - `cd7eb52eea19354620ff8ad26c9b8aad3f9c5eb6`
- `crates/prw-control-plane/src/lib.rs`
  - `668619338b1e085a4ac42bc27f793014e8a03df2`

## Mutation boundary

The only authorized repository mutation for this checkpoint is this audit file.

No production Rust source, Cargo manifest, lockfile, workflow, database schema, migration, runtime, network, Agent/bootstrap, systemd, signing, credential, deployment, or privileged-host state is changed or authorized.

## Executable validation boundary

No new build/test/Clippy/workflow execution is required for this audit-only mutation.

C02e Tranche 6 canonical executable validation remains the latest executable evidence for the unchanged production source.

If a later tranche adds a backend dependency or production adapter, that tranche requires its own dependency review and implementation validation.

## Classification

`C02F_B_PRESELECTION_AUDIT_COMPLETE / NO_EXISTING_CONCRETE_DURABLE_LIVE_OWNER_BACKEND_FOUND / ACCEPTED_STATE_CAS_PRECEDENT_REUSABLE_AS_SEMANTIC_REFERENCE_ONLY / TEST_MEMORY_STORE_NOT_PRODUCTION_BACKEND / REGISTRY_AND_CONTROL_PLANE_NOT_PERSISTENCE_PROVIDERS / CONCRETE_PROVIDER_SELECTION_REQUIRES_SEPARATE_AUTHORITY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`
