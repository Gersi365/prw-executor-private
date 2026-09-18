# C03e-UY — Production Device-Identity Provisioner Installation Transaction Preflight Readiness

Status: `PREFLIGHT — RETAINED ARTIFACT REVERIFIED — PRODUCTION DESTINATION ABSENT — REAL-ROOT INSTALL PRECONDITIONS OBSERVED — EVIDENCE PENDING — NO HOST ARTIFACT TRANSFER — NO REAL-ROOT INSTALL — NO PROVISIONER EXECUTION`

Boundary: `PRODUCTION_DEVICE_IDENTITY_PROVISIONER_INSTALLATION_TRANSACTION_PREFLIGHT_READINESS`

Audit date: 2026-09-18

## Authority

Authoritative predecessor is evidence-closed C03e-UX / PR #688 at exact head:

`bf8ba5d770f41b02b752cc75ac1f3439a1243c44`

C03e-UX retained and independently verified the checksum-bound provisioner artifact but left a hard STOP before production-host artifact transfer/staging and before any real-root installation. C03e-UY is the read-only preflight allowed by that closure. It does not authorize transfer, staging, installation or execution.

The permanent installer contract at the exact predecessor requires the locked destination:

`/usr/lib/private-remote-workspace/prw-device-identity-provision`

and create-only, no-replace semantics. Real-root execution requires both explicit `--allow-root-filesystem` and uid 0. The provisioner must never be executed by the installer.

## Fresh predecessor recovery

Fresh reads before C03e-UY host inspection confirmed:

- repository `Gersi365/prw-executor-private`, repository ID `1334911207`;
- C03e-UX PR #688 remained open, draft, unmerged and mergeable;
- C03e-UX exact branch head remained `bf8ba5d770f41b02b752cc75ac1f3439a1243c44`;
- C03e-UX exact tree remained `1df3e57e28a2675bcbdf2c28e1757a1b1cdc500d`;
- `main` remained `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`, tree `63b8e59ca53797fdea6b95432e16f35eaf473604`;
- canonical C03e-UX Drive audit remained discoverable as ID `1sjLQWOq9PF2PPxTy_utDDjaG2XDNhICH`;
- no C03e-UY PR or Drive evidence collision was found;
- the intended C03e-UY branch did not exist before creation.

## Fresh retained-artifact availability and provider identity

C03e-UY freshly queried exact C03e-UX workflow run `35331996301` and confirmed retained artifact:

- artifact ID: `10541149499`;
- name: `c03e-ux-prw-device-identity-provision-bf8ba5d770f41b02b752cc75ac1f3439a1243c44`;
- source repository ID: `1334911207`;
- source Git head: `bf8ba5d770f41b02b752cc75ac1f3439a1243c44`;
- size: `55827441` bytes;
- expired: `false` at C03e-UY preflight;
- created: `2026-09-18T09:56:33Z`;
- expiry: `2026-12-17T09:54:49Z`;
- provider archive digest: `sha256:1514905144b902bdc6e8059c3fd9d6c96e3954b77b54d8348626cba9f4a5e3d2`.

## Fresh independent artifact re-download verification

C03e-UY downloaded artifact ID `10541149499` again outside the production host. No artifact bytes were sent to the production host.

Fresh downloaded ZIP:

- exact size: `55827441` bytes;
- SHA-256: `1514905144b902bdc6e8059c3fd9d6c96e3954b77b54d8348626cba9f4a5e3d2`;
- archive SHA-256 exactly equals the provider digest;
- exactly three top-level regular files;
- zero symlink entries.

Exact entries:

- `ARTIFACT-MANIFEST.txt`: 759 bytes, mode `0644`, SHA-256 `16647037d61e44508a5226807f47b2fa5b04e6abaaa60433b52a722ae6241725`;
- `SHA256SUMS`: 96 bytes, mode `0644`, SHA-256 `cd0a75607f23e52e2972058670a5002070f03e5150ee5a65f9f67d5abf962168`;
- `prw-device-identity-provision`: 55826168 bytes, mode `0755`, SHA-256 `1cb38c4816e841d8e1fe36b5093a2eff36f17d72a9e6b74062d57482a28a7459`.

`SHA256SUMS` binds the provisioner binary to:

`1cb38c4816e841d8e1fe36b5093a2eff36f17d72a9e6b74062d57482a28a7459`

The manifest freshly re-proved:

- `format=PRW_C03E_UX_PROVISIONER_ARTIFACT_V1`;
- `repository=Gersi365/prw-executor-private`;
- `source_git_sha=bf8ba5d770f41b02b752cc75ac1f3439a1243c44`;
- future production destination `/usr/lib/private-remote-workspace/prw-device-identity-provision`;
- `production_host_transfer=NOT_AUTHORIZED`;
- `real_root_install=NOT_AUTHORIZED`;
- `provisioner_execution=NOT_AUTHORIZED`;
- `identity_provisioning=NOT_AUTHORIZED`.

