# C03e-VH Production prw-agent-configure Artifact Consumption / Installation Readiness Preflight

Status: `READ_ONLY_PREFLIGHT_READY_FOR_SEPARATELY_AUTHORIZED_PRODUCTION_ARTIFACT_TRANSFER_STAGING_AND_INSTALL_TRANSACTION`

This checkpoint is read-only with respect to PowerCode. It re-proves the retained C03e-VG artifact, production destination custody, and intended production staging custody. It does not transfer, stage, install, execute, configure, reload, or activate anything on the production host.

## Authoritative predecessor

Evidence-closed C03e-VG / PR #697 at exact head:

`72578671d422cf77f9993a07b7ade6e8e09a78e7`

Exact predecessor tree:

`4d227824a292450996935980ca22e3220eabdfcb`

C03e-VG retained artifact ID `10549981416` and stopped before production artifact transfer, staging or installation.

## Fresh retained-artifact reproof

GitHub Actions artifact:

- ID: `10549981416`;
- name: `c03e-vg-prw-agent-configure-72578671d422cf77f9993a07b7ade6e8e09a78e7`;
- source run: `35352124227`;
- source head: `72578671d422cf77f9993a07b7ade6e8e09a78e7`;
- provider size: `104942920` bytes;
- provider archive SHA-256: `1efad139b11b8df70b57d860b3ed0427e16607c781b252d464706f0719f08a9a`;
- retained binary SHA-256: `35bc7623828f650f9feaa9490f1897951ac2fcf9f523d15c677934bd4797292d`;
- `expired=false` at preflight;
- expires `2026-12-17T13:45:48Z`.

The artifact was freshly downloaded outside the Actions job during C03e-VH preflight. Independent re-verification proved:

- downloaded ZIP size exactly `104942920` bytes;
- downloaded ZIP SHA-256 exactly `1efad139b11b8df70b57d860b3ed0427e16607c781b252d464706f0719f08a9a`;
- archive contains exactly three root objects;
- symlink count is zero;
- `prw-agent-configure` is a regular file, size `104941656`, mode `0755`;
- retained binary SHA-256 exactly `35bc7623828f650f9feaa9490f1897951ac2fcf9f523d15c677934bd4797292d`;
- `SHA256SUMS` binds that exact binary filename and digest;
- manifest `source_git_sha` equals exact C03e-VG head;
- manifest `future_production_destination` equals `/usr/lib/private-remote-workspace/prw-agent-configure`;
- all production mutation markers remain `NOT_AUTHORIZED`.

The fresh artifact bytes were downloaded only into the audit/runtime environment. They were not transferred to PowerCode.

## Permanent installer contract

Authoritative installer contract at the C03e-VG lineage:

`packaging/systemd/AGENT_CONFIGURE_INSTALL_TRANSACTION.md`

Locked production destination:

`/usr/lib/private-remote-workspace/prw-agent-configure`

A later separately authorized real-root transaction must use the permanent installer `packaging/systemd/install-prw-agent-configure.sh`, explicit `--allow-root-filesystem`, and uid 0. The installer is create-only: destination must be absent; publication is same-filesystem atomic no-replace; final file must be regular, non-symlink, mode `0755`, root-owned, and byte-identical to the retained artifact. The installer never executes `prw-agent-configure` and never performs managed configuration or systemd activation.

## Fresh PowerCode destination custody proof

Read-only host: `PowerCode`.

Execution identity:

- uid `1000`;
- user `gersi365`.

Production destination:

`/usr/lib/private-remote-workspace/prw-agent-configure`

Fresh result:

- destination: `ABSENT`;
- parent `/usr/lib/private-remote-workspace`: directory, mode `0755`, uid/gid `0:0`;
- parent is not a symlink;
- `.prw-agent-configure.tmp.*` installer residue count: `0`;
- filesystem available space at `/`: `19919440` KiB at the sampled preflight point.

No destination or parent mutation was performed.

## Fresh production staging custody proof

Canonical staging root used for PRW production artifact transactions:

`/home/gersi365/.local/state/private-remote-workspace/staging`

Fresh result:

- staging root exists as a directory;
- mode `0700`;
- uid/gid `1000:1000`;
- staging root is not a symlink;
- intended future VH staging target `/home/gersi365/.local/state/private-remote-workspace/staging/c03e-vh-10549981416` is `ABSENT`;
- no relevant `*agent-configure*` or `c03e-vh-*` staged entry was present.

C03e-VH did not create the intended staging target and did not transfer any artifact bytes to PowerCode.

## Readiness classification

Fresh C03e-VH classification:

`RETAINED_ARTIFACT_EXISTS_AND_UNEXPIRED / FRESH_DOWNLOAD_AND_ARCHIVE_DIGEST_VERIFIED / BINARY_CHECKSUM_AND_MANIFEST_VERIFIED / PRODUCTION_DESTINATION_ABSENT_AND_CUSTODY_READY / STAGING_ROOT_CUSTODY_READY / INTENDED_STAGING_TARGET_ABSENT / READY_FOR_SEPARATELY_AUTHORIZED_TRANSFER_STAGING_AND_CREATE_ONLY_INSTALL_TRANSACTION`

This readiness statement does not authorize the next transaction by itself.

## Hard prohibitions

C03e-VH must not:

- transfer artifact bytes to PowerCode;
- create the intended staging target;
- stage or extract the artifact on PowerCode;
- execute the real-root package installer;
- install or execute `prw-agent-configure`;
- execute `write` or `reconfigure-active`;
- create, replace, or remove managed `20`, `30`, or `40` user-systemd drop-ins;
- call production `systemctl`, `loginctl`, or `daemon-reload`;
- activate the Agent;
- read/decrypt/replace private credentials;
- mutate enrollment, revocation, networking, DNS, forwarding, or relay state;
- merge, close, or mark the PR ready without separate authorization.

## STOP

`STOP_BEFORE_PRODUCTION_ARTIFACT_TRANSFER_STAGING_AND_CREATE_ONLY_REAL_ROOT_INSTALL`

The next safe boundary is a separately authorized transaction that first performs one final artifact/destination/staging reproof, then transfers the exact evidence-closed artifact to the absent controlled staging target, re-verifies the archive and binary SHA-256 on PowerCode, and only then performs the create-only real-root install under the permanent installer contract. It must stop before any `prw-agent-configure write`, daemon reload, or Agent activation.

`NO_RACE_FREE_CLAIM`
