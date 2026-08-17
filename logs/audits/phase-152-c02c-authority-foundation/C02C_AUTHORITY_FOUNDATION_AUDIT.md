# PRW Phase 152 C02c Authority Foundation Audit

Status: `SOURCE_SEAM_STAGED / STATIC_SCOPE_VERIFIED / BUILD_GATE_CLOSED`

- repository_id: `1334911207`
- canonical_repository: `powercode2026/prw-executor-private`
- frozen_predecessor: `01f5466504684ea6a2c504613901d24018485887`
- branch: `phase-152-c02c-authority-foundation`
- implementation_head_before_audit_refresh: `1474eb2a0ed1ff80974a75bb7fcc5085b6bbe364`

## Scope

C02c resumes Phase 152 implementation after successful real-host reconciliation. It locks and stages the smallest Agent-owned authority foundation needed before real provider adapters can be reviewed.

Changed files relative to the frozen predecessor at the implementation head:

- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02C_AUTHORITY_FOUNDATION_GATE.md`
- `crates/prw-agent/Cargo.toml`
- `crates/prw-agent/src/local_commands.rs`
- `crates/prw-agent/src/local_commands/management_authority.rs`
- `crates/prw-agent/src/local_commands/management_dispatch.rs`
- this audit file

Static compare result at implementation head plus audit:

- branch relation: `ahead 6 / behind 0`
- changed files: `6`
- source/design/audit additions before audit refresh: `529`
- source/design deletions before audit refresh: `16`
- root `Cargo.toml`: `UNCHANGED`
- root `Cargo.lock`: `UNCHANGED`
- `crates/prw-agent/src/main.rs`: `UNCHANGED`
- production runtime loop/bootstrap files: `UNCHANGED`
- production policy defaults: `UNCHANGED`
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

Its host-path constructor is crate-internal and named `open_trusted_root`; no public request-facing host-root API is introduced. File/transfer family evidence can only retain a reference to this Agent-owned anchored authority.

## Family authority seam

`LocalManagementFamilyAuthority` is crate-internal with private variants. Constructors require the corresponding real authority object:

- Agent: no external provider identity;
- File: `&LocalManagementFilesystemAuthority`;
- Transfer: `&LocalManagementFilesystemAuthority`;
- Terminal: `&LocalManagementRemoteSessionAuthority`;
- Forwarding: `&LocalManagementRemoteSessionAuthority`.

The value reports one exact `LocalManagementAuthorityFamily`; file/transfer values expose only their anchored filesystem authority and terminal/forwarding values expose only their registry/session authority.

## Request-bound context seam

`LocalManagementAuthorityContext::from_agent_owned_authority` is crate-internal and now requires:

- an already-admitted `LocalManagementAdmission`; and
- one `LocalManagementFamilyAuthority`.

Construction returns `None` on family mismatch. On success the context copies request ID, authenticated kernel peer PID/UID/GID, exact admitted capability, canonical operation code, and required family only from the admission token.

This constructor does not dispatch providers and does not make runtime management reachable. The existing processor still requires an explicitly supplied context and remains disconnected from the production server loop.

## Dependency delta and cycle inspection

`prw-agent` adds only existing workspace path dependencies needed by the authority seam:

- `prw-file-service`;
- `prw-forwarding`;
- `prw-registry`;
- `prw-session`;
- `prw-terminal`.

Static inspection of the frozen dependency manifests confirms no dependency returns to `prw-agent`:

- `prw-file-service` depends on `rustix` and `aws-lc-rs`;
- `prw-forwarding` depends on `prw-core` and `prw-registry`;
- `prw-terminal` depends on `prw-core`, `prw-registry`, and `prw-session`;
- `prw-registry` depends on `prw-connectivity`, `prw-control-plane`, `prw-core`, and `prw-session`.

No external crate version is added and no lockfile is modified in this staged source change.

## Validation classification

The build/test/clippy/format gate remains closed by project authorization. Therefore no Cargo command, formatter, linter, test, build, runtime execution, or deployment action is claimed for this branch.

Current validation is limited to connector-grounded source/API inspection, dependency-cycle inspection, and exact GitHub compare scope.

- source syntax/build validation: `NOT_RUN / GATE_CLOSED`
- runtime validation: `NOT_RUN`
- production activation: `NOT_AUTHORIZED`
- C03: `NOT_AUTHORIZED`

## Next implementation step

The next C02c source step is a provider-adapter seam that consumes the already-bound family authority without constructing provider resources inside request decoding. It must remain test-only/crate-internal and must not be connected to the production server loop, policy defaults, `main.rs`, service manager, or deployment.
