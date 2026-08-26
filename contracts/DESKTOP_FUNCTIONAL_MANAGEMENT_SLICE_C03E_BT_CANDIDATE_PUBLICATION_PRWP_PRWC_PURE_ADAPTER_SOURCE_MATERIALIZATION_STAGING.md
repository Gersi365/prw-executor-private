# Phase 152 C03e-BT — Candidate Publication PRWP↔PRWC Pure Adapter Source Materialization

Status: STAGED SOURCE MATERIALIZATION

Gate target:
`C03E_BT_CANDIDATE_PUBLICATION_PRWP_PRWC_PURE_ADAPTER_SOURCE_MATERIALIZED`

## 1. Exact predecessor

Closed C03e-BS:
- branch: `phase-152-c03e-bs-candidate-publication-prwp-prwc-adapter-ownership-dependency-selection-staging`;
- head: `6e4479dc39d18d91277f072075f4ba7f3882af2c`;
- tree: `46610d1968a9ff4ac1dbcb2e6791b12441972039`;
- gate: `C03E_BS_CANDIDATE_PUBLICATION_PRWP_PRWC_ADAPTER_OWNERSHIP_DEPENDENCY_SELECTED`;
- PR `#188`: `Status: CLOSED`, draft/open/unmerged.

BS selected only `prw-remote-bridge` as owner of the future pure adapter and selected the dependency direction `prw-remote-bridge -> prw-control-transport` while keeping generic transport free of candidate-publication semantics.

## 2. Exact bounded purpose

BT materializes only the BS-selected pure in-memory composition between:
- existing BQ `CandidatePublicationWireSubmission` / canonical `PRWP` v1.0 bytes; and
- existing Phase 129 `ControlFrame` using BR-selected `ControlMessageKind::Command`.

BT performs no stream I/O, TLS connection work, request-ID allocation, authentication, routing, publication admission, freshness rotation, reachability mutation, networking activation, deployment or merge.

## 3. Exact authorized paths

BT is authorized to change exactly these four paths:
1. `crates/prw-remote-bridge/Cargo.toml` — add direct path dependency on `prw-control-transport`;
2. `crates/prw-remote-bridge/src/root.rs` — export the pure adapter module;
3. `crates/prw-remote-bridge/src/candidate_publication_control_frame.rs` — pure adapter plus focused in-memory tests;
4. this contract.

Any required fifth product/source path is a stop-and-re-audit condition. In particular, BT does not silently add `Cargo.lock`; canonical locked-dependency validation is authoritative for whether the four-path materialization is viable.

## 4. Exact adapter semantics

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

## 5. Request correlation boundary

The adapter accepts/preserves a caller-supplied outer request ID but never allocates one. PRWC `request_id` remains correlation only and is not authentication, authorization, routing identity, freshness evidence, candidate identity, liveness or readiness.

Still unselected: allocator custody, uniqueness/reuse/persistence/restart semantics, request lifecycle table, response matching, SessionId custody, authenticated session wiring and requester/recipient routing.

## 6. Generic transport byte-stability

BT must not modify:
- `crates/prw-control-transport/src/lib.rs`;
- `crates/prw-control-transport/Cargo.toml`;
- Phase 129 message-kind codes, PRWC codec, TLS implementation or frame limits.

The adapter consumes those existing generic types from the higher bridge layer.

## 7. Inner codec byte-stability

BT must not modify BQ `crates/prw-remote-bridge/src/candidate_publication_wire.rs` absent a separately proven defect. The adapter calls the existing codec rather than cloning its format rules.

## 8. Focused tests

BT adapter tests are in-memory only and cover:
- Command kind construction;
- preservation of supplied non-zero outer request ID and exact PRWP payload;
- decode preservation of typed submission and correlation;
- wrong outer kind rejection;
- malformed PRWP classification;
- zero outer request-ID rejection through existing `ControlFrame::new(...)` validation.

Tests open no socket.

## 9. Explicit non-materializations

BT does not materialize:
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

## 10. Validation and closure

BT may close only after:
- exact BS predecessor lineage remains unchanged;
- exact BS→BT final compare contains only the four authorized paths;
- canonical Rust validation on the exact final head reaches terminal success including locked dependency graph, formatting, Clippy, tests and workspace build;
- Android validation, if automatically triggered, reaches terminal success including native adapter and application validation;
- every other automatically triggered workflow reaches a terminal non-failing verdict;
- immutable Drive audit is uploaded inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-readback verified;
- rolling Drive evidence passes a fresh predecessor guard, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

If canonical locked-dependency validation proves `Cargo.lock` must change, BT must remain unclosed and selection must be re-audited rather than broadening to a fifth path.

No production activation is authorized by BT closure.
