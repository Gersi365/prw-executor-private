# Phase 152 C02f-K — etcd Rust Client / Dependency Selection Readiness Audit

Status: `CLIENT_DEPENDENCY_SELECTION_READINESS_COMPLETE / ETCD_V3_7_PROVIDER_LOCK_INHERITED / ETCD_CLIENT_0_19_0_PREFERRED_FOR_SELECTION_REVIEW / DIRECT_TONIC_BINDINGS_ELIGIBLE_HIGH_MAINTENANCE / OFFICIAL_GO_CLIENT_SIDECAR_REQUIRES_SEPARATE_ARCHITECTURE_APPROVAL / CLIENT_LIBRARY_NOT_SELECTED / ETCD_3_7_COMPATIBILITY_PROOF_REQUIRED / TOKIO_ALIGNMENT_PRESENT / TLS_AWS_LC_ALIGNMENT_AVAILABLE / TONIC_PROST_NEW_DEPENDENCY_SURFACE / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Exact predecessor head: `e4927dbdea8679ce42a1e97defd6cbd5fa560fc0`
Exact predecessor tree: `6f7f2b914ba7b3a2a1b15633a69453981dffcf1a`
Predecessor checkpoint: `C02f-J etcd v3.7 provider selection lock`
Review date: `2026-08-19`

## Purpose

C02f-J selected etcd v3.7 as the backend for the already selected T3 shared control-plane live-owner authority domain. It explicitly deferred the Rust client library/dependency decision as the first implementation gate after provider selection.

C02f-K closes the client/dependency questions that can be answered without mutating Cargo manifests, selecting a concrete crate, selecting schema/encoding, contacting an etcd endpoint, creating credentials, choosing cluster topology or activating runtime behavior.

This checkpoint is selection-readiness only. It does **not** authorize adding `etcd-client`, `tonic`, `prost`, generated protobuf bindings or any other production dependency.

## Inherited locks

The following remain authoritative and are not reopened:

- T3 shared control-plane authority is selected;
- cross-host replacement is required;
- etcd v3.7 is the selected backend family;
- exact authority namespace is `DeviceId + TransportIdentity`;
- live-owner fence is a PRW-owned strictly monotonic non-zero logical `u128`;
- live-owner replacement must be atomic;
- ambiguity, unknown authority outcome and no-quorum conditions fail closed;
- stale release cannot clear newer authority;
- recovery must preserve a durable high-water mark and cannot make an older fence authoritative;
- Watch, Lease/TTL, clocks and heartbeats are not primary stale-owner safety authority;
- R1-R4 reachability side effects must reject stale fences at or atomically with their effect boundary;
- client choice cannot weaken the preceding semantics.

## Exact repository baseline

At predecessor head `e4927dbdea8679ce42a1e97defd6cbd5fa560fc0`:

### Workspace

Root `Cargo.toml`:

- workspace resolver: `2`;
- Rust edition: `2024`;
- workspace forbids unsafe code;
- workspace Clippy `all`, `pedantic` and `nursery` are warnings.

Root manifest blob:

`fbbd220348e3008b38d4cfb1ec5721f8c12199e2`

### `prw-control-plane`

`crates/prw-control-plane/Cargo.toml` currently has exactly one production dependency:

`prw-core = { path = "../prw-core" }`

There is no etcd, gRPC, Tonic, Prost or TLS dependency in the control-plane crate today.

Control-plane manifest blob:

`a940a7eb23764452b9ef1fb24b8d20a91ba712c9`

### Existing Tokio / TLS precedent

`crates/prw-remote-transport/Cargo.toml` already pins:

- `tokio = 1.53.1`;
- `rustls = 0.23.43`;
- `aws-lc-rs = 1.18.0`;
- QUIC using `rustls-aws-lc-rs`.

That is not permission to reuse the remote-transport dependency policy mechanically, but it is repository evidence that Tokio and AWS-LC-backed rustls are already accepted technologies in another production networking boundary.

Remote-transport manifest blob:

`37055b7371cd6325438d8a2cbff00bd37773f6f6`

## External source policy

Current dependency facts were reviewed on `2026-08-19` using upstream/primary project material and package-generated documentation.

Primary sources reviewed:

- etcd upstream repository and v3 client documentation: `https://github.com/etcd-io/etcd`;
- etcd v3 official Go client documentation: `https://github.com/etcd-io/etcd/blob/main/client/v3/README.md`;
- etcd v3.7 changelog/releases: `https://github.com/etcd-io/etcd/releases`, `https://github.com/etcd-io/etcd/blob/main/CHANGELOG/CHANGELOG-3.7.md`;
- Rust `etcd-client` source repository: `https://github.com/etcdv3/etcd-client`;
- package-generated `etcd-client` documentation/version metadata: `https://docs.rs/crate/etcd-client/latest`.

No package was downloaded or executed in the PRW repository during this audit.

## Current etcd v3.7 line

The selected architecture contract names etcd v3.7 as a backend family, not one immutable patch build.

