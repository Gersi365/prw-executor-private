# Desktop Functional Management Slice C03e-KL — Production Durable Capability Post-Auth Accept/Read Wrapper Selection Staging

Status: `SELECTION_STAGING`
Date: `2026-09-04` (Europe/Tirane)

Gate:

`C03E_KL_PRODUCTION_DURABLE_CAPABILITY_POST_AUTH_ACCEPT_READ_WRAPPER_SELECTED`

Intended closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_POST_AUTH_ACCEPT_READ_WRAPPER_SELECTION`

## 1. Selection purpose

C03e-KL selects only the next dormant Agent-owned composition step after the closed C03e-KK production durable capability typed post-auth ingress processor source materialization.

KK already provides one Agent-internal typed-ingress processor that accepts an already-read `PostAuthControlStreamIngress` by value and routes capability traffic through `ProductionDurableCapabilityAuthority` while preserving requester/rendezvous and candidate-publication family semantics.

KL does not select migration of any existing EV/EX/FI/FQ/FU caller. It selects one additive one-transaction wrapper that owns exactly one authenticated control-stream accept and exactly one typed family-ingress read before delegating the resulting ingress by value to the existing KK processor.

This is a selection-only checkpoint. It changes no Rust source, runtime ownership, manifests, lockfiles, workflows, Android source, service configuration, repository configuration, deployment state, database state, control-plane state, readiness, listener binding, or network behavior.

## 2. Exact predecessor authority

Repository:

`Gersi365/prw-executor-private`

Repository ID:

`1334911207`

Default branch:

`main`

Observed repository visibility:

`public`

Integrated `main` remains:

`7c993fa93977a0bb84e0d030874eee7fd0cae77f`

Commit message:

`Restore main after accidental connector file mutation`

Exact predecessor:

`C03e-KK — Production durable capability typed post-auth ingress processor source materialization`

Exact KK branch:

`phase-152-c03e-kk-production-durable-capability-typed-post-auth-ingress-processor-source-materialization`

Exact KK head:

`c750b86c76832533813e7e7a077c7ed1ac44afbf`

Exact KK tree:

`9d8580762f2fdd711e37a5cafd75032ba04ace28`

KK PR #422 remains draft, open, unmerged and evidence-closed.

KK immutable Drive audit:

`C03E_KK_PRODUCTION_DURABLE_CAPABILITY_TYPED_POST_AUTH_INGRESS_PROCESSOR_SOURCE_MATERIALIZATION_AUDIT_2026-09-04.md`

Drive file ID:

`1ljD6LXRVauNlljYb4AwL2qw80uA70v_q`

Failed KI PR #420 remains discovery evidence only and is not a KL predecessor.

## 3. Fresh post-KK source findings

Exact KK source path inspected:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`

Exact KK blob:

`efd935734008e0d914746233f0e1e62fa50137e4`

KK materialized:

`AuthenticatedRemoteSessionRuntimeOwner::process_existing_post_auth_control_stream_ingress_with_production_durable_capability(...)`

That method accepts one already-read typed ingress by value. It performs no `accept_control_stream()` call and no `receive_post_auth_control_stream_ingress(...)` call.

The existing C03e-EV method remains separately present:

`process_one_post_auth_control_stream_ingress(...)`

It still owns exactly one retained-peer `accept_control_stream()` and exactly one `receive_post_auth_control_stream_ingress(...)` read/classification, but its capability arm still uses the legacy `SharedCurrentCapabilityAuthority<P>` composition.

The existing EX repeated loop and worker still call that EV method and still carry `SharedCurrentCapabilityAuthority<P>`.

The failed KI attempt proved that replacing EV authority in place creates compiler-coupled propagation into higher requester-aware callers. KL therefore does not select any in-place EV/EX migration.

## 4. Selected additive wrapper

KL selects one new dormant Agent-internal async method on the existing authenticated remote-session runtime owner.

Selected semantic name:

`process_one_post_auth_control_stream_ingress_with_production_durable_capability(...)`

The later source materialization may choose the exact Rust line wrapping/rustfmt shape, but the semantic method name and input/return contract are selected here.

Selected inputs:

- `&mut self`;
- borrowed `&ProductionDurableCapabilityAuthority`;
- `now_unix_seconds: u64` supplied by the caller;
- mutable `&mut D` where `D: CapabilityDispatcher + Send`.

Selected return:

`Result<AuthenticatedRemoteSessionPostAuthIngressOutcome, AuthenticatedRemoteSessionPostAuthIngressTransactionError>`

