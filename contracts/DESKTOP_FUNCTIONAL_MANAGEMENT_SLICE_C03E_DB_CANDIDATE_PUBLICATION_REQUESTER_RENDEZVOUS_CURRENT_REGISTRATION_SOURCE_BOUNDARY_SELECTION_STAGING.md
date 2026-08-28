# Phase 152 C03e-DB — Candidate Publication Requester/Rendezvous Current Registration Source Boundary Selection — STAGING

## Status

`STAGED SELECTION`

## Target gate

`C03E_DB_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_CURRENT_REGISTRATION_SOURCE_BOUNDARY_SELECTED`

## Exact predecessor

C03e-DB is rooted only at durably closed C03e-DA:

- branch: `phase-152-c03e-da-candidate-publication-requester-rendezvous-lifecycle-ingress-authority-selection-staging`
- head: `579ce5e274977c37a24f70875887d6b1b6a20061`
- tree: `ae489b5618660c60ce01b183f32a8c7037f9566d`
- PR #224: `Status: CLOSED`, draft/open/unmerged
- gate: `C03E_DA_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_LIFECYCLE_INGRESS_AUTHORITY_SELECTED`

No earlier checkpoint is reopened.

## Checkpoint classification

C03e-DB is a documentation-only semantic source-boundary selection checkpoint.

It selects only the minimum authoritative source semantics from which a future requester/rendezvous `Current` registration may be derived.

It does not add a wire command, opcode, frame, parser, command loop, provider mutation method, runtime integration, synchronization topology, listener behavior, retirement source, production networking, deployment, or merge behavior.

## Exact-head prerequisite audit result

The closed-DA exact source establishes all of the following:

1. the remote logical-session authentication transaction can produce an `AuthenticatedDeviceSession`, but it does not carry an expected publisher/reachability target;
2. the existing PRWC `BridgeCommand` surface contains the already-materialized remote administration operations and contains no requester reachability/rendezvous start or cancel operation;
3. the existing candidate-publication path can validate a requester session against an authenticated publisher and an already-selected target connectivity plan, but it does not originate requester intent and must not manufacture requester/rendezvous authority from publisher traffic;
4. distributed reachability live-owner acquisition is keyed by `PeerConnectivityIdentity` and governs transient owner fencing; it is not a requester-session / expected-publisher rendezvous source;
5. the control-plane reachability acquisition evidence facade exposes live-owner acquisition/lifecycle authority, not an authenticated requester-side target-selection lifecycle;
6. the requester/rendezvous provider and one-shot authorization carrier already exist, but the provider has no authoritative production caller for creating its `Current` records;
7. the Agent-owned CZ runtime owner retains the provider by value but intentionally exposes no lifecycle mutation seam.

Therefore no existing exact source is proven reusable as the missing requester/rendezvous registration source.

DB selects a new semantic source boundary only. It does not invent a production caller or protocol implementation.

## Selected current-registration source boundary

A future requester/rendezvous `Current` registration must originate from one explicit **requester-side reachability-start intent** processed only after the requester has an already-authenticated PRW logical application session.

The authoritative registration inputs are split by provenance:

### Server-held requester identity

The requester identity must come from the already-authenticated server-held `AuthenticatedDeviceSession` associated with the requester-side operation context.

The requester identity must not be accepted from request payload fields.

### Requester-selected logical target

The requester may explicitly nominate exactly one target logical `DeviceId` as the device it wants to reach.

That target `DeviceId` is intent input only. It carries no authority by possession and cannot be registered until server-side validation succeeds.

### Server-side validated registration fact

Only after server-side validation establishes the requester session and target as a currently authorized requester/target pair may the system derive the registration fact consumed by the existing requester/rendezvous provider.

The future provider mutation must receive only the already-authenticated requester session and exact validated target publisher `DeviceId`.

## Required validation before registration

Before any future `register_current` call, the server-owned source boundary must fail closed unless it has established all of the following from authoritative current state:

1. the requester `AuthenticatedDeviceSession` is current;
2. requester logical identity comes from that authenticated session, not request payload data;
3. the nominated target `DeviceId` resolves to a current eligible PRW device under authoritative registry state;
4. requester and target belong to the same current workspace context required by the existing candidate-publication admission model;
5. any separately required requester permission/policy decision has succeeded before registration;
6. the exact target used for validation is the exact `expected_publisher_device_id` registered into requester/rendezvous authority;
7. no transport identity, endpoint, candidate, correlation identifier, or publisher-controlled value is substituted for logical target identity.

