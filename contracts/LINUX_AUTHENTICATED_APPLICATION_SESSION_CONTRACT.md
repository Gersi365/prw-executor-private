# Private Remote Workspace Linux Authenticated Application Session Contract

Version: `0.1.0`

Status: Phase 060 authenticated bounded application-processing composition

## Scope

Phase 060 composes the Phase 059 authenticated Linux connection wrapper with the Phase 052 bounded provider-neutral application loop and the existing `LocalServerConnectionState`.

It owns no listener or filesystem pathname lifecycle.

## Construction boundary

`AuthenticatedLocalLinuxSession<S>` may be constructed only from an already-created `AuthenticatedLocalLinuxConnection<S>`.

The session does not accept a raw stream or raw file descriptor. Therefore the Phase 058/059 same-effective-UID authentication boundary necessarily precedes session construction.

## Owned state

The session owns:

- the exact Phase 059 authenticated connection;
- one `LocalServerConnectionState` initialized healthy.

The authorization token remains associated with the same owned stream instance through the authenticated connection wrapper.

## Processing entry point

The session exposes a bounded processing method that accepts:

- a caller-supplied `PolicyEvaluator`;
- caller-supplied status and private-DNS snapshots;
- a caller-supplied non-zero Request budget.

It delegates to the Phase 052 bounded connection loop over the already-authenticated stream and its owned aggregate connection state.

## Duplex stream rule

For stream types where shared references implement both `Read` and `Write` (including Linux `UnixStream`), the session may create separate shared-reference reader/writer handles to the same already-authenticated stream for the duration of one synchronous Phase 052 call.

No stream clone, new connection, listener, or pathname operation is required.

## Authentication vs command policy

The Phase 059 wrapper proves only the locked local transport condition that the kernel-reported peer UID matched the Agent effective UID.

The caller-supplied `PolicyEvaluator` remains a separate command-capability decision surface. Phase 060 does not claim that this evaluator is automatically principal-bound merely because the transport peer passed same-UID authentication.

A later principal/policy-context binding design may further narrow this boundary.

## Failure and stop behavior

Phase 052 stop/error semantics remain authoritative:

- clean EOF is a normal stop;
- budget exhaustion is a normal resumable stop with no over-read;
- framing/Request failures poison inbound aggregate state;
- response-write failures poison response-write aggregate state;
- an unusable aggregate state rejects later processing before application I/O.

## Forbidden interpretation

Phase 060 does not authorize or implement:

- filesystem-backed Unix socket bind/listen/accept/connect;
- XDG runtime-directory or pathname mutation;
- stale-socket cleanup;
- listener/process lifecycle;
- automatic principal-to-policy binding;
- timers, concurrency, cancellation, or task/thread creation;
- service activation;
- network/DNS/TUN mutation;
- database changes;
- private-key operations;
- deployment.
