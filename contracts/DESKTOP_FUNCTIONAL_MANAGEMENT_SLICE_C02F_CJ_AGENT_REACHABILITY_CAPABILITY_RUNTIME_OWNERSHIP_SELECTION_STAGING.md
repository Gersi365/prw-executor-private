# Phase 152 C02f-CJ — Agent Reachability Capability Runtime Ownership Selection Staging

Status: `SELECTED / DOCUMENTATION_ONLY / CI_ADMISSION_PRESERVED / AGENT_OWNED_LIFETIME_BOUNDARY / NO_EXISTING_REMOTE_CONSUMER_REUSED / NO_MAIN_WIRING / LOCAL_READY_UNCHANGED / NO_REMOTE_NETWORKING / NO_RUNTIME_TASK / NO_RETRY_RECONNECT / NO_RECOVERY / NO_PRWF_INIT / NO_R1_R4 / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-23  
Repository: `Gersi365/prw-executor-private`

## Authorization and scope

C02f-CJ selects the first legitimate ownership boundary for the C02f-CI `ReachabilityLiveOwnerAuthorityAdmission` token.

This checkpoint is documentation-only. It does not materialize source, invoke provider bootstrap, create a runtime/executor, alter `main.rs`, change the existing local Linux Agent lifecycle/readiness, open a remote listener, schedule work, retry/reconnect, execute recovery/PRWF initialization, activate R1-R4 effects, deploy, or merge.

## Exact prerequisite

C02f-CJ derives only from closed C02f-CI:

- branch: `phase-152-c02f-ci-agent-reachability-authority-admission-seam-source-materialization-staging`;
- head: `c80917d7029154c306e32a6988265a01a3d810c2`;
- tree: `00f829dc460539a6c136e954cc745d5d4f23ac73`;
- gate: `C02F_CI_AGENT_REACHABILITY_AUTHORITY_ADMISSION_SEAM_SOURCE_MATERIALIZED`.

CI materialized an opaque Agent-owned admission token that owns exactly one `ReachabilityLiveOwnerComposedAsyncAuthority`, plus the bounded bootstrap/admission function that can create the token only after the selected custody/provider/bootstrap path succeeds.

## Exact-head consumer audit

The CI exact-head Agent source contains no concrete remote/public transport runtime, remote session manager, remote listener, reachability worker, or reachability operation owner that can legitimately receive and retain the admission token.

The existing `linux_agent_session_bridge.rs` is explicitly a local Unix-domain composition seam over `UnixStream` and `AuthenticatedLocalLinuxSession`. It performs no remote/public transport role and therefore is not a valid reachability-authority consumer.

The existing `linux_*` production runtime/readiness/worker modules remain the established local Agent runtime surface. C02f-CH already selected that local `Ready` semantics must remain independent from reachability-authority admission.

The bridge-owned `ReachabilityLiveOwnerComposedAsyncAuthority` in `prw-remote-bridge` is a provider-neutral asynchronous authority implementation. Its contract explicitly leaves process-level executor/runtime ownership outside the bridge and prohibits construction of provider-specific etcd clients/endpoints/runtime there.

Therefore no already-materialized concrete runtime consumer should be retrofitted in CJ.

## Selected first consumer

The first legitimate consumer of `ReachabilityLiveOwnerAuthorityAdmission` is a new, bounded **Agent-owned reachability capability lifetime owner**.

This owner is the process-level ownership boundary for the reachability capability only. It is not the existing base/local Agent runtime and it is not a transport/session implementation.

The selected owner must:

1. consume exactly one already-created `ReachabilityLiveOwnerAuthorityAdmission` by value;
2. retain that token for the lifetime of the future reachability capability;
3. expose only bounded mutable authority access required by separately gated live-owner operations;
4. preserve the bridge-owned `ReachabilityLiveOwnerAsyncAuthority` semantics rather than reimplementing acquisition/currentness/release;
5. expose no raw etcd client/store, credentials, endpoint/TLS configuration, recovery object, runtime handle, or alternate provider capability;
6. perform no I/O merely by construction.

A source tranche may name this owner `ReachabilityAuthorityRuntimeOwner` or an equivalently narrow Agent-owned type. The semantic boundary, not the spelling, is selected here.

## Why Agent owns the lifetime boundary

Ownership remains in `prw-agent` because the Agent is the selected process-level composition root for the future reachability capability.

`prw-control-plane` continues to own provider/TLS/client construction and provider-specific etcd behavior.

`prw-remote-bridge` continues to own provider-neutral live-owner acquisition/currentness/release semantics and the composed async authority implementation.

Neither lower layer should acquire process-level lifetime ownership, remote readiness semantics, transport ownership, or Agent lifecycle responsibility.

## Authority access shape

The CI token currently exposes an immutable reference to the composed authority. Because the async bridge trait intentionally requires `&mut self` for acquisition/currentness/release, the next source tranche may add the minimum private/module-bounded consuming or mutable-access seam necessary for the Agent-owned lifetime owner.

Any such accessor must remain narrower than exposing provider internals and must not permit manufacturing an admission token from an arbitrary authority.

The selected direction is:

```text
systemd credential custody
        -> control-plane provider bootstrap
        -> bridge composed async authority
        -> Agent admission token
        -> Agent-owned reachability capability lifetime owner
        -> separately gated authority-dependent operations
