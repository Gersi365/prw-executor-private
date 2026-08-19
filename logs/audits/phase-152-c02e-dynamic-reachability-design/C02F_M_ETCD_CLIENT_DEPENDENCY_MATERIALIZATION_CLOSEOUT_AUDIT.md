# Phase 152 C02f-M — etcd-client Dependency Materialization Closeout Audit

Status: `ETCD_CLIENT_DEPENDENCY_MATERIALIZED / ETCD_CLIENT_0_19_0_EXACT_PIN / DEFAULT_FEATURES_DISABLED / PROTOC_BUILD_PREREQUISITE_MATERIALIZED_IN_CANONICAL_CI / CARGO_LOCK_CARGO_GENERATED / CANONICAL_WORKSPACE_VALIDATION_PASS / TLS_PROFILE_DEFERRED / KEY_SCHEMA_ENCODING_DEFERRED / TRANSACTION_MAPPING_DEFERRED / CLUSTER_DEPLOYMENT_DEFERRED / RUNTIME_ACTIVATION_DEFERRED / PRODUCTION_RUST_SOURCE_BYTE_STABLE / NO_NETWORK_RUNTIME_ACTIVATION`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Frozen C02d head: `857583b25ed1206317641a93fd8f927819c954d8`
C02f-L predecessor head: `23772aae0782da8f1ecaf7683bfcf357c3543cbd`
C02f-L predecessor tree: `3b3ad56bb18712413ef1164a3fb17236adc1c1d7`
C02f-M materialization commit: `bf177140df18e7f0183838b01cfbf8bc7471b120`
C02f-M materialization tree: `b1eb52fae7faa48cd6b57573d8bae07272704b31`
Review date: `2026-08-19`

## Purpose

C02f-L selected `etcd-client 0.19.0` as the Rust client for the already selected etcd v3.7 backend in the T3 shared control-plane live-owner authority domain while explicitly deferring dependency materialization and feature/TLS policy.

C02f-M materializes only the dependency surface required to compile that selected client and proves the resulting workspace with canonical executable validation. It does not implement the live-owner etcd adapter, select the key/value schema, choose external `u128` fence encoding, select a TLS profile, configure endpoints or credentials, choose cluster topology, issue network requests, or activate runtime authority behavior.

## Inherited architecture locks

C02f-M does not reopen any prior architecture decision. The following remain authoritative:

- T3 shared control-plane authority is selected;
- cross-host replacement is required;
- etcd v3.7 is the selected live-owner authority backend;
- `etcd-client 0.19.0` is the selected Rust client;
- exact authority namespace is `DeviceId + TransportIdentity`;
- `ReachabilityLiveOwnerFence` remains a PRW-owned strictly monotonic non-zero logical `u128`;
- provider revisions do not silently replace the PRW logical fence;
- live-owner replacement must be atomic;
- authority ambiguity, indeterminate mutation outcome and no-quorum conditions fail closed;
- stale release cannot clear a newer owner;
- recovery must preserve a durable logical-fence high-water mark;
- clocks, TTL, Lease and Watch are not primary stale-owner safety authority;
- future R1-R4 reachability effect sinks must reject stale fences at or atomically with their effect boundary.

## Exact dependency policy materialized

The production `prw-control-plane` manifest now contains exactly:

```toml
etcd-client = { version = "=0.19.0", default-features = false }
```

This locks the selected client to exact crate version `0.19.0` and deliberately disables default features.

No etcd TLS feature is enabled at this checkpoint.

### Upstream feature correction

Pre-mutation review of the upstream `etcd-client` `v0.19.0` manifest established that the previously discussed `tls-aws-lc` feature does not exist in this release. The actual release exposes TLS routes including the crate's `tls`/Tonic-ring path, `tls-roots`, and an OpenSSL path.

Therefore C02f-M does **not** invent or select a nonexistent feature. TLS profile selection remains a later explicit gate.

This correction changes only the deferred feature-policy understanding; it does not reopen the already approved `etcd-client 0.19.0` client selection.

## Build prerequisite discovered and materialized

The upstream `etcd-client 0.19.0` build script invokes `tonic_prost_build::compile_protos(...)` over bundled etcd protobuf definitions. A `protoc` executable is therefore a real build-time prerequisite for this selected dependency.

The canonical general Rust validation workflow was minimally extended to install:

```text
protobuf-compiler
```

and to record:

```text
protoc --version
```

No production runtime package dependency, daemon, service or listener was introduced by this CI prerequisite.

## Cargo.lock provenance

The root `Cargo.lock` was **not** hand-edited.

Because the local execution environment available to this review had no Cargo toolchain and no external package-network resolution, dependency resolution was performed on isolated GitHub Actions probe branches. Cargo itself generated the candidate lockfile.

