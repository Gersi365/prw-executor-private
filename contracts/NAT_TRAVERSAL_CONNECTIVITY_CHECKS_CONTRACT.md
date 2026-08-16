# Private Remote Workspace — NAT Traversal and Connectivity Checks Contract

Status: Phase 141 implementation lock
Date: 2026-08-16
Repository: `Gersi365/prw-executor-private`
Parent architecture: `contracts/REMOTE_TRANSPORT_ARCHITECTURE_DECISION.md`
Parent transport validation: `contracts/DISPOSABLE_ENCRYPTED_REMOTE_TRANSPORT_CONTRACT.md`

## Purpose

Phase 141 implements standards-based NAT traversal and connectivity checking only as source and disposable/in-memory validation. It turns explicit UDP candidate information into bounded ICE/STUN protocol state that can feed the already validated Phase 135 connectivity selector.

Phase 141 does **not** activate production STUN/ICE/TURN traffic, bind a production UDP endpoint, alter firewall/NAT/router/TUN/TAP/routes/DNS, deploy relay infrastructure, or integrate/restart the production Agent.

## Standards and implementation dependency lock

Normative traversal standards remain:

- ICE — RFC 8445;
- STUN — RFC 8489 architecture/profile, using the selected implementation's compatible STUN protocol machinery;
- TURN — RFC 8656 architecture for relay allocation/fallback, with actual relay provider/allocation implementation deferred to Phase 142.

Initial stable Rust implementation dependencies:

```toml
rtc-ice = { version = "=0.20.2", default-features = false, features = ["aws-lc-rs"] }
rtc-stun = { version = "=0.20.2", default-features = false, features = ["aws-lc-rs"] }
rtc-shared = { version = "=0.20.2", default-features = false }
sansio = "=1.0.0"
```

Internal dependency:

```toml
prw-connectivity = { path = "../prw-connectivity" }
```

The `rtc` 0.21 line is prerelease at this decision point and is not selected for the initial PRW baseline. Phase 141 uses stable `0.20.2`.

A disposable Rust/Cargo 1.97.1 probe run `31952085411`, job `95177012142`, compiled the stable Sans-I/O ICE API with exact rtc-ice/rtc-shared/sansio versions. Scratch lock SHA-256:

`ad791a289eb61b6d5839d8b94d612b2e67fbd6efa837f3648766393935e6cb96`

The preceding run `31951951887` resolved the exact versions successfully but stopped on a probe-only `Vec<u8>` to `BytesMut` construction error before repository mutation; A01 corrected only that harness expression.

## Sans-I/O ownership boundary

`crates/prw-nat-traversal` owns protocol state, not sockets.

The runtime crate MUST NOT own or expose:

- `UdpSocket`, `TcpStream`, `TcpListener` or raw socket creation;
- Tokio or another async runtime;
- DNS lookup/resolver APIs;
- mDNS socket discovery;
- TUN/TAP, route, firewall, NAT/router mutation;
- process/shell execution;
- arbitrary raw network destinations unrelated to a validated traversal session.

The caller supplies explicit local/peer/server `SocketAddr` values and moves bounded datagrams between the PRW traversal state machine and a separately controlled network adapter in a later integration phase.

This keeps Phase 141 testable entirely in memory and prevents traversal protocol code from becoming an unrestricted network capability.

## DNS independence

STUN/TURN server locations and peer candidates supplied to this layer are explicit socket addresses. This core does not resolve hostnames.

ICE mDNS is disabled in the initial PRW profile. Optional Phase 137 private DNS remains outside path discovery and is not required for connectivity.

## Candidate profile

Initial PRW Phase 141 candidate profile is UDP-only:

- IPv4 host;
- IPv6 host;
- IPv4 server-reflexive;
- IPv6 server-reflexive.

ICE-TCP is deferred.

Relay candidates may be represented by the existing Phase 135 `Relay` path only after Phase 142 provides an authorized relay allocation/provider. Phase 141 does not allocate TURN relays by itself.

The existing Phase 135 maximum of 16 candidates per peer remains authoritative. Duplicate candidate identifiers and duplicate path/endpoint tuples remain rejected by `prw-connectivity`.

## STUN discovery boundary

Phase 141 provides a bounded Sans-I/O STUN binding transaction wrapper.

Inputs:

- validated local `SocketAddr`;
- explicit STUN-server `SocketAddr`;
- current monotonic time supplied by caller.

Outputs:

