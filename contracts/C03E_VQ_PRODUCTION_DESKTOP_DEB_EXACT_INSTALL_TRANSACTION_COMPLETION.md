# C03e-VQ — Production Desktop Debian Exact Installation Transaction Completion

Status: `INSTALLED — EXACT_INSTALLED_BYTES_VERIFIED — NO_LAUNCH`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## 1. Exact predecessor and prepared authority

Prepared VQ authority:

- branch: `phase-152-c03e-vq-production-desktop-deb-exact-install-transaction`;
- prepared head: `cdec904d97ac90616ec3a39a9776ad657547124b`;
- prepared tree: `bb8dd2c66d102be5acc56acc42b5acd36f7792b5`;
- PR #707: draft/open/unmerged;
- prepared canonical Drive evidence ID:
  `1MBY5vjJqDQ-oMkfV5-GS4ImS5RSOhiO5`.

Prepared VQ selected exactly:

`/usr/bin/dpkg --install /home/gersi365/.local/state/private-remote-workspace/phase152/c03e-vp/staging/643f9bc942e94dbaad54050461f104117e08c2c3/private-remote-workspace-desktop_0.1.0-1_amd64.deb`

and stopped on local root authentication.

The user authenticated the exact command locally. No sudo password was disclosed to the
assistant.

## 2. Exact installed package state

Read-only post-install verification at `2026-09-19T08:49:31+02:00` proved:

- Package: `private-remote-workspace-desktop`;
- Status: `install ok installed`;
- Version: `0.1.0-1`;
- Architecture: `amd64`.

The package-owned installed paths are exactly:

1. `/usr/lib/private-remote-workspace/prw-desktop`;
2. `/usr/share/applications/io.patchmirror.prw.desktop.desktop`.

## 3. Installed executable exactness

Installed binary:

`/usr/lib/private-remote-workspace/prw-desktop`

Verified:

- regular file;
- non-symlink;
- owner `root:root`, uid/gid `0:0`;
- mode `0755`;
- size `1373600` bytes;
- SHA-256:
  `1adb489772c54b98996845f1ecca1e3a77e47e4cc8f215535f0afb9159fc26af`;
- dpkg owner:
  `private-remote-workspace-desktop`.

The SHA-256 of the packaged binary extracted as a stream from the exact staged .deb was
independently recomputed and matched the installed binary exactly.

## 4. Installed launcher exactness

Installed launcher:

`/usr/share/applications/io.patchmirror.prw.desktop.desktop`

Verified:

- regular file;
- non-symlink;
- owner `root:root`, uid/gid `0:0`;
- mode `0644`;
- size `202` bytes;
- dpkg owner:
  `private-remote-workspace-desktop`;
- installed bytes exactly matched the launcher extracted as a stream from the exact
  staged .deb.

Installed launcher bytes:

`[Desktop Entry]`
`Type=Application`
`Name=Private Remote Workspace`
`Exec=/usr/lib/private-remote-workspace/prw-desktop`
`TryExec=/usr/lib/private-remote-workspace/prw-desktop`
`Terminal=false`
`StartupNotify=true`

No autostart entry was observed.

## 5. Stage integrity after installation

The exact staged package remains:

`/home/gersi365/.local/state/private-remote-workspace/phase152/c03e-vp/staging/643f9bc942e94dbaad54050461f104117e08c2c3/private-remote-workspace-desktop_0.1.0-1_amd64.deb`

Its SHA-256 remained:

`2f29a60187a00101e641499aea68b88abc6ca740bae39d9fc574eb31e5110c71`

and:

`sha256sum --check --strict SHA256SUMS`

returned PASS.

## 6. Agent and managed-state preservation

Post-install verification proved:

- `prw-agent.service` active;
- `ActiveState=active`;
- `SubState=running`;
- `Result=success`;
- `NRestarts=0`;
- manager-visible:
  `PRW_AGENT_EXECUTION_MODE=local_only`;
- managed `40-configured-remote-inputs.conf` absent.

No Agent restart, reload, stop, start, replacement, reconfiguration, credential mutation,
managed-30/40 mutation, configured-remote selection, or network activation occurred.

## 7. No desktop launch

Post-install exact process scan returned:

`DESKTOP_EXACT_EXE_PROCESS_COUNT=0`

Therefore package installation did not launch the desktop application.

No `gtk-launch`, `gio launch`, launcher click, direct binary execution, autostart,
service or timer activation is claimed or authorized in VQ.

## 8. Dpkg installation timestamp evidence

Read-only dpkg info metadata observed at `2026-09-19T08:49:52+02:00`:

- `private-remote-workspace-desktop.list`
  modified at approximately `2026-09-19T08:48:30.090544754+02:00`;
- `private-remote-workspace-desktop.md5sums`
  modified at approximately `2026-09-19T08:48:30.095057637+02:00`.

These filesystem timestamps are supporting observations, not cryptographic installation
proof.

## 9. Completion classification

`PRODUCTION_DESKTOP_DEB_EXACT_INSTALL_COMPLETED / DPKG_STATUS_INSTALL_OK_INSTALLED / VERSION_0_1_0_1_AMD64_VERIFIED / INSTALLED_BINARY_EXACT_HASH_VERIFIED / INSTALLED_LAUNCHER_EXACT_BYTES_VERIFIED / DPKG_OWNERSHIP_VERIFIED / STAGED_PACKAGE_CHECKSUM_STILL_VALID / AGENT_ACTIVE_LOCAL_ONLY_PRESERVED / MANAGED_40_ABSENT / DESKTOP_NOT_LAUNCHED / NO_AGENT_MUTATION / NO_CONFIGURED_REMOTE / NO_NETWORK_MUTATION / NO_RACE_FREE_CLAIM`

## STOP

`STOP_AFTER_EXACT_DESKTOP_PACKAGE_INSTALLATION_AND_POST_INSTALL_VERIFICATION_AND_BEFORE_FIRST_DESKTOP_APPLICATION_LAUNCH`

First desktop application launch is a separate later checkpoint.

`NO_RACE_FREE_CLAIM`
