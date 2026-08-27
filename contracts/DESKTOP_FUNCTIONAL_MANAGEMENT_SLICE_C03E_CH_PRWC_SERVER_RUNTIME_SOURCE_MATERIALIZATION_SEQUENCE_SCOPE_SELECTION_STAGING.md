# Phase 152 C03e-CH — PRWC Server Runtime Source-Materialization Sequence / Scope Selection

Status: `STAGED SELECTION — PENDING EXACT-HEAD VALIDATION`

Target gate:
`C03E_CH_PRWC_SERVER_RUNTIME_SOURCE_MATERIALIZATION_SEQUENCE_SCOPE_SELECTED`

## 1. Exact predecessor

C03e-CH is the direct docs-only successor of closed C03e-CG:

- branch `phase-152-c03e-cg-prwc-generic-server-accepted-stream-bridge-runtime-execution-source-selection-staging`;
- head `647975e8e6baf209a2ae11cc88044b8c774765ef`;
- tree `8df756aaa175574cdc6b60f0b056715c777fd3d6`;
- gate `C03E_CG_PRWC_GENERIC_SERVER_ACCEPTED_STREAM_BRIDGE_RUNTIME_EXECUTION_SOURCE_SELECTED`;
- PR #204 remains draft/open/unmerged with body `Status: CLOSED`.

C03e-CG selected the generic server/listener/accepted-stream and bridge connection-execution authority boundaries. It deliberately required a separately gated audit-first source-materialization plan before Rust source mutation.

## 2. Goal

C03e-CH selects the smallest safe source-materialization sequence and exact first-source scopes. It does not materialize Rust source itself.

The selected objective is to avoid a single coupled commit that simultaneously introduces:
- generic TCP/TLS server primitives;
- bridge request-ID state;
- bridge authentication/frame-loop execution;
- requester/rendezvous composition;
- candidate-publication execution.

Each independently testable authority must be materialized and closed before the next layer is allowed to depend on it.

## 3. Exact audit basis

At exact closed C03e-CG head:

- `crates/prw-control-transport/src/lib.rs` blob `34b0a898572adaa2f77251ca2e9c66ea29973e95`;
- `crates/prw-control-transport/Cargo.toml` blob `ea303d260bf3a1bac9266f72fbdc95bc7a9a4fd0`;
- `crates/prw-control-transport/tests/tls_loopback.rs` blob `eedebde337e0f90c920bd92740a57aa8f068a466`;
- `crates/prw-remote-bridge/Cargo.toml` blob `5fd48263be415aac28dee1c71a4031a4a02ad36c`;
- `crates/prw-remote-bridge/src/root.rs` blob `8fdc1f30d6be12e55e0cfa0c7624810e60466b99`;
- `crates/prw-remote-bridge/src/control_session_auth_wire.rs` blob `77c6f401ef73c0b2a97645ae8bc83524c769a905`;
- `crates/prw-session/src/lib.rs` blob `0b0b6624df93ebcf3efae632d94dfc337ee67761`;
- `crates/prw-session/src/prwa_verifier_source.rs` blob `e34c3d452b9fd5c9787abbf1f36106e3b97e3b0b`;
- `crates/prw-registry/src/lib.rs` blob `cd7eb52eea19354620ff8ad26c9b8aad3f9c5eb6`.

Root `Cargo.lock` is `eeacde7ee776d35088f746a6d09f823f3391b82b`.
Android native `Cargo.lock` is `cce9ca06190a196661ab38d54a747893e26af95f`.

## 4. Dependency audit result

The first two source units require no new dependency selection:

### Generic Phase 129 server primitive

`prw-control-transport` already has production:

`rustls = 0.23.43`, `default-features = false`, features `std` + `aws_lc_rs`.

The existing loopback test already compiles and exercises `ServerConfig`, `ServerConnection`, `TcpListener`, the aws-lc provider, TLS 1.3, exact `CONTROL_ALPN`, certificate/key parsing and no-client-auth behavior.

Therefore a generic production server primitive can be materialized without changing `crates/prw-control-transport/Cargo.toml`, root `Cargo.lock`, or Android native `Cargo.lock`.

