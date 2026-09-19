# C03e-WF — WA Agent Production Activation Read-Only Preflight

Status: `READ_ONLY_PREFLIGHT_PASS — READY_FOR_SEPARATELY_AUTHORIZED_SERVICE_STOP — PRODUCTION_RUNTIME_UNCHANGED`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## Boundary

`WA_AGENT_PRODUCTION_ACTIVATION_READ_ONLY_PREFLIGHT`

This checkpoint is read-only with respect to the production runtime. It re-proves the exact private WE stage, installed OLD Agent identity, vendor-unit identity, active-service state, effective local-only configuration, device-identity credential custody, local listener containment, and the selected future sudo identity handoff/order. It performs no service lifecycle mutation, sudo authentication, privileged reconciliation, Agent replacement, production probe, configured-remote activation, or network mutation.

## Authoritative predecessor

Evidence-closed C03e-WE / PR #721 is the exact predecessor:

- branch: `phase-152-c03e-we-private-exact-deployment-stage-materialization`;
- exact head: `871dd3e704628bc66942ac8ba459236b1e2ca1a5`;
- exact tree: `ae89cfba1f26309f8dc3541bcddde8e046927975`;
- exact WD parent / merge base: `ff8cbd518d22c92930fda2cda0cb8788d8d61e4a`;
- canonical WE evidence ID: `1GJxwvYfy-S9xTsJuNIMl9jKSHiD0Hg3V`.

WE materialized only the private exact deployment stage and explicitly stopped before production activation preflight, service stop, sudo authentication, privileged reconciliation, installed Agent replacement, service start, or production probes.

The transaction order remains the selection closed by C03e-WB / PR #718 at exact head:

`8fdce3efb4bd6013a1710bc5514132afd5e0d3df`

WF implements only WB Gate 4, the active-service read-only mutation preflight.

## Fresh repository authority guard

Immediately before WF materialization:

- canonical repository ID remained `1334911207`;
- default branch remained `main`;
- C03e-WA / PR #717 remained draft/open/unmerged at exact head `fe24713c4a72e7e3ad6048f19df5352b8668f552`;
- C03e-WE / PR #721 remained draft/open/unmerged/mergeable at exact head `871dd3e704628bc66942ac8ba459236b1e2ca1a5`;
- PR #721 base remained exact WD head `ff8cbd518d22c92930fda2cda0cb8788d8d61e4a`;
- no C03e-WF branch was identified;
- no C03e-WF PR was identified;
- no canonical C03e-WF Drive audit-title collision was identified.

## Fresh production preflight host

Read-only host: `PowerCode`.

Execution identity:

- user: `gersi365`;
- uid: `1000`;
- gid: `1000`.

Primary live preflight sampling began at:

`2026-09-19T14:52:43+02:00`

Final read-only guard was captured at:

`2026-09-19T14:55:05+02:00`

No host mutation was performed by WF.

## Exact WE stage reproof

Stage:

`/home/gersi365/.prw-c03e-we-stage`

Path-chain readback remained free of symlink components.

Stage directory remained:

- owner/group: `gersi365:gersi365`;
- uid/gid: `1000:1000`;
- mode: `0700`;
- nlink: `2`;
- inode: `6293644`;
- direct children: `3`;
- regular files: `3`;
- symlink children: `0`;
- directory children: `0`.

Exact stage children remained:

### `C03E_WD_DEPLOYMENT_MANIFEST`

- owner/group: `gersi365:gersi365`;
- mode: `0600`;
- nlink: `1`;
- bytes: `597`;
- SHA-256: `9e792c36126aa3e9c72fd879a935cbdfe88201e0ae84ef9c3db4052a51f3087c`.

### `candidate-prw-agent`

- owner/group: `gersi365:gersi365`;
- mode: `0500`;
- nlink: `1`;
- bytes: `11089904`;
- SHA-256: `6a229c76a6b9cc05a413486dfc5e1c2fb920c2890503fe8091bae91a1cd10c44`.

### `prw-agent-package-reconcile`

- owner/group: `gersi365:gersi365`;
- mode: `0500`;
- nlink: `1`;
- bytes: `2898840`;
- SHA-256: `f2792bb1f41b602c8006eba753b20f70bfa858428882755f64aee9756bec4a0b`.

The final WF guard re-proved all three exact hashes and exact child cardinality.

## Installed OLD Agent identity

Installed production Agent:

`/usr/lib/private-remote-workspace/prw-agent`

Fresh readback:

- owner/group: `root:root`;
- uid/gid: `0:0`;
- mode: `0755`;
- nlink: `1`;
- bytes: `11068384`;
- SHA-256: `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`.

The active process executable resolved to the same fixed production path.

The installed Agent therefore remained the exact OLD identity selected by C03e-WB/C03e-WD.

## Verify-only vendor unit identity

Vendor unit:

`/usr/lib/systemd/user/prw-agent.service`

