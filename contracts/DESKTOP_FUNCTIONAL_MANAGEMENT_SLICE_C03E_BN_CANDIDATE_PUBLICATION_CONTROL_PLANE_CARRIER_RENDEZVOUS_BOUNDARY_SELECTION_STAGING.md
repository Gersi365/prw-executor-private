# Phase 152 C03e-BN — Candidate Publication Control-Plane Carrier / Rendezvous Boundary Selection

Status: STAGED SELECTION

Gate target:
`C03E_BN_CANDIDATE_PUBLICATION_CONTROL_PLANE_CARRIER_RENDEZVOUS_BOUNDARY_SELECTED`

## 1. Exact predecessor

Closed C03e-BM:
- branch: `phase-152-c03e-bm-connectivity-path-kind-provenance-authority-boundary-selection-staging`;
- head: `a3f2e443d3d001ed3d8eba77ff61ba322a3c29eb`;
- tree: `1c647328f1c9800361e32c3a5822fda1a26b29b7`;
- gate: `C03E_BM_CONNECTIVITY_PATH_KIND_PROVENANCE_AUTHORITY_BOUNDARY_SELECTED`.

BM selected only the authority boundary for future product path-kind provenance. It did not select a classifier implementation, candidate-ID allocator, publication wire/rendezvous protocol, production networking, readiness, deployment or merge.

## 2. Repository audit result

The exact BM head contains the semantic publication object and admission/commit machinery, plus generic control/data-plane frame envelopes, but no complete production candidate-set signaling/rendezvous protocol.

Existing boundaries are deliberately distinct:

- `AuthenticatedCandidatePublication` is an in-process semantic snapshot containing a current authenticated publisher-session snapshot, exact peer identity, and already-constructed candidate vector. It is not a wire message.
- `publish_current_candidates(...)` validates current publisher/session/transport provenance and complete candidate-set semantics. It does not serialize, route, rendezvous, dial, or allocate request/session identifiers.
- `ProductionReachabilityOwner::commit_candidate_publication(...)` performs current requester/publisher admission, freshness validation, staged plan refresh, replacement freshness issuance and durable compare-and-commit. It does not define how the candidate vector arrives from another device.
- Phase 129 `prw-control-transport` owns outbound TCP + TLS 1.3 with ALPN `prw-control/1` and bounded `PRWC` frames. Its message kinds are generic envelopes; semantics explicitly remain above the transport layer.
- Phase 129 TLS success is server transport authentication only. It does not itself authenticate a PRW device session or authorize candidate publication.
- Phase 139 explicitly locks candidate exchange/signaling to the authenticated control plane and keeps that plane separate from the mesh data plane.
- Phase 140 `prw-remote-transport` / `PRWM` is the QUIC mesh/data-plane control-stream envelope after mesh transport establishment. It is not the Phase 129 candidate-rendezvous carrier merely because it can carry bounded control frames.
- `reachability_freshness_wire.rs` defines `PRWF` freshness-token delivery/resynchronization over existing `PRWM` framing. Those messages carry freshness state only; they do not carry a candidate vector and must not be reinterpreted as candidate-set publication/rendezvous.
- `session_auth_wire.rs` and capability request wire adapters concern mesh application-session/capability boundaries and do not define pre-mesh candidate exchange.
- `prw-control-plane` contains typed enrollment/session/live-owner semantics but no production candidate publication action or candidate-set routing/rendezvous schema.

Therefore the carrier plane is architecturally selected, but exact candidate publication payload, logical authentication binding, routing/rendezvous semantics, request correlation custody and server/broker dispatch remain unselected.

## 3. Selected carrier boundary

BN selects only this carrier boundary:

> Candidate-set signaling required to establish or refresh remote reachability belongs on the existing Phase 129 control plane selected by Phase 139, not on the mesh data plane whose establishment depends on those candidates.

The selected layering is:

```text
current authenticated PRW control-plane session context
    -> candidate-publication application semantics
    -> bounded Phase 129 control-plane payload/envelope
    -> control-plane coordination/rendezvous service
    -> authenticated recipient/requester-side admission
    -> publication freshness/current-plan validation
    -> durable reachability commit
    -> later ICE/QUIC data-plane establishment
```

This is a dependency-order boundary, not a wire schema or deployment selection.

## 4. Phase 129 transport is carrier only

`prw-control-transport` remains authoritative only for its existing transport mechanics:
- outbound TCP endpoint supplied by configuration;
- TLS 1.3;
- ALPN `prw-control/1`;
- explicit trust anchors/server authentication;
- bounded `PRWC` frame encoding/decoding;
- non-zero request-id representation requirement;
- message-kind envelope.

BN does not make Phase 129 TLS success equivalent to logical device/session authentication.

A candidate publication must be bound to current authenticated PRW session identity through an upper semantic/authentication layer before it can be accepted as an `AuthenticatedCandidatePublication` or committed.

## 5. Data-plane non-selection

BN explicitly does not use `PRWM/prw-mesh/1` as the candidate-set rendezvous carrier merely because:
- `ControlFrame` exists there;
- freshness token delivery currently has a `PRWF` codec there;
- capability request/response adapters use an established mesh control stream.

Candidate exchange is an input required before a usable mesh path may exist. Routing the initial candidate publication through an already-established mesh path would create the wrong dependency direction.

Future post-establishment optimization may require a separate compatibility/ordering decision; BN does not select one.

## 6. Publication semantic object is not wire authority

`AuthenticatedCandidatePublication` remains an in-process semantic representation after current publisher/session/transport validation.

BN does not expose its Rust memory layout as a wire format and does not select:
- field byte order/encoding;
- candidate count encoding;
- candidate-ID encoding on wire;
- IP address family/tag encoding;
- path-kind numeric tags;
- freshness-token placement;
- protocol magic/version;
- request/response/error operation registry.

A future wire codec must be separately bounded/versioned and must decode into typed inputs that still pass existing semantic admission.

## 7. Rendezvous/target routing remains distinct from identity

BN does not select how the control-plane coordination service locates or routes a publication to a requesting peer.

In particular, it does not turn any of these into rendezvous authority:
- `SocketAddr` / `ConnectivityEndpoint`;
- `CandidateId`;
- `ConnectivityPathKind`;
- `TransportIdentity` alone;
- PRWC/PRWM request-id;
- `SessionId`;
- live-owner fence;
- publication freshness token;
- relay route token/provider handle.

Logical routing must remain anchored in current authenticated PRW logical identity/workspace semantics. Exact expected-device request/scheduling provenance remains separately gated.

## 8. Request-id and SessionId custody remain unselected

Phase 129 and PRWM frame constructors require non-zero request identifiers, but repository audit does not establish production allocator/uniqueness/restart semantics for those IDs.

`SessionId` likewise remains authentication correlation only and has no production allocator selected by BN.

Therefore BN does not select:
- monotonic/random request-id generation;
- cross-connection uniqueness;
- restart persistence;
- database allocation;
- deriving request-id from `CandidateId`, `SessionId`, device identity or timestamps;
- using request-id as authentication/currentness evidence.

The future candidate publication wire must consume authoritative correlation identifiers once their custody is separately selected.

## 9. Freshness separation

Candidate publication freshness remains verifier-owned and separate from transport/request correlation.

A control-plane message carrying candidates does not become current merely because:
- it arrived over TLS;
- its PRWC request-id is new/non-zero;
- it belongs to a currently open TCP connection;
- the publisher session was once authenticated.

Current authenticated publisher/requester admission plus exact verifier freshness and durable compare-and-commit remain authoritative at commit time.

## 10. Candidate component provenance remains separately gated

BN does not solve the concrete production sources selected only as boundaries in BL/BM.

Before candidate assembly/publication, a future production path still requires deliberate authoritative provenance for:
- `CandidateId` production/custody;
- `ConnectivityPathKind` classification;
- endpoint discovery/provenance.

The wire carrier must not manufacture or infer those values.

## 11. No hostname/DNS or network activation implication

