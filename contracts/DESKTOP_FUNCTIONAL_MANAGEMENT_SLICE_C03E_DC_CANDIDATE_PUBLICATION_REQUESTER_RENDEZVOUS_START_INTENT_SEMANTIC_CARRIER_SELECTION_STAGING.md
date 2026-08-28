# Phase 152 C03e-DC — Candidate Publication Requester/Rendezvous Start Intent Semantic Carrier Selection — STAGING

## Status

`STAGED SELECTION`

## Target gate

`C03E_DC_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_START_INTENT_SEMANTIC_CARRIER_SELECTED`

## Exact predecessor

C03e-DC is rooted only at durably closed C03e-DB:

- branch: `phase-152-c03e-db-candidate-publication-requester-rendezvous-current-registration-source-boundary-selection-staging`
- head: `0d7875c7025f848166741b70a949effb28d6fc75`
- tree: `d3ef1d01ba699adcf49080e7e46b615950496675`
- PR #225: `Status: CLOSED`, draft/open/unmerged
- gate: `C03E_DB_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_CURRENT_REGISTRATION_SOURCE_BOUNDARY_SELECTED`

Expected rolling predecessor after closed DB:

- Drive ID `1ZSHwAkU_JwjLG6jQRnkee66lDGlBngx6`
- size `935624` bytes
- SHA-256 `aa4f0f8918a7a270ebb4d9f865b73f278c20f64e8ec62c9b88754090336129e0`

No earlier checkpoint is reopened.

## Checkpoint classification

C03e-DC is a documentation-only semantic carrier selection checkpoint.

It selects only the narrowest typed representation for requester-side reachability-start intent after DB selected the authoritative source boundary.

It does not materialize source code, perform registry validation, select or execute policy, mutate requester/rendezvous provider state, extend PRWC/PRWM wire protocol, dispatch a command, select retirement, start a task/listener, activate networking, deploy, or merge.

## Exact-head prerequisite audit result

The exact closed-DB source establishes all of the following:

1. `CandidatePublicationRequesterRendezvousRuntimeOwner` owns one `InMemoryRequesterRendezvousAuthorityProvider` by value but exposes no lifecycle operation or raw provider access;
2. `InMemoryRequesterRendezvousAuthorityProvider::register_current` already accepts exactly one owned `AuthenticatedDeviceSession` and one owned expected-publisher `DeviceId`, but provider mutation is downstream authority and cannot originate requester intent;
3. `WorkspaceDeviceRegistry::validate_authenticated_session` can revalidate requester-session currentness and yields current workspace/device identity;
4. `WorkspaceDeviceRegistry::device` exposes current registered-device state by logical `DeviceId`, but DB deliberately did not select the exact target-validation call sequence;
5. candidate-publication admission already establishes same-workspace/current requester/publisher semantics downstream but does not originate requester-side start intent;
6. the current `Capability` enum has no dedicated requester/reachability/rendezvous-start capability; `DeviceManage` is broader and is not proven semantically equivalent, so DC must not silently reuse it;
7. the existing `AuthorizedRequesterRendezvous` is a downstream one-shot publisher-execution authorization grant and is not a requester-side start-intent carrier.

Therefore the narrowest next seam is a non-authoritative requester-side start-intent carrier. Policy-capability selection, authoritative registry-validation composition, wire/command selection, provider-mutation forwarding, and retirement remain separate later gates.

## Selected semantic carrier

The selected future source type is conceptually:

`RequesterRendezvousStartIntent`

It represents only one server-side operation intent before requester/target validation has produced a registration fact.

It must not be named or treated as a validated registration, authority grant, currentness proof, policy decision, or provider record.

## Exact carrier fields

The carrier contains exactly two owned semantic values:

1. `AuthenticatedDeviceSession` — the requester session supplied by the already-authenticated server-side operation context;
2. `DeviceId` — the logical target nominated by requester intent.

The target field is semantically `target_device_id`, not yet `expected_publisher_device_id`, because DB requires current registry/workspace/authorization validation before the target may become provider-registration authority.

No other field is selected.

## Requester-session provenance rule

