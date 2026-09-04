# Desktop Functional Management Slice C03e-KN — Production Durable Capability Repeated Post-Auth Ingress Loop Selection Staging

Status: `SELECTION_STAGING`
Date: `2026-09-04` (Europe/Tirane)

Gate:

`C03E_KN_PRODUCTION_DURABLE_CAPABILITY_REPEATED_POST_AUTH_INGRESS_LOOP_SELECTED`

Intended closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_REPEATED_POST_AUTH_INGRESS_LOOP_SELECTION`

## 1. Selection purpose

C03e-KN selects only one additive dormant repeated post-authenticated ingress loop after the closed C03e-KM one-transaction production-durable accept/read wrapper source materialization.

KM already provides exactly one dormant one-transaction durable wrapper:

`AuthenticatedRemoteSessionRuntimeOwner::process_one_post_auth_control_stream_ingress_with_production_durable_capability(...)`

That wrapper owns exactly one retained-peer control-stream accept, exactly one typed family-ingress read/classification, then delegates the exact typed ingress by value into the existing KK durable typed-ingress processor.

KN selects only repetition of that already-bounded one-transaction wrapper. KN does not select any FI/FQ/FU caller migration, requester DR authority change, dual-authority aggregate, cancellation worker, production executable population, runtime activation or merge.

This is a documentation-only selection checkpoint. It changes no Rust source, runtime ownership, manifests, lockfiles, workflows, Android source, services, databases, control plane, repository configuration, deployment state or network behavior.

## 2. Exact predecessor authority

Repository:

`Gersi365/prw-executor-private`

Repository ID:

`1334911207`

Default branch:

`main`

Observed repository visibility:

`public`

Integrated main remains:

`7c993fa93977a0bb84e0d030874eee7fd0cae77f`

Exact predecessor branch:

`phase-152-c03e-km-production-durable-capability-post-auth-accept-read-wrapper-source-materialization`

Exact KM head:

`6136e3f2f8b02ee3b692a2d30ee0338557f6a750`

Exact KM tree:

`3cc1937fd571c39f90401c29eab1eb4196b5210b`

Exact KM target source blob:

`abae1f811200238787e66eb102f1d77efa143336`

KM PR #424 remains draft, open, unmerged and evidence-closed.

## 3. Fresh source audit after KM

Exact KM target path:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`

The file contains both:

1. existing legacy repeated ingress loop:
   `run_repeated_post_auth_control_stream_ingress(...)`, using `SharedCurrentCapabilityAuthority<P>`;
2. new KM one-transaction durable wrapper:
   `process_one_post_auth_control_stream_ingress_with_production_durable_capability(...)`, using `ProductionDurableCapabilityAuthority`.

The legacy repeated loop currently samples `verifier_time_unix_seconds()` exactly once before each one-transaction invocation, continues only after `CapabilityProcessed`, and returns the first requester/rendezvous handoff or first typed ingress failure.

KN selects an additive durable analogue of that repetition behavior. It does not replace or mutate the legacy loop.

## 4. Higher caller audit and reason for additive selection

Exact KM FI path:

`crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`

Exact KM FI blob:

`bc0b9c49471d515b721c9cf47cd27ec3111f32ca`

FI uses `SharedCurrentCapabilityAuthority<P>` for two distinct roles:

- legacy capability ingress through the existing EX repeated loop;
- requester/rendezvous DR continuation authority.

Requester DR must continue to use the existing shared-current authority lane until separately selected. Replacing the FI authority parameter in place would therefore conflate capability authority with requester DR authority and would repeat the invalid migration shape discovered by KI.

KN consequently selects no FI signature change and no dual-authority composition.

The new durable repeated loop remains dormant and uninvoked by FI/FQ/FU until a later checkpoint separately selects how both authority lanes are composed.

## 5. Selected future method

The later source materialization may add exactly one dormant method to `AuthenticatedRemoteSessionRuntimeOwner`, conceptually:

`run_repeated_post_auth_control_stream_ingress_with_production_durable_capability(...)`

Selected generic inputs:

- `D: CapabilityDispatcher + Send`;
- `T: FnMut() -> u64 + Send`.

Selected arguments:

- `&mut self`;
- borrowed `&ProductionDurableCapabilityAuthority`;
- verifier-time callback `T`;
- mutable dispatcher `&mut D`.

Selected return:

`Result<RequesterRendezvousResponseStreamCustodyHandoff, AuthenticatedRemoteSessionPostAuthIngressTransactionError>`

No policy generic `P` is selected for this durable loop.

## 6. Exact selected loop order

For each serial iteration, the later source materialization must perform exactly:

1. sample `verifier_time_unix_seconds()` once;
2. call the existing KM one-transaction durable wrapper exactly once with that sampled value;
3. await the wrapper to completion;
4. on `AuthenticatedRemoteSessionPostAuthIngressOutcome::CapabilityProcessed`, begin the next iteration;
5. on `AuthenticatedRemoteSessionPostAuthIngressOutcome::RequesterRendezvous(handoff)`, return that exact handoff by value;
6. on any `AuthenticatedRemoteSessionPostAuthIngressTransactionError`, return the exact first error unchanged through `?` or equivalent lossless propagation.

No second transaction may overlap the current one.

## 7. Serial ownership invariants

The future durable repeated loop must preserve:

- exactly one in-flight post-auth transaction per iteration;
- no pre-accept of the next stream;
- no task spawn;
- no queue/channel;
- no concurrent family worker;
- no retry or reconnect;
- no second accept or read beyond what the KM wrapper itself owns;
- no peer-close policy change;
- no requester response work;
- no candidate provider work;
- no target dialing;
- no readiness/listener/runtime activation.

The loop itself never calls `accept_control_stream()` or `receive_post_auth_control_stream_ingress(...)` directly; it delegates those bounded actions to the existing KM wrapper.

## 8. Verifier-time proof obligation

Verifier time must be sampled exactly once immediately before each KM wrapper invocation.

The sampled value is the only verifier-time value supplied to that transaction.

No second clock sample may occur inside the repeated loop for the same transaction.

The KM wrapper itself continues to sample no time and accepts the caller-provided `now_unix_seconds` unchanged.

## 9. Capability-family proof obligation

For a capability transaction, the selected chain remains:

`durable repeated loop`
→ one KM wrapper invocation
→ one accept
→ one typed ingress read
→ exact capability transaction
→ KK typed-ingress processor
→ KG durable transaction helper
→ `ProductionDurableCapabilityAuthority`
→ dispatch
→ exact same transaction response stream.

The repeated loop itself performs no authorization, dispatch or response I/O.

No `SharedCurrentCapabilityAuthority<P>` fallback is allowed inside the new durable loop.

## 10. Requester/rendezvous proof obligation

For requester/rendezvous ingress, the exact KM wrapper/KK processor result must return one existing `RequesterRendezvousResponseStreamCustodyHandoff` and terminate the durable repeated loop immediately.

The loop must not perform requester DR, requester policy evaluation, requester registration, acknowledgement framing/write, candidate selection or dialing.

Those higher semantics remain outside this checkpoint.

## 11. Candidate-publication proof obligation

Candidate publication continues to fail closed through the existing nested path:

`AuthenticatedRemoteSessionPostAuthIngressTransactionError::CandidatePublicationHandoffNotSelected`

The durable repeated loop returns that first error unchanged and terminates.

No candidate response/provider semantics are selected.

## 12. Error preservation

The new durable repeated loop introduces no new error enum.

It must preserve the existing mixed-family error surface, including:

- `Accept(...)`;
- `Ingress(...)`;
- `ProductionDurableCapability(...)` with nested KG `Authority` / `Dispatch` / `Response` distinction;
- `CandidatePublicationHandoffNotSelected`.

No failure may be flattened, translated to fabricated success or converted to a retry condition.

## 13. Exact source-successor ceiling

The later KN source materialization may change at most one source path:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`

No second Rust path is selected.

The following remain byte-semantically unchanged unless a separate gate is opened:

- `crates/prw-agent/src/remote_session_capability_runtime.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/requester_rendezvous_retained_custody_dr_continuation.rs`;
- `crates/prw-agent/src/remote_session_capability_runtime/remote_session_executor_runtime.rs`;
- FQ/FU requester-aware runtime files;
- `linux_bootstrap.rs`;
- `main.rs`;
- manifests/lockfiles/workflows/Android source.

If compilation requires a second path, public API expansion, FI/FQ/FU change, production aggregate mutation or runtime activation, STOP and open a new gate.

## 14. Explicitly not selected

KN does not select:

- modification of the legacy EV one-transaction path;
- modification of the legacy EX repeated loop;
- a durable cancellation-aware EX worker;
- FI serial lifecycle migration;
- FI cancellation worker migration;
- FQ recoverable spawned worker migration;
- FU persistent real-admission migration;
- dual-authority FI/FQ/FU signatures;
- production aggregate population;
- executable durable-authority population;
- requester DR authority replacement;
- positive capability grants;
- runtime/network/listener/readiness activation;
- deployment/restart/recovery;
- merge/PR close/ready conversion;
- branch deletion/history rewrite;
- repository configuration mutation.

## 15. Successor proof obligations

A source successor is valid only if it proves all of the following:

1. additive method only;
2. exactly one verifier-time sample per iteration;
3. exactly one KM wrapper invocation per iteration;
4. serial transaction completion before the next iteration;
5. capability success alone loops;
6. requester handoff terminates and returns exact custody;
7. first typed error terminates unchanged;
8. no legacy shared-current capability fallback;
9. no requester DR work in the new loop;
10. no candidate semantics expansion;
11. no cancellation semantics added;
12. no higher caller integration;
13. no source path beyond the one selected file;
14. dormant/uninvoked by production executable source.

## 16. Validation and closure rule

KN itself is selection-only.

Close KN only after:

- exact-final-head required CI is terminal-success;
- skipped workflows are recorded as skipped, never PASS;
- immutable raw Markdown audit is published to canonical Drive folder `1jhitnxc9vqtTXQQTG_OB_Kw2pDeYIKhT`;
- Drive raw readback matches frozen local bytes and SHA-256 exactly;
- exact-title post-upload search yields exactly one canonical artifact;
- branch/PR/topology post-publication guards remain unchanged;
- PR metadata is updated only after durable evidence verification.

Keep the KN PR draft, open and unmerged.

## 17. STOP boundary

STOP after KN selection closure.

Do not materialize the durable repeated loop in KN. Do not integrate FI/FQ/FU, add dual-authority composition, populate production inputs, activate runtime/network behavior, merge or deploy without a separately selected successor checkpoint.
