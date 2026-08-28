# Phase 152 C03e-DI — Candidate Publication Requester/Rendezvous Post-Registry-Validation Provenance Carrier Source Materialization — STAGING

## Status

`STAGED SOURCE MATERIALIZATION`

## Target gate

`C03E_DI_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_POST_REGISTRY_VALIDATION_PROVENANCE_CARRIER_SOURCE_MATERIALIZED`

## Exact predecessor

C03e-DI is rooted only at durably closed C03e-DH:

- branch: `phase-152-c03e-dh-candidate-publication-requester-rendezvous-post-registry-validation-provenance-carrier-ownership-transfer-selection-staging`
- head: `1ca6d70c4d4ca02a45d67637e7c83f7a298aa53f`
- tree: `94ce637bbfeed3b9633fb38d2240cdee4f379ca8`
- closed-DH rolling Drive image: `979157` bytes
- closed-DH rolling SHA-256: `6630313f5bb06d43ff60010e335f478948d08bb38655e8e0536d1585ab464ce1`

DI materializes only the post-registry-validation provenance carrier selected by DG and the ownership-transfer semantics selected by DH.

## Materialized source surface

DI modifies only:

`crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_registry_validation.rs`

The start-intent parent module, provider implementation, policy model, manifests, lockfiles, workflows and runtime surfaces remain unchanged.

## Materialized carrier

DI materializes:

```text
RegistryValidatedRequesterRendezvousStart
```

The carrier owns exactly:

- the `AuthenticatedDeviceSession` that was already present in the consumed requester start intent and passed authoritative current-session validation;
- the exact logical target `DeviceId` that was already present in that same intent and passed all current target/workspace checks.

The carrier is deliberately neither `Copy` nor `Clone`.

It has no constructor from arbitrary identity values.

It exposes only borrowed read accessors for the exact carried requester session and target logical device.

Those accessors do not transfer ownership and do not create provider authority.

## Materialized validation entry point

The existing full registry-validation function now consumes the unvalidated start intent by value and returns the validated carrier on success:

```text
validate_current_requester_rendezvous_start_intent(
    &WorkspaceDeviceRegistry,
    RequesterRendezvousStartIntent,
) -> Result<
    RegistryValidatedRequesterRendezvousStart,
    RequesterRendezvousStartRegistryValidationError,
>
```

No parallel raw-pair constructor or bypass entry point is added.

## Validation-before-transfer invariant

DI preserves the DF validation order and error meanings.

The function:

1. borrows the exact requester session from the consumed intent;
2. validates requester-session currentness through `WorkspaceDeviceRegistry::validate_authenticated_session`;
3. borrows the exact nominated target `DeviceId` from the same intent;
4. performs the existing target lookup;
5. requires `DeviceLifecycle::Enrolled`;
6. resolves the target workspace/user membership;
7. requires active membership;
8. requires requester and target workspace equality;
9. requires exact target `DeviceId` preservation;
10. only after all checks succeed, moves the exact owned requester session and target device ID from that consumed intent into the validated carrier.

No validated carrier is produced before the full registry chain succeeds.

## Failure invariant

On any existing registry-validation failure:

- the existing `RequesterRendezvousStartRegistryValidationError` is returned;
- no validated carrier is returned;
- no policy decision is produced;
- no requester/rendezvous provider mutation occurs;
- no retry/deduplication state is created;
- no network/runtime side effect occurs.

The consumed unvalidated intent is dropped normally by Rust ownership semantics.

## Existing error semantics preserved

DI does not add or reinterpret validation failure authority.

The error surface remains:

- `Registry(RegistryError)`;
- `WorkspaceMismatch`;
- `TargetIdentityMismatch`.

Existing registry meanings remain preserved for:

- unknown target;
- target not enrolled;
- revoked target;
- missing target membership;
- suspended/non-active target membership;
- removed target membership.

## No-clone provenance

DI crosses the validation boundary by direct ownership transfer from the exact consumed input object.

It does not require `Clone` on:

- `RequesterRendezvousStartIntent`;
- `RegistryValidatedRequesterRendezvousStart`;
- `AuthenticatedDeviceSession` for this transfer;
- target `DeviceId` for this transfer.

There is no broad public `into_parts()` or mutable identity accessor.

## Test posture

DI updates the compile-time validation function-shape assertion to require:

- owned `RequesterRendezvousStartIntent` input;
- validated carrier success return.

The existing current-target tests remain unchanged and continue to cover:

- current enrolled active same-workspace target success;
- unknown target failure;
- revoked target failure;
- suspended target membership failure before workspace comparison;
- removed target membership failure;
- active cross-workspace target failure.

