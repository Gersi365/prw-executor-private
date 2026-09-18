# C03e-VL Production First Agent Activation Transaction Completion

Status: `PRODUCTION_FIRST_AGENT_ACTIVATION_COMPLETED — LOCAL_ONLY_AGENT_ACTIVE — DEVICE_IDENTITY_LOADED — LOCAL_UNIX_SOCKET_READY — NO_TCP_UDP_NETWORK_ACTIVITY_OBSERVED — VALIDATED — EVIDENCE_PENDING`

Boundary: `PRW_PRODUCTION_FIRST_AGENT_ACTIVATION_TRANSACTION`

Authoritative predecessor: evidence-closed C03e-VK / PR #701 at exact head `14621d529386dc9443504dea9c1ca456a6555f73`.

## Authorized scope

The separately authorized production mutation was limited to first activation of the already manager-loaded `local_only` Agent using exactly one manual user-systemd start request.

The authorization did not include:

- Agent restart;
- `systemctl --user reset-failed`;
- `prw-agent-configure reconfigure-active`;
- managed `40-configured-remote-inputs.conf` creation or any configured-remote mutation;
- credential replacement, enrollment mutation or manual credential-content read/decrypt;
- networking, DNS, forwarding or relay mutation;
- merge, close, ready-for-review transition or branch deletion.

## Immediate pre-activation proof

Fresh production checks immediately before activation proved:

- `/usr/lib/private-remote-workspace/prw-agent`: regular root-owned `0755`, size `11068384`, SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- fixed `20-device-identity-credential.conf`: regular user-owned `0600`, size `170`, SHA-256 `42c585ea57aed19662260f0e0421139fc5e4eab973195c3b6e4e16ab7b3ecc77`;
- managed `30-agent-execution-mode.conf`: regular user-owned `0600`, size `58`, SHA-256 `00a975b8b53ac3b4dd24e0519ab9b5bddf97c0162b28cfed52dcb6cd05673d0f`;
- managed `40-configured-remote-inputs.conf`: ABSENT;
- writer temporary residue count: `0`;
- intended user runtime boundary `/run/user/1000`: user-owned `0700`;
- service `LoadState=loaded`, `ActiveState=failed`, `SubState=failed`, `MainPID=0`, `Result=exit-code`, `NRestarts=6`;
- manager-visible `DropInPaths=` exactly fixed `20` plus managed `30`;
- `UnitFileState=enabled`;
- `NeedDaemonReload=no`;
- manager-visible service environment exactly contained `PRW_AGENT_EXECUTION_MODE=local_only`;
- user-manager queued job list was empty;
- Agent executable process count `0`;
- configure executable process count `0`;
- `systemd-analyze --user verify prw-agent.service` returned `0`.

The immediately preceding read-only first-activation preflight had also re-proved 9 effective user-unit paths, two writer-relevant external fragments and zero selected external PRW configuration conflicts.

## Service activation semantics guard

Before activation, user-systemd reported:

- `Type=exec`;
- `ExecStart=/usr/lib/private-remote-workspace/prw-agent`;
- `Restart=on-failure`;
- `RestartUSec=5s`;
- `StartLimitBurst=5`;
- `StartLimitIntervalUSec=1min`;
- `TimeoutStartUSec=1min 30s`.

Only one manual start request was issued. Any retry would therefore have been systemd policy behavior, not an additional manual restart request.

## Production first activation transaction

The exact production mutation was:

`systemctl --user start prw-agent.service`

Transaction timing:

- begin `2026-09-18T18:59:36+02:00`;
- return code `0`;
- command return `2026-09-18T18:59:38+02:00`.

No manual restart, `reset-failed`, `reconfigure-active`, managed-file write, credential replacement, enrollment mutation or networking mutation was invoked.

## Immediate activation proof

At `2026-09-18T18:59:50+02:00`, fresh read-only verification proved:

- `LoadState=loaded`;
- `ActiveState=active`;
- `SubState=running`;
- `MainPID=3197123`;
- `Result=success`;
- `NRestarts=0`;
- `ExecMainCode=0`;
- `ExecMainStatus=0`;
- `NeedDaemonReload=no`;
- manager-visible `DropInPaths=` remained exactly fixed `20` plus managed `30`;
- manager-visible environment remained `PRW_AGENT_EXECUTION_MODE=local_only`;
- Agent executable process count became exactly `1`;
- configure executable process count remained `0`;
- user-manager queued job list remained empty.

The activation journal contained only the expected startup sequence observed in this transaction:

- user-systemd starting the service;
- user-systemd reporting the service started;
- `prw-agent event=device_identity_loaded` with public SPKI SHA-256 `b417e6b0964cd933209bf9ccefcde3b4ee4caafe96dcdbe29278b008d499ce8d`.

No private credential bytes were read, printed or recorded by this audit.

## Runtime containment proof

Fresh read-only runtime inspection at `2026-09-18T19:00:21+02:00` proved:

- PID `3197123` executable resolved exactly to `/usr/lib/private-remote-workspace/prw-agent`;
- service remained `active/running`, `Result=success`, `NRestarts=0`;
- one Unix-domain listener belonged to the Agent at `agent.sock`;
- no Agent TCP listener was observed;
- no Agent UDP listener was observed;
- user-systemd credential runtime directory existed user-owned with mode `0500`;
- runtime-delivered credential file `prw.device-identity.private-key.v1` existed user-owned with mode `0400`;
- the credential file contents were not read;
- no user-manager job was queued.

At `2026-09-18T19:00:40+02:00`, the local Agent runtime boundary was additionally proved:

- `/run/user/1000/private-remote-workspace`: directory, user-owned `0700`;
- `agent.lock`: regular user-owned `0600`;
- `agent.sock`: Unix socket, user-owned `0600`;
- service still `active/running` with the same MainPID and `NRestarts=0`.

## Closure snapshot

Fresh read-only production snapshot at `2026-09-18T19:01:31+02:00` re-proved:

- fixed `20` unchanged with exact SHA-256 `42c585ea57aed19662260f0e0421139fc5e4eab973195c3b6e4e16ab7b3ecc77`;
- managed `30` unchanged with exact SHA-256 `00a975b8b53ac3b4dd24e0519ab9b5bddf97c0162b28cfed52dcb6cd05673d0f`;
- managed `40` absent;
- writer temporary residue count `0`;
- `LoadState=loaded`, `ActiveState=active`, `SubState=running`;
- `MainPID=3197123`;
- `Result=success`;
- `NRestarts=0`;
- `NeedDaemonReload=no`;
- `UnitFileState=enabled`;
- manager-visible `DropInPaths=` remained exactly fixed `20` plus managed `30`;
- manager-visible service environment remained `PRW_AGENT_EXECUTION_MODE=local_only`;
- exactly one Agent executable process and zero configure executable processes;
- no TCP connection belonging to the Agent was observed;
- no UDP connection belonging to the Agent was observed;
- the only observed listener belonging to the Agent remained the local Unix socket;
- runtime directory, lock and socket remained user-owned with modes `0700`, `0600` and `0600` respectively;
- user-manager queued job list remained empty.

A subsequent writer-style external configuration guard re-proved:

- effective user-unit path count `9`;
- writer-relevant external fragment count `2`;
- external conflict count `0`;
- external configuration PASS;
- manager-global `PRW_*` environment count `0`;
- service remained active with `MainPID=3197123`, `Result=success`, `NRestarts=0`.

## Local-only boundary proof

The manager-loaded execution mode remained `local_only` throughout first activation.

Managed configured-remote input drop-in `40-configured-remote-inputs.conf` remained absent before and after activation.

No Agent TCP/UDP listener or connection was observed during post-start containment checks. The active runtime surface observed by this audit was the local Unix-domain `agent.sock` only.

This evidence does not claim exhaustive or race-free proof that no network syscall could occur; it records the concrete read-only observations above.

## Git evidence scope

C03e-VL is documentation/evidence only. It introduces no Rust source, workflow, packaging implementation, systemd unit, credential, enrollment or networking source mutation.

The evidence branch is stacked directly on the exact C03e-VK head.

## Hard boundary

C03e-VL closes only the production first Agent activation transaction in canonical `local_only` mode.

It does not authorize or perform:

- Agent restart;
- `prw-agent-configure reconfigure-active`;
- managed configured-remote input creation;
- execution-mode transition to `configured_remote`;
- credential mutation or enrollment/revocation mutation;
- networking, DNS, forwarding or relay mutation;
- merge, close, ready-for-review transition or branch deletion.

## STOP

`STOP_BEFORE_ANY_CONFIGURED_REMOTE_INPUT_WRITE_OR_RECONFIGURE_ACTIVE_OR_NETWORKING_MUTATION`

Next safe boundary: a separately authorized fresh read-only durability/readiness preflight of the active local-only Agent. It should re-prove process stability, local Unix-socket custody, canonical manager-loaded configuration and continued absence of configured-remote inputs or unexpected network activity before any later remote-configuration proposal.

`NO_RACE_FREE_CLAIM`
