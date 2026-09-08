# C03e-NA — Production Remote Capability Dispatcher Materialization Seam Selection

Status: `SELECTION / STAGING`
Date: `2026-09-08`

Gate on successful closure:
`C03E_NA_PRODUCTION_REMOTE_CAPABILITY_DISPATCHER_MATERIALIZATION_SEAM_SELECTED`

## 1. Decision and scope

C03e-NA performs the documentation-only source-seam assessment authorized by closed C03e-MZ.

Selection result:

`ONE_FILE_LINUX_BOOTSTRAP_STATUS_ONLY_DISPATCHER_MATERIALIZATION_SEAM_SELECTED / EXISTING_TYPED_STATUS_SNAPSHOT_AND_CODEC_REUSED / NO_VISIBILITY_OR_BRIDGE_CHANGE / RUNTIME_WIRING_DEFERRED`

The exact MZ predecessor selected a status-only production remote `CapabilityDispatcher` design: `BridgeCommand::AgentStatus` may return the existing five-byte typed Agent status body after the Phase 143 authorization chain, while files, transfers, terminal and forwarding remain fail-closed unsupported without provider construction.

NA verifies that the current source already exposes every type/value required to materialize that adapter without a new public API, new module path, dependency change, provider construction source, borrowed runtime authority or runtime activation.

The minimum source successor is therefore limited to exactly one existing path:

`crates/prw-agent/src/linux_bootstrap.rs`

NA changes documentation only. It does not implement the dispatcher, modify runtime inputs, construct a dispatcher in production custody, migrate a caller, alter startup/lifecycle assembly, activate a listener, change authentication/policy/crypto, mutate filesystem/network state, alter `main.rs`, deploy or merge.

## 2. Exact predecessor

Repository: `Gersi365/prw-executor-private`

Predecessor: `C03e-MZ`

Branch:
`phase-152-c03e-mz-production-remote-capability-dispatcher-adapter-design-selection`

Exact MZ head:
`2aad80110715cdb2f592b1b6ac9640c9abfb0c8a`

Exact MZ tree:
`bf4d031f81eda1cb49bdcd2bc02e4cef4a9058ef`

Exact MZ contract:
`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_MZ_PRODUCTION_REMOTE_CAPABILITY_DISPATCHER_ADAPTER_DESIGN_SELECTION_STAGING.md`

MZ contract blob:
`afdf4503acc8bdf49c79ddc7ed0efcf448433476`

MZ selection result:

`STATUS_ONLY_REMOTE_DISPATCHER_ADAPTER_DESIGN_SELECTED / AGENT_STATUS_EXISTING_TYPED_PRODUCTION_SNAPSHOT_EXPLICIT_REMOTE_RESULT_PROJECTION / FILE_TRANSFER_TERMINAL_FORWARDING_FAIL_CLOSED_UNSUPPORTED`

MZ authorizes only a separately bounded assessment of the minimum Rust materialization seam. It does not authorize runtime wiring.

## 3. Exact existing-source evidence

### 3.1 Linux production bootstrap seam

Path:
`crates/prw-agent/src/linux_bootstrap.rs`

Exact MZ blob:
`03f24c74f82c94d892d6a8dae561014c0ad60a3f`

This file already imports:

- `prw_remote_bridge::CapabilityDispatcher`;
- `LocalAgentStatusSnapshot`;
- `LocalAgentRuntimeState`.

It already constructs the production-local runtime input bundle with:

`LocalAgentStatusSnapshot::current(LocalAgentRuntimeState::Ready)`.

It also owns the existing production remote-process composition/population seams that accept a generic `D: CapabilityDispatcher + Send + 'static`.

Therefore a crate-private owned dispatcher type can be materialized locally in this existing file without changing crate-root module topology.

### 3.2 Existing typed status snapshot accessor

Path:
`crates/prw-agent/src/linux_production_runtime_loop.rs`

Exact MZ blob:
`d4e5791908f45f53b35e892c9218e4241b93052f`

