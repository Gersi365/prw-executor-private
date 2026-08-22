# Phase 152 C02f-BT — Full Async-Authority Lifecycle Composition Architecture Selection Staging

Status: `SELECTED / DOCUMENTATION_ONLY / SAME_PROVIDER_CONTEXT / BRIDGE_OWNED_ASYNC_LIFECYCLE_COMPOSITION / NARROW_CONTROL_PLANE_LIFECYCLE_CAPABILITY / NO_RAW_STORE_ESCAPE / NO_PROVIDER_BOOTSTRAP / NO_RUNTIME_ACTIVATION / NO_R1_R4_ACTIVATION / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-22
Repository: `Gersi365/prw-executor-private`
Repository ID: `1334911207`

## Approval basis

The user explicitly authorized:

`Autorizoj C02f-BT full async-authority lifecycle composition architecture selection.`

This checkpoint selects architecture only. It does not materialize Rust source, construct an etcd client, select an endpoint, configure TLS/auth/RBAC, create an executor/runtime/task, activate R1-R4 effects, deploy, or merge any pull request.

## Exact prerequisite

The exact validated prerequisite is closed C02f-BS:

- branch `phase-152-c02f-bs-common-acquisition-composition-source-materialization-staging`;
- head `9d4657e88a255cc333d606b899ce7edea5015ba6`;
- tree `92930244d39fe8b077fbcbf4630249bc116bffde`;
- gate `C02F_BS_COMMON_ACQUISITION_COMPOSITION_MATERIALIZED`.

C02f-BS remains draft/open/unmerged through PR #90 and is not merged or modified by this selection.

## Existing validated lifecycle pieces that BT must compose, not redesign

The repository already contains one validated callable composition for each public async live-owner lifecycle operation:

1. **Acquisition** — C02f-BS `acquire_prepared_live_owner(...)`:
   - calls the C02f-BM preparation facade exactly once;
   - dispatches only on exact BM `Replacement | FirstOwner | Superseded` evidence;
   - replacement reuses C02f-BQ / C02f-AE / C02f-AV semantics;
   - first-owner reuses C02f-BO/BP execution and mapper semantics;
   - `Superseded` maps directly to semantic `Contended`;
   - preparation/provider ambiguity fails closed.

2. **Currentness** — C02f-BF `execute_live_owner_currentness(...)`:
   - derives the provider fence only from the supplied semantic grant;
   - performs the exact existing default-linearizable provider currentness operation;
   - maps exact provider `Current | Stale` one-to-one;
   - performs no retry, cache fallback, Watch inference, reconciliation, or mutation;
   - provider ambiguity fails closed.

3. **Release** — C02f-BD `execute_reconciled_live_owner_release(...)`:
   - derives peer/fence only from the supplied semantic grant;
   - performs the exact initial default-linearizable observation;
   - passes that exact observation into the existing C02f-AE bounded release reconciliation;
   - passes exact terminal evidence plus the same semantic grant into the validated C02f-BB mapper;
   - performs no outer retry or semantic result manufacture;
   - provider ambiguity fails closed.

BT does not reopen or replace any of those state machines.

## Existing async port that BT must satisfy

C02f-X/Y already selected and materialized the public production async authority port:

```rust
pub trait ReachabilityLiveOwnerAsyncAuthority {
    fn acquire<'a>(
        &'a mut self,
        peer: &'a PeerConnectivityIdentity,
    ) -> impl Future<
        Output = Result<ReachabilityLiveOwnerAcquisition, ReachabilityLiveOwnerAuthorityError>,
    > + Send + 'a;

    fn currentness<'a>(
        &'a mut self,
        grant: &'a ReachabilityLiveOwnerGrant,
    ) -> impl Future<
        Output = Result<ReachabilityLiveOwnerCurrentness, ReachabilityLiveOwnerAuthorityError>,
    > + Send + 'a;

    fn release<'a>(
        &'a mut self,
        grant: &'a ReachabilityLiveOwnerGrant,
    ) -> impl Future<
        Output = Result<ReachabilityLiveOwnerRelease, ReachabilityLiveOwnerAuthorityError>,
    > + Send + 'a;
}
```

The selected representation remains native `impl Future + Send`, static dispatch and `&mut self` borrowing. BT does not authorize `async-trait`, mandatory boxed/dynamic futures, trait-object dispatch, `Arc<Mutex<_>>`, hidden worker tasks, nested runtimes or `block_on`.

## Selected lifecycle ownership model

BT selects one bridge-owned concrete async-authority composition whose state is exactly one already-constructed C02f-BM preparation facade.

Conceptually:

```rust
pub struct ReachabilityLiveOwnerComposedAsyncAuthority {
    preparation: ReachabilityLiveOwnerAcquisitionPreparation,
}
```

Exact source naming may be mechanically adjusted during a separately authorized materialization tranche, but the ownership semantics may not be widened.

### Selected constructor boundary

The bridge-side composed authority constructor accepts the already-created preparation facade by value:

```rust
fn new(preparation: ReachabilityLiveOwnerAcquisitionPreparation) -> Self;
```

BT specifically does **not** select a bridge constructor that accepts:

- endpoint strings;
- `etcd_client::Client`;
- `etcd_client::KvClient`;
- TLS configuration;
- credentials;
- RBAC identities;
- CA/certificate material;
- retry/backoff policy;
- recovery provider handles;
- runtime/executor handles.

This preserves provider construction/bootstrap as a separate later gate and avoids adding an `etcd-client` dependency to `prw-remote-bridge` merely to assemble lifecycle orchestration.

## Same-provider-context invariant

The concrete authority must use the exact same live-owner provider context already owned by the supplied C02f-BM preparation facade for **all three** lifecycle operations.

It must not create or accept a second live-owner store/client merely for currentness or release.

The following invariant is selected:

```text
one ReachabilityLiveOwnerAcquisitionPreparation
        |
        +-- existing BM allocation provider context
        |
        +-- existing BM live-owner provider context
                |
                +-- BS acquisition execution
                +-- BF currentness execution
                +-- BD release execution
