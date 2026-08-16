# Private Remote Workspace — Disposable Relay Protocol and Service Contract

Status: Phase 142 implementation lock
Date: 2026-08-16
Repository: `Gersi365/prw-executor-private`

## Purpose

Phase 142 implements a disposable/reference relay protocol and service around the already validated Phase 136 fallback/opacity boundary. The goal is to prove provider behavior, route isolation, bounded forwarding and end-to-end payload opacity without deploying a production relay or terminating the Phase 140 QUIC/TLS peer security association.

Phase 142 does not authorize production relay deployment, a public listener, TURN production allocation, DNS mutation, firewall/NAT/router/TUN/TAP/routes, production credentials, Agent restart/replacement or Android/Desktop distribution.

## Relationship to TURN

The Phase 139 standards decision remains authoritative: TURN RFC 8656 is the standards-based UDP relay mechanism available to Phase 141/142 traversal when direct connectivity fails.

Phase 142 additionally uses the explicit Phase 139 allowance for a **PRW relay service/provider around the Phase 136 opacity contract**. The disposable PRW reference provider is not a new cryptographic protocol and does not replace TURN as the standardized NAT-traversal relay option. It proves the PRW provider boundary and may later be backed by TURN or a deployed PRW relay transport after a production architecture/readiness decision.

No TURN authentication, MESSAGE-INTEGRITY primitive or allocation state is reimplemented from scratch in this phase.

## End-to-end security boundary

The relay sees only:

- bounded relay routing metadata;
- a Phase 136 `RelayRouteToken`;
- provider/session identifiers;
- opaque bytes that the caller has already protected using the Phase 140 end-to-end QUIC/TLS data plane.

The relay MUST NOT:

- terminate peer mesh TLS;
- receive a transport private key;
- receive a Phase 126 device private key;
- parse file/terminal/forwarding/application schemas;
- expose a decrypt API;
- reinterpret successful relay routing as device/session/capability authorization.

`OpaqueRelayFrame` remains the application-opacity boundary.

## Fallback-only requirement

A relay provider session accepts only a `RelaySessionSpec` that already passed Phase 136 construction from a Phase 135 selected `Relay` candidate.

The provider MUST NOT create a relay route from `LocalDirect`, `InternetDirect` or `Offline` state.

The authoritative selection order remains:

`LocalDirect -> InternetDirect -> Relay -> Offline`

## Disposable service ownership

Phase 142 introduces `crates/prw-relay-service`.

The initial service is deliberately in-memory/Sans-network for disposable validation. It owns no:

- UDP/TCP socket;
- async runtime;
- DNS resolver;
- process/shell execution;
- TUN/TAP;
- firewall/NAT/router/route mutation;
- filesystem/database persistence;
- production secret source.

This allows route/provider behavior to be validated deterministically before Phase 153/154 deployment/readiness work.

## Relay routing envelope

The PRW reference relay routing envelope is an application routing header only. It is not an encryption primitive.

Network byte order, exact header length 48 bytes:

1. magic — 4 bytes: `PRWR`;
2. protocol major — `u16`, value `1`;
3. protocol minor — `u16`, value `0`;
4. message kind — `u16`, value `1` for `Data`;
5. flags — `u16`, value `0`;
6. route token — exactly 32 opaque bytes;
7. payload length — `u32`.

Initial payload ceiling remains the Phase 136 `MAX_RELAY_FRAME_BYTES` value of 65,536 bytes.

The decoder MUST reject before payload interpretation/allocation:

- wrong magic;
- unsupported version;
- unknown kind;
- non-zero flags;
- all-zero route token;
- zero payload;
- payload length over the Phase 136 bound;
- truncated or trailing bytes inconsistent with the declared length.

The envelope parser may interpret routing metadata only. Payload bytes remain opaque.

## Route model

One disposable relay route is identified by one non-zero Phase 136 `RelayRouteToken` and the explicit relay endpoint already selected in the Phase 136 `RelaySessionSpec`.

