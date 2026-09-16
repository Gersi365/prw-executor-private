# C03e-TV — managed systemd configuration writer selection staging

Status: **SELECTION STAGING**

## 1. Purpose

C03e-TV selects only the source ownership, API boundary, validation reuse, conflict preflight, filesystem transaction and fail-safe commit ordering for a future managed writer that materializes the already-selected C03e-TT/C03e-TU Agent systemd configuration law.

Canonical selected law:

`NEW_PRW_AGENT_CONFIGURATION_CRATE / PRW_AGENT_INDEPENDENT_CONFIGURATION_AUTHORITY / SHARED_PURE_EXACT_VALUE_VALIDATORS / PRW_AGENT_ENV_LOADERS_DELEGATE_WITHOUT_VALIDATION_DRIFT / LINUX_SYSTEMD_WRITER_OWNS_ONLY_30_AND_40 / NO_DEVICE_IDENTITY_PROVISIONING_SCOPE_EXPANSION / NO_REMOTE_FILE_SERVICE_REUSE / EXPLICIT_INTENDED_USER_XDG_CONFIG_ROOT / PRIVATE_0700_MANAGED_DIRECTORY / MANAGED_0600_REGULAR_LEAVES / DIRECTORY_FD_AND_NOFOLLOW_CUSTODY / READ_ONLY_EFFECTIVE_UNIT_PATH_PREFLIGHT / FOREIGN_OR_AMBIGUOUS_ENVIRONMENT_CUSTODY_FAILS_CLOSED / CANONICAL_30_AND_40_BYTES / FULL_PREVALIDATION_BEFORE_STAGE / SAME_DIRECTORY_TEMP_STAGE_AND_FSYNC / CONFIGURED_REMOTE_COMMIT_40_BEFORE_30 / LOCAL_ONLY_COMMIT_30_BEFORE_40_REMOVAL / POSTCOMMIT_REOPEN_VERIFY / IN_PROCESS_EXACT_FILE_STATE_ROLLBACK / NO_TWO_FILE_ATOMICITY_CLAIM / NO_DAEMON_RELOAD_RESTART_ENABLEMENT_LINGER_DEPLOYMENT / NO_CLI_OR_ORCHESTRATOR_SELECTION / NO_SOURCE_MATERIALIZATION_IN_TV`

TV performs no Rust/source materialization, Cargo/workspace mutation, systemd file mutation, service-manager operation, deployment, merge or host/network mutation.

## 2. Authoritative predecessor

C03e-TU PR `#659` is the exact predecessor.

Exact TU identity:

