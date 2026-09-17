# C03e-UD — Real-host active reconfiguration validation selection

## 1. Status and boundary

`SELECTION — REAL HOST EXECUTION NOT INCLUDED`

Boundary:

`REAL_HOST_ACTIVE_RECONFIGURATION_VALIDATION_SELECTION`

C03e-UD is a docs-only selection checkpoint. It selects the exact preconditions, executable provenance, semantic-noop desired-state law, real-host `reconfigure-active` sequence, rollback/reconciliation ceiling, evidence requirements, and STOP conditions for a separately authorized successor.

C03e-UD itself performs no managed writer execution against the host, no `systemd-analyze verify` against a mutated host state, no `systemctl daemon-reload`, no service restart/try-restart/start/stop, no package/deployment mutation, and no real activation.

## 2. Authoritative predecessor

The authoritative predecessor is evidence-closed C03e-UC / PR #667.

Exact predecessor identity:

- branch: `phase-152-c03e-uc-external-systemd-fragment-content-custody-source-materialization`;
- head: `23c50e20fdfe8c481b22d062706346f8f898a189`;
- tree: `c8b3f7b2e04fa12cc80483573fcee84bd925fac3`;
- predecessor/base: evidence-closed C03e-UB `f4669866199758b43309ab664eb455e4900cfedf`.

UC is source-materialization only. Its immutable audit explicitly records that real host systemd activation was not authorized or executed, and the post-publication PR closure binding marks UC evidence-closed while retaining `NO_RACE_FREE_CLAIM`.

## 3. Selection objective

The first real-host successor must validate the already-materialized active reconfiguration path without introducing a new desired configuration and without widening authority.

UD therefore selects a semantic-noop active reconfiguration: the desired PRW-managed 30/40 configuration supplied to `prw-agent-configure reconfigure-active` must be exactly equivalent to the recognized canonical managed configuration already present on the host immediately before execution.

The successor is an execution/evidence checkpoint, not a source-development checkpoint.

## 4. Decision summary

UD selects:

`EXACT_UC_HEAD_EXECUTABLE / LOCKED_DISPOSABLE_BUILD / NO_INSTALL / SAME_USER_ONLY / ALREADY_ACTIVE_RUNNING_ONLY / BASELINE_LOCAL_IPC_READY_REQUIRED / CANONICAL_EXISTING_MANAGED_STATE_AS_DESIRED / SEMANTIC_NOOP_30_40_TARGET / ENV_CLEAR_AND_EXPLICIT_RECONSTRUCTION / RECONFIGURE_ACTIVE_ONLY / EXISTING_EXTERNAL_FRAGMENT_CUSTODY / EXISTING_TARGET_VERIFY / EXISTING_LOADED_TOPOLOGY_PROOF / FORWARD_TRY_RESTART_ONLY / BOUNDED_ROLLBACK / EXTERNAL_DRIFT_SUPPRESSES_UNSAFE_RESTART / NO_AUTOMATIC_RETRY / READ_ONLY_POSTRUN_RECONCILIATION / IMMUTABLE_EVIDENCE / NO_FIRST_START / NO_ENABLEMENT_OR_LINGER / NO_DEPLOYMENT / NO_EXTERNAL_FRAGMENT_MUTATION / NO_RACE_FREE_CLAIM`

No other real-host activation mechanism is selected.

## 5. Why semantic-noop active reconfiguration is selected

A first real-host proof should exercise the real manager/process transition path while minimizing configuration risk.

Using the host's already-recognized canonical managed 30/40 state as the desired target means the successor does not intentionally change Agent execution mode, remote peer identity, bind address, worker limit, lease duration, rendezvous limit, or scheduling-consumption limit.

The transaction may still replace managed file inodes as part of the recoverable writer and may reload/restart the already-running service as selected by the existing orchestration. Therefore it is a real activation event and remains separately gated.

## 6. No first-start authority

UD does not select any path that starts an inactive Agent.

Before the real executable may mutate managed state, the existing orchestration must require the baseline unit to be loaded, active, running, have a nonzero main PID, have a nonempty invocation ID and unit-file state, and pass local IPC Ready.

