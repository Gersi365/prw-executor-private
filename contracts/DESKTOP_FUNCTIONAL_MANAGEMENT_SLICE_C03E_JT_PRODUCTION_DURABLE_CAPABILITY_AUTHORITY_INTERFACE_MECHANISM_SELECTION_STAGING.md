# Desktop Functional Management Slice — C03e-JT Production Durable Capability-Authority Interface Mechanism Selection

Status: `SELECTION_STAGING`
Date: `2026-09-04`

## 1. Checkpoint classification

C03e-JT is a documentation-only concrete mechanism selection checkpoint.

Target gate:

`C03E_JT_PRODUCTION_DURABLE_CAPABILITY_AUTHORITY_INTERFACE_MECHANISM_SELECTED`

Target closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_AUTHORITY_INTERFACE_MECHANISM_SELECTION`

C03e-JT selects the concrete provider-backed mechanism for the production capability-authority provenance boundary selected by closed C03e-JS.

JT does not materialize Rust source. It does not populate an allow-bearing production policy, wire a remote-session caller, change `LinuxAgentRemoteProcessOperationInputs`, invoke a remote operation, alter `run()` or `main.rs`, activate a listener/runtime/network path, deploy, or merge.

## 2. Exact predecessor authority

Predecessor checkpoint:

`C03e-JS — Production capability-authority provenance boundary selection`

Predecessor branch:

`phase-152-c03e-js-production-capability-authority-provenance-boundary-selection`

Exact predecessor head / merge base:

`eb650196962d69187c5eb85624c0485197651b99`

Exact predecessor tree:

`d320d41afa4446863e3f8453e9e99a62bfee54e1`

Exact predecessor contract blob:

`88311ba002c92642d9f29e6f887ba556d1ba97a2`

Predecessor gate:

`C03E_JS_PRODUCTION_CAPABILITY_AUTHORITY_PROVENANCE_BOUNDARY_SELECTED`

Predecessor closure:

`CLOSED_PRODUCTION_CAPABILITY_AUTHORITY_PROVENANCE_SELECTION`

JS established that the existing in-memory `SharedCurrentCapabilityAuthority<P>` is not itself a production source because it requires one already-populated `WorkspaceDeviceRegistry` plus one already-selected policy evaluator `P`, while exact production durable-registry and remote-policy provenance remain separate concerns.

## 3. Exact current source findings

### 3.1 Existing in-memory shared authority

Exact source:

`crates/prw-agent/src/remote_session_capability_runtime/shared_current_capability_authority.rs`

Exact blob:

`60307fff4dd0fd573192ba6e6fab9dedd3321dda`

The current owner is:

`SharedCurrentCapabilityAuthority<P>`

and stores exactly:

- one in-memory `WorkspaceDeviceRegistry`;
- one principal-agnostic `PolicyEvaluator` value `P`;
- under one Tokio `RwLock` shared through `Arc`.

Its current authorization use site obtains a coherent in-memory registry/policy read and performs synchronous Phase 143 authorization.

This owner has no durable-provider bootstrap, no durable read semantics, no snapshot loading, no synchronization/watch path and no production policy source.

### 3.2 Existing production durable registry

Exact semantic source:

`crates/prw-registry/src/durable_registry_etcd_store.rs`

Exact blob:

`1e04b366471fe2d4433de3c383efb4108d828983`

The durable store already provides provider-backed semantic current authority including:

- exact current membership read;
- exact current device read;
- `current_transport_identity(...)`;
- `validate_transport_identity(...)`;
- `validate_authenticated_session(...)`;
- exact Phase 130 semantic failures;
- provider/currentness failures separated through `DurableRegistryEtcdStoreError`.

`validate_authenticated_session(...)` already obtains membership and device through one existing `linearizable_pair_get(...)` transaction and validates the session against that paired current snapshot.

The complete device record used by that pair already contains the optional current `TransportIdentity`.

### 3.3 Existing production durable-registry Agent custody

Exact source:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

Exact blob:

`fd78512a24b824483a101962c3c63d91ad4b2cc1`

The custody owner intentionally retains one `DurableRegistryEtcdStore` privately and exposes only operation-specific methods. It exposes no generic store getter, no raw provider handle and no extraction path.

JT preserves that custody discipline.

### 3.4 Existing Phase 143 capability bridge

Exact source:

`crates/prw-remote-bridge/src/lib.rs`

Exact blob:

`7b1c5c62339983da6ae2556f73510d7582ec0c5b`

The existing `CapabilityBridge<'a, P>` is synchronous and borrows:

- `&WorkspaceDeviceRegistry`;
- `&P` where `P: PolicyEvaluator`.

Its exact authorization order remains authoritative:

1. reject non-request outer PRWM frames;
2. validate verifier-owned remote session lease time;
3. validate the authenticated session against current registry state;
4. validate the presented transport identity against the same logical device's current transport binding;
5. decode the bounded PRWC command;
6. derive the exact required capability;
7. require `Decision::Allow` from the policy evaluator;
8. only then construct private-field `AuthorizedCapabilityRequest`.

`AuthorizedCapabilityRequest` has no external public constructor. JT preserves `prw-remote-bridge` as the sole constructor authority for that type.

### 3.5 Existing policy model

Exact source:

`crates/prw-policy/src/lib.rs`

Exact blob:

`3745024b5b222fcb36244222fad3c9c05a59cece`

The current `PolicyEvaluator` interface is principal-agnostic.

Existing `BoundedLocalReadPolicy` and `BoundedLocalManagementPolicy` are reviewed local configuration primitives. JT does not reinterpret either as a production remote policy source.

No existing source establishes an allow-bearing production remote capability policy.

### 3.6 Executable remains dormant

Exact `main.rs` blob:

`db6b8028c6df100a961a0fb5818347bea2fdc5c1`

JT does not alter executable reachability.

## 4. Rejected snapshot/mirror mechanism

JT explicitly rejects materializing production `SharedCurrentCapabilityAuthority<P>` by copying the durable registry into a `WorkspaceDeviceRegistry` snapshot.

No selected mechanism may:

- prefix-scan durable registry state;
- enumerate all memberships/devices to synthesize an in-memory registry;
- create a watch-backed or polling mirror;
- cache a prior durable registry authorization result;
- treat a partial key set as a complete registry;
- silently tolerate missed durable updates;
- declare a snapshot current without an explicit currentness protocol;
- use `WorkspaceDeviceRegistry::new()` as production authority.

The current durable model intentionally has no prefix scan, Watch, Lease, TTL or cache authority. JT does not add those mechanisms indirectly.

## 5. Rejected two-read session/transport mechanism

JT also rejects this production sequence:

```text
DurableRegistryEtcdStore::validate_authenticated_session(...).await
then later
DurableRegistryEtcdStore::validate_transport_identity(...).await
```

as the selected production capability-authorization currentness boundary.

Those two operations would perform separate authoritative reads. Registry/device state could change between them, so they would not prove session binding and presented transport against one coherent device observation.

The selected mechanism must validate session binding and presented transport identity from the same paired membership/device transaction result.

## 6. Selected durable current-authorization primitive

JT selects one new semantic operation on the existing durable store named exactly:

`validate_authenticated_session_and_transport_identity`

Selected signature semantics:

```text
DurableRegistryEtcdStore::validate_authenticated_session_and_transport_identity(
    &mut self,
    session: &AuthenticatedDeviceSession,
    presented_transport_identity: TransportIdentity,
) -> Result<RegistryValidatedPrincipal, DurableRegistryEtcdStoreError>
```

The exact Rust formatting may follow rustfmt, but the argument and return authority above may not widen.

The operation must:

1. encode the exact membership key from the supplied authenticated session's workspace/user identity;
2. encode the exact device key from the same authenticated session's logical `DeviceId`;
3. call the existing raw provider `linearizable_pair_get(...)` exactly once;
4. decode and bind the exact returned membership/device observations through existing canonical durable-registry codecs;
5. preserve existing Phase 130 validation precedence for active membership, enrolled device and immutable authenticated-session binding;
6. validate the presented `TransportIdentity` against the exact current transport value in that same decoded device record;
7. return the existing `RegistryValidatedPrincipal` only after every registry and transport check succeeds.

