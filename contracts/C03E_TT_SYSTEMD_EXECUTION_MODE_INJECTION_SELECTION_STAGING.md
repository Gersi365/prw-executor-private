# C03e-TT — systemd execution-mode injection selection staging

Status: **SELECTION STAGING**

## 1. Purpose

C03e-TT selects only the systemd/user-service custody mechanism for the already-materialized and already-activated source-level `PRW_AGENT_EXECUTION_MODE` input.

Canonical selected law:

`GLOBAL_VENDOR_UNIT_BYTE_STABLE / MANAGED_PER_USER_EXECUTION_MODE_DROPIN / DROPIN_PATH_30_AGENT_EXECUTION_MODE_CONF / EXACTLY_ONE_PRW_AGENT_EXECUTION_MODE_ENVIRONMENT_ASSIGNMENT / EXACT_LOCAL_ONLY_OR_CONFIGURED_REMOTE_TOKEN / NO_ENVIRONMENTFILE / NO_SYSTEMD_CREDENTIAL_FOR_NON_SECRET_MODE / NO_CLI_OR_SHELL_WRAPPER / NO_DEFAULT_OR_INFERENCE / IDENTITY_CREDENTIAL_DROPIN_REMAINS_SEPARATE / CONFIGURED_REMOTE_REQUIRES_SEPARATELY_SELECTED_COMPLETE_REMOTE_INPUT_BUNDLE_BEFORE_ACTIVATION / RESTART_ON_FAILURE_IS_NOT_CONFIGURATION_RETRY / DAEMON_RELOAD_RESTART_ENABLEMENT_AND_DEPLOYMENT_SEPARATELY_GATED / NO_SERVICE_MUTATION`

TT performs no service/systemd mutation, daemon reload, restart, deployment or merge.

## 2. Authoritative predecessor

C03e-TS is the exact predecessor.

Evidence-closed PR:

`#657`

Final TS head:

`33006fefed790a114a54d72bcc84448423c61fda`

Final TS tree:

`e3b1e873333a183b664c5b998d6b7704f9e34663`

Final net TS paths remain exactly:

1. `crates/prw-agent/src/main.rs`
2. `crates/prw-agent/tests/phase_102_binary_bootstrap.rs`

Exact TS source blobs:

- `main.rs` -> `85ef70bb776d74cba2ba87d9f75e8f7eb08e2fb7`
- Phase 102 -> `d8f7551fad83f7da917da26b218280eeea3b59ae`
- guard-only `linux_bootstrap.rs` -> `7c739d804deb59090f3ce7eb6491f79f956197ee`

TS final immutable closure authority:

- Drive ID `1rwrlQxCzkESn09egkTenmDsPOsK1crOP`
- filename `C03E_TS_POST_PUBLICATION_BRANCH_REPAIR_AND_FINAL_HEAD_EVIDENCE_REBINDING_AUDIT_2026-09-16.md`
- bytes `10890`
- revision `0Bz5eMiLa5v9xaUkxejlrOTRSUkVaUUYxcnF0SUZOVzRQRmlzPQ`
- exactly one revision
- previous revision `null`

TS remains draft/open/unmerged.

## 3. Exact current packaged service state

At exact TS head, repository service source is:

`packaging/systemd/prw-agent.service`

Blob:

`ac11306669cb5df92eb3a0cbbf026971fca05630`

Current service law remains:

- systemd **user** service;
- `Type=exec`;
- absolute packaged `ExecStart=/usr/lib/private-remote-workspace/prw-agent`;
- `Restart=on-failure`;
- `RestartSec=5s`;
- `TimeoutStopSec=15s`;
- `UMask=0077`;
- `NoNewPrivileges=yes`;
- journal stdout/stderr;
- no socket unit;
- no synthetic `XDG_RUNTIME_DIR`;
- no shell wrapper;
- no `EnvironmentFile=`.

The exact vendor unit currently contains no `PRW_AGENT_EXECUTION_MODE` assignment.

## 4. Current packaging boundary

Current packaging documentation is:

`packaging/systemd/README.md`

Blob:

`8ea6ffefc764f22cf863a6f69ea3f11721880c17`

The vendor unit is package-owned content under:

`/usr/lib/systemd/user/prw-agent.service`

TT preserves that package-owned unit byte-for-byte.

Execution-mode configuration is not selected for direct injection into the vendor unit.

## 5. Existing per-user drop-in precedent

Existing production identity integration already selected a user-specific managed drop-in rather than embedding a user-specific credential path in the global vendor unit.

Current provisioning source blob:

`crates/prw-device-identity-provisioning/src/lib.rs`

`0a3191229e5a9179135062e1c8d3dbdaaeeae367`

