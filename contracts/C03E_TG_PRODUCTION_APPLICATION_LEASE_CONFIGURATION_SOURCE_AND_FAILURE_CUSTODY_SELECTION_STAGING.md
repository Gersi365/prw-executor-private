# C03e-TG Production Application-Lease Configuration Source and Failure Custody Selection

**Status:** `SELECTION — VALIDATION PENDING`

## Boundary

`PRODUCTION_APPLICATION_LEASE_CONFIGURATION_SOURCE_AND_FAILURE_CUSTODY_SELECTION`

Canonical selected law:

`FIXED_NON_SECRET_PROCESS_ENVIRONMENT_SOURCE / PRW_REMOTE_APPLICATION_LEASE_SECONDS / STRICT_ASCII_DECIMAL_U64_NO_TRIM_NO_FALLBACK / SOURCE_RETURNS_VALIDATED_REMOTE_SESSION_APPLICATION_LEASE_POLICY / REMOTE_SESSION_APPLICATION_LEASE_POLICY_NEW_REMAINS_SOLE_SEMANTIC_BOUND_AUTHORITY / ONE_SOURCE_ACQUISITION_PER_HIGHER_OWNER_COMPANION_CONSTRUCTION / SOURCE_AND_POLICY_VALIDATION_PRECEDE_EXPECTED_REQUEST_CHANNEL_AND_CONFIGURED_POPULATION / NEW_HIGHER_OWNER_SOURCE_CUSTODY_ERROR_WRAPS_EXACT_SOURCE_ERROR_OR_EXISTING_TF_COMPANION_ERROR / CONFIG_SOURCE_FAILURE_IS_PROCESS_CONFIGURATION_FAILURE_NOT_K_NOT_E / NO_RETRY_REFRESH_DEFAULT_OR_3600_FALLBACK / EXISTING_TF_PROPAGATION_AND_COMPATIBILITY_LANES_UNCHANGED / TWO_PATH_FUTURE_SOURCE_CEILING / NO_SOURCE_MATERIALIZATION / NO_MAIN_OR_RUNTIME_ACTIVATION`

## 1. Purpose

C03e-TF materialized the production propagation lane for an already-validated
`RemoteSessionApplicationLeasePolicy`, but intentionally selected no raw configuration source.

C03e-TG selects only:

- one concrete process-owned raw source for the application-lease lifetime;
- the parser and semantic validation boundary;
- the bounded source error shape;
- the higher-owner source failure custodian;
- acquisition order relative to channel creation and configured population;
- the future source-materialization ceiling.

TG performs no Rust/source mutation and no runtime activation.

## 2. Authoritative predecessor

C03e-TF is the direct predecessor:

- PR `#645`;
- branch `phase-152-c03e-tf-production-timing-lease-policy-propagation-source-materialization`;
- head `f4f2925c4ad124468f5b55308812abd786d25d03`;
- tree `88b717593e91be81ddc7956a9ea9264ed20afa2d`;
- draft/open/unmerged;
- evidence-closed.

TF materialized exactly five Rust paths and explicitly stopped before any raw lease configuration source,
`RemoteSessionApplicationLeasePolicy::new` production callsite, `main.rs` wiring or runtime activation.

## 3. Audited existing source conventions

At exact TF head, `linux_bootstrap.rs` already uses fixed non-secret process environment variables for
bounded production scalar/address inputs, including:

- `PRW_REMOTE_BIND_ADDR`;
- `PRW_REMOTE_PEER_DEVICE_ID`;
- `PRW_REMOTE_MAX_ACTIVE_WORKERS`;
- `PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS`;
- `PRW_REMOTE_EXPECTED_DEVICE_SCHEDULING_CONSUMPTION_MAX_RECORDS`.

Those sources share the relevant precedent:

- fixed single variable name;
- `std::env::var_os(...)`;
- no trimming or normalization unless required by the semantic type;
- no alternate-variable lookup;
- no fallback/default;
- bounded errors that do not expose configured values;
- process-local ownership, not remote negotiation.

TG follows that existing production-source family rather than introducing CLI, database, registry,
remote message, Android setting, config-file parser or secret-credential machinery.

