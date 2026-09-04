# Desktop Functional Management Slice C03e-KD

## Production Durable Capability-Authority Population Composition Selection — Staging

Date: 2026-09-04
Repository: `Gersi365/prw-executor-private`
Repository ID: `1334911207`

## 1. Checkpoint purpose

C03e-KD is a **selection-only** checkpoint.

It selects the narrow production population composition that may later construct one dormant
`ProductionDurableCapabilityAuthority` from the already-materialized production durable-registry
systemd custody/provider bootstrap.

C03e-KD does not materialize Rust source.

It does not replace any aggregate input, wire any executable caller, authorize any request, invoke a
dispatcher, write a response, read another control frame, start a listener, publish readiness, create
a runtime task, deploy anything, or enable any capability.

Target gate:

`C03E_KD_PRODUCTION_DURABLE_CAPABILITY_AUTHORITY_POPULATION_COMPOSITION_SELECTED`

Target closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_AUTHORITY_POPULATION_COMPOSITION_SELECTION`

## 2. Exact predecessor authority

Closed predecessor:

`C03e-KC — Production Durable Capability-Authority Authorization Invocation Source Materialization`

Predecessor branch:

`phase-152-c03e-kc-production-durable-capability-authority-authorization-invocation-source-materialization`

Exact predecessor head:

`ab232d36e2b38836db126ba0535aa2b15ab28d09`

Exact predecessor tree:

`c1ab16f13fbf6520dac6d8339159e5da143711c3`

Exact predecessor capability-authority source:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

Exact predecessor capability-authority source blob:

`90b12c182d6564b42e3f22f9e3dd594ec94d2fe5`

Predecessor PR:

`#414`

At the fresh KD audit, PR #414 remained draft/open/unmerged at exact KC head.

Predecessor closure:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_AUTHORITY_AUTHORIZATION_INVOCATION_SOURCE_MATERIALIZATION`

## 3. Fresh successor namespace guard

Before KD branch creation:

- branch search for `phase-152-c03e-kd` returned zero results;
- PR search for `C03e-KD` returned zero results;
- the exact KD contract path did not exist at the exact KC head;
- the KD branch was created only from exact KC head
  `ab232d36e2b38836db126ba0535aa2b15ab28d09`.

No existing successor artifact was overwritten or reused.

## 4. Exact source authorities re-audited

### 4.1 Production durable-registry custody/provider bootstrap

Path:

`crates/prw-agent/src/production_durable_registry_custody_bootstrap.rs`

Exact KC blob:

`e27e4d0a34ded1002efcb14a5c9844560a2c8bf1`

Existing operation:

```text
bootstrap_production_durable_registry_from_systemd_credentials()
    -> Result<DurableRegistryEtcdStore,
              ProductionDurableRegistryCustodyBootstrapError>
```

Existing semantics:

1. load the fixed production durable-registry systemd credential set through the existing custody
   loader exactly once;
2. move only the validated opaque provider config into the existing production provider bootstrap;
3. bootstrap the existing bounded production executor exactly once;
4. move that executor directly into `DurableRegistryEtcdStore::new(...)`;
5. return the semantic durable-registry store without performing a registry semantic operation.

Existing failure surface:

```text
ProductionDurableRegistryCustodyBootstrapError::Custody(...)
ProductionDurableRegistryCustodyBootstrapError::ProviderBootstrap(...)
```

The existing display remains provider/secret neutral.

The existing function performs credential-file reads and provider network bootstrap when awaited,
but performs no registry Get/Txn/Put, no retry/fallback/reconnect loop, no background task and no
runtime activation.

### 4.2 Production durable-registry runtime custody

Path:

`crates/prw-agent/src/production_durable_registry_runtime_custody.rs`

Exact KC blob:

`90b12c182d6564b42e3f22f9e3dd594ec94d2fe5`

Existing infallible ownership adaptation:

```text
ProductionDurableRegistryRuntimeCustody::from_store(
    DurableRegistryEtcdStore
) -> ProductionDurableRegistryRuntimeCustody
```

The store remains private and non-cloneable.

No generic store getter, raw executor getter, provider-client getter or extraction callback exists.

### 4.3 Production durable capability-authority custody

Existing infallible ownership adaptation:

```text
ProductionDurableCapabilityAuthority::from_registry_custody(
    ProductionDurableRegistryRuntimeCustody
) -> ProductionDurableCapabilityAuthority
```

Existing custody shape:

```text
ProductionDurableCapabilityAuthority {
    registry_custody:
        Arc<tokio::sync::Mutex<ProductionDurableRegistryRuntimeCustody>>,
    policy:
        ProductionRemoteCapabilityDenyAllPolicy,
}
```

The policy is the fixed production deny-all baseline.

Construction performs no lock acquisition, provider I/O, registry operation, policy evaluation,
authorization, task spawn or runtime activation.

### 4.4 KC authorization invocation remains separate

KC materialized only:

```text
ProductionDurableCapabilityAuthority::authorize_capability_transaction(...)
```

That method performs the bounded current durable-authority authorization transaction only when a
later caller explicitly invokes it.

KD does not invoke that method.

KD population therefore does **not** establish authorization success, capability availability,
registry-current principal validity, transport-currentness validity, or session admission.

## 5. Production composition gap observed at exact KC head

At exact KC head, the repository already contains all three required pieces:

```text
fixed production systemd durable-registry custody/provider bootstrap
    -> DurableRegistryEtcdStore
    -> ProductionDurableRegistryRuntimeCustody
    -> ProductionDurableCapabilityAuthority
