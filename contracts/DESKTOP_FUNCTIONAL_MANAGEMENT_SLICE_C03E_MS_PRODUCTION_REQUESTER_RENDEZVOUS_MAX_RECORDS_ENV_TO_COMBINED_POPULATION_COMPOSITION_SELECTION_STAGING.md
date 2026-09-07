# Phase 152 — C03e-MS Production Requester/Rendezvous Max-Records Environment-to-Combined-Population Composition Selection Staging

Status: `SELECTION_PENDING_VALIDATION`

Gate:

`C03E_MS_PRODUCTION_REQUESTER_RENDEZVOUS_MAX_RECORDS_ENV_TO_COMBINED_POPULATION_COMPOSITION_SELECTED`

## 1. Purpose

C03e-MS selects only the next dormant source-composition seam after the evidence-closed C03e-MR fixed requester/rendezvous max-records environment-source materialization.

The selected future source helper will read the already-materialized fixed environment source exactly once and, only after that succeeds, pass the exact returned target-`usize` value into the already-materialized C03e-MP explicit-capacity combined production-population helper exactly once.

This checkpoint is documentation-only. It does not modify Rust, wire an executable caller, invoke an operation, activate networking/runtime, deploy, merge, or alter host configuration.

## 2. Exact predecessor authority

Immediate predecessor:

`C03e-MR — Production Requester/Rendezvous Max-Records Environment Source Materialization`

Exact predecessor branch:

`phase-152-c03e-mr-production-requester-rendezvous-max-records-env-source-materialization`

Exact predecessor head:

`1a25526ff6b62143aed0ceb8bfb600fdf6e17e8b`

Exact predecessor tree:

`09251161738225bdbbb8a069469858e0db127b67`

Exact MR source blob:

`crates/prw-agent/src/linux_bootstrap.rs`

`03f24c74f82c94d892d6a8dae561014c0ad60a3f`

Existing C03e-MP combined-population source blob inherited unchanged by MR:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

`88e3c51b4b5f1e4caecd1b62919489bcde9c6efd`

C03e-MR is evidence-closed with exact-final-head Rust and Android validation and remains draft/open/unmerged.

## 3. Existing MR source contract

MR materializes exactly one fixed non-secret process configuration name:

`PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS`

through crate-private constant:

`PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS_ENV`

and one crate-private loader:

`load_linux_agent_remote_requester_rendezvous_max_records_from_env()`

with return shape:

```text
Result<usize, LinuxAgentRemoteRequesterRendezvousMaxRecordsSourceError>
```

The MR parser contract is exact:

- fixed environment variable name only;
- Unicode required;
- strict ASCII decimal `[0-9]+` only;
- no trimming;
- no normalization;
- no sign handling;
- no decimal/scientific notation;
- no separators;
- no alias to `PRW_REMOTE_MAX_ACTIVE_WORKERS`;
- no default;
- no fallback;
- no retry;
- no host-derived sizing;
- target `usize` overflow fails closed;
- zero parses successfully as exact `0`.

The MR source error remains exactly bounded as:

1. `Missing`;
2. `NonUnicode`;
3. `InvalidValue`.

The source error does not disclose the configured value.

## 4. Existing MP population contract

The existing C03e-MP helper remains:

`linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_inputs_from_production_sources_with_explicit_nonzero_capacity(...)`

It accepts the existing production population inputs plus one explicit caller-owned:

`max_records: usize`

and returns the exact existing dormant final owner:

```text
LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<
    ProductionRemoteCapabilityDenyAllPolicy,
    D,
    T,
    F,
    C,
    R,
    E,
>
```

with existing error:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessInputPopulationError`

The existing MP error is intentionally exactly two-source:

1. `ProductionSources(LinuxAgentProductionDurableReachabilityRemoteProcessInputPopulationError)`;
2. `RequesterRendezvousRuntime(RequesterRendezvousLifecycleError)`.

C03e-MS does not modify, widen, reinterpret, or add a third variant to this historical MP error.

The MP staged behavior remains exact:

```text
MJ requester-policy production population
 -> if failure: ProductionSources(...), stop

successful MJ carrier
 + exact max_records
 -> ML requester/rendezvous runtime-owner population
 -> if failure: RequesterRendezvousRuntime(...), stop

