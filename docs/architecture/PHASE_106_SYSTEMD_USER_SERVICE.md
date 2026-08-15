# Phase 106 — systemd User-Service Source

Status: `SOURCE_INTEGRATED / STATIC_VALIDATION_PASS / AWAITING_PERMANENT_RUST_CI`

## Locked predecessor

Phase 105 contract-lock commit:

`a2884334f3e7dbca1eb5e0b7448ead66f550c6fe`

## Integrated source

Service source commit:

`007e5adbc64ed874109787971e3127abad8a5a26`

Temporary-workflow cleanup commit:

`eabc15033719cecb60cdab7c99efc7d39d8a8b0d`

Repository service source:

`packaging/systemd/prw-agent.service`

The unit is the Phase 105 locked systemd **user-service** contract and uses the package-owned executable path `/usr/lib/private-remote-workspace/prw-agent`.

## Static validation

Initial Phase 106 workflow run `31906284775` passed the exact service contract and forbidden-surface checks, then stopped safely before integration because `systemd-analyze --user --root=... verify` cannot initialize user-scope unit search paths in the disposable root on the Ubuntu 24.04/systemd 255 runner.

No service source was committed by that failed run.

Corrective run `31906322126` changed only the validation harness:

- preserved the exact service source;
- preserved the exact real `ExecStart=` path in the repository contract;
- projected only `ExecStart=/usr/bin/true` into a disposable copy for user-scope parser verification;
- kept the real `ExecStart=` path under byte-for-byte contract validation;
- corrected the Git mutation guard to distinguish the tracked README replacement from the new untracked service file.

A01 passed:

- Ubuntu 24.04 runner baseline;
- systemd 255 baseline gate;
- exact unit contract;
- forbidden directive/shell/privilege surface;
- `systemd-analyze --user verify` syntax validation on the projection;
- exact repository mutation boundary;
- `git diff --check`.

Only after that PASS were the service source and README committed, followed by removal of both temporary Phase 106 workflows.

## Activation boundary

Phase 106 performs no real install and no service-manager mutation.

Still prohibited:

- real-host copy to `/usr/lib/...`;
- `systemctl --user daemon-reload`;
- enable/start/restart/stop;
- enablement symlink creation;
- `loginctl enable-linger` or `disable-linger`;
- deployment.

Phase 107 remains limited to install/upgrade/remove transaction design and disposable-root validation.
