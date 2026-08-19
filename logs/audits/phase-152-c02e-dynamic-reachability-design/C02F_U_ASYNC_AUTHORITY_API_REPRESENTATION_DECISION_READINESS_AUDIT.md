# Phase 152 C02f-U — Async Authority API Representation Decision Readiness Audit

Status: `ASYNC_AUTHORITY_API_REPRESENTATION_READINESS_COMPLETE / RUST_1_97_1_AND_EDITION_2024_BASELINE_CONFIRMED / NATIVE_ASYNC_TRAIT_SUPPORT_AVAILABLE / PUBLIC_ASYNC_FN_SEND_BOUND_LIMITATION_CONFIRMED / ASYNC_FN_AND_RPITIT_NOT_DYN_COMPATIBLE / NO_CURRENT_DYN_DISPATCH_REQUIREMENT_PROVEN / EXPLICIT_IMPL_FUTURE_PLUS_SEND_STATIC_DISPATCH_PREFERRED_FOR_SELECTION_REVIEW / BOXED_FUTURE_RESERVED_FOR_PROVEN_DYN_REQUIREMENT / ASYNC_TRAIT_MACRO_NOT_REQUIRED_BY_CURRENT_EVIDENCE / RECEIVER_CONCURRENCY_MODEL_DEFERRED / NO_API_SELECTION / NO_SOURCE_MUTATION / NO_RUNTIME_ACTIVATION / DOCS_ONLY / PRODUCTION_SOURCE_BYTE_STABLE_REQUIRED`

Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Active branch: `phase-152-c02e-dynamic-reachability-design`
Frozen C02d head: `857583b25ed1206317641a93fd8f927819c954d8`
C02f-T predecessor head: `80d7c490fd65727fe2a7de5d620435ef219eda39`
C02f-T predecessor tree: `ab845be66a85bedd4f5d03157d94617e3550eabc`
Review date: `2026-08-19`

## Purpose

C02f-T established that production etcd I/O must not be hidden behind the existing synchronous `ReachabilityLiveOwnerAuthority` methods and preferred, for later selection review, a separate explicit asynchronous production authority port.

C02f-U narrows only the Rust representation of that future asynchronous port. It asks whether PRW needs:

- native `async fn` in trait;
- explicit return-position `impl Future`;
- a boxed `dyn Future`;
- an `async-trait` macro transformation;
- or a more verbose associated-future representation.

This audit does not select or add any API, trait, runtime, crate dependency, etcd operation, schema, TLS feature, endpoint or production behavior.

## Verified toolchain baseline

The workspace root currently declares Rust edition `2024`.

The latest canonical full-workspace validation associated with C02f-M recorded:

- `rustc 1.97.1 (8bab26f4f 2026-07-14)`;
- `cargo 1.97.1`;
- `rustfmt 1.9.0-stable`;
- `clippy 0.1.97`.

Therefore this design review does not need to target a pre-1.75 compiler merely for compatibility with the latest validated workspace state.

The canonical workflow currently records the hosted toolchain rather than explicitly pinning an MSRV in the root manifest or validation workflow. This audit therefore does not create a new MSRV contract.

## Rust language facts relevant to this decision

Official Rust documentation for the current toolchain confirms:

1. `async fn` in traits and return-position `impl Trait` in traits are stable language features;
2. an `async fn` in a trait is conceptually a method returning an opaque Future;
3. a publicly reachable trait using bare `async fn` does not promise that its opaque returned Future implements auto traits such as `Send`;
4. rustc's `async_fn_in_trait` lint specifically recommends an explicit `fn -> impl Future + Send` form when downstream users need the Send guarantee;
5. trait methods using `async fn` or return-position opaque `impl Trait` are not dyn-compatible dispatchable methods;
6. dyn compatibility is only necessary when PRW actually needs trait-object dispatch such as `dyn SomeAuthority`.

These facts separate two concerns that must not be conflated:

- asynchronous execution;
- dynamic dispatch.

PRW can require the first without automatically requiring the second.

## Current PRW usage evidence

The current `ReachabilityLiveOwnerAuthority` defining module uses concrete reference implementations in its unit tests.

The exact-peer namespace integration test also defines a concrete `PeerScopedReferenceAuthority` and invokes it directly.

The current reviewed live-owner source/test path therefore demonstrates static/concrete dispatch and does not demonstrate a required `dyn ReachabilityLiveOwnerAuthority` boundary.

