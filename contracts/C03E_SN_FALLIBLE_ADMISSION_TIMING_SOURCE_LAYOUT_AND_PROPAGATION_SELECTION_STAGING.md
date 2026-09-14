# C03e-SN — fallible admission-timing source layout and propagation selection

Status: `SELECTION — VALIDATION PENDING`

Boundary:
`FALLIBLE_ADMISSION_TIMING_SOURCE_LAYOUT_AND_PROPAGATION_SELECTION`

Selection result:
`SIX_PATH_DORMANT_SOURCE_LAYOUT_SELECTED / CRATE_ONLY_TIMING_ERROR_AND_INTACT_REQUEST_CARRIER / SEPARATE_CALLER_OWNED_FAILURE_CUSTODIAN / BOTH_REQUEST_PATHS_SHARE_FALLIBLE_PREPARATION / INFALLIBLE_APIS_PRESERVED / CONCRETE_PROVIDER_AND_EXECUTABLE_ACTIVATION_UNSELECTED`

This checkpoint is documentation-only. It completes the source-layout/propagation selection requested by C03e-SM. It selects exact future symbols, visibility, ownership, control flow, compatibility, changed paths and validation obligations. No Rust/source/runtime implementation is performed by this contract.

## Exact predecessor authority

Authoritative predecessor: C03e-SM PR #630, draft/open/unmerged, with verified post-publication status `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`.

- repository: `Gersi365/prw-executor-private`, stable ID `1334911207`;
- predecessor branch: `phase-152-c03e-sm-production-admission-timing-source-interface-failure-custody-selection`;
- predecessor head: `46bdf72afd9b0a7144741f91d4175c8d6783ec38`;
- predecessor tree: `858513c8af6906c10599ecd5fb7603e738ea29d3`;
- predecessor contract: `contracts/C03E_SM_PRODUCTION_ADMISSION_TIMING_SOURCE_INTERFACE_FAILURE_CUSTODY_SELECTION_STAGING.md`;
- predecessor contract blob: `12e6a1956a48d4802d59d1ed54a72215dc780bf3`;
- direct SL -> SM: one commit, ahead 1, behind 0, one new docs path, +241/-0;
- exact SL parent: `995dfcdc567ef24a4616aca3e379f80878999daa`.

Fresh main remains `a7ffafd6a6d5a032dd8290eec24df1349bade6cc`, tree `63b8e59ca53797fdea6b95432e16f35eaf473604`. Main is not the continuation base and is not changed.

SM Rust Validation #1864, run `34844989301`, job `103978649223`, was re-read as SUCCESS including dependency graph, formatting, Clippy, tests and build. C02f-AD `34844989850` and C02f-AE `34844989303` remain SKIPPED. No Android PASS is claimed. These are predecessor checks only.

Canonical immutable SM evidence:

- title: `C03E_SM_PRODUCTION_ADMISSION_TIMING_SOURCE_INTERFACE_FAILURE_CUSTODY_SELECTION_AUDIT_2026-09-14.md`;
- Drive ID: `1wRRYhUIZJCL8uw4GxLVHDify2kxee2Wx`;
- parent: `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
- MIME: `text/markdown`;
- bytes: `12482`;
- SHA-256: `1707d5eb7dd9bea20911401975f7deccd672279571ce3124a85eed9c35e2366a`;
- final LF: true;
- revision: `0Bz5eMiLa5v9xVW05WVBmeUNqem5jNW50RzlTVmlpMzk1TUxFPQ`;
- previous revision: null.

Fresh metadata, exact-title canonical uniqueness, revision history and complete reconstructed UTF-8 text/hash agree. The immutable audit intentionally retains publication-pending status; live PR #630 is the post-publication closure binding.

## Current mutation ceiling and successor guards

Current checkpoint ceiling: exactly one new documentation path:

`contracts/C03E_SN_FALLIBLE_ADMISSION_TIMING_SOURCE_LAYOUT_AND_PROPAGATION_SELECTION_STAGING.md`

No existing contract is replaced. The six-path source ceiling below is a future implementation selection and is not permission to change those paths during SN.

Before mutation, refresh exact SM ref/PR, main, recent and all-state successor PRs, matching branch namespace, target-file absence and canonical Drive evidence. An existing or conflicting successor requires re-audit rather than duplicate work.

Expected SM -> SN topology: one direct commit, ahead 1, behind 0, merge base exact SM, one added contract, zero Rust/source/runtime/configuration change.

## Pinned six-path future source ceiling

All existing hashes below are from exact SM. P1 does not exist there.

| ID | Exact future source path | Current authority | Only selected future role |
| --- | --- | --- | --- |
| P0 | `crates/prw-agent/src/remote_session_capability_runtime.rs` | `2b6a0fd693f2ddacec608018a4db811cc36ef4c0` | Declare the private child module and crate-only re-exports. No public export changes. |
| P1 | `crates/prw-agent/src/remote_session_capability_runtime/admission_timing_failure.rs` | new path; absent at SM | Add only bounded source-error and intact-request failure-carrier types and their unit tests. |
| P2 | `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs` | `d50e423d01b4161e244c075d59a85fd83533cf62` | Add pure fallible preparation, shared result-capable driver internals, compatible infallible entry, fallible executor/collection siblings and both-path tests. |
| P3 | `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs` | `881846753f51bdf94cff32ff0f9649dbcf50a80f` | Add three forwarding siblings for higher observation, concrete expected-device producer and generic scheduling producer; preserve receipt projection and endpoint custody. |
| P4 | `crates/prw-agent/src/linux_bootstrap.rs` | `b5d236d3c0264ea8870e90af96c76294b50d0fc1` | Add one runtime-input-aware Linux sibling; capture the injected timing-failure custodian beside the unchanged input bundle. |
| P5 | `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs` | `cccefed1192243a16b67c971ca6ba8fe13ae2c82` | Add one dormant higher-owner sibling; preserve channel construction/population and pass the failure custodian separately. |

Future implementation ceiling: six Rust paths, five existing paths modified and one new path added. Tests stay inside these selected modules. No seventh path, manifest, dependency, lockfile, build script, workflow or external test fixture is implied.

P0 declares `mod admission_timing_failure;` and re-exports its two selected types with `pub(crate) use`. No `lib.rs`/main.rs registration or public library export is needed. Descendant modules may import through the existing `crate::remote_session_capability_runtime` scope.

Source implementations must stop and reselect if the six paths cannot contain the complete change. Do not silently widen module visibility, add paths or omit one request path to fit the ceiling.

## Exact-current graph and why population stays unchanged

The existing dormant SJ wrapper accepts four caller-supplied hooks: infallible admission timing, completion, duplicate rejection and AJ admission failure. The configured population chain carries generic F by value without calling it or imposing its callable result bound.

The timing pass-through chain is:

1. `linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_inputs_from_configured_production_sources`;
2. `linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_inputs_from_production_sources_with_explicit_nonzero_capacity`;
3. `linux_agent_production_durable_reachability_requester_policy_remote_process_operation_inputs_from_production_sources`;
4. `linux_agent_production_durable_reachability_remote_process_operation_inputs_from_production_sources_with_fail_closed_current_capability_authority`;
5. `linux_agent_production_durable_reachability_remote_process_operation_inputs_from_production_sources_with_session_authentication`;
6. `linux_agent_production_durable_reachability_remote_process_operation_inputs_from_production_sources`;
7. `linux_agent_remote_process_operation_inputs_from_production_worker_limit`;
8. `linux_agent_remote_process_operation_inputs_from_production_bind_addr`;
9. `LinuxAgentRemoteProcessOperationInputs::new`.

After population, existing same-custody wrappers retain that exact F. The final requester/rendezvous aggregate splits once into requester/rendezvous inputs plus the exact durable capability authority. No intermediate F signature or aggregate field needs to change to carry a fallible closure.

The new failure custodian is a separate argument held by the new higher-owner wrapper and then captured by the new Linux operation. It is not added to these population records or passed through their constructors.

Current preparation lives in:
`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`,
blob `304513eaf4ac72720a96c265f72dd8a56f5ac87f`.

Its existing `prepare_expected_request` and all callers outside the selected P2 fallible-verifier producer driver remain unchanged. SN selects a new local P2 preparation helper for the result-capable path; it does not globally change historical admission semantics.

P2 contains two request arms requiring the same preparation: the idle driver request arm and the pending cooperative producer request arm. P3 adds three forwarding layers between the Linux operation and executor adapter. All of those layers are explicitly covered below.

## Selected types in P1

### RemoteSessionAdmissionTimingSourceError<Cause>

Select a crate-visible, non-exhaustive enum with these exact variants:

- `Acquisition(Cause)`: source observation/conversion failure;
- `Policy(Cause)`: unavailable or invalid source policy, retaining its cause;
- `InvalidChallengeValidity`: invalid challenge-range ordering;
- `InvalidApplicationLease`: invalid application-lease range ordering;
- `ArithmeticOverflow`: checked timing arithmetic failed.

Select no automatic From conversion from verifier errors. A future concrete provider must explicitly map its source domain. The enum selects representation, not a clock, duration, policy source or provider implementation.

Implement bounded phase-oriented Display and `std::error::Error` when Cause implements Error + 'static. Error::source returns the exact Cause for Acquisition and Policy and None for the three structural variants. No formatted-string replacement of a concrete Cause, boxed untyped authority bag, request fields or secret-bearing Debug output is selected.

Implement Debug using only the bounded variant label; do not render the retained Cause or any request data. Do not require Clone or Copy. At the production wrapper boundary, Cause must implement `std::error::Error + Send + 'static`; no Sync bound is added merely for convenience.

