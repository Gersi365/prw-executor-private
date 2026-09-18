# C03e-VD Production Agent Local-Only Configuration Materialization Preflight Readiness

Status: `READ_ONLY_PREFLIGHT_BLOCKED_ON_PRW_AGENT_CONFIGURE_PACKAGING_DEPLOYMENT_INTEGRATION — VALIDATION_PENDING — EVIDENCE_PENDING — ACTUAL_MANAGED_TARGETS_UNMUTATED — NO_DAEMON_RELOAD — AGENT_NOT_ACTIVATED`

Boundary: `PRODUCTION_AGENT_LOCAL_ONLY_CONFIGURATION_MATERIALIZATION_PREFLIGHT_READINESS`

Authoritative predecessor: evidence-closed C03e-VC / PR #693 at exact head `8101f022d3af2deab76a64e38ff9b691f17748fe`, exact tree `b47024f8917bb689f34c1d78ab3e1e814a9faf31`.

## Authorization boundary

C03e-VD is read-only on the production host. It does not authorize or perform managed-file writes, `daemon-reload`, service start/restart, enablement changes, enrollment, revocation, networking, DNS, forwarding, relay, or credential decryption.

## Historical target-name correction

C03e-VC used conceptual filenames `30-local-only.conf` and `40-production-network.conf` while identifying the missing execution-mode blocker.

The exact configuration writer source at the C03e-VC head defines the actual managed filenames as:

- `30-agent-execution-mode.conf` via `EXECUTION_MODE_DROPIN_NAME`;
- `40-configured-remote-inputs.conf` via `CONFIGURED_REMOTE_INPUTS_DROPIN_NAME`.

This correction does not change the C03e-VC blocker conclusion: both actual managed files are absent, the historical conceptual names are also absent, and `PRW_AGENT_EXECUTION_MODE` remains unavailable to the user manager.

Future checkpoints must use the exact source-defined managed filenames.

## Exact source identities

Configuration authority:

- `crates/prw-agent-configuration/src/lib.rs` blob `1a5ef55f4e7036da0f5ac12924f6c08abfdfe525`;
- canonical local-only bytes: `[Service]\nEnvironment=PRW_AGENT_EXECUTION_MODE=local_only\n`;
- execution-mode recognizer accepts only canonical local-only or configured-remote bytes.

Linux managed writer:

- `crates/prw-agent-configuration/src/linux_systemd.rs` blob `ecbc9cbb28054f6e10fe759aa8009e793345f190`;
- managed mode leaf `30-agent-execution-mode.conf`;
- managed remote-input leaf `40-configured-remote-inputs.conf`;
- identity leaf `20-device-identity-credential.conf` is explicitly outside writer ownership.

Orchestration surface:

- `crates/prw-agent-systemd-orchestration/src/lib.rs` blob `3ca8c785b902a0a4e793bc09c02352096ee83922`;
- `crates/prw-agent-systemd-orchestration/src/main.rs` blob `3329089d5934c14fcce1c72f750218eb5fea3682`;
- `crates/prw-agent-systemd-orchestration/Cargo.toml` blob `d1cc94c6b2aab2c7742d55cc5614484c7b96a5a9`;
- binary target name `prw-agent-configure`.

## Exact write-only orchestration contract

`prw-agent-configure` accepts exactly one action token:

- `write`; or
- `reconfigure-active`.

For the pre-activation local-only lane, the relevant source-defined action is `write`.

The `write` action:

1. requires same real/effective user context;
2. resolves absolute `HOME`;
3. resolves `XDG_CONFIG_HOME`, defaulting only when absent to `$HOME/.config`;
4. reads `PRW_AGENT_EXECUTION_MODE` from its own process environment;
5. for exact value `local_only`, constructs `ManagedAgentConfiguration::LocalOnly` without requiring remote bundle variables;
6. calls `apply_managed_systemd_configuration`;
7. performs no `daemon-reload`, start, restart, enablement or linger mutation.

