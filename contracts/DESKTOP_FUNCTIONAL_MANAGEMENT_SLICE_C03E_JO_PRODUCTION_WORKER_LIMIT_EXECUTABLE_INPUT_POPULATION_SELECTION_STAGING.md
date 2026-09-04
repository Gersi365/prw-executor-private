# Phase 152 C03e-JO — Production worker-limit executable input population selection

Status: **SELECTION STAGING**

Target gate:
`C03E_JO_PRODUCTION_WORKER_LIMIT_EXECUTABLE_INPUT_POPULATION_BOUNDARY_SELECTED`

Intended closure classification after exact-head validation and immutable audit:
`CLOSED_PRODUCTION_WORKER_LIMIT_EXECUTABLE_INPUT_POPULATION_SELECTION`

## 1. Exact predecessor

C03e-JO is rooted exactly at the closed C03e-JN production worker-limit environment-source materialization head:

- branch: `phase-152-c03e-jn-production-worker-limit-env-source-materialization`;
- head: `378e092e8fe71ae92c6686b3bbc49dedd173cf36`;
- tree: `328e642a42cfc88115ec3179043597b5efcb5cc8`;
- `crates/prw-agent/src/linux_bootstrap.rs` blob: `cf1edd600a7d9dfdba170584587a9f222fb1b1b2`;
- `crates/prw-agent/src/main.rs` blob: `db6b8028c6df100a961a0fb5818347bea2fdc5c1`;
- predecessor gate: `C03E_JN_PRODUCTION_WORKER_LIMIT_ENV_SOURCE_MATERIALIZED`;
- predecessor closure: `CLOSED_PRODUCTION_WORKER_LIMIT_ENV_SOURCE_MATERIALIZATION`.

C03e-JN materialized and validated only the fixed `PRW_REMOTE_MAX_ACTIVE_WORKERS` source and explicitly left executable-input population separately gated. C03e-JO performs only that next population-boundary selection.

## 2. Exact-source observations

On exact C03e-JN head, `LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>` retains these exact fields by value:

- `bind_addr: SocketAddr`;
- `max_active_workers: NonZeroUsize`;
- `capability_authority: SharedCurrentCapabilityAuthority<P>`;
- `session_authentication: SessionAuthenticationService`;
- `expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`;
- `admission_timing: F`;
- `on_completion: C`;
- `on_rejection: R`;
- `on_admission_failure: E`.

The exact source also establishes:

1. `load_linux_agent_remote_max_active_workers_from_env()` now returns the selected production worker limit as `Result<NonZeroUsize, LinuxAgentRemoteMaxActiveWorkersSourceError>`;
2. the loader uses only fixed process configuration `PRW_REMOTE_MAX_ACTIVE_WORKERS` and is fail-closed with no fallback/default/retry/auto-sizing semantics;
3. the existing C03e-IM helper `linux_agent_remote_process_operation_inputs_from_production_bind_addr(...)` already populates the `bind_addr` field from `load_linux_agent_remote_bind_addr_from_env()` and then calls `LinuxAgentRemoteProcessOperationInputs::new(...)` exactly once;
4. that existing bind helper currently accepts `max_active_workers: NonZeroUsize` as an already-typed caller input;
5. C03e-JK already materialized a later production-peer population helper that consumes one already-built `LinuxAgentRemoteProcessOperationInputs` value and wraps it without changing the lower-level fields;
6. `run()` and `main.rs` do not invoke the worker-limit loader, the production bind helper, the production peer helper, or a full production remote-process executable assembly path;
7. no exact C03e-JN source already connects the new worker-limit loader to `LinuxAgentRemoteProcessOperationInputs`.

The next smallest missing executable-production seam is therefore the one that replaces only the existing caller-supplied `max_active_workers` value at the production input-population boundary while reusing the already-materialized bind-address population helper unchanged.

## 3. Selected boundary

C03e-JO selects exactly one next executable-input population boundary:

> source one production `NonZeroUsize` worker limit through the existing `load_linux_agent_remote_max_active_workers_from_env()` loader, then pass that exact value into the already-materialized production bind-address input-population helper so the existing `LinuxAgentRemoteProcessOperationInputs` owner is produced without duplicating its constructor path.

