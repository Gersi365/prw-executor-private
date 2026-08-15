# Private Remote Workspace

Private Remote Workspace is an independent remote-administration and private-mesh networking product.

The product is designed so users do not need Tailscale, Termius, or another external VPN/SSH/file-management application installed as a runtime prerequisite.

## Initial target platforms

- Ubuntu host/client
- Android mobile client

## Core product capabilities

Planned product capabilities include:

- headless PRW Agent with automatic startup;
- remote terminal / SSH functionality;
- bidirectional file management;
- resumable and integrity-verified transfers;
- port forwarding;
- device enrollment;
- device and user permissions;
- encrypted peer-to-peer private networking;
- NAT traversal;
- relay fallback;
- private device addressing;
- optional private DNS;
- device management;
- device revocation.

## Architecture principles

1. No new cryptographic primitives.
2. Use audited standard protocols and libraries as building blocks.
3. Separate control plane from encrypted data plane.
4. Separate user identity, device identity, and network identity.
5. Prefer direct peer-to-peer connectivity.
6. Use encrypted relay fallback only when direct connectivity is unavailable.
7. Keep the PRW Agent headless and independent from the desktop UI lifecycle.
8. Do not require Wake-on-LAN.
9. Keep private DNS optional.
10. Keep privileged operations narrowly scoped.

## Phase 001

Phase 001 establishes only a compile-oriented architecture baseline and typed domain contracts.

It does not enable production networking, SSH, DNS, relay, authentication, enrollment, deployment, or privileged system changes.
