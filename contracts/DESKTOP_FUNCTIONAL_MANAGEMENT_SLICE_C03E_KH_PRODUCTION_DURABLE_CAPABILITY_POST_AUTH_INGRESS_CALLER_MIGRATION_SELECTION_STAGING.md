# Desktop Functional Management Slice C03e-KH — Production Durable Capability Post-Auth Ingress Caller Migration Selection

Status: `SELECTION_STAGING`

Gate:

`C03E_KH_PRODUCTION_DURABLE_CAPABILITY_POST_AUTH_INGRESS_CALLER_MIGRATION_SELECTED`

Intended closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_POST_AUTH_INGRESS_CALLER_MIGRATION_SELECTION`

## 1. Purpose

C03e-KH selects the next source-materialization boundary after the closed C03e-KG production durable capability post-auth transaction composition.

C03e-KH is documentation-only. It does not modify Rust source, migrate an executable caller, populate production runtime inputs, replace the production process aggregate, activate a listener or network path, change capability policy, deploy a service, mutate repository configuration, merge a pull request, or rewrite history.

The selected successor is limited to the existing dormant Agent-owned mixed-family post-auth ingress call chain that already owns one typed `PostAuthCapabilityTransaction` after exactly one stream acceptance and exactly one family ingress read.

The purpose is to select how that dormant capability-family call chain will use the already-materialized C03e-KG durable capability transaction helper while preserving requester/rendezvous and candidate-publication semantics exactly.

## 2. Authority hierarchy

The selection authority for this checkpoint is, in order:

1. exact GitHub branch/head/tree/file state;
2. exact source at the C03e-KG final head;
3. exact-final-head GitHub Actions evidence for this documentation-only selection;
4. immutable Google Drive readback evidence after validation.

Older handoff material is non-authoritative where it disagrees with current GitHub state.

## 3. Exact predecessor checkpoint

Predecessor:

`C03e-KG — Production durable capability post-auth transaction composition source materialization`

Exact predecessor branch:

`phase-152-c03e-kg-production-durable-capability-post-auth-transaction-composition-source-materialization`

Exact predecessor head:

`7a32394db0f7377dfb1242dd95f36ac8aab3ff75`

Exact predecessor tree:

`899ec8441ff266535998d8a28bf308d13f4a3f11`

Exact KG materialized target path:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`

Exact KG target blob:

`f95c4f1bb2d424ea7d15647ecb1d6153aebc480c`

KG PR:

`#418 — C03e-KG: materialize durable capability post-auth transaction composition`

Required predecessor state at KH creation:

- open;
- draft;
- unmerged;
- status recorded as `SOURCE MATERIALIZATION — VALIDATED — EVIDENCE_RECORDED — CLOSED`;
- head SHA exactly equal to the KG final head above.

C03e-KH must not modify KG history or PR state.

## 4. C03e-KG capability transaction seam already available

At the exact KG head, `AuthenticatedRemoteSessionRuntimeOwner` already owns the dormant Agent-internal operation:

`process_production_durable_capability_transaction(...)`

Its selected and materialized semantic order is:

1. consume one already-read `PostAuthCapabilityTransaction`;
2. borrow the exact retained `BoundRemoteSession`;
3. derive presented transport evidence only through `BoundRemoteSession::transport_identity()`;
4. borrow the existing authenticated lease only through `BoundRemoteSession::lease()`;
5. use caller/verifier-provided `now_unix_seconds`;
6. call `ProductionDurableCapabilityAuthority::authorize_capability_transaction(...)` exactly once;
7. call `dispatch_authorized_request(...)` only after durable authorization succeeds;
8. consume the same retained transaction through `send_response_frame(...)` only after dispatch succeeds;
9. return success only after same-stream response I/O succeeds.

The helper accepts no stream and performs no second frame read. It adds no outer durable-registry lock. The existing durable-authority operation releases durable registry mutex custody before it returns, therefore dispatcher execution and response I/O occur outside durable-registry lock custody.