This is population composition only.

It does not select a concrete deployed worker-limit value, process-environment provisioning, full production aggregate assembly, executable invocation, runtime activation, worker-accounting redesign, or worker lifecycle behavior change.

## 4. Selected future helper

The first source-materialization successor after C03e-JO may add one crate-private helper in:

`crates/prw-agent/src/linux_bootstrap.rs`

with the selected semantic role:

```text
linux_agent_remote_process_operation_inputs_from_production_worker_limit(
    capability_authority,
    session_authentication,
    expected_requests,
    admission_timing,
    on_completion,
    on_rejection,
    on_admission_failure,
)
 -> Result<LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>,
           LinuxAgentProductionRemoteProcessInputPopulationError>
```

The exact generic field types must remain those already required by the existing `LinuxAgentRemoteProcessOperationInputs` and C03e-IM bind-address helper.

The helper must not accept caller-supplied `bind_addr` or caller-supplied `max_active_workers` because both of those fields already have separately selected and materialized production source paths.

## 5. Selected exact call order

The materialization successor must preserve this exact fail-closed ordering:

```text
load_linux_agent_remote_max_active_workers_from_env()
    -> max_active_workers
    -> linux_agent_remote_process_operation_inputs_from_production_bind_addr(
           max_active_workers,
           already_typed_remaining_inputs...
       )
    -> LinuxAgentRemoteProcessOperationInputs
```

The selected helper must:

1. call `load_linux_agent_remote_max_active_workers_from_env()` exactly once;
2. stop immediately if that loader fails;
3. call the existing `linux_agent_remote_process_operation_inputs_from_production_bind_addr(...)` exactly once only after worker-limit source success;
4. pass the exact returned `NonZeroUsize` unchanged as the helper's first `max_active_workers` argument;
5. move all other already-typed values unchanged;
6. return the existing lower-level `LinuxAgentRemoteProcessOperationInputs` owner without invoking any operation factory or runtime;
7. perform no direct second call to `LinuxAgentRemoteProcessOperationInputs::new(...)` because C03e-IM already owns that constructor path.

This ordering deliberately makes worker-limit failure occur before bind-address environment acquisition. C03e-JO selects no alternate ordering, parallel acquisition, retry, fallback, cache, or partial owner.

## 6. Selected bounded composite failure surface

Because the selected helper introduces one new production source stage before an already-existing fallible bind-address population stage, the materialization successor may add one bounded crate-private composite error:

`LinuxAgentProductionRemoteProcessInputPopulationError`

with exactly these semantic variants:

```text
WorkerLimitSource(LinuxAgentRemoteMaxActiveWorkersSourceError)
BindAddressSource(LinuxAgentRemoteBindAddressSourceError)
```

The selected failure law is:

- worker-limit loader failure is preserved as `WorkerLimitSource` and prevents the bind helper from being called;
- bind-address helper failure is preserved as `BindAddressSource` and no owner is returned;
- the underlying exact source error remains available through `std::error::Error::source()`;
- bounded `Display` text must classify only the stage and must not expose configured environment values;
- no fallback/default/retry/alternate-variable behavior is introduced.

The successor may add the minimum `From` conversions required to preserve the exact underlying source-error types without manual remapping at each `?` boundary.

No broader bootstrap/startup/executable-exit error mapping is selected by C03e-JO.

## 7. Existing worker-limit source remains frozen

C03e-JO does not modify the C03e-JN source contract:

- constant remains `PRW_REMOTE_MAX_ACTIVE_WORKERS_ENV = "PRW_REMOTE_MAX_ACTIVE_WORKERS"`;
- accepted grammar remains ASCII decimal digits only;
- no trimming or normalization;
- zero remains invalid;
- target-`usize` overflow remains invalid;
- successful parsing preserves exact positive magnitude as `NonZeroUsize`;
- missing/non-Unicode/invalid failures remain bounded;
- no fallback, retry, alternate source, dynamic refresh, CPU/memory derivation, or active-worker derivation.

Executable population must consume the loader result exactly; it must not reparse or reinterpret the environment value.

## 8. Existing bind-address population remains frozen

