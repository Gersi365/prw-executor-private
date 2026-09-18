# C03e-UR — Post-package service/runtime readiness

## Status and boundary

`READINESS — VALIDATED — RUNTIME BLOCKED ON DEVICE IDENTITY CREDENTIAL DELIVERY — DOWNSTREAM EXECUTION MODE CONFIGURATION ABSENT — SERVICE ACTIVATION NOT AUTHORIZED`

Boundary:

`POST_PACKAGE_SERVICE_RUNTIME_READINESS`

C03e-UR is a read-only checkpoint after evidence-closed C03e-UQ. It proves the newly installed Agent package identity and inspects only current service/runtime prerequisites and observed failure context. It does not install credentials or drop-ins, change manager environment, reset/start/restart/reload the service, invoke sudo, clean the private deployment stage, or perform any other runtime mutation.

## Authoritative predecessor

Authoritative predecessor is evidence-closed C03e-UQ / PR #681.

Exact predecessor:

- branch `phase-152-c03e-uq-user-attended-package-reconciliation-success`;
- head `52d0d9de344d0521923a6f766221145c6a4d5230`;
- tree `f658ff28d681cf0edd1e05ad24065d2c5a4f3337`;
- status `TRANSACTION — VALIDATED — EVIDENCE_RECORDED — CLOSED — PACKAGE RECONCILIATION SUCCEEDED — SERVICE ACTIVATION NOT AUTHORIZED`;
- retained open, draft and unmerged.

Canonical `main` remains outside this checkpoint and is not mutated.

## Installed package identity

The newly installed Agent remains:

`/usr/lib/private-remote-workspace/prw-agent`

- owner/group `root:root`;
- mode `0755`;
- bytes `11068384`;
- SHA-256 `9db768c1536868662150d9a4de321499fa61813f29fda5bc27cde4a50d61fec7`.

This is the exact UQ-selected payload and confirms package reconciliation remains successful.

The vendor unit remains:

`/usr/lib/systemd/user/prw-agent.service`

- owner/group `root:root`;
- mode `0644`;
- bytes `332`;
- SHA-256 `24f646dc96777542904cd824f1678e6c32256b64911d4a031c0315d198c0a909`.

## Vendor unit and effective manager state

The vendor unit is still the fixed unit:

```ini
[Unit]
Description=Private Remote Workspace Agent
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
ExecStart=/usr/lib/private-remote-workspace/prw-agent
Restart=on-failure
RestartSec=2s
NoNewPrivileges=yes
PrivateTmp=yes
ProtectSystem=full
ProtectHome=read-only

[Install]
WantedBy=default.target
```

Fresh effective service state:

- `LoadState=loaded`;
- `ActiveState=failed`;
- `SubState=failed`;
- `FragmentPath=/usr/lib/systemd/user/prw-agent.service`;
- `DropInPaths=` empty;
- `UnitFileState=enabled`;
- `MainPID=0`;
- `Result=exit-code`;
- `ExecMainCode=1`;
- `ExecMainStatus=1`;
- `NRestarts=5`.

The service is enabled and linger remains enabled. C03e-UR makes `NO_RACE_FREE_CLAIM`.

## Observed autonomous post-package start attempts

Read-only journal evidence after UQ shows the user manager attempted the newly installed Agent without any start/restart command from C03e-UR.

The observed startup sequence repeatedly terminated with:

`prw-agent error kind=device_identity`

After repeated on-failure restart attempts, systemd reported the start request repeated too quickly and left the service failed. `NRestarts=5` was observed.

C03e-UR did not cause these attempts and performs no manager action in response.

## Exact first startup gate

At exact UQ source head, `crates/prw-agent/src/main.rs` first calls the production device-identity loader before execution-mode or runtime bootstrap.

The Linux device-identity custody source reads exactly the systemd service-credential environment source:

- environment name `CREDENTIALS_DIRECTORY`;
- fixed credential filename `prw.device-identity.private-key.v1`;
- effective path `$CREDENTIALS_DIRECTORY/prw.device-identity.private-key.v1`;
- fixed 32-byte key custody/validation.

Failure at this first gate is reported by the Agent as:

`prw-agent error kind=device_identity`

The live journal therefore matches the exact first source gate.

## Device-identity delivery is absent

The effective unit has `DropInPaths=` empty and the vendor unit has no `LoadCredentialEncrypted=` directive.

The selected production credential target is absent:

`/home/gersi365/.local/state/private-remote-workspace/credentials/device-identity-private-key-v1.cred`

The selected production drop-in target is also absent:

`/home/gersi365/.config/systemd/user/prw-agent.service.d/20-device-identity-credential.conf`

The parent production credential/drop-in locations are not currently materialized for this service.

Therefore the first proven readiness blocker is:

`BLOCKED_ON_DEVICE_IDENTITY_CREDENTIAL_DELIVERY_ABSENT`

C03e-UR does not install or reconstruct the missing production identity credential.

## Historical phase126 staging observation

A read-only search found one historical staged drop-in candidate:

`/home/gersi365/.local/state/private-remote-workspace/phase126/staging/1e0b5eec93538bcbf120fa800cf85227eeb32410/20-device-identity-credential.conf`

