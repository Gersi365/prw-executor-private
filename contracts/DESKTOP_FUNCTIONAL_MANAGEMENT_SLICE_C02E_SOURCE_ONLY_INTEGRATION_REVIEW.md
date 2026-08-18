# Phase 152 C02e — Source-Only Dynamic Reachability Integration Review

Status: `PASS_STATIC_INTEGRATION_REVIEW / SOURCE_DESIGN_CHAIN_COHERENT / VERIFIER_OWNED_FRESHNESS_GATE_MANDATORY_UNIMPLEMENTED / EXACT_FRESHNESS_REPRESENTATION_UNSELECTED / WIRE_UNSELECTED / BUILD_GATE_CLOSED / NO_NETWORK_IO / NO_RUNTIME_ACTIVATION`

Review base head: `1c64d37b40203557efbe79ac089057cc306c3ad3`

Frozen predecessor C02d head: `857583b25ed1206317641a93fd8f927819c954d8`

## Purpose

This checkpoint reviews the complete C02e source/design chain after authenticated candidate provenance, transactional endpoint refresh, plan-lifetime candidate correlation, transport-identity rotation lifecycle and candidate-publication freshness semantics were separately locked.

It does not select or implement the missing freshness representation, candidate wire format, persistence transaction, socket path or runtime activation.

## Authoritative chain reviewed

The current source/design path is:

`AuthenticatedDeviceSession`

`-> WorkspaceDeviceRegistry current logical-device revalidation`

`-> exact current TransportIdentity revalidation`

`-> bounded AuthenticatedCandidatePublication derived from the authenticated publisher`

`-> mandatory verifier-owned exact-current freshness gate (semantics locked, representation unselected)`

`-> registry-current requester + same-workspace + exact publication/plan target admission`

`-> transactional PeerConnectivityPlan candidate refresh`

`-> plan-lifetime CandidateId correlation`

`-> Phase 141 Sans-I/O reachability observation`

`-> deterministic LocalDirect -> InternetDirect -> Relay -> Offline selection`

Every identity-bearing step remains separate from transient IP/port endpoint data.

## Logical identity and endpoint mobility

The chain preserves the product rule required for mobile network changes:

- `DeviceId` remains the logical enrolled-device identity;
- authenticated PRW session identity remains bound to the logical device/workspace/user/public identity;
- `TransportIdentity` remains a separately rotatable transport identity;
- candidate IP/port values remain transient endpoints only.

A Wi-Fi -> mobile-data -> different-Wi-Fi transition can therefore be represented as candidate replacement while `DeviceId` and the current `TransportIdentity` remain unchanged.

No static IP allowlist is used as device identity.

## Candidate publication provenance

`crates/prw-remote-bridge/src/candidate_reachability.rs` is intentionally unexported and source-only.

Its publication constructor:

1. revalidates the publisher's authenticated session through the current registry;
2. revalidates the exact presented current `TransportIdentity` for that publisher device;
3. derives `PeerConnectivityIdentity` from the authenticated publisher's own `DeviceId` plus current transport identity;
4. validates the complete bounded candidate vector before publication construction.

The caller therefore cannot supply arbitrary endpoint bytes under an unrelated target `DeviceId`.

The earlier main C02e gate statement that the staged semantic type existed only inside integration-test source is superseded only as a source-placement statement by the later semantic-adapter checkpoint. The security boundary is unchanged: the module remains absent from the production `prw-remote-bridge` module graph and is not a runtime or wire API.

## Requester / target admission ordering

Publication consumption preserves current-state checks before endpoint mutation:

1. requester authenticated session registry-current;
2. publisher/target authenticated session registry-current;
3. requester and publisher share the same current workspace;
4. publisher `DeviceId` equals the publication peer logical device;
5. publication peer exactly equals the target plan peer;
6. exact target `TransportIdentity` is still registry-current;
7. only then may candidate refresh validate and mutate transient endpoint state.

Stale membership, revocation, session-binding mismatch, cross-workspace use, retargeting or transport rotation therefore fail before plan mutation.

## Transactional candidate state

`PeerConnectivityPlan` owns candidate-vector validity and mutation.

The current C02e behavior preserves:

- maximum candidate bound;
- duplicate-ID and duplicate exact endpoint rejection;
- full validation before mutation;
- candidate observation reset to `Unknown` after successful refresh;
- failure preserving prior candidates, observations and candidate-ID high-water mark;
- exact retained-candidate ID stability;
- no rebinding of an ID to another path/endpoint;
- no reuse of a removed candidate ID within the same plan lifetime;
- newly introduced IDs above the plan's prior high-water mark.

