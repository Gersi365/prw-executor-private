# C03e-MX — Production Remote Capability Dispatcher / Provider Compatibility Assessment

Status: `ASSESSMENT / STAGING`
Date: `2026-09-08`

Gate on successful closure:
`C03E_MX_PRODUCTION_REMOTE_CAPABILITY_DISPATCHER_PROVIDER_COMPATIBILITY_ASSESSED`

## 1. Decision and scope

C03e-MX performs the documentation-only compatibility assessment authorized by closed C03e-MW.

Assessment result:

`NO_DIRECT_PRODUCTION_DISPATCHER_PROVEN / REUSABLE_TYPED_PROVIDER_PRIMITIVES_PARTIAL / PRODUCTION_PROVIDER_BACKEND_CONSTRUCTION_PREREQUISITE_UNRESOLVED`

No existing source path at the exact MW predecessor proves one complete production `CapabilityDispatcher` construction path that satisfies remote `Send + 'static` custody, already-authorized principal/capability preservation, provider resource ownership, bounded remote response semantics and explicit cleanup across all currently exposed Phase 143 command families.

MX therefore does **not** select or fabricate a dispatcher implementation. The next bounded decision must resolve production typed-provider/backend construction provenance before a later remote dispatcher adapter design or source checkpoint can be selected.

MX changes documentation only. It does not materialize Rust, widen visibility, select host roots or concrete backends, construct requests, mutate policy/authentication, wire a caller or activate runtime.

## 2. Exact predecessor

Repository: `Gersi365/prw-executor-private`
Predecessor: `C03e-MW`
Branch:
`phase-152-c03e-mw-production-remote-capability-dispatcher-provenance-boundary-selection`

Exact MW head: `fed23a0d5c455ae43990cc5be8a9db1128d51f6e`
Exact MW tree: `a759f507f97a2e79c26b09ec0c8b0fa4171dea66`

MW selected this assessment as the immediate successor and explicitly required either one evidence-backed bounded adapter design or the specific missing prerequisite plus the next bounded decision needed.

Integrated `main` remains separately guarded and is not the assessment baseline.

## 3. Authoritative remote dispatch boundary

`crates/prw-remote-bridge/src/lib.rs` remains authoritative for remote authorization and `CapabilityDispatcher`.

Phase 143 authorization completes before dispatcher invocation. `AuthorizedCapabilityRequest` carries the validated principal, current verified transport identity, granted capability, typed `BridgeCommand` and correlation request identifier.

`CapabilityDispatcher::dispatch` therefore consumes an already-authorized request; a provider adapter must not replay local admission, invent authorization, infer a grant from role/transport/construction state, or bypass current registry/policy checks.

`crates/prw-remote-bridge/src/authorized_request_dispatch.rs` retains response framing and maximum-payload enforcement. A provider adapter may produce bounded response bytes or a bounded error, but it does not own the outer PRWM response frame or correlation law.

Exact-MW repository inspection found no non-test concrete `CapabilityDispatcher` implementation that establishes production provider provenance.

## 4. Local typed-provider machinery is compatible only as evidence/primitives

Relevant exact-MW paths:

- `crates/prw-agent/src/local_commands/management_typed_provider_dispatch.rs`;
- `crates/prw-agent/src/local_commands/management_provider_lifecycle.rs`;
- `crates/prw-agent/src/local_commands/management_authority.rs`;
- `crates/prw-agent/src/local_commands/management_response.rs`.

The typed local dispatch seam is useful evidence because it already maps canonical `BridgeCommand` families to typed file, transfer, terminal and forwarding providers and rejects authority-family, filesystem-authority and principal mismatches.

It is **not** a drop-in remote dispatcher:

1. the local management modules are crate-internal (`pub(super)` boundaries);
2. local dispatch requires `LocalManagementAdmission` and local family authority rather than consuming `AuthorizedCapabilityRequest` directly;
3. `LocalManagementProviderLifecycle<'authority, T, F>` borrows Agent-owned filesystem authority, so its lifetime model is not directly the remote `D: Send + 'static` custody model;
4. terminal and forwarding backends are caller-supplied generic inputs and the lifecycle explicitly owns no production backend implementation;
5. local response encoding targets the local IPC status/body contract, not the Phase 143 remote bridge response-byte contract;
6. cleanup is explicit: active transfer/terminal/forwarding resources must be drained before `try_finish` succeeds; `Drop` does not claim cleanup.

