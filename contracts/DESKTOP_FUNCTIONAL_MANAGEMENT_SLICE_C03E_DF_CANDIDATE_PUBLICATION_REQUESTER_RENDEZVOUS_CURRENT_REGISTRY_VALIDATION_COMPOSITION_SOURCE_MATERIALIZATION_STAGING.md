# Phase 152 C03e-DF — Candidate Publication Requester/Rendezvous Current Registry Validation Composition Source Materialization — STAGING

## Status

`STAGED SOURCE MATERIALIZATION`

## Target gate

`C03E_DF_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_CURRENT_REGISTRY_VALIDATION_COMPOSITION_SOURCE_MATERIALIZED`

## Exact predecessor

C03e-DF is rooted only at durably closed C03e-DE:

- branch: `phase-152-c03e-de-candidate-publication-requester-rendezvous-current-registry-validation-composition-selection-staging`;
- head: `95995c736919740b7820998aaec2c834664e1751`;
- tree: `5e6414df1c4a68fbbd6a4c9d767d13baa72b48e9`;
- PR #228: `Status: CLOSED`, draft/open/unmerged;
- gate: `C03E_DE_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_CURRENT_REGISTRY_VALIDATION_COMPOSITION_SELECTED`.

Expected rolling predecessor after closed DE:

- Drive ID `1ZSHwAkU_JwjLG6jQRnkee66lDGlBngx6`;
- size `956008` bytes;
- SHA-256 `edd1431ff43c9c52ce88d0b6881bc27004e9d4f36f94f1b069067d39f95a2617`.

No earlier checkpoint is reopened.

## Checkpoint classification

C03e-DF is a narrowly scoped source-materialization checkpoint.

It materializes only the C03e-DE-selected fail-closed current-registry validation composition for the already-materialized DD `RequesterRendezvousStartIntent`, plus the minimum effective crate-internal submodule registration and this contract.

DF does not materialize a post-validation registration-fact carrier, select/evaluate policy, mutate requester/rendezvous provider state, add wire/command handling, select retirement, add synchronization, activate runtime/networking, deploy, or merge.

## Authorized source surface

The authorized DF source surface is exactly:

1. `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent_registry_validation.rs`;
2. `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs` only for validation-submodule registration;
3. this DF contract.

No other source, manifest, lockfile, workflow, Android, desktop, packaging, database, networking, deployment, or unrelated path is authorized.

## Materialized validation entry point

DF materializes one effective crate-internal function:

`validate_current_requester_rendezvous_start_intent(&WorkspaceDeviceRegistry, &RequesterRendezvousStartIntent) -> Result<(), RequesterRendezvousStartRegistryValidationError>`

Both inputs are borrowed.

The function does not consume, clone, replace, or mutate the DD intent and does not mutate registry state.

Successful return proves only the DE-selected current-registry eligibility observations at that composition point.

## Fixed validation order

The materialized function preserves the exact DE-selected order:

1. requester session currentness through `WorkspaceDeviceRegistry::validate_authenticated_session(intent.requester_session())`;
2. exact nominated target lookup through `WorkspaceDeviceRegistry::device(intent.target_device_id())`;
3. target device lifecycle check;
4. target membership lookup using the target binding's exact workspace/user tuple;
5. target membership lifecycle check;
6. same-workspace equality;
7. exact target `DeviceId` preservation.

No later check runs after an earlier failure.

## Requester currentness materialization

Requester currentness is delegated directly to the existing registry authority:

`validate_authenticated_session(intent.requester_session())`

DF does not reconstruct a requester from IDs, accept requester identity from payload data, or duplicate the registry session-binding algorithm.

The returned `RegistryValidatedPrincipal` is used only to obtain the authoritative requester workspace for the subsequent same-workspace comparison.

DF does not treat the principal as a capability or policy grant.

## Target lookup materialization

The target is resolved only through:

`registry.device(intent.target_device_id())`

Unknown target lookup maps to the existing `RegistryError::DeviceUnknown` meaning through the DF crate-internal composition error.

No transport identity, endpoint, public-key bytes, user ID, request ID, candidate, provider record, or alternate `DeviceId` lookup is added.

