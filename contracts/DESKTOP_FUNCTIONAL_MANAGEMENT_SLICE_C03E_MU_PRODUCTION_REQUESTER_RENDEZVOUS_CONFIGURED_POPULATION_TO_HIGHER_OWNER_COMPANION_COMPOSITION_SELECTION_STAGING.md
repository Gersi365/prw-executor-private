# Desktop Functional Management Slice C03e-MU — Production Requester/Rendezvous Configured Population to Higher-Owner Companion Composition Selection — STAGING

Status: `SELECTION / STAGING`

Gate:

`C03E_MU_PRODUCTION_REQUESTER_RENDEZVOUS_CONFIGURED_POPULATION_TO_HIGHER_OWNER_COMPANION_COMPOSITION_SELECTED`

## 1. Purpose

C03e-MU selects the narrowest composition boundary after C03e-MT.

C03e-MT already materializes one dormant configured requester/rendezvous population helper that obtains the fixed requester/rendezvous max-records process value through the C03e-MR source and passes the exact target-platform `usize` unchanged into the existing C03e-MP combined production population helper.

C03e-LX already materializes one crate-private higher-owner companion assembly that consumes already-typed production durable requester/rendezvous inputs, constructs the existing projection-capable one-shot operation, and passes that operation directly to the existing generic Linux remote-companion runner.

C03e-MU selects only how a future source successor may join those two already-materialized seams without introducing an executable invocation site.

This checkpoint does not materialize Rust source, call `run()`, modify `main.rs`, start a listener, publish readiness, activate networking, deploy, merge, or widen authority.

## 2. Exact predecessor authority

Predecessor checkpoint: `C03e-MT`.

Exact predecessor head:

`61fff903bfad0b6c433b07991294386c262c2610`

Exact predecessor tree:

`c15cfc58f367dfd71f615a61f7d6e9bf50360b12`

Exact C03e-MT target blob:

`eeca0e1dbd42eb177db75f6bf4ea3df2f7d1ced3`

C03e-MT canonical source path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

C03e-MT canonical immutable evidence:

`C03E_MT_PRODUCTION_REQUESTER_RENDEZVOUS_CONFIGURED_COMBINED_POPULATION_SOURCE_MATERIALIZATION_AUDIT_2026-09-07.md`

Drive ID:

`15HpwNwxBjgub6Mvnyzsnpo36GRivD_BT`

Exact evidence bytes:

`13409`

SHA-256:

`1389b3e52c829fd08c4ee18eeec3453a5cfcd0557fee58128225aed928354f28`

## 3. Existing source seams selected for composition

The future source successor may reuse exactly these existing functions and no replacement implementations.

### 3.1 Configured population source seam

Existing C03e-MT helper:

`linux_agent_production_durable_reachability_requester_rendezvous_remote_process_operation_inputs_from_configured_production_sources`

This helper already:

1. loads the fixed requester/rendezvous max-records environment source exactly once;
2. preserves source failure as the bounded configured-population source error;
3. passes the exact parsed target-platform `usize` unchanged;
4. invokes the existing C03e-MP explicit-capacity combined population helper exactly once only after source success;
5. preserves zero until the existing requester/rendezvous provider constructor enforces its non-zero invariant;
6. returns one fully-populated higher-owner input aggregate on success.

### 3.2 Higher-owner companion assembly seam

Existing C03e-LX helper:

`run_with_production_durable_reachability_requester_rendezvous_remote_process_companion_with_production_durable_capability_projection`

This helper already:

1. consumes an already-typed higher-owner input aggregate by value;
2. constructs the existing projection-capable higher-owner operation exactly once;
3. delegates that exact operation to the existing generic Linux remote-companion runner exactly once;
4. returns the existing `LinuxAgentBootstrapWithRemoteReport` / `LinuxAgentBootstrapStartFailure` result domain.

C03e-MU does not replace, duplicate, fork, or widen either seam.

