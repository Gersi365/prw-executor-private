# Phase 152 C03e-BV — Candidate Publication PRWP↔PRWC Pure Adapter Source Materialization

Status: STAGED SOURCE MATERIALIZATION

Gate target:
`C03E_BV_CANDIDATE_PUBLICATION_PRWP_PRWC_PURE_ADAPTER_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Closed C03e-BU is the authoritative predecessor:
- branch: `phase-152-c03e-bu-candidate-publication-prwp-prwc-lockfile-scope-reselection-staging`;
- head: `4de16f726e3a513311f2897747b40c73aeafdf2f`;
- tree: `dc8b1e03419ff8f0591282c1906797023c711ec0`;
- gate: `C03E_BU_CANDIDATE_PUBLICATION_PRWP_PRWC_LOCKFILE_SCOPE_RESELECTED`;
- PR `#190`: `Status: CLOSED`, draft/open/unmerged.

BU re-selected the corrected source-materialization scope after canonical locked validation proved that both root and Android-native lockfiles are required. BV does not change the BS/BR/BQ architecture or semantics.

The blocked C03e-BT branch remains evidence only and is not BV's predecessor.

## 2. Exact bounded purpose

BV materializes only the already-selected pure in-memory composition between:
- existing BQ `CandidatePublicationWireSubmission` / canonical `PRWP` v1.0 bytes; and
- existing Phase 129 `ControlFrame` using BR-selected `ControlMessageKind::Command`.

BV performs no stream I/O, TLS connection work, request-ID allocation, authentication, routing, publication admission, freshness rotation, reachability mutation, networking activation, deployment or merge.

## 3. Exact authorized paths

BV is authorized to change exactly these six final-tree paths relative to BU:
1. `crates/prw-remote-bridge/Cargo.toml` — add the BS-selected direct path dependency on `prw-control-transport`;
2. `Cargo.lock` — materialize only the corresponding root workspace bridge dependency edge required by canonical `--locked` validation;
3. `apps/android/native/Cargo.lock` — materialize only the existing `prw-control-transport` path package plus the bridge dependency edge required by canonical `--locked` validation;
4. `crates/prw-remote-bridge/src/root.rs` — export the pure adapter module;
5. `crates/prw-remote-bridge/src/candidate_publication_control_frame.rs` — pure adapter plus focused in-memory tests;
6. this contract.

Any seventh final changed path is a stop-and-re-audit condition.

Temporary validation mechanics, if needed to obtain Cargo-generated lock state, must be removed before the final compare and may not alter product semantics.

## 4. Source provenance

The three non-lock product source paths reuse the already-reviewed C03e-BT final source bytes rather than redesigning the adapter:
- `crates/prw-remote-bridge/Cargo.toml`;
- `crates/prw-remote-bridge/src/root.rs`;
- `crates/prw-remote-bridge/src/candidate_publication_control_frame.rs`.

BT remains blocked only because its four-path scope omitted lockfiles proven necessary by canonical `--locked` validation. BV corrects that scope defect without broadening adapter semantics.

## 5. Exact adapter semantics

Encode side:
```text
already-typed CandidatePublicationWireSubmission
    -> BQ encode()
    + caller-supplied non-zero outer request_id
    -> ControlFrame::new(ControlMessageKind::Command, request_id, exact PRWP bytes)
```

Decode side:
```text
already-decoded ControlFrame
    -> require ControlMessageKind::Command
    -> preserve frame.request_id() as outer correlation metadata only
    -> CandidatePublicationWireSubmission::decode(frame.payload())
```

Successful decode proves only outer-kind correctness plus BQ bounded structural/type decoding.

## 6. Request correlation boundary

The adapter accepts and preserves a caller-supplied outer request ID but never allocates one. PRWC `request_id` remains correlation only and is not authentication, authorization, routing identity, freshness evidence, candidate identity, liveness or readiness.

Still unselected:
- request-ID allocator/custody;
- uniqueness/reuse/persistence/restart semantics;
- request lifecycle table;
- response matching;
- SessionId custody;
- authenticated session wiring;
- requester/recipient routing;
- expected-device scheduling provenance.

## 7. Root lockfile constraint

BU proved that root `Cargo.lock` already contains the workspace package `prw-control-transport` and that the only required root semantic delta is the direct edge under the `prw-remote-bridge` package:

```text
 "prw-control-plane",
+"prw-control-transport",
 "prw-core",
```

BV does not authorize any root crate version, checksum, registry source or unrelated dependency-edge change.

## 8. Android-native lockfile constraint

BU proved that Android-native lock state requires exactly:
- one `prw-control-transport` path package entry whose existing dependency is `rustls`; and
- one `prw-control-transport` dependency edge under the existing `prw-remote-bridge` package.

No unrelated Android-native crate version, checksum, registry source or dependency edge may change.

## 9. Generic transport byte-stability

BV must not modify:
- `crates/prw-control-transport/src/lib.rs`;
- `crates/prw-control-transport/Cargo.toml`;
- Phase 129 message-kind codes, PRWC codec, TLS implementation or frame limits.

The adapter consumes those existing generic types from the higher bridge layer.

## 10. Inner codec byte-stability

BV must not modify `crates/prw-remote-bridge/src/candidate_publication_wire.rs` absent a separately proven defect. The adapter calls the existing BQ codec rather than cloning its format rules.

## 11. Focused tests

BV adapter tests remain in-memory only and cover:
- Command kind construction;
- preservation of supplied non-zero outer request ID and exact PRWP payload;
- decode preservation of typed submission and correlation;
- wrong outer kind rejection;
- malformed PRWP classification;
- zero outer request-ID rejection through existing `ControlFrame::new(...)` validation.

Tests open no socket.

## 12. Explicit non-materializations

BV does not materialize:
- a new control message kind;
- changes to PRWC codec/TLS/frame limits;
- request-ID allocator/custody;
- response/ack/error protocol;
- logical control-plane auth wire composition or session store;
- requester/recipient routing or expected-device scheduling;
- broker/dispatcher/listener/acceptor;
- TCP/TLS connect or frame read/write execution;
- retries/idempotency/deduplication;
- candidate-ID allocation, path-kind classification or endpoint discovery;
- publication admission/freshness rotation/reachability commit;
- registry/provider/database mutation;
- STUN/ICE/TURN/relay/QUIC activation;
- Agent/Desktop/Android runtime wiring;
- host/network configuration mutation;
- deployment/restart/recovery;
- merge.

## 13. Exact no-churn proof before closure

Before closure, exact BU→BV diff inspection must prove:
- exactly six final changed paths and no seventh path;
- root `Cargo.lock` changed only by the required bridge dependency edge;
- Android-native `Cargo.lock` changed only by the required path-package entry and bridge dependency edge;
- no unrelated crate version, checksum, source or dependency edge changed;
- generic transport and BQ inner codec remain byte-stable.

Any contradiction blocks closure.

## 14. Validation and closure

BV may close only after:
- exact BU predecessor lineage remains unchanged;
- final BU→BV compare contains exactly the six authorized paths;
- canonical Rust validation on the exact final head reaches terminal success including locked dependency graph, formatting, Clippy, tests and workspace build;
- Android validation reaches terminal success including native adapter and Android application validation if triggered;
- every other automatically triggered workflow reaches a terminal non-failing verdict;
- immutable Drive audit is uploaded inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-readback verified;
- rolling Drive evidence passes a fresh predecessor guard, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No production activation is authorized by BV closure.
