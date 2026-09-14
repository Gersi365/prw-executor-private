# C03e-SW — admission-timing production reachability custody layout corrective selection

Status: `CORRECTIVE SELECTION — VALIDATION PENDING`

Boundary:
`FALLIBLE_ADMISSION_TIMING_PRODUCTION_REACHABILITY_CUSTODY_LAYOUT_CORRECTIVE_SELECTION`

Selection result:
`SEVEN_PATH_DORMANT_SOURCE_LAYOUT_SELECTED / EIGHT_FORWARDING_SIBLINGS / MISSING_SH_PRODUCTION_CUSTODY_LAYER_INCLUDED / SN_TIMING_AND_FAILURE_CUSTODY_SEMANTICS_PRESERVED / SU_CONNECTED_LOCAL_EXECUTION_MODEL_PRESERVED / SV_LOCAL_DIAGNOSTIC_NONCANONICAL / NO_SOURCE_MUTATION / NO_RUNTIME_ACTIVATION`

C03e-SW is documentation-only. It corrects the source-layout gap discovered during the directly operated local C03e-SV attempt. It selects the missing production reachability custody forwarding layer for a later source checkpoint, while preserving all existing timing and failure-custody decisions. It performs no Rust implementation, publishes no SV source, replaces no prior contract and claims no source acceptance.

## Current mutation ceiling

Exactly one new documentation path is selected for SW:

`contracts/C03E_SW_ADMISSION_TIMING_PRODUCTION_REACHABILITY_CUSTODY_LAYOUT_CORRECTIVE_SELECTION_STAGING.md`

The seven-path ceiling below applies only to a future source checkpoint. Expected SU -> SW topology is one direct commit, ahead 1 / behind 0, with exactly one added contract and zero existing-file changes. Integrated main remains outside this continuation and unchanged.

## Exact predecessor authority

Repository: `Gersi365/prw-executor-private`, stable ID `1334911207`.

Authoritative predecessor: evidence-closed C03e-SU PR #635, draft/open/unmerged, status `CORRECTIVE SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`.

- branch: `phase-152-c03e-su-connected-local-patch-execution-surface-corrective-selection`;
- exact head: `80d9debde00bac5b64786a3f6b7c82aa4f0b637d`;
- exact tree: `10bd210cfbdfa65fcf1bdb03ea4eb2fa4d53c2ff`;
- SU contract blob: `2dbec1deaee3c9aa9ce3f4155306f631202ef5d7`;
- unchanged SN contract blob: `c081a019cc06012464e5f1cc584dd1814fb6b586`;
- fresh main: `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`.

SU Rust Validation #1869, run `34877368500`, job `104087789075`, was re-read as SUCCESS on exact SU, including locked dependency graph, formatting, Clippy, workspace tests and workspace build. This is predecessor validation only; SW requires its own exact-final-head checks.

Canonical immutable SU evidence:
`C03E_SU_CONNECTED_LOCAL_PATCH_EXECUTION_SURFACE_CORRECTIVE_SELECTION_AUDIT_2026-09-14.md`.

