# systemd Packaging Boundary

The Ubuntu PRW Agent is intended to run as a headless automatically started service.

Phase 001 does not install, enable, start, stop, or modify systemd units.

The final service architecture must be designed only after the privilege model is locked.

Preferred direction:

- an unprivileged long-running PRW Agent;
- a narrowly scoped privileged helper only where operating-system networking or similarly privileged operations require it;
- authenticated local IPC between privilege domains.

A future packaging phase must validate restart policy, startup ordering, filesystem permissions, secret storage, logs, and rollback behavior before activation.
