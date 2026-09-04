# Desktop Functional Management Slice C03e-KJ — Production Durable Capability Typed Post-Auth Ingress Processor Selection

Status: `SELECTION_STAGING`

Gate:

`C03E_KJ_PRODUCTION_DURABLE_CAPABILITY_TYPED_POST_AUTH_INGRESS_PROCESSOR_SELECTED`

Intended closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_TYPED_POST_AUTH_INGRESS_PROCESSOR_SELECTION`

## 1. Purpose

C03e-KJ is a documentation-only corrective reselection after exact-head C03e-KI validation exposed that the C03e-KH source ceiling was incomplete.

KJ does not treat the failed KI candidate as a valid predecessor and does not extend KI in place. The clean source predecessor remains the exact closed C03e-KH head.

The purpose of KJ is to select a smaller, compile-isolated durable-capability ingress processor that can be materialized without changing any existing EV/EX/FI/FQ/FU caller signature and without replacing the existing `SharedCurrentCapabilityAuthority<P>` dependency used by requester/rendezvous DR and real-admission ownership.

KJ selects no runtime activation and no production aggregate or executable caller population.

## 2. Authority hierarchy

Selection authority is, in order:

1. exact GitHub branch/head/tree/file state;
2. exact C03e-KH source topology;
3. exact C03e-KI validation failure evidence, used only as discovery evidence and not as a successful implementation authority;
4. exact-final-head CI for this documentation-only KJ selection;
5. immutable Google Drive readback evidence after validation.

Project Source handoffs remain continuity material only where live GitHub or exact-head CI provides newer evidence.

## 3. Exact clean predecessor

Closed predecessor:

`C03e-KH — Production durable capability post-auth ingress caller migration selection`

Branch:

`phase-152-c03e-kh-production-durable-capability-post-auth-ingress-caller-migration-selection`

Exact KH head / KJ merge base:

`060c272f88a8d43e1dd462dc4333af45c79dadb8`

Exact KH tree:

`45a2d8c2cf878ed0f2d5dadd936eb3e45968f181`

Relevant exact KH blobs:

- `crates/prw-agent/src/remote_session_capability_runtime.rs`
  - `fbeb7016209d02d348e1b04ac4160a1c0895badc`
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`
  - `294b64ba33ba8a4b1d4ed595bf473f6d225ec0d5`
- `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`
  - `bc0b9c49471d515b721c9cf47cd27ec3111f32ca`
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`
  - `bc4520b2c13308b446230b43a2650d02e5b42cc2`
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker/repeated_real_admission_requester_aware_persistent_fl_integration.rs`
  - `2a07f03bb3c1739e4963a16c0ba7c30ae753d24e`

The existing C03e-KG durable helper remains:

`AuthenticatedRemoteSessionRuntimeOwner::process_production_durable_capability_transaction(...)`

at exact blob:

`f95c4f1bb2d424ea7d15647ecb1d6153aebc480c`

KJ does not select mutation of that helper.

## 4. KI discovery evidence and why KH must be decomposed

The failed C03e-KI branch is:

`phase-152-c03e-ki-production-durable-capability-post-auth-ingress-caller-migration-source-materialization`

Current failed KI head:

`d72511c895a6eb979a2b6f298aa6c8f3a0cc4801`

PR #420 remains draft, open and unmerged with status:

`SOURCE MATERIALIZATION — VALIDATION_FAILED — RESELECTION_REQUIRED`

KI is not closed and has no immutable closure audit.

Exact Rust validation #1555 (`33895785901`, job `101097967430`) passed checkout, prerequisites, toolchain, locked dependency graph and rustfmt, then failed during Clippy compilation.

The compiler proved that replacing the existing EV/EX `SharedCurrentCapabilityAuthority<P>` parameter with `ProductionDurableCapabilityAuthority` is not a two-path change:

1. `requester_rendezvous_retained_custody_dr_continuation.rs` has two existing callers of `run_repeated_post_auth_control_stream_ingress(...)` that still supply `&SharedCurrentCapabilityAuthority<P>`;
2. `recoverable_spawned_requester_rendezvous_worker.rs` retains an existing test that constructs the predecessor `Bridge(...)` parent ingress failure;
3. the requester-aware FI lifecycle uses the same shared-current authority for requester DR continuation after an ingress handoff;
4. FQ and FU propagate that shared-current authority higher through spawned/persistent requester-aware ownership and real admission.

Therefore the shared-current authority is not merely a legacy capability parameter that can be globally replaced. It remains a distinct dependency for requester DR and higher existing ownership composition.

