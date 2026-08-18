# PRW Phase 152 C02c Authority Foundation Audit

Status: `IMPLEMENTATION_STAGED / PRE_BUILD_STATIC_CORRECTIVE_COMPLETE / BUILD_GATE_CLOSED / NO_RUNTIME_ACTIVATION`

- repository_id: `1334911207`
- canonical_repository: `powercode2026/prw-executor-private`
- frozen_predecessor: `01f5466504684ea6a2c504613901d24018485887`
- branch: `phase-152-c02c-authority-foundation`
- implementation_head_before_audit_refresh: `72d3636abf8d3c017e9f0128f5cf4c992b2a0dd3`

## Scope

C02c resumes Phase 152 after successful real-host reconciliation and stages the smallest complete crate-internal management implementation seam that remains unreachable from production runtime.

Changed files relative to the frozen predecessor immediately before this audit refresh:

- `.github/workflows/phase-152-c02c-implementation-validation.yml`
- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02C_AUTHORITY_FOUNDATION_GATE.md`
- `crates/prw-agent/Cargo.toml`
- `crates/prw-agent/src/local_commands.rs`
- `crates/prw-agent/src/local_commands/management_authority.rs`
- `crates/prw-agent/src/local_commands/management_dispatch.rs`
- `crates/prw-agent/src/local_commands/management_execution.rs`
- `crates/prw-agent/src/local_commands/management_provider_lifecycle.rs`
- `crates/prw-agent/src/local_commands/management_response.rs`
- `crates/prw-agent/src/local_commands/management_typed_provider_dispatch.rs`
- `crates/prw-policy/src/lib.rs`
- this audit file

Static compare result immediately before this audit refresh:

- branch relation: `ahead 30 / behind 0`
- changed files: `12`
- additions: `2040`
- deletions: `17`
- merge base: exact frozen predecessor `01f5466504684ea6a2c504613901d24018485887`
- root `Cargo.toml`: `UNCHANGED`
- root `Cargo.lock`: `UNCHANGED`
- `crates/prw-agent/src/main.rs`: `UNCHANGED`
- production runtime loop/bootstrap files: `UNCHANGED`
- desktop/Android source: `UNCHANGED`
- deployment/service-manager files: `UNCHANGED`
- production runtime signing/systemd credential loading: `UNCHANGED / NOT_ACTIVATED`

## Authority foundation

### Remote-session identity

`LocalManagementRemoteSessionAuthority` requires an already-authenticated `AuthenticatedDeviceSession` and current `WorkspaceDeviceRegistry::validate_authenticated_session` revalidation.

It retains only:

- the returned `RegistryValidatedPrincipal`; and
- the authenticated PRW `SessionId` copied from the same session.

Terminal and forwarding principals are derived only through:

- `TerminalPrincipal::from_registry`;
- `ForwardingPrincipal::from_registry`.

No workspace/user/device/session identity is fabricated from local PID/UID/GID, request bytes, desktop process identity, or ambient process state.

Pure constructors/accessors that create or return authority evidence are explicitly marked `#[must_use]` in the staged source so discarded authority values are not silently treated as meaningful work.

### Filesystem authority

`LocalManagementFilesystemAuthority` owns one `prw_file_service::AnchoredFileRoot` opened by a crate-internal trusted-root constructor.

Request bytes carry only validated `RemotePath` values and cannot select or replace the host root.

`LocalManagementFamilyAuthority` is crate-internal with private variants. File/transfer values reference the real filesystem authority; terminal/forwarding values reference the real registry/session authority.

### Request-bound context

`LocalManagementAuthorityContext::from_agent_owned_authority` requires an already-admitted request plus real family authority and fails closed on family mismatch.

The resulting context binds only admission-derived request ID, kernel peer credentials, capability, operation code, and required family.

## Explicit policy seam

`prw-policy` stages `BoundedLocalManagementPolicy` as a separate evaluator. The existing production `BoundedLocalReadPolicy::allow_local_reads()` remains unchanged.

Management capability decisions are carried by `BoundedLocalManagementDecisions`, whose fields are named by capability family rather than supplied as an order-sensitive seven-argument constructor.

The policy constructor accepts that single explicit decision bundle. No `allow_all()` constructor exists.

Capabilities outside the reviewed bridge surface remain fail-closed, including:

- `FilesDelete`;
- `DeviceManage`;
- `PolicyManage`.

