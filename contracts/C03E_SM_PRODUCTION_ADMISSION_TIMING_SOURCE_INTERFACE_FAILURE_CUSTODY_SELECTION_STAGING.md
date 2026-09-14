# C03e-SM — production admission-timing source-interface/failure-custody selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:
`PRODUCTION_ADMISSION_TIMING_SOURCE_INTERFACE_FAILURE_CUSTODY_SELECTION`

Selection result:
`FALLIBLE_ADMISSION_TIMING_SOURCE_INTERFACE_SELECTED_AT_CONTRACT_LEVEL / PRE_ADMISSION_FAILURE_RETAINS_EXACT_REQUEST / DISTINCT_TIMING_FAILURE_CUSTODY / EXISTING_INFALLIBLE_INTERFACES_UNCHANGED / CONCRETE_SOURCE_AND_RUST_PROPAGATION_NOT_SELECTED`

This checkpoint materializes one documentation-only selection contract. It selects the semantic interface and failure-custody model required by C03e-SL. It changes no Rust signature, source implementation, runtime behavior, executable caller, configuration or existing contract. Selection is not implementation or activation.

## Exact predecessor authority

The continuation boundary is C03e-SL PR #629, draft/open/unmerged, with post-publication status `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`.

- repository: `Gersi365/prw-executor-private`, stable ID `1334911207`;
- predecessor branch: `phase-152-c03e-sl-production-admission-timing-provenance-source-selection`;
- predecessor head: `995dfcdc567ef24a4616aca3e379f80878999daa`;
- predecessor tree: `a25828b75be3187b7e3b7da16e5c700f5282347b`;
- predecessor contract: `contracts/C03E_SL_PRODUCTION_ADMISSION_TIMING_PROVENANCE_SOURCE_SELECTION_STAGING.md`;
- predecessor contract blob: `e74bb8b292f892043f217667f49dd24f7aeb8f8e`;
- direct corrective SK -> SL comparison: ahead 1, behind 0, one added documentation path, +308/-0;
- corrective SK head: `5e0ad1f3dece52a6465ed464d94d97202bf99223`;
- corrective SK closure is recorded in PR #628 conversation comment `5663006029`; its currently empty PR body does not replace that verified closure record;
- PR #627 remains the non-authoritative earlier SK attempt.

Freshly observed integrated main is `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`, tree `63b8e59ca53797fdea6b95432e16f35eaf473604`. This contract is based on SL, not on main. It does not alter main or reopen its historical forward-only recovery.

### Predecessor validation and durable evidence

Exact SL-head PRW Rust Validation run `34840068012`, job `103962709369`, completed successfully, including locked dependency graph, formatting, Clippy, workspace tests and build. Runs `34840067977` and `34840067972` were skipped. No Android PASS is claimed. SK or SL validation must never be inherited as SM-head validation.

Canonical SL audit:

