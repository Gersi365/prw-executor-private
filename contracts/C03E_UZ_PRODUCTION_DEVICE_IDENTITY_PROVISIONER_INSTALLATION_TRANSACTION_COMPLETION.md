# C03e-UZ Production Device-Identity Provisioner Installation Transaction Completion

Status: `REAL_ROOT_CREATE_ONLY_INSTALL_COMPLETED — POST_INSTALL_VERIFIED — PROVISIONER_NOT_EXECUTED — IDENTITY_NOT_PROVISIONED`

## Authority

This completion continues the already-authorized C03e-UZ production provisioner installation transaction after the prior checkpoint closed as blocked on user-attended sudo authentication.

The prior immutable blocked audit remains preserved and is not rewritten. This completion records the later successful continuation only.

## User-attended sudo continuation

The connected remote execution context could not reuse the user's sudo timestamp because the sudo credential was TTY/session-bound.

The user authenticated sudo in an SSH terminal on host `PowerCode` and executed the exact guarded create-only installer transaction in that same authenticated terminal.

The terminal reported:

`prw-device-identity-provision: OK`

and then:

`installed=//usr/lib/private-remote-workspace/prw-device-identity-provision`

The double slash is the installer's textual concatenation of `ROOT=/` with `/usr/...`; the observed installed filesystem object is the canonical path:

`/usr/lib/private-remote-workspace/prw-device-identity-provision`

No password was provided to ChatGPT or recorded in project evidence.

## Exact installer and source artifact

Permanent installer blob SHA:

`a2bf55f790469a08ef035bcdc90446be297a3751`

Staged provisioner source:

`/home/gersi365/.local/state/private-remote-workspace/staging/c03e-uz-10541149499/prw-device-identity-provision`

Expected provisioner SHA-256:

`1cb38c4816e841d8e1fe36b5093a2eff36f17d72a9e6b74062d57482a28a7459`

Immediately before the user-attended installer invocation, the guarded command re-checked `SHA256SUMS`, the exact installer Git blob, staged binary SHA-256, production destination absence, installer temp-residue absence, and `sudo -n true` in the authenticated terminal.

## Post-install filesystem verification

Fresh read-only post-install verification observed the canonical destination as:

- type: regular file;
- mode: `0755`;
- uid/gid: `0:0` (`root:root`);
- size: `55826168` bytes;
- device: `66306`;
- inode: `3017268`;
- SHA-256: `1cb38c4816e841d8e1fe36b5093a2eff36f17d72a9e6b74062d57482a28a7459`.

The installed destination and staged source were byte-for-byte identical (`cmp` PASS).

Installer temp residue matching `.prw-device-identity-provision.tmp.*` remained `0`.

Existing Agent SHA-256 remained unchanged:

`9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`

## Negative runtime and identity guards

Fresh `/proc/*/exe` inspection found no running process whose executable resolved to the installed provisioner path.

Fresh user-systemd state remained:

- `LoadState=loaded`;
- `ActiveState=failed`;
- `SubState=failed`;
- `MainPID=0`;
- `Result=exit-code`;
- `NRestarts=6`;
- `DropInPaths=` empty;
- `Linger=yes`.

The following remained absent:

- `~/.local/share/private-remote-workspace/credentials/device-identity-private-key-v1.cred`;
- `~/.config/systemd/user/prw-agent.service.d/20-device-identity-credential.conf`;
- `~/.config/systemd/user/prw-agent.service.d/30-local-only.conf`;
- `~/.config/systemd/user/prw-agent.service.d/40-production-network.conf`.

Therefore this checkpoint does not claim provisioner execution, identity creation, credential creation, drop-in creation, service activation, linger mutation, enrollment mutation, or networking mutation.

## Completion classification

`PRODUCTION_PROVISIONER_BINARY_INSTALLED_CREATE_ONLY — POST_INSTALL_BYTES_MODE_OWNERSHIP_VERIFIED — PROVISIONER_NOT_EXECUTED — IDENTITY_NOT_PROVISIONED`

## STOP

`STOP_BEFORE_PROVISIONER_EXECUTION_AND_PRODUCTION_IDENTITY_PROVISIONING`

Any execution of `prw-device-identity-provision`, creation of production identity material, `systemd-creds`, credential/drop-in mutation, service activation, enrollment, or networking remains separately gated.

`NO_RACE_FREE_CLAIM`
