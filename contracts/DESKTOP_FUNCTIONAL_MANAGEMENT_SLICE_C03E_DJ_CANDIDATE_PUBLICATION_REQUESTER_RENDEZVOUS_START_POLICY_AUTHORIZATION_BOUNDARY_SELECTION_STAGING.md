# Phase 152 C03e-DJ — Candidate Publication Requester/Rendezvous Start Policy Authorization Boundary Selection — STAGING

## Status

`STAGED SELECTION`

## Target gate

`C03E_DJ_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_START_POLICY_AUTHORIZATION_BOUNDARY_SELECTED`

## Exact predecessor

C03e-DJ is rooted only at durably closed C03e-DI:

- branch: `phase-152-c03e-di-candidate-publication-requester-rendezvous-post-registry-validation-provenance-carrier-source-materialization-staging`
- head: `04bf3a7a57d7a804e923a1a8592b0e5aacfc9be6`
- tree: `a647119cceaa345ee137d7b91c8aef31ec9fd644`
- PR #232: `Status: CLOSED`, draft/open/unmerged
- rolling Drive image: `987535` bytes
- rolling SHA-256: `ff607b6067b940d4e74fc2c0c04f49f9d7f96b54e10fea8cfa3668da437347e5`

No earlier checkpoint is reopened.

## Checkpoint classification

C03e-DJ is a documentation-only policy-authorization boundary selection checkpoint.

It selects only the minimum semantic policy gate that must exist after successful C03e-DI current-registry provenance validation and before any requester/rendezvous provider registration mutation may be considered.

DJ does not modify `prw-policy`, Agent source, provider source, wire protocol, runtime topology, networking, deployment, or merge state.

## Exact-head prerequisite audit

At exact closed-DI head:

1. `RegistryValidatedRequesterRendezvousStart` exists and can be produced only by the full current-registry validation path;
2. that carrier proves point-in-time registry eligibility only and explicitly is not policy authorization or provider registration authority;
3. current `prw-policy::Capability` has no requester-rendezvous-start or reachability-start capability;
4. `Capability::DeviceManage` is broader device-management authority and is not semantically equivalent to starting requester rendezvous;
5. `Capability::ForwardingCreate` represents authorized port-forward creation and is not semantically equivalent to requester/rendezvous current registration;
6. existing Agent admission code demonstrates the desired fail-closed pattern: map one operation to one exact capability, call a caller-supplied already-bound `PolicyEvaluator`, and create a typed admitted value only on `Decision::Allow`;
7. `PolicyEvaluator` itself is intentionally principal-agnostic, so runtime/composition code—not the capability enum—must supply an evaluator already bound/selected for the authenticated requester principal.

Therefore DJ selects a dedicated requester-rendezvous-start capability and typed post-policy provenance boundary. It does not silently reuse any existing broad capability.

## Selected capability

A future policy source-materialization checkpoint may add exactly one dedicated capability provisionally named:

`Capability::RequesterRendezvousStart`

The semantic meaning is narrowly:

> the already-authenticated requester principal is permitted by the selected policy context to initiate the requester/rendezvous start operation represented by one already registry-validated requester/target pair.

The capability does not authorize:

- arbitrary device management;
- port-forward creation;
- candidate publication;
- target transport authentication;
- provider mutation by itself;
- retirement/cancellation;
- network connection establishment;
- runtime/listener activation;
- deployment.

DJ explicitly rejects silent reuse of:

- `DeviceManage`;
- `ForwardingCreate`;
- `PolicyManage`;
- terminal capabilities;
- file capabilities;
- local read capabilities.

## Selected evaluation input boundary

Policy evaluation must occur only after C03e-DI has successfully produced:

`RegistryValidatedRequesterRendezvousStart`

A raw `RequesterRendezvousStartIntent`, standalone requester `DeviceId`, standalone `SessionId`, target `DeviceId`, `TransportIdentity`, endpoint, request ID, candidate ID, publisher traffic, or live-owner evidence is insufficient policy-admission input for this operation.

The registry-validated carrier remains the minimum provenance input to the policy seam.

## Requester principal binding rule

`PolicyEvaluator` remains principal-agnostic at its trait interface.

Therefore a future policy-composition caller must supply an evaluator already selected/bound for the same authenticated requester principal represented by the validated carrier's server-held `AuthenticatedDeviceSession`.

