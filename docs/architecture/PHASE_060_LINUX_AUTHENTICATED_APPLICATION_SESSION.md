# Phase 060 — Linux Authenticated Application Session

## Objective

Make the validated Phase 052 bounded application loop consume a Phase 059 authenticated stream wrapper rather than a raw Linux stream at this platform integration boundary.

## Type composition

`AuthenticatedLocalLinuxSession<S>` owns:

- `AuthenticatedLocalLinuxConnection<S>`;
- `LocalServerConnectionState`.

Construction accepts only the authenticated connection wrapper and initializes a fresh healthy aggregate state.

## Processing

For stream types where `&S` implements both `Read` and `Write`, one synchronous processing call borrows the already-authenticated stream through shared references and delegates directly to `process_server_connection_with_budget()`.

The caller supplies the non-zero work budget, policy evaluator, and read-only snapshots. The session owns no timeout or scheduling policy.

## Security ordering

The only construction path is:

raw connected stream → Phase 059 wrapper constructor → Phase 058 kernel same-UID authorization → authenticated wrapper → Phase 060 session → Phase 052 application reads/writes.

Thus this adapter does not expose a raw-stream path into its application processing method.

## Policy boundary

Transport authentication and command policy remain distinct. Same-UID authorization does not manufacture or bind a `PolicyEvaluator`; the evaluator remains caller-supplied in Phase 060.

## Test strategy

Linux tests use only anonymous `UnixStream::pair()`:

- authorize/wrap one endpoint;
- construct a session;
- write valid Requests from the peer endpoint;
- process with a bounded budget;
- read and validate correlated responses from the peer endpoint;
- verify session state remains usable;
- verify malformed input updates the owned aggregate state to `InboundRead`.

No filesystem socket pathname or listener is created.

## Runtime boundary

No listener, bind/accept/connect lifecycle, XDG mutation, stale-path cleanup, timers, concurrent tasks, service activation, DNS/network mutation, database work, private-key operation, or deployment is introduced.