### RemoteSessionAdmissionTimingFailure<D, T, TimingError>

Select a crate-visible, non-cloneable carrier with private fields:

- `error: TimingError`;
- `request: RemoteSessionExpectedDeviceAdmissionRequest<D, T>`.

Select exactly these crate-visible operations:

- `new(error, request) -> Self`;
- `error(&self) -> &TimingError`;
- `request(&self) -> &RemoteSessionExpectedDeviceAdmissionRequest<D, T>`;
- `into_parts(self) -> (TimingError, RemoteSessionExpectedDeviceAdmissionRequest<D, T>)`.

The constructor and consuming decomposition only move values. They perform no timing acquisition, request decomposition, dispatcher/verifier call, validation, Drop-based finalization or I/O.

The carrier imposes no Clone/Copy/Debug/Error bound on D, T or TimingError merely to store them. In particular, do not derive Debug for the owned request and accidentally require or expose dispatcher/verifier internals.

Production fallible siblings use this carrier with TimingError = `RemoteSessionAdmissionTimingSourceError<Cause>`. Shared private compatibility internals may use TimingError = `std::convert::Infallible`.

These are selected future definitions. No such enum/carrier is claimed present at the SN head.

## Exact fallible callback and custody signatures

For production fallible siblings, select:

`F: FnMut(&DeviceId) -> Result<RemoteSessionRealAdmissionTiming, RemoteSessionAdmissionTimingSourceError<Cause>>`

and the one separate failure custodian:

`K: FnMut(RemoteSessionAdmissionTimingFailure<D, T, RemoteSessionAdmissionTimingSourceError<Cause>>)`.

K returns unit. It receives the complete carrier by value exactly once per timing failure and becomes the sole custodian of the error and original request. The source F only borrows DeviceId as selector; it never owns or inspects the request, dispatcher, verifier, sender, authority or scheduling grant.

At S0/S1, F and K also require Send + 'static because the remote operation owns them across the process companion boundary. Preserve the existing D/T/P/C/R/E bounds. Inner synchronous executor and forwarding scopes introduce only bounds needed by their actual custody; they must not broaden requirements on existing infallible APIs.

S0/S1 specialize D to `LinuxAgentProductionRemoteCapabilityDispatcher` and T to the exact existing verifier function pointer. The verifier relationship remains:

`fn() -> Result<u64, prw_session::prwa_verifier_source::PrwaVerifierSourceError>`.

No new verifier error lane or adaptation is selected.

K is a caller-supplied ownership sink. Its concrete production implementation is unselected. This checkpoint fixes the ownership transfer and the driver's post-return behavior, not logging, persistence, retry, requeue or destructive disposition inside a production sink.

Normal K return acknowledges completed transfer to the custodian, not destruction of the request or successful admission. A future executable integration must select a concrete custodian and prove its eventual disposition. Dormant source materialization may accept K without supplying that executable sink.

## Exact forwarding sibling names