Repository code-search indexing for this branch did not provide a complete occurrence inventory during this review, so C02f-U does not claim that trait-object use is globally impossible. The narrower conclusion is sufficient for this gate: **no current requirement for dyn dispatch has been proven and therefore dyn-support cost must not be introduced speculatively**.

## Candidate U1 — bare native `async fn` in a public production trait

Classification: `ELIGIBLE / NOT_PREFERRED_FOR_INITIAL_SELECTION`.

Conceptual shape:

```rust
pub trait ProductionAuthority {
    async fn acquire(&mut self, ... ) -> Result<...>;
}
```

Benefits:

- concise;
- native stable Rust;
- no macro dependency;
- no mandatory heap allocation;
- direct implementation syntax.

Constraints:

- the public trait does not itself promise a `Send` Future;
- downstream/runtime integration that needs to move the Future across executor threads cannot infer that promise from the trait;
- the method is not dyn-compatible;
- adding a stronger Future bound after external API adoption can become an API-design problem.

PRW has not selected a runtime owner yet, but the authority port will sit on a network/control-plane path and should not unnecessarily prevent a later multi-threaded executor design. For that reason the lack of an explicit `Send` contract is avoidable ambiguity.

## Candidate U2 — explicit `fn -> impl Future<Output = ...> + Send`

Classification: `PREFERRED_FOR_SELECTION_REVIEW / NOT_SELECTED`.

Conceptual shape only:

```rust
pub trait ProductionAuthority {
    fn acquire(&mut self, ...)
        -> impl Future<Output = Result<...>> + Send;
}
```

Equivalent methods would exist for authoritative currentness and release/reconciliation operations as selected later.

Benefits:

- native stable Rust on the validated toolchain;
- explicit asynchronous boundary;
- explicit `Send` contract for the returned Future;
- no required heap allocation merely to satisfy the trait boundary;
- no `async-trait` macro transformation;
- suitable for generic/static dispatch;
- keeps executor ownership outside the provider-neutral semantic state machines;
- implementors remain free to use ordinary async blocks/functions internally as long as the promised Future bounds are met.

Constraints:

- opaque return types are not dyn-compatible;
- exact lifetime capture and receiver contract must be reviewed when the real API is written;
- this does not itself select `&mut self` versus `&self` provider concurrency semantics;
- this does not choose a runtime or imply that every internal task must be spawned/migrated between threads.

This is the strongest current direction because it buys the future bound PRW plausibly needs without paying dynamic-dispatch/boxing costs that no current caller has justified.

## Candidate U3 — boxed dynamic Future return

Classification: `ELIGIBLE_IF_DYN_DISPATCH_IS_PROVEN / NOT_PREFERRED_NOW`.

Conceptual family:

```text
Pin<Box<dyn Future<Output = Result<...>> + Send + lifetime>>
```

Benefits:

- can be used to design a dyn-compatible authority trait when all other dyn-compatibility rules are satisfied;
- runtime-selected implementations can be stored behind trait objects without exposing their concrete Future types.

Costs:

- heap allocation/indirection at the authority call boundary unless separately optimized;
- more lifetime boilerplate;
- dynamic dispatch complexity;
- makes every operation pay for flexibility that current PRW usage has not demonstrated it needs.

If later runtime architecture explicitly requires heterogeneous provider implementations behind one trait object, U3 should be reconsidered at that concrete gate rather than preselected now.

## Candidate U4 — `async-trait` macro

Classification: `ELIGIBLE_IF_DYN_OR_COMPATIBILITY_NEED_IS_PROVEN / NOT_REQUIRED_BY_CURRENT_EVIDENCE`.

The current Cargo graph already contains `async-trait` transitively through third-party dependencies, but a transitive package is not a PRW API-selection decision and must not be treated as an authorized direct dependency.

Using the macro would generally transform async trait methods into boxed Future forms, which can be convenient for dynamic dispatch and compatibility patterns.

Current reasons not to prefer it:

- the validated compiler natively supports async trait/RPITIT syntax;
- no current dyn-dispatch requirement has been proven;
- selecting the macro would introduce a direct API/dependency choice plus boxing semantics merely to solve a problem not yet present.

If it is later selected, it must be declared intentionally rather than relying on a transitive dependency.

## Candidate U5 — named/GAT associated Future family

Classification: `ELIGIBLE / NOT_PREFERRED_FOR_INITIAL PORT`.

A trait can express operation Futures through associated types with lifetimes and explicit bounds.

Benefits:

- strong type-level control;
- no mandatory heap allocation;
- explicit Future bounds.

