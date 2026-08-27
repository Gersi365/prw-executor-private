# Phase 152 C03e-DA — Candidate Publication Requester/Rendezvous Lifecycle Ingress Authority Selection — STAGING

## Status

`STAGED SELECTION`

## Target gate

`C03E_DA_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_LIFECYCLE_INGRESS_AUTHORITY_SELECTED`

## Exact predecessor

C03e-DA is rooted only at durably closed C03e-CZ:

- branch: `phase-152-c03e-cz-candidate-publication-requester-rendezvous-agent-owned-runtime-lifetime-source-materialization-staging`
- head: `7322da8702331c0f53b4cc810778be459a2d7133`
- tree: `1a50a3dfebf3543333e103b6bfee3e9288dc0b0c`
- PR #223: `Status: CLOSED`, draft/open/unmerged
- gate: `C03E_CZ_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_AGENT_OWNED_RUNTIME_LIFETIME_SOURCE_MATERIALIZED`

No earlier checkpoint is reopened.

## Checkpoint classification

C03e-DA is a documentation-only authority-selection checkpoint.

It does not add source code, methods, event types, wire formats, runtime wiring, synchronization, provider construction, listener behavior, production networking, deployment, or merge behavior.

## Read-only prerequisite audit result

The exact CZ head establishes all of the following:

1. `InMemoryRequesterRendezvousAuthorityProvider` already owns bounded process-local requester/rendezvous records.
2. Its lifecycle mutations are explicit: `register_current`, `retire`, and `remove_retired`.
3. Its authorization read path is `RequesterRendezvousAuthorityProvider::authorize_current_for_publisher`.
4. C03e-CZ places one already-configured provider by value inside `CandidatePublicationRequesterRendezvousRuntimeOwner`.
5. The CZ owner intentionally exposes no provider getter, extraction, lifecycle mutation forwarding, candidate execution, task/thread ownership, synchronization, readiness, or networking behavior.
6. Existing candidate-publication execution is provider-neutral and consumes requester/rendezvous authority only after the publisher is already an authenticated PRWC logical session.
7. No exact current source proves a concrete authoritative requester-side lifecycle-event producer that may safely call the provider mutators through the new Agent-owned lifetime boundary.

Therefore DA selects only the authority provenance required for future lifecycle ingress. It does not pretend that a concrete caller or ingress protocol already exists.

## Preserved requester/rendezvous authority model

DA preserves the previously selected authority meaning:

- requester identity is an already-authenticated `AuthenticatedDeviceSession`;
- the rendezvous target is an exact expected publisher logical `DeviceId`;
- current authority is server-owned state, not publisher-controlled state;
- requester/rendezvous authorization remains independent of transport identity and message-correlation identity;
- ambiguous, stale, missing, or unavailable authority remains fail-closed according to the existing provider contract.

The exact lifecycle record identity remains the existing requester-session identity plus expected publisher device identity.

## Selected lifecycle-ingress authority provenance

A future transition into a `Current` requester/rendezvous record may be initiated only from an already-authoritative server-side requester/rendezvous lifecycle decision that has established, before provider mutation:

1. one already-authenticated requester `AuthenticatedDeviceSession`;
2. one exact expected publisher logical `DeviceId`;
3. server-side provenance that this authenticated requester is currently awaiting or targeting reachability for that exact publisher in the applicable workspace context.

The lifecycle mutation itself does not create this authority. It records state after that authority has already been established by a separately reviewed server-owned source.

DA does not select the concrete source, protocol, command, API, task, or runtime owner that will establish this provenance.

## Registration authority rule

Future `register_current` ingress must be downstream of the selected server-owned authority provenance above.

Registration must not infer requester/rendezvous authority from any publisher-submitted or transport-level value.

The existing provider remains authoritative for its structural lifecycle rules, including finite configured capacity, duplicate-record rejection, and exact record identity.

DA does not select a production capacity value and does not authorize provider construction at registration time.

## Retirement authority rule

