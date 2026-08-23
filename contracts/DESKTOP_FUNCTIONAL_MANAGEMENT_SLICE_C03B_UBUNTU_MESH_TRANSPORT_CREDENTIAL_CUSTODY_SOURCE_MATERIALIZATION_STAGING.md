# Phase 152 C03b — Ubuntu Mesh Transport Credential Custody Source Materialization Staging

Status: `SOURCE_MATERIALIZED / FIXED_SYSTEMD_CREDENTIALS / PRIVATE_KEY_ZEROIZING / NO_RAW_KEY_ACCESSOR / NO_NETWORK_IO / NO_TLS_CONSTRUCTION / NO_ENDPOINT / NO_REMOTE_READY / NO_SYSTEMD_MUTATION / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-23  
Repository: `Gersi365/prw-executor-private`

## Purpose

C03b materializes the Linux/systemd custody boundary for the Ubuntu Agent mesh transport identity selected by C03a.

It loads exactly one private trust-root certificate, one Agent mesh leaf certificate, and one Agent PKCS#8 private key from fixed systemd service-credential names. It does not parse certificates, construct rustls/QUIC state, bind sockets, publish remote readiness, provision credentials, or modify systemd.

## Exact prerequisite

C03b derives only from closed C03a:

- branch: `phase-152-c03a-real-remote-network-runtime-architecture-selection-staging`;
- head: `12d8ffa3f1f43f6ea738dce75230dc138c67de1d`;
- tree: `7c76fcc51314b41bb3bd1ba13071bea568801f30`;
- gate: `C03A_REAL_REMOTE_NETWORK_RUNTIME_ARCHITECTURE_SELECTED`.

## Ownership

The source is placed under `prw-reachability-custody` because that crate already owns the hardened Linux systemd credential-acquisition boundary and already carries the required `rustix + zeroize` primitives.

This does not transfer QUIC/TLS mechanics into the custody crate. `prw-remote-transport` remains the transport owner and the Agent remains the process-level remote-runtime owner.

## Fixed credential names

The loader accepts only:

- `prw.mesh.private-root-certificate.v1`;
- `prw.mesh.agent-certificate.v1`;
- `prw.mesh.agent-private-key.v1`.

The directory is obtained only from `$CREDENTIALS_DIRECTORY`.

No caller-selected file path, home-directory fallback, environment-carried secret value, generated replacement identity, or plaintext persistent-key fallback is allowed.

## Filesystem custody invariants

The implementation reuses the established Phase 122 / C02f-CE Linux custody model:

- credential directory must be absolute;
- directory must be a real directory, not a symlink;
- directory must be owned by the effective service UID;
- directory must reject group/other write bits;
- directory must preserve owner read/search access;
- each credential path is inspected with `symlink_metadata` before open;
- symlinks and non-regular files are rejected;
- each file must be owned by the effective service UID;
- group/other write and execute bits are rejected;
- owner read permission is required;
- files are opened with `RDONLY | CLOEXEC | NOFOLLOW`;
- pre-open and opened `(dev, ino)` identity must match;
- type-specific maximum sizes are checked before and after open;
- reads are bounded by `max + 1` and empty inputs are rejected.

## Material bounds

- private root certificate DER: maximum 65,536 bytes;
- Agent leaf certificate DER: maximum 65,536 bytes;
- Agent PKCS#8 private key DER: maximum 32,768 bytes.

C03b intentionally does not parse or cryptographically validate the DER. The downstream C03c/C03e TLS composition boundary owns that validation.

## Private-key handling

The private key is read directly into `Zeroizing<Vec<u8>>` and retained there for the lifetime of `MeshTransportCredentialMaterial`.

The public material API exposes:

- immutable root-certificate bytes;
- immutable leaf-certificate bytes;
- private-key length only.

It exposes no public private-key byte accessor and its `Debug` representation emits `<redacted>` rather than key bytes.

A later, separately gated composition tranche may add the narrow consuming handoff required by rustls without weakening this custody contract.

## Failure semantics

All failures are explicit and fail closed:

- unsupported platform;
- missing credential directory;
- invalid credential directory;
- insecure credential directory;
- unavailable credential;
- non-regular/symlink credential;
- ownership mismatch;
- insecure credential permissions;
- bounded read failure;
- empty or oversized credential.

There is no degraded-success path.

## Tests

The source-level tests validate at minimum:

- missing/relative credential directory rejection;
- successful fixed three-credential loading without network I/O;
- private-key Debug redaction;
- symlink rejection;
- insecure private-key permission rejection;
- oversized private-key rejection.

The tests use isolated temporary directories and do not require real systemd credentials or network access.

## Permanent scope

The C03b net diff is limited to:

1. this contract;
2. `crates/prw-reachability-custody/src/mesh_transport_custody.rs`;
3. one module declaration in `crates/prw-reachability-custody/src/lib.rs`.

No Cargo manifest, Cargo.lock, workflow, Agent `main.rs`, authority semantics, provider source, QUIC transport source, Android source, systemd unit, recovery, PRWF, R1-R4, production credential, or deployment mutation is authorized.

## Gate

After exact-head canonical CI and evidence closeout:

`C03B_UBUNTU_MESH_TRANSPORT_CREDENTIAL_CUSTODY_SOURCE_MATERIALIZED`