Selected execution order is exactly:

1. `self.peer.accept_control_stream().await?` once;
2. transfer the accepted stream by value into `receive_post_auth_control_stream_ingress(stream).await?` once;
3. transfer the resulting exact `PostAuthControlStreamIngress` by value into the existing KK method `process_existing_post_auth_control_stream_ingress_with_production_durable_capability(...)`;
4. return that KK result unchanged.

No family-specific logic is reimplemented in the wrapper.

## 5. Why this wrapper is selected instead of EV replacement

The wrapper is additive and dormant.

It does not alter:

- `process_one_post_auth_control_stream_ingress(...)`;
- `run_repeated_post_auth_control_stream_ingress(...)`;
- `run_repeated_post_auth_control_stream_ingress_worker(...)`;
- FI requester-aware serial lifecycle/DR continuation;
- FQ recoverable spawned requester-aware worker;
- FS persistent requester-aware custody;
- FU repeated real-admission requester-aware integration.

Therefore no existing caller must gain a second authority parameter and no `SharedCurrentCapabilityAuthority<P>` parameter is replaced.

The wrapper also does not invent a dual-authority aggregate. It takes only the durable capability authority because requester/rendezvous and candidate-publication behavior after the typed read are already encapsulated by the KK processor and require no legacy capability authority input.

## 6. Exact source-successor ceiling

The later KL source materialization may change at most one repository source path:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`

No second Rust source path is selected.

In particular, the following are read-only and must remain byte-identical in the KL source successor unless a separate new gate is opened:

- `crates/prw-agent/src/remote_session_capability_runtime.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`;
- FI requester DR continuation;
- FQ recoverable spawned requester-aware worker;
- FU repeated real-admission requester-aware integration;
- `linux_bootstrap.rs`;
- `main.rs`;
- manifests and lockfiles;
- workflow files;
- Android source.

If correct compilation requires a second source path, public API expansion, manifest/lockfile change, production aggregate mutation, caller migration, higher authority propagation, or runtime activation, STOP and open a new selection gate.

## 7. Permitted future source changes under KL

Only the following are selected:

1. add the one dormant accept/read wrapper described above in the exact selected source path;
2. add only comments/doc comments and `#[allow(...)]` annotations directly required for that dormant method;
3. add focused tests in that same source file only if necessary to prove the wrapper shape without creating runtime invocation;
4. accept rustfmt/Clippy accommodations that do not change semantics.

No existing method body or signature may be changed merely for convenience.

No legacy fallback or dynamic authority selection may be added to the new wrapper.

## 8. Exact ownership and read invariants

The future wrapper must prove all of the following:

1. exactly one retained authenticated peer owns the accept;
2. exactly one control stream is accepted per wrapper invocation;
3. the accepted stream is moved by value into the existing typed family-ingress decoder;
4. exactly one bounded family-ingress frame is read/classified;
5. no second accept occurs;
6. no second family-ingress read occurs;
7. no speculative pre-accept, queue, parallel family worker or competing stream owner is introduced;
8. the resulting typed ingress is moved by value into the existing KK processor;
9. the wrapper itself performs no capability authorization, dispatch, requester adaptation, candidate-publication execution or response write.

## 9. Capability-family proof obligations

For capability ingress, the selected chain is:

`retained authenticated peer accept`
→ `receive_post_auth_control_stream_ingress(...)`
→ `PostAuthControlStreamIngress::Capability(transaction)`
→ existing KK typed processor
→ existing KG durable transaction helper
→ `ProductionDurableCapabilityAuthority`
→ authorized dispatch
→ exact same transaction response stream.

Required invariants:

- transport identity and retained lease remain sourced only from the existing KG `BoundRemoteSession`;
- durable capability authorization occurs before dispatch;
- durable-registry synchronization is not held across dispatch or response I/O;
- exact KG `Authority` / `Dispatch` / `Response` failure provenance remains nested under the KK parent error variant;
- no legacy `SharedCurrentCapabilityAuthority<P>` fallback exists inside the new wrapper;
- no request reconstruction from `request_frame()` is performed by the new wrapper.

## 10. Requester/rendezvous proof obligations

For requester/rendezvous ingress, the wrapper performs no requester logic itself.

The exact typed transaction must pass unchanged into the KK processor, which preserves:

- logical target nomination only from the strict decoded requester transaction;
- requester identity only from the retained authenticated PRW application session;
- exact response-stream custody handoff;
- no requester DR/provider execution in this wrapper/processor stage;
- no requester response construction/write;
- no dialing, candidate selection, retry or readiness behavior.

