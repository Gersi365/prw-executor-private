# Phase 152 C02f-A — Durable Live-Owner Backend Authority Gate

Status: `CONTRACT_LOCK / DOCS_ONLY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / BACKEND_TECHNOLOGY_UNSELECTED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Authoritative predecessor branch: `phase-152-c02e-dynamic-reachability-design`

Authoritative predecessor head: `6acbab52f393dc35d722ac9b129c117a02edbce2`

GitHub repository: `powercode365-dotcom/prw-executor-private`

GitHub repository ID: `1334911207`

## Purpose

C02f-A opens only the architecture authority gate for the future concrete durable live-owner backend required by the validated C02e fencing seam.

It does not select a database, consensus system, lease service, storage engine, wire schema, runtime process topology, network protocol, or deployment target. It does not mutate production Rust source.

The purpose of this checkpoint is to lock the backend safety contract before any implementation/provider choice is permitted.

## Authoritative C02e inputs

C02f-A inherits the executable C02e Tranche 6 authority semantics without reinterpretation:

- live-owner namespace is the exact peer lifecycle `DeviceId + TransportIdentity`;
- `ReachabilityLiveOwnerFence` is a non-zero ordered `u128` logical/in-memory generation representation;
- replacement ownership in the same exact peer namespace requires a strictly newer fencing generation;
- stale owners must not regain authority from cached state;
- authority ambiguity or unavailability fails closed;
- release is a liveness operation and is not relied upon for safety;
- future side effects requiring live-owner authority must be fenced at the side-effect boundary;
- candidate-publication freshness remains a separate authority and must not be reused as the live-owner fence.

C02e executable validation remains the source/API authority baseline. C02f-A adds no new executable claim.

## C02f-A backend safety contract

Any future concrete live-owner backend MUST preserve all of the following properties.

### 1. Exact namespace key

Authority state is keyed by the exact current peer lifecycle:

`DeviceId + TransportIdentity`

The key MUST NOT be replaced by or derived from:

- IP address or port;
- connectivity endpoint;
- candidate ID;
- request ID;
- session ID;
- publication freshness token;
- transient NAT/relay path;
- Linux PID/UID/GID.

A transport-identity rotation creates a distinct authority namespace. Registry currentness remains the higher-level authority for deciding whether an old transport identity is still admissible.

### 2. Durable monotonic fencing generation

For one exact peer namespace, every replacement owner MUST receive a fence strictly greater than every fence previously issued for that namespace.

A concrete backend MUST NOT reset, wrap, reuse, or reconstruct an older generation after:

- process restart;
- host restart;
- backend failover;
- retry;
- stale release;
- partial failure;
- recovery from persisted state.

The logical value remains non-zero `u128`. This checkpoint does not select persistence byte order, serialization, record schema, database column type, or wire representation.

### 3. Atomic replacement authority

Ownership acquisition/replacement must linearize against the current durable authority state for the same exact peer namespace.

A future backend must provide an operation equivalent in safety to:

`observe current -> allocate strictly newer fence -> atomically install replacement -> return installed grant`

No caller may be told that a grant is current before the durable replacement decision has succeeded.

Split read-then-write behavior without a backend-enforced compare/transaction/serialization boundary is insufficient.

### 4. Stale-owner rejection

Once a newer grant is installed for the same exact peer namespace, every older grant is permanently stale for authority purposes.

An older grant MUST NOT become current again because:

- the newer owner releases;
- the newer owner crashes;
- a cache expires;
- a worker retries;
- a backend reconnects;
- local memory is reconstructed.

If currentness cannot be established from authoritative backend state, the decision is fail-closed.

### 5. Release cannot clear newer authority

Release must be conditional on the exact current grant/fence for the namespace.

A stale release MUST NOT delete, overwrite, clear, or otherwise invalidate a newer owner's authority.

Release remains optional for safety. Failure to release may affect liveness/cleanup, but safety depends on monotonic replacement and fenced side effects rather than successful release.

### 6. Recovery preserves fencing history

Recovery must preserve enough durable authority state to prevent fence reuse or rollback.

A backend that loses the last durable generation for a namespace MUST NOT silently restart allocation from a lower value. Ambiguous recovery is an authority failure and must fail closed until a separately reviewed recovery procedure re-establishes monotonicity.

### 7. Side-effect fencing obligation

A successful acquisition alone is not sufficient to authorize a later distributed side effect indefinitely.

Any future side-effecting sink that can race with owner replacement must reject stale fences at, or atomically with, the side-effect boundary.

C02f-A does not select the concrete sink protocol, transaction mechanism, socket/task ownership implementation, or persistence API used to enforce this obligation.

### 8. No clock-based safety dependency

C02f-A does not authorize wall-clock time, TTL expiry, heartbeat cadence, lease duration, or clock synchronization as the primary safety mechanism.

A later liveness design may add leases/heartbeats, but stale-owner safety must continue to derive from authoritative monotonic fencing, not clock assumptions.

### 9. Bounded failure semantics

Concrete backend operations must surface failures explicitly. At minimum, future implementation must distinguish:

- unavailable authority;
- ambiguous/indeterminate authority result;
- stale/non-current grant;
- successful current grant/replacement;
- release that did not match the current grant.

Provider/library/internal backend error text must not become an unbounded remote protocol surface merely because the backend exposes it.

## Backend technology remains unselected

C02f-A intentionally does not choose among possible implementation families such as:

- SQL database transaction/row-lock/CAS designs;
- key-value transactional stores;
- consensus-backed stores;
- dedicated lease/coordination services;
- single-process local storage.

Any future selection must be justified against the exact safety contract above, including crash/restart/failover behavior and side-effect fencing.

A provider choice is not authorized by this document.

## Persistence and wire representation remain unselected

The following remain separate decisions:

- fence persistence schema;
- serialization format;
- endianness/byte encoding;
- database column/key layout;
- storage migration strategy;
- wire message kind/payload;
- authenticated transport for backend coordination;
- replay/freshness semantics for any future remote authority request.

The existing `NonZeroU128` source representation must not be mistaken for permission to invent an external storage/wire format in C02f-A.

## Production mutation boundary

C02f-A is documentation/audit only.

This checkpoint MUST NOT modify:

- `crates/prw-remote-bridge/src/reachability_live_owner.rs`;
- `crates/prw-remote-bridge/src/root.rs`;
- any other production Rust source;
- `Cargo.toml` files;
- `Cargo.lock`;
- canonical validation workflows;
- Agent/bootstrap/runtime wiring;
- network/NAT/relay/QUIC source;
- systemd/service-manager source;
- deployment/signing/credential source;
- database schema/migrations.

The post-commit verification requirement is exact Git-byte stability for production source relative to predecessor head `6acbab52f393dc35d722ac9b129c117a02edbce2`.

## Validation boundary

No new build, format, Clippy, test, workflow dispatch, runtime, network, deployment, or privileged action is required or authorized by this docs-only checkpoint.

C02e Tranche 6 executable validation remains the latest executable authority evidence for the unchanged production source.

If a future C02f tranche mutates production source, it requires its own separately scoped implementation validation evidence.

## Gate to the next C02f tranche

A later C02f tranche may select or stage a concrete backend only after it documents, at minimum:

1. exact durable state model per `DeviceId + TransportIdentity` namespace;
2. atomic monotonic fence allocation/replacement mechanism;
3. crash/restart/failover recovery semantics;
4. stale-release behavior;
5. side-effect fencing integration point;
6. bounded failure mapping;
7. persistence representation and migration implications, if any;
8. why the chosen backend satisfies the C02e/C02f-A authority contract without runtime activation by implication.

## Current classification

`C02F_A_DURABLE_LIVE_OWNER_BACKEND_AUTHORITY_CONTRACT_LOCKED / EXACT_PEER_NAMESPACE_PRESERVED / STRICTLY_MONOTONIC_DURABLE_FENCE_REQUIRED / STALE_OWNER_AND_STALE_RELEASE_FAIL_CLOSED / SIDE_EFFECT_FENCING_REQUIRED / CLOCK_NOT_SAFETY_AUTHORITY / BACKEND_STORAGE_WIRE_RUNTIME_UNSELECTED / DOCS_ONLY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`
