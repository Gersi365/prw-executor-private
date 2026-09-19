# C03e-VP — Production Desktop Debian Artifact Transfer / Staging Prepared Boundary

Status: `TRANSFER_READY — BLOCKED_ON_POWERCODE_ONLINE — NO_TRANSFER_EXECUTED`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## 1. Exact predecessor

Evidence-closed C03e-VO is the exact predecessor:

- branch: `phase-152-c03e-vo-desktop-deb-packaging-source-materialization`;
- exact head: `643f9bc942e94dbaad54050461f104117e08c2c3`;
- exact tree: `3d39fa70945b9e8d6a0922ed41f6a6783ef2e9e1`;
- PR #705: draft/open/unmerged;
- canonical Drive evidence ID: `1v16KNsVTA5TKR9iogUFZegSprQ4_BV_t`.

C03e-VO closed at:

`STOP_AFTER_VALIDATED_CANDIDATE_DEB_ARTIFACT_AND_BEFORE_ANY_PRODUCTION_TRANSFER_INSTALL_OR_LAUNCH`

VP is therefore limited to exact artifact transfer and user-owned staging only.

## 2. Exact candidate authority

Only this package is eligible:

- filename: `private-remote-workspace-desktop_0.1.0-1_amd64.deb`;
- size: `418670` bytes;
- SHA-256:
  `2f29a60187a00101e641499aea68b88abc6ca740bae39d9fc574eb31e5110c71`.

Exact release-binary SHA-256 inside the package:

`1adb489772c54b98996845f1ecca1e3a77e47e4cc8f215535f0afb9159fc26af`

Authoritative GitHub Actions artifact:

- workflow run: `35394772792`;
- artifact ID: `10567202652`;
- artifact name:
  `c03e-vo-private-remote-workspace-desktop-0.1.0-1-amd64-643f9bc942e94dbaad54050461f104117e08c2c3`;
- archive digest:
  `sha256:a324cdfd1350aaed896e1cfedddb1728f024d578e90a105e17815afd09b54402`.

No other build, file, hash, artifact ID, workflow run, local rebuild, or inferred equivalent
may be promoted to VP staging authority.

## 3. Selected staging precedent

Historical PRW production staging uses the non-privileged user XDG state tree with:

- user ownership;
- mode `0700` staging directory;
- exact content hashes;
- privileged installation deferred to a later transaction.

VP follows that precedent and does not stage directly into `/usr`.

## 4. Exact selected staging path

When PowerCode is online, the only authorized VP stage path is:

`/home/gersi365/.local/state/private-remote-workspace/phase152/c03e-vp/staging/643f9bc942e94dbaad54050461f104117e08c2c3`

The transaction is create-only.

If that exact directory already exists, VP must fail closed and reconcile it read-only.
It must not overwrite, merge, clean, replace, or silently reuse a pre-existing stage.

Selected directory state after success:

- type: directory;
- owner: uid/gid `1000:1000`;
- mode: `0700`.

## 5. Exact staged files

VP may create exactly three regular files under the exact staging directory:

1. `private-remote-workspace-desktop_0.1.0-1_amd64.deb`
   - uid/gid `1000:1000`;
   - mode `0600`;
   - size `418670`;
   - SHA-256
     `2f29a60187a00101e641499aea68b88abc6ca740bae39d9fc574eb31e5110c71`.

2. `SHA256SUMS`
   - uid/gid `1000:1000`;
   - mode `0600`;
   - contains only the exact package hash binding.

3. `ARTIFACT-MANIFEST`
   - uid/gid `1000:1000`;
   - mode `0600`;
   - records checkpoint, source head/tree, workflow run, artifact ID/digest,
     package filename/size/hash, and:
     `install_authorized=no`,
     `launch_authorized=no`.

No symlink is permitted for the stage directory or any staged file.

## 6. Selected transport mechanism

The prepared transaction uses the existing PowerCode GitHub self-hosted runner identity
`[self-hosted, prw-host]` only.

The workflow is manual-dispatch only:

`.github/workflows/phase-152-c03e-vp-production-desktop-deb-artifact-staging.yml`

It uses `actions/download-artifact@v4` with:

- exact repository;
- exact source run `35394772792`;
- exact artifact name;
- repository-scoped GitHub token with read-only contents/actions permission.

The runner must not rebuild the package.

The downloaded exact `.deb` must pass size and SHA-256 validation before any stage
directory is created.

## 7. Exact host preconditions

Before staging, the workflow must prove:

- hostname exactly `PowerCode`;
- user exactly `gersi365`;
- uid/gid exactly `1000:1000`;
- HOME exactly `/home/gersi365`;
- source `.deb` is a regular non-symlink file with exact size/hash;
- `prw-agent.service` is active;
- manager-visible `PRW_AGENT_EXECUTION_MODE=local_only`;
- managed `40-configured-remote-inputs.conf` absent;
- installed desktop binary absent;
- installed desktop launcher absent;
- exact stage directory absent.

Any failed precondition blocks the transaction before staging.

## 8. Postconditions

After staging, VP must prove:

