# Phase 152 C03e-DK — Candidate Publication Requester/Rendezvous Start Policy Authorization Source Materialization — STAGING

## Status

`STAGED SOURCE MATERIALIZATION`

## Target gate

`C03E_DK_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_START_POLICY_AUTHORIZATION_SOURCE_MATERIALIZED`

## Exact predecessor

C03e-DK is rooted only at durably closed C03e-DJ:

- branch: `phase-152-c03e-dj-candidate-publication-requester-rendezvous-start-policy-authorization-boundary-selection-staging`;
- head: `f014eeed4c2944bbddfc8f45c8f09b25e17b6c4e`;
- tree: `a78e8459e89813ef743c9a2b7b8ff2f5cbdcfc81`;
- PR #233: `Status: CLOSED`, draft/open/unmerged;
- rolling Drive image: `992124` bytes;
- rolling SHA-256: `8696feee1111a1f421246705513be71ef7b218690baaf303c5d28defa561a339`.

No earlier checkpoint is reopened.

## Checkpoint classification

C03e-DK is a narrow source-materialization checkpoint.

It materializes only:

1. the C03e-DJ-selected dedicated requester/rendezvous-start policy capability;
2. fail-closed handling of that capability by existing bounded local policies;
3. one Agent-internal typed policy-admission carrier and admission function operating only on the C03e-DI registry-validated provenance carrier;
4. private child-module registration for that admission source;
5. this staging contract.

It does not select or materialize provider mutation, provider access, a runtime policy source, policy persistence, wire commands, listener/runtime activation, networking, deployment, or merge.

## Materialized dedicated capability

`crates/prw-policy/src/lib.rs` now represents:

`Capability::RequesterRendezvousStart`

Its meaning is only the policy question whether the already-authenticated requester may begin requester-side rendezvous toward one logical target that has already passed the C03e-DI registry-validation boundary.

The new capability is deliberately distinct from:

- `ForwardingCreate`;
- `DeviceManage`;
- `PolicyManage`;
- terminal capabilities;
- file capabilities;
- local read capabilities.

No existing broader capability is reused as a substitute.

## Existing bounded policy fail-closed rule

Representing `RequesterRendezvousStart` does not grant it.

DK updates both existing bounded local policy evaluators so the new capability is explicitly denied:

- `BoundedLocalReadPolicy` denies `RequesterRendezvousStart`;
- `BoundedLocalManagementPolicy` denies `RequesterRendezvousStart`.

No new decision field is added to either bounded policy.

No `allow_all` path is introduced.

The existing Linux bootstrap continues to construct `BoundedLocalReadPolicy::allow_local_reads()`. Because that evaluator now explicitly denies `RequesterRendezvousStart`, DK does not silently activate requester/rendezvous-start policy in the production/local runtime.

DK selects no production evaluator that allows the new capability.

## Materialized Agent-internal policy admission

DK adds:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_admission.rs`

and registers it only as a child of the existing crate-private requester/rendezvous start-intent module.

The module materializes one owned type:

`PolicyAuthorizedRequesterRendezvousStart`

The type owns exactly one private nested value:

`RegistryValidatedRequesterRendezvousStart`

It does not duplicate or separately store requester/target identity fields.

It is deliberately neither `Copy` nor `Clone`.

It has no public or crate-wide constructor from arbitrary `AuthenticatedDeviceSession`, `DeviceId`, registry record, capability, decision, transport identity, endpoint, request ID, candidate state, live-owner state, or provider record.

## Nested provenance rule

Policy authorization preserves the C03e-DI registry-validation fact as nested provenance.

The policy-admitted carrier does not extract the DI carrier into raw identity values before authorization.

Its read-only accessor exposes only a borrowed reference to the nested `RegistryValidatedRequesterRendezvousStart`.

No `into_parts`, mutable accessor, raw-session accessor, raw-target extraction API, provider conversion, or provider reference is added.

The requester identity therefore remains the exact server-held `AuthenticatedDeviceSession` already carried by DI.

The target identity remains the exact logical `DeviceId` already validated by DI.

`TransportIdentity` remains lower-transport certificate identity only and is not policy authority for this seam.

## Materialized admission function

DK materializes the effective shape:

```text
policy_authorize_requester_rendezvous_start(
    RegistryValidatedRequesterRendezvousStart,
    &impl PolicyEvaluator,
) -> Result<
    PolicyAuthorizedRequesterRendezvousStart,
    RequesterRendezvousStartPolicyAuthorizationError,
