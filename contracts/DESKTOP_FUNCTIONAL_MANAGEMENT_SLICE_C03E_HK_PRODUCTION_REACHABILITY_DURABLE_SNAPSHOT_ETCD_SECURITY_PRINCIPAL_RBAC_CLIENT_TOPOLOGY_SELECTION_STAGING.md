# Private Remote Workspace — C03e-HK Production Reachability Durable Snapshot etcd Security Principal / RBAC / Client Topology Selection Staging

Status: `STAGED_SELECTION_ONLY — DOCS_ONLY — NO_RUNTIME_AUTHORIZATION`

## Purpose

This checkpoint selects the concrete security-principal, RBAC-boundary, connection-topology, and client-lifecycle prerequisites for the production reachability durable-snapshot etcd backend.

It is a direct successor to C03e-HJ and preserves the HJ durable-snapshot key/value contract unchanged.

This checkpoint does **not** authorize runtime implementation, etcd authentication activation, user/role provisioning, certificate issuance, endpoint mutation, service deployment, database migration, production rollout, or any privileged host mutation.

## Authoritative predecessor

C03e-HJ locked the durable-snapshot etcd application contract:

- key prefix: `b"/prw/reachability/durable-snapshot/"`;
- exact key: `prefix || peer_id.as_bytes()`;
- canonical fixed-width value: 112 bytes;
- application read operation: exact-key `Get(raw_key)` only;
- application write operation: exact-key `Put(raw_key, raw_value, None)` only;
- no lease attachment;
- no prefix/range scan;
- no `Delete`;
- no `Watch`;
- no compaction operation;
- fail-closed decoding/profile/domain/hash checks;
- injected preconnected `etcd_client::KvClient` boundary.

C03e-HK changes none of those semantics.

## Audited current source boundary

The current control-plane bootstrap already provides the precedent that this selection must preserve:

- `crates/prw-control-plane/src/reachability_acquisition_evidence/bootstrap.rs` owns authoritative etcd connection construction for production reachability;
- the bootstrap config receives exactly three HTTPS endpoints at runtime;
- the endpoint values are not embedded in source;
- one runtime-supplied trust bundle is validated before connection;
- the pinned TLS server name is `reachability-etcd.prw.internal`;
- live-owner and fence identities are supplied separately as certificate/key pairs;
- separate etcd `Client::connect(...)` calls are used for live-owner and fence roles;
- only role-scoped `KvClient` handles cross into the respective backend boundaries;
- raw etcd clients are not exposed from the public control-plane facade.

C03e-HK extends this existing ownership model conceptually. It does not replace or redesign it.

## Selection 1 — Cluster placement

The durable-snapshot backend uses the **same logical three-member reachability-authority etcd cluster** already used by the production live-owner and authority-fence bootstrap.

No separate durable-snapshot cluster is selected.

Rationale:

- durable snapshots belong to the same production reachability authority domain;
- a second cluster would introduce a new placement, failure-domain, deployment, monitoring, and reconciliation problem not justified by the current slice;
- role isolation is achieved through distinct credentials, RBAC, and distinct client connections rather than by inventing another cluster.

This selection does not authorize changing the cluster or deploying any member.

## Selection 2 — Endpoint authority

The authoritative endpoint list remains the existing runtime-supplied, validated three-endpoint vector owned by the reachability etcd bootstrap configuration.

Rules:

1. Exactly three endpoints remain required.
2. Every endpoint remains HTTPS.
3. Endpoint values remain external runtime configuration; no literal production endpoint is added to source.
4. Durable-snapshot client construction must consume the **same validated endpoint vector instance/value set** used for the live-owner and fence connections during the same bootstrap operation.
5. A second independently supplied durable-snapshot endpoint list is forbidden in this slice because it could silently drift from the reachability-authority cluster selected for the other roles.

## Selection 3 — TLS trust boundary

The durable-snapshot connection uses the same runtime-supplied reachability-authority trust bundle and the same pinned TLS server name already enforced by the existing bootstrap:

`reachability-etcd.prw.internal`

Rules:

- no new trust store is selected;
- no system-default trust fallback is selected;
- no insecure transport is selected;
- no TLS verification bypass is selected;
- durable-snapshot identity material is distinct from the trust bundle and from every other role identity.

