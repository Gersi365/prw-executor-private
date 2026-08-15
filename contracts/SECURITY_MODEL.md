# Private Remote Workspace Security Model

Version: `0.1.0`

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

## Private keys

Long-lived private device identity keys and transport private keys are generated and retained on the device whenever technically possible.

Control-plane services receive only the public material required for coordination and authorization.

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

## Revocation

Device revocation is a first-class security capability.

A revoked device must be denied future authorized connectivity after revocation state has propagated according to the final protocol semantics.