The carrier's `AuthenticatedDeviceSession` must come from server-held authenticated operation state.

The future carrier constructor is not an authentication function and must not parse or reconstruct requester identity from request payload material.

No future wire payload may supply or override:

- requester `DeviceId` as authority;
- requester `SessionId` as authority;
- requester `WorkspaceId` as authority;
- requester `UserId` as authority;
- requester public identity material;
- a serialized `AuthenticatedDeviceSession`.

Possession of an `AuthenticatedDeviceSession` value by arbitrary code does not weaken the requirement that production composition obtain it only from the already-authenticated server context.

## Target-intent rule

The carrier's `DeviceId` is requester-selected logical intent only.

It is not yet proof that:

- the target exists;
- the target is currently enrolled/eligible;
- requester and target share a workspace;
- requester is authorized to initiate reachability;
- the target is the provider's expected publisher;
- candidate publication is current or permitted;
- reachability live-owner authority exists.

A later authoritative validation seam must establish those facts before provider mutation is allowed.

## Non-authoritative type rule

`RequesterRendezvousStartIntent` is deliberately not an authority-bearing grant.

The future type must not directly implement or satisfy:

- `RequesterRendezvousAuthorityProvider`;
- provider registration authority;
- candidate-publication authorization;
- policy authorization;
- registry currentness proof;
- reachability live-owner authority.

No downstream API selected by DC may accept this type as sufficient proof for `register_current`.

## Ownership and replay posture

The selected carrier owns both values by value.

The carrier itself must not be `Copy` or `Clone` in its initial source materialization. This prevents the semantic start transaction from gaining implicit duplication/replay behavior before retry/deduplication semantics are separately selected.

This is not a claim that the underlying domain values are secret or non-cloneable. It is a narrow ownership rule for this operation-intent envelope only.

## Visibility boundary

The initial materialized carrier must remain Agent-internal / crate-internal.

DC does not select a public cross-crate API or user-visible representation.

A future source-materialization checkpoint may place the type in a dedicated private `prw-agent` module and register that module crate-internally. Public export is not authorized.

## Constructor boundary

The future materialization may expose only a crate-internal constructor that takes:

- one owned `AuthenticatedDeviceSession`;
- one owned target `DeviceId`.

Construction performs ownership/provenance packaging only. It must not:

- validate registry state;
- evaluate policy;
- mutate provider state;
- perform I/O;
- allocate request IDs;
- inspect transport identity;
- publish candidates;
- acquire reachability live-owner authority;
- spawn work;
- publish readiness.

Because construction does no validation, successful construction must never be described as authorization or current-registration approval.

## Observation/consumption boundary

The future type may expose read-only access to the requester session and target `DeviceId` as needed by a later validation seam.

A consuming decomposition may be selected only if the source-materialization checkpoint needs it to preserve one-way ownership into a later validator. DC does not require a public getter or raw mutable field access.

No method selected by DC may call `register_current`, `authorize_current_for_publisher`, `retire`, or `remove_retired`.

## Validated-fact separation

DC does not select the post-validation registration-fact type.

If a later checkpoint needs a separate typed carrier proving that requester currentness, target eligibility, workspace relation, and policy have already succeeded, that type must be selected separately and must not be conflated with `RequesterRendezvousStartIntent`.

The unvalidated start-intent carrier must never be renamed into a validated fact merely because it contains the same underlying identity values.

## Registry-validation separation

DC does not select an exact target-validation implementation.

Although current source exposes `WorkspaceDeviceRegistry::validate_authenticated_session` and `WorkspaceDeviceRegistry::device`, a later checkpoint must define the narrow fail-closed composition that proves:

- requester session currentness;
- target current eligibility;
- same-workspace relationship;
- exact target preservation.

No registry mutation is authorized by DC.

## Policy separation

DC selects no capability variant and no policy decision.

The exact current policy model has no dedicated requester/reachability/rendezvous-start capability. `Capability::DeviceManage` is not automatically selected because its name is broader than the operation being gated.

A later policy checkpoint must either prove an existing capability is semantically exact enough or select a new narrow capability before policy-gated production execution is materialized.

DC does not modify `prw-policy`.

