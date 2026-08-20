# Phase 152 C02f-AL — Recovery-Epoch Reconciliation Orchestrator Staging Contract

Status: `SOURCE_ORCHESTRATOR_STAGED / AK_VALIDATED_BASE_6B040A97 / PROVIDER_NEUTRAL / EXACT_RETAINED_H_N_A / STRONG_REOBSERVATION_REQUIRED / ONE_BOUNDED_EXACT_REISSUE / SECOND_INDETERMINATE_REOBSERVED / NO_THIRD_SUBMIT / DEFINITIVE_COMMIT_RECONFIRMS_GLOBAL_HEAD / NO_REPLAN / NO_ATTEMPT_ID_GENERATION / NO_RUNTIME_EXPORT / NO_PROVIDER_CONTACT / NO_CLOUD_RESOURCE / NO_SCHEMA_DDL / NO_RECOVERY_EXECUTION / NO_RUNTIME_ACTIVATION`

Date: 2026-08-20
Repository: `powercode365-dotcom/prw-executor-private`
Repository ID: `1334911207`
Authoritative AK predecessor: `6b040a97a82ce7b49b390c71f00d41c3ca4a03d6`
AK PR: `#56` (`open / draft / unmerged`, canonical Rust #781 + Android #293 PASS)
AL branch: `phase-152-c02f-al-recovery-epoch-orchestrator-staging`

## Purpose

C02f-AL materializes the provider-neutral outer state machine that owns reconciliation of one exact recovery-epoch issuance plan after C02f-AJ defined the domain/authority port and C02f-AK materialized the concrete Spanner adapter.

The AL orchestrator accepts an already-retained `RecoveryEpochIssuancePlan (H, N, A)`. It never creates a new attempt ID, never silently replans to a later predecessor, and never changes `H`, `N`, or `A` while resolving the logical operation.

The source is compiled through a staging integration harness and is not exported through `prw-control-plane::lib.rs`. No runtime construction or activation is introduced by AL.

## Exact state machine

For one retained plan `(H, N, A)`:

1. submit the exact plan once through `RecoveryEpochLedgerAuthority::submit_issuance`;
2. definitive `Aborted` is terminal no-authority for this operation;
3. definitive successful commit does **not** immediately authorize recovered authority startup;
4. after definitive successful commit, perform a fresh strong head read;
5. return `Current` only when the strong head proves exact epoch `N` with exact `LastAttemptId == A`;
6. if the fresh strong head is greater than `N`, return `CommittedButSuperseded`;
7. a head below `N`, bootstrap after successful commit, or same `N` with a different last-attempt ID is contradictory and fails closed;
8. `MutationIndeterminate` triggers a fresh strong `head + exact history(N)` re-observation before any retransmission;
9. classify that observation only with the C02f-AJ canonical classifier;
10. `CommittedCurrent` -> terminal `Current`;
11. `CommittedButSuperseded` -> terminal no-current-authority result;
12. `Superseded` -> terminal no-authority result;
13. only `ProvenNotCommitted` consumes the single reissue budget;
14. submit the **same exact plan `(H,N,A)`** a second and final time;
15. definitive second success again requires fresh strong head confirmation before `Current`;
16. definitive second `Aborted` is terminal no-authority;
17. second `MutationIndeterminate` is strongly re-observed again;
18. exact committed/superseded classifications are terminal;
19. if the second re-observation again proves non-commit, fail `ReissueLimitReached`;
20. there is no third submission path.

## Cancellation and background-work boundary

AL starts no detached task and owns no background retry loop. If the caller drops the in-flight orchestration future while a strong re-observation is pending, no later reissue can occur from AL because the second submit exists only in the continuation of that same future after `ProvenNotCommitted` is returned.

## Authority semantics

`RecoveryEpochResolvedOutcome::Current` is the only AL result asserting that the exact proposed epoch is proven current at the terminal strong observation used by this operation.

`CommittedButSuperseded`, `Superseded`, and `Aborted` grant no recovery/runtime authority.

AL still does not initialize the etcd within-epoch sequence head, allocate a sequence, construct a production provider, or activate recovered service authority. Those remain later explicit boundaries.

## Validation requirements

AL canonical validation must prove at minimum:

1. exact AK base and additive AL-only scope;
2. no `Cargo.toml` or `Cargo.lock` change;
3. canonical rustfmt PASS;
4. Clippy `-D warnings` PASS;
5. full workspace tests/build PASS;
6. Android native/application validation remains PASS;
7. fake-ledger tests cover definitive commit strong-head confirmation;
8. fake-ledger tests cover first indeterminate exact commit without reissue;
9. fake-ledger tests cover one exact reissue after `ProvenNotCommitted`;
10. fake-ledger tests prove a second proven-noncommit indeterminate stops at exactly two submissions;
11. superseded and provider-error paths fail closed without reissue;
12. cancellation while strong re-observation is pending produces no detached reissue.

## Non-claims

AL does not claim or perform:

- attempt-ID generation or custody;
- Google Cloud project/instance/database creation;
- Spanner schema DDL or bootstrap data application;
- service account, ADC, Workload Identity, IAM or FGAC materialization;
- provider endpoint selection/contact;
- real recovery epoch issuance;
- disaster-recovery execution or snapshot restore;
- etcd sequence-head initialization or sequence allocation;
- etcd allocator TLS/auth/RBAC materialization;
- runtime provider construction/export/activation;
- R1-R4 stale-side-effect rejection;
- deployment;
- merge of PR #56 or any AL PR.

Authorization/execution boundary:

`C02F_AL_RECOVERY_EPOCH_ORCHESTRATOR_SOURCE_ONLY / EXACT_PLAN_RECONCILIATION / ONE_BOUNDED_EXACT_REISSUE / NO_THIRD_SUBMIT / NO_PROVIDER_CONTACT / NO_RESOURCE / NO_CREDENTIAL / NO_SCHEMA_DDL / NO_RECOVERY_EXECUTION / NO_RUNTIME_ACTIVATION / NO_DEPLOYMENT / NO_MERGE`