- bounded datagram(s) addressed only to the configured STUN server;
- finite next-timeout deadline;
- one validated XOR-MAPPED server-reflexive endpoint on success;
- stable failure classification on malformed/unexpected/timeout state.

Rules:

- maximum accepted/emitted traversal datagram: 2048 bytes;
- datagrams from an unexpected source endpoint fail closed;
- an invalid/zero/unspecified mapped endpoint is rejected before conversion to a PRW connectivity endpoint;
- the upstream STUN transaction engine owns RFC-style transaction IDs, retransmission and message parsing;
- no STUN response authenticates a PRW user/device or grants a capability.

## ICE connectivity-check boundary

Phase 141 wraps the selected Sans-I/O ICE `Agent` with a PRW-specific bounded API.

Initial configuration:

- mDNS disabled;
- `insecure_skip_verify = false`;
- UDP4/UDP6 network types only;
- host and server-reflexive candidate types only;
- relay candidate gathering/allocation disabled in this phase;
- finite disconnected/failed/check/keepalive behavior;
- maximum binding requests explicitly bounded;
- local ICE credentials generated by the vetted upstream implementation;
- remote ICE credentials supplied only through authenticated PRW coordination metadata.

ICE credentials are traversal credentials only. They are not account, device, transport-certificate or capability credentials.

## Candidate correlation and Phase 135 integration

Every PRW-added ICE candidate is correlated with an existing `CandidateId` and `ConnectivityCandidate` from the Phase 135 plan.

A selected ICE pair produces a typed reachability observation for the corresponding PRW candidate. The caller then updates the Phase 135 plan through its existing `set_observation` API.

Phase 141 MUST NOT duplicate or replace Phase 135 path ordering. The authoritative product selection remains:

`LocalDirect -> InternetDirect -> Relay -> Offline`

A successful ICE check may mark a candidate reachable; it does not by itself choose a less-preferred candidate over a reachable more-preferred Phase 135 path.

## Datagram boundary

A traversal datagram contains only:

- explicit local `SocketAddr`;
- explicit peer `SocketAddr`;
- bounded protocol bytes.

The wrapper rejects:

- payloads above 2048 bytes;
- unspecified/multicast/broadcast endpoint inputs where applicable;
- inbound datagrams whose local address does not belong to the session;
- STUN-discovery responses from a source other than the configured STUN server;
- ICE datagrams that cannot be attributed to the configured candidate/session context.

## Required disposable validation

The authoritative Phase 141 validation must prove at least:

1. exact locked dependency graph on Rust/Cargo 1.97.1;
2. no Tokio/socket/DNS/process/TUN/firewall/route APIs in `prw-nat-traversal` runtime source;
3. bounded STUN Binding request emission;
4. synthetic STUN Binding success decodes a valid XOR-MAPPED endpoint;
5. wrong STUN source and oversized datagrams fail closed;
6. two ICE agents can complete host-candidate checks entirely in memory by exchanging only typed datagrams;
7. wrong remote ICE credentials do not produce a selected pair;
8. UDP-only and mDNS-disabled configuration is explicit;
9. candidate count stays within the Phase 135 bound;
10. selected ICE candidate correlation yields a Phase 135 `Reachable` observation without bypassing the Phase 135 selector;
11. focused rustfmt/Clippy `-D warnings`/tests pass;
12. full workspace rustfmt/Clippy/tests/build pass under the locked graph;
13. no PowerCode or other production network/runtime state is mutated.

## Deferred to Phase 142

Phase 142 owns disposable relay-provider/allocation behavior, including TURN allocation or a PRW relay provider that preserves end-to-end QUIC opacity and Phase 136 fallback-only policy.

Phase 141 may lock TURN as the standard fallback architecture but MUST NOT silently deploy or persist a relay service.

## Production boundary

Until Phase 154 explicit production activation, Phase 141 MUST NOT:

- start persistent STUN/ICE/TURN traffic;
- publish or bind a production data-plane UDP endpoint;
- alter firewall/NAT/router/TUN/TAP/routes/DNS;
- provision production transport credentials;
- integrate/restart/replace the production Agent for traversal;
- deploy relay infrastructure;
- distribute a production Android/Desktop client.

Final target state:

`PHASE_141_IMPLEMENTED_AND_DISPOSABLY_VALIDATED / STANDARD_ICE_STUN / SANS_IO_BOUNDARY / PHASE135_SELECTOR_PRESERVED / NO_PRODUCTION_SIDE_EFFECT / READY_FOR_PHASE_142`
