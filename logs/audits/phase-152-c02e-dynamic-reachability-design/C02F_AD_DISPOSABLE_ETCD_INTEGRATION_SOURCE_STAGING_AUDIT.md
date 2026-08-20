# Phase 152 C02f-AD — Disposable etcd Integration Validation Source Staging Audit

Status: `SOURCE_STAGING_COMPLETE / STATIC_READBACK_PASS / DISPOSABLE_ETCD_EXECUTION_NOT_RUN / NO_PRODUCTION_ENDPOINT / NO_TLS_AUTH_RBAC / NO_RUNTIME_ACTIVATION / NO_PRODUCTION_ACTIVATION`

Date: 2026-08-20
Repository: `powercode365-dotcom/prw-executor-private`
Predecessor C02f-AD validated head: `1d1d16a91295d191b446a3061e1b15561a047f3d`
Disposable integration staging branch: `phase-152-c02f-ad-disposable-etcd-integration-staging`
Disposable integration source head: `674acad8f3e82762bca6beac44f898cd7f12cf7b`
Frozen C02d checkpoint: `857583b25ed1206317641a93fd8f927819c954d8`

## Authorization and boundary

The user explicitly authorized `C02f-AD disposable etcd integration validation staging` after C02f-AD obtained full canonical Rust executable validation PASS in run #744.

This tranche stages only an isolated integration-validation harness and a dedicated CI workflow capable of executing it later on a validation PR. It does not authorize a PR, merge, production endpoint, production credentials, product runtime ownership, or production authority activation.

Explicitly not performed or activated:

- no production/disposable etcd endpoint was contacted during source staging;
- no `Client::connect` was added to production `prw-control-plane` source;
- no production `Cargo.toml` or `Cargo.lock` change;
- no TLS feature, trust root, certificate, credential or RBAC configuration;
- no Watch, lease, TTL or clock-based currentness path;
- no fence allocator or authority-attempt RNG materialization;
- no first-ever production absent-key bootstrap;
- no recovery epoch/high-water implementation;
- no Agent/runtime/task ownership;
- no R1-R4 production effect activation;
- no PR creation, merge, main mutation or deployment.

## Source commit

The staged source commit is:

`674acad8f3e82762bca6beac44f898cd7f12cf7b`
`phase 152 c02f-ad: stage disposable etcd integration validation`

It is exactly one commit ahead of validated C02f-AD head `1d1d16a...` and changes exactly three newly-added validation-only paths:

1. `.github/workflows/phase-152-c02f-ad-disposable-etcd-validation.yml`
2. `scripts/validate-phase-152-c02f-ad-disposable-etcd.sh`
3. `tools/validation/phase-152-c02f-ad-disposable-etcd.rs`

Compare result: `ahead 1 / behind 0`, 419 additions, zero deletions.

No product source path, manifest, lockfile, bridge source, control-plane source, Agent source or runtime source is changed by this commit.

## Disposable endpoint isolation

The Rust harness rejects every endpoint except the exact literal:

`http://127.0.0.1:2379`

The validation script independently pins:

- client endpoint `http://127.0.0.1:2379`;
- peer endpoint `http://127.0.0.1:2380`;
- one temporary data directory created by `mktemp -d`;
- single-member cluster state `new`;
- no TLS/auth/RBAC flags;
- process cleanup and temporary-state deletion on script exit.

This is a disposable test fixture only and cannot select or contact a production endpoint through environment input because the harness performs its own exact endpoint equality check.

## Pinned etcd server fixture

The script pins etcd `v3.7.1`, the current stable patch release in the already-selected etcd 3.7 line as of 2026-08-20.

The script downloads the official Linux amd64 release archive and its `SHA256SUMS` asset from the same exact GitHub release, extracts the selected archive hash, validates a single 64-hex digest, and verifies the archive with `sha256sum --check` before extraction.

After extraction it requires the binary version output to contain the exact line:

`etcd Version: 3.7.1`

The external release artifact is never copied into the repository.

## No production runtime dependency expansion

The integration Rust harness intentionally lives outside Cargo workspace targets.

The script first runs:

`cargo build --locked -p prw-control-plane -p prw-remote-transport`

It then links the standalone harness with `rustc` against the already-built locked workspace artifacts for:

- `etcd_client`;
- `prw_connectivity`;
- `prw_control_plane`;
- `prw_core`;
- `tokio`.

Tokio is already pinned and materialized by the existing `prw-remote-transport` workspace package. Therefore the tranche does not add a Tokio/runtime dependency to production `prw-control-plane` and does not mutate the locked dependency graph.

The script requires exactly one matching rlib for each direct harness dependency and fails closed otherwise.

## Real integration scenarios staged

When later executed against the disposable loopback etcd, the harness is designed to validate:

1. **Absent established authority fails closed**
   - exact peer key is absent;
   - real `ReachabilityLiveOwnerEtcdStore::currentness` must return `Transaction(MissingEstablishedState)`;
   - absence does not become authority and performs no provider write.

2. **Fixture-only established-state seed**
   - test setup directly writes one valid canonical `Released` PRWL record through a fixture `KvClient`;
   - this exists only to establish disposable test state and is not production bootstrap logic.

3. **Real acquisition Txn commit**
   - provider performs the real linearizable exact-key Get;
  - C02f-AB constructs the canonical acquisition plan;
   - C02f-AD executes the real dual-CAS etcd Txn;
  - definitive success must classify as `Committed`;
   - the intended fence must then classify as `Current` and canonical successor bytes must be observable.

4. **Real compare-failure branch authoritative Get**
   - a transaction plan is constructed from an older observation;
  - fixture setup writes a newer valid Current record before execution;
  - the stale Txn must not commit;
   - C02f-AD must return `CompareFailed` carrying the newer authoritative failure-branch Get state;
  - the old fence is Stale and the newer fence remains Current.

5. **Stale release fencing**
   - a release plan is built from a current observation;
   - fixture setup installs a newer owner before stale release execution;
   - the stale release must compare-fail and return the newer state rather than overwrite it.

6. **Definitive release commit**
   - the exact current owner release executes through the real dual-CAS Txn;
   - the final persisted record must be a `Released` tombstone preserving exact peer, fence and attempt ID;
   - the released fence must classify as Stale.

The harness emits explicit PASS markers for each stage and the terminal marker:

`C02F_AD_DISPOSABLE_ETCD_INTEGRATION_PASS`

## Dedicated validation workflow

The staged workflow is:

`Phase 152 C02f-AD Disposable etcd Validation`

Properties:

- `pull_request` and `workflow_dispatch` triggers only;
- no branch `push` trigger;
- repository permission is only `contents: read`;
- PR job is guarded to the exact disposable integration staging branch;
- runner is `ubuntu-24.04`;
- 15-minute job timeout;
- locked dependency metadata verification before integration execution;
- no repository secret or production credential input.

The workflow has not run because no validation PR has been created in this tranche.

## Static validation completed

Source-staging checks completed before publication:

- local `bash -n` on the validation script: PASS;
- YAML structural parse of the workflow: PASS;
- explicit raw-rustc crate name materialized to avoid hyphen-derived crate-name ambiguity;
- curl connection and total time bounds materialized;
- exact etcd binary version check materialized without pipeline/SIGPIPE ambiguity;
- required shell-tool inventory materialized;
- GitHub post-commit readback confirmed exact harness/script/workflow blob SHAs;
- GitHub compare confirmed only the three validation-only files above.

No local Rust compile/fmt result is claimed because the current local execution environment does not provide `cargo`, `rustc`, or `rustfmt`.

## Gate decision

Current gate:

`C02F_AD_DISPOSABLE_ETCD_INTEGRATION_SOURCE_STAGING_COMPLETE -> DISPOSABLE_ETCD_EXECUTION_VALIDATION_PENDING`

A validation-only draft PR would be the next mechanism for actually starting a disposable loopback etcd and executing the staged harness in GitHub Actions. PR creation remains a separate GitHub mutation and is not inferred from this staging authorization.

Even after a disposable integration PASS, TLS/auth/RBAC, recovery/high-water, product runtime ownership and production activation remain separate architecture and authorization gates.
