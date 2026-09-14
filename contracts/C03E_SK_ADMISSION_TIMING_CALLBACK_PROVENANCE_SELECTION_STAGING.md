# C03e-SK — admission timing and callback provenance selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:
`PRODUCTION_DURABLE_POST_AUTH_FALLIBLE_VERIFIER_TIME_REQUESTER_RENDEZVOUS_EXPECTED_DEVICE_ADMISSION_STATUS_ONLY_DISPATCHER_FACTORY_HIGHER_OBSERVATION_RUNTIME_INPUT_AWARE_LINUX_OPERATION_HIGHER_OWNER_CAPACITY_ONE_EXPECTED_REQUEST_CHANNEL_ADMISSION_TIMING_CALLBACK_PROVENANCE_SELECTION`

This checkpoint is documentation-only. It freezes the provenance/custody law for the four policy hooks already exposed by the dormant C03e-SJ higher-owner wrapper. It does not select an executable caller, production configuration source, new time source, Rust source-materialization path, or runtime activation boundary.

## Exact predecessor authority

Authoritative predecessor is evidence-closed C03e-SJ:

- predecessor branch: `phase-152-c03e-sj-higher-owner-capacity-one-expected-request-channel-integration-source`;
- predecessor head: `4b35df19ef1e494ff50c55e282dbdc688f8e8312`;
- predecessor tree: `48fb02995e6b28c580a8d596b0e95ad1523b4067`;
- predecessor parent: exact C03e-SI head `bb93bad58cba7dc55fe427b6e6a05a4a426091d8`;
- predecessor source path: `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`;
- predecessor source blob: `cccefed1192243a16b67c971ca6ba8fe13ae2c82`;
- predecessor PR: `#626`, draft/open/unmerged;
- predecessor status binding: `SOURCE MATERIALIZED — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- immutable SJ evidence Drive ID: `1WqBhPM8gFJJxFLfE5ZMn-3OIR1pI8VKG`;
- SJ Rust Validation #1860 succeeded on exact predecessor head;
- C02f-AD #1108 and C02f-AE #1099 were `SKIPPED`, not PASS;
- no Android run was registered for the exact SJ head and no Android PASS is inherited or claimed.

Integrated `main` was freshly re-read immediately before this SK contract write and remained:

- head `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`;
- tree `63b8e59ca53797fdea6b95432e16f35eaf473604`.

Fresh duplicate guards before this contract write confirmed:

- the proposed SK branch name did not already exist;
- no all-state PR matched the proposed C03e-SK admission-timing/callback-provenance boundary;
- this exact SK contract path did not exist at the exact SJ head;
- the SK branch was then created directly from exact SJ head `4b35df19ef1e494ff50c55e282dbdc688f8e8312` and immediately re-read at the same head/tree before this file write.

## Exact-current source finding

Exact SJ source keeps executable caller wiring separately gated. The C03e-SJ wrapper remains dormant and accepts exactly four caller-supplied policy hooks:

1. `admission_timing: F`;
2. `on_completion: C`;
3. `on_rejection: R`;
4. `on_admission_failure: E`.

The exact current higher-owner source also explicitly preserves earlier population wrappers that do not select admission timing or callbacks. No exact-current source finding establishes a concrete executable owner, environment variable, systemd credential, configuration file, default implementation, or new production provider for these four hooks.

Therefore C03e-SK selects only their interface/custody provenance. It intentionally does not invent a concrete production source.

## Exact hook provenance law

### 1. Admission timing

The selected interface remains exactly:

`admission_timing: F`

with bound:

`F: FnMut(&DeviceId) -> RemoteSessionRealAdmissionTiming + Send + 'static`.

C03e-SK classifies this as a caller-supplied policy hook. The dormant higher-owner wrapper must receive it from a later separately selected caller; it must not synthesize, default, sample, cache, retry, or derive admission timing from transport identity, verifier time, wall clock, durable authority, requester/rendezvous state, or endpoint state.

No concrete implementation or configuration source is selected here.

### 2. Completion observer

The selected interface remains exactly:

`on_completion: C`

with bound:

`C: FnMut(DeviceId, RemoteSessionExpectedDeviceAdmissionFallibleVerifierTimeHandoffObservationProjection) + Send + 'static`.

The completion observer remains bounded and authority-free. Its `DeviceId` is correlation only. The observation projection must not be widened to expose durable capability authority, requester/rendezvous authority, raw request custody, sender/receiver custody, dispatcher internals, raw verifier-time source, transport capability, retry token, continuation, task handle, endpoint authority, or scheduling authority.

No concrete completion sink, telemetry backend, logging target, persistence target, or executable owner is selected here.

### 3. Rejection observer

The selected interface remains exactly caller supplied:

`on_rejection: R`

with the existing rejection reason plus the exact request type:

`RemoteSessionExpectedDeviceAdmissionRequest<LinuxAgentProductionRemoteCapabilityDispatcher, fn() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>>`.

