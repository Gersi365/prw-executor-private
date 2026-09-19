# C03e-VS — Local Management Principal and Authority Routing Source Materialization

Status: `SOURCE_MATERIALIZATION — RUNTIME_ACTIVATION_NOT_AUTHORIZED`

Date: 2026-09-19

Repository: `Gersi365/prw-executor-private`

## 1. Exact predecessor

Evidence-closed C03e-VR is the exact predecessor:

- branch: `phase-152-c03e-vr-first-desktop-launch-read-only-local-agent-verification`;
- exact head: `75bdc8ef025d51bf7d8d90da796288cb5a1507a1`;
- exact tree: `4712be7db3ef943fe8d64cc31bdd19d20507859e`;
- PR #708 remains draft/open/unmerged;
- canonical VR completion evidence ID:
  `1eEWkaG4oexwKsjac_eRo3KQ1UBlj7DuR`.

VR closed after first desktop launch and read-only local Agent verification. The desktop
application remained running in read-only state. Command-3 management remained inactive.

## 2. Current blocker set before VS

Fresh exact-source audit at the VR head proved:

1. production `LocalAgentCommand` recognizes only:
   - code 1: `GetAgentStatus`;
   - code 2: `GetPrivateDnsConfig`;
   - code 3 remains outside the production baseline enum;

2. `management_execution` explicitly states it is not called by the production local
   server loop, Linux bootstrap or `main.rs`;

3. `LocalManagementProviderLifecycle` owns no production terminal/forwarding backend and
   is not wired into the local server loop;

4. provider backend policy explicitly performs no PTY/process/socket/thread/runtime
   operation and requires reviewed production implementation before real adapters;

5. repository-wide implementation search found:
   - `TerminalBackend`: only `SpyBackend` test implementation;
   - `PortForwardBackend`: only `SpyBackend` test implementation;

6. `LocalManagementFilesystemAuthority` requires an Agent-selected descriptor-anchored
   filesystem root, but no production root selection is wired;

7. desktop `local_management_ipc.rs` builds command-3 frames but explicitly performs no
   socket I/O, authorization, dispatch, acknowledgement or completion;

8. desktop navigation beyond Overview remains Phase-151 placeholder UI.

Therefore production management activation is not currently safe or technically complete.

## 3. Historical validated prerequisite provenance

Two older Phase-152 validation checkpoints contain source prerequisites that were never
integrated into the current production lineage.

### PR #17 — principal separation

Historical validation PR:
`Phase 152 C03 local/registry principal checkpoint validation`

Validated source introduced:

- `LocalTerminalPrincipal`;
- `TerminalSessionPrincipal::{Registry, LocalSameUid}`;
- `LocalForwardingPrincipal`;
- `ForwardingSessionPrincipal::{Registry, LocalSameUid}`;
- Agent `LocalManagementLocalPeerAuthority`;
- local/remote authority variants for terminal and forwarding;
- typed dispatch support for exact session-principal variants.

It explicitly did not activate provider backends or runtime sockets.

### PR #19 — authenticated local authority routing

Historical validation PR:
`Phase 152 C03 authenticated local authority routing validation`

Validated source introduced:

- exact required-family classifier visibility inside the Agent crate;
- `LocalManagementLocalAuthoritySet`;
- resolution of Agent/File/Transfer to Agent-owned authority;
- resolution of Terminal/Forwarding to kernel-authenticated `LocalSameUid` authority;
- `process_authenticated_linux_management_with_local_authorities`.

It explicitly did not construct backends or activate runtime sockets.

## 4. Exact blob-provenance proof

Before VS materialization, exact current VR blobs were compared with the historical bases.

Current VR equals PR #17 base exactly for:

- `crates/prw-terminal/src/lib.rs`
  - current/base blob:
    `41e043f02990cbc90ff824ab5c28f74610dddfc9`;

- `crates/prw-forwarding/src/lib.rs`
  - current/base blob:
    `8c9c03d95310cd3b5d5e4186612a0cf5537963b6`;

- `crates/prw-agent/src/local_commands/management_authority.rs`
  - current/base blob:
    `ae99264cdd53ca0ac39c430f27b6b8904b03d71c`;

- `crates/prw-agent/src/local_commands/management_typed_provider_dispatch.rs`
  - current/base blob:
    `02ca7c2eb4d7741557a589d5c9075a40054fe821`.

Current VR equals PR #19 base exactly for:

- `crates/prw-agent/src/local_commands/management_dispatch.rs`
  - current/base blob:
    `f6e5c29313210f103a1125a51c31f2a56f8c9a40`;

