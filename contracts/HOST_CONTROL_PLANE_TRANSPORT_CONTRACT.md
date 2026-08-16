# Private Remote Workspace Host Control-Plane Transport Contract

Version: `0.1.0`

Status: Phase 129 implementation lock

## Purpose

Phase 129 establishes the initial authenticated-server host control-plane transport boundary for the Ubuntu PRW Agent. It provides an outbound-only TCP/TLS transport, a bounded binary frame envelope, explicit lifecycle/error classification, and disposable loopback validation.

Phase 129 does not grant application capabilities and does not replace the enrolled-device session authentication contract from Phase 128. A transport connection may be cryptographically connected to the control-plane service while remaining unauthenticated as a PRW device session.

## Architectural separation

Phase 129 preserves three independent boundaries:

1. **Transport/server identity** — TLS proves the expected control-plane server identity.
2. **Device identity** — the Phase 126 private device key remains separate from TLS client identity.
3. **Authenticated PRW session identity** — Phase 128 proof-of-possession must still complete before authenticated application operations.

TLS success MUST NOT be interpreted as device enrollment, device authentication, workspace membership, or capability authorization.

## Connection direction

The initial Host Agent transport is **outbound-only**.

The Agent MUST NOT add an Internet-facing listener in Phase 129.

The caller supplies:

- an explicit remote `SocketAddr` used for TCP connection;
- a separate expected TLS `ServerName` string;
- one or more explicitly provisioned trust-anchor certificates.

Because the TCP endpoint is an explicit socket address, Phase 129 basic connectivity does not depend on DNS resolution. A DNS name may still be used as the TLS certificate identity/SNI value without being used to resolve the TCP destination.

## TLS implementation profile

Phase 129 locks:

- rustls `0.23.43` exact dependency;
- rustls AWS-LC provider;
- TLS 1.3 only;
- server certificate validation through the normal rustls root-certificate verifier;
- no custom or `dangerous()` certificate verifier;
- no certificate-validation bypass;
- no plaintext fallback;
- no TLS 1.2 fallback;
- no TLS early data / 0-RTT;
- no TLS client certificate in this phase;
- ALPN exactly `prw-control/1`.

The control-plane trust roots are explicit inputs. Phase 129 does not silently load the public OS root store and does not trust every public CA by default.

TLS session resumption, if later enabled, MUST NOT replace Phase 128 application-layer session authentication. Phase 129 does not rely on resumed TLS state as PRW device authorization.

## Connection time bounds

The initial transport configuration MUST require bounded non-zero values for:

- TCP connect timeout;
- read timeout;
- write timeout.

The implementation must expose stable bounded error classifications rather than leaking unbounded provider or OS error strings into protocol state.

## Binary transport envelope

Phase 129 defines a transport envelope distinct from the same-user local IPC protocol.

Exact fixed header layout, network byte order:

1. magic — 4 bytes: `PRWC`;
2. protocol major — unsigned big-endian `u16`, value `1`;
3. protocol minor — unsigned big-endian `u16`, value `0`;
4. message kind — unsigned big-endian `u16`;
5. flags — unsigned big-endian `u16`, value `0` in Phase 129;
6. request identifier — unsigned big-endian `u64`, non-zero;
7. payload length — unsigned big-endian `u32`.

Exact header length: 24 bytes.

Initial message-kind codes:

- `1` — authentication;
- `2` — command;
- `3` — response;
- `4` — event;
- `5` — heartbeat;
- `6` — error.

Phase 129 transport does not define command semantics. Command authorization belongs to later policy/capability phases.

## Frame bounds

Initial maximum payload length:

`65536` bytes.

The reader MUST validate the complete fixed header before allocating payload storage.

The reader MUST reject:

- invalid magic;
- unsupported protocol version;
- unknown kind;
- non-zero flags;
- request identifier zero;
- payload length above the bound;
- truncated header;
- truncated payload.

A frame write MUST emit the exact header followed by the exact payload and MUST reject oversized payloads before writing.

## Transport lifecycle

Initial lifecycle states are conceptually:

- Disconnected;
- Connecting;
- TlsHandshaking;
- Established;
- Closed;
- Failed.

Phase 129 may implement one bounded connection attempt rather than the final long-running reconnect scheduler. Reconnect/backoff policy may be added after the one-attempt transport boundary is proven.

A successful connection requires all of:

- TCP connected within timeout;
- TLS 1.3 handshake complete;
- normal certificate verification success for the expected server name;
- negotiated ALPN exactly `prw-control/1`.

Any failure before those postconditions leaves no established transport.

## Required disposable validation

Phase 129 must prove at least:

- frame codec round-trip;
- exact locked header bytes;
- rejection of invalid magic/version/kind/flags/request-id/oversized payload/truncation;
- outbound TLS 1.3 connection to a disposable loopback server using an explicitly generated disposable CA;
- expected server-name verification succeeds;
- wrong server name fails;
- TLS 1.2-only server fails;
- wrong ALPN fails;
- no plaintext fallback occurs;
- no production PowerCode service/network configuration is mutated.

The disposable certificate authority and server private key are test material only and MUST NOT be promoted to production.

## Explicitly deferred

Phase 129 does not implement or activate:

- durable device/workspace registry;
- account authentication;
- production enrollment/session activation;
- remote file operations;
- terminal/SSH;
- port forwarding;
- NAT traversal;
- peer-to-peer mesh transport;
- relay service;
- private DNS;
- Android/Desktop UI;
- public inbound listener;
- production control-plane CA provisioning or endpoint deployment.

## Production causal boundary

Source/disposable Phase 129 implementation may proceed before the Phase 126 interactive root handoff is completed.

A real production PRW control-plane connection may be activated only when:

1. the identity-aware production Agent and real Phase 126 device identity are active;
2. real Phase 127 enrollment is complete;
3. Phase 128 authenticated-session foundation is validated;
4. a production control-plane endpoint and trust anchor are explicitly provisioned and audited;
5. Phase 129 source/disposable transport validation and permanent CI pass.

The user's authorization through Phase 137 covers the future production mutation once these technical prerequisites exist; no separate generic approval gate is introduced by this contract.
