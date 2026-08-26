# Phase 152 C03e-BU — Candidate Publication PRWP↔PRWC Lockfile Scope Re-selection

Status: STAGED SELECTION

Gate target:
`C03E_BU_CANDIDATE_PUBLICATION_PRWP_PRWC_LOCKFILE_SCOPE_RESELECTED`

## 1. Exact closed predecessor

Closed C03e-BS remains the authoritative predecessor for re-selection:
- branch: `phase-152-c03e-bs-candidate-publication-prwp-prwc-adapter-ownership-dependency-selection-staging`;
- head: `6e4479dc39d18d91277f072075f4ba7f3882af2c`;
- tree: `46610d1968a9ff4ac1dbcb2e6791b12441972039`;
- gate: `C03E_BS_CANDIDATE_PUBLICATION_PRWP_PRWC_ADAPTER_OWNERSHIP_DEPENDENCY_SELECTED`;
- PR `#188`: CLOSED in project status while remaining draft/open/unmerged.

BS selected the adapter owner and dependency direction only:
- owner: `prw-remote-bridge`;
- dependency direction: `prw-remote-bridge -> prw-control-transport`;
- reverse dependency rejected;
- future pure module: `crates/prw-remote-bridge/src/candidate_publication_control_frame.rs`.

BU does not change that architecture.

## 2. Concrete contradiction requiring re-selection

C03e-BT attempted the BS-authorized four-path source materialization on branch:
`phase-152-c03e-bt-candidate-publication-prwp-prwc-pure-adapter-source-materialization-staging`.

Exact blocked BT head/tree:
- head: `589be8fe25b00b8fe213ed6e797995577eb05ab8`;
- tree: `cfb0248e6663d7644f7d96aa239021040e25c68c`;
- PR `#189`: `Status: STAGED — BLOCKED FOR SCOPE RE-AUDIT`, draft/open/unmerged.

BT intentionally remains unclosed. Its source branch is evidence of the contradiction and is not BU's predecessor.

## 3. Exact canonical blocker evidence

### 3.1 Root workspace

PRW Rust Validation #1133:
- run: `32980984719`;
- job: `98217300651`;
- locked metadata: PASS;
- formatting: PASS after one mechanical rustfmt correction in the already-authorized adapter path;
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: blocked before linting because root `Cargo.lock` requires update while `--locked` forbids it;
- tests/build: skipped.

Exact diagnostic class:
`cannot update the lock file .../Cargo.lock because --locked was passed to prevent this`.

### 3.2 Android native workspace

PRW Android Validation #827:
- run: `32980984932`;
- job: `98217301203`;
- toolchain preparation: PASS;
- native validation: blocked because `apps/android/native/Cargo.lock` requires update while `--locked` forbids it;
- Android application validation: skipped after native failure.

Exact diagnostic class:
`cannot update the lock file .../apps/android/native/Cargo.lock because --locked was passed to prevent this`.

### 3.3 Other exact-head workflows

- C02f-AD #386 / run `32980984762`: skipped;
- C02f-AE #377 / run `32980984947`: skipped.

## 4. Immutable blocked evidence

BT blocked audit is stored only as failure evidence, not closure evidence:
- file: `C03E_BT_CANDIDATE_PUBLICATION_PRWP_PRWC_PURE_ADAPTER_SOURCE_MATERIALIZATION_BLOCKED_AUDIT.md`;
- Drive ID: `1Har5GF-nFVdhu5nMJNa8d0dLV57jWW2Q`;
- project folder: `136SuugnComWa-CRGedjNfphubxleUiDQ`;
- exact raw size: `4496` bytes;
- SHA-256: `670134e82a7cb026d748287602ce934ef74f0dbd39f9eeb9ca2e202670635670`;
- raw Drive readback: exact PASS.

No rolling closure append was written for BT because its gate did not close.

## 5. Exact lockfile audit — root workspace

At the blocked BT head, root `Cargo.lock` remains the BS-era lock content.

The lock already contains the workspace package `prw-control-transport`, because it is an existing workspace member. The existing `prw-remote-bridge` package entry lacks only the newly selected direct dependency edge.

Therefore the expected root lock semantic delta for a corrected source-materialization successor is exactly:

```text
[[package]]
name = "prw-remote-bridge"
...
dependencies = [
    ...
    "prw-control-plane",
+   "prw-control-transport",
    ...
]
```

BU does not authorize unrelated root lockfile re-resolution or dependency upgrades.

## 6. Exact lockfile audit — Android native workspace

At the blocked BT head, `apps/android/native/Cargo.lock` remains the prior Android-native lock content.

Its existing `prw-remote-bridge` package entry lacks `prw-control-transport`, and the lock does not currently contain a `prw-control-transport` package entry.

Because `prw-control-transport/Cargo.toml` has the existing dependency `rustls = =0.23.43` and Android-native already carries `rustls` transitively, the corrected semantic lock delta is bounded to:

```text
[[package]]
name = "prw-control-transport"
version = "0.1.0"
dependencies = [
    "rustls",
]
```

plus:

```text
[[package]]
name = "prw-remote-bridge"
...
dependencies = [
    ...
    "prw-control-plane",
+   "prw-control-transport",
    ...
]
```

BU does not authorize unrelated Android-native lockfile re-resolution or dependency upgrades.

## 7. Corrected source-materialization scope selected by BU

A source-materialization successor to BU is authorized to change exactly these six paths:

1. `crates/prw-remote-bridge/Cargo.toml` — add the BS-selected direct path dependency on `prw-control-transport`;
2. `Cargo.lock` — materialize only the corresponding root workspace lock edge required by canonical `--locked` validation;
3. `apps/android/native/Cargo.lock` — materialize only the corresponding Android-native path-package/bridge-edge lock state required by canonical `--locked` validation;
4. `crates/prw-remote-bridge/src/root.rs` — one pure adapter module export;
5. `crates/prw-remote-bridge/src/candidate_publication_control_frame.rs` — pure in-memory PRWP↔PRWC Command adapter plus focused tests;
6. one exact successor source-materialization contract.

Any seventh changed path is a stop-and-re-audit condition.

## 8. Existing adapter semantics remain selected

The corrected source successor may materialize only the same pure composition already selected by BS/BR/BQ:

Encode:
```text
already-typed CandidatePublicationWireSubmission
    -> BQ encode()
    + caller-supplied non-zero outer request_id
    -> ControlFrame::new(ControlMessageKind::Command, request_id, exact PRWP bytes)
```

Decode:
```text
already-decoded ControlFrame
    -> require ControlMessageKind::Command
    -> preserve frame.request_id() as outer correlation metadata only
    -> CandidatePublicationWireSubmission::decode(frame.payload())
```

Successful decode proves only outer-kind correctness plus BQ bounded structural/type decoding.

## 9. Request-ID boundary remains unchanged

The pure adapter may accept and preserve a caller-supplied non-zero outer `request_id`, but it may not allocate, persist, route by, authenticate by, authorize by, or infer freshness/currentness from that value.

Still unselected:
- request-ID allocator/custody;
- uniqueness/reuse/restart semantics;
- response matching/lifecycle table;
- SessionId custody;
- authenticated control-plane session wiring;
- requester/recipient routing;
- expected-device scheduling provenance.

## 10. Byte-stability requirements

The corrected source successor must leave byte-stable unless a separately proven defect emerges:
- `crates/prw-control-transport/src/lib.rs`;
- `crates/prw-control-transport/Cargo.toml`;
- `crates/prw-remote-bridge/src/candidate_publication_wire.rs`;
- unrelated bridge source;
- Agent/Desktop/Android runtime source;
- workflows.

The two lockfiles are authorized only because exact canonical `--locked` validation proved they are required by the selected direct path dependency.

## 11. No unrelated lock churn

Before closure, the corrected successor must prove by exact diff inspection that:
- root `Cargo.lock` changed only as required by the new bridge dependency edge;
- Android-native `Cargo.lock` changed only as required to add the existing path package and bridge dependency edge;
- no crate version, checksum, registry source, or unrelated dependency edge changed.

Any unrelated lock churn blocks closure.

## 12. Explicit non-materializations

BU does not select or materialize:
- a new control message kind;
- changes to PRWC codec/TLS/frame limits;
- request-ID allocation/custody;
- response/ack/error protocol;
- auth/session establishment or session store;
- requester/recipient routing or scheduling;
- broker/dispatcher/listener/acceptor;
- TCP/TLS connect or frame read/write execution;
- retries/idempotency/deduplication;
- candidate-ID allocation;
- path-kind classification;
- endpoint discovery;
- publication admission/freshness rotation/reachability commit;
- registry/provider/database mutation;
- STUN/ICE/TURN/relay/QUIC activation;
- Agent/Desktop/Android runtime wiring;
- host/network mutation;
- deployment/restart/recovery;
- merge.

## 13. Source-materialization successor validation rule

The corrected source successor may close only after:
- exact BU predecessor lineage remains unchanged;
- final compare contains exactly the six selected paths and no seventh path;
- both lockfile diffs satisfy the no-unrelated-churn rule;
- canonical Rust validation on the exact final head reaches terminal success including locked dependency graph, formatting, Clippy, tests and workspace build;
- Android validation reaches terminal success including native adapter and Android application validation if triggered;
- every other automatically triggered workflow reaches a terminal non-failing verdict;
- immutable Drive audit raw-readback verifies exactly;
- rolling Drive evidence passes fresh predecessor guard, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No production activation is authorized by source materialization.

## 14. BU closure conditions

BU itself is docs-only and may close only after:
- exact BS predecessor lineage remains unchanged;
- exact BS→BU compare contains only this contract;
- automatically triggered workflows reach terminal non-failing verdicts;
- immutable BU Drive audit is uploaded in project folder and raw-readback verified;
- rolling Drive evidence passes fresh predecessor guard, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

Gate target remains:
`C03E_BU_CANDIDATE_PUBLICATION_PRWP_PRWC_LOCKFILE_SCOPE_RESELECTED`
