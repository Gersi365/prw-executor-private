# Phase 152 C03e-CB — PRWC Pre-Mesh Authentication Pure Codec Source Materialization

Status: STAGED SOURCE MATERIALIZATION

Gate target:
`C03E_CB_PRWC_PRE_MESH_AUTHENTICATION_PURE_CODEC_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Closed C03e-CA is the exact authoritative predecessor:
- branch `phase-152-c03e-ca-prwc-pre-mesh-authentication-wire-transaction-selection-staging`;
- head `ed4d891b5e5b6f87f01526397982de4fd643afba`;
- tree `70fc2b3d27adfcf11ba03ba99a3e35cabd2eb6f9`;
- gate `C03E_CA_PRWC_PRE_MESH_AUTHENTICATION_WIRE_TRANSACTION_SELECTED`;
- PR #196 body `Status: CLOSED`, draft/open/unmerged.

CA selected the complete bounded PRWA v1.0 byte schema, exact operation/outer-kind pairing, typed bounds, one-request-ID transaction semantics, generic external rejection, and fail-closed decode requirements. CA explicitly authorized one pure in-memory codec materialization successor with a four-path maximum and no runtime execution.

## 2. Exact bounded purpose

CB materializes only the CA-selected **pure in-memory PRWA codec** inside `prw-remote-bridge`.

The source boundary is:

```text
already-typed PRWA semantic value + caller-supplied non-zero request_id
    -> pure bounded PRWA encode
    -> existing Phase 129 ControlFrame::new(...)

already-decoded Phase 129 ControlFrame
    -> strict PRWA header/body/outer-kind validation
    -> existing typed DeviceId / SessionId / SessionAuthNonce / DeviceIdentitySignature values
