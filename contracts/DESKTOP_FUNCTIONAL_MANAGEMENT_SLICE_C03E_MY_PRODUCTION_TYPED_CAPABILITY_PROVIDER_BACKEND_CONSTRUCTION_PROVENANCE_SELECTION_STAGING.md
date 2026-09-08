# C03e-MY — Production Typed Capability-Provider / Backend Construction Provenance Selection

Status: `SELECTION / STAGING`
Date: `2026-09-08`

Gate on successful closure:
`C03E_MY_PRODUCTION_TYPED_CAPABILITY_PROVIDER_BACKEND_CONSTRUCTION_PROVENANCE_SELECTED`

## 1. Decision and scope

C03e-MY performs the documentation-only provider/backend construction-provenance selection authorized by closed C03e-MX.

Selection result:

`NO_COMPATIBLE_PRODUCTION_REMOTE_PROVIDER_CONSTRUCTION_SOURCE_PROVEN / FILE_TRANSFER_TERMINAL_FORWARDING_REMOTE_FAMILIES_FAIL_CLOSED_UNSUPPORTED`

The exact MX tree contains reusable typed provider primitives, but it does not prove an already-runtime-wired production construction/custody source for the filesystem/transfer authority or a concrete non-test production terminal or forwarding backend. MY therefore selects no fabricated provider, no synthetic backend and no request-controlled host authority.

For the remote dispatcher boundary, file, transfer, terminal and forwarding command families remain unsupported and must fail closed until a separately reviewed checkpoint proves compatible production construction provenance.

MY changes documentation only. It does not implement or wire a `CapabilityDispatcher`, widen visibility, add dependencies, choose a host root, construct a backend, spawn a process, bind a forwarding socket, mutate filesystem/network state, alter authentication/policy/crypto, change `main.rs`, activate runtime, deploy or merge.

## 2. Exact predecessor

Repository: `Gersi365/prw-executor-private`
Predecessor: `C03e-MX`
Branch:
`phase-152-c03e-mx-production-remote-capability-dispatcher-provider-compatibility-assessment`

Exact MX head: `f5c84aa86e3ba5f2eae43504ad421251f55cb555`
Exact MX tree: `682df90a84c0f44480cdc9a85b50a1378f315444`

MX assessment result:

`NO_DIRECT_PRODUCTION_DISPATCHER_PROVEN / REUSABLE_TYPED_PROVIDER_PRIMITIVES_PARTIAL / PRODUCTION_PROVIDER_BACKEND_CONSTRUCTION_PREREQUISITE_UNRESOLVED`

MX explicitly authorized MY to inventory and select already-authorized construction/custody sources for filesystem, transfer, terminal and forwarding resources, including an explicit fail-closed unsupported-family choice where no production backend exists.

Integrated `main` remains separately guarded and is not the MY assessment baseline.

## 3. Authoritative selection rule inherited from MX

MY treats a production provider source as proven only when the exact predecessor tree establishes all required construction and custody facts without convenience defaults:

1. the concrete provider/backend or descriptor authority exists;
2. a production owner obtains or constructs it from trusted host/configuration authority rather than request data;
3. ownership and lifetime are compatible with the later remote `D: CapabilityDispatcher + Send + 'static` custody model;
4. principal/resource isolation remains explicit;
5. cleanup ownership is explicit and does not claim cleanup from `Drop`;
6. unsupported construction or command families fail closed without synthetic success.

A trait, broker, typed primitive, test backend, local-only authority seam, echo/no-op implementation or unmaterialized future bootstrap requirement does not satisfy production provenance.

## 4. Filesystem authority inventory and selection

Exact MX path:
`crates/prw-agent/src/local_commands/management_authority.rs`
Blob: `ae99264cdd53ca0ac39c430f27b6b8904b03d71c`

`LocalManagementFilesystemAuthority` is a real descriptor-anchored authority wrapper around `AnchoredFileRoot`. Its `open_trusted_root` constructor opens a trusted Agent-selected path into descriptor authority and does not accept a request-controlled root through its public surface.

However, the same source explicitly states that this module assembles no provider backend and is not wired into the local server loop. It further states that a later trusted configuration/bootstrap assembly must choose the host path before local management dispatch can receive this authority.

Exact MX path:
`crates/prw-agent/src/local_commands.rs`
Blob: `9b133dacdebdb26fdabc2983f80e9a617c1813bc`

The parent module marks `management_authority` as:
`C02c Agent-owned authority foundation is intentionally not runtime-wired`.