## Target lifecycle materialization

DF observes `target.binding().lifecycle` and preserves existing registry failure semantics:

- `DeviceLifecycle::Enrolled` proceeds;
- `DeviceLifecycle::PendingEnrollment` fails as `RegistryError::DeviceNotEnrolled`;
- `DeviceLifecycle::Revoked` fails as `RegistryError::DeviceRevoked`.

No lifecycle mutation or re-enrollment occurs.

## Target membership materialization

DF resolves membership only from the target binding's authoritative tuple:

`registry.membership(&binding.workspace_id, &binding.user_id)`

Existing membership semantics are preserved:

- missing membership -> `RegistryError::MembershipUnknown`;
- `MembershipLifecycle::Active` proceeds;
- `MembershipLifecycle::Suspended` -> `RegistryError::MembershipNotActive`;
- `MembershipLifecycle::Removed` -> `RegistryError::MembershipRemoved`.

This check occurs before same-workspace comparison exactly as selected by DE.

## Same-workspace materialization

After requester and target currentness succeed, DF compares:

`requester.workspace_id()` with `target.binding().workspace_id`.

Mismatch fails with the crate-internal `WorkspaceMismatch` composition error before any policy or provider operation.

No workspace identity is supplied by payload or inferred from transport/provider state.

## Exact target preservation materialization

DF explicitly verifies:

`target.binding().device_id == *intent.target_device_id()`

Although the current registry map is keyed by `DeviceId`, the explicit check preserves the DE provenance invariant and prevents future composition from silently treating a different logical target as equivalent.

Structural mismatch fails with crate-internal `TargetIdentityMismatch`.

## Materialized error surface

DF materializes only one effective crate-internal error enum:

`RequesterRendezvousStartRegistryValidationError`

Variants:

- `Registry(RegistryError)`;
- `WorkspaceMismatch`;
- `TargetIdentityMismatch`.

The enum preserves existing `RegistryError` values rather than replacing their meanings.

It adds no wire/status/response/error-code mapping and no public cross-crate API.

## Module visibility boundary

The validation source is registered as a submodule of the already effective crate-internal DD start-intent module using an explicit source path.

The registration is `pub` only inside the enclosing `pub(crate)` start-intent module. Effective visibility therefore remains bounded to `prw-agent` and avoids the canonical `clippy::redundant_pub_crate` condition previously observed in DD.

No crate-root public module or re-export is added.

## Test boundary

DF does not fabricate an `AuthenticatedDeviceSession` merely to unit-test an already-tested registry currentness primitive.

The top-level validation function's exact borrowed signature is checked at compile time.

The newly composed target-currentness seam is tested directly with typed disposable in-memory registry state requiring no new dependency:

- current Enrolled target with Active same-workspace membership passes;
- unknown target fails `DeviceUnknown`;
- revoked target fails `DeviceRevoked`;
- suspended target membership fails `MembershipNotActive` before workspace comparison;
- removed target membership fails `MembershipRemoved`;
- active cross-workspace target fails `WorkspaceMismatch`.

Existing `WorkspaceDeviceRegistry` tests remain authoritative for `validate_authenticated_session` session binding semantics.

No crypto fixture, network fixture, provider fixture, runtime fixture, or dependency addition is introduced.

## Policy separation

DF changes no `prw-policy` source, imports no policy evaluator, selects no capability, and performs no policy decision.

The current policy surface still contains no requester-rendezvous-start/reachability-start capability.

DF does not reinterpret `DeviceManage`, `ForwardingCreate`, or another existing capability.

Registry-validation success remains insufficient for provider registration until any separately required policy authorization is selected and succeeds.

## Post-validation fact separation

DF returns `Result<(), ...>` only.

It does not materialize a post-validation authority/fact carrier and does not rename the DD unvalidated intent into an authorized registration fact.

A later checkpoint must independently decide how DE/DF registry success is preserved across any required policy gate and into eventual provider mutation.

DF therefore does not add a consuming accessor, `into_parts`, clone/replay behavior, or implicit promotion of the DD carrier.

## Provider isolation

