# C03e-LC Production Durable Capability Higher-Owner Arc Custody Modular Source Layout Reselection Staging

Status: `SELECTION_STAGING`
Date: `2026-09-05`

Gate:

`C03E_LC_PRODUCTION_DURABLE_CAPABILITY_HIGHER_OWNER_ARC_CUSTODY_MODULAR_SOURCE_LAYOUT_RESELECTED`

Intended closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_HIGHER_OWNER_ARC_CUSTODY_MODULAR_SOURCE_LAYOUT_RESELECTION`

## 1. Purpose

C03e-LC is a transport-safe source-layout reselection after the evidence-closed C03e-LB ownership selection.

LB correctly selected the first production owner for the KX/KZ/LA outer:

`Arc<ProductionDurableCapabilityAuthority>`

and selected one additive, dormant, non-cloneable aggregate that wraps the existing production/reachability/requester-rendezvous aggregate by value beside exactly one outer durable-authority Arc.

LB selected `crates/prw-agent/src/linux_bootstrap.rs` as the immediate one-file source location. The connected GitHub write surface available at LC provides whole-file replacement and Git-object creation, but no bounded line/patch insertion primitive. The exact LB `linux_bootstrap.rs` blob is approximately 101 KiB. Reconstructing that full blob only to add a small dormant aggregate would enlarge transport risk without changing semantics.

LC does not change the LB semantic selection. It reselects only a modular two-path source layout that leaves the large exact LB `linux_bootstrap.rs` blob byte-identical and places the new aggregate in a small crate-private Linux-only sibling module registered from `lib.rs`.

## 2. Exact predecessor authority

Repository:

`Gersi365/prw-executor-private`

Exact LB branch:

`phase-152-c03e-lb-production-durable-capability-higher-owner-arc-custody-selection`

Exact LB head:

`d538cce9e3302a393631d720715fcfcc1b0d0187`

Exact LB tree:

`db54a2d48409c0783c90d8a2ef586b5d5cb2fd90`

LB PR #438 remains draft/open/unmerged and is evidence-closed.

LB Rust validation #1574 / run `33957579847` completed successfully on exact LB head, including locked dependency graph, formatting, Clippy, tests, workspace build, and cleanup.

Namespace audit before LC creation found no `phase-152-c03e-lc*` successor.

## 3. Frozen LB source evidence

### 3.1 Large Linux bootstrap blob

Path:

`crates/prw-agent/src/linux_bootstrap.rs`

Exact LB blob:

`f2a87c45bd8d96bf1555b65210531c94c722eb2f`

The file contains the existing non-cloneable:

`LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<P, D, T, F, C, R, E>`

and the current active/legacy runtime path. LC selects no mutation to this blob.

### 3.2 Crate-root module registry

Path:

`crates/prw-agent/src/lib.rs`

Exact LB blob:

`8b50cb5c5c2e711648cba8424ed2015be5606360`

The crate root already gates `linux_bootstrap` to Linux and registers dormant production custody modules through narrow crate-private declarations. This is the selected small registration surface for the modular successor.

### 3.3 New module absence proof

At exact LB head, this path does not exist:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

LC therefore selects an additive new source module rather than relocation or replacement of existing source.

## 4. Semantic selection remains LB

LC preserves the exact LB-selected type name:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<P, D, T, F, C, R, E>`

The aggregate still owns exactly two logical values:

1. one existing `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<P, D, T, F, C, R, E>` by value; and
2. one outer `Arc<ProductionDurableCapabilityAuthority>`.

The constructor still consumes one raw `ProductionDurableCapabilityAuthority` by value and performs exactly one:

`Arc::new(capability_authority)`

No LB ownership, identity, authorization, lifecycle, or runtime law changes.

## 5. Selected modular source layout

The immediate source materialization after LC may change exactly two paths.

### Path A — crate-root registration

`crates/prw-agent/src/lib.rs`

Selected mutation is only one Linux-gated crate-private module registration for:

`production_durable_capability_higher_owner_custody`

