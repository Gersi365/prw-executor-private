# Phase 152 C03e-IW — Production Durable Registry Provider Bootstrap Boundary Selection — STAGING

Status: `SELECTION_ONLY_STAGED`

Selected gate:

`C03E_IW_PRODUCTION_DURABLE_REGISTRY_PROVIDER_BOOTSTRAP_BOUNDARY_SELECTED`

## 1. Scope

C03e-IW is the documentation-only prerequisite after closed C03e-IV. It selects the production provider bootstrap ownership/configuration boundary required to turn the already-materialized raw durable-registry etcd executor and semantic durable-registry store into a production-connectable but still non-activated composition.

C03e-IW does not materialize Rust/Kotlin/Cargo/lockfile/workflow/runtime source, connect to etcd, read a production credential directory, create or rotate credentials, modify TLS/auth/RBAC, provision provider resources, populate durable registry records, migrate registry state, wire Agent startup/readiness/runtime, deploy, merge, close a PR, or delete a branch.

## 2. Exact predecessor authority

Exact predecessor C03e-IV head:

`95c4a78ecfd05290ede6366904281603cb45e28a`

Exact predecessor tree:

`fff16a405ff6ba94495fa24fe23e6f5cd69edd8c`

C03e-IV materialized only:

- `crates/prw-registry/src/durable_registry_etcd_store.rs`;
- one minimal module export in `crates/prw-registry/src/lib.rs`.

C03e-IV deliberately left provider endpoint/security/bootstrap, production registry population, Agent production composition and runtime activation unselected.

## 3. Existing registry provider mechanics

Exact source:

`crates/prw-control-plane/src/durable_registry_etcd.rs`

C03e-IT owns raw etcd mechanics only. Its executor accepts an already-created `etcd_client::KvClient` and exposes only bounded exact-key/transaction operations selected by C03e-IQ/C03e-IS.

It does not connect to endpoints, choose trust, receive credentials, decode PRWM/PRWD, classify registry semantics, retry mutations, scan prefixes, use Watch/Lease/TTL, create registry records on construction, or activate runtime behavior.

The production bootstrap seam must therefore terminate in exactly one dedicated `DurableRegistryEtcdExecutor`; broad `Client`/`KvClient` custody must not escape that boundary.

## 4. Existing semantic registry owner

Exact source:

`crates/prw-registry/src/durable_registry_etcd_store.rs`

C03e-IV materializes `DurableRegistryEtcdStore` around one already-created `DurableRegistryEtcdExecutor`.

The store owns canonical registry semantic behavior including exact key/value/request binding, membership/device transitions, current transport validation, session revalidation and authoritative compare-failure classification.

The provider bootstrap layer must not import or reproduce these semantics.

## 5. Production reachability bootstrap precedent

Exact control-plane precedent:

`crates/prw-control-plane/src/reachability_acquisition_evidence/bootstrap.rs`

Exact Agent precedent:

`crates/prw-agent/src/production_reachability_bootstrap.rs`

Exact custody precedent:

`crates/prw-reachability-custody/src/lib.rs`

The existing production reachability chain demonstrates these repository laws:

1. endpoint/trust/client-identity validation belongs below Agent semantic composition;
2. private-key material is retained in non-printable/zeroizing custody before provider connection;
3. provider bootstrap receives validated opaque configuration;
4. `Client::connect` occurs in control-plane provider bootstrap;
5. broad `Client` handles are dropped immediately after extracting role-scoped `KvClient` values;
6. only narrowed provider executors/preparations leave control-plane bootstrap;
7. credential loading is separate from provider connection;
8. Agent composition does not accept raw endpoint/certificate/private-key bytes;
9. bootstrap failure returns no partial/degraded composition;
10. no retry/fallback/runtime activation is implied merely by having a bootstrap function.

C03e-IW adopts these architecture laws for registry without reusing reachability-specific semantic types or credential names.

## 6. Registry authority-cluster selection law

C03e-IW selects a registry-specific immutable production etcd authority bootstrap input consisting of:

- exactly three client endpoints;
- one explicit private authority trust bundle;
- one dedicated registry-authority mTLS client identity.

The three-endpoint structural law matches the already-selected production etcd authority topology precedent:

- endpoint scheme must be HTTPS;
- host must be a stable ASCII FQDN;
- no plaintext HTTP;
- no localhost;
- no IP literals;
- no wildcard host;
- no path/query/fragment/user-info;
- optional port must be valid and non-zero;
- textual member FQDNs must be unique;
- exactly three endpoints are required.

C03e-IW selects the structural bootstrap law, not concrete endpoint values.

## 7. No implicit reachability-cluster aliasing

C03e-IW does **not** infer registry endpoint or trust values from reachability configuration.