This prevents delayed reachability correlation from aliasing a new endpoint through an old numeric candidate identifier.

## Transport-identity rotation lifecycle

Endpoint refresh and transport rotation are different transitions.

When `DeviceId + TransportIdentity` remains current, candidate endpoints may refresh in the existing plan.

When the registry rotates to a new `TransportIdentity`:

- logical `DeviceId` remains unchanged;
- the old plan becomes stale;
- old-transport publication fails current transport validation;
- new-transport publication has a different `PeerConnectivityIdentity` and cannot be applied to the old plan;
- no in-place peer-identity rebind is permitted;
- a replacement plan must be constructed for the same logical `DeviceId` plus new current `TransportIdentity`;
- old reachability observations/selected path/candidate state are not authorization evidence for the replacement plan.

## Freshness / replay gate

Identity provenance plus candidate-ID lifetime freshness is still insufficient to make a complete publication replay-safe.

The current freshness checkpoint correctly requires verifier-owned exact-current freshness state and one logical atomic transition:

`current authenticated peer + expected current freshness + valid bounded candidate set -> refreshed candidate state + advanced freshness state`

Stale, duplicate or replayed freshness state must fail before candidate mutation, and two updates racing from the same prior freshness state must not both commit.

However, current repository precedent does not determine the exact candidate-publication freshness representation or durable transaction authority. Therefore all of the following remain deliberately unselected:

- counter/generation type or width;
- initial generation value;
- nonce length/encoding;
- timestamp or replay window;
- restart/failover recovery semantics;
- persistence backend/schema;
- candidate application message kind;
- wire payload magic/version/layout.

Phase 129 generic `request_id` remains correlation only and must not be promoted into freshness authority.

Because this gate is mandatory but unimplemented, the source-only semantic adapter must remain unexported and production candidate signaling must remain fail-closed/unwired.

## Phase 141 boundary

Current `prw-nat-traversal` remains an observation/correlation layer, not an identity or publication authority.

Its remote candidate input already receives a typed `ConnectivityCandidate`; selected-pair output carries only the correlated existing `CandidateId` and a `ReachabilityObservation`.

It fails closed when:

- the selected upstream peer endpoint cannot be correlated to its configured remote candidate set;
- a reachability update references a candidate absent from the current `PeerConnectivityPlan`.

Phase 141 therefore cannot create a new authorized candidate from an observed IP/port, cannot assign logical identity, cannot validate publication freshness and cannot bypass the C02e admission chain.

## No parallel identity or discovery system

C02e reuses existing responsibilities:

- `prw-session` — authenticated PRW session identity;
- `prw-registry` — current workspace/device/transport authority;
- `prw-connectivity` — candidate state, observations and deterministic selection;
- `prw-nat-traversal` — Sans-I/O STUN/ICE protocol and candidate correlation;
- `prw-remote-bridge` — source-only composition seam for authenticated application semantics;
- Phase 129/139 control-plane architecture — future bounded candidate signaling transport location.

No second registry, endpoint identity system, static IP identity mechanism or parallel discovery authority is introduced.

## Integration conclusion

The C02e source/design chain is internally coherent through the boundary that can be derived safely from existing repository contracts.

The remaining production signaling gap is explicit rather than hidden: verifier-owned freshness is mandatory, but its exact representation and transaction authority are not yet selected by authoritative precedent.

Accordingly, no safe source/runtime step may serialize/export/activate candidate publication until that representation and atomic state authority are separately reviewed. The correct current behavior is fail-closed and unconfigured, not invention of production values.

## Validation boundary

This checkpoint is static integration review only.

No build, `cargo fmt`, Clippy, test, workflow dispatch, TCP/UDP I/O, ICE/STUN/TURN execution, QUIC connection/migration, PTY/process I/O, signing, Agent/bootstrap wiring, deployment, privileged mutation or Host Mirror source synchronization is performed.

## Next safe step

Preserve this source/design checkpoint and mutable Branch Evidence. Further C02e candidate signaling source must wait until authoritative repository work safely selects both:

1. the concrete verifier-owned freshness representation; and
2. the atomic transaction authority/durability semantics that make candidate-state replacement and freshness advance one fail-closed transition.

Until then, candidate wire/runtime integration remains intentionally closed.
