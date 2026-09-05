# C03e-LW — Production durable-capability higher-owner projection companion assembly selection

Status: `SELECTION_ONLY — NO_RUST_SOURCE_MUTATION`

This checkpoint is documentation-only. It selects one later dormant source-materialization boundary above exact evidence-closed C03e-LV. It does not itself modify Rust/source/runtime behavior, activate an executable caller, merge, deploy, restart, or alter repository configuration.

## 1. Exact predecessor authority

C03e-LV is the sole predecessor authority for this selection:

- branch: `phase-152-c03e-lv-production-durable-capability-higher-owner-callback-projection-caller-migration-source-materialization`
- exact head: `b8e2d8dbffbbff438f603abc2baf93c6a2eaedde`
- exact tree: `affa19e1db51b3727df55804f1de1448990c4567`
- exact higher-owner source blob: `d785a592e3ec3b15497b5ab87b3d3c7f96f32355`
- LV status: `SOURCE_MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`

No later `phase-152-c03e-lw*` branch existed at the fresh pre-selection namespace audit.

## 2. Fresh exact-source finding

### 2.1 Higher-owner durable custody module

Exact LV path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

Exact LV blob:

`d785a592e3ec3b15497b5ab87b3d3c7f96f32355`

The module already owns one non-cloneable higher-owner input aggregate:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<P,D,T,F,C,R,E>`

That aggregate retains:

1. one existing production/reachability/requester-rendezvous aggregate by value; and
2. exactly one outer `Arc<ProductionDurableCapabilityAuthority>`.

C03e-LV adds the dormant crate-private operation factory:

`linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_with_production_durable_capability_projection(...)`

The LV factory consumes the higher-owner aggregate by value, transfers the retained requester/rendezvous inputs plus the retained durable authority into the existing C03e-LT projection-capable Linux operation exactly once, and returns the resulting one-shot operation directly.

The LV factory does not create or clone another durable-authority `Arc`, does not reconstruct a legacy callback aggregate, and does not itself create an executable caller.

### 2.2 Existing Linux remote-companion runner

Exact LV path:

`crates/prw-agent/src/linux_bootstrap.rs`

Exact blob:

`7940a69e598355176a61b0bef5c7571dab9fb530`

The file already exposes the generic runner:

`run_with_remote_process_companion<F>(operation: F)`

with the existing return surface:

`Result<LinuxAgentBootstrapWithRemoteReport, LinuxAgentBootstrapStartFailure>`

The runner accepts one already-typed `FnOnce(LinuxAgentRemoteSupervisorShutdownPublisher) + Send + 'static` operation and composes it with the existing local Linux bootstrap and remote process lifecycle owner.

The same file also contains an older dormant crate-private typed companion assembly:

`run_with_production_reachability_requester_rendezvous_remote_process_companion(...)`

That older assembly consumes only the pre-durable higher-owner requester/rendezvous aggregate and calls the legacy requester/rendezvous operation factory. It does not own or transfer the C03e-LV durable-capability higher-owner aggregate.

### 2.3 Executable entrypoint remains lower and unchanged

Exact LV `crates/prw-agent/src/main.rs` blob:

`db6b8028c6df100a961a0fb5818347bea2fdc5c1`

The Linux executable still calls only:

`prw_agent::linux_bootstrap::run()`

The executable does not call `run_with_remote_process_companion(...)`, the older typed companion assembly, the C03e-LV higher-owner projection operation, or any durable-capability higher-owner executable assembly.

Exact LV `crates/prw-agent/src/lib.rs` blob:

`53e6b9c33d1a3be644fb6645289f6854cc096eee`

The higher-owner custody module remains crate-private. No visibility widening is required for the selected next seam.

## 3. Selected immediate later source boundary

The smallest next source boundary is not `main.rs`, not `run()`, and not production input population.

A later source-materialization checkpoint may modify exactly one Rust path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

It may add one additive dormant crate-private companion-assembly sibling conceptually named:

`run_with_production_durable_reachability_requester_rendezvous_remote_process_companion_with_production_durable_capability_projection(...)`

