# Private Remote Workspace Linux Authenticated Connection Wrapper Contract

Version: `0.1.0`

Status: Phase 059 typed authenticated connected-stream ownership

## Scope

Phase 059 wraps an already-existing Linux Unix-domain connected stream/file-descriptor owner only after the Phase 058 same-effective-UID authorization succeeds.

It creates no socket and owns no listener or filesystem-path lifecycle.

## Construction ordering

Construction is strict:

1. receive ownership of an already-connected stream value implementing `AsFd`;
2. run Phase 058 `SO_PEERCRED` same-effective-UID authorization against that stream;
3. only on success create `AuthenticatedLocalLinuxConnection<S>` containing both the stream and the immutable authorization token;
4. on failure return the stream together with the bounded authorization failure so the caller retains explicit disposal ownership.

The constructor performs no application-protocol read or write.

## Type boundary

The wrapper has no public unchecked constructor.

Mutable stream access is available only after successful construction and only inside the crate-internal Linux platform boundary. The authorization token remains associated with the same owned stream instance.

The wrapper exposes read-only peer credential metadata and connection-state ownership for future runtime composition but does not itself evaluate command policy.

## Failure ownership

Authorization failure must not silently drop the caller's connected stream. The typed construction error returns:

- the original stream value;
- the bounded Phase 058 authorization error.

This allows a future accept loop to close/drop the rejected stream explicitly without ever reading application bytes.

## Test ordering proof

A Linux test may write sentinel application bytes from the opposite endpoint of an anonymous `UnixStream::pair()` before wrapper construction. After successful authorization, those bytes must remain unread and be recoverable exactly from the wrapped stream. This proves authorization does not consume application bytes.

No filesystem socket pathname is created.

## Forbidden interpretation

Phase 059 does not authorize or implement:

- filesystem-backed Unix socket bind/listen/accept/connect;
- XDG runtime-directory or pathname mutation;
- stale-socket cleanup;
- command-policy evaluation;
- the Phase 052 bounded application loop;
- service activation;
- network/DNS/TUN mutation;
- database changes;
- private-key operations;
- deployment.
