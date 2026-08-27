# Phase 152 C03e-CI — PRWC Generic Server / Accepted-Stream Source Materialization

Status: `STAGED SOURCE — PENDING EXACT-HEAD VALIDATION`

Target gate:
`C03E_CI_PRWC_GENERIC_SERVER_ACCEPTED_STREAM_SOURCE_MATERIALIZED`

## 1. Exact predecessor

C03e-CI is a direct source-materialization successor of closed C03e-CH:

- branch: `phase-152-c03e-ch-prwc-server-runtime-source-materialization-sequence-scope-selection-staging`;
- head: `55c00148e0ff8ab2397cb61d3e167bf153d87908`;
- tree: `058e581e020e0350f72abdae4557c231227ae4f7`;
- gate: `C03E_CH_PRWC_SERVER_RUNTIME_SOURCE_MATERIALIZATION_SEQUENCE_SCOPE_SELECTED`;
- PR #205 remains draft/open/unmerged with body `Status: CLOSED`.

C03e-CH authorized C03e-CI as the first bounded source unit before BY request-ID source
materialization or bridge authentication connection execution.

## 2. Exact repository scope

C03e-CI is limited to exactly these three paths:

1. `crates/prw-control-transport/src/lib.rs`;
2. `crates/prw-control-transport/tests/tls_loopback.rs`;
3. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CI_PRWC_GENERIC_SERVER_ACCEPTED_STREAM_SOURCE_MATERIALIZATION_STAGING.md`.

Any fourth changed path requires a fresh scope audit before mutation.

Explicitly excluded:
- `crates/prw-control-transport/Cargo.toml`;
- root and Android Cargo lockfiles;
- all `prw-remote-bridge` source;
- Agent/Desktop/Android application source;
- workflows;
- production credential fixture replacement;
- provider/database state;
- deployment/network configuration.

## 3. Dependency and lock proof basis

At exact C03e-CH:

- `crates/prw-control-transport/Cargo.toml` already has production
  `rustls = 0.23.43` with `std` and `aws_lc_rs`;
- existing disposable loopback tests already compile `ServerConfig`,
  `ServerConnection`, `TcpListener`, TLS 1.3 and the same provider;
- no new dependency is required for the selected production primitive;
- root `Cargo.lock` is `eeacde7ee776d35088f746a6d09f823f3391b82b`;
- Android native `Cargo.lock` is `cce9ca06190a196661ab38d54a747893e26af95f`.

Both lock blobs and the control-transport manifest must remain byte-stable.

## 4. Materialized generic server profile

The production transport source may expose a server configuration that consumes only
caller-supplied owned certificate-chain and private-key DER material plus bounded read/write
socket timeouts.

The server profile is locked to:
- existing aws-lc rustls provider;
- TLS 1.3 only;
- exact `CONTROL_ALPN = b"prw-control/1"` only;
- no client-certificate authentication in the initial profile;
- server early data disabled;
- no credential discovery, provisioning, persistence or rotation.

TLS success remains transport authentication only. It never establishes a PRW logical session.

## 5. Listener ownership boundary

The materialized generic transport may:
- bind one caller-supplied `SocketAddr`;
- return one owned listener value;
- expose the actual local address;
- perform exactly one accept per explicit caller invocation.

It must not:
- spawn a background accept loop;
- own bridge runtime lifetime;
- retry bind or accept;
- discover addresses;
- publish readiness;
- create worker/task ownership.

## 6. Accepted socket and TLS boundary

Each explicit accept:
1. accepts one TCP socket;
2. applies the already selected non-zero bounded read timeout;
3. applies the already selected non-zero bounded write timeout;
4. enables TCP no-delay;
5. constructs one rustls `ServerConnection`;
6. completes the TLS handshake before return;
7. verifies negotiated TLS is exactly TLS 1.3;
8. verifies negotiated ALPN is exactly `CONTROL_ALPN`;
9. returns one server-side PRWC stream only after those checks.

Any failure returns a bounded generic transport error and no usable accepted stream.

## 7. Accepted PRWC stream

The server-side accepted stream reuses the existing Phase 129 frame codec:
- `read_control_frame(...)`;
- `write_control_frame(...)`;
- existing frame bounds, magic, version, kinds, non-zero request ID and payload maximum.

The accepted stream exposes bounded `read_frame(...)` / `write_frame(...)` operations only.
It carries no semantic authentication or routing state.

## 8. Authority separation

The generic server primitive must not know or own:
- PRWA operation state;
- `DeviceId`, `SessionId`, `AuthenticatedDeviceSession`;
- `SessionAuthenticationService`;
- `WorkspaceDeviceRegistry`;
- BY request-ID allocation/outstanding lifecycle;
- requester/rendezvous state;
- PRWP candidate publication;
- freshness/current-plan authority;
- capability authorization or dispatch.

Those remain above transport at separately gated bridge/runtime boundaries.

## 9. Focused disposable validation

`tls_loopback.rs` continues to use only existing disposable loopback fixtures.

Focused validation must prove:
- new production server primitive and existing production client establish TLS 1.3 with exact ALPN;
- server-side accepted stream reads a bounded PRWC frame and writes a correlated bounded frame;
- empty/invalid server credentials fail before a usable server/listener exists;
- zero/over-bound socket timeout configuration fails closed;
- wrong ALPN fails closed;
- TLS 1.2-only fallback remains unavailable;
- plaintext fallback remains unavailable.

Disposable fixture credentials remain test-only evidence and are not production credential authority.

## 10. Explicit non-materializations

C03e-CI does not materialize:
- bridge accept loop ownership;
- BY request-ID lifecycle source;
- PRWA transaction execution;
- session-auth service composition;
- registry composition;
- requester/rendezvous provider;
- candidate-publication command execution;
- authenticated-session product wiring;
- persistent listener/service process;
- production credential storage;
- networking activation;
- deployment/restart/recovery;
- rebase or merge.

## 11. Closure requirements

C03e-CI may close only if:
- predecessor remains exact closed CH head;
- final compare is ahead-only with exact CH merge-base;
- final net diff contains exactly the three authorized paths and no fourth path;
- control-transport Cargo manifest is byte-stable;
- root and Android lock blobs remain exact;
- canonical exact-head Rust validation is FULL PASS;
- any automatically triggered Android validation is reported exactly by its terminal result;
- skipped workflows are reported only as skipped;
- immutable Drive audit is raw-readback verified;
- rolling `C02E_BRANCH_STATUS.md` is append-only with fresh predecessor hash guard;
- candidate PR remains draft/open/unmerged.

## 12. Completion gate and next boundary

On successful closure:

`C03E_CI_PRWC_GENERIC_SERVER_ACCEPTED_STREAM_SOURCE_MATERIALIZED`

The next checkpoint is C03e-CJ, but only after a fresh post-CI source audit confirms the
exact current bridge root and selected BY request-ID representation scope.

C03e-CI itself authorizes no live listener activation or production runtime wiring.
