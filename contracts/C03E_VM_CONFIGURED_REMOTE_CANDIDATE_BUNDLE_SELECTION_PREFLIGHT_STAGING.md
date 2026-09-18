# C03e-VM Configured-Remote Candidate Bundle Selection Preflight

## Status

`READ_ONLY_CONFIGURED_REMOTE_CANDIDATE_BUNDLE_SELECTION_PREFLIGHT_BLOCKED_ON_EXPLICIT_SIX_VALUE_SELECTION — VALIDATED — EVIDENCE_PENDING — ACTIVE_LOCAL_ONLY_UNCHANGED — NO_REMOTE_VALUES_INVENTED — NO_PRODUCTION_MUTATION`

Boundary:

`CONFIGURED_REMOTE_CANDIDATE_BUNDLE_OFFLINE_SELECTION_PREFLIGHT`

Date: `2026-09-18`

## 1. Authority and predecessor

Repository: `Gersi365/prw-executor-private`.

Authoritative predecessor: evidence-closed C03e-VL / PR #702 at exact head:

`cdf31be822f7d2d837ad402e500902480e40637b`

Exact predecessor tree:

`fa1dfea5e9fa8116bfe5ce7c7cfb9cccf21333cd`

Canonical `main` at this preflight remained:

`a7ffafd6a6d5a032dd8290eec24df1349bade6cc`

No `C03e-VM` branch or PR existed before this checkpoint was opened.

## 2. C03e-VL hard boundary retained

C03e-VL closed first Agent activation in explicit `local_only` mode.

Its STOP remained:

`STOP_BEFORE_ANY_CONFIGURED_REMOTE_INPUT_WRITE_OR_RECONFIGURE_ACTIVE_OR_NETWORKING_MUTATION`

The immediate successor scope is therefore selection/read-only work only. C03e-VM does not authorize any configured-remote production mutation.

## 3. Fresh production durability proof

At `2026-09-18T19:51:28+02:00`, fresh read-only host inspection showed:

- `LoadState=loaded`;
- `ActiveState=active`;
- `SubState=running`;
- `MainPID=3197123`;
- executable `/usr/lib/private-remote-workspace/prw-agent`;
- `Result=success`;
- `NRestarts=0`;
- `ExecMainCode=0`;
- `ExecMainStatus=0`;
- `NeedDaemonReload=no`;
- `UnitFileState=enabled`;
- manager-loaded drop-ins exactly fixed `20-device-identity-credential.conf` plus managed `30-agent-execution-mode.conf`;
- manager-visible `PRW_AGENT_EXECUTION_MODE=local_only`;
- managed `40-configured-remote-inputs.conf` absent;
- exactly one Agent process;
- zero queued user-manager jobs;
- zero observed Agent TCP connections/listeners;
- zero observed Agent UDP connections/listeners;
- exactly one observed Unix listener;
- zero journal startup-failure/main-exit events since the first activation transaction.

The historical `pgrep` warning for names longer than fifteen characters makes that command unsuitable as sole configure-process evidence; no configuration mutation was attempted or needed in this checkpoint.

## 4. Existing selection authority does not contain values

C03e-US remains the latest explicit runtime-prerequisite selection authority before the real host bootstrap sequence.

US selected:

`PRW_AGENT_EXECUTION_MODE=local_only`

and explicitly stated that configured-remote bootstrap was not selected and remote production values must not be invented.

US required managed `40-configured-remote-inputs.conf` absent in the selected local-only desired state and selected no `PRW_REMOTE_*` value.

No later checkpoint through C03e-VL selected an exact production six-value configured-remote bundle.

## 5. C03e-TU conditional custody authority

C03e-TU remains conditional authority for configured-remote systemd input custody only.

It selects:

- managed per-user leaf `40-configured-remote-inputs.conf`;
- exactly six fixed non-secret `PRW_REMOTE_*` `Environment=` assignments;
- deterministic assignment order;
- canonical escaping and exact semantic-value round trip;
- no `EnvironmentFile=`;
- no systemd credential for these non-secret inputs;
- complete validated `30` plus `40` state before configured-remote activation;
- no default, normalization, inference or fallback.

TU does not provide a production value-set for the six fields.

## 6. Exact six-value semantic contract

Exact C03e-VL lineage source `crates/prw-agent-configuration/src/lib.rs`, blob:

`1a5ef55f4e7036da0f5ac12924f6c08abfdfe525`

requires `ConfiguredRemoteBundle::try_new(...)` to receive exactly:

1. `PRW_REMOTE_BIND_ADDR`;
2. `PRW_REMOTE_PEER_DEVICE_ID`;
3. `PRW_REMOTE_MAX_ACTIVE_WORKERS`;
4. `PRW_REMOTE_APPLICATION_LEASE_SECONDS`;
5. `PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS`;
6. `PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS`.

Validation is fail-closed and field-specific.

## 7. Exact field laws

`PRW_REMOTE_BIND_ADDR`:

- must be non-empty exact `SocketAddr` text;
- unspecified addresses are rejected;
- multicast addresses are rejected;
- IPv4 limited broadcast is rejected;
- no DNS/interface/route/default inference is selected.

`PRW_REMOTE_PEER_DEVICE_ID`:

- must satisfy existing `DeviceId` semantics;
- empty or whitespace-only identity is rejected;
- process peer intent is configuration and does not become current peer authority by itself.

`PRW_REMOTE_MAX_ACTIVE_WORKERS`:

- strict ASCII decimal;
- target-`usize` representable;
- strictly positive;
- zero rejected;
- no host auto-sizing/default is selected.

`PRW_REMOTE_APPLICATION_LEASE_SECONDS`:

- strict ASCII decimal `u64`;
- strictly positive;
- maximum `3600` seconds;
- no default is selected.

`PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS`:

- strict ASCII decimal;
- target-`usize` representable;
- zero is semantically permitted;
- no default is selected.

`PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS`:

- strict ASCII decimal;
- target-`usize` representable;
- zero is semantically permitted;
- no default is selected.

## 8. Canonical renderer law

Only after all six semantic values validate may `render_configured_remote_drop_in(...)` render canonical `40-configured-remote-inputs.conf`.

Exact output order is the six variables listed above.

The renderer:

- begins with `[Service]`;
- emits each whole `NAME=value` assignment inside double quotes;
- escapes backslash as doubled backslash;
- escapes double quote;
- emits `%%` for a literal percent;
- rejects control characters;
- preserves other printable Unicode semantic text.

No canonical `40` candidate can be rendered from missing semantic values.

## 9. Authority search result

Project-context searches for `configured_remote`, `PRW_REMOTE_BIND_ADDR`, `remote peer device`, and `40-configured-remote-inputs.conf` returned no indexed candidate value-set.

GitHub PR searches located the historical source/custody checkpoints, including C03e-TU and earlier per-field source/materialization checkpoints. Those checkpoints establish source provenance and validators, but their explicit exclusions retain no concrete deployed value where relevant.

A GitHub PR search for a later `six-value configured remote bundle` returned no result.

Google Drive searches located canonical audits for TU, writer/source materialization, individual field provenance and related production seams. They did not establish an exact selected six-value candidate bundle that supersedes the C03e-US `local_only / no remote value invention` selection.

These searches are evidence that no authoritative bundle was identified by the performed queries; they are not an exhaustive claim about every possible external artifact.

## 10. Candidate selection matrix

Current authoritative candidate status:

- `PRW_REMOTE_BIND_ADDR`: `UNSELECTED`;
- `PRW_REMOTE_PEER_DEVICE_ID`: `UNSELECTED`;
- `PRW_REMOTE_MAX_ACTIVE_WORKERS`: `UNSELECTED`;
- `PRW_REMOTE_APPLICATION_LEASE_SECONDS`: `UNSELECTED`;
- `PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS`: `UNSELECTED`;
- `PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS`: `UNSELECTED`.

No placeholder, localhost address, current interface address, peer guess, capacity guess, lease guess, zero, historical test fixture or inferred value is promoted to production candidate authority.

## 11. Blocker

Offline construction cannot proceed from structure alone because the selected law requires six exact semantic values and explicitly forbids defaults/inference.

Therefore C03e-VM is blocked on:

`EXPLICIT_CONFIGURED_REMOTE_SIX_VALUE_SELECTION`

This is a configuration-intent/data blocker, not a host-health blocker.

## 12. Production state intentionally unchanged

C03e-VM performs no:

- `30-agent-execution-mode.conf` write or replacement;
- `40-configured-remote-inputs.conf` creation/replacement/removal;
- `prw-agent-configure write`;
- `prw-agent-configure reconfigure-active`;
- daemon-reload;
- service start/stop/restart/try-restart/reset-failed;
- credential read/decrypt/replacement;
- enrollment/revocation mutation;
- listener/network/DNS/forwarding/relay mutation;
- registry/control-plane/database mutation;
- shell/manager environment mutation;
- `main` mutation;
- merge, ready conversion, PR close, branch deletion or history rewrite.

The active Agent remains intentionally in evidence-closed `local_only` state.

## 13. Classification

`READ_ONLY_CONFIGURED_REMOTE_CANDIDATE_BUNDLE_SELECTION_PREFLIGHT_BLOCKED / ACTIVE_LOCAL_ONLY_AGENT_STABLE / MANAGED_40_ABSENT / EXACT_SIX_FIELD_VALIDATION_LAW_REPROVED / CANONICAL_40_RENDERER_LAW_REPROVED / NO_AUTHORITATIVE_SIX_VALUE_BUNDLE_IDENTIFIED / BLOCKED_ON_EXPLICIT_CONFIGURED_REMOTE_SIX_VALUE_SELECTION / NO_REMOTE_VALUE_INVENTION / NO_PRODUCTION_MUTATION / NO_RACE_FREE_CLAIM`

## 14. STOP / next safe boundary

`STOP_BEFORE_EXPLICIT_CONFIGURED_REMOTE_SIX_VALUE_SELECTION_AND_BEFORE_ANY_40_SERIALIZATION_OR_PRODUCTION_MUTATION`

The next safe boundary is a separately authorized **selection-only** checkpoint that supplies all six exact semantic values from explicit human/policy authority.

That successor may validate the six values and, only after validation, render a canonical `40-configured-remote-inputs.conf` candidate **offline** for inspection. It must still stop before writing `30`/`40`, daemon-reload, restart, `reconfigure-active`, or any networking mutation.

`NO_RACE_FREE_CLAIM`
