# PRW Phase 152 C02c Authority Foundation Audit

Status: `SOURCE_SEAM_STAGED / STATIC_SCOPE_VERIFIED / BUILD_GATE_CLOSED`

- repository_id: `1334911207`
- canonical_repository: `powercode2026/prw-executor-private`
- frozen_predecessor: `01f5466504684ea6a2c504613901d24018485887`
- branch: `phase-152-c02c-authority-foundation`
- implementation_head_before_audit_refresh: `e2344a34d0bc21e02935ee789f940cc3fc1028e8`

## Scope

C02c resumes Phase 152 implementation after successful real-host reconciliation. It now stages the Agent-owned authority, explicit management policy, provider lifecycle, and typed provider-dispatch seams required before any response encoding or production activation can be reviewed.

Changed files relative to the frozen predecessor before this audit refresh:

- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02C_AUTHORITY_FOUNDATION_GATE.md`
- `crates/prw-agent/Cargo.toml`
- `crates/prw-agent/src/local_commands.rs`
- `crates/prw-agent/src/local_commands/management_authority.rs`
- `crates/prw-agent/src/local_commands/management_dispatch.rs`
- `crates/prw-agent/src/local_commands/management_provider_lifecycle.rs`
- `crates/prw-agent/src/local_commands/management_typed_provider_dispatch.rs`
- `crates/prw-policy/src/lib.rs`
- this audit file

Static compare result immediately before this audit refresh:

- branch relation: `ahead 14 / behind 0`
- changed files: `9`
- additions: `1193`
- deletions: `17`
- root `Cargo.toml`: `UNCHANGED`
- root `Cargo.lock`: `UNCHANGED`
- `crates/prw-agent/src/main.rs`: `UNCHANGED`
- production runtime loop/bootstrap files: `UNCHANGED`
- existing production `BoundedLocalReadPolicy::allow_local_reads()`: `UNCHANGED`
- desktop/Android/deployment files: `UNCHANGED`

## Locked identity seam

`LocalManagementRemoteSessionAuthority` can be constructed only by passing an already-authenticated `AuthenticatedDeviceSession` through `WorkspaceDeviceRegistry::validate_authenticated_session`.

It retains:

- the returned `RegistryValidatedPrincipal`; and
- the `SessionId` copied from the same authenticated session.

It derives terminal/forwarding provider principals only through the existing provider APIs:

- `TerminalPrincipal::from_registry`;
- `ForwardingPrincipal::from_registry`.

No workspace/user/device/session identity is derived from local PID/UID/GID or request payload bytes.

## Locked filesystem seam

`LocalManagementFilesystemAuthority` owns an already-opened `prw_file_service::AnchoredFileRoot`.

Its host-path constructor is crate-internal and named `open_trusted_root`; no public request-facing host-root API is introduced. File/transfer family evidence can retain only a reference to this Agent-owned anchored authority.

The typed provider-dispatch seam additionally requires pointer identity between the supplied family authority and the lifecycle's exact `LocalManagementFilesystemAuthority`. A file/transfer authority for one trusted root cannot authorize operations through another lifecycle root merely because the family enum matches.

## Family authority and request-bound context

`LocalManagementFamilyAuthority` is crate-internal with private variants. Constructors require the corresponding real authority object:

- Agent: no external provider identity;
- File: `&LocalManagementFilesystemAuthority`;
- Transfer: `&LocalManagementFilesystemAuthority`;
- Terminal: `&LocalManagementRemoteSessionAuthority`;
- Forwarding: `&LocalManagementRemoteSessionAuthority`.

`LocalManagementAuthorityContext::from_agent_owned_authority` remains crate-internal and requires an already-admitted `LocalManagementAdmission` plus one family authority. Construction returns `None` on family mismatch. On success the context copies request ID, authenticated kernel peer PID/UID/GID, exact admitted capability, canonical operation code, and required family only from the admission token.

This constructor does not dispatch providers and does not make runtime management reachable.

## Explicit management policy seam

`prw-policy` now stages `BoundedLocalManagementPolicy` as a separate evaluator rather than widening `BoundedLocalReadPolicy`.

The management policy carries independent decisions only for capabilities represented by the current canonical bridge surface:

- `AgentStatusRead`;
- `FilesRead`;
- `FilesWrite`;
- `TerminalOpen`;
- `TerminalExec`;
- `ForwardingCreate`.

`PrivateDnsConfigRead` remains independently represented for the existing local read surface. Capabilities with no authorized C02c management operation remain fail-closed, including `FilesDelete`, `DeviceManage`, and `PolicyManage`.

No `allow_all()` constructor is introduced. Production bootstrap policy selection is unchanged.

## Provider lifecycle seam

`LocalManagementProviderLifecycle` composes:

- a borrowed exact `LocalManagementFilesystemAuthority`;
- `UploadTransferManager` borrowing the authority's `AnchoredFileRoot`;
- `TerminalBroker<T>` around a caller-supplied typed terminal backend;
- `PortForwardBroker<F>` around a caller-supplied typed forwarding backend.

This avoids self-referential ownership: the filesystem authority outlives the transfer manager by construction.

The lifecycle deliberately has no `Drop` implementation claiming cleanup. `try_finish(self)` returns `Ok(())` only when:

- active transfer count is zero;
- terminal broker is empty; and
- forwarding broker is empty.

If any provider state remains active, `try_finish` returns the entire lifecycle owner unchanged so typed cleanup can continue. C02c therefore does not fabricate clean rollback evidence by silently dropping active broker state.

## Typed provider dispatch seam

`dispatch_admitted_management_command` consumes only:

- an already-admitted canonical management request;
- one real `LocalManagementFamilyAuthority`;
- an already-assembled `LocalManagementProviderLifecycle`; and
- the existing bounded Agent status snapshot.

It returns `LocalManagementTypedProviderResult`, not local response bytes. Response encoding remains a later reviewed gate.

The typed dispatcher reuses the already-decoded `BridgeCommand` domain values and existing provider APIs for:

- descriptor-anchored file list/stat/create/directory-create;
- create-only upload begin/resume/chunk/finalize/abort;
- bounded download chunks;
- terminal open/input/resize/read/close;
- forwarding open/close; and
- bounded Agent status.

It does not parse raw shell text, executable paths, host filesystem roots, DNS names, arbitrary bind addresses, privilege instructions, or provider configuration from request bytes.

### Principal-binding protection

Terminal and forwarding broker IDs are broker-scoped identifiers, not principals. Therefore C02c explicitly checks principal ownership before using an existing broker record.

For terminal input/resize/read/close and forwarding close, the dispatcher derives the current provider principal only from the registry-revalidated PRW-session authority and compares it with the immutable principal already stored in the broker record. Principal mismatch fails before provider mutation.

For terminal/forwarding open, if the requested broker ID already exists under another principal, the same fail-closed principal mismatch is applied before the provider open path.

This prevents a valid authority for one registry/session principal from reusing another principal's known terminal or forwarding broker ID.

## Dependency delta and cycle inspection

`prw-agent` adds only existing workspace path dependencies needed by the authority/lifecycle seams:

- `prw-file-service`;
- `prw-file-transfer`;
- `prw-forwarding`;
- `prw-registry`;
- `prw-session`;
- `prw-terminal`.

Static inspection of frozen dependency manifests confirms no dependency returns to `prw-agent`:

- `prw-file-service` depends on `rustix` and `aws-lc-rs`;
- `prw-file-transfer` depends only on `prw-file-service`;
- `prw-forwarding` depends on `prw-core` and `prw-registry`;
- `prw-terminal` depends on `prw-core`, `prw-registry`, and `prw-session`;
- `prw-registry` depends on `prw-connectivity`, `prw-control-plane`, `prw-core`, and `prw-session`.

No external crate version is added and no lockfile is modified in this staged source change.

## Validation classification

The build/test/clippy/format gate remains closed by project authorization. Therefore no Cargo command, formatter, linter, test, build, runtime execution, or deployment action is claimed for this branch.

Current validation is limited to connector-grounded source/API inspection, exact dependency-manifest inspection, security-boundary inspection, and exact GitHub compare scope.

- source syntax/build validation: `NOT_RUN / GATE_CLOSED`
- formatter/linter/tests: `NOT_RUN / GATE_CLOSED`
- runtime validation: `NOT_RUN`
- production activation: `NOT_AUTHORIZED`
- runtime signer/systemd credential loading: `NOT_AUTHORIZED`
- deployment/privileged changes: `NOT_AUTHORIZED`
- C03: `NOT_AUTHORIZED`

## Next reviewed step

The remaining C02c blocker is response semantics: define a bounded, deterministic mapping from `LocalManagementTypedProviderResult` and typed provider failures into correlated local response payloads without exposing implementation-sensitive detail.

That next step must remain disconnected from the production server loop and must not change production policy defaults, `main.rs`, service-manager behavior, deployment, or C03 activation.