For every row below, the selected NEW symbol is the complete existing symbol followed by the literal suffix `_with_fallible_admission_timing`. This naming rule is normative; it does not rename or replace the existing symbol. P3 names are methods on RemoteSessionEndpointLifecycleRuntime; P2 names are methods on RemoteSessionExecutorRuntime.

| ID | Path | Existing symbol to which the suffix is appended | New visibility | New call target |
| --- | --- | --- | --- | --- |
| S0 | P5 | `run_with_production_durable_reachability_requester_rendezvous_fallible_verifier_time_expected_device_admission_remote_process_companion_from_configured_production_sources` | `pub(crate)` | S1 after unchanged configured population |
| S1 | P4 | `run_with_production_reachability_requester_rendezvous_fallible_verifier_time_expected_device_admission_remote_process_companion` | `pub(crate)` | S2 inside unchanged remote operation |
| S2 | P3 | `drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_fallible_verifier_time_expected_device_admission_producer_with_higher_observation_projection` | `pub(crate)` | S3 with unchanged receipt projector |
| S3 | P3 | `drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_fallible_verifier_time_expected_device_admission_producer` | `private method` | S4 with unchanged concrete producer and suppression mapper |
| S4 | P3 | `drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_fallible_verifier_time_scheduling_producer` | `pub(super)` | S5 with exact consumed endpoint owner |
| S5 | P2 | `drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_fallible_verifier_time_scheduling_producer` | `pub(in super::super::super::super)` | S6, then unchanged close/wait_idle |
| S6 | P2 | `drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_fallible_verifier_time_scheduling_producer` | `pub(in super::super::super)` | shared result-capable collection core |

Every new sibling preserves the original ordered inputs and output type, changes only its F bound to the selected Result shape, adds generic Cause and K, and appends `on_timing_failure: K` as the final ordinary argument. F remains named `admission_timing`. The existing C/R/E or O/R/E callbacks keep their types and meaning.

S1 retains the original input aggregate carrying F; its final new argument is K after the original sender argument. No new field or generic parameter is added to that aggregate.

S0 accepts F, C, R, E, K. It constructs and consumes one existing capacity-one channel, calls configured population once with the fallible F, and invokes S1 only on successful population, passing K separately. Existing ConfiguredPopulation and Bootstrap error classifications are unchanged. No timing decision occurs during population.

S1 preserves `with_initial_runtime_inputs`, exact dispatcher-factory capture and `run_with_remote_process_companion_inputs`. The closure captures K once and passes it through the lifecycle call to S2. Do not duplicate runtime-input acquisition or construct a dispatcher early.

S2 projects each existing receipt once with the existing projector before invoking O. Timing failure is sent to K, never to O or a forged receipt.

S3 retains the exact lending concrete producer, borrowed dispatcher factory, borrowed sole sender, and shutdown-suppression mapper. K is forwarded beside these values; it is not captured by or exposed to the producer.

S4 consumes the same endpoint owner once and transfers the existing executor/transport/shutdown values to S5.

S5 calls S6 once, then reproduces the existing endpoint close followed by wait_idle on the retained executor, returning the same collection result. Timing failure does not become a collection-configuration error or bypass cleanup.

S6 delegates to the shared result-capable driver internals described next.

## Shared core and compatibility selection in P2

Select these exact private names:

- `prepare_expected_request_with_timing_result`;
- `drive_pending_cooperative_fallible_verifier_time_scheduling_producer_with_timing_result`;
- `drive_recoverable_repeated_real_remote_admission_collection_with_production_durable_fallible_verifier_time_scheduling_producer_with_timing_result` as a private RemoteSessionExecutorRuntime method.

The result-capable core uses F returning `Result<RemoteSessionRealAdmissionTiming, TimingError>` and K receiving `RemoteSessionAdmissionTimingFailure<D, T, TimingError>`. TimingError is generic internally so the existing infallible entry can instantiate it with Infallible. Do not erase error types or convert the fallible production path into infallible timing.

Move only the currently selected P2 fallible-verifier cooperative driver body and its private pending-producer body into these shared result-capable internals. Preserve poll functions, event priority, active-map operations, producer lifetime, receipt handling, grant disposition, in-flight admission finish/drain and shutdown operations. No second independently maintained copy of the large driver state machine is selected.

