# Phase 152 C03e-DG — Candidate Publication Requester/Rendezvous Post-Registry-Validation Provenance Carrier Selection — STAGING

## Status

`STAGED SELECTION`

## Target gate

`C03E_DG_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_POST_REGISTRY_VALIDATION_PROVENANCE_CARRIER_SELECTED`

## Exact predecessor

C03e-DG is rooted only at durably closed C03e-DF:

- branch: `phase-152-c03e-df-candidate-publication-requester-rendezvous-current-registry-validation-composition-source-materialization-staging`;
- head: `346bcce31abc05afba872df1a266a041b2bb3174`;
- tree: `9aa7d918e4d95c104c05ef329aae649b1b231ae8`;
- PR #229: `Status: CLOSED`, draft/open/unmerged;
- gate: `C03E_DF_CANDIDATE_PUBLICATION_REQUESTER_RENDEZVOUS_CURRENT_REGISTRY_VALIDATION_COMPOSITION_SOURCE_MATERIALIZED`.

No earlier checkpoint is reopened.

## Checkpoint classification

C03e-DG is a documentation-only semantic carrier-selection checkpoint.

It selects only the minimum typed provenance boundary needed to preserve the fact that one server-held requester session and one exact logical target passed the C03e-DF current-registry validation composition.

It does not materialize source code, select policy authorization, call the requester/rendezvous provider, select a wire command, change synchronization/runtime topology, activate networking, deploy, or merge.

## Exact-head prerequisite audit result

The closed-DF exact source proves all of the following:

1. `RequesterRendezvousStartIntent` is deliberately an unvalidated Agent-internal carrier containing the server-held `AuthenticatedDeviceSession` plus requester-nominated target `DeviceId`;
2. `validate_current_requester_rendezvous_start_intent(...)` performs the selected current-registry validation but currently returns only `Result<(), RequesterRendezvousStartRegistryValidationError>`;
3. after a successful DF call, the caller still holds the same syntactically unvalidated intent type; no typed value distinguishes the successfully registry-validated pair from the pre-validation pair;
4. `InMemoryRequesterRendezvousAuthorityProvider::register_current(...)` accepts raw `AuthenticatedDeviceSession` plus `DeviceId` and relies on its caller to have established upstream authority/provenance correctly;
5. provider mutation therefore must not be wired directly from the DD intent merely because DF can return `Ok(())`;
6. exact current `prw-policy::Capability` still contains no requester-rendezvous-start/reachability-start capability, so policy authorization remains an independent later gate rather than something DG may silently infer or reuse.

No existing exact type was found that represents only the post-DF current-registry-validated requester/target pair.

Therefore the narrowest next seam is a typed post-registry-validation provenance carrier. Provider mutation and policy selection remain separately gated.

## Selected carrier meaning

A future source-materialization checkpoint may introduce one effective crate-internal owned value provisionally named:

`RegistryValidatedRequesterRendezvousStart`

The name is semantic, not a requirement to expose a public API.

Possession of this value means only:

- the requester identity came from the server-held authenticated PRW application session carried by the DD intent;
- that requester session passed the exact C03e-DF current-registry validation at the validation point;
- the nominated target logical `DeviceId` resolved to an enrolled device whose exact workspace/user membership was active;
- requester and target were in the same current workspace;
- the exact validated target remained the exact nominated target `DeviceId`.

It does not mean policy authorization, provider registration, continued future currentness, transport readiness, publisher publication authority, live-owner authority, or network reachability.

## Selected authority-bearing fields

The future carrier must contain only the two values needed to preserve DB/DF provenance:

- the server-held `AuthenticatedDeviceSession` that was actually validated by DF;
- the exact logical target `DeviceId` that was actually validated by DF.

No additional value becomes requester/rendezvous authority.

In particular, the carrier must not contain as authority:

- requester `DeviceId` supplied separately from the authenticated session;
- requester `SessionId` supplied separately from the authenticated session;
- caller-supplied workspace identity;
- target `TransportIdentity`;
- `ConnectivityEndpoint`;
- `CandidateId`;
- candidate path kind;
- candidate-publication freshness state;
- PRWC/PRWM request ID;
- live-owner generation/fence/grant;
- provider capacity;
- lifecycle current/retired flag;
- clock/TTL data;
- socket/connection handles.

## Construction provenance rule

The future carrier must not have a general public constructor that accepts an arbitrary `AuthenticatedDeviceSession` and `DeviceId`.

Construction must be reachable only from the successful C03e-DF validation path, or from an equivalently narrow private helper called exclusively after the full DF validation sequence has succeeded.

