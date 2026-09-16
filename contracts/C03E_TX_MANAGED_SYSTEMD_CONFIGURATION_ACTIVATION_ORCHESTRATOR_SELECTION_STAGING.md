# C03e-TX — managed systemd configuration activation orchestrator selection staging

Status: **SELECTION STAGING**

## 1. Purpose

C03e-TX selects only the concrete same-user caller/orchestration boundary above the evidence-closed C03e-TW managed configuration writer. It freezes the split between configuration-file mutation, target-systemd parse/load validation, user-manager daemon reload, active-service restart, local readiness proof, and bounded post-write rollback.

Canonical selected law:

`NEW_DEDICATED_PRW_AGENT_SYSTEMD_ORCHESTRATION_CRATE / ONE_SHOT_PRW_AGENT_CONFIGURE_BINARY / FIXED_WRITE_OR_RECONFIGURE_ACTIVE_ACTIONS / SAME_USER_ONLY / EXISTING_SEVEN_CONFIGURATION_ENV_SOURCES / NO_CLI_VALUE_GRAMMAR / NO_DESKTOP_OR_AGENT_SELF_MANAGEMENT / FORWARD_ONLY_TW_SYSTEMD_SUBPROCESS_ENV_SANITIZATION / OPAQUE_RECOVERABLE_MANAGED_FILE_STATE_TOKEN / WRITE_LANE_HAS_NO_SERVICE_MANAGER_MUTATION / RECONFIGURE_ACTIVE_REQUIRES_HEALTHY_ACTIVE_RUNNING_READY_BASELINE / POST_WRITE_SYSTEMD_VERIFY_WITH_WARNING_FAILURE / DAEMON_RELOAD_BEFORE_ACTIVE_SERVICE_RESTART / TRY_RESTART_ONLY_ON_FORWARD_PATH / LOCAL_IPC_GET_AGENT_STATUS_READY_IS_REQUIRED_POSTCONDITION / LOCAL_READINESS_IS_NOT_REMOTE_REACHABILITY / FAILURE_AFTER_RELOAD_RESTORES_FILES_RELOADS_AND_RESTARTS_PREVIOUS_ACTIVE_STATE / NO_FIRST_START / NO_ENABLEMENT_OR_LINGER_CHANGE / NO_DEPLOYMENT_OR_HOST_MUTATION_IN_TX`

TX is documentation-only. It performs no Rust/Cargo/source materialization, managed drop-in mutation, daemon reload, service start/restart/stop, enablement/linger change, deployment, network activation, merge, ready conversion, PR close, branch deletion or history rewrite.

## 2. Authoritative predecessor

The exact predecessor is evidence-closed C03e-TW PR `#661`.

Exact TW identity:

- branch: `phase-152-c03e-tw-managed-systemd-configuration-writer-source-materialization`;
- head: `20ea7db4dae6bcdc11a9363b8132826524b83178`;
- tree: `3a4c97415c84472f8e6c5f038c1a459fc1f5adb5`;
- parent/base: C03e-TV `c7423306bce29e05608bc2de44724249bd896ad4`;
- PR remains draft/open/unmerged;
- exact-final-head Rust and Android validation are SUCCESS;
- immutable Drive evidence is already recorded by PR #661 closure binding.

TW remains authoritative for exact value validation, serialization, managed-leaf custody, commit ordering, and in-writer rollback. TX does not reinterpret those laws.

## 3. Preserved TT/TU/TW artifacts

The package-owned vendor unit remains byte-stable:

`packaging/systemd/prw-agent.service`

The selected user drop-ins remain:

- identity: `20-device-identity-credential.conf`;
- execution mode: `30-agent-execution-mode.conf`;
- configured-remote inputs: `40-configured-remote-inputs.conf`.

The TW writer continues to own only `30-` and `40-`. The selected orchestration layer does not gain authority over the vendor unit, the identity drop-in, unrelated drop-ins, enablement, linger, package installation, firewall, route, DNS, control-plane state, authentication state, database state, or arbitrary filesystem paths.

