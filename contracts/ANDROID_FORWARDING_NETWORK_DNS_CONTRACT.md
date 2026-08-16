# Private Remote Workspace Android Forwarding, Network Status and Optional DNS Contract

Version: `0.1.0`

Status: Phase 149 implementation lock
Date: 2026-08-16

## Purpose

Phase 149 completes the currently planned Android functional-client slices by adding non-production Android surfaces for:

- typed TCP port-forward management;
- private-network status presentation;
- optional private-DNS configuration presentation and validation.

The phase reuses already validated Phase 134 forwarding, Phase 135 connectivity, Phase 137 private-DNS, Phase 143 remote capability-bridge and Phase 144 Android architecture authorities.

It does **not** activate a production forward, create a socket, perform a connectivity probe, change a route/firewall/NAT rule, create a TUN/TAP interface, mutate an operating-system resolver, publish DNS, restart/replace the production Agent, sign/distribute Android production artifacts, or cross the Phase 154 production remote-network mutation gate.

## Baseline gate

Implementation may materialize only from a repository state whose Phase 148 authoritative report proves all of:

- `PHASE_148_FUNCTIONALLY_VALIDATED`;
- `MATERIALIZED_ON_MAIN`;
- `PERMANENT_ANDROID_CI_SUCCESS`;
- `ROOT_RUST_CI_SUCCESS`;
- `READY_FOR_PHASE_149`.

Phase 149 must preserve the existing Phase 145–148 Android architecture and toolchain. No framework/toolchain architecture decision is reopened.

## Authority reuse

### Port forwarding

Forwarding validation remains owned by `prw-forwarding`.

The Android native adapter may construct only existing validated types:

- `PortForwardId`;
- `LoopbackFamily::{Ipv4, Ipv6}`;
- `LoopbackBind`;
- `ForwardTarget`;
- `TcpForwardSpec`.

Remote intent encoding must use the existing `prw-remote-bridge` operations:

- `BridgeCommand::ForwardOpen`;
- `BridgeCommand::ForwardClose`.

The Android adapter must not introduce a second forwarding protocol, arbitrary bind IP, hostname target, UDP mode, SOCKS mode, raw socket options, shell command, firewall instruction or privilege instruction.

A target is an explicit IP address plus a non-zero TCP port. Bind selection remains named loopback IPv4 or loopback IPv6 plus a non-zero TCP port.

### Private-network status

Connectivity validation and deterministic selection remain owned by `prw-connectivity`.

Android may display a typed authoritative/disposable snapshot containing the selected path classification:

- `LocalDirect`;
- `InternetDirect`;
- `Relay`;
- `Offline`.

Android must not fabricate reachability from UI state, connection labels, hostnames, DNS state or forwarding state. A UI request must not create a `Reachable` observation. For disposable validation, typed observations may be injected explicitly into an in-memory `PeerConnectivityPlan`; the resulting selected path must come from `PeerConnectivityPlan::selected_path()`.

No Phase 149 code performs discovery, STUN/ICE traffic, dialing, keepalive scheduling, endpoint publication, route mutation or relay byte transport.

### Optional private DNS

DNS syntax, bounds and semantic validation remain owned by `prw-private-dns`.

Android may create and present only validated `PrivateDnsConfig` values using existing typed authorities:

- `PrivateDnsMode::{Disabled, Enabled}`;
- `DnsDomainSuffix`;
- `ResolverEndpoint`.

The Android UI must distinguish **validated requested mode** from **operating-system applied state**. `Enabled` means the typed configuration requests enabled mode; Phase 149 must not claim that Android, Ubuntu, systemd-resolved, NetworkManager or any other resolver has been changed.

Private DNS remains optional and must not gate basic private-connectivity status. Phase 149 must not add a DNS mutation command to the remote capability bridge.

## Android presentation lifecycle

Phase 149 continues the established intent-versus-authoritative-state discipline used by terminal and file/transfer slices.

### Forwarding lifecycle

Android presentation state may represent:

- `Closed`;
- `Opening` after a valid open intent has been encoded;
- `Active` only after an explicit disposable/authoritative open acknowledgement;
- `Closing` after a valid close intent has been encoded;
- `Failed` only after an explicit disposable/authoritative failure indication.

Encoding `ForwardOpen` must not itself mark a forward `Active`.

Encoding `ForwardClose` must not itself claim a successful close. A separate explicit completion acknowledgement returns presentation state to `Closed`.

The UI tracks only bounded presentation metadata and the last encoded payload size. It owns no socket/backend handle.

### Network status lifecycle

The default network presentation is `Offline`/not-authoritatively-observed.

A selected path becomes visible only from an explicitly applied typed snapshot validated through `prw-connectivity`.

### DNS settings lifecycle

The default DNS presentation is `Disabled` with no claim of OS application.

A settings draft becomes visible as validated only after native typed validation succeeds. UI edit actions alone do not constitute OS resolver mutation or authoritative remote configuration.

## Native adapter boundary

Phase 149 may add one focused Android native module for forwarding/network/DNS projection and only the internal workspace path dependencies required to reuse:

- `prw-forwarding`;
- `prw-connectivity`;
- `prw-private-dns`.

`prw-remote-bridge` is already an Android native dependency and remains the forwarding wire authority.

No new crates.io dependency is authorized by this phase.

JNI methods must remain bounded, fail closed and return an empty/false result on invalid presentation input following the established adapter convention. JNI must not expose arbitrary commands, socket ownership, filesystem authority, private keys, resolver mutation instructions or raw network mutation handles.

## Kotlin boundary

Phase 149 may add one focused Kotlin controller/model file for forwarding, connectivity status and DNS presentation and may minimally extend:

- `NativeBridge.kt` for bounded JNI declarations;
- `MainViewModel.kt` for disposable intent/acknowledgement demonstrations;
- `MainActivity.kt` for the Phase 149 Compose surfaces.

No Android permission expansion is authorized. No background/foreground networking service, VPN service, socket transport, production endpoint or external storage authority is added.

## Required validation

Phase 149 validation must prove at least:

### Native forwarding

- zero/negative/overflow forwarding identifiers fail closed;
- invalid loopback-family code fails closed;
- zero/overflow bind and target ports fail closed;
- target hostname/non-IP/unspecified/multicast/IPv4-broadcast input fails closed through existing authority;
- valid IPv4 and IPv6 targets encode to exact existing `BridgeCommand::ForwardOpen` values;
- close intent encodes to exact existing `BridgeCommand::ForwardClose` value;
- Phase 143 decode identifies generated forwarding payloads without adding a new operation code.

### Connectivity status

- default/no reachable candidate produces `Offline`;
- reachable `LocalDirect` wins over reachable `InternetDirect` and `Relay`;
- reachable `InternetDirect` wins over relay when local is unavailable;
- relay is selected only when no direct candidate is reachable;
- UI cannot manufacture a selected path without an explicit typed snapshot/application action.

### Optional private DNS

- default presentation is disabled and not OS-applied;
- valid enabled/disabled typed configurations are accepted without mutation;
- invalid domain/resolver values fail closed through `prw-private-dns` constructors;
- device naming without a device domain fails closed;
- split domains without a resolver fail closed;
- DNS state does not alter connectivity selection.

### Android state

- forward open intent produces `Opening`, not `Active`;
- explicit open acknowledgement produces `Active`;
- close intent produces `Closing` and explicit close acknowledgement returns `Closed`;
- disposable authoritative connectivity status updates presentation only through the controller boundary;
- DNS validation updates requested/validated settings but never an `osApplied=true` claim;
- existing enrollment/device/terminal/file/transfer behavior remains build- and test-valid.

### Whole repository

Validation must run the exact locked toolchains and pass:

- Android native `cargo fmt --check`;
- Android native `cargo clippy --locked --all-targets --all-features -- -D warnings`;
- Android native tests;
- both existing Android native ABI builds;
- Android `testDebugUnitTest`, `lintDebug`, and `assembleDebug`;
- root locked dependency graph check;
- root Rust format, Clippy, tests and build.

After materialization, permanent Android CI and root Rust CI must both succeed before Phase 149 can be classified complete.

## Scope boundary checks

The materialized Phase 149 source must prove absence of:

- production/public/LAN listener activation;
- direct `Socket`, `ServerSocket`, `DatagramSocket`, `VpnService` or equivalent Android networking ownership added by this slice;
- shell/terminal use as a forwarding or DNS API;
- wildcard/arbitrary forwarding bind addresses;
- DNS hostnames as forwarding targets;
- `resolvectl`, `/etc/resolv.conf`, NetworkManager or systemd-resolved mutation;
- route/firewall/NAT/router/TUN/TAP mutation;
- production Agent service replacement/restart;
- release signing/distribution or production-account cutover;
- new external Android/native dependency.

## Audit evidence

The authoritative Phase 149 report must be preserved at:

`logs/audits/phase-149-android-forwarding-network-dns/PRW-PHASE-149-ANDROID-FORWARDING-NETWORK-DNS-VALIDATION.txt`

The report must distinguish candidate/preflight validation from permanent post-materialization CI and record exact materialized commit, permanent-CI trigger commit, workflow run identifiers, relevant source hashes and production-boundary result.

## Explicitly deferred

- real production forwarding activation;
- real socket forwarding backend ownership on Android;
- real network discovery/probing from the Android Phase 149 surface;
- production route/firewall/NAT/TUN/TAP mutation;
- OS-level private-DNS application;
- production Android account/device activation;
- release signing and distribution;
- desktop client implementation (Phases 150–152);
- production remote-network activation readiness (Phase 153);
- production remote-network activation (Phase 154, explicit approval required).

## Phase 150 handoff

Phase 149 is complete only after source materialization and permanent Android/root Rust CI success are recorded in the authoritative report.

The next roadmap phase is **Phase 150 — Desktop client architecture decision**. Phase 150 is an architecture decision and therefore requires explicit architecture approval before desktop framework/toolchain and authenticated local-IPC ownership are locked.

Final Phase 149 readiness marker after successful closure: `READY_FOR_EXPLICIT_PHASE_150_ARCHITECTURE_DECISION`.