A caller that merely possesses a DD `RequesterRendezvousStartIntent` must not be able to construct the post-validation carrier without executing the selected registry validation.

A caller that merely possesses a target `DeviceId`, registry device record, transport identity, candidate publication, live-owner grant, or request ID must not be able to construct it.

## Validation-return selection

The preferred future materialization shape is for the DF validation composition to return the typed carrier on success rather than `()`:

`Result<RegistryValidatedRequesterRendezvousStart, RequesterRendezvousStartRegistryValidationError>`

This selection exists specifically to make validated provenance explicit in the type flow.

DG does not yet edit the function signature or source code.

Whether future implementation borrows or consumes the original DD intent while validating is not selected here. Either implementation must preserve the same authority rule: only the successful full DF validation path may create the typed carrier.

## Ownership and replay posture

The future carrier should be owned and should not be `Copy`.

DG selects no requirement for `Clone`. The safer default for later materialization is to avoid `Clone` unless a concrete downstream seam proves duplication is necessary and authority-safe.

The carrier is not a replay token and is not a durable authorization lease.

Its existence records successful validation at one operation boundary only. It does not freeze registry state or guarantee that membership/device/session state remains current indefinitely afterward.

## Accessor boundary

If later source materialization requires accessors, they should expose borrowed references only to:

- the validated authenticated requester session;
- the validated target logical `DeviceId`.

DG does not select a public `into_parts`, mutable access, provider getter, provider reference, or arbitrary extraction API.

Any future consuming adapter into a provider-ready registration path must be selected separately and must preserve all intervening authority gates.

## Policy separation

The DG carrier is explicitly **not policy-authorized**.

Exact current `prw-policy::Capability` has no requester-rendezvous-start/reachability-start capability, and DG does not:

- add a capability variant;
- reuse `DeviceManage`;
- reuse `ForwardingCreate`;
- reuse any file/terminal capability;
- select a `PolicyEvaluator` topology;
- select policy decision ownership;
- select policy error mapping.

If requester-rendezvous start requires a policy decision, that decision must be selected and succeed at a separate gate before provider registration.

A registry-validated DG carrier alone must never be treated as proof that policy has allowed the operation.

## Provider separation

DG does not call, wrap, forward, expose, or otherwise activate:

- `InMemoryRequesterRendezvousAuthorityProvider::register_current`;
- `retire`;
- `remove_retired`;
- `authorize_current_for_publisher`;
- provider extraction/getters;
- raw/mutable provider references.

The existing provider continues to accept its existing raw input shape, but no DG selection authorizes feeding it from the DD intent or the future registry-validated carrier.

A later provider-mutation checkpoint must prove how all required upstream authority—including any separately selected policy decision—is preserved before `register_current` executes.

## Target identity and transport separation

The validated target remains logical `DeviceId` only.

`TransportIdentity` remains lower-transport certificate identity and is not required to construct the DG carrier.

DG does not require a target to have a bound/current transport identity before its logical requester/rendezvous start provenance may be represented.

Transport readiness, endpoint resolution and candidate publication remain downstream/separate concerns.

Transport-key rotation must not redefine the logical target stored in the carrier.

## Candidate-publication isolation

The future DG carrier originates only from requester-side DD intent after DF registry validation.

It must never be constructible from publisher-side candidate-publication traffic.

The following remain insufficient to create it:

- authenticated publisher session;
- publisher `TransportIdentity`;
- candidate-publication frame/request ID;
- candidate/freshness state;
- observed socket/endpoint;
- current connectivity plan;
- successful candidate commit;
- live-owner acquisition/grant/currentness.

Publisher traffic remains downstream of requester/rendezvous current-registration authority.

## Live-owner authority separation

A `ReachabilityLiveOwnerGrant` or related fencing/currentness evidence does not imply the requester currently intends to reach the target and cannot construct the DG carrier.

Conversely, possession of the DG carrier does not grant distributed reachability ownership.

No new ordering with live-owner acquisition is selected here.

## Point-in-time currentness rule

The future carrier records successful DF validation at its creation boundary; it is not a perpetual currentness guarantee.

DG does not select:

- a TTL;
- an expiry timestamp;
- a monotonic-clock dependency;
- automatic revalidation cadence;
- registry snapshot persistence;
- lease semantics.

Any future gap between validation and provider mutation must be evaluated by the later composition checkpoint. DG itself authorizes no long-lived caching or asynchronous retention of the carrier.

## No registration lifecycle selection

DG does not select how or when a future successful registration becomes retired.

It does not select requester cancellation, session close, disconnect, target revocation, membership removal, candidate completion, live-owner release, timeout, process shutdown, or another event as a retirement trigger.