Future retirement may occur only after a separately authoritative server-side requester/rendezvous lifecycle decision establishes that the exact requester-session / expected-publisher record is no longer current.

The retirement decision must remain bound to the same exact lifecycle identity used by the existing provider.

DA does not select any implicit retirement trigger.

In particular, DA does not define any of the following as sufficient retirement authority by itself:

- lower-transport connection close;
- PRWC stream close;
- socket disappearance;
- publisher disconnect;
- requester disconnect;
- a timer or elapsed duration;
- process shutdown;
- candidate-publication completion;
- reachability commit completion;
- a request-ID lifecycle event.

A later checkpoint may select one or more lifecycle termination conditions only after proving their authority and failure semantics.

## Retired-record removal rule

`remove_retired` remains cleanup after an explicit retired state.

Removal must not be used to manufacture retirement, currentness, requester authorization, or publisher authorization.

DA selects no cleanup schedule, retention duration, background task, storage policy, or capacity-reclamation policy beyond the existing explicit provider operation.

## Publisher-path isolation

The candidate publisher path must not self-create or self-retire requester/rendezvous authority.

None of the following may serve as lifecycle-ingress authority:

- candidate-publication payload fields;
- PRWC request ID;
- PRWM/control-frame correlation ID;
- `TransportIdentity`;
- socket or endpoint address;
- `CandidateId`;
- `ConnectivityEndpoint`;
- `ConnectivityPathKind`;
- candidate ordering;
- candidate-publication freshness token;
- reachability generation/fence values;
- publisher-controlled retry state.

The authenticated publisher session remains evidence only for publisher logical identity on the candidate-publication path. It does not create the independent requester/rendezvous lifecycle record.

## Identity boundary

DA preserves the established identity separation:

- `DeviceId` / authenticated PRW session identity = logical identity;
- `TransportIdentity` = lower-transport certificate identity only;
- `SessionId` = authentication/session correlation within the existing requester record identity;
- `CandidateId` = plan-scoped candidate correlation only;
- `ConnectivityEndpoint` = transient endpoint/configuration state only;
- `ConnectivityPathKind` = product path classification only;
- PRWC/PRWM request IDs = message correlation only.

No one of these identities may be silently substituted for another.

## No concrete ingress source selected

The exact CZ repository state does not, by this checkpoint alone, prove a concrete authoritative requester-side rendezvous lifecycle producer suitable for provider mutation.

DA therefore does not claim that such a source already exists.

A later read-only audit must determine one of two outcomes:

1. an existing server-owned authenticated requester/rendezvous lifecycle source can be reused without broadening its authority; or
2. no suitable source exists, in which case requester-side lifecycle source/protocol selection must occur before mutation wiring.

This unresolved source is intentional and is not permission to invent an ad hoc caller.

## No operation seam selected

DA does not select or authorize a wrapper method for:

- `register_current`;
- `retire`;
- `remove_retired`;
- `authorize_current_for_publisher`;
- candidate-publication execution;
- result-frame writing;
- raw provider access;
- mutable provider access;
- provider extraction.

Whether a future Agent-owned mutation seam forwards one or more existing provider operations remains a separate gate after the concrete lifecycle authority source is proven.

## No synchronization or worker topology selected

DA does not add or select:

- `Arc`;
- `Mutex`;
- `RwLock`;
- channels;
- actor/mailbox ownership;
- static/global/singleton state;
- task-local or thread-local authority;
- cross-worker cloning;
- one-worker versus many-worker topology;
- a `Clone`/`Copy` contract;
- an architectural `Send`/`Sync` requirement.

Ordinary Rust auto traits are not interpreted as a selected concurrency architecture.

## No time, persistence, or cleanup policy selected

DA does not add or select:

- TTL;
- expiry timestamp;
- wall clock or monotonic clock source;
- heartbeat;
- lease renewal;
- automatic retirement;
- background cleanup;
- database/schema/table;
- journal or snapshot;
- durable queue;
- broker/distributed coordination;
- cross-process replication.

