# Phase 152 C03e-JC — Production Durable Registry Runtime Custody Boundary Selection — STAGING

Status: `SELECTION_STAGED_VALIDATION_EVIDENCE_PENDING`

Candidate gate:
`C03E_JC_PRODUCTION_DURABLE_REGISTRY_RUNTIME_CUSTODY_BOUNDARY_SELECTED`

## 1. Purpose

C03e-JC is the documentation-only successor to closed C03e-JB.

It selects the narrow Agent-owned runtime-custody boundary for the already-materialized production durable-registry semantic store before any startup/readiness/use-site activation.

C03e-JC does not materialize Rust/Kotlin/Cargo/lockfile/workflow/runtime source, call the JB bootstrap facade, read credentials, connect to etcd, perform registry semantic I/O, create registry records, wire startup/readiness, activate networking, deploy, merge, close a PR, mark a PR ready, or delete a branch.

## 2. Exact predecessor

Exact predecessor C03e-JB head:

`57ca97de00a41543acc566f03147a94748e1765d`

C03e-JB materialized an Agent-owned zero-secret-argument async facade that composes:

1. fixed systemd durable-registry credential custody;
2. bounded control-plane provider bootstrap;
3. `DurableRegistryEtcdStore::new(executor)`.

The JB facade returns only `DurableRegistryEtcdStore` and performs no semantic registry operation or startup/readiness wiring.

## 3. Exact semantic store evidence

Exact source at the predecessor:

`crates/prw-registry/src/durable_registry_etcd_store.rs`

The store owns semantic durable-registry operations over the already-bounded raw provider executor.

Its construction is side-effect-free with respect to provider and registry I/O.

The store is the semantic authority handle for later current membership/device/transport/session validation and mutations, but C03e-JC does not invoke any of those methods.

## 4. Exact Agent composition evidence

Exact source:

`crates/prw-agent/src/production_durable_registry_custody_bootstrap.rs`

Exact predecessor blob:

`e27e4d0a34ded1002efcb14a5c9844560a2c8bf1`

The module states that successful composition returns one dormant semantic store and performs no registry semantic operation, retry/fallback/runtime task, startup/readiness callsite, deployment, or production registry mutation.

Therefore the next ownership step may retain the store in Agent runtime custody without activating it.

## 5. Existing runtime-custody precedent

Exact Agent precedent:

`crates/prw-agent/src/production_reachability_runtime_custody.rs`

Exact predecessor blob:

`ffcddc0253de2b5430be798061ddad8e920a07ac`

The reachability precedent establishes a repository architecture pattern in which:

- an already-produced production composition is consumed exactly once;
- semantic authority values become Agent-owned runtime custody;
- ownership adaptation itself performs no credential read, provider bootstrap, background task, readiness publication, listener activation, candidate publication, traversal, dialing, executable startup wiring, or deployment;
- later separately gated methods may consume runtime custody for specific operational transitions;
- source existence does not itself activate runtime behavior.

C03e-JC selects the same ownership phase for durable registry without copying reachability domain semantics.

## 6. Selected runtime-custody owner

C03e-JC selects one crate-internal Agent type equivalent to:

`ProductionDurableRegistryRuntimeCustody`

The owner contains exactly one:

`DurableRegistryEtcdStore`

The store remains private, non-global and non-clone authority.

The first source checkpoint must not expose a generic public getter, borrowed store reference, mutable store reference, raw executor, raw provider client, or provider-security material.

## 7. Selected constructor law

The selected ownership adaptation is equivalent to:

`ProductionDurableRegistryRuntimeCustody::from_store(store: DurableRegistryEtcdStore) -> Self`

or an equivalently narrow by-value constructor.

The constructor:

- consumes exactly one existing durable registry store;
- performs no credential read;
- performs no network I/O;
- performs no etcd operation;
- performs no membership/device/session/transport operation;
- creates no task/thread;
- registers no global/static state;
- publishes no readiness;
- activates no listener/requester/rendezvous/candidate/traversal/dialing path.

## 8. No second bootstrap law

Runtime custody must not call:

`bootstrap_production_durable_registry_from_systemd_credentials()`

inside its ownership constructor.

C03e-JC keeps composition and ownership adaptation separate:

1. JB facade may later produce the store when explicitly invoked;
2. runtime custody consumes that already-produced store exactly once;
3. later use-site checkpoints may consume or borrow custody only through separately selected methods.

No duplicate credential load, provider connection, executor construction, or semantic store construction is selected.

## 9. No extraction seam

The first runtime-custody source checkpoint does not select:

- `into_inner()` returning the store;
- `store()` borrowed getter;
- `store_mut()` getter;
- raw executor extraction;
- provider client extraction;
- clone/arc/global registry storage.

A later exact operation-specific checkpoint must select any access method required for a concrete runtime use case.

This prevents runtime custody from becoming a generic escape hatch around semantic authority gating.

## 10. No semantic read at custody construction

The runtime-custody constructor must not prove or infer:

- registry availability;
- membership existence;
- device existence;
- device enrollment;
- current transport identity;
- authenticated-session validity;
- provider readiness;
- registry record population.

A constructed store may point to an empty, unavailable, malformed, or not-yet-provisioned provider environment; C03e-JC makes no production state claim.