`LocalLinuxProductionRuntimeInputs<'a>` stores:

`status_snapshot: LocalAgentStatusSnapshot`

and already exposes:

`pub const fn status_snapshot(self) -> LocalAgentStatusSnapshot`.

`LocalAgentStatusSnapshot` is an owned `Copy` value. No borrowed status authority or new accessor is required.

NA therefore rejects any modification to `LocalLinuxProductionRuntimeInputs`, its visibility, its constructor, or its status accessor merely to materialize the adapter.

### 3.3 Existing status type visibility

Path:
`crates/prw-agent/src/local_commands/status_snapshot.rs`

Exact MZ blob:
`587aeefa7b70c27552f5c24d690cdc1316253d80`

`LocalAgentStatusSnapshot` is public, `Clone + Copy`, and exposes the typed runtime-state/protocol-version accessors required by the locked codec.

The containing `local_commands` module is already public from `crates/prw-agent/src/lib.rs`.

No visibility widening is selected or required.

### 3.4 Existing five-byte status codec

Path:
`crates/prw-agent/src/local_commands/status_snapshot/codec.rs`

Exact MZ blob:
`1e63553f6f25ae81017c955c2b7f9ff42f84b235`

The existing public function:

`encode_status_snapshot(LocalAgentStatusSnapshot) -> [u8; LOCAL_AGENT_STATUS_BODY_LENGTH]`

produces the locked status body, and:

`LOCAL_AGENT_STATUS_BODY_LENGTH == 5`.

The selected source successor must call this existing encoder. It may not duplicate the five-byte layout manually or reuse a local IPC response envelope.

### 3.5 Existing remote dispatcher contract

Path:
`crates/prw-remote-bridge/src/lib.rs`

Exact MZ blob:
`ad6833cc4e71a372810b260f157126a3df6645e5`

The existing public boundary is:

`fn dispatch(&mut self, request: &AuthorizedCapabilityRequest) -> Result<Vec<u8>, Self::Error>`.

`AuthorizedCapabilityRequest::command()` returns the already-validated typed `BridgeCommand`.

The selected adapter therefore needs no request parser, correlation field, principal reconstruction, role inference, transport lookup or policy evaluator.

### 3.6 Existing bridge response/error ownership

Path:
`crates/prw-remote-bridge/src/authorized_request_dispatch.rs`

Exact MZ blob:
`d3c25ce18aa56a3924fe2ab2b5f82e3e81bea2aa`

The existing bridge-owned helper:

- invokes the dispatcher only for an already-authorized request;
- maps any backend dispatcher error to `RemoteBridgeError::DispatchFailed`;
- enforces `MAX_CONTROL_PAYLOAD_BYTES`;
- constructs the PRWM response frame;
- preserves the authorized request ID for correlation.

The production adapter must therefore return only result bytes or its bounded backend error. It must not create response frames or new bridge errors.

## 4. Minimum materialization location

The selected materialization location is exactly:

`crates/prw-agent/src/linux_bootstrap.rs`

Reasons:

1. the file already owns Linux production bootstrap composition;
2. it already imports the `CapabilityDispatcher` trait;
3. it already has direct access to the typed production status snapshot type;
4. the existing runtime input bundle exposes the status snapshot by value;
5. the file already carries the `D: CapabilityDispatcher + Send + 'static` production remote-process seams;
6. keeping the type in this file avoids a new module declaration or crate-root path;
7. no Cargo dependency or bridge crate mutation is required.

NA rejects adding a new module/file solely for the initial status-only dispatcher because that would expand the changed-path ceiling without proving a required ownership or visibility benefit.

## 5. Selected dormant adapter shape

The immediate source successor may materialize one crate-private owned adapter in `linux_bootstrap.rs`.

Selected semantic shape:

- one crate-private dispatcher struct;
- exactly one owned field: `LocalAgentStatusSnapshot`;
- one crate-private constructor receiving that snapshot by value;
- one `CapabilityDispatcher` implementation;
- no borrowed fields;
- no `Arc`, mutex, channel, registry, policy, session, transport, filesystem, transfer, terminal or forwarding provider field;
- no process-global configuration read;
- no I/O during construction or dispatch for the selected status operation.

