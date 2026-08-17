# PRW Phase 152 C02c Authority Foundation Audit

Status: `SOURCE_SEAM_STAGED / STATIC_SCOPE_VERIFIED / BUILD_GATE_CLOSED`

- repository_id: `1334911207`
- canonical_repository: `powercode2026/prw-executor-private`
- frozen_predecessor: `01f5466504684ea6a2c504613901d24018485887`
- branch: `phase-152-c02c-authority-foundation`
- staged_head_before_audit: `ba3b7645807865dbeba08a4d3d1349a69b31b758`

## Scope

C02c resumes Phase 152 implementation after successful real-host reconciliation. It locks and stages the smallest Agent-owned authority foundation needed before real provider adapters can be reviewed.

Changed source/design files relative to the frozen predecessor:

- `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C02C_AUTHORITY_FOUNDATION_GATE.md`
- `crates/prw-agent/Cargo.toml`
- `crates/prw-agent/src/local_commands.rs`
- `crates/prw-agent/src/local_commands/management_authority.rs`

Static compare result before this audit file:

- branch relation: `ahead 4 / behind 0`
- changed files: `4`
- additions: `375`
- deletions: `0`
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

This source seam is deliberately not connected to `LocalManagementAuthorityContext` construction or provider dispatch yet. That preserves C02a fail-closed production behavior while making the real authority ingredients explicit and reviewable.

## Dependency delta

`prw-agent` adds only existing workspace path dependencies needed by the authority seam:

- `prw-file-service`;
- `prw-forwarding`;
- `prw-registry`;
- `prw-session`;
- `prw-terminal`.

No external crate version is added and no lockfile is modified in this staged source change.

## Validation classification

The build/test/clippy/format gate remains closed by project authorization. Therefore no Cargo command, formatter, linter, test, build, runtime execution, or deployment action is claimed for this branch.

Current validation is limited to connector-grounded source/API inspection and exact GitHub compare scope.

- source syntax/build validation: `NOT_RUN / GATE_CLOSED`
- runtime validation: `NOT_RUN`
- production activation: `NOT_AUTHORIZED`
- C03: `NOT_AUTHORIZED`

## Next implementation step

The next C02c source step is to make `LocalManagementAuthorityContext` constructible only from an admitted request plus one `LocalManagementFamilyAuthority`, preserving exact request/caller/capability/operation/family matching. Provider adapters and runtime wiring remain later gates.