with a narrowly scoped `dead_code` allowance documenting that the module is intentionally dormant pending separately gated propagation/caller migration.

The registration must remain under:

`#[cfg(target_os = "linux")]`

because the new module depends on the Linux-only `crate::linux_bootstrap` aggregate.

No existing module declaration, public API, constant, type, test, or implementation in `lib.rs` is selected for mutation.

### Path B — new higher-owner custody module

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

The new file owns only the LB-selected dormant aggregate and its side-effect-free constructor.

It may import exactly the types required to express that aggregate:

- `std::sync::Arc`;
- `crate::linux_bootstrap::LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs`;
- `crate::production_durable_registry_runtime_custody::ProductionDurableCapabilityAuthority`.

No operation function, runtime facade, bootstrap caller, endpoint wrapper, executor wrapper, accessor, extraction method, or task is selected.

## 6. Exact selected aggregate shape

The new module must express the LB-selected semantics equivalent to:

```rust
pub(crate) struct LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<
    P,
    D,
    T,
    F,
    C,
    R,
    E,
> {
    requester_rendezvous_inputs:
        LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<
            P,
            D,
            T,
            F,
            C,
            R,
            E,
        >,
    capability_authority: Arc<ProductionDurableCapabilityAuthority>,
}
```

Exact formatting and line wrapping may be determined by rustfmt.

No third field is selected.

## 7. Constructor law

The aggregate constructor must consume by value:

- one existing production/reachability/requester-rendezvous aggregate; and
- one raw `ProductionDurableCapabilityAuthority`.

The constructor performs exactly one outer ownership adaptation:

`Arc::new(capability_authority)`

and stores the result.

The constructor performs no credential read, provider/bootstrap call, registry operation, mutex acquisition, policy evaluation, authorization, request decoding, response I/O, endpoint bind, listener activation, readiness publication, task spawn, retry, fallback, or runtime activation.

## 8. Outer Arc non-cloning law

The new module must contain no:

`Arc::clone`

for the durable authority.

It must expose no accessor that clones the retained Arc.

It must expose no `into_parts`, extraction, propagation, operation-factory, or caller-migration method.

The first currently materialized outer-Arc clone remains the LA successful vacant-slot worker insertion path.

## 9. Non-cloneable higher owner

The new aggregate must not derive or implement `Clone` or `Copy`.

The nested existing requester-rendezvous production aggregate remains owned by value and textually unchanged in `linux_bootstrap.rs`.

The raw durable authority is consumed once into the outer Arc. Worker recreation must not reconstruct the raw authority or create another higher-owner Arc.

## 10. Existing source preservation

The following exact LB source remains byte-identical in the immediate source successor:

- `crates/prw-agent/src/linux_bootstrap.rs` blob `f2a87c45bd8d96bf1555b65210531c94c722eb2f`;
- `crates/prw-agent/src/production_durable_registry_runtime_custody.rs`;
- `crates/prw-agent/src/production_durable_registry_custody_bootstrap.rs`;
- all `remote_session_capability_runtime` source including LA;
- `main.rs`.

In particular, the existing:

`LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs<...>`

constructor, fields, operation factory, callers, tests, and current runtime behavior remain unchanged.

## 11. Shared-current authority separation law

`SharedCurrentCapabilityAuthority<P>` remains distinct from `ProductionDurableCapabilityAuthority`.

LC does not alter `LinuxAgentRemoteProcessOperationInputs::capability_authority`.

Production durable authority does not enter AJ admission or requester-DR authority.

LC preserves:

`ProductionDurableCapabilityAuthority != SharedCurrentCapabilityAuthority<P>`

## 12. Active call graph remains unchanged

The current active path remains unchanged:

`linux_bootstrap -> RemoteSessionEndpointLifecycleRuntime -> legacy repeated real-admission lifecycle`

The LA production-durable requester-aware repeated-admission overload remains dormant and uninvoked.

The new LC-selected aggregate has no caller.

No existing function receives it, constructs it, extracts from it, or invokes runtime behavior from it.

## 13. Population boundary remains closed