Therefore MY does not treat `LocalManagementFilesystemAuthority::open_trusted_root` as an existing production remote construction source. The typed authority primitive is reusable evidence, but production host-root selection, ownership and remote-compatible lifetime remain unmaterialized.

Selection:
`FILES = FAIL_CLOSED_UNSUPPORTED / DESCRIPTOR_PRIMITIVE_PRESENT / PRODUCTION_HOST_AUTHORITY_SOURCE_NOT_PROVEN`.

No request path, environment fallback, current working directory, home directory, `/`, temporary directory or leaked/static borrowed authority may be selected by convenience.

## 5. Transfer provider inventory and selection

Exact MX path:
`crates/prw-agent/src/local_commands/management_provider_lifecycle.rs`
Blob: `d7edae3182882ae29d17245a0ee5fc62e831fc45`

`LocalManagementProviderLifecycle<'authority, T, F>` constructs `UploadTransferManager<'authority>` from the borrowed Agent-owned filesystem authority and requires explicit resource drainage before clean completion.

The lifecycle source explicitly states that it owns no production backend implementation, is not wired into the local server loop, and composes caller-supplied backends around the Agent-owned filesystem authority.

The parent module likewise marks `management_provider_lifecycle` as intentionally not runtime-wired.

Transfer construction is therefore coupled to the unresolved filesystem authority provenance and to a borrowed lifetime model not proven compatible with remote `Send + 'static` dispatcher custody.

Selection:
`TRANSFERS = FAIL_CLOSED_UNSUPPORTED / TYPED_TRANSFER_PRIMITIVE_PRESENT / HOST_AUTHORITY_AND_REMOTE_LIFETIME_SOURCE_NOT_PROVEN`.

MY selects no independent transfer root and no lifecycle shortcut that would discard active transfer state.

## 6. Terminal backend inventory and selection

Exact MX assessment found `TerminalBroker<T>` and the `TerminalBackend` trait as typed lifecycle/principal primitives, but no concrete non-test production `TerminalBackend` construction path.

`LocalManagementProviderLifecycle` accepts terminal backend `T` as a caller-supplied generic input and explicitly owns no production backend implementation.

MY found no predecessor evidence authorizing a new dependency, process-spawning backend architecture, test backend promotion, no-op backend or synthetic terminal implementation.

Selection:
`TERMINAL = FAIL_CLOSED_UNSUPPORTED / BROKER_AND_TRAIT_PRESENT / PRODUCTION_BACKEND_NOT_PROVEN`.

Terminal commands must not produce synthetic success while this state holds.

## 7. Forwarding backend inventory and selection

Exact MX assessment found `PortForwardBroker<F>` and the `PortForwardBackend` trait as typed lifecycle/principal primitives, but no concrete non-test production `PortForwardBackend` construction path.

`LocalManagementProviderLifecycle` accepts forwarding backend `F` as a caller-supplied generic input and explicitly owns no production backend implementation.

MY found no predecessor evidence authorizing a new network backend architecture, dependency, socket-binding implementation, no-op backend or test backend promotion.

Selection:
`FORWARDING = FAIL_CLOSED_UNSUPPORTED / BROKER_AND_TRAIT_PRESENT / PRODUCTION_BACKEND_NOT_PROVEN`.

Forwarding commands must not bind sockets or produce synthetic success while this state holds.

## 8. Local typed-provider seam remains evidence, not remote production custody

Exact MX paths:

- `crates/prw-agent/src/local_commands/management_authority.rs`;
- `crates/prw-agent/src/local_commands/management_provider_lifecycle.rs`;
- `crates/prw-agent/src/local_commands/management_typed_provider_dispatch.rs`;
- `crates/prw-agent/src/local_commands/management_provider_backend_policy.rs`;
- `crates/prw-agent/src/local_commands.rs`.

These modules demonstrate useful authority separation, exact filesystem-authority binding, terminal/forwarding principal binding, typed provider dispatch and explicit cleanup semantics.

They remain local, crate-internal staging seams. The parent module explicitly marks the relevant authority, lifecycle, backend-policy and typed-dispatch modules as intentionally not runtime-wired. MY therefore does not widen their visibility or reinterpret their existence as production remote custody.

## 9. Production remote selection matrix