The helper remains dormant and is not itself production runtime activation.

## 5. Exact bounded KG failure surface

C03e-KG already materialized:

`ProductionDurableCapabilityTransactionError`

with exactly these semantic stages:

- `Authority(DurableCapabilityBridgeError)`;
- `Dispatch(RemoteBridgeError)`;
- `Response(CapabilityRequestWireError)`.

The stage error remains available through `std::error::Error::source()`.

A later caller migration must preserve this typed stage distinction. It must not collapse durable authority failure into legacy `RemoteBridgeError`, flatten response failure into a generic ingress failure, suppress the original source, fabricate success, retry, or fall back to the legacy in-memory/shared-current path.

## 6. Exact current mixed-family post-auth caller

Exact source path:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`

Exact KG blob:

`294b64ba33ba8a4b1d4ed595bf473f6d225ec0d5`

The existing dormant method is:

`AuthenticatedRemoteSessionRuntimeOwner::process_one_post_auth_control_stream_ingress(...)`

Its current transaction boundary is already correct for family ingress:

1. accept exactly one control stream from the retained authenticated peer;
2. transfer that stream by value into `receive_post_auth_control_stream_ingress(...)`;
3. perform exactly one bounded frame read and family classification;
4. match one of capability, requester/rendezvous, or candidate-publication ingress.

C03e-KH does not select any change to this one-accept/one-read family ingress boundary.

## 7. Exact current capability-family gap

At the exact KG head, the capability arm inside `process_one_post_auth_control_stream_ingress(...)` still reconstructs the legacy shared-current authorization path inline.

Current capability-family semantic path:

`PostAuthControlStreamIngress::Capability(transaction)`
→ retained `BoundRemoteSession`
→ `SharedCurrentCapabilityAuthority<P>::with_current_authority(...)`
→ temporary `CapabilityBridge::new(registry, policy)`
→ `BoundRemoteSession::authorize(...)`
→ `dispatch_authorized_request(...)`
→ same `transaction.send_response_frame(...)`.

This path duplicates the capability transaction composition that C03e-KG has now materialized for production durable authority.

The mixed-family ingress method is therefore the first concrete dormant caller seam where C03e-KG can replace the legacy capability-family composition without accepting a new stream, reading a second frame, inventing a new identity source, or touching executable production assembly.

## 8. Requester/rendezvous family must remain unchanged

The existing requester/rendezvous arm currently:

1. receives the already-classified `PostAuthRequesterRendezvousTransaction`;
2. reads the nominated target only from the strict already-read request;
3. adapts that logical target through the existing target-intent helper;
4. derives requester identity only from the retained authenticated application session;
5. returns exact same-stream response custody in `RequesterRendezvousResponseStreamCustodyHandoff`.

It stops before requester policy/provider execution, candidate selection, requester response construction/write, target dialing, retry, close policy, readiness, or runtime activation.

The later KH source successor must leave these semantics unchanged.

No durable capability authority is requester/rendezvous authority.

No capability authorization result may be reused as requester/rendezvous authorization.

## 9. Candidate-publication family must remain fail-closed

The current candidate-publication arm returns:

`AuthenticatedRemoteSessionPostAuthIngressTransactionError::CandidatePublicationHandoffNotSelected`

No Agent candidate-publication handoff/execution semantics are selected by KH.

The later source successor must preserve the same explicit fail-closed barrier. It must not reinterpret candidate publication as capability traffic, requester/rendezvous traffic, success, or an alternate provider action.

## 10. Existing repeated dormant call chain

The same exact source file contains:

`run_repeated_post_auth_control_stream_ingress(...)`

and:

`run_repeated_post_auth_control_stream_ingress_worker(...)`.

Both are already dormant and both delegate to the one-transaction mixed-family method above.

The repeated loop:

- samples verifier time exactly once immediately before each one-transaction invocation;
- permits only capability success to continue to another iteration;
- returns the first requester/rendezvous handoff barrier;
- returns the first transaction failure unchanged.

The executor-neutral worker:

- races exactly one repeated-loop future against caller-owned cancellation;
- polls the ingress loop first;
- creates no task itself;
- does not close the peer on cancellation;
- does not activate a runtime or listener.

Because both method signatures carry the same legacy `SharedCurrentCapabilityAuthority<P>` parameter solely to call the one-transaction method, changing the one-transaction capability authority is compiler-coupled to this local dormant call chain.

KH therefore selects only the minimum signature propagation needed inside this existing dormant call chain. This is not production executable caller population or activation.

## 11. Existing parent post-auth error surface

Exact source path:

`crates/prw-agent/src/remote_session_capability_runtime.rs`

Exact KG blob:

`fbeb7016209d02d348e1b04ac4160a1c0895badc`

The current `AuthenticatedRemoteSessionPostAuthIngressTransactionError` retains legacy capability-specific variants:

- `Accept(RemoteServerTransportRuntimeError)`;
- `Ingress(PostAuthControlStreamIngressError)`;
- `Bridge(RemoteBridgeError)`;
- `CapabilityResponse(CapabilityRequestWireError)`;
- `CandidatePublicationHandoffNotSelected`.

Once the dormant capability arm delegates to the KG durable helper, a durable authority failure cannot be represented faithfully by the legacy `Bridge(RemoteBridgeError)` variant.

KH therefore selects bounded error adaptation rather than lossy translation.

## 12. Selected caller-migration boundary

The later source successor is selected to migrate only the existing dormant mixed-family post-auth capability lane and its compiler-coupled local repeated call chain.

Selected semantic order for one capability-family transaction:

`PostAuthControlStreamIngress::Capability(transaction)`
→ existing `AuthenticatedRemoteSessionRuntimeOwner::process_production_durable_capability_transaction(...)`
→ `AuthenticatedRemoteSessionPostAuthIngressOutcome::CapabilityProcessed` only on full durable authorization + dispatch + same-stream response success.

The caller must pass the already-existing verifier time and mutable dispatcher unchanged.

The transaction must be moved directly into the KG helper. It must not call `request_frame()` to reconstruct a second authorization path, accept another stream, perform another ingress read, copy transport identity from caller input, or build a temporary legacy `CapabilityBridge`.

## 13. Selected durable authority parameter

The dormant mixed-family one-transaction method, its repeated loop, and its executor-neutral worker are selected to receive an explicit borrowed:

`&ProductionDurableCapabilityAuthority`

in place of the legacy:

`&SharedCurrentCapabilityAuthority<P>`

for the capability-family lane.

The later source successor must not:

- bootstrap durable authority internally;
- read credentials or environment variables inside these methods;
- create a second durable authority owner;
- construct a synthetic in-memory registry;
- adapt durable authority into `SharedCurrentCapabilityAuthority<P>`;
- retain both authorities as fallback alternatives;
- select one authority dynamically.

Production population of this durable authority into an executable aggregate is explicitly outside KH.

## 14. Selected outer error adaptation

The later source successor may revise only the mixed-family ingress error surface required to carry the existing KG error without losing stage provenance.

The selected capability failure representation is one bounded outer variant containing:

`ProductionDurableCapabilityTransactionError`

The exact variant spelling may follow local rustfmt/Clippy conventions, but its semantics must be unambiguous: it represents only the KG durable capability transaction stages.

The outer error must continue to preserve:

- stream acceptance failure;
- typed family ingress failure;
- durable capability transaction failure with the complete nested `Authority` / `Dispatch` / `Response` classification;
- the explicit candidate-publication-handoff-not-selected barrier.

`std::error::Error::source()` must preserve the nested durable capability transaction error, whose own `source()` preserves the exact underlying stage error.

The obsolete legacy-only `Bridge(RemoteBridgeError)` and direct capability-response variant may be removed if they have no remaining use after this exact caller migration. No unrelated error surface may be changed.

## 15. Exact source-successor path ceiling

The later source-materialization checkpoint may change at most these two paths:

1. `crates/prw-agent/src/remote_session_capability_runtime.rs`
2. `crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime/requester_rendezvous_one_shot_transaction.rs`

No change is selected for the C03e-KG helper file:

`crates/prw-agent/src/remote_session_capability_runtime/authenticated_remote_session_runtime.rs`

Its final KG blob remains the source authority for the durable helper contract unless a fresh gate explicitly authorizes otherwise.

The successor must STOP if correct compilation requires a third repository path, manifest/lockfile mutation, dependency change, public API expansion, production aggregate mutation, bootstrap/provider mutation, or executable/runtime activation.

## 16. Permitted source-successor changes

Within the two-path ceiling, the later materialization may perform only:

1. imports required to reference `ProductionDurableCapabilityAuthority` and the existing KG transaction error;
2. removal of imports made obsolete solely by replacement of the legacy inline capability path;
3. bounded outer post-auth ingress error adaptation preserving the complete nested KG stage error;
4. minimal `Display`, `Error::source`, and exact `From` plumbing for that bounded error adaptation;
5. replacement of the EV capability arm's inline `SharedCurrentCapabilityAuthority<P>`/`CapabilityBridge` authorization-dispatch-response sequence with exactly one call to the KG helper;
6. replacement of the one-transaction method's capability-authority parameter with borrowed `ProductionDurableCapabilityAuthority`;
7. compiler-coupled propagation of that same authority parameter through the existing dormant repeated ingress loop;
8. compiler-coupled propagation of that same authority parameter through the existing dormant executor-neutral worker;
9. removal of the now-unneeded policy generic bounds from only those local methods if no longer semantically used;
10. focused same-file or parent-file shape/error tests required to prove the selected migration;
11. only local rustfmt/Clippy accommodation directly required by the selected source shape.

No other semantic change is selected.

## 17. Required proof obligations for materialization

The later source materialization must make the following mechanically reviewable:

1. capability-family ingress still accepts no additional stream;
2. capability-family ingress performs no second frame read;
3. the exact `PostAuthCapabilityTransaction` produced by family ingress is consumed by the KG helper;
4. transport identity is sourced only from the owner's retained `BoundRemoteSession` inside the KG helper;
5. lease is sourced only from the same retained bound session;
6. durable authority authorization occurs before dispatch;
7. no durable-registry mutex spans dispatch or response I/O;
8. same-stream response occurs only after successful dispatch;
9. all KG `Authority` / `Dispatch` / `Response` failures remain distinguishable through the typed error chain;
10. no legacy shared-current fallback remains in the migrated mixed-family capability arm;
11. requester/rendezvous success semantics are unchanged;
12. candidate publication remains explicit fail-closed;
13. repeated ingress remains serial with one transaction in flight;
14. verifier time remains sampled once immediately before each one-transaction invocation;
15. executor-neutral cancellation behavior remains unchanged;
16. the mixed-family chain remains dormant and uninvoked by production executable source.

## 18. Identity and authority invariants

KH preserves the project identity hierarchy:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`.