Fresh readback:

- owner/group: `root:root`;
- mode: `0644`;
- bytes: `332`;
- SHA-256: `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`.

The vendor unit remained verify-only. WF performed no vendor-unit write or daemon reload.

## Active service state

Using the existing UID-1000 user-manager bus at `/run/user/1000/bus`, fresh read-only manager state proved:

- `LoadState=loaded`;
- `ActiveState=active`;
- `SubState=running`;
- `MainPID=2983`;
- `Result=success`;
- `NRestarts=0`;
- `NeedDaemonReload=no`;
- `UnitFileState=enabled`;
- `FragmentPath=/usr/lib/systemd/user/prw-agent.service`;
- `ExecStart=/usr/lib/private-remote-workspace/prw-agent`;
- manager-visible `Environment=PRW_AGENT_EXECUTION_MODE=local_only`.

The final WF guard re-proved the same active/running state, PID, result, restart count, reload state, and local-only environment.

The first connector-local `systemctl --user` attempt did not reach the user manager because that connector shell lacked `XDG_RUNTIME_DIR` and `DBUS_SESSION_BUS_ADDRESS`. This was an audit-environment issue, not a service defect or host mutation. Rebinding read-only inspection to the existing UID-1000 bus succeeded.

## Effective systemd configuration

Effective manager-loaded configuration was freshly rendered with `systemctl --user cat prw-agent.service`.

It contained exactly:

1. vendor unit `/usr/lib/systemd/user/prw-agent.service`;
2. fixed credential drop-in `20-device-identity-credential.conf`;
3. local-only execution-mode drop-in `30-agent-execution-mode.conf`.

Manager-visible `DropInPaths` contained exactly those two drop-ins:

- `/home/gersi365/.config/systemd/user/prw-agent.service.d/20-device-identity-credential.conf`;
- `/home/gersi365/.config/systemd/user/prw-agent.service.d/30-agent-execution-mode.conf`.

No `40-configured-remote-inputs.conf` was present in the user, `/etc`, or vendor candidate locations checked.

Fixed `20` remained:

- owner/group `gersi365:gersi365`;
- mode `0600`;
- bytes `170`;
- SHA-256 `42c585ea57aed19662260f0e0421139fc5e4eab973195c3b6e4e16ab7b3ecc77`;
- directive `LoadCredentialEncrypted=prw.device-identity.private-key.v1:<fixed user-state credential path>`.

Managed `30` remained:

- owner/group `gersi365:gersi365`;
- mode `0600`;
- bytes `58`;
- SHA-256 `00a975b8b53ac3b4dd24e0519ab9b5bddf97c0162b28cfed52dcb6cd05673d0f`;
- directive `Environment=PRW_AGENT_EXECUTION_MODE=local_only`.

The effective unit candidate scan identified only:

- the enabled `default.target.wants/prw-agent.service` link;
- fixed `20`;
- managed `30`;
- the vendor unit.

Manager-global `PRW_*` environment count remained `0`.

No queued user-manager job was present.

## Device-identity credential custody

WF did not read private credential contents.

Encrypted source credential path custody was metadata-checked only:

`/home/gersi365/.local/state/private-remote-workspace/credentials/device-identity-private-key-v1.cred`

It remained:

- owner/group `gersi365:gersi365`;
- uid/gid `1000:1000`;
- mode `0600`;
- nlink `1`;
- regular file.

The active process exposed only the expected safe environment names/values:

- `CREDENTIALS_DIRECTORY=/run/user/1000/credentials/prw-agent.service`;
- `PRW_AGENT_EXECUTION_MODE=local_only`.

Runtime credential directory remained user-owned mode `0500`.

Runtime credential metadata remained:

- owner/group `gersi365:gersi365`;
- mode `0400`;
- nlink `1`;
- regular file.

Its bytes were not read.

Current Agent journal retained the public identity marker:

`device_identity_loaded public_spki_sha256=b417e6b0964cd933209bf9ccefcde3b4ee4caafe96dcdbe29278b008d499ce8d`

No private key material was captured.

## Runtime containment

Runtime directory:

`/run/user/1000/private-remote-workspace`

remained:

- owner/group `gersi365:gersi365`;
- mode `0700`.

`agent.lock` remained:

- regular user-owned file;
- mode `0600`.

`agent.sock` remained:

- user-owned Unix socket;
- mode `0600`.

Fresh final counts:

- exact `prw-agent` process count: `1`;
- exact `prw-agent-configure` process count: `0`;
- Agent Unix listener count: `1`;
- Agent TCP activity count: `0`;
- Agent UDP activity count: `0`;
- queued user-manager jobs: `0`.

The one live Agent Unix listener was associated with PID `2983`.

## Selected future service-stop/reconciliation/start order

C03e-WB selected the future mutation order. WF does not change it.

The next mutation gate, if separately authorized, is exactly:

`systemctl --user stop prw-agent.service`