## Fresh production-host read-only proof

Authorized remote device `PowerCode` was online. C03e-UY ran read-only shell inspection only.

Observed user context:

- user `gersi365`;
- uid/gid `1000:1000`;
- home `/home/gersi365`.

The locked destination was freshly proven absent:

`/usr/lib/private-remote-workspace/prw-device-identity-provision` → `ABSENT`

Ancestor chain at the observation point:

- `/`: directory, mode `0755`, uid/gid `0:0`, device `66306`, inode `2`;
- `/usr`: directory, mode `0755`, uid/gid `0:0`, device `66306`, inode `2228225`;
- `/usr/lib`: directory, mode `0755`, uid/gid `0:0`, device `66306`, inode `2228229`;
- `/usr/lib/private-remote-workspace`: directory, mode `0755`, uid/gid `0:0`, device `66306`, inode `3035841`.

`namei -l` showed the existing ancestor chain as root-owned directories and the final provisioner leaf as `No such file or directory`. No observed existing ancestor component was a symlink.

Parent inventory contained exactly the pre-existing Agent payload at the inspection depth:

- `prw-agent`: regular file, mode `0755`, root:root, size `11068384` bytes;
- SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`.

No installer temporary residue matching `.prw-device-identity-provision.tmp.*` was present; count was `0`.

Filesystem proof for `/usr/lib/private-remote-workspace`:

- mount target `/`;
- source `/dev/nvme0n1p2`;
- filesystem `ext4`;
- mount options included `rw,relatime`;
- block size `4096`;
- available blocks `8623868` at the observation point, far exceeding the retained binary size.

The installer deliberately creates its temporary file in the same locked parent and publishes by a same-filesystem hard link, so the observed parent/filesystem layout is compatible with the already validated no-replace mechanism. This is a preflight observation, not an install execution proof.

## Observed Agent metadata drift

The current Agent bytes remain exactly the previously recorded SHA-256, but the current inode is `3017267`. A prior C03e-UU host proof recorded a different inode for the same Agent path. Current timestamps are:

- Agent mtime: `2026-09-18 07:16:21.497221996 +0200`;
- Agent ctime: `2026-09-18 07:16:21.596222660 +0200`;
- parent directory mtime/ctime: `2026-09-18 07:16:21.635222922 +0200`.

C03e-UY does not infer a cause for this metadata drift. It does not alter the provisioner destination result: the destination is absent and the parent remains root-owned `0755`. The drift is preserved as audit context rather than hidden or normalized away.

## Read-only command behavior note

The first host read command used `set -e` and `namei -l` against the intentionally absent final destination. `namei` returned non-zero because the leaf did not exist, so that read-only command ended before later read sections. A second read-only command re-ran the expected-missing path walk with that exit explicitly tolerated and completed the remaining inventory/filesystem guards. Neither command performed a host mutation.

## Preflight classification

Observed installation preconditions for a future separately authorized create-only transaction:

- retained artifact still exists and is unexpired;
- provider archive digest freshly matches downloaded bytes;
- retained bundle layout/checksum/manifest freshly re-verifies;
- exact artifact source SHA remains bound to evidence-closed C03e-UX head;
- locked production destination is absent;
- locked ancestor chain exists as root-owned non-symlink directories;
- no installer temp residue is present;
- target filesystem is currently read-write and has ample free space;
- permanent installer retains explicit real-root opt-in and uid-0 gates;
- atomic no-replace publication semantics remain in source.

This supports the classification:

`READ_ONLY_PREFLIGHT_READY_FOR_SEPARATELY_AUTHORIZED_CREATE_ONLY_INSTALL_TRANSACTION`

It does not prove a race-free future state. Destination and filesystem observations are point-in-time only. The installer's atomic no-replace publication is the later transaction's defense if the destination appears after this preflight.

## Hard prohibitions and mutation statement

C03e-UY performed no production-host artifact transfer or staging.

C03e-UY did not:

- write any file on the production host;
- invoke the real-root installer;
- invoke `prw-device-identity-provision`;
- generate or rotate a production identity key;
- invoke production `systemd-creds`;
- create, replace or delete the encrypted identity credential;
- create, replace or delete PRW `20/30/40` drop-ins;
- invoke `systemctl` or `loginctl`;
- mutate the systemd user manager or Agent service;
- mutate linger;
- mutate enrollment or networking;
- change the existing Agent payload;
- mutate `main`.

## STOP

`STOP_BEFORE_PRODUCTION_ARTIFACT_TRANSFER_OR_STAGING_AND_REAL_ROOT_INSTALL`

No production-host artifact transfer/staging and no real-root provisioner file installation is authorized by C03e-UY.

A later transaction must freshly re-prove the retained artifact and destination immediately before mutation, preserve the exact checksum-bound artifact identity, use the permanent create-only installer with explicit real-root opt-in under uid 0, and stop before provisioner execution or identity provisioning unless those later actions are separately authorized.

`NO_RACE_FREE_CLAIM`