### BY request-ID lifecycle source

The BY-selected state requires only standard-library integer/counter plus bounded in-memory collection semantics. No new crate dependency, clock, RNG, persistence, socket or async runtime is needed.

Therefore request-ID lifecycle source can be materialized in `prw-remote-bridge` without changing its Cargo manifest or either lockfile.

## 5. Selected source-materialization order

C03e-CH selects this order:

1. **C03e-CI — generic PRWC server/listener/accepted-stream primitive materialization**;
2. **C03e-CJ — bridge-owned BY request-ID lifecycle source materialization**;
3. **C03e-CK — bridge PRWC authentication connection-execution source materialization**, only after a fresh post-CI/CJ source audit;
4. candidate-publication execution composition remains later and separately gated.

No later unit may be folded backward into an earlier one merely because its dependencies become available.

## 6. C03e-CI exact authorized source scope

The first source checkpoint C03e-CI is authorized, subject to fresh exact-head race guards, to change exactly these three paths and no others:

1. `crates/prw-control-transport/src/lib.rs`;
2. `crates/prw-control-transport/tests/tls_loopback.rs`;
3. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CI_PRWC_GENERIC_SERVER_ACCEPTED_STREAM_SOURCE_MATERIALIZATION_STAGING.md`.

Any fourth path requires a fresh scope audit before mutation.

Explicitly excluded from C03e-CI:
- `crates/prw-control-transport/Cargo.toml`;
- all Cargo lockfiles;
- all `prw-remote-bridge` source;
- Agent/Desktop/Android source;
- workflows;
- credential fixture replacement;
- deployment/network configuration.

## 7. C03e-CI exact semantic implementation target

C03e-CI may materialize only the generic CG-selected transport primitive:

- one server configuration type/builder from caller-supplied certificate chain/private-key material;
- TLS 1.3 only under the existing aws-lc rustls provider;
- exact `CONTROL_ALPN` only;
- early-data/protocol fallback disabled as applicable to the server profile;
- no client-certificate authentication in the initial profile;
- explicit bind `SocketAddr`;
- a generic listener value that performs no background accept loop;
- one explicit `accept(...)` operation;
- accepted socket read/write timeouts validated under existing non-zero `MAX_CONTROL_TIMEOUT` bound;
- TCP no-delay;
- completed `ServerConnection` handshake before accepted-stream return;
- exact TLS1.3 and ALPN verification before return;
- one server-side accepted PRWC stream type exposing the existing bounded `read_frame(...)` / `write_frame(...)` codec.

The primitive must not know PRWA, DeviceId, SessionId, requester/rendezvous state, PRWP, freshness, candidate publication or capability semantics.

## 8. C03e-CI focused test requirements

C03e-CI must reuse the existing disposable loopback fixture authority and add focused coverage for at least:

- exact TLS1.3 + expected ALPN client/server establishment through the new production server primitive;
- server-side accepted stream can read/write existing bounded `ControlFrame` values;
- wrong ALPN fails closed;
- TLS-version fallback remains unavailable;
- invalid/empty certificate or key material fails before a usable server config/listener exists;
- zero or over-bound accepted-socket timeout configuration fails closed;
- no test may promote disposable fixture credentials into production credential authority.

Tests may use loopback sockets only. No externally reachable bind is selected.

## 9. C03e-CI required negative guarantees

C03e-CI must not:
- spawn a persistent/background accept loop;
- start PRWA authentication;
- own BY request IDs;
- bind an authenticated logical session;
- call registry/session services;
- execute candidate publication;
- inject requester/rendezvous authority;
- discover/provision/store production credentials;
- wire product surfaces;
- activate production networking or deployment.

## 10. C03e-CJ exact authorized source scope

After C03e-CI closes, C03e-CJ is selected as a separate pure source checkpoint with exactly these intended paths, subject to a fresh exact-head scope audit before mutation:

1. `crates/prw-remote-bridge/src/prwc_request_id_lifecycle.rs`;
2. `crates/prw-remote-bridge/src/root.rs`;
3. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CJ_PRWC_REQUEST_ID_LIFECYCLE_SOURCE_MATERIALIZATION_STAGING.md`.

