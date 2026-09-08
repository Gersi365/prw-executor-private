# C03e-MZ — Production Remote Capability Dispatcher Adapter Design Selection

Status: `SELECTION / STAGING`
Date: `2026-09-08`

Gate on successful closure:
`C03E_MZ_PRODUCTION_REMOTE_CAPABILITY_DISPATCHER_ADAPTER_DESIGN_SELECTED`

## 1. Decision and scope

C03e-MZ performs the documentation-only remote capability-dispatcher adapter design selection authorized by closed C03e-MY.

Selection result:

`STATUS_ONLY_REMOTE_DISPATCHER_ADAPTER_DESIGN_SELECTED / AGENT_STATUS_EXISTING_TYPED_PRODUCTION_SNAPSHOT_EXPLICIT_REMOTE_RESULT_PROJECTION / FILE_TRANSFER_TERMINAL_FORWARDING_FAIL_CLOSED_UNSUPPORTED`

The exact MY predecessor proves one production-owned typed Agent status snapshot source and preserves the Phase 143 post-authorization `CapabilityDispatcher` seam. It does not prove compatible production provider construction for files, transfers, terminal or forwarding; MY explicitly selected those four families fail-closed unsupported.

MZ therefore selects one bounded design in which an already-authorized `BridgeCommand::AgentStatus` may project the existing typed Agent status snapshot into an explicit five-byte remote result payload. The Phase 143 bridge remains the sole owner of request correlation, response-frame construction and maximum response-payload enforcement.

All file, transfer, terminal and forwarding commands return a bounded dispatcher failure before any provider operation. They do not construct providers and do not produce synthetic success.

MZ changes documentation only. It does not implement `CapabilityDispatcher`, widen visibility, add dependencies, alter provider construction, wire expected requests, change startup/lifecycle assembly, activate listeners, change authentication/policy/crypto, mutate filesystem/network state, alter `main.rs`, deploy or merge.

## 2. Exact predecessor

Repository: `Gersi365/prw-executor-private`
Predecessor: `C03e-MY`
Branch:
`phase-152-c03e-my-production-typed-capability-provider-backend-construction-provenance-selection`

Exact MY head: `ee2b34b61855f73e533e6f688fe4c71a3faf901d`
Exact MY tree: `b09a58c83e396943d15031a4bf132758cbeb2b77`
MY contract blob: `d20f85d1002a3408bd943658b2c2bbcc983ea08c`

MY selection result:

`NO_COMPATIBLE_PRODUCTION_REMOTE_PROVIDER_CONSTRUCTION_SOURCE_PROVEN / FILE_TRANSFER_TERMINAL_FORWARDING_REMOTE_FAMILIES_FAIL_CLOSED_UNSUPPORTED`

MY authorizes a later separately bounded adapter design assessment/selection that preserves existing Phase 143 authorization and response boundaries while failing closed for provider families without proven production construction sources. It does not authorize adapter Rust materialization or runtime wiring.

## 3. Authoritative Phase 143 dispatcher boundary

Exact MY contract:
`contracts/END_TO_END_AUTHENTICATED_CAPABILITY_BRIDGE_CONTRACT.md`
Blob: `1466aa61549b7512b39a847af4f61e4cd047204b`

Phase 143 remains authoritative:

1. PRWM request framing is validated;
2. remote application-session lease is validated;
3. authenticated session is revalidated against the current registry;
4. presented transport identity must equal current registered transport identity;
5. PRWC payload decodes to one supported typed `BridgeCommand`;
6. exact required capability must evaluate to `Allow`;
7. only then may `CapabilityDispatcher` receive the authorized request.

`AuthorizedCapabilityRequest` carries the correlated request ID, current registry principal snapshot, current transport identity, exact granted capability and typed command.

MZ adds no authorization rule and performs no role inference. A successful transport connection, request decode, status construction, command identifier, request identifier or local UID never becomes capability authority.

## 4. Exact bridge response and error ownership

Exact MY source:
`crates/prw-remote-bridge/src/lib.rs`
Blob: `ad6833cc4e71a372810b260f157126a3df6645e5`

`CapabilityDispatcher` has a backend-specific associated error and receives only `&AuthorizedCapabilityRequest`:

`fn dispatch(&mut self, request: &AuthorizedCapabilityRequest) -> Result<Vec<u8>, Self::Error>`.

Exact MY source:
`crates/prw-remote-bridge/src/authorized_request_dispatch.rs`
Blob: `d3c25ce18aa56a3924fe2ab2b5f82e3e81bea2aa`

The bridge-owned helper:

- calls the dispatcher only after authorization;
- maps every dispatcher error to `RemoteBridgeError::DispatchFailed`;
- rejects dispatcher output above `MAX_CONTROL_PAYLOAD_BYTES`;
- constructs `ControlMessageKind::Response` itself;
- reuses the exact authorized request ID for correlation;
- maps response-frame construction rejection to the existing bounded bridge error.

Therefore the selected adapter returns only bounded result bytes or a bounded backend error. It never constructs a PRWM response frame and never owns request correlation.

## 5. Production Agent status provenance

Exact MY source:
`crates/prw-agent/src/linux_bootstrap.rs`
Blob: `03f24c74f82c94d892d6a8dae561014c0ad60a3f`

The production Linux bootstrap constructs `LocalLinuxProductionRuntimeInputs` with:

`LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready)`.

This is production bootstrap provenance, not a test backend, echo/no-op implementation or request-controlled value.

MZ does not reinterpret the local same-UID trust boundary as remote authority. The snapshot is merely the already-existing typed status value available after the Phase 143 remote authorization chain has independently granted `AgentStatusRead`.

## 6. AgentStatus remote result projection

Exact MY source:
`crates/prw-agent/src/local_commands/status_snapshot/codec.rs`
Blob: `1e63553f6f25ae81017c955c2b7f9ff42f84b235`

The existing typed status body has a locked five-byte representation:

- byte 0: `LocalAgentRuntimeState` code;
- bytes 1-2: local IPC protocol major, network byte order;
- bytes 3-4: local IPC protocol minor, network byte order.

`LOCAL_AGENT_STATUS_BODY_LENGTH` is exactly `5` and the existing codec rejects non-exact lengths, unknown runtime states and unsupported protocol versions.

MZ explicitly selects those five status-body bytes as the `AgentStatus` dispatcher result payload.

This is an explicit remote result projection decision, not silent reuse of the local IPC response envelope. The selected remote result contains:

- no local IPC request identifier;
- no local IPC response status prefix;
- no local command code;
- no PRWC request header;
- no PRWM `ControlFrame` header.

Only the five typed status-body bytes are returned by the adapter. Phase 143 remains responsible for the outer PRWM `Response` frame and correlation.

No additional remote result version field is invented in MZ. The status snapshot already carries the local protocol version as part of its locked typed body.

## 7. Why this does not create an alternate command protocol

The Phase 143 PRWC request registry remains unchanged. Operation `1` remains `BridgeCommand::AgentStatus` and requires `AgentStatusRead`.

MZ defines only the bounded result projection for that already-existing typed operation. It does not add an operation code, command string, alternate request codec or generic execution surface.

The desktop functional-management contract remains authoritative that Phase 143 `BridgeCommand` and PRWC are the canonical remote capability operation registry.

## 8. Unsupported provider-family behavior

MY closed production provider construction for the following families as unsupported:

- files;
- transfers;
- terminal;
- forwarding.

MZ preserves that decision exactly.

For every already-authorized command in those families, the selected adapter design returns its bounded backend error without:

- opening a filesystem root;
- creating or touching a transfer manager;
- spawning a terminal/process;
- binding or connecting a forwarding socket;
- calling a test/no-op provider;
- creating a permissive fallback;
- manufacturing an empty or always-success payload.

The existing Phase 143 bridge maps that backend error to `RemoteBridgeError::DispatchFailed` and therefore produces no successful response frame.

MZ does not add a new public bridge error variant for unsupported providers.

## 9. Command-family design matrix

| Family | MY production provenance | MZ selected adapter behavior |
|---|---|---|
| Agent status | Production bootstrap constructs typed `LocalAgentStatusSnapshot` | Return explicit five-byte status-body projection after Phase 143 authorization |
| Files | Production host authority source not proven | Bounded dispatcher error -> `DispatchFailed`; no provider call |
| Transfers | Host authority and remote lifetime source not proven | Bounded dispatcher error -> `DispatchFailed`; no provider call |
| Terminal | Concrete production backend not proven | Bounded dispatcher error -> `DispatchFailed`; no provider call |
| Forwarding | Concrete production backend not proven | Bounded dispatcher error -> `DispatchFailed`; no provider call |

This matrix is a design selection only. It does not claim the adapter has been materialized or wired into production custody.

## 10. Principal and resource mapping

For `AgentStatus`, the Phase 143 authorized request already proves:

- current application-session validity;
- current registry membership/device validity;
- current transport identity;
- exact PRWC operation decode;
- exact `AgentStatusRead` capability allow.

The status projection performs no independent provider-side resource grant and owns no principal-bound mutable resource.

