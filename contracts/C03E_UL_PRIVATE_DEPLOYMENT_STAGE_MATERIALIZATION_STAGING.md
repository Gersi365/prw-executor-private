# C03e-UL — Private deployment-stage materialization

## 1. Status and boundary

`STAGE MATERIALIZATION — VALIDATED — PRIVILEGED EXECUTION NOT AUTHORIZED`

Boundary:

`PRIVATE_DEPLOYMENT_STAGE_MATERIALIZATION`

C03e-UL materializes only the unprivileged intended-user deployment stage selected by evidence-closed C03e-UK. It does not run sudo, does not obtain EUID 0, does not touch the installed package or vendor unit, and does not invoke the package reconciler against production.

This checkpoint separates custody-ready stage preparation from any future privileged package transaction.

## 2. Authoritative predecessor

Authoritative predecessor is evidence-closed C03e-UK / PR #675.

Exact predecessor identity:

- branch: `phase-152-c03e-uk-corrected-package-reconciliation-deployment-preflight`;
- head: `d4d45809c80bd27a62de0b3bc61819e21191c605`;
- tree: `9137f813f097826ee325826f31473041f2ee8cf7`;
- status: `PREFLIGHT — VALIDATED — EVIDENCE_RECORDED — CLOSED — PRIVATE STAGE MATERIALIZATION REQUIRED`;
- retained open, draft and unmerged.

UK selected exactly one future private stage and three fixed children with exact custody and byte identities.

## 3. Repository and duplicate guards

Canonical repository:

`Gersi365/prw-executor-private`

Repository stable ID:

`1334911207`

At UL start, canonical `main` remained head:

`a7ffafd6a6d5a032dd8290eec24df1349bade6cc`

No all-state PR titled `C03e-UL` existed before UL materialization.

No `phase-152-c03e-ul*` branch existed before UL materialization.

Exact audit-title search in the canonical Drive parent returned zero objects before UL materialization.

UL does not mutate `main`.

## 4. Intended-user execution context

Host/device:

`PowerCode`

Materialization process identity:

- real UID `1000`;
- effective UID `1000`;
- GID `1000`;
- user `gersi365`.

No sudo command was used for stage creation or validation.

No setuid helper, sudoers mutation, alternatives mutation, NOPASSWD, stored credential or root process was used.

## 5. Private parent custody before write

The selected parent chain was re-read immediately before stage creation.

- `/home`: root:root, mode `0755`, device `66306`, non-symlink;
- `/home/gersi365`: `1000:1000`, mode `0750`, device `66306`, non-symlink;
- `/home/gersi365/.local`: `1000:1000`, mode `0700`, device `66306`, non-symlink;
- `/home/gersi365/.local/state`: `1000:1000`, mode `0700`, device `66306`, non-symlink;
- `/home/gersi365/.local/state/private-remote-workspace`: `1000:1000`, mode `0700`, device `66306`, non-symlink.

Reserved stage path:

`/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage`

was absent immediately before creation.

No overwrite or reuse path was taken.

## 6. Corrected reconciler provenance before copy

Source provenance artifact:

`/tmp/prw-c03e-uk-reconciler-target/release/prw-agent-package-reconcile`

Fresh pre-write readback:

- UID/GID `1000:1000`;
- mode `0775`;
- regular file;
- non-symlink;
- bytes `2898840`;
- SHA-256 `a73a82a7fabc6113c97a48d1e61008e4c45589856be29573918026aca950114c`;
- filesystem device `40`.

The source mode and `/tmp` location remained provenance-only and were not propagated to stage custody.

The corrected reconciler build ID retained by the staged exact bytes is:

`4010d38d9ea37d1bcc2f0ac916954c75b267e82e`

## 7. Agent candidate provenance before copy

Source provenance artifact:

`/tmp/prw-c03e-uf-canonical-target/release/prw-agent`

Fresh pre-write readback:

- UID/GID `1000:1000`;
- mode `0775`;
- regular file;
- non-symlink;
- bytes `11068384`;
- SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- filesystem device `40`.

The source mode and `/tmp` location remained provenance-only and were not propagated to stage custody.

## 8. Exact deployment manifest bytes

Fixed child filename:

`C03E_UF_DEPLOYMENT_MANIFEST`

