# Phase 152 C03c — Reusable Real QUIC Socket Runtime Source Materialization Staging

Status: `SOURCE_MATERIALIZATION_STAGING / REAL_UDP_SOCKET / QUIC_V1 / TLS13_MTLS / EXPECTED_TRANSPORT_IDENTITY_REVALIDATION / BOUNDED_PRWM_STREAMS / NO_LOGICAL_SESSION_AUTH / NO_AGENT_WIRING / NO_ICE / NO_RELAY / NO_DEPLOYMENT / NO_MERGE`

Date: 2026-08-23  
Repository: `Gersi365/prw-executor-private`

## Purpose

C03c materializes a reusable production-source socket runtime around the already-validated `prw-remote-transport` QUIC/TLS mechanics. It moves the real UDP/Quinn endpoint, connection, and bounded PRWM stream operations out of disposable integration-test helpers into a reusable module.

## Exact prerequisite

C03c derives only from closed C03b:

- branch: `phase-152-c03b-ubuntu-mesh-transport-credential-custody-source-materialization-staging`;
- head: `2791c9d785ca45939072757a480f33413466c404`;
- tree: `083accfb96ae0f327b75c80b57446ea7848af5a1`;
- gate: `C03B_UBUNTU_MESH_TRANSPORT_CREDENTIAL_CUSTODY_SOURCE_MATERIALIZED`.

## Existing transport invariants retained

C03c does not define a new wire or security protocol. It reuses unchanged:

- QUIC version 1 only;
- TLS 1.3 mTLS;
- ALPN `prw-mesh/1`;
- certificate-SPKI-derived `TransportIdentity`;
- deterministic transport server name;
- explicit trust roots;
- disabled TLS early data/resumption behavior selected by the existing transport source;
- bounded PRWM v1 control framing;
- existing `OPERATION_TIMEOUT`.

## Runtime surface

A new `prw_remote_transport::runtime` module may own only:

- real UDP socket bind;
- Quinn endpoint construction;
- kernel-assigned local address inspection;
- bounded accept/connect;
- negotiated ALPN validation;
- expected peer `TransportIdentity` revalidation;
- bounded bidirectional control-stream open/accept;
- bounded one-frame PRWM send/receive;
- explicit connection/endpoint close and idle wait.

The runtime returns no authenticated logical PRW session and grants no PRW capability.

## Fail-closed behavior

Socket bind, endpoint construction, connect start, accept closure, timeout, handshake, ALPN mismatch, peer identity mismatch, stream open/accept, read, write, finish, or PRWM decoding failures are terminal errors for that operation. No plaintext fallback, unauthenticated fallback, identity downgrade, retry loop, or alternate protocol is introduced.

## Dependency boundary

No manifest or lockfile change is required. `prw-remote-transport` already has the exact production dependencies needed: Quinn `0.11.11` with Tokio runtime support, Tokio `1.53.1` with networking/time/I/O features, rustls `0.23.43`, and AWS-LC-backed TLS.

## Canonical real-socket validation

Integration tests must bind actual kernel UDP sockets on loopback, establish Quinn QUIC v1/TLS1.3 mTLS using disposable certificates, validate both peer transport identities, exchange a bounded PRWM request/response over a real bidirectional QUIC stream, and fail closed when the expected peer identity is wrong.

This is real socket/network I/O but is not represented as a physical Internet-to-Internet production deployment proof.

## Permanent scope

The C03c net diff is limited to:

1. this contract;
2. `crates/prw-remote-transport/src/runtime.rs`;
3. one module export in `crates/prw-remote-transport/src/lib.rs`;
4. `crates/prw-remote-transport/tests/runtime.rs`.

No Cargo manifest, Cargo.lock, workflow, Agent `main.rs`, reachability authority, custody source, logical session-auth contract, Android source, ICE, relay, systemd, recovery/PRWF, R1-R4, production credential, deployment, or merge mutation is authorized.

## Gate

After exact-head canonical CI and evidence closeout:

`C03C_REUSABLE_REAL_QUIC_SOCKET_RUNTIME_SOURCE_MATERIALIZED`
