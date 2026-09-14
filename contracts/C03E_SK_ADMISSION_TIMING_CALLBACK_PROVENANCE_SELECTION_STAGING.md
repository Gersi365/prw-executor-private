# C03e-SK — admission timing and callback provenance selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:
`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_STATUS_ONLY_DISPATCHER_FACTORY_HIGHER_OBSERVATION_RUNTIME_INPUT_AWARE_LINUX_OPERATION_HIGHER_OWNER_CAPACITY_ONE_EXPECTED_REQUEST_CHANNEL_ADMISSION_TIMING_CALLBACK_PROVENANCE_SELECTION`

This checkpoint is documentation-only. It freezes the provenance/custody law for the four policy hooks already exposed by the dormant C03e-SJ higher-owner wrapper. It does not select an executable caller, production configuration source, new time source, Rust source-materialization path, or runtime activation boundary.

## Corrective replacement context

This file is the corrective replacement for the earlier un-evidenced C03e-SK selection attempt on branch `phase-152-c03e-sk-admission-timing-callback-provenance-selection` / PR `#627`.

That earlier attempt remains draft/open/unmerged and is not modified, closed, deleted, rebased, reset or rewritten by this checkpoint. Its immutable-evidence publication was intentionally blocked before upload because its predecessor-authority section contained two factual defects: an incorrect C03e-SJ Drive evidence ID and an incorrect claim that no Android run existed for the exact C03e-SJ head.

This replacement keeps the same C03e-SK semantic boundary and corrects only authority/audit metadata needed for a truthful selection record. No additional runtime, source, API or architecture authority is introduced.

## Exact predecessor authority

Authoritative predecessor is evidence-closed C03e-SJ:

- predecessor branch: `phase-152-c03e-sj-higher-owner-capacity-one-expected-request-channel-integration-source`;
- predecessor head: `4b35df19ef1e494ff50c55e282dbdc688f8e8312`;
- predecessor tree: `48fb02995e6b28c580a8d596b0e95ad1523b4067`;
- predecessor parent: exact C03e-SI head `bb93bad58cba7dc55fe427b6e6a05a4a426091d8`;
- predecessor source path: `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`;
- predecessor source blob: `cccefed1192243a16b67c971ca6ba8fe13ae2c82`;
- predecessor PR: `#626`, draft/open/unmerged/mergeable;
- predecessor status binding: `SOURCE MATERIALIZED — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- immutable SJ evidence Drive ID: `1oxAOzlqck9wX2fT0EG0l2sIgJoFW0mb-`;
- immutable SJ evidence title: `C03E_SJ_HIGHER_OWNER_CAPACITY_ONE_EXPECTED_REQUEST_CHANNEL_INTEGRATION_SOURCE_MATERIALIZATION_AUDIT_2026-09-14.md`;
- SJ Rust Validation #1860 — run `34828559011`, job `103926225723`: `SUCCESS`;
- SJ Android Validation #1806 — run `34828559030`, job `103926225382`: `SUCCESS`;
- C02f-AD #1108 — run `34828559055`: `SKIPPED`;
- C02f-AE #1099 — run `34828559077`: `SKIPPED`;
- `SKIPPED` is not PASS.

Integrated `main` was freshly re-read immediately before this corrective contract write and remained:

- head `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`;
- tree `63b8e59ca53797fdea6b95432e16f35eaf473604`.

## Fresh corrective guards

Before this file write:

- no existing branch matched the corrective-replacement branch name;
- no all-state PR matched a C03e-SK corrective replacement;
- the corrective branch was created directly from exact C03e-SJ head `4b35df19ef1e494ff50c55e282dbdc688f8e8312`;
- immediate branch readback showed the exact same SJ head/tree before this file write;
- this contract path did not exist on the corrective branch before this write;
- PR #627 and its original SK branch remained unchanged, draft/open/unmerged, and outside the mutation scope of this corrective checkpoint.

## Exact-current source finding

Exact C03e-SJ source keeps executable caller wiring separately gated. The dormant higher-owner wrapper accepts exactly four caller-supplied policy hooks:

1. `admission_timing: F`;
2. `on_completion: C`;
3. `on_rejection: R`;
4. `on_admission_failure: E`.

No exact-current source finding establishes a unique concrete executable owner, environment variable, systemd credential, configuration file, default implementation, logging sink, telemetry sink, persistence sink, or new production provider for these hooks.

Therefore C03e-SK selects only their interface/custody provenance. It intentionally does not invent a concrete production source.

## Exact hook provenance law

### Admission timing

The selected interface remains exactly `admission_timing: F` with bound:

`F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming + Send + 'static`.

This remains a caller-supplied policy hook. The higher-owner wrapper must not synthesize, default, sample, cache, retry, or derive admission timing from transport identity, verifier time, wall clock, durable authority, requester/rendezvous state, endpoint state, or a new configuration source.

No concrete implementation or production source is selected here.

### Completion observer

The selected interface remains exactly `on_completion: C` with bound:

`C: FnMut(DeviceId, RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffObservationProjection) + Send + 'static`.

The completion observer remains bounded and authority-free. Its `DeviceId` is correlation only. It must not be widened to expose durable capability authority, requester/rendezvous authority, raw request custody, sender/receiver custody, dispatcher internals, raw verifier-time source, transport capability, retry token, continuation, task handle, endpoint authority, or scheduling authority.