## 4. Fresh source archaeology

At exact TW head there is no dedicated Agent service-configuration orchestration crate or binary.

The current surfaces are intentionally unsuitable as the new owner:

1. `prw-agent` is the long-lived headless service executable. Making it rewrite its own systemd configuration or restart itself would collapse the service/runtime boundary.
2. `apps/desktop` is a client/read-only local-status surface and its architecture explicitly excludes Agent installation/restart/replacement ownership.
3. `prw-device-identity-provisioning` is a first-device-identity provisioning tool. TV already restricted it to filesystem-safety precedent and forbids widening its semantic ownership.
4. packaging scripts/service source are not a typed same-user configuration orchestrator and must not become a shell-based configuration grammar.

The repository does have a useful administrative precedent: a separate one-shot identity provisioning binary with fixed behavior and bounded diagnostics.

## 5. Selected new orchestration owner

A future source-materialization checkpoint must create one new workspace crate:

`crates/prw-agent-systemd-orchestration`

Package name:

`prw-agent-systemd-orchestration`

It must provide one one-shot administrative binary:

`prw-agent-configure`

Dependency direction is one-way:

- `prw-agent-systemd-orchestration` may depend on `prw-agent-configuration`;
- `prw-agent-systemd-orchestration` may depend on `prw-agent` only for existing public local IPC contracts/codecs/status types;
- `prw-agent-configuration` must not depend on the orchestration crate;
- `prw-agent` must not depend on the orchestration crate.

This prevents a service self-management cycle and keeps the shared configuration authority below the administrative caller.

## 6. Exact command surface

The binary accepts exactly one action token and no additional positional or option arguments:

- `write`
- `reconfigure-active`

Any missing, additional, or unknown argument fails before configuration acquisition or filesystem/systemd mutation.

Configuration values are never accepted as argv strings. TX selects no arbitrary key/value CLI, shell fragment, unit name, path, environment name, command, timeout, service action, or subprocess argument supplied by the caller.

The unit name remains fixed:

`prw-agent.service`

The systemd executables remain fixed absolute paths:

- `/usr/bin/systemd-analyze`
- `/usr/bin/systemctl`

No shell is involved.

## 7. Configuration source law

The caller reuses the existing fixed process-configuration family; it does not create a second configuration grammar.

Execution mode is read only from:

`PRW_AGENT_EXECUTION_MODE`

It must validate through `prw-agent-configuration::validate_agent_execution_mode`.

For exact `local_only`, the caller constructs `ManagedAgentConfiguration::LocalOnly` and does not acquire or validate the six configured-remote values.

For exact `configured_remote`, the caller must acquire all six existing fixed variables and pass their exact Unicode semantic text to `ConfiguredRemoteBundle::try_new`:

1. `PRW_REMOTE_BIND_ADDR`
2. `PRW_REMOTE_PEER_DEVICE_ID`
3. `PRW_REMOTE_MAX_ACTIVE_WORKERS`
4. `PRW_REMOTE_APPLICATION_LEASE_SECONDS`
5. `PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS`
6. `PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS`

Missing, empty where the existing validator rejects empty, non-Unicode, malformed, out-of-range or otherwise invalid values fail before file mutation. There is no trim, case-folding, alias, default, inference, fallback or normalization.

## 8. Intended-user context

The orchestration binary is an unprivileged same-user tool.

Before either action it must require:

- real UID equals effective UID;
- intended UID is that exact UID;
- `HOME` is present, non-empty and absolute;
- `XDG_CONFIG_HOME`, when present, is non-empty and absolute;
- otherwise the effective configuration root is exactly `$HOME/.config`.

It then constructs the existing `IntendedUserSystemdContext` and delegates managed-file custody to `prw-agent-configuration`.

The binary must not accept another UID, HOME, XDG root or service account as an argument and must not use sudo, setuid, privilege switching or root escalation.

## 9. Newly discovered TW subprocess-environment defect

