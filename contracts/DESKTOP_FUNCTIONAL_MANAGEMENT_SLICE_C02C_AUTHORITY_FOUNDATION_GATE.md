# Phase 152 Slice C02c — Agent-Owned Management Authority Foundation Gate

Status: `DESIGN_LOCK / NO_RUNTIME_WIRING / NO_PRODUCTION_ACTIVATION`

Frozen predecessor candidate: `01f5466504684ea6a2c504613901d24018485887` (`phase-152-desktop-functional-management`).

## Purpose

C02a proved authenticated admission, exact authority-context matching, provider-neutral dispatch ordering, and success acknowledgement only after provider success. C02b then locked the rule that real authority must not be fabricated from same-UID local IPC credentials or request bytes.

C02c resolves the remaining authority-model blockers before any real provider adapter is introduced. It defines the smallest Agent-owned authority foundation that later implementation may assemble explicitly, while keeping production management unreachable by default.

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

## Locked filesystem authority model

`prw_file_service::AnchoredFileRoot` is the only filesystem-root authority admitted by this gate.

The Agent must obtain the host root path only from trusted Agent-owned configuration/bootstrap state outside request decoding. It then opens that path once into an `AnchoredFileRoot` under the existing descriptor/no-follow semantics.

After construction:

- management request bytes may carry only validated `RemotePath` values relative to the root;
- request bytes must never supply or replace the host root path;
- desktop/local caller input must never be promoted into an ambient host path;
- file and transfer adapters receive an already-existing anchored root authority;
- `UploadTransferManager` may borrow that same `&AnchoredFileRoot`; it does not create or widen filesystem authority.

C02c does not choose a production host pathname. That value belongs to a later deployment/configuration review and remains absent from the production bootstrap in this gate.

## Locked authority ownership model

A future Agent-owned management authority set must be assembled outside canonical request decoding and must own or borrow provider resources explicitly.

The conceptual ownership boundary is:

- Agent status snapshot: immutable Agent-owned state;
- file root: Agent-owned `AnchoredFileRoot`;
- transfer manager: Agent-owned manager borrowing the anchored root for no longer than the authority owner;
- terminal broker: Agent-owned `TerminalBroker<B>` around one reviewed typed backend;
- forwarding broker: Agent-owned `PortForwardBroker<B>` around one reviewed typed backend;
- remote principal registry/session evidence: Agent-owned authenticated-session state revalidated through `WorkspaceDeviceRegistry` before principal construction.

No request handler may instantiate a broker, open a root, create a registry principal, or invent a session merely because a command was admitted by policy.

## Locked lifecycle and cleanup model

Provider resources follow the existing PRW lifecycle pattern already used by `LocalLinuxProductionLifecycleExecution`:

1. assemble resources in a deterministic order;
2. retain ownership in one enclosing Agent-owned lifecycle object;
3. if a later assembly stage fails, roll back all earlier successfully created resources;
4. expose explicit cleanup result/evidence where cleanup may fail;
5. on unwind/drop, perform bounded best-effort cleanup without creating success acknowledgement;
6. never leave a half-created authority object reachable by request dispatch.

Provider-specific requirements:

- terminal sessions must be explicitly closed through the terminal broker before broker teardown when possible;
- forwarding sessions must be explicitly closed through the forwarding broker before broker teardown when possible;
- active staged uploads must be explicitly aborted/cleaned according to the transfer manager/storage contract before filesystem-root teardown when possible;
- the anchored root remains live for every transfer manager that borrows it and is dropped only after those borrowers are gone;
- cleanup failure must be recorded and must not be rewritten as operation success.

C02c does not authorize real PTY, socket-forwarding, arbitrary host networking, or deployment-specific cleanup implementations. It locks ownership semantics only.

## Locked policy model

Provider authority and capability policy are independent requirements.

C01 continues to derive the exact `Capability` from the canonical decoded `BridgeCommand`. A future production authority constructor/dispatcher must require both:

- policy decision `Allow` for that exact capability; and
- the exact family-specific Agent-owned authority described above.

Possessing an `AnchoredFileRoot`, registry principal, broker, backend, or authenticated session never implies policy permission.

The current production bootstrap default `BoundedLocalReadPolicy::allow_local_reads()` remains unchanged. C02c grants no terminal, forwarding, write, transfer, or other management capability in production.

The exact production policy configuration capable of granting management operations remains a separate reviewed gate.

## Exact authority-context binding

Any future production constructor for `LocalManagementAuthorityContext` must be non-public outside the Agent authority-assembly boundary and must bind at least:

- admitted local request ID;
- authenticated local peer PID;
- authenticated local peer UID;
- authenticated local peer GID;
- capability derived from the decoded canonical command;
- canonical operation code;
- required provider family.

For terminal/forwarding families, the enclosing authority must additionally retain the independently authenticated registry/session principal described above.

For file/transfer families, the enclosing authority must additionally retain the trusted Agent-owned anchored filesystem authority described above.

A context mismatch, missing family authority, stale local correlation, stale remote-session principal, or unavailable provider lifecycle resource fails closed before provider invocation.

## Assembly ordering

The reviewed implementation sequence after this design lock is:

1. construct trusted long-lived Agent-owned authority inputs outside request handling;
2. validate/revalidate remote-session principal state when terminal/forwarding authority is needed;
3. perform C01 local authenticated admission and exact capability policy evaluation;
4. construct the request-bound opaque authority context only from the admitted request plus already-existing family authority;
5. invoke one typed provider adapter;
6. construct success acknowledgement only after provider success;
7. preserve cleanup/rollback evidence independently from operation success.

No step may move provider construction or authority selection into canonical request decoding.

## Explicitly out of scope

C02c does not authorize or implement:

- runtime server-loop wiring;
- `main.rs` activation;
- systemd credential loading;
- runtime signing;
- production filesystem root selection;
- real PTY/shell backend selection;
- real forwarding socket backend selection;
- firewall, routing, DNS, or privilege changes;
- production management policy grants;
- deployment or service-manager changes;
- root Cargo workspace activation;
- C03 production activation.

## Implementation exit criteria

A subsequent C02c implementation branch may add the smallest source seam only when all of the following remain true:

- no fabricated `RegistryValidatedPrincipal` or provider principal path exists;
- no request-selected host filesystem root exists;
- no provider resource is constructed inside request decoding/admission;
- policy denial still guarantees zero provider calls;
- authority-context matching remains exact and fail-closed;
- resource ownership and cleanup order are explicit;
- production policy defaults remain unchanged;
- runtime wiring remains absent unless separately reviewed;
- C03 remains closed.

This design lock is the authority foundation for that implementation work; it is not production activation evidence.