## 4. Selected raw source

The selected configuration mechanism is one fixed non-secret process environment variable:

`PRW_REMOTE_APPLICATION_LEASE_SECONDS`

Selected Rust constant name:

`PRW_REMOTE_APPLICATION_LEASE_SECONDS_ENV`

The variable represents exactly one whole-second application lease lifetime for the process-level
production higher-owner instance.

It is not:

- a maximum;
- a default;
- a challenge lifetime;
- a proof freshness lifetime;
- a renewal interval;
- a per-device override;
- a per-request override;
- a remote-negotiated value.

## 5. Why environment is selected

The value is a non-secret bounded scalar and existing Linux production scalar configuration already uses
environment-source custody.

Selecting the environment preserves the existing source family and avoids inventing a higher-privilege or
secret-bearing mechanism for a value that is not secret material.

TG does not imply that arbitrary environment data is trusted for authorization. The raw value is only a
local process configuration input and must pass the semantic policy constructor before it can enter the TF
propagation lane.

## 6. Exact parser selection

Selected private parser name:

`parse_linux_agent_remote_application_lease_policy_value`

Selected input:

`Option<OsString>`

Selected parse order:

1. absent value -> `Missing`;
2. non-Unicode OS value -> `NonUnicode`;
3. empty Unicode string -> `InvalidValue`;
4. any byte that is not an ASCII decimal digit -> `InvalidValue`;
5. parse as `u64`; overflow/malformed parse -> `InvalidValue`;
6. pass the exact parsed `u64` to `RemoteSessionApplicationLeasePolicy::new(...)`;
7. preserve constructor failure as the source error's semantic-policy variant;
8. return the exact validated typed policy on success.

No trim, sign, decimal point, separator, unit suffix, whitespace, case conversion or locale parsing is selected.

Leading zeroes remain lexically accepted because they are ASCII decimal digits; semantic authority applies to
the parsed integer. Therefore `0001` is policy value `1`, while `0000` fails semantic policy validation.

## 7. Sole semantic bounds authority

`RemoteSessionApplicationLeasePolicy::new(lifetime_seconds)` remains the sole semantic authority for:

`1 <= lifetime_seconds <= MAX_REMOTE_SESSION_LEASE_SECONDS`

where the current maximum remains `3_600` seconds.

The environment parser must not duplicate, widen or weaken that semantic range.

In particular:

- `0` reaches the policy constructor and fails there;
- `3600` is accepted;
- `3601` reaches the policy constructor and fails there;
- a future change to the semantic maximum remains owned by the policy type rather than duplicated source logic.

## 8. No implicit lifetime

TG selects no default.

Missing configuration fails closed.

The source must not substitute:

- `3600`;
- `300`;
- challenge lifetime;
- prior process value;
- previous request value;
- any compile-time fallback;
- any remote value.

`MAX_REMOTE_SESSION_LEASE_SECONDS` remains a maximum, never an automatic production duration.

## 9. Selected source loader

Selected crate-internal loader name:

`load_linux_agent_remote_application_lease_policy_from_env`

Selected return type:

```text
Result<
    RemoteSessionApplicationLeasePolicy,
    LinuxAgentRemoteApplicationLeasePolicySourceError,
>
```

The loader performs exactly one `std::env::var_os(PRW_REMOTE_APPLICATION_LEASE_SECONDS_ENV)` acquisition and
passes the result to the selected parser.

The loader performs no cache, refresh, watcher, retry, alternate lookup, request construction, channel
construction, registry access, verifier-time sampling, admission or runtime activation.

## 10. Selected source error

Selected crate-internal error:

`LinuxAgentRemoteApplicationLeasePolicySourceError`

Selected variants:

```text
Missing
NonUnicode
InvalidValue
Policy(RemoteSessionApplicationLeasePolicyError)
```

Meaning:

- `Missing`: fixed variable absent;
- `NonUnicode`: OS value cannot be represented as Unicode;
- `InvalidValue`: empty/non-ASCII-decimal/u64-overflow raw representation;
- `Policy(...)`: exact semantic policy constructor rejection.

The error must not include or format the configured value.

