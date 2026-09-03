# Phase 152 C03e-IY — Production Durable Registry systemd Credential Custody Boundary Selection — STAGING

Status: `SELECTION_ONLY_STAGED`

Selected gate:

`C03E_IY_PRODUCTION_DURABLE_REGISTRY_SYSTEMD_CREDENTIAL_CUSTODY_BOUNDARY_SELECTED`

## 1. Scope

C03e-IY is the documentation-only prerequisite after closed C03e-IX. It selects the bounded production service-credential custody source required to construct the already-materialized registry-specific control-plane etcd bootstrap config without exposing provider secrets to Agent composition.

C03e-IY does not materialize Rust/Kotlin/Cargo/lockfile/workflow/systemd-unit/runtime source, create or rotate credentials, write credential files, change `LoadCredential`, connect to etcd, mutate TLS/auth/RBAC, provision provider resources, populate registry records, migrate state, wire Agent startup/readiness/runtime, deploy, merge, close a PR, mark a PR ready, or delete a branch.

## 2. Exact predecessor authority

Exact predecessor C03e-IX head:

`e7cde86df2176548ecc5431acb31ab0e4eafc04c`

Exact predecessor C03e-IX provider-bootstrap source:

`crates/prw-control-plane/src/durable_registry_etcd_bootstrap.rs`

Final exact C03e-IX bootstrap blob:

`787c82fe19a8a11428193921c696d3cfc551fb60`

C03e-IX already materializes:

- `DurableRegistryEtcdClientIdentityMaterial`;
- zeroizing private-key custody inside that type;
- `DurableRegistryProductionEtcdBootstrapConfig`;
- exact three-member HTTPS/FQDN endpoint validation;
- explicit private trust-bundle validation;
- one dedicated registry mTLS identity;
- exactly one provider `Client::connect` attempt;
- immediate broad `Client -> KvClient -> DurableRegistryEtcdExecutor` narrowing;
- bounded provider connection failure;
- no systemd credential read;
- no concrete endpoint/certificate/key values;
- no RBAC/provisioning;
- no production record creation;
- no Agent/runtime activation.

C03e-IY does not reopen those decisions.

## 3. Existing custody topology

Exact C03e-IX `prw-reachability-custody` manifest:

`crates/prw-reachability-custody/Cargo.toml`

Exact blob:

`f8ff3ecdac1e5cb0b580818ad6f55ae6076ff4f7`

It already depends on:

- `prw-control-plane`;
- `zeroize = 1.9.0`;
- Linux `rustix` filesystem/process features.

Exact C03e-IX Agent manifest:

`crates/prw-agent/Cargo.toml`

Exact blob:

`4c70d6be9b56f39edc10810eefa3428314ed7559`

Agent already depends on both:

- `prw-reachability-custody`;
- `prw-control-plane`;
- `prw-registry`.

Therefore no new dependency direction is required for a registry credential custody module inside the existing custody crate.

## 4. Existing non-reachability custody precedent

`crates/prw-reachability-custody/src/mesh_transport_custody.rs`

Exact C03e-IX blob:

`679fea2eb5bfc7345d14166a4481546d4174c050`

Although the crate name originated with reachability, the crate already owns a separate fixed systemd service-credential custody module for mesh transport identity.

The mesh custody precedent establishes that this crate is already the repository's bounded Linux service-credential custody boundary for more than one production domain.

The precedent includes:

- fixed credential names;
- `$CREDENTIALS_DIRECTORY` as the systemd-provided directory locator;
- Linux-only acquisition;
- absolute credential-directory requirement;
- directory ownership and permission validation;
- no symlink/unstable-file acceptance;
- exact regular-file requirement;
- effective-service-user ownership validation;
- locked credential permission checks;
- bounded reads;
- zeroizing private-key buffers;
- no network I/O;
- no runtime readiness or deployment activation.

C03e-IY selects the same custody architecture for durable registry authority.

## 5. Selected custody owner

C03e-IY selects:

`prw-reachability-custody`

as the existing crate in which the durable-registry systemd custody adapter belongs.

The selected future module path is:

`crates/prw-reachability-custody/src/durable_registry_custody.rs`

with a minimal module export from:

`crates/prw-reachability-custody/src/lib.rs`

This selection deliberately avoids:

- a new crate merely for naming purity;
- a new Cargo dependency;
- direct filesystem/systemd credential access from `prw-agent`;
- direct filesystem/systemd credential access from `prw-control-plane` provider bootstrap;
- direct secret acquisition from `prw-registry`.

## 6. Selected fixed credential set

The production durable-registry custody adapter reads exactly six fixed systemd service credentials:

1. `prw.registry.authority-endpoint-1.v1`
2. `prw.registry.authority-endpoint-2.v1`
3. `prw.registry.authority-endpoint-3.v1`
4. `prw.registry.authority-ca-bundle.v1`
5. `prw.registry.client-certificate.v1`
6. `prw.registry.client-private-key.v1`