The operation must perform no second provider read.

## 7. Exact same-pair transport rule

The current transport comparison must use the `RegisteredDevice` decoded from the same pair transaction already used for session validation.

It must not call:

- `device(...)`;
- `current_transport_identity(...)`;
- `validate_transport_identity(...)`;
- another `linearizable_get(...)`;
- another `linearizable_pair_get(...)`.

after the selected pair has been obtained.

A missing transport, stale/mismatched transport or revoked/nonparticipating device fails closed using existing Phase 130/durable semantic classifications.

No fallback transport, cached transport, alternate device or caller-selected transport authority is permitted.

## 8. Provider/currentness failure semantics remain unchanged

The selected method returns the existing:

`DurableRegistryEtcdStoreError`

without inventing a second provider error taxonomy.

Existing meanings remain authoritative:

- `Semantic(RegistryError)` for semantic failure proven from canonical current state;
- `ReadUnavailable` when an authoritative read cannot be obtained;
- `MutationIndeterminate` remains a mutation-only category and is not fabricated by this read path;
- `InvalidAuthority` for malformed/inconsistent provider or canonical authority;
- `CurrentnessConflict` remains mutation/CAS currentness classification and is not fabricated by this read path.

No provider failure becomes authorization success.

## 9. Selected provider-aware Phase 143 bridge mechanism

After the durable combined validator is materialized and closed, a later separately selected/materialized bridge checkpoint may add a provider-aware Phase 143 authorization path in `prw-remote-bridge`.

JT selects the concrete bridge type name:

`DurableCapabilityBridge<'a, P>`

with conceptual custody:

```text
DurableCapabilityBridge {
    registry: &'a mut DurableRegistryEtcdStore,
    policy: &'a P,
}
```

where `P: PolicyEvaluator`.

The type is provider-aware only at the semantic registry layer. It accepts no raw etcd client, endpoint, credential, TLS/auth/RBAC object, service URL or Agent runtime owner.

Its future async `authorize(...)` operation must preserve the current Phase 143 order exactly:

1. outer request-kind validation;
2. lease-time validation;
3. exactly one call to `validate_authenticated_session_and_transport_identity(...)`;
4. bounded PRWC decode;
5. exact capability derivation;
6. policy evaluation;
7. private `AuthorizedCapabilityRequest` construction only after success.

This avoids duplicating Phase 143 authorization in the Agent crate.

## 10. Selected bridge error envelope

A future durable bridge may add one bounded public error envelope named:

`DurableCapabilityBridgeError`

with only these semantic categories:

- `Bridge(RemoteBridgeError)` — existing Phase 143 request/lease/semantic rejection/decode/policy categories;
- `Authority(DurableRegistryEtcdStoreError)` — durable provider/canonical authority failures that cannot be represented truthfully as an ordinary semantic denial.

For `DurableRegistryEtcdStoreError::Semantic(...)`, the bridge may preserve the existing coarse Phase 143 external classification:

- membership/device/session semantic rejection -> `RemoteBridgeError::RegistryRejected`;
- exact transport missing/mismatch semantic rejection after otherwise-valid session binding -> `RemoteBridgeError::TransportIdentityRejected`.

Non-semantic provider/canonical authority failures must remain available as `Authority(...)` and must not be translated to success, policy denial or fabricated transport mismatch.

No provider endpoint, credential, key material or record bytes may appear in bounded display text.

## 11. Existing synchronous bridge remains valid

JT does not remove or weaken the existing `CapabilityBridge<'a, P>`.

That bridge remains the in-memory/source/disposable path and continues to use `WorkspaceDeviceRegistry` exactly as today.

The future durable bridge is a sibling production semantic path, not a silent change to the meaning of the existing in-memory owner.

## 12. Selected initial production policy evaluator

JT selects one dedicated fail-closed production remote policy baseline named exactly:

`ProductionRemoteCapabilityDenyAllPolicy`