| Family | Existing primitive evidence | Proven production construction/custody at exact MX | MY selection |
|---|---|---|---|
| Files | `AnchoredFileRoot` through `LocalManagementFilesystemAuthority` | No; trusted host-root bootstrap is explicitly deferred/not runtime-wired | `FAIL_CLOSED_UNSUPPORTED` |
| Transfers | `UploadTransferManager` bound to filesystem authority | No; depends on unresolved host authority and borrowed lifecycle | `FAIL_CLOSED_UNSUPPORTED` |
| Terminal | `TerminalBroker<T>` + `TerminalBackend` trait | No concrete non-test production backend proven | `FAIL_CLOSED_UNSUPPORTED` |
| Forwarding | `PortForwardBroker<F>` + `PortForwardBackend` trait | No concrete non-test production backend proven | `FAIL_CLOSED_UNSUPPORTED` |

This matrix is a production provenance decision, not a claim that reusable provider primitives are absent from the repository.

## 10. Authority, identity and failure invariants

The canonical identity law remains:
`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`.

Remote authorization must complete before dispatcher/provider side effects. DeviceId, SessionId, request IDs, endpoint data, host paths, ports, request payloads, local UID admission, successful object construction and queue membership are not grants.

For the selected unsupported families:

- no provider operation may be attempted by convenience;
- no missing backend may be represented as success;
- no request-controlled root or port becomes host authority;
- no local UID authority substitutes for remote registry/session/capability authority;
- no borrowed authority is leaked or promoted to `'static`;
- no cleanup guarantee is inferred from `Drop`.

## 11. Rejected alternatives

MY explicitly rejects:

- selecting `/`, the current directory, a home directory or a temporary directory as an implicit filesystem root;
- adding a new environment/configuration root source inside this selection checkpoint;
- making the local authority/lifecycle seams runtime-wired or public to satisfy remote custody;
- static leaking of borrowed filesystem authority;
- promoting test terminal/forwarding backends to production;
- creating no-op, echo or always-success terminal/forwarding implementations;
- adding terminal/process or forwarding/network dependencies without a separate reviewed decision;
- treating local IPC response encoding as the remote bridge response contract;
- wiring expected requests or runtime activation before a separately authorized adapter/materialization decision.

## 12. Closure of the MX prerequisite

MX named the missing prerequisite:
`PRODUCTION_TYPED_CAPABILITY_PROVIDER_BACKEND_CONSTRUCTION_PROVENANCE`.

MY closes the selection question by recording that the exact predecessor does not provide compatible production construction provenance for file/transfer/terminal/forwarding remote provider families and by selecting explicit fail-closed unsupported behavior for all four.

This is a closed provenance decision, not a production implementation. A later checkpoint may reconsider one family only when exact repository evidence or a separately reviewed architecture/dependency decision introduces a compatible production construction source.

## 13. Successor ceiling

After MY validation and durable evidence closure, a later separately bounded checkpoint may assess or select a remote dispatcher adapter design that preserves the existing Phase 143 authorization and response boundaries while returning fail-closed unsupported behavior for families without production providers.

MY does not authorize that adapter's Rust materialization, visibility changes, provider construction, expected-request wiring, startup/lifecycle integration, runtime activation, deployment or merge.

If supporting files, transfers, terminal or forwarding requires a new production construction source, each such source remains separately gated before adapter materialization may claim support for that family.

## 14. Exact MY repository scope

Only this new contract may differ from exact MX:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_MY_PRODUCTION_TYPED_CAPABILITY_PROVIDER_BACKEND_CONSTRUCTION_PROVENANCE_SELECTION_STAGING.md`

No Rust/source, Cargo/lockfile, workflow, packaging/systemd, registry/provider implementation, authentication, policy, networking, `main.rs` or deployment path belongs to MY.

Runtime activation remains `0%`.

## 15. Validation and durable closure

MY closes only after:

- exact MX predecessor remains `f5c84aa86e3ba5f2eae43504ad421251f55cb555`;
- MX -> MY is exactly one added documentation path with no deletion or source mutation;
- the existing Rust workflow on the exact final MY head reaches terminal success;
- every automatically triggered workflow is reported with its actual terminal result; `SKIPPED` is not `PASS`;
- one immutable Markdown audit is written to the canonical Drive evidence parent with exact-title presearch zero, raw readback byte/hash equality and postsearch exactly one;
- only after evidence acceptance may the PR body record `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- the PR remains draft/open/unmerged;
- final GitHub/Drive race checks confirm MX, MY, main and successor state.

No source materialization is authorized by MY closure.

## 16. Closure meaning

MY closes only the production typed capability-provider/backend construction-provenance selection required by MX.

It establishes that the exact predecessor contains reusable typed primitives but no proven production remote construction/custody source for file, transfer, terminal or forwarding provider families. Those families are therefore explicitly fail-closed unsupported until a separately reviewed production construction source exists.

MY authorizes no increase in deployed runtime functionality.