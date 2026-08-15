# Private Remote Workspace Boundary-Aware Inbound Guard Contract

Version: `0.1.0`

Status: Phase 050 provider-neutral inbound-state composition

## Scope

Phase 050 composes the existing Phase 043 inbound Request safety state with the Phase 049 boundary-aware request/response transaction.

It remains generic over caller-supplied `std::io::Read` and `std::io::Write` objects and owns no transport.

## Ordering

The ordering is strict:

1. reject an already `ReadPoisoned` inbound state before consuming input;
2. delegate to the Phase 049 boundary-aware transaction;
3. `CleanEof` returns normally and leaves both protocol-direction states unchanged;
4. a Request acquisition/decoding failure transitions inbound state to `ReadPoisoned` before return;
5. policy-response construction or guarded response-write failures do not by themselves poison inbound framing state;
6. response-write ambiguity remains owned by the existing response-write state.

## Inbound poisoning rule

Only an error structurally classified as:

`Processing(LocalRequestProcessorError::Request(...))`

is an inbound framing/Request-decoding failure for Phase 050.

This includes truncated/invalid frames and Request decode failures. It excludes clean EOF, defensive response construction failures, and response-write failures.

## Clean EOF

Clean EOF before any byte of a new frame is a normal connection-stop outcome. It must not transition inbound state to `ReadPoisoned` and must not write response bytes.

## Authentication boundary

The supplied policy evaluator is still assumed to belong to an already authenticated policy context. Phase 050 does not authenticate or bind a principal.

## Forbidden interpretation

Phase 050 does not authorize or implement:

- Unix socket bind/listen/accept/connect/close;
- peer credentials or authentication;
- XDG runtime-path mutation;
- aggregate connection-state replacement;
- multi-request loops;
- timers, concurrency, cancellation, or task/thread creation;
- systemd activation;
- network/DNS/TUN mutation;
- database changes;
- private-key operations;
- deployment.
