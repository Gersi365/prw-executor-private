# Phase 152 C03e-DE — Candidate Publication Requester/Rendezvous Current Registry Validation Composition Selection — STAGING

## Status

`STAGED SELECTION`

## Target gate

`C03E_DE_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_CURRENT_REGISTRY_VALIDATION_COMPOSITION_SELECTED`

## Exact predecessor

C03e-DE is rooted only at durably closed C03e-DD:

- branch: `phase-152-c03e-dd-candidate-publication-requester-rendezvous-start-intent-semantic-carrier-source-materialization-staging`;
- head: `c131f0a0d630bf6afade81a8c170f5acdc1d5be0`;
- tree: `7da14ccb0255fd9a5142df607a60838db08a33be`;
- PR #227: `Status: CLOSED`, draft/open/unmerged;
- gate: `C03E_DD_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_START_INTENT_SEMANTIC_CARRIER_SOURCE_MATERIALIZED`.

Expected rolling predecessor after closed DD:

- Drive ID `1ZSHwAkU_JwjLG6jQRnkee66lDGlBngx6`;
- size `949800` bytes;
- SHA-256 `541431d05e7e06d9d98298e846ec77df5f4bc7363f6d772b252d39a737d72bbf`.

No earlier checkpoint is reopened.

## Checkpoint classification

C03e-DE is a documentation-only semantic composition-selection checkpoint.

It selects only the exact fail-closed current-registry observations required to turn the DD `RequesterRendezvousStartIntent` from unvalidated requester intent into evidence that the authenticated requester and nominated logical target are currently eligible under the existing `WorkspaceDeviceRegistry` state.

DE does not materialize source code, a validated registration-fact carrier, a policy capability, policy evaluation, provider mutation, wire/command handling, retirement, synchronization, runtime activation, networking, deployment, or merge behavior.

## Exact-head prerequisite audit result

The closed-DD exact source establishes all of the following:

1. `RequesterRendezvousStartIntent` carries exactly one server-held `AuthenticatedDeviceSession` and one requester-nominated target `DeviceId`;
2. possession of that carrier is explicitly non-authoritative and performs no registry or policy validation;
3. `WorkspaceDeviceRegistry::validate_authenticated_session` already provides the authoritative fail-closed requester currentness check and returns a `RegistryValidatedPrincipal` with current workspace identity;
4. `WorkspaceDeviceRegistry::device` can resolve the nominated logical target to the retained `RegisteredDevice`, but retained device presence alone does not prove current eligibility because a retained binding may be revoked;
5. `RegisteredDevice::binding` exposes the current `DeviceIdentityBinding`, whose `workspace_id`, `user_id`, `device_id`, and `lifecycle` are authoritative registry fields;
6. `WorkspaceDeviceRegistry::membership` can resolve the target binding's current workspace/user membership, and `MembershipLifecycle::Active` is the only membership state allowed to participate in current-registry validation;
7. the existing candidate-publication admission path already establishes the architectural rule that requester and publisher must be registry-current and share the same current workspace before reachability mutation;
8. the current `prw-policy::Capability` surface contains no requester-rendezvous-start or reachability-start capability;
9. `Capability::DeviceManage` is an existing device-management capability and is not proven equivalent to requester-side rendezvous-start authorization;
10. the in-memory requester/rendezvous provider already exposes `register_current`, but provider mutation remains downstream of current-registry validation and any separately selected policy decision.

Therefore the narrowest missing seam after DD is current-registry validation composition. Policy selection, validated-fact materialization, and provider forwarding remain independent later gates.

## Selected input boundary

The selected validation composition consumes only:

- the already-materialized `RequesterRendezvousStartIntent`;
- a read-only authoritative `WorkspaceDeviceRegistry` view.

The composition must derive requester identity only from `intent.requester_session()` and target identity only from `intent.target_device_id()`.

It must not accept replacement requester or target identity fields from a wire payload, transport connection, provider record, candidate publication, request ID, endpoint observation, or caller-supplied workspace identifier.

## Selected validation order

The selected fail-closed semantic order is:

1. revalidate the requester session against current registry state;
2. resolve the exact nominated target logical `DeviceId` from current registry state;
3. require the resolved target device binding to remain `DeviceLifecycle::Enrolled`;
4. resolve the target binding's exact current workspace/user membership;
5. require that target membership to remain `MembershipLifecycle::Active`;
6. require the validated requester workspace and target binding workspace to be exactly equal;
7. require the resolved target binding's `device_id` to remain exactly equal to the nominated `intent.target_device_id()`;
8. only after those checks may a later checkpoint derive a post-validation registration fact or proceed to a separately selected policy gate.

No later step may run after an earlier current-registry failure.

## Requester currentness selection

Requester currentness is selected through the existing exact registry API:

`WorkspaceDeviceRegistry::validate_authenticated_session(intent.requester_session())`

A successful result must be treated only as a `RegistryValidatedPrincipal` snapshot proving that, at that registry observation:

- the requester's workspace/user membership exists and is Active;
- the requester's registered device exists and is Enrolled;
- the authenticated session workspace/user/device/public-identity tuple exactly matches current registry identity.

DE does not invent a second requester-currentness algorithm and does not reconstruct requester identity from IDs.

The returned principal contains no capability set and therefore is not policy authorization.

## Target logical lookup selection

The nominated target is selected through the existing exact registry lookup:

`WorkspaceDeviceRegistry::device(intent.target_device_id())`

Absence must fail closed as target unknown.

Successful map lookup alone is insufficient for DE success because the registry intentionally retains revoked device bindings.

No fallback lookup by transport identity, public-key bytes, endpoint, user ID, request ID, candidate ID, or provider record is selected.

## Target device lifecycle selection

After exact target lookup, the composition must observe:

`target.binding().lifecycle`

The only accepted target device lifecycle is:

`DeviceLifecycle::Enrolled`

`PendingEnrollment`, `Revoked`, unknown, or otherwise non-participating target state must fail closed.

DE does not mutate device lifecycle and does not re-enroll or replace a target.

## Target membership currentness selection

The target binding's workspace/user tuple must be revalidated through the existing exact registry membership lookup:

`WorkspaceDeviceRegistry::membership(&target.binding().workspace_id, &target.binding().user_id)`

Absence must fail closed.

The only accepted target membership lifecycle is:

`MembershipLifecycle::Active`

A retained target device whose associated membership is Suspended or Removed is not a current eligible rendezvous target even if its device binding remains present and Enrolled.

DE does not alter membership state and does not infer membership authority from device presence alone.

## Same-workspace selection

After requester currentness and target currentness have independently succeeded, DE requires exact equality between:

- `requester_principal.workspace_id()`;
- `target.binding().workspace_id`.

Cross-workspace requester/target intent must fail closed before any policy evaluation or provider mutation.

No workspace relationship is inferred from transport topology, provider records, endpoints, candidate publication, user equality, or historical enrollment state.

## Exact target preservation selection

The logical target validated by DE must remain the exact target nominated in DD:

`target.binding().device_id == *intent.target_device_id()`

The existing registry is keyed by `DeviceId`, so this equality is structurally expected; DE records it explicitly as a provenance invariant so a later validated-fact carrier or provider call cannot silently retarget the operation.

No alternate publisher device may be substituted after validation.

## Transport identity separation

DE does not require a target `TransportIdentity` to exist and does not call `validate_transport_identity`.

Transport identity is a separately rotatable lower-transport certificate identity and is not requester/rendezvous logical target authority.

A target may be current and eligible in the logical registry without DE selecting transport readiness, endpoint availability, candidate publication, or network reachability.

Any later transport currentness requirement remains downstream and independently gated.

## Public identity separation

DE does not compare or copy target public-key material into requester/rendezvous authority.

The requester's authenticated-session public identity is already checked by `validate_authenticated_session`; target logical eligibility is selected from authoritative registry membership/device state.

No target public-key byte representation becomes record identity or expected-publisher authority.

## Existing candidate-publication precedent

The exact closed-DD source already contains `validate_authenticated_publication_admission`, whose fixed ordering revalidates requester and publisher authenticated sessions, requires same-workspace equality, verifies exact publication target identity, and only then permits downstream target currentness/commit work.

