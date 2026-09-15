# C03e-SW — production-reachability wrapper propagation corrective selection

Status: `CORRECTIVE SELECTION — VALIDATION PENDING`

Boundary:
`FALLIBLE_ADMISSION_TIMING_PRODUCTION_REACHABILITY_WRAPPER_PROPAGATION_CORRECTIVE_SELECTION`

Selection result:
`SEVEN_PATH_SOURCE_CEILING_REQUIRED / P6_PRODUCTION_REACHABILITY_ENDPOINT_LIFECYCLE_FORWARDING_SIBLING_REQUIRED / C03E_SN_SOURCE_SEMANTICS_UNCHANGED / S0_S6_NAMES_AND_MEANING_UNCHANGED / PRODUCTION_REACHABILITY_CUSTODY_PRESERVED / LOCAL_SV_ATTEMPT_NONCANONICAL / NO_SOURCE_MUTATION / NO_RUNTIME_ACTIVATION`

C03e-SW is documentation-only. It corrects one propagation-layer omission discovered only after the C03e-SU-selected connected local execution surface became available and a noncanonical local C03e-SV compile attempt exercised the exact selected source graph. The omission is structural: the Linux production operation owns `ProductionReachabilityEndpointLifecycleRuntime`, not the raw `RemoteSessionEndpointLifecycleRuntime` on which C03e-SN selected S2. The production wrapper retains distinct durable reachability custody and exposes no extraction seam. C03e-SW adds only the missing production-wrapper forwarding path to the future source ceiling.

## 1. Authoritative predecessor

Authoritative predecessor: evidence-closed C03e-SU PR #635.

- repository: `Gersi365/prw-executor-private`, stable ID `1334911207`;
- predecessor branch: `phase-152-c03e-su-connected-local-patch-execution-surface-corrective-selection`;
- predecessor exact head: `80d9debde00bac5b64786a3f6b7c82aa4f0b637d`;
- predecessor exact tree: `10bd210cfbdfa65fcf1bdb03ea4eb2fa4d53c2ff`;
- predecessor contract blob: `2dbec1deaee3c9aa9ce3f4155306f631202ef5d7`;
- predecessor PR remains draft/open/unmerged;
- predecessor status: `CORRECTIVE SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`.

Canonical C03e-SU Drive evidence:
`C03E_SU_CONNECTED_LOCAL_PATCH_EXECUTION_SURFACE_CORRECTIVE_SELECTION_AUDIT_2026-09-14.md`

- Drive ID: `129F6l0ij1jP-a99GZVKPSQ1uMzE5acE6`;
- canonical parent: `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
- bytes: `11442`;
- SHA-256: `98e8ec86eb867509b16a72347db49d89f54eca626cd2ff3fbd001e85834a59ea`;
- one revision, no predecessor revision.

## 2. Noncanonical local SV diagnostic attempt

After the connected local surface became available, a local worktree was created from exact SU solely to execute the selected source graph. The worktree branch name is `phase-152-c03e-sv-fallible-admission-timing-six-path-source-materialization`.

That local attempt has no remote GitHub branch, no PR, no canonical Drive audit and no source authority. No commit or push was performed. The attempt is diagnostic only.

The local compile reached the Linux production operation after successfully type-checking the new timing failure types, the shared result-capable executor path, the endpoint S2-S4 siblings and the executor S5 sibling. Compilation then failed because the Linux lifecycle value is `ProductionReachabilityEndpointLifecycleRuntime`, while the selected new S2 method exists on `RemoteSessionEndpointLifecycleRuntime`.

C03e-SN explicitly requires STOP and reselection if its six selected paths cannot contain the complete change. C03e-SW follows that rule and does not silently widen the source ceiling in the diagnostic worktree.

## 3. Exact source contradiction

At exact SU, `linux_bootstrap.rs` binds the remote endpoint through `ProductionReachabilityRuntimeCustody::bind_remote_endpoint_with_executor_from_systemd_credentials(...)`. Successful bind returns `ProductionReachabilityEndpointLifecycleRuntime`.

`ProductionReachabilityEndpointLifecycleRuntime` is defined in:

`crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs`

Exact SU blob:

`260d53f5c46b912a6b3a592ca58a44e82d8f6d8e`

It owns exactly:

- `endpoint: RemoteSessionEndpointLifecycleRuntime`;
- `owner_custody: ProductionReachabilityEtcdOwnerCustody`.

Both fields are private. There is no endpoint extraction or `into_parts` seam.

The wrapper already preserves the required custody law with private `drive_with_retained_custody(endpoint, owner_custody, drive)`: the lower endpoint drive executes first, retained durable production-reachability custody stays alive for the entire lower drive, custody is dropped only after the lower drive returns, and the exact lower result is returned unchanged.

The existing higher-observation production wrapper method delegates once through that custody helper to the existing raw endpoint higher-observation method. Therefore `linux_bootstrap.rs` cannot call the new raw S2 sibling directly without either bypassing retained custody, widening private fields, or modifying the production wrapper path.

## 4. Historical corroboration

The earlier evidence-closed C03e-PW selection recorded the same architectural fact: Linux receives `ProductionReachabilityEndpointLifecycleRuntime`, direct Linux migration to a raw endpoint adapter would bypass or reconstruct the distinct `ProductionReachabilityEtcdOwnerCustody` lifecycle role, and propagation must first cross the production wrapper.

C03e-SW does not revive or replace C03e-PW semantics. It uses the current exact SU source graph as authority; the historical selection only corroborates the already-live layering rule.

## 5. Corrected future source ceiling

The future source materialization ceiling is corrected from six to exactly seven Rust paths:

P0 `crates/prw-agent/src/remote_session_capability_runtime.rs`
- predecessor blob `2b6a0fd693f2ddacec608018a4db811cc36ef4c0`.

P1 `crates/prw-agent/src/remote_session_capability_runtime/admission_timing_failure.rs`
- predecessor state: absent.

P2 `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration/production_durable_repeated_real_admission_collection.rs`
- predecessor blob `d50e423d01b4161e244c075d59a85fd83533cf62`.

P3 `crates/prw-agent/src/remote_session_capability_runtime/remote_session_endpoint_lifecycle_runtime.rs`
- predecessor blob `881846753f51bdf94cff32ff0f9649dbcf50a80f`.

P4 `crates/prw-agent/src/linux_bootstrap.rs`
- predecessor blob `b5d236d3c0264ea8870e90af96c76294b50d0fc1`.

P5 `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`
- predecessor blob `cccefed1192243a16b67c971ca6ba8fe13ae2c82`.

P6 `crates/prw-agent/src/production_reachability_endpoint_lifecycle.rs`
- predecessor blob `260d53f5c46b912a6b3a592ca58a44e82d8f6d8e`.

Protected unchanged paths include:

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`
- predecessor blob `1539b6b9a08bf18883d7a16022f15f7c240eaf08`.

`crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`
- predecessor blob `304513eaf4ac72720a96c265f72dd8a56f5ac87f`.

No eighth source path, manifest, lockfile, build script, workflow, Android source, package/service path, repository configuration, provider, sink or executable caller is selected.

## 6. P6 selected role

P6 may add exactly one dormant crate-visible forwarding sibling on `ProductionReachabilityEndpointLifecycleRuntime` corresponding to the existing method:

`drive_repeated_real_remote_admission_endpoint_lifecycle_with_production_durable_fallible_verifier_time_expected_device_admission_producer_with_higher_observation_projection`

The new symbol is the complete existing symbol followed by the literal suffix:

`_with_fallible_admission_timing`

Selected visibility: `pub(crate)`.

The new sibling preserves the existing ordered non-timing inputs and output type, changes only F to the C03e-SN selected Result shape, adds generic `Cause` and `K`, and appends `on_timing_failure: K` as the final ordinary argument.

The sibling must:

1. consume `self` exactly once;
2. destructure only `endpoint` and `owner_custody`;
3. invoke `drive_with_retained_custody(endpoint, owner_custody, ...)` exactly once;
4. inside that closure invoke P3/S2 `_with_fallible_admission_timing` exactly once;
5. forward all existing inputs and callbacks unchanged and forward K once as the final argument;
6. preserve the exact lower `RemoteSessionPersistentCollectionConfigError` result;
7. perform no receipt projection itself;
8. perform no timing-source invocation itself;
9. perform no retry, requeue, listener bind, endpoint replacement, provider operation, readiness publication or runtime activation.

The wrapper must not expose either private field, add an extraction seam, clone or replace `owner_custody`, or bypass `drive_with_retained_custody`.

## 7. Corrected propagation chain

C03e-SN S0-S6 names and semantics remain unchanged.

The corrected production call chain is:

S0 P5 higher owner
→ S1 P4 Linux runtime-input-aware operation
→ P6 production-reachability retained-custody forwarding sibling
→ S2 P3 higher-observation raw endpoint sibling
→ S3 P3 concrete expected-device producer sibling
→ S4 P3 generic scheduling producer sibling
→ S5 P2 executor endpoint-lifecycle sibling
→ S6 P2 collection sibling
→ shared result-capable collection core.

P6 is an inserted custody-preserving propagation layer; it is not a new timing semantic stage and does not rename S0-S6.

## 8. Preserved C03e-SN source semantics

C03e-SN remains the sole timing-source semantic authority. C03e-SW does not change:

- `RemoteSessionAdmissionTimingSourceError<Cause>` variants or cause law;
- `RemoteSessionAdmissionTimingFailure<D,T,TimingError>` intact-request custody;
- F seeing only `&DeviceId`;
- K receiving the complete carrier once;
- duplicate preflight before F;
- zero F/K on duplicate;
- one F on vacant request;
- one K and no AJ/network/worker effect on timing failure;
- shared idle/pending preparation;
- pending producer future retention and real-receipt law;
- unrelated later work continuation;
- explicit `Infallible` lift for existing APIs;
- existing C/R/E/O callback meanings;
- no provider, sink, executable or runtime activation.

## 9. Future source acceptance

The later source checkpoint must begin from exact evidence-closed SW head and fresh-read the seven predecessor source blobs above plus P1 absence.

Acceptance requires:

- exactly P0-P6 changed and no eighth path;
- P6 sibling exactly preserves retained production-reachability custody;
- S1 calls P6, not raw S2;
- P6 calls S2 once through `drive_with_retained_custody`;
- S0-S6 retain the C03e-SN selected names, visibility, type/custodian/control-flow law;
- both protected blobs remain exact;
- complete local locked metadata, formatting, Clippy with `-D warnings`, workspace tests and workspace build pass;
- exact-final-head GitHub CI PASS before evidence publication;
- no inherited validation from SU or the local diagnostic attempt.

If correct implementation requires any eighth path or a change in timing-source semantics, STOP and reselect again.

## 10. Current non-actions and STOP

C03e-SW is documentation-only. It does not mutate Rust source, the local diagnostic SV worktree, `main`, workflows, dependencies, repository configuration, services, deployment state or product runtime.

It does not merge or close a PR, mark a PR ready for review, delete branches or rewrite history.

After exact-head validation, immutable Drive evidence publication and postpublication GitHub closure binding, STOP. The next source-materialization checkpoint may materialize the corrected seven-path layout from exact SW authority using the already-connected local execution surface.