## 5. Corrected architectural decomposition

KJ locks the following distinction:

`production durable capability authority != shared-current requester/admission authority`

The two authority roles must not be conflated.

For capability-family processing, the already-materialized KG helper is the selected durable authority boundary:

`already-read PostAuthCapabilityTransaction`
→ retained bound-session transport identity + lease
→ `ProductionDurableCapabilityAuthority::authorize_capability_transaction(...)`
→ authorized dispatch
→ exact same-stream capability response.

For requester/rendezvous DR, the existing shared-current authority remains unchanged and separately authoritative for the already-established DR composition.

For real admission and higher requester-aware worker custody, existing authority dependencies remain unchanged unless a later checkpoint explicitly selects a dual-authority aggregate/propagation boundary.

## 6. Selected immediate source successor

The immediate source successor after KJ may materialize only one new dormant typed-ingress processing seam beneath the current EV accept/read boundary.

The selected method name is exactly:

`process_existing_post_auth_control_stream_ingress_with_production_durable_capability(...)`

The method is selected as an Agent-internal method on `AuthenticatedRemoteSessionRuntimeOwner`.

Selected inputs are exactly:

- `&mut self`;
- borrowed `&ProductionDurableCapabilityAuthority`;
- `now_unix_seconds: u64`;
- `&mut D` where `D: CapabilityDispatcher + Send`;
- one already-read `PostAuthControlStreamIngress` by value.

It must not accept a raw control stream and must not call `accept_control_stream()` or `receive_post_auth_control_stream_ingress(...)`.

It therefore creates no second accept/read owner and cannot independently consume an additional frame.

## 7. Selected typed-ingress behavior

For:

`PostAuthControlStreamIngress::Capability(transaction)`

the new method must transfer the exact already-read transaction by value into:

`AuthenticatedRemoteSessionRuntimeOwner::process_production_durable_capability_transaction(...)`

using the supplied durable authority, verifier time and dispatcher.

Only KG helper success may produce:

`AuthenticatedRemoteSessionPostAuthIngressOutcome::CapabilityProcessed`

For:

`PostAuthControlStreamIngress::RequesterRendezvous(transaction)`

the new method must preserve the current EV requester handoff semantics exactly:

1. read the nominated logical target only from the exact already-decoded requester transaction;
2. adapt it through the existing target-intent helper;
3. derive requester identity only through the retained authenticated-session owner;
4. return the exact same `RequesterRendezvousResponseStreamCustodyHandoff` with the same transaction/stream custody.

It must not run requester DR, provider execution or response I/O.

For:

`PostAuthControlStreamIngress::CandidatePublication(transaction)`

the method must preserve the existing fail-closed higher-owner barrier and return the existing candidate-publication-not-selected classification without invoking candidate publication authority/provider behavior.

The candidate transaction must not be reinterpreted as capability or requester traffic.

## 8. Additive parent error selection

The immediate source successor may add one new variant to the existing:

`AuthenticatedRemoteSessionPostAuthIngressTransactionError`

The exact selected new variant is:

`ProductionDurableCapability(ProductionDurableCapabilityTransactionError)`

The existing predecessor variants must remain present and semantically unchanged:

- `Accept(RemoteServerTransportRuntimeError)`
- `Ingress(PostAuthControlStreamIngressError)`
- `Bridge(RemoteBridgeError)`
- `CapabilityResponse(CapabilityRequestWireError)`
- `CandidatePublicationHandoffNotSelected`

This is intentionally additive. The successor must not delete, rename or repurpose `Bridge` or `CapabilityResponse`, because existing legacy EV/EX and higher tests/callers remain outside this source materialization.

The new durable variant must preserve its exact nested KG error as `std::error::Error::source()` and may add only minimal exact `From<ProductionDurableCapabilityTransactionError>` plumbing.

No error flattening from nested KG `Authority`, `Dispatch`, or `Response` is selected.

## 9. Exact source ceiling

The immediate source successor may mutate exactly these two Agent paths and no others:

1. `crates/prw-agent/src/remote_session_capability_runtime.rs`
2. `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`

The following are explicitly outside the immediate source ceiling:

- `authenticated_remote_session_runtime.rs` / KG helper;
- `requester_rendezvous_retained_custody_dr_continuation.rs`;
- `remote_session_executor_runtime/recoverable_spawned_requester_rendezvous_worker.rs`;
- recoverable persistent requester-aware worker custody;
- repeated real-admission requester-aware persistent integration;
- `linux_bootstrap.rs`;
- `main.rs`;
- manifests, lockfiles and workflows.