- title: `C03E_SL_PRODUCTION_ADMISSION_TIMING_PROVENANCE_SOURCE_SELECTION_AUDIT_2026-09-14.md`;
- Drive ID: `18Qowx_c4EYIBiAboPYu6SsdF5QpT-mBC`;
- parent: `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
- MIME: `text/markdown`;
- bytes: `15552`;
- SHA-256: `9bfa4e07b6c937f845e4b907433f708b557378c94ab8b7f858ea9eb5214b6f15`;
- final LF: true;
- revision: `0Bz5eMiLa5v9xajhGMklGZjluY1IvSWJqa0t6L25NenQxcERBPQ`;
- previous revision: null.

Fresh metadata, canonical-parent search and revision readback agree. Complete connector text reconstructed as UTF-8 matches the recorded byte count and SHA-256. The raw-download response independently reports 15552 bytes; the hash above was recomputed from complete text readback, not claimed to be independently hashed from the streamed file reference. The immutable audit retains its pre-publication status; live PR #629 supplies the later closure binding.

## Fresh successor guards and changed-path ceiling

Before mutation, refresh SL head/PR, main, relevant recent/all-state PR search, matching branch refs and canonical Drive evidence. An existing or conflicting successor requires re-audit and reuse/reconciliation, not duplicate creation or overwrite.

The selected repository ceiling is exactly one new file:

`contracts/C03E_SM_PRODUCTION_ADMISSION_TIMING_SOURCE_INTERFACE_FAILURE_CUSTODY_SELECTION_STAGING.md`

Expected direct SL -> SM topology is one commit, ahead 1, behind 0, merge base exact SL. All existing blobs and paths must be preserved. Zero Rust/source/runtime/Cargo/lockfile/workflow/Android/package/service/repository-configuration mutation is allowed. No existing contract is replaced.

## Exact-current source findings

All findings below are pinned to the SL head above.

| Existing surface | Exact blob | Finding |
| --- | --- | --- |
| `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs` | `cccefed1192243a16b67c971ca6ba8fe13ae2c82` | Dormant SJ wrapper accepts caller-supplied timing and the three existing callbacks; constructs one capacity-one channel and transfers its sole endpoints. |
| `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs` | `1539b6b9a08bf18883d7a16022f15f7c240eaf08` | Defines the timing bundle, intact request, duplicate-only rejection reason and existing real-admission failure carrier. |
| `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs` | `304513eaf4ac72720a96c265f72dd8a56f5ac87f` | `prepare_expected_request` rejects duplicate active devices without sampling; otherwise samples once and returns the intact request beside timing. |
| `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs` | `d50e423d01b4161e244c075d59a85fd83533cf62` | Both cooperative producer-request and idle-request paths reuse preparation before request decomposition and AJ construction. |
| `crates/prw-agent/src/remote_session_capability_runtime/real_remote_admission_transaction.rs` | `812b56e9b948a41f2f746eb406ba24567efbd528` | AJ error categories are Registry, Accept, Challenge, Authentication and Binding; transport/session cleanup belongs to later transaction stages. |
| `crates/prw-session/src/prwa_verifier_source.rs` | `e34c3d452b9fd5c9787abbf1f36106e3b97e3b0b` | Server-local verifier clock is fallible; pre-epoch conversion and expiry overflow have explicit errors. |

Existing timing bound:

`F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming + Send + 'static`.

Existing bundle:

- `challenge_validity_unix_seconds: Range<u64>`;
- `authentication_now_unix_seconds: u64`;
- `application_lease_unix_seconds: Range<u64>`.

The current constructor packages these inputs; it does not validate their provenance or ranges. A successfully constructed bundle alone therefore proves neither production provenance nor policy validity.

The request retains `DeviceId`, `SessionId`, PRWM authentication request correlation, dispatcher and verifier-time provider until consuming decomposition. It has no timing-error lane. `RemoteSessionExpectedDeviceAdmissionRejectionReason` currently has only `DuplicateActiveDevice`. `RemoteSessionRepeatedAdmissionFailure` retains DeviceId and an AJ error, not the intact pre-admission request.

## Selection between SL alternatives

### Earlier infallible production authority is not established

The audited surfaces do not establish an existing infallible production source satisfying all three timing components and per-admission freshness. Startup validation may validate policy inputs, but cannot by itself prove that a later wall-clock observation or checked arithmetic will succeed. Fixed test timing and process-start time are not production evidence.

This is a bounded finding about the audited lineage, not a repository-wide proof that no possible implementation exists.

The earlier-validation alternative is therefore not selected as a solution to the current failure mismatch.

### Fallible interface evolution is selected semantically

Select a future explicit fallible timing acquisition boundary with this conceptual shape:

`(&DeviceId) -> Result<RemoteSessionRealAdmissionTiming, AdmissionTimingSourceFailure>`.

`AdmissionTimingSourceFailure` is a contract role, not an existing or newly materialized Rust symbol. Exact type names, visibility, generic bounds, modules and exports remain for a separate source-layout/propagation gate.

The source borrows only the exact expected logical DeviceId as selector. It must not receive, consume, clone or derive authority from the request, dispatcher, verifier provider, channel endpoint, session/capability authority, transport identity or scheduling grant.