The type is a zero-external-source `PolicyEvaluator` whose evaluation result is always:

`Decision::Deny`

for every represented `Capability`.

This is an explicit production safety baseline, not reuse of either local policy type and not a claim that remote policy provenance capable of granting operations is complete.

Construction of this deny-all baseline performs no I/O, environment read, service mutation, provider lookup or role mapping.

## 13. Why deny-all is selected first

No exact source currently establishes an authoritative allow-bearing remote capability policy.

JT therefore refuses to invent:

- role -> capability grants;
- implicit Owner/Admin allow rules;
- global `allow_all`;
- environment-controlled grants;
- unreviewed systemd credential formats;
- database/control-plane policy schemas;
- local-management policy reuse;
- test fixture policy promotion.

A dedicated deny-all production baseline allows the durable authority interface to be materialized and validated without accidentally enabling any remote capability.

Any future policy capable of returning `Decision::Allow` requires a new explicit production-policy source selection and materialization checkpoint before executable/runtime activation.

## 14. Selected Agent production authority custody shape

After the durable validator, durable bridge and deny-all policy prerequisites are separately materialized, a later separately gated Agent checkpoint may select/materialize one production authority owner named:

`ProductionDurableCapabilityAuthority`

The selected conceptual custody is:

```text
ProductionDurableCapabilityAuthority {
    registry_custody: Arc<tokio::sync::Mutex<ProductionDurableRegistryRuntimeCustody>>,
    policy: ProductionRemoteCapabilityDenyAllPolicy,
}
```

The Tokio mutex is selected only because the existing durable semantic store is privately retained and its provider operations require mutable custody.

Clones of the future authority owner may share one outer `Arc`; they must not clone durable registry state into snapshots.

The mutex may serialize one bounded authority transaction at a time. It must not be held across dispatcher execution, response I/O, worker cancellation waits or unrelated runtime lifecycle work.

## 15. Existing durable custody remains private

The future production authority must reuse the existing `ProductionDurableRegistryRuntimeCustody` rather than expose the inner `DurableRegistryEtcdStore` through a generic getter.

A later operation-specific custody method may invoke the durable bridge against the privately held store.

JT does not select:

- `store()`;
- `store_mut()`;
- `into_store()`;
- raw executor extraction;
- raw etcd client access.

## 16. Existing `SharedCurrentCapabilityAuthority<P>` is not silently repurposed

JT does not mutate the semantics of existing:

`SharedCurrentCapabilityAuthority<P>`.

That owner remains the current in-memory registry/policy authority used by existing dormant/source paths.

The selected production durable mechanism is a sibling authority path because the existing in-memory owner cannot truthfully represent provider-backed currentness without snapshot/mirror semantics that JS/JT reject.

Any future replacement of the `SharedCurrentCapabilityAuthority<P>` field inside `LinuxAgentRemoteProcessOperationInputs` is a later aggregate-interface selection and is not authorized by JT.

## 17. Production policy capable of allow remains separately gated

`ProductionRemoteCapabilityDenyAllPolicy` is intentionally non-enabling.

Before any production remote capability operation may become executable, a fresh checkpoint must select authoritative allow-bearing policy provenance, including at minimum:

- exact principal scope;
- exact capability decision source;
- currentness/reload semantics;
- missing/invalid source behavior;
- custody/lifetime;
- whether policy is global, workspace-bound, user-bound or device-bound;
- exact failure semantics.

No such source is inherited from JT.

## 18. Immediate source-materialization successor ceiling

After JT closes, the **first** source-materialization successor may change exactly one repository path:

`crates/prw-registry/src/durable_registry_etcd_store.rs`

It may materialize only:

1. `validate_authenticated_session_and_transport_identity(...)`;
2. minimal private helper refactoring required to reuse the current paired membership/device decode without changing existing behavior;
3. exact same-pair transport validation using the already-decoded current device record;
4. focused tests proving one-pair semantics and failure classification without production provider mutation;
5. strictly local lint/rustfmt corrections required by the exact source shape.

It must not yet change:

- `prw-remote-bridge`;
- `prw-policy`;
- `prw-agent`;
- manifests/lockfile;
- workflows;
- `main.rs`;
- runtime/executable callers.

## 19. First successor must stop before bridge materialization

After the one-file durable combined-validator source checkpoint closes: **STOP**.

A fresh exact-head audit is required before selecting/materializing the `DurableCapabilityBridge` surface.

JT does not authorize combining the durable validator, durable bridge, deny-all policy and Agent owner into one source commit or one broad source checkpoint.

## 20. Authorization order invariant

The selected production mechanism must preserve this order:

```text
authenticated bound session / presented transport
    -> outer request-kind validation
    -> verifier-owned lease validation
    -> one paired durable membership/device current read
    -> session binding validation
    -> presented transport validation from the same device observation
    -> PRWC decode
    -> exact capability derivation
    -> production policy decision
    -> AuthorizedCapabilityRequest
    -> dispatcher only after authorization
```

No decode success, provider read success, request-id match, transport match or policy object possession is itself authorization.

## 21. Identity and security invariants

JT preserves:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

Specifically:

- `DeviceId` / authenticated PRW session identity remains logical identity;
- `TransportIdentity` remains transport identity/current transport evidence;
- transport-key rotation never redefines logical device identity;
- IP/`SocketAddr` remains transient reachability data only;
- `SessionId` remains authentication/session correlation/lifetime context, not device identity;
- PRWM `request_id` remains transaction correlation only;
- policy evaluation does not authenticate a principal;
- durable registry validation does not itself grant a capability;
- deny-all policy does not create readiness or successful runtime behavior.

No PID/UID/GID or host-account identity becomes PRW logical identity.

## 22. Explicit exclusions

C03e-JT does not perform or authorize:

- Rust/source materialization in JT itself;
- registry snapshot, scan, Watch, Lease, TTL, cache or mirror authority;
- a second provider read inside the selected combined validator;
- registry mutation;
- policy persistence/load/mutation;
- any allow-bearing production policy;
- role-to-capability mapping;
- local policy promotion;
- `SharedCurrentCapabilityAuthority` mutation;
- `LinuxAgentRemoteProcessOperationInputs` mutation;
- session-authentication production population;
- expected-request producer/receiver population;
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

## 23. Exact intended JS -> JT scope

JT is documentation-only.

The exact branch must differ from closed JS only by:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_JT_PRODUCTION_DURABLE_CAPABILITY_AUTHORITY_INTERFACE_MECHANISM_SELECTION_STAGING.md`

Any Rust/source, manifest, lockfile, workflow, Android, packaging, registry, policy, auth, provider, runtime, `main.rs`, service/systemd or deployment path change blocks JT closure.

## 24. Validation semantics

Exact-final-head CI must be tied only to the exact final JT head.

Path-filtered workflows reporting `SKIPPED` remain `SKIPPED` and must not be represented as PASS.

A successful docs-only JT validation proves only the selection artifact and repository integrity. It does not validate future durable-validator, bridge, policy or Agent-owner source implementations.

## 25. Successor rule

After JT closure: **STOP**.

The immediate successor may only materialize the one-file durable combined session+transport validator selected in section 18.

After that materialization closes, stop again for a fresh exact-head audit before any durable bridge source selection/materialization.

No policy allow source, Agent production authority owner, aggregate input replacement, session/expected-request/dispatcher/timing/callback/requester-rendezvous/executable/runtime boundary is inherited as executable authority from JT.

## 26. Closure requirements

C03e-JT may close only after:

1. exact JT branch head is re-read;
2. exact JS -> JT merge base equals `eb650196962d69187c5eb85624c0485197651b99`;
3. aggregate changed-file set contains exactly the single JT contract path;
4. exact-final-head CI reaches terminal expected conclusions;
5. immutable Drive audit evidence is published and raw-readback verified;
6. exact-title Drive uniqueness is verified under the canonical audit parent;
7. the JT PR remains draft/open/unmerged at the exact audited head.

After closure, the branch and PR remain staging evidence only. No merge, deployment or runtime activation is implied.
