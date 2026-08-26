# Phase 152 C03e-BO — Candidate Publication Control-Plane Correlation + Authenticated Sender Identity Selection Staging

Status: STAGED

Gate: `C03E_BO_CANDIDATE_PUBLICATION_CONTROL_PLANE_CORRELATION_SENDER_IDENTITY_SELECTED`

## Purpose

Select the narrow existing control-plane semantics that a future authenticated candidate-publication carrier may reuse for message correlation and authenticated sender identity, without selecting recipient routing, a publication codec, a broker, an identifier allocator, or any production activation.

This checkpoint is selection-only and docs-only. It does not change Rust/Android/Desktop source behavior.

## Canonical predecessor

- C03e-BN branch: `phase-152-c03e-bn-candidate-publication-control-plane-carrier-rendezvous-boundary-selection-staging`
- exact predecessor head: `6600f0bdbfad03b8b89517935a4df50e1e66ee7d`
- exact predecessor tree: `5390e4bf7673da147b4fa11ebed9b323890b3b42`

## Selected boundary

A future candidate-publication message carried over the authenticated control plane may preserve the same separation already present in the control-plane/capability path:

1. PRWM/control-frame `request_id` is message correlation only.
2. The authenticated PRW session's `DeviceId` is the logical sender identity input.
3. `TransportIdentity` remains lower-transport certificate identity and is not promoted into PRW logical identity.
4. Request correlation and authenticated sender identity remain separate dimensions.
5. Current registry/session/transport/policy evaluation remains authoritative where protected operations require it; publication transport does not create authorization by itself.

The selected semantic shape is therefore deliberately abstract:

```text
correlation_request_id
+ authenticated_sender_device_id
+ authenticated candidate-publication semantic payload
-> candidate-publication control-plane carrier context
```

This is a boundary selection only. It is not a wire format or runtime dispatcher.

## Exact source observations supporting the selection

At the canonical BN snapshot, the existing control-plane/capability path already establishes these invariants:

- authorized capability requests retain the incoming control frame's `request_id` as request correlation;
- the response path reuses the authorized request's request ID for the corresponding response correlation;
- authenticated device-session `DeviceId` is validated separately from lower-transport identity/current registry state before protected dispatch;
- `AuthorizedCapabilityRequest` keeps logical principal/session information, transport identity and request correlation as separate fields/concepts;
- the control frame is a bounded carrier and does not by itself define higher-level candidate-publication semantics.

BO selects only this already-existing separation pattern for the future candidate-publication carrier boundary.

## Explicit non-selections

BO does **not** select or materialize any of the following:

- recipient/target `DeviceId` semantics;
- rendezvous or discovery authority;
- routing table, route selection or peer lookup;
- broker, mailbox, queue, topic, fan-out or store-and-forward semantics;
- candidate-publication codec or wire schema;
- control-frame message-kind assignment for candidate publication;
- request-ID production, allocation, persistence, restart or uniqueness policy;
- `SessionId` production or allocation;
- `CandidateId` production, custody, reservation or `high_water + 1` policy;
- `ConnectivityPathKind` classifier/provenance authority;
- candidate publication freshness/currentness/coherence semantics beyond existing authenticated/current validation boundaries;
- durable persistence;
- production listeners, STUN/ICE/TURN/relay activation, TUN/TAP, route/NAT/DNS/firewall mutation;
- systemd/host activation, deployment, readiness promotion or Phase-154 activation.

In particular, no recipient/target is inferred from sender identity, transport identity, socket address, endpoint, candidate ID, request ID or session ID.

## Identity invariants preserved

- `DeviceId` / authenticated PRW session identity = logical identity.
- `TransportIdentity` = lower-transport certificate identity only.
- `SocketAddr` / `ConnectivityEndpoint` = transient network configuration/state only.
- `CandidateId` = plan-scoped candidate correlation only.
- `ConnectivityPathKind` = product path classification only.
- `SessionId` = authentication correlation only.
- PRWM/control-frame request ID = message correlation only.

No runtime/task/thread/controller/channel/lock/endpoint/candidate identifier becomes PRW identity through this checkpoint.

## Safety / staging verdict

This checkpoint is safe for Phase 152 because it is documentation-only and selects only an existing semantic separation already demonstrated by the canonical source snapshot. It performs no production networking or user-impacting mutation.

A later checkpoint must separately prove and select recipient/target authority, publication codec/carrier representation and any identifier-production authority before source materialization can safely advance.