C03e-JO reuses the existing C03e-IM helper unchanged:

`linux_agent_remote_process_operation_inputs_from_production_bind_addr(...)`

That helper remains authoritative for:

- exactly one `load_linux_agent_remote_bind_addr_from_env()` call;
- exact `LinuxAgentRemoteBindAddressSourceError` semantics;
- exact construction of `LinuxAgentRemoteProcessOperationInputs::new(...)`;
- moving all supplied typed values unchanged.

The C03e-JO successor must not duplicate bind parsing, add another bind source, change bind-address validation, modify `LinuxAgentRemoteProcessOperationInputs` fields, or bypass the existing helper.

## 9. Production peer population remains separately downstream

C03e-JO does not modify or invoke the existing C03e-JK helper:

`linux_agent_production_reachability_remote_process_operation_inputs_from_production_peer(...)`

The selected JO helper returns only the lower-level `LinuxAgentRemoteProcessOperationInputs` value that this existing downstream peer helper can consume in a later separately selected aggregate/caller composition.

C03e-JO does not perform:

- process peer-device environment loading;
- durable-registry bootstrap;
- current same-device peer lookup;
- `PeerConnectivityIdentity` construction;
- reachability recovery;
- endpoint/candidate selection.

## 10. Identity and authority invariants

C03e-JO preserves:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

Worker-limit configuration is scheduling/configuration provenance only.

It is not and must not become:

- `DeviceId`;
- `PeerConnectivityIdentity`;
- `TransportIdentity`;
- session identity;
- requester identity;
- requester/rendezvous target identity;
- endpoint/candidate identity;
- capability identity or authorization evidence;
- policy principal;
- PRWM `request_id`.

Bind address remains transient reachability data only. PRWM `request_id` remains correlation only.

## 11. Side-effect boundary

Definition of the future helper is dormant source materialization.

When uninvoked, it performs no I/O.

If a later separately gated caller invokes it, the only selected acquisition side effects are the already-authorized process-environment reads performed by:

1. `load_linux_agent_remote_max_active_workers_from_env()`;
2. the existing bind helper through `load_linux_agent_remote_bind_addr_from_env()`.

No socket bind/listen/connect, credential read, provider I/O, registry I/O, DNS/network discovery, task/thread spawn, readiness publication, candidate publication, traversal/dialing, policy mutation, session mutation, or worker mutation is selected.

## 12. All remaining production provenance stays deferred

C03e-JO does not select production population for:

- `SharedCurrentCapabilityAuthority<P>`;
- `SessionAuthenticationService` state/population;
- expected-request producer/channel lifecycle;
- admission timing/clock source;
- completion callback;
- rejection callback;
- repeated-admission-failure callback;
- requester/rendezvous policy source;
- requester/rendezvous provider construction/capacity/lifecycle;
- full production aggregate assembly;
- executable operation-factory invocation;
- startup failure/exit policy for the new composite population error.

Those surfaces remain separately gated.

## 13. `run()`, `main.rs`, operation factories and runtime remain frozen

The C03e-JO selection and its first source successor must not modify or make newly reachable:

- `run()`;
- `crates/prw-agent/src/main.rs`;
- `linux_agent_remote_process_operation(...)`;
- `linux_agent_production_reachability_remote_process_operation(...)`;
- requester/rendezvous production operation wrappers;
- process-companion executable assembly;
- endpoint startup/listener/readiness paths.

No service startup path may invoke the JO helper merely because it exists in source.

## 14. First source-successor ceiling

After C03e-JO is canonically validated and evidence-recorded, the immediate source-materialization successor may modify exactly one source file:

`crates/prw-agent/src/linux_bootstrap.rs`

Authorized source change is limited to:

1. one crate-private `LinuxAgentProductionRemoteProcessInputPopulationError` with only the two selected stage variants;
2. bounded `Display`, `Error::source`, and minimum exact `From` plumbing for those variants;
3. one crate-private helper named exactly `linux_agent_remote_process_operation_inputs_from_production_worker_limit`;
4. exactly one call to `load_linux_agent_remote_max_active_workers_from_env()`;
5. exactly one call to the existing `linux_agent_remote_process_operation_inputs_from_production_bind_addr(...)` after worker-limit success;
6. exact forwarding of the sourced `NonZeroUsize` and all already-typed remaining inputs;
7. focused source-shape/error-shape tests that do not mutate process-global environment;
8. any strictly local lint annotation required by the exact source shape, without global lint weakening or semantic widening.

