# Phase 152 C03e-NF — Production expected-device scheduling event source provenance selection

Status: `STAGED_SELECTION`

Target gate:

`C03E_NF_PRODUCTION_EXPECTED_DEVICE_SCHEDULING_EVENT_SOURCE_PROVENANCE_SELECTED`

## 1. Purpose

C03e-NE proved that the current production remote-session composition owns only the receiver side of repeated expected-device admission and does not prove either a production scheduling ingress or an expected-request sender/channel owner.

NE narrowed the next prerequisite to:

`PRODUCTION_EXPECTED_DEVICE_SCHEDULING_EVENT_SOURCE_PROVENANCE`

C03e-NF audits only that prerequisite.

This checkpoint asks one bounded question:

> Does the exact post-NE repository already contain a production lifecycle/event whose authority semantics prove that one logical `DeviceId` may be scheduled for exactly one pre-authentication expected-device admission attempt?

C03e-NF does not create that event, does not create a channel, and does not modify Rust source.

## 2. Exact predecessor

Canonical repository:

`Gersi365/prw-executor-private`

Exact predecessor branch:

`phase-152-c03e-ne-production-expected-device-scheduling-ingress-expected-request-sender-channel-ownership-selection`

Exact predecessor head:

`f84e19a592e3c5669a8287865b0e303abd7b9359`

Exact predecessor tree:

`ebb8293cc53edbc819295b482fbf8fc4886c80dd`

C03e-NE is documentation-only and records:

`NO_PRODUCTION_EXPECTED_DEVICE_SCHEDULING_INGRESS_OR_EXPECTED_REQUEST_SENDER_CHANNEL_OWNER_PROVEN / EXISTING_PRODUCTION_COMPOSITION_REMAINS_RECEIVER_ONLY / REQUESTER_RENDEZVOUS_TARGET_INTENT_NOT_ADMISSION_SCHEDULING_AUTHORITY / SOURCE_MATERIALIZATION_BLOCKED`

## 3. Selection result

C03e-NF selects:

`NO_PRODUCTION_EXPECTED_DEVICE_SCHEDULING_EVENT_SOURCE_PROVEN / REQUESTER_RENDEZVOUS_AUTHORITY_REMAINS_CANDIDATE_PUBLICATION_SCOPED / REPEATED_ADMISSION_EXPECTED_DEVICE_REMAINS_INJECTED_PREFLIGHT_EVIDENCE / SOURCE_MATERIALIZATION_BLOCKED`

The exact post-NE source contains no production event whose established authority contract can be reused as the producer of repeated-admission `expected_device_id` without widening or reinterpreting that authority.

Therefore C03e-NF authorizes no Rust source materialization.

## 4. Required authority law

A valid scheduling event source must establish all of the following before one logical `DeviceId` can enter the expected-device admission lane:

1. the logical `DeviceId` is selected by an authoritative production operation or lifecycle event, not by configuration convenience;
2. the event's existing authority semantics permit initiating one pre-authentication expected-device admission attempt for that exact device;
3. logical identity remains separate from current `TransportIdentity`, endpoint, IP, port, candidate, and path state;
4. the event is current at the point where admission scheduling is committed, or a separately selected currentness rule revalidates it;
5. revocation, retirement, ambiguity, unavailable authority, or stale authority fails closed;
6. the event does not derive requester identity, authorization, or target identity from PRWM correlation IDs;
7. the event does not fabricate `SessionId`, authentication PRWM request ID, verifier time, dispatcher custody, or channel ownership;
8. one event cannot silently authorize an unbounded admission loop, retry, replacement, reconnect, or replay.

C03e-NF finds no existing production source satisfying that complete law.

## 5. Requester target nomination is not the event

`RequesterRendezvousTargetIntent` contains one caller-nominated logical target `DeviceId`.

Its source contract explicitly states that:

- it carries no requester identity;
- possession is not authorization;
- it is not a current-registration fact;
- construction performs no registry validation, policy evaluation, provider mutation, or I/O.

The larger `RequesterRendezvousStartIntent` combines the already-authenticated requester session with the nominated target, but that composition is still non-authoritative until later validation and authorization stages succeed.

Therefore neither raw target nomination nor start-intent construction is expected-device admission scheduling authority.

## 6. Registry validation is not the event

The existing requester/rendezvous registry validation consumes the start intent and produces `RegistryValidatedRequesterRendezvousStart` only after point-in-time checks of requester currentness, target lookup, enrolled lifecycle, active membership, workspace relationship, and target identity preservation.

That carrier explicitly proves only point-in-time current-registry eligibility.

It does not prove:

- requester/rendezvous policy authorization;
- provider registration authority;
- candidate-publication authority;
- admission scheduling authority;
- transport readiness;
- network reachability;
- a lease, TTL, or perpetual-currentness guarantee.

A successful registry lookup/validation therefore cannot be upgraded into expected-device admission scheduling authority.

## 7. Requester/rendezvous policy authorization is operation-scoped

The existing dedicated requester/rendezvous policy stage evaluates exactly the requester/rendezvous-start capability and returns operation-specific authorized provenance.

That provenance remains bounded to requester/rendezvous semantics.

No exact source contract states that successful `RequesterRendezvousStart` authorization also authorizes a repeated-admission scheduler to enqueue the target `DeviceId`.

C03e-NF therefore does not reinterpret requester/rendezvous policy authorization as remote-session admission scheduling authority.

## 8. Provider registration is not sufficient scheduling provenance

The existing DI -> DP -> DK -> DN requester/rendezvous composition performs, in order:

1. current-registry validation;
2. requester-aware policy-source resolution;
3. dedicated requester/rendezvous-start policy authorization;
4. one private requester/rendezvous provider registration mutation.

The concrete in-memory provider records one current requester-session / expected-publisher-device pair.

Its own contract states that `register_current(...)` does not grant publication authority by itself.

A later provider authorization may return one `AuthorizedRequesterRendezvous`, but that grant is explicitly scoped to one candidate-publication execution attempt and later candidate-publication composition must repeat currentness checks before reachability commit.

No provider record, registration success, or candidate-publication grant currently states that it authorizes a pre-auth expected-device admission attempt.

Promoting that authority would be a new cross-operation authority binding, not a source read.

## 9. Candidate publication is explicitly excluded

C03e-NE expressly prohibits inferring admission scheduling provenance from candidate publication.

C03e-NF preserves that boundary.

Candidate publication can establish operation-specific publication/reachability state only under its own authority law. Candidate data, endpoint data, reachability state, relay data, or publication completion must not become the identity or authorization source for expected-device scheduling.

## 10. Repeated-admission `expected_device_id` is a consumer input

The existing repeated real-admission supervisor receives typed requests containing an `expected_device_id` together with other already-typed admission inputs.

Historical integration checkpoints preserve the following law:

- pre-auth `expected_device_id` is scheduling/preflight evidence only;
- post-authenticated `session_owner.logical_device_id()` is authoritative for active-worker insertion identity;
- the supervisor does not manufacture scheduling intent from the authenticated result;
- the expected request is supplied from an external sender/producer side that remains unmaterialized in production composition.

This proves consumer semantics, not producer provenance.

## 11. Real-admission transaction does not source scheduling intent

The existing expected-device real remote admission transaction accepts an already-supplied expected logical `DeviceId`.

It then uses fresh current registry authority to resolve the device's current `TransportIdentity` for lower-transport acceptance and repeats current-registry checking around logical authentication.

That transaction deliberately preserves:

`logical DeviceId != TransportIdentity != endpoint/IP`

The transaction validates and consumes scheduling intent; it does not originate that intent.

Its fresh registry reads therefore cannot be used backward as the producer of the expected logical device.

## 12. Configured peer identity is not the event

Production peer/bootstrap identity and repeated-admission expected-device scheduling identity are distinct provenance domains.

A configured or pre-bootstrap logical peer identity cannot be promoted into an arbitrary sequence of expected-device admission events.

Likewise, a current `PeerConnectivityIdentity`, `TransportIdentity`, certificate identity, bind address, IP, port, endpoint, candidate, or reachability observation does not prove that the logical device has been authoritatively scheduled for one admission attempt.

## 13. Correlation and session values remain separate

C03e-NF does not select or infer production provenance for:

- `SessionId`;
- authentication PRWM `request_id`;
- candidate-publication PRWC `request_id`;
- verifier time;
- request/admission timing policy;
- dispatcher construction or per-request caller custody.

All request IDs remain transaction correlation only.

No request ID is requester identity, target identity, expected-device identity, authorization evidence, transport identity, or scheduling authority.

## 14. Sender/channel ownership remains unresolved

Because no authoritative scheduling event source is proven, C03e-NF cannot lawfully select a concrete producer-side channel owner.

C03e-NF therefore does not select:

- `mpsc::channel(...)` construction;
- capacity;
- sender clone policy;
- sender custody;
- send timing;
- backpressure policy;
- closed-channel behavior beyond already-existing receiver semantics;
- sender-drop ordering;
- retry/replay behavior;
- shutdown coupling.

Those decisions remain downstream of a separately selected scheduling-event authority binding.

## 15. Fail-closed conclusion

