# Private Remote Workspace — Direct-Connect Discovery and NAT-Traversal Foundation Contract

Status: Phase 141 source/disposable implementation contract
Date: 2026-08-16
Repository: `Gersi365/prw-executor-private`
Input baseline: `27c1903691421ec78822d68ca7b2ef8cda356436`

## Purpose

Phase 141 adds a bounded, standards-based direct-connect discovery and NAT-traversal foundation that feeds current reachability observations into the already validated Phase 135 connectivity selector. This phase remains source/disposable only. It does not activate production STUN/ICE/TURN traffic, expose a production UDP listener, mutate firewall/NAT/router/TUN/TAP/routes/DNS, deploy a relay, provision production credentials, replace or restart the production Agent, or distribute Android/Desktop clients.

## Authoritative inherited architecture

Phase 141 inherits the reconciled Phase 139/140 profile without reinterpretation:

- control-plane candidate exchange remains separate from the mesh data plane;
- mesh data transport remains QUIC v1 + TLS 1.3 mTLS in `crates/prw-remote-transport`;
- mesh ALPN remains `prw-mesh/1`;
- `TransportIdentity` remains SHA-256 of canonical transport-leaf SPKI DER;
- application control magic remains `PRWM` with protocol 1.0 and a 65,536-byte control-payload ceiling;
- connectivity preference remains `LocalDirect -> InternetDirect -> Relay -> Offline`;
- successful network reachability or ICE selection does not grant any PRW application capability.

The late duplicate Phase 139 documentation values reconciled by Phase 139 A02 have no normative force.

## Standards lock

Phase 141 uses standard traversal semantics:

- ICE — RFC 8445;
- STUN — RFC 8489;
- TURN — RFC 8656.

STUN is discovery/reachability machinery, not authentication or authorization. TURN is fallback traversal infrastructure and does not terminate the peer-to-peer PRW mesh TLS security association.

No proprietary hole-punching protocol is introduced.

## Exact dependency lock

Phase 139 deliberately deferred the exact ICE/STUN/TURN Rust implementation dependency to Phase 141 and required a current compatibility probe before pinning it.

The Phase 141 protocol engine is now locked to the following exact direct dependencies:

```toml
rtc-ice = { version = "=0.20.2", default-features = false, features = ["aws-lc-rs"] }
rtc-shared = { version = "=0.20.2", default-features = false }
sansio = "=1.0.0"
```

`rtc-ice` is used as a Sans-I/O ICE state machine: PRW owns the surrounding product bounds and eventual socket/runtime integration, while the dependency owns ICE/STUN/TURN protocol behavior. This selection does not authorize production network activation.

Phase 141 dependency probe run `31952085411`, after the bounded A01 harness correction, proved under Rust/Cargo 1.97.1 that:

1. the exact direct versions resolve as `rtc-ice 0.20.2`, `rtc-shared 0.20.2`, `sansio 1.0.0`;
2. the exact scratch graph compiles successfully;
3. the exercised `Agent` Sans-I/O API compiles successfully;
4. the scratch `Cargo.lock` SHA-256 is `ad791a289eb61b6d5839d8b94d612b2e67fbd6efa837f3648766393935e6cb96`;
5. the repository `Cargo.toml` and `Cargo.lock` remain unchanged by the scratch probe.

The initial probe failed only in the disposable harness because `BytesMut` does not implement `From<Vec<u8>>`; A01 changed only the probe construction to use a byte slice and the rerun passed. This corrective did not weaken the dependency gate or mutate the repository dependency graph.

Phase 141 must not silently implement ICE/STUN/TURN authentication machinery from scratch.

## Component ownership

Phase 141 may introduce one dedicated crate named `prw-nat-traversal`.

Ownership boundaries:

- `prw-nat-traversal` owns the selected ICE/STUN/TURN protocol state-machine adapter, traversal candidate normalization, bounded observation production, transaction/cancellation/time bounds and disposable traversal validation;
- `prw-connectivity` remains provider-neutral and continues to own product path classes, bounded candidate plans, reachability observations and deterministic selection;
- `prw-remote-transport` remains the QUIC/TLS owner and must not acquire ICE policy or TURN credentials;
- `prw-relay` remains the opaque relay-fallback product boundary and is not converted into a TURN plaintext terminator;
- `prw-agent` must not gain production traversal activation in Phase 141;
- control-plane signaling remains the authenticated mechanism by which peer candidate metadata is eventually exchanged.

## Candidate model

The Phase 141 adapter must normalize traversal results into the existing explicit-IP `ConnectivityEndpoint` and `ConnectivityCandidate` types.

Traversal candidate classes map as follows:

- host candidate on a local/private interface -> `LocalDirect` only when the candidate is explicitly classified as local/private and is valid for the peer context;
- server-reflexive, peer-reflexive or directly routable non-local candidate -> `InternetDirect`;
- TURN relayed candidate -> `Relay`;
- failed, expired or unusable candidate -> observation `Unreachable` or no candidate, never an invented reachable path.

