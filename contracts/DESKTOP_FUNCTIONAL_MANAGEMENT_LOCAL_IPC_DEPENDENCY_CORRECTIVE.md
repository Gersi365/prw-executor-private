# Private Remote Workspace — Desktop Functional Management Local IPC Dependency Corrective

Status: Phase 152-B01 corrective lock
Date: 2026-08-17
Repository: `Gersi365/prw-executor-private`
Parent contract: `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_CONTRACT.md`
Candidate branch: `phase-152-desktop-functional-management`
Validated Slice A head: `e4fdc357bbd2c06022af091c1d45b622fe79cb53`

## Trigger evidence

Phase 152 Slice B requires the live local Agent boundary to decode an embedded management operation through the canonical Phase 143 `prw-remote-bridge::BridgeCommand` codec before capability admission or provider dispatch.

The current dependency graph prevents that exact reuse:

- `prw-remote-bridge` depends on `prw-agent`;
- the only `prw-agent` symbol referenced by `prw-remote-bridge` is `LocalAgentCommand`;
- that reference exists only to map `BridgeCommand::AgentStatus` to `LocalAgentCommand::GetAgentStatus`;
- therefore adding `prw-remote-bridge` as an Agent dependency would currently create a Cargo dependency cycle.

Accepting opaque PRWC bytes in the Agent, trusting a client-supplied capability, or duplicating the PRWC decoder would violate the Phase 152 authority contract and is not authorized.

## Corrective decision

The dependency edge is inverted with the smallest source/API adjustment required for Agent-side canonical decode:

1. remove the `prw-agent` dependency from `prw-remote-bridge`;
2. remove the narrow `BridgeCommand::local_agent_command()` convenience adapter from `prw-remote-bridge`;
3. add `prw-remote-bridge` as an internal path dependency of `prw-agent` for Slice B decoding and capability derivation;
4. preserve `BridgeCommand`, its PRWC wire format, operation codes, validation and capability mapping unchanged;
5. preserve the existing local Agent command codes `1` and `2` byte-for-byte.

The removed convenience adapter is not a wire authority. `AgentStatus` remains operation code `1` in PRWC and `GetAgentStatus` remains local command code `1`; any required mapping belongs at the Agent/local-boundary adapter layer rather than forcing the canonical bridge registry to depend on its consumer.

## Local IPC extension decision

Slice B retains local IPC protocol version `1.0` as an additive command-namespace extension because existing command semantics and frame/header encoding remain unchanged.

A new local command code `3` is reserved for a typed management bridge request.

Request payload schema for command `3`:

```text
u16 command_code = 3
u32 bridge_payload_length
u8[bridge_payload_length] canonical PRWC BridgeCommand payload
```

Correlation remains exclusively the existing non-zero `LocalIpcRequestId` in the 24-byte local frame header. No second request identifier is introduced.

Compatibility rules:

- command codes `1` and `2` continue to require exactly the existing two-byte payload;
- command code `3` requires the extended schema above;
- unknown command codes fail closed;
- malformed, truncated, trailing, oversized or non-canonical PRWC payloads fail closed;
- `BridgeCommand::decode` is the only operation decoder used for the embedded management payload;
- required capability is derived from the decoded `BridgeCommand::required_capability()` and is never accepted from the client;
- Slice B performs no provider dispatch and no host mutation; live effects remain gated by Slice C.

## Bulk-transfer boundary

The local control request may carry only payloads already accepted by the canonical bounded PRWC codec. It does not create a new bulk-transfer path, remove exact-offset upload semantics, bypass integrity verification, or raise the existing PRWC inline-data bounds.

## Mutation boundary

Phase 152-B01 may change only the files needed to:

- invert the internal `prw-agent` / `prw-remote-bridge` dependency edge;
- define and test local command code `3` and its bounded request codec;
- derive exact capability from decoded `BridgeCommand`;
- preserve legacy local request behavior;
- extend the Phase 152 validation workflow/evidence.

No external dependency, framework/toolchain change, provider dispatch, production listener, shell execution API, filesystem mutation, forwarding socket activation, privileged-helper pass-through, network mutation or OS DNS mutation is authorized.

## Required validation

At minimum the candidate must prove:

1. the dependency graph is acyclic and locked;
2. PRWC `BridgeCommand` encoding/decoding remains unchanged;
3. local commands `1` and `2` remain byte-for-byte compatible;
4. command `3` round-trips a canonical typed management operation with the original request ID;
5. malformed/truncated/trailing/unknown command payloads fail closed;
6. client bytes cannot choose or forge the required capability independently of the decoded operation;
7. the local control path does not dispatch providers in Slice B;
8. `cargo fmt --all -- --check` passes;
9. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` passes;
10. `cargo test --locked --workspace --all-targets` passes;
11. `cargo build --locked --workspace --all-targets` passes;
12. the Phase 152 no-production-side-effect boundary remains intact.

## Classification

`PHASE_152_B01_LOCAL_IPC_DEPENDENCY_CORRECTIVE_LOCKED / CANONICAL_PRWC_DECODE_AT_AGENT_BOUNDARY / LEGACY_LOCAL_COMMANDS_PRESERVED / CAPABILITY_DERIVED_NOT_CLIENT_DECLARED / NO_PROVIDER_DISPATCH / NO_PRODUCTION_SIDE_EFFECT`