As of this review, upstream etcd release material shows v3.7.1 as the current v3.7 patch release after v3.7.0.

This observation does not silently rewrite C02f-J to `v3.7.1`. A later deployment/version-pin checkpoint must select and validate an exact server patch level.

The client library decision therefore must target the etcd v3 API semantics required by the selected v3.7 family and must be validated against the exact server patch eventually chosen.

## Candidate C1 — `etcd-client` Rust crate

Reviewed package line: `etcd-client 0.19.0`.

Classification:

`PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`

### Positive fit

The crate presents an asynchronous etcd v3 API client backed by Tokio and Tonic.

Its published API surface includes:

- KV;
- transactions / compare operations required to express CAS-style ownership replacement;
- Watch;
- Lease;
- Auth;
- Maintenance;
- Cluster;
- Lock;
- Election;
- Namespace support.

For PRW live-owner authority, the relevant capability is the KV transaction/compare surface. Watch and Lease availability do not change the C02f-J rule that neither may become primary safety authority.

The current package line uses the following dependency families:

- Tokio `1`;
- Tonic `0.14`;
- Prost `0.14`;
- tonic-prost `0.14`;
- Tower `0.5`.

The current crate declares an MSRV of Rust 1.80. The already existing PRW remote-transport crate declares Rust 1.97.1, so the crate's documented MSRV does not create an obvious repository-level compiler floor conflict.

### TLS fit

The upstream Rust client exposes rustls TLS features including an AWS-LC-backed option (`tls-aws-lc`) as well as native/webpki root choices.

That is directionally compatible with PRW's existing use of rustls + AWS-LC in remote transport.

This is only compatibility evidence. It does not select:

- `tls-aws-lc` versus another TLS feature;
- trust roots;
- mTLS certificate model;
- server-name policy;
- credential loading/distribution;
- endpoint identity;
- certificate rotation.

Those remain part of the later TLS/auth/trust-boundary gate.

### Material dependency expansion

Selecting `etcd-client` would expand `prw-control-plane` from a provider-neutral one-dependency crate into a networked gRPC client dependency graph.

At minimum, the selected client package transitively introduces or relies on Tonic/Prost/Tower/Tokio surfaces that are not currently dependencies of `prw-control-plane`.

This is a real dependency-boundary decision and must be explicit. Provider selection alone does not authorize it.

### Compatibility proof gap

The `etcd-client` project documentation states that its test setup targets etcd 3.5.

PRW has selected etcd v3.7.

Therefore, package availability and API shape are insufficient to claim production compatibility for PRW. Before executable integration can be accepted, PRW must validate the exact selected client version against an exact etcd v3.7 patch server for the operations PRW actually depends on.

Required compatibility proof must include at least:

1. linearizable Range/Get behavior used for authoritative re-observation;
2. Txn compare/success/failure semantics used for atomic ownership replacement;
3. delete/release comparison semantics proving stale release isolation;
4. gRPC deadline/transport failure mapping;
5. endpoint failover behavior under one unreachable member;
6. no-quorum behavior and bounded failure return;
7. TLS/mTLS behavior under the later selected trust model;
8. behavior after leader movement or member loss;
9. no accidental use of serializable reads for authority;
10. no silent retry policy that can convert an indeterminate mutation into an assumed success.

Until those are executable, C1 is preferred for review but not proven for PRW's selected server line.

## Candidate C2 — PRW-owned generated Tonic bindings

Classification:

`ELIGIBLE / HIGH_MAINTENANCE / NOT_SELECTED`

etcd v3 exposes a gRPC API. PRW could theoretically compile or vendor the relevant protobuf API and build a narrow Tonic client around only the KV/Auth operations needed by live-owner authority.

Advantages:

- smallest conceptual API surface can be exposed to the PRW adapter;
- complete control over retry classification and response mapping;
- no high-level client behavior is inherited accidentally;
- PRW can make linearizable-only semantics explicit in its own adapter.

Costs and risks:

- PRW owns protobuf/build synchronization;
- PRW owns endpoint/balancing behavior;
- PRW owns TLS channel construction;
- PRW owns gRPC compatibility tracking;
- PRW owns error/status interpretation;
- PRW owns version compatibility with etcd v3.7 and future upgrades;
- maintenance surface is materially larger than using a purpose-built client.

C2 is not required by any existing architecture lock. The extra ownership burden is not justified merely to avoid a third-party client dependency.

## Candidate C3 — sidecar/service using the official Go client

Classification:

`OUTSIDE_CURRENT_CLIENT_LIBRARY_GATE / REQUIRES_SEPARATE_ARCHITECTURE_APPROVAL`

etcd upstream identifies `clientv3` as the official Go etcd v3 client.

Using it from this Rust workspace would require a process/service/IPC boundary or another language integration mechanism rather than a normal Rust library dependency.

That would alter runtime topology, deployment surface, trust boundaries and failure modes. Generic continuation and the C02f-J client-library gate do not authorize that expansion.