## 11. Readiness law

C03e-JC does not select durable-registry readiness semantics.

In particular, the following are not readiness proofs:

- custody object construction;
- successful credential loading;
- successful provider connection;
- successful `DurableRegistryEtcdStore::new`;
- existence of an etcd namespace prefix;
- empty keyspace;
- cached/session state;
- first successful unrelated provider call.

A later readiness/use-site checkpoint must define any exact authoritative probe and failure policy if readiness needs durable registry state.

## 12. Current peer-provenance law remains deferred

C03e-JC does not yet wire durable registry current-device/current-transport lookup into reachability bootstrap, endpoint startup, candidate publication, traversal, or dialing.

The semantic store already contains current transport lookup/validation capability, but the exact runtime caller, logical DeviceId source, error propagation, startup ordering, and readiness relationship remain separately gated.

No environment value, endpoint, request/session ID, IP address, or fixture may substitute for the later exact logical DeviceId authority.

## 13. Authenticated-session law remains deferred

C03e-JC does not wire paired transactional authenticated-session registry validation into a runtime request path.

The later runtime-use checkpoint must preserve the semantic store's authoritative membership/device pair-read law and fail-closed error handling.

No old session snapshot or in-memory registry may become implicit fallback merely because runtime custody exists.

## 14. Lifecycle/mutation law remains deferred

Runtime custody construction performs none of:

- membership creation;
- membership suspension/removal;
- device registration;
- transport bind/rotation;
- device revocation.

No background reconciliation or migration loop is selected.

## 15. Ownership and dependency topology

`prw-agent` owns runtime custody.

`prw-registry` retains semantic durable-registry behavior.

`prw-control-plane` retains raw provider execution/bootstrap.

`prw-reachability-custody` retains systemd credential custody.

No dependency direction changes are selected.

No new crate, Cargo.toml change or Cargo.lock change is required for the first source checkpoint.

## 16. Failure law

The side-effect-free custody constructor is selected as infallible because it only re-owns one already-created `DurableRegistryEtcdStore` by value.

No provider or semantic error exists at this ownership-adaptation boundary.

Any future operation method must carry its own exact semantic/provider-neutral failure contract.

## 17. Shutdown/drop law

C03e-JC selects no explicit shutdown action for the registry store.

Dropping runtime custody may drop the contained semantic store/executor according to existing Rust ownership semantics, but C03e-JC does not claim provider-side cleanup, lease release, record mutation, or semantic shutdown transaction.

No background task exists to supervise or join at this checkpoint.

## 18. Global-state prohibition

The first runtime-custody source must not place the store in:

- a global/static;
- process-wide singleton;
- lazy global;
- mutable global;
- ad hoc `Arc<Mutex<_>>` service locator;
- environment-derived registry.

Any future sharing topology requires an exact separately gated ownership contract.

## 19. Production activation prohibition

C03e-JC does not authorize changes to:

- Agent `run()`;
- executable `main.rs`;
- Linux startup/bootstrap callsites;
- service readiness;
- listener lifecycle;
- requester/rendezvous runtime;
- candidate publication;
- traversal/dialing;
- systemd units;
- credential provisioning;
- provider RBAC/auth;
- deployment/restart.

## 20. First source-materialization ceiling

After C03e-JC closure, the next separately gated source checkpoint may change only:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

plus a minimal crate-internal module declaration in:

`crates/prw-agent/src/lib.rs`

Allowed behavior only:

- private storage of one `DurableRegistryEtcdStore`;
- by-value side-effect-free constructor;
- provider-free type/signature/ownership tests;
- comments/docs establishing later gating.

Still prohibited:

- calling JB bootstrap inside custody construction;
- semantic registry operations;
- provider operations;
- store extraction/getters;
- global/shared service registration;
- runtime/startup/readiness wiring;
- production record creation/migration;
- deployment.

## 21. Focused next-source test matrix

The next source checkpoint should prove without production I/O:

- constructor accepts exactly one `DurableRegistryEtcdStore` by value;
- constructor returns exactly `ProductionDurableRegistryRuntimeCustody`;
- no async/future requirement for ownership adaptation;
- no provider-security argument exists;
- no public generic store/executor extraction API is introduced;
- module remains crate-internal and dead-code-allowed until a later use-site checkpoint.

## 22. Explicit non-authorization

C03e-JC does not authorize or perform:

- Rust source materialization;
- Cargo/lockfile changes;
- credential reads;
- provider connection;
- registry semantic reads/writes;
- current peer lookup;
- session validation wiring;
- production registry population/migration;
- provider/security provisioning;
- systemd unit mutation;
- startup/readiness/runtime activation;
- deployment/restart;
- merge/PR close/ready-for-review;
- branch deletion/history rewrite;
- repository visibility/configuration mutation.

## 23. Closure meaning

C03e-JC closure means only:

`PRODUCTION_DURABLE_REGISTRY_RUNTIME_CUSTODY_BOUNDARY_SELECTED`

It selects one side-effect-free Agent-owned runtime-custody wrapper around the already-composed durable registry semantic store.

It does not mean the store is constructed by production startup, provider state is ready, registry records exist, current peer identity is resolved, sessions are validated through the store, or any runtime path is active.