If the service is inactive, failed, activating, deactivating, not loaded, or not locally Ready, the successor must stop without attempting `start`, `restart`, `try-restart`, enablement repair, linger repair, package repair, or deployment.

## 7. Exact executable source provenance

The real-host successor must build `prw-agent-configure` only from exact evidence-closed UC head:

`23c50e20fdfe8c481b22d062706346f8f898a189`

The selected Cargo binary is:

- package: `prw-agent-systemd-orchestration`;
- binary: `prw-agent-configure`;
- repository path declaration: `crates/prw-agent-systemd-orchestration/Cargo.toml`;
- binary source: `crates/prw-agent-systemd-orchestration/src/main.rs`.

No later branch, working-tree edit, unstaged patch, generated source replacement, or locally modified Cargo manifest/lockfile may supply the executable.

## 8. Locked disposable build

The successor must use an isolated disposable checkout/worktree pinned to exact UC head and run:

`cargo build --locked -p prw-agent-systemd-orchestration --bin prw-agent-configure`

The resulting executable path is expected to be the checkout-local `target/debug/prw-agent-configure` unless Cargo is explicitly directed to an equivalent disposable target directory.

Before execution, evidence must record:

- exact Git head and tree;
- clean tracked source state;
- `rustc --version`;
- `cargo --version`;
- exact build command;
- executable byte length;
- executable SHA-256.

The built executable must not be installed, copied over a packaged binary, moved into `/usr`, `/usr/local`, a user bin directory, a systemd unit path, or another persistent activation surface.

## 9. No deployment coupling

Building the exact-head executable in a disposable checkout is not deployment authority.

UD selects direct execution of that disposable exact-head binary only for the separately authorized host-validation transaction.

No package manager operation, desktop updater, service unit replacement, vendor unit replacement, symlink installation, PATH mutation, or persistent wrapper installation is selected.

## 10. Same-user execution context

The successor must execute as the same real/effective unprivileged user whose user-systemd instance and PRW Agent are being validated.

No `sudo`, `su`, setuid wrapper, privileged helper, root shell, or UID transition is selected for `prw-agent-configure`.

The executable's existing same-user checks remain authoritative.

## 11. Intended-user context inputs

The invocation context must use explicit absolute values for:

- `HOME`;
- `XDG_CONFIG_HOME` or the exact selected default derived from HOME;
- `XDG_RUNTIME_DIR`.

The runtime directory must already satisfy the existing ownership/type/mode custody checks.

The successor must not fabricate a different HOME/XDG tree to make validation pass.

## 12. Existing managed state is the only selected desired state

Immediately before real execution, the successor must read the intended user's existing PRW-managed files from the selected managed `prw-agent.service.d` directory without modifying them.

The selected semantic-noop law is:

1. `30-agent-execution-mode.conf` must exist as recognized canonical PRW managed content;
2. if that file selects local-only mode, `40-configured-remote-inputs.conf` must be absent;
3. if that file selects configured-remote mode, `40-configured-remote-inputs.conf` must exist as recognized canonical PRW managed content containing the complete selected six-value remote bundle;
4. the exact recognized values become the desired environment supplied to the executable;
5. no value may be chosen from an unrelated shell profile, stale `.env`, previous chat, test fixture, package default, or operator memory.

If existing managed state is absent, foreign, partial, internally inconsistent, or cannot be proven canonical, the successor must stop without executing `reconfigure-active`.

## 13. Managed-state evidence minimization

Raw configured-remote values must not be copied into the canonical audit merely to prove the semantic-noop target.

The successor should record:

- execution-mode classification;
- presence/absence law for 40;
- exact byte length and SHA-256 for each present managed leaf before execution;
- the same byte length/SHA-256 after execution.

Stable audit text must not disclose unnecessary environment values.

## 14. Minimal invocation environment

The real executable must be launched from a cleared environment equivalent to `env -i` and reconstructed only with the fields required by the selected action.

Required context variables are:

- `HOME`;
- `XDG_CONFIG_HOME`;
- `XDG_RUNTIME_DIR`;
- `LC_ALL=C`;
- `LANG=C`.

For local-only mode, add only the exact recognized `PRW_AGENT_EXECUTION_MODE` value.

