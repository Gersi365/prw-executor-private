# Private Remote Workspace — Phase 139 Duplicate Decision Reconciliation Note

Status: `NON_NORMATIVE / RECONCILED`
Date: 2026-08-16
Repository: `Gersi365/prw-executor-private`

## Purpose

This path was created after Phase 140 had already been implemented and validated during a connection interruption. Its first revision accidentally restated Phase 139 with several values that conflicted with the earlier authoritative Phase 139 decision and the validated Phase 140 implementation.

It is **not** a second architecture contract.

## Normative authority

The authoritative Phase 139 architecture remains, in order:

1. `contracts/REMOTE_TRANSPORT_ARCHITECTURE_DECISION.md`;
2. `contracts/REMOTE_TRANSPORT_ARCHITECTURE_DECISION_A01.md`;
3. `logs/audits/phase-139-remote-transport-architecture/PRW-PHASE-139-REMOTE-TRANSPORT-ARCHITECTURE-DECISION.txt`;
4. `logs/audits/phase-139-remote-transport-architecture/PRW-PHASE-139-A01-TRANSPORT-IDENTITY-DIGEST-CLARIFICATION.txt`;
5. Phase 140 implementation/validation evidence where it proves conformance to those contracts.

The initial revision of this file MUST NOT override those sources.

## Reconciled Phase 139 profile

The authoritative profile is:

- control plane remains Phase 129 TCP/TLS 1.3 with ALPN `prw-control/1`;
- mesh data plane uses QUIC v1 over UDP with TLS 1.3 mutual authentication;
- mesh ALPN is exactly `prw-mesh/1`;
- runtime transport owner is `crates/prw-remote-transport`;
- direct runtime dependencies are Quinn `0.11.11`, rustls `0.23.43`, Tokio `1.53.1`, and narrowly scoped aws-lc-rs `1.18.0` per A01;
- `TransportIdentity` is SHA-256 of canonical leaf SPKI DER;
- peer certificate SAN/SNI is `t-<first32hex>.<last32hex>.mesh.prw.invalid` derived from the full transport identity and is not DNS-resolved for endpoint discovery;
- mesh application magic is `PRWM`, protocol `1.0`, fixed header 24 bytes, maximum control payload 65,536 bytes;
- initial remote stream limits are 32 bidirectional and 16 unidirectional;
- 0-RTT and TLS resumption are disabled; no application QUIC DATAGRAM API is enabled;
- deterministic initiator is the peer with lexicographically smaller 32-byte `TransportIdentity`;
- successful transport mTLS never grants PRW application capabilities without current registry mapping, authenticated application-session proof, membership revalidation and capability policy;
- ICE/STUN/TURN standards remain RFC 8445/RFC 8489/RFC 8656 for Phase 141 traversal work;
- Phase 154 remains the production remote-network activation gate.

## Corrected conflicting statements from the first revision of this path

The following late duplicate values are withdrawn and have no normative force:

- `prw-data/1` ALPN — authoritative value is `prw-mesh/1`;
- `PRWD` magic — authoritative value is `PRWM`;
- 2 MiB control-envelope ceiling — authoritative Phase 140 control ceiling is 65,536 bytes;
- disabling all unidirectional streams — authoritative initial remote unidirectional limit is 16;
- provisional crate name `prw-data-transport` — implemented/validated owner is `prw-remote-transport`;
- certificate-DER fingerprint as the transport binding — authoritative transport identity is SPKI SHA-256 as locked by Phase 139 A01.

## Evidence

Phase 139 exact dependency probe run `31949211214`, job `95169943036`, passed on Rust/Cargo 1.97.1.

Phase 139 A01 SPKI/provider probe run `31949682590`, job `95171127021`, passed.

Phase 140 authoritative validation run `31950645958` passed focused and full-workspace validation for the reconciled profile.

Permanent Rust validation run `31951517003` also passed on the repository head containing the late duplicate file before this reconciliation; this note corrects architecture semantics rather than source compilation.

## Production boundary

This reconciliation changes documentation only. It does not provision a production key/certificate, open a production UDP listener, alter PowerCode, change firewall/NAT/router/TUN/TAP/routes/DNS, start ICE/STUN/TURN production traffic, deploy relay infrastructure, restart/replace the production Agent, or distribute a client.

Final classification:

`PHASE_139_A02_RECONCILED / ORIGINAL_AUTHORITY_PRESERVED / PHASE_140_VALIDATED_PROFILE_RETAINED / NO_PRODUCTION_MUTATION`
