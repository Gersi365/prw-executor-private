# Private Remote Workspace Local Request-Boundary Stream Contract

Version: `0.1.0`

Status: Phase 047 provider-neutral request-boundary composition

## Scope

Phase 047 composes the Phase 046 frame-boundary reader with the existing Phase 031 Request-frame decoder. It remains generic over caller-supplied `std::io::Read` and owns no transport.

## Outcomes

The boundary-aware Request reader returns one of two successful outcomes:

- `CleanEof` when Phase 046 reports EOF before any byte of a new frame;
- `Request(LocalAgentRequestEnvelope)` when one complete frame is acquired and passes the existing Request-specific decoder.

## Error preservation

Phase 047 reuses the existing `LocalAgentRequestStreamReadError` taxonomy:

- Phase 046 frame-acquisition failures map to `Read(...)` without reclassification;
- a complete non-Request frame, malformed command payload, or unknown command maps to the existing `Decode(...)` error.

Clean EOF is never produced after any byte of a malformed/truncated frame has been consumed.

## Stream preservation

One successful `Request` outcome consumes exactly one frame. Bytes belonging to a following frame remain unread. Repeated calls may therefore yield multiple Requests and finally `CleanEof`.

## Security behavior

This layer performs only framing and Request decoding. It does not evaluate policy and does not imply that the Request is authenticated or authorized.

## Forbidden interpretation

Phase 047 does not authorize or implement:

- policy evaluation or success-response construction;
- Unix socket bind/listen/accept/connect/close;
- peer credentials or authentication;
- XDG runtime-path mutation;
- timers, concurrency, cancellation, or task/thread creation;
- systemd activation;
- network/DNS/TUN mutation;
- database changes;
- private-key operations;
- deployment.
