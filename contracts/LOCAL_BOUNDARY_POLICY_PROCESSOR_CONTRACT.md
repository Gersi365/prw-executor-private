# Private Remote Workspace Boundary-Aware Policy Processor Contract

Version: `0.1.0`

Status: Phase 048 provider-neutral policy composition

## Scope

Phase 048 composes the Phase 047 boundary-aware Request reader with the existing Phase 038 policy-gated read-only response builder. It performs no response I/O and owns no transport.

## Successful outcomes

The processor returns one of two successful outcomes:

- `CleanEof` when no byte of a new frame was received;
- `Response(LocalIpcFrame)` when a complete valid Request was decoded, policy-evaluated, and converted into the existing correlated terminal success/error frame.

## Ordering

The order is strict:

1. boundary-aware frame acquisition;
2. Request-specific decoding;
3. policy evaluation;
4. terminal response construction in memory.

Policy evaluation must not occur for clean EOF, truncated/invalid frames, non-Request frames, malformed command payloads, or unknown commands.

## Error preservation

Phase 048 reuses `LocalRequestProcessorError`:

- boundary/Request failures map to `Request(...)`;
- defensive policy-response construction failures map to `Response(...)`.

## Authentication boundary

The caller-supplied `PolicyEvaluator` is still assumed to belong to an already authenticated policy context, exactly as in Phase 038. Phase 048 does not authenticate or bind a principal.

## Forbidden interpretation

Phase 048 does not authorize or implement:

- response writing;
- Unix socket bind/listen/accept/connect/close;
- peer credentials or authentication;
- XDG runtime-path mutation;
- timers, concurrency, cancellation, or task/thread creation;
- systemd activation;
- network/DNS/TUN mutation;
- database changes;
- private-key operations;
- deployment.