The generated lockfile was reproduced independently and then persisted as a Git object before being referenced by the active materialization tree.

Canonical generated lockfile evidence:

- byte size: `76635`;
- SHA-256: `93a3f77eb06c521497d12a837538ac3bafd4a7a9ac03709955ce613a98cedd6b`;
- Git blob SHA: `875af4399166d0a8ae9eba6422e809b89a2ceed8`.

The lockfile includes:

- `etcd-client 0.19.0`;
- registry checksum `ef5da6e9a6ae89f4a91f80ba1caae45a5a924397a19947e18f5121a43285e9bc`;
- Tonic/Prost transport and code-generation dependencies required by that crate;
- `prw-control-plane` dependency linkage to `etcd-client` and existing `prw-core`.

The lockfile does not by itself enable etcd TLS or activate any endpoint.

## Isolated probe evidence

### Probe PR #40

An isolated draft/non-merge probe was created from exact C02f-L to generate and validate the candidate dependency graph without mutating the active branch first.

PR #40 was explicitly validation-only and was closed unmerged.

### Initial general-workspace probe — run #714

Run:

- workflow: `PRW Rust Validation`;
- run number: `714`;
- run ID: `32276707700`;
- job ID: `96145737394`;
- synthetic merge: `6ecf486f8352fd5bdf189892e48f567abd79027b`.

Result:

- Cargo dependency resolution: PASS;
- generated lockfile artifact: PASS;
- rustfmt: PASS;
- Clippy: FAIL;
- tests/build: not reached.

The failure was not a Rust API or dependency-version incompatibility. `etcd-client 0.19.0` failed its build script because `protoc` was absent from the runner environment.

This failure established the `protobuf-compiler` build prerequisite rather than invalidating the client selection.

### Scoped corrected dependency probe — run #3

Run:

- workflow: `PRW Phase 152 C02m etcd-client Dependency Probe`;
- run number: `3`;
- run ID: `32279092798`;
- job ID: `96153551391`;
- synthetic merge checked out: `8ba7507aa4cda34b62bec9f508166237847f58fc`.

Result: PASS.

The run installed `protobuf-compiler`, generated the dependency graph, reproduced the same lockfile SHA-256, and passed:

- rustfmt;
- `cargo clippy --locked -p prw-control-plane --all-targets --all-features -- -D warnings`;
- `cargo test --locked -p prw-control-plane --all-targets`;
- `cargo build --locked -p prw-control-plane --all-targets`.

`prw-control-plane` tests passed `23/23`.

Toolchain evidence included:

- Ubuntu `24.04.4`;
- rustc `1.97.1`;
- Cargo `1.97.1`;
- rustfmt `1.9.0-stable`;
- Clippy `0.1.97`;
- `libprotoc 3.21.12`.

### Persistence probe — run #5

Run:

- run number: `5`;
- run ID: `32279791171`;
- job ID: `96155728254`.

Result: PASS.

This probe regenerated the same Cargo lockfile, persisted that exact generated file on the probe branch, and then passed formatting, control-plane Clippy, tests and build.

The resulting probe bot commit was:

`a00248e2a384d09f4958d3689934cf18d089bf83`

The persisted lockfile Git blob was:

`875af4399166d0a8ae9eba6422e809b89a2ceed8`

PR #40 was then closed unmerged. Probe-only validation workflow changes did not enter the active branch.

## Active materialization commit

After the probe evidence passed, the active branch was reverified immediately before mutation at exact C02f-L:

- head `23772aae0782da8f1ecaf7683bfcf357c3543cbd`;
- tree `3b3ad56bb18712413ef1164a3fb17236adc1c1d7`.

An atomic active tree was built from the C02f-L tree with exactly three changed paths:

1. `.github/workflows/phase-001-rust-validation.yml`;
2. `Cargo.lock`;
3. `crates/prw-control-plane/Cargo.toml`.

The resulting materialization commit is:

- commit `bf177140df18e7f0183838b01cfbf8bc7471b120`;
- tree `b1eb52fae7faa48cd6b57573d8bae07272704b31`;
- parent `23772aae0782da8f1ecaf7683bfcf357c3543cbd`;
- message `phase 152 c02f-m: materialize etcd-client dependency`.

Exact C02f-L → C02f-M materialization diff:

- `.github/workflows/phase-001-rust-validation.yml`: `+2 / -0`;
- `Cargo.lock`: `+615 / -5`;
- `crates/prw-control-plane/Cargo.toml`: `+1 / -0`.

No `.rs` production source changed.

## Canonical active-workspace validation

### Validation PR #41

A fresh validation-only branch was created from exact active materialization commit `bf177140df18e7f0183838b01cfbf8bc7471b120`.

The branch differed from the active base by one audit marker only. Draft PR #41 used:

- base branch: `phase-152-c02e-dynamic-reachability-design`;
- exact base SHA: `bf177140df18e7f0183838b01cfbf8bc7471b120`;
- validation head: `be748631e22ad17ed1b04149aa937c7e65d5c627`;
- changed files: `1`;
- additions: `5`;
- deletions: `0`.

PR #41 was closed unmerged after successful validation.

### Canonical run #722

Authoritative validation evidence:

- workflow: `PRW Rust Validation`;
- run number: `722`;
- run ID: `32280348419`;
- job ID: `96157393482`;
- synthetic merge: `38d194920eaa1187a7c1942f56f0abee92c7665a`;
- result: `SUCCESS`.

Every canonical step passed:

1. checkout;
2. installation of native build prerequisites including `protobuf-compiler`;
3. toolchain recording including `protoc`;
4. `cargo metadata --locked --no-deps --format-version 1`;
5. `cargo fmt --all -- --check`;
6. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
7. `cargo test --locked --workspace --all-targets`;
8. `cargo build --locked --workspace --all-targets`.

Canonical toolchain/runtime evidence:

- runner OS: Ubuntu `24.04.4`;
- runner image: `ubuntu-24.04`, image version `20260816.277.1`;
- rustc `1.97.1 (8bab26f4f 2026-07-14)`;
- Cargo `1.97.1 (c980f4866 2026-06-30)`;
- rustfmt `1.9.0-stable`;
- Clippy `0.1.97`;
- `protoc`: `libprotoc 3.21.12`;
- GTK `4.14.5`;
- libadwaita `1.5.0`.

Canonical Clippy explicitly compiled `etcd-client v0.19.0`, Tonic `0.14.6`, Tonic-Prost `0.14.6`, `prw-control-plane`, and the rest of the PRW workspace before finishing successfully under `-D warnings`.

Canonical workspace tests passed. Relevant retained safety suites included:

- `prw-agent`: `373/373`;
- `prw-control-plane`: `23/23`;
- live-owner unit tests: `5/5`;
- exact-peer live-owner namespace tests: `4/4`;
- reachability owner production seam tests: `6/6`;
- Phase 141 reachability integration tests: `2/2`.

The final full-workspace `cargo build --locked --workspace --all-targets` also passed.

## Production Rust source byte stability

C02f-M materializes dependency/build infrastructure only. The key production Rust sources remain byte-identical to C02f-L:

- `crates/prw-control-plane/src/lib.rs` blob `668619338b1e085a4ac42bc27f793014e8a03df2`;
- `crates/prw-remote-bridge/src/reachability_live_owner.rs` blob `ad21a7cc4369e1f5b9953f72c5b12bf64e50a404`.

Therefore no etcd adapter, network operation, authority mutation or effect-sink activation exists yet.

## Explicitly deferred after C02f-M

C02f-M does **not** select or implement:

- etcd TLS feature/profile;
- certificate or CA representation;
- mTLS credential distribution;
- endpoint configuration;
- connection bootstrap;
- key namespace layout;
- `DeviceId` key encoding;
- `TransportIdentity` key encoding;
- value schema;
- external `u128` fence encoding;
- transaction/CAS request mapping;
- indeterminate-commit recovery algorithm;
- retry/re-observation policy;
- recovery high-water storage layout;
- snapshot/restore procedure;
- etcd cluster member count;
- AZ/region topology;
- self-hosted versus managed deployment;
- runtime authority adapter;
- background Watch task;
- Lease/keepalive task;
- outbound etcd network I/O;
- R1-R4 production effect-sink activation.

## Safety interpretation of dependency materialization

Materializing a client dependency does not make the client an authority by itself.

Any future adapter must still enforce the already locked PRW semantics independently of convenience APIs exposed by the crate:

- exact namespace `DeviceId + TransportIdentity`;
- linearizable/current authority operations only;
- atomic compare-and-replace semantics;
- PRW-owned strictly monotonic non-zero logical `u128` fencing;
- no stale-read/currentness authority;
- no Watch/currentness authority;
- no Lease/TTL safety authority;
- ambiguity and no-quorum fail closed;
- no blind retry of an indeterminate mutation;
- stale release isolation;
- recovery high-water preservation;
- stale-fence rejection at the actual R1-R4 effect boundary.

## Closed conclusion

C02f-M closes the dependency-materialization gate with the authoritative result:

`ETCD_CLIENT_0_19_0_DEPENDENCY_MATERIALIZED_AND_CANONICALLY_VALIDATED`

The dependency and its build prerequisite are now part of the repository's compile-time surface, while all authority schema, TLS/security topology, endpoint/deployment configuration, transaction implementation, network behavior and runtime activation remain deferred to later explicit checkpoints.
