# systemd Packaging Boundary

The Ubuntu PRW Agent is packaged as an unprivileged **systemd user service**. The service source is `prw-agent.service`; no socket-activation unit is part of the initial package contract.

## Locked Phase 105 paths

- packaged executable: `/usr/lib/private-remote-workspace/prw-agent`
- global user unit: `/usr/lib/systemd/user/prw-agent.service`
- repository unit source: `packaging/systemd/prw-agent.service`

The unit runs as the existing user-manager UID. It does not set `User=`, synthesize `XDG_RUNTIME_DIR`, create a working directory, or delegate PRW socket lifecycle to systemd. The Agent retains ownership of XDG validation, instance locking, bind/listen/readiness, and socket cleanup.

## Locked service policy

- `Type=exec`
- `Restart=on-failure`
- `RestartSec=5s`
- `StartLimitIntervalSec=60s`
- `StartLimitBurst=5`
- `TimeoutStopSec=15s`
- `UMask=0077`
- `NoNewPrivileges=yes`
- stdout/stderr captured by the journal
- future enablement relationship: `WantedBy=default.target`

No `ExecStop=`, shell wrapper, `.socket` unit, `Type=notify`, `RuntimeDirectory=`, `Environment=XDG_RUNTIME_DIR=...`, or `network-online.target` dependency is introduced.

## Activation gate

This repository source does **not** install, enable, start, restart, or reload the real service and does not mutate user lingering. Real-host installation/activation and any `loginctl` linger change remain separately gated after the Phase 108 completion review.