Costs:

- more API surface and lifetime complexity than U2 for the currently required operations;
- generic associated types themselves also obstruct simple dyn compatibility;
- no current need has been identified that U2 cannot express more directly.

This candidate remains available if future compiler/API constraints make an explicit associated Future advantageous.

## Receiver and concurrency model remains separate

C02f-U intentionally does not select whether the future async provider port takes:

- `&mut self`;
- `&self` with internally concurrency-safe provider state;
- an owned/cloned operation handle;
- or another explicit ownership form.

The current synchronous reference seam uses `&mut self`, but copying that receiver mechanically into a distributed provider could accidentally serialize operations through one mutable borrow even when etcd/client semantics permit safe concurrency.

Conversely, choosing `&self` without an explicit synchronization/cancellation model can hide mutable provider state.

This must be selected with the runtime/orchestrator ownership model, not by syntax convenience.

## Send does not mean spawn

Selecting a `+ Send` Future contract later would mean only that the returned Future is safe to move between threads while suspended.

It would **not** authorize:

- Tokio as the PRW runtime owner;
- spawning one task per authority operation;
- detaching authority work;
- parallel mutation of one namespace;
- background retries;
- network activation.

Linearization, cancellation and outcome reconciliation remain explicit authority semantics regardless of executor choice.

## Operation surface considerations

The future production port should not merely mirror three synchronous method names if doing so loses provider failure semantics.

C02f-N/T already require explicit handling of:

- acquisition/replacement;
- authoritative currentness;
- release/tombstone;
- indeterminate mutation reconciliation;
- fail-closed unavailability;
- fence exhaustion;
- recovery-required/invalid state classifications where applicable.

The exact result/error types and whether reconciliation is an explicit method or internal state machine remain separate API-selection work.

## Compile-time proof required after selection

If U2 or another representation is selected later, the first source tranche should include compile-time/runtime-independent tests proving at least:

1. the production authority port is asynchronous and cannot return ownership before its Future resolves;
2. the returned Future satisfies the selected `Send` bound when that bound is part of the contract;
3. semantic grant/fence values are not request-constructible through the provider port;
4. reference/Sans-I/O tests remain independent of an executor;
5. no hidden `block_on` or nested runtime is introduced;
6. cancellation/drop of a pending Future cannot be interpreted as a successful grant;
7. no dyn compatibility is claimed unless separately tested and selected.

Only after those API proofs should mock Txn behavior and then disposable real-etcd integration follow.

## Updated safe implementation ordering

C02f-U refines the C02f-S/T order as follows:

1. explicitly approve Group A schema/record/fence/CAS semantics;
2. explicitly approve the async production authority port placement/API representation and receiver model;
3. implement pure key/value/fence codecs and state-transition helpers;
4. implement a mockable async provider boundary and deterministic Txn outcome model;
5. implement etcd Get/Txn/reconciliation mapping without production endpoints;
6. validate contention, ambiguity, cancellation and stale release;
7. select/materialize TLS/auth/RBAC;
8. run disposable etcd integration;
9. close recovery epoch/provider selections and restore tests;
10. implement R1-R4 effect-boundary fencing;
11. only then enter Phase 153 production activation readiness.

## What C02f-U does not authorize

This audit does not authorize:

- adding or modifying a Rust trait;
- selecting U2 or any candidate;
- adding `async-trait` directly;
- adding Tokio/runtime dependencies;
- adding `prw-connectivity` to `prw-control-plane`;
- choosing `&self` or `&mut self` for the production port;
- choosing runtime/orchestrator ownership;
- schema/encoding/CAS selection;
- TLS/auth/RBAC selection;
- cluster/recovery provider selection;
- etcd endpoint contact;
- R1-R4 network effects;
- production activation.

## C02f-U conclusion

The current validated Rust toolchain can express PRW's future asynchronous authority port without a compatibility macro or mandatory boxed Future.

No current dyn-dispatch requirement has been proven. Therefore the preferred direction for explicit selection review is a statically dispatched public port whose methods return `impl Future<Output = ...> + Send`, preserving an honest async I/O boundary and an explicit Future `Send` promise without speculative allocation/dynamic dispatch.

Bare `async fn` remains technically eligible but is not preferred for the public production port because its Future `Send` contract is not expressed at the trait boundary. Boxed futures and `async-trait` remain available if a later runtime architecture proves a real dyn-dispatch requirement.

No candidate is selected by this audit. C02d remains frozen.