DI does not fabricate an authenticated-session cryptographic fixture merely to construct the success carrier in a runtime test.

No dependency is added for testing.

## Carrier authority boundary

Possession of `RegistryValidatedRequesterRendezvousStart` proves only successful point-in-time current-registry validation of the carried requester/target pair.

It does not prove or grant:

- policy authorization;
- requester/rendezvous provider registration authority;
- target transport readiness;
- live-owner authority;
- candidate-publication authority;
- network reachability;
- a TTL/lease;
- perpetual currentness;
- cancellation/retirement authority.

A future consumer must still satisfy every separately selected authority gate before any provider mutation.

## Logical/transport identity separation

Requester logical identity remains inside the authenticated server-held PRW session.

Target logical identity remains the exact validated `DeviceId`.

`TransportIdentity` remains lower-transport certificate identity only.

The carrier contains no:

- `TransportIdentity`;
- endpoint;
- socket;
- candidate ID;
- request ID;
- candidate-publication payload;
- live-owner grant/fence;
- provider record/lifecycle flag;
- freshness/TTL value.

None of those values can substitute for requester or target logical identity.

## Policy remains separate

DI does not:

- add or reuse a `prw-policy::Capability`;
- select a requester-rendezvous-start policy capability;
- evaluate `PolicyEvaluator`;
- embed policy state or a policy decision in the validated carrier.

Any required policy authorization remains a later independent checkpoint.

## Provider remains separate

DI does not call, expose or forward:

- `InMemoryRequesterRendezvousAuthorityProvider::register_current`;
- `retire`;
- `remove_retired`;
- `authorize_current_for_publisher`;
- provider getters;
- provider extraction;
- mutable provider references.

The carrier is not directly converted into a provider record by DI.

## Wire/runtime/networking remain separate

DI adds no:

- `BridgeCommand` variant;
- opcode;
- frame/codec/parser;
- dispatcher;
- listener;
- command loop;
- task;
- synchronization primitive;
- shared-worker topology;
- persistence/database/distributed coordination;
- production networking;
- deployment/restart/recovery;
- merge.

## Source lineage

DI source-materialization lineage begins from exact closed DH and contains only the validation-source materialization/canonical formatting plus this staging contract.

Candidate source commits before final closure:

1. `29680b252c02b6ab28ed9303748911f1da425c29` — materialize validated rendezvous provenance carrier and consuming validation;
2. `2bc56c6d64aa2480d1385e61387fb7895abd2320` — canonical formatter-sensitive function-signature layout only.

Current validation source blob after the canonical layout correction:

`48eeaa3a729f93e3ed6571653a03d5f985b46e30`

## Dependency and lock guards

DI authorizes no manifest or lockfile change.

Expected exact blobs remain:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- Android native `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

## Closure requirements

DI may close only if:

1. closed DH remains the exact predecessor and merge base;
2. final DH -> DI net diff is restricted to the exact validation source plus this DI contract;
3. no start-intent parent-module, provider, policy, manifest, lockfile or workflow drift exists;
4. the materialized carrier remains owned/non-Clone and contains only the exact validated session/target pair;
5. the full existing DF validation chain executes before ownership transfer;
6. no broad extraction or arbitrary carrier-construction API exists;
7. no policy capability/evaluation is added;
8. no provider mutation/access is added;
9. no wire/runtime/networking/deployment behavior is added;
10. all automatically triggered canonical validations on the exact final head reach terminal non-failing verdicts;
11. Android validation is required if triggered by the source path and must be classified from its actual terminal result;
12. manifest and lock guards remain exact;
13. an immutable DI audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-read back byte-exact;
14. rolling Drive evidence is freshly guarded against exact closed DH (`979157` bytes / `6630313f5bb06d43ff60010e335f478948d08bb38655e8e0536d1585ab464ce1`);
15. the DI closure record is appended only to those exact predecessor bytes;
16. the complete closed-DH rolling prefix remains byte-for-byte unchanged;
17. rolling Drive readback matches the intended bytes/hash exactly;
18. only after durable Drive proof may PR status move `STAGED -> CLOSED`;
19. PR remains draft/open/unmerged;
20. final GitHub/Drive race checks remain exact.

## Safe successor rule

After durable DI closure, begin with a fresh exact-head read-only audit.

Do not automatically bundle the next independent seams. Candidate later seams include policy authorization selection/materialization and, only after all required gates exist, provider mutation forwarding.

No direct jump is authorized to synchronization, runtime/listener activation, production networking, deployment, or merge.
