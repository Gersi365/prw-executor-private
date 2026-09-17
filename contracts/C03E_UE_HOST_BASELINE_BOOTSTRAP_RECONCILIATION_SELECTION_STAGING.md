# C03e-UE — Host baseline bootstrap/reconciliation selection

## 1. Status and boundary

`SELECTION — REAL HOST MUTATION NOT INCLUDED`

Boundary:

`HOST_BASELINE_BOOTSTRAP_RECONCILIATION_SELECTION`

C03e-UE is a docs-only selection checkpoint. It incorporates the read-only C03e-UD successor preflight and selects the staged authority required to move a host from the observed `loaded + failed + no managed 30/40` state toward one canonical `loaded + active + running + local IPC Ready` baseline without silently widening identity, deployment, first-start, enablement, linger, network, or rollback authority.

UE itself performs no production binary replacement, no identity creation, no identity drop-in installation, no managed 30/40 writer invocation, no target-systemd mutation transaction, no daemon reload, no service start/stop/restart/try-restart, no enablement/linger change, and no deployment.

## 2. Authoritative predecessor

The authoritative predecessor is evidence-closed C03e-UD / PR #668.

Exact predecessor identity:

- branch: `phase-152-c03e-ud-real-host-active-reconfiguration-validation-selection`;
- head: `270aa5bf7d14a1f86d51b9a1ed6415e3ec1c63c5`;
- tree: `b964cadc0651c6d7e5e7c794e39c16dc1a386988`;
- parent/base: evidence-closed C03e-UC `23c50e20fdfe8c481b22d062706346f8f898a189`.

UD selected semantic-noop `reconfigure-active` only for an already loaded, active, running and locally Ready Agent with already-recognized canonical managed 30/40 state. UD explicitly rejects first-start, package repair/deployment, enablement repair and linger repair.

## 3. Triggering read-only preflight

The separately authorized read-only eligibility probe stopped before any mutation because the real host did not satisfy UD.

Observed PowerCode user-service state under explicit same-user bus context:

- `LoadState=loaded`;
- `ActiveState=failed`;
- `SubState=failed`;
- `MainPID=0`;
- `InvocationID` nonempty;
- `UnitFileState=enabled`;
- loaded `DropInPaths` count `0`;
- ordered loaded-drop-in-vector digest: SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.

Managed configuration state:

- `30-agent-execution-mode.conf`: absent;
- `40-configured-remote-inputs.conf`: absent.

Identity integration state:

- `20-device-identity-credential.conf`: absent;
- persistent encrypted device-identity credential: absent.

Installed package state:

- vendor unit path: `/usr/lib/systemd/user/prw-agent.service`;
- vendor unit bytes/mode/owner: `332` / `0644` / root-owned;
- vendor unit SHA-256: `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`;
- installed Agent path: `/usr/lib/private-remote-workspace/prw-agent`;
- installed Agent bytes/mode/owner: `2865776` / `0755` / root-owned;
- installed Agent SHA-256: `4dbb114edc5ad131dd8abcf2ba798a413a249f2a3931a43b59f67b496e0d242e`.

No raw home path, remote configuration value, credential bytes, private key or external-fragment content is needed by this selection.

## 4. Exact-source disposable-build evidence

The read-only preflight also rebuilt the current source-bearing predecessor in an isolated disposable checkout pinned to C03e-UC:

- exact source head: `23c50e20fdfe8c481b22d062706346f8f898a189`;
- exact source tree: `c8b3f7b2e04fa12cc80483573fcee84bd925fac3`;
- tracked source clean before and after build;
- `rustc 1.97.1 (8bab26f4f 2026-07-14)`;
- `cargo 1.97.1 (c980f4866 2026-06-30)`.

Exact administrative build command:

`cargo build --locked -p prw-agent-systemd-orchestration --bin prw-agent-configure`

Result:

- PASS;
- binary bytes: `104998864`;
- binary SHA-256: `323a5998331d83d2284f5d941723f70ce9db2823fa22cd37cbc66ac86b52ca02`.

One exact-source Agent build was also performed read-only with:

`cargo build --locked -p prw-agent --bin prw-agent`

Result:

- PASS;
- binary bytes: `127541888`;
- binary SHA-256: `0f47ae098ad02bc48588ae37b6a4e609bdc1ff819a5075f854b0017e6ba1aad9`.