## Wire and correlation isolation

DC does not add a `BridgeCommand`, operation code, PRWM/PRWC control kind, codec, parser, response mapping, or command-loop branch.

A future request ID may correlate one start transaction but remains outside the carrier's authority-bearing fields and must not define lifecycle identity.

No socket, stream, endpoint, `TransportIdentity`, request ID, candidate ID, freshness token, live-owner fence, or timing value is selected into this carrier.

## Provider isolation

The existing in-memory provider remains private inside its current Agent owner.

DC does not expose or forward:

- `register_current`;
- `authorize_current_for_publisher`;
- `retire`;
- `remove_retired`;
- a raw/mutable provider reference;
- provider extraction.

A future provider mutation seam must consume only a separately validated registration fact, not this unvalidated intent carrier directly.

## Retirement isolation

DC selects only requester-side start intent representation.

It does not select cancellation, retirement, disconnect handling, target-revocation cleanup, timeout/TTL, request completion cleanup, process-shutdown cleanup, retired-record removal, or background maintenance.

DA/DB retirement-source constraints remain unchanged.

## Synchronization and topology isolation

DC does not add/select:

- `Arc`;
- `Mutex`;
- `RwLock`;
- channels;
- actor/mailbox ownership;
- static/global/singleton state;
- task/thread-local authority;
- worker count/topology;
- cross-worker sharing;
- command-loop ownership;
- listener ownership.

## Persistence/distributed isolation

DC does not add/select database/schema, journal/snapshot, durable queue, broker, replication, lease/heartbeat, TTL, clock dependency, or distributed requester/rendezvous authority.

The existing provider remains process-local and bounded exactly as before.

## Dependency and lock guard

DC requires no dependency or lockfile mutation.

Expected unchanged blobs from closed DB:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- Android native `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

No source, manifest, lockfile, workflow, binary, Android, desktop, packaging, database, networking, deployment, or unrelated path is authorized by this selection checkpoint.

## Explicit non-selections

DC does not select, materialize, or authorize:

- source implementation of the carrier;
- a post-validation registration-fact carrier;
- exact registry validation call sequence;
- a policy capability or evaluator change;
- wire command/opcode/frame/codec;
- request retry/deduplication semantics;
- provider mutation forwarding;
- provider getter/extraction;
- provider capacity changes;
- requester cancellation/retirement source;
- cleanup policy;
- synchronization/shared-worker topology;
- command-loop integration;
- listener/accept-loop integration;
- authenticated connection ownership changes;
- Agent binary wiring/readiness;
- persistence/database/distributed coordination;
- firewall/NAT/route/DNS/TUN/TAP mutation;
- STUN/ICE/TURN/relay activation;
- production networking;
- deployment/restart/recovery;
- merge.

## Closure requirements

DC may close only if:

1. DB remains the exact merge base and predecessor;
2. final DB -> DC diff is restricted to this one documentation path;
3. no source, manifest, lockfile, workflow, binary, Android, desktop, packaging, database, networking, deployment, or unrelated path changes;
4. any automatically triggered canonical validation reaches a terminal non-failing verdict on the exact final head;
5. unchanged manifest and lock blobs remain exact;
6. an immutable DC audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-read back exactly;
7. rolling Drive evidence is freshly guarded against exact closed DB;
8. the DC closure record is appended to those exact predecessor bytes;
9. the entire closed-DB rolling prefix is preserved byte-for-byte;
10. rolling Drive update raw-readback matches the intended bytes/hash exactly;
11. only after durable Drive proof may the PR body move `STAGED -> CLOSED`;
12. the PR remains draft/open/unmerged;
13. final GitHub/Drive race checks remain exact.

## Safe successor rule

After durable DC closure, the next checkpoint may materialize only this selected crate-internal non-authoritative start-intent carrier and its minimal module registration/documentation.

That source materialization must not add registry validation, policy selection, provider mutation, wire/command handling, retirement, synchronization, listener/runtime activation, production networking, deployment, or merge.

After carrier source materialization closes, a fresh read-only audit must select the next independent seam rather than bundling authoritative validation, policy, wire, and mutation together.