Existing identity drop-in logical path:

`${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user/prw-agent.service.d/20-device-identity-credential.conf`

Its authority remains exclusively device-identity credential custody.

TT does not modify, replace, merge with or reinterpret `20-device-identity-credential.conf`.

## 6. Selected execution-mode drop-in path

TT selects one distinct managed per-user systemd user-service drop-in:

`${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user/prw-agent.service.d/30-agent-execution-mode.conf`

The `30-` ordering deliberately follows the existing `20-device-identity-credential.conf` custody artifact while remaining semantically independent from it.

The mode drop-in is non-secret process configuration.

It is not a credential container.

## 7. Exact drop-in semantic surface

The selected drop-in has exactly one `[Service]` configuration assignment beyond the section header.

For local-only mode, exact semantic content is:

```ini
[Service]
Environment=PRW_AGENT_EXECUTION_MODE=local_only
```

For configured-remote mode, exact semantic content is:

```ini
[Service]
Environment=PRW_AGENT_EXECUTION_MODE=configured_remote
```

No third token is selected.

No empty assignment, reset assignment, alternate variable name or duplicate assignment is selected.

## 8. Exact mode authority

`30-agent-execution-mode.conf` is the only selected systemd service-layer authority for `PRW_AGENT_EXECUTION_MODE`.

It must not be simultaneously supplied by:

- the vendor unit;
- an `EnvironmentFile=`;
- another PRW-managed drop-in;
- `ExecStart=` inline environment syntax;
- a shell wrapper;
- systemd credentials;
- command-line flags;
- inferred presence of `PRW_REMOTE_*` variables.

The executable remains authoritative for strict token validation.

## 9. Missing mode remains fail-closed

TT does not introduce a service-layer default.

If the managed mode drop-in is absent and no external unsupported mutation supplies the variable, the TS executable continues to fail closed with the bounded `kind=execution_mode` startup failure.

TT does not weaken this behavior.

## 10. Why the vendor unit remains unchanged

The execution-mode decision is deployment/process-instance configuration, not package identity.

Keeping the vendor unit unchanged preserves:

- package-file transaction stability;
- upgrade rollback identity of the vendor unit;
- separation between package payload and per-user deployment configuration;
- ability to select `local_only` or `configured_remote` without replacing `/usr/lib/systemd/user/prw-agent.service`;
- existing identity-drop-in precedent.

TT therefore rejects adding `Environment=PRW_AGENT_EXECUTION_MODE=...` directly to `packaging/systemd/prw-agent.service`.

## 11. Why `EnvironmentFile=` is not selected

TT does not add an `EnvironmentFile=` parser/custody layer.

The execution mode has only two exact non-secret tokens and already has a bounded Agent parser.

A separate environment file would add:

- another mutable file authority;
- another parsing/escaping boundary;
- another path/ownership lifecycle;
- ambiguity with existing fixed process-environment source ownership.

TT therefore preserves the historical no-`EnvironmentFile=` service boundary.

## 12. Why systemd credentials are not selected

`PRW_AGENT_EXECUTION_MODE` is non-secret process configuration.

It is not:

- a private key;
- certificate material;
- password/token;
- trust anchor;
- authorization capability.

Using `LoadCredential=` or `LoadCredentialEncrypted=` for the mode would mix non-secret process configuration into secret custody without a security need.

The existing device-identity credential drop-in remains separate.

## 13. Local-only deployment semantics

If a future separately authorized activation transaction chooses `local_only`, the mode drop-in alone supplies the execution-mode variable.

The Agent then uses the TS-selected local-only lane.

No `PRW_REMOTE_*` source is required by that lane.

TT does not select or authorize removal of unrelated existing remote variables from the user-manager environment; however PRW deployment tooling must not intentionally populate configured-remote inputs for a local-only activation because that creates ambiguous stale configuration custody.

## 14. Configured-remote deployment prerequisite

TT does **not** authorize installing or activating `configured_remote` by itself.

The configured-remote lane currently requires the complete fixed non-secret process-configuration family already materialized in Agent source, including:

- `PRW_REMOTE_BIND_ADDR`;
- `PRW_REMOTE_PEER_DEVICE_ID`;
- `PRW_REMOTE_MAX_ACTIVE_WORKERS`;
- `PRW_REMOTE_APPLICATION_LEASE_SECONDS`;
- `PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS`;
- `PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS`.

A future fresh selection must define their systemd deployment custody before `30-agent-execution-mode.conf` may be activated with `configured_remote`.

No value, escaping grammar, remote-input drop-in filename, provider, config file or deployment writer for those six variables is selected by TT.

## 15. No partial configured-remote activation