`std::error::Error::source()` returns the exact `RemoteSessionApplicationLeasePolicyError` only for the
`Policy(...)` variant and `None` for the three raw-source variants.

No syntax error is reclassified as a policy error, and no policy error is flattened into raw syntax failure.

## 11. Validation boundary

The selected boundary is:

`raw environment bytes -> strict scalar parser -> u64 -> RemoteSessionApplicationLeasePolicy::new -> typed policy`

Only the typed policy may cross into the TF higher-owner propagation lane.

Raw seconds must not be propagated through:

- higher-owner custody;
- Linux companion composition;
- retained reachability custody;
- endpoint lifecycle;
- repeated-admission collection;
- F/K/E callbacks;
- request objects.

## 12. Acquisition frequency

The environment source is acquired exactly once per construction/invocation of the new higher-owner
configured-lease sibling.

The resulting validated typed policy is then moved by value into the existing TF sibling and reused as the
process-owned policy for that higher-owner instance.

TG selects no:

- per-request environment read;
- per-admission environment read;
- dynamic refresh;
- signal-triggered reload;
- lease renewal policy;
- hot configuration update.

## 13. Higher-owner source wrapper

A new crate-private sibling is selected beside the TF higher-owner function.

Selected semantic role:

`configured application-lease source -> validated typed policy -> existing TF higher-owner sibling`

Selected name:

`run_with_production_durable_reachability_requester_rendezvous_fallible_verifier_time_expected_device_admission_remote_process_companion_from_configured_production_sources_with_pre_aj_timing_and_configured_application_lease_policy`

Its inputs are the same F/C/R/E/K inputs as the TF sibling, except it does **not** accept a lease policy
parameter. It acquires the selected source internally once.

## 14. Exact higher-owner order

The new sibling must perform:

1. call `load_linux_agent_remote_application_lease_policy_from_env()` exactly once;
2. on source failure, return immediately;
3. only after source success, invoke the existing TF sibling exactly once with the exact validated typed policy;
4. TF then owns expected-request channel creation, existing configured population and dormant companion assembly.

Therefore invalid/missing lease configuration fails before:

- expected-request channel creation;
- configured requester/rendezvous population;
- worker/bind/peer source acquisition inside that population;
- K custody;
- E custody;
- endpoint/listener lifecycle;
- any request-specific activity.

## 15. Source failure is not K

K remains exclusively the pre-AJ request-owned timing-source failure custodian selected earlier.

Application-lease configuration source failure occurs before an expected request exists in the new wrapper.

It therefore must not:

- construct `RemoteSessionAdmissionTimingFailure`;
- fabricate an untouched request;
- invoke K;
- retry/requeue.

## 16. Source failure is not E

E remains real-admission failure custody after a concrete request enters the admission lane.

Configuration source failure occurs before admission begins.

It therefore must not be represented as:

- `RemoteSessionRealAdmissionError`;
- `ApplicationLeaseVerifierTime`;
- `ApplicationLeaseExpiryOverflow`;
- `Binding`;
- any other E callback event.

Post-authentication lease-time and expiry failures remain exactly the TD/TF E-domain behavior and are not
changed by TG.

## 17. Selected higher-owner error custody

TG selects a new crate-private wrapper error rather than modifying the historical TF companion error.