successful MJ carrier
 + successful ML runtime owner
 -> MN custody join
 -> exact final dormant owner
```

The existing ML/provider constructor remains the sole semantic authority for the non-zero requester/rendezvous capacity invariant.

## 5. Selected C03e-MS composition

C03e-MS selects exactly one future crate-private dormant async wrapper that composes:

1. the exact existing MR fixed environment loader;
2. the exact existing MP explicit-capacity combined population helper.

The selected staged order is exact:

```text
existing production-population arguments
 -> MR max-records environment loader exactly once
 -> if MR source fails: return bounded source-stage error and STOP

successful exact usize max_records
 + unchanged existing production-population arguments
 -> existing MP helper exactly once with that exact max_records
 -> if MP fails: return bounded population-stage error and STOP

successful MP owner
 -> return exact final dormant owner unchanged
```

The MP helper is never called after an MR source failure.

The MR loader is never called more than once.

The parsed `usize` is passed to MP unchanged.

There is no capacity pre-validation, clamp, normalization, fallback or substitution between MR and MP.

## 6. Selected zero-capacity law

Zero remains intentionally valid at the MR lexical/source boundary.

C03e-MS does not add a pre-check such as:

```text
if max_records == 0 { ... }
```

and does not convert to `NonZeroUsize`.

If the configured source value is `0` or any ASCII-decimal representation of zero, MR returns exact `0` and the future MS-selected wrapper passes exact `0` unchanged to MP.

MP then invokes the existing ML helper, whose existing provider constructor remains the sole semantic authority that rejects zero capacity and returns the existing requester/rendezvous lifecycle error through the existing MP `RequesterRendezvousRuntime(...)` variant.

Therefore:

- source syntax/representation validity remains MR authority;
- requester/rendezvous non-zero semantic validity remains existing provider/runtime authority;
- C03e-MS introduces no duplicate invariant enforcement.

## 7. Selected bounded outer error

Because the MR source error is distinct from the already-closed two-source MP error, C03e-MS selects exactly one new minimum crate-private two-stage outer error.

Conceptual name:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousConfiguredPopulationError`

Mechanical naming may adjust minimally as Rust requires without semantic change.

The selected outer error has exactly two variants:

```text
RequesterRendezvousMaxRecordsSource(
    LinuxAgentRemoteRequesterRendezvousMaxRecordsSourceError
)

Population(
    LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessInputPopulationError
)
```

Allowed ordinary Rust plumbing only:

- `Display`;
- `std::error::Error::source` preserving the exact underlying source;
- `From<LinuxAgentRemoteRequesterRendezvousMaxRecordsSourceError>`;
- `From<LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessInputPopulationError>`.

No third variant is selected.

No opaque boxed error is selected.

No string erasure is selected.

No retry classification is selected.

No startup/process-exit mapping is selected.

The existing MP two-source error remains unchanged and nested intact as the `Population(...)` source.

## 8. Selected future helper shape

Conceptual helper name:

`linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_inputs_from_configured_production_sources`

Mechanical naming may adjust minimally without semantic change.

Selected conceptual input shape remains exactly the current MP non-capacity inputs:

```text
expected_requests: mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>
admission_timing: F
on_completion: C
on_rejection: R
on_admission_failure: E
```

There is no explicit `max_records` parameter in this wrapper because its sole purpose is to acquire that exact value from the fixed MR environment source.

Selected conceptual return shape:

```text
Result<
    LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessOperationInputs<
        ProductionRemoteCapabilityDenyAllPolicy,
        D,
        T,
        F,
        C,
        R,
        E,
    >,
    LinuxAgentProductionDurableReachabilityRequesterRendezvousConfiguredPopulationError,
>
```

Selected mechanical implementation law:

```text
let max_records =
    load_linux_agent_remote_requester_rendezvous_max_records_from_env()?;

let inputs =
    linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_inputs_from_production_sources_with_explicit_nonzero_capacity(
        expected_requests,
        admission_timing,
        on_completion,
        on_rejection,
        on_admission_failure,
        max_records,
    )
    .await?;

Ok(inputs)
```

Equivalent formatting and minimal explicit `map_err` are allowed only if required mechanically; they must preserve the exact two stages and exact underlying error types.

## 9. Side-effect ordering

The fixed max-records environment source is intentionally evaluated before invoking MP.

