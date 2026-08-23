# Phase 152 C02f-CI — Agent Reachability Authority Admission Seam Source Materialization Staging

Status: `MATERIALIZING / SOURCE_ONLY / CH_ORDERING_PRESERVED / PRIVATE_ADMISSION_CONSTRUCTOR / EXACT_CG_ERROR_PRESERVED / NO_MAIN_WIRING / NO_RUNTIME_ACTIVATION / NO_REMOTE_NETWORKING / NO_RETRY_WORKER / NO_RECOVERY / NO_PRWF_INIT / NO_R1_R4 / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-23  
Repository: `Gersi365/prw-executor-private`

## Authorization and scope

C02f-CI materializes the smallest Agent-owned source seam that encodes the C02f-CH reachability-authority admission ordering.

The seam is source-only. It does not wire the C02f-CG bootstrap into `main.rs`, the existing local Linux runtime, service-manager readiness, a remote listener, a background task, retry/reconnect logic, recovery, PRWF initialization, R1-R4 effects, deployment, or merge.

## Exact prerequisite

C02f-CI derives only from closed C02f-CH:

- branch: `phase-152-c02f-ch-agent-reachability-authority-runtime-readiness-ordering-selection-staging`;
- head: `84769db9ea0ac236340c7a2a1e1d58a957f6eaf1`;
- tree: `412ee4cd44768e7d80c01946b24321eba43e6115`;
- gate: `C02F_CH_AGENT_REACHABILITY_AUTHORITY_RUNTIME_READINESS_ORDERING_SELECTED`.

CH selected reachability authority bootstrap as a prerequisite for future authority-dependent remote/reachability admission while preserving the existing base local Agent bootstrap and local IPC `Ready` meaning.

## Existing source reused without redesign

CI must reuse the C02f-CG Agent facade exactly as the only bootstrap input:

```rust
bootstrap_reachability_live_owner_authority_from_systemd_credentials()
```

That existing async function:

- reads the fixed C02f-CE systemd credential set when invoked;
- fails with the bounded existing `ReachabilityAuthorityCustodyBootstrapError` taxonomy;
- performs provider bootstrap through the C02f-CF facade when custody succeeds;
- returns only `ReachabilityLiveOwnerComposedAsyncAuthority`.

CI must not reimplement custody, provider bootstrap, TLS/client construction, or bridge authority composition.

## Selected admission token

CI materializes one Agent-owned opaque admission token:

```rust
pub struct ReachabilityLiveOwnerAuthorityAdmission
```

The token owns exactly one:

```rust
ReachabilityLiveOwnerComposedAsyncAuthority
```

The authority field remains private.

There is no public constructor from a composed authority. This prevents later external callers from manufacturing the selected admission token without going through the Agent admission seam.

The token may expose only a bounded immutable authority reference required for later, separately gated integration:

```rust
pub const fn authority(&self) -> &ReachabilityLiveOwnerComposedAsyncAuthority
```

CI does not add an alternate authority implementation or raw provider capability.

## Selected source-only bootstrap/admission operation

CI materializes one async Agent operation:

```rust
pub async fn bootstrap_and_admit_reachability_live_owner_authority_from_systemd_credentials()
    -> Result<
        ReachabilityLiveOwnerAuthorityAdmission,
        ReachabilityAuthorityCustodyBootstrapError,
    >
```

When and only when this function is invoked, it must:

1. call the existing C02f-CG bootstrap facade exactly once;
2. propagate the exact existing C02f-CG bounded error unchanged;
3. on success, wrap the returned composed authority in the private-constructor admission token;
4. return no raw etcd client/store, secret material, endpoint/TLS configuration, runtime handle, or recovery object.

The function does not itself perform live-owner acquisition, currentness, release, recovery, PRWF initialization, or R1-R4 effects.

## Fail-closed semantics

CI intentionally introduces no new fallback or degraded-success variant.

The public result shape is binary:

```text
Err(existing CG custody/provider-bootstrap error)
    = reachability authority not admitted

Ok(ReachabilityLiveOwnerAuthorityAdmission)
    = composed authority successfully admitted at this source boundary
