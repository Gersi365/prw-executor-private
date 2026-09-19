# C03e-VR — First Desktop Launch and Read-Only Local Agent Connection Verification

Status: `FIRST_LAUNCH_PREPARED — NO_LAUNCH_EXECUTED`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## 1. Exact predecessor

Evidence-closed C03e-VQ is the exact predecessor:

- branch: `phase-152-c03e-vq-production-desktop-deb-exact-install-transaction`;
- exact head: `7cef99ff78ab11582158201f8fcb2e13b5701161`;
- exact tree: `704decad6d1f6b8ca851b4f1453d50daf85e6810`;
- PR #707: draft/open/unmerged;
- canonical completion evidence ID:
  `1g4MSQkW3QAtgS-QLVhSmma0in1Y6jQGV`.

VQ closed after exact package installation and post-install verification and explicitly
stopped before first desktop application launch.

## 2. Exact installed launch authority

Only this executable may be launched:

`/usr/lib/private-remote-workspace/prw-desktop`

Required pre-launch identity:

- dpkg package `private-remote-workspace-desktop`;
- status `install ok installed`;
- version `0.1.0-1`;
- architecture `amd64`;
- executable regular/non-symlink;
- owner `root:root`;
- mode `0755`;
- size `1373600`;
- SHA-256:
  `1adb489772c54b98996845f1ecca1e3a77e47e4cc8f215535f0afb9159fc26af`.

Launcher remains:

`/usr/share/applications/io.patchmirror.prw.desktop.desktop`

with direct:
`Exec=/usr/lib/private-remote-workspace/prw-desktop`.

## 3. Current desktop runtime semantics

At exact VQ source authority, the desktop startup path:

- presents the GTK/libadwaita window;
- renders a deterministic Overview page;
- starts one bounded worker named `prw-desktop-readonly-agent-probe`;
- derives the Agent endpoint only from `XDG_RUNTIME_DIR`;
- validates runtime-root, PRW runtime-directory and Agent-socket ownership/type/modes;
- uses exact socket:
  `/run/user/1000/private-remote-workspace/agent.sock`;
- performs only `GetAgentStatus` and `GetPrivateDnsConfig`;
- uses 2-second read/write timeouts;
- validates protocol response framing and request IDs;
- presents Agent Availability=Online only after a valid status response.

No command-3 management request is emitted by the startup probe.

All non-Overview navigation destinations remain Phase-151 placeholders and imply no
capability.

## 4. Graphical-session preconditions

Immediately before launch, prove:

- PowerCode online;
- user `gersi365`, uid/gid `1000:1000`;
- installed package exactness from Section 2;
- desktop exact executable process count `0`;
- Agent `active/running`, `Result=success`, `NRestarts=0`;
- manager-visible `PRW_AGENT_EXECUTION_MODE=local_only`;
- managed `40-configured-remote-inputs.conf` absent;
- `/run/user/1000` directory user-owned mode `0700`;
- `/run/user/1000/private-remote-workspace` directory user-owned mode `0700`;
- Agent socket is Unix socket, user-owned mode `0600`;
- user manager environment provides:
  `DISPLAY=:0`,
  `WAYLAND_DISPLAY=wayland-0`,
  `XDG_RUNTIME_DIR=/run/user/1000`,
  `DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus`.

Any mismatch blocks launch.

## 5. Selected first-launch observation boundary

The exact installed binary may be launched once under a read-only syscall observation
wrapper:

`/usr/bin/strace -f -e trace=connect -s 256 -o <VR_CONNECT_TRACE> /usr/lib/private-remote-workspace/prw-desktop`

with only the verified graphical-session environment exported.

The `strace` wrapper does not alter application bytes and may record only `connect(2)`
system-call metadata needed to prove endpoint selection.

Selected evidence directory:

`/home/gersi365/.local/state/private-remote-workspace/phase152/c03e-vr/launch/7cef99ff78ab11582158201f8fcb2e13b5701161`

Required directory:
- create-only;
- uid/gid `1000:1000`;
- mode `0700`.

Selected trace:
`connect.trace`
- regular file;
- uid/gid `1000:1000`;
- mode `0600`.

If the exact evidence directory already exists, VR fails closed before launch and
reconciles it read-only.

## 6. Required launch proof

After launch, prove:

- exactly one exact installed desktop executable process is running;
- executable resolves to the dpkg-owned installed path;
- GTK application D-Bus registration is present if observable;
- `connect.trace` records successful `connect(2)=0` calls from the desktop process to
  `/run/user/1000/private-remote-workspace/agent.sock`;
- no successful AF_INET or AF_INET6 connect from the desktop process is present in the
  bounded connect trace;
- no TCP/UDP socket owned by the desktop process is observed after startup;
- Agent remains active/running, Result=success, NRestarts=0, local_only;
- managed 40 remains absent;
- package bytes and installed binary hash remain unchanged.

Because the current desktop startup code performs exactly two read-only Agent queries,
two successful Agent-socket connects are expected. A different count is evidence to
reconcile, not something to normalize or hide.

## 7. Read-only connection interpretation

A successful Agent-socket `connect(2)=0` proves local transport establishment by the
launched desktop process.

The current source proves that after each successful connection the desktop sends only:

1. `GetAgentStatus`;
2. `GetPrivateDnsConfig`.

No terminal/file/transfer/forwarding management command is reachable from the startup
probe.

VR does not claim user-interface text unless directly observable through a bounded
application-specific mechanism. It must not capture an unrestricted desktop screenshot
or inspect unrelated user application content merely to prove PRW state.

## 8. Explicitly forbidden actions

VR must not:

- install/reinstall/remove/upgrade the package;
- launch a second independent desktop instance;
- use command-3 management;
- open terminal/file/transfer/forwarding providers;
- mutate private DNS;
- select/write configured-remote values;
- activate remote TCP/UDP networking;
- restart/reload/start/stop/reconfigure Agent;
- write managed 30/40;
- change credentials;
- use sudo;
- modify system configuration;
- merge/ready/close predecessor/current PRs;
- rewrite Git history.

## 9. Prepared classification

`FIRST_DESKTOP_APPLICATION_LAUNCH_PREPARED / EXACT_INSTALLED_BINARY_BOUND / READ_ONLY_LOCAL_AGENT_STARTUP_PROBE_BOUND / CONNECT_TRACE_OBSERVATION_BOUND / NO_COMMAND3 / NO_CONFIGURED_REMOTE / NO_NETWORK_MUTATION / NO_RACE_FREE_CLAIM`

## STOP

`STOP_AFTER_FIRST_DESKTOP_LAUNCH_AND_READ_ONLY_LOCAL_AGENT_CONNECTION_VERIFICATION`

A later checkpoint may address additional desktop functionality only after VR evidence
closes. VR itself grants no management capability.

`NO_RACE_FREE_CLAIM`