Selected name:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousConfiguredApplicationLeaseCompanionError`

Selected variants:

```text
ApplicationLeasePolicySource(LinuxAgentRemoteApplicationLeasePolicySourceError)
Companion(
    LinuxAgentProductionDurableReachabilityRequesterRendezvousConfiguredPopulationCompanionError
)
```

`ApplicationLeasePolicySource(...)` preserves the exact source error chain.

`Companion(...)` preserves the complete existing TF error classification for configured population or bootstrap
assembly without remapping its internal distinction.

This keeps compatibility surfaces unchanged and makes the new source boundary explicit.

## 18. No changes to existing configured population

The existing function:

`linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_inputs_from_configured_production_sources`

remains unchanged.

The lease policy is not added to that existing aggregate or its existing error enum.

Rationale:

- TE/TF intentionally kept lease policy outside the existing configured population aggregate;
- the new source is selected after that propagation architecture exists;
- a thin wrapper preserves historical configured-population behavior and avoids reinterpreting prior errors.

## 19. No new aggregate

TG does not select a new broad production-input aggregate solely for the lease policy.

The source loader returns the typed policy and the new higher-owner wrapper immediately moves it into the TF
sibling as an explicit parameter.

## 20. Security properties

The selected environment value is process-local configuration, not identity or authorization.

It must not be selected from or overridden by:

- `DeviceId`;
- requester identity;
- target identity;
- transport identity;
- IP/endpoint/candidate;
- request ID;
- scheduling grant;
- capability authority;
- remote peer message;
- challenge/proof contents;
- callback history.

No secret material is logged or required for this setting.

## 21. Compatibility

TG selects no changes to historical APIs or timing lanes.

The following remain unchanged:

- `RemoteSessionRealAdmissionTiming` compatibility lanes;
- TF challenge-only production lane;
- existing configured-production population helpers;
- existing TF higher-owner sibling that accepts an already-validated policy;
- F/K/E ownership;
- TD admission transaction;
- worker/provider custody;
- shutdown/scheduling behavior.

The new source wrapper is additive.

## 22. Future source-materialization ceiling

If separately authorized, the first TG-following source checkpoint is limited to exactly two Rust paths:

1. `crates/prw-agent/src/linux_bootstrap.rs`
   - fixed environment constant;
   - bounded source error;
   - strict parser;
   - typed loader;
   - bounded source tests.

2. `crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`
   - import source loader/error;
   - new wrapper error;
   - new configured-lease higher-owner sibling;
   - bounded ordering/error-custody tests where practical.

No lower TF propagation path should change in that first source checkpoint.

## 23. Future source validation expectations

A separately authorized source checkpoint should prove:

- exact constant is `PRW_REMOTE_APPLICATION_LEASE_SECONDS`;
- missing -> `Missing`;
- non-Unicode -> `NonUnicode`;
- empty -> `InvalidValue`;
- whitespace/sign/suffix/non-digit -> `InvalidValue`;
- u64 overflow -> `InvalidValue`;
- `0` -> exact wrapped policy error;
- `1` -> valid typed policy;
- `3600` -> valid typed policy;
- `3601` -> exact wrapped policy error;
- leading-zero positive integer is accepted as the same semantic value;
- raw configured value is absent from error display/debug policy surfaces intended for callers;
- source is read once per new wrapper invocation;
- source failure occurs before expected-request channel/configured population;
- source failure invokes neither K nor E;
- successful source acquisition moves exact typed policy into TF sibling;
- existing TF sibling and configured-population errors remain unchanged;
- only the selected two Rust paths change.

## 24. Runtime activation remains separately gated

Even after future source materialization, the following remain separately gated:

- concrete executable callsite that chooses the new configured-lease wrapper;
- concrete fresh pre-AJ timing F provider if not already activated at that callsite;
- callback/custodian executable bindings if not already activated;
- `main.rs` wiring;
- listener/readiness/network activation;
- service-manager configuration;
- deployment;
- restart;
- merge.

Source existence is not runtime activation.

## 25. Explicit non-actions

C03e-TG performs no:

- Rust source materialization;
- environment read at runtime;
- source loader implementation;
- policy construction callsite implementation;
- raw value defaulting;
- change to TF five-path propagation;
- change to TD transaction;
- request-type redesign;
- K/E redesign;
- `main.rs` wiring;
- listener/readiness/network activation;
- service-manager activation;
- deployment/restart;
- dependency/Cargo/lockfile/workflow change;
- repository configuration change;
- merge/ready conversion/PR close/branch deletion/history rewrite/force push;
- destructive evidence cleanup.

## 26. Selected result

C03e-TG selects:

`PRW_REMOTE_APPLICATION_LEASE_SECONDS`

as the sole concrete raw production application-lease configuration source for the next gated materialization,
with strict ASCII-decimal `u64` parsing, semantic validation only through
`RemoteSessionApplicationLeasePolicy::new`, a loader that returns the validated typed policy, and a new
higher-owner wrapper whose source failure is a process-configuration failure before channel creation,
configured population, K, E or runtime activity.

STOP before source materialization or runtime activation.