## 4. Selected future composition

A future immediate source successor may add one crate-private async composition helper that accepts the same still-unselected caller-supplied production provenance currently required by C03e-MT:

- `expected_requests`;
- `admission_timing`;
- `on_completion`;
- `on_rejection`;
- `on_admission_failure`.

The helper must perform exactly this order:

1. invoke the existing C03e-MT configured production-population helper exactly once;
2. await that helper exactly once;
3. on population failure, return immediately without invoking the C03e-LX companion assembly;
4. on population success, move the exact returned higher-owner inputs by value into the existing C03e-LX companion assembly exactly once;
5. return the exact existing companion result, wrapped only in the bounded outer error selected below.

No cloned population result, second population attempt, alternate source, fallback, retry, partial owner, or duplicate companion assembly is allowed.

## 5. Selected bounded outer error

Because C03e-MT population and C03e-LX bootstrap/companion assembly have distinct existing error domains, the immediate source successor may add exactly one bounded Agent-local outer error.

Conceptual variants are exactly:

1. `ConfiguredPopulation(LinuxAgentProductionDurableReachabilityRequesterRendezvousConfiguredPopulationError)`
2. `Bootstrap(LinuxAgentBootstrapStartFailure)`

The names may be adjusted only if canonical Rust formatting/naming requires a mechanically equivalent form; the semantic partition must remain exactly two-stage.

The outer error may implement only ordinary bounded error plumbing:

- `Debug`;
- `Clone`, `Copy`, `PartialEq`, `Eq` only if supported by both nested error types without widening them;
- `Display` with bounded non-secret messages;
- `std::error::Error` with nested `source()`;
- `From` conversions for the two exact nested error domains.

No configured value, peer identifier, endpoint, secret, credential, token, challenge material, capability grant, or host-derived detail may be added to error text.

If either nested error domain cannot support a requested derive, the future source successor must omit that derive rather than alter the nested type.

## 6. Failure ordering and short-circuit contract

The future composition must remain strictly fail-closed.

### Population failure

If C03e-MT configured population fails:

- return `ConfiguredPopulation(...)`;
- do not invoke C03e-LX;
- do not construct a second population path;
- do not retry;
- do not synthesize inputs;
- do not activate local or remote runtime through the companion assembly.

### Bootstrap/companion failure

If C03e-MT population succeeds and C03e-LX returns `LinuxAgentBootstrapStartFailure`:

- preserve the exact existing failure as `Bootstrap(...)`;
- do not retry population;
- do not retry companion assembly;
- do not translate the failure into a new exit-code or terminal policy in this checkpoint.

## 7. Runtime-activation boundary

C03e-MU authorizes definition of a callable composition helper only.

The immediate source successor must not add any invocation of that helper from:

- `run()`;
- the Agent executable;
- `main.rs`;
- a startup hook;
- a listener/readiness path;
- a background thread/task;
- test setup that performs real runtime activation.

Therefore source materialization of the helper remains dormant in repository state.

When the helper is eventually invoked by a separately selected executable caller, the existing C03e-LX semantics may run the existing remote-companion bootstrap. That future invocation is explicitly outside C03e-MU and its immediate source successor.

## 8. Remaining provenance intentionally unresolved

C03e-MU does not select concrete production provenance for:

- the expected-request receiver/producer;
- admission timing;
- completion callback;
- rejection callback;
- admission-failure callback.

The immediate source successor must keep these values caller-supplied and pass them unchanged into C03e-MT.

No synthetic channel, default callback, no-op callback, wall-clock source, monotonic-clock policy, durable registry hydration, current authority synchronization, requester binding, dispatcher source, verifier-time source, or admission-time source is selected here.

If a real executable caller requires any such provenance to exist concretely, that requires a later selection checkpoint.

## 9. Identity, authentication, authorization, and capability guardrails

The canonical PRW identity law remains:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

C03e-MU changes none of these authority relationships.