Therefore if the source is missing/non-Unicode/malformed/out-of-range:

- no MP production-source population runs;
- no worker-limit/bind source population runs through MP;
- no peer/durable-registry production population runs through MP;
- no requester-policy carrier is built through MP;
- no requester/rendezvous provider/runtime owner is constructed;
- no final owner is returned.

This is fail-fast configuration acquisition, not runtime activation.

After source success, MP preserves its existing internal stage ordering unchanged.

## 10. Ownership and identity invariants

The returned final owner remains exactly the existing MP/MN owner.

No new custody type is selected.

No clone/share/extraction API is selected.

The current capability policy remains exactly:

`ProductionRemoteCapabilityDenyAllPolicy`

The requester/rendezvous policy source remains the existing empty fail-closed source.

The canonical identity law remains:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

The environment capacity value is configuration provenance only and is not:

- logical identity;
- endpoint authority;
- authentication;
- authorization;
- a capability grant;
- requester/rendezvous authority;
- readiness;
- reachability;
- publication provenance.

## 11. Explicit non-selection

C03e-MS does not select or authorize:

- any alternate capacity source;
- default requester/rendezvous capacity;
- hard-coded requester/rendezvous capacity;
- worker-limit aliasing;
- host/device/session-count sizing;
- dynamic refresh;
- cache;
- retry;
- pre-clamp or pre-validation of zero;
- modification of the MR parser/loader/error;
- modification of the existing MP two-source error;
- modification of the ML provider/runtime helper;
- modification of the MN custody join;
- a new owner type;
- requester-policy evaluation;
- positive requester bindings;
- provider registration/mutation;
- current requester/rendezvous grant selection;
- cleanup/TTL/persistence;
- current registry hydration;
- durable-to-current synchronization;
- expected-request producer provenance;
- dispatcher provenance;
- verifier current-time provenance;
- admission timing provenance;
- callback policy provenance;
- executable caller/input assembly;
- startup error mapping;
- process-exit policy;
- operation invocation;
- `run()` mutation;
- `main.rs` mutation;
- listener/readiness/network activation;
- deployment/restart/release;
- repository configuration mutation;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- force update;
- rebase/squash/history rewrite.

## 12. Immediate source-successor ceiling

If this selection is exact-head validated and evidence-closed, the immediate source successor may modify exactly one Rust path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

The successor may add only:

1. the minimum imports needed to reference the existing MR loader and MR source error from `crate::linux_bootstrap`;
2. one minimum crate-private two-stage outer error carrying exactly the MR source error and existing MP population error;
3. ordinary narrow `Display`, `Error::source`, and `From` plumbing for those exact two sources;
4. one minimum crate-private dormant async wrapper implementing exact MR-loader -> existing-MP-helper order;
5. only same-file focused type/dormancy tests if mechanically necessary to preserve existing CI conventions.

Narrow `dead_code`, `type_complexity`, or `future_not_send` allows are permitted only where the existing module/CI pattern requires them.

The immediate successor must STOP if implementation requires:

- a second changed path;
- public API widening;
- a third outer error source;
- mutation of the existing MP error;
- mutation of `linux_bootstrap.rs`;
- mutation of provider/runtime code;
- concrete default/alternate capacity;
- caller wiring;
- startup mapping;
- runtime behavior;
- deployment configuration.

## 13. Validation expectations

C03e-MS itself is documentation-only.

Exact-final-head Rust validation is required for closure.

Path-filtered workflows may be skipped and must not be reported as PASS.

Android PASS must not be claimed unless an Android workflow actually runs and reaches terminal SUCCESS for the exact final MS head.

## 14. Closure boundary

C03e-MS closes only the selection of the fixed MR environment source -> existing MP combined population composition seam.

The selected composition remains dormant source architecture.

Even after a future source materialization, separately gated work still includes at least:

- expected-request producer/channel provenance;
- dispatcher provenance;
- verifier current-time provenance;
- admission timing provenance;
- callback policies;
- positive requester policy/current grants;
- current registry hydration and durable-to-current synchronization;
- executable caller/input assembly;
- startup error mapping;
- `run()` / `main.rs` wiring;
- listener/readiness/network activation;
- deployment/restart/release.

This selection is not a merge, launch, release, deployment, runtime activation or production cutover.
