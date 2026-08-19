# Phase 152 C02f-J — etcd v3.7 Provider Selection Lock

Status: `PROVIDER_SELECTION_LOCK / T3_SHARED_CONTROL_PLANE_AUTHORITY_SELECTED / ETCD_V3_7_SELECTED / LINEARIZABLE_KV_TRANSACTION_AUTHORITY / FAIL_CLOSED_AMBIGUITY_NO_QUORUM / PRW_OWNED_MONOTONIC_U128_FENCE / RECOVERY_HIGH_WATER_SAFETY_REQUIRED / SINK_SIDE_STALE_FENCE_REJECTION_REQUIRED / CLIENT_LIBRARY_DEFERRED / KEY_SCHEMA_ENCODING_DEFERRED / CLUSTER_DEPLOYMENT_DEFERRED / RUNTIME_ACTIVATION_DEFERRED / DOCS_ONLY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Exact predecessor head: `dfdb9704b8e899bee3caff582c622d8cf414f320`
Exact predecessor tree: `3de0f8b6663fb0989e73767d861c6945a8832806`
Predecessor checkpoint: `C02f-I etcd provider selection readiness audit`
Approval date: `2026-08-19`

## Purpose

C02f-J materializes the architecture owner's explicit provider-selection approval for the Phase 152 distributed live-owner authority.

The approved backend is etcd v3.7 inside the already selected T3 shared control-plane authority domain.

This checkpoint locks the provider family and the safety semantics that must govern its use. It does not select a Rust client library, key layout, external encoding, cluster member count, region/AZ topology, endpoint, certificate format, credential distribution mechanism, deployment platform, runtime adapter or production activation path.

## Explicit architecture approval

The project architecture owner approved the following direction on `2026-08-19`:

- select etcd v3.7 as the T3 shared control-plane live-owner authority backend;
- preserve fail-closed ambiguity and no-quorum behavior;
- use linearizable KV transaction authority;
- preserve a PRW-owned monotonic logical `u128` live-owner fence;
- preserve recovery high-water safety;
- preserve sink-side stale-fence rejection;
- defer client library selection;
- defer schema/key/external encoding selection;
- defer cluster deployment selection;
- defer runtime activation.

C02f-J records exactly that approval and does not extend it by inference.

## Provider selection

### etcd v3.7

Classification: `SELECTED`.

The initial T3 shared control-plane live-owner authority backend is etcd v3.7.

Provider selection means future implementation work for this authority must target etcd v3.7 semantics unless a separately approved architecture change supersedes this checkpoint.

It does not mean that an etcd cluster exists, that any endpoint is reachable, that any crate dependency has been added, that any key has been written, or that production authority has been activated.

### PostgreSQL 18

Classification: `NOT_SELECTED`.

PostgreSQL remains outside the selected initial live-owner authority path. Reintroducing it as the live-owner authority backend requires a separate architecture decision.

### CockroachDB v26.2

Classification: `NOT_SELECTED`.

CockroachDB remains outside the selected initial live-owner authority path. Reintroducing it as the live-owner authority backend requires a separate architecture decision.

## Inherited placement lock

C02f-G remains authoritative:

- T3 shared control-plane authority is selected;
- cross-host replacement is required;
- standalone Agent-local T2 authority is eliminated for the initial architecture;
- T4 dedicated authority service is not selected;
- all contenders for one exact namespace must serialize through one shared authority domain.

The selected etcd backend must implement T3. Provider convenience must not silently move authority back to an Agent-local process or into a separately authoritative service domain.

## Exact live-owner namespace

The authoritative namespace remains exactly:

`DeviceId + TransportIdentity`

Neither etcd key placement nor encoding may redefine identity.

The following remain transient location/routing data and are not identity:

- IP address;
- port;
- endpoint;
- NAT mapping;
- candidate;
- relay route;
- QUIC initiator role;
- process-local task identifier.

Any future key schema must preserve the exact logical namespace even if the physical etcd key representation differs.

## Linearizable authority rule

All currentness-sensitive authority operations must be based on an etcd operation mode whose semantics provide the required linearization point for the live-owner state transition.

The implementation must preserve one atomic authority transition for the exact namespace:

1. observe the currently authoritative generation/state;
2. establish that the contender's preconditions still hold;
3. allocate or commit a strictly newer PRW live-owner fence;
4. replace the prior authoritative owner state atomically;
5. return an unambiguous authoritative result before effects are permitted.

The concrete transaction shape, compare operands and key layout are deferred, but the resulting behavior must be equivalent to one linearizable conditional state transition.

A stale or non-linearizable observation must never be used as sufficient proof of current authority.

## Reads and currentness

A currentness decision that can enable an effect must use the selected backend in a mode that supplies the required authoritative consistency.

The following are not sufficient by themselves to establish currentness:

- locally cached state;
- stale/serializable reads;
- Watch event delivery;
- client-side last-seen revision;
- heartbeat freshness;
- lease TTL;
- wall-clock age;
- a one-time check performed before later asynchronous effects.

Caches and watches may accelerate observation or invalidation, but they cannot replace the authoritative linearization point.

## PRW-owned fence semantics

`ReachabilityLiveOwnerFence(NonZeroU128)` remains the logical PRW fencing generation.

The fence must remain:

- non-zero;
- strictly ordered for one exact namespace;
- strictly newer on every accepted replacement;
- never reused;
- never rolled back by restart, failover or restore;
- independent from wall-clock time.

Provider metadata is not automatically the PRW fence.

In particular, a future implementation must not silently substitute any etcd-native revision, version, lease identifier or watch cursor for the PRW logical `u128` fence unless a later explicit contract proves the representation preserves all locked monotonicity and recovery properties.

External storage/wire encoding of the `u128` fence remains deferred.

## Atomic replacement

For one exact namespace, a replacement must be committed as a single authoritative conditional transition.

A contender cannot become current merely because it successfully wrote a value after separately reading an older value.

A read-then-write sequence without an atomic compare/transaction boundary is insufficient.

The old owner must become stale as part of the same authoritative ordering that grants the newer owner.

## Stale release isolation

A release operation from an older fence must not delete, clear, overwrite or otherwise weaken a newer authoritative owner.

Any future release transaction must condition its mutation on the exact authoritative generation/owner state it intends to release.

If the state has already advanced, stale release must be rejected or become a no-op that cannot affect the newer owner.

## Ambiguity and no-quorum behavior

Safety policy is locked as fail closed.

If the control plane cannot establish an unambiguous authoritative etcd result because of conditions including:

- lost quorum;
- partition;
- unavailable members;
- request timeout;
- transport interruption;
- authentication/TLS failure;
- indeterminate transaction outcome;
- restored state whose monotonicity cannot yet be proven;

then the caller must not infer Current and must not enable currentness-sensitive reachability effects.

Availability mechanisms may improve liveness, but they may not weaken this rule.

## Indeterminate mutation outcomes

A timeout or broken connection after a mutating request does not authorize the caller to guess whether the transition committed.

The future client adapter must expose a bounded explicit outcome model that distinguishes at least:

- authoritative success;
- authoritative rejection/stale conflict;
- unavailable/ambiguous outcome requiring recovery or re-observation;
- invalid/corrupt state.

On ambiguity, the system must re-establish state through an authoritative linearizable observation before granting effects or issuing a logically conflicting transition.

Blind retry behavior that can violate the PRW state machine is prohibited.

## Recovery high-water safety

Provider durability alone is not sufficient to satisfy the live-owner fence contract.

The system must prove that restore/recovery cannot make a previously issued older generation authoritative again and cannot allocate a generation less than or equal to the maximum generation that may already have been observed by an effect sink.

Therefore a future recovery design must include a PRW high-water safety proof.

At minimum, production activation after restore must remain blocked until the implementation can prove one of the following equivalent properties:

- the restored authority state already preserves a monotonic floor greater than or equal to every generation that could previously have been issued; or
- a separately durable high-water source is reconciled and the next generation is forced strictly above it; or
- another explicitly reviewed mechanism proves equivalent no-reuse/no-rollback semantics.

Snapshot restore, revision restoration, revision bumping or cluster recreation must not be assumed to satisfy PRW fence monotonicity without that proof.

If high-water state is ambiguous, authority remains fail closed.

## Side-effect fencing

Selecting etcd does not weaken the effect-boundary rule.

A valid authority grant is necessary but not sufficient for a racing reachability effect.

Every future currentness-sensitive effect sink must either:

- atomically validate the exact fence at the effect boundary; or
- carry the fence into a sink that rejects stale generations before or atomically with the effect.

A one-time `currentness()` result obtained earlier in an asynchronous workflow is insufficient.

The future R1-R4 reachability sink classes remain subject to this rule:

- R1 traversal UDP transmit boundary;
- R2 traversal timer/retransmit/task ownership;
- R3 selected-path QUIC connection establishment/retirement;
- R4 reachability-owned connection/task registry replacement.

Candidate publication freshness remains a separate authority domain and is not replaced by etcd live-owner fencing.

Application capability effects are not automatically reclassified as live-owner sinks; any coupling must be reviewed explicitly.

## Cross-host replacement

Cross-host replacement remains required.

A surviving authorized contender on another host must be able to establish a strictly newer owner through the shared control-plane etcd authority domain when higher-level identity/session/registry policy permits replacement.

The failed or partitioned prior host must not regain authority merely by reconnecting with stale local state.

Host-local caches, task registries and adapters may improve performance but cannot become an independently authoritative namespace.

## Security boundary

Provider selection establishes that future production authority traffic will cross an etcd trust boundary inside the T3 control-plane authority domain.

Before production activation, the later deployment/security checkpoint must explicitly define and validate:

- authenticated client identity;
- encrypted transport requirements;
- credential issuance and rotation;
- endpoint trust roots;
- authorization scope for authority keys;
- secret storage/custody;
- auditability of privileged backend access.

This checkpoint does not select certificate formats, PKI topology, user/role names or credential distribution mechanisms.

## Deferred client-library decision

No Rust etcd client library is selected by C02f-J.

A later dependency checkpoint must evaluate the concrete client against the selected etcd v3.7 contract, including:

- supported etcd API compatibility;
- linearizable read and transaction controls;
- compare/transaction expressiveness;
- TLS/auth support;
- timeout and retry semantics;
- explicit treatment of indeterminate outcomes;
- error typing;
- dependency/security surface;
- maintenance/support status.

No crate dependency may be added merely because etcd is now selected.

## Deferred key/schema/encoding decision

C02f-J does not select:

- key prefix;
- binary/text key encoding;
- value schema;
- serialization format;
- `DeviceId` encoding;
- `TransportIdentity` encoding;
- `u128` external representation;
- owner-token representation;
- tombstone/release representation;
- migration/versioning format.

Those choices must preserve the logical namespace, atomic replacement semantics and recovery high-water contract.

## Deferred cluster deployment decision

C02f-J does not select:

- member count;
- host placement;
- AZ distribution;
- region distribution;
- managed versus self-hosted operation;
- discovery/bootstrap method;
- endpoint addressing;
- snapshot cadence;
- compaction policy;
- backup storage;
- disaster-recovery runbook;
- maintenance windows;
- monitoring/alerting stack.

The later deployment topology must preserve one authoritative linearization domain and fail closed when quorum/authority is ambiguous.

## Deferred runtime activation

This checkpoint performs no runtime activation.

It does not authorize:

- outbound etcd connections;
- listener creation;
- production endpoint configuration;
- credential loading;
- Agent bootstrap wiring;
- control-plane service activation;
- background watch tasks;
- lease keepalive tasks;
- network I/O;
- production effect execution.

## Source mutation boundary

C02f-J is a contract-only architecture checkpoint.

Production Rust source, Cargo manifests and `Cargo.lock` must remain byte-stable relative to C02f-I.

No build, rustfmt, Clippy, test or workflow run is required solely for this docs-only provider-selection lock because executable source is unchanged.

The latest executable validation evidence remains the already closed C02e Tranche 6 canonical PASS until a later executable checkpoint changes production source.

## Implementation gates after C02f-J

Provider selection is now closed. The next implementation-oriented work must not reopen the provider comparison absent contradictory evidence or explicit redesign.

Before a production etcd adapter can be staged, the remaining decisions must be handled as explicit checkpoints, including:

1. client-library/dependency selection;
2. exact key/value schema and external fence encoding;
3. transaction/CAS state-machine mapping;
4. indeterminate-outcome/retry/re-observation behavior;
5. recovery high-water persistence and restore procedure;
6. cluster deployment/availability topology;
7. TLS/auth/credential trust boundary;
8. concrete R1-R4 effect-sink fence propagation and rejection;
9. executable tests proving stale-owner, stale-release, failover and recovery behavior;
10. runtime activation only after the preceding gates pass.

## Locked conclusion

C02f-J closes the provider-selection gate with the following authoritative result:

`ETCD_V3_7_SELECTED_AS_T3_SHARED_CONTROL_PLANE_LIVE_OWNER_AUTHORITY_BACKEND`

The selected backend must be used only under these locked semantics:

- linearizable KV transaction authority;
- fail-closed ambiguity and no-quorum behavior;
- exact `DeviceId + TransportIdentity` namespace;
- PRW-owned strictly monotonic non-zero logical `u128` fence;
- atomic replacement;
- permanent stale-owner rejection;
- stale-release isolation;
- recovery high-water safety;
- sink-side stale-fence rejection at the effect boundary;
- clocks/TTL/Watch not primary safety authority.

Client library, key/schema/encoding, cluster deployment and runtime activation remain explicitly deferred.