Fresh TX archaeology proved that exact TW `discover_user_unit_search_paths(...)` invokes `/usr/bin/systemd-analyze --user unit-paths` while inheriting the caller process environment except for explicit HOME/XDG configuration assignments.

A disposable probe proved that an inherited `SYSTEMD_UNIT_PATH=/tmp/...` replaces the normal user-unit search path completely.

Classification:

`SOURCE HARDENING DEFECT DISCOVERED AFTER EVIDENCE-CLOSED TW`

This does not rewrite or invalidate immutable TW historical evidence. It means a successor must correct the live source forward-only before any orchestration activation can be considered complete.

The correction is selected here:

- the writer's `systemd-analyze --user unit-paths` child must start from `env_clear()`;
- it must receive only the explicit intended `HOME`, effective `XDG_CONFIG_HOME`, and stable locale required for deterministic text parsing;
- ambient `SYSTEMD_UNIT_PATH`, `SYSTEMD_LOG_*`, alternate D-Bus values, XDG data overrides and unrelated process environment must not influence custody discovery.

A disposable `env -i HOME=... XDG_CONFIG_HOME=... systemd-analyze --user unit-paths` probe on the current target systemd retained the expected normal user/system search path family without the injected override.

## 10. Recoverable file-state authority required for activation

Exact TW exposes public canonical recognizers/renderers and the public writer, but the secure filesystem `LeafSnapshot` and `restore_leaf` mechanisms are private to `linux_systemd`.

Therefore an external caller cannot safely reconstruct or restore exact pretransaction `30-`/`40-` filesystem state after a successful writer return by ad hoc path reads.

The future source-materialization checkpoint must extend `prw-agent-configuration::linux_systemd` with an opaque, consuming recoverable transaction surface.

The selected semantic shape is:

1. apply one validated desired configuration through the same directory-FD/NOFOLLOW writer custody;
2. retain the exact pretransaction managed-leaf bytes/existence inside a non-inspectable rollback token;
3. retain the exact committed post-write state inside that token;
4. return the token only after the existing postcommit verification succeeds;
5. allow one consuming restore operation that first proves the current managed leaves still equal the transaction's exact committed state;
6. only then restore the exact pretransaction `30-`/`40-` bytes/existence using the existing safe staging/fsync/rename/unlink machinery;
7. fail closed rather than overwrite if current managed state drifted, became foreign, changed metadata, or otherwise no longer matches the token's committed state.

The rollback token carries no authority over service-manager state and no arbitrary path or foreign file.

The existing simple `apply_managed_systemd_configuration(...)` API may remain for configuration-only callers.

## 11. `write` action boundary

`prw-agent-configure write` performs only:

1. exact action validation;
2. same-user HOME/XDG configuration context validation;
3. exact selected configuration acquisition and semantic validation;
4. call to the hardened TW writer;
5. existing writer postcommit readback/verification;
6. bounded non-secret success/failure reporting.

It performs no:

- `XDG_RUNTIME_DIR` requirement;
- user-manager connection;
- `systemd-analyze verify` activation gate;
- daemon reload;
- service start/restart/stop;
- enable/disable;
- linger operation;
- local IPC readiness probe.

A successful `write` proves only managed file desired state, exactly as TW specifies.

## 12. `reconfigure-active` is not first activation

`prw-agent-configure reconfigure-active` is selected only for an already healthy running `prw-agent.service` owned by the same user.

It must fail before file mutation unless the pretransaction service is proven:

- `LoadState=loaded`;
- `ActiveState=active`;
- `SubState=running`;
- `MainPID` is non-zero;
- `InvocationID` is present;
- local PRW IPC `GetAgentStatus` returns correlated `OK`, runtime `Ready`, and the current local IPC protocol version.

The exact pretransaction `UnitFileState` is captured for later equality verification but is not changed.

This precondition prevents `reconfigure-active` from silently becoming a first-start, failed-service recovery, enablement transaction or linger transaction.

During TX selection archaeology, the currently observed PowerCode service was `enabled` but `failed`, not `active/running`. Under this selected law the current host state would be rejected with zero mutation. That observation is not a frozen deployment target and TX performs no repair.