Production management-policy selection remains absent.

## Provider lifecycle ownership

`LocalManagementProviderLifecycle<'authority, T, F>` composes:

- `&LocalManagementFilesystemAuthority`;
- `UploadTransferManager<'authority>` borrowing the anchored root;
- `TerminalBroker<T>`;
- `PortForwardBroker<F>`.

The transfer manager cannot outlive the filesystem authority by construction. No provider backend is selected from request data.

### Explicit quiescence

The lifecycle deliberately has no `Drop` implementation claiming provider cleanup.

`try_finish(self)` returns `Ok(())` only when:

- active transfer count is zero;
- terminal broker is empty;
- forwarding broker is empty.

If state remains active, `try_finish` returns `Err(Box<Self>)`. The box preserves the complete lifecycle owner for continued typed cleanup while avoiding a large inline error variant. Active state is not silently discarded and is not recorded as clean rollback evidence.

Lifecycle constructors/accessors and quiescence observations are explicitly `#[must_use]` where applicable.

## Typed provider dispatch

`dispatch_admitted_management_command` operates only on the already-decoded canonical `BridgeCommand` plus real family authority and an already-assembled lifecycle.

It covers the existing typed operations for:

- Agent status;
- descriptor-anchored file list/stat/create/directory-create;
- upload begin/resume/chunk/finalize/abort;
- bounded download chunks;
- terminal open/input/resize/read/close;
- forwarding open/close.

It does not accept raw shell text, executable paths, host roots, DNS names, arbitrary bind addresses, environment bags, privilege instructions, routes, firewall rules, or provider configuration.

### Pre-mutation security guards

Every typed dispatch first proves family correlation through the request-bound authority constructor.

File/transfer operations additionally require reference identity between the supplied family authority and the lifecycle's exact filesystem authority. A family token for root A cannot drive lifecycle root B.

Terminal/forwarding operations against existing broker records derive the current provider principal from registry-revalidated PRW-session authority and compare it with the immutable principal stored in the record before mutation.

This prevents cross-principal reuse of known terminal or forwarding broker identifiers.

### Pre-build structural corrective

The original single large typed-dispatch match was split into file, transfer, terminal, and forwarding family helpers before toolchain validation. This preserves the same outer exhaustive `BridgeCommand` classification while reducing function-size/lint risk.

Each family helper retains a fail-closed wildcard guard for commands outside its preclassified family. Those local wildcard arms carry narrow `clippy::wildcard_enum_match_arm` allowances with reasons; no crate- or workspace-wide suppression was added.

## Deterministic response semantics

The existing local terminal-response framing remains authoritative:

- two-byte `LocalAgentResponseStatus` prefix;
- `Response` outer kind only for `Ok`;
- `Error` outer kind for non-success;
- existing request ID correlation.

C02c success bodies add one stable result tag:

- `1` — Agent status + existing five-byte status codec;
- `2` — directory list: `u16 count` then repeated `u8 type + u16 name_len + UTF-8 name`;
- `3` — metadata: `u8 type + u64 size`;
- `4` — empty acknowledgement;
- `5` — big-endian `u64` offset;
- `6` — bounded raw bytes.

File-type codes are:

- `1` regular file;
- `2` directory;
- `3` symbolic link;
- `4` other.

All success bodies must fit the existing local terminal-response body bound. If provider success cannot be encoded within that bound, the response becomes correlated `InternalError` with an empty body; an encoding failure never creates `Ok`.

Provider strings and host details are never serialized.

Typed failures collapse into the existing coarse statuses:

- invalid operation/bound semantics → `InvalidRequest`;
- stale/duplicate/missing state or authority/principal mismatch → `Conflict`;
- backend/filesystem/storage/postcondition/encoding failure → `InternalError`.

Capability denial occurs earlier and remains `Unauthorized`.

External provider error enums are non-exhaustive. Their fail-closed wildcard fallbacks are locally annotated with a narrow lint allowance and reason so future unknown variants remain `InternalError` without adding a global warning suppression.

## Complete C02c execution seam

`process_authenticated_linux_management_with_typed_providers` composes:

1. existing authenticated C01 admission;
2. canonical decode and exact capability policy evaluation;
3. caller-supplied real family authority;
4. typed provider dispatch with exact root/principal guards;
5. deterministic correlated response construction.

This function is crate-internal and is not called by:

- production local server loop;
- Linux production runtime loop;
- Linux bootstrap;
- `main.rs`;
- service-manager integration;
- deployment code.

Therefore C02c is implementation-staged, not runtime-activated.

## Dependency delta and cycle inspection

`prw-agent` adds only existing workspace path dependencies required by the C02c seams:

- `prw-file-service`;
- `prw-file-transfer`;
- `prw-forwarding`;
- `prw-registry`;
- `prw-session`;
- `prw-terminal`.

Static frozen-manifest inspection confirms no dependency returns to `prw-agent`:

- `prw-file-service` → `rustix`, `aws-lc-rs`;
- `prw-file-transfer` → `prw-file-service`;
- `prw-forwarding` → `prw-core`, `prw-registry`;
- `prw-terminal` → `prw-core`, `prw-registry`, `prw-session`;
- `prw-registry` → `prw-connectivity`, `prw-control-plane`, `prw-core`, `prw-session`.

No external crate version was added and no lockfile was changed by C02c.

## Contract/source alignment

`DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02C_AUTHORITY_FOUNDATION_GATE.md` matches the staged source semantics: no fabricated identity, no request-selected host root, exact root/principal binding, no fabricated Drop cleanup, deterministic bounded response semantics, and no runtime activation.

The explicit quiescence contract remains intact after boxing the active lifecycle return path: boxing changes only the in-memory error representation, not cleanup or authority semantics.

## Manual-only implementation-validation specification

`.github/workflows/phase-152-c02c-implementation-validation.yml` is staged as a validation specification with **only** a `workflow_dispatch` trigger. It has no `push`, `pull_request`, schedule, deployment, or environment trigger.

The specification requires an exact caller-supplied 40-character approved head, exact C02c branch name, frozen-lineage ancestry, and an allowlisted changed-file scope before any Cargo command.

Its future validation sequence is limited to:

1. `cargo metadata --locked --no-deps` and deterministic lockfile comparison;
2. `cargo fmt --all -- --check`;
3. `cargo clippy --locked -p prw-policy -p prw-agent --all-targets --all-features -- -D warnings`;
4. `cargo test --locked -p prw-policy -p prw-agent --all-targets`;
5. `cargo build --locked -p prw-policy -p prw-agent --all-targets`;
6. final `git diff --exit-code`.

The workflow definition itself is **not** build evidence. It has not been dispatched or run under the current gate.

## Pre-build static corrective summary

Before opening any toolchain gate, the staged source was tightened by inspection to reduce predictable strict-warning failures:

- split the typed provider dispatcher into family-specific helpers;
- changed active lifecycle finish failure from large inline `Self` to `Box<Self>`;
- added `#[must_use]` to authority/lifecycle constructors and accessors where discarded values would be misleading;
- replaced the seven-position management policy constructor with named `BoundedLocalManagementDecisions`;
- simplified duplicated provider error mappings;
- documented narrow wildcard enum lint allowances only at intentional fail-closed family/error boundaries.

No `cargo`, `rustfmt`, `clippy`, test, build, runtime, package, deployment, or privileged command was executed as part of these corrective changes.

## Validation classification

The project build gate remains closed. Therefore this audit claims no Cargo, format, Clippy, test, build, runtime, service-manager, signing, systemd-credential, deployment, or privileged validation.

Current validation is limited to connector-grounded source/API inspection, exact dependency-manifest inspection, security-boundary review, contract/source alignment, static warning-risk reduction, workflow-definition inspection, and exact GitHub diff scope.

- source syntax/build validation: `NOT_RUN / BUILD_GATE_CLOSED`
- formatter/linter/tests: `NOT_RUN / BUILD_GATE_CLOSED`
- manual validation workflow: `STAGED / NOT_RUN`
- runtime validation: `NOT_RUN`
- production management policy: `NOT_SELECTED`
- runtime signing: `NOT_AUTHORIZED`
- systemd credential loading: `NOT_AUTHORIZED`
- deployment/privileged changes: `NOT_AUTHORIZED`
- C03: `NOT_AUTHORIZED`

## Next gate

C02c is `IMPLEMENTATION_STAGED` with its pre-build static corrective pass complete.

The next technical promotion gate is explicit authorization to run the staged implementation-validation scope. A separate later gate may design concrete terminal/forwarding backends.

Neither validation nor backend design authorizes production server-loop wiring. Production policy selection, runtime activation, deployment, and C03 remain separate and closed.