No concrete completion sink or executable owner is selected here.

### Rejection observer

The selected interface remains exactly caller supplied as `on_rejection: R` over the existing rejection reason plus the exact request type:

`RemoteSessionExpectedDeviceAdmissionRequest<LinuxAgentProductionRemoteCapabilityDispatcher, fn() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>>`.

This preserves the already-locked dispatcher and fallible verifier-time type relationship. C03e-SK does not widen request, dispatcher, authority, transport, or runtime visibility.

No retry policy, requeue owner, dead-letter queue, persistence target, alternate queue or executable caller is selected here.

### Admission-failure observer

The selected interface remains exactly `on_admission_failure: E` with bound:

`E: FnMut(DeviceId, RemoteSessionRealAdmissionError) + Send + 'static`.

This remains a caller-supplied observation hook. C03e-SK does not select retry, fallback, alternate admission, state repair, degraded success, queue replay, process restart, authority substitution, concrete sink, or executable owner.

## Verifier-time is outside the SK caller-provenance boundary

The C03e-SJ higher-owner wrapper does not accept verifier time as a fifth caller-supplied hook. Its verifier-time type remains fixed internally to:

`fn() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>`.

C03e-SK does not select or replace a verifier-time provider. It must not add a caller-supplied verifier-time argument, sample verifier time at the higher owner, cache a verifier-time value, convert failure to a default, retry a failed sample, flatten `PrwaVerifierSourceError`, or substitute another time source.

Any future verifier-time provenance change requires a fresh separately gated selection checkpoint.

## Dispatcher, authority and channel boundaries remain unchanged

C03e-SK does not select dispatcher construction. Dispatcher provenance remains inside the already-materialized lower SH/SJ runtime-input-aware lineage and the exact production dispatcher type remains `LinuxAgentProductionRemoteCapabilityDispatcher`.

C03e-SK does not expose durable capability authority through any callback. The exact populated `Arc<ProductionDurableCapabilityAuthority>` remains owned by the existing higher-owner/runtime-input path and is passed only through the already-selected SJ -> SH composition.

Expected-request channel custody remains the C03e-SJ law: one bounded capacity-one channel, one sender, one receiver, no sender clone, no spare endpoint, no second channel, no unbounded queue and no alternate queue ownership.

## Concrete executable caller owner remains unselected

No concrete executable caller owner is selected in C03e-SK.

Accordingly:

- `run()` is not selected;
- `main.rs` is not selected;
- no systemd/service/package owner is selected;
- no environment/config/credential source is selected for these hooks;
- no production logging/telemetry/persistence sink is selected;
- no public API widening is selected;
- no startup-error migration is selected;
- no Rust file is authorized for future mutation by this checkpoint.

Future executable caller/source selection is `UNSELECTED — fresh successor selection/audit required`.

## Source-materialization ceiling

C03e-SK authorizes no Rust/source materialization.

Future source ceiling: `UNSELECTED — fresh successor selection required`.

If a later candidate requires any Rust path, public/crate visibility change, new configuration source, new callback type, new time source, new error/result type, second channel, caller wiring, runtime activation, listener/readiness activation, network mutation, package/service mutation or repository configuration change, work must STOP and return to a fresh successor selection/audit before mutation.

## Validation requirements for this corrective replacement

This corrective C03e-SK replacement is valid only if final GitHub topology proves:

- direct exact SJ -> corrective-SK ancestry;
- ahead `1`, behind `0`;
- merge base exact SJ head `4b35df19ef1e494ff50c55e282dbdc688f8e8312`;
- exactly one changed path, this contract under `contracts/`;
- zero Rust/source/runtime/Cargo/lockfile/workflow/Android-source/package/service/deployment/repository-config mutation;
- exact SJ source blob `cccefed1192243a16b67c971ca6ba8fe13ae2c82` remains unchanged on the corrective head.

Any PASS claim must bind only to the exact final corrective SK head. No SJ, original-SK, or historical workflow result may be inherited as corrective-SK validation authority. `SKIPPED` is not PASS. A docs-only corrective head does not require an Android PASS unless an Android workflow actually registers for that exact head.

Draft PR creation, exact-head CI observation, immutable Drive evidence publication, and post-publication GitHub closure binding remain separately gated after this contract materialization.

## Explicit non-actions / STOP

This corrective C03e-SK contract write performs no Rust/source/runtime/API mutation. It does not modify `run()` or `main.rs`; select a concrete executable caller; add a production timing/configuration provider; alter verifier-time provenance; construct a dispatcher; expose durable authority; clone or replace channel endpoints; add retry/requeue/default/cache behavior; add a new error type; activate listener/readiness/network behavior; mutate Cargo/lockfile/workflows/Android/package/service/systemd/repository configuration; mutate `main`; modify/close/delete PR #627 or its branch; create or modify a PR for this corrective branch; publish Drive evidence; merge; deploy; restart; change authentication or database state; convert a PR ready-for-review; close a PR; delete a branch; clean accidental refs; reset/rebase/squash/force-update; or rewrite history.

After this docs-only corrective contract materialization: `STOP`. Exact-head validation and draft-PR creation remain separately gated actions.
