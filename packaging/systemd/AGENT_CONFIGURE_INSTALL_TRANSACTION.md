# PRW Agent Configure Package File Transaction

Status: `C03E_VE_NON_ACTIVATING_CREATE_ONLY_AGENT_CONFIGURE_DEPLOYMENT_CONTRACT`

This contract materializes a permanent packaging/deployment path for the existing Rust binary target `prw-agent-configure`. It does not authorize or perform production installation, managed configuration writes, systemd manager reload, Agent activation, enrollment, or networking mutation.

## Source and build identity

Rust package:

`prw-agent-systemd-orchestration`

Binary source:

`crates/prw-agent-systemd-orchestration/src/main.rs`

Locked build command:

```text
cargo build --locked -p prw-agent-systemd-orchestration --bin prw-agent-configure
```

Expected build artifact basename:

`prw-agent-configure`

The packaging transaction copies this executable as bytes. It must never execute `prw-agent-configure`.

The executable itself exposes the fixed administrative actions `write` and `reconfigure-active`; those runtime actions remain outside this package-file transaction and require later separately authorized checkpoints.

## Locked destination

`/usr/lib/private-remote-workspace/prw-agent-configure`

Expected installed metadata for a future separately authorized production file transaction:

- regular file, never a symlink;
- mode `0755`;
- root-owned when installed into the real root filesystem by uid 0;
- exact bytes equal to the validated build artifact.

No systemd unit, socket unit, enablement relationship, environment drop-in, credential binding, or activation link is created for `prw-agent-configure`. It is an explicitly invoked administrative package artifact, not an automatically activated service.

## Permanent installer source

`packaging/systemd/install-prw-agent-configure.sh`

Interface:

```text
bash packaging/systemd/install-prw-agent-configure.sh \
  --root <filesystem-root> \
  --artifact <path-to-prw-agent-configure>
```

`ROOT=/` is refused unless `--allow-root-filesystem` is supplied explicitly, and real-root mode additionally requires uid 0. C03e-VE does not invoke real-root mode.

## Create-only semantics

This checkpoint deliberately defines only first file materialization because C03e-VD freshly proved that the production `prw-agent-configure` artifact was absent from the checked production package destinations.

The installer must:

1. require an existing absolute non-symlink root directory;
2. require a regular non-symlink artifact whose basename is exactly `prw-agent-configure`;
3. validate or create the locked `/usr/lib/private-remote-workspace` directory chain without traversing an existing symlink component;
4. fail if the destination already exists in any form, including a symlink;
5. copy the artifact to a private same-directory temporary file;
6. set the staged mode to `0755`;
7. verify staged bytes against the source artifact;
8. publish with an atomic same-filesystem hard-link operation that cannot replace an existing destination;
9. remove the temporary name;
10. verify final regular-file type, non-symlink status, mode and bytes;
11. for separately authorized real-root execution only, verify final uid/gid `0:0`.

Upgrade, replacement, rollback of a pre-existing `prw-agent-configure`, and removal are intentionally outside C03e-VE. A future checkpoint must define those semantics before replacing or deleting a deployed configure executable.

## Hard prohibitions

This transaction must not:

- execute `prw-agent-configure` with `write`, `reconfigure-active`, or any other argument;
- create, replace or remove `30-agent-execution-mode.conf`;
- create, replace or remove `40-configured-remote-inputs.conf`;
- create, replace or remove `20-device-identity-credential.conf`;
- set `PRW_AGENT_EXECUTION_MODE` for the user manager or Agent;
- call `systemctl`, `loginctl`, or `systemd-analyze`;
- run `daemon-reload`;
- start, restart, stop, reset, enable or disable `prw-agent.service`;
- mutate linger or the user-manager environment;
- decrypt, copy, export or regenerate device-identity credential material;
- invoke the device-identity provisioner;
- create a systemd unit, socket unit, or activation link for `prw-agent-configure`;
- mutate enrollment, revocation, networking, DNS, forwarding, or relay state;
- write into the intended user's home directory.

## Disposable validation

Permanent validation source:

`scripts/validate-phase-152-c03e-ve-agent-configure-packaging.sh`

The validation must run only under a disposable filesystem root and prove:

- the exact Rust binary target builds under the locked dependency graph;
- the exact destination is materialized with mode `0755`;
- installed bytes and SHA-256 equal the built artifact;
- no other file or symlink payload is created in the disposable root;
- a second create-only invocation fails without changing bytes or mode;
- a destination symlink is rejected without altering its target;
- a symlink in the locked parent chain is rejected without out-of-root publication;
- the permanent installer source contains none of the forbidden manager/configuration mutation command surfaces checked by validation;
- the real-root opt-in and uid-0 guards remain present.

The validation builds `prw-agent-configure` but never executes the built configure artifact. Disposable validation proves packaging mechanics only. It does not prove current production-host state and does not authorize a production install or configuration write.

## Success boundary

C03e-VE may be considered implementation-complete only after exact-head CI proves the permanent installer and disposable validation path and immutable evidence is recorded.

Even after that closure:

`NO_PRODUCTION_AGENT_CONFIGURE_INSTALL / NO_MANAGED_CONFIGURATION_WRITE / NO_DAEMON_RELOAD / NO_AGENT_ACTIVATION`

A later separately authorized checkpoint must freshly re-prove production host state before any real `prw-agent-configure` file installation. A still-later checkpoint must separately authorize any `prw-agent-configure write` execution.