Keep the existing externally visible infallible collection method's signature exactly unchanged. Its body delegates to the shared core using an explicit success lift:

`|device_id| Ok::<RemoteSessionRealAdmissionTiming, Infallible>(admission_timing(device_id))`.

Its timing-failure consumer exhaustively eliminates Infallible after consuming the carrier through into_parts. It must not use panic, unreachable!, unwrap, expect, default or a runtime fallback. This lifts an already-infallible caller into Result; it never converts a fallible source to infallible.

Other historical infallible endpoint/higher-owner APIs retain their existing signatures and call graph. Only their selected collection entry's internal implementation is shared. No repository-wide FnMut bound replacement, new bound on an existing public API or mechanical caller migration is allowed.

S6 uses the same core with the actual source-error type and actual K. The pending helper is invoked with the same borrowed F and K from every continuation point, including the path where AJ finishes while a producer future is still pending.

## Pure preparation law

The new private helper takes the existing active map by shared reference, owns one request, and borrows F, R and K mutably. Its return shape remains:

`Option<(RemoteSessionExpectedDeviceAdmissionRequest<D, T>, RemoteSessionRealAdmissionTiming)>`.

None is only an internal control-flow result after an explicit distinct ownership callback. It must never substitute for delivery of the timing cause or intact request.

Ordered behavior:

1. Read/clone only the expected DeviceId selector as already done by preparation.
2. If active contains that DeviceId, invoke R once with DuplicateActiveDevice and the exact request, return None, and call neither F nor K.
3. Otherwise invoke F exactly once.
4. Ok(timing): return Some with the original request and complete timing; invoke neither R nor K.
5. Err(error): construct one failure carrier from the exact error and still-intact request; invoke K once; return None.

Both the idle request arm and pending producer request arm call this one helper before request.into_parts, timing.into_parts and AJ future construction. No asynchronous suspension or producer polling occurs inside preparation.

The helper does not sample time, validate a concrete policy or silently repair a returned bundle itself; the selected future production source owns those checks before returning Ok. RemoteSessionRealAdmissionTiming and its constructor remain unchanged.

## Exact post-failure control flow and terminal custody

In the idle request arm, after K returns normally, continue the supervisor loop and re-enter the existing shutdown/completion/capacity/request poll. The failed request is no longer owned by the driver and is never put back in the channel.

In the pending producer request arm, after K returns normally, continue the same pending-producer loop with the same pinned producer future, same receiver, same borrowed F/K and same request-source-open flag. Do not cancel, restart, resolve early or drop that producer because timing failed. Its eventual receipt is still observed exactly once under the existing law.

This is continuation with independent work, not retry of the failed request. Do not invoke K twice, invoke R/E/O for the same timing failure, add a spare sender/channel, replay a scheduling receipt, recover an already handed-off grant or fabricate success.

Shutdown retains its existing poll precedence. Because acquisition/custody handoff are synchronous, a shutdown that becomes ready during them is observed at the next existing poll. No new asynchronous cancellation point, interruptible clock or panic-catching policy is selected.

A timing failure creates no AJ future, accepted peer, pending logical-authentication session, authenticated owner or worker for that request. Do not execute later peer cleanup against uncreated resources. Already-active workers retain their current recovery/disposition behavior.

The driver holds no copy of the failed request after K receives it. Later production sink behavior requires its own selection; ordinary Drop is not evidence that external cleanup or durable handling completed.

## Protected source and behavioral boundaries

Outside the six future paths, preserve all blobs exactly. In particular:

- `remote_session_executor_runtime.rs`, blob `1539b6b9a08bf18883d7a16022f15f7c240eaf08`: request/timing/rejection/AJ-failure definitions unchanged;
- `repeated_real_admission_requester_aware_persistent_fl_integration.rs`, blob `304513eaf4ac72720a96c265f72dd8a56f5ac87f`: historical preparation and worker helpers unchanged;
- `real_remote_admission_transaction.rs`, blob `812b56e9b948a41f2f746eb406ba24567efbd528`: AJ transaction and peer cleanup unchanged;
- `prwa_verifier_source.rs`, blob `e34c3d452b9fd5c9787abbf1f36106e3b97e3b0b`: verifier source unchanged;
- `crates/prw-agent/src/main.rs`, blob `db6b8028c6df100a961a0fb5818347bea2fdc5c1`: executable unchanged.