The configured requester/rendezvous capacity value remains process configuration provenance only.

It is not:

- device identity;
- session identity;
- request identity;
- authentication;
- authorization;
- a capability grant;
- requester registration;
- registry membership;
- readiness;
- reachability;
- endpoint authority.

Provider/runtime construction remains distinct from authorization.

No new cryptographic primitive, authentication scheme, capability policy, host-derived privilege, request-controlled endpoint authority, or fallback authority is selected.

## 10. Immediate source-successor ceiling — C03e-MV

If C03e-MU is validated and closed, the immediate source successor may modify exactly one existing Rust path:

`crates/prw-agent/src/production_durable_capability_higher_owner_custody.rs`

The future source successor may add only:

1. imports already needed by the selected outer error/result domain, if not already in scope;
2. one bounded two-stage outer error matching Section 5;
3. ordinary bounded `Display`, `Error`, and `From` plumbing for that error;
4. one crate-private async helper implementing exactly the Section 4 order;
5. focused compile-time/signature/error-shape tests in the same file if useful and non-invasive.

The future source successor must reuse the existing C03e-MT and C03e-LX helpers exactly; it must not duplicate their bodies.

## 11. Immediate source-successor STOP conditions

C03e-MV must STOP without source mutation if implementation requires any of the following:

- a second changed path;
- public API widening;
- `run()` mutation;
- `main.rs` mutation;
- executable invocation wiring;
- concrete expected-request producer selection;
- concrete timing/callback provenance selection;
- new channel construction;
- requester binding or registration;
- current-registry hydration/synchronization;
- capability-policy widening;
- provider implementation change;
- requester/rendezvous max-records parser change;
- worker-limit alias or fallback;
- error-domain mutation outside the new bounded outer error;
- retry/fallback/default behavior;
- new cryptography/authentication/authorization logic;
- listener/readiness/network activation;
- deployment/restart;
- merge/rebase/squash/history rewrite.

Any such need requires a new selection checkpoint.

## 12. Validation requirements

C03e-MU itself is documentation/selection only.

Validation claims must bind to the exact final C03e-MU head.

For the documentation-only selection head:

- run the repository's Rust validation if triggered;
- claim only terminal `SUCCESS` results actually observed;
- classify path-filtered `SKIPPED` workflows as `SKIPPED`, never PASS;
- do not claim Android PASS unless an Android workflow actually runs on the exact final head and succeeds.

The future C03e-MV Rust-source head must receive exact-final-head Rust validation and Android validation when the repository workflow path filters trigger them.

Prior-head success is not valid evidence for a later head.

## 13. Durable evidence requirements

After exact-final-head validation, C03e-MU closure requires one immutable Markdown audit in the canonical PRW Drive evidence parent.

Protocol:

1. exact-title pre-upload search must return `0`;
2. create/upload exactly one canonical audit;
3. raw readback must match the local bytes exactly;
4. byte length and SHA-256 must match exactly;
5. exact-title post-upload search must return exactly `1` canonical artifact;
6. record Drive ID, byte length, SHA-256 and exact CI evidence in the C03e-MU PR body;
7. leave the PR draft/open/unmerged.

## 14. Non-goals

C03e-MU does not:

- materialize Rust source;
- activate the C03e-MT helper;
- activate the C03e-LX companion assembly;
- select a real executable caller;
- select expected-request producer provenance;
- select timing/callback provenance;
- map errors to process exit codes;
- modify existing startup terminal classes;
- create a listener;
- publish readiness;
- expose an endpoint;
- deploy;
- merge;
- close a PR;
- mark a PR ready for review;
- delete a branch;
- rewrite history.

## 15. Selected continuation boundary

C03e-MU closes only the composition decision:

`existing MT configured population -> existing LX higher-owner companion assembly`

with one bounded two-stage outer error and no invocation site.

The selected immediate materialization successor is C03e-MV and is limited to the single Rust path and source ceiling in Sections 10–11.