DJ does not select how that evaluator is stored, derived, cached, looked up, shared, synchronized, or owned.

It does not authorize selecting policy from:

- target identity;
- publisher identity;
- transport certificate identity;
- endpoint/address;
- candidate publication;
- request/correlation ID;
- live-owner grant.

If later source cannot prove that the evaluator is the one selected for the authenticated requester context, policy authorization must fail closed rather than infer principal equivalence.

## Selected fail-closed decision rule

The future policy gate must evaluate exactly:

`Capability::RequesterRendezvousStart`

against the caller-supplied requester-bound evaluator.

Only `Decision::Allow` may produce a typed policy-admitted value.

`Decision::Deny` must:

- return a stable policy-denied error;
- produce no policy-admitted value;
- produce no requester/rendezvous provider record;
- perform no provider mutation;
- create no retry token, lease, TTL, request authority, runtime task, or network side effect.

No default-allow, fallback-capability, capability substitution, or broad-capability implication is selected.

## Selected typed policy provenance

A future source-materialization checkpoint may introduce an effective crate-internal owned value provisionally named:

`PolicyAuthorizedRequesterRendezvousStart`

Possession of this value means only:

1. the contained requester/target provenance first passed the exact DI current-registry validation chain; and
2. the requester-bound policy evaluator subsequently returned `Decision::Allow` for the exact dedicated requester-rendezvous-start capability.

The preferred representation is to own the already validated `RegistryValidatedRequesterRendezvousStart` value as a nested/private field rather than decomposing it into raw identity parts.

This preserves the distinction between:

- unvalidated requester intent;
- registry-validated provenance;
- policy-authorized provenance.

The policy-authorized value should be owned, non-`Copy`, and should not require `Clone` unless a later independent seam proves duplication necessary and authority-safe.

DJ selects no arbitrary/public constructor for it.

## Preferred future authorization shape

The selected semantic shape is equivalent to:

```text
fn policy_authorize_requester_rendezvous_start<E: PolicyEvaluator + ?Sized>(
    validated: RegistryValidatedRequesterRendezvousStart,
    evaluator: &E,
) -> Result<
    PolicyAuthorizedRequesterRendezvousStart,
    RequesterRendezvousStartPolicyAuthorizationError,
>
```

This is semantic selection only. DJ does not materialize this function or any source signature.

The validated carrier is consumed so successful authorization can preserve exact provenance without cloning or broad extraction APIs.

On denial, the consumed registry-validated carrier may be dropped normally; it must not be converted into provider authority or returned as an authorization token.

## Selected policy error meaning

A future source-materialization may define a narrow stable failure with at least the semantic case:

`Denied`

DJ does not select wire/status mapping for that error.

It does not select PRWC/PRWM response codes, local IPC statuses, retries, telemetry, logging, or UI presentation.

## Provider separation

DJ does not call, expose, wrap, forward, or authorize:

- `InMemoryRequesterRendezvousAuthorityProvider::register_current`;
- `retire`;
- `remove_retired`;
- `authorize_current_for_publisher`;
- provider getters;
- provider extraction;
- mutable/raw provider references.

Even a future `PolicyAuthorizedRequesterRendezvousStart` is only typed proof that registry and policy prerequisites succeeded. The exact provider-mutation adapter remains a separate later checkpoint.

No provider mutation may execute on `Decision::Deny`.

## Currentness separation

Policy authorization does not freeze registry state.

DJ does not turn the DI point-in-time currentness proof into:

- a lease;
- a TTL;
- an expiry timestamp;
- a perpetual membership/device/session guarantee;
- a cached registration right.

Any material delay or asynchronous retention between validation/policy authorization and provider mutation must be evaluated by a later composition checkpoint. DJ authorizes no long-lived caching semantics.

## Identity boundaries

Requester logical identity remains the server-held authenticated PRW application session.

Target logical identity remains the exact validated `DeviceId`.

`TransportIdentity` remains lower-transport certificate identity only.

The policy capability does not promote any of the following into logical authority:

- `TransportIdentity`;
- IP/socket/endpoint;
- request ID;
- candidate ID;
- candidate freshness;
- live-owner fence/grant;
- publisher candidate-publication payload.

## Candidate-publication and live-owner isolation

Publisher-side candidate-publication traffic cannot invoke or satisfy the requester policy gate merely because it is authenticated.