- Drive ID: `129F6l0ij1jP-a99GZVKPSQ1uMzE5acE6`;
- canonical parent: `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
- MIME: `text/markdown`;
- bytes: `11442`;
- SHA-256: `98e8ec86eb867509b16a72347db49d89f54eca626cd2ff3fbd001e85834a59ea`;
- final LF: true;
- revision: `0Bz5eMiLa5v9xZ2JpRW5nVGpIVEhJWDVmeHJVK0czcnNoL2cwPQ`;
- previous revision: null.

Fresh metadata, canonical exact-title uniqueness, complete reconstructed UTF-8 readback and revision history agree. The audit's frozen publication-pending status remains unchanged; SU's live PR body binds postpublication closure.

## Verified graph defect and SV diagnostic classification

SN's S1-to-S2 graph omits the existing C03e-SH production reachability custody forwarding layer. The Linux operation receives `ProductionReachabilityEndpointLifecycleRuntime`, not `RemoteSessionEndpointLifecycleRuntime`. Its private `endpoint` and `owner_custody` fields are decomposed only inside its defining module. Its existing forwarding method accepts infallible admission timing. That method retains the durable production reachability owner for the complete delegated endpoint drive, including endpoint close and idle drain.

Authoritative omitted file:

`crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs`

Exact SU blob: `260d53f5c46b912a6b3a592ca58a44e82d8f6d8e`.

Source anchors at SU: `drive_with_retained_custody` at line 54; runtime and private fields at line 69; existing SH forwarding method at line 331; retained-custody delegation at line 390. The existing endpoint startup method in `production_reachability_runtime_custody.rs` returns this production wrapper at lines 127–150.

Local compiler evidence: the six-path candidate reports E0599 at its new Linux call because the new fallible-admission-timing method is absent on this production wrapper. This is a source-layout gap, not an authentication or dependency-installation failure.

## Selected corrective boundary

SW continues directly from exact SU on `phase-152-c03e-sw-admission-timing-production-reachability-custody-layout-corrective-selection`. Pre-mutation live checks found no SV or SW remote branch, no all-state SV/SW PR, no later checkpoint in the recent lineage and no canonical SV/SW evidence object. These are creation guards, not a substitute for refreshing before each consequential write.

The local SV branch remains at SU with an uncommitted six-path candidate. It has zero source commits, zero remote refs, zero PRs and zero canonical audits. Its isolated Clippy diagnostic is E0599; candidate formatting/metadata/diff checks passed, but source acceptance tests and final-head CI did not complete. No SV PASS or source authority is inherited.

The retained local diagnostic patch is 55857 bytes, SHA-256 `07f7a4d87dea2568aaeb9892ce4b029ba2c6e65b23913a287790add4cbd1dd8f`. This identifies local archaeology only. The graph defect is independently established by exact-SU GitHub source, not by trusting the partial patch.

Preserve the local SV candidate, the noncanonical ST ref and every historical branch, PR, contract and immutable audit. SW does not reset, rename, reuse as authority, overwrite or delete any of them.

This contract supersedes only the six-path ceiling and direct S1 -> S2 edge in SN/SU as future implementation selection; it changes the future count to seven paths and adds S1a. All unmodified clauses of SN remain source-semantic authority, and SU remains the execution-surface authority. It inherits SN's source-error variants, error causality, intact-request carrier, callback signatures, duplicate preflight, timing failure transfer, idle/pending continuation, compatibility lift, producer/receipt custody and no-runtime boundary unchanged.

The docs checkpoint itself changes zero Rust paths. A later separately authorized source checkpoint begins only after the corrective contract has passed exact-head checks and evidence closure.

## Pinned seven-path future source ceiling

P0–P5 retain the roles and exact symbols selected by SN; P4 now resolves its production lifecycle call through S1a. P6 adds only the omitted production-custody layer. All hashes are exact SU, where P1 is absent.

| ID | Exact future source path | SU blob/state | Selected future role |
| --- | --- | --- | --- |
| P0 | `crates/prw-agent/src/remote_session_capability_runtime.rs` | `2b6a0fd693f2ddacec608018a4db811cc36ef4c0` | Private child module declaration and crate-only error/carrier re-exports. |
| P1 | `crates/prw-agent/src/remote_session_capability_runtime/admission_timing_failure.rs` | `absent` | New bounded timing-source error and intact-request failure carrier with unit tests. |
| P2 | `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs` | `d50e423d01b4161e244c075d59a85fd83533cf62` | Pure fallible preparation; shared result-capable collection/pending core; compatible infallible entry; two executor siblings and both-path tests. |
| P3 | `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs` | `881846753f51bdf94cff32ff0f9649dbcf50a80f` | Three existing selected forwarding siblings; preserve projection, producer, sender and endpoint custody. |
| P4 | `crates/prw-agent/src/linux_bootstrap.rs` | `b5d236d3c0264ea8870e90af96c76294b50d0fc1` | One runtime-input-aware Linux sibling; its production lifecycle receiver calls S1a and carries K separately. |
| P5 | `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs` | `cccefed1192243a16b67c971ca6ba8fe13ae2c82` | One dormant configured higher-owner sibling; preserve one channel and unchanged F-carrying population. |
| P6 | `crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs` | `260d53f5c46b912a6b3a592ca58a44e82d8f6d8e` | Add the fallible-admission-timing SH sibling and local custody/forwarding tests. |

The later source ceiling becomes seven Rust paths: six existing paths modified and the same one new child module P1 added. No eighth path, public field, public library API, manifest, dependency, lockfile, workflow, configuration, service, concrete provider, concrete sink or executable caller is included.

P6 retains the current private fields and the current `drive_with_retained_custody` helper without semantic change. All existing infallible methods remain unchanged.

## Corrected propagation graph and exact new sibling

The selected future graph is:

`S0 (P5) -> S1 (P4) -> S1a (P6) -> S2 (P3) -> S3 (P3) -> S4 (P3) -> S5 (P2) -> S6 (P2) -> shared result core`.

This is eight new forwarding/entry siblings, with one added production custody boundary. S1 still uses the unchanged runtime-input acquisition, dispatcher-factory capture, production bootstrap/bind, publisher and remote operation. Its actual production lifecycle receiver resolves the call to S1a. S1a delegates once to S2 on the privately retained inner endpoint.

S1a is a `pub(crate)` method on `ProductionReachabilityEndpointLifecycleRuntime` with the exact name:

`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_fallible_verifier_time_expected_device_admission_producer_with_higher_observation_projection_with_fallible_admission_timing`.

Preserve the original ordered arguments and return type of the SH method. Add generic `Cause` and `K`; append `on_timing_failure: K` as the last ordinary argument. Change only F's result bound to:

`F: FnMut(&DeviceId) -> Result<RemoteSessionRealAdmissionTiming, RemoteSessionAdmissionTimingSourceError<Cause>>`.

Use the separate sink:

`K: FnMut(RemoteSessionAdmissionTimingFailure<D, fn() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>, RemoteSessionAdmissionTimingSourceError<Cause>>)`.

Keep `Cause: std::error::Error + Send + 'static`; do not add Sync or impose new bounds on existing infallible APIs. Preserve P/D/PS/DF/O/R/E and the exact verifier relationship. Use the P0 crate-only type re-exports; no module relocation or visibility widening is selected.

