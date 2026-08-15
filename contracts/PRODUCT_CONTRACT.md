# Private Remote Workspace Product Contract

Contract version: `0.1.0`

## Product identity

Private Remote Workspace is a standalone desktop/mobile remote-administration and private-mesh networking product.

It must not require Tailscale, Termius, or another separately installed VPN, SSH, or remote-file-management application as a runtime dependency.

Open-source audited protocols and libraries may be embedded as implementation building blocks.

## Initial platforms

### Ubuntu

Ubuntu is an initial host and client platform.

The host-side PRW Agent is:

- headless;
- persistent;
- intended for automatic startup;
- independent of the desktop UI lifecycle.

The desktop UI is a separate client/admin application.

### Android

Android is an initial mobile client platform and private-network endpoint.

## Remote connectivity

A powered-on Ubuntu host with Internet connectivity and a running PRW Agent must be designed for remote reachability from another network.

The connectivity architecture must prefer:

1. direct local connectivity when appropriate;
2. direct Internet peer-to-peer connectivity when possible;
3. encrypted relay fallback otherwise.

Manual router port forwarding, static public IP addresses, and external mesh VPN applications must not be baseline requirements.

## Remote administration

The product direction includes:

- terminal sessions;
- SSH-compatible functionality;
- remote file browsing;
- upload and download;
- copy;
- move;
- rename;
- directory creation;
- delete;
- resumable transfer;
- retry;
- partial-transfer recovery;
- integrity verification;
- atomic destination finalization;
- port forwarding.

## File-transfer safety

Cross-device move operations must not delete the source before destination integrity has been verified and finalized.

## Private DNS

Private DNS is optional.

The architecture must be capable of supporting:

- PRW device naming;
- custom DNS resolvers;
- split DNS.

Private DNS failure must not make basic device connectivity unrecoverable.

## Wake behavior

Wake-on-LAN and remote wake are outside the current product scope.

## Identity

The architecture treats these as separate concepts:

- user identity;
- device identity;
- network/transport identity.

Transport-key rotation must not require replacement of device identity.

## Authorization

The product must be architecture-ready for multiple users and workspaces.

Authorization must be capability-oriented rather than represented only by a single administrator flag.

Receiving devices remain enforcement points for remote operations.

## Security

Private cryptographic keys must not be uploaded to the control plane as part of normal operation.

The product must not invent new cryptographic primitives.

## Workspace and build model

Google Drive folder `Drive Workspace` is intended as a project workspace for source and build artifacts.

Ubuntu is the intended build/test execution environment.

Future synchronization between Drive and Ubuntu should use controlled rclone workflows rather than compiling directly on a cloud-mounted filesystem.