This preserves the already-locked dispatcher and fallible verifier-time type relationship. The rejection callback may observe only the exact values already exposed by the existing interface; C03e-SK does not widen request, dispatcher, authority, transport, or runtime visibility.

No concrete rejection sink, retry policy, requeue owner, dead-letter queue, persistence target, or executable caller is selected here.

### 4. Admission-failure observer

The selected interface remains exactly:

`on_admission_failure: E`

with bound:

`E: FnMut(DeviceId, RemoteSessionRealAdmissionError) + Send + 'static`.

This remains a caller-supplied observation hook. C03e-SK does not select retry, fallback, alternate admission, state repair, degraded success, queue replay, process restart, or authority substitution behavior.

No concrete admission-failure sink or executable owner is selected here.

## Verifier-time is explicitly outside the SK caller provenance boundary

The C03e-SJ higher-owner wrapper does not accept verifier time as a fifth caller-supplied hook. Its verifier-time type remains fixed internally to:

`fn() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>`.

C03e-SK therefore does not select or replace a verifier-time provider. It must not add a caller-supplied verifier-time argument, sample verifier time at the higher owner, cache a verifier-time value, convert failure to a default, retry a failed sample, flatten `PrwaVerifierSourceError`, or substitute another time source.

Any future change to verifier-time provenance is outside this contract and requires a fresh separately gated selection checkpoint.

## Dispatcher and authority boundaries remain unchanged

C03e-SK does not select dispatcher construction. Dispatcher provenance remains inside the already-materialized lower SH/SJ runtime-input-aware lineage and the exact production dispatcher type remains `LinuxAgentProductionRemoteCapabilityDispatcher`.

C03e-SK does not expose durable capability authority through any callback. The exact populated `Arc<ProductionDurableCapabilityAuthority>` remains owned by the existing higher-owner/runtime-input path and is passed only through the already-selected SJ -> SH composition.

Expected-request channel custody remains the C03e-SJ law: one bounded capacity-one channel, one sender, one receiver, no sender clone, no spare endpoint, no second channel, and no alternate queue ownership.

## Concrete executable caller owner remains unselected

No concrete executable caller owner is selected in C03e-SK.

This is deliberate. The exact current source establishes the four generic policy-hook interfaces but does not establish a unique production owner/source that may be bound without introducing a new architecture or policy decision.

Accordingly:

- `run()` is not selected;
- `main.rs` is not selected;
- no systemd/service/package owner is selected;
- no environment/config/credential source is selected for these hooks;
- no production logging/telemetry/persistence sink is selected;
- no Rust file is authorized for future mutation by this checkpoint.

Future executable caller/source selection is `UNSELECTED — fresh successor selection/audit required`.

A successor must first prove the concrete owner and exact source ceiling from fresh repository state. It may not infer authorization from this SK contract.

## Source-materialization ceiling

C03e-SK authorizes no Rust/source materialization.

Future source ceiling: `UNSELECTED — fresh successor selection required`.

If a later candidate requires any Rust path, public/crate visibility change, new configuration source, new callback type, new time source, new error/result type, second channel, caller wiring, or runtime activation, work must STOP and return to a fresh successor selection/audit before mutation.

## Validation requirements for this selection checkpoint

C03e-SK is valid only if final GitHub topology proves:

- direct exact SJ -> SK ancestry;
- ahead `1`, behind `0`;
- merge base exact SJ head `4b35df19ef1e494ff50c55e282dbdc688f8e8312`;
- exactly one changed path, this contract under `contracts/`;
- zero Rust/source/runtime/Cargo/lockfile/workflow/Android-source/package/service/deployment/repository-config mutation;
- exact SJ source blob `cccefed1192243a16b67c971ca6ba8fe13ae2c82` remains unchanged on SK.

Any PASS claim must bind only to the exact final SK head. No SJ or historical workflow result may be inherited as SK validation authority. `SKIPPED` is not PASS. A docs-only SK head does not require an Android PASS unless an Android workflow actually registers for that exact head.

Draft PR creation, exact-head CI observation, immutable Drive evidence publication, and post-publication GitHub closure binding are separately gated after this contract materialization.

## Explicit non-actions / STOP

This C03e-SK contract write performs no Rust/source/runtime/API mutation. It does not modify `run()` or `main.rs`; select a concrete executable caller; add a production timing/configuration provider; alter verifier-time provenance; construct a dispatcher; expose durable authority; clone or replace channel endpoints; add retry/requeue/default/cache behavior; add a new error type; activate listener/readiness/network behavior; mutate Cargo/lockfile/workflows/Android/package/service/systemd/repository configuration; mutate `main`; create or modify a PR; publish Drive evidence; merge; deploy; restart; change authentication or database state; convert a PR ready-for-review; close a PR; delete a branch; clean accidental refs; reset/rebase/squash/force-update; or rewrite history.

After this docs-only contract materialization: `STOP`. Exact-head validation and draft-PR creation remain separately gated actions.