Only after the stop returns may a fresh post-stop read-only proof establish at minimum:

- `MainPID=0`;
- exact Agent process count `0`;
- local Agent Unix listener absent;
- no queued user-manager job;
- no automatic restart;
- installed OLD Agent identity still exact;
- WE stage/reconciler/vendor-unit identities still exact.

Failure of any post-stop requirement is STOP before privileged reconciliation.

Only after that separate post-stop proof may the selected same-terminal sudo reconciliation be considered.

## Selected user-attended sudo shape and identity handoff

WF does not invoke sudo.

The C03e-WB-selected future argv remains:

1. `/usr/bin/sudo`;
2. `-u`;
3. `root`;
4. `--`;
5. `/home/gersi365/.prw-c03e-we-stage/prw-agent-package-reconcile`;
6. `reconcile-current-agent`;
7. `/home/gersi365/.prw-c03e-we-stage`.

Exact WD reconciler source at head `ff8cbd518d22c92930fda2cda0cb8788d8d61e4a`, blob `871d01eebd4694ec4aa0a6e3e5acfbf3dd4b20b2`, requires:

- real UID `0`;
- effective UID `0`;
- authenticated invoking identity from `SUDO_UID`, `SUDO_GID`, and `SUDO_USER`;
- invoking UID/GID to be nonzero canonical decimal IDs;
- `SUDO_USER` to be nonempty and not `root`;
- stage custody to match the invoking UID/GID.

Fresh WF host identity is `gersi365`, uid/gid `1000:1000`, and exact WE stage custody is `1000:1000`.

This proves static compatibility of the selected identity handoff. WF does NOT claim that a future sudo process environment has been runtime-proved. No sudo authentication or root process was invoked.

No password may be requested, transported, stored, captured, or injected by ChatGPT or automation.

## Readiness classification

Fresh C03e-WF classification:

`EXACT_WE_STAGE_REPROVED / INSTALLED_OLD_AGENT_EXACT / VERIFY_ONLY_VENDOR_UNIT_EXACT / ACTIVE_RUNNING / ONE_AGENT_PROCESS / RESULT_SUCCESS / NRESTARTS_ZERO / NEED_DAEMON_RELOAD_NO / FIXED_20_LOADED / LOCAL_ONLY_30_LOADED / MANAGED_40_ABSENT / DEVICE_IDENTITY_CUSTODY_INTACT / DEVICE_IDENTITY_PUBLIC_MARKER_PRESENT / MANAGER_GLOBAL_PRW_ENV_ZERO / NO_USER_MANAGER_JOBS / ONE_AGENT_UNIX_LISTENER / ZERO_AGENT_TCP_ACTIVITY / ZERO_AGENT_UDP_ACTIVITY / SELECTED_STOP_RECONCILE_START_ORDER_REBOUND / SUDO_ARGV_SHAPE_REBOUND / INVOKING_USER_STATIC_COMPATIBILITY_PROVEN / READY_FOR_SEPARATELY_AUTHORIZED_SERVICE_STOP / PRODUCTION_RUNTIME_UNCHANGED / NO_RACE_FREE_CLAIM`

This readiness statement does not authorize service stop by itself.

## Hard prohibitions

C03e-WF must not:

- stop, start, restart, reload, or otherwise mutate `prw-agent.service`;
- authenticate with sudo;
- invoke the reconciler against the stage;
- execute any root/privileged transaction;
- replace the installed Agent;
- replace the vendor unit;
- mutate fixed `20`, managed `30`, or create managed `40`;
- execute production command-1 or command-3 probes;
- dispatch desktop command-3;
- transition execution mode;
- activate configured-remote inputs;
- read private credential bytes;
- mutate credentials, enrollment, or revocation;
- mutate networking, DNS, firewall, routes, forwarding, relay, database, or control-plane state;
- merge, close, mark ready, delete branch, rebase, reset, squash, force-update, or rewrite history without separate authorization.

## Explicit non-actions

WF performed no:

- sudo authentication;
- sudo/root command;
- service lifecycle mutation;
- privileged reconciler execution;
- installed Agent replacement;
- vendor-unit replacement;
- daemon reload;
- managed configuration write;
- production command-1 probe;
- production command-3 probe;
- desktop command-3 dispatch;
- execution-mode transition;
- credential/enrollment mutation;
- network mutation;
- merge;
- ready conversion;
- PR close;
- branch deletion;
- history rewrite.

## STOP

`STOP_AFTER_PRODUCTION_ACTIVATION_READ_ONLY_PREFLIGHT_AND_BEFORE_SERVICE_STOP`

The next safe boundary is one separately authorized service-stop transaction only. It must perform an immediate pre-stop drift guard, execute exactly `systemctl --user stop prw-agent.service`, and then independently prove the required stopped/quiescent state. It must stop before sudo authentication or privileged reconciliation.

`NO_RACE_FREE_CLAIM`