The successor must stop if implementation requires another source path, a public API, a second owner struct, a new configuration source, executable invocation, `run()`/`main.rs` change, worker runtime change, peer lookup, policy/session population, or any second repository path.

## 15. Validation obligations for the materialization successor

The first source successor must prove on its exact final head:

1. only `crates/prw-agent/src/linux_bootstrap.rs` changed;
2. the helper success type remains exactly `LinuxAgentRemoteProcessOperationInputs<P, D, T, F, C, R, E>`;
3. worker-limit load occurs exactly once before bind-helper invocation;
4. bind-helper invocation occurs exactly once after worker-limit success;
5. no direct duplicate bind loader call exists in the JO helper;
6. no direct duplicate `LinuxAgentRemoteProcessOperationInputs::new(...)` call exists in the JO helper;
7. underlying worker-limit and bind-address source errors remain separately identifiable through the selected composite error;
8. all remaining values are moved unchanged;
9. `run()`, `main.rs`, operation factories, peer population, requester/rendezvous, runtime/readiness and packaging remain unchanged;
10. exact-final-head workspace CI is green, with skipped workflows reported only as skipped.

The existing pure parser/source tests remain authoritative for worker-limit lexical/value semantics. New tests must not mutate process-global environment merely to force loader outcomes.

## 16. Explicit exclusions

C03e-JO does not perform or authorize:

- Rust/source materialization in this selection checkpoint;
- environment/service/systemd/package mutation;
- a concrete deployed worker-limit value;
- worker-limit environment provisioning;
- worker spawn/cancel behavior changes;
- worker-accounting redesign;
- dynamic worker-limit mutation/reload;
- listener/bind/readiness/runtime/network activation;
- peer lookup/re-selection;
- registry or policy mutation;
- capability/session/expected-request/timing/callback production population;
- requester/rendezvous provider population or invocation;
- candidate publication, traversal, dialing, retry, reconnect, rebind, rebootstrap, or replacement;
- credential/certificate/private-key/trust/RBAC mutation;
- database/schema/control-plane mutation;
- deployment, restart, recovery activation;
- repository visibility/configuration mutation;
- merge, PR close, ready-for-review conversion, branch deletion, or history rewrite.

## 17. Successor rule

After C03e-JO closes, **STOP**.

The immediate next checkpoint may only materialize the selected one-file worker-limit executable-input population seam within Section 14.

After that source-materialization successor closes, work must stop again and begin from a fresh exact-head audit before selecting any remaining production provenance, aggregate assembly, executable caller, startup error policy, or runtime activation boundary.

No later boundary is inherited merely because worker-limit source and executable-input population are both present.

## 18. Validation target for C03e-JO

C03e-JO itself is documentation-only and may close only if all of the following are true on the exact final head:

- exact predecessor remains C03e-JN head `378e092e8fe71ae92c6686b3bbc49dedd173cf36` as merge base;
- branch is ahead only by the intended documentation commit(s) and zero behind;
- exactly one changed path exists:
  `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_JO_PRODUCTION_WORKER_LIMIT_EXECUTABLE_INPUT_POPULATION_SELECTION_STAGING.md`;
- there are zero Rust/source/runtime/manifest/lockfile/workflow/Android/packaging/host changes;
- repository CI required for the exact final docs-only head is observed and recorded without inheriting verdicts from another head;
- skipped workflows are recorded only as skipped, never as PASS;
- immutable Drive evidence is frozen, uploaded, raw-read back, and verified byte-exact before closure metadata is claimed;
- the PR remains draft, open, and unmerged.

Only then may `C03E_JO_PRODUCTION_WORKER_LIMIT_EXECUTABLE_INPUT_POPULATION_BOUNDARY_SELECTED` and `CLOSED_PRODUCTION_WORKER_LIMIT_EXECUTABLE_INPUT_POPULATION_SELECTION` be claimed.
