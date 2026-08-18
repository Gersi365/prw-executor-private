# Desktop Functional Management Slice C02e — Tranche 2 Actual Phase 141 Integration Closeout

Status: `TRANCHE2_ACTUAL_PHASE141_INTEGRATION_PASS`

## Authority

This checkpoint closes the separately authorized Phase 152 C02e Tranche 2 for the actual Phase 141 integration-test dependency edge and its executable validation.

Frozen predecessor C02d remains:

`857583b25ed1206317641a93fd8f927819c954d8`

Tranche 1 closeout remains:

`d80e81903077c2c9dd7baf2e1743e83bf06f7dc0`

The exact final Tranche 2 validation head was:

`b3db8a047d159fbf7ef1fc97adbf24412585badd`

The authoritative PASS evidence and validated lockfile were committed by GitHub Actions as child commit:

`4583b005a3fed876104e802a1f765c550349315b`

Authoritative report:

`logs/audits/phase-152-c02e-dynamic-reachability-design/C02E_TRANCHE2_FINAL_VALIDATION_b3db8a047d159fbf7ef1fc97adbf24412585badd.txt`

Report blob:

`3b2e1c713457dbc36838d64a4ef71e97b4f9bb4b`

## Authorized mutation closed by this tranche

The production dependency graph remains unchanged.

The only manifest edge added by Tranche 2 is the test-only edge in `crates/prw-remote-bridge/Cargo.toml`:

```toml
[dev-dependencies]
prw-nat-traversal = { path = "../prw-nat-traversal" }
```

This edge exists only so the remote-bridge integration test can instantiate the actual Phase 141 Sans-I/O traversal implementation. It does not make `prw-remote-bridge` the production traversal owner and does not authorize a corresponding normal dependency.

## Actual Phase 141 integration-test result

`crates/prw-remote-bridge/tests/reachability_phase141_integration.rs` uses actual `IceConnectivitySession`, `TraversalDatagram`, and `CandidateReachabilityUpdate` values from `prw-nat-traversal`.

The test remains disposable and in-memory. It opens no socket and performs no production STUN/ICE/TURN traffic.

The integration surface proves:

1. a current owner can hold one actual Phase 141 traversal session for the current candidate plan;
2. a successful authenticated candidate refresh invalidates the old traversal session even when a candidate is exactly retained;
3. an already-polled/queued observation from that old actual session is rejected by upper lifecycle currentness after refresh;
4. a newly constructed actual session for the refreshed plan may produce a current observation that applies;
5. a failed candidate refresh preserves the existing actual session and its still-current queued observation;
6. target transport rotation replaces the plan and invalidates the prior traversal lifecycle;
7. session currentness in this test harness is object identity only and is not a production freshness counter, nonce, timestamp, wire field, persistence key, or replay mechanism.

## Cargo materialization result

Cargo 1.97.1 materialized the dependency edge rather than the lockfile being edited by hand.

SHA-256:

- lock before Tranche 2 materialization: `c22b0efad4fc6e9e404ce68d182da6713296f88039338429a3af983b409b24cb`;
- validated candidate/final `Cargo.lock`: `becbd46de66354591afd3a4d755a9b4ba06f9c9c15045069b85e04a99525423a`.

The exact lock audit records:

- `PACKAGE_IDENTITIES=UNCHANGED`;
- `EXTERNAL_DEPENDENCY_TUPLES=UNCHANGED`;
- `ONLY_LOCAL_GRAPH_DELTA=prw-remote-bridge:+prw-nat-traversal`.

The only `Cargo.lock` semantic diff is addition of `"prw-nat-traversal"` to the existing `prw-remote-bridge` dependency list.

## Executed validation result

The exact final validation head passed:

1. Cargo dependency materialization;
2. exact semantic lock audit;
3. `cargo metadata --locked --no-deps --format-version 1`;
4. `cargo fmt --all -- --check`;
5. actual Phase 141 remote-bridge integration test;
6. focused locked Clippy for `prw-remote-bridge` and `prw-nat-traversal`, all targets/features, warnings denied;
7. focused locked tests for `prw-remote-bridge` and `prw-nat-traversal`;
8. full locked workspace Clippy;
9. full locked workspace tests;
10. full locked workspace build.

Final markers are:

- `MATERIALIZE_RC=0`;
- `LOCK_AUDIT_RC=0`;
- `LOCKED_METADATA_RC=0`;
- `FORMAT_RC=0`;
- `INTEGRATION_TEST_RC=0`;
- `FOCUSED_CLIPPY_RC=0`;
- `FOCUSED_TESTS_RC=0`;
- `WORKSPACE_CLIPPY_RC=0`;
- `WORKSPACE_TESTS_RC=0`;
- `WORKSPACE_BUILD_RC=0`;
- `PRE_NORMALIZE_DRIFT_RC=0`;
- `TARGET_RESTORE_RC=0`;
- `FINAL_DRIFT_RC=0`;
- `FIRST_FAILURE=NONE`;
- `STATUS=PASS`.

## Tracked Cargo cache normalization

The repository currently tracks Cargo build/cache state under `target/`. Full validation therefore rewrote tracked cache/build artifacts even though source and dependency authority did not change.

The final validator recorded the pre-normalization tracked diff and proved that every changed path other than `Cargo.lock` was under `target/` (`PRE_NORMALIZE_UNEXPECTED_DIFF=`).

It then restored the complete tracked `target/` tree from the validation head and re-ran the strict drift guard. The post-normalization result was exactly:

`FINAL_TRACKED_DIFF=Cargo.lock`

This normalization does not loosen dependency validation. It prevents tracked Cargo cache artifacts from being mistaken for source/dependency authority.

## Historical validation progression

Historical Tranche 2 reports are retained as evidence rather than rewritten:

- the initial run identified deterministic rustfmt debt;
- the next run proved the actual integration test passed and identified four focused Clippy diagnostics;
- those diagnostics were corrected without lint suppression or dependency drift;
- the following run passed every Cargo/source gate but exposed the tracked-`target/` harness false positive;
- the final isolated validator normalized the complete tracked target tree and passed the unchanged strict final drift rule.

## Temporary harness cleanup

The final validator removed itself after committing the authoritative PASS report and validated lockfile. Earlier temporary Tranche 2 corrective/validation workflows were removed before the final run.

The retained repository evidence consists of reports, the validated dev edge, the actual integration test, the materialized lockfile, and this closeout record.

## Boundaries that remain closed

This closeout does **not** authorize or imply:

- a production `prw-remote-bridge -> prw-nat-traversal` dependency;
- selection of the production owner that jointly owns candidate plan, publication-freshness authority and traversal session;
- a production freshness counter/nonce/timestamp representation;
- publication-freshness wire fields, persistence, replication, recovery or re-baselining mechanics;
- async runtime/task/cancellation/queue ownership;
- real socket ownership or TCP/UDP/STUN/ICE/TURN/QUIC activation;
- Agent/bootstrap traversal activation;
- deployment, signing, privileged/system mutation, PR merge or production activation.

Tranche 2 therefore closes only the actual Phase 141 **test composition and validated dependency materialization** boundary.