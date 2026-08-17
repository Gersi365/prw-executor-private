# Private Remote Workspace — Desktop Functional Management Slice C02 Provider Proof

Status: `PHASE_152_C02_CONTRACT_LOCKED / AUDIT_FIRST / NO_LIVE_PROVIDER_ACTIVATION`
Date: 2026-08-17
Repository: `Gersi365/prw-executor-private`
Branch: `phase-152-desktop-functional-management`
Phase 152 baseline: `3aec5d9307edfa5abe53c4ebe169a2445599a2c0`
Validated C01 source commit: `daf126a8e77bd6e2fc03179fa167aca028a2e7c5`
Authoritative C01 validation head: `1e12e6533490f343e1259265dd44eb1b496b4a25`
Authoritative C01 validation run: `32005803618` (run #21)
Parent contract: `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C_AUTHORITY_GATE.md`

## Purpose

C02 begins only after C01 canonical decode and authenticated local policy admission are green. Its purpose is to prove the ordering between a typed `LocalManagementAdmission`, explicit operation-family authority requirements, provider dispatch, and correlated outcome handling without activating real terminal, filesystem, transfer, or forwarding providers.

C02 must preserve the rule:

```text
malformed/unadmitted request
    -> zero dispatcher calls

admitted request + missing required authority
    -> zero dispatcher calls

admitted request + explicit required authority
    -> exactly one provider-neutral dispatcher call
    -> dispatcher result
    -> correlated outcome only after the dispatcher result
```

C02 is not a production-management activation phase.

## Audited provider boundaries

### Remote bridge dispatcher cannot be reused as a local principal shortcut

`prw-remote-bridge::CapabilityDispatcher` consumes an `AuthorizedCapabilityRequest` whose principal is a `RegistryValidatedPrincipal`. That type is produced by current-registry revalidation of authenticated PRW device sessions.

The C01 local admission instead binds kernel-authenticated same-UID Unix peer credentials from `AuthenticatedLocalLinuxConnection`.

These principal domains are distinct. C02 must not fabricate, synthesize, or infer a `RegistryValidatedPrincipal` from local PID/UID/GID values merely to reuse the remote dispatcher path.

### Terminal authority is registry-derived today

`prw-terminal::TerminalPrincipal::from_registry(...)` accepts a `RegistryValidatedPrincipal` plus authenticated PRW `SessionId`. `TerminalBroker` then owns lifecycle around a typed `TerminalBackend`.

C02 therefore must not call `TerminalBroker` from the local desktop path until an explicit reviewed adapter defines legitimate local terminal principal semantics. A fake registry principal is forbidden.

### Forwarding authority is registry-derived today

`prw-forwarding::ForwardingPrincipal::from_registry(...)` has the same registry-derived identity requirement. `PortForwardBroker` wraps a typed backend and can ultimately represent a listener/target resource.

C02 must not create a real forwarding session or listener. Local PID/UID/GID values are not a substitute for `RegistryValidatedPrincipal`.

### File and transfer authority is descriptor-anchored

`prw-file-service::AnchoredFileRoot` is the filesystem authority. It opens an explicit existing root descriptor and resolves remote paths only beneath that descriptor. Request path bytes do not select the host root.

`prw-file-transfer::UploadTransferManager` and download operations consume that same descriptor-anchored root authority.

C02 must not open a host path to manufacture an authority context. Real root selection/ownership belongs to later runtime assembly and C03 activation review.

### Agent dependency surface remains intentionally narrow

At the C02 contract lock, `prw-agent` has no direct dependency on:

- `prw-registry`;
- `prw-terminal`;
- `prw-forwarding`;
- `prw-file-service`;
- `prw-file-transfer`.

C02a should not add those direct provider dependencies merely to make tests pass. Any future direct provider dependency requires an explicit C02b adapter review proving that its authority can be constructed legitimately from runtime-owned state rather than request data.

## C02 staging decision

C02 is split into two narrower gates.

### C02a — provider-neutral dispatch-order proof

C02a may add an Agent-owned, crate-local provider-neutral dispatch boundary around an already-created `LocalManagementAdmission`.

It may define only the minimum typed concepts needed to prove:

1. which authority family an admitted `BridgeCommand` requires;
2. that the required authority is checked before a dispatcher call;
3. that missing authority causes zero dispatcher calls;
4. that an admitted operation with explicit test authority causes exactly one expected dispatcher call;
5. that dispatcher failure never becomes success;
6. that success is correlated to the original `LocalIpcRequestId` only after dispatcher success.

C02a authority proof objects must not claim to be real terminal principals, forwarding principals, or filesystem roots. Test-only fake/spy authority markers are acceptable only when they are unmistakably test/provider-neutral proof objects and cannot be confused with production authority.

C02a must not import or call real `TerminalBroker`, `PortForwardBroker`, `AnchoredFileRoot`, `UploadTransferManager`, or provider backends.

### C02b — real authority-context adapter design

C02b is a separate reviewed adapter gate after C02a is green.

Before any real provider call, C02b must resolve and lock:

- local terminal principal semantics without fabricating `RegistryValidatedPrincipal`;
- local forwarding principal semantics without fabricating `RegistryValidatedPrincipal`;
- runtime-owned descriptor-anchored file root selection and ownership;
- transfer manager ownership/lifetime over that root;
- cleanup semantics on local desktop disconnect and Agent shutdown;
- how provider errors become bounded correlated local responses;
- exact provider dependency additions to `prw-agent` or a dedicated runtime integration crate.

If legitimate local identity cannot satisfy the existing terminal/forwarding principal contracts, C02b must stop and propose a separately reviewed provider-neutral principal abstraction. It must not weaken the existing remote registry identity contract.

## C02a required ordering

C02a must preserve C01 as the only admission constructor:

```text
LocalIpcFrame
    -> command-3 framing decode
    -> BridgeCommand::decode
    -> BridgeCommand::required_capability
    -> authenticated-local PolicyEvaluator
    -> LocalManagementAdmission
    -> C02a operation-family authority requirement
    -> explicit provider-neutral authority proof
    -> exactly one provider-neutral dispatcher call
    -> dispatcher result
    -> correlated local outcome
```

No C02a API may accept a capability, caller UID/GID/PID, filesystem root path, registry principal, shell command string, executable path, bind address, or success status directly from request-controlled bytes outside the existing typed codec/admission path.

## C02a test obligations

At minimum C02a must prove deterministically:

1. malformed outer framing -> zero policy calls and zero dispatcher calls;
2. malformed canonical PRWC -> zero policy calls and zero dispatcher calls;
3. policy denial -> zero dispatcher calls;
4. missing operation-family authority -> zero dispatcher calls;
5. mismatched authority family -> zero dispatcher calls;
6. exact admitted operation + matching test authority -> exactly one dispatcher call;
7. dispatcher receives the same admitted command/capability/correlation ID produced by C01;
8. dispatcher failure -> correlated failure classification and no success state;
9. dispatcher success -> correlated success classification only after exactly one call;
10. production `BoundedLocalReadPolicy::allow_local_reads()` remains management-denying;
11. no real terminal/file/transfer/forwarding provider crate is called by C02a;
12. no direct process, filesystem, socket, DNS, service, firewall, routing, TUN/TAP or privileged-helper mutation is added;
13. `cargo fmt --all -- --check` passes;
14. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` passes;
15. `cargo test --locked --workspace --all-targets` passes;
16. `cargo build --locked --workspace --all-targets` passes.

## Production policy invariant

`linux_bootstrap::run()` must continue to use `BoundedLocalReadPolicy::allow_local_reads()` through C02a. C02a must not add a management-capable production policy, environment switch, feature flag, command-line switch, hidden allowlist, or test-default pathway that could enable management in the running Agent.

## Explicitly forbidden in C02a

- constructing or fabricating `RegistryValidatedPrincipal` from local peer credentials;
- constructing `TerminalPrincipal` or `ForwardingPrincipal` for live local dispatch;
- opening an `AnchoredFileRoot` from request data or C02a runtime code;
- creating `UploadTransferManager` over a live root;
- invoking `TerminalBroker`, `PortForwardBroker`, real file-service methods, or real transfer methods;
- PTY or shell process activation;
- filesystem read/write/create/delete/finalization from the C02a local management path;
- socket bind/connect/listen/forward activation;
- privileged helper pass-through;
- production management policy expansion;
- authentication/database/systemd/network/DNS/firewall/TUN mutation;
- deployment or merge to `main` as part of implementation validation.

## C02a candidate mutation boundary

After this contract is validated, C02a may touch only:

- Agent-local provider-neutral authority/dispatch proof modules;
- the minimum module wiring needed to expose them crate-locally;
- deterministic tests;
- this Phase 152 validation workflow and C02 evidence.

No provider crate or production bootstrap source may be modified during C02a without a new reviewed contract expansion.

## Lifecycle classification

`PHASE_152_C02_CONTRACT_LOCKED / C01_GREEN / LOCAL_AND_REMOTE_PRINCIPALS_REMAIN_DISTINCT / FILE_ROOT_AUTHORITY_DESCRIPTOR_OWNED / C02A_PROVIDER_NEUTRAL_PROOF_ONLY / C02B_REAL_ADAPTERS_SEPARATELY_GATED / PRODUCTION_POLICY_REMAINS_MANAGEMENT_DENY / C03_PRODUCTION_ACTIVATION_NOT_AUTHORIZED`
