# C03e-UU — Production device-identity provisioning deployment preflight/readiness

## 1. Final checkpoint status

`PREFLIGHT — HOST READ-ONLY PROOFS COMPLETE — BLOCKED ON PROVISIONER PACKAGING/DEPLOYMENT INTEGRATION — NO PROVISIONING TRANSACTION — NO HOST MUTATION — EVIDENCE PUBLICATION PENDING`

Boundary:

`PRODUCTION_DEVICE_IDENTITY_PROVISIONING_DEPLOYMENT_PREFLIGHT_READINESS`

C03e-UU is a read-only deployment-preflight checkpoint following evidence-closed C03e-UT. It proves the production-host prerequisites immediately before any separately authorized first-production device-identity provisioning transaction.

C03e-UU does not authorize or perform production key generation, production `systemd-creds` encryption, credential/drop-in creation, `20/30/40` mutation, systemd manager mutation, service activation/control, package mutation, sudo/root mutation, enrollment, networking, merge, ready conversion, PR close, branch deletion, `main` mutation, history rewrite, or evidence cleanup.

The fresh host inspection completed through the authorized read-only remote-host connector. The result is not a provisioning PASS: the source provisioner executable exists in the exact repository lineage, but no corresponding executable is deployed on the production host and the exact repository packaging contract does not integrate that executable.

Canonical classification:

`BLOCKED_ON_PROVISIONER_PACKAGING_DEPLOYMENT_INTEGRATION`

## 2. Authoritative predecessor

Authoritative predecessor is evidence-closed C03e-UT / PR #684:

- branch: `phase-152-c03e-ut-device-identity-dropin-integration-source-materialization`;
- exact head: `3d5a1df14c65a9e0e92276be49cd69cde763b5dc`;
- exact tree: `8ca5fbabf47ad26d12bfb78b30c0fe4fdf5e3619`;
- final source Git blob: `faf3da9a4a412c309c091f3fb9f75716736ada09`;
- exact source path: `crates/prw-device-identity-provisioning/src/lib.rs`;
- PR #684 remained open, draft, unmerged;
- Rust Validation #1933 / run `35317667095`: `SUCCESS`;
- Android Validation #1858 / run `35317666980`: `SUCCESS`;
- C02f-AD #1181 and C02f-AE #1172: `SKIPPED`; `SKIPPED` is not PASS.

C03e-UT provides creation-only first identity provisioning plus fixed per-user `20-device-identity-credential.conf` materialization. It does not by itself prove current production-host state or deployment of the provisioner executable.

## 3. C03e-UU staging lineage before final host findings

Before this final staging update, UU was:

- branch: `phase-152-c03e-uu-production-device-identity-provisioning-deployment-preflight-readiness`;
- head: `9147af9e17f77d0ce9d8f8b1b4fc02e36e608262`;
- tree: `2cda8f18a5577e755e5535e4693607a3e61e5cfa`;
- exact UT base: `3d5a1df14c65a9e0e92276be49cd69cde763b5dc`;
- exactly one changed path: this contract;
- prior contract blob: `f4a021b6a20c3ff78719ac97f451f533d6be94fc`;
- prior delta: `+201/-0`;
- PR #685: open, draft, unmerged.

Prior exact-head CI on `9147af9e17f77d0ce9d8f8b1b4fc02e36e608262`:

- PRW Rust Validation #1934 / run `35319525989` / job `105518685655`: `SUCCESS`;
- C02f-AD #1182 / run `35319526040`: `SKIPPED`;
- C02f-AE #1173 / run `35319526007`: `SKIPPED`;
- no Android run was reported for that docs-only head; no Android PASS claim is made.

This document update requires a new exact-head CI binding before evidence publication.

## 4. Fresh production-host read-only inspection

Authorized host:

- connector device name: `PowerCode`;
- authorized device ID: `a43dde76-22d5-4efa-b388-38d04ee9635d`.

All observations below are point-in-time reads. `NO_RACE_FREE_CLAIM` applies throughout.

### 4.1 Intended user and effective XDG roots

Fresh identity:

- user: `gersi365`;
- UID: `1000`;
- GID: `1000`;
- HOME: `/home/gersi365`;
- `XDG_STATE_HOME`: unset, therefore effective state root `/home/gersi365/.local/state`;
- `XDG_CONFIG_HOME`: unset, therefore effective config root `/home/gersi365/.config`.

Fresh path custody:

- `/home/gersi365`: directory, mode `0750`, UID/GID `1000/1000`;
- `/home/gersi365/.local`: directory, mode `0700`, UID/GID `1000/1000`;
- `/home/gersi365/.local/state`: directory, mode `0700`, UID/GID `1000/1000`;
- `/home/gersi365/.local/state/private-remote-workspace`: directory, mode `0700`, UID/GID `1000/1000`;
- `/home/gersi365/.local/state/private-remote-workspace/credentials`: absent;
- `/home/gersi365/.config`: directory, mode `0700`, UID/GID `1000/1000`;
- `/home/gersi365/.config/systemd`: directory, mode `0755`, UID/GID `1000/1000`, not group/other writable;
- `/home/gersi365/.config/systemd/user`: directory, mode `0700`, UID/GID `1000/1000`;
- `/home/gersi365/.config/systemd/user/prw-agent.service.d`: absent.

A `namei -l` custody read showed the expected root-owned `/` and `/home`, user-owned custody after `/home/gersi365`, and no observed symlink substitution on the inspected paths.

### 4.2 Production identity objects and `20/30/40`

Freshly absent:

- `/home/gersi365/.local/state/private-remote-workspace/credentials/device-identity-private-key-v1.cred`;
- `/home/gersi365/.config/systemd/user/prw-agent.service.d/20-device-identity-credential.conf`;
- `/home/gersi365/.config/systemd/user/prw-agent.service.d/30-agent-execution-mode.conf`;
- `/home/gersi365/.config/systemd/user/prw-agent.service.d/40-configured-remote-inputs.conf`.

No object was created, removed, renamed, chmodded, chowned, opened for writing, or otherwise mutated by UU.

### 4.3 Exact `systemd-creds` identity

Fresh `/usr/bin/systemd-creds` proof:

- exact path: `/usr/bin/systemd-creds`;
- regular file;
- mode: `0755`;
- owner: `root:root`;
- size: `92800` bytes;
- inode: `1903848`;
- device: `259`;
- mtime: `2026-04-14 15:57:19 +0300`;
- SHA-256: `a3b41ed113e5778af9a147722b36e92d15020ae4929cbb7351e6d51b739c4503`;
- version: `systemd 257 (257.9-1ubuntu3.2)`.

UU invoked only version/identity reads. It did not execute production encryption and did not provide private-key material to `systemd-creds`.

### 4.4 Installed Agent identity

Installed Agent:

`/usr/lib/private-remote-workspace/prw-agent`

Fresh identity:

- regular file;
- mode `0755`;
- owner `root:root`;
- size `11068384` bytes;
- inode `1961642`;
- device `259`;
- SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`.

This matches the previously reconciled candidate/new Agent payload identity.

### 4.5 Vendor user unit identity

Vendor unit:

`/usr/lib/systemd/user/prw-agent.service`

Fresh identity:

- regular file;
- mode `0644`;
- owner `root:root`;
- size `805` bytes;
- inode `1961093`;
- device `259`;
- SHA-256 `1080eb14d38d6befb37bc1c2fa228ec714dc61dbb2de062d252089364d654046`.

The effective vendor service retains:

- `Type=simple`;
- `ExecStart=/usr/lib/private-remote-workspace/prw-agent`;
- `Restart=on-failure`;
- `RestartSec=5`;
- the previously established hardening directives;
- `WantedBy=default.target`.

### 4.6 Fresh user-service state

The connector shell initially lacked the user-bus environment, so an initial `systemctl --user` read returned `Failed to connect to bus: No medium found`. No service operation occurred.

For the subsequent read-only query only, the shell exported:

- `XDG_RUNTIME_DIR=/run/user/1000`;
- `DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus`.

Existing bus custody was read as:

- `/run/user/1000`: directory, mode `0700`, UID/GID `1000/1000`;
- `/run/user/1000/bus`: socket, UID/GID `1000/1000`.

Fresh service state:

- Id: `prw-agent.service`;
- LoadState: `loaded`;
- ActiveState: `failed`;
- SubState: `failed`;
- MainPID: `0`;
- NRestarts: `6`;
- FragmentPath: `/usr/lib/systemd/user/prw-agent.service`;
- DropInPaths: empty;
- UnitFileState: `enabled`;
- UnitFilePreset: `enabled`;
- ExecMainCode: `1`;
- ExecMainStatus: `1`;
- Result: `start-limit-hit`;
- `systemctl --user is-enabled`: `enabled`;
- `systemctl --user is-active`: `failed`;
- `systemctl --user is-failed`: `failed`.

`systemctl --user cat prw-agent.service` showed only the vendor unit and no effective drop-ins.

The user manager environment contained no `PRW_*` assignments.

Fresh login-manager read:

- `Linger=yes`;
- user state `active`;
- one session reported.

Fresh process snapshot:

- exact UID-1000 process count with `comm=prw-agent`: `0`.

Because the unit remains enabled and linger is enabled, these observations cannot prove that an external activation surface will not race after inspection. `NO_RACE_FREE_CLAIM` remains mandatory.

## 5. Source provisioner entrypoint proof

Exact repository lineage contains an executable target:

`crates/prw-device-identity-provisioning/src/bin/prw-device-identity-provision.rs`

Git blob:

`cfef16a33643260d7284070e5f49a60be22e6cf1`

Its executable entrypoint directly calls:

`provision_first_ubuntu_device_identity()`

and exits nonzero on provisioning error.

The crate package is `prw-device-identity-provisioning`, and the `src/bin/` file establishes the executable target `prw-device-identity-provision` in the exact source lineage.

UU did not execute that target.

## 6. Production-host provisioner deployment proof

Fresh host lookup found no deployed `prw-device-identity-provision` executable.

`command -v prw-device-identity-provision` returned no path.

Freshly absent at the canonical candidate locations inspected:

- `/usr/bin/prw-device-identity-provision`;
- `/usr/local/bin/prw-device-identity-provision`;
- `/usr/lib/private-remote-workspace/prw-device-identity-provision`;
- `/usr/libexec/private-remote-workspace/prw-device-identity-provision`.

The installed `prw-*` executable inventory in the inspected canonical system directories contained only:

`/usr/lib/private-remote-workspace/prw-agent`

Therefore the selected creation-only source cannot currently be invoked through a deployed production provisioner artifact.

## 7. Package/deployment integration diagnosis

Fresh host package-database reads returned no `dpkg-query -S` ownership match for:

- `/usr/lib/private-remote-workspace/prw-agent`;
- `/usr/lib/systemd/user/prw-agent.service`.

UU therefore does not claim these installed files are currently managed by an installed dpkg package.

Exact repository packaging tree at the UU predecessor contains only:

- `packaging/systemd/INSTALL_TRANSACTION.md`;
- `packaging/systemd/README.md`;
- `packaging/systemd/prw-agent.service`.

The exact package file transaction contract locks only these managed destinations:

- executable `/usr/lib/private-remote-workspace/prw-agent`;
- global user unit `/usr/lib/systemd/user/prw-agent.service`.

That contract explicitly states that Phase 107 does not define a package-manager database implementation and that managed-file identity plus signed/reproducible release-payload verification remain release-packaging work.

Exact `scripts/` contains only the C02f-AD/AE disposable-etcd validation scripts. Exact `tools/` contains only the corresponding validation sources. Neither tree provides a provisioner installer/deployment path.

Canonical result:

`SOURCE_PROVISIONER_TARGET_EXISTS / PRODUCTION_PROVISIONER_ARTIFACT_ABSENT / EXACT_PACKAGING_CONTRACT_AGENT_AND_UNIT_ONLY / NO_PROVISIONER_INSTALL_INTEGRATION / BLOCKED_ON_PROVISIONER_PACKAGING_DEPLOYMENT_INTEGRATION`

This is a deployment-readiness blocker. The next safe checkpoint is source/package/deployment integration materialization, not production identity provisioning.

## 8. Incidental Git object disclosure

During C03e-UU source inspection, the connector was mistakenly invoked with Git commit-object creation twice. This created two Git commit objects that were not attached to any branch/ref/PR:

1. `7554573df4c68cd764479b2d8f6939cad56b70ad`, message `noop`;
2. `7099c6961dc15535304c6a666446675a91afaca4`, message `noop2`.

Both objects use:

- tree `2cda8f18a5577e755e5535e4693607a3e61e5cfa`;
- parent `9147af9e17f77d0ce9d8f8b1b4fc02e36e608262`.

Immediately after disclosure, all further Git operations were restricted to explicit read functions until this intentional contract update.

Fresh ref reproof after the incident showed:

- UU branch still at `9147af9e17f77d0ce9d8f8b1b4fc02e36e608262` before this intentional update;
- `main` still at `a7ffafd6a6d5a032dd8290eec24df1349bade6cc` / tree `63b8e59ca53797fdea6b95432e16f35eaf473604`;
- neither incidental commit object was attached to the active UU lineage or `main`.

The objects are not deleted, rewritten, hidden, or promoted into active history. They are preserved as an explicit evidence fact.

## 9. Host non-actions

C03e-UU performed no:

- production P-256 key generation;
- production `systemd-creds` encryption;
- encrypted credential creation/replacement/deletion;
- production `20/30/40` creation/replacement/deletion;
- systemd daemon reload;
- service start/restart/stop/reset-failed;
- enable/disable operation;
- linger mutation;
- manager-environment mutation;
- sudo/root host mutation;
- Agent/package/vendor-unit rewrite;
- provisioner installation;
- enrollment or networking mutation.

All shell actions used for the successful host inspection were read-only state/identity queries.

## 10. Readiness decision

The base host prerequisites required by the selected first-production identity semantics are currently observable and consistent with a pre-provisioning state:

- intended user and effective XDG roots identified;
- relevant existing path custody is user-owned and non-writable by group/other where required;
- production encrypted identity credential absent;
- production `20/30/40` leaves absent;
- exact `/usr/bin/systemd-creds` identified;
- expected Agent payload identified;
- vendor unit identified;
- current service state established;
- zero current UID-1000 `prw-agent` processes observed.

However deployment readiness fails closed because the source provisioner executable is not deployed and no exact packaging/deployment integration for it exists in the inspected repository packaging/scripts/tools surfaces.

C03e-UU therefore MUST NOT authorize a production identity transaction.

If the provisioner artifact later becomes deployable, a subsequent checkpoint must still freshly re-prove host state immediately before any production transaction. Historical UU state must not be promoted to future current state.

## 11. Validation and evidence closure requirements

This contract update is docs-only. Before immutable evidence publication:

- bind final exact UU head/tree/blob/diff;
- verify exact UT merge-base/topology;
- verify exactly one changed docs path;
- run and wait for exact-final-head CI;
- record `SKIPPED` as skipped, not PASS;
- refresh PR #685 open/draft/unmerged state;
- refresh `main` exact identity;
- verify reserved Drive audit title remains collision-free.

Reserved immutable audit filename:

`C03E_UU_PRODUCTION_DEVICE_IDENTITY_PROVISIONING_DEPLOYMENT_PREFLIGHT_READINESS_AUDIT_2026-09-18.md`

Canonical Drive parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

The immutable audit should freeze with evidence-publication-pending wording. The post-publication PR #685 body is the closure binding and must remain open/draft/unmerged.

## 12. Final STOP boundary

`STOP_BEFORE_PROVISIONER_PACKAGING_DEPLOYMENT_MATERIALIZATION`

No production identity provisioning transaction is authorized.

The next safe boundary is a separately authorized source/package/deployment checkpoint that makes `prw-device-identity-provision` a validated, mechanically deployable non-activating artifact while preserving the creation-only semantics selected by C03e-US and materialized by C03e-UT.

That successor must not itself provision the real identity, run production `systemd-creds` encryption, create production `20/30/40`, reload/start/restart the service, or mutate production networking.