There is no selected semantic transaction spanning registry keys and reachability keys. Registry membership/device atomicity is entirely inside the `/prw/registry/...` domain, while reachability authority owns `/prw/reachability/...` domains.

Therefore:

- reachability endpoint credentials are not registry authority merely because both providers are etcd;
- reachability trust material is not registry trust material by implication;
- a registry bootstrap must receive its own explicit validated endpoint/trust input;
- operations may later choose equal infrastructure values only through explicit deployment/provisioning decisions outside this contract.

C03e-IW neither requires nor forbids physical cluster co-location. It forbids silently deriving one authority domain from another.

## 8. Dedicated registry client identity

Production durable registry provider access uses one dedicated role-scoped mTLS client identity.

That identity is not:

- the reachability live-owner identity;
- the reachability fence-allocator identity;
- the reachability durable-snapshot identity;
- an Agent identity;
- a device identity;
- a transport identity;
- a username/password fallback.

The registry identity is provider-access identity only. Provider authentication success does not classify registry membership/device semantic state.

Client certificate and private-key bytes are runtime-supplied and are not embedded in source/contracts.

## 9. Registry bootstrap config ownership

C03e-IW selects `prw-control-plane` as owner of the validated provider bootstrap config and provider connect function.

The future config type must be provider-specific but registry-semantic-neutral. A selected shape equivalent to the following is allowed:

`DurableRegistryProductionEtcdBootstrapConfig`

retaining by value:

- exact endpoint vector;
- explicit trust bundle bytes;
- dedicated registry client identity material.

Construction performs structural validation only and no network I/O.

The future bootstrap function may perform exactly one provider connection attempt using the validated input and return exactly one narrowed `DurableRegistryEtcdExecutor`.

No broad provider client may be returned.

## 10. Private-key custody law

Future registry bootstrap source must preserve the existing repository security custody pattern:

- private-key material must not implement ordinary printable/debug exposure;
- PRW-owned private-key plaintext must be retained in a zeroizing owner while in custody;
- the provider TLS identity receives the key only at the connection boundary;
- source code contains no production key/certificate bytes;
- errors must not echo key/certificate/trust/endpoint contents;
- no accessor returns private-key bytes.

C03e-IW does not select concrete credential files or create secret material.

## 11. Provider connection law

The future control-plane bootstrap function must:

1. consume one already-validated registry bootstrap config;
2. create TLS options from the explicit private trust bundle and dedicated registry identity;
3. call provider connect once for the exact supplied endpoint set;
4. fail closed if the provider connection cannot be established;
5. extract exactly one `KvClient` from the broad provider client;
6. drop the broad `Client` handle;
7. move the role-scoped `KvClient` directly into `DurableRegistryEtcdExecutor::new(...)`;
8. return only the narrowed executor.

No retry, fallback, endpoint discovery, alternate trust roots, plaintext downgrade, username/password path, serializable-read policy, Watch/Lease/TTL, background task or runtime activation is selected.

## 12. Bootstrap failure law

The future public bootstrap error must be bounded and must not retain or expose provider connection detail capable of leaking endpoint/security material.

At minimum the selected public failure classes are:

- invalid identity material before network I/O;
- invalid registry authority bootstrap configuration before network I/O;
- registry provider connection failure.

Underlying provider errors must not escape into Agent/registry semantic APIs.

A connection failure returns no executor and no partial/degraded provider result.

## 13. Semantic composition law after provider bootstrap

After successful provider bootstrap, the dedicated executor may be moved directly into:

`prw_registry::durable_registry_etcd_store::DurableRegistryEtcdStore::new(...)`

This construction performs no additional provider connection.

The resulting semantic store remains dormant until an explicit caller performs a registry operation.

C03e-IW does not select any Agent startup callsite for that store.

## 14. Credential-source separation

C03e-IW deliberately keeps credential acquisition separate from provider connection.

The exact systemd credential names, custody crate/path, service-unit declarations and deployment provisioning for registry credentials remain deferred to a later separately gated custody checkpoint.

The provider bootstrap source must therefore accept already-owned validated input; it must not read `$CREDENTIALS_DIRECTORY`, environment variables, arbitrary files, command-line arguments or secret stores directly.

## 15. No direct Agent secret input

A future Agent semantic composition facade must not accept:

- endpoint strings;
- CA bytes;
- certificate bytes;
- private-key bytes;
- credential directory path;
- raw `Client`;
- raw `KvClient`.

Those are below the Agent composition boundary.

Agent may later accept only an opaque validated bootstrap config or, after custody join is selected, no provider-security arguments at all.

## 16. No automatic production record creation

Successful provider bootstrap proves only that one authenticated provider connection was narrowed into a registry executor.

It must not automatically:

- create membership records;
- create device records;
- bind transport identity;
- migrate in-memory registry state;
- seed admin/owner membership;
- rewrite malformed records;
- probe by prefix scan;
- create namespace markers;
- infer registry readiness from an empty keyspace.

Production durable registry population remains a separate migration/provenance gate.

## 17. No readiness/runtime authority

Provider connection success alone does not authorize Agent readiness or networking.

C03e-IW does not select changes to:

- `crates/prw-agent/src/main.rs`;
- `crates/prw-agent/src/linux_bootstrap.rs`;
- production runtime lifecycle/readiness modules;
- requester/rendezvous/candidate publication;
- listener/socket startup;
- traversal/dialing;
- systemd service state.

Any runtime callsite requires a later explicit composition/activation contract.

## 18. No cross-domain security inference

The registry provider identity must be authorized only for the minimum registry namespace operations selected by the provider/security deployment contract.

C03e-IW does not authorize RBAC mutation and does not claim existing RBAC is sufficient.

A later provisioning/security checkpoint must prove the exact provider permissions. It may not infer permission to reachability/fence namespaces from registry connectivity.

Likewise, reachability roles may not be inferred to have registry namespace authority.

## 19. Exact namespace preservation

C03e-IQ selected exact registry namespace prefixes:

- `/prw/registry/membership/`
- `/prw/registry/device/`

C03e-IW does not change those prefixes.

Bootstrap configuration does not carry a dynamic key prefix. Runtime configuration cannot redirect the semantic registry to another namespace.

## 20. First source-materialization ceiling

After C03e-IW closure, the next separately gated source checkpoint may materialize only the control-plane provider bootstrap boundary.

Selected first source path:

`crates/prw-control-plane/src/durable_registry_etcd_bootstrap.rs`

Allowed companion change:

- minimal module declaration/export in `crates/prw-control-plane/src/lib.rs`.

Allowed source behavior:

- registry role identity material carrier;
- registry bootstrap config validation;
- exact three-endpoint structural validation;
- explicit trust-bundle validation;
- TLS option construction;
- one provider connection attempt;
- broad `Client` -> dedicated `KvClient` narrowing;
- construction of `DurableRegistryEtcdExecutor`;
- provider-free focused config tests.

Still prohibited in that first source checkpoint:

- Cargo/lockfile changes unless exact source proves unavoidable and a separate gate authorizes them;
- systemd credential reading;
- credential-name definitions;
- secret provisioning;
- auth/RBAC mutation;
- production record creation;
- semantic store operations;
- Agent composition;
- runtime activation;
- deployment.

## 21. Focused source test matrix

The first bootstrap source checkpoint must test without a live endpoint at least:

- exact three-member HTTPS FQDN configuration acceptance;
- wrong member-count rejection;
- plaintext HTTP rejection;
- IP/localhost/wildcard rejection;
- path/query/fragment/user-info rejection;
- invalid/zero port rejection;
- duplicate member-FQDN rejection;
- empty trust rejection;
- empty certificate rejection;
- empty private-key rejection;
- private-key carrier non-debug/non-clone intent where statically enforceable;
- exact return/signature boundary showing narrowed `DurableRegistryEtcdExecutor` only.

Live provider connection validation remains a later disposable/operational checkpoint unless an existing path-filtered workflow already authoritatively covers the exact new source.

## 22. Later prerequisites remain blocked

After provider bootstrap source materialization, separate checkpoints remain required for:

1. registry credential custody/source selection;
2. credential custody source materialization;
3. exact provider RBAC/provisioning law and evidence;
4. production registry population/migration provenance;
5. Agent production registry composition;
6. current logical-device/current-transport lookup composition into production peer provenance;
7. runtime/readiness integration;
8. deployment/operational validation.

No later step is pre-authorized by C03e-IW.

## 23. Closure meaning

C03e-IW closure means only:

`PRODUCTION_DURABLE_REGISTRY_PROVIDER_BOOTSTRAP_BOUNDARY_SELECTED`

Specifically:

- provider bootstrap belongs in `prw-control-plane`;
- registry uses an explicit registry-specific three-endpoint HTTPS/FQDN authority config;
- registry receives an explicit private trust bundle and one dedicated role-scoped mTLS identity;
- reachability config/credentials are not silently reused as registry authority;
- one successful provider connection is narrowed immediately into `DurableRegistryEtcdExecutor`;
- broad provider clients do not escape;
- private-key custody follows the existing zeroizing/non-printable pattern;
- credential loading remains separate and deferred;
- production records/runtime/readiness/deployment remain blocked;
- the first source ceiling is only `durable_registry_etcd_bootstrap.rs` plus minimal control-plane module export.

It does not mean endpoint values exist, credentials exist, RBAC is provisioned, production records exist, Agent is wired, runtime is active, or deployment is complete.
