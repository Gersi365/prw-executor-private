# Phase 152 Slice C02b — Local Management Authority Assembly Gate

Status: `AUDIT_ONLY_CONTRACT / NO_PROVIDER_WIRING / NO_PRODUCTION_ACTIVATION`

Validated predecessor candidate: `05b27e31024fbca480df1f0b09d959a8d4ffbb77` (C02a provider-neutral dispatch proof).

## Purpose

C02a proved ordering only: canonical authenticated admission, exact authority matching, provider-neutral dispatch, and success only after provider success. C02a deliberately provides no production constructor for `LocalManagementAuthorityContext` and wires no real provider crate into the Agent runtime.

C02b must not fill that gap by fabricating authority. This contract locks the evidence required before any real terminal, file, transfer, or forwarding authority can be assembled.

## Current production-local identity boundary

The Linux local IPC boundary authenticates an already-connected Unix peer from kernel `SO_PEERCRED` and requires same-UID authorization. The resulting local session carries `LocalLinuxPeerCredentials` (`pid`, `uid`, `gid`).

This local transport identity is not a `RegistryValidatedPrincipal` and must not be converted into one by inventing workspace, user, device, or authenticated-session identifiers.

## Current production runtime inputs

`LocalLinuxProductionRuntimeInputs` currently carries only:

- validated runtime configuration;
- `BoundedLocalReadPolicy`;
- immutable local Agent status snapshot;
- immutable private-DNS snapshot.

The authenticated session, worker, and server-loop chain propagates the policy/snapshots and I/O bounds. It has no terminal/forwarding registry-principal authority and no descriptor-anchored file/transfer root authority slot.

The production bootstrap continues to construct `BoundedLocalReadPolicy::allow_local_reads()`. C02b does not expand that default into management capability admission.

## Provider authority facts

### Terminal

`prw-terminal` derives `TerminalPrincipal` only from a `RegistryValidatedPrincipal` plus an authenticated PRW `SessionId`. The terminal foundation does not grant terminal capability by itself.

### Forwarding

`prw-forwarding` derives `ForwardingPrincipal` only from a `RegistryValidatedPrincipal` plus an authenticated PRW `SessionId`. The forwarding foundation does not grant forwarding capability by itself.

### File service

`prw-file-service::AnchoredFileRoot` is the filesystem authority. It is created by opening an explicit root directory descriptor and then resolves remote paths relative to that descriptor under bounded no-follow semantics.

Request payload bytes must never select an arbitrary host root or be upgraded into ambient filesystem authority.

### File transfer

`prw-file-transfer::UploadTransferManager` requires an already-existing `&AnchoredFileRoot`; raw filesystem authority remains in `prw-file-service`.

## Locked C02b decisions

1. **No fabricated remote principal.** Same-UID local IPC credentials are transport/local-caller evidence, not proof of workspace/user/device/session registry identity.
2. **No request-selected filesystem root.** Any future file/transfer authority root must be selected by Agent-owned trusted configuration or bootstrap assembly and opened into an `AnchoredFileRoot` before request dispatch.
3. **No ambient provider construction in request code.** Real provider authority objects must be assembled outside canonical request decoding/admission and passed through an explicit authority-context boundary.
4. **Policy admission remains independent.** Provider authority possession does not replace capability policy admission; denial must still yield zero provider calls.
5. **Authority remains caller/request bound.** Any production constructor replacing the C02a test-only authority constructor must bind the exact admitted request ID, kernel peer credentials, derived capability, canonical operation code, and provider family.
6. **Missing authority fails closed.** Absence, stale correlation, caller mismatch, capability mismatch, operation mismatch, or provider-family mismatch must yield no provider invocation and no success acknowledgement.
7. **Production bootstrap remains closed to management.** C02b audit does not alter production policy defaults, runtime inputs, provider dependencies, filesystem roots, principal semantics, service activation, deployment, DNS/network state, or system privilege.

## Open authority blockers

Before C02b real-provider implementation can begin, an explicitly reviewed design must choose:

- the local-management principal model for terminal and forwarding without pretending same-UID IPC is a registry-authenticated remote session;
- the trusted configuration/ownership model for the descriptor-anchored file root;
- lifecycle ownership for terminal brokers, forwarding brokers, anchored roots, and transfer managers;
- cleanup/rollback semantics for those authority objects;
- the exact production policy configuration that may grant management capabilities.

Until those are locked, C02a's production authority constructor remains absent and real provider dispatch remains intentionally unreachable.

## Audit-only validation scope

This gate may change only:

- this contract; and
- `.github/workflows/phase-152-c02b-authority-audit.yml`.

It must not change Rust source, any `Cargo.toml`, any `Cargo.lock`, provider dependencies, runtime wiring, policy defaults, desktop code, Android code, packaging, or deployment state.

## Next gate

After this audit-only contract is green, the next reviewed C02b implementation gate may introduce the smallest explicit Agent-owned authority assembly seam needed to make real provider adapters testable. It still must not activate production management by default.

C03 remains the separate production activation gate and is not authorized here.