>
```

The registry-validated carrier is consumed by value.

The evaluator is borrowed.

The function evaluates exactly:

`Capability::RequesterRendezvousStart`

and exactly one decision controls the transition:

- `Decision::Allow` -> return the typed policy-authorized carrier containing the exact consumed DI carrier;
- `Decision::Deny` -> return `RequesterRendezvousStartPolicyAuthorizationError::Denied` and produce no policy-authorized carrier.

No second capability, fallback capability, implicit allow, or capability widening exists.

## Evaluator-binding boundary

The `PolicyEvaluator` interface is principal-agnostic.

Therefore DK explicitly preserves the DJ rule that the caller must already have selected/bound the supplied evaluator for the same authenticated requester principal represented inside the DI carrier.

DK does not:

- discover an evaluator;
- bind an evaluator to a user/device/session;
- select a workspace policy source;
- load policy from disk/database/network;
- derive policy from target identity;
- derive policy from transport identity;
- infer policy from provider state.

Passing an arbitrary evaluator remains outside this source seam and does not authenticate anyone.

A later composition checkpoint must prove evaluator provenance before using this admission function in a runtime path.

## Denial rule

Policy denial is fail-closed.

On `Decision::Deny`:

- no `PolicyAuthorizedRequesterRendezvousStart` exists;
- the consumed DI carrier is dropped normally;
- no provider registration is attempted;
- no provider state is changed;
- no retry token is created;
- no request ID becomes authority;
- no live-owner state is created;
- no transport or endpoint state is consulted;
- no I/O occurs.

The bounded error contains no requester/session/target secret or transport metadata.

## Test posture

DK source tests prove the selected seam without fabricating an authenticated-session fixture.

The Agent admission module tests:

- compile-time function shape: owned DI carrier + borrowed evaluator -> typed policy-authorized carrier or bounded denial;
- exact dedicated capability evaluation through a test evaluator that asserts `Capability::RequesterRendezvousStart`;
- exactly one evaluator invocation for the decision helper;
- bounded denial error text.

The `prw-policy` tests additionally prove:

- `RequesterRendezvousStart` is distinct from `ForwardingCreate`;
- `RequesterRendezvousStart` is distinct from `DeviceManage`;
- `BoundedLocalReadPolicy` denies the new capability;
- `BoundedLocalManagementPolicy` denies the new capability even when all currently represented management decisions are configured `Allow`;
- `deny_all` also denies it.

No new dependency or test-only cryptographic fixture is added.

## Registry-validation boundary remains intact

DK does not modify the C03e-DI validation source.

The exact DI carrier is still created only after:

1. requester authenticated-session currentness validation;
2. exact target lookup;
3. enrolled target lifecycle;
4. active target membership;
5. requester/target same-workspace equality;
6. exact nominated target `DeviceId` preservation.

Policy admission cannot construct or replace that registry-validation fact.

A raw `RequesterRendezvousStartIntent` cannot enter DK policy admission directly.

## Provider separation

DK does not call, expose, wrap, forward, or compose:

- `InMemoryRequesterRendezvousAuthorityProvider::register_current`;
- `retire`;
- `remove_retired`;
- `authorize_current_for_publisher`;
- provider getters;
- provider extraction;
- raw/mutable provider references.

`PolicyAuthorizedRequesterRendezvousStart` is still not provider registration authority by itself.

A later checkpoint must separately select how all prerequisite authority reaches any provider mutation boundary.

## No publisher-side authority inversion

Publisher-side candidate-publication state remains insufficient to create either the DI registry-validated carrier or the DK policy-authorized carrier.

The following cannot satisfy the requester policy gate:

- publisher authenticated session;
- publisher `TransportIdentity`;
- candidate-publication request ID;
- candidate ID/path kind;
- endpoint/socket observation;
- candidate freshness;
- live-owner grant/fence/currentness;
- successful candidate commit.

The requester path remains the sole provenance origin for this policy admission seam.

## No wire or command materialization

DK adds no:

- `BridgeCommand` variant;
- PRWC/PRWM operation code;
- frame/control-message kind;
- codec/parser;
- dispatcher branch;
- request/response mapping;
- retry/deduplication semantics.

No request ID is added to the policy-authorized carrier.

## No synchronization/runtime topology materialization

DK adds no:

- `Arc`;
- `Mutex`;
- `RwLock`;
- channel;
- actor/mailbox;
- global/singleton;
- task/thread-local authority;
- command-loop ownership;
- listener ownership;
- shared-worker provider topology;
- Agent binary/main wiring.

The new admission module is source-only and has no runtime invocation path selected by DK.

## No persistence/networking/deployment materialization

DK adds no:

- policy database/schema/table;
- requester/rendezvous database/schema/table;
- journal/snapshot;
- durable queue/broker;
- cross-process replication;
- endpoint discovery;
- STUN/ICE/TURN/relay activation;
- firewall/NAT/route/DNS/TUN/TAP changes;
- production listener changes;
- deployment/restart/recovery;
- merge.

## Source surface before contract commit

From exact closed DJ to the source-materialized pre-contract DK head, the net source surface is exactly three paths:

1. `crates/prw-policy/src/lib.rs`;
2. `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs`;
3. `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_policy_admission.rs`.

Pre-contract source head:

- commit: `c5dd322cf3815309f5c5852c6b27635db84de69b`;
- tree: `cf703524d760daf50ba13b5e7618d7ceb548e97f`.

Pre-contract DJ -> DK source compare:

- ahead: `4`;
- behind: `0`;
- exact merge base: closed DJ;
- parent-module registration: `+7/-0`;
- new policy-admission module: `+158/-0`;
- policy model: `+24/-4`.

## Exact source blobs before contract commit

- new policy-admission module: `a40c322d161765b0d2f505cdfd400da6259edde0`;
- requester/rendezvous parent module: `04a42dfd71a4fe838bfc4f7bbb933dae383ac710`;
- `prw-policy` model: `3745024b5b222fcb36244222fad3c9c05a59cece`;
- unchanged DI registry-validation source: `1c021bc95a3d674722bfd70559156fa75e07e578`.

## Dependency and lock guard

DK requires no dependency or lockfile mutation.

Expected unchanged blobs remain:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`;
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`;
- Android native `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`.

No workflow file is authorized to change.

## Closure requirements

DK may close only if:

1. closed DJ remains the exact predecessor and merge base;
2. final DJ -> DK net diff is restricted to the three selected source paths plus this contract;
3. no manifest, lockfile, workflow, provider, wire, binary, Android, desktop, packaging, database, networking, or deployment path drifts;
4. the dedicated capability remains distinct and existing bounded policies deny it fail-closed;
5. the policy-admitted carrier remains owned, nested-provenance-only, non-`Copy`, non-`Clone`, and without arbitrary constructor/extraction API;
6. policy admission consumes only the DI carrier and evaluates exactly `Capability::RequesterRendezvousStart`;
7. denial produces no admitted carrier or side effect;
8. evaluator selection/binding remains outside DK runtime materialization;
9. provider mutation/access remains absent;
10. all automatically triggered canonical validations on the exact final head reach terminal non-failing verdicts;
11. Android validation, if triggered by the source diff, must reach a terminal non-failing verdict and be classified from its actual result;
12. unchanged manifest/lock blobs remain exact;
13. an immutable DK audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-read back byte-exact;
14. rolling Drive evidence is freshly guarded against exact closed DJ (`992124` bytes / `8696feee1111a1f421246705513be71ef7b218690baaf303c5d28defa561a339`);
15. the DK closure record is appended only to those exact predecessor bytes;
16. the complete closed-DJ rolling prefix remains byte-for-byte unchanged;
17. rolling Drive readback matches intended predecessor+suffix bytes/hash exactly;
18. only after durable Drive proof may PR status move `STAGED -> CLOSED`;
19. PR remains draft/open/unmerged;
20. final GitHub/Drive race checks remain exact.

## Safe successor rule

After durable DK closure, begin with a fresh exact-head read-only audit.

The next independent seam must not be assumed automatically.

Potential later seams include:

- composition of the DK policy-authorized provenance with requester/rendezvous provider mutation;
- evaluator provenance/runtime selection if separately required;
- registration retirement/cancellation source selection;
- wire/command selection.

No direct jump is authorized to provider mutation, synchronization, runtime/listener activation, production networking, deployment, or merge without its own selected checkpoint and evidence chain.