C03e-CJ may not change `Cargo.toml`, lockfiles, control-transport source, network runtime source or product source unless a new audit proves the intended pure boundary cannot be represented as selected.

## 11. C03e-CJ exact semantic implementation target

C03e-CJ must implement only C03e-BY state:

- one fresh connection-local namespace starts at next allocatable ID `1`;
- zero forbidden;
- strictly monotonic increment;
- no same-connection reuse after completion;
- wrap/exhaustion fails closed;
- maximum 64 simultaneously outstanding locally originated requests;
- allocation atomically reserves an ID;
- fail before mutation at the 64-outstanding bound;
- terminal completion succeeds exactly once only for an outstanding ID;
- unknown/non-outstanding completion fails closed;
- completion removes outstanding state but never makes the ID reusable;
- abandon-all on connection discard/shutdown clears outstanding state without successful completion;
- focused read-only state introspection only as needed for tests.

No clock, timeout, retry, cancellation, RNG, socket, auth, routing or persistence state belongs in this module.

## 12. C03e-CK remains separately re-audited

C03e-CH intentionally does not pre-authorize an exact C03e-CK path list.

Reason: bridge connection execution will depend on the exact public APIs produced by C03e-CI and C03e-CJ. A path list selected before those APIs exist could either force unnecessary churn or hide an additional representation/provider requirement.

After CI and CJ close, a fresh audit must verify:
- exact accepted-stream type/API;
- exact request-ID lifecycle type/API;
- current PRWA codec and verifier-source APIs;
- current session-auth/registry APIs;
- whether requester/rendezvous needs only an abstract port/type declaration or a separately selected representation checkpoint;
- exact test seam required to validate failure cleanup without activating production networking.

Only then may C03e-CK source paths be authorized.

## 13. Candidate-publication execution remains excluded

Neither CI, CJ nor the later CK authentication connection loop may silently execute candidate publication.

After successful PRWA terminal delivery, Command admission may become possible, but PRWP decode + publisher currentness + TransportIdentity currentness + requester/rendezvous authority + freshness/current-plan validation + durable compare-and-commit remain a separate later execution checkpoint.

## 14. Validation policy for source checkpoints

Every source checkpoint selected here must:

- start from the exact closed predecessor head;
- build its candidate audit-first;
- prove exact path scope before branch movement;
- preserve all non-authorized lock/manifest paths byte-for-byte;
- use exact-head canonical Rust validation;
- use Android validation only when automatically triggered or when the changed dependency surface makes it applicable;
- report skipped workflows only as skipped;
- close with immutable Drive audit + raw readback + guarded append-only rolling evidence;
- leave its PR draft/open/unmerged.

No closure may rely on a predecessor or superseded corrective head's CI verdict.

## 15. C03e-CH exact repository scope

C03e-CH itself is limited to exactly one docs-only path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CH_PRWC_SERVER_RUNTIME_SOURCE_MATERIALIZATION_SEQUENCE_SCOPE_SELECTION_STAGING.md`

Any second changed path requires a fresh scope audit.

Root and Android Cargo locks, all manifests, source and workflows remain byte-stable in CH.

## 16. Explicit non-materializations

C03e-CH performs no:
- Rust/Kotlin source mutation;
- listener/server/accepted-stream implementation;
- request-ID implementation;
- frame loop/authentication execution;
- requester/rendezvous provider implementation;
- candidate-publication execution;
- credential provisioning/storage;
- product runtime wiring;
- provider/database mutation;
- networking activation;
- deployment/restart/recovery;
- rebase/merge.

## 17. Completion gate

After exact-head validation and evidence closure:

`C03E_CH_PRWC_SERVER_RUNTIME_SOURCE_MATERIALIZATION_SEQUENCE_SCOPE_SELECTED`

The next authorized source checkpoint is C03e-CI with exactly the three-path scope in section 6. No other source materialization is authorized by CH.