That one disposable debug-build hash is evidence of the observed build invocation only; UE does not promote it to a release/package payload identity or claim all materially different Cargo invocations are byte-identical.

## 5. Historical installed-Agent provenance finding

The currently installed Agent hash `4dbb114e...` is not treated as current-source provenance merely because the file is present and root-owned.

Repository audit history binds that exact hash to the historical Phase 126 identity-aware candidate Agent. Phase 126-A04 recorded the same candidate SHA-256 and an earlier source/activation manifest, and Phase 126-A05 required that historical candidate hash as the installed identity-aware payload before the then-selected identity transaction.

The current exact-source preflight produced different bytes under its observed UC build invocation. Therefore current-source deployment provenance for the installed Agent is not established.

UE does not infer that a root-owned installed payload is current merely from location, mode, unit enablement or prior historical acceptance.

## 6. Fresh archaeology: existing command surface

Exact current `prw-agent-configure` source exposes only two administrative actions:

- `write`;
- `reconfigure-active`.

`write` owns managed-file mutation only. It does not call systemctl, reload, start, restart, enable or alter linger.

`reconfigure-active` deliberately rejects inactive/failed state before writer mutation and uses `try-restart` only because its precondition is already-active service state.

There is no current bounded `first-start`, `bootstrap`, `bootstrap-local-only`, `recover-failed`, or equivalent administrative action.

Therefore the observed failed host cannot be made active by reinterpreting `reconfigure-active` or by passing a hidden/free-form systemctl verb.

## 7. Fresh archaeology: identity provisioning gap

`prw-device-identity-provisioning` is creation-only identity provisioning.

Its current production function:

- validates intended-user state/config roots;
- requires both the final encrypted credential and the identity drop-in destination to be absent;
- generates a new P-256 private identity locally;
- encrypts it through fixed `/usr/bin/systemd-creds` custody;
- commits and fsyncs the encrypted credential;
- returns the public SPKI SHA-256 plus encrypted credential path.

Current source does **not** create or commit `20-device-identity-credential.conf`.

The `DEVICE_IDENTITY_DROPIN_RELATIVE_PATH` constant is currently used as a collision/absence guard, not as an installation action.

Therefore invoking the existing provisioner alone would not complete systemd identity integration for a future Agent start.

## 8. Fresh archaeology: package/deployment gap

The repository still contains the Phase 107 non-activating package-file transaction contract for:

- `/usr/lib/private-remote-workspace/prw-agent`;
- `/usr/lib/systemd/user/prw-agent.service`.

That contract selects staged exact bytes, root-owned final metadata, atomic replacement, bounded rollback and no service activation.

It explicitly does not provide a current package-manager database implementation, release-signature mechanism, privileged installer implementation or real-host deployment authority.

No current narrow privileged deployment helper for replacing the root-owned Agent payload was found in the exact source tree.

UE therefore rejects an ad-hoc `sudo cp`, `install`, shell overwrite, broad NOPASSWD grant, stored sudo password, desktop privileged pass-through, or hidden manual replacement as a substitute for a selected deployment boundary.

## 9. Decision summary

UE selects:

`STAGED_BASELINE_BOOTSTRAP / CURRENT_SOURCE_PROVENANCE_BEFORE_START / NON_ACTIVATING_PACKAGE_RECONCILIATION_FIRST / IDENTITY_CIPHERTEXT_AND_20_DROPIN_AS_EXPLICIT_PREREQUISITE / EXPLICIT_LOCAL_ONLY_INITIAL_MODE / NO_CONFIGURED_REMOTE_BOOTSTRAP / MANAGED_WRITE_BEFORE_MANAGER_MUTATION / FIRST_START_SEPARATELY_MATERIALIZED / TARGET_VERIFY_BEFORE_RELOAD / EXTERNAL_FRAGMENT_CUSTODY_RETAINED / LOADED_TOPOLOGY_PROOF_RETAINED / ONE_FORWARD_DAEMON_RELOAD / ONE_FORWARD_START / BOUNDED_FAILED_START_RECONCILIATION / NO_ENABLEMENT_OR_LINGER_MUTATION / NO_AUTOMATIC_RETRY / NO_AD_HOC_SHELL_PRIVILEGE / NO_IDENTITY_ROLLBACK_AFTER_DURABLE_PROVISIONING / NO_RACE_FREE_CLAIM`