For configured-remote mode, add the exact recognized execution mode plus the exact six recognized remote bundle variables.

No ambient `SYSTEMD_UNIT_PATH`, shell startup state, arbitrary PATH-dependent executable selection, credential injection, or unrelated PRW environment is selected.

The executable itself uses fixed absolute systemd tool paths for the selected manager operations.

## 15. Read-only preflight before the mutation boundary

Before invoking the real action, the successor must perform read-only observation sufficient to establish that execution is eligible and evidence is recoverable.

At minimum record, without changing state:

- current UTC timestamp;
- current Git exact-head/build identity;
- current user identity sufficient to prove same-user execution;
- `LoadState`;
- `ActiveState`;
- `SubState`;
- whether `MainPID` is nonzero;
- whether `InvocationID` is nonempty;
- `UnitFileState`;
- current `DropInPaths` count plus a privacy-preserving exact digest of the ordered vector;
- managed 30/40 presence and byte hashes;
- exact-title Drive evidence collision state for the future execution audit.

Read-only preflight does not replace the executable's own baseline status, IPC Ready, topology, writer, external custody, or target-verify checks.

## 16. Local IPC Ready remains mandatory

The existing UC orchestration proves local Agent IPC Ready before external-custody capture and before managed mutation.

UD does not introduce a weaker health signal such as PID existence, socket-path existence, or `systemctl is-active` alone.

If local IPC Ready fails inside the executable, the action must terminate before the recoverable managed writer.

## 17. Selected real invocation

The only selected real administrative action is:

`<exact-disposable-UC-binary> reconfigure-active`

executed with the minimal explicit environment selected above.

The `write` action is not selected for this checkpoint because it does not exercise the manager/process activation path.

No other positional argument or free-form systemctl verb is selected.

## 18. Existing forward sequence remains authoritative

The real-host successor relies on the exact evidence-closed UC sequencing:

1. prove baseline systemd unit active/running and loaded topology valid;
2. prove local IPC Ready;
3. capture original external-fragment custody;
4. perform recoverable managed 30/40 writer;
5. reprove original external custody before target-systemd verification;
6. perform target-systemd verification;
7. reprove original external custody before `daemon-reload`;
8. perform `daemon-reload`;
9. prove loaded topology after reload;
10. reprove original external custody before forward `try-restart`;
11. perform `try-restart`;
12. prove post-restart local Ready and loaded topology;
13. reprove the same original external custody before success.

UD does not reorder, skip, weaken, or duplicate these gates.

## 19. Target-systemd verification remains exact

The selected target verification remains the existing fixed command equivalent to:

`/usr/bin/systemd-analyze --user --recursive-errors=no --man=no --generators=no verify prw-agent.service`

under the executable's sanitized intended-user environment.

Warnings/errors handled by the existing source remain authoritative. UD does not add a permissive fallback.

## 20. External-fragment custody remains exact

The UC external-fragment custody token remains the selected authority for external main-unit/drop-in byte and metadata stability during the transaction.

No external fragment is copied, repaired, normalized, locked as universal authority, overwritten, chmodded, chowned, renamed, removed, or persisted by the successor.

`NO_RACE_FREE_CLAIM` remains authoritative.

## 21. Loaded topology remains independently required

Success still requires the UA/UC loaded-topology law, including valid absolute/unique `DropInPaths`, exact foreign ordered-subsequence stability, exact desired managed 30/40 membership, and unchanged selected unit-file-state law at the existing gates.

External byte custody does not replace loaded topology proof, and loaded topology proof does not replace external byte custody.

## 22. Successful-path mutation ceiling

On a successful real-host transaction, the selected manager/process mutation ceiling is exactly the existing forward path:

- managed writer may atomically replace the PRW-owned managed 30/40 state with byte-equivalent canonical desired state;
- one forward `daemon-reload`;
- one forward `try-restart prw-agent.service`;
- no explicit `start`;
- no explicit `stop`;
- no ordinary `restart` on the success path;
- no enable/disable;
- no linger change.

Because the baseline must already be active/running, `try-restart` is selected instead of first-start authority.

## 23. General rollback ceiling with unchanged external custody

