# Private Remote Workspace — Phase 152 C03e-B Mesh Transport TLS Handoff Source Materialization Staging

Status: source/materialization staging
Date: 2026-08-23
Repository: `Gersi365/prw-executor-private`
Exact predecessor branch: `phase-152-c03e-remote-session-transport-binding-source-materialization-staging`
Exact predecessor head: `1f98ba092946b84399336d8d35b5b1219fbc3075`
Exact predecessor tree: `4ed702a5c9a9562cc727d8c6e2fd52e5edeab79b`
Predecessor gate: `C03E_REMOTE_SESSION_TRANSPORT_BINDING_SOURCE_MATERIALIZED`

## Purpose

C03e-B is a security-driven split of the C03a planned C03e Agent remote-runtime checkpoint. C03b deliberately retained the Agent mesh PKCS#8 private key in `Zeroizing<Vec<u8>>` and exposed no reusable raw-key getter. C03c's locked TLS builder, correctly, consumes an owned rustls private-key value. A bounded ownership-transfer seam is therefore required before the Agent can construct a real authority-gated QUIC listener without weakening credential custody.

C03e-B materializes only that transfer and the transport-owned DER conversion helper. It does not construct or bind an Agent endpoint and does not publish remote readiness.

## Selected secret handoff

`MeshTransportCredentialMaterial` gains one consuming operation:

`into_transport_tls_der(self) -> (Vec<u8>, Vec<u8>, Zeroizing<Vec<u8>>)`

The tuple contains, in order:

1. the fixed private-root certificate DER;
2. the fixed Agent leaf certificate DER;
3. the fixed Agent PKCS#8 private-key DER still wrapped in `Zeroizing`.

The operation consumes the custody object. There is no `&[u8]`, `Vec<u8>` clone, `AsRef`, mutable borrow, or reusable raw private-key accessor. Debug remains redacted. The zeroizing key container remains live until the immediate caller transfers the owned bytes into the transport TLS builder.

This transfer is not authorization, endpoint readiness, or network activation.

## Transport-owned DER conversion

`prw-remote-transport::runtime` gains a helper that accepts owned root DER, leaf DER and PKCS#8 DER bytes and converts them only into the existing locked `build_server_config(...)` path:

- exactly one explicit root certificate;
- exactly one Agent leaf certificate;
- PKCS#8 typed as rustls `PrivatePkcs8KeyDer` / `PrivateKeyDer`;
- TLS 1.3 only;
- mandatory mTLS;
- ALPN `prw-mesh/1`;
- QUIC v1;
- no early data and no TLS tickets, as already locked by the existing builder.

The helper does not parse application identities, bind sockets, accept connections, grant capabilities, or publish readiness.

## Ownership and layering invariant

Credential custody remains responsible only for fixed, bounded, permission-checked systemd acquisition and zeroizing secret ownership. TLS/Quinn mechanics remain in `prw-remote-transport`.

C03e-C may consume this seam only from Agent source that itself requires the existing `ReachabilityAuthorityRuntimeOwner`. C03e-B does not weaken the C03a rule that an Agent remote runtime cannot be constructed before authority admission.

## Failure semantics

- malformed root DER -> existing `RemoteTransportError::InvalidTrustRoots` / transport failure;
- malformed or mismatched leaf/key -> existing TLS configuration failure;
- custody acquisition failure -> no handoff object and no TLS construction;
- no fallback credential source is introduced.

No error path creates a socket or remote-ready state.

## Protected boundaries

Relative to the predecessor, C03e-B must not change:

- `crates/prw-reachability-custody/Cargo.toml`;
- `crates/prw-remote-transport/Cargo.toml`;
- root `Cargo.lock`;
- `apps/android/native/Cargo.lock`;
- `crates/prw-agent/src/main.rs` or any Agent runtime/lifecycle source;
- workflows;
- Android application source.

## Expected source mutation

Exactly three paths are expected:

1. this contract;
2. `crates/prw-reachability-custody/src/mesh_transport_custody.rs`;
3. `crates/prw-remote-transport/src/runtime.rs`.

## Negative guarantees

C03e-B does not:

- bind UDP or QUIC sockets;
- construct Agent runtime ownership;
- accept a remote connection;
- execute logical session authentication;
- create `RemoteSessionLease`;
- dispatch a capability;
- run ICE/STUN/TURN or relay;
- spawn tasks or retries;
- mutate local Agent readiness;
- provision credentials;
- mutate systemd/firewall/NAT/routes/DNS/TUN/TAP;
- deploy, restart, merge, initialize PRWF, execute recovery epochs, or activate R1–R4 effects.

## Completion gate

After exact-head canonical Rust/Android validation and Drive closeout:

`C03E_B_MESH_TRANSPORT_TLS_HANDOFF_SOURCE_MATERIALIZED`
