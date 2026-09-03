# Phase 152 C03e-JA — Production Durable Registry Agent Custody Composition Boundary Selection — STAGING

Status: `SELECTION_ONLY_STAGED`

Selected gate:

`C03E_JA_PRODUCTION_DURABLE_REGISTRY_AGENT_CUSTODY_COMPOSITION_BOUNDARY_SELECTED`

## 1. Scope

C03e-JA is the documentation-only prerequisite after closed C03e-IZ. It selects the narrow Agent-owned production composition seam that joins the already-materialized fixed systemd credential custody loader, the already-materialized control-plane provider bootstrap, and the already-materialized semantic durable-registry store.

C03e-JA does not materialize Rust/Kotlin/Cargo/lockfile/workflow/runtime source, read production credentials, connect to etcd, create or rotate credentials, mutate systemd units, modify provider TLS/auth/RBAC, provision provider resources, create production registry records, run registry semantic operations, migrate registry state, wire Agent startup/readiness/networking, deploy, merge, close a PR, mark a PR ready, or delete a branch.

## 2. Exact predecessor authority

Exact predecessor C03e-IZ head:

`f6ec64dbea89343f4b2dcd3f64fafb4bde8af2d0`

Exact predecessor tree:

`52ba30ba98e1d955b70eb9490e415a9cd27a2c80`

C03e-IZ materialized the fixed Linux systemd credential-custody source that returns only `DurableRegistryProductionEtcdBootstrapConfig` and performs no provider network I/O.

C03e-IX materialized `bootstrap_durable_registry_production_executor(config)` in `prw-control-plane`, which performs one provider connection attempt and returns only `DurableRegistryEtcdExecutor`.

C03e-IV materialized `DurableRegistryEtcdStore::new(executor)` in `prw-registry`, which performs no provider connection and owns semantic registry behavior.

C03e-JA selects only the Agent composition between these already-existing seams.

## 3. Exact source evidence — systemd custody input

Exact source:

`crates/prw-reachability-custody/src/durable_registry_custody.rs`

Exact C03e-IZ blob:

`bb1ad2dfdbfbc16c5ecb720fe71951bfbd66bae3`

Exact production facade:

`load_durable_registry_production_etcd_bootstrap_config_from_systemd_credentials()`

The facade:

- accepts no endpoint/trust/certificate/private-key arguments;
- reads exactly the six fixed registry service credentials;
- validates systemd/Linux filesystem custody;
- preserves zeroizing private-key ownership;
- returns only `DurableRegistryProductionEtcdBootstrapConfig`;
- performs no provider network I/O.

C03e-JA therefore does not select a second credential-reading path.

## 4. Exact source evidence — provider bootstrap

Exact source:

`crates/prw-control-plane/src/durable_registry_etcd_bootstrap.rs`

Exact blob:

`787c82fe19a8a11428193921c696d3cfc551fb60`

Exact provider bootstrap:

`bootstrap_durable_registry_production_executor(config)`

It:

- consumes one validated registry bootstrap config;
- performs exactly one provider `Client::connect` attempt;
- narrows the broad `Client` to one role-scoped `KvClient`;
- drops the broad client;
- returns only `DurableRegistryEtcdExecutor`;
- exposes only bounded `RegistryConnect` provider-bootstrap failure;
- performs no registry semantic operation.

C03e-JA does not select a second provider connection or direct raw `Client`/`KvClient` path.

## 5. Exact source evidence — semantic durable registry

Exact source:

`crates/prw-registry/src/durable_registry_etcd_store.rs`

Exact blob:

`1e04b366471fe2d4433de3c383efb4108d828983`

Exact constructor:

`DurableRegistryEtcdStore::new(provider)`

The constructor:

- accepts only `DurableRegistryEtcdExecutor`;
- performs no network I/O;
- does not read credentials;
- does not create registry records;
- returns the semantic registry adapter around the bounded provider executor.

The resulting store remains dormant until an explicit semantic registry method is called.

## 6. Existing Agent composition precedent

Exact precedent:

`crates/prw-agent/src/production_reachability_custody_bootstrap.rs`

Exact C03e-IZ blob:

`ba1e9bb318a4d64206eb745ccb33a00d587f87a3`

The repository already uses an Agent-owned facade that:

1. acquires fixed production systemd credentials through a custody crate;
2. receives only a validated opaque config;
3. invokes an existing provider/bootstrap composition seam;
4. exposes a bounded Agent-level error split;
5. accepts no endpoint/trust/certificate/private-key input;
6. creates no retry/fallback/runtime task;
7. does not wire itself into startup/readiness/deployment.