DE reuses the same fail-closed architectural principle—current authoritative identity before same-workspace admission before mutation—but does not call that candidate-publication function because requester-side start intent has no authenticated target session, candidate publication, connectivity plan, transport identity, or candidate set.

No publisher traffic is required to validate requester-side start intent.

## Policy separation

DE changes no `prw-policy` source and selects no capability.

The exact current `Capability` enum contains:

- `AgentStatusRead`;
- `PrivateDnsConfigRead`;
- `TerminalOpen`;
- `TerminalExec`;
- `FilesRead`;
- `FilesWrite`;
- `FilesDelete`;
- `ForwardingCreate`;
- `DeviceManage`;
- `PolicyManage`.

There is no requester-rendezvous-start or reachability-start capability.

DE therefore does not silently reinterpret `DeviceManage`, `ForwardingCreate`, or any other existing capability as requester/rendezvous-start authorization.

A later independent policy checkpoint must decide whether a new capability is required or prove another exact authorization boundary. Until then, registry-validation success is not sufficient to call `register_current`.

## Validation-success posture

DE success means only that the DD intent has passed the selected current-registry observations at one composition point:

- requester session current;
- target device present and Enrolled;
- target membership present and Active;
- requester and target in the same current workspace;
- exact nominated target preserved.

DE success does not mean:

- requester policy authorization;
- requester/rendezvous provider registration;
- expected publisher authority already exists;
- candidate-publication authority;
- transport readiness;
- live-owner authority;
- network reachability;
- runtime activation.

## Failure posture

Every selected validation step is fail closed.

Existing registry failure meanings should be preserved where they already exist, including requester `RegistryError` outcomes and target unknown/inactive/revoked conditions.

DE does not select a new public error enum, wire error code, HTTP/status mapping, PRWC response, retry behavior, or user-visible message.

Cross-workspace and exact-target mismatch remain semantic failures whose concrete source-level error representation is deferred to a later materialization checkpoint.

No validation failure may mutate requester/rendezvous provider state.

## Post-validation fact separation

DE does not materialize the server-side validated registration-fact carrier anticipated by DB.

A later checkpoint may select/materialize a private typed value containing only the authority-bearing post-validation values needed by the existing provider boundary:

- the already-authenticated requester session;
- the exact validated target publisher logical `DeviceId`.

Such a value must not exist before all DE registry validations succeed and must not itself bypass any separately required policy decision.

## Provider mutation separation

DE does not expose, forward, call, wrap, or otherwise authorize:

- `InMemoryRequesterRendezvousAuthorityProvider::register_current`;
- `RequesterRendezvousAuthorityProvider::authorize_current_for_publisher`;
- `retire`;
- `remove_retired`;
- a mutable/raw provider reference;
- provider extraction.

`CandidatePublicationRequesterRendezvousRuntimeOwner` remains unchanged and its provider remains private.

Provider mutation forwarding remains a separate later source-level gate.

## Wire/command isolation

DE adds no `BridgeCommand`, PRWC/PRWM opcode/control kind, frame variant, decoder, encoder, request ID, response mapping, command-loop branch, or requester-visible request representation.

The exact wire/command source for requester-side start intent remains independently gated.

## Retirement and cleanup isolation

DE does not select cancellation, requester retirement, target-revocation cleanup, disconnect handling, timeout/TTL, completion cleanup, shutdown cleanup, retired-record removal, or background maintenance.

Existing DA/DB retirement constraints remain unchanged.

## Synchronization/topology isolation

DE adds no `Arc`, `Mutex`, `RwLock`, channel, actor/mailbox, static/global/singleton state, worker topology, command-loop ownership, task/thread ownership, or listener ownership.

This documentation-only selection does not decide how a future registry view and provider mutation are synchronized across workers or processes.

## Persistence/distributed isolation

DE adds no database/schema, migration, journal/snapshot, durable queue, broker, replication, lease, heartbeat, TTL, clock dependency, distributed requester/rendezvous authority, or external service dependency.

## Runtime/networking isolation