`reconfigure-active` is not selected by C03e-VD because the Agent is not active/ready and that action owns manager/restart operations outside this boundary.

## Writer filesystem contract

The writer validates the intended user, discovers user-unit search paths with `/usr/bin/systemd-analyze --user unit-paths`, and validates parent/managed directory custody.

Managed directory policy:

- path `$XDG_CONFIG_HOME/systemd/user/prw-agent.service.d`;
- exact mode `0700`;
- owned by intended user;
- opened with directory `NOFOLLOW` semantics;
- exclusive advisory writer lock held during the transaction.

Existing managed leaves, when present, must be:

- regular files;
- intended-user owned;
- exact mode `0600`;
- canonical PRW-recognized content.

Foreign or unsafe managed leaf state fails closed.

## Writer staging/commit semantics

For local-only desired state, the writer:

1. snapshots actual `30-agent-execution-mode.conf` and `40-configured-remote-inputs.conf`;
2. rejects external systemd fragments that can affect the seven selected PRW environment values;
3. renders canonical local-only mode bytes;
4. stages them in the managed directory under a private `.prw-agent-configuration.*.tmp` name using `CREATE|EXCL|NOFOLLOW`, mode `0600`;
5. writes, `sync_all`s, reopens and byte-compares the staged file;
6. re-snapshots both managed leaves and refuses commit if they differ from the pre-stage snapshots;
7. commits the mode file with same-directory `renameat`;
8. removes canonical remote inputs only if they existed before local-only transition;
9. `sync_all`s the managed directory;
10. reopens and verifies exact final desired state;
11. rolls managed files back to exact pretransaction snapshots on supported failure paths.

This is an atomic managed replacement transaction with rollback and post-commit verification. It is **not** a strict create-only/no-replace transaction: `renameat` can replace an existing destination after the writer's custody checks. C03e-VD makes no strict no-replace or race-free claim.

## Fresh production filesystem custody

Observed caller context:

- user `gersi365`;
- uid `1000`;
- `HOME=/home/gersi365`;
- shell `XDG_CONFIG_HOME` unset;
- source-defined effective config root therefore `/home/gersi365/.config`.

Fresh read-only parent/managed custody:

- `/home/gersi365`: directory, mode `0750`, uid/gid `1000:1000`;
- `/home/gersi365/.config`: directory, mode `0700`, uid/gid `1000:1000`;
- `/home/gersi365/.config/systemd`: directory, mode `0755`, uid/gid `1000:1000`;
- `/home/gersi365/.config/systemd/user`: directory, mode `0700`, uid/gid `1000:1000`;
- `/home/gersi365/.config/systemd/user/prw-agent.service.d`: directory, mode `0700`, uid/gid `1000:1000`.

No observed path in that chain was a symlink.

Fixed identity binding remains:

- `20-device-identity-credential.conf`: regular file, mode `0600`, uid/gid `1000:1000`, size `170`;
- SHA-256 `42c585ea57aed19662260f0e0421139fc5e4eab973195c3b6e4e16ab7b3ecc77`.

Fresh managed-target state:

- actual `30-agent-execution-mode.conf`: absent;
- actual `40-configured-remote-inputs.conf`: absent;
- historical `30-local-only.conf`: absent;
- historical `40-production-network.conf`: absent;
- `.prw-agent-configuration.*.tmp` residue count: `0`.

## Fresh external-configuration custody preflight

Exact writer-style `systemd-analyze --user unit-paths` under explicit HOME/XDG context returned nine unique absolute paths:

1. `/home/gersi365/.config/systemd/user.control`;
2. `/home/gersi365/.config/systemd/user`;
3. `/etc/systemd/user`;
4. `/run/systemd/user`;
5. `/home/gersi365/.local/share/systemd/user`;
6. `/usr/local/share/systemd/user`;
7. `/usr/share/systemd/user`;
8. `/usr/local/lib/systemd/user`;
9. `/usr/lib/systemd/user`.

