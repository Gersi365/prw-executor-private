# Private Remote Workspace
# C03e-VC Production Agent Credential-Load Activation Preflight Readiness

Status: `READ_ONLY_PREFLIGHT_BLOCKED_ON_AGENT_EXECUTION_MODE_CONFIGURATION — VALIDATED — EVIDENCE_PUBLICATION_PENDING — NO_DAEMON_RELOAD — AGENT_NOT_ACTIVATED — ENROLLMENT_NOT_PERFORMED — NETWORKING_NOT_MUTATED`

Date: 2026-09-18
Repository: `Gersi365/prw-executor-private`
Repository ID: `1334911207`
Boundary: `PRODUCTION_AGENT_CREDENTIAL_LOAD_ACTIVATION_PREFLIGHT_READINESS`

## 1. Authority and predecessor

Authoritative predecessor is evidence-closed C03e-VB / PR #692 at exact head:

`13415f15a7ec6605a04e0f0b7dda87c51340eac5`

Exact predecessor tree:

`b883d3995e3a7432e4e569b1c7da76e3b0d77425`

C03e-VB created the production encrypted device identity credential and fixed `20-device-identity-credential.conf`, verified them, and explicitly stopped before user-systemd manager reload, Agent activation, enrollment or networking.

C03e-VC is authorized only as a fresh read-only activation preflight. It does not authorize `daemon-reload`, Agent start/restart, enable/disable mutation, credential decryption, enrollment, revocation or networking.

## 2. Duplicate and collision guards

Before Git materialization:

- no pre-existing C03e-VC branch existed;
- no pre-existing C03e-VC PR existed;
- exact canonical VC audit-title search in the canonical evidence folder returned no collision;
- PR #692 remained open, draft, unmerged and mergeable at exact VB head;
- `main` remained `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`, tree `63b8e59ca53797fdea6b95432e16f35eaf473604`.

## 3. Exact production unit and Agent identity

Canonical unit:

`/usr/lib/systemd/user/prw-agent.service`

Fresh host proof:

- regular file;
- mode `0644`;
- uid/gid `0:0`;
- size `332` bytes;
- SHA-256 `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`;
- non-symlink.

The on-host unit bytes match the repository unit at exact VB head.

Repository unit source blob:

`9d148087bdaad295307023090b57f1da8af90be2`

Relevant unit semantics:

- `Type=exec`;
- `ExecStart=/usr/lib/private-remote-workspace/prw-agent`;
- `Restart=on-failure`;
- `RestartSec=5s`;
- `StartLimitIntervalSec=60s`;
- `StartLimitBurst=5`;
- `WantedBy=default.target`.

Canonical Agent:

`/usr/lib/private-remote-workspace/prw-agent`

Fresh SHA-256:

`9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`

Exact Agent process count remained `0`.

## 4. Credential and fixed 20 binding

Encrypted credential remains at:

`/home/gersi365/.local/state/private-remote-workspace/credentials/device-identity-private-key-v1.cred`

Fresh metadata:

- regular file;
- mode `0600`;
- uid/gid `1000:1000`;
- size `762` bytes.

The credential contents were not read, hashed, decrypted, exported or copied by C03e-VC.

Fixed drop-in:

`/home/gersi365/.config/systemd/user/prw-agent.service.d/20-device-identity-credential.conf`

Fresh SHA-256:

`42c585ea57aed19662260f0e0421139fc5e4eab973195c3b6e4e16ab7b3ecc77`

Exact non-secret payload comparison remained `PASS`:

`[Service]`
`LoadCredentialEncrypted=prw.device-identity.private-key.v1:/home/gersi365/.local/state/private-remote-workspace/credentials/device-identity-private-key-v1.cred`

Still absent:

- `30-local-only.conf`;
- `40-production-network.conf`.

## 5. Current user-manager state

Read-only access used the already-existing user bus at `/run/user/1000/bus`.

Fresh manager state:

- `LoadState=loaded`;
- `ActiveState=failed`;
- `SubState=failed`;
- `MainPID=0`;
- `Result=exit-code`;
- `NRestarts=6`;
- `FragmentPath=/usr/lib/systemd/user/prw-agent.service`;
- manager-visible `DropInPaths=` empty;
- `UnitFileState=enabled`;
- `NeedDaemonReload=yes`.

The service is already enabled. Existing enablement link resolves to:

`/usr/lib/systemd/user/prw-agent.service`

`systemctl --user cat prw-agent.service` showed the stale cached global unit and warned that unit files changed on disk and `daemon-reload` is needed. It did not yet show the new `20` drop-in.

`systemd-analyze --user verify prw-agent.service` returned `0`; observed warnings concerned unrelated user units and did not identify a PRW parse failure. This is static syntax evidence only and is not a runtime credential-decryption/load claim.

No `daemon-reload` was executed.

## 6. Exact Agent startup contract