Retirement-source selection remains separately gated under the existing DA rules.

## No wire or correlation selection

DG does not add or select:

- `BridgeCommand` variant;
- PRWC/PRWM opcode;
- frame/control-message kind;
- codec/parser;
- command dispatcher;
- request/response mapping;
- retry/deduplication semantics.

Any future request ID remains message correlation only and must not become part of the carrier's authority identity.

## No synchronization or runtime topology selection

DG does not select or add:

- `Arc`;
- `Mutex`;
- `RwLock`;
- channels;
- actor/mailbox ownership;
- globals/singletons;
- task-local/thread-local authority;
- command-loop ownership;
- listener ownership;
- cross-worker provider sharing;
- Agent binary/main wiring.

## No persistence or deployment selection

DG does not select or add:

- database/schema/table;
- journal/snapshot;
- durable queue;
- broker;
- distributed requester/rendezvous authority;
- cross-process replication;
- production networking;
- firewall/NAT/route/DNS/TUN/TAP changes;
- STUN/ICE/TURN/relay activation;
- deployment/restart/recovery;
- merge.

## Dependency and lock guard

DG requires no dependency or lockfile mutation.

Expected unchanged blobs from closed DF:

- `crates/prw-agent/Cargo.toml`: `18ed32b080cac9b4540b33f870388499d7e5bc52`;
- root `Cargo.lock`: `eeacde7ee776d35088f746a6d09f823f3391b82b`;
- Android native `apps/android/native/Cargo.lock`: `cce9ca06190a196661ab38d54a747893e26af95f`.

No source, manifest, lockfile, workflow, binary, Android, desktop, packaging, database, networking, or deployment path is authorized by DG.

## Exact source anchors audited at closed DF

- DF registry-validation source: `1f614382cda149270405d9aafd7264bac0157610`;
- DD start-intent carrier module after DF registration: `34d7af94e56bf085cececf08b080e55ed8e32cdd`;
- requester/rendezvous in-memory provider: `d01cfbc37433f6099e216397b9bf243aa55c53bc`;
- current policy model: `c2a02e5640a3274fa7a6d04dacb91d06a8d0df93`;
- DF contract: `0c593c4a574014f64557271ed4b84900556cd49b`.

## Explicit non-selections

DG does not select, materialize, or authorize:

- source implementation of the carrier;
- source modification of the DF validation return type;
- carrier extraction/consumption API;
- provider mutation forwarding;
- provider access/getter/extraction;
- requester-rendezvous-start policy capability;
- policy evaluation;
- target transport readiness;
- wire command/opcode/frame/parser/dispatcher;
- request-ID authority;
- retry/deduplication;
- registration retirement/cancellation/cleanup;
- synchronization/shared-worker topology;
- command-loop/listener activation;
- Agent binary wiring;
- persistence/database/distributed coordination;
- production networking;
- deployment/restart/recovery;
- merge.

## Closure requirements

DG may close only if:

1. C03e-DF remains the exact merge base and predecessor;
2. final DF -> DG diff is restricted to this one documentation path;
3. no source, manifest, lockfile, workflow, binary, Android, desktop, packaging, database, networking, or deployment path changes;
4. all automatically triggered canonical validations on the exact final head reach terminal non-failing verdicts;
5. unchanged manifest/lock blobs remain exact;
6. an immutable DG audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-read back byte-exact;
7. rolling Drive evidence is freshly guarded against exact closed DF;
8. the DG closure record is appended only to those exact predecessor bytes;
9. the entire closed-DF rolling prefix is preserved byte-for-byte;
10. rolling Drive update raw-readback matches intended bytes/hash exactly;
11. only after durable Drive proof may the PR body move `STAGED -> CLOSED`;
12. the PR remains draft/open/unmerged;
13. final GitHub/Drive race checks remain exact.

Expected rolling predecessor after closed DF:

- Drive ID: `1ZSHwAkU_JwjLG6jQRnkee66lDGlBngx6`;
- size: `965513` bytes;
- SHA-256: `079a3c8bd8481a8914ec3d05bf53513874262d1e314a43756ec27dfa5c16da32`.

## Safe successor rule

After durable DG closure, the next checkpoint must begin with a fresh exact-head read-only audit.

Candidate independent seams remain separate:

- materializing the selected post-registry-validation provenance carrier;
- independently selecting requester-rendezvous-start policy authorization;
- provider mutation forwarding only after all prerequisite authority gates are selected;
- wire/command selection;
- retirement/cancellation source selection.

No automatic bundling is authorized. No direct jump is authorized to provider mutation, synchronization, runtime/listener activation, production networking, deployment, or merge.
