# Phase 152 C02f-CC — Reachability Custody Zeroizing Handoff and Credential Schema Selection Staging

Status: `SELECTED / DEDICATED_REACHABILITY_CUSTODY_CRATE / FIXED_SYSTEMD_CREDENTIAL_IDENTITIES / ZEROIZING_PRIVATE_KEY_HANDOFF_BY_VALUE / OPAQUE_CONTROL_PLANE_BOOTSTRAP_CONFIG / NO_REAL_VALUES / NO_LOADER_SOURCE / NO_SERVICE_WIRING / NO_PROVIDER_BOOTSTRAP_INVOCATION / NO_RUNTIME_ACTIVATION / NO_RECOVERY / NO_R1_R4 / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-22  
Repository: `Gersi365/prw-executor-private`

## Exact prerequisite

C02f-CC derives only from canonical C02f-CB:

- branch: `phase-152-c02f-cb-control-plane-role-private-key-zeroization-source-hardening-staging`;
- head: `249f6f72acba652dc678f38e0a8b448a1d67aaa4`;
- tree: `386b269b8d8ecd8b1d34af57c2db19167270b65b`;
- gate: `C02F_CB_CONTROL_PLANE_ROLE_PRIVATE_KEY_ZEROIZATION_HARDENED`.

CB hardened retained control-plane role private-key storage, but it did not materialize the systemd custody reader or an end-to-end zeroizing handoff from custody into that storage.

## Dedicated custody boundary selected

The future Ubuntu reachability custody implementation is a dedicated workspace crate:

`crates/prw-reachability-custody`

It is not an extension of `prw-device-identity-custody` and is not a generic PRW secret store.

Its purpose is only:

1. acquire the fixed reachability bootstrap inputs from systemd service credentials;
2. apply the already-proven Phase 122 file/directory safety model;
3. preserve zeroizing ownership of private-key plaintext;
4. construct one validated opaque `ReachabilityLiveOwnerEtcdBootstrapConfig`;
5. return only that config to the Agent composition root.

The custody crate may depend on `prw-control-plane`, exact `zeroize`, and Linux filesystem/process primitives needed for the Phase 122 security model. It must not depend on `prw-remote-bridge` or own authority lifecycle semantics.

## Zeroizing private-key handoff selected

The existing public identity constructor accepts `impl Into<Vec<u8>>` for the private key. That shape is acceptable for current tests but is not selected as the production custody handoff because a custody-owned `Zeroizing<Vec<u8>>` should not be copied or intentionally unwrapped into an ordinary PRW-owned `Vec<u8>` between the bounded read and retained control-plane storage.

Before the custody loader is materialized, control-plane must expose a narrow constructor path that accepts a `Zeroizing<Vec<u8>>` private key **by value** and stores that zeroizing owner directly.

Conceptually:

```text
systemd credential file
        -> bounded Zeroizing<Vec<u8>> read
        -> ReachabilityEtcdClientIdentityMaterial zeroizing constructor
        -> retained Zeroizing<Vec<u8>> storage