A future success means the complete three-component bundle has passed the selected provenance, conversion, ordering and arithmetic checks. A failure must remain distinguishable from success and must retain bounded diagnostic causality.

The contract selects a synchronous fallible decision at the existing sampling boundary. It selects no async acquisition, blocking I/O, retry loop, background sampler or new task. A production source requiring such behavior must return to a separate review.

Existing infallible APIs are preserved in this checkpoint. No signature replacement, compatibility wrapper or new overload is implemented or authorized for immediate source materialization.

## Acquisition order and atomic success law

The future implementation must preserve these ordered decisions:

1. Existing shutdown/capacity/request-selection behavior retains precedence; timing is not sampled at startup, enqueue, send or while no eligible request has been selected.
2. Once a request is held, duplicate-active-device preflight occurs first. A duplicate follows the existing rejection path with its exact intact request and zero timing acquisition.
3. For a vacant expected DeviceId, invoke the selected fallible timing source exactly once while the full request remains owned and intact.
4. On success, retain the returned complete timing bundle beside the exact request. Only then may the existing path decompose the request and construct AJ.
5. On failure, produce no accepted timing bundle and construct no AJ future. Transfer failure plus the exact intact request once to the selected pre-admission failure custodian.

Both idle-request and cooperative producer-request paths must eventually obey the same law. Changing one path alone is not a valid production propagation result.

One invocation is a boundary-level count; it does not silently select an exact number of underlying clock reads, concrete durations or cross-window policy. Those remain explicit provider-design decisions. Acquisition must not reuse a stale successful value after a current failure.

The existing sample-before-AJ placement does not prove freshness after a long transport-accept wait. SM selects no later resampling or time-domain conversion. Any requirement to resample after accept or during authentication needs a separate transaction/timing review.

## Failure-custody selection

### Preserve the complete pre-admission request

Select a dedicated pre-admission timing-failure ownership lane. Its semantic payload is:

- one bounded timing-source failure;
- the exact original `RemoteSessionExpectedDeviceAdmissionRequest<D, T>`, by value.

Expected DeviceId remains inspectable from that request. No dispatcher, verifier provider, session identifier or correlation value is cloned, substituted, reconstructed or discarded to manufacture a failure record.

The consumer must be able to recover both the failure and the intact request. A status-only observation is insufficient as the ownership handoff. No automatic Drop, logging side effect or discarded error is proof of completed custody.

The abstract lane is selected here; concrete carrier/callback/result signatures and their propagation are not. The future implementation gate must identify exactly one terminal custodian and prove every error branch reaches it once, without losing the request. It may not claim production readiness while that custodian remains unimplemented.

### Keep failure domains distinct

Timing-source failure happens before AJ construction and is not:

- `DuplicateActiveDevice`;
- a registry, accept, challenge, authentication or binding failure;
- a worker completion;
- a scheduling grant disposal receipt;
- a successful handoff or admission.

Do not coerce it into the current `RemoteSessionRealAdmissionError`, the existing duplicate rejection reason, or the current `on_admission_failure(DeviceId, RemoteSessionRealAdmissionError)` callback by erasing the source cause or intact request.

The existing `on_completion`, `on_rejection` and `on_admission_failure` signatures and behavior remain unchanged. The later propagation gate must review the dedicated timing-failure lane rather than silently overloading these callbacks.

### No fabricated cleanup or replay

A pre-AJ timing failure creates no accepted peer, pending authentication session, authenticated owner or worker. It must not call the later code-5/code-1/code-2 peer-cleanup paths for resources it never acquired.

Any previously completed producer handoff or scheduling receipt remains historical fact; timing failure must not relabel or replay it. No grant reacquisition, retry, requeue, alternate queue, dead-letter sink, restart or recovery policy is selected.

Disposition after the exact failure/request transfer is separately reviewed. This contract does not instruct the future custodian to retry, drop, persist, stop the process or continue automatically. The implementation gate must make the control-flow outcome explicit while preserving current shutdown and active-worker custody.

## Bounded failure semantics and production provenance

The future failure model must distinguish, where applicable:

- acquisition/conversion failure in the explicitly selected timing domain;
- invalid or unavailable challenge/lease policy;
- invalid range ordering or checked arithmetic overflow.

These are required semantic distinctions, not an enum declaration or selection of concrete provider dependencies. Exact variants and source chaining remain part of the reviewed interface layout.

A future provider must establish provenance for the complete bundle and validate it before returning success. No default/epoch substitution, panic/unwrap/expect, saturation, wrapping arithmetic, clamp, cache fallback, stale-value reuse, hidden retry or error flattening may turn failure into a valid-looking bundle.

No challenge duration, lease duration, environment key, configuration file, credential, persistence field or service property is selected.

## Verifier-time separation and preserved invariants

The existing verifier-time relationship remains:

`fn() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>`.

It is a separate custody domain. SM does not reuse that function as a production admission-timing source or map its error into a new admission type. Shared underlying clock provenance, if later selected, must still preserve distinct acquisition and failure roles.

DeviceId remains selector/correlation; PRWM request_id remains transaction correlation. Neither becomes timing, authorization, transport or scheduling authority.

Preserve:

- one capacity-one expected-request channel;
- sole sender, no sender clone, one receiver transfer;
- exact dispatcher provenance and ownership;
- durable capability and requester/rendezvous authority custody;
- current shutdown precedence, active-worker recovery and explicit cleanup;
- bounded authority-free completion observation;
- dormant higher-owner/executable boundary.

No second channel, spare endpoint, authority carrier, sink implementation, listener, readiness publication, network activation, run()/main.rs wiring, service change or deployment is selected.

## Future source ceiling and next gate

Current Rust/source ceiling: `ZERO`.

The semantic fallible interface and intact-request failure custody are selected. This does not yet select an implementable Rust changed-path ceiling.

Next recommended gate: documentation-only exact source-layout and propagation selection for the fallible admission-timing lane. It must inventory the complete forwarding chain from the dormant higher owner through Linux/endpoint wrappers to both preparation call sites, then select:

- exact carrier/error/callback symbols and visibility;
- backward-compatible treatment of existing infallible callers;
- one exact custodian and explicit post-transfer control flow;
- exact changed paths and public/API ceiling;
- propagation without global mechanical signature widening;
- focused tests and unchanged-boundary guards.

Only after that gate is verified and separately authorized may source materialization be considered. Concrete production provider policy/provenance and executable/runtime activation remain additional unselected boundaries.

## Validation requirements

SM must be validated against its own exact final head:

- direct exact SL parent, one commit, ahead 1/behind 0;
- exactly the new contract path and no existing-file replacement;
- all pre-existing tree entries and blob identities preserved;
- source statements checked against the pinned blobs above;
- explicit distinction between current implementation and selected future semantics;
- no unsupported claim that a production timing provider or failure lane already exists;
- successful repository-required exact-head checks reported by their actual outcomes;
- skipped workflows remain SKIPPED, not PASS;
- no inherited SL/SK/Android PASS.

Particular future behavioral obligations are duplicate => zero samples and exact rejection custody; vacant/success => one sample and unchanged request/timing; vacant/failure => one sample, intact request/error custody and zero AJ/network/worker effects; both producer and idle paths => identical preparation law; shutdown/capacity precedence => preserved. These are acceptance requirements for future source work, not tests claimed executed by this docs-only checkpoint.

Immutable evidence, if published, must bind the exact SM commit/tree/blob, predecessor comparison and exact-head CI; freeze UTF-8 bytes/hash/final LF, check canonical uniqueness, read back content/metadata/revisions, and use the established post-publication GitHub closure binding. No evidence-closure status may be claimed before those checks.

## Explicit STOP

After docs-only selection and its validation/evidence workflow: STOP.

No Rust/source/runtime mutation; no concrete provider; no new implemented type, overload or callback; no callback sink; no main/run wiring; no configuration, Cargo, lockfile, workflow, Android, package, service or repository-setting mutation; no merge, ready-for-review conversion, PR closure, branch deletion, deployment, restart, destructive cleanup or history rewrite.

All predecessor contracts, PRs, branches and immutable audits remain unchanged.
