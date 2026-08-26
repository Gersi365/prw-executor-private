# Phase 152 C03e-CC — PRWC Pre-Mesh Authentication Codec Lockfile Scope Re-selection

Status: STAGED SELECTION

Gate target:
`C03E_CC_PRWC_PRE_MESH_AUTHENTICATION_CODEC_LOCKFILE_SCOPE_RESELECTED`

## 1. Exact closed predecessor

Closed C03e-CA is the authoritative predecessor:
- branch: `phase-152-c03e-ca-prwc-pre-mesh-authentication-wire-transaction-selection-staging`;
- head: `ed4d891b5e5b6f87f01526397982de4fd643afba`;
- tree: `70fc2b3d27adfcf11ba03ba99a3e35cabd2eb6f9`;
- gate: `C03E_CA_PRWC_PRE_MESH_AUTHENTICATION_WIRE_TRANSACTION_SELECTED`;
- PR #196: body `Status: CLOSED`, draft/open/unmerged.

CA remains the last closed authority. Blocked C03e-CB is evidence only and is not an ancestor of CC.

## 2. Blocked-CB evidence requiring re-selection

C03e-CB attempted CA's four-path pure-codec materialization and is frozen at:
- branch: `phase-152-c03e-cb-prwc-pre-mesh-authentication-pure-codec-source-materialization-staging`;
- final blocked head: `fa670d799bfd8cfe5b380a033d38a5f78cd58f87`;
- final tree: `1c32f3e866b28e953972c1305388b8e943f74e11`;
- PR #197: `Status: STAGED — BLOCKED FOR LOCKFILE SCOPE RE-AUDIT`, draft/open/unmerged.

At that exact head:
- Rust #1145 / run `33006752983` / job `98302377128` is FULL PASS;
- Android #842 / run `33006752962` / job `98302376760` fails in native validation because `apps/android/native/Cargo.lock` requires update under `--locked`;
- Android application validation is skipped;
- AD #398 and AE #389 are skipped.

The immutable blocker audit is:
`C03E_CB_PRWC_PRE_MESH_AUTHENTICATION_PURE_CODEC_SOURCE_MATERIALIZATION_BLOCKED_AUDIT.md`
- Drive ID `1KElGa4dYEBdvV_VM8THSU-xsY1lP1zlm`;
- exact raw size 8517 bytes;
- SHA-256 `ef3cbdfc569c09a9b674a615e7fa9edf950978b53f8090553c65c05909602999`;
- raw readback exact PASS.

No rolling closure evidence was appended for CB.

## 3. Purpose of CC

CC corrects only the source-materialization path scope discovered by canonical Android validation.

CC does not alter CA's PRWA protocol, typed values, operation mapping, request-ID semantics, authentication authority, failure model, payload bounds, or non-selections.

CC is docs-only. It does not materialize Rust source, a lockfile, runtime logic, listener/socket I/O, session-service calls, registry calls, requester/rendezvous storage, candidate-publication execution, networking, deployment, or merge.

## 4. PRWA semantics remain exactly CA

The corrected successor must preserve CA exactly:
- inner magic `PRWA`;
- version 1.0;
- 12-byte inner header;
- operations Begin=1, Challenge=2, Proof=3, Authenticated=4, Rejected=5;
- outer `Authentication` for Begin/Challenge/Proof;
- outer `Response` for Authenticated;
- outer `Error` for Rejected;
- one caller-supplied non-zero BY-managed PRWC request ID spans the transaction and is not duplicated in PRWA;
- Begin carries only one untrusted typed `DeviceId` selector;
- Challenge carries verifier-provided typed `SessionId`, exact 32-byte nonce, issue time and expiry time with lifetime >0 and <=300 seconds;
- Proof carries typed `SessionId`, exact nonce, locked P-256/SHA-256 + DER profile tags, and 1..256 signature bytes;
- Authenticated carries only the completed typed `SessionId`;
- Rejected remains generic and detail-free externally;
- successful decode remains structural/type validity only and is not authentication success.

Existing `SessionAuthenticationService` semantics and current registry revalidation remain separately authoritative.

## 5. Root lockfile remains byte-stable

Root `Cargo.lock` is not part of the corrected source scope.

Blocked CB's final Rust #1145 passed its locked dependency-graph step while using root lock blob:
`eeacde7ee776d35088f746a6d09f823f3391b82b`.

Therefore the corrected successor must preserve root `Cargo.lock` byte-for-byte. Any root-lock modification is outside CC's selected scope and blocks successor closure pending a new contradiction.

## 6. Exact Android native lockfile correction

Current authoritative Android-native lock before corrected materialization:
- path: `apps/android/native/Cargo.lock`;
- blob: `56137987fa58c62c314ebba1e27e36d3811a5650`.