## Selection 4 — Durable-snapshot security principal

The canonical etcd username/security-principal identifier selected for the durable-snapshot role is:

`prw-reachability-durable-snapshot`

The durable-snapshot client certificate identity must be dedicated to this role and must not reuse the live-owner or authority-fence client identity.

When the authority cluster uses etcd client-certificate authentication with etcd authentication enabled, the certificate Common Name must map to the exact canonical etcd username above.

C03e-HK selects the identity mapping contract only. It does **not** authorize:

- enabling etcd authentication;
- creating the etcd user;
- issuing or rotating certificates;
- changing a CA;
- distributing private keys;
- editing server startup flags;
- restarting etcd members.

## Selection 5 — RBAC role

The canonical etcd RBAC role selected for the durable-snapshot principal is:

`prw-reachability-durable-snapshot-rw`

The role is bound only to the durable-snapshot key prefix:

`/prw/reachability/durable-snapshot/`

No permission outside that prefix is permitted by this selection.

The selected provider-level permission class is the smallest etcd-native grant that supports the currently required durable-snapshot application operations: read/write access scoped to that prefix.

The canonical principal has no selected access to:

- live-owner keys;
- authority-fence keys;
- enrollment keys;
- device-registry keys;
- unrelated control-plane prefixes;
- cluster administration;
- member administration;
- authentication administration;
- role administration.

## Provider-enforceable excess and application ceiling

C03e-HJ requires only exact-key `Get` and exact-key `Put` for durable snapshots.

etcd RBAC expresses permissions by operation class (`read`, `write`, or `readwrite`) over a key/range/prefix. It does not provide a distinct capability that means "allow Put but deny Delete" or "allow exact Get but deny every range read" while still granting the required operations over a dynamic per-peer keyspace.

Therefore the smallest provider-enforceable excess selected by C03e-HK is:

- etcd-native read/write permission over **only** `/prw/reachability/durable-snapshot/`.

This provider-level permission does **not** expand the PRW application contract.

The durable-snapshot backend remains contractually limited to:

- exact-key `Get`;
- exact-key `Put` without a lease.

It remains forbidden for PRW source in this slice to issue:

- `Delete`;
- prefix/range scans;
- arbitrary range reads;
- `Watch`;
- lease operations;
- compaction operations;
- administrative etcd operations.

A future runtime-materialization checkpoint must preserve tests or equivalent evidence that the backend still uses only the HJ operation set.

## Selection 6 — Credential material boundary

Future runtime configuration may extend the existing reachability etcd bootstrap configuration with a dedicated durable-snapshot client identity pair:

- durable-snapshot client certificate PEM;
- durable-snapshot client private-key PEM.

The existing trust bundle and endpoint vector are reused as shared authority inputs; the private client credential is not shared across roles.

Required validation before any connection attempt:

1. durable-snapshot certificate material is non-empty;
2. durable-snapshot private-key material is non-empty;
3. all existing endpoint/trust validations continue to pass;
4. existing live-owner and fence credential validations remain unchanged.

No credential value is to be embedded in source, contract files, CI logs, or audit reports.

## Selection 7 — Client topology

The selected topology is **three separately established role connections** to the same validated reachability-authority endpoint set:

1. live-owner client connection;
2. authority-fence client connection;
3. durable-snapshot client connection.

The durable-snapshot client must be created through its own `Client::connect(...)` call using:

- the shared validated endpoint list;
- the shared validated reachability-authority trust bundle;
- the pinned TLS server name;
- the dedicated durable-snapshot certificate/key identity.

The durable-snapshot backend receives only the `KvClient` obtained from that dedicated role connection.

Forbidden topology changes in this slice:

- reusing the live-owner client identity for durable snapshots;
- reusing the fence client identity for durable snapshots;
- passing one raw etcd `Client` into multiple role backends as a new shared authority surface;
- introducing a global etcd singleton;
- introducing an independent durable-snapshot endpoint configuration;
- introducing a new process/service solely to own the durable-snapshot connection.

## Selection 8 — Lifecycle ownership and teardown

Connection construction remains owned by the existing reachability control-plane bootstrap boundary.