- `crates/prw-agent/src/local_commands/management_execution.rs`
  - current/base blob:
    `ed0d6e5ce5279f440f4efa538244b07ec424d6da`.

This proves no later source divergence exists in the exact files being recovered.

## 5. Exact selected source materialization

VS reuses the exact final historical PR #19 source blobs:

1. `crates/prw-terminal/src/lib.rs`
   - `9c5647fd1372f0d48ea23b748bddfdb4e5abe15a`;

2. `crates/prw-forwarding/src/lib.rs`
   - `37da972fcdf620c02abca29c57e875eee172522c`;

3. `crates/prw-agent/src/local_commands/management_authority.rs`
   - `86f4b4775f7cd78f30c290fdbaf3c12970e0f7af`;

4. `crates/prw-agent/src/local_commands/management_typed_provider_dispatch.rs`
   - `f67bbd7be1afe4f8bd415e87440412898bb2217a`;

5. `crates/prw-agent/src/local_commands/management_dispatch.rs`
   - `196fcad9ea03a9e1de433c6ee96dc9c2863f154f`;

6. `crates/prw-agent/src/local_commands/management_execution.rs`
   - `26d62d98826b1f61264e7fa6787dade0fa05df80`.

These are reused byte-for-byte; VS does not recreate or reinterpret the historical patch.

## 6. Semantic result

After materialization, source can distinguish:

- registry-validated remote terminal/forwarding identity;
- kernel-authenticated local same-UID terminal/forwarding identity.

Local same-UID authority is derived only from the authenticated Linux connection and
cannot be fabricated from request bytes.

The local authority set resolves the already-admitted command to the existing exact
provider-family classifier and preserves:

`admission -> exact authority resolution -> typed provider dispatch -> correlated response`

ordering.

This is a prerequisite seam only.

## 7. Still unresolved after VS

VS deliberately does not resolve:

- production local server-loop command-3 framing/dispatch wiring;
- production filesystem-root selection;
- concrete Linux terminal backend construction;
- concrete forwarding backend construction;
- forwarding egress allowlist selection;
- provider lifecycle ownership in production bootstrap;
- exact production capability policy grants for management;
- desktop management socket I/O;
- desktop response decode/acknowledgement wiring;
- replacement of Phase-151 placeholder management UI;
- configured-remote values;
- remote networking.

## 8. Production invariants during VS

The currently installed/running production state must remain unchanged:

- Agent remains active/local_only;
- managed 40 remains absent;
- installed desktop bytes remain unchanged;
- currently running desktop remains read-only;
- no new command-3 request is sent;
- no terminal process is created by PRW;
- no file/transfer mutation is initiated;
- no forwarding listener is created;
- no TCP/UDP remote activity is activated.

## 9. Explicit non-actions

VS performs no:

- production package rebuild/install/replacement;
- Agent restart/reload/start/stop/reconfiguration;
- command-3 runtime activation;
- concrete backend construction;
- PTY/shell launch;
- filesystem-root selection;
- filesystem mutation;
- transfer operation;
- forwarding listener/socket operation;
- capability-policy grant;
- configured-remote selection/write;
- networking/DNS/firewall/route mutation;
- sudo/root operation;
- PR merge/ready/close;
- branch deletion;
- reset/rebase/squash/force update/history rewrite.

## 10. Classification

`LOCAL_MANAGEMENT_PRINCIPAL_AUTHORITY_PREREQUISITE_SOURCE_MATERIALIZED / HISTORICAL_VALIDATED_BLOBS_REUSED_BYTE_EXACT / LOCAL_SAME_UID_DISTINCT_FROM_REGISTRY_IDENTITY / AUTHENTICATED_LOCAL_AUTHORITY_ROUTING_MATERIALIZED / RUNTIME_NOT_WIRED / PROVIDER_BACKENDS_NOT_CONSTRUCTED / DESKTOP_MANAGEMENT_NOT_DISPATCHED / PRODUCTION_READ_ONLY_STATE_PRESERVED / NO_COMMAND3_ACTIVATION / NO_CONFIGURED_REMOTE / NO_NETWORK_MUTATION / NO_RACE_FREE_CLAIM`

## STOP

`STOP_AFTER_LOCAL_MANAGEMENT_PRINCIPAL_AND_AUTHORITY_ROUTING_SOURCE_MATERIALIZATION_AND_BEFORE_RUNTIME_OR_PROVIDER_ACTIVATION`

A later separately gated checkpoint must select and implement a bounded first production
management runtime slice. It must explicitly resolve provider construction, policy and
runtime ownership before any command-3 request can reach a provider.

`NO_RACE_FREE_CLAIM`
