# Private Remote Workspace — Desktop Client Architecture Decision

Status: Phase 150 architecture lock
Date: 2026-08-17
Repository: `Gersi365/prw-executor-private`
Input baseline: `63a036af6e2ec8fe1d7c43df51335c1a0ad12764`
Parent roadmap: `contracts/PRODUCTIZATION_ROADMAP_AND_MUTATION_GATES_CONTRACT.md`

## Approval

The user explicitly approved the Phase 150 recommendation on 2026-08-17 and authorized continued work without requiring intervention at each normal non-production step.

This approval locks the desktop architecture below. It does not authorize production remote-network activation, production credential provisioning, production Agent replacement/restart, firewall/NAT/router/TUN/TAP/route/DNS mutation, distribution signing, package publication, or any Phase 154 production transaction.

## Product role

The PRW desktop application is a native Ubuntu/Linux client and local administration surface.

It is a separate process from the headless PRW Agent and must not be required for the host to remain remotely reachable.

The desktop client owns presentation and user interaction. The Agent remains authoritative for host runtime state, policy enforcement, local privileged boundaries, remote capability execution, and production lifecycle ownership.

## Framework and language lock

The desktop application is implemented in Rust using:

- Rust toolchain `1.97.1`, matching the repository toolchain lock;
- GTK 4 through `gtk4` crate `0.11.3`;
- libadwaita through `libadwaita` crate `0.9.1`;
- GTK API feature floor `v4_14`;
- libadwaita API feature floor `v1_5`;
- Ubuntu 24.04 LTS as the minimum supported desktop compatibility baseline for the first implementation slice.

The dependency versions above are a Phase 150 architecture lock. Phase 151 must materialize them through normal Cargo dependency resolution and preserve the resulting `Cargo.lock` changes for validation.

No Electron, embedded browser application shell, Tauri, Slint, or second desktop UI framework is authorized by this phase.

## Rationale

GTK 4 and libadwaita are selected because they provide a native Ubuntu/GNOME application model, standard accessibility integration, adaptive application primitives, native input/focus semantics, and a small conceptual gap between the PRW Rust workspace and the desktop UI process.

The selected bindings are compatible with the repository Rust 1.97.1 baseline. Ubuntu 24.04 provides GTK 4.14 and libadwaita 1.5, so the feature floors remain compatible with that baseline while allowing newer supported Ubuntu systems to use newer runtime libraries.

## Desktop process boundary

The desktop UI is an unprivileged user process.

It must not:

- run as root merely to administer PRW;
- own the production Agent lifecycle;
- replace Agent policy evaluation;
- read Agent private key material;
- perform direct privileged network mutation;
- expose an unrestricted shell bridge to the Agent;
- bypass typed PRW request/response contracts.

The desktop process may request only explicitly supported typed Agent operations and must represent Agent acknowledgements separately from user intent.

## Local IPC ownership

The desktop client communicates with the local headless Agent through the existing authenticated Unix-domain IPC boundary.

The authoritative endpoint is the fixed filesystem socket:

`$XDG_RUNTIME_DIR/private-remote-workspace/agent.sock`

The existing Linux Agent contracts remain authoritative for:

- validated XDG runtime-root ownership and type;
- fixed PRW runtime-directory basename `private-remote-workspace`;
- PRW runtime-directory mode `0700`;
- fixed socket basename `agent.sock`;
- socket mode `0600`;
- same-effective-UID peer authorization using kernel `SO_PEERCRED` before application-protocol reads;
- authenticated local connection/session wrappers;
- typed local request/response framing;
- Agent-side policy and status authority.

There is no TCP, abstract Unix-socket, `/tmp`, D-Bus, shell-command, or ambient-path fallback for the desktop-to-Agent control path.

## Desktop-side IPC safety rules

Phase 151 must introduce a narrow desktop IPC adapter rather than embedding protocol mechanics throughout UI widgets.

The adapter must:

1. derive the expected socket path only from a validated local runtime context;
2. connect only to the fixed PRW socket endpoint;
3. fail closed on missing, malformed, wrong-type, wrong-owner, or otherwise untrusted endpoint state;
4. use bounded request identifiers and the existing local Agent frame/response codecs where reusable;
5. preserve explicit request/response correlation;
6. separate transport failure, protocol failure, authorization failure, unsupported command, conflict, and Agent internal error in presentation state;
7. never convert arbitrary UI text into executable Agent commands;
8. never treat a UI status label as authorization.

Agent-side `SO_PEERCRED` authorization remains mandatory and cannot be replaced by desktop-side checks.

## State model

The desktop application maintains immutable/presentation-oriented state derived from Agent responses and local UI intent.

Initial top-level UI state includes:

- Agent availability: `Unknown`, `Offline`, `Connecting`, `Online`, `Error`;
- selected PRW device/workspace context;
- authenticated local IPC state;
- remote connectivity-path presentation;
- active terminal/session count;
- active transfer count and progress summaries;
- forwarding summary;
- optional private-DNS status;
- bounded user-visible errors.