The future durable-snapshot `KvClient` handle is owned for the lifetime of the durable-snapshot backend/executor that consumes it, following the same etcd-client handle-lifetime model already used by the live-owner and fence paths.

Selected lifecycle rules:

1. validate configuration before any role connection is attempted;
2. create the durable-snapshot role connection during reachability bootstrap, alongside the existing role connections;
3. inject only the durable role's `KvClient` into the durable-snapshot backend;
4. retain no additional global connection owner;
5. do not add a bespoke reconnect daemon, background supervisor, or service in this checkpoint;
6. normal owner teardown uses ordinary Rust handle drop/process teardown semantics unless a later audited provider requirement proves an explicit close sequence is necessary.

C03e-HK does not authorize changing retry/reconnect semantics beyond what the currently selected etcd client library already provides.

## Selection 9 — Public API boundary

No new public control-plane API is selected.

Concrete etcd connection types, credential types, and durable-snapshot backend construction remain private implementation details under the existing `reachability_acquisition_evidence` boundary.

The public facade must continue to expose domain-level preparation/execution results rather than raw etcd clients.

## Dependency ceiling

No dependency change is required for the selected design.

The current control-plane crate already includes the selected etcd client with TLS support. C03e-HK does not authorize dependency upgrades, feature expansion, or unrelated dependency cleanup.

## Future source-materialization ceiling

A later explicitly authorized runtime-materialization checkpoint may implement only the minimum source changes needed to realize this selection, expected primarily in:

- `crates/prw-control-plane/src/reachability_acquisition_evidence/bootstrap.rs`;
- narrowly related tests/evidence for bootstrap validation and role isolation;
- narrowly related durable-snapshot wiring only if required to inject the dedicated `KvClient`.

That later checkpoint must not broaden into redesign or unrelated refactoring.

In particular, retaining the existing bootstrap config type name is acceptable for a minimal implementation even if the name reflects its original live-owner scope. A rename/refactor is not required by this contract.

## Explicitly not authorized by C03e-HK

This checkpoint does not authorize any of the following:

- Rust source modification;
- runtime wiring;
- etcd authentication enablement or disablement;
- etcd user creation/deletion/password mutation;
- etcd role creation/deletion/grant/revoke operations;
- certificate generation, issuance, rotation, or installation;
- CA/trust-bundle mutation;
- server configuration changes;
- endpoint changes;
- etcd member changes;
- cluster deployment;
- service activation/restart;
- DNS mutation;
- firewall/network mutation;
- privileged host mutation;
- database migration;
- production rollout;
- remote-access activation;
- unrelated cleanup, refactoring, or dependency upgrades.

## Implementation-validation requirements for the later runtime checkpoint

Before any later source materialization can be considered complete, evidence must show at minimum:

1. existing three-endpoint HTTPS validation still passes;
2. trust-bundle validation still passes;
3. live-owner and fence identity validation still passes unchanged;
4. durable-snapshot certificate/key validation fails closed when absent;
5. durable-snapshot connection construction uses a distinct identity input;
6. all three role connections consume the same validated endpoint set;
7. the durable backend receives only its dedicated `KvClient`;
8. no raw etcd client becomes part of the public facade;
9. the HJ key/value mapping remains byte-for-byte unchanged;
10. durable backend operations remain limited to exact-key `Get` and exact-key `Put` without leases;
11. formatting, clippy, tests, and workspace build succeed, or any environment/tooling failure is clearly separated from a source defect.

## Audit interpretation

C03e-HK is a selection checkpoint, not a runtime implementation checkpoint.

Successful completion means only that the previously unresolved security-principal, RBAC scope, endpoint/trust authority, client topology, and lifecycle ownership questions are locked tightly enough for a subsequent minimal implementation checkpoint.

It must not be interpreted as evidence that production etcd authentication, credentials, RBAC grants, or durable-snapshot runtime wiring have been configured or validated on a live authority cluster.

## Next checkpoint readiness

After this contract is reviewed and accepted, the next safe checkpoint may materialize the selected durable-snapshot etcd connection/bootstrap wiring in source under the source-materialization ceiling above.

That successor must remain implementation-only and must not provision or activate etcd authentication/RBAC/certificates/deployment unless separately and explicitly authorized.
