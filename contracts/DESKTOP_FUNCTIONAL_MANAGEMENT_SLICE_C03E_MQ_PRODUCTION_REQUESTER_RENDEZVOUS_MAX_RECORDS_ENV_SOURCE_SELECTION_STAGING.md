# C03e-MQ — Production Requester/Rendezvous Max-Records Environment Source Selection

## Status

`SELECTION — VALIDATION_PENDING`

## Gate

`C03E_MQ_PRODUCTION_REQUESTER_RENDEZVOUS_MAX_RECORDS_ENV_SOURCE_SELECTED`

## 1. Exact predecessor authority

Evidence-closed predecessor:

`C03e-MP — Production Durable Reachability Requester/Rendezvous Explicit-Capacity Combined Population Source Materialization`

Exact predecessor branch:

`phase-152-c03e-mp-production-durable-reachability-requester-rendezvous-explicit-capacity-combined-population-source-materialization`

Exact predecessor head:

`1741b5826deba0c0b5229e4b4a36dc49f957d671`

Exact predecessor tree:

`fbf0ba2545437d42eb23afd3d2a89b2f5d734e6e`

Exact predecessor higher-owner source blob:

`88e3c51b4b5f1e4caecd1b62919489bcde9c6efd`

C03e-MP is evidence-closed. Its exact-final-head Rust and Android validations are successful.

Canonical immutable MP audit authority remains Drive file ID:

`1VWKDeVxwE8FZ8DQyl9sFCJF_LQqPitF_`

with exact readback:

- bytes: `16108`;
- SHA-256: `5b3ed23bb12ea12b061a74e50fdf1794d97b1a567caa0f1920bc1a0e7477009a`.

A later byte-identical post-closure duplicate exists at Drive file ID `145BFSCeNQe8TL6OQpCXuPEh1MtljEDN7`. It is non-canonical and preserved under the no-destructive-cleanup guardrail. The anomaly is recorded separately at Drive file ID `1ia_6WT0HKfh4niUsZP7N5U8_IrzDdNWS`.

C03e-MP materializes the combined dormant population seam but intentionally leaves the explicit `usize max_records` provenance unresolved and does not wire an executable caller.

## 2. Selection question

After C03e-MP, the exact source graph can already:

1. populate the fail-closed requester-policy production carrier;
2. construct a requester/rendezvous runtime owner from one explicit caller-owned `usize max_records`;
3. join both successful custody halves into the final durable requester/rendezvous owner;
4. preserve the bounded two-stage MP population error surface.

The smallest unresolved production prerequisite before executable population is therefore the source of the explicit requester/rendezvous provider capacity.

C03e-MQ selects only one fixed non-secret process-environment source for that numeric value.

It does not materialize source code, populate the value into the MP helper, select executable callbacks, invoke an operation, or activate runtime/network behavior.

## 3. Exact adjacent production-configuration precedent

At exact MP authority, `crates/prw-agent/src/linux_bootstrap.rs` already defines fixed non-secret process-environment sources for:

- `PRW_REMOTE_BIND_ADDR`;
- `PRW_REMOTE_PEER_DEVICE_ID`;
- `PRW_REMOTE_MAX_ACTIVE_WORKERS`.

Those sources use fixed names, process-local acquisition, bounded fail-closed parsing, no fallback/default, no alternate-variable probing, and diagnostics that do not disclose configured values.

The existing worker-limit source is a particularly close numeric precedent, but requester/rendezvous capacity is a distinct authority and must not alias the worker-limit value.

## 4. Selected concrete source

C03e-MQ selects one new fixed non-secret process environment variable:

`PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS`

Selected future constant:

`PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS_ENV`

The source is process configuration only.

It is not:

- requester identity;
- publisher identity;
- authorization evidence;
- transport identity;
- durable-registry authority;
- worker-limit authority;
- endpoint/candidate authority;
- readiness evidence.

## 5. Selected acquisition law

The future source loader must read exactly one process-local value through the fixed environment name above.

It must not:

- inspect any alternate variable;
- inspect CLI arguments;
- read a config file;
- read a systemd credential;
- query a database/control plane;
- infer capacity from CPU or memory;
- reuse `PRW_REMOTE_MAX_ACTIVE_WORKERS`;
- derive from expected-request channel capacity;
- derive from device/session counts;
- cache or dynamically refresh the value;
- mutate the process environment.

## 6. Selected lexical grammar

The acquired Unicode value is accepted only when all bytes are ASCII decimal digits:

`[0-9]+`

The future parser performs no:

- trimming;
- whitespace normalization;
- sign handling;
- decimal-point parsing;
- exponent parsing;
- underscore stripping;
- locale conversion;
- hexadecimal/octal/binary interpretation.

