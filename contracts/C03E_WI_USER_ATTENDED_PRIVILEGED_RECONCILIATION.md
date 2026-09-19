# C03e-WI — User-Attended Privileged Reconciliation

Status: `PRIVILEGED_RECONCILIATION_COMPLETED — EXACT_NEW_AGENT_INDEPENDENTLY_PROVEN — SERVICE_REMAINS_STOPPED — READY_FOR_SEPARATELY_AUTHORIZED_SERVICE_START_PREFLIGHT`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## Boundary

`USER_ATTENDED_PRIVILEGED_RECONCILIATION`

C03e-WI records one separately authorized privileged package reconciliation transaction performed by the user in a local interactive terminal, followed by an independent read-only installed-byte proof.

It does not authorize or perform service start, service restart, production command probes, execution-mode transition, configured-remote activation, credential mutation, network mutation, or database/control-plane mutation.

## Authoritative predecessor

Evidence-closed C03e-WH / PR #724 is the exact predecessor:

- branch `phase-152-c03e-wh-privileged-reconciliation-readiness-preflight`;
- exact head `52b07a3f7cde79f72a1202d54356e032f96ec33d`;
- exact tree `7ab8c1d2f9bfd9627d6d39a8679b4f7d863f9016`;
- exact WG parent / merge base `51f6fc096305bf62d38824e06e1f53bdb1024796`;
- canonical WH evidence ID `1c15x2RaFthh_mlihX6Noq0-sgwx2e4LI`.

C03e-WB / PR #718 remains the production-activation transaction-order authority.

Exact C03e-WD reconciliation source authority remains:

- WD head `ff8cbd518d22c92930fda2cda0cb8788d8d61e4a`;
- reconciliation source blob `871d01eebd4694ec4aa0a6e3e5acfbf3dd4b20b2`;
- main blob `e973038d2281f9b51f3fbce9d8763158b5ffc6dc`.

## Fresh pre-sudo drift guard

Immediately before the user-attended transaction, at:

`2026-09-19T15:51:29+02:00`

PowerCode freshly re-proved:

- user `gersi365`, uid/gid `1000:1000`;
- `LoadState=loaded`;
- `ActiveState=inactive`;
- `SubState=dead`;
- `MainPID=0`;
- `Result=success`;
- `NRestarts=0`;
- `NeedDaemonReload=no`;
- manager-visible `PRW_AGENT_EXECUTION_MODE=local_only`;
- manager-loaded drop-ins exactly fixed `20` plus managed `30`;
- exact Agent process count `0`;
- exact configure process count `0`;
- user-manager jobs `NONE`;
- Agent Unix listener count `0`;
- runtime `agent.sock` absent;
- managed `40` absent.

Exact stage remained:

`/home/gersi365/.prw-c03e-we-stage`

with:

- owner/group `gersi365:gersi365`;
- uid/gid `1000:1000`;
- mode `0700`;
- exactly `3` direct children;
- symlink children `0`.

Exact pre-transaction hashes:

- deployment manifest:
  `9e792c36126aa3e9c72fd879a935cbdfe88201e0ae84ef9c3db4052a51f3087c`;
- candidate Agent:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- package reconciler:
  `f2792bb1f41b602c8006eba753b20f70bfa858428882755f64aee9756bec4a0b`;
- installed OLD Agent:
  `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`;
- verify-only vendor unit:
  `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`.

Fixed package parents remained root-owned mode `0755`.

Pre-transaction stale transaction sibling count remained `0`.

The connector shell itself remained non-TTY and was not used for the privileged command.

## User-attended privileged transaction

The user personally executed the WB-selected exact command in a local interactive terminal:

`/usr/bin/sudo -u root -- /home/gersi365/.prw-c03e-we-stage/prw-agent-package-reconcile reconcile-current-agent /home/gersi365/.prw-c03e-we-stage`

The local terminal displayed a sudo authentication prompt. The user handled authentication locally.

No password value was provided to ChatGPT, captured by ChatGPT, transported through the connector, stored in evidence, or injected by automation.

The user-reported reconciler result was exactly:

`prw-agent-package-reconcile result=current_agent_payload_reconciled`

The user then returned to the normal `gersi365` shell prompt.

No service-start command was entered as part of this transaction.

## Independent post-reconciliation installed-byte proof

Independent read-only proof began at:

`2026-09-19T15:57:28+02:00`

This proof did not invoke sudo or the reconciler.

### Service state remained stopped

Fresh manager state:

- `LoadState=loaded`;
- `ActiveState=inactive`;
- `SubState=dead`;
- `MainPID=0`;
- `Result=success`;
- `NRestarts=0`;
- `NeedDaemonReload=no`;
- `UnitFileState=enabled`;
- manager-visible `PRW_AGENT_EXECUTION_MODE=local_only`;
- manager-loaded drop-ins remained fixed `20` plus managed `30`.

Fresh quiescence:

- exact Agent process count `0`;
- exact configure process count `0`;
- user-manager jobs `NONE`;
- Agent Unix listener count `0`;
- `agent.sock` absent.

### Installed Agent is exact NEW payload

Installed path:

`/usr/lib/private-remote-workspace/prw-agent`

Fresh metadata:

- owner/group `root:root`;
- uid/gid `0:0`;
- mode `0755`;
- nlink `1`;
- inode `3017271`;
- regular file;
- exact bytes `11089904`;
- SHA-256:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`.

This exactly equals the frozen WC/WE candidate identity and the WD compiled NEW Agent identity.

The prior OLD Agent identity:

`9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`

is no longer the installed Agent payload.

### Verify-only vendor unit unchanged

`/usr/lib/systemd/user/prw-agent.service`

remained:

- owner/group `root:root`;
- mode `0644`;
- bytes `332`;
- SHA-256:
  `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`.

No vendor-unit replacement occurred.

### Private stage unchanged

Stage remained:

- owner/group `gersi365:gersi365`;
- uid/gid `1000:1000`;
- mode `0700`;
- exactly `3` children;
- symlink children `0`.

Exact stage hashes remained:

- manifest:
  `9e792c36126aa3e9c72fd879a935cbdfe88201e0ae84ef9c3db4052a51f3087c`;
- candidate:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- reconciler:
  `f2792bb1f41b602c8006eba753b20f70bfa858428882755f64aee9756bec4a0b`.

### Package sibling cleanup

Fresh package-parent listing showed the canonical installed Agent and other established package executables.

The transaction-owned sibling pattern:

`/usr/lib/private-remote-workspace/.prw-agent.c03e-ug.*.candidate`

had count:

`0`

No stale reconciliation sibling remained after successful reconciliation.

### Managed configuration unchanged

Managed `40-configured-remote-inputs.conf` remained absent.

No execution-mode transition occurred.

## Reconciliation result classification

The privileged transaction and independent readback jointly establish:

`USER_ATTENDED_SUDO_COMPLETED / RECONCILER_REPORTED_CURRENT_AGENT_PAYLOAD_RECONCILED / INDEPENDENT_INSTALLED_NEW_AGENT_HASH_PROVEN / INDEPENDENT_INSTALLED_NEW_AGENT_BYTES_PROVEN / ROOT_0755_INSTALLED_AGENT_PROVEN / VERIFY_ONLY_VENDOR_UNIT_UNCHANGED / PRIVATE_STAGE_UNCHANGED / NO_TRANSACTION_SIBLING_REMAINS / SERVICE_REMAINS_INACTIVE_DEAD / MAINPID_ZERO / AGENT_PROCESS_ZERO / AGENT_UNIX_LISTENER_ZERO / USER_MANAGER_JOBS_NONE / LOCAL_ONLY_CONFIGURATION_PRESERVED / MANAGED_40_ABSENT / READY_FOR_SEPARATELY_AUTHORIZED_SERVICE_START_PREFLIGHT / NO_SERVICE_START / NO_PRODUCTION_PROBE / NO_RACE_FREE_CLAIM`

## Security and mutation ceiling

WI records only the user-attended privileged reconciliation already completed.

It does not authorize or perform:

- service start;
- service restart;
- daemon reload;
- production command-1 probe;
- production command-3 probe;
- desktop command-3 dispatch;
- configured-remote activation;
- execution-mode transition;
- private credential read;
- credential/enrollment mutation;
- terminal/file/forwarding provider activation;
- network/DNS/firewall/route mutation;
- database/control-plane mutation.

## Explicit non-actions

WI performed no:

- connector-mediated password handling;
- `sudo -S`;
- askpass;
- password pipe or redirection;
- NOPASSWD/sudoers mutation;
- root shell;
- `su`;
- `pkexec`;
- service start;
- service restart;
- daemon reload;
- vendor-unit replacement;
- managed configuration write;
- production command-1 request;
- production command-3 request;
- desktop command-3 dispatch;
- configured-remote activation;
- credential/enrollment mutation;
- network mutation;
- database/control-plane mutation;
- merge;
- ready conversion;
- PR close;
- branch deletion;
- history rewrite.

## Immediate successor

The next checkpoint is separately gated and must be a service-start readiness/preflight before any start command.

It must freshly re-prove:

- exact WI Git/evidence authority;
- service still `inactive/dead`;
- `MainPID=0`;
- Agent process and listener absent;
- user-manager jobs absent;
- installed Agent still exact NEW identity;
- vendor unit still exact;
- fixed `20` and local-only `30` still exact;
- managed `40` absent;
- no unexpected transaction sibling or package drift.

Only after a fresh passing start preflight and a separate explicit authorization may the exact service-start gate be considered.

## STOP

`STOP_AFTER_PRIVILEGED_RECONCILIATION_AND_INDEPENDENT_NEW_AGENT_BYTE_PROOF_AND_BEFORE_SERVICE_START`

`NO_RACE_FREE_CLAIM`