Transport identity is not logical device identity.

Endpoint addressing is not logical device identity.

PRWM `request_id` remains correlation only and is not requester identity, target identity, session identity, transport identity, authorization evidence, or registry authority.

Successful stream acceptance is not authorization.

Successful family ingress is not authorization.

Successful durable registry lookup or decode is not authorization.

Only successful durable capability authorization yields the `AuthorizedCapabilityRequest` consumed by capability dispatch.

Requester/rendezvous authorization remains a distinct authority lane.

Candidate-publication authority remains a distinct authority lane.

## 19. Production policy remains unchanged

KH selects no positive production capability policy.

The existing production durable capability authority continues to retain the previously materialized fail-closed production policy boundary.

No permit-bearing configuration source, policy refresh, policy watch, registry scan, cache mirror, alternate evaluator, fallback evaluator, or degraded authorization mode is selected.

## 20. Production aggregate explicitly excluded

At the exact KG head, `crates/prw-agent/src/linux_bootstrap.rs` still carries:

`capability_authority: SharedCurrentCapabilityAuthority<P>`

inside `LinuxAgentRemoteProcessOperationInputs<P, ...>`.

KH does not replace or extend that aggregate.

KH does not populate `ProductionDurableCapabilityAuthority` into Linux process inputs.

KH does not modify session authentication, expected-request production population, verifier-time production population, dispatcher production population, callback population, worker-limit population, peer population, bind-address population, endpoint lifecycle, supervisor lifecycle, or process lifecycle.

