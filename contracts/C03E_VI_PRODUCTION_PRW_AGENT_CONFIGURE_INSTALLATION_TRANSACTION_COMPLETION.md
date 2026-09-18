# C03e-VI Production prw-agent-configure Installation Transaction Completion

Status: `PRODUCTION_ARTIFACT_TRANSFERRED_STAGED_AND_INSTALLED_CREATE_ONLY — POST_INSTALL_BYTES_MODE_OWNERSHIP_VERIFIED — VALIDATED — EVIDENCE_PENDING — PRW_AGENT_CONFIGURE_NOT_EXECUTED — NO_MANAGED_CONFIGURATION_WRITE — NO_DAEMON_RELOAD — AGENT_NOT_ACTIVATED`

Boundary: `PRW_AGENT_CONFIGURE_PRODUCTION_TRANSFER_STAGING_CREATE_ONLY_INSTALL_TRANSACTION`

Authoritative predecessor: evidence-closed C03e-VH / PR #698 at exact head `f1940b5611b9667099e2d14ab73970508677910d`.

## Production transaction

Retained artifact ID `10549981416` was freshly re-proved `expired=false`, exact provider ZIP size `104942920` and provider/archive SHA-256 `1efad139b11b8df70b57d860b3ed0427e16607c781b252d464706f0719f08a9a` before transfer.

Controlled staging target:

`/home/gersi365/.local/state/private-remote-workspace/staging/c03e-vi-10549981416`

The exact retained ZIP was transferred to that staging target and verified on PowerCode:

- ZIP size `104942920` bytes;
- ZIP SHA-256 `1efad139b11b8df70b57d860b3ed0427e16607c781b252d464706f0719f08a9a`;
- `SHA256SUMS` verification: PASS;
- staged `prw-agent-configure` size `104941656` bytes;
- staged binary SHA-256 `35bc7623828f650f9feaa9490f1897951ac2fcf9f523d15c677934bd4797292d`;
- manifest source SHA / future production destination / mutation markers reverified.

Permanent installer source was staged from the exact C03e-VH lineage and verified with Git blob:

`8e206ef6da0d5ee64105128243b9706aaeea7156`

The remote automation context had no noninteractive sudo timestamp, so the real-root installer was executed user-attended in the authenticated PowerCode terminal with explicit `--allow-root-filesystem`. Reported result:

`installed=//usr/lib/private-remote-workspace/prw-agent-configure`

No password was recorded or transmitted through project evidence.

## Post-install proof

Canonical destination:

`/usr/lib/private-remote-workspace/prw-agent-configure`

Fresh read-only post-install verification:

- type: regular file;
- mode: `0755`;
- owner/group: `0:0` (`root:root`);
- size: `104941656` bytes;
- SHA-256: `35bc7623828f650f9feaa9490f1897951ac2fcf9f523d15c677934bd4797292d`;
- byte identity versus staged binary: PASS;
- `.prw-agent-configure.tmp.*` residue count: `0`;
- exact running `prw-agent-configure` process count: `0`.

## Negative runtime/configuration guards

Post-install state remained outside the configuration / activation boundary:

- `20-device-identity-credential.conf`: present, regular file, mode `0600`, uid/gid `1000:1000`, size `170`;
- `30-agent-execution-mode.conf`: ABSENT;
- `40-configured-remote-inputs.conf`: ABSENT;
- user manager `ActiveState=failed`;
- `SubState=failed`;
- `MainPID=0`;
- `NRestarts=6`;
- `NeedDaemonReload=yes`;
- manager-visible `DropInPaths=` remains empty;
- unit remains enabled;
- exact Agent process count: `0`.

No `prw-agent-configure write` or `reconfigure-active` was executed. No user-manager daemon reload, Agent start/restart, managed configuration mutation, credential read/decrypt/replace, enrollment mutation or networking mutation occurred.

## Hard boundary

C03e-VI stops after successful create-only binary installation and post-install verification.

It does not authorize:

- execution of `prw-agent-configure write` or `reconfigure-active`;
- creation/replacement/removal of managed `30` or `40` drop-ins;
- `systemctl --user daemon-reload`;
- Agent start/restart/activation;
- credential decryption or replacement;
- enrollment, revocation, networking, DNS, forwarding or relay mutation;
- merge, close, ready-for-review transition or branch deletion.

## STOP

`STOP_BEFORE_PRW_AGENT_CONFIGURE_MANAGED_LOCAL_ONLY_WRITE_AND_BEFORE_USER_SYSTEMD_MANAGER_RELOAD_OR_AGENT_ACTIVATION`

Next safe boundary: a separately authorized fresh read-only preflight for the production `prw-agent-configure write` transaction, using the installed canonical binary and exact writer semantics. It must stop before any actual managed configuration write unless separately authorized.

`NO_RACE_FREE_CLAIM`