DF does not expose, forward, call, or wrap:

- `register_current`;
- `authorize_current_for_publisher`;
- `retire`;
- `remove_retired`;
- a mutable/raw provider reference;
- provider extraction.

The existing Agent requester/rendezvous runtime owner remains unchanged.

## Transport identity separation

DF does not inspect target `TransportIdentity`, does not call `validate_transport_identity`, and does not require target transport binding/readiness.

Transport identity remains lower-transport identity only and cannot replace logical target `DeviceId` authority.

## Wire/command isolation

DF adds no `BridgeCommand`, PRWC/PRWM opcode/control kind, frame variant, codec, response mapping, request ID, command-loop branch, or requester-visible request representation.

The validation function is not a wire handler.

## Retirement/cleanup isolation

DF does not select or materialize cancellation, requester retirement, target-revocation cleanup, disconnect handling, timeout/TTL, completion cleanup, shutdown cleanup, retired-record removal, or background maintenance.

## Synchronization/topology isolation

DF adds no `Arc`, `Mutex`, `RwLock`, channel, actor/mailbox, static/global/singleton state, worker topology, task/thread ownership, command-loop ownership, or listener ownership.

The validation is synchronous and read-only over a caller-supplied registry reference.

## Persistence/distributed isolation

DF adds no database/schema, migration, journal/snapshot, durable queue, broker, replication, lease, heartbeat, TTL, clock dependency, distributed requester/rendezvous authority, or external service dependency.

## Runtime/networking isolation

DF does not start or wire an Agent runtime path, accept/connect loop, listener, socket operation, STUN/ICE/TURN/relay behavior, NAT/firewall/route/DNS/TUN/TAP mutation, readiness publication, systemd/host mutation, deployment, restart, or recovery behavior.

## Dependency and lock guard

DF requires no dependency or lockfile mutation.

Required unchanged blobs:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`;
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`;
- Android native `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`.

Any drift invalidates DF closure until separately audited.

## Explicit non-selections

C03e-DF does not select, materialize, or authorize:

- a post-validation registration-fact carrier;
- policy capability/evaluator composition;
- provider mutation forwarding;
- provider extraction/public access;
- provider capacity changes;
- wire command/opcode/frame/codec;
- retry/deduplication semantics;
- retirement/cancellation/cleanup source;
- synchronization/shared-worker topology;
- target transport readiness;
- candidate publication;
- command-loop/listener integration;
- Agent binary/main wiring;
- persistence/database/distributed coordination;
- production networking;
- deployment/restart/recovery;
- merge.

## Closure requirements

DF may close only if:

1. C03e-DE remains the exact merge base and predecessor;
2. final DE -> DF diff is restricted to exactly the validation source, minimum DD carrier submodule registration, and this DF contract;
3. selected validation order remains byte-for-byte semantically equivalent to DE;
4. target lifecycle/membership errors preserve existing `RegistryError` meanings;
5. no target transport identity is promoted to logical authority;
6. no policy selection/evaluation is added;
7. no provider mutation/access is added;
8. no manifest/lockfile drift occurs;
9. all automatically triggered canonical validations on the exact final head reach terminal non-failing verdicts;
10. an immutable DF audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-read back byte-exact;
11. rolling Drive evidence is freshly guarded against exact closed DE;
12. the DF closure record is appended only to those exact predecessor bytes;
13. the entire closed-DE rolling prefix is preserved byte-for-byte;
14. rolling Drive update raw-readback matches intended bytes/hash exactly;
15. only after durable Drive proof may the PR body move `STAGED -> CLOSED`;
16. the PR remains draft/open/unmerged;
17. final GitHub/Drive race checks remain exact.

## Safe successor rule

After durable DF closure, the next checkpoint must begin with a fresh exact-head read-only audit from closed DF.

Candidate independent seams include:

- selecting/materializing a post-validation registration-fact carrier;
- independently selecting requester-rendezvous-start policy authorization;
- wire/command selection;
- provider mutation forwarding.

Those seams must not be bundled automatically.

No direct jump is authorized to provider mutation, synchronization, runtime/listener activation, production networking, deployment, or merge.