The existing:

`bootstrap_production_durable_capability_authority_from_systemd_credentials()`

remains the selected raw authority population helper, but LC does not call it.

Where a real executable/bootstrap caller awaits that helper remains a later independent gate.

No systemd credential name/path, provider endpoint, TLS material, raw client, registry record, fallback authority, or synthetic authority enters the new module.

## 14. Executor/endpoint boundary remains closed

LC does not change LA visibility.

It does not export requester-aware completion envelopes or disposal helpers.

It does not add endpoint lifecycle or executor lifecycle wrappers.

It does not select how the retained outer Arc crosses from process-level custody to LA.

That propagation/interface question remains a later independent gate.

## 15. Transport-integrity rationale

The LC reselection is source-layout-only.

The connected GitHub write surface can safely create a new small file and can replace a smaller crate-root module registry, but it does not expose a bounded patch insertion primitive for the approximately 101 KiB `linux_bootstrap.rs` blob.

LC therefore prefers a normal Rust sibling module over reconstructing a large existing source blob solely for a dormant ownership carrier.

This changes no semantic selection and keeps the large Linux bootstrap source byte-identical.

If the materialization transport cannot reproduce the frozen `lib.rs` candidate exactly, STOP before attaching a source commit and open a corrective gate rather than accepting drift.

## 16. Exact future source ceiling

The immediate source successor may change only these two paths:

1. `crates/prw-agent/src/lib.rs` — module registration only;
2. `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs` — new dormant aggregate and constructor only.

No third source path is selected.

No manifest, lockfile, workflow, Android, executable, service-manager, deployment, registry, or runtime configuration path is selected.

If compilation requires a third path, existing runtime/caller mutation, visibility widening, bootstrap invocation, or outer-Arc propagation method, STOP and open a new gate.

## 17. Immediate source successor proof obligations

A later source materialization must prove:

1. exact predecessor is LC head;
2. exactly two selected source paths changed;
3. `linux_bootstrap.rs` remains exact LB blob `f2a87c45bd8d96bf1555b65210531c94c722eb2f`;
4. `lib.rs` changes only by the Linux-gated crate-private module registration and dormant allowance;
5. new module contains only required imports, the selected aggregate, and its constructor;
6. aggregate owns the existing requester-rendezvous production aggregate by value;
7. aggregate owns exactly one `Arc<ProductionDurableCapabilityAuthority>`;
8. constructor receives one raw authority by value;
9. exactly one `Arc::new(capability_authority)` is present;
10. no durable-authority `Arc::clone` is present;
11. no `Clone`/`Copy` is added to the aggregate;
12. no accessor/extraction/propagation/operation method is present;
13. durable bootstrap helper is not invoked;
14. no existing aggregate/callsite is changed;
15. no endpoint/executor source changes;
16. no listener/readiness/network/runtime behavior changes;
17. no manifest/lock/workflow/Android/systemd/deployment change occurs;
18. Rust formatting, Clippy, tests, and workspace build succeed;
19. Android validation, if triggered, is regression evidence only and authorizes no Android mutation.

## 18. Later gates left open

LC deliberately does not select:

- executable/bootstrap population of the new higher-owner aggregate;
- outer-Arc extraction or propagation;
- production operation boundary changes;
- endpoint lifecycle durable-authority custody;
- executor lifecycle caller migration to LA;
- visibility widening;
- runtime activation;
- readiness/listener changes;
- deployment/restart/recovery/service-manager behavior.

## 19. Exact STOP boundary

C03e-LC is selection-only.

Do not mutate either selected Rust path in LC.

Do not mutate `linux_bootstrap.rs`.

Do not call the durable capability-authority bootstrap helper from executable/runtime source.

Do not add an outer-Arc clone, accessor, extraction seam, propagation method, endpoint wrapper, executor wrapper, or caller migration.

Do not modify LA, LB predecessor source, `main.rs`, manifests, workflows, Android, systemd, deployment, or runtime configuration.

Do not mark any PR ready, merge, deploy, restart, activate networking, or rewrite history.