C3 is therefore not a candidate that may be selected under this audit. It can only be reconsidered through a separate architecture decision.

## Candidate comparison

| Criterion | C1 `etcd-client 0.19.0` | C2 PRW Tonic bindings | C3 Go client sidecar |
|---|---|---|---|
| Rust in-process integration | strong | strong | no |
| etcd v3 KV/Txn surface | provided | buildable | official client provides |
| Existing package maintenance burden | lower | PRW-owned high | external service high |
| Tokio alignment | yes | yes if Tonic chosen | not relevant |
| Tonic/Prost dependency | transitive/direct | direct | IPC stack instead |
| Rustls/AWS-LC path | available | PRW-owned | separate Go TLS stack |
| Explicit PRW retry control | adapter must constrain client | maximum | IPC + Go client behavior |
| v3.7 PRW compatibility proof | required | required | still required end-to-end |
| New process/deployment boundary | no | no | yes |
| Eligible for simple dependency selection | yes | yes | no |

## Required adapter boundary regardless of client

No selected client may leak raw provider semantics into `ReachabilityLiveOwnerAuthority` callers.

A production adapter must translate provider/client outcomes into the existing bounded authority contract.

At minimum:

- definitive successful transaction -> committed PRW authority result;
- definitive compare failure -> stale/not-current result;
- transport timeout after a mutation may have reached quorum -> `RecoveryRequired` / indeterminate, not automatic retry success;
- no quorum / unavailable authority -> fail closed;
- malformed or contradictory provider response -> fail closed;
- stale release compare failure -> must not delete newer authority;
- currentness must use an authoritative linearizable observation/transaction path;
- Watch events may accelerate cache invalidation but cannot establish currentness;
- Lease/TTL expiry cannot prove an older owner is harmless.

## Retry policy constraint

Client convenience retry is not correctness authority.

For read-only linearizable observations, bounded retry may be allowed later if the final error remains explicit.

For mutating ownership transactions, a timeout or transport loss after dispatch can be indeterminate. The adapter must not blindly replay a mutation if replay could allocate or commit a different fence or make success ambiguous.

The recovery pattern remains:

1. return an explicit indeterminate/recovery-required result;
2. perform a fresh authoritative re-observation;
3. reconcile observed durable state with the attempted operation;
4. only issue another mutation when the PRW state machine proves it safe.

## Fence ownership constraint

No client or etcd metadata field becomes the logical `ReachabilityLiveOwnerFence` by convenience.

In particular, the adapter must not silently substitute:

- etcd global revision;
- key create revision;
- key mod revision;
- lease ID;
- transaction header revision;
- watcher revision;
- server clock/time;
- client request sequence.

The external representation of the PRW-owned `u128` fence remains deferred to the later schema/encoding checkpoint.

## Dependency pinning requirement

If a Rust client is later selected, the dependency checkpoint must explicitly lock:

- package name;
- exact accepted version policy;
- Cargo feature set;
- default-features policy;
- TLS feature/provider;
- direct versus transitive Tokio/Tonic/Prost ownership;
- MSRV compatibility;
- license acceptance;
- vulnerability/advisory review at selection time;
- reproducible `Cargo.lock` result.

No floating dependency policy should become architecture by accident.

## Selection-readiness conclusion

Repo-native evidence and current upstream package evidence support the following ranking:

1. `etcd-client 0.19.0` — preferred candidate for an explicit dependency-selection checkpoint;
2. PRW-owned Tonic bindings — technically eligible but higher maintenance and no current necessity;
3. official Go client through a sidecar/service — outside the current gate and requires a new architecture approval.

The preference for C1 is not a dependency selection.

Before C1 can be selected, the approving checkpoint must accept the third-party Rust client dependency surface and must preserve the mandatory PRW adapter constraints above.

Before C1 can become executable production integration, PRW must additionally prove compatibility against the exact selected etcd v3.7 patch line and later-selected TLS/auth/schema/recovery semantics.

## Mutation boundary

C02f-K is audit-only.

It must not change:

- production Rust source;
- any `Cargo.toml`;
- `Cargo.lock`;
- provider lock;
- schema or key encoding;
- live-owner fence encoding;
- cluster topology;
- credentials/endpoints;
- runtime wiring;
- network behavior.

No build, rustfmt, Clippy, test or workflow run is required solely for this audit because executable source is unchanged.

The latest executable evidence remains C02e Tranche 6 canonical PASS.

## Next gate

The next architecture/dependency decision is explicit client selection.

A mechanically sufficient approval would need to identify the chosen Rust dependency path and preserve the following constraints:

- etcd v3.7 backend lock remains unchanged;
- linearizable KV/Txn authority only;
- PRW-owned monotonic `u128` fence;
- fail-closed no-quorum/ambiguity;
- indeterminate mutation outcomes require re-observation/reconciliation;
- Watch/Lease/TTL remain non-authoritative for stale-owner safety;
- TLS/auth/schema/cluster/runtime remain deferred unless separately approved.

Until that approval exists, no Cargo mutation is authorized.