Materialized bytes are the deterministic manifest already selected by UF/UK:

- bytes `602`;
- SHA-256 `56a379041a469f95c4b778bcb3b3d8aff5fa856e76c338eb15b01288b469ace1`;
- final LF present.

The manifest binds:

- schema `c03e-uf-current-agent-package-reconciliation-v1`;
- source head `10714024a4df71bd3b5d0232bb0c6b6d7c9fb71f`;
- source tree `cd0218229280c470e8995b3341f2461afd660523`;
- Cargo.lock SHA-256 `e2d650e7a60663b651f8dfa3013eda729d1b02e763d2c3d8cf1823f43121e909`;
- canonical Agent build target `/tmp/prw-c03e-uf-canonical-target`;
- Agent candidate bytes/hash;
- installed-old Agent hash;
- vendor-unit hash;
- semantic operation `reconcile-current-agent`.

## 9. Materialization procedure

The stage was created only after all predecessor/source guards passed.

Materialization was performed by an unprivileged Python process running as UID/GID `1000:1000`.

The procedure:

1. re-proved the complete private-parent chain as non-symlink;
2. required the exact stage path to be absent;
3. opened both provenance source artifacts with no-follow semantics;
4. proved both source files regular, UID/GID `1000:1000`, and exact expected bytes/hash;
5. created the stage directory with selected mode `0700`;
6. opened the new stage as a directory with no-follow semantics;
7. created each fixed child with exclusive creation;
8. copied bytes from the already-open source descriptors;
9. set the exact selected child modes;
10. `fsync`ed each written child;
11. independently reopened and rehashed each staged child;
12. `fsync`ed the stage directory;
13. rehashed both provenance source descriptors after copy to detect source drift;
14. proved the stage contained exactly the three selected child names.

Materialization result:

`MATERIALIZATION=PASS`

No partial-failure recovery path was required.

## 10. Final stage directory identity

Stage:

`/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage`

Independent post-write readback:

- UID `1000`;
- GID `1000`;
- mode `0700`;
- directory;
- non-symlink;
- filesystem device `66306`;
- inode `6183861`.

Exact child set after materialization:

- `C03E_UF_DEPLOYMENT_MANIFEST`;
- `candidate-prw-agent`;
- `prw-agent-package-reconcile`.

No fourth or unexpected child existed.

## 11. Staged corrected reconciler identity

Path:

`/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage/prw-agent-package-reconcile`

Independent post-write readback:

- UID/GID `1000:1000`;
- mode `0500`;
- regular file;
- non-symlink;
- bytes `2898840`;
- SHA-256 `a73a82a7fabc6113c97a48d1e61008e4c45589856be29573918026aca950114c`;
- filesystem device `66306`;
- inode `6189545`;
- GNU build ID `4010d38d9ea37d1bcc2f0ac916954c75b267e82e`.

This exact staged path, not the `/tmp` provenance path, is the only reconciler path selected for any future authenticated transaction checkpoint.

## 12. Staged Agent candidate identity

Path:

`/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage/candidate-prw-agent`

Independent post-write readback:

- UID/GID `1000:1000`;
- mode `0500`;
- regular file;
- non-symlink;
- bytes `11068384`;
- SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- filesystem device `66306`;
- inode `6189549`.

The prior provenance source's `0775` mode was not propagated.

## 13. Staged deployment-manifest identity

Path:

`/home/gersi365/.local/state/private-remote-workspace/c03e-ul-package-reconciliation-stage/C03E_UF_DEPLOYMENT_MANIFEST`

Independent post-write readback:

- UID/GID `1000:1000`;
- mode `0600`;
- regular file;
- non-symlink;
- bytes `602`;
- SHA-256 `56a379041a469f95c4b778bcb3b3d8aff5fa856e76c338eb15b01288b469ace1`;
- filesystem device `66306`;
- inode `6189550`.

## 14. Source reproof after copy

After staged copies were complete, the already-open provenance source descriptors were independently rehashed before the materialization process returned success.

Corrected reconciler source remained:

- bytes `2898840`;
- SHA-256 `a73a82a7fabc6113c97a48d1e61008e4c45589856be29573918026aca950114c`.

Agent candidate source remained:

