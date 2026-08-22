# Phase 152 C02f-CE — Reachability Custody Source Materialization Staging

Status: `MATERIALIZING / DEDICATED_REACHABILITY_CUSTODY_CRATE / SYSTEMD_CREDENTIALS_DIRECTORY_ONLY / FIXED_VERSIONED_CREDENTIAL_NAMES / O_NOFOLLOW_AND_FILE_IDENTITY_GUARDS / BOUNDED_READS / PRIVATE_KEYS_ZEROIZING / OPAQUE_CONTROL_PLANE_CONFIG_RETURN / NO_REAL_VALUES / NO_SERVICE_PROVISIONING / NO_PROVIDER_BOOTSTRAP_INVOCATION / NO_RUNTIME_ACTIVATION / NO_RECOVERY / NO_R1_R4 / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-22  
Repository: `Gersi365/prw-executor-private`

## Exact prerequisite

C02f-CE derives only from canonical C02f-CD:

- branch: `phase-152-c02f-cd-control-plane-zeroizing-private-key-handoff-source-materialization-staging`;
- head: `e93f45ed55acdb4cf7cdc4e7a4d53a578f8a7c30`;
- tree: `0745aa0ab2b7413b6062ba6e572a5669672213e1`;
- gate: `C02F_CD_CONTROL_PLANE_ZEROIZING_PRIVATE_KEY_HANDOFF_SOURCE_MATERIALIZED`.

## Scope

CE materializes the dedicated Linux custody boundary selected by C02f-CC. The crate is:

`crates/prw-reachability-custody`

Its single public acquisition function loads the fixed Phase 152 reachability authority credential set from systemd service credentials and returns only one validated opaque control-plane value:

`ReachabilityLiveOwnerEtcdBootstrapConfig`

The crate does not own provider execution, authority lifecycle semantics, endpoint discovery, service provisioning, or Agent runtime activation.

## Fixed credential identities

The only service-visible credential names accepted by this crate are:

1. `prw.reachability.authority-endpoint-1.v1`
2. `prw.reachability.authority-endpoint-2.v1`
3. `prw.reachability.authority-endpoint-3.v1`
4. `prw.reachability.authority-ca-bundle.v1`
5. `prw.reachability.live-owner.client-certificate.v1`
6. `prw.reachability.live-owner.client-private-key.v1`
7. `prw.reachability.fence-allocator.client-certificate.v1`
8. `prw.reachability.fence-allocator.client-private-key.v1`

No caller-supplied path, filename, credential identifier, or fallback source is allowed.

## Custody and filesystem invariants

CE reuses the Phase 122 systemd custody safety model:

- `$CREDENTIALS_DIRECTORY` is the only directory source;
- the directory path must be absolute;
- symlink directories are rejected;
- the directory must be owned by the effective service user;
- group/other write access is rejected;
- owner read/search access is required;
- each credential is pre-inspected with `symlink_metadata`;
- symlink and non-regular credential nodes are rejected;
- files are opened with `O_RDONLY | O_CLOEXEC | O_NOFOLLOW`;
- pre-open and post-open device/inode identity must match;
- credential owner must equal the effective service user;
- group/other write and all execute bits are rejected;
- owner-read permission is required;
- every read is explicitly bounded;
- no hard-coded `/run/credentials` path exists;
- no persistent plaintext fallback exists.

## Selected bounds

The custody boundary uses narrow type-specific upper bounds before control-plane validation:

- each endpoint credential: `2048` bytes;
- authority CA bundle: `262144` bytes;
- each client certificate: `131072` bytes;
- each client private key: `32768` bytes.

All credentials must be non-empty. Endpoint credentials must be valid UTF-8 and are passed byte-for-byte as strings to the existing control-plane HTTPS/FQDN/member-count/uniqueness validator; custody does not trim or normalize them.

## Secret-memory invariant

Private-key plaintext is read directly into `Zeroizing<Vec<u8>>` bounded buffers. Those owners are moved by value into `ReachabilityEtcdClientIdentityMaterial::new_with_zeroizing_private_key(...)` and are never deliberately unwrapped, cloned, formatted, serialized, or copied into ordinary PRW-owned plaintext buffers.

CA and certificate material are non-private identity/trust material and remain ordinary bounded byte vectors.

## Public API boundary

The crate exposes only:

- fixed credential-name constants;
- `SYSTEMD_CREDENTIALS_DIRECTORY_ENV`;
- one bounded non-secret error enum;
- `load_reachability_live_owner_etcd_bootstrap_config_from_systemd_credentials()`.

It does not expose generic credential readers, arbitrary secret APIs, raw private-key accessors, provider clients, or authority execution handles.

## Construction ordering

The loader must:

1. validate `$CREDENTIALS_DIRECTORY`;
2. read the three endpoint credentials;
3. read the authority CA bundle;
4. read the live-owner certificate and zeroizing private key;
5. read the fence-allocator certificate and zeroizing private key;
6. construct both role-scoped `ReachabilityEtcdClientIdentityMaterial` values through the zeroizing constructor;
7. construct and return `ReachabilityLiveOwnerEtcdBootstrapConfig`;
8. perform no provider connection or network I/O.

Any custody, UTF-8, identity-material, or bootstrap-config validation failure fails closed.

## Validation

The final CE gate requires:

- exact CD ancestry;
- final net diff limited to the CE contract, root workspace membership/lock material, and the new custody crate;
- no temporary helper workflow in the final tree;
- canonical Rust validation full pass;
- Android/AD/AE verdicts reported exactly as triggered/skipped/not-triggered;
- tests covering fixed-name happy path and fail-closed custody cases without real credentials or provider network I/O;
- Drive audit/readback and append-only rolling status preservation;
- PR draft/open/unmerged.

## Explicit exclusions

CE does not authorize or materialize:

- real endpoint hostnames or ports;
- real CA/certificate/private-key values;
- `LoadCredential=` / `LoadCredentialEncrypted=` service-unit edits;
- encrypted credential provisioning;
- credential generation or rotation;
- `bootstrap_reachability_live_owner_preparation(...)` invocation;
- etcd connection/auth/RBAC/membership mutation;
- Agent startup/readiness/runtime task wiring;
- authority acquisition/currentness/release activation;
- recovery epoch issuance;
- PRWF initialization;
- R1-R4 activation;
- deployment;
- merge.

## Gate

`C02F_CE_REACHABILITY_CUSTODY_SOURCE_MATERIALIZED`

This gate means only that the dedicated reachability systemd-custody acquisition boundary exists as validated source and can construct the already-selected opaque control-plane bootstrap configuration without activating it.