```

No operation may silently switch authority backend, endpoint set, credential context, cluster or provider instance.

## Selected control-plane lifecycle capability

C02f-BS already materialized a narrow acquisition-only capability:

`ReachabilityLiveOwnerAcquisitionExecution<'_>`

That capability intentionally exposes no currentness or release operation. BT preserves that boundary unchanged.

For full lifecycle composition, BT selects a **separate, non-escaping, lifetime-bounded lifecycle execution capability** borrowed from the same preparation facade.

Conceptually:

```rust
pub struct ReachabilityLiveOwnerLifecycleExecution<'a> {
    live_owner: &'a mut ReachabilityLiveOwnerEtcdStore,
}
```

The field remains private. Construction remains private to `ReachabilityLiveOwnerAcquisitionPreparation`.

The preparation facade may expose only a narrow borrow constructor conceptually equivalent to:

```rust
fn lifecycle_execution(&mut self) -> ReachabilityLiveOwnerLifecycleExecution<'_>;
```

This performs no provider I/O.

### Non-escape requirements

The lifecycle capability must not expose or return:

- `&mut ReachabilityLiveOwnerEtcdStore`;
- `&ReachabilityLiveOwnerEtcdStore`;
- `KvClient`;
- `Client`;
- endpoint/configuration state;
- generic Txn/Get/Delete operations;
- `Deref` or `DerefMut` to the store;
- a clonable provider handle;
- a second store/client constructor;
- a handle that can outlive the mutable borrow of the preparation facade.

The capability exists only to let bridge-owned BF/BD orchestration invoke the already-selected provider operations on the exact preparation-owned live-owner store.

## Selected lifecycle capability operation surface

The future source tranche may expose on the lifecycle capability only the provider-level operations strictly required to preserve existing BF/BD bridge orchestration.

### Currentness support

The capability may provide the exact existing provider currentness primitive, using only:

- exact borrowed `PeerConnectivityIdentity`;
- exact non-zero provider fence derived by bridge BF from the semantic grant.

It must return the existing provider currentness classification/error shape without manufacturing bridge semantics.

### Release support

The capability may provide only the exact provider primitives already consumed by BD:

1. exact default-linearizable live-owner observation for the supplied peer;
2. exact existing bounded release reconciliation using the same peer/fence and retained observation.

The bridge remains the owner of the BD orchestration sequence and semantic mapper invocation.

The capability must not absorb BD into a new control-plane semantic release state machine.

### Explicitly not selected on the lifecycle capability

BT does not select:

- acquisition planning;
- fence-sequence allocation;
- attempt-ID generation;
- replacement or first-owner handoff construction;
- generic mutation submission;
- arbitrary re-observation;
- Watch/lease/TTL helpers;
- retry scheduling;
- endpoint/client lifecycle;
- semantic grant/currentness/release construction.

Acquisition remains on the existing BS acquisition capability/composition.

## Selected bridge adapters for BF and BD

A future source tranche may mechanically factor BF and BD so each has a crate-private or otherwise narrow adapter that accepts the new scoped lifecycle capability while preserving the existing public raw-store entry point for compatibility/testing if required.