Those are later executable integration concerns requiring separate exact-head selection gates.

## 21. Historical capability-only path remains separate

The existing historical capability-only methods in `authenticated_remote_session_runtime.rs`, including the older `process_one_capability_request(...)` path and its capability-only loop/worker lineage, are not selected for migration by KH.

KH does not delete, redirect, invoke, or activate those paths.

The KH source successor is limited to the already-materialized mixed-family EV/EX dormant call chain because that chain already owns the typed post-auth family transaction required by the KG helper.

Any later cleanup or removal of historical legacy paths requires a separate gate.

## 22. Explicit exclusions

C03e-KH does not perform or authorize:

- Rust/source materialization;
- modification of the KG helper implementation;
- third-path source changes;
- replacement of `LinuxAgentRemoteProcessOperationInputs<P, ...>`;
- executable production durable-authority population;
- session-authentication production population;
- verifier-time production population;
- dispatcher production population;
- expected-request producer migration;
- callback migration;
- activation of the mixed-family repeated loop or worker;
- activation of requester/rendezvous provider execution;
- requester response construction/write;
- candidate-publication handoff or execution;
- candidate response construction/write;
- target dialing;
- positive production capability grants;
- durable registry scans, watches, cache mirrors or background refresh;
- provider/credential/RBAC/systemd/service/package mutation;
- listener bind/readiness/endpoint lifecycle activation;
- new task/channel/queue creation;
- retry/reconnect/fallback/degraded authority behavior;
- startup or process-exit policy change;
- `run()` or `main.rs` wiring;
- deployment, restart or recovery activation;
- database/schema/control-plane mutation;
- manifest or lockfile mutation;
- workflow mutation;
- Android source mutation;
- repository visibility/configuration mutation;
- merge;
- ready-for-review conversion;
- PR close;
- branch deletion;
- force-push or history rewrite.