If materializing the selected typed processor requires a third repository path, the successor must stop and return to selection rather than widen scope.

## 10. Existing EV/EX/FI/FQ/FU preservation

The immediate successor must leave these existing method signatures and call relationships unchanged:

- `process_one_post_auth_control_stream_ingress(...)`;
- `run_repeated_post_auth_control_stream_ingress(...)`;
- `run_repeated_post_auth_control_stream_ingress_worker(...)`;
- requester-aware FI/FJ/FL serial lifecycle and worker;
- FQ recoverable spawned requester-aware worker;
- FS persistent custody;
- FU repeated real-admission requester-aware integration.

The existing methods continue to compile against `SharedCurrentCapabilityAuthority<P>` exactly as before.

The new typed durable processor is dormant and uninvoked by those legacy/higher owners until a later separately selected caller-propagation boundary.

No legacy behavior is deleted merely to make the durable path compile.

## 11. Why the selected seam is not a second acceptor

The new seam starts after family ingress has already been decoded into one typed `PostAuthControlStreamIngress` value.

It owns no stream acceptance and performs no family ingress read itself.

Therefore source materialization does not introduce:

- a second `accept_control_stream()` call;
- a second PRWM frame read;
- speculative pre-accept;
- parallel family queues;
- competing control-stream ownership;
- retry/reconnect behavior.

Any later caller that accepts/reads and then transfers a typed ingress into this processor requires its own separately reviewed ownership gate.

## 12. Security and authority invariants

Continue to preserve:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

`request_id` remains correlation only.

Transport endpoint/address remains reachability data, not logical identity.

The durable capability helper must continue sourcing presented transport identity and lease only from the retained bound session.

Requester identity must continue deriving only from the authenticated PRW application session.

A decoded requester target remains nomination, not requester authorization.

Candidate publication decode remains insufficient to authorize candidate provider execution.

No successful decode, lookup, transport connection or correlation match is upgraded into capability authority.

## 13. Explicit exclusions

KJ does not select or authorize:

- source materialization in KJ itself;
- mutation of the KG helper;
- replacement of existing EV/EX signatures;
- deletion of legacy `Bridge` or `CapabilityResponse` error variants;
- FI/FQ/FU dual-authority propagation;
- production aggregate replacement;
- executable construction/population of durable capability authority;
- session-authentication population;
- verifier-time source population;
- dispatcher production population;
- expected-request source population;
- callback population;
- requester/rendezvous provider population or invocation;
- positive production capability policy;
- registry/provider mutation;
- candidate publication/traversal/dialing/retry/reconnect/rebind/rebootstrap;
- listener/bind/readiness/runtime/network activation;
- `run()` or `main.rs` mutation;
- manifest or lockfile mutation;
- workflow mutation;
- service/systemd/package/security/credential/certificate/private-key/trust/RBAC mutation;
- database/schema/control-plane mutation;
- deployment/restart/recovery activation;
- repository visibility/configuration mutation;
- merge;
- PR close;
- ready-for-review conversion;
- branch deletion;
- history rewrite/force-push.

## 14. Required validation for the immediate source successor

The successor must be validated only at its exact final head.

Required source-topology proof:

- exact KJ final head as merge base;
- no behind drift;
- only the two selected Agent paths changed;
- KG helper blob remains byte-identical;
- requester DR / FQ / FU source paths remain byte-identical.

Required Rust validation:

- locked dependency graph;
- rustfmt;
- Clippy with repository warning policy;
- workspace tests;
- workspace build.

If Android validation is triggered for the exact source head, its terminal result must also be recorded before closure and any failure blocks closure.

Skipped workflows must be recorded as skipped, never as PASS.

No superseded candidate validation may be reused as final evidence.

## 15. KJ closure rule

KJ itself may close only after its own exact-final-head required documentation-selection CI succeeds and one immutable raw Markdown Drive audit is published and read back byte-for-byte.

The KJ audit must use exact-title presearch, one canonical upload, exact raw readback byte/hash verification, and exact-title postsearch proving exactly one canonical artifact.

The KJ PR must remain draft, open and unmerged.

## 16. STOP

After KJ closure, STOP before source materialization.

The next checkpoint may materialize only the additive durable parent error plus the one already-read typed-ingress processor selected here, in exactly the two selected Agent paths.

After that source materialization closes, perform a fresh exact-head audit before selecting any EV/EX caller migration, FI dual-authority propagation, FQ/FU authority composition, production aggregate, executable caller, startup policy, listener/runtime/network activation, or allow-bearing production policy.