The bootstrap is intentionally decomposed into independently gated checkpoints. UE selects no one-shot mega-transaction that combines privileged deployment, private-key creation, drop-in installation, managed configuration mutation and first process activation under one undifferentiated approval.

## 10. Canonical target baseline

The eventual target baseline selected by UE is:

1. root-owned packaged Agent payload has separately proven current-source/release provenance;
2. vendor unit remains the exact selected PRW package unit unless a separately authorized package transaction replaces it with exact evidence-bound bytes;
3. one durable device identity exists under existing credential custody;
4. exact `20-device-identity-credential.conf` integration exists and references only that fixed intended-user encrypted credential path;
5. exact canonical `30-agent-execution-mode.conf` selects `local_only`;
6. `40-configured-remote-inputs.conf` is absent;
7. target-systemd verify passes;
8. user manager has reloaded the exact desired topology;
9. `prw-agent.service` is loaded, active and running;
10. MainPID is nonzero and InvocationID is nonempty;
11. local IPC reports Ready using the existing supported protocol;
12. loaded topology and external-fragment custody satisfy the existing UA/UC laws.

This is a local operational baseline, not remote-production readiness.

## 11. Why `local_only` is selected for bootstrap

`local_only` is an existing explicit execution mode, not a fallback and not a default inferred by Agent source.

TT/TQ establish that local-only:

- requires no configured-production `PRW_REMOTE_*` bundle;
- performs no configured-remote facade call;
- constructs no configured-remote Tokio driver;
- does not invent peer identity, bind address, scheduling authority or remote endpoint authority.

The observed host has no existing canonical 30/40 desired state from which UD semantic-noop configuration could be derived.

UE therefore explicitly selects `local_only` as the new bootstrap desired state. This is an explicit checkpoint decision, not an implicit missing-mode default.

UE does **not** select `configured_remote` because no complete six-value remote bundle is authorized by the read-only preflight and no value may be invented from shell state, package defaults, old chat context or unrelated configuration.

## 12. Managed bootstrap configuration law

The selected eventual managed state is exactly:

- canonical `30-agent-execution-mode.conf` containing the existing exact `local_only` semantic rendering;
- `40-configured-remote-inputs.conf` absent.

The existing managed writer remains the sole owner of 30/40.

No future bootstrap step may hand-write 30/40 with shell redirection, arbitrary template substitution or direct filesystem commands as a replacement for the evidence-closed writer custody.

## 13. Identity integration law

Identity and managed mode custody remain separate.

The future identity integration must bind the existing fixed systemd credential name to the fixed intended-user encrypted credential path using an exact PRW-owned `20-device-identity-credential.conf` representation derived only from validated XDG state/config roots and fixed relative names.

The semantic shape remains the existing Phase 124/126 systemd service-credential integration:

```ini
[Service]
LoadCredentialEncrypted=prw.device-identity.private-key.v1:<validated-fixed-encrypted-credential-path>
```

The concrete path is derived from the intended user's validated state root plus the fixed `private-remote-workspace/credentials/device-identity-private-key-v1.cred` relative path. No arbitrary caller-supplied credential name or arbitrary target path is selected.

## 14. Identity integration source gap must be closed before real provisioning

Because current `prw-device-identity-provision` commits the encrypted credential but not `20-`, UE forbids real first provisioning under the current incomplete integration surface as the next mutation.

A later source-materialization checkpoint must first provide bounded identity-drop-in installation/reconciliation under the provisioning crate's existing filesystem custody.

The expected source ceiling for that dedicated source checkpoint is at most:

`crates/prw-device-identity-provisioning/src/lib.rs`

No Cargo manifest/lockfile, Agent runtime, desktop, Android, network/control-plane, database/auth, vendor-unit or orchestration widening is expected merely to complete 20-drop-in integration.

If implementation requires another path, that source checkpoint must STOP and return to selection.

## 15. Required identity-integration source behavior

A future narrow identity-integration source checkpoint should minimally:

1. retain creation-only first-identity semantics;
2. retain fixed P-256 generation and fixed `systemd-creds` encryption authority;
3. retain no existing-key import/export;
4. derive the encrypted credential and 20-drop-in destinations only from validated intended-user XDG roots plus fixed relative paths;
5. stage exact 20-drop-in bytes without following symlinks;
6. require regular-file/directory ownership and mode custody consistent with existing provisioning rules;
7. use no-replace commits for first provisioning;
8. fsync committed state and relevant parent directories;
9. keep private PKCS#8 material zeroizing/in-memory-only before encryption;
10. never call systemctl, loginctl, daemon reload, start, restart, enablement or linger operations;
11. never modify 30/40;
12. return bounded path/content-free diagnostics.

## 16. Identity partial-state and crash law

UE does not claim cross-file crash atomicity between encrypted credential persistence and 20-drop-in persistence.

The source successor must fail closed on ambiguous pre-existing partial state rather than generating a second identity, overwriting a foreign file, or silently deleting durable identity material.

If an in-process second-step commit fails and the implementation can prove the first committed object is exactly the object created by that same transaction, bounded same-transaction cleanup may be selected and tested.

A process crash after one durable commit is different: on the next run, pre-existing partial state must be surfaced for explicit reconciliation. No automatic key rotation, replacement or deletion is selected.

Once a durable production identity is successfully evidence-closed, later bootstrap/activation failure does not authorize automatic identity deletion or regeneration.

## 17. Deployment reconciliation precedes first activation

Before any future first-start action, the root-owned installed Agent payload must have exact evidence-bound provenance suitable for the current source/release checkpoint.

The currently installed `4dbb114e...` payload is a known historical Phase 126 candidate hash. That historical acceptability does not prove it contains later current-source execution-mode, configured-remote, systemd orchestration or other subsequently materialized source.

Therefore a separately gated deployment/reconciliation checkpoint must decide and prove the exact current Agent payload before first activation.

UE does not itself select the final release build profile, package signature mechanism or privileged replacement mechanism.

## 18. Deployment transaction requirements

The future deployment selection must preserve at least the existing Phase 107 package-file laws:

- exact fixed executable and vendor-unit destinations;
- exact candidate bytes and hashes frozen before privileged mutation;
- root-owned `0755` executable and root-owned `0644` vendor unit;
- destination must be absent or proven PRW-managed prior package payload;
- private same-filesystem staging;
- exact rollback copy of prior managed payload before upgrade;
- atomic replacement;
- post-write byte/mode/ownership readback;
- no service activation in the package-file transaction;
- no user-home mutation by the privileged installer;
- no broad privilege grant or stored password.

Deployment success does not imply daemon reload, start, Ready or identity success.

## 19. Managed local-only materialization is a separate mutation boundary

After current Agent deployment provenance and identity integration are independently evidence-closed, the next baseline materialization may use exact evidence-bound `prw-agent-configure write` with desired mode `local_only`.

That action must remain non-activating exactly as TX selected:

- validate intended-user context;
- validate exact local-only desired configuration;
- apply only managed 30/40 custody;
- postcommit readback/verification;
- no daemon reload;
- no start/stop/restart/try-restart;
- no enablement/linger change;
- no identity mutation;
- no deployment.

A successful write proves only canonical managed files.

## 20. No ambient bootstrap configuration source

The future local-only write invocation must use a cleared/minimal explicit environment and the exact explicitly selected `PRW_AGENT_EXECUTION_MODE=local_only` value.

No configured-remote variables are supplied.

No mode value is read from an ambient login shell, `.env`, stale process environment, previous service environment, old audit, package default or test fixture.

`local_only` is selected here once as the bootstrap configuration authority.

## 21. First-start orchestration remains a source gap

Even after deployment, identity integration and 30/40 materialization, current `prw-agent-configure` still has no first-start action.

UE explicitly rejects using `reconfigure-active` for this purpose because that would defeat its active/running precondition and `try-restart` safety law.

UE also rejects bypassing the orchestration layer with an undocumented sequence of shell-issued `systemctl daemon-reload` and `systemctl start` commands.

A later fresh selection/source checkpoint must materialize a bounded first-start orchestration surface before real activation.

## 22. Selected future first-start semantic action

The future first-start action should be semantically dedicated to establishing the selected local-only baseline, for example an internal/public administrative action equivalent to:

`bootstrap-local-only`

The exact stable token/name remains for that later source checkpoint to freeze if implementation archaeology requires a different bounded representation.

It must not accept arbitrary systemctl verbs, arbitrary service names, arbitrary executable paths, arbitrary environment maps, or configured-remote values.