Outer `request_id` remains correlation only.

## 11. Candidate-publication proof obligations

For candidate-publication ingress, the exact typed transaction must pass unchanged into the KK processor.

The existing fail-closed result remains:

`AuthenticatedRemoteSessionPostAuthIngressTransactionError::CandidatePublicationHandoffNotSelected`

The wrapper must not invoke candidate publication authority/provider semantics, reinterpret candidate traffic as capability/requester traffic, write a candidate response, or accept another stream.

## 12. Error preservation

The wrapper uses the already-existing parent error surface.

Accept failure remains:

`AuthenticatedRemoteSessionPostAuthIngressTransactionError::Accept(...)`

Typed ingress read/decode failure remains:

`AuthenticatedRemoteSessionPostAuthIngressTransactionError::Ingress(...)`

Durable capability failure remains:

`AuthenticatedRemoteSessionPostAuthIngressTransactionError::ProductionDurableCapability(...)`

Candidate publication remains the existing fail-closed not-selected variant.

No new error enum or error translation is selected.

## 13. Verifier-time boundary

The wrapper receives `now_unix_seconds` from its caller.

It does not sample verifier time itself.

No clock source, callback, environment lookup or production time population is selected.

A later repeated-loop selection, if any, must separately prove exactly-once time sampling per transaction before invoking this wrapper.

## 14. Dormancy and activation boundary

The selected wrapper remains dormant and uninvoked by production executable source.

KL does not select:

- replacement of the current EV call chain;
- EX repeated durable loop integration;
- FI/FQ/FU authority propagation;
- dual-authority worker composition;
- production aggregate changes;
- durable-authority executable population;
- session-authentication population;
- dispatcher/time/callback population;
- requester provider execution;
- candidate provider execution;
- listener/bind/readiness/runtime/network activation.

The new wrapper is source material only until a later, separately selected caller-integration gate.

## 15. Explicit exclusions

KL does not perform or authorize:

- mutation of the existing KG helper;
- mutation of the KK parent error path;
- deletion/repurposing of legacy `Bridge` or `CapabilityResponse` variants;
- existing EV/EX signature changes;
- FI/FQ/FU dual-authority propagation;
- production aggregate replacement;
- executable population of durable authority;
- positive capability policy/grants;
- registry/provider mutation;
- candidate traversal/dial/retry/reconnect/rebind/rebootstrap;
- requester DR/provider invocation;
- peer-close policy change;
- task spawn or queue creation;
- listener/readiness/runtime/network activation;
- `run()`/`main.rs` mutation;
- manifest/lockfile/workflow mutation;
- Android mutation;
- deployment/restart/recovery;
- database/schema/control-plane mutation;
- repository visibility/configuration mutation;
- merge, PR close, ready-for-review conversion, branch deletion or history rewrite.

## 16. Source-successor validation requirements

Any later source materialization selected by KL must use exact-final-head validation only.

At minimum:

- locked dependency graph;
- rustfmt;
- Clippy;
- workspace tests;
- workspace build.

If Android validation is triggered for the exact final source head, its terminal result is also required for closure.

Skipped workflows are not PASS.

Any corrective source commit must be a normal descendant commit. No force-push/history rewrite is permitted.

## 17. Durable closure requirements for KL selection

This KL selection checkpoint may close only after:

1. exact branch head/tree re-read;
2. exact KK → KL topology proves one docs-only commit and no source drift;
3. draft/open/unmerged PR is created against exact KK base;
4. exact-final-head required CI is terminal-success;
5. exact-title Drive presearch is zero;
6. frozen raw Markdown audit is uploaded to the canonical checkpoint folder;
7. raw Drive readback verifies exact bytes and SHA-256;
8. exact-title postsearch returns exactly one canonical artifact;
9. branch and PR are re-read after publication;
10. only then may PR metadata record `SELECTION — VALIDATED — EVIDENCE_RECORDED — CLOSED`.

The PR itself remains draft, open and unmerged.

## 18. STOP boundary

KL selects only the additive dormant one-transaction accept/read wrapper.

STOP after KL selection closure.

Do not materialize the wrapper in this selection checkpoint.

Do not migrate EV/EX/FI/FQ/FU callers, introduce dual-authority propagation, populate production executable inputs, activate runtime/network behavior, merge, deploy, restart, recover, or mutate repository configuration without a separately selected successor gate.
