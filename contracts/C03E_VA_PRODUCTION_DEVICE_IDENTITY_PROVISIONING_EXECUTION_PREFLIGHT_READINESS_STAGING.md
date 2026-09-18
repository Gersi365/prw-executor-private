# Private Remote Workspace
# C03e-VA Production Device-Identity Provisioning Execution Preflight Readiness Staging

Status: `READ_ONLY_PREFLIGHT_READY_FOR_SEPARATELY_AUTHORIZED_PROVISIONER_EXECUTION — PROVISIONER_NOT_EXECUTED — IDENTITY_NOT_PROVISIONED`

Audit date: 2026-09-18
Repository: `Gersi365/prw-executor-private`
Repository ID: `1334911207`
Boundary: `PRODUCTION_DEVICE_IDENTITY_PROVISIONING_EXECUTION_PREFLIGHT_READINESS`

## 1. Authority and predecessor

Authoritative predecessor is evidence-closed C03e-UZ / PR #690 at exact completion head `38ff3f2d844091adb11ad06624ee0c4a12bbe31b`, tree `d9ea1b0ec197f031f02429562fb67327f176d6e5`.

C03e-UZ completed only the create-only installation of `/usr/lib/private-remote-workspace/prw-device-identity-provision` and stopped before provisioner execution and production identity provisioning.

This checkpoint is read-only with respect to the production host. It must not execute the provisioner, generate identity material, invoke production `systemd-creds encrypt`, create credentials/drop-ins, activate services, mutate linger, enrollment or networking.

## 2. Fresh predecessor / duplicate / main guards

Fresh reads confirmed:
- PR #690 remains open, draft, unmerged and mergeable;
- C03e-UZ exact head remains `38ff3f2d844091adb11ad06624ee0c4a12bbe31b`;
- no pre-existing C03e-VA branch was found before creation;
- no pre-existing C03e-VA PR was found;
- canonical Drive completion evidence for UZ remains the authoritative predecessor evidence;
- `main` remains `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`, tree `63b8e59ca53797fdea6b95432e16f35eaf473604`.

## 3. Exact installed provisioner identity

Fresh host read-only proof:
- path `/usr/lib/private-remote-workspace/prw-device-identity-provision`;
- regular file, non-symlink by observed path/custody checks;
- mode `0755`;
- uid/gid `0:0` (`root:root`);
- size `55826168` bytes;
- device `66306`;
- inode `3017268`;
- SHA-256 `1cb38c4816e841d8e1fe36b5093a2eff36f17d72a9e6b74062d57482a28a7459`.

Fresh `/proc/[pid]/exe` scan found `provisioner_running=0`.

## 4. Source execution contract

Exact CLI source blob at the UZ head is `f47856ee1675f7af6bb5831235bd73ec0b9dbff4`.

The CLI accepts no arguments. Any argument causes failure. On success it calls `provision_first_ubuntu_device_identity()` and reports only the public SPKI SHA-256 plus encrypted credential path. On error it emits a bounded failure classification.

Exact provisioning library blob at the UZ head is `faf3da9a4a412c309c091f3fb9f75716736ada09`.

Creation-only source semantics:
- resolves state root from `XDG_STATE_HOME`, otherwise `HOME/.local/state`;
- resolves config root from `XDG_CONFIG_HOME`, otherwise `HOME/.config`;
- fixed encrypted credential relative path `private-remote-workspace/credentials/device-identity-private-key-v1.cred`;
- fixed drop-in relative path `systemd/user/prw-agent.service.d/20-device-identity-credential.conf`;
- requires both final targets absent before creation;
- creates P-256 key material locally in process memory;
- persists only encrypted credential ciphertext and the fixed non-secret systemd binding;
- exposes no key import/export, enrollment or networking operation.

Fixed service-visible credential name is `prw.device-identity.private-key.v1`.

`systemd-creds` invocation is locked to `/usr/bin/systemd-creds --user encrypt --name=prw.device-identity.private-key.v1 - <temporary-ciphertext-path>` with private PKCS#8 supplied on stdin. The private buffer is zeroized after the write.

Ciphertext is validated and hardened to `0600`; the final drop-in is `0600`; both final commits use no-replace semantics.

## 5. Fresh XDG and target-path proof

Remote execution context:
- caller `gersi365`, uid/gid `1000:1000`;
- `HOME=/home/gersi365`;
- `XDG_STATE_HOME` unset;
- `XDG_CONFIG_HOME` unset.

Therefore the exact production targets for this execution are:
- encrypted credential: `/home/gersi365/.local/state/private-remote-workspace/credentials/device-identity-private-key-v1.cred`;
- credential drop-in: `/home/gersi365/.config/systemd/user/prw-agent.service.d/20-device-identity-credential.conf`.

Fresh read-only proof found both exact targets absent.

Also absent:
- `30-local-only.conf`;
- `40-production-network.conf`.

