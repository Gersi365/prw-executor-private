# C03e-WG — WA Agent Service-Stop Transaction

Status: `SERVICE_STOP_COMPLETED — POST_STOP_QUIESCENCE_PROVEN — STOPPED_STATE_PRESERVED — READY_FOR_SEPARATELY_AUTHORIZED_PRIVILEGED_RECONCILIATION_PREFLIGHT`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## Boundary

`WA_AGENT_SERVICE_STOP_TRANSACTION`

C03e-WG contains exactly one production runtime mutation:

`systemctl --user stop prw-agent.service`

It begins with a fresh drift guard, executes that one service-stop command, independently proves the required stopped/quiescent state, records evidence, and stops before sudo authentication, privileged reconciliation, installed Agent replacement, service start, or production probes.

## Authoritative predecessor

Evidence-closed C03e-WF / PR #722 is the exact predecessor:

- branch `phase-152-c03e-wf-wa-agent-production-activation-read-only-preflight`;
- exact head `0c35b4ec31d9455caad57195e1d678fd16aecc93`;
- exact tree `338d2d9a19869a679d3d7adcc645e113f0672f43`;
- exact WE parent / merge base `871dd3e704628bc66942ac8ba459236b1e2ca1a5`;
- canonical WF evidence ID `1iXE37Z-xPif2pQJl2yZC0GCScJHIlbkv`.

WF selected the immediate successor as one explicit service-stop transaction only and required STOP before sudo authentication or privileged reconciliation.

C03e-WB / PR #718 remains the transaction-order authority.

## Pre-mutation authority and duplicate guard

Immediately before the service-stop transaction:

- WF / PR #722 remained draft/open/unmerged/mergeable;
- WF exact head remained `0c35b4ec31d9455caad57195e1d678fd16aecc93`;
- exact-head WF Rust Validation remained success;
- no C03e-WG branch existed;
- no C03e-WG PR existed;
- no C03e-WG canonical evidence title collision existed.

Immediately before WG Git materialization after the stop:

- WF remained exact predecessor;
- no competing C03e-WG branch or PR existed;
- the selected canonical WG evidence title still had zero matches.

## Immediate pre-stop drift guard

Host:

`PowerCode`

Guard time:

`2026-09-19T15:15:07+02:00`

Execution identity:

- user `gersi365`;
- uid `1000`;
- gid `1000`.

The fresh guard proved:

- `LoadState=loaded`;
- `ActiveState=active`;
- `SubState=running`;
- `MainPID=2983`;
- `Result=success`;
- `NRestarts=0`;
- `NeedDaemonReload=no`;
- `UnitFileState=enabled`;
- manager-visible `PRW_AGENT_EXECUTION_MODE=local_only`;
- manager-loaded drop-ins exactly fixed `20` plus managed `30`;
- exact Agent process count `1`;
- exact `prw-agent-configure` process count `0`;
- queued user-manager jobs `NONE`;
- managed `40` absent;
- one Agent Unix listener;
- zero Agent TCP activity;
- zero Agent UDP activity.

Exact stage remained:

`/home/gersi365/.prw-c03e-we-stage`

Stage guard:

- owner/group `gersi365:gersi365`;
- uid/gid `1000:1000`;
- mode `0700`;
- exactly `3` direct children;
- symlink children `0`.

Exact stage hashes remained:

- deployment manifest:
  `9e792c36126aa3e9c72fd879a935cbdfe88201e0ae84ef9c3db4052a51f3087c`;
- candidate Agent:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- package reconciler:
  `f2792bb1f41b602c8006eba753b20f70bfa858428882755f64aee9756bec4a0b`.

Installed OLD Agent remained:

- path `/usr/lib/private-remote-workspace/prw-agent`;
- SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`.

Verify-only vendor unit remained:

- path `/usr/lib/systemd/user/prw-agent.service`;
- SHA-256 `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`.

The guard therefore matched the evidence-closed WF preconditions with no observed drift.

## Authorized service-stop transaction

The sole production mutation was exactly:

`systemctl --user stop prw-agent.service`

The existing UID-1000 user-manager bus was used:

- `XDG_RUNTIME_DIR=/run/user/1000`;
- `DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus`.

Transaction timing:

- begin `2026-09-19T15:15:14+02:00`;
- return `2026-09-19T15:15:14+02:00`;
- return code `0`.

No second stop, restart, start, daemon reload, reset-failed, reconfigure, or sudo operation was performed.

## Immediate post-stop proof

First post-stop snapshot:

`2026-09-19T15:15:29+02:00`

Fresh manager state:

- `LoadState=loaded`;
- `ActiveState=inactive`;
- `SubState=dead`;
- `MainPID=0`;
- `Result=success`;
- `NRestarts=0`;
- `NeedDaemonReload=no`;
- `UnitFileState=enabled`;
- `ActiveExitTimestamp=Sat 2026-09-19 15:15:14 CEST`;
- `InactiveEnterTimestamp=Sat 2026-09-19 15:15:14 CEST`;
- manager-visible `PRW_AGENT_EXECUTION_MODE=local_only`;
- manager-loaded drop-ins remained fixed `20` plus managed `30`.

Quiescence proof:

- exact Agent process count `0`;
- exact `prw-agent-configure` process count `0`;
- queued user-manager jobs `NONE`;
- Agent Unix listener count `0`;
- runtime `agent.sock` path `ABSENT`;
- Agent TCP activity count `0`;
- Agent UDP activity count `0`.

Exact immutable production/stage identities after stop remained:

- manifest SHA `9e792c36126aa3e9c72fd879a935cbdfe88201e0ae84ef9c3db4052a51f3087c`;
- candidate Agent SHA `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- reconciler SHA `f2792bb1f41b602c8006eba753b20f70bfa858428882755f64aee9756bec4a0b`;
- installed OLD Agent SHA `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- vendor-unit SHA `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`;
- managed `40` remained absent.

## Automatic-restart exclusion recheck

Second post-stop snapshot:

`2026-09-19T15:15:33+02:00`

The same stopped/quiescent state remained:

- `ActiveState=inactive`;
- `SubState=dead`;
- `MainPID=0`;
- `Result=success`;
- `NRestarts=0`;
- Agent process count `0`;
- configure process count `0`;
- user-manager jobs `NONE`;
- Agent Unix listener count `0`;
- `agent.sock` path absent;
- Agent TCP/UDP activity `0/0`;
- exact stage, OLD Agent, and vendor-unit hashes unchanged.

No automatic restart was observed across the two post-stop snapshots.

## Service journal stop evidence

Fresh journal lines at the stop boundary recorded:

- Agent terminal event `terminal=sigterm`;
- `exit=success`;
- `cleanup=clean`;
- signal-mask restore `restored`;
- systemd `Stopping prw-agent.service`;
- systemd `Stopped prw-agent.service`.

This journal evidence is consistent with the manager and process/socket proof.

## Pre-materialization stopped-state guard

At:

`2026-09-19T15:15:54+02:00`

the stopped state remained stable:

- `ActiveState=inactive`;
- `SubState=dead`;
- `MainPID=0`;
- `Result=success`;
- `NRestarts=0`;
- `NeedDaemonReload=no`;
- manager-visible mode still `local_only`;
- exact Agent process count `0`;
- jobs `NONE`;
- Unix listener count `0`;
- `agent.sock` absent.

WG therefore remained stopped before documentation/evidence materialization.

## Security and mutation ceiling

C03e-WG authorizes and performs only the service-stop transaction already recorded above.

It does not authorize:

- sudo authentication;
- root command execution;
- privileged reconciler execution;
- installed Agent replacement;
- vendor-unit replacement;
- service start;
- service restart;
- daemon reload;
- managed `20`, `30`, or `40` mutation;
- production command-1 probe;
- production command-3 probe;
- desktop command-3 dispatch;
- execution-mode transition;
- configured-remote activation;
- private credential read;
- credential/enrollment mutation;
- network/DNS/firewall/route mutation;
- database/control-plane mutation.

## Explicit non-actions

WG performed no:

- sudo authentication;
- sudo/root command;
- reconciler invocation;
- reconciler execution against the stage;
- Agent replacement;
- vendor-unit replacement;
- service start/restart;
- daemon reload;
- managed configuration write;
- production probe;
- execution-mode transition;
- private credential read;
- credential/enrollment mutation;
- network mutation;
- merge;
- ready conversion;
- PR close;
- branch deletion;
- reset/rebase/squash/force/history rewrite.

## Closure classification

`SERVICE_STOP_COMPLETED / RETURN_CODE_ZERO / POST_STOP_INACTIVE_DEAD / MAINPID_ZERO / AGENT_PROCESS_ZERO / AGENT_UNIX_LISTENER_ZERO / AGENT_SOCKET_ABSENT / USER_MANAGER_JOBS_NONE / NO_AUTOMATIC_RESTART_OBSERVED / NRESTARTS_ZERO / CLEAN_SIGTERM_EXIT / EXACT_WE_STAGE_PRESERVED / INSTALLED_OLD_AGENT_PRESERVED / VERIFY_ONLY_VENDOR_UNIT_PRESERVED / LOCAL_ONLY_CONFIGURATION_PRESERVED / MANAGED_40_ABSENT / READY_FOR_SEPARATELY_AUTHORIZED_PRIVILEGED_RECONCILIATION_PREFLIGHT / NO_SUDO / NO_PRIVILEGED_RECONCILIATION / NO_AGENT_REPLACEMENT / NO_SERVICE_START / NO_PRODUCTION_PROBE / NO_RACE_FREE_CLAIM`

## Immediate successor

The next checkpoint is separately gated and must begin with a fresh stopped-state and identity drift guard.

It may prepare for the user-attended same-terminal sudo reconciliation selected by C03e-WB, but this WG checkpoint does not authorize sudo authentication or reconciler execution.

The next gate must STOP if any of these have changed:

- service no longer `inactive/dead`;
- `MainPID` nonzero;
- Agent process present;
- Agent Unix listener present;
- queued user-manager job present;
- installed OLD Agent identity changed;
- stage/reconciler/vendor-unit identity changed;
- stage custody changed.

## STOP

`STOP_AFTER_AGENT_SERVICE_STOP_AND_POST_STOP_QUIESCENCE_PROOF_AND_BEFORE_SUDO_AUTHENTICATION_OR_PRIVILEGED_RECONCILIATION`

`NO_RACE_FREE_CLAIM`