DB selects these semantic validation requirements only. It does not select the concrete registry method, policy capability, error wire mapping, or call sequence implementation.

## Source record shape

The selected semantic source record contains exactly the authority-bearing values needed by the existing provider registration boundary:

- server-held authenticated requester session;
- exact validated target publisher logical `DeviceId`.

No additional field is selected as requester/rendezvous authority.

A later source-materialization checkpoint may introduce a private typed carrier for this already-validated fact only if needed to preserve provenance before provider mutation.

DB does not select a public API type or wire representation.

## Request payload boundary

A future requester-side start request may carry the target logical `DeviceId` as intent data.

It must not carry or control:

- requester `DeviceId` as authority;
- requester `SessionId` as authority;
- requester workspace identity as authority;
- requester `AuthenticatedDeviceSession`;
- publisher `TransportIdentity` as target authority;
- reachability live-owner fence/generation;
- requester/rendezvous provider state;
- candidate-publication freshness tokens;
- `CandidateId`;
- `ConnectivityEndpoint`;
- `ConnectivityPathKind`;
- a provider capacity value;
- a lifecycle current/retired flag.

Any such values, if later needed for unrelated correlation or transport mechanics, remain non-authoritative unless separately selected by a later gate.

## Correlation identity rule

A future PRWC/PRWM request ID may correlate one requester-side start transaction, but it must never become requester/rendezvous authority or lifecycle record identity.

The requester/rendezvous record identity remains the existing authenticated requester session identity plus exact expected publisher logical `DeviceId`.

Request retry/deduplication semantics are not selected by DB.

## Transport identity rule

`TransportIdentity` remains lower-transport certificate identity only.

It may be revalidated as required by existing authenticated connection/session mechanics, but it must not replace:

- requester logical identity;
- requester session identity;
- target publisher logical `DeviceId`;
- requester/rendezvous record identity.

Transport-key rotation must not redefine the logical requester/target rendezvous relationship.

## Candidate-publication isolation

The publisher-side candidate-publication path remains strictly downstream of requester/rendezvous registration.

Publisher traffic may consume current requester/rendezvous authority through the existing provider authorization port, but it must not create the registration fact selected here.

The following remain insufficient to create requester-side start authority:

- receipt of a candidate publication;
- publisher authenticated-session identity;
- publisher transport identity;
- publisher socket/endpoint observation;
- candidate publication request ID;
- candidate freshness state;
- target connectivity-plan existence;
- live-owner acquisition/grant/currentness;
- successful candidate commit.

## Live-owner authority separation

Reachability live-owner fencing and requester/rendezvous registration remain separate authorities.

A `ReachabilityLiveOwnerGrant`, `ReachabilityLiveOwnerAcquisition::Granted`, currentness result, or release result does not prove that an authenticated requester currently wants to reach an expected publisher.

Conversely, requester/rendezvous `Current` registration does not grant distributed live-owner authority.

No ordering between future requester registration and live-owner acquisition is selected by DB beyond the existing rule that publisher candidate execution requires current requester/rendezvous authority before reachability commit.

## Existing remote capability protocol is not extended here

DB does not add a new `BridgeCommand`, PRWC operation code, control message kind, codec, or dispatcher branch.

The exact wire location and command family for a future requester-side start intent remain separate selections.

This avoids silently broadening the existing remote administration capability protocol merely because it already carries authenticated remote sessions.

## Authorization capability is not selected here

DB requires any separately required requester policy decision to succeed before registration, but it does not select:

- a new `Capability` variant;
- reuse of an existing capability;
- policy ownership/topology;
- policy error mapping.

A later gate must prove and select that boundary before source materialization if a new policy capability is required.

## Registration lifetime semantics

DB selects only the source of entry into `Current` state.

It does not select how long `Current` remains valid and does not select any automatic or implicit retirement trigger.

In particular, registration does not become retired merely because of:

- PRWC request completion;
- request-ID completion;
- lower-transport close;
- remote-session lease expiry;
- requester disconnect;
- publisher disconnect;
- candidate-publication completion;
- live-owner release;
- candidate commit;
- timeout or elapsed duration;
- Agent process shutdown.

Retirement-source selection remains a separate later checkpoint under the DA authority rules.

## Duplicate/current registration semantics

DB does not weaken or replace the existing provider structural rules.

If the existing provider rejects a duplicate exact current requester-session / target pair or rejects registration because bounded capacity is exhausted, a future caller must preserve that failure rather than silently replacing, merging, evicting, or broadening records.

No eviction or replacement policy is selected here.

## No provider operation seam selected

