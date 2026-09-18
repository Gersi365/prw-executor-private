# systemd Packaging Boundary

The Ubuntu PRW Agent is packaged as an unprivileged **systemd user service**. The service source is `prw-agent.service`; no socket-activation unit is part of the initial package contract.

## Locked Phase 105 paths

- packaged executable: `/usr/lib/private-remote-workspace/prw-agent`
- global user unit: `/usr/lib/systemd/user/prw-agent.service`
- repository unit source: `packaging/systemd/prw-agent.service`

The unit runs as the existing user-manager UID. It does not set `User=`, synthesize `XDG_RUNTIME_DIR`, create a working directory, or delegate PRW socket lifecycle to systemd. The Agent retains ownership of XDG validation, instance locking, bind/listen/readiness, and socket cleanup.

## Locked service policy

- `Type=exec`
- `Restart=on-failure`
- `RestartSec=5s`
- `StartLimitIntervalSec=60s`
- `StartLimitBurst=5`
- `TimeoutStopSec=15s`
- `UMask=0077`
- `NoNewPrivileges=yes`
- stdout/stderr captured by the journal
- future enablement relationship: `WantedBy=default.target`

No `ExecStop=`, shell wrapper, `.socket` unit, `Type=notify`, `RuntimeDirectory=`, `Environment=XDG_RUNTIME_DIR=...`, or `network-online.target` dependency is introduced.

## C03e-UV device-identity provisioner package boundary

The existing Rust binary target `prw-device-identity-provision` is packaged as a separate, explicitly invoked administrative executable at:

`/usr/lib/private-remote-workspace/prw-device-identity-provision`

Its permanent create-only package-file transaction is defined by:

- `packaging/systemd/PROVISIONER_INSTALL_TRANSACTION.md`;
- `packaging/systemd/install-prw-device-identity-provision.sh`;
- `scripts/validate-phase-152-c03e-uv-provisioner-packaging.sh`.

The provisioner has no systemd unit, socket unit, enablement relationship, or automatic activation surface. The C03e-UV transaction copies validated executable bytes only and never executes the provisioner. Real-root installation is separately gated and is not performed or authorized by repository validation.

## C03e-UX retained artifact delivery boundary

The provisioner delivery surface is separately defined by:

- `packaging/systemd/PROVISIONER_ARTIFACT_DELIVERY.md`;
- `scripts/prepare-phase-152-c03e-ux-provisioner-artifact.sh`;
- `.github/workflows/phase-152-c03e-ux-provisioner-artifact-delivery.yml`.

The retained Actions artifact contains exactly the provisioner binary, `SHA256SUMS`, and `ARTIFACT-MANIFEST.txt`. The binary checksum is verified before upload, while the upload action separately emits an archive-level SHA-256 artifact digest. The manifest binds the retained bytes to the exact checked-out Git head and retains explicit `NOT_AUTHORIZED` markers for production-host transfer, real-root installation, provisioner execution, and identity provisioning.

This retained artifact is a checksum-bound delivery surface, not a GitHub Release and not a signed-release provenance claim. A later production-install checkpoint must independently download and re-verify the evidence-closed artifact before it may request authorization for real-root installation.

## C03e-VE agent-configuration administrative executable boundary

The existing Rust binary target `prw-agent-configure` is packaged as a separate, explicitly invoked administrative executable at:

`/usr/lib/private-remote-workspace/prw-agent-configure`

Its permanent create-only package-file transaction is defined by:

- `packaging/systemd/AGENT_CONFIGURE_INSTALL_TRANSACTION.md`;
- `packaging/systemd/install-prw-agent-configure.sh`;
- `scripts/validate-phase-152-c03e-ve-agent-configure-packaging.sh`.

C03e-VE copies validated executable bytes only. The installer never executes `prw-agent-configure`, never writes `30-agent-execution-mode.conf` or `40-configured-remote-inputs.conf`, never calls `daemon-reload`, and never starts or restarts the Agent. Real-root installation remains separately gated; any later `prw-agent-configure write` transaction is a distinct production mutation and requires separate authorization.

The configure executable has no systemd unit, socket unit, enablement relationship, or automatic activation surface. C03e-VE does not define retained artifact delivery or release provenance; those may be materialized by a later separately bounded checkpoint if required before production installation.

## Activation gate

This repository source does **not** install, enable, start, restart, or reload the real service and does not mutate user lingering. C03e-UV does not install the provisioner on a real host or perform device-identity provisioning. C03e-UX adds only retained provisioner artifact delivery and likewise does not transfer, stage, install, or execute the provisioner on the production host. C03e-VE adds only a non-activating create-only package path for `prw-agent-configure`; it does not install or execute that administrative binary on the production host and does not materialize managed Agent configuration. Real-host file installation, any managed configuration write, any later systemd manager operation, and any identity transaction remain separately gated.