The exact spelling may be mechanically adjusted only if required by Rust naming/formatting constraints, while preserving this selected semantic boundary.

## 4. Required future assembly law

The selected future sibling must:

1. accept exactly one existing `LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<P,D,T,F,C,R,E>` by value;
2. call `linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_with_production_durable_capability_projection(inputs)` exactly once;
3. pass the returned one-shot operation directly to the existing `crate::linux_bootstrap::run_with_remote_process_companion(...)` exactly once;
4. return the existing `Result<LinuxAgentBootstrapWithRemoteReport, LinuxAgentBootstrapStartFailure>` unchanged;
5. keep the sibling `pub(crate)` and dormant, with no caller added in the same source checkpoint.

The future sibling may import only the existing Linux bootstrap report/start-failure/runner surfaces necessary for this composition. It must not mutate `linux_bootstrap.rs` merely to make the composition possible.

## 5. Callback and authority shape remains frozen

The selected assembly must preserve the exact C03e-LV projection-capable callback bounds:

- completion: `FnMut(DeviceId, RemoteSessionRequesterAwareEndpointLifecycleCompletionProjection)`;
- rejection: `FnMut(RemoteSessionExpectedDeviceAdmissionRejectionReason, RemoteSessionExpectedDeviceAdmissionRequest<D,T>)`;
- admission failure: `FnMut(DeviceId, RemoteSessionRealAdmissionError)`.

It must not reintroduce the legacy completion/rejection/admission-failure aggregate shapes.

The following authority/identity lanes remain distinct and must not be collapsed or substituted:

- `SharedCurrentCapabilityAuthority<P>`;
- `Arc<ProductionDurableCapabilityAuthority>`;
- requester/rendezvous authority;
- authenticated `DeviceId`;
- reachability/socket address;
- PRWM `request_id` correlation.

Logical/authenticated device identity remains separate from IP/port. `request_id` remains correlation only and is not identity/authentication evidence.

## 6. Ownership and side-effect ceiling

The future source sibling selects only executable-composition wiring around already-typed owned values.

It must add no:

- `Arc::new` for durable capability authority;
- `Arc::clone` for durable capability authority;
- accessor or `Clone`/`Copy` widening on higher-owner custody;
- callback translation or logging/metrics policy;
- provider bootstrap or provider I/O before the existing operation/runner boundaries;
- new retry/reconnect/fallback behavior;
- new endpoint bind/listener/readiness semantics;
- new controller publication semantics;
- new process-exit policy.

Invocation of the selected future runner would, by design, enter the already-existing local/remote companion runtime. However the future source checkpoint itself must add no invocation site, so repository/runtime behavior remains dormant until a separately gated caller checkpoint.

## 7. Explicitly not selected

C03e-LW does not select or authorize:

- Rust/source mutation in LW itself;
- `crates/prw-agent/src/linux_bootstrap.rs` mutation;
- `crates/prw-agent/src/main.rs` mutation;
- mutation of `run()`;
- executable entrypoint migration;
- concrete production aggregate population;
- production durable-authority provider/bootstrap population;
- requester/rendezvous provider/population changes;
- callback logging, metrics, counters, restart or process-exit policy;
- startup error composition beyond the existing returned type;
- listener/readiness/network activation changes;
- authentication/authorization/trust weakening;
- identity/address conflation;
- manifest, lockfile, workflow, Android, packaging, systemd, credential, certificate, RBAC or repository-configuration changes;
- merge or ready-for-review conversion;
- deploy, restart, recovery or production activation;
- PR close, branch deletion, force update, rebase, squash or history rewrite;
- destructive cleanup.

## 8. Gate and successor discipline

C03e-LW selection gate:

`C03E_LW_PRODUCTION_DURABLE_CAPABILITY_HIGHER_OWNER_PROJECTION_COMPANION_ASSEMBLY_SELECTED`

After C03e-LW is validated and evidence-recorded: **STOP**.

A later source checkpoint may materialize only the one-file dormant higher-owner projection companion assembly selected above, after a fresh namespace/head/source audit. It must not simultaneously add the executable caller, populate real production inputs, modify `main.rs`/`run()`, or activate runtime behavior.
