# PRW Device-Identity Provisioner Package File Transaction

Status: `C03E_UV_NON_ACTIVATING_CREATE_ONLY_PROVISIONER_DEPLOYMENT_CONTRACT`

This contract materializes a permanent packaging/deployment path for the existing Rust binary target `prw-device-identity-provision`. It does not authorize or perform production installation, identity provisioning, service activation, or systemd manager mutation.

## Source and build identity

Rust package:

`prw-device-identity-provisioning`

Binary source:

`crates/prw-device-identity-provisioning/src/bin/prw-device-identity-provision.rs`

Locked build command:

```text
cargo build --locked -p prw-device-identity-provisioning --bin prw-device-identity-provision
```

Expected build artifact basename:

`prw-device-identity-provision`

The packaging transaction copies this executable as bytes. It must never execute the provisioner.

## Locked destination

`/usr/lib/private-remote-workspace/prw-device-identity-provision`

Expected installed metadata for a future separately authorized production file transaction:

- regular file, never a symlink;
- mode `0755`;
- root-owned when installed into the real root filesystem by uid 0;
- exact bytes equal to the validated build artifact.

No systemd unit is created for the provisioner. It is an explicitly invoked administrative package artifact, not an automatically activated service.

## Permanent installer source

`packaging/systemd/install-prw-device-identity-provision.sh`

Interface:

```text
bash packaging/systemd/install-prw-device-identity-provision.sh \
  --root <filesystem-root> \
  --artifact <path-to-prw-device-identity-provision>
```

`ROOT=/` is refused unless `--allow-root-filesystem` is supplied explicitly, and real-root mode additionally requires uid 0. C03e-UV does not invoke that mode.

## Create-only semantics

This checkpoint deliberately defines only first file materialization because the production host proved the provisioner destination absent.

The installer must:

1. require an existing absolute non-symlink root directory;
2. require a regular non-symlink artifact whose basename is exactly `prw-device-identity-provision`;
3. validate or create the locked `/usr/lib/private-remote-workspace` directory chain without traversing an existing symlink component;
4. fail if the destination already exists in any form, including a symlink;
5. copy the artifact to a private same-directory temporary file;
6. set the staged mode to `0755`;
7. verify staged bytes against the source artifact;
8. publish with an atomic same-filesystem hard-link operation that cannot replace an existing destination;
9. remove the temporary name;
10. verify final regular-file type, non-symlink status, mode and bytes;
11. for separately authorized real-root execution only, verify final uid/gid `0:0`.

Upgrade, replacement, rollback of a pre-existing provisioner, and removal are intentionally outside C03e-UV. A future checkpoint must define those semantics before replacing or deleting a deployed provisioner.

## Hard prohibitions

This transaction must not:

- execute `prw-device-identity-provision`;
- generate a P-256 identity key;
- call production `systemd-creds`;
- create, replace or delete the encrypted device-identity credential;
- create, replace or delete PRW `20/30/40` drop-ins;
- call `systemctl` or `loginctl`;
- run `daemon-reload`;
- start, restart, stop, reset, enable or disable `prw-agent.service`;
- mutate linger or the user-manager environment;
- create a systemd unit or activation link for the provisioner;
- mutate enrollment or networking;
- write into the user's home directory.

## Disposable validation

Permanent validation source:

`scripts/validate-phase-152-c03e-uv-provisioner-packaging.sh`

The validation must run only under a disposable filesystem root and prove:

- the exact Rust binary target builds under the locked dependency graph;
- the exact destination is materialized with mode `0755`;
- installed bytes and SHA-256 equal the built artifact;
- no other file or symlink payload is created in the disposable root;
- a second create-only invocation fails without changing bytes or mode;
- a destination symlink is rejected without altering its target;
- a symlink in the locked parent chain is rejected without out-of-root publication;
- the permanent installer source contains none of the forbidden service/identity mutation command surfaces checked by validation;
- the real-root opt-in and uid-0 guards remain present.

Disposable validation proves packaging mechanics only. It does not prove current production-host state and does not authorize a production install.

## Success boundary

C03e-UV may be considered complete only after exact-head CI proves the permanent installer and disposable validation path and immutable evidence is recorded.

Even after that closure:

`NO_PRODUCTION_PROVISIONER_INSTALL / NO_IDENTITY_PROVISIONING / NO_SYSTEMD_MANAGER_MUTATION`

A later separately authorized checkpoint must freshly re-prove production host state before any real provisioner file installation.