These names are selected identifiers only.

C03e-IY does not create, provision, rotate, install or assign any credential with these names.

No reachability credential name is reused as registry authority.

## 7. Credential-directory source

The selected runtime directory source is the existing systemd convention:

`CREDENTIALS_DIRECTORY`

The custody adapter may read that environment variable only as the systemd-provided directory locator.

It must not treat arbitrary environment variables as endpoint, trust, certificate or private-key authority.

The directory value must:

- be present and non-empty;
- resolve to an absolute path;
- pass the existing locked directory ownership/permission checks before any credential read.

No caller-supplied arbitrary credential directory is selected for the production facade.

## 8. Fixed-file lookup law

Each credential path is formed only by joining the validated systemd credential directory with one fixed selected credential name.

The adapter must not:

- accept a caller-supplied credential filename;
- scan a directory to discover credentials;
- choose a first matching file;
- follow symlink chains as authority;
- accept device/FIFO/socket/directory objects as credentials;
- fall back to another directory;
- search `$HOME`, `/etc`, working directory or command-line paths.

## 9. Directory and file security law

The future source must preserve the hardened custody rules already established in the existing custody crate.

At minimum, fail closed when:

- platform is not Linux;
- `CREDENTIALS_DIRECTORY` is missing/empty;
- credential directory is not absolute;
- directory ownership does not match the effective service user;
- directory permissions allow insecure group/other mutation;
- a fixed credential cannot be securely opened;
- a fixed path is not one stable regular file;
- credential ownership does not match the effective service user;
- credential permissions violate the locked runtime policy;
- bounded read fails;
- credential is empty;
- credential exceeds its type-specific upper bound.

No insecure permission is normalized or repaired by the reader.

## 10. Selected byte bounds

C03e-IY selects the existing production etcd custody bounds already used by reachability for equivalent serialized material:

- each endpoint credential: maximum `2_048` bytes;
- authority CA bundle: maximum `262_144` bytes;
- client certificate: maximum `131_072` bytes;
- client private key: maximum `32_768` bytes.

Each credential must also be non-empty.

The endpoint credentials must decode as valid UTF-8 before being passed to control-plane bootstrap validation.

The custody layer does not duplicate full endpoint/FQDN/TLS semantic validation; it delegates exact provider-config validation to C03e-IX's `DurableRegistryProductionEtcdBootstrapConfig`.

## 11. Private-key custody law

The registry private-key credential must be read directly into `Zeroizing<Vec<u8>>` or an equivalently zeroizing owned buffer already established by the crate.

The key must then move directly into:

`DurableRegistryEtcdClientIdentityMaterial::new_with_zeroizing_private_key(...)`

The future custody source must not:

- expose a borrowed private-key slice through public API;
- expose a mutable private-key accessor;
- clone the private-key buffer;
- convert the key into an ordinary long-lived PRW-owned plaintext buffer after read;
- implement `Debug` output that prints key bytes;
- include key bytes in errors or logs.

## 12. Non-secret material law

Endpoint strings, CA bundle bytes and certificate bytes are not private keys, but remain bounded provider-bootstrap inputs.

The custody adapter may move them into the existing control-plane bootstrap config.

It must not expose an alternate public raw-material carrier when the exact config can be constructed directly.

## 13. Selected output boundary

The selected production custody facade returns only:

`DurableRegistryProductionEtcdBootstrapConfig`

from `prw-control-plane`.

It does not return:

- raw endpoint strings;
- raw CA bundle bytes;
- raw certificate/private-key pairs;
- `Client`;
- `KvClient`;
- `DurableRegistryEtcdExecutor`;
- `DurableRegistryEtcdStore`.

Provider connection therefore remains exclusively in the C03e-IX control-plane bootstrap function.

## 14. Selected facade behavior

The future public facade may be equivalent to:

`load_durable_registry_production_etcd_bootstrap_config_from_systemd_credentials()`

Its law is:

1. validate Linux/systemd credential-directory custody;
2. read exactly the six fixed credentials;
3. decode exactly three endpoint credentials as UTF-8;
4. retain the private key in zeroizing ownership;
5. construct `DurableRegistryEtcdClientIdentityMaterial`;
6. construct `DurableRegistryProductionEtcdBootstrapConfig`;
7. return only that validated opaque config.

The facade performs no network I/O.

## 15. Failure boundary

The future registry custody API must expose bounded custody failures equivalent to:

- `UnsupportedPlatform`;
- `CredentialsDirectoryMissing`;
- `CredentialsDirectoryInvalid`;
- `CredentialsDirectoryNotSecure`;
- `CredentialUnavailable`;
- `CredentialNotRegular`;
- `CredentialOwnershipMismatch`;
- `CredentialPermissionsInsecure`;
- `CredentialReadFailed`;
- `CredentialSizeOutOfBounds`;
- `EndpointEncodingInvalid`;
- `IdentityMaterial(...)`;
- `BootstrapConfig(...)`.

