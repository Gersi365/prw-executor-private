# Private Remote Workspace Linux Agent Accept / Readiness Contract

Version: `0.1.1`

Status: Phase 069 security/readiness decision lock — corrected type-state strategy; accept implementation not yet activated

## Scope

Phase 069 locks the blocking/readiness and accepted-peer transition semantics for the Linux filesystem-backed PRW Agent listener created through Phases 062–068.

This contract is decision-only. It does not call `accept`/`accept4`, change socket flags in production source, read application bytes, start an accept loop, or activate the Agent bootstrap/service.

## Preconditions

Accept work begins only from a Phase 068 `ListeningAgentSocket` that transitively retains:

- the validated XDG runtime-root descriptor;
- the validated same-UID `0700` PRW runtime-directory descriptor;
- the live Phase 065 exclusive instance lock;
- the validated `agent.sock` filesystem identity and exact `0600` mode;
- a successfully completed kernel `listen` transition with caller-supplied backlog.

No raw pathname or unrelated file descriptor is an acceptable runtime entry point.

## Readiness decision

A production accept operation must not be callable on the plain Phase 068 `ListeningAgentSocket`.

The next implementation phase introduces a one-way type-state transition:

`ListeningAgentSocket -> AcceptReadyAgentSocket`

Only `AcceptReadyAgentSocket` may expose the one-shot accept operation.

The transition must:

1. read current descriptor status flags;
2. set `O_NONBLOCK` while preserving existing status flags;
3. read the status flags again;
4. require `O_NONBLOCK` to be present before returning the accept-ready type;
5. on transition failure, return the original `ListeningAgentSocket` together with a bounded readiness-transition error so existing cleanup ownership is preserved.

Reasoning:

- a single `accept` call on a blocking listener can suspend indefinitely when no client is queued;
- Phase 052 already establishes caller-bounded application work and does not authorize an unbounded transport wait;
- a nonblocking listener lets a future runtime scheduler decide when and how often to retry without embedding thread, timeout, polling, or busy-loop policy into the accept adapter;
- the explicit type-state proves that nonblocking readiness preparation occurred before accept;
- keeping the transition after Phase 068 avoids retroactively changing the validated Phase 067 bound-socket construction semantics.

This corrected strategy supersedes the earlier Phase 069 draft idea of adding nonblocking at Phase 067 socket creation time. The required runtime property is unchanged: accept is exposed only on a verified nonblocking listener.

## One-shot accept semantics

The accept-ready adapter performs at most one kernel accept attempt per call.

Possible outcomes are:

- one connection accepted and authenticated;
- no connection currently ready (`EAGAIN`/`EWOULDBLOCK`), represented as a normal typed readiness outcome rather than a fatal error;
- another accept failure, represented as a bounded transport error;
- accepted-peer authorization failure, represented as the existing bounded same-UID authorization classification.

The adapter must not spin, sleep, poll, block awaiting readiness, spawn a thread/task, or retry internally.

A future caller may invoke the one-shot operation again according to a separately reviewed runtime scheduling policy.

## Accepted descriptor flags

The listener is verified nonblocking, but the accepted connected stream must remain blocking for the current application pipeline.

The accept operation must request close-on-exec for the accepted descriptor and must not request nonblocking status for that accepted descriptor.

This preserves compatibility with the existing Phase 059/060 `std::io::Read`/`Write` processing semantics, which currently model blocking connected streams and do not classify `WouldBlock` as a normal application-processing stop.

The implementation phase must test the actual Linux runner behavior rather than rely on an unverified inheritance assumption.

## Mandatory authentication ordering

The Phase 064 accepted-connection ordering remains binding.

For every successfully accepted connected stream:

1. obtain ownership of the accepted descriptor;
2. convert/wrap it only inside the crate-internal Linux platform boundary;
3. immediately construct the Phase 059 `AuthenticatedLocalLinuxConnection`;
4. Phase 059 obtains Linux kernel `SO_PEERCRED` and requires peer UID exactly equal to the Agent effective UID;
5. on authorization failure, close/drop the rejected connected stream without reading any PRW application-protocol byte;
6. only an authenticated wrapper may be returned for later Phase 060 session construction and bounded Request processing.

A raw accepted connected stream must not be returned to provider-neutral application code.

Same-UID kernel transport authentication remains separate from principal/capability policy binding.

## Listener preservation

A successful accept transfers ownership only of the newly accepted connected descriptor.

`AcceptReadyAgentSocket` owns the Phase 068 `ListeningAgentSocket` and therefore transitively retains the Phase 065 instance-lock authority and validated socket-path lifecycle state.

A readiness miss or rejected peer must not close, rebind, chmod, unlink, or otherwise mutate `agent.sock`.

Explicit listener cleanup delegates through Phase 068 to the existing Phase 067 exact-identity cleanup.

## Error model

The implementation phase must distinguish at least:

### Readiness transition failures

- status-flag read failure;
- status-flag write failure;
- post-write status revalidation failure;
- nonblocking flag absent after revalidation.

These failures retain the original listening object.

### One-shot accept failures/outcomes

- no connection ready;
- kernel accept failure;
- same-UID peer-authorization failure.

Authorization failure must preserve the existing bounded Phase 058 error classification while ensuring the rejected stream is disposed before returning to the caller.

## Test requirements

The implementation phase must prove on Linux runner tests at least:

- the Phase 068 listening descriptor is not assumed accept-ready until the explicit transition succeeds;
- the transition produces a descriptor with `O_NONBLOCK` present;
- calling one-shot accept with no queued client returns the no-ready outcome rather than blocking;
- a local same-UID client can connect and be accepted;
- the accepted descriptor has close-on-exec semantics;
- the accepted stream is blocking even though the listener is nonblocking;
- Phase 059 same-effective-UID authorization occurs before any application byte is consumed;
- bytes written by the client before acceptance/authentication remain unread and recoverable only after successful authenticated wrapping;
- accepting one client leaves the accept-ready listener usable for a later client;
- listener cleanup and the Phase 065 instance lock remain governed by the existing Phase 067/068 lifecycle.

Tests may use only temporary runner-local Unix sockets and must clean them through the validated lifecycle.

## Deferred scheduling policy

Phase 069 intentionally does not select:

- `poll`, `ppoll`, `epoll`, async runtime, thread-per-listener, or another readiness engine;
- accept-loop batch size;
- wall-clock timeout;
- cancellation mechanism;
- connection concurrency limit;
- per-client worker/task model.

Those are runtime orchestration decisions and must not be smuggled into the one-shot accept primitive.

## Forbidden interpretation

Phase 069 does not authorize or implement:

- a production accept/accept4 call in this decision-only phase;
- an unbounded accept loop;
- busy polling;
- application-protocol reads/writes before same-UID authentication;
- principal/policy binding changes;
- Agent bootstrap wiring or systemd/service activation;
- TCP/abstract-socket fallback;
- network/DNS/TUN mutation;
- database changes;
- private-key operations;
- deployment.