Conceptually:

```rust
execute_live_owner_currentness_with_prepared_execution(
    execution: &mut ReachabilityLiveOwnerLifecycleExecution<'_>,
    grant: &ReachabilityLiveOwnerGrant,
)
```

and:

```rust
execute_reconciled_live_owner_release_with_prepared_execution(
    execution: &mut ReachabilityLiveOwnerLifecycleExecution<'_>,
    grant: &ReachabilityLiveOwnerGrant,
)
```

The factored adapters must preserve BF/BD semantics exactly. They may not introduce additional reads, retries, evidence reconstruction or mapping changes.

If retaining the public raw-store BF/BD functions requires small internal factoring, their externally observable behavior and existing tests must remain semantically equivalent.

## Selected concrete async-authority dispatch

The future bridge-owned composed authority implements the existing `ReachabilityLiveOwnerAsyncAuthority` exactly as follows.

### acquire

```text
self.preparation
    -> existing C02f-BS acquire_prepared_live_owner(...)
    -> exact existing BS result/error
```

Requirements:

- exactly one BS acquisition composition call per public `acquire` invocation;
- no second preparation;
- no outer retry;
- no second fence allocation;
- no authority-attempt regeneration outside BM;
- no provider fallback;
- no conversion of `Contended` into retry/grant.

### currentness

```text
self.preparation.lifecycle_execution()
    -> exact BF currentness composition
    -> exact BF semantic result/error
```

Requirements:

- exactly one lifecycle capability borrow;
- exact peer/fence derived from the semantic grant;
- exactly the existing BF authoritative provider read/classification path;
- no cache or Watch proof;
- no retry or recheck;
- `Current` remains only a point-in-time proof.

### release

```text
self.preparation.lifecycle_execution()
    -> exact BD reconciled release composition
    -> exact BD semantic result/error
```

Requirements:

- exactly one lifecycle capability borrow;
- exact peer/fence derived from the same semantic grant;
- exact BD initial observation + bounded reconciliation + BB mapping;
- no outer retry;
- no semantic success manufacture;
- stale/non-current release cannot clear a newer owner.

## Borrowing and sequencing semantics

The public async authority retains the C02f-X/Y `&mut self` receiver model.

Therefore one mutable borrow of the authority serializes operations through that authority value unless a later separately selected concurrency tranche proves a need for another model.

BT does not select:

- per-peer lock maps;
- sharded authorities;
- `Arc<Mutex<_>>`;
- channels;
- detached tasks;
- background workers;
- concurrent mutation scheduling.

This is not a claim that production must remain globally serialized forever. It is the narrow first concrete lifecycle composition selected by the existing API contract.

## Cancellation semantics

Dropping a pending future never means acquisition/currentness/release succeeded.

Provider operations that may have reached the authority backend remain governed by their already-materialized reconciliation semantics where applicable.

BT adds no cancellation retry, compensation transaction or background completion task.

## Failure mapping

BT preserves existing operation-specific failure semantics without adding a new top-level error taxonomy.

- BS preparation/provider ambiguity -> `UnavailableOrAmbiguous`.
- BF provider/classification ambiguity -> `UnavailableOrAmbiguous`.
- BD provider/reconciliation ambiguity -> `UnavailableOrAmbiguous`.
- Existing mapper `FenceExhausted` conversions remain unchanged where already selected.
- No unavailable/ambiguous state may become `Granted`, `Current` or `Released`.
- No retry exhaustion error is introduced because BT selects no outer retry.

## Relationship to C02f-AC provider bridge

The existing C02f-AC `ReachabilityLiveOwnerProviderBridge<P>` and `ReachabilityLiveOwnerDefinitiveProviderPort` remain valid earlier/reference machinery and are not deleted by BT.

However, BT does **not** select that lower definitive-provider port as the production owner for the now-materialized full acquisition chain.

A future production composed authority must not bypass C02f-BM/BS by creating a second acquisition implementation through C02f-AC.

BT therefore selects direct composition of the validated BS/BF/BD paths rather than a parallel second provider port/state machine.

## Layering

The selected layering remains:

```text
executor/bootstrap/provider construction (still deferred)
        |
        v
ReachabilityLiveOwnerAcquisitionPreparation
        |
        v
prw-remote-bridge composed async authority/orchestration
        |
        v
narrow preparation-owned acquisition/lifecycle capabilities
        |
        v
prw-control-plane etcd provider implementation
        |
        v
etcd-client
```

