# Private Remote Workspace — Desktop Functional Management Slice C Authority Gate

Status: Phase 152-C00 authority contract locked
Date: 2026-08-17
Repository: `Gersi365/prw-executor-private`
Branch: `phase-152-desktop-functional-management`
Phase 152 baseline: `3aec5d9307edfa5abe53c4ebe169a2445599a2c0`
Validated Slice A head: `e4fdc357bbd2c06022af091c1d45b622fe79cb53`
Validated Slice B head: `7ca1f62d9fb05c998822d3185bb8cbaa8d940b42`
Parent contracts:
- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_CONTRACT.md`
- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_LOCAL_IPC_DEPENDENCY_CORRECTIVE.md`

## Purpose

Slice C is the first Phase 152 gate that may join the validated local command-3 envelope to Agent-side canonical management authority. It must not collapse local peer authentication, operation decoding, policy admission, principal construction, provider authority and success acknowledgement into one unchecked path.

The required ordering is:

```text
authenticated same-UID local session
    -> bounded local command-3 frame
    -> exact outer LocalIpcRequestId correlation
    -> canonical PRWC BridgeCommand decode
    -> exact BridgeCommand::required_capability()
    -> caller-bound PolicyEvaluator decision
    -> typed local management admission token
    -> operation-specific authority-context validation
    -> typed provider dispatch
    -> provider result validation
    -> correlated local terminal response
```

Any error before provider dispatch must prevent provider invocation. Any provider failure must prevent a successful acknowledgement.

## Audited current state

### Canonical operation authority already exists

`prw-remote-bridge` currently owns:

- `BridgeCommand` with 18 stable typed management operations;
- canonical PRWC encode/decode;
- exact `BridgeCommand::required_capability()` mapping;
- fail-closed malformed/truncated/trailing-payload handling;
- a remote authorization/dispatcher pattern in which policy denial prevents dispatch and response correlation is preserved.

Slice C must reuse the canonical `BridgeCommand` codec and capability mapping. It must not create a second terminal/files/forwarding operation enum or alternate wire representation.

### Dependency edge blocks direct Agent reuse

The current graph contains:

```text
prw-remote-bridge -> prw-agent
```

The audited source dependency is used for the narrow `BridgeCommand::AgentStatus -> LocalAgentCommand::GetAgentStatus` convenience adapter. `prw-agent` currently has no `prw-remote-bridge` dependency.

Directly adding `prw-remote-bridge` to `prw-agent` without correcting this edge would create a Cargo cycle.

GitHub code-search indexing for the private repository is not authoritative enough to prove workspace-wide adapter use. Therefore any dependency correction must first execute an exact repository `git grep` in CI and preserve that output as evidence. No public adapter removal is authorized unless the exact workspace search shows no required caller that would be broken by the change or every affected caller is explicitly included in the reviewed mutation scope.

### Existing local policy remains fail-closed for management

The production Agent bootstrap currently supplies `BoundedLocalReadPolicy::allow_local_reads()`. That policy allows only:

- `AgentStatusRead`;
- `PrivateDnsConfigRead`.

It explicitly denies terminal, file and forwarding capabilities.

Slice C must not silently broaden this production bootstrap policy. A management-capable policy selection/activation is a separate activation gate and requires explicit evidence that the policy is bound to the authenticated local principal context.

### Provider identity/authority mismatch is real

The existing terminal and forwarding brokers require registry-derived principal types:

- `TerminalPrincipal::from_registry(...)`;
- `ForwardingPrincipal::from_registry(...)`.

The local desktop boundary currently authenticates a same-UID Unix peer, not a remote `RegistryValidatedPrincipal`. Fabricating a registry principal for a local desktop request is forbidden.

The file service uses a descriptor-anchored `AnchoredFileRoot`; therefore live file dispatch additionally requires an explicit, validated local filesystem authority root. A path inside PRWC is not itself authorization to choose the host root.

These identity/root requirements are blockers for broad live provider activation and must be resolved explicitly rather than inferred.

## Slice C staging decision

Phase 152 Slice C is divided into narrowly reviewable sub-gates.

### C01 — canonical Agent decode and policy admission

C01 may implement only the pure Agent-side authority pipeline through admission-token creation:

1. correct the internal dependency edge only if CI proves the narrow adapter-use precondition;
2. allow `prw-agent` to reuse canonical `BridgeCommand::decode()` and `required_capability()`;
3. decode the already-validated Slice B command-3 envelope;
4. preserve the outer `LocalIpcRequestId` as the sole local correlation identifier;
5. derive capability exclusively from the decoded command;
6. evaluate the caller-bound `PolicyEvaluator` exactly once for that capability;
7. create a typed local management admission token only on `Decision::Allow`;
8. return a correlated Unauthorized/Invalid terminal response on fail-closed denial/codec failure where the existing framing contract permits a response;
9. perform no terminal, filesystem, transfer or forwarding provider invocation.

C01 must not modify `linux_bootstrap::run()` to grant management capabilities.

### C02 — authority-context adapters and provider-neutral dispatcher proof

C02 may proceed only after C01 is fully green.

C02 must define explicit typed authority contexts for each provider family before provider invocation:

- terminal authority context;
- file/root authority context;
- forwarding authority context;
- transfer authority context where applicable.