- stage directory exact owner/mode;
- all three staged files exact type/owner/mode;
- staged `.deb` exact size/hash;
- `sha256sum --check --strict SHA256SUMS` passes;
- Agent remains active;
- Agent remains manager-visible `local_only`;
- managed `40` remains absent;
- installed desktop binary remains absent;
- installed desktop launcher remains absent.

No package manager command is part of VP.

## 9. Explicitly forbidden actions

VP must not:

- run `apt`, `apt-get`, `dpkg -i`, `gdebi`, or equivalent install commands;
- write `/usr/lib/private-remote-workspace/prw-desktop`;
- write `/usr/share/applications/io.patchmirror.prw.desktop.desktop`;
- launch `prw-desktop`;
- change Agent configuration;
- write managed `30` or `40`;
- reload/restart/start/stop the Agent;
- change credentials;
- activate command-3;
- select configured-remote values;
- activate TCP/UDP listeners;
- change DNS/firewall/routes;
- use sudo;
- modify sudoers;
- merge or mark predecessor PRs ready;
- overwrite or clean a pre-existing VP stage.

## 10. Current environmental blocker

Fresh connector inventory on 2026-09-19 reported:

- device: `PowerCode`;
- device ID: `a43dde76-22d5-4efa-b388-38d04ee9635d`;
- status: `offline`;
- auth token: valid;
- last seen: `2026-09-19T01:16:44.192+00:00`.

Therefore the real VP transfer/staging transaction cannot execute now.

No attempt is made to pretend the host is reachable, to substitute a different machine,
or to claim staged state from historical evidence.

## 11. Prepared classification

Until a later authorized continuation finds PowerCode online and executes the exact
manual-dispatch staging workflow successfully, VP remains:

`PRODUCTION_DESKTOP_DEB_ARTIFACT_TRANSFER_STAGING_PREPARED / EXACT_CANDIDATE_AND_STAGE_PATH_BOUND / CREATE_ONLY_STAGE_LAW_BOUND / POWERCODE_OFFLINE / TRANSFER_NOT_EXECUTED / NO_INSTALL / NO_LAUNCH / NO_AGENT_MUTATION / NO_CONFIGURED_REMOTE / NO_NETWORK_MUTATION / NO_RACE_FREE_CLAIM`

## STOP

`STOP_BLOCKED_ON_POWERCODE_ONLINE_BEFORE_EXACT_ARTIFACT_TRANSFER_AND_STAGING`

When PowerCode is online again, the next continuation may execute only the already-bound
VP staging workflow. It must stop again after exact staged-byte verification and before
any package installation or desktop launch.

`NO_RACE_FREE_CLAIM`


## 12. Forward-only transport correction after host reconnection

After this preparation checkpoint was recorded, PowerCode was brought online through the
already authorized Remote Desktop Commander connection. Fresh preflight proved the exact
host/user/uid/gid, Agent active/local_only, managed 40 absent, desktop install destinations
absent, and the exact VP stage absent.

The connected GitHub tool surface available to this chat does not expose a
`workflow_dispatch` mutation, and PowerCode does not have an authenticated `gh` CLI.
No GitHub token, runner secret, stored credential, sudo grant, or manual user token may be
read, copied, fabricated, or introduced merely to dispatch the prepared workflow.

Therefore VP additionally binds one equivalent direct connected transport path, with no
change to candidate authority, stage path, stage payload, install boundary, or
postconditions:

1. the exact authoritative GitHub Actions artifact ID `10567202652` is downloaded through
   the connected GitHub application;
2. its exact archive digest must equal
   `a324cdfd1350aaed896e1cfedddb1728f024d578e90a105e17815afd09b54402`;
3. the resulting short-lived authenticated artifact URL may be consumed only by the
   authorized PowerCode Remote Desktop Commander session;
4. PowerCode downloads that archive into a temporary user-owned directory;
5. the archive digest is verified before extraction;
6. only `private-remote-workspace-desktop_0.1.0-1_amd64.deb` is selected for staging;
7. that .deb must equal size `418670` and SHA-256
   `2f29a60187a00101e641499aea68b88abc6ca740bae39d9fc574eb31e5110c71`;
8. the exact create-only staging law from Sections 4–8 is then executed unchanged;
9. all temporary transfer/extraction material is removed after successful staged-byte
   verification;
10. the transaction stops before any package-manager install or desktop launch.

The direct connected transport must fail closed before staging if any archive digest,
package size/hash, host identity, Agent/local_only state, managed-40 state, install
destination, or stage-absence precondition differs.

This correction is a transport-channel substitution only. It does not authorize an
alternate artifact, package rebuild, package install, desktop launch, Agent mutation,
configured-remote selection, networking mutation, or cleanup/overwrite of a pre-existing
stage.

The prepared GitHub workflow remains valid but need not be dispatched when this exact
direct connected transport path is used.

The corrected execution boundary becomes:

`STOP_AFTER_EXACT_ARTIFACT_TRANSFER_AND_CREATE_ONLY_STAGING_VERIFICATION_AND_BEFORE_ANY_PACKAGE_INSTALL_OR_DESKTOP_LAUNCH`

`NO_RACE_FREE_CLAIM`