```

However, there is no operation-specific production helper that composes this exact chain and returns
one populated `ProductionDurableCapabilityAuthority`.

A later executable/aggregate caller should not be required to reopen raw provider custody or recreate
these ownership steps ad hoc.

The missing seam is therefore a narrow composition helper, not a new authority model and not an
aggregate replacement.

## 6. Selected future helper

C03e-KD selects the following future operation-specific helper in Agent production durable-registry
custody composition:

```text
bootstrap_production_durable_capability_authority_from_systemd_credentials()
```

Selected conceptual Rust signature:

```rust
pub(crate) async fn bootstrap_production_durable_capability_authority_from_systemd_credentials()
    -> Result<
        ProductionDurableCapabilityAuthority,
        ProductionDurableRegistryCustodyBootstrapError,
    >
```

The exact source-materialization checkpoint may use equivalent rustfmt-compliant layout, but must not
widen arguments, visibility, return ownership or error semantics beyond this selection.

## 7. Exact selected composition law

The future helper must perform exactly this semantic sequence:

```text
bootstrap_production_durable_registry_from_systemd_credentials()
    .await
    ?
    -> DurableRegistryEtcdStore

ProductionDurableRegistryRuntimeCustody::from_store(store)
    -> ProductionDurableRegistryRuntimeCustody

ProductionDurableCapabilityAuthority::from_registry_custody(registry_custody)
    -> ProductionDurableCapabilityAuthority

return authority
```

Equivalent conceptual body:

```rust
let store = bootstrap_production_durable_registry_from_systemd_credentials().await?;
let registry_custody = ProductionDurableRegistryRuntimeCustody::from_store(store);
Ok(ProductionDurableCapabilityAuthority::from_registry_custody(
    registry_custody,
))
```

No additional provider bootstrap, registry semantic read, policy evaluation, authorization or runtime
operation is permitted inside this helper.

## 8. Bootstrap cardinality and failure law

The future helper must call the existing production durable-registry bootstrap exactly once.

It must not:

- retry custody loading;
- retry provider bootstrap;
- attempt a second provider bootstrap;
- fall back to an in-memory registry;
- construct an empty/default registry;
- recover from custody/provider failure with a synthetic authority;
- return a partially initialized authority;
- swallow or remap the existing bootstrap failure.

The two ownership adaptations after successful store creation are currently infallible.

Therefore KD selects **no new error enum**.

The future helper must preserve:

`ProductionDurableRegistryCustodyBootstrapError`

unchanged.

## 9. Currentness law

KD population establishes custody only.

It does not perform a durable registry semantic read.

Registry/session/transport currentness remains established later only by the existing provider-aware
authorization path when `authorize_capability_transaction(...)` reaches
`DurableCapabilityBridge::authorize(...)` and the existing combined durable validator.

Therefore:

- provider bootstrap success is not registry-currentness proof;
- custody success is not session validity;
- custody success is not transport validity;
- custody success is not authorization;
- construction success is not capability grant.

No cache, registry snapshot, mirror or stale authority is introduced by KD.

## 10. Policy law

The populated authority must contain only the existing:

`ProductionRemoteCapabilityDenyAllPolicy`

through the existing `ProductionDurableCapabilityAuthority::from_registry_custody(...)` constructor.

The population helper must not:

- accept a policy argument;
- load policy from environment/systemd/filesystem;
- map roles to capabilities;
- reuse `BoundedLocalReadPolicy` or `BoundedLocalManagementPolicy`;
- install an allow-all/test policy;
- mutate policy after construction.

No positive production capability is enabled by KD.

Any production policy capable of `Decision::Allow` remains separately gated.

## 11. Identity law

KD preserves the canonical identity model:

```text
PRW logical device/session identity
    -> registry/discovery
    -> current reachable endpoint/candidates
    -> authenticated transport
