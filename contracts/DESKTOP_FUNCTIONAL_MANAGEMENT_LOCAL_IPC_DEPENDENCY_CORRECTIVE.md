# Private Remote Workspace — Desktop Functional Management Local IPC Dependency Corrective

Status: Phase 152-B01 corrective lock
Date: 2026-08-17
Repository: `Gersi365/prw-executor-private`
Parent contract: `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_CONTRACT.md`
Candidate branch: `phase-152-desktop-functional-management`
Validated Slice A head: `e4fdc357bbd2c06022af091c1d45b622fe79cb53`

## Trigger evidence

Phase 152 Slice B must extend the existing local IPC boundary without creating an alternate terminal/files/forwarding command language and without allowing client-declared capability authority.

The canonical management operation registry remains Phase 143 `prw-remote-bridge::BridgeCommand` and its PRWC codec. The current dependency graph is:

- `prw-remote-bridge` depends on `prw-agent` only for the narrow `LocalAgentCommand` convenience mapping used by `BridgeCommand::AgentStatus`;
- `prw-agent` does not depend on `prw-remote-bridge`;
- `apps/desktop` already depends on both crates.

An initial corrective hypothesis considered inverting the `prw-agent` / `prw-remote-bridge` dependency so the Agent could decode PRWC directly during Slice B. Exact source audit showed that inversion is unnecessary for this slice because Slice B is a protocol/enforcement boundary only and performs no live provider dispatch. Changing the dependency direction now would increase API and graph surface without producing required Slice B authority.

## Corrective decision

Phase 152-B01 preserves the current dependency graph and public `prw-remote-bridge` API.

1. `prw-remote-bridge -> prw-agent` remains unchanged in Slice B.
2. `BridgeCommand::local_agent_command()` remains unchanged in Slice B.
3. `prw-agent` does not gain a `prw-remote-bridge` dependency in Slice B.
4. The local Agent crate owns only the bounded local framing schema for the new management request command.
5. `apps/desktop`, which already has both dependencies, proves by integration tests that the embedded body is canonical PRWC, round-trips through `BridgeCommand::decode`, and derives its exact `prw-policy::Capability` from `BridgeCommand::required_capability()`.
6. No capability value is carried in the local management request schema.
7. Live Agent-side canonical decode, policy decision and provider dispatch remain a Slice C gate. If Slice C requires dependency inversion or a dedicated integration layer, that decision must be based on the exact validated Slice B state and separately locked before mutation.

This preserves the Phase 152 requirement that PRWC remain canonical while avoiding a speculative dependency/API change.

## Local IPC extension decision

Slice B retains local IPC protocol version `1.0` as an additive command-namespace extension. Existing frame/header semantics and existing command meanings do not change.

A new local command code `3` is reserved for a typed management bridge request.

Request payload schema for command `3`:

```text
u16 command_code = 3
u32 bridge_payload_length
u8[bridge_payload_length] canonical PRWC BridgeCommand payload
```

Correlation remains exclusively the existing non-zero `LocalIpcRequestId` in the 24-byte local frame header. No second request identifier is introduced.

Compatibility rules:

- the legacy Phase 015 codec remains dedicated to command codes `1` and `2` and continues to require exactly two bytes;
- legacy command `1` remains `GetAgentStatus` encoded as `[0, 1]`;
- legacy command `2` remains `GetPrivateDnsConfig` encoded as `[0, 2]`;
- the legacy two-byte decoder continues to reject `[0, 3]`;
- command code `3` is decoded only by the separate management-request framing codec;
- command code `3` requires the exact extended schema above;
- unknown command codes fail closed;
- malformed, truncated, trailing, zero-length or oversized embedded payloads fail closed;
- the embedded payload bound is the existing PRWC maximum of 65,536 bytes and is not increased by local IPC;
- Slice B performs no provider dispatch and no host mutation; live effects remain gated by Slice C.

A Phase 151/legacy Agent that does not implement the separate code `3` boundary therefore continues to reject that command rather than misinterpreting it. Coordinated live use is not authorized until Slice C.

## Canonical-operation and capability proof

Because `apps/desktop` already imports both `prw-agent` and `prw-remote-bridge`, Slice B integration tests must prove the complete pure transformation:

1. construct an existing typed `BridgeCommand`;
2. encode it with canonical `BridgeCommand::encode()`;
3. wrap those exact bytes in local command code `3` with an existing `LocalIpcRequestId`;
4. decode the local management envelope with the Agent-owned framing codec;
5. decode the embedded bytes with canonical `BridgeCommand::decode()`;
6. derive the required capability only from `BridgeCommand::required_capability()`;
7. preserve the original local request correlation identifier.

The local code `3` schema carries no capability field, authorization flag, provider name, shell string or privileged-helper command. Therefore a client cannot independently select a capability that differs from the decoded canonical operation.

## Bulk-transfer boundary

The local control request may carry only payloads already accepted by the canonical bounded PRWC codec. It does not create a new bulk-transfer path, remove exact-offset upload semantics, bypass integrity verification, or raise the existing PRWC inline-data bounds.

## Mutation boundary

Phase 152-B01 may change only the files needed to:

- preserve this corrective evidence;
- define and test a separate Agent-owned local command code `3` framing codec;
- add pure desktop-side integration glue/tests using the already-present `prw-agent` and `prw-remote-bridge` dependencies;
- preserve legacy local request behavior byte-for-byte;
- extend the Phase 152 validation workflow/evidence.

No manifest dependency change is authorized by this Slice B corrective.

No external dependency, framework/toolchain change, live provider dispatch, production listener, shell execution API, filesystem mutation, forwarding socket activation, privileged-helper pass-through, network mutation or OS DNS mutation is authorized.

## Required validation

At minimum the candidate must prove:

1. the existing dependency graph remains unchanged by Slice B;
2. PRWC `BridgeCommand` encoding/decoding remains unchanged;
3. local commands `1` and `2` remain byte-for-byte compatible;
4. the legacy two-byte command decoder still rejects code `3`;
5. the separate command `3` framing codec round-trips a bounded canonical PRWC payload with the original request ID;
6. malformed, truncated, trailing, zero-length, oversized and wrong-command management payloads fail closed;
7. desktop integration tests decode the embedded body only through canonical `BridgeCommand::decode()`;
8. exact capability is derived from the decoded `BridgeCommand::required_capability()` and cannot be supplied separately by the client;
9. the local control path performs no provider dispatch in Slice B;
10. `cargo fmt --all -- --check` passes;
11. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` passes;
12. `cargo test --locked --workspace --all-targets` passes;
13. `cargo build --locked --workspace --all-targets` passes;
14. the Phase 152 no-production-side-effect boundary remains intact.

## Classification

`PHASE_152_B01_LOCAL_IPC_CORRECTIVE_LOCKED / DEPENDENCY_GRAPH_PRESERVED_FOR_SLICE_B / SEPARATE_LOCAL_COMMAND_3_FRAMING / CANONICAL_PRWC_PROOF_IN_EXISTING_DESKTOP_INTEGRATION_BOUNDARY / CAPABILITY_DERIVED_NOT_CLIENT_DECLARED / NO_PROVIDER_DISPATCH / NO_PRODUCTION_SIDE_EFFECT`
