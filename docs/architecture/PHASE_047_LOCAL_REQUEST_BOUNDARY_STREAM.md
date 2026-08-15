# Phase 047 — Local Request-Boundary Stream

## Objective

Compose the Phase 046 frame-boundary distinction with the existing Request-specific decoder, without adding policy, response writing, or runtime transport.

## Data flow

`std::io::Read` → `read_frame_at_boundary()` → either clean EOF or one complete validated frame → existing `decode_local_command_request_frame()`.

The resulting typed outcome is:

- `CleanEof`; or
- `Request(LocalAgentRequestEnvelope)`.

## Error behavior

The existing `LocalAgentRequestStreamReadError` remains authoritative. Frame acquisition failures preserve `Read(...)`; Request-specific decoding failures preserve `Decode(...)`.

This prevents a future connection loop from treating partial frames or malformed Requests as orderly shutdown.

## Admission boundary

A decoded Request is not an authorization token. Policy evaluation remains downstream and crate-internal through the existing admission pipeline.

## Runtime boundary

No socket, file descriptor, peer credential, filesystem pathname, timer, concurrent task, authentication, service activation, DNS/network mutation, database work, private-key operation, or deployment is introduced.