```

No private-key clone, debug/display, serialization, accessor, or ordinary-Vec handoff is selected for production custody.

The existing Vec-based constructor may remain only if it does not become the production custody path. A later source tranche may narrow or supplement it as needed while preserving tests and public compatibility.

## Fixed service-visible credential identities

CC selects fixed, versioned service-visible credential identities. They are non-secret identifiers; this checkpoint contains no credential values.

Exactly these eight logical inputs are selected:

1. `prw.reachability.authority-endpoint-1.v1`
2. `prw.reachability.authority-endpoint-2.v1`
3. `prw.reachability.authority-endpoint-3.v1`
4. `prw.reachability.authority-ca-bundle.v1`
5. `prw.reachability.live-owner.client-certificate.v1`
6. `prw.reachability.live-owner.client-private-key.v1`
7. `prw.reachability.fence-allocator.client-certificate.v1`
8. `prw.reachability.fence-allocator.client-private-key.v1`

The three endpoints are separate credentials so the custody boundary does not need a caller-controlled list, delimiter grammar, discovery mechanism, or arbitrary file naming surface.

## Payload semantics

Endpoint credentials:

- contain one exact UTF-8 endpoint string each;
- must not contain NUL, CR, LF, or leading/trailing ASCII whitespace;
- are passed to the existing control-plane endpoint validator without hostname rewriting or discovery;
- therefore still require HTTPS, stable FQDNs, exact member count, and unique member hostnames.

Authority CA bundle and both client certificates:

- are treated as exact opaque PEM bytes;
- are not trimmed, normalized, logged, or reserialized by custody;
- remain subject to provider/TLS validation downstream.

Both private-key credentials:

- are exact opaque PEM bytes;
- are read into bounded zeroizing buffers;
- remain zeroizing through custody-to-control-plane handoff;
- are never logged, formatted, serialized, exposed by accessor, or copied into a second PRW-owned plaintext buffer.

Numeric byte bounds remain a source-materialization constant set. They must be finite, tested, and purpose-specific; runtime callers cannot override them.

## Phase 122 security model reused

The custody source tranche must reuse these semantics:

- `$CREDENTIALS_DIRECTORY` only;
- absolute credential-directory requirement;
- no hardcoded `/run/credentials/...` path;
- no persistent plaintext fallback;
- fixed filenames only;
- symlink and non-regular-file rejection;
- `O_RDONLY | O_CLOEXEC | O_NOFOLLOW` open;
- pre-open/post-open same-device-and-inode check;
- effective-user ownership validation;
- insecure permission rejection;
- bounded reads;
- bounded non-secret error classifications;
- no caller-supplied path, filename, secret identifier, or fallback source.

## Selected public handoff

The future custody public surface should be one narrow operation conceptually equivalent to:

```text
load_reachability_etcd_bootstrap_config_from_systemd_credentials()
    -> Result<ReachabilityLiveOwnerEtcdBootstrapConfig, BoundedCustodyError>
```

The Agent receives only the opaque validated config. It does not receive raw endpoint arrays, CA bytes, certificate bytes, private-key bytes, file paths, or generic credential handles through the custody public API.

## Layering

```text
systemd service credentials
        |
        v
prw-reachability-custody
        |
        | zeroizing private-key handoff
        v
prw-control-plane::ReachabilityLiveOwnerEtcdBootstrapConfig
        |
        v
prw-agent composition root
        |
        | later provider-bootstrap invocation
        v
ReachabilityLiveOwnerAcquisitionPreparation
        |
        v
BZ authority composition seam
```

No dependency cycle is selected: custody may depend on control-plane; control-plane does not depend on custody.

## Next source ordering

The next source tranche must remain non-activating and should first materialize the control-plane zeroizing private-key constructor/handoff seam.

Only after that exact-head source is validated may a later tranche materialize `prw-reachability-custody` and its fixed systemd credential reader.

Service-unit credential provisioning and Agent provider-bootstrap invocation remain separate later gates.

## Explicit exclusions

CC does not authorize or materialize:

- any real endpoint hostname or port;
- any real CA, client certificate, or private key;
- encrypted credential blob paths or provisioning commands;
- systemd unit/drop-in edits;
- custody source code;
- provider network I/O;
- `bootstrap_reachability_live_owner_preparation(...)` invocation;
- etcd auth/RBAC/membership mutation;
- Agent startup/readiness/runtime-task wiring;
- authority acquisition/currentness/release activation;
- recovery epoch issuance;
- PRWF initialization;
- R1-R4 activation;
- deployment;
- merge.

## Gate

`C02F_CC_REACHABILITY_CUSTODY_ZEROIZING_HANDOFF_AND_CREDENTIAL_SCHEMA_SELECTED`

This gate selects only the custody ownership, fixed credential identities, payload semantics, and zeroizing handoff requirement. It does not mean custody or reachability authority runtime is active.