## 13. User-manager context for `reconfigure-active`

The active-service lane additionally requires process `XDG_RUNTIME_DIR` to be present, non-empty and absolute.

The runtime root must be opened/validated without symlink following and must be:

- a directory;
- owned by the intended UID;
- exact mode `0700`.

The caller must not synthesize `/run/user/<uid>` merely because a UID is known.

The caller must not trust an ambient `DBUS_SESSION_BUS_ADDRESS`. Every `systemctl`/activation-validation child starts from a cleared environment and receives only the selected explicit same-user context, including the validated `XDG_RUNTIME_DIR`, intended HOME/XDG configuration root and stable locale.

A read-only selection probe confirmed that `/usr/bin/systemctl --user` can reach the current user manager from a validated real `XDG_RUNTIME_DIR` with ambient `DBUS_SESSION_BUS_ADDRESS` removed.

## 14. Pretransaction local readiness proof

Systemd process state alone is not sufficient health proof.

The orchestration crate must reuse the existing public `prw-agent` local IPC contracts:

- `LocalIpcContract::socket_path(...)`;
- `LocalAgentCommand::GetAgentStatus`;
- canonical request-frame stream writer;
- canonical frame reader;
- terminal response validation;
- successful status-frame decoder;
- `LocalAgentRuntimeState::is_ready()`;
- `LocalIpcProtocolVersion::current()`.

Before any file mutation, the socket leaf must be a Unix-domain socket owned by the intended UID with exact mode `0600`.

The readiness request uses one non-zero correlation ID, verifies exact response correlation, requires terminal status `OK`, requires runtime `Ready`, and requires the current protocol version.

This proves only the existing local Agent request surface. It is not remote reachability, remote authentication, endpoint health, peer connectivity, control-plane health, capability readiness or deployment completion.

## 15. Recoverable file commit for active reconfiguration

After all pretransaction service/readiness checks pass, `reconfigure-active` must apply the desired managed configuration through the recoverable writer surface selected in section 10.

All existing TW semantics remain authoritative:

- full semantic validation before staging;
- exact canonical bytes;
- external custody conflict preflight;
- `40-` then configured-remote `30-` forward ordering;
- local-only `30-` then recognized `40-` removal;
- exact postcommit reopen verification;
- no two-file atomicity claim.

The returned rollback token remains in-process and must not be logged or serialized.

## 16. Target-systemd parse gate before daemon reload

After successful file commit and before any daemon reload, the caller must execute the fixed target-systemd verification command equivalent to:

`/usr/bin/systemd-analyze --user --recursive-errors=no --man=no --generators=no verify prw-agent.service`

under the sanitized explicit same-user environment.

A disposable current-systemd probe proved two relevant facts:

1. with valid user-manager context, a valid target unit/drop-in passes;
2. without `--recursive-errors=no`, an unknown target-unit directive may emit a warning yet exit zero, while `--recursive-errors=no` converts warnings in the specified target unit into non-zero failure.

Accordingly TX requires both process success and the warning-failure mode above. A non-zero result blocks daemon reload and triggers file-state restore only.

Unrelated unit warnings must not be promoted into PRW target authority; `--recursive-errors=no` scopes the non-zero warning rule to the specified unit rather than its full dependency graph.

## 17. Daemon-reload boundary

Only after the desired file state and target-systemd parse gate both succeed may the caller execute:

`/usr/bin/systemctl --user daemon-reload`

This is a distinct service-manager phase after file commit.

After daemon reload, the caller must prove:

- `LoadState=loaded`;
- the manager reports the intended PRW-managed `30-` drop-in in its loaded drop-in set;
- for configured-remote, the intended PRW-managed `40-` drop-in is also present;
- for local-only, the PRW-managed `40-` drop-in is absent;
- captured `UnitFileState` is unchanged.

No restart is attempted if these post-reload checks fail.

## 18. Forward restart law

The forward path must use:

`/usr/bin/systemctl --user try-restart prw-agent.service`

