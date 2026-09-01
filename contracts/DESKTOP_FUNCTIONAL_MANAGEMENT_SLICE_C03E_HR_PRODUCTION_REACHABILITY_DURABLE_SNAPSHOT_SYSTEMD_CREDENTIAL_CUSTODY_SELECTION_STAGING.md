# Private Remote Workspace — C03e-HR Production Reachability Durable Snapshot Systemd Credential Custody Selection Staging

Status: `STAGED_SELECTION_ONLY — DOCS_ONLY — NO_CREDENTIAL_PROVISIONING — NO_SYSTEMD_WIRING — NO_RUNTIME_AUTHORIZATION`

Gate target:

```text
C03E_HR_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_SYSTEMD_CREDENTIAL_CUSTODY_BOUNDARY_SELECTED
```

Canonical closure target:

```text
CLOSED_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_SYSTEMD_CREDENTIAL_CUSTODY_SELECTION
```

## 1. Purpose

C03e-HR performs the exact-source selection required after closed C03e-HQ before dedicated durable-snapshot client certificate/private-key material may be acquired through the existing Linux systemd service-credential custody boundary.

C03e-HQ already materialized the control-plane production provider bootstrap:

```text
ReachabilityProductionEtcdBootstrapConfig
 -> bootstrap_reachability_production_preparation(...)
 -> ReachabilityProductionEtcdBootstrapPreparation
 -> (
      ReachabilityLiveOwnerAcquisitionPreparation,
      ReachabilityDurableSnapshotEtcdExecutor,
    )
```

The remaining custody question is how `prw-reachability-custody` may construct that production config from a fixed systemd credential set while preserving the current eight-credential/two-role loader unchanged, preserving private-key zeroizing ownership, preventing identity fallback/reuse, and avoiding any systemd unit/runtime activation in the same checkpoint.

HR selects only that custody source shape. It does not read live production credentials, provision credentials, issue certificates, generate private keys, mutate etcd auth/RBAC, change systemd units, invoke provider bootstrap, activate Agent runtime behavior, deploy, restart services, merge, or mutate production state.

## 2. Exact predecessor guard

Canonical predecessor: C03e-HQ.

Exact predecessor branch:

```text
phase-152-c03e-hq-production-reachability-durable-snapshot-control-plane-bootstrap-source-materialization
```

Exact predecessor head:

```text
c5d68c0f10bb28a91570ecd08b734a55e32a24ef
```

Exact predecessor tree:

```text
247bcd73612c203251b65d0e84cea616e8c4964a
```

Exact HQ control-plane bootstrap blob:

```text
8780848d1ae40e850c459a6ab4ce92a5451e4dae
```

HQ gate:

```text
C03E_HQ_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_CONTROL_PLANE_BOOTSTRAP_SOURCE_MATERIALIZED
```

HQ closure:

```text
CLOSED_PRODUCTION_REACHABILITY_DURABLE_SNAPSHOT_CONTROL_PLANE_BOOTSTRAP_SOURCE_MATERIALIZATION
```

HQ immutable Drive audit ID:

```text
1vpWVeoAMjgM3zt_MgPnwdBk1hiUFY6Qc
```

HR must remain a docs-only successor of that exact closed source head.

## 3. Frozen architecture

The production ownership law remains:

```text
prw-reachability-custody: bounded systemd credential acquisition and validation
prw-control-plane: provider connection/TLS/client bootstrap
prw-remote-bridge: reachability semantic authority/store
prw-agent: cross-crate composition and owner custody
```

The custody crate may construct opaque validated control-plane config values, but it must not connect to etcd or return raw secret material.

The selected durable provider identity remains distinct from live-owner and fence-allocator identities.

## 4. Exact current-source evidence — custody is eight-credential/two-role

Exact path at closed HQ:

```text
crates/prw-reachability-custody/src/lib.rs
```

Exact blob:

```text
cc3dcb80344fc62af31db25a9d93469392d29103
```

The current source imports and produces only:

```text
ReachabilityLiveOwnerEtcdBootstrapConfig
```

Its fixed public credential-name constants are exactly:

```text
prw.reachability.authority-endpoint-1.v1
prw.reachability.authority-endpoint-2.v1
prw.reachability.authority-endpoint-3.v1
prw.reachability.authority-ca-bundle.v1
prw.reachability.live-owner.client-certificate.v1
prw.reachability.live-owner.client-private-key.v1
prw.reachability.fence-allocator.client-certificate.v1
prw.reachability.fence-allocator.client-private-key.v1
```

