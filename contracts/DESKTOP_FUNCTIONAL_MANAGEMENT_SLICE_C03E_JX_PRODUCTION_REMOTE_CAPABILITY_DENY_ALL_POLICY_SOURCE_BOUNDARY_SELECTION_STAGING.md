# C03e-JX — Production Remote Capability Deny-All Policy Source Boundary Selection

Status: **SELECTION STAGING — VALIDATION PENDING**

Target gate:

`C03E_JX_PRODUCTION_REMOTE_CAPABILITY_DENY_ALL_POLICY_SOURCE_BOUNDARY_SELECTED`

Intended closure:

`CLOSED_PRODUCTION_REMOTE_CAPABILITY_DENY_ALL_POLICY_SOURCE_BOUNDARY_SELECTION`

## 1. Exact predecessor authority

Predecessor checkpoint:

`C03e-JW — Production Durable Capability Bridge Source Materialization`

Predecessor branch:

`phase-152-c03e-jw-production-durable-capability-bridge-source-materialization`

Exact predecessor head / required merge base:

`3800f84f5af73ed732f49d2837b88ba08bf0a336`

Exact predecessor tree:

`8e70bb6d60c56613973192fd4aa4d013185b5676`

Exact predecessor `crates/prw-remote-bridge/src/lib.rs` blob:

`ad6833cc4e71a372810b260f157126a3df6645e5`

JW is closed with exact-final-head Rust and Android validation plus immutable Drive evidence. JW does not materialize or authorize production policy provenance.

## 2. Fresh post-JW audit result

The exact JW head still contains the policy source at:

`crates/prw-policy/src/lib.rs`

with exact blob:

`3745024b5b222fcb36244222fad3c9c05a59cece`

The exact policy manifest remains:

`crates/prw-policy/Cargo.toml`

with exact blob:

`4c23933e9f82787bd4736f4bd41091e4c168cd91`

No `ProductionRemoteCapabilityDenyAllPolicy` source type exists at the exact JW head.

The current policy crate contains:

- `Capability`;
- `Decision`;
- `PolicyEvaluator`;
- `BoundedLocalReadPolicy`;
- `BoundedLocalManagementPolicy`.

The local policy types are explicitly bounded local configuration primitives and are not promoted by JX into production remote capability-policy provenance.

## 3. Prior mechanism selection retained

Closed C03e-JT already selected the initial production remote policy baseline name and semantics:

`ProductionRemoteCapabilityDenyAllPolicy`

with these properties:

- it is a dedicated production remote policy type;
- it has zero external source/provisioning dependency;
- it implements `PolicyEvaluator`;
- for every represented `Capability`, evaluation returns exactly `Decision::Deny`;
- construction performs no I/O, environment read, provider lookup, role mapping, service mutation, runtime activation or network operation;
- it is not an allow-bearing policy and does not establish authoritative production grants.

JX does not change those semantics.

## 4. Why a source-boundary checkpoint is required

JT selected the conceptual production policy type while its immediate source ceiling was limited to the durable registry validator. Subsequent JU/JV/JW checkpoints separately materialized the durable validator and durable bridge.

After JW closure there is no still-open exact path ceiling authorizing a policy source mutation.

JX therefore selects the smallest post-JW source boundary before any Rust mutation.

## 5. Exact immediate successor path ceiling

After JX closes, the immediate source-materialization successor may change exactly one repository path:

`crates/prw-policy/src/lib.rs`

No other path is selected.

In particular, the immediate successor must not mutate:

- `crates/prw-policy/Cargo.toml`;
- `Cargo.lock`;
- `crates/prw-remote-bridge/*`;
- `crates/prw-registry/*`;
- `crates/prw-agent/*`;
- Android/application code;
- workflow files;
- packaging/service/systemd files;
- `main.rs` or any executable wiring.

## 6. Exact selected source type

The immediate successor may add exactly one production policy type named:

`ProductionRemoteCapabilityDenyAllPolicy`

Selected structural properties:

- zero-sized or equivalently zero-external-state;
- `Debug`;
- `Clone`;
- `Copy`;
- `PartialEq`;
- `Eq`;
- `Send` and `Sync` by ordinary Rust type semantics;
- no interior mutability;
- no provider/client/registry/session/runtime/socket ownership.

A constructor may be `new()` or an equivalent constant/default construction shape only if it cannot select grants or external sources.

No configuration field is selected.

## 7. Exact `PolicyEvaluator` behavior

For every value of the exact current `Capability` enum, including:

- `AgentStatusRead`;
- `PrivateDnsConfigRead`;
- `TerminalOpen`;
- `TerminalExec`;
- `FilesRead`;
- `FilesWrite`;
- `FilesDelete`;
- `ForwardingCreate`;
- `RequesterRendezvousStart`;
- `DeviceManage`;
- `PolicyManage`;

`ProductionRemoteCapabilityDenyAllPolicy::evaluate(...)` must return exactly:

`Decision::Deny`

There is no capability-specific exception in JX.

## 8. No allow-bearing constructor or mutation surface

