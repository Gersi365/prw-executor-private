# PRW Desktop Application

The desktop application is a separate UI/client process.

It must not be required for the Ubuntu host to remain remotely reachable.

Planned responsibilities:

- device list;
- terminal UI;
- remote file browser;
- transfer UI;
- port-forward management;
- private-network status;
- private-DNS settings;
- enrollment and device-management UI.

The desktop UI should communicate with the local headless PRW Agent through authenticated local IPC.

No desktop application implementation is included in Phase 001.
