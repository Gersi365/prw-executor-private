# C03e-WJ — Service-Start Readiness Preflight

Status: `SERVICE_START_READINESS_PREFLIGHT_PASS — EXACT_NEW_AGENT_REPROVED — STOPPED_STATE_REPROVED — START_NOT_EXECUTED`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## Boundary

`SERVICE_START_READINESS_PREFLIGHT`

This checkpoint is read-only with respect to production runtime.

It freshly re-proves the evidence-closed WI authority, stopped/quiescent state, exact installed NEW Agent identity, verify-only vendor unit, fixed credential binding, local-only execution-mode drop-in, managed-40 absence, private stage identities, and zero transaction sibling state.

It does not start or restart the service.

## Authoritative predecessor

Evidence-closed C03e-WI / PR #725 is the exact predecessor:

- branch `phase-152-c03e-wi-user-attended-privileged-reconciliation`;
- exact head `5e6ce6f772ae68e6baa24e273ed6873fa02ee41b`;
- exact tree `265ca3c3e0fbf85efb6fe2a67c0348f9076bdef9`;
- exact WH parent / merge base `52b07a3f7cde79f72a1202d54356e032f96ec33d`;
- canonical WI evidence ID `1vSgkwJXxU5Pdx4JVA8Zel2ZCNWWcaUku`.

WI closed the privileged reconciliation and independently proved the installed Agent as the exact NEW payload while the service remained stopped.

The WI audit requires a separate service-start readiness/preflight before any service-start mutation.

## Fresh authority and duplicate guard

Immediately before WJ materialization:

- WI / PR #725 remained draft/open/unmerged/mergeable;
- WI exact head remained `5e6ce6f772ae68e6baa24e273ed6873fa02ee41b`;
- WI canonical evidence remained a singleton file with one revision;
- no C03e-WJ branch existed;
- no C03e-WJ PR existed;
- no canonical C03e-WJ audit-title collision existed;
- production service remained stopped/quiescent;
- installed Agent remained exact NEW identity.

## Primary service-start readiness sampling

Host:

`PowerCode`

Primary WJ timestamp:

`2026-09-19T16:14:27+02:00`

Credential/binding follow-up:

`2026-09-19T16:14:53+02:00`

Pre-write guard:

`2026-09-19T16:15:14+02:00`

## Fresh stopped-state proof

Fresh user-manager state:

- `LoadState=loaded`;
- `ActiveState=inactive`;
- `SubState=dead`;
- `MainPID=0`;
- `Result=success`;
- `NRestarts=0`;
- `NeedDaemonReload=no`;
- `UnitFileState=enabled`;
- manager-visible `PRW_AGENT_EXECUTION_MODE=local_only`.

Manager-loaded DropInPaths remained exactly:

- `/home/gersi365/.config/systemd/user/prw-agent.service.d/20-device-identity-credential.conf`;
- `/home/gersi365/.config/systemd/user/prw-agent.service.d/30-agent-execution-mode.conf`.

Fresh quiescence:

- exact `prw-agent` process count `0`;
- exact `prw-agent-configure` process count `0`;
- user-manager jobs `NONE`;
- Agent Unix listener count `0`;
- runtime `agent.sock` absent.

No service-start command was invoked.

## Installed NEW Agent exact identity

Path:

`/usr/lib/private-remote-workspace/prw-agent`

Fresh metadata:

- owner/group `root:root`;
- uid/gid `0:0`;
- mode `0755`;
- nlink `1`;
- inode `3017271`;
- type regular file;
- exact bytes `11089904`;
- SHA-256:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`.

This exactly equals:

- the frozen WC candidate;
- the exact WE staged candidate;
- the WD compiled NEW Agent identity;
- the WI independently proven installed NEW Agent.

## Verify-only vendor unit exact identity

Path:

`/usr/lib/systemd/user/prw-agent.service`

Fresh metadata:

- owner/group `root:root`;
- uid/gid `0:0`;
- mode `0644`;
- nlink `1`;
- inode `2231750`;
- exact bytes `332`;
- SHA-256:
  `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`.

The vendor unit remains verification-only and unchanged.

## Fixed device-identity credential binding

Fixed drop-in:

`/home/gersi365/.config/systemd/user/prw-agent.service.d/20-device-identity-credential.conf`

Fresh metadata:

- owner/group `gersi365:gersi365`;
- uid/gid `1000:1000`;
- mode `0600`;
- nlink `1`;
- exact bytes `170`;
- SHA-256:
  `42c585ea57aed19662260f0e0421139fc5e4eab973195c3b6e4e16ab7b3ecc77`.

Fresh content remained exactly:

`[Service]`

`LoadCredentialEncrypted=prw.device-identity.private-key.v1:/home/gersi365/.local/state/private-remote-workspace/credentials/device-identity-private-key-v1.cred`

Encrypted source credential path:

`/home/gersi365/.local/state/private-remote-workspace/credentials/device-identity-private-key-v1.cred`

Fresh source metadata:

- owner/group `gersi365:gersi365`;
- uid/gid `1000:1000`;
- mode `0600`;
- nlink `1`;
- inode `6181707`;
- type regular file;
- bytes `762`;
- source file is not a symlink.

Path-chain inspection showed no symlink component.

Private credential bytes were not read.

The runtime credentials directory for `prw-agent.service` was absent while the service remained stopped.

WJ makes no claim that the future start has already materialized the runtime credential. It proves only the exact encrypted source and fixed manager binding needed before the separate start transaction.

## Local-only execution-mode binding

Managed drop-in:

`/home/gersi365/.config/systemd/user/prw-agent.service.d/30-agent-execution-mode.conf`

Fresh metadata:

- owner/group `gersi365:gersi365`;
- uid/gid `1000:1000`;
- mode `0600`;
- nlink `1`;
- exact bytes `58`;
- SHA-256:
  `00a975b8b53ac3b4dd24e0519ab9b5bddf97c0162b28cfed52dcb6cd05673d0f`.

Manager-visible environment remained:

`PRW_AGENT_EXECUTION_MODE=local_only`

No execution-mode transition occurred.

## Managed 40 guard

`40-configured-remote-inputs.conf`

remained:

`ABSENT`

No configured-remote activation is part of WJ.

## Exact private stage

Stage:

`/home/gersi365/.prw-c03e-we-stage`

Path-chain inspection remained free of symlink components.

Fresh stage custody:

- owner/group `gersi365:gersi365`;
- uid/gid `1000:1000`;
- mode `0700`;
- nlink `2`;
- inode `6293644`;
- direct children `3`;
- regular files `3`;
- symlink children `0`;
- subdirectories `0`.

Exact stage hashes:

- deployment manifest:
  `9e792c36126aa3e9c72fd879a935cbdfe88201e0ae84ef9c3db4052a51f3087c`;
- candidate Agent:
  `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`;
- package reconciler:
  `f2792bb1f41b602c8006eba753b20f70bfa858428882755f64aee9756bec4a0b`.

## Package transaction cleanup guard

Fixed Agent package parent:

`/usr/lib/private-remote-workspace`

Fresh metadata:

- owner/group `root:root`;
- uid/gid `0:0`;
- mode `0755`;
- type directory.

Transaction-owned sibling pattern:

`/usr/lib/private-remote-workspace/.prw-agent.c03e-ug.*.candidate`

Fresh count:

`0`

No stale privileged-reconciliation sibling remains.

## Exact future service-start command

The only selected future service-start command is:

`systemctl --user start prw-agent.service`

WJ did not invoke that command.

No `restart`, `try-restart`, `reload`, `daemon-reload`, `reset-failed`, or alternate service-manager mutation is selected.

## Readiness classification

`SERVICE_START_READINESS_PREFLIGHT_PASS / WI_AUTHORITY_REPROVED / INACTIVE_DEAD / MAINPID_ZERO / AGENT_PROCESS_ZERO / AGENT_CONFIGURE_PROCESS_ZERO / AGENT_UNIX_LISTENER_ZERO / AGENT_SOCKET_ABSENT / USER_MANAGER_JOBS_NONE / NRESTARTS_ZERO / NEED_DAEMON_RELOAD_NO / INSTALLED_NEW_AGENT_6A229C76 / INSTALLED_NEW_AGENT_BYTES_11089904 / VERIFY_ONLY_VENDOR_UNIT_24F646DC / FIXED_20_EXACT / ENCRYPTED_CREDENTIAL_SOURCE_CUSTODY_INTACT / RUNTIME_CREDENTIAL_DIR_ABSENT_WHILE_STOPPED / LOCAL_ONLY_30_EXACT / MANAGED_40_ABSENT / EXACT_WE_STAGE_REPROVED / NO_TRANSACTION_SIBLING / READY_FOR_SEPARATELY_AUTHORIZED_SERVICE_START_TRANSACTION / START_NOT_EXECUTED / NO_PRODUCTION_PROBE / NO_RACE_FREE_CLAIM`

## Security and mutation ceiling

WJ does not authorize or perform:

- service start;
- service restart;
- daemon reload;
- production command-1 probe;
- production command-3 probe;
- desktop command-3 dispatch;
- execution-mode transition;
- configured-remote activation;
- private credential read;
- credential replacement;
- enrollment/revocation mutation;
- provider/backend activation;
- network/DNS/firewall/route mutation;
- database/control-plane mutation.

## Explicit non-actions

WJ performed no:

- service start;
- service restart;
- daemon reload;
- reset-failed;
- sudo/root command;
- reconciler invocation;
- installed Agent replacement;
- vendor-unit replacement;
- managed configuration write;
- private credential read;
- production command request;
- desktop command-3 dispatch;
- execution-mode transition;
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

The next checkpoint is separately gated and would be the exact service-start transaction only.

It must begin with another immediate drift guard.

Only if that guard passes may it execute exactly:

`systemctl --user start prw-agent.service`

Immediately after start returns it must independently prove at minimum:

- service `active/running`;
- nonzero `MainPID`;
- exact one Agent process;
- `Result=success`;
- no unexpected restart loop;
- installed Agent still exact NEW identity;
- manager-visible `local_only`;
- fixed `20` and managed `30` still exact;
- managed `40` absent;
- runtime credential materialization exists with expected private custody without reading private key bytes;
- Agent Unix listener is healthy;
- no unexpected Agent TCP/UDP activity.

That successor must STOP before production command-1 or command-3 probes.

## STOP

`STOP_AFTER_SERVICE_START_READINESS_PREFLIGHT_AND_BEFORE_SERVICE_START`

`NO_RACE_FREE_CLAIM`