The repository has multiple values that mention or carry a logical target/expected device, but none may be substituted for the missing production scheduling event merely because the type is `DeviceId`.

The following are explicitly non-equivalent:

- requester-nominated target;
- registry-validated target;
- requester/rendezvous policy-authorized target;
- requester/rendezvous provider record;
- candidate-publication publisher identity;
- candidate/reachability endpoint state;
- configured production peer;
- current transport identity;
- repeated-admission expected-device consumer field;
- authenticated-session logical identity after admission.

Shared type representation does not imply shared authority provenance.

## 16. Selected blocking boundary

C03e-NF selects the next missing prerequisite as:

`PRODUCTION_EXPECTED_DEVICE_SCHEDULING_EVENT_AUTHORITY_BINDING`

A later separately gated documentation-only checkpoint may audit/select only the authority binding that permits one already-authoritative production operation/lifecycle result to produce one bounded expected-device scheduling event.

That successor must answer, before any source materialization:

1. which exact production operation/lifecycle owns the decision to schedule an admission attempt;
2. which authenticated principal/session and policy decision authorize that decision, if any;
3. which exact logical `DeviceId` is carried into the event and from which authoritative carrier it originates;
4. when current registry/revocation state must be checked;
5. whether the event is one-shot, consumable, cancellable, or invalidated on authority change;
6. how duplicate or concurrent scheduling for the same device is classified before the existing receiver-side duplicate guard;
7. where sender/channel ownership may later attach without creating an alternate authority path.

The successor must not materialize the event, sender, channel, SessionId, authentication request ID, timing source, dispatcher caller, or runtime activation unless separately selected afterward.

## 17. Exact NF scope

C03e-NF is documentation-only.

Permitted repository delta:

- exactly this one contract path.

Expected delta classes:

- zero Rust source changes;
- zero Kotlin/Android changes;
- zero Cargo manifest/lockfile changes;
- zero workflow changes;
- zero systemd/package/security/configuration changes;
- zero runtime/listener/network activation changes.

## 18. Explicit exclusions

C03e-NF does not perform or authorize:

- Rust source materialization;
- new scheduling-event carrier/source;
- new capability or policy rule;
- requester/rendezvous policy redesign;
- requester/rendezvous provider redesign;
- candidate-publication semantic widening;
- expected-request channel construction;
- sender ownership or sender cloning;
- request enqueue/send behavior;
- `SessionId` generation;
- authentication PRWM request-ID allocation or lane reuse;
- verifier-time construction;
- admission timing policy;
- dispatcher caller migration;
- worker/repeated-admission semantic changes;
- peer lookup/re-selection;
- transport selection;
- candidate selection;
- reachability mutation;
- direct dialing;
- retry/reconnect/rebind/rebootstrap/replacement;
- listener/readiness activation;
- `run()` or `main.rs` invocation changes;
- systemd/service/package mutation;
- credential/certificate/private-key/trust/RBAC mutation;
- database/schema/control-plane mutation;
- repository visibility/configuration mutation;
- deployment/restart/recovery activation;
- merge;
- PR close or ready-for-review conversion;
- branch deletion;
- history rewrite or force update;
- destructive cleanup of historical branches/evidence.

## 19. Identity and authorization invariants

Continue to preserve:

`PRW logical device/session identity -> registry/discovery -> current endpoint/candidates -> authenticated transport`

Logical device identity is not a static IP.

Transport-key rotation does not redefine logical device identity.

A `TransportIdentity` is transport evidence only.

An endpoint/IP/port is transient reachability only.

A PRWM `request_id` is correlation only.

A successful decode, registry lookup, provider lookup, transport accept, candidate publication, or correlation match is not silently upgraded into authorization.

Receiving devices remain enforcement points for protected remote operations.

## 20. Validation and closure requirements

C03e-NF may be closed only after all of the following are verified on the exact final NF head:

1. exact NE -> NF topology is ahead-only from exact NE head;
2. exactly one changed path exists and it is this contract;
3. no Rust/runtime/manifest/lockfile/workflow/Android/packaging/security path changed;
4. canonical Rust validation for the exact final NF head completes successfully;
5. path-filtered skipped workflows are recorded as `SKIPPED`, never PASS;
6. no Android PASS is claimed unless an exact-head Android workflow actually runs and succeeds;
7. immutable Google Drive audit evidence is published and byte-exact readback verified;
8. post-evidence branch/PR reads confirm the exact final head is unchanged;
9. the PR remains draft/open/unmerged.

## 21. STOP boundary

After C03e-NF durable closure, STOP.

Do not materialize a scheduling event or producer until the separately gated production expected-device scheduling-event authority binding has been explicitly selected from fresh exact-head evidence.