The conceptual diagram describes ownership and call direction; it does not authorize process bootstrap.

No inverse `prw-control-plane -> prw-remote-bridge` dependency is selected.

## Provider bootstrap remains a separate gate

BT intentionally stops before selecting how the initial `ReachabilityLiveOwnerAcquisitionPreparation` is created in a production process.

Still deferred:

- endpoint discovery/selection;
- etcd `Client::connect`;
- obtaining/splitting a `KvClient`;
- TLS feature/profile;
- CA/certificate material;
- authentication credentials;
- RBAC identities/permissions;
- cluster topology;
- startup ordering;
- reconnect policy;
- health/readiness policy;
- process executor ownership;
- lifecycle shutdown.

A future authority materialization may accept an already-created preparation facade but must not infer any of the above.

## Recovery boundary

Normal live-owner acquisition still assumes the surrounding authority lifecycle has established a valid initialized PRWF head for the current recovery epoch.

The composed async authority must not:

- issue a new recovery epoch;
- initialize missing PRWF state as an acquisition fallback;
- contact Spanner or another recovery ledger;
- silently advance epoch state;
- reinterpret missing initialized sequence state as contention or success.

Those remain separately controlled recovery/bootstrap concerns.

## R1-R4 boundary

A successful bridge-level acquisition/currentness result does not itself authorize unfenced effect execution.

R1-R4 effect sinks must still reject stale fences at or atomically with the actual side effect boundary.

BT does not materialize, activate or weaken R1-R4.

## Cargo/dependency expectation for later materialization

At the exact BS base, `prw-remote-bridge` already has `prw-control-plane` as a normal dependency.

The selected lifecycle composition therefore has no currently proven need for:

- a new direct `etcd-client` dependency in `prw-remote-bridge`;
- `async-trait`;
- Tokio;
- additional synchronization crates;
- Cargo.lock changes.

If a later compiler proves a manifest change unavoidable, materialization must stop and re-audit before widening the dependency surface.

## Compiler/API stop condition for later source materialization

The exact Rust spelling of the narrow lifecycle capability and adapters may be mechanically adjusted to satisfy the repository toolchain.

If the selected same-provider, non-escaping lifecycle ownership cannot be expressed without any of the following, source materialization must stop for a new compiler/API selection checkpoint:

- raw store/client exposure;
- mandatory boxed/dynamic futures;
- `async-trait`;
- `Arc<Mutex<_>>` or equivalent hidden synchronization;
- a second provider store/client;
- moving BF/BD semantic orchestration downward into control-plane;
- weakening the `&mut self` authority contract.

No such fallback is pre-authorized by BT.

## Minimum scope for the separately authorized source tranche

A later source tranche should prefer only:

1. add one narrow `ReachabilityLiveOwnerLifecycleExecution<'_>` capability to the existing BM preparation facade;
2. expose only the exact provider operations required by BF/BD;
3. mechanically adapt BF currentness to that scoped capability while preserving existing behavior;
4. mechanically adapt BD release to that scoped capability while preserving existing behavior;
5. add one bridge-owned composed authority holding exactly one `ReachabilityLiveOwnerAcquisitionPreparation`;
6. implement `ReachabilityLiveOwnerAsyncAuthority` by exact delegation to BS/BF/BD;
7. add deterministic focused tests for dispatch/fail-closed behavior and static `Future + Send` shape;
8. narrow root exports only as needed.

No provider/bootstrap/runtime/deployment work belongs in that tranche.

## Explicit exclusions

C02f-BT does not authorize or perform:

- Rust source changes;
- Cargo/Cargo.lock changes;
- workflow changes;
- Android or Agent changes;
- raw store/client accessors;
- a second store/client/provider context;
- endpoint selection or `Client::connect`;
- TLS/auth/RBAC/credential selection;
- reconnect/backoff policy;
- recovery epoch issuance or PRWF initialization;
- new fence/attempt allocation policy;
- new acquisition/currentness/release state machines;
- outer retries;
- Watch/lease/TTL authority proofs;
- runtime/executor/task ownership;
- full process bootstrap;
- R1-R4 effect activation;
- transport/NAT/file/terminal effect changes;
- deployment;
- merge.

## Selected gate

After exact documentation-only materialization, canonical validation and exact BS -> BT scope verification, the gate is:

`C02F_BT_FULL_ASYNC_AUTHORITY_LIFECYCLE_COMPOSITION_SELECTED`

That gate means only that the full async-authority lifecycle composition architecture is selected. It does not mean source materialization, provider bootstrap, runtime activation, deployment or merge is authorized.