For a non-custody failure after manager mutation, the existing bounded rollback law remains authoritative when original external custody can still be re-proved.

The maximum selected recovery surface may include:

- restoration of prior managed 30/40 bytes through the existing one-use recovery token;
- at most one rollback `daemon-reload` for that recovery path;
- at most one rollback `restart prw-agent.service` where existing orchestration permits it;
- readiness/topology proof after rollback restart.

Across a transaction that reached the forward restart gate, this means there may have been one forward `try-restart` attempt plus one rollback `restart` attempt. UD does not describe this as zero-risk or no-restart validation.

## 24. External-drift rollback ceiling

If original external custody changes, the UC external-drift disposition remains authoritative.

In particular:

- pre-reload drift restores managed state and stops without manager reload/restart;
- post-reload/pre-restart drift may restore managed state and perform at most one rollback daemon reload, but suppresses ordinary restart;
- after a forward restart attempt, known external drift suppresses an additional automatic rollback restart;
- final external drift prevents success classification;
- no external file repair is selected.

The stable bounded classification remains `external_configuration_drift`.

## 25. No automatic retry

The separately authorized real-host checkpoint gets at most one automatic invocation of the selected real action.

If the executable returns failure, is interrupted, loses the remote control channel, or its completion state becomes ambiguous, the successor must not simply invoke `reconfigure-active` again.

A second invocation would be a new mutation transaction and requires fresh read-only reconciliation plus separate explicit authorization if still desired.

## 26. Ambiguous interruption law

If command transport is lost while the process may have crossed the managed-writer boundary, the successor must assume state is uncertain.

It must:

1. stop issuing mutating commands;
2. perform read-only systemd and managed-file reconciliation;
3. capture available process/command evidence;
4. avoid claiming success or exact rollback unless directly proven;
5. return for explicit operator authorization before any repair or retry.

No timeout or transport failure authorizes an extra restart, reload, rewrite, or rerun.

## 27. Success classification

The successor may classify the real-host transaction successful only when all of the following are proven:

- exact selected executable returned exit success;
- stdout contains the bounded completion event for `action=reconfigure-active`;
- no failure event was returned;
- the executable therefore completed its internal post-restart Ready/topology/external-custody success gates;
- post-run read-only systemd state remains loaded, active and running;
- managed 30/40 bytes remain semantically-noop equivalent to the pre-run selected desired state;
- no external repair/deployment/enablement/linger action occurred.

The audit must distinguish command success from the broader post-run evidence closure.

## 28. Read-only post-run reconciliation

After the selected executable exits, the successor must perform read-only observation before publishing evidence.

At minimum record:

- exit status and bounded stdout/stderr event classification;
- post-run `LoadState`, `ActiveState`, `SubState`, nonzero-main-PID boolean, nonempty-invocation-ID boolean, and `UnitFileState`;
- post-run ordered `DropInPaths` count and exact digest;
- post-run managed 30/40 presence and byte hashes;
- comparison of pre/post managed byte hashes;
- whether the service remained within the selected active-running boundary.

No post-run observation may mutate service state.

## 29. Failure classification and evidence

If the real action fails, the execution audit must preserve the stable failure kind emitted by `prw-agent-configure` and record read-only post-failure state.

It must not reinterpret a bounded error as success merely because the service is later observed running.

It must not expose raw external fragment bytes, managed remote values, credentials, or unrelated environment content.

## 30. No hidden repair authority

The successor may not use manual `systemctl daemon-reload`, `restart`, `try-restart`, `start`, `stop`, managed-file editing, package reinstall, unit copying, chmod/chown, environment rewriting, or external-fragment editing to make the validation pass.

If a prerequisite is not satisfied, the correct selected behavior is STOP and evidence, not repair.

Any desired repair must be selected in a separate checkpoint.

## 31. Repository mutation ceiling for the execution successor

The separately authorized real-host execution checkpoint should require no repository source change.

It may create only the normal metadata/evidence artifacts needed for closure, such as a draft PR or evidence-binding record if the established checkpoint pattern requires one.

If execution reveals a source defect requiring code changes, the checkpoint must stop. A new source-materialization selection is required; the host execution checkpoint must not silently patch Rust/Cargo/source files.

