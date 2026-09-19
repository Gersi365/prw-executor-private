# C03e-VR — First Desktop Launch and Read-Only Local Agent Connection Verification Completion

Status: `FIRST_LAUNCH_COMPLETED — LOCAL_AGENT_ONLINE_READY_VERIFIED — APPLICATION_LEFT_RUNNING`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## 1. Exact prepared authority

Prepared VR authority:

- branch: `phase-152-c03e-vr-first-desktop-launch-read-only-local-agent-verification`;
- prepared head: `99d48c1959728ed83fa7e36c36cb647482226518`;
- prepared tree: `4fad30fb35e8de8edda95c2af4e9753f7edab20e`;
- PR #708: draft/open/unmerged.

Prepared VR bound only the installed exact desktop binary and a bounded `connect(2)`
trace observation. It explicitly prohibited command-3 management, configured-remote
selection, Agent mutation and remote networking activation.

## 2. Pre-launch production state

Fresh pre-launch verification at `2026-09-19T09:07:59+02:00` proved:

- package `private-remote-workspace-desktop`: `install ok installed`;
- version `0.1.0-1`;
- architecture `amd64`;
- installed binary:
  `/usr/lib/private-remote-workspace/prw-desktop`;
- installed binary SHA-256:
  `1adb489772c54b98996845f1ecca1e3a77e47e4cc8f215535f0afb9159fc26af`;
- Agent active/local_only;
- managed 40 absent;
- desktop exact executable process count `0`;
- exact VR evidence directory absent.

The user graphical session environment had already been read-only verified as:

- `DISPLAY=:0`;
- `WAYLAND_DISPLAY=wayland-0`;
- `XDG_RUNTIME_DIR=/run/user/1000`;
- `DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus`;
- `XDG_SESSION_TYPE=wayland`;
- `XDG_CURRENT_DESKTOP=ubuntu:GNOME`.

Runtime trust state:

- `/run/user/1000`: user-owned directory mode `0700`;
- `/run/user/1000/private-remote-workspace`: user-owned directory mode `0700`;
- Agent socket: Unix socket, user-owned mode `0600`.

## 3. Exact-head validation before launch

All prepared-source PASS claims bind only to:

`99d48c1959728ed83fa7e36c36cb647482226518`

PRW Rust Validation:
- workflow ID `334913600`;
- run ID `35428306828`;
- run number `1965`;
- job ID `105858073553`;
- status `completed`;
- conclusion `success`.

Locked graph, formatting, Clippy, tests and workspace build all succeeded.

Other exact-head workflows:
- C02f-AD run `35428306841` / #1213 — `SKIPPED`;
- C02f-AE run `35428306829` / #1204 — `SKIPPED`.

`SKIPPED` is not PASS.

## 4. First desktop launch

The exact installed binary was launched once on PowerCode under the selected bounded
read-only syscall observation wrapper.

Observed application process:

- PID `853113`;
- executable:
  `/usr/lib/private-remote-workspace/prw-desktop`;
- exact executable process count after startup: `1`.

The GTK application registered on the user session bus as:

`io.patchmirror.prw.desktop`

with PID `853113`.

The application was intentionally left running after VR verification.

## 5. Bounded connect trace

Evidence directory:

`/home/gersi365/.local/state/private-remote-workspace/phase152/c03e-vr/launch/7cef99ff78ab11582158201f8fcb2e13b5701161`

Directory:
- type: directory;
- owner: `gersi365:gersi365`;
- mode: `0700`.

Trace:

`connect.trace`

- regular file;
- owner `gersi365:gersi365`;
- mode `0600`;
- size `758` bytes;
- SHA-256:
  `448fa595e64eb897341df896f26ee0e7aaee1a51f6c95a5926bfd6e533f00335`.

The trace records only successful AF_UNIX connects for:
- user session D-Bus;
- Wayland;
- AT-SPI;
- exact PRW Agent Unix socket.

Exact PRW Agent socket connect evidence:

`/run/user/1000/private-remote-workspace/agent.sock`

was observed exactly twice with:

`connect(...)=0`

from the desktop startup probe worker.

Observed successful Agent-socket connect count:

`2`

Observed `AF_INET` / `AF_INET6` connect count:

`0`

No TCP or UDP socket owned by the desktop process was observed after startup.

## 6. Application-specific accessibility verification