Live-owner authority cannot substitute for requester policy authorization.

Conversely, requester policy authorization does not create live-owner authority or publisher publication authority.

No ordering with live-owner acquisition is selected here.

## Wire/runtime/networking separation

DJ selects no:

- `BridgeCommand` variant;
- opcode/frame/codec/parser;
- dispatcher;
- request/response mapping;
- listener;
- command loop;
- task;
- channel/actor;
- `Arc`, `Mutex`, `RwLock`, global, singleton or shared-worker topology;
- database/persistence/distributed coordination;
- network socket/connect/listen behavior;
- STUN/ICE/TURN/relay behavior;
- deployment/restart/recovery;
- merge.

## Exact audited anchors at closed DI

- DI validated provenance source: `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_registry_validation.rs`
  - blob `1c021bc95a3d674722bfd70559156fa75e07e578`
- current policy model: `crates/prw-policy/src/lib.rs`
  - blob `c2a02e5640a3274fa7a6d04dacb91d06a8d0df93`
- existing exact local typed policy-admission precedent: `crates/prw-agent/src/local_commands/admission.rs`
  - blob `9c78280de69e44199d37795037eac79316826694`
- boundary policy-processing precedent: `crates/prw-agent/src/local_commands/boundary_policy_processor.rs`
  - blob `3bf8952610b0d19e6bf073afb19acd428944ed57`
- DI contract: `ca9c56440fe6bfb5a3b117e93de1d2ddafa15384`

The local-command modules are precedent only for fail-closed typed admission. DJ does not reuse their request envelopes, response mapping, boundary reader, or runtime composition.

## Dependency and lock guards

DJ requires no manifest or lockfile mutation.

Expected unchanged blobs from closed DI:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- Android native `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

## Explicit non-selections

DJ does not materialize or authorize:

- `Capability::RequesterRendezvousStart` source;
- policy-admission source/function;
- `PolicyAuthorizedRequesterRendezvousStart` source;
- evaluator lookup/storage/ownership topology;
- provider mutation/access/forwarding;
- provider-ready extraction or conversion;
- target transport readiness;
- wire command/opcode/frame/parser/dispatcher;
- request-ID authority;
- retry/deduplication;
- registration retirement/cancellation/cleanup;
- synchronization/shared-worker topology;
- command-loop/listener activation;
- Agent binary wiring;
- persistence/database/distributed coordination;
- production networking;
- deployment/restart/recovery;
- merge.

## Closure requirements

DJ may close only if:

1. closed DI remains the exact predecessor and merge base;
2. final DI -> DJ diff is exactly this one documentation path;
3. no source, manifest, lockfile, workflow, binary, Android, desktop, networking or deployment path changes;
4. no existing broad capability is silently reused;
5. the dedicated capability remains requester-rendezvous-start-specific;
6. policy evaluation occurs only after DI registry validation;
7. evaluator principal binding remains the caller's precondition and is not inferred from target/transport/correlation state;
8. denial remains fail-closed and produces no admitted token/provider mutation;
9. typed policy provenance remains distinct from provider registration authority;
10. all automatically triggered canonical validations on the exact final head reach terminal non-failing verdicts;
11. Android no-trigger, if applicable to the docs-only diff, is recorded as no-trigger and not misreported as PASS;
12. manifest/lock guards remain exact;
13. an immutable DJ audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-read back byte-exact;
14. rolling Drive evidence is freshly guarded against exact closed DI (`987535` bytes / `ff607b6067b940d4e74fc2c0c04f49f9d7f96b54e10fea8cfa3668da437347e5`);
15. the DJ closure record is appended only to those exact predecessor bytes;
16. the complete closed-DI rolling prefix is preserved byte-for-byte;
17. rolling Drive update raw-readback matches intended bytes/hash exactly;
18. only after durable Drive proof may PR status move `STAGED -> CLOSED`;
19. PR remains draft/open/unmerged;
20. final GitHub/Drive race checks remain exact.

## Safe successor rule

After durable DJ closure, begin with a fresh exact-head read-only audit.

The next narrow candidate seam, if still supported, is source materialization of the dedicated capability and typed policy-admission boundary only.

That source-materialization checkpoint must not bundle provider mutation/access, wire changes, retirement/cancellation, runtime/listener activation, synchronization, networking, deployment, or merge.
