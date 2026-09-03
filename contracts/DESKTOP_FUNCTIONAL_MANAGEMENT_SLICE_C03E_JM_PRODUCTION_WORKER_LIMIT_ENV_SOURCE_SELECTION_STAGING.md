# Phase 152 C03e-JM — Production worker-limit environment source selection

Status: **SELECTION STAGING**

Target gate:
`C03E_JM_PRODUCTION_WORKER_LIMIT_ENV_SOURCE_SELECTED`

Intended closure classification after exact-head validation and immutable audit:
`CLOSED_PRODUCTION_WORKER_LIMIT_ENV_SOURCE_SELECTION`

## 1. Exact predecessor

C03e-JM is rooted exactly at the closed C03e-JL production worker-limit provenance-selection head:

- branch: `phase-152-c03e-jl-production-worker-limit-provenance-selection`;
- head: `fc50e6152c2827329bfc03929702ffbb608e7b0e`;
- tree: `aaff001f59d3902794df215bca47ed65c7799851`;
- predecessor gate: `C03E_JL_PRODUCTION_WORKER_LIMIT_PROVENANCE_BOUNDARY_SELECTED`;
- predecessor closure: `CLOSED_PRODUCTION_WORKER_LIMIT_PROVENANCE_SELECTION`.

C03e-JL selected only the still-missing production provenance boundary for the existing `max_active_workers: NonZeroUsize` field and required a separate exact-source checkpoint to select one concrete source mechanism before any source materialization. C03e-JM performs only that concrete-source selection.

## 2. Exact-source observations

On exact C03e-JL head `fc50e6152c2827329bfc03929702ffbb608e7b0e`, `crates/prw-agent/src/linux_bootstrap.rs` establishes all of the following:

1. `LinuxAgentRemoteProcessOperationInputs` stores `max_active_workers` as `NonZeroUsize`; zero is therefore outside the already-selected typed input domain.
2. The same production remote-input assembly lane already owns fixed, non-secret process-environment sources:
   - `PRW_REMOTE_BIND_ADDR_ENV = "PRW_REMOTE_BIND_ADDR"` with `load_linux_agent_remote_bind_addr_from_env()`;
   - `PRW_REMOTE_PEER_DEVICE_ID_ENV = "PRW_REMOTE_PEER_DEVICE_ID"` with `load_linux_agent_remote_peer_device_id_from_env()`.
3. Those existing loaders perform fixed-name process-local acquisition, fail closed, expose bounded errors, do not disclose configured values, and do not add fallback/default/retry semantics.
4. No exact C03e-JL source defines `PRW_REMOTE_MAX_ACTIVE_WORKERS`, `MAX_ACTIVE_WORKERS`, or another authoritative production worker-limit source.
5. `initial_runtime_config()` contains historical Phase 101 fixed runtime capacities, but those values belong to a separate local runtime configuration owner and are not connected by exact source semantics to `LinuxAgentRemoteProcessOperationInputs::max_active_workers`.
6. `LocalLinuxWorkerCapacity` is accounting over an already-selected bound and is not production-source authority.
7. The worker limit is non-secret operational configuration. No exact-source evidence justifies treating it as a credential, identity value, registry fact, policy principal, or network-discovered authority.

The nearest exact-source precedent is therefore the existing fixed non-secret process-environment configuration lane used by the adjacent production remote-input fields. C03e-JM selects that mechanism explicitly rather than inheriting it implicitly from C03e-JL.

## 3. Selected concrete source mechanism

C03e-JM selects exactly one concrete source mechanism for the C03e-JL worker-limit provenance boundary:

> one fixed, non-secret process environment variable named `PRW_REMOTE_MAX_ACTIVE_WORKERS`, acquired by `prw-agent` at the future source-call boundary and converted fail-closed into the existing `NonZeroUsize` worker limit.

The future source constant is selected as:

```rust
pub const PRW_REMOTE_MAX_ACTIVE_WORKERS_ENV: &str = "PRW_REMOTE_MAX_ACTIVE_WORKERS";
```

The future source API shape selected for the materialization successor is:

```text
std::env::var_os(PRW_REMOTE_MAX_ACTIVE_WORKERS_ENV)
    -> exact source parser
    -> NonZeroUsize
```