No unrestricted screenshot was captured.

The AT-SPI accessibility bus identified exactly the PRW application process and its own
application tree.

Observed PRW application root:

- role: `application`;
- name: `prw-desktop`;
- AccessibleId: `io.patchmirror.prw.desktop`.

Observed window:

`Private Remote Workspace`

Observed Overview labels:

- `Overview`;
- `Read-only local Agent status. Phase 151 performs no production network mutation.`;
- `Agent status\nAvailability: Online\nRuntime: Ready`;
- `Private DNS\nEnabled: No\nDevice naming: No\nResolvers: 0\nSplit domains: 0`;
- `Local IPC protocol 1.0`.

The observed labels prove that the launched desktop completed the bounded startup probe
and accepted a valid Agent status response sufficient to present:

- Agent availability `Online`;
- Agent runtime `Ready`;
- local IPC protocol `1.0`;
- a validated Private DNS snapshot.

All other navigation destinations remained present as the existing Phase-151 surfaces.
No management action was executed.

## 7. Source-to-observation binding

At the exact installed/source authority, desktop startup:

1. derives the Agent endpoint from `XDG_RUNTIME_DIR`;
2. validates runtime-root, PRW runtime-directory and socket trust;
3. issues `GetAgentStatus`;
4. issues `GetPrivateDnsConfig`;
5. validates terminal response framing and request-ID correlation;
6. presents `Availability: Online` only after valid status decoding.

Therefore the two successful exact Agent-socket connections plus the observed
`Online / Ready / Local IPC protocol 1.0` application labels establish read-only local
Agent connection success.

No command-3 management request is part of this startup path.

## 8. Post-launch Agent and package preservation

Read-only reconciliation at `2026-09-19T09:10:34+02:00` proved:

- Agent `active/running`;
- `Result=success`;
- `NRestarts=0`;
- manager-visible `PRW_AGENT_EXECUTION_MODE=local_only`;
- managed `40-configured-remote-inputs.conf` absent;
- installed desktop binary SHA-256 unchanged:
  `1adb489772c54b98996845f1ecca1e3a77e47e4cc8f215535f0afb9159fc26af`;
- exactly one desktop executable process active;
- no observed TCP/UDP socket owned by the desktop process.

## 9. Explicit non-actions

C03e-VR performed no:

- package reinstall/remove/upgrade;
- command-3 management dispatch;
- terminal/file/transfer/forwarding provider action;
- private-DNS mutation;
- configured-remote value selection or write;
- remote TCP/UDP activation;
- Agent restart/reload/start/stop/reconfiguration;
- managed 30/40 mutation;
- credential/private-key mutation;
- sudo/root action;
- unrestricted screenshot capture;
- unrelated application UI inspection;
- merge, ready conversion, PR close, branch deletion, reset, rebase, squash, force update,
  or history rewrite.

## 10. Completion classification

`FIRST_DESKTOP_APPLICATION_LAUNCH_COMPLETED / EXACT_INSTALLED_BINARY_RUNNING / GTK_APPLICATION_REGISTERED / EXACT_AGENT_UNIX_SOCKET_CONNECTED_TWICE / AF_INET_AF_INET6_CONNECT_COUNT_0 / NO_DESKTOP_TCP_UDP_SOCKET_OBSERVED / APPLICATION_ACCESSIBILITY_OVERVIEW_VERIFIED / AGENT_AVAILABILITY_ONLINE / AGENT_RUNTIME_READY / LOCAL_IPC_PROTOCOL_1_0 / PRIVATE_DNS_READ_ONLY_SNAPSHOT_RENDERED / AGENT_ACTIVE_LOCAL_ONLY_PRESERVED / MANAGED_40_ABSENT / DESKTOP_LEFT_RUNNING / NO_COMMAND3 / NO_CONFIGURED_REMOTE / NO_NETWORK_MUTATION / NO_RACE_FREE_CLAIM`

## STOP

`STOP_AFTER_FIRST_DESKTOP_LAUNCH_AND_READ_ONLY_LOCAL_AGENT_CONNECTION_VERIFICATION`

The application may remain open in its current read-only state.

Any activation of terminal, files, transfers, forwarding, command-3 management, remote
peer configuration, or configured-remote networking requires a separate later
checkpoint.

`NO_RACE_FREE_CLAIM`