That lock already contains the workspace package:

```text
[[package]]
name = "prw-core"
version = "0.1.0"
```

Its existing `prw-remote-bridge` package block contains the other current production bridge dependencies but lacks `"prw-core"`.

The corrected lock semantic delta is exactly one dependency edge inside the existing `prw-remote-bridge` dependency list:

```text
 "prw-core",
```

No new package record, package version, registry source, checksum, feature resolution, or unrelated dependency edge is selected.

Before corrected source closure, exact diff review must prove that the Android lockfile changed only by this one semantic edge. Any unrelated lock churn blocks closure.

## 7. Corrected source-materialization scope

A future corrected pure-codec source-materialization checkpoint is authorized to change exactly these five paths relative to closed CC:

1. `crates/prw-remote-bridge/Cargo.toml`
2. `apps/android/native/Cargo.lock`
3. `crates/prw-remote-bridge/src/root.rs`
4. `crates/prw-remote-bridge/src/control_session_auth_wire.rs`
5. one exact successor source-materialization contract

No sixth path is authorized.

The intended manifest delta remains only promotion of existing workspace `prw-core` from bridge dev-dependency to bridge production dependency. No new third-party dependency is authorized.

## 8. Corrected codec boundary

The future codec source remains pure in-memory only. It may:
- define bounded typed PRWA messages;
- encode them into existing Phase-129 `ControlFrame` values;
- decode existing `ControlFrame` values into typed PRWA values;
- validate CA-selected header/version/flags/opcode/outer-kind/bounds/UTF-8/nonce/lifetime/signature-profile/trailing-data rules;
- preserve the supplied request ID without allocating or owning it;
- include focused in-memory tests and compile-time payload-ceiling assertions.

It must not:
- allocate request IDs;
- allocate SessionIds or challenge nonces;
- invoke `SessionAuthenticationService`;
- verify signatures cryptographically;
- read or mutate registry state;
- establish authenticated connection state;
- perform socket/frame-loop/network I/O;
- own a listener or accepted connection;
- resolve requester/rendezvous authority;
- execute candidate publication;
- activate Agent/Desktop/Android runtime behavior;
- mutate deployment/network/system state.

## 9. Byte-stability requirements

Absent a new concrete compiler/validator contradiction, corrected materialization must preserve at least:
- root `Cargo.lock` at blob `eeacde7ee776d35088f746a6d09f823f3391b82b`;
- `crates/prw-control-transport/src/lib.rs`;
- `crates/prw-control-plane/src/session_auth.rs`;
- `crates/prw-session/src/lib.rs`;
- `crates/prw-registry/src/lib.rs`;
- existing PRWM `crates/prw-remote-bridge/src/session_auth_wire.rs`;
- candidate-publication source;
- workflows;
- Agent/Desktop/Android application source.

Only the selected Android-native lock edge may represent lockfile churn.

## 10. Explicit non-selections

CC does not select or materialize:
- PRWA transaction runtime execution;
- challenge/SessionId source or custody;
- request-ID allocator implementation;
- authentication retry/reauthentication;
- authentication timeout policy;
- session-store persistence;
- listener/server socket ownership;
- accepted-stream/frame-loop behavior;
- detailed authentication error oracle;
- requester/rendezvous provider representation;
- candidate-publication execution;
- Agent/Desktop/Android application wiring;
- STUN/ICE/TURN/relay/QUIC production activation;
- host/systemd/firewall/NAT/route/DNS/TUN/TAP changes;
- deployment/restart/recovery;
- merge.

## 11. Safe successor rule

After CC closure, the next safe checkpoint is a corrected pure in-memory PRWA codec source-materialization checkpoint from exact closed CC using exactly the five paths in section 7.

That successor must incorporate only the one Android-native lock dependency edge selected in section 6 and must prove root-lock stability.

Blocked CB must remain untouched as evidence and must not become an ancestor of corrected source work.

## 12. Exact CC path scope

CC itself is docs-only and may contain exactly one changed path:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_CC_PRWC_PRE_MESH_AUTHENTICATION_CODEC_LOCKFILE_SCOPE_RESELECTION_STAGING.md`

Any Rust/Kotlin source, manifest, lockfile, workflow, runtime, networking, provider, application, or deployment path blocks CC closure.

## 13. Validation and closure

CC may close only after:
- exact closed CA lineage remains unchanged;
- exact CA→CC compare contains one docs-only path;
- blocked-CB evidence remains unchanged;
- every automatically triggered CC workflow reaches terminal non-failing verdict;
- immutable Drive audit is uploaded under project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-readback verified;
- rolling Drive predecessor is still the exact closed CA post-state, with append-only predecessor-prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No source/runtime/networking/deployment mutation is authorized by CC closure.