## 23. Future first-start preconditions

Before any first-start manager mutation, the future orchestration must prove at least:

1. same intended unprivileged user context;
2. valid explicit HOME/XDG/runtime-root custody;
3. exact target unit is `prw-agent.service`;
4. `LoadState=loaded`;
5. service is not already active/running;
6. `MainPID=0` before first start;
7. nonempty stable `UnitFileState` captured as baseline metadata;
8. expected exact package Agent provenance has already been evidence-closed;
9. encrypted device identity exists under selected custody;
10. exact 20 identity integration exists and is canonical;
11. canonical managed 30 selects local-only;
12. managed 40 is absent;
13. external target fragment inventory is safe and custody-captured;
14. no competing external environment assignment conflicts with selected PRW variables.

Failure of any precondition stops before daemon reload/start.

## 24. Future target-systemd gate

Before first daemon reload, the future bootstrap must retain the exact target verification command already selected for active reconfiguration:

`/usr/bin/systemd-analyze --user --recursive-errors=no --man=no --generators=no verify prw-agent.service`

under sanitized intended-user environment.

Warnings/errors remain fail-closed according to existing source.

No manager mutation occurs when this gate fails.

## 25. Future external-fragment custody

The UC exact external-fragment custody remains relevant to first activation.

The future first-start lane must capture/reprove the external baseline around manager/process transitions. The inventory includes the main unit and non-managed drop-ins, including canonical `20-device-identity-credential.conf`; managed 30/40 remain excluded only under their exact managed ownership law.

The custody token remains in-process, exact-byte/metadata based and non-persistent.

No external file is mutated or locked as universal authority.

`NO_RACE_FREE_CLAIM` remains authoritative.

## 26. Selected future first-start forward sequence

The later first-start orchestration selection should preserve this ordering:

1. prove the complete inactive/failed-but-not-running precondition set;
2. capture exact baseline systemd status/topology metadata;
3. capture external-fragment custody;
4. run target-systemd verify against already-materialized canonical 20/30 and absent 40;
5. reprove external custody immediately before manager mutation;
6. execute exactly one `systemctl --user daemon-reload`;
7. re-read systemd status/topology and prove exact desired loaded 20/30 membership, absent managed 40 and unchanged selected UnitFileState law;
8. reprove original external custody;
9. execute exactly one `systemctl --user start prw-agent.service`;
10. wait only within the existing bounded readiness interval;
11. require loaded + active + running, nonzero MainPID and local IPC Ready/current protocol;
12. reprove loaded topology and original external custody before success.

No automatic retry is selected around reload, start or readiness.

## 27. Successful-path mutation ceiling

A future successful first-start checkpoint may perform at most:

- one forward daemon reload;
- one forward start of `prw-agent.service`.

It may not:

- enable or disable the unit;
- alter linger;
- execute ordinary restart or try-restart on the successful first-start path;
- mutate identity or 20/30/40 files during the manager/process action itself;
- replace package files;
- modify external fragments;
- activate configured-remote networking.

Package, identity and managed-file materialization must already be independently evidence-closed before this manager/process gate.

## 28. First-start failure before `start`

If target verify, pre-reload custody, daemon reload, post-reload topology or post-reload custody fails before a start attempt:

- do not start the service;
- do not retry daemon reload automatically;
- do not delete the durable identity;
- do not remove canonical 20/30 merely to imitate the earlier failed baseline;
- preserve the materialized baseline artifacts for read-only reconciliation;
- return bounded terminal failure.

Because deployment/identity/configuration are separately committed checkpoints, first-start failure does not roll those durable checkpoints back implicitly.

## 29. First-start failure after a start attempt

A forward start attempt may fail synchronously or may produce a process that later fails readiness/topology/custody checks.

UE does not select an automatic second start or restart.

A later first-start source checkpoint must define one bounded safety reconciliation path. At most one stop may be selected if direct observation proves the forward start produced an active/activating process that must be quiesced after terminal readiness/topology/custody failure.

That stop is safety containment, not restoration of the historical `failed` state and not retry authority.

No `reset-failed`, restart loop, enablement repair, linger repair, identity deletion, package rollback or configured-remote fallback is selected.

If stop/containment fails or command completion is ambiguous, mutation stops and explicit manual reconciliation is required.

