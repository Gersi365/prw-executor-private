# C03e-VJ Production prw-agent-configure Managed Local-Only Write Transaction Completion

Status: `PRODUCTION_MANAGED_LOCAL_ONLY_WRITE_COMPLETED — CANONICAL_30_CREATED_AND_VERIFIED — MANAGED_40_ABSENT — FIXED_20_UNCHANGED — EXTERNAL_CONFIGURATION_CLEAR — VALIDATED — EVIDENCE_PENDING — NO_DAEMON_RELOAD — AGENT_NOT_ACTIVATED`

Boundary: `PRW_AGENT_CONFIGURE_PRODUCTION_MANAGED_LOCAL_ONLY_WRITE_TRANSACTION`

Authoritative predecessor: evidence-closed C03e-VI / PR #699 at exact head `3a40cfee08ef050f99b5b30c3e7f232a221c931a`.

## Authorized scope

The separately authorized production transaction was limited to the installed canonical `prw-agent-configure` `write` action with desired execution mode `local_only`.

The authorization did not include:

- `reconfigure-active`;
- `systemctl --user daemon-reload`;
- Agent start, restart or activation;
- managed remote-input creation;
- credential read, decrypt, replace or enrollment mutation;
- networking, DNS, forwarding or relay mutation;
- merge, close, ready-for-review transition or branch deletion.

## Immediate pre-mutation proof

Immediately before the write transaction, PowerCode was re-proved:

- intended uid/euid: `1000/1000` (`gersi365`);
- canonical binary `/usr/lib/private-remote-workspace/prw-agent-configure` remained a regular root-owned `0755` file;
- binary size `104941656` bytes;
- binary SHA-256 `35bc7623828f650f9feaa9490f1897951ac2fcf9f523d15c677934bd4797292d`;
- managed directory was user-owned mode `0700`;
- fixed `20-device-identity-credential.conf` was present as regular user-owned mode `0600`, size `170`;
- managed `30-agent-execution-mode.conf` was absent;
- managed `40-configured-remote-inputs.conf` was absent;
- writer temporary residue count was `0`;
- user runtime directory `/run/user/1000` was re-proved as user-owned mode `0700`;
- user service remained loaded but `failed/failed`, `MainPID=0`, `NRestarts=6`, `DropInPaths=` empty, `UnitFileState=enabled`, `NeedDaemonReload=yes`;
- exact configure executable process count `0`;
- exact Agent executable process count `0`.

A writer-semantics-equivalent read-only external-configuration scan immediately before authorization had scanned the effective user unit paths and reported zero conflicts.

## Production write transaction

The write action was invoked under a minimal explicit environment containing the intended user HOME/XDG configuration context, C locale, and exactly:

`PRW_AGENT_EXECUTION_MODE=local_only`

The action was exactly:

`/usr/lib/private-remote-workspace/prw-agent-configure write`

No `reconfigure-active` action was invoked.

The binary reported:

`prw-agent-configure event=complete action=write`

## Immediate post-write proof

Fresh post-commit verification proved:

- canonical binary unchanged, size `104941656`, SHA-256 `35bc7623828f650f9feaa9490f1897951ac2fcf9f523d15c677934bd4797292d`;
- fixed `20-device-identity-credential.conf` remained regular user-owned `0600`, size `170`, inode unchanged from the immediate pre-write proof;
- fixed `20` SHA-256 remained `42c585ea57aed19662260f0e0421139fc5e4eab973195c3b6e4e16ab7b3ecc77`;
- managed `30-agent-execution-mode.conf` became a regular user-owned `0600` file, size `58`;
- managed `30` SHA-256 `00a975b8b53ac3b4dd24e0519ab9b5bddf97c0162b28cfed52dcb6cd05673d0f`;
- exact canonical `local_only` bytes verification: PASS;
- managed `40-configured-remote-inputs.conf`: ABSENT;
- writer temporary residue count: `0`;
- parent / managed-leaf no-symlink guards: PASS;
- external configuration scan: 9 effective unit paths, 2 scanned fragments, conflict count `0`, PASS;
- exact configure executable process count `0`;
- exact Agent executable process count `0`.

The canonical managed `30` content is exactly the writer-defined local-only representation:

```text
[Service]
Environment=PRW_AGENT_EXECUTION_MODE=local_only
```

## Fresh evidence snapshot

A subsequent fresh read-only production snapshot at `2026-09-18T18:07:40+02:00` re-proved the durable post-write state:

- canonical binary: regular, `0755`, uid/gid `0:0`, size `104941656`, SHA-256 `35bc7623828f650f9feaa9490f1897951ac2fcf9f523d15c677934bd4797292d`;
- fixed `20`: regular, `0600`, uid/gid `1000:1000`, size `170`, SHA-256 `42c585ea57aed19662260f0e0421139fc5e4eab973195c3b6e4e16ab7b3ecc77`;
- managed `30`: regular, `0600`, uid/gid `1000:1000`, size `58`, SHA-256 `00a975b8b53ac3b4dd24e0519ab9b5bddf97c0162b28cfed52dcb6cd05673d0f`;
- managed `40`: ABSENT;
- writer temporary residue count `0`;
- user service `LoadState=loaded`, `ActiveState=failed`, `SubState=failed`, `MainPID=0`, `Result=exit-code`, `NRestarts=6`;
- manager-visible `DropInPaths=` remains empty;
- `UnitFileState=enabled`;
- `NeedDaemonReload=yes`;
- configure executable process count `0`;
- Agent executable process count `0`.

## Runtime boundary proof

No user-manager reload occurred during or after the write transaction.

The manager-visible `DropInPaths=` remaining empty is consistent with the newly written managed drop-in not yet being loaded by the user manager. `NeedDaemonReload=yes` remained `yes`; this checkpoint does not treat that value alone as proof of a new state transition because it was already `yes` before the write.

No Agent process was activated. No start or restart was issued.

No remote-input drop-in was created. The production configuration remains intentionally `local_only` at the filesystem state boundary.

## Git evidence scope

C03e-VJ is documentation/evidence only. It introduces no Rust source, workflow, packaging implementation, Agent unit, credential, enrollment or networking source mutation.

The evidence branch is stacked directly on the exact C03e-VI head.

## Hard boundary

C03e-VJ closes only the production managed `local_only` write transaction.

It does not authorize or perform:

- `systemctl --user daemon-reload`;
- Agent start, restart or activation;
- `prw-agent-configure reconfigure-active`;
- managed configured-remote input creation;
- credential mutation;
- enrollment, revocation, networking, DNS, forwarding or relay mutation;
- merge, close, ready-for-review transition or branch deletion.

## STOP

`STOP_BEFORE_USER_SYSTEMD_MANAGER_RELOAD_AND_BEFORE_AGENT_ACTIVATION`

Next safe boundary: a separately authorized fresh read-only preflight for the user systemd manager reload boundary. It must re-prove exact managed drop-in bytes/custody, current manager state, process quiescence and absence of unexpected external configuration, and must stop before any actual `daemon-reload` unless separately authorized.

`NO_RACE_FREE_CLAIM`
