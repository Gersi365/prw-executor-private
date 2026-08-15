# Private Remote Workspace Security Model

Version: `0.3.0`

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

The initial device-identity signature primitive is ECDSA over NIST P-256 with SHA-256.

This selection applies only to device identity. It does not select the future private-network transport primitive or authorize reuse of one long-lived key across identity and transport roles.

Future implementations must use an appropriately reviewed cryptographic provider or library. Phase 003 intentionally does not bind the product to a specific Rust cryptography crate or operating-system provider.

## Device identity private keys

Long-lived private device identity keys are generated and retained on the device whenever technically possible.

Control-plane contracts receive only public device-identity material.

Phase 003 does not define:

- the private-key storage backend;
- hardware-backed key requirements;
- key attestation;
- backup or recovery;
- key rotation;
- public-key wire encoding;
- signature wire encoding.

The Android implementation should preserve compatibility with Android Keystore-backed EC keys where the final platform policy permits it, without forcing the Ubuntu implementation to use the same storage backend.

## Transport identity

Transport identity remains separate from device identity.

Phase 003 does not select:

- WireGuard or another concrete encrypted transport implementation;
- transport key primitive;
- transport key rotation;
- private-address allocation;
- TUN integration.

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

The control-plane identity boundary carries an explicit device-identity algorithm identifier together with opaque public bytes. It carries no private key.

## Enrollment

Enrollment uses distinct WorkspaceId, UserId, DeviceId, and EnrollmentId values.

A pending enrollment may receive one terminal typed decision: approved or rejected.

The state model does not decide who is authorized to approve enrollment, how the approving actor authenticates, how trust bootstrap is transported, or how enrollment data is persisted.

Those decisions remain security-sensitive future work.

## Revocation

Device revocation is a first-class security capability.

A revoked device must be denied future authorized connectivity after revocation state has propagated according to the final protocol semantics.

Phase 003 deliberately does not define propagation timing, stale-device behavior, acknowledgement, retry, or persistence semantics.

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

Phase 003 does not introduce a single administrator flag or bypass receiving-device enforcement.
