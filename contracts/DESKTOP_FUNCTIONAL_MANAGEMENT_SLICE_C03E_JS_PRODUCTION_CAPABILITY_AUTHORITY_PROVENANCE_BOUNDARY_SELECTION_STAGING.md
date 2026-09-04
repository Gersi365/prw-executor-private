# Desktop Functional Management Slice — C03e-JS Production Capability-Authority Provenance Boundary Selection

Status: `SELECTION_STAGING`
Date: `2026-09-04`

## 1. Checkpoint classification

C03e-JS is a documentation-only provenance-gap selection checkpoint.

Target gate:

`C03E_JS_PRODUCTION_CAPABILITY_AUTHORITY_PROVENANCE_BOUNDARY_SELECTED`

Target closure:

`CLOSED_PRODUCTION_CAPABILITY_AUTHORITY_PROVENANCE_SELECTION`

C03e-JS selects only the next still-missing production input responsibility after the closed C03e-JR composition checkpoint: authoritative production provenance for the existing `SharedCurrentCapabilityAuthority<P>` input consumed by the remote-process population lane.

JS does not select a concrete registry provider, policy provider, snapshot/adapter mechanism, synchronization strategy, generic policy type, environment variable, systemd credential, database schema, control-plane service, runtime caller, or executable activation path.

## 2. Exact predecessor authority

Predecessor checkpoint:

`C03e-JR — Production reachability remote-process input population composition source materialization`

Predecessor branch:

`phase-152-c03e-jr-production-reachability-remote-process-input-population-composition-source-materialization`

Exact predecessor head / merge base:

`45a80a9a301121897031dedb800596cac8c3ac47`

Exact predecessor tree:

`8b48669243115fdba13efe711ea6ed5aa36023a7`

Exact predecessor `crates/prw-agent/src/linux_bootstrap.rs` blob:

`f2a87c45bd8d96bf1555b65210531c94c722eb2f`

Predecessor gate:

`C03E_JR_PRODUCTION_REACHABILITY_REMOTE_PROCESS_INPUT_POPULATION_COMPOSITION_SOURCE_MATERIALIZED`

Predecessor closure:

`CLOSED_PRODUCTION_REACHABILITY_REMOTE_PROCESS_INPUT_POPULATION_COMPOSITION_SOURCE_MATERIALIZATION`

JR now composes the already-selected worker-limit/bind population and peer population into one `LinuxAgentProductionReachabilityRemoteProcessOperationInputs` owner while all capability/session/request/timing/callback inputs remain caller-provided typed values.

## 3. Why capability authority is the next provenance boundary

The existing production input lane ultimately consumes:

`SharedCurrentCapabilityAuthority<P>`

before one remote admission worker can perform current registry plus capability-policy authorization.

The earlier closed C03e-BG production-input provenance ordering explicitly required the following responsibility before expected-request assembly:

1. authoritative current registry + policy source capable of producing `SharedCurrentCapabilityAuthority`;
2. typed capability-provider/dispatcher custody;
3. `SessionId` production;
4. authentication request-id production;
5. verifier-owned timing sources;
6. authoritative pre-handshake expected-`DeviceId` scheduling provenance;
7. worker capacity and callbacks;
8. only then full expected-request and remote-process input composition.

Subsequent checkpoints have now materialized production bind-address provenance, durable-registry provider/custody and current peer lookup, peer logical-device provenance, worker-limit provenance, and the worker-limit/bind/peer population composition.

No closed checkpoint has materialized production provenance for `SharedCurrentCapabilityAuthority<P>`.

JS therefore selects this authority input as the next missing responsibility and does not skip ahead to dispatcher, session/request identifier, timing, expected-request, requester/rendezvous, operation-factory, executable or runtime activation work.

## 4. Exact current `SharedCurrentCapabilityAuthority` shape

Exact JR source:

`crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`

Exact blob:

`60307fff4dd0fd573192ba6e6fab9dedd3321dda`