## 30. No exact rollback-to-failed claim

The observed baseline is currently `failed/failed` with `MainPID=0`.

UE does not claim that a future failed first-start transaction can or should restore the exact previous failure result, prior InvocationID, prior journal state or prior systemd failure counters.

The selected safety goal after failed first-start is bounded quiescence plus exact evidence of durable package/identity/configuration state, not synthetic recreation of historical failure metadata.

## 31. Unit enablement and linger remain observations only

The current unit is observed `enabled`.

UE does not treat that observation as authority to enable/disable the unit in bootstrap.

Linger is not mutated by any selected stage.

If a future execution requires enablement or linger change to satisfy a product requirement, it must STOP and return to a fresh selection checkpoint rather than silently widening baseline bootstrap.

## 32. No remote readiness claim

A successful UE-derived local baseline proves only local Agent service readiness.

It does not prove:

- configured-remote execution;
- remote peer identity/configuration;
- transport reachability;
- external provider health;
- remote authentication;
- control-plane readiness;
- database readiness;
- capability admission beyond existing local IPC proof.

Configured-remote activation remains separately gated.

## 33. Evidence requirements for every future mutation stage

Each future mutation checkpoint must independently freeze and publish:

- exact predecessor identity;
- exact source/build provenance relevant to that stage;
- exact pre-state readback;
- exact authorized mutation ceiling;
- command/action identity;
- exit/result classification;
- exact post-state readback;
- rollback/reconciliation disposition if failure occurred;
- explicit non-actions;
- immutable canonical evidence and post-publication closure binding.

No later stage may use a previous stage's approval as implicit authority for its own mutation.

## 34. Immediate successor boundary

UE does **not** authorize any real-host mutation or source implementation merely by selecting the staged bootstrap architecture.

The immediate successor should be a fresh, separately approved checkpoint selecting the exact current-Agent deployment provenance and narrow privileged non-activating package-file reconciliation mechanism needed before first activation.

That successor must reconcile the root-owned installed `prw-agent` payload against current source/release authority while preserving the Phase 107 package transaction and without daemon reload, service start/stop/restart/try-restart, enablement/linger change, identity creation or managed 20/30/40 mutation.

Only after deployment provenance is evidence-closed should the identity-integration source gap be materialized and independently validated.

## 35. Later staged successor order

Subject to separate approval at every boundary, the selected dependency order is:

1. current-Agent deployment provenance/privileged non-activating reconciliation selection and any required materialization;
2. evidence-closed non-activating package reconciliation on the host;
3. identity 20-drop-in integration source materialization;
4. evidence-closed real first-identity provisioning/integration with no manager mutation;
5. evidence-closed local-only 30/40 managed write with no manager mutation;
6. first-start orchestration selection/source materialization;
7. evidence-closed real first-start transaction;
8. only then return to UD-style semantic-noop active reconfiguration validation if still useful.

UE does not pre-authorize any numbered successor.

## 36. Validation expectations for UE

UE is docs-only. Its own closure should require:

- exact one-contract-path delta from evidence-closed UD;
- exact predecessor/head/tree/parent proof;
- `git diff --check`;
- exact-head repository CI registered for the docs-only head;
- `SKIPPED` checks represented as skipped, never PASS;
- immutable raw Markdown publication in the canonical Drive evidence folder;
- byte-identical raw readback;
- exact-title singleton proof;
- one-revision lineage with previous revision `null`;
- metadata-only PR closure binding;
- PR retained open, draft and unmerged.

## 37. Explicit non-actions / STOP

C03e-UE performs no:

- Rust/Cargo/runtime source materialization;
- package-file replacement;
- privileged install/helper execution;
- real device-identity key generation;
- encrypted credential persistence;
- 20 identity-drop-in creation;
- 30/40 managed writer invocation;
- target verify as part of a mutating host transaction;
- daemon reload;
- service start, stop, restart or try-restart;
- enable/disable mutation;
- linger mutation;
- configured-remote activation;
- network/listener mutation;
- database/auth/control-plane mutation;
- external fragment mutation;
- main-branch mutation;
- repository configuration mutation;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- reset, rebase, squash, force-push or history rewrite;
- destructive canonical evidence cleanup.

STOP after evidence closure of this selection. The next deployment-provenance checkpoint remains separately gated.