The immediate successor must not add:

- `allow_all()`;
- `allow_*()` production constructors;
- per-capability grant setters;
- mutable decision fields;
- role-to-capability maps;
- user/workspace/device allow lists;
- environment-controlled grants;
- file/config/database policy load;
- systemd credential parsing;
- remote control-plane policy fetch;
- dynamic reload or watch behavior.

The selected type is intentionally incapable of returning `Decision::Allow`.

## 9. Local policy separation

JX does not reuse or rename:

- `BoundedLocalReadPolicy`;
- `BoundedLocalManagementPolicy`;
- `BoundedLocalManagementDecisions`.

Those types remain local/source policy primitives with their existing semantics.

The production remote deny-all baseline must be a distinct named type so later custody cannot silently inherit local allow-bearing configuration.

## 10. Production provenance meaning

`ProductionRemoteCapabilityDenyAllPolicy` is production-safe provenance only for the statement:

> no remote capability is granted by the initial production baseline.

It is not production provenance for any positive authorization decision.

Successful authentication, durable registry validation, transport validation, PRWC decoding, capability derivation or possession of this policy object does not grant a capability.

## 11. Allow-bearing production policy remains separately gated

Before any production remote capability may become executable through `Decision::Allow`, a fresh checkpoint must select authoritative allow-bearing policy provenance, including at minimum:

- principal scope;
- workspace/user/device binding dimension;
- capability decision source;
- currentness/reload semantics;
- missing/invalid-source behavior;
- custody/lifetime;
- failure semantics;
- privilege and revocation behavior.

JX selects none of those mechanisms.

## 12. Focused immediate-successor test ceiling

The one-file source successor may add same-file tests proving only:

1. every represented `Capability` is denied;
2. construction is deterministic and zero-source;
3. the type is `Copy + Send + Sync` if those assertions are useful;
4. no local allow-bearing policy behavior is inherited.

Tests must require no process-global environment mutation, provider/network I/O, filesystem mutation, service mutation or runtime activation.

## 13. Manifest and dependency invariant

The existing `prw-policy` manifest already contains the dependencies required by the current policy model.

The selected deny-all type requires no new dependency.

Therefore any manifest or lockfile mutation would exceed JX scope.

## 14. Durable bridge relationship

Closed JW materialized `DurableCapabilityBridge<'a, P: PolicyEvaluator + Sync>` as a generic policy consumer.

JX does not wire `ProductionRemoteCapabilityDenyAllPolicy` into that bridge.

Materializing a deny-all type in `prw-policy` does not instantiate a durable bridge, acquire registry custody, authorize a request, dispatch an operation or activate runtime behavior.

## 15. Agent custody remains unresolved

JX does not select or materialize:

`ProductionDurableCapabilityAuthority`

or any Agent-side owner combining durable registry custody with production policy custody.

That remains a later separately gated boundary after the deny-all policy source materialization closes and a new exact-head audit is performed.

## 16. Existing in-memory shared authority remains unchanged

JX does not mutate or reinterpret:

`SharedCurrentCapabilityAuthority<P>`

and does not replace any `LinuxAgentRemoteProcessOperationInputs` field.

The existing in-memory authority path remains distinct from the durable production authority path.

## 17. Identity and authorization invariants

JX preserves:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

Specifically:

- `DeviceId` remains logical device identity;
- `TransportIdentity` remains current transport evidence;
- IP/port remains transient reachability data;
- `SessionId` remains session correlation/lifetime context;
- PRWM `request_id` remains transaction correlation only;
- policy evaluation does not authenticate identity;
- deny-all policy possession does not establish authorization success.

No PID/UID/GID or host account identity becomes PRW logical identity.

## 18. Explicit JX exclusions

JX does not perform or authorize:

- Rust/source materialization in JX;
- any second changed repository path;
- allow-bearing production policy;
- policy provider/database/control-plane creation;
- role mapping;
- policy reload/watch/cache semantics;
- durable registry mutation;
- durable bridge mutation;
- Agent durable authority custody;
- aggregate input replacement;
- session/expected-request/dispatcher/timing/callback production population;
- requester/rendezvous population or invocation;
- operation-factory invocation;
- remote-process companion spawn;
- `run()` or `main.rs` mutation;
- listener/bind/readiness/runtime/network activation;
- candidate publication/traversal/dialing/retry/reconnect/rebind/rebootstrap;
- service/systemd/package/security/credential/certificate/private-key/trust/RBAC mutation;
- database/schema/control-plane mutation;
- deployment/restart/recovery activation;
- repository visibility/configuration mutation;
- merge, PR close, ready-for-review conversion, branch deletion or history rewrite.

## 19. Closure and successor rule

After JX closure: **STOP**.

The immediate successor may only materialize `ProductionRemoteCapabilityDenyAllPolicy` inside:

`crates/prw-policy/src/lib.rs`

After that source materialization closes, a fresh exact-head audit is mandatory before selecting Agent durable capability-authority custody or any broader composition.

No executable/runtime authority is inherited from JX.