The existing owner is:

`SharedCurrentCapabilityAuthority<P>`

and its constructor remains:

```text
SharedCurrentCapabilityAuthority::new(
    registry: WorkspaceDeviceRegistry,
    policy: P,
)
```

The owner stores one combined Tokio `RwLock` state containing exactly:

- one `WorkspaceDeviceRegistry`;
- one policy evaluator `P`.

Clones share the outer `Arc`; they do not create independent per-worker authority snapshots.

Existing read operations retain coherent current registry/policy custody for the selected lexical operation and do not expose lock guards.

Construction itself performs no production source acquisition, registry population, policy load, provider I/O, synchronization, network I/O, task spawn, readiness publication or runtime activation.

## 5. `WorkspaceDeviceRegistry::new()` is not production authority

Exact JR registry source:

`crates/prw-registry/src/lib.rs`

Exact blob:

`98efd22858960cd441237049a5578a78eecc13ab`

`WorkspaceDeviceRegistry` remains the bounded in-memory Phase 130 registry authority.

`WorkspaceDeviceRegistry::new()` creates an empty registry.

An empty/default registry does not prove current production membership, enrolled-device state, public identity, transport identity, revocation state or any other current registry fact.

JS explicitly rejects satisfying `SharedCurrentCapabilityAuthority<P>` by constructing an empty/default `WorkspaceDeviceRegistry` and treating that value as successful production registry provenance.

No implicit registry enumeration, synthetic membership/device insertion, fixture reuse or test-state promotion is selected.

## 6. Durable registry is authoritative but not an implicit `WorkspaceDeviceRegistry` source

Exact JR durable registry source:

`crates/prw-registry/src/durable_registry_etcd_store.rs`

Exact blob:

`1e04b366471fe2d4433de3c383efb4108d828983`

The existing `DurableRegistryEtcdStore` provides semantic, provider-backed current operations including exact membership/device reads, Phase 130 mutations and `validate_authenticated_session(...)` using authoritative linearizable reads.

That durable adapter explicitly does not own:

- production registry population;
- Agent composition;
- provider bootstrap details;
- retry/reconciliation;
- Watch/lease/TTL;
- runtime activation;
- networking or deployment.

Most importantly for JS, exact JR source contains no reviewed conversion, snapshot loader or synchronization bridge that turns the durable provider-backed registry into the complete in-memory `WorkspaceDeviceRegistry` required by `SharedCurrentCapabilityAuthority<P>`.

JS therefore rejects all of the following implicit shortcuts:

- treating `DurableRegistryEtcdStore` as type-equivalent to `WorkspaceDeviceRegistry`;
- taking a partial durable read and calling it a complete current in-memory registry;
- silently scanning or enumerating durable keys to synthesize a full registry snapshot;
- introducing an unreviewed watch/cache mirror;
- copying durable records into an in-memory authority without explicit currentness and lifecycle semantics;
- replacing the current shared-authority interface as an incidental implementation detail.

Any such mechanism requires a separately reviewed concrete-source/interface selection.

## 7. Policy provenance is independently unresolved

Exact JR policy source:

`crates/prw-policy/src/lib.rs`

Exact blob:

`3745024b5b222fcb36244222fad3c9c05a59cece`

The policy crate currently exposes the principal-agnostic `PolicyEvaluator` trait and bounded local configuration primitives such as:

- `BoundedLocalReadPolicy`;
- `BoundedLocalManagementPolicy`.

Those types are configuration primitives for reviewed local surfaces. They do not constitute a production remote capability-policy source.

`BoundedLocalReadPolicy::allow_local_reads()` is not selected as a remote production policy.

`BoundedLocalReadPolicy::deny_all()` and `BoundedLocalManagementPolicy::deny_all()` may be valid explicit fail-closed values for their intended local/testing surfaces, but JS does not reinterpret convenient construction as production policy provenance.

