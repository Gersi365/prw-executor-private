# Private Remote Workspace Outstanding Request Contract

Version: `0.1.0`

Status: Phase 013 bounded per-connection request-tracking baseline

## Scope

This contract defines the pure in-memory state required to correlate concurrently outstanding local IPC Requests with their terminal Response or Error frames.

It builds on:

- Phase 007 non-zero request IDs;
- Phase 008 request/response envelopes;
- Phase 011/012 generic frame I/O boundaries.

Phase 013 does not create a connection loop, socket, task, thread, timeout, cancellation mechanism, or payload serializer.

## Per-connection bound

Maximum simultaneously outstanding requests on one local IPC connection:

`64`

The bound applies separately to each future accepted client connection.

The purpose is to prevent an authenticated but defective or hostile local client from creating an unbounded correlation-state set before connection-level backpressure and timeout policy exist.

The value is a protocol/runtime resource limit, not an authorization limit or capability count.

## Registration

Before a future client-side or connection-side state machine treats a Request as outstanding, its non-zero `LocalIpcRequestId` must be registered.

Registration fails when:

- the same request ID is already outstanding on that connection: `DuplicateRequestId`;
- 64 other request IDs are already outstanding: `TooManyOutstandingRequests`.

Duplicate detection is evaluated before capacity failure so an already-active ID remains explicitly classified as duplicate even when the tracker is full.

## Completion

One terminal Response or Error completes one outstanding request ID.

Completion removes the request ID from the tracker.

Completion fails with:

- `UnknownRequestId`

when the supplied ID is not currently outstanding.

This prevents a future connection layer from silently accepting an unsolicited, duplicated, or stale terminal response as though it belonged to a live request.

## Reuse

A request ID may be reused on the same connection only after its previous request has reached a terminal completion and has been removed from the outstanding set.

Phase 013 does not define a monotonic-ID requirement.

Request IDs remain correlation values only; they are not:

- secrets;
- authentication tokens;
- authorization capabilities;
- cryptographic nonces;
- durable object identifiers.

## Data structure boundary

The Phase 013 implementation uses a bounded in-memory collection and performs exact equality checks on typed `LocalIpcRequestId` values.

The public contract does not require a particular internal collection implementation, ordering, hashing strategy, or persistence format.

Outstanding request state is connection-local and ephemeral.

## Timeouts and cancellation

Phase 013 does not remove requests automatically.

It does not define:

- request timeout duration;
- cancellation messages;
- connection-close cleanup API;
- retry semantics;
- idempotency semantics;
- whether a future command is safe to retry.

Those rules require command/runtime semantics and remain explicit future work.

## Backpressure

Reaching the outstanding-request limit is a fail-closed resource condition.

A future runtime must not bypass the limit by dropping an arbitrary live request ID or overwriting an existing one.

How the runtime surfaces backpressure to the calling UI/client remains deferred.

## Forbidden interpretation

Phase 013 does not authorize or implement:

- socket connection loops;
- concurrent thread/task execution;
- command execution or dispatch;
- shell/PTY execution;
- file/network/DNS mutation;
- privileged-helper invocation;
- account authentication;
- cryptographic operations;
- systemd activation;
- database changes;
- deployment.
