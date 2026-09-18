# C03e-UQ — User-attended package reconciliation success

## Status and boundary

`TRANSACTION — VALIDATED — PACKAGE RECONCILIATION SUCCEEDED — SERVICE ACTIVATION NOT AUTHORIZED`

Boundary:

`USER_ATTENDED_PACKAGE_RECONCILIATION_SUCCESS`

C03e-UQ records the successful retry after evidence-closed C03e-UP. The user executed the exact UO-frozen sudo-rs transaction command directly in a local terminal on `PowerCode`, entered the credential only into the local sudo-rs prompt, and reported the non-secret terminal result line:

`prw-agent-package-reconcile result=current_agent_payload_reconciled`

Independent post-execution host readback, not the user-reported line alone, is the authority for the installed package-success claim.

## Authoritative predecessor

Authoritative predecessor is evidence-closed C03e-UP / PR #680.

Exact predecessor:

- branch `phase-152-c03e-up-user-attended-package-reconciliation-transaction`;
- head `87b56283d88450ed8cbcb7dd6dd16993daed7e9b`;
- tree `9ce9ffc802e18890302f07ed6e4c3f087f0f6258`;
- status `TRANSACTION — EVIDENCE_RECORDED — CLOSED — BLOCKED ON HOST CONNECTION BEFORE IMMEDIATE REPROOF — NO COMMAND HANDOFF — NO PACKAGE MUTATION`;
- retained open, draft and unmerged.

UP's historical blocked attempt remains immutable. UQ is the separately recorded successful retry after host reconnection.

## Fresh immediate reproof before user handoff

After `PowerCode` reconnected, UQ repeated the complete transaction guard set.

Fresh proof immediately before handoff established:

- operator UID/GID `1000:1000`;
- private stage parent chain remained non-symlink;
- stage `/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage` remained UID/GID `1000:1000`, mode `0700`;
- stage contained exactly `C03E_UF_DEPLOYMENT_MANIFEST`, `candidate-prw-agent`, and `prw-agent-package-reconcile`;
- staged reconciler remained mode `0500`, bytes `2898840`, SHA-256 `a73a82a7fabc6113c97a48d1e61008e4c45589856be29573918026aca950114c`;
- staged Agent remained mode `0500`, bytes `11068384`, SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- staged manifest remained mode `0600`, bytes `602`, SHA-256 `56a379041a469f95c4b778bcb3b3d8aff5fa856e76c338eb15b01288b469ace1`;
- `/usr/bin/sudo` resolved to `/usr/lib/cargo/bin/sudo`;
- active sudo-rs remained `0.2.13-0ubuntu1.2`, root-owned mode `4755`, bytes `1090848`, SHA-256 `2eb5d31f91a12d75a2a05b54b7f79775e5565898af0b3263414fcabaad922afb`;
- root still resolved UID/GID `0:0`;
- installed old Agent remained root-owned mode `0755`, bytes `2865776`, SHA-256 `4dbb114edc5ad131dd8abcf2ba798a413a249f2a3931a43b59f67b496e0d242e`;
- vendor unit remained root-owned mode `0644`, bytes `332`, SHA-256 `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`;
- root reconciliation residue count was zero;
- immediate service state remained `failed/failed`, `MainPID=0`, exact vendor FragmentPath, empty DropInPaths;
- exact `prw-agent` process count for UID 1000 was zero.

An additional immediate service/process refresh directly before handoff reproduced the non-running service proof.

`NO_RACE_FREE_CLAIM` remains mandatory because the service is enabled and linger remains enabled.

## Exact user-attended transaction

The exact transaction argv remained unchanged from UM/UO:

- argv[0] `/usr/bin/sudo`
- argv[1] `-u`
- argv[2] `root`
- argv[3] `--`
- argv[4] `/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage/prw-agent-package-reconcile`
- argv[5] `reconcile-current-agent`
- argv[6] `/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage`