## 23. Selection closure rule

C03e-KH may be recorded closed only when all of the following are true at one exact final KH head:

1. KH branch descends directly from exact KG head `7a32394db0f7377dfb1242dd95f36ac8aab3ff75`;
2. KH contains only this documentation contract path;
3. KG branch and PR #418 remain unchanged, draft/open/unmerged and closed by evidence;
4. exact-final-head required CI is terminal-success for the documentation-only KH change;
5. skipped/path-filtered workflows are recorded as skipped, not PASS;
6. exact-title Drive pre-publication search returned zero matches;
7. one canonical raw Markdown audit is uploaded to the established Phase 152 audit folder;
8. raw Drive readback byte count and SHA-256 match the frozen local audit candidate exactly;
9. exact-title post-publication search returns exactly one canonical KH audit artifact;
10. KH branch and draft PR are re-read unchanged after evidence publication;
11. KH PR closure metadata records exact head/tree/contract blob, CI evidence and Drive evidence while the PR remains draft/open/unmerged.

After durable KH selection closure: **STOP**.

The next checkpoint may materialize only the two-path dormant caller migration selected here. Any executable aggregate population, runtime activation, legacy-path removal, positive production policy, requester/candidate execution, listener/readiness work, or wider integration requires another fresh exact-head gate.