Presentation state does not grant capabilities and must not manufacture success before an authoritative acknowledgement is received.

## Initial desktop information architecture

The Phase 151–152 desktop shell should use a restrained native layout with a stable navigation surface and one primary content area.

Planned top-level destinations:

- Overview;
- Machines;
- Sessions;
- Files;
- Transfers;
- Activity;
- Settings.

The UI may use a dark-first PRW visual treatment through supported GTK/libadwaita theming and semantic status styling, but must preserve platform accessibility, focus, contrast, text scaling, keyboard navigation, and native widget behavior.

Visual styling must not duplicate security-sensitive state. Connectivity, encryption, authorization, and success/failure labels are projections of authoritative state only.

## Phase 151 bounded implementation scope

Phase 151 is authorized to implement a non-production desktop foundation containing:

- the Rust GTK/libadwaita desktop crate/application shell;
- deterministic application identity and startup;
- native window/navigation structure;
- typed presentation state;
- a narrow local IPC client adapter for read-only Agent status surfaces;
- `GetAgentStatus` and `GetPrivateDnsConfig` presentation using existing Agent command contracts;
- explicit offline/unavailable/error states;
- focused unit/integration tests that do not require production remote-network activation;
- CI/build validation for the desktop crate where the required Linux system libraries are available.

Phase 151 must not silently widen local Agent commands beyond the existing read-only command baseline merely to populate the UI.

If additional Agent commands are required for later Phase 152 capabilities, each must be introduced through a separate typed contract and validated before the desktop UI can invoke it.

## Phase 152 bounded implementation scope

Phase 152 may build the desktop terminal, files/transfers, forwarding, network, enrollment/device-management, and optional-DNS management surfaces only by reusing existing PRW authorities and explicit typed local IPC extensions.

Phase 152 is not authorization for arbitrary local shell execution, direct filesystem authority outside Agent policy, privileged networking, production remote-network activation, or remote-desktop screen streaming.

## Remote desktop boundary

The current productization roadmap explicitly keeps RustDesk-like remote desktop outside the locked Phase 150–154 scope.

Screen capture, video encoding/streaming, remote input injection, clipboard synchronization, multi-monitor handling, and their permission/security model require a separate explicit roadmap decision before implementation.

A visual placeholder or future navigation slot must not claim that this capability exists.

## Packaging and distribution boundary

Phase 150 does not select or publish a final Linux distribution package format.

Phase 151 may build and test the desktop executable in CI. Packaging into `.deb`, Flatpak, AppImage, repository packages, signing, auto-update, desktop-store publication, or production installation remains a later explicit packaging decision.

No application installation or system-wide dependency mutation on the user's Ubuntu host is authorized by this decision.

## Validation requirements for Phase 151

At minimum, the implementation must preserve:

- `cargo fmt --all -- --check`;
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` where the locked system dependencies are present;
- `cargo test --workspace --all-targets`;
- `cargo build --workspace --all-targets`;
- focused desktop tests for state projection and IPC error handling;
- no production networking or service mutation during validation.

Environment/tooling failures caused by unavailable GTK/libadwaita system packages must be classified separately from Rust source defects.

## Official source checks at decision date

The architecture decision was checked on 2026-08-17 against the official GTK-rs/libadwaita-rs documentation and Ubuntu package metadata:

- stable `gtk4` Rust bindings: `0.11.3`;
- GTK-rs minimum supported Rust version: `1.83`;
- stable `libadwaita` Rust bindings: `0.9.1`;
- libadwaita APIs introduced in 1.5 are available behind the `v1_5` feature;
- Ubuntu 24.04 LTS package baseline includes GTK `4.14.2` and libadwaita `1.5.0`.

These checks justify the compatibility floor but do not claim that Ubuntu 24.04 is the newest Ubuntu release.

## Production boundary

Phase 150 performs no production runtime mutation.

It does not authorize:

- production public/LAN remote listeners;
- production STUN/ICE/TURN/relay activation;
- TUN/TAP or persistent route creation;
- firewall/NAT/router mutation;
- system resolver/private-DNS mutation;
- production Agent replacement/restart;
- production credential/key provisioning;
- real Android/Desktop distribution signing;
- production account/device cutover;
- remote-desktop capture/input activation.

Those remain behind their existing explicit gates.

## Completion classification

`PHASE_150_DONE / RUST_GTK4_LIBADWAITA_DESKTOP_ARCHITECTURE_LOCKED / GTK4_0_11_3_LIBADWAITA_0_9_1 / GTK_4_14_ADWAITA_1_5_COMPATIBILITY_FLOOR / UBUNTU_24_04_MINIMUM_BASELINE / FIXED_AUTHENTICATED_UNIX_IPC / AGENT_OWNS_HOST_AUTHORITY / NO_PRODUCTION_SIDE_EFFECT / READY_FOR_PHASE_151`
