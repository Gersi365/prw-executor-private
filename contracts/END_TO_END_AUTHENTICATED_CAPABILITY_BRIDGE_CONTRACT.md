# Private Remote Workspace — End-to-End Authenticated Capability Bridge Contract

Status: Phase 143 implementation lock
Date: 2026-08-16
Repository: `Gersi365/prw-executor-private`
Input baseline: `a9b61bde6484f5fdd48a62822d6c28b81fc75c44`
Parent roadmap: `contracts/PRODUCTIZATION_ROADMAP_AND_MUTATION_GATES_CONTRACT.md`
Parent transport architecture: `contracts/REMOTE_TRANSPORT_ARCHITECTURE_DECISION.md`
Parent application session: `contracts/REMOTE_DEVICE_SESSION_AUTH_CONTRACT.md`
Parent current registry: `contracts/DEVICE_REGISTRY_WORKSPACE_MEMBERSHIP_CONTRACT.md`
Parent relay: `contracts/DISPOSABLE_RELAY_PROTOCOL_SERVICE_CONTRACT.md`

## Purpose

Phase 143 connects the already validated remote transport identity and PRWM control framing to existing PRW application-session, current-registry, policy and typed capability boundaries. It is source/disposable only.

A successful QUIC/TLS connection, ICE path, relay route or valid certificate never grants a PRW capability. Every protected remote request must pass the full chain:

1. PRWM request framing is valid and the outer message kind is `Request`;
2. the remote application-session lease is currently valid;
3. the Phase 128 authenticated device-session snapshot is revalidated against the current Phase 130 registry;
4. the presented Phase 140 `TransportIdentity` matches the current transport identity bound to that exact registered device;
5. the request payload decodes to one supported typed Phase 143 operation;
6. the exact required `prw-policy::Capability` evaluates to `Allow`;
7. only then may a typed dispatcher receive the authorized operation.

Failure at any gate is fail-closed and must occur before capability dispatch.

Phase 143 does not activate a production QUIC listener, NAT traversal, relay, Agent replacement/restart, firewall/NAT/router/TUN/TAP/route/DNS mutation, production credentials or client distribution.

## Current transport identity in registry

Phase 139 already locked the requirement that the device registry bind each current `DeviceId` to its expected `TransportIdentity` and that transport-key rotation atomically replace the current identity while the old identity ceases to authorize new connections.

Phase 130 predates that architecture decision, so its source model currently stores only the immutable device identity tuple. Phase 143 therefore makes the minimal source-model extension inside `prw-registry`:

- `RegisteredDevice` retains its immutable Phase 130 `DeviceIdentityBinding` and adds separate optional current `TransportIdentity` metadata;
- first transport binding is allowed only for a currently enrolled registered device;
- a second initial bind is rejected;
- rotation is compare-and-replace: caller supplies the exact expected current identity and a distinct replacement;
- missing, stale, mismatched or unchanged transport identity fails closed;
- device revocation remains authoritative and invalidates transport validation even if a matching identity is still stored;
- no transport private key, certificate bytes or CA material are stored in the registry.

This is an in-memory/source realization of the already locked Phase 139 registry binding. It is not a database migration or production credential change.

## Remote application-session lease

Phase 128 authenticates one device-session proof and produces an immutable `AuthenticatedDeviceSession`, but the completed source object intentionally has no long-lived refresh/expiry policy.

Phase 143 wraps that authenticated session in a separate `RemoteSessionLease` used only by the remote capability bridge.

Initial lease rules:

- verifier-owned `issued_at` and `expires_at` are Unix seconds;
- `expires_at` must be strictly greater than `issued_at`;
- maximum lease lifetime is 3,600 seconds;
- verifier time before issue is rejected;
- verifier time at or after expiry is rejected;
- the wrapped Phase 128 authenticated identity is unchanged;
- there is no refresh token, bearer token or implicit reconnect ticket in this phase.

Current registry validation is still performed on every protected operation, so an unexpired lease cannot override membership suspension/removal or device revocation.

## PRWM capability request envelope

Phase 143 uses the Phase 140 `ControlFrame` as the outer transport envelope and introduces one bounded request payload format. The outer `ControlFrame` must have `ControlMessageKind::Request` and a non-zero request identifier.

The Phase 143 request payload begins with a fixed 12-byte header in network byte order:

1. magic — 4 bytes: `PRWC`;
2. protocol major — `u16`, value `1`;
3. protocol minor — `u16`, value `0`;
4. operation code — `u16`;
5. flags — `u16`, value `0`.

The remainder is the operation body. The total payload remains within the Phase 140 65,536-byte PRWM ceiling.

Unsupported version/operation, non-zero flags, malformed field length, invalid UTF-8, invalid typed path/identifier/geometry/endpoint, truncation, trailing data or an operation-specific bound violation fails closed.

## Initial operation registry and exact capabilities

The bridge exposes only operations backed by existing typed source boundaries. It does not expose a generic command string, executable path, shell argv, raw filesystem authority, hostname resolver, arbitrary bind address or unrestricted network destination.

Initial operation registry:

- `1` — Agent status read → `AgentStatusRead`;
- `2` — file list → `FilesRead`;
- `3` — file stat → `FilesRead`;
- `4` — create file → `FilesWrite`;
- `5` — create directory → `FilesWrite`;
- `6` — begin upload → `FilesWrite`;
- `7` — resume upload → `FilesWrite`;
- `8` — upload chunk → `FilesWrite`;
- `9` — finalize upload → `FilesWrite`;
- `10` — abort upload → `FilesWrite`;
- `11` — download chunk → `FilesRead`;
- `12` — terminal open → `TerminalOpen`;
- `13` — terminal input → `TerminalExec`;
- `14` — terminal resize → `TerminalExec`;
- `15` — terminal read → `TerminalExec`;
- `16` — terminal close → `TerminalExec`;
- `17` — port-forward open → `ForwardingCreate`;
- `18` — port-forward close → `ForwardingCreate`.

No role automatically grants any capability. `WorkspaceRole` remains registry metadata only.

`FilesDelete`, `DeviceManage`, `PolicyManage` and private-DNS mutation are not remotely exposed by this initial bridge. File deletion is intentionally absent because the current Phase 131 file-service source does not implement it.

## Typed operation bodies

Phase 143 decodes directly into the already validated domain types wherever they exist:

- `prw_file_service::RemotePath`;
- `prw_file_transfer::{TransferId, UploadPlan}`;
- `prw_terminal::{TerminalSessionId, TerminalProfile, TerminalGeometry}`;
- `prw_forwarding::{PortForwardId, LoopbackBind, ForwardTarget, TcpForwardSpec}`;
- `prw_agent::local_commands::LocalAgentCommand::GetAgentStatus`.

The bridge does not weaken those constructors or duplicate their security policy.

All variable byte strings are length-prefixed and checked before allocation/copy. One Phase 143 inline data chunk is capped at 60,000 bytes so its metadata always fits within the Phase 140 PRWM 65,536-byte ceiling. This is intentionally narrower than the underlying 1 MiB file-transfer storage chunk and 65,536-byte terminal chunk bounds; clients may split larger data into multiple authorized requests.

Forwarding remains loopback-bind plus explicit IP target only. No DNS name is accepted.

## Authorization and dispatch

`CapabilityBridge::authorize` returns an `AuthorizedCapabilityRequest` only after every gate succeeds. The authorized object contains:

- outer PRWM request identifier;
- current `RegistryValidatedPrincipal` snapshot;
- exact presented/current `TransportIdentity`;
- typed `BridgeCommand`;
- exact required `Capability`.

A `CapabilityDispatcher` receives only `AuthorizedCapabilityRequest`. `process_request` performs authorize-then-dispatch and wraps a bounded dispatcher response in a Phase 140 `ControlFrame::Response` using the same request identifier.

Dispatcher errors are mapped to a bounded bridge error and do not bypass policy. Dispatcher response payload may not exceed the Phase 140 control payload ceiling.

The bridge crate itself does not own a filesystem root, PTY/shell process, port-forward socket, QUIC endpoint or production Agent runtime. Existing capability providers remain the owners of those effects.

## Required disposable validation

Phase 143 must prove at minimum:

1. current transport identity first-bind, compare-and-rotate and mismatch/revocation behavior in `prw-registry`;
2. valid Phase 128 device proof produces an authenticated session that can be wrapped only in a bounded valid remote lease;
3. expired and not-yet-valid leases fail before policy/dispatch;
4. current membership suspension/removal and device revocation invalidate an otherwise unexpired remote lease;
5. stale/wrong presented transport identity fails before policy/dispatch;
6. valid PRWM request + current session + current registry + current transport binding + allowed capability yields an authorized typed request;
7. denied capability never reaches the dispatcher;
8. successful transport/relay state alone cannot produce an authorized request;
9. wrong outer `ControlMessageKind` fails closed;
10. all 18 operation codes round-trip through the bounded PRWC codec into existing typed domain values;
11. malformed magic/version/flags/opcode/length/path/identifier/geometry/IP/truncation/trailing bytes fail closed;
12. exact operation-to-capability mapping is tested and contains no role-to-capability inference;
13. dispatcher receives no call on authentication, registry, transport, codec or policy failure;
14. successful dispatch produces a correlated Phase 140 `Response` frame and oversized response fails closed;
15. runtime source introduces no new socket, DNS resolver, process/shell execution, TUN/TAP, firewall or route mutation surface;
16. focused rustfmt, Clippy `-D warnings`, tests and build pass;
17. full locked workspace rustfmt, Clippy, tests and build pass;
18. no production state is changed.

## Phase 144 boundary

Phase 143 completion authorizes no Android framework or toolchain selection. Phase 144 is a separate architecture decision and, per the Phase 138 roadmap, requires explicit architecture approval before a framework/toolchain lock is written.

## Production boundary

Until the exact Phase 154 transaction is separately approved, Phase 143 MUST NOT:

- publish or bind a production remote data-plane listener;
- activate production STUN/ICE/TURN or relay traffic;
- alter firewall/NAT/router/TUN/TAP/routes/DNS;
- provision production transport credentials;
- replace/restart the production Agent for the new remote data plane;
- sign/distribute a production Android/Desktop client.

## Completion classification

Target final state:

`PHASE_143_DONE / E2E_AUTHENTICATED_CAPABILITY_BRIDGE_VALIDATED / CURRENT_REGISTRY_AND_TRANSPORT_REVALIDATION / PER_CAPABILITY_POLICY_ENFORCED / NO_PRODUCTION_SIDE_EFFECT / READY_FOR_EXPLICIT_PHASE_144_ARCHITECTURE_DECISION`