Exact Agent executable source at VB head:

`crates/prw-agent/src/main.rs`

blob:

`85ef70bb776d74cba2ba87d9f75e8f7eb08e2fb7`

Startup order is fail-closed:

1. load the Ubuntu enrollment signer from the systemd-delivered credential;
2. then call `load_linux_agent_execution_mode_from_env()`;
3. if execution-mode acquisition fails, emit `startup_failure kind=execution_mode` and return failure;
4. only a valid execution mode selects either the local-only or configured-remote runtime lane.

Therefore successful identity loading is necessary but not sufficient for Agent startup.

Exact execution-mode implementation is in `crates/prw-agent/src/linux_bootstrap.rs` at the same VB head.

`load_linux_agent_execution_mode_from_env()` reads the fixed source `PRW_AGENT_EXECUTION_MODE` and performs no trimming, defaulting, inference, retry or fallback. It accepts exactly:

- `local_only`; or
- `configured_remote`.

Missing, non-Unicode or invalid configuration fails closed.

## 7. Canonical execution-mode serialization

Exact configuration authority:

`crates/prw-agent-configuration/src/lib.rs`

The fixed process-configuration name is:

`PRW_AGENT_EXECUTION_MODE`

Canonical local-only drop-in bytes are source-defined as:

`[Service]`
`Environment=PRW_AGENT_EXECUTION_MODE=local_only`

Canonical configured-remote selection instead uses:

`Environment=PRW_AGENT_EXECUTION_MODE=configured_remote`

No implicit default exists.

## 8. Fresh execution-mode source proof on PowerCode

Fresh read-only manager-environment probe found:

`PRW_AGENT_EXECUTION_MODE` absent.

Also absent from the user-manager environment were the checked remote values:

- `PRW_REMOTE_BIND_ADDR`;
- `PRW_REMOTE_PEER_DEVICE_ID`;
- `PRW_REMOTE_MAX_ACTIVE_WORKERS`.

On-disk `30-local-only.conf` is absent.

On-disk `40-production-network.conf` is absent.

Therefore there is currently no source-defined execution-mode configuration that the Agent can consume after identity loading.

## 9. Activation blocker classification

A `daemon-reload` would cause the manager to consume the current on-disk drop-in state, including the valid `20` credential binding.

However, a subsequent Agent start with the currently observed state would still lack `PRW_AGENT_EXECUTION_MODE`.

Under exact Agent source this is a deterministic startup blocker at the `execution_mode` stage. Because the unit has `Restart=on-failure`, an attempted activation could also enter bounded restart attempts before systemd start limiting intervenes.

C03e-VC therefore does not classify the host as ready for Agent activation.

Technical classification:

`IDENTITY_CREDENTIAL_PRESENT / FIXED_20_BINDING_VALID / USER_MANAGER_STALE_NEEDS_RELOAD / UNIT_ALREADY_ENABLED / AGENT_EXECUTION_MODE_SOURCE_ABSENT / LOCAL_ONLY_30_ABSENT / REMOTE_40_ABSENT / ACTIVATION_BLOCKED_BEFORE_DAEMON_RELOAD_OR_START`

## 10. Production mutation statement

C03e-VC performed no production-host mutation.

It did not:

- run `systemctl --user daemon-reload`;
- start or restart `prw-agent.service`;
- enable or disable the service;
- clear failed state or reset start limits;
- read/decrypt/hash/export the encrypted credential;
- create `30-local-only.conf`;
- create `40-production-network.conf`;
- mutate enrollment/revocation;
- mutate network, DNS, forwarding or relay state.

## 11. Required predecessor correction before activation

Before any Agent activation can be authorized, a separate production configuration checkpoint must choose and materialize an explicit execution mode.

For the currently selected local-only safety boundary, the minimal source-defined non-secret configuration is the canonical `30-local-only.conf` payload:

`[Service]`
`Environment=PRW_AGENT_EXECUTION_MODE=local_only`

That materialization itself is a production-host mutation and is not authorized by C03e-VC.

Configured-remote mode would require a broader separately gated remote configuration bundle and is not selected or authorized here.

## 12. STOP

C03e-VC closes only as a read-only blocker checkpoint.

Final target classification after immutable evidence publication:

`READ_ONLY_PREFLIGHT_BLOCKED_ON_AGENT_EXECUTION_MODE_CONFIGURATION — VALIDATED — EVIDENCE_RECORDED — CLOSED — NO_DAEMON_RELOAD — AGENT_NOT_ACTIVATED — ENROLLMENT_NOT_PERFORMED — NETWORKING_NOT_MUTATED`

STOP:

`STOP_BEFORE_30_LOCAL_ONLY_CONFIGURATION_MATERIALIZATION_AND_BEFORE_USER_SYSTEMD_MANAGER_RELOAD_OR_AGENT_ACTIVATION`

`NO_RACE_FREE_CLAIM`
