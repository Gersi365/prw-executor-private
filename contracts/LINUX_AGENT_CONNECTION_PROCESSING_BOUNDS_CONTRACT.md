# Private Remote Workspace Linux Agent Connection Processing Bounds Contract

Version: `0.1.0`

Status: Phase 072 runtime-readiness decision lock — no Agent bootstrap activation

## Purpose

Lock the wait, concurrency, and finite-session semantics that must exist before an authenticated Phase 071 local Linux session may be processed by the long-running Agent runtime.

The existing Phase 052 request-count budget is retained, but it is not interpreted as a wall-clock I/O bound.

## Evidence baseline

The validated repository already establishes:

- Phase 052: caller-supplied non-zero Request-count budget over generic `Read`/`Write`;
- Phase 059/060: same-UID authenticated blocking `UnixStream` ownership and session composition;
- Phase 070: nonblocking listener, one-shot accept, accepted stream deliberately blocking and `CLOEXEC`;
- Phase 071: authenticated accept outcome to authenticated application session composition with no application I/O.

Rust 1.97.1 `std::os::unix::net::UnixStream` provides read/write timeout configuration. A `None` timeout allows blocking indefinitely and zero `Duration` is rejected.

On Linux, `SO_RCVTIMEO` / `SO_SNDTIMEO` apply to individual socket I/O calls. A timed operation may return partial progress when bytes were transferred before the timeout. Therefore a per-syscall socket timeout alone is not accepted as a complete Request wall-clock bound when higher-level code uses retrying operations such as `read_exact` or `write_all`.

## Decision 1 — accepted application streams remain blocking

Phase 070 blocking accepted-stream semantics remain authoritative.

The connection-processing design will not switch authenticated application streams to `O_NONBLOCK` in this phase. This preserves the already validated provider-neutral `std::io::Read` / `Write` pipeline while wait bounds are added as an adapter above the socket.

## Decision 2 — absolute per-Request I/O deadlines

Every runtime-processed authenticated session must receive caller-supplied, strictly positive:

- Request-read wall-clock budget;
- terminal-response-write wall-clock budget.

The implementation must enforce an absolute monotonic deadline for each phase, not merely reapply the same relative timeout after every partial read/write.

Before each underlying blocking socket read/write call, the adapter must calculate the remaining time until the absolute deadline and configure the socket operation to wait for no longer than that remainder.

Partial progress does not reset or extend the absolute deadline.

A deadline expiry, timeout-class I/O error, or incomplete operation after deadline is a terminal connection-processing failure. Existing inbound/response poisoning and connection-discard semantics remain authoritative; the same connection is not reused after an ambiguous partial frame/response.

## Decision 3 — finite Request count per runtime session

The existing Phase 052 `NonZeroUsize` Request budget becomes the maximum Request count for one runtime session-worker invocation.

For the runtime orchestration path, `BudgetExhausted` means the worker closes/discards that authenticated connection after the final permitted response. The runtime path does not resume the same connection with a fresh budget.

Provider-neutral Phase 052 remains reusable by tests/other callers; this rule applies to the future Linux Agent runtime orchestration layer.

## Decision 4 — bounded worker concurrency

The accept/control path must never execute application Request I/O directly.

Each accepted/authenticated session is processed by an independent worker execution context, while the number of active workers is bounded by a caller-supplied strictly positive capacity.

No low-level library default is hard-coded by this contract.

The runtime must not create an unbounded number of threads/tasks.

When all worker capacity is occupied, the scheduler must not perform another accept merely to hold or reject a raw accepted stream. It leaves pending connections in the kernel listen queue until capacity becomes available or shutdown begins.

## Decision 5 — initial worker model

The initial implementation direction is standard-library OS threads, one worker per active authenticated session, under the explicit active-worker capacity above.

No async runtime dependency is introduced by this decision.

This keeps the current blocking application stream model intact and isolates a slow client from the accept/control path.

A future async/nonblocking application-stream architecture would require a separate decision and migration.

## Decision 6 — shutdown remains separately controlled

Phase 072 does not implement shutdown/cancellation.

Before Agent bootstrap activation, runtime orchestration must define how shutdown stops new accepts and causes/awaits active workers to terminate. The finite Request count and absolute per-Request I/O deadlines provide a bounded wait foundation but are not themselves a complete shutdown protocol.

No worker thread may be detached without lifecycle accounting in the final runtime.

## Required next implementation sequence

The safe implementation order after this decision is:

1. absolute-deadline blocking `UnixStream` I/O adapter, tested independently from the Agent bootstrap;
2. integration of deadline phases with one Request transaction while preserving current poisoning semantics;
3. bounded worker-capacity state/RAII accounting;
4. authenticated session worker using a single finite Phase 052 Request budget;
5. accept scheduler decision/implementation using capacity-gated one-shot Phase 070 accept;
6. explicit shutdown/cancellation lifecycle;
7. only then Agent bootstrap/service wiring.

## Deferred policy values

This contract deliberately does not choose concrete production values for:

- read deadline duration;
- write deadline duration;
- Requests per connection;
- maximum active workers;
- listener backlog;
- shutdown grace period.

Those values must be supplied/configured by the future runtime configuration boundary and validated there.

## Forbidden interpretation

Phase 072 does not authorize or implement:

- an accept loop;
- thread spawning in production source;
- Agent bootstrap activation;
- systemd/service activation;
- an async runtime dependency;
- nonblocking application streams;
- arbitrary retry after partial frame/response I/O;
- policy/principal binding changes;
- network/DNS/TUN/database/private-key/deployment mutation.