- branch: `phase-152-c03e-tu-configured-remote-systemd-input-custody-selection`;
- head: `3a5ea2bc679570d1cd38fd8f1f12bc84f28cf1a8`;
- tree: `e372af6a306524632319a5c469a5a80a69020f6b`;
- status: `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- PR remains draft/open/unmerged.

TU remains authoritative for the exact managed artifacts and serialization law. TV does not reinterpret TU.

## 3. Preserved TT/TU artifact law

The package-owned vendor unit remains byte-stable:

`packaging/systemd/prw-agent.service`

TT mode custody remains:

`${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user/prw-agent.service.d/30-agent-execution-mode.conf`

TU configured-remote bundle custody remains:

`${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user/prw-agent.service.d/40-configured-remote-inputs.conf`

Existing identity custody remains separate:

`${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user/prw-agent.service.d/20-device-identity-credential.conf`

TV selects no mutation of the vendor unit or `20-device-identity-credential.conf`.

## 4. Fresh source archaeology findings

The workspace currently has no general-purpose Agent systemd/service-configuration crate.

The existing `prw-device-identity-provisioning` crate is intentionally first-device identity provisioning. It already demonstrates useful Linux custody patterns including XDG path ownership checks, private directory handling, `NOFOLLOW` reads, same-directory temporary staging, durable sync and no-overwrite commit behavior, but its semantic ownership is device identity and creation-only provisioning.

TV therefore treats that crate as a filesystem-safety precedent only. It is not widened into mutable Agent mode/remote-input configuration ownership.

The existing `prw-file-service` filesystem code belongs to the remote file-service capability domain and is not selected as a local Agent service-configuration dependency.

## 5. Current validation-drift problem

At exact TU, the execution-mode and configured-remote environment loaders are in `crates/prw-agent/src/linux_bootstrap.rs`.

The underlying exact-value parsers are private or crate-private and currently sit inside the Agent crate. A separate writer that copies those parser rules would create two semantic authorities and permit future drift.

TV rejects duplicated validator implementations.

## 6. Selected new source owner

A future materialization checkpoint must create one new workspace library crate:

`crates/prw-agent-configuration`

Package name:

`prw-agent-configuration`

This crate is selected as the single reusable source authority for:

1. exact non-secret Agent process-configuration names and token grammar;
2. pure exact-value validation needed by both Agent environment loading and managed-file preflight;
3. canonical TT/TU systemd serialization;
4. Linux-only managed `30-`/`40-` filesystem custody and file transaction mechanics.

It must not depend on `prw-agent`.

The future `prw-agent` integration may depend on `prw-agent-configuration` and delegate pure value validation to it. This dependency direction prevents a configuration writer from importing the full Agent runtime and prevents a circular dependency.

## 7. Selected pure configuration module

The future crate must expose a side-effect-free configuration-contract surface in its library root or a narrow pure module.

It must own exact symbolic names for:

- `PRW_AGENT_EXECUTION_MODE`;
- `PRW_REMOTE_BIND_ADDR`;
- `PRW_REMOTE_PEER_DEVICE_ID`;
- `PRW_REMOTE_MAX_ACTIVE_WORKERS`;
- `PRW_REMOTE_APPLICATION_LEASE_SECONDS`;
- `PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS`;
- `PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS`.

The pure surface performs no environment read, filesystem read/write, subprocess execution, systemd operation, network I/O or runtime activation.

## 8. Execution-mode parser ownership

The shared pure contract must accept only exact execution-mode text:

- `local_only`;
- `configured_remote`.

No trimming, case folding, aliases, default, inference or fallback is selected.

The future Agent environment loader may retain its current bounded missing/non-Unicode source errors, but after Unicode acquisition it must delegate exact token semantics to the shared configuration authority rather than retain an independent token parser.

Existing public Agent behavior must remain compatible unless a separately approved API migration is required.

## 9. Configured-remote exact-value bundle

The shared configuration crate must expose one validated bundle representation that retains both:

- the exact caller-supplied semantic text required for byte-preserving TU serialization; and
- typed validation projections needed to prove semantic validity.

The bundle must contain exactly the six TU values and no seventh field.

Validation remains equivalent to current Agent behavior:

- bind address: exact `SocketAddr`, rejecting empty, malformed, unspecified, multicast and IPv4 limited broadcast while retaining port `0` as valid pre-bind;
- peer device ID: existing `DeviceId` contract, rejecting empty/whitespace-only while preserving otherwise accepted exact text;
- max active workers: strict ASCII decimal, target-`usize`, non-zero;
- application lease seconds: strict ASCII decimal, `u64`, positive and within the existing remote-session lease ceiling;
- requester/rendezvous max records: strict ASCII decimal within target `usize`, including zero;
- expected-device scheduling-consumption max records: the existing strict ASCII-decimal target-`usize` law, including its current zero semantics.

Leading zeroes that current parsers accept remain valid semantic text and must remain byte-preserved in the systemd value.

## 10. Agent migration boundary

The future writer materialization checkpoint must not leave duplicated exact-value semantics in `linux_bootstrap.rs`.

The selected migration is:

1. environment acquisition and bounded source-error mapping stay in `prw-agent` where process-source ownership already exists;
2. Unicode semantic text is passed to `prw-agent-configuration` pure validators;
3. validated typed results are mapped back into the existing Agent runtime types as required;
4. tests prove historical valid/invalid cases remain behaviorally equivalent.

Application-lease runtime policy may remain Agent-owned; the shared configuration validator may validate against the existing shared lease ceiling and return an already-range-validated whole-second value that the Agent then constructs into its runtime policy without re-parsing text.

TV does not move remote-session lifecycle authority into the configuration crate.

## 11. Selected Linux writer module

The new crate must contain one Linux-only module for managed user-systemd configuration.

Logical owner name:

`linux_systemd`

The module owns only:

- resolution/validation of the intended user's managed systemd configuration root;
- canonical rendering of `30-agent-execution-mode.conf` and `40-configured-remote-inputs.conf`;
- read-only conflict inspection;
- safe staging/replacement/removal of those two managed leaves;
- exact post-commit verification and bounded file-state rollback.

It does not own `20-device-identity-credential.conf`, the vendor unit, service activation, deployment or process lifecycle.

## 12. No CLI selected

TV selects a library surface only.

No new binary, command-line grammar, desktop call site, installer call site, package script, daemon, IPC command or administrative UI is selected by TV.

A caller/orchestrator that supplies desired configuration and later decides whether to reload/restart remains separately gated.

This keeps writer materialization testable without accidentally selecting activation authority.

## 13. Intended-user/XDG root contract

The writer must operate for one explicit intended user context.

The selected managed root is the intended user's effective XDG configuration root:

`${XDG_CONFIG_HOME:-$HOME/.config}`

followed by:

`systemd/user/prw-agent.service.d`

The implementation must not silently substitute root's home, another account's home or an ambient unrelated XDG value.

Resolution inputs must be explicit/testable and must resolve to absolute paths before mutation.

The future caller that derives those inputs from login/session/process context is separately gated; the writer itself must not guess intended-user identity.

## 14. Managed directory custody

The `prw-agent.service.d` directory, if created by the future writer, uses intended-user ownership and mode `0700`.

Existing path components selected for direct PRW custody must be verified as directories, not symlinks, and owned by the intended user before leaf mutation.

The writer must use directory-file-descriptor-relative operations for the managed directory after validation rather than repeatedly resolving attacker-replaceable absolute leaf paths.

No traversal through a managed leaf symlink is allowed.

## 15. Managed leaf custody

The only mutable leaves selected for this writer are:

- `30-agent-execution-mode.conf`;
- `40-configured-remote-inputs.conf`.

Each present managed leaf must be:

- a regular file;
- not a symlink;
- owned by the intended user;
- mode `0600`;
- structurally recognized as canonical PRW-managed content before replacement or removal.

A non-regular leaf, symlink, wrong owner, wrong mode or unrecognized content fails closed before mutation.

TV selects no forced takeover, chmod repair of foreign content, unlink-and-replace of unrecognized files or destructive cleanup.

## 16. Canonical `30-` bytes

For local-only desired state, canonical bytes are exactly:

```ini
[Service]
Environment=PRW_AGENT_EXECUTION_MODE=local_only
```

with exactly one final LF.

For configured-remote desired state, canonical bytes are exactly:

```ini
[Service]
Environment=PRW_AGENT_EXECUTION_MODE=configured_remote
```

with exactly one final LF.

A present `30-` leaf is recognized as PRW-managed only when its bytes exactly match one of these two canonical forms and its metadata satisfies the managed-leaf contract.

## 17. Canonical `40-` rendering

The `40-` renderer must implement TU exactly:

```ini
[Service]
Environment="PRW_REMOTE_BIND_ADDR=<encoded-bind-address>"
Environment="PRW_REMOTE_PEER_DEVICE_ID=<encoded-peer-device-id>"
Environment="PRW_REMOTE_MAX_ACTIVE_WORKERS=<encoded-max-active-workers>"
Environment="PRW_REMOTE_APPLICATION_LEASE_SECONDS=<encoded-application-lease-seconds>"
Environment="PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS=<encoded-requester-rendezvous-max-records>"
Environment="PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS=<encoded-expected-device-scheduling-consumption-max-records>"
```

with exactly one final LF and no comments or additional directives.

For each exact semantic value the trusted renderer:

- emits `\\` for one literal backslash;
- emits `\"` for one literal double quote;
- emits `%%` for one literal percent;
- preserves all other allowed printable UTF-8 exactly;
- preserves `$` literally;
- rejects NUL/control/non-printable input before rendering.

The renderer must prove its own encode/decode round-trip in tests.

## 18. Recognized `40-` content

A present `40-` leaf is recognized as PRW-managed only when all of the following hold:

1. exact `[Service]` section shape;
2. exactly six assignments in TU order;
3. exact fixed variable names;
4. no duplicate or extra directive;
5. selected TU escaping decodes successfully;
6. each decoded semantic value passes the shared pure validator;
7. re-rendering the decoded bundle yields byte-for-byte identical file content;
8. metadata satisfies intended-user regular-file mode-`0600` custody.

Any mismatch is foreign/unrecognized and fails closed.

## 19. Read-only effective-custody preflight

Before staging either managed leaf, the writer must perform a read-only preflight for competing service configuration that could change one of the seven selected process variables.

The selected production search-path source is the local systemd user-unit search path reported by the installed systemd tooling for the intended user context, with failure to obtain a trustworthy search path treated as a fail-closed preflight error.

The implementation must inspect the effective `prw-agent.service` fragments/drop-ins visible in those paths without issuing daemon reload, start, restart, stop, enablement or linger operations.

Known package-owned vendor content, the separate recognized `20-` identity drop-in, and the recognized PRW-managed `30-`/`40-` leaves are permitted.

## 20. Competing environment custody

The preflight must fail closed on unresolved external configuration that can affect the selected seven variables.

At minimum this includes an external unit/drop-in fragment that:

- assigns any selected variable through `Environment=`;
- uses `UnsetEnvironment=` for any selected variable;
- uses `PassEnvironment=` for any selected variable;
- introduces an `EnvironmentFile=` whose effect on selected variables is not fully and safely resolved before mutation.

The implementation may conservatively reject an ambiguous fragment rather than attempt a permissive partial systemd parser.

No external configuration is silently deleted or rewritten.

Manager-global environment does not override a later explicit managed `Environment=` assignment, but unresolved final unsetting or later drop-in precedence must still fail closed.

## 21. Concurrency custody

The future writer must serialize cooperating PRW writers across one managed transaction without creating an additional persistent configuration artifact.

The selected mechanism is an advisory exclusive lock held on an opened managed-directory file descriptor for the full preflight/stage/commit/verify/rollback critical section, where supported by the target Linux filesystem.

Failure to obtain or maintain the lock fails closed.

This lock is coordination, not a security boundary against an unrelated same-user process. Metadata/content guards and post-commit verification remain mandatory.

## 22. Full prevalidation before staging

Before any temp file or managed leaf is changed, the writer must complete:

- intended-user/XDG root validation;
- managed-directory custody validation;
- current `30-`/`40-` recognition or confirmed absence;
- external effective-custody conflict preflight;
- execution-mode desired-state validation;
- complete six-value validation for configured-remote;
- TU serialization round-trip checks;
- computation of exact target bytes/existence state for both managed leaves.

Failure in any step causes zero managed-leaf mutation.

## 23. Staging mechanics

New/replacement bytes must be staged in same-directory temporary regular files created without following symlinks or overwriting an existing unrelated name.

Each staged file must:

- be created with restrictive permissions;
- be set/verified as intended-user mode `0600`;
- receive exact canonical bytes;
- be flushed/synced before commit;
- be reopened or otherwise verified as required before it can replace a managed leaf.

The managed directory must be synced across durable namespace changes.

TV selects no temporary file in `/tmp`, no cross-filesystem rename and no shell command for file replacement.

## 24. Configured-remote commit ordering

For desired state `configured_remote`, both target artifacts are fully prevalidated/staged before any managed-leaf replacement.

Commit order is exactly:

1. install/replace canonical complete `40-configured-remote-inputs.conf`;
2. install/replace canonical `30-agent-execution-mode.conf` with `configured_remote`.

This order is selected so a crash between the two commits cannot leave `configured_remote` selected while the required complete remote bundle is absent because of this transaction.

No daemon reload/restart boundary exists between those two file commits.

## 25. Local-only commit ordering

For desired state `local_only`, canonical local-only `30-` is staged first.

Commit order is exactly:

1. install/replace canonical `30-agent-execution-mode.conf` with `local_only`;
2. remove `40-configured-remote-inputs.conf` only if the existing `40-` leaf was recognized as PRW-managed.

This order is selected so a crash between the two namespace changes cannot leave `configured_remote` selected after its required remote bundle was removed by this transaction.

A crash may temporarily leave local-only mode with a stale recognized `40-` file; local-only ignores those inputs and the next writer preflight must converge the stale managed bundle to absent before any activation transaction is authorized.

## 26. No two-file atomicity claim

POSIX filesystem operations do not make the two separate managed leaf changes one indivisible rename.

TV therefore makes no false cross-file atomicity claim.

The selected safety model is:

- all validation before mutation;
- both configured-remote targets staged before commit;
- fail-safe commit ordering;
- no service-manager consumption boundary until the entire pair is verified;
- exact in-process rollback on commit failure;
- final pair verification before returning success.

A future activation checkpoint must independently require a verified complete desired state before daemon reload/restart.

## 27. Exact pre-state snapshot and rollback

Immediately before first managed-leaf mutation, the writer retains the exact recognized pre-transaction existence/bytes/metadata state of `30-` and `40-` needed for bounded rollback.

If a commit or post-commit verification fails, rollback attempts to restore only those two managed leaves to their exact pre-transaction PRW-managed existence/bytes state, in reverse-safe order.

Rollback must not alter:

- vendor unit;
- `20-device-identity-credential.conf`;
- unrelated drop-ins;
- service-manager state;
- enablement/linger;
- host/network state.

Rollback failure is surfaced explicitly; it is never reported as success.

## 28. Post-commit verification

Before returning success, the writer must reopen managed leaves without following symlinks and verify:

- expected existence/absence;
- regular-file type;
- intended-user owner;
- mode `0600`;
- exact canonical bytes;
- desired-state pair completeness.

For configured-remote, success requires canonical configured-remote `30-` plus canonical complete `40-`.

For local-only, success requires canonical local-only `30-` plus absent `40-`.

No service-manager state is consulted as proof of this file transaction.

## 29. Writer result boundary

A successful writer result proves only that the selected PRW-managed file desired state was committed and verified under this transaction contract.

It does not prove:

- daemon reload occurred;
- systemd accepted newly written files into manager state;
- service restart/start succeeded;
- Agent runtime reached readiness;
- remote reachability exists;
- deployment completed.

Those are separately gated authorities.

## 30. Explicit non-actions

C03e-TV performs and authorizes no:

- Rust/source materialization in this checkpoint;
- Cargo/workspace member addition;
- `30-` or `40-` file mutation;
- vendor service mutation;
- identity credential mutation;
- environment-file creation;
- systemd credential creation;
- daemon reload;
- service start/restart/stop;
- enable/disable;
- linger mutation;
- deployment;
- host/network mutation;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- reset/rebase/squash/force/history rewrite;
- destructive evidence cleanup.

## 31. Validation expectations for future materialization

The source-materialization successor must add focused tests proving at least:

- exact historical Agent parser acceptance/rejection parity;
- leading-zero preservation where already accepted;
- exact `DeviceId` text preservation;
- TU encode/decode round-trip including spaces, backslash, quote, percent, dollar and Unicode;
- rejection of control/NUL values;
- exact canonical `30-` recognition;
- exact canonical `40-` recognition and re-render identity;
- symlink/non-regular/wrong-owner/wrong-mode failures;
- external conflicting environment custody failure;
- zero mutation when prevalidation fails;
- configured-remote `40` then `30` commit order;
- local-only `30` then `40` removal order;
- injected second-step failure rollback;
- post-commit verification failure rollback;
- rollback-failure classification;
- no service-manager mutation.

Workspace rustfmt, Clippy, tests and build remain required for the exact materialization head.

## 32. Immediate successor boundary

After TV evidence closure, a separately approved checkpoint may materialize the selected `prw-agent-configuration` crate and migrate the Agent loaders to the shared pure validators while keeping any concrete caller/orchestrator and service-manager activation separately gated.

That materialization must stay within the TV-selected source/API/filesystem boundary and must not opportunistically redesign unrelated Agent runtime code.

TV does not authorize daemon reload, restart or deployment.

## 33. Current STOP

Keep C03e-TV docs-only, draft/open/unmerged.

STOP before any Rust/Cargo/source materialization, actual managed-file mutation, daemon reload, service restart/start/stop, enablement/linger change, deployment, merge or successor checkpoint.