```

## Readiness ordering

C02f-CH remains authoritative:

- existing local Agent startup and local IPC `Ready` do not depend on reachability authority;
- failure to bootstrap/admit authority means the future reachability capability is unavailable;
- only successful authority admission may permit construction of the selected reachability capability lifetime owner;
- construction of that owner still does not mean remote transport is ready;
- remote/public transport readiness remains a later, separate gate.

No local status-schema or service-manager readiness change is selected by CJ.

## Fail-closed semantics

CJ selects no degraded/fallback authority.

If custody/provider bootstrap fails, no admission token exists and therefore no reachability capability lifetime owner can be constructed through the selected path.

If a future authority operation returns unavailable/ambiguous state, the bridge authority error remains authoritative. The Agent owner must not manufacture currentness, substitute local cache state, or convert ambiguity into success.

## Lifecycle boundary

The selected owner is only a lifetime/ownership boundary.

CJ does not select or materialize:

- when `main.rs` invokes bootstrap;
- a Tokio or other executor/runtime;
- a background authority worker;
- retry/backoff/reconnect policy;
- watch subscriptions;
- provider-client reload/rotation;
- remote connection/session lifecycle;
- public listener binding;
- NAT traversal or relay behavior;
- service-manager notification;
- shutdown release policy.

Those are runtime/transport lifecycle concerns and require separate gates if later authorized.

## Recovery / PRWF boundary

The lifetime owner does not imply that recovery/currentness prerequisites are complete for a peer lifecycle.

CJ does not issue recovery epochs, initialize missing PRWF/fence state, or execute recovery operations.

## R1-R4 boundary

Possession of the selected lifetime owner proves only that a configured authority was admitted and retained at the Agent reachability-capability boundary.

It does not authorize externally visible effects. Effect-side stale-fence enforcement at R1-R4 remains mandatory and separately gated.

## Selected next source tranche

The next bounded tranche may materialize only the selected Agent-owned lifetime owner and the minimum admission-token access needed to let that owner delegate to the already-composed bridge authority.

That tranche must remain source-only and side-effect-free on construction. It must not wire the owner into `main.rs`, local readiness, a remote listener, worker scheduler, executor, retry/reconnect policy, recovery, PRWF initialization, or R1-R4 effects.

## Expected permanent file scope

C02f-CJ requires exactly this contract file.

Expected permanent net scope excludes all source, manifests, lockfiles, workflows, `main.rs`, and predecessor contract mutations.

## Byte-stability requirements

At final CJ head, the following CI surfaces must remain byte-stable:

- `crates/prw-agent/src/main.rs`;
- `crates/prw-agent/src/reachability_authority_admission.rs`;
- `crates/prw-agent/src/reachability_authority_custody_bootstrap.rs`;
- `crates/prw-agent/src/reachability_authority_bootstrap.rs`;
- `crates/prw-agent/src/reachability_authority_composition.rs`;
- `crates/prw-agent/src/lib.rs`;
- `crates/prw-agent/Cargo.toml`;
- root `Cargo.lock`;
- reachability custody/control-plane/remote-bridge source.

## Stop conditions

CJ must stop for re-selection if evidence proves an already-materialized concrete remote/reachability runtime consumer exists and semantically owns this lifetime, or if selecting the owner would require changing local `Ready`, `main.rs`, provider ownership, bridge semantics, runtime/task behavior, remote networking, recovery, PRWF initialization, R1-R4 activation, deployment, or merge.

The exact-head audit found no such contradiction.

## Validation requirements

The CJ gate may be claimed only after:

1. exact CI ancestry is reverified;
2. CI -> CJ compare proves exactly one documentation-only contract addition;
3. source/manifests/lock/workflows remain byte-stable;
4. exact-head canonical Rust validation reaches its terminal state as triggered by the docs-only delta;
5. Android/AD/AE workflow states are reported exactly as triggered/skipped/not-triggered;
6. PR remains draft/open/unmerged;
7. Drive audit is uploaded and raw-read back byte-exact;
8. rolling status is appended in place with the complete prior prefix byte-identical.

## Gate

On successful validation and evidence closeout:

`C02F_CJ_AGENT_REACHABILITY_CAPABILITY_RUNTIME_OWNERSHIP_SELECTED`

This gate means only that the first legitimate ownership boundary of the admitted reachability authority is selected. It does not mean the owner source exists, the running Agent constructs it, or remote networking/readiness/effects are active.
