# Phase 152 C03e-BM — Connectivity Path-Kind Provenance Authority Boundary Selection

Status: STAGED SELECTION

Gate target:
`C03E_BM_CONNECTIVITY_PATH_KIND_PROVENANCE_AUTHORITY_BOUNDARY_SELECTED`

## 1. Exact predecessor

Closed C03e-BL:
- branch: `phase-152-c03e-bl-candidate-id-production-authority-boundary-selection-staging`;
- head: `7958759e76bc110dafd1856e24bb40f17e847fd1`;
- tree: `2a1ff535af0b089d8da9f59313b0bc7d23a13ff4`;
- gate: `C03E_BL_CANDIDATE_ID_PRODUCTION_AUTHORITY_BOUNDARY_SELECTED`.

BL selected only the authority boundary for future `CandidateId` production and explicitly did not select a concrete allocator, path-kind classifier, persistence/restart semantics, networking activation, readiness, deployment or merge.

## 2. Repository audit result

The exact BL head contains no production `ConnectivityPathKind` classifier/provenance authority that can be reused without inventing topology semantics.

Current boundaries prove:

- `ConnectivityPathKind` is an explicit product enum: `LocalDirect`, `InternetDirect`, `Relay`.
- `ConnectivityPathKind::selection_rank()` orders already-classified paths; it does not classify them.
- `ConnectivityCandidate::new(...)` and BI `assemble_explicit_connectivity_candidate(...)` consume an already-selected path kind.
- `PeerConnectivityPlan` validates candidate uniqueness/anti-reuse and selects among reachable candidates; it does not infer path kind.
- Phase 141 NAT traversal consumes an existing `ConnectivityCandidate`; its ICE `Host` / `ServerReflexive` classes are protocol classes, not product path-kind authority.
- relay code accepts only an already-selected `Relay` candidate and does not classify a candidate into `Relay`.
- Android native uses fixed disposable `LocalDirect` / `InternetDirect` / `Relay` fixtures to validate deterministic selection.
- Android application maps authoritative selected-path codes to UI views and does not classify network topology.
- the materialized production bind-address source is a `SocketAddr`/endpoint provenance source only; it does not establish whether that endpoint is local-direct, internet-direct, or relay.
- reachability observations establish current reachability only and do not establish product path class.

Therefore BM must not reinterpret any current endpoint, ICE, relay, UI or reachability value as a production path-kind classifier.

## 3. Selected authority boundary

BM selects only this ownership boundary:

> Production `ConnectivityPathKind` provenance is a distinct, separately reviewed discovery/provider authority responsibility that occurs before explicit candidate assembly. The authority supplies an explicit product path classification together with independently sourced candidate identity and endpoint state.

Selected ordering:

```text
authoritative candidate-ID production/custody
    + authoritative path-kind provenance
    + authoritative endpoint provenance
    -> explicit candidate assembly
    -> authenticated candidate publication
    -> current-plan validation
    -> reachability observation / deterministic selection
```

The path-kind authority is responsible only for the product classification it supplies. It does not gain authority over identity, authentication, publication freshness, reachability, readiness or endpoint validity.

## 4. No inference from IP shape

BM explicitly forbids deriving `ConnectivityPathKind` merely from address shape or scope.

The following are insufficient by themselves to classify `LocalDirect`, `InternetDirect`, or `Relay`:
- IPv4 private/reserved/public ranges;
- IPv6 global/link-local/ULA shape;
- loopback;
- interface address family;
- `SocketAddr`;
- `ConnectivityEndpoint`;
- bound listener address;
- port number;
- hostname/DNS name.

Endpoint configuration/observation is not topology authority.

## 5. ICE/STUN separation

BM does not map protocol candidate classes directly to product path kinds.

In particular:
- ICE `Host` does not automatically mean `LocalDirect`;
- ICE `ServerReflexive` does not automatically mean `InternetDirect`;
- a STUN XOR-mapped address does not automatically mean `InternetDirect`;
- successful ICE connectivity does not retroactively determine product path classification.

Phase 141 remains protocol/reachability machinery only.

## 6. Relay separation

BM does not infer `Relay` merely because:
- a relay service exists;
- a relay route token exists;
- a relay provider handle exists;
- direct candidates are currently unreachable;
- fallback policy would prefer relay next.

A candidate may be assembled with `ConnectivityPathKind::Relay` only when a separately authoritative relay/discovery provenance source explicitly classifies it as a product relay path.

Phase 136/142 consume relay classification; they are not selected here as the classifier.

## 7. Reachability and selection separation

`ReachabilityObservation` remains current provider observation only.

A candidate becoming `Reachable` or `Unreachable` does not change its path kind.

`PeerConnectivityPlan::selected_path()` remains deterministic ordering over already-classified, already-observed candidates:

```text
LocalDirect -> InternetDirect -> Relay -> Offline
```

Selection rank is not classification authority.

## 8. Endpoint provenance remains independent

Closed BC/BD/BE/BF endpoint work remains distinct:
- observed `SocketAddr` can be projected into `ConnectivityEndpoint`;
- production bind-address input can be selected/materialized;
- endpoint validation remains exact IP/port validation.