The current public loader is:

```text
load_reachability_live_owner_etcd_bootstrap_config_from_systemd_credentials()
 -> Result<ReachabilityLiveOwnerEtcdBootstrapConfig, ReachabilityCustodyError>
```

It reads only the eight fixed names relative to `$CREDENTIALS_DIRECTORY`.

The existing loader performs no network I/O and must remain source-compatible and behaviorally unchanged.

## 5. Existing custody safety law remains mandatory

The custody source already enforces:

- Linux-only systemd credential-directory acquisition;
- absolute credential-directory boundary;
- no symlink directory/file acceptance;
- stable regular-file validation before and after open;
- effective-service-user ownership checks;
- locked credential-directory and credential-file permission checks;
- `O_NOFOLLOW` and `O_CLOEXEC` opens;
- bounded type-specific reads;
- endpoint UTF-8 validation without normalization;
- private-key ownership in `Zeroizing<Vec<u8>>`;
- direct move into `ReachabilityEtcdClientIdentityMaterial::new_with_zeroizing_private_key(...)`;
- no public accessor returning private-key bytes;
- no provider connection or runtime operation.

The future durable loader must reuse the same law exactly rather than inventing a parallel weaker reader.

## 6. Exact current-source evidence — control-plane production config now exists

Exact path:

```text
crates/prw-control-plane/src/reachability_acquisition_evidence/bootstrap.rs
```

Exact blob:

```text
8780848d1ae40e850c459a6ab4ce92a5451e4dae
```

The HQ-materialized type is:

```text
ReachabilityProductionEtcdBootstrapConfig
```

Its constructor consumes:

```text
endpoints
trust_bundle_pem
live_owner_identity
fence_allocator_identity
durable_snapshot_identity
```

and reuses the existing bounded endpoint/trust validation plus exact pairwise certificate/private-key reuse rejection across all three roles.

This type is the selected custody output. HR does not select another aggregate config or raw secret carrier.

## 7. Exact current-source evidence — no Cargo change is required

Exact custody manifest:

```text
crates/prw-reachability-custody/Cargo.toml
```

Exact blob:

```text
f8ff3ecdac1e5cb0b580818ad6f55ae6076ff4f7
```

The custody crate already depends on:

```text
prw-control-plane
zeroize
rustix (Linux fs/process features)
```

Therefore the first custody source materialization requires no Cargo manifest or lockfile change.

## 8. Exact current-source evidence — Agent custody bootstrap is still two-role

Exact path:

```text
crates/prw-agent/src/reachability_authority_custody_bootstrap.rs
```

Exact blob:

```text
2843cbf9cfed7ae26e336ec4a2ead6a97855b2c0
```

It calls only:

```text
load_reachability_live_owner_etcd_bootstrap_config_from_systemd_credentials()
 -> bootstrap_reachability_live_owner_authority(config)
```

It is not wired into Agent startup/readiness.

HR does not select modification of this Agent file in the first custody source materialization. The production durable loader will remain dormant until a later Agent composition checkpoint explicitly selects a caller.

## 9. Exact current-source evidence — durable owner consumer remains separate

Exact path:

```text
crates/prw-agent/src/production_reachability_owner_composition.rs
```

Exact blob:

```text
6a338b43995ecc069383e8aee63d7b53a35bc6ff
```

It consumes only an already-narrowed:

```text
ReachabilityDurableSnapshotEtcdExecutor
```

and performs the existing bridge-store plus Agent-custody recovery chain.

It does not read credentials or connect to etcd.

HR keeps this separation unchanged.

## 10. Selected dedicated durable systemd credential names

C03e-HR selects exactly two new fixed custody credential names:

```text
DURABLE_SNAPSHOT_CLIENT_CERTIFICATE_CREDENTIAL_NAME
 = "prw.reachability.durable-snapshot.client-certificate.v1"

DURABLE_SNAPSHOT_CLIENT_PRIVATE_KEY_CREDENTIAL_NAME
 = "prw.reachability.durable-snapshot.client-private-key.v1"
```

These names are role-specific and must not alias any existing live-owner or fence-allocator credential name.

No username/password credential, token credential, endpoint override, alternate CA bundle, or fallback credential name is selected.

The durable role continues to use the same validated authority endpoint set and the same explicit authority CA bundle; only its client identity is distinct.

## 11. Selected additive public custody loader

C03e-HR selects one additive public function in `prw-reachability-custody`:

```text
load_reachability_production_etcd_bootstrap_config_from_systemd_credentials()
 -> Result<ReachabilityProductionEtcdBootstrapConfig, ReachabilityCustodyError>
```

The existing public loader remains unchanged:

```text
load_reachability_live_owner_etcd_bootstrap_config_from_systemd_credentials()
```

No existing caller is redirected to the new function.

## 12. Selected ten-credential production custody set

The new production loader reads exactly these ten fixed names:

```text
3 authority endpoint credentials
1 authority CA bundle credential
2 live-owner identity credentials
2 fence-allocator identity credentials
2 durable-snapshot identity credentials
```

The resulting logical construction is:

```text
endpoint-1
endpoint-2
endpoint-3
shared authority CA bundle
live-owner certificate + zeroizing private key
fence-allocator certificate + zeroizing private key
durable-snapshot certificate + zeroizing private key
 -> ReachabilityProductionEtcdBootstrapConfig::new(...)
```

No credential scan, directory enumeration, dynamic name discovery, environment fallback, or optional durable identity is selected.

## 13. Selected durable private-key custody law

The durable private key must follow the existing private-key path exactly:

```text
open_validated_credential(...)
 -> read_bounded_zeroizing(...)
 -> Zeroizing<Vec<u8>>
 -> ReachabilityEtcdClientIdentityMaterial::new_with_zeroizing_private_key(...)
```

The durable key must never be moved through an ordinary non-zeroizing PRW-owned plaintext buffer for convenience.

No clone, debug formatting, logging, public accessor, serialization, or persistence of private-key bytes is selected.

## 14. Selected durable certificate custody law

The durable client certificate is non-secret identity material but remains bounded and role-scoped.

It must use the existing certificate size bound and validated credential-file open path.

No certificate parsing or trust decision is moved into custody; control-plane/rustls remains responsible for provider TLS parsing during bootstrap.

## 15. Selected control-plane validation handoff

After all ten fixed credentials are read and the three role identities are constructed, custody must call exactly:

```text
ReachabilityProductionEtcdBootstrapConfig::new(...)
```

This preserves:

- exact three-endpoint topology validation;
- explicit non-empty trust bundle;
- live/fence separation;
- live/durable separation;
- fence/durable separation.

A pairwise credential collision must fail through the existing:

```text
ReachabilityCustodyError::BootstrapConfig(
    ReachabilityLiveOwnerEtcdBootstrapConfigError::{...}
)
```

No new fallback path is selected after such a failure.

## 16. Existing error surface remains sufficient

The first custody source materialization does not require a new public error enum.

Existing `ReachabilityCustodyError` classes remain sufficient for:

- missing/unavailable durable credential;
- non-regular durable credential;
- ownership mismatch;
- insecure permissions;
- bounded read failure;
- size violation;
- identity material validation failure;
- production bootstrap configuration validation failure.

The user-visible `Display` surface must continue to avoid secret/path/provider detail leakage.

## 17. Additive implementation law

The source successor must preserve the current eight-credential loader behavior and API.

Selected implementation strategy:

- add the two durable credential-name constants;
- add the production public loader;
- add Linux production environment/directory loader helpers;
- reuse existing secure reader functions and size bounds;
- construct the three identities by value;
- call `ReachabilityProductionEtcdBootstrapConfig::new(...)`;
- add bounded tests in the same file.

The successor should avoid refactoring the existing two-role loader merely to reduce duplication. Preserving the closed path is preferred over widening the diff.

## 18. Selected source-materialization ceiling

The first successor source materialization selected by HR is restricted to exactly:

```text
crates/prw-reachability-custody/src/lib.rs
```

No other path is selected for that first materialization.

Specifically not selected in the same checkpoint:

- `crates/prw-reachability-custody/Cargo.toml`;
- Cargo lockfile;
- `crates/prw-agent/src/reachability_authority_custody_bootstrap.rs`;
- `crates/prw-agent/src/reachability_authority_bootstrap.rs`;
- `crates/prw-agent/src/production_reachability_owner_composition.rs`;
- control-plane bootstrap source;
- systemd service/unit/package files;
- deployment manifests;
- workflows.

If compilation mechanically requires another source path, the successor must stop and use a separate corrective/extension checkpoint rather than broaden scope silently.

## 19. Selected bounded tests

The source successor must add tests in the same custody file sufficient to prove at least:

1. the two new durable credential names are exact, fixed, and distinct from existing role names;
2. a complete ten-credential fixture constructs `ReachabilityProductionEtcdBootstrapConfig` without network I/O;
3. a missing durable certificate or private key fails closed as `CredentialUnavailable`;
4. an oversized durable private key fails before control-plane construction;
5. durable private-key reuse with live-owner or fence-allocator reaches the existing production config reuse rejection;
6. existing eight-credential/two-role tests remain unchanged and passing.

No test may read production credentials or connect to a production endpoint.

## 20. Systemd unit wiring remains separately gated

HR selects custody source names only. It does not add or modify any systemd `LoadCredential=`/`SetCredential=`/unit/package declaration.

Therefore the running service remains unable to receive the new durable credentials through its current unit configuration.

A later checkpoint must explicitly select any unit/package wiring before production custody could supply the new files.

No fallback to existing live-owner or fence-allocator credentials is permitted while that wiring is absent.

## 21. Credential provisioning remains separately gated

HR does not authorize or perform:

- creation of `prw-reachability-durable-snapshot` credentials;
- private-key generation;
- certificate signing/issuance;
- certificate installation;
- secret-manager writes;
- systemd credential file installation;
- rotation;
- revocation.

The already-selected logical durable principal/role remains conceptual evidence only until separately provisioned.

## 22. etcd auth/RBAC remains separately gated

The frozen intended boundary remains:

```text
principal: prw-reachability-durable-snapshot
role: prw-reachability-durable-snapshot-rw
prefix: /prw/reachability/durable-snapshot/
```

HR does not create, mutate, bind, or inspect production etcd auth/RBAC state.

## 23. Provider network I/O remains absent

The selected custody loader only constructs an opaque validated control-plane config.

It does not call:

```text
bootstrap_reachability_production_preparation(...)
Client::connect(...)
KvClient operations
```

Provider network I/O remains owned by control-plane and requires a later explicit caller.

## 24. Agent composition remains separately gated

HR does not select a new Agent-level function that joins:

```text
systemd production custody loader
 -> control-plane three-role provider bootstrap
 -> live-owner authority composition
 -> durable owner custody recovery
```

That cross-crate join remains a later checkpoint after custody source materialization is closed.

## 25. Runtime activation remains separately gated

Not selected:

- `main.rs` callsite;
- Linux startup callsite;
- readiness publication;
- shutdown ownership;
- listener installation;
- candidate publication;
- traversal provisioning;
- peer dialing;
- task/runtime spawn;
- service restart;
- deployment.

## 26. Durable protocol invariants remain unchanged

No custody selection changes the durable semantic protocol:

- exact durable-snapshot key/value binding;
- default-linearizable exact Get;
- exact dual CAS on `mod_revision` plus observed bytes;
- exact replacement Put;
- no create-if-absent recovery;
- no prefix scan;
- no Watch authority path;
- no lease/TTL authority;
- no blind retry;
- no in-memory fallback.

Logical device identity remains independent of fixed IP. Dynamic IP remains transient reachability only. Request IDs remain correlation only.

## 27. Validation obligations for the source successor

The source successor must obtain exact-head evidence for:

```text
cargo fmt --all -- --check
cargo clippy workspace validation
workspace tests
workspace build
```

Android validation, if triggered by the source diff, must also complete successfully before closure.

No failed or pending required exact-head check may be ignored in favor of a historical SHA.

## 28. Explicit non-authorization

C03e-HR does not authorize or perform:

- Rust source materialization;
- Cargo/dependency changes;
- production credential reads;
- credential generation/provisioning/rotation;
- certificate issuance/installation;
- etcd auth/RBAC mutation;
- systemd unit/package changes;
- provider network connections;
- Agent/runtime/startup wiring;
- candidate/traversal/listener activation;
- service restart;
- deployment;
- production-state mutation;
- merge;
- branch deletion;
- repository visibility mutation.

## 29. Closure law

C03e-HR may close only when:

1. it remains a docs-only successor of exact closed HQ head `c5d68c0f10bb28a91570ecd08b734a55e32a24ef`;
2. the diff contains only this staging contract;
3. required exact-head CI is complete without failure;
4. an immutable Drive audit is written and verified;
5. PR state remains draft/open/unmerged;
6. no credential/security/systemd/runtime/deployment action occurred.

On closure, the next admissible source checkpoint is restricted initially to:

```text
crates/prw-reachability-custody/src/lib.rs
```

and may materialize only the additive durable systemd credential custody boundary selected above.
