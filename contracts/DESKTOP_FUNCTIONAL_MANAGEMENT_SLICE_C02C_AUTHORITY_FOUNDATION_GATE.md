# Phase 152 Slice C02c — Agent-Owned Management Authority Foundation Gate

Status: `IMPLEMENTATION_LOCK / NO_RUNTIME_WIRING / NO_PRODUCTION_ACTIVATION`

Frozen predecessor candidate: `01f5466504684ea6a2c504613901d24018485887` (`phase-152-desktop-functional-management`).

## Purpose

C02a proved authenticated admission, exact authority-context matching, provider-neutral dispatch ordering, and success acknowledgement only after provider success. C02b then locked the rule that real authority must not be fabricated from same-UID local IPC credentials or request bytes.

C02c now implements the smallest crate-internal Agent-owned authority, management-policy, provider-lifecycle, typed-dispatch, and deterministic-response seams needed for controlled implementation validation while keeping production management unreachable by default.

## Locked identity model

### Local caller identity

`AuthenticatedLocalLinuxConnection` and its kernel `SO_PEERCRED` tuple remain the authenticated local transport/caller identity used by C01 admission. The tuple (`pid`, `uid`, `gid`) may bind one local request to an Agent-owned authority grant, but it is not a PRW workspace principal.

### Terminal and forwarding identity

Terminal and forwarding operations require an independently authenticated PRW session identity. The only accepted provider-principal construction path is:

1. an `AuthenticatedDeviceSession` already exists outside local request bytes;
2. `WorkspaceDeviceRegistry::validate_authenticated_session` revalidates it against current active membership and enrolled-device state;
3. the result is a `RegistryValidatedPrincipal`;
4. the same authenticated PRW `SessionId` is paired with that registry-current principal;
5. `TerminalPrincipal::from_registry` or `ForwardingPrincipal::from_registry` creates the provider principal.

No constructor may derive workspace, user, device, public identity, role, or PRW `SessionId` from local UID/GID/PID, request payload fields, desktop process identity, or ambient process state.

A terminal/forwarding management request therefore requires both identities at once:

- authenticated local peer credentials for the local IPC request; and
- an Agent-owned registry-revalidated PRW session principal for provider authority.

Missing, stale, revoked, suspended, mismatched, or absent remote-session identity fails closed before provider invocation.

### Existing broker-record ownership

Terminal and forwarding broker identifiers are not principals. For every operation against an already-existing terminal/forwarding record, C02c must derive the current provider principal from registry-revalidated PRW-session authority and compare it with the immutable principal stored in that broker record before provider mutation.

A known broker identifier therefore cannot be reused across registry/session principals merely because the caller has a valid management capability.

## Locked filesystem authority model

`prw_file_service::AnchoredFileRoot` is the only filesystem-root authority admitted by this gate.

The Agent must obtain the host root path only from trusted Agent-owned configuration/bootstrap state outside request decoding. It then opens that path into an `AnchoredFileRoot` under the existing descriptor/no-follow semantics.

After construction:

- management request bytes may carry only validated `RemotePath` values relative to the root;
- request bytes must never supply or replace the host root path;
- desktop/local caller input must never be promoted into an ambient host path;
- file and transfer adapters receive an already-existing anchored root authority;
- `UploadTransferManager` borrows that same `&AnchoredFileRoot`; it does not create or widen filesystem authority.

C02c does not choose a production host pathname. That value belongs to a later deployment/configuration review and remains absent from the production bootstrap in this gate.

### Exact root identity

A matching provider family enum is insufficient by itself. File/transfer dispatch must prove that the family authority references the exact same `LocalManagementFilesystemAuthority` used by the provider lifecycle. C02c uses reference identity for this check before file or transfer mutation.

An authority for trusted root A cannot authorize operations through lifecycle root B.

## Implemented authority ownership model

`LocalManagementRemoteSessionAuthority` retains:

- one registry-current `RegistryValidatedPrincipal`; and
- the exact authenticated PRW `SessionId` from the same authenticated session.

`LocalManagementFilesystemAuthority` owns one already-opened `AnchoredFileRoot`.

`LocalManagementFamilyAuthority` has private family variants. Its constructors require the corresponding real authority object:

- Agent: no external provider identity;
- File: `&LocalManagementFilesystemAuthority`;
- Transfer: `&LocalManagementFilesystemAuthority`;
- Terminal: `&LocalManagementRemoteSessionAuthority`;
- Forwarding: `&LocalManagementRemoteSessionAuthority`.

No request handler may instantiate a broker, open a root, create a registry principal, or invent a session merely because a command was admitted by policy.

## Request-bound authority context

`LocalManagementAuthorityContext::from_agent_owned_authority` is crate-internal. It requires:

- an already-admitted `LocalManagementAdmission`; and
- one existing `LocalManagementFamilyAuthority`.

Construction fails on family mismatch and copies only from the admission token:

- admitted local request ID;
- authenticated local peer PID;
- authenticated local peer UID;
- authenticated local peer GID;
- exact capability derived from the decoded canonical command;
- canonical operation code;
- required provider family.

The context remains correlation evidence, not provider authority itself.

## Explicit management policy model

Provider authority and capability policy remain independent requirements.

C01 continues to derive the exact `Capability` from the canonical decoded `BridgeCommand`.

C02c introduces a separate `BoundedLocalManagementPolicy` rather than widening the existing production-local read policy. Management decisions are explicit per represented capability. No `allow_all()` constructor is introduced.

Capabilities outside the reviewed management surface remain fail-closed, including:

- `FilesDelete`;
- `DeviceManage`;
- `PolicyManage`.

The existing production bootstrap default `BoundedLocalReadPolicy::allow_local_reads()` remains unchanged. Introducing a management-policy type does not select or activate it in production.