not ordinary `restart`.

Rationale: the precondition requires an already-active service, and `try-restart` does not intentionally start a unit that became inactive before the restart operation. This prevents a race from widening `reconfigure-active` into first activation.

There is no retry loop around daemon reload or try-restart.

`Restart=on-failure` in the vendor unit remains service policy, not configuration retry, orchestration fallback or proof of success.

## 19. Bounded post-restart readiness

After successful forward `try-restart`, the caller must wait only for a bounded readiness interval.

Selected bounds reuse existing repository precedent:

- overall startup/readiness deadline: `5 seconds`;
- each local IPC read/write timeout: `2 seconds`;
- polling must sleep between attempts and must not busy-loop.

Before classifying success, the caller must prove:

- `LoadState=loaded`;
- `ActiveState=active`;
- `SubState=running`;
- non-zero `MainPID`;
- non-empty posttransaction `InvocationID` distinct from the captured pretransaction invocation;
- captured `UnitFileState` unchanged;
- trusted same-UID mode-`0600` Agent socket;
- correlated `GetAgentStatus` response status `OK`;
- runtime state `Ready`;
- current local IPC protocol version.

Again, this is local process/request readiness only. TX selects no remote connectivity probe and no claim that configured-remote peer reachability exists.

## 20. Failure before daemon reload

If recoverable file application or target-systemd verification fails before daemon reload:

1. stop forward progress;
2. if a recoverable writer token exists, consume it to restore exact prior managed `30-`/`40-` state;
3. verify exact restored file state;
4. do not daemon-reload;
5. do not restart/start/stop the service.

The already-running baseline service therefore remains on its previously loaded configuration.

A restore failure is terminal bounded rollback failure and must not trigger destructive best-effort cleanup.

## 21. Failure after daemon reload

If any post-reload check, forward try-restart, or post-restart readiness proof fails:

1. stop forward activation;
2. restore exact pretransaction managed file state using the consuming rollback token;
3. verify exact restored file state;
4. execute one `/usr/bin/systemctl --user daemon-reload` to load the restored files;
5. execute one ordinary `/usr/bin/systemctl --user restart prw-agent.service` because the captured pretransaction authority proved the service was active/running and rollback is restoring that prior active state;
6. re-run the bounded systemd and local IPC readiness proof for the restored configuration;
7. require captured `UnitFileState` to remain unchanged.

Ordinary `restart` is selected only on this rollback path. It is not a first-start authority; it restores the exact pretransaction active-state class after a failed reconfiguration attempt.

Any restore/reload/restart/readiness failure in this sequence is reported as terminal rollback failure. No additional retry, enable/disable, linger mutation, package rollback, identity mutation or arbitrary cleanup is selected.

## 22. Crash semantics

TX does not claim crash-atomicity across files, user-manager daemon reload and process restart.

The recoverable file token is intentionally in-process and non-persistent. A process crash can therefore leave an inspectable intermediate state, for example new files with an old manager load or a reloaded manager before process restart.

TX selects no persistent rollback journal, hidden service drop-in, state database or transaction marker.

A later invocation must fail closed on contradictory file/manager/service state and require an explicit subsequent recovery/reconfiguration decision rather than invent the lost pre-crash state.

## 23. Bounded diagnostics

The new binary must report only stable action/stage/result classifications and non-secret identifiers already safe for diagnostics.

It must not print:

- configured bind address;
- peer device ID;
- capacity/lease values;
- raw environment values;
- HOME/XDG paths unless separately proven necessary for a bounded diagnostic;
- D-Bus addresses;
- arbitrary systemd stderr;
- credential paths or secret material.

Detailed subprocess stderr may be consumed internally for classification/tests but must not be blindly relayed as production diagnostic output.

## 24. No desktop, remote IPC or shell activation authority

TX explicitly rejects:

- adding write/restart controls to the desktop client;
- adding configuration mutation commands to the existing Agent local-management IPC;
- remote invocation of this administrative transaction;
- Agent self-restart/self-configuration;
- shell-script ownership;
- arbitrary `systemctl` passthrough;
- caller-selected service/unit names;
- caller-selected paths or subprocesses.

