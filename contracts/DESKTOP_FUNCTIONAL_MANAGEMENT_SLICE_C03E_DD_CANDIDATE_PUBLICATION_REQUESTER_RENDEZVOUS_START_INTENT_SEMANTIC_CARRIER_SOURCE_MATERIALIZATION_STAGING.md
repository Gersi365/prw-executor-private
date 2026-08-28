# Phase 152 C03e-DD — Candidate Publication Requester/Rendezvous Start Intent Semantic Carrier Source Materialization — STAGING

## Status

`STAGED SOURCE MATERIALIZATION`

## Target gate

`C03E_DD_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_START_INTENT_SEMANTIC_CARRIER_SOURCE_MATERIALIZED`

## Exact predecessor

C03e-DD is rooted only at durably closed C03e-DC:

- branch: `phase-152-c03e-dc-candidate-publication-requester-rendezvous-start-intent-semantic-carrier-selection-staging`
- head: `8506a63610b32832aeb88b38024f8e1a1fb3eb53`
- tree: `0064ff6ae8903777da352f7569319e07e17c648d`
- PR #226: `Status: CLOSED`, draft/open/unmerged
- gate: `C03E_DC_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_START_INTENT_SEMANTIC_CARRIER_SELECTED`

Expected rolling predecessor after closed DC:

- Drive ID `1ZSHwAkU_JwjLG6jQRnkee66lDGlBngx6`
- size `941132` bytes
- SHA-256 `ddf4a37f8fceef212631251494eb22def34b757aab36ad7f19fba681fc0d37b2`

No earlier checkpoint is reopened.

## Checkpoint classification

C03e-DD is a narrowly scoped source-materialization checkpoint.

It materializes only the crate-internal, non-authoritative `RequesterRendezvousStartIntent` selected by C03e-DC plus the minimum crate-internal module registration and this contract.

It does not materialize registry validation, policy selection/evaluation, a validated registration fact, requester/rendezvous provider mutation, PRWC/PRWM wire handling, command dispatch, retirement, synchronization, listener/runtime activation, networking, deployment, or merge.

## Materialized source surface

The authorized DD source surface is exactly:

1. `crates/prw-agent/src/candidate_publication_requester_rendezvous_start_intent.rs`;
2. `crates/prw-agent/src/lib.rs` only for crate-internal module registration;
3. this DD contract.

No other source, manifest, lockfile, workflow, Android, desktop, packaging, database, networking, deployment, or unrelated path is authorized.

## Materialized carrier

The materialized type is:

`RequesterRendezvousStartIntent`

The type is `pub(crate)` and lives only inside `prw-agent`.

It owns exactly two fields by value:

- `requester_session: AuthenticatedDeviceSession`;
- `target_device_id: DeviceId`.

No additional identity, authority, timing, transport, request/correlation, candidate, freshness, synchronization, provider, or runtime field is materialized.

## Requester-session provenance

`requester_session` is an already-authenticated server-held `AuthenticatedDeviceSession`.

DD does not create a new public constructor for `AuthenticatedDeviceSession`, parse identity from request payload material, reconstruct a session from IDs, or permit wire input to replace server-held authenticated requester identity.

The carrier constructor accepts an existing owned `AuthenticatedDeviceSession` only because production composition must receive that value from the already-authenticated server-side operation context.

Construction does not authenticate the requester and possession of the carrier is not authentication proof by itself.

## Target-intent semantics

`target_device_id` is requester-nominated logical intent only.

It is deliberately not named or treated as `expected_publisher_device_id` authority at this stage.

DD does not establish that the target:

- exists in the current registry;
- is currently enrolled or eligible;
- shares the requester workspace;
- is authorized for requester-side reachability start;
- is the provider's expected publisher;
- owns current candidate-publication authority;
- has current transport identity;
- has live-owner reachability authority.

Those facts remain separately gated.

## Non-authoritative type posture

`RequesterRendezvousStartIntent` is not an authority grant, registration fact, currentness proof, policy decision, provider record, or live-owner proof.

It does not implement `RequesterRendezvousAuthorityProvider` and cannot directly mutate the provider.

No DD API treats this unvalidated carrier as sufficient proof for `register_current`, candidate publication, reachability mutation, policy authorization, or registry currentness.

## Ownership and replay posture

The carrier owns both selected semantic values by value.

The carrier derives no `Clone` or `Copy` implementation.

DD therefore does not introduce implicit semantic duplication/replay behavior for requester-side start intent.

This is a property of the operation envelope only and does not change cloneability of underlying domain values.

## Constructor boundary

The only materialized constructor is crate-internal:

`RequesterRendezvousStartIntent::new(AuthenticatedDeviceSession, DeviceId) -> RequesterRendezvousStartIntent`

Construction performs ownership composition only.

It does not:

- validate registry state;
- evaluate policy;
- mutate requester/rendezvous provider state;
- inspect transport identity;
- allocate a request ID;
- parse or encode wire data;
- publish candidates;
- acquire reachability live-owner authority;
- perform socket/network I/O;
- spawn a task/thread/listener;
- publish readiness.

Successful construction is therefore not described as authorization or current-registration approval.

## Read-only observation boundary

The materialized carrier exposes crate-internal read-only accessors only:

- `requester_session(&self) -> &AuthenticatedDeviceSession`;
- `target_device_id(&self) -> &DeviceId`.