Read-only conflict scanning using the source-selected semantics inspected the relevant fixed `20` drop-in and vendor `prw-agent.service` fragment. No selected-variable `Environment`, `UnsetEnvironment`, `PassEnvironment`, `EnvironmentFile`, continued-line ambiguity, unsafe symlink/non-file fragment, or other competing custody was found.

Result: `VD_EXTERNAL_CONFIGURATION_PREFLIGHT=PASS`, conflict count `0`.

## Fresh manager/runtime guard

User-manager state remains:

- `LoadState=loaded`;
- `ActiveState=failed`;
- `SubState=failed`;
- `MainPID=0`;
- `Result=exit-code`;
- `NRestarts=6`;
- manager-visible `DropInPaths=` empty;
- `UnitFileState=enabled`;
- `NeedDaemonReload=yes`;
- manager `PRW_AGENT_EXECUTION_MODE` absent;
- exact Agent process count `0`.

No manager operation was executed.

## Production configure execution surface

Source contains a real binary target named `prw-agent-configure` with the fixed `write` action described above.

Fresh production-host lookup found `prw-agent-configure` absent from:

- `/usr/bin/prw-agent-configure`;
- `/usr/local/bin/prw-agent-configure`;
- `/usr/lib/private-remote-workspace/prw-agent-configure`;
- `/usr/libexec/private-remote-workspace/prw-agent-configure`.

Exact running `prw-agent-configure` process count: `0`.

The current `packaging/systemd` package boundary documents the Agent binary + global user unit, plus the separately integrated device-identity provisioner. The locked Agent package transaction manages only:

- `/usr/lib/private-remote-workspace/prw-agent`;
- `/usr/lib/systemd/user/prw-agent.service`.

No `prw-agent-configure` deployment destination or installation transaction is defined by the currently observed permanent systemd packaging contract.

C03e-VD therefore does not authorize an ad-hoc Cargo build or direct execution from an unretained source checkout as a substitute for an evidence-controlled production artifact.

## Technical classification

`SOURCE_CONFIGURE_BINARY_EXISTS / WRITE_ONLY_LOCAL_ONLY_ACTION_EXISTS / CANONICAL_ACTUAL_30_NAME_30_AGENT_EXECUTION_MODE_CONF / CANONICAL_ACTUAL_40_NAME_40_CONFIGURED_REMOTE_INPUTS_CONF / FILESYSTEM_CUSTODY_READY / ACTUAL_30_ABSENT / ACTUAL_40_ABSENT / WRITER_TEMP_RESIDUE_ZERO / EXTERNAL_CONFIGURATION_PREFLIGHT_CLEAR / MANAGER_UNACTIVATED / PRODUCTION_CONFIGURE_ARTIFACT_ABSENT / CURRENT_PACKAGE_BOUNDARY_EXCLUDES_CONFIGURE_BINARY / BLOCKED_ON_PRW_AGENT_CONFIGURE_PACKAGING_DEPLOYMENT_INTEGRATION`

## Production mutation statement

C03e-VD performs no production-host mutation.

It does not:

- create, replace or remove actual managed `30`/`40` files;
- create the historical conceptual filenames;
- execute `prw-agent-configure`;
- build/stage/install a production `prw-agent-configure` artifact;
- run `daemon-reload`;
- start/restart/enable/disable the Agent;
- clear failed state or reset start limits;
- read/decrypt/export private credential material;
- mutate enrollment, revocation, network, DNS, forwarding or relay state.

## STOP

`STOP_BEFORE_PRW_AGENT_CONFIGURE_PACKAGING_DEPLOYMENT_INTEGRATION_AND_BEFORE_ANY_MANAGED_CONFIGURATION_WRITE_OR_USER_SYSTEMD_MANAGER_RELOAD_OR_AGENT_ACTIVATION`

The next safe boundary is source/package/deployment integration materialization for `prw-agent-configure` as a validated, mechanically deployable, non-activating artifact. Production installation and the subsequent `write` action must remain separately gated.

`NO_RACE_FREE_CLAIM`