Observed identity:

- owner/group `gersi365:gersi365`;
- mode `0600`;
- bytes `170`;
- SHA-256 `42c585ea57aed19662260f0e0421139fc5e4eab973195c3b6e4e16ab7b3ecc77`.

Its exact content is:

```ini
[Service]
LoadCredentialEncrypted=prw.device-identity.private-key.v1:/home/gersi365/.local/state/private-remote-workspace/credentials/device-identity-private-key-v1.cred
```

No `device-identity-private-key-v1.cred` file was found anywhere under the searched current PRW state/config roots.

Earlier historical path assumptions such as `phase126/a04/...` were not reproduced live and are not treated as available assets.

The staged drop-in alone is not sufficient runtime authority and is not installed or reused by UR.

## Exact downstream execution-mode requirement

Exact UQ source defines the fixed non-secret process configuration name:

`PRW_AGENT_EXECUTION_MODE`

The source loader accepts exactly two values:

- `local_only`;
- `configured_remote`.

The loader has no trimming, case folding, aliasing, defaulting, inference, retry, or fallback. Missing configuration fails closed.

This gate is downstream of device identity, so the live journal has not observed an execution-mode failure in the current startup sequence.

## Current execution-mode configuration is absent

Fresh read-only environment inspection showed:

- unit `Environment=` empty;
- user-manager environment contains no `PRW_*` variables;
- ordinary shell environment contains no `PRW_*` variables.

Therefore `PRW_AGENT_EXECUTION_MODE` is currently absent.

This is a proven downstream readiness prerequisite, but not the currently observed first failure. The correct classification is:

`DEVICE_IDENTITY_BLOCKS_FIRST / DOWNSTREAM_EXECUTION_MODE_CONFIGURATION_ALSO_ABSENT`

UR does not select `local_only` versus `configured_remote` and does not inject any manager or unit environment value.

## Remote configuration bundle is not yet authority

The exact configuration crate also defines fixed configured-remote names including:

- `PRW_REMOTE_BIND_ADDR`;
- `PRW_REMOTE_PEER_DEVICE_ID`;
- `PRW_REMOTE_MAX_ACTIVE_WORKERS`;
- `PRW_REMOTE_APPLICATION_LEASE_SECONDS`;
- `PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS`;
- `PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS`.

No `PRW_*` environment values are present in the effective manager/unit environment.

UR does not classify the configured-remote bundle as an active blocker because no execution mode has been selected and the process currently fails earlier at device identity. These remain later conditional prerequisites if `configured_remote` is separately selected.

## Private package-reconciliation stage remains out of runtime authority

The C03e-UL private reconciliation stage remains evidence of package custody and is not a runtime configuration source.

UR does not execute the staged reconciler, does not clean the stage, and does not read arbitrary runtime configuration from it.

## Readiness conclusion

Current package state:

`PACKAGE_PAYLOAD_READY=YES`

Current runtime/service readiness:

`RUNTIME_READY=NO`

First proven blocker:

`BLOCKED_ON_DEVICE_IDENTITY_CREDENTIAL_DELIVERY_ABSENT`

Additional downstream prerequisite:

`PRW_AGENT_EXECUTION_MODE_ABSENT`

Observed service state:

`FAILED_AFTER_DEVICE_IDENTITY_STARTUP_ERROR / NRESTARTS_5 / MAINPID_0`

Service activation, restart, reset-failed, daemon-reload, credential installation and environment configuration remain unauthorized.

## Security boundary

UR does not weaken any earlier security invariant:

- DeviceId remains logical identity, not IP identity;
- no request-controlled executable/env/cwd authority is introduced;
- no arbitrary forwarding or command authority is added;
- credential delivery must remain fixed-purpose and systemd-mediated if selected;
- no plaintext private key is placed in GitHub/Drive evidence;
- no race-free service claim is made.

## Explicit non-actions / STOP

C03e-UR performs no:

- `systemctl --user start`, `restart`, `try-restart`, `reload`, `daemon-reload`, or `reset-failed`;
- service enable/disable change;
- linger change;
- manager environment mutation;
- `PRW_AGENT_EXECUTION_MODE` selection or injection;
- remote-bundle configuration;
- credential ciphertext creation/copy/install;
- systemd drop-in creation/copy/install;
- sudo or root execution;
- vendor-unit rewrite;
- Agent package rewrite;
- private-stage cleanup;
- identity/20/30/40 activation;
- network/listener/database/auth/control-plane mutation;
- repository main mutation;
- merge;
- ready conversion;
- PR close;
- branch deletion;
- history rewrite;
- destructive evidence cleanup.

STOP after evidence closure of this read-only readiness checkpoint.

The next safe boundary is a separately authorized C03e-US selection-only production runtime-prerequisite delivery checkpoint. It must select, without installing, the authoritative method for restoring device-identity encrypted credential custody and the exact non-secret execution-mode configuration. It must treat the historical staged drop-in as historical input only, because the corresponding encrypted credential artifact is not currently present. Runtime activation remains separately gated after prerequisite materialization and validation.