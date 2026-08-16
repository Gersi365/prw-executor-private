# Private Remote Workspace — Productization Roadmap and Mutation Gates Contract

Status: Phase 138 decision/readiness lock
Date: 2026-08-16
Repository: `Gersi365/prw-executor-private`
Baseline head: `837fb20f65575b53fc2dd2145fd93702216de6d3`

## Purpose

Phase 138 transitions PRW from the completed source/disposable foundation roadmap through Phase 137 into a productization roadmap. This phase is documentation and decision-lock work only. It authorizes no production networking, client deployment, credential mutation, firewall/router/DNS change, systemd mutation, or replacement/restart of the production Agent.

## Baseline facts

1. The headless Ubuntu Agent and its local production lifecycle foundation already exist.
2. Device identity, authenticated session, registry, file, transfer, terminal, forwarding, connectivity-path, relay-fallback and optional-private-DNS foundations exist as validated Rust workspace components.
3. `apps/android` and `apps/desktop` are still placeholder application directories containing only planning README files; neither is a functional client.
4. Basic connectivity is intentionally independent from optional private DNS.
5. The production PowerCode networking path has not been converted into the full remote product data plane by Phases 134–137.
6. The existing narrow production device-identity/network mutation gates remain in force and are not widened by this contract.

## Productization principles

- Preserve control-plane/data-plane separation.
- Preserve user identity, device identity and transport/network identity separation.
- Prefer direct connectivity; relay remains fallback.
- Keep the Agent headless and independent from desktop/mobile UI lifecycle.
- Keep private DNS optional.
- Do not introduce new cryptographic primitives.
- Use audited standard protocols/libraries and explicit dependency review before transport implementation.
- Keep privileged operations narrow and separately gated.
- Do not make remote reachability depend on Wake-on-LAN.
- Do not make Tailscale, Termius or another remote-access product a runtime prerequisite.

## New roadmap

### Phase 139 — Remote transport standards and dependency decision

Decision-only / source-readiness phase. Select and document the standard transport/security building blocks, dependency ownership, protocol versioning, framing limits, identity binding and key/certificate lifecycle. No socket activation or production network mutation.

This phase is an architecture decision and requires an explicit architecture approval before the selected stack is locked.

### Phase 140 — Disposable encrypted transport implementation foundation

Implement the selected encrypted transport only in source/disposable test environments. Require authenticated peer identity binding, bounded framing, cancellation/timeouts, fail-closed verification and no production endpoint publication.

### Phase 141 — Direct-connect discovery and NAT-traversal foundation

Implement disposable discovery/reachability/NAT traversal mechanics and deterministic candidate observation feeding the Phase 135 selector. No production TUN, firewall, route or public listener mutation.

### Phase 142 — Relay protocol and disposable relay service

Implement the relay transport/provider around the Phase 136 opacity/fallback contract. Relay must not become a plaintext termination point for end-to-end protected payloads. No production relay deployment.

### Phase 143 — End-to-end authenticated capability bridge

Connect authenticated remote transport to already-built Agent capability boundaries for device state, files/transfers, terminal and port-forward control. Validate capability checks, session expiry/revocation and fail-closed behavior end-to-end in disposable environments.

### Phase 144 — Android client architecture decision

Lock Android implementation architecture, minimum API/toolchain, local key custody, background/foreground service boundaries, transport ownership and UI/state model. This is an architecture decision and requires explicit approval before framework/toolchain lock.

### Phases 145–149 — Android functional client slices

145. Minimal application shell, identity custody and authenticated connection bootstrap.
146. Enrollment, device list/status and revocation surfaces.
147. Remote terminal UX and bounded terminal session lifecycle.
148. Remote file browser plus upload/download/resume/progress/retry.
149. Port-forward management, private-network status and optional private-DNS settings.

These remain non-production until separate Android distribution/signing and real-account/real-device activation gates are explicitly approved.

### Phase 150 — Desktop client architecture decision

Lock desktop framework/toolchain and authenticated local-IPC ownership model. This is an architecture decision and requires explicit approval before framework/toolchain lock.

### Phases 151–152 — Desktop functional client foundation

151. Device/status/enrollment shell and authenticated local Agent IPC.
152. Terminal, files/transfers, forwarding, network and optional-DNS management surfaces.

### Phase 153 — Production remote-network activation readiness

Read-only/readiness phase. Produce exact production artifacts, hashes, ports/endpoints, privileges, firewall/route/DNS effects, rollback bytes, service/drop-in changes, credential dependencies and postconditions. No production mutation.

### Phase 154 — Explicit production remote-network activation gate

No execution under generic continuation. Requires explicit approval for the exact real-host transaction. The approval must enumerate the intended production mutations, rollback path and user-visible networking effects.

## Explicit production mutation boundary

Until Phase 154 is separately approved, the following remain forbidden on PowerCode or any real production endpoint:

- enabling a public/LAN remote listener for the new PRW data plane;
- creating a TUN/TAP device or persistent private route;
- firewall/NAT/router mutation;
- STUN/ICE/NAT-traversal traffic used as production service activation;
- production relay enrollment/deployment or persistent relay route-token installation;
- system resolver/private-DNS mutation;
- production Agent replacement/restart for the new remote data plane;
- production credential/key provisioning for the new transport stack;
- real Android/Desktop distribution signing or production-account cutover.

Read-only inspection and disposable validation do not cross this gate.

## Remote desktop boundary

RustDesk-like remote desktop is not locked into this roadmap. The current architecture should preserve room for a later first-class remote-desktop capability, but screen capture, encoding/streaming, input injection, clipboard, multi-monitor handling and its additional permission/security model require a separate explicit roadmap decision.

## Completion criteria for Phase 138

Phase 138 is complete when:

1. the post-137 roadmap is recorded;
2. architecture decisions are distinguished from implementation phases;
3. production mutation gates are explicit;
4. Android and desktop placeholder status is acknowledged;
5. remote desktop is not silently added to scope;
6. no runtime/source/production state is changed by the phase.

Final state: `PHASE_138_DECISION_LOCKED / READY_FOR_EXPLICIT_PHASE_139_ARCHITECTURE_DECISION`.