An empty value is invalid.

Any non-ASCII-decimal form is invalid.

Target-`usize` overflow is invalid.

Leading zeroes are permitted because they do not change the exact parsed numeric magnitude.

## 7. Selected return domain

The future source returns the exact parsed target-architecture:

`usize`

It does **not** convert to `NonZeroUsize`.

This is deliberate because the existing requester/rendezvous provider constructor already owns the semantic non-zero capacity invariant.

Successful lexical parsing preserves the exact `usize` magnitude.

## 8. Zero-capacity authority law

The numeric value `0` is lexically valid for the environment source and must parse successfully as:

`usize(0)`

The future source loader must not classify zero as `InvalidValue`.

The exact zero must be forwarded unchanged by later separately gated population into the existing C03e-ML helper:

`linux_agent_production_requester_rendezvous_runtime_owner_from_explicit_nonzero_capacity(max_records)`

That helper delegates to:

`InMemoryRequesterRendezvousAuthorityProvider::new(max_records)`

whose existing authority is:

`RequesterRendezvousLifecycleError::InvalidCapacity`

for zero capacity.

C03e-MQ therefore preserves the existing provider lifecycle constructor as the sole semantic authority for the non-zero rule and does not duplicate that validation inside configuration parsing.

## 9. Selected bounded source error

The future source error has exactly these semantic classes:

1. `Missing` — the fixed environment value is absent;
2. `NonUnicode` — the operating-system value is not valid Unicode;
3. `InvalidValue` — the value is empty, contains any non-ASCII-decimal byte, or cannot fit target `usize`.

Zero is not `InvalidValue`.

The future bounded source error must not expose the configured value.

No fallback/retry/recovery class is selected.

## 10. Selected future source names

Selected conceptual names:

- error: `LinuxAgentRemoteRequesterRendezvousMaxRecordsSourceError`;
- parser: `parse_linux_agent_remote_requester_rendezvous_max_records_value(...)`;
- loader: `load_linux_agent_remote_requester_rendezvous_max_records_from_env()`.

Mechanical formatting may adjust only as required by Rust while preserving this exact semantic boundary.

## 11. Visibility boundary

The immediate source successor should use the narrowest visibility compatible with the current crate-internal production population path.

C03e-MQ does not require public API widening.

The source successor may keep the constant/error/parser/loader crate-private if no existing compiler/test requirement forces broader visibility.

If broader public exposure appears necessary, the successor must stop and require a separate review rather than widening this gate automatically.

## 12. No worker-limit alias

The requester/rendezvous record bound and remote active-worker bound represent different resource dimensions.

C03e-MQ explicitly rejects:

`requester_rendezvous_max_records = PRW_REMOTE_MAX_ACTIVE_WORKERS`

as implicit provenance.

No equality, ratio, clamp, minimum, maximum, or computed relationship between those capacities is selected.

Both remain independently configured authorities.

## 13. No default capacity

C03e-MQ selects no default literal or fallback value.

In particular it does not select:

- `1`;
- `16`;
- `64`;
- `1024`;
- worker count;
- channel capacity;
- active session count;
- registered device count;
- a host-derived heuristic.

If the fixed environment value is missing, the future source must fail closed with `Missing`.

## 14. No secret/configuration escalation

Requester/rendezvous record capacity is selected as non-secret process configuration.

C03e-MQ does not move it into systemd credentials, secret storage, certificate material, private-key handling, or privileged configuration.

No service-manager or host configuration mutation is performed by this selection.

## 15. Relationship to the existing MP population seam

C03e-MQ does not modify or call the MP helper:

`linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_inputs_from_production_sources_with_explicit_nonzero_capacity(...)`

The future env-source materialization checkpoint must stop after source acquisition/parsing exists.

A later separately selected population checkpoint may compose:

```text
fixed requester/rendezvous max-records env source
 -> exact usize max_records
 -> existing C03e-MP combined population helper
```

That composition is not selected here beyond acknowledging it as a later dependency.

## 16. Future population error boundary remains separately gated

Because the selected source can fail before the existing MP population helper runs, a later executable/population composition will require a separately reviewed outer error boundary.

C03e-MQ does not select that outer error type yet.

It does not flatten source errors into the existing MP two-source error.

It does not add a third variant to:

`LinuxAgentProductionDurableReachabilityRequesterRendezvousRemoteProcessInputPopulationError`

The MP error remains unchanged.

## 17. Focused source-successor test ceiling

The immediate source materialization successor may add focused tests covering only the selected parser/source contract, including:

- fixed constant/name shape;
- missing value;
- empty value;
- non-Unicode value on supported test platforms;
- whitespace/sign/decimal/underscore/exponent/alphanumeric forms;
- target-`usize` overflow;
- ordinary positive values;
- leading-zero positive values;
- exact `usize::MAX`;
- zero parsing successfully as exact `0`.

Tests must not mutate process-global environment when an injected parser-value test can cover the shape.

No runtime/provider registration, lifecycle cleanup, networking, or deployment test is selected.

## 18. First source-materialization successor ceiling

The immediate successor may modify exactly one Rust path:

`crates/prw-agent/src/linux_bootstrap.rs`

It may add only:

1. the fixed environment-name constant;
2. the bounded source error;
3. one private parser helper;
4. one fixed-name environment loader;
5. focused parser/source-shape tests;
6. narrowly required lint/import formatting adjustments inside the same path.

It must not modify:

- `production_durable_capability_higher_owner_custody.rs`;
- Cargo manifests or lockfile;
- workflows;
- Android source;
- `run()` or `main.rs`;
- service/systemd configuration;
- database/control-plane code.

## 19. Source-successor implementation obligations

The immediate source materialization must:

1. acquire only `PRW_REMOTE_REQUESTER_RENDEZVOUS_MAX_RECORDS`;
2. preserve OS non-Unicode distinction;
3. reject empty/non-digit/overflow values as `InvalidValue`;
4. parse target `usize` exactly;
5. return zero successfully as exact `0`;
6. preserve leading-zero positive magnitude after parsing;
7. expose no configured value in diagnostics;
8. add no fallback/default/retry/cache/refresh;
9. invoke no requester/rendezvous provider constructor;
10. invoke no MP population helper;
11. add no executable caller.

If implementation requires any second path or broader authority, the successor must stop rather than broaden this gate.

## 20. Identity and authorization invariants

C03e-MQ preserves:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

The capacity value cannot substitute for:

- logical device/session identity;
- authenticated requester identity;
- expected publisher identity;
- transport identity;
- authorization policy;
- capability authority;
- endpoint/candidate authority.

PRWM `request_id` remains correlation only.

Configuration validity is not authorization.

## 21. Validation authority

Only the exact final C03e-MQ head may serve as validation authority.

MQ is documentation-only.

Rust validation is sufficient if that is the only workflow triggered by the changed path.

If Android validation triggers, its terminal result must be recorded.

`SKIPPED` is not PASS.

A successful workflow tied to a superseded head cannot validate a later head.

## 22. Durable evidence discipline

Canonical audit filename:

`C03E_MQ_PRODUCTION_REQUESTER_RENDEZVOUS_MAX_RECORDS_ENV_SOURCE_SELECTION_AUDIT_2026-09-06.md`

Canonical Drive parent:

`1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`

Closure requires:

- exact-title pre-upload search;
- immutable upload;
- raw byte readback;
- exact byte-count and SHA-256 verification;
- exact-title post-upload verification;
- PR closure metadata update;
- post-publication branch/main/successor guard.

If a duplicate evidence artifact is discovered, do not destructively clean it without explicit authorization; record the anomaly and preserve canonical authority explicitly.

## 23. Explicit non-selection

C03e-MQ does not select or authorize:

- Rust/source mutation in MQ itself;
- a concrete deployed capacity value;
- worker-limit aliasing;
- default/fallback capacity;
- environment provisioning/mutation;
- systemd credential usage for this value;
- source-to-MP population composition;
- outer executable population error mapping;
- provider registration or mutation;
- requester-policy evaluation or positive bindings;
- current requester/rendezvous grant selection;
- cleanup/TTL/timer/persistence;
- current capability policy widening;
- current registry hydration;
- executable caller wiring;
- operation invocation;
- `run()` or `main.rs` mutation;
- listener/readiness/network activation;
- candidate publication/traversal/dialing/retry/reconnect/rebind/rebootstrap;
- service/systemd/package/security/credential/certificate/private-key/trust/RBAC mutation;
- database/schema/control-plane mutation;
- repository visibility/configuration mutation;
- merge, ready-for-review conversion, PR close, branch deletion, force update, rebase, squash, history rewrite or destructive cleanup;
- deployment, restart, recovery or production cutover.

## 24. STOP boundary

After C03e-MQ closure, **STOP**.

The immediate successor may only materialize the selected one-file fixed environment source in:

`crates/prw-agent/src/linux_bootstrap.rs`

using the exact constant/error/parser/loader/focused-test ceiling above.

It must not yet feed the sourced value into C03e-MP or wire an executable caller.

After that source materialization closes, perform a fresh exact-head audit before selecting any source-to-MP population composition, executable caller, startup error policy, or runtime activation boundary.