Initial route constraints:

- at most 32 active routes;
- exactly two participants maximum per active route;
- participant handles are non-zero provider-local identifiers;
- duplicate participant handle fails closed;
- a participant may belong to only one route in one service instance;
- two participants on one route must target the same explicit relay endpoint;
- forwarding before a route has two participants fails closed;
- sender route token in a routing envelope must match the route bound to the authenticated provider handle;
- frames are never broadcast to another route;
- one receiver queue is bounded to 64 frames and 1 MiB total buffered opaque payload;
- queue-capacity failure must not silently drop an already accepted earlier frame.

The route token is routing metadata, not proof of PRW application authorization. Control-plane issuance/production authentication of route tokens remains deferred.

## Provider boundary

`DisposableRelayProvider` implements the existing Phase 136 `RelayBackend` interface for open/transmit/close and additionally exposes a bounded receive poll by provider handle for disposable data-flow validation.

Provider open:

- consumes only a validated `RelaySessionSpec`;
- registers the spec with the shared disposable service;
- returns one opaque provider handle;
- fails with no retained participant on capacity/route mismatch.

Provider transmit:

- encodes the already-bounded `OpaqueRelayFrame` into the fixed routing envelope;
- passes the encoded bytes to the disposable service;
- service validates only relay metadata and queues the exact opaque payload to the other participant;
- transmit never parses/decrypts the opaque payload.

Provider receive:

- returns one `OpaqueRelayFrame` reconstructed from the exact routed payload bytes;
- does not expose another participant's route metadata as application data.

Provider close:

- removes only the same provider handle;
- queued frames for that handle are removed;
- an empty route record is removed;
- closing one participant cannot close an unrelated route.

## Logging/secrets

The Phase 142 provider/service MUST NOT derive `Debug` output that exposes the raw 32-byte route token. Any debug representation containing route context must redact the token.

Opaque payload bytes MUST NOT be included in provider/service debug output.

## Required disposable validation

Phase 142 must prove at least:

1. fixed 48-byte envelope byte layout and round-trip;
2. malformed magic/version/kind/flags/token/length/truncation/trailing data rejection;
3. fallback-only `RelaySessionSpec` remains the provider admission type;
4. two providers sharing one disposable service can open the same route and transfer exact opaque bytes bidirectionally;
5. payload remains byte-for-byte unchanged;
6. route token mismatch fails before forwarding;
7. forwarding before pairing fails closed;
8. a third participant on one route is rejected;
9. unrelated routes cannot receive each other's frames;
10. queue frame/byte capacity is bounded and fail-closed;
11. close isolates/removes only the intended participant/route;
12. route token and opaque payload are absent from Debug output;
13. runtime source contains no socket/DNS/Tokio/process/TUN/firewall/route mutation API;
14. focused rustfmt/Clippy `-D warnings`/tests pass;
15. full workspace rustfmt/Clippy/tests/build pass;
16. no production state is changed.

## Phase 143 handoff

Phase 143 may consume the Phase 142 relay provider through a narrow transport path only after Phase 140 transport identity/session checks and Phase 130 current registry state are revalidated.

Relay success does not grant capabilities. Phase 143 must still enforce authenticated application-session and per-capability policy boundaries.

## Production boundary

Until Phase 154 explicit production approval, Phase 142 MUST NOT:

- deploy a public relay service;
- bind a production relay port;
- start persistent TURN/relay traffic;
- provision production relay route tokens or credentials;
- alter PowerCode service/network state;
- alter firewall/NAT/router/TUN/TAP/routes/DNS;
- terminate or proxy plaintext PRW application payloads;
- restart/replace the production Agent for relay integration.

Final target state:

`PHASE_142_DISPOSABLE_RELAY_PROVIDER_VALIDATED / FALLBACK_ONLY / OPAQUE_END_TO_END_PAYLOAD / NO_PRODUCTION_RELAY_SIDE_EFFECT / READY_FOR_PHASE_143`