A later configured-remote deployment transaction must not:

1. install `PRW_AGENT_EXECUTION_MODE=configured_remote`;
2. reload/restart the service;
3. then discover/provision missing remote inputs.

The mode and the complete separately-selected configured-remote input custody must be staged and validated before any activation/restart boundary.

TT does not yet select that transaction implementation.

## 16. `Restart=on-failure` is not retry authority

The existing vendor unit contains:

`Restart=on-failure`

TT does not reinterpret this as configuration retry, fallback or recovery authority.

A missing/invalid mode or missing/invalid configured-remote input remains a process-start failure.

A later deployment transaction must preflight configuration rather than relying on repeated systemd restarts to eventually become valid.

TT does not change `Restart=` or StartLimit settings.

## 17. Selected filesystem custody properties

A future writer for `30-agent-execution-mode.conf` must preserve the existing managed per-user drop-in custody model:

- resolve XDG config root or HOME using a separately validated deployment boundary;
- operate as the intended user;
- reject symlink traversal at PRW-managed leaves;
- reject foreign/unmanaged conflicting content;
- stage complete content before replacement;
- atomically replace only the PRW-managed mode drop-in;
- synchronize parent directory durability where the selected writer contract requires it;
- preserve the vendor unit byte-for-byte;
- preserve the device-identity drop-in byte-for-byte;
- preserve enablement and linger policy unless separately authorized.

TT selects these as future transaction obligations, not as source materialization.

## 18. File mode and secrecy classification

The execution-mode drop-in contains no secret.

TT selects a conservative per-user managed-file mode of `0600`, matching the existing PRW-managed user drop-in custody tier and preventing unrelated local users from modifying or casually inspecting deployment configuration.

This file mode does not convert the value into credential/secret material.

## 19. No service-manager activation in TT

TT performs and authorizes no:

- `systemctl --user daemon-reload`;
- service start;
- service restart;
- service stop;
- service enable/disable;
- linger mutation;
- drop-in installation on the real host;
- global unit installation/update;
- executable deployment.

All such actions remain separately gated.

## 20. No network dependency widening

TT does not add:

- `network-online.target`;
- `After=network-online.target`;
- `Wants=network-online.target`;
- socket activation;
- bind/listen directives;
- firewall/routing changes.

Configured-remote runtime/network activation remains owned by the Agent after a separately authorized deployment/start boundary.

## 21. Exact future diagnostic behavior remains source-owned

systemd configuration does not generate PRW semantic failure tokens.

TS/TQ/TR source remains authoritative for:

- `kind=execution_mode`;
- strict `local_only` / `configured_remote` parsing;
- bounded configured-production failure classes;
- local and remote terminal diagnostics.

The service layer must not duplicate or reinterpret those semantics.

## 22. Immediate successor boundary

TT does not authorize a mode-drop-in writer or real-host mutation.

After TT evidence closure, the next fresh checkpoint should select the **configured-remote systemd input bundle custody** for the six required `PRW_REMOTE_*` non-secret values.

That selection must decide, at minimum:

- exact managed artifact topology;
- exact variable/value serialization and escaping law;
- ownership/mode and conflict policy;
- local-only stale-config behavior;
- atomic staging relationship with `30-agent-execution-mode.conf`;
- preflight before daemon reload/restart;
- rollback scope.

Only after that selection is evidence-closed may a concrete non-activating writer/materialization ceiling be selected.

## 23. Local selection validation

The TT docs checkpoint should validate only repository invariants:

- exact TS predecessor head/tree;
- exact service/README/provisioning blobs;
- exactly one changed docs contract path;
- no Rust/source change;
- no packaging/systemd source change;
- no workflow/Cargo/lock change;
- `cargo fmt --all -- --check` remains PASS;
- locked Cargo metadata remains PASS;
- diff check remains PASS.

## 24. Explicit non-actions

C03e-TT does not perform or authorize:

- service unit mutation;
- per-user drop-in creation;
- remote-input configuration;
- environment mutation;
- systemd credential mutation;
- source/Rust mutation;
- `main.rs` mutation;
- Phase 102 mutation;
- Cargo/lock/workflow mutation;
- daemon reload;
- start/restart/stop;
- enable/disable;
- linger mutation;
- deployment;
- host/network mutation;
- database/control-plane mutation;
- repository configuration mutation;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- reset/rebase/squash/force/history rewrite;
- destructive evidence cleanup.

## 25. STOP

C03e-TT selects only the non-secret systemd execution-mode injection custody mechanism.

The vendor unit remains unchanged. No drop-in is created or activated. `configured_remote` remains deployment-blocked until a separately selected complete remote-input custody exists.