```

Specifically:

- `DeviceId` remains logical device identity;
- authenticated PRW session identity remains logical session identity;
- `TransportIdentity` remains transport evidence only;
- request IDs remain correlation only;
- IP/port remains transient reachability only;
- transport key rotation does not redefine logical identity.

Population takes no DeviceId, TransportIdentity, IP, port, request ID, PID, UID or GID input.

## 12. Relationship to existing production peer lookup

The existing production peer-input population path separately performs its own operation-specific
production durable-registry bootstrap and current same-device transport lookup.

KD does not alter, deduplicate, share, merge or replace that path.

KD does not claim that capability-authority population and peer-identity lookup use the same provider
connection or the same `DurableRegistryEtcdStore` instance.

Any future decision to share provider/store custody across those operations requires a separate
explicit checkpoint.

KD therefore introduces no hidden shared global registry owner.

## 13. Ownership and lifetime law

On successful future population:

1. one returned `DurableRegistryEtcdStore` is consumed exactly once;
2. it is moved into one `ProductionDurableRegistryRuntimeCustody`;
3. that custody is consumed exactly once by
   `ProductionDurableCapabilityAuthority::from_registry_custody(...)`;
4. the returned authority owns the resulting private `Arc<Mutex<_>>` custody;
5. no raw store/executor/provider handle escapes.

The helper returns the authority by value.

It does not register global state or retain a second alias in the bootstrap module.

## 14. No authorization invocation during population

The future helper must not call:

- `authorize_capability_transaction(...)`;
- `DurableCapabilityBridge::authorize(...)`;
- durable session validation;
- transport validation;
- policy `evaluate(...)`;
- PRWC decode;
- capability derivation;
- `AuthorizedCapabilityRequest` construction.

The returned authority remains dormant until a later explicitly gated caller uses it.

## 15. No lock acquisition during population

The future helper must not acquire the authority mutex.

`ProductionDurableCapabilityAuthority::from_registry_custody(...)` may construct the mutex container,
but no `.lock().await` belongs in the population helper.

Provider bootstrap work completes before the runtime custody is moved into the authority.

## 16. Selected source placement

The future population helper belongs in:

`crates/prw-agent/src/production_durable_registry_custody_bootstrap.rs`

Rationale:

- this module already owns the fixed production systemd custody/provider bootstrap seam;
- the new helper is a narrow composition immediately above that existing bootstrap;
- keeping the helper here avoids importing credential/provider bootstrap concerns into runtime custody;
- it requires no new public module or manifest dependency;
- it leaves `production_durable_registry_runtime_custody.rs` focused on runtime ownership and
  operation-specific authorization/lookup methods.

## 17. Immediate source-materialization successor ceiling

After KD closes, the immediate source-materialization successor may change exactly **one repository
path**:

`crates/prw-agent/src/production_durable_registry_custody_bootstrap.rs`

Permitted in that one path:

1. import the existing runtime-custody and capability-authority types;
2. materialize
   `bootstrap_production_durable_capability_authority_from_systemd_credentials(...)`;
3. compose exactly one existing durable-registry bootstrap with the two existing infallible ownership
   adaptations;
4. preserve `ProductionDurableRegistryCustodyBootstrapError` unchanged;
5. add focused same-file tests that prove signature/ownership shape without polling the production
   bootstrap future;
6. local rustfmt/lint corrections required by the exact materialization.

Forbidden in the immediate successor:

- `crates/prw-agent/src/production_durable_registry_runtime_custody.rs`;
- `crates/prw-agent/src/linux_bootstrap.rs`;
- `crates/prw-agent/src/lib.rs`;
- `crates/prw-agent/src/main.rs`;
- any `prw-remote-bridge` file;
- any `prw-registry` file;
- any `prw-policy` file;
- manifests;
- `Cargo.lock`;
- workflows;
- Android source;
- packaging/service/systemd/runtime configuration.

Changed-path ceiling for that successor is exactly one path.

## 18. Focused future test law

The source-materialization successor may add only tests that remain dormant with respect to production
I/O.

A valid focused test may type-check that calling the helper returns a future whose output is:

```text
Result<
    ProductionDurableCapabilityAuthority,
    ProductionDurableRegistryCustodyBootstrapError,