The current bounded provider remains process-local and non-durable exactly as previously materialized.

## Preserved candidate-publication execution boundary

DA does not change the existing C03e-CQ execution ordering:

1. derive publisher logical identity from the already-authenticated connection/session;
2. construct the current candidate publication under existing identity checks;
3. perform exactly one requester/rendezvous authorization lookup for the publisher;
4. require exact expected-publisher equality;
5. invoke the existing reachability-owner commit.

Lifecycle ingress must remain independent of that publisher execution path.

## Preserved Agent lifetime boundary

DA does not change the CZ lifetime owner:

`CandidatePublicationRequesterRendezvousRuntimeOwner`

It remains ownership-only and does not expose lifecycle operations.

DA does not add the owner to the Agent binary, transport runtime, command loop, readiness state, listener owner, or any task/thread topology.

## Dependency and lock guard

DA requires no dependency or lockfile mutation.

Expected unchanged blobs from closed CZ:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`
- Android native `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`

No source, manifest, lockfile, workflow, binary, Android, desktop, or packaging path is authorized by DA.

## Explicit non-selections

DA does not select, materialize, or authorize:

- a concrete requester-side lifecycle producer;
- a requester rendezvous command or wire message;
- a lifecycle event schema;
- lifecycle mutation forwarding methods;
- provider getter/extraction;
- production provider capacity policy/value;
- synchronization/shared-worker topology;
- a command loop;
- retry/reconnect behavior;
- malformed-command response mapping;
- connection keepalive/close policy;
- listener/accept-loop activation;
- authenticated PRWC connection ownership changes;
- `WorkspaceDeviceRegistry` ownership changes;
- `SessionAuthenticationService` ownership changes;
- `ProductionReachabilityOwner` ownership changes;
- persistence/database/schema/journal/snapshot;
- TTL/clock/expiry/background cleanup;
- distributed broker/coordination semantics;
- credentials/certificates/bootstrap material;
- production bind address;
- Agent binary wiring;
- readiness publication;
- firewall/NAT/route/DNS/TUN/TAP mutation;
- STUN/ICE/TURN/relay activation;
- production networking;
- deployment/restart/recovery;
- merge.

## Closure requirements

DA may close only if:

1. CZ remains the exact merge base and predecessor;
2. final CZ -> DA diff is restricted to this one documentation path;
3. no source, manifest, lockfile, workflow, binary, Android, desktop, or packaging path changes;
4. any automatically triggered canonical validation reaches a terminal non-failing verdict on the exact final head;
5. the unchanged manifest and lock blobs remain exact;
6. an immutable DA audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-read back exactly;
7. rolling Drive evidence is freshly guarded against exact closed CZ;
8. the DA closure record is appended to those exact predecessor bytes;
9. the entire closed-CZ rolling prefix is preserved byte-for-byte;
10. rolling Drive update raw-readback matches the intended bytes/hash exactly;
11. only after durable Drive proof may the PR body move `STAGED -> CLOSED`;
12. the PR remains draft/open/unmerged;
13. final GitHub/Drive race checks remain exact.

Expected rolling predecessor after closed CZ:

- Drive ID `1ZSHwAkU_JwjLG6jQRnkee66lDGlBngx6`
- size `915320` bytes
- SHA-256 `76f02b866bc8f0fbbdbcc9319d9ee21df9a6a8e0353893464bb75d7eb576db35`

## Safe successor rule

DA closure does not authorize lifecycle mutation source materialization by itself.

The next checkpoint must begin with a fresh exact-head read-only audit and determine whether an existing authoritative server-owned requester/rendezvous lifecycle source can be reused.

- If such a source is proven, a later gate may select a narrow Agent-internal mutation seam that preserves the DA provenance rules.
- If no such source is proven, requester-side lifecycle source/protocol selection must occur before any provider mutation wiring.

Candidate-execution integration, synchronization/shared-worker topology, command-loop construction, listener activation, production networking, deployment, and merge remain separate later gates.