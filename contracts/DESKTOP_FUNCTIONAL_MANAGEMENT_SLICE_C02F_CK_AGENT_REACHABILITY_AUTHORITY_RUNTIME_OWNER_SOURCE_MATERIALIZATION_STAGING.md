# Phase 152 C02f-CK — Agent Reachability Authority Runtime Owner Source Materialization Staging

Status: `MATERIALIZING / SOURCE_ONLY / CJ_OWNERSHIP_PRESERVED / AGENT_OWNED_ADMISSION_LIFETIME / CRATE_BOUNDED_MUTABLE_AUTHORITY_ACCESS / SAME_MODULE_OPACITY / NO_MAIN_WIRING / LOCAL_READY_UNCHANGED / NO_RUNTIME_ACTIVATION / NO_REMOTE_NETWORKING / NO_RETRY_RECONNECT / NO_RECOVERY / NO_PRWF_INIT / NO_R1_R4 / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-23  
Repository: `Gersi365/prw-executor-private`

## Authorization and scope

C02f-CK materializes the C02f-CJ-selected Agent-owned lifetime boundary for one successfully admitted reachability live-owner authority.

This tranche is source-only. Construction performs no I/O and the source remains uninvoked by `main.rs`, the existing local Linux runtime, service-manager readiness, any remote/public listener, background worker, retry/reconnect loop, recovery path, PRWF initialization, or R1-R4 effect path.

## Exact prerequisite

C02f-CK derives only from closed C02f-CJ:

- branch: `phase-152-c02f-cj-agent-reachability-capability-runtime-ownership-selection-staging`;
- head: `3d51dcea646e49444ac686e97e9547a16eaef9c0`;
- tree: `197356b146c9a5bdd9140c76e209b8943abdec87`;
- gate: `C02F_CJ_AGENT_REACHABILITY_CAPABILITY_RUNTIME_OWNERSHIP_SELECTED`.

CJ proved that no already-materialized remote runtime consumer exists and selected a bounded Agent-owned reachability-capability lifetime owner as the first legitimate consumer of the C02f-CI admission token.

## Selected source type

CK adds the owner in the existing Agent admission module so the admission token's private authority field remains module-private:

```rust
pub struct ReachabilityAuthorityRuntimeOwner {
    admission: ReachabilityLiveOwnerAuthorityAdmission,
}
```

The owner consumes one already-created admission token by value and retains that token unchanged for its own lifetime.

The constructor is pure ownership composition:

```rust
pub const fn new(admission: ReachabilityLiveOwnerAuthorityAdmission) -> Self
```

No alternate constructor from a composed authority, provider client, store, credentials, endpoints, TLS configuration, or recovery object is added.

## Minimum mutable authority seam

The bridge async authority intentionally requires mutable authority state for acquisition/currentness/release. Because the new owner lives in the same module as the private admission token, CK does not widen the admission token API.

The runtime owner exposes only a crate-bounded mutable reference:

```rust
pub(crate) const fn authority_mut(
    &mut self,
) -> &mut ReachabilityLiveOwnerComposedAsyncAuthority
```

The method reaches the private `admission.authority` field only from the same module. It is not public outside `prw-agent` and exposes no provider-specific etcd client/store, credentials, endpoint/TLS configuration, runtime handle, or recovery object.

This is the minimum source seam needed for a future, separately gated Agent-owned reachability operation consumer to use the already-admitted bridge authority.

## Dependency boundary

CK requires no Cargo manifest or lockfile changes.

All named types are already present in the existing admission module through the normal `prw-remote-bridge` dependency.

CK deliberately does not implement the bridge async authority trait on the Agent owner. Acquisition/currentness/release semantics remain owned by `prw-remote-bridge`; CK only owns process-level lifetime/capability custody.

## Readiness and runtime boundary

CK preserves C02f-CH/CJ ordering:

- base local Agent bootstrap/local IPC `Ready` is unchanged;
- authority bootstrap/admission remains uninvoked by the running Agent;
- owner construction can occur only after an admission token exists;
- owner construction performs no provider I/O;
- owner existence is not remote transport readiness;
- remote/public transport/runtime wiring remains separately gated.

## Fail-closed semantics

CK creates no fallback or degraded authority.

No admission token means no selected runtime owner can be constructed through the selected constructor.

The owner does not catch, remap, suppress, or manufacture provider/bridge authority results. It owns only the admitted authority capability.

## Recovery / PRWF / R1-R4 boundary

CK does not issue recovery epochs, initialize missing fence/PRWF state, prove peer currentness, or activate externally visible effects.

Possession of `ReachabilityAuthorityRuntimeOwner` is not authorization for R1-R4 effects; effect-side fence enforcement remains separate.

## Test boundary

Tests prove only source/type properties without creating credentials, provider endpoints, async runtimes, sockets, or network effects.

Focused tests prove the exact admission-token -> runtime-owner constructor shape and the crate-bounded mutable composed-authority accessor shape. No authority Future is polled.

## Expected permanent file scope

CK should require exactly:

1. this contract;
2. one bounded update to `crates/prw-agent/src/reachability_authority_admission.rs` adding the owner, its pure constructor, its crate-bounded mutable authority accessor, and compile/type-only tests.

Expected permanent net scope excludes `lib.rs`, all Cargo manifests, `Cargo.lock`, workflows, `main.rs`, Linux runtime/readiness modules, control-plane source, custody source, and remote-bridge source.

## Byte-stability requirements

At final CK head the following must remain byte-stable relative to CJ:

- `crates/prw-agent/src/main.rs`;
- `crates/prw-agent/src/lib.rs`;
- `crates/prw-agent/Cargo.toml`;
- root `Cargo.lock`;
- C02f-CG custody bootstrap facade;
- C02f-CF bootstrap composition;
- C02f-BZ authority composition;
- C02f-CE custody source;
- `prw-control-plane` provider implementation;
- `prw-remote-bridge` live-owner authority implementation;
- existing local Linux runtime/readiness/session sources.

## Stop conditions

CK must stop for re-selection if implementation requires a public arbitrary-authority admission constructor, widening the admission token's public API, Cargo dependency/lockfile change, `main.rs` or local readiness wiring, runtime/executor/task creation, provider ownership changes, remote/public networking, retry/reconnect, recovery/PRWF execution, R1-R4 activation, deployment, or merge.

## Validation requirements

The CK gate may be claimed only after:

1. exact CJ ancestry is reverified;
2. CJ -> CK compare proves exactly the intended contract plus bounded admission-module delta;
3. Cargo manifests/lockfile and protected predecessor surfaces remain byte-stable;
4. exact-head canonical Rust validation is terminal FULL PASS;
5. Android/AD/AE states are reported exactly as triggered/skipped/not-triggered;
6. PR remains draft/open/unmerged;
7. Drive audit is uploaded and raw-read back byte-exact;
8. rolling status append preserves the complete prior prefix byte-identically.

## Gate

On successful validation and evidence closeout:

`C02F_CK_AGENT_REACHABILITY_AUTHORITY_RUNTIME_OWNER_SOURCE_MATERIALIZED`

This gate means the admitted authority now has its selected Agent-owned lifetime boundary in source. It does not mean the running Agent bootstraps or uses it, and it does not activate remote transport/readiness/effects.