Selecting the Phase 129 control plane as carrier does not select:
- candidate endpoint DNS discovery;
- interface enumeration;
- public-routability testing;
- STUN/ICE/TURN activation;
- QUIC dialing/listening;
- relay allocation;
- firewall/NAT/route/TUN/TAP mutation.

Candidate signaling is coordination metadata only until separately validated and activated by later authorized stages.

## 12. Exact non-authorities

The following are explicitly not selected as candidate publication/rendezvous authority:
- raw Phase 129 TLS connection existence;
- `PRWC` envelope request-id;
- `PRWM` envelope request-id;
- `PRWF` freshness wire message;
- mesh capability request wire adapter;
- Rust struct layout of `AuthenticatedCandidatePublication`;
- `PeerConnectivityPlan` itself;
- candidate endpoint address/port;
- candidate ID/high-water;
- path-kind selection rank;
- STUN/ICE transaction/foundation state;
- relay tokens/handles;
- Android UI/native fixtures.

## 13. Explicit non-selections

BN does not select or materialize:
- exact candidate publication wire codec/schema/version/magic;
- exact candidate publication operation codes;
- control-plane server/broker implementation;
- listener/acceptor or production endpoint;
- logical control-plane authentication wire composition;
- expected-device rendezvous/scheduling protocol;
- requester-to-publisher routing table/provider;
- request-id allocator/custody;
- `SessionId` allocator/custody;
- retries/idempotency/deduplication/ack timing;
- candidate-ID allocator;
- path-kind classifier;
- endpoint discovery provider;
- STUN/ICE/TURN/relay production activation;
- registry/provider/database mutation;
- Agent activation/readiness;
- host/systemd/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## 14. Next materialization rule

A source-materialization successor to BN may proceed only after the exact candidate-publication wire semantic inputs and correlation/authentication ownership are selected without inventing request/session ID custody or rendezvous identity.

A future wire selection must preserve:
- Phase 129 control-plane carrier direction selected by Phase 139;
- logical authenticated PRW session provenance above transport TLS;
- exact bounded/versioned decoding before semantic admission;
- no client-controlled target retargeting around current workspace/device checks;
- independent verifier freshness/currentness at commit time;
- no reuse of PRWM mesh/data-plane framing as initial rendezvous by assumption;
- no production listener/network activation merely from codec materialization.

If exact requester/publisher routing or request/session correlation ownership remains unavailable, work must stay at bounded audit/selection rather than fabricate it.

## 15. Identity and security invariants

BN preserves:
- `DeviceId` / authenticated PRW session identity as logical identity;
- `TransportIdentity` as lower transport certificate identity only;
- `CandidateId` as plan-scoped candidate correlation only;
- `ConnectivityEndpoint` as transient endpoint/configuration state only;
- `ConnectivityPathKind` as product path classification only;
- `SessionId` as authentication correlation only;
- frame request IDs as message correlation only.

Control-plane transport success, request correlation, candidate configuration and candidate publication are not themselves authentication, authorization, publication currentness, reachability, public-routability, liveness or readiness evidence.

## 16. Exact intended BM -> BN scope

The final BN branch must differ from closed BM only by this contract:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BN_CANDIDATE_PUBLICATION_CONTROL_PLANE_CARRIER_RENDEZVOUS_BOUNDARY_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, Agent implementation, registry/provider, packaging/systemd, networking or deployment change blocks BN closure.

## 17. Validation requirements

BN can close only after:
- exact BM predecessor lineage remains unchanged;
- exact BM -> BN compare is one docs-only path;
- every automatically triggered workflow reaches a terminal non-failing verdict;
- immutable audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-readback verified;
- rolling Drive evidence passes fresh predecessor guard, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No publication codec, rendezvous service, ID allocator, production networking, readiness or deployment materialization is authorized merely by BN closure.

Gate target remains:
`C03E_BN_CANDIDATE_PUBLICATION_CONTROL_PLANE_CARRIER_RENDEZVOUS_BOUNDARY_SELECTED`