Credential temp residue count matching `.device-identity-private-key-v1.cred.phase126.*.tmp`: `0`.
Drop-in temp residue count matching `.20-device-identity-credential.conf.*.tmp`: `0`.

## 6. Directory custody / symlink proof

Observed existing path components are non-symlink and owned by uid/gid `1000:1000` where user-owned:
- `/home/gersi365`: mode `0750`;
- `/home/gersi365/.local`: mode `0700`;
- `/home/gersi365/.local/state`: mode `0700`;
- `/home/gersi365/.local/state/private-remote-workspace`: mode `0700`;
- `/home/gersi365/.config`: mode `0700`;
- `/home/gersi365/.config/systemd`: mode `0755`;
- `/home/gersi365/.config/systemd/user`: mode `0700`.

The credential directory `/home/gersi365/.local/state/private-remote-workspace/credentials` is currently absent and, under source semantics, would be created mode `0700` by the provisioner.

The drop-in directory `/home/gersi365/.config/systemd/user/prw-agent.service.d` is currently absent and, under source semantics, would be created mode `0700` by the provisioner.

`namei -l` showed no symlink substitution in the existing ancestor chain for either final target.

## 7. systemd-creds capability / user binding proof

Fresh host proof:
- `/usr/bin/systemd-creds` exists as regular root-owned file mode `0755`;
- size `60256` bytes;
- SHA-256 `f5c5f7edac86be6fba6409c52d76cd3159e95ac5d7a45bf7c715b3e65a1e1702`;
- reported version `systemd 259 (259.5-0ubuntu3.4)`;
- local help exposes `encrypt`, `--name=NAME`, and `--user`.

The source does not require sudo for provisioning. It executes as the calling user. Current caller identity resolves through NSS as `gersi365:1000:1000:/home/gersi365:/bin/bash`.

`/etc/machine-id` was verified present, non-empty and in 32-hex format without recording its value.

`sudo -n true` is currently unavailable in the Remote Desktop execution context, but this is not a blocker for the source-defined per-user provisioning operation.

No production encryption test was performed because this checkpoint is read-only.

## 8. User-systemd state

The raw Remote Desktop shell initially lacked `XDG_RUNTIME_DIR` and `DBUS_SESSION_BUS_ADDRESS`; this affected only the first read-only `systemctl --user` probe.

Using the existing user bus explicitly at `/run/user/1000/bus`, read-only service state was recovered:
- `LoadState=loaded`;
- `ActiveState=failed`;
- `SubState=failed`;
- `MainPID=0`;
- `Result=exit-code`;
- `NRestarts=6`;
- `DropInPaths=` empty;
- `Linger=yes`.

No service/systemd/linger mutation occurred.

## 9. Audit correction to predecessor wording

The C03e-UZ completion audit negative-credential guard recorded absence at `~/.local/share/private-remote-workspace/credentials/device-identity-private-key-v1.cred`.

The authoritative provisioning source uses `XDG_STATE_HOME` and, with `XDG_STATE_HOME` unset, resolves the actual credential target under `~/.local/state`, not `~/.local/share`.

C03e-VA explicitly corrects that historical evidence-path mismatch and freshly proves the actual source-defined target absent at:
`/home/gersi365/.local/state/private-remote-workspace/credentials/device-identity-private-key-v1.cred`.

This correction does not invalidate C03e-UZ installation evidence because C03e-UZ proved only binary installation and did not execute the provisioner. It is material for the next identity-provisioning boundary and is preserved here rather than silently inherited.

## 10. Preflight classification

`INSTALLED_PROVISIONER_IDENTITY_VERIFIED / SOURCE_DEFINED_TARGETS_ABSENT / DIRECTORY_CUSTODY_ACCEPTABLE / SYSTEMD_CREDS_USER_ENCRYPT_SURFACE_PRESENT / USER_BINDING_INPUTS_AVAILABLE / SERVICE_NOT_RUNNING / READY_FOR_SEPARATELY_AUTHORIZED_PROVISIONER_EXECUTION`

Readiness does not claim that an actual `systemd-creds --user encrypt` operation has succeeded on the production host. That can only be established during a separately authorized execution transaction and must fail closed on any encryption, persistence, ownership, mode, no-replace or durability error.

## 11. STOP

`STOP_BEFORE_PROVISIONER_EXECUTION_AND_PRODUCTION_IDENTITY_CREATION`

Not authorized in C03e-VA:
- execute `/usr/lib/private-remote-workspace/prw-device-identity-provision`;
- generate a production P-256 identity;
- invoke production `systemd-creds encrypt`;
- create encrypted credential ciphertext;
- create the `20` credential drop-in;
- create/modify `30` or `40` drop-ins;
- daemon-reload, start/restart/enable service;
- mutate linger;
- enrollment or networking mutation.

`NO_RACE_FREE_CLAIM`
