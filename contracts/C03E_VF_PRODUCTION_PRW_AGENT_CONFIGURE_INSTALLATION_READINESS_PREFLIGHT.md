# C03e-VF Production prw-agent-configure Installation Readiness Preflight

Status: `READ_ONLY_PREFLIGHT — BLOCKED_ON_ARTIFACT_DELIVERY_RETENTION — VALIDATED — EVIDENCE_PENDING — NO_PRODUCTION_TRANSFER — NO_STAGING — NO_INSTALL — NO_MANAGED_CONFIGURATION_WRITE — NO_DAEMON_RELOAD — AGENT_NOT_ACTIVATED`

Date: 2026-09-18

Repository: `Gersi365/prw-executor-private`  
Repository ID: `1334911207`

## 1. Boundary

C03e-VF is a fresh read-only production installation-readiness preflight for the C03e-VE validated `prw-agent-configure` package artifact.

C03e-VF does not authorize:

- rebuilding the production artifact ad hoc;
- transferring an artifact to PowerCode;
- staging an artifact on PowerCode;
- invoking the package installer against `/`;
- executing `prw-agent-configure write` or `reconfigure-active`;
- changing managed `20`, `30`, or `40` user-systemd drop-ins;
- `systemctl --user daemon-reload`;
- Agent start/restart/enable/disable;
- credential decryption or replacement;
- enrollment, revocation, networking, DNS, forwarding, or relay mutation.

## 2. Authoritative predecessor

Evidence-closed C03e-VE / PR #695 exact identity:

- head `23d65031d051f53822f65685e2a2dc19298522df`;
- tree `a7d5e7762659dc3d11aeb348029f07e173a48a9d`;
- predecessor/base C03e-VD head `e80a5d8d2376f1e340ea66628fda41cfa362d394`.

C03e-VE validated package/deployment mechanics for `prw-agent-configure` but explicitly stopped before production artifact delivery/retention and installation.

## 3. Locked validated artifact identity

C03e-VE dedicated validation recorded:

- package `prw-agent-systemd-orchestration`;
- binary target `prw-agent-configure`;
- build command `cargo build --locked -p prw-agent-systemd-orchestration --bin prw-agent-configure`;
- validated artifact SHA-256 `35bc7623828f650f9feaa9490f1897951ac2fcf9f523d15c677934bd4797292d`;
- locked production destination `/usr/lib/private-remote-workspace/prw-agent-configure`;
- dedicated validation run `35347200726` / #1, job `105606465804`, `SUCCESS`.

The validated checksum is an artifact identity, not by itself a retained delivery object.

## 4. Retention / delivery inspection

The exact C03e-VE workflow has no artifact-upload step. Its final validation step builds and validates the package only inside the ephemeral GitHub Actions runner.

GitHub Actions artifact listing for dedicated run `35347200726` returned zero retained artifacts.

Canonical Drive evidence-folder inspection found no raw file named exactly `prw-agent-configure`.

Therefore there is no retained GitHub Actions or canonical Drive object whose bytes can be downloaded and directly proven against the locked SHA-256 before production transfer.

## 5. Fresh PowerCode destination custody proof

Read-only production-host probes were executed as user `gersi365` / uid `1000`.

Locked destination:

`/usr/lib/private-remote-workspace/prw-agent-configure`

Fresh result:

- destination absent;
- parent `/usr/lib/private-remote-workspace` is a directory;
- parent mode `0755`;
- parent owner `0:0`;
- parent is not a symlink;
- installer private temporary residue matching `.prw-agent-configure.tmp.*`: `0`.

No `prw-agent-configure` candidate exists in the canonical PRW staging area under:

`/home/gersi365/.local/state/private-remote-workspace/staging`

The filesystem installation target is therefore currently clear, but this does not solve artifact delivery/retention.

## 6. Old local workspace build outputs

Two historical workspace build outputs were found under PowerCode home storage:

1. `/home/gersi365/prw-c03e-ty/target/debug/prw-agent-configure`
   - SHA-256 `fd943286d13a98a90c9616be29a2e2df5c841d42ecd518483f0fc64c34d9033c`;
   - size `104137880` bytes;
   - workspace Git head `cadeb32f3e2229f183fb97f502889a7800d0053e`;
   - branch `phase-152-c03e-ty-managed-systemd-orchestration-source-materialization`;
   - clean worktree.

2. `/home/gersi365/prw-c03e-ua/target/debug/prw-agent-configure`
   - SHA-256 `2c11e350a898de44d227383040a70f5d8ad4604020fc56a7a12e4c1dae6300d4`;
   - size `104714064` bytes.

Neither checksum equals the C03e-VE validated artifact SHA-256 `35bc7623828f650f9feaa9490f1897951ac2fcf9f523d15c677934bd4797292d`.

These historical workspace outputs are therefore rejected as installation inputs for C03e-VF.

## 7. Readiness classification

Fresh classification:

`FILESYSTEM_INSTALL_TARGET_READY / VALIDATED_ARTIFACT_IDENTITY_KNOWN / RETAINED_EXACT_ARTIFACT_ABSENT / BLOCKED_ON_ARTIFACT_DELIVERY_RETENTION`

The production destination is suitable for a later separately authorized create-only installation transaction, but no evidence-controlled retained artifact currently exists for safe transfer and exact checksum reproof.

A rebuild in a new environment would create a new delivery transaction and requires its own source/toolchain/artifact-retention evidence. C03e-VF does not silently substitute such a rebuild for the missing retained object.

## 8. Production mutation statement

C03e-VF performed read-only host inspection only.

It did not:

- create, copy, move, stage, or install `prw-agent-configure` on PowerCode;
- modify `/usr/lib/private-remote-workspace`;
- execute the package installer;
- execute `prw-agent-configure`;
- create/replace/remove managed user-systemd configuration;
- call `daemon-reload`;
- start or restart the Agent;
- read or decrypt private credential contents;
- mutate enrollment or networking state.

## 9. STOP

C03e-VF stops at:

`STOP_BEFORE_CHECKSUM_BOUND_ARTIFACT_DELIVERY_RETENTION_MATERIALIZATION_AND_BEFORE_PRODUCTION_TRANSFER_STAGING_OR_INSTALL`

Next safe work must first materialize a checksum-bound, retained `prw-agent-configure` delivery artifact through an evidence-controlled path, without installing or executing it on PowerCode. Production transfer/staging/install remains separately gated after that artifact is available and revalidated.

`NO_RACE_FREE_CLAIM`
