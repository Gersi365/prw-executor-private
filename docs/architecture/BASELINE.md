# Private Remote Workspace Architecture Baseline

Status: initial baseline

## System view

```text
Android Client
      |
      |  PRW application protocol over future encrypted mesh
      |
Ubuntu PRW Agent
      |
      +-- terminal capability
      +-- file capability
      +-- forwarding capability
      +-- policy enforcement
      +-- network capability
```

The Ubuntu graphical client is separate from the Agent:

```text
PRW Desktop UI
      |
      | authenticated local IPC
      v
PRW Agent
```

## Planned connectivity hierarchy

```text
local direct
    |
internet direct
    |
encrypted relay fallback
```

No one connectivity strategy is implemented in Phase 001.

## Planned identity hierarchy

```text
User Identity
      |
Device Identity
      |
Network / Transport Identity
```

The three identities have separate lifecycle concerns.

## Planned networking building blocks

The final implementation may use audited standard protocol/library components such as:

- WireGuard-compatible encrypted tunneling;
- SSH;
- TLS;
- STUN;
- ICE concepts;
- TURN/relay concepts;
- TUN interfaces;
- standard DNS.

These names describe architecture candidates, not Phase 001 runtime dependencies.

## DNS

Private DNS is optional.

Future design must support:

- automatic PRW device names;
- custom resolver addresses;
- split DNS;
- recovery when DNS is misconfigured.

## Agent privilege boundary

Preferred direction:

```text
Unprivileged PRW Agent
          |
          | narrow authenticated IPC
          v
Privileged helper
```

A privileged helper is not implemented or installed in Phase 001.

## Remote files

File management is an application-level capability rather than an alias for unrestricted shell execution.

Transfers should eventually support:

- resumability;
- integrity verification;
- temporary destination files;
- atomic finalization;
- safe cross-device move semantics.

## Non-goals for this phase

Phase 001 does not:

- install services;
- configure systemd;
- configure DNS;
- create TUN interfaces;
- open network listeners;
- connect to Google Drive;
- run rclone;
- create accounts;
- perform authentication;
- modify databases;
- deploy infrastructure.