- bytes `11068384`;
- SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`.

A later readback again observed both provenance artifacts at the same exact hashes.

## 15. Production package non-mutation proof

Immediately after stage materialization, fixed production objects were independently read back.

Installed Agent:

`/usr/lib/private-remote-workspace/prw-agent`

remained:

- root:root;
- mode `0755`;
- bytes `2865776`;
- SHA-256 `4dbb114edc5ad131dd8abcf2ba798a413a249f2a3931a43b59f67b496e0d242e`.

Vendor unit:

`/usr/lib/systemd/user/prw-agent.service`

remained:

- root:root;
- mode `0644`;
- bytes `332`;
- SHA-256 `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`.

Active sudo-rs binary remained:

- root:root;
- mode `4755`;
- bytes `1090848`;
- SHA-256 `2eb5d31f91a12d75a2a05b54b7f79775e5565898af0b3263414fcabaad922afb`.

No root package byte was changed by UL.

## 16. Service-state non-mutation proof

Read-only user-manager state immediately after stage materialization remained:

- `LoadState=loaded`;
- `ActiveState=failed`;
- `SubState=failed`;
- `MainPID=0`;
- `FragmentPath=/usr/lib/systemd/user/prw-agent.service`;
- `DropInPaths=` empty;
- `UnitFileState=enabled`;
- `Result=exit-code`.

No exact `prw-agent` process was found by process-name probe during UL readback.

No manager command changed the service.

Enabled + linger remains an external activation race surface inherited from UK.

Therefore:

`NO_RACE_FREE_CLAIM`

remains mandatory.

## 17. Root staging residue

No `.prw-agent.c03e-ug.*.candidate` root reconciliation sibling was observed in the fixed Agent parent after UL materialization.

UL created no object beneath `/usr/lib/private-remote-workspace`.

## 18. Privilege boundary retained

UL did not execute the staged reconciler.

UL did not authenticate sudo.

UL did not obtain real or effective UID 0.

UL did not dynamically exercise the sudo-rs-injected `SUDO_UID/SUDO_GID/SUDO_USER` production path.

The corrected source's selected identity law remains evidence-closed from UI/UJ, while a real privileged transaction remains separately gated.

## 19. Canonical UL classification

`EXACT_UK_PREDECESSOR / UNPRIVILEGED_UID1000_GID1000_MATERIALIZATION / PRIVATE_STAGE_0700 / STAGED_RECONCILER_0500_A73A82A7 / STAGED_AGENT_0500_9DB768C1 / STAGED_MANIFEST_0600_56A37904 / EXACT_THREE_CHILD_SET / FSYNC_AND_INDEPENDENT_READBACK / SOURCE_REPROOF_AFTER_COPY / PRODUCTION_AGENT_UNCHANGED_4DBB114E / VENDOR_UNIT_UNCHANGED_24F646DC / SUDO_BINARY_UNCHANGED_2EB5D31F / SERVICE_FAILED_FAILED_MAINPID0 / NO_ROOT_STAGING_RESIDUE / NO_SUDO / NO_PRIVILEGED_EXECUTION / NO_PACKAGE_TRANSACTION / NO_RACE_FREE_CLAIM`

## 20. Explicit non-actions / STOP

C03e-UL performed no:

- Rust/source/Cargo mutation;
- arbitrary path creation outside the selected private stage;
- stage overwrite/reuse;
- sudo invocation;
- sudo authentication;
- sudoers mutation;
- sudo alternatives mutation;
- NOPASSWD or stored-password configuration;
- root-owned staging creation;
- installed Agent replacement;
- vendor-unit rewrite;
- systemd manager mutation;
- service start/stop/restart/try-restart/reload;
- enable/disable mutation;
- linger mutation;
- identity/20/30/40 mutation;
- network/listener/database/auth/control-plane mutation;
- repository main mutation;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- history rewrite;
- destructive evidence cleanup.

STOP after evidence closure of this private-stage materialization checkpoint.

The next safe boundary is a separately authorized read-only final transaction-readiness checkpoint. It must re-prove the exact staged tree, active sudo-rs identity, fixed installed package/unit identities, absence of root reconciliation residue, and immediate non-running service state before any authenticated sudo command is even handed off or considered for execution.

C03e-UL does not authorize that future privileged transaction.