No concrete wall clock, challenge/lease duration, configuration key, credential, callback sink, source retry/fallback/cache, later authentication-time resampling or executable caller is selected.

DeviceId is selector only; request_id is transaction correlation only. Durable capability, requester/rendezvous, scheduling, transport and timing authority remain distinct. Do not derive timing from identities, endpoints, grants or previous callback results.

## Future implementation acceptance tests

Tests belong inside P1/P2/P3/P4/P5 as applicable; no new external test path is selected. They must exercise behavior rather than merely repeat type definitions.

| Obligation | Required evidence |
| --- | --- |
| Error causality | Acquisition/Policy retain a typed sentinel cause through Error::source; structural variants remain distinct. |
| Intact carrier | Non-cloneable dispatcher/verifier sentinels and exact DeviceId/SessionId/request_id survive new/borrow/into_parts; no early call or drop. |
| Duplicate preflight | R once, F/K zero; exact original request recovered. |
| Vacant success | F once, R/K zero; exact request and all three timing fields retained. |
| Vacant failure | F/K once, R/E/O zero; exact error/request received; no AJ construction or worker/transport effect. |
| Both request arms | Idle and pending producer arms exercise the same failure law; testing preparation alone is insufficient for final propagation validation. |
| Producer custody | Timing failure leaves the same producer pending, produces no synthetic receipt, and its eventual real receipt is observed once. |
| Continued work | After a failure and normal K return, a distinct later eligible request can proceed; the failed request is never resampled/requeued. |
| Shutdown/capacity | Existing shutdown-first and full-capacity-no-receive behavior remains; failure adds no polling or cancellation point. |
| Infallible compatibility | Existing typed callers compile unchanged; representative success/duplicate/shutdown behavior matches the pre-extraction driver. |
| Forwarding | Each selected sibling forwards F/K and owned/borrowed inputs once; construction/population never invokes timing or K. |
| Lifecycle/receipt | Existing higher observation projection and endpoint close/wait_idle order are unchanged. |

Use non-network injected/sentinel seams for the new failure behavior. Do not activate production listeners, credentials, services or real peers merely to validate the dormant lane. The normal repository-required exact-final-head checks still apply.

If an implementation cannot demonstrate both request arms and the pending producer/receipt law inside the selected layout, do not claim full propagation validation; correct within the ceiling or return to selection.

## SN documentation validation and evidence closure

For this docs-only checkpoint, verify:

- fresh SM authority and no competing successor;
- one direct SM -> SN commit and exactly this new contract;
- all existing files/blobs/modes unchanged;
- source graph, current signatures and module visibility grounded in pinned SM files;
- P1 absence and all five existing future-path blob identities;
- exact names/visibility/types/custodian/control flow and six-path future ceiling;
- explicit separation of current docs-only mutation from future source selection;
- complete GitHub content readback and independently recomputed blob identity;
- repository-required CI for the exact final SN head, not inherited SM CI;
- honest reporting of skipped or absent workflows.

Future source tests above are selected acceptance obligations, not tests claimed executed against a currently implemented lane.

Publish one canonical immutable audit only after freezing final UTF-8 bytes, byte count/hash/final LF, checking title collisions and binding exact head/tree/blob/CI. Verify metadata, complete readback identity, unique title and revision lineage. Preserve the frozen pre-publication audit status and record verified closure in the SN draft PR body.

## Remaining gate and STOP

SN selects the complete six-path dormant source layout and propagation plan. After documentation validation and evidence closure, the next recommended action is separately authorized source materialization within that exact ceiling, with the acceptance tests above.

The user authorized only the documentation selection in this checkpoint. No Rust/source/runtime mutation follows automatically from SN closure.

Concrete production timing-provider policy/provenance, executable custodian/sink, main/run wiring and runtime activation remain separately unselected even after a future dormant source implementation.

STOP after this docs-only checkpoint. No merge, deployment, restart, service/configuration/repository-setting change, ready-for-review conversion, PR closure, branch deletion, destructive cleanup or history rewrite. Preserve all prior contracts, PRs, branches, immutable audits and synced Project Sources.