The source is process configuration only. It is not identity, authentication, authorization, registry, endpoint, requester/rendezvous, admission-request, or runtime-observation authority.

## 4. Exact value grammar and conversion

The selected environment value represents one strictly-positive decimal worker bound.

The materialization successor must preserve this grammar and conversion:

1. The environment variable must be present.
2. The platform value must be Unicode.
3. The Unicode string must be non-empty.
4. The accepted lexical form is ASCII decimal digits only: `[0-9]+`.
5. No trimming or normalization is selected. Leading/trailing whitespace is invalid.
6. No leading sign, sign prefix, decimal point, exponent, underscore, locale separator, suffix, or unit is accepted.
7. The decimal integer must fit in `usize` on the compiling target.
8. The parsed integer must be strictly greater than zero.
9. Successful conversion yields exactly `NonZeroUsize` with the parsed magnitude; no clamp, scaling, rounding, substitution, or host-derived adjustment is allowed.

Leading zeroes on a positive value are not semantically significant and may parse to the same positive magnitude; the all-zero value remains invalid because it converts to zero.

## 5. Selected failure semantics

The future source materialization must expose a bounded source error owned by `prw-agent` for this configuration boundary. C03e-JM selects these semantic classes:

- `Missing` — the fixed variable is absent;
- `NonUnicode` — the fixed variable exists but cannot be represented as Unicode;
- `InvalidValue` — the Unicode value violates the selected decimal grammar, does not fit in `usize`, or parses to zero.

The exact future type/function names selected for the narrow source implementation are:

```text
LinuxAgentRemoteMaxActiveWorkersSourceError
parse_linux_agent_remote_max_active_workers_value(...)
load_linux_agent_remote_max_active_workers_from_env()
```

The bounded diagnostic surface must not include or echo the configured environment value. Missing, malformed, zero, overflow, and non-Unicode failures are terminal for this source call. No fallback value, default value, retry, alternate variable, stale cache, dynamic refresh, CPU-count derivation, memory derivation, active-worker derivation, or other ambient host derivation is selected.

## 6. Acquisition and custody semantics

C03e-JM selects fixed-name process-local acquisition only.

The materialized loader must read `PRW_REMOTE_MAX_ACTIVE_WORKERS` exactly once per loader invocation. C03e-JM does not select polling, watching, dynamic reconfiguration, signal-triggered reload, file/provider lookup, registry lookup, or control-plane lookup.

A successfully sourced `NonZeroUsize` remains an ordinary already-typed value. Merely sourcing it must not:

- create worker slots;
- mutate current active-worker accounting;
- start or stop workers;
- bind or accept a listener;
- authenticate or authorize a session;
- create expected requests;
- construct capability authority;
- publish requester/rendezvous state;
- mutate registry/policy/session state;
- publish readiness;
- spawn tasks;
- alter process lifecycle.

## 7. Materialization ceiling for the immediate successor

After C03e-JM is canonically validated and evidence-recorded, the immediate source-materialization successor may modify only the minimum source surface in:

`crates/prw-agent/src/linux_bootstrap.rs`

That successor may add only:

1. `PRW_REMOTE_MAX_ACTIVE_WORKERS_ENV` with exact value `PRW_REMOTE_MAX_ACTIVE_WORKERS`;
2. the bounded `LinuxAgentRemoteMaxActiveWorkersSourceError` source-error surface;
3. one pure parser over an injected optional OS string/value boundary suitable for deterministic tests;
4. `load_linux_agent_remote_max_active_workers_from_env()` using the fixed environment name exactly once;
5. focused tests for constant/signature, missing, non-Unicode, empty, whitespace/sign/non-decimal malformed values, zero, positive values, leading-zero positive values, target-`usize` overflow where representable by test input, and exact positive magnitude preservation.

The immediate successor MUST NOT yet pass the sourced value into `LinuxAgentRemoteProcessOperationInputs::new(...)` or any production aggregate/input-population helper. Executable input population remains a later separately selected boundary after the source itself is materialized and validated.

## 8. Mechanisms explicitly rejected by this selection

C03e-JM does not select:

- command-line flags;
- systemd credentials;
- systemd credential files;
- configuration files;
- registry/database/control-plane values;
- a compile-time production worker-limit constant;
- Phase 101 `initial_runtime_config()` worker capacity as this value;
- CPU/memory/connection-derived auto-sizing;
- current active-worker count as configuration;
- a hard-coded default;
- a fallback or alternate environment-variable name;
- dynamic environment re-read/reload semantics;
- secret/configuration-store abstraction;
- requester/rendezvous-derived capacity;
- expected-request queue capacity as this worker limit.

The selected environment source is a direct design decision of C03e-JM, grounded in exact adjacent production-source precedent. It does not claim that `PRW_REMOTE_MAX_ACTIVE_WORKERS` existed before this checkpoint.

## 9. Identity and authority invariants

C03e-JM preserves the PRW identity invariant:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

`PRW_REMOTE_MAX_ACTIVE_WORKERS` is scheduling/configuration provenance only. It MUST NOT be used as, compared to, or derived into:

- `DeviceId`;
- `PeerConnectivityIdentity`;
- `TransportIdentity`;
- `SessionId`;
- requester identity;
- requester/rendezvous target identity;
- endpoint or candidate identity;
- capability identity, policy principal, or authorization evidence;
- PRWM `request_id`.

IP addresses remain transient reachability endpoints only. PRWM `request_id` remains correlation only.

## 10. Exact exclusions

C03e-JM itself does not materialize or authorize:

- Rust/source/runtime changes;
- environment mutation or provisioning;
- a concrete production worker-limit value;
- source invocation by `run()`, `main.rs`, or another executable caller;
- worker-limit executable input population;
- full production aggregate input assembly;
- dynamic worker-limit mutation;
- worker-accounting redesign;
- worker spawn/cancel behavior changes;
- bind/listener/readiness activation;
- peer lookup or peer re-selection;
- capability-authority production population;
- session-authentication production population/state restoration;
- expected-request producer/channel lifecycle production;
- admission timing/clock production;
- completion/rejection/admission-failure callback production;
- requester/rendezvous policy/provider population or invocation;
- registry or policy mutation;
- candidate publication, traversal, dialing, retry, reconnect, rebind, or rebootstrap;
- service/systemd/package/security/credential/certificate/trust/RBAC mutation;
- database/schema/control-plane mutation;
- deployment, restart, recovery activation, or merge;
- PR readiness conversion, PR close, branch deletion, or history rewrite.

## 11. Successor rule

After C03e-JM closes, **STOP**.

The next checkpoint may materialize only the selected fixed environment source in `crates/prw-agent/src/linux_bootstrap.rs`, bounded by Section 7. It must begin from the exact closed C03e-JM head, re-audit that exact source state, and preserve the selected failure/value semantics.

After that source-materialization checkpoint closes, a separate fresh selection is required before the sourced `NonZeroUsize` may be populated into production `LinuxAgentRemoteProcessOperationInputs`. No source materialization, executable-input population, or runtime activation is inherited merely from this selection contract.

## 12. Validation target for C03e-JM

C03e-JM is documentation-only and may close only if all of the following are true on the exact final head:

- exact predecessor remains C03e-JL head `fc50e6152c2827329bfc03929702ffbb608e7b0e` as merge base;
- branch is ahead only by the intended documentation commit(s) and zero behind;
- exactly one changed path exists:
  `contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_JM_PRODUCTION_WORKER_LIMIT_ENV_SOURCE_SELECTION_STAGING.md`;
- there are zero Rust/source/runtime/manifest/lockfile/workflow/Android/packaging/host changes;
- repository CI required for the exact final docs-only head is observed and recorded without inheriting verdicts from another head;
- skipped workflows are recorded only as skipped, never as PASS;
- immutable Drive evidence is frozen, uploaded, raw-read back, and verified byte-exact before closure metadata is claimed;
- the PR remains draft, open, and unmerged.

Only then may `C03E_JM_PRODUCTION_WORKER_LIMIT_ENV_SOURCE_SELECTED` and `CLOSED_PRODUCTION_WORKER_LIMIT_ENV_SOURCE_SELECTION` be claimed.