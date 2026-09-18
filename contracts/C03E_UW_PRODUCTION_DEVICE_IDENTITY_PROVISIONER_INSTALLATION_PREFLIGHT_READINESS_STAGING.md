# Private Remote Workspace
# C03e-UW Production Device-Identity Provisioner Installation Preflight / Readiness Staging

Status: `PREFLIGHT — HOST READ-ONLY PROOFS COMPLETE — BLOCKED ON RETAINED VALIDATED PROVISIONER ARTIFACT DELIVERY — NO PRODUCTION INSTALL — NO IDENTITY PROVISIONING`

Audit date: 2026-09-18
Repository: `Gersi365/prw-executor-private`
Repository ID: `1334911207`
Boundary: `PRODUCTION_DEVICE_IDENTITY_PROVISIONER_INSTALLATION_PREFLIGHT_READINESS`

## 1. Authority and predecessor

Authoritative predecessor is evidence-closed C03e-UV / PR #686 at exact head:

`8ec861036a265f9d3d4a5066b762999c47ac452b`

Exact predecessor tree:

`78671e9a8db3f811f1b5a613c079ce82e3c6e199`

C03e-UV materialized and validated the permanent non-activating create-only package-file transaction for `prw-device-identity-provision`. It explicitly stopped before any production-host installation or identity provisioning.

C03e-UW is limited to fresh read-only production-host installation preflight/readiness. It does not authorize or perform the real-root package-file transaction.

## 2. Locked future destination and installer source

Locked future production destination:

`/usr/lib/private-remote-workspace/prw-device-identity-provision`

Permanent installer source at the authoritative predecessor:

`packaging/systemd/install-prw-device-identity-provision.sh`

Installer blob SHA:

`a2bf55f790469a08ef035bcdc90446be297a3751`

Provisioner install contract:

`packaging/systemd/PROVISIONER_INSTALL_TRANSACTION.md`

Contract blob SHA:

`40719d31609398f114215d7c06f782bf257382f3`

Relevant locked semantics retained from C03e-UV:

- create-only destination materialization;
- source artifact must be a regular non-symlink file with basename `prw-device-identity-provision`;
- `ROOT=/` requires explicit `--allow-root-filesystem`;
- real-root mode requires uid 0;
- destination must be absent in every form;
- staged bytes are copied into a private same-directory temporary file;
- mode is fixed to `0755`;
- staged bytes must equal source artifact bytes;
- publication uses same-filesystem atomic hard-link no-replace semantics;
- final type, non-symlink state, mode and bytes are re-verified;
- real-root final ownership must be `0:0`;
- the installer never executes the provisioner.

## 3. Fresh duplicate / predecessor guards

Before creating C03e-UW:

- `main` was freshly read at exact head `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`, tree `63b8e59ca53797fdea6b95432e16f35eaf473604`;
- C03e-UV PR #686 remained open, draft, unmerged and evidence-closed at exact head `8ec861036a265f9d3d4a5066b762999c47ac452b`;
- GitHub branch search for `phase-152-c03e-uw` returned no branch;
- GitHub PR search for `C03e-UW` returned zero results;
- canonical Drive evidence-folder search for `C03E_UW` returned no result.

C03e-UW branch was therefore created from the exact evidence-closed C03e-UV head, not from `main` and not from an earlier predecessor.

## 4. Fresh production-host identity

Authorized remote host channel:

- device name: `PowerCode`;
- device ID: `a43dde76-22d5-4efa-b388-38d04ee9635d`;
- host name observed by the read-only shell: `PowerCode`.

Read-only shell identity:

- user: `gersi365`;
- uid: `1000`;
- gid: `1000`.

No `sudo` command was invoked. No privileged command was attempted. Current shell identity does not satisfy the installer's separately gated real-root uid-0 requirement.

## 5. Fresh locked-destination proof

The exact future production destination was freshly probed:

`/usr/lib/private-remote-workspace/prw-device-identity-provision`

Observed state:

`ABSENT`

`namei -l` proved the existing ancestor chain as:

- `/` — directory, mode `0755`, uid/gid `0:0`;
- `/usr` — directory, mode `0755`, uid/gid `0:0`;
- `/usr/lib` — directory, mode `0755`, uid/gid `0:0`;
- `/usr/lib/private-remote-workspace` — directory, mode `0755`, uid/gid `0:0`;
- final provisioner path — absent.

Observed device number for the existing chain:

`66306`

No symlink component was observed in the locked existing parent chain.

The absence proof is point-in-time only. C03e-UW does not claim a race-free reservation of the destination.

## 6. Existing canonical PRW inventory

Fresh inventory directly under `/usr/lib/private-remote-workspace` contained only:

`prw-agent`

Observed Agent metadata:

- regular file;
- mode `0755`;
- uid/gid `0:0`;
- size `11068384` bytes;
- device `66306`;
- inode `3017267`;
- observed mtime `2026-09-18 07:16:21.497221996 +0200`.

Fresh Agent SHA-256:

`9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`

Alternate provisioner locations were also freshly checked and absent:

- `/usr/bin/prw-device-identity-provision`;
- `/usr/local/bin/prw-device-identity-provision`;
- `/usr/libexec/private-remote-workspace/prw-device-identity-provision`.

No matching `prw-device-identity-provision` process was present in the fresh process-list reproof.

One earlier read-only process-list probe had a shell/awk quoting error and yielded no valid process-state evidence. It performed no mutation and was immediately superseded by the successful simpler read-only process-list probe above.

## 7. Filesystem and installer-utility readiness

The locked parent path resolves to:

- mount target: `/`;
- source: `/dev/nvme0n1p2`;
- filesystem: `ext4`;
- mount options observed: `rw,relatime`.

Fresh `df -Pk` output for the parent filesystem reported:

- total 1K blocks: `243937628`;
- used: `196763152`;
- available: `34710300`;
- utilization: `86%`.

All command surfaces used by the permanent installer were freshly resolvable:

- `bash=/usr/bin/bash`;
- `id=/usr/bin/id`;
- `basename=/usr/bin/basename`;
- `mkdir=/usr/bin/mkdir`;
- `chmod=/usr/bin/chmod`;
- `mktemp=/usr/bin/mktemp`;
- `cp=/usr/bin/cp`;
- `stat=/usr/bin/stat`;
- `cmp=/usr/bin/cmp`;
- `ln=/usr/bin/ln`;
- `rm=/usr/bin/rm`.

No write test was performed against `/usr` or the locked destination parent.

## 8. Validated build identity from C03e-UV

C03e-UV exact-head dedicated validation succeeded at run:

`35324710761`

Run number:

`2`

The exact provisioner binary built during that disposable validation emitted SHA-256:

`1cb38c4816e841d8e1fe36b5093a2eff36f17d72a9e6b74062d57482a28a7459`

That proves the disposable build and package-file transaction used those exact build bytes during the successful UV proof.

It does not, by itself, prove that those bytes are currently retained as a production-deliverable artifact.

## 9. Retained artifact delivery guard

Fresh GitHub Actions artifact query for the successful dedicated UV run returned:

- `total_count: 0`;
- `artifacts: []`.

The UV workflow builds and validates the binary in the runner but contains no artifact-upload step.

Fresh repository release query returned an empty release list:

`[]`

Therefore C03e-UW did not find a retained GitHub Actions artifact or GitHub release asset containing the exact validated provisioner build bytes.

The repository's older Phase 107 Agent transaction contract also explicitly states that managed-file identity and signed/reproducible release payload verification remain release-packaging work.

## 10. Readiness classification

### Host-side transaction mechanics

The observed host is mechanically compatible with the already-validated create-only installer contract:

- locked destination absent;
- existing parent chain is root-owned, non-symlink and mode `0755`;
- parent filesystem is writable ext4;
- installer utility dependencies are available;
- no alternate deployed provisioner was observed;
- no matching provisioner process was observed.

This is a read-only readiness observation only. It does not authorize the transaction.

### Privilege boundary

Current authorized shell is uid 1000. Real-root installation requires a separately authorized uid-0 transaction and explicit `--allow-root-filesystem` opt-in. C03e-UW neither acquires nor exercises that privilege.

### Artifact-delivery boundary

Production installation is **not ready** because the exact validated provisioner artifact bytes are not retained in a canonical production-delivery surface.

C03e-UW therefore classifies the current state as:

`HOST_DESTINATION_AND_MECHANICS_READY / RETAINED_VALIDATED_PROVISIONER_ARTIFACT_DELIVERY_ABSENT / REAL_ROOT_INSTALL_NOT_AUTHORIZED`

No claim is made that an arbitrary fresh rebuild would be byte-identical to the UV validation artifact. No production rebuild, transfer, signing, release publication, or host staging is performed here.

## 11. Hard prohibitions preserved

C03e-UW does not:

- build the provisioner on the production host;
- upload or transfer a provisioner binary to the production host;
- execute `install-prw-device-identity-provision.sh` against `ROOT=/`;
- write `/usr/lib/private-remote-workspace/prw-device-identity-provision`;
- execute `prw-device-identity-provision`;
- generate a P-256 production identity;
- execute production `systemd-creds` encryption;
- create, replace, rotate, import or delete the production encrypted identity credential;
- create, replace or delete PRW `20/30/40` user-service drop-ins;
- call mutating `systemctl` or `loginctl` operations;
- run `daemon-reload`;
- start, stop, restart, reset, enable or disable `prw-agent.service`;
- mutate linger or systemd user-manager environment;
- mutate enrollment or networking;
- modify package/vendor-unit files;
- merge, close, ready, or delete any PR/branch.

## 12. C03e-UW STOP boundary

C03e-UW stops before any artifact-retention/delivery materialization and before any production real-root install.

The next safe source/evidence boundary is to materialize a retained, checksum-bound provisioner artifact delivery path that can preserve or independently prove the exact bytes intended for installation without executing the provisioner and without touching the production host.

Only after that separate checkpoint is evidence-closed may a later checkpoint freshly re-prove host destination absence again and request explicit authorization for the real-root create-only install transaction.

`NO_RACE_FREE_CLAIM`
