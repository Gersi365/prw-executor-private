# Private Remote Workspace Security Model

Version: `0.2.0`

## Trust boundaries

Private Remote Workspace separates:

- user/account identity;
- device identity;
- transport identity;
- control plane;
- data plane;
- relay infrastructure;
- local privileged operations;
- unprivileged application logic.

## Cryptography

Private Remote Workspace must not design proprietary cryptographic primitives.

Future implementations should use established and audited protocol/library building blocks for:

- encrypted transport;
- authentication;
- SSH;
- TLS;
- NAT traversal;
- relay transport.

Phase 002 does not select the concrete device-identity or transport cryptographic primitive.

## Private keys

Long-lived private device identity keys and transport private keys are generated and retained on the device whenever technically possible.

Control-plane contracts receive only the public material required for coordination and authorization.

Phase 002 models device public-identity material as opaque public bytes and has no type that carries a private identity key.

## Control plane

The control plane may coordinate metadata such as:

- users;
- workspaces;
- device public identities;
- transport public keys;
- assigned private addresses;
- permissions;
- device status;
- endpoint candidates;
- revocation state.

It must not be designed as the plaintext intermediary for terminal or file payloads.

Phase 002 locks only a transport-agnostic typed action boundary for enrollment decisions and device revocation. It does not select a network protocol, persistence layer, account-authentication mechanism, authorization credential, or deployment topology.

## Enrollment

Enrollment uses distinct WorkspaceId, UserId, DeviceId, and EnrollmentId values.

A pending enrollment may receive one terminal typed decision: approved or rejected.

The Phase 002 state model does not decide who is authorized to approve enrollment, how the approving actor authenticates, how trust bootstrap is transported, or how enrollment data is persisted.

Those decisions remain security-sensitive future work.

## Revocation

Device revocation is a first-class security capability.

A revoked device must be denied future authorized connectivity after revocation state has propagated according to the final protocol semantics.

Phase 002 deliberately does not define propagation timing, stale-device behavior, acknowledgement, retry, or persistence semantics.

## Relay

A relay is transport infrastructure.

Relay fallback must preserve end-to-end encryption between peers such that the relay is not granted application plaintext merely because traffic transits it.

## Agent

The PRW Agent is a local enforcement boundary.

The Agent is headless and independent of the graphical UI.

Future privileged networking or host-management operations should be isolated behind the narrowest practical privilege boundary instead of making the full Agent permanently privileged.

## File operations

File management must use typed operations and explicit authorization checks.

The file-management protocol must not rely on an unrestricted arbitrary-shell primitive as its normal filesystem API.

Representative operation classes:

- list;
- stat;
- read;
- write;
- copy;
- move;
- rename;
- mkdir;
- delete.

## Authorization

Authorization should support scoped capabilities such as:

- terminal.open;
- terminal.exec;
- files.read;
- files.write;
- files.delete;
- forwarding.create;
- device.manage;
- policy.manage.

Default file access should remain bounded by the effective local-user permissions unless an explicitly authorized privileged capability is introduced later.

Phase 002 does not introduce a single administrator flag or bypass receiving-device enforcement.
