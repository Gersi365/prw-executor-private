# Private Remote Workspace — Desktop Functional Management Contract

Version: `0.1.0`

Status: Phase 152 implementation lock
Date: 2026-08-17
Repository: `Gersi365/prw-executor-private`
Input baseline: `3d4f27ef90a38f7939b545c02cdcd760d663e2b9`
Parent roadmap: `contracts/PRODUCTIZATION_ROADMAP_AND_MUTATION_GATES_CONTRACT.md`
Parent desktop architecture: `contracts/DESKTOP_CLIENT_ARCHITECTURE_DECISION.md`
Parent desktop foundation: `contracts/DESKTOP_CLIENT_FOUNDATION_CONTRACT.md`
Parent remote capability bridge: `contracts/END_TO_END_AUTHENTICATED_CAPABILITY_BRIDGE_CONTRACT.md`

## Purpose

Phase 152 turns the validated Phase 151 native GTK4/libadwaita desktop shell into bounded functional management surfaces for terminal, files/transfers, forwarding, private-network status and optional private-DNS configuration.

The phase reuses existing typed PRW authorities. The desktop remains an unprivileged client/presentation process and the Agent remains the local enforcement and host-authority boundary.

Phase 152 is non-production product implementation. It does not cross the Phase 154 production remote-network mutation gate and does not authorize production remote listeners, production STUN/ICE/TURN/relay activation, TUN/TAP or route/firewall/NAT mutation, production DNS mutation, production credential provisioning, Agent replacement/restart, distribution signing or deployment.

## Baseline gate

Implementation may materialize only from a repository state whose Phase 151 authoritative evidence proves:

- `PHASE_151_DONE`;
- desktop architecture and dependency closure;
- permanent root Rust CI success;
- permanent Android CI success;
- read-only local Agent startup probe validation;
- `READY_FOR_PHASE_152`.

The Phase 151 fixed local endpoint, same-effective-UID trust boundary, runtime-directory/socket ownership and mode checks, request correlation and bounded timeout behavior remain authoritative and must not be weakened.

## Existing authority that MUST be reused

Phase 152 MUST NOT create a second terminal protocol, filesystem syntax, transfer protocol, forwarding protocol, capability model, connectivity-selection algorithm or private-DNS validator.

The existing Rust workspace remains authoritative for:

### Terminal

- `prw-terminal::TerminalSessionId`;
- `TerminalProfile::{PosixShell, BashShell}`;
- `TerminalGeometry` bounds;
- terminal lifecycle and bounded I/O semantics;
- Phase 143 `BridgeCommand::{TerminalOpen, TerminalInput, TerminalResize, TerminalRead, TerminalClose}`;
- exact server-side `TerminalOpen` versus `TerminalExec` capability separation.

### Files and transfers

- `prw-file-service::RemotePath`;
- descriptor-anchored filesystem confinement and path validation;
- `prw-file-transfer::{TransferId, UploadPlan}`;
- exact-offset resumable upload semantics;
- SHA-256 integrity verification;
- atomic create-only finalization;
- bounded download semantics;
- Phase 143 file/transfer `BridgeCommand` variants and capability mapping.

### Forwarding

- `prw-forwarding::{PortForwardId, LoopbackFamily, LoopbackBind, ForwardTarget, TcpForwardSpec}`;
- loopback-only bind authority;
- explicit-IP TCP target authority;
- Phase 143 `BridgeCommand::{ForwardOpen, ForwardClose}`;
- `ForwardingCreate` capability enforcement.

### Connectivity

- `prw-connectivity` typed connectivity/path selection;
- path classifications `LocalDirect`, `InternetDirect`, `Relay`, `Offline`;
- authoritative/disposable typed observations rather than UI-fabricated reachability.

### Optional private DNS

- `prw-private-dns` validated configuration types and semantic constraints;
- explicit distinction between requested/validated DNS configuration and operating-system applied state;
- private DNS remaining optional and not gating basic private connectivity.

### Remote capability wire authority

For terminal, files/transfers and forwarding, Phase 143 `prw-remote-bridge::BridgeCommand` and its PRWC codec remain the canonical typed operation registry. Desktop code may construct and encode those existing commands for bounded non-production intent projection. It MUST NOT duplicate the PRWC codec or invent an alternate command string protocol.

## Desktop authority model

Desktop is not an authorization authority.

A UI state, navigation destination, device lifecycle label, workspace role, successful local socket connection or valid typed intent MUST NOT be treated as proof that an operation is authorized or completed.

For any real future dispatch:

1. the desktop constructs only validated typed intent;
2. the local Agent receives the request over the existing authenticated local boundary;
3. the Agent maps the exact operation to explicit capability/policy authority;
4. the authoritative provider performs the operation only after policy and operation-specific validation;
5. the desktop updates authoritative state only from a validated correlated result.

Same-UID peer authentication alone does not grant terminal, filesystem, forwarding, network or DNS capability.

## Phase 152 implementation slices

Phase 152 is one roadmap phase but may be materialized through bounded validation slices. Every slice must preserve the complete Phase 152 contract.

### Slice A — typed desktop management projection

Slice A may add internal path dependencies from `apps/desktop` to existing workspace crates required to construct validated management intents:

- `prw-remote-bridge`;
- `prw-terminal`;
- `prw-file-service`;
- `prw-file-transfer`;
- `prw-forwarding`;
- `prw-connectivity`;
- `prw-private-dns`.

No new crates.io dependency is authorized by Slice A.

Slice A may replace Phase 151 placeholders with functional non-production presentation/controllers that:

- construct existing typed terminal/file/transfer/forwarding intent payloads;
- validate connectivity and DNS presentation through existing domain authorities;
- distinguish local intent/pending state from authoritative success;
- support explicit disposable authoritative acknowledgements/snapshots for deterministic CI tests;
- perform no terminal process spawn, filesystem mutation, forwarding socket creation, network probing or DNS OS mutation from the desktop process.

### Slice B — explicit typed local IPC capability extension

Before the desktop sends any mutating management intent to the live local Agent, Phase 152 must extend the Phase 008 read-only local command namespace through an explicit typed protocol contract and source implementation.

The extension MUST:

- preserve the fixed Phase 151 UDS trust boundary;
- preserve bounded framing and request correlation;
- define an exact command code and bounded payload schema;
- decode the embedded operation through existing typed authorities rather than accepting arbitrary shell/command text;
- map each admitted operation to its exact `prw-policy::Capability`;
- fail closed before provider dispatch on malformed input or denied capability;
- not expose privileged-helper commands directly;
- retain dedicated bulk-transfer architecture rather than using the local control frame to bypass transfer bounds.

If compatibility rules require a local IPC version change, that version change must be explicitly locked and tested before runtime use. Phase 152 MUST NOT silently change `1.0` compatibility semantics.

### Slice C — bounded Agent-backed management flow

A live local Agent management flow may be wired only after Slice B is validated.

Any Agent-backed operation remains within existing capability-provider boundaries and MUST NOT introduce a generic arbitrary-command dispatcher. Terminal/file/forwarding effects must be owned by the existing typed providers and guarded by exact capability policy.

Phase 152 does not authorize privileged networking or OS DNS mutation. Network and DNS surfaces remain status/validated-request presentation unless a later separately gated phase explicitly authorizes operating-system mutation.

## Intent-versus-authoritative-state rules

The desktop follows the already validated Android Phase 147–149 discipline.

### Terminal

- open intent may move presentation to `Opening`, never directly to `Open`;
- input/resize/read are valid only while authoritative state is `Open`;
- close intent may move presentation to `Closing`, never directly to `Closed`;
- only an authoritative/disposable result may move `Opening -> Open`, `Closing -> Closed`, or any state to `Failed`;
- desktop input is terminal-session input, not a new `runCommand` API.

### Files and transfers

- list/stat requests do not fabricate entries or metadata;
- upload begin/resume intent does not advance committed offset;
- sending a chunk does not advance progress;
- only an authoritative/disposable acknowledgement advances committed offset;
- finalize intent does not claim completion;
- download requests do not advance progress before authoritative bytes arrive;
- file management MUST NOT be implemented by terminal shell commands.

### Forwarding

- open intent yields `Opening`, not `Active`;
- close intent yields `Closing`, not `Closed`;
- only authoritative/disposable acknowledgement changes active/closed authority state;
- desktop owns no forwarding socket.

### Connectivity

- default/no authoritative observation is `Offline` or explicitly unknown;
- UI state must not fabricate a reachable path;
- path selection must come from existing `prw-connectivity` typed authority.

### Private DNS

- UI edits may produce only a validated requested configuration;
- validated `Enabled` does not mean the operating-system resolver was changed;
- Phase 152 must not claim `os_applied=true` without a separately authorized OS integration contract and authoritative result;
- private DNS does not grant network reachability or any other capability.

## No arbitrary local execution API

Phase 152 MUST NOT introduce any normal API shaped as:

- `runCommand(String)`;
- arbitrary executable path plus argument vector;
- arbitrary shell fragment execution;
- environment-variable injection for process spawn;
- arbitrary filesystem path bypassing `RemotePath`;
- arbitrary bind IP or hostname forwarding target;
- generic privileged-helper pass-through.

## Local IPC compatibility boundary

The current local IPC implementation accepts exactly protocol `1.0` and the Phase 008 command decoder admits only the two-byte read-only commands `GetAgentStatus` and `GetPrivateDnsConfig`.

Slice A must not mutate that wire contract.

Slice B must explicitly decide and test one compatibility-safe extension before any new command is sent over the live Agent socket. Existing Phase 151 status/private-DNS requests must remain valid across that extension or the version bump must be intentionally coordinated on both sides.

## Dependency and source boundary

Phase 152 may change only files required for the approved desktop management slices, local typed IPC extension, existing provider integration and validation evidence.

No architecture redesign, unrelated refactor, dependency upgrade or opportunistic cleanup is authorized.

Any new external dependency or framework/toolchain change requires a separate dependency/architecture decision before materialization.

## Validation requirements

Phase 152 completion requires at least:

1. exact Phase 151 input commit recorded;
2. changed-file scope matches this contract;
3. desktop tests prove typed terminal operations round-trip through existing `BridgeCommand`;
4. desktop tests prove typed file/transfer operations round-trip through existing `BridgeCommand`;
5. desktop tests prove typed forwarding operations round-trip through existing `BridgeCommand`;
6. invalid terminal/session/geometry/input bounds fail closed through existing constructors/codec;
7. invalid/traversal/absolute file paths and invalid transfer metadata fail closed through existing constructors/codec;
8. invalid forwarding identifiers/bind/target inputs fail closed through existing constructors;
9. terminal open/close intent cannot forge authoritative state;
10. file/transfer intent cannot forge authoritative directory data, committed progress or completion;
11. forwarding intent cannot forge active/closed state;
12. connectivity state cannot be manufactured without explicit typed authoritative/disposable input;
13. DNS validation cannot claim OS-applied state;
14. no generic shell/filesystem/forwarding/privileged pass-through API exists;
15. any local IPC extension has explicit version/command/payload/capability tests;
16. existing Phase 151 read-only Agent status/private-DNS startup probe remains compatible;
17. desktop does not spawn a shell/process, mutate filesystem, own forwarding sockets, probe production network or mutate OS DNS directly;
18. root dependency graph remains locked except approved internal path edges and deterministic lockfile consequences;
19. `cargo fmt --all -- --check` passes;
20. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` passes;
21. `cargo test --locked --workspace --all-targets` passes;
22. `cargo build --locked --workspace --all-targets` passes;
23. permanent root Rust CI passes after materialization;
24. permanent Android CI remains green;
25. authoritative audit evidence records exact source hashes, commits, workflow run identifiers and production-boundary result.

## Production boundary

Until the exact Phase 154 transaction is separately approved, Phase 152 MUST NOT:

- publish or bind a production remote data-plane listener;
- activate production STUN/ICE/TURN or relay traffic;
- alter firewall/NAT/router/TUN/TAP/routes;
- mutate production/system DNS;
- provision production transport credentials;
- replace/restart the production Agent for the remote data plane;
- sign/distribute a production Android/Desktop client;
- perform deployment or production-account cutover.

## Audit evidence

The authoritative Phase 152 report must be preserved at:

`logs/audits/phase-152-desktop-functional-management/PRW-PHASE-152-DESKTOP-FUNCTIONAL-MANAGEMENT-VALIDATION.txt`

It must distinguish contract lock, candidate validation, materialization and permanent post-materialization CI. It must record exact commits, workflow runs, relevant source hashes and confirmation that Phase 154 production activation was not performed.

## Phase 153 handoff

Phase 152 is complete only after the functional management source is materialized on `main`, permanent root Rust CI and Android CI are green, and the authoritative audit report is committed.

Phase 153 is production remote-network activation **readiness only**. It may audit prerequisites and produce an activation plan but must not perform the Phase 154 production mutation transaction.

Target completion marker:

`PHASE_152_FUNCTIONALLY_VALIDATED / DESKTOP_TYPED_MANAGEMENT_SURFACES / EXISTING_DOMAIN_AND_BRIDGE_AUTHORITIES_REUSED / EXPLICIT_LOCAL_IPC_CAPABILITY_BOUNDARY / INTENT_NEVER_FORGES_AUTHORITY / ROOT_RUST_PASS / ANDROID_CI_PASS / NO_PRODUCTION_SIDE_EFFECT / READY_FOR_PHASE_153_READINESS_AUDIT`