The exact Rust identifier may be:

`LinuxAgentProductionRemoteCapabilityDispatcher`

The source successor must not make this type public.

## 6. Selected bounded adapter error

The source successor may add one crate-private bounded error for unsupported provider-backed command families.

Selected semantic class:

`UnsupportedProviderFamily`

The exact Rust error type may be:

`LinuxAgentProductionRemoteCapabilityDispatchError`

The error should remain zero-data and must not disclose:

- path values;
- transfer identifiers;
- terminal identifiers;
- forwarding identifiers or endpoints;
- principal identifiers;
- configured values;
- provider construction details.

A minimal `Display` and `std::error::Error` implementation is permitted. No public bridge error variant is selected.

## 7. Exact command behavior selected for materialization

The `CapabilityDispatcher` implementation must inspect only `request.command()`.

### `BridgeCommand::AgentStatus`

Return:

`encode_status_snapshot(self.status_snapshot).to_vec()`

This is the exact MZ-selected five-byte status result projection.

The adapter must not prepend or append:

- local IPC request ID;
- local IPC message kind;
- local status prefix;
- local command code;
- PRWC metadata;
- PRWM control-frame metadata;
- a new result version field.

### Every file command

Return the bounded unsupported-provider-family error before any provider operation.

### Every transfer command

Return the bounded unsupported-provider-family error before any provider operation.

### Every terminal command

Return the bounded unsupported-provider-family error before any provider operation.

### Every forwarding command

Return the bounded unsupported-provider-family error before any provider operation.

No unsupported family may return empty, echo, no-op or synthetic success.

## 8. Authorization and identity invariants

The adapter is not an authorization layer.

It must not reinterpret any of the following as remote capability authority:

- PRWM `request_id`;
- command operation code;
- successful request decode;
- local UID;
- process PID/UID/GID;
- endpoint or bind address;
- worker-limit configuration;
- requester/rendezvous record;
- logical device identifier by itself;
- transport identity by itself;
- status snapshot construction.

Phase 143 remains authoritative for session validity, current registry state, current transport identity, exact operation decode and exact capability decision before dispatcher invocation.

The stable identity model remains:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

No status or dispatcher materialization changes that model.

## 9. Ownership and `'static` proof

The selected adapter owns `LocalAgentStatusSnapshot` by value.

Because the snapshot is `Copy` and contains no borrowed runtime state, an adapter constructed from the existing `LocalLinuxProductionRuntimeInputs::status_snapshot()` value can satisfy the existing `D: CapabilityDispatcher + Send + 'static` remote-process bound without:

- leaking the `LocalPrivateDnsSnapshot` lifetime;
- borrowing `LocalLinuxProductionRuntimeInputs`;
- retaining a stack reference;
- introducing `Arc` solely for status custody;
- duplicating mutable host authority.

NA selects no caller migration yet. It only proves that a later caller-composition checkpoint can construct the owned adapter without a lifetime redesign.

## 10. Source-successor ceiling

After NA closure, the immediate source materialization successor is limited to exactly one repository path:

`crates/prw-agent/src/linux_bootstrap.rs`

Permitted changes are limited to:

1. import the existing `AuthorizedCapabilityRequest` / `BridgeCommand` names needed by the implementation;
2. import the existing `encode_status_snapshot` function;
3. add one crate-private owned status-only dispatcher type;
4. add one crate-private bounded unsupported-provider-family error;
5. add minimal `Display` / `Error` only for that bounded error;
6. implement `CapabilityDispatcher` with exact MZ/NA command behavior;
7. add focused tests in the existing `linux_bootstrap.rs` test module or an existing nested test scope;
8. add narrowly required lint acknowledgement only if the dormant source shape requires it.

The successor may not change a second path.

## 11. Focused source-validation expectations

The source successor should prove at minimum:

- the adapter is constructible from an owned `LocalAgentStatusSnapshot`;
- the status projection uses the existing codec and returns exactly five bytes;
- all four unsupported provider families fail closed;
- unsupported dispatch performs no provider construction or provider call;
- no local IPC response framing is copied into the remote result;
- the adapter type satisfies `Send + 'static` as required by the existing production remote-process generic bound;
- repository formatting, Clippy, tests and workspace build pass on the exact final head.

Tests must not require process-global environment mutation for adapter semantics.

## 12. Explicitly deferred caller composition

NA does not select where production startup first constructs this adapter from `LocalLinuxProductionRuntimeInputs::status_snapshot()` and inserts it into the broader production remote-process input population.

That remains a separate checkpoint after dormant adapter source materialization closes.

In particular, the immediate source successor may not:

- alter `with_initial_runtime_inputs` behavior;
- replace an injected dispatcher in an executable caller;
- change `linux_agent_remote_process_operation` invocation;
- change expected-request construction;
- change repeated-admission worker creation;
- activate a production remote listener;
- change `run()` or `main.rs`.

A fresh exact-head audit after source materialization must select the minimum caller-composition seam separately.

## 13. Rejected alternatives

NA explicitly rejects:

- changing `crates/prw-agent/src/lib.rs` to add a new dispatcher module;
- changing `LocalLinuxProductionRuntimeInputs` or adding another status accessor;
- widening visibility of Linux runtime internals;
- modifying `prw-remote-bridge`;
- adding a new public bridge error;
- duplicating the five-byte status codec;
- returning the local IPC response envelope;
- using a test/no-op dispatcher for production custody;
- constructing file/transfer/terminal/forwarding providers;
- selecting filesystem roots, shells, PTYs or forwarding endpoints;
- introducing `Arc`, mutexes or channels without an ownership requirement;
- adding a second policy check inside the dispatcher;
- mapping request ID, endpoint, UID or process identity to capability authority;
- runtime caller migration in the same source-materialization checkpoint;
- listener, network, systemd, package, service, deployment or restart activation;
- database/schema/control-plane mutation;
- authentication or credential cutover;
- merge, PR close, ready-for-review conversion, branch deletion or history rewrite.

## 14. Exact NA repository scope

Only this new documentation path may differ from exact MZ:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_NA_PRODUCTION_REMOTE_CAPABILITY_DISPATCHER_MATERIALIZATION_SEAM_SELECTION_STAGING.md`

No Rust/source, Cargo/lockfile, workflow, packaging/systemd, registry/provider implementation, authentication, policy, networking, `main.rs` or deployment path belongs to NA.

Runtime activation remains `0%`.

## 15. Validation and durable closure requirements

NA closes only after:

- exact MZ predecessor remains `2aad80110715cdb2f592b1b6ac9640c9abfb0c8a`;
- MZ -> NA is exactly one added documentation path with no deletion or source mutation;
- the existing Rust validation workflow on the exact final NA head reaches terminal success;
- every automatically triggered workflow is reported with its actual terminal conclusion; `SKIPPED` is not `PASS`;
- one immutable Markdown audit is written to the canonical Drive evidence parent with exact-title presearch zero, raw readback byte/hash equality and postsearch exactly one;
- only after evidence acceptance may the PR body record `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- the PR remains draft/open/unmerged;
- final race checks confirm MZ, NA, `main` and the next-successor namespace.

## 16. Closure meaning

NA closes only the minimum source-materialization seam question left by MZ.

It selects a one-file dormant Rust successor in `crates/prw-agent/src/linux_bootstrap.rs` that may materialize an owned status-only `CapabilityDispatcher` using the existing typed status snapshot and existing five-byte codec, while all provider-backed command families fail closed.

NA authorizes no runtime caller migration, no listener activation, no provider construction and no deployed functionality increase.

After the source-materialization successor closes, stop and perform a fresh exact-head audit before selecting any production adapter construction/caller propagation or runtime activation boundary.