Consume the production wrapper once, destructure its existing fields inside P6, then call the existing `drive_with_retained_custody(endpoint, owner_custody, ...)`. Within that closure invoke the selected S2 sibling once with the same original inputs plus F/K. Return the same collection configuration result. The durable owner must remain alive until the lower drive returns, including its existing close-then-wait-idle sequence. Timing failure remains a K handoff and never becomes a collection error or receipt.

## Future implementation acceptance obligations

Retain all SN acceptance obligations, including both actual request paths, no AJ on failure, continued distinct requests, one pending producer and one eventual receipt, compatibility, exact forwarding and full local plus exact-final-head repository checks.

Add non-network sentinel evidence inside P6 for:

1. Durable owner stays alive during the entire delegated drive and drops once after it returns, on both ordinary and collection-error returns.
2. New typed forwarding compiles with F and K together and does not execute either callback during ownership setup.
3. F and K move once to the same lower operation; an intact failure carrier reaches K without O/R/E misclassification.
4. Endpoint cleanup stays in S5; the production owner remains retained across that cleanup and no extra close/rebind is introduced.

Tests must exercise the production-used custody/forwarding composition. Static name matching alone is not acceptance evidence. Do not activate production credentials, listeners, peers, services or providers for new custody tests.

## Protected paths and execution boundary

All future source changes remain confined to P0–P6. In particular preserve:

- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`, blob `1539b6b9a08bf18883d7a16022f15f7c240eaf08`: request/timing/rejection/AJ definitions;
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`, blob `304513eaf4ac72720a96c265f72dd8a56f5ac87f`: historical preparation and worker helpers;
- `crates/prw-agent/src/production_reachability_runtime_custody.rs`, blob `ffcddc0253de2b5430be798061ddad8e920a07ac`: existing production startup return type and owner composition;
- every manifest, lockfile, workflow, service/configuration file, executable caller and source file outside the selected seven paths.

SU's directly connected, user-authorized local patch/validation/ordinary commit-and-push capability class remains selected. This development surface was available and authenticated during the diagnostic attempt; its availability and repository access must be refreshed for the later source checkpoint. GitHub remains repository/CI authority and Drive remains immutable evidence authority. No cloud provisioning, user terminal relay or product runtime dependency is introduced.

The seven-path selection cannot be widened implicitly. If it proves insufficient for complete propagation or acceptance, stop and return to selection. In particular, do not omit a request arm, bypass the production owner, expose its private fields or turn a fallible source into infallible timing to fit the layout.

## SW documentation validation and evidence closure

Validate SW's one-added-contract ceiling, direct SU topology, complete source/tree preservation, P1 absence, exact P0/P2–P6 and protected blob identities, and the corrected complete graph from configured population through production custody to the shared collection core. Verify the names, visibility, ordered inputs, F/K signatures and retained owner lifetime against exact SU source.

These are documentation/source-graph checks; no future behavioral acceptance test is claimed executed merely because its obligation is selected. Run the repository-required checks for SW's exact final head and record success, skipped and absent workflows honestly. No predecessor or local SV validation is inherited.

Canonical SW audit title:
`C03E_SW_ADMISSION_TIMING_PRODUCTION_REACHABILITY_CUSTODY_LAYOUT_CORRECTIVE_SELECTION_AUDIT_2026-09-14.md`.

Freeze complete UTF-8 bytes, byte count, SHA-256 and final LF after binding exact final head/tree/contract blob and CI. Check exact-title canonical-parent collisions before one upload. Verify metadata, complete readback identity, unique title and single revision/no predecessor after publication. Preserve the immutable audit's publication-pending status; re-read exact GitHub state and bind verified postpublication closure in the SW PR body. Keep the PR draft/open/unmerged.

## Remaining gate and STOP

After docs closure, a separately authorized source checkpoint starts from the exact evidence-closed SW head, refreshes successor namespaces and all seven path identities, and completes the full behavioral acceptance suite. The existing partial local SV patch is diagnostic input only; it is not a selected predecessor, a validated implementation or authority to inherit any PASS.

Concrete production timing policy/provenance, concrete failure sink and its eventual disposition, executable wiring and runtime activation remain separately unselected. STOP after this docs-only checkpoint. No source materialization, merge, deployment, restart, repository-setting change, ready-for-review conversion, PR closure, branch deletion or history rewrite follows from SW closure.