```

Successful decode proves structural/type validity only. CB performs no authentication verification and creates no `AuthenticatedDeviceSession`.

## 3. Exact changed-path authorization

Relative to exact CA, CB may change exactly these four paths and no others:

1. `crates/prw-remote-bridge/Cargo.toml`
2. `crates/prw-remote-bridge/src/root.rs`
3. `crates/prw-remote-bridge/src/control_session_auth_wire.rs`
4. `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CB_PRWC_PRE_MESH_AUTHENTICATION_PURE_CODEC_SOURCE_MATERIALIZATION_STAGING.md`

Any fifth product/source path, root `Cargo.lock`, Android native lockfile, workflow, Agent/Desktop/Android implementation, provider/database, networking configuration, packaging/systemd, deployment, or host path blocks CB closure and requires re-audit.

## 4. Dependency promotion only

`crates/prw-remote-bridge/Cargo.toml` promotes the already-workspace-local `prw-core` path dependency from dev-only use into normal production dependencies because the pure codec publicly represents existing typed `DeviceId` and `SessionId` values.

Exact intended manifest change:
- add `prw-core = { path = "../prw-core" }` under `[dependencies]`;
- remove the duplicate `prw-core` line from `[dev-dependencies]`;
- make no other dependency or feature change.

No third-party dependency, version, lockfile, feature, runtime, crypto backend, or networking dependency is added.

## 5. Root export only

`crates/prw-remote-bridge/src/root.rs` exports exactly one new module:

`pub mod control_session_auth_wire;`

No existing module ordering or runtime activation semantics are changed beyond this source visibility.

## 6. Materialized PRWA constants

`control_session_auth_wire.rs` materializes the CA-selected constants:
- magic `PRWA`;
- major/minor `1/0`;
- fixed header 12 bytes;
- operation codes Begin=1, Challenge=2, Proof=3, Authenticated=4, Rejected=5;
- signature profile tags algorithm=1 and DER encoding=1;
- max signature 256 bytes;
- exact maximum payload constants: Begin 1038, Challenge 1086, Proof 1332, Authenticated 1038, Rejected 12.

Compile-time assertions preserve that every selected maximum fits the existing Phase 129 65,536-byte payload ceiling.

## 7. Typed codec surface

The module exposes one typed enum with exactly five variants:
- `Begin { device_id: DeviceId }`;
- `Challenge { session_id, nonce, issued_at_unix_seconds, expires_at_unix_seconds }`;
- `Proof { session_id, nonce, signature }`;
- `Authenticated { session_id }`;
- `Rejected`.

It exposes pure frame functions:
- `encode_control_session_authentication_frame(request_id, message)`;
- `decode_control_session_authentication_frame(frame)`.

The encoder preserves caller-supplied request ID exactly and chooses the CA-selected outer kind from the typed variant. It does not allocate request IDs.

The decoder receives an already-decoded Phase 129 frame, validates exact PRWA bytes and outer-kind/operation pairing, and returns typed values. It does not read from a stream.

## 8. Exact outer-kind pairing

Materialized mapping:
- Begin → `ControlMessageKind::Authentication`;
- Challenge → `Authentication`;
- Proof → `Authentication`;
- Authenticated → `Response`;
- Rejected → `Error`.

A valid PRWA body in any other outer kind is rejected with the stable codec classification `InvalidOuterKind`.

## 9. Exact field validation

The codec preserves CA bounds:
- identifier byte length `1..=1024`;
- UTF-8 required;
- whitespace-only identifier rejected;
- Begin reconstructs through `DeviceId::new(...)`;
- Challenge/Proof/Authenticated reconstruct through `SessionId::new(...)`;
- nonce exactly 32 bytes via existing `SessionAuthNonce` constructor;
- challenge lifetime positive and at most 300 seconds;
- Proof profile exactly P-256/SHA-256 + DER SigValue tags;
- signature length `1..=256` and reconstruction through existing `DeviceIdentitySignature`;
- reserved header must be zero;
- trailing bytes rejected.

The codec neither accepts nor serializes workspace/user/public-key/TransportIdentity/requester-rendezvous/candidate/freshness fields.

## 10. Error boundary

The pure codec error surface is bounded to:
- `InvalidOuterKind`;
- `InvalidPayload`;
- `UnsupportedSignatureProfile`;
- existing Phase 129 `ControlFrameError` during frame construction.

It does not classify enrollment, membership, registry currentness, cryptographic proof validity, session-service state, policy, requester/rendezvous authority, candidate publication, networking, persistence, or runtime I/O failures.

## 11. Focused in-memory tests

CB includes only unit tests in the new pure codec module. They verify:
- exact Begin golden byte layout;
- exact request-ID preservation;
- all five selected operations round-trip with exact outer-kind pairing;
- wrong outer kind fails closed;
- exact 300-second challenge edge is accepted and 301 seconds rejected;
- maximum 1024-byte identifiers round-trip;
- maximum 256-byte signature round-trips;
- malformed/truncated/trailing bytes fail closed;
- unsupported signature tags fail closed;
- exact maximum payload constants are 1038/1086/1332/1038/12 and fit Phase 129 ceiling.

No socket, async runtime, service, registry, filesystem, network, database, or host behavior appears in the tests.

## 12. Authority separation preserved

CB does not weaken CA/BZ authority separation:
- Begin DeviceId remains an untrusted lookup selector;
- SessionId remains authentication-session correlation only;
- nonce remains verifier challenge/replay input;
- signature remains proof input;
- request ID remains outer one-connection correlation only;
- TLS success remains lower transport authentication only;
- `AuthenticatedDeviceSession` remains produced only by existing semantic proof verification, not by the codec;
- registry currentness remains outside the codec;
- requester/rendezvous authority remains separate server-side state outside PRWA;
- candidate IDs/endpoints/path/freshness remain unrelated candidate-publication state.

## 13. Explicit non-materializations

CB does not call or materialize:
- `SessionAuthenticationService`;
- `WorkspaceDeviceRegistry`;
- challenge generation or SessionId issuance;
- request-ID allocation/custody source;
- listener/server/accepted socket;
- control-frame stream read/write loop;
- authentication transaction orchestration;
- pending-state abort execution;
- authenticated connection/session binding;
- requester/rendezvous provider;
- candidate-publication execution/ack/error protocol;
- capability policy or dispatch;
- Agent/Desktop/Android runtime wiring;
- STUN/ICE/TURN/relay/QUIC mesh activation;
- host/systemd/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## 14. Safe successor rule

After CB closure, runtime authentication execution remains blocked by an explicitly unselected authority: **server-side SessionId issuance/custody/lifecycle for PRWA challenge creation**.

The next safe checkpoint is therefore a docs-only selection of the PRWA pre-mesh SessionId issuance/custody/lifecycle boundary. It must decide ownership, uniqueness scope, restart/reuse rules, lifecycle/abandonment semantics, and how a verifier obtains one typed SessionId without deriving it from DeviceId, request ID, transport identity, nonce, timestamp, or candidate state.

That successor must not yet materialize listener/socket/frame-loop authentication execution, requester/rendezvous provider storage, candidate-publication execution, networking, deployment, or merge.

## 15. Validation and closure

CB may close only after:
- exact closed CA predecessor lineage remains unchanged;
- exact CA→CB compare contains only the four authorized paths;
- root `Cargo.lock` and Android native lockfile remain byte-stable;
- all automatically triggered workflows reach terminal non-failing verdict, including Android if source/Cargo scope triggers it;
- immutable Drive audit is uploaded only under project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-readback verified;
- rolling Drive predecessor guard, append-only prefix proof, and raw post-write verification pass;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No runtime/networking/deployment/merge action is authorized by CB closure.