Candidate identifiers remain non-zero and plan-scoped. The existing Phase 135 capacity of 16 candidates remains authoritative for the product plan. Phase 141 must fail closed or deterministically prune before exceeding that bound; it must not expand the Phase 135 capacity silently.

## Reachability observation rules

A candidate becomes `Reachable` only after the traversal engine has evidence of a currently selected/succeeded candidate pair or an equivalent completed reachability check appropriate to the standards implementation.

Merely gathering a host, server-reflexive, peer-reflexive or relayed address does not make it reachable.

Unknown, pending, failed, timed-out and withdrawn checks must not be promoted to `Reachable`.

When observations are applied to `PeerConnectivityPlan`, the existing selector remains authoritative. Phase 141 does not implement a competing preference algorithm.

## Identity and authorization boundary

Traversal state binds to the already authenticated peer context and expected `TransportIdentity`, but ICE/STUN/TURN credentials are not PRW application authorization.

A successful selected candidate pair only authorizes an attempt to establish the Phase 140 mesh transport. The subsequent QUIC/TLS mTLS identity checks, current registry validation, authenticated application session and capability policy still must all succeed independently.

No STUN/TURN response may directly create a PRW authenticated session.

## Bounded configuration

The Phase 141 source foundation must use explicit finite bounds for at least:

- number of STUN servers;
- number of TURN servers;
- number of local candidates admitted to one traversal session;
- number of remote candidates admitted to one traversal session;
- total product candidates exported to `PeerConnectivityPlan` (maximum 16);
- per-pair binding request count;
- overall gather/check deadline;
- maximum queued protocol transmits/events exposed by the adapter;
- credential/string lengths accepted by any PRW-owned wrapper types.

The exact numeric values may be selected by the Phase 141 implementation, but must be finite, tested at the boundary and recorded in the authoritative report. PRW must keep `AgentConfig::insecure_skip_verify` false.

## Credential handling

TURN credentials, when modeled in Phase 141, are secret inputs and must not appear in PRW-owned `Debug`, error text, audit evidence, logs or public state snapshots.

Phase 141 uses only disposable test credentials. No production TURN account, API token, long-term password or production STUN/TURN endpoint is committed to the repository.

## Disposable validation requirements

Phase 141 must prove at minimum:

1. the selected exact ICE/STUN/TURN dependency graph compiles under Rust/Cargo 1.97.1;
2. the repository lockfile is regenerated transactionally only after the candidate dependency passes the scratch gate;
3. provider-neutral Phase 135 selection semantics remain unchanged;
4. the traversal adapter can configure standard ICE/STUN/TURN inputs through the selected library without production socket activation;
5. gathered/selected traversal results map deterministically to `LocalDirect`, `InternetDirect` or `Relay` and never bypass the 16-candidate bound;
6. pending/failed/timed-out traversal state does not become `Reachable`;
7. selected direct candidate beats relay through the existing Phase 135 selector;
8. when no direct candidate is reachable, a reachable relay candidate remains fallback only;
9. invalid endpoints, zero IDs, duplicate candidates and capacity overflow fail closed through existing typed boundaries;
10. TURN secret wrapper debug/error behavior does not disclose secret bytes;
11. cancellation/deadline state is finite and terminal;
12. focused rustfmt, Clippy `-D warnings`, tests and build pass;
13. full workspace locked metadata, rustfmt, Clippy, tests and build pass;
14. no production network, Agent, DNS, firewall/router, credential or client-distribution state is mutated.

A disposable in-memory or loopback harness is acceptable. Public Internet STUN/TURN traffic is not required to complete Phase 141 and must not be used as a substitute for deterministic tests.

## Phase 142 handoff

Phase 141 may model TURN relay candidates and standard TURN client protocol state, but Phase 142 owns the PRW relay protocol/provider and disposable relay service around the existing Phase 136 opacity/fallback contract.

Phase 141 must not deploy a production relay or broaden the relay into a plaintext application-data terminator.

## Production boundary

Until the separately approved exact Phase 154 transaction, Phase 141 MUST NOT:

- expose a production UDP listener or public/LAN QUIC endpoint;
- enable persistent production ICE/STUN/TURN activity;
- create or alter firewall/NAT/router/TUN/TAP/routes;
- mutate resolver/private-DNS state;
- provision production transport or TURN credentials;
- replace/restart the production Agent for traversal;
- distribute/sign Android/Desktop clients;
- deploy a production relay.

## Completion classification

Phase 141 is complete only when the dependency probe, implementation, focused validation, full locked-workspace validation, authoritative audit report and cleanup of temporary validation workflow(s) all pass.

Target final state:

`PHASE_141_DONE / DIRECT_CONNECT_NAT_TRAVERSAL_FOUNDATION_VALIDATED / ICE_RFC8445_STUN_RFC8489_TURN_RFC8656 / PHASE135_SELECTOR_PRESERVED / NO_PRODUCTION_SIDE_EFFECT / READY_FOR_PHASE_142`
