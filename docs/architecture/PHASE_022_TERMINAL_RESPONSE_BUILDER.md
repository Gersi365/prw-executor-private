# Phase 022 — Terminal Response Frame Builder

## Objective

Add the encode-side counterpart to the Phase 020 terminal-response validator. A caller supplies only request ID, typed response status, and opaque body bytes; the builder derives every other protocol property through existing validated components.

## Construction sequence

1. reject a command-specific body larger than 1,048,574 bytes;
2. encode the two-byte status prefix using Phase 016;
3. append the opaque command-specific body;
4. build a bounded `LocalIpcPayload`;
5. derive `Response` or `Error` through the Phase 020 status-to-kind mapping;
6. create a current-version `LocalIpcFrameHeader` using the existing request ID and payload length;
7. couple header and payload through `LocalIpcFrame::new()`.

## Why no kind argument

Allowing callers to provide both status and message kind would recreate the invalid combinations that Phase 020 rejects. Phase 022 therefore makes inconsistent terminal frames unrepresentable through its builder API.

## Bound

The global payload limit is 1 MiB. Two bytes are mandatory status prefix, leaving exactly 1,048,574 bytes for command-specific body data.

The body bound is checked before frame construction. The existing payload/header/frame constructors still remain authoritative lower-level guards.

## Validation symmetry

Tests feed builder output back through the Phase 020 validator. Both success and all current non-success statuses must round trip with the same request ID/status.

## Runtime boundary

This phase creates only in-memory Rust objects. It does not write a frame to a stream, open a socket, dispatch a command, touch request-tracker state, add a dependency, or activate a service.

## Next bounded step

After validation, `GetAgentStatus` can use this generic builder with its existing Phase 018 five-byte body to construct a complete terminal response frame without duplicating status-prefix logic.
