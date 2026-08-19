# Phase 152 — Slice C03 Local Management Activation

Status: `AUTHORIZED_IMPLEMENTATION_BOUNDARY`

## Purpose

C03 authorizes the previously designed Phase 152 local-management path to become a real same-user desktop-to-Agent management path. This contract does not authorize an alternate protocol, an authorization bypass, fabricated remote identity, request-selected host authority, unrestricted shell execution, unrestricted network egress, or remote-network activation.

The required request order remains:

1. authenticated same-UID local Linux connection;
2. existing local IPC command `3` envelope;
3. preserved non-zero local request correlation identifier;
4. canonical `prw_remote_bridge::BridgeCommand` decode;
5. exact `BridgeCommand::required_capability()` derivation;
6. caller-bound explicit local-management `PolicyEvaluator` decision;
7. `LocalManagementAdmission` creation only on `Allow`;
8. exact Agent-owned provider-family authority selection;
9. typed provider dispatch through the existing C02c seam;
10. provider result encoding into the existing correlated local terminal response;
11. desktop presentation state changes only from that correlated Agent response.

## Local and remote principal separation

A same-UID local Linux peer is not a `RegistryValidatedPrincipal` and MUST NOT be converted into one.

Terminal and forwarding provider state MAY accept a separately typed `LocalSameUid` session principal derived only from an `AuthenticatedLocalLinuxConnection`. Existing registry-derived terminal/forwarding principal semantics remain distinct and unchanged for authenticated PRW remote sessions.

Principal equality is variant-sensitive. A local same-UID principal never compares equal to a registry-derived principal, and different local UIDs never compare equal.

## Policy activation

Production local management MUST use the existing explicit `BoundedLocalManagementPolicy`/`BoundedLocalManagementDecisions` surface or a stricter caller-bound evaluator. There is no `allow_all` authorization path.

The initial desktop management configuration MAY allow only currently represented bridge capabilities:

- `AgentStatusRead`;
- `PrivateDnsConfigRead`;
- `TerminalOpen`;
- `TerminalExec`;
- `FilesRead`;
- `FilesWrite`;
- `ForwardingCreate`.

`FilesDelete`, `DeviceManage`, and `PolicyManage` remain denied unless a later reviewed protocol surface and authority contract explicitly opens them.

## Filesystem authority

File and transfer providers require one Agent-selected `AnchoredFileRoot` opened before request decoding. The host root MUST NOT be selected or overridden by command payload data.

Production activation is fail-closed when the configured management root is absent, invalid, or cannot be opened. No implicit `$HOME`, current-working-directory, or root-filesystem fallback is authorized.

## Terminal provider boundary

Terminal open accepts only the existing named `TerminalProfile::{PosixShell,BashShell}` values and bounded `TerminalGeometry`. No arbitrary executable path, argv, raw command string, working directory, request-controlled environment, privilege instruction, or shell interpolation surface may be added by C03.

A concrete Linux adapter must map the named profile to provider-owned launch configuration and must report backend failure instead of silently pretending unsupported lifecycle operations succeeded.

## Forwarding provider boundary

Forwarding remains loopback-bind only and retains the typed explicit-IP/non-zero-port target surface. Production egress must pass an Agent-owned reviewed policy before connecting.

The existing C02d bounds remain authoritative: exact targets only; no hostname/CIDR/port-range/wildcard egress; maximum 32 connections; bounded connect and idle timeouts; explicit half-close behavior; explicit close ordering.

## Runtime activation

Commands `1` and `2` remain byte-for-byte compatible. Command `3` is additive.

The local Agent worker may route a validated frame to either the existing read-only response path or the C03 management path. Malformed input, correlation ambiguity, policy denial, missing/mismatched authority, provider failure, or response-write ambiguity fails closed and never creates a success acknowledgement.

Production management activation SHOULD be explicitly configured. A runtime lacking valid management configuration remains read-only rather than inventing authority.

## Desktop activation

The desktop may send an already-constructed command-3 frame through the existing trusted local Unix-socket endpoint only after endpoint trust checks. It must set bounded read/write timeouts, preserve the request ID, validate the terminal response, and reject mismatched correlation.

`Validated locally`, `sent`, `authorized`, `provider accepted`, and `completed` are separate presentation states. The UI must not display an authoritative success state from local intent construction alone.

## Dynamic reachability boundary

C03 does not change C02e identity semantics. `DeviceId` remains logical identity, `TransportIdentity` remains independently rotatable, and IP/port remains transient reachability data. No IP-as-identity fallback is authorized.

C03 also does not claim distributed live-owner fencing that C02e did not prove. A single-Agent/single-owner runtime may be validated without representing multi-owner safety as solved.

## Validation requirements

Before C03 is called complete, CI must prove at minimum:

- commands 1/2 compatibility;
- command-3 decode/correlation and malformed-request failure;
- same-UID local principal separation from registry principal;
- capability denial before provider mutation;
- family/authority mismatch before provider mutation;
- descriptor-root confinement;
- terminal named-profile bounds;
- forwarding exact-target/loopback/egress-policy bounds;
- correlated provider success/error response semantics;
- desktop request-ID mismatch failure;
- workspace rustfmt, Clippy, tests, and build.

A real Ubuntu desktop/Agent smoke test remains separately identifiable from repository CI evidence and must never be fabricated.