The current policy module has no `allow_all` constructor and JS does not add one.

JS selects no policy persistence, role-to-capability mapping, workspace-specific policy source, account/role authority, dynamic reload, control-plane service, file format, environment representation or systemd credential.

## 8. Current authorization semantics remain unchanged

The existing remote capability path continues to require authenticated session ownership plus fresh current registry/transport/policy validation before dispatcher invocation.

JS does not weaken, bypass or duplicate that chain.

In particular:

- successful durable-registry lookup is not capability authorization;
- successful peer lookup is not requester authorization;
- possession of `SharedCurrentCapabilityAuthority<P>` is not authentication;
- a policy decision without current authenticated/session registry binding is insufficient;
- current registry validity without an explicit capability allow is insufficient.

## 9. Selected provenance responsibility

JS selects exactly this missing responsibility:

```text
authoritative production registry provenance
    +
authoritative production capability-policy provenance
    -> reviewed construction/custody mechanism
    -> SharedCurrentCapabilityAuthority<P>
```

The selected responsibility must preserve one coherent current registry/policy authority for every protected remote capability operation.

JS does not select how that responsibility is implemented.

## 10. Concrete mechanism remains unselected

A successor must perform a fresh exact-head audit before selecting any concrete mechanism.

JS does not choose between possibilities such as:

- a fully populated `WorkspaceDeviceRegistry` loaded from an authoritative source;
- a provider-backed redesign/adaptation of the shared-current authority owner;
- a bounded snapshot with explicit currentness semantics;
- a synchronized mirror/watch model;
- a different reviewed policy evaluator type;
- a dedicated policy provider or control-plane source.

Listing these possibilities does not authorize any of them.

If the current `SharedCurrentCapabilityAuthority<P>` type cannot represent production currentness without broader interface changes, the successor must stop at selection and explicitly review that interface boundary rather than forcing a lossy adapter.

## 11. No source-materialization successor is directly authorized by JS

Unlike a checkpoint that has already selected one exact file/function/source mechanism, JS selects only the provenance boundary.

Therefore JS closure does **not** authorize immediate Rust/source materialization.

The next checkpoint may only select the concrete production capability-authority source/interface mechanism, including:

- exact owner/type boundary;
- exact registry source/currentness semantics;
- exact policy source/evaluator type;
- exact failure classification;
- exact path ceiling;
- whether any new provider/custody object is required.

Only after that concrete selection closes may a later checkpoint materialize source.

## 12. Session authentication remains separately gated

`SessionAuthenticationService::new()` constructs the existing in-memory authentication transaction owner, but constructor availability is not full remote production input assembly.

JS does not change or source `SessionAuthenticationService`.

It does not select session persistence, session recovery, identifier generation, external session provider state, startup population or process-exit semantics.

Session authentication input composition remains separately gated after capability-authority provenance.

## 13. Expected-request production remains separately gated

JS does not construct or populate:

`mpsc::Receiver<RemoteSessionExpectedDeviceAdmissionRequest<D, T>>`

and does not select a producer for `RemoteSessionExpectedDeviceAdmissionRequest<D, T>`.

The existing expected request still owns expected logical `DeviceId`, `SessionId`, authentication correlation request ID, dispatcher and verifier-time provider.

No one of those values may be fabricated merely to satisfy the request type.

Pre-handshake expected-device scheduling provenance remains separately gated.

## 14. Dispatcher, timing and callbacks remain separately gated

JS does not select:

- a concrete production `CapabilityDispatcher`;
- terminal/files/forwarding production provider construction;
- verifier-time provider;
- admission timing provider;
- completion callback semantics;
- rejection callback semantics;
- repeated-admission-failure callback semantics;
- logging/metrics/retry/replacement/process-exit consequences.

These remain independent typed responsibilities.

## 15. Requester/rendezvous custody remains separately gated

JS does not construct or populate:

- `BoundedRequesterRendezvousStartPolicySource`;
- `CandidatePublicationRequesterRendezvousRuntimeOwner`;
- `LinuxAgentProductionReachabilityRequesterRendezvousRemoteProcessOperationInputs`.

Existing requester/rendezvous policy/runtime types do not solve the remote capability-authority provenance selected by JS.

No requester/rendezvous provider is invoked.

## 16. Identity and authority invariant

C03e-JS preserves:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

Specifically:

- `DeviceId` and authenticated PRW session identity remain logical identity;
- `TransportIdentity` remains transport identity/current transport evidence;
- `SocketAddr`/IP remain transient reachability data;
- peer logical-device configuration remains peer intent only;
- worker-limit remains scheduling configuration only;
- `SessionId` remains authentication/session correlation and ownership context, not device identity;
- PRWM `request_id` remains transaction correlation only;
- policy evaluation does not create authenticated identity;
- registry lookup does not by itself grant capabilities.

No PID/UID/GID or host account identity becomes PRW logical identity.

## 17. Security exclusions

JS does not perform or authorize:

- Rust/source materialization;
- public API widening;
- `SharedCurrentCapabilityAuthority` interface changes;
- durable registry scans/watch/cache/mirror creation;
- registry data population or mutation;
- policy persistence/load/mutation;
- default/empty registry promotion;
- fail-open or synthetic policy promotion;
- role-to-capability policy invention;
- session authentication production population;
- expected-request production;
- dispatcher/provider production assembly;
- timing/callback production sourcing;
- requester/rendezvous custody population/invocation;
- operation-factory invocation;
- remote-process companion spawn;
- `run()` or `main.rs` mutation;
- listener/bind/readiness/runtime/network activation;
- candidate publication, traversal, dialing, retry, reconnect, rebind or rebootstrap;
- service/systemd/package/security/credential/certificate/private-key/trust/RBAC mutation;
- database/schema/control-plane mutation;
- deployment, restart or recovery activation;
- repository visibility/configuration mutation;
- merge, PR close, ready-for-review conversion, branch deletion or history rewrite.

## 18. Exact intended JR -> JS scope

JS is documentation-only.

The exact branch must differ from closed JR only by:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_JS_PRODUCTION_CAPABILITY_AUTHORITY_PROVENANCE_BOUNDARY_SELECTION_STAGING.md`

Any Rust/source, manifest, lockfile, workflow, Android, packaging, service/systemd, registry, policy, auth, provider, runtime or `main.rs` change blocks closure.

## 19. Validation semantics

Exact-final-head CI must be tied to the exact final JS head before closure.

Path-filtered workflows reporting `SKIPPED` remain `SKIPPED` and must not be represented as PASS.

A successful docs-only JS validation does not validate any future concrete authority source/interface selection or source implementation.

## 20. Successor rule

After JS closure: **STOP**.

The next checkpoint may only select the concrete production capability-authority source/interface mechanism for the already-selected `SharedCurrentCapabilityAuthority<P>` provenance responsibility.

It must not skip directly to source implementation unless that successor first proves and freezes an exact mechanism, error surface and path ceiling.

After concrete mechanism selection closes, stop again before source materialization.

No session/expected-request/dispatcher/timing/callback/requester-rendezvous/aggregate/executable/runtime boundary is inherited from JS.

## 21. Closure requirements

C03e-JS may close only after all of the following are verified:

1. exact JS branch head is re-read;
2. exact JR -> JS topology has merge base equal to JR head `45a80a9a301121897031dedb800596cac8c3ac47`;
3. aggregate changed-file set contains exactly the single JS contract path;
4. exact-final-head CI reaches terminal expected conclusions;
5. immutable Drive audit evidence is published and raw-readback verified;
6. exact-title Drive uniqueness is verified under the canonical audit parent;
7. the JS PR remains draft/open/unmerged at the exact audited head.

After closure the branch/PR remain staging evidence only and no merge, deployment or runtime activation is implied.
