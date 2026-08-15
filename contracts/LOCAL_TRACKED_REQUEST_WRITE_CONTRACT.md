# Local Tracked Request Write Contract

## Status

Phase 033 locks provider-neutral ordering between Request frame construction, outstanding-request registration, and generic stream writing.

## Required ordering

For one typed local read-only Request:

1. build the complete validated Phase 031 Request frame;
2. register the request ID in the per-connection Phase 013 tracker;
3. only after successful registration, begin generic frame writing;
4. do not flush implicitly.

## Registration failure

If registration fails because the request ID is already outstanding or the bounded tracker is full:

- no Request byte may be written;
- existing tracker state remains unchanged;
- the caller receives a typed registration error.

## Write failure

If generic frame writing fails after registration:

- the request ID remains outstanding;
- the transaction does not roll back the tracker entry;
- the failure is treated as stream-state ambiguous because a prefix of the frame may already have been accepted by the writer;
- later runtime code must not silently reuse the same stream/request ID after this failure without an explicit connection-reset policy.

This conservative rule prevents tracker rollback from creating correlation reuse on a potentially desynchronized stream.

## Successful write

A successful write leaves the request ID outstanding until a later fully validated terminal response completes it through the existing decode-before-completion paths.

## Runtime boundary

Phase 033 does not create, bind, accept, connect, close, or reset a Unix socket. It does not implement connection poisoning state, retries, timeout/cancellation, flushing, command dispatch, or async concurrency. Those remain future runtime concerns.