None of those steps establishes path-kind provenance.

A single endpoint may be reachable through different topological contexts across environments; BM therefore does not hard-code a path kind from the bind source alone.

## 9. Candidate-ID separation

Closed BL remains authoritative for the candidate-ID production boundary.

BM does not derive path kind from `CandidateId`, high-water state, candidate numeric ordering or candidate allocation provenance.

Candidate ID remains plan-scoped correlation only.

## 10. Authentication/publication separation

Authenticated candidate publication proves current publisher/session/transport provenance for the publication object; it does not by itself prove that a supplied `ConnectivityPathKind` was classified by an authoritative topology source.

Publication freshness/currentness and durable CAS remain separate verifier-owned concerns.

A future concrete path-kind source must be deliberately composed into the production input path before candidate assembly/publication. BM does not select that composition yet.

## 11. Exact non-authorities

The following are explicitly **not** selected as production path-kind authority:
- IP range tables or string parsing;
- `SocketAddr` / `ConnectivityEndpoint`;
- production bind-address environment input;
- `CandidateId` or candidate high-water;
- `ConnectivityPathKind::selection_rank()`;
- `PeerConnectivityPlan::selected_path()`;
- `ReachabilityObservation`;
- STUN response shape;
- ICE `Host` / `ServerReflexive` classes;
- ICE selected pair success;
- relay availability or fallback state;
- relay route tokens/provider handles;
- Android UI/native selection fixtures;
- `DeviceId`, `TransportIdentity`, `SessionId`, PRWM request IDs;
- publication freshness tokens;
- live-owner fencing generations.

## 12. Explicit non-selections

BM does not select or materialize:
- a concrete path-kind classifier implementation;
- interface enumeration or route-table inspection;
- LAN/subnet membership policy;
- public-routability tests;
- cloud-provider metadata classification;
- STUN/ICE-to-product-kind mapping;
- relay allocation/provider activation;
- endpoint discovery beyond already-closed endpoint seams;
- candidate-ID allocator/custody;
- candidate publication wire/rendezvous protocol;
- reachability observation generation changes;
- registry/provider/database mutation;
- Agent `main.rs` activation;
- readiness/process-exit changes;
- retry/reconnect/rebind/rebootstrap/replacement;
- systemd/host/firewall/NAT/route/DNS/TUN/TAP mutation;
- deployment/restart/recovery;
- merge.

## 13. Next materialization rule

A source-materialization successor to BM is allowed only if an authoritative product topology/discovery source can be identified from existing project architecture or separately selected without pretending that IP/ICE/STUN/relay state is equivalent to product path classification.

If no concrete authority exists, the next work must remain a bounded audit/selection seam rather than fabricate a classifier.

A future source materialization must preserve:
- explicit path-kind input before candidate assembly;
- no inference from endpoint address shape alone;
- no direct ICE-class-to-product-kind mapping by assumption;
- no reachability-to-classification conflation;
- no weakening of authenticated publication or current-plan validation;
- no production networking activation merely from classification.

## 14. Identity and security invariants

BM preserves:
- `DeviceId` / authenticated PRW session identity as logical identity;
- `TransportIdentity` as lower-transport certificate identity only;
- `CandidateId` as plan-scoped candidate correlation only;
- `ConnectivityEndpoint` as transient endpoint/configuration state only;
- `ConnectivityPathKind` as explicit product path classification only;
- `SessionId` as authentication correlation only;
- PRWM request IDs as message correlation only.

Path-kind classification is not authentication, authorization, membership, publication freshness/currentness, reachability, public-routability proof, liveness or readiness evidence.

## 15. Exact intended BL -> BM scope

The final BM branch must differ from closed BL only by this contract:

`contracts/DESKTOP_FUNCTIONAL_MANAGEMENT_SLICE_C03E_BM_CONNECTIVITY_PATH_KIND_PROVENANCE_AUTHORITY_BOUNDARY_SELECTION_STAGING.md`

Any Rust/Kotlin source, Cargo manifest/lockfile, workflow, Agent, remote-bridge implementation, registry/provider, packaging/systemd, networking or deployment change blocks BM closure.

## 16. Validation requirements

BM can close only after:
- exact BL predecessor lineage remains unchanged;
- exact BL -> BM compare is one docs-only path;
- every automatically triggered workflow reaches a terminal non-failing verdict;
- immutable audit is stored only inside project folder `136SuugnComWa-CRGedjNfphubxleUiDQ` and raw-readback verified;
- rolling Drive evidence passes fresh predecessor guard, append-only prefix proof and raw post-write verification;
- PR body moves `STAGED -> CLOSED` only after Drive verification;
- PR remains draft/open/unmerged;
- final GitHub/Drive race checks remain clean.

No classifier implementation, candidate-ID allocator, production networking, readiness or deployment materialization is authorized merely by BM closure.

Gate target remains:
`C03E_BM_CONNECTIVITY_PATH_KIND_PROVENANCE_AUTHORITY_BOUNDARY_SELECTED`