```

A custody/provider-bootstrap failure must never produce an admission token.

CI must not:

- swallow the CG error;
- remap it to success/degraded authority;
- create a local/in-memory fallback authority;
- retry provider bootstrap;
- spawn detached work;
- infer remote readiness from existing local Agent `Ready` state.

## Dependency boundary

CI must not require a Cargo manifest or lockfile change.

All required crates/types are already normal dependencies of `prw-agent` on the CH base:

- `prw-remote-bridge` for the composed authority type;
- existing Agent C02f-CG module for the bootstrap function and bounded error.

If the compiler proves any new dependency is required, CI must stop for re-selection rather than widening the tranche.

## Test boundary

CI tests must not execute the async bootstrap/admission operation.

Focused tests may prove only compile/type properties, including the exact function signature.

Tests must not:

- read `$CREDENTIALS_DIRECTORY`;
- create synthetic credential files;
- connect to etcd;
- construct fake provider endpoints;
- create a Tokio runtime merely to poll the bootstrap future;
- execute acquisition/currentness/release;
- execute recovery or R1-R4 effects.

## `main.rs` and runtime boundary

CI must leave `crates/prw-agent/src/main.rs` byte-stable.

The new operation remains uninvoked by:

- the standalone Agent binary;
- the current local Linux bootstrap;
- listener readiness;
- worker scheduling;
- service-manager readiness;
- any remote/public transport.

Therefore CI source materialization does not change process startup, local IPC `Ready`, or production runtime behavior.

## Recovery / PRWF boundary

An admitted composed authority is not proof that recovery/currentness prerequisites are satisfied.

CI does not issue recovery epochs, initialize missing fence/PRWF state, or manufacture currentness.

## R1-R4 boundary

The admission token proves only that the configured reachability authority was successfully constructed through the selected custody/provider/bridge path.

It does not authorize externally visible effects. R1-R4 effect-side enforcement remains separately gated.

## Expected permanent file scope

CI should require exactly:

1. this contract;
2. one new Agent source module, expected at `crates/prw-agent/src/reachability_authority_admission.rs`;
3. one `crates/prw-agent/src/lib.rs` module export.

Expected permanent net scope excludes:

- `Cargo.toml` changes;
- `Cargo.lock` changes;
- workflow changes;
- `main.rs` changes;
- predecessor source mutations.

## Byte-stability requirements

At final CI head, the following predecessor surfaces must remain byte-stable:

- `crates/prw-agent/src/main.rs`;
- C02f-CG `crates/prw-agent/src/reachability_authority_custody_bootstrap.rs`;
- C02f-CF `crates/prw-agent/src/reachability_authority_bootstrap.rs`;
- C02f-BZ `crates/prw-agent/src/reachability_authority_composition.rs`;
- C02f-CE custody source;
- root Cargo manifests/lockfile except no changes are expected anywhere in that set.

## Stop conditions

CI must stop for re-selection if implementation would require:

- a public constructor that can manufacture admission from an arbitrary composed authority;
- a new error taxonomy unrelated to the exact CG failure;
- Cargo dependency or lockfile changes;
- `main.rs` or Linux runtime wiring;
- retry/reconnect/background execution;
- service-manager readiness publication;
- remote/public networking;
- local status semantic changes;
- recovery/PRWF execution;
- R1-R4 effect activation;
- raw provider-client/store or secret exposure.

## Validation requirements

The CI gate may be claimed only after:

1. exact CH ancestry is reverified;
2. CH -> CI compare proves only the intended contract/new-module/lib-export delta;
3. Cargo metadata/lockfile remain byte-stable;
4. exact-head canonical Rust validation reaches terminal success for locked graph, rustfmt, Clippy, tests, and build;
5. Android/AD/AE workflow states are reported exactly as triggered/skipped/not-triggered;
6. PR remains draft/open/unmerged;
7. Drive audit is uploaded and raw-read back byte-exact;
8. rolling status is appended in place with the complete prior prefix byte-identical.

## Gate

On successful validation and evidence closeout:

`C02F_CI_AGENT_REACHABILITY_AUTHORITY_ADMISSION_SEAM_SOURCE_MATERIALIZED`

This gate means only that the source-level admission token and callable bootstrap/admission seam exist and validate. It does not mean the running Agent invokes them or that remote readiness/effects are active.