Error display must remain non-secret and must not include endpoint, certificate, private-key, trust-bundle or full credential-path content.

## 16. No provider I/O in custody

The custody adapter must not call:

- `Client::connect`;
- etcd Get/Txn/Put;
- DNS discovery;
- provider auth/RBAC APIs;
- retry/backoff/reconnect;
- Watch/Lease/TTL.

Its output is inert validated bootstrap configuration only.

## 17. No production credential provisioning

C03e-IY does not select or authorize:

- how credentials are generated;
- certificate issuer/CA operations;
- private-key generation;
- provider RBAC creation;
- systemd `LoadCredential=` lines;
- credential source files outside service runtime;
- ownership changes;
- chmod/chown mutations;
- secret rotation cadence;
- deployment mechanism.

A successful custody loader does not prove that any production credential is provisioned today.

## 18. No implicit reachability reuse

The registry custody adapter uses registry-specific fixed names.

It must not silently read:

- `prw.reachability.authority-endpoint-*.v1`;
- `prw.reachability.authority-ca-bundle.v1`;
- reachability live-owner credentials;
- reachability fence-allocator credentials;
- reachability durable-snapshot credentials.

Equal infrastructure values, if ever intentionally provisioned, remain an operations/deployment decision and are not inferred by custody code.

## 19. Agent composition remains deferred

C03e-IY does not add an Agent callsite.

A later composition checkpoint must join:

1. registry credential custody -> validated opaque registry bootstrap config;
2. C03e-IX control-plane bootstrap -> `DurableRegistryEtcdExecutor`;
3. C03e-IV semantic adapter -> `DurableRegistryEtcdStore`;
4. only then the specifically selected Agent consumer seam.

Agent must not accept endpoint/certificate/private-key bytes directly.

## 20. First source-materialization ceiling

After C03e-IY closure, the next source checkpoint may materialize only:

`crates/prw-reachability-custody/src/durable_registry_custody.rs`

plus minimal:

`pub mod durable_registry_custody;`

in:

`crates/prw-reachability-custody/src/lib.rs`

No Cargo.toml or Cargo.lock change is expected because the existing crate already owns the required dependencies.

Allowed first-source behavior:

- fixed six credential-name constants;
- Linux-only systemd directory lookup;
- hardened directory/file validation equivalent to existing custody precedent;
- bounded reads;
- endpoint UTF-8 decode;
- zeroizing registry private-key ownership;
- direct construction of C03e-IX identity/config types;
- provider-free focused tests.

Still prohibited:

- provider connection;
- systemd unit mutation;
- credential provisioning;
- RBAC/provisioning;
- production registry records;
- Agent composition;
- runtime activation;
- deployment.

## 21. Focused next-source test matrix

The next source checkpoint must cover, without production credentials:

- unsupported platform fails closed;
- missing/empty credential directory value;
- relative directory rejection;
- insecure/wrong-owner directory rejection where test platform permits;
- missing credential rejection;
- symlink/non-regular credential rejection;
- wrong-owner/insecure-permission credential rejection where test platform permits;
- empty and oversized endpoint/CA/certificate/private-key rejection;
- endpoint UTF-8 rejection;
- exact six fixed-name surface;
- private-key transfer remains zeroizing;
- successful fixture custody returns only `DurableRegistryProductionEtcdBootstrapConfig`;
- no provider network I/O is needed by tests.

## 22. Explicitly not authorized

C03e-IY does not authorize or perform:

- Rust source materialization;
- Cargo/lockfile changes;
- concrete production endpoint/trust/certificate/private-key values;
- credential file creation;
- secret provisioning/rotation;
- systemd unit changes;
- provider connection;
- provider auth/RBAC mutation;
- provider resource provisioning;
- production registry population/migration;
- Agent production composition;
- startup/readiness/runtime changes;
- listener/requester/rendezvous/candidate/traversal/dialing activation;
- deployment/restart;
- repository visibility/configuration mutation;
- merge/PR close/ready-for-review transition;
- branch deletion/history rewrite.

## 23. Validation law

C03e-IY closure requires:

- exact IX -> IY compare with exact IX merge base;
- exactly one changed docs path;
- exact-head automatic Rust validation SUCCESS;
- path-filtered workflows recorded accurately;
- immutable Drive audit with pre-upload zero exact-title matches;
- raw Drive readback with exact bytes and SHA-256;
- exactly one post-upload canonical artifact;
- PR remains draft/open/unmerged.

## 24. Closure meaning

C03e-IY closes only:

`PRODUCTION_DURABLE_REGISTRY_SYSTEMD_CREDENTIAL_CUSTODY_BOUNDARY_SELECTED`

It establishes the fixed credential names, existing custody crate ownership, hardened read law, zeroizing secret handoff and opaque-config output boundary.

It does not establish that production credentials exist, that systemd loads them, that provider RBAC is provisioned, that the registry contains production records, or that Agent/runtime composition is active.