The selected caller is a local same-user one-shot administrative executable only.

## 25. Future source-materialization ceiling

If the future source implementation matches this selection without discovering a contradiction, the expected maximum source delta is exactly these six paths:

1. `Cargo.toml` — add the new workspace member;
2. `Cargo.lock` — record the new workspace package/direct dependencies without opportunistic upgrades;
3. `crates/prw-agent-configuration/src/linux_systemd.rs` — sanitize the writer's systemd-analyze environment and materialize the opaque recoverable transaction/restore surface;
4. `crates/prw-agent-systemd-orchestration/Cargo.toml`;
5. `crates/prw-agent-systemd-orchestration/src/lib.rs`;
6. `crates/prw-agent-systemd-orchestration/src/main.rs`.

The new crate may reuse already-locked direct workspace/dependency versions, including `prw-agent`, `prw-agent-configuration` and `rustix`, but must not introduce an unrelated third-party version upgrade.

No `prw-agent` source change is selected because the public local IPC request/status surface required for readiness already exists.

If correct materialization requires a seventh path, Agent API visibility widening, desktop change, package/service-unit mutation, workflow mutation, Android mutation, or any runtime/host activation, the future checkpoint must STOP and return to a new selection/audit before mutation.

## 26. Validation requirements for future materialization

The future source checkpoint must at minimum prove:

- all historical TW writer tests still pass;
- ambient `SYSTEMD_UNIT_PATH` and related cleared-environment inputs cannot replace custody discovery;
- valid env-cleared normal unit-path discovery remains accepted;
- rollback token cannot expose raw snapshots or arbitrary paths;
- restore refuses drifted/foreign post-commit managed state;
- `write` invokes no systemctl/systemd activation operation;
- local-only caller does not acquire six configured-remote sources;
- configured-remote requires all six exact sources;
- action parser rejects every extra/unknown argument;
- inactive/failed service blocks `reconfigure-active` before writer mutation;
- healthy active/ready baseline is required;
- target-systemd warning on the specified unit fails verification;
- daemon reload occurs only after file and parse validation;
- forward path uses try-restart and never enable/start;
- post-restart success requires active/running plus local `GetAgentStatus` Ready/current protocol;
- post-reload failure restores files before rollback reload/restart;
- rollback preserves UnitFileState and never touches linger;
- local readiness is not reported as remote reachability;
- bounded timeout paths terminate without retry loops or busy-waiting.

Full locked workspace formatting, Clippy, tests and build remain required on the exact final head.

## 27. Exact non-actions in C03e-TX

This selection checkpoint performs no:

- Rust/source implementation;
- Cargo or lockfile mutation;
- creation/materialization of `prw-agent-systemd-orchestration`;
- modification of the TW writer;
- managed `30-` or `40-` file mutation;
- vendor-unit or identity-drop-in mutation;
- `systemd-analyze verify` against the real PRW unit as an activation operation;
- `systemctl --user daemon-reload`;
- `systemctl --user restart` or `try-restart`;
- service start/stop;
- enable/disable;
- linger change;
- desktop or Agent IPC mutation;
- deployment;
- network/listener/firewall/route/DNS mutation;
- repository configuration mutation;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- reset/rebase/squash/force update/history rewrite;
- destructive evidence cleanup.

The disposable systemd probes used during selection operated only on temporary fixture files or read-only service metadata. They did not mutate the real PRW service manager or PRW managed drop-ins.

## 28. Successor boundary

After TX is exact-head validated and evidence-closed, the next separately authorized checkpoint may materialize only the six-path source ceiling above, including the forward-only TW subprocess-environment hardening and the dormant one-shot orchestration surfaces.

That source-materialization successor must still perform no real `write`, daemon reload, try-restart, restart, service recovery or deployment on the host.

A later real-host configuration/activation transaction remains separately gated even after the orchestration source exists and passes disposable validation.

STOP after TX evidence closure.