No mutable field accessor, provider getter, raw authority extraction, consuming provider operation, or cross-crate public export is added.

## Test boundary

Because `AuthenticatedDeviceSession` has no public arbitrary constructor and is produced through authenticated session proof, DD does not fabricate a fake authenticated identity merely to instantiate the carrier in a unit test.

DD intentionally adds no carrier-specific runtime unit fixture. Canonical workspace compilation, Clippy, tests, and build validate that the declared constructor/accessor source type-checks while preserving authenticated-session provenance.

This avoids introducing unrelated cryptographic/session fixture orchestration solely to exercise a non-authoritative data envelope.

DD does not add integration tests that exercise provider mutation, registry validation, policy, wire, runtime, or networking.

## Module visibility boundary

`crates/prw-agent/src/lib.rs` registers the new module as `pub(crate)` only.

A narrow `dead_code` allowance documents that the carrier is materialized before its separately gated consumer exists.

No public re-export or cross-crate API is introduced.

## Validated-fact separation

DD does not materialize the post-validation registration-fact carrier.

The unvalidated intent is not renamed or transformed into a validated fact in this checkpoint.

A later checkpoint must independently select/materialize any carrier proving requester currentness, target eligibility, same-workspace relationship, exact target preservation, and the applicable policy result before provider mutation.

## Registry-validation separation

DD performs no call to `WorkspaceDeviceRegistry::validate_authenticated_session`, `WorkspaceDeviceRegistry::device`, transport validation, membership mutation, device mutation, or other registry operation.

The exact fail-closed requester/target validation composition remains a later independent seam.

## Policy separation

DD changes no `prw-policy` source and selects no capability.

It does not reuse `DeviceManage` or any other existing capability as an implicit requester/rendezvous-start decision.

Policy remains independently gated after current registry semantics are selected.

## Provider isolation

DD does not expose, forward, call, or wrap:

- `register_current`;
- `authorize_current_for_publisher`;
- `retire`;
- `remove_retired`;
- a mutable/raw provider reference;
- provider extraction.

The existing `CandidatePublicationRequesterRendezvousRuntimeOwner` remains unchanged and its provider remains private.

## Wire/command isolation

DD adds no `BridgeCommand`, PRWC/PRWM opcode/control kind, frame variant, decoder, encoder, response mapping, request ID, command-loop branch, or user-visible request representation.

The carrier is not a wire type.

## Retirement and cleanup isolation

DD does not select cancellation, requester retirement, target-revocation cleanup, disconnect handling, timeout/TTL, completion cleanup, shutdown cleanup, retired-record removal, or background maintenance.

Existing DA/DB lifecycle constraints remain unchanged.

## Synchronization/topology isolation

DD adds no `Arc`, `Mutex`, `RwLock`, channel, actor/mailbox, static/global/singleton state, worker topology, command-loop ownership, task/thread ownership, or listener ownership.

## Persistence/distributed isolation

DD adds no database/schema, migration, journal/snapshot, durable queue, broker, replication, lease, heartbeat, TTL, clock dependency, distributed requester/rendezvous authority, or external service dependency.

## Runtime/networking isolation

DD does not start or wire an Agent runtime path, accept/connect loop, socket listener, STUN/ICE/TURN/relay behavior, NAT/firewall/route/DNS/TUN/TAP mutation, readiness publication, systemd/host mutation, deployment, restart, or recovery behavior.

## Dependency and lock guard

DD requires no dependency or lockfile mutation.

Required unchanged blobs:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`;
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`;
- Android native `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`.

Any drift in those blobs invalidates DD closure until separately audited.

## Explicit non-selections

C03e-DD does not select, materialize, or authorize:

- post-validation registration-fact source;
- exact requester/target registry validation sequence;
- a new/existing policy capability selection;
- policy evaluation composition;
- requester/rendezvous provider mutation forwarding;
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
- readiness;
- persistence/database/distributed coordination;
- production networking;
- deployment/restart/recovery;
- merge.

## Closure requirements

DD may close only if:

1. C03e-DC remains the exact merge base and predecessor;
2. final DC -> DD diff is restricted to exactly the carrier module, minimal `lib.rs` module registration, and this DD contract;
3. the carrier fields/visibility/constructor/accessors remain exactly within DC-selected semantics;
4. no registry, policy, provider mutation, wire/command, retirement, synchronization, runtime/networking, deployment, or unrelated source is added;
5. required manifest/lock blobs remain exact;
6. all automatically triggered canonical validations on the exact final head reach terminal non-failing verdicts;
7. an immutable DD audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-read back byte-exact;
8. rolling Drive evidence is freshly guarded against exact closed DC;
9. the DD closure record is appended only to those exact predecessor bytes;
10. the entire closed-DC rolling prefix is preserved byte-for-byte;
11. rolling Drive update raw-readback matches intended bytes/hash exactly;
12. only after durable Drive proof may the PR body move `STAGED -> CLOSED`;
13. the PR remains draft/open/unmerged;
14. final GitHub/Drive race checks remain exact.

## Safe successor rule

After durable DD closure, the next checkpoint must begin with a fresh read-only audit from exact closed DD.

That audit may select only the narrowest next independent semantic seam. It must not automatically bundle requester/target current-registry validation, a validated registration-fact carrier, policy capability selection, wire/command handling, provider mutation, retirement, synchronization, runtime activation, networking, deployment, or merge.