C03e-JA selects the same architecture pattern for durable registry, without importing reachability semantic types.

## 7. Existing dependency topology

Exact Agent manifest:

`crates/prw-agent/Cargo.toml`

Exact blob:

`4c70d6be9b56f39edc10810eefa3428314ed7559`

Agent already depends on:

- `prw-control-plane`;
- `prw-reachability-custody`;
- `prw-registry`.

No Cargo.toml or Cargo.lock change is required for the selected first source materialization.

No reverse dependency or new shared crate is selected.

## 8. Selected Agent-owned facade

C03e-JA selects one crate-internal Agent production composition facade equivalent to:

`bootstrap_production_durable_registry_from_systemd_credentials()`

The facade accepts **no arguments**.

It must not accept:

- endpoint strings;
- trust bundle bytes;
- certificate bytes;
- private-key bytes;
- credential-directory paths;
- raw `Client`;
- raw `KvClient`;
- `DurableRegistryEtcdExecutor` from arbitrary caller input;
- request IDs;
- IP addresses;
- environment-derived registry semantic identity.

The zero-argument shape ensures production provider-security material can only enter through the already-selected fixed systemd custody boundary.

## 9. Selected operation order

The Agent composition facade performs exactly:

1. call `load_durable_registry_production_etcd_bootstrap_config_from_systemd_credentials()`;
2. on custody failure, fail before provider network I/O;
3. move the validated opaque config into `bootstrap_durable_registry_production_executor(config)`;
4. on provider-bootstrap failure, return no semantic store;
5. move the returned bounded executor into `DurableRegistryEtcdStore::new(executor)`;
6. return only the resulting `DurableRegistryEtcdStore`.

No semantic registry method is called by the facade.

No membership/device/transport record is read or written during composition.

## 10. Selected return boundary

The successful facade returns only:

`DurableRegistryEtcdStore`

It does not return a tuple containing:

- raw config;
- credential bytes;
- provider `Client`;
- raw `KvClient`;
- raw executor alongside the store.

Ownership moves forward exactly once:

systemd custody -> validated config -> bounded executor -> semantic store.

A successful composition therefore gives Agent custody of one semantic durable-registry store and no broader provider capability.

## 11. Selected Agent-level error split

The future facade must expose a bounded error type equivalent to:

- `Custody(DurableRegistryCustodyError)`;
- `ProviderBootstrap(DurableRegistryProductionEtcdBootstrapError)`.

No semantic registry error variant is selected because the composition facade performs no registry semantic operation.

No underlying provider `etcd_client::Error` is exposed.

Display strings must not reveal endpoint, trust, certificate or private-key content.

The original bounded error may remain available as `source()` for the two already-sanitized public boundaries.

## 12. Fail-closed partial-construction law

If credential custody fails:

- provider connection is not attempted;
- no executor exists;
- no store exists.

If provider bootstrap fails:

- no executor is returned;
- no semantic store is returned;
- no degraded/offline/in-memory fallback is returned.

A partial or fallback durable registry composition is not selected.

## 13. No in-memory production fallback

The existing Phase 130 `WorkspaceDeviceRegistry` remains a source/disposable semantic model.

C03e-JA does not select fallback from production durable registry to:

- a new empty `WorkspaceDeviceRegistry`;
- cached registry state;
- fixture state;
- session state;
- environment hints;
- reachability state.

If production custody/provider composition fails, composition fails closed.

## 14. No provider retry or reconnect policy

The Agent facade invokes the existing one-shot control-plane provider bootstrap once.

It does not add:

- automatic retry;
- backoff;
- endpoint rotation outside provider behavior;
- reconnect loop;
- background supervisor;
- degraded mode;
- fallback provider;
- cached store.

A later operational/runtime checkpoint must separately select any lifecycle/recovery behavior if required.

## 15. No semantic registry I/O during composition

Creating `DurableRegistryEtcdStore` is not registry readiness proof.

C03e-JA explicitly does not call:

- membership lookup;
- device lookup;
- current transport lookup;
- session validation;
- membership creation;
- device registration;
- transport bind/rotation;
- revocation;
- any etcd Get/Txn/Put through the store.

An empty, unavailable or malformed production registry is not inspected by this composition seam.

## 16. No production-record or migration authority

C03e-JA does not authorize:

- owner/admin seed creation;
- membership seeding;
- device seeding;
- in-memory-to-durable migration;
- import/export;
- namespace marker creation;
- malformed-record repair;
- provider schema migration.

Production registry population remains separately gated.

## 17. No provider-security provisioning authority

C03e-JA consumes only the existing validated config path.

It does not authorize or perform:

- CA issuance;
- client certificate issuance;
- private-key generation;
- credential installation;
- systemd `LoadCredential=` mutation;
- provider user creation;
- provider role creation;
- provider RBAC grant/revoke;
- namespace provisioning.

A successful local composition test does not prove production provider authorization exists.

## 18. No reachability authority aliasing

The returned durable registry store is independent of production reachability ownership/composition.

C03e-JA does not combine registry and reachability provider clients or stores.

No cross-domain transaction or readiness implication is selected.

Reachability success does not imply registry success, and registry composition success does not imply reachability success.

## 19. No startup/readiness activation

The selected facade is a dormant composition seam only.

C03e-JA does not wire it into:

- Agent `run()`;
- `main.rs`;
- Linux bootstrap;
- process startup;
- readiness publication;
- local IPC admission;
- remote session admission;
- listener lifecycle;
- requester/rendezvous;
- candidate publication;
- traversal;
- peer dialing;
- shutdown/restart handling.

A later exact runtime-composition checkpoint is required before production Agent behavior can depend on the store.

## 20. Selected first source-materialization ceiling

After C03e-JA closure, the next separately gated checkpoint may change only:

`crates/prw-agent/src/production_durable_registry_custody_bootstrap.rs`

plus one minimal crate-internal module declaration in:

`crates/prw-agent/src/lib.rs`

No Cargo.toml or Cargo.lock change is expected.

Allowed first source behavior:

- Agent-level bounded error enum;
- zero-argument async facade;
- fixed systemd custody call;
- control-plane provider bootstrap call;
- `DurableRegistryEtcdStore::new(executor)` composition;
- exact type/signature/error-wrap tests that do not require production credentials or endpoints.

Still prohibited:

- runtime/startup callsite;
- registry semantic read/write;
- provider retry/reconnect;
- systemd unit mutation;
- credential provisioning;
- RBAC/provisioning;
- production records;
- deployment.

## 21. Focused next-source tests

The first source checkpoint must prove without production provider access:

- facade has zero-argument async shape;
- return type is only `DurableRegistryEtcdStore`;
- error surface has only custody/provider-bootstrap categories;
- custody errors wrap without secret detail in display;
- provider-bootstrap errors wrap without provider detail in display;
- no endpoint/trust/certificate/private-key argument appears in the public facade;
- no Cargo change is needed.

No live production credential/provider test is required for this pure composition source checkpoint.

## 22. Validation expectations

C03e-JA itself is documentation-only.

Exact-head PRW Rust Validation is required before closure.

Path-filtered AD/AE workflows must be reported by their actual result.

No Android validation is claimed unless the workflow is automatically triggered on the exact JA head.

## 23. Explicit non-authorization

C03e-JA does not authorize or perform:

- Rust source materialization;
- Cargo/lockfile changes;
- production credential read;
- provider connection;
- systemd unit or `LoadCredential` mutation;
- credential provisioning/rotation;
- provider auth/RBAC mutation;
- provider resource provisioning;
- production registry records;
- registry migration;
- semantic registry operations;
- Agent startup/readiness/runtime activation;
- listener/requester/rendezvous/candidate/traversal/dialing activation;
- deployment/restart;
- repository visibility/configuration mutation;
- merge/PR close/ready-for-review;
- branch deletion/history rewrite.

## 24. Closure meaning

C03e-JA closure means only:

`PRODUCTION_DURABLE_REGISTRY_AGENT_CUSTODY_COMPOSITION_BOUNDARY_SELECTED`

Specifically:

- Agent owns the custody-to-provider-to-semantic-store join;
- the production facade has zero provider-security arguments;
- the facade returns only `DurableRegistryEtcdStore`;
- custody failure occurs before provider I/O;
- provider-bootstrap failure returns no store;
- no fallback/retry/semantic I/O is selected;
- runtime activation and production records remain blocked;
- first source ceiling is only `production_durable_registry_custody_bootstrap.rs` plus minimal crate-internal export.

It does not mean production credentials exist, provider RBAC is provisioned, registry records exist, the Agent calls the facade at startup, readiness depends on the store, or deployment is complete.