DE does not start or wire an Agent runtime path, accept/connect loop, socket listener, STUN/ICE/TURN/relay behavior, NAT/firewall/route/DNS/TUN/TAP mutation, readiness publication, systemd/host mutation, deployment, restart, or recovery behavior.

## Exact audited source anchors

The selection is grounded in closed-DD exact source including:

- `crates/prw-registry/src/lib.rs` blob `cd7eb52eea19354620ff8ad26c9b8aad3f9c5eb6`;
- `crates/prw-policy/src/lib.rs` blob `c2a02e5640a3274fa7a6d04dacb91d06a8d0df93`;
- `crates/prw-remote-bridge/src/candidate_reachability.rs` blob `51b294cfb3772925651a05bdcb034cd051204efb`;
- `crates/prw-remote-bridge/src/requester_rendezvous_authority.rs` blob `260024b7aca2aea6109dc72e778bcda3dcca8038`;
- `crates/prw-remote-bridge/src/requester_rendezvous_in_memory_provider.rs` blob `d01cfbc37433f6099e216397b9bf243aa55c53bc`;
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs` blob `36688714cddcc76c89523fedd5d5833ca587c4a2`;
- `crates/prw-agent/src/candidate_publication_requester_rendezvous_runtime.rs` blob `04133d3da5fa05a2f14ae91b50d189a9fa6ec1ab`.

DE does not modify any of those source paths.

## Dependency and lock guard

DE requires no dependency or lockfile mutation.

Required unchanged blobs:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`;
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`;
- Android native `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`.

Any drift in those blobs invalidates DE closure until separately audited.

## Explicit non-selections

C03e-DE does not select, materialize, or authorize:

- validation source code;
- a validated registration-fact carrier;
- a new or reused policy capability;
- policy evaluator composition;
- provider registration forwarding;
- provider extraction or public access;
- provider capacity changes;
- wire command/opcode/frame/codec;
- request retry/deduplication semantics;
- retirement/cancellation/cleanup source;
- synchronization/shared-worker topology;
- command-loop integration;
- listener/accept-loop integration;
- authenticated connection ownership changes;
- Agent binary/main wiring;
- target transport readiness;
- candidate publication;
- readiness;
- persistence/database/distributed coordination;
- production networking;
- deployment/restart/recovery;
- merge.

## Closure requirements

DE may close only if:

1. C03e-DD remains the exact merge base and predecessor;
2. final DD -> DE diff is restricted to this one documentation path;
3. no source, manifest, lockfile, workflow, binary, Android, desktop, packaging, database, networking, or unrelated path changes;
4. the selected validation order remains requester-currentness -> exact target lookup -> target Enrolled -> target membership Active -> same-workspace equality -> exact target preservation;
5. no transport identity, public identity bytes, request ID, candidate, endpoint, provider record, or payload-supplied requester identity becomes logical registration authority;
6. no policy capability is selected or implicitly reused;
7. no provider mutation is introduced;
8. required manifest/lock blobs remain exact;
9. all automatically triggered canonical validations on the exact final head reach terminal non-failing verdicts;
10. an immutable DE audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-read back byte-exact;
11. rolling Drive evidence is freshly guarded against exact closed DD;
12. the DE closure record is appended only to those exact predecessor bytes;
13. the entire closed-DD rolling prefix is preserved byte-for-byte;
14. rolling Drive update raw-readback matches intended bytes/hash exactly;
15. only after durable Drive proof may the PR body move `STAGED -> CLOSED`;
16. the PR remains draft/open/unmerged;
17. final GitHub/Drive race checks remain exact.

## Safe successor rule

After durable DE closure, the next checkpoint must begin with a fresh exact-head read-only audit from closed DE.

The immediate successor must again choose only one narrow independent seam. Candidate options include:

- source materialization of the selected registry-validation composition;
- post-validation registration-fact carrier selection/materialization;
- requester-rendezvous-start policy capability selection, if required;
- wire/command selection;
- provider mutation forwarding.

Those options must not be bundled automatically.

No direct jump is authorized to provider mutation, synchronization, command-loop activation, listener activation, production networking, deployment, or merge.