>
```

The test must not poll that future against real systemd credentials or a real provider.

No disposable provider workflow is selected by KD.

## 19. Aggregate boundary remains unresolved

KD does not modify or select a replacement for the current aggregate field:

```text
LinuxAgentRemoteProcessOperationInputs<...>::capability_authority
    : SharedCurrentCapabilityAuthority<P>
```

The in-memory `SharedCurrentCapabilityAuthority<P>` remains the current aggregate type at this
checkpoint.

The durable production authority is a sibling dormant production path.

Replacing or generalizing that aggregate field requires a later explicit interface checkpoint because
it affects downstream remote-session execution types and cannot be smuggled into population.

## 20. Production caller provenance remains unresolved

KD does not select an executable caller for the future population helper.

It does not decide:

- where startup calls it;
- whether a later aggregate owns the returned authority;
- how session-authentication production inputs are populated;
- where presented `TransportIdentity` comes from;
- where `RemoteSessionLease` comes from;
- where production `now_unix_seconds` comes from;
- how expected requests are produced;
- how authorization success reaches dispatch;
- how authorization failure is framed on the same stream.

Each remains separately gated.

## 21. Runtime exclusions

KD does not authorize or perform:

- aggregate replacement;
- session-authentication production population;
- expected-request receiver/producer integration;
- admission-timing production population;
- completion/rejection/repeated-failure callback population;
- requester/rendezvous population or invocation;
- operation-factory invocation;
- companion spawn;
- dispatcher invocation;
- filesystem/terminal/forwarding operation execution;
- authorization-failure response framing;
- success-response write;
- repeated ingress/connection loop;
- `run()` or `main.rs` mutation;
- listener bind;
- readiness publication;
- candidate publication;
- traversal;
- dialing;
- retry/reconnect/rebind/rebootstrap loops;
- service/systemd/package mutation;
- credential/certificate/private-key/trust/RBAC mutation;
- registry mutation;
- database/schema/control-plane mutation;
- deployment/restart/recovery activation;
- repository visibility/configuration mutation;
- merge;
- PR close;
- ready-for-review conversion;
- branch deletion;
- history rewrite.

## 22. Security boundary preserved

KD preserves:

- no request-selected terminal executable/argv/env/cwd;
- no request-selected host filesystem root;
- no implicit PRW identity-to-Linux-account mapping;
- no setuid/setgid/sudo/su/pkexec/ambient privilege assumptions;
- no arbitrary public/LAN forwarding bind;
- no DNS/hostname widening inside exact-target primitives;
- no firewall/route/TUN/TAP expansion;
- no arbitrary socket-option bags;
- no detached terminal/forwarding workers;
- no cross-principal terminal/forwarding ID reuse;
- no interpretation of `Drop` as proof cleanup completed.

## 23. Selected production population result

The selected future population path is exactly:

```text
fixed production durable-registry systemd custody
    -> validated opaque provider config
    -> existing one-shot production provider bootstrap
    -> DurableRegistryEtcdStore
    -> ProductionDurableRegistryRuntimeCustody::from_store(...)
    -> ProductionDurableCapabilityAuthority::from_registry_custody(...)
    -> dormant ProductionDurableCapabilityAuthority
```

No registry semantic operation occurs during this composition.

No authorization occurs during this composition.

No positive capability is enabled.

## 24. Successor rule

After this selection checkpoint closes, the immediate successor may materialize only the one-file
population composition selected above.

After that source-materialization checkpoint closes: **STOP again** for a fresh exact-head audit before
any aggregate replacement, executable caller, session-auth population, transport/lease/time provenance,
dispatch/response integration, repeated ingress or runtime/network activation.

## 25. Gate and closure

Gate after exact-final-head validation and immutable evidence:

`C03E_KD_PRODUCTION_DURABLE_CAPABILITY_AUTHORITY_POPULATION_COMPOSITION_SELECTED`

Closure after exact-final-head validation and immutable evidence:

`CLOSED_PRODUCTION_DURABLE_CAPABILITY_AUTHORITY_POPULATION_COMPOSITION_SELECTION`