For terminal/forwarding, C02 must resolve how an authenticated local desktop identity maps to the existing provider identity requirements without fabricating a remote registry principal. If the correct design requires a new local-principal type or provider-neutral principal abstraction, that change requires a separate exact-source compatibility audit before implementation.

For files, the Agent must receive an already-established descriptor-anchored root authority from configuration/runtime assembly. The desktop request must never select an arbitrary host root.

C02 provider tests must use deterministic spy/fake backends first and prove:

- malformed request -> zero provider calls;
- policy denial -> zero provider calls;
- missing authority context -> zero provider calls;
- exact allowed operation -> exactly one expected provider call;
- provider failure -> correlated failure response, never success;
- provider success -> correlated success response only after backend success;
- request ID is preserved end-to-end.

C02 does not authorize production bootstrap activation.

### C03 — production local management activation

C03 is a separate activation gate. It is not authorized by this contract.

Before C03, evidence must lock at least:

- authenticated local principal semantics;
- management policy selection and defaults;
- filesystem root authority and ownership;
- concrete terminal backend and launch profile behavior;
- concrete forwarding backend and loopback/target behavior;
- transfer staging/root ownership;
- session/resource cleanup on desktop disconnect and Agent shutdown;
- bounded response encoding for every live operation;
- restart/rollback behavior;
- explicit no-privileged-helper escalation unless separately authorized;
- Ubuntu integration validation.

No production management capability is enabled merely because C01/C02 compile or test successfully.

## Dependency-correction rules for C01

The preferred minimal correction is allowed only after exact CI evidence:

1. `git grep` all `LocalAgentCommand` and `local_agent_command` references in the workspace;
2. if the `prw-remote-bridge -> prw-agent` dependency is proven to exist only for the convenience adapter, remove that adapter/dependency in one reviewed mutation;
3. add `prw-remote-bridge` as an internal `prw-agent` dependency only after the cycle is removed;
4. preserve the `BridgeCommand` PRWC wire format, operation codes and capability mapping byte-for-byte;
5. preserve local legacy command codes `1` and `2` and Slice B command `3` framing byte-for-byte;
6. preserve `BridgeCommand::AgentStatus` semantics by mapping it at the Agent-side local authority adapter rather than through a bridge-to-Agent dependency.

If exact CI evidence shows additional workspace callers or another cycle, stop the dependency mutation and re-audit. Do not force the preferred correction.

## C01 response semantics

C01 may add a local management response/error envelope only if it reuses the existing local frame header and outer request correlation.

Success semantics are reserved for provider-backed C02/C03 outcomes. C01 admission alone must never produce a management `Ok` result that could be interpreted by desktop presentation state as operation completion.

The desktop remains required to treat request construction/send/admission as intent, not authoritative success.

## Explicitly forbidden in C00/C01

- broadening `BoundedLocalReadPolicy` production defaults;
- fabricating `RegistryValidatedPrincipal` for same-UID local desktop requests;
- arbitrary host filesystem root selection from request bytes;
- direct shell strings/executable paths/argument vectors from desktop protocol;
- PTY/shell spawn activation;
- filesystem mutation;
- transfer finalization mutation;
- forwarding listener/socket activation;
- privileged-helper forwarding;
- DNS/network/firewall/TUN mutation;
- authentication cutover;
- database mutation;
- systemd/service installation or restart;
- deployment;
- merge to `main` as part of implementation validation.

## C01 candidate mutation boundary

After this C00 contract validates, a C01 candidate may touch only files needed for:

- exact dependency-edge correction;
- canonical Agent-side command-3 decode;
- exact capability policy admission;
- a typed local-management admission token;
- correlated fail-closed local error response construction;
- tests and Phase 152 validation workflow/evidence.

Provider crates may be read/audited but must not be mutated during C01 unless a new reviewed contract explicitly expands scope.

## Required C01 validation

At minimum C01 must prove:

1. exact workspace adapter-use search evidence is preserved;
2. Cargo dependency graph is acyclic;
3. canonical PRWC bytes and all 18 operation codes remain unchanged;
4. legacy local commands `1` and `2` remain unchanged;
5. Slice B local command `3` framing remains unchanged;
6. malformed/truncated/trailing PRWC fails before policy evaluation and dispatch;
7. exact capability comes only from decoded `BridgeCommand::required_capability()`;
8. policy denial creates no admission token and invokes no dispatcher/provider;
9. allow creates exactly one typed local-management admission token with preserved request ID/command/capability;
10. C01 creates no management success acknowledgement;
11. production `BoundedLocalReadPolicy` bootstrap remains unchanged and management-denying;
12. no provider/host-effect primitive is introduced in C01 modules;
13. `cargo fmt --all -- --check` passes;
14. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` passes;
15. `cargo test --locked --workspace --all-targets` passes;
16. `cargo build --locked --workspace --all-targets` passes.

## Lifecycle classification

`PHASE_152_C00_AUTHORITY_GATE_LOCKED / SLICE_B_GREEN / CANONICAL_PRWC_REQUIRED / EXACT_CAPABILITY_DERIVATION_REQUIRED / LOCAL_PRINCIPAL_AND_FILE_ROOT_AUTHORITY_UNRESOLVED_FOR_LIVE_DISPATCH / PRODUCTION_MANAGEMENT_POLICY_REMAINS_DENY / C01_PURE_DECODE_AND_ADMISSION_ONLY / C02_PROVIDER_PROOF_GATED / C03_PRODUCTION_ACTIVATION_NOT_AUTHORIZED`