For the four unsupported families, MZ performs no provider principal/resource mapping because provider dispatch is not attempted.

No local UID admission, workspace role, path, endpoint, transfer ID, terminal ID, forwarding ID or successful object construction is promoted to remote authorization.

## 11. Response and error invariants

The selected design preserves:

- bridge-owned request correlation;
- bridge-owned control-payload ceiling;
- bridge-owned `ControlFrame::Response` construction;
- no response frame on dispatcher failure;
- bounded dispatcher error without host/provider-detail leakage;
- no local IPC envelope reuse;
- no synthetic success for unsupported families;
- no provider side effect before or after a selected unsupported-family failure.

MZ does not redefine `RemoteBridgeError`, PRWM framing or PRWC request encoding.

## 12. Ownership and lifetime design

The status-only adapter requires only an owned/copyable typed Agent status snapshot source compatible with remote dispatcher custody. It does not require filesystem authority, transfer lifecycle ownership, terminal broker/backend or forwarding broker/backend.

MZ does not select a visibility change or runtime assembly path for constructing that adapter. Exact materialization must separately prove how the already-existing production status snapshot reaches an owned `D: CapabilityDispatcher + Send + 'static` value without leaking borrowed state or duplicating host authority.

Because the unsupported provider families are not constructed, MZ introduces no transfer/terminal/forwarding cleanup obligation.

## 13. Rejected alternatives

MZ explicitly rejects:

- reusing the local IPC response envelope as PRWM response framing;
- returning local response-status prefixes from the dispatcher;
- inventing a new generic remote response envelope inside the adapter;
- treating the five-byte status body as authorization evidence;
- allowing files/transfers/terminal/forwarding to return empty, echo or no-op success;
- adding a test backend to satisfy production custody;
- choosing a filesystem root by environment/current-directory/home/root/temp convenience;
- spawning a shell/process or binding forwarding sockets;
- widening crate visibility solely inside this selection checkpoint;
- leaking borrowed authority to satisfy `'static`;
- performing a second policy/role inference path inside the adapter;
- changing Phase 143 request operation codes or capability mapping;
- wiring the adapter into expected-request construction, startup or authenticated workers in MZ.

## 14. Materialization prerequisite left by MZ

MZ selects the adapter design but does not prove or authorize its Rust materialization seam.

A later separately bounded checkpoint must assess the minimum source seam needed to:

1. expose or project the production-owned typed status snapshot to a remote adapter without weakening visibility/authority boundaries;
2. construct an owned `CapabilityDispatcher + Send + 'static` value before authenticated worker admission;
3. preserve the exact five-byte status result semantics without copying local IPC response framing;
4. represent unsupported provider families through a bounded adapter error that maps to existing `DispatchFailed`;
5. avoid provider construction for unsupported families;
6. preserve Phase 143 authorization, correlation and response-size ownership unchanged.

That assessment must choose the minimum existing-source reuse path or explicitly record the specific missing prerequisite. It may not materialize runtime wiring unless separately authorized by its own predecessor boundary.

## 15. Exact MZ repository scope

Only this new contract may differ from exact MY:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_MZ_PRODUCTION_REMOTE_CAPABILITY_DISPATCHER_ADAPTER_DESIGN_SELECTION_STAGING.md`

No Rust/source, Cargo/lockfile, workflow, packaging/systemd, registry/provider implementation, authentication, policy, networking, `main.rs` or deployment path belongs to MZ.

Runtime activation remains `0%`.

## 16. Validation and durable closure

MZ closes only after:

- exact MY predecessor remains `ee2b34b61855f73e533e6f688fe4c71a3faf901d`;
- MY -> MZ is exactly one added documentation path with no deletion or source mutation;
- the existing Rust workflow on the exact final MZ head reaches terminal success;
- every automatically triggered workflow is reported with its actual terminal result; `SKIPPED` is not `PASS`;
- one immutable Markdown audit is written to the canonical Drive evidence parent with exact-title presearch zero, raw readback byte/hash equality and postsearch exactly one;
- only after evidence acceptance may the PR body record `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- the PR remains draft/open/unmerged;
- final GitHub/Drive race checks confirm MY, MZ, main and successor state.

No Rust adapter materialization is authorized by MZ closure.

## 17. Closure meaning

MZ closes only the remote dispatcher adapter design-selection question authorized by MY.

It selects a bounded status-only design: `AgentStatus` may return the existing five-byte typed production status body after Phase 143 authorization, while files/transfers/terminal/forwarding remain explicit fail-closed dispatcher failures with no provider construction.

MZ authorizes no increase in deployed runtime functionality.