MX must not solve these gaps by visibility widening, static leaks, fake roots, no-op backends, always-success responses or duplicated admission.

## 5. Command-family compatibility inventory

### Agent status

A typed local status snapshot path exists, but its authority source and remote response representation are not by themselves a complete production dispatcher construction path.

Classification: `PARTIAL_PRIMITIVE / RESPONSE_ADAPTER_REQUIRED`.

### Files

Descriptor-anchored file-service primitives are real typed provider authority and the local dispatcher already enforces exact filesystem-authority identity before file operations.

The unresolved production question is who opens/owns the exact host filesystem descriptor authority for the remote provider lifetime and how that owned authority satisfies the remote dispatcher custody model without accepting request-controlled roots.

Classification: `REUSABLE_TYPED_PRIMITIVE / HOST_AUTHORITY_PROVENANCE_UNRESOLVED`.

### Transfers

`UploadTransferManager` and bounded download primitives are reusable typed mechanisms tied to the same descriptor-anchored filesystem authority. Their active state also participates in explicit lifecycle drainage.

Classification: `REUSABLE_TYPED_PRIMITIVE / HOST_AUTHORITY_AND_LIFECYCLE_PROVENANCE_UNRESOLVED`.

### Terminal

`TerminalBroker<T>` and `TerminalBackend` provide typed lifecycle/principal boundaries. Exact-MW inspection did not prove a concrete non-test production `TerminalBackend` construction path.

Classification: `BROKER_PRIMITIVE_PRESENT / PRODUCTION_BACKEND_NOT_PROVEN`.

### Forwarding

`PortForwardBroker<F>` and `PortForwardBackend` provide typed lifecycle/principal boundaries. Exact-MW inspection did not prove a concrete non-test production `PortForwardBackend` construction path.

Classification: `BROKER_PRIMITIVE_PRESENT / PRODUCTION_BACKEND_NOT_PROVEN`.

## 6. Principal and authorization compatibility

The remote principal/capability result is authoritative because it is produced only after the Phase 143 session/current-registry/current-transport/policy chain.

The local provider machinery demonstrates that terminal and forwarding resources must remain principal-bound and that filesystem/transfer operations must use the exact owned filesystem authority. That invariant is compatible with the remote boundary, but the existing local authority wrapper is not selected as the remote authority model.

A later adapter design must specify an explicit, lossless mapping from the already-authorized remote principal/capability to the provider principal/resource checks needed for each family. It must prove that no UID-only local admission, role inference, request-controlled path/root, endpoint, session ID, request ID or successful construction can become provider authority.

MX does not select that adapter mapping while the underlying production provider construction inputs remain unresolved.

## 7. Response and error compatibility

Local management response encoding is local-wire-specific and therefore cannot be reused as the remote response contract by convenience.

A later remote adapter must define bounded command-family result bytes compatible with the already-authoritative Phase 143 bridge while preserving:

- bridge-owned request correlation;
- bridge-owned maximum response payload enforcement;
- fail-closed unsupported-command behavior;
- bounded error classification without host/provider detail leakage;
- no synthetic success when provider construction or operation is unavailable.

MX selects no new wire format.

## 8. Ownership and cleanup compatibility

The current local lifecycle borrows filesystem authority but owns terminal/forwarding brokers around caller-provided backends. It explicitly requires transfer, terminal and forwarding resources to be drained before clean completion.

A remote dispatcher stored inside `RemoteSessionExpectedDeviceAdmissionRequest<D, T>` must be constructed before admission and later moved into the authenticated worker under `D: CapabilityDispatcher + Send + 'static`.

The current evidence therefore does not prove that the borrowed local lifecycle can simply be moved into that remote custody boundary. A later design must decide owned versus shared provider-resource custody, session isolation, cancellation cleanup and deterministic terminal/forwarding/transfer drainage without claiming cleanup from `Drop`.

## 9. Rejected shortcuts

MX rejects all of the following as production provenance:

- test, echo, no-op or always-success `CapabilityDispatcher`;
- empty/default provider authority;
- request-controlled filesystem root selection;
- synthetic terminal or forwarding backend;
- direct reuse of local UID admission as remote authorization;
- response-byte reuse that silently treats local IPC encoding as PRWM response semantics;
- visibility widening solely to make local internals callable remotely;
- leaking borrowed authority to obtain a `'static` lifetime;
- unsupported-command success or permissive fallback;
- runtime wiring before provider construction provenance is closed.

## 10. Specific missing prerequisite

The missing prerequisite is:

`PRODUCTION_TYPED_CAPABILITY_PROVIDER_BACKEND_CONSTRUCTION_PROVENANCE`

Before one bounded remote dispatcher adapter design can be selected, evidence must determine how the Agent obtains and owns the concrete production provider inputs required by the Phase 143 command registry, specifically:

1. exact Agent-owned descriptor filesystem authority/root provenance and lifetime;
2. transfer-manager ownership tied to that exact authority;
3. concrete production `TerminalBackend` construction/custody, or an explicit decision that terminal commands remain unsupported and fail closed;
4. concrete production `PortForwardBackend` construction/custody, or an explicit decision that forwarding commands remain unsupported and fail closed;
5. resource isolation and explicit cleanup ownership across authenticated remote sessions;
6. construction inputs that are host/configuration authority rather than request data.

This prerequisite must not be satisfied by convenience defaults or test/disposable implementations.

## 11. Immediate successor ceiling — C03e-MY

After MX validation and evidence closure, the next bounded decision is:

`C03e-MY — production typed capability-provider/backend construction provenance selection`.

MY is a selection checkpoint only. It may inventory and select the already-authorized construction/custody sources for filesystem/transfer/terminal/forwarding provider resources, including an explicit fail-closed unsupported-family choice where no production backend exists.

MY must not implement a remote `CapabilityDispatcher`, widen module visibility, select request-controlled host roots, add a backend dependency, spawn terminal/processes, bind forwarding sockets, mutate filesystem/network state, wire expected requests, change auth/policy/crypto, alter `main.rs`, activate listeners/readiness, deploy or merge.

Any new external dependency or new terminal/forwarding backend architecture requires a separate reviewed architecture/dependency decision rather than implicit selection inside MY.

Only after provider construction provenance is closed may a later checkpoint decide a concrete remote dispatcher adapter design/materialization path.

## 12. Other unresolved expected-request inputs remain unchanged

MX does not resolve or alter:

- SessionId production;
- authentication request-ID allocation;
- verifier-time production;
- admission timing;
- pre-handshake expected-DeviceId scheduling provenance;
- completion/rejection/admission-failure callback policy;
- expected-request channel/producer construction.

C03e-BG dependency ordering remains authoritative: dispatcher/provider custody is a prerequisite, not a reason to fabricate the remaining inputs.

## 13. Exact MX repository scope

Only this new contract may differ from exact MW:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_MX_PRODUCTION_REMOTE_CAPABILITY_DISPATCHER_PROVIDER_COMPATIBILITY_ASSESSMENT.md`

No Rust/source, Cargo/lockfile, workflow, packaging/systemd, registry/provider implementation, authentication, policy, networking, `main.rs` or deployment path belongs to MX.

Runtime activation remains `0%`.

## 14. Validation and durable closure

MX closes only after:

- exact MW predecessor remains `fed23a0d5c455ae43990cc5be8a9db1128d51f6e`;
- MW -> MX is exactly one added documentation path with no deletion or source mutation;
- the existing Rust workflow on the exact final MX head reaches terminal success;
- every automatically triggered workflow is reported with its actual terminal result; `SKIPPED` is not `PASS`;
- one immutable Markdown audit is written to the canonical Drive evidence parent with exact-title presearch zero, raw readback byte/hash equality and postsearch exactly one;
- only after evidence acceptance may the PR body record `ASSESSMENT — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- the PR remains draft/open/unmerged;
- final GitHub/Drive race checks confirm MW, MX, main and successor state.

No source materialization is authorized by MX closure.

## 15. Closure meaning

MX closes only the compatibility assessment required by MW.

It establishes that typed provider primitives are partially reusable but a complete production remote dispatcher cannot yet be selected because production provider/backend construction provenance is unresolved. It selects C03e-MY as the next bounded documentation decision and authorizes no increase in deployed runtime functionality.