## 32. Validation expectations for C03e-UD itself

UD is docs-only.

Its repository delta must be exactly one contract path:

`contracts/C03E_UD_REAL_HOST_ACTIVE_RECONFIGURATION_VALIDATION_SELECTION_STAGING.md`

No Rust, Cargo manifest, Cargo lockfile, workflow, packaging, unit, Android, desktop, Agent, identity, network/control-plane, or repository configuration path may change.

After the final UD head is frozen, exact-head GitHub CI must be recorded. `SKIPPED` remains `SKIPPED`, never PASS.

## 33. C03e-UD evidence expectations

UD closes using the established immutable evidence pattern:

- exact predecessor/head/tree/parent;
- exact one-contract-path delta;
- contract blob and SHA-256;
- exact-head CI conclusions;
- immutable raw Markdown audit publication to canonical Drive parent `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
- byte-identical raw readback;
- exact-title singleton proof;
- exactly one Drive revision with previous revision `null`;
- metadata-only PR closure binding;
- draft/open/unmerged PR retained.

No real host mutation may occur as part of UD evidence closure.

## 34. Execution-successor evidence expectations

A separately authorized real-host successor must record enough evidence to reconstruct the mutation boundary without publishing sensitive configuration values.

At minimum bind:

- predecessor UD closure identity;
- exact UC source head/tree used to build;
- locked build command and toolchain versions;
- executable byte size/SHA-256;
- host preflight eligibility facts;
- pre-run managed leaf presence/size/SHA-256;
- pre-run loaded-topology digest;
- exact one-shot invocation surface;
- exit code and bounded event classification;
- post-run active/running facts;
- post-run managed leaf presence/size/SHA-256;
- post-run loaded-topology digest;
- whether pre/post managed bytes match exactly;
- any bounded failure kind;
- explicit non-actions;
- immutable Drive publication/readback/revision proof.

## 35. Privacy and diagnostic boundary

Canonical evidence should prefer booleans, counts, fixed classifications, and SHA-256 digests over raw host-specific paths or configuration values where exact raw disclosure is unnecessary.

The evidence must not publish credentials, secret material, arbitrary environment dumps, external fragment contents, or configured-remote payload values.

Fixed repository paths and fixed PRW managed filenames are not treated as secret, but host-specific home paths should be minimized where possible.

## 36. Existing laws retained unchanged

UD does not alter:

- exact seven Agent configuration variables and validators;
- intended-user HOME/XDG/runtime-root custody;
- `SYSTEMD_UNIT_PATH` clearing in sanitized systemd subprocesses;
- managed writer ownership of only 30/40;
- recoverable managed-file token semantics;
- external-fragment exact snapshot/reproof custody;
- target-systemd warning-fail parse gate;
- loaded `DropInPaths` topology law;
- forward `try-restart` law;
- ordinary `restart` only where rollback remains valid;
- local IPC protocol/correlation/Ready proof;
- existing readiness/IPC timeout bounds;
- no first-start authority;
- no enablement/linger authority;
- no remote-readiness claim;
- no external-fragment takeover;
- `NO_RACE_FREE_CLAIM`.

## 37. Explicit non-actions / STOP

C03e-UD performs no:

- Rust/Cargo/source materialization;
- real `prw-agent-configure` execution against host configuration;
- real managed writer invocation against host 30/40 state;
- external unit/drop-in mutation;
- `systemd-analyze verify` as part of a mutating transaction;
- daemon reload;
- service start, stop, restart, or try-restart;
- enable/disable mutation;
- linger mutation;
- package/install/deployment mutation;
- listener/network mutation;
- database/auth/credential mutation;
- repository configuration mutation;
- main-branch mutation;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- reset, rebase, squash, force-push, or history rewrite;
- destructive canonical evidence cleanup.

## 38. Selected successor statement

The immediate successor to evidence-closed C03e-UD, if separately and explicitly authorized, is one real-host execution/evidence checkpoint performing exactly one semantic-noop `prw-agent-configure reconfigure-active` transaction under the preconditions and ceilings selected here.

That successor is not authorized by creation or closure of UD itself.