## Implemented provider lifecycle model

`LocalManagementProviderLifecycle<'authority, T, F>` composes:

- `&LocalManagementFilesystemAuthority`;
- `UploadTransferManager<'authority>` borrowing the anchored root;
- `TerminalBroker<T>` around a caller-supplied typed terminal backend;
- `PortForwardBroker<F>` around a caller-supplied typed forwarding backend.

This structure avoids self-referential ownership: the filesystem authority is owned outside the lifecycle and outlives the transfer manager by construction.

### Explicit quiescence instead of fabricated cleanup

C02c deliberately does **not** add a `Drop` implementation that claims terminal close, forwarding close, or transfer abort happened when no provider-specific cleanup API has proved that outcome.

`try_finish(self)` reports clean completion only when all three conditions hold:

- active transfer count is zero;
- terminal broker is empty;
- forwarding broker is empty.

If any state remains active, the entire lifecycle owner is returned unchanged so the caller can continue explicit typed cleanup. Dropping an active lifecycle is not treated as cleanup evidence.

A later concrete backend/deployment gate may add bounded best-effort shutdown behavior only after its exact cleanup semantics are reviewed. C02c does not fabricate those semantics.

## Typed provider dispatch

`dispatch_admitted_management_command` receives only:

- an already-admitted canonical `LocalManagementAdmission`;
- one existing `LocalManagementFamilyAuthority`;
- one already-assembled `LocalManagementProviderLifecycle`; and
- an existing bounded Agent status snapshot.

It dispatches only already-decoded typed `BridgeCommand` domain values through existing APIs for:

- Agent status;
- descriptor-anchored file list/stat/create/directory-create;
- create-only upload begin/resume/chunk/finalize/abort;
- bounded download chunks;
- terminal open/input/resize/read/close;
- forwarding open/close.

It does not parse or accept raw shell text, executable paths, arbitrary host roots, DNS names, arbitrary bind addresses, environment bags, privilege instructions, firewall rules, routes, or provider configuration.

### Pre-mutation guards

Before provider mutation the typed dispatcher requires:

1. family correlation through `LocalManagementAuthorityContext::from_agent_owned_authority`;
2. exact filesystem-authority reference identity for file/transfer operations;
3. current registry/session principal equality for operations against existing terminal/forwarding broker records.

Failure of any guard produces no provider mutation and no success acknowledgement.

## Deterministic response semantics

The common local response protocol remains unchanged:

- the existing two-byte `LocalAgentResponseStatus` prefix;
- `Response` outer kind only for `Ok`;
- `Error` outer kind for every non-success status;
- the existing local request ID remains the correlation identifier.

C02c adds only a command-body encoding after successful typed dispatch. The first byte is a result tag:

- `1`: Agent status, followed by the existing five-byte Agent-status codec;
- `2`: directory listing, followed by `u16` entry count and repeated `u8 type + u16 name_len + UTF-8 name`;
- `3`: metadata, followed by `u8 type + u64 size`;
- `4`: empty acknowledgement;
- `5`: exact big-endian `u64` offset;
- `6`: bounded raw result bytes for download/terminal output.

Remote file-type codes are locked as:

- `1`: regular file;
- `2`: directory;
- `3`: symbolic link;
- `4`: other.

The body must remain within the existing local terminal-response body limit. If a provider succeeds but its result cannot be encoded within that bound, C02c emits correlated `InternalError` with an empty body; encoding failure never creates an `Ok` acknowledgement.

### Error disclosure boundary

Provider error strings and host details are never serialized. Typed failures collapse into the existing coarse statuses:

- malformed/out-of-bound operation semantics → `InvalidRequest`;
- stale/duplicate/missing resource state or authority/principal mismatch → `Conflict`;
- backend, filesystem, storage, postcondition, or success-encoding failure → `InternalError`.

Capability denial continues to map to `Unauthorized` before provider dispatch.

## Complete crate-internal execution seam

`process_authenticated_linux_management_with_typed_providers` composes the complete C02c sequence:

1. existing C01 authenticated local admission;
2. canonical command decode and exact capability policy evaluation;
3. required existing family authority;
4. typed provider dispatch with exact authority/principal guards;
5. deterministic correlated terminal-response construction.

The function is crate-internal and is not called by the production local server loop, Linux bootstrap, `main.rs`, service-manager integration, or deployment code in C02c.

This is implementation-validation plumbing, not production activation.

## Explicitly out of scope

C02c does not authorize or implement:

- production server-loop wiring;
- `main.rs` activation;
- systemd credential loading;
- runtime signing;
- production filesystem-root selection;
- real PTY/shell backend selection;
- real forwarding socket backend selection;
- firewall, routing, DNS, or privilege changes;
- production management-policy selection;
- deployment or service-manager changes;
- root Cargo workspace activation;
- C03 production activation.

## Implementation validation status

The source seams above are staged for implementation validation. Under the current project gates:

- source syntax/build validation: `NOT_RUN / BUILD_GATE_CLOSED`;
- formatter/linter/tests: `NOT_RUN / BUILD_GATE_CLOSED`;
- runtime validation: `NOT_RUN`;
- production activation: `NOT_AUTHORIZED`;
- C03: `NOT_AUTHORIZED`.

Static validation is limited to exact source/API inspection, dependency-manifest inspection, authority-boundary review, and GitHub diff scope.

## Next reviewed gate

The next reviewed gate may validate this C02c implementation under an explicitly opened build/test scope, or may design concrete terminal/forwarding backend implementations. Neither action may imply production runtime activation.

Production server-loop wiring and production policy selection remain separate from implementation validation, and C03 remains closed.