The user executed that exact command in a local terminal on `PowerCode` and entered the password only into the same terminal's sudo-rs prompt.

No credential was sent to ChatGPT or Remote Desktop Commander and no `sudo -S`, askpass, NOPASSWD, sudoers mutation, alternatives mutation, shell wrapper, env assignment, redirect, pipe, alternate executable or alternate stage path was used.

User-reported terminal result:

`prw-agent-package-reconcile result=current_agent_payload_reconciled`

UQ does not claim an independently captured shell exit status because the command ran outside the connected execution surface.

## Independent post-execution package proof

Fresh host readback after the local command terminated proved the installed Agent changed to the selected payload:

`/usr/lib/private-remote-workspace/prw-agent`

- owner/group `root:root`;
- mode `0755`;
- bytes `11068384`;
- filesystem device `66306`;
- SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`.

This exactly matches the selected staged Agent candidate.

The prior Agent identity `4dbb114edc5ad131dd8abcf2ba798a413a249f2a3931a43b59f67b496e0d242e` is no longer installed.

Therefore the current-Agent package payload reconciliation succeeded.

## Vendor unit unchanged

Independent post-readback proved:

`/usr/lib/systemd/user/prw-agent.service`

- owner/group `root:root`;
- mode `0644`;
- bytes `332`;
- SHA-256 `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`.

The vendor unit was not rewritten by the package transaction.

## Private stage retained

Independent post-readback proved the private stage remained UID/GID `1000:1000`, mode `0700`, with exactly the same three selected children.

Staged identities remained exact:

- reconciler: mode `0500`, bytes `2898840`, SHA-256 `a73a82a7fabc6113c97a48d1e61008e4c45589856be29573918026aca950114c`;
- Agent candidate: mode `0500`, bytes `11068384`, SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- manifest: mode `0600`, bytes `602`, SHA-256 `56a379041a469f95c4b778bcb3b3d8aff5fa856e76c338eb15b01288b469ace1`.

## Root reconciliation residue cleanup

Independent post-readback found:

`root_residue_count=0`

No `.prw-agent.c03e-ug.*.candidate` root reconciliation sibling remained after the successful exchange.

## Service state after package transaction

Independent post-readback observed:

- `LoadState=loaded`;
- `ActiveState=failed`;
- `SubState=failed`;
- `FragmentPath=/usr/lib/systemd/user/prw-agent.service`;
- `DropInPaths=` empty;
- `UnitFileState=enabled`;
- `MainPID=0`;
- `Result=exit-code`;
- exact `prw-agent` process count for UID 1000 was `0`;
- login state remained active;
- linger remained enabled.

No service start/restart/reload or manager mutation was performed as part of the transaction or post-readback.

Package-byte success does not imply service activation success.

## Security and evidence limits

UQ does not claim:

- race-free package replacement;
- that the user credential is known or recorded;
- service activation or runtime health;
- service restart success;
- vendor-unit mutation;
- identity/20/30/40 mutation;
- network/listener/database/auth/control-plane mutation.

The user-reported terminal result is supplementary. Independent installed-Agent bytes/hash are the package-success authority.

## Explicit non-actions / STOP

UQ performs no:

- additional sudo command;
- service start/stop/restart/reload;
- enablement or linger mutation;
- private-stage cleanup;
- vendor-unit rewrite;
- sudoers or sudo-alternatives mutation;
- credential storage/transmission;
- arbitrary root shell;
- identity/20/30/40 mutation;
- network/listener/database/auth/control-plane mutation;
- repository main mutation;
- merge;
- ready conversion;
- PR close;
- branch deletion;
- history rewrite;
- destructive evidence cleanup.

STOP after evidence closure of this package-reconciliation success checkpoint.

The next safe boundary is a separately authorized read-only post-package service/runtime readiness checkpoint. It may inspect the newly installed Agent identity and service/runtime preconditions, but it must not start/restart/activate the service unless separately and explicitly authorized.