DB does not add or select an Agent-owner forwarding method for:

- `register_current`;
- `retire`;
- `remove_retired`;
- `authorize_current_for_publisher`.

It also does not expose a raw or mutable provider reference and does not authorize provider extraction.

The future mutation seam remains a separate source-level gate after the requester-side start source itself has been selected/materialized sufficiently to preserve provenance.

## No concrete source materialization selected

DB does not add:

- `RequesterRendezvousStartIntent` source code;
- a validated-start carrier;
- a new command enum variant;
- request parsing;
- a new control frame;
- command dispatch;
- a start transaction function;
- a policy check implementation;
- a registry lookup implementation;
- provider mutation wiring;
- runtime-owner integration.

Those are later gates.

## No retirement or cleanup source selected

DB intentionally does not select:

- requester cancellation semantics;
- session-close retirement;
- disconnect retirement;
- target disappearance/revocation retirement;
- completion-driven retirement;
- TTL/expiry retirement;
- explicit cleanup scheduling;
- retired-record retention;
- background cleanup.

DA's retirement authority requirements remain unchanged.

## No synchronization or topology selection

DB does not select or add:

- `Arc`;
- `Mutex`;
- `RwLock`;
- channels;
- actor/mailbox ownership;
- static/global/singleton state;
- task-local/thread-local authority;
- one-worker versus many-worker topology;
- cross-worker provider sharing;
- command-loop ownership;
- listener ownership.

## No persistence or distributed coordination selection

DB does not select or add:

- database/schema/table;
- journal/snapshot;
- durable queue;
- broker;
- distributed requester/rendezvous authority;
- cross-process replication;
- lease/TTL/heartbeat;
- wall-clock or monotonic-clock dependency.

The existing in-memory provider remains process-local exactly as previously selected.

## Dependency and lock guard

DB requires no dependency or lockfile mutation.

Expected unchanged blobs from closed DA/CZ lineage:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- Android native `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

No source, manifest, lockfile, workflow, binary, Android, desktop, packaging, database, or networking path is authorized by DB.

## Explicit non-selections

DB does not select, materialize, or authorize:

- production protocol opcode or wire encoding;
- command-loop integration;
- policy capability choice;
- exact registry API choice;
- provider mutation forwarding;
- provider getter/extraction;
- production provider capacity policy/value;
- retirement source/trigger;
- cleanup policy;
- synchronization/shared-worker topology;
- remote-session worker topology changes;
- retry/deduplication semantics;
- negative response/error wire mapping;
- listener/accept-loop changes;
- binary wiring;
- readiness publication;
- firewall/NAT/route/DNS/TUN/TAP mutation;
- STUN/ICE/TURN/relay activation;
- production networking;
- deployment/restart/recovery;
- merge.

## Closure requirements

DB may close only if:

1. DA remains the exact merge base and predecessor;
2. final DA -> DB diff is restricted to this one documentation path;
3. no source, manifest, lockfile, workflow, binary, Android, desktop, packaging, database, or networking path changes;
4. any automatically triggered canonical validation reaches a terminal non-failing verdict on the exact final head;
5. unchanged manifest and lock blobs remain exact;
6. an immutable DB audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-read back exactly;
7. rolling Drive evidence is freshly guarded against exact closed DA;
8. the DB closure record is appended to those exact predecessor bytes;
9. the entire closed-DA rolling prefix is preserved byte-for-byte;
10. rolling Drive update raw-readback matches the intended bytes/hash exactly;
11. only after durable Drive proof may the PR body move `STAGED -> CLOSED`;
12. the PR remains draft/open/unmerged;
13. final GitHub/Drive race checks remain exact.

Expected rolling predecessor after closed DA:

- Drive ID `1ZSHwAkU_JwjLG6jQRnkee66lDGlBngx6`
- size `925692` bytes
- SHA-256 `25fe805ff2618fe3675bf034081a0ad3898d892822faaf0b61c29c10ca123629`

## Safe successor rule

DB closure does not authorize a wire command or provider mutation.

The next checkpoint must begin with a fresh exact-head read-only audit and select the narrowest missing seam needed to materialize the requester-side start source while preserving DB provenance.

Candidate options must be considered independently rather than bundled:

- requester-side start intent typed semantic carrier;
- policy-capability selection, only if required and not already represented;
- authoritative registry validation seam;
- wire/command selection;
- provider mutation forwarding.

Retirement-source selection remains separate from current-registration source work.

No direct jump is authorized to synchronization, command-loop activation, listener activation, production networking, deployment, or merge.